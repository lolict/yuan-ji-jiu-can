## ═══════════════════════════════════════════════════════════════
## container_special.nim — 第2韵母/第2声母/剩余/分身 容器系统
## ═══════════════════════════════════════════════════════════════
##
## 从对话数据中提取的独立编码容器:
## 1. FQ-YM100: 第2韵母 (100进制, 50小写+50大写)
## 2. FQ-SM50+: 第2声母 (50+扩展)
## 3. FQ-CMPD: 复韵母容器 (从剩余提取)
## 4. FQ-NASAL: 鼻韵母容器 (从剩余提取)
## 5. FQ-SPCON: 特殊辅音容器 (ⴀ-ⴿ等)
## 6. FQ-CLONE: 分身容器 (长得一模一样的符号)
##
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, algorithm, sets]

# ═══════════════════════════════════════════════════════════════
## 容器类型定义
## ═══════════════════════════════════════════════════════════════

type
  SymbolEntry* = object
    ch*: string
    name*: string        # 说明 (如 "下点韵母", "长音1声")
    group*: string       # 所属家族 (如 "下点家族", "声调家族")

  CodeContainer* = object
    id*: string
    name*: string
    base*: int
    kind*: string         # "韵母" / "声母" / "复韵母" / "鼻韵母" / "辅音" / "分身"
    symbols*: seq[SymbolEntry]
    version*: string      # 版本号 (如 "v2.0")
    parent*: string       # 父容器ID
    history*: seq[string] # 历史版本

# ═══════════════════════════════════════════════════════════════
## 容器1: FQ-YM100 — 第2韵母 (100进制)
## 来源: 对话中确认的"最优版"100进制韵母集
## ═══════════════════════════════════════════════════════════════

proc makeFQ_YM100*(): CodeContainer =
  result.id = "FQ-YM100"
  result.name = "满全法韵母-100"
  result.base = 100
  result.kind = "韵母"
  result.version = "v2.0"
  result.parent = "FQ-YM66"  # 基于韵母层扩展
  result.history = @["v1.0(60进制时钟)", "v1.5(48+60拆用)", "v2.0(100独立)"]

  # 50 小写 — 按对话确认的顺序
  const lower_groups = [
    ("下点", @["ạ", "ẹ", "ị", "ọ", "ụ", "ä", "ë", "ï", "ö", "ü"]),
    ("倒短音", @["ȃ", "ȇ", "ȋ", "ȏ", "ȗ", "a̋", "e̋", "i̋", "ő", "ű"]),
    ("上圈反尾", @["å", "e̊", "i̊", "o̊", "ů", "ą", "ę", "į", "ǫ", "ų"]),
    ("波浪帽子", @["ã", "ẽ", "ĩ", "õ", "ũ", "â", "ê", "î", "ô", "û"]),
    ("短音反钝", @["ă", "ĕ", "ĭ", "ŏ", "ŭ", "ȁ", "ȅ", "ȉ", "ȍ", "ȕ"]),
  ]
  for (grp_name, syms) in lower_groups:
    for s in syms:
      result.symbols.add(SymbolEntry(ch: s, name: grp_name & "小写", group: grp_name))

  # 50 大写 — 严格对应小写
  const upper_groups = [
    ("下点", @["Ạ", "Ẹ", "Ị", "Ọ", "Ụ", "Ä", "Ë", "Ï", "Ö", "Ü"]),
    ("倒短音", @["Ȃ", "Ȇ", "Ȋ", "Ȏ", "Ȗ", "A̋", "E̋", "I̋", "Ő", "Ű"]),
    ("上圈反尾", @["Å", "E̊", "I̊", "O̊", "Ů", "Ą", "Ę", "Į", "Ǫ", "Ų"]),
    ("波浪帽子", @["Ã", "Ẽ", "Ĩ", "Õ", "Ũ", "Â", "Ê", "Î", "Ô", "Û"]),
    ("短音反钝", @["Ă", "Ĕ", "Ĭ", "Ŏ", "Ŭ", "Ȁ", "Ȅ", "Ȉ", "Ȍ", "Ȕ"]),
  ]
  for (grp_name, syms) in upper_groups:
    for s in syms:
      result.symbols.add(SymbolEntry(ch: s, name: grp_name & "大写", group: grp_name))

