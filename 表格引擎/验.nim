import std/[strutils, os]
import 边界
import 表

proc 过(s: string): 行 =
  let r = 解析行(s)
  doAssert r.ok, "应通过: " & s & " / " & r.错
  result = r.行

proc 拒(s, 关键词: string) =
  let r = 解析行(s)
  doAssert not r.ok, "应拒收: " & s
  doAssert r.错.contains(关键词), "拒收原因不含「" & 关键词 & "」: " & r.错

let 基本 = 过("gttx://站|甲|乙")
doAssert 基本.表 == "站", "表名错"
doAssert 基本.单元s.len == 2, "顶层列数错"
doAssert 基本.单元s[0].类 == 文, "基本单元类型错"
doAssert 基本.单元s[0].文本 == "甲", "基本单元文本错"

let 引号 = 过("gttx://站|\"a|b,c{d};e\"|乙")
doAssert 引号.单元s.len == 2, "引号内分隔符被拆"
doAssert 引号.单元s[0].类 == 引, "引号单元类型错"
doAssert 引号.单元s[0].文本 == "a|b,c{d};e", "引号内容错"

let 转义 = 过("gttx://站|\"a\\\"b\"")
doAssert 转义.单元s[0].文本 == "a\"b", "引号转义错"

let 围墙 = 过("gttx://站|@1:6|{总管:3,视觉:1,听觉:2}")
doAssert 围墙.单元s.len == 2, "围墙行列数错"
doAssert 围墙.单元s[0].类 == 头, "头部类型错"
doAssert 围墙.单元s[0].索引 == "1", "头部索引错"
doAssert 围墙.单元s[0].容量 == 6, "头部容量错"
doAssert 围墙.单元s[1].类 == 墙, "围墙类型错"
doAssert 围墙.单元s[1].记录s.len == 1, "围墙记录数错"
doAssert 围墙.单元s[1].记录s[0].总管 == "总管", "总管名错"
doAssert 围墙.单元s[1].记录s[0].总管宽 == 3, "总管位宽错"
doAssert 围墙.单元s[1].记录s[0].域.len == 2, "位域数错"
doAssert 围墙.单元s[1].记录s[0].域[1].名 == "听觉", "位域名错"
doAssert 围墙.单元s[1].记录s[0].域[1].宽 == 2, "位域宽错"

let 双记 = 过("gttx://站|@1:9|{总管:3,视觉:1;总管:4,听觉:1}")
doAssert 双记.单元s[1].记录s.len == 2, "双记录数错"
doAssert 双记.单元s[1].记录s[1].总管 == "总管", "双记录总管错"
doAssert 双记.单元s[1].记录s[1].总管宽 == 4, "双记录总管宽错"

let 兼容 = 过("gttx://索引|微加载.微加载.chain|1|ABC|# 标题")
doAssert 兼容.表 == "索引", "兼容表名错"
doAssert 兼容.单元s.len == 4, "兼容列数错"
doAssert 兼容.单元s[3].文本 == "# 标题", "兼容文本错"

let 中括 = 过("gttx://站|文本{不是围墙}")
doAssert 中括.单元s[0].文本 == "文本{不是围墙}", "中段花括号被误判"

let 引头文 = 过("gttx://站|\"ADD64\" + 0x4889C3 = 加法容器位")
doAssert 引头文.单元s[0].类 == 文, "引号开头但后面有文字应作文本"
doAssert 引头文.单元s[0].文本 == "\"ADD64\" + 0x4889C3 = 加法容器位", "引头文本错"

let 代码墙 = 过("gttx://站|{FeedbackLoop::propagate → strength 衰减调整}")
doAssert 代码墙.单元s[0].类 == 文, "代码花括号应作文本"
doAssert 代码墙.单元s[0].文本 == "{FeedbackLoop::propagate → strength 衰减调整}", "代码花括号文本错"

let 空墙文 = 过("gttx://站|{}")
doAssert 空墙文.单元s[0].类 == 文, "空花括号应作文本"
doAssert 空墙文.单元s[0].文本 == "{}", "空花括号文本错"

let 缺宽文 = 过("gttx://站|{总管:x}")
doAssert 缺宽文.单元s[0].类 == 文, "无正整数位宽的花括号应作文本"
doAssert 缺宽文.单元s[0].文本 == "{总管:x}", "无位宽文本错"

let 单引文 = 过("gttx://站|\"#)?;")
doAssert 单引文.单元s[0].类 == 文, "单个引号应作文本"
doAssert 单引文.单元s[0].文本 == "\"#)?;", "单个引号文本错"

let 注释 = 解析行("# 这是注释")
doAssert 注释.ok and 注释.注释, "注释未跳过"

for 行s in readFile("边界.chain").splitLines():
  let r = 解析行(行s)
  doAssert r.ok, "自描述表解析失败: " & 行s & " / " & r.错

拒("gttx://站|{总管:3", "围墙")

let 严引 = 解析行("gttx://站|\"abc", 严格 = true)
doAssert not 严引.ok and 严引.错.contains("引号"), "严格模式应拒未闭合引号"
let 严墙 = 解析行("gttx://站|{总管:x}", 严格 = true)
doAssert not 严墙.ok and 严墙.错.contains("围墙"), "严格模式应拒坏围墙"
let 严空 = 解析行("gttx://站|{}", 严格 = true)
doAssert not 严空.ok and 严空.错.contains("围墙"), "严格模式应拒空围墙"
let 严尾 = 解析行("gttx://站|\"ADD64\" + 0x4889C3", 严格 = true)
doAssert not 严尾.ok and 严尾.错.contains("引号"), "严格模式应拒引号后多字"
拒("gttx://站|@1:4|{总管:3,视觉:2}", "容量")
拒("gttx://站|@1:8|甲", "头部")
拒("gttx://站|@1:8", "头部")
拒("gttx://站|@:8|{总管:8}", "头部")
拒("gttx://站|@1:x|{总管:8}", "容量")
拒("gttx://站||甲", "空单元")
拒("gttx://站|甲|", "尾随")
拒("gttx://站", "缺表名分隔")

let 模式v = 读模式("schema.chain")
doAssert 模式v.表s.len == 11, "schema 表数错"
doAssert 找表(模式v, "写入") >= 0, "schema 缺写入"
doAssert 有列(模式v, "站", "地址"), "schema 缺站.地址"
doAssert 有列(模式v, "写入", "验收"), "schema 缺写入.验收"
doAssert 有列(模式v, "坐标", "位"), "schema 缺坐标.位"
doAssert 有列(模式v, "刻", "当前"), "schema 缺刻.当前"
doAssert 有列(模式v, "刻", "候车"), "schema 缺刻.候车"
doAssert 有列(模式v, "格", "值"), "schema 缺格.值"
doAssert 有列(模式v, "线", "耗时"), "schema 缺线.耗时"
doAssert 有列(模式v, "线", "候车"), "schema 缺线.候车"
doAssert 有列(模式v, "站", "候车"), "schema 缺站.候车"
doAssert 有列(模式v, "站", "换乘"), "schema 缺站。换乘"

var e = 新引擎(模式v)
doAssert 写(e, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "T1"), "首行写入失败"
doAssert e.站s.len == 1, "站数错"
doAssert e.站s[0].地址 == 1, "地址错"
doAssert e.站s[0].符号 == "甲", "符号错"
doAssert e.站s[0].部 == "未定", "部错"
doAssert e.站s[0].状态 == "待定", "状态错"
doAssert e.动作s.len == 1, "动作数错"
doAssert e.路口s.len == 1, "路口数错"
doAssert e.写入s.len == 1, "写入数错"
doAssert e.拒收s.len == 0, "首行不应拒收"
doAssert e.下一地址 == 2, "下一地址错"

