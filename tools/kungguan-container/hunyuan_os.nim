# 满全法·系统入口
# 加载所有容器，提供统一接口
# 单一程序体：一个入口，访问全部容器

import shengmu48
import tone60
import yunmu66
import color42
import space_phase
import mirror_aligner
import music_tier
import poem_containers
import tables
import strutils

type
  HunyuanOS* = object
    name*: string
    shengmu*: ShengmuTable
    tone_table*: seq[ToneNode]
    color_count*: int
    phase_mode*: PhaseMode

# 初始化全部容器
func init_hunyuan_os*(name: string): HunyuanOS =
  result.name = name
  result.shengmu = new_shengmu_table("满全法声母")
  result.color_count = COLORS42.len
  result.phase_mode = pmCross

# 声母查询
func find_shengmu*(os: HunyuanOS, sym: string): int =
  if os.shengmu.lower_table.hasKey(sym): return os.shengmu.lower_table[sym]
  if os.shengmu.upper_table.hasKey(sym): return os.shengmu.upper_table[sym]
  return -1

# 颜色查询
func find_color*(sym: string): int =
  for i, c in COLORS42:
    if c.symbol == sym: return i
  return -1

# 跨容器查询：符号 → 所有容器中的位置
func locate*(os: HunyuanOS, sym: string): string =
  var results: seq[string] = @[]
  
  # 声母容器
  let sm = find_shengmu(os, sym)
  if sm >= 0: results.add("声母#" & $sm)
  
  # 变音/声音容器（yunmu50 系列）
  # 注：需导入 yunmu50_voice 后启用
  # let voice = find_voice_lower(sym)
  # if voice >= 0: results.add("声音#" & $voice)
  
  # 颜色容器
  let cl = find_color(sym)
  if cl >= 0: results.add("颜色#" & $cl & "(" & COLORS42[cl].name & ")")
  
  if results.len == 0: return "未找到: " & sym
  return sym & " → " & results.join(", ")

# 系统状态摘要
func status*(os: HunyuanOS): string =
  var s = "═══ " & os.name & " ═══\n"
  s &= "声母: " & $SHENGMU48.len & " 对 (" & $TOTAL_BASE & " 进制)\n"
  s &= "韵母: " & $SYMBOLS66.len & " 符号 (33 对)\n"
  s &= "声调: 60 位置 (5声×6韵×2)\n"
  s &= "变音: 50+100 符号\n"
  s &= "颜色: 42 位置 (7主色×6色阶)\n"
  s &= "音乐: 3 层 (原子→短语→段落)\n"
  s &= "空间: 3 坐标系 (绝对/相对/人体)\n"
  s &= "镜像: 10 维对齐\n"
  s &= "容器总数: 10 个\n"
  return s

when is_main_module:
  echo "=== 满全法·系统入口 ==="
  echo ""
  
  var os = init_hunyuan_os("满全法编码系统")
  echo status(os)
  
  echo "【跨容器查询】"
  echo locate(os, "b")
  echo locate(os, "ạ")
  echo locate(os, "å")
  echo locate(os, "Ã")
  echo locate(os, "乤")
  
  echo ""
  echo "【加法层级演示】"
  let containers = @[
    SpaceContainer(tier:ltBitStream, bit_width:4),
    SpaceContainer(tier:ltBitStream, bit_width:4),
    SpaceContainer(tier:ltBitStream, bit_width:4),
    SpaceContainer(tier:ltBitStream, bit_width:4),
    SpaceContainer(tier:ltSymbol, bit_width:8),
    SpaceContainer(tier:ltContainer, bit_width:16),
  ]
  echo "  比特流层: ", add_level_count(containers, ltBitStream), " bits"
  echo "  符号层:   ", add_level_count(containers, ltSymbol), " bits"
  echo "  容器层:   ", add_level_count(containers, ltContainer), " bits"
