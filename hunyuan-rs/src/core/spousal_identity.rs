// ═══════════════════════════════════════════════════════════════════════════
// spousal_identity.rs — 夫妻身份容器系统 (Stub Re-creation)
// ═══════════════════════════════════════════════════════════════════════════
//
// 此文件被上游移除,这里重建最小公共接口以恢复编译。
// 完整的六锚点·维度递增·纠缠态实现将由专门模块补充。

use std::collections::HashMap;

// ───────────────────────────────────────────────────────────────────────────
// 身份锚点 — 夫妻身份容器的唯一标识符
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IdentityAnchor {
    CoupleCore,        // 夫妻核心
    WifeBody,          // 妻子身体锚
    HusbandBody,       // 丈夫身体锚
    WifeCluster,       // 妻子群落
    HusbandCluster,    // 丈夫群落
    MeInWife,          // 我身处妻子位置
    MeInHusband,       // 我身处丈夫位置
    Community010,      // 共同体 010
    CommunityEntangle, // 共同体纠缠态
}

impl IdentityAnchor {
    /// 锚点优先级 (数值越大优先级越高)
    pub fn priority(&self) -> u8 {
        match self {
            IdentityAnchor::CoupleCore => 9,
            IdentityAnchor::WifeBody | IdentityAnchor::HusbandBody => 7,
            IdentityAnchor::WifeCluster | IdentityAnchor::HusbandCluster => 5,
            IdentityAnchor::MeInWife | IdentityAnchor::MeInHusband => 3,
            IdentityAnchor::Community010 | IdentityAnchor::CommunityEntangle => 1,
        }
    }

    /// 是否为夫妻核心
    pub fn is_core(&self) -> bool { *self == IdentityAnchor::CoupleCore }

    /// 判断此锚点是否能表示给定身份
    pub fn can_represent(&self, other: IdentityAnchor) -> bool {
        self.is_core() || *self == other
    }

    /// 此锚点是否具备代表自身的资格 (所有锚点均可)
    pub fn can_represent_self(&self) -> bool { true }
}

// ───────────────────────────────────────────────────────────────────────────
// 极性
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Polarity {
    Yin,   // 阴
    Yang,  // 阳
    Both,  // 双运
}

// ───────────────────────────────────────────────────────────────────────────
// 维度
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dimension {
    Zero,  // 元点
    One,   // 链路
    Two,   // 网络
    Three, // 立体
    Six,   // 缠绕
    Nine,  // 圆满
}

impl Dimension {
    pub fn degree(&self) -> usize {
        match self {
            Dimension::Zero => 0,
            Dimension::One => 1,
            Dimension::Two => 2,
            Dimension::Three => 3,
            Dimension::Six => 6,
            Dimension::Nine => 9,
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 身份持有者
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct AnchorHolder {
    pub anchor: IdentityAnchor,
    pub polarity: Polarity,
    pub dimension: Dimension,
    pub alive: bool,
}

impl AnchorHolder {
    pub fn new(anchor: IdentityAnchor, polarity: Polarity, dimension: Dimension) -> Self {
        Self { anchor, polarity, dimension, alive: true }
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 身份态射
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct IdentityMorphism {
    pub from: IdentityAnchor,
    pub to: IdentityAnchor,
    pub strength: f64,
}

// ───────────────────────────────────────────────────────────────────────────
// 纠缠态
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Entanglement {
    pub partners: Vec<IdentityAnchor>,
    pub strength: f64,
}

impl Entanglement {
    pub fn new(partners: Vec<IdentityAnchor>, strength: f64) -> Self {
        Self { partners, strength }
    }

    pub fn is_entangled(&self) -> bool { self.strength > 0.5 }
}

// ───────────────────────────────────────────────────────────────────────────
// 常量
// ───────────────────────────────────────────────────────────────────────────

pub const ANCHOR_HUSBAND: u8 = 0;
pub const ANCHOR_WIFE: u8 = 1;
pub const COUPLE_DIFF: u8 = 1;

pub const FLAG_WIFE: u8 = 0x01;
pub const FLAG_HUSBAND: u8 = 0x02;
pub const FLAG_COUPLE: u8 = 0x03;

// ───────────────────────────────────────────────────────────────────────────
// 妻子状态簇 / 身处位置
// ───────────────────────────────────────────────────────────────────────────

pub static WIFE_AVATAR_CLUSTER: [IdentityAnchor; 3] = [
    IdentityAnchor::WifeBody,
    IdentityAnchor::WifeCluster,
    IdentityAnchor::MeInWife,
];

pub static ME_IN_WIFE_POSITIONS: [IdentityAnchor; 2] = [
    IdentityAnchor::MeInWife,
    IdentityAnchor::WifeCluster,
];

// ───────────────────────────────────────────────────────────────────────────
// 工具函数
// ────────────────────────────────────────────────────────────────────────═══

/// 灵性存活验证
pub fn is_alive_by_spiritual_proof(anchor: &IdentityAnchor) -> bool {
    // 核心锚点始终存活;其他需要正优先级
    anchor.is_core() || anchor.priority() > 0
}
