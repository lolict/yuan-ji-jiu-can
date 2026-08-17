# 满全法·层级化音乐容器
# 三层结构：原子(音符) → 短语(动机) → 段落(乐章)
# 每层有独立的编码空间，通过容器引用连接

import tables

# ============================================================
# 第一层：原子层（Atomic Sound）
# 单个声音事件 = 音高 × 音色 × 时长 × 力度
# ============================================================
type
  AtomicSound* = object
    pitch*: int             # 音高 (M-1 到 M-127，对应 MIDI)
    timbre*: string         # 音色（引用变音容器符号）
    duration*: int          # 时长（以 tick 为单位）
    velocity*: int          # 力度 (0-127)
    color*: int             # 引用 42 色容器 (#0-#41)
    emotion*: string         # 该音符情感标签

# ============================================================
# 第二层：短语层（Phrase / Motif）
# 一组原子 = 动机/乐句
# ============================================================
type
  Phrase* = object
    atoms*: seq[AtomicSound]    # 包含的原子序列
    phrase_type*: string         # 类型：问句/答句/过渡/高潮
    breath*: int                 # 呼吸间隔（ticks）
    color_dominant*: int         # 主色调（引用 42 色容器）
    dynamic_arc*: string         # 力度弧线：渐强/渐弱/平稳/波动

# ============================================================
# 第三层：段落层（Section / Movement）
# 一组短语 = 一个音乐段落
# ============================================================
type
  Section* = object
    phrases*: seq[Phrase]       # 包含的短语序列
    section_type*: string        # 类型：呈示/发展/高潮/收尾
    tempo*: int                  # 速度 (BPM)
    key*: string                 # 调性（引用声母容器）
    color_palette*: seq[int]     # 该段落的色彩配置（引用 42 色容器）
    emotion_arc*: string         # 段落情感弧线

# ============================================================
# 容器操作
# ============================================================

# 创建原子
func new_atomic*(pitch: int, timbre: string, dur, vel, color: int, emotion: string): AtomicSound =
  result.pitch = pitch
  result.timbre = timbre
  result.duration = dur
  result.velocity = vel
  result.color = color
  result.emotion = emotion

# 创建短语
func new_phrase*(atoms: seq[AtomicSound], ptype: string, breath, color: int, arc: string): Phrase =
  result.atoms = atoms
  result.phrase_type = ptype
  result.breath = breath
  result.color_dominant = color
  result.dynamic_arc = arc

# 创建段落
func new_section*(phrases: seq[Phrase], stype: string, tempo: int, key: string, colors: seq[int], arc: string): Section =
  result.phrases = phrases
  result.section_type = stype
  result.tempo = tempo
  result.key = key
  result.color_palette = colors
  result.emotion_arc = arc

# ============================================================
# 层级遍历
# ============================================================

# 段落中的原子总数
func atom_count*(sec: Section): int =
  result = 0
  for ph in sec.phrases:
    result += ph.atoms.len

# 段落总时长（ticks）
func total_duration*(sec: Section): int =
  result = 0
  for ph in sec.phrases:
    for a in ph.atoms:
      result += a.duration
    result += ph.breath

# 段落的颜色序列
func section_colors*(sec: Section): seq[string] =
  result = @[]
  for ph in sec.phrases:
    for a in ph.atoms:
      result.add("c" & $a.color)

# 段落的音高序列
func section_pitches*(sec: Section): seq[int] =
  result = @[]
  for ph in sec.phrases:
    for a in ph.atoms:
      result.add(a.pitch)

# ============================================================
# 跨容器引用
# ============================================================

# 音高 → 声母容器映射（示例）
func pitch_to_shengmu*(pitch: int): string =
  # 简化映射：每八度对应一个声母循环
  const sm = ["b","p","m","f","d","t","n","l","g","k","h","w"]
  let idx = pitch mod 12
  return sm[idx]

# 音色 → 变音容器符号（示例）
func timbre_to_diacritic*(timbre: string): string =
  case timbre
  of "piano":     return "ạ"   # 下点 = 沉
  of "strings":   return "å"   # 上圈 = 飘
  of "brass":     return "a̋"   # 双锐 = 尖锐
  of "woodwind":  return "ã"   # 波浪 = 颤
  of "voice":     return "â"   # 帽子 = 升降
  else:           return "a"

# 情感 → 颜色容器索引（示例）
func emotion_to_color*(emotion: string): int =
  case emotion
  of "激情":  return 1    # 大红
  of "宁静":  return 18   # 深绿
  of "忧伤":  return 30   # 深蓝
  of "快乐":  return 13   # 明黄
  of "神秘":  return 36   # 深紫
  else:       return 0    # 深红

when is_main_module:
  echo "=== 层级化音乐容器 ==="
  echo ""

  #示例：创建一个 4 音符的短语
  let atoms = @[
    new_atomic(60, "ạ", 480, 80, 1, "激情"),   # C4, 沉
    new_atomic(64, "å", 480, 70, 18, "宁静"),   # E4, 飘
    new_atomic(67, "ã", 480, 75, 13, "快乐"),   # G4, 颤
    new_atomic(72, "â", 960, 90, 36, "神秘"),   # C5, 升降
  ]

  let phrase = new_phrase(atoms, "问句", 120, 1, "渐强")
  let section = new_section(@[phrase], "呈示", 80, "b", @[1,18,13,36], "起承转合")

  echo "段落信息:"
  echo "  类型: ", section.section_type
  echo "  速度: ", section.tempo, " BPM"
  echo "  调性: ", section.key
  echo "  原子数: ", section.atom_count()
  echo "  总时长: ", section.total_duration(), " ticks"
  echo "  音高序列: ", section_pitches(section)
  echo "  颜色序列: ", section_colors(section)
  echo ""
  echo "跨容器引用:"
  echo "  音高 60 → 声母: ", pitch_to_shengmu(60)
  echo "  音色 piano → 变音: ", timbre_to_diacritic("piano")
  echo "  情感 激情 → 颜色: c", emotion_to_color("激情")
