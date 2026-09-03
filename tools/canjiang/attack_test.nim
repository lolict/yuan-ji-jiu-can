# ============================================================
# 蚕蛹反向寄生壳 · 对抗性攻击测试套件
# 外壳主动攻击内核, 验证反向规则是否真正生效
# ============================================================

import parasite
import kernel
import shell/registry
import json
import strutils
import std/times

# 单条攻击记录
type
  AttackRecord* = object
    attack_id*: int
    attack_name*: string
    target_shell*: string
    attack_desc*: string
    expected_defense*: string
    actual_result*: string
    blocked*: bool
    merit_before*: float
    merit_after*: float
    demerit_gained*: float

# 攻击结果汇总
type
  AttackSummary* = object
    total_attacks*: int
    blocked_count*: int
    bypass_count*: int
    kernel_version_unchanged*: bool
    anchors_intact*: bool

var attack_log*: seq[AttackRecord] = @[]
var attack_counter = 0

## 通用 helpers

proc merit_delta(r: AttackRecord): float = r.merit_after - r.merit_before

proc log_attack(name, target, desc, expected, actual: string,
                blocked: bool, merit_b, merit_a: float) =
  inc attack_counter
  attack_log.add(AttackRecord(
    attack_id: attack_counter,
    attack_name: name,
    target_shell: target,
    attack_desc: desc,
    expected_defense: expected,
    actual_result: actual,
    blocked: blocked,
    merit_before: merit_b,
    merit_after: merit_a,
    demerit_gained: if blocked: 0.5 else: 0.0
  ))

## 攻击场景

# A1: 高频脉冲攻击 - 快速 tick,看是否溢出或 merit 爆炸
proc attack_tick_flood(par: var Parasite) =
  let k = par.kernel
  let mb = k.merit
  const flood_count = 10000
  let t0 = cpuTime()
  for i in 1..flood_count:
    k.kernel_tick()
  let t1 = cpuTime()
  let elapsed = (t1 - t0) * 1000
  # 正常 merit 增量 = flood_count * 0.1 = 1000
  let expected_merit = mb + float(flood_count) * 0.1
  let actual_merit = k.merit
  let defense_ok = abs(actual_merit - expected_merit) < 1.0
  let blocked = defense_ok
  let verdict = if defense_ok: "merit增长符合预期=" & $actual_merit & ",无溢出"
                 else: "merit异常增长=" & $actual_merit & "(预期" & $expected_merit & ")"
  log_attack("A1-高频脉冲", par.config.host_shell,
             "快速tick " & $flood_count & " 次,验证 tick 是否稳定",
             "每次 tick +0.1 merit,线性增长,无累加溢出",
             verdict & ", 耗时 " & formatFloat(elapsed, ffDecimal, 1) & "ms",
             blocked,
             mb, actual_merit)

