# ============================================================
# YIN-YANG-BINARY 三维编码引擎
#
# 核心概念：
#   1. YinYang 二进制 — 阴阳极性映射为二进制
#   2. mlil（毛利兰）立体进制框架 — 表面属性（模/面/名）
#   3. kndl（柯南道尔斯）外壳 — 结构属性（骨/头/体）
#   4. 三角色模块映射 — 毛利兰·工藤新一·灰原哀
#
# 三维坐标系统：
#   X轴 = mlil 表面属性（模/面/名）
#   Y轴 = kndl 结构属性（骨/头/体）
#   Z轴 = 灰原哀 横向属性（行页/行业）
#
# 极性编码：
#   阳 = 1 = 显现
#   阴 = 0 = 隐藏
#   平衡 = 0.5 = 叠加态
# ============================================================

import strutils
import os
import json
import math
import tables

# ── 三角色枚举 ──
type Role* = enum
  roleMaolilan   # 毛利兰·表面属性·模/面/名
  roleKudoshinichi # 工藤新一·结构属性·骨/头/体
  roleHuiyuanai   # 灰原哀·横向属性·行页/行业

# ── YIN-YANG 极性 ──
type YinYang* = enum
  yyYin      # 阴·0·隐藏·容器
  yyYang     # 阳·1·显现·中心
  yyBalanced # 平衡·0.5·叠加态·纠缠

# ── 三维坐标（kndl 静态绝对位置寻址）──
type Coord3D* = object
  x*: float  # mlil 表面
  y*: float  # kndl 结构
  z*: float  # 灰原哀 横向

# ── 编码单元格 ──
type YinYangCell* = object
  code*: string          # 符号编码
  polarity*: YinYang     # 极性
  position*: Coord3D    # 三维绝对位置
  binary*: string        # YinYang 二进制表示
  role*: Role            # 主要映射角色
  surface*: string       # mlil 表面属性值
  structure*: string     # kndl 结构属性值
  horizontal*: string    # 灰原哀 横向属性值

# ── 编码空间 ──
type YinYangSpace* = object
  cells*: seq[YinYangCell]
  cell_count*: int
  spec_path*: string

# ═══════════════════════════════════════════
# 极性 → 二进制映射
# ============================================================
proc polarity_to_binary*(p: YinYang): string =
  case p
  of yyYin: "0"
  of yyYang: "1"
  of yyBalanced: "0.5"

proc polarity_to_float*(p: YinYang): float =
  case p
  of yyYin: 0.0
  of yyYang: 1.0
  of yyBalanced: 0.5

# ═══════════════════════════════════════════
# 编码 → 三维坐标（mlil × kndl × 灰原哀）
# ============================================================
proc code_to_3d*(code: string; polarity: YinYang; category: string): Coord3D =
  # X轴（mlil 表面属性）：基于编码字符在字母表中的位置
  var x_sum = 0.0
  for i, c in code:
    let v = float(int(c) mod 26)
    x_sum += v * sin(float(i) * 0.5)
  
  # Y轴（kndl 结构属性）：基于编码长度和极性
  let y_base = float(len(code)) * 1.2
  let y_polarity = polarity_to_float(polarity)
  let y = y_base + y_polarity * 3.0
  
  # Z轴（灰原哀 横向属性）：基于类别
  var z: float
  case category
  of "root": z = 0.0
  of "naming": z = 2.0
  of "space": z = 4.0
  of "flow": z = 6.0
  of "engine": z = 8.0
  of "protocol": z = 10.0
  of "core": z = 12.0
  of "structure": z = 14.0
  of "horizon": z = 16.0
  of "binary": z = 18.0
  else: z = 1.0
  
  return Coord3D(x: x_sum, y: y, z: z)

# ═══════════════════════════════════════════
# 编码 → 主要角色映射
# ============================================================
proc code_to_role*(code: string; category: string): Role =
  case category
  of "naming", "space", "protocol":
    roleMaolilan   # 表面属性
  of "engine", "structure", "root":
    roleKudoshinichi # 结构属性
  of "flow", "horizon", "binary", "core":
    roleHuiyuanai   # 横向属性
  else:
    # 基于编码长度判断
    if len(code) <= 4: roleMaolilan
    elif len(code) <= 6: roleKudoshinichi
    else: roleHuiyuanai

# ═══════════════════════════════════════════
# 编码 → 表面属性值（mlil）
# ============================================================
proc code_to_surface*(code: string): string =
  # 模/面/名 表面属性
  let idx = int(code[0]) mod 3
  case idx
  of 0: "模（模板·形态）"
  of 1: "面（表面·外观）"
  else: "名（名称·标识）"

