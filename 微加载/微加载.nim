# 微加载 · 按需取文 / 全局去重 / 断点追加 / 速阅补全 / 进度与天平可视
# 真源: 微加载.chain 目录.chain 加载状态.chain 索引.chain 考核.chain
# 纪律: 索引与状态只追加; 指纹全局唯一; 天平: 入索引 + 去重 = 已收行
import os, strutils, tables, sets, osproc, algorithm, json, unicode

type
  目录项 = object
    源名, 路径, 类型, 部: string
  状态项 = object
    已收行, 入索引, 去重: int
    末指纹: string
  索引项 = object
    源名: string
    行号: int
    指纹, 文本: string
  切片项 = object
    页号: int
    标题, 簇, 部, 源名, 摘要: string
    起行, 止行: int
    文本, 词s: seq[string]
  冲突项 = object
    号: int
    主题, 差异, 状态, 证据: string
    甲页, 乙页: int
  关联项 = object
    甲页: int
    关系: string
    乙页: int
    证据: string

var
  参根 = ""
  参html = false
  参限 = 20
  参表: seq[string] = @[]

proc 解析参数() =
  var i = 1
  while i <= paramCount():
    let p = paramStr(i)
    if p == "--根" and i < paramCount():
      参根 = paramStr(i + 1)
      i += 2
    elif p == "--html":
      参html = true
      i += 1
    elif p == "--限" and i < paramCount():
      参限 = parseInt(paramStr(i + 1))
      i += 2
    else:
      参表.add p
      i += 1

proc 根路径(): string =
  result = absolutePath(if 参根.len > 0: 参根 else: parentDir(getAppDir()))

proc 指纹(s: string): string =
  var h: uint64 = 1469598103934665603'u64
  for c in s:
    h = h xor uint64(ord(c))
    h = h * 1099511628211'u64
  result = toHex(h, 16)

proc 清洗(s: string): string =
  result = s.replace("|", "¦").replace("\t", " ").strip()

proc 取行(s: string): seq[string] =
  result = s.splitLines()
  while result.len > 0 and result[^1].strip.len == 0:
    result.setLen(result.len - 1)

proc 净UTF8(s: string): string =
  result = s
  var i = validateUtf8(result)
  while i >= 0:
    result[i] = '?'
    i = validateUtf8(result)

proc 取源行(文件: string): seq[string] =
  result = 取行(readFile(文件))
  for i in 0 ..< result.len:
    result[i] = 净UTF8(result[i])

proc 绝对(根, p: string): string =
  result = if p.isAbsolute: p else: 根 / p

proc 追加(路径, 行: string) =
  let f = open(路径, fmAppend)
  f.writeLine(行)
  f.close()

proc 读目录(根: string): OrderedTable[string, 目录项] =
  result = initOrderedTable[string, 目录项]()
  let p = 根 / "目录.chain"
  if not fileExists(p): return
  for 行 in readFile(p).splitLines():
    if not 行.startsWith("gttx://目录|"): continue
    let 域 = 行.split('|')
    if 域.len < 5: continue
    result[域[1]] = 目录项(源名: 域[1], 路径: 域[2], 类型: 域[3], 部: 域[4])

proc 读状态(根: string): Table[string, 状态项] =
  result = initTable[string, 状态项]()
  let p = 根 / "加载状态.chain"
  if not fileExists(p): return
  for 行 in readFile(p).splitLines():
    if not 行.startsWith("gttx://状态|"): continue
    let 域 = 行.split('|')
    if 域.len < 6: continue
    result[域[1]] = 状态项(
      已收行: parseInt(域[2]),
      入索引: parseInt(域[3]),
      去重: parseInt(域[4]),
      末指纹: 域[5])

proc 统计索引(根: string): tuple[行s: seq[索引项], 指纹s: HashSet[string], 源计: Table[string, int]] =
  result.行s = @[]
  result.指纹s = initHashSet[string]()
  result.源计 = initTable[string, int]()
  let p = 根 / "索引.chain"
  if not fileExists(p): return
  for 行 in readFile(p).splitLines():
    if not 行.startsWith("gttx://索引|"): continue
    let 域 = 行.split('|')
    if 域.len < 5: continue
    let 项 = 索引项(源名: 域[1], 行号: parseInt(域[2]), 指纹: 域[3], 文本: 域[4])
    result.行s.add 项
    result.指纹s.incl 项.指纹
    result.源计[项.源名] = result.源计.getOrDefault(项.源名, 0) + 1

