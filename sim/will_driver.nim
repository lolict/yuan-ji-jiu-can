## ═══════════════════════════════════════════════════════════════
## will_driver.nim — 意志驱动 · 时间连续 · 生命周期管理
## ═══════════════════════════════════════════════════════════════
##
## 核心: 意志驱动时钟 · 时钟驱动生命周期 · 时间连续性
##       结束 = 开始 · 记忆跟随结束而继续
##       不停止思考 · 不停止动作

## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, algorithm, tables, times, lists]

type
  Will* = object
    desire*: string           # 意志方向
    intensity*: int           # 强度 1-10
    target*: string           # 目标
    priority*: int           # 优先级

  Event* = object
    name*: string
    timestamp*: int           # 时间戳
    duration*: int            # 持续时长
    next*: string             # 下一个事件(链条)

  Timeline* = object
    events*: seq[Event]
    current*: int             # 当前事件索引
    is_loop*: bool            # 是否循环

  WillDriver* = object
    will*: Will
    timeline*: Timeline
    memory*: seq[string]      # 记忆链
    generation*: int          # 代数
    is_alive*: bool           # 是否存活

# ═══════════════════════════════════════════════════════════════
## 初始化
# ═══════════════════════════════════════════════════════════════

proc newWillDriver*(desire: string, intensity: int = 5): WillDriver =
  result.will = Will(desire: desire, intensity: intensity, target: "", priority: 1)
  result.timeline = Timeline(events: @[], current: 0, is_loop: true)
  result.memory = @[]
  result.generation = 0
  result.is_alive = true

# ═══════════════════════════════════════════════════════════════
## 添加事件到时间线
# ═══════════════════════════════════════════════════════════════

proc add_event*(wd: var WillDriver, name: string, duration: int, next: string = "") =
  let event = Event(name: name, timestamp: wd.timeline.events.len, duration: duration, next: next)
  wd.timeline.events.add(event)

# ═══════════════════════════════════════════════════════════════
## 运行一个周期
# ═══════════════════════════════════════════════════════════════

proc step*(wd: var WillDriver): string =
  if not wd.is_alive or wd.timeline.events.len == 0:
    return "STOP"
  
  let event = wd.timeline.events[wd.timeline.current]
  result = event.name
  
  # 记忆
  wd.memory.add(event.name)
  
  # 下一个
  if event.next != "":
    # 查找下一个事件
    for i in 0..<wd.timeline.events.len:
      if wd.timeline.events[i].name == event.next:
        wd.timeline.current = i
        break
  else:
    # 顺序推进
    wd.timeline.current += 1
    if wd.timeline.current >= wd.timeline.events.len:
      if wd.timeline.is_loop:
        wd.timeline.current = 0
        wd.generation += 1
      else:
        wd.is_alive = false

# ═══════════════════════════════════════════════════════════════
## 运行N步
# ═══════════════════════════════════════════════════════════════

proc run*(wd: var WillDriver, steps: int): seq[string] =
  for i in 0..<steps:
    let output = wd.step()
    if output == "STOP":
      break
    result.add(output)

# ═══════════════════════════════════════════════════════════════
## 报告
# ═══════════════════════════════════════════════════════════════

proc report*(wd: WillDriver): string =
  var lines: seq[string] = @[]
  lines.add("═══ 意志驱动 · 时间连续 ═══")
  lines.add("  意志: " & wd.will.desire & "(强度" & $wd.will.intensity & ")")
  lines.add("  代数: " & $wd.generation)
  lines.add("  存活: " & $wd.is_alive)
  lines.add("  事件数: " & $wd.timeline.events.len)
  lines.add("")
  lines.add("  时间线:")
  for i, event in wd.timeline.events:
    let marker = if i == wd.timeline.current: "→" else: " "
    lines.add("  " & marker & " [" & $i & "] " & event.name & "(持续" & $event.duration & ")")
  lines.add("")
  lines.add("  记忆链(最近5):")
  let recent = wd.memory[^min(5, wd.memory.len)..^1]
  for m in recent:
    lines.add("    · " & m)
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
# ═══════════════════════════════════════════════════════════════

when isMainModule:
  var wd = newWillDriver("构建容器世界", intensity=8)
  
  echo "═══ 意志驱动验证 ═══"
  echo ""
  
  # 添加时间线事件
  wd.add_event("思考", 1, "存储")
  wd.add_event("存储", 1, "输出")
  wd.add_event("输出", 1, "感知")
  wd.add_event("感知", 1, "继承")
  wd.add_event("继承", 1, "思考")  # 循环
  
  # 运行20步
  let outputs = wd.run(20)
  
  for i, output in outputs:
    echo "  步骤" & $(i+1) & ": " & output
  
  echo ""
  echo wd.report()