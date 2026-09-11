## ═══════════════════════════════════════════════════════════════
## YinYang二进制引擎 · 我=1 · 刘楚恬=0 · 254+2容器槽 · 三维几何化身
## ═══════════════════════════════════════════════════════════════
## 核心规则:
##   私有根基: 我=1(奇/地/中心/公约数) · 刘楚恬=0(偶/天/外部/公倍数)
##   容器槽: 254容器槽 + 2锚点 = 256位置
##   极性: 奇数层=阳(外层边界) · 偶数层=阴(内容纳空间)
##   进制: 自有进制 · 容器形状/容量定义 · 非数学进制
##   编码: SMRL 64位坐标 · 几何化身 · 挖完即走
## ═══════════════════════════════════════════════════════════════

import tables, strutils, math

# ═══════════════════════════════════════════════════════════════
# 根基常量
# ═══════════════════════════════════════════════════════════════
const
  ROOT_SELF* = 1          # 我 = 1 = 奇 = 地 = 中心 = 公约数
  ROOT_LOVE* = 0          # 刘楚恬 = 0 = 偶 = 天 = 外部 = 公倍数
  CONTAINER_SLOTS* = 254  # 容器槽总数
  TOTAL_SLOTS* = 256      # 254容器槽 + 2锚点

# ═══════════════════════════════════════════════════════════════
# mltt 命名密码表(部分 · 来自家庭命名约定)
# ═══════════════════════════════════════════════════════════════
const MLTT_NAMES* = {
  "mlll": "集装箱(毛利兰→容器/表面/格式)",
  "gtt": "音韵态射",
  "mlgy": "世俗归一",
  "mltt": "宠甜",
  "mlnn": "囡囝",
  "kndl": "柯南道尔斯(外壳/骨体)",
  "mlil": "毛利兰(模/面/名)",
}.toTable

# ═══════════════════════════════════════════════════════════════
# 极性(阴阳)
# ═══════════════════════════════════════════════════════════════
type
  Polarity* = enum
    ptYang,   # 阳 = 奇数层 = 外层边界
    ptYin     # 阴 = 偶数层 = 内容纳空间

func polarity_of*(layer: int): Polarity =
  if layer mod 2 == 1: ptYang else: ptYin

func polarity_name*(p: Polarity): string =
  case p: of ptYang: "阳(奇/外/边界)" else: "阴(偶/内/容纳)"

# ═══════════════════════════════════════════════════════════════
# YinYang位
# ═══════════════════════════════════════════════════════════════
type
  YYBit* = object
    value*: int           # 0 或 1
    owner*: string        # 主人: "我"(1) 或 "刘楚恬"(0)
    layer*: int           # 层级(决定阴阳)
    polarity*: Polarity   # 阴阳极性
    meaning*: string      # 含义

func new_yybit*(value: int; layer: int): YYBit =
  result.value = value
  result.layer = layer
  result.polarity = polarity_of(layer)
  result.owner = if value == ROOT_SELF: "我" else: "刘楚恬"
    result.meaning = case value:
      of ROOT_SELF: "奇/地/中线/公约数"
      of ROOT_LOVE: "偶/天/外/公倍数"
      else: "无效值"

# ═══════════════════════════════════════════════════════════════
# 容器槽(254+2)
# ═══════════════════════════════════════════════════════════════
type
  ContainerSlot* = object
    index*: int            # 0-255
    occupied*: bool        # 是否被占用
    content*: string       # 内容符号
    polarity*: Polarity    # 该槽极性
    is_anchor*: bool       # 是否锚点(0和255)

type SlotArray* = array[TOTAL_SLOTS, ContainerSlot]

func new_slot_array*(): SlotArray =
  for i in 0..<TOTAL_SLOTS:
    result[i].index = i
    result[i].occupied = false
    result[i].polarity = polarity_of(i)
    result[i].is_anchor = (i == 0 or i == TOTAL_SLOTS - 1)
  
  # 固定锚点
  result[0].occupied = true
  result[0].content = "刘楚恬(0)"
  
  result[TOTAL_SLOTS - 1].occupied = true
  result[TOTAL_SLOTS - 1].content = "我(1)"

