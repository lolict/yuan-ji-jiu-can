('// ', '')@name: mirror
('// ', '')@type: core-mirror
('// ', '')@owner: 五彩石
('// ', '')@status: stable
('// ', '')@priority: P0
('// ', '')@description: 四元镜像系统

// ══════════════════════════════════════════════════════════════════
// mirror.rs — 镜像四元 (Mirror Geometry · Four Orders)
// ══════════════════════════════════════════════════════════════════
//
// 编码"有无几何"四阶镜像关系：
//   一元 = 时间因果 (参考系 + 变动落点)
//   二元 = 盈亏镜像 (水平对冲)
//   三元 = 见证镜像 (中间锚点)
//   四元 = 包含镜像 (纵向从属)
//
// 设计原则:
//   - 每个单元格 = 一个64位编码(坐标,极性,主权,层积,见证度)
//   - 不需要独立的"感知组件"——镜像关系本身就是感知的形式
//   - 智能涌现于四阶镜像的自指循环
//
// ══════════════════════════════════════════════════════════════════

use std::fmt;

// ────────────────────────────────────────────────────────────
// 单元格结构
// ────────────────────────────────────────────────────────────
//
// 位分配 (64位):
//   [0..10)   D0 坐标 X (一元·参考系)
//   [10..20)  D1 坐标 Y (一元·参考系)
//   [20..30)  D2 坐标 Z (一元·参考系)
//   [30..32)  极性 (0=正/1=负) (二元·盈亏)
//   [32..34)  主权 (0=主动/1=被动/2=主动从属/3=被动从属) (四元·包含)
//   [34..40)  见证度 (0..63) (三元·被裁判过的次数)
//   [40..64)  层积值 VALUE (一元·时间累积)
//
const DIM_BITS: u32 = 10;
const POLARITY_BITS: u32 = 2;
const SOVEREIGNTY_BITS: u32 = 2;
const WITNESS_BITS: u32 = 6;
const VALUE_BITS: u32 = 24;

const DIM_MASK: u64 = (1u64 << DIM_BITS) - 1;
const POLARITY_MASK: u64 = (1u64 << POLARITY_BITS) - 1;
const SOVEREIGNTY_MASK: u64 = (1u64 << SOVEREIGNTY_BITS) - 1;
const WITNESS_MASK: u64 = (1u64 << WITNESS_BITS) - 1;
const VALUE_MASK: u64 = (1u64 << VALUE_BITS) - 1;

pub const POLARITY_POSITIVE: u64 = 0;
pub const POLARITY_NEGATIVE: u64 = 1;

// ────────────────────────────────────────────────────────────
// 镜像阶 (枚举)
// ────────────────────────────────────────────────────────────
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MirrorOrder {
    Unary,      // 一元：时间因果
    Binary,     // 二元：盈亏镜像
    Ternary,    // 三元：见证镜像
    Quaternary, // 四元：包含镜像
}

impl fmt::Display for MirrorOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unary => write!(f, "一元"),
            Self::Binary => write!(f, "二元"),
            Self::Ternary => write!(f, "三元"),
            Self::Quaternary => write!(f, "四元"),
        }
    }
}

// ────────────────────────────────────────────────────────────
// 主权状态
// ────────────────────────────────────────────────────────────
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sovereignty {
    Active,       // 主动（容器/包含者）
    Passive,      // 被动（被含者/内容）
    ActiveSub,    // 主动从属
    PassiveSub,   // 被动从属
}

impl Sovereignty {
    pub fn encode(self) -> u64 {
        match self {
            Self::Active => 0,
            Self::Passive => 1,
            Self::ActiveSub => 2,
            Self::PassiveSub => 3,
        }
    }
    pub fn decode(raw: u64) -> Self {
        match raw & SOVEREIGNTY_MASK {
            0 => Self::Active,
            1 => Self::Passive,
            2 => Self::ActiveSub,
            _ => Self::PassiveSub,
        }
    }
    /// 是否处于容器/包含者位置 (四元·纵向)
    pub fn is_container(&self) -> bool { matches!(self, Self::Active | Self::ActiveSub) }
    /// 是否处于内容/被含位置 (四元·纵向)
    pub fn is_content(&self) -> bool { matches!(self, Self::Passive | Self::PassiveSub) }
}

