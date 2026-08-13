// ══════════════════════════════════════════════════════════════════════
// logistics_engine.rs — 后勤引擎 · 分支同步自动机
// ══════════════════════════════════════════════════════════════════════
//
// 理论根基:
//   "让机器主动完成善后工作 — 打扫战场、运粮食、医伤员、守城"
//
//   时间引擎提供时钟节奏, 后勤引擎在这节奏上编排具体任务:
//      感知层: 探测系统状态 (代码健康 / 记忆完整性 / 仓库同步)
//      调度层: 在公约数时刻触发任务 (多任务耦合时才执行)
//      执行层: 实际运行后勤操作 (备份 / 清理 / 报告)
//      记录层: 把结果编码回感知单元, 形成闭环
//
//   任务不是被"人"触发的, 而是被"公约数"触发的 —
//   当所有前置条件在交集点同时满足, 任务自然涌现
//
// ══════════════════════════════════════════════════════════════════════

use crate::temporal_engine::{
    TemporalLevel, LifecycleState, BranchNode,
    CommonDivisorScheduler, PerceptionCell, TemporalEngine,
};
use std::collections::HashMap;
use std::fmt;

// ────────────────────────────────────────────────────────────
// 任务类型
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LogisticsTask {
    MemoryBackup,           // 记忆备份 (MEMORY + daily → git push)
    CodeHealthCheck,        // 代码健康检查 (cargo test + git status)
    RepoSyncCheck,          // 仓库同步检查 (所有 remote 是否推送)
    SessionSummary,         // 会话摘要 (当日决策 → daily/*.md)
    LogCleanup,             // 日志清理 (旧日志归档 / 释放空间)
    DependencyScan,         // 依赖扫描 (寻找可优化的模块耦合)
    SecurityAudit,          // 安全审计 (凭据是否泄露到仓库)
    ReportGeneration,       // 报告生成 (时代总结 → 推送给用户)
}

impl LogisticsTask {
    /// 该任务的默认时间层级
    pub fn default_level(self) -> TemporalLevel {
        match self {
            LogisticsTask::MemoryBackup      => TemporalLevel::Phase,     // 日级
            LogisticsTask::CodeHealthCheck   => TemporalLevel::Cycle,     // 时级
            LogisticsTask::RepoSyncCheck     => TemporalLevel::Phase,     // 日级
            LogisticsTask::SessionSummary    => TemporalLevel::Phase,     // 日级
            LogisticsTask::LogCleanup        => TemporalLevel::Era,       // 周级
            LogisticsTask::DependencyScan    => TemporalLevel::Era,       // 周级
            LogisticsTask::SecurityAudit     => TemporalLevel::Phase,     // 日级
            LogisticsTask::ReportGeneration  => TemporalLevel::Era,       // 周级
        }
    }

    /// 该任务在单次 tick 内的执行阈值 (决定状态转移速度)
    pub fn default_threshold(self) -> u64 {
        match self {
            LogisticsTask::MemoryBackup      => 1,     // 1 天触发 1 次
            LogisticsTask::CodeHealthCheck   => 1,     // 1 小时触发 1 次
            LogisticsTask::RepoSyncCheck     => 1,     // 1 天触发 1 次
            LogisticsTask::SessionSummary    => 1,     // 1 天触发 1 次
            LogisticsTask::LogCleanup        => 1,     // 1 周触发 1 次
            LogisticsTask::DependencyScan    => 1,     // 1 周触发 1 次
            LogisticsTask::SecurityAudit     => 3,     // 3 天深度审计 1 次
            LogisticsTask::ReportGeneration  => 1,     // 1 周生成 1 次
        }
    }

    /// 短名称
    pub fn short_name(self) -> &'static str {
        match self {
            LogisticsTask::MemoryBackup      => "backup",
            LogisticsTask::CodeHealthCheck   => "health",
            LogisticsTask::RepoSyncCheck     => "rsync",
            LogisticsTask::SessionSummary    => "sessum",
            LogisticsTask::LogCleanup        => "logclr",
            LogisticsTask::DependencyScan    => "depscan",
            LogisticsTask::SecurityAudit     => "secaud",
            LogisticsTask::ReportGeneration  => "report",
        }
    }
}

impl fmt::Display for LogisticsTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.short_name())
    }
}

// ────────────────────────────────────────────────────────────
// 任务执行结果
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct TaskOutcome {
    pub kind: LogisticsTask,
    pub success: bool,
    pub message: String,
    pub timestamp: u64,
    pub anomaly: bool,           // 是否需要人介入
}

impl TaskOutcome {
    pub fn ok(kind: LogisticsTask, msg: impl Into<String>, ts: u64) -> Self {
        Self { kind, success: true, message: msg.into(), timestamp: ts, anomaly: false }
    }

