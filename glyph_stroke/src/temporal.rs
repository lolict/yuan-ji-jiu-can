// ═══════════════════════════════════════════════════════════════════════════════
// temporal.rs — 时间触发引擎 · 容器=时间 · 时序调度
// ═══════════════════════════════════════════════════════════════════════════════
//
// 核心原则:
//   容器=时间, 编码执行由时间条件触发
//   时钟动作: 意志驱动时间, 结束即开始
//   时间线线性推进, 不允许分叉
//   触发规则: 时间条件满足 → 容器操作激活
//
// 触发类型:
//   Absolute   — 在指定时刻触发
//   Relative   — 距上次触发后 N 周期
//   Condition  — 容器状态满足条件时
//   Recurrence — 周期性循环
//
// ═══════════════════════════════════════════════════════════════════════════════

use crate::container_runtime::ContainerOS;

/// 时间戳 (从系统起点开始的 tick 计数)
pub type Tick = u64;

/// 触发条件类型
#[derive(Clone, Debug, PartialEq)]
pub enum TriggerKind {
    /// 绝对时间触发
    Absolute(Tick),
    /// 相对时间触发 (每 N tick)
    Relative(Tick),
    /// 条件触发: 容器 cell 某位被置位
    ContainerActive(u8),
    /// 深度达到阈值时触发
    DepthReached(u32),
    /// 极性与指定值匹配时
    MatchPolarity(bool),
}

/// 触发规则 = 条件 + 动作
#[derive(Clone, Debug)]
pub struct TriggerRule {
    pub kind: TriggerKind,
    /// 触发的操作 (容器索引列表)
    pub actions: Vec<u8>,
    /// 触发后的回调描述
    pub label: String,
    /// 是否已触发
    pub fired: bool,
    /// 触发次数计数
    pub fire_count: u32,
    /// 相对模式下的周期计数器
    pub period_counter: Tick,
}

impl TriggerRule {
    pub fn new(kind: TriggerKind, actions: Vec<u8>, label: &str) -> Self {
        Self {
            kind,
            actions,
            label: label.to_string(),
            fired: false,
            fire_count: 0,
            period_counter: 0,
        }
    }

    /// 检查是否应该触发
    pub fn check(&mut self, os: &ContainerOS, current_tick: Tick) -> bool {
        let should_fire = match &self.kind {
            TriggerKind::Absolute(t) => current_tick >= *t,
            TriggerKind::Relative(period) => {
                self.period_counter += 1;
                self.period_counter >= *period
            }
            TriggerKind::ContainerActive(idx) => os.cell() & (1u64 << *idx) != 0,
            TriggerKind::DepthReached(d) => os.depth() >= *d,
            TriggerKind::MatchPolarity(is_yang) => {
                let pol = (os.cell() >> 62) & 1;
                (*is_yang && pol == 1) || (!*is_yang && pol == 0)
            }
        };

        if should_fire && (self.period_counter == 0 || matches!(self.kind, TriggerKind::Relative(_))) {
            self.fired = true;
            self.fire_count += 1;
            if matches!(self.kind, TriggerKind::Relative(_)) {
                self.period_counter = 0;
            }
            true
        } else if !matches!(self.kind, TriggerKind::Relative(_)) {
            should_fire
        } else {
            false
        }
    }
}

/// 时间线事件
#[derive(Clone, Debug)]
pub struct TimeEvent {
    pub tick: Tick,
    pub action: TimeAction,
    pub completed: bool,
}

/// 时间动作类型
#[derive(Clone, Debug)]
pub enum TimeAction {
    /// 激活容器
    Activate(u8, String),
    /// 执行容器程序
    Execute(String),
    /// 记录快照
    Snapshot(String),
    /// 触发仲裁
    Arbitrate(String),
}

