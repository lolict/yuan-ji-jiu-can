# 微加载验收 · 沙箱内跑真命令 · 只 doAssert, 不写 if 分支判据
# 门: 去重 / 断点追加 / 幂等 / 指纹唯一 / 天平平 / 索命中 / 报可视 / 扫不重
import os, strutils, tables, sets, osproc, times

const CLI = "bin/微加载"

proc 跑(根, 命令: string): string =
  let 全 = CLI & " " & 命令 & " --根 " & quoteShell(根)
  let r = execCmdEx(全)
  doAssert r.exitCode == 0, "命令失败: " & 全 & "\n" & r.output
  result = r.output

proc 写(p, s: string) =
  writeFile(p, s)

proc 读索引(根: string): seq[tuple[源名: string, 行号: int, 指纹: string, 文本: string]] =
  result = @[]
  for 行 in readFile(根 / "索引.chain").splitLines():
    if not 行.startsWith("gttx://索引|"): continue
    let 域 = 行.split('|')
    doAssert 域.len == 5, "索引列数错: " & 行
    result.add (源名: 域[1], 行号: parseInt(域[2]), 指纹: 域[3], 文本: 域[4])

proc 读目录源(根: string): HashSet[string] =
  result = initHashSet[string]()
  for 行 in readFile(根 / "目录.chain").splitLines():
    if not 行.startsWith("gttx://目录|"): continue
    result.incl 行.split('|')[1]

proc 读状态(根: string): Table[string, tuple[已收, 入, 去: int]] =
  result = initTable[string, tuple[已收, 入, 去: int]]()
  for 行 in readFile(根 / "加载状态.chain").splitLines():
    if not 行.startsWith("gttx://状态|"): continue
    let 域 = 行.split('|')
    doAssert 域.len == 6, "状态列数错: " & 行
    result[域[1]] = (已收: parseInt(域[2]), 入: parseInt(域[3]), 去: parseInt(域[4]))

let 基 = "/tmp/opencode/微加载验" & $getCurrentProcessId() & "_" & $getTime().toUnix()
createDir(基)
写(基 / "目录.chain", "# 目录\n")
写(基 / "加载状态.chain", "# 状态\n")
写(基 / "索引.chain", "# 索引\n")
写(基 / "考核.chain", "# 考核\n")
写(基 / "微加载.chain", "# 微加载\n")
写(基 / "a.txt", "alpha\nshared\nbeta\ngamma\n")
写(基 / "b.txt", "shared\ndelta\nalpha\nepsilon\n")
写(基 / "目录.chain", readFile(基 / "目录.chain") &
  "gttx://目录|试验.a|a.txt|文本|试验\n" &
  "gttx://目录|试验.b|b.txt|文本|试验\n")

discard 跑(基, "收 试验.a")
discard 跑(基, "收 试验.b")

let 索1 = 读索引(基)
let 状1 = 读状态(基)
doAssert 索1.len == 6, "首轮唯一行应为 6"
doAssert 状1["试验.a"].已收 == 4, "a 已收错"
doAssert 状1["试验.a"].入 == 4, "a 入索引错"
doAssert 状1["试验.a"].去 == 0, "a 去重错"
doAssert 状1["试验.b"].已收 == 4, "b 已收错"
doAssert 状1["试验.b"].入 == 2, "b 入索引错"
doAssert 状1["试验.b"].去 == 2, "b 去重错"
doAssert 状1["试验.a"].入 + 状1["试验.a"].去 == 状1["试验.a"].已收, "a 天平不平"
doAssert 状1["试验.b"].入 + 状1["试验.b"].去 == 状1["试验.b"].已收, "b 天平不平"

var 总入 = 0
var 总去 = 0
var 总收 = 0
for _, s in 状1:
  总入 += s.入
  总去 += s.去
  总收 += s.已收
doAssert 总入 == 6, "全局入索引错"
doAssert 总去 == 2, "全局去重错"
doAssert 总收 == 8, "全局已收错"
doAssert 总入 + 总去 == 总收, "全局天平不平"

var 指纹集 = initHashSet[string]()
for 项 in 索1:
  doAssert not 指纹集.contains(项.指纹), "指纹重复: " & 项.指纹
  指纹集.incl 项.指纹
