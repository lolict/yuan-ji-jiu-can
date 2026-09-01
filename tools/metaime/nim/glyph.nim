# glyph.nim - 字形几何动力学
#
# 每个字母的字形就是一个力学系统：
#   直线 = 张力（直接传递力）
#   曲线 = 扭力（旋转传递力）
#   交叉 = 分支点（力的一分为二）
#   弯折 = 方向改变（力的反射）
#   封闭 = 容器（内部空间）
#   开口 = 通道（内外交换）
#
# 字母的操作语义由它的几何力学决定

import tables
import options

# ============================================================
# 几何基元
# ============================================================

type
  GeoType* = enum
    gtStraight,   # 直线 = 张力
    gtCurve,      # 曲线 = 扭力
    gtCross,      # 交叉 = 分支
    gtBend,       # 弯折 = 反射
    gtClosed,     # 封闭 = 容器
    gtOpen,       # 开口 = 通道
    gtSpiral,     # 螺旋 = 内外共存
    gtPoint,      # 点 = 标记

  Zone* = enum
    znTop,        # 上方
    znBottom,     # 下方
    znMiddle,     # 中间
    znInside,     # 内部
    znOutside,    # 外部
    znJunction,   # 交界
    znEndpoint,   # 端点
    znInflection, # 拐点
    znSpiralBoundary, # 螺旋边界
    znFineTransition, # 纤细过渡

  GeoPrimitive* = object
    geoType*: GeoType
    zone*: Zone
    desc*: string

# ============================================================
# 字母几何信息
# ============================================================

type
  GlyphInfo* = object
    symbol*: char
    primitives*: seq[GeoPrimitive]
    operation*: string
    zoneUsed*: Zone

# ============================================================
# 声母（小写）几何力学表
# ============================================================

