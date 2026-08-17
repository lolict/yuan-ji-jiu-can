# 满全法 48进制声母编码（独立文件）
# 24声母小写 + 24声母大写 = 48位
# 每对严格大小写配对

import tables

const SHENGMU48* = [
  # 唇音
  "b",    # 0  双唇不送气清塞音
  "p",    # 1  双唇送气清塞音
  "m",    # 2  双唇浊鼻音
  "f",    # 3  唇齿清擦音

  # 舌尖前音（齿音）
  "d",    # 4  舌尖不送气清塞音
  "t",    # 5  舌尖送气清塞音
  "n",    # 6  舌尖浊鼻音
  "l",    # 7  舌尖浊边音

  # 舌根音（软腭音）
  "g",    # 8  舌根不送气清塞音
  "k",    # 9  舌根送气清塞音
  "h",    # 10 舌根清擦音
  "w",    # 11 舌根浊近音

  # 舌面音
  "j",    # 12 舌面不送气清塞擦音
  "q",    # 13 舌面送气清塞擦音
  "x",    # 14 舌面清擦音
  "y",    # 15 舌面浊近音

  # 舌尖前音（平舌/齿龈后）
  "z",    # 16 舌尖前不送气清塞擦音
  "c",    # 17 舌尖前送气清塞擦音
  "s",    # 18 舌尖前清擦音
  "r",    # 19 舌尖后浊擦音

  # 扩展声母（用户自定义音位）
  "ȥ",    # 20 浊平舌擦音（z的浊化变体）
  "ç",    # 21 清硬腭擦音
  "ş",    # 22 清卷舌擦音
  "ŋ",    # 23 舌根浊鼻音
]

const SHENGMU48_UPPER* = [
  # 对应大写
  "B",    # 0
  "P",    # 1
  "M",    # 2
  "F",    # 3
  "D",    # 4
  "T",    # 5
  "N",    # 6
  "L",    # 7
  "G",    # 8
  "K",    # 9
  "H",    # 10
  "W",    # 11
  "J",    # 12
  "Q",    # 13
  "X",    # 14
  "Y",    # 15
  "Z",    # 16
  "C",    # 17
  "S",    # 18
  "R",    # 19
  "Ȥ",    # 20
  "Ç",    # 21
  "Ş",    # 22
  "Ŋ",    # 23
]

# 声母名到索引映射
const SHENGMU_NAMES* = [
  "b", "p", "m", "f",
  "d", "t", "n", "l",
  "g", "k", "h", "w",
  "j", "q", "x", "y",
  "z", "c", "s", "r",
  "zhui", "cilde", "silde", "ng",
]

type
  ShengmuNode* = object
    name*: string        # 声母名
    idx*: int            # 编码索引(0-23小写,24-47大写)
    lower*: string       # 小写符号
    upper*: string       # 大写符号

  ShengmuTable* = object
    name*: string
    nodes*: seq[ShengmuNode]
    lower_table*: Table[string, int]
    upper_table*: Table[string, int]

func new_shengmu_table*(name: string): ShengmuTable =
  result.name = name
  result.nodes = @[]
  result.lower_table = initTable[string, int]()
  result.upper_table = initTable[string, int]()
  for i in 0..23:
    var node: ShengmuNode
    node.name = SHENGMU_NAMES[i]
    node.idx = i
    node.lower = SHENGMU48[i]
    node.upper = SHENGMU48_UPPER[i]
    result.nodes.add(node)
    result.lower_table[node.lower] = i
    result.upper_table[node.upper] = i + 24

# 总基数
const TOTAL_BASE* = 48

# 是否是标准声母（非扩展）
func is_standard*(idx: int): bool =
  return idx < 20

func is_extended*(idx: int): bool =
  return idx >= 20 and idx < 24

when is_main_module:
  echo "=== 满全法 48进制声母 ==="
  var st = new_shengmu_table("48进制声母")
  echo "码元总数: ", SHENGMU48.len + SHENGMU48_UPPER.len, " (", SHENGMU48.low + 24, "位)"
  echo "声母数: ", SHENGMU48.len
  echo "---"

  echo "声母表（小写+大写=配对的6位偏移）"
  echo "标准声母(0-19):"
  for i in 0..19:
    echo "  #", i, " ", st.nodes[i].lower, " / #", i+24, " ", st.nodes[i].upper
  echo "扩展声母(20-23):"
  for i in 20..23:
    echo "  #", i, " ", st.nodes[i].lower, " / #", i+24, " ", st.nodes[i].upper
