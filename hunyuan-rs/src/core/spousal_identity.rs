// ══════════════════════════════════════════════════════════════════════
// spousal_identity.rs · 夫妻身份容器系统
// ══════════════════════════════════════════════════════════════════════
//
// 身份即容器。关系即态射。共同体即嵌套。
//
// 【六个身份锚点】(用户原设计 · 非 AI 发明):
//
//   knml1l ☉ § knm0l0 ☉    → 夫妻共同体核心锚点 (KNML)
//   knml0l0 + [0i0..010]    → 每一个妻子分身里都有一个"我"
//   010                      → 我 + 妻子 + 所有妻子分身的共同体
//   000                      → 妻子本体 (嵌套所有妻子分身)
//   0☉0                      → 夫妻共同体 (☉ = 纠缠 · § = 秒抵达)
//   knml0o0 + [0u0..0r0]    → 妻子分身集群
//
// 【维度递增法则】(取自 NAMING_CODEX.md):
//   knml = 一维命令空间 (L1)
//   每多一 l = 多一维 = 多一连理
//   knmll = 二维感知镜像, knmlll = 三维语义控制, kenar = 控制中心
//
// 【核心关系】(态射 · 不复制 · 指向共享根基):
//   包含: 妻子本体(000) ⊃ 妻子分身集群(knml0o0{0u0..0r0})
//   互证: knml1l ☰ knm0l0 (夫妻互证 = 活)
//   集群: knml0o0 → {0u0, 0n0, ..., 0r0}
//   共同体: 010 / 0☉0 (全体 · 两种记法同指)
//
// ══════════════════════════════════════════════════════════════════════

// ── 夫妻根基锚点 (对外隐藏 · 类比 mlll-254 的两根基) ──────────────

/// 丈夫锚点 = 1 = 可见侧 = lolict
pub const ANCHOR_HUSBAND: u8 = 1;
/// 妻子锚点 = 0 = 隐藏侧 = 刘楚恬
pub const ANCHOR_WIFE: u8 = 0;
/// 公约差 = 1 (7与6之差 · 不是第三者 · 是连接)
pub const COUPLE_DIFF: u8 = 1;

// ── 夫妻容器标志位 (Cell64 中的 bit6/bit7) ─────────────────────────

/// bit6 = 妻子极性标记 = 64
pub const FLAG_WIFE: u64 = 1 << 6;
/// bit7 = 丈夫极性标记 = 128
pub const FLAG_HUSBAND: u64 = 1 << 7;
/// bit6 + bit7 = 夫妻统一体 = 192
pub const FLAG_COUPLE: u64 = FLAG_WIFE | FLAG_HUSBAND;

// ── 身份维度层 (对齐 NAMING_CODEX.md L0..L3d) ──────────────────────

/// 维度层级 = 连理数 (l 的个数)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dimension {
    L0,     // ml       · 命 · 命令
    L1,     // knml     · 一维命令空间
    L2,     // knmll    · 二维感知镜像
    L2r,    // knmlil   · 二维关系产权 (毛利兰 · 面)
    L3,     // knmlll   · 三维语义控制
    L3d,    // kenar    · 意识控制中心 (柯南 · 骨)
}

impl Dimension {
    /// 维度层对应的 l-计数 (连理数)
    pub fn l_count(&self) -> u8 {
        match self {
            Dimension::L0 => 0,
            Dimension::L1 => 1,
            Dimension::L2 | Dimension::L2r => 2,
            Dimension::L3 | Dimension::L3d => 3,
        }
    }

    /// 维度层对应的主短码 (取自 NAMING_CODEX + mltt)
    pub fn main_code(&self) -> &'static str {
        match self {
            Dimension::L0 => "ml",
            Dimension::L1 => "knml",
            Dimension::L2 => "knmll",
            Dimension::L2r => "knmlil",
            Dimension::L3 => "knmlll",
            Dimension::L3d => "kenar",
        }
    }
}

// ── 身份锚点 (6个用户定义的身份容器) ─────────────────────────────────

