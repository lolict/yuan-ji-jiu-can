# 节拍机 v0.3 · 射线律版
# 内核继承: 满全法天平(manquanfa_judge) + 锚点我=1(SYMBOL_AUTHORITY) + gttx://(table-trans)
#           + 负样本 + append-only 日志 + 加法纪律(禁乘除 mod, §3.5) + 课程注册制
# 射线律: 线段使用不得封射线 — 存在态活载轴.toml[态](新态=添行不重编译);
#         符号.toml 用户自有(机器只读不重写), 机器标注账落 节拍状态.toml;
#         口诀座=行加法序, 新砖追加不动旧段
import os, strutils, md5, osproc, times, tables, unicode

type
  符号节点 = object
    名: string
    部: string
    笔画: int
    笔画待拆: bool
    范畴: string
    码位: string
    态: string
    座位: int
    频率: int
    上拍频率: int
    速度: int
  关系边 = object
    源: string
    关系名: string
    目标: string
    依据: string
  值行 = object
    节: string
    键: string
    值: string

const 汉源默认 = "/workspace/hanzi-addr"

var
  根目录: string = ""
  汉源目录: string = 汉源默认
  符号表: seq[符号节点] = @[]
  边表: seq[关系边] = @[]
  任务队列: seq[(string, string)] = @[]
  tick = 0
  计数统计 = 0
  间隔统计 = 2
  日志行: seq[string] = @[]
  触过: seq[string] = @[]
  在用段: seq[string] = @[]
  部序列: seq[string] = @[]
  态映射: Table[string, string] = initTable[string, string]()
  态值表: Table[string, bool] = initTable[string, bool]()
  态序: seq[string] = @[]
  偏旁部表: Table[string, string] = initTable[string, string]()
  部件笔画表: Table[string, int] = initTable[string, int]()
  拆解表: Table[string, (string, string, string)] = initTable[string, (string, string, string)]()
  范畴表: Table[string, string] = initTable[string, string]()
  注音表: Table[string, string] = initTable[string, string]()
  口诀座表: Table[string, int] = initTable[string, int]()
  课程表: Table[string, string] = initTable[string, string]()

proc 日志(行: string) =
  let 全行 = "gttx://" & 行 & "|锚1"
  日志行.add 全行
  echo "拍" & $tick & " " & 全行

proc 读值表(path: string): seq[值行] =
  var 节 = ""
  if not fileExists(path): return @[]
  for 原行 in readFile(path).splitLines():
    let 行 = 原行.strip()
    if 行.len == 0 or 行.startsWith("#"): continue
    if 行.startsWith("["):
      节 = 行.strip(chars = {'[', ']'})
      continue
    let 等号 = 行.find('=')
    if 等号 > 0:
      let 值 = 行[等号 + 1 .. ^1].strip().strip(chars = {'"'})
      result.add 值行(节: 节, 键: 行[0 .. 等号 - 1].strip(), 值: 值)

proc 载轴() =
  部序列 = @[]
  态映射 = initTable[string, string]()
  态值表 = initTable[string, bool]()
  态序 = @[]
  偏旁部表 = initTable[string, string]()
  for 行 in 读值表(汉源目录 & "/轴.toml"):
    if 行.节 == "先后" and 行.键 == "列":
      for 项 in 行.值.strip(chars = {'[', ']'}).split(','):
        let 名 = 项.strip().strip(chars = {'"'})
        if 名.len > 0: 部序列.add 名
    elif 行.节 == "态":
      态映射[行.键] = 行.值
      态值表[行.值] = true
    elif 行.节 == "态先后" and 行.键 == "列":
      for 项 in 行.值.strip(chars = {'[', ']'}).split(','):
        let 名 = 项.strip().strip(chars = {'"'})
        if 名.len > 0: 态序.add 名
    elif 行.节 == "偏旁":
      偏旁部表[行.键] = 行.值