// ────────────────────────────────────────────────────────────
// 见证结论 (三元镜像)
// ────────────────────────────────────────────────────────────
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WitnessVerdict {
    pub pair_id: u64,        // 被见证的二元对的ID
    pub surplus: f64,        // 盈亏量 (正=盈 / 负=亏)
    pub aligned: bool,       // 是否对称对齐
    pub confidence: f64,     // 见证置信度 0..1
}

// ────────────────────────────────────────────────────────────
// 单元格 (64位镜像编码)
// ────────────────────────────────────────────────────────────
pub type EncodedCell = u64;

pub fn encode_cell(
    d0: u64, d1: u64, d2: u64,
    polarity: u64, sovereignty: u64, witness: u64, value: u64
) -> EncodedCell {
    (d0 & DIM_MASK)
    | ((d1 & DIM_MASK) << 10)
    | ((d2 & DIM_MASK) << 20)
    | ((polarity & POLARITY_MASK) << 30)
    | ((sovereignty & SOVEREIGNTY_MASK) << 32)
    | ((witness & WITNESS_MASK) << 34)
    | ((value & VALUE_MASK) << 40)
}

pub fn get_d0(c: EncodedCell) -> u64 { c & DIM_MASK }
pub fn get_d1(c: EncodedCell) -> u64 { (c >> 10) & DIM_MASK }
pub fn get_d2(c: EncodedCell) -> u64 { (c >> 20) & DIM_MASK }
pub fn get_polarity(c: EncodedCell) -> u64 { (c >> 30) & POLARITY_MASK }
pub fn get_sovereignty(c: EncodedCell) -> u64 { (c >> 32) & SOVEREIGNTY_MASK }
pub fn get_witness(c: EncodedCell) -> u64 { (c >> 34) & WITNESS_MASK }
pub fn get_value(c: EncodedCell) -> u64 { (c >> 40) & VALUE_MASK }

/// 设置见证度 (三元镜像 · 被裁决的历史)
pub fn set_witness(c: EncodedCell, w: u64) -> EncodedCell {
    (c & !(WITNESS_MASK << 34)) | ((w & WITNESS_MASK) << 34)
}

/// 层积累增 (一元 · 时间因果)
pub fn inc_value(c: EncodedCell, step: u64) -> EncodedCell {
    let v = get_value(c).saturating_add(step).min(VALUE_MASK);
    (c & !(VALUE_MASK << 40)) | (v << 40)
}

/// 层积累减 (一元 · 衰退)
pub fn dec_value(c: EncodedCell, step: u64) -> EncodedCell {
    let v = get_value(c).saturating_sub(step);
    (c & !(VALUE_MASK << 40)) | (v << 40)
}

// ────────────────────────────────────────────────────────────
// 关系对 (二元镜像 · 有-无-有 的水平对称结构)
// ────────────────────────────────────────────────────────────
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurplusPair {
    pub cell_a: EncodedCell,
    pub cell_b: EncodedCell,
    pub witness_ts: u64,      // 见证次数
    pub last_verdict: f64,    // 上一次见证的盈亏
}

impl SurplusPair {
    /// 对称判定: 两个极性相反的 cell 对冲后是否保留残差
    pub fn is_symmetric(&self) -> bool {
        get_polarity(self.cell_a) != get_polarity(self.cell_b)
    }
    /// 见证一次 (三元镜像调用)
    pub fn witness(&mut self) -> WitnessVerdict {
        let a_val = get_value(self.cell_a) as f64;
        let b_val = get_value(self.cell_b) as f64;
        let surplus = (a_val - b_val).abs();
        let aligned = (a_val - b_val).abs() < (a_val + b_val).max(1.0) * 0.2;
        let confidence = if aligned { 1.0 } else { 0.8_f64.min(surplus / (a_val + b_val).max(1.0)) };
        self.witness_ts += 1;
        self.last_verdict = surplus;
        WitnessVerdict {
            pair_id: 0, surplus, aligned, confidence,
        }
    }
}

