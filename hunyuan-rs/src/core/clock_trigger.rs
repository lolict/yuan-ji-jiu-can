// ╔══════════════════════════════════════════════════════════════════╗
// ║  v0.86 时钟触发引擎                                              ║
// ║  冰箱模型 · 时间触发 · 意志驱动 · 继承连贯                        ║
// ║                                                                  ║
// ║  核心: 结束即开始, 记忆跟随结束而继续                              ║
// ║  · 冰箱模型: 容器如冰箱, 内含时间触发器, 到点自动加载执行          ║
// ║  · 意志驱动: 不是被动等待, 是时间本身被意志驱动                    ║
// ║  · 继承连贯: 上一次执行的终点是下一次的起点                        ║
// ╚══════════════════════════════════════════════════════════════════╝

use crate::radix_addressing::Code64;
use std::collections::HashMap;

// ══════════════════════════════════════════════════════════════
// 触发条件 — 不是时间轮询, 是"到点即发"
// ══════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TriggerKind {
    /// 一次性触发 — 到达即执行, 执行后归档
    Once,
    /// 周期触发 — 每次间隔到达即执行
    Periodic,
    /// 条件触发 — 满足地址条件时执行 (冷链不断)
    Conditional,
    /// 连锁触发 — 前一个完成后触发下一个
    Chain,
}

#[derive(Clone, Debug)]
pub struct TriggerCondition {
    pub kind: TriggerKind,
    /// 触发时刻 (tick数)
    pub at_tick: u64,
    /// 周期 (仅 Periodic)
    pub period: Option<u64>,
    /// 条件地址 (仅 Conditional) — 地址匹配时触发
    pub condition_addr: Option<u64>,
    /// 连锁前驱 (仅 Chain) — 前驱触发器ID
    pub chain_prev: Option<u64>,
    /// 已触发次数
    pub fired_count: u64,
    /// 是否已完成 (Once 后自动标记)
    pub done: bool,
}

impl TriggerCondition {
    pub fn once(at_tick: u64) -> Self {
        Self { kind: TriggerKind::Once, at_tick, period: None, condition_addr: None, chain_prev: None, fired_count: 0, done: false }
    }

    pub fn periodic(at_tick: u64, period: u64) -> Self {
        Self { kind: TriggerKind::Periodic, at_tick, period: Some(period), condition_addr: None, chain_prev: None, fired_count: 0, done: false }
    }

    pub fn conditional(at_tick: u64, addr: u64) -> Self {
        Self { kind: TriggerKind::Conditional, at_tick, period: None, condition_addr: Some(addr), chain_prev: None, fired_count: 0, done: false }
    }

    pub fn chain(at_tick: u64, prev: u64) -> Self {
        Self { kind: TriggerKind::Chain, at_tick, period: None, condition_addr: None, chain_prev: Some(prev), fired_count: 0, done: false }
    }

    /// 检查是否应该在当前tick触发
    pub fn should_fire(&self, current_tick: u64) -> bool {
        if self.done { return false; }
        if current_tick < self.at_tick { return false; }
        match self.kind {
            TriggerKind::Once | TriggerKind::Chain | TriggerKind::Conditional => {
                current_tick == self.at_tick
            }
            TriggerKind::Periodic => {
                let period = self.period.unwrap_or(1);
                if period == 0 { return false; }
                (current_tick - self.at_tick) % period == 0
            }
        }
    }

    /// 触发并更新状态
    pub fn fire(&mut self) {
        self.fired_count += 1;
        match self.kind {
            TriggerKind::Once | TriggerKind::Conditional | TriggerKind::Chain => {
                self.done = true;
            }
            TriggerKind::Periodic => {
                if let Some(p) = self.period {
                    if self.fired_count > 0 {
                        self.at_tick += p;
                    }
                }
            }
        }
    }
}

// ══════════════════════════════════════════════════════════════
// 冰箱容器 — 内含触发器, 到点即加载执行
// ══════════════════════════════════════════════════════════════

