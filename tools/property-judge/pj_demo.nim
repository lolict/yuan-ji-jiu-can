# ============================================================
# 产权流侵权判定树 · 完整演示
# 演示判定树的 9 种路径全覆盖
# ============================================================

import strutils
import tables
import property_judge/types
import property_judge/tree

echo "═══════════════════════════════════════════"
echo "  产权流侵权判定树 · 完整演示"
echo "═══════════════════════════════════════════"
echo ""

# ── 构建容器注册表 ──
var judge = new_judge()

# 容器 A: 我的私有容器
judge.register(ContainerInfo(
  id: "c-a",
  state: PropertyState(kind: pskPrivate, owner_id: "lolict"),
  metadata: init_table[string, string]()
))

# 容器 B: 夫妻共有容器
judge.register(ContainerInfo(
  id: "c-b",
  state: PropertyState(kind: pskCoupleShared, owner_a: "lolict", owner_b: "刘楚恬"),
  metadata: init_table[string, string]()
))

# 容器 C: 无主容器
judge.register(ContainerInfo(
  id: "c-c",
  state: PropertyState(kind: pskOwnerless),
  metadata: init_table[string, string]()
))

# 容器 D: 集体共有容器
judge.register(ContainerInfo(
  id: "c-d",
  state: PropertyState(kind: pskCollective, owners: @["lolict", "刘楚恬", "charlie"]),
  metadata: init_table[string, string]()
))

echo "【容器注册表】"
for id, info in judge.containers:
  echo "  " & id & " → " & state_desc(info.state)
echo ""

# ── 判定场景 ──
echo "【判定场景全集】"
echo ""

var case_idx = 0

# 场景 1: 主人操作自己的容器 → 无侵权
inc case_idx
let flow1 = make_flow("f1", ftTransformation, "c-a", "lolict")
  .with_auth(Authorization(kind: akFull))
let r1 = judge.judge(flow1)
echo "场景 " & $case_idx & ": 主人 lolict 变换自己的容器 c-a"
echo "  → " & infringement_desc(r1.infringement) & " | " & r1.explanation
echo "  → 路径: " & r1.path.join(" → ")
echo ""

# 场景 2: 非主人无授权转移 → 盗用
inc case_idx
let flow2 = make_flow("f2", ftTransfer, "c-a", "charlie")
let r2 = judge.judge(flow2)
echo "场景 " & $case_idx & ": 非主人 charlie 无授权转移 c-a"
echo "  → " & infringement_desc(r2.infringement) & " | " & r2.explanation
echo "  → 路径: " & r2.path.join(" → ")
echo ""

# 场景 3: 有授权但越界 → 越界侵权
inc case_idx
let flow3 = make_flow("f3", ftBorrow, "c-a", "charlie")
  .with_auth(Authorization(kind: akBounded, boundary: "read_only"))
  .with_attr("scope", "write")
let r3 = judge.judge(flow3)
echo "场景 " & $case_idx & ": charlie 借用 c-a，授权边界 read_only 但 scope=write"
echo "  → " & infringement_desc(r3.infringement) & " | " & r3.explanation
echo "  → 路径: " & r3.path.join(" → ")
echo ""

# 场景 4: 无主容器 → 自由占用
inc case_idx
let flow4 = make_flow("f4", ftCreation, "c-c", "anyone")
let r4 = judge.judge(flow4)
echo "场景 " & $case_idx & ": anyone 操作无主容器 c-c"
echo "  → " & infringement_desc(r4.infringement) & " | " & r4.explanation
echo "  → 路径: " & r4.path.join(" → ")
echo ""

# 场景 5: 隐匿来源 → 隐匿侵权
inc case_idx
let flow5 = make_flow("f5", ftTransfer, "c-a", "charlie")
  .with_attr("source_hidden", "true")
let r5 = judge.judge(flow5)
echo "场景 " & $case_idx & ": charlie 隐藏来源转移 c-a"
echo "  → " & infringement_desc(r5.infringement) & " | " & r5.explanation
echo "  → 路径: " & r5.path.join(" → ")
echo ""

# 场景 6: 篡改编码 → 篡改侵权
inc case_idx
let flow6 = make_flow("f6", ftTransformation, "c-a", "charlie")
let r6 = judge.judge(flow6)
echo "场景 " & $case_idx & ": charlie 无授权变换 c-a"
echo "  → " & infringement_desc(r6.infringement) & " | " & r6.explanation
echo "  → 路径: " & r6.path.join(" → ")
echo ""

# 场景 7: 授权撤回 → 盗用
inc case_idx
let flow7 = make_flow("f7", ftTransfer, "c-a", "charlie")
  .with_auth(Authorization(kind: akRevoked))
let r7 = judge.judge(flow7)
echo "场景 " & $case_idx & ": charlie 在授权撤回后转移 c-a"
echo "  → " & infringement_desc(r7.infringement) & " | " & r7.explanation
echo "  → 路径: " & r7.path.join(" → ")
echo ""

# 场景 8: 夫妻共有，任一方操作 → 无侵权
inc case_idx
let flow8 = make_flow("f8", ftTransformation, "c-b", "刘楚恬")
  .with_auth(Authorization(kind: akFull))
let r8 = judge.judge(flow8)
echo "场景 " & $case_idx & ": 妻子刘楚恬变换共有容器 c-b"
echo "  → " & infringement_desc(r8.infringement) & " | " & r8.explanation
echo "  → 路径: " & r8.path.join(" → ")
echo ""

# 场景 9: 授权条件满足 → 无侵权
inc case_idx
let flow9 = make_flow("f9", ftBorrow, "c-a", "charlie")
  .with_auth(Authorization(kind: akConditional, condition: "pay_fee"))
  .with_attr("condition_status", "satisfied")
let r9 = judge.judge(flow9)
echo "场景 " & $case_idx & ": charlie 借用 c-a，条件 pay_fee 已满足"
echo "  → " & infringement_desc(r9.infringement) & " | " & r9.explanation
echo "  → 路径: " & r9.path.join(" → ")
echo ""

# 场景 10: 集体容器，成员操作 → 无侵权
inc case_idx
let flow10 = make_flow("ft10", ftTransformation, "c-d", "charlie")
  .with_auth(Authorization(kind: akFull))
let r10 = judge.judge(flow10)
echo "场景 " & $case_idx & ": 集体成员 charlie 变换集体容器 c-d"
echo "  → " & infringement_desc(r10.infringement) & " | " & r10.explanation
echo "  → 路径: " & r10.path.join(" → ")
echo ""

# ── 统计报告 ──
echo judge_report(judge)

echo "═══════════════════════════════════════════"
echo "  判定树 9 路径全覆盖完成"
echo "═══════════════════════════════════════════"
