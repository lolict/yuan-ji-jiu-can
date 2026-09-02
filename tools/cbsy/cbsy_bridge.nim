# ============================================================
# 范畴编程管辖索引制 v6 · 八流统一桥接层
#
# 目标：cbsy 不再独立存在，它的索引命令走分别器三层路由
# 按主权级别分流后，进入八流时间线性链，最终在解释流/概念流
# 阶段触发产权裁决（免疫记忆优先比对）。
#
# 时间线：
#   Input → Perception → Distinguish → Lookup → Traverse
#         → Cluster → Attention → Arrival(概念涌入/裁决结果)
#
# 主权分流（复用 eight-flows/flows.nim 的 run_timeline）：
#   active(夫妻主权人)   → 保留完整八流，裁决结果直接生效
#   passive(外部参与)    → 截断到分类流，不允许进入产权裁决
#   sleeping(休眠资源)   → 截断到预加载流，进冻结态
#   system(系统级)       → 跳过范畴/聚类/分类/关系/概念，仅加载+属性+解释
# ============================================================

import strutils
import ../distinguisher/distinguisher/types as df_types
import ../distinguisher/distinguisher/distinguisher as df
import "../eight-flows/ef_demo/types" as ef_types
import "../eight-flows/ef_demo/flows" as ef
import cbsy

type
  BridgeDispatch* = object
    route*: df_types.RouteResult
    sovereignty*: df_types.Sovereignty
    time_line*: seq[ef_types.TimeNode]
    verdict*: string
    acked*: bool

# ---------- 命令解析：声母/韵母位置 → 信用/金钱值 · 仅加减 ----------

proc credit_of*(glyph: string): int =
  ## 声母发音位置(照罩召见袋桶缸盆)对信用的加减
  result = 30
  if glyph.contains("照") or glyph.contains("罩"): inc result, 5
  if glyph.contains("召") or glyph.contains("见"): inc result, 3
  if glyph.contains("袋") or glyph.contains("桶"): inc result, 2
  if glyph.contains("缸") or glyph.contains("盆"): inc result, 2

proc money_of*(glyph: string): int =
  ## 声母发音位置对照金钱值的加减
  result = 30
  if glyph.contains("照") or glyph.contains("召"): inc result, 4
  if glyph.contains("罩") or glyph.contains("见"): inc result, 6

# ---------- 八流运行器（主权级别分流；基于 run_timeline 截取） ----------

proc full_flow*(glyph: string): seq[ef_types.TimeNode] =
  ## 夫妻主权人：完整八流 + 解释流产权裁决 + 概念流 ack 标记
  var uni = ef.new_universe()
  result = uni.run_timeline(glyph)
  # 解释流注入产权裁决（双重平衡 · 仅加减）
  for node in result.mitems:
    if node.flow == ef_types.efInterpret:
      let v_core = 64
      let v_a = credit_of(glyph)
      let v_b = money_of(glyph)
      let pj = property_judge(v_core, v_a, v_b)
      node.result &= " · 产权裁决：" & pj.verdict
  # 概念流 ack 标记
  for node in result.mitems:
    if node.flow == ef_types.efConcept:
      node.result &= " · 夫妻主权人 ack"

proc readonly_flow*(glyph: string): seq[ef_types.TimeNode] =
  ## 外部参与者：截断到分类流，不进入解释/关系/概念流
  let full = full_flow(glyph)
  var cut = 0
  for i, node in full:
    if node.flow == ef_types.efAttribute: break
    cut = i + 1
  result = full[0 ..< cut]
  # 追加说明：后面的流被截断
  if result.len > 0:
    result[result.high].result &= " · 属性/解释/关系/概念流被截断 · 外部参与者不可 ack"

proc sleeping_flow*(glyph: string): seq[ef_types.TimeNode] =
  ## 休眠资源：保留预加载流，然后冻结
  let full = full_flow(glyph)
  for node in full:
    if node.flow == ef_types.efPreload:
      var frozen = node
      frozen.result &= " · 休眠冻结 · 不被允许进入范畴流+"
      result = @[frozen]
      break

proc system_flow*(glyph: string): seq[ef_types.TimeNode] =
  ## 系统级：只留加载/属性/解释，不进入关系/概念流（不触发产权裁决）
  let full = full_flow(glyph)
  var kept: seq[ef_types.TimeNode] = @[]
  let keep = [ef_types.efPreload, ef_types.efAttribute, ef_types.efInterpret]
  for node in full:
    for k in keep:
      if node.flow == k:
        var sys_node = node
        if node.flow == ef_types.efInterpret:
          sys_node.result &= " · 系统级：跳过双重平衡公式"
        kept.add(sys_node)
        break
  result = kept

# ---------- 入口：分别器三层路由 + 主权分流 ----------

