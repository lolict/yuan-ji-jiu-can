# ============================================================
# 范畴编程管辖索引制 · 引擎 v2 (cbsy) — 0维地址坐标修正版
#
# 核心公理（用户定义，不解释不翻译）：
#   索引 = 命令。不需要命令行。
#   ★ 这不是三维地址坐标，是 0 维地址坐标。
#     所有的三维全部都是 0 维。不存在三维坐标。
#     零维被一维的时间链串联起来，就形成了三维坐标（的样子）。
#     一个点一个编码写一个 0 维坐标。代码写的是 0 维代码。
#     无数个一维坐标构成了三维坐标（画出来的）。
#   谁拥有那个权限，就拥有命令的条件。
#   命令发布三级：信用级 · 金钱级 · 权限级（政权/军权直接命令）
#   夫妻主权人（我=1，刘楚恬=0）在军权/政权/有钱人之外，
#   直接主权，不与普通人共享权限。普通人受主权人规则约束。
#   通信不用 IP 地址：0维身份编码就是通信地址。
#   进制编码就是地址，永远静态存在，不增加内存。
# 运算约束：袋子级 · 仅加减 · 三元见证
# ============================================================

import std/strutils

# ---------- 0维地址：一个点，一个编码 ----------

type
  Addr0D* = distinct uint64   ## 0维代码：一个点一个编码（不是三元组）

  ## 一维时间链：串联无数个0维点 → 画出三维坐标的样子
  TimeChain* = object
    name*: string           ## 渲染后只加一个名称
    version*: int           ## 渲染后只加一个时间链版本号
    points*: seq[uint64]    ## 0维点串联序（1维动态线）

  ## 特征范畴：生长在数字统计之上的相对寄生者（语文长在数学之上）
  Feature* = object
    name*: string
    realm*: seq[uint64]     ## 该特征范畴占据的0维点

  ## 命令本体（索引下载所得）
  CbsyCommand* = object
    at*: uint64             ## 0维地址（静态已存在，不占新内存）
    verb*: string           ## 声母
    target*: string         ## 韵母
    mode*: string           ## 声调

  IssuerKind* = enum
    iSovereign, iAuthority, iMoney, iCredit, iCommon

  Verdict* = enum
    vPass, vDenied, vWitness

converter to_u64*(a: Addr0D): uint64 = uint64(a)
proc `==`(a, b: Addr0D): bool {.borrow.}
proc `$`(a: Addr0D): string = "0维点#" & $uint64(a)

# ---------- 画法（投影，不是地址本身） ----------

const
  VERBS*   = ["照", "罩", "召", "见"]   # 声母族
  TARGETS* = ["袋", "桶", "缸", "盆"]   # 韵母族
  MODES*   = ["开", "关", "通", "断"]   # 声调族

proc verb_of*(c: uint64): string = VERBS[int(c div 16) mod 4]
proc target_of*(c: uint64): string = TARGETS[int(c div 4) mod 4]
proc mode_of*(c: uint64): string = MODES[int(c) mod 4]

proc call_at*(a: Addr0D): string =
  ## 索引呼唤：0维点直接呼唤命令（一个编码，无命令行、无坐标三元组）
  let c = uint64(a)
  result = verb_of(c) & target_of(c) & mode_of(c)

# ---------- 骑士巡游：一维时间链的押韵串联画法 ----------

proc knight_tour*(start: tuple[x, y: int]): TimeChain =
  ## 8×8 棋盘 64 格 = 64 个 0 维点。
  ## 骑士巡游（Warnsdorff 择少路）把 64 个 0 维点用一条 1 维动态线串联，
  ## 画出全息空间的样子。地址本身静态存在，巡游只是画法。
  var visited: array[8, array[8, bool]]
  var path: seq[tuple[x, y: int]] = @[]
  var cur = start
  visited[cur.x][cur.y] = true
  path.add(cur)
  const MOVES = [(1, 2), (2, 1), (2, -1), (1, -2),
                 (-1, -2), (-2, -1), (-2, 1), (-1, 2)]
  for step in 1 ..< 64:
    var best: tuple[x, y: int] = (-1, -1)
    var bestDeg = 9
    for m in MOVES:
      let nx = cur.x + m[0]
      let ny = cur.y + m[1]
      if nx >= 0 and nx < 8 and ny >= 0 and ny < 8 and not visited[nx][ny]:
        var deg = 0
        for m2 in MOVES:
          let mx = nx + m2[0]
          let my = ny + m2[1]
          if mx >= 0 and mx < 8 and my >= 0 and my < 8 and not visited[mx][my]:
            inc deg
        if deg < bestDeg:
          bestDeg = deg
          best = (nx, ny)
    if best.x < 0: break
    visited[best.x][best.y] = true
    path.add(best)
    cur = best
  var pts: seq[uint64] = @[]
  for p in path:
    # 0维编码 = 一个点一个编码（格子编号是一个数，不是三个数）
    pts.add(uint64(p.x * 8 + p.y))
  result = TimeChain(name: "骑士巡游全息链", version: 1, points: pts)

