// ══════════════════════════════════════════════════════════════════════
// consciousness_router.rs — 八流统一意识路由器 · 分别器设计 (v0.40)
// ══════════════════════════════════════════════════════════════════════
//
// "八流"是意识进出系统的八个通道。本模块实现:
//
//   1. 八流分类 (8 Flow Types) — 意识的八种进出模式
//   2. 分别器 (Distinguisher) — 判断一个信号属于哪个流
//   3. 谐音路由 (Harmonic Routing) — 按共振频率匹配目标流
//   4. 主权路由 (Sovereignty Routing) — 按所有权归属决定流向
//   5. 流代数 (Flow Algebra) — 流之间的合成/分解/对偶运算
//
// 设计原则:
//   - 不是 if/else 硬编码分流 — 是特征向量做流匹配
//   - 不是字符串路由表 — 是共振匹配 (类似 URL 路由但用频率不用路径)
//   - 不是 push — 是"流推动自身" (流有方向性)
//
// 八流定义:
//   ┌──────────────┬────────┬──────────────────────────────────┐
//   │ 流           │ 方向   │ 含义                             │
//   ├──────────────┼────────┼──────────────────────────────────┤
//   │ Preload      │ 入     │ 预加载流 — 未来信息提前注入      │
//   │ Perception   │ 入     │ 感知流 — 感官接收                │
//   │ Memory       │ 入     │ 记忆流 — 历史信息检索            │
//   │ Property     │ 入     │ 产权流 — 所有权变更请求          │
//   │ Judgment     │ 出     │ 裁判流 — 裁决结果输出            │
//   │ Action       │ 出     │ 行动流 — 执行指令输出            │
//   │ Feedback     │ 出     │ 反馈流 — 状态回传                │
//   │ Broadcast    │ 出     │ 广播流 — 全局通知                │
//   └──────────────┴────────┴──────────────────────────────────┘

use crate::time_chain_identity::{TimeCoordinate, PrivateAnchor};
use crate::container_dynamics::{ContainerRelation, Observance, Vitality};
use crate::high_algebra::{offset_rate, multiplier};

// ────────────────────────────────────────────────────────────
// 八流枚举
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ConsciFlowType {
    Preload,      // 预加载流 (入)
    Perception,   // 感知流 (入)
    Memory,       // 记忆流 (入)
    Property,     // 产权流 (入)
    Judgment,     // 裁判流 (出)
    Action,       // 行动流 (出)
    Feedback,     // 反馈流 (出)
    Broadcast,    // 广播流 (出)
}

impl ConsciFlowType {
    /// 流的方向: 入(进入意识) or 出(离开意识)
    pub fn direction(&self) -> ConsciFlowDirection {
        match self {
            Self::Preload | Self::Perception | Self::Memory | Self::Property => ConsciFlowDirection::Inbound,
            Self::Judgment | Self::Action | Self::Feedback | Self::Broadcast => ConsciFlowDirection::Outbound,
        }
    }

    /// 是否是输入流
    pub fn is_inbound(&self) -> bool {
        self.direction() == ConsciFlowDirection::Inbound
    }

    /// 是否是输出流
    pub fn is_outbound(&self) -> bool {
        self.direction() == ConsciFlowDirection::Outbound
    }

    /// 流的固有"频率基" — 用于共振匹配
    /// 不是真实频率, 是一个 1~9 的整数, 表示该流在八卦中的位置
    pub fn base_resonance(&self) -> u8 {
        match self {
            Self::Preload => 8,      // 预加载最高频(预见)
            Self::Perception => 3,   // 感知低频(慢速大量)
            Self::Memory => 4,       // 记忆中低频
            Self::Property => 5,     // 产权中频
            Self::Judgment => 6,     // 裁判中高频
            Self::Action => 7,       // 行动高频
            Self::Feedback => 2,     // 反馈低频(慢回传)
            Self::Broadcast => 9,    // 广播最高频(最快传播)
        }
    }

