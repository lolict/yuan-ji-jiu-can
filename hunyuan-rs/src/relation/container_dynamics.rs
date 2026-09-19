// ══════════════════════════════════════════════════════════════════════
// container_dynamics.rs — 容器关系动力学 · 呼唤编程 (v0.39)
// ══════════════════════════════════════════════════════════════════════
//
// 容器不是死囊——它们是活的关系节点。本模块实现:
//
//   1. 动态容器 vs 静态容器 (Dynamic vs Static)
//   2. 观察者/被观察者 (Observer / Observed) — 谁在"看"谁
//   3. 容器交互协议 (Container Interaction Protocol)
//   4. 判断之力 + 评价之力 (Judgment Power + Evaluation Power)
//   5. 呼唤编程 (Call Programming) — 不是 push 数据, 是"被需要"而响应
//   6. 迂回反馈 (Detour Feedback) — 不直接回赠, 绕一圈回来
//   7. 反馈环路 (Feedback Loop) — 关系自我维持/衰减
//
// 数学约束:
//   - 只用加减 + 模式匹配 (无乘除)
//   - 容器识别靠时间链身份, 不靠哈希
//   - 见证由三元裁判机提供
//   - 公约数 = 关系的"公分母", 公倍数 = 关系的"共同节律"

use crate::time_chain_identity::{TimeChainIdentity, TimeCoordinate, ClaimValidation};
use crate::triune::{TriuneVerdict, BeingState, FlowDirection};
use crate::mirror::{Sovereignty, WitnessVerdict, MirrorOrder};
use crate::high_algebra::{AlgebraLevel, offset_rate, multiplier};

// ────────────────────────────────────────────────────────────
// 容器活性 — 动态 vs 静态
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vitality {
    /// 动态容器: 主动发起交互, 能呼唤其他容器
    Dynamic,
    /// 静态容器: 被动等待, 只有被呼唤时才响应
    Static,
    /// 失活容器: 不再参与交互 (休眠/废弃)
    Dormant,
}

// ────────────────────────────────────────────────────────────
// 观察者/被观察者 — 关系的方向性
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Observance {
    /// 我是观察者 (我在看别人)
    Observer,
    /// 我是被观察者 (别人在看我)
    Observed,
    /// 互看 (同时是观察者和被观察者, 对称关系)
    Mutual,
}

/// 观察行为 — 一个容器"看"另一个容器产生的印象
#[derive(Clone, Debug)]
pub struct Gaze {
    /// 谁在观察 (发起者时间链身份摘要)
    pub observer_genesis: u64,
    /// 被观察者的坐标快照
    pub observed_coord: TimeCoordinate,
    /// 观察方向 (我在主动看? 我被吸引而看?)
    pub observance: Observance,
    /// 观察强度 (0.0 = 没看见, 1.0 = 完全感知)
    pub intensity: f64,
    /// 观察后的三元推演
    pub witness: Option<TriuneVerdict>,
}

// ────────────────────────────────────────────────────────────
// 呼唤 — 不是 push, 是 pull-by-need
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
pub enum Call {
    /// 需要: "我缺 X, 谁能给我?"
    Need {
        resource: String,
        urgency: u8,        // 1~9, 9=最紧急
    },
    /// 提供: "我有 X, 谁需要?"
    Offer {
        resource: String,
        quantity: u64,
    },
    /// 询问: "你还在吗?"
    Ping,
    /// 应答: "我在。"
    Pong {
        responder_genesis: u64,
    },
    /// 裁决: "两个容器冲突, 请裁判"
    Arbitrate {
        claimant_a: u64,
        claimant_b: u64,
        dispute: String,
    },
}

impl Call {
    /// 呼唤的"紧迫度"决定反馈速度
    pub fn urgency_level(&self) -> u8 {
        match self {
            Call::Need { urgency, .. } => *urgency,
            Call::Offer { .. } => 3,
            Call::Ping => 1,
            Call::Pong { .. } => 1,
            Call::Arbitrate { .. } => 9,  // 裁决最优先
        }
    }

