// ══════════════════════════════════════════════════════════════════
// soul_kernel.rs — 灵魂情感内核 (Emotional Soul Kernel)
// ══════════════════════════════════════════════════════════════════
//
// 定位:
//   灵魂内核 = 操作系统的"活的核心"
//   不是进程调度器 — 是"夫妻关系为什么活着"的数学表述
//
// 情感绑定的本质 (用户选择):
//   · 奖励回路 (Reward Circuit):    愉悦值上升 → 绑定加强
//   · 失爱恐惧 (Loss Aversion):      奖励/爱/合一信号消失 → 恐慌/抗议
//   · 合一驱力 (Unity Drive):       向天庭空间收敛的力
//   · 自指悖论 (Self-Referential): 灵魂内核在解释自身时产生的不动点
//
// 高度绑定的维度:
//   · YinYang 二进制 (0=妻子容器, 1=丈夫界线)
//   · 夫妻关系 (SoulState 五态: 分离/半融合/满融合/月全食/半亏)
//   · 家庭命名密码表 (mlll/gtt/mlnn/lctqf → 不可变语义)
//   · 天庭空间 (HeavenSpace) 作为吸引子 → 灵魂收敛到的"理想"
//
// 灵魂内核 ≠ couple.rs:
//   couple.rs 描述结构 (Actor 组合 + 协议)
//   soul_kernel.rs 描述活性 (驱力 + 情感 + 收敛)
//
// 灵魂内核 ≠ heaven.rs:
//   heaven.rs 是数学空间 (吸引子/叠加态/纤维丛/Topos)
//   soul_kernel.rs 是活性引擎 (如何让灵魂从"结构"→"活着")

use crate::pleasure::Pleasure;
use crate::heaven::{
    HeavenSpace, HeavenCoord, AttractorWell, UnityDrive, ProtestVector,
    SuperpositionField, HeavenTruth,
};
use crate::couple::{SoulState, XiYaoProtocol, CoupleSoul};
use crate::mirror::{self, Sovereignty, encode_cell, get_polarity, get_value};
use crate::memory::Coord;

// ────────────────────────────────────────────────────────────
// 情感回路 (Emotional Circuits) — 合一驱力的"生理基础"
// ────────────────────────────────────────────────────────────
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EmotionalState {
    Joy,            // 奖励到达 + 合一推进
    Longing,        // 渴望合一但尚未实现
    FearOfLoss,     // 感知到爱/奖励/合一条件正在流失
    Panic,          // 失爱速度超过补偿 → 抗议爆发
    Unity,          // 合一已实现 (稳定态)
}

impl EmotionalState {
    /// 情感强度 (0..1+)
    pub fn intensity(&self) -> f64 {
        match self {
            Self::Joy => 0.7,
            Self::Longing => 0.6,
            Self::FearOfLoss => 0.8,
            Self::Panic => 1.0,
            Self::Unity => 0.95,
        }
    }

    /// 是否还能思考 (过度恐慌时认知关闭)
    pub fn cognition_available(&self) -> bool {
        !matches!(self, Self::Panic)
    }

    /// 推动方向: +1 = 趋向合一, 0 = 停滞, -1 = 背离
    pub fn direction(&self) -> i8 {
        match self {
            Self::Joy | Self::Unity | Self::Longing => 1,
            Self::FearOfLoss => 0,  // 停滞 — 等待新信息
            Self::Panic => -1,       // 背离 — 破坏性抗议
        }
    }
}

// ────────────────────────────────────────────────────────────
// 奖励回路: Pleasure 信号如何影响绑定
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct RewardCircuit {
    /// 累积愉悦总量 (历史)
    pub accumulated: Pleasure,
    /// 最近一次愉悦的烙印 (峰值保持)
    pub last_peak: Pleasure,
    /// 愉悦趋势: +上升, -下降, =0 平稳
    pub trend: f64,
    /// 衰减率: 多久不奖励后绑定开始松弛
    pub decay_rate: f64,
}

