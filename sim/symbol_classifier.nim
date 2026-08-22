## ═══════════════════════════════════════════════════════════════
## symbol_classifier.nim — 符号分类系统
## 音调类型 / 韵母类型 / 分身类型 / 备用类型
## ═══════════════════════════════════════════════════════════════
##
## 分类原则:
## 1. 音调类型: 同一个基础音+不同音调 → 独立容器
## 2. 韵母类型: 主音(a/o/e/i/u/ü) → 独立容器
## 3. 分身类型: 长得一模一样 → 独立出来放同一容器
## 4. 备用类型: 富余符号 → 多套备用容器用于视觉搭配组合
##
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, algorithm, sets, tables]

type
  SymbolInfo* = object
    ch*: string
    base*: string        # 基础音 (如 "a", "o", "e", "ai")
    tone_type*: string   # 声调类型 ("轻声", "1声", "2声", "3声", "4声", "下点", "倒短音", "双重音", "上圈", "反尾", "波浪", "短音", "反钝音")
    group*: string       # 分类 ("音调", "韵母", "分身", "备用", "复韵母", "鼻韵母")
    variant_of*: string   # 是哪个符号的异体
    source*: string      # 原始来源行

  ClassResult* = object
    category*: string    # 大类名
    subcat*: string      # 子分类名 (如 "a音调", "o韵母")
    kind*: string        # "音调" / "韵母" / "分身" / "备用" / "复韵母" / "鼻韵母" / "辅音"
    symbols*: seq[SymbolInfo]
    container_name*: string
    base_count*: int     # 基础符号数
    spare_count*: int    # 备用数
    version*: string
    description*: string

# ═══════════════════════════════════════════════════════════════
## 基础元音定义 (FQ-YM6)
## ═══════════════════════════════════════════════════════════════

const BASE_YIN* = @["a", "o", "e", "i", "u", "ü"]

# ═══════════════════════════════════════════════════════════════
## 声调标记类型 (从对话提取的60进制)
## ═══════════════════════════════════════════════════════════════

let TONE_GROUPS* = [
  ("轻声", @["a", "e", "i", "o", "u", "ü"]),
  ("1声长音", @["ā", "ē", "ī", "ō", "ū", "ǖ"]),
  ("2声锐音", @["á", "é", "í", "ó", "ú", "ǘ"]),
  ("3声抑扬", @["ǎ", "ě", "ǐ", "ǒ", "ǔ", "ǚ"]),
  ("4声钝音", @["à", "è", "ì", "ò", "ù", "ǜ"]),
]

# ═══════════════════════════════════════════════════════════════
## 高级声调类型 (从100进制韵母提取)
## ═══════════════════════════════════════════════════════════════

let ADV_TONE_GROUPS* = [
  ("下点", @["ạ", "ẹ", "ị", "ọ", "ụ", "ä", "ë", "ï", "ö", "ü"]),
  ("倒短音", @["ȃ", "ȇ", "ȋ", "ȏ", "ȗ", "a̋", "e̋", "i̋", "ő", "ű"]),
  ("双重音", @["å", "e̊", "i̊", "o̊", "ů", "ą", "ę", "į", "ǫ", "ų"]),
  ("波浪帽", @["ã", "ẽ", "ĩ", "õ", "ũ", "â", "ê", "î", "ô", "û"]),
  ("短音反钝", @["ă", "ĕ", "ĭ", "ŏ", "ŭ", "ȁ", "ȅ", "ȉ", "ȍ", "ȕ"]),
]

# ═══════════════════════════════════════════════════════════════
## 分类主函数
## ═══════════════════════════════════════════════════════════════

