# ============================================================
# FQM 三维进制解释器 · 零运行时内存
# 声母×韵母×声调 = 三维空间坐标
# ============================================================

import strutils
import math
import options
import ../nim/fqm_constants
import ../nim/loader
import ../nim/linker

# ============================================================
# 数据类型
# ============================================================

type
  Point3D* = object
    shengmu*: array[MAX_NAME_LEN, char]
    shengmu_len*: int
    yunmu*: array[MAX_NAME_LEN, char]
    yunmu_len*: int
    tone*: array[MAX_NAME_LEN, char]
    tone_len*: int

  Coordinate* = object
    point*: Point3D
    sm_symbol*: array[MAX_SYMBOL_LEN, char]
    sm_symbol_len*: int
    ym_symbol*: array[MAX_SYMBOL_LEN, char]
    ym_symbol_len*: int
    t_symbol*: array[MAX_SYMBOL_LEN, char]
    t_symbol_len*: int

  Radix3DInterpreter* = object
    loader*: ptr FQMLoader
    linker*: FQMLinker
    coordinates*: array[MAX_SYMBOLS, Coordinate]
    coord_count*: int

# ============================================================
# 辅助函数
# ============================================================

proc set_point(p: var Point3D, sm, ym, t: string) =
  set_arr(p.shengmu, p.shengmu_len, sm)
  set_arr(p.yunmu, p.yunmu_len, ym)
  set_arr(p.tone, p.tone_len, t)

proc point_str(p: Point3D): string =
  result = "(声母." & get_arr(p.shengmu, p.shengmu_len) &
           ", 韵母." & get_arr(p.yunmu, p.yunmu_len) &
           ", 声调." & get_arr(p.tone, p.tone_len) & ")"

proc identity_index(identity: string): int =
  for i in 0..<IDENTITY_NAMES.len:
    if IDENTITY_NAMES[i] == identity:
      return i
  return -1

proc ontology_distance(id1, id2: string): int =
  let idx1 = identity_index(id1)
  let idx2 = identity_index(id2)
  if idx1 >= 0 and idx2 >= 0:
    return abs(idx1 - idx2)
  return 0

# ============================================================
# 构建坐标
# ============================================================

proc build_coordinates*(interpreter: var Radix3DInterpreter) =
  interpreter.coord_count = 0
  let loader = interpreter.loader

  var sm_container_idx = -1
  var ym_container_idx = -1
  var t_container_idx = -1

  for i in 0..<loader.container_count:
    let name = get_arr(loader.containers[i].name, loader.containers[i].name_len)
    if name == DOMAIN_SHENGMU: sm_container_idx = i
    elif name == DOMAIN_YUNMU: ym_container_idx = i
    elif name == DOMAIN_TONE: t_container_idx = i

  if sm_container_idx < 0 or ym_container_idx < 0 or t_container_idx < 0:
    return

  for i in 0..<interpreter.linker.resolved_count:
    let r = interpreter.linker.resolved[i]
    let from_d = get_arr(r.from_domain, r.from_domain_len)
    if from_d != DOMAIN_SHENGMU: continue

    let from_sym = get_arr(r.from_symbol, r.from_symbol_len)
    let to_sym = get_arr(r.to_symbol, r.to_symbol_len)
    let from_id = get_arr(r.from_path.identity, r.from_path.identity_len)
    let to_id = get_arr(r.to_path.identity, r.to_path.identity_len)

    let t_container = addr loader.containers[t_container_idx]
    for j in 0..<t_container.symbol_count:
      let tone_sym = get_arr(t_container.symbols[j].symbol, t_container.symbols[j].symbol_len)
      let tone_idx = t_container.symbols[j].position
      let tone_identity = if tone_idx < 24: IDENTITY_NAMES[tone_idx]
                        else: "第" & $(tone_idx + 1)

      if interpreter.coord_count < MAX_SYMBOLS:
        let ci = interpreter.coord_count
        set_point(interpreter.coordinates[ci].point, from_id, to_id, tone_identity)
        set_arr(interpreter.coordinates[ci].sm_symbol, interpreter.coordinates[ci].sm_symbol_len, from_sym)
        set_arr(interpreter.coordinates[ci].ym_symbol, interpreter.coordinates[ci].ym_symbol_len, to_sym)
        set_arr(interpreter.coordinates[ci].t_symbol, interpreter.coordinates[ci].t_symbol_len, tone_sym)
        interpreter.coord_count += 1