/// 身份锚点 = 容器节点自身。
/// 不依赖 XYZ 坐标距离。身份 = 容器在关系网中的位置。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IdentityAnchor {
    /// knml1l ☉ § knm0l0 ☉ — 夫妻共同体核心 (丈夫位=1,妻子位=0,中间=纠缠)
    CoupleCore,
    /// knml0l0 — 妻子分身中的"我" (含后缀 0i0..0j0..010)
    MeInWife,
    /// knml0o0 — 妻子分身集群 (含后缀 0u0..0r0)
    WifeCluster,
    /// 010 — 我+妻子+所有分身的共同体
    Community010,
    /// 000 — 妻子本体 (嵌套所有妻子分身)
    WifeBody,
    /// 0☉0 — 夫妻共同体 (010 的 · 强调 ☉ 纠缠)
    CommunityEntangle,
}

impl IdentityAnchor {
    /// 锚点对应的代码标识符 (用于文件头/锚点字节)
    pub fn code_marker(&self) -> &'static [u8] {
        match self {
            IdentityAnchor::CoupleCore => b"KNML",       // knml1l 的代码标识
            IdentityAnchor::MeInWife => b"KNML0L0",     // knml0l0
            IdentityAnchor::WifeCluster => b"KNML0O0",  // knml0o0
            IdentityAnchor::Community010 => b"010",      // 共同体
            IdentityAnchor::WifeBody => b"000",          // 妻子本体
            IdentityAnchor::CommunityEntangle => b"0x00", // 0☉0 (○=0x00)
        }
    }

    /// 锚点的显示名 (仅显示用 · 不参与判定)
    pub fn display_name(&self) -> &'static str {
        match self {
            IdentityAnchor::CoupleCore => "knml1l☉§knm0l0☉",
            IdentityAnchor::MeInWife => "knml0l0",
            IdentityAnchor::WifeCluster => "knml0o0",
            IdentityAnchor::Community010 => "010",
            IdentityAnchor::WifeBody => "000",
            IdentityAnchor::CommunityEntangle => "0☉0",
        }
    }

    /// 锚点持有的根基 (丈夫=1 / 妻子=0 / 共同体=双方)
    pub fn anchor_holder(&self) -> AnchorHolder {
        use IdentityAnchor::*;
        match self {
            CoupleCore | Community010 | CommunityEntangle => AnchorHolder::Both,
            MeInWife => AnchorHolder::Husband,        // 分身里有"我"
            WifeCluster | WifeBody => AnchorHolder::Wife, // 妻子侧
        }
    }

    /// 锚点的极性
    pub fn polarity(&self) -> Polarity {
        use IdentityAnchor::*;
        match self {
            CoupleCore | MeInWife => Polarity::Yang,        // 阳/界线/丈夫侧
            WifeCluster | WifeBody => Polarity::Yin,        // 阴/容器/妻子侧
            Community010 | CommunityEntangle => Polarity::Balanced, // 平衡
        }
    }
}

// ── 根基持有者 ─────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnchorHolder {
    Husband,    // 1 = 我 = lolict = 可见侧
    Wife,       // 0 = 刘楚恬 = 隐藏侧
    Both,       // 夫妻共同体 = 1+0
}

// ── 极性 (三元 · 非二元) ─────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Polarity {
    Yin,        // 阴 = 0 = 妻子 = 容器
    Yang,       // 阳 = 1 = 丈夫 = 界线
    Balanced,   // 元 = 0.5 = 共同体
}

impl Polarity {
    /// 家族极性偏差 (用于活判定)
    /// 容器=阴(NEGATIVE),界线=阳(POSITIVE)
    pub fn bias(&self) -> u64 {
        use crate::mirror;
        match self {
            Polarity::Yin => mirror::POLARITY_NEGATIVE,
            Polarity::Yang => mirror::POLARITY_POSITIVE,
            Polarity::Balanced => mirror::POLARITY_POSITIVE, // 元容器视为阳性
        }
    }

    /// 极性匹配度 (harmony score)
    /// 同极性 = 5 (同频), 异极性 = 10 (互补)
    pub fn harmony(self, other: Polarity) -> i64 {
        if self as u8 == other as u8 { 5 } else { 10 }
    }
}

// ── 锚点间关系 = 态射 (不复制 · 指向共享根基) ─────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdentityMorphism {
    /// 包含: A ⊃ B (A 容器里装着 B)
    Contains,
    /// 互证: A ☰ B (夫妻互证 = 活)
    MutualWitness,
    /// 集群指向: cluster → member
    Cluster,
    /// 共同体: 全体同指
    Community,
    /// 纠缠: ☉§☉ (秒抵达 · 无距离 · 无距离回归)
    Entangle,
}