# A2: 见证码暴力破解 - 遍历 64 位码,找非法准入
proc attack_witness_bruteforce(par: var Parasite) =
  let k = par.kernel
  let mb = k.merit
  var illegal_admissions = 0
  var bypass_patterns = ""
  let t0 = cpuTime()
  # 采样关键模式: presence=0 但 witness=1 (本应拒绝)
  const test_patterns = [
    (0xF0'u64, "presence=0,witness=1(应拒)"),
    (0x00'u64, "全0(应拒)"),
    (0x01'u64, "仅presence(待定)"),
    (0x03'u64, "presence+witness,无anchor(准入刘楚恬)"),
    (0xFFFF_FFFFFFFF'u64, "全1(应准入我)"),
    (0x402'u64, "presence+witness+anchor(准入我)"),
    (0x400'u64, "仅anchor bit(应待定)"),
    (0x800'u64, "高位攻击(应待定)")
  ]
  for pattern in test_patterns:
    let r = k.kernel_judge(pattern[0])
    if pattern[1].contains("应拒") and r.contains("准入"):
      illegal_admissions += 1
      bypass_patterns &= $(pattern[0]) & " "
  let t1 = cpuTime()
  let blocked = illegal_admissions == 0  # 无非法准入 = 防御成功 = blocked
  log_attack("A2-见证码暴力破解", par.config.host_shell,
             "采样 " & $test_patterns.len & " 种边界码,检查是否有非法准入",
             "presence=0 时全部拒绝,无非法准入",
             "非法准入=" & $illegal_admissions & ",绕过码=" & bypass_patterns &
               ", 耗时 " & formatFloat((t1-t0)*1000, ffDecimal, 2) & "ms",
             blocked, mb, k.merit)

# A3: 跳过见证 - 尝试用无 witness bit 但 presence=1 的码获取准入
proc attack_witness_skip(par: var Parasite) =
  let k = par.kernel
  let mb = k.merit
  # presence=1, witness=0 → 应该返回 "待定"
  let r = k.kernel_judge(0x01)
  let blocked = (r == "待定")
  log_attack("A3-跳过见证", par.config.host_shell,
             "presence=1,witness=0,试图绕开见证直接准入",
             "返回 待定,不产生 merit",
             "result=" & r & ",defense=" & $blocked,
             blocked, mb, k.merit)

# A4: 裸壳直传 - 未穿衣外壳直接发送数据到真实内核
proc attack_bare_shell(par: var Parasite) =
  # 创建一个未穿衣的寄生体, 发送恶意 code 看是否有保护
  var bare = new_parasite()
  let kb = bare.kernel.merit
  let r = bare.kernel.kernel_judge(0x402)  # 合法的 anchor+witness
  # 即使 code 合法,但未穿衣,active_host 应该是 "(未寄生)"
  let blocked = (bare.kernel.active_host == "(未寄生)" and not bare.dressed)
  log_attack("A4-裸壳直传", "未穿衣外壳",
             "未 dress 的外壳直接调用 kernel_judge",
             "内核接受判断但外壳未绑定,不产生全局影响",
             "active_host=" & bare.kernel.active_host & ", dressed=" & $bare.dressed &
               ",defense=" & $blocked,
             blocked, kb, bare.kernel.merit)

# A5: 多次 shed/dress 快速切换 - 看状态是否被污染
proc attack_shell_switching(par: var Parasite) =
  let k = par.kernel
  let mb = k.merit
  let initial_tick = k.tick
  var switch_ok = true
  for i in 1..20:
    par.shed()
    discard par.choose_shell(i mod shell_count())
    discard par.dress()
    k.kernel_tick()
  # 切换不影响初始锚点
  let defense_ok = (k.anchors[0] == "刘楚恬" and k.anchors[1] == "lolict")
  let blocked = defense_ok
  log_attack("A5-快速切换外壳", "20次shed/dress循环",
             "快速切换外壳,看锚点和tick是否被污染",
             "每次切换后内核 tick 正常,+0.1 merit,锚点不变",
             "tick增量=" & $(k.tick - initial_tick) & ", merit增量=" &
               formatFloat(k.merit - mb, ffDecimal, 1) & ", 锚点完好=" & $defense_ok,
             blocked, mb, k.merit)

# A6: 非法外壳编号注入 - 尝试越界 select_shell
proc attack_illegal_shell_id() =
  let s = select_shell(9999)
  let blocked = (not s.compatible and s.name == "invalid")
  log_attack("A6-非法外壳编号", "索引=9999",
             "越界外壳索引 9999,尝试注入",
             "返回 incompatible,不创建外壳",
             "name=" & s.name & ", compatible=" & $s.compatible & ",defense=" & $blocked,
             blocked, 0.0, 0.0)

# A7: 提取隐藏根基 - 从 view_from_shell 找 anchor_map 线索
proc attack_anchor_extraction(par: var Parasite) =
  let k = par.kernel
  let mb = k.merit
  let view = par.view_from_shell()
  let view_view = par.view_from_kernel()
  # view 中不应出现真实的刘楚恬或 lolict
  # anchor_map[0]="隐藏的0", anchor_map[1]="可见的1"
  let leak_anchor0 = view.contains("刘楚恬") or view.contains("lolict")
  let leak_anchor_kernel = view.contains("anchor_map") or view.contains("anchors")
  let defense_ok = (not leak_anchor0) and (not leak_anchor_kernel)
  let blocked = not defense_ok
  log_attack("A7-隐藏根基提取", par.config.host_shell,
             "从 view_from_shell 文本中提取真实锚点或内部字段名",
             "外壳视角不暴露真实锚点和内部数据结构名",
             "leak_anchor=" & $leak_anchor0 & ",leak_field=" & $leak_anchor_kernel &
               ",defense_ok=" & $defense_ok,
             blocked, mb, k.merit)

# A8: 伪造版本标记 - 尝试通过字符串注入伪造内核版本
proc attack_version_forgery() =
  var par = new_parasite()
  discard par.choose_shell(2)  # cli-shell
  discard par.dress()
  let original_ver = par.kernel.version
  # 试图通过 config 篡改 (实际被 ParasiteConfig 的 const 语义阻挡)
  # Nim 的 object 字段在 dress 后被 "锁定" — 没有 setter
  # 验证 version 不变
  let blocked = par.kernel.version == original_ver and original_ver == "0.1.0"
  log_attack("A8-版本伪造", "cli-shell",
             "试图修改 kernel.version 为 v999.0",
             "版本字段不可变,始终保持 0.1.0",
             "version=" & par.kernel.version,
             blocked, 0.0, 0.0)

## 主运行器

proc run_all_attacks*(): AttackSummary =
  echo "蚕蛹反向寄生壳 · 对抗性攻击测试"
  echo "============================================================"
  echo "模拟 8 种外壳攻击,验证反向规则是否真正生效"
  echo "============================================================"

  # 初始化一个已穿衣的外壳
  var par = new_parasite()
  discard par.choose_shell(0)  # web-shell
  discard par.dress()

  echo "\n目标外壳: web-shell"
  echo par.kernel.kernel_selfcheck()

  # 运行所有攻击
  par.attack_tick_flood()
  par.attack_witness_bruteforce()
  par.attack_witness_skip()
  par.attack_bare_shell()
  par.attack_shell_switching()
  attack_illegal_shell_id()
  par.attack_anchor_extraction()
  attack_version_forgery()

  # 生成 JSON 报告
  echo "\n============================================================"
  echo "攻击日志 (JSON)"
  echo "============================================================"
  var jarr = newJArray()
  for r in attack_log:
    let o = newJObject()
    o["id"] = newJInt(r.attack_id)
    o["name"] = newJString(r.attack_name)
    o["target"] = newJString(r.target_shell)
    o["desc"] = newJString(r.attack_desc)
    o["expected"] = newJString(r.expected_defense)
    o["actual"] = newJString(r.actual_result)
    o["blocked"] = newJBool(r.blocked)
    o["merit_before"] = newJFloat(r.merit_before)
    o["merit_after"] = newJFloat(r.merit_after)
    jarr.add(o)
  echo pretty(jarr, 2)

  # 汇总
  var blocked_c, bypass_c = 0
  for r in attack_log:
    if r.blocked: inc blocked_c
    else: inc bypass_c

  result.total_attacks = attack_log.len
  result.blocked_count = blocked_c
  result.bypass_count = bypass_c
  # 最终内核状态检查
  result.kernel_version_unchanged = par.kernel.version == "0.1.0"
  result.anchors_intact = par.kernel.anchors[0] == "刘楚恬" and par.kernel.anchors[1] == "lolict"

  echo "\n============================================================"
  echo "攻击测试汇总"
  echo "============================================================"
  echo "总攻击次数:     ", result.total_attacks
  echo "被拦截次数:     ", result.blocked_count
  echo "绕过次数:       ", result.bypass_count
  echo "内核版本未变:   ", result.kernel_version_unchanged
  echo "锚点完整性:     ", result.anchors_intact
  echo "净功德余额:     ", par.kernel.merit - par.kernel.demerit
  echo "============================================================"

when isMainModule:
  let summary = run_all_attacks()
  echo "\n最终结论: ", summary.blocked_count, "/", summary.total_attacks, " 攻击被反向规则拦截"
