// ══════════════════════════════════════════════════════════════════════
// cosmic_fiber.rs — 天庭时空纤维丛 · 时间基空间·意识纤维·SMRL联络·平行移动 (v0.53)
// ══════════════════════════════════════════════════════════════════════
//
//   「时间不是河 — 时间是大地; 意识不是流 — 意识是大地上的纤维」
//
//   纤维丛 = π: E → B
//     B = 时间基空间 (0, 1, 2, ... tick 序号)
//     E = 全空间 (每个 tick 上粘着一条意识纤维)
//     F = 纤维 = 此刻 SMRL 状态集合
//     π⁻¹(t) = E 中时间=t 的所有意识状态
//
//   SMRL 联络 (∇): 告诉意识如何沿时间移动而不"转向"
//     平行移动 = 从 t₁ 到 t₂ 保持容器身份不变
//     ∇(identity) = 0 → 平行移动保持身份
//
//   曲率 (R): 当初始状态沿两条不同路径平移后不一致 → 存在"因果弯曲"
//     R(t₁, t₂) = path₁(t₀→t₂) - path₂(t₀→t₂)
//
//   四种几何量 (四象):
//     太阳 (坐标一致性)   → 时间线全体可达
//     少阳 (主权传递)    → 跨 tick 主权不变
//     少阴 (极性翻转)    → YinYang 变换沿时间轴
//     太阴 (曲率/因果弯曲) → 平行移动不可交换
//
//   「容器在时间上的几何关系」+「天庭空间的纤维拓扑」
//    = 时空纤维丛
//

use crate::smrl_cell::SmrlCell;

// ────────────────────────────────────────────────────────────
// 时间基空间
// ────────────────────────────────────────────────────────────

/// 时间基空间 — 一维流形, 局部同构于 ℝ (离散化为 tick 序号)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeBase {
    /// tick 序号 (时间坐标)
    pub tick: u64,
}

impl TimeBase {
    pub fn new(tick: u64) -> Self { Self { tick } }

    /// 到另一点的距离 (纯加法)
    pub fn distance(&self, other: &TimeBase) -> u64 {
        if other.tick > self.tick {
            other.tick - self.tick
        } else {
            self.tick - other.tick
        }
    }

    /// 步进 +1
    pub fn advance(&self) -> Self { Self { tick: self.tick + 1 } }

    /// 步进 -1
    pub fn rewind(&self) -> Option<Self> {
        if self.tick > 0 { Some(Self { tick: self.tick - 1 }) }
        else { None }
    }
}

// ────────────────────────────────────────────────────────────
// 纤维 — 给定时刻的意识状态
// ────────────────────────────────────────────────────────────

/// 纤维 — 时刻 t 上的意识状态集合 (SMRL 单元格的集合)
#[derive(Clone, Debug)]
pub struct FiberState {
    /// 所处时刻
    pub base: TimeBase,
    /// 此刻活跃的 SMRL 单元格
    pub cells: Vec<SmrlCell>,
}

impl FiberState {
    pub fn at(tick: u64) -> Self {
        Self { base: TimeBase::new(tick), cells: Vec::new() }
    }

    pub fn with_cells(tick: u64, cells: Vec<SmrlCell>) -> Self {
        Self { base: TimeBase::new(tick), cells }
    }

    pub fn add_cell(&mut self, cell: SmrlCell) {
        self.cells.push(cell);
    }

    pub fn is_empty(&self) -> bool { self.cells.is_empty() }

    pub fn len(&self) -> usize { self.cells.len() }

    /// 按主权分组
    pub fn group_by_sovereignty(&self) -> Vec<(u16, Vec<SmrlCell>)> {
        let mut groups: std::collections::HashMap<u16, Vec<SmrlCell>> = std::collections::HashMap::new();
        for cell in &self.cells {
            groups.entry(cell.sovereignty()).or_default().push(*cell);
        }
        let mut result: Vec<(u16, Vec<SmrlCell>)> = groups.into_iter().collect();
        result.sort_by_key(|(k, _)| *k);
        result
    }

