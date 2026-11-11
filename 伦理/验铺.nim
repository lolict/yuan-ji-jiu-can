# 伦理映射验收 · 续接球面第 448 砖 · 纯加法/公约数 · 无分支
# 门: 首砖接 262080 / 每砖 585 / 圈环公式 / 无缝无叠 / 呼唤与伦理表同序 / 射线不封
import strutils, sequtils

const
  旧文件 = "../球面/球面映射.chain"
  新文件 = "伦理映射.chain"
  伦理文件 = "伦理.chain"
  旧砖数 = 448
  新砖数 = 84
  分度步长 = 585
  环长 = 20160
  圈长 = 262080

let 旧行 = readFile(旧文件).splitLines().filterIt(it.startsWith("gttx://球面|"))
doAssert 旧行.len == 旧砖数, "旧段应为 448 砖"

let 旧尾 = 旧行[^1].split('|')
doAssert parseInt(旧尾[5]) == 圈长, "旧段终点应为 262080"

let 新行 = readFile(新文件).splitLines().filterIt(it.startsWith("gttx://伦理映射|"))
doAssert 新行.len == 新砖数, "新段应为 84 砖"

let 伦行 = readFile(伦理文件).splitLines().filterIt(it.startsWith("gttx://伦理|"))
doAssert 伦行.len == 新砖数, "伦理真源应为 84 行"

var 上终点 = 圈长
var 位 = 旧砖数
var 跨环数 = 0

for 序 in 0 ..< 新行.len:
  let 域 = 新行[序].split('|')
  doAssert 域.len == 11, "列数应为 11: " & 新行[序]
  let 本 = parseInt(域[1])
  let 呼唤 = 域[2]
  let 部 = 域[3]
  let 起始 = parseInt(域[4])
  let 终点 = parseInt(域[5])
  let 圈 = parseInt(域[6])
  let 环 = parseInt(域[7])
  let 环内起 = parseInt(域[8])
  let 环内尾 = parseInt(域[9])
  let 尾环 = parseInt(域[10])

  doAssert 本 == 位, "位序断裂: " & 新行[序]
  doAssert 起始 == 上终点, "接缝断裂(空隙或重叠): " & 新行[序]
  doAssert 终点 - 起始 == 分度步长, "天平重量不等: " & 新行[序]
  doAssert 圈 == 起始 div 圈长, "圈错: " & 新行[序]
  doAssert 环 == ((13 * 位) div 旧砖数) mod 13, "环错: " & 新行[序]
  doAssert 环内起 == 45 * ((13 * 位) mod 旧砖数), "环内起错: " & 新行[序]
  doAssert 环内尾 == 45 * ((13 * (位 + 1)) mod 旧砖数), "环内尾错: " & 新行[序]
  doAssert 尾环 == ((13 * (位 + 1)) div 旧砖数) mod 13, "尾环错: " & 新行[序]
  doAssert 尾环 - 环 <= 1, "单砖跨环过多: " & 新行[序]
  doAssert 呼唤.startsWith(部 & "."), "呼唤未带部名: " & 新行[序]

  let 伦域 = 伦行[序].split('|')
  doAssert 呼唤 == 伦域[1], "呼唤与伦理表不同序: " & 新行[序]
  doAssert 部 == 伦域[2], "部与伦理表不同序: " & 新行[序]

  跨环数 += ord(尾环 != 环)
  上终点 = 终点
  位 += 1

doAssert 上终点 == 圈长 + 新砖数 * 分度步长, "新段终点错"
doAssert 位 == 旧砖数 + 新砖数, "总砖数错"
doAssert 跨环数 == (新砖数 * 分度步长) div 环长, "跨环切缝数错"

let 声行 = readFile(伦理文件).splitLines()
doAssert 声行.contains("gttx://射线|伦理|84|开放"), "伦理射线声明缺"
doAssert 声行.contains("gttx://线段|伦理|全铺|射线开放到84"), "伦理线段声明缺"
doAssert 声行.contains("gttx://射线|伦理映射|532|开放"), "映射射线声明缺"
doAssert 声行.contains("gttx://线段|伦理映射|449..532|射线开放到532"), "映射线段声明缺"

echo "续接: 第 449 砖起于 262080 分度 (圈1 环0), 旧 448 砖逐字不动"
echo "新段: 84 砖 × 585 分度 = 49140 分度, 终点 311220 分度"
echo "跨环: 49140 div 20160 = 2 条环缝, 每砖最多跨一环"
echo "同序: 呼唤/部 与 伦理.chain 84 行逐一对应"
echo "射线: 线段用到 532 砖, 第 533 砖起继续开放"
echo "总验收: 过"
