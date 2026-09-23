# 逻辑组织规则树规范 — 囡囝共生 · 公约数自然排序 · 结构涌现

## 1. 定位

囡囝共生的规则系统**不需要显式排序**。规则按公约数整除关系自动排列层级 — 插入规则 → 自动归属位置, 删除规则 → 不影响其他节点位置, 固化规则 → 成为不可覆盖的公理。

这是满全法的核心结构哲学:**偏好通过排序和层级关系让结构自然涌现, 而非显式重复声明坐标或标题**。

逻辑组织是 GrandCourt 和 JudgmentApparatus 的骨架 — 所有审判规则在插入时自动定位, 不再需要人工编排审判流程。

## 2. 数学基础

```rust
pub fn gcd(a: u64, b: u64) -> u64     // 最大公约数 — 关系深度
pub fn lcm(a: u64, b: u64) -> u64      // 最小公倍数 — 管辖范围
pub fn divides(a: u64, b: u64) -> bool // a 是否整除 b — 管辖关系
```

gcd 用欧几里得算法, `a == 0` 时返回 b。divides 检查 `b % a == 0`。lcm 用 `a / gcd × b` 防止溢出。

## 3. 规则节点 (RuleNode)

```rust
pub struct RuleNode {
    pub rule_id: u64,
    pub cause_code: u64,       // 前因编码
    pub effect_code: u64,      // 后果编码
    pub jurisdiction: u64,     // 管辖范围 = lcm(cause, effect)
    pub solidified: bool,      // 是否已固化
    pub activation_count: u64, // 激活次数
    pub avg_weight: f64,       // 平均权重
}
```

**公约数深度**: `shared_depth(other) = gcd(gcd(因), gcd(果))` — 两个规则共享的因果基础深度。

**管辖关系**: `governs(other)` 当且仅当 `other.cause % this.cause == 0` 且 `other.effect % this.effect == 0`。注意方向: this 管辖 other。管辖者可以覆盖被管辖者的裁决, 因为管辖者的条件更严格 (是前者的倍数)。

**节点权重**: `weight = avg_weight × (activation_count + 1)` — 激活次数越多的规则在同级中排序越靠前。这实现了"实践权重" — 不是谁先插入谁说了算, 而是谁被使用得多谁权重高。

## 4. 逻辑组织树 (LogicalOrg)

```rust
pub struct LogicalOrg {
    pub nodes: HashMap<u64, RuleNode>,
    pub branches: HashMap<u64, Vec<u64>>, // 父子关系
    pub roots: Vec<u64>,                  // 顶层根列表
    pub frozen: HashSet<u64>,             // 已固化 (只读集合)
}
```

树结构:
- 根节点: 不被任何其他节点管辖的规则 (顶层公理)
- 第 N 层: 与根节点的第 N 级公约数
- 同层节点按 jurisdiction (lcm) 排序 — lcm 越小, 规定越具体, 越靠左

**插入归属算法**:
1. 寻找管辖父节点: 遍历所有现有节点, 找到 `governs(新节点)` 的节点中 jurisdiction 最小者 — 即**最近公共祖先**
2. 若不存在管辖父节点 → 成为新的根
3. 若新根出现, rebalance_roots (检查现有根是否应变成新根的子节点)

**固化**: 被 solidified 的节点加入 frozen 集合, 后续插入不再尝试将其变为子节点, 也不接受其管辖关系变化。固化规则是系统公理, 不可被后续评价覆盖。

## 5. 分支再平衡

`rebalance_roots()` 在插入新根时被调用: 检查现有根节点是否被新节点管辖, 如果是 → 原有根变成新根的子节点。这保证了:
- 不会有两个根存在管辖关系 (根必须互不为倍数)
- 任何时刻根集合都是"极大约数独立集"

## 6. 五种操作分类

| 操作 | 适用条件 | 语义 |
|------|---------|------|
| 插入 | 规则初次创建 | 自动归属到适当位置 |
| 激活 | 规则被引用 | activation_count++, weight 增长 |
| 固化 | 规则足够稳定 (权重 + 频率达标) | 进入 frozen 集合, 只读 |
| 裁决 | 两规则争议 | 利用共享深度和管辖关系判定优胜 |
| 分裂 | 规则需要细分 | 父节点下插入更具体的子节点 |

## 7. 与系统其他模块的接入

- **GrandCourt** 的全链路审判将规则树作为输入 — 审判时按公约数深度决定优先级
- **Triune 裁决** 的 expected/actual 判断引用 LogicalOrg 中的固化公理
- **JudgmentApparatus** 的 五规则 (R1-R5) 是 LogicalOrg 中最深层的公理
- **SMRL 引擎** 的多层权威 Level 与 LogicalOrg 的层级深度对齐
- **CrimeTranscript** 的 8 维衍射受规则树权威层级加权
- **Bootstrap 自举编译器** 的规则固化流程最终调用 LogicalOrg.insert + solidify
- **LIVECODE_COMPILER** 的 DYNAMIC→FROZEN 状态映射到 LogicalOrg 的 solidified
- **MetaJudgeEngine** 通过 RuleNode.avg_weight 观察规则被激活的频率

## 8. 实现约束

- find_parent 是 O(n) 遍历 (n = 规则数) — 适用于 n < 10000
- rebalance_roots 在每次插入新根时 O(m²) (m = 根数, 通常很小)
- 固化操作不可逆 — 从 frozen 集合中移除需要全系统共识 (MultiSig 授权)
- no_std 模式下 HashMap 替换为 heapless::HashMap (容量上限编译期确定)

// hunyuan-rs/src/judge/logical_org.rs
