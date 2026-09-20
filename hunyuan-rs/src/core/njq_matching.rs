// njq_matching.rs — 聂小倩匹配引擎 (v0.1)
// 本质匹配 · 公约数桥梁 · 声母遍历韵母 · 撞墙回退
// ══════════════════════════════════════════════════════════════
//
//   核心命题 (满全法·聂小倩匹配):
//
//   聂小倩 = 游魂寻找匹配的容器。
//   匹配不是搜索, 而是"本质共振"。
//
//   匹配算法:
//   1. 声母遍历韵母: 在 shengmu×yunmu 空间中搜索
//   2. 公约数桥梁: 两容器的公共因子作为连接纽带
//   3. 撞墙回退: 当匹配不可行时, 回退并尝试替代路径
//
//   "撞墙" = 容器试图匹配但遇到不可逾越的差异,
//   此时系统不是强行突破, 而是记住这面墙,
//   并在未来的匹配中避开它。
//
// ══════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ─────────────────────────────────────────────────────────
// 游魂 (未匹配的容器)
// ─────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct WanderingSoul {
    pub id: u64,
    /// 本质特征 (用于匹配)
    pub essence: Essence,
    /// 匹配尝试次数
    pub attempt_count: u32,
    /// 已撞墙的容器 (不再尝试)
    pub walls: Vec<u64>,
    /// 是否已匹配
    pub matched: bool,
}

#[derive(Clone, Debug)]
pub struct Essence {
    /// 声母 (初始特征)
    pub shengmu: u8,
    /// 韵母 (韵特征)
    pub yunmu: u8,
    /// 声调 (调特征)
    pub tone: u8,
    /// 语义编码
    pub semantic_code: u64,
}

impl Essence {
    pub fn new(shengmu: u8, yunmu: u8, tone: u8) -> Self {
        Self { shengmu, yunmu, tone, semantic_code: 0 }
    }

    /// 音拍空间位置 (一维)
    pub fn syllable_position(&self) -> u32 {
        self.shengmu as u32 * 200 + self.yunmu as u32 * 5 + self.tone as u32
    }

    /// 与另一本质的距离
    pub fn distance(&self, other: &Essence) -> f64 {
        let dp = (self.syllable_position() as i64 - other.syllable_position() as i64).abs() as f64;
        let ds = (self.semantic_code as i64 - other.semantic_code as i64).abs() as f64;
        (dp + ds.min(1000.0)) / 2000.0
    }
}

impl WanderingSoul {
    pub fn new(id: u64, essence: Essence) -> Self {
        Self { id, essence, attempt_count: 0, walls: Vec::new(), matched: false }
    }

    /// 记录一面墙
    pub fn remember_wall(&mut self, container_id: u64) {
        if !self.walls.contains(&container_id) {
            self.walls.push(container_id);
        }
    }

    /// 是否已撞过这面墙
    pub fn has_hit_wall(&self, container_id: u64) -> bool {
        self.walls.contains(&container_id)
    }
}

// ─────────────────────────────────────────────────────────
// 匹配结果
// ─────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct NJQMatchResult {
    pub soul_id: u64,
    pub target_id: u64,
    pub strength: f64,
    pub via_gcd: u64,
    pub distance: f64,
}

impl NJQMatchResult {
    pub fn is_strong(&self) -> bool {
        self.strength > 0.7
    }

    pub fn is_viable(&self) -> bool {
        self.strength > 0.3 && self.distance < 0.5
    }
}

// ─────────────────────────────────────────────────────────
// 匹配引擎
// ─────────────────────────────────────────────────────────

pub struct NJQMatchingEngine {
    /// 容器池 (候选匹配目标)
    pub containers: HashMap<u64, Essence>,
    /// 游魂队列 (等待匹配)
    pub souls: Vec<WanderingSoul>,
    /// 成功匹配记录
    pub matches: Vec<NJQMatchResult>,
    /// 最佳匹配尝试次数上限
    pub max_attempts: u32,
}

impl NJQMatchingEngine {
    pub fn new() -> Self {
        Self {
            containers: HashMap::new(),
            souls: Vec::new(),
            matches: Vec::new(),
            max_attempts: 20,
        }
    }

    /// 注册容器
    pub fn add_container(&mut self, id: u64, essence: Essence) {
        self.containers.insert(id, essence);
    }

    /// 添加游魂
    pub fn add_soul(&mut self, soul: WanderingSoul) {
        self.souls.push(soul);
    }

    /// 为单个游魂寻找最佳匹配
    pub fn match_soul(&self, soul: &WanderingSoul) -> Option<NJQMatchResult> {
        let mut best: Option<NJQMatchResult> = None;
        for (id, essence) in &self.containers {
            if *id == soul.id { continue; }
            if soul.has_hit_wall(*id) { continue; }
            let gcd = compute_gcd(soul.essence.semantic_code, essence.semantic_code);
            let distance = soul.essence.distance(essence);
            let strength = if distance < 0.01 { 1.0 } else {
                (gcd as f64 / 100.0).min(1.0) * (1.0 - distance).max(0.0)
            };
            if best.as_ref().map_or(true, |b| strength > b.strength) {
                best = Some(NJQMatchResult {
                    soul_id: soul.id,
                    target_id: *id,
                    strength,
                    via_gcd: gcd,
                    distance,
                });
            }
        }
        best
    }