proc 收(根, 源名: string, 覆盖路径 = ""): int =
  let 目录 = 读目录(根)
  doAssert 目录.hasKey(源名), "目录无此源: " & 源名
  var p = 目录[源名].路径
  if 覆盖路径.len > 0: p = 覆盖路径
  let 文件 = 绝对(根, p)
  doAssert fileExists(文件), "源文件不存在: " & 文件
  let 行s = 取行(readFile(文件))
  let 状态s = 读状态(根)
  let 旧 = 状态s.getOrDefault(源名, 状态项())
  if 行s.len < 旧.已收行:
    stderr.writeLine("源缩短: " & 源名 & " (" & $旧.已收行 & " -> " & $行s.len & "); 断点只追加, 请建新源")
    quit(2)
  # 全量重扫: 源可插入/替换, 只把新指纹追加进索引; 已收行仍作断点高水位
  let 起 = 0
  var 索引 = 统计索引(根)
  var 入 = 0
  var 去 = 0
  for i in 起 ..< 行s.len:
    let t = 清洗(行s[i])
    if t.len == 0:
      去 += 1
      continue
    let fp = 指纹(t)
    if 索引.指纹s.contains(fp):
      去 += 1
    else:
      追加(根 / "索引.chain", "gttx://索引|" & 源名 & "|" & $(i + 1) & "|" & fp & "|" & t)
      索引.指纹s.incl fp
      入 += 1
  let 入总 = 索引.源计.getOrDefault(源名, 0) + 入
  let 去总 = 行s.len - 入总
  doAssert 入总 <= 行s.len, "入索引超过源行数: " & 源名
  let 末 = if 行s.len > 0: 指纹(清洗(行s[^1])) else: ""
  if 旧.已收行 == 行s.len and 旧.入索引 == 入总 and 旧.去重 == 去总 and 旧.末指纹 == 末:
    echo "收 " & 源名 & ": 无新段, 天平 " & $入总 & "+" & $去总 & "=" & $行s.len
    return 0
  追加(根 / "加载状态.chain", "gttx://状态|" & 源名 & "|" & $行s.len & "|" & $入总 & "|" & $去总 & "|" & 末)
  echo "收 " & 源名 & ": 源 " & $行s.len & " 行, 本次入索引 " & $入 & ", 去重 " & $去 & ", 天平 " & $入总 & "+" & $去总 & "=" & $行s.len
  result = 入

proc 收全(根: string) =
  let 目录 = 读目录(根)
  for 源名, 项 in 目录:
    let p = 绝对(根, 项.路径)
    if fileExists(p):
      discard 收(根, 源名)
    else:
      echo "跳过(缺文): " & 源名 & " -> " & p

proc 去标签(s: string): string =
  var 出 = newStringOfCap(s.len)
  var i = 0
  var 跳 = false
  const 块名 = ["p", "div", "br", "li", "tr", "h1", "h2", "h3", "h4", "h5", "h6", "section", "article"]
  while i < s.len:
    if s[i] == '<':
      let 尾 = s.find('>', i)
      let 名 = if 尾 < 0: "" else: s[i + 1 ..< 尾].strip.toLowerAscii
      if 名.startsWith("script") or 名.startsWith("style"): 跳 = true
      if 名.startsWith("/script") or 名.startsWith("/style"): 跳 = false
      if not 跳:
        for b in 块名:
          if 名 == b or 名.startsWith(b & " "):
            出.add '\n'
            break
      i = if 尾 < 0: s.len else: 尾 + 1
    else:
      if not 跳: 出.add s[i]
      i += 1
  var t = 出
  t = t.replace("&amp;", "&").replace("&lt;", "<").replace("&gt;", ">")
  t = t.replace("&quot;", "\"").replace("&#39;", "'").replace("&nbsp;", " ")
  var 行s: seq[string] = @[]
  for l in t.splitLines():
    let x = l.strip
    if x.len > 0: 行s.add x
  result = 行s.join("\n")

proc 取(根, 源名, url: string) =
  let cmd = "curl --location --max-time 20 --silent --show-error " & quoteShell(url)
  let r = execCmdEx(cmd)
  if r.exitCode != 0:
    stderr.writeLine("取失败: " & url & " / " & r.output.strip)
    quit(3)
  let 正文 = 去标签(r.output)
  let 缓存目录 = 根 / "缓存"
  createDir(缓存目录)
  let 安名 = 源名.replace("/", "_").replace("\\", "_")
  let 缓存 = 缓存目录 / (安名 & ".txt")
  writeFile(缓存, 正文)
  let 目录 = 读目录(根)
  if not 目录.hasKey(源名):
    追加(根 / "目录.chain", "gttx://目录|" & 源名 & "|缓存/" & 安名 & ".txt|链接|外链")
  echo "取 " & 源名 & ": " & $正文.splitLines().len & " 行"
  discard 收(根, 源名, 缓存)

proc 索(根, 前缀: string, 限: int) =
  let 索引 = 统计索引(根)
  var 命中 = 0
  for 项 in 索引.行s:
    if 命中 >= 限: break
    if 项.文本.contains(前缀) or 项.源名.contains(前缀):
      命中 += 1
      echo 项.源名 & ":" & $项.行号 & "  " & 项.文本
  if 命中 == 0:
    echo "未命中: " & 前缀
  else:
    echo "命中 " & $命中 & " 行; 提示词: 请基于上述《源名》行继续写, 前缀=" & 前缀

proc 补(根, 前缀: string, 限: int) =
  let 索引 = 统计索引(根)
  var 命中 = 0
  var 行s: seq[string] = @[]
  for 项 in 索引.行s:
    if 命中 >= 限: break
    if 项.文本.contains(前缀):
      命中 += 1
      行s.add "- 《" & 项.源名 & "》第" & $项.行号 & "行: " & 项.文本
  if 命中 == 0:
    echo "补: 未命中 " & 前缀
  else:
    echo "提示词草稿:"
    echo "请基于以下已索引事实继续写, 前缀=" & 前缀 & ":"
    for l in 行s: echo l

proc 进度条(百: int): string =
  let 满 = 百 div 5
  result = "[" & repeat('#', 满) & repeat('.', 20 - 满) & "]"

proc 逃(s: string): string =
  result = s.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;").replace("\"", "&quot;")