proc 载字() =
  部件笔画表 = initTable[string, int]()
  拆解表 = initTable[string, (string, string, string)]()
  for 行 in 读值表(汉源目录 & "/字.toml"):
    if 行.节 == "偏旁":
      部件笔画表[行.键] = 行.值.runeLen
  var 当前字 = ""
  for 原行 in readFile(汉源目录 & "/字.toml").splitLines():
    let 行 = 原行.strip()
    if 行.len == 0 or 行.startsWith("#"): continue
    if 行.startsWith("["):
      当前字 = 行.strip(chars = {'[', ']'})
      if 当前字 != "偏旁" and not 拆解表.contains(当前字):
        拆解表[当前字] = ("", "", "")
      continue
    let 等号 = 行.find('=')
    if 等号 > 0 and 当前字 != "" and 当前字 != "偏旁":
      let 键 = 行[0 .. 等号 - 1].strip()
      let 值 = 行[等号 + 1 .. ^1].strip().strip(chars = {'"'})
      var 左, 右, 结构: string
      (左, 右, 结构) = 拆解表[当前字]
      if 键 == "左": 左 = 值
      elif 键 == "右": 右 = 值
      elif 键 == "结构": 结构 = 值
      拆解表[当前字] = (左, 右, 结构)

proc 载感知() =
  范畴表 = initTable[string, string]()
  注音表 = initTable[string, string]()
  for 行 in 读值表(汉源目录 & "/感知.toml"):
    if 行.节 == "范畴":
      范畴表[行.键] = 行.值
    elif 行.节 == "注音":
      注音表[行.键] = 行.值

proc 载口诀() =
  口诀座表 = initTable[string, int]()
  let path = 汉源目录 & "/口诀表.txt"
  if not fileExists(path): return
  var 座 = 0
  for 原行 in readFile(path).splitLines():
    let 音节 = 原行.strip()
    if 音节.len == 0: continue
    座 += 1
    口诀座表[音节] = 座

proc 笔画数(部件: string, 深: int = 0): int =
  if 部件笔画表.contains(部件): return 部件笔画表[部件]
  if 深 >= 2 or not 拆解表.contains(部件): return 0
  let (左, 右, 结构) = 拆解表[部件]
  if 结构 == "独体": return 0
  result = 笔画数(左, 深 + 1) + 笔画数(右, 深 + 1)

proc 部名(符号: string): string =
  if 偏旁部表.contains(符号): return 偏旁部表[符号]
  if 拆解表.contains(符号):
    let (左, 右, 结构) = 拆解表[符号]
    if 结构 != "独体" and 偏旁部表.contains(左): return 偏旁部表[左]
  ""

proc 范畴名(符号: string): string =
  if 范畴表.contains(符号): return 范畴表[符号]
  if 拆解表.contains(符号):
    let (左, 右, 结构) = 拆解表[符号]
    if 结构 != "独体" and 范畴表.contains(左): return 范畴表[左]
  "积木"

proc 码位名(符号: string): string =
  if 注音表.contains(符号): return 注音表[符号] & "-待授"
  "待授"

proc 存在名(符号: string): string =
  # 态活载轴.toml[态]: 结构名→态 (左右→有, 独体→乙); 新态=添行
  if 拆解表.contains(符号):
    let (左, 右, 结构) = 拆解表[符号]
    if 态映射.contains(结构): return 态映射[结构]
  "有"

proc 部序(部: string): int =
  for i, b in 部序列:
    if b == 部: return i + 1
  0

proc 查符号(名: string): int =
  for i, s in 符号表:
    if s.名 == 名: return i
  -1

