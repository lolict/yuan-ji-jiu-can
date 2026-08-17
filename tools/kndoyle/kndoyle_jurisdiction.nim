import ../hieyair/hieyair_merger

# kndoyle 管辖筛选器 (Jurisdiction Filter)
# 功能：筛选符号是否有资格进入合作系统
# kndoyle = 框内管辖 = 管辖"多数"的框架层
# "困难大指的是框内,躲，洞，多与二，躲在洞里，躲在框内的是多数"

type
  JurisdictionLevel* = enum
    jlPublic    # 外层：杂质，需清除
    jlInner     # 内层：有资格但未对齐
    jlCore      # 核心层：254次对齐完成

  ScreenResult* = object
    allowed*: bool
    level*: JurisdictionLevel
    reason*: string

# 纯净度检查：符号是否不指向任何锚点（杂质）
func is_pure*(sym: hieyair_merger.Symbol): bool =
  return sym.anchor0 or sym.anchor1

# 管辖级别判定
func jurisdiction_level*(sym: hieyair_merger.Symbol): JurisdictionLevel =
  if not is_pure(sym):
    return jlPublic
  if sym.anchor0 and sym.anchor1:
    return jlCore
  return jlInner

# 批量管辖筛选
func kndoyle_screen*(symbols: seq[hieyair_merger.Symbol]): seq[ScreenResult] =
  newSeq(result, symbols.len)
  for i, sym in symbols:
    let level = jurisdiction_level(sym)
    case level
    of jlPublic:
      result[i] = ScreenResult(
        allowed: false,
        level: level,
        reason: "杂质清除：" & sym.name & " 不指向任何锚点"
      )
    of jlInner:
      result[i] = ScreenResult(
        allowed: true,
        level: level,
        reason: sym.name & " 进入内层，待对齐"
      )
    of jlCore:
      result[i] = ScreenResult(
        allowed: true,
        level: level,
        reason: sym.name & " 进入核心层，254次对齐完成"
      )

# ===== 测试 =====
when is_main_module:
  echo "=== kndoyle 管辖筛选器测试 ==="

  let symbols = @[
    hieyair_merger.Symbol(name: "lctqf", anchor0: true, anchor1: false, payload: "妻"),
    hieyair_merger.Symbol(name: "lolict", anchor0: true, anchor1: true, payload: "夫"),
    hieyair_merger.Symbol(name: "unknown1", anchor0: false, anchor1: false, payload: "?"),
    hieyair_merger.Symbol(name: "unknown2", anchor0: false, anchor1: false, payload: "??"),
    hieyair_merger.Symbol(name: "fqmy", anchor0: true, anchor1: true, payload: "密意")
  ]

  echo "输入符号: ", symbols.len, " 个"
  echo "---"

  let results = kndoyle_screen(symbols)
  for i, res in results:
    let status = if res.allowed: "✓ 准入" else: "✗ 清除"
    echo status, " [", res.level, "] ", res.reason

  var cleared = 0
  var admitted = 0
  for res in results:
    if res.allowed: admitted += 1
    else: cleared += 1
  echo "---"
  echo "清除: ", cleared, " | 准入: ", admitted