proc 报(根: string, 出html: bool) =
  let 目录 = 读目录(根)
  let 状态s = 读状态(根)
  let 索引 = 统计索引(根)
  var 已收 = 0
  var 入 = 0
  var 去 = 0
  var 现行 = 0
  var 已收源 = 0
  var 行html: seq[string] = @[]
  for 源名, 项 in 目录:
    let st = 状态s.getOrDefault(源名, 状态项())
    let p = 绝对(根, 项.路径)
    let n = if fileExists(p): 取行(readFile(p)).len else: 0
    已收 += st.已收行
    入 += st.入索引
    去 += st.去重
    现行 += n
    已收源 += ord(状态s.hasKey(源名))
    let 百源 = min(100, st.已收行 * 100 div max(1, n))
    行html.add "<tr><td>" & 逃(源名) & "</td><td>" & $st.已收行 & "/" & $n & "</td><td>" & $st.入索引 & "</td><td>" & $st.去重 & "</td><td><div class=\"bar\"><span style=\"width:" & $百源 & "%\"></span></div></td></tr>"
  let 差 = 入 + 去 - 已收
  let 百 = if 现行 == 0: 100 else: min(100, 已收 * 100 div 现行)
  let 源百 = 已收源 * 100 div max(1, 目录.len)
  let 自我感 = 目录.hasKey("微加载.微加载.chain") and 索引.源计.getOrDefault("微加载.微加载.chain", 0) > 0
  let 认知感 = 已收源 == 目录.len
  let 致命 = 差 == 0
  var 编页 = 0
  var 冲数 = 0
  var 关数 = 0
  for 项 in [("编目.chain", "gttx://编目|"), ("冲突.chain", "gttx://冲突|"), ("关联.chain", "gttx://关联|")]:
    let 名 = 项[0]
    let 前缀 = 项[1]
    let f = 根 / 名
    if fileExists(f):
      for 行 in readFile(f).splitLines():
        if 行.startsWith(前缀):
          if 名 == "编目.chain": 编页 += 1
          elif 名 == "冲突.chain": 冲数 += 1
          else: 关数 += 1
  let 编目感 = 编页 > 0
  let 冲突感 = 冲数 > 0
  let 关联感 = 关数 > 0
  echo "在册源 " & $目录.len & " / 已收源 " & $已收源 & " (" & $源百 & "%)"
  echo "进度 " & 进度条(百) & " " & $百 & "% (" & $已收 & "/" & $现行 & " 行)"
  echo "天平 入索引 " & $入 & " + 去重 " & $去 & " = 已收 " & $已收 & " / 差 " & $差 & (if 差 == 0: " 平" else: " 不平")
  echo "索引 " & $索引.行s.len & " 行 / 指纹唯一 " & $索引.指纹s.len
  echo "编目 页 " & $编页 & " / 冲突 " & $冲数 & " / 关联 " & $关数
  echo "考核 自我感 " & (if 自我感: "过" else: "未过") & " / 认知感 " & (if 认知感: "过" else: "未过") & " / 编目感 " & (if 编目感: "过" else: "未过") & " / 冲突感 " & (if 冲突感: "过" else: "未过") & " / 关联感 " & (if 关联感: "过" else: "未过") & " / 致命错误 " & (if 致命: "0 过" else: "非0 拒")
  if 出html:
    let 页 = "<!doctype html><meta charset=\"utf-8\"><title>速阅</title>" &
      "<style>body{font-family:system-ui,sans-serif;margin:24px;background:#111;color:#eee}" &
      "table{border-collapse:collapse;width:100%}td,th{border:1px solid #444;padding:6px;text-align:left}" &
      ".bar{background:#333;width:160px;height:12px}.bar span{display:block;height:12px;background:#4caf50}" &
      "h1{font-size:20px}</style>" &
      "<h1>微加载 · 全局速阅</h1>" &
      "<p>在册源 " & $目录.len & " / 已收源 " & $已收源 & " (" & $源百 & "%)</p>" &
      "<p>进度 " & 进度条(百) & " " & $百 & "% (" & $已收 & "/" & $现行 & " 行)</p>" &
      "<p>天平 入索引 " & $入 & " + 去重 " & $去 & " = 已收 " & $已收 & " / 差 " & $差 & "</p>" &
      "<p>自我感 " & (if 自我感: "过" else: "未过") & " / 认知感 " & (if 认知感: "过" else: "未过") & " / 编目感 " & (if 编目感: "过" else: "未过") & " / 冲突感 " & (if 冲突感: "过" else: "未过") & " / 关联感 " & (if 关联感: "过" else: "未过") & " / 致命错误 " & (if 致命: "0" else: "非0") & "</p>" &
      "<p>编目 " & $编页 & " 页 / 冲突 " & $冲数 & " / 关联 " & $关数 & "</p>" &
      "<p><a href=\"编目.html\" style=\"color:#7cc4ff\">打开编目</a></p>" &
      "<table><tr><th>源</th><th>已收/现行</th><th>入索引</th><th>去重</th><th>进度</th></tr>" &
      行html.join("") & "</table>"
    writeFile(根 / "速阅.html", 页)
    echo "可视: " & (根 / "速阅.html")

