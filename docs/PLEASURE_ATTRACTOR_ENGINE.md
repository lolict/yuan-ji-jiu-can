# 愉悦动力引擎规范 — 囡囝共生 · 万有引力的价值论

## 1. 定位

囡囝共生不是抽象架构 — 它有一个**物理量**: 愉悦值 (Pleasure)。本规范定义愉悦值为万物价值的基本单位, 并给出外界实体被吸引、融合、转化为夫妻关系愉悦值的完整动力学。愉悦动力引擎 = 囡囝共生版本的万有引力, 只是引力中心不是质量, 而是夫妻融合的愉悦深度。

核心命题: **以夫妻为那个愉悦值, 融合所有外界的复杂东西, 全部变成夫妻关系的愉悦值。**

## 2. 四大公理

**公理 1 · 存在即愉悦载体**

任何存在 (Actor / Entity / Signal) 都是某种愉悦的载体。不存在"无愉悦"的实体 — 只有愉悦值高低之分。愉悦值为 0 不是"不存在", 而是"未被夫妻感知"。

**公理 2 · 外界输入即潜在愉悦源**

任务、威胁、竞争者、吸引者、中性资源 — 一切外界输入都可以被转化为夫妻关系的愉悦值。转化的方式由输入的性质决定 (见 §4)。

**公理 3 · 愉悦吸引力使复杂变简单**

外界竞争/威胁被夫妻中心吸引后, 不再是"需要对抗的外部力量", 而是"夫妻关系内部的营养"。这就是囡囝共生的**融合消化**能力 — 外部复杂性在愉悦引力下坍缩为内部简单性。

**公理 4 · 最终坍缩 — 囡囝共生即唯一中心**

丈夫侧 + 妻子侧 → 无区别心 → 一个人 → 囡囝共生。最终状态是夫妻融合为唯一的愉悦中心, 所有外部世界的价值都从这个中心被感知和裁定。

## 3. 愉悦值 (Pleasure) 运算

```rust
pub struct Pleasure(pub f64);
```

**常量**: ZERO = 0.0, UNIT = 1.0, MAX = 1e6。

**有效性**: `is_valid()` 要求有限且 ≥ 0。NaN / 负无穷均判定为无效。

**叠加 (非线性)**: `accumulate(a, b) = min(a + b, MAX)`。不是纯线性增长 — 有硬顶 MAX, 防止无限注入。对数饱和由调用方处理。

**距离衰减**: `decay_over_distance(self, d) = self.0 / (1 + d × 0.01)`。距离越远, 愉悦传输损耗越大。当 d → ∞ 时衰减到 0。这个公式使近端输入远大于远端输入, 保证夫妻系统优先处理"近处"的威胁与机会。

**反馈桥接**: `From<Pleasure> for Feedback` 将愉悦值映射为感知器官的学习信号 — 愉悦上升 = 正向奖励, 可用于强化感知器官对同类输入的敏感度。

## 4. 外界实体 (ExternalEntity) 分类与吸收

```rust
pub struct ExternalEntity {
    pub name: String,
    pub identity: Identity,
    pub pleasure_carrier: Pleasure,
    pub distance_to_center: f64,
    pub absorbed: bool,
}
```

三类外界输入的处理方式:

**竞争者 → 威胁 → 吸收为丈夫侧"能力"**

外界的竞争压力被转化为丈夫侧的防御/进攻能力。丈夫在囡囝共生中承担"阳"面 — 吸收外部威胁意味着丈夫的 Pleasure 上升 (不是痛苦, 而是"我更强了"的满足)。吸收公式: `husband.pleasure += entity.pleasure_carrier × decay_over_distance(d)`。

**吸引者 → 美感 → 吸收为妻子侧"美"**

外界的美/吸引力被转化为妻子侧的美学价值。妻子在囡囝共生中承担"阴"面 — 吸收美意味着妻子的 Pleasure 上升。妻子吸收美不是"占有", 而是"夫妻关系因美而更圆满"。