proc 载标注() =
  # 用户真源种子: 符号.toml [标注] 行 (机器只读此文件, 永不重写)
  for 行 in 读值表(根目录 & "/符号.toml"):
    if 行.节 != "标注": continue
    let 段 = 行.值.split('|')
    if 段.len < 3: continue
    var 命中 = false
    for i in mitems(符号表):
      if i.名 == 行.键:
        i.频率 = parseInt(段[0])
        i.座位 = parseInt(段[1])
        i.上拍频率 = i.频率
        if 态值表.contains(段[2]): i.态 = 段[2]
        命中 = true
    if not 命中:
      var s: 符号节点
      s.名 = 行.键
      s.部 = "音节"
      s.笔画 = 0
      s.笔画待拆 = false
      s.范畴 = "积木"
      s.码位 = 码位名(行.键)
      s.频率 = parseInt(段[0])
      s.座位 = parseInt(段[1])
      s.上拍频率 = s.频率
      s.态 = if 态值表.contains(段[2]): 段[2] else: "有"
      符号表.add s

proc 载账() =
  # 机器标注账: 节拍状态.toml [标注账] 行, 后行盖前行 (机器自有文件)
  for 行 in 读值表(根目录 & "/节拍状态.toml"):
    if 行.节 != "标注账": continue
    let 段 = 行.值.split('|')
    if 段.len < 3: continue
    for i in mitems(符号表):
      if i.名 == 行.键:
        i.频率 = parseInt(段[0])
        i.座位 = parseInt(段[1])
        i.上拍频率 = i.频率
        if 态值表.contains(段[2]): i.态 = 段[2]

proc 建节点() =
  符号表 = @[]
  for 部件, 笔画n in 部件笔画表:
    var s: 符号节点
    s.名 = 部件
    s.部 = 部名(部件)
    s.笔画 = 笔画n
    s.笔画待拆 = false
    s.范畴 = 范畴名(部件)
    s.码位 = 码位名(部件)
    s.态 = 存在名(部件)
    符号表.add s
  for 字名, 拆 in 拆解表:
    if 部件笔画表.contains(字名): continue
    var s: 符号节点
    s.名 = 字名
    s.部 = 部名(字名)
    let n = 笔画数(字名)
    s.笔画 = n
    s.笔画待拆 = n == 0
    s.范畴 = 范畴名(字名)
    s.码位 = 码位名(字名)
    s.态 = 存在名(字名)
    符号表.add s
  # 部容器节点: 轴.toml [先后] 列就是空间分段
  for 部 in 部序列:
    if 查符号(部) >= 0: continue
    var s: 符号节点
    s.名 = 部
    s.部 = 部
    s.笔画 = 0
    s.笔画待拆 = false
    s.范畴 = "容器"
    s.码位 = "待授"
    s.态 = "有"
    符号表.add s
  载标注()
  载账()

proc 载关系() =
  边表 = @[]
  let path = 根目录 & "/关系.chain"
  if not fileExists(path): return
  for 原行 in readFile(path).splitLines():
    let 行 = 原行.strip()
    if not 行.startsWith("gttx://rel|"): continue
    let 段 = 行.split('|')
    if 段.len >= 5:
      边表.add 关系边(源: 段[1], 关系名: 段[2], 目标: 段[3], 依据: 段[4])

proc 载任务() =
  任务队列 = @[]
  let path = 根目录 & "/任务.chain"
  if not fileExists(path): return
  for 原行 in readFile(path).splitLines():
    let 行 = 原行.strip()
    if not 行.startsWith("gttx://task|"): continue
    let 段 = 行.split('|', 2)
    if 段.len == 3:
      任务队列.add (段[1], 段[2])

proc 载课程() =
  课程表 = initTable[string, string]()
  let path = 根目录 & "/课程.chain"
  if not fileExists(path): return
  for 原行 in readFile(path).splitLines():
    let 行 = 原行.strip()
    if not 行.startsWith("gttx://课程|"): continue
    let 段 = 行.split('|')
    if 段.len >= 5:
      课程表[段[1]] = 段[4]

proc 载配置() =
  for 行 in 读值表(根目录 & "/节拍.toml"):
    if 行.节 == "间隔" and 行.键 == "统计":
      间隔统计 = parseInt(行.值)