/// 时间线管理器
pub struct Timeline {
    pub current_tick: Tick,
    pub events: Vec<TimeEvent>,
    pub triggers: Vec<TriggerRule>,
    pub event_log: Vec<(Tick, String)>,
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            current_tick: 0,
            events: Vec::new(),
            triggers: Vec::new(),
            event_log: Vec::new(),
        }
    }

    /// 添加触发规则
    pub fn add_trigger(&mut self, rule: TriggerRule) {
        self.triggers.push(rule);
    }

    /// 添加时间事件
    pub fn schedule(&mut self, tick: Tick, action: TimeAction) {
        self.events.push(TimeEvent { tick, action, completed: false });
        // 保持有序
        self.events.sort_by_key(|e| e.tick);
    }

    /// 推进一个 tick, 执行所有到期事件, 检查触发规则
    pub fn step(&mut self, os: &mut ContainerOS) -> Vec<String> {
        self.current_tick += 1;
        let mut results = Vec::new();
        let tick = self.current_tick;

        // 执行到期事件
        for event in self.events.iter_mut() {
            if !event.completed && event.tick <= tick {
                event.completed = true;
                let desc = match &event.action {
                    TimeAction::Activate(idx, name) => {
                        os.absorb(*idx);
                        format!("tick {}: 激活容器[{}] ({})", tick, idx, name)
                    }
                    TimeAction::Execute(prog) => {
                        format!("tick {}: 执行程序[{}]", tick, prog)
                    }
                    TimeAction::Snapshot(name) => {
                        format!("tick {}: 快照 cell=0x{:04X} ({})", tick, os.cell() & 0xFFFF, name)
                    }
                    TimeAction::Arbitrate(label) => {
                        format!("tick {}: 裁决[{}]", tick, label)
                    }
                };
                results.push(desc.clone());
                self.event_log.push((tick, desc));
            }
        }

        // 检查触发规则
        for trigger in self.triggers.iter_mut() {
            if trigger.check(os, tick) {
                for &action in &trigger.actions {
                    os.absorb(action);
                }
                let msg = format!("tick {}: TRIGGER fired [{}] (count={})",
                    tick, trigger.label, trigger.fire_count);
                results.push(msg.clone());
                self.event_log.push((tick, msg));
            }
        }

        results
    }

    /// 运行多个 tick
    pub fn run(&mut self, os: &mut ContainerOS, ticks: Tick) -> Vec<String> {
        let mut all_results = Vec::new();
        for _ in 0..ticks {
            let mut r = self.step(os);
            all_results.append(&mut r);
        }
        all_results
    }

    /// 获取事件日志
    pub fn log(&self) -> &[(Tick, String)] {
        self.event_log.as_slice()
    }

    /// 导出时间线日志
    pub fn export_log(&self) -> String {
        let mut buf = String::new();
        buf.push_str("═══ 时间线日志 ═══\n");
        buf.push_str(&format!("当前 tick: {}\n", self.current_tick));
        buf.push_str(&format!("事件总数: {} · 触发规则: {}\n\n", self.events.len(), self.triggers.len()));
        for (tick, desc) in &self.event_log {
            buf.push_str(&format!("  {:4} | {}\n", tick, desc));
        }
        buf
    }
}

impl Default for Timeline {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod temporal_tests {
    use super::*;

    #[test]
    fn test_timeline_basic() {
        let mut tl = Timeline::new();
        let mut os = ContainerOS::new();

        tl.schedule(3, TimeAction::Activate(0, "袋".to_string()));
        tl.schedule(5, TimeAction::Snapshot("s1".to_string()));

        let results = tl.run(&mut os, 7);

        assert_eq!(tl.current_tick, 7);
        assert!(os.cell() & 1 != 0); // container 0 activated
        assert!(results.len() >= 2);
    }

    #[test]
    fn test_trigger_rule() {
        let mut tl = Timeline::new();
        let mut os = ContainerOS::new();

        let mut rule = TriggerRule::new(
            TriggerKind::DepthReached(3),
            vec![5, 7],
            "depth3"
        );
        tl.add_triggers(rule.clone());

        os.absorb(0); os.absorb(1); // depth grows
        let results = tl.step(&mut os);
        // depth should be 2 after 2 absorbs on empty os
        assert_eq!(os.depth(), 2);
    }

    #[test]
    fn test_trigger_recurrence() {
        let mut tl = Timeline::new();
        let mut os = ContainerOS::new();

        let rule = TriggerRule::new(
            TriggerKind::Relative(3),
            vec![10],
            "每3tick"
        );
        tl.add_trigger(rule);

        let results = tl.run(&mut os, 10);
        // Should have fired at tick 3, 6, 9
        let trigger_fires: Vec<_> = results.iter().filter(|r| r.contains("TRIGGER")).collect();
        assert!(trigger_fires.len() >= 2);
    }
}

impl Timeline {
    /// Helper to add a single trigger (avoids collect)
    pub fn add_triggers(&mut self, rule: TriggerRule) {
        self.triggers.push(rule);
    }
}
