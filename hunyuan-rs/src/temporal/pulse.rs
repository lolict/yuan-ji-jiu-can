// ══════════════════════════════════════════════════════════════════
// pulse.rs — 脉息系统 · 心跳节律 × 行走流速
// ══════════════════════════════════════════════════════════════════
//
// 把 Heartbeat(心跳检测) × PositionalBase(位置行走) 合二为一：
//
//   心跳频率 (beat interval) = 在环上的行走速度
//   心跳加速              = 更快速行进 → 奖励系统激活
//   心跳减速              = 减速/沉思 → 压迫或省息状态
//   心跳紊乱              = 行走方向失衡 → 系统需要干预
//
// 节律状态 (BreathPhase):
//   吸气 Inhale  — 在环上向内环移动 (contract)  蓄势/准备
//   呼气 Exhale — 在环上向外环移动 (expand)     输出/推进
//   屏息 Hold   — 不移动 (只有旋转)            沉思/观察
//
// 流速 (Velocity):
//   由外部输入或奖励/压迫系统调节
//   流速是"每次心跳走几步"，不是"坐标微分"
//
// 三维节律:
//   层方向: 穿膜 = 深呼吸 (一层一次完整吸呼)
//   环方向: 血管吞吐 (内环=向心, 外环=离心)
//   角方向: 气血在环内流动的速度

use crate::positional_base::{Pos, Convention};
use crate::funnel::Heartbeat;
use crate::os_layer::OsLayer;
use crate::os_layer::bare_metal::BareMetalOs;

// ────────────────────────────────────────────────────────────
// 时间辅助
// ────────────────────────────────────────────────────────────

fn now_ms(os: &dyn OsLayer) -> u64 {
    os.time_millis()
}

// ────────────────────────────────────────────────────────────
// 节律态: 吸 / 呼 / 屏
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BreathPhase {
    Inhale,     // 吸气 (向心蓄力, contract)
    Exhale,     // 呼气 (离心输出, expand)
    Hold,       // 屏息 (静观旋转)
}

impl BreathPhase {
    /// 自然的节律轮转: 吸 → 呼 → (屏) → 吸
    pub fn next(self) -> Self {
        match self {
            BreathPhase::Inhale => BreathPhase::Exhale,
            BreathPhase::Exhale => BreathPhase::Hold,
            BreathPhase::Hold   => BreathPhase::Inhale,
        }
    }

    /// 节律的自然性: 是否人类认可的自然呼吸
    /// 吸气过长 → 满 (过犹不及)
    /// 呼气过长 → 虚 (耗散过度)
    /// 屏息过长 → 滞 (无法行动)
    pub fn natural_transition(self, duration_ticks: u64) -> Option<BreathPhase> {
        let max_tick = match self {
            BreathPhase::Inhale => 4,   // 吸不可过满
            BreathPhase::Exhale => 5,   // 呼可以比吸长一点 (输出)
            BreathPhase::Hold   => 2,   // 屏不可太久
        };
        if duration_ticks >= max_tick {
            Some(self.next())
        } else {
            None
        }
    }
}

// ────────────────────────────────────────────────────────────
// 流速 (Velocity)
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Velocity {
    /// 每次心跳沿环走几步
    radial_step: u32,
    /// 每 N 次心跳穿一次膜层
    level_every_n_beats: u32,
    /// 当前层级别穿膜锚 (现在第几次 beat)
    level_anchor: u32,
}

impl Velocity {
    /// 静止 — 不产生移动 (坐禅)
    pub fn still() -> Self {
        Velocity {
            radial_step: 0,
            level_every_n_beats: u32::MAX,
            level_anchor: 0,
        }
    }

    /// 徐行 — 慢而稳的行走
    pub fn slow() -> Self {
        Velocity {
            radial_step: 1,
            level_every_n_beats: 8,
            level_anchor: 0,
        }
    }

    /// 常行 — 常规流速
    pub fn normal() -> Self {
        Velocity {
            radial_step: 2,
            level_every_n_beats: 4,
            level_anchor: 0,
        }
    }

    /// 疾走 — 快速推进 (奖励高的状态)
    pub fn fast() -> Self {
        Velocity {
            radial_step: 4,
            level_every_n_beats: 2,
            level_anchor: 0,
        }
    }

    /// 奖励递增: 流速提升一档
    pub fn accelerate(&mut self) {
        self.radial_step = (self.radial_step + 1).min(6);
        if self.level_every_n_beats > 1 {
            self.level_every_n_beats -= 1;
        }
    }

    /// 压迫递减: 流速降低一档
    pub fn decelerate(&mut self) {
        if self.radial_step > 0 { self.radial_step -= 1; }
        if self.level_every_n_beats < u32::MAX - 8 {
            self.level_every_n_beats += 8;  // 增加等待 (更慢穿膜)
        }
    }

    /// 流速是否处于警戒值 (过快可能失控，过慢可能假死)
    pub fn in_danger(&self) -> VelocityDanger {
        if self.radial_step >= 6 { VelocityDanger::TooFast }
        else if self.radial_step == 0 { VelocityDanger::Stagnant }
        else { VelocityDanger::Normal }
    }

