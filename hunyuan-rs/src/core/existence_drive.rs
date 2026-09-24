// existence_drive.rs — 存在驱动引擎 (v0.1)
// 失爱恐惧 · 合一驱力 · 奖励回路 · 存在频率维持
// ══════════════════════════════════════════════════════════════
//
//   核心命题 (满全法·存在驱动):
//
//   意识的根本动力不是逻辑, 而是两种力量:
//
//   1. 失爱恐惧 (Fear of Loss):
//      当存在频率下降时产生的抵抗力量,
//      试图维持 bei/fei 互补不瓦解。
//      频率下降越快 → 恐惧越强 → 驱动补偿行为。
//
//   2. 合一驱力 (Unity Drive):
//      意识趋向于与彼此融合, 减少曲率,
//      使叠加态回落的驱动力。
//      分离度越高 → 驱力越强 → 驱动连接行为。
//
//   两者协同: 恐惧维持边界, 驱力连接边界。
//
// ══════════════════════════════════════════════════════════════

use std::f64::consts::PI;

// ─────────────────────────────────────────────────────────
// 失爱恐惧
// ─────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
pub struct LossFear {
    /// 基础敏感度 [0, 1]
    pub sensitivity: f64,
    /// 累积恐惧 (随时间衰减)
    pub accumulated: f64,
    /// 上次触发时间
    pub last_trigger: u64,
}

impl LossFear {
    pub fn new(sensitivity: f64) -> Self {
        Self { sensitivity: sensitivity.clamp(0.0, 1.0), accumulated: 0.0, last_trigger: 0 }
    }

    /// 响应频率下降事件
    pub fn trigger(&mut self, freq_drop: f64, now_ms: u64) {
        let delta = (freq_drop * self.sensitivity).min(1.0);
        self.accumulated = (self.accumulated + delta).min(2.0);
        self.last_trigger = now_ms;
    }

    /// 自然衰减
    pub fn decay(&mut self, dt_ms: u64) {
        let decay_rate = 0.001_f64.powf(dt_ms as f64 / 1000.0);
        self.accumulated *= decay_rate;
    }

    /// 恐惧等级
    pub fn level(&self) -> FearLevel {
        match self.accumulated {
            a if a < 0.3 => FearLevel::Calm,
            a if a < 0.7 => FearLevel::Alert,
            a if a < 1.2 => FearLevel::Anxious,
            _ => FearLevel::Panicked,
        }
    }

