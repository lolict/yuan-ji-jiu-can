# 球面验收 · 纯加法/公约数 · 无分支
# 门: 原点数 448 / 天平两盘平 / 环梯度公式 / 无缝无叠 / 每缝一刀 / 射线延展
import strutils, tables, sequtils

const
  映射文件 = "球面映射.chain"
  声明文件 = "球面.chain"
  流账文件 = "流账.chain"
  原点数 = 448
  分度步长 = 585
  环数 = 13
  全域分度 = 262080

type 球面条目 = object
  呼唤: string
  环: int
  内起: int
  内尾: int
  尾环: int

let 映射行 = readFile(映射文件).splitLines()
let 声明行 = readFile(声明文件).splitLines()
let 账行 = readFile(流账文件).splitLines()
let 球面行 = 映射行.filterIt(it.startsWith("gttx://球面|"))

doAssert 球面行.len == 原点数, "原点数应为 448"

var 映射序 = newSeq[球面条目](原点数)

var 上终点 = 0
var 位置 = 0
var 呼唤集 = initTable[string, int]()
var 部次 = initTable[string, int]()
var 跨环数 = 0
var 跨环尾 = initTable[int, int]()

for 行 in 球面行:
  let 域 = 行.split('|')
  doAssert 域.len == 11, "列数应为 11: " & 行
  let 位 = parseInt(域[1])
  let 呼唤 = 域[2]
  let 部 = 域[3]
  let 起始 = parseInt(域[4])
  let 终点 = parseInt(域[5])
  let 圈 = parseInt(域[6])
  let 环 = parseInt(域[7])
  let 环内起 = parseInt(域[8])
  let 环内尾 = parseInt(域[9])
  let 尾环 = parseInt(域[10])

  doAssert 位 == 位置, "位序断裂: " & 行
  doAssert 起始 == 上终点, "接缝断裂(空隙或重叠): " & 行
  doAssert 终点 - 起始 == 分度步长, "天平重量不等: " & 行
  doAssert 圈 == 起始 div 全域分度, "圈错: " & 行
  doAssert 环 == (环数 * 位) div 原点数, "环错: " & 行
  doAssert 环内起 == 45 * ((环数 * 位) mod 原点数), "环内起错: " & 行
  doAssert 环内尾 == 45 * ((环数 * (位 + 1)) mod 原点数), "环内尾错: " & 行
  doAssert 尾环 == (环数 * (位 + 1)) div 原点数, "尾环错: " & 行
  doAssert 尾环 - 环 <= 1, "单块跨环过多: " & 行
  doAssert 呼唤.startsWith(部 & "."), "呼唤未带部名: " & 行
  doAssert 圈 == 0, "首圈之外: " & 行

  呼唤集[呼唤] = 呼唤集.getOrDefault(呼唤, 0) + 1
  部次[部] = 部次.getOrDefault(部, 0) + 1
  跨环数 += ord(尾环 != 环)
  跨环尾[尾环] = 跨环尾.getOrDefault(尾环, 0) + ord(尾环 != 环)
  映射序[位] = 球面条目(呼唤: 呼唤, 环: 环, 内起: 环内起, 内尾: 环内尾, 尾环: 尾环)
  上终点 = 终点
  位置 += 1

doAssert 上终点 == 全域分度, "未铺满 4680 度"
doAssert 呼唤集.len == 原点数, "呼唤名重复"
doAssert 部次["数字"] == 10, "数字部数错"
doAssert 部次["声母"] == 24, "声母部数错"
doAssert 部次["韵尾组"] == 128, "韵尾组部数错"
doAssert 部次["声调"] == 160, "声调部数错"
doAssert 部次["颜色"] == 42, "颜色部数错"
doAssert 部次["声母大写"] == 24, "声母大写部数错"
doAssert 部次["韵母声调"] == 60, "韵母声调部数错"

doAssert 跨环数 == 环数, "跨环切缝数应等于环数"
for 缝 in 1..环数:
  doAssert 跨环尾[缝] == 1, "每条环缝应恰好切一刀: " & $缝

doAssert 原点数 * 分度步长 == 全域分度, "天平左盘不平"
doAssert 4680 * 56 == 全域分度, "天平右盘不平"
doAssert 4680 div 8 == 585, "公约数未落到 585"
doAssert 原点数 div 8 == 56, "公约数未落到 56"

let 新位 = 原点数
let 新起始 = 分度步长 * 新位
doAssert 新起始 == 全域分度, "新砖起点错"
doAssert 新起始 div 全域分度 == 1, "新砖未落下一圈"
doAssert ((环数 * 新位) div 原点数) mod 环数 == 0, "新砖环号错"
doAssert 45 * ((环数 * 新位) mod 原点数) == 0, "新砖环内起错"

for 必 in [
  "gttx://射线|球面|448|开放",
  "gttx://射线|球面环|13|开放",
  "gttx://线段|球面|全铺|射线开放到448",
  "gttx://天平|结果|平|0",
  "gttx://校验|下一砖|262080|圈1环0"]:
  doAssert 声明行.contains(必) or 映射行.contains(必), "缺声明: " & 必

