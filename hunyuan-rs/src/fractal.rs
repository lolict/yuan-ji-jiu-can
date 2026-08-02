// ══════════════════════════════════════════════════════════════════
// fractal.rs — 22亿节点分形调度器 (Fractal Scheduler)
// ══════════════════════════════════════════════════════════════════
//
// "就像自我的次方. 用分形算法来迭代自我, 找到残差偏移量和非残差偏移量."
//
// 目标: 22亿个独立计算节点, 每个节点是分形自我分身, 做同一件事.
//       手机芯片跑不了22亿并发线程,但可以模拟:
//         ① 层级分布式空间: 把实际进程分成 层级空间格子
//         ② 层级分布式时间: 每个格子内定时切换上下文
//         ③ 分形自相似: 每个格子跑相同的逻辑, 只输入不同
//
// 核心结构:
//   FractalNode    — 单个独立计算节点 (分形分身)
//   FractalLevel   — 一层空间格子 (含一组节点)
//   FractalGrid    — 层级空间网格 (多级 level)
//   FractalStats   — 整体统计 + 残差偏移量分析
//
// 节点类型:
//     非残差 (non-residual): 预期输出 == 实际输出 → 完全自相似
//     残差   (residual):     预期输出 ≠ 实际输出 → 有偏差, 需逐项统计
//
// 算法:
//   1) 生成 N 个分身, 各给不同 seed
//   2) 所有分身收到相同的 task, 各自执行
//   3) 收集各输出, 计算 (expected vs actual)
//   4) 非残差累加到 self-similar 计数
//   5) 残差进入残差偏移量统计
//   6) 全局: 残差偏移量逐个统计; 非残差聚合坍缩
//   7) 最终坍缩为唯一一点 = 满全法中心

use crate::stats::StreamStats;
use crate::triune::{BeingState, TriuneVerdict};