proc classifyAll*(): seq[ClassResult] =
  # ═══════════════════════════════════════════════════════════
  ## 类别1: 音调类型 — 60进制 · 4声×6音=24 + 轻声6 = 30小写
  ## ═══════════════════════════════════════════════════════════
  var tone_container = ClassResult(
    category: "音调类型",
    subcat: "声调分层",
    kind: "音调",
    container_name: "FQ-TONE60",
    base_count: 30,
    version: "v1.0",
    description: "6个基础音 × 5声调组(轻声/1/2/3/4) · 独立容器",
  )
  for (tone, syms) in TONE_GROUPS:
    for s in syms:
      tone_container.symbols.add(SymbolInfo(
        ch: s, base: s, tone_type: tone,
        group: "音调", variant_of: s, source: "60进制时钟系统",
      ))
  result.add(tone_container)

  # ═══════════════════════════════════════════════════════════
  ## 类别2: 高级音调 — 100进制 · 5层×10符号=50小写
  ## ═══════════════════════════════════════════════════════════
  var adv_tone = ClassResult(
    category: "音调类型",
    subcat: "高级声调",
    kind: "音调",
    container_name: "FQ-ATONE50",
    base_count: 50,
    version: "v1.0",
    description: "5层音调(下点/倒短音/双重音/波浪帽/短音反钝) · 100进制",
  )
  for (layer, syms) in ADV_TONE_GROUPS:
    for s in syms:
      adv_tone.symbols.add(SymbolInfo(
        ch: s, base: s, tone_type: layer,
        group: "音调", variant_of: s, source: "100进制韵母系统",
      ))
  result.add(adv_tone)

  # ═══════════════════════════════════════════════════════════
  ## 类别3: 韵母类型 — 按主音分6个容器 · 显式枚举
  ## ═══════════════════════════════════════════════════════════
  const ym_a_content = @[
    ("a", "轻声音"), ("ā", "1声长音"), ("á", "2声锐音"), ("ǎ", "3声抑扬"), ("à", "4声钝音"),
    ("â", "帽子"), ("ã", "波浪"), ("ă", "短音"), ("ȁ", "反钝音"), ("ạ", "下点"),
    ("ä", "下点分"), ("å", "上圈"), ("ȃ", "倒短音"), ("ą", "反尾"),
    ("A", "轻声大写"), ("Ā", "1声大写"), ("Á", "2声大写"), ("Ǎ", "3声大写"), ("À", "4声大写"),
    ("Â", "帽子大写"), ("Ã", "波浪大写"), ("Ă", "短音大写"), ("Ȁ", "反钝大写"), ("Ạ", "下点大写"),
    ("Ä", "下点分大写"), ("Å", "上圈大写"), ("Ȃ", "倒短音大写"), ("Ą", "反尾大写"),
  ]
  var yunmu_a = ClassResult(
    category: "韵母类型", subcat: "a韵母族", kind: "韵母",
    container_name: "FQ-YM-a", base_count: ym_a_content.len, version: "v1.0",
    description: "主音a的所有变体和声调 · 独立容器",
  )
  for (ch, desc) in ym_a_content:
    yunmu_a.symbols.add(SymbolInfo(ch: ch, base: "a", tone_type: desc, group: "韵母", variant_of: ch, source: "a韵母族显式枚举"))
  result.add(yunmu_a)

  const ym_o_content = @[
    ("o", "轻声"), ("ō", "1声"), ("ó", "2声"), ("ǒ", "3声"), ("ò", "4声"),
    ("ô", "帽子"), ("õ", "波浪"), ("ŏ", "短音"), ("ȍ", "倒短音"), ("ȏ", "上圈"),
    ("ọ", "下点"), ("ö", "下点分"), ("ő", "双重音"), ("ǫ", "反尾"),
    ("O", "轻声大写"), ("Ō", "1声大写"), ("Ó", "2声大写"), ("Ǒ", "3声大写"), ("Ò", "4声大写"),
    ("Ô", "帽子大写"), ("Õ", "波浪大写"), ("Ŏ", "短音大写"), ("Ȏ", "倒短音大写"), ("Ȑ", "上圈大写"),
    ("Ọ", "下点大写"), ("Ö", "下点分大写"), ("Ő", "双重音大写"), ("Ǫ", "反尾大写"),
  ]
  var yunmu_o = ClassResult(
    category: "韵母类型", subcat: "o韵母族", kind: "韵母",
    container_name: "FQ-YM-o", base_count: ym_o_content.len, version: "v1.0",
    description: "主音o的所有变体和声调 · 独立容器",
  )
  for (ch, desc) in ym_o_content:
    yunmu_o.symbols.add(SymbolInfo(ch: ch, base: "o", tone_type: desc, group: "韵母", variant_of: ch, source: "o韵母族显式枚举"))
  result.add(yunmu_o)

  const ym_e_content = @[
    ("e", "轻声"), ("ē", "1声"), ("é", "2声"), ("ě", "3声"), ("è", "4声"),
    ("ê", "帽子"), ("ẽ", "波浪"), ("ĕ", "短音"), ("ȅ", "反钝音"), ("ȇ", "倒短音"),
    ("ẹ", "下点"), ("ë", "下点分"), ("ę", "反尾"),
    ("E", "轻声大写"), ("Ē", "1声大写"), ("É", "2声大写"), ("Ě", "3声大写"), ("È", "4声大写"),
    ("Ê", "帽子大写"), ("Ẽ", "波浪大写"), ("Ĕ", "短音大写"), ("Ȅ", "反钝大写"), ("Ȇ", "倒短音大写"),
    ("Ẹ", "下点大写"), ("Ë", "下点分大写"), ("Ę", "反尾大写"),
  ]
  var yunmu_e = ClassResult(
    category: "韵母类型", subcat: "e韵母族", kind: "韵母",
    container_name: "FQ-YM-e", base_count: ym_e_content.len, version: "v1.0",
    description: "主音e的所有变体和声调 · 独立容器",
  )
  for (ch, desc) in ym_e_content:
    yunmu_e.symbols.add(SymbolInfo(ch: ch, base: "e", tone_type: desc, group: "韵母", variant_of: ch, source: "e韵母族显式枚举"))
  result.add(yunmu_e)

  const ym_i_content = @[
    ("i", "轻声"), ("ī", "1声"), ("í", "2声"), ("ǐ", "3声"), ("ì", "4声"),
    ("î", "帽子"), ("ĩ", "波浪"), ("ĭ", "短音"), ("ȉ", "反钝音"), ("ȋ", "倒短音"),
    ("ị", "下点"), ("ï", "下点分"), ("į", "反尾"),
    ("I", "轻声大写"), ("Ī", "1声大写"), ("Í", "2声大写"), ("Ǐ", "3声大写"), ("Ì", "4声大写"),
    ("Î", "帽子大写"), ("Ĩ", "波浪大写"), ("Ĭ", "短音大写"), ("Ȉ", "反钝大写"), ("Ȋ", "倒短音大写"),
    ("Ị", "下点大写"), ("Ï", "下点分大写"), ("Į", "反尾大写"),
  ]
  var yunmu_i = ClassResult(
    category: "韵母类型", subcat: "i韵母族", kind: "韵母",
    container_name: "FQ-YM-i", base_count: ym_i_content.len, version: "v1.0",
    description: "主音i的所有变体和声调 · 独立容器",
  )
  for (ch, desc) in ym_i_content:
    yunmu_i.symbols.add(SymbolInfo(ch: ch, base: "i", tone_type: desc, group: "韵母", variant_of: ch, source: "i韵母族显式枚举"))
  result.add(yunmu_i)

  const ym_u_content = @[
    ("u", "轻声"), ("ū", "1声"), ("ú", "2声"), ("ǔ", "3声"), ("ù", "4声"),
    ("û", "帽子"), ("ũ", "波浪"), ("ŭ", "短音"), ("ȕ", "反钝音"), ("ȗ", "倒短音"),
    ("ụ", "下点"), ("ů", "上圈"), ("ų", "反尾"),
    ("U", "轻声大写"), ("Ū", "1声大写"), ("Ú", "2声大写"), ("Ǔ", "3声大写"), ("Ù", "4声大写"),
    ("Û", "帽子大写"), ("Ũ", "波浪大写"), ("Ŭ", "短音大写"), ("Ȕ", "反钝大写"), ("Ȗ", "倒短音大写"),
    ("Ụ", "下点大写"), ("Ů", "上圈大写"), ("Ų", "反尾大写"),
  ]
  var yunmu_u = ClassResult(
    category: "韵母类型", subcat: "u韵母族", kind: "韵母",
    container_name: "FQ-YM-u", base_count: ym_u_content.len, version: "v1.0",
    description: "主音u的所有变体和声调 · 独立容器",
  )
  for (ch, desc) in ym_u_content:
    yunmu_u.symbols.add(SymbolInfo(ch: ch, base: "u", tone_type: desc, group: "韵母", variant_of: ch, source: "u韵母族显式枚举"))
  result.add(yunmu_u)

  const ym_v_content = @[
    ("ü", "轻声"), ("ǖ", "1声"), ("ǘ", "2声"), ("ǚ", "3声"), ("ǜ", "4声"),
    ("ӗ", "倒短音小写"), ("Ӗ", "倒短音大写"),
  ]
  var yunmu_v = ClassResult(
    category: "韵母类型", subcat: "ü韵母族", kind: "韵母",
    container_name: "FQ-YM-ü", base_count: ym_v_content.len, version: "v1.0",
    description: "主音ü的所有变体和声调 · 独立容器",
  )
  for (ch, desc) in ym_v_content:
    yunmu_v.symbols.add(SymbolInfo(ch: ch, base: "ü", tone_type: desc, group: "韵母", variant_of: ch, source: "ü韵母族显式枚举"))
  result.add(yunmu_v)

  # ═══════════════════════════════════════════════════════════
  ## 类别4: 韵母类型 — 复韵母(组合韵母)
  ## ═══════════════════════════════════════════════════════════
  var compound_yunmu = ClassResult(
    category: "韵母类型",
    subcat: "复韵母族",
    kind: "复韵母",
    container_name: "FQ-YM-CMPD",
    base_count: 140,
    version: "v1.0",
    description: "10个复韵母 × 变体 · 独立容器",
  )
  const compound_list = [
    ("ai", @["ꬰ", "ꬱ", "Ѧ", "ѧ", "Ɐ", "ɐ"]),
    ("ei", @["ə", "Ə", "Ӭ", "ӭ"]),
    ("ui", @["Ѽ", "ѽ", "Ѱ", "ѱ", "Ѿ", "ѿ"]),
    ("ao", @["ɒ", "Ꜵ", "ꜵ"]),
    ("ou", @["Ȣ", "ȣ", "႘"]),
    ("iu", @["Ⴞ", "ꭎ", "Ⴏ", "Ⴎ"]),
    ("ie", @["ɤ", "ɣ", "Ꙋ", "Ɣ", "ɬ", "ꝷ", "Ѥ", "ѥ"]),
    ("üe", @["ᵫ", "ꭂ", "Ё", "ё", "ʊ̈", "ⱷ"]),
    ("er", @["ɛ", "Ɛ", "ꝵ", "ꝶ", "ɘ", "Ǝ", "ꞟ", "Ꞟ", "Ⰵ"]),
  ]
  for (name, syms) in compound_list:
    for s in syms:
      compound_yunmu.symbols.add(SymbolInfo(
        ch: s, base: name, tone_type: "组合",
        group: "复韵母", variant_of: name, source: "复韵母提取",
      ))
  result.add(compound_yunmu)

  # ═══════════════════════════════════════════════════════════
  ## 类别5: 分身类型 — 长得一模一样的符号
  ## ═══════════════════════════════════════════════════════════
  var clone = ClassResult(
    category: "分身类型",
    subcat: "同形异用",
    kind: "分身",
    container_name: "FQ-CLONE",
    base_count: 12,
    version: "v1.0",
    description: "长得一模一样但用途不同的符号 · 独立容器作为备用方案",
  )
  const clone_list = [
    ("Ⴐ", "ün小写大写同形", "ün"),
    ("႘", "ou大写出现两次", "ou"),
    ("Ⴓ", "ong变体出现多次", "ong"),
    ("Ⴍ", "ong另一变体", "ong"),
    ("â", "帽子韵母a", "a"),
    ("ê", "帽子韵母e", "e"),
    ("î", "帽子韵母i", "i"),
    ("ô", "帽子韵母o", "o"),
    ("û", "帽子韵母u", "u"),
    ("Ä", "下点韵母A同时是A的大写", "a"),
    ("ə/Ə", "ei小写=ə, 大写=Ə", "ei"),
  ]
  for (ch, desc, src) in clone_list:
    clone.symbols.add(SymbolInfo(
      ch: ch, base: src, tone_type: "分身",
      group: "分身", variant_of: src, source: desc,
    ))
  result.add(clone)

  # ═══════════════════════════════════════════════════════════
  ## 类别6: 备用类型 — 多套备用容器 · 用于视觉搭配
  ## ═══════════════════════════════════════════════════════════
  for set_id in 1..3:
    var spare = ClassResult(
      category: "备用类型",
      subcat: "备用集" & $set_id,
      kind: "备用",
      container_name: "FQ-SPARE" & $set_id,
      base_count: 60,
      version: "v1." & $set_id,
      description: "富余符号 · 用于视觉匹配搭配 · 备用集" & $set_id,
    )
    case set_id
    of 1:
      # 备用集1: 鼻韵母全套
      const nasal_list = [
        ("an", @["Ꙙ", "ꙙ", "Ã", "ã"]),
        ("en", @["ⱄ", "ⱅ", "ⱺ", "Ⰶ", "Ẽ", "ẽ", "ꬴ", "ꬲ"]),
        ("in", @["ꞃ", "Ꞃ", "ⲗ", "Ⲗ", "ꙉ", "Ꙉ", "ႁ", "႔"]),
        ("un", @["տ", "໙", "ꦮ", "ꦱ", "ꦯ"]),
        ("ün", @["Ⴐ"]),
        ("ian", @["Ꙝ", "ꙝ"]),
        ("uan", @["Ⴄ", "Ⴂ", "Ⴅ", "ꦰ", "ꦲ", "ꭌ", "ꭏ", "ꭋ"]),
        ("üan", @["ѫ", "Ѫ", "ⴉ"]),
        ("ang", @["Ⴊ", "Ⴆ", "ᶏ", "ᶐ"]),
        ("eng", @["Ӫ", "ӫ", "ᶒ", "ⱺ"]),
        ("ing", @["Ꙟ", "ꙟ", "ꙗ", "႞"]),
        ("ong", @["ꭀ", "ꭁ", "Ȏ", "ȏ", "Ǫ", "ǫ"]),
        ("iang", @["ѭ", "Ѭ", "Ѩ", "ѩ"]),
        ("uang", @["ꙛ", "Ꙛ"]),
        ("iong", @["Ȱ", "ȱ", "Ѻ", "ѻ", "Ӻ"]),
        ("ueng", @["ɞ", "ɝ"]),
      ]
      for (name, syms) in nasal_list:
        for s in syms:
          spare.symbols.add(SymbolInfo(
            ch: s, base: name, tone_type: "鼻音",
            group: "备用", variant_of: name, source: "鼻韵母备用集1",
          ))
    of 2:
      # 备用集2: 复韵母扩展
      const ext_compound = [
        ("ai2", @["Ꜽ", "ꜽ"]),
        ("ei2", @["Ə", "ə"]),
        ("ao2", @["ɒ"]),
        ("ou2", @["႘", "Ⴏ", "Ⴎ"]),
        ("ie2", @["ɤ", "ɣ", "Ɣ", "Ꙋ", "Ѥ", "ѥ"]),
        ("üe2", @["ᵫ", "ꭂ", "ⱷ"]),
        ("er2", @["ꝵ", "ꝶ", "Ⰵ", "Ǝ", "ꞟ", "Ꞟ"]),
        ("ue", @["ᵫ"]),
        ("oi", @["Ꙕ", "ꙕ", "Ѹ", "ѹ"]),
        ("ao3", @["ɒ"]),
      ]
      for (name, syms) in ext_compound:
        for s in syms:
          spare.symbols.add(SymbolInfo(
            ch: s, base: name, tone_type: "组合",
            group: "备用", variant_of: name, source: "复韵母备用集2",
          ))
    of 3:
      # 备用集3: 特殊音节+独立辅音
      const special_syl = [
        ("aa", @["Ꜳ", "ꜳ"]),
        ("ae", @["Æ", "æ", "ᴁ", "ᴂ", "Ǽ", "ǽ"]),
        ("au", @["Ꜷ", "ꜷ", "ႀ"]),
        ("av", @["Ꜹ", "ꜹ"]),
        ("oo", @["Ꝏ", "ꝏ", "∞", "♾"]),
        ("ib", @["Ꙓ", "ꙓ"]),
        ("jiong", @["Ѻ", "ѻ", "ꝺ", "Ꝺ"]),
        ("xiong", @["Ȣ", "ȣ", "ꭔ", "ꭖ"]),
        ("kua", @["Ꞣ", "ꞣ"]),
        ("nua", @["Ꞥ", "ꞥ"]),
      ]
      for (name, syms) in special_syl:
        for s in syms:
          spare.symbols.add(SymbolInfo(
            ch: s, base: name, tone_type: "特殊",
            group: "备用", variant_of: name, source: "特殊音节备用集3",
          ))
    else: discard
    result.add(spare)

  # ═══════════════════════════════════════════════════════════
  ## 类别7: 辅音类型(声母+特殊辅音)
  ## ═══════════════════════════════════════════════════════════
  var consonant = ClassResult(
    category: "辅音类型",
    subcat: "声母+特殊辅音",
    kind: "辅音",
    container_name: "FQ-SM-PLUS",
    base_count: 80,
    version: "v1.0",
    description: "25声母+大写+特殊辅音ⴀ-ⴿ · 独立容器",
  )
  const sm_list = [
    ("b", "唇音"), ("p", "唇音"), ("m", "唇音"), ("f", "唇音"), ("v", "唇齿"),
    ("d", "舌音"), ("t", "舌音"), ("n", "舌音"), ("l", "舌音"),
    ("g", "牙音"), ("k", "牙音"), ("h", "喉音"), ("w", "喉音"),
    ("j", "腭音"), ("q", "腭音"), ("x", "腭音"), ("y", "半舌"),
    ("z", "齿音"), ("c", "齿音"), ("s", "齿音"), ("r", "半齿"),
    ("ȥ", "齿音"), ("ç", "齿音"), ("ş", "齿音"), ("ŋ", "鼻音"),
  ]
  for (s, grp) in sm_list:
    consonant.symbols.add(SymbolInfo(
      ch: s, base: s, tone_type: "辅音",
      group: "辅音", variant_of: s, source: grp,
    ))
  for (s, grp) in sm_list:
    consonant.symbols.add(SymbolInfo(
      ch: s.toUpper(), base: s.toUpper(), tone_type: "辅音",
      group: "辅音", variant_of: s.toUpper(), source: grp,
    ))
  # ⴀ-ⴿ 特殊辅音
  for ch in ["ⴀ","ⴆ","ⴌ","ⴇ","ⴶ","ⴱ","ⴴ","ⴸ","ⴳ","ⴲⴻ","ⴺ","ⴹ","ⴷ","ⴽ","ⴼ","ⴿ","ⴾ","ⴵ","ⴔ","ⴃ","ⴒ","ⴎ","ⴈ","ⴐ","ⴂ","ⴓ","ⴕ","ⴊ","ⴁ","ⴄ","ⴍ","ⴚ","ⴜ","ⴛ","ⴞ","ⴝ"]:
    consonant.symbols.add(SymbolInfo(
      ch: ch, base: ch, tone_type: "辅音",
      group: "辅音", variant_of: ch, source: "ⴀ-ⴿ特殊辅音",
    ))
  result.add(consonant)

