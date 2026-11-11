import std/[strutils, os]
import 边界

const 候车时限* = 120

type
  表结构* = object
    名*: string
    列s*: seq[string]

  模式* = object
    表s*: seq[表结构]

  站行* = object
    地址*: int
    符号*: string
    部*: string
    入边*: string
    出边*: string
    状态*: string
    候车*: int
    换乘*: int

  动作行* = object
    动作号*: string
    程序*: string
    参数模板*: string
    前置*: string
    后置*: string
    超时*: int

  路口行* = object
    当前站*: string
    输入*: string
    条件行*: string
    下一站*: string
    动作*: string
    验收*: string

  写入行* = object
    符号*: string
    动作*: string
    条件*: string
    验收*: string

  拒收行* = object
    原行*: string
    原因*: string
    时刻*: string

  线行* = object
    起站*: string
    止站*: string
    方向*: string
    载具*: string
    时刻*: string
    容量*: int
    耗时*: int
    候车*: int

  客行* = object
    请求号*: string
    当前站*: string
    目标站*: string
    可上车*: string
    状态*: string
    线号*: string
    调用号*: string

  调用行* = object
    调用站*: string
    被调站*: string
    返回站*: string
    参数*: string
    结果*: string

  坐标行* = object
    站*: string
    线*: string
    段*: int
    位*: int

  格行* = object
    线*: string
    段*: int
    位*: int
    动作*: string
    时刻*: string
    值*: string

  表达式* = object
    表*: string
    字段*: string
    操作符*: string
    值*: string

  引擎* = object
    模式*: 模式
    站s*: seq[站行]
    动作s*: seq[动作行]
    路口s*: seq[路口行]
    写入s*: seq[写入行]
    拒收s*: seq[拒收行]
    线s*: seq[线行]
    客s*: seq[客行]
    调用s*: seq[调用行]
    坐标s*: seq[坐标行]
    格s*: seq[格行]
    刻*: int
    候车*: int
    下一地址*: int

proc 单元文*(u: 单元): string =
  case u.类
  of 文, 引:
    result = u.文本
  else:
    result = ""

proc 找表*(模式: 模式, 名: string): int =
  for i, t in 模式.表s:
    if t.名 == 名:
      return i
  result = -1

proc 有列*(模式: 模式, 表名, 列名: string): bool =
  let i = 找表(模式, 表名)
  if i < 0:
    return false
  for c in 模式.表s[i].列s:
    if c == 列名:
      return true
  result = false

proc 读模式*(路径: string): 模式 =
  for 行s in readFile(路径).splitLines():
    let r = 解析行(行s)
    if r.注释:
      continue
    doAssert r.ok, "schema 行解析失败: " & 行s & " / " & r.错
    doAssert r.行.表 == "schema", "schema 行表名错: " & 行s
    doAssert r.行.单元s.len >= 2, "schema 行列不足: " & 行s
    var t = 表结构(名: 单元文(r.行.单元s[0]), 列s: @[])
    for k in 1 ..< r.行.单元s.len:
      let 列 = 单元文(r.行.单元s[k])
      doAssert 列.len > 0, "schema 空列: " & 行s
      t.列s.add 列
    doAssert t.名.len > 0, "schema 空表名: " & 行s
    doAssert 找表(result, t.名) < 0, "schema 表重复: " & t.名
    result.表s.add t
  doAssert 找表(result, "写入") >= 0, "schema 缺写入表"
  doAssert result.表s[找表(result, "写入")].列s.len == 4, "写入表列数不是4"

proc 合法符号*(s: string): bool =
  if s.len == 0:
    return false
  for c in s:
    if c in {' ', '\t', '\n', '\r', '|', ',', '{', '}', '"', ';', ':', '@', '#'}:
      return false
  result = true

proc 解析表达式*(s: string, 表达式: var 表达式): bool =
  let s2 = s.replace("。", ".")
  let 段 = s2.splitWhitespace()
  if 段.len < 3:
    return false
  let tf = 段[0].split('.')
  if tf.len != 2:
    return false
  表达式.表 = tf[0]
  表达式.字段 = tf[1]
  表达式.操作符 = 段[1]
  表达式.值 = 段[2 .. ^1].join(" ")
  result = 表达式.表.len > 0 and 表达式.字段.len > 0 and
    表达式.操作符 in ["=", "!=", ">", "<", ">=", "<="] and 表达式.值.len > 0

proc 新引擎*(模式: 模式): 引擎 =
  result.模式 = 模式
  result.下一地址 = 1
  result.刻 = -1
  result.候车 = 候车时限

proc 时分钟*(s: string): int =
  let 段 = s.split(':')
  if 段.len != 2:
    return -1
  try:
    let 时 = parseInt(段[0])
    let 分 = parseInt(段[1])
    if 时 < 0 or 时 > 23 or 分 < 0 or 分 > 59:
      return -1
    result = 时 * 60 + 分
  except ValueError:
    result = -1

proc 当日刻*(刻, 时: int): int =
  result = (刻 div 1440) * 1440 + 时

proc 待发刻*(刻, 时: int): int =
  result = 当日刻(刻, 时)
  if result < 刻:
    result += 1440


proc 设刻*(e: var 引擎, 时刻: string): bool =
  let 分 = 时分钟(时刻)
  if 分 < 0:
    return false
  e.刻 = 分
  result = true

proc 设候车*(e: var 引擎, 分钟: int): bool =
  if 分钟 < 0:
    return false
  e.候车 = 分钟
  result = true

proc 找站*(e: 引擎, 符号: string): int =
  for i, s in e.站s:
    if s.符号 == 符号:
      return i
  result = -1

proc 找动作*(e: 引擎, 动作号: string): int =
  for i, a in e.动作s:
    if a.动作号 == 动作号:
      return i
  result = -1

proc 找客*(e: 引擎, 请求号: string): int =
  for i, c in e.客s:
    if c.请求号 == 请求号:
      return i
  result = -1

