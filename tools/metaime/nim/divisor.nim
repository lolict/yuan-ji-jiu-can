# ============================================================
# FQM 公约数公倍数解释器 · 关系运算 · 零运行时内存
# ============================================================

import strutils
import options
import ../nim/fqm_constants
import ../nim/loader
import ../nim/linker

# ============================================================
# 数据类型
# ============================================================

type
  DomainInfo* = object
    name*: array[MAX_NAME_LEN, char]
    name_len*: int
    identities*: array[MAX_SYMBOLS, array[MAX_NAME_LEN, char]]
    identity_lens*: array[MAX_SYMBOLS, int]
    identity_count*: int

  DivisorInterpreter* = object
    loader*: ptr FQMLoader
    linker*: FQMLinker
    domains*: array[MAX_CONTAINERS, DomainInfo]
    domain_count*: int

# ============================================================
# 辅助
# ============================================================

proc find_domain(interpreter: var DivisorInterpreter, name: string): int =
  for i in 0..<interpreter.domain_count:
    if arr_eq(interpreter.domains[i].name, interpreter.domains[i].name_len, name):
      return i
  return -1

proc add_domain_identity(interpreter: var DivisorInterpreter, domain, identity: string) =
  var idx = find_domain(interpreter, domain)
  if idx < 0:
    if interpreter.domain_count >= MAX_CONTAINERS: return
    idx = interpreter.domain_count
    set_arr(interpreter.domains[idx].name, interpreter.domains[idx].name_len, domain)
    interpreter.domains[idx].identity_count = 0
    interpreter.domain_count += 1

  for j in 0..<interpreter.domains[idx].identity_count:
    if arr_eq(interpreter.domains[idx].identities[j], interpreter.domains[idx].identity_lens[j], identity):
      return

  if interpreter.domains[idx].identity_count < MAX_SYMBOLS:
    let ic = interpreter.domains[idx].identity_count
    set_arr(interpreter.domains[idx].identities[ic], interpreter.domains[idx].identity_lens[ic], identity)
    interpreter.domains[idx].identity_count += 1

# ============================================================
# 构建域信息
# ============================================================

proc build_domain_info*(interpreter: var DivisorInterpreter) =
  interpreter.domain_count = 0

  for k in 0..<interpreter.linker.stp_count:
    let domain = get_arr(interpreter.linker.symbol_to_path[k].domain,
                         interpreter.linker.symbol_to_path[k].domain_len)
    let identity = get_arr(interpreter.linker.symbol_to_path[k].path.identity,
                           interpreter.linker.symbol_to_path[k].path.identity_len)
    add_domain_identity(interpreter, domain, identity)

proc find_identity(interpreter: DivisorInterpreter, domain, symbol: string): Option[string] =
  for k in 0..<interpreter.linker.stp_count:
    let d = get_arr(interpreter.linker.symbol_to_path[k].domain,
                    interpreter.linker.symbol_to_path[k].domain_len)
    let s = get_arr(interpreter.linker.symbol_to_path[k].symbol,
                    interpreter.linker.symbol_to_path[k].symbol_len)
    if d == domain and s == symbol:
      return some(get_arr(interpreter.linker.symbol_to_path[k].path.identity,
                            interpreter.linker.symbol_to_path[k].path.identity_len))
  return none(string)

proc find_symbol(interpreter: DivisorInterpreter, domain, identity: string): Option[string] =
  for k in 0..<interpreter.linker.stp_count:
    let d = get_arr(interpreter.linker.symbol_to_path[k].domain,
                    interpreter.linker.symbol_to_path[k].domain_len)
    let id = get_arr(interpreter.linker.symbol_to_path[k].path.identity,
                     interpreter.linker.symbol_to_path[k].path.identity_len)
    if d == domain and id == identity:
      return some(get_arr(interpreter.linker.symbol_to_path[k].symbol,
                          interpreter.linker.symbol_to_path[k].symbol_len))
  return none(string)

proc find_path(interpreter: DivisorInterpreter, symbol: string): Option[tuple[domain: string, identity: string]] =
  for k in 0..<interpreter.linker.stp_count:
    let s = get_arr(interpreter.linker.symbol_to_path[k].symbol,
                    interpreter.linker.symbol_to_path[k].symbol_len)
    if s == symbol:
      let d = get_arr(interpreter.linker.symbol_to_path[k].domain,
                      interpreter.linker.symbol_to_path[k].domain_len)
      let id = get_arr(interpreter.linker.symbol_to_path[k].path.identity,
                       interpreter.linker.symbol_to_path[k].path.identity_len)
      return some((d, id))
  return none(tuple[domain: string, identity: string])

