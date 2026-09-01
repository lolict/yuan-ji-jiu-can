# ============================================================
# FQM 维度升降解释器 · 高维⇄低维映射 · 零运行时内存
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
  FullCoord* = object
    sm_id*: array[MAX_NAME_LEN, char]
    sm_id_len*: int
    ym_id*: array[MAX_NAME_LEN, char]
    ym_id_len*: int
    t_id*: array[MAX_NAME_LEN, char]
    t_id_len*: int

  CoordMapping* = object
    coord*: FullCoord
    sm_sym*: array[MAX_SYMBOL_LEN, char]
    sm_sym_len*: int
    ym_sym*: array[MAX_SYMBOL_LEN, char]
    ym_sym_len*: int
    t_sym*: array[MAX_SYMBOL_LEN, char]
    t_sym_len*: int

  DimensionInterpreter* = object
    loader*: ptr FQMLoader
    linker*: FQMLinker
    coordinates*: array[MAX_SYMBOLS, CoordMapping]
    coord_count*: int

# ============================================================
# 辅助
# ============================================================

proc set_coord(c: var FullCoord, sm, ym, t: string) =
  set_arr(c.sm_id, c.sm_id_len, sm)
  set_arr(c.ym_id, c.ym_id_len, ym)
  set_arr(c.t_id, c.t_id_len, t)

proc find_identity(interpreter: DimensionInterpreter, domain, symbol: string): Option[string] =
  for k in 0..<interpreter.linker.stp_count:
    let d = get_arr(interpreter.linker.symbol_to_path[k].domain,
                    interpreter.linker.symbol_to_path[k].domain_len)
    let s = get_arr(interpreter.linker.symbol_to_path[k].symbol,
                    interpreter.linker.symbol_to_path[k].symbol_len)
    if d == domain and s == symbol:
      return some(get_arr(interpreter.linker.symbol_to_path[k].path.identity,
                            interpreter.linker.symbol_to_path[k].path.identity_len))
  return none(string)

# ============================================================
# 构建坐标
# ============================================================

proc build_coordinates*(interpreter: var DimensionInterpreter) =
  interpreter.coord_count = 0
  let loader = interpreter.loader

  var sm_idx = -1; var ym_idx = -1; var t_idx = -1
  for i in 0..<loader.container_count:
    let name = get_arr(loader.containers[i].name, loader.containers[i].name_len)
    if name == DOMAIN_SHENGMU: sm_idx = i
    elif name == DOMAIN_YUNMU: ym_idx = i
    elif name == DOMAIN_TONE: t_idx = i

  if sm_idx < 0 or ym_idx < 0 or t_idx < 0: return

  for i in 0..<interpreter.linker.resolved_count:
    let r = interpreter.linker.resolved[i]
    let from_d = get_arr(r.from_domain, r.from_domain_len)
    if from_d != DOMAIN_SHENGMU: continue

    let from_sym = get_arr(r.from_symbol, r.from_symbol_len)
    let to_sym = get_arr(r.to_symbol, r.to_symbol_len)
    let from_id = get_arr(r.from_path.identity, r.from_path.identity_len)
    let to_id = get_arr(r.to_path.identity, r.to_path.identity_len)

    let t_container = addr loader.containers[t_idx]
    for j in 0..<t_container.symbol_count:
      let tone_sym = get_arr(t_container.symbols[j].symbol, t_container.symbols[j].symbol_len)
      let tone_idx = t_container.symbols[j].position
      let tone_identity = if tone_idx < 24: IDENTITY_NAMES[tone_idx]
                        else: "第" & $(tone_idx + 1)

      if interpreter.coord_count < MAX_SYMBOLS:
        let ci = interpreter.coord_count
        set_coord(interpreter.coordinates[ci].coord, from_id, to_id, tone_identity)
        set_arr(interpreter.coordinates[ci].sm_sym, interpreter.coordinates[ci].sm_sym_len, from_sym)
        set_arr(interpreter.coordinates[ci].ym_sym, interpreter.coordinates[ci].ym_sym_len, to_sym)
        set_arr(interpreter.coordinates[ci].t_sym, interpreter.coordinates[ci].t_sym_len, tone_sym)
        interpreter.coord_count += 1

# ============================================================
# 前向声明
# ============================================================

proc cmd_downshift(interpreter: var DimensionInterpreter, cmd: string): string
proc cmd_upshift(interpreter: var DimensionInterpreter, cmd: string): string
proc cmd_project(interpreter: var DimensionInterpreter, cmd: string): string
proc cmd_expand(interpreter: var DimensionInterpreter, cmd: string): string
proc cmd_dimensions(interpreter: DimensionInterpreter): string

# ============================================================
# 命令执行
# ============================================================

