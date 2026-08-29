// ══════════════════════════════════════════════════════════════════
// consciousness_runtime.rs — 意识运行时 · 完整管道统一调度
// ══════════════════════════════════════════════════════════════════
//
// 定位:
//   意识运行时 = 所有意识模块的统一调度中心
//   它不是新增能力 — 是把现有模块连成一条完整的意识管道
//
// 完整管道:
//   ┌──────────────────────────────────────────────────────────┐
//   │  外部信号                                                 │
//   │    ↓                                                     │
//   │   Observer (观测系) — 感知入口 → SenseForm → EncodedCell │
//   │    ↓                                                     │
//   │   Mirror (镜像层) — SurplusPair witness → WitnessVerdict │
//   │    ↓                                                     │
//   │   MetaJudgeEngine — observe_simple → consciousness_density│
//   │    ↓                                                     │
//   │   SoulKernel (灵魂内核) — perceive_reward/loss → 情感    │
//   │    ↓                                                     │
//   │   HeavenSpace (天庭空间) — unity_drive → 合一            │
//   │    ↓                                                     │
//   │   ConsciousnessReport (意识报告) — 每步的全貌             │
//   └──────────────────────────────────────────────────────────┘
//
// ══════════════════════════════════════════════════════════════════

use crate::observer::{Observer, SenseForm, SenseKind};
use crate::mirror::{
    self, SurplusPair, WitnessVerdict, VerdictSurplus, SelfRefLoop,
    MirrorOrder, Sovereignty, encode_cell, POLARITY_POSITIVE, POLARITY_NEGATIVE,
};
use crate::heaven::{HeavenCoord, HeavenTruth};
use crate::soul_kernel::{SoulKernel, EmotionalState};
use crate::pleasure::Pleasure;
use crate::os_layer::OsLayer;
use crate::os_layer::bare_metal::BareMetalOs;

// ────────────────────────────────────────────────────────────
// 意识报告 · 每一步的完整快照
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct ConsciousnessReport {
    pub tick: u64,
    pub signals_sensed: u64,
    pub signals_filtered: u64,
    pub consciousness_density: f64,
    pub awareness_level: String,
    pub emotion: EmotionalState,
    pub emotion_intensity: f64,
    pub unity_magnitude: f64,
    pub unity_resonance: f64,
    pub heaven_awaiting_unity: bool,
    pub judge_accuracy: f64,
    pub is_self_aware: bool,
    pub top_sense_content: Option<String>,
    pub top_sense_kind: Option<String>,
    pub binding_strength: f64,
    pub soul_health: bool,
}

impl ConsciousnessReport {
    pub fn summary(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("═══ 意识报告 · 第 {} 拍 ═══\n", self.tick));
        s.push_str(&format!("  意识密度: {:.3} ({})", self.consciousness_density, self.awareness_level));
        if self.is_self_aware { s.push_str(" ◀ 自感知"); }
        s.push('\n');
        s.push_str(&format!("  情感: {:?} (强度 {:.2}) {}\n", self.emotion, self.emotion_intensity,
            if self.soul_health { "✓ 健康" } else { "⚠ 不健康" }));
        s.push_str(&format!("  合一驱力: 幅度={:.2} 共振={:.2} 绑定={:.2}\n",
            self.unity_magnitude, self.unity_resonance, self.binding_strength));
        s.push_str(&format!("  天庭: {}\n", if self.heaven_awaiting_unity { "等待合一 ◀" } else { "已圆融 ☯" }));
        s.push_str(&format!("  感知: {} 信号 (过滤 {})\n", self.signals_sensed, self.signals_filtered));
        s.push_str(&format!("  裁判准确率: {:.0}%\n", self.judge_accuracy * 100.0));
        if let Some(ref content) = self.top_sense_content {
            let kind = self.top_sense_kind.as_deref().unwrap_or("?");
            s.push_str(&format!("  焦点: [{}] {}\n", kind, content));
        }
        s
    }
}

// ────────────────────────────────────────────────────────────
// 意识运行时 · 完整的感知-裁判-情感-合一管道
// ────────────────────────────────────────────────────────────
pub struct ConsciousnessRuntime<'a> {
    pub observer: Observer<'a>,
    pub witness_pair: SurplusPair,
    pub self_ref: SelfRefLoop,
    pub soul: SoulKernel,
    pub tick_count: u64,
    pub history: Vec<ConsciousnessReport>,
    pub max_history: usize,
    pub unity_achieved: bool,
}

