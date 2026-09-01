# ============================================================
# FQM 元解释器 · 一元、二元、三元、四元关系层级 · 零运行时内存
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
  OneEntry* = object
    target*: array[MAX_ONTOLOGY_LEN, char]
    target_len*: int

  TwoEntry* = object
    center*: array[MAX_ONTOLOGY_LEN, char]
    center_len*: int
    auxiliary*: array[MAX_ONTOLOGY_LEN, char]
    auxiliary_len*: int

  ThreeEntry* = object
    observer*: array[MAX_ONTOLOGY_LEN, char]
    observer_len*: int
    party_a*: array[MAX_ONTOLOGY_LEN, char]
    party_a_len*: int
    party_b*: array[MAX_ONTOLOGY_LEN, char]
    party_b_len*: int

  FourEntry* = object
    shell*: array[MAX_ONTOLOGY_LEN, char]
    shell_len*: int
    content*: array[MAX_ONTOLOGY_LEN, char]
    content_len*: int

  YuanInterpreter* = object
    loader*: ptr FQMLoader
    linker*: FQMLinker
    ones*: array[64, OneEntry]
    one_count*: int
    twos*: array[64, TwoEntry]
    two_count*: int
    threes*: array[64, ThreeEntry]
    three_count*: int
    fours*: array[64, FourEntry]
    four_count*: int

# ============================================================
# 查找关系
# ============================================================

proc find_relation(interpreter: YuanInterpreter, a, b: string): string =
  for i in 0..<interpreter.linker.resolved_count:
    let from_sym = get_arr(interpreter.linker.resolved[i].from_symbol,
                           interpreter.linker.resolved[i].from_symbol_len)
    let to_sym = get_arr(interpreter.linker.resolved[i].to_symbol,
                         interpreter.linker.resolved[i].to_symbol_len)
    if from_sym == a and to_sym == b:
      return "直接连接 " & a & " → " & b
    if from_sym == b and to_sym == a:
      return "反向连接 " & b & " → " & a

  var domain_a = ""; var domain_b = ""
  var id_a = ""; var id_b = ""

  for k in 0..<interpreter.linker.stp_count:
    let sym = get_arr(interpreter.linker.symbol_to_path[k].symbol,
                      interpreter.linker.symbol_to_path[k].symbol_len)
    if sym == a:
      domain_a = get_arr(interpreter.linker.symbol_to_path[k].domain,
                         interpreter.linker.symbol_to_path[k].domain_len)
      id_a = get_arr(interpreter.linker.symbol_to_path[k].path.identity,
                     interpreter.linker.symbol_to_path[k].path.identity_len)
    if sym == b:
      domain_b = get_arr(interpreter.linker.symbol_to_path[k].domain,
                         interpreter.linker.symbol_to_path[k].domain_len)
      id_b = get_arr(interpreter.linker.symbol_to_path[k].path.identity,
                     interpreter.linker.symbol_to_path[k].path.identity_len)

  if domain_a.len > 0 and domain_b.len > 0:
    if domain_a == domain_b:
      if id_a == id_b:
        return "同域同位(" & domain_a & "." & id_a & ")"
      else:
        return "同域异位(" & domain_a & "内)"
    else:
      if id_a == id_b:
        return "异域同公(" & id_a & ")"
      else:
        return "异域异公"

  return "无直接关系"

# ============================================================
# 前向声明
# ============================================================

proc cmd_one(interpreter: var YuanInterpreter, cmd: string): string
proc cmd_two(interpreter: var YuanInterpreter, cmd: string): string
proc cmd_three(interpreter: var YuanInterpreter, cmd: string): string
proc cmd_four(interpreter: var YuanInterpreter, cmd: string): string
proc cmd_hierarchy(interpreter: YuanInterpreter): string
proc cmd_judge(interpreter: var YuanInterpreter, cmd: string): string
proc cmd_upgrade(interpreter: var YuanInterpreter, cmd: string): string
proc cmd_downgrade(interpreter: var YuanInterpreter, cmd: string): string

# ============================================================
# 命令执行
# ============================================================

proc execute*(interpreter: var YuanInterpreter, command: string): Option[string] =
  let cmd = strip(command)
  if cmd.len == 0 or cmd[0] == '#':
    return none(string)

  if cmd.startswith("一元("):
    return some(interpreter.cmd_one(cmd))
  if cmd.startswith("二元("):
    return some(interpreter.cmd_two(cmd))
  if cmd.startswith("三元("):
    return some(interpreter.cmd_three(cmd))
  if cmd.startswith("四元("):
    return some(interpreter.cmd_four(cmd))
  if cmd == "层级":
    return some(interpreter.cmd_hierarchy())
  if cmd.startswith("裁判("):
    return some(interpreter.cmd_judge(cmd))
  if cmd.startswith("升元("):
    return some(interpreter.cmd_upgrade(cmd))
  if cmd.startswith("降元("):
    return some(interpreter.cmd_downgrade(cmd))

  return some("错误: 未知命令 " & cmd)

