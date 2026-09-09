// ══════════════════════════════════════════════════════════════════════
// perception_engine.rs — 感知引擎 · 认知闭环
// ══════════════════════════════════════════════════════════════════════
//
// 理论根基:
//   "感知状态机" — 不是被动响应, 是主动观察 + 编码 + 决策
//
//   三层架构:
//      时间引擎 (temporal)  = 时钟 — 何时感知
//      感知引擎 (perception) = 眼睛 — 感知到什么
//      后勤引擎 (logistics)  = 手   — 执行什么
//
//   认知闭环:
//      观察 → 编码 → 比较 → 决策 → 行动 → 再观察
//
//   感知编码 (64-bit cell):
//       [2:0]   level       — 感知来源的层级
//       [5:3]   state       — 被观察对象的状态
//       [13:6]  signal      — 信号强度 (0-255)
//       [21:14] threshold   — 触发反应的阈值
//       [29:22] delta       — 与上次的变化量
//       [37:30] anomaly     — 异常评分 (0=正常, 255=严重)
//       [63:38] timestamp   — 相对时间戳
//
// ══════════════════════════════════════════════════════════════════════

use crate::temporal_engine::{
    TemporalLevel, LifecycleState, PerceptionCell,
};
use crate::logistics_engine::{LogisticsTask, TaskOutcome};
use std::collections::HashMap;

// ────────────────────────────────────────────────────────────
// 感知信号类型
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SignalKind {
    GitUncommitted,         // 有未提交的代码变更
    GitUnpushed,            // 有未推送的 commit
    TestFailure,            // 测试失败
    MemoryDrift,            // 记忆文件有未备份的变更
    CredentialExposure,     // 凭据可能泄露
    LogOverflow,            // 日志文件过大
    DependencyStale,        // 依赖过时
    RepoOutOfSync,          // 多仓库不同步
    BranchStale,            // 分支长期未更新
    SystemAnomaly,          // 未知异常
}

impl SignalKind {
    /// 该信号对应的默认感知层级
    pub fn default_level(self) -> TemporalLevel {
        match self {
            SignalKind::GitUncommitted   => TemporalLevel::Cycle,     // 时级感知
            SignalKind::GitUnpushed      => TemporalLevel::Phase,     // 日级感知
            SignalKind::TestFailure      => TemporalLevel::Tick,      // 秒级感知 (紧急)
            SignalKind::MemoryDrift      => TemporalLevel::Phase,     // 日级感知
            SignalKind::CredentialExposure => TemporalLevel::Tick,    // 秒级 (最紧急)
            SignalKind::LogOverflow      => TemporalLevel::Era,       // 周级感知
            SignalKind::DependencyStale  => TemporalLevel::Era,       // 周级感知
            SignalKind::RepoOutOfSync    => TemporalLevel::Phase,     // 日级感知
            SignalKind::BranchStale      => TemporalLevel::Era,       // 周级感知
            SignalKind::SystemAnomaly    => TemporalLevel::Instant,   // 瞬态 (最高优先)
        }
    }

    /// 该信号对应的后勤响应任务
    pub fn response_task(self) -> LogisticsTask {
        match self {
            SignalKind::GitUncommitted   => LogisticsTask::CodeHealthCheck,
            SignalKind::GitUnpushed      => LogisticsTask::RepoSyncCheck,
            SignalKind::TestFailure      => LogisticsTask::CodeHealthCheck,
            SignalKind::MemoryDrift      => LogisticsTask::MemoryBackup,
            SignalKind::CredentialExposure => LogisticsTask::SecurityAudit,
            SignalKind::LogOverflow      => LogisticsTask::LogCleanup,
            SignalKind::DependencyStale  => LogisticsTask::DependencyScan,
            SignalKind::RepoOutOfSync    => LogisticsTask::RepoSyncCheck,
            SignalKind::BranchStale      => LogisticsTask::DependencyScan,
            SignalKind::SystemAnomaly    => LogisticsTask::ReportGeneration,
        }
    }

