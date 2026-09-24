# 满全法 · 韵母横版注册 (自动生成，勿手改)
# 韵母数: 31, 分身总数: 177
# 架构: 横版=多分身共存, 竖版=规范选择, 分身=同身份替身

import tables

type
  AvatarIndex* = Table[string, string]

# ai
const YV_AI* = @["ꬰ", "ꬱ", "Ѧ", "ѧ", "Ⴇ", "∀", "ɐ", "Ɐ", "ꜽ", "Ꜽ"]

# ei
const YV_EI* = @["ɘ", "Ǝ", "ə", "Ə", "ӭ", "Ӭ"]

# ui
const YV_UI* = @["ѱ", "Ѱ", "Ⴤ", "ⴤ", "ѽ", "Ѽ", "ѿ", "Ѿ"]

# ao
const YV_AO* = @["ꜵ", "Ꜵ", "ɒ", "Ɒ"]

# ou
const YV_OU* = @["ȣ", "Ȣ", "ꙍ", "Ꙍ", "႘"]

# iu
const YV_IU* = @["ꭎ", "Ⴞ", "ⴞ", "⊱", "⊰"]

# ie
const YV_IE* = @["ɤ", "Ɣ", "ɣ", "Ꙋ", "ѥ", "Ѥ"]

# üe
const YV_UE* = @["ᵫ", "ꭂ", "ⱷ"]

# er
const YV_ER* = @["ɛ", "Ɛ", "є", "Є", "ɘ", "Ǝ"]

# an
const YV_AN* = @["ꙙ", "Ꙙ", "ã", "Ã"]

# en
const YV_EN* = @["ⱄ", "Ⱄ", "ⱅ", "Ⱅ", "ⰶ", "Ⰶ", "ẽ", "Ẽ"]

# in
const YV_IN* = @["ꞃ", "Ꞃ", "ꙉ", "Ꙉ", "ꭅ", "ႁ", "ⲗ", "Ⲗ", "ĩ", "Ĩ"]

# un
const YV_UN* = @["տ", "໙", "ꦮ", "ꦱ", "Ⴐ", "ⴐ"]

# ün
const YV_UN* = @["ỹ", "Ỹ", "ꞟ", "Ꞟ", "ઇ", "ଓ"]

# ang
const YV_ANG* = @["Ⴆ", "Ⴊ", "Ⴉ", "ⴉ", "ᶏ", "ᶐ"]

# eng
const YV_ENG* = @["ӫ", "Ӫ", "Ⱞ", "ⱞ", "ᶒ", "ᶓ"]

# ing
const YV_ING* = @["ꙟ", "Ꙟ", "ꞃ", "Ꞃ"]

# ong
const YV_ONG* = @["ꭁ", "ꭀ", "Ⴍ", "Ⴓ", "ǫ", "Ǫ", "ȏ", "Ȏ"]

# ia
const YV_IA* = @["ꙗ", "Ꙗ"]

# iao
const YV_IAO* = @["ᧀ", "ᦿ", "ꮂ", "ᴥ", "ᦾ"]

# ua
const YV_UA* = @["ã", "Ã", "ʉ", "Ʉ", "ᳫ", "ᳪ", "ᰝ", "ꭏ", "ǟ", "Ǟ"]

# uai
const YV_UAI* = @["ꜻ", "Ꜻ", "Ꜽ", "ꜽ", "ɝ", "ꝡ", "Ꝡ"]

# uo
const YV_UO* = @["ơ", "Ơ", "ꭃ", "ꭄ", "ꜿ", "Ꜿ", "ȫ", "Ȫ", "ꙍ", "Ꙍ"]

# ian
const YV_IAN* = @["ꙝ", "Ꙝ"]

# uan
const YV_UAN* = @["Ⴄ", "Ⴂ", "Ⴆ", "Ⴊ", "ꭋ", "ꭌ", "ꦰ", "ꦲ"]

# üan
const YV_UAN* = @["ѫ", "Ѫ", "ʚ", "ɞ"]

# iang
const YV_IANG* = @["ѭ", "Ѭ", "ѩ", "Ѩ"]

# uang
const YV_UANG* = @["ꙛ", "Ꙛ", "๓", "ღ"]

# iong
const YV_IONG* = @["ȱ", "Ȱ", "ю", "Ю", "ѻ", "Ѻ"]

# duang
const YV_DUANG* = @["ⴋ", "Ⴋ"]

# huang
const YV_HUANG* = @["ⴌ", "Ⴌ"]

# 韵母 → 分身序表
const YUNMU_VARIANTS* = {
  "ai": YV_AI,
  "ei": YV_EI,
  "ui": YV_UI,
  "ao": YV_AO,
  "ou": YV_OU,
  "iu": YV_IU,
  "ie": YV_IE,
  "üe": YV_UE,
  "er": YV_ER,
  "an": YV_AN,
  "en": YV_EN,
  "in": YV_IN,
  "un": YV_UN,
  "ün": YV_UN,
  "ang": YV_ANG,
  "eng": YV_ENG,
  "ing": YV_ING,
  "ong": YV_ONG,
  "ia": YV_IA,
  "iao": YV_IAO,
  "ua": YV_UA,
  "uai": YV_UAI,
  "uo": YV_UO,
  "ian": YV_IAN,
  "uan": YV_UAN,
  "üan": YV_UAN,
  "iang": YV_IANG,
  "uang": YV_UANG,
  "iong": YV_IONG,
  "duang": YV_DUANG,
  "huang": YV_HUANG,
}.toTable

# 分身索引 (惰性构建)
var g_avatar_index*: AvatarIndex = initTable[string, string]()
var g_built* = false

proc build_avatar_index*() =
  if g_built: return
  for ym, syms in YUNMU_VARIANTS:
    for s in syms:
      if not g_avatar_index.hasKey(s):
        g_avatar_index[s] = ym
  g_built = true

proc resolve*(symbol: string): string =
  build_avatar_index()
  if g_avatar_index.hasKey(symbol): return g_avatar_index[symbol]
  return ""

proc vertical*(yunmu: string): string =
  if YUNMU_VARIANTS.hasKey(yunmu) and YUNMU_VARIANTS[yunmu].len > 0:
    return YUNMU_VARIANTS[yunmu][0]
  return ""