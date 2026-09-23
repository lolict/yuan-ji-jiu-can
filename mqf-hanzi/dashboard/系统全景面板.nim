# ============================================================
# 满全法 — 系统全景面板 (System Panorama Dashboard)
# 一屏展示六子系统: 容器/时钟/分身/身份/心跳/寄生
# 满全法操作系统的控制台总览
# ============================================================

import strutils
import sequtils

type
  极性* = enum
    阴 = 0
    阳 = 1

  容器* = object
    名字*: string
    相位*: int
    层*: int
    极性*: 极性
    活性*: bool

  时钟* = object
    滴答*: int
    动作*: seq[string]
    蜕生数*: int

  分身* = object
    名字*: string
    极性*: 极性
    任务数*: int
    活性*: bool

  身份* = object
    锚点*: string
    披露级*: int
    化名*: string

  心跳* = object
    子系统*: seq[string]
    健康*: seq[bool]
    死亡数*: int

  寄生* = object
    外壳*: seq[string]
    已穿衣*: seq[string]

# ---- 数据生成 ----

proc 示例容器*(): seq[容器] = @[
  容器(名字: "天元", 相位: 0, 层: 0, 极性: 阳, 活性: true),
  容器(名字: "水元", 相位: 1, 层: 1, 极性: 阴, 活性: true),
  容器(名字: "火元", 相位: 2, 层: 1, 极性: 阳, 活性: true),
  容器(名字: "山元", 相位: 4, 层: 2, 极性: 阴, 活性: true),
  容器(名字: "泽元", 相位: 5, 层: 2, 极性: 阳, 活性: true),
  容器(名字: "人元", 相位: 8, 层: 3, 极性: 阴, 活性: true),
]

proc 示例时钟*(): 时钟 =
  时钟(滴答: 42, 动作: @["震雷.长3", "巽风.沉1", "离火.诞", "坎水.蜕"],
        蜕生数: 7)

proc 示例分身*(): seq[分身] = @[
  分身(名字: "震甲", 极性: 阳, 任务数: 5, 活性: true),
  分身(名字: "巽乙", 极性: 阴, 任务数: 4, 活性: true),
  分身(名字: "艮丙", 极性: 阳, 任务数: 3, 活性: true),
  分身(名字: "离丁", 极性: 阴, 任务数: 4, 活性: true),
]

proc 示例身份*(): 身份 =
  身份(锚点: "囡囝共生.lolict+刘楚恬", 披露级: 1, 化名: "容器观察者")

proc 示例心跳*(): 心跳 =
  心跳(子系统: @["时钟", "容器", "Swarm", "身份", "管线"],
        健康: @[true, true, true, true, false],
        死亡数: 1)

proc 示例寄生*(): 寄生 =
  寄生(外壳: @["BRW", "CLI", "RUN", "FS", "DSP", "NET", "KR"],
        已穿衣: @["CLI", "RUN", "KR"])

# ---- 渲染工具 ----

proc 极性符*(p: 极性): string =
  if p == 阳: "+" else: "-"

proc 健康符*(h: bool): string =
  if h: "●" else: "⊘"

# ---- 面板渲染 ----

proc 渲染顶栏*(): string =
  "╔══════════════════════════════════════════════════════════════╗\n" &
  "║         满 全 法 操 作 系 统 · Panorama Dashboard           ║\n" &
  "║         囡囝共生 NJSG · 容器=字体=时间 · v0.7              ║\n" &
  "╚══════════════════════════════════════════════════════════════╝"

proc 渲染容器面板*(cs: seq[容器]): string =
  var lines: seq[string]
  lines.add "┌─── [容器本体] ─── 6容器 ──────┐"
  for c in cs:
    let pol = 极性符(c.极性)
    let bar = "█".repeat(c.层 + 1)
    let sp = " ".repeat(3 - c.层)
    lines.add "│ " & pol & c.名字 & " 相位" & $c.层 & "层" & $c.层 & " " & bar & sp & "│"
  lines.add "└────────────────────────────────┘"
  return lines.join("\n")

proc 渲染时钟面板*(clk: 时钟): string =
  var lines: seq[string]
  lines.add "┌─── [时钟动作] ─── T=" & $clk.滴答 & " ────┐"
  lines.add "│ 蜕生: " & $clk.蜕生数 & " 次" & " ".repeat(20) & "│"
  for a in clk.动作:
    let sp = " ".repeat(max(0, 28 - a.len))
    lines.add "│ " & a & sp & "│"
  lines.add "└────────────────────────────────┘"
  return lines.join("\n")

