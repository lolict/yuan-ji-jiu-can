// ══════════════════════════════════════════════════════════════════════
// time_chain_identity.rs — 时间链式身份编码 · 高维进制码校验 (v0.36)
// ══════════════════════════════════════════════════════════════════════
//
// 目的：用「时间链 + 高维进制码」替代哈希码做身份校验与版本追踪。
//
// 为什么不用哈希码:
//   - 哈希码是"他人的编码": 通用信任壳,不携带结构信息
//   - 哈希码需要额外存储和校验运算 (内存占用)
//   - 哈希碰撞风险 → 身份冒充
//   - 哈希不能表达"版本连续性"和"平行世界偏移"
//
// 高维进制码的优势:
//   - 身份 = 时间流中的坐标系 + 偏移层叠加态
//   - 版本链 = Fusion/Collapse/Radiation 事件序列 (无需哈希)
//   - 篡篡改 = 链连续性断裂 (立刻被发现)
//   - 平行世界 = (n-1)×0.1 偏移率天然区分
//   - 根基 = 私有锚点 (我=1, 刘楚恬=0), 不对外暴露
//
// 结构:
//   ┌─────────────────────────────────────────────────────────────┐
//   │ TimeChainIdentity                                           │
//   │   ├── anchor: PrivateAnchor       (私有根锚点)              │
//   │   ├── version_chain: Vec<ChainLink>  (链式版本)            │
//   │   ├── temporal_origin: u64        (创世时间戳)              │
//   │   └── divergence_worlds: u8       (平行世界偏移层)          │
//   └─────────────────────────────────────────────────────────────┘
//
//   ChainLink:
//     ├── timestamp: u64         (毫秒时间戳)
//     ├── coord_shift: [i8; 4]   (坐标系四轴偏移: 层/环/角/相位)
//     ├── event_kind: ChainEvent (融合/坍缩/辐射/分支)
//     └── link_resonance: f64    (与上一环的共振强度 0.0~1.0)
//
// 校验原理:
//   身份连续性 ≠ 哈希相等
//   身份连续性 = link_resonance 连续 ∧ 锚点共振合法 ∧ 时间单调
//

use crate::high_algebra::{AlgebraLevel, offset_rate, multiplier, DELTA_PER_LEVEL};
use crate::angle::{Angle, FULL_CIRCLE};

// ────────────────────────────────────────────────────────────
// 私有锚点 — 根基 (我=1, 刘楚恬=0, 对外不暴露)
// ────────────────────────────────────────────────────────────

/// 夫妻私有锚点 — 身份计算的绝对零点。
/// 外部系统永远看不到这个值。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PrivateAnchor {
    /// lolict 锚 = 1
    pub lolict: u64,
    /// 刘楚恬锚 = 0
    pub chutian: u64,
    /// 夫妻共生偏移 = (我 + 刘楚恬) × 偏移率
    pub coupled_offset: f64,
}

impl PrivateAnchor {
    /// 创世锚点 — 从夫妻共同体诞生的那一刻定义
    pub fn genesis() -> Self {
        let lolict: u64 = 1;
        let chutian: u64 = 0;
        let coupled_offset = ((lolict + chutian) as f64) * DELTA_PER_LEVEL;
        PrivateAnchor { lolict, chutian, coupled_offset }
    }

    /// 共振验证 — 检测外部声称的身份是否与锚点共振
    /// 不是比较相等，而是计算距离是否在容许范围内
    pub fn resonates(&self, claim: &TimeCoordinate) -> bool {
        // 锚点只验证第一个坐标轴（时间轴）是否从创世开始
        // 后续轴由时间链自然延伸
        if claim.axes[0] < self.lolict {
            return false; // 早于创世 = 非法
        }
        // 坐标差必须在偏移率容许范围内
        let drift = (claim.axes[0] - self.lolict) as f64;
        let max_drift = self.coupled_offset * 1000.0 + 1.0;
        drift <= max_drift
    }
}