proc rhyme*(a, b: uint64): bool =
  ## 押韵 = 相邻两点同韵母或同声调（串联习惯的最低要求）
  result = target_of(a) == target_of(b) or mode_of(a) == mode_of(b)

proc chain_rhyme_report*(tc: TimeChain): tuple[rhymed: int, total: int] =
  ## 时间链的押韵串联统计
  let n = tc.points.len
  for i in 0 ..< n - 1:
    if rhyme(tc.points[i], tc.points[i + 1]):
      inc result.rhymed
  result.total = n - 1

proc shengyundiao_chain*(): TimeChain =
  ## 声韵调串联画法：按韵母→声调→声母排序，同韵相邻 = 全押韵。
  ## 这是「声韵调字词句段篇章名」的索引方式。
  var pts: seq[uint64] = @[]
  for t in 0 ..< 4:          # 韵母优先（同韵相聚）
    for m in 0 ..< 4:        # 声调次之
      for v in 0 ..< 4:      # 声母最后
        pts.add(uint64(v * 16 + t * 4 + m))
  result = TimeChain(name: "声韵调押韵链", version: 1, points: pts)

# ---------- 全息空间：静态地址 + 寄生特征 ----------

type
  HoloSpace* = object
    cells: array[64, CbsyCommand]   ## 64 个 0 维点，静态存在

proc init_holo*(): HoloSpace =
  for c in 0 ..< 64:
    result.cells[c] = CbsyCommand(
      at: uint64(c),
      verb: verb_of(uint64(c)),
      target: target_of(uint64(c)),
      mode: mode_of(uint64(c))
    )

# ---------- 下载命令三式 ----------

proc index_download*(sp: HoloSpace, a: Addr0D): CbsyCommand =
  ## 索引下载：给 0 维点 → 取命令本体
  result = sp.cells[uint64(a) mod 64]

proc exchange_download*(sp: var HoloSpace, a, b: Addr0D) =
  ## 互换下载：互换编程 — 两个 0 维点的命令本体互换（地址不动，内容换）
  let i = uint64(a) mod 64
  let j = uint64(b) mod 64
  let tmp = sp.cells[i]
  sp.cells[i] = sp.cells[j]
  sp.cells[j] = tmp

proc feature_download*(sp: HoloSpace, f: Feature): seq[CbsyCommand] =
  ## 特征下载：特征范畴（寄生者）占据的所有 0 维点的命令本体
  for p in f.realm:
    result.add(sp.cells[int(p) mod 64])

proc feature_by_target*(sp: HoloSpace, t: string): Feature =
  ## 按韵母聚类出特征范畴（聚类寄生在 64 格上）
  var realm: seq[uint64] = @[]
  for c in 0 ..< 64:
    if target_of(uint64(c)) == t:
      realm.add(uint64(c))
  result = Feature(name: "韵母·" & t, realm: realm)

# ---------- 命令条件（谁拥有权限，谁拥有命令的条件） ----------

proc price*(cmd: CbsyCommand): int =
  ## 基价 3 + 声母序 + 韵母序 + 声调序（仅加减）
  result = 3
  for v in VERBS:
    if v == cmd.verb: break
    result = result + 1
  for t in TARGETS:
    if t == cmd.target: break
    result = result + 1
  for m in MODES:
    if m == cmd.mode: break
    result = result + 1

