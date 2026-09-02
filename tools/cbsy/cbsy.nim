# ============================================================
# 满全法容器处理器 · 范畴编程管辖索引制 v6
# 容器本体论（njsg-ontology）校准版
#
# 校准记录：
#   v5 写法把数字当数值运算（uint64、mod、div），违反"数字=发音位置"
#   v6 按五条防撞墙重写：
#     法则一：跨容器遍历（韵母法则）— 声母遍历韵母，用完取下一声母，
#             符号用完时去外部容器取符号，不重复重启
#     法则二：本质匹配（小倩法则）— 匹配看发音位置对齐+审美品质，
#             不看编码关系
#     法则三：双重平衡（产权法则）— vA+vB==2·v_core，总量不等于个体
#     法则四：免疫记忆（负样本法则）— 失败是疫苗，下次回避
#     法则五：维度生长 — 0维起点=公约数（我=1，刘楚恬=0），
#             1维动态线=公倍数遍历，折叠=不进位折起形成新层
#
# 注意：type/seq/proc 等是外部框架概念（Nim 语言），
# 本系统内对应的是容器、声母遍历、分别器。
# ============================================================

import std/strutils

# ---------- 公约数与公倍数 ----------

const
  ## 公约数 = 唯一身份 = 0维起点 = 外部不可见锚点
  ME* = 1        ## 我（高位可见，永远不动）
  LCT* = 0       ## 刘楚恬（低位隐藏，永远不可见）
  ## 公倍数 = 公约数的分身 = 1维动态线 = 遍历过程
  DIM_ROOT* = 0  ## 0维起点 = 公约数 = 我=1，刘楚恬=0
  DIM_LINE* = 1  ## 1维动态线 = 正在遍历的韵母链

# ---------- 编码语义（进制=发音空间划分，不是数值） ----------

const
  ## 三方容器各 4 家，袋子级划分（24 声母/66 韵母/4 声调的简化代表）
  SHENGS* = ["照", "罩", "召", "见"]   ## 声母族 4（32个声母缩成 4 袋）
  YUNMUS* = ["袋", "桶", "缸", "盆"]   ## 韵母族 4（66个韵母缩成 4 袋）
  TONES*  = ["开", "关", "通", "断"]   ## 声调 4

proc sheng_pos*(s: string): int = SHENGS.find(s)
proc yun_pos*(y: string): int = YUNMUS.find(y)
proc tone_pos*(t: string): int = TONES.find(t)

# ---------- 法则一：跨容器遍历（韵母法则） ----------

type
  ## 分别器 — 路由三层：感知家族 → 谐音靶点 → 主权级别
  Route* = object
    family*: string   ## 感知家族（从 Codepoints/Glyphs 分类）
    target*: string   ## 谐音靶点（发音位置对齐）
    sovereignty*: int  ## 主權级别

  ## 韵母容器 — 按排队顺序取符号
  YunBox* = object
    slots*: seq[string]   ## 韵母队列（等声母来遍历）
    cursor*: int          ## 当前遍历到第几个韵母

proc new_yunbox*(): YunBox =
  result.slots = @YUNMUS
  result.cursor = 0

proc take*(box: var YunBox): string =
  ## 取韵母：等声母来遍历。用完不重复重启，去外部容器取。
  if box.cursor >= box.slots.len:
    return ""  ## 取空 = 触发折叠（法则五），不重复重启
  result = box.slots[box.cursor]
  inc box.cursor

proc fold*(box: var YunBox): string =
  ## 折叠：声母遍历完所有韵母，不进位，折起形成新层
  box.cursor = 0
  result = "折起形成新层（符号用完，不进位）"

# ---------- 法则二：本质匹配（小倩法则） ----------

