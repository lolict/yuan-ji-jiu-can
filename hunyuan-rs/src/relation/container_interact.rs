// ══════════════════════════════════════════════════════════════════════
// container_interact.rs — 容器交互协议引擎 · 观察者路由·Need/Offer匹配·公约数共振 (v0.47)
// ══════════════════════════════════════════════════════════════════════
//
//   「容器不互相调用 — 容器被共振频率吸引到彼此」
//
//   容器交互协议引擎是容器关系的"协议栈", 实现三个核心协议:
//
//     1. 观察者路由 (Observer Routing)
//        呼唤发出时, 不应广播给所有容器 — 应按"观察方向"路由
//        观察者 = 当下注意力所在的容器
//        被观察者 = 被注视的容器
//        路由结果 = 呼唤送往正确的观察链路
//
//     2. Need/Offer 匹配 (Need-Offer Matching)
//        容器 A 呼唤 Need{resource: X}
//        容器 B 呼唤 Offer{resource: X}
//        匹配不是按名称搜索 — 按公约数共振/语义签名匹配
//
//     3. 公约数共振 (Common-Divisor Resonance)
//        两个容器的签名 GCD > 1 → 构成"共震群"
//        匹配成功的容器对需要落入同一共振群才能交互
//
//   设计约束:
//     - 不搜索容器 — 按共振频率直接吸引
//     - 不中断交互 — 非阻塞协议: pending 呼唤排队
//     - 容器身份靠时间链/签名, 不靠 ID 指针
//

use crate::container_dynamics::Call;
use crate::consciousness_router::ConsciFlowType;
use crate::resonance_discovery::{gcd_additive, is_resonant};
use crate::logic_organizer::{CausalPath, InvocationStrategy};

// ────────────────────────────────────────────────────────────
// 容器端口 — 一个容器的交互端点
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct ContainerPort {
    /// 容器时间链签名 (公约数计算基底)
    pub signature: u64,
    /// 容器名称 (在场标识, 非代理)
    pub name: String,
    /// 当前未决呼唤
    pub pending_calls: Vec<Call>,
    /// 是否活跃 (可接受交互)
    pub active: bool,
}

impl ContainerPort {
    pub fn new(signature: u64, name: &str) -> Self {
        Self {
            signature,
            name: name.to_string(),
            pending_calls: Vec::new(),
            active: true,
        }
    }

    /// 发出呼唤
    pub fn call(&mut self, call: Call) {
        self.pending_calls.push(call);
    }

    /// 是否仍有未决呼唤
    pub fn has_pending(&self) -> bool {
        !self.pending_calls.is_empty()
    }

    /// 取出所有未决呼唤 (清空排队)
    pub fn drain_calls(&mut self) -> Vec<Call> {
        std::mem::take(&mut self.pending_calls)
    }

    /// 与另一容器的共振强度
    pub fn resonance_with(&self, other: &ContainerPort) -> u64 {
        gcd_additive(self.signature, other.signature)
    }

    /// 是否与另一容器共振
    pub fn is_resonant_with(&self, other: &ContainerPort) -> bool {
        self.signature != other.signature && is_resonant(self.signature, other.signature, 1)
    }
}

// ────────────────────────────────────────────────────────────
// 交互匹配 — Need 与 Offer 的配对结果
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct InteractionMatch {
    /// 需要者签名
    pub needer_sig: u64,
    /// 提供者签名
    pub offerer_sig: u64,
    /// 共振强度 (GCD)
    pub resonance: u64,
    /// 匹配的呼唤资源描述
    pub resource: String,
    /// 紧迫度 (来自 Need 的 urgency)
    pub urgency: u8,
}

impl InteractionMatch {
    pub fn new(needer_sig: u64, offerer_sig: u64, resonance: u64, resource: &str, urgency: u8) -> Self {
        Self {
            needer_sig,
            offerer_sig,
            resonance,
            resource: resource.to_string(),
            urgency,
        }
    }

    /// 匹配是否有效 (共振 + 紧迫度足够)
    pub fn is_valid(&self) -> bool {
        self.resonance > 1 && self.urgency > 0
    }