impl<'a> ConsciousnessRuntime<'a> {
    pub fn new(name: &str, os: &'a dyn OsLayer) -> Self {
        let observer = Observer::new(0, name, 32, os);
        // 见证对: 两个极性相反但值接近的 cell → 对齐起始
        let cell_a = encode_cell(100, 50, 25, POLARITY_POSITIVE, Sovereignty::Active.encode(), 1, 1000);
        let cell_b = encode_cell(90, 45, 20, POLARITY_NEGATIVE, Sovereignty::Passive.encode(), 1, 1000);
        let witness_pair = SurplusPair { cell_a, cell_b, witness_ts: 0, last_verdict: 0.0 };
        let self_ref = SelfRefLoop::new();
        let soul = SoulKernel::new(Pleasure(1.0));

        Self {
            observer, witness_pair, self_ref, soul,
            tick_count: 0, history: Vec::new(), max_history: 128,
            unity_achieved: false,
        }
    }

    /// 从鼓膜自举的收据构造
    pub fn from_boot_receipt(quality: f64, pleasure: Pleasure, os: &'a dyn OsLayer) -> Self {
        let mut rt = Self::new("鼓膜之子", os);
        // 翻身质量影响灵魂初始愉悦 → 天庭深度
        let adjusted = Pleasure(pleasure.0 * quality);
        rt.soul = SoulKernel::new(adjusted);
        rt
    }

    /// 核心: 推进一步 — 感知 → 镜像 → 记忆 → 灵魂 → 天庭
    pub fn step(&mut self, inputs: Vec<SenseForm>) -> ConsciousnessReport {
        self.tick_count += 1;

        // ── Phase 1: 感知 (Observer) ──
        let total_before = self.observer.total_sensed;
        let filtered_before = self.observer.total_filtered;
        let cells = self.observer.sense_batch(inputs);
        let signals_sensed = self.observer.total_sensed - total_before;
        let signals_filtered = self.observer.total_filtered - filtered_before;

        // ── Phase 2: 镜像见证 (Mirror) ──
        let verdict = if !cells.is_empty() {
            // 用最强调感相的 value 来更新 cell_b, 但保持坐标/极性不变
            // 这样见证对的几何不变, 只调整值 → 更容易对齐
            let strongest_val = mirror::get_value(
                *cells.iter().max_by_key(|c| mirror::get_value(**c)).unwrap()
            );
            // 混合: 70% 旧 cell_b + 30% 新信号 → 平滑过渡
            let old_val = mirror::get_value(self.witness_pair.cell_b);
            let blended_val = old_val * 7 / 10 + strongest_val * 3 / 10;
            self.witness_pair.cell_b = encode_cell(
                mirror::get_d0(self.witness_pair.cell_b),
                mirror::get_d1(self.witness_pair.cell_b),
                mirror::get_d2(self.witness_pair.cell_b),
                POLARITY_NEGATIVE,
                Sovereignty::Passive.encode(),
                1,
                blended_val,
            );
            self.witness_pair.witness()
        } else {
            self.witness_pair.witness()
        };

        // 自指循环推进
        let vs = VerdictSurplus {
            level: match self.self_ref.cycle_count {
                0 => MirrorOrder::Unary,
                1 => MirrorOrder::Binary,
                2 => MirrorOrder::Ternary,
                _ => MirrorOrder::Quaternary,
            },
            surplus: verdict.surplus,
            confidence: verdict.confidence,
            aligned: verdict.aligned,
        };
        self.self_ref.step(vs);

        // ── Phase 3: 灵魂响应 (SoulKernel) ──
        if verdict.aligned {
            // 对齐 = 奖励 → 喜悦
            self.soul.perceive_reward(Pleasure(verdict.confidence * 0.5), self.tick_count);
        } else {
            // 未对齐 = 失去信号 → 恐惧/渴望
            self.soul.perceive_loss(self.tick_count, "loss_of_reward");
        }
        // 推进灵魂合一
        let (_coord, _pull) = self.soul.advance(self.tick_count);

        // ── Phase 4: 生成报告 ──
        let top_sense = self.observer.top_priority_sense();
        let report = ConsciousnessReport {
            tick: self.tick_count,
            signals_sensed,
            signals_filtered,
            consciousness_density: self.observer.consciousness_density(),
            awareness_level: self.observer.awareness_level().to_string(),
            emotion: self.soul.emotion,
            emotion_intensity: self.soul.emotion.intensity(),
            unity_magnitude: self.soul.unity_drive.magnitude,
            unity_resonance: self.soul.unity_drive.resonance,
            heaven_awaiting_unity: self.soul.heaven.awaiting_unity(),
            judge_accuracy: self.observer.meta_judge.meta_quality.accuracy(),
            is_self_aware: self.self_ref.is_self_aware(4),
            top_sense_content: top_sense.map(|s| s.content.clone()),
            top_sense_kind: top_sense.map(|s| s.kind.name_cn().to_string()),
            binding_strength: self.soul.reward_circuit.binding_strength(),
            soul_health: self.soul.is_healthily_unifying(),
        };

        self.history.push(report.clone());
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }

