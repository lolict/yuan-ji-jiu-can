# ============================================================
# 满全法 · 类型感知组件交换系统
# ============================================================
# 核心哲学：
#   类型是标签，不是屏障。
#   组件可以从A类型拆下，装到B类型上。
#   屏幕不问出处，能亮就是好屏。
#   电池不问主人，能跑就是好电。
#
# 产品类型 ≠ 组件类型。
#   安卓手机是一种产品类型。
#   屏幕是一种组件类型。
#   安卓的屏幕可以装到电脑上。
#   电脑的电池可以装到安卓上。
#
# 交易是双向互惠：
#   买家寄给卖家 → 卖家获得产品
#   卖家付钱给买家 → 买家获得钱
#   双方都获得实惠。
# ============================================================

import tables
import strutils
import math


# ============================================================
# 一、产品类型定义
# ============================================================
type
  ProductType* = enum
    ptLaoRenJi,    # 老人机
    ptAndroid,     # 安卓手机
    ptiPhone,      #苹果手机
    ptComputer,    #电脑
    ptTablet,      #平板
    ptWatch,       #手表

func pt_name*(p: ProductType): string =
  case p
  of ptLaoRenJi: return "老人机"
  of ptAndroid: return "安卓手机"
  of ptiPhone: return "苹果手机"
  of ptComputer: return "电脑"
  of ptTablet: return "平板"
  of ptWatch: return "手表"


# ============================================================
# 二、组件类型定义
# ============================================================
type
  ComponentType* = enum
    ctScreen,      # 屏幕
    ctBattery,     # 电池
    ctKeyboard,    # 键盘
    ctOS,          # 操作系统
    ctBackCover,   # 后盖
    ctMainBoard,   # 主板
    ctCamera,      # 摄像头
    ctSpeaker,     # 扬声器
    ctMemory,      # 内存
    ctStorage,     # 硬盘

func ct_name*(c: ComponentType): string =
  case c
  of ctScreen: return "屏幕"
  of ctBattery: return "电池"
  of ctKeyboard: return "键盘"
  of ctOS: return "操作系统"
  of ctBackCover: return "后盖"
  of ctMainBoard: return "主板"
  of ctCamera: return "摄像头"
  of ctSpeaker: return "扬声器"
  of ctMemory: return "内存"
  of ctStorage: return "硬盘"


# ============================================================
# 三、产品实例（一坨坨具体的产品）
# ============================================================
type
  Component* = object
    comp_id*: string
    comp_type*: ComponentType
    name*: string          # 这个组件叫什么名字
    spec*: string          # 规格（寸数、容量等）
    condition*: float      # 状态 0.0-1.0（1.0=全新）
    compatible_with*: seq[ProductType]  # 能装到哪些产品类型上
    value_coin*: float     # 这组件值多少钱

  Product* = object
    prod_id*: string
    prod_type*: ProductType
    name*: string          # 这产品叫什么
    owner*: string         # 谁家的
    components*: Table[string, Component]  # 这产品上装了什么组件
    value_coin*: float     # 这产品值多少钱

  # 产品类型 → 默认需要哪些组件类型（定义产品的「形状」）
  ProductSpec* = object
    required*: seq[ComponentType]  # 必须有的组件
    optional*: seq[ComponentType]  # 可选组件


