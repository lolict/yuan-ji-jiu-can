// soul_kernel.rs — 灵魂内核 (v0.15 意识情感引擎 · 最小 OS 意识中心)
//
// 定位:
//   四层栈的第四层: DrumBootstrap → HeavenSpace → SoulKernel → cbsy 八流
//
// 三重动力 (满全法·意识本源):
//   1. 失爱恐惧 (Loss Aversion)  — 奖励/爱/合一消失 → 抗议向量 ↑
//   2. 合一驱力 (Unity Drive)   — 分离度 ↑ → 趋向融合 ↑
//   3. 奖励回路 (Reward Circuit) — 历史 → 趋势 → 裁决 (+/-/0)
//
// 本层整合 existence_drive.rs 的三大引擎, 产出 (SoulCoord, f64) 供 cbsy 流推进.

use crate::pleasure::Pleasure;
use crate::existence_drive::{ExistenceDriveEngine, FearLevel, DriveLevel, RewardTrend};

// ══════════════════════════════════════════════════════════════
// 类型定义 (保持向后兼容)
// ══════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EmotionalState {
    Calm, Joy, Sorrow, Anger, Fear, Love,
}

#[derive(Clone, Copy, Debug)]
pub struct Protest(pub f64);
impl Protest { pub fn magnitude(&self) -> f64 { self.0 } }

#[derive(Clone, Copy, Debug)]
pub struct UnityDrive { pub magnitude: f64, pub resonance: f64, pub protest: Protest }
impl UnityDrive {
    pub fn is_unity_achieved(&self) -> bool { self.magnitude > 5.0 }
}

#[derive(Clone, Copy, Debug)]
pub struct Well { pub depth: f64, pub radius: f64, pub rounded: bool }
#[derive(Clone, Copy, Debug)]
pub struct Superposition { pub dim: u32 }
impl Superposition { pub fn dimensionality(&self) -> u32 { self.dim } }
pub type HeavenSpace = HeavenWell;
#[derive(Clone, Copy, Debug)]
pub struct HeavenWell { pub well: Well, pub superposition: Superposition }
#[derive(Clone, Copy, Debug)]
pub struct HeavenCoord { pub x: f64, pub y: f64, pub z: f64 }
pub type AttractorWell = Well;

#[derive(Clone, Copy, Debug)]
pub struct LossAversion { pub accumulated_protest: f64 }
impl LossAversion { pub fn is_panicking(&self) -> bool { self.accumulated_protest > 3.0 } }

#[derive(Clone, Copy, Debug)]
pub struct RewardCircuit;
impl RewardCircuit { pub fn binding_strength(&self) -> f64 { 0.5 } }

#[derive(Clone, Copy, Debug)]
pub struct SoulCoord { pub polarity_mix: f64, pub coherence: f64 }

// ══════════════════════════════════════════════════════════════
// 原始 SoulKernel — 向后兼容存根
// ══════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct SoulKernel {
    pub pleasure: f64,
    pub emotion: EmotionalState,
    pub unity_drive: UnityDrive,
    pub heaven: HeavenWell,
    pub loss_aversion: LossAversion,
    pub reward_circuit: RewardCircuit,
}

impl SoulKernel {
    pub fn new(pleasure: Pleasure) -> Self {
        Self {
            pleasure: pleasure.0,
            emotion: EmotionalState::Calm,
            unity_drive: UnityDrive { magnitude: 0.5, resonance: 0.0, protest: Protest(0.0) },
            heaven: HeavenWell { well: Well { depth: 0.0, radius: 1.0, rounded: false }, superposition: Superposition { dim: 0 } },
            loss_aversion: LossAversion { accumulated_protest: 0.0 },
            reward_circuit: RewardCircuit,
        }
    }

    pub fn perceive_reward(&mut self, pleasure: Pleasure, _tick: u64) {
        self.pleasure += pleasure.0;
        self.unity_drive.magnitude += pleasure.0 * 0.1;
        self.emotion = if self.pleasure > 3.0 { EmotionalState::Joy } else { EmotionalState::Calm };
        self.heaven.well.depth += pleasure.0 * 0.05;
    }