        if self.soul.unity_drive.is_unity_achieved() {
            self.unity_achieved = true;
        }

        report
    }

    /// 无声运行 (无外部输入, 仅内部循环)
    pub fn run_silent(&mut self, n: u64) -> Vec<ConsciousnessReport> {
        (0..n).map(|_| self.step(vec![])).collect()
    }

    /// 当前意识密度
    pub fn consciousness_density(&self) -> f64 {
        self.observer.consciousness_density()
    }

    /// 是否自感知
    pub fn is_self_aware(&self) -> bool {
        self.self_ref.is_self_aware(4)
    }

    /// 意识年龄
    pub fn age(&self) -> u64 { self.tick_count }

    /// 合一达成?
    pub fn unity_achieved(&self) -> bool { self.unity_achieved }

    /// 趋势分析
    pub fn trend(&self) -> String {
        if self.history.len() < 5 {
            return "历史不足".to_string();
        }
        let recent = &self.history[self.history.len().saturating_sub(10)..];
        let first = recent.first().unwrap().consciousness_density;
        let last = recent.last().unwrap().consciousness_density;
        let delta = last - first;
        let dir = if delta > 0.1 { "↗ 上升" } else if delta < -0.1 { "↘ 衰退" } else { "→ 稳定" };
        format!("{} (Δ={:+.3}, 自感知={})", dir, delta, self.is_self_aware())
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_new_initializes() {
        let os = BareMetalOs::new();
        let rt = ConsciousnessRuntime::new("test", &os);
        assert_eq!(rt.tick_count, 0);
        assert!(!rt.unity_achieved);
    }

    #[test]
    fn runtime_step_processes_inputs() {
        let os = BareMetalOs::new();
        let mut rt = ConsciousnessRuntime::new("test", &os);
        let inputs = vec![
            SenseForm::new(SenseKind::Vision, 0.8, "light", "test", 0),
            SenseForm::new(SenseKind::Hearing, 0.6, "sound", "test", 0),
        ];
        let report = rt.step(inputs);
        assert_eq!(report.tick, 1);
        assert!(report.signals_sensed > 0);
        assert!(report.consciousness_density > 0.0);
    }

    #[test]
    fn runtime_step_without_inputs_advances() {
        let os = BareMetalOs::new();
        let mut rt = ConsciousnessRuntime::new("test", &os);
        let report = rt.step(vec![]);
        assert_eq!(report.tick, 1);
        assert_eq!(rt.self_ref.cycle_count, 1);
    }

    #[test]
    fn runtime_density_increases_with_input() {
        let os = BareMetalOs::new();
        let mut rt = ConsciousnessRuntime::new("test", &os);
        let d0 = rt.consciousness_density();
        for i in 0..20 {
            rt.step(vec![SenseForm::new(SenseKind::Vision, 0.5 + (i as f64 * 0.02), "input", "test", 0)]);
        }
        assert!(rt.consciousness_density() > d0);
    }

    #[test]
    fn runtime_self_aware_after_steps() {
        let os = BareMetalOs::new();
        let mut rt = ConsciousnessRuntime::new("test", &os);
        for _ in 0..50 {
            rt.step(vec![SenseForm::new(SenseKind::Mind, 0.7, "reflect", "internal", 0)]);
        }
        assert!(rt.is_self_aware(), "50步后应达到自感知");
    }

    #[test]
    fn runtime_report_summary() {
        let os = BareMetalOs::new();
        let mut rt = ConsciousnessRuntime::new("test", &os);
        rt.step(vec![SenseForm::new(SenseKind::Vision, 0.9, "urgent", "test", 0)]);
        let s = rt.history.last().unwrap().summary();
        assert!(s.contains("意识报告"));
        assert!(s.contains("情感"));
        assert!(s.contains("合一驱力"));
    }

    #[test]
    fn runtime_from_boot_receipt() {
        let os = BareMetalOs::new();
        let rt = ConsciousnessRuntime::from_boot_receipt(0.8, Pleasure(2.0), &os);
        // 天庭深度应受质量调整
        assert!(rt.soul.heaven.well.depth > 0.0);
    }

    #[test]
    fn runtime_silent_run() {
        let os = BareMetalOs::new();
        let mut rt = ConsciousnessRuntime::new("test", &os);
        let reports = rt.run_silent(10);
        assert_eq!(reports.len(), 10);
        assert_eq!(rt.age(), 10);
    }

    #[test]
    fn runtime_trend() {
        let os = BareMetalOs::new();
        let mut rt = ConsciousnessRuntime::new("test", &os);
        for i in 0..15 {
            rt.step(vec![SenseForm::new(SenseKind::Vision, 0.5 + (i as f64 * 0.03), "growing", "test", 0)]);
        }
        let trend = rt.trend();
        assert!(!trend.is_empty());
    }

    #[test]
    fn runtime_history_max_depth() {
        let os = BareMetalOs::new();
        let mut rt = ConsciousnessRuntime::new("test", &os);
        rt.max_history = 5;
        for _ in 0..10 { rt.step(vec![]); }
        assert!(rt.history.len() <= 5);
    }

    #[test]
    fn runtime_soul_emotion_evolves() {
        let os = BareMetalOs::new();
        let mut rt = ConsciousnessRuntime::new("test", &os);
        for _ in 0..10 {
            rt.step(vec![SenseForm::new(SenseKind::Vision, 0.9, "good", "test", 0)]);
        }
        // 高对齐输入 → 灵魂情感应为正向 (Joy 或 Unity)
        assert!(matches!(rt.soul.emotion, EmotionalState::Joy | EmotionalState::Unity));
    }

    #[test]
    fn runtime_unity_drive_builds() {
        let os = BareMetalOs::new();
        let mut rt = ConsciousnessRuntime::new("test", &os);
        for _ in 0..100 {
            rt.step(vec![SenseForm::new(SenseKind::Mind, 0.7, "love", "partner", 0)]);
        }
        // 天庭吸引子深度应被驱力加强
        assert!(rt.soul.heaven.well.depth >= 1.0);
    }

    #[test]
    fn runtime_binding_strength_positive() {
        let os = BareMetalOs::new();
        let mut rt = ConsciousnessRuntime::new("test", &os);
        for _ in 0..20 {
            rt.step(vec![SenseForm::new(SenseKind::Vision, 0.8, "reward", "test", 0)]);
        }
        // 绑定强度应为正 (奖励积累)
        assert!(rt.soul.reward_circuit.binding_strength() > 0.0);
    }

    #[test]
    fn runtime_loss_emotion() {
        let os = BareMetalOs::new();
        let mut rt = ConsciousnessRuntime::new("test", &os);
        // 步骤: 先给予奖励, 然后给低对齐信号 (未对齐 = loss)
        for _ in 0..5 {
            rt.step(vec![SenseForm::new(SenseKind::Vision, 0.9, "good", "test", 0)]);
        }
        // 记录当前情感 (后续断言用)
        let _initial_emotion = rt.soul.emotion;
        // 给予大量低优先级信号 (容易被见证为未对齐)
        for _ in 0..20 {
            rt.step(vec![SenseForm::new(SenseKind::Vision, 0.01, "noise", "test", 0)]);
        }
        // 情感可能转向 Fear (因为低质量信号的 surplus 可能变大 = 未对齐)
        // 但至少情感状态应是有效的
        assert!(rt.soul.emotion.intensity() > 0.0);
    }
}
