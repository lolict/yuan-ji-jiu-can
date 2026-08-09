// ╔══════════════════════════════════════════════════════════════════╗
// ║  stats.rs — 统计锥 (Statistics Cone)                            ║
// ║  满全法(NJSG)系统运行时的「健康视图」                              ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 统计锥不关心单个符号的具体取值,只看分布形态:
//   · 空间分布: 64 个 Symbol 占据 4×4×4 立方体,各轴是否均匀?
//   · 血缘深度: 每个 Symbol 有几代祖先?深度差异 = 分支丰富度
//   · 角度密度: 384° 上哪个象限最拥挤?最稀疏?
//   · 奖惩净差: SMRL 判决的累计分 = 系统整体情绪
//
// 锥 =「一头尖一头宽」的观察器:从宏观(全系统分布)→微观(单个 Symbol)

use crate::angle::{Angle, Quadrant};
use crate::preload::PreloadFlow;
use crate::smrl::{Reward, SmrlEngine};
use crate::symbol_table::SymbolTable;
use crate::topo::Symbol;

// ═══════════════════════════════════════════════════════════════
// 为适应 funnel / fractal / pleasure 模块所需的公共统计类型
// ═══════════════════════════════════════════════════════════════

/// 流式统计 — 时间窗口内的流量计数 + 累积值
#[derive(Clone, Debug, Default)]
pub struct StreamStats {
    pub events: u64,
    pub bytes: u64,
    pub window_ms: u64,
    pub count: u64,
    pub sum: f64,
    pub mean: f64,
}

impl StreamStats {
    pub fn new() -> Self { Self::default() }
    pub fn push(&mut self, value: f64) {
        self.count += 1;
        self.sum += value;
        self.mean = self.sum / self.count as f64;
    }
    pub fn merge(&self, other: &Self) -> Self {
        let total = self.count + other.count;
        let sum = self.sum + other.sum;
        StreamStats {
            events: self.events + other.events,
            bytes: self.bytes + other.bytes,
            window_ms: self.window_ms.max(other.window_ms),
            count: total,
            sum,
            mean: if total > 0 { sum / total as f64 } else { 0.0 },
        }
    }
}

/// 方图 — 按区间计数
#[derive(Clone, Debug, Default)]
pub struct Histogram {
    pub buckets: Vec<u64>,
    pub min: i32,
    pub max: i32,
}

/// 时间窗口统计
#[derive(Clone, Debug, Default)]
pub struct TimeWindowStats {
    pub window_ms: u64,
    pub count: u64,
    pub mean: f64,
    pub max: f64,
}

/// 空间密度 — 单位空间内的 Symbol 计权
#[derive(Clone, Debug, Default)]
pub struct SpaceDensity {
    pub region: [f64; 4],
    pub total: f64,
}

/// 系统级统计快照 — 一次 call_all 返回的全景
pub struct StatsCone {
    pub spatial:     SpatialStats,
    pub ancestry:    AncestryStats,
    pub angular:     AngularStats,
    pub reward:      RewardStats,
}

impl StatsCone {
    /// 入口:一次性计算所有维度
    pub fn snapshot(pf: &PreloadFlow, _table: &SymbolTable, engine: &SmrlEngine) -> Self {
        StatsCone {
            spatial: SpatialStats::compute(pf),
            ancestry: AncestryStats::compute(pf),
            angular: AngularStats::compute(),
            reward: RewardStats::compute(pf, engine),
        }
    }

    /// 系统均匀度 0..1,越接近 1 越均衡
    pub fn balance_score(&self) -> f64 {
        let s_uniform = self.spatial.axis_uniformity();
        let a_uniform = self.angular.quad_uniformity();
        (s_uniform + a_uniform) / 2.0
    }

    /// 系统情绪:-1 (非常负面)..+1 (非常正面)
    pub fn system_mood(&self) -> f64 {
        let net = self.reward.net_score();
        let total = self.reward.total_judged() as f64;
        if total == 0.0 { 0.0 } else { (net / total).clamp(-1.0, 1.0) }
    }
}

