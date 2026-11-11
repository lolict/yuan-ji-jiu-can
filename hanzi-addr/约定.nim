## 约定法 · 声明式编程语言 + 呼唤式编程语法
## 主导者 / 跟随者 / 功能类型 / 约定 / 号令 / 范畴符 / 大约定 / 小约定
## 用约定的号令作私有门禁内部的范畴符，用范畴符本质的功能去干活。
## 大约定驾驭小约定；小约定只在自己的范畴内生效。
## 门禁可编目、可地址、可导航、可计算（地址来自 码位表.toml）。

import os, strutils, tables, unicode
import 呼唤

type
  类 = enum
    c主导者, c跟随者, c功能类型, c约定, c号令, c范畴符, c大约定, c小约定
  声明 = ref object
    类: 类
    名: string
    属: string
    值: string
    域: string
    行: int
  模型 = object
    序: seq[声明]
    表: Table[string, 声明]

const
  约定源 = staticRead("约定.chain")
  码位源 = staticRead("码位表.toml")

let 码位表 = 读toml(码位源)

proc 键(域, 名: string): string =
  if 域.len == 0: 名 else: 域 & "/" & 名

proc 类名(c: 类): string =
  case c
  of c主导者: "主导者"
  of c跟随者: "跟随者"
  of c功能类型: "功能类型"
  of c约定: "约定"
  of c号令: "号令"
  of c范畴符: "范畴符"
  of c大约定: "大约定"
  of c小约定: "小约定"

proc 读(文本: string): 模型 =
  var 域栈 = @[""]
  var 行号 = 0
  for raw in 文本.splitLines:
    inc 行号
    var 行 = raw
    let c = 行.find('#')
    if c >= 0: 行 = 行[0 ..< c]
    行 = 行.strip
    if 行.len == 0: continue
    if 行 == "}":
      if 域栈.len > 1: discard 域栈.pop
      continue
    var 词 = strutils.splitWhitespace(行)
    var 开 = false
    if 词.len > 0 and 词[^1] == "{":
      开 = true
      discard 词.pop
    if 词.len < 2: continue
    let 域 = 域栈[^1]
    let 类名 = 词[0]
    var d: 声明
    case 类名
    of "主导者": d = 声明(类: c主导者, 名: 词[1], 属: 域, 域: 域, 行: 行号)
    of "跟随者": d = 声明(类: c跟随者, 名: 词[1], 值: 词[^1], 属: 域, 域: 域, 行: 行号)
    of "功能类型": d = 声明(类: c功能类型, 名: 词[1], 属: 域, 域: 域, 行: 行号)
    of "约定": d = 声明(类: c约定, 名: 词[1], 值: 词[^1], 属: 域, 域: 域, 行: 行号)
    of "号令": d = 声明(类: c号令, 名: 词[1], 值: 词[^1], 属: 域, 域: 域, 行: 行号)
    of "范畴符": d = 声明(类: c范畴符, 名: 词[1], 值: 词[^1], 属: 域, 域: 域, 行: 行号)
    of "小约定": d = 声明(类: c小约定, 名: 词[1], 值: 词[^1], 属: 域, 域: 域, 行: 行号)
    of "大约定": d = 声明(类: c大约定, 名: 词[1], 属: 域, 域: 域, 行: 行号)
    else:
      echo "第", 行号, "行 未识别声明：", 类名
      continue
    result.序.add d
    result.表[键(域, 词[1])] = d
    if 类名 == "大约定" and 开:
      域栈.add 键(域, 词[1])

proc 找(模型: 模型; 域, 名: string): 声明 =
  if 名.len == 0: return nil
  if 模型.表.hasKey(名): return 模型.表[名]
  var 层 = 域
  while true:
    if 模型.表.hasKey(键(层, 名)):
      return 模型.表[键(层, 名)]
    if 层.len == 0: break
    let i = 层.rfind('/')
    if i < 0: 层 = "" else: 层 = 层[0 ..< i]
  nil

proc 链(模型: 模型; d: 声明): string =
  result = 类名(d.类) & " " & d.名
  if d.值.len > 0:
    result &= " → " & d.值
    var t = 模型.找(d.域, d.值)
    var 防 = 0
    while t != nil and t.值.len > 0 and 防 < 8:
      result &= " → " & t.值
      t = 模型.找(t.域, t.值)
      inc 防
    if d.类 == c约定: result &= "（约定）"
    elif d.类 == c号令: result &= "（号令·可作范畴符）"

proc 展开(模型: 模型) =
  echo "约定法 声明（", 模型.序.len, " 条）："
  for d in 模型.序:
    let 缩 = if d.域.len == 0: "" else: "    "
    if d.域.len > 0 and d.类 == c小约定:
      echo 缩, "（私属于 ", d.域, "）", 链(模型, d)
    else:
      echo 缩, 链(模型, d)

