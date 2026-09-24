#==============================================================
#  活码编译器引擎 — LiveCode Compiler
#
#  核心公理：起点钥匙激活活码 · 未固定任意活动 ·
#           固定后加法统计锁定权限与数值
#==============================================================

import std/[tables, strutils, sequtils, math, algorithm]

#--------------------------------------------------------------
#  类型定义
#--------------------------------------------------------------

type
  # 锁状态
  锁状态* = enum
    未锁定
    已锁定

  # 坐标系（对齐64容器族）
  坐标系* = enum
    极坐标系    # 角度+半径
    直角坐标系  # x+y+z
    族槽坐标系  # 8族×8槽

  # 活码单元 — 可活动的编码个体
  活码单元* = object
    名字*: string
    编码*: int              # 容器编码
    x*, y*, z*: float       # 三维坐标（活动时自由浮动）
    固定坐标*: tuple[x, y, z: float]  # 锁定后的固定坐标
    锁*: 锁状态
    权限*: int              # 加法统计产生的权限值
    数值*: int              # 加法统计产生的数值
    声母*: string           # 用于谐音路由
    韵母*: string
    声调*: int
    极性*: uint8

  # 起点钥匙 — 激活活码的触发条件
  起点钥匙* = object
    编码*: int
    坐标*: tuple[x, y, z: float]
    激活圈*: float         # 激活半径
    激活*: bool

  # 活码编译器
  活码编译器* = object
    活码表*: Table[int, 活码单元]     # 编码→活码
    钥匙*: 起点钥匙
    统计次数*: int
    锁定总数*: int

#--------------------------------------------------------------
#  初始化
#--------------------------------------------------------------

proc 初始化活码编译器*(): 活码编译器 =
  result.钥匙 = 起点钥匙(
    编码: 0x22,  # "我"的编码区域
    坐标: (0.0, 0.0, 0.0),
    激活圈: 100.0,
    激活: false
  )
  result.统计次数 = 0
  result.锁定总数 = 0

#--------------------------------------------------------------
#  注册活码
#--------------------------------------------------------------

proc 注册活码*(c: var 活码编译器; 名字, 声母, 韵母: string;
              声调: int; 编码: int; 极性: uint8;
              初始x, 初始y, 初始z: float) =
  c.活码表[编码] = 活码单元(
    名字: 名字,
    编码: 编码,
    x: 初始x, y: 初始y, z: 初始z,
    固定坐标: (0.0, 0.0, 0.0),
    锁: 未锁定,
    权限: 0, 数值: 0,
    声母: 声母, 韵母: 韵母, 声调: 声调,
    极性: 极性
  )

#--------------------------------------------------------------
#  起点激活
#--------------------------------------------------------------

proc 起点激活*(c: var 活码编译器; 编码: int): bool =
  if not c.活码表.hasKey(编码):
    return false

  c.钥匙.编码 = 编码
  c.钥匙.激活 = true
  let target = c.活码表[编码]
  c.钥匙.坐标 = (target.x, target.y, target.z)

  # 激活圈内的所有活码被唤醒
  var activated = 0
  for k, unit in c.活码表:
    let dx = unit.x - c.钥匙.坐标.x
    let dy = unit.y - c.钥匙.坐标.y
    let dz = unit.z - c.钥匙.坐标.z
    let dist = sqrt(dx * dx + dy * dy + dz * dz)
    if dist <= c.钥匙.激活圈:
      activated += 1

  return activated > 1

#--------------------------------------------------------------
#  活动 — 未固定时位置任意浮动（微调）
#--------------------------------------------------------------

proc 自由活动*(c: var 活码编译器; 编码: int;
              偏移x, 偏移y, 偏移z: float) =
  if not c.活码表.hasKey(编码):
    return

  let unit = c.活码表[编码]
  if unit.锁 == 已锁定:
    return  # 已锁定的不可活动

  c.活码表[编码].x += 偏移x
  c.活码表[编码].y += 偏移y
  c.活码表[编码].z += 偏移z

