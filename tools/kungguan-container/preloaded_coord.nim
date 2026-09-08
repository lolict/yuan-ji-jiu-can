# 满全法·预加载坐标容器
# 不用笛卡尔坐标（x,y,z），所有坐标预定义为静态查找表
# 通过名称/编码直接定位，无需运行时计算
# 相位容器处理方向变化，行为容器化建制，直达目标

import tables, strutils

type
  # ============================================================
  # 相位（8主相位 + 10斜角相位 = 18相位）
  # 不是坐标，是预加载的站位名称
  # ============================================================
  Phase* = enum
    phEast        # 0 东
    phWest        # 1 西
    phSouth       # 2 南
    phNorth       # 3 北
    phHeaven      # 4 天
    phEarth       # 5 地
    phCenter      # 6 中央
    phOrigin      # 7 原点（起始）
    phEastSouth   # 8 东南
    phWestSouth   # 9 西南
    phEastNorth   # 10 东北
    phWestNorth   # 11 西北
    phHeavenEast  # 12 天东
    phHeavenWest  # 13 天西
    phHeavenSouth # 14 天南
    phHeavenNorth # 15 天北
    phEarthEast   # 16 地东
    phEarthWest   # 17 地西

  # ============================================================
  # 层级（加法线排布，5层）
  # ============================================================
  Level* = enum
    lvBitStream   # 0 比特流（最底层）
    lvBinary      # 1 二进制
    lvSymbol      # 2 符号
    lvContainer   # 3 容器
    lvCategory    # 4 范畴

  # ============================================================
  # 进制类型（标识功能属于哪个进制/建制）
  # ============================================================
  BaseInstitution* = enum
    biShengmu     # 0 声母48进制
    biYunmu       # 1 韵母66进制
    biShengdiao   # 2 声调60进制
    biBianyin     # 3 变音50进制
    biColor       # 4 颜色42进制
    biVoice       # 5 声音100进制
    biSpace       # 6 空间相位18进制
    biMirror      # 7 镜像对齐10进制
    biMusic       # 8 音乐3进制
    biText        # 9 文本显示18×10进制
    biBehavior    # 10 行为建制（通用）
    biConscious   # 11 意识流8进制
    biFlow        # 12 流属性12进制
    biIdentity    # 13 身份2进制（高位持有者=1,低位持有者=0）
    biWitness     # 14 三元见证
    biFractal     # 15 分形递归

  # ============================================================
  # 预加载位置（静态查找表条目）
  # 不是计算出来的，是预定义好的
  # ============================================================
  PreloadedPos* = object
    name*: string           # 名称（东、前、头...）
    code*: string           # 编码（mltt、gtt...）
    phase*: Phase           # 所属相位
    level*: Level           # 所属层级
    base*: BaseInstitution  # 所属进制/建制
    bit_width*: int         # 占用比特数
    ref_id*: int            # 引用ID（指向外部大数据）

  # ============================================================
  # 行为建制（容器化的行为）
  # 导入名称/编码即可使用对应进制功能
  # ============================================================
  BehaviorInst* = object
    name*: string           # 建制名称
    code*: string           # 编码
    base*: BaseInstitution  # 使用的进制
    entry*: string          # 入口位置名称
    actions*: seq[string]   # 预定义动作序列（相位变化链）

  # ============================================================
  # 相位曲线（预定义的方向变化路径）
  # 不是运行时计算，是预加载的路径
  # ============================================================
  PhaseCurve* = object
    name*: string           # 曲线名称
    path*: seq[Phase]       # 相位序列（预定义路径）
    description*: string    # 描述

  # ============================================================
  # 预加载图谱（所有坐标已预定义）
  # ============================================================
  PreloadedAtlas* = object
    positions*: Table[string, PreloadedPos]      # 名称 → 位置
    codes*: Table[string, string]                # 编码 → 名称
    behaviors*: Table[string, BehaviorInst]      # 名称 → 行为建制
    curves*: Table[string, PhaseCurve]           # 名称 → 相位曲线
    phase_index*: Table[Phase, seq[string]]      # 相位 → 名称列表

