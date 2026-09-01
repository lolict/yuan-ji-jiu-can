# ============================================================
# FQM 解释器 · 本体论执行 · 零运行时内存
# 执行容器操作：装入、取出、比较
# ============================================================

import strutils
import options
import std/os
import ../nim/fqm_constants
import ../nim/loader as fqm_loader
import ../nim/linker as fqm_linker

# ============================================================
# 数据类型
# ============================================================

type
  # 输出记录
  OutputEntry* = object
    line*: array[128, char]
    line_len*: int

  # FQM 解释器
  FQMInterpreter* = object
    loader*: ptr FQMLoader
    linker*: FQMLinker
    output*: array[256, OutputEntry]
    output_count*: int

# ============================================================
# 辅助：解析 袋.xxx.yyy 路径
# ============================================================

proc parse_path(path_str: string): Option[tuple[domain: string, identity: string]] =
  if path_str.len < 4 or not path_str.startsWith("袋."):
    return none(tuple[domain: string, identity: string])
  let rest = path_str[2..^1]
  let dot_pos = rest.find('.')
  if dot_pos < 0:
    return some((rest, ""))
  else:
    let domain = rest[0..<dot_pos]
    let identity = rest[dot_pos+1..^1]
    return some((domain, identity))

# ============================================================
# 前向声明
# ============================================================

proc op_query*(interpreter: var FQMInterpreter, cmd: string): string
proc op_operation*(interpreter: var FQMInterpreter, cmd: string): string
proc op_link*(interpreter: var FQMInterpreter, cmd: string): string
proc op_load*(interpreter: var FQMInterpreter, domain: string, symbol: string): string
proc op_extract*(interpreter: var FQMInterpreter, domain: string, identity: string): string
proc op_compare*(interpreter: var FQMInterpreter, domain: string, sym1: string, sym2: string): string

# ============================================================
# 命令执行
# ============================================================

proc execute*(interpreter: var FQMInterpreter, command: string): Option[string] =
  let cmd = strip(command)
  if cmd.len == 0 or cmd[0] == '#':
    return none(string)

  # 链接查询: 袋.声母.引.韵母(首)
  if "引" in cmd and "(" in cmd:
    return some(interpreter.op_link(cmd))

  # 带参数操作: 袋.声母.装入(b)
  if "(" in cmd:
    return some(interpreter.op_operation(cmd))

  # 无参数查询: 袋.声母 或 袋.声母.首
  if cmd.len > 2 and cmd.startsWith("袋."):
    return some(interpreter.op_query(cmd))

  return some("错误: 无效命令 " & cmd)

# ============================================================
# 查询处理
# ============================================================

proc op_query*(interpreter: var FQMInterpreter, cmd: string): string =
  let path_opt = parse_path(cmd)
  if path_opt.is_none:
    return "错误: 路径格式 " & cmd

  let (domain, identity) = path_opt.get()

  if identity.len == 0:
    # 袋.声母 - 列出容器内所有符号
    let loader = interpreter.loader
    var symbols: seq[string] = @[]
    for i in 0..<loader.container_count:
      let name = get_arr(loader.containers[i].name, loader.containers[i].name_len)
      if name == domain:
        for j in 0..<loader.containers[i].symbol_count:
          let sym = get_arr(loader.containers[i].symbols[j].symbol,
                            loader.containers[i].symbols[j].symbol_len)
          symbols.add(sym)
        break
    if symbols.len > 0:
      result = "袋." & domain & " = ["
      for i, s in symbols:
        if i > 0: result.add(", ")
        result.add(s)
      result.add("]")
    else:
      result = "袋." & domain & " = []"
  else:
    # 袋.声母.首 - 查询具体位置的符号
    var found = false
    for k in 0..<interpreter.linker.stp_count:
      let d = get_arr(interpreter.linker.symbol_to_path[k].domain,
                      interpreter.linker.symbol_to_path[k].domain_len)
      let id = get_arr(interpreter.linker.symbol_to_path[k].path.identity,
                       interpreter.linker.symbol_to_path[k].path.identity_len)
      if d == domain and id == identity:
        let sym = get_arr(interpreter.linker.symbol_to_path[k].symbol,
                          interpreter.linker.symbol_to_path[k].symbol_len)
        result = cmd & " = " & sym
        found = true
        break
    if not found:
      result = "错误: " & cmd & " 为空"

  if interpreter.output_count < 256:
    let oi = interpreter.output_count
    set_arr(interpreter.output[oi].line, interpreter.output[oi].line_len, result)
    interpreter.output_count += 1

