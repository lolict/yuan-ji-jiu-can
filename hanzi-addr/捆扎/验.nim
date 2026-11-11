## 验 · 同表走一遍 查 / 选址 / 防止符 / 拼装
## 禁用 16/8/32/4 进制封装汉字或字母。
## 子命令退出码为 0 才算过；任一未过，本命令非零退出。

import osproc

proc 跑(命令: string): bool =
  echo "======== ", 命令, " ========"
  let (输出, 码) = execCmdEx(命令)
  stdout.write 输出
  if 码 == 0:
    result = true
  else:
    echo "验收：未过，退出码 ", 码
    result = false

when isMainModule:
  var 全过 = true
  全过 = 跑("查 江") and 全过
  全过 = 跑("查 明") and 全过
  全过 = 跑("选址") and 全过
  全过 = 跑("防止符") and 全过
  全过 = 跑("许可 检查") and 全过
  全过 = 跑("进制串联 走 12") and 全过
  全过 = 跑("进制串联 约定") and 全过
  全过 = 跑("约定 检查") and 全过
  全过 = 跑("约定 呼唤 声母.b") and 全过
  全过 = 跑("拼装 江") and 全过
  if 全过:
    echo "总验收：过"
  else:
    echo "总验收：未过"
    quit(1)
