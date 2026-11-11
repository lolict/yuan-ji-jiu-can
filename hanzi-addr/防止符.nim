## 13字节防止符
## 轴按名称呼唤。空座位防错位。不是字体文件，不是颜色文件。
## 禁用 16/8/32/4 进制封装汉字或字母。

import os, strutils
import 呼唤

const
  每位轴 = 2
  座位数* = 13

type
  防止符* = array[座位数, uint8]

proc 空座位*(): 防止符 =
  discard

proc 置位(s: var 防止符; 位号: int; 值: uint8) =
  if 位号 < 0: return
  if 值 > 3'u8: return
  let 位 = 位号 * 每位轴
  let 字节 = 位 div 8
  if 字节 < 0 or 字节 >= 座位数: return
  let 偏 = 位 mod 8
  let 清 = not (0b11'u8 shl 偏)
  s[字节] = (s[字节] and 清) or (值 shl 偏)

proc 取位(s: 防止符; 位号: int): uint8 =
  if 位号 < 0: return 0
  let 位 = 位号 * 每位轴
  let 字节 = 位 div 8
  if 字节 < 0 or 字节 >= 座位数: return 0
  let 偏 = 位 mod 8
  (s[字节] shr 偏) and 0b11'u8

proc 置*(s: var 防止符; 轴名, 态名: string) =
  置位(s, 轴位(轴名), 态位(态名))

proc 取*(s: 防止符; 轴名: string): string =
  态唤(取位(s, 轴位(轴名)))

proc 交*(a, b: 防止符): 防止符 =
  for i in 0 ..< 座位数:
    result[i] = a[i] and b[i]

proc 并*(a, b: 防止符): 防止符 =
  for i in 0 ..< 座位数:
    result[i] = a[i] or b[i]

proc 占用名*(s: 防止符): string =
  for 名 in 轴列():
    if 取(s, 名) != "空":
      if result.len > 0: result.add(" ")
      result.add(名)

proc 占用*(s: 防止符): int =
  for 名 in 轴列():
    if 取(s, 名) != "空": inc result

proc 显*(名: string; s: 防止符) =
  echo "名：", 名
  echo "长度：", 座位数, "字节"
  echo "轴：", 轴列().join(" ")
  echo "占用：", 占用名(s)
  echo "空座位：", 轴列().len - 占用(s)
  echo "显示：呼唤名，不用十六八三十二四进制封装汉字或字母"
  echo "不是字体文件"
  echo "不是颜色文件"

proc 演示() =
  var 金 = 空座位()
  置(金, "工部", "乙")
  置(金, "金", "有")
  var 青 = 空座位()
  置(青, "工部", "有")
  置(青, "青", "乙")
  echo "======== 金 ========"
  显("金", 金)
  echo "======== 青 ========"
  显("青", 青)
  echo "======== 交 ========"
  显("金交青", 交(金, 青))
  echo "======== 并 ========"
  显("金并青", 并(金, 青))
  let 过 = 占用名(金) == "工部 金" and 占用名(青) == "工部 青" and
    占用(交(金, 青)) == 0 and 占用名(并(金, 青)) == "工部 金 青"
  echo if 过: "验收：过" else: "验收：未过"
  if not 过: quit(1)

when isMainModule:
  if paramCount() >= 1 and paramStr(1) == "金":
    var 金 = 空座位()
    置(金, "工部", "乙")
    置(金, "金", "有")
    显("金", 金)
  else:
    演示()