doAssert not 写(e, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "T2"), "重复符号应拒"
doAssert e.拒收s.len == 1 and e.拒收s[0].原因.contains("符号重复"), "重复符号原因错"

doAssert not 写(e, "gttx://写入|乙|动作B|客.状态 = 待定", "T3"), "缺列应拒"
doAssert e.拒收s[1].原因.contains("列数"), "缺列原因错"

doAssert not 写(e, "gttx://写入|乙|\"\"|客.状态 = 待定|客.状态 = 已上车", "T4"), "空动作应拒"
doAssert e.拒收s[2].原因.contains("缺动作"), "空动作原因错"

doAssert not 写(e, "gttx://写入|甲 乙|动作A|客.状态 = 待定|客.状态 = 已上车", "T5"), "坏符号应拒"
doAssert e.拒收s[3].原因.contains("符号不合法"), "坏符号原因错"

doAssert not 写(e, "gttx://写入|乙|动作B|客状态|客.状态 = 已上车", "T6"), "坏条件应拒"
doAssert e.拒收s[4].原因.contains("条件格式"), "坏条件原因错"

doAssert not 写(e, "gttx://写入|乙|动作B|客.不存在 = 1|客.状态 = 已上车", "T7"), "未知条件字段应拒"
doAssert e.拒收s[5].原因.contains("条件表或字段"), "未知条件字段原因错"

doAssert not 写(e, "gttx://写入|乙|动作B|客.状态 = 待定|客.不存在 = 1", "T8"), "未知验收字段应拒"
doAssert e.拒收s[6].原因.contains("验收表或字段"), "未知验收字段原因错"

doAssert 写(e, "gttx://写入|乙|动作B|站.状态 = 待定|站.状态 = 已定", "T9"), "第二行写入失败"
doAssert e.站s.len == 2 and e.站s[1].地址 == 2, "第二站错"
doAssert e.动作s.len == 2, "第二动作错"
doAssert e.路口s.len == 2, "第二路口错"
doAssert e.写入s.len == 2, "第二写入错"
doAssert e.拒收s.len == 7, "拒收数错"

doAssert 写(e, "gttx://写入|丙|动作A|客.状态 = 待定|客.状态 = 已上车", "T10"), "第三行写入失败"
doAssert e.站s.len == 3 and e.站s[2].地址 == 3, "第三站错"
doAssert e.动作s.len == 2, "同动作应复用"
doAssert e.写入s.len == 3, "第三写入错"
doAssert e.路口s[2].当前站 == "丙" and e.路口s[2].动作 == "动作A", "第三路口错"

doAssert 合法符号("甲"), "合法符号误判"
doAssert not 合法符号("甲 乙"), "空格符号应不合法"
doAssert not 合法符号("甲|乙"), "竖号符号应不合法"

let 出 = "/tmp/opencode/表格引擎/出验"
存(e, 出)
doAssert fileExists(出 / "站.chain"), "缺站表"
doAssert fileExists(出 / "动作.chain"), "缺动作表"
doAssert fileExists(出 / "路口.chain"), "缺路口表"
doAssert fileExists(出 / "写入.chain"), "缺写入表"
doAssert fileExists(出 / "拒收.chain"), "缺拒收表"
doAssert fileExists(出 / "线.chain"), "缺线表"
doAssert fileExists(出 / "客.chain"), "缺客表"
doAssert fileExists(出 / "调用.chain"), "缺调用表"
doAssert fileExists(出 / "坐标.chain"), "缺坐标表"
doAssert fileExists(出 / "刻.chain"), "缺刻表"
doAssert fileExists(出 / "格.chain"), "缺格表"
doAssert readFile(出 / "站.chain").strip.splitLines().len == 4, "站表行数错"
doAssert readFile(出 / "动作.chain").strip.splitLines().len == 3, "动作表行数错"
doAssert readFile(出 / "路口.chain").strip.splitLines().len == 4, "路口表行数错"
doAssert readFile(出 / "写入.chain").strip.splitLines().len == 4, "写入表行数错"
doAssert readFile(出 / "拒收.chain").strip.splitLines().len == 8, "拒收表行数错"
doAssert readFile(出 / "线.chain").strip.splitLines().len == 1, "线表行数错"
doAssert readFile(出 / "客.chain").strip.splitLines().len == 1, "客表行数错"
doAssert readFile(出 / "调用.chain").strip.splitLines().len == 1, "调用表行数错"
doAssert readFile(出 / "坐标.chain").strip.splitLines().len == 1, "坐标表行数错"
doAssert readFile(出 / "刻.chain").strip.splitLines().len == 2, "刻表行数错"
doAssert readFile(出 / "格.chain").strip.splitLines().len == 1, "格表行数错"
doAssert readFile(出 / "站.chain").contains("gttx://站|1|甲|未定|\"\"|\"\"|待定"), "站行内容错"
doAssert readFile(出 / "拒收.chain").contains("符号重复"), "拒收原因丢失"
for f in ["站.chain", "动作.chain", "路口.chain", "写入.chain", "拒收.chain", "线.chain", "客.chain", "调用.chain", "坐标.chain", "刻.chain", "格.chain"]:
  for 行s in readFile(出 / f).strip.splitLines():
    let r = 解析行(行s, 严格 = true)
    doAssert r.ok, f & " 回读失败: " & 行s & " / " & r.错

var e2 = 新引擎(模式v)
doAssert 写(e2, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "S1"), "主循环甲失败"
doAssert 写(e2, "gttx://写入|乙|动作B|客.状态 = 待定|客.状态 = 已上车", "S2"), "主循环乙失败"
doAssert 设下一站(e2, "甲", "乙"), "设下一站失败"
doAssert not 设下一站(e2, "甲", "丙"), "未知站应拒"
doAssert 上车(e2, "R1", "甲", "乙"), "上车失败"
doAssert not 上车(e2, "R1", "甲", "乙"), "重复请求应拒"
doAssert 找客(e2, "R1") == 0, "客行缺失"
doAssert not 一步(e2, "无此请求"), "未知请求应停"
doAssert 一步(e2, "R1"), "第一步失败"
doAssert e2.客s[0].当前站 == "乙", "未到乙"
doAssert e2.客s[0].状态 == "已验收", "验收未过"
doAssert not 一步(e2, "R1"), "到站后仍前进"
doAssert e2.客s[0].状态 == "到站", "到站状态错"
let 出2 = "/tmp/opencode/表格引擎/出验2"
存(e2, 出2)
doAssert readFile(出2 / "客.chain").contains("gttx://客|R1|乙|乙|是|到站|\"\"|\"\""), "客表内容错"
for 行s in readFile(出2 / "客.chain").strip.splitLines():
  let r = 解析行(行s, 严格 = true)
  doAssert r.ok, "客表回读失败: " & 行s & " / " & r.错

let e3 = 读(出, 模式v)
doAssert e3.站s.len == e.站s.len, "回读站数错"
doAssert e3.动作s.len == e.动作s.len, "回读动作数错"
doAssert e3.路口s.len == e.路口s.len, "回读路口数错"
doAssert e3.写入s.len == e.写入s.len, "回读写入数错"
doAssert e3.拒收s.len == e.拒收s.len, "回读拒收数错"
doAssert e3.站s[0].符号 == "甲" and e3.站s[0].地址 == 1, "回读站内容错"
doAssert e3.路口s[0].条件行 == "客.状态 = 待定", "回读条件错"
doAssert e3.拒收s[0].原因.contains("符号重复"), "回读拒收错"
doAssert e3.下一地址 == 4, "回读下一地址错"
doAssert e3.刻 == -1, "回读刻错"

