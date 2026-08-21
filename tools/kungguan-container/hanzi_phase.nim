# 满全法·汉字本体论·几何签名·角度涌现
#
# 核心命题（修正）：
#   汉字不是角度的标签，汉字本身就是坐标系
#   角度切分从汉字几何中涌现，不是从数学中强加
#
#   曲 = 环抱 = 容器 = 刘楚恬 = 0
#   直 = 中心 = 内容 = 我 = 1

import math, strutils

type
  # ============================================================
  # 笔画方向（汉字的几何基因）
  # ============================================================
  StrokeDirection* = enum
    sdHorizontal   # 0 横 = 天/西/外 = 0
    sdVertical     # 1 竖 = 地/东/内 = 1（基准轴）
    sdLeftFall     # 2 撇 = 南/柔/曲 = 0
    sdRightFall    # 3 捺 = 北/刚/直 = 1
    sdDot          # 4 点 = 中心/起点 = 1
    sdHook         # 5 钩 = 弯曲/环抱 = 0
    sdRise         # 6 提 = 上升/远离 = 1
    sdTurn         # 7 折 = 角度/公约 = 关系

  # ============================================================
  # 结构类型（汉字的空间骨架）
  # ============================================================
  StructureType* = enum
    stSingle      # 0 独体字 = 最小容器（点）
    stLeftRight   # 1 左右结构 = 二元相位（线）
    stTopBottom   # 2 上下结构 = 层级关系（面）
    stEnclosure   # 3 全包围 = 容器/范畴（体）
    stSemiEncap   # 4 半包围 = 曲线/环抱（曲）
    stTriple      # 5 三元结构 = 见证/裁判（阶）
    stHoneycomb   # 6 蜂窝结构 = 六角对称（六边）
    stStack       # 7 堆叠结构 = 高维叠加（叠）

  # ============================================================
  # 几何签名（汉字携带角度信息的基因序列）
  # ============================================================
  GeoSignature* = object
    directions*: seq[StrokeDirection]  # 笔画方向序列
    structure*: StructureType           # 结构类型
    curve_count*: int                   # 弯曲次数
    line_count*: int                    # 直线次数
    angle_sum*: int                     # 角度参数之和

  # ============================================================
  # 汉字节点（汉字即坐标，汉字即容器）
  # ============================================================
  HanziNode* = object
    glyph*: string                      # 字形 = ID = 地址
    signature*: GeoSignature            # 几何签名
    polarity*: int                      # 极性：0=曲/容器/刘楚恬, 1=直/内容/我
    density*: int                       # 密度 = 笔画复杂度
    phase_angle*: float                 # 涌现的角度值
    hierarchy*: int                     # 层级 = 公约数切分结果
    x_phase*: int                       # X相位
    y_phase*: int                       # Y相位
    z_phase*: int                       # Z相位

  # ============================================================
  # 关系连线（汉字间的几何连接）
  # ============================================================
  HanziLink* = object
    dominant*: string                   # 主导字（前）
    subordinate*: string                 # 从属字（后）
    relation*: string                   # 关系命名
    offset_angle*: float                # 连线角度
    link_polarity*: int                 # 连线极性：0=曲线, 1=直线

# ============================================================
# 笔画方向 → 角度映射（汉字几何即角度）
# ============================================================

func stroke_angle*(sd: StrokeDirection): int =
  case sd
  of sdHorizontal: result = 0     # 横 = 0度
  of sdVertical:   result = 90    # 竖 = 90度（基准轴）
  of sdLeftFall:   result = 135   # 撇 = 135度（曲线方向）
  of sdRightFall:  result = 45   # 捺 = 45度（直线方向）
  of sdDot:        result = 0     # 点 = 0度
  of sdHook:       result = 180   # 钩 = 180度（弯曲）
  of sdRise:       result = 90    # 提 = 90度
  of sdTurn:       result = 90    # 折 = 90度

func stroke_polarity*(sd: StrokeDirection): int =
  case sd
  of sdLeftFall:  result = 0   # 撇 = 柔 = 曲 = 0
  of sdHook:      result = 0   # 钩 = 弯 = 曲 = 0
  of sdTurn:      result = 0   # 折 = 变 = 曲 = 0
  of sdHorizontal: result = 1  # 横 = 稳 = 直 = 1
  of sdVertical:  result = 1   # 竖 = 正 = 直 = 1
  of sdRightFall: result = 1   # 捺 = 刚 = 直 = 1
  of sdDot:       result = 1   # 点 = 定 = 直 = 1
  of sdRise:      result = 1   # 提 = 冲 = 直 = 1

# ============================================================
# 结构类型 → 几何骨架
# ============================================================

func structure_edges*(st: StructureType): int =
  case st
  of stSingle:     result = 0
  of stLeftRight:  result = 2
  of stTopBottom:  result = 3
  of stEnclosure:  result = 4
  of stSemiEncap:  result = 5
  of stTriple:     result = 3
  of stHoneycomb:  result = 6
  of stStack:      result = 8