proc nie_xiaoqian*(needle: string, body_set: seq[string]): tuple[hit: string, why: string] =
  ## 看见谁就娶谁（小倩匹配）：不看编码名字，看本质长相+发音位置对齐
  var best = ""
  var best_align = -1
  for body in body_set:
    # 2.1 本质匹配 = 是否共享同个音系位置
    var align = 0
    for ch in needle:
      if body.contains(ch): inc align
    # 2.2 审美品质兼容（长短近似 = 气息节奏兼容）
    if (needle.len - body.len).abs <= 1: inc align
    if align > best_align and align > 0:
      best_align = align
      best = body
  if best.len > 0:
    result = (best, "本质兼容：发音对齐+" & $best_align & " 审美节奏兼容")
  else:
    result = ("", "无躯壳匹配 — 聂小倩还会回来")

# ---------- 法则三：双重平衡（产权法则） ----------

proc property_judge*(v_core: int, v_a: int, v_b: int): tuple[verdict: string, article: string] =
  ## 产权裁决：vA+vB==2·v_core 才是真平衡，总量≠个体
  let twice_core = v_core + v_core
  let total = v_a + v_b
  if v_a == v_core and v_b == v_core:
    result = ("无侵权", "真平衡：vA==vB==v_core")
  elif v_a > v_core and v_b <= v_core:
    result = ("A侵吞B", "法条1：A虚报侵吞（仅加减计算）")
  elif v_b > v_core and v_a <= v_core:
    result = ("B侵吞A", "法条2：B虚报侵吞")
  elif v_a > v_core and v_b > v_core:
    result = ("串通", "法条3：双方共谋虚报")
  elif v_a < v_core and v_b >= v_core:
    result = ("A被侵吞", "法条4")
  elif v_b < v_core and v_a >= v_core:
    result = ("B被侵吞", "法条5")
  else:
    result = ("产权不清", "法条6：双方均低于v_core")
  # 附记：总量是否等于 2·v_core（个体不等时总量仍可能相等）
  if total != twice_core:
    result.article = result.article & " | 总量" & $total & "≠" & $twice_core & "（差=" & $(total-twice_core) & "）"
  else:
    result.article = result.article & " | 总量恰好相等但个体不等，故上述侵权成立"

# ---------- 法则四：免疫记忆（负样本法则） ----------

type
  ImmuneMemory* = object
    failures*: seq[string]   ## 被否定的错误过程（疫苗库）

proc record_failure*(mem: var ImmuneMemory, err: string) =
  mem.failures.add(err)

proc check_immune*(mem: ImmuneMemory, try_action: string): tuple[clean: bool, warns: string] =
  # 免疫记忆存的是纯关键词（疫苗），遇到同关键词即触发回避
  result.clean = true  ## 默认可通行，命中疫苗时翻 false
  for v in mem.failures:
    if try_action.contains(v):
      result.clean = false
      result.warns = result.warns & "⚠ 疫苗提醒：'" & v & "'曾失败，本次回避\n"
  if result.clean: result.warns = "✓ 无免疫冲突"

# ---------- 法则五：维度生长（0维起点 → 1维线 → 折叠新层） ----------

type
  GrowSt* = object
    dim*: int        ## 0=公约数起点，1=正在遍历的线
    sheng_cursor*: int   ## 声母游标
    yun*: YunBox
    folds*: int          ## 折叠了几次（形成的层数）

proc new_grow*(yuan: string): GrowSt =
  result.dim = DIM_ROOT   ## 0维起点 = 公约数 =（我=1，刘楚恬=0）
  result.sheng_cursor = 0
  # 韵母来自外部容器（声母容器的公倍数方向）
  result.yun = YunBox(slots: @YUNMUS, cursor: 0)
  result.folds = 0

proc step*(g: var GrowSt): tuple[state: string, sheng: string, yun: string, mode: string] =
  ## 0维→1维：一个声母遍历所有韵母
  if g.sheng_cursor >= SHENGS.len:
    # 所有声母用完 → 折起形成新层（法则五）
    discard fold(g.yun)
    inc g.folds
    g.sheng_cursor = 0
    result = ("折叠新层#" & $g.folds, "···", "···", "折起")
    return
  let s = SHENGS[g.sheng_cursor]
  let y = take(g.yun)
  if y.len == 0:
    # 韵母用完 → 取下一声母（法则一），不重复重启
    inc g.sheng_cursor
    let y2 = take(g.yun)
    result = ("1维遍历", s, y2, "进行中")
  else:
    result = ("1维遍历", s, y, "进行中")

