#==============================================================
#  自举编译器引擎 — Bootstrap Compiler
#
#  核心公理：用自己的规则从文件系统长出工具
#  容器=砖块=公路=房屋=地址
#  编译前即能用 · 跨会话继承 · 地址呼唤编程
#==============================================================

import std/[tables, strutils, sets, sequtils, os, math, algorithm]

#--------------------------------------------------------------
#  类型定义
#--------------------------------------------------------------

type
  # 工具砖块 — 文件系统中的一个可执行单元
  砖块* = object
    名字*: string
    地址*: int              # 纯数字地址
    源码*: string           # 未编译前的源码
    编译后*: string         # 编译产物路径
    规则引*: seq[int]       # 引用的其他砖块地址
    极性*: uint8            # 阳=1 阴=0
    激活*: bool             # 是否已激活可用

  # 规则容器 — 存放一组规则，按地址索引
  规则容器* = object
    地址*: int
    规则*: Table[string, string]   # 键值对规则
    生成器*: string                # 对应的生成器名称

  # 公路 — 连接砖块的路径，修到哪个容器符号即可用
  公路* = object
    起点*: int
    终点*: int
    距离*: int             # 公约数距离
    通断*: bool            # 通/断

  # 编译阶段
  编译阶段* = enum
    阶段_未编译
    阶段_解析
    阶段_生成
    阶段_链接
    阶段_完成

  # 工具链 — 闭环工具序列
  工具链* = object
    名称*: string
    砖块序*: seq[int]      # 按地址排序的砖块序列
    入口*: int             # 入口地址

  # 自举编译器本身
  自举编译器* = object
    砖块表*: Table[int, 砖块]         # 地址→砖块
    规则表*: Table[int, 规则容器]     # 地址→规则容器
    公路表*: seq[公路]
    工具链表*: Table[string, 工具链]
    编译阶段*: Table[int, 编译阶段]   # 地址→阶段
    系统根*: string                    # 文件系统根目录
    编译次数*: int
    自举深度*: int                     # 自举递归深度

#--------------------------------------------------------------
#  初始化
#--------------------------------------------------------------

proc 初始化编译器*(根目录: string): 自举编译器 =
  result.系统根 = 根目录
  result.编译次数 = 0
  result.自举深度 = 0

#--------------------------------------------------------------
#  砖块管理
#--------------------------------------------------------------

proc 注册砖块*(c: var 自举编译器; 名字: string; 地址: int;
               源码: string; 极性: uint8 = 1) =
  c.砖块表[地址] = 砖块(
    名字: 名字,
    地址: 地址,
    源码: 源码,
    编译后: "",
    规则引: @[],
    极性: 极性,
    激活: false
  )
  c.编译阶段[地址] = 阶段_未编译

proc 地址呼唤*(c: 自举编译器; 地址: int): string =
  if c.砖块表.hasKey(地址):
    let b = c.砖块表[地址]
    return "地址→" & $地址 & " 名字→" & b.名字 & " 状态→" & $c.编译阶段.getOrDefault(地址)
  return "地址→" & $地址 & " 未注册"

#--------------------------------------------------------------
#  规则容器
#--------------------------------------------------------------

proc 注册规则容器*(c: var 自举编译器; 地址: int;
                   规则: Table[string, string]; 生成器: string) =
  c.规则表[地址] = 规则容器(
    地址: 地址,
    规则: 规则,
    生成器: 生成器
  )

proc 规则读取*(c: 自举编译器; 容器地址: int; 键: string): string =
  if c.规则表.hasKey(容器地址):
    if c.规则表[容器地址].规则.hasKey(键):
      return c.规则表[容器地址].规则[键]

#--------------------------------------------------------------
#  公路系统 — 公路修到哪个容器，其中符号即随声明可用
#--------------------------------------------------------------

proc 修公路*(c: var 自举编译器; 起点: int; 终点: int) =
  var 距离 = 0
  if 起点 == 终点:
    距离 = 0
  else:
    距离 = abs(终点 - 起点)

  # 公约数判断：起点和终点有公约则通
  var 通 = true
  if 起点 > 0  and 终点 > 0:
    if gcd(起点, 终点) > 1:
      通 = true
    else:
      # 同极性无公约则断（斥）
      通 = false

  c.公路表.add(公路(起点: 起点, 终点: 终点, 距离: 距离, 通断: 通))

