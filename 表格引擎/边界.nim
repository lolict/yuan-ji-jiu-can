import std/strutils

type
  单元类* = enum 文, 引, 墙, 头

  位域* = object
    名*: string
    宽*: int

  墙记录* = object
    总管*: string
    总管宽*: int
    域*: seq[位域]

  单元* = object
    case 类*: 单元类
    of 文, 引:
      文本*: string
    of 墙:
      记录s*: seq[墙记录]
    of 头:
      索引*: string
      容量*: int

  行* = object
    表*: string
    单元s*: seq[单元]

  解析结果* = object
    ok*: bool
    注释*: bool
    行*: 行
    错*: string

proc 置错(结果: var 解析结果, 消息: string) =
  结果.ok = false
  结果.错 = 消息

proc 找引号(s: string, 起: int): int =
  var i = 起 + 1
  while i < s.len:
    if s[i] == '\\':
      i += 2
    elif s[i] == '"':
      return i
    else:
      i += 1
  result = -1

proc 找花括号(s: string, 起: int): int =
  var 深 = 0
  var i = 起
  var 引 = false
  while i < s.len:
    let c = s[i]
    if 引:
      if c == '\\':
        i += 2
        continue
      if c == '"':
        引 = false
    else:
      if c == '"':
        引 = true
      elif c == '{':
        深 += 1
      elif c == '}':
        深 -= 1
        if 深 == 0:
          return i
    i += 1
  result = -1

proc 切分(s: string, 分隔: set[char]): seq[string] =
  var 段s: seq[string] = @[]
  var 段 = ""
  var 引 = false
  var i = 0
  while i < s.len:
    let c = s[i]
    if 引:
      if c == '\\' and i + 1 < s.len:
        段.add c
        段.add s[i + 1]
        i += 2
        continue
      if c == '"':
        引 = false
      段.add c
    else:
      if c == '"':
        引 = true
        段.add c
      elif c in 分隔:
        段s.add 段.strip
        段 = ""
      else:
        段.add c
    i += 1
  段s.add 段.strip
  result = 段s

proc 反转义(s: string): string =
  var i = 0
  while i < s.len:
    if s[i] == '\\' and i + 1 < s.len:
      result.add s[i + 1]
      i += 2
    else:
      result.add s[i]
      i += 1

proc 拆名宽(s: string, 名, 宽: var string): bool =
  let i = s.find(':')
  if i <= 0 or i == s.high:
    return false
  名 = s[0 ..< i].strip
  宽 = s[i + 1 .. ^1].strip
  result = 名.len > 0 and 宽.len > 0

proc 数宽(s: string, 宽: var int): bool =
  try:
    宽 = parseInt(s)
    result = 宽 > 0
  except ValueError:
    result = false

proc 像墙首(s: string): bool =
  if s.len < 2 or s[0] != '{':
    return false
  var 终 = s.len
  for c in [',', ';', '}', '|']:
    let k = s.find(c, 1)
    if k >= 0 and k < 终:
      终 = k
  let 首 = s[1 ..< 终].strip
  var 名, 宽S: string
  if not 拆名宽(首, 名, 宽S):
    return false
  var 宽: int
  result = 数宽(宽S, 宽)

proc 像围墙(内容: string): bool =
  if 内容.len < 4 or 内容[0] != '{' or 内容[^1] != '}':
    return false
  let 内 = 内容[1 ..< 内容.len - 1]
  let 记s = 切分(内, {';'})
  var 有 = false
  for 记录 in 记s:
    if 记录.len == 0:
      continue
    let 项s = 切分(记录, {','})
    for 项 in 项s:
      if 项.len == 0:
        return false
      var 名, 宽S: string
      if not 拆名宽(项, 名, 宽S):
        return false
      var 宽: int
      if not 数宽(宽S, 宽):
        return false
    有 = true
  result = 有

proc 解析墙(内容: string, 结果: var 解析结果): 单元 =
  let 内 = 内容[1 ..< 内容.len - 1]
  let 记s = 切分(内, {';'})
  var 记录s: seq[墙记录] = @[]
  for 记录 in 记s:
    if 记录.len == 0:
      continue
    let 项s = 切分(记录, {','})
    if 项s.len == 0:
      置错(结果, "空围墙记录")
      return
    var 名, 宽S: string
    if not 拆名宽(项s[0], 名, 宽S):
      置错(结果, "总管缺名或位宽: " & 项s[0])
      return
    var 宽: int
    if not 数宽(宽S, 宽):
      置错(结果, "总管位宽不是正整数: " & 宽S)
      return
    var 记 = 墙记录(总管: 名, 总管宽: 宽, 域: @[])
    for k in 1 ..< 项s.len:
      if 项s[k].len == 0:
        continue
      var 域名, 域宽S: string
      if not 拆名宽(项s[k], 域名, 域宽S):
        置错(结果, "位域缺名或位宽: " & 项s[k])
        return
      var 域宽: int
      if not 数宽(域宽S, 域宽):
        置错(结果, "位域宽不是正整数: " & 域宽S)
        return
      记.域.add 位域(名: 域名, 宽: 域宽)
    记录s.add 记
  if 记录s.len == 0:
    置错(结果, "围墙为空")
    return
  result = 单元(类: 墙, 记录s: 记录s)