# ============================================================
# 查找辅助
# ============================================================

proc find_identity(interpreter: Radix3DInterpreter, domain, symbol: string): Option[string] =
  for k in 0..<interpreter.linker.stp_count:
    let d = get_arr(interpreter.linker.symbol_to_path[k].domain,
                    interpreter.linker.symbol_to_path[k].domain_len)
    let s = get_arr(interpreter.linker.symbol_to_path[k].symbol,
                    interpreter.linker.symbol_to_path[k].symbol_len)
    if d == domain and s == symbol:
      return some(get_arr(interpreter.linker.symbol_to_path[k].path.identity,
                            interpreter.linker.symbol_to_path[k].path.identity_len))
  return none(string)

proc find_symbol(interpreter: Radix3DInterpreter, domain, identity: string): Option[string] =
  for k in 0..<interpreter.linker.stp_count:
    let d = get_arr(interpreter.linker.symbol_to_path[k].domain,
                    interpreter.linker.symbol_to_path[k].domain_len)
    let id = get_arr(interpreter.linker.symbol_to_path[k].path.identity,
                     interpreter.linker.symbol_to_path[k].path.identity_len)
    if d == domain and id == identity:
      return some(get_arr(interpreter.linker.symbol_to_path[k].symbol,
                          interpreter.linker.symbol_to_path[k].symbol_len))
  return none(string)

# ============================================================
# 前向声明
# ============================================================

proc cmd_coordinate(interpreter: var Radix3DInterpreter, cmd: string): string
proc cmd_distance(interpreter: var Radix3DInterpreter, cmd: string): string
proc cmd_neighbors(interpreter: var Radix3DInterpreter, cmd: string): string
proc cmd_path(interpreter: var Radix3DInterpreter, cmd: string): string
proc cmd_space(interpreter: Radix3DInterpreter): string

# ============================================================
# 命令执行
# ============================================================

proc execute*(interpreter: var Radix3DInterpreter, command: string): Option[string] =
  let cmd = strip(command)
  if cmd.len == 0 or cmd[0] == '#':
    return none(string)

  if cmd.startswith("三维.坐标("):
    return some(interpreter.cmd_coordinate(cmd))
  if cmd.startswith("三维.距离("):
    return some(interpreter.cmd_distance(cmd))
  if cmd.startswith("三维.邻近("):
    return some(interpreter.cmd_neighbors(cmd))
  if cmd.startswith("三维.路径("):
    return some(interpreter.cmd_path(cmd))
  if cmd == "三维.空间":
    return some(interpreter.cmd_space())

  return some("错误: 未知命令 " & cmd)

# ============================================================
# 坐标查询
# ============================================================

