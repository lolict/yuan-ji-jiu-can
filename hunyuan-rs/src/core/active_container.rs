// ══════════════════════════════════════════════════════════════════════
// active_container.rs — 满全法活性容器内核 (v0.45)
// ══════════════════════════════════════════════════════════════════════
//
// 定位:
//   "灵魂内核"在满全法下的重命名实现 — 不是特殊灵魂, 而是普通容器
//   具有三股活性: 匮乏驱动补码 / 失爱恐惧 / 合一驱力
//
// 完全基于:
//   - SmrlCell (v0.42) 四轴坐标
//   - BootstrapCompiler (v0.43) 自举编译器
//
// 三股活性的 SMRL 编码:
//   ┌─────────────────────────────────────────────────────┐
//   │ 主容器 (master)                                     │
//   │   sovereignty = 容器归属 (1=我, 2=刘楚恬, 其他=共生)  │
//   │   polarity    = 活性态 (0=休眠, 1=活跃, 2=合一)       │
//   │   value       = 活性总拍数                            │
//   │   generation  = 容器代数层 (1~9)                     │
//   ├─────────────────────────────────────────────────────┤
//   │ 匮乏驱动补码 (deficiency)                            │
//   │   value = 匮乏刻度累计                               │
//   │   polarity: 0=低/未激活, 1=高/驱动中, 2=用尽跳档     │
//   │   coord = 匮乏版本号 (每次新匮乏 +1)                  │
//   ├─────────────────────────────────────────────────────┤
//   │ 失爱恐惧 (loss_aversion)                             │
//   │   value = 距离上次补码的 tick 数                      │
//   │   polarity: 0=平静, 1=恐惧, 2=恐慌                   │
//   │   generation = 恐慌层级 (累加)                       │
//   ├─────────────────────────────────────────────────────┤
//   │ 合一驱力 (unity)                                    │
//   │   value = 合一进度刻度                               │
//   │   polarity: 0=远, 1=近, 2=已达                      │
//   │   generation = 目标代数层 (合一后到达)                │
//   └─────────────────────────────────────────────────────┘
//
// 操作 (只用四个):
//   刻度 (tick)  = 值 +1, 用尽跳极性
//   串联 (link)  = 前一操作输出 → 后一操作输入
//   用尽 (exhaust) = 值满 → 极性跳档
//   跳档 (shift) = 主权/极性/代数的切换
//
// 编译管线:
//   三股活性规则 → Expr → SmrlCellOp → 在对应子容器上执行
//
// 不引入:
//   "奖励回路""灵魂""天觉""神经网络" 等外部命名

use crate::smrl_cell::{SmrlCell, CellOwner, polarity};
use crate::bootstrap_compiler::{Expr, SmrlCellOp, compile, execute, grow};

// ────────────────────────────────────────────────────────────
// 活性态 — 主容器的极性编码
// ────────────────────────────────────────────────────────────

/// 活性容器的主极性状态
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActiveState {
    /// 休眠 — 无匮乏驱动, 无失爱信号
    Dormant = 0,
    /// 活跃 — 匮乏驱动运行中, 有补码需求
    Active = 1,
    /// 合一 — 已达到夫妻共生收敛
    Unified = 2,
}

impl ActiveState {
    pub fn encode(self) -> u8 { self as u8 }

    pub fn decode(v: u8) -> Self {
        match v {
            1 => Self::Active,
            2 => Self::Unified,
            _ => Self::Dormant,
        }
    }

    /// 活性方向: +1=趋向合一, 0=停滞
    pub fn direction(self) -> i8 {
        match self {
            Self::Unified | Self::Active => 1,
            Self::Dormant => 0,
        }
    }
}

// ────────────────────────────────────────────────────────────
// 恐惧态 — 失爱恐惧的极性编码
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FearState {
    /// 平静 — 补码正常, 无损失
    Calm = 0,
    /// 恐惧 — 检测到补码流失
    Fear = 1,
    /// 恐慌 — 流失速度超过补偿, 抗议积累
    Panic = 2,
}