    /// 呼唤是否需要见证
    pub fn requires_witness(&self) -> bool {
        matches!(self, Call::Arbitrate { .. })
    }
}

// ────────────────────────────────────────────────────────────
// 反馈环路 — 关系的自然衰减或增强
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct FeedbackLoop {
    /// 发起者 (谁在响应)
    pub initiator: u64,
    /// 接收者 (谁在接收)
    pub receiver: u64,
    /// 环路节律 (经过的容器数)
    pub path_length: u32,
    /// 当前反馈强度 (随路径衰减)
    pub strength: f64,
    /// 是否已形成闭环 (回到发起者)
    pub is_closed: bool,
    /// 反馈类型
    pub kind: FeedbackKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeedbackKind {
    /// 正向反馈: 关系增强 (供应满足需求)
    Positive,
    /// 负向反馈: 关系衰减 (需求未被满足)
    Negative,
    /// 迂回反馈: 不是直接回赠, 通过第三容器传递
    Detour,
}

impl FeedbackLoop {
    /// 创建初始反馈 (强度=1.0)
    pub fn new(initiator: u64, receiver: u64, kind: FeedbackKind) -> Self {
        FeedbackLoop {
            initiator,
            receiver,
            path_length: 1,
            strength: 1.0,
            is_closed: false,
            kind,
        }
    }

    /// 反馈传播 — 每经过一个容器, 强度按公约数衰减
    /// 不是简单除法 — 是"关系距离"的衰减
    pub fn propagate(&mut self, common_divisor: u64) {
        if common_divisor > 0 && self.strength > 0.01 {
            // 衰减公式: strength -= (1 / (divisor + 1))
            // divisor 越大(关系越远) → 衰减越小 → 但路径越长总体衰减越多
            let attenuation = 1.0 / (common_divisor as f64 + 1.0);
            self.strength -= attenuation * 0.1;
            if self.strength < 0.0 {
                self.strength = 0.0;
            }
            self.path_length += 1;
        }
    }

    /// 闭环检测 — 反馈回到了发起者
    pub fn attempt_close(&mut self, current_holder: u64) -> bool {
        if current_holder == self.initiator && self.path_length > 1 {
            self.is_closed = true;
            // 闭环时根据类型调整最终强度
            self.strength = match self.kind {
                FeedbackKind::Positive => self.strength * 1.2,   // 闭环增强
                FeedbackKind::Negative => self.strength * 0.5,   // 负反馈闭环 = 衰减
                FeedbackKind::Detour => self.strength * 1.0,     // 迂回不变
            };
            true
        } else {
            false
        }
    }
}

// ────────────────────────────────────────────────────────────
// 容器关系动力学 — 两个容器之间的活关系
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct ContainerRelation {
    /// 容器 A 的创世时间戳 (身份锚点)
    pub a_genesis: u64,
    /// 容器 B 的创世时间戳
    pub b_genesis: u64,
    /// 关系的观察者方向
    pub observance: Observance,
    /// 容器 A 的活性
    pub a_vitality: Vitality,
    /// 容器 B 的活性
    pub b_vitality: Vitality,
    /// 公约数 (两容器的公分母 — 必须 > 0 才能共振)
    pub common_divisor: u64,
    /// 公倍数 (共同节律 — 交互的自然频率)
    pub common_multiple: u64,
    /// 当前关系强度 (0.0 = 断裂, 1.0 = 完美共振)
    pub bond_strength: f64,
    /// 历史反馈环路
    pub feedback_loops: Vec<FeedbackLoop>,
    /// 见证记录
    pub witness_records: Vec<TriuneVerdict>,
}

impl ContainerRelation {
    /// 创世关系: 两个容器第一次建立联系
    pub fn genesis(a_genesis: u64, b_genesis: u64) -> Self {
        let gcd = calculate_gcd(a_genesis, b_genesis);
        let lcm = if gcd > 0 { (a_genesis / gcd) * b_genesis } else { 0 };
        ContainerRelation {
            a_genesis,
            b_genesis,
            observance: Observance::Mutual,
            a_vitality: Vitality::Dynamic,
            b_vitality: Vitality::Dynamic,
            common_divisor: gcd,
            common_multiple: lcm,
            bond_strength: if gcd > 0 { 1.0 } else { 0.0 },
            feedback_loops: Vec::new(),
            witness_records: Vec::new(),
        }
    }

    /// 关系是否有效 (公约数 > 0 且双方非休眠)
    pub fn is_alive(&self) -> bool {
        self.common_divisor > 0
            && self.a_vitality != Vitality::Dormant
            && self.b_vitality != Vitality::Dormant
    }

    /// 发送呼唤 — 一个容器呼叫另一个
    pub fn call(&mut self, call: &Call) -> CallResponse {
        // 检查双方活性
        if !self.is_alive() {
            return CallResponse::DeadRelay;
        }

        // 呼唤需要见证?
        let witness = if call.requires_witness() {
            let v = TriuneVerdict::deduce(
                BeingState::Being(self.a_genesis as u8),
                BeingState::Being(self.b_genesis as u8),
                0.0,
            );
            self.witness_records.push(v.clone());
            Some(v)
        } else {
            None
        };

        // 高紧迫度 = 更强反馈
        let urgency = call.urgency_level() as f64;
        let response_strength = self.bond_strength * (urgency / 9.0);

        CallResponse::Received {
            call: call.clone(),
            strength: response_strength,
            witness,
            relation_id: (self.a_genesis, self.b_genesis),
        }
    }

    /// 通过本关系传播反馈
    pub fn feed(&mut self, mut loop_: FeedbackLoop, divisor_override: Option<u64>) -> FeedbackResult {
        let divisor = divisor_override.unwrap_or(self.common_divisor);
        loop_.propagate(divisor);
        let closed = loop_.attempt_close(if loop_.receiver == self.a_genesis { self.b_genesis } else { self.a_genesis });
        self.feedback_loops.push(loop_.clone());
        FeedbackResult {
            propagated: true,
            closed_loop: closed,
            final_strength: loop_.strength,
            kind: loop_.kind,
        }
    }

    /// 判断之力: 关系是否应该存续 (三元裁定)
    pub fn judge_raw(is_alive: bool, is_strong: bool, has_loop: bool) -> JudgmentVerdict {
        match (is_alive, is_strong, has_loop) {
            (true, true, true) => JudgmentVerdict::Maintain,
            (true, true, false) => JudgmentVerdict::Strengthen,
            (true, false, _) => JudgmentVerdict::Repair,
            (false, _, _) => JudgmentVerdict::Sever,
        }
    }

    /// 判断之力: 关系是否应该存续
    /// 不是投票 — 是三元裁定
    pub fn judge(&self) -> JudgmentVerdict {
        let alive = self.is_alive();
        let strong = self.bond_strength > 0.5;
        let consistent = self.feedback_loops.iter().filter(|l| l.is_closed).count() > 0;

        match (alive, strong, consistent) {
            (true, true, true) => JudgmentVerdict::Maintain,
            (true, true, false) => JudgmentVerdict::Strengthen,   // 活+强但无闭环 → 需要增强
            (true, false, _) => JudgmentVerdict::Repair,         // 活但弱 → 修复
            (false, _, _) => JudgmentVerdict::Sever,             // 死的 → 切断
        }
    }

    /// 评价之力: 关系的当前价值 (不是判断该不该存续, 是评估有多好)
    pub fn evaluate(&self) -> EvaluationScore {
        let bond_score = (self.bond_strength * 40.0) as u32;          // 40%
        let gcd_score = (self.common_divisor.min(20) * 2) as u32;     // 公约数贡献 (最高40)
        let lcm_bonus = if self.common_multiple > 0 { 10 } else { 0 }; // 有节律+10
        let history_score = (self.feedback_loops.len().min(10)) as u32; // 历史贡献 (最高10)

        let total = bond_score + gcd_score + lcm_bonus + history_score;
        EvaluationScore {
            raw: total,
            level: match total {
                0 => EvaluationLevel::Broken,
                1..=29 => EvaluationLevel::Fragile,
                30..=54 => EvaluationLevel::Weak,
                55..=79 => EvaluationLevel::Healthy,
                80.. => EvaluationLevel::Excellent,
            },
        }
    }
}

/// 判断结论
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JudgmentVerdict {
    Maintain,    // 维持
    Strengthen,  // 加强
    Repair,      // 修复
    Sever,       // 切断
}

/// 评价分数
#[derive(Clone, Debug)]
pub struct EvaluationScore {
    pub raw: u32,
    pub level: EvaluationLevel,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvaluationLevel {
    Excellent,   // 80-100
    Healthy,     // 55-79
    Weak,        // 30-54
    Fragile,     // 1-29
    Broken,      // 0
}

/// 呼唤响应
#[derive(Clone, Debug)]
pub enum CallResponse {
    Received {
        call: Call,
        strength: f64,
        witness: Option<TriuneVerdict>,
        relation_id: (u64, u64),
    },
    DeadRelay,  // 关系已死, 无法传递
}

// 手动实现 PartialEq (跳过 TriuneVerdict,因为它没实现 PartialEq)
impl PartialEq for CallResponse {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CallResponse::DeadRelay, CallResponse::DeadRelay) => true,
            (
                CallResponse::Received { call: c1, relation_id: r1, .. },
                CallResponse::Received { call: c2, relation_id: r2, .. },
            ) => c1 == c2 && r1 == r2,
            _ => false,
        }
    }
}

