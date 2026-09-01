# 诊断：统计 FINAL_ENCODING_STRING 实际能解析出多少对
import unicode
import strutils

const FINAL_ENCODING_STRING* =
  "Ȍ}ȍ}Ꙙ}ꙙ}Ȑ}ȑ}Ӱ}ӱ}Ü}ü}" &
  "Ѧ}ѧ}Ӗ}ӗ}Ə}ə}Ѽ}ѽ}Ѱ}ѱ}Ꜵ}ꜵ}Ȣ}ȣ}Ꙍ}ꙍ}" &
  "Ⴘ}Ⴏ}Ⴎ}Ѥ}ѥ}Ё}ё}ᵫ}ꭂ}Є}є}Ꙗ}ꙗ}" &
  "ⱄ}ⱅ}ѩ}Ǟ}ǟ}ꬰ}ꬱ}Ꜻ}ꜻ}Ȫ}ȫ}Ⴊ}Ⴆ}Ѿ}ѿ}ꭃ}ꭄ}" &
  "Ꙙ}ꙙ}ɤ}ɣ}ꙝ}Ꙝ}Ⴄ}Ⴂ}Ѫ}ѫ}ꭌ}Ⴋ}Ⴉ}" &
  "ӫ}Ӭ}Ꙟ}ꙟ}Ȏ}ȏ}Ⴓ}Ⴍ}ꭀ}ꭁ}ѭ}Ѭ}ꙛ}Ꙛ}Ⴔ}Ⴇ}Ȱ}ȱ}Ѻ}ѻ"

let runes = toRunes(FINAL_ENCODING_STRING)
echo "总 rune 数: ", runes.len

# 统计 } 分隔符
var sep_count = 0
for r in runes:
  if Rune(0x7D) == r:
    sep_count += 1
echo "} 分隔符数: ", sep_count
echo "非} rune 数: ", runes.len - sep_count

# 手动配对走一遍
type Pair = tuple[u, l: int32]
var pairs: seq[Pair]
var i = 0
while i < runes.len:
  if runes[i] == Rune(0x7D):
    i += 1
    continue
  let cu = int32(runes[i])
  i += 1
  while i < runes.len and runes[i] == Rune(0x7D):
    i += 1
  let cl = if i < runes.len: int32(runes[i]) else: cu
  if i < runes.len and runes[i] != Rune(0x7D):
    i += 1
  pairs.add((cu, cl))

echo "实际解析出对数: ", pairs.len

# 打印每对
for idx, p in pairs:
  echo idx, ": U+", toHex(p.u, 4), " / U+", toHex(p.l, 4)
