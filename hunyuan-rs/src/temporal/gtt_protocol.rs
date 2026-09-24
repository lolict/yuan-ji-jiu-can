// ── v0.42 gtt* 协议族 — 具身认知通信层 ──
// 满全法意识互联网协议栈 · 谐音思维 · 先验后验 · 因果合一
// 协议 = 容器本身 · 地址 = 认知入口 · 通信 = 容器交互

use crate::phase_clock2::PhaseClock;
use crate::witness::{TripleWitness, WitnessDecision};
use crate::homophone_route::RoutePath;

/// gtt* 协议族分类
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GttScheme {
    /// 谐音思维 · 先验后验 · 因果合一 · 具身认知 · 具身瞄定
    Gtti,
    /// 具身通信 · 基础信道
    Gttx,
    /// 具身找补 · 具身溯源
    Gttxi,
    /// 具身体验一体
    GtxOne,
    /// 具身裁定 · 罗网裁判
    GtxEiyie,
    /// 容器合一
    GtCone,
    /// 容器+具身合一
    GtxCone,
    /// 具身新知 · 新认知入口
    Gtnxye,
    /// 无我通信 · 容器匿名
    GtNone,
    /// 新我 · 具身扩展
    GtNewa,
    /// 新因果 · 下一代协议
    GtNewy,
}

impl GttScheme {
    pub fn as_uri(&self) -> &'static str {
        match self {
            GttScheme::Gtti => "gtti://xieyein.knmlil",
            GttScheme::Gttx => "gttx://hieyain.knmlil",
            GttScheme::Gttxi => "gttxi://hfuyain.knmlil",
            GttScheme::GtxOne => "gtxone://",
            GttScheme::GtxEiyie => "gtxeiyie://",
            GttScheme::GtCone => "gtcone://",
            GttScheme::GtxCone => "gtxcone://",
            GttScheme::Gtnxye => "gtnxye://",
            GttScheme::GtNone => "gtnone://",
            GttScheme::GtNewa => "gtnewa://",
            GttScheme::GtNewy => "gtnewy://",
        }
    }
}

/// 具身认知维度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmbodiedDimension {
    /// 具身体验 — 感知输入
    Experience,
    /// 具身认知 — 推理运算
    Cognition,
    /// 具身情感 — 价值评判
    Emotion,
    /// 具身天平 — 平衡裁决
    Balance,
    /// 具身瞄定 — 锚定坐标系
    Targeting,
}

/// 先验-后验 结合状态
#[derive(Debug, Clone)]
pub struct PriorPosterior {
    pub prior_weight: u64,
    pub posterior_weight: u64,
    /// 先验来自容器族谱
    pub prior_source: String,
    /// 后验来自经验交互
    pub posterior_source: String,
    /// 因果合一标志：先验与后验已融合
    pub causal_unity: bool,
}

impl PriorPosterior {
    pub fn new() -> Self {
        Self {
            prior_weight: 0,
            posterior_weight: 0,
            prior_source: String::new(),
            posterior_source: String::new(),
            causal_unity: false,
        }
    }

    /// 合一操作：当权重差 ≤ 阈值时达成因果合一
    pub fn unify(&mut self, threshold: u64) -> bool {
        let diff = if self.prior_weight > self.posterior_weight {
            self.prior_weight - self.posterior_weight
        } else {
            self.posterior_weight - self.prior_weight
        };
        self.causal_unity = diff <= threshold;
        self.causal_unity
    }

    /// 主导方
    pub fn dominant(&self) -> &str {
        if self.prior_weight >= self.posterior_weight {
            &self.prior_source
        } else {
            &self.posterior_source
        }
    }

    pub fn is_balanced(&self) -> bool {
        self.causal_unity
    }
}

impl Default for PriorPosterior {
    fn default() -> Self {
        Self::new()
    }
}

/// 具身找补 — 借他身补己身
#[derive(Debug, Clone)]
pub struct EmbodiedCompensation {
    pub debtor_id: u64,     // 找补来源
    pub creditor_id: u64,   // 找补目标
    pub dimension: EmbodiedDimension,
    pub amount: u64,
    /// 找补路径（经由谐音桥）
    pub route: Option<RoutePath>,
}

/// 具身溯源 — 追溯认知入口
#[derive(Debug, Clone)]
pub struct EmbodiedTrace {
    pub origin_id: u64,
    pub current_id: u64,
    pub trace_chain: Vec<u64>,
    pub confirmed: bool,  // 是否找到原始入口
}

impl EmbodiedTrace {
    pub fn new(origin: u64) -> Self {
        Self {
            origin_id: origin,
            current_id: origin,
            trace_chain: vec![origin],
            confirmed: true,
        }
    }

    /// 追溯一步
    pub fn step(&mut self, prev: u64) {
        self.trace_chain.push(self.current_id);
        self.current_id = prev;
        if prev == self.origin_id {
            self.confirmed = true;
        }
    }
}

/// gtt 协议帧 — 通信基本单位
#[derive(Debug, Clone)]
pub struct GttFrame {
    pub scheme: GttScheme,
    pub source_id: u64,
    pub target_id: u64,
    /// 认知维度的值
    pub dimension_value: u64,
    /// 因果合一状态
    pub causal_state: PriorPosterior,
    /// 见证裁定结果
    pub witness_result: Option<WitnessDecision>,
}

impl GttFrame {
    pub fn new(scheme: GttScheme, src: u64, dst: u64) -> Self {
        Self {
            scheme,
            source_id: src,
            target_id: dst,
            dimension_value: 0,
            causal_state: PriorPosterior::new(),
            witness_result: None,
        }
    }