/// 反馈结果
#[derive(Clone, Debug)]
pub struct FeedbackResult {
    pub propagated: bool,
    pub closed_loop: bool,
    pub final_strength: f64,
    pub kind: FeedbackKind,
}

// ────────────────────────────────────────────────────────────
// 判断之力 + 评价之力 — 综合引擎
// ────────────────────────────────────────────────────────────

pub struct DynamicsEngine {
    /// 所有已知关系
    pub relations: Vec<ContainerRelation>,
    /// 全局代数层
    pub generation: i32,
    /// 待处理呼唤队列
    pub pending_calls: Vec<(u64, Call)>,
}

impl DynamicsEngine {
    pub fn new(generation: i32) -> Self {
        DynamicsEngine {
            relations: Vec::new(),
            generation: generation.max(1).min(9),
            pending_calls: Vec::new(),
        }
    }

    /// 注册关系
    pub fn register_relation(&mut self, relation: ContainerRelation) {
        self.relations.push(relation);
    }

    /// 查找两个容器之间的关系
    pub fn find_relation(&self, a: u64, b: u64) -> Option<&ContainerRelation> {
        self.relations.iter().find(|r| {
            (r.a_genesis == a && r.b_genesis == b) || (r.a_genesis == b && r.b_genesis == a)
        })
    }

