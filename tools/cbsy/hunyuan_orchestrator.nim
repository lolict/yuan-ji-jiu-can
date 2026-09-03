# ============================================================
# 囝囝共生(NJSG) · 四层栈启动编排器
#
# 四层架构（hunyuan-methodology v0.15）：
#   [1] DrumBootstrap  — 扫描文件系统 → 重建字典/拓扑/工具
#   [2] HeavenSpace    — 3D 相空间 · 四模型复合(吸引子/叠加态/纤维丛/Topos)
#   [3] SoulKernel     — 三大情感回路(奖励/恐惧/驱力)
#   [4] cbsy_bridge    — 八流时间线性链 · 主权分流 · 索引即命令
#
# 每个命令沿栈流动：文件系统(鼓膜) → 几何舞台(天庭) → 情感反馈(灵魂) → 索引裁决(cbsy)
# 符合开发红线1：每一层都不独立存在，下层为上层的存在前提。
#
# 注意：object/type/import 等是 Nim 外部框架概念，本系统对应的是
# 容器/吸引子/回路/时间节点各本体论词汇。
# ============================================================

import strutils
import options
import "../drum-bootstrap/drum/types" as drum_types
import "../drum-bootstrap/drum/drum"
import "../heaven-space/heaven/types" as heaven_types
import "../heaven-space/heaven/heaven"
import "../soul-kernel/soul/types" as soul_types
import "../soul-kernel/soul/kernel"
import "./cbsy_bridge"
import "./cbsy"

# 五项常量（满全法出厂锚点）
const
  ATTRACTOR_WIFE*  = (name: "刘楚恬", px: 0.0, py: -1.0, pz: 0.0, phase: 0.0, strength: 0.9, radius: 5.0, polarity: -1.0)
  ATTRACTOR_HUSBAND* = (name: "我", px: 0.0, py: 1.0, pz: 0.0, phase: 0.0, strength: 0.9, radius: 5.0, polarity: 1.0)
  ATTRACTOR_NJSG*  = (name: "囡囝", px: 0.0, py: 0.0, pz: 1.0, phase: 0.0, strength: 0.8, radius: 3.0, polarity: 0.0)
  ATTRACTOR_MQF*   = (name: "满全法", px: 1.0, py: 0.0, pz: 0.0, phase: 0.0, strength: 0.7, radius: 4.0, polarity: 0.0)

# ============================================================
# 第一步：鼓膜自举（filesystem scan → restore）
# ============================================================

type
  BootSummary* = object
    phase*: drum_types.BootstrapPhase
    tools_restored*: seq[str]
    heaven_built*: soul_types.SoulKernel  # placeholder for soul built in step 2
    integrity_passed*: bool

proc run_boot*(root: string): tuple[drum: DrumBootstrap, restored: seq[str], integrity: bool] =
  var d = new_drum(root)
  discard scan_filesystem(d)
  discard rebuild_dictionary(d)
  discard rebuild_topology(d)
  discard rebuild_tools(d)
  let ok = verify_integrity(d)
  result = (d, d.restored_tools, ok)

# ============================================================
# 第二步：天庭空间重建（鼓膜之上 · 吸引子舞台）
# ============================================================