# ============================================================
# 操作处理
# ============================================================

proc op_operation*(interpreter: var FQMInterpreter, cmd: string): string =
  let paren_start = cmd.find('(')
  let paren_end = cmd.rfind(')')

  if paren_start < 0 or paren_end < 0 or paren_end <= paren_start:
    return "错误: 无法解析 " & cmd

  let path_str = cmd[0..<paren_start]
  let args_str = cmd[paren_start+1..<paren_end]
  let args = args_str.split(',')

  let parts = path_str.split('.')
  if parts.len < 3:
    return "错误: 路径格式 " & path_str

  let domain = parts[1]
  let op = parts[2]

  if op == "装入":
    if args.len < 1: return "错误: 装入需要参数"
    let symbol = strip(args[0])
    result = interpreter.op_load(domain, symbol)
  elif op == "取出":
    if args.len < 1: return "错误: 取出需要参数"
    let identity = strip(args[0])
    result = interpreter.op_extract(domain, identity)
  elif op == "比较":
    if args.len < 2: return "错误: 比较需要2个参数"
    let sym1 = strip(args[0])
    let sym2 = strip(args[1])
    result = interpreter.op_compare(domain, sym1, sym2)
  else:
    result = "错误: 未知操作 " & op

  if interpreter.output_count < 256:
    let oi = interpreter.output_count
    set_arr(interpreter.output[oi].line, interpreter.output[oi].line_len, result)
    interpreter.output_count += 1

# ============================================================
# 装入操作
# ============================================================

proc op_load*(interpreter: var FQMInterpreter, domain: string, symbol: string): string =
  let existing = interpreter.linker.find_identity(domain, symbol)
  if existing.is_some:
    return symbol & " 已在 " & domain & "." & existing.get()

  let loader = interpreter.loader
  var container_idx = -1
  for i in 0..<loader.container_count:
    let name = get_arr(loader.containers[i].name, loader.containers[i].name_len)
    if name == domain:
      container_idx = i
      break

  if container_idx < 0:
    return "错误: 容器 " & domain & " 不存在"

  add_symbol(loader.containers[container_idx], symbol)

  # 重建索引
  var mutable_linker = interpreter.linker
  discard mutable_linker.build_ontology_index()

  let path_opt = interpreter.linker.find_identity(domain, symbol)
  if path_opt.is_some:
    return "装入成功: " & symbol & " → " & domain & "." & path_opt.get()
  return "装入成功: " & symbol

# ============================================================
# 取出操作
# ============================================================

proc op_extract*(interpreter: var FQMInterpreter, domain: string, identity: string): string =
  for k in 0..<interpreter.linker.stp_count:
    let d = get_arr(interpreter.linker.symbol_to_path[k].domain,
                    interpreter.linker.symbol_to_path[k].domain_len)
    let id = get_arr(interpreter.linker.symbol_to_path[k].path.identity,
                     interpreter.linker.symbol_to_path[k].path.identity_len)
    if d == domain and id == identity:
      let sym = get_arr(interpreter.linker.symbol_to_path[k].symbol,
                        interpreter.linker.symbol_to_path[k].symbol_len)
      return "取出: " & domain & "." & identity & " = " & sym

  return "错误: " & domain & "." & identity & " 为空"

# ============================================================
# 比较操作
# ============================================================

proc op_compare*(interpreter: var FQMInterpreter, domain: string, sym1: string, sym2: string): string =
  let path1_opt = interpreter.linker.find_identity(domain, sym1)
  let path2_opt = interpreter.linker.find_identity(domain, sym2)

  if path1_opt.is_none:
    return "错误: " & sym1 & " 不在 " & domain & " 容器中"
  if path2_opt.is_none:
    return "错误: " & sym2 & " 不在 " & domain & " 容器中"

  let path1 = path1_opt.get()
  let path2 = path2_opt.get()

  if path1 == path2:
    return "比较: " & sym1 & " == " & sym2 & "（同一位置 " & domain & "." & path1 & "）"
  else:
    return "比较: " & sym1 & "(" & domain & "." & path1 & ") ≠ " & sym2 & "(" & domain & "." & path2 & ")"