    /// 匹配质量评分 (共振 × 紧迫度, 纯加法)
    pub fn quality(&self) -> u64 {
        self.resonance.saturating_add(self.urgency as u64)
    }
}

// ────────────────────────────────────────────────────────────
// 观察者路由 — 决定呼唤去向哪个观察方向
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoutingDirection {
    /// 路由给感知观察当下容器
    ToPerceiver,
    /// 路由给被观察容器
    ToPerceived,
    /// 双向路由 (观察者+被观察者都给)
    Bidirectional,
    /// 路由给共鸣群所有人
    ToResonanceGroup,
}

#[derive(Clone, Debug)]
pub struct RouterEntry {
    /// 呼唤发起者
    pub caller_sig: u64,
    /// 路由方向
    pub direction: RoutingDirection,
    /// 目标八流类型
    pub flow_type: ConsciFlowType,
    /// 原始呼唤
    pub call: Call,
}

impl RouterEntry {
    pub fn route(&self) -> Vec<u64> {
        // 路由签名 = 呼唤紧迫度 × 流共振基底
        let urgency = self.call.urgency_level() as u64;
        let base = self.flow_type.base_resonance() as u64;
        // 纯加法签名: urgency 重复叠加 base 次
        let mut route_sig: u64 = 0;
        for _ in 0..urgency {
            route_sig = route_sig.wrapping_add(base);
        }

        // 路由目标列表: 方向决定包含哪些签名
        match self.direction {
            RoutingDirection::ToPerceiver => vec![route_sig.wrapping_add(self.caller_sig)],
            RoutingDirection::ToPerceived => vec![self.caller_sig.wrapping_add(route_sig)],
            RoutingDirection::Bidirectional => vec![
                route_sig.wrapping_add(self.caller_sig),
                self.caller_sig.wrapping_add(route_sig),
            ],
            RoutingDirection::ToResonanceGroup => {
                // 共振群 = caller_sig 的所有公约数配对
                vec![route_sig] // 入口签名, 由引擎展开
            }
        }
    }
}

// ────────────────────────────────────────────────────────────
// 容器交互引擎 — 统一协议处理
// ────────────────────────────────────────────────────────────

pub struct InteractionEngine {
    /// 所有已注册容器端口
    pub ports: Vec<ContainerPort>,
    /// 路由日志
    pub route_log: Vec<RouterEntry>,
    /// 匹配结果缓存
    pub matches: Vec<InteractionMatch>,
    /// 匹配策略
    pub strategy: InvocationStrategy,
}

impl InteractionEngine {
    pub fn new(strategy: InvocationStrategy) -> Self {
        Self {
            ports: Vec::new(),
            route_log: Vec::new(),
            matches: Vec::new(),
            strategy,
        }
    }

    /// 注册容器端口
    pub fn register(&mut self, port: ContainerPort) {
        self.ports.push(port);
    }

    pub fn port_count(&self) -> usize {
        self.ports.len()
    }

    /// 路由呼唤 — 确定方向并记录
    pub fn route(&mut self, caller_sig: u64, call: Call, direction: RoutingDirection) {
        // 根据呼唤类型确定八流归属
        let flow_type = match &call {
            Call::Need { .. } => ConsciFlowType::Perception,
            Call::Offer { .. } => ConsciFlowType::Property,
            Call::Ping | Call::Pong { .. } => ConsciFlowType::Feedback,
            Call::Arbitrate { .. } => ConsciFlowType::Judgment,
        };

        self.route_log.push(RouterEntry {
            caller_sig,
            direction,
            flow_type,
            call,
        });
    }