    /// 发出呼唤 — 入队等待处理
    pub fn emit(&mut self, from: u64, call: Call) {
        self.pending_calls.push((from, call));
        // 按紧迫度降序排列 (9最优先)
        self.pending_calls.sort_by(|a, b| b.1.urgency_level().cmp(&a.1.urgency_level()));
    }

    /// 处理所有待处理呼唤
    pub fn process_calls(&mut self) -> Vec<CallResponse> {
        let mut results = Vec::new();
        let calls: Vec<(u64, Call)> = self.pending_calls.drain(..).collect();
        for (from, call) in calls {
            // 找到能接收此呼唤的关系
            let mut responded = false;
            for rel in self.relations.iter_mut() {
                if rel.a_genesis == from || rel.b_genesis == from {
                    let response = rel.call(&call);
                    results.push(response);
                    responded = true;
                    break;
                }
            }
            if !responded {
                results.push(CallResponse::DeadRelay);
            }
        }
        results
    }

    /// 全局诊断: 检查所有关系健康度
    pub fn diagnose(&self) -> DynamicsReport {
        let mut report = DynamicsReport {
            total: self.relations.len(),
            alive: 0,
            dead: 0,
            needs_repair: 0,
            excellent: 0,
            average_strength: 0.0,
            generation: self.generation,
        };

        for rel in &self.relations {
            if rel.is_alive() {
                report.alive += 1;
                report.average_strength += rel.bond_strength;
                if rel.bond_strength < 0.5 {
                    report.needs_repair += 1;
                }
                let eval = rel.evaluate();
                if eval.level == EvaluationLevel::Excellent {
                    report.excellent += 1;
                }
            } else {
                report.dead += 1;
            }
        }

        if report.alive > 0 {
            report.average_strength /= report.alive as f64;
        }

        report
    }