/// 冰箱容器 — 时间触发的执行单元
#[derive(Clone, Debug)]
pub struct FridgeContainer {
    /// 容器ID — 冰箱号
    pub id: u64,
    /// 触发条件
    pub trigger: TriggerCondition,
    /// 待执行载荷 — 触发时加载的内容
    pub payload: Code64,
    /// 执行结果遗产 — 留给下一次继承
    pub legacy: Option<u64>,
    /// 是否已冷却 (刚触发后需冷却)
    pub cooling: bool,
}

impl FridgeContainer {
    pub fn new(id: u64, trigger: TriggerCondition, payload: Code64) -> Self {
        Self { id, trigger, payload, legacy: None, cooling: false }
    }

    /// 触发并执行 — 加载payload, 产生遗产
    pub fn execute(&mut self, current_tick: u64) -> Option<u64> {
        if !self.trigger.should_fire(current_tick) { return None; }

        self.trigger.fire();
        self.cooling = true;

        // 遗产 = 上一次结果 + 本次载荷值 (加法统计法)
        let result = match self.legacy {
            Some(prev) => prev.saturating_add(self.payload.value() as u64),
            None => self.payload.value() as u64,
        };
        self.legacy = Some(result);
        Some(result)
    }

    /// 冷却完成
    pub fn cool_down(&mut self) {
        self.cooling = false;
    }

    /// 当前应输出的遗产值
    pub fn legacy(&self) -> Option<u64> {
        self.legacy
    }
}

// ══════════════════════════════════════════════════════════════
// 意志驱动器 — 时间被意志驱动, 不是被动等待
// ══════════════════════════════════════════════════════════════

/// 意志 — 持续的驱动力量, 结束即开始
#[derive(Clone, Debug)]
pub struct Will {
    /// 意志标识
    pub id: u64,
    /// 强度 — 驱动时钟的力度 (加法统计法累加)
    pub intensity: u64,
    /// 连续计数 — 意志持续的tick数
    pub continuity: u64,
    /// 上一个终点 — 继承连贯
    pub last_endpoint: Option<u64>,
}

impl Will {
    pub fn new(id: u64) -> Self {
        Self { id, intensity: 1, continuity: 0, last_endpoint: None }
    }

    /// 意志驱动时间 — 每次tick加强连续性
    pub fn drive(&mut self) {
        self.continuity += 1;
        // 强度随连续性增长 (加法, 不是乘法)
        self.intensity = self.intensity.saturating_add(1);
    }

    /// 设置终点 — 结束即开始的下一次起点
    pub fn set_endpoint(&mut self, endpoint: u64) {
        self.last_endpoint = Some(endpoint);
    }

    /// 获取起点 — 上一次终点, 若无则为0
    pub fn start_point(&self) -> u64 {
        self.last_endpoint.unwrap_or(0)
    }
}

/// 意志驱动引擎 — 多个意志并行驱动多个冰箱
pub struct WillEngine {
    wills: HashMap<u64, Will>,
}

impl WillEngine {
    pub fn new() -> Self {
        Self { wills: HashMap::new() }
    }

    pub fn add_will(&mut self, id: u64) {
        self.wills.entry(id).or_insert_with(|| Will::new(id));
    }

    pub fn drive_all(&mut self) {
        for will in self.wills.values_mut() {
            will.drive();
        }
    }

    pub fn get_will(&self, id: u64) -> Option<&Will> {
        self.wills.get(&id)
    }

    pub fn total_intensity(&self) -> u64 {
        self.wills.values().map(|w| w.intensity).fold(0u64, |a, b| a.saturating_add(b))
    }
}

// ══════════════════════════════════════════════════════════════
// 继承链 — 上一次终点是下一次起点, 连续贯穿
// ══════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct InheritanceLink {
    /// 前一动作的终点
    pub prev_end: u64,
    /// 后一动作的起点
    pub next_start: u64,
    /// 传递状态 — 加法统计
    pub carried: u64,
}