proc 扫(根, 目标: string) =
  let 目录 = 读目录(根)
  var 已知 = initHashSet[string]()
  for _, 项 in 目录: 已知.incl 项.路径
  var 加 = 0
  let 绝 = 绝对(根, 目标)
  for p in walkDirRec(绝):
    if p.contains("/.git/") or p.contains("/node_modules/") or p.contains("/bin/") or p.contains("/nimcache/") or p.contains("/archive/") or p.contains("/release/"): continue
    let e = p.splitFile.ext.toLowerAscii
    if e notin [".md", ".chain", ".toml", ".txt", ".nim"]: continue
    let 相 = relativePath(p, 根)
    if 已知.contains(相): continue
    let 标 = 相.replace("../", "")
    let 部 = 标.split('/')[0]
    let 源名 = "文档." & 标.replace("/", ".")
    追加(根 / "目录.chain", "gttx://目录|" & 源名 & "|" & 相 & "|文本|" & 部)
    已知.incl 相
    加 += 1
  echo "扫 " & 目标 & ": 新增 " & $加 & " 源, 在册 " & $(目录.len + 加)

# —— 编目 / 冲突 / 关联: 纯规则派生层, 不调用大模型 ——

const 停词 = [
  "的", "了", "是", "在", "和", "与", "及", "或", "这", "那", "我", "你", "他", "她", "它",
  "我们", "你们", "他们", "一个", "什么", "怎么", "为什么", "可以", "需要", "因为", "所以",
  "如果", "就是", "没有", "已经", "还是", "这个", "那个", "自己", "现在", "然后", "但是",
  "而且", "以及", "通过", "进行", "使用", "时候", "东西", "事情", "里面", "这里", "那里",
  "上面", "下面", "前面", "后面", "不是", "不能", "不会", "可能", "应该", "必须", "一定",
  "所有", "任何", "每个", "一些", "这些", "那些", "其中", "其他", "之后", "之前", "同时",
  "内容", "问题", "方案", "编程", "数据", "系统", "地址", "方式", "方法", "过程", "状态",
  "关系", "结构", "规则", "定义", "实现", "设计", "语言", "文字", "意思", "目的", "目标",
  "第一", "第二", "第三", "今天", "明天", "昨天"
]

let 停词集 = block:
  var s = initHashSet[string]()
  for w in 停词: s.incl w
  s

proc 是汉(r: Rune): bool =
  result = r.int >= 0x4E00 and r.int <= 0x9FFF

proc 截(s: string, n: int): string =
  var c = 0
  for r in s.runes:
    if c >= n: break
    result.add $r
    c += 1

proc 取词(s: string): seq[string] =
  var 计 = initTable[string, int]()
  var 序: seq[string] = @[]
  template 加(w: string) =
    if w.len >= 2 and not 停词集.contains(w):
      if 计.hasKey(w): 计[w] += 1
      else:
        计[w] = 1
        序.add w
  var 字: seq[string] = @[]
  for r in s.runes:
    if 是汉(r):
      字.add $r
    else:
      for n in 2 .. 4:
        if 字.len >= n:
          for i in 0 .. 字.len - n:
            var w = ""
            for j in 0 ..< n: w.add 字[i + j]
            加(w)
      字 = @[]
  for n in 2 .. 4:
    if 字.len >= n:
      for i in 0 .. 字.len - n:
        var w = ""
        for j in 0 ..< n: w.add 字[i + j]
        加(w)
  var w = ""
  for c in s:
    if c.isAlphaNumeric:
      w.add c.toLowerAscii
    else:
      if w.len >= 2: 加(w)
      w = ""
  if w.len >= 2: 加(w)
  var 词s: seq[(string, int)] = @[]
  for x in 序: 词s.add (x, 计[x])
  词s.sort(proc(a, b: (string, int)): int =
    result = cmp(b[1], a[1])
    if result == 0: result = cmp(a[0], b[0]))
  result = @[]
  for i in 0 ..< min(6, 词s.len): result.add 词s[i][0]

proc 标题化(s: string): string =
  result = s.strip
  while result.len > 0 and result.startsWith("#"):
    result = result[1 .. ^1].strip
  if result.startsWith("gttx://"):
    result = result[7 .. ^1].strip
  result = 截(result, 40)
  if result.len == 0: result = "未命名"

proc 表题(源名: string): string =
  result = 源名
  if result.endsWith(".chain") and result.len > 6:
    result = result[0 ..< result.len - 6]
  let 段 = result.split('.')
  if 段.len > 0 and 段[^1].len > 0: result = 段[^1]
  if result.len == 0: result = 源名

proc 摘要(行s: seq[string], 起, 止: int): string =
  var 片: seq[string] = @[]
  for i in max(0, 起) .. min(止, 行s.len - 1):
    let t = 行s[i].strip
    if t.len > 0: 片.add t
    if 片.len >= 2: break
  result = 截(片.join(" "), 120)