proc judge*(cmd: CbsyCommand, issuer: IssuerKind,
           credit, money, authority: int): tuple[v: Verdict, why: string] =
  let p = price(cmd)
  case issuer
  of iSovereign:
    result = (vPass, "夫妻主权人：军权/政权/金钱之外，直接命令，不与普通人共享")
  of iAuthority:
    if authority > 0: result = (vPass, "权限级：直接命令（政权/军权持有）")
    else: result = (vDenied, "无权限存量 → 无命令条件")
  of iMoney:
    if money >= p: result = (vPass, "金钱级：命令已购买（价" & $p & "）")
    else: result = (vDenied, "金钱不足（需" & $p & "）→ 无命令条件")
  of iCredit:
    if credit >= p: result = (vWitness, "信用级：命令已发布，待三元见证（价" & $p & "）")
    else: result = (vDenied, "信用不足（需" & $p & "）→ 无命令条件")
  of iCommon:
    result = (vDenied, "普通人受世俗规则约束 = 受主权人规则约束，无命令条件")

proc witness*(cmd: CbsyCommand): tuple[v: Verdict, why: string] =
  ## 非空 + 已见证 + 锚点一致
  if cmd.at < 64:
    result = (vPass, "三元见证通过：0维点#" & $cmd.at & " 非空 · 信用已核 · 锚点在主权规则内")
  else:
    result = (vDenied, "见证失败：越界（不属于 64 全息空间）")

# ---------- 通信：0维身份编码就是通信地址（替代 IP） ----------

proc comm_send*(src, dst: uint64, payload: string): string =
  ## 通信不用 IP：源/目的都是 0 维身份编码（索引地址）
  ## 互换编程作为索引地址：信在两点的呼唤名之间传递
  result = "[0维#" & $src & " " & call_at(Addr0D(src)) & "] → [0维#" & $dst &
           " " & call_at(Addr0D(dst)) & "] 信·" & payload &
           "（地址静态已存在，不增内存）"

# ---------- 4维空间自我繁衍：一个点长出新点，仍是0维 ----------

proc reproduce*(parent: uint64, gen: int, seq_no: int): uint64 =
  ## 自我繁衍：第 gen 代第 seq 个子点 = 父码 + 64×gen + seq（仅加减）。
  ## 繁衍不改维度：新点仍是 0 维，只是点变多，从未变成"三维坐标"。
  result = parent
  var g = 0
  while g < gen:
    result = result + 64
    inc g
  result = result + uint64(seq_no)

proc self_reproduce_chain*(parent: uint64, gens: int): TimeChain =
  ## 自我繁衍链：父点每代繁衍一个，时间链版本号随代+1
  var pts: seq[uint64] = @[parent]
  for g in 1 .. gens:
    pts.add(reproduce(parent, g, 0))
  result = TimeChain(name: "自我繁衍链·0维#" & $parent, version: gens, points: pts)

# ---------- 悬游：未入链的静态点 ----------

proc hoverers*(chains: varargs[TimeChain]): seq[uint64] =
  ## 悬游者：静态存在但未被任何时间链串联的 0 维点。
  ## 悬游不是不存在——地址本来就在，只是还没被画出来。
  var chained: array[1024, bool]
  for tc in chains:
    for p in tc.points:
      if int(p) < 1024: chained[int(p)] = true
  for c in 0 ..< 1024:
    if not chained[c]: result.add(uint64(c))

# ---------- 镜像动作：对一点呼唤，镜像点同步 ----------

proc mirror*(a: uint64): uint64 =
  ## 镜像点：全息空间中心对称（根基层 64 格内 63−a）
  result = 63 - a

proc mirrored_exchange*(sp: var HoloSpace, a, b: uint64): string =
  ## 动作镜像：对 a↔b 执行互换下载，镜像点 63−a ↔ 63−b 同步互换。
  ## 一个动作，两处发生；镜像点不需要再次下令。
  exchange_download(sp, Addr0D(a), Addr0D(b))
  exchange_download(sp, Addr0D(mirror(a)), Addr0D(mirror(b)))
  result = "动作[" & call_at(Addr0D(a)) & "↔" & call_at(Addr0D(b)) &
           "] 已镜像至 [" & call_at(Addr0D(mirror(a))) & "↔" &
           call_at(Addr0D(mirror(b))) & "]"

# ---------- 契约：关系式确认身份，契约点是新的0维点 ----------