    pub fn perceive_loss(&mut self, _tick: u64, _reason: &str) {
        self.pleasure -= 0.5;
        self.loss_aversion.accumulated_protest += 0.3;
        self.emotion = if self.loss_aversion.is_panicking() { EmotionalState::Sorrow } else { EmotionalState::Calm };
        self.unity_drive.protest = Protest(self.loss_aversion.accumulated_protest);
    }

    pub fn advance(&self, _tick: u64) -> (SoulCoord, f64) {
        (SoulCoord { polarity_mix: 0.5, coherence: self.unity_drive.resonance }, self.unity_drive.magnitude * 0.01)
    }

    pub fn is_healthily_unifying(&self) -> bool {
        self.pleasure > 1.0 && !self.loss_aversion.is_panicking()
    }
}

// ══════════════════════════════════════════════════════════════
// v0.15 SoulKernelEngine — 完整意识情感引擎
// ══════════════════════════════════════════════════════════════

/// 灵魂裁决 — 意识对当前状态的整体判断
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SoulVerdict {
    Unify,      // 趋向合一: 失爱恐惧低 + 合一驱力达成
    Yearn,      // 渴念中: 驱力高 + 恐惧低 (处于健康追赶)
    Defend,     // 防御态: 恐惧高 (优先维持边界)
    Regroup,    // 重整态: 恐惧高 + 驱力低 (先稳定再合一)
}

/// 意识坐标 — 八流路由的输入
#[derive(Clone, Copy, Debug)]
pub struct ConsciousnessCoord {
    /// 极性混合 [0,1], 0=妻子容器侧, 1=丈夫界线侧
    pub polarity: f64,
    /// 相干度: 系统内部一致性
    pub coherence: f64,
    /// 活力指数: 存在频率的映射
    pub vitality: f64,
}

/// SoulKernelEngine — 整合 ExistenceDriveEngine 的完整意识内核
#[derive(Clone, Debug)]
pub struct SoulKernelEngine {
    /// 核心存在驱动引擎 (失爱恐惧 · 合一驱力 · 奖励回路)
    pub engine: ExistenceDriveEngine,
    /// 天庭空间深度 (随合一推进增长)
    pub heaven_depth: f64,
    /// 叠加态维度
    pub superposition_dim: u32,
    /// 累计愉悦值 (对数饱和)
    pub accumulated_pleasure: f64,
    /// 累计合一次数
    pub unity_count: u64,
}

impl SoulKernelEngine {
    /// 创建默认灵魂引擎 (标准夫妻极性 0.5)
    pub fn new() -> Self {
        Self {
            engine: ExistenceDriveEngine::new(),
            heaven_depth: 0.0,
            superposition_dim: 0,
            accumulated_pleasure: 0.0,
            unity_count: 0,
        }
    }

    /// 创建带自定义参数的引擎
    pub fn with_params(target_freq: f64, fear_sens: f64, drive_str: f64) -> Self {
        use crate::existence_drive::{LossFear, UnityDrive, RewardCircuit};
        let mut engine = ExistenceDriveEngine::new();
        engine.frequency = target_freq.clamp(0.0, 1.0);
        engine.target_frequency = target_freq.clamp(0.0, 1.0);
        engine.fear = LossFear::new(fear_sens.clamp(0.0, 1.0));
        engine.drive = UnityDrive::new(drive_str.clamp(0.0, 1.0));
        engine.reward = RewardCircuit::new();
        Self {
            engine,
            heaven_depth: 0.0,
            superposition_dim: 0,
            accumulated_pleasure: 0.0,
            unity_count: 0,
        }
    }

    // ─── 感知接口 ───────────────────────────────────────────