# —— 流账: 天平流 ——
let 称重行 = 账行.filterIt(it.startsWith("gttx://称重|"))
let 合计行 = 称重行.filterIt(it.startsWith("gttx://称重|合计|"))
let 判行 = 称重行.filterIt(it.startsWith("gttx://称重|判|"))
let 分部行 = 称重行.filterIt(not (it.startsWith("gttx://称重|合计|") or it.startsWith("gttx://称重|判|")))

doAssert 合计行.len == 1, "合计行应唯一"
doAssert 判行.len == 1, "判行应唯一"

var 称块 = 0
var 称重 = 0
for 行 in 分部行:
  let 域 = 行.split('|')
  doAssert 域.len == 7, "称重列数应为 7: " & 行
  let 块 = parseInt(域[2])
  let 重量 = parseInt(域[3])
  let 累计 = parseInt(域[5])
  doAssert 域[4] == $(重量 div 56) & "+" & $(重量 mod 56) & "/56", "度数格式错: " & 行
  doAssert 重量 == 块 * 分度步长, "部重量错: " & 行
  doAssert 累计 == 称重 + 重量, "累计不平: " & 行
  称块 += 块
  称重 += 重量

doAssert 称块 == 原点数, "天平块数错"
doAssert 称重 == 全域分度, "天平总重错"
doAssert 合计行[0].contains("|448|262080|4680+0/56|262080|"), "合计错"
doAssert 判行[0].contains("|判|平|0|左448×585|右4680×56|262080"), "判词错"

# —— 流账: 梯度流 ——
let 切缝行 = 账行.filterIt(it.startsWith("gttx://切缝|"))
doAssert 切缝行.len == 环数, "切缝行数应等于环数"

var 缝位置 = 0
for 行 in 切缝行:
  let 域 = 行.split('|')
  doAssert 域.len == 9, "切缝列数应为 9: " & 行
  let 缝 = parseInt(域[1])
  let 位 = parseInt(域[2])
  let 呼唤 = 域[3]
  let 环 = parseInt(域[4])
  let 内起 = parseInt(域[5])
  let 切到 = parseInt(域[6])
  let 下环 = parseInt(域[7])
  let 切宽 = parseInt(域[8])
  doAssert 缝 == 缝位置 + 1, "缝序断裂: " & 行
  doAssert 环 == 缝 - 1, "缝所在环错: " & 行
  doAssert 切到 == 20160, "切到环尾错: " & 行
  doAssert 切宽 == 分度步长, "切宽应等于一块: " & 行
  doAssert 下环 == 分度步长 - 20160 + 内起, "下环切入错: " & 行
  doAssert 映射序[位].呼唤 == 呼唤, "切缝呼唤不符: " & 行
  doAssert 映射序[位].环 == 环, "切缝映射环不符: " & 行
  doAssert 映射序[位].内起 == 内起, "切缝映射内起不符: " & 行
  doAssert 映射序[位].内尾 == 下环, "切缝映射内尾不符: " & 行
  doAssert 映射序[位].尾环 == 缝, "切缝映射尾环不符: " & 行
  缝位置 += 1

# —— 流账: 射线流环账 ——
let 环账行 = 账行.filterIt(it.startsWith("gttx://环账|"))
doAssert 环账行.len == 环数, "环账行数应等于环数"

var 环盖 = 0
var 环位置 = 0
for 行 in 环账行:
  let 域 = 行.split('|')
  doAssert 域.len == 7, "环账列数应为 7: " & 行
  let 环 = parseInt(域[1])
  let 起 = parseInt(域[2])
  let 终 = parseInt(域[3])
  let 块 = parseInt(域[4])
  let 覆盖 = parseInt(域[5])
  doAssert 环 == 环位置, "环序断裂: " & 行
  doAssert 起 == 环 * 20160, "环起错: " & 行
  doAssert 终 == 起 + 20160, "环终错: " & 行
  doAssert 覆盖 == 20160, "环覆盖未满 360 度: " & 行
  doAssert 块 > 0, "环内无块: " & 行
  环盖 += 覆盖
  环位置 += 1

doAssert 环盖 == 全域分度, "环账总覆盖错"

echo "分母容器: 4680 度 = 13 环 × 360 度 = 262080 分度"
echo "天平: 4680:448 = 585:56 (公约数 8), 两盘 262080 分度 平"
echo "梯度: 每原点 585 分度, 环号每原点推进 13/448"
echo "切缝: 13 条环缝各被梯度恰好切一刀"
echo "铺满: 448 原点覆盖 0..262080 分度, 无缝无叠, 圈0"
echo "流账: 天平 7 部累计 262080 分度 / 切缝 13 条 / 环账 13×360 度"
echo "射线: 第449砖起于 262080 分度 (圈1 环0), 旧448段不动"
echo "总验收: 过"