proc 存状态() =
  # 机器自有文件: 节拍状态.toml (标注账只落这里, 符号.toml 永不被机器重写)
  var 文 = "# 节拍机状态 (机器回写, 勿手改)\n[节拍]\ntick = " & $tick & "\n"
  文.add "队列长度 = " & $任务队列.len & "\n\n[待办]\n"
  for t in 任务队列:
    文.add "gttx://task|" & t[0] & "|" & t[1] & "\n"
  文.add "\n[标注账]\n"
  for s in 符号表:
    文.add s.名 & " = \"" & $s.频率 & "|" & $s.座位 & "|" & s.态 & "\"\n"
  writeFile(根目录 & "/节拍状态.toml", 文)

proc 写负样本(名, 参数, 理由: string) =
  let path = 根目录 & "/负样本.chain"
  var 文 = ""
  if fileExists(path): 文 = readFile(path)
  文.add "gttx://neg|" & $tick & "|" & 名 & "|" & 参数 & "|" & 理由 & "\n"
  writeFile(path, 文)

proc 处理任务(名, 参数: string): bool =
  if 名 == "造字":
    let 段 = 参数.split(',')
    if 段.len != 3: return false
    let 音节 = 段[2]
    if not 口诀座表.contains(音节): return false
    var i = 查符号(音节)
    if i < 0:
      var s: 符号节点
      s.名 = 音节
      s.部 = "音节"
      s.笔画 = 0
      s.笔画待拆 = false
      s.范畴 = "积木"
      s.码位 = 段[0] & "-" & 段[1] & "-" & 音节 & "-待授"
      s.态 = "有"
      符号表.add s
      i = 符号表.len - 1
    符号表[i].频率 += 1
    if 符号表[i].座位 == 0: 符号表[i].座位 = 口诀座表[音节]
    if not 在用段.contains(音节): 在用段.add 音节
    日志 "造字|" & 音节 & "|口诀座" & $符号表[i].座位 & "|码位" & 符号表[i].码位 & "|频率" & $符号表[i].频率
    任务队列.add ("分类", 音节)
    true
  elif 名 == "分类":
    let i = 查符号(参数)
    if i < 0: return false
    var 同部 = 0
    for s in 符号表:
      if s.部 == 符号表[i].部 and s.座位 > 0: 同部 += 1
    if 符号表[i].座位 == 0:
      符号表[i].座位 = 同部 + 1
    日志 "分类|" & 参数 & "|部" & 符号表[i].部 & "|座位" & $符号表[i].座位
    true
  elif 名 == "距离":
    let 段 = 参数.split(',')
    if 段.len != 2: return false
    let a = 查符号(段[0])
    let b = 查符号(段[1])
    if a < 0 or b < 0: return false
    let 拆 = (if 符号表[a].笔画待拆 or 符号表[b].笔画待拆: "含待拆|" else: "")
    let 距 = abs(符号表[a].笔画 - 符号表[b].笔画) +
      abs(部序(符号表[a].部) - 部序(符号表[b].部)) +
      abs(符号表[a].座位 - 符号表[b].座位)
    日志 "距离|" & 段[0] & "," & 段[1] & "|" & 拆 & "加法距" & $距
    true
  elif 名 == "关系":
    let i = 查符号(参数)
    if i < 0: return false
    var 传 = 0
    for e in 边表:
      if e.源 == 参数 or e.目标 == 参数:
        let 邻 = if e.源 == 参数: e.目标 else: e.源
        let j = 查符号(邻)
        if j >= 0:
          符号表[j].频率 += 1
          传 += 1
          日志 "关系|" & 参数 & "-" & e.关系名 & "-" & 邻 & "|传播频率" & $符号表[j].频率
    if 传 == 0: 日志 "关系|" & 参数 & "|无邻接"
    true
  elif 名 == "存在":
    let 段 = 参数.split(',')
    if 段.len != 2: return false
    let i = 查符号(段[0])
    if i < 0: return false
    if not 态值表.contains(段[1]): return false
    日志 "存在|" & 段[0] & "|" & 符号表[i].态 & "变" & 段[1]
    符号表[i].态 = 段[1]
    true
  else:
    false