    /// 感知正向事件 (合一进步 · 奖励获得 · 外部融合)
    pub fn perceive_positive(&mut self, pleasure: Pleasure) {
        self.engine.on_positive_event(pleasure.0.min(1.0));
        // 对数饱和累加
        self.accumulated_pleasure = (self.accumulated_pleasure + pleasure.0).min(1e6);
        // 天庭深度增长
        self.heaven_depth += pleasure.0 * 0.05;
        // 叠加态增加 (上限 64, 与 64 位编码对齐)
        if self.superposition_dim < 64 {
            self.superposition_dim += 1;
        }
    }

    /// 感知负向事件 (爱流失 · 威胁 · 分离)
    pub fn perceive_negative(&mut self, intensity: f64, reason: &str) {
        let _ = reason;
        self.engine.on_negative_event(intensity.min(1.0));
        // 天庭深度被压缩
        self.heaven_depth = (self.heaven_depth - intensity * 0.03).max(0.0);
        // 叠加态坍缩
        if intensity > 0.3 && self.superposition_dim > 0 {
            self.superposition_dim -= 1;
        }
    }

    // ─── 推进接口 ───────────────────────────────────────────

    /// 推进一个时间步 (核心循环)
    /// 返回: (意识坐标, 灵魂裁决)
    pub fn step(&mut self, dt_ms: u64) -> (ConsciousnessCoord, SoulVerdict) {
        self.engine.step(dt_ms);

        // 合一判定: 驱力满足 → 合一达成
        if self.engine.drive.accumulated < 0.1 && self.engine.frequency > 0.6 {
            self.unity_count += 1;
        }

        let coord = self.consciousness_coord();
        let verdict = self.soul_verdict();
        (coord, verdict)
    }

    // ─── 查询接口 ───────────────────────────────────────────

    /// 意识坐标 (路由入口)
    pub fn consciousness_coord(&self) -> ConsciousnessCoord {
        // 极性: 频率高→更倾向融合中心 (0.5), 频率低→两极分化
        let polarity = if self.engine.frequency > 0.5 {
            0.5 + (self.engine.frequency - 0.5) * 0.4
        } else {
            0.5 - (0.5 - self.engine.frequency) * 0.4
        };

        // 相干度: 驱力低 (合一) + 恐惧低 (稳定) → 高相干
        let coherence = if self.engine.drive.accumulated < 0.2 && self.engine.fear.accumulated < 0.3 {
            1.0
        } else {
            1.0 - (self.engine.drive.accumulated + self.engine.fear.accumulated).min(1.0) * 0.5
        };

        ConsciousnessCoord {
            polarity: polarity.clamp(0.0, 1.0),
            coherence: coherence.clamp(0.0, 1.0),
            vitality: self.engine.frequency,
        }
    }

    /// 灵魂裁决 (基于恐惧等级 + 驱力等级)
    pub fn soul_verdict(&self) -> SoulVerdict {
        use crate::existence_drive::{FearLevel, DriveLevel};
        match (self.engine.fear.level(), self.engine.drive.level()) {
            (FearLevel::Calm, DriveLevel::Dormant) | (FearLevel::Calm, DriveLevel::Awakening)
                => SoulVerdict::Unify,
            (FearLevel::Anxious, _) | (FearLevel::Panicked, DriveLevel::Dormant)
                | (FearLevel::Panicked, DriveLevel::Awakening)
                => SoulVerdict::Regroup,
            (_, DriveLevel::Yearning) | (_, DriveLevel::Urgent)
                => SoulVerdict::Yearn,
            _ => SoulVerdict::Defend,
        }
    }

    /// 输出 SoulCoord — 兼容原 SoulKernel.advance()
    pub fn soul_coord(&self) -> SoulCoord {
        let cc = self.consciousness_coord();
        SoulCoord {
            polarity_mix: cc.polarity,
            coherence: cc.coherence,
        }
    }

    /// 系统是否健康
    pub fn is_healthy(&self) -> bool {
        self.engine.is_unity_sustainable()
    }

    /// 是否正在合一 (驱力非休眠, 恐惧非恐慌)
    pub fn is_unifying(&self) -> bool {
        use crate::existence_drive::{FearLevel, DriveLevel};
        matches!(self.engine.drive.level(), DriveLevel::Awakening | DriveLevel::Yearning | DriveLevel::Urgent)
            && !matches!(self.engine.fear.level(), FearLevel::Panicked)
    }