var e4 = 读(出2, 模式v)
doAssert e4.客s.len == 1, "回读客数错"
doAssert e4.客s[0].当前站 == "乙" and e4.客s[0].目标站 == "乙", "回读客位置错"
doAssert e4.客s[0].可上车 == "是" and e4.客s[0].状态 == "到站", "回读客状态错"
doAssert e4.路口s[0].下一站 == "乙", "回读路线错"
doAssert not 一步(e4, "R1"), "到站回读后仍前进"

var e5 = 新引擎(模式v)
doAssert 写(e5, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "L1"), "排班站甲失败"
doAssert 写(e5, "gttx://写入|乙|动作B|客.状态 = 待定|客.状态 = 已上车", "L2"), "排班站乙失败"
doAssert 排班(e5, "甲", "乙", "前", "车", "08:00", 2), "排班失败"
doAssert 找线(e5, "甲", "乙") == 0, "找线失败"
doAssert e5.线s[0].容量 == 2 and e5.线s[0].载具 == "车", "线内容错"
doAssert not 排班(e5, "甲", "丙", "前", "车", "08:10", 2), "未知止站应拒"
doAssert not 排班(e5, "甲", "甲", "前", "车", "08:20", 2), "同站应拒"
doAssert not 排班(e5, "甲", "乙", "前", "车", "08:30", 0), "零容量应拒"
doAssert 线键(e5.线s[0]) == "甲>乙@08:00", "线键错"
doAssert 找线键(e5, "甲>乙@08:00") == 0, "找线键失败"
doAssert not 排班(e5, "甲", "乙", "前", "车", "08:00", 2), "重复班次应拒"
doAssert 上车(e5, "R2", "甲", "乙"), "R2 上车失败"
doAssert 一步(e5, "R2"), "R2 登车失败"
doAssert e5.客s[0].状态 == "在车上", "R2 未在车上"
doAssert e5.客s[0].线号 == "甲>乙@08:00", "R2 线号错"
doAssert e5.客s[0].当前站 == "甲", "登车后不应立刻移动"
doAssert 线上人数(e5, "甲>乙@08:00") == 1, "线上人数错"
doAssert 一步(e5, "R2"), "R2 乘车失败"
doAssert e5.客s[0].当前站 == "乙", "R2 未到乙"
doAssert e5.客s[0].状态 == "到站", "R2 未到站"
doAssert e5.客s[0].线号 == "", "R2 下车未清线号"
doAssert 线上人数(e5, "甲>乙@08:00") == 0, "下车后人数错"
doAssert not 一步(e5, "R2"), "到站后仍前进"

doAssert 上车(e5, "R3", "甲", "乙"), "R3 上车失败"
doAssert 上车(e5, "R4", "甲", "乙"), "R4 上车失败"
doAssert 上车(e5, "R5", "甲", "乙"), "R5 上车失败"
doAssert 一步(e5, "R3") and 一步(e5, "R4"), "R3/R4 登车失败"
doAssert 线上人数(e5, "甲>乙@08:00") == 2, "满员人数错"
doAssert 找可乘线(e5, "R5") < 0, "满员仍可乘"
doAssert not 一步(e5, "R5"), "满员不应登车"
doAssert e5.客s[3].状态 == "无下一站", "满员回退状态错"
doAssert 一步(e5, "R3"), "R3 未发车"
doAssert 线上人数(e5, "甲>乙@08:00") == 1, "发车后人数错"
doAssert 找可乘线(e5, "R5") == 0, "空位未释放"
doAssert 一步(e5, "R5"), "R5 未补位"
doAssert e5.客s[3].状态 == "在车上", "R5 补位状态错"

let 出5 = "/tmp/opencode/表格引擎/出验5"
存(e5, 出5)
doAssert readFile(出5 / "线.chain").strip.splitLines().len == 2, "线表排班行数错"
let e6 = 读(出5, 模式v)
doAssert e6.线s.len == 1 and e6.线s[0].止站 == "乙" and e6.线s[0].时刻 == "08:00", "线表回读错"

var e7 = 新引擎(模式v)
doAssert 写(e7, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "C1"), "调用站甲失败"
doAssert 写(e7, "gttx://写入|乙|动作B|站.状态 = 待定|站.状态 = 待定", "C2"), "调用站乙失败"
doAssert 写(e7, "gttx://写入|丙|动作C|站.状态 = 待定|站.状态 = 待定", "C3"), "调用站丙失败"
doAssert 写(e7, "gttx://写入|丁|动作D|站.状态 = 待定|站.状态 = 待定", "C4"), "调用站丁失败"
doAssert 加调用(e7, "甲", "乙", "丙", "P1", ""), "加调用失败"
doAssert not 加调用(e7, "甲", "乙", "丙", "P2", ""), "重复调用应拒"
doAssert not 加调用(e7, "甲", "甲", "丙", "P3", ""), "同站调用应拒"
doAssert not 加调用(e7, "甲", "乙", "乙", "P4", ""), "同返回站应拒"
doAssert not 加调用(e7, "甲", "乙", "无此站", "P5", ""), "未知返回站应拒"
doAssert 调用键(e7.调用s[0]) == "甲>乙@丙", "调用键错"
doAssert 找调用键(e7, "甲>乙@丙") == 0 and 找调用(e7, "甲") == 0, "找调用失败"
doAssert 设下一站(e7, "乙", "丙") and 设下一站(e7, "丙", "丁"), "调用路线设置失败"
doAssert 上车(e7, "C1", "甲", "丁"), "C1 上车失败"
doAssert 一步(e7, "C1"), "发起调用失败"
doAssert e7.客s[0].当前站 == "乙", "调用未进入被调站"
doAssert e7.客s[0].状态 == "调用中", "调用状态错"
doAssert e7.客s[0].调用号 == "甲>乙@丙", "调用号错"
doAssert not 发起调用(e7, "C1"), "调用中不应重入"
doAssert 一步(e7, "C1"), "被调站前进失败"
doAssert e7.客s[0].当前站 == "丙" and e7.客s[0].状态 == "调用中", "调用中移动丢状态"
doAssert e7.客s[0].调用号 == "甲>乙@丙", "调用中移动丢调用号"
let 出7 = "/tmp/opencode/表格引擎/出验7"
存(e7, 出7)
doAssert readFile(出7 / "调用.chain").strip.splitLines().len == 2, "调用表行数错"
var e8 = 读(出7, 模式v)
doAssert e8.调用s.len == 1 and e8.客s[0].调用号 == "甲>乙@丙", "调用回读错"
doAssert e8.客s[0].状态 == "调用中" and e8.客s[0].当前站 == "丙", "调用状态回读错"
doAssert 一步(e8, "C1"), "调用返回失败"
doAssert e8.客s[0].状态 == "调用返回" and e8.客s[0].调用号 == "", "返回状态错"
doAssert 一步(e8, "C1"), "返回后继续失败"
doAssert e8.客s[0].当前站 == "丁", "返回后未到丁"
doAssert not 一步(e8, "C1"), "到目标后仍前进"
doAssert e8.客s[0].状态 == "到站", "最终未到站"