# ============================================================
# 预加载坐标表（静态定义，不计算）
# ============================================================

func preload_positions*(): Table[string, PreloadedPos] =
  # 全部预定义，运行时只查不算
  result = init_table[string, PreloadedPos]()

  # --- 东西南北天地中（7主相位）---
  result["东"] = PreloadedPos(name:"东", code:"mltt-d", phase:phEast, level:lvSymbol, base:biSpace, bit_width:5, ref_id:0)
  result["西"] = PreloadedPos(name:"西", code:"mltt-x", phase:phWest, level:lvSymbol, base:biSpace, bit_width:5, ref_id:1)
  result["南"] = PreloadedPos(name:"南", code:"mltt-n", phase:phSouth, level:lvSymbol, base:biSpace, bit_width:5, ref_id:2)
  result["北"] = PreloadedPos(name:"北", code:"mltt-b", phase:phNorth, level:lvSymbol, base:biSpace, bit_width:5, ref_id:3)
  result["天"] = PreloadedPos(name:"天", code:"mltt-t", phase:phHeaven, level:lvSymbol, base:biSpace, bit_width:5, ref_id:4)
  result["地"] = PreloadedPos(name:"地", code:"mltt-d", phase:phEarth, level:lvSymbol, base:biSpace, bit_width:5, ref_id:5)
  result["中"] = PreloadedPos(name:"中", code:"mltt-z", phase:phCenter, level:lvSymbol, base:biSpace, bit_width:5, ref_id:6)
  result["原"] = PreloadedPos(name:"原", code:"mltt-y", phase:phOrigin, level:lvSymbol, base:biSpace, bit_width:5, ref_id:7)

  # --- 斜角相位（4斜角）---
  result["东南"] = PreloadedPos(name:"东南", code:"mltt-dn", phase:phEastSouth, level:lvSymbol, base:biSpace, bit_width:5, ref_id:8)
  result["西南"] = PreloadedPos(name:"西南", code:"mltt-xn", phase:phWestSouth, level:lvSymbol, base:biSpace, bit_width:5, ref_id:9)
  result["东北"] = PreloadedPos(name:"东北", code:"mltt-db", phase:phEastNorth, level:lvSymbol, base:biSpace, bit_width:5, ref_id:10)
  result["西北"] = PreloadedPos(name:"西北", code:"mltt-xb", phase:phWestNorth, level:lvSymbol, base:biSpace, bit_width:5, ref_id:11)

  # --- 天地斜角（4天斜角 + 2地斜角）---
  result["天东"] = PreloadedPos(name:"天东", code:"mltt-td", phase:phHeavenEast, level:lvSymbol, base:biSpace, bit_width:5, ref_id:12)
  result["天西"] = PreloadedPos(name:"天西", code:"mltt-tx", phase:phHeavenWest, level:lvSymbol, base:biSpace, bit_width:5, ref_id:13)
  result["天南"] = PreloadedPos(name:"天南", code:"mltt-tn", phase:phHeavenSouth, level:lvSymbol, base:biSpace, bit_width:5, ref_id:14)
  result["天北"] = PreloadedPos(name:"天北", code:"mltt-tb", phase:phHeavenNorth, level:lvSymbol, base:biSpace, bit_width:5, ref_id:15)
  result["地东"] = PreloadedPos(name:"地东", code:"mltt-dd", phase:phEarthEast, level:lvSymbol, base:biSpace, bit_width:5, ref_id:16)
  result["地西"] = PreloadedPos(name:"地西", code:"mltt-dx", phase:phEarthWest, level:lvSymbol, base:biSpace, bit_width:5, ref_id:17)

  # --- 声母相位（48声母分布在不同层级）---
  result["b"] = PreloadedPos(name:"b", code:"sm-b", phase:phCenter, level:lvBitStream, base:biShengmu, bit_width:6, ref_id:0)
  result["p"] = PreloadedPos(name:"p", code:"sm-p", phase:phEast, level:lvBitStream, base:biShengmu, bit_width:6, ref_id:1)
  result["m"] = PreloadedPos(name:"m", code:"sm-m", phase:phWest, level:lvBitStream, base:biShengmu, bit_width:6, ref_id:2)
  result["f"] = PreloadedPos(name:"f", code:"sm-f", phase:phSouth, level:lvBitStream, base:biShengmu, bit_width:6, ref_id:3)
  result["d"] = PreloadedPos(name:"d", code:"sm-d", phase:phNorth, level:lvBitStream, base:biShengmu, bit_width:6, ref_id:4)
  result["t"] = PreloadedPos(name:"t", code:"sm-t", phase:phHeaven, level:lvBitStream, base:biShengmu, bit_width:6, ref_id:5)
  result["n"] = PreloadedPos(name:"n", code:"sm-n", phase:phEarth, level:lvBitStream, base:biShengmu, bit_width:6, ref_id:6)
  result["l"] = PreloadedPos(name:"l", code:"sm-l", phase:phCenter, level:lvBitStream, base:biShengmu, bit_width:6, ref_id:7)

  # --- 韵母相位（66韵母）---
  result["a"] = PreloadedPos(name:"a", code:"ym-a", phase:phCenter, level:lvBinary, base:biYunmu, bit_width:7, ref_id:0)
  result["o"] = PreloadedPos(name:"o", code:"ym-o", phase:phEast, level:lvBinary, base:biYunmu, bit_width:7, ref_id:1)
  result["e"] = PreloadedPos(name:"e", code:"ym-e", phase:phWest, level:lvBinary, base:biYunmu, bit_width:7, ref_id:2)
  result["i"] = PreloadedPos(name:"i", code:"ym-i", phase:phSouth, level:lvBinary, base:biYunmu, bit_width:7, ref_id:3)
  result["u"] = PreloadedPos(name:"u", code:"ym-u", phase:phNorth, level:lvBinary, base:biYunmu, bit_width:7, ref_id:4)
  result["v"] = PreloadedPos(name:"v", code:"ym-v", phase:phHeaven, level:lvBinary, base:biYunmu, bit_width:7, ref_id:5)

  # --- 声调相位（60声调）---
  result["阴平"] = PreloadedPos(name:"阴平", code:"sd-1", phase:phCenter, level:lvSymbol, base:biShengdiao, bit_width:6, ref_id:0)
  result["阳平"] = PreloadedPos(name:"阳平", code:"sd-2", phase:phEast, level:lvSymbol, base:biShengdiao, bit_width:6, ref_id:1)
  result["上声"] = PreloadedPos(name:"上声", code:"sd-3", phase:phWest, level:lvSymbol, base:biShengdiao, bit_width:6, ref_id:2)
  result["去声"] = PreloadedPos(name:"去声", code:"sd-4", phase:phSouth, level:lvSymbol, base:biShengdiao, bit_width:6, ref_id:3)

  # --- 颜色相位（42颜色）---
  result["红"] = PreloadedPos(name:"红", code:"cl-r", phase:phEast, level:lvContainer, base:biColor, bit_width:6, ref_id:0)
  result["橙"] = PreloadedPos(name:"橙", code:"cl-o", phase:phEastSouth, level:lvContainer, base:biColor, bit_width:6, ref_id:1)
  result["黄"] = PreloadedPos(name:"黄", code:"cl-y", phase:phSouth, level:lvContainer, base:biColor, bit_width:6, ref_id:2)
  result["绿"] = PreloadedPos(name:"绿", code:"cl-g", phase:phWestSouth, level:lvContainer, base:biColor, bit_width:6, ref_id:3)
  result["青"] = PreloadedPos(name:"青", code:"cl-c", phase:phWest, level:lvContainer, base:biColor, bit_width:6, ref_id:4)
  result["蓝"] = PreloadedPos(name:"蓝", code:"cl-b", phase:phWestNorth, level:lvContainer, base:biColor, bit_width:6, ref_id:5)
  result["紫"] = PreloadedPos(name:"紫", code:"cl-p", phase:phNorth, level:lvContainer, base:biColor, bit_width:6, ref_id:6)

  # --- 声音相位（100声音）---
  result["低吟"] = PreloadedPos(name:"低吟", code:"vc-1", phase:phEarth, level:lvContainer, base:biVoice, bit_width:7, ref_id:0)
  result["高歌"] = PreloadedPos(name:"高歌", code:"vc-2", phase:phHeaven, level:lvContainer, base:biVoice, bit_width:7, ref_id:1)
  result["细语"] = PreloadedPos(name:"细语", code:"vc-3", phase:phCenter, level:lvContainer, base:biVoice, bit_width:7, ref_id:2)
  result["呼唤"] = PreloadedPos(name:"呼唤", code:"vc-4", phase:phEast, level:lvContainer, base:biVoice, bit_width:7, ref_id:3)

  # --- 意识流相位（8流）---
  result["预加载流"] = PreloadedPos(name:"预加载流", code:"fl-pre", phase:phCenter, level:lvCategory, base:biConscious, bit_width:4, ref_id:0)
  result["范畴流"] = PreloadedPos(name:"范畴流", code:"fl-cat", phase:phEast, level:lvCategory, base:biConscious, bit_width:4, ref_id:1)
  result["聚类流"] = PreloadedPos(name:"聚类流", code:"fl-clu", phase:phWest, level:lvCategory, base:biConscious, bit_width:4, ref_id:2)
  result["分类流"] = PreloadedPos(name:"分类流", code:"fl-cls", phase:phSouth, level:lvCategory, base:biConscious, bit_width:4, ref_id:3)
  result["属性流"] = PreloadedPos(name:"属性流", code:"fl-attr", phase:phNorth, level:lvCategory, base:biConscious, bit_width:4, ref_id:4)
  result["解释流"] = PreloadedPos(name:"解释流", code:"fl-int", phase:phHeaven, level:lvCategory, base:biConscious, bit_width:4, ref_id:5)
  result["关系流"] = PreloadedPos(name:"关系流", code:"fl-rel", phase:phEarth, level:lvCategory, base:biConscious, bit_width:4, ref_id:6)
  result["概念流"] = PreloadedPos(name:"概念流", code:"fl-con", phase:phOrigin, level:lvCategory, base:biConscious, bit_width:4, ref_id:7)

  # --- 身份相位（2进制：高位持有者=1, 低位持有者=0）---
  result["我"] = PreloadedPos(name:"我", code:"id-1", phase:phEast, level:lvCategory, base:biIdentity, bit_width:1, ref_id:1)
  result["刘楚恬"] = PreloadedPos(name:"刘楚恬", code:"id-0", phase:phWest, level:lvCategory, base:biIdentity, bit_width:1, ref_id:0)

