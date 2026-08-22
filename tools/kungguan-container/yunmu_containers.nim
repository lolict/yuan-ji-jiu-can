## ═══════════════════════════════════════════════════════════════
## 额外集装箱 0 · 1 · 2 · 5 · 备用编码容器
## ═══════════════════════════════════════════════════════════════
## 用途: 存放"aoeiuv"基础元音及其变音符号
##   集装箱0 = 一声组 (长音 ā ē ī ō ū ǖ)
##   集装箱1 = 二声组 (锐音 á é í ó ú ǘ)
##   集装箱2 = 三声组 (抑扬 ǎ ě ǐ ǒ ǔ ǚ)
##   集装箱5 = 四声组 (钝音 à è ì ò ù ǜ)
##
## 这4个集装箱从"韵母系统"拆出 · 归入60声系统使用
## 每个集装箱9符号: 6小写 + 3大写(节省空间)
## ═══════════════════════════════════════════════════════════════

import tables

# ═══════════════════════════════════════════════════════════════
# 集装箱0: 长音一声组
# ═══════════════════════════════════════════════════════════════
const CONTAINER_0* = [
  # 小写 (6)
  "ā",    # 0  a长音
  "ē",    # 1  e长音
  "ī",    # 2  i长音
  "ō",    # 3  o长音
  "ū",    # 4  u长音
  "ǖ",    # 5  ü长音
]

# ═══════════════════════════════════════════════════════════════
# 集装箱1: 锐音二声组
# ═══════════════════════════════════════════════════════════════
const CONTAINER_1* = [
  # 小写 (6)
  "á",    # 0  a锐音
  "é",    # 1  e锐音
  "í",    # 2  i锐音
  "ó",    # 3  o锐音
  "ú",    # 4  u锐音
  "ǘ",    # 5  ü锐音
]

# ═══════════════════════════════════════════════════════════════
# 集装箱2: 抑扬三声组
# ═══════════════════════════════════════════════════════════════
const CONTAINER_2* = [
  # 小写 (6)
  "ǎ",    # 0  a抑扬
  "ě",    # 1  e抑扬
  "ǐ",    # 2  i抑扬
  "ǒ",    # 3  o抑扬
  "ǔ",    # 4  u抑扬
  "ǚ",    # 5  ü抑扬
]

# ═══════════════════════════════════════════════════════════════
# 集装箱5: 钝音四声组 + 轻声
# ═══════════════════════════════════════════════════════════════
const CONTAINER_5* = [
  # 小写 (6)
  "à",    # 0  a钝音
  "è",    # 1  e钝音
  "ì",    # 2  i钝音
  "ò",    # 3  o钝音
  "ù",    # 4  u钝音
  "ǜ",    # 5  ü钝音
]

# ═══════════════════════════════════════════════════════════════
# 全角大写组(跨集装箱共享)
# ═══════════════════════════════════════════════════════════════
const CONTAINER_UPPER* = [
  "Ā", "Ē", "Ī", "Ō", "Ū", "Ǖ",    # 一声大写
  "Á", "É", "Í", "Ó", "Ú", "Ǘ",    # 二声大写
  "Ǎ", "Ě", "Ǐ", "Ǒ", "Ǔ", "Ǚ",    # 三声大写
  "À", "È", "Ì", "Ò", "Ù", "Ǜ",    # 四声大写
]

# ═══════════════════════════════════════════════════════════════
# 容器定义
# ═══════════════════════════════════════════════════════════════
type
  YinDiaoContainer* = object
    name*: string
    id*: int                  # 集装箱编号 (0/1/2/5)
    symbols*: seq[string]
    description*: string

  ContainerSystem* = object
    containers*: seq[YinDiaoContainer]
    name_to_container*: Table[string, int]
    id_to_container*: Table[int, int]