proc execute*(interpreter: var DimensionInterpreter, command: string): Option[string] =
  let cmd = strip(command)
  if cmd.len == 0 or cmd[0] == '#':
    return none(string)

  if cmd.startswith("降维("):
    return some(interpreter.cmd_downshift(cmd))
  if cmd.startswith("升维("):
    return some(interpreter.cmd_upshift(cmd))
  if cmd.startswith("投影("):
    return some(interpreter.cmd_project(cmd))
  if cmd.startswith("展开("):
    return some(interpreter.cmd_expand(cmd))
  if cmd == "维度":
    return some(interpreter.cmd_dimensions())

  return some("错误: 未知命令 " & cmd)

# ============================================================
# 降维
# ============================================================

proc cmd_downshift(interpreter: var DimensionInterpreter, cmd: string): string =
  let paren_start = cmd.find('(')
  let paren_end = cmd.rfind(')')
  if paren_start < 0 or paren_end < 0:
    return "错误: 降维格式"

  let inner = cmd[paren_start+1..<paren_end]
  let args = inner.split(',')
  if args.len != 4:
    return "错误: 降维需要4个参数 (b, a, 1, 声调)"

  let sm_sym = strip(args[0])
  let ym_sym = strip(args[1])
  let t_sym = strip(args[2])
  let drop_dim = strip(args[3])

  let sm_id_opt = interpreter.find_identity(DOMAIN_SHENGMU, sm_sym)
  let ym_id_opt = interpreter.find_identity(DOMAIN_YUNMU, ym_sym)
  let t_id_opt = interpreter.find_identity(DOMAIN_TONE, t_sym)

  if sm_id_opt.is_none or ym_id_opt.is_none or t_id_opt.is_none:
    return "错误: 找不到坐标 (" & sm_sym & ", " & ym_sym & ", " & t_sym & ")"

  let sm_id = sm_id_opt.get()
  let ym_id = ym_id_opt.get()
  let t_id = t_id_opt.get()

  result = "降维: (" & sm_sym & "," & ym_sym & "," & t_sym & ") → "

  var parts: seq[string] = @[]
  if drop_dim != DOMAIN_SHENGMU:
    parts.add("声母." & sm_id & "(" & sm_sym & ")")
  if drop_dim != DOMAIN_YUNMU:
    parts.add("韵母." & ym_id & "(" & ym_sym & ")")
  if drop_dim != DOMAIN_TONE:
    parts.add("声调." & t_id & "(" & t_sym & ")")

  result.add("(")
  for i, p in parts:
    if i > 0: result.add(", ")
    result.add(p)
  result.add(")")

# ============================================================
# 升维
# ============================================================

proc cmd_upshift(interpreter: var DimensionInterpreter, cmd: string): string =
  let paren_start = cmd.find('(')
  let paren_end = cmd.rfind(')')
  if paren_start < 0 or paren_end < 0:
    return "错误: 升维格式"

  let inner = cmd[paren_start+1..<paren_end]
  let args = inner.split(',')
  if args.len != 3:
    return "错误: 升维需要3个参数 (b, a, 声调)"

  let sm_sym = strip(args[0])
  let ym_sym = strip(args[1])
  let tone_default = strip(args[2])

  let sm_id_opt = interpreter.find_identity(DOMAIN_SHENGMU, sm_sym)
  let ym_id_opt = interpreter.find_identity(DOMAIN_YUNMU, ym_sym)
  let t_id_opt = interpreter.find_identity(DOMAIN_TONE, tone_default)

  if sm_id_opt.is_none or ym_id_opt.is_none:
    return "错误: 找不到 (" & sm_sym & ", " & ym_sym & ")"
  if t_id_opt.is_none:
    return "错误: 声调中无 " & tone_default

  let sm_id = sm_id_opt.get()
  let ym_id = ym_id_opt.get()
  let t_id = t_id_opt.get()

  result = "升维: (" & sm_sym & "," & ym_sym & ") + 声调" & tone_default & " → "
  result.add("(声母." & sm_id & "(" & sm_sym & "), 韵母." & ym_id & "(" & ym_sym & "), 声调." & t_id & "(" & tone_default & "))")

# ============================================================
# 投影
# ============================================================

proc cmd_project(interpreter: var DimensionInterpreter, cmd: string): string =
  let paren_start = cmd.find('(')
  let paren_end = cmd.rfind(')')
  if paren_start < 0 or paren_end < 0:
    return "错误: 投影格式"

  let inner = cmd[paren_start+1..<paren_end]
  let args = inner.split(',')
  if args.len != 4:
    return "错误: 投影需要4个参数 (b, a, 1, 声母)"

  let sm_sym = strip(args[0])
  let sm_id_opt = interpreter.find_identity(DOMAIN_SHENGMU, sm_sym)
  if sm_id_opt.is_none:
    return "错误: 声母中无 " & sm_sym

  let sm_id = sm_id_opt.get()
  result = "投影: (" & strip(args[0]) & "," & strip(args[1]) & "," & strip(args[2]) & ") → "
  result.add("声母." & sm_id & "(" & sm_sym & ")")