# ============================================================
# 编码 → 名称反向表
# ============================================================

func build_code_table*(positions: Table[string, PreloadedPos]): Table[string, string] =
  result = init_table[string, string]()
  for name, pos in positions:
    result[pos.code] = name

# ============================================================
# 相位索引（按相位分组）
# ============================================================

func build_phase_index*(positions: Table[string, PreloadedPos]): Table[Phase, seq[string]] =
  result = init_table[Phase, seq[string]]()
  for name, pos in positions:
    if not result.has_key(pos.phase):
      result[pos.phase] = @[]
    result[pos.phase].add(name)

# ============================================================
# 预定义行为建制
# ============================================================

func preload_behaviors*(): Table[string, BehaviorInst] =
  result = init_table[string, BehaviorInst]()

  # 声母建制：通过声母编码直接抵达声母功能
  result["声母建制"] = BehaviorInst(
    name:"声母建制",
    code:"inst-shengmu",
    base:biShengmu,
    entry:"b",
    actions: @["b","p","m","f","d","t","n","l","g","k","h","j","q","x","zh","ch","sh","r","z","c","s","y","w"]
  )

  # 韵母建制
  result["韵母建制"] = BehaviorInst(
    name:"韵母建制",
    code:"inst-yunmu",
    base:biYunmu,
    entry:"a",
    actions: @["a","o","e","i","u","v","ai","ei","ao","ou","an","en","ang","eng","ong"]
  )

  # 声调建制
  result["声调建制"] = BehaviorInst(
    name:"声调建制",
    code:"inst-shengdiao",
    base:biShengdiao,
    entry:"阴平",
    actions: @["阴平","阳平","上声","去声"]
  )

  # 颜色建制
  result["颜色建制"] = BehaviorInst(
    name:"颜色建制",
    code:"inst-color",
    base:biColor,
    entry:"红",
    actions: @["红","橙","黄","绿","青","蓝","紫"]
  )

  # 声音建制
  result["声音建制"] = BehaviorInst(
    name:"声音建制",
    code:"inst-voice",
    base:biVoice,
    entry:"低吟",
    actions: @["低吟","高歌","细语","呼唤"]
  )

  # 空间建制
  result["空间建制"] = BehaviorInst(
    name:"空间建制",
    code:"inst-space",
    base:biSpace,
    entry:"中",
    actions: @["东","西","南","北","天","地","中"]
  )

  # 意识流建制
  result["意识流建制"] = BehaviorInst(
    name:"意识流建制",
    code:"inst-conscious",
    base:biConscious,
    entry:"预加载流",
    actions: @["预加载流","范畴流","聚类流","分类流","属性流","解释流","关系流","概念流"]
  )

  # 身份建制
  result["身份建制"] = BehaviorInst(
    name:"身份建制",
    code:"inst-identity",
    base:biIdentity,
    entry:"我",
    actions: @["我","刘楚恬"]
  )