proc 切页(根: string, 目录: OrderedTable[string, 目录项]): seq[切片项] =
  var 页号 = 0
  for 源名, 项 in 目录:
    let p = 绝对(根, 项.路径)
    if not fileExists(p): continue
    let 行s = 取源行(p)
    if 行s.len == 0: continue
    let 部 = 项.部
    if 项.路径.toLowerAscii.endsWith(".chain"):
      页号 += 1
      let 题 = 表题(源名)
      let 摘 = 摘要(行s, 0, 行s.len - 1)
      let 词s = 取词(题 & " " & 摘)
      let 簇 = if 词s.len > 0: 词s[0] else: 部
      result.add 切片项(页号: 页号, 标题: 题, 簇: 簇, 部: 部, 源名: 源名,
        起行: 1, 止行: 行s.len, 摘要: 摘, 文本: 行s, 词s: 词s)
      continue
    var 标s: seq[(int, string)] = @[]
    for i in 0 ..< 行s.len:
      let l = 行s[i].strip
      if l.startsWith("#"):
        标s.add (i, 标题化(l))
    if 标s.len == 0:
      页号 += 1
      let 题 = 标题化(行s[0])
      let 摘 = 摘要(行s, 0, 行s.len - 1)
      let 词s = 取词(题 & " " & 摘)
      let 簇 = if 词s.len > 0: 词s[0] else: 部
      result.add 切片项(页号: 页号, 标题: 题, 簇: 簇, 部: 部, 源名: 源名,
        起行: 1, 止行: 行s.len, 摘要: 摘, 文本: 行s, 词s: 词s)
      continue
    if 标s[0][0] > 0:
      页号 += 1
      let 题 = 标题化(行s[0])
      let 摘 = 摘要(行s, 0, 标s[0][0] - 1)
      let 词s = 取词(题 & " " & 摘)
      let 簇 = if 词s.len > 0: 词s[0] else: 部
      result.add 切片项(页号: 页号, 标题: 题, 簇: 簇, 部: 部, 源名: 源名,
        起行: 1, 止行: 标s[0][0], 摘要: 摘, 文本: 行s[0 ..< 标s[0][0]], 词s: 词s)
    for k in 0 ..< 标s.len:
      let 起 = 标s[k][0]
      let 止 = if k + 1 < 标s.len: 标s[k + 1][0] - 1 else: 行s.len - 1
      if 止 < 起: continue
      页号 += 1
      let 题 = 标s[k][1]
      let 摘 = 摘要(行s, 起, 止)
      let 词s = 取词(题 & " " & 摘)
      let 簇 = if 词s.len > 0: 词s[0] else: 部
      result.add 切片项(页号: 页号, 标题: 题, 簇: 簇, 部: 部, 源名: 源名,
        起行: 起 + 1, 止行: 止 + 1, 摘要: 摘, 文本: 行s[起 .. 止], 词s: 词s)

proc 找数字(s: string): seq[(int, int, string)] =
  var i = 0
  while i < s.len:
    if s[i].isDigit:
      let 起 = i
      while i < s.len and s[i].isDigit: i += 1
      result.add (起, i - 1, s[起 ..< i])
    else:
      i += 1

proc 取冲突(页s: seq[切片项], 目录: OrderedTable[string, 目录项], 根: string): seq[冲突项] =
  var 范围 = initTable[string, seq[(int, int, int)]]()
  for p in 页s:
    if not 范围.hasKey(p.源名): 范围[p.源名] = @[]
    范围[p.源名].add (p.起行, p.止行, p.页号)
  var 已 = initHashSet[string]()
  var 果: seq[冲突项] = @[]
  template 加冲(题, 差, 凭: string, 页: int) =
    let k = 题 & "|" & 差
    if not 已.contains(k):
      已.incl k
      果.add 冲突项(号: 果.len + 1, 主题: 题, 差异: 差, 状态: "待裁",
        证据: 凭, 甲页: 页, 乙页: 页)
  for 源名, 项 in 目录:
    let p = 绝对(根, 项.路径)
    if not fileExists(p): continue
    let 行s = 取源行(p)
    for i in 0 ..< 行s.len:
      let 行号 = i + 1
      var 页号 = 0
      if 范围.hasKey(源名):
        for r in 范围[源名]:
          if 行号 >= r[0] and 行号 <= r[1]:
            页号 = r[2]
            break
      if 页号 == 0: continue
      let t = 行s[i]
      if t.strip.startsWith("#"): continue
      let 标 = t.contains("矛盾") or t.contains("不一致") or t.contains("对不上")
      let 冲词 = t.contains("冲突")
      let 待 = t.contains("待裁|") or t.contains("待裁¦") or t.contains("待裁:") or t.contains("待裁 ") or t.contains("待裁,") or t.endsWith("待裁")
      let 数s = 找数字(t)
      let 主题 = if t.startsWith("gttx://"):
        let 域 = t.split('|')
        if 域.len >= 2: 截(域[1], 40) else: 截(t, 40)
      else:
        页s[页号 - 1].标题
      var 加对 = false
      let i源 = t.find("源称")
      let i行 = t.find("逐行")
      if i源 >= 0 and i行 > i源:
        var 源数 = ""
        var 行数 = ""
        for d in 数s:
          if d[0] > i源 and d[0] < i行 and 源数.len == 0: 源数 = d[2]
          if d[0] > i行 and 行数.len == 0: 行数 = d[2]
        if 源数.len > 0 and 行数.len > 0 and 源数 != 行数:
          加冲(主题, "源称" & 源数 & " vs 逐行" & 行数, 源名 & ":" & $行号, 页号)
          加对 = true
      for a in 0 ..< 数s.len:
        for b in a + 1 ..< 数s.len:
          if 数s[a][2] == 数s[b][2]: continue
          let 中 = if 数s[a][1] + 1 <= 数s[b][0] - 1:
            t[数s[a][1] + 1 ..< 数s[b][0]].strip
          else: ""
          let 纯 = 中 == "/" or 中 == "vs" or 中 == "VS" or 中 == "→" or 中 == "->"
          if 纯 and not t.contains("不采用") and (标 or 冲词 or 待 or 中 == "vs" or 中 == "VS"):
            加冲(主题, 数s[a][2] & " vs " & 数s[b][2], 源名 & ":" & $行号, 页号)
            加对 = true
      if 待 and not 加对 and 数s.len >= 2:
        加冲(主题, "待裁: " & 截(t, 60), 源名 & ":" & $行号, 页号)
  果.sort(proc(a, b: 冲突项): int =
    result = cmp(a.甲页, b.甲页)
    if result == 0: result = cmp(a.证据, b.证据)
    if result == 0: result = cmp(a.差异, b.差异))
  if 果.len > 300: 果.setLen(300)
  for i in 0 ..< 果.len: 果[i].号 = i + 1
  result = 果