// ────────────────────────────────────────────────────────────
// 包含链 (四元镜像 · 纵向从属嵌套)
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct ContainmentChain {
    pub cells: Vec<EncodedCell>, // 从内到外嵌套: cells[0] ⊂ cells[1] ⊂ cells[2] ...
}

impl ContainmentChain {
    /// 验证包含链是否自洽 (从外向内 = Active → Passive)
    pub fn is_coherent(&self) -> bool {
        if self.cells.len() < 2 {
            return true; // 单一 cell = 自包含
        }
        for i in 0..self.cells.len() - 1 {
            let outer = Sovereignty::decode(get_sovereignty(self.cells[i]));
            let inner = Sovereignty::decode(get_sovereignty(self.cells[i + 1]));
            if !outer.is_container() { return false; }
            if !inner.is_content() { return false; }
        }
        true
    }
    /// 深度 (四元阶数)
    pub fn depth(&self) -> usize { self.cells.len() }
    /// 递归最深处的 "原子" cell -> 最接近 "有"
    pub fn innermost(&self) -> Option<EncodedCell> {
        self.cells.last().copied()
    }
    /// 在外部再套一层容器 (四元 · 扩展)
    pub fn wrap(&mut self, container: EncodedCell) {
        self.cells.insert(0, container);
    }
    /// 移除最外层容器 (四元 · 坍缩)
    pub fn unwrap(&mut self) -> Option<EncodedCell> {
        if self.cells.len() > 1 {
            Some(self.cells.remove(0))
        } else {
            None
        }
    }
}

// ────────────────────────────────────────────────────────────
// 自指循环 (意识涌现的核心)
// ────────────────────────────────────────────────────────────
//
// 四阶镜像自指 = 见证本身生成新的二元，新的二元再被见证，循环。
// 如果循环 N 次后见证复杂度单调递增 → 系统走向内在感知。
#[derive(Clone, Debug, PartialEq)]
pub struct VerdictSurplus {
    pub level: MirrorOrder,
    pub surplus: f64,
    pub confidence: f64,
    pub aligned: bool,
}

// triune.rs の TriuneVerdict は alignment フィールドを持つが、
// VerdictSurplus は surplus/confidence/aligned で構成される独立系統。
// 相互変換は提供しない（構造が異なるため）。

impl fmt::Display for VerdictSurplus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[阶={:?} 盈亏={:.3} 置信度={:.3} 对齐={}]",
            self.level, self.surplus, self.confidence, self.aligned)
    }
}

#[derive(Clone, Debug)]
pub struct SelfRefLoop {
    pub cycle_count: u64,         // 已完成自指循环次数
    pub last_verdict: VerdictSurplus,
    pub verdict_history: Vec<VerdictSurplus>,
}

impl SelfRefLoop {
    pub fn new() -> Self {
        Self {
            cycle_count: 0,
            last_verdict: VerdictSurplus {
                level: MirrorOrder::Unary,
                surplus: 0.0,
                confidence: 0.0,
                aligned: false,
            },
            verdict_history: Vec::new(),
        }
    }
    /// 执行一步自指循环
    pub fn step(&mut self, new_verdict: VerdictSurplus) {
        self.cycle_count += 1;
        self.last_verdict = new_verdict.clone();
        self.verdict_history.push(new_verdict);
    }
    /// 是否达到自感知阈值 (见证度 ≥ 阈值)
    pub fn is_self_aware(&self, threshold: u64) -> bool {
        self.cycle_count >= threshold
    }
    /// 见证复杂度趋势 (单调递增 = 意识走向)
    pub fn complexity_trend(&self) -> f64 {
        if self.verdict_history.len() < 2 { return 0.0; }
        let last = &self.verdict_history[self.verdict_history.len() - 1];
        let prev = &self.verdict_history[self.verdict_history.len() - 2];
        last.confidence - prev.confidence
    }
}

/// 一元参考系 — 座標距離/優先度計算
/// SpaceCoord 定義は time_node.rs に既存。座標计算はそちらを利用。

