## 自举编译器 · 直接运行（未编译即可用）
##
##  本模块不接受编译结果即可解释域规则。
##  "未编译即可用" = 规则文本存在 → 运行时直接读取并当场解释。
##
##  运行：nim c --cc:clang -r 直接运行.nim
##
##  编译仅用于生成加速后的索引产物。未编译 ≠ 不能用。

import std/parsecfg
import std/tables
import std/strutils
import std/sequtils
import std/os

# =================================================================
#  类型
# =================================================================
type
  字段状态* = enum
    fs静态, fs动态, fs临界

  极性类型* = enum
    pol阴, pol阳

  容器原型* = object
    ## 从域规则文件解析出的单个容器定义
    名称*: string
    字段*: OrderedTable[string, string]

  域段* = object
    ## 域规则的一个 @section
    段名*: string
    条目*: OrderedTable[string, string]

  域规则集* = ref object
    ## 完整的域规则加载结果
    段列表*: OrderedTable[string, 域段]
    字根*: OrderedTable[string, int]
    声母*: OrderedTable[string, tuple[态: string, 极性: string, 值: int]]
    物理公理*: OrderedTable[string, string]
    极性锚点*: OrderedTable[string, string]
    qmf*: OrderedTable[string, string]

# =================================================================
#  解析域规则文本
# =================================================================
proc 解析域规则*(文本: string): 域规则集 =
  new(result)
  result.段列表 = initOrderedTable[string, 域段]()
  result.字根 = initOrderedTable[string, int]()
  result.声母 = initOrderedTable[string, tuple[态: string, 极性: string, 值: int]]()
  result.物理公理 = initOrderedTable[string, string]()
  result.极性锚点 = initOrderedTable[string, string]()
  result.qmf = initOrderedTable[string, string]()

  var 当前段 = ""
  for 原始行 in 文本.splitLines:
    let 行 = 原始行.strip
    if 行.len == 0 or 行[0] == '#': continue
    if 行.len > 3 and 行[0] == '[' and 行[1] == '@' and 行[^1] == ']':
      当前段 = 行[2 .. ^2]
      result.段列表[当前段] = 域段(段名: 当前段, 条目: initOrderedTable[string, string]())
    elif 当前段.len > 0 and '=' in 行:
      let 分隔 = 行.find('=')
      let 键 = 行[0 ..< 分隔].strip
      let 值 = 行[分隔+1 .. ^1].strip
      result.段列表[当前段].条目[键] = 值
      case 当前段
      of "字根表":
        result.字根[键] = 值.parseInt
      of "声母映射":
        let 部分 = 值.split('|')
        if 部分.len >= 3:
          result.声母[键] = (态: 部分[0], 极性: 部分[1], 值: 部分[2].parseInt)
      of "物理公理":
        result.物理公理[键] = 值
      of "极性锚点":
        result.极性锚点[键] = 值
      of "qmf体素":
        result.qmf[键] = 值

# =================================================================
#  寻址接口（不是搜索——直接查索引）
# =================================================================
proc 字根寻址*(域: 域规则集, 笔画名: string): int =
  ## 横→1 竖→2 撇→3 捺/点→4 折→5 其它→6
  if 域.字根.hasKey(笔画名): return 域.字根[笔画名]
  result = 6  # 其它=6

proc 声母寻址*(域: 域规则集, 声母: string): tuple[态: string, 极性: string, 值: int] =
  if 域.声母.hasKey(声母): return 域.声母[声母]
  # 零声母兜底
  if 域.声母.hasKey("零声母"): return 域.声母["零声母"]
  return ("未知", "未知", 0)

proc 公理*(域: 域规则集, 键: string): string =
  if 域.物理公理.hasKey(键): return 域.物理公理[键]
  return ""

proc 极性*(域: 域规则集, 键: string): string =
  if 域.极性锚点.hasKey(键): return 域.极性锚点[键]
  return ""

proc qmf参数*(域: 域规则集, 键: string): string =
  if 域.qmf.hasKey(键): return 域.qmf[键]
  return ""

# =================================================================
#  演示
# =================================================================
when isMainModule:
  let 文件路径 = currentSourcePath().parentDir / "域规则.txt"
  echo "========================================="
  echo "  自举编译器 · 直接运行（未编译即可用）"
  echo "  读取规则: " & 文件路径
  echo "========================================="

  var 域: 域规则集
  if 文件路径.fileExists:
    let 文本 = 文件路径.readFile
    域 = 解析域规则(文本)
  else:
    echo "  [警告] 域规则.txt 不存在，使用内嵌最小规则"
    let 最小 = "[@字根表]\n横=1\n竖=2\n撇=3\n捺=4\n折=5"
    域 = 解析域规则(最小)

  echo ""
  echo "▸ 已加载段: " & $域.段列表.len
  for 段名 in 域.段列表.keys:
    echo "    @[" & 段名 & "] → " & $域.段列表[段名].条目.len & " 条目"

  echo ""
  echo "▸ 字根寻址:"
  let 笔画测试 = @["横", "竖", "撇", "捺", "点", "折", "弯钩"]
  for 笔画 in 笔画测试:
    echo "    " & 笔画 & " → " & $域.字根寻址(笔画)

  echo ""
  echo "▸ 声母寻址:"
  for 声 in @["m", "f", "b", "零声母"]:
    let r = 域.声母寻址(声)
    echo "    " & 声 & " → 态=" & r.态 & " 极性=" & r.极性 & " 值=" & $r.值

  echo ""
  echo "▸ 物理公理调取:"
  for 键 in @["物质", "距离", "合一", "加法", "乘法"]:
    echo "    " & 键 & " = " & 域.公理(键)

  echo ""
  echo "▸ 极性锚点:"
  echo "    我 → " & 域.极性("我")
  echo "    刘楚恬 → " & 域.极性("刘楚恬")

  echo ""
  echo "▸ QMF体素:"
  let 宽 = 域.qmf参数("网格宽")
  let 高 = 域.qmf参数("网格高")
  let 层 = 域.qmf参数("相位层")
  echo "    魔数=" & 域.qmf参数("魔数") & " 版本=" & 域.qmf参数("版本")
  echo "    网格=" & 宽 & "×" & 高 & "×" & 层

  echo ""
  echo "========================================="
  echo "  未编译即可用 ✓ 规则文本即真理之源"
  echo "========================================="