    /// 执行 Need/Offer 匹配 — 扫描所有活跃容器的未决呼唤
    pub fn match_needs_offers(&mut self) -> Vec<InteractionMatch> {
        let mut new_matches = Vec::new();

        // 收集所有呼唤
        let all_calls: Vec<(usize, Call)> = self.ports.iter()
            .enumerate()
            .filter(|(_, p)| p.active)
            .flat_map(|(i, p)| {
                p.pending_calls.iter()
                    .map(move |c| (i, c.clone()))
            })
            .collect();

        // 分离 Needs 和 Offers
        let needs: Vec<(usize, String, u8)> = all_calls.iter()
            .filter_map(|(i, c)| match c {
                Call::Need { resource, urgency } => Some((*i, resource.clone(), *urgency)),
                _ => None,
            })
            .collect();

        let offers: Vec<(usize, String)> = all_calls.iter()
            .filter_map(|(i, c)| match c {
                Call::Offer { resource, .. } => Some((*i, resource.clone())),
                _ => None,
            })
            .collect();

        // 匹配: Need.resource == Offer.resource → 检查共振
        for (need_idx, need_res, urgency) in &needs {
            for (offer_idx, offer_res) in &offers {
                if need_res == offer_res && need_idx != offer_idx {
                    let needer = &self.ports[*need_idx];
                    let offerer = &self.ports[*offer_idx];
                    let resonance = gcd_additive(needer.signature, offerer.signature);
                    if resonance > 1 {
                        new_matches.push(InteractionMatch::new(
                            needer.signature,
                            offerer.signature,
                            resonance,
                            need_res,
                            *urgency,
                        ));
                    }
                }
            }
        }

        // 按策略排序
        match self.strategy {
            InvocationStrategy::MaxResonance | InvocationStrategy::FullyResonant => {
                // 按共振降序 (选择排序, 避免引入快速排序的复杂性)
                new_matches.sort_by(|a, b| b.resonance.cmp(&a.resonance));
            }
            _ => {
                // 按紧迫度降序
                new_matches.sort_by(|a, b| b.urgency.cmp(&a.urgency));
            }
        }

        self.matches.extend(new_matches.clone());
        new_matches
    }

    /// 共振群发现 — 从指定容器出发找到所有共振成员
    pub fn resonance_group(&self, start_sig: u64) -> Vec<u64> {
        let mut group = vec![start_sig];
        for port in &self.ports {
            if port.signature != start_sig
                && is_resonant(start_sig, port.signature, 1)
                && port.active
            {
                group.push(port.signature);
            }
        }
        group
    }

    /// 共振路径发现 — 从容器 A 到容器 B 的共振链路
    pub fn find_resonance_path(&self, from_sig: u64, to_sig: u64) -> Option<CausalPath> {
        // 如果直接共振, 直接返回
        if is_resonant(from_sig, to_sig, 1) {
            let mut path = CausalPath::new();
            // 使用 entry_id 作为签名占位
            use crate::logic_organizer::CausalNode;
            path.push(CausalNode::new(from_sig as u32, from_sig));
            path.push(CausalNode::new(to_sig as u32, to_sig));
            return Some(path);
        }

        // 寻找中间共振容器: 与 from_sig 和 to_sig 都共振的第三容器
        for port in &self.ports {
            if port.signature != from_sig
                && port.signature != to_sig
                && port.active
                && is_resonant(from_sig, port.signature, 1)
                && is_resonant(port.signature, to_sig, 1)
            {
                let mut path = CausalPath::new();
                use crate::logic_organizer::CausalNode;
                path.push(CausalNode::new(from_sig as u32, from_sig));
                path.push(CausalNode::new(port.signature as u32, port.signature));
                path.push(CausalNode::new(to_sig as u32, to_sig));
                return Some(path);
            }
        }

        None
    }

    /// 执行匹配结果 — 清空已匹配容器的对应呼唤
    pub fn execute_matches(&mut self) -> usize {
        let mut executed = 0;
        for m in &self.matches {
            for port in self.ports.iter_mut() {
                if port.signature == m.needer_sig || port.signature == m.offerer_sig {
                    let before = port.pending_calls.len();
                    port.pending_calls.retain(|c| {
                        let resource = match c {
                            Call::Need { resource, .. } => resource.clone(),
                            Call::Offer { resource, .. } => resource.clone(),
                            _ => String::new(),
                        };
                        // 移除已匹配的资源呼唤
                        resource != m.resource
                    });
                    executed += before - port.pending_calls.len();
                }
            }
        }
        self.matches.clear();
        executed
    }

