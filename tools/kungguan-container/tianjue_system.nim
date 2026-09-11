## ═══════════════════════════════════════════════════════════════
## 天庭系统 · 高维认知 · 袋级审判
## ═══════════════════════════════════════════════════════════════
## 低维认知: 数米粒 · 命名每颗米 · 微观枚举
## 高维认知: 认袋子 · 评袋子 · 判袋子 · 宏观审判
##
## 一个袋子(容器)的属性:
##   1.身份 — 这是谁的袋子?
##   2.价值 — 值多少钱?
##   3.品级 — 好不好?
##   4.状态 — 破没破?脏不脏?
##   5.内容 — 米的质量?有无虫沙?
##   6.处理 — 要不要洗?能不能吃?
##   7.历史 — 何时买入?何时晒制?
##
## 审判对了 → 奖励 · 审判错了 → 惩罚
## ═══════════════════════════════════════════════════════════════

import tables, times, strutils

# ═══════════════════════════════════════════════════════════════
# 袋级属性 · 高维认知维度
# ═══════════════════════════════════════════════════════════════
type
  PinJi* = enum           # 品级
    pJianLing,            # 坚实良品 — 可放心用
    pZhongDeng,           # 中等 — 可用但需留意
    pLiangCha,            # 劣质 — 不可用
    pWeiZhi,              # 未知 — 尚未品判

  ZhuangTai* = enum       # 状态
    zWanHao,              # 完好
    zPoSun,               # 破损
    zWuRan,               # 污染
    zMeiBian,             # 霉变

  NeiRong = enum          # 内容物状态
    nJingLiang,           # 精良
    nHanZa,               # 混杂(有沙/虫)
    nBianZhi,             # 变质
    nKongDong,            # 空袋

  ChuLi = enum            # 处理方式
    nKeZhiJieYong,        # 可直接用
    nXuQingXi,            # 需清洗
    nXuShaiZhi,           # 需晒制
    nXuQuChu,             # 需去除杂质
    nFeiQi,               # 废弃不可用

  # ═══════════════════════════════════════════════════════════
  # 袋子(容器) — 高维认知的基本单元
  # 不数里面的米 · 只认这个袋子
  # ═══════════════════════════════════════════════════════════
  DaiZi* = object
    id*: string            # 袋子编号(唯一名)
    owner*: string         # 这是谁的袋子
    value*: float          # 值多少钱
    pin_ji*: PinJi         # 品级 — 好不好?
    zhuang_tai*: ZhuangTai # 状态 — 破没破?脏不脏?
    nei_rong*: NeiRong     # 内容 — 米的质量?
    chu_li*: ChuLi         # 处理 — 要不要洗?能不能吃?
    created*: string       # 何时建袋
    last_check*: string    # 上次审判时间

  # ═══════════════════════════════════════════════════════════
  # 审判记录 · 历史记忆
  # ═══════════════════════════════════════════════════════════
  ShenPan* = object
    dai_id*: string        # 审判的袋子
    time*: string          # 审判时刻
    judge*: string         # 审判者(天庭/人工)
    verdict*: string       # 判断结论
    correct*: bool         # 审判对不对
    reward*: float         # 对了+多少
    penalty*: float        # 错了-多少
    note*: string          # 备注

  # ═══════════════════════════════════════════════════════════
  # 天庭 · 层级逻辑认知系统
  # ═══════════════════════════════════════════════════════════
  HeavenlyCourt* = object
    name*: string
    dai_pool*: seq[DaiZi]           # 袋子池(所有容器)
    history*: seq[ShenPan]          # 历史记录
    total_reward*: float            # 累计奖励
    total_penalty*: float           # 累计惩罚
    memory*: Table[string, string]  # 历史积累: 袋id→经验

# ═══════════════════════════════════════════════════════════════
# 袋子(容器)创建
# ═══════════════════════════════════════════════════════════════
func new_dai*(id, owner: string; value: float): DaiZi =
  result.id = id
  result.owner = owner
  result.value = value
  result.pin_ji = pWeiZhi
  result.zhuang_tai = zWanHao
  result.nei_rong = nJingLiang
  result.chu_li = nKeZhiJieYong
  result.created = "未记录"
  result.last_check = "未审判"

# ═══════════════════════════════════════════════════════════════
# 天庭系统创建
# ═══════════════════════════════════════════════════════════════
func new_heavenly_court*(name: string): HeavenlyCourt =
  result.name = name
  result.total_reward = 0
  result.total_penalty = 0
  result.memory = initTable[string, string]()