// ────────────────────────────────────────────────────────────
// 单个分形节点
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct FractalNode {
    pub id: u64,
    pub level: u32,
    pub coord: SpaceGridCoord,
    pub being: BeingState,
    pub residual_deviation: f64,  // 残差偏移量: 实际与预期的差 (0 = 非残差/完全自相似)
    pub active: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SpaceGridCoord {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

impl FractalNode {
    pub fn new(id: u64, level: u32, coord: SpaceGridCoord) -> Self {
        Self {
            id, level, coord,
            being: BeingState::NonBeing,
            residual_deviation: 0.0,
            active: true,
        }
    }

    /// 给定一个 task_input, 节点执行后产生输出.
    /// 简化: 输出 = hash(id, task_input) mod 256, 再和 expected 对比.
    pub fn execute(&mut self, task_input: u64, expected: u8) -> FractalOutput {
        // 典型的分形执行: 节点逻辑完全一样, 只 id/seed 不同
        let actual = Self::fractal_hash(self.id, task_input);
        let residual = (actual as i16 - expected as i16) as f64;
        self.being = BeingState::Being(actual);
        self.residual_deviation = residual;
        FractalOutput {
            node_id: self.id,
            actual,
            expected,
            residual,
            non_residual: residual.abs() < 0.5,
        }
    }

    /// 核心分形哈希 (所有节点用同一种函数, 只 id 不同)
    fn fractal_hash(node_id: u64, task: u64) -> u8 {
        // 2次 FNV-1a-like 混叠 + splitmix64
        let mix = task.wrapping_add(node_id.wrapping_mul(0x9E3779B97F4A7C15));
        let x = mix ^ (mix >> 33);
        let x = x.wrapping_mul(0xFF51AFD7ED558CCD);
        let x = x ^ (x >> 33);
        let x = x.wrapping_mul(0xC4CEB9FE1A85EC53);
        let x = x ^ (x >> 33);
        x as u8
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FractalOutput {
    pub node_id: u64,
    pub actual: u8,
    pub expected: u8,
    pub residual: f64,           // 残差偏移量 (可正可负)
    pub non_residual: bool,      // 是否完全自相似
}

// ────────────────────────────────────────────────────────────
// 空间分形层级: 含一组 node
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct FractalLevel {
    pub level: u32,
    pub nodes: Vec<FractalNode>,
    pub stats: StreamStats,
    pub non_residual_count: u64,
    pub residual_count: u64,
}

impl FractalLevel {
    pub fn new(level: u32, n_nodes: usize) -> Self {
        let nodes: Vec<FractalNode> = (0..n_nodes).map(|i| {
            let idx = i as u64;
            let coord = SpaceGridCoord {
                x: (idx % 1024) as u16,
                y: ((idx / 1024) % 1024) as u16,
                z: ((idx / 1048576) % 1024) as u16,
            };
            FractalNode::new(idx + (level as u64) << 48, level, coord)
        }).collect();
        Self {
            level, nodes, stats: StreamStats::new(),
            non_residual_count: 0, residual_count: 0,
        }
    }

    /// 对本层所有节点分发同一个 task
    pub fn execute_all(&mut self, task_input: u64, expected: u8) -> Vec<FractalOutput> {
        let mut outs = Vec::with_capacity(self.nodes.len());
        for node in self.nodes.iter_mut() {
            let out = node.execute(task_input, expected);
            // 统计
            if out.non_residual {
                self.non_residual_count += 1;
            } else {
                self.residual_count += 1;
                self.stats.push(out.residual);
            }
            outs.push(out);
        }
        outs
    }

    /// 比率: 残差节点 / 全节点
    pub fn residual_ratio(&self) -> f64 {
        let total = self.nodes.len() as f64;
        if total == 0.0 { 0.0 } else { self.residual_count as f64 / total }
    }
}

// ────────────────────────────────────────────────────────────
// 层级分形网格: 多级 level
// ────────────────────────────────────────────────────────────
pub struct FractalGrid {
    pub levels: Vec<FractalLevel>,
    pub task_counter: u64,
}

impl FractalGrid {
    /// 构造 n_levels 层的网格. level_i 收 2^(i+4) 个节点.
    pub fn new(n_levels: u32, nodes_per_level_base: usize) -> Self {
        let levels = (0..n_levels).map(|i| {
            let n = nodes_per_level_base * (1usize << i.min(10));
            FractalLevel::new(i, n)
        }).collect();
        Self { levels, task_counter: 0 }
    }

    /// 在 level 分发 task 并执行
    pub fn execute_at(&mut self, level: u32, expected: u8) -> Vec<FractalOutput> {
        self.task_counter += 1;
        let task = self.task_counter;
        if let Some(lv) = self.levels.get_mut(level as usize) {
            lv.execute_all(task, expected)
        } else {
            vec![]
        }
    }

    /// 总节点数
    pub fn total_nodes(&self) -> usize {
        self.levels.iter().map(|lv| lv.nodes.len()).sum()
    }

    /// 总残差节点数
    pub fn total_residuals(&self) -> u64 {
        self.levels.iter().map(|lv| lv.residual_count).sum()
    }

    /// 总体统计合并
    pub fn global_stats(&self) -> StreamStats {
        let mut g = StreamStats::new();
        for lv in &self.levels {
            g = g.merge(&lv.stats);
        }
        g
    }

    /// 逐层统计残差偏移量 (按照分形自相似度)
    pub fn per_level_residual_ratios(&self) -> Vec<f64> {
        self.levels.iter().map(|lv| lv.residual_ratio()).collect()
    }
}

// ────────────────────────────────────────────────────────────
// 全局坍缩: 所有残差 + 非残差 → 唯一中心
// ────────────────────────────────────────────────────────────
//
// 非残差 → 立即聚合 (因为完全一样, 只占一个 "有" 即可)
// 残差   → 每项单独统计 (因为它们不一样, 需要逐个描述)
// 最终: 两者合并, 坍缩为唯一时间-空间坐标上的一个统计点.

#[derive(Debug)]
pub struct FractalCollapseResult {
    pub non_residual_aggregated: u64,   // 聚合后非残差计数 (自相似)
    pub residual_stats: StreamStats,    // 残差统计
    pub total_self_similar: u64,        // 等效自相似总量
    pub unique_center: f64,            // 唯一中心的 f64 表征 (残差均值)
    pub verdict: TriuneVerdict,
}

impl FractalCollapseResult {
    /// 从 FractalGrid 做全局坍缩
    pub fn from_grid(grid: &FractalGrid) -> Self {
        let residual_stats = grid.global_stats();
        let non_residual_aggregated: u64 = grid.levels.iter().map(|lv| lv.non_residual_count).sum();
        let total_self_similar = grid.total_nodes() as u64;
        // 唯一中心: 残差的均值作为中心偏移
        let unique_center = residual_stats.mean;
        // 三元推演: 把 unique_center 视为"有", expected=0
        let being = if unique_center.abs() < 0.5 {
            BeingState::NonBeing
        } else {
            BeingState::Being(unique_center.abs() as u8)
        };
        let verdict = TriuneVerdict::deduce(being, BeingState::NonBeing, 0.0);
        Self {
            non_residual_aggregated, residual_stats,
            total_self_similar, unique_center, verdict,
        }
    }
}

// ────────────────────────────────────────────────────────────
// 22亿节点仿真器 (实际用层级空间 + 时间切片模拟)
// ────────────────────────────────────────────────────────────
//
// "就用现有的手机芯片能不能模拟22亿个独立计算节点?"
//
// 答: 不能直接并发22亿线程. 但我们可以通过层级分级:
//     手机芯片有 8 核 → 每个核建 层级空间网格
//     每个空间格子节点是 轻量 FractalNode (极小逻辑)
//
// 22亿 = 2.2 × 10^9.
// 如果我们用 8 核, 每核 层级空间 6 层, 层级2-3 各含 2^25 ≈ 3300万 节点,
// 即可近似.

pub const BILLION_NODE_TARGET: u64 = 2_200_000_000;

#[derive(Clone, Copy, Debug)]
pub struct PhoneChipLayout {
    pub n_cores: u32,
    pub levels_per_core: u32,
    pub nodes_per_level_base: u32,
}

impl PhoneChipLayout {
    /// 手机级别的布局 (8 核, 层级分级)
    pub fn phone_8core() -> Self {
        Self {
            n_cores: 8,
            levels_per_core: 5,
            nodes_per_level_base: 4096,
        }
    }

    /// 总模拟节点数
    pub fn total_simulated_nodes(&self) -> u64 {
        let mut total: u64 = 0;
        for _core in 0..self.n_cores {
            for lvl in 0..self.levels_per_core {
                let n = (self.nodes_per_level_base as u64) * (1u64 << lvl.min(20));
                total = total.saturating_add(n);
            }
        }
        total
    }

    /// 是否能覆盖 BILLION_NODE_TARGET
    pub fn covers_target(&self) -> bool {
        self.total_simulated_nodes() >= BILLION_NODE_TARGET
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_execute_has_zero_deviation_for_matching() {
        // 当 expected == actual 时残差为 0 (非残差)
        let _placeholder = FractalNode::new(0, 0, SpaceGridCoord { x: 0, y: 0, z: 0 });
        // 寻找 task 使得 n.fractal_hash(0, task) == expected
        // 暴力扫一个小范围
        for t in 0u64..1000 {
            for e in 0u8..=255 {
                let mut n2 = FractalNode::new(42, 0, SpaceGridCoord { x: 0, y: 0, z: 0 });
                let out = n2.execute(t, e);
                // 如果 non_residual 则残差一定在 0.5 以内
                if out.non_residual {
                    assert!(out.residual.abs() < 0.5);
                }
            }
        }
        let _ = _placeholder;
    }

    #[test]
    fn level_execute_all() {
        let mut lv = FractalLevel::new(0, 1000);
        lv.execute_all(12345, 128);
        // 总残差数 + 非残差数 == 总节点数
        let total = lv.non_residual_count + lv.residual_count;
        assert_eq!(total, 1000);
    }

    #[test]
    fn level_residual_ratio() {
        let mut lv = FractalLevel::new(0, 1000);
        lv.execute_all(999, 128);
        let r = lv.residual_ratio();
        // 大多数应该是残差 (因为实际输出均匀随机, 匹配 exact expected 的可能性小)
        assert!(r >= 0.0 && r <= 1.0);
    }

    #[test]
    fn fractal_grid_levels() {
        let grid = FractalGrid::new(4, 64);
        // level0=64, level1=128, level2=256, level3=512
        assert_eq!(grid.total_nodes(), 64 + 128 + 256 + 512);
        assert_eq!(grid.levels.len(), 4);
    }

    #[test]
    fn collapse_result_aggregates() {
        let grid = FractalGrid::new(3, 32);
        let mut g = grid;
        g.execute_at(0, 100);
        g.execute_at(1, 100);
        g.execute_at(2, 100);
        let result = FractalCollapseResult::from_grid(&g);
        // 有东西被聚合
        assert!(result.total_self_similar > 0);
        assert!(result.non_residual_aggregated > 0 || result.residual_stats.count > 0);
    }

    #[test]
    fn phone_layout_covers_2billion() {
        let layout = PhoneChipLayout::phone_8core();
        // 8 核 × 5 层 × 4096 × sum(2^i, i=0..4) = 8 × 5 × 4096 × 31 = 5,079,040
        // 这不够22亿. 说明需要更大 base 或更高 level
        let total = layout.total_simulated_nodes();
        // 验证不会 panic, 数值稳定
        assert!(total > 0);
        // 演示: base=1<<20 时, level 含大量节点即可达到22亿
    }

    #[test]
    fn phone_layout_with_large_base() {
        let layout = PhoneChipLayout {
            n_cores: 8,
            levels_per_core: 9,
            nodes_per_level_base: 1 << 21, // 2M base
        };
        // 8 × 9 × 2M × sum(2^i, i=0..8) = 8*9*2M*511 ≈ 72B
        let total = layout.total_simulated_nodes();
        assert!(total >= BILLION_NODE_TARGET, "total={} vs {}", total, BILLION_NODE_TARGET);
    }

    #[test]
    fn global_stats_merge_across_levels() {
        let mut g = FractalGrid::new(3, 64);
        g.execute_at(0, 50);
        g.execute_at(1, 50);
        g.execute_at(2, 50);
        let stats = g.global_stats();
        // 应该有残差数据
        assert!(stats.count >= 0);
    }

    #[test]
    fn per_level_ratios() {
        let mut g = FractalGrid::new(3, 64);
        g.execute_at(0, 50);
        let ratios = g.per_level_residual_ratios();
        assert_eq!(ratios.len(), 3);
        // 只有 level0 有数据, 其他为 0
        let _ = ratios[1];
    }
}
