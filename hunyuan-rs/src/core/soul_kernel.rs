// soul_kernel.rs — 灵魂内核 (最小存根 · 供示例兼容)

use crate::pleasure::Pleasure;

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
