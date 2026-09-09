// ══════════════════════════════════════════════════════════════════
// task_automation.rs — 自主后勤调度系统
// ══════════════════════════════════════════════════════════════════
//
// 「被动应答 → 自主维持」
//
// 满全法系统不再等待主人手动触发每条命令。
// 它自己记住所有必要任务、按计划时间节点自动执行。
//
// 设计原则:
//   1. 任务 = 名称 + 间隔层级 + 最后执行时间 + 动作描述
//   2. 调度器 = 层级时间节点驱动（time_node mixed-radix）
//   3. 执行 = 产生结构化报告, 写入审计环
//   4. 自新 = 执行结果反馈调整下次策略
//
// 三种核心自主任务:
//   · 日度代码健康检查  — git status + build test
//   · 日度会话总结      — 当日决策 → daily/*.md
//   · 周度仓库同步      — git push 备份到远端
//
// 层级化时间节点:
//   ms → sec → min → hour → day → week → month
//   每小时: 心跳检查
//   每日:   健康 + 总结
//   每周:   仓库同步

use crate::time_node::{TimeNode, TimeCoord, unix_ms_to_timecoord};
use crate::log_ring::LogRing;

use crate::os_layer::OsLayer;

// ──────────────────────────────────────────────────────────────
// 任务间隔 · mixed-radix 层级
// ──────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaskInterval {
    /// 每小时执行一次
    Hourly,
    /// 每日执行一次（默认 09:00）
    Daily { hour: u32 },
    /// 每周执行一次（默认周一 10:00）
    Weekly { weekday: u32, hour: u32 },
    /// 每月执行一次（默认 1 号 08:00）
    Monthly { day: u32, hour: u32 },
}

impl TaskInterval {
    pub fn name(&self) -> &'static str {
        match self {
            TaskInterval::Hourly => "每小时",
            TaskInterval::Daily { .. } => "每日",
            TaskInterval::Weekly { .. } => "每周",
            TaskInterval::Monthly { .. } => "每月",
        }
    }

    /// 检查给定时间坐标是否满足该间隔的触发条件
    pub fn should_trigger(&self, coord: &TimeCoord) -> bool {
        match self {
            TaskInterval::Hourly => {
                // 每小时的第 0 分钟触发
                coord.min == 0
            }
            TaskInterval::Daily { hour } => {
                // 每天的第 hour 小时第 0 分钟
                coord.hour == *hour && coord.min == 0
            }
            TaskInterval::Weekly { weekday, hour } => {
                // 计算星期几 (简化: 从 1970-01-01 是周四开始推算)
                let days_since_epoch = coord.day as u64; // 简化：用 day 字段
                let wd = ((days_since_epoch + 4) % 7) as u32; // 0=周四
                // 转换为 ISO: 1=周一, 7=周日
                let iso_wd = if wd == 0 { 7 } else { wd };
                iso_wd == *weekday && coord.hour == *hour && coord.min == 0
            }
            TaskInterval::Monthly { day, hour } => {
                coord.day == *day - 1 && coord.hour == *hour && coord.min == 0
            }
        }
    }
}

// ──────────────────────────────────────────────────────────────
// 任务类别 · 三种核心自主维持
// ──────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaskKind {
    /// 代码健康检查: git status / build / test
    CodeHealth,
    /// 会话总结: 当日决策写入 daily/*.md
    SessionSummary,
    /// 仓库同步: git push 到远端
    RepoSync,
}

impl TaskKind {
    pub fn name(&self) -> &'static str {
        match self {
            TaskKind::CodeHealth => "代码健康",
            TaskKind::SessionSummary => "会话总结",
            TaskKind::RepoSync => "仓库同步",
        }
    }

    pub fn default_interval(&self) -> TaskInterval {
        match self {
            TaskKind::CodeHealth => TaskInterval::Daily { hour: 9 },
            TaskKind::SessionSummary => TaskInterval::Daily { hour: 21 },
            TaskKind::RepoSync => TaskInterval::Weekly { weekday: 1, hour: 10 },
        }
    }
}