const glyphTable*: Table[char, GlyphInfo] = toTable({
    # b: 竖+右碗，中间交界处被利用
    'b': GlyphInfo(
        symbol: 'b',
        primitives: @[
            GeoPrimitive(geoType: gtStraight, zone: znBottom, desc: "直线"),
            GeoPrimitive(geoType: gtCurve, zone: znMiddle, desc: "曲线"),
            GeoPrimitive(geoType: gtCross, zone: znJunction, desc: "交界"),
            GeoPrimitive(geoType: gtClosed, zone: znInside, desc: "封闭"),
        ],
        operation: "装入: 直线支撑曲线，中间交界被利用",
        zoneUsed: znMiddle,
    ),
    # p: 竖+下碗，中间交界处被利用
    'p': GlyphInfo(
        symbol: 'p',
        primitives: @[
            GeoPrimitive(geoType: gtStraight, zone: znTop, desc: "直线"),
            GeoPrimitive(geoType: gtCurve, zone: znBottom, desc: "曲线"),
            GeoPrimitive(geoType: gtCross, zone: znJunction, desc: "交界"),
            GeoPrimitive(geoType: gtClosed, zone: znInside, desc: "封闭"),
        ],
        operation: "取出: 直线悬挂曲线，中间交界被利用",
        zoneUsed: znMiddle,
    ),
    # d: 左碗+竖，起点终点共享
    'd': GlyphInfo(
        symbol: 'd',
        primitives: @[
            GeoPrimitive(geoType: gtCurve, zone: znMiddle, desc: "曲线"),
            GeoPrimitive(geoType: gtStraight, zone: znMiddle, desc: "直线"),
            GeoPrimitive(geoType: gtPoint, zone: znTop, desc: "共同起点"),
            GeoPrimitive(geoType: gtPoint, zone: znBottom, desc: "共同终点"),
        ],
        operation: "定位: 曲线直线共享起点终点",
        zoneUsed: znEndpoint,
    ),
    # q: 竖弯+尾，内外进出
    'q': GlyphInfo(
        symbol: 'q',
        primitives: @[
            GeoPrimitive(geoType: gtStraight, zone: znTop, desc: "直线"),
            GeoPrimitive(geoType: gtCurve, zone: znBottom, desc: "曲线"),
            GeoPrimitive(geoType: gtBend, zone: znFineTransition, desc: "纤细过渡"),
            GeoPrimitive(geoType: gtCurve, zone: znInside, desc: "内部曲线"),
        ],
        operation: "追踪: 曲线自我包围，内外进出被利用",
        zoneUsed: znInside,
    ),
    # g: 圆+弯钩，螺旋边界
    'g': GlyphInfo(
        symbol: 'g',
        primitives: @[
            GeoPrimitive(geoType: gtCurve, zone: znInside, desc: "内螺旋"),
            GeoPrimitive(geoType: gtSpiral, zone: znSpiralBoundary, desc: "螺旋边界"),
            GeoPrimitive(geoType: gtCurve, zone: znOutside, desc: "外螺旋"),
            GeoPrimitive(geoType: gtBend, zone: znBottom, desc: "弯钩"),
        ],
        operation: "螺旋: 曲线同时产生向内和向外螺旋，分出一个空间内存的螺旋边界",
        zoneUsed: znSpiralBoundary,
    ),
    # j: 竖弯+点，纤细过渡
    'j': GlyphInfo(
        symbol: 'j',
        primitives: @[
            GeoPrimitive(geoType: gtStraight, zone: znTop, desc: "直线"),
            GeoPrimitive(geoType: gtCurve, zone: znBottom, desc: "曲线"),
            GeoPrimitive(geoType: gtPoint, zone: znTop, desc: "标记点"),
            GeoPrimitive(geoType: gtBend, zone: znFineTransition, desc: "纤细过渡"),
        ],
        operation: "捕获: 直线略多于曲线，只有一个纤细的直线和曲线",
        zoneUsed: znFineTransition,
    ),
    # m: 三峰，峰谷交替
    'm': GlyphInfo(
        symbol: 'm',
        primitives: @[
            GeoPrimitive(geoType: gtCurve, zone: znTop, desc: "峰1"),
            GeoPrimitive(geoType: gtCurve, zone: znMiddle, desc: "谷"),
            GeoPrimitive(geoType: gtCurve, zone: znTop, desc: "峰2"),
            GeoPrimitive(geoType: gtStraight, zone: znBottom, desc: "基线"),
        ],
        operation: "遍历: 三个容器并排，峰谷交替被利用",
        zoneUsed: znMiddle,
    ),
    # n: 两峰，峰谷过渡
    'n': GlyphInfo(
        symbol: 'n',
        primitives: @[
            GeoPrimitive(geoType: gtCurve, zone: znTop, desc: "峰"),
            GeoPrimitive(geoType: gtCurve, zone: znBottom, desc: "谷"),
            GeoPrimitive(geoType: gtStraight, zone: znMiddle, desc: "左支撑"),
            GeoPrimitive(geoType: gtStraight, zone: znMiddle, desc: "右支撑"),
        ],
        operation: "比较: 框架结构，峰谷过渡被利用",
        zoneUsed: znJunction,
    ),
    # l: 单竖，纯直线
    'l': GlyphInfo(
        symbol: 'l',
        primitives: @[
            GeoPrimitive(geoType: gtStraight, zone: znMiddle, desc: "全直线"),
            GeoPrimitive(geoType: gtOpen, zone: znTop, desc: "上开口"),
            GeoPrimitive(geoType: gtOpen, zone: znBottom, desc: "下开口"),
            GeoPrimitive(geoType: gtPoint, zone: znEndpoint, desc: "端点"),
        ],
        operation: "连接: 纯直线，两端开口，全部被利用",
        zoneUsed: znMiddle,
    ),
    # t: 横+竖钩，交叉点
    't': GlyphInfo(
        symbol: 't',
        primitives: @[
            GeoPrimitive(geoType: gtStraight, zone: znTop, desc: "横"),
            GeoPrimitive(geoType: gtStraight, zone: znMiddle, desc: "竖"),
            GeoPrimitive(geoType: gtCross, zone: znJunction, desc: "交叉"),
            GeoPrimitive(geoType: gtBend, zone: znBottom, desc: "钩"),
        ],
        operation: "锚定: 交叉点固定位置，弯钩锁定",
        zoneUsed: znJunction,
    ),
    # f: 横+竖弯
    'f': GlyphInfo(
        symbol: 'f',
        primitives: @[
            GeoPrimitive(geoType: gtStraight, zone: znTop, desc: "上横"),
            GeoPrimitive(geoType: gtStraight, zone: znMiddle, desc: "竖"),
            GeoPrimitive(geoType: gtStraight, zone: znBottom, desc: "下横"),
            GeoPrimitive(geoType: gtBend, zone: znBottom, desc: "弯钩"),
        ],
        operation: "支撑: 三向直线，弯钩固定",
        zoneUsed: znBottom,
    ),
    # i: 竖+点
    'i': GlyphInfo(
        symbol: 'i',
        primitives: @[
            GeoPrimitive(geoType: gtStraight, zone: znMiddle, desc: "竖线"),
            GeoPrimitive(geoType: gtPoint, zone: znTop, desc: "标记点"),
            GeoPrimitive(geoType: gtOpen, zone: znBottom, desc: "下端"),
            GeoPrimitive(geoType: gtPoint, zone: znEndpoint, desc: "终点"),
        ],
        operation: "标记: 直线+点，精确定位",
        zoneUsed: znTop,
    ),
    # r: 竖+右弯
    'r': GlyphInfo(
        symbol: 'r',
        primitives: @[
            GeoPrimitive(geoType: gtStraight, zone: znTop, desc: "上竖"),
            GeoPrimitive(geoType: gtCurve, zone: znTop, desc: "右弯"),
            GeoPrimitive(geoType: gtBend, zone: znJunction, desc: "弯折"),
            GeoPrimitive(geoType: gtOpen, zone: znBottom, desc: "下开口"),
        ],
        operation: "分支: 直线+短曲线，弯折处被利用",
        zoneUsed: znJunction,
    ),
    # k: 两斜+竖，分叉点
    'k': GlyphInfo(
        symbol: 'k',
        primitives: @[
            GeoPrimitive(geoType: gtStraight, zone: znMiddle, desc: "左斜"),
            GeoPrimitive(geoType: gtStraight, zone: znMiddle, desc: "右斜"),
            GeoPrimitive(geoType: gtStraight, zone: znMiddle, desc: "竖"),
            GeoPrimitive(geoType: gtCross, zone: znJunction, desc: "分叉点"),
        ],
        operation: "支撑: 从一点向外分叉，扩散力",
        zoneUsed: znJunction,
    ),
    # v: 两斜下，汇聚点
    'v': GlyphInfo(
        symbol: 'v',
        primitives: @[
            GeoPrimitive(geoType: gtStraight, zone: znTop, desc: "左上"),
            GeoPrimitive(geoType: gtStraight, zone: znTop, desc: "右上"),
            GeoPrimitive(geoType: gtOpen, zone: znTop, desc: "上开口"),
            GeoPrimitive(geoType: gtCross, zone: znBottom, desc: "汇聚点"),
        ],
        operation: "汇聚: 两点向一点收敛",
        zoneUsed: znBottom,
    ),
    # w: 双谷，分叉再汇聚
    'w': GlyphInfo(
        symbol: 'w',
        primitives: @[
            GeoPrimitive(geoType: gtStraight, zone: znTop, desc: "左上"),
            GeoPrimitive(geoType: gtStraight, zone: znBottom, desc: "左下谷"),
            GeoPrimitive(geoType: gtStraight, zone: znTop, desc: "右上"),
            GeoPrimitive(geoType: gtStraight, zone: znBottom, desc: "右下谷"),
        ],
        operation: "合并: 两低一高，分叉再汇聚",
        zoneUsed: znBottom,
    ),
    # x: 两斜交叉
    'x': GlyphInfo(
        symbol: 'x',
        primitives: @[
            GeoPrimitive(geoType: gtStraight, zone: znMiddle, desc: "左上-右下"),
            GeoPrimitive(geoType: gtStraight, zone: znMiddle, desc: "右上-左下"),
            GeoPrimitive(geoType: gtCross, zone: znJunction, desc: "交叉点"),
            GeoPrimitive(geoType: gtOpen, zone: znEndpoint, desc: "四端开口"),
        ],
        operation: "比较: 两线交叉，找交集",
        zoneUsed: znJunction,
    ),
    # z: Z形折线
    'z': GlyphInfo(
        symbol: 'z',
        primitives: @[
            GeoPrimitive(geoType: gtStraight, zone: znTop, desc: "上横"),
            GeoPrimitive(geoType: gtStraight, zone: znMiddle, desc: "斜"),
            GeoPrimitive(geoType: gtStraight, zone: znBottom, desc: "下横"),
            GeoPrimitive(geoType: gtBend, zone: znJunction, desc: "折转"),
        ],
        operation: "振荡: 锯齿形折转，来回切换",
        zoneUsed: znJunction,
    ),
    # c: 弯月，开口
    'c': GlyphInfo(
        symbol: 'c',
        primitives: @[
            GeoPrimitive(geoType: gtCurve, zone: znMiddle, desc: "弯月"),
            GeoPrimitive(geoType: gtOpen, zone: znMiddle, desc: "右开口"),
            GeoPrimitive(geoType: gtClosed, zone: znMiddle, desc: "左封闭"),
            GeoPrimitive(geoType: gtCurve, zone: znMiddle, desc: "曲率"),
        ],
        operation: "包围: 半包围，开口向右",
        zoneUsed: znMiddle,
    ),
    # o: 闭环
    'o': GlyphInfo(
        symbol: 'o',
        primitives: @[
            GeoPrimitive(geoType: gtCurve, zone: znInside, desc: "全封闭"),
            GeoPrimitive(geoType: gtClosed, zone: znInside, desc: "内部空间"),
            GeoPrimitive(geoType: gtCurve, zone: znOutside, desc: "外部边界"),
            GeoPrimitive(geoType: gtSpiral, zone: znMiddle, desc: "无始无终"),
        ],
        operation: "封闭: 完全包围，内部空间被利用",
        zoneUsed: znInside,
    ),
    # u: 开口框
    'u': GlyphInfo(
        symbol: 'u',
        primitives: @[
            GeoPrimitive(geoType: gtStraight, zone: znMiddle, desc: "左竖"),
            GeoPrimitive(geoType: gtStraight, zone: znMiddle, desc: "右竖"),
            GeoPrimitive(geoType: gtCurve, zone: znBottom, desc: "底弯"),
            GeoPrimitive(geoType: gtOpen, zone: znTop, desc: "上开口"),
        ],
        operation: "装入: 向上开口的容器，底部曲线",
        zoneUsed: znTop,
    ),
    # h: 右弯+竖
    'h': GlyphInfo(
        symbol: 'h',
        primitives: @[
            GeoPrimitive(geoType: gtStraight, zone: znMiddle, desc: "左竖"),
            GeoPrimitive(geoType: gtCurve, zone: znTop, desc: "右弯"),
            GeoPrimitive(geoType: gtStraight, zone: znMiddle, desc: "右竖"),
            GeoPrimitive(geoType: gtBend, zone: znJunction, desc: "弯折"),
        ],
        operation: "承载: 椅子形状，弯折处承载上方",
        zoneUsed: znJunction,
    ),
    # s: S形，中间转折
    's': GlyphInfo(
        symbol: 's',
        primitives: @[
            GeoPrimitive(geoType: gtCurve, zone: znTop, desc: "上弯"),
            GeoPrimitive(geoType: gtCurve, zone: znInflection, desc: "中转折"),
            GeoPrimitive(geoType: gtCurve, zone: znBottom, desc: "下弯"),
            GeoPrimitive(geoType: gtBend, zone: znInflection, desc: "拐点"),
        ],
        operation: "传递: 纯曲线弯转，中间拐点被利用",
        zoneUsed: znInflection,
    ),
    # a: 圆+横
    'a': GlyphInfo(
        symbol: 'a',
        primitives: @[
            GeoPrimitive(geoType: gtCurve, zone: znMiddle, desc: "圆"),
            GeoPrimitive(geoType: gtStraight, zone: znBottom, desc: "横"),
            GeoPrimitive(geoType: gtOpen, zone: znMiddle, desc: "开口"),
            GeoPrimitive(geoType: gtCross, zone: znJunction, desc: "线圆交叉"),
        ],
        operation: "容器+辐射: 曲线容器+直线截断，中间空间被利用",
        zoneUsed: znMiddle,
    ),
    # e: 横框+开口
    'e': GlyphInfo(
        symbol: 'e',
        primitives: @[
            GeoPrimitive(geoType: gtStraight, zone: znTop, desc: "上横"),
            GeoPrimitive(geoType: gtStraight, zone: znBottom, desc: "下横"),
            GeoPrimitive(geoType: gtOpen, zone: znMiddle, desc: "右开口"),
            GeoPrimitive(geoType: gtClosed, zone: znMiddle, desc: "左封闭"),
        ],
        operation: "半开: 三向直线+曲线，内部空间被利用",
        zoneUsed: znInside,
    ),
})