impl InheritanceLink {
    pub fn new(prev_end: u64, next_start: u64) -> Self {
        Self { prev_end, next_start, carried: prev_end.saturating_add(next_start) }
    }

    /// 状态传递 — 终点值叠加到下一轮
    pub fn carry(&self, new_value: u64) -> u64 {
        self.carried.saturating_add(new_value)
    }
}

/// 继承链管理器 — 维护动作之间的连贯性
pub struct InheritanceChain {
    links: Vec<InheritanceLink>,
    /// 当前累积状态
    accumulated: u64,
}

impl InheritanceChain {
    pub fn new() -> Self {
        Self { links: Vec::new(), accumulated: 0 }
    }

    /// 添加链接 — 终点接起点
    pub fn link(&mut self, prev_end: u64, next_start: u64) {
        let link = InheritanceLink::new(prev_end, next_start);
        self.accumulated = self.accumulated.saturating_add(link.carried);
        self.links.push(link);
    }

    /// 获取当前累积
    pub fn accumulated(&self) -> u64 {
        self.accumulated
    }

    /// 链长
    pub fn len(&self) -> usize {
        self.links.len()
    }

    /// 从某一起点开始新链
    pub fn restart_from(&mut self, start: u64) {
        self.accumulated = start;
        self.links.clear();
    }
}

// ══════════════════════════════════════════════════════════════
// 时钟触发引擎 — 整合冰箱/意志/继承
// ══════════════════════════════════════════════════════════════

pub struct ClockTriggerEngine {
    /// 冰箱容器集合 — 触发器ID → 冰箱
    fridges: HashMap<u64, FridgeContainer>,
    /// 意志引擎
    will_engine: WillEngine,
    /// 继承链
    inheritance: InheritanceChain,
    /// 当前tick
    current_tick: u64,
    /// 总触发次数统计
    total_fires: u64,
}

impl ClockTriggerEngine {
    pub fn new() -> Self {
        Self {
            fridges: HashMap::new(),
            will_engine: WillEngine::new(),
            inheritance: InheritanceChain::new(),
            current_tick: 0,
            total_fires: 0,
        }
    }

    /// 放置冰箱 — 注册一个触发容器
    pub fn place_fridge(&mut self, id: u64, trigger: TriggerCondition, payload: Code64) {
        let fridge = FridgeContainer::new(id, trigger, payload);
        self.fridges.insert(id, fridge);
    }

    /// 前进一步 — tick+1, 检查所有触发器
    pub fn advance(&mut self) -> Vec<(u64, u64)> {
        self.current_tick += 1;
        self.will_engine.drive_all();

        let mut fired = Vec::new();
        let tick = self.current_tick;

        for (id, fridge) in self.fridges.iter_mut() {
            if let Some(result) = fridge.execute(tick) {
                fired.push((*id, result));
                self.total_fires += 1;
                // 冰箱遗产进入继承链
                self.inheritance.link(result, tick);
            }
        }
        fired
    }

    /// 获取冰箱
    pub fn get_fridge(&self, id: u64) -> Option<&FridgeContainer> {
        self.fridges.get(&id)
    }

    /// 冷却所有冰箱
    pub fn cool_all(&mut self) {
        for fridge in self.fridges.values_mut() {
            fridge.cool_down();
        }
    }

    /// 当前tick
    pub fn tick(&self) -> u64 {
        self.current_tick
    }

    /// 注册意志
    pub fn add_will(&mut self, id: u64) {
        self.will_engine.add_will(id);
    }

    /// 获取意志
    pub fn get_will(&self, id: u64) -> Option<&Will> {
        self.will_engine.get_will(id)
    }

    /// 总触发次数
    pub fn total_fires(&self) -> u64 {
        self.total_fires
    }

    /// 继承链累积值
    pub fn inherited_accumulated(&self) -> u64 {
        self.inheritance.accumulated()
    }

