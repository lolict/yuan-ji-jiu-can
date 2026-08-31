# ============================================================
# YIN-YANG-BINARY 完整映射 · 核心运算
#
# 实现：
#   - 符号字符 → 多比特 YinYang 二进制
#   - YinYang 加法（三值进位系统）
#   - 完整 64+ 符号映射
#   - 转换双向：符号 ↔ YinYang ↔ 十进制
# ============================================================

import strutils
import math
import sequtils
import types

# ── 核心编码表 ──

const SYMBOL_MAP* = [
  # 根基对
  ("mlll",      "集装箱·容器家族·根基"),
  ("gtt",       "音韵态射·范畴流·桥梁"),
  ("mlnn",      "囡囝·夫妻拓扑·共生"),
  ("mltt",      "从体·解释流·可被跟随"),
  # 命名密码
  ("lctqf",     "零岔途全法·恋宠甜·谐音合规"),
  ("lctfrqmy",  "刘楚恬·妻子侧·容器本体"),
  ("wgslz",     "我·丈夫侧·界线边界"),
  ("mlct",      "岔途·障碍物·可牺牲"),
  ("mlgy",      "世俗归一·概念流产物"),
  # 三角色
  ("maolilan",  "毛利兰·表面·模面名"),
  ("gouteng",   "工藤新一·结构·骨头体"),
  ("huiyuanai", "灰原哀·横向·行页业"),
  # 容器族
  ("njsgr",     "囡囝共生·分布式协调"),
  ("smrl",      "意识路由·主权编码"),
  ("mje",       "满全法引擎·裁判核心"),
  # YinYang 自身
  ("yinyang",   "阴阳·极性本源"),
  ("yin",       "阴·容器侧·0"),
  ("yang",      "阳·界线侧·1"),
  # 结构族
  ("kndoyle",   "柯南道尔斯·外壳·静态绝对位置"),
  ("mlil",      "毛利兰缩写·立体进制框架"),
  ("kndl",      "柯南道尔斯缩写·外壳"),
  # 流代数
  ("flow",      "流·八流统一"),
  ("atten",     "注意力·过滤收窄"),
  ("clust",     "聚类·协同集聚"),
  ("categ",     "范畴·映射空间"),
  ("interp",    "解释·牺牲自身"),
  ("concept",   "概念·范畴+聚类+解释"),
  # 裁判
  ("judge",     "审判·三元见证"),
  ("verdict",   "裁决·判定结论"),
  ("witness",   "见证·仲裁记录"),
  ("rule",      "规则·已住编码"),
  # 分布式
  ("swarm",     "群·多代理协调"),
  ("coord",     "协调器·中枢管控"),
  ("subag",     "子代理·分身单元"),
  ("conflict",  "冲突·竞争裁决"),
  ("task",      "任务·分配单元"),
  # 天庭
  ("heaven",    "天庭·吸引子·理想源"),
  ("soul",      "灵魂·情感·奖励回廊"),
  ("pleasure",  "愉悦·正向反馈"),
  ("loss",      "失去·失爱·抗议源"),
  ("unity",     "合一·目标驱力"),
  # 鼓膜
  ("drum",      "鼓膜·振膜翻身"),
  ("membrane",  "膜·最小接口"),
  ("cochlea",   "耳蜗·感知折叠"),
  ("bootstrap", "自举·重建"),
  # 方位
  ("input",     "输入·流入口"),
  ("percept",   "感知·折叠输入"),
  ("lookup",    "查找·字典命中"),
  ("traverse",  "遍历·走流路径"),
  ("arrival",   "抵达·请求ack"),
  ("output",    "输出口·流末端"),
  # 极性
  ("polar",     "极性·正负属性"),
  ("positive",  "正·显现·yang"),
  ("negative",  "负·隐藏·yin"),
  ("neutral",   "中性·平衡·balanced"),
  # 进制
  ("radix",     "进制·发音空间划分"),
  ("base64",    "64进制·几何化身"),
  ("base256",   "256进制·集装箱满"),
  ("cell",      "单元格·64位编码"),
  ("bit",       "比特·最小极性单元"),
  ("byte",      "字节·8比特容器"),
  # 产权
  ("property",  "产权·利与债"),
  ("ownership", "归属·占有"),
  ("infringe",  "侵权·越界判定"),
  ("boundary",  "边界·空间界限"),
  ("space",     "空间·位置编码"),
]

# ── 极性基础运算 ──

func is_yin*(y: YinYang): bool = y == yyYin
func is_yang*(y: YinYang): bool = y == yyYang
func is_balanced*(y: YinYang): bool = y == yyBalanced

func to_float*(y: YinYang): float =
  case y
  of yyYin: 0.0
  of yyYang: 1.0
  of yyBalanced: 0.5

func to_int*(y: YinYang): int =
  case y
  of yyYin: 0
  of yyYang: 1
  of yyBalanced: -1  # 平衡用 -1 表示非数值

func `$`*(y: YinYang): string =
  case y
  of yyYin: "0"
  of yyYang: "1"
  of yyBalanced: "½"