# ============================================================
# 链接查询
# ============================================================

proc op_link*(interpreter: var FQMInterpreter, cmd: string): string =
  let paren_start = cmd.find('(')
  let paren_end = cmd.rfind(')')
  if paren_start < 0 or paren_end < 0:
    return "错误: 链接格式 " & cmd

  let identity = strip(cmd[paren_start+1..<paren_end])
  let before_paren = cmd[0..<paren_start]

  let parts = before_paren.split('.')
  if parts.len < 4:
    return "错误: 链接路径格式 " & before_paren

  let from_domain = parts[1]
  let to_domain = parts[3]

  let from_path_str = from_domain & "." & identity

  for i in 0..<interpreter.linker.resolved_count:
    let r = interpreter.linker.resolved[i]
    let from_d = get_arr(r.from_domain, r.from_domain_len)
    let from_id = get_arr(r.from_path.identity, r.from_path.identity_len)
    if from_d == from_domain and from_id == identity:
      let to_sym = get_arr(r.to_symbol, r.to_symbol_len)
      let to_path = path_str(r.to_path)
      return "链接: " & from_path_str & " → " & to_path & "（" & to_sym & "）"

  return "错误: " & from_path_str & " 无链接"

# ============================================================
# 批量执行
# ============================================================

proc execute_batch*(interpreter: var FQMInterpreter, commands: seq[string]): seq[string] =
  for cmd in commands:
    let exec_result = interpreter.execute(cmd)
    if exec_result.is_some:
      result.add(exec_result.get())

# ============================================================
# 显示
# ============================================================

proc display_memory*(interpreter: FQMInterpreter) =
  echo ""
  echo "============================================================"
  echo "内存状态（本体论路径 → 符号）"
  echo "------------------------------------------------------------"

  for k in 0..<interpreter.linker.stp_count:
    let domain = get_arr(interpreter.linker.symbol_to_path[k].domain,
                         interpreter.linker.symbol_to_path[k].domain_len)
    let identity = get_arr(interpreter.linker.symbol_to_path[k].path.identity,
                           interpreter.linker.symbol_to_path[k].path.identity_len)
    let sym = get_arr(interpreter.linker.symbol_to_path[k].symbol,
                      interpreter.linker.symbol_to_path[k].symbol_len)
    echo "  ", domain, ".", identity, " → ", sym

  echo "============================================================"
  echo ""

proc display_output*(interpreter: FQMInterpreter) =
  echo ""
  echo "============================================================"
  echo "执行记录"
  echo "------------------------------------------------------------"
  for i in 0..<interpreter.output_count:
    echo "  ", i + 1, ". ", get_arr(interpreter.output[i].line, interpreter.output[i].line_len)
  echo "============================================================"
  echo ""

# ============================================================
# 入口
# ============================================================

when false:
  echo "============================================================"
  echo "FQM 解释器测试"
  echo "============================================================"

  var fLoader: FQMLoader
  discard fLoader.load_file("containers.fqm")

  var fLinker: FQMLinker
  fLinker.loader = addr fLoader
  discard fLinker.build_ontology_index()
  discard fLinker.resolve_links()

  var interpreter: FQMInterpreter
  var myInterpLinker = fLinker
  interpreter.linker = addr myInterpLinker
  interpreter.output_count = 0

  echo ""
  echo "执行测试:"
  let commands = @[
    "袋.声母",
    "袋.声母.首",
    "袋.声母.取出(首)",
    "袋.声母.取出(次)",
    "袋.韵母.取出(首)",
    "袋.声母.引.韵母(首)",
    "袋.声母.比较(b, p)"
  ]

  for cmd in commands:
    let result = interpreter.execute(cmd)
    if result.is_some:
      echo "  ", cmd
      echo "    → ", result.get()

  echo ""
  interpreter.display_memory()
  interpreter.display_output()