proc 部名(符号: string): string =
  let i = 符号.find('.')
  if i > 0:
    result = 符号[0 ..< i]
  else:
    result = "未定"

proc 拒*(e: var 引擎, 原行, 原因, 时刻: string) =
  e.拒收s.add 拒收行(原行: 原行, 原因: 原因, 时刻: 时刻)

proc 字段(s: string): string =
  let t = s.replace("|", "¦").replace("\n", " ").replace("\r", " ")
  if t.len == 0:
    return "\"\""
  if t[0] in {'"', '{', '@'}:
    result = "\"" & t.replace("\\", "\\\\").replace("\"", "\\\"") & "\""
  else:
    result = t

proc 存*(e: 引擎, 目录: string) =
  createDir(目录)
  var 站s = "gttx://站|地址|符号|部|入边|出边|状态|候车|换乘\n"
  for x in e.站s:
    站s.add "gttx://站|" & $x.地址 & "|" & 字段(x.符号) & "|" & 字段(x.部) & "|" &
      字段(x.入边) & "|" & 字段(x.出边) & "|" & 字段(x.状态) & "|" & $x.候车 &
      "|" & $x.换乘 & "\n"
  writeFile(目录 / "站.chain", 站s)

  var 动作s = "gttx://动作|动作号|程序|参数模板|前置|后置|超时\n"
  for x in e.动作s:
    动作s.add "gttx://动作|" & 字段(x.动作号) & "|" & 字段(x.程序) & "|" &
      字段(x.参数模板) & "|" & 字段(x.前置) & "|" & 字段(x.后置) & "|" & $x.超时 & "\n"
  writeFile(目录 / "动作.chain", 动作s)

  var 路口s = "gttx://路口|当前站|输入|条件行|下一站|动作|验收\n"
  for x in e.路口s:
    路口s.add "gttx://路口|" & 字段(x.当前站) & "|" & 字段(x.输入) & "|" &
      字段(x.条件行) & "|" & 字段(x.下一站) & "|" & 字段(x.动作) & "|" &
      字段(x.验收) & "\n"
  writeFile(目录 / "路口.chain", 路口s)

  var 格s = "gttx://格|线|段|位|动作|时刻|值\n"
  for x in e.格s:
    格s.add "gttx://格|" & 字段(x.线) & "|" & $x.段 & "|" & $x.位 & "|" &
      字段(x.动作) & "|" & 字段(x.时刻) & "|" & 字段(x.值) & "\n"
  writeFile(目录 / "格.chain", 格s)

  writeFile(目录 / "刻.chain", "gttx://刻|当前|候车\n" &
    "gttx://刻|" & $e.刻 & "|" & $e.候车 & "\n")

  var 坐标s = "gttx://坐标|站|线|段|位\n"
  for x in e.坐标s:
    坐标s.add "gttx://坐标|" & 字段(x.站) & "|" & 字段(x.线) & "|" &
      $x.段 & "|" & $x.位 & "\n"
  writeFile(目录 / "坐标.chain", 坐标s)

  var 写入s = "gttx://写入|符号|动作|条件|验收\n"
  for x in e.写入s:
    写入s.add "gttx://写入|" & 字段(x.符号) & "|" & 字段(x.动作) & "|" &
      字段(x.条件) & "|" & 字段(x.验收) & "\n"
  writeFile(目录 / "写入.chain", 写入s)

  var 拒收s = "gttx://拒收|原行|原因|时刻\n"
  for x in e.拒收s:
    拒收s.add "gttx://拒收|" & 字段(x.原行) & "|" & 字段(x.原因) & "|" &
      字段(x.时刻) & "\n"
  writeFile(目录 / "拒收.chain", 拒收s)

  var 线s = "gttx://线|起站|止站|方向|载具|时刻|容量|耗时|候车\n"
  for x in e.线s:
    线s.add "gttx://线|" & 字段(x.起站) & "|" & 字段(x.止站) & "|" &
      字段(x.方向) & "|" & 字段(x.载具) & "|" & 字段(x.时刻) & "|" & $x.容量 &
      "|" & $x.耗时 & "|" & $x.候车 & "\n"
  writeFile(目录 / "线.chain", 线s)

  var 客s = "gttx://客|请求号|当前站|目标站|可上车|状态|线号|调用号\n"
  for x in e.客s:
    客s.add "gttx://客|" & 字段(x.请求号) & "|" & 字段(x.当前站) & "|" &
      字段(x.目标站) & "|" & 字段(x.可上车) & "|" & 字段(x.状态) & "|" &
      字段(x.线号) & "|" & 字段(x.调用号) & "\n"
  writeFile(目录 / "客.chain", 客s)

  var 调用s = "gttx://调用|调用站|被调站|返回站|参数|结果\n"
  for x in e.调用s:
    调用s.add "gttx://调用|" & 字段(x.调用站) & "|" & 字段(x.被调站) & "|" &
      字段(x.返回站) & "|" & 字段(x.参数) & "|" & 字段(x.结果) & "\n"
  writeFile(目录 / "调用.chain", 调用s)

proc 排班*(e: var 引擎, 起站, 止站, 方向, 载具, 时刻: string,
    容量: int, 耗时: int = 0, 候车: int = -1): bool =
  if 找站(e, 起站) < 0 or 找站(e, 止站) < 0:
    return false
  if 起站 == 止站 or 容量 <= 0 or 耗时 < 0 or 候车 < -1 or 时分钟(时刻) < 0:
    return false
  for l in e.线s:
    if l.起站 == 起站 and l.止站 == 止站 and l.时刻 == 时刻:
      return false
  e.线s.add 线行(起站: 起站, 止站: 止站, 方向: 方向, 载具: 载具,
    时刻: 时刻, 容量: 容量, 耗时: 耗时, 候车: 候车)
  result = true

proc 找线键*(e: 引擎, 键: string): int

proc 设线候车*(e: var 引擎, 键: string, 分钟: int): bool =
  if 分钟 < -1:
    return false
  let i = 找线键(e, 键)
  if i < 0:
    return false
  e.线s[i].候车 = 分钟
  result = true

