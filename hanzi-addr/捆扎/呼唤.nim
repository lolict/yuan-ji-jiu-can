## 呼唤 · TOML 真源
## 笔画 / 结构 / 感知 / 字 按名称呼唤。嗅未接。

import strutils, tables, unicode

type
  源表* = object
    量*: OrderedTable[string, OrderedTable[string, string]]
    列*: OrderedTable[string, OrderedTable[string, seq[string]]]
  字条* = object
    字*, 左*, 右*, 结构*, 地址*, 序*, 注音*, 范畴*: string

proc 去引(s: string): string =
  result = s.strip
  if result.len >= 2 and result[0] == '"' and result[^1] == '"':
    result = result[1 .. ^2]

proc 读toml*(文本: string): 源表 =
  result.量 = initOrderedTable[string, OrderedTable[string, string]]()
  result.列 = initOrderedTable[string, OrderedTable[string, seq[string]]]()
  var 节 = ""
  var 列键 = ""
  var 在列 = false
  for raw in 文本.splitLines:
    let 行 = raw.strip
    if 行.len == 0 or 行[0] == '#': continue
    if 在列:
      var 体 = 行
      let 收 = 体.startsWith("]") or 体.endsWith("]")
      体 = 体.strip(chars = {']', ','}).strip
      if 体.len > 0:
        result.列[节][列键].add(去引(体))
      if 收:
        在列 = false
        列键 = ""
      continue
    if 行[0] == '[':
      var n = 行.strip(chars = {'[', ']'})
      节 = 去引(n)
      if not result.量.hasKey(节):
        result.量[节] = initOrderedTable[string, string]()
      if not result.列.hasKey(节):
        result.列[节] = initOrderedTable[string, seq[string]]()
      continue
    let eq = 行.find('=')
    if eq < 0: continue
    let k = 行[0 ..< eq].strip
    var v = 行[eq + 1 .. ^1].strip
    if not result.量.hasKey(节):
      result.量[节] = initOrderedTable[string, string]()
    if not result.列.hasKey(节):
      result.列[节] = initOrderedTable[string, seq[string]]()
    if v.startsWith("["):
      result.列[节][k] = @[]
      let rest = v[1 .. ^1].strip
      if rest.endsWith("]"):
        let body = rest[0 .. ^2].strip
        if body.len > 0:
          for p in body.split(','):
            let t = 去引(p.strip)
            if t.len > 0: result.列[节][k].add(t)
      else:
        列键 = k
        在列 = true
        var 项 = rest
        if 项.endsWith(","): 项 = 项[0 .. ^2]
        项 = 去引(项.strip)
        if 项.len > 0: result.列[节][k].add(项)
    else:
      result.量[节][k] = 去引(v)

proc 量取*(源: 源表; 节, 键: string): string =
  if 源.量.hasKey(节) and 源.量[节].hasKey(键):
    return 源.量[节][键]

proc 列取*(源: 源表; 节, 键: string): seq[string] =
  if 源.列.hasKey(节) and 源.列[节].hasKey(键):
    return 源.列[节][键]

const
  笔画源 = staticRead("笔画.toml")
  结构源 = staticRead("结构.toml")
  感知源 = staticRead("感知.toml")
  字源 = staticRead("字.toml")
  轴源 = staticRead("轴.toml")
  许可源 = staticRead("许可.toml")
  进制源 = staticRead("进制串联.toml")

let
  笔画表* = 读toml(笔画源)
  结构表* = 读toml(结构源)
  感知表* = 读toml(感知源)
  字源表* = 读toml(字源)
  轴表* = 读toml(轴源)
  许可表* = 读toml(许可源)
  进制表* = 读toml(进制源)

proc 形列*(笔: string): seq[string] =
  列取(笔画表, 笔, "形")

proc 注音*(字: string): string =
  量取(感知表, "注音", 字)

proc 范畴*(符: string): string =
  量取(感知表, "范畴", 符)