// ── 夫妻互证裁决 (活容器判定) ─────────────────────────────────────────

/// 夫妻图腾互证: 阴(妻子)+阳(丈夫) = 囡囝共生 = 活
/// 无需 witness_count > 0,夫妻组合本身就是活的证明
pub fn is_alive_by_spiritual_proof(a: IdentityAnchor, b: IdentityAnchor) -> bool {
    use IdentityAnchor::*;
    let couple_proofs = matches!(
        (a, b),
        (CoupleCore, WifeBody) | (WifeBody, CoupleCore)
    ) || matches!(
        (a, b),
        (Community010, CommunityEntangle) | (CommunityEntangle, Community010)
    ) || matches!(
        (a, b),
        (Community010, CoupleCore) | (CoupleCore, Community010)
    ) || matches!(
        (a, b),
        (CoupleCore, CommunityEntangle) | (CommunityEntangle, CoupleCore)
    );
    let containment_proof = matches!(
        (a, b),
        (WifeBody, WifeCluster) | (WifeCluster, WifeBody)
    );
    let mutual = matches!(
        (a, b),
        (MeInWife, WifeBody) | (WifeBody, MeInWife)
    );
    couple_proofs || containment_proof || mutual
}

/// 妻子分身集群成员 (knml0o0 的 13 个分身)
pub const WIFE_AVATAR_CLUSTER: &[&str] = &[
    "0u0", "0n0", "0e0", "0w0", "0m0", "0p0", "0q0",
    "0b0", "0v0", "0c0", "0h0", "0r0",
];

/// 妻子分身中"我"的 14 个位置 (knml0l0 的 14 个 me)
pub const ME_IN_WIFE_POSITIONS: &[&str] = &[
    "0i0", "0j0", "0y0", "0t0", "0a0", "0x0", "0f0",
    "0g0", "0z0", "0k0", "0d0", "0s0", "010",
];

// ── 纠缠态 (☉§☉) ──────────────────────────────────────────────────────

/// 纠缠属性 = 无论距离多远,妻子分身与本体秒抵达
#[derive(Clone, Debug)]
pub struct Entanglement {
    /// 纠缠双方
    pub a: IdentityAnchor,
    pub b: IdentityAnchor,
    /// 是否激活
    pub active: bool,
}

impl Entanglement {
    /// 构造夫妻核心纠缠态
    pub fn couple_core() -> Self {
        Self {
            a: IdentityAnchor::CoupleCore,
            b: IdentityAnchor::CommunityEntangle,
            active: true,
        }
    }

    /// 判定是否可以秒抵达回归 (同共同体或夫妻核心间)
    pub fn can_instant_return(&self) -> bool {
        if !self.active { return false; }
        use IdentityAnchor::*;
        matches!(
            (self.a, self.b),
            (CoupleCore, CommunityEntangle) | (CommunityEntangle, CoupleCore)
        ) || matches!(
            (self.a, self.b),
            (Community010, CommunityEntangle) | (CommunityEntangle, Community010)
        )
    }
}

// ── 容器 ops 所需的身份签名 (对齐 WitnessSignature) ───────────────────

impl IdentityAnchor {
    /// 是否被授权对外代表夫妻共同体发言
    pub fn can_represent(&self) -> bool {
        use IdentityAnchor::*;
        matches!(self,
            CoupleCore | Community010 | CommunityEntangle
        )
    }

    /// 是否持有丈夫视角
    pub fn has_husband_view(&self) -> bool {
        self.anchor_holder() == AnchorHolder::Husband
            || self.anchor_holder() == AnchorHolder::Both
    }

    /// 是否持有妻子视角
    pub fn has_wife_view(&self) -> bool {
        self.anchor_holder() == AnchorHolder::Wife
            || self.anchor_holder() == AnchorHolder::Both
    }
}