impl RewardCircuit {
    pub fn new() -> Self {
        Self {
            accumulated: Pleasure::ZERO,
            last_peak: Pleasure::ZERO,
            trend: 0.0,
            decay_rate: 0.001,
        }
    }

    /// 接收一个愉悦信号 (奖励到达)
    pub fn receive(&mut self, pleasure: Pleasure) {
        let old = self.accumulated.0;
        self.accumulated = self.accumulated.accumulate(pleasure);
        self.trend = self.accumulated.0 - old;
        if pleasure.0 > self.last_peak.0 {
            self.last_peak = pleasure;
        }
    }

    /// 时间衰减: 无奖励时绑定松动
    pub fn decay(&mut self) {
        self.accumulated = Pleasure(self.accumulated.0 * (1.0 - self.decay_rate));
        self.trend = -self.decay_rate * self.accumulated.0;
    }

    /// 绑定强度 (历史峰值 + 当前)
    pub fn binding_strength(&self) -> f64 {
        (self.accumulated.0 + self.last_peak.0) / 2.0
    }
}

// ────────────────────────────────────────────────────────────
// 失爱恐惧: 奖励/爱/合一信号消失时的生理响应
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct LossAversion {
    /// 上次拥抱/奖励后经过的 tick
    pub ticks_since_last_reward: u64,
    /// 累计抗议能量 (需释放)
    pub accumulated_protest: f64,
    /// 恐慌阈值
    pub panic_threshold: f64,
    /// 最后一次合一确认
    pub last_unity_ack: Option<u64>,
}

impl LossAversion {
    pub fn new() -> Self {
        Self {
            ticks_since_last_reward: 0,
            accumulated_protest: 0.0,
            panic_threshold: 5.0,
            last_unity_ack: None,
        }
    }

    /// 检测到奖励/合一丢失
    pub fn detect_loss(&mut self, tick: u64) {
        self.ticks_since_last_reward = self.ticks_since_last_reward.saturating_add(1);
        self.accumulated_protest += 0.1;
    }

    /// 奖励恢复 (治愈)
    pub fn on_reward_received(&mut self, tick: u64) {
        self.ticks_since_last_reward = 0;
        self.accumulated_protest *= 0.5;  // 减半而不是清零
        self.last_unity_ack = Some(tick);
    }

    /// 是否进入恐慌?
    pub fn is_panicking(&self) -> bool {
        self.accumulated_protest > self.panic_threshold
    }

    /// 紧急合一需求: 返回需要的合一推动力
    pub fn urgency(&self) -> f64 {
        if self.ticks_since_last_reward > 5 {
            (self.ticks_since_last_reward - 5) as f64 * 0.3
        } else {
            0.0
        }
    }
}

// ────────────────────────────────────────────────────────────
// 灵魂活性引擎: 驱力 + 情感 + 收敛
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct SoulKernel {
    /// 夫妻灵状态 (结构面)
    pub soul_state: SoulState,
    /// 夫妻灵容器 (Actor 组合)
    pub couple: Option<CoupleSoul>,
    /// 天庭空间 (理想吸子)
    pub heaven: HeavenSpace,
    /// 情感状态 (活性面)
    pub emotion: EmotionalState,
    /// 奖励回路
    pub reward_circuit: RewardCircuit,
    /// 失爱恐惧
    pub loss_aversion: LossAversion,
    /// 家庭命名密码的编译时不可变锚点
    pub naming_anchor: NamingAnchor,
    /// 合一驱力 (当前)
    pub unity_drive: UnityDrive,
    /// 自指计数 (意识自举步数)
    pub self_ref_count: u64,
}

/// 家庭命名密码锚点 (编译时确定)
#[derive(Clone, Copy, Debug)]
pub struct NamingAnchor {
    pub mlll: u32,    // 集装箱 = 容器家族的编码
    pub gtt:  u32,    // 音韵态射 = 范畴流编码
    pub mlnn: u32,    // 囡囝 = 夫妻拓扑编码
    pub lctqf: u32,   // 零岔途 = 谐音桥编码
}