proc 解析头(内容: string, 结果: var 解析结果): 单元 =
  let 体 = 内容[1 .. ^1].strip
  let i = 体.find(':')
  if i <= 0 or i == 体.high:
    置错(结果, "头部缺索引或容量: " & 体)
    return
  let 索引 = 体[0 ..< i].strip
  let 容量S = 体[i + 1 .. ^1].strip
  var 容量: int
  if not 数宽(容量S, 容量):
    置错(结果, "头部容量不是正整数: " & 容量S)
    return
  result = 单元(类: 头, 索引: 索引, 容量: 容量)

proc 墙总宽(u: 单元): int =
  for 记录 in u.记录s:
    result += 记录.总管宽
    for 域 in 记录.域:
      result += 域.宽

proc 解析行*(原: string, 严格 = false): 解析结果 =
  result.ok = true
  let s = 原.strip
  if s.len == 0 or s[0] == '#':
    result.注释 = true
    return
  if not s.startsWith("gttx://"):
    置错(result, "缺 gttx:// 入口")
    return
  let 表终 = s.find('|', 7)
  if 表终 < 0:
    置错(result, "缺表名分隔")
    return
  let 表名 = s[7 ..< 表终].strip
  if 表名.len == 0:
    置错(result, "表名为空")
    return
  result.行.表 = 表名

  var i = 表终 + 1
  var 待容量 = -1
  while true:
    if i >= s.len:
      置错(result, "尾随分隔符或空行")
      return
    let c = s[i]
    var 终 = -1
    var 单元v: 单元

    if c == '"':
      let j = 找引号(s, i)
      if j < 0:
        if 严格:
          置错(result, "引号未闭合")
          return
        let e = s.find('|', i)
        终 = if e < 0: s.len else: e
        let 文本 = s[i ..< 终].strip
        if 待容量 >= 0:
          置错(result, "头部后必须是围墙")
          return
        单元v = 单元(类: 文, 文本: 文本)
      else:
        var k = j + 1
        while k < s.len and s[k] == ' ':
          k += 1
        if k < s.len and s[k] != '|':
          if 严格:
            置错(result, "引号后有多余字符")
            return
          let e = s.find('|', i)
          终 = if e < 0: s.len else: e
          let 文本 = s[i ..< 终].strip
          if 待容量 >= 0:
            置错(result, "头部后必须是围墙")
            return
          单元v = 单元(类: 文, 文本: 文本)
        else:
          单元v = 单元(类: 引, 文本: 反转义(s[i + 1 ..< j]))
          终 = k

    elif c == '{':
      let j = 找花括号(s, i)
      let e = s.find('|', i)
      let 终片 = if e < 0: s.len else: e
      if j < 0:
        if 严格 or 像墙首(s[i ..< 终片]):
          置错(result, "围墙未闭合")
          return
        let 文本 = s[i ..< 终片].strip
        if 待容量 >= 0:
          置错(result, "头部后必须是围墙")
          return
        单元v = 单元(类: 文, 文本: 文本)
        终 = 终片
      elif 像围墙(s[i .. j]):
        单元v = 解析墙(s[i .. j], result)
        if not result.ok:
          return
        var k = j + 1
        while k < s.len and s[k] == ' ':
          k += 1
        终 = k
        if 终 < s.len and s[终] != '|':
          置错(result, "围墙后有多余字符")
          return
        if 待容量 >= 0:
          let 实宽 = 墙总宽(单元v)
          if 实宽 != 待容量:
            置错(result, "容量不符: 头 " & $待容量 & " / 墙 " & $实宽)
            return
          待容量 = -1
      else:
        if 严格:
          置错(result, "围墙格式不对")
          return
        let 文本 = s[i ..< 终片].strip
        if 待容量 >= 0:
          置错(result, "头部后必须是围墙")
          return
        单元v = 单元(类: 文, 文本: 文本)
        终 = 终片

    elif c == '@':
      let j = s.find('|', i)
      终 = if j < 0: s.len else: j
      单元v = 解析头(s[i ..< 终], result)
      if not result.ok:
        return
      if 待容量 >= 0:
        置错(result, "连续头部")
        return
      待容量 = 单元v.容量

    else:
      let j = s.find('|', i)
      终 = if j < 0: s.len else: j
      let 文本 = s[i ..< 终].strip
      if 文本.len == 0:
        置错(result, "空单元")
        return
      if 待容量 >= 0:
        置错(result, "头部后必须是围墙")
        return
      单元v = 单元(类: 文, 文本: 文本)

    result.行.单元s.add 单元v
    if 终 >= s.len:
      break
    i = 终 + 1
    if i >= s.len:
      置错(result, "尾随分隔符")
      return

  if 待容量 >= 0:
    置错(result, "头部后缺围墙")

proc 摘要(行: 行): string =
  result = 行.表 & " / " & $行.单元s.len & " 单元"
  for 单元v in 行.单元s:
    case 单元v.类
    of 文:
      result.add " 文:" & 单元v.文本
    of 引:
      result.add " 引:" & 单元v.文本
    of 墙:
      result.add " 墙:" & $单元v.记录s.len & " 记录"
    of 头:
      result.add " 头:" & 单元v.索引 & "=" & $单元v.容量