# ═══════════════════════════════════════════════════════════════
## 容器2: FQ-SM50P — 第2声母 (50进制, 含v/V)
## 来源: 原24声母 + v/V + ȥçşŋ扩展 + ȤÇŞŊ
## ═══════════════════════════════════════════════════════════════

proc makeFQ_SMP50P*(): CodeContainer =
  result.id = "FQ-SM50P"
  result.name = "满全法声母-50扩展"
  result.base = 50
  result.kind = "声母"
  result.version = "v2.0"
  result.parent = "FQ-SM48"
  result.history = @["v1.0(24声母)", "v1.5(48含大写)", "v2.0(50含v/V)"]

  # 25 小写声母
  const sm_lower = [
    ("b", "唇音"), ("p", "唇音"), ("m", "唇音"), ("f", "唇音"),
    ("d", "舌音"), ("t", "舌音"), ("n", "舌音"), ("l", "舌音"),
    ("g", "牙音"), ("k", "牙音"), ("h", "喉音"), ("w", "喉音"),
    ("j", "腭音"), ("q", "腭音"), ("x", "腭音"), ("y", "半舌"),
    ("z", "齿音"), ("c", "齿音"), ("s", "齿音"), ("r", "半齿"),
    ("ȥ", "齿音"), ("ç", "齿音"), ("ş", "齿音"), ("ŋ", "鼻音"),
    ("v", "唇齿音"),
  ]
  for (s, grp) in sm_lower:
    result.symbols.add(SymbolEntry(ch: s, name: "声母小写-" & s, group: grp))

  # 25 大写声母
  const sm_upper = [
    ("B", "唇音"), ("P", "唇音"), ("M", "唇音"), ("F", "唇音"),
    ("D", "舌音"), ("T", "舌音"), ("N", "舌音"), ("L", "舌音"),
    ("G", "牙音"), ("K", "牙音"), ("H", "喉音"), ("W", "喉音"),
    ("J", "腭音"), ("Q", "腭音"), ("X", "腭音"), ("Y", "半舌"),
    ("Z", "齿音"), ("C", "齿音"), ("S", "齿音"), ("R", "半齿"),
    ("Ȥ", "齿音"), ("Ç", "齿音"), ("Ş", "齿音"), ("Ŋ", "鼻音"),
    ("V", "唇齿音"),
  ]
  for (s, grp) in sm_upper:
    result.symbols.add(SymbolEntry(ch: s, name: "声母大写-" & s, group: grp))

# ═══════════════════════════════════════════════════════════════
## 从剩余数据提取: 复韵母、鼻韵母、特殊辅音
## ═══════════════════════════════════════════════════════════════

proc makeFQ_CMPD*(): CodeContainer =
  result.id = "FQ-CMPD"
  result.name = "满全法复韵母"
  result.base = 0  # 待计算
  result.kind = "复韵母"
  result.version = "v1.0"
  result.parent = "FQ-YM100"
  result.history = @["v1.0(从对话数据提取)"]

  # 从对话数据: 复韵母表
  const compound = [
    ("ai", "ꬰ/ꬱ/Ѧ/ѧ/Ɐ/ɐ/∀/Ꜽ/ꜽ/AI"),
    ("ei", "Ə/ə/Ӭ/ӭ"),
    ("ui", "Ѽ/ѽ/Ѱ/ѱ/Ѿ/ѿ"),
    ("ao", "Ꜵ/ꜵ/ɒ"),
    ("ou", "Ȣ/ȣ/႘"),
    ("iu", "Ⴞ/ꭎ/Ⴏ/Ⴎ"),
    ("ie", "ɤ/ɣ/Ꙋ/Ɣ/Ѥ/ѥ/ɬ/ꝷ"),
    ("üe", "ᵫ/ꭂ/Ё/ё/ʊ̈/ⱷ"),
    ("er", "ɛ/Ɛ/ꝵ/ꝶ/ɘ/Ǝ/ꞟ/Ꞟ/Ⰵ"),
    ("oi", "ꙕ/Ꙕ/Ѹ/ѹ"),
  ]
  for (base, variants) in compound:
    for v in variants.split("/"):
      result.symbols.add(SymbolEntry(ch: v, name: "复韵母-" & base, group: base))
  result.base = result.symbols.len