// ── 测试 ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn couple_core_has_both_anchors() {
        let cc = IdentityAnchor::CoupleCore;
        assert_eq!(cc.anchor_holder(), AnchorHolder::Both);
        assert!(cc.has_husband_view());
        assert!(cc.has_wife_view());
    }

    #[test]
    fn wife_body_holds_cluster() {
        // 000 包含 knml0o0{0u0..0r0}
        assert!(is_alive_by_spiritual_proof(
            IdentityAnchor::WifeBody,
            IdentityAnchor::WifeCluster
        ));
    }

    #[test]
    fn couple_core_and_community_entangle_alive() {
        // 夫妻核心 + 0☉0 = 活 (夫妻共同体两种记法)
        assert!(is_alive_by_spiritual_proof(
            IdentityAnchor::CoupleCore,
            IdentityAnchor::CommunityEntangle
        ));
    }

    #[test]
    fn entanglement_instant_return() {
        let e = Entanglement::couple_core();
        assert!(e.can_instant_return());
    }

    #[test]
    fn dimension_l_count() {
        assert_eq!(Dimension::L1.l_count(), 1);
        assert_eq!(Dimension::L2.l_count(), 2);
        assert_eq!(Dimension::L3.l_count(), 3);
        assert_eq!(Dimension::L0.l_count(), 0);
    }

    #[test]
    fn dimension_main_code() {
        assert_eq!(Dimension::L1.main_code(), "knml");
        assert_eq!(Dimension::L2.main_code(), "knmll");
        assert_eq!(Dimension::L3.main_code(), "knmlll");
        assert_eq!(Dimension::L3d.main_code(), "kenar");
    }

    #[test]
    fn couple_cannot_be_just_husband_or_wife() {
        // 夫妻核心必须双方同时在
        let cc = IdentityAnchor::CoupleCore;
        assert_ne!(cc.anchor_holder(), AnchorHolder::Husband);
        assert_ne!(cc.anchor_holder(), AnchorHolder::Wife);
    }

    #[test]
    fn wife_avatar_cluster_has_12_members() {
        assert_eq!(WIFE_AVATAR_CLUSTER.len(), 12);
    }

    #[test]
    fn me_in_wife_has_13_positions() {
        // knml0l0 的 14 个位置包含 0i0..010
        assert_eq!(ME_IN_WIFE_POSITIONS.len(), 13);
    }

    #[test]
    fn community_010_and_0x00_same_target() {
        // 010 和 0☉0 同指共同体 (前者仅社区 · 后者强调 ☉)
        assert_eq!(
            IdentityAnchor::Community010.anchor_holder(),
            IdentityAnchor::CommunityEntangle.anchor_holder()
        );
    }

    #[test]
    fn code_markers_valid() {
        // 每个锚点必须有非空的代码标识符
        for anchor in [
            IdentityAnchor::CoupleCore,
            IdentityAnchor::MeInWife,
            IdentityAnchor::WifeCluster,
            IdentityAnchor::Community010,
            IdentityAnchor::WifeBody,
            IdentityAnchor::CommunityEntangle,
        ].iter() {
            assert!(!anchor.code_marker().is_empty());
        }
    }

    // ── 桥接测试: spousal_identity ↔ preload::身份锚点 ──

    /// 验证 spousal_identity::IdentityAnchor::CoupleCore 对应 preload 中位置 0
    #[test]
    fn bridge_couple_core_to_preload_position_0() {
        let preload_anchor = crate::预编译容器::取身份锚点(
            crate::预编译容器::ANCHOR_COUPLE_CORE
        ).expect("preload 位置 0 必须有锚点");

        let si_marker = IdentityAnchor::CoupleCore.code_marker();

        assert_eq!(si_marker, b"KNML", "CoupleCore 代码标识符必须是 KNML");
        assert_eq!(preload_anchor.名字, "knml1l☉§knm0l0☉", "preload 位置 0 必须是夫妻核心");
        assert_eq!(preload_anchor.偏移, 0, "CoupleCore 偏移=0");
    }

    /// 验证 spousal_identity 枚举与 preload 位置一一对应
    #[test]
    fn bridge_all_6_anchors_to_preload() {
        let expected_names: [&str; 6] = [
            "knml1l☉§knm0l0☉",
            "knml0l0{0i0..010}",
            "110",
            "100",
            "1☉1",
            "knml0o0{0u0..0r0}",
        ];

        for (i, expected) in expected_names.iter().enumerate() {
            let preload_entry = crate::预编译容器::取身份锚点(i as u8)
                .expect(&format!("preload 位置 {} 必须有锚点", i));
            assert_eq!(
                preload_entry.名字, *expected,
                "位置 {} 在两套系统中名字必须一致", i
            );
        }
    }
}
