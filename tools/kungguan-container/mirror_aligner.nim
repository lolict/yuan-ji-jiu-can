# 满全法·镜像对齐引擎
# 人类判断 = 镜像对比 + 时序维度跃迁 + 贴标签 + 奖励回路
#
# 核心流:
#   对象输入 → 多维蓝本对齐 → 逐层贴标签 → 奖励计算 → 综合榜(凭据)

import tables
import sequtils
import algorithm

type
  # 对齐维度（时序跃迁的每一层）
  AlignDim* = enum
    adVoice        # 声音蓝本
    adPhase        # 形象相位
    adContour      # 轮廓
    adSize         # 大小
    adWeight       # 粗细
    adInclusion    # 包含/被包含逻辑
    adColor        # 色差
    adHeight       # 高低对齐
    adWidth        # 宽度对齐
    adSlope        # 斜度对齐

  # 对齐结果（单维度）
  AlignResult* = object
    dim*: AlignDim
    score*: float        # 0.0-1.0 对齐度
    delta*: float        # 偏差量
    label*: string       # 该维度贴的标签

  # 对象特征（被评判者）
  ObjectFeature* = object
    id*: string
    voice_blueprint*: string    # 声音蓝本引用
    phase_blueprint*: string    # 形象相位蓝本
    contour*: string            # 轮廓描述
    size*: float                # 大小值
    weight*: float              # 粗细值
    inclusion*: string          # 包含逻辑
    color*: string              # 色彩
    height*: float              # 高低
    width*: float               # 宽度
    slope*: float               # 斜度
    emotion*: string            # 情感状态
    personality*: string        # 个性化性格

  # 蓝本（对齐参照系）
  Blueprint* = object
    name*: string
    voice_ref*: string
    phase_ref*: string
    contour_ref*: string
    size_ref*: float
    weight_ref*: float
    color_ref*: string
    height_ref*: float
    width_ref*: float
    slope_ref*: float

  # 标签（贴给对象的个性化感受）
  Label* = object
    tag*: string           # 标签名
    dim*: AlignDim         # 所属维度
    value*: float          # 感受值
    emotion*: string       # 情感色彩
    note*: string          # 直白描述

  # 奖励（自己发放的愉悦感）
  Reward* = object
    source*: string        # 奖励来源维度
    amount*: float         # 奖励量
    duration*: float       # 愉悦感持久度
    reason*: string        # 为什么奖励

  # 综合凭据（榜单条目）
  Credential* = object
    obj_id*: string
    total_score*: float
    align_seq*: seq[AlignResult]   # 时序对齐序列
    labels*: seq[Label]            # 所有标签
    rewards*: seq[Reward]          # 奖励记录
    pleasure_sum*: float           # 愉悦感总和
    rank_note*: string             # 排名凭据说明

# ============================================================
# 对齐计算
# ============================================================

func align_single*(bp: Blueprint, obj: ObjectFeature, dim: AlignDim): AlignResult =
  result.dim = dim
  case dim
  of adVoice:
    result.score = if bp.voice_ref == obj.voice_blueprint: 1.0 else: 0.3
    result.delta = if bp.voice_ref == obj.voice_blueprint: 0.0 else: 0.7
    result.label = "声:" & obj.voice_blueprint
  of adPhase:
    result.score = if bp.phase_ref == obj.phase_blueprint: 1.0 else: 0.4
    result.delta = if bp.phase_ref == obj.phase_blueprint: 0.0 else: 0.6
    result.label = "相:" & obj.phase_blueprint
  of adContour:
    result.score = if bp.contour_ref == obj.contour: 1.0 else: 0.5
    result.delta = if bp.contour_ref == obj.contour: 0.0 else: 0.5
    result.label = "廓:" & obj.contour
  of adSize:
    result.delta = abs(obj.size - bp.size_ref)
    result.score = 1.0 / (1.0 + result.delta)
    result.label = "尺:" & $obj.size
  of adWeight:
    result.delta = abs(obj.weight - bp.weight_ref)
    result.score = 1.0 / (1.0 + result.delta)
    result.label = "粗:" & $obj.weight
  of adInclusion:
    result.score = if obj.inclusion == "包含" or obj.inclusion == "被包含": 0.8 else: 0.2
    result.delta = if obj.inclusion == bp.contour_ref: 0.0 else: 0.5
    result.label = "含:" & obj.inclusion
  of adColor:
    result.score = if bp.color_ref == obj.color: 1.0 else: 0.6
    result.delta = if bp.color_ref == obj.color: 0.0 else: 0.4
    result.label = "色:" & obj.color
  of adHeight:
    result.delta = abs(obj.height - bp.height_ref)
    result.score = 1.0 / (1.0 + result.delta)
    result.label = "高:" & $obj.height
  of adWidth:
    result.delta = abs(obj.width - bp.width_ref)
    result.score = 1.0 / (1.0 + result.delta)
    result.label = "宽:" & $obj.width
  of adSlope:
    result.delta = abs(obj.slope - bp.slope_ref)
    result.score = 1.0 / (1.0 + result.delta)
    result.label = "斜:" & $obj.slope