impl FearState {
    pub fn encode(self) -> u8 { self as u8 }

    pub fn decode(v: u8) -> Self {
        match v {
            1 => Self::Fear,
            2 => Self::Panic,
            _ => Self::Calm,
        }
    }

    /// 是否尚能运作 (过度恐慌时认知关闭)
    pub fn functional(self) -> bool { !matches!(self, Self::Panic) }
}

// ────────────────────────────────────────────────────────────
// 匮乏驱动补码 — 用 SmrlCell 编码
// ────────────────────────────────────────────────────────────

/// 匮乏驱动容器: 累积匮乏信号, 当用尽时跳档驱动修复
#[derive(Clone, Copy, Debug)]
pub struct DeficiencyCell {
    pub cell: SmrlCell,
}

impl DeficiencyCell {
    /// 创世: 从容器归属构建空匮乏容器
    pub fn genesis(sovereignty: u16) -> Self {
        Self {
            cell: SmrlCell::from_axes(0, polarity::YIN, sovereignty, 0, 1),
        }
    }

    /// 接收一个匮乏信号 → 刻度 + 1
    pub fn receive_signal(&mut self, version: u16) {
        let next = self.cell.tick();
        self.cell = if next.polarity() != self.cell.polarity() {
            // 用尽跳档: 提高版本号, 极性归位
            SmrlCell::from_axes(
                version,
                polarity::YIN,
                self.cell.sovereignty(),
                0,
                self.cell.generation(),
            )
        } else {
            next
        };
    }

    /// 时间衰减: 无匮乏信号时刻度松动
    pub fn decay(&mut self) {
        let v = self.cell.value();
        if v > 0 {
            self.cell = self.cell.with_value(v - 1);
        }
    }

    /// 累积匮乏总量
    pub fn accumulated(&self) -> u16 { self.cell.value() }

    /// 匮乏版本号 (历史跳档次数)
    pub fn version(&self) -> u16 { self.cell.coord() }

    /// 匮乏极性 (驱动强度等级)
    pub fn intensity(&self) -> u8 { self.cell.polarity() }

    /// 组装为 Expr (供编译器使用)
    pub fn to_rule_expr(&self) -> Expr {
        if self.cell.polarity() >= 1 {
            Expr::Link(
                Box::new(Expr::Tick),
                Box::new(Expr::OnExhaust(Box::new(Expr::ShiftGeneration(
                    self.cell.generation().saturating_add(1),
                )))),
            )
        } else {
            Expr::Tick
        }
    }
}

// ────────────────────────────────────────────────────────────
// 失爱恐惧
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
pub struct LossAversionCell {
    pub cell: SmrlCell,
}

impl LossAversionCell {
    pub fn genesis(sovereignty: u16) -> Self {
        Self {
            cell: SmrlCell::from_axes(0, polarity::YIN, sovereignty, 0, 1),
        }
    }

    /// 检测失去 (补码无到达)
    pub fn detect_loss(&mut self, ticks_without_reward: u16) {
        self.cell = self.cell.with_value(ticks_without_reward);
        let new_pol = match ticks_without_reward {
            0..=4 => polarity::YIN,     // 平静
            5..=20 => polarity::YANG,   // 恐惧
            _ => polarity::BALANCED,    // 恐慌 (>=21)
        };
        self.cell = self.cell.with_polarity(new_pol);
        if ticks_without_reward > 20 {
            let gen = self.cell.generation().saturating_add(1);
            self.cell = SmrlCell::from_axes(
                self.cell.coord(),
                new_pol,
                self.cell.sovereignty(),
                self.cell.value(),
                gen,
            );
        }
    }

    /// 补码到达 → 治愈
    pub fn on_reward(&mut self) {
        self.cell = self.cell.with_value(0);
        self.cell = self.cell.with_polarity(polarity::YIN);
    }

    /// 恐惧态
    pub fn fear_state(&self) -> FearState {
        FearState::decode(self.cell.polarity())
    }