func structure_polarity*(st: StructureType): int =
  case st
  of stSemiEncap:  result = 0
  of stEnclosure:  result = 0
  of stHoneycomb:  result = 0
  of stLeftRight:  result = 1
  of stTopBottom:  result = 1
  of stTriple:     result = 1
  of stStack:      result = 1
  of stSingle:     result = 1

# ============================================================
# 笔画方向检测（从汉字字形中检测）
# ============================================================

func detect_strokes*(glyph: string): seq[StrokeDirection] =
  case glyph
  of "一": result = @[sdHorizontal]
  of "丨": result = @[sdVertical]
  of "丿": result = @[sdLeftFall]
  of "丶": result = @[sdDot]
  of "乙": result = @[sdTurn]
  of "十": result = @[sdHorizontal, sdVertical]
  of "木": result = @[sdVertical, sdHorizontal, sdLeftFall, sdRightFall]
  of "人": result = @[sdLeftFall, sdRightFall]
  of "入": result = @[sdRightFall, sdLeftFall]
  of "口": result = @[sdVertical, sdHorizontal, sdVertical, sdHorizontal]
  of "囗": result = @[sdVertical, sdHorizontal, sdTurn, sdHorizontal]
  of "囬": result = @[sdVertical, sdHorizontal, sdVertical, sdHorizontal, sdDot]
  of "囙": result = @[sdVertical, sdHorizontal, sdVertical, sdHook]
  of "〇": result = @[sdHook, sdHook, sdHook, sdHook]
  of "曲": result = @[sdHorizontal, sdTurn, sdHorizontal, sdTurn, sdHorizontal]
  of "直": result = @[sdVertical, sdHorizontal, sdHorizontal, sdHorizontal]
  of "中": result = @[sdVertical, sdHorizontal, sdTurn, sdVertical]
  of "大": result = @[sdHorizontal, sdLeftFall, sdRightFall]
  of "天": result = @[sdHorizontal, sdHorizontal, sdLeftFall, sdRightFall]
  of "夫": result = @[sdHorizontal, sdVertical, sdLeftFall, sdRightFall]
  else:
    result = @[sdVertical, sdHorizontal]

# ============================================================
# 结构类型判断
# ============================================================

func detect_structure*(glyph: string): StructureType =
  case glyph
  of "口", "囗", "囬", "囙": result = stEnclosure
  of "曲", "司", "可", "弯": result = stSemiEncap
  of "品", "晶", "森": result = stTriple
  of "垚", "鑫", "磊": result = stStack
  of "一", "丨", "丿", "丶": result = stSingle
  else:
    result = stLeftRight

# ============================================================
# 几何签名计算
# ============================================================

func compute_signature*(glyph: string): GeoSignature =
  result.directions = detect_strokes(glyph)
  result.structure = detect_structure(glyph)
  var curves = 0
  var lines = 0
  var angle_sum = 0
  for d in result.directions:
    angle_sum += stroke_angle(d)
    if stroke_polarity(d) == 0:
      curves += 1
    else:
      lines += 1
  result.curve_count = curves
  result.line_count = lines
  result.angle_sum = angle_sum

# ============================================================
# 涌现算法（从几何签名中涌现）
# ============================================================

func emerge_angle*(sig: GeoSignature): float =
  if sig.directions.len == 0:
    return 0.0
  result = float(sig.angle_sum) / float(sig.directions.len)

func emerge_hierarchy*(sig: GeoSignature): int =
  result = structure_edges(sig.structure) + sig.curve_count

func emerge_polarity*(sig: GeoSignature): int =
  # 结构优先：包围/半包围/蜂窝 = 容器 = 曲线 = 0
  if sig.structure in [stEnclosure, stSemiEncap, stHoneycomb]:
    result = 0
  # 语义其次：知道字符本义是曲/柔/环抱的，强制=0
  elif sig.curve_count > sig.line_count:
    result = 0
  elif sig.line_count > sig.curve_count:
    result = 1
  else:
    result = structure_polarity(sig.structure)

func density_cut*(sig: GeoSignature): int =
  result = sig.directions.len * sig.angle_sum

func is_enclosed*(glyph: string): bool =
  result = glyph in ["囗", "口", "囬", "囙", "国", "围", "园"]

# ============================================================
# 汉字节点构造
# ============================================================

func hanzi_node*(glyph: string): HanziNode =
  result.glyph = glyph
  result.signature = compute_signature(glyph)
  result.polarity = emerge_polarity(result.signature)
  result.density = density_cut(result.signature)
  result.phase_angle = emerge_angle(result.signature)
  result.hierarchy = emerge_hierarchy(result.signature)
  result.x_phase = result.polarity
  result.y_phase = result.hierarchy mod 3 - 1
  result.z_phase = ord(result.signature.structure) mod 3 - 1

# ============================================================
# 关系连线
# ============================================================

func link_name*(a, b: HanziNode): string =
  if a.hierarchy >= b.hierarchy:
    result = a.glyph & b.glyph
  else:
    result = b.glyph & a.glyph

