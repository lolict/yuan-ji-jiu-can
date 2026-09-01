# ============================================================
# 范畴编程管辖索引制 · 引擎 (cbsy)
# 核心公理（用户定义，不解释不翻译）：
#   索引 = 命令。三维全息进制空间位置索引 = 呼唤编程的命令。
#   不需要命令行。
#   谁拥有那个权限，就拥有命令的条件。
#   命令发布三级：信用级（信用完成发布）· 金钱级（用钱买命令）·
#                 权限级（直接命令 = 政权/军权）
#   夫妻主权人（我=1，刘楚恬=0）在军权/政权/有钱人之外，
#   直接主权，不与普通人共享权限。
#   普通人受世俗规则约束 = 受主权人的规则约束。
# 运算约束：袋子级 · 仅加减 · 三元见证
# ============================================================

import std/strutils

type
  # 三维全息进制空间位置索引：x=声母族 y=韵母族 z=声调族，各 0..3 → 64 格
  Index3D* = tuple[x, y, z: int]

  # 命令发布者层级
  IssuerKind* = enum
    iSovereign   # 夫妻主权人 — 权限之上
    iAuthority   # 权限级 — 政权/军权，直接命令
    iMoney       # 金钱级 — 买的命令
    iCredit      # 信用级 — 信用发布
    iCommon      # 普通人 — 受规则约束

  # 命令 = 索引格子本身（不需要命令行字符串解析）
  CbsyCommand* = object
    idx*: Index3D
    issuer*: IssuerKind
    credit*: int     # 信用存量
    money*: int      # 金钱存量
    authority*: int  # 权限存量

  # 裁决
  Verdict* = enum
    vPass, vDenied, vWitness

# ---------- 索引 → 命令（格子即命令） ----------

const
  VERBS*   = ["照", "罩", "召", "见"]   # x 轴：操作族
  TARGETS* = ["袋", "桶", "缸", "盆"]   # y 轴：容器族
  MODES*   = ["开", "关", "通", "断"]   # z 轴：声调（命令方式）

proc cell_code*(idx: Index3D): int =
  ## 64 格编址：仅加减合成
  result = 0
  var i = 0
  while i < idx.x:
    result = result + 16
    inc i
  i = 0
  while i < idx.y:
    result = result + 4
    inc i
  result = result + idx.z

proc command_at*(idx: Index3D): string =
  ## 索引呼唤：给出位置即给出命令，无命令行
  result = VERBS[idx.x] & TARGETS[idx.y] & MODES[idx.z]

# ---------- 命令价格（仅加减） ----------

proc price*(cmd: CbsyCommand): int =
  ## 基价 3 + 声母序 + 韵母序 + 声调序（仅加减）
  result = 3
  var i = 0
  while i < cmd.idx.x: result = result + 1; inc i
  i = 0
  while i < cmd.idx.y: result = result + 1; inc i
  i = 0
  while i < cmd.idx.z: result = result + 1; inc i

# ---------- 条件裁判（谁拥有权限，谁拥有命令的条件） ----------

proc judge*(cmd: CbsyCommand): tuple[v: Verdict, why: string] =
  let p = price(cmd)
  case cmd.issuer
  of iSovereign:
    result = (vPass, "夫妻主权人：军权/政权/金钱之外，直接命令，不与普通人共享")
  of iAuthority:
    if cmd.authority > 0:
      result = (vPass, "权限级：直接命令（政权/军权持有）")
    else:
      result = (vDenied, "无权限存量 → 无命令条件")
  of iMoney:
    if cmd.money >= p:
      result = (vPass, "金钱级：命令已购买（价" & $p & "）")
    else:
      result = (vDenied, "金钱不足（需" & $p & "）→ 无命令条件")
  of iCredit:
    if cmd.credit >= p:
      result = (vWitness, "信用级：命令已发布，待三元见证（价" & $p & "）")
    else:
      result = (vDenied, "信用不足（需" & $p & "）→ 无命令条件")
  of iCommon:
    result = (vDenied, "普通人受世俗规则约束 = 受主权人规则约束，无命令条件")

# ---------- 三元见证（信用级命令的闭环） ----------

proc witness*(cmd: CbsyCommand): tuple[v: Verdict, why: string] =
  ## 非空 + 已见证 + 锚点一致
  let j = judge(cmd)
  if j.v != vWitness:
    return (j.v, j.why)
  let code = cell_code(cmd.idx)
  if code >= 0 and code < 64:
    result = (vPass, "三元见证通过：格" & $code & " 非空 · 信用已核 · 锚点在主权规则内")
  else:
    result = (vDenied, "见证失败：格子越界（不属于 64 全息空间）")

# ---------- 演示 ----------

when isMainModule:
  echo "范畴编程管辖索引制 · 索引即命令"
  echo "============================================================"

  let idx: Index3D = (0, 0, 0)   # 格0 = 照·袋·开

  var cmds = @[
    CbsyCommand(idx: idx, issuer: iSovereign, credit: 0, money: 0, authority: 0),
    CbsyCommand(idx: idx, issuer: iAuthority, credit: 0, money: 0, authority: 1),
    CbsyCommand(idx: idx, issuer: iMoney,     credit: 0, money: 99, authority: 0),
    CbsyCommand(idx: idx, issuer: iCredit,    credit: 99, money: 0, authority: 0),
    CbsyCommand(idx: idx, issuer: iCredit,    credit: 1, money: 0, authority: 0),
    CbsyCommand(idx: idx, issuer: iCommon,    credit: 0, money: 0, authority: 0),
  ]

  for c in cmds:
    let j = judge(c)
    var final = j
    if j.v == vWitness:
      final = witness(c)
    echo "[", command_at(c.idx), "] Issuer=", $c.issuer,
        " → ", $final.v, " · ", final.why

  echo ""
  echo "索引呼唤表（64格抽样）："
  for x in 0 ..< 2:
    for y in 0 ..< 2:
      for z in 0 ..< 2:
        let i3: Index3D = (x, y, z)
        echo "  (", x, ",", y, ",", z, ") 格", cell_code(i3), " = ",
            command_at(i3)