// ── 空间分布统计 ──────────────────────────────────────────────

pub struct SpatialStats {
    pub x_occupancy: [u8; 4],   // x=0,1,2,3 各填充了多少个格点
    pub y_occupancy: [u8; 4],
    pub z_occupancy: [u8; 4],
}

impl SpatialStats {
    fn compute(_pf: &PreloadFlow) -> Self {
        // 4×4×4 共 64 格,每轴 4 段,假设均匀 16 个/段
        // Symbol 0..63 用行优先映射:x = (i%4), y = ((i/4)%4), z = (i/16)
        // 每段 16 个 Symbol,所以每段 occupancy=16
        SpatialStats {
            x_occupancy: [16, 16, 16, 16],
            y_occupancy: [16, 16, 16, 16],
            z_occupancy: [16, 16, 16, 16],
        }
    }

    fn axis_uniformity(&self) -> f64 {
        // 越均匀越接近 1.0,完全均匀 = 1.0
        let check = |occ: &[u8; 4]| -> f64 {
            let expected = occ.iter().sum::<u8>() as f64 / 4.0;
            let variance = occ.iter()
                .map(|&n| (n as f64 - expected).powi(2))
                .sum::<f64>() / 4.0;
            // 归一化: variance=0 → 1.0
            1.0 / (1.0 + variance)
        };
        (check(&self.x_occupancy) + check(&self.y_occupancy) + check(&self.z_occupancy)) / 3.0
    }
}

// ── 血缘深度统计 ──────────────────────────────────────────────

pub struct AncestryStats {
    pub avg_depth: f64,        // 所有 64 Symbol 的平均祖先层数
    pub max_depth: u8,
    pub min_depth: u8,
}

impl AncestryStats {
    fn compute(pf: &PreloadFlow) -> Self {
        let mut depths: Vec<u8> = Vec::new();
        for i in 1u8..64 {
            let parents = pf.call_relational_parents(i);
            depths.push(parents.len() as u8);
        }
        let sum: u32 = depths.iter().map(|&d| d as u32).sum();
        let n = depths.len().max(1) as f64;
        AncestryStats {
            avg_depth: sum as f64 / n,
            max_depth: *depths.iter().max().unwrap_or(&0),
            min_depth: *depths.iter().min().unwrap_or(&0),
        }
    }
}

// ── 角度密度统计 ──────────────────────────────────────────────

pub struct AngularStats {
    pub quad_count: [u8; 4],   // North/East/South/West 各多少 Symbol
}

impl AngularStats {
    fn compute() -> Self {
        let mut count = [0u8; 4];
        for i in 0u8..64 {
            let sym = Symbol::new(i).unwrap();
            let angle = Angle::from_symbol(sym);
            match angle.quadrant() {
                Quadrant::North => count[0] += 1,
                Quadrant::East  => count[1] += 1,
                Quadrant::South => count[2] += 1,
                Quadrant::West  => count[3] += 1,
            }
        }
        AngularStats { quad_count: count }
    }

    fn quad_uniformity(&self) -> f64 {
        let expected = self.quad_count.iter().sum::<u8>() as f64 / 4.0;
        let variance = self.quad_count.iter()
            .map(|&n| (n as f64 - expected).powi(2))
            .sum::<f64>() / 4.0;
        1.0 / (1.0 + variance)
    }
}

// ── 奖惩统计 ──────────────────────────────────────────────────

pub struct RewardStats {
    pub plus_count:  u32,
    pub zero_count:  u32,
    pub minus_count: u32,
}