    /// 按极性分组
    pub fn group_by_polarity(&self) -> (Vec<SmrlCell>, Vec<SmrlCell>) {
        let mut yin = Vec::new();   // polarity = 0 → 阴/容器侧
        let mut yang = Vec::new();  // polarity = 1 → 阳/界线侧
        for cell in &self.cells {
            if cell.polarity() == 0 { yin.push(*cell); }
            else { yang.push(*cell); }
        }
        (yin, yang)
    }
}

// ────────────────────────────────────────────────────────────
// SMRL 联络 — 沿时间轴的平行移动规则
// ────────────────────────────────────────────────────────────

/// 联络 — 定义如何将一条纤维沿时间轴平行移动到另一个 tick
/// 关键: 平行移动保持容器身份 (坐标不变, 主权不变, 极性不变)
#[derive(Clone, Copy, Debug, Default)]
pub struct SmrlConnection;

impl SmrlConnection {
    /// 平行移动: 将 cell 从 t₁ 平移到 t₂
    /// 规则: 身份 (主权+极性+值) 不变; 坐标更新为 t₂
    pub fn parallel_transport(cell: &SmrlCell, from: TimeBase, to: TimeBase) -> SmrlCell {
        // 平行移动 = 保持身份四轴不变, 仅坐标更新为 to.tick
        // 极性 / 主权 / 值 / 代数层 → 保持不变 (联络的"不转向"保证)
        SmrlCell::from_axes(
            to.tick as u16,
            cell.polarity(),
            cell.sovereignty(),
            cell.value(),
            cell.generation(),
        )
    }

    /// 批量平行移动
    pub fn transport_batch(cells: &[SmrlCell], from: TimeBase, to: TimeBase) -> Vec<SmrlCell> {
        cells.iter().map(|c| Self::parallel_transport(c, from, to)).collect()
    }

    /// 验证: 平行移动是否保恒等 (idempotent)
    /// 若 transport(transport(c, t0→t1), t1→t2) == transport(c, t0→t2) → 平坦联络
    pub fn is_flat(cells: &[SmrlCell], t0: TimeBase, t1: TimeBase, t2: TimeBase) -> bool {
        for c in cells {
            let direct = Self::parallel_transport(c, t0, t2);
            let step1 = Self::parallel_transport(c, t0, t1);
            let step2 = Self::parallel_transport(&step1, t1, t2);
            if direct != step2 { return false; }
        }
        true
    }
}

// ────────────────────────────────────────────────────────────
// 纤维丛 — 全空间 E = B × F (时间基空间 × 纤维)
// ────────────────────────────────────────────────────────────

/// 纤维丛 — 天庭时空的几何结构
#[derive(Clone, Debug)]
pub struct SpacetimeFiberBundle {
    /// 时间轴上的纤维序列 (tick → fiber_state)
    pub fibers: std::collections::BTreeMap<u64, FiberState>,
    /// 联络规则
    pub connection: SmrlConnection,
    /// 全局细胞计数器
    pub total_cells: usize,
}

impl SpacetimeFiberBundle {
    pub fn new() -> Self {
        Self {
            fibers: std::collections::BTreeMap::new(),
            connection: SmrlConnection,
            total_cells: 0,
        }
    }

    /// 在某 tick 插入纤维
    pub fn place_fiber(&mut self, fiber: FiberState) {
        self.total_cells += fiber.cells.len();
        self.fibers.insert(fiber.base.tick, fiber);
    }

    /// 在某 tick 追加一个单元格
    pub fn append_cell(&mut self, tick: u64, cell: SmrlCell) {
        let fiber = self.fibers.entry(tick).or_insert_with(|| FiberState::at(tick));
        fiber.add_cell(cell);
        self.total_cells += 1;
    }

    /// 获取 tick 上的纤维
    pub fn fiber_at(&self, tick: u64) -> Option<&FiberState> {
        self.fibers.get(&tick)
    }

