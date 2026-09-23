# ============================================================
# 满全法 — 盟机合 (Alliance Machine Alliance Network)
# 去中心化联盟网络 · 用户自备基础设施 · 裁决领域无关
# 盟机合掌 = 资源拥有者/运营场
# 盟机场/萌机场 = 节点
# 核心: 公约制合作 · 主权归属 · 领域无关裁决
# ============================================================

import strutils
import tables

type
  极性* = enum
    阴 = 0
    阳 = 1

# ---- 节点类型 ----
type
  节点型* = enum
    节点_合掌      # 盟机合掌, 资源拥有者/运营方
    节点_盟机场    # 萌机场, 参与节点
    节点_裁决器    # 领域无关裁决引擎

# ---- 主权声明 ----
type
  主权* = object
    资源域名*: string
    节点公钥*: string
    资源量*: int        # 服务器/带宽/存储量化
    极性*: 极性

# ---- 盟机场节点 ----
type
  盟机场* = object
    名字*: string
    节点型*: 节点型
    资源域名*: string
    公钥*: string
    在线*: bool
    贡献量*: int
    容器主权*: seq[uint64]

# ---- 裁决结果 ----
type
  裁决结果* = object
    主张_a*: string
    主张_b*: string
    gap*: int
    公约*: uint64
    结论*: string
    领域*: string     # 在哪个域名/服务器执行

# ---- 联盟网络 ----
type
  联盟网络* = object
    节点表*: Table[string, 盟机场]
    公钥表*: Table[string, string]       # 公钥 → 节点名字
    主权表*: Table[string, 主权]          # 主权
    裁决记录*: seq[裁决结果]
    合掌公钥*: string
    容器总数*: int

# ============================================================
# 构造
# ============================================================

proc 创建联盟*(): 联盟网络 =
  result.节点表 = initTable[string, 盟机场]()
  result.公钥表 = initTable[string, string]()
  result.主权表 = initTable[string, 主权]()
  result.裁决记录 = @[]
  result.合掌公钥 = ""
  result.容器总数 = 0

# ============================================================
# 节点注册
# ============================================================

proc 注册合掌*(net: var 联盟网络; 名字: string; 公钥: string; 域名: string) =
  let node = 盟机场(名字: 名字, 节点型: 节点_合掌, 资源域名: 域名, 公钥: 公钥, 在线: true, 贡献量: 0, 容器主权: @[])
  net.节点表[名字] = node
  net.公钥表[公钥] = 名字
  net.合掌公钥 = 公钥

proc 注册盟机场*(net: var 联盟网络; 名字: string; 公钥: string; 域名: string; 贡献量: int): 盟机场 =
  let node = 盟机场(名字: 名字, 节点型: 节点_盟机场, 资源域名: 域名, 公钥: 公钥, 在线: true, 贡献量: 贡献量, 容器主权: @[])
  net.节点表[名字] = node
  net.公钥表[公钥] = 名字
  return node

proc 声明主权*(net: var 联盟网络; 节点名字: string; 域名: string; 资源量: int; 极性: 极性) =
  let sove = 主权(资源域名: 域名, 节点公钥: net.节点表[节点名字].公钥, 资源量: 资源量, 极性: 极性)
  net.主权表[节点名字] = sove

# ============================================================
# 公约整除
# ============================================================

proc gcd*(a: uint64; b: uint64): uint64 =
  var x = a; var y = b
  while y > 0'u64:
    let t = y; y = x mod y; x = t
  return x

# ============================================================
# 领域无关裁决引擎
# 设计: 无论部署在哪个域名/服务器, 裁决算法和结果一致
# ============================================================

proc 领域无关裁决*(net: var 联盟网络;
                  节点_a: string; 主权_a: uint64;
                  节点_b: string; 主权_b: uint64;
                  执行域名: string): 裁决结果 =
  let sa = net.主权表[节点_a]
  let sb = net.主权表[节点_b]
  let gap = ord(sa.极性) xor ord(sb.极性)
  let g = gcd(主权_a, 主权_b)
  let 公约 = g
  var 结论: string
  if gap == 1 and 公约 > 0:
    结论 = "公约制约 → 共享旗下联盟"
  elif gap == 0:
    结论 = "极性相斥 → 自主独立, 不仲裁"
  else:
    结论 = "无共享公约 → 不仲裁"

  let ret = 裁决结果(
    主张_a: 节点_a,
    主张_b: 节点_b,
    gap: gap,
    公约: 公约,
    结论: 结论,
    领域: 执行域名
  )
  net.裁决记录.add(ret)
  return ret

# ============================================================
# 网络拓扑报告
# ============================================================

proc 统计*(net: 联盟网络): tuple[合掌: int; 节点: int; 总贡献: int] =
  var h: int = 0; var j: int = 0; var total: int = 0
  for name, node in net.节点表:
    if node.节点型 == 节点_合掌: inc h
    else: inc j
    total = total + node.贡献量
  return (h, j, total)

