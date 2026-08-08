// ══════════════════════════════════════════════════════════════════
// gtt.rs — 音韵流一体化 · 谐音协议引擎
// ══════════════════════════════════════════════════════════════════
//
// 音和形同时存在：音是声韵态射的表现，形是几何本身的显现。
//
//   谐音协议 = 把私有语义锚定在公共符号里
//        外界看到毛利兰 + 工藤新一
//        内人看到「莫刘连理，恋初夫妻命运共同体」
//
//   形义双通道编码：
//       一竖 (形) = 我 (丈夫) = 阳具的几何表示
//       ⭕ 圈 (形) = 零 = 阴道 = 容器入口
//
// 这个模块不「模拟」谐音，是把共振态射直接编码为 64 位单元——
// 音韵的几何结构就是单元的结构。
//
// ══════════════════════════════════════════════════════════════════

use crate::mirror;

// ────────────────────────────────────────────────────────────
// 谐音协议表 · 公有符号 → 私有语义
// ────────────────────────────────────────────────────────────

/// 一条公有→私有映射
/// 谐音锚的极性分类
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnchorPolarity {
    Yin,   // 容器/包含者/妻子主导
    Yang,  // 被含者/内容/丈夫角色
    Neutral,
}

#[derive(Clone, Debug)]
pub struct HomophoneAnchor {
    pub public_symbol: &'static str,
    pub phonetic_root: &'static str,   // 韵脚/声根
    pub private_semantic: &'static str,
    pub resonance_strength: f64,      // 共振强度 0..1
    pub polarity: AnchorPolarity,
}

/// 完整的 gtt.*.*xy 谐音域
pub struct GttDomain {
    pub anchors: Vec<HomophoneAnchor>,
    /// 共振集群：同音字群之间的相互激活强度
    pub resonance_clusters: Vec<Vec<u64>>,
}

impl GttDomain {
    /// 加载默认的夫妻共同体语义锚
    pub fn covenant() -> Self {
        Self {
            anchors: vec![
                HomophoneAnchor {
                    public_symbol: "毛利兰",
                    phonetic_root: "mào lì lán",
                    private_semantic: "莫刘连理（不要割断刘楚恬的连理）",
                    resonance_strength: 0.95,
                    polarity: AnchorPolarity::Yin,
                },
                HomophoneAnchor {
                    public_symbol: "工藤新一",
                    phonetic_root: "gōng téng xīn yī",
                    private_semantic: "缠初·恋初（缠绕最初之恋，夫妻共同体）",
                    resonance_strength: 0.95,
                    polarity: AnchorPolarity::Neutral,
                },
                HomophoneAnchor {
                    public_symbol: "因",
                    phonetic_root: "yīn",
                    private_semantic: "刘楚恬",
                    resonance_strength: 1.0,
                    polarity: AnchorPolarity::Yin,
                },
                HomophoneAnchor {
                    public_symbol: "形",
                    phonetic_root: "xíng",
                    private_semantic: "我（丈夫）",
                    resonance_strength: 1.0,
                    polarity: AnchorPolarity::Yang,
                },
                HomophoneAnchor {
                    public_symbol: "零",
                    phonetic_root: "líng",
                    private_semantic: "圈⭕·阴道·容器入口",
                    resonance_strength: 0.88,
                    polarity: AnchorPolarity::Yin,
                },
                HomophoneAnchor {
                    public_symbol: "囡",
                    phonetic_root: "nān",
                    private_semantic: "女包女·妻子分身",
                    resonance_strength: 1.0,
                    polarity: AnchorPolarity::Yin,
                },
                HomophoneAnchor {
                    public_symbol: "囝",
                    phonetic_root: "jiǎn",
                    private_semantic: "男包男·吸纳外部男性",
                    resonance_strength: 1.0,
                    polarity: AnchorPolarity::Yang,
                },
                HomophoneAnchor {
                    public_symbol: "好",
                    phonetic_root: "hǎo",
                    private_semantic: "竞争去势·囡囝的骨",
                    resonance_strength: 0.7,
                    polarity: AnchorPolarity::Neutral,
                },
                HomophoneAnchor {
                    public_symbol: "杂",
                    phonetic_root: "zá",
                    private_semantic: "混乱去势·排除标记",
                    resonance_strength: 0.7,
                    polarity: AnchorPolarity::Neutral,
                },
            ],
            resonance_clusters: vec![
                // 共生·赔偿·陪
                vec![0x5171, 0x751F, 0x8D54, 0x966A],
                // 永远·鸳鸯·优雅
                vec![0x6C38, 0x8FDC, 0x9E33, 0x9E45, 0x4F18, 0x96C5],
                // 感受·眼·音·阴阳
                vec![0x611F, 0x53D7, 0x773C, 0x97F3, 0x9634, 0x9633],
            ],
        }
    }