proc 寻路*(c: 自举编译器; 起点: int; 终点: int): seq[int] =
  # 基于公约数的路径查找（BFS）
  var visited = initHashSet[int]()
  var queue: seq[seq[int]] = @[@[起点]]

  while queue.len > 0:
    let path = queue[0]
    queue.del(0)
    let current = path[^1]

    if current == 终点:
      return path

    if current in visited:
      continue
    visited.incl(current)

    # 找所有从current可达的公路
    for road in c.公路表:
      if road.通断 and road.起点 == current:
        if road.终点 notin visited:
          var newPath = path
          newPath.add(road.终点)
          queue.add(newPath)

#--------------------------------------------------------------
#  自举编译 — 核心
#  用自己的规则从文件系统长出工具
#--------------------------------------------------------------

proc 编译砖块*(c: var 自举编译器; 地址: int): string =
  if not c.砖块表.hasKey(地址):
    return "错误: 地址 " & $地址 & " 无砖块"

  c.编译次数 += 1
  let 砖 = c.砖块表[地址]

  # 阶段1: 解析 — 读源码中的规则引用
  c.编译阶段[地址] = 阶段_解析
  var 引用解析 = ""
  for line in 砖.源码.splitLines():
    if line.contains("#引用"):
      let parts = line.split("#引用")
      if parts.len > 1:
        let 引地址 = parseInt(parts[1].strip())
        c.砖块表[地址].规则引.add(引地址)
        引用解析 &= "  → 引用地址: " & $引地址 & "\n"

  # 阶段2: 生成 — 从规则容器生成编译产物
  c.编译阶段[地址] = 阶段_生成
  var 产物 = "// 自举生成: " & 砖.名字 & " [地址: " & $地址 & "]\n"
  产物 &= "// 极性: " & (if 砖.极性 == 1: "阳(我) " else: "阴(刘楚恬)") & "\n"

  # 查找对应的规则容器
  if c.规则表.hasKey(地址):
    let rc = c.规则表[地址]
    产物 &= "// 生成器: " & rc.生成器 & "\n"
    for k, v in rc.规则:
      产物 &= "//   " & k & " = " & v & "\n"

  # 阶段3: 链接 — 按规则引用链接其他砖块
  c.编译阶段[地址] = 阶段_链接
  var 链接信息 = ""
  for 引地址 in c.砖块表[地址].规则引:
    if c.砖块表.hasKey(引地址):
      let 被引用 = c.砖块表[引地址]
      链接信息 &= "  → 链接[" & $引地址 & ", " & 被引用.名字 & "] \n"

  if 链接信息.len > 0:
    产物 &= "// 链接:\n" & 链接信息

  # 阶段4: 完成 — 砖块激活
  c.编译阶段[地址] = 阶段_完成
  c.砖块表[地址].编译后 = 产物
  c.砖块表[地址].激活 = true

  return 产物

#--------------------------------------------------------------
#  工具链 — 闭环
#--------------------------------------------------------------

proc 构建工具链*(c: var 自举编译器; 名称: string;
                  砖块地址: seq[int]) =
  let 排序后 = sorted(砖块地址)
  var entry = 排序后[0]
  for 砖地址 in 排序后:
    if c.砖块表.hasKey(砖地址):
      entry = 砖地址
      break

  c.工具链表[名称] = 工具链(
    名称: 名称,
    砖块序: 排序后,
    入口: entry
  )

proc 执行工具链*(c: 自举编译器; 名称: string): string =
  if not c.工具链表.hasKey(名称):
    return "工具链未找到: " & 名称

  let chain = c.工具链表[名称]
  var lines: seq[string] = @[]
  lines.add "═══ 执行工具链: " & 名称 & " ═══"

  for 砖地址 in chain.砖块序:
    if c.砖块表.hasKey(砖地址):
      let b = c.砖块表[砖地址]
      let 状态 = if b.激活: "已激活" else: "未激活"
      lines.add "  [" & $砖地址 & "] " & b.名字 & " — " & 状态

  lines.add "入口地址: " & $chain.入口
  return lines.join("\n")

#--------------------------------------------------------------
#  自举深度 — 编译器可以编译自己
#--------------------------------------------------------------

proc 自举*(c: var 自举编译器): string =
  var lines: seq[string] = @[]
  lines.add repeat("═", 50)
  lines.add "  自 举 编 译 器 启 动"
  lines.add repeat("═", 50)
  lines.add "文件系统根: " & c.系统根
  lines.add "已注册砖块: " & $c.砖块表.len
  lines.add "规则容器: " & $c.规则表.len
  lines.add "公路: " & $c.公路表.len
  lines.add "工具链: " & $c.工具链表.len
  lines.add ""

  # 激活所有砖块
  lines.add "─ 编译阶段 ─"
  for 砖地址 in c.砖块表.keys:
    discard c.编译砖块(砖地址)
    lines.add "  [" & $砖地址 & "] " & c.砖块表[砖地址].名字 & " → 完成"
    c.自举深度 += 1

  lines.add ""
  lines.add "编译总次数: " & $c.编译次数
  lines.add "自举深度: " & $c.自举深度

  return lines.join("\n")

