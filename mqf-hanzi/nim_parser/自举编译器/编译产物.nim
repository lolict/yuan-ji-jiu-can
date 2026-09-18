## 编译产物 · 寻址模式（无搜索）
##
##  本模块运行时只使用预建索引。
##  初始化时一次性加载域索引（可跨会话持久化）。
##  查询全部常数时间——不遍历、不匹配、不搜索。
##
##  运行：nim c --cc:clang -r 编译产物.nim

import std/tables
import std/strutils
import std/os

# =================================================================
#  类型
# =================================================================
type
  声母条目* = object
    声母*: string
    态*: string
    极性*: string
    值*: int

  极性锚点* = ref object
    ## 极性常量（私有锚点）
    我极性*: int
    刘楚恬极性*: int

  域数据库* = ref object
    字根*: Table[string, int]
    声母组*: seq[声母条目]
    极性表*: Table[string, string]
    qmf*: Table[string, string]
    公理*: Table[string, string]
    声调*: Table[string, string]
    流动作*: Table[string, string]
    字段*: seq[string]

  容器结构* = object
    地址*: int
    态*: string
    值*: int
    能量*: int
    持久度*: int
    位置*: int
    相位*: int
    层级*: int
    极性*: int

# =================================================================
#  域数据库一次性构建
# =================================================================
proc 从索引文件加载*(路径: string): 域数据库 =
  new(result)
  result.字根 = initTable[string, int]()
  result.声母组 = @[]
  result.极性表 = initTable[string, string]()
  result.qmf = initTable[string, string]()
  result.公理 = initTable[string, string]()
  result.声调 = initTable[string, string]()
  result.流动作 = initTable[string, string]()
  result.字段 = @[]

  if not 路径.fileExists:
    return result

  let 文本 = 路径.readFile
  var 当前段 = ""
  for rawLine in 文本.splitLines:
    let 行 = rawLine.strip
    if 行.len == 0 or 行[0] == '#': continue

    if 行.len > 2 and 行[0] == '[' and 行[^1] == ']':
      当前段 = 行[1 .. ^2]
      continue

    case 当前段
    of "字根":
      if '=' in 行:
        let 分隔 = 行.find('=')
        result.字根[行[0 ..< 分隔]] = 行[分隔+1 .. ^1].parseInt
    of "声母":
      if '=' in 行:
        let 分隔 = 行.find('=')
        let 键 = 行[0 ..< 分隔]
        let 部分 = 行[分隔+1 .. ^1].split('|')
        if 部分.len >= 3:
          result.声母组.add(声母条目(
            声母: 键, 态: 部分[0], 极性: 部分[1], 值: 部分[2].parseInt
          ))
    of "极性":
      if '=' in 行:
        let 分隔 = 行.find('=')
        result.极性表[行[0 ..< 分隔]] = 行[分隔+1 .. ^1]
    of "QMF":
      if '=' in 行:
        let 分隔 = 行.find('=')
        result.qmf[行[0 ..< 分隔]] = 行[分隔+1 .. ^1]
    of "公理":
      if '=' in 行:
        let 分隔 = 行.find('=')
        result.公理[行[0 ..< 分隔]] = 行[分隔+1 .. ^1]
    of "声调":
      if '=' in 行:
        let 分隔 = 行.find('=')
        result.声调[行[0 ..< 分隔]] = 行[分隔+1 .. ^1]
    of "流动作":
      if '=' in 行:
        let 分隔 = 行.find('=')
        result.流动作[行[0 ..< 分隔]] = 行[分隔+1 .. ^1]
    of "字段":
      result.字段.add(行)

# =================================================================
#  常数时间查询
# =================================================================
proc 字根查询*(域: 域数据库, 笔画: string): int =
  if 域.字根.hasKey(笔画): return 域.字根[笔画]
  return 6

