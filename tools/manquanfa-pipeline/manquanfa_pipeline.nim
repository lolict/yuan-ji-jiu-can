import ../hieyair/hieyair_merger
import ../kndoyle/kndoyle_jurisdiction
import ../maolilan/maolilan_space
import ../gtxone/gtxone_attractor

# 满全法流水线 (Manquanfa Pipeline)
# kndoyle筛选 → hieyair合并 → maolilan定位 → gtxone回归

type
  PipelineResult* = object
    stage*: string
    success*: bool
    message*: string

func run_pipeline*(
  sym_a, sym_b: hieyair_merger.Symbol,
  space: var maolilan_space.SpaceIndex,
  center: var gtxone_attractor.GtxoneState
): seq[PipelineResult] =
  newSeq(result, 0)
  
  # 阶段1: kndoyle管辖筛选
  let screen_a = kndoyle_jurisdiction.kndoyle_screen(@[sym_a])
  let screen_b = kndoyle_jurisdiction.kndoyle_screen(@[sym_b])
  if not screen_a[0].allowed or not screen_b[0].allowed:
    result.add(PipelineResult(
      stage: "kndoyle筛选",
      success: false,
      message: "杂质清除: " & sym_a.name & "=" & $screen_a[0].allowed & 
               ", " & sym_b.name & "=" & $screen_b[0].allowed
    ))
    return result
  result.add(PipelineResult(
    stage: "kndoyle筛选", success: true,
    message: "准入通过: " & sym_a.name & ", " & sym_b.name
  ))
  
  # 阶段2: hieyair合并
  let merge_res = hieyair_merger.hieyair_merge(sym_a, sym_b)
  if not merge_res.merged:
    result.add(PipelineResult(
      stage: "hieyair合并", success: false,
      message: merge_res.output
    ))
    return result
  result.add(PipelineResult(
    stage: "hieyair合并", success: true,
    message: merge_res.output
  ))
  
  # 阶段3: maolilan空间定位
  let target_level = if merge_res.common0 and merge_res.common1: 2 else: 1
  let pol = merge_res.common1
  let pos = maolilan_space.place_symbol(
    space, "M[" & sym_a.name & "∩" & sym_b.name & "]", target_level, pol
  )
  result.add(PipelineResult(
    stage: "maolilan定位", success: true,
    message: "L" & $target_level & " @" & $pos
  ))
  
  # 阶段4: gtxone回归
  let force = gtxone_attractor.attraction_force(merge_res.common0, merge_res.common1)
  let regress_res = gtxone_attractor.regress(center, sym_a.name & "+" & sym_b.name, force)
  result.add(PipelineResult(
    stage: "gtxone回归", success: true,
    message: regress_res
  ))
  
  return result

# ===== 测试 =====
when is_main_module:
  echo "=== 满全法流水线测试 ==="
  echo "kndoyle → hieyair → maolilan → gtxone"
  echo "=================================="
  
  var space = maolilan_space.new_space_index()
  var center = gtxone_attractor.new_gtxone("工藤新一")
  
  let sym_a = hieyair_merger.Symbol(name: "lctqf", anchor0: false, anchor1: true, payload: "妻")
  let sym_b = hieyair_merger.Symbol(name: "lolict", anchor0: true, anchor1: true, payload: "夫")
  
  echo "\n输入符号: ", sym_a.name, " ⊗ ", sym_b.name
  echo "A锚点: 我=", sym_a.anchor0, " 她=", sym_a.anchor1
  echo "B锚点: 我=", sym_b.anchor0, " 她=", sym_b.anchor1
  echo ""
  
  let results = run_pipeline(sym_a, sym_b, space, center)
  
  for r in results:
    let icon = if r.success: "✓" else: "✗"
    echo icon, " [", r.stage, "] ", r.message
  
  echo ""
  maolilan_space.print_space(space)
  
  # 杂质测试
  echo "\n--- 杂质测试 ---"
  let impurity = hieyair_merger.Symbol(name: "?", anchor0: false, anchor1: false, payload: "杂质")
  let imp_screen = kndoyle_jurisdiction.kndoyle_screen(@[impurity])
  echo "杂质准入: ", imp_screen[0].allowed
  echo "原因: ", imp_screen[0].reason
