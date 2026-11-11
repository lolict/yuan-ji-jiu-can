## 选址 · 查表跳转
## 一字一槽。槽是 13 字节防止符。地址串仍是 总笔画-偏旁笔画-剩余笔画-字。
## 轴按名称呼唤。槽是地址状态，不是机器码。
## 禁用 16/8/32/4 进制封装汉字或字母。

import os
import 防止符
import 呼唤

type
  槽条 = object
    字: string
    地址: string
    槽: 防止符

proc 造槽(结构, 偏旁, 剩余: string): 防止符 =
  result = 空座位()
  if 结构.len > 0: 置(result, "结构", 结构)
  let 左名 = 部轴名(偏旁)
  if 左名.len > 0: 置(result, 左名, "有")
  let 右名 = 部轴名(剩余)
  if 右名.len > 0: 置(result, 右名, "有")

proc 跳(字: string): 槽条 =
  let 条 = 字取(字)
  if 条.字.len == 0: return
  result.字 = 条.字
  result.地址 = 条.地址
  result.槽 = 造槽(条.结构, 条.左, 条.右)

proc 跑(字: string) =
  let t = 跳(字)
  if t.字.len == 0:
    echo "未收录：", 字
    return
  echo "字：", t.字
  echo "地址：", t.地址
  echo "跳转：查表命中"
  显(t.字, t.槽)
  echo "槽是地址状态，不是机器码"

proc 交验() =
  let 江 = 跳("江")
  let 河 = 跳("河")
  let 他 = 跳("他")
  echo "======== 江交河 ========"
  let 江河 = 交(江.槽, 河.槽)
  显("江交河", 江河)
  echo "水部：", 取(江河, "水部")
  echo "======== 江交他 ========"
  let 江他 = 交(江.槽, 他.槽)
  显("江交他", 江他)
  echo "水部：", 取(江他, "水部")
  let 过 = 取(江河, "水部") == "有" and 取(江他, "水部") == "空"
  echo if 过: "验收：过" else: "验收：未过"
  if not 过: quit(1)

when isMainModule:
  if paramCount() >= 1:
    跑(paramStr(1))
  else:
    交验()