func new_container_system*(): ContainerSystem =
  result.containers = @[]
  result.name_to_container = initTable[string, int]()
  result.id_to_container = initTable[int, int]()

  # 装载4个集装箱
  var c0: YinDiaoContainer
  c0.name = "一声·长音"
  c0.id = 0
  c0.symbols = @CONTAINER_0
  c0.description = "ā ē ī ō ū ǖ — 高平长音"
  result.containers.add(c0)

  var c1: YinDiaoContainer
  c1.name = "二声·锐音"
  c1.id = 1
  c1.symbols = @CONTAINER_1
  c1.description = "á é í ó ú ǘ — 上扬锐音"
  result.containers.add(c1)

  var c2: YinDiaoContainer
  c2.name = "三声·抑扬"
  c2.id = 2
  c2.symbols = @CONTAINER_2
  c2.description = "ǎ ě ǐ ǒ ǔ ǚ — 曲折抑扬"
  result.containers.add(c2)

  var c5: YinDiaoContainer
  c5.name = "四声·钝音"
  c5.id = 5
  c5.symbols = @CONTAINER_5
  c5.description = "à è ì ò ù ǜ — 下沉钝音"
  result.containers.add(c5)

  for i, c in result.containers:
    for sym in c.symbols:
      result.name_to_container[sym] = c.id
    result.id_to_container[c.id] = i

func get_container*(cs: ContainerSystem, id: int): YinDiaoContainer =
  if cs.id_to_container.hasKey(id):
    let idx = cs.id_to_container[id]
    return cs.containers[idx]
  return YinDiaoContainer()

func lookup*(cs: ContainerSystem, sym: string): int =
  if cs.name_to_container.hasKey(sym): return cs.name_to_container[sym]
  return -1

# ═══════════════════════════════════════════════════════════════
# 变音韵母容器(100进制的"剩余符号"组)
# ═══════════════════════════════════════════════════════════════
const REMAINING_100* = [
  # 这些是 yunmu50_lower 中未入选66进制的符号
  # 可用作"扩展编码"或"双拼第二字母"
  "ạ", "ẹ", "ị", "ọ", "ụ",  # 下点组(不使用)
  "ä", "ë", "ï", "ö",        # 分音组(不使用)
  "å", "e̊", "i̊", "o̊", "ů",  # 上圈组(不使用)
  "ã", "ẽ", "ĩ", "õ", "ũ",  # 波浪组(不使用)
  "â", "ê", "î", "ô", "û",  # 帽子组(不使用)
  "ă", "ĕ", "ĭ", "ŏ", "ŭ",  # 短音组(不使用)
  "ȁ", "ȅ", "ȉ", "ȍ", "ȕ",  # 反钝音组(不使用)
]

when is_main_module:
  echo "═══════════════════════════════════════════════════════════"
  echo "  声调集装箱系统 · 0 · 1 · 2 · 5 · + 剩余符号"
  echo "═══════════════════════════════════════════════════════════"

  let cs = new_container_system()
  echo "集装箱数量: ", cs.containers.len
  echo ""

  for c in cs.containers:
    echo "  集装箱", c.id, " — ", c.name, " — ", c.description
    var line = "    "
    for sym in c.symbols:
      line &= sym & " "
    echo line
    echo ""

  echo "── 大写共享组 ──"
  var uline = "  "
  for sym in CONTAINER_UPPER:
    uline &= sym & " "
  echo uline
  echo ""

  echo "── 剩余符号 (", REMAINING_100.len, "个) ──"
  var rline = "  "
  for sym in REMAINING_100:
    rline &= sym & " "
  echo rline
  echo ""

  echo "── 统计 ──"
  let total = CONTAINER_0.len + CONTAINER_1.len + CONTAINER_2.len + CONTAINER_5.len + CONTAINER_UPPER.len + REMAINING_100.len
  echo "  4集装箱小写: ", CONTAINER_0.len + CONTAINER_1.len + CONTAINER_2.len + CONTAINER_5.len
  echo "  大写共享组: ", CONTAINER_UPPER.len
  echo "  剩余符号: ", REMAINING_100.len
  echo "  总计: ", total