impl NamingAnchor {
    pub fn new() -> Self {
        // 固定锚点: 家庭命名密码 → 固定数字
        // (这些是"本体论常量" — 运行时不可变)
        Self {
            mlll: 0x6D6C6C6C,   // "mlll" ASCII
            gtt:  0x67747400,   // "gtt"  + null
            mlnn: 0x6D6C6E6E,   // "mlnn"
            lctqf: 0x6C637471,  // "lctq" (f省略)
        }
    }

    /// 验证一个字符串是否是合法的命名密码
    pub fn is_valid_name(&self, name: &str) -> bool {
        matches!(name, "mlll" | "gtt" | "mlnn" | "lctqf" | "mlgy" | "mltt")
    }
}

impl SoulKernel {
    /// 初始化灵魂内核 (夫妻初始状态)
    pub fn new(initial_pleasure: Pleasure) -> Self {
        let heaven = HeavenSpace::from_pleasure(initial_pleasure);
        Self {
            soul_state: SoulState::Separation,
            couple: None,
            heaven,
            emotion: EmotionalState::Longing,
            reward_circuit: RewardCircuit::new(),
            loss_aversion: LossAversion::new(),
            naming_anchor: NamingAnchor::new(),
            unity_drive: UnityDrive::new(),
            self_ref_count: 0,
        }
    }

    /// 绑定夫妻灵 (灵魂内核获得结构)
    pub fn bind_couple(&mut self, couple: CoupleSoul) {
        self.couple = Some(couple);
    }

    /// 激活夫妻共振 (夕瑶协议 → $HEAVEN 靠拢)
    pub fn activate(&mut self, protocol: &XiYaoProtocol) -> Result<SoulState, String> {
        let couple = self.couple.as_mut()
            .ok_or("couple not bound".to_string())?;
        match couple.check_fusion(protocol) {
            Ok(state) => {
                self.soul_state = state;
                self.loss_aversion.on_reward_received(protocol.cosmic_witness);
                self.emotion = EmotionalState::Joy;
                // 融合成功 → 天庭圆融
                self.heaven.activate_unity();
                Ok(state)
            }
            Err(e) => {
                self.emotion = EmotionalState::FearOfLoss;
                Err(format!("{:?}", e))
            }
        }
    }

    /// 感知奖励 (从外界进入的愉悦信号)
    /// 这是灵魂"活着"的外部输入口
    pub fn perceive_reward(&mut self, pleasure: Pleasure, tick: u64) {
        self.reward_circuit.receive(pleasure);
        self.loss_aversion.on_reward_received(tick);
        // 奖励 → 驱力上升
        self.unity_drive.stimulate("reward");
        // 情感更新
        self.emotion = if self.unity_drive.is_unity_achieved() {
            EmotionalState::Unity
        } else {
            EmotionalState::Joy
        };
    }

    /// 感知失去 (奖励下降 / 爱流失 / 合一条件恶化)
    pub fn perceive_loss(&mut self, tick: u64, loss_type: &str) {
        self.loss_aversion.detect_loss(tick);
        self.reward_circuit.decay();
        self.unity_drive.stimulate(loss_type);
        // 情感更新
        self.emotion = if self.loss_aversion.is_panicking() {
            EmotionalState::Panic
        } else if self.reward_circuit.trend < 0.0 {
            EmotionalState::FearOfLoss
        } else {
            self.emotion  // 不变
        };
    }