proc makeFQ_NASAL*(): CodeContainer =
  result.id = "FQ-NASAL"
  result.name = "满全法鼻韵母"
  result.base = 0
  result.kind = "鼻韵母"
  result.version = "v1.0"
  result.parent = "FQ-YM100"
  result.history = @["v1.0(从对话数据提取)"]

  # 从对话数据: 前鼻+后鼻
  const nasals = [
    ("an", "Ꙙ/ꙙ/Ã/ã"),
    ("en", "ⱄ/ⱅ/ⱺ/Ⰶ/Ẽ/ẽ/ꬴ/ꬲ"),
    ("in", "ꞃ/Ꞃ/ⲗ/Ⲗ/ꙉ/Ꙉ/ႁ/႔/Ĩ/ĩ/Ⰰ/ꭅ/ⴶ/ꬽ/ⴆ/ⴇ/ⴊ/ⴅ/ⴌ"),
    ("un", "տ/໙/ꦮ/ꦱ/ꦯ"),
    ("ün", "Ⴐ"),
    ("ian", "Ꙝ/ꙝ"),
    ("uan", "Ⴄ/Ⴂ/Ⴅ/ꦰ/ꦲ/ꭌ/ꭏ/ꭋ"),
    ("üan", "ѫ/Ѫ/ⴉ"),
    ("ang", "Ⴊ/Ⴆ/ᶏ/ᶐ"),
    ("eng", "Ӫ/ӫ/ᶒ/ⱺ"),
    ("ing", "Ꙟ/ꙟ/ꙗ/႞"),
    ("ong", "ꭀ/ꭁ/Ȏ/ȏ/Ⴓ/Ⴍ/Ǫ/ǫ/Ⴍ/Ⴓ"),
    ("iang", "ѭ/Ѭ/Ѩ/ѩ"),
    ("uang", "ꙛ/Ꙛ"),
    ("iong", "Ȱ/ȱ/Ѻ/ѻ/Ӻ"),
    ("ueng", "ɞ/ɝ"),
  ]
  for (base, variants) in nasals:
    for v in variants.split("/"):
      result.symbols.add(SymbolEntry(ch: v, name: "鼻韵母-" & base, group: base))
  result.base = result.symbols.len

# ═══════════════════════════════════════════════════════════════
## 容器: FQ-SPCON — 特殊辅音 (ⴀ-ⴿ + 独立辅音)
## ═══════════════════════════════════════════════════════════════

proc makeFQ_SPCON*(): CodeContainer =
  result.id = "FQ-SPCON"
  result.name = "满全法特殊辅音"
  result.base = 0
  result.kind = "辅音"
  result.version = "v1.0"
  result.parent = "FQ-SM50P"
  result.history = @["v1.0(ⴀ-ⴿ+独立辅音)"]

  # ⴀ-ⴿ 系列 (从对话提取)
  const spcon = [
    ("ⴀ", "yene"), ("ⴆ", "bor"), ("ⴋ", "dua"), ("ⴌ", "fhu"),
    ("ⴇ", "mana"), ("ⴶ", "noue"), ("ⴱ", "noue2"), ("ⴴ", "xvdbxv"),
    ("ⴸ", "xpqn"), ("ⴳ", "xpqn2"), ("ⴲ", "ojo"), ("ⴻ", "boloq"),
    ("ⴺ", "boloq2"), ("ⴹ", "boloq3"), ("ⴷ", "tkq-相位左上角"),
    ("ⴽ", "tkq-相位左上角2"), ("ⴼ", "xhkun-相位左下角"),
    ("ⴿ", "kvl-相位左下角2"), ("ⴾ", "xb"), ("ⴵ", "bx"),
    ("ⴔ", "qp"), ("ⴃ", "ot"), ("ⴒ", "E"),
    ("ⴎ", "lur"), ("ⴈ", "luhn"), ("ⴐ", "nabo"),
    ("ⴂ", "uang"), ("ⴓ", "li"), ("ⴕ", "mala"),
    ("ⴊ", "hin"), ("ⴙ", "hin2"), ("ⴗ", "hin3"), ("ⴖ", "hin4"),
    ("ⴁ", "luj"), ("ⴄ", "ni"), ("ⴍ", "toke"),
    ("ⴚ", "lonol"), ("ⴜ", "inm"), ("ⴛ", "dham"),
    ("ⴞ", "lul"), ("ⴝ", "nigjo"), ("ⴘ", "luj2"),
    ("ⴠ", "nrujn"),
    # 独立辅音
    ("h", "ɦ"), ("n", "ꝴ/ɲ/ɳ"), ("m", "ꝳ"),
    ("ⱷ", "uo"), ("ⱶ", "gkrl"), ("Ⱶ", "GKL"),
    ("Ⱳ", "uo2"), ("ⱳ", "w"), ("Ɀ", "Z"), ("Ȿ", "S"),
    ("ɬ", "ie"), ("ꝷ", "IE"), ("ꝵ", "ie2"),
    ("ʊ̈", "ue"), ("Ⰵ", "er"),
    ("ʌ", "ian"), ("Λ", "ian2"),
  ]
  for (s, nm) in spcon:
    result.symbols.add(SymbolEntry(ch: s, name: nm, group: "特殊辅音"))
  result.base = result.symbols.len