    /// 信号紧急度 (0=低, 255=极高)
    pub fn urgency(self) -> u8 {
        match self {
            SignalKind::CredentialExposure => 255,
            SignalKind::TestFailure      => 230,
            SignalKind::SystemAnomaly    => 220,
            SignalKind::GitUncommitted   => 150,
            SignalKind::GitUnpushed      => 120,
            SignalKind::MemoryDrift      => 100,
            SignalKind::RepoOutOfSync    => 90,
            SignalKind::LogOverflow      => 60,
            SignalKind::DependencyStale  => 40,
            SignalKind::BranchStale      => 30,
        }
    }
}

// ────────────────────────────────────────────────────────────
// 感知快照 — 某一时刻的系统观察
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct PerceptionSnapshot {
    pub timestamp: u64,
    pub signals: Vec<SignalKind>,
    pub intensities: Vec<u8>,           // 每个信号的强度
    pub anomaly_score: u8,              // 综合异常评分
    pub encoded_cells: Vec<u64>,        // 编码后的感知单元
}

impl PerceptionSnapshot {
    pub fn new(timestamp: u64) -> Self {
        Self {
            timestamp,
            signals: Vec::new(),
            intensities: Vec::new(),
            anomaly_score: 0,
            encoded_cells: Vec::new(),
        }
    }

    /// 添加一个感知信号
    pub fn observe(&mut self, kind: SignalKind, intensity: u8) {
        self.signals.push(kind);
        self.intensities.push(intensity);
        // 编码到 64-bit cell
        let cell = PerceptionCell::encode(
            kind.default_level(),
            LifecycleState::Mature,       // 感知信号本身视为成熟信息
            intensity,
            kind.urgency(),
            0,                            // delta (首次观察 = 0)
            anomaly_from_intensity(intensity),
        );
        self.encoded_cells.push(cell);
        self.recompute_anomaly();
    }

    /// 重新计算综合异常评分
    fn recompute_anomaly(&mut self) {
        if self.intensities.is_empty() {
            self.anomaly_score = 0;
            return;
        }
        // 加权平均: 紧急度 * 强度
        let total: u32 = self.signals.iter()
            .zip(self.intensities.iter())
            .map(|(s, i)| s.urgency() as u32 * *i as u32)
            .sum();
        let count = self.signals.len() as u32;
        self.anomaly_score = ((total / count) / 256).min(255) as u8;
    }

    /// 是否需要立即反应 (异常评分超阈值)
    pub fn needs_immediate_action(&self, threshold: u8) -> bool {
        self.anomaly_score >= threshold
    }

    /// 获取最紧急的信号
    pub fn most_urgent(&self) -> Option<(SignalKind, u8)> {
        self.signals.iter()
            .zip(self.intensities.iter())
            .max_by_key(|(s, _)| s.urgency())
            .map(|(s, i)| (*s, *i))
    }
}

/// 强度 → 异常分 (非线性: 高强度更危险)
fn anomaly_from_intensity(intensity: u8) -> u8 {
    if intensity > 200 { 255 }
    else if intensity > 150 { 180 }
    else if intensity > 100 { 120 }
    else if intensity > 50 { 60 }
    else { intensity / 2 }
}

// ────────────────────────────────────────────────────────────
// 感知引擎 — 主入口
// ────────────────────────────────────────────────────────────

pub struct PerceptionEngine {
    pub history: Vec<PerceptionSnapshot>,
    pub max_history: usize,
    pub baseline: Option<PerceptionSnapshot>,     // 基准线 (用于计算 delta)
    pub reaction_threshold: u8,                   // 触发反应的异常阈值
    /// 信号 → 响应任务的映射
    pub response_map: HashMap<SignalKind, LogisticsTask>,
}

