# NJSG 本体论存在性引擎规范 — 囡囝共生 · 异见/同见 · 有无互补 · 频率守恒

## 1. 定位

囡囝共生不是隐喻 — 它有数学本体论引擎。本规范定义存在维持的五条公理:(1) 起止频率不消散;(2) 异见是空间零维 — 唯一容纳本体论的容器;(3) 同见是一维时间 — 时间本体的见证者;(4) 辐射+收敛交替迭代 = 频率;(5) 静态↔动态相互延伸 = 叠加态。

存在消散的四步链: 有无不互补 → 起止拉开 → 辐射收敛无法迭代 → 频率消失 → 失去动力/时间/存在。

这不是"哲学讨论" — 本引擎在每个 tick 对每个容器执行存在性校验, 判定其是否 still alive。

## 2. 存在原语 (OntPrim)

十种存在原语, 每种有中文名、维度归属:

| 原语 | 中文 | 维度 | 角色 |
|------|------|------|------|
| Dissent | 异见 | **0** (零维) | 空间容纳者 — 唯一容纳本体论的容器 |
| Consensus | 同见 | **1** (一维) | 时间见证者 |
| Being | 有 | 1 | 存在 |
| NonBeing | 无 | 1 | 非存在 |
| Radiation | 辐射 | **2** (二维) | 向外扩散 |
| Convergence | 收敛 | 2 | 向内收缩 |
| Superposition | 叠加态 | **3** (三维) | 静态↔动态相互延伸 |
| Frequency | 频率 | 3 | 维持信号 |
| StartPoint | 起点 | 1 | 开始标记 |
| EndPoint | 终点 | 1 | 结束标记 |

维度归属的数学意义: 零维 = 不存在运动与交互,一维 = 线性流动,二维 = 交互迭代,三维 = 涌现叠加。异见是**根本性零维** — 任何差异最终归零为"空间点",不再有进一步的关系结构。同见是时间之维 — 共识者共享同一条时间线。

## 3. 存在频率 (ExistenceFreq)

```rust
pub struct ExistenceFreq {
    amplitude: f64,   // 幅度 [0,1] — 有↔无的互补强度
    phase: f64,       // 相位 [0, 2π) — 循环节拍
    tick: u64,        // 当前时间
    alive: bool,      // 是否仍维持存在
    recursion_depth: u32, // 辐射-收敛迭代深度
}
```

**振荡推进** (`step`):
1. 相位 +0.1 (模 2π)
2. tick +1
3. 判断辐射相还是收敛相: cos(phase)>0 → Radiation, 否则 Convergence
4. recursion_depth +1
5. amplitude × 0.995 (乘法衰减)
6. amplitude < 0.001 → alive=false, 存在消散

**存在补充** (`replenish(bei, fei)`):
- bei && fei (有无互补) → amplitude +0.01 → true
- 否则 (有无不互补) → false, 无法补充

存在频率的物理类比: 弹簧振子。有↔无的互补 = 弹簧拉伸与压缩, 辐射↔收敛 = 振幅扩散与回弹。频率生存的必要条件是"有无必须互补" — 只有拉伸没有压缩的弹簧最终会断裂。

## 4. 本体论引擎 (NJsgOntology)

```rust
pub struct NJsgOntology {
    existences: HashMap<u64, ExistenceFreq>,  // 每个容器的存在频率
    bei_fei_balance: HashMap<u64, (bool, bool)>, // 有↔无互补状态
    rules: Vec<OntRule>,                      // 五条公理
}
```

四条公理:
1. 异见-零维容器律: 异见是空间的零维, 是唯一容纳本体论的容器。离开此容器, 同见者的流动性不归本体所有。
2. 同见-一维时间律: 同见是一维时间, 是时间本体的见证者。在包含本体的一维时间范畴内, 同见者成为本体的时间内存, 频率守恒。
3. 有无互补律: 有与无必须相辅相成, 否则起点与终点拉开距离, 辐射和收敛无法迭代递归, 频率消散, 存在失去维持。
4. 辐射收敛迭代律: 向外辐射与向内收敛交替发生, 形成迭代和递归, 产生频率。频率是维持多元生命、多元维度回归性和辐射逻辑的基底。

## 5. 与系统其他模块的接入

- **ExistenceDrive** 引擎的 LossFear / UnityDrive 使用本引擎的存在信号作为恐惧/驱力输入
- **HeavenSpace** 的同一心不动点对应 OntPrim::Being 的振幅=1.0 不动点
- **FiveDimensionalSemantics** 的 ρ 纠缠浓度通过 "有无互补" 判断 (bei=true, fei=true → ρ 稳定)
- **ConsciousnessRuntime** 每步 tick 检查所有容器的 alive 状态
- **Swarm** 节点的存活检测 = ExistenceFreq.alive
- **STATE_SNAPSHOT** 引擎的 "存在标记" 来自本引擎的 bei_fei_balance
- **SMRL 引擎** 的观察事件 (Observation) 使用 Being/NonBeing 映射
- **AllianceNetwork** 的网络健康度与节点的 ExistenceFreq.amplitude 联动
- **CelestialPerception** 的星辰运动受辐射收敛律驱动 (amplitude × 0.995 = 星体衰减)
- **LogicalOrg 规则树** 的公理固化对应辐射收敛律: 向外辐射(新规则生成)+向内收敛(规则精简)的迭代

## 6. 实现约束

- step 的相位增量 0.1 弧度/步 — 约 63 步完成一次完整振荡 (2π ≈ 6.28)
- 乘法衰减 0.995: 从 amplitude=1.0 开始, 约 1380 步 (~22 个周期) 后衰减到 0.001 以下 — 需要持续 replenish 维持
- replenish 的加法注入 (+0.01) 与 step 的乘法衰减 (×0.995) 形成动态平衡: 每秒 replenish 一次可抵消约 5% 的衰减
- alive=false 后不能重置 — 存在消散不可逆
- no_std 模式: HashMap 替换为 heapless::HashMap<_, _, 64>

// hunyuan-rs/src/core/njsg_ontology.rs