proc 设站换乘*(e: var 引擎, 站: string, 分钟: int): bool =
  if 分钟 < 0:
    return false
  let i = 找站(e, 站)
  if i < 0:
    return false
  e.站s[i].换乘 = 分钟
  result = true

proc 站换乘值*(e: 引擎, 站: string): int =
  let i = 找站(e, 站)
  if i >= 0 and e.站s[i].换乘 >= 0:
    result = e.站s[i].换乘
  else:
    result = 0

proc 设站候车*(e: var 引擎, 站: string, 分钟: int): bool =
  if 分钟 < -1:
    return false
  let i = 找站(e, 站)
  if i < 0:
    return false
  e.站s[i].候车 = 分钟
  result = true

proc 站候车值*(e: 引擎, 站: string): int =
  let i = 找站(e, 站)
  if i >= 0 and e.站s[i].候车 >= 0:
    result = e.站s[i].候车
  else:
    result = e.候车

proc 有效候车*(e: 引擎, 站: string, l: 线行): int =
  if l.候车 >= 0:
    result = l.候车
  else:
    result = 站候车值(e, 站)

proc 找线*(e: 引擎, 起站, 止站: string): int =
  for i, l in e.线s:
    if l.起站 == 起站 and l.止站 == 止站:
      return i
  result = -1

proc 客值(c: 客行, 字段: string): string =
  case 字段
  of "请求号": c.请求号
  of "当前站": c.当前站
  of "目标站": c.目标站
  of "可上车": c.可上车
  of "状态": c.状态
  of "线号": c.线号
  of "调用号": c.调用号
  else: ""

proc 站值(s: 站行, 字段: string): string =
  case 字段
  of "地址": $s.地址
  of "符号": s.符号
  of "部": s.部
  of "入边": s.入边
  of "出边": s.出边
  of "状态": s.状态
  else: ""

proc 取值*(e: 引擎, 表名, 字段, 请求号: string): string =
  case 表名
  of "客":
    let i = 找客(e, 请求号)
    if i >= 0:
      result = 客值(e.客s[i], 字段)
  of "站":
    let ci = 找客(e, 请求号)
    if ci >= 0:
      let si = 找站(e, e.客s[ci].当前站)
      if si >= 0:
        result = 站值(e.站s[si], 字段)
  else:
    result = ""

proc 求值*(e: 引擎, x: 表达式, 请求号: string): bool =
  let 左 = 取值(e, x.表, x.字段, 请求号)
  var 左n, 右n: int
  var 有数 = true
  try:
    左n = parseInt(左)
    右n = parseInt(x.值)
  except ValueError:
    有数 = false
  if 有数:
    case x.操作符
    of "=": result = 左n == 右n
    of "!=": result = 左n != 右n
    of ">": result = 左n > 右n
    of "<": result = 左n < 右n
    of ">=": result = 左n >= 右n
    of "<=": result = 左n <= 右n
    else: result = false
  else:
    case x.操作符
    of "=": result = 左 == x.值
    of "!=": result = 左 != x.值
    else: result = false

proc 条件成立*(e: 引擎, 原文, 请求号: string): bool =
  var x: 表达式
  if 解析表达式(原文, x):
    result = 求值(e, x, 请求号)

proc 解址*(e: 引擎, s: string): string =
  if s.contains('#'):
    let 段 = s.split('#')
    if 段.len == 3:
      try:
        let 段n = parseInt(段[1])
        let 位n = parseInt(段[2])
        for c in e.坐标s:
          if c.线 == 段[0] and c.段 == 段n and c.位 == 位n:
            return c.站
      except ValueError:
        discard
  else:
    if 找站(e, s) >= 0:
      return s
  result = ""

proc 设下一站*(e: var 引擎, 当前站, 下一站: string): bool =
  if 找站(e, 当前站) < 0 or 解址(e, 下一站).len == 0:
    return false
  for i in 0 ..< e.路口s.len:
    if e.路口s[i].当前站 == 当前站:
      e.路口s[i].下一站 = 下一站
      return true
  result = false

proc 加路口*(e: var 引擎, 当前站, 输入, 条件, 下一站, 动作, 验收: string): bool =
  if 找站(e, 当前站) < 0 or 解址(e, 下一站).len == 0:
    return false
  if 找动作(e, 动作) < 0:
    return false
  var 条, 验: 表达式
  if not 解析表达式(条件, 条) or not 解析表达式(验收, 验):
    return false
  if not 有列(e.模式, 条.表, 条.字段) or not 有列(e.模式, 验.表, 验.字段):
    return false
  for r in e.路口s:
    if r.当前站 == 当前站 and r.输入 == 输入 and r.条件行 == 条件 and
      r.下一站 == 下一站 and r.动作 == 动作 and r.验收 == 验收:
      return false
  e.路口s.add 路口行(当前站: 当前站, 输入: 输入, 条件行: 条件,
    下一站: 下一站, 动作: 动作, 验收: 验收)
  result = true

proc 上车*(e: var 引擎, 请求号, 当前站, 目标站: string): bool =
  if 请求号.len == 0 or 找站(e, 当前站) < 0:
    return false
  if 目标站.len > 0 and 找站(e, 目标站) < 0:
    return false
  if 找客(e, 请求号) >= 0:
    return false
  e.客s.add 客行(请求号: 请求号, 当前站: 当前站, 目标站: 目标站,
    可上车: "是", 状态: "待定", 线号: "", 调用号: "")
  result = true

proc 线键*(l: 线行): string =
  l.起站 & ">" & l.止站 & "@" & l.时刻

proc 找线键*(e: 引擎, 键: string): int =
  for i, l in e.线s:
    if 线键(l) == 键:
      return i
  result = -1