#--------------------------------------------------------------
#  加法统计锁定 — 核心
#  锁定后：按坐标排列顺序 + 加法统计 → 权限与数值
#--------------------------------------------------------------

proc 加法统计*(vals: seq[float]): int =
  # 纯加法累加取整（不乘不加权，仅统计清点）
  var total = 0.0
  for v in vals:
    total += v
  return int(total) and 0x3F  # 限6位=0-63（契合64容器）

proc 锁固定*(c: var 活码编译器; 编码: int): bool =
  if not c.活码表.hasKey(编码):
    return false

  let unit = c.活码表[编码]
  if unit.锁 == 已锁定:
    return true

  c.统计次数 += 1

  # 固定当前坐标为固定坐标
  c.活码表[编码].固定坐标 = (unit.x, unit.y, unit.z)

  # 坐标系选择（基于极性）
  var 坐标值: seq[float] = @[]
  if unit.极性 == 1:
    # 阳 → 直角坐标 x+y+z
    坐标值 = @[unit.x, unit.y, unit.z]
  else:
    # 阴 → 极坐标 r+theta
    let r = sqrt(unit.x * unit.x + unit.y * unit.y + unit.z * unit.z)
    let theta = arctan2(unit.y, unit.x)
    坐标值 = @[r, theta, float(unit.声调)]

  # 加声调作为第三维度（兼顾阴阳统一）
  坐标值.add(float(unit.声调))

  # 声母序（字母表顺序转数值）
  var 声母序 = 0
  if unit.声母.len > 0:
    声母序 = int(unit.声母[0]) mod 24
  坐标值.add(float(声母序))

  # 韵母序
  var 韵母序 = 0
  if unit.韵母.len > 0:
    韵母序 = int(unit.韵母[0]) mod 24
  坐标值.add(float(韵母序))

  # 加法统计 → 权限（6位）
  let 权限 = 加法统计(坐标值)
  c.活码表[编码].权限 = 权限

  # 数值 = 编码与权限的加法组合
  c.活码表[编码].数值 = (unit.编码 + 权限) and 0xFFFFFFFF

  # 锁定
  c.活码表[编码].锁 = 已锁定
  c.锁定总数 += 1

  return true

#--------------------------------------------------------------
#  谐音路由（对齐聂小倩匹配）
#--------------------------------------------------------------

proc 谐音寻码*(c: var 活码编译器; 声母, 韵母: string): seq[int] =
  var result_seq: seq[int] = @[]
  for k, unit in c.活码表:
    if unit.声母 == 声母 and unit.韵母 == 韵母:
      result_seq.add(k)
  return result_seq

#--------------------------------------------------------------
#  权限校验
#--------------------------------------------------------------

proc 校验权限*(c: 活码编译器; 编码: int; 需权限: int): bool =
  if not c.活码表.hasKey(编码):
    return false
  let unit = c.活码表[编码]
  if unit.锁 == 未锁定:
    return false  # 必须先锁定
  return unit.权限 >= 需权限

#--------------------------------------------------------------
#  报告
#--------------------------------------------------------------

proc 系统报告*(c: 活码编译器): string =
  var lines: seq[string] = @[]
  lines.add repeat("═", 50)
  lines.add "  活 码 编 译 器 系 统 报 告"
  lines.add repeat("═", 50)
  lines.add "起点编码: 0x" & toHex(c.钥匙.编码, 2)
  lines.add "激活状态: " & (if c.钥匙.激活: "已激活"  else: "未激活")
  lines.add "激活圈: " & $c.钥匙.激活圈
  lines.add "活码数: " & $c.活码表.len
  lines.add "锁定数: " & $c.锁定总数
  lines.add "统计次数: " & $c.统计次数
  lines.add ""

  lines.add "─ 活码明细 ─"
  for k in sorted(toSeq(c.活码表.keys)):
    let u = c.活码表[k]
    let 锁符 = if u.锁 == 已锁定: "◎" else: "◎"
    let 极性符 = if u.极性 == 1: "阳" else: "阴"
    lines.add "  " & 锁符 & " [0x" & toHex(u.编码, 4) & "] " & u.名字 &
             " " & u.声母 & u.韵母 & $u.声调 &
             " " & 极性符 &
             " 坐标(" & formatFloat(u.x, ffDecimal, 2) & "," &
             formatFloat(u.y, ffDecimal, 2) & "," &
             formatFloat(u.z, ffDecimal, 2) & ")" &
             " 权限:" & $u.权限 & " 数值:0x" & toHex(u.数值, 8)
  return lines.join("\n")