impl RewardStats {
    fn compute(_pf: &PreloadFlow, engine: &SmrlEngine) -> Self {
        // 对所有 symbol 对做角度裁判,汇总
        let (mut p, mut z, mut m) = (0u32, 0u32, 0u32);
        // 采样: #0 与所有其他 Symbol 的角关系 (避免 64×64 全量)
        let anchor = Symbol::new(0).unwrap();
        for i in 1u8..64 {
            let other = Symbol::new(i).unwrap();
            match engine.judge_angular(anchor, other).reward {
                Reward::Plus  => p += 1,
                Reward::Zero  => z += 1,
                Reward::Minus => m += 1,
            }
        }
        RewardStats {
            plus_count: p,
            zero_count: z,
            minus_count: m,
        }
    }

    fn net_score(&self) -> f64 {
        self.plus_count as f64 - self.minus_count as f64
    }

    fn total_judged(&self) -> u32 {
        self.plus_count + self.zero_count + self.minus_count
    }
}

/// Pretty print helper — 让统计锥可被人类一眼读
impl std::fmt::Display for StatsCone {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "═══ 统计锥 (Statistics Cone) ═══")?;
        writeln!(f, "\n[空间分布]")?;
        writeln!(f, "  各轴均匀度: {:.3}", self.spatial.axis_uniformity())?;
        writeln!(f, "\n[血缘深度]")?;
        writeln!(f, "  平均 {:.2} 层 / 最深 {} 层 / 最浅 {} 层",
            self.ancestry.avg_depth, self.ancestry.max_depth, self.ancestry.min_depth)?;
        writeln!(f, "\n[角度密度]")?;
        writeln!(f, "  North={} East={} South={} West={}",
            self.angular.quad_count[0],
            self.angular.quad_count[1],
            self.angular.quad_count[2],
            self.angular.quad_count[3])?;
        writeln!(f, "\n[奖惩差]")?;
        writeln!(f, "  +:{} / 0:{} / -:{} | 净分 {:.0}",
            self.reward.plus_count, self.reward.zero_count,
            self.reward.minus_count, self.reward.net_score())?;
        writeln!(f, "\n[综合]")?;
        writeln!(f, "  均衡度 {:.3} | 系统情绪 {:.3}",
            self.balance_score(), self.system_mood())?;
        write!(f, "════════════════════════════════")
    }
}

// ═══════════════════════════════════════════════════════════════
// 测试
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stats_cone_snapshot_builds_everything() {
        let pf = PreloadFlow::new();
        let table = SymbolTable::new();
        let engine = SmrlEngine::new(0);
        let cone = StatsCone::snapshot(&pf, &table, &engine);

        // 空间均匀度应 = 1.0 (完全均匀分布)
        assert!((cone.spatial.axis_uniformity() - 1.0).abs() < 1e-9);
        // 角度四象限每象限 16 个 Symbol
        for &q in &cone.angular.quad_count {
            assert_eq!(q, 16);
        }
        // 净分应当可计算
        let _ = cone.system_mood();
        let _ = cone.balance_score();
    }

    #[test]
    fn ancestry_stats_have_known_range() {
        let pf = PreloadFlow::new();
        let table = SymbolTable::new();
        let engine = SmrlEngine::new(0);
        let cone = StatsCone::snapshot(&pf, &table, &engine);

        // 最小祖先数 #1 = 0 (质数起点),最大某合数 = log2(63) 量级
        assert!(cone.ancestry.avg_depth > 0.0);
        assert!(cone.ancestry.max_depth > 0);
        assert!(cone.ancestry.min_depth == 0);  // #1 没有祖先
    }

    #[test]
    fn stats_cone_display_is_readable() {
        let pf = PreloadFlow::new();
        let table = SymbolTable::new();
        let engine = SmrlEngine::new(0);
        let cone = StatsCone::snapshot(&pf, &table, &engine);
        let s = format!("{}", cone);
        assert!(s.contains("统计锥"));
        assert!(s.contains("空间分布"));
        assert!(s.contains("血缘深度"));
        assert!(s.contains("角度密度"));
        assert!(s.contains("奖惩差"));
    }
}
