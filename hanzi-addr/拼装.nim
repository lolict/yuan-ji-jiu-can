## 拼装 · 笔画先后 → 田字格像素 → 可存字体
## 不以先天结构为目的。积木按相位封装进田字格，结果是新字。
## 点击笔画序号生成像素。注音从感知表呼唤。

import os, unicode, strutils, tables, algorithm
import 呼唤

const
  砖 = 8
  格 = 16

type
  点阵 = array[格, array[格, bool]]
  砖形 = array[砖, string]

proc 砖取(笔: string): 砖形 =
  var 列 = 形列(笔)
  if 列.len == 0: 列 = 形列("丶")
  for i in 0 ..< 砖:
    var s = if i < 列.len: 列[i] else: "........"
    while s.len < 砖: s.add('.')
    if s.len > 砖: s = s[0 ..< 砖]
    result[i] = s

const 相位 = [("左上", 0, 0), ("右上", 8, 0), ("左下", 0, 8), ("右下", 8, 8)]

proc 格坐标(名: string; 序: int): (int, int) =
  case 名
  of "左上": (0, 0)
  of "右上": (8, 0)
  of "左下": (0, 8)
  of "右下": (8, 8)
  of "左": (0, (序 mod 2) * 8)
  of "右": (8, (序 mod 2) * 8)
  of "上": ((序 mod 2) * 8, 0)
  of "下": ((序 mod 2) * 8, 8)
  of "里": (4, 4)
  of "半包": (0, (序 mod 2) * 8)
  of "外":
    let p = 相位[序 mod 相位.len]
    (p[1], p[2])
  else:
    let p = 相位[序 mod 相位.len]
    (p[1], p[2])

proc 切点(路: seq[string]; 笔数, 左画: int): int =
  if 路.len <= 1: 笔数
  elif 路[0] == "左" and 左画 > 0: 左画
  else: (笔数 + 1) div 2

proc 格名(路: seq[string]; i, 切, 笔数: int; 循环: bool): string =
  if 路.len == 0: return 相位[i mod 相位.len][0]
  if 循环: return 路[i mod 路.len]
  if i < 切 or 路.len == 1: 路[0]
  else: 路[^1]

proc 拆笔(序: string): seq[string] =
  for ch in 序.runes:
    result.add($ch)

proc 贴(d: var 点阵; 笔: string; ox, oy: int) =
  let b = 砖取(笔)
  for y in 0 ..< 砖:
    for x in 0 ..< 砖:
      if b[y][x] == '#':
        let gx = ox + x
        let gy = oy + y
        if gx >= 0 and gx < 格 and gy >= 0 and gy < 格:
          d[gy][gx] = true

proc 空格(): 点阵 =
  discard

proc 路径封(笔列: seq[string]; 路: seq[string]; 左画, 止: int; 循环: bool): 点阵 =
  result = 空格()
  let n = if 止 < 0: 笔列.len else: min(止, 笔列.len)
  let 切 = 切点(路, 笔列.len, 左画)
  var 区序: Table[string, int]
  for i in 0 ..< n:
    let 名 = 格名(路, i, 切, 笔列.len, 循环)
    let 序 = 区序.getOrDefault(名, 0)
    let xy = 格坐标(名, 序)
    贴(result, 笔列[i], xy[0], xy[1])
    区序[名] = 序 + 1

proc 显格(d: 点阵): string =
  result = "田字格：\n"
  result.add("┌────────┬────────┐\n")
  for y in 0 ..< 格:
    if y == 8: result.add("├────────┼────────┤\n")
    result.add("│")
    for x in 0 ..< 格:
      if x == 8: result.add("│")
      result.add(if d[y][x]: "#" else: ".")
    result.add("│\n")
  result.add("└────────┴────────┘\n")

proc 存pbm(d: 点阵; 路径: string) =
  var s = "P1\n# 田字格积木字体 16 16 可注释源另见 笔画.toml\n16 16\n"
  for y in 0 ..< 格:
    for x in 0 ..< 格:
      if x > 0: s.add(" ")
      s.add(if d[y][x]: "1" else: "0")
    s.add("\n")
  writeFile(路径, s)

proc 画数键(序: string): string =
  let n = 拆笔(序).len
  $n & "-" & 序

proc 像素数(笔: string): int =
  let b = 砖取(笔)
  for y in 0 ..< 砖:
    for x in 0 ..< 砖:
      if b[y][x] == '#': inc result

proc 左画数(条: 字条): int =
  画数(偏旁序(条.左))