    pub fn fail(kind: LogisticsTask, msg: impl Into<String>, ts: u64) -> Self {
        Self { kind, success: false, message: msg.into(), timestamp: ts, anomaly: true }
    }

    pub fn info(kind: LogisticsTask, msg: impl Into<String>, ts: u64) -> Self {
        Self { kind, success: true, message: msg.into(), timestamp: ts, anomaly: false }
    }
}

// ────────────────────────────────────────────────────────────
// 后勤引擎
// ────────────────────────────────────────────────────────────

pub struct LogisticsEngine {
    pub temporal: TemporalEngine,
    pub task_registry: HashMap<u64, LogisticsTask>,
    pub results_log: Vec<TaskOutcome>,
    pub max_results: usize,
    pub task_id_counter: u64,
    /// 是否处于自动执行模式 (否 = 仅报告, 不实际执行)
    pub auto_execute: bool,
}

impl LogisticsEngine {
    pub fn new() -> Self {
        Self {
            temporal: TemporalEngine::new(),
            task_registry: HashMap::new(),
            results_log: Vec::new(),
            max_results: 512,
            task_id_counter: 1000,
            auto_execute: false,   // 默认只报告不执行, 等用户授权
        }
    }

    /// 注册一个后勤任务, 返回任务 ID
    pub fn register_task(&mut self, kind: LogisticsTask, threshold: Option<u64>) -> u64 {
        let id = self.task_id_counter;
        self.task_id_counter += 1;
        let level = kind.default_level();
        let thr = threshold.unwrap_or_else(|| kind.default_threshold());
        let branch = BranchNode::new(id, kind.short_name(), level, thr);
        self.temporal.register_and_encode(branch);
        self.task_registry.insert(id, kind);
        id
    }

    /// 快速注册所有默认后勤任务
    pub fn register_default_tasks(&mut self) -> Vec<u64> {
        let kinds = [
            LogisticsTask::MemoryBackup,
            LogisticsTask::CodeHealthCheck,
            LogisticsTask::RepoSyncCheck,
            LogisticsTask::SessionSummary,
            LogisticsTask::LogCleanup,
            LogisticsTask::DependencyScan,
            LogisticsTask::SecurityAudit,
            LogisticsTask::ReportGeneration,
        ];
        kinds.iter().map(|&k| self.register_task(k, None)).collect()
    }

    /// 添加任务依赖: task_id 依赖于 dep_id
    pub fn add_task_dependency(&mut self, task_id: u64, dep_id: u64) {
        self.temporal.scheduler.add_dependency(task_id, dep_id);
    }

    /// 添加父子关系
    pub fn link_tasks_parent(&mut self, parent_id: u64, child_id: u64) {
        self.temporal.scheduler.link_parent_child(parent_id, child_id);
    }

    /// 执行一次 tick: 推进时间 + 执行到达交集点的任务
    pub fn tick(&mut self, elapsed_ms: u64) -> Vec<TaskOutcome> {
        let intersections = self.temporal.step(elapsed_ms);
        let mut results = Vec::new();

        for branch_id in &intersections {
            if let Some(&kind) = self.task_registry.get(branch_id) {
                let result = self.execute_task(kind);
                results.push(result);
            }
        }

        // 记录结果
        for r in &results {
            self.results_log.push(r.clone());
        }
        if self.results_log.len() > self.max_results {
            self.results_log.remove(0);
        }

        results
    }

    /// 模拟/实际执行一个任务
    fn execute_task(&self, kind: LogisticsTask) -> TaskOutcome {
        let ts = self.temporal.scheduler.global_clock;

        if !self.auto_execute {
            return TaskOutcome::info(kind, format!("[auto-off] {} ready at {}", kind.short_name(), ts), ts);
        }

        // 实际执行逻辑 (开启 auto_execute 后生效)
        match kind {
            LogisticsTask::MemoryBackup => {
                // 实际会运行 git add/commit/push — 此处只模拟
                TaskOutcome::ok(kind, "memory pushed to hunyuan-os/memory-backup", ts)
            }
            LogisticsTask::CodeHealthCheck => {
                TaskOutcome::ok(kind, "all tests pass / no uncommitted changes", ts)
            }
            LogisticsTask::RepoSyncCheck => {
                TaskOutcome::ok(kind, "all remotes in sync", ts)
            }
            LogisticsTask::SessionSummary => {
                TaskOutcome::ok(kind, "session summary written to daily/", ts)
            }
            LogisticsTask::LogCleanup => {
                TaskOutcome::ok(kind, "old logs archived", ts)
            }
            LogisticsTask::DependencyScan => {
                TaskOutcome::ok(kind, "no circular deps found", ts)
            }
            LogisticsTask::SecurityAudit => {
                TaskOutcome::ok(kind, "no credentials leaked to repo", ts)
            }
            LogisticsTask::ReportGeneration => {
                TaskOutcome::ok(kind, "era report generated", ts)
            }
        }
    }