# ═══════════════════════════════════════════════════════════════
## 容器: FQ-CLONE — 分身容器 (长得一模一样的符号)
## ═══════════════════════════════════════════════════════════════

proc makeFQ_CLONE*(): CodeContainer =
  result.id = "FQ-CLONE"
  result.name = "满全法分身符号"
  result.base = 0
  result.kind = "分身"
  result.version = "v1.0"
  result.parent = "FQ-YM100/FQ-SM50P"
  result.history = @["v1.0(重复符号独立化)"]

  # 长得一模一样的符号 → 放在独立分身上
  const clones = [
    ("Ⴐ", "ün小写=ün大写(同形)"),
    ("႘", "ou大写(出现在不同容器)"),
    ("Ⴍ", "ong大写(出现在不同容器)"),
    ("Ⴓ", "ong大写(出现在不同容器)"),
    ("â", "帽子(小写,出现在波浪帽子组)"),
    ("ê", "帽子(小写,出现在波浪帽子组)"),
    ("î", "帽子(小写,出现在波浪帽子组)"),
    ("ô", "帽子(小写,出现在波浪帽子组)"),
    ("û", "帽子(小写,出现在波浪帽子组)"),
  ]
  for (s, nm) in clones:
    result.symbols.add(SymbolEntry(ch: s, name: nm, group: "分身"))
  result.base = result.symbols.len

# ═══════════════════════════════════════════════════════════════
## 容器注册表
## ═══════════════════════════════════════════════════════════════

proc allContainers*(): seq[CodeContainer] =
  result = @[
    makeFQ_YM100(),
    makeFQ_SMP50P(),
    makeFQ_CMPD(),
    makeFQ_NASAL(),
    makeFQ_SPCON(),
    makeFQ_CLONE(),
  ]

# ═══════════════════════════════════════════════════════════════
## 容器关联关系
## ═══════════════════════════════════════════════════════════════

type
  ContainerRelation* = object
    a_id*, b_id*: string
    relation_type*: string
    via*: string
    strength*: float
    version_a*, version_b*: string