proc 语法() =
  echo "约定法 · 声明式编程语法（一行一约，可 # 注释）："
  echo "  主导者 <名>"
  echo "  跟随者 <名> 属于 <主导者>"
  echo "  功能类型 <名>"
  echo "  约定 <名> 是 <功能类型>"
  echo "  号令 <名> 取 <约定>"
  echo "  范畴符 <名> 取 <号令>"
  echo "  大约定 <名> {"
  echo "    小约定 <名> 是 <约定>"
  echo "  }"
  echo "呼唤式："
  echo "  呼唤 <名>            按名称呼唤主导者/跟随者/功能类型/约定/号令/范畴符"
  echo "  呼唤 <门禁>.<符>     呼唤门禁符号的可计算地址"
  echo "  范畴 <作用域> <名>   看某名在某作用域内是否生效（小约定不外泄）"

proc 检查(模型: 模型): int =
  var 错 = 0
  for d in 模型.序:
    case d.类
    of c跟随者:
      let t = 模型.找(d.域, d.值)
      if t == nil or t.类 != c主导者:
        echo "错 第", d.行, "行：跟随者 ", d.名, " 归属的不是主导者：", d.值
        inc 错
    of c约定:
      let t = 模型.找(d.域, d.值)
      if t == nil or t.类 != c功能类型:
        echo "错 第", d.行, "行：约定 ", d.名, " 不是功能类型：", d.值
        inc 错
    of c号令:
      let t = 模型.找(d.域, d.值)
      if t == nil or t.类 != c约定:
        echo "错 第", d.行, "行：号令 ", d.名, " 取的不是约定：", d.值
        inc 错
    of c范畴符:
      let t = 模型.找(d.域, d.值)
      if t == nil or t.类 != c号令:
        echo "错 第", d.行, "行：范畴符 ", d.名, " 取的不是号令：", d.值
        inc 错
    of c小约定:
      if d.域.len == 0:
        echo "错 第", d.行, "行：小约定 ", d.名, " 不在任何大约定内"
        inc 错
      let t = 模型.找(d.域, d.值)
      if t == nil or t.类 != c约定:
        echo "错 第", d.行, "行：小约定 ", d.名, " 不是约定：", d.值
        inc 错
    else: discard
  echo "约定检查：", (if 错 == 0: "过" else: "未过 " & $错 & " 处")
  错

proc 呼唤名(模型: 模型; 名: string) =
  if 名.contains('.'):
    let p = 名.split('.')
    if p.len == 2 and 码位表.量.hasKey(p[0]) and 码位表.量[p[0]].hasKey(p[1]):
      echo "门禁 ", p[0], " 的符号 ", p[1], " → 地址 ", 码位表.量[p[0]][p[1]]
    else:
      echo "无此门禁符号：", 名
    return
  let d = 模型.找("", 名)
  if d != nil:
    echo 链(模型, d)
    return
  if 门禁列().contains(名):
    echo "门禁 ", 名, "：跟随者 ", 段符(名).join(" "), "（", 段符(名).len, " 符）"
    return
  echo "呼唤不到：", 名

proc 范畴(模型: 模型; 作用域, 名: string) =
  let d = 模型.找(作用域, 名)
  let 域名 = if 作用域.len == 0: "顶层" else: 作用域
  if d == nil:
    echo 域名, " 内不可见：", 名
  else:
    echo 域名, " 内可见：", 链(模型, d)

const 源目录 = currentSourcePath().parentDir

proc 原话() =
  let p = 源目录 / "原版" / "约定法_门禁阶梯_原话.txt"
  if fileExists(p):
    stdout.write(readFile(p))
  else:
    echo "找不到原话文件：", p

when isMainModule:
  let 模 = 读(约定源)
  if paramCount() == 0:
    展开(模)
  else:
    case paramStr(1)
    of "语法": 语法()
    of "检查":
      if 检查(模) != 0: quit(1)
    of "呼唤":
      if paramCount() >= 2: 呼唤名(模, paramStr(2)) else: 呼唤名(模, "")
    of "计算":
      if paramCount() >= 2: 呼唤名(模, paramStr(2)) else: 呼唤名(模, "")
    of "范畴":
      let 作用域 = if paramCount() >= 3: paramStr(2) else: ""
      let 名 = if paramCount() >= 3: paramStr(3) else: (if paramCount() >= 2: paramStr(2) else: "")
      范畴(模, 作用域, 名)
    of "原话": 原话()
    else:
      echo "未知子命令：", paramStr(1)
      quit(1)