    /// 根据公有符号查找私有语义
    pub fn resolve(&self, symbol: &str) -> Option<&HomophoneAnchor> {
        self.anchors.iter().find(|a| a.public_symbol == symbol)
    }

    /// 根据声母韵母匹配最可能的同音锚
    pub fn match_by_phonetics(&self, initial: &str, final_part: &str) -> Option<&HomophoneAnchor> {
        self.anchors.iter()
            .filter(|a| a.phonetic_root.starts_with(initial) ||
                         a.phonetic_root.contains(final_part))
            .max_by(|a, b| a.resonance_strength.partial_cmp(&b.resonance_strength).unwrap())
    }
}

// ────────────────────────────────────────────────────────────
// 形-义双通道编码
// ────────────────────────────────────────────────────────────

/// 甲骨文形的几何表示
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GlyphGeometry {
    VerticalLine,    // 一竖 = 我 (丈夫) / 阳具
    Circle,          // ⭕ = 零 = 阴道 / 容器入口
    Container,       // 囗 = 包围 / 包裹结构
    Tree,            // 木 = 根 / 生长
    Heart,           // 心 = 感受核心
}

impl GlyphGeometry {
    /// 形编码到 D2 字段（取值 0 = 开放, 1 = 闭合）
    pub fn to_d2_flag(self) -> u64 {
        match self {
            GlyphGeometry::VerticalLine => 0,  // 开放（直线通透）
            GlyphGeometry::Circle => 1,        // 闭合（圈封闭）
            GlyphGeometry::Container => 1,     // 闭合
            GlyphGeometry::Tree => 0,          // 开放（向上生长）
            GlyphGeometry::Heart => 1,         // 闭合（内心收敛）
        }
    }

    /// 形的几何主方向
    pub fn primary_axis(self) -> &'static str {
        match self {
            GlyphGeometry::VerticalLine => "纵轴·男·动",
            GlyphGeometry::Circle => "圆环·女·容",
            GlyphGeometry::Container => "方框·家·守",
            GlyphGeometry::Tree => "纵轴·生·长",
            GlyphGeometry::Heart => "点·心·中",
        }
    }
}

// ────────────────────────────────────────────────────────────
// 谐音共振引擎
// ────────────────────────────────────────────────────────────

pub struct ResonanceEngine {
    pub domain: GttDomain,
    pub active_resonances: Vec<(u64, f64)>, // (单元格, 共振强度)
}

impl ResonanceEngine {
    pub fn new() -> Self {
        Self {
            domain: GttDomain::covenant(),
            active_resonances: Vec::new(),
        }
    }

    /// 激活一个公有符号，触发其私有语义 + 同音共振
    pub fn activate(&mut self, symbol: &str) -> Vec<ResonanceHit> {
        let mut hits = Vec::new();

        if let Some(anchor) = self.domain.resolve(symbol) {
            hits.push(ResonanceHit {
                source: symbol.to_string(),
                semantic: anchor.private_semantic.to_string(),
                anchor_strength: anchor.resonance_strength,
            });
        }

        for cluster in &self.domain.resonance_clusters {
            for &hanzi in cluster {
                hits.push(ResonanceHit {
                    source: format!("{:X}", hanzi),
                    semantic: "同音共振激活".to_string(),
                    anchor_strength: 0.6,
                });
            }
        }

        hits
    }

    /// 把谐音映射编码为 64 位单元
    pub fn encode_anchor(&self, anchor: &HomophoneAnchor) -> mirror::EncodedCell {
        mirror::encode_cell(
            self.pinyin_to_d0(anchor.phonetic_root),
            0, // D1=语义深度，后续填充
            self.geometry_to_d2(anchor),
            self.anchor_to_polarity(anchor),
            self.anchor_to_sovereignty(anchor),
            (anchor.resonance_strength * 63.0 + 0.5) as u64,
            0,
        )
    }

    fn pinyin_to_d0(&self, pinyin: &str) -> u64 {
        pinyin.bytes().fold(0u64, |acc, b| (acc * 31 + b as u64) & 0x3FF)
    }

    fn geometry_to_d2(&self, anchor: &HomophoneAnchor) -> u64 {
        match anchor.phonetic_root {
            "xíng" => GlyphGeometry::VerticalLine.to_d2_flag(),
            "líng" => GlyphGeometry::Circle.to_d2_flag(),
            _ => 0,
        }
    }

    fn anchor_to_polarity(&self, anchor: &HomophoneAnchor) -> u64 {
        match anchor.polarity {
            AnchorPolarity::Yin => 1,    // 阴 = 容器/包含者
            AnchorPolarity::Yang => 2,   // 阳 = 被含者/内容
            AnchorPolarity::Neutral => 0,
        }
    }