# ============================================================
# 时序维度跃迁（按顺序逐层对齐）
# ============================================================

func get_align_order*(): seq[AlignDim] =
  return @[adVoice, adPhase, adContour, adSize, adWeight,
           adInclusion, adColor, adHeight, adWidth, adSlope]

func align_sequence*(bp: Blueprint, obj: ObjectFeature): seq[AlignResult] =
  result = @[]
  for dim in get_align_order():
    result.add(align_single(bp, obj, dim))

# ============================================================
# 贴标签（状态直白，标靓个性化感受）
# ============================================================

func tag_object*(obj: ObjectFeature, aligns: seq[AlignResult]): seq[Label] =
  result = @[]
  # 情感标签
  result.add(Label(tag:"情感", dim:adVoice, value:0.8, emotion:obj.emotion, note:"她的情感是" & obj.emotion))
  # 性格标签
  result.add(Label(tag:"性格", dim:adPhase, value:0.7, emotion:obj.personality, note:"性格" & obj.personality))
  # 逐维度标签
  for a in aligns:
    let tag = case a.dim
      of adVoice: "声线"
      of adPhase: "相位"
      of adContour: "轮廓"
      of adSize: "尺寸"
      of adWeight: "粗细"
      of adInclusion: "包含"
      of adColor: "色彩"
      of adHeight: "高低"
      of adWidth: "宽度"
      of adSlope: "斜度"
    result.add(Label(tag:tag, dim:a.dim, value:a.score, emotion:obj.emotion, note:a.label & "=" & $a.score))

# ============================================================
# 奖励回路（自己发放，控制愉悦感）
# ============================================================

func calc_rewards*(aligns: seq[AlignResult], obj: ObjectFeature): seq[Reward] =
  result = @[]
  var pleasure_acc = 0.0
  for a in aligns:
    let amount = a.score * 10.0
    let dur = if a.score > 0.8: 5.0 elif a.score > 0.5: 2.0 else: 0.5
    pleasure_acc += amount * dur
    result.add(Reward(
      source: $a.dim,
      amount: amount,
      duration: dur,
      reason: "对齐" & $a.dim & "得分" & $a.score & "，感受" & obj.emotion
    ))
  # 综合奖励
  result.add(Reward(
    source:"综合",
    amount: pleasure_acc * 0.1,
    duration: pleasure_acc * 0.05,
    reason: "综合愉悦感"
  ))

# ============================================================
# 综合凭据（榜单）
# ============================================================

func make_credential*(obj: ObjectFeature, bp: Blueprint): Credential =
  let aligns = align_sequence(bp, obj)
  let labels = tag_object(obj, aligns)
  let rewards = calc_rewards(aligns, obj)
  var total = 0.0
  var pleasure = 0.0
  for a in aligns: total += a.score
  for r in rewards: pleasure += r.amount * r.duration
  result = Credential(
    obj_id: obj.id,
    total_score: total,
    align_seq: aligns,
    labels: labels,
    rewards: rewards,
    pleasure_sum: pleasure,
    rank_note: "总分" & $total & "愉悦" & $pleasure
  )

# ============================================================
# 凭据排序（综合榜）
# ============================================================