# ============================================================
# 前向声明
# ============================================================

proc cmd_common_divisor(interpreter: var DivisorInterpreter, cmd: string): string
proc cmd_common_multiple(interpreter: var DivisorInterpreter, cmd: string): string
proc cmd_same_divisor(interpreter: var DivisorInterpreter, cmd: string): string
proc cmd_aligned(interpreter: var DivisorInterpreter, cmd: string): string
proc cmd_relation(interpreter: var DivisorInterpreter, cmd: string): string
proc cmd_list_divisors(interpreter: DivisorInterpreter): string

# ============================================================
# 命令执行
# ============================================================

proc execute*(interpreter: var DivisorInterpreter, command: string): Option[string] =
  let cmd = strip(command)
  if cmd.len == 0 or cmd[0] == '#':
    return none(string)

  if cmd.startswith("公约("):
    return some(interpreter.cmd_common_divisor(cmd))
  if cmd.startswith("公倍("):
    return some(interpreter.cmd_common_multiple(cmd))
  if cmd.startswith("同公("):
    return some(interpreter.cmd_same_divisor(cmd))
  if cmd.startswith("齐公("):
    return some(interpreter.cmd_aligned(cmd))
  if cmd.startswith("关系("):
    return some(interpreter.cmd_relation(cmd))
  if cmd == "公约数":
    return some(interpreter.cmd_list_divisors())

  return some("错误: 未知命令 " & cmd)

# ============================================================
# 公约数
# ============================================================

proc cmd_common_divisor(interpreter: var DivisorInterpreter, cmd: string): string =
  let paren_start = cmd.find('(')
  let paren_end = cmd.rfind(')')
  if paren_start < 0 or paren_end < 0:
    return "错误: 公约格式"

  let inner = cmd[paren_start+1..<paren_end]
  let args = inner.split(',')
  if args.len != 2:
    return "错误: 公约需要2个参数 (声母, 韵母)"

  let domain1 = strip(args[0])
  let domain2 = strip(args[1])

  var idx1 = find_domain(interpreter, domain1)
  var idx2 = find_domain(interpreter, domain2)

  if idx1 < 0 or idx2 < 0:
    return "错误: 域不存在"

  var common: seq[string] = @[]
  for i in 0..<interpreter.domains[idx1].identity_count:
    let id = get_arr(interpreter.domains[idx1].identities[i],
                     interpreter.domains[idx1].identity_lens[i])
    for j in 0..<interpreter.domains[idx2].identity_count:
      if arr_eq(interpreter.domains[idx2].identities[j],
                  interpreter.domains[idx2].identity_lens[j], id):
        common.add(id)
        break

  if common.len > 0:
    result = "公约(" & domain1 & ", " & domain2 & "):\n"
    result.add("  共有身份: ")
    for i, c in common:
      if i > 0: result.add(", ")
      result.add(c)
    result.add("\n")
    result.add("  数量: " & $common.len)
  else:
    result = "公约(" & domain1 & ", " & domain2 & "): 无共有身份"

# ============================================================
# 公倍
# ============================================================

proc cmd_common_multiple(interpreter: var DivisorInterpreter, cmd: string): string =
  let paren_start = cmd.find('(')
  let paren_end = cmd.rfind(')')
  if paren_start < 0 or paren_end < 0:
    return "错误: 公倍格式"

  let inner = cmd[paren_start+1..<paren_end]
  let args = inner.split(',')
  if args.len != 2:
    return "错误: 公倍需要2个参数 (声母.首, 韵母.首)"

  let pos1 = strip(args[0])
  let pos2 = strip(args[1])

  let parts1 = pos1.split('.')
  let parts2 = pos2.split('.')
  if parts1.len != 2 or parts2.len != 2:
    return "错误: 位置格式应为 域.身份"

  let domain1 = parts1[0]; let id1 = parts1[1]
  let domain2 = parts2[0]; let id2 = parts2[1]

  let sym1_opt = interpreter.find_symbol(domain1, id1)
  let sym2_opt = interpreter.find_symbol(domain2, id2)

  let sym1_str = if sym1_opt.is_some: sym1_opt.get() else: "?"
  let sym2_str = if sym2_opt.is_some: sym2_opt.get() else: "?"

  result = "公倍(" & pos1 & ", " & pos2 & "):\n"
  result.add("  " & pos1 & " = " & sym1_str & "\n")
  result.add("  " & pos2 & " = " & sym2_str & "\n")
  result.add("  组合: (" & domain1 & "." & sym1_str & ", " & domain2 & "." & sym2_str & ")")