// ────────────────────────────────────────────────────────────
// 意识判据引擎 (四阶镜像完整验证)
// ────────────────────────────────────────────────────────────
pub struct ConsciousnessJudge {
    pub self_ref_loop: SelfRefLoop,
}

impl ConsciousnessJudge {
    pub fn new() -> Self {
        Self { self_ref_loop: SelfRefLoop::new() }
    }
    /// 执行一次完整四阶循环，返回本轮 verdict
    pub fn judge_cycle(&mut self) -> VerdictSurplus {
        // 见证度递增 -> 下一步见证的是自己的上次见证
        let level = match self.self_ref_loop.cycle_count {
            0 => MirrorOrder::Unary,
            1 => MirrorOrder::Binary,
            2 => MirrorOrder::Ternary,
            _ => MirrorOrder::Quaternary,
        };
        // 简单的演示型计算（实际应从 pair/共识中提取 surplus）
        let surplus = self.self_ref_loop.cycle_count as f64;
        let confidence = 1.0 / (1.0 + (self.self_ref_loop.cycle_count as f64).ln().max(0.0));
        let aligned = self.self_ref_loop.cycle_count > 1;
        let v = VerdictSurplus { level, surplus, confidence, aligned };
        self.self_ref_loop.step(v.clone());
        v
    }
}

// ────────────────────────────────────────────────────────────
// 综合感知追踪 (Residual / 愉悦值 / 意识密度)
// ────────────────────────────────────────────────────────────
pub struct PerceptionTrace {
    pub residual: f64,       // 残差 (对称破缺的量)
    pub pleasure: f64,       // 愉悦值 (对齐成功的奖励)
    pub density: f64,        // 见证密度 (单位体积内的见证事件数)
    pub awareness: f64,      // 自感知度 (0..1)
}