proc 速度过拍() =
  for i in mitems(符号表):
    i.速度 = i.频率 - i.上拍频率
    if i.速度 != 0:
      日志 "速度|" & i.名 & "|本拍" & $i.频率 & "|上拍" & $i.上拍频率 & "|速度" & $i.速度
    i.上拍频率 = i.频率

proc 走(拍数: int) =
  for _ in 1 .. 拍数:
    tick += 1
    触过 = @[]
    if 任务队列.len > 0:
      let t = 任务队列[0]
      任务队列.delete(0)
      日志 "机会|" & t[0] & "|" & t[1]
      if not 课程表.contains(t[0]):
        日志 "天平|vrReject|" & t[0] & "|未注册教师"
        写负样本(t[0], t[1], "未注册教师")
      elif 课程表[t[0]] != "开课中":
        日志 "天平|vrReject|" & t[0] & "|待开课"
        写负样本(t[0], t[1], "待开课")
      else:
        let 过 = 处理任务(t[0], t[1])
        if 过:
          日志 "天平|vrAccept|" & t[0]
          触过.add t[0]
        else:
          日志 "天平|vrReject|" & t[0] & "|负样本登记"
          写负样本(t[0], t[1], "处理失手")
    else:
      日志 "机会|空拍|待机"
    计数统计 += 1
    if 计数统计 == 间隔统计:
      计数统计 = 0
      速度过拍()
    日志 "拍毕|" & $tick & "|见拍" & $触过.len
  # 线段声明: 在用几段, 射线开放到几段
  var 在用 = ""
  for i, 段 in 在用段:
    if i > 0: 在用.add ","
    在用.add 段
  日志 "线段|口诀表|" & (if 在用 == "": "空段" else: 在用) & "|射线开放到" & $口诀座表.len
  存状态()
  let f = open(根目录 & "/节拍日志.chain", fmAppend)
  for 行 in 日志行: f.writeLine(行)
  f.close()

proc 状态() =
  echo "拍 " & $tick
  var 开课 = 0
  var 待开 = 0
  for 课, 态 in 课程表:
    if 态 == "开课中": 开课 += 1 else: 待开 += 1
  echo "教师 " & $课程表.len & " 位 (开课中 " & $开课 & ", 待开课 " & $待开 & "), 符号节点 " & $符号表.len & " 个, 关系边 " & $边表.len & " 条, 待办 " & $任务队列.len & " 件"
  for s in 符号表:
    let 拆 = if s.笔画待拆: "待拆" else: $s.笔画
    echo "  " & s.名 & " 部" & s.部 & " 笔画" & 拆 & " 范畴" & s.范畴 & " 频率" & $s.频率 & " 座位" & $s.座位 & " 态" & s.态