impl PerceptionEngine {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            max_history: 256,
            baseline: None,
            reaction_threshold: 100,
            response_map: HashMap::new(),
        }
    }

    /// 建立一次感知快照 (观察系统状态)
    pub fn perceive(&mut self, timestamp: u64, signals: Vec<(SignalKind, u8)>) -> &PerceptionSnapshot {
        let mut snapshot = PerceptionSnapshot::new(timestamp);
        for (kind, intensity) in signals {
            snapshot.observe(kind, intensity);
        }

        // 与基准线比较, 计算 delta
        if let Some(ref base) = self.baseline {
            self.compute_deltas(&mut snapshot, base);
        }

        self.history.push(snapshot);
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }

        self.history.last().unwrap()
    }

    /// 计算与基准线的差异
    fn compute_deltas(&self, current: &mut PerceptionSnapshot, base: &PerceptionSnapshot) {
        // 找出新增的信号 (在 current 中但不在 base 中)
        for (i, sig) in current.signals.iter().enumerate() {
            if !base.signals.contains(sig) {
                // 新出现的信号 → delta = 强度本身 (全新)
                // 已在 observe 中编码, 这里可以追加标记
                let _ = i;
            }
        }
    }

    /// 设置基准线 (用于后续比较)
    pub fn set_baseline(&mut self, snapshot: PerceptionSnapshot) {
        self.baseline = Some(snapshot);
    }

    /// 获取最近一次感知
    pub fn last_perception(&self) -> Option<&PerceptionSnapshot> {
        self.history.last()
    }

    /// 判断是否需要触发行动
    /// 返回需要响应的任务列表
    pub fn decide_actions(&self) -> Vec<LogisticsTask> {
        let mut tasks = Vec::new();
        if let Some(last) = self.history.last() {
            if last.needs_immediate_action(self.reaction_threshold) {
                // 超阈值: 所有信号都触发响应
                for sig in &last.signals {
                    tasks.push(sig.response_task());
                }
            } else if let Some((urgent_sig, urgency)) = last.most_urgent() {
                // 未超阈值: 只有极紧急信号(urgency≥200)才单独触发
                if urgency >= 200 {
                    tasks.push(urgent_sig.response_task());
                }
            }
        }
        // 去重
        tasks.dedup();
        tasks
    }

    /// 感知趋势: 异常在恶化还是好转?
    pub fn trend(&self, window: usize) -> Trend {
        let start = self.history.len().saturating_sub(window);
        let slice = &self.history[start..];
        if slice.len() < 2 {
            return Trend::Stable;
        }
        let first_avg = slice[..slice.len()/2].iter()
            .map(|s| s.anomaly_score as u32).sum::<u32>() / (slice.len()/2).max(1) as u32;
        let second_avg = slice[slice.len()/2..].iter()
            .map(|s| s.anomaly_score as u32).sum::<u32>() / (slice.len() - slice.len()/2).max(1) as u32;

        if second_avg > first_avg + 20 { Trend::Worsening }
        else if second_avg + 20 < first_avg { Trend::Improving }
        else { Trend::Stable }
    }

    /// 生成感知报告 (人类可读)
    pub fn report(&self) -> String {
        let mut r = String::new();
        if let Some(last) = self.history.last() {
            r.push_str(&format!("perception @ t={}: anomaly={}/255\n", last.timestamp, last.anomaly_score));
            for (sig, int) in last.signals.iter().zip(last.intensities.iter()) {
                r.push_str(&format!("  [{:>3}] {:?} (urgency={})\n", int, sig, sig.urgency()));
            }
            match self.trend(10) {
                Trend::Worsening => r.push_str("trend: WORSENING\n"),
                Trend::Improving => r.push_str("trend: improving\n"),
                Trend::Stable    => r.push_str("trend: stable\n"),
            }
            let actions = self.decide_actions();
            if !actions.is_empty() {
                r.push_str("actions: ");
                for t in &actions {
                    r.push_str(&format!("{:?} ", t));
                }
                r.push('\n');
            }
        } else {
            r.push_str("no perception data\n");
        }
        r
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trend {
    Worsening,
    Improving,
    Stable,
}

// ════════════════════════════════════════════════════════════
// 测试
// ════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_kind_urgency() {
        assert!(SignalKind::CredentialExposure.urgency() > SignalKind::GitUncommitted.urgency());
        assert!(SignalKind::BranchStale.urgency() < SignalKind::LogOverflow.urgency());
    }

    #[test]
    fn test_perception_snapshot_adds_signals() {
        let mut snap = PerceptionSnapshot::new(1000);
        snap.observe(SignalKind::GitUncommitted, 128);
        snap.observe(SignalKind::TestFailure, 200);
        assert_eq!(snap.signals.len(), 2);
        assert_eq!(snap.encoded_cells.len(), 2);
    }

    #[test]
    fn test_anomaly_computation() {
        let mut snap = PerceptionSnapshot::new(0);
        snap.observe(SignalKind::BranchStale, 10);     // 低强度低紧急
        assert!(snap.anomaly_score < 50);

        let mut snap2 = PerceptionSnapshot::new(0);
        snap2.observe(SignalKind::CredentialExposure, 255);  // 极高
        assert!(snap2.anomaly_score > 100);
    }

    #[test]
    fn test_needs_immediate_action() {
        let mut snap = PerceptionSnapshot::new(0);
        snap.observe(SignalKind::TestFailure, 250);
        assert!(snap.needs_immediate_action(100));

        let mut snap2 = PerceptionSnapshot::new(0);
        snap2.observe(SignalKind::BranchStale, 5);
        assert!(!snap2.needs_immediate_action(100));
    }

    #[test]
    fn test_most_urgent_signal() {
        let mut snap = PerceptionSnapshot::new(0);
        snap.observe(SignalKind::GitUncommitted, 100);
        snap.observe(SignalKind::CredentialExposure, 50);
        snap.observe(SignalKind::BranchStale, 200);
        let (urgent, _) = snap.most_urgent().unwrap();
        assert_eq!(urgent, SignalKind::CredentialExposure);  // 紧急度最高
    }

    #[test]
    fn test_perception_engine_decide_actions() {
        let mut engine = PerceptionEngine::new();
        engine.perceive(1000, vec![
            (SignalKind::TestFailure, 200),
            (SignalKind::GitUncommitted, 150),
        ]);
        let actions = engine.decide_actions();
        assert!(!actions.is_empty());
        // 应包含 CodeHealthCheck (响应 TestFailure)
        assert!(actions.contains(&LogisticsTask::CodeHealthCheck));
    }

    #[test]
    fn test_perception_engine_no_action_when_calm() {
        let mut engine = PerceptionEngine::new();
        engine.perceive(1000, vec![
            (SignalKind::BranchStale, 5),
        ]);
        let actions = engine.decide_actions();
        // 低强度低紧急, 不触发
        assert!(actions.is_empty());
    }

    #[test]
    fn test_trend_stable() {
        let mut engine = PerceptionEngine::new();
        for i in 0..10 {
            engine.perceive(i * 1000, vec![(SignalKind::GitUncommitted, 50)]);
        }
        assert_eq!(engine.trend(10), Trend::Stable);
    }

    #[test]
    fn test_trend_worsening() {
        let mut engine = PerceptionEngine::new();
        for i in 0..5 {
            engine.perceive(i * 1000, vec![(SignalKind::GitUncommitted, 20)]);
        }
        for i in 5..10 {
            engine.perceive(i * 1000, vec![(SignalKind::GitUncommitted, 200)]);
        }
        assert_eq!(engine.trend(10), Trend::Worsening);
    }

    #[test]
    fn test_report_generation() {
        let mut engine = PerceptionEngine::new();
        engine.perceive(5000, vec![
            (SignalKind::MemoryDrift, 120),
            (SignalKind::RepoOutOfSync, 80),
        ]);
        let report = engine.report();
        assert!(report.contains("anomaly="));
        assert!(report.contains("MemoryDrift"));
    }

    #[test]
    fn test_signal_response_mapping() {
        assert_eq!(SignalKind::TestFailure.response_task(), LogisticsTask::CodeHealthCheck);
        assert_eq!(SignalKind::MemoryDrift.response_task(), LogisticsTask::MemoryBackup);
        assert_eq!(SignalKind::CredentialExposure.response_task(), LogisticsTask::SecurityAudit);
    }
}