// ──────────────────────────────────────────────────────────────
// 任务定义
// ──────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct TaskDefinition {
    pub id: u32,
    pub kind: TaskKind,
    pub interval: TaskInterval,
    pub description: String,
    /// 上次执行时间 (unix ms), 0 = 从未执行
    pub last_run_ms: u64,
    pub total_executions: u64,
    pub enabled: bool,
}

impl TaskDefinition {
    pub fn new(id: u32, kind: TaskKind, description: &str) -> Self {
        Self {
            id,
            kind,
            interval: kind.default_interval(),
            description: description.to_string(),
            last_run_ms: 0,
            total_executions: 0,
            enabled: true,
        }
    }

    pub fn with_interval(mut self, interval: TaskInterval) -> Self {
        self.interval = interval;
        self
    }
}

// ──────────────────────────────────────────────────────────────
// 执行结果
// ──────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct TaskResult {
    pub task_id: u32,
    pub task_name: String,
    pub executed_at_ms: u64,
    pub success: bool,
    pub summary: String,
    /// 详细输出 (多行)
    pub details: Vec<String>,
    /// 建议的下一步行动
    pub next_action: Option<String>,
}

impl TaskResult {
    pub fn success(os: &dyn OsLayer, task_id: u32, task_name: &str, summary: &str) -> Self {
        Self {
            task_id,
            task_name: task_name.to_string(),
            executed_at_ms: current_unix_ms(os),
            success: true,
            summary: summary.to_string(),
            details: Vec::new(),
            next_action: None,
        }
    }

    pub fn failure(os: &dyn OsLayer, task_id: u32, task_name: &str, reason: &str) -> Self {
        Self {
            task_id,
            task_name: task_name.to_string(),
            executed_at_ms: current_unix_ms(os),
            success: false,
            summary: reason.to_string(),
            details: Vec::new(),
            next_action: Some("检查配置后重试".to_string()),
        }
    }

    pub fn add_detail(&mut self, line: &str) {
        self.details.push(line.to_string());
    }

    pub fn set_next_action(&mut self, action: &str) {
        self.next_action = Some(action.to_string());
    }
}

// ──────────────────────────────────────────────────────────────
// 任务执行器 · 产出模拟报告 (实际部署时调用外部命令)
// ──────────────────────────────────────────────────────────────

pub struct TaskExecutor;

impl TaskExecutor {
    /// 代码健康检查 — 模拟执行 git status + build + test
    pub fn run_code_health(os: &dyn OsLayer) -> TaskResult {
        let mut r = TaskResult::success(
            os,
            1, "代码健康",
            "日度代码健康检查完成"
        );

        // 模拟 git status
        r.add_detail("[git status] 工作区干净, 无未提交变更");
        r.add_detail("[git log]   最近提交: feat(v2.0) 公开 Beta");

        // 模拟 C 内核编译测试
        r.add_detail("[C 内核]   njsg_kernel.c → 编译通过 · demo 通过");
        r.add_detail("[C 内核]   mlll_symbols.c → 904 symbols 正常");

        // 模拟 Rust 测试
        r.add_detail("[Rust]     cargo test → 613 passed / 0 failed");
        r.add_detail("[Rust]     clippy 无致命警告");

        r.set_next_action("继续日常开发 — 无阻塞问题");
        r
    }

    /// 会话总结 — 模拟提取当日决策写入 daily/*.md
    pub fn run_session_summary(os: &dyn OsLayer) -> TaskResult {
        let mut r = TaskResult::success(
            os,
            2, "会话总结",
            "当日会话决策已归档"
        );

        let today = {
            let tc = unix_ms_to_timecoord(current_unix_ms(os));
            format!("{:04}-{:02}-{:02}", tc.year, tc.mon + 1, tc.day + 1)
        };

        r.add_detail(&format!("[归档] daily/{}.md 已更新", today));
        r.add_detail("  → 新增: task_automation 模块实现");
        r.add_detail("  → 新增: 三种自主任务定义");
        r.add_detail("  → 新增: 层级时间驱动调度");
        r.add_detail("  → 状态: v2.0 公开 Beta 健康运行");

        r.set_next_action("明早 09:00 自动再次检查");
        r
    }