    /// 就绪的冰箱数量 (未冷却, 未完成的)
    pub fn active_fridge_count(&self) -> usize {
        self.fridges.values().filter(|f| !f.trigger.done).count()
    }
}

// ══════════════════════════════════════════════════════════════
// 测试
// ══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::radix_addressing::Code64;

    fn mk_code(val: u32) -> Code64 {
        Code64::new(1, 1, 1, val)
    }

    #[test]
    fn trigger_once_fires_at_tick() {
        let t = TriggerCondition::once(5);
        assert!(!t.should_fire(3));
        assert!(!t.should_fire(4));
        assert!(t.should_fire(5));
    }

    #[test]
    fn trigger_once_done_after_fire() {
        let mut t = TriggerCondition::once(3);
        assert!(t.should_fire(3));
        t.fire();
        assert!(t.done);
        assert!(!t.should_fire(3)); // 已触发过, 不再触发
    }

    #[test]
    fn trigger_periodic_fires_repeatedly() {
        let t = TriggerCondition::periodic(2, 3);
        // tick 2, 5, 8, 11... 触发
        assert!(t.should_fire(2));
        assert!(!t.should_fire(3));
        assert!(!t.should_fire(4));
        assert!(t.should_fire(5));
        assert!(!t.should_fire(6));
        assert!(t.should_fire(8));
    }

    #[test]
    fn trigger_conditional_fires_at_addr() {
        let mut t = TriggerCondition::conditional(10, 42);
        assert!(!t.should_fire(9));
        assert!(t.should_fire(10));
        t.fire();
        assert!(!t.should_fire(10)); // Once后关闭
    }

    #[test]
    fn fridge_executes_and_produces_legacy() {
        let mut fridge = FridgeContainer::new(
            1,
            TriggerCondition::once(3),
            mk_code(42),
        );
        assert!(fridge.execute(2).is_none());
        let result = fridge.execute(3);
        assert_eq!(result, Some(42));
        // 第二次执行: 遗产累加
        // Once触发后done, 所以第二次execute返回None
        assert!(fridge.execute(3).is_none());
    }

    #[test]
    fn fridge_periodic_accumulates() {
        let mut fridge = FridgeContainer::new(
            1,
            TriggerCondition::periodic(1, 5),
            mk_code(10),
        );
        let r1 = fridge.execute(1);
        assert_eq!(r1, Some(10));
        let r2 = fridge.execute(6);
        assert_eq!(r2, Some(20)); // 10 + 10
        let r3 = fridge.execute(11);
        assert_eq!(r3, Some(30)); // 20 + 10
    }

    #[test]
    fn will_grows_with_continuity() {
        let mut will = Will::new(1);
        assert_eq!(will.intensity, 1);
        will.drive();
        will.drive();
        assert_eq!(will.intensity, 3);
        assert_eq!(will.continuity, 2);
    }

    #[test]
    fn will_endpoint_inheritance() {
        let mut will = Will::new(1);
        will.set_endpoint(42);
        assert_eq!(will.start_point(), 42);
    }

    #[test]
    fn will_engine_total_intensity() {
        let mut engine = WillEngine::new();
        engine.add_will(1);
        engine.add_will(2);
        engine.drive_all();
        // 两个意志各驱动一次: 初始1, drive后变2, 总和4
        assert_eq!(engine.total_intensity(), 4);
    }

    #[test]
    fn inheritance_chain_accumulates() {
        let mut chain = InheritanceChain::new();
        chain.link(10, 20); // carried = 30
        chain.link(30, 50); // carried = 80, total = 30+80 = 110
        assert_eq!(chain.len(), 2);
        assert_eq!(chain.accumulated(), 110);
    }

    #[test]
    fn inheritance_restart_from() {
        let mut chain = InheritanceChain::new();
        chain.link(10, 20);
        chain.restart_from(100);
        assert_eq!(chain.accumulated(), 100);
        assert_eq!(chain.len(), 0);
    }

    #[test]
    fn inheritance_carry() {
        let link = InheritanceLink::new(10, 20);
        assert_eq!(link.carried, 30);
        assert_eq!(link.carry(5), 35); // 30 + 5
    }

    #[test]
    fn clock_engine_advance_fires_fridge() {
        let mut engine = ClockTriggerEngine::new();
        engine.place_fridge(1, TriggerCondition::once(3), mk_code(42));

        let r1 = engine.advance(); // tick 1
        assert!(r1.is_empty());
        let r2 = engine.advance(); // tick 2
        assert!(r2.is_empty());
        let r3 = engine.advance(); // tick 3 → 触发
        assert_eq!(r3.len(), 1);
        assert_eq!(r3[0], (1, 42));
    }

    #[test]
    fn clock_engine_periodic_fridge() {
        let mut engine = ClockTriggerEngine::new();
        engine.place_fridge(1, TriggerCondition::periodic(2, 3), mk_code(10));

        engine.advance(); // tick 1
        let r = engine.advance(); // tick 2 → 触发
        assert_eq!(r.len(), 1);
        engine.advance(); // tick 3
        engine.advance(); // tick 4
        let r2 = engine.advance(); // tick 5 → 触发
        assert_eq!(r2.len(), 1);
        assert_eq!(r2[0].1, 20); // 累积
    }

    #[test]
    fn clock_engine_total_fires() {
        let mut engine = ClockTriggerEngine::new();
        engine.place_fridge(1, TriggerCondition::periodic(1, 1), mk_code(1));
        engine.place_fridge(2, TriggerCondition::periodic(1, 1), mk_code(1));

        for _ in 0..5 {
            engine.advance();
        }
        assert_eq!(engine.total_fires(), 10); // 2 * 5
    }

    #[test]
    fn clock_engine_will_integration() {
        let mut engine = ClockTriggerEngine::new();
        engine.add_will(1);
        engine.advance();
        engine.advance();
        let will = engine.get_will(1).unwrap();
        assert_eq!(will.continuity, 2);
        assert_eq!(will.intensity, 3); // 1 + 2
    }

    #[test]
    fn clock_engine_cool_down() {
        let mut engine = ClockTriggerEngine::new();
        engine.place_fridge(1, TriggerCondition::periodic(1, 1), mk_code(1));
        engine.advance();
        assert!(engine.get_fridge(1).unwrap().cooling);
        engine.cool_all();
        assert!(!engine.get_fridge(1).unwrap().cooling);
    }

    #[test]
    fn clock_engine_active_count() {
        let mut engine = ClockTriggerEngine::new();
        engine.place_fridge(1, TriggerCondition::once(1), mk_code(1));
        engine.place_fridge(2, TriggerCondition::periodic(1, 1), mk_code(1));
        assert_eq!(engine.active_fridge_count(), 2);
        engine.advance(); // fridge 1 done, fridge 2 active
        assert_eq!(engine.active_fridge_count(), 1);
    }

    #[test]
    fn clock_engine_inheritance_integration() {
        let mut engine = ClockTriggerEngine::new();
        engine.place_fridge(1, TriggerCondition::periodic(1, 1), mk_code(5));

        engine.advance(); // tick 1: fire=5, inheritance: link(5, 1) = 6
        engine.advance(); // tick 2: fire=10, inheritance: link(10, 2) = 12

        let acc = engine.inherited_accumulated();
        assert!(acc > 0);
    }

    #[test]
    fn trigger_not_fired_before_due() {
        let t = TriggerCondition::once(100);
        assert!(!t.should_fire(0));
        assert!(!t.should_fire(99));
    }

    #[test]
    fn chain_trigger_creates_order() {
        let mut engine = ClockTriggerEngine::new();
        engine.place_fridge(1, TriggerCondition::once(1), mk_code(1));
        engine.place_fridge(2, TriggerCondition::once(2), mk_code(1));

        let r1 = engine.advance();
        assert_eq!(r1.len(), 1); // fridge 1
        let r2 = engine.advance();
        assert_eq!(r2.len(), 1); // fridge 2
    }
}