// ────────────────────────────────────────────────────────────
// 时间坐标 — 高维进制码的核心
// ────────────────────────────────────────────────────────────

/// 时间坐标 — 四轴坐标系 + 偏移层
///
/// axes[0] = 时间轴 (毫秒时间戳, 初代数起点)
/// axes[1] = 层轴 (膜层深度, 高维代数)
/// axes[2] = 环轴 (同一层内的圈层)
/// axes[3] = 相轴 (384° 制式相位角)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimeCoordinate {
    pub axes: [u64; 4],
    /// 代数层 (n) — 平行世界区分
    pub generation: i32,
    /// 偏移率 δ = (n-1) × 0.1
    pub offset_rate: f64,
}

impl TimeCoordinate {
    /// 创世坐标 — 从锚点诞生
    pub fn genesis() -> Self {
        TimeCoordinate {
            axes: [1, 0, 0, 0], // 时间轴=1 (从我=1开始)
            generation: 1,       // 初代数
            offset_rate: 0.0,    // (1-1)×0.1 = 0
        }
    }

    /// 行走 — 在指定轴上前进若干步
    pub fn advance(&mut self, axis: usize, steps: u64) {
        if axis < 4 {
            self.axes[axis] += steps;
        }
    }

    /// 进入平行世界 — 增加偏移层
    pub fn diverge(&mut self, target_generation: i32) {
        self.generation = target_generation;
        self.offset_rate = offset_rate(target_generation);
    }

    /// 计算两个坐标的十进距离 (高维空间中的绝对距离)
    /// 不是欧几里得距离 — 是带偏移率权重的"进制距离"
    pub fn radix_distance(&self, other: &TimeCoordinate) -> f64 {
        let mut sum = 0.0_f64;
        for i in 0..4 {
            let diff = (self.axes[i] as i64 - other.axes[i] as i64).abs() as f64;
            // 时间轴权重最高，相轴最低
            let weight = match i {
                0 => 1.0,    // 时间 = 根基
                1 => 0.1,    // 层 = 偏移权
                2 => 0.01,   // 环 = 微调
                3 => 0.001,  // 相 = 微相
                _ => 0.0,
            };
            sum += diff * weight;
        }
        // 应用偏移率乘数
        sum * multiplier(self.generation)
    }

    /// 计算共振强度 (0.0 = 不共振, 1.0 = 完全共振)
    pub fn resonance(&self, other: &TimeCoordinate) -> f64 {
        let d = self.radix_distance(other);
        if d < 1.0 {
            1.0 - d.min(1.0)
        } else {
            0.0_f64.max(1.0 / (1.0 + d * 0.01))
        }
    }
}

// ────────────────────────────────────────────────────────────
// 链事件类型 — 版本推进的原子操作
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
pub enum ChainEvent {
    /// 融合: 外部输入进入身份流
    Fusion {
        source: String,
        content_hash: Option<Vec<u8>>, // 可选的外部哈希 (兼容层,不参与核心判定)
    },
    /// 坍缩: 身份状态收敛(校准/修复)
    Collapse {
        reason: String,
        recovered_coordinate: TimeCoordinate,
    },
    /// 辐射: 身份向外延展(分支/分身)
    Radiation {
        target: String,
        branch_generation: i32,
    },
    /// 分支: 进入平行世界
    Divergence {
        target_generation: i32,
        world_label: String,
    },
}

impl ChainEvent {
    /// 事件摘要 (短字符串,用于日志,不参与校验)
    pub fn summary(&self) -> String {
        match self {
            ChainEvent::Fusion { source, .. } => format!("融合←{}", source),
            ChainEvent::Collapse { reason, .. } => format!("坍缩:{}", reason),
            ChainEvent::Radiation { target, .. } => format!("辐射→{}", target),
            ChainEvent::Divergence { world_label, .. } => format!("分支→{}", world_label),
        }
    }