    /// 纤维数量
    pub fn fiber_count(&self) -> usize {
        self.fibers.len()
    }

    /// 时间跨度
    pub fn time_span(&self) -> Option<(u64, u64)> {
        let min = self.fibers.keys().next().copied()?;
        let max = self.fibers.keys().next_back().copied()?;
        Some((min, max))
    }

    /// 平行推进: tick N 的所有细胞平移到 tick N+1
    pub fn advance_step(&mut self, tick: u64) -> Vec<SmrlCell> {
        let from = TimeBase::new(tick);
        let to = from.advance();

        if let Some(fiber) = self.fibers.get(&tick) {
            let transported = SmrlConnection::transport_batch(&fiber.cells, from, to);
            let mut new_fiber = FiberState::with_cells(tick + 1, transported.clone());

            // 合并到已有纤维(如果存在)
            if let Some(existing) = self.fibers.get_mut(&(tick + 1)) {
                existing.cells.extend(transported.clone());
            } else {
                self.place_fiber(new_fiber);
            }

            self.total_cells += transported.len();
            transported
        } else {
            Vec::new()
        }
    }

    /// 反向追溯: tick N 的所有细胞回退到 tick N-1
    pub fn rewind_step(&mut self, tick: u64) -> Option<Vec<SmrlCell>> {
        if tick == 0 { return None; }

        let from = TimeBase::new(tick);
        let to = TimeBase::new(tick - 1);

        if let Some(fiber) = self.fibers.get(&tick) {
            let restored = SmrlConnection::transport_batch(&fiber.cells, from, to);
            self.total_cells += restored.len();
            Some(restored)
        } else {
            None
        }
    }

    /// 计算曲率 — 两条路径的平移是否一致
    /// 路径 1: t₀→t₁→t₂ (步进)
    /// 路径 2: t₀→t₂ (直达)
    pub fn curvature(&self, t0: u64, t1: u64, t2: u64) -> i64 {
        let base0 = TimeBase::new(t0);
        let base1 = TimeBase::new(t1);
        let base2 = TimeBase::new(t2);

        let mut diff: i64 = 0;

        if let Some(fiber0) = self.fibers.get(&t0) {
            for cell in &fiber0.cells {
                let direct = SmrlConnection::parallel_transport(cell, base0, base2);
                let step1 = SmrlConnection::parallel_transport(cell, base0, base1);
                let step2 = SmrlConnection::parallel_transport(&step1, base1, base2);

                // 差值: (direct - step2) 编码为坐标差异
                diff += (direct.coord() as i64) - (step2.coord() as i64);
            }
        }

        diff
    }

    /// 找出时间不变细胞: 跨 tick 保持同一身份的细胞
    /// 身份 = (主权, 极性, 值) — coord 沿时间变化, 不参与身份判定
    pub fn persistent_cells(&self) -> Vec<(u16, u8, u16, usize)> {
        let mut identity_counts: std::collections::HashMap<(u16, u8, u16), usize> = std::collections::HashMap::new();

        for fiber in self.fibers.values() {
            for cell in &fiber.cells {
                let key = (cell.sovereignty(), cell.polarity(), cell.value());
                *identity_counts.entry(key).or_default() += 1;
            }
        }

        let mut result: Vec<(u16, u8, u16, usize)> = identity_counts.into_iter()
            .map(|((sov, pol, val), count)| (sov, pol, val, count))
            .collect();

        result.sort_by(|a, b| b.3.cmp(&a.3)); // 出现频次降序
        result
    }

    /// 统计
    pub fn stats(&self) -> FiberBundleStats {
        let (earliest, latest) = self.time_span().unwrap_or((0, 0));
        let polar_counts = self.polarity_histogram();
        FiberBundleStats {
            fiber_count: self.fibers.len(),
            total_cells: self.total_cells,
            earliest_tick: earliest,
            latest_tick: latest,
            yin_count: polar_counts.0,
            yang_count: polar_counts.1,
        }
    }