proc 加坐标*(e: var 引擎, 站, 线: string, 段, 位: int): bool =
  if 找站(e, 站) < 0 or 找线键(e, 线) < 0:
    return false
  if 段 < 0 or 位 < 0:
    return false
  for c in e.坐标s:
    if c.站== 站 and c.线 == 线 and c.段 == 段 and c.位 == 位:
      return false
  e.坐标s.add 坐标行(站: 站, 线: 线, 段: 段, 位: 位)
  result = true

proc 加格*(e: var 引擎, 线: string, 段, 位: int, 动作, 时刻, 值: string): bool =
  if 找线键(e, 线) < 0 or 找动作(e, 动作) < 0:
    return false
  if 段 < 0 or 位 < 0 or 时分钟(时刻) < 0 or 值.len == 0:
    return false
  for x in e.格s:
    if x.线 == 线 and x.段 == 段 and x.位 == 位 and x.动作 == 动作 and
      x.时刻 == 时刻:
      return false
  e.格s.add 格行(线: 线, 段: 段, 位: 位, 动作: 动作, 时刻: 时刻, 值: 值)
  result = true

proc 找格*(e: 引擎, 线: string, 段, 位: int, 动作, 时刻: string): int =
  for i, x in e.格s:
    if x.线 == 线 and x.段 == 段 and x.位 == 位 and x.动作 == 动作 and
      x.时刻 == 时刻:
      return i
  result = -1

proc 格值*(e: 引擎, 线: string, 段, 位: int, 动作, 时刻: string): string =
  let i = 找格(e, 线, 段, 位, 动作, 时刻)
  if i >= 0:
    result = e.格s[i].值

proc 找坐标*(e: 引擎, 站, 线: string): int =
  for i, c in e.坐标s:
    if c.站== 站 and c.线 == 线:
      return i
  result = -1

proc 按址找站*(e: 引擎, 线: string, 段, 位: int): string =
  for c in e.坐标s:
    if c.线 == 线 and c.段 == 段 and c.位 == 位:
      return c.站
  result = ""

proc 站坐标*(e: 引擎, 站: string): seq[坐标行] =
  for c in e.坐标s:
    if c.站== 站:
      result.add c

proc 交叉数*(e: 引擎, 站: string): int =
  var 已: seq[string] = @[]
  for c in e.坐标s:
    if c.站== 站:
      var 有 = false
      for x in 已:
        if x == c.线:
          有 = true
      if not 有:
        已.add c.线
  result = 已.len

proc 线上人数*(e: 引擎, 键: string): int =
  for c in e.客s:
    if (c.状态 == "在车上" or c.状态 == "候车") and c.线号 == 键:
      inc result

proc 找可乘线*(e: 引擎, 请求号: string): int =
  let ci = 找客(e, 请求号)
  if ci < 0:
    return -1
  for i, l in e.线s:
    let t = 时分钟(l.时刻)
    if e.刻 >= 0 and t >= 0:
      let 当日 = 当日刻(e.刻, t)
      let 发 = 待发刻(e.刻, t)
      if 当日 < e.刻 and 发 - e.刻 > 有效候车(e, e.客s[ci].当前站, l):
        continue
    if l.起站 == e.客s[ci].当前站 and 线上人数(e, 线键(l)) < l.容量:
      return i
  result = -1

proc 有站*(序: seq[string], 值: string): bool =
  for x in 序:
    if x == 值:
      return true
  result = false

proc 序找(名s: seq[string], 名: string): int =
  for i, x in 名s:
    if x == 名:
      return i
  result = -1

proc 下一站*(e: 引擎, 请求号: string): string =
  let ci = 找客(e, 请求号)
  if ci < 0 or e.客s[ci].目标站.len == 0:
    return ""
  let 起 = e.客s[ci].当前站
  let 终 = e.客s[ci].目标站
  if 起 == 终:
    return ""
  if e.刻 >= 0:
    var 名s = @[起]
    var 距 = @[e.刻]
    var 父 = @[""]
    var 定 = @[false]
    var 到 = -1
    while true:
      var 当 = -1
      var 最 = -1
      for i in 0 ..< 名s.len:
        if not 定[i] and (当 < 0 or 距[i] < 最):
          当 = i
          最 = 距[i]
      if 当 < 0:
        break
      定[当] = true
      if 名s[当] == 终:
        到 = 当
        break
      for l in e.线s:
        if l.起站 != 名s[当]:
          continue
        let t = 时分钟(l.时刻)
        if t < 0:
          continue
        let 当日 = 当日刻(距[当], t)
        let 发 = 待发刻(距[当], t)
        if 当日 < 距[当] and 发 - 距[当] > 有效候车(e, 名s[当], l):
          continue
        if 线上人数(e, 线键(l)) >= l.容量:
          continue
        let 到刻 = 发 + l.耗时 + 站换乘值(e, l.止站)
        var 去 = 序找(名s, l.止站)
        if 去 < 0:
          名s.add l.止站
          距.add 到刻
          父.add 名s[当]
          定.add false
        elif 到刻 < 距[去]:
          距[去] = 到刻
          父[去] = 名s[当]
    if 到 < 0 or 父[到].len == 0:
      return ""
    var 节 = 名s[到]
    var 父节 = 父[到]
    while 父节.len > 0 and 父节 != 起:
      节 = 父节
      let i = 序找(名s, 节)
      if i < 0:
        return ""
      父节 = 父[i]
    result = 节
    return
  var 队站 = @[起]
  var 队首 = @[""]
  var 已 = @[起]
  var 头 = 0
  while 头 < 队站.len:
    let 当 = 队站[头]
    let 首 = 队首[头]
    inc 头
    for l in e.线s:
      if 线上人数(e, 线键(l)) >= l.容量:
        continue
      if l.起站 == 当 and not 有站(已, l.止站):
        let 新首 = if 当 == 起: l.止站 else: 首
        if l.止站 == 终:
          return 新首
        已.add l.止站
        队站.add l.止站
        队首.add 新首
  result = ""