doAssert 指纹集.len == 6, "指纹唯一数错"

discard 跑(基, "收 试验.a")
doAssert 读索引(基).len == 6, "重复收同一源不应增行"

写(基 / "b.txt", "shared\ndelta\nalpha\nepsilon\nzeta\neta\n")
discard 跑(基, "收 试验.b")
let 索2 = 读索引(基)
let 状2 = 读状态(基)
doAssert 索2.len == 8, "断点追加后唯一行应为 8"
doAssert 状2["试验.b"].已收 == 6, "b 断点已收错"
doAssert 状2["试验.b"].入 == 4, "b 断点入索引错"
doAssert 状2["试验.b"].去 == 2, "b 断点去重错"
doAssert 状2["试验.b"].入 + 状2["试验.b"].去 == 状2["试验.b"].已收, "b 断点天平不平"
doAssert 状2["试验.a"].入 + 状2["试验.a"].去 == 状2["试验.a"].已收, "旧源天平被扰动"

let 搜 = 跑(基, "索 delta")
doAssert 搜.contains("delta"), "索未命中正文"
doAssert 搜.contains("试验.b"), "索未带源名"
doAssert 搜.contains("提示词"), "索未给提示词"

let 补 = 跑(基, "补 delta")
doAssert 补.contains("提示词草稿"), "补未给提示词草稿"
doAssert 补.contains("delta"), "补未命中正文"

let 报 = 跑(基, "报")
doAssert 报.contains("进度"), "报缺进度"
doAssert 报.contains("天平"), "报缺天平"
doAssert 报.contains("平"), "报缺平判"
doAssert 报.contains("自我感"), "报缺自我感"
doAssert 报.contains("认知感"), "报缺认知感"

discard 跑(基, "报 --html")
let 页 = 基 / "速阅.html"
doAssert fileExists(页), "速阅.html 未生成"
doAssert readFile(页).contains("微加载 · 全局速阅"), "速阅.html 缺标题"
doAssert readFile(页).contains("天平"), "速阅.html 缺天平"

discard 跑(基, "编目")
let 编1 = readFile(基 / "编目.chain")
doAssert 编1.contains("gttx://编目|1|"), "编目缺第1页"
doAssert 编1.contains("gttx://编目|2|"), "编目缺第2页"
doAssert not 编1.contains("gttx://编目|3|"), "编目页数应恰为2"
doAssert fileExists(基 / "冲突.chain"), "冲突.chain 未生成"
doAssert fileExists(基 / "关联.chain"), "关联.chain 未生成"
let 编页 = readFile(基 / "编目.html")
doAssert 编页.contains("微加载 · 编目"), "编目.html 缺标题"
doAssert 编页.contains("const PAGES="), "编目.html 缺页数据"
doAssert 编页.contains("const CONFLICTS="), "编目.html 缺冲突数据"
discard 跑(基, "编目")
doAssert readFile(基 / "编目.chain") == 编1, "编目应确定性"

createDir(基 / "文档")
写(基 / "文档" / "说明.md", "# 说明\n微加载 测试\n")
discard 跑(基, "扫 文档")
doAssert readFile(基 / "目录.chain").contains("说明.md"), "扫未登记新文档"
discard 跑(基, "扫 文档")
doAssert readFile(基 / "目录.chain").count("gttx://目录|文档.文档.说明.md|") == 1, "重复扫应幂等"

echo "去重: a 4 行 + b 4 行, 共享 2 行, 唯一 6 行"
echo "断点: b 追加 2 行后 唯一 8 行, 旧源天平不动"
echo "幂等: 重复收同一源不增索引"
echo "天平: 全局 入索引 + 去重 = 已收"
echo "速阅: 前缀命中并给提示词"
echo "补全: 前缀命中生成提示词草稿"
echo "可视: 速阅.html 生成, 含进度/天平/考核"
echo "扫: 新文档入目录, 重复扫不重"
# —— 真表门: 在册源全部已收, 指纹唯一, 天平差 0 ——
let 真根 = parentDir(getAppDir())
let 真目 = 读目录源(真根)
let 真索 = 读索引(真根)
let 真状 = 读状态(真根)
doAssert 真目.len > 0, "真表目录为空"
doAssert 真索.len > 0, "真表索引为空"