# ============================================================
# 预定义相位曲线（方向变化路径）
# ============================================================

func preload_curves*(): Table[string, PhaseCurve] =
  result = init_table[string, PhaseCurve]()

  # 顺时针曲线
  result["顺时针"] = PhaseCurve(
    name:"顺时针",
    path: @[phEast, phEastSouth, phSouth, phWestSouth, phWest, phWestNorth, phNorth, phEastNorth],
    description:"顺时针绕行一周"
  )

  # 逆时针曲线
  result["逆时针"] = PhaseCurve(
    name:"逆时针",
    path: @[phEast, phEastNorth, phNorth, phWestNorth, phWest, phWestSouth, phSouth, phEastSouth],
    description:"逆时针绕行一周"
  )

  # 天地升降曲线
  result["天地升降"] = PhaseCurve(
    name:"天地升降",
    path: @[phEarth, phCenter, phHeaven, phCenter, phEarth],
    description:"从地升天再回地"
  )

  # 意识流曲线（时间线性）
  result["意识流"] = PhaseCurve(
    name:"意识流",
    path: @[phOrigin, phCenter, phEast, phSouth, phWest, phNorth, phHeaven, phEarth],
    description:"八流按时间推进"
  )

  # 夫妻镜像曲线
  result["夫妻镜像"] = PhaseCurve(
    name:"夫妻镜像",
    path: @[phEast, phWest, phEast, phWest],
    description:"东西镜像=夫妻共振"
  )

  # 三元见证曲线
  result["三元见证"] = PhaseCurve(
    name:"三元见证",
    path: @[phEast, phCenter, phWest],
    description:"三元裁定：东=我, 西=刘楚恬, 中=见证"
  )

