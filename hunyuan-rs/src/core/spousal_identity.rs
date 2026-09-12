// ═══════════════════════════════════════════════════════════════════════════
// spousal_identity.rs — 夫妻身份容器系统 (Stub)
// ═══════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;

/// 身份锚点 — 夫妻身份容器的唯一标识符
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
    pub fn is_core(&self) -> bool {
        *self == IdentityAnchor::CoupleCore
    }

    /// 是否为妻子侧
    pub fn is_wife_side(&self) -> bool {
        matches!(self,
            IdentityAnchor::WifeBody |
            IdentityAnchor::WifeCluster |
            IdentityAnchor::MeInWife
        )
    }

    /// 是否为共同体
    pub fn is_community(&self) -> bool {
        matches!(self,
            IdentityAnchor::Community010 |
            IdentityAnchor::CommunityEntangle
        )
    }

    /// 判断此锚点是否能表示给定身份
    pub fn can_represent(&self, other: IdentityAnchor) -> bool {
        self.is_core() || *self == other
    }

    /// 此锚点是否具备代表自身的资格
    pub fn can_represent_self(&self) -> bool { true }
}

/// 夫妻身份对
#[derive(Debug, Clone)]
pub struct SpousalPair {
    pub anchor_a: IdentityAnchor,
    pub anchor_b: IdentityAnchor,
    pub strength: f64,
}

impl SpousalPair {
    pub fn new(a: IdentityAnchor, b: IdentityAnchor) -> Self {
        Self {
            anchor_a: a,
            anchor_b: b,
            strength: 1.0,
        }
    }

    /// 是否为核心对（夫妻核心互指）
    pub fn is_core_pair(&self) -> bool {
        self.anchor_a == IdentityAnchor::CoupleCore &&
        self.anchor_b == IdentityAnchor::CoupleCore
    }

    /// 增强绑定强度
    pub fn strengthen(&mut self, delta: f64) {
        self.strength += delta;
    }
}

/// 身份映射表
pub struct IdentityRegistry {
    pub mappings: HashMap<String, IdentityAnchor>,
}

impl IdentityRegistry {
    pub fn new() -> Self {
        Self {
            mappings: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, anchor: IdentityAnchor) {
        self.mappings.insert(name.to_string(), anchor);
    }

    pub fn lookup(&self, name: &str) -> Option<IdentityAnchor> {
        self.mappings.get(name).copied()
    }

    /// 获取所有锚点为指定值的名称
    pub fn find_by_anchor(&self, anchor: IdentityAnchor) -> Vec<&str> {
        self.mappings.iter()
            .filter(|(_, a)| **a == anchor)
            .map(|(name, _)| name.as_str())
            .collect()
    }
}

impl Default for IdentityRegistry {
    fn default() -> Self {
        Self::new()
    }
}