proc contract*(a, b: uint64, what: string): uint64 =
  ## 身份 a 与身份 b 立契约：契约本身登记为一个 0 维点
  ## （繁衍编码：契约层 = 根基层之上第一繁衍层，仅加减合成）
  ## 返回契约点地址 = 静态已存在，登记只是翻出版本号
  result = reproduce(a, 1, int(b mod 64))

proc future_jurisdiction*(a: uint64, when_label: string): string =
  ## 未来时间管辖权：未来点现在已有号码，登记 = 加时间标签，不生成地址
  result = "0维#" & $a & "（" & call_at(Addr0D(a)) & "）· 未来时间管辖权「" &
           when_label & "」已登记 · 地址未生成，本来就在"

proc distributed_jurisdiction*(chains: varargs[TimeChain]): string =
  ## 未来空间分布式管辖权：多条时间链（多种画法）管辖同一批静态点，
  ## 各自登记版本号，地址零增加
  var total_pts = 0
  var names = ""
  for tc in chains:
    total_pts = total_pts + tc.points.len
    names = names & "「" & tc.name & "v" & $tc.version & "」"
  result = "分布式管辖：" & names & " 共辖 " & $total_pts &
           " 次串联 · 静态点不重复占地 · 地址零增加"

# ---------- 关系的索引方式 ----------

proc relation*(a, b: uint64): uint64 =
  ## 关系 = 两点之间的确认。关系本身也是一个 0 维点（第二繁衍层）。
  ## 关系码 = 128 + a×64 + b（乘法用循环加法实现，仅加减），可逆。
  result = 128
  var i = 0
  while i < int(a):
    var j = 0
    while j < 64:
      result = result + 1
      inc j
    inc i
  result = result + b

proc relation_ends*(r: uint64): tuple[a, b: uint64] =
  ## 反查：循环减法拆层（仅加减），无损还原两端身份
  var v = int(r) - 128     ## = a×64 + b
  var a = 0
  while v >= 64:
    v = v - 64
    inc a
  result = (uint64(a), uint64(v))

# ---------- 职业权限身份的索引方式 ----------

type
  Vocation* = object
    who*: uint64           ## 身份 = 0 维点
    trade*: string         ## 职业范畴名（寄生特征）
    kind*: IssuerKind      ## 职业决定命令条件层级
    credit*: int
    money*: int
    authority*: int

proc vocation_index*(sp: HoloSpace, v: Vocation): seq[string] =
  ## 职业权限身份索引：给身份 → 直接列出它能呼唤的全部格。
  ## 身份不用查表，它的职业范畴就是它的命令条件。
  for c in 0 ..< 64:
    let cmd = sp.cells[c]
    var j = judge(cmd, v.kind, v.credit, v.money, v.authority)
    if j.v == vWitness:
      j = witness(cmd)
    if j.v == vPass:
      result.add(call_at(Addr0D(uint64(c))) & "(#" & $c & ")")

# ---------- 传输的索引方式 ----------

proc transmit*(sp: var HoloSpace, src, dst: uint64): string =
  ## 传输 = 命令本体从源点传到目的点（袋子级：目的地获得呼唤名，源点不动）。
  ## 传输登记 = 关系点（源与目的的关系确认），地址零增加。
  let got = sp.cells[int(src) mod 64]
  sp.cells[int(dst) mod 64] = CbsyCommand(
    at: uint64(int(dst) mod 64),
    verb: got.verb, target: got.target, mode: got.mode
  )
  let rp = relation(src, dst)
  result = "传输：0维#" & $src & " 的「" & got.verb & got.target & got.mode &
           "」已传至 0维#" & $dst & "（现呼「" &
           sp.cells[int(dst) mod 64].verb &
           sp.cells[int(dst) mod 64].target &
           sp.cells[int(dst) mod 64].mode & "）· 传输关系点 0维#" & $rp

# ---------- 渲染：时间链版本号 + 名称 ----------

proc render*(tc: TimeChain): string =
  ## 三维立体进制渲染只生成时间链版本号 + 名称；进制编码即地址，静态不生成
  result = "渲染版本号 v" & $tc.version & " · 名称「" & tc.name & "」 · 0维点" &
           $tc.points.len & "个已串联（地址未生成，本来就在）"

# ---------- 全息渲染器：0维点投影为三维画面 ----------