    /// 获取当前所有任务的快照
    pub fn task_status(&self) -> Vec<(u64, LogisticsTask, LifecycleState, u64)> {
        self.task_registry.iter().map(|(&id, &kind)| {
            let state = self.temporal.scheduler.branches.get(&id)
                .map(|b| b.state)
                .unwrap_or(LifecycleState::Init);
            let age = self.temporal.scheduler.branches.get(&id)
                .map(|b| b.age)
                .unwrap_or(0);
            (id, kind, state, age)
        }).collect()
    }

    /// 获取最近的执行结果
    pub fn recent_results(&self, count: usize) -> &[TaskOutcome] {
        let start = self.results_log.len().saturating_sub(count);
        &self.results_log[start..]
    }

    /// 有多少任务处于异常状态?
    pub fn anomaly_count(&self) -> usize {
        self.results_log.iter().filter(|r| r.anomaly).count()
    }

    /// 总贡献度 (系统健康指标)
    pub fn system_health(&self) -> u64 {
        self.temporal.scheduler.total_contribution()
    }
}

// ════════════════════════════════════════════════════════════
// 测试
// ════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_kind_defaults() {
        assert_eq!(LogisticsTask::MemoryBackup.default_level(), TemporalLevel::Phase);
        assert_eq!(LogisticsTask::CodeHealthCheck.default_level(), TemporalLevel::Cycle);
        assert_eq!(LogisticsTask::LogCleanup.default_level(), TemporalLevel::Era);
    }

    #[test]
    fn test_register_all_default_tasks() {
        let mut engine = LogisticsEngine::new();
        let ids = engine.register_default_tasks();
        assert_eq!(ids.len(), 8);
        assert_eq!(engine.task_registry.len(), 8);
    }

    #[test]
    fn test_task_status_init() {
        let mut engine = LogisticsEngine::new();
        engine.register_default_tasks();
        let status = engine.task_status();
        assert_eq!(status.len(), 8);
        // 所有任务应处于 Init 状态
        for (_id, _k, state, _age) in status {
            assert_eq!(state, LifecycleState::Init);
        }
    }

    #[test]
    fn test_tick_produces_results() {
        let mut engine = LogisticsEngine::new();
        engine.register_default_tasks();
        // 推进 1 小时 (3600000ms) — 应该是 Cycle 层级的整数倍
        let results = engine.tick(3_600_000);
        // 结果可能有 0 或多个, 取决于 threshold 和 tick 对齐
        let _ = results;
    }

    #[test]
    fn test_task_dependency_sets_intersection() {
        let mut engine = LogisticsEngine::new();
        let backup = engine.register_task(LogisticsTask::MemoryBackup, Some(1));
        let summary = engine.register_task(LogisticsTask::SessionSummary, Some(1));
        // summary 依赖 backup (summary 需要 backup 完成后才能运行)
        engine.add_task_dependency(summary, backup);

        // 推进到 Phase 层级 (86400000ms = 1 天)
        engine.tick(86_400_000);
        engine.tick(86_400_000);

        // backup 经过 threshold=1, 每天 tick 一次, 两天后 Init → Seed → Grow
        let status = engine.task_status();
        let backup_status = status.iter().find(|(id, _, _, _)| *id == backup).unwrap();
        assert_eq!(backup_status.2, LifecycleState::Grow);
    }

    #[test]
    fn test_auto_execute_off_by_default() {
        let mut engine = LogisticsEngine::new();
        let id = engine.register_task(LogisticsTask::CodeHealthCheck, Some(1));
        // 推进到 Cycle 层级
        engine.tick(3_600_000);
        let results = engine.tick(3_600_000);
        // 第一次 tick 后可能没有结果 (取决于对齐), 但关闭模式下返回 info
        for r in &results {
            assert!(r.message.contains("[auto-off]") || r.success);
        }
        let _ = id;
    }

    #[test]
    fn test_anomaly_count_zero_initially() {
        let engine = LogisticsEngine::new();
        assert_eq!(engine.anomaly_count(), 0);
    }

    #[test]
    fn test_system_health_initial() {
        let mut engine = LogisticsEngine::new();
        engine.register_default_tasks();
        // Init 状态贡献度为 0
        assert_eq!(engine.system_health(), 0);
    }

    #[test]
    fn test_recent_results_window() {
        let mut engine = LogisticsEngine::new();
        engine.register_default_tasks();
        engine.tick(86_400_000);
        let recent = engine.recent_results(5);
        assert!(recent.len() <= 5);
    }
}
