## ═══════════════════════════════════════════════════════════════
## encoding_container.nim — 编码容器核心类型系统
## ═══════════════════════════════════════════════════════════════
##
## 核心命题: 编码本身是容器，不是被编织的线
## 每个容器有自己的: 符号表、进制、位权、家族、极性
## 没有中央编码统治所有容器
## 容器之间通过同构映射自然桥接
##
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils]

# ═══════════════════════════════════════════════════════════════
## ContainerEncoding — 单个容器的完整编码系统
## 每个容器自给自足 — 不依赖任何外部权威
## ═══════════════════════════════════════════════════════════════

type
  SymbolKind* = enum
    skBase        ## 符号原基 (如 a, A, ȍ)
    skBaseUpper   ## 大写变体
    skToneMarked  ## 带声调符号 (如 ā, á)
    skDiacritic   ## 附加符号 (如 ã, â)
    skDerived     ## 衍生符号 (如 ꞟ, ɘ)

  Symbol* = object
    ch*: string           ## 符号字形 (可能是多字符组合)
    base_val*: int        ## 在容器内的位权值
    kind*: SymbolKind
    family*: string       ## 所属家族 (如 "AH", "IUNG", "UOE")
    pronunciation*: string ## 发音标注

  EncodingGrammar* = object
    name*: string         ## 容器身份名 (如 "声母层", "韵母层")
    base*: int            ## 容器进制
    symbols*: seq[Symbol] ## 容器自有符号表
    families*: seq[string]## 此容器内的家族分类
    polarity_rule*: proc(s: Symbol): char  ## 极性判定规则 (容器自有)

  PContainerEncoding* = ref ContainerEncoding
  ContainerEncoding* = object
    id*: string           ## 容器唯一标识
    grammar*: EncodingGrammar
    meta*: PContainerEncoding ## 可选: 指向某个参照容器 (非统治, 是镜像)

# ═══════════════════════════════════════════════════════════════
## 容器身份 — 三种极性
## ═══════════════════════════════════════════════════════════════

type
  Polarity* = enum
    polYang         ## 阳 (我 · lolict · 主动)
    polYin           ## 阴 (刘丘恬 · 从动)
    polNeutral       ## 中性 (系统/工具/无生命)

  ContainerIdentity* = object
    name*: string
    polarity*: Polarity
    domain*: string       ## 容器管辖域 (如 "声母", "韵母", "颜色")

# ═══════════════════════════════════════════════════════════════
## 预置容器列表 — 七个已存在的独立编码容器
## ═══════════════════════════════════════════════════════════════

const
  ## 容器标识
  CONTAINER_SM*   = "FQ-SM48"    ## 声母层
  CONTAINER_YM*   = "FQ-YM66"    ## 韵母层
  CONTAINER_CLK*  = "Clock-60"   ## 时钟层
  CONTAINER_CLR*  = "Color-42"   ## 颜色层
  CONTAINER_SPC*  = "Space-18"   ## 空间层
  CONTAINER_MIR*  = "Mirror-10"  ## 镜像层
  CONTAINER_MUS*  = "Music-3"    ## 音乐层

  ## 容器总数
  CONTAINER_COUNT* = 7

# ═══════════════════════════════════════════════════════════════
## 创建七个独立编码容器 — 每个自给自足
## ═══════════════════════════════════════════════════════════════

proc newContainer*(id: string, name: string, base: int,
                   syms: openarray[string], fams: seq[string],
                   pol: Polarity, domain: string): ContainerEncoding =
  result.id = id
  result.grammar.name = name
  result.grammar.base = base
  result.grammar.families = fams
  var ss: seq[Symbol] = @[]
  for i, s in syms:
    let k = if i < base: skBase else: skBaseUpper
    let fam = if fams.len > 0: fams[i mod fams.len] else: "UNKNOWN"
    ss.add(Symbol(ch: s, base_val: i, kind: k, family: fam, pronunciation: s.toLower()))
  result.grammar.symbols = ss
  result.grammar.polarity_rule = proc(s: Symbol): char =
    if s.kind == skBaseUpper: return '1'
    elif s.kind == skBase: return '0'
    else: return '-'