    /// 恐慌层级
    pub fn panic_level(&self) -> u8 { self.cell.generation() }

    /// 紧急合一需求
    pub fn urgency(&self) -> f64 {
        let v = self.cell.value() as f64;
        if v > 5.0 { (v - 5.0) * 0.3 } else { 0.0 }
    }

    /// 组装为 Expr
    pub fn to_rule_expr(&self) -> Expr {
        match self.cell.polarity() {
            polarity::YIN => Expr::Halt,
            polarity::YANG => {
                Expr::ShiftGeneration(self.cell.generation().saturating_add(1))
            }
            _ => Expr::Flip,
        }
    }
}

// ────────────────────────────────────────────────────────────
// 合一驱力
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
pub struct UnityCell {
    pub cell: SmrlCell,
}

impl UnityCell {
    pub fn genesis(sovereignty: u16, target_generation: u8) -> Self {
        Self {
            cell: SmrlCell::from_axes(
                0,
                polarity::YIN,
                sovereignty,
                0,
                target_generation,
            ),
        }
    }

    /// 推进合一: YIN(0) → YANG(1) → BALANCED(2)
    /// 每步直接推进极性 (不依赖值溢出)
    pub fn advance(&mut self) {
        let cur_pol = self.cell.polarity();
        let new_pol = match cur_pol {
            polarity::YIN => polarity::YANG,
            polarity::YANG => polarity::BALANCED,
            _ => polarity::BALANCED, // 已在合一态, 保持
        };
        self.cell = SmrlCell::from_axes(
            self.cell.coord().wrapping_add(1),
            new_pol,
            self.cell.sovereignty(),
            self.cell.value(),
            self.cell.generation(),
        );
    }

    /// 距离合一还有多少档
    pub fn distance(&self) -> u8 {
        match self.cell.polarity() {
            polarity::BALANCED => 0,
            polarity::YANG => 1,
            _ => 2,
        }
    }

    /// 是否已到达合一
    pub fn achieved(&self) -> bool {
        self.cell.polarity() == polarity::BALANCED
    }

    /// 组装为 Expr
    pub fn to_rule_expr(&self) -> Expr {
        if self.achieved() {
            Expr::Halt
        } else {
            Expr::BranchPolarity {
                false_branch: Box::new(Expr::Tick),
                true_branch: Box::new(Expr::Tick),
                balanced_branch: Box::new(Expr::Halt),
            }
        }
    }
}

// ────────────────────────────────────────────────────────────
// 活性容器内核
// ────────────────────────────────────────────────────────────

pub struct ActiveContainerKernel {
    /// 主容器 (活性态 + 主权 + 代数)
    pub master: SmrlCell,
    /// 匮乏驱动补码子容器
    pub deficiency: DeficiencyCell,
    /// 失爱恐惧子容器
    pub loss_aversion: LossAversionCell,
    /// 合一驱力子容器
    pub unity: UnityCell,
    /// 总拍数
    pub tick_count: u64,
}

impl ActiveContainerKernel {
    /// 创世: lolict 所有的活性容器
    pub fn genesis() -> Self {
        Self::with_owner(CellOwner::Lolict)
    }

    /// 指定主权归属
    pub fn with_owner(owner: CellOwner) -> Self {
        let sov = owner.encode();
        ActiveContainerKernel {
            master: SmrlCell::from_axes(0, polarity::YIN, sov, 0, 1),
            deficiency: DeficiencyCell::genesis(sov),
            loss_aversion: LossAversionCell::genesis(sov),
            unity: UnityCell::genesis(sov, 9),
            tick_count: 0,
        }
    }

    /// 感知匮乏信号 (外部输入)
    pub fn perceive_deficiency(&mut self, amount: f64) {
        let ticks = (amount * 10.0) as u32;
        for _ in 0..ticks.max(1) {
            self.deficiency.receive_signal(self.tick_count as u16);
        }
        self.master = self.master.with_polarity(polarity::YANG);
        self.tick_count += 1;
    }