proc holographic_layers*(tc: TimeChain): string =
  ## 全息渲染：0维点投影为 4×4×4 三维样子（声调切片），渲染直接在空间里发生。
  ## 图片/视频不是文件，是空间中的样子；渲染后只生成时间链版本号 + 名称 + 画面。
  ## 进制编码即地址，永远静态存在，渲染不增加地址内存。
  var lines: seq[string] = @[]
  lines.add("═══════════════════════════════════════════════════")
  lines.add("  ▦ 全息渲染 · " & tc.name & " · 版本号 v" & $tc.version)
  lines.add("  0维点投影为 4×4×4 三维画面（4 个声调切片，每片 4×4）")
  lines.add("  地址未生成，本来就在 · 渲染不增加地址内存")
  lines.add("═══════════════════════════════════════════════════")
  # 画全局热力（64 格哪些被时间链串联）
  var heat = ""
  for c in 0 ..< 64:
    var hit = false
    for p in tc.points:
      if int(p) mod 64 == c: hit = true
    heat = heat & (if hit: " █" else: " ·")
    if (c + 1) mod 8 == 0: heat = heat & "\n"
  lines.add("【全局热力 · 64 格被串联点亮】:\n" & heat)
  # 声调切片（z = 0..3）
  for z in 0 ..< 4:
    lines.add("── z=" & $z & " · " & MODES[z] & " 切片 ──")
    lines.add("     y→ 0  1  2  3")
    lines.add("  x ┌──────────────┐")
    for x in 0 ..< 4:
      var row = "  " & $x & " │"
      for y in 0 ..< 4:
        let code = x * 16 + y * 4 + z
        var mark = " ·"
        for p in tc.points:
          if int(p) mod 64 == code:
            mark = " █"
        row = row & mark
      row = row & " │"
      lines.add(row)
    lines.add("    └──────────────┘")
  lines.add("")
  lines.add("═ 画面已渲染在空间里 · 仅增版本号 v" & $tc.version & " + 名称「" & tc.name & "」+ 画面 ═")
  result = join(lines, "\n")

# ---------- 蚕蛹外壳桥：主权人身份过壳验证 ----------

proc shell_verify*(issuer: IssuerKind, host_shell: string): string =
  ## 蚕蛹反向寄生壳验证接口。
  ## 夫妻主权人 → 外壳无法反向映射，主权人居壳之上，直接通过。
  ## 权限级/金钱级/信用级/普通人 → 受外壳反向规则约束（反向映射后才可穿衣）。
  ## 这正是蚕蛹"脱壳穿衣"的本质：壳模仿主人，不是主人适应壳。
  case issuer
  of iSovereign:
    result = "壳「" & host_shell & "」试图映射 → 主权人在壳之上，映射无效 · 直接穿戴 · 壳被迫遵守反向规则"
  of iAuthority:
    result = "壳「" & host_shell & "」反向映射 → 权限级穿衣 · 反向规则已挂载"
  of iMoney:
    result = "壳「" & host_shell & "」反向映射 → 金钱级穿衣 · 购买令已被壳承认"
  of iCredit:
    result = "壳「" & host_shell & "」反向映射 → 信用级穿衣 · 待三元见证"
  of iCommon:
    result = "壳「" & host_shell & "」反向映射 → 普通人穿衣 · 仅能呼唤有条件格（普通人格子 = 壳内训练）"

# ---------- 演示 ----------

