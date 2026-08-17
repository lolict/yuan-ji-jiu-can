# 满全法 42进制颜色容器
# 每颜色 = 一个编码位置 + 符号 + 情感映射

import tables

type
  ColorNode* = object
    idx*: int           # 编码位置 #0-#41
    symbol*: string     # 对应符号
    name*: string       # 颜色名
    emotion*: string     # 情感映射
    pitch*: string       # 对应音高（颜色→声音）

# 42 颜色编码表
# 分组：7 主色 × 6 色阶 = 42
const COLORS42*: array[42, ColorNode] = [
  # === 红色系 (#0-#5) ===
  ColorNode(idx:0,  symbol:"ạ",  name:"深红",     emotion:"激情",    pitch:"低音"),
  ColorNode(idx:1,  symbol:"a̋",  name:"大红",     emotion:"兴奋",    pitch:"中高音"),
  ColorNode(idx:2,  symbol:"å",  name:"浅红",     emotion:"温柔",    pitch:"高音"),
  ColorNode(idx:3,  symbol:"ã",  name:"粉红",     emotion:"羞怯",    pitch:"颤音高音"),
  ColorNode(idx:4,  symbol:"â",  name:"玫红",     emotion:"诱惑",    pitch:"升降中音"),
  ColorNode(idx:5,  symbol:"ă",  name:"殷红",     emotion:"危险",    pitch:"短促低音"),

  # === 橙色系 (#6-#11) ===
  ColorNode(idx:6,  symbol:"ẹ",  name:"深橙",     emotion:"温暖",    pitch:"低音"),
  ColorNode(idx:7,  symbol:"e̋",  name:"橙色",     emotion:"活力",    pitch:"中高音"),
  ColorNode(idx:8,  symbol:"e̊",  name:"浅橙",     emotion:"明亮",    pitch:"高音"),
  ColorNode(idx:9,  symbol:"ẽ",  name:"杏色",     emotion:"柔和",    pitch:"颤音高音"),
  ColorNode(idx:10, symbol:"ê",  name:"琥珀",     emotion:"神秘",    pitch:"升降中音"),
  ColorNode(idx:11, symbol:"ĕ",  name:"橘红",     emotion:"急切",    pitch:"短促低音"),

  # === 黄色系 (#12-#17) ===
  ColorNode(idx:12, symbol:"ị",  name:"深黄",     emotion:"明朗",    pitch:"低音"),
  ColorNode(idx:13, symbol:"i̋",  name:"明黄",     emotion:"快乐",    pitch:"中高音"),
  ColorNode(idx:14, symbol:"i̊",  name:"浅黄",     emotion:"纯净",    pitch:"高音"),
  ColorNode(idx:15, symbol:"ĩ",  name:"鹅黄",     emotion:"天真",    pitch:"颤音高音"),
  ColorNode(idx:16, symbol:"î",  name:"金黄",     emotion:"辉煌",    pitch:"升降中音"),
  ColorNode(idx:17, symbol:"ĭ",  name:"枯黄",     emotion:"衰败",    pitch:"短促低音"),

  # === 绿色系 (#18-#23) ===
  ColorNode(idx:18, symbol:"ọ",  name:"深绿",     emotion:"宁静",    pitch:"低音"),
  ColorNode(idx:19, symbol:"ő",  name:"翠绿",     emotion:"生机",    pitch:"中高音"),
  ColorNode(idx:20, symbol:"o̊",  name:"浅绿",     emotion:"清新",    pitch:"高音"),
  ColorNode(idx:21, symbol:"õ",  name:"草绿",     emotion:"舒适",    pitch:"颤音高音"),
  ColorNode(idx:22, symbol:"ô",  name:"墨绿",     emotion:"深沉",    pitch:"升降中音"),
  ColorNode(idx:23, symbol:"ŏ",  name:"黄绿",     emotion:"不安",    pitch:"短促低音"),

  # === 青色系 (#24-#29) ===
  ColorNode(idx:24, symbol:"ụ",  name:"深青",     emotion:"冷静",    pitch:"低音"),
  ColorNode(idx:25, symbol:"ű",  name:"青色",     emotion:"理性",    pitch:"中高音"),
  ColorNode(idx:26, symbol:"ů",  name:"浅青",     emotion:"透明",    pitch:"高音"),
  ColorNode(idx:27, symbol:"ũ",  name:"天青",     emotion:"忧郁",    pitch:"颤音高音"),
  ColorNode(idx:28, symbol:"û",  name:"藏青",     emotion:"严肃",    pitch:"升降中音"),
  ColorNode(idx:29, symbol:"ŭ",  name:"青绿",     emotion:"矛盾",    pitch:"短促低音"),

  # === 蓝色系 (#30-#35) ===
  ColorNode(idx:30, symbol:"ȃ",  name:"深蓝",     emotion:"深邃",    pitch:"低音"),
  ColorNode(idx:31, symbol:"ȇ",  name:"蓝色",     emotion:"平和",    pitch:"中高音"),
  ColorNode(idx:32, symbol:"ȋ",  name:"浅蓝",     emotion:"自由",    pitch:"高音"),
  ColorNode(idx:33, symbol:"ȏ",  name:"蔚蓝",     emotion:"梦想",    pitch:"颤音高音"),
  ColorNode(idx:34, symbol:"ȗ",  name:"宝蓝",     emotion:"高贵",    pitch:"升降中音"),
  ColorNode(idx:35, symbol:"ȁ",  name:"灰蓝",     emotion:"忧郁",    pitch:"短促低音"),

  # === 紫色系 (#36-#41) ===
  ColorNode(idx:36, symbol:"ȅ",  name:"深紫",     emotion:"神秘",    pitch:"低音"),
  ColorNode(idx:37, symbol:"ȉ",  name:"紫色",     emotion:"优雅",    pitch:"中高音"),
  ColorNode(idx:38, symbol:"ȍ",  name:"浅紫",     emotion:"浪漫",    pitch:"高音"),
  ColorNode(idx:39, symbol:"ȕ",  name:"薰衣草",   emotion:"治愈",    pitch:"颤音高音"),
  ColorNode(idx:40, symbol:"ä",  name:"紫罗兰",     emotion:"灵性",    pitch:"升降中音"),
  ColorNode(idx:41, symbol:"ö",  name:"暗紫",     emotion:"压抑",    pitch:"短促低音"),
]

# 查找
func find_color*(sym: string): int =
  for i, c in COLORS42:
    if c.symbol == sym: return i
  return -1

func color_name*(idx: int): string =
  if idx >= 0 and idx < 42: return COLORS42[idx].name
  return "未知"

when is_main_module:
  echo "=== 42进制颜色容器 ==="
  for i, c in COLORS42:
    echo "  #", i, " ", c.symbol, " ", c.name, " [", c.emotion, "/", c.pitch, "]"
  echo "总数: ", COLORS42.len