var e9 = 新引擎(模式v)
doAssert 写(e9, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "A1"), "动作站甲失败"
doAssert 写(e9, "gttx://写入|乙|动作B|客.状态 = 待定|客.状态 = 已上车", "A2"), "动作站乙失败"
doAssert 执行动作(e9, "动作A", "X"), "空动作应通过"
e9.动作s[0].程序 = "站.状态=已定;客.可上车=否"
doAssert 设下一站(e9, "甲", "乙"), "动作路线失败"
doAssert 上车(e9, "X", "甲", "乙"), "动作客上车失败"
doAssert 执行动作(e9, "动作A", "X"), "执行动作失败"
doAssert 一步(e9, "X"), "动作步进失败"
doAssert e9.客s[0].当前站 == "乙", "动作未到乙"
doAssert e9.站s[1].状态 == "已定", "站动作未执行"
doAssert e9.客s[0].可上车 == "否", "客动作未执行"
doAssert e9.客s[0].状态 == "已验收", "动作后验收失败"
e9.动作s[0].程序 = "无等号"
doAssert not 执行动作(e9, "动作A", "X"), "坏动作应拒"
e9.动作s[0].程序 = "站.不存在=1"
doAssert not 执行动作(e9, "动作A", "X"), "未知字段应拒"
doAssert not 设值(e9, "X", "客", "不存在", "1"), "未知客字段应拒"
e9.动作s[0].程序 = "站.状态=已定;客.可上车=否"
let 出9 = "/tmp/opencode/表格引擎/出验9"
存(e9, 出9)
let e10 = 读(出9, 模式v)
doAssert e10.动作s[0].程序 == "站.状态=已定;客.可上车=否", "动作程序回读错"
doAssert e10.站s[1].状态 == "已定", "站状态回读错"
doAssert e10.客s[0].可上车 == "否", "客字段回读错"

var e11 = 新引擎(模式v)
doAssert 写(e11, "gttx://写入|甲|动作A|客.状态 = 无|客.状态 = 已上车", "J1"), "岔口站甲失败"
doAssert 写(e11, "gttx://写入|乙|动作B|客.状态 = 待定|客.状态 = 已上车", "J2"), "岔口站乙失败"
doAssert 写(e11, "gttx://写入|丙|动作C|客.状态 = 待定|客.状态 = 已上车", "J3"), "岔口站丙失败"
doAssert 加路口(e11, "甲", "", "客.可上车 = 否", "丙", "动作C", "客.状态 = 已上车"), "加岔口一失败"
doAssert 加路口(e11, "甲", "", "客.可上车 = 是", "乙", "动作B", "客.状态 = 已上车"), "加岔口二失败"
doAssert not 加路口(e11, "甲", "", "客.可上车 = 是", "乙", "动作B", "客.状态 = 已上车"), "重复岔口应拒"
doAssert not 加路口(e11, "无此站", "", "客.状态 = 待定", "乙", "动作B", "客.状态 = 已上车"), "未知站岔口应拒"
doAssert not 加路口(e11, "甲", "", "客.状态 = 待定", "乙", "无此动作", "客.状态 = 已上车"), "未知动作岔口应拒"
doAssert not 加路口(e11, "甲", "", "客.不存在 = 1", "乙", "动作B", "客.状态 = 已上车"), "未知字段岔口应拒"
doAssert e11.路口s.len == 5, "岔口行数错"
doAssert 上车(e11, "X", "甲", "乙"), "岔口客上车失败"
doAssert 一步(e11, "X"), "岔口步进失败"
doAssert e11.客s[0].当前站 == "乙", "岔口未选对分支"
doAssert e11.客s[0].状态 == "已验收", "岔口验收失败"
let 出11 = "/tmp/opencode/表格引擎/出验11"
存(e11, 出11)
let e12 = 读(出11, 模式v)
doAssert e12.路口s.len == 5, "岔口回读行数错"
doAssert e12.客s[0].当前站 == "乙" and e12.客s[0].状态 == "已验收", "岔口回读状态错"

var e13 = 新引擎(模式v)
doAssert 写(e13, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "Z1"), "坐标站甲失败"
doAssert 写(e13, "gttx://写入|乙|动作B|客.状态 = 待定|客.状态 = 已上车", "Z2"), "坐标站乙失败"
doAssert 写(e13, "gttx://写入|丙|动作C|客.状态 = 待定|客.状态 = 已上车", "Z3"), "坐标站丙失败"
doAssert 排班(e13, "甲", "乙", "前", "车", "09:00", 2), "坐标线一失败"
let 键1 = 线键(e13.线s[0])
doAssert 加坐标(e13, "甲", 键1, 0, 0), "坐标甲失败"
doAssert 加坐标(e13, "乙", 键1, 0, 1), "坐标乙失败"
doAssert not 加坐标(e13, "乙", 键1, 0, 1), "重复坐标应拒"
doAssert not 加坐标(e13, "无此站", 键1, 0, 0), "未知站坐标应拒"
doAssert not 加坐标(e13, "甲", "无此线", 0, 0), "未知线坐标应拒"
doAssert not 加坐标(e13, "甲", 键1, -1, 0), "负段坐标应拒"
doAssert not 加坐标(e13, "甲", 键1, 0, -1), "负位坐标应拒"
doAssert 找坐标(e13, "甲", 键1) == 0, "找坐标失败"
doAssert 按址找站(e13, 键1, 0, 1) == "乙", "按址找站失败"
doAssert 站坐标(e13, "甲").len == 1, "站坐标数错"
doAssert 交叉数(e13, "甲") == 1, "单线交叉数错"
doAssert 排班(e13, "甲", "丙", "前", "车", "09:10", 2), "坐标线二失败"
let 键2 = 线键(e13.线s[1])
doAssert 加坐标(e13, "甲", 键2, 0, 0), "坐标甲二失败"
doAssert 交叉数(e13, "甲") == 2, "交叉数错"
doAssert 按址找站(e13, 键2, 0, 0) == "甲", "按址找站二失败"
let 出13 = "/tmp/opencode/表格引擎/出验13"
存(e13, 出13)
doAssert readFile(出13 / "坐标.chain").strip.splitLines().len == 4, "坐标表行数错"
let e14 = 读(出13, 模式v)
doAssert e14.坐标s.len == 3, "坐标回读数错"
doAssert 找坐标(e14, "甲", 键1) == 0, "坐标回读定位错"
doAssert 交叉数(e14, "甲") == 2, "坐标回读交叉数错"

doAssert 时分钟("09:00") == 540, "时分钟错"
doAssert 时分钟("24:00") == -1 and 时分钟("09:60") == -1, "时分钟越界错"
var e15 = 新引擎(模式v)
doAssert 写(e15, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "T1"), "刻站甲失败"
doAssert 写(e15, "gttx://写入|乙|动作B|客.状态 = 待定|客.状态 = 已上车", "T2"), "刻站乙失败"
doAssert 排班(e15, "甲", "乙", "前", "车", "09:00", 2), "刻班次失败"
doAssert not 排班(e15, "甲", "乙", "前", "车", "9点", 2), "坏时刻应拒"
doAssert 上车(e15, "T1", "甲", "乙"), "刻客上车失败"
doAssert 一步(e15, "T1") and 一步(e15, "T1"), "无刻时不应阻塞"
doAssert e15.客s[0].状态 == "到站", "无刻到站错"

var e16 = 新引擎(模式v)
doAssert 写(e16, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "T3"), "时刻站甲失败"
doAssert 写(e16, "gttx://写入|乙|动作B|客.状态 = 待定|客.状态 = 已上车", "T4"), "时刻站乙失败"
doAssert 排班(e16, "甲", "乙", "前", "车", "09:00", 2), "时刻班次失败"
doAssert 设刻(e16, "08:30"), "设刻失败"
doAssert not 设刻(e16, "25:00"), "坏刻应拒"
doAssert 上车(e16, "T2", "甲", "乙"), "时刻客上车失败"
doAssert 一步(e16, "T2"), "时刻登车失败"
doAssert e16.客s[0].状态 == "在车上", "时刻未登车"
doAssert not 一步(e16, "T2"), "未到点不应发车"
doAssert e16.客s[0].状态 == "候车", "候车状态错"
doAssert 设刻(e16, "09:00"), "到点设刻失败"
doAssert 一步(e16, "T2"), "到点未发车"
doAssert e16.客s[0].状态 == "到站", "到点未到站"
doAssert 排班(e16, "乙", "甲", "后", "车", "09:00", 2), "返程班次失败"
doAssert 设刻(e16, "09:30"), "过点设刻失败"
doAssert 上车(e16, "T3", "乙", "甲"), "过点客上车失败"
doAssert 找可乘线(e16, "T3") < 0, "过点仍可乘"
doAssert 下一站(e16, "T3") == "", "过点仍进寻路"
doAssert not 一步(e16, "T3"), "过点不应登车"
let 出16 = "/tmp/opencode/表格引擎/出验16"
存(e16, 出16)
let e17 = 读(出16, 模式v)
doAssert e17.刻 == 570, "刻回读错"
doAssert e17.线s.len == 2, "刻回读线数错"