proc 找线到*(e: 引擎, 请求号, 止站: string): int =
  let ci = 找客(e, 请求号)
  if ci < 0:
    return -1
  result = -1
  for i, l in e.线s:
    if l.起站 != e.客s[ci].当前站 or l.止站 != 止站:
      continue
    if 线上人数(e, 线键(l)) >= l.容量:
      continue
    if e.刻 >= 0:
      let t = 时分钟(l.时刻)
      if t < 0:
        continue
      let 当日 = 当日刻(e.刻, t)
      let 发 = 待发刻(e.刻, t)
      if 当日 < e.刻 and 发 - e.刻 > 有效候车(e, e.客s[ci].当前站, l):
        continue
      let 到刻 = 发 + l.耗时 + 站换乘值(e, l.止站)
      if result >= 0 and
        到刻 >= 待发刻(e.刻, 时分钟(e.线s[result].时刻)) + e.线s[result].耗时:
        continue
    elif result >= 0:
      continue
    result = i

proc 登车到*(e: var 引擎, 请求号, 止站: string): bool =
  let li = 找线到(e, 请求号, 止站)
  if li < 0:
    return false
  let ci = 找客(e, 请求号)
  e.客s[ci].线号 = 线键(e.线s[li])
  e.客s[ci].状态 = "在车上"
  result = true

proc 登车*(e: var 引擎, 请求号: string): bool =
  let li = 找可乘线(e, 请求号)
  if li < 0:
    return false
  let ci = 找客(e, 请求号)
  e.客s[ci].线号 = 线键(e.线s[li])
  e.客s[ci].状态 = "在车上"
  result = true

proc 调用键*(c: 调用行): string =
  c.调用站 & ">" & c.被调站 & "@" & c.返回站

proc 找调用键*(e: 引擎, 键: string): int =
  for i, c in e.调用s:
    if 调用键(c) == 键:
      return i
  result = -1

proc 找调用*(e: 引擎, 调用站: string): int =
  for i, c in e.调用s:
    if c.调用站 == 调用站:
      return i
  result = -1

proc 加调用*(e: var 引擎, 调用站, 被调站, 返回站, 参数, 结果: string): bool =
  if 找站(e, 调用站) < 0 or 找站(e, 被调站) < 0 or 找站(e, 返回站) < 0:
    return false
  if 调用站 == 被调站 or 调用站 == 返回站 or 被调站 == 返回站:
    return false
  let 键 = 调用站 & ">" & 被调站 & "@" & 返回站
  if 找调用键(e, 键) >= 0:
    return false
  e.调用s.add 调用行(调用站: 调用站, 被调站: 被调站, 返回站: 返回站,
    参数: 参数, 结果: 结果)
  result = true

proc 发起调用*(e: var 引擎, 请求号: string): bool =
  let ci = 找客(e, 请求号)
  if ci < 0 or e.客s[ci].调用号.len > 0:
    return false
  let ki = 找调用(e, e.客s[ci].当前站)
  if ki < 0:
    return false
  e.客s[ci].当前站 = e.调用s[ki].被调站
  e.客s[ci].调用号 = 调用键(e.调用s[ki])
  e.客s[ci].状态 = "调用中"
  result = true

proc 算值 (e: 引擎, 请求号,原值:string): string =
  var 文 = 原值
  if 文.len == 0:
    return ""
  let ci = 找客(e, 请求号)
  if ci >= 0:
    for f in ["请求号", "当前站", "目标站", "可上车", "状态", "线号", "调用号"]:
      文 = 文.replace("客。" & f, 客值(e.客s[ci], f))
    let si = 找站(e, e.客s[ci].当前站)
    if si >= 0:
      文 = 文.replace("站。地址", $e.站s[si].地址)
      文 = 文.replace("站。符号", e.站s[si].符号)
      文 = 文.replace("站。部", e.站s[si].部)
      文 = 文.replace("站。入边", e.站s[si].入边)
      文 = 文.replace("站。出边", e.站s[si].出边)
      文 = 文.replace("站。状态", e.站s[si].状态)
      文 = 文.replace("站。候车", $e.站s[si].候车)
      文 = 文.replace("站。换乘", $e.站s[si].换乘)
  文 = 文.replace("刻", $e.刻)
  if '+' in 文:
    let 段 = 文.split('+')
    if 段.len == 2:
      try:
        return $(parseInt(段[0].strip) + parseInt(段[1].strip))
      except: discard
  if '-' in 文:
    let 段 = 文.split('-')
    if 段.len == 2:
      try:
        return $(parseInt(段[0].strip) - parseInt(段[1].strip))
      except: discard
  result = 文

proc 设值*(e: var 引擎, 请求号, 表名, 字段, 值: string): bool =
  let ci = 找客(e, 请求号)
  if ci < 0:
    return false
  case 表名
  of "客":
    case 字段
    of "请求号": e.客s[ci].请求号 = 值
    of "当前站": e.客s[ci].当前站 = 值
    of "目标站": e.客s[ci].目标站 = 值
    of "可上车": e.客s[ci].可上车 = 值
    of "状态": e.客s[ci].状态 = 值
    of "线号": e.客s[ci].线号 = 值
    of "调用号": e.客s[ci].调用号 = 值
    else: return false
  of "站":
    let si = 找站(e, e.客s[ci].当前站)
    if si < 0:
      return false
    case 字段
    of "地址":
      try:
        e.站s[si].地址 = parseInt(值)
      except ValueError:
        return false
    of "符号": e.站s[si].符号 = 值
    of "部": e.站s[si].部 = 值
    of "入边": e.站s[si].入边 = 值
    of "出边": e.站s[si].出边 = 值
    of "状态": e.站s[si].状态 = 值
    of "候车":
      try:
        e.站s[si].候车 = parseInt(值)
      except ValueError:
        return false
    of "换乘":
      try:
        e.站s[si].换乘 = parseInt(值)
      except ValueError:
        return false
    else: return false
  else:
    return false
  result = true