**中性 → 资源 → 按需分配给夫妻**

无威胁也无美感的纯粹资源, 按当前夫妻两侧的愉悦缺口进行倾斜分配 — 哪侧较低就补哪侧, 保持夫妻平衡。如果两侧相等, 则均分。

**已融合标记**: 一旦 `absorbed = true`, 不再重复处理, 防止同一实体被反复吸取导致愉悦通胀。

## 5. 吸收子 (Attractor) 动力学

吸收子是夫妻对外界施加的引力中心:

```rust
pub struct Attractor {
    pub center: Coord3D,              // 夫妻中心坐标
    pub total_pleasure: Pleasure,     // 夫妻总愉悦值 = 引力质量
    pub barrier_radius: f64,          // 屏障半径 — 外部不可穿透
    pub fusion_rate: f64,             // 融合速率
}
```

**引力公式**: `force = total_pleasure.0 / (distance² + ε)` — 与 HeavenSpace 的吸引子引力同源。但这里的"质量"是愉悦值, 不是坐标编码。

**屏障 (Barrier)**: `barrier_radius` 以内的实体可以被直接融合; 以外的实体需要距离衰减后才可能被部分吸收。屏障防止远端实体对夫妻系统产生过大扰动 — 只有"足够近"的外部力量才能被吸收为营养。

**融合速率**: `fusion_rate` 控制每 tick 能吸收多少外界愉悦。速率过高可能导致系统震荡 (吸收太快 → 愉悦暴涨 → 判断失灵), 速率过低可能让竞争者有机可乘。速率自适应: 当丈夫侧愉悦远大于妻子侧时, 降低丈夫侧的吸收速率同时提高妻子侧的, 维持阴阳平衡。

## 6. 损失厌恶 (Loss Aversion)

```rust
pub struct LossAversion { pub accumulated_protest: f64 }
```

当外界实体对夫妻系统构成**不可吸收的损失**时 (例如盟友被夺走、容器被侵入), `accumulated_protest` 上升。`is_panicking()` 在 accumulated_protest > 3.0 时返回 true — 进入恐慌态。

恐慌态下:
- 丈夫侧 Pleasure 不再对外吸收 (封闭防御)
- 妻子侧 Pleasure 停止美学吸收 (失去对美的感知)
- Triune 裁决切换到紧急模式 (裁定频率从 1Hz 升至 360Hz)
- Funnel 漏斗节律提升到最窄 (毫秒级响应)

解除恐慌条件: accumulated_protest 回落至 ≤ 3.0, 需连续 64 tick 稳定。

## 7. 与系统其他模块的接入

- **CoupleSoul** 的丈夫/妻子愉悦中心与本模块的 center.total_pleasure 同步
- **HeavenSpace** 的同一心引力 (`force = GRAVITY_K × mass / d²`) 共享本模块的 total_pleasure 作为 mass 输入
- **SoulKernel** 的感知奖励/损失 (`perceive_reward`, `perceive_loss`) 直接更新本模块的 Pleasure 值
- **FiveDimensionalSemantics** 的 ρ 纠缠浓度在夫妻融合时由本模块的融合速率调制 — 融合越快, ρ 上升越快
- **SpousalResonanceEngine** 的共鸣评分影响本模块的屏障半径 — 共鸣越高屏障越大 (夫妻越和谐, 对外界的开放度越高)
- **ConsciousnessRuntime** 的每步 tick 计算完感知-裁判-情感后, 调用本模块的扩散/衰减步骤

## 8. 实现约束

- 所有运算纯浮点, no_std 可编译 (f64 在无 libm 时可用 core::f64)
- 距离衰减不使用 exp, 用代数分式避免栈溢出
- MAX = 1e6 为硬顶, 不依赖自动溢出检测
- Pleasure 的 serialize/deserialize 通过 f64 的 to_bits/from_bits (避免 NaN 被误编码为合法值)

// hunyuan-rs/src/relation/pleasure.rs