var e18 = 新引擎(模式v)
doAssert 写(e18, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "R1"), "址路站甲失败"
doAssert 写(e18, "gttx://写入|乙|动作B|客.状态 = 待定|客.状态 = 已上车", "R2"), "址路站乙失败"
doAssert 排班(e18, "甲", "乙", "前", "车", "10:00", 2), "址路班次失败"
let 键3 = 线键(e18.线s[0])
doAssert 加坐标(e18, "甲", 键3, 0, 0), "址路坐标甲失败"
doAssert 加坐标(e18, "乙", 键3, 0, 1), "址路坐标乙失败"
doAssert 解址(e18, "甲") == "甲", "解址站名失败"
doAssert 解址(e18, 键3 & "#0#1") == "乙", "解址坐标失败"
doAssert 解址(e18, 键3 & "#9#9") == "", "解址越界应空"
doAssert not 设下一站(e18, "甲", 键3 & "#9#9"), "坏坐标路应拒"
doAssert 设下一站(e18, "甲", 键3 & "#0#1"), "坐标设路失败"
doAssert 设刻(e18, "11:00"), "址路过点设刻失败"
doAssert 上车(e18, "X", "甲", "乙"), "址路客上车失败"
doAssert 一步(e18, "X"), "址路步进失败"
doAssert e18.客s[0].当前站 == "乙", "址路未按坐标到乙"
doAssert e18.客s[0].状态 == "已验收", "址路验收失败"
let 出18 = "/tmp/opencode/表格引擎/出验18"
存(e18, 出18)
var e19 = 读(出18, 模式v)
doAssert e19.路口s[0].下一站 == 键3 & "#0#1", "坐标路回读错"
doAssert e19.客s[0].当前站 == "乙", "坐标路客回读错"
doAssert not 一步(e19, "X"), "坐标路到站后仍前进"
doAssert e19.客s[0].状态 == "到站", "坐标路最终未到站"

var e20 = 新引擎(模式v)
doAssert 写(e20, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "G1"), "格站甲失败"
doAssert 写(e20, "gttx://写入|乙|动作B|客.状态 = 待定|客.状态 = 已上车", "G2"), "格站乙失败"
doAssert 排班(e20, "甲", "乙", "前", "车", "10:00", 2), "格班次失败"
let 键4 = 线键(e20.线s[0])
doAssert 加格(e20, 键4, 0, 0, "动作A", "10:00", "甲"), "加格失败"
doAssert not 加格(e20, 键4, 0, 0, "动作A", "10:00", "甲"), "重复格应拒"
doAssert not 加格(e20, "无此线", 0, 0, "动作A", "10:00", "甲"), "未知线格应拒"
doAssert not 加格(e20, 键4, 0, 0, "无此动作", "10:00", "甲"), "未知动作格应拒"
doAssert not 加格(e20, 键4, 0, 0, "动作A", "10:99", "甲"), "坏时刻格应拒"
doAssert not 加格(e20, 键4, -1, 0, "动作A", "10:00", "甲"), "负段格应拒"
doAssert not 加格(e20, 键4, 0, 0, "动作A", "10:00", ""), "空格值应拒"
doAssert 找格(e20, 键4, 0, 0, "动作A", "10:00") == 0, "找格失败"
doAssert 格值(e20, 键4, 0, 0, "动作A", "10:00") == "甲", "格值错"
doAssert 格值(e20, 键4, 9, 9, "动作A", "10:00") == "", "空格值应为空"
let 出20 = "/tmp/opencode/表格引擎/出验20"
存(e20, 出20)
doAssert readFile(出20 / "格.chain").strip.splitLines().len == 2, "格表行数错"
let e21 = 读(出20, 模式v)
doAssert e21.格s.len == 1, "格回读数错"
doAssert 格值(e21, 键4, 0, 0, "动作A", "10:00") == "甲", "格回读值错"

var e22 = 新引擎(模式v)
doAssert 写(e22, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "K1"), "格步站甲失败"
doAssert 写(e22, "gttx://写入|乙|动作B|客.状态 = 待定|客.状态 = 已上车", "K2"), "格步站乙失败"
doAssert 排班(e22, "甲", "乙", "前", "车", "10:00", 2), "格步班次失败"
let 键5 = 线键(e22.线s[0])
doAssert 加坐标(e22, "甲", 键5, 0, 0), "格步坐标甲失败"
doAssert 加坐标(e22, "乙", 键5, 0, 1), "格步坐标乙失败"
doAssert 加格(e22, 键5, 0, 0, "动作A", "10:00", 键5 & "#0#1"), "格步加格失败"
doAssert 设刻(e22, "09:59"), "格步早刻失败"
doAssert 找格时(e22, "甲") < 0, "未到点不应命中格"
doAssert 设刻(e22, "10:00"), "格步到点失败"
doAssert 找格时(e22, "甲") == 0, "到点未命中格"
doAssert 找格时(e22, "乙") < 0, "乙不应命中格"
doAssert 上车(e22, "X", "甲", "乙"), "格步客上车失败"
doAssert 一步(e22, "X"), "格步执行失败"
doAssert e22.客s[0].当前站 == "乙", "格步未按值移动"
doAssert e22.客s[0].状态 == "到站", "格步未到站"
doAssert not 一步(e22, "X"), "格步到站后仍前进"
let 出22 = "/tmp/opencode/表格引擎/出验22"
存(e22, 出22)
let e23 = 读(出22, 模式v)
doAssert 格值(e23, 键5, 0, 0, "动作A", "10:00") == 键5 & "#0#1", "格步值回读错"
doAssert 找格时(e23, "甲") == 0, "格步回读未命中"
doAssert e23.客s[0].状态 == "到站", "格步客回读错"

var e24 = 新引擎(模式v)
doAssert 写(e24, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "L1"), "候车甲失败"
doAssert 写(e24, "gttx://写入|乙|动作B|客.状态 = 待定|客.状态 = 已上车", "L2"), "候车乙失败"
doAssert 排班(e24, "甲", "乙", "前", "车", "10:00", 2), "候车班次失败"
doAssert 设刻(e24, "09:00"), "候车设刻失败"
doAssert 上车(e24, "X", "甲", "乙"), "候车客失败"
doAssert 一步(e24, "X"), "候车登车失败"
doAssert e24.客s[0].状态 == "在车上", "候车未登车"
doAssert not 一步(e24, "X"), "候车不应发车"
doAssert e24.刻 == 600, "候车未自动推进到发车刻"
doAssert e24.客s[0].状态 == "候车", "候车状态错"
doAssert 一步(e24, "X"), "候车到点未发车"
doAssert e24.客s[0].状态 == "到站", "候车未到站"

