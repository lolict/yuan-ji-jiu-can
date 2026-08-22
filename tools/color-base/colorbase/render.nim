# ============================================================
# 颜色进制 · 渲染导出
# 相位画布 → PPM 图像文件 (纯文本，零依赖)
# ============================================================

import phase
import canvas
import strutils

# ------------------------------------------------------------
# 导出为 PPM 格式 (P3 纯文本)
# 相位 → RGB 映射后输出，空白区域为黑色
# ------------------------------------------------------------
proc export_ppm*(c: PhaseCanvas, path: string): int {.raises: [IOError].} =
  var s = "P3\n"
  s &= $c.width & " " & $c.height & "\n"
  s &= "255\n"
  
  var pixel_count = 0
  for y in 0..<c.height:
    for x in 0..<c.width:
      let cell = c.cells[y][x]
      if cell.present:
        let rgb = phase_to_rgb(cell.phase, cell.weight)
        s &= $rgb.r & " " & $rgb.g & " " & $rgb.b & " "
        pixel_count += 1
      else:
        s &= "0 0 0 "
    s &= "\n"
  
  let f = open(path, fmWrite)
  f.write(s)
  f.close()
  result = pixel_count

# ------------------------------------------------------------
# 导出元数据报告 (JSON-like)
# ------------------------------------------------------------
func export_report*(c: PhaseCanvas): string =
  var s = "{\n"
  s &= "  \"width\": " & $c.width & ",\n"
  s &= "  \"height\": " & $c.height & ",\n"
  s &= "  \"tick\": " & $c.tick & ",\n"
  s &= "  \"pixel_count\": " & $c.count_present() & ",\n"
  let lc = c.level_count()
  s &= "  \"level_L1\": " & $lc.l1 & ",\n"
  s &= "  \"level_L2\": " & $lc.l2 & ",\n"
  s &= "  \"level_L3\": " & $lc.l3 & ",\n"
  s &= "  \"operations\": [\n"
  for i, op in c.op_log:
    s &= "    \"" & op & "\""
    if i < c.op_log.high: s &= ","
    s &= "\n"
  s &= "  ]\n"
  s &= "}\n"
  result = s

# ------------------------------------------------------------
# ASCII 预览 (终端直接显示)
# ------------------------------------------------------------
func ascii_preview*(c: PhaseCanvas, max_w: int = 64, max_h: int = 32): string =
  let step_x = max(1, c.width div max_w)
  let step_y = max(1, c.height div max_h)
  var s = "+" & repeat("-", min(c.width div step_x, max_w)) & "+\n"
  var y = 0
  while y < c.height:
    s &= "|"
    var x = 0
    while x < c.width:
      let cell = c.cells[y][x]
      if cell.present:
        let brightness = int(cell.weight * 9.0)
        s &= $brightness
      else:
        s &= "."
      x += step_x
    s &= "|\n"
    y += step_y
  s &= "+" & repeat("-", min(c.width div step_x, max_w)) & "+\n"
  result = s