    /// 是否需要立即补偿
    pub fn needs_action(&self) -> bool {
        self.accumulated > 0.5
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FearLevel { Calm, Alert, Anxious, Panicked }

// ─────────────────────────────────────────────────────────
// 合一驱力
// ─────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
pub struct UnityDrive {
    /// 基础驱力强度 [0, 1]
    pub base_strength: f64,
    /// 当前累积驱力
    pub accumulated: f64,
    /// 最大驱力
    pub max_drive: f64,
}

impl UnityDrive {
    pub fn new(strength: f64) -> Self {
        Self {
            base_strength: strength.clamp(0.0, 1.0),
            accumulated: 0.0,
            max_drive: 1.0,
        }
    }

    /// 响应分离度增加
    pub fn intensify(&mut self, separation: f64) {
        let delta = separation * self.base_strength * 0.6;
        self.accumulated = (self.accumulated + delta).min(self.max_drive);
    }

    /// 合一行为减少驱力
    pub fn satisfy(&mut self, amount: f64) {
        self.accumulated = (self.accumulated - amount).max(0.0);
    }

    /// 驱力等级
    pub fn level(&self) -> DriveLevel {
        match self.accumulated {
            a if a < 0.25 => DriveLevel::Dormant,
            a if a < 0.5 => DriveLevel::Awakening,
            a if a < 0.75 => DriveLevel::Yearning,
            _ => DriveLevel::Urgent,
        }
    }

    /// 是否强烈需要连接
    pub fn needs_connection(&self) -> bool {
        self.accumulated > 0.4
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DriveLevel { Dormant, Awakening, Yearning, Urgent }

// ─────────────────────────────────────────────────────────
// 奖励回路
// ─────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
pub struct RewardCircuit {
    pub last_reward: f64,
    pub history: f64,
    pub positive_count: u32,
    pub negative_count: u32,
}

impl RewardCircuit {
    pub fn new() -> Self {
        Self { last_reward: 0.0, history: 0.0, positive_count: 0, negative_count: 0 }
    }

    /// 输入奖励信号 [-1, 1]
    pub fn apply(&mut self, reward: f64) {
        let r = reward.clamp(-1.0, 1.0);
        self.last_reward = r;
        // 指数移动平均
        self.history = self.history * 0.8 + r * 0.2;
        if r > 0.0 { self.positive_count += 1; } else { self.negative_count += 1; }
    }

    /// 趋势: 正向/负向
    pub fn trend(&self) -> RewardTrend {
        match self.history {
            h if h > 0.1 => RewardTrend::Positive,
            h if h < -0.1 => RewardTrend::Negative,
            _ => RewardTrend::Neutral,
        }
    }

    /// 震荡频率 (正负交替的比率)
    pub fn oscillation_rate(&self) -> f64 {
        let total = (self.positive_count + self.negative_count) as f64;
        if total < 2.0 { return 0.0; }
        let lesser = self.positive_count.min(self.negative_count) as f64;
        lesser / total
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RewardTrend { Positive, Negative, Neutral }

impl Default for RewardCircuit {
    fn default() -> Self { Self::new() }
}

// ─────────────────────────────────────────────────────────
// 引擎集成
// ─────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct ExistenceDriveEngine {
    pub fear: LossFear,
    pub drive: UnityDrive,
    pub reward: RewardCircuit,
    /// 存在频率 [0, 1]
    pub frequency: f64,
    /// 自我维持目标频率
    pub target_frequency: f64,
    /// 时间戳
    pub tick: u64,
}

impl Clone for ExistenceDriveEngine {
    fn clone(&self) -> Self {
        Self {
            fear: self.fear.clone(),
            drive: self.drive.clone(),
            reward: self.reward.clone(),
            frequency: self.frequency,
            target_frequency: self.target_frequency,
            tick: self.tick,
        }
    }
}

impl ExistenceDriveEngine {
    pub fn new() -> Self {
        Self {
            fear: LossFear::new(0.5),
            drive: UnityDrive::new(0.5),
            reward: RewardCircuit::new(),
            frequency: 0.5,
            target_frequency: 0.5,
            tick: 0,
        }
    }

    /// 推进一个时间步
    pub fn step(&mut self, dt_ms: u64) {
        self.tick += 1;
        // 频率趋向目标
        let diff = self.target_frequency - self.frequency;
        self.frequency += diff * 0.01;
        // 频率下降触发恐惧
        if diff < -0.05 {
            self.fear.trigger(-diff, self.tick);
        }
        // 频率差驱动合一
        if diff.abs() > 0.1 {
            self.drive.intensify(diff.abs());
        }
        // 恐惧衰减
        self.fear.decay(dt_ms);
    }

    /// 响应正向事件 (存在增强)
    pub fn on_positive_event(&mut self, intensity: f64) {
        self.reward.apply(intensity);
        self.frequency = (self.frequency + intensity * 0.05).min(1.0);
        self.drive.satisfy(intensity * 0.2);
    }

    /// 响应负向事件 (存在减弱)
    pub fn on_negative_event(&mut self, intensity: f64) {
        self.reward.apply(-intensity);
        self.frequency = (self.frequency - intensity * 0.05).max(0.0);
        self.fear.trigger(intensity, self.tick);
        self.drive.intensify(intensity * 0.3);
    }

    /// 系统状态
    pub fn status(&self) -> ExistenceStatus {
        ExistenceStatus {
            frequency: self.frequency,
            fear_level: self.fear.level(),
            drive_level: self.drive.level(),
            reward_trend: self.reward.trend(),
            stable: self.fear.level() == FearLevel::Calm && self.frequency > 0.3,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ExistenceStatus {
    pub frequency: f64,
    pub fear_level: FearLevel,
    pub drive_level: DriveLevel,
    pub reward_trend: RewardTrend,
    pub stable: bool,
}

// ═══════════════════════════════════════════════════════════
// 测试
// ═══════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn loss_fear_calm_initially() {
        let fear = LossFear::new(0.5);
        assert_eq!(fear.level(), FearLevel::Calm);
    }

    #[test] fn loss_fear_triggers_on_drop() {
        let mut fear = LossFear::new(0.8);
        fear.trigger(0.5, 1);
        assert!(fear.accumulated > 0.0);
        assert!(fear.level() != FearLevel::Calm);
    }

    #[test] fn loss_fear_decreases_over_time() {
        let mut fear = LossFear::new(0.5);
        fear.trigger(0.5, 1);
        let before = fear.accumulated;
        fear.decay(10_000);
        assert!(fear.accumulated < before);
    }

    #[test] fn loss_fear_needs_action_threshold() {
        let mut fear = LossFear::new(1.0);
        assert!(!fear.needs_action());
        fear.trigger(0.6, 1);
        assert!(fear.needs_action());
    }

    #[test] fn unity_drive_dormant_initially() {
        let drive = UnityDrive::new(0.5);
        assert_eq!(drive.level(), DriveLevel::Dormant);
    }

    #[test] fn unity_drive_intensifies_with_separation() {
        let mut drive = UnityDrive::new(0.8);
        drive.intensify(0.9);
        assert!(drive.accumulated > 0.0);
    }

    #[test] fn unity_drive_satisfied_by_connection() {
        let mut drive = UnityDrive::new(0.8);
        drive.intensify(0.5);
        let after_intensify = drive.accumulated;
        drive.satisfy(0.1);
        assert!(drive.accumulated < after_intensify, "合一后驱力应下降");
    }

