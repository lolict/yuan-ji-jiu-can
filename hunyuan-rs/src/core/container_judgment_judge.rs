pub use crate::cross_isa::fridge::ContainerPack;
use crate::cross_isa::fridge::Verdict;

#[derive(Debug, Clone)]
pub enum JudgmentResult {
    Allow,
    Deny,
    Defer,
}

pub struct JudgmentEngine;

impl Default for JudgmentEngine {
    fn default() -> Self { Self }
}

impl JudgmentEngine {
    pub fn new() -> Self { Self }
    pub fn judge(&self, _pack: &ContainerPack) -> bool { true }
    pub fn judge_batch(&self, _packs: &[ContainerPack]) -> Vec<JudgmentResult> {
        Vec::new()
    }
}

pub fn stub() {}