proc 执行动作*(e: var 引擎, 动作号, 请求号: string): bool =
  let ai = 找动作(e, 动作号)
  if ai < 0:
    return false
  let 程序 = e.动作s[ai].程序.strip
  if 程序.len == 0:
    return true
  for 段 in 程序.split(';'):
    let t = 段.strip
    if t.len == 0:
      continue
    let i = t.find('=')
    if i <= 0:
      return false
    let 左 = t[0 ..< i].strip
    let 值 = t[i + 1 .. ^1].strip
    let tf = 左.split('.')
    if tf.len != 2:
      return false
    if not 设值(e, 请求号, tf[0], tf[1], 值):
      return false
  result = true

proc 下一事件刻*(e: 引擎, 请求号: string): int =
  let ci = 找客(e, 请求号)
  if ci < 0 or e.刻 < 0:
    return -1
  result = -1
  for l in e.线s:
    if l.起站 == e.客s[ci].当前站 and 线上人数(e, 线键(l)) < l.容量:
      let t = 时分钟(l.时刻)
      if t >= 0:
        let 当日 = 当日刻(e.刻, t)
        let 发 = 待发刻(e.刻, t)
        if 发 > e.刻 and (当日 >= e.刻 or 发 - e.刻 <= 有效候车(e, e.客s[ci].当前站, l)) and
          (result < 0 or 发 < result):
          result = 发
  for c in 站坐标(e, e.客s[ci].当前站):
    for x in e.格s:
      if x.线 == c.线 and x.段 == c.段 and x.位 == c.位:
        let t = 时分钟(x.时刻)
        if t >= 0:
          let 当日 = 当日刻(e.刻, t)
          let 发 = 待发刻(e.刻, t)
          if 发 > e.刻 and (当日 >= e.刻 or 发 - e.刻 <= 站候车值(e, e.客s[ci].当前站)) and
            (result < 0 or 发 < result):
            result = 发

proc 找格时*(e: 引擎, 站: string): int =
  if e.刻 < 0:
    return -1
  for c in 站坐标(e, 站):
    for i, x in e.格s:
      if x.线 == c.线 and x.段 == c.段 and x.位 == c.位 and
        时分钟(x.时刻) == (e.刻 mod 1440):
        return i
  result = -1

proc 一步*(e: var 引擎, 请求号: string): bool =
  let ci = 找客(e, 请求号)
  if ci < 0:
    return false
  if e.客s[ci].状态 == "在车上" or e.客s[ci].状态 == "候车":
    let li = 找线键(e, e.客s[ci].线号)
    if li < 0:
      e.客s[ci].状态 = "线丢失"
      e.客s[ci].线号 = ""
      return false
    let 发 = 待发刻(e.刻, 时分钟(e.线s[li].时刻))
    if e.刻 >= 0 and 时分钟(e.线s[li].时刻) >= 0 and e.刻 < 发:
      e.刻 = 发
      e.客s[ci].状态 = "候车"
      return false
    let 下 = e.线s[li].止站
    if e.刻 >= 0 and 时分钟(e.线s[li].时刻) >= 0:
      let 到刻 = 发 + e.线s[li].耗时 + 站换乘值(e, e.线s[li].止站)
      if 到刻 > e.刻:
        e.刻 = 到刻
    e.客s[ci].当前站 = 下
    e.客s[ci].线号 = ""
    if e.客s[ci].调用号.len > 0:
      e.客s[ci].状态 = "调用中"
    elif e.客s[ci].目标站.len > 0 and 下 == e.客s[ci].目标站:
      e.客s[ci].状态 = "到站"
    else:
      e.客s[ci].状态 = "已下车"
    return true
  if e.客s[ci].调用号.len > 0:
    let ki = 找调用键(e, e.客s[ci].调用号)
    if ki < 0:
      e.客s[ci].状态 = "调用丢失"
      e.客s[ci].调用号 = ""
      return false
    if e.客s[ci].当前站 == e.调用s[ki].返回站:
      e.客s[ci].调用号 = ""
      e.客s[ci].状态 = "调用返回"
      return true
  if e.客s[ci].目标站.len > 0 and e.客s[ci].当前站 == e.客s[ci].目标站:
    e.客s[ci].状态 = "到站"
    return false
  let 格i = 找格时(e, e.客s[ci].当前站)
  if 格i >= 0:
    if not 执行动作(e, e.格s[格i].动作, 请求号):
      e.客s[ci].状态 = "动作失败"
      return false
    let 下 = 解址(e, e.格s[格i].值)
    if 下.len > 0:
      e.客s[ci].当前站 = 下
      if e.客s[ci].调用号.len > 0:
        e.客s[ci].状态 = "调用中"
      elif e.客s[ci].目标站.len > 0 and 下 == e.客s[ci].目标站:
        e.客s[ci].状态 = "到站"
      else:
        e.客s[ci].状态 = "已验收"
    else:
      e.客s[ci].状态 = "格动作"
    return true
  if e.客s[ci].调用号.len == 0 and 发起调用(e, 请求号):
    return true
  if e.客s[ci].目标站.len > 0:
    let 下 = 下一站(e, 请求号)
    if 下.len > 0 and 登车到(e, 请求号, 下):
      return true
  elif 登车(e, 请求号):
    return true
  var 路 = -1
  for i, r in e.路口s:
    if r.当前站 == e.客s[ci].当前站 and r.下一站.len > 0 and 条件成立(e, r.条件行,请求号):
      路 = i
      break
  if 路 < 0:
    let t = 下一事件刻(e, 请求号)
    if t > e.刻:
      e.刻 = t
      e.客s[ci].状态 = "候刻"
    else:
      e.客s[ci].状态 = "无下一站"
    return false
  if e.路口s[路].下一站.len == 0:
    let t = 下一事件刻(e, 请求号)
    if t > e.刻:
      e.刻 = t
      e.客s[ci].状态 = "候刻"
      return false
    e.客s[ci].状态 = "无下一站"
    return false
  let 下站 = 解址(e, e.路口s[路].下一站)
  if 下站.len == 0:
    e.客s[ci].状态 = "地址无站"
    return false
  let 在调用 = e.客s[ci].调用号.len > 0
  e.客s[ci].当前站 = 下站
  if not 执行动作(e, e.路口s[路].动作, 请求号):
    e.客s[ci].状态 = "动作失败"
    return false
  if 在调用:
    e.客s[ci].状态 = "调用中"
  else:
    e.客s[ci].状态 = "已上车"
    if 条件成立(e, e.路口s[路].验收, 请求号):
      e.客s[ci].状态 = "已验收"
    else:
      e.客s[ci].状态 = "验收未过"
  result = true

