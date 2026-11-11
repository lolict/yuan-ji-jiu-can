## 许可 · 批判排除之后的固化
## 允许 / 禁止 / 门禁。纠正后的批判逻辑 = 许可。
## 禁止给符号或门禁入口编数字门牌。

import std/strutils
import os
import 呼唤

proc 有数字(s: string): bool =
  for ch in s:
    if ch in {'0'..'9'}: return true

proc 显示(节: string) =
  echo "【", 节, "】"
  for (k, v) in 许列(节):
    echo "  ", k, "：", v

proc 检查(): bool =
  result = true
  let 禁 = 许列("禁止")
  let 门 = 门禁列()
  echo "门禁顺序：", 门.join(" → ")
  for 段名 in 门:
    if 有数字(段名):
      echo "违规：门禁名含数字门牌 ", 段名
      result = false
    let 符 = 段符(段名)
    if 符.len == 0:
      echo "违规：门禁无符号 ", 段名
      result = false
    for (k, _) in 禁:
      if 段名 == k:
        echo "违规：门禁名落在禁止项 ", 段名
        result = false

when isMainModule:
  if paramCount() >= 1 and paramStr(1) == "检查":
    if 检查():
      echo "验收：过"
    else:
      echo "验收：未过"
      quit(1)
  else:
    echo "======== 允许 ========"
    显示("允许")
    echo "======== 禁止 ========"
    显示("禁止")
    echo "======== 门禁 ========"
    显示("门禁")
    echo "门禁顺序：", 门禁列().join(" → ")