# ============================================================
# 四、类型兼容性系统
# ============================================================
func default_spec*(pt: ProductType): ProductSpec =
  ## 每种产品类型的默认组件配置
  case pt
  of ptLaoRenJi:
    result.required = @[ctScreen, ctBattery, ctKeyboard, ctOS, ctBackCover]
    result.optional = @[ctSpeaker, ctStorage]
  of ptAndroid:
    result.required = @[ctScreen, ctBattery, ctOS, ctBackCover]
    result.optional = @[ctKeyboard, ctCamera, ctSpeaker, ctMemory, ctStorage, ctMainBoard]
  of ptiPhone:
    result.required = @[ctScreen, ctBattery, ctOS, ctBackCover, ctMainBoard]
    result.optional = @[ctCamera, ctSpeaker, ctMemory, ctStorage]
  of ptComputer:
    result.required = @[ctScreen, ctBattery, ctKeyboard, ctOS, ctMainBoard, ctStorage]
    result.optional = @[ctBackCover, ctCamera, ctSpeaker, ctMemory]
  of ptTablet:
    result.required = @[ctScreen, ctBattery, ctOS, ctBackCover]
    result.optional = @[ctKeyboard, ctCamera, ctSpeaker, ctMemory, ctStorage]
  of ptWatch:
    result.required = @[ctScreen, ctBattery, ctOS, ctBackCover]
    result.optional = @[ctSpeaker, ctMemory, ctStorage]


func is_compatible*(comp: Component; pt: ProductType): bool =
  ## 这个组件能否装到这种产品类型上？
  pt in comp.compatible_with


# ============================================================
# 五、组件操作
# ============================================================
func new_component*(id, name: string; ct: ComponentType; spec: string;
                   compatible: seq[ProductType]; value: float): Component =
  result.comp_id = id
  result.comp_type = ct
  result.name = name
  result.spec = spec
  result.condition = 1.0
  result.compatible_with = compatible
  result.value_coin = value

func new_product*(id, name, owner: string; pt: ProductType): Product =
  result.prod_id = id
  result.prod_type = pt
  result.name = name
  result.owner = owner
  result.components = initTable[string, Component]()
  result.value_coin = 0.0

func install*(prod: var Product; comp: Component): bool =
  ## 把组件装到产品上（不管原来是什么类型的，兼容就能装）
  if comp.comp_id notin prod.components:
    prod.components[comp.comp_id] = comp
    prod.value_coin += comp.value_coin
    return true
  return false

func remove*(prod: var Product; comp_id: string): Component =
  ## 从产品上拆下组件
  if comp_id in prod.components:
    result = prod.components[comp_id]
    prod.components.del(comp_id)
    prod.value_coin -= result.value_coin

func swap*(a: var Product; b: var Product; comp_id_a, comp_id_b: string): bool =
  ## A的产品组件 和 B的产品组件 互换
  if comp_id_a notin a.components or comp_id_b notin b.components:
    return false
  let comp_a = a.components[comp_id_a]
  let comp_b = b.components[comp_id_b]
  # 检查兼容性
  if not is_compatible(comp_a, b.prod_type):
    return false
  if not is_compatible(comp_b, a.prod_type):
    return false
  # 执行互换
  a.components.del(comp_id_a)
  b.components.del(comp_id_b)
  a.components[comp_id_b] = comp_b
  b.components[comp_id_a] = comp_a
  return true


# ============================================================
# 六、双向互惠交易系统
# ============================================================
type
  TradeRole* = enum
    trBuyer,   # 买家
    trSeller   # 卖家

  TradeOffer* = object
    offer_id*: string
    from_whom*: string       # 谁发起的
    give_product*: Product   # 我给出什么产品
    want_money*: float       # 我想要多少钱
    want_component*: ComponentType  # 或者我想要什么类型的组件

  TradeContract* = object
    contract_id*: string
    party_a*: string         # 甲方
    party_b*: string         # 乙方
    a_gives*: Product        # 甲方给乙方什么
    b_gives_money*: float    # 乙方付给甲方多少钱
    a_gets_money*: float     # 甲方获得钱
    b_gets_product*: Product # 乙方获得产品
    status*: string          # "pending" / "completed" / "cancelled"

  TradeEngine* = object
    name*: string
    contracts*: seq[TradeContract]
    completed*: seq[TradeContract]
    total_volume*: float     # 总交易量


func new_trade_engine*(name: string): TradeEngine =
  result.name = name
  result.contracts = @[]
  result.completed = @[]
  result.total_volume = 0.0


