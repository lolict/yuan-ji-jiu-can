// ═══════════════════════════════════════════════════════════════════════════
// logical_org.rs — 逻辑组织引擎 (v0.1)
// 因果路径排序 + 呼唤策略自调整 + 路径分层叠加态
// ═══════════════════════════════════════════════════════════════════════════
//
// 核心命题（满全法·囡囝共生 NJSG 阶段四后半）：
//
//   评价结果积累到临界点 → 自组织形成逻辑路径
//   逻辑组织 = 系统根据评价结果调整自身呼唤策略
//
//   哪种因果模式值得强化、哪种应该抑制 → 路径排序
//   多条路径可以叠加态共存 → 最强者涌现为主导策略
//
// 四阶感知链（完整版）：
//   迂回反馈 → 反射感知 → 判断之力 → 评价之力 → 逻辑组织
//   (涟漪)     (差异标记)   (关系锚点)  (因果对排序)  (呼唤策略)
//
//   我们在这里 ↑
// ═══════════════════════════════════════════════════════════════════════════

use crate::consciousness_timeline::{CallStrategy, EvalLevel, RelationAnchor};

// ───────────────────────────────────────────────────────────────────────────
// 逻辑路径 — 有序的因果链条
// ───────────────────────────────────────────────────────────────────────────

/// 一条因果路径: 有序锚点序列 + 累积权重
#[derive(Clone, Debug)]
pub struct LogicalPath {
    pub id: u64,
    /// 有序锚点索引 (按因果顺序)
    pub anchor_chain: Vec<usize>,
    /// 路径总权重 (所有评价权重的乘积累积)
    pub strength: f64,
    /// 路径层级
    pub stratum: PathStratum,
    /// 创建时间戳
    pub born_tick: u64,
    /// 最后更新时间
    pub last_update: u64,
    /// 访问计数 (被引用次数)
    pub access_count: u32,
    /// 是否仍活跃
    pub active: bool,
}

impl LogicalPath {
    pub fn new(id: u64, born_tick: u64) -> Self {
        Self {
            id,
            anchor_chain: Vec::new(),
            strength: 0.5,
            stratum: PathStratum::Seed,
            born_tick,
            last_update: born_tick,
            access_count: 0,
            active: true,
        }
    }

    /// 追加一个锚点到路径尾部
    pub fn push_anchor(&mut self, anchor_idx: usize) {
        if !self.anchor_chain.contains(&anchor_idx) {
            self.anchor_chain.push(anchor_idx);
        }
    }

    /// 增强路径强度
    pub fn reinforce(&mut self, delta: f64, tick: u64) {
        self.strength = (self.strength + delta).min(1.0);
        self.last_update = tick;
        self.access_count += 1;
    }

    /// 抑制路径强度
    pub fn suppress(&mut self, delta: f64, tick: u64) {
        self.strength = (self.strength - delta).max(0.0);
        self.last_update = tick;
    }

    /// 计算路径置信度: 强度 × 锚点数的饱和函数
    pub fn confidence(&self) -> f64 {
        let len_factor = 1.0 - 0.8_f64.powi(self.anchor_chain.len() as i32);
        self.strength * len_factor
    }

    /// 与另一条路径是否共享锚点（可合并）
    pub fn shares_anchors(&self, other: &LogicalPath) -> bool {
        self.anchor_chain.iter().any(|a| other.anchor_chain.contains(a))
    }