proc 验() =
  let 基 = "/tmp/opencode/节拍验" & $now().toTime().toUnix()
  var 全过 = true
  var 段数 = 0
  for d in [基 & "1", 基 & "2"]:
    createDir(d)
    for f in ["符号.toml", "关系.chain", "任务.chain", "节拍.toml", "课程.chain", "射线.chain"]:
      copyFile(根目录 & "/" & f, d & "/" & f)
    # 射线延展沙箱: 汉源四件拷进沙箱, 铺第 7 段新砖
    let 汉 = d & "/汉源"
    createDir(汉)
    for f in ["轴.toml", "字.toml", "感知.toml", "口诀表.txt"]:
      copyFile(汉源目录 & "/" & f, 汉 & "/" & f)
    var 段内 = 0
    for l in readFile(汉 & "/口诀表.txt").splitLines():
      if l.strip().len > 0: 段内 += 1
    段数 = 段内
    let 新音节 = "bǎi"
    let 新座 = 段数 + 1
    let 砖 = open(汉 & "/口诀表.txt", fmAppend)
    砖.writeLine(新音节)
    砖.close()
    let 追 = open(d & "/任务.chain", fmAppend)
    追.writeLine("gttx://task|造字|b,ai," & 新音节)
    追.close()
    let 进程 = execCmdEx(getAppFilename() & " 走 9 --根 " & d & " --汉源 " & 汉)
    if 进程.exitCode != 0:
      echo "跑不拢 " & d
      全过 = false
  let 日1 = readFile(基 & "1/节拍日志.chain")
  let 日2 = readFile(基 & "2/节拍日志.chain")
  if getMD5(日1) == getMD5(日2):
    echo "确定性: 两沙箱日志同指纹 过"
  else:
    echo "确定性: 两沙箱日志异指纹 未过"
    全过 = false
  var 射线记 = "新砖延展"
  var 命中新 = false
  for 行 in 日1.splitLines():
    if 行.contains("造字|bǎi|口诀座" & $(段数 + 1)):
      命中新 = true
      射线记 = 行
  if 命中新:
    echo "新砖延展: 过 (" & 射线记 & ")"
  else:
    echo "新砖延展: 未过 (bǎi 未落第 " & $(段数 + 1) & " 座)"
    全过 = false
  for (记, 名) in [("vrAccept", "天平放行"), ("未注册教师", "未注册教师驳回"), ("待开课", "待开课驳回"), ("造字|bā|口诀座2", "旧段bā不动"), ("分类|bā|部", "空间绑定"), ("加法距", "距离计算"), ("传播频率", "关系传播"), ("速度|bā", "速度计算"), ("变乙", "存在切换轴态"), ("线段|口诀表|", "线段声明在册")]:
    if 日1.contains(记):
      echo 名 & ": 过"
    else:
      echo 名 & ": 未过 (缺 " & 记 & ")"
      全过 = false
  let 负 = readFile(基 & "1/负样本.chain")
  if 负.contains("不详"):
    echo "负样本在册: 过"
  else:
    echo "负样本在册: 未过"
    全过 = false
  if 日1.contains("距离|江,工|加法距"):
    echo "真源活载(江工笔画出自字.toml): 过"
  else:
    echo "真源活载: 未过"
    全过 = false
  if 全过:
    echo "总验收: 过"
  else:
    echo "总验收: 未过"
    quit(1)

proc 主() =
  if 根目录 == "":
    let d = getAppDir()
    根目录 = if fileExists(d & "/符号.toml"): d else: d.parentDir()
  var 参数 = commandLineParams()
  var i = 0
  var 命令行: seq[string] = @[]
  while i < 参数.len:
    if 参数[i] == "--根" and i + 1 < 参数.len:
      根目录 = 参数[i + 1]
      i += 2
    elif 参数[i] == "--汉源" and i + 1 < 参数.len:
      汉源目录 = 参数[i + 1]
      i += 2
    else:
      命令行.add 参数[i]
      i += 1
  if 命令行.len == 0:
    命令行 = @[""]
  let 命令 = 命令行[0]
  if 命令 == "走":
    载配置()
    载轴()
    载字()
    载感知()
    载口诀()
    建节点()
    载关系()
    载任务()
    载课程()
    if 命令行.len < 2: quit(1)
    走(parseInt(命令行[1]))
    echo "走了 " & 命令行[1] & " 拍, 日志 " & 根目录 & "/节拍日志.chain"
  elif 命令 == "状态":
    载轴()
    载字()
    载感知()
    载口诀()
    建节点()
    载关系()
    载任务()
    载课程()
    状态()
  elif 命令 == "验":
    验()
  else:
    echo "用法: 节拍 走 <拍数> | 状态 | 验"

主()