func rank_credentials*(creds: seq[Credential]): seq[Credential] =
  result = creds
  proc mycmp(a, b: Credential): int =
    if a.pleasure_sum > b.pleasure_sum: return -1
    if a.pleasure_sum < b.pleasure_sum: return 1
    return 0
  sort(result, mycmp)

# ============================================================
# 凭据用来干什么
# ============================================================

func credential_usage*(cred: Credential): string =
  ## 凭据作为:
  ## 1. 融合优先级（愉悦感高的先融合）
  ## 2. 时间层级排序（谁先谁后）
  ## 3. 奖励发放凭证（领取愉悦感的凭据）
  ## 4. 自定义编码索引（替代传统量化分数）
  var s = "凭据[" & cred.obj_id & "] 用途:\n"
  s &= "  融合优先级: " & $cred.pleasure_sum & "\n"
  s &= "  时间层级序: " & $cred.total_score & "\n"
  s &= "  奖励凭证: " & $cred.rewards.len & "条\n"
  s &= "  编码索引: " & cred.obj_id & "→" & $cred.total_score
  return s

# ============================================================
# 测试
# ============================================================

when is_main_module:
  echo "=== 镜像对齐引擎 ==="
  echo ""

  # 定义蓝本（自己的审美标准）
  var bp = Blueprint(
    name: "我的蓝本",
    voice_ref: "温柔",
    phase_ref: "正写",
    contour_ref: "圆润",
    size_ref: 5.0,
    weight_ref: 3.0,
    color_ref: "暖色",
    height_ref: 5.0,
    width_ref: 5.0,
    slope_ref: 0.0
  )

  # 被评判对象
  var obj1 = ObjectFeature(
    id: "姑娘甲",
    voice_blueprint: "温柔",
    phase_blueprint: "正写",
    contour: "圆润",
    size: 5.2,
    weight: 3.1,
    inclusion: "被包含",
    color: "暖色",
    height: 5.0,
    width: 4.8,
    slope: 0.1,
    emotion: "愉悦",
    personality: "柔和"
  )

  var obj2 = ObjectFeature(
    id: "姑娘乙",
    voice_blueprint: "高亢",
    phase_blueprint: "镜像",
    contour: "棱角",
    size: 6.0,
    weight: 4.0,
    inclusion: "独立",
    color: "冷色",
    height: 6.0,
    width: 5.5,
    slope: 0.5,
    emotion: "冷静",
    personality: "刚强"
  )

  var obj3 = ObjectFeature(
    id: "姑娘丙",
    voice_blueprint: "温柔",
    phase_blueprint: "正写",
    contour: "圆润",
    size: 4.8,
    weight: 2.9,
    inclusion: "被包含",
    color: "暖色",
    height: 4.9,
    width: 5.1,
    slope: -0.1,
    emotion: "羞怯",
    personality: "娇柔"
  )

  # 生成凭据
  var creds = @[
    make_credential(obj1, bp),
    make_credential(obj2, bp),
    make_credential(obj3, bp)
  ]

  # 排序
  let ranked = rank_credentials(creds)

  echo "=== 综合榜 ==="
  for i, c in ranked:
    echo "#", i+1, " ", c.obj_id, " 愉悦:", c.pleasure_sum, " 总分:", c.total_score

  echo ""
  echo "=== 凭据详情 ==="
  for c in ranked:
    echo "---"
    echo c.obj_id
    echo "  对齐序列:"
    for a in c.align_seq:
      echo "    ", $a.dim, " → ", a.score
    echo "  标签:"
    for l in c.labels:
      echo "    [", l.tag, "] ", l.note
    echo "  奖励:"
    for r in c.rewards:
      echo "    ", r.source, ": +", r.amount, " (持久", r.duration, ")"
    echo "  ", credential_usage(c)

  echo ""
  echo "=== 凭据用途 ==="
  echo "1. 融合优先级: 愉悦感高的先融合进身体"
  echo "2. 时间层级: 按总分排序决定谁先谁后"
  echo "3. 奖励凭证: 每条奖励记录是领取愉悦感的凭据"
  echo "4. 编码索引: 替代传统分数，用自定义符号编码"