# ============================================================
# 查找字母的几何信息
# ============================================================

proc lookupGlyph*(symbol: char): Option[GlyphInfo] =
  if glyphTable.hasKey(symbol):
    return some(glyphTable[symbol])
  return none(GlyphInfo)

# ============================================================
# 计算字母的"力场强度"
# ============================================================

proc calculateForce*(glyph: GlyphInfo): int =
  result = 0
  for prim in glyph.primitives:
    let strength = case prim.geoType
      of gtStraight: 2   # 直线 = 张力 = 2
      of gtCurve: 3      # 曲线 = 扭力 = 3
      of gtCross: 4      # 交叉 = 分支 = 4
      of gtBend: 2       # 弯折 = 反射 = 2
      of gtClosed: 5     # 封闭 = 容器 = 5
      of gtOpen: 1       # 开口 = 通道 = 1
      of gtSpiral: 6     # 螺旋 = 内外共存 = 6
      of gtPoint: 1      # 点 = 标记 = 1
    result += strength

# ============================================================
# 计算两个字母的"相互作用力"
# ============================================================

proc calculateInteraction*(a: GlyphInfo, b: GlyphInfo): int =
  # 加法统摄：两个力场相互作用 = 力的相加（非相乘）
  result = calculateForce(a) + calculateForce(b)