proc cmd_coordinate(interpreter: var Radix3DInterpreter, cmd: string): string =
  let paren_start = cmd.find('(')
  let paren_end = cmd.rfind(')')
  if paren_start < 0 or paren_end < 0:
    return "错误: 坐标格式"

  let inner = cmd[paren_start+1..<paren_end]
  let parts = inner.split(',')
  if parts.len != 3:
    return "错误: 坐标需要3个参数 (b, a, 1)"

  let sm_sym = strip(parts[0])
  let ym_sym = strip(parts[1])
  let t_sym = strip(parts[2])

  let sm_id_opt = interpreter.find_identity(DOMAIN_SHENGMU, sm_sym)
  let ym_id_opt = interpreter.find_identity(DOMAIN_YUNMU, ym_sym)
  let t_id_opt = interpreter.find_identity(DOMAIN_TONE, t_sym)

  if sm_id_opt.is_none: return "错误: 声母中无 " & sm_sym
  if ym_id_opt.is_none: return "错误: 韵母中无 " & ym_sym
  if t_id_opt.is_none: return "错误: 声调中无 " & t_sym

  let sm_id = sm_id_opt.get()
  let ym_id = ym_id_opt.get()
  let t_id = t_id_opt.get()

  for i in 0..<interpreter.coord_count:
    let p = interpreter.coordinates[i].point
    if arr_eq(p.shengmu, p.shengmu_len, sm_id) and
       arr_eq(p.yunmu, p.yunmu_len, ym_id) and
       arr_eq(p.tone, p.tone_len, t_id):
      let sm_s = get_arr(interpreter.coordinates[i].sm_symbol,
                         interpreter.coordinates[i].sm_symbol_len)
      let ym_s = get_arr(interpreter.coordinates[i].ym_symbol,
                         interpreter.coordinates[i].ym_symbol_len)
      let t_s = get_arr(interpreter.coordinates[i].t_symbol,
                        interpreter.coordinates[i].t_symbol_len)
      return "坐标 " & point_str(p) & " = (声母." & sm_s & ", 韵母." & ym_s & ", 声调." & t_s & ")"

  return "坐标 (声母." & sm_id & ", 韵母." & ym_id & ", 声调." & t_id & ") = 空（无符号占据）"

# ============================================================
# 距离计算
# ============================================================

proc cmd_distance(interpreter: var Radix3DInterpreter, cmd: string): string =
  let paren_start = cmd.find('(')
  let paren_end = cmd.rfind(')')
  if paren_start < 0 or paren_end < 0:
    return "错误: 距离格式"

  let inner = cmd[paren_start+1..<paren_end]
  let args = inner.split(',')
  if args.len != 6:
    return "错误: 距离需要6个参数 (b, a, 1, p, o, 2)"

  let sm1_opt = interpreter.find_identity(DOMAIN_SHENGMU, strip(args[0]))
  let ym1_opt = interpreter.find_identity(DOMAIN_YUNMU, strip(args[1]))
  let t1_opt = interpreter.find_identity(DOMAIN_TONE, strip(args[2]))
  let sm2_opt = interpreter.find_identity(DOMAIN_SHENGMU, strip(args[3]))
  let ym2_opt = interpreter.find_identity(DOMAIN_YUNMU, strip(args[4]))
  let t2_opt = interpreter.find_identity(DOMAIN_TONE, strip(args[5]))

  if sm1_opt.is_none or ym1_opt.is_none or t1_opt.is_none or
     sm2_opt.is_none or ym2_opt.is_none or t2_opt.is_none:
    return "错误: 坐标参数中有未定义符号"

  let dist = ontology_distance(sm1_opt.get(), sm2_opt.get()) +
             ontology_distance(ym1_opt.get(), ym2_opt.get()) +
             ontology_distance(t1_opt.get(), t2_opt.get())

  return "距离: (" & strip(args[0]) & "," & strip(args[1]) & "," & strip(args[2]) &
         ") → (" & strip(args[3]) & "," & strip(args[4]) & "," & strip(args[5]) &
         ") = " & $dist

# ============================================================
# 邻近查询
# ============================================================