#==============================================================
#  入口
#==============================================================

when isMainModule:
  var c = 初始化活码编译器()

  # 注册满全法核心活码单元
  c.注册活码("满", "m", "an", 3, 0x22801, 1, 1.0, 2.0, 3.0)
  c.注册活码("全", "q", "van", 2, 0xC4C010, 0, 4.0, 5.0, 6.0)
  c.注册活码("法", "f", "a", 3, 0x30031, 1, 7.0, 8.0, 9.0)
  c.注册活码("囡", "n", "an", 1, 0x6281E, 0, 10.0, 11.0, 12.0)
  c.注册活码("囝", "j", "ian", 3, 0xB4C03, 1, 13.0, 14.0, 15.0)
  c.注册活码("声", "sh", "eng", 1, 0x1A005, 1, 16.0, 17.0, 18.0)
  c.注册活码("调", "d", "iao", 4, 0x2D405, 0, 19.0, 20.0, 21.0)

  echo "═══ 活码编译器 ═══"
  echo ""

  # 测试1：自由活动
  echo "─ 自由活动测试 ─"
  c.自由活动(0x22801, 0.5, -0.3, 0.1)
  let u1 = c.活码表[0x22801]
  echo "  满: 活动后坐标(" & formatFloat(u1.x, ffDecimal, 2) & "," &
       formatFloat(u1.y, ffDecimal, 2) & "," &
       formatFloat(u1.z, ffDecimal, 2) & ") = 状态: 未锁定"

  # 测试2：起点激活
  echo ""
  echo "─ 起点激活 ─"
  let activated = c.起点激活(0x22801)
  echo "  起点[0x22] 激活: " & (if activated: "✓" else: "○")
  echo "  激活圈: " & $c.钥匙.激活圈

  # 测试3：锁固定（加法统计）
  echo ""
  echo "─ 锁固定（加法统计） ─"
  for k in sorted(toSeq(c.活码表.keys)):
    discard c.锁固定(k)
    let u = c.活码表[k]
    echo "  " & u.名字 & " → 权限:" & $u.权限 &
         " 数值:0x" & toHex(u.数值, 8) &
         " 固定坐标(" & formatFloat(u.固定坐标.x, ffDecimal, 2) & "," &
         formatFloat(u.固定坐标.y, ffDecimal, 2) & "," &
         formatFloat(u.固定坐标.z, ffDecimal, 2) & ") "

  # 测试4：锁后不能再活动
  echo ""
  echo "─ 锁后稳定性 ─"
  c.自由活动(0x30031, 100.0, 100.0, 100.0)  # 法已锁定，活动应被拒绝
  let uf = c.活码表[0x30031]
  echo "  法 尝试活动 → 坐标不变(" &
       formatFloat(uf.x, ffDecimal, 2) & "," &
       formatFloat(uf.y, ffDecimal, 2) & "," &
       formatFloat(uf.z, ffDecimal, 2) & ") = 已锁定"

  # 测试5：谐音路由
  echo ""
  echo "─ 谐音路由 （an韵） ─"
  let an_codes = c.谐音寻码("f", "a")
  for code in an_codes:
    echo "  f+a → 0x" & toHex(code, 4) & " " & c.活码表[code].名字

  # 测试6：权限校验
  echo ""
  echo "─ 权限校验 ─"
  let ok = c.校验权限(0x22801, 30)
  echo "  满(0x22801) 需权限30: " & (if ok: "通过 ✓" else: "拒绝 ○")

  echo ""
  echo c.系统报告()