proc 路径*(构: string): seq[string] =
  result = 列取(结构表, "先后", 构)
  if result.len == 0:
    result = 列取(结构表, "相位路径", 构)

proc 六根列*(): seq[string] =
  if 感知表.量.hasKey("频率"):
    for k in 感知表.量["频率"].keys:
      result.add(k)
  elif 感知表.量.hasKey("六根"):
    for k in 感知表.量["六根"].keys:
      result.add(k)

proc 偏旁序*(符: string): string =
  if 符.len == 0: return
  量取(字源表, "偏旁", 符)

proc 画数*(序: string): int =
  for ch in 序.runes:
    inc result

proc 字名列*(): seq[string] =
  for k in 字源表.量.keys:
    if k != "偏旁": result.add(k)

proc 字取*(名: string): 字条 =
  if 名.len == 0 or 名 == "偏旁": return
  if not 字源表.量.hasKey(名): return
  result.字 = 名
  result.左 = 量取(字源表, 名, "左")
  result.右 = 量取(字源表, 名, "右")
  result.结构 = 量取(字源表, 名, "结构")
  let 左序 = 偏旁序(result.左)
  let 右序 = 偏旁序(result.右)
  result.序 = 左序 & 右序
  let 左画 = 画数(左序)
  let 右画 = 画数(右序)
  result.地址 = $(左画 + 右画) & "-" & $左画 & "-" & $右画 & "-" & 名
  result.注音 = 注音(名)
  if result.注音.len == 0: result.注音 = "未注音"
  result.范畴 = 范畴(result.左)
  if result.范畴.len == 0: result.范畴 = 范畴(名)
  if result.范畴.len == 0: result.范畴 = "积木"

proc 轴列*(): seq[string] =
  列取(轴表, "先后", "列")

proc 轴位*(名: string): int =
  result = -1
  for i, n in 轴列():
    if n == 名: return i

proc 态列*(): seq[string] =
  列取(轴表, "态先后", "列")

proc 态归*(名: string): string =
  result = 量取(轴表, "态", 名)
  if result.len == 0: result = 名

proc 态位*(名: string): uint8 =
  let 归 = 态归(名)
  for i, n in 态列():
    if n == 归: return i.uint8

proc 态唤*(位: uint8): string =
  let 列 = 态列()
  if 位.int < 列.len: return 列[位.int]
  if 位 != 0: return "有"
  "空"

proc 部轴名*(偏旁: string): string =
  量取(轴表, "偏旁", 偏旁)

proc 许列*(节: string): seq[(string, string)] =
  if 许可表.量.hasKey(节):
    for k, v in 许可表.量[节]:
      result.add((k, v))

proc 门禁列*(): seq[string] =
  列取(进制表, "顺序", "列")

proc 段符*(段: string): seq[string] =
  列取(进制表, "段", 段)

proc 韵尾组列*(): seq[string] =
  if 进制表.列.hasKey("韵尾组"):
    for k in 进制表.列["韵尾组"].keys:
      result.add(k)

proc 韵尾组*(名: string): seq[string] =
  列取(进制表, "韵尾组", 名)

proc 声母大写列*(): seq[string] =
  列取(进制表, "声母大写", "列")

proc 声调行列*(): seq[string] =
  if 进制表.列.hasKey("声调行"):
    for k in 进制表.列["声调行"].keys:
      result.add(k)

proc 声调行*(名: string): seq[string] =
  列取(进制表, "声调行", 名)

proc 颜色系列*(): seq[string] =
  if 进制表.列.hasKey("颜色建制一"):
    for k in 进制表.列["颜色建制一"].keys:
      result.add(k)

proc 韵母声调列*(): seq[string] =
  if 进制表.列.hasKey("韵母声调"):
    for k in 进制表.列["韵母声调"].keys:
      result.add(k)

proc 韵母声调*(名: string): seq[string] =
  列取(进制表, "韵母声调", 名)
