## ═══════════════════════════════════════════════════════════════
## yinyang_binary.nim — YIN-YANG-BINARY 映射器
## ═══════════════════════════════════════════════════════════════
##
## 核心: 拆出两个作夫妻关系 · 以加法统摄剩余存在
##       摆脱幂次(蜜次方)二进制扩张
##
## 私有锚点定义:
##   我(lolict) = 1  → 主动 · 阳 · 显现
##   刘楚恬 = 0      → 从动 · 阴 · 隐藏
##
## 规则:
##   - 阴阳不是对立 · 是共生
##   - 0和1不在同一个维度 · 不能比较大小
##   - 加法 = 统摄剩余存在的方式
##   - 摆脱"幂次"= 不让2的幂次统治一切
##   - 根基隐藏 = 保护(被看见的只是加法结果 · 不是根基)
##
## ═══════════════════════════════════════════════════════════════

import std/[tables, sequtils, strutils, strformat]

type
  YinYang* = enum
    yyYin    # 0 · 刘楚恬 · 隐藏 · 从动
    yyYang   # 1 · lolict · 显现 · 主动

  Anchor* = object
    name*: string
    value*: YinYang
    visible*: bool       # 是否对外可见
    description*: string

  SumResult* = object
    operands*: seq[YinYang]
    sum_value*: int      # 加法结果(可见)
    hidden*: bool        # 根基是否隐藏

  YinYangEngine* = object
    anchors*: Table[string, Anchor]
    sum_history*: seq[SumResult]
    private_mode*: bool  # 隐藏根基模式

# ═══════════════════════════════════════════════════════════════
## 初始化引擎 · 注册私有锚点
# ═══════════════════════════════════════════════════════════════

proc new_yinyang_engine*(): YinYangEngine =
  result.anchors = init_table[string, Anchor]()
  result.sum_history = @[]
  result.private_mode = true  # 默认隐藏根基
  
  # 注册私有锚点
  result.anchors["lolict"] = Anchor(
    name: "lolict",
    value: yyYang,
    visible: true,
    description: "我=1 · 主动 · 显现"
  )
  result.anchors["刘楚恬"] = Anchor(
    name: "刘楚恬",
    value: yyYin,
    visible: true,
    description: "刘楚恬=0 · 从动 · 隐藏"
  )

# ═══════════════════════════════════════════════════════════════
## 以加法统摄剩余存在
# ═══════════════════════════════════════════════════════════════

proc add*(e: var YinYangEngine, parts: seq[YinYang]): SumResult =
  var s = 0
  for p in parts:
    if p == yyYang: s += 1
  
  result = SumResult(
    operands: parts,
    sum_value: s,
    hidden: e.private_mode
  )
  e.sum_history.add(result)

# ═══════════════════════════════════════════════════════════════
## 摆脱幂次: 不用2^n · 用加法链
# ═══════════════════════════════════════════════════════════════

proc addition_chain*(e: YinYangEngine, n: int): seq[int] =
  ## 加法链: 每一步都是前面某两步之和
  ## 不依赖2的幂次 · 自由度更高
  if n <= 0: return @[]
  if n == 1: return @[1]
  
  var chain = @[1]
  var target = 2
  while target <= n:
    # 选择能生成target的加法组合(不一定是翻倍)
    var best = target - 1  # 默认 1+(target-1)
    for i in 0 ..< chain.len:
      for j in i ..< chain.len:
        if chain[i] + chain[j] == target:
          best = chain[i] + chain[j]
          break
    chain.add(best)
    target += 1
  
  result = chain

# ═══════════════════════════════════════════════════════════════
## 阴阳解码 · 从加法结果反推可能的组合
# ═══════════════════════════════════════════════════════════════

proc decode_sum*(sum_val: int, total: int): seq[YinYang] =

  result = @[]
  var remaining = sum_val
  for i in 0 ..< total:
    if remaining > 0:
      result.add(yyYang)
      remaining -= 1
    else:
      result.add(yyYin)

