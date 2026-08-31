# ============================================================
# 三维几何可视化引擎
#
# 核心愿景：
#   将进制、相位、曲线等编码做成可视化三维几何实体
#   追求感知与关系的物理体验而非无法身体感知的抽象代码
#
# 映射规则：
#   - 每个符号 → 一个三维几何实体（形状/位置/颜色）
#   - 编码（code）→ 空间坐标（x,y,z）
#   - 极性（polarity）→ 颜色（阳=暖·阴=冷·平衡=中性）
#   - 类别（category）→ 几何形状
#   - 关系 → 连接线（容器-中心-纠缠态）
# ============================================================

import strutils
import os
import json
import math
import tables

# ── 三维坐标 ──
type Vec3* = object
  x*: float
  y*: float
  z*: float

# ── 几何实体 ──
type GeoEntity* = object
  code*: string           # 符号编码
  position*: Vec3         # 空间位置
  color*: string          # 颜色（十六进制）
  shape*: string          # 形状（cube/sphere/tetra/octa/line）
  size*: float            # 大小
  polarity*: string       # 极性
  category*: string       # 类别
  meaning*: string        # 含义

# ── 关系线 ──
type GeoRelation* = object
  from_code*: string
  to_code*: string
  relation_type*: string  # container/center/entanglement/witness
  color*: string
  strength*: float        # 0..1

# ── 三维场景 ──
type GeoScene* = object
  entities*: seq[GeoEntity]
  relations*: seq[GeoRelation]
  entity_count*: int
  relation_count*: int

# ═══════════════════════════════════════════
# 编码 → 空间坐标映射
# ============================================================
proc code_to_position*(code: string; index: int; total: int): Vec3 =
  # 使用编码的字符值决定基础位置
  var char_sum = 0
  for c in code:
    char_sum += int(c)
  
  # 角度：基于编码在符号表中的位置
  let angle = float(index) * 2.0 * PI / float(total)
  
  # 半径：基于编码长度（越长的编码越远）
  let radius = float(len(code)) * 1.5
  
  # 高度：基于字符值之和
  let height = float(char_sum mod 10) * 0.8 - 4.0
  
  return Vec3(
    x: radius * cos(angle),
    y: height,
    z: radius * sin(angle)
  )

# ═══════════════════════════════════════════
# 极性 → 颜色映射
# ============================================================
proc polarity_to_color*(polarity: string): string =
  case polarity
  of "yang": "#FF6B35"      # 阳·暖橙色·主动
  of "yin": "#35B8FF"       # 阴·冷蓝色·被动
  of "balanced": "#B8FF35"  # 平衡·中性绿色
  else: "#CCCCCC"

# ═══════════════════════════════════════════
# 类别 → 形状映射
# ============================================================
proc category_to_shape*(category: string): string =
  case category
  of "root": "tetrahedron"      # 根基·四面体（稳定）
  of "naming": "sphere"         # 命名·球体（圆满）
  of "space": "octahedron"      # 空间·八面体（多维）
  of "flow": "torus"            # 流·环面（循环）
  of "engine": "cube"           # 引擎·立方体（结构）
  of "protocol": "icosahedron"  # 协议·二十面体（复杂）
  of "core": "dodecahedron"     # 核心·十二面体（包容）
  of "structure": "cone"        # 结构·锥体（方向）
  of "horizon": "cylinder"      # 横向·柱体（延伸）
  of "binary": "tetrahedron"    # 二进制·四面体（二元）
  else: "sphere"

# ═══════════════════════════════════════════
# 创建三维场景
# ============================================================
proc build_scene*(spec_path: string): GeoScene =
  var scene = GeoScene(
    entities: @[],
    relations: @[],
    entity_count: 0,
    relation_count: 0
  )
  
  # 加载共享域规范
  let json_content = readFile(spec_path)
  let node = parseJson(json_content)
  
  if not node.hasKey("symbols"):
    return scene
  
  let sym_node = node["symbols"]
  var codes: seq[string] = @[]
  for code, _ in sym_node:
    codes.add(code)
  
  let total = codes.len
  
  # 创建几何实体
  for i, code in codes:
    let val = sym_node[code]
    let polarity = val["polarity"].getStr()
    let category = val["category"].getStr()
    let meaning = val["meaning"].getStr()
    
    let entity = GeoEntity(
      code: code,
      position: code_to_position(code, i, total),
      color: polarity_to_color(polarity),
      shape: category_to_shape(category),
      size: 0.5 + float(len(code)) * 0.15,
      polarity: polarity,
      category: category,
      meaning: meaning
    )
    
    scene.entities.add(entity)
    scene.entity_count += 1
  
  # 创建关系线
  # 1. 根基连接（mlll → 所有根基符号）
  for entity in scene.entities:
    if entity.category == "root" and entity.code != "mlll":
      scene.relations.add(GeoRelation(
        from_code: "mlll",
        to_code: entity.code,
        relation_type: "root_foundation",
        color: "#FF6B35",
        strength: 0.8
      ))
      scene.relation_count += 1
  
  # 2. 命名连接（mltt → 所有命名符号）
  for entity in scene.entities:
    if entity.category == "naming" and entity.code != "mltt":
      scene.relations.add(GeoRelation(
        from_code: "mltt",
        to_code: entity.code,
        relation_type: "naming",
        color: "#B8FF35",
        strength: 0.6
      ))
      scene.relation_count += 1
  
  # 3. 纠缠态连接（njsgr ↔ smrl 双向）
  scene.relations.add(GeoRelation(
    from_code: "njsgr",
    to_code: "smrl",
    relation_type: "entanglement",
    color: "#FF35B8",
    strength: 1.0
  ))
  scene.relation_count += 1
  
  # 4. YinYang 连接
  scene.relations.add(GeoRelation(
    from_code: "yinyang",
    to_code: "njsgr",
    relation_type: "polarity_balance",
    color: "#B8FF35",
    strength: 0.9
  ))
  scene.relation_count += 1
  
  # 5. 三元见证（gtt → 核心符号）
  for entity in scene.entities:
    if entity.category == "core":
      scene.relations.add(GeoRelation(
        from_code: "gtt",
        to_code: entity.code,
        relation_type: "witness",
        color: "#35B8FF",
        strength: 0.7
      ))
      scene.relation_count += 1
  
  return scene

