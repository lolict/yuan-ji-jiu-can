# SMRL 引擎 · 观察-裁判-奖励一体化层规范

## 1. 定位

SMRL 不是编码格式 — 是**运行时的观察者+裁判+奖励发放者三位一体**。它看系统当前状态(观察者),对比预期状态做出裁定(裁判),并输出 +1/0/-1 三级奖惩值(奖励者)。SMRL 引擎是满全法的**心脏自主节律**: 每一次 tick, 它扫描整个系统状态表, 发现对齐就奖励, 发现偏差就惩罚, 歪缩则降低置信度。

SMRL 与 Triune 的关系: Triune 提供三种几何推演 (Collapse / Expand / Static), SMRL 把几何结果翻译为奖惩决策 — Triune 是代数, SMRL 是经济。

## 2. 观察事件 (Observation)

```rust
pub struct Observation {
    pub symbol: u8,        // 哪个符号/节点被观察 (0~255)
    pub present: bool,      // 此刻是否"有"(在表中/在空间/在因果链)
    pub expected: bool,     // 系统期望它此时有/无
}
```

7 个字段精简到 3 个 — 这是**最小可裁判单元**。每个观察事件是一个布尔对 (实际,预期)。不需要历史、不需要上下文 — 单个事实的对照就足够做出奖惩。

`to_being()` 将 present 映射为 BeingState::Being(symbol) 或 BeingState::NonBeing, 直接输入 Triune。

## 3. 奖惩判决 (SmrlVerdict)

```rust
pub struct SmrlVerdict {
    pub symbol: u8,         // 谁被裁决
    pub reward: Reward,     // Plus(+1) / Zero(0) / Minus(-1)
    pub level: u8,          // 权威级别 — 谁覆盖谁
    pub confidence: f64,    // 来自 triune 的置信度 [0,1]
    pub crooked: f64,       // 漏斗歪斜度 [0,1] — 0=完美
}
```

奖惩规则:
- 对齐 + 有内容 → **Plus** (系统运行正常, 有物)
- 对齐 + 都无内容 → **Zero** (空位是合理的)
- 不对齐 → **Minus** (出错)

`weighted_score()` = base × (level + 1) × confidence。level 0 → 乘 1, level 3 → 乘 4 — 高级别裁判乘以强硬系数。这不仅是"权威加权",更是**权威放大**: 高级别的一分奖励等于低级别的四分。

## 4. 权威层级 (Level)

SMRL 引擎可以多实例并存, 每个实例标注自己的 level。`resolve_authority()` 用高 level 覆盖低 level:

```rust
pub fn overridden_by(&self, other: &SmrlVerdict) -> SmrlVerdict {
    if other.level > self.level { *other } else { *self }
}
```

这是**非辩论式覆盖** — 高级别不需要说服低级别,直接取代。适用于:
- 系统级 (level 4) 覆盖进程级 (level 2)
- 天庭级 (level 7) 覆盖人间级 (level 3)
- 夫妻联合级 (level 9) 覆盖任何单方级

level 0 时加权值为 1, 而非 0 — 确保最基础的观察者仍然有意义。

## 5. 角度裁判 (judge_angular)

除了布尔对照, SMRL 还提供**角度关系裁判**: 两个 Symbol 在几何空间中的角度关系映射为奖惩:

| 角度关系 | 奖励 | 语义 |
|---------|------|------|
| Overlap  | Plus  | 同体共振: 完美对齐 → 奖励 |
| Aligned  | Plus  | 方向一致: 协调 → 奖励 |
| Diagonal | Zero  | 正交独立: 不增不减 → 中性 |
| Opposite | Minus | 反向相消: 消磨 → 惩罚 |
| Off      | Minus | 不相关噪音: 干扰 → 惩罚 |

置信度由 `cos(弧差弧度)` 决定: 角度为 0 → confidence=1.0; 角度增大 → confidence 余弦衰减。`Diagonal` 即使中性, confidence 也适中。

## 6. 漏斗歪斜 (Crookedness)

```rust
pub fn with_crookness(level: u8, crooked: f64) -> Self
```

歪斜度 crooked ∈ [0, 1]。crooked = 0 代表完美漏斗, crooked = 0.8 代表严重偏移。歪斜引擎的奖惩方向仍然正确, 但 confidence 被 triune 调低。

crooked 的物理意义: 漏斗 (Funnel) 节律器失灵, 裁定频率飘移。此时 SMRL 的判定仍然可以运行, 但输出不可全信 — 必须降低置信度, 让上级 SMRL 更容易覆盖它。

## 7. 批量观察 (judge_batch)

```rust
pub fn judge_batch(&self, obs_list: &[Observation]) -> Vec<SmrlVerdict>
```

批量输入观察列表, 逐条裁定, 按 level 降序排列输出。适用于 PerceptionEngine 一次感知多信号时, SMRL 批量处理 + 按优先级排序。

## 8. 与系统其他模块的接入

- **Triune 裁决** 的 `TriuneVerdict { alignment, being, confidence }` 直接翻譯为 SmrlVerdict
- **SoulKernel** 的感知输入通过 Observation::new(symbol, present, expected) 编码
- **ConsciousnessRuntime** 的每步 tick 调用 SmrlEngine::judge_batch 收集奖惩
- **ManQuanFaContainer** 的 `perceive_reward/loss` 从 SmrlVerdict.reward 取值
- **RewardCircuit** (ExistenceDrive) 接收 SMRL 的 Plus/Zero/Minus 驱动历史 EMA
- **MetaJudgeEngine** 在评估裁判准确率时, 比较 SmrlVerdict 与后续反馈
- **ParasiticShell** 检测用 SMRL 判断外部寄生体的存在/期望差异
- **Funnel 漏斗节律器** 的 frequency 偏移量映射到 SmrlVerdict.crooked
- **CelestialSpace** 的天庭空间用 SMRL 判断天象(日月星辰)的预期/实际对齐

## 9. 实现约束

- entire module < 200 行代码, 单文件, 零内部 mod
- 纯整数 + 浮点运算, 无堆分配 (judge_batch 除外, 它分配 Vec)
- 不依赖系统时钟 — 时间由外部注入
- Reward 使用 enum (Plus/Zero/Minus), 不用整数常量
- no_std 兼容: 去除 judge_batch 后可用固定大小数组

// hunyuan-rs/src/core/smrl.rs