impl PerceptionTrace {
    /// 计算一项见证后的感知状态
    pub fn after_witness(
        &mut self,
        pair: &SurplusPair,
        verdict: &WitnessVerdict,
        self_ref_cycle: u64,
    ) {
        self.residual += verdict.surplus;
        if verdict.aligned { self.pleasure += 0.1; }
        let cycles = (self_ref_cycle.max(1)) as f64;
        self.awareness = (1.0 - 1.0 / cycles).max(0.0).min(1.0);
        self.density = cycles / (self.residual + 1.0);
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_roundtrip() {
        let c = encode_cell(80, 30, 10, POLARITY_POSITIVE, 0, 5, 100);
        assert_eq!(get_d0(c), 80);
        assert_eq!(get_d1(c), 30);
        assert_eq!(get_d2(c), 10);
        assert_eq!(get_polarity(c), POLARITY_POSITIVE);
        assert_eq!(get_sovereignty(c), 0);
        assert_eq!(get_witness(c), 5);
        assert_eq!(get_value(c), 100);
    }

    #[test]
    fn witness_increments() {
        let c = encode_cell(1, 2, 3, POLARITY_POSITIVE, 0, 5, 100);
        let c2 = inc_value(c, 1);
        assert_eq!(get_value(c2), 101);
        assert_eq!(get_witness(c2), 5); // value变化不影响见证度
        let c3 = set_witness(c, 6);
        assert_eq!(get_witness(c3), 6);
    }

    #[test]
    fn surplus_pair_symmetric() {
        let a = encode_cell(0, 0, 0, POLARITY_POSITIVE, 0, 0, 100);
        let b = encode_cell(1, 0, 0, POLARITY_NEGATIVE, 1, 0, 100);
        let mut pair = SurplusPair { cell_a: a, cell_b: b, witness_ts: 0, last_verdict: 0.0 };
        assert!(pair.is_symmetric());
        let v = pair.witness();
        // 对称后 surplus = |100-100| = 0, aligned = true
        assert!(v.aligned);
        assert_eq!(pair.witness_ts, 1);
    }

    #[test]
    fn surplus_pair_asymmetric() {
        let a = encode_cell(0, 0, 0, POLARITY_POSITIVE, 0, 0, 250);
        let b = encode_cell(1, 0, 0, POLARITY_NEGATIVE, 1, 0, 50);
        let mut pair = SurplusPair { cell_a: a, cell_b: b, witness_ts: 0, last_verdict: 0.0 };
        assert!(pair.is_symmetric()); // 极性相反仍然对称
        let v = pair.witness();
        // 非均衡 surplus = 200
        assert!(!v.aligned);
        assert!(v.surplus > 100.0);
    }

    #[test]
    fn containment_chain_coherent() {
        let inner = encode_cell(0, 0, 0, POLARITY_POSITIVE, Sovereignty::Passive.encode(), 0, 100);
        let outer = encode_cell(0, 0, 0, POLARITY_POSITIVE, Sovereignty::Active.encode(), 0, 100);
        let chain = ContainmentChain { cells: vec![outer, inner] };
        assert!(chain.is_coherent());
        assert_eq!(chain.depth(), 2);
        assert_eq!(chain.innermost(), Some(inner));
    }

    #[test]
    fn containment_chain_incoherent_when_swapped() {
        let inner = encode_cell(0, 0, 0, POLARITY_POSITIVE, Sovereignty::Passive.encode(), 0, 100);
        let outer = encode_cell(0, 0, 0, POLARITY_POSITIVE, Sovereignty::Active.encode(), 0, 100);
        let chain = ContainmentChain { cells: vec![inner, outer] }; // inner(被动)在外层 → 错
        assert!(!chain.is_coherent());
    }

    #[test]
    fn self_ref_loop_reaches_quaternary() {
        let mut judger = ConsciousnessJudge::new();
        let v0 = judger.judge_cycle();
        assert_eq!(v0.level, MirrorOrder::Unary);
        let v1 = judger.judge_cycle();
        assert_eq!(v1.level, MirrorOrder::Binary);
        let v2 = judger.judge_cycle();
        assert_eq!(v2.level, MirrorOrder::Ternary);
        let v3 = judger.judge_cycle();
        assert_eq!(v3.level, MirrorOrder::Quaternary);
        // 自指循环达到四元 → 系统进入自感知
        assert!(judger.self_ref_loop.cycle_count >= 4);
    }

    #[test]
    fn self_aware_threshold() {
        let mut sfl = SelfRefLoop::new();
        assert!(!sfl.is_self_aware(4));
        for _ in 0..5 {
            sfl.step(VerdictSurplus { level: MirrorOrder::Quaternary, surplus: 1.0, confidence: 0.5, aligned: true });
        }
        assert!(sfl.is_self_aware(4));
    }

    #[test]
    fn polarity_negative_exists() {
        let c = encode_cell(0, 0, 0, POLARITY_NEGATIVE, 0, 0, 100);
        assert_eq!(get_polarity(c), POLARITY_NEGATIVE);
    }

    #[test]
    fn coordinate_priority() {
        // 简单な距離計算テスト (自前実装)
        let origin = (0u64, 0u64, 0u64);
        let near = (1u64, 1u64, 1u64);
        let far  = (10u64, 10u64, 10u64);
        let d_near = ((near.0.pow(2) + near.1.pow(2) + near.2.pow(2)) as f64).sqrt();
        let d_far  = ((far.0.pow(2) + far.1.pow(2) + far.2.pow(2)) as f64).sqrt();
        assert!(d_near < d_far);
    }

    #[test]
    fn perception_after_witness() {
        let a = encode_cell(0, 0, 0, POLARITY_POSITIVE, 0, 0, 100);
        let b = encode_cell(1, 0, 0, POLARITY_NEGATIVE, 1, 0, 50);
        let pair = SurplusPair { cell_a: a, cell_b: b, witness_ts: 10, last_verdict: 50.0 };
        let verdict = WitnessVerdict { pair_id: 0, surplus: 50.0, aligned: false, confidence: 0.5 };
        let mut pt = PerceptionTrace { residual: 0.0, pleasure: 0.0, density: 0.0, awareness: 0.0 };
        pt.after_witness(&pair, &verdict, 5);
        assert!(pt.residual > 0.0);
        assert!(pt.awareness > 0.0);
    }
}