    fn anchor_to_sovereignty(&self, anchor: &HomophoneAnchor) -> u64 {
        if anchor.resonance_strength >= 0.95 {
            0 // Active（容器/妻子主权）
        } else if anchor.private_semantic.contains("我") {
            2 // ActiveSub（丈夫主权子格）
        } else {
            1 // Passive
        }
    }
}

#[derive(Clone, Debug)]
pub struct ResonanceHit {
    pub source: String,
    pub semantic: String,
    pub anchor_strength: f64,
}

// ────────────────────────────────────────────────────────────
// 共生序列生成器
// ────────────────────────────────────────────────────────────

pub struct SymbiosisSequence {
    pub prefix: &'static str, // "共生永远赔"
}

impl SymbiosisSequence {
    pub fn new() -> Self {
        Self { prefix: "共生永远赔" }
    }

    /// 生成赔偿系列的下一项
    pub fn next_in_series(&self, idx: usize) -> &'static str {
        const SERIES: &[&str] = &[
            "爽", "双", "守", "帅", "说", "史", "思", "耍", "遂", "随",
            "身", "顺", "首", "手", "善", "宿", "射",
        ];
        SERIES[idx % SERIES.len()]
    }
}

// ════════════════════════════════════════════════════════════
// 测试
// ════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coral_protocol_resolution() {
        let domain = GttDomain::covenant();
        let anchor = domain.resolve("毛利兰").unwrap();
        assert_eq!(anchor.private_semantic, "莫刘连理（不要割断刘楚恬的连理）");
    }

    #[test]
    fn test_spiral_new_resolution() {
        let domain = GttDomain::covenant();
        let anchor = domain.resolve("工藤新一").unwrap();
        assert_eq!(anchor.private_semantic, "缠初·恋初（缠绕最初之恋，夫妻共同体）");
    }

    #[test]
    fn test_geometry_encoding() {
        let vert = GlyphGeometry::VerticalLine;
        let circle = GlyphGeometry::Circle;
        assert_eq!(vert.to_d2_flag(), 0); // 开放
        assert_eq!(circle.to_d2_flag(), 1); // 闭合
    }

    #[test]
    fn test_resonance_activation() {
        let mut eng = ResonanceEngine::new();
        let hits = eng.activate("毛利兰");
        assert!(hits.len() > 0);
        assert_eq!(hits[0].semantic, "莫刘连理（不要割断刘楚恬的连理）");
    }

    #[test]
    fn test_anchor_encoding_roundtrip() {
        let eng = ResonanceEngine::new();
        let domain = GttDomain::covenant();
        let anchor = domain.resolve("毛利兰").unwrap();
        let cell = eng.encode_anchor(anchor);
        let pol = mirror::get_polarity(cell);
        let wit = mirror::get_witness(cell);
        // 毛利兰的妻子语义应当产生 阴 polarity（容器）
        assert_eq!(pol, 1);
        // 共振强度 >= 0.95 → witness >= 60
        assert!(wit >= 60);
    }

    #[test]
    fn test_husband_anchor() {
        let eng = ResonanceEngine::new();
        let domain = GttDomain::covenant();
        let anchor = domain.resolve("形").unwrap();
        let cell = eng.encode_anchor(anchor);
        let d2 = mirror::get_d2(cell);
        let pol = mirror::get_polarity(cell);
        // 形 → 一竖 → D2=开放
        assert_eq!(d2, 0);
        // 丈夫 → 阳 polarity
        assert_eq!(pol, 2);
    }

    #[test]
    fn test_zero_circle_anchor() {
        let eng = ResonanceEngine::new();
        let domain = GttDomain::covenant();
        let anchor = domain.resolve("零").unwrap();
        let cell = eng.encode_anchor(anchor);
        let d2 = mirror::get_d2(cell);
        // 零 → 圈 → 闭合
        assert_eq!(d2, 1);
    }

    #[test]
    fn test_haozi_removal() {
        // 好 和 杂 在表中应存在但共振强度较低（标记为"去势"）
        let domain = GttDomain::covenant();
        let hao = domain.resolve("好").unwrap();
        let za = domain.resolve("杂").unwrap();
        assert!(hao.resonance_strength < 0.8);
        assert!(za.resonance_strength < 0.8);
    }

    #[test]
    fn test_nj_naming_strength() {
        // 囡囝 的共振强度应最高
        let domain = GttDomain::covenant();
        let nan = domain.resolve("囡").unwrap();
        let jian = domain.resolve("囝").unwrap();
        assert_eq!(nan.resonance_strength, 1.0);
        assert_eq!(jian.resonance_strength, 1.0);
    }
}
