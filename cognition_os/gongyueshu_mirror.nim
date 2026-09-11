## ═══════════════════════════════════════════════════════════════
## 公约数镜子 · 公倍数旋转审判
## ═══════════════════════════════════════════════════════════════
## 公约数 = 锚/标准/镜子（应该长什么样）
## 公倍数 = 待审判版本（实际做出来的）
## 审判 = 公倍数绕公约数旋转 · 照镜子 · 匹配度
##
## 匹配度计算: gcd(公约数, 公倍数) / 公约数
##   = 1.0 → 完全匹配 · 通过
##   < 1.0 → 部分匹配 · 修正重做
##   = 0.0 → 完全不匹配 · 废弃
## ═══════════════════════════════════════════════════════════════

import tables, math, strutils

type
  MirrorResult* = object
    gongyueshu*: int       # 公约数（标准）
    gongbeishu*: int       # 公倍数（待审）
    gcd_val*: int          # 最大公约数
    match_rate*: float     # 匹配度 0.0~1.0
    verdict*: string       # "通过" / "修正" / "废弃"
    reward*: float
    penalty*: float

  SubSystemMirror* = object
    name*: string
    gongyueshu*: int       # 锚值（公约数锚点）
    versions*: seq[int]    # 各版本的公倍数
    results*: seq[MirrorResult]

# ═══════════════════════════════════════════════════════════════
# 公约数锚点定义 · 各子系统的标准值
# ═══════════════════════════════════════════════════════════════
const ANCHORS* = {
  "视觉": 12,     # 12色 · 3原色×4阶
  "听觉": 24,     # 24音 · 12平均律×2
  "审判": 36,     # 36判 · 6面×6级
  "天庭": 60,     # 60甲子 · 时间层级
  "审美": 48,     # 48 · 对称美学
  "思考": 72,     # 72 · 逻辑密度
  "行为": 96,     # 96动作
  "判断": 120,    # 120 · 决策树
  "情进": 144,    # 144 · 情感觉察
  "奖励": 180,    # 180 · 价值梯度
  "处罚": 240,    # 240 · 惩罚级差
  "淘汰": 360,    # 360 · 圆周全淘汰
  "过滤": 720,    # 720 · 平方过滤
  "记忆": 1440,   # 1440 · 分钟记忆
}.toTable

# ═══════════════════════════════════════════════════════════════
# 审判函数 · 公倍数绕公约数旋转
# ═══════════════════════════════════════════════════════════════
func shenpan*(name: string; version_value: int): MirrorResult =
  let anchor = ANCHORS[name]    # 公约数
  let candidate = version_value # 公倍数
  let g = gcd(anchor, candidate)
  let rate = float(g) / float(anchor)

  result.gongyueshu = anchor
  result.gongbeishu = candidate
  result.gcd_val = g
  result.match_rate = rate

  if rate >= 0.95:
    result.verdict = "通过"
    result.reward = rate * 10.0
    result.penalty = 0
  elif rate >= 0.5:
    result.verdict = "修正"
    result.reward = 0
    result.penalty = (1.0 - rate) * 5.0
  else:
    result.verdict = "废弃"
    result.reward = 0
    result.penalty = 10.0

