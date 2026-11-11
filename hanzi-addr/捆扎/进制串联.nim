## 进制串联 · 门禁入口阶梯
## 入口按呼唤名串联，符号用完即跳转到下一个入口。禁数字门牌。
## 阶梯：数字 → 声母 → 韵母 → 韵尾组 → 声调 → 颜色

import os, strutils, tables
import 呼唤

proc 后继(门: seq[string]; i: int): string =
  if i + 1 < 门.len: 门[i + 1] else: "（末端）"

proc 容量(段名: string): int =
  段符(段名).len

proc 阶梯() =
  let 门 = 门禁列()
  echo "门禁阶梯（符号用完即跳转）："
  for i, 名 in 门:
    echo "  入口", i + 1, " ", 名, "（", 容量(名), " 符） → 用完跳转 ", 后继(门, i)

proc 段看(名: string) =
  let 符 = 段符(名)
  if 符.len == 0:
    echo "未收录入口：", 名
    return
  let 门 = 门禁列()
  let i = 门.find(名)
  echo "入口", i + 1, " ", 名, "（", 符.len, " 符）"
  echo "符号：", 符.join(" ")
  echo "用完跳转：", 后继(门, i)

proc 走(步数: int) =
  let 门 = 门禁列()
  var 剩 = 步数
  for i, 段名 in 门:
    let 容 = 容量(段名)
    if 剩 <= 容:
      echo "第 ", 步数, " 步落在 入口", i + 1, " ", 段名, " 的 ", 段符(段名)[剩 - 1]
      return
    echo "入口", i + 1, " ", 段名, " 用完（", 容, " 容量）跳转 ", 后继(门, i)
    剩 -= 容
  echo "第 ", 步数, " 步超出所有入口"

proc 约定() =
  echo "阶梯：", 门禁列().join(" → ")
  echo "声母大写：", 声母大写列().join(" ")
  echo "韵尾组（", 韵尾组列().len, " 组）："
  for 名 in 韵尾组列():
    echo "  ", 名, "（", 韵尾组(名).len div 2, " 韵母）"
  echo "声调行（", 声调行列().len, " 行）"
  echo "颜色建制一（", 颜色系列().len, " 系）"

proc 溯源() =
  echo "来源（用户原话 vs 我推断）："
  if 进制表.量.hasKey("来源"):
    for k, v in 进制表.量["来源"]:
      echo "  ", k, "：", v

proc 口诀(存: bool) =
  var 行: seq[string]
  for s in 段符("声母"):
    for v in 韵母声调列():
      let 形 = 韵母声调(v)
      if 形.len == 0 or not 形[0][0].isLowerAscii: continue
      for f in 形:
        行.add(s & f)
  if 存:
    writeFile("口诀表.txt", 行.join("\n") & "\n")
    echo "已存 口诀表.txt（", 行.len, " 行）"
  else:
    echo "口诀：声母 × 韵母（含四声），共 ", 行.len, " 行"
    echo 行.join(" ")

proc 码位(存: bool) =
  var cp = 0xE000
  var 行: seq[string]
  行.add("# 码位表 · 每个门禁入口符号的私有码位（PUA U+E000 起）")
  行.add("# 由 进制串联 码位 生成。给门禁入口一个地址值，使门禁可计算。")
  for 门 in 门禁列():
    行.add("")
    行.add("[" & 门 & "]")
    for 符 in 段符(门):
      行.add(符 & " = \"U+" & toHex(cp, 4) & "\"")
      cp += 1
  行.add("")
  行.add("[声母大写]")
  for 符 in 声母大写列():
    行.add(符 & " = \"U+" & toHex(cp, 4) & "\"")
    cp += 1
  行.add("")
  行.add("[韵母声调]")
  for k in 韵母声调列():
    for 符 in 韵母声调(k):
      行.add(k & "_" & 符 & " = \"U+" & toHex(cp, 4) & "\"")
      cp += 1
  let 文 = 行.join("\n") & "\n"
  if 存:
    writeFile("码位表.toml", 文)
    echo "已存 码位表.toml（", cp - 0xE000, " 个码位，U+E000..U+", toHex(cp - 1, 4), "）"
  else:
    echo 文

const 源目录 = currentSourcePath().parentDir

proc 原话() =
  let p = 源目录 / "原版" / "约定法_门禁阶梯_原话.txt"
  if fileExists(p):
    stdout.write(readFile(p))
  else:
    echo "找不到原话文件：", p

when isMainModule:
  if paramCount() == 0:
    阶梯()
  elif paramStr(1) == "走":
    let n = if paramCount() >= 2: parseInt(paramStr(2)) else: 12
    走(n)
  elif paramStr(1) == "约定":
    约定()
  elif paramStr(1) == "溯源":
    溯源()
  elif paramStr(1) == "口诀":
    let 存 = paramCount() >= 2 and paramStr(2) == "存"
    口诀(存)
  elif paramStr(1) == "码位":
    let 存 = paramCount() >= 2 and paramStr(2) == "存"
    码位(存)
  elif paramStr(1) == "原话":
    原话()
  else:
    段看(paramStr(1))