# ============================================================
# 同公
# ============================================================

proc cmd_same_divisor(interpreter: var DivisorInterpreter, cmd: string): string =
  let paren_start = cmd.find('(')
  let paren_end = cmd.rfind(')')
  if paren_start < 0 or paren_end < 0:
    return "错误: 同公格式"

  let inner = cmd[paren_start+1..<paren_end]
  let args = inner.split(',')
  if args.len != 2:
    return "错误: 同公需要2个参数"

  let sym1 = strip(args[0])
  let sym2 = strip(args[1])

  let path1_opt = interpreter.find_path(sym1)
  let path2_opt = interpreter.find_path(sym2)

  if path1_opt.is_none or path2_opt.is_none:
    return "错误: 找不到符号 " & sym1 & " 或 " & sym2

  let (d1, id1) = path1_opt.get()
  let (d2, id2) = path2_opt.get()

  if id1 == id2:
    result = "同公(" & sym1 & ", " & sym2 & "): 是\n"
    result.add("  " & sym1 & " 在 " & d1 & "." & id1 & "\n")
    result.add("  " & sym2 & " 在 " & d2 & "." & id2 & "\n")
    result.add("  共享公约数: " & id1)
  else:
    result = "同公(" & sym1 & ", " & sym2 & "): 否\n"
    result.add("  " & sym1 & " 在 " & d1 & "." & id1 & "\n")
    result.add("  " & sym2 & " 在 " & d2 & "." & id2)

# ============================================================
# 齐公
# ============================================================

proc cmd_aligned(interpreter: var DivisorInterpreter, cmd: string): string =
  let paren_start = cmd.find('(')
  let paren_end = cmd.rfind(')')
  if paren_start < 0 or paren_end < 0:
    return "错误: 齐公格式"

  let inner = cmd[paren_start+1..<paren_end]
  let args = inner.split(',')
  if args.len != 2:
    return "错误: 齐公需要2个参数"

  let pos1 = strip(args[0])
  let pos2 = strip(args[1])

  let parts1 = pos1.split('.')
  let parts2 = pos2.split('.')
  if parts1.len != 2 or parts2.len != 2:
    return "错误: 位置格式应为 域.身份"

  let domain1 = parts1[0]; let id1 = parts1[1]
  let domain2 = parts2[0]; let id2 = parts2[1]

  let sym1_opt = interpreter.find_symbol(domain1, id1)
  let sym2_opt = interpreter.find_symbol(domain2, id2)

  let sym1_str = if sym1_opt.is_some: sym1_opt.get() else: "?"
  let sym2_str = if sym2_opt.is_some: sym2_opt.get() else: "?"

  if id1 == id2:
    result = "齐公(" & pos1 & ", " & pos2 & "): 对齐\n"
    result.add("  身份: " & id1 & "\n")
    result.add("  " & domain1 & "的" & id1 & " = " & sym1_str & "\n")
    result.add("  " & domain2 & "的" & id1 & " = " & sym2_str)
  else:
    result = "齐公(" & pos1 & ", " & pos2 & "): 不对齐\n"
    result.add("  " & pos1 & " 身份为 " & id1 & "\n")
    result.add("  " & pos2 & " 身份为 " & id2)

# ============================================================
# 关系
# ============================================================