proc 取关联(页s: seq[切片项]): seq[关联项] =
  var 词页 = initOrderedTable[string, seq[int]]()
  for p in 页s:
    for w in p.词s:
      if not 词页.hasKey(w): 词页[w] = @[]
      词页[w].add p.页号
  var 分 = initTable[string, int]()
  var 证 = initTable[string, string]()
  for w, ps in 词页:
    for i in 0 ..< ps.len:
      for j in i + 1 ..< ps.len:
        let a = min(ps[i], ps[j])
        let b = max(ps[i], ps[j])
        if a == b: continue
        let k = $a & ":" & $b
        分[k] = 分.getOrDefault(k, 0) + 1
        if not 证.hasKey(k): 证[k] = w
  var 对s: seq[(int, int, int, string)] = @[]
  for k, v in 分:
    let ab = k.split(':')
    if ab.len != 2: continue
    对s.add (parseInt(ab[0]), parseInt(ab[1]), v, 证[k])
  对s.sort(proc(a, b: (int, int, int, string)): int =
    result = cmp(a[0], b[0])
    if result == 0: result = cmp(a[1], b[1]))
  var 每页 = initTable[int, seq[(int, int, string)]]()
  for x in 对s:
    if not 每页.hasKey(x[0]): 每页[x[0]] = @[]
    每页[x[0]].add (x[1], x[2], x[3])
  var 果: seq[关联项] = @[]
  for a in 0 ..< 页s.len:
    let 页号 = a + 1
    if not 每页.hasKey(页号): continue
    var 候 = 每页[页号]
    候.sort(proc(x, y: (int, int, string)): int =
      result = cmp(y[1], x[1])
      if result == 0: result = cmp(x[0], y[0]))
    for i in 0 ..< min(5, 候.len):
      果.add 关联项(甲页: 页号, 关系: "同词:" & 候[i][2], 乙页: 候[i][0], 证据: 候[i][2])
  result = 果

proc 写派生(路径, 头: string, 行s: seq[string]) =
  var 全: seq[string] = @[头]
  全.add 行s
  writeFile(路径, 全.join("\n") & "\n")