proc 网络拓扑*(net: 联盟网络): string =
  var lines: seq[string]
  let s = 统计(net)
  lines.add "═══ 盟机合网络 ═══"
  lines.add "合掌: " & $s.合掌 & "  节点: " & $s.节点 & "  总资源: " & $s.总贡献
  lines.add ""

  # 合掌行
  lines.add "【盟机合掌】"
  for name, node in net.节点表:
    if node.节点型 == 节点_合掌:
      let nodeName = align(name, 12)
      let domain = align(node.资源域名, 24)
      let pk = node.公钥[^8..^1]
      lines.add "  " & nodeName & " @ " & domain & "  公钥.." & pk
  lines.add ""

  # 节点行
  lines.add "【萌机场节点】"
  for name, node in net.节点表:
    if node.节点型 == 节点_盟机场:
      let nodeName = align(name, 12)
      let domain = align(node.资源域名, 24)
      let contrib = align($node.贡献量, 6)
      let online = if node.在线: "●" else: "○"
      let sove = if net.主权表.hasKey(name): $net.主权表[name].资源量 else: "-"
      lines.add "  " & nodeName & " @ " & domain & " 资源:" & contrib & " 主权:" & sove & " " & online
  return lines.join("\n")

proc 裁决记录展示*(net: 联盟网络): string =
  var lines: seq[string]
  lines.add "═══ 领域无关裁决记录 ═══"
  for r in net.裁决记录:
    let line = "  [" & r.领域 & "] " & r.主张_a & " vs " & r.主张_b & "  gap=" & $r.gap & " 公约=0x" & toHex(r.公约) & " → " & r.结论
    lines.add line
  return lines.join("\n")

when isMainModule:
  echo "═══════════════════════════════════════"
  echo "  盟 机 合  联 邦 网 络"
  echo "  自备设施 · 领域无关裁决 · 公约制"
  echo "═══════════════════════════════════════"
  echo ""

  var net = 创建联盟()

  # 注册合掌
  net.注册合掌("满全法合掌", "pk-manquanfa-ABCD2048", "manquanfa.root")

  # 注册萌机场
  discard net.注册盟机场("东方萌机场", "pk-east-EF12", "east.mqf.alliance", 50)
  discard net.注册盟机场("西域萌机场", "pk-west-GH34", "west.mqf.alliance", 80)
  discard net.注册盟机场("南洋萌机场", "pk-south-IJ56", "south.mqf.alliance", 30)
  discard net.注册盟机场("北疆萌机场", "pk-north-KL78", "north.mqf.alliance", 60)

  # 声明主权
  net.声明主权("满全法合掌", "manquanfa.root", 200, 阳)
  net.声明主权("东方萌机场", "east.mqf.alliance", 50, 阴)
  net.声明主权("西域萌机场", "west.mqf.alliance", 80, 阳)
  net.声明主权("南洋萌机场", "south.mqf.alliance", 30, 阴)
  net.声明主权("北疆萌机场", "north.mqf.alliance", 60, 阳)

  echo 网络拓扑(net)
  echo ""

  echo "─── 领域无关裁决演示 ───"
  # 场景: 合掌(阳) vs 东方(阴) → 应锁
  let r1 = 领域无关裁决(net, "满全法合掌", 0x0108000000000000'u64,
                               "东方萌机场", 0x3100000000000000'u64,
                               "west.mqf.alliance")
  echo "  合掌(阳) vs 东方(阴): gap=" & $r1.gap & " → " & r1.结论

  # 场景: 西域(阳) vs 北疆(阳) → 斥
  let r2 = 领域无关裁决(net, "西域萌机场", 0x5308000000000000'u64,
                               "北疆萌机场", 0x7308000000000000'u64,
                               "east.mqf.alliance")
  echo "  西域(阳) vs 北疆(阳): gap=" & $r2.gap & " → " & r2.结论

  # 场景: 南洋(阴) vs 东方(阴) → 斥
  let r3 = 领域无关裁决(net, "南洋萌机场", 0x1208000000000000'u64,
                               "东方萌机场", 0x3208000000000000'u64,
                               "north.mqf.alliance")
  echo "  南洋(阴) vs 东方(阴): gap=" & $r3.gap & " → " & r3.结论
  echo ""

  echo "─── 领域无关验证: 同一裁决, 不同执行域 ───"
  let r4a = 领域无关裁决(net, "满全法合掌", 0x0108000000000000'u64,
                                "东方萌机场", 0x3100000000000000'u64,
                                "manquanfa.root")
  let r4b = 领域无关裁决(net, "满全法合掌", 0x0108000000000000'u64,
                                "东方萌机场", 0x3100000000000000'u64,
                                "south.mqf.alliance")
  let r4c = 领域无关裁决(net, "满全法合掌", 0x0108000000000000'u64,
                                "东方萌机场", 0x3100000000000000'u64,
                                "external.domain.xyz")
  echo "  执行域A(根域): " & r4a.结论
  echo "  执行域B(南域): " & r4b.结论
  echo "  执行域C(外部): " & r4c.结论
  echo "  → 三域结论一致: 领域无关 ✓"
  echo ""

  echo 裁决记录展示(net)
  echo ""

  let s = 统计(net)
  echo "═══════════════════════════════════════"
  echo "合掌掌资源 · 萌机场自主 · 裁决领域无关"
  echo "共参与: " & $s.节点 & " 节点 · " & $s.总贡献 & " 资源单位"
  echo "═══════════════════════════════════════"