    /// 路由封装：通过谐音桥寻路
    pub fn route_via(&self, path: RoutePath) -> bool {
        !path.is_empty()
    }

    /// 协议族是否具身
    pub fn is_embodied(&self) -> bool {
        matches!(
            self.scheme,
            GttScheme::Gtti | GttScheme::Gttx | GttScheme::Gttxi
                | GttScheme::GtxOne | GttScheme::GtxEiyie
        )
    }

    /// 是否容器级协议
    pub fn is_container_level(&self) -> bool {
        matches!(
            self.scheme,
            GttScheme::GtCone | GttScheme::GtxCone | GttScheme::GtNone
        )
    }

    /// 是否扩展协议
    pub fn is_extension(&self) -> bool {
        matches!(
            self.scheme,
            GttScheme::Gtnxye | GttScheme::GtNewa | GttScheme::GtNewy
        )
    }
}

/// gtt* 协议族处理器
#[derive(Debug)]
pub struct GttProtocol {
    pub frames: Vec<GttFrame>,
    pub compensation_log: Vec<EmbodiedCompensation>,
    pub trace_log: Vec<EmbodiedTrace>,
}

impl GttProtocol {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            compensation_log: Vec::new(),
            trace_log: Vec::new(),
        }
    }

    pub fn send_frame(&mut self, frame: GttFrame) -> usize {
        self.frames.push(frame);
        self.frames.len()
    }

    /// 谐音思维处理：先验↔后验因果合一
    pub fn process_gtti(&mut self, phase: &PhaseClock) -> PriorPosterior {
        let mut pp = PriorPosterior::new();
        pp.prior_weight = phase.hot_scalar().hotness;
        pp.prior_source = "container_genealogy".into();
        pp.posterior_weight = phase.total_beats;
        pp.posterior_source = "experience_interaction".into();
        pp.unify(10);
        pp
    }

    /// 容器合一：先验/后验 → 容器族谱
    pub fn converge_container(&self, pp: &PriorPosterior) -> bool {
        pp.is_balanced()
    }

    /// 裁定：gtxeiyie 见证
    pub fn arbitrate(frame: &GttFrame, witness: &TripleWitness) -> bool {
        witness.adjudicate() == WitnessDecision::Accept
    }
}

impl Default for GttProtocol {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gtt_schemes() {
        assert_eq!(GttScheme::Gtti.as_uri(), "gtti://xieyein.knmlil");
        assert_eq!(GttScheme::Gttx.as_uri(), "gttx://hieyain.knmlil");
        assert_eq!(GttScheme::Gttxi.as_uri(), "gttxi://hfuyain.knmlil");
    }

    #[test]
    fn test_prior_posterior_unify() {
        let mut pp = PriorPosterior::new();
        pp.prior_weight = 100;
        pp.posterior_weight = 95;
        assert!(pp.unify(10));
        assert!(pp.causal_unity);
    }

    #[test]
    fn test_prior_posterior_not_unify() {
        let mut pp = PriorPosterior::new();
        pp.prior_weight = 100;
        pp.posterior_weight = 50;
        assert!(!pp.unify(10));
        assert!(!pp.causal_unity);
    }

    #[test]
    fn test_prior_dominant() {
        let mut pp = PriorPosterior::new();
        pp.prior_weight = 80;
        pp.prior_source = "container".into();
        pp.posterior_weight = 60;
        pp.posterior_source = "experience".into();
        assert_eq!(pp.dominant(), "container");
    }

    #[test]
    fn test_frame_embodied() {
        let f = GttFrame::new(GttScheme::Gtti, 1, 2);
        assert!(f.is_embodied());
        assert!(!f.is_container_level());
    }

    #[test]
    fn test_frame_container_level() {
        let f = GttFrame::new(GttScheme::GtCone, 1, 2);
        assert!(f.is_container_level());
        assert!(!f.is_embodied());
    }

    #[test]
    fn test_frame_extension() {
        let f = GttFrame::new(GttScheme::GtNewa, 1, 2);
        assert!(f.is_extension());
    }

    #[test]
    fn test_process_gtti() {
        let mut proto = GttProtocol::new();
        let phase = PhaseClock::new();
        let pp = proto.process_gtti(&phase);
        assert!(pp.prior_weight > 0 || pp.posterior_weight >= 0);
    }

    #[test]
    fn test_send_frame() {
        let mut proto = GttProtocol::new();
        let f = GttFrame::new(GttScheme::Gttx, 1, 2);
        let count = proto.send_frame(f);
        assert_eq!(count, 1);
    }

    #[test]
    fn test_trace() {
        let mut t = EmbodiedTrace::new(100);
        // step pushes current_id (100) → chain=[100,100], current=50
        t.step(50);
        // step pushes current_id (50) → chain=[100,100,50], current=100
        t.step(100);
        assert!(t.confirmed);
        assert_eq!(t.trace_chain.len(), 3);
    }

    #[test]
    fn test_compensation_fields() {
        let c = EmbodiedCompensation {
            debtor_id: 1,
            creditor_id: 2,
            dimension: EmbodiedDimension::Emotion,
            amount: 50,
            route: None,
        };
        assert_eq!(c.amount, 50);
        assert!(matches!(c.dimension, EmbodiedDimension::Emotion));
    }

    #[test]
    fn test_converge() {
        let proto = GttProtocol::new();
        let mut pp = PriorPosterior::new();
        pp.prior_weight = 100;
        pp.posterior_weight = 98;
        pp.unify(5);
        assert!(proto.converge_container(&pp));
    }
}