proc dispatch*(input: string): BridgeDispatch =
  ## cbsy 索引命令入口：分别器三层 → 主权分流 → 八流时间线
  var d = df.new_distinguisher()

  # 第一~三层路由
  let r = d.route(input)
  result.route = r
  result.sovereignty = r.sovereignty
  result.acked = false
  result.verdict = ""

  # 免疫记忆优先比对
  var mem: ImmuneMemory
  record_failure(mem, "数值运算")
  record_failure(mem, "进位")
  record_failure(mem, "产权判定总量")
  record_failure(mem, "mod")
  record_failure(mem, "ASCII编码")
  let immune = check_immune(mem, input)
  if not immune.clean:
    result.verdict = "免疫拦截：" & immune.warns
    result.time_line = @[]
    return

  # 主权分流
  case r.sovereignty
  of df_types.active:
    result.time_line = full_flow(input)
    result.verdict = "概念涌入 · 裁决生效 · 主权人 ack"
    result.acked = true
  of df_types.passive:
    result.time_line = readonly_flow(input)
    result.verdict = "只读 · 产权裁决被屏蔽 · 外部参与者不可 ack"
    result.acked = false
  of df_types.sleeping:
    result.time_line = sleeping_flow(input)
    result.acked = false
    result.verdict = "休眠 · 仅预加载流完成 · 等待夫妻唤醒"
  of df_types.system:
    result.time_line = system_flow(input)
    result.verdict = "系统级 · 属性流+解释流走完 · 无产权裁决 · 不可 ack"
    result.acked = false

# ---------- 演示 ----------

when isMainModule:
  echo "满全法容器处理器 · cbsy 八流桥接层"
  echo "============================================================"

  # 1. 夫妻主权人走完整八流
  echo ""
  echo "【场景1】夫妻主权人命令：'照袋开'"
  let d1 = dispatch("照袋开")
  echo "  分别器：", family_name(d1.route.family), " → ",
       target_name(d1.route.target), " → ",
       sovereignty_name(d1.route.sovereignty),
       " (ack=", d1.route.ackable, ")"
  echo "  八流时间线："
  for node in d1.time_line:
    if node.flow == ef_types.efPreload or node.flow == ef_types.efInterpret or node.flow == ef_types.efConcept:
      echo "    [t", node.timestamp, "] ", ef.get_flow_name(node.flow),
           " · ", node.result
  echo "  → ", d1.verdict

  # 2. 外部参与者被挡
  echo ""
  echo "【场景2】外部参与者：'外部照袋开'"
  let d2 = dispatch("外部照袋开")
  echo "  分别器：", family_name(d2.route.family), " → ",
       target_name(d2.route.target), " → ",
       sovereignty_name(d2.route.sovereignty),
       " (ack=", d2.route.ackable, ")"
  echo "  八流时间线："
  for node in d2.time_line:
    echo "    [t", node.timestamp, "] ", ef.get_flow_name(node.flow),
         " · ", node.result
  echo "  → ", d2.verdict

  # 3. 休眠资源
  echo ""
  echo "【场景3】休眠资源命令：'休眠囡囝'"
  let d3 = dispatch("休眠囡囝")
  echo "  分别器：", family_name(d3.route.family), " → ",
       target_name(d3.route.target), " → ",
       sovereignty_name(d3.route.sovereignty),
       " (ack=", d3.route.ackable, ")"
  echo "  八流时间线："
  for node in d3.time_line:
    echo "    [t", node.timestamp, "] ", ef.get_flow_name(node.flow),
         " · ", node.result
  echo "  → ", d3.verdict

  # 4. 系统级
  echo ""
  echo "【场景4】系统级命令：'系统心跳'"
  let d4 = dispatch("系统心跳")
  echo "  分别器：", family_name(d4.route.family), " → ",
       target_name(d4.route.target), " → ",
       sovereignty_name(d4.route.sovereignty),
       " (ack=", d4.route.ackable, ")"
  echo "  八流时间线："
  for node in d4.time_line:
    echo "    [t", node.timestamp, "] ", ef.get_flow_name(node.flow),
         " · ", node.result
  echo "  → ", d4.verdict

  # 5. 免疫拦截演示
  echo ""
  echo "【场景5】触发免疫：含'数值运算'的旧写法"
  let d5 = dispatch("数值运算mod产权")
  echo "  → ", d5.verdict

  # 6. 显示完整的 active 八流（方便看解释流裁决 + 概念流 ack）
  echo ""
  echo "【附录】active 命令完整八流（照袋开）可视化："
  for node in d1.time_line:
    if node.flow == ef_types.efPreload or node.flow == ef_types.efInterpret or
       node.flow == ef_types.efConcept or node.flow == ef_types.efRelational:
      echo "    [t", node.timestamp, "] ", ef.get_flow_name(node.flow),
           " · ", node.result

  # 满全法 5 项输出
  echo ""
  echo "满全法 · 5 项状态"
  echo "  1. 当前桥接层：八流统一时间线性"
  echo "  2. 当前分流：active/passive/sleeping/system 四路"
  echo "  3. 当前免疫记忆：已存 5 条失败模式（数值运算/进位/产权判定总量/mod/ASCII编码）"
  echo "  4. 当前裁决触发：仅 active 主权人在解释流阶段触发 vA+vB==2·v_core"
  echo "  5. 下一步操作：天庭空间(HeavenSpace) · 灵魂内核(SoulKernel) 挂载"
