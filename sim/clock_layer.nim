## 时钟层 · 声调韵母60进制
## 嵌套时钟结构: 秒层(÷60) → 分层的(÷60) → 时层
## 码元即位置 · 字母自己携带值
## 4个声调 + 轻声 = 5个时态层


# ═══════════════════════════════════════════════════════════════
# 时钟码元 60 个 · 即进制的位子
# ═══════════════════════════════════════════════════════════════

const CLOCK_60* = [
  "a", "e", "i", "o", "u", "ü",   # 0-5 轻声
  "A", "E", "I", "O", "U", "Ü",   # 6-11 轻声大
  "ā", "ē", "ī", "ō", "ū", "ǖ",   # 12-17 1声
  "Ā", "Ē", "Ī", "Ō", "Ū", "Ǖ",   # 18-23 1声大
  "á", "é", "í", "ó", "ú", "ǘ",   # 24-29 2声
  "Á", "É", "Í", "Ó", "Ú", "Ǘ",   # 30-35 2声大
  "ǎ", "ě", "ǐ", "ǒ", "ǔ", "ǚ",   # 36-41 3声
  "Ǎ", "Ě", "Ǐ", "Ǒ", "Ǔ", "Ǚ",   # 42-47 3声大
  "à", "è", "ì", "ò", "ù", "ǜ",   # 48-53 4声
  "À", "È", "Ì", "Ò", "Ù", "Ǜ",   # 54-59 4声大
]

# ═══════════════════════════════════════════════════════════════
# 时钟 → 数字 · 码元即位置
# ═══════════════════════════════════════════════════════════════

proc clock_value*(c: string): int =
  for i, s in CLOCK_60:
    if s == c: return i
  return 0

# ═══════════════════════════════════════════════════════════════
# 嵌套时钟 · 秒+分+时 · 用时钟码元表达三维时间坐标
# ═══════════════════════════════════════════════════════════════

type
  ClockCoord* = object
    miao*: string   # 秒层码元 (÷60)
    fen*: string    # 分层码元 (÷60)
    shi*: string    # 时层码元 (÷24 · 超出部分进位)

  ClockAddress* = object
    coord*: ClockCoord
    ticks*: int      # 绝对滴答 (从0滴答起)

proc new_clock*(miao, fen, shi: string): ClockAddress =
  result.coord.miao = miao
  result.coord.fen = fen
  result.coord.shi = shi
  let mv = clock_value(miao)
  let fv = clock_value(fen)
  let sv = clock_value(shi)
  result.ticks = sv * 3600 + fv * 60 + mv

proc clock_tick*(self: ClockAddress): ClockAddress =
  let total = self.ticks + 1
  let miao = total mod 60
  let fen = (total div 60) mod 60
  let shi = (total div 3600) mod 24
  result.coord.miao = CLOCK_60[miao]
  result.coord.fen = CLOCK_60[fen]
  result.coord.shi = CLOCK_60[shi]
  result.ticks = total

proc clock_to_container_index*(self: ClockAddress, containers: seq[int]): int =
  let idx = self.ticks mod containers.len
  return idx

proc clock_tier*(self: ClockAddress): int =
  ## 时钟层级: 秒=1 · 分=2 · 时=3 · 以主导位判定
  let mv = clock_value(self.coord.miao)
  let fv = clock_value(self.coord.fen)
  if fv >= 12: return 3   # 时层有声 → 高
  elif mv >= 12: return 2
  else: return 1

# ═══════════════════════════════════════════════════════════════
# 字符串显示
# ═══════════════════════════════════════════════════════════════

proc `$`*(c: ClockCoord): string =
  result = c.shi & ":" & c.fen & ":" & c.miao

proc clock_display*(self: ClockAddress): string =
  result = $self.coord
  result &= " (滴答=" & $self.ticks & ", 层级=" & $clock_tier(self) & ")"

when is_main_module:
  echo "═══ 时钟层 · 声调韵母60进制 ═══"
  echo ""
  echo "时钟码元: ", CLOCK_60.len, " 个"
  echo "轻声: a e i o u ü"
  echo "1声: ā ē ī ō ū ǖ"
  echo "2声: á é í ó ú ǘ"
  echo "3声: ǎ ě ǐ ǒ ǔ ǚ"
  echo "4声: à è ì ò ù ǜ"
  echo ""

  echo "── 时钟走动 10 步 ──"
  var clock = new_clock("a", "a", "a")
  for i in 0..<10:
    echo clock_display(clock)
    clock = clock.clock_tick()

  echo ""
  echo "── 跨越60秒 ──"
  var clock2 = new_clock("ǜ", "ǜ", "ǜ")
  echo "起点: ", clock_display(clock2)
  for i in 0..<3:
    for _ in 0..<60:
      clock2 = clock2.clock_tick()
    echo "走60×", i+1, " = ", clock_display(clock2)