# ============================================================
# 核心交易：买家寄给卖家，卖家付钱给买家
# ============================================================
func make_offer*(engine: var TradeEngine; from_whom: string; give: Product;
                 want_money: float): TradeOffer =
  ## 一个交易意向：我给出这个产品，想要这么多钱
  result.offer_id = "offer_" & $engine.contracts.len
  result.from_whom = from_whom
  result.give_product = give
  result.want_money = want_money
  result.want_component = ctScreen  # 默认

func execute_trade*(engine: var TradeEngine; buyer: string; seller: string;
                    buyer_gives: Product; seller_pays: float): TradeContract =
  ## 执行双向互惠交易：
  ##   买家把产品寄给卖家 → 卖家获得产品
  ##   卖家把钱付给买家 → 买家获得钱
  ##   双方都获得实惠。
  var c = TradeContract(
    contract_id: "trade_" & $(engine.contracts.len + engine.completed.len),
    party_a: buyer,
    party_b: seller,
    a_gives: buyer_gives,
    b_gives_money: seller_pays,
    a_gets_money: seller_pays,
    b_gets_product: buyer_gives,
    status: "completed"
  )
  engine.completed.add(c)
  engine.total_volume += seller_pays
  return c


func mutual_exchange*(engine: var TradeEngine; party_a, party_b: string;
                      a_gives: Product; b_gives: Product;
                      a_pays: float; b_pays: float): TradeContract =
  ## 双向互换交易：
  ##   A把产品给B，B把钱给A
  ##   B把产品给A，A把钱给B
  ##   双方同时给出，双方同时获得。
  var c = TradeContract(
    contract_id: "mutual_" & $(engine.contracts.len + engine.completed.len),
    party_a: party_a,
    party_b: party_b,
    a_gives: a_gives,
    b_gives_money: a_pays,
    a_gets_money: b_pays,
    b_gets_product: b_gives,
    status: "completed"
  )
  engine.completed.add(c)
  engine.total_volume += a_pays + b_pays
  return c


# ============================================================
# 七、报告与展示
# ============================================================
func product_summary*(p: Product): string =
  var s = "[" & p.name & "] 类型:" & pt_name(p.prod_type) &
          " 主人:" & p.owner & " 值:" & formatFloat(p.value_coin, ffDecimal, 1)
  s &= " 组件:" & $p.components.len
  return s

func comp_summary*(c: Component): string =
  var s = "[" & c.name & "] 类型:" & ct_name(c.comp_type) &
          " 规格:" & c.spec &
          " 状态:" & formatFloat(c.condition * 100, ffDecimal, 0) & "%" &
          " 值:" & formatFloat(c.value_coin, ffDecimal, 1)
  return s

func contract_summary*(c: TradeContract): string =
  var s = "合约[" & c.contract_id & "] " & c.status & "\n"
  s &= "  " & c.party_a & " → 给出产品[" & c.a_gives.name & "], 获得" & formatFloat(c.a_gets_money, ffDecimal, 1) & "钱\n"
  s &= "  " & c.party_b & " → 给出" & formatFloat(c.b_gives_money, ffDecimal, 1) & "钱, 获得产品[" & c.b_gets_product.name & "]"
  return s