# ============================================================
# 一元
# ============================================================

proc cmd_one(interpreter: var YuanInterpreter, cmd: string): string =
  let paren_start = cmd.find('(')
  let paren_end = cmd.rfind(')')
  if paren_start < 0 or paren_end < 0:
    return "错误: 一元格式"

  let target = strip(cmd[paren_start+1..<paren_end])

  if interpreter.one_count < 64:
    set_arr(interpreter.ones[interpreter.one_count].target,
            interpreter.ones[interpreter.one_count].target_len, target)
    interpreter.one_count += 1

  result = "一元声明:\n"
  result.add("  目标: " & target & "\n")
  result.add("  性质: " & YUAN_ONE_DESC & "\n")
  result.add("  角色: " & YUAN_ONE_ROLE)

# ============================================================
# 二元
# ============================================================

proc cmd_two(interpreter: var YuanInterpreter, cmd: string): string =
  let paren_start = cmd.find('(')
  let paren_end = cmd.rfind(')')
  if paren_start < 0 or paren_end < 0:
    return "错误: 二元格式"

  let inner = cmd[paren_start+1..<paren_end]
  let args = inner.split(',')
  if args.len != 2:
    return "错误: 二元需要2个参数"

  let center = strip(args[0])
  let auxiliary = strip(args[1])

  if interpreter.two_count < 64:
    set_arr(interpreter.twos[interpreter.two_count].center,
            interpreter.twos[interpreter.two_count].center_len, center)
    set_arr(interpreter.twos[interpreter.two_count].auxiliary,
            interpreter.twos[interpreter.two_count].auxiliary_len, auxiliary)
    interpreter.two_count += 1

  result = "二元声明:\n"
  result.add("  中心: " & center & "\n")
  result.add("  辅助: " & auxiliary & "\n")
  result.add("  性质: " & YUAN_TWO_DESC & "\n")
  result.add("  角色: " & YUAN_TWO_ROLE)

# ============================================================
# 三元
# ============================================================

proc cmd_three(interpreter: var YuanInterpreter, cmd: string): string =
  let paren_start = cmd.find('(')
  let paren_end = cmd.rfind(')')
  if paren_start < 0 or paren_end < 0:
    return "错误: 三元格式"

  let inner = cmd[paren_start+1..<paren_end]
  let args = inner.split(',')
  if args.len != 3:
    return "错误: 三元需要3个参数"

  let observer = strip(args[0])
  let party_a = strip(args[1])
  let party_b = strip(args[2])

  if interpreter.three_count < 64:
    set_arr(interpreter.threes[interpreter.three_count].observer,
            interpreter.threes[interpreter.three_count].observer_len, observer)
    set_arr(interpreter.threes[interpreter.three_count].party_a,
            interpreter.threes[interpreter.three_count].party_a_len, party_a)
    set_arr(interpreter.threes[interpreter.three_count].party_b,
            interpreter.threes[interpreter.three_count].party_b_len, party_b)
    interpreter.three_count += 1

  result = "三元声明:\n"
  result.add("  观测者: " & observer & "\n")
  result.add("  甲方: " & party_a & "\n")
  result.add("  乙方: " & party_b & "\n")
  result.add("  性质: " & YUAN_THREE_DESC & "\n")
  result.add("  角色: " & YUAN_THREE_ROLE)

# ============================================================
# 四元
# ============================================================

proc cmd_four(interpreter: var YuanInterpreter, cmd: string): string =
  let paren_start = cmd.find('(')
  let paren_end = cmd.rfind(')')
  if paren_start < 0 or paren_end < 0:
    return "错误: 四元格式"

  let inner = cmd[paren_start+1..<paren_end]
  let args = inner.split(',')
  if args.len != 2:
    return "错误: 四元需要2个参数"

  let shell = strip(args[0])
  let content = strip(args[1])

  if interpreter.four_count < 64:
    set_arr(interpreter.fours[interpreter.four_count].shell,
            interpreter.fours[interpreter.four_count].shell_len, shell)
    set_arr(interpreter.fours[interpreter.four_count].content,
            interpreter.fours[interpreter.four_count].content_len, content)
    interpreter.four_count += 1

  result = "四元声明:\n"
  result.add("  外壳: " & shell & "\n")
  result.add("  内容: " & content & "\n")
  result.add("  性质: " & YUAN_FOUR_DESC & "\n")
  result.add("  角色: " & YUAN_FOUR_ROLE)

# ============================================================
# 层级结构
# ============================================================