proc 心跳文(条: 字条; 笔列: seq[string]; 路: seq[string]; 止: int; 循环: bool): string =
  let n = if 止 < 0: 笔列.len else: min(止, 笔列.len)
  let 左满 = 左画数(条)
  let 切 = 切点(路, 笔列.len, 左满)
  var 视, 听, 触, 说, 味, 嗅: int
  result = "心跳：一笔一拍 攒温 触达=线\n"
  if 路.len >= 2 and not 循环:
    result.add("阈：切=" & $切 & " 字满=" & $笔列.len & "\n")
  else:
    result.add("阈：字满=" & $笔列.len & "\n")
  for i in 0 ..< n:
    let 笔 = 笔列[i]
    let 视加 = 像素数(笔)
    inc 触
    inc 说
    视 += 视加
    var 判 = "未达"
    var 线 = ""
    if 路.len >= 2 and not 循环 and 触 == 切:
      判 = "已达"
      线 = " 发达线 " & 条.范畴
      inc 味
    if 触 == 笔列.len:
      判 = "已达"
      if 条.注音 != "未注音" and 条.注音.len > 0:
        inc 听
        线.add(" 发达线 音")
      if 循环 or 路.len <= 1:
        inc 味
        线.add(" 发达线 " & 条.范畴)
      else:
        线.add(" 发达线 整字")
    let 相位名 = 格名(路, i, 切, 笔列.len, 循环)
    result.add("拍 " & $(i + 1) & " " & 笔 & " " & 相位名 &
      " 触+1 视+" & $视加 & " 说" & 笔 & " 温" & $触 & " 判断" & 判 & 线 & "\n")
  result.add("六根：视" & $视 & " 听" & $听 & " 触" & $触 &
    " 说" & $说 & " 味" & $味 & " 嗅" & $嗅 & "\n")
  if 听 > 0:
    result.add("听：" & 条.注音 & " 注音已呼唤\n")
  else:
    result.add("听：未呼唤\n")
  result.add("嗅：未接\n")

proc 跑(名: string; 点击: int; 存: bool; 构: string) =
  var 条 = 字取(名)
  var 笔列: seq[string]
  var 标签 = 名
  if 条.字.len == 0:
    笔列 = 拆笔(名)
    if 笔列.len == 0:
      echo "未收录：", 名
      return
    标签 = "新字"
    条.序 = 名
    条.结构 = if 构.len > 0: 构 else: "动态"
    条.注音 = 注音(名)
    if 条.注音.len == 0: 条.注音 = "未注音"
    条.范畴 = 范畴(名)
    if 条.范畴.len == 0: 条.范畴 = "积木"
    条.地址 = $笔列.len & "-" & 条.结构 & "-" & 名
  else:
    笔列 = 拆笔(条.序)
    let 音 = 注音(条.字)
    if 音.len > 0: 条.注音 = 音
    let 范 = 范畴(条.左)
    if 范.len > 0: 条.范畴 = 范
    if 构.len > 0: 条.结构 = 构
  echo "字：", 标签
  echo "地址：", 条.地址
  echo "结构路径：", 条.结构
  echo "笔画先后：", 条.序
  echo "笔画进制：", 画数键(条.序)
  echo "从小到大：", 笔列.len, "画"
  echo "注音：", 条.注音
  echo "范畴：", 条.范畴
  echo "感知：", 六根列().join(" ")
  let 止 = if 点击 > 0: 点击 else: -1
  let 路 = 路径(条.结构)
  let 循环 = 条.结构 == "动态"
  let 左画 = 左画数(条)
  stdout.write 心跳文(条, 笔列, 路, 止, 循环)
  if 点击 > 0:
    echo "点击笔画：", 点击, " ", 笔列[min(点击, 笔列.len) - 1]
  let d = 路径封(笔列, 路, 左画, 止, 循环)
  echo "封装：", 路.join("→")
  stdout.write 显格(d)
  echo "结果：田字格积木，不是先天字形"
  if 存:
    let 路径名 = "/workspace/hanzi-addr/字体/" & 标签 & ".pbm"
    createDir("/workspace/hanzi-addr/字体")
    存pbm(d, 路径名)
    echo "已存：", 路径名
  if 名 == "江" and 点击 == 0 and not 存 and 构.len == 0:
    let 心 = 心跳文(条, 笔列, 路, 止, 循环)
    let 过 = 条.地址 == "6-3-3-江" and
      "发达线 流动" in 心 and "发达线 音" in 心 and "发达线 整字" in 心 and
      "六根：视36 听1 触6 说6 味1 嗅0" in 心
    echo if 过: "验收：过" else: "验收：未过"
    if not 过: quit(1)

proc 从小到大() =
  var 桶: Table[int, seq[string]]
  for 名 in 字名列():
    let c = 字取(名)
    let n = 拆笔(c.序).len
    桶.mgetOrPut(n, @[]).add(c.字)
  echo "笔画从小到大："
  var ks: seq[int]
  for k in 桶.keys: ks.add(k)
  ks.sort()
  for k in ks:
    echo k, "画：", 桶[k].join(" ")

when isMainModule:
  if paramCount() == 0:
    从小到大()
    echo ""
    跑("江", 0, false, "")
  else:
    var 名 = paramStr(1)
    var 点击 = 0
    var 存 = false
    var 构 = ""
    for i in 2 .. paramCount():
      let a = paramStr(i)
      if a == "存": 存 = true
      elif a in ["动态", "左右", "上下", "独体", "里外", "半包"]: 构 = a
      else:
        try: 点击 = parseInt(a)
        except: discard
    跑(名, 点击, 存, 构)