# ═══════════════════════════════════════════
# 生成 HTML 可视化（CSS 3D + 交互）
# ============================================================
proc generate_html*(scene: GeoScene; output_path: string) =
  var html = """<!DOCTYPE html>
<html lang="zh">
<head>
<meta charset="UTF-8">
<title>满全法·三维几何符号空间</title>
<style>
* { margin: 0; padding: 0; box-sizing: border-box; }
body { background: #0a0a1a; color: #e0e0e0; font-family: monospace; overflow: hidden; }
#scene { width: 100vw; height: 100vh; perspective: 1200px; position: relative; }
#space { width: 100%; height: 100%; transform-style: preserve-3d; animation: rotate 60s linear infinite; }
@keyframes rotate { from { transform: rotateY(0deg) rotateX(20deg); } to { transform: rotateY(360deg) rotateX(20deg); } }
.entity { position: absolute; width: 40px; height: 40px; transform-style: preserve-3d; cursor: pointer; transition: all 0.3s; }
.entity:hover { transform: scale(1.5); z-index: 100; }
.entity .label { position: absolute; top: -25px; left: 50%; transform: translateX(-50%); font-size: 11px; white-space: nowrap; color: #fff; text-shadow: 0 0 5px #000; }
.entity .meaning { position: absolute; top: 45px; left: 50%; transform: translateX(-50%); font-size: 9px; white-space: nowrap; color: #aaa; }
.cube { width: 100%; height: 100%; position: relative; transform-style: preserve-3d; }
.cube .face { position: absolute; width: 40px; height: 40px; border: 1px solid rgba(255,255,255,0.3); opacity: 0.85; }
.sphere { width: 100%; height: 100%; border-radius: 50%; box-shadow: 0 0 15px currentColor; }
.tetra { width: 100%; height: 100%; clip-path: polygon(50% 0%, 0% 100%, 100% 100%); }
.octa { width: 100%; height: 100%; clip-path: polygon(50% 0%, 100% 50%, 50% 100%, 0% 50%); }
.controls { position: fixed; top: 20px; left: 20px; z-index: 1000; background: rgba(0,0,0,0.7); padding: 15px; border-radius: 8px; border: 1px solid #333; }
.controls h3 { color: #B8FF35; margin-bottom: 10px; font-size: 14px; }
.controls p { font-size: 11px; color: #888; margin: 3px 0; }
.legend { position: fixed; bottom: 20px; left: 20px; z-index: 1000; background: rgba(0,0,0,0.7); padding: 15px; border-radius: 8px; border: 1px solid #333; }
.legend h4 { color: #B8FF35; margin-bottom: 8px; font-size: 12px; }
.legend-item { display: flex; align-items: center; margin: 4px 0; font-size: 10px; }
.legend-color { width: 12px; height: 12px; border-radius: 2px; margin-right: 8px; }
</style>
</head>
<body>
<div class="controls">
<h3>满全法·三维几何符号空间</h3>
<p>符号数: COUNT_ENT | 关系数: COUNT_REL</p>
<p>拖动旋转·滚轮缩放·悬停查看</p>
<p style="color:#FF6B35">阳=暖橙 · 阴=冷蓝 · 平衡=中性绿</p>
</div>
<div class="legend">
<h4>极性图例</h4>
<div class="legend-item"><div class="legend-color" style="background:#FF6B35"></div>阳·主动·中心</div>
<div class="legend-item"><div class="legend-color" style="background:#35B8FF"></div>阴·被动·容器</div>
<div class="legend-item"><div class="legend-color" style="background:#B8FF35"></div>平衡·纠缠态</div>
<h4 style="margin-top:10px">关系图例</h4>
<div class="legend-item"><div class="legend-color" style="background:#FF6B35"></div>根基连接</div>
<div class="legend-item"><div class="legend-color" style="background:#FF35B8"></div>纠缠态 ☉§☉</div>
<div class="legend-item"><div class="legend-color" style="background:#35B8FF"></div>三元见证</div>
</div>
<div id="scene">
<div id="space">
"""
  
  # 替换计数
  html = html.replace("COUNT_ENT", $scene.entity_count)
  html = html.replace("COUNT_REL", $scene.relation_count)
  
  # 添加实体
  for entity in scene.entities:
    let px = entity.position.x * 30.0 + 400.0
    let py = entity.position.y * 30.0 + 300.0
    let pz = entity.position.z * 30.0
    
    let shape_html = case entity.shape
      of "sphere": "<div class=\"sphere\" style=\"background:" & entity.color & ";color:" & entity.color & "\"></div>"
      of "tetra": "<div class=\"tetra\" style=\"background:" & entity.color & "\"></div>"
      of "octa": "<div class=\"octa\" style=\"background:" & entity.color & "\"></div>"
      else: "<div class=\"cube\"><div class=\"face\" style=\"background:" & entity.color & ";transform:translateZ(20px)\"></div><div class=\"face\" style=\"background:" & entity.color & ";transform:rotateY(180deg) translateZ(20px)\"></div><div class=\"face\" style=\"background:" & entity.color & ";transform:rotateY(90deg) translateZ(20px)\"></div><div class=\"face\" style=\"background:" & entity.color & ";transform:rotateY(-90deg) translateZ(20px)\"></div><div class=\"face\" style=\"background:" & entity.color & ";transform:rotateX(90deg) translateZ(20px)\"></div><div class=\"face\" style=\"background:" & entity.color & ";transform:rotateX(-90deg) translateZ(20px)\"></div></div>"
    
    html &= "<div class=\"entity\" style=\"left:" & $px & "px;top:" & $py & "px;transform:translateZ(" & $pz & "px)\">\n"
    html &= shape_html & "\n"
    html &= "<div class=\"label\">" & entity.code & "</div>\n"
    html &= "<div class=\"meaning\">" & entity.meaning & "</div>\n"
    html &= "</div>\n"
  
  html &= """
</div>
</div>
<script>
// 交互：鼠标拖动旋转
let isDragging = false;
let startX = 0, startY = 0;
let rotX = 20, rotY = 0;
const space = document.getElementById('space');

document.addEventListener('mousedown', (e) => {
  isDragging = true;
  startX = e.clientX;
  startY = e.clientY;
  space.style.animation = 'none';
});

document.addEventListener('mousemove', (e) => {
  if (!isDragging) return;
  rotY += (e.clientX - startX) * 0.5;
  rotX -= (e.clientY - startY) * 0.5;
  rotX = Math.max(-90, Math.min(90, rotX));
  space.style.transform = `rotateX(${rotX}deg) rotateY(${rotY}deg)`;
  startX = e.clientX;
  startY = e.clientY;
});

document.addEventListener('mouseup', () => { isDragging = false; });
</script>
</body>
</html>"""
  
  writeFile(output_path, html)