# ═══════════════════════════════════════════════════════════════
## 分类结果展示
## ═══════════════════════════════════════════════════════════════

proc classifyReport*(classes: seq[ClassResult]): string =
  var lines: seq[string] = @[]
  lines.add("═══════════════════════════════════════════════════════")
  lines.add("  符号分类系统 · 音调/韵母/分身/备用/复韵/辅音")
  lines.add("═══════════════════════════════════════════════════════")
  lines.add("")

  var total_symbols = 0
  for cls in classes:
    total_symbols += cls.symbols.len

  lines.add("总分类数: " & $classes.len)
  lines.add("总符号数: " & $total_symbols)
  lines.add("")

  for cls in classes:
    lines.add("── " & cls.container_name & " ──")
    lines.add("  类别: " & cls.category)
    lines.add("  子类: " & cls.subcat)
    lines.add("  类型: " & cls.kind)
    lines.add("  符号数: " & $cls.symbols.len)
    lines.add("  版本: " & cls.version)
    lines.add("  说明: " & cls.description)

    # 显示符号预览 (最多20个)
    var preview: seq[string] = @[]
    for i, sym in cls.symbols:
      if i >= 20:
        preview.add("...")
        break
      preview.add(sym.ch)
    lines.add("  符号: " & preview.join(" "))
    lines.add("")

  result = lines.join("\n")

proc detailReport*(classes: seq[ClassResult]): string =
  var lines: seq[string] = @[]
  for cls in classes:
    if cls.kind in ["分身", "备用"]:
      lines.add("── " & cls.container_name & " (详细) ──")
      for sym in cls.symbols:
        lines.add("  " & sym.ch & " → " & sym.source)
      lines.add("")
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
## ═══════════════════════════════════════════════════════════════

when isMainModule:
  let classes = classifyAll()
  echo classifyReport(classes)
  echo ""
  echo "── 分身/备用详细 ──"
  echo detailReport(classes)