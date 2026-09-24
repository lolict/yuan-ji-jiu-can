// ══════════════════════════════════════════════════════════════════
// judgment_act_bridge.rs — 判断→行动桥梁 · 感知-思考-执行闭环
// ══════════════════════════════════════════════════════════════════
//
//   「判断产出评价 → 评价驱动呼唤 → 呼唤触发行动 → 行动结果反馈回判断」
//
//   这是"大脑"与"双手"之间的桥梁:
//   - 判断引擎产出评价 (noise / notice / respond / intervene / alert)
//   - 不同评价等级映射到不同的呼唤入口 (invocation table entries)
//   - 同一评价等级可有多条候选入口, 按权重排序
//   - 执行结果 (成功/失败) 反馈回判断引擎, 影响后续感知灵敏度
//
//   设计原则 (源自满全法):
//   - 不搜索入口: 评价等级直接索引位置
//   - 不传参: 评价本身携带权重, 呼唤表按权重选入口
//   - 不判断两次: 一次评价终身有效 (直到反馈修正)
//   - 执行即反馈: 每次 invocation 都是对判断的验证
//
// ══════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::hash::Hash;

use crate::judgment_power::{Evaluation, EvaluationLevel, FeedbackSignal};
use crate::invocation_table::{InvocationTable, InvocationEntry, Invoker, OpKind, InvokeError};

// ────────────────────────────────────────────────────────────
// 评价-呼唤映射
// ────────────────────────────────────────────────────────────

/// 评价等级 → 呼唤入口编号
///
/// 这是"判断→行动"的映射表:
///   评价等级      呼唤范围      含义
///   ─────────────────────────────────────
///   Noise         100-199      仅记录, 不采取行动
///   Notice        200-299      记录 + 轻量通知
///   Respond       300-399      响应: 调用对应处理程序
///   Intervene     400-499      干预: 强制修改系统状态
///   Alert         500-599      警报: 紧急措施 + 扩大观察
#[derive(Clone, Debug)]
pub struct ActMapping {
    /// 等级 → 候选呼唤入口列表
    pub entries: HashMap<EvaluationLevel, Vec<u32>>,
    /// 默认入口 (无匹配时回退)
    pub default_entry: u64,
}

impl ActMapping {
    pub fn new(default_entry: u64) -> Self {
        let mut entries = HashMap::new();
        entries.insert(EvaluationLevel::Noise, (100..200).collect());
        entries.insert(EvaluationLevel::Notice, (200..300).collect());
        entries.insert(EvaluationLevel::Respond, (300..400).collect());
        entries.insert(EvaluationLevel::Intervene, (400..500).collect());
        entries.insert(EvaluationLevel::Alert, (500..600).collect());
        Self { entries, default_entry }
    }

    /// 自定义某等级的呼唤入口范围
    pub fn set_range(&mut self, level: EvaluationLevel, range: std::ops::Range<u32>) {
        self.entries.insert(level, range.collect());
    }

    /// 获取评价等级对应的候选入口
    pub fn candidates(&self, level: &EvaluationLevel) -> &[u32] {
        static EMPTY: &[u32] = &[];
        self.entries.get(level).map(|v| v.as_slice()).unwrap_or(EMPTY)
    }
}

// ────────────────────────────────────────────────────────────
// 行动请求 — 评价 → 呼唤的一次调度
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct ActRequest {
    /// 关联的评价
    pub evaluation_weight: f64,
    pub evaluation_level: EvaluationLevel,
    /// 选中的呼唤入口
    pub invocation_entry_id: u32,
    /// 寻址坐标 (进制地址)
    pub address: u64,
    /// 执行步骤计数
    pub steps: u32,
    /// 是否已被执行
    pub executed: bool,
    /// 执行结果
    pub result: Option<ActResult>,
}

/// 行动执行结果
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActResult {
    /// 行动成功
    Success,
    /// 行动失败 (附错误)
    Failed(String),
    /// 行动被拒绝 (规则不允许)
    Rejected,
    /// 行动引发新评价 (反馈回判断引擎)
    TriggeredNewEval,
}

// ────────────────────────────────────────────────────────────
// 判断-行动执行器 — 桥梁核心
// ────────────────────────────────────────────────────────────

/// 判断→行动执行器
///
/// 将评价转换为呼唤, 在 InvocationTable 上执行,
/// 收集执行结果, 生成反馈信号
pub struct JudgmentActBridge {
    /// 评价-呼唤映射
    pub mapping: ActMapping,
    /// 呼唤表 (行动库)
    pub invocation_table: InvocationTable,
    /// 执行历史
    pub history: Vec<ActRequest>,
}

impl JudgmentActBridge {
    pub fn new() -> Self {
        Self {
            mapping: ActMapping::new(100),
            invocation_table: InvocationTable::new(),
            history: Vec::new(),
        }
    }