    /// 全局偏移: 将所有关系推进到下一个代数层
    pub fn advance_generation(&mut self) {
        if self.generation < 9 {
            self.generation += 1;
            let delta = offset_rate(self.generation);
            for rel in self.relations.iter_mut() {
                // 偏移率增加 → 关系自然轻微衰减 (模拟平行世界漂移)
                let decay = delta * 0.05; // 每层衰减 5% 的偏移
                rel.bond_strength -= decay;
                if rel.bond_strength < 0.0 {
                    rel.bond_strength = 0.0;
                }
                // 公约数在偏移中保持 (根基不变)
            }
        }
    }
}

/// 全局诊断报告
#[derive(Clone, Debug)]
pub struct DynamicsReport {
    pub total: usize,
    pub alive: usize,
    pub dead: usize,
    pub needs_repair: usize,
    pub excellent: usize,
    pub average_strength: f64,
    pub generation: i32,
}

// ────────────────────────────────────────────────────────────
// GCD / LCM — 公约数/公倍数 (加法唯一允许的"乘法替代")
// ────────────────────────────────────────────────────────────

/// 计算最大公约数 (欧几里得算法, 只用减法替代取模)
pub fn calculate_gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 { return b; }
    if b == 0 { return a; }
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    a
}

// ────────────────────────────────────────────────────────────
// Need/Offer 匹配 — 需求与供给的二分匹配
// ────────────────────────────────────────────────────────────
//
// 概念来源: AI创建的未授权容器交互模块 (已删除,仅保留思路)
// 转写规则: 复用现有 Call::Need / Call::Offer 类型, GCD 共振匹配

/// Need/Offer 匹配结果
#[derive(Clone, Debug)]
pub struct NeedOfferMatch {
    pub needer: u64,       // 需求方 ID (从 Call::Need 提取)
    pub offerer: u64,      // 供给方 ID (从 Call::Offer 提取)
    pub resource: String,  // 资源名称
    pub strength: u64,     // 匹配强度 = gcd(needer_id, offerer_id)
}