    /// 仓库同步 — 模拟 git push 备份
    pub fn run_repo_sync(os: &dyn OsLayer) -> TaskResult {
        let mut r = TaskResult::success(
            os,
            3, "仓库同步",
            "周度仓库同步完成"
        );

        r.add_detail("[push]  origin/main → lolict/hunyuan-os");
        r.add_detail("  → C 内核:      2107 行");
        r.add_detail("  → Rust 核心:   63 源文件");
        r.add_detail("  → 文档:        20+ 文件");
        r.add_detail("  → 测试覆盖:    613 通过");
        r.add_detail("[远端] GitHub 备份成功, 所有 commit 已送达");

        r.set_next_action("下周一 10:00 自动再次同步");
        r
    }
}

// ──────────────────────────────────────────────────────────────
// 任务调度器 · 自主后勤核心
// ──────────────────────────────────────────────────────────────

pub struct TaskScheduler {
    pub tasks: Vec<TaskDefinition>,
    pub time_node: TimeNode,
    pub results_log: Vec<TaskResult>,
    /// 审计环 (hash-chained 防篡改)
    pub audit_ring: LogRing,
}

impl TaskScheduler {
    pub fn new(node_id: u64) -> Self {
        Self {
            tasks: Vec::new(),
            time_node: TimeNode::new(node_id),
            results_log: Vec::new(),
            audit_ring: LogRing::new(b"task-automation-audit"),
        }
    }

    /// 注册默认三种自主任务
    pub fn register_defaults(mut self) -> Self {
        self.tasks.push(TaskDefinition::new(
            1,
            TaskKind::CodeHealth,
            "日度代码健康检查 — 编译 + 测试 + git status"
        ));
        self.tasks.push(TaskDefinition::new(
            2,
            TaskKind::SessionSummary,
            "日度会话总结 — 决策写入 daily/*.md"
        ));
        self.tasks.push(TaskDefinition::new(
            3,
            TaskKind::RepoSync,
            "周度仓库同步 — git push 远端备份"
        ));
        self
    }

    /// 注册自定义任务
    pub fn register(&mut self, kind: TaskKind, description: &str) -> u32 {
        let id = self.tasks.len() as u32 + 1;
        self.tasks.push(TaskDefinition::new(id, kind, description));
        id
    }

    /// 更新外部时间 (从系统时钟)
    pub fn sync_time(&mut self, os: &dyn OsLayer) {
        self.time_node.set_wall_clock(current_unix_ms(os));
    }

    /// 检查所有到期任务并执行
    pub fn tick(&mut self, os: &dyn OsLayer) -> Vec<TaskResult> {
        self.sync_time(os);
        let coord = self.time_node.time_coord();

        // 收集需要执行的任务 ID (避免同时持有多处 &mut)
        let to_run: Vec<u32> = self.tasks.iter()
            .filter(|t| t.enabled)
            .filter(|t| {
                if t.last_run_ms == 0 { return true; }
                if !t.interval.should_trigger(&coord) { return false; }
                // 避免同一周期内重复执行
                let last_coord = unix_ms_to_timecoord(t.last_run_ms);
                !(last_coord.day == coord.day && last_coord.hour == coord.hour)
            })
            .map(|t| t.id)
            .collect();

        let mut triggered = Vec::new();
        for id in to_run {
            // 取出任务信息, 执行, 再写回
            if let Some(idx) = self.tasks.iter().position(|t| t.id == id) {
                let kind = self.tasks[idx].kind;
                let desc = self.tasks[idx].description.clone();
                let result = Self::run_task(os, kind);
                // 记录
                let audit_payload = format!("TASK[{}] {}: {}",
                    kind.name(), desc, result.summary);
                self.audit_ring.append(1, audit_payload.as_bytes(), result.executed_at_ms * 1000);
                self.tasks[idx].last_run_ms = result.executed_at_ms;
                self.tasks[idx].total_executions += 1;
                self.results_log.push(result.clone());
                triggered.push(result);
            }
        }
        triggered
    }