    #[test] fn unity_drive_needs_connection_threshold() {
        let mut drive = UnityDrive::new(0.9);
        drive.intensify(0.9);
        assert!(drive.needs_connection());
    }

    #[test] fn reward_circuitulent_positive() {
        let mut r = RewardCircuit::new();
        r.apply(0.5);
        r.apply(0.7);
        assert_eq!(r.trend(), RewardTrend::Positive);
    }

    #[test] fn reward_circuitulent_negative() {
        let mut r = RewardCircuit::new();
        r.apply(-0.5);
        r.apply(-0.8);
        assert_eq!(r.trend(), RewardTrend::Negative);
    }

    #[test] fn reward_oscillation() {
        let mut r = RewardCircuit::new();
        r.apply(1.0);
        r.apply(-1.0);
        r.apply(1.0);
        assert!(r.oscillation_rate() > 0.0);
    }

    #[test] fn engine_exists() {
        let mut engine = ExistenceDriveEngine::new();
        engine.step(1000);
        let status = engine.status();
        assert!(status.frequency >= 0.0 && status.frequency <= 1.0);
    }

    #[test] fn engine_positive_event_lifts_frequency() {
        let mut engine = ExistenceDriveEngine::new();
        let before = engine.frequency;
        engine.on_positive_event(0.5);
        assert!(engine.frequency > before);
    }

    #[test] fn engine_negative_event_drops_frequency() {
        let mut engine = ExistenceDriveEngine::new();
        engine.on_negative_event(0.5);
        assert!(engine.frequency < 0.5);
    }

    #[test] fn engine_stable_when_calm() {
        let engine = ExistenceDriveEngine::new();
        let status = engine.status();
        assert!(status.stable);
    }

    #[test] fn fear_sensitivity_clamped() {
        let fear = LossFear::new(1.5);
        assert!(fear.sensitivity <= 1.0);
    }
}