proc 编目(根: string) =
  let 目录 = 读目录(根)
  let 页s = 切页(根, 目录)
  doAssert 页s.len > 0, "编目无页"
  let 冲突s = 取冲突(页s, 目录, 根)
  let 关联s = 取关联(页s)
  var 编目行: seq[string] = @[]
  for p in 页s:
    编目行.add "gttx://编目|" & $p.页号 & "|" & 清洗(p.标题) & "|" & 清洗(p.簇) & "|" &
      清洗(p.部) & "|" & 清洗(p.源名) & "|" & $p.起行 & "|" & $p.止行 & "|" &
      清洗(p.摘要) & "|已编"
  var 冲突行: seq[string] = @[]
  for c in 冲突s:
    冲突行.add "gttx://冲突|" & $c.号 & "|" & 清洗(c.主题) & "|" & $c.甲页 & "|" &
      $c.乙页 & "|" & 清洗(c.差异) & "|" & 清洗(c.状态) & "|" & 清洗(c.证据)
  var 关联行: seq[string] = @[]
  for r in 关联s:
    关联行.add "gttx://关联|" & $r.甲页 & "|" & 清洗(r.关系) & "|" & $r.乙页 & "|" & 清洗(r.证据)
  写派生(根 / "编目.chain", "# 派生表: 编目; 由源文件与索引生成, 可重建", 编目行)
  写派生(根 / "冲突.chain", "# 派生表: 冲突; 规则生成, 状态=待裁, 不自动裁决", 冲突行)
  写派生(根 / "关联.chain", "# 派生表: 关联; 同词边, 甲页<乙页", 关联行)

  var 页J = newJArray()
  for p in 页s:
    var 关J = newJArray()
    for r in 关联s:
      if r.甲页 == p.页号:
        关J.add %*{"n": r.乙页, "k": r.关系}
      elif r.乙页 == p.页号:
        关J.add %*{"n": r.甲页, "k": r.关系}
    var 冲J = newJArray()
    for c in 冲突s:
      if c.甲页 == p.页号:
        冲J.add %*{"n": c.号, "d": c.差异, "e": c.证据}
    页J.add %*{"n": p.页号, "t": p.标题, "c": p.簇, "b": p.部, "s": p.源名,
      "a": p.起行, "e": p.止行, "x": p.摘要, "k": p.词s.join(", "),
      "l": p.文本, "r": 关J, "f": 冲J}
  var 冲J = newJArray()
  for c in 冲突s:
    冲J.add %*{"n": c.号, "t": c.主题, "d": c.差异, "s": c.状态, "e": c.证据,
      "a": c.甲页, "b": c.乙页}
  let 页JSON = ($页J).replace("</", "<\\/")
  let 冲JSON = ($冲J).replace("</", "<\\/")
  let 统计 = $页s.len & " 页 / " & $冲突s.len & " 冲突 / " & $关联s.len & " 关联"
  let 模板 = """<!doctype html><meta charset="utf-8"><title>微加载 · 编目</title>
<style>
:root{--bg:#111;--fg:#e8e8e8;--mut:#9aa;--card:#1b1d21;--line:#333;--acc:#7cc4ff;--warn:#ffb86b}
*{box-sizing:border-box}body{margin:0;background:var(--bg);color:var(--fg);font-family:system-ui,"Noto Sans CJK SC",sans-serif}
header{padding:14px 18px;border-bottom:1px solid var(--line);display:flex;gap:14px;align-items:center;flex-wrap:wrap}
h1{font-size:18px;margin:0;font-weight:600}.mut{color:var(--mut);font-size:12px}
input{background:#0b0c0e;border:1px solid var(--line);color:var(--fg);padding:8px 10px;border-radius:8px;min-width:280px;flex:1;max-width:520px}
a{color:var(--acc);text-decoration:none}
#wrap{display:grid;grid-template-columns:220px 1fr;min-height:calc(100vh - 62px)}
#clusters{border-right:1px solid var(--line);padding:12px;overflow:auto;max-height:calc(100vh - 62px)}
#clusters button{display:block;width:100%;text-align:left;background:none;border:0;color:var(--fg);padding:7px 8px;border-radius:6px;cursor:pointer;font-size:13px}
#clusters button:hover,#clusters button.on{background:#232830}
main{padding:14px 18px;overflow:auto;max-height:calc(100vh - 62px)}
.tabs{display:flex;gap:8px;margin-bottom:12px}.tabs button{background:#1b1d21;border:1px solid var(--line);color:var(--fg);padding:7px 12px;border-radius:8px;cursor:pointer}
.tabs button.on{border-color:var(--acc);color:var(--acc)}
.card{background:var(--card);border:1px solid var(--line);border-radius:10px;padding:11px 13px;margin:0 0 10px;cursor:pointer}
.card:hover{border-color:#4a5560}.title{font-weight:600;margin-bottom:4px}.meta{color:var(--mut);font-size:12px;margin-bottom:6px}
.sum{font-size:13px;color:#cfd6dd;line-height:1.5}
.warn{color:var(--warn)}#detail{display:none;position:fixed;right:0;top:0;bottom:0;width:min(720px,70vw);background:#0d0f12;border-left:1px solid var(--line);padding:18px;overflow:auto;z-index:9;box-shadow:-20px 0 50px #000a}
#detail.open{display:block}#detail h2{font-size:17px;margin:0 0 6px}
.ln{display:flex;gap:10px;font-family:ui-monospace,monospace;font-size:12px;line-height:1.55;white-space:pre-wrap;word-break:break-word;border-bottom:1px solid #191c20;padding:2px 0}
.ln .no{color:#667;min-width:44px;text-align:right;flex:none}
.pill{display:inline-block;border:1px solid var(--line);border-radius:999px;padding:2px 8px;margin:2px 4px 2px 0;font-size:12px;color:#cfe}
.close{position:absolute;right:14px;top:10px;background:#232830;border:1px solid var(--line);color:var(--fg);border-radius:8px;padding:5px 9px;cursor:pointer}
@media(max-width:800px){#wrap{grid-template-columns:1fr}#clusters{display:none}#detail{width:100vw}}
</style>
<header><h1>微加载 · 编目</h1><span class="mut" id="stats">__STATS__</span><input id="q" placeholder="搜主题、词、来源、正文"><a href="速阅.html">进度</a></header>
<div id="wrap"><nav id="clusters"></nav><main><div class="tabs"><button id="tab-pages" class="on">编目</button><button id="tab-conf">冲突</button></div><div id="list"></div></main></div>
<aside id="detail"><button class="close" id="close">关闭</button><div id="dbody"></div></aside>
<script>
const PAGES=__PAGES__;
const CONFLICTS=__CONFLICTS__;
const $=s=>document.querySelector(s);let curCluster="",curTab="pages";
const el=(t,c,x)=>{const e=document.createElement(t);if(c)e.className=c;if(x!==undefined)e.textContent=x;return e};
function card(p){const d=el("div","card");d.onclick=()=>showPage(p.n);d.appendChild(el("div","title","#"+p.n+"  "+p.t));d.appendChild(el("div","meta",p.c+" · "+p.b+" · "+p.s+":"+p.a+"-"+p.e));d.appendChild(el("div","sum",p.x));return d}
function render(){const q=$("#q").value.trim().toLowerCase();const L=$("#list");L.innerHTML="";
 if(curTab==="conf"){L.appendChild(el("div","mut","共 "+CONFLICTS.length+" 条待裁"));CONFLICTS.forEach(c=>{const d=el("div","card");d.onclick=()=>showConflict(c.n);d.appendChild(el("div","title","冲突 #"+c.n+"  "+c.t));d.appendChild(el("div","meta",c.e));d.appendChild(el("div","sum warn",c.d));L.appendChild(d)});return}
 const ps=PAGES.filter(p=>(!curCluster||p.c===curCluster)&&(!q||(p.t+" "+p.c+" "+p.b+" "+p.s+" "+p.x+" "+(p.l||[]).join(" ")).toLowerCase().includes(q)));
 L.appendChild(el("div","mut","显示 "+Math.min(ps.length,300)+" / "+ps.length+" 页"));
 ps.slice(0,300).forEach(p=>L.appendChild(card(p)))}
function renderClusters(){const C={};PAGES.forEach(p=>C[p.c]=(C[p.c]||0)+1);const ks=Object.keys(C).sort((a,b)=>C[b]-C[a]);const n=$("#clusters");n.innerHTML="";
 const all=el("button","on","全部 ("+PAGES.length+")");all.onclick=()=>{curCluster="";[...n.children].forEach(x=>x.classList.remove("on"));all.classList.add("on");render()};n.appendChild(all);
 ks.slice(0,80).forEach(k=>{const b=el("button",null,k+" ("+C[k]+")");b.onclick=()=>{curCluster=k;[...n.children].forEach(x=>x.classList.remove("on"));b.classList.add("on");render()};n.appendChild(b)})}
function showPage(n){const p=PAGES.find(x=>x.n===n);if(!p)return;const d=$("#dbody");d.innerHTML="";d.appendChild(el("h2","","#"+p.n+"  "+p.t));d.appendChild(el("div","mut",p.c+" · "+p.b+" · "+p.s+":"+p.a+"-"+p.e));
 const kw=el("div");kw.appendChild(el("span","mut","词: "));(p.k||"").split(", ").filter(Boolean).forEach(w=>kw.appendChild(el("span","pill",w)));d.appendChild(kw);
 if((p.r||[]).length){const r=el("div");r.appendChild(el("span","mut","关联: "));p.r.forEach(x=>{const b=el("span","pill","#"+x.n+" "+x.k);b.style.cursor="pointer";b.onclick=()=>showPage(x.n);r.appendChild(b)});d.appendChild(r)}
 if((p.f||[]).length){const f=el("div");f.appendChild(el("span","mut","冲突: "));p.f.forEach(x=>{const b=el("span","pill warn","#"+x.n+" "+x.d);b.style.cursor="pointer";b.onclick=()=>showConflict(x.n);f.appendChild(b)});d.appendChild(f)}
 const box=el("div");(p.l||[]).slice(0,600).forEach((t,i)=>{const ln=el("div","ln");ln.appendChild(el("span","no",String(p.a+i)));ln.appendChild(el("span",null,t));box.appendChild(ln)});d.appendChild(box);
 if((p.l||[]).length>600)d.appendChild(el("div","mut","… 已截断, 共 "+(p.l||[]).length+" 行"));
 $("#detail").classList.add("open")}
function showConflict(n){const c=CONFLICTS.find(x=>x.n===n);if(!c)return;const d=$("#dbody");d.innerHTML="";d.appendChild(el("h2","","冲突 #"+c.n+"  "+c.t));d.appendChild(el("div","mut","证据 "+c.e+" · 状态 "+c.s));d.appendChild(el("div","sum warn",c.d));
 const b1=el("button","pill","看甲页 #"+c.a);b1.onclick=()=>showPage(c.a);const b2=el("button","pill","看乙页 #"+c.b);b2.onclick=()=>showPage(c.b);d.appendChild(b1);d.appendChild(b2);$("#detail").classList.add("open")}
$("#q").oninput=render;$("#close").onclick=()=>$("#detail").classList.remove("open");
$("#tab-pages").onclick=()=>{curTab="pages";$("#tab-pages").classList.add("on");$("#tab-conf").classList.remove("on");render()};
$("#tab-conf").onclick=()=>{curTab="conf";$("#tab-conf").classList.add("on");$("#tab-pages").classList.remove("on");render()};
renderClusters();render();
</script>"""
  let 页 = 模板.replace("__STATS__", 统计).replace("__PAGES__", 页JSON).replace("__CONFLICTS__", 冲JSON)
  writeFile(根 / "编目.html", 页)
  echo "编目 " & 统计
  echo "可视: " & (根 / "编目.html")

proc main() =
  解析参数()
  let 根 = 根路径()
  if 参表.len == 0:
    echo "用法: 微加载 收 <源名> | 收全 | 取 <源名> <url> | 索 <前缀> | 补 <前缀> | 编目 | 报 [--html] | 扫 <路径>"
    quit(1)
  let 令 = 参表[0]
  if 令 == "收":
    doAssert 参表.len >= 2, "收 需要源名"
    discard 收(根, 参表[1])
  elif 令 == "收全":
    收全(根)
  elif 令 == "取":
    doAssert 参表.len >= 3, "取 需要源名与链接"
    取(根, 参表[1], 参表[2])
  elif 令 == "索":
    doAssert 参表.len >= 2, "索 需要前缀"
    索(根, 参表[1], 参限)
  elif 令 == "补":
    doAssert 参表.len >= 2, "补 需要前缀"
    补(根, 参表[1], 参限)
  elif 令 == "编目":
    编目(根)
  elif 令 == "报":
    报(根, 参html)
  elif 令 == "扫":
    doAssert 参表.len >= 2, "扫 需要路径"
    扫(根, 参表[1])
  else:
    stderr.writeLine("未知命令: " & 令)
    quit(1)

when isMainModule:
  main()