#--------------------------------------------------------------
#  文件系统扫描 — 从文件系统长出工具
#--------------------------------------------------------------

proc 文件系统长出*(c: var 自举编译器; 目录: string): string =
  var lines: seq[string] = @[]
  lines.add "─ 文件系统长出工具 ─"
  lines.add "扫描目录: " & 目录

  # 模拟从文件系统读取符号
  var cur_addr = 100
  for kind, path in walkDir(目录):
    if kind == pcFile and path.endsWith(".nim"):
      let 名字 = path.extractFilename()
      lines.add "  发现: " & 名字 & " → 地址[" & $cur_addr & "]"
      c.注册砖块(名字, cur_addr, "// 源码从文件读入\n#引用 " & $(cur_addr + 10))
      cur_addr += 10

  lines.add "共发现 " & $c.砖块表.len & " 个砖块"
  return lines.join("\n")

#--------------------------------------------------------------
#  报告
#--------------------------------------------------------------

proc 系统报告*(c: 自举编译器): string =
  var lines: seq[string] = @[]
  lines.add repeat("═", 50)
  lines.add "  自 举 编 译 器 系 统 报 告"
  lines.add repeat("═", 50)
  lines.add "砖块数: " & $c.砖块表.len
  lines.add "规则容器: " & $c.规则表.len
  lines.add "公路: " & $c.公路表.len
  lines.add "工具链: " & $c.工具链表.len
  lines.add "编译次数: " & $c.编译次数
  lines.add "自举深度: " & $c.自举深度
  lines.add ""

  lines.add "─ 砖块清单 ─"
  for 砖地址 in sorted(toSeq(c.砖块表.keys)):
    let b = c.砖块表[砖地址]
    let 极性符 = if b.极性 == 1: "阳" else: "阴"
    let 状态 = if b.激活: "✓" else: "○"
    let 阶段 = $c.编译阶段.getOrDefault(砖地址)
    lines.add "  " & 状态 & " [" & $砖地址 & "] " & b.名字 & " " & 极性符 & " 阶段:" & 阶段

  lines.add ""
  lines.add "─ 公路图 ─"
  for road in c.公路表:
    let 状态 = if road.通断: "通" else: "断"
    lines.add "  " & $road.起点 & " ─→ " & $road.终点 & " [距离:" & $road.距离 & ", " & 状态 & "] "

  return lines.join("\n")

#==============================================================
#  入口
#==============================================================

when isMainModule:
  var c = 初始化编译器("/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os")

  # 注册核心砖块
  c.注册砖块("容器编码", 10, "#容器基板\nconst 容器数 = 64\n#引用 20", 1)
  c.注册砖块("极性路由", 20, "#极性模块\nproc 路由() =\n  呼唤(10)\n#引用 30", 0)
  c.注册砖块("呼唤总线", 30, "#呼唤中枢\nproc 呼唤(addr: int) =\n  公路连通(addr)\n#引用 40", 1)
  c.注册砖块("八流调度", 40, "#八流统一\nsched()\n#引用 50", 0)
  c.注册砖块("字符串生成", 50, "#输出模块\noutput()", 1)

  # 注册规则容器
  var 规则集A = {"容器族": "8", "槽数": "8", "总槽": "64"}.toTable
  c.注册规则容器(10, 规则集A, "容器生成器")

  var 规则集B = {"阳锚": "1", "阴锚": "0", "间隙吸": "1", "间隙斥": "0"}.toTable
  c.注册规则容器(20, 规则集B, "极性规则器")

  # 修公路
  c.修公路(10, 20)
  c.修公路(20, 30)
  c.修公路(30, 40)
  c.修公路(40, 50)
  c.修公路(10, 50)  # 直通

  # 构建工具链
  c.构建工具链("编译链", @[10, 20, 30, 40, 50])

  # 执行
  echo c.自举()
  echo ""
  echo c.系统报告()
  echo ""
  echo c.执行工具链("编译链")
  echo ""

  # 寻路测试
  echo "─ 寻路测试 ─"
  let path = c.寻路(10, 50)
  if path.len > 0:
    echo "  10→50 路径: " & path.join(" → ")
  else:
    echo "  10→50 无路径"
