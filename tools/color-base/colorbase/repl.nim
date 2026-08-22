# ============================================================
# 颜色进制 · REPL 解释器
# 交互式命令行，解释执行颜色进制操作
# ============================================================

import phase
import canvas
import render
import strutils

type
  CompEntry = object
    name*: string
    canvas*: PhaseCanvas

  ReplState = object
    canvas*: PhaseCanvas
    running*: bool
    comp_store*: seq[CompEntry]  # 组件库

# ------------------------------------------------------------
# 帮助信息
# ------------------------------------------------------------
func repl_help*(): string =
  result = """
颜色进制 REPL · 可用命令
─────────────────────────────────
  new <w> <h>          创建新画布
  show                 显示画布摘要
  preview              ASCII 预览
  set <x> <y> <phase> <weight>  设置单元格
  fill <phase> <weight> <level> 填充空白区域
  comp <name> <w> <h>  创建组件
  cset <name> <x> <y> <phase> <weight>  设置组件单元
  paste <name> <dx> <dy>  粘贴组件到画布
  erase <dx> <dy> <w> <h>   减法:挖除区域
  level <x> <y> <L1|L2|L3>  设置层级
  info <x> <y>         单元格详情
  export <path.ppm>    导出为 PPM 图像
  report               元数据报告
  log                  操作日志
  help                 本文档
  quit                 退出
─────────────────────────────────
示例:
  new 32 16
  fill 10 0.8 L1
  comp tone 3 3
  cset tone 1 1 50 1.0
  paste tone 5 5
  export out.ppm
"""

# ------------------------------------------------------------
# 查找组件
# ------------------------------------------------------------
func find_comp(state: var ReplState, name: string): int =
  for i, c in state.comp_store:
    if c.name == name: return i
  return -1

# ------------------------------------------------------------
# 执行单条命令
# ------------------------------------------------------------
proc repl_exec(state: var ReplState, line: string): string {.sideEffect, raises: [].} =
  let parts = line.strip().splitWhitespace()
  if parts.len == 0: return ""
  let cmd = parts[0].to_lower()

  try:
    case cmd
    of "new":
      if parts.len < 3: return "用法: new <w> <h>"
      state.canvas = new_phase_canvas(parse_int(parts[1]), parse_int(parts[2]))
      return "画布已创建: " & canvas_summary(state.canvas)

    of "show":
      return canvas_summary(state.canvas)

    of "preview":
      return ascii_preview(state.canvas)

    of "set":
      if parts.len < 5: return "用法: set <x> <y> <phase> <weight>"
      let x = parse_int(parts[1])
      let y = parse_int(parts[2])
      let ph = parse_int(parts[3])
      let wt = parse_float(parts[4])
      state.canvas.set(x, y, make_cell(ph, wt))
      return "OK (" & $x & "," & $y & ") 相位=" & $ph & " 权重=" & $wt

    of "fill":
      if parts.len < 4: return "用法: fill <phase> <weight> <L1|L2|L3>"
      let ph = parse_int(parts[1])
      let wt = parse_float(parts[2])
      let lvl = if parts[3].to_lower().startswith("l2"): plL2
                elif parts[3].to_lower().startswith("l3"): plL3
                else: plL1
      var cnt = 0
      for y in 0..<state.canvas.height:
        for x in 0..<state.canvas.width:
          if not state.canvas.cells[y][x].present:
            state.canvas.set(x, y, make_cell(ph, wt, lvl))
            cnt += 1
      return "填充 " & $cnt & " 单元"

    of "comp":
      if parts.len < 4: return "用法: comp <name> <w> <h>"
      let name = parts[1]
      let w = parse_int(parts[2])
      let h = parse_int(parts[3])
      let idx = state.find_comp(name)
      let entry = CompEntry(name: name, canvas: new_phase_canvas(w, h))
      if idx >= 0:
        state.comp_store[idx] = entry
      else:
        state.comp_store.add(entry)
      return "组件 '" & name & "' 已创建: " & $w & "×" & $h

    of "cset":
      if parts.len < 6: return "用法: cset <comp> <x> <y> <phase> <weight>"
      let name = parts[1]
      let idx = state.find_comp(name)
      if idx < 0: return "组件不存在: " & name
      let x = parse_int(parts[2])
      let y = parse_int(parts[3])
      let ph = parse_int(parts[4])
      let wt = parse_float(parts[5])
      state.comp_store[idx].canvas.set(x, y, make_cell(ph, wt))
      return "OK " & name & "(" & $x & "," & $y & ")"

    of "paste":
      if parts.len < 4: return "用法: paste <comp> <dx> <dy>"
      let name = parts[1]
      let idx = state.find_comp(name)
      if idx < 0: return "组件不存在: " & name
      let dx = parse_int(parts[2])
      let dy = parse_int(parts[3])
      let n = state.canvas.add(state.comp_store[idx].canvas, dx, dy)
      return "加法安装 " & $n & " 单元"

    of "erase":
      if parts.len < 5: return "用法: erase <dx> <dy> <w> <h>"
      let dx = parse_int(parts[1])
      let dy = parse_int(parts[2])
      let w = parse_int(parts[3])
      let h = parse_int(parts[4])
      let n = state.canvas.subtract(dx, dy, w, h)
      return "减法拆解 " & $n & " 单元"

    of "level":
      if parts.len < 4: return "用法: level <x> <y> <L1|L2|L3>"
      let x = parse_int(parts[1])
      let y = parse_int(parts[2])
      let c = state.canvas.get(x, y)
      if not c.present: return "该位置无像素"
      let lvl = if parts[3].to_lower().startswith("l2"): plL2
                elif parts[3].to_lower().startswith("l3"): plL3
                else: plL1
      var nc = c
      nc.level = lvl
      state.canvas.set(x, y, nc)
      return "层级已设为 " & level_name(lvl)

    of "info":
      if parts.len < 3: return "用法: info <x> <y>"
      let x = parse_int(parts[1])
      let y = parse_int(parts[2])
      let c = state.canvas.get(x, y)
      if not c.present: return "(" & $x & "," & $y & ") 空"
      return "(" & $x & "," & $y & ") 相位=" & $c.phase &
             " 权重=" & formatFloat(c.weight, ffDecimal, 2) &
             " 层级=" & level_name(c.level) &
             " 归属=" & c.owner

    of "export":
      if parts.len < 2: return "用法: export <path.ppm>"
      let n = export_ppm(state.canvas, parts[1])
      return "已导出 " & $n & " 像素到 " & parts[1]

    of "report":
      return export_report(state.canvas)

    of "log":
      if state.canvas.op_log.len == 0: return "(无操作)"
      var s = ""
      for i, op in state.canvas.op_log:
        s &= $i & ": " & op & "\n"
      return s

    of "help":
      return repl_help()

    of "quit", "exit":
      state.running = false
      return "再见。"

    else:
      return "未知命令: " & cmd & "  (输入 help 查看帮助)"

  except:
    return "错误: " & getCurrentExceptionMsg()

# ------------------------------------------------------------
# 启动 REPL
# ------------------------------------------------------------
proc repl_run*() =
  var state = ReplState(
    canvas: new_phase_canvas(1, 1),
    running: true,
    comp_store: @[]
  )
  echo "═══ 颜色进制 REPL ═══"
  echo "输入 help 查看命令"
  echo ""
  while state.running:
    stdout.write("cb> ")
    stdout.flush_file()
    let line = read_line(stdin)
    let output = repl_exec(state, line)
    if output.len > 0:
      echo output
