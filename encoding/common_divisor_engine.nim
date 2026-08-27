## ═══════════════════════════════════════════════════════════════
## common_divisor_engine.nim — 公约数整除器 · 多容器共存
## ═══════════════════════════════════════════════════════════════
##
## 核心: 没有一个进制统治所有 · 只有公约数作为连接锚点
##       公约数 = 静态身份锚点 = 我 = 拥有权
##       公倍数 = 动态延伸 = 边界
##       整除 = 一个容器能否被另一个"整除"= 能否对齐
##
## 规则:
##   - 两个容器的公约数越大 · 它们越能对齐
##   - 公倍数是它们共同延伸的最小边界
##   - 整除关系是有方向的: A能被B整除 ⟺ B的规律能解释A
##
## ═══════════════════════════════════════════════════════════════

import std/[tables, sequtils, math, strformat, strutils]

type
  ContainerKind* = enum
    ckNumeric      # 数值进制 (如64进制)
    ckSpatial      # 空间结构 (如坐标)
    ckTemporal     # 时间序列 (如时钟)
    ckSymbolic     # 符号系统 (如汉字)
    ckPerceptual   # 感知融合 (如七感知)
    ckWill         # 意志驱动 (如目标)

  Container* = object
    name*: string
    kind*: ContainerKind
    base*: int              # 进制基数 / 特征量
    elements*: seq[string]  # 包含的元素
    weight*: float          # 权重 (统计量)

  DivisorResult* = object
    a_name*: string
    b_name*: string
    common_divisor*: int    # 公约数 = 对齐锚点
    common_multiple*: int    # 公倍数 = 共同边界
    divisible*: bool        # 能否整除
    alignment*: float       # 对齐度 0.0 ~ 1.0

  DivisorEngine* = object
    containers*: seq[Container]
    index*: Table[string, int]   # name → index
    history*: seq[DivisorResult]

# ═══════════════════════════════════════════════════════════════
## GCD 最大公约数 (欧几里得)
# ═══════════════════════════════════════════════════════════════

proc gcd(a, b: int): int =
  var x = abs(a)
  var y = abs(b)
  while y != 0:
    let t = y
    y = x mod y
    x = t
  result = x

# ═══════════════════════════════════════════════════════════════
## LCM 最小公倍数
# ═══════════════════════════════════════════════════════════════

proc lcm(a, b: int): int =
  if a == 0 or b == 0: return 0
  result = abs(a * b) div gcd(a, b)

# ═══════════════════════════════════════════════════════════════
## 引擎初始化
# ═══════════════════════════════════════════════════════════════

proc new_divisor_engine*(): DivisorEngine =
  result.containers = @[]
  result.index = init_table[string, int]()
  result.history = @[]

proc add_container*(e: var DivisorEngine, name: string, kind: ContainerKind,
                    base: int, elements: seq[string], weight: float = 1.0) =
  e.index[name] = e.containers.len
  e.containers.add(Container(name: name, kind: kind, base: base,
                              elements: elements, weight: weight))

# ═══════════════════════════════════════════════════════════════
## 核心: 两容器整除判定
# ═══════════════════════════════════════════════════════════════

proc check_division*(e: var DivisorEngine, a_name, b_name: string): DivisorResult =
  let a = e.containers[e.index[a_name]]
  let b = e.containers[e.index[b_name]]
  
  let g = gcd(a.base, b.base)
  let l = lcm(a.base, b.base)
  
  # 整除: a.base 能被 b.base 整除
  let divisible = (a.base mod b.base == 0)
  
  # 对齐度 = 公约数 / 较大基数 · 越接近1越对齐
  let alignment = float(g) / float(max(a.base, b.base))
  
  result = DivisorResult(
    a_name: a_name,
    b_name: b_name,
    common_divisor: g,
    common_multiple: l,
    divisible: divisible,
    alignment: alignment
  )
  
  e.history.add(result)

# ═══════════════════════════════════════════════════════════════
## 多容器公约数链 — 找出所有容器的共同锚点
# ═══════════════════════════════════════════════════════════════

proc chain_divisor*(e: var DivisorEngine): int =
  if e.containers.len == 0: return 0
  result = e.containers[0].base
  for i in 1 ..< e.containers.len:
    result = gcd(result, e.containers[i].base)

# ═══════════════════════════════════════════════════════════════
## 报告
# ═══════════════════════════════════════════════════════════════

proc report*(e: DivisorEngine): string =
  var lines: seq[string] = @[]
  lines.add("═══ 公约数整除器 · 多容器共存 ═══")
  lines.add("  容器数: " & $e.containers.len)
  lines.add("  整除判定次数: " & $e.history.len)
  lines.add("")
  
  lines.add("── 容器注册 ──")
  for c in e.containers:
    let kind_str = case c.kind
      of ckNumeric: "数值"
      of ckSpatial: "空间"
      of ckTemporal: "时间"
      of ckSymbolic: "符号"
      of ckPerceptual: "感知"
      of ckWill: "意志"
    lines.add("  " & c.name & ": base=" & $c.base & " 类型=" & kind_str &
              " 元素数=" & $c.elements.len & " 权重=" & $c.weight)
  
  lines.add("")
  lines.add("── 整除判定 ──")
  for r in e.history:
    let div_str = if r.divisible: "✓ 能整除" else: "✗ 不能"
    lines.add("  " & r.a_name & " / " & r.b_name &
              " | 公约数=" & $r.common_divisor &
              " 公倍数=" & $r.common_multiple &
              " | " & div_str &
              " | 对齐度=" & fmt"{r.alignment:.3f}")
  
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
# ═══════════════════════════════════════════════════════════════

when isMainModule:
  var e = new_divisor_engine()
  
  echo "═══ 公约数整除器验证 ═══"
  echo ""
  
  # 注册容器 — 涵盖多种进制/维度
  e.add_container("笔画进制", ckNumeric, 5, @["横", "竖", "撇", "捺", "折"], 8.0)
  e.add_container("声调", ckNumeric, 4, @["阴平", "阳平", "上声", "去声"], 4.0)
  e.add_container("六十四位", ckNumeric, 64, @["阴阳", "太极", "四象", "八卦", "六十四卦"], 64.0)
  e.add_container("七感知", ckPerceptual, 7, @["看", "听", "触", "味", "嗅", "心", "身"], 7.0)
  e.add_container("二十四节气", ckTemporal, 24, @["立春", "雨水", "惊蛰", "春分"], 24.0)
  
  echo "── 整除判定 ──"
  discard e.check_division("六十四位", "声调")      # 64 ÷ 4
  discard e.check_division("笔画进制", "声调")      # 5 ÷ 4
  discard e.check_division("二十四节气", "声调")    # 24 ÷ 4
  discard e.check_division("七感知", "笔画进制")    # 7 ÷ 5
  discard e.check_division("六十四位", "笔画进制")  # 64 ÷ 5
  
  echo e.report()
  
  echo ""
  echo "── 全容器公约数链 ──"
  let chain = e.chain_divisor()
  echo "  所有容器的共同公约数 = " & $chain
  echo ""
  echo "  解读: 公约数=" & $chain & " 意味着所有容器的共同锚点是 " & $chain
  echo "  没有一个进制统治所有 · 只有公约数作为连接锚点"