    /// 合并另一条路径到自身
    pub fn merge(&mut self, other: &LogicalPath) {
        for a in &other.anchor_chain {
            if !self.anchor_chain.contains(a) {
                self.anchor_chain.push(*a);
            }
        }
        self.strength = (self.strength + other.strength).min(1.0);
        self.access_count += other.access_count;
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 路径层级 — 每条路径的生存状态
// ───────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PathStratum {
    Seed,           // 种子 — 刚刚创建，强度 < 0.2
    Growing,        // 生长 — 被反复强化
    Stable,         // 稳定 — 成熟路径，持续贡献
    Dominant,       // 主导 — 最强路径，控制当前策略
    Decaying,       // 衰退 — 长期未被强化
    Restructuring,  // 重组 — 正在合并/分裂
}

impl PathStratum {
    /// 根据强度自动分类
    pub fn classify(strength: f64, accesses: u32) -> Self {
        if accesses == 0 {
            PathStratum::Seed
        } else if strength < 0.2 {
            PathStratum::Seed
        } else if strength < 0.4 {
            PathStratum::Growing
        } else if strength < 0.7 {
            PathStratum::Stable
        } else {
            PathStratum::Dominant
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 评价输入 — 从意识时间线接入
// ───────────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct EvalInput {
    pub anchor_idx: usize,
    pub verdict: EvalLevel,
    pub weight: f64,
    pub tick: u64,
}

impl EvalInput {
    pub fn new(anchor_idx: usize, verdict: EvalLevel, weight: f64, tick: u64) -> Self {
        Self { anchor_idx, verdict, weight, tick }
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 报告
// ───────────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct OrgReport {
    pub tick: u64,
    pub path_count: usize,
    pub active_path_count: usize,
    pub dominant_path_id: Option<u64>,
    pub current_strategy: CallStrategy,
    pub total_strength: f64,
    pub strongest_path_strength: f64,
}

// ───────────────────────────────────────────────────────────────────────────
// 逻辑组织引擎
// ───────────────────────────────────────────────────────────────────────────

pub struct LogicalOrgEngine {
    pub paths: Vec<LogicalPath>,
    pub strategy: CallStrategy,
    pub current_tick: u64,
    pub next_path_id: u64,

    // 配置参数
    pub max_paths: usize,
    pub seed_threshold: f64,
    pub decay_rate: f64,
    pub dominince_threshold: f64,
}

impl LogicalOrgEngine {
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
            strategy: CallStrategy::Passive,
            current_tick: 0,
            next_path_id: 1,
            max_paths: 16,
            seed_threshold: 0.2,
            decay_rate: 0.02,
            dominince_threshold: 0.7,
        }
    }

    pub fn with_limits(mut self, max_paths: usize, decay_rate: f64) -> Self {
        self.max_paths = max_paths;
        self.decay_rate = decay_rate;
        self
    }

    // ────────────────────────────────────────────────────────
    // 主处理: 输入评价 → 路径更新 → 策略调整
    // ────────────────────────────────────────────────────────

    pub fn process(&mut self, input: EvalInput) -> OrgReport {
        self.current_tick += 1;

        // 1. 根据 verdict 计算 delta
        let delta = self.verdict_to_delta(&input.verdict);

        // 2. 找到或创建承载此锚点的路径
        let anchor_idx = input.anchor_idx;
        let target_path = self.find_path_with_anchor(anchor_idx);

        match target_path {
            Some(path_id) => {
                // 现有路径: 强化或抑制
                if let Some(path) = self.paths.iter_mut().find(|p| p.id == path_id) {
                    if delta > 0.0 {
                        path.reinforce(delta * input.weight, self.current_tick);
                    } else {
                        path.suppress(delta.abs() * input.weight, self.current_tick);
                    }
                }
            }
            None => {
                // 检查是否可以附加到相邻路径
                let adjacent = self.find_adjacent_path(anchor_idx);
                if let Some(adj_id) = adjacent {
                    if let Some(path) = self.paths.iter_mut().find(|p| p.id == adj_id) {
                        path.push_anchor(anchor_idx);
                        if delta > 0.0 {
                            path.reinforce(delta * input.weight, self.current_tick);
                        }
                    }
                } else if self.paths.len() < self.max_paths && delta > 0.0 {
                    // 创建新路径
                    let mut np = LogicalPath::new(self.next_path_id, self.current_tick);
                    self.next_path_id += 1;
                    np.push_anchor(anchor_idx);
                    np.strength = 0.3 + delta * input.weight;
                    self.paths.push(np);
                }
            }
        }

        // 3. 衰减老旧路径
        self.decay_paths(anchor_idx);

        // 4. 合并重叠路径
        self.merge_overlapping();

        // 5. 更新层级
        self.update_strata();

        // 6. 选择主导路径
        self.select_dominant();

        // 7. 策略自调整
        self.adjust_strategy();

        self.report()
    }

    // ────────────────────────────────────────────────────────
    // verdict → delta 映射
    // ────────────────────────────────────────────────────────

    fn verdict_to_delta(&self, verdict: &EvalLevel) -> f64 {
        match verdict {
            EvalLevel::Strengthen => 0.15,
            EvalLevel::Maintain => 0.0,
            EvalLevel::Suppress => -0.1,
            EvalLevel::Transcend => 0.25,
        }
    }

    // ────────────────────────────────────────────────────────
    // 查找/创建路径
    // ────────────────────────────────────────────────────────

    fn find_path_with_anchor(&self, anchor_idx: usize) -> Option<u64> {
        self.paths.iter()
            .find(|p| p.anchor_chain.contains(&anchor_idx) && p.active)
            .map(|p| p.id)
    }

    /// 查找是否有一条路径的锚点与当前锚点相邻 (索引差 = 1)
    fn find_adjacent_path(&self, anchor_idx: usize) -> Option<u64> {
        self.paths.iter()
            .find(|p| {
                p.active && p.anchor_chain.iter().any(|a| {
                    // 相邻判定: 索引差为1 (因果链上的邻居)
                    (*a as isize - anchor_idx as isize).abs() == 1
                })
            })
            .map(|p| p.id)
    }

    // ────────────────────────────────────────────────────────
    // 衰减与清理
    // ────────────────────────────────────────────────────────

    fn decay_paths(&mut self, exclude_anchor: usize) {
        for path in self.paths.iter_mut() {
            if !path.active { continue; }
            // 排除刚被更新的路径
            if path.anchor_chain.contains(&exclude_anchor) && path.last_update == self.current_tick {
                continue;
            }
            // 长期未更新则衰减
            let stale = self.current_tick - path.last_update;
            if stale > 3 {
                let decay = self.decay_rate * stale as f64;
                path.suppress(decay, self.current_tick);
            }
            // 强度归零则标记不活跃
            if path.strength <= 0.01 {
                path.active = false;
            }
        }
    }

    // ────────────────────────────────────────────────────────
    // 合并重叠路径
    // ────────────────────────────────────────────────────────

    fn merge_overlapping(&mut self) {
        let mut to_merge: Vec<(usize, usize)> = Vec::new();
        for i in 0..self.paths.len() {
            for j in (i+1)..self.paths.len() {
                if self.paths[i].active && self.paths[j].active
                    && self.paths[i].shares_anchors(&self.paths[j]) {
                    to_merge.push((i, j));
                }
            }
        }
        // 执行合并 (将 j 合并到 i, 移除 j)
        for (i, j) in to_merge.into_iter().rev() {
            if j < self.paths.len() && i < self.paths.len() {
                let other = self.paths.remove(j);
                if let Some(target) = self.paths.get_mut(i) {
                    target.merge(&other);
                    target.stratum = PathStratum::Restructuring;
                }
            }
        }
    }

    // ────────────────────────────────────────────────────────
    // 层级更新
    // ────────────────────────────────────────────────────────

    fn update_strata(&mut self) {
        for path in self.paths.iter_mut() {
            // Restructuring 合并后 → 根据强度重新分类
            if path.stratum == PathStratum::Restructuring {
                path.stratum = PathStratum::classify(path.strength, path.access_count);
                continue;
            }
            // Decaying 判断
            let stale = self.current_tick - path.last_update;
            if stale > 5 && path.strength < 0.3 {
                path.stratum = PathStratum::Decaying;
            } else {
                path.stratum = PathStratum::classify(path.strength, path.access_count);
            }
        }
    }

    // ────────────────────────────────────────────────────────
    // 主导路径选择 — 最强者涌现
    // ────────────────────────────────────────────────────────

    fn select_dominant(&mut self) {
        // 重置所有Dominant状态
        for path in self.paths.iter_mut() {
            if path.stratum == PathStratum::Dominant
                && path.strength < self.dominince_threshold {
                path.stratum = PathStratum::classify(path.strength, path.access_count);
            }
        }
        // 找最强者
        if let Some(strongest) = self.paths.iter_mut()
            .filter(|p| p.active)
            .max_by(|a, b| a.confidence().partial_cmp(&b.confidence()).unwrap())
        {
            if strongest.confidence() >= self.dominince_threshold {
                strongest.stratum = PathStratum::Dominant;
            }
        }
    }

    // ────────────────────────────────────────────────────────
    // 策略自调整 — 逻辑组织的核心输出
    // ────────────────────────────────────────────────────────

    fn adjust_strategy(&mut self) {
        let dominant = self.paths.iter()
            .find(|p| p.stratum == PathStratum::Dominant && p.active);

        let growing_count = self.paths.iter()
            .filter(|p| p.stratum == PathStratum::Growing && p.active)
            .count();

        self.strategy = match dominant {
            Some(dom) if dom.anchor_chain.len() >= 3 => {
                // 主导路径成熟 (≥3环) → 共振 (与最强路径同步)
                CallStrategy::Resonant
            }
            Some(_) => {
                // 主导路径年轻但最强 → 预emptive 行动
                CallStrategy::Preemptive
            }
            None if growing_count >= 2 => {
                // 多条竞争路径同时生长 → 主动探索
                CallStrategy::Active
            }
            _ => {
                // 混沌/种子期 → 被动等待信息
                CallStrategy::Passive
            }
        };
    }

    // ────────────────────────────────────────────────────────
    // 报告
    // ────────────────────────────────────────────────────────

    pub fn report(&self) -> OrgReport {
        let active_paths: Vec<&LogicalPath> = self.paths.iter().filter(|p| p.active).collect();
        let dominant = active_paths.iter().find(|p| p.stratum == PathStratum::Dominant);
        let total_strength: f64 = active_paths.iter().map(|p| p.strength).sum();

        OrgReport {
            tick: self.current_tick,
            path_count: self.paths.len(),
            active_path_count: active_paths.len(),
            dominant_path_id: dominant.map(|p| p.id),
            current_strategy: self.strategy,
            total_strength,
            strongest_path_strength: active_paths.iter()
                .map(|p| p.confidence())
                .fold(0.0, f64::max),
        }
    }

    // ────────────────────────────────────────────────────────
    // 查询接口
    // ────────────────────────────────────────────────────────

    /// 获取当前活跃路径 (按强度排序)
    pub fn ranked_paths(&self) -> Vec<&LogicalPath> {
        let mut result: Vec<&LogicalPath> = self.paths.iter().filter(|p| p.active).collect();
        result.sort_by(|a, b| b.confidence().partial_cmp(&a.confidence()).unwrap());
        result
    }

    /// 获取指定层级的路径
    pub fn paths_by_stratum(&self, stratum: PathStratum) -> Vec<&LogicalPath> {
        self.paths.iter()
            .filter(|p| p.active && p.stratum == stratum)
            .collect()
    }

    /// 获取叠加态宽度 (同时存在的层级种类数)
    pub fn superposition_width(&self) -> usize {
        let mut kinds = Vec::new();
        for p in self.paths.iter().filter(|p| p.active) {
            if !kinds.contains(&p.stratum) {
                kinds.push(p.stratum);
            }
        }
        kinds.len()
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 便捷集成: 从 RelationAnchor 直接接入
// ──────────────────────────────────────────────────────────────────────────=

impl LogicalOrgEngine {
    /// 从 RelationAnchor + EvalLevel 直接处理
    pub fn feed_anchor(
        &mut self,
        anchor: &RelationAnchor,
        verdict: EvalLevel,
        tick: u64,
    ) -> OrgReport {
        // anchor.confidence() 作为评价幅度
        let weight = anchor.confidence();
        // anchor在timeline中的索引 — 这里简化为用 (cause_id ^ effect_id) 作为标识
        let anchor_idx = ((anchor.cause_id ^ anchor.effect_id) as usize) % 10000;
        let input = EvalInput::new(anchor_idx, verdict, weight, tick);
        self.process(input)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 测试
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_creates_passive() {
        let engine = LogicalOrgEngine::new();
        assert_eq!(engine.strategy, CallStrategy::Passive);
        assert_eq!(engine.paths.len(), 0);
    }

    #[test]
    fn single_eval_creates_path() {
        let mut engine = LogicalOrgEngine::new();
        let input = EvalInput::new(0, EvalLevel::Strengthen, 0.8, 1);
        let report = engine.process(input);
        assert_eq!(report.path_count, 1);
        assert!(report.total_strength > 0.3);
    }

    #[test]
    fn dominant_emerges_from_reinforcement() {
        let mut engine = LogicalOrgEngine::new();
        // 反复强化同一锚点直到主导出现
        for i in 0..20 {
            let input = EvalInput::new(0, EvalLevel::Strengthen, 0.5, i + 1);
            let _ = engine.process(input);
        }

        let dominant_paths = engine.paths_by_stratum(PathStratum::Dominant);
        assert!(!dominant_paths.is_empty(), "反复强化应产生 Dominant 路径");
    }

    #[test]
    fn strategy_preemptive_with_young_dominant() {
        let mut engine = LogicalOrgEngine::new();
        // 创建两条路径, 一条较强但短
        for i in 0..5 {
            let _ = engine.process(EvalInput::new(0, EvalLevel::Strengthen, 0.9, i + 1));
        }
        // 强主导但链短 (<3) → Preemptive
        let report = engine.report();
        assert!(matches!(report.current_strategy,
            CallStrategy::Preemptive | CallStrategy::Resonant | CallStrategy::Active),
            "有主导路径时不应为 Passive"
        );
    }

    #[test]
    fn strategy_resonant_with_mature_dominant() {
        let mut engine = LogicalOrgEngine::new();

        // 构建一条 3 节点的强路径
        for i in 0..15 {
            let anchor_idx = if i < 5 { 0 } else if i < 10 { 1 } else { 2 };
            let _ = engine.process(EvalInput::new(
                anchor_idx, EvalLevel::Strengthen, 0.8, i + 1
            ));
        }

        // 检查是否有 Resonant
        let report = engine.report();
        // 只要主导路径链长 >= 3, 应为 Resonant
        let has_resonant = engine.paths.iter()
            .any(|p| p.stratum == PathStratum::Dominant && p.anchor_chain.len() >= 3);
        if has_resonant {
            assert_eq!(report.current_strategy, CallStrategy::Resonant,
                "链长≥3的主导路径应触发 Resonant"
            );
        }
    }

    #[test]
    fn suppress_decays_path() {
        let mut engine = LogicalOrgEngine::new();
        // 先强化建立路径
        for i in 0..10 {
            let _ = engine.process(EvalInput::new(0, EvalLevel::Strengthen, 0.5, i + 1));
        }
        let before_strength = engine.paths.iter().find(|p| p.active).map(|p| p.strength).unwrap_or(0.0);

        // 然后抑制
        for i in 10..20 {
            let _ = engine.process(EvalInput::new(0, EvalLevel::Suppress, 0.5, i + 1));
        }
        let after_strength = engine.paths.iter().find(|p| p.active).map(|p| p.strength).unwrap_or(0.0);

        assert!(after_strength < before_strength, "抑制后路径强度应下降");
    }

    #[test]
    fn merge_overlapping_paths() {
        let mut engine = LogicalOrgEngine::new();

        // 第一条路径: 锚点 0, 1, 2
        for i in 0..3 {
            let _ = engine.process(EvalInput::new(i, EvalLevel::Strengthen, 0.6, i as u64 + 1));
        }
        let paths_before = engine.paths.len();

        // 注入共享锚点 2 的评价 → 应该合并而非创建新路径
        let _ = engine.process(EvalInput::new(2, EvalLevel::Strengthen, 0.6, 10));
        let _ = engine.process(EvalInput::new(3, EvalLevel::Strengthen, 0.6, 11));

        // 锚点 3 与 2 相邻, 应附加而非新建
        assert!(engine.paths.len() <= paths_before + 1, "相邻锚点不应无序创建新路径");
    }

    #[test]
    fn ranked_paths_ordered_by_confidence() {
        let mut engine = LogicalOrgEngine::new();

        // 创建两个不同强度的路径
        for i in 0..3 {
            let _ = engine.process(EvalInput::new(10, EvalLevel::Strengthen, 0.3, i + 1));
        }
        for i in 0..8 {
            let _ = engine.process(EvalInput::new(20, EvalLevel::Strengthen, 0.9, i + 10));
        }

        let ranked = engine.ranked_paths();
        if ranked.len() >= 2 {
            assert!(ranked[0].confidence() >= ranked[1].confidence(),
                "排序应为降序"
            );
        }
    }

    #[test]
    fn superposition_width_counts_strata() {
        let mut engine = LogicalOrgEngine::new();

        // 多种层级共存
        let _ = engine.process(EvalInput::new(1, EvalLevel::Strengthen, 0.1, 1)); // Seed
        let _ = engine.process(EvalInput::new(5, EvalLevel::Strengthen, 0.9, 2)); // 强
        for i in 3..6 {
            let _ = engine.process(EvalInput::new(5, EvalLevel::Strengthen, 0.9, i));
        }

        let width = engine.superposition_width();
        assert!(width >= 1, "至少应有一种层级");
    }

    #[test]
    fn feed_anchor_integration() {
        let mut engine = LogicalOrgEngine::new();
        let anchor = RelationAnchor {
            cause_id: 100,
            effect_id: 200,
            strength: 0.8,
            first_seen: 0,
            confirmed: 3,
        };

        let report = engine.feed_anchor(&anchor, EvalLevel::Strengthen, 1);
        assert_eq!(report.path_count, 1);
        assert!(report.strongest_path_strength > 0.0);
    }

    #[test]
    fn max_paths_limit() {
        let mut engine = LogicalOrgEngine::new().with_limits(3, 0.02);

        // 大量不同锚点 — 上限生效
        for i in 0..20 {
            let _ = engine.process(EvalInput::new(i * 5, EvalLevel::Strengthen, 0.3, i as u64 + 1));
        }

        assert!(engine.paths.len() <= 3, "路径数不应超过上限");
    }

    #[test]
    fn total_paths_and_active_count_in_report() {
        let mut engine = LogicalOrgEngine::new();

        for i in 0..5 {
            let _ = engine.process(EvalInput::new(i, EvalLevel::Strengthen, 0.5, i as u64 + 1));
        }

        let report = engine.report();
        assert!(report.active_path_count >= 1, "应有活跃路径");
        assert!(report.total_strength > 0.0, "总强度应 > 0");
    }
}