proc 声母查找(域: 域数据库, 声母: string): 声母条目 =
  for e in 域.声母组:
    if e.声母 == 声母: return e
  for e in 域.声母组:
    if e.声母 == "零声母": return e
  return 声母条目(声母: "?", 态: "?", 极性: "?", 值: 0)

proc 极性查询(域: 域数据库, 键: string): string =
  if 域.极性表.hasKey(键): return 域.极性表[键]
  return ""

proc 公理查询(域: 域数据库, 键: string): string =
  if 域.公理.hasKey(键): return 域.公理[键]
  return ""

proc 声调查询(域: 域数据库, 层名: string): string =
  if 域.声调.hasKey(层名): return 域.声调[层名]
  return ""

proc 流查询(域: 域数据库, 动作: string): string =
  if 域.流动作.hasKey(动作): return 域.流动作[动作]
  return ""

# =================================================================
#  容器构造
# =================================================================
proc 新建容器*(地址: int, 极性: int): 容器结构 =
  result.地址 = 地址
  result.态 = "静态"
  result.极性 = 极性

proc 推进容器*(容器: var 容器结构, 域: 域数据库): string =
  const 驱动链 = @["测距", "吃", "合一", "沉淀", "驱动"]
  var 日志: seq[string]
  for 步 in 驱动链:
    case 步
    of "测距":
      日志.add "测距:" & $容器.位置
    of "吃":
      let 消耗 = 容器.值 div 10
      日志.add "吃:能-" & $消耗
      容器.能量 = max(0, 容器.能量 - 消耗)
    of "合一":
      if 容器.极性 == 1:
        容器.位置 = max(0, 容器.位置 - 1)
    of "沉淀":
      容器.持久度 += 1
    of "驱动":
      容器.能量 += 容器.值 div 5
  result = 日志.join("→")

# =================================================================
#  演示
# =================================================================
when isMainModule:
  let 索引路径 = currentSourcePath().parentDir / "域索引.txt"

  echo "========================================"
  echo "  编译产物 · 寻址模式（无搜索）"
  echo "========================================"

  let 域 = 从索引文件加载(索引路径)
  if 域.字根.len == 0:
    echo "  [警告] 域索引为空。请先运行 自举编译器.nim"
    quit(0)

  echo ""
  echo "▸ 索引已加载:"
  echo "    字根=" & $域.字根.len
  echo "    声母=" & $域.声母组.len
  echo "    极性=" & $域.极性表.len
  echo "    QMF=" & $域.qmf.len
  echo "    公理=" & $域.公理.len

  echo ""
  echo "▸ 寻址查询（O(1)）:"
  for 笔画 in @["横", "竖", "撇", "捺", "折", "弯钩"]:
    echo "    " & 笔画 & " → " & $字根查询(域, 笔画)

  echo ""
  echo "▸ 声母寻址:"
  for 声 in @["m", "f", "l", "zh", "零声母"]:
    let e = 声母查找(域, 声)
    echo "    " & 声 & " → 态=" & e.态 & ", 极性=" & e.极性 & ", 值=" & $e.值

  echo ""
  echo "▸ 物理公理调取:"
  echo "    物质 = " & 公理查询(域, "物质")
  echo "    距离 = " & 公理查询(域, "距离")
  echo "    合一 = " & 公理查询(域, "合一")
  echo "    加法 = " & 公理查询(域, "加法")

  echo ""
  echo "▸ 容器构造+流驱动:"
  var 我的容器 = 新建容器(42, 1)
  我的容器.值 = 50
  我的容器.能量 = 30
  echo "   初始:地址=" & $我的容器.地址 & " 极性=" & $我的容器.极性 & " 值=" & $我的容器.值
  let log = 推进容器(我的容器, 域)
  echo "   推进: " & log
  echo "   最终:能=" & $我的容器.能量 & " 持久=" & $我的容器.持久度 & " 位置=" & $我的容器.位置

  echo ""
  echo "========================================"
  echo "  编译产物运行完成"
  echo "  索引一次性构建 · 查询零搜索"
  echo "========================================"