var e25 = 新引擎(模式v)
doAssert 写(e25, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "C1"), "候格甲失败"
doAssert 写(e25, "gttx://写入|乙|动作B|客.状态 = 待定|客.状态 = 已上车", "C2"), "候格乙失败"
doAssert 排班(e25, "甲", "乙", "前", "车", "08:59", 2), "候格班次失败"
let 键6 = 线键(e25.线s[0])
doAssert 加坐标(e25, "甲", 键6, 0, 0), "候格坐标甲失败"
doAssert 加坐标(e25, "乙", 键6, 0, 1), "候格坐标乙失败"
doAssert 加格(e25, 键6, 0, 0, "动作A", "10:00", 键6 & "#0#1"), "候格加格失败"
doAssert 设刻(e25, "09:00"), "候格设刻失败"
doAssert 上车(e25, "X", "甲", "乙"), "候格客失败"
doAssert 下一事件刻(e25, "X") == 600, "候格下一事件错"
doAssert not 一步(e25, "X"), "候格不应直接到站"
doAssert e25.刻 == 600, "候格未自动推进"
doAssert e25.客s[0].状态 == "候刻", "候格状态错"
doAssert 一步(e25, "X"), "候格到点未执行"
doAssert e25.客s[0].状态 == "到站", "候格未到站"

var e26 = 新引擎(模式v)
doAssert 写(e26, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "P1"), "寻路甲失败"
doAssert 写(e26, "gttx://写入|乙|动作B|客.状态 = 待定|客.状态 = 已上车", "P2"), "寻路乙失败"
doAssert 写(e26, "gttx://写入|丙|动作C|客.状态 = 待定|客.状态 = 已上车", "P3"), "寻路丙失败"
doAssert 排班(e26, "甲", "乙", "前", "车", "09:00", 2), "寻路线一失败"
doAssert 排班(e26, "乙", "丙", "前", "车", "09:00", 2), "寻路线二失败"
doAssert 上车(e26, "X", "甲", "丙"), "寻路客失败"
doAssert 下一站(e26, "X") == "乙", "寻路首站错"
doAssert 找线到(e26, "X", "乙") == 0, "寻路线定位错"
doAssert not 登车到(e26, "X", "丙"), "直达线不应存在"
doAssert 一步(e26, "X"), "寻路登车失败"
doAssert e26.客s[0].状态 == "在车上", "寻路未登车"
doAssert 一步(e26, "X"), "寻路一未到乙"
doAssert e26.客s[0].当前站 == "乙" and e26.客s[0].状态 == "已下车", "寻路一到站错"
doAssert 下一站(e26, "X") == "丙", "寻路二首站错"
doAssert 一步(e26, "X"), "寻路二登车失败"
doAssert 一步(e26, "X"), "寻路二未到丙"
doAssert e26.客s[0].当前站 == "丙" and e26.客s[0].状态 == "到站", "寻路二到站错"
doAssert 下一站(e26, "X") == "", "到站后仍有下一站"

var e27 = 新引擎(模式v)
doAssert 写(e27, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "Q1"), "容量甲失败"
doAssert 写(e27, "gttx://写入|乙|动作B|客.状态 = 待定|客.状态 = 已上车", "Q2"), "容量乙失败"
doAssert 写(e27, "gttx://写入|丙|动作C|客.状态 = 待定|客.状态 = 已上车", "Q3"), "容量丙失败"
doAssert 排班(e27, "甲", "乙", "前", "车", "09:00", 1), "容量线一失败"
doAssert 排班(e27, "乙", "丙", "前", "车", "09:00", 2), "容量线二失败"
doAssert 上车(e27, "A", "甲", "乙"), "容量客A失败"
doAssert 上车(e27, "B", "甲", "丙"), "容量客B失败"
doAssert 一步(e27, "A"), "容量A未登车"
doAssert e27.客s[0].状态 == "在车上", "容量A状态错"
doAssert 下一站(e27, "B") == "", "满线仍进寻路"
doAssert not 一步(e27, "B"), "满线B不应登车"
doAssert e27.客s[1].状态 == "无下一站", "满线B状态错"
doAssert 一步(e27, "A"), "容量A未下车"
doAssert 下一站(e27, "B") == "乙", "容量释放后寻路错"
doAssert 一步(e27, "B"), "容量B未登车"

var e28 = 新引擎(模式v)
doAssert 写(e28, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "S1"), "择时甲失败"
doAssert 写(e28, "gttx://写入|乙|动作B|客.状态 = 待定|客.状态 = 已上车", "S2"), "择时乙失败"
doAssert 写(e28, "gttx://写入|丙|动作C|客.状态 = 待定|客.状态 = 已上车", "S3"), "择时丙失败"
doAssert 写(e28, "gttx://写入|丁|动作D|客.状态 = 待定|客.状态 = 已上车", "S4"), "择时丁失败"
doAssert 排班(e28, "甲", "乙", "前", "车", "10:00", 2), "择时线甲乙失败"
doAssert 排班(e28, "乙", "丙", "前", "车", "10:05", 2), "择时线乙丙失败"
doAssert 排班(e28, "甲", "丁", "前", "车", "09:00", 2), "择时线甲丁失败"
doAssert 排班(e28, "丁", "丙", "前", "车", "09:10", 2), "择时线丁丙失败"
doAssert 设刻(e28, "09:00"), "择时设刻失败"
doAssert 上车(e28, "X", "甲", "丙"), "择时客失败"
doAssert 下一站(e28, "X") == "丁", "择时未选早到路径"
doAssert 找线到(e28, "X", "丁") == 2, "择时未选早班线"
doAssert 一步(e28, "X"), "择时登车失败"
doAssert 一步(e28, "X"), "择时未到丁"
doAssert e28.客s[0].当前站 == "丁", "择时到丁错"
doAssert 下一站(e28, "X") == "丙", "择时二段错"
doAssert 设刻(e28, "09:10"), "择时二段设刻失败"
doAssert 一步(e28, "X"), "择时二段登车失败"
doAssert 一步(e28, "X"), "择时未到丙"
doAssert e28.客s[0].状态 == "到站", "择时到丙错"

var e29 = 新引擎(模式v)
doAssert 写(e29, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "S5"), "择时晚甲失败"
doAssert 写(e29, "gttx://写入|乙|动作B|客.状态 = 待定|客.状态 = 已上车", "S6"), "择时晚乙失败"
doAssert 写(e29, "gttx://写入|丙|动作C|客.状态 = 待定|客.状态 = 已上车", "S7"), "择时晚丙失败"
doAssert 写(e29, "gttx://写入|丁|动作D|客.状态 = 待定|客.状态 = 已上车", "S8"), "择时晚丁失败"
doAssert 排班(e29, "甲", "乙", "前", "车", "10:00", 2), "择时晚线甲乙失败"
doAssert 排班(e29, "乙", "丙", "前", "车", "10:05", 2), "择时晚线乙丙失败"
doAssert 排班(e29, "甲", "丁", "前", "车", "09:00", 2), "择时晚线甲丁失败"
doAssert 排班(e29, "丁", "丙", "前", "车", "09:10", 2), "择时晚线丁丙失败"
doAssert 设刻(e29, "09:30"), "择时晚设刻失败"
doAssert 上车(e29, "Y", "甲", "丙"), "择时晚客失败"
doAssert 下一站(e29, "Y") == "乙", "择时晚未选可行路径"
doAssert 找线到(e29, "Y", "丁") < 0, "择时晚仍乘过点线"