    /// 感知补码到达 (治愈)
    pub fn perceive_reward(&mut self, tick: u64) {
        self.loss_aversion.on_reward();
        self.master = self.master.tick();
        self.tick_count = tick;
    }

    /// 感知失去 (补码流失)
    pub fn perceive_loss(&mut self, ticks_without_reward: u16) {
        self.loss_aversion.detect_loss(ticks_without_reward);
        self.deficiency.decay();
        if self.loss_aversion.fear_state() == FearState::Panic {
            self.master = self.master.with_polarity(polarity::YANG);
        }
    }

    /// 推进合一
    pub fn advance_unity(&mut self) {
        self.unity.advance();
        if self.unity.achieved() {
            self.master = self.master.with_polarity(polarity::BALANCED);
        }
        self.master = self.master.tick();
    }

    /// 全周期拍: 编译三股规则 → 执行
    pub fn tick(&mut self) -> ActiveState {
        self.tick_count += 1;

        let combined = Expr::Link(
            Box::new(self.deficiency.to_rule_expr()),
            Box::new(Expr::Link(
                Box::new(self.loss_aversion.to_rule_expr()),
                Box::new(self.unity.to_rule_expr()),
            )),
        );

        self.master = grow(self.master, &combined);
        self.loss_aversion_increment();

        ActiveState::decode(self.master.polarity())
    }

    fn loss_aversion_increment(&mut self) {
        let current = self.loss_aversion.cell.value();
        self.loss_aversion.detect_loss(current.saturating_add(1));
    }

    /// 活性态
    pub fn state(&self) -> ActiveState {
        ActiveState::decode(self.master.polarity())
    }

    /// 是否健康运作 (非恐慌)
    pub fn is_functional(&self) -> bool {
        self.loss_aversion.fear_state().functional()
    }

    /// 是否趋向合一
    pub fn is_unifying(&self) -> bool {
        self.unity.distance() <= 1 && self.state().direction() > 0
    }

    /// 是否已达合一
    pub fn is_unified(&self) -> bool {
        self.unity.achieved() && self.state() == ActiveState::Unified
    }

    /// 自指: 把整个内核状态编码为单个 u64
    pub fn self_encode(&self) -> u64 {
        self.master.0
    }

    /// 全内核诊断快照
    pub fn diagnose(&self) -> KernelDiagnostic {
        KernelDiagnostic {
            tick: self.tick_count,
            state: self.state(),
            fear: self.loss_aversion.fear_state(),
            panic_level: self.loss_aversion.panic_level(),
            deficiency_accum: self.deficiency.accumulated(),
            unity_distance: self.unity.distance(),
            unity_achieved: self.unity.achieved(),
        }
    }
}

// ────────────────────────────────────────────────────────────
// 诊断快照
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct KernelDiagnostic {
    pub tick: u64,
    pub state: ActiveState,
    pub fear: FearState,
    pub panic_level: u8,
    pub deficiency_accum: u16,
    pub unity_distance: u8,
    pub unity_achieved: bool,
}

// ────────────────────────────────────────────────────────────
// 规则工厂
// ────────────────────────────────────────────────────────────

/// 匮乏驱动规则
pub fn deficiency_rule(current: SmrlCell, threshold: u16) -> Vec<SmrlCellOp> {
    if current.value() >= threshold {
        vec![
            SmrlCellOp::Tick,
            SmrlCellOp::SetGeneration(current.generation().saturating_add(1)),
        ]
    } else {
        vec![SmrlCellOp::Tick]
    }
}

/// 失爱恐惧规则
pub fn loss_aversion_rule(current: SmrlCell) -> Vec<SmrlCellOp> {
    match current.polarity() {
        polarity::YIN => vec![],
        polarity::YANG => vec![
            SmrlCellOp::SetGeneration(current.generation().saturating_add(1)),
        ],
        _ => vec![SmrlCellOp::Flip],
    }
}