proc 文列(r: 行): seq[string] =
  for u in r.单元s:
    result.add 单元文(u)

proc 读*(目录: string, 模式: 模式): 引擎 =
  result = 新引擎(模式)
  if fileExists(目录 / "刻.chain"):
    let 行s = readFile(目录 / "刻.chain").strip.splitLines()
    for i in 1 ..< 行s.len:
      let r = 解析行(行s[i], 严格 = true)
      doAssert r.ok, "刻表回读失败: " & 行s[i] & " / " & r.错
      doAssert r.行.表 == "刻", "刻表表名错: " & 行s[i]
      let 文 = 文列(r.行)
      doAssert 文.len == 1 or 文.len == 2, "刻表列数错: " & 行s[i]
      result.刻 = parseInt(文[0])
      if 文.len == 2:
        result.候车 = parseInt(文[1])
  if fileExists(目录 / "站.chain"):
    let 行s = readFile(目录 / "站.chain").strip.splitLines()
    for i in 1 ..< 行s.len:
      let r = 解析行(行s[i], 严格 = true)
      doAssert r.ok, "站表回读失败: " & 行s[i] & " / " & r.错
      doAssert r.行.表 == "站", "站表表名错: " & 行s[i]
      let 文 = 文列(r.行)
      doAssert 文.len >= 6 and 文.len <= 8, "站表列数错: " & 行s[i]
      let 地址 = parseInt(文[0])
      let 候车 = if 文.len >= 7: parseInt(文[6]) else: -1
      let 换乘 = if 文.len == 8: parseInt(文[7]) else: 0
      result.站s.add 站行(地址: 地址, 符号: 文[1], 部: 文[2],
        入边: 文[3], 出边: 文[4], 状态: 文[5], 候车: 候车, 换乘: 换乘)
      if 地址 >= result.下一地址:
        result.下一地址 = 地址 + 1
  if fileExists(目录 / "动作.chain"):
    let 行s = readFile(目录 / "动作.chain").strip.splitLines()
    for i in 1 ..< 行s.len:
      let r = 解析行(行s[i], 严格 = true)
      doAssert r.ok, "动作表回读失败: " & 行s[i] & " / " & r.错
      doAssert r.行.表 == "动作", "动作表表名错: " & 行s[i]
      let 文 = 文列(r.行)
      doAssert 文.len == 6, "动作表列数错: " & 行s[i]
      result.动作s.add 动作行(动作号: 文[0], 程序: 文[1], 参数模板: 文[2],
        前置: 文[3], 后置: 文[4], 超时: parseInt(文[5]))
  if fileExists(目录 / "路口.chain"):
    let 行s = readFile(目录 / "路口.chain").strip.splitLines()
    for i in 1 ..< 行s.len:
      let r = 解析行(行s[i], 严格 = true)
      doAssert r.ok, "路口表回读失败: " & 行s[i] & " / " & r.错
      doAssert r.行.表 == "路口", "路口表表名错: " & 行s[i]
      let 文 = 文列(r.行)
      doAssert 文.len == 6, "路口表列数错: " & 行s[i]
      result.路口s.add 路口行(当前站: 文[0], 输入: 文[1], 条件行: 文[2],
        下一站: 文[3], 动作: 文[4], 验收: 文[5])
  if fileExists(目录 / "线.chain"):
    let 行s = readFile(目录 / "线.chain").strip.splitLines()
    for i in 1 ..< 行s.len:
      let r = 解析行(行s[i], 严格 = true)
      doAssert r.ok, "线表回读失败: " & 行s[i] & " / " & r.错
      doAssert r.行.表 == "线", "线表表名错: " & 行s[i]
      let 文 = 文列(r.行)
      doAssert 文.len >= 6 and 文.len <= 8, "线表列数错: " & 行s[i]
      let 耗时 = if 文.len >= 7: parseInt(文[6]) else: 0
      let 候车 = if 文.len == 8: parseInt(文[7]) else: -1
      result.线s.add 线行(起站: 文[0], 止站: 文[1], 方向: 文[2],
        载具: 文[3], 时刻: 文[4], 容量: parseInt(文[5]), 耗时: 耗时, 候车: 候车)
  if fileExists(目录 / "客.chain"):
    let 行s = readFile(目录 / "客.chain").strip.splitLines()
    for i in 1 ..< 行s.len:
      let r = 解析行(行s[i], 严格 = true)
      doAssert r.ok, "客表回读失败: " & 行s[i] & " / " & r.错
      doAssert r.行.表 == "客", "客表表名错: " & 行s[i]
      let 文 = 文列(r.行)
      doAssert 文.len == 7, "客表列数错: " & 行s[i]
      result.客s.add 客行(请求号: 文[0], 当前站: 文[1], 目标站: 文[2],
        可上车: 文[3], 状态: 文[4], 线号: 文[5], 调用号: 文[6])
  if fileExists(目录 / "调用.chain"):
    let 行s = readFile(目录 / "调用.chain").strip.splitLines()
    for i in 1 ..< 行s.len:
      let r = 解析行(行s[i], 严格 = true)
      doAssert r.ok, "调用表回读失败: " & 行s[i] & " / " & r.错
      doAssert r.行.表 == "调用", "调用表表名错: " & 行s[i]
      let 文 = 文列(r.行)
      doAssert 文.len == 5, "调用表列数错: " & 行s[i]
      result.调用s.add 调用行(调用站: 文[0], 被调站: 文[1], 返回站: 文[2],
        参数: 文[3], 结果: 文[4])
  if fileExists(目录 / "格.chain"):
    let 行s = readFile(目录 / "格.chain").strip.splitLines()
    for i in 1 ..< 行s.len:
      let r = 解析行(行s[i], 严格 = true)
      doAssert r.ok, "格表回读失败: " & 行s[i] & " / " & r.错
      doAssert r.行.表 == "格", "格表表名错: " & 行s[i]
      let 文 = 文列(r.行)
      doAssert 文.len == 6, "格表列数错: " & 行s[i]
      doAssert 找线键(result, 文[0]) >= 0, "格线不存在: " & 行s[i]
      doAssert 找动作(result, 文[3]) >= 0, "格动作不存在: " & 行s[i]
      result.格s.add 格行(线: 文[0], 段: parseInt(文[1]), 位: parseInt(文[2]),
        动作: 文[3], 时刻: 文[4], 值: 文[5])
  if fileExists(目录 / "坐标.chain"):
    let 行s = readFile(目录 / "坐标.chain").strip.splitLines()
    for i in 1 ..< 行s.len:
      let r = 解析行(行s[i], 严格 = true)
      doAssert r.ok, "坐标表回读失败: " & 行s[i] & " / " & r.错
      doAssert r.行.表 == "坐标", "坐标表表名错: " & 行s[i]
      let 文 = 文列(r.行)
      doAssert 文.len == 4, "坐标表列数错: " & 行s[i]
      doAssert 找站(result, 文[0]) >= 0, "坐标站不存在: " & 行s[i]
      doAssert 找线键(result, 文[1]) >= 0, "坐标线不存在: " & 行s[i]
      result.坐标s.add 坐标行(站: 文[0], 线: 文[1], 段: parseInt(文[2]),
        位: parseInt(文[3]))
  if fileExists(目录 / "写入.chain"):
    let 行s = readFile(目录 / "写入.chain").strip.splitLines()
    for i in 1 ..< 行s.len:
      let r = 解析行(行s[i], 严格 = true)
      doAssert r.ok, "写入表回读失败: " & 行s[i] & " / " & r.错
      doAssert r.行.表 == "写入", "写入表表名错: " & 行s[i]
      let 文 = 文列(r.行)
      doAssert 文.len == 4, "写入表列数错: " & 行s[i]
      result.写入s.add 写入行(符号: 文[0], 动作: 文[1], 条件: 文[2], 验收: 文[3])
  if fileExists(目录 / "拒收.chain"):
    let 行s = readFile(目录 / "拒收.chain").strip.splitLines()
    for i in 1 ..< 行s.len:
      let r = 解析行(行s[i], 严格 = true)
      doAssert r.ok, "拒收表回读失败: " & 行s[i] & " / " & r.错
      doAssert r.行.表 == "拒收", "拒收表表名错: " & 行s[i]
      let 文 = 文列(r.行)
      doAssert 文.len == 3, "拒收表列数错: " & 行s[i]
      result.拒收s.add 拒收行(原行: 文[0], 原因: 文[1], 时刻: 文[2])