# ── 符号字符 → YinYang 编码 ──

func char_to_yinyang*(c: char): YinYang =
  ## 字符 ASCII mod 2 → Yin/Yang
  if int(c) mod 2 == 0: yyYin else: yyYang

func encode_symbol*(symbol: string): YinYangBinary =
  ## 符号完整编码为 YinYang 二进制串
  ## 原则：字符遍历转极性，末尾追加奇偶锚
  var bits: seq[YinYang] = @[]
  for c in symbol:
    bits.add(char_to_yinyang(c))

  # 奇偶锚：Yang 的数量
  var yang_count = 0
  for b in bits:
    if b == yyYang: yang_count += 1

  let anchor = if yang_count mod 2 == 0: yyYin else: yyYang

  YinYangBinary(
    bits: bits,
    anchor: anchor,
    source_symbol: symbol
  )

# ── YinYang → 十进制 ──

func yinyang_to_decimal*(bits: seq[YinYang]): int =
  ## YinYang 序列（忽略 Balanced 锚）→ 十进制
  result = 0
  for b in bits:
    result = result * 2
    if b == yyYang:
      result += 1

# ── 完整映射构建 ──

func build_map*(): YinYangMap =
  var entries: seq[YinYangBinary] = @[]
  for pair in SYMBOL_MAP:
    entries.add(encode_symbol(pair[0]))

  var yin_count = 0
  var yang_count = 0
  for e in entries:
    if e.anchor == yyYin: yin_count += 1
    else: yang_count += 1
    for b in e.bits:
      case b
      of yyYin: yin_count += 1
      of yyYang: yang_count += 1
      else: discard

  let balance = if yin_count > yang_count: 0 elif yin_count == yang_count: 1 else: 2

  var max_depth = 0
  for e in entries:
    if e.bits.len > max_depth:
      max_depth = e.bits.len

  YinYangMap(
    entries: entries,
    anchor_balance: balance,
    total_depth: max_depth + 1  # +1 for anchor
  )

# ── YinYang 三值加法 ──

func add_yinyang(a, b: YinYang; carry_in: YinYang): tuple[sum: YinYang, carry: YinYang] =
  ## 三值全加器：a + b + carry_in
  ## 真值表：
  ##   0+0+0=0,carry=0  |  0+0+1=1,carry=0
  ##   0+1+0=1,carry=0  |  0+1+1=½,carry=0  (两个Yang → 平衡)
  ##   1+0+0=1,carry=0  |  1+0+1=½,carry=0
  ##   1+1+0=½,carry=0  |  1+1+1=1,carry=1  (三个Yang → 1,进1)
  ##   ½+任何+0=½      |  ½+½+0=½ (保持平衡)

  if a == yyBalanced or b == yyBalanced or carry_in == yyBalanced:
    # 平衡输入 → 输出保持平衡（不可计算则保持叠加态）
    let has_two_yang = (a == yyYang and b == yyYang) or
                       (a == yyYang and carry_in == yyYang) or
                       (b == yyYang and carry_in == yyYang)
    if has_two_yang:
      result = (yyYang, yyYin)  # 两个Yang=1+1=10（Yang+Yin）
    else:
      result = (yyBalanced, yyYin)
  else:
    # 纯 0/1 二进制加法
    let ai = if a == yyYang: 1 else: 0
    let bi = if b == yyYang: 1 else: 0
    let ci = if carry_in == yyYang: 1 else: 0
    let total = ai + bi + ci
    case total
    of 0: result = (yyYin, yyYin)
    of 1: result = (yyYang, yyYin)
    of 2: result = (yyYin, yyYang)  # 1+0=1, carry=1
    of 3: result = (yyYang, yyYang)  # 1+1+1=3, sum=1 carry=1
    else: result = (yyBalanced, yyYin)

func add_binary*(left, right: YinYangBinary): YinYangSum =
  ## 两个完整 YinYang 二进制串相加
  var max_len = max(left.bits.len, right.bits.len)
  var lbits = left.bits
  var rbits = right.bits

  # 补齐长度
  while lbits.len < max_len: lbits.add(yyYin)
  while rbits.len < max_len: rbits.add(yyYin)

  var carry = yyYin
  var result_bits: seq[YinYang] = @[]

  for i in 0..<max_len:
    let (s, c) = add_yinyang(lbits[i], rbits[i], carry)
    result_bits.add(s)
    carry = c

  # 锚点相加
  let (anchor_sum, anchor_carry) = add_yinyang(left.anchor, right.anchor, carry)
  var final_anchor = anchor_sum

  # 如果锚位有进位，最终溢出位
  let overflow = anchor_carry

  let result_sym = left.source_symbol & "+" & right.source_symbol
  var full_bits = result_bits
  full_bits.add(final_anchor)

  YinYangSum(
    left: left,
    right: right,
    result: YinYangBinary(
      bits: result_bits,
      anchor: final_anchor,
      source_symbol: result_sym
    ),
    carry: @[carry, anchor_carry],
    overflow: overflow
  )