    /// 注册一个呼唤入口
    pub fn register_action(&mut self, entry: InvocationEntry) {
        self.invocation_table.register(entry)
    }

    /// 将评价转化为行动请求并执行
    ///
    /// 调度算法:
    ///   1. 查找评价等级对应的候选入口
    ///   2. 按评价权重选择入口 (高权重 → 编号大的入口)
    ///   3. 创建 Invoker 并执行
    ///   4. 收集结果
    pub fn act_on<T: Clone + Eq + Hash>(
        &mut self,
        evaluation: &Evaluation<T>,
    ) -> Option<ActRequest> {
        let candidates = self.mapping.candidates(&evaluation.level);
        if candidates.is_empty() {
            return None;
        }

        // 按评价权重选择入口 (加权选择)
        let entry_idx = if candidates.len() == 1 {
            0
        } else {
            let pos = (evaluation.weight * (candidates.len() - 1) as f64).round() as usize;
            pos.min(candidates.len() - 1)
        };
        let entry_id = candidates[entry_idx];

        // 创建执行器并运行
        let mut invoker = Invoker::new();
        let result = match invoker.call_entry(&self.invocation_table, entry_id) {
            Ok(()) => ActResult::Success,
            Err(InvokeError::Halted) => ActResult::Success, // Halt 视为正常结束
            Err(e) => ActResult::Failed(format!("{:?}", e)),
        };

        // 寻址坐标 = 入口 ID 的一维展开
        let address = entry_id as u64;

        let request = ActRequest {
            evaluation_weight: evaluation.weight,
            evaluation_level: evaluation.level,
            invocation_entry_id: entry_id,
            address,
            steps: invoker.steps as u32,
            executed: true,
            result: Some(result),
        };

        self.history.push(request.clone());
        Some(request)
    }

    /// 执行完整的感知-思考-行动-反馈循环
    ///
    /// 输入: 评价 + 规则
    /// 输出: (行动请求, 反馈信号列表)
    pub fn think_and_act<T: Clone + Eq + Hash>(
        &mut self,
        evaluation: &Evaluation<T>,
    ) -> (Option<ActRequest>, Vec<FeedbackSignal>) {
        let request = self.act_on(evaluation);
        let feedback = self.generate_feedback(&request);
        (request, feedback)
    }

    /// 根据行动结果生成反馈信号
    fn generate_feedback(&self, request: &Option<ActRequest>) -> Vec<FeedbackSignal> {
        let mut signals = Vec::new();
        match request {
            Some(req) => match &req.result {
                Some(ActResult::Success) => {
                    signals.push(FeedbackSignal::Maintain);
                }
                Some(ActResult::Failed(_)) => {
                    // 行动失败 → 扩大观察 (可能需要更多信息)
                    signals.push(FeedbackSignal::ExpandObservation("diagnostic".into()));
                }
                Some(ActResult::Rejected) => {
                    // 被拒绝 → 抑制相关感知
                    signals.push(FeedbackSignal::SuppressPerception("rejected_source".into()));
                }
                Some(ActResult::TriggeredNewEval) => {
                    signals.push(FeedbackSignal::ExpandObservation("cascade".into()));
                }
                None => {}
            },
            None => {
                // 无行动 → 维持
                signals.push(FeedbackSignal::Maintain);
            }
        }
        signals
    }

    /// 获取行动统计
    pub fn stats(&self) -> ActStats {
        let total = self.history.len();
        let success = self.history.iter()
            .filter(|r| r.result == Some(ActResult::Success))
            .count();
        let failed = self.history.iter()
            .filter(|r| matches!(r.result, Some(ActResult::Failed(_))))
            .count();
        let rejected = self.history.iter()
            .filter(|r| r.result == Some(ActResult::Rejected))
            .count();

        ActStats {
            total,
            success,
            failed,
            rejected,
            success_rate: if total > 0 { success as f64 / total as f64 } else { 0.0 },
        }
    }
}

impl Default for JudgmentActBridge {
    fn default() -> Self {
        Self::new()
    }
}

/// 行动统计
#[derive(Clone, Debug)]
pub struct ActStats {
    pub total: usize,
    pub success: usize,
    pub failed: usize,
    pub rejected: usize,
    pub success_rate: f64,
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::judgment_power::{CausalPair, RippleDiff, DiffKind};

    #[test]
    fn act_mapping_default_ranges() {
        let mapping = ActMapping::new(100);
        assert_eq!(mapping.candidates(&EvaluationLevel::Noise).len(), 100);
        assert_eq!(mapping.candidates(&EvaluationLevel::Notice).len(), 100);
        assert_eq!(mapping.candidates(&EvaluationLevel::Alert).len(), 100);
        assert_eq!(mapping.candidates(&EvaluationLevel::Noise)[0], 100);
        assert_eq!(mapping.candidates(&EvaluationLevel::Alert)[0], 500);
    }

