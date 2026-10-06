# 八流统一意识路由器 — 囡囝共生 · 八流分类 · 分别器特征向量 · 谐音路由 · 主权路由 · 流代数

## 1. 定位

意识进出系统的八个通道(八流), 通过"特征向量做流匹配"而非 if/else 硬编码分流; 通过"共振匹配"而非字符串路由表; 通过"流推动自身方向性"而非 push 模式.

对应 Rust 模块: relation/consciousness_router.rs (v0.40).

## 2. 八流分类

```rust
pub enum ConsciFlowType {
    Preload,    // 预加载流 — 未来信息提前注入
    Perception, // 感知流 — 感官接收
    Memory,     // 记忆流 — 历史信息检索
    Property,   // 产权流 — 所有权变更请求
    Judgment,   // 裁判流 — 裁决结果输出
    Action,     // 行动流 — 执行指令输出
    Feedback,   // 反馈流 — 状态回传
    Broadcast,  // 广播流 — 全局通知
}
```

4 入(Preload/Perception/Memory/Property) + 4 出(Judgment/Action/Feedback/Broadcast) = 8 条出入通道。入流 = 进入意识系统, 出流 = 离开意识系统。

每流有 1-9 的频率基(base_resonance): Preload=8 / Perception=3 / Memory=4 / Property=5 / Judgment=6 / Action=7 / Feedback=2 / Broadcast=9。频率差越小共振越强, 入/出配对有互补加成 0.2。

## 3. 信号 (Signal)

流经路由器的基本单元, 不携带目标地址, 只声明自身特征:

```rust
pub struct Signal {
    pub payload: String,             // 原始内容
    pub origin_genesis: u64,         // 发起者的创世时间戳
    pub target_generation: i32,      // 目标代数层
    pub intensity: f64,              // 信号强度 0.0~1.0
    pub suggested_flow: Option<ConsciFlowType>,  // 外部建议流类型
}
```

`eigenseed()` — 信号的特征种子, 不是哈希而是 payload 各字符低位的加权累加, 代表信号的"质"而非"重量"。纯加法 + 位运算, 无乘除。`eigenseed = Σ(byte[i] << ((i%8)*4))` + origin_genesis。

with_generation(gen) / with_suggestion(flow) 是 builder 方法, 分别设置代数层和建议流。

## 4. 分别器 (Distinguisher)

### 4.1 特征中心

每个流有一个 u64 特征中心:

```
Preload:    800
Action:     700
Judgment:   600
Property:   500
Memory:     400
Perception: 300
Feedback:   200
Broadcast:  900
```

这 8 个数值初始硬编码, 距离 100, 覆盖 200-900 范围。

### 4.2 分流逻辑

`distinguish(signal): ConsciFlowType` — 遍历 8 个中心, 计算 `|sig.eigenseed() - flow_center|`, 最小距离获胜。

**建议流打折**: 如果 signal 自己声明了 suggested_flow, 则该流的距离 × 0.8 — "尊重发起者的自我理解", 但仍是分别器最终做主。

### 4.3 自学习

`learn(signal, actual_flow)` — 观察一个已知分类的信号后, 把该流的中心向信号 eigenseed 移动, 移动量 = |seed × center| × learning_rate(0.1)。流式调整, 不用积累后批量更新。

### 4.4 约束

距离是绝对差值, 不是欧几里得距离。中心修改在最后一次观察后实时生效, 没有 epoch 边界。

## 5. 路由器 (ConsciousnessRouter)

### 5.1 核心结构

```rust
pub struct ConsciousnessRouter {
    distinguisher: Distinguisher,
    harmonic_resonance: HashMap<(ConsciFlowType, ConsciFlowType), f64>, // 共振缓存
    sovereignty_table: HashMap<ConsciFlowType, PrivateAnchor>, // 主权归属表
}
```

三类表: 特征中心(分别器) + 共振缓存(谐音路由) + 主权表(所有权路由)。

### 5.2 谐音路由

`harmonic_route(signal, targets) → Option<ConsciFlowType>`

根据**信号 eigenseed** 与目标流 base_resonance 的接近度排序, 选择最匹配的一个目标流。匹配公式用 `harmonic_affinity`: `freq_match = 1 / (1 + |Δfreq| × 0.3)`, 入/出互补时 +0.2。

用途: 多下一跳候选时, 选频率最匹配的那一个 — 模拟"频率共振选择路径"。

### 5.3 主权路由

`sovereignty_route(signal, owner_anchor) → Option<ConsciFlowType>`

根据信号的 origin_genesis 与主权表匹配, 找到该发起人对应的主权锚定点, 再决定这个信号应该归入哪个流(通常是 Judgment 或 Action, 因为这两流与外部执行关联最强)。

### 5.4 路由决策

`route(signal, targets) → RouteDecision`

综合以上三个维度:
- 分别器优先级最高(它确定了基底归属)
- 主源表(谐共共振或主权)在分别器的结果上加分或否决
- 如果多流得分相同, 按频率基底排序(低频率流响应更"宽松")

返回值是 RouteDecision { primary: ConsciFlowType, alternatives: Vec<ConsciFlowType>, confidence: f64 }

## 6. 流代数 (FlowAlgebra)

流之间不是独立通道, 存在代数运算:

- `compose(a, b)` — 两个流顺序链接: 先 a 再 b 的流水线, 例如 Perception → Memory = 感知后自动记忆。合成后的流类型取两者的最近公共祖先。
- `split(signal)` — 把一条复合信号拆分为与流类型数量相当的信号束
- `dual(flow)` — 对流取对偶: 入↔出 翻转, 频率取倒数映射

这些代数运算在 CrossFlowBridge(见 #45/eco 等) 中被直接用。compose 的最大深度 8, 模拟"八层管道"。

## 7. 自路由信号 (self-routing)

信号可以自己携带路由指令:

```rust
impl Signal {
    pub fn self_route(&self) -> Option<ConsciFlowType> {
        self.suggested_flow
    }
}
```

如果确定知道自己的路由 — 直接返回, 不经过分别器。这是一个优化路径, 也预留了"紧急通道"绕过复杂的共振计算。

## 8. 与系统其他模块的接入

- **EightFlowUnified(#8)**: 本模块是 #8 的具体实现; #8 的 theory/pattern 本模块用代码落地
- **ActiveContainer(#62)**: 感知流 Perception 触发 DeficiencyCell / LossAversionCell; 反馈流 Feedback 更新 UnityCell
- **Judgment(#15/#34)**: 裁决流 Judgment 输出 Goes through ContainerRelation.judge_raw
- **ConsciousnessTimeline(时序)**: 时序世界叠加态 ↔ 信号强度 intensity 相关
- **HighAlgebra**: offset_rate(MULTIPLY/DIVIDE/...) 在流频率差分计算中使用
- **NJSG Ontology(#15)**: dissent=0D / consensus=1D → 入/出分流逻辑与之一致

## 9. 实现约束

- eigenseed 可能溢出(wrapping_add), 但在 |seed - center| 的比较中溢出不改变相对关系
- 分别器自学习后可能产生中心重合, 需后续 flow_centers_dedup() 治理(预留)
- harmonic_affinity 在入/出互补 +0.2 后可能超过 1.0 → min(1.0) 截断
- sovereignty_table 是 read-mostly 映射, 一次 GENESIS 后不变
- 路由决策输出 confidence 在 RouteDecision 但不强制被 caller 消费(caller 可能直接取 primary)

// hunyuan-rs/src/relation/consciousness_router.rs