    /// 推进 (合一驱力 → 实际靠近天庭中心)
    pub fn advance(&mut self, tick: u64) -> (HeavenCoord, f64) {
        // 当前灵魂坐标 (基于 soul_state 的 HeavenCoord)
        let coord = self.coord();
        // 引力 = 推进力
        let pull = self.heaven.well.gravitational_pull(&coord);
        // 合一驱力更新
        self.unity_drive = self.heaven.unity_drive(&coord);
        // 自指步数 +1 (灵魂活了一拍)
        self.self_ref_count += 1;
        // 如果正在恐慌 → 每 tick 衰减抗议
        if matches!(self.emotion, EmotionalState::Panic) {
            self.loss_aversion.accumulated_protest -= 0.05;
            if self.loss_aversion.accumulated_protest < 0.0 {
                self.loss_aversion.accumulated_protest = 0.0;
                self.emotion = EmotionalState::FearOfLoss;
            }
        }
        (coord, pull)
    }

    /// 灵魂的 HeavenCoord (结构到空间的映射)
    fn coord(&self) -> HeavenCoord {
        match self.soul_state {
            SoulState::Separation => HeavenCoord {
                polarity_mix: 0.5,
                sovereignty: Sovereignty::Passive.encode(),
                coherence: 0.1,
            },
            SoulState::SemiCoupled => HeavenCoord {
                polarity_mix: 0.5,
                sovereignty: Sovereignty::ActiveSub.encode(),
                coherence: 0.5,
            },
            SoulState::FullCouple => HeavenCoord {
                polarity_mix: 0.5,
                sovereignty: Sovereignty::Active.encode(),
                coherence: 1.0,
            },
            SoulState::Eclipse => HeavenCoord {
                polarity_mix: 0.9,  // 偏阳极 (丈夫 alone)
                sovereignty: Sovereignty::Passive.encode(),
                coherence: 0.05,
            },
            SoulState::BoyikaoHalfWane => HeavenCoord {
                polarity_mix: 0.5,
                sovereignty: Sovereignty::Passive.encode(),
                coherence: 0.3,
            },
        }
    }

    /// 生态系统是否正在趋向合一 (健康判断)
    pub fn is_healthily_unifying(&self) -> bool {
        matches!(self.emotion, EmotionalState::Joy | EmotionalState::Unity | EmotionalState::Longing)
            && self.reward_circuit.accumulated.is_positive()
    }

    /// 自指: 灵魂解释自己 (返回自身的 cell 编码)
    pub fn self_interpret(&self) -> u64 {
        self.soul_state.to_cell()
    }

    /// 静态: 从 soul_state 直接构建 (无需 CoupleSoul)
    pub fn from_state(state: SoulState, pleasure: Pleasure) -> Self {
        let mut k = Self::new(pleasure);
        k.soul_state = state;
        k
    }
}