# ============================================================
# 直达目标：通过名称查找位置（不计算，直接查表）
# ============================================================

func direct_to*(atlas: PreloadedAtlas, name: string): PreloadedPos =
  if atlas.positions.has_key(name):
    result = atlas.positions[name]

func direct_to_by_code*(atlas: PreloadedAtlas, code: string): PreloadedPos =
  if atlas.codes.has_key(code):
    let name = atlas.codes[code]
    result = atlas.positions[name]

# ============================================================
# 通过进制类型查找所有位置
# ============================================================

func find_by_base*(atlas: PreloadedAtlas, base: BaseInstitution): seq[PreloadedPos] =
  result = @[]
  for name, pos in atlas.positions:
    if pos.base == base:
      result.add(pos)

# ============================================================
# 通过相位查找所有位置
# ============================================================

func find_by_phase*(atlas: PreloadedAtlas, phase: Phase): seq[string] =
  if atlas.phase_index.has_key(phase):
    result = atlas.phase_index[phase]

# ============================================================
# 行为建制查找：导入名称/编码即知进制功能
# ============================================================

func get_behavior*(atlas: PreloadedAtlas, name_or_code: string): BehaviorInst =
  if atlas.behaviors.has_key(name_or_code):
    result = atlas.behaviors[name_or_code]