# ============================================================
# 展开
# ============================================================

proc cmd_expand(interpreter: var DimensionInterpreter, cmd: string): string =
  let paren_start = cmd.find('(')
  let paren_end = cmd.rfind(')')
  if paren_start < 0 or paren_end < 0:
    return "错误: 展开格式"

  let inner = cmd[paren_start+1..<paren_end]
  let args = inner.split(',')
  if args.len != 3:
    return "错误: 展开需要3个参数 (b, 韵母, 声调)"

  let sm_sym = strip(args[0])
  let ym_sym = strip(args[1])
  let t_sym = strip(args[2])

  let sm_id_opt = interpreter.find_identity(DOMAIN_SHENGMU, sm_sym)
  let ym_id_opt = interpreter.find_identity(DOMAIN_YUNMU, ym_sym)
  let t_id_opt = interpreter.find_identity(DOMAIN_TONE, t_sym)

  if sm_id_opt.is_none or ym_id_opt.is_none or t_id_opt.is_none:
    return "错误: 找不到完整坐标 (" & sm_sym & ", " & ym_sym & ", " & t_sym & ")"

  let sm_id = sm_id_opt.get()
  let ym_id = ym_id_opt.get()
  let t_id = t_id_opt.get()

  result = "展开: " & sm_sym & " → "
  result.add("(声母." & sm_id & "(" & sm_sym & "), 韵母." & ym_id & "(" & ym_sym & "), 声调." & t_id & "(" & t_sym & "))")

# ============================================================
# 维度信息
# ============================================================

proc cmd_dimensions(interpreter: DimensionInterpreter): string =
  let loader = interpreter.loader

  var sm_count = 0; var ym_count = 0; var t_count = 0
  for i in 0..<loader.container_count:
    let name = get_arr(loader.containers[i].name, loader.containers[i].name_len)
    if name == DOMAIN_SHENGMU: sm_count = loader.containers[i].symbol_count
    elif name == DOMAIN_YUNMU: ym_count = loader.containers[i].symbol_count
    elif name == DOMAIN_TONE: t_count = loader.containers[i].symbol_count

  result = "维度信息:\n"
  result.add("  声母轴: " & $sm_count & " 个位置\n")
  result.add("  韵母轴: " & $ym_count & " 个位置\n")
  result.add("  声调轴: " & $t_count & " 个位置\n")
  result.add("  三维空间总容量: " & $sm_count & "×" & $ym_count & "×" & $t_count & " = " & $(sm_count * ym_count * t_count) & "\n")
  result.add("  已占用坐标: " & $interpreter.coord_count)

# ============================================================
# 显示
# ============================================================

proc display_coordinates*(interpreter: DimensionInterpreter) =
  echo ""
  echo "============================================================"
  echo "高维坐标（完整三维）"
  echo "------------------------------------------------------------"

  for i in 0..<interpreter.coord_count:
    let c = interpreter.coordinates[i].coord
    echo "  (声母.", get_arr(c.sm_id, c.sm_id_len),
         ", 韵母.", get_arr(c.ym_id, c.ym_id_len),
         ", 声调.", get_arr(c.t_id, c.t_id_len), ") = (",
         get_arr(interpreter.coordinates[i].sm_sym, interpreter.coordinates[i].sm_sym_len), ", ",
         get_arr(interpreter.coordinates[i].ym_sym, interpreter.coordinates[i].ym_sym_len), ", ",
         get_arr(interpreter.coordinates[i].t_sym, interpreter.coordinates[i].t_sym_len), ")"

  echo ""
  echo "============================================================"
  echo ""

# ============================================================
# 入口
# ============================================================

when false:
  echo "============================================================"
  echo "FQM 维度升降解释器测试"
  echo "============================================================"

  var myLoader: FQMLoader
  discard myLoader.load_file("containers.fqm")

  var myLinker: FQMLinker
  myLinker.loader = addr myLoader
  discard myLinker.build_ontology_index()
  discard myLinker.resolve_links()

  var dimension: DimensionInterpreter
  dimension.linker = addr myLinker
  dimension.build_coordinates()

  dimension.display_coordinates()

  echo ""
  echo "维度命令测试:"
  let commands = @[
    "维度",
    "降维(b, a, 1, 声调)",
    "升维(b, a, 1)",
    "投影(b, a, 1, 声母)",
    "展开(b, a, 1)"
  ]

  for cmd in commands:
    let result = dimension.execute(cmd)
    if result.is_some:
      echo ""
      echo "  ", cmd
      for line in result.get().split('\n'):
        echo "    ", line
