# 满全法·诗歌容器
# 每首诗 = 一个编码序列（MCCP 流）
# 两首诗 = 两个独立容器

import tables

# ============================================================
# 诗容器 A（第一首诗）
# 编码：诗句 → MCCP 序列
# ============================================================
type
  PoemLine* = object
    line_num*: int          # 行号
    text*: string           # 原文
    mccp_seq*: seq[int]     # 对应的 MCCP 编码序
    tone*: string           # 声调模式
    color*: string          # 对应颜色引用
    emotion*: string         # 该行情感

# 请填入第一首诗的具体内容
# 以下为结构模板
const POEM_A*: array[0, PoemLine] = [
  # 示例格式：
  # PoemLine(line_num:1, text:"第一句原文", mccp_seq:@[0,1,2], tone:"阴平", color:"深红", emotion:"开头情感"),
  # PoemLine(line_num:2, text:"第二句原文", mccp_seq:@[3,4,5], tone:"阳平", color:"翠绿", emotion:"承接情感"),
  # ...继续填充
]

func poem_a_title*(): string = "（待填入诗题 A）"
func poem_a_author*(): string = "（待填入作者）"
func poem_a_lines*(): int = POEM_A.len

# ============================================================
# 诗容器 B（第二首诗）
# ============================================================
const POEM_B*: array[0, PoemLine] = [
  # 请填入第二首诗的具体内容
  # 格式同上
]

func poem_b_title*(): string = "（待填入诗题 B）"
func poem_b_author*(): string = "（待填入作者）"
func poem_b_lines*(): int = POEM_B.len

# ============================================================
# 通用诗歌工具
# ============================================================

# 诗行情感统计
func poem_emotion_count*(lines: seq[PoemLine]): Table[string, int] =
  result = initTable[string, int]()
  for line in lines:
    if not result.hasKey(line.emotion):
      result[line.emotion] = 0
    result[line.emotion] += 1

# 诗行颜色序列
func poem_color_seq*(lines: seq[PoemLine]): seq[string] =
  result = @[]
  for line in lines:
    result.add(line.color)

# 查找行
func find_poem_line*(lines: seq[PoemLine], n: int): int =
  for i, line in lines:
    if line.line_num == n: return i
  return -1

when is_main_module:
  echo "=== 诗歌容器 ==="
  echo "诗 A: ", poem_a_title(), " (", poem_a_lines(), " 行)"
  echo "诗 B: ", poem_b_title(), " (", poem_b_lines(), " 行)"
  echo ""
  echo "请在 POEM_A 和 POEM_B 中填入诗句的具体内容和 MCCP 编码序列"