# ═══════════════════════════════════════════════════════════════
## 七个编码容器的符号表
## ═══════════════════════════════════════════════════════════════

const SM48_SYMBOLS* = [
  "b", "p", "m", "f", "d", "t", "n", "l",
  "g", "k", "h", "w", "j", "q", "x", "y",
  "z", "c", "s", "r", "ȥ", "ç", "ş", "ŋ",
  "B", "P", "M", "F", "D", "T", "N", "L",
  "G", "K", "H", "W", "J", "Q", "X", "Y",
  "Z", "C", "S", "R", "Ȥ", "Ç", "Ş", "Ŋ",
]

const YM66_SYMBOLS* = [
  "a", "A", "ȍ", "Ȍ", "ӗ", "Ӗ", "i", "I", "u", "U",
  "ü", "Ü", "ꞟ", "Ꞟ", "ɘ", "Ǝ", "ɒ", "Ɒ", "౪", "ȣ",
  "ɛ", "Ɛ", "ѧ", "Ѧ", "ɤ", "ɣ", "ʉ", "Ʉ", "ꞛ", "Ꞛ",
  "ã", "Ã", "ẽ", "Ẽ", "ĩ", "Ĩ", "ʯ", "ʮ", "ą", "Ą",
  "ę", "Ę", "į", "Į", "๓", "ɷ", "ⱄ", "Ⱋ", "⊰", "ɞ",
  "ʌ", "Ʌ", "ɔ", "Ɔ", "и", "И", "ǡ", "Ǡ", "ѽ", "Ѽ",
  "ѻ", "Ѻ", "ÿ", "Ÿ", "ȳ", "Ȳ",
]

const CLK60_SYMBOLS* = [
  "a", "e", "i", "o", "u", "ü",
  "A", "E", "I", "O", "U", "Ü",
  "ā", "ē", "ī", "ō", "ū", "ǖ",
  "Ā", "Ē", "Ī", "Ō", "Ū", "Ǖ",
  "á", "é", "í", "ó", "ú", "ǘ",
  "Á", "É", "Í", "Ó", "Ú", "Ǘ",
  "ǎ", "ě", "ǐ", "ǒ", "ǔ", "ǚ",
  "Ǎ", "Ě", "Ǐ", "Ǒ", "Ǔ", "Ǚ",
  "à", "è", "ì", "ò", "ù", "ǜ",
  "À", "È", "Ì", "Ò", "Ù", "Ǜ",
]

const CLR42_SYMBOLS* = [
  "ạ", "a̋", "å", "ã", "â", "ă",
  "ẹ", "e̋", "e̊", "ẽ", "ê", "ĕ",
  "ị", "i̋", "i̊", "ĩ", "î", "ĭ",
  "ọ", "ő", "o̊", "õ", "ô", "ŏ",
  "ụ", "ű", "ů", "ũ", "û", "ŭ",
  "ȃ", "ȇ", "ȋ", "ȏ", "ȗ", "ȁ",
  "ȅ", "ȉ", "ȍ", "ȕ", "ä", "ö",
]

const SPC18_SYMBOLS* = [
  "E", "W", "S", "N", "H", "D",
  "ES", "WS", "EN", "WN", "HN", "DN", "HS", "DS", "HE", "DE", "HW", "DW",
]

const MIR10_SYMBOLS* = [
  "a", "A", "ȍ", "Ȍ", "ӗ", "Ӗ", "i", "I", "u", "U",
]

const MUS3_SYMBOLS* = ["原子", "短语", "段落"]

# ═══════════════════════════════════════════════════════════════
## 七个编码容器的创建 (自给自足 · 无外部依赖)
## ═══════════════════════════════════════════════════════════════