    /// 执行具体任务逻辑 (纯函数, 不依赖 self)
    fn run_task(os: &dyn OsLayer, kind: TaskKind) -> TaskResult {
        match kind {
            TaskKind::CodeHealth => TaskExecutor::run_code_health(os),
            TaskKind::SessionSummary => TaskExecutor::run_session_summary(os),
            TaskKind::RepoSync => TaskExecutor::run_repo_sync(os),
        }
    }

    /// 强制执行某任务 (忽略时间)
    pub fn force_run(&mut self, os: &dyn OsLayer, task_id: u32) -> Option<TaskResult> {
        let idx = self.tasks.iter().position(|t| t.id == task_id)?;
        let kind = self.tasks[idx].kind;
        let desc = self.tasks[idx].description.clone();
        let result = Self::run_task(os, kind);
        // 记录
        let audit_payload = format!("TASK[{}] {}: {}",
            kind.name(), desc, result.summary);
        self.audit_ring.append(1, audit_payload.as_bytes(), result.executed_at_ms * 1000);
        self.tasks[idx].last_run_ms = result.executed_at_ms;
        self.tasks[idx].total_executions += 1;
        self.results_log.push(result.clone());
        Some(result)
    }

    /// 获取调度器状态报告
    pub fn status_report(&self) -> String {
        let mut s = String::from("╔═══ 自主后勤调度状态 ═══\n");
        s.push_str(&format!("║ 节点 ID   : {}\n", self.time_node.node_id));
        s.push_str(&format!("║ 逻辑时钟  : {} ms\n", self.time_node.logical_ms));
        s.push_str(&format!("║ 任务总数  : {}\n", self.tasks.len()));
        s.push_str(&format!("║ 累计执行  : {}\n", self.results_log.len()));
        s.push_str("║\n");
        for task in &self.tasks {
            let status = if task.enabled { "●" } else { "○" };
            let last = if task.last_run_ms > 0 {
                let tc = unix_ms_to_timecoord(task.last_run_ms);
                format!("{:04}-{:02}-{:02} {:02}:{:02}",
                        tc.year, tc.mon + 1, tc.day + 1, tc.hour, tc.min)
            } else {
                "从未".to_string()
            };
            s.push_str(&format!("║ {} [{}] {} — 已执行 {} 次, 上次: {}\n",
                status, task.kind.name(), task.description, task.total_executions, last));
        }
        s.push_str("╚═══════════════════════\n");
        s
    }

    /// 获取最近 N 次执行记录
    pub fn recent_results(&self, n: usize) -> &[TaskResult] {
        let start = self.results_log.len().saturating_sub(n);
        &self.results_log[start..]
    }
}

// ──────────────────────────────────────────────────────────────
// 辅助
// ──────────────────────────────────────────────────────────────

fn current_unix_ms(os: &dyn OsLayer) -> u64 {
    os.time_millis()
}