proc cmd_hierarchy(interpreter: YuanInterpreter): string =
  result = "元层级结构:\n"
  result.add("==================================================\n")

  if interpreter.one_count > 0:
    result.add("【一元】(时间因果)\n")
    for i in 0..<interpreter.one_count:
      result.add("  ◆ " & get_arr(interpreter.ones[i].target,
                                    interpreter.ones[i].target_len) & "\n")
    result.add("\n")

  if interpreter.two_count > 0:
    result.add("【二元】(盈亏镜像)\n")
    for i in 0..<interpreter.two_count:
      let c = get_arr(interpreter.twos[i].center, interpreter.twos[i].center_len)
      let a = get_arr(interpreter.twos[i].auxiliary, interpreter.twos[i].auxiliary_len)
      result.add("  ◇ 中心:" & c & " ← 辅助:" & a & "\n")
    result.add("\n")

  if interpreter.three_count > 0:
    result.add("【三元】(见证镜像)\n")
    for i in 0..<interpreter.three_count:
      let o = get_arr(interpreter.threes[i].observer, interpreter.threes[i].observer_len)
      let a = get_arr(interpreter.threes[i].party_a, interpreter.threes[i].party_a_len)
      let b = get_arr(interpreter.threes[i].party_b, interpreter.threes[i].party_b_len)
      result.add("  ▷ 观测:" & o & " 裁定 " & a & " ? " & b & "\n")
    result.add("\n")

  if interpreter.four_count > 0:
    result.add("【四元】(包含镜像)\n")
    for i in 0..<interpreter.four_count:
      let s = get_arr(interpreter.fours[i].shell, interpreter.fours[i].shell_len)
      let c = get_arr(interpreter.fours[i].content, interpreter.fours[i].content_len)
      result.add("  □ 外壳:" & s & " ⟶ 内容:" & c & "\n")
    result.add("\n")

  if interpreter.one_count == 0 and interpreter.two_count == 0 and
     interpreter.three_count == 0 and interpreter.four_count == 0:
    result.add("  （暂无声明）")

# ============================================================
# 裁判
# ============================================================

proc cmd_judge(interpreter: var YuanInterpreter, cmd: string): string =
  let paren_start = cmd.find('(')
  let paren_end = cmd.rfind(')')
  if paren_start < 0 or paren_end < 0:
    return "错误: 裁判格式"

  let inner = cmd[paren_start+1..<paren_end]
  let args = inner.split(',')
  if args.len != 2:
    return "错误: 裁判需要2个参数"

  let a = strip(args[0])
  let b = strip(args[1])
  let relation = interpreter.find_relation(a, b)

  result = "裁判(" & a & ", " & b & "):\n"
  result.add("  甲方: " & a & "\n")
  result.add("  乙方: " & b & "\n")
  result.add("  关系: " & relation)

# ============================================================
# 升元
# ============================================================

proc cmd_upgrade(interpreter: var YuanInterpreter, cmd: string): string =
  let paren_start = cmd.find('(')
  let paren_end = cmd.rfind(')')
  if paren_start < 0 or paren_end < 0:
    return "错误: 升元格式"

  let element = strip(cmd[paren_start+1..<paren_end])

  result = "升元(" & element & "):\n"
  result.add("  元素: " & element & "\n")
  result.add("  升元路径: 一元 → 二元 → 三元 → 四元\n")
  result.add("  说明: 从中心目标开始，逐步被镜像、被裁判、被包含")

# ============================================================
# 降元
# ============================================================

proc cmd_downgrade(interpreter: var YuanInterpreter, cmd: string): string =
  let paren_start = cmd.find('(')
  let paren_end = cmd.rfind(')')
  if paren_start < 0 or paren_end < 0:
    return "错误: 降元格式"

  let element = strip(cmd[paren_start+1..<paren_end])

  result = "降元(" & element & "):\n"
  result.add("  元素: " & element & "\n")
  result.add("  降元路径: 四元 → 三元 → 二元 → 一元\n")
  result.add("  说明: 从包含外壳逐步回归中心目标")

# ============================================================
# 显示
# ============================================================

proc display_yuan*(interpreter: YuanInterpreter) =
  echo ""
  echo "============================================================"
  echo "元层级"
  echo "------------------------------------------------------------"
  echo "一元声明: ", interpreter.one_count
  echo "二元声明: ", interpreter.two_count
  echo "三元声明: ", interpreter.three_count
  echo "四元声明: ", interpreter.four_count
  echo "============================================================"
  echo ""

# ============================================================
# 入口
# ============================================================

when false:
  echo "============================================================"
  echo "FQM 元解释器测试"
  echo "============================================================"

  var myLoader: FQMLoader
  discard myLoader.load_file("containers.fqm")

  var myLinker: FQMLinker
  myLinker.loader = addr myLoader
  discard myLinker.build_ontology_index()
  discard myLinker.resolve_links()

  var yuan: YuanInterpreter
  yuan.linker = addr myLinker
  yuan.one_count = 0
  yuan.two_count = 0
  yuan.three_count = 0
  yuan.four_count = 0

  yuan.display_yuan()

  echo ""
  echo "元命令测试:"
  let commands = @[
    "一元(囝囝共生)",
    "二元(目标, 声母b)",
    "三元(满全法, 声母.b, 韵母.a)",
    "四元(声母容器, b)",
    "裁判(b, a)",
    "层级"
  ]

  for cmd in commands:
    let result = yuan.execute(cmd)
    if result.is_some:
      echo ""
      echo "  ", cmd
      for line in result.get().split('\n'):
        echo "    ", line