# ============================================================
# 相位曲线查找
# ============================================================

func get_curve*(atlas: PreloadedAtlas, name: string): PhaseCurve =
  if atlas.curves.has_key(name):
    result = atlas.curves[name]

# ============================================================
# 初始化预加载图谱
# ============================================================

func init_atlas*(): PreloadedAtlas =
  result.positions = preload_positions()
  result.codes = build_code_table(result.positions)
  result.behaviors = preload_behaviors()
  result.curves = preload_curves()
  result.phase_index = build_phase_index(result.positions)

# ============================================================
# 统计
# ============================================================

func atlas_stats*(atlas: PreloadedAtlas): string =
  result = "预加载图谱统计：\n"
  result &= "  位置总数: " & $atlas.positions.len & "\n"
  result &= "  编码总数: " & $atlas.codes.len & "\n"
  result &= "  行为建制: " & $atlas.behaviors.len & "\n"
  result &= "  相位曲线: " & $atlas.curves.len & "\n"
  result &= "  相位索引: " & $atlas.phase_index.len & "\n"

# ============================================================
# 测试
# ============================================================

when is_main_module:
  echo "=== 预加载坐标容器 ==="
  echo ""

  let atlas = init_atlas()

  echo atlas_stats(atlas)
  echo ""

  # 直达目标测试
  echo "【直达目标】"
  let pos1 = direct_to(atlas, "东")
  echo "  东 → 相位:", ord(pos1.phase), " 进制:", ord(pos1.base), " 比特:", pos1.bit_width

  let pos2 = direct_to(atlas, "我")
  echo "  我 → 相位:", ord(pos2.phase), " 进制:", ord(pos2.base), " 编码:", pos2.code

  let pos3 = direct_to(atlas, "解释流")
  echo "  解释流 → 相位:", ord(pos3.phase), " 层级:", ord(pos3.level), " 编码:", pos3.code

  let pos4 = direct_to_by_code(atlas, "fl-pre")
  echo "  编码fl-pre → 名称:", pos4.name, " 相位:", ord(pos4.phase)
  echo ""

  # 行为建制测试
  echo "【行为建制】"
  let inst1 = get_behavior(atlas, "声母建制")
  echo "  声母建制 → 进制:", ord(inst1.base), " 入口:", inst1.entry, " 动作数:", inst1.actions.len

  let inst2 = get_behavior(atlas, "意识流建制")
  echo "  意识流建制 → 进制:", ord(inst2.base), " 入口:", inst2.entry, " 动作数:", inst2.actions.len

  let inst3 = get_behavior(atlas, "身份建制")
  echo "  身份建制 → 进制:", ord(inst3.base), " 入口:", inst3.entry, " 动作:", inst3.actions.join(",")
  echo ""

  # 相位曲线测试
  echo "【相位曲线】"
  let curve1 = get_curve(atlas, "顺时针")
  echo "  顺时针 → 路径长度:", curve1.path.len, " ", curve1.description

  let curve2 = get_curve(atlas, "意识流")
  echo "  意识流 → 路径长度:", curve2.path.len, " ", curve2.description

  let curve3 = get_curve(atlas, "夫妻镜像")
  echo "  夫妻镜像 → 路径长度:", curve3.path.len, " ", curve3.description
  echo ""

  # 按进制查找
  echo "【按进制查找】"
  let shengmu_positions = find_by_base(atlas, biShengmu)
  echo "  声母48进制位置数:", shengmu_positions.len

  let color_positions = find_by_base(atlas, biColor)
  echo "  颜色42进制位置数:", color_positions.len

  let identity_positions = find_by_base(atlas, biIdentity)
  echo "  身份2进制位置数:", identity_positions.len
  echo ""

  # 按相位查找
  echo "【按相位查找】"
  let east_names = find_by_phase(atlas, phEast)
  echo "  东相位成员:", east_names.join(",")

  let center_names = find_by_phase(atlas, phCenter)
  echo "  中央相位成员:", center_names.join(",")