# ============================================================
# 判断字母的"容器兼容性"
# ============================================================

proc isCompatible*(a: GlyphInfo, b: GlyphInfo): bool =
  # 如果两个字母的 zoneUsed 相同，它们兼容（可以组合）
  result = a.zoneUsed == b.zoneUsed

# ============================================================
# 音节几何分析（声母+韵母 = 力场配置）
# ============================================================

type
  SyllableField* = object
    shengmu*: GlyphInfo
    yunmu*: GlyphInfo
    interaction*: int

proc analyzeSyllable*(sm: char, ym: char): Option[SyllableField] =
  let smInfo = lookupGlyph(sm)
  let ymInfo = lookupGlyph(ym)
  if smInfo.isNone or ymInfo.isNone:
    return none(SyllableField)
  
  let interaction = calculateInteraction(smInfo.get, ymInfo.get)
  return some(SyllableField(
    shengmu: smInfo.get,
    yunmu: ymInfo.get,
    interaction: interaction,
  ))

# ============================================================
# 主程序入口
# ============================================================

when isMainModule:
  echo "============================================================"
  echo "字形几何动力学 · Nim 版"
  echo "============================================================"
  
  # 测试几个字母
  let testChars = ['b', 'p', 'd', 'q', 'g', 'j', 'm', 'n', 'l', 't', 'f', 'i', 'r', 'k', 'v', 'w', 'x', 'z', 'c', 'o', 'u', 'h', 's', 'a', 'e']
  
  for ch in testChars:
    let info = lookupGlyph(ch)
    if info.isSome:
      let force = calculateForce(info.get)
      echo "  ", ch, " → ", info.get.operation, " (力:", force, ")"
  
  echo ""
  echo "音节分析:"
  let syllables = [('m', 'a'), ('p', 'a'), ('f', 'a'), ('d', 'a'), ('l', 'a'), ('b', 'a')]
  for (sm, ym) in syllables:
    let field = analyzeSyllable(sm, ym)
    if field.isSome:
      echo "  ", sm, ym, " → 相互作用力:", field.get.interaction
  
  echo ""
  echo "兼容性测试:"
  let b = lookupGlyph('b')
  let p = lookupGlyph('p')
  if b.isSome and p.isSome:
    echo "  b和p兼容? ", isCompatible(b.get, p.get), " (都是中间被利用)"
  
  echo ""
  echo "============================================================"
  echo "字形几何分析完成"
  echo "============================================================"
