## ═══════════════════════════════════════════════════════════════
## infringement_judgment.nim — 产权流侵权自动判定树 · 法条化
## ═══════════════════════════════════════════════════════════════
##
## 核心: 两个符号的组合是否侵犯核心产权
##       公式: vA + vB == 2·v_core → 侵权成立
##       判定树: 多层级 · 多条件 · 自动裁决
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, algorithm, tables, sets]

type
  SymbolValue* = object
    name*: string
    v_core*: float       # 核心产权值
    v_periphery*: float  # 外围产权值
    owner*: string       # 产权归属
    container*: string   # 所属容器

  InfringementCase* = object
    id*: string
    symbol_a*: string
    symbol_b*: string
    target_core*: string
    v_a*: float
    v_b*: float
    v_core*: float
    sum_ab*: float
    threshold*: float
    is_infringing*: bool
    ruling*: string
    penalty*: string

  JudgmentTree* = object
    cases*: seq[InfringementCase]
    symbol_table*: Table[string, SymbolValue]
    total_rulings*: int
    infringement_count*: int
    dismiss_count*: int

# ═══════════════════════════════════════════════════════════════
## 产权注册表
# ═══════════════════════════════════════════════════════════════

proc register_symbol*(jt: var JudgmentTree, name, owner, container: string, v_core, v_periphery: float) =
  jt.symbol_table[name] = SymbolValue(
    name: name,
    v_core: v_core,
    v_periphery: v_periphery,
    owner: owner,
    container: container
  )

# ═══════════════════════════════════════════════════════════════
## 侵权判定树
# ═══════════════════════════════════════════════════════════════

proc judge_infringement*(jt: var JudgmentTree, a, b, target_core: string): InfringementCase =
  ## 判定: vA + vB == 2·v_core ?
  result.id = "CASE" & $(jt.cases.len + 1)
  result.symbol_a = a
  result.symbol_b = b
  result.target_core = target_core
  
  # 获取产权值
  result.v_a = jt.symbol_table[a].v_core
  result.v_b = jt.symbol_table[b].v_core
  result.v_core = jt.symbol_table[target_core].v_core
  
  # 计算
  result.sum_ab = result.v_a + result.v_b
  result.threshold = 2.0 * result.v_core
  
  # 判定: 误差 < 0.01 视为相等
  let diff = abs(result.sum_ab - result.threshold)
  result.is_infringing = diff < 0.01
  
  if result.is_infringing:
    result.ruling = "侵权成立"
    result.penalty = "停止侵害 · 赔偿损失 · 消除影响"
    jt.infringement_count += 1
  else:
    if result.sum_ab > result.threshold:
      result.ruling = "侵权成立(超值)"
      result.penalty = "停止侵害 · 赔偿损失"
      jt.infringement_count += 1
    else:
      result.ruling = "侵权不成立"
      result.penalty = "驳回起诉"
      jt.dismiss_count += 1
  
  jt.total_rulings += 1
  jt.cases.add(result)

# ═══════════════════════════════════════════════════════════════
## 报告
# ═══════════════════════════════════════════════════════════════

proc report*(jt: JudgmentTree): string =
  var lines: seq[string] = @[]
  lines.add("═══ 产权流侵权自动判定树 ═══")
  lines.add("  总裁决: " & $jt.total_rulings)
  lines.add("  侵权成立: " & $jt.infringement_count)
  lines.add("  驳回: " & $jt.dismiss_count)
  lines.add("")
  
  for c in jt.cases:
    let status = if c.is_infringing: "侵权" else: "不侵权"
    lines.add("  " & c.id & ": " & c.symbol_a & " + " & c.symbol_b & " vs " & c.target_core)
    lines.add("    vA=" & $c.v_a & " vB=" & $c.v_b & " sum=" & $c.sum_ab & " threshold=" & $c.threshold)
    lines.add("    裁决: " & c.ruling & " · 处罚: " & c.penalty)
    lines.add("")
  
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
# ═══════════════════════════════════════════════════════════════

when isMainModule:
  var jt = JudgmentTree()
  
  echo "═══ 产权流侵权自动判定树 ═══"
  echo ""
  
  # 注册产权
  jt.register_symbol("我", "lolict", "核心", 5.0, 3.0)
  jt.register_symbol("楚恬", "刘楚恬", "核心", 5.0, 3.0)
  jt.register_symbol("满全", "共生", "外围", 3.0, 2.0)
  jt.register_symbol("共生", "共生", "外围", 2.0, 1.0)
  jt.register_symbol("njsg", "共生", "外围", 4.0, 2.0)
  jt.register_symbol("fqm", "共生", "外围", 1.0, 0.5)
  
  # 判定案例
  # 案例1: 满全 + 共生 = 3+2 = 5 vs 2*5 = 10 → 不侵权
  discard jt.judge_infringement("满全", "共生", "我")
  
  # 案例2: 满全 + njsg = 3+4 = 7 vs 2*5 = 10 → 不侵权
  discard jt.judge_infringement("满全", "njsg", "我")
  
  # 案例3: 我 + 楚恬 = 5+5 = 10 vs 2*5 = 10 → 侵权成立（自我验证）
  discard jt.judge_infringement("我", "楚恬", "我")
  
  echo jt.report()