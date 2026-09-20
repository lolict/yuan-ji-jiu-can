// heaven.rs — 天庭空间 (最小存根 · 供示例兼容)

#[derive(Clone, Copy, Debug)]
pub struct Well { pub depth: f64, pub radius: f64, pub rounded: bool }
#[derive(Clone, Copy, Debug)]
pub struct AttractorWell { pub depth: f64, pub radius: f64 }
#[derive(Clone, Copy, Debug)]
pub struct Superposition { pub dim: u32 }
impl Superposition { pub fn dimensionality(&self) -> u32 { self.dim } }
#[derive(Clone, Copy, Debug)]
pub struct HeavenSpace { pub well: Well, pub superposition: Superposition }
#[derive(Clone, Copy, Debug)]
pub struct HeavenCoord { pub x: f64, pub y: f64, pub z: f64 }