# ═══════════════════════════════════════════
# 辅助函数
# ═══════════════════════════════════════════
proc shape_name_cn*(shape: string): string =
  case shape
  of "tetrahedron": "四面体"
  of "sphere": "球体"
  of "octahedron": "八面体"
  of "torus": "环面"
  of "cube": "立方体"
  of "icosahedron": "二十面体"
  of "dodecahedron": "十二面体"
  of "cone": "锥体"
  of "cylinder": "柱体"
  else: shape

proc stats*(scene: GeoScene): string =
  var s = "═══ 三维几何符号空间统计 ═══\n"
  s &= "实体数: " & $scene.entity_count & "\n"
  s &= "关系数: " & $scene.relation_count & "\n"
  
  # 统计极性分布
  var yang_count = 0
  var yin_count = 0
  var balanced_count = 0
  for e in scene.entities:
    case e.polarity
    of "yang": yang_count += 1
    of "yin": yin_count += 1
    else: balanced_count += 1
  
  s &= "极性分布: 阳=" & $yang_count & " 阴=" & $yin_count & " 平衡=" & $balanced_count & "\n"
  
  # 统计形状分布
  var shape_counts: Table[string, int] = initTable[string, int]()
  for e in scene.entities:
    if shape_counts.hasKey(e.shape):
      shape_counts[e.shape] = shape_counts[e.shape] + 1
    else:
      shape_counts[e.shape] = 1
  
  s &= "形状分布: "
  for shape, count in shape_counts:
    s &= shape & "=" & $count & " "
  s &= "\n"
  
  return s