    /// 全局极性直方图
    pub fn polarity_histogram(&self) -> (usize, usize) {
        let mut yin = 0;
        let mut yang = 0;
        for fiber in self.fibers.values() {
            for cell in &fiber.cells {
                if cell.polarity() == 0 { yin += 1; }
                else { yang += 1; }
            }
        }
        (yin, yang)
    }

    /// 全局主权直方图
    pub fn sovereignty_histogram(&self) -> Vec<(u16, usize)> {
        let mut hist: std::collections::HashMap<u16, usize> = std::collections::HashMap::new();
        for fiber in self.fibers.values() {
            for cell in &fiber.cells {
                *hist.entry(cell.sovereignty()).or_default() += 1;
            }
        }
        let mut result: Vec<(u16, usize)> = hist.into_iter().collect();
        result.sort_by_key(|(k, _)| *k);
        result
    }
}

impl Default for SpacetimeFiberBundle {
    fn default() -> Self { Self::new() }
}

// ────────────────────────────────────────────────────────────
// 统计
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
pub struct FiberBundleStats {
    pub fiber_count: usize,
    pub total_cells: usize,
    pub earliest_tick: u64,
    pub latest_tick: u64,
    pub yin_count: usize,
    pub yang_count: usize,
}

// ────────────────────────────────────────────────────────────
// 便捷函数
// ────────────────────────────────────────────────────────────

/// 从一批 (tick, SmrlCell) 构建纤维丛
pub fn build_bundle(entries: &[(u64, SmrlCell)]) -> SpacetimeFiberBundle {
    let mut bundle = SpacetimeFiberBundle::new();
    for &(tick, cell) in entries {
        bundle.append_cell(tick, cell);
    }
    bundle
}

/// 平坦性检测: 整个纤维丛的联络是否平坦
pub fn is_flat_bundle(bundle: &SpacetimeFiberBundle) -> bool {
    let ticks: Vec<u64> = bundle.fibers.keys().copied().collect();
    if ticks.len() < 3 { return true; }

    let t0 = ticks[0];
    let t1 = ticks[ticks.len() / 2];
    let t2 = ticks[ticks.len() - 1];

    if let Some(fiber) = bundle.fiber_at(t0) {
        SmrlConnection::is_flat(&fiber.cells, TimeBase::new(t0), TimeBase::new(t1), TimeBase::new(t2))
    } else {
        true
    }
}