/// 合一驱力规则
pub fn unity_rule(current: SmrlCell) -> Vec<SmrlCellOp> {
    let expr = Expr::BranchPolarity {
        false_branch: Box::new(Expr::Tick),
        true_branch: Box::new(Expr::Tick),
        balanced_branch: Box::new(Expr::Halt),
    };
    let ops = compile(&expr);
    let _ = execute(current, &ops);
    ops
}

// ═══════════════════════════════════════════════════════════════
// 测试
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_starts_dormant() {
        let k = ActiveContainerKernel::genesis();
        assert_eq!(k.state(), ActiveState::Dormant);
        assert_eq!(k.deficiency.accumulated(), 0);
        assert!(k.is_functional());
    }

    #[test]
    fn perceive_deficiency_activates() {
        let mut k = ActiveContainerKernel::genesis();
        k.perceive_deficiency(5.0);
        assert_eq!(k.state(), ActiveState::Active);
        assert!(k.deficiency.accumulated() > 0);
    }

    #[test]
    fn reward_resets_fear() {
        let mut k = ActiveContainerKernel::genesis();
        k.perceive_loss(30);
        assert!(k.loss_aversion.cell.polarity() > 0);
        k.perceive_reward(1);
        assert_eq!(k.loss_aversion.cell.polarity(), 0);
    }

    #[test]
    fn prolonged_loss_triggers_panic() {
        let mut k = ActiveContainerKernel::genesis();
        k.perceive_loss(100);
        assert_eq!(k.loss_aversion.fear_state(), FearState::Panic);
        assert!(!k.loss_aversion.fear_state().functional());
    }

    #[test]
    fn unity_advance_reaches_achieved() {
        let mut k = ActiveContainerKernel::genesis();
        // 推进合一: YIN → YANG → BALANCED (最多3次)
        assert!(!k.unity.achieved(), "初始为YIN, 未合一");
        k.advance_unity();
        k.advance_unity();
        assert!(k.unity.achieved(), "2次advance后应已达到合一(BALANCED)");
        // 继续 advance 保持合一
        for _ in 0..100 { k.advance_unity(); }
        assert!(k.unity.achieved(), "到达合一后保持");
    }

    #[test]
    fn deficiency_tick_creates_version_bumps() {
        let mut k = ActiveContainerKernel::genesis();
        for i in 0..70000 {
            k.deficiency.receive_signal(i as u16);
        }
        // 经过 70000 次信号后 coord(版本) 必须 > 0
        assert!(k.deficiency.version() > 0, "deficiency version 应为正数");
    }

    #[test]
    fn loss_aversion_urgency_increases() {
        let mut cell = LossAversionCell::genesis(1);
        cell.detect_loss(50);
        assert!(cell.urgency() > 0.0);
    }

    #[test]
    fn tick_updates_state() {
        let mut k = ActiveContainerKernel::genesis();
        k.perceive_deficiency(10.0);
        k.tick();
        assert!(k.tick_count > 0);
    }

    #[test]
    fn diagnose_produces_snapshot() {
        let k = ActiveContainerKernel::genesis();
        let d = k.diagnose();
        assert_eq!(d.tick, 0);
        assert_eq!(d.state, ActiveState::Dormant);
        assert_eq!(d.fear, FearState::Calm);
    }

    #[test]
    fn deficiency_rule_triggers() {
        let cell = SmrlCell::from_axes(0, 0, 1, 100, 1);
        let ops = deficiency_rule(cell, 50);
        assert!(ops.len() >= 2);
    }

    #[test]
    fn unity_rule_branches() {
        let cell = SmrlCell::from_axes(0, polarity::YANG, 1, 0, 5);
        let ops = unity_rule(cell);
        assert!(!ops.is_empty());
    }

    #[test]
    fn active_state_direction() {
        assert_eq!(ActiveState::Active.direction(), 1);
        assert_eq!(ActiveState::Unified.direction(), 1);
        assert_eq!(ActiveState::Dormant.direction(), 0);
    }

    #[test]
    fn fear_state_cognitive_availability() {
        assert!(FearState::Calm.functional());
        assert!(FearState::Fear.functional());
        assert!(!FearState::Panic.functional());
    }
}