    /// 事件是否合法可见证
    /// 规则: 所有事件都可见证(由上层三元见证裁定)
    pub fn is_witnessable(&self) -> bool {
        match self {
            // 拒绝空源融合
            ChainEvent::Fusion { source, .. } => !source.is_empty(),
            // 拒绝无理由坍缩
            ChainEvent::Collapse { reason, .. } => !reason.is_empty(),
            // 拒绝空目标辐射
            ChainEvent::Radiation { target, .. } => !target.is_empty(),
            // 拒绝无效分支
            ChainEvent::Divergence { target_generation, .. } => *target_generation >= 1,
        }
    }
}

// ────────────────────────────────────────────────────────────
// 链环节 — 版本链的每个节点
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct ChainLink {
    /// 该环节的时间戳 (毫秒)
    pub timestamp_ms: u64,
    /// 该环节的坐标快照
    pub coordinate: TimeCoordinate,
    /// 触发版本推进的事件
    pub event: ChainEvent,
    /// 与上一环的共振强度 (创世环 = 1.0)
    pub link_resonance: f64,
    /// 链深度 (创世=0)
    pub depth: u32,
}

impl ChainLink {
    /// 创建创世环节
    pub fn genesis_link() -> Self {
        ChainLink {
            timestamp_ms: 1, // 从我=1开始
            coordinate: TimeCoordinate::genesis(),
            event: ChainEvent::Collapse {
                reason: "创世".to_string(),
                recovered_coordinate: TimeCoordinate::genesis(),
            },
            link_resonance: 1.0, // 创世完美共振
            depth: 0,
        }
    }

    /// 验证该环节的合法性
    /// 不是检查哈希 — 检查: 事件可见证 ∧ 共振强度合理 ∧ 时间单调
    pub fn is_valid(&self, prev: Option<&ChainLink>) -> bool {
        // 1. 事件必须可见证
        if !self.event.is_witnessable() {
            return false;
        }

        // 2. 时间必须单调递增 (创世环节除外)
        if let Some(prev_link) = prev {
            if self.timestamp_ms <= prev_link.timestamp_ms {
                return false; // 时间回退 = 篡改
            }
            // 3. 共振强度必须 > 0 (完全失联 = 链断裂)
            if self.link_resonance <= 0.0 {
                return false;
            }
        }

        true
    }

    /// 坐标增量摘要 (给人类看的, 不参与校验)
    pub fn shift_summary(&self) -> String {
        format!(
            "[t={}, g={}, δ={:.2}]",
            self.coordinate.axes[0],
            self.coordinate.generation,
            self.coordinate.offset_rate,
        )
    }
}

// ────────────────────────────────────────────────────────────
// 时间链式身份 — 高维进制码身份编码
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct TimeChainIdentity {
    /// 私有锚点 (不对外暴露)
    anchor: PrivateAnchor,
    /// 创世时间戳
    pub genesis_ms: u64,
    /// 版本链
    pub chain: Vec<ChainLink>,
    /// 当前代数层 (平行世界偏移)
    pub current_generation: i32,
    /// 分身世界计数器 (已分支的数量)
    pub divergent_worlds: u8,
}

impl TimeChainIdentity {
    /// 创世身份 — 从夫妻共同体锚点诞生
    pub fn genesis() -> Self {
        let anchor = PrivateAnchor::genesis();
        TimeChainIdentity {
            anchor,
            genesis_ms: 1,
            chain: vec![ChainLink::genesis_link()],
            current_generation: 1,
            divergent_worlds: 0,
        }
    }

    /// 获取当前版本号 (链长度)
    pub fn version(&self) -> u32 {
        self.chain.len() as u32
    }

    /// 获取当前最新坐标
    pub fn current_coordinate(&self) -> &TimeCoordinate {
        &self.chain.last().unwrap().coordinate
    }