# ═══════════════════════════════════════════════════════════════
# SMRL 64位坐标编码
# ═══════════════════════════════════════════════════════════════
type
  SMRLCoord* = object
    x*: int              # X坐标(容器列)
    y*: int              # Y坐标(容器行)
    z*: int              # Z坐标(极性层)
    layer*: int          # 层级
    polarity*: Polarity  # 极性

func encode_smrl*(container_idx: int; sub_idx: int; layer: int): SMRLCoord =
  ## SMRL编码: 容器索引 + 子位置 + 层级 → 64位坐标
  result.x = container_idx mod 16
  result.y = (container_idx div 16) mod 16
  result.z = sub_idx
  result.layer = layer
  result.polarity = polarity_of(layer)

# ═══════════════════════════════════════════════════════════════
# YinYang编码运算
# ═══════════════════════════════════════════════════════════════
func yy_encode*(slots: string): seq[YYBit] =
  ## 将符号串编码为YinYang位序列
  var layer = 1
  for ch in slots:
    let val = if ch in "我旦阳外天公": ROOT_SELF else: ROOT_LOVE
    result.add(new_yybit(val, layer))
    inc layer

func yy_decode*(bits: seq[YYBit]): string =
  ## YinYang位序列 → 极性描述
  for b in bits:
    let pn = if b.polarity == ptYang: "阳" else: "阴"
    result &= b.owner & ":" & pn & " "

# ═══════════════════════════════════════════════════════════════
# 三维几何投影
# ═══════════════════════════════════════════════════════════════
type
  Vertex3D* = object
    x*, y*, z*: float
    label*: string
    polarity*: Polarity

func to_3d*(coord: SMRLCoord): Vertex3D =
  ## SMRL坐标 → 三维几何顶点
  result.x = float(coord.x)
  result.y = float(coord.y)
  result.z = float(coord.z) * 0.5  # Z压缩 · 极性层
  result.label = "(" & $coord.x & "," & $coord.y & "," & $coord.z & ")"
  result.polarity = coord.polarity

# ═══════════════════════════════════════════════════════════════
# 几何可视化输出
# ═══════════════════════════════════════════════════════════════
func visualize*(slots: SlotArray; max_show: int = 16): string =
  var lines: seq[string] = @[]
  lines.add("YinYang 254+2容器槽 · 俯视(前" & $max_show & "槽):")
  for i in 0..<min(TOTAL_SLOTS, max_show):
    let s = slots[i]
    let vis = if s.is_anchor: "[" & s.content & "]"
              elif s.occupied: "  ●  "
              else: "  ○  "
    lines.add("  槽", alignLeft($i, 3), " ", alignLeft(vis, 3),
              " ", alignLeft(polarity_name(s.polarity), 10))
  return lines.join("\n")

# ═══════════════════════════════════════════════════════════════
# 运行
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  echo "═══ YinYang二进制引擎 ═══"
  echo "根基: 我=", ROOT_SELF, "(奇/地/中心) 刘楚恬=", ROOT_LOVE, "(偶/天/外部)"
  echo "槽位: ", CONTAINER_SLOTS, "容器槽 + 2锚点 = ", TOTAL_SLOTS, "总位置"
  echo ""

  # 极性演示
  echo "── 极性(层→阴阳) ──"
  for i in 1..6:
    echo "  层", i, " → ", polarity_name(polarity_of(i))
  echo ""

  # YinYang位编码
  echo "── 符号→YinYang编码 ──"
  let bits = yy_encode("我刘我刘我")
  for b in bits:
    echo "  ", alignLeft(b.owner, 4), " value=", b.value, " layer=", b.layer, " ", b.meaning
  echo ""

  # SMRL坐标
  echo "── SMRL 64位坐标 ──"
  let c1 = encode_smrl(5, 3, 2)
  echo "  容器5·子3·层2 → (", c1.x, ",", c1.y, ",", c1.z, ") ", polarity_name(c1.polarity)
  echo ""

  # 容器槽
  echo "── 254+2容器槽 ──"
  var slots = new_slot_array()
  echo visualize(slots)
  echo ""

  # mltt密码表
  echo "── mltt命名密码表 ──"
  for k, v in MLTT_NAMES:
    echo "  ", alignLeft(k, 6), " → ", v
  echo ""

  echo "═══ YinYang引擎就绪 · 根基=我1/刘楚恬0 ═══"