# ═══════════════════════════════════════════════════════════════
## 验证: 加法结果是否匹配(不知道具体谁是谁)
# ═══════════════════════════════════════════════════════════════

proc verify_sum*(parts: seq[YinYang], expected_sum: int): bool =
  var s = 0
  for p in parts:
    if p == yyYang: s += 1
  result = (s == expected_sum)

# ═══════════════════════════════════════════════════════════════
## 共生编码: 用夫妻关系统一编码多个容器
# ═══════════════════════════════════════════════════════════════

proc encode_symbiosis*(e: YinYangEngine, container_values: Table[string, int]): string =
  ## 输入: 各容器的值
  ## 输出: 加法总和(根基隐藏)
  var total = 0
  for name, val in container_values:
    total += val
  
  if e.private_mode:
    result = fmt"[总和={total}] (根基隐藏)"
  else:
    var detail: seq[string] = @[]
    for name, val in container_values:
      detail.add(fmt"{name}={val}")
    result = detail.join(" + ") & fmt" = {total}"

# ═══════════════════════════════════════════════════════════════
## 报告
# ═══════════════════════════════════════════════════════════════

proc report*(e: YinYangEngine): string =
  var lines: seq[string] = @[]
  lines.add("═══ YIN-YANG-BINARY 映射器 ═══")
  lines.add("  私有锚点: 我(lolict)=1 · 刘楚恬=0")
  lines.add("  根基模式: " & (if e.private_mode: "隐藏" else: "可见"))
  lines.add("  加法运算次数: " & $e.sum_history.len)
  lines.add("")
  
  lines.add("── 锚点定义 ──")
  for name, anchor in e.anchors:
    let v = if anchor.value == yyYang: "1" else: "0"
    lines.add("  " & name & " = " & v & "  (" & anchor.description & ")")
  
  lines.add("")
  lines.add("── 加法链 (摆脱幂次) ──")
  let chain = e.addition_chain(8)
  for i, v in chain:
    lines.add("  第" & $(i+1) & "步: " & $v)
  
  lines.add("")
  lines.add("── 共生编码示例 ──")
  var test_values = init_table[string, int]()
  test_values["声母"] = 5
  test_values["韵母"] = 6
  test_values["声调"] = 4
  test_values["笔画"] = 3
  let encoded = e.encode_symbiosis(test_values)
  lines.add("  输入: 声母=5 韵母=6 声调=4 笔画=3")
  lines.add("  编码: " & encoded)
  lines.add("")
  lines.add("  解读: 看见总和=18 · 不知道各容器具体值 · 根基被隐藏")

  lines.add("")
  lines.add("── 验证: 只知道总和也能验证 ──")
  let test_parts = @[yyYang, yyYin, yyYang, yyYin, yyYang]  # 1+0+1+0+1=3
  let ok = verify_sum(test_parts, 3)
  lines.add("  输入: 1,0,1,0,1 | 期望总和=3 | 验证=" & (if ok: "✓" else: "✗"))
  
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
# ═══════════════════════════════════════════════════════════════

when isMainModule:
  var e = new_yinyang_engine()
  
  echo "═══ YIN-YANG-BINARY 验证 ═══"
  echo ""
  
  # 加法运算
  discard e.add(@[yyYang, yyYin, yyYang])        # 1+0+1=2
  discard e.add(@[yyYin, yyYin, yyYin])          # 0+0+0=0
  discard e.add(@[yyYang, yyYang, yyYang, yyYang]) # 1+1+1+1=4
  discard e.add(@[yyYang, yyYin, yyYin, yyYang, yyYin]) # 1+0+0+1+0=2
  
  echo e.report()
  
  echo ""
  echo "── 核心验证 ──"
  echo "  阴阳共生: 我(1) + 刘楚恬(0) = 不是对立 · 是加法统摄"
  echo "  根基隐藏: 外人只见加法结果 · 不知 0/1 分别是谁"
  echo "  摆脱幂次: 不用 2^n · 用加法链自由选择组合"
  echo "  保护逻辑: 隐藏 0/1 的定义 = 保护夫妻关系的根基"