proc 写*(e: var 引擎, 原行: string, 时刻 = "0"): bool =
  let r = 解析行(原行, 严格 = true)
  if not r.ok:
    拒(e, 原行, r.错, 时刻)
    return false
  if r.行.表 != "写入":
    拒(e, 原行, "表不是写入: " & r.行.表, 时刻)
    return false
  if r.行.单元s.len != 4:
    拒(e, 原行, "列数不对: " & $r.行.单元s.len, 时刻)
    return false

  var 文s: seq[string] = @[]
  for u in r.行.单元s:
    if u.类 == 墙 or u.类 == 头:
      拒(e, 原行, "写入列必须是文本", 时刻)
      return false
    文s.add 单元文(u)

  if 文s[0].len == 0:
    拒(e, 原行, "缺符号", 时刻)
    return false
  if 文s[1].len == 0:
    拒(e, 原行, "缺动作", 时刻)
    return false
  if 文s[2].len == 0:
    拒(e, 原行, "缺条件", 时刻)
    return false
  if 文s[3].len == 0:
    拒(e, 原行, "缺验收", 时刻)
    return false
  if not 合法符号(文s[0]):
    拒(e, 原行, "符号不合法: " & 文s[0], 时刻)
    return false

  var 条件, 验收: 表达式
  if not 解析表达式(文s[2], 条件):
    拒(e, 原行, "条件格式不对: " & 文s[2], 时刻)
    return false
  if not 解析表达式(文s[3], 验收):
    拒(e, 原行, "验收格式不对: " & 文s[3], 时刻)
    return false
  if not 有列(e.模式, 条件.表, 条件.字段):
    拒(e, 原行, "条件表或字段不存在: " & 条件.表 & "." & 条件.字段, 时刻)
    return false
  if not 有列(e.模式, 验收.表, 验收.字段):
    拒(e, 原行, "验收表或字段不存在: " & 验收.表 & "." & 验收.字段, 时刻)
    return false
  if 找站(e, 文s[0]) >= 0:
    拒(e, 原行, "符号重复: " & 文s[0], 时刻)
    return false

  e.站s.add 站行(地址: e.下一地址, 符号: 文s[0], 部: 部名(文s[0]),
    入边: "", 出边: "", 状态: "待定", 候车: -1, 换乘: 0)
  e.下一地址 += 1
  if 找动作(e, 文s[1]) < 0:
    e.动作s.add 动作行(动作号: 文s[1], 程序: "", 参数模板: "",
      前置: "", 后置: "", 超时: 0)
  e.路口s.add 路口行(当前站: 文s[0], 输入: "", 条件行: 文s[2],
    下一站: "", 动作: 文s[1], 验收: 文s[3])
  e.写入s.add 写入行(符号: 文s[0], 动作: 文s[1], 条件: 文s[2], 验收: 文s[3])
  result = true