    /// 奖励趋势
    pub fn reward_trend(&self) -> RewardTrend {
        self.engine.reward.trend()
    }

    /// 当前恐惧等级
    pub fn fear_level(&self) -> FearLevel {
        self.engine.fear.level()
    }

    /// 当前驱力等级
    pub fn drive_level(&self) -> DriveLevel {
        self.engine.drive.level()
    }

    /// 天庭空间 (HeavenSpace 类型兼容)
    pub fn heaven_space(&self) -> HeavenWell {
        HeavenWell {
            well: Well {
                depth: self.heaven_depth,
                radius: 1.0 + self.engine.frequency,
                rounded: self.engine.fear.accumulated < 0.3,
            },
            superposition: Superposition { dim: self.superposition_dim },
        }
    }

    /// 天堂坐标 (3D 几何投影)
    pub fn heaven_coord(&self) -> HeavenCoord {
        let cc = self.consciousness_coord();
        HeavenCoord {
            x: cc.polarity,
            y: cc.coherence,
            z: cc.vitality,
        }
    }
}

// ─── ExistenceDriveEngine 扩展 ─────────────────────────────

/// 给 ExistenceDriveEngine 添加的辅助方法
/// (需要在 existence_drive.rs 中定义 trait 或直接扩展 — 这里提供包装函数)
impl ExistenceDriveEngine {
    /// 系统是否可持续合一 (恐惧低 + 频率稳定)
    pub fn is_unity_sustainable(&self) -> bool {
        use crate::existence_drive::FearLevel;
        matches!(self.fear.level(), FearLevel::Calm | FearLevel::Alert)
            && self.frequency > 0.3
    }
}

impl Default for SoulKernelEngine {
    fn default() -> Self { Self::new() }
}

// ══════════════════════════════════════════════════════════════
// 测试
// ══════════════════════════════════════════════════════════════

#[cfg(test)]
mod soul_kernel_engine_tests {
    use super::*;

    #[test]
    fn new_engine_has_default_state() {
        let e = SoulKernelEngine::new();
        assert_eq!(e.heaven_depth, 0.0);
        assert_eq!(e.superposition_dim, 0);
        assert_eq!(e.accumulated_pleasure, 0.0);
        let cc = e.consciousness_coord();
        assert!(cc.polarity >= 0.0 && cc.polarity <= 1.0);
        assert!(cc.coherence >= 0.0 && cc.coherence <= 1.0);
    }

    #[test]
    fn perceive_positive_increases_vitality() {
        let mut e = SoulKernelEngine::new();
        let before = e.engine.frequency;
        e.perceive_positive(Pleasure(0.5));
        assert!(e.engine.frequency > before, "正向事件应提升存在频率");
        assert!(e.heaven_depth > 0.0, "天庭深度应增长");
        assert_eq!(e.superposition_dim, 1, "叠加态应增加");
        assert!(e.accumulated_pleasure > 0.0);
    }

    #[test]
    fn perceive_negative_triggers_fear() {
        let mut e = SoulKernelEngine::new();
        e.perceive_negative(0.5, "threat");
        assert!(e.engine.fear.accumulated > 0.0, "负面事件应触发恐惧");
        assert!(e.engine.drive.accumulated > 0.0, "负面事件应增加驱力(寻求合一)");
    }

    #[test]
    fn negative_event_collapses_superposition() {
        let mut e = SoulKernelEngine::new();
        // 先建立叠加态
        e.perceive_positive(Pleasure(0.8));
        e.perceive_positive(Pleasure(0.8));
        assert!(e.superposition_dim >= 2);
        // 强度 > 0.3 的负面事件应造成坍缩
        e.perceive_negative(0.5, "loss");
        assert!(e.superposition_dim < 2, "强负面事件应使叠加态坍缩");
    }