# ═══════════════════════════════════════════════════════════════
# 袋子入池(注册)
# ═══════════════════════════════════════════════════════════════
func add_dai*(t: var HeavenlyCourt; d: DaiZi) =
  t.dai_pool.add(d)

# ═══════════════════════════════════════════════════════════════
# 审判 · 天庭对袋子的判断
# ═══════════════════════════════════════════════════════════════
func shenpan*(t: var HeavenlyCourt; dai_id: string; expected: PinJi): ShenPan =
  result.dai_id = dai_id
  result.judge = t.name

  # 找到袋子
  var found = false
  for d in t.dai_pool:
    if d.id == dai_id:
      found = true
      result.verdict = $d.pin_ji
      # 审判对了?
      result.correct = (d.pin_ji == expected)
      if result.correct:
        result.reward = d.value * 0.1   # 对了奖励10%价值
        result.penalty = 0
        result.note = "审判正确 — 品级=" & $d.pin_ji
        t.total_reward += result.reward
      else:
        result.reward = 0
        result.penalty = d.value * 0.2  # 错了惩罚20%价值
        result.note = "审判错误 — 实际=" & $d.pin_ji & " 预期=" & $expected
        t.total_penalty += result.penalty
      # 写入历史积累
      t.memory[dai_id] = result.note
      break

  if not found:
    result.correct = false
    result.reward = 0
    result.penalty = 1.0
    result.note = "袋子不存在: " & dai_id
    t.total_penalty += 1.0

  t.history.add(result)

# ═══════════════════════════════════════════════════════════════
# 高维报告 · 层级逻辑认知
# ═══════════════════════════════════════════════════════════════
proc report*(t: HeavenlyCourt) =
  echo "═══ 天庭报告 · ", t.name, " ═══"
  echo ""

  # 按品级分组统计
  var
    jianding = 0  # 坚良
    zhongdeng = 0 # 中等
    liangcha = 0  # 劣质
    weizhi = 0    # 未判

  for d in t.dai_pool:
    case d.pin_ji:
    of pJianLing: jianding += 1
    of pZhongDeng: zhongdeng += 1
    of pLiangCha: liangcha += 1
    of pWeiZhi: weizhi += 1

  echo "── 袋子品级分布 ──"
  echo "  坚实良品:  ", jianding
  echo "  中等可用:  ", zhongdeng
  echo "  劣质弃用:  ", liangcha
  echo "  未判未知:  ", weizhi
  echo "  ──────────────"
  echo "  总袋数:    ", t.dai_pool.len
  echo ""

  echo "── 袋子详情 ──"
  for d in t.dai_pool:
    echo "  [", d.id, "] ", d.owner.alignLeft(8),
         " 值:", $d.value,
         " ", ($d.pin_ji).alignLeft(10),
         " ", ($d.zhuang_tai).alignLeft(8),
         " → ", $d.chu_li

  echo ""
  echo "── 审判统计 ──"
  echo "  累计奖励:  ", t.total_reward
  echo "  累计惩罚:  ", t.total_penalty
  echo "  净收益:    ", t.total_reward - t.total_penalty
  echo "  审判次数:  ", t.history.len
  echo ""

  echo "── 历史积累(记忆) ──"
  for k, v in t.memory:
    echo "  ", k.alignLeft(8), " → ", v

# ═══════════════════════════════════════════════════════════════
# 演示: 袋子入池 + 天庭审判
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  var tj = new_heavenly_court("天庭·袋级认知")

  # 创建袋子(不数米 · 只建袋)
  var d1 = new_dai("袋-甲", "张家庄", 100.0)
  d1.pin_ji = pJianLing
  d1.zhuang_tai = zWanHao
  d1.nei_rong = nJingLiang
  d1.chu_li = nKeZhiJieYong

  var d2 = new_dai("袋-乙", "李家堡", 50.0)
  d2.pin_ji = pZhongDeng
  d2.zhuang_tai = zWuRan
  d2.nei_rong = nHanZa
  d2.chu_li = nXuQingXi

  var d3 = new_dai("袋-丙", "王家屯", 20.0)
  d3.pin_ji = pLiangCha
  d3.zhuang_tai = zPoSun
  d3.nei_rong = nBianZhi
  d3.chu_li = nFeiQi

  tj.add_dai(d1)
  tj.add_dai(d2)
  tj.add_dai(d3)

  # 天庭审判
  discard tj.shenpan("袋-甲", pJianLing)    # 对 → 奖励
  discard tj.shenpan("袋-乙", pLiangCha)    # 错 → 惩罚(实际中等)
  discard tj.shenpan("袋-丙", pLiangCha)    # 对 → 奖励

  # 输出报告
  tj.report()