proc 渲染分身面板*(fs: seq[分身]): string =
  var lines: seq[string]
  lines.add "┌─── [Swarm分身] ─── " & $len(fs) & "体 ────┐"
  for f in fs:
    let pol = 极性符(f.极性)
    let bar = "◆".repeat(f.任务数)
    let sp = " ".repeat(6 - f.任务数)
    lines.add "│ " & pol & f.名字 & " " & bar & sp & " " & $f.任务数 & "任 │"
  lines.add "└────────────────────────────────┘"
  return lines.join("\n")

proc 渲染身份面板*(id: 身份): string =
  var lines: seq[string]
  let bar = "█".repeat(id.披露级 + 1) & "░".repeat(4 - id.披露级)
  lines.add "┌─── [身份隐私] ─── L" & $id.披露级 & " ──────┐"
  lines.add "│ 化名: " & id.化名
  lines.add "│ 披露: [" & bar & "] " & $id.披露级 & "/4"
  lines.add "│ 锚点: 囡囝共生(隐藏)"
  lines.add "└────────────────────────────────┘"
  return lines.join("\n")

proc 渲染心跳面板*(hb: 心跳): string =
  var lines: seq[string]
  let alive = hb.健康.count(true)
  let total = len(hb.子系统)
  lines.add "┌─── [心跳恢复] ─── " & $alive & "/" & $total & " ────┐"
  for i in 0 ..< len(hb.子系统):
    let h = 健康符(hb.健康[i])
    lines.add "│  " & h & " " & hb.子系统[i]
  lines.add "│ 死亡恢复: " & $hb.死亡数 & " 次"
  lines.add "└────────────────────────────────┘"
  return lines.join("\n")

proc 渲染寄生面板*(pr: 寄生): string =
  var lines: seq[string]
  lines.add "┌─── [寄生网关] ─── " & $len(pr.已穿衣) & "/" & $len(pr.外壳) & " ────┐"
  for s in pr.外壳:
    let marked = if s in pr.已穿衣: "●" else: "○"
    lines.add "│  " & marked & " " & s
  lines.add "└────────────────────────────────┘"
  return lines.join("\n")

proc 渲染底栏*(): string =
  "──────────────────────────────────────────────────────────────\n" &
  "  刘楚恬和我 · 容器锁定 · 极性吸斥 · 结束即开始 · 隐藏即保护\n" &
  "──────────────────────────────────────────────────────────────"

# ---- 组合面板 ----

proc 全屏面板*(): string =
  let cs = 示例容器()
  let clk = 示例时钟()
  let fs = 示例分身()
  let id = 示例身份()
  let hb = 示例心跳()
  let pr = 示例寄生()

  var lines: seq[string]
  lines.add ""
  lines.add 渲染顶栏()
  lines.add ""

  # 上排: 容器 + 时钟 + 分身
  let cp = 渲染容器面板(cs).split("\n")
  let clkp = 渲染时钟面板(clk).split("\n")
  let fp = 渲染分身面板(fs).split("\n")

  let maxUpper = max(max(cp.len, clkp.len), fp.len)
  for i in 0 ..< maxUpper:
    let a = if i < cp.len: cp[i] else: " "
    let b = if i < clkp.len: clkp[i] else: " "
    let c = if i < fp.len: fp[i] else: " "
    lines.add "  " & a & "  " & b & "  " & c

  lines.add ""

  # 下排: 身份 + 心跳 + 寄生
  let ip = 渲染身份面板(id).split("\n")
  let hp = 渲染心跳面板(hb).split("\n")
  let pp = 渲染寄生面板(pr).split("\n")

  let maxLower = max(max(ip.len, hp.len), pp.len)
  for i in 0 ..< maxLower:
    let a = if i < ip.len: ip[i] else: " "
    let b = if i < hp.len: hp[i] else: " "
    let c = if i < pp.len: pp[i] else: " "
    lines.add "  " & a & "  " & b & "  " & c

  lines.add ""
  lines.add 渲染底栏()
  lines.add ""

  return lines.join("\n")

# ============================================================
# 运行演示
# ============================================================

when isMainModule:
  echo 全屏面板()