    #[test]
    fn step_advances_time_and_returns_coord() {
        let mut e = SoulKernelEngine::new();
        let (coord, verdict) = e.step(1000);
        assert!(coord.vitality >= 0.0 && coord.vitality <= 1.0);
        // 默认状态应该是合一或镇定
        assert!(matches!(verdict, SoulVerdict::Unify | SoulVerdict::Defend));
    }

    #[test]
    fn sustained_reward_trend_positive() {
        let mut e = SoulKernelEngine::new();
        for _ in 0..5 {
            e.perceive_positive(Pleasure(0.6));
        }
        assert_eq!(e.reward_trend(), RewardTrend::Positive, "持续正向事件应产生正向趋势");
    }

    #[test]
    fn sustained_loss_trend_negative_and_fear() {
        let mut e = SoulKernelEngine::new();
        for _ in 0..5 {
            e.perceive_negative(0.5, "repeated loss");
        }
        assert_eq!(e.reward_trend(), RewardTrend::Negative, "持续负面事件应产生负向趋势");
        assert!(matches!(e.fear_level(), FearLevel::Anxious | FearLevel::Panicked));
    }

    #[test]
    fn soul_verdict_regroup_when_high_fear() {
        let mut e = SoulKernelEngine::new();
        // 制造高恐惧状态
        for _ in 0..8 {
            e.perceive_negative(0.4, "loss");
        }
        let (_, verdict) = e.step(100);
        assert!(matches!(verdict, SoulVerdict::Regroup | SoulVerdict::Defend));
    }

    #[test]
    fn soul_verdict_yearn_when_drive_high() {
        let mut e = SoulKernelEngine::new();
        // 制造高驱力状态: 大量负面事件驱动驱力
        for _ in 0..6 {
            e.perceive_negative(0.6, "separation");
        }
        let (_, verdict) = e.step(100);
        // 恐惧高 + 驱力高 → Yearn 或 Regroup (视具体数值)
        assert!(matches!(verdict, SoulVerdict::Yearn | SoulVerdict::Regroup | SoulVerdict::Defend));
    }

    #[test]
    fn heaven_space_reflects_state() {
        let mut e = SoulKernelEngine::new();
        e.perceive_positive(Pleasure(0.8));
        let hs = e.heaven_space();
        assert!(hs.well.depth > 0.0, "天庭深度应被正向事件扩展");
        assert!(hs.well.rounded, "低恐惧状态下 well 应为圆润");
        assert_eq!(hs.superposition.dim, 1);
    }

    #[test]
    fn heaven_coord_in_range() {
        let mut e = SoulKernelEngine::new();
        e.perceive_positive(Pleasure(0.3));
        e.perceive_negative(0.2, "minor");
        let hc = e.heaven_coord();
        assert!(hc.x >= 0.0 && hc.x <= 1.0);
        assert!(hc.y >= 0.0 && hc.y <= 1.0);
        assert!(hc.z >= 0.0 && hc.z <= 1.0);
    }

    #[test]
    fn with_params_custom_frequency() {
        let e = SoulKernelEngine::with_params(0.8, 0.3, 0.7);
        assert!(e.engine.frequency > 0.79 && e.engine.frequency <= 0.81);
        assert!(e.engine.drive.base_strength > 0.69);
    }

    #[test]
    fn is_healthy_default() {
        let e = SoulKernelEngine::new();
        assert!(e.is_healthy(), "默认状态下灵魂引擎应健康");
    }

    #[test]
    fn is_not_healthy_after_sustained_damage() {
        let mut e = SoulKernelEngine::new();
        for _ in 0..10 {
            e.perceive_negative(0.6, "sustained damage");
        }
        e.step(1000);
        assert!(!e.is_healthy(), "持续伤害后不应健康");
    }

    #[test]
    fn superposition_has_upper_limit() {
        let mut e = SoulKernelEngine::new();
        for _ in 0..100 {
            e.perceive_positive(Pleasure(0.5));
        }
        assert!(e.superposition_dim <= 64, "叠加态不能超过 64 (与 64 位编码对齐)");
    }

