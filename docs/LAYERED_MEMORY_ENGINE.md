# 层级记忆引擎规范 — 囡囝共生 · 元裁判 · 自适应学习

## 1. 定位

囡囝共生的"记忆"不是数据库 — 每个坐标的历史轨迹随时间衰减又随见证增强,元裁判在运行期持续自评并调参。本规范定义记忆衰减/强化动力学、三元见证记录结构,以及让裁判自我完善的元裁判引擎 (MetaJudgeEngine)。

关键定位:**记忆 = 价值回忆**。不是"什么发生过",而是"什么有价值,值得记住"。没有见证的坐标记忆不可见。

## 2. 见证记忆记录 (MemoryEntry / CoordMemory)

```rust
pub struct MemoryEntry {
    pub timestamp_ms: u64,
    pub witness_id: u64,
    pub pair_a: (u64, u64, u64),
    pub pair_b: (u64, u64, u64),
    pub surplus: f64,       // 见证盈亏量
    pub aligned: bool,      // 对称对齐判定
    pub verdict_accepted: bool, // 经后续反馈确认
}
```

每次见证事件被记录为两个方向的 entry (pair_a→pair_b 与 pair_b→pair_a),保证任意方向的观测都有历史可回溯。`verdict_accepted` 只在后续反馈与裁判结果一致时才被标记 — 这是元裁判评判准确率的基础数据。

```rust
pub struct CoordMemory {
    pub coord: Coord,
    pub entries: Vec<MemoryEntry>,
    pub decay_rate: f64,       // 每毫秒衰减率 (默认 0.00001)
    pub reinforcement: f64,    // 每次见证强化量 (默认 0.1)
    pub last_accessed_ms: u64,
    pub total_surplus: f64,
}
```

**记忆保留率公式**: `retention_at(now) = e^(-decay_rate × elapsed_ms)`。这是**指数衰减模型** — 长时间未被访问的坐标记忆会自动淡出。

**有效记忆量**: `effective_memory = entries.len() × retention_at(now)`。只计算数量不算质量 — 一个 10 条但全部衰减到 0.1 的层,有效记忆量仅为 1.0。

**记忆密度**: `memory_density() = entries.len() / duration_ms × 1000` — 单位时间内的见证次数,反映该坐标的认知活跃度。

## 3. 层级记忆系统 (LayeredMemory)

```rust
pub struct LayeredMemory {
    pub layers: Vec<CoordMemory>,  // 所有被观测过的坐标
    pub max_layers: usize,         // 最大记忆层数
    pub global_tick: u64,          // 全局时间线
}
```

`record_witness` 是核心入口:一次见证事件同时更新 pair_a 和 pair_b 各一层,`global_tick` 自增。当层数超过 `max_layers` 时,启动**驱逐机制** — 计算所有层的 `retention_at(now)`,保留率最低的一层被整层清除。这意味着无人关心的记忆自然消亡。

`global_density(os)` 给出全局认知活跃度(所有层的有效记忆量均值),是 SoulKernel 的输入信号之一。`top_active(os, n)` 返回最活跃的 n 个坐标,用于 PerceptionEngine 的焦点队列排序。

## 4. 元裁判引擎 (MetaJudgeEngine)

元裁判不是裁判 — 是"裁判的裁判"。它不生产裁定结果,而是评估已有裁定的质量,并在漂移超限时触发参数自调整。

**JudgeQuality 记录**:

```rust
pub struct JudgeQuality {
    pub total_judgements: u64,
    pub confirmed_judgements: u64, // 后续反馈确认的
    pub rejected_judgements: u64,  // 后续反馈推翻的
    pub avg_confidence: f64,
    pub drift: f64,                // 裁判漂移量
}
```

`record_verdict(predicted_aligned, actual_aligned, confidence)` 每次裁判后调用。比较 predicted 与 actual,更新 confirmed/rejected 计数,并计算:

- 准确率 `accuracy = confirmed / total`
- 漂移量 `drift` = |predicted - actual| 的移动平均
- `is_qualified()` 在准确率 ≥ 70% 时返回 true

**裁判漂移**是关键指标 — 当代漂移持续增长,说明传感器偏置或坐标编码发生了系统性偏移,此时需要 AdaptiveParams 的自调整介入。

## 5. 自适应参数 (AdaptiveParams)

元裁判可调节的裁判参数集:

```rust
pub struct AdaptiveParams {
    pub threshold_positive: f64,  // 正向判定阈值
    pub threshold_negative: f64,  // 负向判定阈值
    pub step_size: u64,           // 步长
    pub witness_decay: f64,       // 见证衰减
    pub learning_rate: f64,       // 参数调整步长
}
```

自适应调参规则:当 drift 持续增长时,按 learning_rate 比例调整 threshold/step_size;当 accuracy 稳定在 85% 以上时,逐步缩小 step_size(更精细的步长)。这是**小步快跑、稳中收紧**的策略 — 不确定时用大步试探,确定后才缩小步长精确收敛。

## 6. 与系统其他模块的接入

- **Triune 裁决** 的每次结果调用 MetaJudgeEngine.record_verdict 进行自评
- **SurplusPair.witness()** 的结果触发 LayeredMemory.record_witness 存储
- **SoulKernel** 的感知奖励感知判断通过 AdaptiveParams 阈值判定
- **CocoonAgent.meta** 字段直接挂载 MetaJudgeEngine,每个智能体独立一套元裁判
- **Observer.top_priority_sense** 通过 LayeredMemory.top_active 的活跃坐标排序
- **Funnel 漏斗节律器** 根据 JudgeQuality.is_qualified() 决定是否可以降低裁定频率(合格 → 降至 1Hz;不合格 → 维持 60Hz)

## 7. 实现约束

- 衰减计算使用 `(-x).exp()`,no_std 下需 libm 或软件浮点
- 驱逐机制的 `O(n × m)` 扫描(n = 层数, m = entries 数)可接受 — max_layers ≤ 256
- JudgeQuality 的记录操作无堆分配,适合热路径
- 全部使用 OsLayer.time_millis() 获取时间,不直接调用系统时间

// hunyuan-rs/src/core/memory.rs