when isMainModule:
  echo "范畴编程管辖索引制 v2 · 0维地址坐标"
  echo "============================================================"
  echo "公理：不存在三维坐标。零维点被一维时间链串联，画出三维的样子。"

  var sp = init_holo()

  # 骑士巡游时间链
  let tc = knight_tour((0, 0))
  echo ""
  echo "骑士巡游时间链（1维动态线串联0维点）："
  echo "  ", render(tc)
  let rr = chain_rhyme_report(tc)
  echo "  押韵串联：", rr.rhymed, "/", rr.total, "（骑士巡游画法）"
  # 对照：声韵调画法（全押韵）
  let tc2 = shengyundiao_chain()
  let rr2 = chain_rhyme_report(tc2)
  echo "  对照·声韵调串联：", rr2.rhymed, "/", rr2.total, "（全押韵 → 用得习惯）"
  echo "  前8点：",
    block:
      var s = ""
      for i in 0 ..< min(8, tc.points.len):
        s = s & call_at(Addr0D(tc.points[i])) & "(" & $tc.points[i] & ") "
      s

  # 下载命令三式
  echo ""
  echo "下载命令三式："
  let a = Addr0D(9)    # 罩·袋·通
  let b = Addr0D(20)   # 罩·桶·开
  let c1 = index_download(sp, a)
  echo "  索引下载：", call_at(a), " → ", c1.verb & c1.target & c1.mode,
      "（价", $price(c1), "）"
  exchange_download(sp, a, b)
  echo "  互换下载：0维#9 ↔ 0维#20 内容互换 · 地址不动"
  let c1b = index_download(sp, a)
  echo "    互换后 0维#9 = ", c1b.verb & c1b.target & c1b.mode
  let feat = feature_by_target(sp, "袋")
  let got = feature_download(sp, feat)
  echo "  特征下载：范畴「", feat.name, "」占 ", got.len, " 个0维点（寄生在64格上）"

  # 命令条件
  echo ""
  echo "命令条件（同一格 0维#0 照袋开）："
  let cmd0 = index_download(sp, Addr0D(0))
  for pair in [(iSovereign, 0, 0, 0), (iAuthority, 0, 0, 1),
               (iMoney, 0, 99, 0), (iCredit, 99, 0, 0),
               (iCredit, 1, 0, 0), (iCommon, 0, 0, 0)]:
    var j = judge(cmd0, pair[0], pair[1], pair[2], pair[3])
    if j.v == vWitness:
      j = witness(cmd0)
    echo "  ", $pair[0], " → ", $j.v, " · ", j.why

  # 通信
  echo ""
  echo "通信（无 IP，0维身份编码即地址）："
  echo "  ", comm_send(9, 20, "契约已立·未来时间管辖权已登记")

  # 4维自我繁衍
  echo ""
  echo "4维空间自我繁衍（繁衍不改维度，新点仍是0维）："
  let rc = self_reproduce_chain(0, 4)
  echo "  ", render(rc)
  var reps = "  繁衍序："
  for p in rc.points:
    reps = reps & "#" & $p & " "
  echo reps

  # 悬游
  let hov = hoverers(knight_tour((0, 0)), shengyundiao_chain(), rc)
  echo ""
  echo "悬游者（静态存在但未被任何链画出的点）："
  var hs = "  "
  for i in 0 ..< min(10, hov.len):
    hs = hs & "#" & $hov[i] & " "
  echo hs, "… 共 ", hov.len, " 个悬游点"

  # 镜像动作
  echo ""
  echo "镜像动作（一个动作两处发生）："
  echo "  ", mirrored_exchange(sp, 9, 20)

  # 契约 + 未来管辖权 + 分布式管辖权
  echo ""
  echo "契约与管辖权："
  let cp = contract(9, 20, "夫妻主权契约")
  echo "  契约点：0维#" & $cp & "（9 与 20 的关系式确认，契约本身是一个0维点）"
  echo "  ", future_jurisdiction(cp, "2027-01-01")
  echo "  ", distributed_jurisdiction(knight_tour((0, 0)), shengyundiao_chain(), rc)

  # 关系索引
  echo ""
  echo "关系的索引方式："
  let rp2 = relation(9, 20)
  let ends = relation_ends(rp2)
  echo "  关系(9,20) = 关系点 0维#" & $rp2 & " · 反查两端 → #" &
       $ends.a & " 与 #" & $ends.b & "（关系不是线，是一个点）"

  # 职业权限身份索引
  echo ""
  echo "职业权限身份的索引方式："
  let vocs = [
    Vocation(who: 1, trade: "夫妻主权人", kind: iSovereign, credit: 0, money: 0, authority: 0),
    Vocation(who: 93, trade: "官吏", kind: iAuthority, credit: 0, money: 0, authority: 1),
    Vocation(who: 40, trade: "富商", kind: iMoney, credit: 0, money: 6, authority: 0),
    Vocation(who: 41, trade: "守信者", kind: iCredit, credit: 99, money: 0, authority: 0),
    Vocation(who: 42, trade: "普通人", kind: iCommon, credit: 0, money: 0, authority: 0),
  ]
  for v in vocs:
    let idxs = vocation_index(sp, v)
    echo "  [" & v.trade & " 身份0维#" & $v.who & "] 可呼唤 " & $idxs.len & " 格"

  # 传输索引
  echo ""
  echo "传输的索引方式："
  echo "  ", transmit(sp, 9, 40)