var 真入 = 0
var 真去 = 0
var 真收 = 0
for 源名, s in 真状:
  doAssert 真目.contains(源名), "状态源不在目录: " & 源名
  doAssert s.入 + s.去 == s.已收, "真表天平不平: " & 源名
  真入 += s.入
  真去 += s.去
  真收 += s.已收

doAssert 真入 + 真去 == 真收, "真表全局天平不平"
doAssert 真入 == 真索.len, "状态入索引与索引行数不符"
doAssert 真状.len == 真目.len, "在册源未全部已收"

var 真指纹 = initHashSet[string]()
for 项 in 真索:
  doAssert not 真指纹.contains(项.指纹), "真表指纹重复: " & 项.指纹
  真指纹.incl 项.指纹
  doAssert 真目.contains(项.源名), "索引源不在目录: " & 项.源名
doAssert 真指纹.len == 真索.len, "真表指纹唯一数不符"
doAssert 真状.hasKey("微加载.微加载.chain"), "自我感缺: 微加载.chain 未收"

discard 跑(真根, "编目")
let 真报 = 跑(真根, "报")
doAssert 真报.contains("差 0 平"), "真表报缺天平差0"
doAssert 真报.contains("认知感 过"), "真表报缺认知感过"
doAssert 真报.contains("编目感 过"), "真表报缺编目感过"
doAssert 真报.contains("冲突感 过"), "真表报缺冲突感过"
doAssert 真报.contains("关联感 过"), "真表报缺关联感过"
doAssert 真报.contains("致命错误 0 过"), "真表报缺致命错误0"

let 真编 = 真根 / "编目.chain"
var 真页号 = initHashSet[int]()
var 真编数 = 0
for 行 in readFile(真编).splitLines():
  if not 行.startsWith("gttx://编目|"): continue
  let 域 = 行.split('|')
  doAssert 域.len == 10, "编目列数错: " & 行
  let n = parseInt(域[1])
  doAssert not 真页号.contains(n), "编目页号重复: " & $n
  真页号.incl n
  doAssert 真目.contains(域[5]), "编目源不在目录: " & 域[5]
  真编数 += 1
doAssert 真编数 > 100, "真表编目页太少"

var 真冲数 = 0
for 行 in readFile(真根 / "冲突.chain").splitLines():
  if not 行.startsWith("gttx://冲突|"): continue
  let 域 = 行.split('|')
  doAssert 域.len == 8, "冲突列数错: " & 行
  doAssert 真页号.contains(parseInt(域[3])), "冲突甲页不存在: " & 域[3]
  doAssert 真页号.contains(parseInt(域[4])), "冲突乙页不存在: " & 域[4]
  真冲数 += 1
doAssert 真冲数 > 0, "真表无冲突"

var 真关数 = 0
for 行 in readFile(真根 / "关联.chain").splitLines():
  if not 行.startsWith("gttx://关联|"): continue
  let 域 = 行.split('|')
  doAssert 域.len == 5, "关联列数错: " & 行
  doAssert 真页号.contains(parseInt(域[1])), "关联甲页不存在: " & 域[1]
  doAssert 真页号.contains(parseInt(域[3])), "关联乙页不存在: " & 域[3]
  真关数 += 1
doAssert 真关数 > 0, "真表无关联"

let 真编页 = readFile(真根 / "编目.html")
doAssert 真编页.contains("微加载 · 编目"), "编目.html 缺标题"
doAssert 真编页.contains("待裁"), "编目.html 缺待裁"
doAssert 真编页.contains("const PAGES="), "编目.html 缺页数据"
doAssert readFile(真根 / "冲突.chain").contains("源称34 vs 逐行42"), "伦理冲突未生成"

echo "真表: 在册 " & $真目.len & " 源全部已收, 索引 " & $真索.len & " 行, 指纹唯一"
echo "真表天平: " & $真入 & " + " & $真去 & " = " & $真收 & " / 差 0"
echo "编目: " & $真编数 & " 页 / " & $真冲数 & " 冲突 / " & $真关数 & " 关联"
echo "总验收: 过"