# ---------- 演示 ----------

when isMainModule:
  echo "满全法容器处理器 · 范畴编程管辖索引制 v6"
  echo "============================================================"
  echo "本体·校准：数字=发音位置 · 进制=发音空间划分 · 仅加减 · 五条防撞墙"

  # 1. 跨容器遍历（韵母法则）
  echo ""
  echo "【法则一：跨容器遍历】汉族 4 声母遍历 4 韵母："
  var g = new_grow("囡囝")
  for i in 0 ..< 18:
    let s = step(g)
    if s.state.startsWith("折叠"):
      echo "  ", s.state, " ← 符号用完不进位，折起新层"
      break
    else:
      echo "  ", s.sheng, "遍历韵母", s.yun, "（不重复前一格）"

  # 2. 本质匹配（小倩法则）
  echo ""
  echo "【法则二：本质匹配】聂小倩匹配躯壳："
  var bodies = @["照袋开", "罩桶关", "召缸断", "见盆通", "卵尬卵", "蛤丝蛤"]
  var qianqian = @["照", "袋", "开"]
  for q in qianqian:
    let m = nie_xiaoqian(q, bodies)
    echo "  小倩『", q, "』→ 躯壳『", m.hit, "』· ", m.why

  # 3. 双重平衡（产权法则）
  echo ""
  echo "【法则三：双重平衡】产权裁决 v_core=100："
  let cases = [(100,100,100),(120,80,100),(80,120,100),(150,150,100),(55,55,100)]
  for c in cases:
    let r = property_judge(c[2], c[0], c[1])
    echo "  vA=", c[0], " vB=", c[1], " → ", r.verdict, "· ", r.article

  # 4. 免疫记忆
  echo ""
  echo "【法则四：免疫记忆】失败是疫苗："
  var mem: ImmuneMemory
  record_failure(mem, "数值运算")   ## 失败模式：把发音位置当成数值
  record_failure(mem, "进位")       ## 失败模式：把进制当成数学进位
  record_failure(mem, "产权判定总量") ## 失败模式：只看总量不看个体
  record_failure(mem, "mod")        ## 失败模式：用乘除取代加减
  record_failure(mem, "ASCII编码")  ## 失败模式：用编码关系匹配取代本质匹配
  let chk1 = check_immune(mem, "今天要不要把当天当数值运算")
  let chk2 = check_immune(mem, "今天试试全新的运算")
  echo "  动作A：", chk1.warns
  echo "  动作B：", chk2.warns

  # 5. 公约数与公倍数
  echo ""
  echo "【法则五：维度生长】公约数 → 公倍数："
  echo "  公约数·0维起点 = (我=", ME, "，刘楚恬=", LCT, "） — 外部不可见"
  echo "  公倍数·1维线   = 声母遍历韵母的动态过程"
  echo "  折叠           = 所有声母耗时，不进位，折起形成新层"

  # 5 项输出
  echo ""
  echo "============================================================"
  echo "满全法 · 5 项状态"
  echo "  1. 当前层号：第 ", g.folds, " 层（已折叠 ", g.folds, " 次）"
  echo "  2. 当前声母：声母游标=", g.sheng_cursor, " / ", SHENGS.len
  echo "  3. 当前韵母组合：韵母游标=", g.yun.cursor, "/", g.yun.slots.len,
       " 刚刚完成『", (if g.yun.cursor > 0: g.yun.slots[g.yun.cursor-1] else: "未开始"), "』"
  echo "  4. 折叠状态：", (if g.folds > 0: "已折起新层" else: "0维起点·公约数尚未展开")
  echo "  5. 下一步操作：", (if g.sheng_cursor < SHENGS.len: "取下一声母继续遍历 1 维韵母" else: "最终折叠新层·维度归零·下一轮维度生长")