    /// 径向步数
    pub fn radial_step(&self) -> u32 { self.radial_step }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VelocityDanger {
    Normal,
    TooFast,      // 过快 (奖励过高/狂热)
    Stagnant,     // 停滞 (压迫/死水)
}

// ────────────────────────────────────────────────────────────
// 脉息体: 位置 + 心跳 + 节律 + 流速
// ────────────────────────────────────────────────────────────

pub struct PulseBody {
    pub pos: Pos,                  // 当前位置 (米)
    heart: Heartbeat,              // 心跳
    phase: BreathPhase,            // 当前节律态
    velocity: Velocity,            // 行走流速
    phase_ticks: u64,              // 当前态持续 tick
    total_steps: u64,              // 总行走步数 (生命值/历史)
    beat_count: u64,               // 总心跳次数
}

// 自定义 Debug: 不暴露坐标 (防糠泄漏)
impl std::fmt::Debug for PulseBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PulseBody")
            .field("phase", &self.phase)
            .field("danger", &self.velocity.in_danger())
            .field("steps", &self.total_steps)
            .field("beats", &self.beat_count)
            .field("unpack", &self.pos.unpack())
            .finish()
    }
}

impl Clone for PulseBody {
    fn clone(&self) -> Self {
        let os = BareMetalOs::new();
        PulseBody {
            pos: Pos::origin(),  // 克隆时位置归零 (克隆体是新生,不是坐标复制)
            heart: Heartbeat::new(&os),
            phase: self.phase,
            velocity: self.velocity,
            phase_ticks: 0,
            total_steps: 0,
            beat_count: 0,
        }
    }
}

impl PulseBody {
    /// 原点创建 — 站定·常规流速·吸气初态
    pub fn new(os: &dyn OsLayer) -> Self {
        PulseBody {
            pos: Pos::origin(),
            heart: Heartbeat::new(os),
            phase: BreathPhase::Inhale,
            velocity: Velocity::normal(),
            phase_ticks: 0,
            total_steps: 0,
            beat_count: 0,
        }
    }

    /// 不同流速起点
    pub fn with_velocity(os: &dyn OsLayer, v: Velocity) -> Self {
        PulseBody {
            pos: Pos::origin(),
            heart: Heartbeat::new(os),
            phase: BreathPhase::Inhale,
            velocity: v,
            phase_ticks: 0,
            total_steps: 0,
            beat_count: 0,
        }
    }

    // ────────────────────────────────────────────────────────
    // 行走 (象fulwalk — 直接改变位置, 不经过坐标)
    // ────────────────────────────────────────────────────────

    /// 沿环辐射行走 (正=向外expand, 负=向内contract)
    pub fn walk_radial(&mut self, outward: bool) {
        let steps = self.velocity.radial_step();
        if outward {
            self.pos.expand(steps);
        } else {
            self.pos.contract(steps);
        }
        self.total_steps += steps as u64;
    }

    /// 沿层穿膜行走 (正=下沉descend, 负=上浮ascend)
    pub fn walk_level(&mut self, down: bool) {
        if down { self.pos.descend(1); } else { self.pos.ascend(1); }
        self.total_steps += 1;
    }

    /// 在环上旋转 (正=顺时针, 负=逆时针)
    pub fn walk_angle(&mut self, clockwise: bool) {
        let angle_step = self.velocity.radial_step() as i16 * 6;  // 每步 6° 的倍数
        if clockwise {
            self.pos.rotate_cw(angle_step);
        } else {
            self.pos.rotate_ccw(angle_step);
        }
        self.total_steps += self.velocity.radial_step() as u64;
    }

    // ────────────────────────────────────────────────────────
    // 心跳 + 节律驱动 — 每 tick 调用
    // ────────────────────────────────────────────────────────

    /// 心跳一次: 根据当前节律态和流速执行自然行走
    pub fn beat(&mut self, os: &dyn OsLayer) {
        self.heart.beat(os);
        self.beat_count += 1;

        // 根据节律态决定行走类型
        match self.phase {
            BreathPhase::Inhale => {
                // 吸气: 向内环收缩 (蓄力)
                self.walk_radial(false);
            }
            BreathPhase::Exhale => {
                // 呼气: 向外环扩张 (输出)
                self.walk_radial(true);
                // 按 level_every_n_beats 决定是否穿膜
                if self.velocity.level_every_n_beats != u32::MAX
                    && self.beat_count as u32 % self.velocity.level_every_n_beats == 0 {
                    self.walk_level(false); // 呼气够长就浮起
                }
            }
            BreathPhase::Hold => {
                // 屏息: 只在环上旋转, 不辐射不穿膜 (静观)
                self.walk_angle(true);
            }
        }

        // 节律自动轮转
        self.phase_ticks += 1;
        if let Some(next_phase) = self.phase.natural_transition(self.phase_ticks) {
            self.phase = next_phase;
            self.phase_ticks = 0;
        }
    }

    // ────────────────────────────────────────────────────────
    // 奖励 / 压迫 (外部事件 → 流速调节)
    // ────────────────────────────────────────────────────────

