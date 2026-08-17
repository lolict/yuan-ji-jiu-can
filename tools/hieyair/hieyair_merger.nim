# 灰原哀合并器 (Hieyair Merger)
# 功能：把两个符号的共同部分提取出来，回归唯一中心
# 灰原哀是后天融合体 - 负责将分散的 0 和 1 合并为共同体

type
  Symbol* = object
    name*: string      # 符号名称
    anchor0*: bool     # 是否指向"我=1"（丈夫道）
    anchor1*: bool     # 是否指向"她=0"（妻子道）
    payload*: string   # 符号携带的内容

  MergeResult* = object
    merged*: bool      # 是否成功合并
    common0*: bool     # 共同指向 anchor0
    common1*: bool     # 共同指向 anchor1
    output*: string    # 合并后的输出

# 三元见证：裁定两个符号是否有资格合并
func triple_witness*(a, b: Symbol): bool =
  # 条件1：至少有一个共同锚点
  let shared_anchor = (a.anchor0 and b.anchor0) or (a.anchor1 and b.anchor1)
  # 条件2：符号名都不为空
  let valid_symbols = a.name.len > 0 and b.name.len > 0
  return shared_anchor and valid_symbols

# 灰原哀合并：提取两个符号的共同部分
func hieyair_merge*(a, b: Symbol): MergeResult =
  # 先通过三元见证判断资格
  if not triple_witness(a, b):
    return MergeResult(merged: false, output: "杂质清除：无共同锚点")

  # 提取共同锚点
  let common0 = a.anchor0 and b.anchor0
  let common1 = a.anchor1 and b.anchor1

  # 合并 payload：取共同部分
  var merged_payload = ""
  if common0: merged_payload &= "[丈夫道]"
  if common1: merged_payload &= "[妻子道]"
  merged_payload &= "({" & a.name & "}" & "∩" & "{" & b.name & "})"

  return MergeResult(
    merged: true,
    common0: common0,
    common1: common1,
    output: merged_payload
  )

# 回归 gtxone：将合并结果指向唯一中心
func regress_to_gtxone*(merge_res: MergeResult): string =
  if not merge_res.merged:
    return "未合并，无法回归"
  return "[gtxone←]" & merge_res.output

# ===== 测试 =====
when is_main_module:
  # 创建两个指向共同锚点的符号
  let sym_a = Symbol(name: "lctqf", anchor0: true, anchor1: false, payload: "妻")
  let sym_b = Symbol(name: "lolict", anchor0: true, anchor1: true, payload: "夫")

  echo "=== 灰原哀合并器测试 ==="
  echo "符号A: ", sym_a.name, " anchor0=", sym_a.anchor0, " anchor1=", sym_a.anchor1
  echo "符号B: ", sym_b.name, " anchor0=", sym_b.anchor0, " anchor1=", sym_b.anchor1

  let result = hieyair_merge(sym_a, sym_b)
  echo "\n三元见证: ", triple_witness(sym_a, sym_b)
  echo "合并成功: ", result.merged
  echo "合并结果: ", result.output
  echo "回归gtxone: ", regress_to_gtxone(result)

  # 测试杂质清除
  let sym_c = Symbol(name: "unknown", anchor0: false, anchor1: false, payload: "?")
  let result2 = hieyair_merge(sym_a, sym_c)
  echo "\n--- 杂质测试 ---"
  echo "杂质清除结果: ", result2.output