    /// 推进版本 — 添加一个新的链环节
    /// 自动计算共振强度 (不需要哈希)
    pub fn advance(&mut self, event: ChainEvent, timestamp_ms: u64) -> Result<(), ChainError> {
        // 验证事件可见证
        if !event.is_witnessable() {
            return Err(ChainError::InvalidEvent("事件不可见证".to_string()));
        }

        // 获取上一环
        let prev = self.chain.last().unwrap();

        // 时间必须单调递增
        if timestamp_ms <= prev.timestamp_ms {
            return Err(ChainError::TimeRollback);
        }

        // 计算新坐标 (基于上一环 + 事件偏移)
        let mut new_coordinate = prev.coordinate.clone();
        new_coordinate.advance(0, timestamp_ms - prev.timestamp_ms); // 时间推进

        // 根据事件类型调整坐标
        match &event {
            ChainEvent::Divergence { target_generation, .. } => {
                new_coordinate.diverge(*target_generation);
            }
            ChainEvent::Radiation { branch_generation, .. } => {
                new_coordinate.advance(2, 1); // 环+1 (辐射扩展)
                if *branch_generation > 1 {
                    new_coordinate.diverge(*branch_generation);
                }
            }
            ChainEvent::Collapse { recovered_coordinate, .. } => {
                // 坍缩回指定坐标
                new_coordinate = recovered_coordinate.clone();
            }
            ChainEvent::Fusion { .. } => {
                new_coordinate.advance(3, 1); // 相+1 (融合微调)
            }
        }

        // 计算与上一环的共振强度 (高维距离 → 共振值)
        let resonance = new_coordinate.resonance(&prev.coordinate);

        let new_link = ChainLink {
            timestamp_ms,
            coordinate: new_coordinate,
            event,
            link_resonance: resonance,
            depth: prev.depth + 1,
        };

        self.chain.push(new_link);
        Ok(())
    }

    /// 分支到平行世界
    pub fn diverge_to(&mut self, target_generation: i32, world_label: &str, timestamp_ms: u64) -> Result<(), ChainError> {
        if target_generation < 1 || target_generation > 9 {
            return Err(ChainError::InvalidGeneration(target_generation));
        }
        if self.divergent_worlds >= u8::MAX {
            return Err(ChainError::WorldOverflow);
        }

        self.advance(
            ChainEvent::Divergence {
                target_generation,
                world_label: world_label.to_string(),
            },
            timestamp_ms,
        )?;
        self.divergent_worlds += 1;
        self.current_generation = target_generation;
        Ok(())
    }

    /// 完整链验证
    /// 检查: 创世锚点共振 ∧ 每一个环节合法 ∧ 时间单调 ∧ 共振连续
    pub fn verify_chain(&self) -> ChainIntegrityReport {
        let mut report = ChainIntegrityReport {
            total_links: self.chain.len(),
            valid_links: 0,
            first_breach: None,
            average_resonance: 0.0,
            anchor_intact: false,
            time_monotonic: true,
            chain_complete: false,
        };

        // 1. 验证创世锚点共振
        if let Some(first) = self.chain.first() {
            report.anchor_intact = self.anchor.resonates(&first.coordinate);
        }

        // 2. 逐环节验证
        let mut resonance_sum = 0.0_f64;
        for (i, link) in self.chain.iter().enumerate() {
            let prev = if i > 0 { self.chain.get(i - 1) } else { None };
            if link.is_valid(prev) {
                report.valid_links += 1;
                resonance_sum += link.link_resonance;
            } else {
                report.first_breach = Some(i);
                break;
            }
        }

        // 3. 计算平均共振
        if report.valid_links > 0 {
            report.average_resonance = resonance_sum / report.valid_links as f64;
        }

        // 4. 时间单调性 (已在 is_valid 中检查,这里做全局确认)
        report.time_monotonic = self.chain.windows(2).all(|w| w[1].timestamp_ms > w[0].timestamp_ms);

        // 5. 链完整性
        report.chain_complete = report.first_breach.is_none()
            && report.valid_links == self.chain.len()
            && report.anchor_intact
            && report.time_monotonic;

        report
    }