proc computeRelations*(): seq[ContainerRelation] =
  # FQ-YM100 ↔ FQ-YM66
  result.add(ContainerRelation(
    a_id: "FQ-YM100", b_id: "Clock-60", relation_type: "韵母继承",
    via: "YM100基于YM66扩展", strength: 0.8,
    version_a: "v2.0", version_b: "v1.0",
  ))
  # FQ-YM100 ↔ FQ-YM42
  result.add(ContainerRelation(
    a_id: "FQ-YM100", b_id: "Color-42", relation_type: "颜色对应",
    via: "下点组(ạ ä)≈红色系", strength: 0.6,
    version_a: "v2.0", version_b: "v1.0",
  ))
  # FQ-SM50P ↔ FQ-SM48
  result.add(ContainerRelation(
    a_id: "FQ-SM50P", b_id: "FQ-SM48", relation_type: "声母进化",
    via: "50P=48+2(v/V)", strength: 0.9,
    version_a: "v2.0", version_b: "v1.0",
  ))
  # FQ-CMPD ↔ FQ-YM100
  result.add(ContainerRelation(
    a_id: "FQ-CMPD", b_id: "FQ-YM100", relation_type: "组合引用",
    via: "复韵母=yunmu100中符号组合", strength: 0.7,
    version_a: "v1.0", version_b: "v2.0",
  ))
  # FQ-NASAL ↔ FQ-YM100
  result.add(ContainerRelation(
    a_id: "FQ-NASAL", b_id: "FQ-YM100", relation_type: "鼻音扩展",
    via: "鼻韵母=yunmu100中n/ng结尾", strength: 0.7,
    version_a: "v1.0", version_b: "v2.0",
  ))
  # FQ-SPCON ↔ FQ-SM50P
  result.add(ContainerRelation(
    a_id: "FQ-SPCON", b_id: "FQ-SM50P", relation_type: "辅音分支",
    via: "ⴀ-ⴿ扩展了SM50P", strength: 0.5,
    version_a: "v1.0", version_b: "v2.0",
  ))
  # FQ-CLONE → 所有容器
  result.add(ContainerRelation(
    a_id: "FQ-CLONE", b_id: "FQ-YM100", relation_type: "同源分身",
    via: "同形符号独立化", strength: 0.3,
    version_a: "v1.0", version_b: "v2.0",
  ))

# ═══════════════════════════════════════════════════════════════
## 统一转字符串
## ═══════════════════════════════════════════════════════════════

proc `$`*(e: SymbolEntry): string =
  result = e.ch & "(" & e.name & ")"

proc `$`*(c: CodeContainer): string =
  result = c.id & "(base=" & $c.base & ", " & c.kind & ", v=" & c.version & ")"

proc `$`*(r: ContainerRelation): string =
  result = r.a_id & " ↔ " & r.b_id & " [" & r.relation_type & "] via=" & r.via & " s=" & $r.strength

# ═══════════════════════════════════════════════════════════════
## 报告生成
## ═══════════════════════════════════════════════════════════════

proc containerReport*(containers: seq[CodeContainer], relations: seq[ContainerRelation]): string =
  var lines: seq[string] = @[]
  lines.add("═══════════════════════════════════════════════════")
  lines.add("  满全法容器系统 · 第2韵母/第2声母/剩余/分身")
  lines.add("═══════════════════════════════════════════════════")
  lines.add("")

  lines.add("── 容器清单 ──")
  for c in containers:
    lines.add("  " & $c)
    lines.add("    父容器: " & c.parent)
    lines.add("    版本历史: " & c.history.join(" → "))
    lines.add("    符号数: " & $c.symbols.len)

  lines.add("")
  lines.add("── 第2韵母 (FQ-YM100) 符号表 ──")
  for c in containers:
    if c.id == "FQ-YM100":
      var line: string
      for i, s in c.symbols:
        if i > 0 and i mod 10 == 0:
          lines.add("  " & line)
          line = ""
        line &= s.ch & "  "
      if line.len > 0: lines.add("  " & line)

  lines.add("")
  lines.add("── 第2声母 (FQ-SM50P) 符号表 ──")
  for c in containers:
    if c.id == "FQ-SM50P":
      var line: string
      for i, s in c.symbols:
        if i > 0 and i mod 10 == 0:
          lines.add("  " & line)
          line = ""
        line &= s.ch & "  "
      if line.len > 0: lines.add("  " & line)

  lines.add("")
  lines.add("── 关联关系 ──")
  for r in relations:
    lines.add("  " & $r)

  lines.add("")
  lines.add("── 版本号选择表 ──")
  lines.add("  当前版本 | 历史版本 | 说明")
  for c in containers:
    let hist = if c.history.len > 1: c.history[0..^2].join("→") else: "无"
    lines.add("  " & c.version & " | " & hist & " | " & c.id)

  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
## ═══════════════════════════════════════════════════════════════

when isMainModule:
  let containers = allContainers()
  let relations = computeRelations()
  echo containerReport(containers, relations)