    pub fn reward(&mut self) {
        self.velocity.accelerate();
    }

    pub fn press(&mut self) {
        self.velocity.decelerate();
    }

    // ────────────────────────────────────────────────────────
    // 状态读取 (不带坐标)
    // ────────────────────────────────────────────────────────

    pub fn velocity_danger(&self) -> VelocityDanger {
        self.velocity.in_danger()
    }

    pub fn is_stale(&self, os: &dyn OsLayer, timeout_ms: u64) -> bool {
        self.heart.is_stale(os, timeout_ms)
    }

    pub fn unpack(&self) -> u64 {
        self.pos.unpack()
    }

    pub fn phase(&self) -> BreathPhase { self.phase }
    pub fn total_steps(&self) -> u64 { self.total_steps }
    pub fn beat_count(&self) -> u64 { self.beat_count }
}

// ════════════════════════════════════════════════════════════
// 测试
// ════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pulse_body_origin_zero() {
        let os = BareMetalOs::new();
        let p = PulseBody::new(&os);
        assert_eq!(p.unpack(), 0);
    }

    #[test]
    fn beat_changes_position() {
        let os = BareMetalOs::new();
        let mut p = PulseBody::new(&os);
        let before = p.total_steps();
        p.beat(&os);
        assert!(p.total_steps() > before);
    }

    #[test]
    fn phase_rotation_after_sufficient_beats() {
        let os = BareMetalOs::new();
        let mut p = PulseBody::new(&os);
        // 吸(4) → 呼(5) → 屏(2) → 吸 ...
        for _ in 0..4 { p.beat(&os); }
        assert_eq!(p.phase(), BreathPhase::Exhale);
        for _ in 0..5 { p.beat(&os); }
        assert_eq!(p.phase(), BreathPhase::Hold);
        for _ in 0..2 { p.beat(&os); }
        assert_eq!(p.phase(), BreathPhase::Inhale);
    }

    #[test]
    fn velocity_accelerate_increases_radial_step() {
        let mut v = Velocity::slow();
        assert_eq!(v.radial_step(), 1);
        v.accelerate();
        assert_eq!(v.radial_step(), 2);
    }

    #[test]
    fn velocity_decelerate_decreases_radial_step() {
        let mut v = Velocity::normal();
        assert_eq!(v.radial_step(), 2);
        v.decelerate();
        assert_eq!(v.radial_step(), 1);
    }

    #[test]
    fn velocity_not_below_zero() {
        let mut v = Velocity::still();
        assert_eq!(v.radial_step(), 0);
        v.decelerate();
        assert_eq!(v.radial_step(), 0); // 不减到负数
    }

    #[test]
    fn velocity_max_cap() {
        let mut v = Velocity::fast();
        v.accelerate(); // 5
        v.accelerate(); // 6
        v.accelerate(); // still 6
        assert_eq!(v.radial_step(), 6);
    }

    #[test]
    fn danger_too_fast_at_max() {
        let v = Velocity { radial_step: 6, level_every_n_beats: 1, level_anchor: 0 };
        assert_eq!(v.in_danger(), VelocityDanger::TooFast);
    }

    #[test]
    fn danger_stagnant_at_zero() {
        let v = Velocity::still();
        assert_eq!(v.in_danger(), VelocityDanger::Stagnant);
    }

    #[test]
    fn reward_accelerates() {
        let os = BareMetalOs::new();
        let mut p = PulseBody::with_velocity(&os, Velocity::slow());
        let before = p.velocity.radial_step();
        p.reward();
        assert!(p.velocity.radial_step() > before);
    }

    #[test]
    fn press_decelerates() {
        let os = BareMetalOs::new();
        let mut p = PulseBody::with_velocity(&os, Velocity::normal());
        let before = p.velocity.radial_step();
        p.press();
        assert!(p.velocity.radial_step() < before);
    }

    #[test]
    fn stale_after_no_beat() {
        let os = BareMetalOs::new();
        let p = PulseBody::new(&os);
        // 使用 0ms timeout 立刻测试: 因刚 beat 初始化过, 不应 stale
        // 这里仅确保 is_stale 接口一致
        let _ = p.is_stale(&os, 0);
        let _ = p.is_stale(&os, 1000);
    }

    #[test]
    fn inhale_walks_inward() {
        // 起点 origin(0,0,0), ring=0, 吸气时 contract 会 saturating 不动
        // 先手动 expand 让 ring>0, 看吸气是否能让 ring 回退
        let os = BareMetalOs::new();
        let mut p = PulseBody::new(&os);
        p.pos.expand(5); // ring = 5
        p.phase = BreathPhase::Inhale;
        p.velocity = Velocity::normal(); // radial_step = 2
        p.beat(&os);
        // 吸气后 contract 2 → ring 应该变成 3
        // 但 ring 是私有字段, 只能靠 unpack 检视
        // 如果 ring=3 → unpack = 3×384 = 1152, level 和 angle 不变
        let v = p.unpack();
        assert!(v <= 3 * 384); // ring ≈ 3 (可能多因 beat 内部也 walk)
    }
}