proc cmd_neighbors(interpreter: var Radix3DInterpreter, cmd: string): string =
  let paren_start = cmd.find('(')
  let paren_end = cmd.rfind(')')
  if paren_start < 0 or paren_end < 0:
    return "错误: 邻近格式"

  let inner = cmd[paren_start+1..<paren_end]
  let parts = inner.split(',')
  if parts.len != 3:
    return "错误: 邻近需要3个参数 (b, a, 1)"

  let sm_sym = strip(parts[0])
  let ym_sym = strip(parts[1])
  let t_sym = strip(parts[2])

  let sm_id_opt = interpreter.find_identity(DOMAIN_SHENGMU, sm_sym)
  let ym_id_opt = interpreter.find_identity(DOMAIN_YUNMU, ym_sym)
  let t_id_opt = interpreter.find_identity(DOMAIN_TONE, t_sym)

  if sm_id_opt.is_none or ym_id_opt.is_none or t_id_opt.is_none:
    return "错误: 坐标参数中有未定义符号"

  let sm_id = sm_id_opt.get()
  let ym_id = ym_id_opt.get()
  let t_id = t_id_opt.get()

  result = "邻近 (" & sm_sym & "," & ym_sym & "," & t_sym & "):"

  var nearby: seq[tuple[domain: string, identity: string]] = @[]
  for domain_id in [sm_id, ym_id, t_id]:
    let idx = identity_index(domain_id)
    if idx > 0:
      let domain = if domain_id == sm_id: DOMAIN_SHENGMU
                  elif domain_id == ym_id: DOMAIN_YUNMU
                  else: DOMAIN_TONE
      nearby.add((domain, IDENTITY_NAMES[idx-1]))
    if idx >= 0 and idx < 23:
      let domain = if domain_id == sm_id: DOMAIN_SHENGMU
                  elif domain_id == ym_id: DOMAIN_YUNMU
                  else: DOMAIN_TONE
      nearby.add((domain, IDENTITY_NAMES[idx+1]))

  for (domain, identity) in nearby:
    let sym_opt = interpreter.find_symbol(domain, identity)
    if sym_opt.is_some:
      result.add("\n  " & domain & "." & identity & "(" & sym_opt.get() & ")")

# ============================================================
# 路径查找
# ============================================================

proc cmd_path(interpreter: var Radix3DInterpreter, cmd: string): string =
  let paren_start = cmd.find('(')
  let paren_end = cmd.rfind(')')
  if paren_start < 0 or paren_end < 0:
    return "错误: 路径格式"

  let inner = cmd[paren_start+1..<paren_end]
  let args = inner.split(',')
  if args.len != 6:
    return "错误: 路径需要6个参数 (b, a, 1, p, o, 2)"

  let sm1_opt = interpreter.find_identity(DOMAIN_SHENGMU, strip(args[0]))
  let ym1_opt = interpreter.find_identity(DOMAIN_YUNMU, strip(args[1]))
  let t1_opt = interpreter.find_identity(DOMAIN_TONE, strip(args[2]))
  let sm2_opt = interpreter.find_identity(DOMAIN_SHENGMU, strip(args[3]))
  let ym2_opt = interpreter.find_identity(DOMAIN_YUNMU, strip(args[4]))
  let t2_opt = interpreter.find_identity(DOMAIN_TONE, strip(args[5]))

  if sm1_opt.is_none or ym1_opt.is_none or t1_opt.is_none or
     sm2_opt.is_none or ym2_opt.is_none or t2_opt.is_none:
    return "错误: 路径参数中有未定义符号"

  let sm1 = sm1_opt.get(); let sm2 = sm2_opt.get()
  let ym1 = ym1_opt.get(); let ym2 = ym2_opt.get()
  let t1 = t1_opt.get(); let t2 = t2_opt.get()

  var path: seq[string] = @[]

  let sm_idx1 = identity_index(sm1)
  let sm_idx2 = identity_index(sm2)
  if sm_idx1 >= 0 and sm_idx2 >= 0:
    let step = if sm_idx2 > sm_idx1: 1 else: -1
    var idx = sm_idx1
    while idx != sm_idx2:
      let next = idx + step
      path.add("声母." & IDENTITY_NAMES[idx] & " → 声母." & IDENTITY_NAMES[next])
      idx = next

  let ym_idx1 = identity_index(ym1)
  let ym_idx2 = identity_index(ym2)
  if ym_idx1 >= 0 and ym_idx2 >= 0:
    let step = if ym_idx2 > ym_idx1: 1 else: -1
    var idx = ym_idx1
    while idx != ym_idx2:
      let next = idx + step
      path.add("韵母." & IDENTITY_NAMES[idx] & " → 韵母." & IDENTITY_NAMES[next])
      idx = next

  let t_idx1 = identity_index(t1)
  let t_idx2 = identity_index(t2)
  if t_idx1 >= 0 and t_idx2 >= 0:
    let step = if t_idx2 > t_idx1: 1 else: -1
    var idx = t_idx1
    while idx != t_idx2:
      let next = idx + step
      path.add("声调." & IDENTITY_NAMES[idx] & " → 声调." & IDENTITY_NAMES[next])
      idx = next

  if path.len > 0:
    result = "路径 (" & strip(args[0]) & "," & strip(args[1]) & "," & strip(args[2]) &
             ") → (" & strip(args[3]) & "," & strip(args[4]) & "," & strip(args[5]) & "):"
    for i, p in path:
      result.add("\n  " & $(i+1) & ". " & p)
  else:
    result = "已在同一点"

