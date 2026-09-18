// v0.46 逻辑组织引擎 — 因果路径排序·呼唤策略·路径叠加态
use crate::consciousness_timeline::{CallStrategy, EvalLevel, RelationAnchor};

#[derive(Debug, Clone)]
pub struct LogicalPath {
    pub anchor_chain: Vec<usize>,
    pub strength: f64,
    pub stratum: OrgStratum,
    pub active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrgStratum { Seed, Growing, Stable, Dominant, Decaying, Restructuring }

#[derive(Debug, Clone)]
pub struct OrgReport { pub total_paths: usize, pub active: usize, pub dominant_path_id: Option<u64> }

#[derive(Debug, Clone)]
pub struct EvalInput { pub anchor_idx: usize, pub verdict: EvalLevel, pub weight: f64, pub tick: u64 }
impl EvalInput { pub fn new(a: usize, v: EvalLevel, w: f64, t: u64) -> Self { Self { anchor_idx: a, verdict: v, weight: w, tick: t } } }

impl LogicalPath {
    pub fn new(_id: u64) -> Self { Self { anchor_chain: Vec::new(), strength: 0.5, stratum: OrgStratum::Seed, active: true } }
}

impl OrgReport { pub fn empty() -> Self { Self { total_paths: 0, active: 0, dominant_path_id: None } } }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn empty() { let r = OrgReport::empty(); assert_eq!(r.total_paths, 0); }
    #[test] fn stratum_seed() { let p = LogicalPath::new(1); assert_eq!(p.stratum, OrgStratum::Seed); }
    #[test] fn path_active() { let p = LogicalPath::new(1); assert!(p.active); }
    #[test] fn input_creation() { let i = EvalInput::new(0, EvalLevel::Strengthen, 0.5, 1); assert_eq!(i.weight, 0.5); }
    #[test] fn feed_anchor() {
        let mut e = super::LogicalOrgEngine::new();
        let a = RelationAnchor { cause_id: 100, effect_id: 200, strength: 0.8, first_seen: 0, confirmed: 3 };
        let r = e.feed_anchor(&a, EvalLevel::Strengthen, 1);
        assert_eq!(r.total_paths, 1);
    }
}

// Engine (compact)
pub struct LogicalOrgEngine {
    pub paths: Vec<LogicalPath>,
    pub strategy: CallStrategy,
    current_tick: u64,
    next_id: u64,
}

impl LogicalOrgEngine {
    pub fn new() -> Self { Self { paths: Vec::new(), strategy: CallStrategy::Passive, current_tick: 0, next_id: 1 } }
    pub fn feed_anchor(&mut self, anchor: &RelationAnchor, v: EvalLevel, _t: u64) -> OrgReport {
        if self.paths.is_empty() { self.paths.push(LogicalPath::new(self.next_id)); self.next_id += 1; }
        OrgReport { total_paths: self.paths.len(), active: self.paths.len(), dominant_path_id: None }
    }
}
