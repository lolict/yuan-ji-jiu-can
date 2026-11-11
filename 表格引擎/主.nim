import std/[os, strutils]
import 表

proc 报数(前缀: string, e: 引擎) =
  echo 前缀, " 站 ", e.站s.len, " / 线 ", e.线s.len, " / 路口 ", e.路口s.len,
    " / 动作 ", e.动作s.len, " / 客 ", e.客s.len, " / 调用 ", e.调用s.len,
    " / 坐标 ", e.坐标s.len, " / 格 ", e.格s.len, " / 刻 ", e.刻,
    " / 候车 ", e.候车, " / 写入 ", e.写入s.len,
    " / 拒收 ", e.拒收s.len

let 模式v = 读模式("schema.chain")

if paramCount() >= 2 and paramStr(1) == "载":
  let e = 读(paramStr(2), 模式v)
  报数("载入", e)
  quit(0)

if paramCount() >= 8 and paramStr(1) == "排":
  var e = 读(paramStr(2), 模式v)
  let 耗时 = if paramCount() >= 9: parseInt(paramStr(9)) else: 0
  let 候车 = if paramCount() >= 10: parseInt(paramStr(10)) else: -1
  doAssert 排班(e, paramStr(3), paramStr(4), paramStr(5), paramStr(6),
    paramStr(7), parseInt(paramStr(8)), 耗时, 候车), "排班失败"
  存(e, paramStr(2))
  报数("排班", e)
  quit(0)

if paramCount() >= 8 and paramStr(1) == "格":
  var e = 读(paramStr(2), 模式v)
  doAssert 加格(e, paramStr(3), parseInt(paramStr(4)), parseInt(paramStr(5)),
    paramStr(6), paramStr(7), paramStr(8)), "加格失败"
  存(e, paramStr(2))
  报数("加格", e)
  quit(0)

if paramCount() >= 3 and paramStr(1) == "刻":
  var e = 读(paramStr(2), 模式v)
  doAssert 设刻(e, paramStr(3)), "设刻失败"
  存(e, paramStr(2))
  报数("设刻", e)
  quit(0)

if paramCount() >= 4 and paramStr(1) == "站换":
  var e = 读(paramStr(2), 模式v)
  doAssert 设站换乘(e, paramStr(3), parseInt(paramStr(4))), "设站换乘失败"
  存(e, paramStr(2))
  报数("设站换乘", e)
  quit(0)

if paramCount() >= 4 and paramStr(1) == "站候":
  var e = 读(paramStr(2), 模式v)
  doAssert 设站候车(e, paramStr(3), parseInt(paramStr(4))), "设站候车失败"
  存(e, paramStr(2))
  报数("设站候车", e)
  quit(0)

if paramCount() >= 4 and paramStr(1) == "线候":
  var e = 读(paramStr(2), 模式v)
  doAssert 设线候车(e, paramStr(3), parseInt(paramStr(4))), "设线候车失败"
  存(e, paramStr(2))
  报数("设线候车", e)
  quit(0)

if paramCount() >= 3 and paramStr(1) == "候":
  var e = 读(paramStr(2), 模式v)
  doAssert 设候车(e, parseInt(paramStr(3))), "设候车失败"
  存(e, paramStr(2))
  报数("设候车", e)
  quit(0)

if paramCount() >= 6 and paramStr(1) == "址":
  var e = 读(paramStr(2), 模式v)
  doAssert 加坐标(e, paramStr(3), paramStr(4), parseInt(paramStr(5)),
    parseInt(paramStr(6))), "加坐标失败"
  存(e, paramStr(2))
  报数("加坐标", e)
  quit(0)

if paramCount() >= 8 and paramStr(1) == "岔":
  var e = 读(paramStr(2), 模式v)
  doAssert 加路口(e, paramStr(3), paramStr(4), paramStr(5), paramStr(6),
    paramStr(7), paramStr(8)), "加路口失败"
  存(e, paramStr(2))
  报数("加路口", e)
  quit(0)

if paramCount() >= 4 and paramStr(1) == "编":
  var e = 读(paramStr(2), 模式v)
  let ai = 找动作(e, paramStr(3))
  doAssert ai >= 0, "无此动作"
  e.动作s[ai].程序 = paramStr(4)
  存(e, paramStr(2))
  报数("编动作", e)
  quit(0)

if paramCount() >= 7 and paramStr(1) == "调":
  var e = 读(paramStr(2), 模式v)
  doAssert 加调用(e, paramStr(3), paramStr(4), paramStr(5), paramStr(6),
    paramStr(7)), "加调用失败"
  存(e, paramStr(2))
  报数("调用", e)
  quit(0)

if paramCount() >= 4 and paramStr(1) == "路":
  var e = 读(paramStr(2), 模式v)
  doAssert 设下一站(e, paramStr(3), paramStr(4)), "设下一站失败"
  存(e, paramStr(2))
  报数("设路", e)
  quit(0)

if paramCount() >= 4 and paramStr(1) == "编":
  var e = 读(paramStr(2), 模式v)
  let ai = 找动作(e, paramStr(3))
  doAssert ai >= 0, "无此动作"
  e.动作s[ai].程序 = paramStr(4)
  存(e, paramStr(2))
  报数("编动作", e)
  quit(0)

if paramCount() >= 5 and paramStr(1) == "上":
  var e = 读(paramStr(2), 模式v)
  doAssert 上车(e, paramStr(3), paramStr(4), paramStr(5)), "上车失败"
  存(e, paramStr(2))
  报数("上车", e)
  quit(0)

if paramCount() >= 3 and paramStr(1) == "步":
  var e = 读(paramStr(2), 模式v)
  let 成 = 一步(e, paramStr(3))
  存(e, paramStr(2))
  let ci = 找客(e, paramStr(3))
  echo "一步 ", 成, " / 状态 ", (if ci >= 0: e.客s[ci].状态 else: "无此客")
  quit(0)

let 入 = if paramCount() >= 1: paramStr(1) else: "写入.chain"
let 出 = if paramCount() >= 2: paramStr(2) else: "出"

if fileExists(入):
  var e = 新引擎(模式v)
  for 行s in readFile(入).splitLines():
    let t = 行s.strip
    if t.len == 0 or t[0] == '#' or t == "gttx://写入|符号|动作|条件|验收":
      continue
    discard 写(e, 行s)
  存(e, 出)
  报数("写入", e)