    /// 批量匹配所有游魂
    pub fn match_all(&mut self) -> Vec<NJQMatchResult> {
        let mut results = Vec::new();
        let soul_ids: Vec<u64> = self.souls.iter().map(|s| s.id).collect();
        for soul_id in soul_ids {
            if let Some(soul) = self.souls.iter().find(|s| s.id == soul_id) {
                if let Some(mr) = self.match_soul(soul) {
                    if mr.is_viable() {
                        results.push(mr.clone());
                        if let Some(s) = self.souls.iter_mut().find(|s| s.id == soul_id) {
                            s.matched = true;
                        }
                        self.matches.push(mr);
                    }
                }
            }
        }
        results
    }

    /// 撞墙回退: 记录失败并尝试替代
    pub fn handle_wall_collision(&mut self, soul_id: u64, wall_id: u64) {
        if let Some(soul) = self.souls.iter_mut().find(|s| s.id == soul_id) {
            soul.remember_wall(wall_id);
            soul.attempt_count += 1;
        }
    }

    /// 未匹配的游魂
    pub fn unmatched_souls(&self) -> Vec<&WanderingSoul> {
        self.souls.iter().filter(|s| !s.matched).collect()
    }

    /// 匹配率
    pub fn match_rate(&self) -> f64 {
        if self.souls.is_empty() { return 0.0; }
        let matched = self.souls.iter().filter(|s| s.matched).count();
        matched as f64 / self.souls.len() as f64
    }
}

fn compute_gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 || b == 0 { return 1; }
    while b != 0 { let t = b; b = a % b; a = t; }
    if a == 0 { 1 } else { a }
}

// ═══════════════════════════════════════════════════════════
// 测试
// ═══════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn essence_syllable_position() {
        let e = Essence::new(5, 10, 3);
        assert_eq!(e.syllable_position(), 5 * 200 + 10 * 5 + 3);
    }

    #[test] fn essence_distance_same() {
        let e = Essence::new(1, 2, 1);
        assert!(e.distance(&e) < 1e-10);
    }

    #[test] fn essence_distance_different() {
        let a = Essence::new(0, 0, 1);
        let b = Essence::new(20, 30, 5);
        assert!(a.distance(&b) > 0.0);
    }

    #[test] fn soul_no_walls_initially() {
        let s = WanderingSoul::new(1, Essence::new(1, 1, 1));
        assert!(!s.has_hit_wall(999));
    }

    #[test] fn soul_remembers_wall() {
        let mut s = WanderingSoul::new(1, Essence::new(1, 1, 1));
        s.remember_wall(42);
        assert!(s.has_hit_wall(42));
    }

    #[test] fn gcd_basic() {
        assert_eq!(compute_gcd(12, 8), 4);
        assert_eq!(compute_gcd(17, 13), 1);
    }

    #[test] fn engine_add_and_find() {
        let mut engine = NJQMatchingEngine::new();
        engine.add_container(10, Essence::new(5, 5, 1));
        engine.add_soul(WanderingSoul::new(1, Essence::new(5, 5, 1)));
        let result = engine.match_soul(&engine.souls[0]);
        assert!(result.is_some());
    }

    #[test] fn match_is_strong_for_similar() {
        let mut engine = NJQMatchingEngine::new();
        engine.add_container(10, Essence::new(5, 5, 1));
        engine.add_soul(WanderingSoul::new(1, Essence::new(5, 5, 1)));
        let r = engine.match_soul(&engine.souls[0]).unwrap();
        assert!(r.is_viable());
    }

    #[test] fn wall_avoids_retry() {
        let mut engine = NJQMatchingEngine::new();
        engine.add_container(10, Essence::new(5, 5, 1));
        let mut soul = WanderingSoul::new(1, Essence::new(5, 5, 1));
        soul.remember_wall(10);
        engine.add_soul(soul);
        let r = engine.match_soul(&engine.souls[0]);
        assert!(r.is_none() || r.unwrap().target_id != 10);
    }

    #[test] fn match_all_produces_results() {
        let mut engine = NJQMatchingEngine::new();
        engine.add_container(10, Essence::new(5, 5, 1));
        engine.add_container(11, Essence::new(6, 6, 2));
        engine.add_soul(WanderingSoul::new(1, Essence::new(5, 5, 1)));
        let results = engine.match_all();
        assert!(!results.is_empty());
    }

    #[test] fn handle_wall_collision() {
        let mut engine = NJQMatchingEngine::new();
        engine.add_soul(WanderingSoul::new(1, Essence::new(1, 1, 1)));
        engine.handle_wall_collision(1, 100);
        assert!(engine.souls[0].has_hit_wall(100));
        assert_eq!(engine.souls[0].attempt_count, 1);
    }

    #[test] fn unmatched_souls_tracking() {
        let mut engine = NJQMatchingEngine::new();
        engine.add_soul(WanderingSoul::new(1, Essence::new(1, 1, 1)));
        assert_eq!(engine.unmatched_souls().len(), 1);
    }

    #[test] fn match_rate_empty() {
        let engine = NJQMatchingEngine::new();
        assert_eq!(engine.match_rate(), 0.0);
    }

    #[test] fn match_rate_partial() {
        let mut engine = NJQMatchingEngine::new();
        engine.add_container(10, Essence::new(5, 5, 1));
        engine.add_soul(WanderingSoul::new(1, Essence::new(5, 5, 1)));
        let _ = engine.match_all();
        assert!(engine.match_rate() > 0.0);
    }
}