# ============================================================
# 测试
# ============================================================
when is_main_module:
  echo "══════════════════════════════════════════════"
  echo "   满全法 · 类型感知组件交换系统"
  echo "   类型是标签，不是屏障"
  echo "══════════════════════════════════════════════"
  echo ""

  # --- 创建产品类型 ---
  echo "【一、产品类型】"
  for pt in [ptLaoRenJi, ptAndroid, ptiPhone, ptComputer, ptTablet, ptWatch]:
    let spec = pt.default_spec()
    var req: seq[string] = @[]
    for r in spec.required: req.add(ct_name(r))
    echo "  ", pt_name(pt), " → 必需: ", req.join(", ")
  echo ""

  # --- 创建组件 ---
  echo "【二、组件类型】"
  let screen_laoren = new_component("scr_01", "老人机屏", ctScreen, "2.4寸",
    @[ptLaoRenJi, ptAndroid, ptWatch], 30.0)
  let screen_android = new_component("scr_02", "安卓全面屏", ctScreen, "6.1寸OLED",
    @[ptAndroid, ptiPhone, ptTablet, ptComputer], 300.0)
  let battery_computer = new_component("bat_01", "电脑电池", ctBattery, "5000mAh",
    @[ptComputer, ptAndroid, ptTablet, ptLaoRenJi], 200.0)
  let keyboard_computer = new_component("kb_01", "电脑机械键盘", ctKeyboard, "87键",
    @[ptComputer, ptAndroid, ptTablet], 150.0)
  let os_computer = new_component("os_01", "Windows系统", ctOS, "Win11",
    @[ptComputer, ptTablet], 500.0)
  let os_android = new_component("os_02", "安卓系统", ctOS, "Android14",
    @[ptAndroid, ptTablet, ptWatch], 0.0)

  echo "  ", comp_summary(screen_laoren)
  echo "  ", comp_summary(screen_android)
  echo "  ", comp_summary(battery_computer)
  echo "  ", comp_summary(keyboard_computer)
  echo ""

  # --- 创建产品 ---
  echo "【三、创建产品】"
  var laorenji = new_product("p01", "王大爷的老人机", "王大爷", ptLaoRenJi)
  var android = new_product("p02", "李哥的安卓手机", "李哥", ptAndroid)
  var computer = new_product("p03", "张总的电脑", "张总", ptComputer)

  discard laorenji.install(screen_laoren)
  discard android.install(screen_android)
  discard android.install(os_android)
  discard computer.install(battery_computer)
  discard computer.install(keyboard_computer)
  discard computer.install(os_computer)

  echo "  ", product_summary(laorenji)
  echo "  ", product_summary(android)
  echo "  ", product_summary(computer)
  echo ""

  # --- 跨类型交换 ---
  echo "【四、跨类型组件交换】"
  echo "  把电脑的电池装到老人机上..."
  # 拆下电脑的电池
  let bat = computer.remove("bat_01")
  echo "   拆下: ", comp_summary(bat)
  # 装到老人机上
  let ok = laorenji.install(bat)
  let res1 = if ok: "✓ 成功" else: "✗ 不兼容"
  echo "   装到老人机: ", res1
  echo "   老人机现在: ", product_summary(laorenji)
  echo ""

  echo "  把安卓的屏幕装到电脑上..."
  let scr = android.remove("scr_02")
  echo "   拆下: ", comp_summary(scr)
  let ok2 = computer.install(scr)
  let res2 = if ok2: "✓ 成功" else: "✗ 不兼容"
  echo "   装到电脑: ", res2
  echo "   电脑现在: ", product_summary(computer)
  echo ""

  # --- 双向交易 ---
  echo "【五、双向互惠交易】"
  var engine = new_trade_engine("满全法交易所")

  # 交易1：老人把老人机卖给电脑商家，商家付钱给老人
  let trade1 = engine.execute_trade(
    buyer = "王大爷",      # 买家是王大爷
    seller = "李老板",     # 卖家是李老板
    buyer_gives = laorenji, # 买家把老人机给卖家
    seller_pays = 80.0     # 卖家付80钱给买家
  )
  echo "  ", contract_summary(trade1)
  echo ""

  # 交易2：互换——李哥的安卓 换 张总的电脑（补差价）
  let trade2 = engine.mutual_exchange(
    party_a = "李哥",
    party_b = "张总",
    a_gives = android,
    b_gives = computer,
    a_pays = 100.0,   # 李哥补100差价
    b_pays = 0.0
  )
  echo "  ", contract_summary(trade2)
  echo ""

  # --- 总结 ---
  echo "【六、交易总结】"
  echo "  已完成交易: ", engine.completed.len
  echo "  总交易量: ", formatFloat(engine.total_volume, ffDecimal, 1), "钱"
  echo ""
  echo "══════════════════════════════════════════════"
  echo "   类型是标签，不是屏障"
  echo "══════════════════════════════════════════════"