# ═══════════════════════════════════════════════════════════════
# 为每个子系统审判版本
# ═══════════════════════════════════════════════════════════════
proc run_all*() =
  echo "═══ 公约数镜子 · 公倍数旋转审判 ═══"
  echo ""

  # 视觉: 公约数=12 · 公倍数候选
  echo "── 视觉(公约数=", $ANCHORS["视觉"], ") ──"
  var r1 = shenpan("视觉", 24)   # 24是12的倍数 → 应通过
  echo "  公倍数=24 → 匹配度=", formatFloat(r1.match_rate, ffDecimal, 2),
       " ", r1.verdict, " 奖赏=", $r1.reward
  var r2 = shenpan("视觉", 18)   # 18和12的gcd=6 → 匹配度0.5
  echo "  公倍数=18 → 匹配度=", formatFloat(r2.match_rate, ffDecimal, 2),
       " ", r2.verdict, " 惩罚=", $r2.penalty
  var r3 = shenpan("视觉", 36)   # 36是12的倍数 → 应通过
  echo "  公倍数=36 → 匹配度=", formatFloat(r3.match_rate, ffDecimal, 2),
       " ", r3.verdict, " 奖赏=", $r3.reward

  # 听觉: 公约数=24
  echo ""
  echo "── 听觉(公约数=", $ANCHORS["听觉"], ") ──"
  var a1 = shenpan("听觉", 48)   # 48是24的倍数
  echo "  公倍数=48 → 匹配度=", formatFloat(a1.match_rate, ffDecimal, 2),
       " ", a1.verdict
  var a2 = shenpan("听觉", 72)   # 72是24的倍数
  echo "  公倍数=72 → 匹配度=", formatFloat(a2.match_rate, ffDecimal, 2),
       " ", a2.verdict
  var a3 = shenpan("听觉", 36)
  echo "  公倍数=36 → 匹配度=", formatFloat(a3.match_rate, ffDecimal, 2),
       " ", a3.verdict

  # 审美: 公约数=48
  echo ""
  echo "── 审美(公约数=", $ANCHORS["审美"], ") ──"
  var m1 = shenpan("审美", 96)   # 96是48的倍数
  echo "  公倍数=96 → 匹配度=", formatFloat(m1.match_rate, ffDecimal, 2),
       " ", m1.verdict
  var m2 = shenpan("审美", 144)  # 144是48的倍数
  echo "  公倍数=144 → 匹配度=", formatFloat(m2.match_rate, ffDecimal, 2),
       " ", m2.verdict

  # 行为: 公约数=96
  echo ""
  echo "── 行为(公约数=", $ANCHORS["行为"], ") ──"
  var b1 = shenpan("行为", 192)
  echo "  公倍数=192 → 匹配度=", formatFloat(b1.match_rate, ffDecimal, 2),
       " ", b1.verdict
  var b2 = shenpan("行为", 288)
  echo "  公倍数=288 → 匹配度=", formatFloat(b2.match_rate, ffDecimal, 2),
       " ", b2.verdict

  # 判断: 公约数=120
  echo ""
  echo "── 判断(公约数=", $ANCHORS["判断"], ") ──"
  var j1 = shenpan("判断", 240)
  echo "  公倍数=240 → 匹配度=", formatFloat(j1.match_rate, ffDecimal, 2),
       " ", j1.verdict
  var j2 = shenpan("判断", 360)
  echo "  公倍数=360 → 匹配度=", formatFloat(j2.match_rate, ffDecimal, 2),
       " ", j2.verdict

  # 天庭: 公约数=60
  echo ""
  echo "── 天庭(公约数=", $ANCHORS["天庭"], ") ──"
  var t1 = shenpan("天庭", 120)
  echo "  公倍数=120 → 匹配度=", formatFloat(t1.match_rate, ffDecimal, 2),
       " ", t1.verdict
  var t2 = shenpan("天庭", 180)
  echo "  公倍数=180 → 匹配度=", formatFloat(t2.match_rate, ffDecimal, 2),
       " ", t2.verdict
  var t3 = shenpan("天庭", 300)
  echo "  公倍数=300 → 匹配度=", formatFloat(t3.match_rate, ffDecimal, 2),
       " ", t3.verdict

  # 情进: 公约数=144
  echo ""
  echo "── 情进(公约数=", $ANCHORS["情进"], ") ──"
  var q1 = shenpan("情进", 288)
  echo "  公倍数=288 → 匹配度=", formatFloat(q1.match_rate, ffDecimal, 2),
       " ", q1.verdict

  # 奖励: 公约数=180
  echo ""
  echo "── 奖励(公约数=", $ANCHORS["奖励"], ") ──"
  var rw1 = shenpan("奖励", 360)
  echo "  公倍数=360 → 匹配度=", formatFloat(rw1.match_rate, ffDecimal, 2),
       " ", rw1.verdict

  # 处罚: 公约数=240
  echo ""
  echo "── 处罚(公约数=", $ANCHORS["处罚"], ") ──"
  var pw1 = shenpan("处罚", 480)
  echo "  公倍数=480 → 匹配度=", formatFloat(pw1.match_rate, ffDecimal, 2),
       " ", pw1.verdict

  # 淘汰: 公约数=360
  echo ""
  echo "── 淘汰(公约数=", $ANCHORS["淘汰"], ") ──"
  var e1 = shenpan("淘汰", 720)
  echo "  公倍数=720 → 匹配度=", formatFloat(e1.match_rate, ffDecimal, 2),
       " ", e1.verdict

  # 过滤: 公约数=720
  echo ""
  echo "── 过滤(公约数=", $ANCHORS["过滤"], ") ──"
  var f1 = shenpan("过滤", 1440)
  echo "  公倍数=1440 → 匹配度=", formatFloat(f1.match_rate, ffDecimal, 2),
       " ", f1.verdict

  # 记忆: 公约数=1440
  echo ""
  echo "── 记忆(公约数=", $ANCHORS["记忆"], ") ──"
  var mem1 = shenpan("记忆", 2880)
  echo "  公倍数=2880 → 匹配度=", formatFloat(mem1.match_rate, ffDecimal, 2),
       " ", mem1.verdict

when is_main_module:
  run_all()