# ============================================================
# 空间列出
# ============================================================

proc cmd_space(interpreter: Radix3DInterpreter): string =
  if interpreter.coord_count == 0:
    return "三维空间为空"

  result = "三维空间坐标:"
  for i in 0..<interpreter.coord_count:
    let p = interpreter.coordinates[i].point
    let sm_s = get_arr(interpreter.coordinates[i].sm_symbol,
                       interpreter.coordinates[i].sm_symbol_len)
    let ym_s = get_arr(interpreter.coordinates[i].ym_symbol,
                       interpreter.coordinates[i].ym_symbol_len)
    let t_s = get_arr(interpreter.coordinates[i].t_symbol,
                      interpreter.coordinates[i].t_symbol_len)
    result.add("\n  " & point_str(p) & " → (声母." & sm_s & ", 韵母." & ym_s & ", 声调." & t_s & ")")

# ============================================================
# 显示
# ============================================================

proc display_space*(interpreter: Radix3DInterpreter) =
  echo ""
  echo "============================================================"
  echo "三维进制空间"
  echo "------------------------------------------------------------"

  if interpreter.coord_count == 0:
    echo "  空间为空"
  else:
    for i in 0..<interpreter.coord_count:
      let p = interpreter.coordinates[i].point
      let sm_s = get_arr(interpreter.coordinates[i].sm_symbol,
                         interpreter.coordinates[i].sm_symbol_len)
      let ym_s = get_arr(interpreter.coordinates[i].ym_symbol,
                         interpreter.coordinates[i].ym_symbol_len)
      let t_s = get_arr(interpreter.coordinates[i].t_symbol,
                        interpreter.coordinates[i].t_symbol_len)
      echo "  ", point_str(p)
      echo "    → 声母.", sm_s, ", 韵母.", ym_s, ", 声调.", t_s

  echo "============================================================"
  echo ""

# ============================================================
# 入口
# ============================================================

when false:
  echo "============================================================"
  echo "FQM 三维进制解释器测试"
  echo "============================================================"

  var myLoader: FQMLoader
  discard myLoader.load_file("containers.fqm")

  var myLinker: FQMLinker
  myLinker.loader = addr myLoader
  discard myLinker.build_ontology_index()
  discard myLinker.resolve_links()

  var radix3d: Radix3DInterpreter
  radix3d.linker = addr myLinker
  radix3d.build_coordinates()

  radix3d.display_space()

  echo ""
  echo "三维命令测试:"
  let commands = @[
    "三维.空间",
    "三维.坐标(b, a, 1)",
    "三维.距离(b, a, 1, p, o, 2)",
    "三维.邻近(b, a, 1)",
    "三维.路径(b, a, 1, p, o, 2)"
  ]

  for cmd in commands:
    let result = radix3d.execute(cmd)
    if result.is_some:
      echo ""
      echo "  ", cmd
      for line in result.get().split('\n'):
        echo "    ", line
