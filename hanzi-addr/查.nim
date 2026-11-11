## 汉字地址系统 · 查
## 拆字 → 查笔画 → 拼接 → 地址 → 渲染
## 真源：字.toml 。验收字：江  地址：6-3-3-江  笔画：丶丶一一丨一

import os, unicode
import 呼唤

proc 渲染(序: string): string =
  result = ""
  for ch in 序.runes:
    result.add("拍 " & $ch & "\n")

proc 跑(字: string) =
  let 条 = 字取(字)
  if 条.字.len == 0:
    echo "未收录：", 字
    return
  let 左序 = 偏旁序(条.左)
  let 右序 = 偏旁序(条.右)
  let 左画 = 画数(左序)
  let 右画 = 画数(右序)
  let 右名 = if 条.右.len == 0: "（无）" else: 条.右
  echo "字：", 条.字
  echo "地址：", 条.地址
  echo "偏旁：", 条.左, " ", 左画, "画"
  echo "剩余：", 右名, " ", 右画, "画"
  echo "结构：", 条.结构
  echo "部：", 部轴名(条.左), if 部轴名(条.右).len > 0: " " & 部轴名(条.右) else: ""
  echo "总笔画：", 左画 + 右画
  echo "笔画顺序：", 条.序
  echo "注音：", 条.注音
  echo "范畴：", 条.范畴
  echo "渲染："
  stdout.write 渲染(条.序)
  case 条.字
  of "江":
    let 过 = 条.地址 == "6-3-3-江" and 条.序 == "丶丶一一丨一" and 左画 == 3 and 右画 == 3
    echo if 过: "验收：过" else: "验收：未过"
    if not 过: quit(1)
  of "明":
    let 过 = 条.地址 == "8-4-4-明" and 条.序 == "丨乙一一丿乙一一" and 左画 == 4 and 右画 == 4
    echo if 过: "验收：过" else: "验收：未过"
    if not 过: quit(1)
  else:
    discard

when isMainModule:
  let 字 = if paramCount() >= 1: paramStr(1) else: "江"
  跑(字)