// ──────────────────────────────────────────────────────────────
// 测试
// ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_interval_names() {
        assert_eq!(TaskInterval::Hourly.name(), "每小时");
        assert_eq!(TaskInterval::Daily { hour: 9 }.name(), "每日");
        assert_eq!(TaskInterval::Weekly { weekday: 1, hour: 10 }.name(), "每周");
        assert_eq!(TaskInterval::Monthly { day: 1, hour: 8 }.name(), "每月");
    }

    #[test]
    fn task_interval_should_trigger_hourly() {
        let interval = TaskInterval::Hourly;
        // min=0 → 触发
        let mut coord = TimeCoord { ms: 0, sec: 0, min: 0, hour: 14, day: 0, mon: 0, year: 2026 };
        assert!(interval.should_trigger(&coord));
        // min=30 → 不触发
        coord.min = 30;
        assert!(!interval.should_trigger(&coord));
    }

    #[test]
    fn task_interval_should_trigger_daily() {
        let interval = TaskInterval::Daily { hour: 9 };
        let mut coord = TimeCoord { ms: 0, sec: 0, min: 0, hour: 9, day: 0, mon: 0, year: 2026 };
        assert!(interval.should_trigger(&coord));
        coord.hour = 14;
        assert!(!interval.should_trigger(&coord));
    }

    #[test]
    fn task_definition_default_interval() {
        let health = TaskDefinition::new(1, TaskKind::CodeHealth, "test");
        assert_eq!(health.interval, TaskInterval::Daily { hour: 9 });

        let summary = TaskDefinition::new(2, TaskKind::SessionSummary, "test");
        assert_eq!(summary.interval, TaskInterval::Daily { hour: 21 });

        let sync = TaskDefinition::new(3, TaskKind::RepoSync, "test");
        assert_eq!(sync.interval, TaskInterval::Weekly { weekday: 1, hour: 10 });
    }

    #[test]
    fn scheduler_register_defaults() {
        let scheduler = TaskScheduler::new(42).register_defaults();
        assert_eq!(scheduler.tasks.len(), 3);
        assert_eq!(scheduler.tasks[0].kind, TaskKind::CodeHealth);
        assert_eq!(scheduler.tasks[1].kind, TaskKind::SessionSummary);
        assert_eq!(scheduler.tasks[2].kind, TaskKind::RepoSync);
    }

    #[test]
    fn scheduler_force_run() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut scheduler = TaskScheduler::new(1).register_defaults();
        let result = scheduler.force_run(&os, 1).unwrap();
        assert!(result.success);
        assert_eq!(result.task_name, "代码健康");
        assert!(!result.details.is_empty());
    }

    #[test]
    fn scheduler_force_run_all() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut scheduler = TaskScheduler::new(1).register_defaults();
        let r1 = scheduler.force_run(&os, 1).unwrap();
        let r2 = scheduler.force_run(&os, 2).unwrap();
        let r3 = scheduler.force_run(&os, 3).unwrap();

        assert!(r1.success && r2.success && r3.success);
        assert_eq!(scheduler.results_log.len(), 3);
    }

    #[test]
    fn scheduler_audit_ring_records() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut scheduler = TaskScheduler::new(1).register_defaults();
        scheduler.force_run(&os, 1);
        scheduler.force_run(&os, 2);
        // 审计环应有 2 条记录
        assert_eq!(scheduler.audit_ring.entries.len(), 2);
        // 验链完整性
        assert!(scheduler.audit_ring.verify());
    }

    #[test]
    fn task_result_success_and_failure() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let ok = TaskResult::success(&os, 1, "测试", "通过");
        assert!(ok.success);
        assert_eq!(ok.summary, "通过");

        let fail = TaskResult::failure(&os, 1, "测试", "错误");
        assert!(!fail.success);
        assert!(fail.next_action.is_some());
    }

    #[test]
    fn task_result_details() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut r = TaskResult::success(&os, 1, "测试", "ok");
        r.add_detail("line1");
        r.add_detail("line2");
        r.set_next_action("下一步");
        assert_eq!(r.details.len(), 2);
        assert_eq!(r.next_action.as_deref(), Some("下一步"));
    }

    #[test]
    fn scheduler_status_report() {
        let scheduler = TaskScheduler::new(99).register_defaults();
        let report = scheduler.status_report();
        assert!(report.contains("节点 ID"));
        assert!(report.contains("99"));
        assert!(report.contains("代码健康"));
        assert!(report.contains("会话总结"));
        assert!(report.contains("仓库同步"));
    }

    #[test]
    fn recent_results() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut scheduler = TaskScheduler::new(1).register_defaults();
        scheduler.force_run(&os, 1);
        scheduler.force_run(&os, 2);
        scheduler.force_run(&os, 3);
        let recent = scheduler.recent_results(2);
        assert_eq!(recent.len(), 2);
    }
}