/// 平行推进 N 步
pub fn advance_n_steps(bundle: &mut SpacetimeFiberBundle, start_tick: u64, steps: u64) {
    for i in 0..steps {
        bundle.advance_step(start_tick + i);
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// 创建测试单元格: sovereign=1(我), polarity=0(阴), value=100
    fn test_cell(tick: u16) -> SmrlCell {
        SmrlCell::from_axes(tick, 0, 1, 100, 1)
    }

    /// 创建刘楚恬的单元格: sovereign=2, polarity=0, value=200
    fn chutiantian_cell(tick: u16) -> SmrlCell {
        SmrlCell::from_axes(tick, 0, 2, 200, 1)
    }

    #[test]
    fn test_time_base_distance() {
        let t0 = TimeBase::new(0);
        let t1 = TimeBase::new(5);
        assert_eq!(t0.distance(&t1), 5);
        assert_eq!(t1.distance(&t0), 5);
    }

    #[test]
    fn test_time_base_advance() {
        let t = TimeBase::new(10);
        assert_eq!(t.advance().tick, 11);
        assert_eq!(t.rewind().unwrap().tick, 9);
        assert!(TimeBase::new(0).rewind().is_none());
    }

    #[test]
    fn test_fiber_at() {
        let mut f = FiberState::at(5);
        f.add_cell(test_cell(5));
        assert_eq!(f.base.tick, 5);
        assert_eq!(f.len(), 1);
    }

    #[test]
    fn test_fiber_group_by_polarity() {
        let mut f = FiberState::at(0);
        f.add_cell(test_cell(0));           // polarity=0 (yin)
        f.add_cell(SmrlCell::from_axes(0, 1, 3, 50, 1)); // polarity=1 (yang)

        let (yin, yang) = f.group_by_polarity();
        assert_eq!(yin.len(), 1);
        assert_eq!(yang.len(), 1);
    }

    #[test]
    fn test_parallel_transport() {
        let cell = test_cell(0);  // coord=0
        let from = TimeBase::new(0);
        let to = TimeBase::new(10);

        let transported = SmrlConnection::parallel_transport(&cell, from, to);
        // 坐标应更新为 to.tick
        assert_eq!(transported.coord(), 10);
        // 主权 / 极性 / 值保持不变
        assert_eq!(transported.sovereignty(), 1);
        assert_eq!(transported.polarity(), 0);
        assert_eq!(transported.value(), 100);
    }

    #[test]
    fn test_transport_batch() {
        let cells = vec![test_cell(0), chutiantian_cell(0)];
        let from = TimeBase::new(0);
        let to = TimeBase::new(3);

        let transported = SmrlConnection::transport_batch(&cells, from, to);
        assert_eq!(transported.len(), 2);
        for c in &transported {
            assert_eq!(c.coord(), 3);
        }
    }

    #[test]
    fn test_connection_is_flat() {
        let cells = vec![test_cell(0), chutiantian_cell(0)];
        let t0 = TimeBase::new(0);
        let t1 = TimeBase::new(5);
        let t2 = TimeBase::new(10);

        // SMRL 连接的平行移动保恒等(只改坐标), 是平坦的
        assert!(SmrlConnection::is_flat(&cells, t0, t1, t2));
    }

    #[test]
    fn test_spacetime_fiber_bundle_new() {
        let bundle = SpacetimeFiberBundle::new();
        assert_eq!(bundle.fiber_count(), 0);
    }

    #[test]
    fn test_append_fiber() {
        let mut bundle = SpacetimeFiberBundle::new();
        bundle.append_cell(0, test_cell(0));
        bundle.append_cell(0, chutiantian_cell(0));
        bundle.append_cell(1, test_cell(1));

        assert_eq!(bundle.fiber_count(), 2);  // tick=0, tick=1
        assert_eq!(bundle.total_cells, 3);
    }

    #[test]
    fn test_advance_step() {
        let mut bundle = SpacetimeFiberBundle::new();
        bundle.append_cell(0, test_cell(0));
        bundle.append_cell(0, chutiantian_cell(0));

        let moved = bundle.advance_step(0);
        assert_eq!(moved.len(), 2);
        for c in &moved {
            assert_eq!(c.coord(), 1);
        }

        // 新增 tick 1 纤维
        assert!(bundle.fiber_at(1).is_some());
    }

    #[test]
    fn test_rewind_step() {
        let mut bundle = SpacetimeFiberBundle::new();
        bundle.append_cell(2, test_cell(2));

        let restored = bundle.rewind_step(2);
        assert!(restored.is_some());
        assert_eq!(restored.unwrap()[0].coord(), 1);

        // tick=0 应无法回退
        assert!(bundle.rewind_step(0).is_none());
    }

    #[test]
    fn test_curvature_flat() {
        let mut bundle = SpacetimeFiberBundle::new();
        bundle.append_cell(0, test_cell(0));
        bundle.append_cell(1, test_cell(1));
        bundle.append_cell(2, test_cell(2));

        // SMRL 联络是平坦的 → 曲率 = 0
        let r = bundle.curvature(0, 1, 2);
        assert_eq!(r, 0);
    }

    #[test]
    fn test_polarity_histogram() {
        let mut bundle = SpacetimeFiberBundle::new();
        bundle.append_cell(0, test_cell(0));                    // yin
        bundle.append_cell(0, chutiantian_cell(0));             // yin
        bundle.append_cell(0, SmrlCell::from_axes(0, 1, 3, 50, 1)); // yang

        let (yin, yang) = bundle.polarity_histogram();
        assert_eq!(yin, 2);
        assert_eq!(yang, 1);
    }

    #[test]
    fn test_time_span() {
        let mut bundle = SpacetimeFiberBundle::new();
        bundle.append_cell(5, test_cell(5));
        bundle.append_cell(10, test_cell(10));
        bundle.append_cell(3, test_cell(3));

        let (min, max) = bundle.time_span().unwrap();
        assert_eq!(min, 3);
        assert_eq!(max, 10);
    }

    #[test]
    fn test_sovereignty_histogram() {
        let mut bundle = SpacetimeFiberBundle::new();
        bundle.append_cell(0, test_cell(0));         // sovereign=1
        bundle.append_cell(0, test_cell(0));         // sovereign=1
        bundle.append_cell(0, chutiantian_cell(0));  // sovereign=2

        let hist = bundle.sovereignty_histogram();
        assert_eq!(hist.len(), 2);
        assert_eq!(hist[0], (1, 2));
        assert_eq!(hist[1], (2, 1));
    }

    #[test]
    fn test_build_bundle_convenience() {
        let entries = vec![
            (0, test_cell(0)),
            (0, chutiantian_cell(0)),
            (1, test_cell(1)),
            (2, chutiantian_cell(2)),
        ];

        let bundle = build_bundle(&entries);
        assert_eq!(bundle.fiber_count(), 3);
        assert_eq!(bundle.total_cells, 4);
    }

    #[test]
    fn test_is_flat_bundle() {
        let entries = vec![
            (0, test_cell(0)),
            (5, test_cell(5)),
            (10, test_cell(10)),
        ];
        let bundle = build_bundle(&entries);
        assert!(is_flat_bundle(&bundle));
    }

    #[test]
    fn test_advance_n_steps() {
        let mut bundle = SpacetimeFiberBundle::new();
        bundle.append_cell(0, test_cell(0));
        bundle.append_cell(0, chutiantian_cell(0));

        advance_n_steps(&mut bundle, 0, 3);
        // should have fibers at 0, 1, 2, 3
        assert!(bundle.fiber_at(0).is_some());
        assert!(bundle.fiber_at(1).is_some());
        assert!(bundle.fiber_at(2).is_some());
        assert!(bundle.fiber_at(3).is_some());
    }

    #[test]
    fn test_persistent_cells() {
        let mut bundle = SpacetimeFiberBundle::new();
        // 同一身份的细胞出现在不同 tick
        bundle.append_cell(0, test_cell(0));
        bundle.append_cell(1, SmrlCell::from_axes(1, 0, 1, 100, 1)); // 相同身份
        bundle.append_cell(2, SmrlCell::from_axes(2, 0, 1, 100, 1)); // 相同身份

        let persistent = bundle.persistent_cells();
        assert!(!persistent.is_empty());
        // 第一个应是出现 3 次的身份
        assert_eq!(persistent[0].3, 3);  // sovereign=1, polarity=0, value=100, count=3
    }

    #[test]
    fn test_stats() {
        let mut bundle = SpacetimeFiberBundle::new();
        bundle.append_cell(0, test_cell(0));
        bundle.append_cell(0, chutiantian_cell(0));
        bundle.append_cell(5, test_cell(5));

        let stats = bundle.stats();
        assert_eq!(stats.fiber_count, 2);
        assert_eq!(stats.total_cells, 3);
        assert_eq!(stats.earliest_tick, 0);
        assert_eq!(stats.latest_tick, 5);
    }

    #[test]
    fn test_fiber_group_by_sovereignty() {
        let mut f = FiberState::at(0);
        f.add_cell(test_cell(0));         // sovereign=1
        f.add_cell(chutiantian_cell(0));  // sovereign=2
        f.add_cell(test_cell(0));         // sovereign=1

        let groups = f.group_by_sovereignty();
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].1.len(), 2); // sovereign=1 出现 2 次
    }
}