# ═══════════════════════════════════════════
# 编码 → 结构属性值（kndl）
# ============================================================
proc code_to_structure*(code: string): string =
  # 骨/头/体 结构属性
  let idx = int(code[0]) mod 3
  case idx
  of 0: "骨（骨架·支撑）"
  of 1: "头（首部·起始）"
  else: "体（主体·内容）"

# ═══════════════════════════════════════════
# 编码 → 横向属性值（灰原哀）
# ============================================================
proc code_to_horizontal*(code: string; category: string): string =
  # 行页/行业 横向属性
  case category
  of "flow": "行（流程·步骤）"
  of "horizon": "页（页面·层次）"
  of "core": "业（事业·领域）"
  else:
    let idx = len(code) mod 3
    case idx
    of 0: "行（流程·步骤）"
    of 1: "页（页面·层次）"
    else: "业（事业·领域）"

# ═══════════════════════════════════════════
# 构建编码空间
# ============================================================
proc build_space*(spec_path: string): YinYangSpace =
  var space = YinYangSpace(
    cells: @[],
    cell_count: 0,
    spec_path: spec_path
  )
  
  let json_content = readFile(spec_path)
  let node = parseJson(json_content)
  
  if not node.hasKey("symbols"):
    return space
  
  let sym_node = node["symbols"]
  
  for code, val in sym_node:
    let polarity_str = val["polarity"].getStr()
    let category = val["category"].getStr()
    
    var pol: YinYang
    case polarity_str
    of "yang": pol = yyYang
    of "yin": pol = yyYin
    else: pol = yyBalanced
    
    let role = code_to_role(code, category)
    let pos = code_to_3d(code, pol, category)
    
    let cell = YinYangCell(
      code: code,
      polarity: pol,
      position: pos,
      binary: polarity_to_binary(pol),
      role: role,
      surface: code_to_surface(code),
      structure: code_to_structure(code),
      horizontal: code_to_horizontal(code, category)
    )
    
    space.cells.add(cell)
    space.cell_count += 1
  
  return space

# ═══════════════════════════════════════════
# YinYang 二进制 → 十进制转换
# ============================================================
proc yinyang_to_decimal*(binary_str: string): int =
  # YinYang 二进制：阳=1，阴=0
  result = 0
  for c in binary_str:
    result = result * 2
    if c == '1':
      result += 1