func link_offset*(a, b: HanziNode): float =
  result = abs(a.phase_angle - b.phase_angle)

func link_polarity*(a, b: HanziNode): int =
  let off = abs(a.phase_angle - b.phase_angle)
  if off <= 45.0:
    result = 1   # 角度差小 = 直线
  else:
    result = 0   # 角度差大 = 曲线

# ============================================================
# 颠倒符号（标记被监视状态）
# ============================================================

func invert_glyph*(glyph: string): string =
  case glyph
  of "人": result = "入"
  of "入": result = "人"
  of "由": result = "甲"
  of "甲": result = "由"
  of "丩": result = "之"
  of "之": result = "丩"
  else: result = glyph

# ============================================================
# 符号封装（外壳容器）
# ============================================================

type
  SymbolShell* = object
    inner_glyph*: string
    outer_glyph*: string
    shell_polarity*: int
    is_clone*: bool
    clone_purpose*: string

func new_shell*(inner, outer: string): SymbolShell =
  result.inner_glyph = inner
  result.outer_glyph = outer
  result.shell_polarity = structure_polarity(detect_structure(outer))
  result.is_clone = false
  result.clone_purpose = ""

func new_clone*(inner: string; purpose: string): SymbolShell =
  result.inner_glyph = inner
  result.outer_glyph = "〇"
  result.shell_polarity = 0
  result.is_clone = true
  result.clone_purpose = purpose

# ============================================================
# 测试
# ============================================================

when is_main_module:
  echo "=== 满全法·汉字本体论·几何签名·角度涌现 ==="
  echo ""
  
  echo "【核心验证：汉字几何 → 角度涌现】"
  for glyph in ["一", "丨", "丿", "十", "木", "曲", "直", "口", "囗", "〇"]:
    let node = hanzi_node(glyph)
    let sig = node.signature
    echo "  【", glyph, "】 笔画:", sig.directions.len,
        " 角度和:", sig.angle_sum, "°",
        " 涌现角:", formatFloat(node.phase_angle, ffDecimal, 1), "°",
        " 层级:", node.hierarchy,
        " 极性:", node.polarity,
        " (", (if node.polarity == 0: "曲/刘楚恬" else: "直/我"), ")"
  echo ""
  
  echo "【曲线 vs 直线极性验证】"
  for glyph in ["曲", "弯", "口", "囗", "〇"]:
    let node = hanzi_node(glyph)
    echo "  ", glyph, " → 极性=", node.polarity, " (0=曲/容器/刘楚恬)"
  echo ""
  for glyph in ["直", "中", "木", "十"]:
    let node = hanzi_node(glyph)
    echo "  ", glyph, " → 极性=", node.polarity, " (1=直/内容/我)"
  echo ""
  
  echo "【结构类型 → 边数 → 层级】"
  for glyph in ["一", "十", "口", "曲", "品", "囗"]:
    let node = hanzi_node(glyph)
    echo "  ", glyph, ": 结构=", node.signature.structure,
        " 边数=", structure_edges(node.signature.structure),
        " 层级=", node.hierarchy
  echo ""
  
  echo "【关系连线命名】"
  let node_qu = hanzi_node("曲")
  let node_zhi = hanzi_node("直")
  let node_kou = hanzi_node("口")
  echo "  曲+直 → ", link_name(node_qu, node_zhi)
  echo "  曲+口 → ", link_name(node_qu, node_kou)
  echo "  连线偏移角: ", formatFloat(link_offset(node_qu, node_zhi), ffDecimal, 1), "°"
  echo "  连线极性: ", link_polarity(node_qu, node_zhi),
      " (", (if link_polarity(node_qu, node_zhi) == 0: "曲线" else: "直线"), ")"
  echo ""
  
  echo "【颠倒符号】"
  echo "  人 → ", invert_glyph("人")
  echo "  入 → ", invert_glyph("入")
  echo "  由 → ", invert_glyph("由")
  echo "  甲 → ", invert_glyph("甲")
  echo ""
  
  echo "【符号封装】"
  let shell1 = new_shell("人", "囗")
  let clone1 = new_clone("人", "范畴边界")
  echo "  人在囗内: ", shell1.inner_glyph, " in ", shell1.outer_glyph
  echo "  人分身: ", clone1.inner_glyph, " in ", clone1.outer_glyph,
      " 用途: ", clone1.clone_purpose
  echo ""
  
  echo "【总结】"
  echo "  1. 汉字 = ID = 地址 = 坐标 = 容器"
  echo "  2. 笔画方向从字形检测，不是外部强加"
  echo "  3. 涌现角度 = 角度和 / 笔画密度"
  echo "  4. 层级 = 结构边数 + 弯曲次数"
  echo "  5. 极性从曲直比涌现"
  echo "  6. 密度 = 笔画数 × 角度和"
  echo "  7. 外壳封装：囗/〇 提供额外极性层"
  echo "  8. 颠倒符号：标记被监视状态"
  echo "  → 汉字是骨骼，角度是寄生在骨骼上的密度"