var e30 = 新引擎(模式v)
doAssert 写(e30, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "U1"), "耗时甲失败"
doAssert 写(e30, "gttx://写入|乙|动作B|客.状态 = 待定|客.状态 = 已上车", "U2"), "耗时乙失败"
doAssert 写(e30, "gttx://写入|丙|动作C|客.状态 = 待定|客.状态 = 已上车", "U3"), "耗时丙失败"
doAssert 写(e30, "gttx://写入|丁|动作D|客.状态 = 待定|客.状态 = 已上车", "U4"), "耗时丁失败"
doAssert 排班(e30, "甲", "丁", "前", "车", "09:00", 2, 20), "耗时线甲丁失败"
doAssert 排班(e30, "丁", "丙", "前", "车", "09:25", 2, 5), "耗时线丁丙失败"
doAssert 排班(e30, "甲", "乙", "前", "车", "09:00", 2, 10), "耗时线甲乙失败"
doAssert 排班(e30, "乙", "丙", "前", "车", "09:15", 2, 5), "耗时线乙丙失败"
doAssert not 排班(e30, "甲", "乙", "前", "车", "10:00", 2, -1), "负耗时应拒"
doAssert 设刻(e30, "09:00"), "耗时设刻失败"
doAssert 上车(e30, "X", "甲", "丙"), "耗时客失败"
doAssert 下一站(e30, "X") == "乙", "耗时未选早到路径"
doAssert 找线到(e30, "X", "乙") == 2, "耗时未选早班线"
doAssert 一步(e30, "X"), "耗时登车失败"
doAssert 一步(e30, "X"), "耗时未到乙"
doAssert e30.客s[0].当前站 == "乙", "耗时到乙错"
doAssert e30.刻 == 550, "耗时未推进到到达刻"
doAssert 下一站(e30, "X") == "丙", "耗时二段错"
doAssert 设刻(e30, "09:15"), "耗时二段设刻失败"
doAssert 一步(e30, "X"), "耗时二段登车失败"
doAssert 一步(e30, "X"), "耗时二段未到丙"
doAssert e30.客s[0].状态 == "到站", "耗时到丙错"
doAssert e30.刻 == 560, "耗时二段到达刻错"
let 出30 = "/tmp/opencode/表格引擎/出验30"
存(e30, 出30)
let e31 = 读(出30, 模式v)
doAssert e31.线s[0].耗时 == 20 and e31.线s[2].耗时 == 10, "耗时回读错"
doAssert readFile(出30 / "线.chain").splitLines()[0].contains("耗时"), "耗时表头缺失"
let 旧线 = "/tmp/opencode/表格引擎/旧线"
createDir(旧线)
writeFile(旧线 / "线.chain", "gttx://线|起站|止站|方向|载具|时刻|容量\n" &
  "gttx://线|甲|乙|前|车|09:00|2\n")
let e32 = 读(旧线, 模式v)
doAssert e32.线s.len == 1 and e32.线s[0].耗时 == 0, "旧线耗时兼容错"

var e33 = 新引擎(模式v)
doAssert 写(e33, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "V1"), "跨日甲失败"
doAssert 写(e33, "gttx://写入|乙|动作B|客.状态 = 待定|客.状态 = 已上车", "V2"), "跨日乙失败"
doAssert 排班(e33, "甲", "乙", "前", "车", "23:50", 2, 30), "跨日班次失败"
doAssert 设刻(e33, "23:50"), "跨日设刻失败"
doAssert 上车(e33, "X", "甲", "乙"), "跨日客失败"
doAssert 一步(e33, "X"), "跨日登车失败"
doAssert 一步(e33, "X"), "跨日未到站"
doAssert e33.客s[0].状态 == "到站", "跨日状态错"
doAssert e33.刻 == 1460, "跨日到达刻错"

var e34 = 新引擎(模式v)
doAssert 写(e34, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "W1"), "跨日换甲失败"
doAssert 写(e34, "gttx://写入|乙|动作B|客.状态 = 待定|客.状态 = 已上车", "W2"), "跨日换乙失败"
doAssert 写(e34, "gttx://写入|丙|动作C|客.状态 = 待定|客.状态 = 已上车", "W3"), "跨日换丙失败"
doAssert 排班(e34, "甲", "乙", "前", "车", "23:50", 2, 30), "跨日换线一失败"
doAssert 排班(e34, "乙", "丙", "前", "车", "00:30", 2, 10), "跨日换线二失败"
doAssert 设刻(e34, "23:50"), "跨日换设刻失败"
doAssert 上车(e34, "Y", "甲", "丙"), "跨日换客失败"
doAssert 一步(e34, "Y"), "跨日换登车失败"
doAssert 一步(e34, "Y"), "跨日换未到乙"
doAssert e34.刻 == 1460, "跨日换到乙刻错"
doAssert 下一站(e34, "Y") == "丙", "跨日换寻路错"
doAssert 一步(e34, "Y"), "跨日换二段登车失败"
doAssert not 一步(e34, "Y"), "跨日换未候车"
doAssert e34.刻 == 1470 and e34.客s[0].状态 == "候车", "跨日换候车错"
doAssert 一步(e34, "Y"), "跨日换二段未到丙"
doAssert e34.客s[0].状态 == "到站" and e34.刻 == 1480, "跨日换到丙错"

var e35 = 新引擎(模式v)
doAssert 写(e35, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "X1"), "跨日格甲失败"
doAssert 写(e35, "gttx://写入|乙|动作B|客.状态 = 待定|客.状态 = 已上车", "X2"), "跨日格乙失败"
doAssert 写(e35, "gttx://写入|丙|动作C|客.状态 = 待定|客.状态 = 已上车", "X3"), "跨日格丙失败"
doAssert 排班(e35, "甲", "乙", "前", "车", "23:50", 2, 30), "跨日格班次失败"
doAssert 排班(e35, "乙", "丙", "前", "车", "00:30", 2, 10), "跨日格二段失败"
let 键7 = 线键(e35.线s[0])
doAssert 加坐标(e35, "乙", 键7, 0, 1), "跨日格坐标失败"
doAssert 加格(e35, 键7, 0, 1, "动作A", "00:20", "丙"), "跨日格加格失败"
doAssert 设刻(e35, "23:50"), "跨日格设刻失败"
doAssert 上车(e35, "Z", "甲", "丙"), "跨日格客失败"
doAssert 一步(e35, "Z"), "跨日格登车失败"
doAssert 一步(e35, "Z"), "跨日格未到乙"
doAssert e35.刻 == 1460, "跨日格到乙刻错"
doAssert 一步(e35, "Z"), "跨日格未命中"
doAssert e35.客s[0].当前站 == "丙" and e35.客s[0].状态 == "到站", "跨日格到丙错"

var e36 = 新引擎(模式v)
doAssert 写(e36, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "Y1"), "候夜甲失败"
doAssert 写(e36, "gttx://写入|乙|动作B|客.状态 = 待定|客.状态 = 已上车", "Y2"), "候夜乙失败"
doAssert 排班(e36, "甲", "乙", "前", "车", "00:10", 2, 5), "候夜班次失败"
doAssert 设刻(e36, "23:50"), "候夜设刻失败"
doAssert 上车(e36, "X", "甲", "乙"), "候夜客失败"
doAssert 候车时限 == 120, "候车时限错"
doAssert 找可乘线(e36, "X") >= 0, "临近午夜次日班不应拒"
doAssert 一步(e36, "X"), "候夜登车失败"
doAssert not 一步(e36, "X"), "候夜未等待"
doAssert e36.刻 == 1450 and e36.客s[0].状态 == "候车", "候夜候车错"
doAssert 一步(e36, "X"), "候夜未发车"
doAssert e36.客s[0].状态 == "到站" and e36.刻 == 1455, "候夜到站错"

