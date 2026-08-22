# ============================================================
# 满全法 · 袋子核心类型
# ============================================================
# 袋子不是符号，袋子是身份。
# 米不是重点，袋子才是重点。
# 袋子里有米、有虫、有沙、有脏东西。
# 袋子有主人、有价值、有状态、有是否需要清洗。
# 袋子晒太阳、袋子坏掉、袋子被扔掉。
# ============================================================

import tables
import strutils

type
  # --- 袋子状态 ---
  RongQiState* = enum
    rqsNew,        # 新袋
    rqsUsed,       # 旧袋
    rqsDamaged,    # 破损
    rqsDirty,      # 脏了
    rqsWashing,    # 正在洗
    rqsDrying,     # 晾晒中
    rqsClean,      # 干净
    rqsStored,     # 存储中
    rqsDiscarded,  # 已丢弃
    rqsRecycled,   # 回收复用

  # --- 内容物品项（袋子里的东西）---
  ContentItem* = object
    name*: string          # 物品名称（米、沙、虫...）
    kind*: string          # 类别（粮食/杂质/生物）
    quantity*: float       # 数量（不数粒，论堆/论把/论重量）
    is_edible*: bool       # 能吃吗
    is_contaminated*: bool # 污染了吗

  # --- 品鉴结论（每袋一个）---
  PinQuality* = object
    rice_ok*: bool         # 米还吃不
    has_sand*: bool        # 有没有沙子
    has_insect*: bool      # 有没有虫
    needs_washing*: bool   # 要不要洗
    needs_drying*: bool    # 晒没晒干
    discard_reason*: string # 为什么要扔

  # --- 袋子（核心身份）---
  RongQi* = object
    id*: string            # 唯一标识
    name*: string          # 这袋叫什么（不是米的名字，是袋子的名字）
    owner*: string         # 谁家的袋子
    state*: RongQiState    # 袋子状态
    value_coin*: float     # 这袋子值多少钱
    contents*: seq[ContentItem]  # 袋子里的东西（米、沙、虫...）
    quality*: PinQuality   # 品鉴结论
    created_tick*: int     # 何时创建
    last_judged*: int      # 上次审判时间

  # --- 袋子注册表（掌柜能看到的所有袋子）---
  RongQiRegistry* = object
    bags*: Table[string, RongQi]
    master*: string        # 谁是天觉的主人


# ============================================================
# 构造
# ============================================================
func new_rongqi*(id, name, owner: string): RongQi =
  result.id = id
  result.name = name
  result.owner = owner
  result.state = rqsNew
  result.value_coin = 0.0
  result.contents = @[]
  result.quality = PinQuality(
    rice_ok: true,
    has_sand: false,
    has_insect: false,
    needs_washing: false,
    needs_drying: false,
    discard_reason: ""
  )
  result.created_tick = 0
  result.last_judged = -1

func new_registry*(master: string): RongQiRegistry =
  result.master = master
  result.bags = initTable[string, RongQi]()


# ============================================================
# 内容物操作
# ============================================================
func add_content*(bag: var RongQi; item: ContentItem) =
  bag.contents.add(item)

func total_rice*(bag: RongQi): float =
  ## 袋子里还有多少米（不数粒，论重量/论堆）
  for c in bag.contents:
    if c.kind == "粮食" and c.is_edible:
      result += c.quantity

func total_contaminants*(bag: RongQi): float =
  ## 袋子里有多少脏东西
  for c in bag.contents:
    if not c.is_edible or c.is_contaminated:
      result += c.quantity

func has_sand*(bag: RongQi): bool =
  for c in bag.contents:
    if c.name in ["沙子", "沙", "碎石", "土"]:
      return true

func has_insect*(bag: RongQi): bool =
  for c in bag.contents:
    if c.kind == "生物":
      return true


# ============================================================
# 状态判断（简洁、直接）
# ============================================================
func needs_washing*(bag: RongQi): bool =
  bag.has_sand or bag.quality.has_sand

func needs_drying*(bag: RongQi): bool =
  bag.quality.needs_drying

func is_edible*(bag: RongQi): bool =
  bag.quality.rice_ok and not bag.quality.has_insect

func should_discard*(bag: RongQi): bool =
  ## 判断该袋子是否该扔
  if bag.quality.has_insect and bag.total_contaminants > 0.1:
    return true
  if bag.state == rqsDamaged or bag.state == rqsDirty:
    return true
  if not bag.quality.rice_ok:
    return true
  return false

func summary*(bag: RongQi): string =
  ## 一袋子的简洁描述
  var s = "[" & bag.name & "] 主:" & bag.owner &
          " 值:" & formatFloat(bag.value_coin, ffDecimal, 1) &
          " 米:" & formatFloat(bag.total_rice, ffDecimal, 1)
  if bag.has_sand: s &= " [有沙]"
  if bag.has_insect: s &= " [有虫]"
  if bag.needs_washing: s &= " [要洗]"
  if bag.needs_drying: s &= " [要晒]"
  if bag.should_discard: s &= " [该扔]"
  return s


# ============================================================
# 测试
# ============================================================
when is_main_module:
  echo "=== 袋子测试 ==="

  # --- 创建一袋好米 ---
  var riceBag = new_rongqi("bag_001", "王老五家的米袋", "王老五")
  riceBag.value_coin = 50.0
  riceBag.state = rqsStored
  riceBag.add_content(ContentItem(name: "米", kind: "粮食", quantity: 50.0, is_edible: true))
  riceBag.add_content(ContentItem(name: "沙子", kind: "杂质", quantity: 0.5, is_edible: false))

  echo riceBag.summary()
  echo "  要吃吗:", riceBag.is_edible
  echo "  要洗吗:", riceBag.needs_washing

  # --- 创建一袋坏米 ---
  var badBag = new_rongqi("bag_002", "李家的陈米", "李四")
  badBag.value_coin = 20.0
  badBag.state = rqsDirty
  badBag.quality.rice_ok = false
  badBag.quality.has_insect = true
  badBag.quality.discard_reason = "虫蛀严重"
  badBag.add_content(ContentItem(name: "陈米", kind: "粮食", quantity: 30.0, is_edible: true))
  badBag.add_content(ContentItem(name: "米虫", kind: "生物", quantity: 3.0, is_edible: false))

  echo badBag.summary()
  echo "  该扔吗:", badBag.should_discard
  echo "  原因:", badBag.quality.discard_reason