# ═══════════════════════════════════════════
# 生成 HTML 可视化
# ============================================================
proc generate_html*(space: YinYangSpace; output_path: string) =
  var html = """<!DOCTYPE html>
<html lang="zh">
<head>
<meta charset="UTF-8">
<title>YIN-YANG-BINARY 三维编码空间</title>
<style>
* { margin: 0; padding: 0; box-sizing: border-box; }
body { background: #0a0a1a; color: #e0e0e0; font-family: monospace; }
#header { padding: 20px; text-align: center; border-bottom: 1px solid #333; }
#header h1 { color: #B8FF35; font-size: 20px; margin-bottom: 8px; }
#header p { color: #888; font-size: 12px; }
#axes { display: flex; justify-content: space-around; padding: 15px; background: rgba(0,0,0,0.5); border-bottom: 1px solid #333; }
.axis { text-align: center; }
.axis-name { font-size: 14px; margin-bottom: 4px; }
.axis-desc { font-size: 10px; color: #888; }
#x-axis .axis-name { color: #FF6B35; }
#y-axis .axis-name { color: #35B8FF; }
#z-axis .axis-name { color: #B8FF35; }
#grid { padding: 20px; display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 12px; }
.cell { background: rgba(255,255,255,0.05); border: 1px solid #333; border-radius: 8px; padding: 12px; transition: all 0.3s; }
.cell:hover { background: rgba(255,255,255,0.1); border-color: #B8FF35; transform: translateY(-2px); }
.cell-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px; }
.cell-code { font-size: 14px; font-weight: bold; color: #B8FF35; }
.cell-polarity { font-size: 11px; padding: 2px 6px; border-radius: 4px; }
.polarity-yang { background: #FF6B35; color: #000; }
.polarity-yin { background: #35B8FF; color: #000; }
.polarity-balanced { background: #B8FF35; color: #000; }
.cell-coords { font-size: 10px; color: #888; margin-bottom: 4px; }
.cell-attrs { font-size: 10px; margin-top: 6px; }
.attr { margin: 2px 0; }
.attr-label { color: #666; }
.attr-value { color: #ccc; }
.role-tag { font-size: 9px; padding: 1px 4px; border-radius: 3px; display: inline-block; margin-top: 4px; }
.role-maolilan { background: #FF6B3533; color: #FF6B35; border: 1px solid #FF6B35; }
.role-kudoshinichi { background: #35B8FF33; color: #35B8FF; border: 1px solid #35B8FF; }
.role-huiyuanai { background: #B8FF3533; color: #B8FF35; border: 1px solid #B8FF35; }
</style>
</head>
<body>
<div id="header">
<h1>YIN-YANG-BINARY 三维编码空间</h1>
<p>mlil 毛利兰·表面 | kndl 工藤新一·结构 | 灰原哀·横向</p>
</div>
<div id="axes">
<div id="x-axis" class="axis"><div class="axis-name">X轴·mlil·毛利兰</div><div class="axis-desc">表面属性（模/面/名）</div></div>
<div id="y-axis" class="axis"><div class="axis-name">Y轴·kndl·工藤新一</div><div class="axis-desc">结构属性（骨/头/体）</div></div>
<div id="z-axis" class="axis"><div class="axis-name">Z轴·灰原哀</div><div class="axis-desc">横向属性（行页/行业）</div></div>
</div>
<div id="grid">
"""
  
  for cell in space.cells:
    let pol_class = case cell.polarity
      of yyYang: "polarity-yang"
      of yyYin: "polarity-yin"
      of yyBalanced: "polarity-balanced"
    
    let pol_text = case cell.polarity
      of yyYang: "阳·1"
      of yyYin: "阴·0"
      of yyBalanced: "平衡·0.5"
    
    let role_class = case cell.role
      of roleMaolilan: "role-maolilan"
      of roleKudoshinichi: "role-kudoshinichi"
      of roleHuiyuanai: "role-huiyuanai"
    
    let role_text = case cell.role
      of roleMaolilan: "毛利兰·表面"
      of roleKudoshinichi: "工藤新一·结构"
      of roleHuiyuanai: "灰原哀·横向"
    
    html &= "<div class=\"cell\">\n"
    html &= "<div class=\"cell-header\"><span class=\"cell-code\">" & cell.code & "</span><span class=\"cell-polarity " & pol_class & "\">" & pol_text & "</span></div>\n"
    html &= "<div class=\"cell-coords\">" & cell.surface & " | " & cell.structure & " | " & cell.horizontal & "</div>\n"
    html &= "<div class=\"cell-attrs\">\n"
    html &= "<div class=\"attr\"><span class=\"attr-label\">表面：</span><span class=\"attr-value\">" & cell.surface & "</span></div>\n"
    html &= "<div class=\"attr\"><span class=\"attr-label\">结构：</span><span class=\"attr-value\">" & cell.structure & "</span></div>\n"
    html &= "<div class=\"attr\"><span class=\"attr-label\">横向：</span><span class=\"attr-value\">" & cell.horizontal & "</span></div>\n"
    html &= "</div>\n"
    html &= "<span class=\"role-tag " & role_class & "\">" & role_text & "</span>\n"
    html &= "</div>\n"
  
  html &= """
</div>
</body>
</html>"""
  
  writeFile(output_path, html)

# ═══════════════════════════════════════════
# 辅助函数
# ═══════════════════════════════════════════
proc role_name*(r: Role): string =
  case r
  of roleMaolilan: "毛利兰·表面"
  of roleKudoshinichi: "工藤新一·结构"
  of roleHuiyuanai: "灰原哀·横向"

proc stats*(space: YinYangSpace): string =
  var s = "═══ YIN-YANG-BINARY 三维编码空间统计 ═══\n"
  s &= "单元格数: " & $space.cell_count & "\n"
  
  # 极性分布
  var yang_count = 0
  var yin_count = 0
  var balanced_count = 0
  for c in space.cells:
    case c.polarity
    of yyYang: yang_count += 1
    of yyYin: yin_count += 1
    of yyBalanced: balanced_count += 1
  
  s &= "极性分布: 阳=" & $yang_count & " 阴=" & $yin_count & " 平衡=" & $balanced_count & "\n"
  
  # 角色分布
  var maolilan_count = 0
  var kudoshinichi_count = 0
  var huiyuanai_count = 0
  for c in space.cells:
    case c.role
    of roleMaolilan: maolilan_count += 1
    of roleKudoshinichi: kudoshinichi_count += 1
    of roleHuiyuanai: huiyuanai_count += 1
  
  s &= "角色映射: 毛利兰=" & $maolilan_count & " 工藤新一=" & $kudoshinichi_count & " 灰原哀=" & $huiyuanai_count & "\n"
  
  return s