var e37 = 新引擎(模式v)
doAssert 写(e37, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "Y3"), "远夜甲失败"
doAssert 写(e37, "gttx://写入|乙|动作B|客.状态 = 待定|客.状态 = 已上车", "Y4"), "远夜乙失败"
doAssert 排班(e37, "甲", "乙", "前", "车", "21:00", 2, 5), "远夜班次失败"
doAssert 设刻(e37, "23:50"), "远夜设刻失败"
doAssert 上车(e37, "Y", "甲", "乙"), "远夜客失败"
doAssert 找可乘线(e37, "Y") < 0, "超出候车时限仍可乘"
doAssert 下一站(e37, "Y") == "", "超出候车时限仍寻路"
doAssert 下一事件刻(e37, "Y") < 0, "超出候车时限仍候刻"
doAssert not 一步(e37, "Y"), "远夜不应登车"
doAssert e37.客s[0].状态 == "无下一站", "远夜状态错"

var e38 = 新引擎(模式v)
doAssert 写(e38, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "Y5"), "候格夜甲失败"
doAssert 写(e38, "gttx://写入|乙|动作B|客.状态 = 待定|客.状态 = 已上车", "Y6"), "候格夜乙失败"
doAssert 写(e38, "gttx://写入|丙|动作C|客.状态 = 待定|客.状态 = 已上车", "Y7"), "候格夜丙失败"
doAssert 排班(e38, "乙", "丙", "前", "车", "00:30", 2, 10), "候格夜班次失败"
let 键8 = 线键(e38.线s[0])
doAssert 加坐标(e38, "甲", 键8, 0, 0), "候格夜坐标失败"
doAssert 加格(e38, 键8, 0, 0, "动作A", "00:10", "乙"), "候格夜加格失败"
doAssert 设刻(e38, "23:50"), "候格夜设刻失败"
doAssert 上车(e38, "Z", "甲", "乙"), "候格夜客失败"
doAssert 下一事件刻(e38, "Z") == 1450, "候格夜下一事件错"
doAssert not 一步(e38, "Z"), "候格夜未等待"
doAssert e38.刻 == 1450 and e38.客s[0].状态 == "候刻", "候格夜候刻错"
doAssert 一步(e38, "Z"), "候格夜未命中"
doAssert e38.客s[0].当前站 == "乙" and e38.客s[0].状态 == "到站", "候格夜到站错"

var e39 = 新引擎(模式v)
doAssert 写(e39, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "Z1"), "候限甲失败"
doAssert 写(e39, "gttx://写入|乙|动作B|客.状态 = 待定|客.状态 = 已上车", "Z2"), "候限乙失败"
doAssert 排班(e39, "甲", "乙", "前", "车", "00:10", 2, 5), "候限班次失败"
doAssert 设刻(e39, "23:50"), "候限设刻失败"
doAssert 上车(e39, "X", "甲", "乙"), "候限客失败"
doAssert e39.候车 == 120, "默认候车时限错"
doAssert not 设候车(e39, -1), "负候车时限应拒"
doAssert 设候车(e39, 10), "候限设十失败"
doAssert 找可乘线(e39, "X") < 0, "十刻限仍可乘"
doAssert 下一站(e39, "X") == "", "十刻限仍寻路"
doAssert 下一事件刻(e39, "X") < 0, "十刻限仍候刻"
doAssert 设候车(e39, 30), "候限设三十失败"
doAssert 找可乘线(e39, "X") >= 0, "三十刻限不可乘"
doAssert 下一站(e39, "X") == "乙", "三十刻限寻路错"
doAssert 下一事件刻(e39, "X") == 1450, "三十刻限事件错"
let 出39 = "/tmp/opencode/表格引擎/出验39"
存(e39, 出39)
let e40 = 读(出39, 模式v)
doAssert e40.候车 == 30, "候车时限回读错"
doAssert readFile(出39 / "刻.chain").splitLines()[0].contains("候车"), "刻表头缺候车"
let 旧刻 = "/tmp/opencode/表格引擎/旧刻"
createDir(旧刻)
writeFile(旧刻 / "刻.chain", "gttx://刻|当前\n" & "gttx://刻|570\n")
let e41 = 读(旧刻, 模式v)
doAssert e41.刻 == 570 and e41.候车 == 120, "旧刻候车兼容错"

var e42 = 新引擎(模式v)
doAssert 写(e42, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "A1"), "线候甲失败"
doAssert 写(e42, "gttx://写入|乙|动作B|客.状态 = 待定|客.状态 = 已上车", "A2"), "线候乙失败"
doAssert 排班(e42, "甲", "乙", "前", "车", "00:10", 2, 5, 10), "线候班次失败"
let 键9 = 线键(e42.线s[0])
doAssert e42.线s[0].候车 == 10, "线候字段错"
doAssert 设刻(e42, "23:50"), "线候设刻失败"
doAssert 上车(e42, "X", "甲", "乙"), "线候客失败"
doAssert 找可乘线(e42, "X") < 0, "线候十仍可乘"
doAssert 下一事件刻(e42, "X") < 0, "线候十仍候刻"
doAssert not 设线候车(e42, "无此线", 30), "未知线应拒"
doAssert 设线候车(e42, 键9, 30), "线候设三十失败"
doAssert 找可乘线(e42, "X") >= 0, "线候三十不可乘"
doAssert 下一事件刻(e42, "X") == 1450, "线候三十事件错"

var e43 = 新引擎(模式v)
doAssert 写(e43, "gttx://写入|甲|动作A|客.状态 = 待定|客.状态 = 已上车", "A3"), "站候甲失败"
doAssert 写(e43, "gttx://写入|乙|动作B|客.状态 = 待定|客.状态 = 已上车", "A4"), "站候乙失败"
doAssert 排班(e43, "甲", "乙", "前", "车", "00:10", 2, 5), "站候班次失败"
doAssert 设刻(e43, "23:50"), "站候设刻失败"
doAssert 上车(e43, "Y", "甲", "乙"), "站候客失败"
doAssert not 设站候车(e43, "无此站", 30), "未知站应拒"
doAssert 设站候车(e43, "甲", 10), "站候设十失败"
doAssert 找可乘线(e43, "Y") < 0, "站候十仍可乘"
doAssert 下一事件刻(e43, "Y") < 0, "站候十仍候刻"
doAssert 设站候车(e43, "甲", 30), "站候设三十失败"
doAssert 找可乘线(e43, "Y") >= 0, "站候三十不可乘"
doAssert 下一事件刻(e43, "Y") == 1450, "站候三十事件错"

doAssert 设站候车(e42, "甲", 30), "线站优先级设站失败"
doAssert 找可乘线(e42, "X") >= 0, "线候三十仍不可乘"
doAssert 设线候车(e42, 键9, 10), "线站优先级设线失败"
doAssert 找可乘线(e42, "X") < 0, "线候十优先级失效"
let 出42 = "/tmp/opencode/表格引擎/出验42"
存(e42, 出42)
let e44 = 读(出42, 模式v)
doAssert e44.线s[0].候车 == 10, "线候回读错"
doAssert e44.站s[0].候车 == 30, "站候回读错"
doAssert readFile(出42 / "线.chain").splitLines()[0].contains("候车"), "线表头缺候车"
doAssert readFile(出42 / "站.chain").splitLines()[0].contains("候车"), "站表头缺候车"
let 旧站 = "/tmp/opencode/表格引擎/旧站"
createDir(旧站)
writeFile(旧站 / "站.chain", "gttx://站|地址|符号|部|入边|出边|状态\n" &
  "gttx://站|1|甲|未定|\"\"|\"\"|待定\n")
let e45 = 读(旧站, 模式v)
doAssert e45.站s.len == 1 and e45.站s[0].候车 == -1, "旧站候车兼容错"
doAssert e32.线s[0].候车 == -1, "旧线候车兼容错"

echo "边界解析: 基本/引号/转义/围墙/双记录/兼容/中括/注释 全过"
echo "边界拒收: 围墙/容量/头后/空单元/尾随 全拒"
echo "严格模式: 未闭合引号/坏围墙/空围墙/引号后多字 全拒"