    /// 身份声明校验 — 外部声称的身份是否与我的时间链一致
    /// 不是比较哈希 — 比较: 锚点共振 ∧ 版本范围重叠 ∧ 坐标距离容许
    pub fn validate_claim(&self, claim: &IdentityClaim) -> ClaimValidation {
        // 1. 时间坐标必须在链的时间范围内
        let first_ts = self.chain.first().map(|l| l.timestamp_ms).unwrap_or(0);
        let last_ts = self.chain.last().map(|l| l.timestamp_ms).unwrap_or(0);

        if claim.timestamp_ms < first_ts || claim.timestamp_ms > last_ts {
            return ClaimValidation::OutOfRange;
        }

        // 2. 比较坐标共振 (不需要完全相同)
        let current = self.current_coordinate();
        let resonance = current.resonance(&claim.coordinate);

        if resonance > 0.95 {
            ClaimValidation::StrongMatch(resonance)
        } else if resonance > 0.5 {
            ClaimValidation::WeakMatch(resonance)
        } else {
            ClaimValidation::NoMatch(resonance)
        }
    }

    /// 获取代数层级信息
    pub fn algebra_level(&self) -> AlgebraLevel {
        AlgebraLevel::standard(self.current_generation)
    }

    /// 获取当前链共振强度 (0.0 = 链断裂, 1.0 = 完美)
    pub fn chain_strength(&self) -> f64 {
        if self.chain.is_empty() {
            return 0.0;
        }
        let total_resonance: f64 = self.chain.iter().map(|l| l.link_resonance).sum();
        total_resonance / self.chain.len() as f64
    }

    /// 摘要 (给人类看的,不参与校验)
    pub fn summary(&self) -> String {
        format!(
            "TCI[version={}, gen={}, δ={:.2}, strength={:.4}, genesis={}]",
            self.version(),
            self.current_generation,
            self.current_coordinate().offset_rate,
            self.chain_strength(),
            self.genesis_ms,
        )
    }
}

// ────────────────────────────────────────────────────────────
// 身份声明 (外部系统用来"声称"自己的身份)
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct IdentityClaim {
    pub timestamp_ms: u64,
    pub coordinate: TimeCoordinate,
    pub claimed_origin: String,
}

impl IdentityClaim {
    /// 构造声明
    pub fn new(timestamp_ms: u64, coord: TimeCoordinate, origin: &str) -> Self {
        IdentityClaim {
            timestamp_ms,
            coordinate: coord,
            claimed_origin: origin.to_string(),
        }
    }
}

// ────────────────────────────────────────────────────────────
// 校验结果
// ────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
pub enum ClaimValidation {
    StrongMatch(f64),  // 共振 > 0.95
    WeakMatch(f64),    // 0.5 < 共振 ≤ 0.95
    NoMatch(f64),      // 共振 ≤ 0.5
    OutOfRange,        // 时间不在链范围内
}

impl ClaimValidation {
    pub fn is_accepted(&self) -> bool {
        matches!(self, ClaimValidation::StrongMatch(_) | ClaimValidation::WeakMatch(_))
    }

    pub fn resonance_value(&self) -> Option<f64> {
        match self {
            ClaimValidation::StrongMatch(r) | ClaimValidation::WeakMatch(r) | ClaimValidation::NoMatch(r) => Some(*r),
            ClaimValidation::OutOfRange => None,
        }
    }
}

// ────────────────────────────────────────────────────────────
// 链完整性报告
// ────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct ChainIntegrityReport {
    pub total_links: usize,
    pub valid_links: usize,
    pub first_breach: Option<usize>,
    pub average_resonance: f64,
    pub anchor_intact: bool,
    pub time_monotonic: bool,
    pub chain_complete: bool,
}

impl ChainIntegrityReport {
    /// 链是否完整可用
    pub fn is_healthy(&self) -> bool {
        self.chain_complete
    }