    #[test]
    fn act_mapping_custom_range() {
        let mut mapping = ActMapping::new(0);
        mapping.set_range(EvaluationLevel::Respond, 300..305);
        assert_eq!(mapping.candidates(&EvaluationLevel::Respond).len(), 5);
        assert_eq!(mapping.candidates(&EvaluationLevel::Respond)[0], 300);
        assert_eq!(mapping.candidates(&EvaluationLevel::Respond)[4], 304);
    }

    fn make_eval(level: EvaluationLevel, amp: f64) -> Evaluation<&'static str> {
        // Create a simple CausalPair with given magnitude
        let pair: CausalPair<&str> = CausalPair::new(
            RippleDiff {
                from_ts: 0, to_ts: 1,
                source_id: "test".into(),
                amp_delta: amp,
                phase_delta: 0.0,
                kind: DiffKind::Amplitude,
            },
            "cause_a", "effect_b"
        );
        Evaluation::new(pair, level, "test")
    }

    #[test]
    fn bridge_register_and_act() {
        let mut bridge = JudgmentActBridge::new();

        // 将 Notice 范围设为只包含 201, 避免选中未注册的入口
        bridge.mapping.set_range(EvaluationLevel::Notice, 201..202);
        bridge.register_action(InvocationEntry::with_label(
            201,
            "log_notice",
            vec![OpKind::PushI(42), OpKind::Halt],
        ));

        let eval = make_eval(EvaluationLevel::Notice, 0.3);
        let request = bridge.act_on(&eval);
        assert!(request.is_some());
        let req = request.unwrap();
        assert_eq!(req.evaluation_level, EvaluationLevel::Notice);
        assert!(req.executed);
        assert_eq!(req.result, Some(ActResult::Success));
    }

    #[test]
    fn bridge_weighted_selection() {
        let mut bridge = JudgmentActBridge::new();

        // Notice 范围 200-299, 注册几个候选入口
        for id in [200u32, 220, 250, 299] {
            bridge.register_action(InvocationEntry::with_label(
                id, &format!("action_{}", id),
                vec![OpKind::PushI(id as u64), OpKind::Halt],
            ));
        }

        // 中等权重评价, 选择中间位置的入口
        let eval = make_eval(EvaluationLevel::Notice, 0.25);
        let request = bridge.act_on(&eval).unwrap();
        // Notice 范围 200-299
        assert!(request.invocation_entry_id >= 200);
        assert!(request.invocation_entry_id < 300);
    }

    #[test]
    fn think_and_act_generate_feedback() {
        let mut bridge = JudgmentActBridge::new();

        bridge.mapping.set_range(EvaluationLevel::Notice, 201..202);
        bridge.register_action(InvocationEntry::with_label(
            201, "log",
            vec![OpKind::Halt],
        ));

        let eval = make_eval(EvaluationLevel::Notice, 0.3);
        let (request, feedback) = bridge.think_and_act(&eval);

        assert!(request.is_some());
        // 成功 → Maintain
        assert!(feedback.iter().any(|s| matches!(s, FeedbackSignal::Maintain)));
    }

    #[test]
    fn act_stats() {
        let mut bridge = JudgmentActBridge::new();
        bridge.mapping.set_range(EvaluationLevel::Notice, 201..202);
        bridge.register_action(InvocationEntry::with_label(
            201, "log",
            vec![OpKind::Halt],
        ));

        for _ in 0..5 {
            let eval = make_eval(EvaluationLevel::Notice, 0.3);
            bridge.act_on(&eval);
        }

        let stats = bridge.stats();
        assert_eq!(stats.total, 5);
        assert_eq!(stats.success, 5);
        assert!((stats.success_rate - 1.0).abs() < 1e-6);
    }

    #[test]
    fn empty_candidates_graceful() {
        let mut bridge = JudgmentActBridge::new();
        // 清空所有候选入口
        bridge.mapping = ActMapping::new(999);
        bridge.mapping.entries.clear();

        let eval = make_eval(EvaluationLevel::Noise, 0.05);

        // 没有候选入口时返回 None
        let request = bridge.act_on(&eval);
        assert!(request.is_none());
    }

    #[test]
    fn eval_level_preserved_in_request() {
        let mut bridge = JudgmentActBridge::new();
        bridge.register_action(InvocationEntry::with_label(
            501, "emergency",
            vec![OpKind::PushI(999), OpKind::Halt],
        ));

        let eval = make_eval(EvaluationLevel::Alert, 1.5);
        let request = bridge.act_on(&eval).unwrap();
        assert_eq!(request.evaluation_level, EvaluationLevel::Alert);
        assert!(request.address >= 500);
    }
}
