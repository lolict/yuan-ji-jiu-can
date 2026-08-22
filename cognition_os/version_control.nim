## ═══════════════════════════════════════════════════════════════
## 版本控制系统 · 认知OS的演化引擎
## ═══════════════════════════════════════════════════════════════
## 规则:
##   1. 每次产出 = 一个新文件 · 从不修改旧文件
##   2. 命名: 子系统.版本号 (视觉.001 · 视觉.002 ...)
##   3. 用户审判: 对的✓ · 错的✗ · 奖励/惩罚
##   4. 历史全部保留 · 所有版本可对比
##   5. 通过了的版本才进入下一轮
## ═══════════════════════════════════════════════════════════════

import tables, strutils

type
  Version* = object
    id*: string          # "视觉.001"
    sub_system*: string  # "视觉"
    version*: int        # 1
    status*: string      # "待审判" / "通过" / "淘汰"
    reward*: float       # 奖励值
    penalty*: float      # 惩罚值
    judge_note*: string  # 审判备注
    created*: string     # 时间戳

  VersionControl* = object
    versions*: seq[Version]
    current_round*: int   # 当前轮次
    pass_count*: int      # 通过数
    fail_count*: int      # 淘汰数
    total_reward*: float
    total_penalty*: float

func new_version_control*(): VersionControl =
  result.current_round = 1
  result.pass_count = 0
  result.fail_count = 0
  result.total_reward = 0
  result.total_penalty = 0

proc next_version_id*(vc: var VersionControl; sub_system: string): string =
  var max_v = 0
  for v in vc.versions:
    if v.sub_system == sub_system and v.version > max_v:
      max_v = v.version
  let new_v = max_v + 1
  let v_str = align($new_v, 3, '0')
  vc.versions.add(Version(
    id: sub_system & "." & v_str,
    sub_system: sub_system,
    version: new_v,
    status: "待审判",
    reward: 0,
    penalty: 0,
    judge_note: "",
  ))
  result = sub_system & "." & v_str

proc judge*(vc: var VersionControl; id: string; correct: bool; note: string) =
  for i in 0..<vc.versions.len:
    if vc.versions[i].id == id:
      if correct:
        vc.versions[i].status = "通过"
        vc.versions[i].reward = 10.0
        vc.versions[i].penalty = 0
        vc.total_reward += 10.0
        vc.pass_count += 1
      else:
        vc.versions[i].status = "淘汰"
        vc.versions[i].reward = 0
        vc.versions[i].penalty = 5.0
        vc.total_penalty += 5.0
        vc.fail_count += 1
      vc.versions[i].judge_note = note
      break

proc get_sub_system_versions*(vc: VersionControl; sub_system: string): seq[Version] =
  for v in vc.versions:
    if v.sub_system == sub_system:
      result.add(v)

proc get_all_sub_systems*(vc: VersionControl): seq[string] =
  var seen = initTable[string, bool]()
  for v in vc.versions:
    if not seen.hasKey(v.sub_system):
      seen[v.sub_system] = true
      result.add(v.sub_system)

proc passed_versions*(vc: VersionControl): seq[Version] =
  for v in vc.versions:
    if v.status == "通过":
      result.add(v)

proc rejected_versions*(vc: VersionControl): seq[Version] =
  for v in vc.versions:
    if v.status == "淘汰":
      result.add(v)

proc report*(vc: VersionControl) =
  echo "═══ 版本控制报告 ═══"
  echo "轮次: ", vc.current_round
  echo "总版本: ", vc.versions.len
  echo "通过: ", vc.pass_count
  echo "淘汰: ", vc.fail_count
  echo "奖励: ", vc.total_reward
  echo "惩罚: ", vc.total_penalty
  echo "净收益: ", vc.total_reward - vc.total_penalty
  echo ""

  let subs = get_all_sub_systems(vc)
  for sub in subs:
    let vs = get_sub_system_versions(vc, sub)
    echo "── ", sub, " (", vs.len, "个版本) ──"
    for v in vs:
      echo "  ", v.id.alignLeft(12), " ", v.status.alignLeft(8), " ", v.judge_note

when is_main_module:
  var vc = new_version_control()

  # 各子系统版本1
  discard vc.next_version_id("视觉")
  discard vc.next_version_id("听觉")
  discard vc.next_version_id("审判")
  discard vc.next_version_id("天觉")
  discard vc.next_version_id("审美")
  discard vc.next_version_id("思考")
  discard vc.next_version_id("行为")
  discard vc.next_version_id("判断")
  discard vc.next_version_id("情进")
  discard vc.next_version_id("奖励")
  discard vc.next_version_id("处罚")
  discard vc.next_version_id("淘汰")
  discard vc.next_version_id("过滤")
  discard vc.next_version_id("记忆")

  # 审判一些版本
  vc.judge("视觉.001", false, "符号太丑 · 重做")
  vc.judge("听觉.001", false, "音值不对 · 重做")
  vc.judge("审判.001", true, "逻辑清晰")
  vc.judge("天觉.001", true, "层级正确")
  vc.judge("审美.001", false, "太复杂 · 简化")
  vc.judge("思考.001", true, "推理正确")
  vc.judge("行为.001", false, "动作不连贯")

  vc.report()