proc cmd_relation(interpreter: var DivisorInterpreter, cmd: string): string =
  let paren_start = cmd.find('(')
  let paren_end = cmd.rfind(')')
  if paren_start < 0 or paren_end < 0:
    return "错误: 关系格式"

  let inner = cmd[paren_start+1..<paren_end]
  let args = inner.split(',')
  if args.len != 2:
    return "错误: 关系需要2个参数"

  let sym1 = strip(args[0])
  let sym2 = strip(args[1])

  let path1_opt = interpreter.find_path(sym1)
  let path2_opt = interpreter.find_path(sym2)

  if path1_opt.is_none or path2_opt.is_none:
    return "错误: 找不到符号"

  let (d1, id1) = path1_opt.get()
  let (d2, id2) = path2_opt.get()

  if d1 == d2:
    result = "关系(" & sym1 & ", " & sym2 & "): 同域(" & d1 & ")\n"
    result.add("  " & sym1 & " 在 " & d1 & "." & id1 & "\n")
    result.add("  " & sym2 & " 在 " & d2 & "." & id2)
  elif id1 == id2:
    result = "关系(" & sym1 & ", " & sym2 & "): 同公(" & id1 & ")\n"
    result.add("  " & sym1 & " 在 " & d1 & "." & id1 & "\n")
    result.add("  " & sym2 & " 在 " & d2 & "." & id2 & "\n")
    result.add("  共享身份: " & id1)
  else:
    result = "关系(" & sym1 & ", " & sym2 & "): 异域异公\n"
    result.add("  " & sym1 & " 在 " & d1 & "." & id1 & "\n")
    result.add("  " & sym2 & " 在 " & d2 & "." & id2)

# ============================================================
# 列出公约数
# ============================================================

proc cmd_list_divisors(interpreter: DivisorInterpreter): string =
  if interpreter.domain_count == 0:
    return "无域信息"

  var common: seq[string] = @[]
  for i in 0..<interpreter.domains[0].identity_count:
    let id = get_arr(interpreter.domains[0].identities[i],
                     interpreter.domains[0].identity_lens[i])
    var in_all = true
    for d in 1..<interpreter.domain_count:
      var found = false
      for j in 0..<interpreter.domains[d].identity_count:
        if arr_eq(interpreter.domains[d].identities[j],
                    interpreter.domains[d].identity_lens[j], id):
          found = true
          break
      if not found:
        in_all = false
        break
    if in_all:
      common.add(id)

  result = "公约数（所有域共有）:\n"
  if common.len > 0:
    result.add("  ")
    for i, c in common:
      if i > 0: result.add(", ")
      result.add(c)
    result.add("\n")
    result.add("  数量: " & $common.len)
  else:
    result.add("  无")

  result.add("\n\n各域身份:\n")
  for d in 0..<interpreter.domain_count:
    let name = get_arr(interpreter.domains[d].name, interpreter.domains[d].name_len)
    result.add("  " & name & ": ")
    let show = min(interpreter.domains[d].identity_count, 5)
    for i in 0..<show:
      if i > 0: result.add(", ")
      result.add(get_arr(interpreter.domains[d].identities[i],
                         interpreter.domains[d].identity_lens[i]))
    if interpreter.domains[d].identity_count > 5:
      result.add("... (共" & $interpreter.domains[d].identity_count & "个)")
    result.add("\n")

# ============================================================
# 显示
# ============================================================

proc display_divisors*(interpreter: DivisorInterpreter) =
  echo ""
  echo "============================================================"
  echo "公约数信息"
  echo "------------------------------------------------------------"

  for d in 0..<interpreter.domain_count:
    echo "  ", get_arr(interpreter.domains[d].name, interpreter.domains[d].name_len),
         ": ", interpreter.domains[d].identity_count, " 个身份"

  echo "============================================================"
  echo ""

# ============================================================
# 入口
# ============================================================

when false:
  echo "============================================================"
  echo "FQM 公约数公倍数解释器测试"
  echo "============================================================"

  var myLoader: FQMLoader
  discard myLoader.load_file("containers.fqm")

  var myLinker: FQMLinker
  myLinker.loader = addr myLoader
  discard myLinker.build_ontology_index()
  discard myLinker.resolve_links()

  var divisor: DivisorInterpreter
  divisor.linker = addr myLinker
  divisor.build_domain_info()

  divisor.display_divisors()

  echo ""
  echo "公约数命令测试:"
  let commands = @[
    "公约数",
    "公约(声母, 韵母)",
    "公倍(声母.首, 韵母.首)",
    "同公(b, p)",
    "齐公(声母.首, 韵母.首)",
    "关系(b, a)"
  ]

  for cmd in commands:
    let result = divisor.execute(cmd)
    if result.is_some:
      echo ""
      echo "  ", cmd
      for line in result.get().split('\n'):
        echo "    ", line
