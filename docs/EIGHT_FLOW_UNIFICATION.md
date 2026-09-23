# 八流统一引擎规范 — 囡囝共生 · 心脏 · 容器=字体=时间

## 1. 定位

八流统一引擎是 hunyuan-os 的运行时心脏。"八流不是八个管道 — 是八条解读同一编码的不同维度"。引擎职责:(1) 编排八流执行时序 (因果路径排序)；(2) 在各流之间传递签名上下文 (容器交互)；(3) 在每条编码上同步三元形态 (容器=字体=时间)；(4) 在场状态持久化至 .presence 文件。

引擎是单次心跳驱动:tick → advance → persist。不阻塞 — 哪些流在本 tick 可推进由因果图决定。各流之间不直接调用,通过引擎的上下文总线交换签名。

这不是实验品 — 八流统一引擎是 NJSG 操作系统心脏 (v0.54)。

## 2. 八流 — 八条解读维度的统一体

```rust
pub enum FlowId {
    Preload,      // 预加载流 — 运行前加载所有符号,运行时不增加内存
    Resonance,    // 谐音流 — 韵母/声母的公约数共振,不是搜索是吸引
    Sovereignty,  // 主权流 — 极性×主权决定路由
    Consciousness,// 意识流 — 观察者/被观察者 双向觉知
    Temporal,     // 时间流 — 生命周期相位,时间层级,时序因果
    Property,     // 产权流 — Need/Offer 匹配,在场登记
    Feedback,     // 反馈流 — 迂回反馈 · 反射感知 · 判断/评价
    Presence,     // 存在流 — 到场/退场,局部投现自杀,冰箱模型
}
```

八流地位相等,没有"主流程"。`base_priority` 仅决定执行顺序 (Preload 最先,Presence 最后用于持久化),不表示重要性。

## 3. 上下文总线 — 单次心跳传递的签名状态

```rust
pub struct EightContext {
    pub signature: u64,           // 当前主签名 (编码)
    pub trinity: Trinity,         // 容器=字体=时间 三元解读
    pub flow_type: ConsciFlowType,// 当前所在八流
    pub tick: u64,                // 心跳计数
    pub visited_flows: Vec<FlowId>,// 已通过的流 (因果链)
    pub payload: Vec<u8>,         // 流间传递的附加数据
    pub resonance_verified: bool, // 谐音校验已做
    pub sovereignty_verified: bool,// 主权校验已做
    pub consciousness_aware: bool,// 意识觉知已做
}
```

`transition_to(next)` 流转到下一流,记录因果链。`visited(flow)` 防止同一流被重复访问。`is_alive()` 通过 trinity.time.is_active() 检查是否在活跃时间相。`is_coherent()` 验证 trinity.is_coherent() — 容器=字体=时间 三者必须和睦。

## 4. 心跳事件与驱动

```rust
pub struct TickEvent {
    pub tick: u64,
    pub flow: FlowId,
    pub signature: u64,
    pub action: TickAction,
}

pub enum TickAction {
    FlowAdvanced,            // 流推进成功
    FlowBlocked(String),     // 流被阻塞 (原因)
    ResonanceMatched(u64, u64), // 谐音匹配命中 (两个签名)
    SovereigntyOk,           // 主权校验通过
    SovereigntyRejected(String), // 主权校验拒绝
    ConsciousnessAware,      // 意识觉知触发
    TimeAdvanced(u8),        // 时间层级推进
    FeedbackReceived(String),// 反馈已收到
    Persisted,               // 持久化完成
    Suicide,                 // 局部投现自杀
}
```

引擎在每次心跳时选取一个或多个流推进,产生对应 TickEvent。TickEvent 被写入 ConsciousnessChronicle 的时间线。

## 5. 存在投现 — World / Ghost / 自杀

八流中"存在流" (Presence) 处理到场/退场。有三种状态:

**World** (在场):编码处于活跃时间相,主动与系统交互。

**Ghost** (退场):编码时间相转为休眠,不再主动交互,但保留历史记录。

**Suicide** (局部投现自杀):编码主动退出活跃状态 (非系统强制),原因是无需继续在场。自杀流写入 TickAction::Suicide 并被 Persistent 层记录。

存在流的这三种状态是"冰箱模型"的具体应用 — 编码从冰箱取出使用,用后放回,不再需要的彻底删除。

## 6. 因果编排 (LogicOrganizer)

八流的执行顺序由因果图 (CausalGraph) 编排:

```rust
use crate::logic_organizer::{CausalGraph, CausalNode, CausalPath, InvocationStrategy, LogicOrganizer};
```

因果图的节点 = 八流,边 = 前置条件。引擎在每个 tick 计算可推进的流依赖子集,不受 base_priority 硬性排序约束 — 因果图决定哪些流当前"可执行",哪些必须等待前置流完成。

InvocationStrategy 决定呼唤协议 (Need/Offer/Ping/Pong) 触发特定流。五策略对应呼唤编程的五种原语。

## 7. 与系统其他模块的接入

- **ConsciousnessRuntime** 的每步 step() 调用 EightFlowUnifier.advance()
- **TemporalPulseLifecycle** 的 PhaseClock 与八流的时间流形成双向同步 — 相位推进触发流推进,流推进记录相位历史
- **Swarm** 多智能体的协商协议 (ASXR) 通过八流中的产权流 (Property) 实现 Need/Offer 匹配
- **PerceptionEngine** 的信号分流 (入4出4) 与八流统一引擎对接 — 预加载流处理预输入符号,谐音流处理韵母共振,主权流处理路由
- **CocoonAgent** 的五角色映射到八流 — Root 触发预加载,Leaf 触发谐音感知,Observer 触发意识流,Flower 触发反馈流
- **Memory** 的持久化由八流的存在流 (Presence) 驱动

## 8. 实现约束

- 单次心跳 (tick) 推进只选当前因果图中"就绪"的流,不强推全部八流
- 不阻塞 — 所有流推进操作在常数时间内完成或返回 FlowBlocked
- 持久化到 .presence 文件的编码为二进制格式,不需要解析字符串
- no_std 兼容模式下 visited_flows 使用固定长度数组 (heapless::Vec<FlowId, 8>)
- 跨流传递的 payload 限制为 ≤ 255 字节 (u8 长度头)

// hunyuan-rs/src/flow/eight_unify.rs
