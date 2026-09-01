# 诊断：读入源代码后，直接用 split("}") 看 token 结构
import unicode
import strutils

const SRC* =
  "Ȍ}ȍ}Ꙙ}ꙙ}Ȑ}ȑ}Ӱ}ӱ}Ü}ü}" &
  "Ѧ}ѧ}Ӗ}ӗ}Ə}ə}Ѽ}ѽ}Ѱ}ѱ}Ꜵ}ꜵ}Ȣ}ȣ}Ꙍ}ꙍ}" &
  "Ⴘ}Ⴏ}Ⴎ}Ѥ}ѥ}Ё}ё}ᵫ}ꭂ}Є}є}Ꙗ}ꙗ}" &
  "ⱄ}ⱅ}ѩ}Ǟ}ǟ}ꬰ}ꬱ}Ꜻ}ꜻ}Ȫ}ȫ}Ⴊ}Ⴆ}Ѿ}ѿ}ꭃ}ꭄ}" &
  "Ꙙ}ꙙ}ɤ}ɣ}ꙝ}Ꙝ}Ⴄ}Ⴂ}Ѫ}ѫ}ꭌ}Ⴋ}Ⴉ}" &
  "ӫ}Ӭ}Ꙟ}ꙟ}Ȏ}ȏ}Ⴓ}Ⴍ}ꭀ}ꭁ}ѭ}Ѭ}ꙛ}Ꙛ}Ⴔ}Ⴇ}Ȱ}ȱ}Ѻ}ѻ"

let runes = toRunes(SRC)
echo "runes = ", runes.len

# 按 } 切
var toks: seq[string]
var cur = ""
for r in runes:
  if r == Rune(0x7D):
    if cur != "":
      toks.add(cur)
      cur = ""
  else:
    cur.add($r)
if cur != "":
  toks.add(cur)

echo "tokens (非空) = ", toks.len

# 配对
var pairs = toks.len div 2
echo "对数 = ", pairs

for i in 0 ..< pairs:
  let a = toks[2*i]
  let b = toks[2*i+1]
  let ca = int32(toRunes(a)[0])
  let cb = int32(toRunes(b)[0])
  echo i, ": U+", toHex(ca, 4), " U+", toHex(cb, 4), "  (", a, " / ", b, ")"

if toks.len mod 2 == 1:
  echo "多余单 token: ", toks[toks.len-1]