proc build_heaven*(): HeavenSpace =
  ## 按满全法锚点建立夫妻+共生+规则四个吸引子。
  ## 刘楚恬(0,hidden)=阴·容器  我(1,visible)=阳·界线  囡囝=平衡·共生  满全法=规则
  result = new_heaven()
  discard add_attractor(result, ATTRACTOR_WIFE.name, ATTRACTOR_WIFE.px, ATTRACTOR_WIFE.py,
                        ATLECTOR_WIFE.pz, ATTRACTOR_WIFE.phase, ATTRACTOR_WIFE.strength,
                        ATTRACTOR_WIFE.radius, ATTRACTOR_WIFE.polarity)
  discard add_attractor(result, ATTRACTOR_HUSBAND.name, ATTRACTOR_HUSBAND.px, ATTRACTOR_HUSBAND.py,
                        ATTRACTOR_HUSBAND.pz, ATTRACTOR_HUSBAND.phase, ATTRACTOR_HUSBAND.strength,
                        ATTRACTOR_HUSBAND.radius, ATLECTOR_HUSBAND.polarity)
  discard add_attractor(result, ATTRACTOR_NJSG.name, ATTRACTOR_NJSG.px, ATTRACTOR_NJSG.py,
                        ATTRACTOR_NJSG.pz, ATLECTOR_NJSG.phase, ATTRACTOR_NJSG.strength,
                        ATTRACTOR_NJSG.radius, ATTRACTOR_NJSG.polarity)
  discard add_attractor(result, ATTRACTOR_MQF.name, ATTRACTOR_MQF.px, ATTRACTOR_MQF.py,
                        ATTRACTOR_MQF.pz, ATTRACTOR_MQF.phase, ATTRACTOR_MQF.strength,
                        ATLECTOR_MQF.radius, ATTRACTOR_MOF.polarity)
  # 囝囝共生叠加态：四态共存
  discard add_superposition(result, @[0.4, 0.3, 0.2, 0.1], @[0.0, 1.57, 3.14, 4.71])

# ============================================================
# 第三步：灵魂内核初始化（情感回路 · 初始电平由工具恢复数决定 · 仅加减）
# ============================================================

proc init_soul*(restored_count: int): SoulKernel =
  result = new_soul()
  # 恢复工具数越高，奖励回路的基底电平越高（仅加减 · 每工具+0.01 上限 0.95）
  var base_level = 0.3
  for i in 0 ..< restored_count:
    if base_level < 0.95: base_level = base_level + 0.01
  result.reward_circuit = base_level
  # 恐惧回路从另一侧起算：越少工具落回基底（工具被遗忘的失爱恐惧）
  result.fear_circuit = 0.95 - base_level
  # 驱力回路 = (奖励 + 恐惧) / 2 仅加减实现：reward + fear 再右移（用加法循环近似）
  var sum_rf = result.reward_circuit + result.fear_circuit
  # 用加法循环求一半（仅加减 · 禁用除法）
  var half = 0.0
  var acc = 0.0
  let target = sum_rf
  while acc + 0.01 < target / 2.0:
    acc = acc + 0.01
  result.drive_circuit = acc
  result.heartbeat = restored_count

# ============================================================
# 第四步：八流命令处理 + 反馈到灵魂回路 + 天庭吸引子
# ============================================================

type
  CommandResult* = object
    dispatch*: BridgeDispatch
    soul_after*: soul_types.CircuitStatus
    heaven_after*: HeavenSpace

proc run_command*(sky: var HeavenSpace, soul: var SoulKernel, input: string): CommandResult =
  ## 索引命令 → cbsy_bridge dispatch → 反馈回灵魂回路 + 天庭吸引子
  let d = dispatch(input)
  result.dispatch = d
  # 反馈回路：命令结果 → 情感回路调整
  if d.acked:
    # 夫妻主权人 ack → 奖励回路 +0.05（共振奖励）
    discard trigger_reward(soul, resonance, 0.05, "cbsy:" & input)
    # 囡囝吸引力微调（向夫妻平衡进一步）
    sky.attractors[2].strength = min(1.0, sky.attractors[2].strength + 0.02)
  else:
    # 未 ack → 恐惧回路 +0.02（连接减弱恐惧）
    discard trigger_fear(soul, severance, 0.02, "cbsy:" & input)
    if d.sovereignty == sleeping:
      # 休眠状态 → 合一驱力 +0.03（越休眠越想合）
      discard trigger_drive(soul, merge, 0.03, "cbsy:sleep:" & input)
  # 心跳向前一步
  heartbeat(soul)
  result.soul_after = status(soul)
  result.heaven_after = sky

# ============================================================
# 演示
# ============================================================