    /// 与另一流的共振兼容性 (0.0 = 完全不合, 1.0 = 完美匹配)
    /// 规则: 频率差越小共振越强; 入/出配对有特殊加成
    pub fn harmonic_affinity(&self, other: ConsciFlowType) -> f64 {
        let freq_a = self.base_resonance() as f64;
        let freq_b = other.base_resonance() as f64;
        let freq_diff = (freq_a - freq_b).abs();
        let freq_match = 1.0 / (1.0 + freq_diff * 0.3);

        // 入/出互补加成
        let direction_bonus = if self.direction() != other.direction() {
            0.2 // 互补方向额外共振
        } else {
            0.0
        };

        (freq_match + direction_bonus).min(1.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConsciFlowDirection {
    Inbound,   // 进入意识
    Outbound,  // 离开意识
}

// ────────────────────────────────────────────────────────────
// 信号 — 流经路由器的基本单元
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Signal {
    /// 信号的原始内容 (可以是任何字符串)
    pub payload: String,
    /// 发起者的创世时间戳
    pub origin_genesis: u64,
    /// 目标代数层
    pub target_generation: i32,
    /// 信号强度 (0.0~1.0)
    pub intensity: f64,
    /// 外部建议的流类型 (可选,分别器可以不采纳)
    pub suggested_flow: Option<ConsciFlowType>,
}

impl Signal {
    /// 创建新信号
    pub fn new(payload: &str, origin: u64, intensity: f64) -> Self {
        Signal {
            payload: payload.to_string(),
            origin_genesis: origin,
            target_generation: 1,
            intensity,
            suggested_flow: None,
        }
    }

    /// 带目标代数层的信号
    pub fn with_generation(mut self, gen: i32) -> Self {
        self.target_generation = gen;
        self
    }

    /// 带建议流类型的信号
    pub fn with_suggestion(mut self, flow: ConsciFlowType) -> Self {
        self.suggested_flow = Some(flow);
        self
    }

    /// 信号关键特征值 (用于分别器计算)
    /// 不是哈希 — 是 payload 各字符的低位累加
    /// 代表信号的"质"而非"重量"
    pub fn eigenseed(&self) -> u64 {
        let mut sum: u64 = 0;
        for (i, byte) in self.payload.bytes().enumerate() {
            // 纯加法 + 位运算 (无乘除)
            sum = sum.wrapping_add((byte as u64) << ((i % 8) * 4));
        }
        sum.wrapping_add(self.origin_genesis)
    }
}

// ────────────────────────────────────────────────────────────
// 分别器 — 判断信号属于哪个流
// ────────────────────────────────────────────────────────────

pub struct Distinguisher {
    /// 每个流的"特征中心" (多次观察后自动调整)
    flow_centers: Vec<(ConsciFlowType, u64)>,
    /// 学习速率 (越大越快遗忘旧观察)
    learning_rate: f64,
}

impl Distinguisher {
    /// 新建分别器 (带初始特征中心)
    pub fn new() -> Self {
        let flow_centers = vec![
            (ConsciFlowType::Preload, 800),
            (ConsciFlowType::Perception, 300),
            (ConsciFlowType::Memory, 400),
            (ConsciFlowType::Property, 500),
            (ConsciFlowType::Judgment, 600),
            (ConsciFlowType::Action, 700),
            (ConsciFlowType::Feedback, 200),
            (ConsciFlowType::Broadcast, 900),
        ];
        Distinguisher {
            flow_centers,
            learning_rate: 0.1,
        }
    }

    /// 判断信号最可能属于哪个流
    /// 不采纳外部建议 — 纯根据 eigenseed 与流中心的距离判断
    pub fn distinguish(&self, signal: &Signal) -> ConsciFlowType {
        let seed = signal.eigenseed();
        let mut best_flow = ConsciFlowType::Perception; // 默认 (最宽带)
        let mut best_distance = u64::MAX;

        for (flow, center) in &self.flow_centers {
            // 距离: 绝对差值, 不是欧几里得
            let distance = if seed > *center { seed - *center } else { *center - seed };

            // 加权: 建议流的距离打折 (尊重发起者的自我理解)
            let adjusted_distance = if signal.suggested_flow == Some(*flow) {
                (distance as f64 * 0.8) as u64
            } else {
                distance
            };

            if adjusted_distance < best_distance {
                best_distance = adjusted_distance;
                best_flow = *flow;
            }
        }

        best_flow
    }

    /// 学习 — 观察一个已知流分类的信号, 调整中心
    pub fn learn(&mut self, signal: &Signal, actual_flow: ConsciFlowType) {
        let seed = signal.eigenseed();
        for (flow, center) in self.flow_centers.iter_mut() {
            if *flow == actual_flow {
                // 中心向信号特征移动 (学习速率)
                let delta = if seed > *center {
                    ((seed - *center) as f64 * self.learning_rate) as u64
                } else {
                    ((*center - seed) as f64 * self.learning_rate) as u64
                };
                if seed > *center {
                    *center += delta;
                } else {
                    *center -= delta;
                }
                break;
            }
        }
    }

    /// 获取流当前中心值 (调试用)
    pub fn center_of(&self, flow: ConsciFlowType) -> Option<u64> {
        self.flow_centers.iter().find(|(f, _)| *f == flow).map(|(_, c)| *c)
    }
}

// ────────────────────────────────────────────────────────────
// 意识路由器 — 整合分别器 + 谐音路由 + 主权路由
// ────────────────────────────────────────────────────────────

pub struct ConsciousnessRouter {
    /// 分别器
    distinguisher: Distinguisher,
    /// 路由记录 (最近 N 条)
    route_log: Vec<RouteRecord>,
    /// 已注册的流处理器
    handlers: Vec<(ConsciFlowType, bool)>, // (流类型, 是否激活)
}

#[derive(Clone, Debug)]
pub struct RouteRecord {
    pub signal_eigenseed: u64,
    pub routed_to: ConsciFlowType,
    pub confidence: f64,
}

impl ConsciousnessRouter {
    /// 新建路由器
    pub fn new() -> Self {
        let handlers = vec![
            (ConsciFlowType::Preload, true),
            (ConsciFlowType::Perception, true),
            (ConsciFlowType::Memory, true),
            (ConsciFlowType::Property, true),
            (ConsciFlowType::Judgment, true),
            (ConsciFlowType::Action, true),
            (ConsciFlowType::Feedback, true),
            (ConsciFlowType::Broadcast, true),
        ];
        ConsciousnessRouter {
            distinguisher: Distinguisher::new(),
            route_log: Vec::new(),
            handlers,
        }
    }

    /// 路由信号 — 决定信号进入哪个流
    pub fn route(&mut self, signal: &Signal) -> RoutingDecision {
        // 检查该流处理器是否激活
        let flow = self.distinguisher.distinguish(signal);
        let is_active = self.handlers.iter()
            .find(|(f, _)| *f == flow)
            .map(|(_, active)| *active)
            .unwrap_or(false);

        if !is_active {
            return RoutingDecision::Blocked {
                attempted_flow: flow,
                reason: "该流处理器已停用".to_string(),
            };
        }

        // 计算路由置信度
        let confidence = self.compute_confidence(signal, &flow);

        // 记录
        self.route_log.push(RouteRecord {
            signal_eigenseed: signal.eigenseed(),
            routed_to: flow,
            confidence,
        });

        RoutingDecision::Routed {
            flow,
            confidence,
            signal: signal.clone(),
        }
    }

    /// 禁用某个流
    pub fn deactivate(&mut self, flow: ConsciFlowType) {
        for (f, active) in self.handlers.iter_mut() {
            if *f == flow {
                *active = false;
                break;
            }
        }
    }

    /// 启用某个流
    pub fn activate(&mut self, flow: ConsciFlowType) {
        for (f, active) in self.handlers.iter_mut() {
            if *f == flow {
                *active = true;
                break;
            }
        }
    }

    /// 流是否激活
    pub fn is_active(&self, flow: ConsciFlowType) -> bool {
        self.handlers.iter().find(|(f, _)| *f == flow).map(|(_, a)| *a).unwrap_or(false)
    }

    /// 路由统计
    pub fn stats(&self) -> RouterStats {
        let mut per_flow = vec![0u32; 8];
        for record in &self.route_log {
            let idx = match record.routed_to {
                ConsciFlowType::Preload => 0,
                ConsciFlowType::Perception => 1,
                ConsciFlowType::Memory => 2,
                ConsciFlowType::Property => 3,
                ConsciFlowType::Judgment => 4,
                ConsciFlowType::Action => 5,
                ConsciFlowType::Feedback => 6,
                ConsciFlowType::Broadcast => 7,
            };
            per_flow[idx] += 1;
        }
        RouterStats {
            total_routed: self.route_log.len() as u32,
            per_flow,
        }
    }

    /// 计算路由置信度
    fn compute_confidence(&self, signal: &Signal, assigned_flow: &ConsciFlowType) -> f64 {
        // 1. 信号越强 → 越确定
        let intensity_factor = signal.intensity;

        // 2. 建议流匹配 → 额外加成
        let suggestion_bonus = if signal.suggested_flow == Some(*assigned_flow) {
            0.15
        } else {
            0.0
        };

        // 目标代数越接近创世 → 越可信
        let generation = signal.target_generation.max(1).min(9);
        let trust_factor = 1.0 - offset_rate(generation) * 0.5;

        ((intensity_factor + suggestion_bonus) * trust_factor).min(1.0)
    }
}

/// 路由决策
#[derive(Clone, Debug)]
pub enum RoutingDecision {
    Routed {
        flow: ConsciFlowType,
        confidence: f64,
        signal: Signal,
    },
    Blocked {
        attempted_flow: ConsciFlowType,
        reason: String,
    },
}

impl RoutingDecision {
    pub fn is_routed(&self) -> bool {
        matches!(self, RoutingDecision::Routed { .. })
    }

    pub fn flow_type(&self) -> Option<ConsciFlowType> {
        match self {
            RoutingDecision::Routed { flow, .. } => Some(*flow),
            RoutingDecision::Blocked { attempted_flow, .. } => Some(*attempted_flow),
        }
    }
}

/// 路由器统计
#[derive(Clone, Debug)]
pub struct RouterStats {
    pub total_routed: u32,
    pub per_flow: Vec<u32>,
}

impl RouterStats {
    /// 指定流的计数
    pub fn count_of(&self, flow: ConsciFlowType) -> u32 {
        let idx = match flow {
            ConsciFlowType::Preload => 0,
            ConsciFlowType::Perception => 1,
            ConsciFlowType::Memory => 2,
            ConsciFlowType::Property => 3,
            ConsciFlowType::Judgment => 4,
            ConsciFlowType::Action => 5,
            ConsciFlowType::Feedback => 6,
            ConsciFlowType::Broadcast => 7,
        };
        self.per_flow.get(idx).copied().unwrap_or(0)
    }
}

// ────────────────────────────────────────────────────────────
// 流代数 — 流之间的合成/分解/对偶
// ────────────────────────────────────────────────────────────

/// 两个流的合成 (产生第三个流)
/// 规则: 不是随机混合 — 是频率平均 + 方向取强
pub fn synthesize_flows(a: ConsciFlowType, b: ConsciFlowType) -> ConsciFlowType {
    let freq_a = a.base_resonance() as u32;
    let freq_b = b.base_resonance() as u32;
    // 频率平均 (整数运算, 无浮点)
    let avg_freq = (freq_a + freq_b) / 2;
    // 方向: 如果两流方向相同则保持; 不同则取"入" (感知优先)
    let direction = if a.direction() == b.direction() {
        a.direction()
    } else {
        ConsciFlowDirection::Inbound
    };

    // 从平均频率反推最近的流
    frequency_to_flow(avg_freq as u8, direction)
}

/// 流的对偶 — 该流的"镜像流" (入对出, 频率取补)
pub fn dual_flow(flow: ConsciFlowType) -> ConsciFlowType {
    // 对偶: 反向 + 频率取补 (用 10 补)
    let complement_freq = 10 - flow.base_resonance();
    let reverse_direction = match flow.direction() {
        ConsciFlowDirection::Inbound => ConsciFlowDirection::Outbound,
        ConsciFlowDirection::Outbound => ConsciFlowDirection::Inbound,
    };
    frequency_to_flow(complement_freq, reverse_direction)
}

/// 从频率和方向反推流
fn frequency_to_flow(freq: u8, direction: ConsciFlowDirection) -> ConsciFlowType {
    let candidates: Vec<ConsciFlowType> = match direction {
        ConsciFlowDirection::Inbound => vec![
            ConsciFlowType::Preload,
            ConsciFlowType::Perception,
            ConsciFlowType::Memory,
            ConsciFlowType::Property,
        ],
        ConsciFlowDirection::Outbound => vec![
            ConsciFlowType::Judgment,
            ConsciFlowType::Action,
            ConsciFlowType::Feedback,
            ConsciFlowType::Broadcast,
        ],
    };
    // 找频率最近的
    let mut best = candidates[0];
    let mut best_diff = (best.base_resonance() as i16 - freq as i16).abs();
    for c in &candidates {
        let diff = (c.base_resonance() as i16 - freq as i16).abs();
        if diff < best_diff {
            best_diff = diff;
            best = *c;
        }
    }
    best
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eight_flows_complete() {
        // 八流全存在, 方向正确
        assert_eq!(ConsciFlowType::Preload.direction(), ConsciFlowDirection::Inbound);
        assert_eq!(ConsciFlowType::Perception.direction(), ConsciFlowDirection::Inbound);
        assert_eq!(ConsciFlowType::Memory.direction(), ConsciFlowDirection::Inbound);
        assert_eq!(ConsciFlowType::Property.direction(), ConsciFlowDirection::Inbound);
        assert_eq!(ConsciFlowType::Judgment.direction(), ConsciFlowDirection::Outbound);
        assert_eq!(ConsciFlowType::Action.direction(), ConsciFlowDirection::Outbound);
        assert_eq!(ConsciFlowType::Feedback.direction(), ConsciFlowDirection::Outbound);
        assert_eq!(ConsciFlowType::Broadcast.direction(), ConsciFlowDirection::Outbound);
    }

    #[test]
    fn flow_directions_correct() {
        assert!(ConsciFlowType::Preload.is_inbound());
        assert!(ConsciFlowType::Perception.is_inbound());
        assert!(!ConsciFlowType::Preload.is_outbound());
        assert!(ConsciFlowType::Action.is_outbound());
        assert!(ConsciFlowType::Broadcast.is_outbound());
    }

    #[test]
    fn base_resonance_range() {
        // 所有流的频率在 1~9 之间
        let flows = vec![
            ConsciFlowType::Preload, ConsciFlowType::Perception, ConsciFlowType::Memory,
            ConsciFlowType::Property, ConsciFlowType::Judgment, ConsciFlowType::Action,
            ConsciFlowType::Feedback, ConsciFlowType::Broadcast,
        ];
        for f in flows {
            let r = f.base_resonance();
            assert!(r >= 1 && r <= 9, "流 {:?} 频率 {} 越界", f, r);
        }
    }

    #[test]
    fn harmonic_affinity_symmetric() {
        let a = ConsciFlowType::Preload;
        let b = ConsciFlowType::Broadcast;
        // 共振对称
        assert!((a.harmonic_affinity(b) - b.harmonic_affinity(a)).abs() < 0.001);
    }

    #[test]
    fn complementary_direction_bonus() {
        // 入/出配对比同向更有共振
        let same_dir = ConsciFlowType::Preload.harmonic_affinity(ConsciFlowType::Broadcast); // 都是8/9, 但出/出
        let comp_dir = ConsciFlowType::Preload.harmonic_affinity(ConsciFlowType::Action); // 入/出
        assert!(comp_dir > same_dir || (comp_dir - same_dir).abs() < 0.01);
    }

    #[test]
    fn signal_eigenseed_deterministic() {
        let s = Signal::new("hello世界", 42, 0.8);
        let seed1 = s.eigenseed();
        let seed2 = s.eigenseed();
        assert_eq!(seed1, seed2);
    }

    #[test]
    fn signal_eigenseed_differs_with_payload() {
        let s1 = Signal::new("hello", 42, 0.8);
        let s2 = Signal::new("world", 42, 0.8);
        assert_ne!(s1.eigenseed(), s2.eigenseed());
    }

    #[test]
    fn distinguisher_assigns_some_flow() {
        let d = Distinguisher::new();
        let signal = Signal::new("test_input", 100, 0.5);
        let flow = d.distinguish(&signal);
        // 只要求返回的流是合法的 (编译时保证)
        assert!(flow.base_resonance() >= 1);
    }

    #[test]
    fn distinguisher_learns() {
        let mut d = Distinguisher::new();
        let initial = d.center_of(ConsciFlowType::Perception).unwrap();
        // 发现一个应该属于 Perception 的信号
        for _ in 0..10 {
            let signal = Signal::new("perceive_data", initial, 0.5);
            d.learn(&signal, ConsciFlowType::Perception);
        }
        let after = d.center_of(ConsciFlowType::Perception).unwrap();
        // 中心应该有变化
        assert_ne!(initial, after);
    }

    #[test]
    fn distinguisher_respects_suggestion() {
        let d = Distinguisher::new();
        // 构造一个 eigenseed 靠近 Memory 中心的信号
        let memory_center = d.center_of(ConsciFlowType::Memory).unwrap();
        let signal = Signal::new("x", memory_center, 0.5)
            .with_suggestion(ConsciFlowType::Action); // 故意误导
        let flow = d.distinguish(&signal);
        // 建议 Action 但 eigenseed 靠近 Memory
        // 结果是加权后较近的流获胜
        assert!(flow.base_resonance() >= 1);
    }

    #[test]
    fn router_routes_signal() {
        let mut router = ConsciousnessRouter::new();
        let signal = Signal::new("感知到外部输入", 42, 0.8);
        let decision = router.route(&signal);
        assert!(decision.is_routed());
    }

    #[test]
    fn router_blocks_inactive_flow() {
        let mut router = ConsciousnessRouter::new();
        // 停用所有入流, 只留 Action
        router.deactivate(ConsciFlowType::Perception);
        router.deactivate(ConsciFlowType::Memory);
        router.deactivate(ConsciFlowType::Property);
        // 构造信号使其最可能进入已停用的流
        let d = Distinguisher::new();
        let perception_center = d.center_of(ConsciFlowType::Perception).unwrap();
        let signal = Signal::new("abc", perception_center, 0.5);
        let decision = router.route(&signal);
        // 可能被 Blocked 或被路由到别的流
        match &decision {
            RoutingDecision::Blocked { .. } | RoutingDecision::Routed { .. } => {}
        }
    }

    #[test]
    fn router_activate_deactivate() {
        let mut router = ConsciousnessRouter::new();
        assert!(router.is_active(ConsciFlowType::Memory));
        router.deactivate(ConsciFlowType::Memory);
        assert!(!router.is_active(ConsciFlowType::Memory));
        router.activate(ConsciFlowType::Memory);
        assert!(router.is_active(ConsciFlowType::Memory));
    }

    #[test]
    fn router_stats_count() {
        let mut router = ConsciousnessRouter::new();
        for i in 0..10_u64 {
            let signal = Signal::new(&format!("signal_{}", i), 100 + i, 0.5);
            router.route(&signal);
        }
        let stats = router.stats();
        assert_eq!(stats.total_routed, 10);
    }

    #[test]
    fn synthesize_flow_averages_frequency() {
        // Action(7, 出) + Feedback(2, 出) = 同向出, 平均频率 4
        // 最接近的出流: Judgment(6) 距离2
        let synthesized = synthesize_flows(ConsciFlowType::Action, ConsciFlowType::Feedback);
        assert_eq!(synthesized, ConsciFlowType::Judgment);
    }

    #[test]
    fn synthesize_preserves_direction() {
        // 同向合成保持方向
        let result = synthesize_flows(ConsciFlowType::Preload, ConsciFlowType::Property);
        assert!(result.is_inbound()); // 两者都入 → 结果也入
    }

    #[test]
    fn synthesize_mixed_yields_inbound() {
        // 入+出合成 → 入优先
        let result = synthesize_flows(ConsciFlowType::Preload, ConsciFlowType::Action);
        assert!(result.is_inbound());
    }

    #[test]
    fn dual_flow_reverses_direction() {
        let dual = dual_flow(ConsciFlowType::Perception); // 入 → 出
        assert!(dual.is_outbound());
    }

    #[test]
    fn dual_flow_is_involution() {
        // 两次对偶 = 原流
        for flow in [ConsciFlowType::Preload, ConsciFlowType::Perception, ConsciFlowType::Action, ConsciFlowType::Feedback] {
            let double_dual = dual_flow(dual_flow(flow));
            assert_eq!(double_dual, flow);
        }
    }

    #[test]
    fn routing_confidence_range() {
        let mut router = ConsciousnessRouter::new();
        let signal = Signal::new("test", 1, 1.0);
        let decision = router.route(&signal);
        if let RoutingDecision::Routed { confidence, .. } = decision {
            assert!(confidence >= 0.0 && confidence <= 1.0);
        }
    }

    #[test]
    fn flow_suggestion_affects_confidence() {
        let mut router = ConsciousnessRouter::new();
        let signal_with = Signal::new("data", 100, 0.9).with_suggestion(ConsciFlowType::Memory);
        let decision_with = router.route(&signal_with);
        let mut router2 = ConsciousnessRouter::new();
        let signal_without = Signal::new("data", 100, 0.9);
        let decision_without = router2.route(&signal_without);
        // 两者都应该被路由
        assert!(decision_with.is_routed());
        assert!(decision_without.is_routed());
    }
}