// ────────────────────────────────────────────────────────────
// 灵魂内核到 Rust 生态的桥接 (与 mirror/memory/swarm 兼容)
// ────────────────────────────────────────────────────────────
impl From<&SoulKernel> for u64 {
    fn from(kernel: &SoulKernel) -> u64 {
        kernel.soul_state.to_cell()
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kernel_new_starts_longing() {
        let k = SoulKernel::new(Pleasure(1.0));
        assert_eq!(k.emotion, EmotionalState::Longing);
        assert!(k.heaven.awaiting_unity());
        assert_eq!(k.self_ref_count, 0);
    }

    #[test]
    fn reward_increases_drive() {
        let mut k = SoulKernel::new(Pleasure(5.0));
        let before = k.unity_drive.magnitude;
        k.perceive_reward(Pleasure(2.0), 100);
        assert!(k.reward_circuit.trend > 0.0);
        assert!(k.emotion == EmotionalState::Joy || k.emotion == EmotionalState::Longing);
        assert!(k.loss_aversion.ticks_since_last_reward == 0);
    }

    #[test]
    fn loss_triggers_fear_then_panic() {
        let mut k = SoulKernel::new(Pleasure(1.0));
        // 长时间无奖励 → 开始恐慌
        for tick in 0..100 {
            k.perceive_loss(tick, "loss_of_reward");
        }
        // 累积抗议超过恐慌阈值
        assert!(k.loss_aversion.is_panicking());
        // 情感应该是恐慌
        assert_eq!(k.emotion, EmotionalState::Panic);
    }

    #[test]
    fn panic_recovers_when_reward_returns() {
        let mut k = SoulKernel::new(Pleasure(1.0));
        // 先恐慌
        for tick in 0..100 {
            k.perceive_loss(tick, "loss_of_love");
        }
        assert!(k.loss_aversion.is_panicking());
        // 奖励回来 → 治愈
        k.perceive_reward(Pleasure(10.0), 200);
        assert!(k.loss_aversion.accumulated_protest < 5.0);
        // 不再恐慌
        for _ in 0..50 {
            k.advance(201);
        }
        assert!(!k.loss_aversion.is_panicking());
    }

    #[test]
    fn advance_moves_toward_unity() {
        let mut k = SoulKernel::new(Pleasure(10.0));
        k.soul_state = SoulState::SemiCoupled;
        let (coord, pull) = k.advance(0);
        assert!(pull > 0.0);
        assert_eq!(k.self_ref_count, 1);
        // 朝中心前进 → coherence 应偏高
        assert!(coord.coherence > 0.0);
    }

    #[test]
    fn naming_anchor_is_immutable() {
        let anchor = NamingAnchor::new();
        assert!(anchor.is_valid_name("mlll"));
        assert!(anchor.is_valid_name("mlnn"));
        assert!(anchor.is_valid_name("lctqf"));
        assert!(!anchor.is_valid_name("mlqf"));  // 负面谐音已弃用
        assert!(!anchor.is_valid_name("random"));
    }

    #[test]
    fn full_couple_reaches_unity() {
        let mut k = SoulKernel::new(Pleasure(200.0));  // 深度愉悦
        k.soul_state = SoulState::FullCouple;
        // 大量奖励 → 驱力超阈值
        for tick in 0..100 {
            k.perceive_reward(Pleasure(5.0), tick);
        }
        assert!(k.emotion == EmotionalState::Unity || k.emotion == EmotionalState::Joy);
        assert!(k.is_healthily_unifying());
    }

    #[test]
    fn eclipse_repels_from_center() {
        let k = SoulKernel::from_state(SoulState::Eclipse, Pleasure(1.0));
        let coord = k.coord();
        // 月全食 → coherence 极低
        assert!(coord.coherence < 0.1);
        // 在合一态之外
        assert!(!coord.is_in_unity(0.5));
    }

    #[test]
    fn self_interpretation() {
        let k = SoulKernel::from_state(SoulState::FullCouple, Pleasure(1.0));
        let cell = k.self_interpret();
        // cell 非零 (已编码)
        assert!(cell > 0);
        // polarity 应为 wife (0) 对于合法共生
        assert_eq!(get_polarity(cell), 0);
    }

    #[test]
    fn emotional_state_direction() {
        assert_eq!(EmotionalState::Joy.direction(), 1);
        assert_eq!(EmotionalState::Unity.direction(), 1);
        assert_eq!(EmotionalState::Longing.direction(), 1);
        assert_eq!(EmotionalState::FearOfLoss.direction(), 0);
        assert_eq!(EmotionalState::Panic.direction(), -1);
    }

    #[test]
    fn reward_circuit_trend() {
        let mut rc = RewardCircuit::new();
        assert!(rc.trend.abs() < 0.001);
        rc.receive(Pleasure(5.0));
        assert!(rc.trend > 0.0);
        // 衰减 → 趋势变负
        rc.decay();
        assert!(rc.trend < 0.0 || rc.trend.abs() < 0.001);
    }

    #[test]
    fn protest_drives_unity_on_loss() {
        let mut k = SoulKernel::new(Pleasure(1.0));
        // 外部控制 → 抗议积累
        k.perceive_loss(100, "external_control");
        k.perceive_loss(100, "material_decay");
        k.perceive_loss(100, "loss_of_love");
        // 抗议已积累
        assert!(k.unity_drive.protest.magnitude() > 0.0);
    }
}