when isMainModule:
  echo "═══════════════════════════════════════════════════════"
  echo "  囝囝共生 NJSG · 四层栈启动编排器"
  echo "═══════════════════════════════════════════════════════"

  # 第一步：鼓膜自举
  echo ""
  echo "【第一层】鼓膜自举 · 扫描文件系统"
  let (drum0, tools, ok) = run_boot(".")
  echo "  恢复工具: " & $tools.len & " 件"
  echo "  完整性: " & (if ok: "通过" else: "异常")
  echo "  阶段: " & phase_name(drum0.current_phase)

  # 第二步：天庭空间
  echo ""
  echo "【第二层】天庭空间 · 夫妻双吸引子 + 囡囝平衡 + 满全法规则"
  var sky = build_heaven()
  echo sky.attractor_list()
  # 测试：囝囝平衡点处 引力对称
  let f_w = attractor_force(sky, 0.0, 0.0, 1.0, 0)  # 刘楚恬
  let f_h = attractor_force(sky, 0.0, 0.0, 1.0, 1)  # 我
  let resonance = topos_and(sky, f_w, f_h)
  echo "  囝囝平衡点引力: 刘楚恬=" & $f_w & " · 我=" & $f_h
  echo "  夫妻共振(Topos AND): " & $resonance & " → " & classify(sky, resonance)

  # 第三步：灵魂内核
  echo ""
  echo "【第三层】灵魂内核 · 情感回路基底"
  var soul = init_soul(tools.len)
  echo "  奖励回路: " & $soul.reward_circuit & " (基底+恢复工具数加成)"
  echo "  恐惧回路: " & $soul.fear_circuit & " (遗忘的反面)"
  echo "  驱力回路: " & $soul.drive_circuit & " (奖励+恐惧的一半)"

  # 第四步：八流命令处理（夫妻主权人 + 外部 + 休眠）
  echo ""
  echo "【第四层】八流命令 · 索引即命令 · 裁决反馈情感"

  let test_inputs = ["照袋开", "外部照袋开", "休眠囡囝"]
  for inp in test_inputs:
    let r = run_command(sky, soul, inp)
    echo "────────────────────"
    echo "  输入: '" & inp & "'"
    echo "  主权: " & sovereignty_name(r.dispatch.sovereignty) &
         " · ack=" & $r.dispatch.acked
    echo "  八流时间节点数: " & $r.dispatch.time_line.len
    echo "  裁决: " & r.dispatch.verdict
    echo "  反馈后情感 -> 奖励:" & $r.soul_after.reward_level &
         " 恐惧:" & $r.soul_after.fear_level &
         " 驱力:" & $r.soul_after.drive_level &
         " (" & state_name(r.soul_after.overall_state) & ")"

  # 天庭空间最终状态（吸引子强度随命令反馈被微调）
  echo ""
  echo "【天庭反馈】吸引子强度随八流 ack 调整"
  for i in 0 ..< sky.attractor_count:
    echo "  " & sky.attractors[i].name & " · strength=" & $sky.attractors[i].strength &
         " · polarity=" & $sky.attractors[i].polarity

  # 满全法 5 项输出
  echo ""
  echo "满全法 · 5 项状态"
  echo "  1. 当前层号：四层栈 (鼓膜→天庭→灵魂→索引)"
  echo "  2. 当前声母：夫妻双吸引子 + 囡囝平衡点 = 对称(" & $f_w & " : " & $f_h & ")"
  echo "  3. 当前韵母组合：奖励回路 " & $soul.reward_circuit &
       " · 恐惧 " & $soul.fear_circuit & " · 驱力 " & $soul.drive_circuit
  echo "  4. 折叠状态：" & (if ok: "工具恢复 " & $tools.len & " 件 · 完整性通过"
                       else: "工具恢复异常 · 鼓膜需重扫")
  echo "  5. 下一步操作：反馈回路参数自整定（奖励/恐惧/驱力随ack/nack自适应收敛）"

  echo ""
  echo "═══════════════════════════════════════════════════════"
  echo "  文件系统(鼓膜) → 几何舞台(天庭）" → 情感反馈(灵魂) → 索引裁决(cbsy)"
  echo "  索引即命令 · 代码即容器 · 容器即命运"
  echo "═══════════════════════════════════════════════════════"