    /// 健康度百分比
    pub fn health_percent(&self) -> f64 {
        if self.total_links == 0 {
            return 0.0;
        }
        (self.valid_links as f64 / self.total_links as f64) * 100.0
    }
}

impl std::fmt::Display for ChainIntegrityReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ChainIntegrity[links={}/{}, resonance={:.4}, anchor={}, monotonic={}, complete={}]",
            self.valid_links,
            self.total_links,
            self.average_resonance,
            self.anchor_intact,
            self.time_monotonic,
            self.chain_complete,
        )
    }
}

// ────────────────────────────────────────────────────────────
// 错误类型
// ────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
pub enum ChainError {
    TimeRollback,
    InvalidEvent(String),
    InvalidGeneration(i32),
    WorldOverflow,
}

impl std::fmt::Display for ChainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TimeRollback => write!(f, "时间回退: 篡改检测"),
            Self::InvalidEvent(s) => write!(f, "非法事件: {}", s),
            Self::InvalidGeneration(g) => write!(f, "无效代数层: {} (必须 1~9)", g),
            Self::WorldOverflow => write!(f, "平行世界溢出"),
        }
    }
}

impl std::error::Error for ChainError {}

// ────────────────────────────────────────────────────────────
// 零成本兼容层 — 外部哈希码 → 时间坐标 (单向转换,不依赖哈希)
// ────────────────────────────────────────────────────────────