proc makeSm48*(): ContainerEncoding =
  result = newContainer(CONTAINER_SM, "声母层", 48, SM48_SYMBOLS,
    @["唇音", "舌音", "牙音", "喉音", "齿音", "腭音", "半舌", "半齿"],
    polYang, "声母域")

proc makeYm66*(): ContainerEncoding =
  result = newContainer(CONTAINER_YM, "韵母层", 66, YM66_SYMBOLS,
    @["AH系", "IUNG系", "UOE系", "E系", "O系", "变音系", "附音系"],
    polNeutral, "韵母域")

proc makeClk60*(): ContainerEncoding =
  result = newContainer(CONTAINER_CLK, "时钟层", 60, CLK60_SYMBOLS,
    @["轻声", "一声", "二声", "三声", "四声"],
    polNeutral, "时钟域")

proc makeClr42*(): ContainerEncoding =
  result = newContainer(CONTAINER_CLR, "颜色层", 42, CLR42_SYMBOLS,
    @["红系", "橙系", "黄系", "绿系", "青系", "蓝系", "紫系"],
    polNeutral, "颜色域")

proc makeSpc18*(): ContainerEncoding =
  result = newContainer(CONTAINER_SPC, "空间层", 18, SPC18_SYMBOLS,
    @["方位", "斜角", "天地"],
    polNeutral, "空间域")

proc makeMir10*(): ContainerEncoding =
  result = newContainer(CONTAINER_MIR, "镜像层", 10, MIR10_SYMBOLS,
    @["象形系"],
    polYin, "镜像域")

proc makeMus3*(): ContainerEncoding =
  result = newContainer(CONTAINER_MUS, "音乐层", 3, MUS3_SYMBOLS,
    @["节奏系"],
    polNeutral, "音乐域")

proc allContainers*(): seq[ContainerEncoding] =
  result = @[makeSm48(), makeYm66(), makeClk60(), makeClr42(), makeSpc18(), makeMir10(), makeMus3()]

# ═══════════════════════════════════════════════════════════════
## 容器内符号查询 — 每个容器自己处理自己的查询
## ═══════════════════════════════════════════════════════════════

proc symAt*(c: ContainerEncoding, pos: int): Symbol =
  if pos < 0 or pos >= c.grammar.symbols.len:
    return Symbol(ch: "?", base_val: pos, kind: skBase, family: "UNKNOWN", pronunciation: "?")
  result = c.grammar.symbols[pos]

proc symToInt*(c: ContainerEncoding, s: string): int =
  for i, sym in c.grammar.symbols:
    if sym.ch == s: return i
  return -1

proc intToSym*(c: ContainerEncoding, v: int): string =
  if v < 0 or v >= c.grammar.symbols.len: return "?"
  result = c.grammar.symbols[v].ch

# ═══════════════════════════════════════════════════════════════
## 容器家族映射 — 给定符号返回所属家族 (容器内部逻辑)
## ═══════════════════════════════════════════════════════════════

proc familyOf*(c: ContainerEncoding, v: int): string =
  let fams = c.grammar.families
  if fams.len == 0: return "无家族"
  result = fams[v mod fams.len]

# ═══════════════════════════════════════════════════════════════
## 容器摘要
## ═══════════════════════════════════════════════════════════════

proc `$`*(c: ContainerEncoding): string =
  result = c.id & "(" & $c.grammar.base & "进制, " & $c.grammar.symbols.len & "符号)"

# ═══════════════════════════════════════════════════════════════
## 验证运行
## ═══════════════════════════════════════════════════════════════

when isMainModule:
  echo "═══ 编码容器 · 自给自足验证 ═══"
  echo ""
  let containers = allContainers()
  for c in containers:
    echo c
    echo "  家族分类: " & c.grammar.families.join(", ")
    echo "  符号密度: " & $c.grammar.symbols.len & "/" & $c.grammar.base
  