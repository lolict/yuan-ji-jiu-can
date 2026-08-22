# ============================================================
# 颜色进制 · CLI 编译入口
# 编译: nim c cb_cli.nim
# 运行: ./cb_cli
# ============================================================

import colorbase/phase
import colorbase/canvas
import colorbase/render
import os

# ------------------------------------------------------------
# 演示: 用加减运算拼装一个成品
# ------------------------------------------------------------
proc demo() =
  echo "═══ 颜色进制 · 演示 ═══\n"

  # 32×16 画布
  var c = new_phase_canvas(32, 16)
  echo "画布: ", canvas_summary(c)

  # 填充基底 (L1 基元)
  for y in 0..<16:
    for x in 0..<32:
      c.set(x, y, make_cell(x + y * 2, 0.6, plL1))
  echo "填充后: ", canvas_summary(c)

  # 创建 L2 组件 "tone"
  var tone = new_phase_canvas(5, 5)
  for y in 0..<5:
    for x in 0..<5:
      tone.set(x, y, make_cell(50, 1.0, plL2))

  # 加法: 安装组件
  let n1 = c.add(tone, 3, 2)
  echo "加法安装 tone: ", n1, " 单元"

  # 创建另一个组件 "mark"
  var mark = new_phase_canvas(3, 3)
  mark.set(1, 1, make_cell(60, 0.9, plL2))
  let n2 = c.add(mark, 12, 6)
  echo "加法安装 mark: ", n2, " 单元"

  # 减法: 挖除中心区域
  let n3 = c.subtract(8, 4, 4, 3)
  echo "减法拆解: ", n3, " 单元"

  # 最终状态
  echo "\n最终: ", canvas_summary(c)

  # ASCII 预览
  echo "\n预览:"
  echo ascii_preview(c, 32, 16)

  # 导出
  let args = command_line_params()
  let out_path = if args.len > 0: args[0] else: "colorbase_out.ppm"
  let pixels = export_ppm(c, out_path)
  echo "导出 ", pixels, " 像素 → ", out_path

  # 元数据报告
  echo "\n", export_report(c)

when is_main_module:
  demo()