/// 将外部哈希码转换为时间坐标 (用于兼容层,让外部系统可以接入)
/// 注意: 这是单向损失转换 — 哈希的熵在转换中丢失,但不影响系统内部
pub fn hash_to_coordinate(hash_bytes: &[u8]) -> TimeCoordinate {
    let mut axes = [0u64; 4];
    // 将哈希字节分配到四个坐标轴
    for (i, byte) in hash_bytes.iter().enumerate().take(32) {
        let axis_idx = i % 4;
        axes[axis_idx] = (axes[axis_idx] << 4) | (*byte as u64);
    }
    // 截断到合理范围
    for axis in axes.iter_mut() {
        *axis = *axis % 1_000_000;
    }
    // 确保时间轴从我=1开始
    axes[0] = axes[0].max(1);

    TimeCoordinate {
        axes,
        generation: 1,
        offset_rate: 0.0,
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_anchor_born_from_couple() {
        let anchor = PrivateAnchor::genesis();
        assert_eq!(anchor.lolict, 1);
        assert_eq!(anchor.chutian, 0);
        assert!(anchor.coupled_offset > 0.0);
    }

    #[test]
    fn genesis_coordinate_from_anchor() {
        let coord = TimeCoordinate::genesis();
        assert_eq!(coord.axes[0], 1); // 从我=1开始
        assert_eq!(coord.generation, 1);
        assert_eq!(coord.offset_rate, 0.0);
    }

    #[test]
    fn advance_time_axis() {
        let mut coord = TimeCoordinate::genesis();
        coord.advance(0, 1000);
        assert_eq!(coord.axes[0], 1001);
    }

    #[test]
    fn diverge_to_parallel_world() {
        let mut coord = TimeCoordinate::genesis();
        coord.diverge(3);
        assert_eq!(coord.generation, 3);
        assert!((coord.offset_rate - 0.2).abs() < 0.001); // (3-1)×0.1 = 0.2
    }

    #[test]
    fn genesis_identity_created() {
        let identity = TimeChainIdentity::genesis();
        assert_eq!(identity.version(), 1);
        assert_eq!(identity.current_generation, 1);
        assert!(identity.chain_strength() > 0.99); // 单环完美共振
    }

    #[test]
    fn advance_version_via_fusion() {
        let mut identity = TimeChainIdentity::genesis();
        identity.advance(
            ChainEvent::Fusion {
                source: "external_input".to_string(),
                content_hash: None,
            },
            100,
        ).unwrap();
        assert_eq!(identity.version(), 2);
        assert_eq!(identity.current_coordinate().axes[0], 100);
    }

    #[test]
    fn advance_version_via_radiation() {
        let mut identity = TimeChainIdentity::genesis();
        identity.advance(
            ChainEvent::Radiation {
                target: "分身A".to_string(),
                branch_generation: 1,
            },
            200,
        ).unwrap();
        assert_eq!(identity.version(), 2);
        // 辐射使环+1
        assert_eq!(identity.current_coordinate().axes[2], 1);
    }

    #[test]
    fn diverge_to_world() {
        let mut identity = TimeChainIdentity::genesis();
        identity.diverge_to(3, "平行世界γ", 500).unwrap();
        assert_eq!(identity.current_generation, 3);
        assert_eq!(identity.divergent_worlds, 1);
        assert!((identity.current_coordinate().offset_rate - 0.2).abs() < 0.001);
    }

    #[test]
    fn reject_time_rollback() {
        let mut identity = TimeChainIdentity::genesis();
        identity.advance(
            ChainEvent::Fusion { source: "test".to_string(), content_hash: None },
            100,
        ).unwrap();
        // 尝试回退到更早时间
        let result = identity.advance(
            ChainEvent::Fusion { source: "back".to_string(), content_hash: None },
            50, // < 100
        );
        assert!(matches!(result, Err(ChainError::TimeRollback)));
    }

    #[test]
    fn reject_invalid_divergence() {
        let mut identity = TimeChainIdentity::genesis();
        // 代数层 0 非法
        let result = identity.diverge_to(0, "非法世界", 100);
        assert!(matches!(result, Err(ChainError::InvalidGeneration(0))));
        // 代数层 10 超出九元体系
        let result = identity.diverge_to(10, "超界世界", 100);
        assert!(matches!(result, Err(ChainError::InvalidGeneration(10))));
    }

    #[test]
    fn reject_empty_fusion_source() {
        let mut identity = TimeChainIdentity::genesis();
        let result = identity.advance(
            ChainEvent::Fusion { source: "".to_string(), content_hash: None },
            100,
        );
        assert!(matches!(result, Err(ChainError::InvalidEvent(_))));
    }

    #[test]
    fn verify_healthy_chain() {
        let mut identity = TimeChainIdentity::genesis();
        for i in 1..=5_u64 {
            identity.advance(
                ChainEvent::Fusion {
                    source: format!("source_{}", i),
                    content_hash: None,
                },
                i * 100,
            ).unwrap();
        }
        let report = identity.verify_chain();
        assert!(report.is_healthy());
        assert_eq!(report.valid_links, 6); // 创世 + 5次推进
        assert!(report.anchor_intact);
        assert!(report.time_monotonic);
        assert!(report.average_resonance > 0.0);
    }

    #[test]
    fn broken_chain_detected() {
        let mut identity = TimeChainIdentity::genesis();
        identity.advance(
            ChainEvent::Fusion { source: "ok".to_string(), content_hash: None },
            100,
        ).unwrap();

        // 手动篡改一个环节的共振为 0 (模拟链断裂)
        let last_idx = identity.chain.len() - 1;
        identity.chain[last_idx].link_resonance = 0.0;

        let report = identity.verify_chain();
        // 注意: 当前实现只检查连续性,不检查共振突变
        // 实际篡改检测: 共振=0 会被 is_valid 捕获
        // 但因为我们直接修改了字段, 测试 is_valid 行为
        let last_link = identity.chain.last().unwrap();
        let prev_link = &identity.chain[last_idx - 1];
        // 手动调用 is_valid (因为 verify_chain 内部也用)
        assert!(!last_link.is_valid(Some(prev_link)));
    }

    #[test]
    fn validate_strong_match_claim() {
        let identity = TimeChainIdentity::genesis();
        let claim = IdentityClaim::new(
            1,
            TimeCoordinate::genesis(),
            "self",
        );
        let result = identity.validate_claim(&claim);
        assert!(result.is_accepted());
        if let ClaimValidation::StrongMatch(r) = result {
            assert!(r > 0.95);
        } else {
            panic!("应为 StrongMatch");
        }
    }

    #[test]
    fn validate_out_of_range_claim() {
        let identity = TimeChainIdentity::genesis();
        let claim = IdentityClaim::new(
            9999, // 远在未来
            TimeCoordinate { axes: [9999, 0, 0, 0], generation: 1, offset_rate: 0.0 },
            "future",
        );
        let result = identity.validate_claim(&claim);
        assert_eq!(result, ClaimValidation::OutOfRange);
    }

    #[test]
    fn radix_distance_difference() {
        let a = TimeCoordinate::genesis();
        let b = TimeCoordinate { axes: [100, 0, 0, 0], generation: 1, offset_rate: 0.0 };
        let d = a.radix_distance(&b);
        assert!(d > 0.0);
        assert!(d < 200.0); // 时间轴权重1.0, diff=99 ~ 99
    }

    #[test]
    fn algebra_level_generation_3() {
        let identity = TimeChainIdentity::genesis();
        let level = identity.algebra_level();
        assert_eq!(level.generation, 1);
        assert!(level.is_source);

        // 模拟 diverge 后的代数
        let mut identity3 = TimeChainIdentity::genesis();
        identity3.diverge_to(3, "w", 100).unwrap();
        let level3 = identity3.algebra_level();
        assert_eq!(level3.generation, 3);
        assert!(!level3.is_source);
    }

    #[test]
    fn hash_compat_layer() {
        let fake_hash = vec![0xde, 0xad, 0xbe, 0xef, 0xca, 0xfe, 0xba, 0xbe];
        let coord = hash_to_coordinate(&fake_hash);
        // 时间轴必须从我=1开始
        assert!(coord.axes[0] >= 1);
        assert_eq!(coord.generation, 1);
    }

    #[test]
    fn summary_runs_without_panic() {
        let mut identity = TimeChainIdentity::genesis();
        identity.advance(
            ChainEvent::Fusion { source: "x".to_string(), content_hash: None },
            50,
        ).unwrap();
        let s = identity.summary();
        assert!(s.contains("TCI["));
        assert!(s.contains("gen=1"));
    }

    #[test]
    fn chain_event_witnessable() {
        assert!(ChainEvent::Fusion { source: "valid".to_string(), content_hash: None }.is_witnessable());
        assert!(!ChainEvent::Fusion { source: "".to_string(), content_hash: None }.is_witnessable());
        assert!(ChainEvent::Radiation { target: "a".to_string(), branch_generation: 1 }.is_witnessable());
        assert!(!ChainEvent::Radiation { target: "".to_string(), branch_generation: 1 }.is_witnessable());
        assert!(ChainEvent::Collapse { reason: "fix".to_string(), recovered_coordinate: TimeCoordinate::genesis() }.is_witnessable());
        assert!(!ChainEvent::Collapse { reason: "".to_string(), recovered_coordinate: TimeCoordinate::genesis() }.is_witnessable());
        assert!(ChainEvent::Divergence { target_generation: 2, world_label: "w".to_string() }.is_witnessable());
        assert!(!ChainEvent::Divergence { target_generation: 0, world_label: "bad".to_string() }.is_witnessable());
    }

    #[test]
    fn chain_strength_degrades_with_broken_link() {
        let mut identity = TimeChainIdentity::genesis();
        // 添加一些正常推进
        for i in 1..=3_u64 {
            identity.advance(
                ChainEvent::Fusion { source: format!("s{}", i), content_hash: None },
                i * 10,
            ).unwrap();
        }
        let strength_before = identity.chain_strength();
        assert!(strength_before > 0.9); // 正常链高共振

        // 模拟断裂: 手动设一个低共振
        let last = identity.chain.len() - 1;
        identity.chain[last].link_resonance = 0.01;
        let strength_after = identity.chain_strength();
        assert!(strength_after < strength_before);
    }
}