    /// 统计
    pub fn stats(&self) -> InteractionStats {
        let active = self.ports.iter().filter(|p| p.active).count();
        let total_pending: usize = self.ports.iter().map(|p| p.pending_calls.len()).sum();

        InteractionStats {
            port_count: self.ports.len(),
            active_ports: active,
            pending_calls: total_pending,
            matches_found: self.matches.len(),
            routes_logged: self.route_log.len(),
        }
    }
}

impl Default for InteractionEngine {
    fn default() -> Self {
        Self::new(InvocationStrategy::MaxResonance)
    }
}

// ────────────────────────────────────────────────────────────
// 统计
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InteractionStats {
    pub port_count: usize,
    pub active_ports: usize,
    pub pending_calls: usize,
    pub matches_found: usize,
    pub routes_logged: usize,
}

// ────────────────────────────────────────────────────────────
// 便捷函数
// ────────────────────────────────────────────────────────────

/// 快速组建交互引擎 (从签名+名称列表)
pub fn build_engine(signatures: &[(u64, &str)], strategy: InvocationStrategy) -> InteractionEngine {
    let mut engine = InteractionEngine::new(strategy);
    for &(sig, name) in signatures {
        engine.register(ContainerPort::new(sig, name));
    }
    engine
}

/// 对一组容器执行一次完整匹配-执行周期
pub fn run_cycle(engine: &mut InteractionEngine) -> (Vec<InteractionMatch>, usize) {
    let matches = engine.match_needs_offers();
    let executed = engine.execute_matches();
    (matches, executed)
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_port_resonance() {
        let a = ContainerPort::new(6, "A");   // 6 = 2×3
        let b = ContainerPort::new(9, "B");   // 9 = 3×3 → GCD = 3
        let c = ContainerPort::new(7, "C");   // 7 是质数 → 与 6 无共振

        assert!(a.is_resonant_with(&b));
        assert!(!a.is_resonant_with(&c));
        assert_eq!(a.resonance_with(&b), 3);
    }

    #[test]
    fn test_interaction_engine_basic() {
        let mut engine = InteractionEngine::new(InvocationStrategy::MaxResonance);
        engine.register(ContainerPort::new(6, "A"));
        engine.register(ContainerPort::new(9, "B"));

        assert_eq!(engine.port_count(), 2);
    }

    #[test]
    fn test_match_needs_offers() {
        let mut engine = InteractionEngine::new(InvocationStrategy::MaxResonance);

        let mut port_a = ContainerPort::new(6, "A_need");
        port_a.call(Call::Need {
            resource: "glyph_codec".to_string(),
            urgency: 8,
        });

        let mut port_b = ContainerPort::new(9, "B_offer");
        port_b.call(Call::Offer {
            resource: "glyph_codec".to_string(),
            quantity: 1,
        });

        // C 提供不同的资源, 不应匹配
        let mut port_c = ContainerPort::new(12, "C_other");
        port_c.call(Call::Offer {
            resource: "something_else".to_string(),
            quantity: 5,
        });

        engine.register(port_a);
        engine.register(port_b);
        engine.register(port_c);

        let matches = engine.match_needs_offers();
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].resonance, 3); // GCD(6,9) = 3
        assert_eq!(matches[0].resource, "glyph_codec");
    }

    #[test]
    fn test_resonance_group() {
        let mut engine = InteractionEngine::new(InvocationStrategy::MaxResonance);
        // 6 与 9 共振, 6 与 12 共振, 9 与 12 共振
        engine.register(ContainerPort::new(6, "A"));
        engine.register(ContainerPort::new(9, "B"));
        engine.register(ContainerPort::new(12, "C"));
        engine.register(ContainerPort::new(7, "D"));  // 孤立

        let group = engine.resonance_group(6);
        assert!(group.contains(&6));
        assert!(group.contains(&9));
        assert!(group.contains(&12));
        assert!(!group.contains(&7));
    }

    #[test]
    fn test_route_need_to_perception() {
        let mut engine = InteractionEngine::new(InvocationStrategy::MaxResonance);
        engine.register(ContainerPort::new(100, "A"));

        engine.route(100, Call::Need {
            resource: "x".to_string(),
            urgency: 5,
        }, RoutingDirection::ToPerceiver);

        assert_eq!(engine.route_log.len(), 1);
        assert_eq!(engine.route_log[0].flow_type, ConsciFlowType::Perception);
    }

    #[test]
    fn test_execute_matches_clears_pending() {
        let mut engine = InteractionEngine::new(InvocationStrategy::MaxResonance);

        let mut port_a = ContainerPort::new(30, "A");
        port_a.call(Call::Need {
            resource: "r".to_string(),
            urgency: 5,
        });

        let mut port_b = ContainerPort::new(42, "B");
        port_b.call(Call::Offer {
            resource: "r".to_string(),
            quantity: 1,
        });

        engine.register(port_a);
        engine.register(port_b);

        let matches = engine.match_needs_offers();
        assert!(!matches.is_empty());

        let cleared = engine.execute_matches();
        assert!(cleared > 0);
    }

    #[test]
    fn test_no_match_when_not_resonant() {
        let mut engine = InteractionEngine::new(InvocationStrategy::MaxResonance);

        let mut port_a = ContainerPort::new(7, "A");   // 质数
        port_a.call(Call::Need {
            resource: "shared".to_string(),
            urgency: 9,
        });

        let mut port_b = ContainerPort::new(11, "B");  // 另一个质数
        port_b.call(Call::Offer {
            resource: "shared".to_string(),
            quantity: 1,
        });

        engine.register(port_a);
        engine.register(port_b);

        let matches = engine.match_needs_offers();
        assert!(matches.is_empty()); // 无共振 = 不匹配
    }

    #[test]
    fn test_run_cycle() {
        let mut engine = build_engine(&[
            (6, "A"),
            (9, "B"),
        ], InvocationStrategy::MaxResonance);

        // 手动添加呼唤
        engine.ports[0].call(Call::Need {
            resource: "x".to_string(),
            urgency: 3,
        });
        engine.ports[1].call(Call::Offer {
            resource: "x".to_string(),
            quantity: 1,
        });

        let (matches, executed) = run_cycle(&mut engine);
        assert!(!matches.is_empty());
        assert!(executed > 0);
    }

    #[test]
    fn test_stats() {
        let mut engine = InteractionEngine::new(InvocationStrategy::MaxResonance);
        engine.register(ContainerPort::new(10, "A"));
        engine.register(ContainerPort::new(20, "B"));
        engine.ports[1].active = false;

        let stats = engine.stats();
        assert_eq!(stats.port_count, 2);
        assert_eq!(stats.active_ports, 1);
    }

    #[test]
    fn test_match_validity() {
        let valid = InteractionMatch::new(6, 9, 3, "r", 5);
        assert!(valid.is_valid());

        let no_resonance = InteractionMatch::new(6, 7, 1, "r", 5);
        assert!(!no_resonance.is_valid());

        let no_urgency = InteractionMatch::new(6, 9, 3, "r", 0);
        assert!(!no_urgency.is_valid());
    }

    #[test]
    fn test_quality_score() {
        let a = InteractionMatch::new(6, 9, 3, "r", 5);
        let b = InteractionMatch::new(12, 18, 6, "r", 2);

        // a.quality = 3 + 5 = 8
        // b.quality = 6 + 2 = 8
        assert_eq!(a.quality(), 8);
        assert_eq!(b.quality(), 8);
    }

    #[test]
    fn test_bidirectional_route() {
        let mut engine = InteractionEngine::new(InvocationStrategy::MaxResonance);
        engine.register(ContainerPort::new(100, "A"));

        engine.route(100, Call::Ping, RoutingDirection::Bidirectional);
        assert_eq!(engine.stats().routes_logged, 1);
    }
}
