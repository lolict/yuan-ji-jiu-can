# 伦理表验收 · 纯计数/加法 · 无分支
# 门: 身份带部名 / 同名归部 / 部数逐部对 / 内外合计对 / 流一致 / 源称差异留待裁
import strutils, tables, sequtils

const
  账文件 = "伦理.chain"
  内部部 = ["直系上下", "己身同辈", "父系旁支", "母系旁支", "下一辈", "祖辈旁支"]
  内部数 = [12, 6, 4, 4, 8, 8]
  外部部 = ["姻亲", "宗族外延", "拟制", "世交"]
  外部数 = [16, 8, 14, 4]

let 行 = readFile(账文件).splitLines()
let 伦理行 = 行.filterIt(it.startsWith("gttx://伦理|"))

doAssert 伦理行.len == 84, "伦理行数应为 84"

var 部次 = initTable[string, int]()
var 部流 = initTable[string, string]()
var 身份集 = initTable[string, int]()

for 条 in 伦理行:
  let 域 = 条.split('|')
  doAssert 域.len == 4, "列数应为 4: " & 条
  let 身份 = 域[1]
  let 部 = 域[2]
  let 流 = 域[3]
  doAssert 身份.startsWith(部 & "."), "身份未带部名: " & 条
  doAssert 流.len > 0, "流为空: " & 条
  doAssert not 身份.contains({'0'..'9'}), "身份不得用数字编号: " & 条
  doAssert 部流.getOrDefault(部, 流) == 流, "同部流不一致: " & 条
  部流[部] = 流
  身份集[身份] = 身份集.getOrDefault(身份, 0) + 1
  部次[部] = 部次.getOrDefault(部, 0) + 1

doAssert 身份集.len == 84, "身份重复"
doAssert 部次.len == 10, "部数应为 10"

var 内计 = 0
for 位 in 0 ..< 内部部.len:
  doAssert 部次[内部部[位]] == 内部数[位], "内部部数错: " & 内部部[位]
  内计 += 内部数[位]

var 外计 = 0
for 位 in 0 ..< 外部部.len:
  doAssert 部次[外部部[位]] == 外部数[位], "外部部数错: " & 外部部[位]
  外计 += 外部数[位]

doAssert 内计 == 42, "内部合计错"
doAssert 外计 == 42, "外部合计错"
doAssert 内计 + 外计 == 84, "总合计错"

let 计行 = 行.filterIt(it.startsWith("gttx://伦理计|"))
doAssert 计行.contains("gttx://伦理计|内部|42"), "内部结算行缺"
doAssert 计行.contains("gttx://伦理计|外部|42"), "外部结算行缺"
doAssert 计行.contains("gttx://伦理计|合计|84"), "总结算行缺"

let 裁行 = 行.filterIt(it.startsWith("gttx://伦理待裁|"))
doAssert 裁行.contains("gttx://伦理待裁|外部计数|源称34|逐行42|消息524"), "外部待裁行缺"
doAssert 裁行.contains("gttx://伦理待裁|合计计数|源称76|逐行84|消息526"), "合计待裁行缺"

echo "伦理表: 内部 42 (12+6+4+4+8+8) / 外部 42 (16+8+14+4) / 合计 84"
echo "身份: 全部带部名, 无数字编号, 同名跨部不冲突"
echo "流: 串联 / 并联 / 向上并联+外联 / 交叉并联+外联 / 向下串联+并联 / 向上级联+并联 / 交联 / 级联外延 / 声明流 / 世交长幼"
echo "待裁: 源称 34/76, 逐行 42/84, 未获裁决不采用源称"
echo "总验收: 过"