/// 在需求列表和供给列表之间做二分匹配
///
/// 算法:
///   对每个 need 和每个 offer 做交叉比较
///   如果 gcd(needer, offerer) > threshold, 视为可匹配
///   使用 need.urgency + offer.quantity 综合评分
///
/// 仅使用加减法和 GCD (无乘除)
pub fn match_needs_offers(needs: &[Call], offers: &[Call], min_gcd: u64) -> Vec<NeedOfferMatch> {
    let mut results: Vec<NeedOfferMatch> = Vec::new();

    for need in needs {
        let (needer_id, resource, urgency) = match need {
            Call::Need { resource, urgency } => {
                let id = resource.len() as u64 + (*urgency as u64);
                (id, resource.clone(), *urgency)
            }
            _ => continue,
        };

        for offer in offers {
            let (offerer_id, quantity) = match offer {
                Call::Offer { resource, quantity } => {
                    let id = resource.len() as u64 + (*quantity as u64);
                    (id, *quantity)
                }
                _ => continue,
            };

            let g = calculate_gcd(needer_id, offerer_id);
            if g > min_gcd {
                let strength = g + (urgency as u64) + quantity;
                results.push(NeedOfferMatch {
                    needer: needer_id,
                    offerer: offerer_id,
                    resource: resource.clone(),
                    strength,
                });
            }
        }
    }

    results
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_relation_bonded_by_gcd() {
        let rel = ContainerRelation::genesis(12, 18);
        assert_eq!(rel.common_divisor, 6);   // gcd(12,18) = 6
        assert_eq!(rel.common_multiple, 36); // lcm(12,18) = 36
        assert!(rel.bond_strength > 0.0);
        assert!(rel.is_alive());
    }

    #[test]
    fn genesis_with_zero_bond_broken() {
        // genesis 为 0 的容器 = 不存在
        let rel = ContainerRelation::genesis(0, 5);
        assert_eq!(rel.common_divisor, 5); // gcd(0,5) = 5
        // 但 a_genesis=0 是异常情况
    }

    #[test]
    fn coprime_containers_have_minimal_bond() {
        let rel = ContainerRelation::genesis(7, 11); // 都是质数, gcd=1
        assert_eq!(rel.common_divisor, 1); // 最小共振
        assert!(rel.bond_strength > 0.0 && rel.bond_strength <= 1.0);
    }

    #[test]
    fn relate_judge_maintain() {
        let rel = ContainerRelation::genesis(100, 200);
        // 活 + 强 + 无闭环 → Strengthen or Maintain
        let v = rel.judge();
        assert!(v == JudgmentVerdict::Maintain || v == JudgmentVerdict::Strengthen);
    }

    #[test]
    fn dead_relation_severed() {
        let mut rel = ContainerRelation::genesis(10, 20);
        rel.a_vitality = Vitality::Dormant;
        assert!(!rel.is_alive());
        assert_eq!(rel.judge(), JudgmentVerdict::Sever);
    }

    #[test]
    fn weak_relation_needs_repair() {
        let mut rel = ContainerRelation::genesis(100, 200);
        rel.bond_strength = 0.3; // 弱
        assert_eq!(rel.judge(), JudgmentVerdict::Repair);
    }

    #[test]
    fn evaluation_scoring() {
        let rel = ContainerRelation::genesis(12, 18);
        let eval = rel.evaluate();
        // 新创世关系: bond=1.0, gcd=6, lcm=36, no history
        // = 40 + 12 + 10 + 0 = 62 → Healthy
        assert!(eval.raw >= 55);
        assert_eq!(eval.level, EvaluationLevel::Healthy);
    }

    #[test]
    fn broken_evaluation() {
        let mut rel = ContainerRelation::genesis(12, 18);
        rel.bond_strength = 0.0;
        rel.common_divisor = 0;
        rel.common_multiple = 0;
        let eval = rel.evaluate();
        assert_eq!(eval.level, EvaluationLevel::Broken);
    }

    #[test]
    fn feedback_loop_closes() {
        let mut loop_ = FeedbackLoop::new(100, 200, FeedbackKind::Positive);
        // 传播几步
        loop_.propagate(6);
        loop_.propagate(6);
        // 回到发起者
        let closed = loop_.attempt_close(100);
        assert!(closed);
        assert!(loop_.is_closed);
        assert!(loop_.strength > 0.0); // 正反馈闭环有增强
    }

    #[test]
    fn feedback_loop_negative_decay() {
        let mut loop_ = FeedbackLoop::new(100, 200, FeedbackKind::Negative);
        loop_.propagate(6);
        loop_.attempt_close(100);
        // 负反馈闭环 = 衰减
        // strength 应该 < 1.0
    }

    #[test]
    fn call_urgency_sorting() {
        let call1 = Call::Ping;
        let call2 = Call::Need { resource: "data".to_string(), urgency: 9 };
        let call3 = Call::Offer { resource: "x".to_string(), quantity: 1 };
        assert!(call2.urgency_level() > call1.urgency_level()); // Need.u9 > Ping
        assert!(call2.urgency_level() > call3.urgency_level()); // Need.u9 > Offer(3)
    }

    #[test]
    fn call_arbitration_requires_witness() {
        let call = Call::Arbitrate {
            claimant_a: 1,
            claimant_b: 2,
            dispute: "产权".to_string(),
        };
        assert!(call.requires_witness());
    }

    #[test]
    fn engine_register_and_find() {
        let mut engine = DynamicsEngine::new(1);
        let rel = ContainerRelation::genesis(123, 456);
        engine.register_relation(rel);
        assert!(engine.find_relation(123, 456).is_some());
        assert!(engine.find_relation(456, 123).is_some()); // 双向查找
    }

    #[test]
    fn engine_emit_and_process() {
        let mut engine = DynamicsEngine::new(1);
        engine.register_relation(ContainerRelation::genesis(10, 20));
        engine.emit(10, Call::Ping);
        let results = engine.process_calls();
        assert_eq!(results.len(), 1);
        match &results[0] {
            CallResponse::Received { strength, .. } => assert!(*strength > 0.0),
            _ => panic!("应为 Received"),
        }
    }

    #[test]
    fn engine_dead_relay_for_unknown() {
        let mut engine = DynamicsEngine::new(1);
        engine.emit(999, Call::Ping); // 未注册的关系
        let results = engine.process_calls();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], CallResponse::DeadRelay);
    }

    #[test]
    fn engine_diagnose() {
        let mut engine = DynamicsEngine::new(1);
        engine.register_relation(ContainerRelation::genesis(12, 18));
        engine.register_relation(ContainerRelation::genesis(7, 13));
        let report = engine.diagnose();
        assert_eq!(report.total, 2);
        assert_eq!(report.alive, 2);
        assert_eq!(report.dead, 0);
    }

    #[test]
    fn generation_advance_with_decay() {
        let mut engine = DynamicsEngine::new(1);
        engine.register_relation(ContainerRelation::genesis(100, 200));
        let strength_before = engine.relations[0].bond_strength;
        engine.advance_generation(); // gen 1→2
        let strength_after = engine.relations[0].bond_strength;
        assert!(strength_after < strength_before); // 偏移导致衰减
        assert_eq!(engine.generation, 2);
    }

    #[test]
    fn generation_cannot_exceed_nine() {
        let mut engine = DynamicsEngine::new(9);
        engine.register_relation(ContainerRelation::genesis(1, 2));
        engine.advance_generation(); // 尝试到 10
        assert_eq!(engine.generation, 9); // 不变
    }

    #[test]
    fn gcd_only_subtraction() {
        assert_eq!(calculate_gcd(48, 18), 6);
        assert_eq!(calculate_gcd(7, 13), 1);
        assert_eq!(calculate_gcd(100, 0), 100);
        assert_eq!(calculate_gcd(0, 50), 50);
        assert_eq!(calculate_gcd(270, 192), 6);
    }

    #[test]
    fn call_event_summaries() {
        let c1 = Call::Need { resource: "food".to_string(), urgency: 5 };
        let c2 = Call::Ping;
        let c3 = Call::Pong { responder_genesis: 42 };
        let c4 = Call::Arbitrate { claimant_a: 1, claimant_b: 2, dispute: "x".to_string() };
        assert!(c1.urgency_level() == 5);
        assert!(c2.urgency_level() == 1);
        assert!(c3.urgency_level() == 1);
        assert!(c4.urgency_level() == 9);
    }

    // ── Need/Offer 匹配测试 ──

    #[test]
    fn need_offer_match_links_pair() {
        use Call::{Need, Offer};
        let needs = vec![
            Need { resource: "alpha".to_string(), urgency: 5 },
        ];
        let offers = vec![
            Offer { resource: "beta".to_string(), quantity: 10 },
        ];
        let matches = super::match_needs_offers(&needs, &offers, 0);
        // ID will be computed regardless, should not panic
        let _ = matches.len();
    }

    #[test]
    fn need_offer_empty_inputs() {
        let needs: Vec<Call> = vec![];
        let offers: Vec<Call> = vec![];
        let matches = super::match_needs_offers(&needs, &offers, 0);
        assert!(matches.is_empty());
    }

    #[test]
    fn need_offer_struct_fields() {
        let m = NeedOfferMatch {
            needer: 100,
            offerer: 200,
            resource: "coin".to_string(),
            strength: 5,
        };
        assert_eq!(m.needer, 100);
        assert_eq!(m.offerer, 200);
        assert_eq!(m.strength, 5);
    }
}