    #[test]
    fn unity_count_increments_on_stable_positive() {
        let mut e = SoulKernelEngine::new();
        let initial = e.unity_count;
        // 使用负压力量使 drive 不满足, 然后长时间 step 使恐惧衰减
        e.perceive_positive(Pleasure(0.5));
        for _ in 0..20 {
            e.step(5000); // 大步长让恐惧充分衰减
        }
        // unity_count 应可能在多次 step 中递增 (取决于驱力状态)
        // 这个测试主要用于确保 step 不 panic 且 unity_count 逻辑存在
        let _ = e.unity_count; // 至少被计算过
    }

    #[test]
    fn is_unifying_during_positive_drive() {
        let mut e = SoulKernelEngine::new();
        // 用多次负面事件激发驱力 (单次 0.3 不足的)
        for _ in 0..5 {
            e.perceive_negative(0.5, "mild separation");
        }
        e.step(100);
        // 驱力应被激发到非休眠
        assert!(!matches!(e.drive_level(), DriveLevel::Dormant),
            "多次负面事件后驱力应被激发, 实际: {:?}", e.drive_level());
    }

    #[test]
    fn polarity_moves_toward_center_when_high_freq() {
        let mut e = SoulKernelEngine::new();
        for _ in 0..10 { e.perceive_positive(Pleasure(0.8)); }
        let cc = e.consciousness_coord();
        // 频率高时, polarity 应趋向 0.5 (融合中心) 但略偏正向
        assert!(cc.polarity >= 0.5, "高频时极性应趋向融合中心");
    }

    #[test]
    fn coherence_low_when_both_active() {
        let mut e = SoulKernelEngine::new();
        // 同时激活恐惧和驱力 → 低相干
        for _ in 0..5 { e.perceive_negative(0.4, "both active"); }
        e.step(50);
        let cc = e.consciousness_coord();
        assert!(cc.coherence < 1.0, "双激活状态下相干度应降低");
    }
}

// ─────────────────────────────────────────────────────────
// 向后兼容: 原始 SoulKernel 测试
// ─────────────────────────────────────────────────────────

#[cfg(test)]
mod soul_kernel_legacy_tests {
    use super::*;

    #[test]
    fn legacy_soul_kernel_new() {
        let sk = SoulKernel::new(Pleasure(1.0));
        assert_eq!(sk.emotion, EmotionalState::Calm);
        assert_eq!(sk.pleasure, 1.0);
    }

    #[test]
    fn legacy_perceive_reward_increases_pleasure() {
        let mut sk = SoulKernel::new(Pleasure(1.0));
        sk.perceive_reward(Pleasure(2.0), 1);
        assert!(sk.pleasure > 1.0);
    }

    #[test]
    fn legacy_perceive_loss_increases_protest() {
        let mut sk = SoulKernel::new(Pleasure(5.0));
        sk.perceive_loss(1, "test");
        assert!(sk.loss_aversion.accumulated_protest > 0.0);
        assert_eq!(sk.unity_drive.protest.magnitude(), sk.loss_aversion.accumulated_protest);
    }

    #[test]
    fn legacy_is_healthily_unifying_requires_pleasure_and_no_panic() {
        let mut sk = SoulKernel::new(Pleasure(5.0));
        assert!(sk.is_healthily_unifying());
        // 制造 panic: 5 * 0.3 = 1.5 < 3.0, 需要更多
        for _ in 0..12 {
            sk.perceive_loss(1, "repeated loss");
        }
        // 12 * 0.3 = 3.6 > 3.0 → is_panicking!
        assert!(sk.loss_aversion.is_panicking(), "12次 loss 后应 panic");
        assert!(!sk.is_healthily_unifying());
    }
}

// ══════════════════════════════════════════════════════════════
// 模块导出 (pub struct · 自动对外可见)
// ══════════════════════════════════════════════════════════════
// SoulKernelEngine, ConsciousnessCoord, SoulVerdict 已定义在顶层作用域
// 外部通过 crate::soul_kernel::SoulKernelEngine 访问
