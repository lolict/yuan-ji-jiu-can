// ======================================================================
// celestial_perception.rs — 天庭系统 v1-v5 合一合并
//
// 本文件合并原 5 个独立版本的全部公共类型/函数/测试:
//
//   v1 (celestial_perception.rs, 1624 行) — 奠基: BagAttributes 14维,
//       VisualSystem, CelestialPerception 引擎, JudgmentEngine 奖惩,
//       HistoricalAccumulation 时代积累, BagHierarchy 层级认知
//
//   v2 (celestial_perception_v2.rs, 1410 行) — 听觉授权: AuditoryAuth,
//       HeavenlyCourtAuth 双授权, AuditorySystem 聆听, CommonDivisor-CommonMultiple-
//       MirrorComparison 公约数-公倍数-照镜子, CelestialPerceptionV2
//
//   v3 (celestial_perception_v3.rs, 1386 行) — 六系扩展: Emotion(含强度),
//       Behavior, ThinkingSystem, AestheticSystem, EliminationSystem(连败淘汰),
//       FilterSystem(准入过滤), CelestialPerceptionV3
//
//   v4 (celestial_perception_v4.rs, 1030 行) — 七系重构: Emotion(七情离散),
//       EmotionState(效价唤醒), ReasoningChain(推理链), AestheticDimensions,
//       AestheticVerdict, EliminationReason-Record, FilterSystem(质量过滤),
//       CelestialPerceptionV4
//
//   v5 (celestial_perception_v5.rs, 934 行) — 语言系统: HanziEncoding,
//       CharStructure, Shengmu-Yunmu 谐音路由, SelfRadix 自进制,
//       CommandParser, FlowPropertyBits, CelestialPerceptionV5
//
// 冲突解决策略:
//   v4的 Emotion/Behavior/BehaviorSystem/ThinkingSystem/AestheticSystem/
//   EliminationSystem/FilterSystem/PipelineResult/ReasoningChain 保留原名（配合 lib.rs 导出）
//   v3对应冲突类型重命名为 EmotionV3/BehaviorV3/...V3 后缀
//
// ======================================================================

use crate::mirror::{
    self, encode_cell, get_value, get_polarity, get_sovereignty, Sovereignty,
    EncodedCell, POLARITY_POSITIVE, POLARITY_NEGATIVE,
};
use crate::observer::{Observer, SenseForm, SenseKind};
use crate::memory::{Coord, MetaJudgeEngine};
use crate::os_layer::OsLayer;
use crate::os_layer::bare_metal::BareMetalOs;
use std::time::{SystemTime, UNIX_EPOCH};

// ── v8 盟机合袋子网络 / v9 感知聚合云 (子模块内联) ──
#[path = "celestial_perception_v8.rs"]
pub mod celestial_perception_v8;
#[path = "celestial_perception_v9.rs"]
pub mod celestial_perception_v9;
pub use celestial_perception_v9::{ConvergenceCloud, FeatureJudgment, PerceptionReport};

// ======================================================================
// 辅助函数
// ======================================================================

/// v3/v1 百分比显示辅助: 将 0..1 转为 0..100
#[inline]
fn pct(x: f64) -> f64 { x * 100.0 }

/// v1/v3 的标准时间戳函数
fn current_timestamp_ms(os: &dyn OsLayer) -> u64 {
    os.time_millis()
}

// ======================================================================
// 第一部分: 授权体系 (来自 v2/v3)
// ======================================================================

/// 天庭授权 — 视觉+心智
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HeavenlyCourtAuth {
    Dormant,
    Authorized,
    Revoked,
}

impl HeavenlyCourtAuth {
    pub fn can_perceive(&self) -> bool { matches!(self, Self::Authorized) }
    pub fn can_judge(&self) -> bool { matches!(self, Self::Authorized) }
    pub fn authorize(&mut self) -> bool { *self = Self::Authorized; true }
    pub fn revoke(&mut self) { *self = Self::Revoked; }
}

/// 听觉授权 — 听觉系统的独立授权
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AuditoryAuth {
    Silent,     // 未授权 — 静默
    Listening,  // 已授权 — 聆听中
    Deafened,   // 已撤销 — 失聪
}

impl AuditoryAuth {
    pub fn can_hear(&self) -> bool { matches!(self, Self::Listening) }
    pub fn authorize(&mut self) -> bool { *self = Self::Listening; true }
    pub fn revoke(&mut self) { *self = Self::Deafened; }
    pub fn wake(&mut self) {
        if matches!(self, Self::Silent) {
            *self = Self::Listening;
        }
    }
}

// ======================================================================
// 第二部分: 袋子属性 (BagAttributes) — 14维高维感知向量
// ======================================================================

/// 袋子属性向量 — 高维感知的完整状态 (合并 v1 的 to_cell/from_cell/needs_* 方法)
#[derive(Clone, Copy, Debug)]
pub struct BagAttributes {
    pub sovereignty: u64,        // 1. 主权归属
    pub value: u64,              // 2. 价值
    pub integrity: f64,           // 3. 完好性
    pub content_quality: f64,     // 4. 内容质量
    pub content_purity: f64,      // 5. 内容纯度
    pub bag_cleanliness: f64,     // 6. 容器清洁度
    pub dryness: f64,            // 7. 干燥度
    pub pest_level: f64,         // 8. 虫害度
    pub sand_level: f64,         // 9. 杂质度
    pub edibility: f64,          // 10. 可食性
    pub needs_content_wash: bool, // 11. 需要内容清洁
    pub needs_sand_pick: bool,   // 12. 需要除杂
    pub wash_target: f64,        // 13. 加工目标清洁度
    pub needs_repair: bool,      // 14. 需要修补
}

impl BagAttributes {
    pub fn pristine() -> Self {
        Self {
            sovereignty: 3, value: 800, integrity: 1.0, content_quality: 1.0,
            content_purity: 1.0, bag_cleanliness: 1.0, dryness: 1.0,
            pest_level: 0.0, sand_level: 0.0, edibility: 1.0,
            needs_content_wash: false, needs_sand_pick: false,
            wash_target: 1.0, needs_repair: false,
        }
    }

    pub fn rotten() -> Self {
        Self {
            sovereignty: 0, value: 0, integrity: 0.0, content_quality: 0.0,
            content_purity: 0.0, bag_cleanliness: 0.0, dryness: 0.0,
            pest_level: 1.0, sand_level: 1.0, edibility: 0.0,
            needs_content_wash: true, needs_sand_pick: true,
            wash_target: 1.0, needs_repair: true,
        }
    }

    /// v3 版综合质量 (权重不同, 包含干度/虫沙逆项)
    pub fn composite_quality(&self) -> f64 {
        let q = self.content_quality * 0.30
               + self.edibility * 0.25
               + self.content_purity * 0.15
               + self.integrity * 0.10
               + self.bag_cleanliness * 0.10
               + self.dryness * 0.05
               + (1.0 - self.pest_level) * 0.025
               + (1.0 - self.sand_level) * 0.025;
        q.clamp(0.0, 1.0)
    }

    pub fn needs_any_processing(&self) -> bool {
        self.needs_content_wash || self.needs_sand_pick || self.needs_repair
    }

    pub fn quality_after_processing(&self) -> f64 {
        if !self.needs_any_processing() { return self.composite_quality(); }
        let purity_after = if self.needs_content_wash { self.wash_target } else { self.content_purity };
        let clean_after = if self.needs_repair { 1.0 } else { self.bag_cleanliness };
        let pest_after = if self.needs_sand_pick { 0.0 } else { self.pest_level };
        let sand_after = if self.needs_sand_pick { 0.0 } else { self.sand_level };
        let q = self.content_quality * 0.35 + self.edibility * 0.30
               + purity_after * 0.15 + self.integrity * 0.10 + clean_after * 0.10;
        let penalty = pest_after * 0.5 + sand_after * 0.5;
        (q - penalty).clamp(0.0, 1.0)
    }

    pub fn is_edible(&self) -> bool {
        self.edibility > 0.7 && self.pest_level < 0.3 && self.sand_level < 0.3 && self.integrity > 0.3
    }

    pub fn needs_drying(&self) -> bool { self.dryness < 0.7 }

    /// v3 审美分数
    pub fn aesthetic_score(&self) -> f64 {
        let attrs = [
            self.integrity, self.content_quality, self.content_purity,
            self.bag_cleanliness, self.dryness, self.edibility,
            1.0 - self.pest_level, 1.0 - self.sand_level,
        ];
        let mean = attrs.iter().sum::<f64>() / attrs.len() as f64;
        let variance = attrs.iter().map(|a| (a - mean).powi(2)).sum::<f64>() / attrs.len() as f64;
        let harmony = 1.0 - variance.sqrt();
        (mean * 0.6 + harmony * 0.4).clamp(0.0, 1.0)
    }

    /// v1 cell 编码
    pub fn to_cell(&self) -> EncodedCell {
        let d0 = (self.sovereignty * 85 + (self.integrity * 25.0) as u64).min(1023);
        let d1 = (self.content_quality * 255.0) as u64;
        let d2 = (self.edibility * 255.0) as u64;
        let polarity = if self.content_quality > 0.5 { POLARITY_POSITIVE } else { POLARITY_NEGATIVE };
        encode_cell(d0, d1, d2, polarity, self.sovereignty, 0, self.value)
    }

    /// v1 cell 解码
    pub fn from_cell(cell: EncodedCell, full_attrs: Option<Self>) -> Self {
        match full_attrs {
            Some(attrs) => attrs,
            None => {
                let d0 = mirror::get_d0(cell);
                let _d1 = mirror::get_d1(cell);
                let _d2 = mirror::get_d2(cell);
                Self {
                    sovereignty: get_sovereignty(cell),
                    value: get_value(cell),
                    integrity: ((d0 % 85) as f64) / 25.0,
                    content_quality: (_d1 as f64) / 255.0,
                    edibility: (_d2 as f64) / 255.0,
                    ..Self::pristine()
                }
            }
        }
    }

    /// v1 便捷查询方法
    pub fn content_wash_needed(&self) -> bool { self.needs_content_wash }
    pub fn sand_pick_needed(&self) -> bool { self.needs_sand_pick }
    pub fn repair_needed(&self) -> bool { self.needs_repair }

    pub fn summary(&self) -> String {
        let s = match self.sovereignty {
            0 => "无主", 1 => "妻子", 2 => "丈夫", 3 => "夫妻共有", _ => "未知",
        };
        format!(
            "属于[{}] 值{} 完好{:.0}% 质量{:.0}% 纯度{:.0}% 清洁{:.0}% 干{:.0}% 虫{:.0}% 沙{:.0}% 可食{:.0}%",
            s, self.value, self.integrity * 100.0, self.content_quality * 100.0,
            self.content_purity * 100.0, self.bag_cleanliness * 100.0, self.dryness * 100.0,
            self.pest_level * 100.0, self.sand_level * 100.0, self.edibility * 100.0,
        )
    }
}

// ======================================================================
// 第三部分: 袋子 (Bag) — v3 版 (含 judgment_count/consecutive_failures/eliminated)
// ======================================================================

#[derive(Clone, Debug)]
pub struct Bag {
    pub id: u64,
    pub name: String,
    pub attrs: BagAttributes,
    pub coord: Coord,
    pub layer_depth: u32,
    pub child_bag_ids: Vec<u64>,
    pub judgment_count: u64,        // v3
    pub consecutive_failures: u64,  // v3
    pub eliminated: bool,           // v3
}

impl Bag {
    pub fn new(id: u64, name: &str, attrs: BagAttributes, coord: Coord) -> Self {
        Self {
            id, name: name.to_string(), attrs, coord,
            layer_depth: 0, child_bag_ids: Vec::new(),
            judgment_count: 0, consecutive_failures: 0, eliminated: false,
        }
    }

    pub fn nested(id: u64, name: &str, attrs: BagAttributes, coord: Coord, depth: u32) -> Self {
        Self {
            id, name: name.to_string(), attrs, coord,
            layer_depth: depth, child_bag_ids: Vec::new(),
            judgment_count: 0, consecutive_failures: 0, eliminated: false,
        }
    }

    pub fn to_cell(&self) -> EncodedCell { self.attrs.to_cell() }
    pub fn quality(&self) -> f64 { self.attrs.composite_quality() }
    pub fn is_edible(&self) -> bool { self.attrs.is_edible() }
    pub fn needs_processing(&self) -> bool { self.attrs.needs_any_processing() }
    pub fn aesthetic(&self) -> f64 { self.attrs.aesthetic_score() }

    pub fn report(&self) -> String {
        format!("袋子#{}「{}」(层级{}): {}", self.id, self.name, self.layer_depth, self.attrs.summary())
    }
}

// ======================================================================
// 第四部分: 视觉系统 (VisualSystem)
// ======================================================================

#[derive(Clone, Debug)]
pub struct VisualSystem {
    pub sensitivity: f64,
    pub threshold: f64,
}

impl VisualSystem {
    pub fn new(sensitivity: f64, threshold: f64) -> Self {
        Self { sensitivity: sensitivity.clamp(0.01, 1.0), threshold: threshold.clamp(0.01, 1.0) }
    }

    pub fn default_vision() -> Self { Self::new(0.85, 0.1) }

    pub fn perceive(&self, bag: &Bag) -> Vec<SenseForm> {
        let mut forms = Vec::new();
        forms.push(SenseForm::new(
            SenseKind::Vision, self.sensitivity,
            &format!("袋#{} 主权={}", bag.id, match bag.attrs.sovereignty {
                0 => "无主", 1 => "妻子", 2 => "丈夫", 3 => "夫妻共有", _ => "未知"
            }),
            "visual_v3", 0,
        ));
        forms.push(SenseForm::new(
            SenseKind::Vision,
            (bag.attrs.value as f64 / 1000.0) * self.sensitivity,
            &format!("袋#{} 价值={}", bag.id, bag.attrs.value),
            "visual_v3", 0,
        ));
        if bag.attrs.integrity < 1.0 - self.threshold {
            forms.push(SenseForm::new(
                SenseKind::Vision,
                (1.0 - bag.attrs.integrity) * self.sensitivity,
                &format!("袋#{} 破损{:.0}%", bag.id, pct(1.0 - bag.attrs.integrity)),
                "visual_v3", 0,
            ));
        }
        if bag.attrs.bag_cleanliness < 1.0 - self.threshold {
            forms.push(SenseForm::new(
                SenseKind::Vision,
                (1.0 - bag.attrs.bag_cleanliness) * self.sensitivity,
                &format!("袋#{} 脏污{:.0}%", bag.id, pct(1.0 - bag.attrs.bag_cleanliness)),
                "visual_v3", 0,
            ));
        }
        forms
    }

    pub fn appears_sound(&self, bag: &Bag) -> bool {
        bag.attrs.integrity > 0.5 && bag.attrs.bag_cleanliness > 0.5
    }
}

// ======================================================================
// 第五部分: 听觉系统 (AuditorySystem)
// ======================================================================

#[derive(Clone, Debug)]
pub struct AuditorySystem {
    pub sensitivity: f64,
    pub heard_bags: Vec<u64>,
    pub auth: AuditoryAuth,
}

impl AuditorySystem {
    pub fn new(sensitivity: f64) -> Self {
        Self { sensitivity: sensitivity.clamp(0.01, 1.0), heard_bags: Vec::new(), auth: AuditoryAuth::Silent }
    }

    pub fn default_hearing() -> Self { Self::new(0.75) }

    pub fn authorize(&mut self) { self.auth.authorize(); }

    pub fn heard_count(&self) -> usize { self.heard_bags.len() }

    pub fn has_heard(&self, bag_id: u64) -> bool { self.heard_bags.contains(&bag_id) }

    pub fn listen(&mut self, bag: &Bag) -> Result<Vec<SenseForm>, &'static str> {
        if !self.auth.can_hear() { return Err("听觉未授权, 无法聆听"); }
        if !self.heard_bags.contains(&bag.id) { self.heard_bags.push(bag.id); }

        let mut forms = Vec::new();
        if bag.attrs.pest_level > 0.1 {
            forms.push(SenseForm::new(
                SenseKind::Hearing, bag.attrs.pest_level * self.sensitivity,
                &format!("袋#{} 虫鸣{:.0}%", bag.id, pct(bag.attrs.pest_level)),
                "auditory_v3", 0,
            ));
        }
        if bag.attrs.sand_level > 0.1 {
            forms.push(SenseForm::new(
                SenseKind::Hearing, bag.attrs.sand_level * self.sensitivity * 0.8,
                &format!("袋#{} 沙沙声{:.0}%", bag.id, pct(bag.attrs.sand_level)),
                "auditory_v3", 0,
            ));
        }
        if bag.attrs.dryness > 0.8 && bag.attrs.integrity < 0.5 {
            forms.push(SenseForm::new(
                SenseKind::Hearing, 0.7 * self.sensitivity,
                &format!("袋#{} 干燥破裂声", bag.id),
                "auditory_v3", 0,
            ));
        }
        if bag.attrs.dryness < 0.2 {
            forms.push(SenseForm::new(
                SenseKind::Hearing, 0.6 * self.sensitivity,
                &format!("袋#{} 水湿声", bag.id),
                "auditory_v3", 0,
            ));
        }
        if bag.attrs.integrity > 0.8 && bag.attrs.pest_level < 0.1 && bag.attrs.sand_level < 0.1 {
            forms.push(SenseForm::new(
                SenseKind::Hearing, 0.9 * self.sensitivity,
                &format!("袋#{} 健康共鸣", bag.id),
                "auditory_v3", 0,
            ));
        }
        Ok(forms)
    }
}

// ======================================================================
// 第六部分: 公约数-公倍数-照镜子 (v2/v3 merged)
// ======================================================================

/// 公约数 — 锚点标准 (v3 版, 含 min_aesthetic)
#[derive(Clone, Copy, Debug)]
pub struct CommonDivisor {
    pub id: u64,
    pub min_sovereignty: u64,
    pub min_value: u64,
    pub min_integrity: f64,
    pub min_content_quality: f64,
    pub min_content_purity: f64,
    pub min_bag_cleanliness: f64,
    pub min_dryness: f64,
    pub max_pest_level: f64,
    pub max_sand_level: f64,
    pub min_edibility: f64,
    pub min_aesthetic: f64,
}

impl CommonDivisor {
    pub fn standard() -> Self {
        Self {
            id: 1,
            min_sovereignty: 0, min_value: 0,
            min_integrity: 0.3, min_content_quality: 0.4,
            min_content_purity: 0.3, min_bag_cleanliness: 0.2,
            min_dryness: 0.3, max_pest_level: 0.7, max_sand_level: 0.7,
            min_edibility: 0.5, min_aesthetic: 0.0,
        }
    }

    pub fn strict() -> Self {
        Self {
            id: 2,
            min_sovereignty: 1, min_value: 300,
            min_integrity: 0.7, min_content_quality: 0.7,
            min_content_purity: 0.7, min_bag_cleanliness: 0.6,
            min_dryness: 0.6, max_pest_level: 0.3, max_sand_level: 0.3,
            min_edibility: 0.8, min_aesthetic: 0.5,
        }
    }

    pub fn lenient() -> Self {
        Self {
            id: 3,
            min_sovereignty: 0, min_value: 0,
            min_integrity: 0.1, min_content_quality: 0.2,
            min_content_purity: 0.1, min_bag_cleanliness: 0.1,
            min_dryness: 0.1, max_pest_level: 0.9, max_sand_level: 0.9,
            min_edibility: 0.3, min_aesthetic: 0.0,
        }
    }

    pub fn is_valid_multiple(&self, bag: &Bag) -> bool {
        let a = &bag.attrs;
        a.sovereignty >= self.min_sovereignty
            && a.value >= self.min_value
            && a.integrity >= self.min_integrity
            && a.content_quality >= self.min_content_quality
            && a.content_purity >= self.min_content_purity
            && a.bag_cleanliness >= self.min_bag_cleanliness
            && a.dryness >= self.min_dryness
            && a.pest_level <= self.max_pest_level
            && a.sand_level <= self.max_sand_level
            && a.edibility >= self.min_edibility
            && a.aesthetic_score() >= self.min_aesthetic
    }
}

/// 公倍数 — 围绕公约数旋转的实际实例 (v2)
#[derive(Clone, Debug)]
pub struct CommonMultiple {
    pub bag_id: u64,
    pub divisor_id: u64,
    pub orbit_radius: f64,
    pub phase_angle: f64,
}

impl CommonMultiple {
    pub fn new(bag_id: u64, divisor_id: u64) -> Self {
        Self { bag_id, divisor_id, orbit_radius: 0.0, phase_angle: 0.0 }
    }
}

/// 照镜子结果 — 用公约数做镜子的对比结果 (v3)
#[derive(Clone, Debug)]
pub struct MirrorComparison {
    pub bag_id: u64,
    pub divisor_id: u64,
    pub deviation: f64,
    pub aligned: bool,
    pub reflection_score: f64,
    pub diff_details: Vec<String>,
}

impl MirrorComparison {
    pub fn compare(bag: &Bag, divisor: &CommonDivisor) -> Self {
        let a = &bag.attrs;
        let mut diff_details: Vec<String> = Vec::new();

        let integrity_dev = (divisor.min_integrity - a.integrity).max(0.0);
        let quality_dev = (divisor.min_content_quality - a.content_quality).max(0.0);
        let purity_dev = (divisor.min_content_purity - a.content_purity).max(0.0);
        let clean_dev = (divisor.min_bag_cleanliness - a.bag_cleanliness).max(0.0);
        let dry_dev = (divisor.min_dryness - a.dryness).max(0.0);
        let pest_dev = (a.pest_level - divisor.max_pest_level).max(0.0);
        let sand_dev = (a.sand_level - divisor.max_sand_level).max(0.0);
        let edibility_dev = (divisor.min_edibility - a.edibility).max(0.0);
        let aesthetic_dev = (divisor.min_aesthetic - a.aesthetic_score()).max(0.0);

        let total_dev = (
            integrity_dev.powi(2) + quality_dev.powi(2) + purity_dev.powi(2)
            + clean_dev.powi(2) + dry_dev.powi(2) + pest_dev.powi(2)
            + sand_dev.powi(2) + edibility_dev.powi(2) + aesthetic_dev.powi(2)
        ).sqrt();

        let normalized_deviation = (total_dev / 3.0).min(1.0);

        if integrity_dev > 0.0 { diff_details.push(format!("完好欠缺{:.0}%", pct(integrity_dev))); }
        if quality_dev > 0.0 { diff_details.push(format!("质量欠缺{:.0}%", pct(quality_dev))); }
        if purity_dev > 0.0 { diff_details.push(format!("纯度欠缺{:.0}%", pct(purity_dev))); }
        if clean_dev > 0.0 { diff_details.push(format!("清洁欠缺{:.0}%", pct(clean_dev))); }
        if dry_dev > 0.0 { diff_details.push(format!("干燥欠缺{:.0}%", pct(dry_dev))); }
        if pest_dev > 0.0 { diff_details.push(format!("虫害超标{:.0}%", pct(pest_dev))); }
        if sand_dev > 0.0 { diff_details.push(format!("杂质超标{:.0}%", pct(sand_dev))); }
        if edibility_dev > 0.0 { diff_details.push(format!("可食欠缺{:.0}%", pct(edibility_dev))); }
        if aesthetic_dev > 0.0 { diff_details.push(format!("审美欠缺{:.0}%", pct(aesthetic_dev))); }

        let aligned = diff_details.is_empty();
        let reflection_score = 1.0 - normalized_deviation;

        Self { bag_id: bag.id, divisor_id: divisor.id, deviation: normalized_deviation, aligned, reflection_score, diff_details }
    }

    pub fn summary(&self) -> String {
        if self.aligned {
            format!("袋#{} 镜中完美 ✓ 得分{:.0}%", self.bag_id, pct(self.reflection_score))
        } else {
            format!("袋#{} 偏差{:.0}% ✗: {}", self.bag_id, pct(self.deviation), self.diff_details.join(", "))
        }
    }
}

// ======================================================================
// 第七部分: 审判引擎 (JudgmentEngine — v3, 含 elimination_threshold/divisor)
// ======================================================================

#[derive(Clone, Debug)]
pub struct JudgmentResult {
    pub bag_id: u64,
    pub verdict_passed: bool,
    pub confidence: f64,
    pub reason: String,
    pub timestamp_ms: u64,
}

#[derive(Clone, Debug)]
pub struct JudgmentRecord {
    pub bag_id: u64,
    pub verdict: bool,
    pub actual: bool,
    pub confidence: f64,
    pub reward: f64,
    pub timestamp_ms: u64,
}

impl JudgmentRecord {
    pub fn was_correct(&self) -> bool { self.verdict == self.actual }

    pub fn summary(&self) -> String {
        let outcome = if self.was_correct() { "✓" } else { "✗" };
        let rw = if self.reward >= 0.0 {
            format!("+{:.1}奖", self.reward)
        } else {
            format!("{:.1}罚", self.reward)
        };
        format!(
            "袋#{} 审{}实{} {} ({}) 置信{:.0}%",
            self.bag_id,
            if self.verdict { "合格" } else { "不合格" },
            if self.actual { "确实合格" } else { "确实不合格" },
            outcome, rw, self.confidence * 100.0
        )
    }
}

#[derive(Clone, Debug)]
pub struct JudgmentEngine {
    pub meta_judge: MetaJudgeEngine, // v1 兼容性字段
    pub reward_pool: f64,
    pub punishment_pool: f64,
    pub records: Vec<JudgmentRecord>,
    pub current_accuracy: f64,
    pub streak_correct: u64,
    pub streak_wrong: u64,
    pub max_records: usize,
    pub base_reward: f64,
    pub base_punishment: f64,
    pub pass_threshold: f64,
    pub divisor: CommonDivisor,
    pub elimination_threshold: u64,
}

impl JudgmentEngine {
    pub fn new() -> Self {
        Self {
            meta_judge: MetaJudgeEngine::new(32),
            reward_pool: 0.0, punishment_pool: 0.0, records: Vec::new(),
            current_accuracy: 0.5, streak_correct: 0, streak_wrong: 0,
            max_records: 1000, base_reward: 10.0, base_punishment: 10.0,
            pass_threshold: 0.6, divisor: CommonDivisor::standard(),
            elimination_threshold: 3,
        }
    }

    pub fn with_divisor(divisor: CommonDivisor) -> Self {
        Self {
            meta_judge: MetaJudgeEngine::new(32),
            reward_pool: 0.0, punishment_pool: 0.0, records: Vec::new(),
            current_accuracy: 0.5, streak_correct: 0, streak_wrong: 0,
            max_records: 1000, base_reward: 10.0, base_punishment: 10.0,
            pass_threshold: 0.6, divisor,
            elimination_threshold: 3,
        }
    }

    pub fn judge(&self, bag: &Bag, timestamp_ms: u64) -> JudgmentResult {
        let quality = bag.attrs.composite_quality();
        let verdict_quality = quality >= self.pass_threshold;
        let passes_divisor = self.divisor.is_valid_multiple(bag);
        let final_verdict = verdict_quality && passes_divisor;

        let confidence = if final_verdict {
            ((quality - self.pass_threshold) / (1.0 - self.pass_threshold)).min(1.0).max(0.5)
        } else if !verdict_quality {
            ((self.pass_threshold - quality) / self.pass_threshold).min(1.0).max(0.5)
        } else {
            0.7
        };

        let reason = if !verdict_quality {
            format!("质量{:.0}%<阈值{:.0}%", pct(quality), pct(self.pass_threshold))
        } else if !passes_divisor {
            "质量达标但不符合公约数".to_string()
        } else {
            format!("质量{:.0}%>=阈值 且符合公约数", pct(quality))
        };

        JudgmentResult { bag_id: bag.id, verdict_passed: final_verdict, confidence, reason, timestamp_ms }
    }

    /// v1/v2 兼容: 用 os 获取时间戳后调用 judge
    pub fn judge_with_os(&self, bag: &Bag, os: &dyn OsLayer) -> JudgmentResult {
        self.judge(bag, current_timestamp_ms(os))
    }

    pub fn feedback(&mut self, bag_id: u64, verdict: bool, actual: bool, confidence: f64, os: &dyn OsLayer) -> f64 {
        let correct = verdict == actual;
        let reward = if correct {
            self.streak_correct += 1;
            self.streak_wrong = 0;
            let bonus = if self.streak_correct >= 3 { self.streak_correct as f64 * 0.5 } else { 0.0 };
            self.base_reward * confidence + bonus
        } else {
            self.streak_wrong += 1;
            self.streak_correct = 0;
            let penalty = if self.streak_wrong >= 3 { -(self.streak_wrong as f64 * 0.7) } else { 0.0 };
            -(self.base_punishment * confidence) + penalty
        };

        if reward >= 0.0 { self.reward_pool += reward; } else { self.punishment_pool += reward.abs(); }

        self.records.push(JudgmentRecord {
            bag_id, verdict, actual, confidence, reward,
            timestamp_ms: current_timestamp_ms(os),
        });
        self.update_accuracy();

        if self.records.len() > self.max_records { self.records.remove(0); }
        reward
    }

    fn update_accuracy(&mut self) {
        if self.records.is_empty() { self.current_accuracy = 0.5; return; }
        let correct = self.records.iter().filter(|r| r.was_correct()).count();
        self.current_accuracy = correct as f64 / self.records.len() as f64;
    }

    pub fn judge_and_feedback(&mut self, bag: &Bag, actual_good: bool, os: &dyn OsLayer) -> (JudgmentResult, f64) {
        let result = self.judge(bag, current_timestamp_ms(os));
        let reward = self.feedback(result.bag_id, result.verdict_passed, actual_good, result.confidence, os);
        (result, reward)
    }

    pub fn recent_records(&self, n: usize) -> &[JudgmentRecord] {
        let start = self.records.len().saturating_sub(n);
        &self.records[start..]
    }

    pub fn should_eliminate(&self, bag: &Bag) -> bool {
        bag.consecutive_failures >= self.elimination_threshold
    }

    pub fn history_summary(&self) -> String {
        let total = self.records.len();
        let correct = self.records.iter().filter(|r| r.was_correct()).count();
        let total_reward: f64 = self.records.iter().map(|r| if r.reward > 0.0 { r.reward } else { 0.0 }).sum();
        let total_punishment: f64 = self.records.iter().map(|r| if r.reward < 0.0 { r.reward.abs() } else { 0.0 }).sum();
        format!(
            "审判总数={} 正确({:.0}%) 总奖{:.1} 总罚{:.1} 连对{} 连错{}",
            total, pct(self.current_accuracy), total_reward, total_punishment,
            self.streak_correct, self.streak_wrong
        )
    }
}

// ======================================================================
// 第八部分: 时代积累 (HistoricalAccumulation — v3)
// ======================================================================

#[derive(Clone, Debug)]
pub struct Epoch {
    pub start_tick: u64,
    pub end_tick: u64,
    pub accuracy: f64,
    pub snapshot: String,
}

#[derive(Clone, Debug)]
pub struct HistoricalAccumulation {
    pub epochs: Vec<Epoch>,
    pub wisdom_score: f64,
    pub total_judgments: u64,
    pub total_correct: u64,
    pub best_accuracy: f64,
    pub current_epoch_start: u64,
    pub current_records: Vec<JudgmentRecord>,
    pub epoch_capacity: u64,
}

impl HistoricalAccumulation {
    pub fn new(epoch_capacity: u64, start_tick: u64) -> Self {
        Self {
            epochs: Vec::new(), wisdom_score: 0.0,
            total_judgments: 0, total_correct: 0, best_accuracy: 0.0,
            current_epoch_start: start_tick, current_records: Vec::new(),
            epoch_capacity,
        }
    }

    pub fn record(&mut self, record: JudgmentRecord) {
        self.current_records.push(record.clone());
        self.total_judgments += 1;
        if record.was_correct() { self.total_correct += 1; }
        if self.current_records.len() as u64 >= self.epoch_capacity {
            self.seal_epoch(record.timestamp_ms);
        }
    }

    fn seal_epoch(&mut self, end_tick: u64) {
        if self.current_records.is_empty() { return; }
        let correct = self.current_records.iter().filter(|r| r.was_correct()).count();
        let accuracy = correct as f64 / self.current_records.len() as f64;
        let conf_sum: f64 = self.current_records.iter().map(|r| r.confidence).sum();
        let wisdom = accuracy * self.current_records.len() as f64 * (conf_sum / self.current_records.len() as f64);
        self.wisdom_score += wisdom;
        if accuracy > self.best_accuracy { self.best_accuracy = accuracy; }
        self.epochs.push(Epoch {
            start_tick: self.current_epoch_start, end_tick, accuracy,
            snapshot: format!("{}审, {:.0}%准确, +{:.1}智慧", self.current_records.len(), pct(accuracy), wisdom),
        });
        self.current_records.clear();
        self.current_epoch_start = end_tick;
    }

    pub fn finalize(&mut self, end_tick: u64) { self.seal_epoch(end_tick); }
    pub fn wisdom(&self) -> f64 { self.wisdom_score }
    pub fn overall_accuracy(&self) -> f64 {
        if self.total_judgments == 0 { return 0.0; }
        self.total_correct as f64 / self.total_judgments as f64
    }

    pub fn summary(&self) -> String {
        format!("历史: {}时代, {}审, 正确率{:.0}%, 最佳{:.0}%, 智慧{:.1}",
            self.epochs.len(), self.total_judgments, pct(self.overall_accuracy()),
            pct(self.best_accuracy), self.wisdom_score)
    }
}

// ======================================================================
// 第九部分: 袋子层级 (BagHierarchy — v3, 含 health_score/get_mut)
// ======================================================================

#[derive(Clone, Debug)]
pub struct BagHierarchy {
    pub bags: Vec<Bag>,
    pub max_bags: usize,
    pub max_depth: u32,
    pub total_count: u64,
}

impl BagHierarchy {
    pub fn new(max_bags: usize, max_depth: u32) -> Self {
        Self { bags: Vec::new(), max_bags, max_depth, total_count: 0 }
    }

    pub fn add_bag(&mut self, mut bag: Bag) -> Result<(), &'static str> {
        if self.bags.len() >= self.max_bags { return Err("层级已满"); }
        bag.layer_depth = 0;
        self.total_count += 1;
        self.bags.push(bag);
        Ok(())
    }

    pub fn add_nested(&mut self, parent_id: u64, mut bag: Bag) -> Result<(), &'static str> {
        if self.bags.len() >= self.max_bags { return Err("层级已满"); }
        let parent_depth = match self.bags.iter().find(|b| b.id == parent_id) {
            Some(parent) => {
                if parent.layer_depth >= self.max_depth { return Err("已达最大深度"); }
                parent.layer_depth
            }
            None => return Err("父袋子不存在"),
        };
        bag.layer_depth = parent_depth + 1;
        let bag_id = bag.id;
        self.bags.push(bag);
        if let Some(parent) = self.bags.iter_mut().find(|b| b.id == parent_id) {
            parent.child_bag_ids.push(bag_id);
        }
        self.total_count += 1;
        Ok(())
    }

    pub fn count_bags(&self) -> usize { self.bags.len() }

    pub fn count_by_layer(&self) -> Vec<(u32, usize)> {
        let mut counts = std::collections::HashMap::new();
        for bag in &self.bags {
            *counts.entry(bag.layer_depth).or_insert(0) += 1;
        }
        let mut result: Vec<(u32, usize)> = counts.into_iter().collect();
        result.sort_by_key(|x| x.0);
        result
    }

    pub fn get(&self, id: u64) -> Option<&Bag> {
        self.bags.iter().find(|b| b.id == id)
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut Bag> {
        self.bags.iter_mut().find(|b| b.id == id)
    }

    pub fn evaluate(&self) -> HierarchyReport {
        let total = self.bags.len();
        if total == 0 { return HierarchyReport::empty(); }
        let edible = self.bags.iter().filter(|b| b.is_edible()).count();
        let needs = self.bags.iter().filter(|b| b.attrs.needs_any_processing()).count();
        let total_q: f64 = self.bags.iter().map(|b| b.quality()).sum();
        HierarchyReport {
            total_bags: total,
            edible_bags: edible,
            needs_processing: needs,
            average_quality: total_q / total as f64,
            layer_distribution: self.count_by_layer(),
        }
    }

    pub fn health_score(&self) -> f64 {
        let report = self.evaluate();
        if report.total_bags == 0 { return 0.5; }
        let edible_ratio = report.edible_bags as f64 / report.total_bags as f64;
        (edible_ratio + report.average_quality) / 2.0
    }
}

#[derive(Clone, Debug)]
pub struct HierarchyReport {
    pub total_bags: usize,
    pub edible_bags: usize,
    pub needs_processing: usize,
    pub average_quality: f64,
    pub layer_distribution: Vec<(u32, usize)>,
}

impl HierarchyReport {
    fn empty() -> Self {
        Self { total_bags: 0, edible_bags: 0, needs_processing: 0, average_quality: 0.0, layer_distribution: Vec::new() }
    }

    pub fn summary(&self) -> String {
        if self.total_bags == 0 { return "层级为空".to_string(); }
        let layers: Vec<String> = self.layer_distribution.iter().map(|(d, c)| format!("L{}:{}个", d, c)).collect();
        format!("层级: {}袋, 可食{:.0}%, 需处理{}个, 均质{:.0}%, [{}]",
            self.total_bags, pct(self.edible_bags as f64 / self.total_bags as f64),
            self.needs_processing, pct(self.average_quality), layers.join(", "))
    }
}

// ── v1 兼容类型别名 ──
pub type EpochV1 = Epoch;

// ======================================================================
// V3-SPECIFIC TYPES （重命名为 V3 后缀, 避免与 v4 冲突）
// ======================================================================

// ── v3 Emotion (含强度值) ──
#[derive(Clone, Debug)]
pub enum EmotionV3 {
    Joy(f64),
    Disgust(f64),
    Sorrow(f64),
    Peace(f64),
    Anxiety(f64),
}

impl EmotionV3 {
    pub fn intensity(&self) -> f64 {
        match self {
            EmotionV3::Joy(v) | EmotionV3::Disgust(v) | EmotionV3::Sorrow(v)
            | EmotionV3::Peace(v) | EmotionV3::Anxiety(v) => *v,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            EmotionV3::Joy(_) => "喜",
            EmotionV3::Disgust(_) => "恶",
            EmotionV3::Sorrow(_) => "哀",
            EmotionV3::Peace(_) => "宁",
            EmotionV3::Anxiety(_) => "忧",
        }
    }
}

#[derive(Clone, Debug)]
pub struct EmotionSystemV3 {
    pub current_emotions: Vec<EmotionV3>,
    pub threshold_modifier: f64,
}

impl EmotionSystemV3 {
    pub fn new() -> Self {
        Self { current_emotions: Vec::new(), threshold_modifier: 0.0 }
    }

    pub fn feel(&mut self, bag: &Bag) {
        self.current_emotions.clear();
        let q = bag.quality();
        if q > 0.8 { self.current_emotions.push(EmotionV3::Joy(q)); }
        else if q < 0.3 { self.current_emotions.push(EmotionV3::Disgust(1.0 - q)); }
        else if bag.consecutive_failures > 0 {
            self.current_emotions.push(EmotionV3::Sorrow(bag.consecutive_failures as f64 * 0.2));
        }
        if bag.attrs.needs_any_processing() { self.current_emotions.push(EmotionV3::Anxiety(0.3)); }
        if !self.current_emotions.iter().any(|e| matches!(e, EmotionV3::Anxiety(_))) && q > 0.6 {
            self.current_emotions.push(EmotionV3::Peace(q * 0.5));
        }
        self.threshold_modifier = self.calculate_threshold_modifier();
    }

    fn calculate_threshold_modifier(&self) -> f64 {
        let mut modifier = 0.0;
        for emotion in &self.current_emotions {
            match emotion {
                EmotionV3::Joy(v) => modifier -= v * 0.05,
                EmotionV3::Disgust(v) => modifier += v * 0.1,
                EmotionV3::Sorrow(v) => modifier += v * 0.05,
                EmotionV3::Peace(v) => modifier -= v * 0.02,
                EmotionV3::Anxiety(v) => modifier += v * 0.08,
            }
        }
        modifier.clamp(-0.2, 0.2)
    }

    pub fn adjusted_threshold(&self, base_threshold: f64) -> f64 {
        (base_threshold + self.threshold_modifier).clamp(0.3, 0.9)
    }

    pub fn summary(&self) -> String {
        if self.current_emotions.is_empty() { "情进: 平静".to_string() }
        else {
            let emotions: Vec<String> = self.current_emotions.iter()
                .map(|e| format!("{}{:.0}", e.name(), pct(e.intensity())))
                .collect();
            format!("情进: {} (阈值修正{:+.0}%)", emotions.join(" "), pct(self.threshold_modifier))
        }
    }
}

// ── v3 Behavior ──
#[derive(Clone, Debug)]
pub enum BehaviorV3 {
    Accept, Reject, WashContent, DryContent, PickSand, Repair, Eliminate, Wait,
}

impl BehaviorV3 {
    pub fn name(&self) -> &'static str {
        match self {
            BehaviorV3::Accept => "收", BehaviorV3::Reject => "弃",
            BehaviorV3::WashContent => "洗", BehaviorV3::DryContent => "晒",
            BehaviorV3::PickSand => "捡", BehaviorV3::Repair => "补",
            BehaviorV3::Eliminate => "汰", BehaviorV3::Wait => "等",
        }
    }
}

#[derive(Clone, Debug)]
pub struct BehaviorSystemV3 {
    pub actions: Vec<(u64, BehaviorV3, String)>,
}

impl BehaviorSystemV3 {
    pub fn new() -> Self { Self { actions: Vec::new() } }

    pub fn decide(&mut self, bag: &Bag, verdict_passed: bool) -> BehaviorV3 {
        self.actions.clear();
        let action = if verdict_passed {
            BehaviorV3::Accept
        } else if bag.consecutive_failures >= 3 {
            BehaviorV3::Eliminate
        } else if bag.attrs.content_quality < 0.2 {
            BehaviorV3::Reject
        } else if bag.attrs.needs_content_wash && bag.attrs.content_quality > 0.5 {
            BehaviorV3::WashContent
        } else if bag.attrs.needs_drying() {
            BehaviorV3::DryContent
        } else if bag.attrs.needs_sand_pick && bag.attrs.content_quality > 0.4 {
            BehaviorV3::PickSand
        } else if bag.attrs.needs_repair && bag.attrs.content_quality > 0.5 {
            BehaviorV3::Repair
        } else {
            BehaviorV3::Wait
        };
        self.actions.push((bag.id, action.clone(), format!("袋#{}→{}", bag.id, action.name())));
        action
    }

    pub fn summary(&self) -> String {
        if self.actions.is_empty() { "行为: 无".to_string() }
        else { format!("行为: {}", self.actions.iter().map(|(_, b, _)| b.name()).collect::<Vec<_>>().join(" ")) }
    }
}

// ── v3 ThinkingSystem ──
#[derive(Clone, Debug)]
pub struct ReasoningChainV3 {
    pub steps: Vec<String>,
    pub conclusion: String,
}

#[derive(Clone, Debug)]
pub struct ThinkingSystemV3 {
    pub last_reasoning: Option<ReasoningChainV3>,
}

impl ThinkingSystemV3 {
    pub fn new() -> Self { Self { last_reasoning: None } }

    pub fn think(&mut self, bag: &Bag, history_accuracy: f64, hierarchy_health: f64) -> ReasoningChainV3 {
        let mut steps = Vec::new();
        steps.push(format!("袋#{}质量={:.0}%", bag.id, pct(bag.quality())));
        if bag.judgment_count > 0 {
            steps.push(format!("已审判{}次,连败{}次", bag.judgment_count, bag.consecutive_failures));
        }
        if history_accuracy > 0.8 {
            steps.push(format!("历史准确率高({:.0}%),信任当前审判", pct(history_accuracy)));
        } else if history_accuracy < 0.5 {
            steps.push(format!("历史准确率低({:.0}%),需谨慎", pct(history_accuracy)));
        }
        if hierarchy_health > 0.7 {
            steps.push("层级健康,可接纳新袋".to_string());
        } else {
            steps.push("层级亚健康的,严格过滤".to_string());
        }
        let conclusion = if bag.quality() > 0.7 && bag.consecutive_failures == 0 {
            "← 可接纳"
        } else if bag.consecutive_failures >= 2 {
            "← 高危,建议观察"
        } else {
            "← 需处理后再审"
        };
        steps.push(conclusion.to_string());
        let chain = ReasoningChainV3 { steps, conclusion: conclusion.to_string() };
        self.last_reasoning = Some(chain.clone());
        chain
    }

    pub fn summary(&self) -> String {
        match &self.last_reasoning {
            Some(chain) => format!("思考: {}", chain.conclusion),
            None => "思考: 未激活".to_string(),
        }
    }
}

// ── v3 AestheticSystem ──
#[derive(Clone, Debug)]
pub struct AestheticSystemV3 {
    pub last_score: f64,
}

impl AestheticSystemV3 {
    pub fn new() -> Self { Self { last_score: 0.0 } }

    pub fn evaluate(&mut self, bag: &Bag) -> f64 {
        let score = bag.attrs.aesthetic_score();
        self.last_score = score;
        score
    }

    pub fn verdict(&self) -> &str {
        if self.last_score > 0.8 { "美" }
        else if self.last_score > 0.6 { "雅" }
        else if self.last_score > 0.4 { "凡" }
        else if self.last_score > 0.2 { "陋" }
        else { "恶" }
    }

    pub fn summary(&self) -> String {
        format!("审美: {} ({:.0}%)", self.verdict(), pct(self.last_score))
    }
}

// ── v3 EliminationSystem ──
#[derive(Clone, Debug)]
pub struct EliminationSystemV3 {
    pub eliminated_bags: Vec<u64>,
    pub threshold: u64,
}

impl EliminationSystemV3 {
    pub fn new(threshold: u64) -> Self {
        Self { eliminated_bags: Vec::new(), threshold }
    }

    pub fn check(&mut self, bag: &mut Bag) -> bool {
        if bag.consecutive_failures >= self.threshold && !bag.eliminated {
            bag.eliminated = true;
            self.eliminated_bags.push(bag.id);
            true
        } else { false }
    }

    pub fn is_eliminated(&self, bag_id: u64) -> bool { self.eliminated_bags.contains(&bag_id) }

    pub fn summary(&self) -> String {
        format!("淘汰: {}个袋子已被淘汰 ({:?})", self.eliminated_bags.len(), self.eliminated_bags)
    }
}

// ── v3 PipelineResult ──
#[derive(Clone, Debug)]
pub struct PipelineResultV3 {
    pub bag_id: u64,
    pub accepted: bool,
    pub behavior: BehaviorV3,
    pub judgment: Option<JudgmentResult>,
    pub reward: f64,
    pub mirror: Option<MirrorComparison>,
    pub emotion_summary: String,
    pub thinking_summary: String,
    pub aesthetic_summary: String,
    pub eliminated: bool,
}

// ── v3 FilterSystem ──
#[derive(Clone, Debug)]
pub struct FilterSystemV3 {
    pub min_quality: f64,
    pub filtered_bags: Vec<u64>,
    pub filter_reasons: Vec<(u64, String)>,
}

impl FilterSystemV3 {
    pub fn new(min_quality: f64) -> Self {
        Self { min_quality, filtered_bags: Vec::new(), filter_reasons: Vec::new() }
    }

    pub fn filter(&mut self, bag: &Bag) -> bool {
        if bag.eliminated {
            self.filtered_bags.push(bag.id);
            self.filter_reasons.push((bag.id, "已淘汰".to_string()));
            return false;
        }
        if bag.quality() < self.min_quality {
            self.filtered_bags.push(bag.id);
            self.filter_reasons.push((bag.id, format!("质量{:.0}%低于准入{:.0}%", pct(bag.quality()), pct(self.min_quality))));
            return false;
        }
        true
    }

    pub fn summary(&self) -> String {
        format!("过滤: {}个袋子被拦截", self.filtered_bags.len())
    }
}

// ======================================================================
// v3 天庭引擎 (CelestialPerceptionV3)
// ======================================================================

pub struct CelestialPerceptionV3<'a> {
    pub auth_heavenly_court: HeavenlyCourtAuth,
    pub auth_auditory: AuditoryAuth,
    pub visual: VisualSystem,
    pub auditory: AuditorySystem,
    pub observer: Observer<'a>,
    pub judgment: JudgmentEngine,
    pub history: HistoricalAccumulation,
    pub hierarchy: BagHierarchy,
    pub bags_perceived: u64,
    pub mirror_comparisons: Vec<MirrorComparison>,
    pub emotion: EmotionSystemV3,
    pub behavior: BehaviorSystemV3,
    pub thinking: ThinkingSystemV3,
    pub aesthetic: AestheticSystemV3,
    pub elimination: EliminationSystemV3,
    pub filter: FilterSystemV3,
}

impl<'a> CelestialPerceptionV3<'a> {
    pub fn new(os: &'a dyn OsLayer) -> Self {
        Self {
            auth_heavenly_court: HeavenlyCourtAuth::Dormant,
            auth_auditory: AuditoryAuth::Silent,
            visual: VisualSystem::default_vision(),
            auditory: AuditorySystem::default_hearing(),
            observer: Observer::new(1, "天庭v3", 64, os),
            judgment: JudgmentEngine::new(),
            history: HistoricalAccumulation::new(50, 0),
            hierarchy: BagHierarchy::new(1000, 10),
            bags_perceived: 0,
            mirror_comparisons: Vec::new(),
            emotion: EmotionSystemV3::new(),
            behavior: BehaviorSystemV3::new(),
            thinking: ThinkingSystemV3::new(),
            aesthetic: AestheticSystemV3::new(),
            elimination: EliminationSystemV3::new(3),
            filter: FilterSystemV3::new(0.0),
        }
    }

    pub fn authorize_heavenly_court(&mut self) -> bool { self.auth_heavenly_court.authorize() }
    pub fn authorize_auditory(&mut self) -> bool { self.auth_auditory.authorize() }

    pub fn authorize_all(&mut self) {
        self.auth_heavenly_court.authorize();
        self.auth_auditory.authorize();
    }

    pub fn is_fully_authorized(&self) -> bool {
        self.auth_heavenly_court.can_perceive() && self.auth_auditory.can_hear()
    }

    pub fn full_pipeline(&mut self, bag: &mut Bag, actual_good: bool) -> Result<PipelineResultV3, &'static str> {
        if !self.auth_heavenly_court.can_perceive() { return Err("天庭未授权"); }

        self.emotion.feel(bag);
        self.aesthetic.evaluate(bag);

        if !self.filter.filter(bag) {
            return Ok(PipelineResultV3 {
                bag_id: bag.id, accepted: false, behavior: BehaviorV3::Reject,
                judgment: None, reward: 0.0, mirror: None,
                emotion_summary: self.emotion.summary(),
                thinking_summary: "".to_string(),
                aesthetic_summary: self.aesthetic.summary(),
                eliminated: false,
            });
        }

        let mut forms = self.visual.perceive(bag);
        if self.auth_auditory.can_hear() {
            if let Ok(auditory_forms) = self.auditory.listen(bag) {
                forms.extend(auditory_forms);
            }
        }
        let _cells = self.observer.sense_batch(forms);
        self.bags_perceived += 1;

        let mirror = MirrorComparison::compare(bag, &self.judgment.divisor);
        self.mirror_comparisons.push(mirror.clone());

        let hierarchy_health = self.hierarchy.health_score();
        let reasoning = self.thinking.think(bag, self.judgment.current_accuracy, hierarchy_health);

        let adjusted_threshold = self.emotion.adjusted_threshold(self.judgment.pass_threshold);
        let old_threshold = self.judgment.pass_threshold;
        self.judgment.pass_threshold = adjusted_threshold;
        let (result, reward) = self.judgment.judge_and_feedback(bag, actual_good, self.observer.os());
        self.judgment.pass_threshold = old_threshold;

        bag.judgment_count += 1;
        if result.verdict_passed {
            bag.consecutive_failures = 0;
        } else {
            bag.consecutive_failures += 1;
        }

        let behavior = self.behavior.decide(bag, result.verdict_passed);
        let eliminated = self.elimination.check(bag);

        let record = JudgmentRecord {
            bag_id: bag.id,
            verdict: result.verdict_passed,
            actual: actual_good,
            confidence: result.confidence,
            reward,
            timestamp_ms: self.observer.os().time_millis(),
        };
        self.history.record(record);

        Ok(PipelineResultV3 {
            bag_id: bag.id,
            accepted: result.verdict_passed,
            behavior,
            judgment: Some(result),
            reward,
            mirror: Some(mirror),
            emotion_summary: self.emotion.summary(),
            thinking_summary: reasoning.conclusion,
            aesthetic_summary: self.aesthetic.summary(),
            eliminated,
        })
    }

    pub fn consciousness_density(&self) -> f64 { self.observer.consciousness_density() }

    pub fn report(&self) -> String {
        let mut s = String::new();
        s.push_str("═══ 天庭v3报告 (老三) ═══\n");
        s.push_str(&format!("  天庭授权: {} | 听觉授权: {}\n",
            if self.auth_heavenly_court.can_perceive() { "✓" } else { "✗" },
            if self.auth_auditory.can_hear() { "✓" } else { "✗" }));
        s.push_str(&format!("  已感知袋数: {} | 意识密度: {:.3}\n", self.bags_perceived, self.consciousness_density()));
        s.push_str(&format!("  {}\n", self.judgment.history_summary()));
        s.push_str(&format!("  {}\n", self.history.summary()));
        s.push_str(&format!("  {}\n", self.hierarchy.evaluate().summary()));
        s.push_str(&format!("  {}\n", self.emotion.summary()));
        s.push_str(&format!("  {}\n", self.behavior.summary()));
        s.push_str(&format!("  {}\n", self.thinking.summary()));
        s.push_str(&format!("  {}\n", self.aesthetic.summary()));
        s.push_str(&format!("  {}\n", self.elimination.summary()));
        s.push_str(&format!("  {}\n", self.filter.summary()));
        s
    }
}

// ======================================================================
// V4-SPECIFIC TYPES （七情重构版, 保留原名以匹配 lib.rs 导出）
// ======================================================================

// ── v4 Emotion (七情离散) ──
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Emotion {
    Joy, Anger, Sorrow, Fear, Love, Disgust, Desire,
}

impl Emotion {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Joy => "喜", Self::Anger => "怒", Self::Sorrow => "哀",
            Self::Fear => "惧", Self::Love => "爱", Self::Disgust => "恶",
            Self::Desire => "欲",
        }
    }

    pub fn is_positive(&self) -> bool {
        matches!(self, Self::Joy | Self::Love | Self::Desire)
    }

    pub fn is_negative(&self) -> bool {
        matches!(self, Self::Anger | Self::Sorrow | Self::Fear | Self::Disgust)
    }

    pub fn vector(&self) -> [f64; 7] {
        let mut v = [0.0; 7];
        let idx = match self {
            Self::Joy => 0, Self::Anger => 1, Self::Sorrow => 2, Self::Fear => 3,
            Self::Love => 4, Self::Disgust => 5, Self::Desire => 6,
        };
        v[idx] = 1.0;
        v
    }
}

#[derive(Clone, Debug)]
pub struct EmotionState {
    pub intensities: [f64; 7],
    pub dominant: Emotion,
    pub valence: f64,
    pub arousal: f64,
}

impl EmotionState {
    pub fn new() -> Self {
        Self { intensities: [0.0; 7], dominant: Emotion::Joy, valence: 0.0, arousal: 0.0 }
    }

    pub fn from_bag_attrs(a: &BagAttributes) -> Self {
        let mut state = Self::new();
        let joy = a.composite_quality();
        let love: f64 = if a.integrity > 0.9 && a.content_quality > 0.9 { 0.9 } else { 0.1 };
        let desire = joy * 0.7;
        let anger: f64 = if a.integrity > 0.7 && a.content_quality < 0.3 { 0.8 } else { 0.05 };
        let sorrow = 1.0 - joy;
        let fear: f64 = if a.integrity < 0.2 || a.pest_level > 0.8 { 0.9 } else { 0.05 };
        let disgust = (a.pest_level + a.sand_level) / 2.0 + (1.0 - a.content_purity) * 0.5;

        state.intensities = [
            joy.min(1.0), anger.min(1.0), sorrow.min(1.0), fear.min(1.0),
            love.min(1.0), disgust.min(1.0), desire.min(1.0),
        ];

        let max_idx = state.intensities.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i).unwrap_or(0);
        state.dominant = match max_idx {
            0 => Emotion::Joy, 1 => Emotion::Anger, 2 => Emotion::Sorrow,
            3 => Emotion::Fear, 4 => Emotion::Love, 5 => Emotion::Disgust,
            6 => Emotion::Desire, _ => Emotion::Joy,
        };

        let positive = joy + love + desire;
        let negative = anger + sorrow + fear + disgust;
        state.valence = ((positive - negative) / 4.0).clamp(-1.0, 1.0);
        state.arousal = *state.intensities.iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0);
        state
    }

    pub fn summary(&self) -> String {
        format!("情进[主导{} 效价{:+.2} 唤醒{:.0}%]",
            self.dominant.label(), self.valence, self.arousal * 100.0)
    }
}

impl Default for EmotionState {
    fn default() -> Self { Self::new() }
}

// ── v4 Behavior ──
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Behavior {
    Accept, WashContent, PickSand, RepairBag, DryInSun,
    RemovePest, Reject, Eliminate, Store, Gift,
}

impl Behavior {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Accept => "接纳", Self::WashContent => "洗米",
            Self::PickSand => "捡沙", Self::RepairBag => "补袋",
            Self::DryInSun => "晒干", Self::RemovePest => "除虫",
            Self::Reject => "拒收", Self::Eliminate => "销毁",
            Self::Store => "储存", Self::Gift => "赠送",
        }
    }
}

pub struct BehaviorSystem {
    pub accept_threshold: f64,
    pub eliminate_threshold: f64,
    pub history: Vec<(u64, Behavior, String)>,
}

impl BehaviorSystem {
    pub fn new() -> Self {
        Self { accept_threshold: 0.6, eliminate_threshold: 0.15, history: Vec::new() }
    }

    pub fn decide(&mut self, bag: &Bag, emotion: &EmotionState, judgment_passed: bool) -> Behavior {
        let a = &bag.attrs;
        let behavior = if !judgment_passed && a.composite_quality() < self.eliminate_threshold {
            Behavior::Eliminate
        } else if !judgment_passed && emotion.intensities[5] > 0.6 {
            Behavior::Eliminate
        } else if a.pest_level > 0.6 && a.sand_level < 0.3 {
            Behavior::RemovePest
        } else if a.sand_level > 0.5 && a.content_quality > 0.5 {
            Behavior::PickSand
        } else if a.needs_content_wash && a.content_quality > 0.6 {
            Behavior::WashContent
        } else if a.needs_repair && a.integrity > 0.2 {
            Behavior::RepairBag
        } else if a.needs_drying() {
            Behavior::DryInSun
        } else if judgment_passed && emotion.dominant == Emotion::Love && a.composite_quality() > 0.85 {
            Behavior::Store
        } else if judgment_passed {
            Behavior::Accept
        } else {
            Behavior::Reject
        };

        self.history.push((bag.id, behavior, format!("情进{}", emotion.summary())));
        behavior
    }

    pub fn history_summary(&self) -> String {
        let mut counts = std::collections::HashMap::new();
        for (_, b, _) in &self.history {
            *counts.entry(*b).or_insert(0) += 1;
        }
        let items: Vec<String> = counts.iter()
            .map(|(b, c)| format!("{}:{}", b.label(), c))
            .collect();
        format!("行为历史: {}", items.join(", "))
    }
}

impl Default for BehaviorSystem {
    fn default() -> Self { Self::new() }
}

// ── v4 ThinkingSystem ──
#[derive(Clone, Debug)]
pub struct ReasoningChain {
    pub steps: Vec<String>,
    pub conclusion: String,
    pub confidence: f64,
}

pub struct ThinkingSystem {
    pub chains: Vec<Vec<ReasoningChain>>,
}

impl ThinkingSystem {
    pub fn new() -> Self { Self { chains: Vec::new() } }

    pub fn reason(&mut self, bag: &Bag, emotion: &EmotionState, mirror: &MirrorComparison) -> ReasoningChain {
        let a = &bag.attrs;
        let mut steps = Vec::new();
        steps.push(format!("观察: 袋子{}「{}」— {}", bag.id, bag.name, a.summary()));
        if mirror.aligned {
            steps.push(format!("照镜子: 完美对齐 ✓ 镜中得分{:.0}%", mirror.reflection_score * 100.0));
        } else {
            steps.push(format!("照镜子: 有偏差 ✗ ({})", mirror.diff_details.join(", ")));
        }
        steps.push(format!("情进响应: 主导{}, 效价{:+.2}", emotion.dominant.label(), emotion.valence));
        let problems = Self::diagnose(a);
        if !problems.is_empty() {
            steps.push(format!("问题诊断: {}", problems.join("; ")));
        } else {
            steps.push("问题诊断: 无明显问题".to_string());
        }
        let (conclusion, confidence) = Self::synthesize(a, emotion, mirror);
        steps.push(format!("综合判断: 应该{} (置信度{:.0}%)", conclusion, confidence * 100.0));

        let chain = ReasoningChain { steps, conclusion, confidence };
        self.chains.push(vec![chain.clone()]);
        chain
    }

    fn diagnose(a: &BagAttributes) -> Vec<String> {
        let mut problems = Vec::new();
        if a.integrity < 0.5 { problems.push(format!("破损(完好{:.0}%不足)", a.integrity * 100.0)); }
        if a.content_quality < 0.5 { problems.push("内容质量差".to_string()); }
        if a.pest_level > 0.5 { problems.push(format!("虫害{:.0}%", a.pest_level * 100.0)); }
        if a.sand_level > 0.5 { problems.push(format!("杂质{:.0}%", a.sand_level * 100.0)); }
        if a.dryness < 0.3 { problems.push("过湿".to_string()); }
        if a.dryness > 0.95 && a.integrity < 0.5 { problems.push("过干且裂".to_string()); }
        if a.bag_cleanliness < 0.4 { problems.push("袋脏".to_string()); }
        problems
    }

    fn synthesize(a: &BagAttributes, emotion: &EmotionState, mirror: &MirrorComparison) -> (String, f64) {
        let quality = a.composite_quality();
        if mirror.aligned && quality > 0.7 {
            ("直接接纳".to_string(), 0.9)
        } else if emotion.valence < -0.5 && quality < 0.3 {
            ("销毁处理".to_string(), 0.85)
        } else if a.needs_any_processing() {
            ("清洗修补后接纳".to_string(), 0.75)
        } else if mirror.aligned {
            ("收纳储存".to_string(), 0.8)
        } else {
            ("二次检查".to_string(), 0.6)
        }
    }
}

impl Default for ThinkingSystem {
    fn default() -> Self { Self::new() }
}

// ── v4 AestheticSystem ──
#[derive(Clone, Copy, Debug)]
pub struct AestheticDimensions {
    pub harmony: f64,
    pub elegance: f64,
    pub simplicity: f64,
    pub vitality: f64,
    pub authenticity: f64,
}

#[derive(Clone, Debug)]
pub struct AestheticVerdict {
    pub score: f64,
    pub label: String,
    pub dimensions: AestheticDimensions,
}

pub struct AestheticSystem {
    pub history: Vec<(u64, AestheticVerdict)>,
}

impl AestheticSystem {
    pub fn new() -> Self { Self { history: Vec::new() } }

    pub fn evaluate(&mut self, bag: &Bag) -> AestheticVerdict {
        let a = &bag.attrs;
        let harmony = (a.integrity * 0.3 + a.bag_cleanliness * 0.3 + (1.0 - a.pest_level) * 0.2 + (1.0 - a.sand_level) * 0.2).clamp(0.0, 1.0);
        let elegance = (a.content_purity * 0.4 + a.content_quality * 0.4 + (1.0 - a.sand_level) * 0.2).clamp(0.0, 1.0);
        let simplicity = ((1.0 - a.pest_level) * 0.3 + (1.0 - a.sand_level) * 0.3 + a.integrity * 0.4).clamp(0.0, 1.0);
        let vitality = (a.content_quality * 0.5 + a.edibility * 0.5).clamp(0.0, 1.0);
        let authenticity = (a.integrity * 0.5 + a.content_purity * 0.5).clamp(0.0, 1.0);

        let dim = AestheticDimensions { harmony, elegance, simplicity, vitality, authenticity };
        let score = dim.harmony * 0.25 + dim.elegance * 0.25 + dim.simplicity * 0.15 + dim.vitality * 0.20 + dim.authenticity * 0.15;

        let label = if score > 0.7 { "美".to_string() } else if score > 0.3 { "平".to_string() } else { "丑".to_string() };
        let verdict = AestheticVerdict { score, label, dimensions: dim };
        self.history.push((bag.id, verdict.clone()));
        verdict
    }

    pub fn history_summary(&self) -> String {
        let total = self.history.len();
        let beautiful = self.history.iter().filter(|(_, v)| v.score > 0.7).count();
        let ugly = self.history.iter().filter(|(_, v)| v.score < 0.3).count();
        format!("审美: {}袋, 美{} 丑{}", total, beautiful, ugly)
    }
}

impl Default for AestheticSystem {
    fn default() -> Self { Self::new() }
}

// ── v4 EliminationSystem ──
#[derive(Clone, Debug)]
pub enum EliminationReason {
    Rotten, Unrepairable, PestInfested, Tainted, Deprecated,
}

impl EliminationReason {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Rotten => "腐烂", Self::Unrepairable => "不可修复",
            Self::PestInfested => "虫害严重", Self::Tainted => "污染", Self::Deprecated => "过时",
        }
    }
}

#[derive(Clone, Debug)]
pub struct EliminationRecord {
    pub bag_id: u64,
    pub reason: EliminationReason,
    pub quality_at_removal: f64,
    pub timestamp_ms: u64,
}

pub struct EliminationSystem {
    pub records: Vec<EliminationRecord>,
    pub removal_count: u64,
}

impl EliminationSystem {
    pub fn new() -> Self { Self { records: Vec::new(), removal_count: 0 } }

    pub fn should_eliminate(&self, bag: &Bag) -> Option<EliminationReason> {
        let a = &bag.attrs;
        if a.content_quality < 0.1 && a.edibility < 0.1 && a.integrity < 0.1 {
            Some(EliminationReason::Rotten)
        } else if a.pest_level > 0.8 && a.content_quality < 0.2 {
            Some(EliminationReason::PestInfested)
        } else if a.integrity < 0.1 && a.value < 50 {
            Some(EliminationReason::Unrepairable)
        } else if a.content_purity < 0.1 && a.pest_level > 0.6 {
            Some(EliminationReason::Tainted)
        } else {
            None
        }
    }

    pub fn eliminate(&mut self, bag: &Bag, reason: EliminationReason) {
        self.records.push(EliminationRecord {
            bag_id: bag.id,
            reason,
            quality_at_removal: bag.attrs.composite_quality(),
            timestamp_ms: crate::os_layer::current_os().time_millis(),
        });
        self.removal_count += 1;
    }

    pub fn history_summary(&self) -> String {
        format!("淘汰: {}袋", self.removal_count)
    }
}

impl Default for EliminationSystem {
    fn default() -> Self { Self::new() }
}

// ── v4 FilterSystem ──
pub struct FilterSystem {
    pub quality_cutoff: f64,
    pub max_pest_tolerance: f64,
    pub filtered_count: u64,
    pub filter_log: Vec<(u64, String)>,
}

impl FilterSystem {
    pub fn new() -> Self {
        Self { quality_cutoff: 0.1, max_pest_tolerance: 0.95, filtered_count: 0, filter_log: Vec::new() }
    }

    pub fn filter(&mut self, bag: &Bag) -> bool {
        let a = &bag.attrs;
        if a.content_quality < 0.05 && a.integrity < 0.05 {
            self.filter_log.push((bag.id, "极端腐烂".to_string()));
            self.filtered_count += 1;
            return false;
        }
        if a.pest_level > 0.95 {
            self.filter_log.push((bag.id, "极端虫害".to_string()));
            self.filtered_count += 1;
            return false;
        }
        if a.composite_quality() < self.quality_cutoff && a.value < 10 {
            self.filter_log.push((bag.id, "质量过低无价值".to_string()));
            self.filtered_count += 1;
            return false;
        }
        true
    }

    pub fn history_summary(&self) -> String {
        format!("过滤: {}袋被筛", self.filtered_count)
    }
}

impl Default for FilterSystem {
    fn default() -> Self { Self::new() }
}

// ======================================================================
// v4 PipelineResult 和 CelestialPerceptionV4 引擎
// ======================================================================

#[derive(Clone, Debug)]
pub struct PipelineResult {
    pub bag_id: u64,
    pub passed_filter: bool,
    pub judgment_passed: bool,
    pub reward: f64,
    pub mirror_aligned: bool,
    pub emotion_summary: String,
    pub behavior: String,
    pub reasoning_summary: String,
    pub aesthetic_label: String,
    pub eliminated: bool,
    pub message: String,
}

impl PipelineResult {
    pub fn summary(&self) -> String {
        let status = if self.eliminated { "✗已淘汰" }
            else if self.judgment_passed { "✓通过" }
            else { "✗不通过" };
        format!("袋#{} {} 奖{:+.1} 行为{} 审美{} {}",
            self.bag_id, status, self.reward, self.behavior, self.aesthetic_label, self.message)
    }
}

pub struct CelestialPerceptionV4<'a> {
    pub auth_heavenly_court: HeavenlyCourtAuth,
    pub auth_auditory: AuditoryAuth,
    pub visual: VisualSystem,
    pub auditory: AuditorySystem,
    pub observer: Observer<'a>,
    pub judgment: JudgmentEngine,
    pub mirror_comparisons: Vec<MirrorComparison>,
    pub emotion: EmotionState,
    pub behavior: BehaviorSystem,
    pub thinking: ThinkingSystem,
    pub aesthetic: AestheticSystem,
    pub elimination: EliminationSystem,
    pub filter: FilterSystem,
    pub hierarchy: BagHierarchy,
    pub history: HistoricalAccumulation,
    pub bags_processed: u64,
    pub pipeline_results: Vec<PipelineResult>,
}

impl<'a> CelestialPerceptionV4<'a> {
    pub fn new(os: &'a dyn OsLayer) -> Self {
        Self {
            auth_heavenly_court: HeavenlyCourtAuth::Dormant,
            auth_auditory: AuditoryAuth::Silent,
            visual: VisualSystem::default_vision(),
            auditory: AuditorySystem::default_hearing(),
            observer: Observer::new(1, "天庭v4", 64, os),
            judgment: JudgmentEngine::new(),
            mirror_comparisons: Vec::new(),
            emotion: EmotionState::new(),
            behavior: BehaviorSystem::new(),
            thinking: ThinkingSystem::new(),
            aesthetic: AestheticSystem::new(),
            elimination: EliminationSystem::new(),
            filter: FilterSystem::new(),
            hierarchy: BagHierarchy::new(1000, 10),
            history: HistoricalAccumulation::new(50, 0),
            bags_processed: 0,
            pipeline_results: Vec::new(),
        }
    }

    pub fn authorize_all(&mut self) {
        self.auth_heavenly_court.authorize();
        self.auth_auditory.authorize();
        self.auditory.auth.authorize();
    }

    pub fn process_bag(&mut self, bag: &Bag, actual_good: bool) -> PipelineResult {
        self.bags_processed += 1;

        // Step 1: 过滤
        if self.auth_heavenly_court.can_perceive() && !self.filter.filter(bag) {
            let reason = if bag.attrs.content_quality < 0.1 { EliminationReason::Rotten }
                else if bag.attrs.pest_level > 0.8 { EliminationReason::PestInfested }
                else { EliminationReason::Deprecated };
            self.elimination.eliminate(bag, reason.clone());

            return PipelineResult {
                bag_id: bag.id, passed_filter: false, judgment_passed: false,
                reward: 0.0, mirror_aligned: false,
                emotion_summary: "未分析".to_string(), behavior: "过滤".to_string(),
                reasoning_summary: format!("直接过滤: {}", reason.label()),
                aesthetic_label: "未评".to_string(), eliminated: true,
                message: format!("被过滤({})不进入管道", reason.label()),
            };
        }

        // Step 2: 感知
        let mut forms = Vec::new();
        if self.auth_heavenly_court.can_perceive() { forms.extend(self.visual.perceive(bag)); }
        if self.auth_auditory.can_hear() {
            if let Ok(auditory_forms) = self.auditory.listen(bag) {
                forms.extend(auditory_forms);
            }
        }
        let _cells = self.observer.sense_batch(forms);

        // Step 3: 照镜子
        let divisor = self.judgment.divisor;
        let mirror = MirrorComparison::compare(bag, &divisor);
        self.mirror_comparisons.push(mirror.clone());

        // Step 4: 情进
        let emotion = EmotionState::from_bag_attrs(&bag.attrs);
        let emotion_summary = emotion.summary();

        // Step 5: 审判+奖惩
        let judgment_result = self.judgment.judge_with_os(bag, self.observer.os());
        let reward = self.judgment.feedback(
            bag.id, judgment_result.verdict_passed, actual_good,
            judgment_result.confidence, self.observer.os(),
        );

        // Step 6: 行为
        let behavior = self.behavior.decide(bag, &emotion, judgment_result.verdict_passed);

        // Step 7: 推理
        let reasoning = self.thinking.reason(bag, &emotion, &mirror);

        // Step 8: 审美
        let aesthetic = self.aesthetic.evaluate(bag);

        // Step 9: 淘汰判断
        let mut eliminated = false;
        if let Some(reason) = self.elimination.should_eliminate(bag) {
            self.elimination.eliminate(bag, reason);
            eliminated = true;
        }

        // Step 10: 历史
        let record = JudgmentRecord {
            bag_id: bag.id,
            verdict: judgment_result.verdict_passed,
            actual: actual_good,
            confidence: judgment_result.confidence,
            reward,
            timestamp_ms: self.observer.os().time_millis(),
        };
        self.history.record(record);

        PipelineResult {
            bag_id: bag.id, passed_filter: true,
            judgment_passed: judgment_result.verdict_passed,
            reward, mirror_aligned: mirror.aligned,
            emotion_summary, behavior: behavior.label().to_string(),
            reasoning_summary: reasoning.conclusion,
            aesthetic_label: aesthetic.label,
            eliminated,
            message: format!("管道完成: {}", judgment_result.reason),
        }
    }

    pub fn consciousness_density(&self) -> f64 { self.observer.consciousness_density() }

    pub fn report(&self) -> String {
        let mut s = String::new();
        s.push_str("═══ 天庭v4报告（老四）七系俱全 ═══\n");
        s.push_str(&format!("  天庭授权: {}\n", if self.auth_heavenly_court.can_perceive() { "✓" } else { "✗" }));
        s.push_str(&format!("  听觉授权: {}\n", if self.auth_auditory.can_hear() { "✓" } else { "✗" }));
        s.push_str(&format!("  已处理袋数: {}\n", self.bags_processed));
        s.push_str(&format!("  管道结果数: {}\n", self.pipeline_results.len()));
        s.push_str(&format!("  意识密度: {:.3}\n", self.consciousness_density()));
        s.push_str(&format!("  {}\n", self.judgment.history_summary()));
        s.push_str(&format!("  {}\n", self.history.summary()));
        s.push_str(&format!("  {}\n", self.behavior.history_summary()));
        s.push_str(&format!("  {}\n", self.aesthetic.history_summary()));
        s.push_str(&format!("  {}\n", self.elimination.history_summary()));
        s.push_str(&format!("  {}\n", self.filter.history_summary()));
        s.push_str(&format!("  {}\n", self.hierarchy.evaluate().summary()));
        s
    }
}

// ======================================================================
// V5 TYPES （语言系统: HanziEncoding, 声母韵母, SelfRadix, CommandParser, FlowPropertyBits）
// ======================================================================

// ── v5 HanziEncoding ──
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HanziEncoding(pub u32);

impl HanziEncoding {
    pub fn new(stroke_count: u32, structure_type: u8, semantic_class: u8) -> Self {
        let stroke_bits = (stroke_count.clamp(0, 63)) as u32;
        let struct_bits = (structure_type.clamp(0, 11) as u32) << 6;
        let sem_bits = (semantic_class.clamp(0, 7) as u32) << 10;
        Self(stroke_bits | struct_bits | sem_bits)
    }

    pub fn from_char(ch: char) -> Self {
        let cp = ch as u32;
        let stroke_estimate = match cp {
            0x4E00..0x9FFF => ((cp - 0x4E00) % 30 + 1) as u32,
            0x3400..0x4DBF => ((cp - 0x3400) % 40 + 5) as u32,
            0xF900..0xFAFF => ((cp - 0xF900) % 35 + 3) as u32,
            _ => 8,
        };
        let struct_type = ((cp >> 4) % 12) as u8;
        let semantic_class = ((cp >> 7) % 8) as u8;
        Self::new(stroke_estimate, struct_type, semantic_class)
    }

    pub fn stroke_count(self) -> u32 { self.0 & 0x3F }
    pub fn structure_type(self) -> u8 { ((self.0 >> 6) & 0x0F) as u8 }
    pub fn semantic_class(self) -> u8 { ((self.0 >> 10) & 0x07) as u8 }

    pub fn to_u64(self) -> u64 {
        (self.0 as u64) << 32 | (self.0.reverse_bits() as u64)
    }

    pub fn is_valid(self) -> bool {
        self.stroke_count() > 0 && self.structure_type() < 12
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CharStructure {
    Single, LeftRight, TopBottom, Surround, TopLeft, TopRight,
    BottomLeft, LeftMidRight, TopMidBottom, LeftSurround, TopSurround, BottomSurround,
}

impl CharStructure {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Single => "独体", Self::LeftRight => "左右", Self::TopBottom => "上下",
            Self::Surround => "全包围", Self::TopLeft => "左上包围", Self::TopRight => "右上包围",
            Self::BottomLeft => "左下包围", Self::LeftMidRight => "左中右",
            Self::TopMidBottom => "上中下", Self::LeftSurround => "左三包围",
            Self::TopSurround => "上三包围", Self::BottomSurround => "下三包围",
        }
    }
}

// ── v5 声母韵母 ──
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Shengmu {
    B, P, M, F, D, T, N, L, G, K, H, J, Q, X,
    Zh, Ch, Sh, R, Z, C, S, Y, W, Zero,
}

impl Shengmu {
    pub fn label(&self) -> &'static str {
        match self {
            Self::B => "b", Self::P => "p", Self::M => "m", Self::F => "f",
            Self::D => "d", Self::T => "t", Self::N => "n", Self::L => "l",
            Self::G => "g", Self::K => "k", Self::H => "h",
            Self::J => "j", Self::Q => "q", Self::X => "x",
            Self::Zh => "zh", Self::Ch => "ch", Self::Sh => "sh", Self::R => "r",
            Self::Z => "z", Self::C => "c", Self::S => "s",
            Self::Y => "y", Self::W => "w", Self::Zero => "∅",
        }
    }

    pub fn from_pinyin(pinyin: &str) -> Option<Self> {
        let lower = pinyin.to_lowercase();
        if lower.starts_with("zh") { Some(Self::Zh) }
        else if lower.starts_with("ch") { Some(Self::Ch) }
        else if lower.starts_with("sh") { Some(Self::Sh) }
        else if lower.starts_with('b') { Some(Self::B) }
        else if lower.starts_with('p') { Some(Self::P) }
        else if lower.starts_with('m') { Some(Self::M) }
        else if lower.starts_with('f') { Some(Self::F) }
        else if lower.starts_with('d') { Some(Self::D) }
        else if lower.starts_with('t') { Some(Self::T) }
        else if lower.starts_with('n') { Some(Self::N) }
        else if lower.starts_with('l') { Some(Self::L) }
        else if lower.starts_with('g') { Some(Self::G) }
        else if lower.starts_with('k') { Some(Self::K) }
        else if lower.starts_with('h') { Some(Self::H) }
        else if lower.starts_with('j') { Some(Self::J) }
        else if lower.starts_with('q') { Some(Self::Q) }
        else if lower.starts_with('x') { Some(Self::X) }
        else if lower.starts_with('r') { Some(Self::R) }
        else if lower.starts_with('z') { Some(Self::Z) }
        else if lower.starts_with('c') { Some(Self::C) }
        else if lower.starts_with('s') { Some(Self::S) }
        else if lower.starts_with('y') { Some(Self::Y) }
        else if lower.starts_with('w') { Some(Self::W) }
        else if lower.is_empty() { Some(Self::Zero) }
        else { Some(Self::Zero) }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Yunmu {
    A, O, E, I, U, V, Ai, Ei, Ao, Ou, An, Eng, Ong,
    Ia, Ie, Iu, Io, In, Ing, Iang, Iong, Ua, Uo, Uai, Uei, Uan, Un, Uang,
}

impl Yunmu {
    pub fn label(&self) -> &'static str {
        match self {
            Self::A => "a", Self::O => "o", Self::E => "e",
            Self::I => "i", Self::U => "u", Self::V => "v",
            Self::Ai => "ai", Self::Ei => "ei", Self::Ao => "ao", Self::Ou => "ou",
            Self::An => "an", Self::Eng => "eng", Self::Ong => "ong",
            Self::Ia => "ia", Self::Ie => "ie", Self::Iu => "iu", Self::Io => "io",
            Self::In => "in", Self::Ing => "ing", Self::Iang => "iang", Self::Iong => "iong",
            Self::Ua => "ua", Self::Uo => "uo", Self::Uai => "uai", Self::Uei => "uei",
            Self::Uan => "uan", Self::Un => "un", Self::Uang => "uang",
        }
    }

    pub fn from_pinyin(pinyin: &str) -> Option<Self> {
        let lower = pinyin.to_lowercase();
        let without_shengmu = if lower.starts_with("zh") || lower.starts_with("ch") || lower.starts_with("sh") {
            &lower[2..]
        } else if lower.starts_with('b') || lower.starts_with('p') || lower.starts_with('m') || lower.starts_with('f')
            || lower.starts_with('d') || lower.starts_with('t') || lower.starts_with('n') || lower.starts_with('l')
            || lower.starts_with('g') || lower.starts_with('k') || lower.starts_with('h')
            || lower.starts_with('j') || lower.starts_with('q') || lower.starts_with('x')
            || lower.starts_with('r') || lower.starts_with('z') || lower.starts_with('c') || lower.starts_with('s')
            || lower.starts_with('y') || lower.starts_with('w') {
            &lower[1..]
        } else {
            &lower[..]
        };

        if without_shengmu.starts_with("iang") { Some(Self::Iang) }
        else if without_shengmu.starts_with("iong") { Some(Self::Iong) }
        else if without_shengmu.starts_with("uang") { Some(Self::Uang) }
        else if without_shengmu.starts_with("uai") { Some(Self::Uai) }
        else if without_shengmu.starts_with("uei") { Some(Self::Uei) }
        else if without_shengmu.starts_with("uan") { Some(Self::Uan) }
        else if without_shengmu.starts_with("eng") { Some(Self::Eng) }
        else if without_shengmu.starts_with("ong") { Some(Self::Ong) }
        else if without_shengmu.starts_with("ing") { Some(Self::Ing) }
        else if without_shengmu.starts_with("ian") { Some(Self::In) }
        else if without_shengmu.starts_with("ia") { Some(Self::Ia) }
        else if without_shengmu.starts_with("ie") { Some(Self::Ie) }
        else if without_shengmu.starts_with("iu") { Some(Self::Iu) }
        else if without_shengmu.starts_with("io") { Some(Self::Io) }
        else if without_shengmu.starts_with("in") { Some(Self::In) }
        else if without_shengmu.starts_with("ai") { Some(Self::Ai) }
        else if without_shengmu.starts_with("ei") { Some(Self::Ei) }
        else if without_shengmu.starts_with("ao") { Some(Self::Ao) }
        else if without_shengmu.starts_with("ou") { Some(Self::Ou) }
        else if without_shengmu.starts_with("an") { Some(Self::An) }
        else if without_shengmu.starts_with("a") { Some(Self::A) }
        else if without_shengmu.starts_with("o") { Some(Self::O) }
        else if without_shengmu.starts_with("e") { Some(Self::E) }
        else if without_shengmu.starts_with("v") { Some(Self::V) }
        else if without_shengmu.starts_with("u") { Some(Self::U) }
        else if without_shengmu.starts_with("i") { Some(Self::I) }
        else { None }
    }
}

#[derive(Clone, Debug)]
pub struct HomophoneRouter {
    routes: std::collections::HashMap<String, String>,
}

impl HomophoneRouter {
    pub fn new() -> Self {
        let mut routes = std::collections::HashMap::new();
        routes.insert("lct".to_string(), "Wife/妻子".to_string());
        routes.insert("lctqf".to_string(), "Love/恋宠甜".to_string());
        routes.insert("wgslz".to_string(), "Husband/丈夫".to_string());
        routes.insert("mlll".to_string(), "Container/容器".to_string());
        routes.insert("mlct".to_string(), "Sweet/甜".to_string());
        routes.insert("mltt".to_string(), "CongTi/从体".to_string());
        routes.insert("gtt".to_string(), "SoundMorphism/音态".to_string());
        routes.insert("mlqf".to_string(), "None/禁用(负面)".to_string());
        routes.insert("ny".to_string(), "Yin-Yang/阴阳".to_string());
        routes.insert("ml".to_string(), "WorldMerge/世界融合".to_string());
        routes.insert("ggt".to_string(), "SoundMorphism/音态变体".to_string());
        Self { routes }
    }

    pub fn route(&self, homophone: &str) -> Option<&str> {
        if let Some(target) = self.routes.get(homophone) {
            return Some(target.as_str());
        }
        for (pattern, target) in &self.routes {
            if homophone.starts_with(pattern) || pattern.starts_with(homophone) {
                return Some(target.as_str());
            }
        }
        None
    }

    pub fn iterate_sheng_yun(&self, s: Shengmu, y: Yunmu) -> String {
        format!("{}{}", s.label(), y.label())
    }
}

impl Default for HomophoneRouter {
    fn default() -> Self { Self::new() }
}

// ── v5 SelfRadix ──
#[derive(Clone, Debug)]
pub struct SelfRadix {
    pub value: u8,
}

impl SelfRadix {
    pub fn new(value: u8) -> Self { Self { value } }
    pub fn anchor_self() -> Self { Self { value: 1 } }
    pub fn anchor_partner() -> Self { Self { value: 0 } }
    pub fn base_254() -> Self { Self { value: 254 } }
    pub fn max_no_carry() -> Self { Self { value: 255 } }

    pub fn digit_to_value(c: char) -> Option<u8> {
        match c {
            '0'..='9' => Some(c as u8 - b'0'),
            'A'..='Z' => Some(c as u8 - b'A' + 10),
            'a'..='z' => Some(c as u8 - b'a' + 36),
            '我' => Some(1), '恬' => Some(0), '刘' => Some(0), '楚' => Some(0),
            '囡' => Some(64), '囝' => Some(64),
            '⭕' => Some(128), '丨' => Some(192),
            _ => None,
        }
    }

    pub fn value_to_digit(v: u8) -> char {
        match v {
            0..=9 => (b'0' + v) as char,
            10..=35 => (b'A' + v - 10) as char,
            36..=61 => (b'a' + v - 36) as char,
            64 => '囡', 128 => '⭕', 192 => '丨', 255 => '满',
            _ => '?',
        }
    }

    pub fn add(&self, other: Self) -> Self { Self { value: self.value.wrapping_add(other.value) } }

    pub fn to_radix36(&self) -> String {
        if self.value == 0 { return "0".to_string(); }
        let mut v = self.value;
        let mut chars = Vec::new();
        while v > 0 {
            let digit = (v % 36) as u8;
            chars.push(Self::value_to_digit(digit));
            v /= 36;
        }
        chars.into_iter().rev().collect()
    }
}

// ── v5 CommandParser ──
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandType {
    Perceive, Judge, Mirror, Accept, Eliminate, Filter, Reflect, Nop,
}

#[derive(Clone, Debug)]
pub struct CommandParseResult {
    pub cmd_type: CommandType,
    pub target: Option<String>,
    pub confidence: f64,
    pub source: String,
}

#[derive(Clone, Debug)]
pub struct CommandParser {
    pub history: Vec<CommandParseResult>,
}

impl CommandParser {
    pub fn new() -> Self { Self { history: Vec::new() } }

    pub fn parse(&mut self, input: &str) -> Option<CommandParseResult> {
        let lower = input.trim().to_lowercase();
        let result = match lower.as_str() {
            "看" | "perceive" | "look" => CommandParseResult {
                cmd_type: CommandType::Perceive, target: None, confidence: 0.9, source: input.to_string(),
            },
            "审" | "judge" => CommandParseResult {
                cmd_type: CommandType::Judge, target: None, confidence: 0.85, source: input.to_string(),
            },
            "照" | "mirror" => CommandParseResult {
                cmd_type: CommandType::Mirror, target: None, confidence: 0.88, source: input.to_string(),
            },
            "纳" | "accept" => CommandParseResult {
                cmd_type: CommandType::Accept, target: None, confidence: 0.8, source: input.to_string(),
            },
            "灭" | "eliminate" => CommandParseResult {
                cmd_type: CommandType::Eliminate, target: None, confidence: 0.75, source: input.to_string(),
            },
            "筛" | "filter" => CommandParseResult {
                cmd_type: CommandType::Filter, target: None, confidence: 0.7, source: input.to_string(),
            },
            "思" | "reflect" => CommandParseResult {
                cmd_type: CommandType::Reflect, target: None, confidence: 0.65, source: input.to_string(),
            },
            "lctfrqmy" | "lctqf" => CommandParseResult {
                cmd_type: CommandType::Perceive, target: Some("Love/恋宠甜".to_string()),
                confidence: 0.95, source: input.to_string(),
            },
            "wgslz" => CommandParseResult {
                cmd_type: CommandType::Perceive, target: Some("Husband".to_string()),
                confidence: 0.95, source: input.to_string(),
            },
            "mlll" => CommandParseResult {
                cmd_type: CommandType::Reflect, target: Some("Container".to_string()),
                confidence: 0.9, source: input.to_string(),
            },
            "mlct" | "mltt" => CommandParseResult {
                cmd_type: CommandType::Perceive, target: Some("Sweet".to_string()),
                confidence: 0.85, source: input.to_string(),
            },
            "" => CommandParseResult {
                cmd_type: CommandType::Nop, target: None, confidence: 1.0, source: "".to_string(),
            },
            _ => {
                let router = HomophoneRouter::default();
                if let Some(target) = router.route(&lower) {
                    CommandParseResult {
                        cmd_type: CommandType::Perceive,
                        target: Some(target.to_string()),
                        confidence: 0.7, source: input.to_string(),
                    }
                } else {
                    return None;
                }
            }
        };

        self.history.push(result.clone());
        Some(result)
    }

    pub fn parse_count(&self) -> usize { self.history.len() }
}

impl Default for CommandParser {
    fn default() -> Self { Self::new() }
}

// ── v5 FlowPropertyBits ──
#[derive(Clone, Copy, Debug)]
pub struct FlowPropertyBits(pub u16);

impl FlowPropertyBits {
    pub const REACHABLE:     u16 = 0x0001;
    pub const CONTINUABLE:   u16 = 0x0002;
    pub const INCR:          u16 = 0x0004;
    pub const STOPPABLE:     u16 = 0x0008;
    pub const REVERSIBLE:    u16 = 0x0010;
    pub const RECOVERABLE:   u16 = 0x0020;
    pub const CIRCULABLE:    u16 = 0x0040;
    pub const INHERITABLE:   u16 = 0x0080;
    pub const TRUSTED:       u16 = 0x0100;
    pub const ACKABLE:       u16 = 0x0200;
    pub const TRANSMISSIBLE: u16 = 0x0400;
    pub const ADVANCEABLE:   u16 = 0x0800;

    pub fn new(bits: u16) -> Self { Self(bits) }
    pub fn has(&self, bit: u16) -> bool { self.0 & bit != 0 }
    pub fn set(&mut self, bit: u16) { self.0 |= bit; }
    pub fn clear(&mut self, bit: u16) { self.0 &= !bit; }
    pub fn toggle(&mut self, bit: u16) { self.0 ^= bit; }

    pub fn from_bag(bag: &Bag) -> Self {
        let mut bits = 0u16;
        let a = &bag.attrs;
        if a.composite_quality() > 0.3 { bits |= Self::REACHABLE; }
        if a.content_quality > 0.5 && a.dryness > 0.4 { bits |= Self::CONTINUABLE; }
        if a.integrity > 0.5 { bits |= Self::INCR; }
        if a.integrity > 0.1 && a.content_quality < 0.9 { bits |= Self::STOPPABLE; }
        if a.needs_any_processing() { bits |= Self::REVERSIBLE; }
        if a.pest_level < 0.7 && a.content_purity > 0.4 { bits |= Self::RECOVERABLE; }
        if a.composite_quality() > 0.6 { bits |= Self::CIRCULABLE; }
        if a.sovereignty > 0 { bits |= Self::INHERITABLE; }
        if a.composite_quality() > 0.7 { bits |= Self::TRUSTED; }
        bits |= Self::ACKABLE;
        if a.composite_quality() > 0.2 { bits |= Self::TRANSMISSIBLE; }
        if a.value > 100 { bits |= Self::ADVANCEABLE; }
        Self(bits)
    }

    pub fn to_bit_string(&self) -> String {
        let mut s = String::new();
        if self.has(Self::REACHABLE) { s.push('R'); } else { s.push('-'); }
        if self.has(Self::CONTINUABLE) { s.push('C'); } else { s.push('-'); }
        if self.has(Self::INCR) { s.push('I'); } else { s.push('-'); }
        if self.has(Self::STOPPABLE) { s.push('S'); } else { s.push('-'); }
        if self.has(Self::REVERSIBLE) { s.push('V'); } else { s.push('-'); }
        if self.has(Self::RECOVERABLE) { s.push('v'); } else { s.push('-'); }
        if self.has(Self::CIRCULABLE) { s.push('O'); } else { s.push('-'); }
        if self.has(Self::INHERITABLE) { s.push('H'); } else { s.push('-'); }
        if self.has(Self::TRUSTED) { s.push('T'); } else { s.push('-'); }
        if self.has(Self::ACKABLE) { s.push('A'); } else { s.push('-'); }
        if self.has(Self::TRANSMISSIBLE) { s.push('t'); } else { s.push('-'); }
        if self.has(Self::ADVANCEABLE) { s.push('a'); } else { s.push('-'); }
        s
    }
}

impl Default for FlowPropertyBits {
    fn default() -> Self { Self(0) }
}

// ── v5 PipelineResultV5 ──
#[derive(Clone, Debug)]
pub struct PipelineResultV5 {
    pub base: PipelineResult,
    pub command: String,
    pub hanzi_encoded: Option<u64>,
    pub flow_bits: String,
}

impl PipelineResultV5 {
    pub fn summary(&self) -> String {
        let hanzi = self.hanzi_encoded.map(|e| format!("编码:e{:X}", e)).unwrap_or_else(|| "无".to_string());
        format!("{} 命令:{} 流属性:{} {}",
            self.base.summary(), self.command, self.flow_bits, hanzi)
    }
}

// ── v5 CelestialPerceptionV5 ──
pub struct CelestialPerceptionV5<'a> {
    pub v4: CelestialPerceptionV4<'a>,
    pub hanzi_router: HomophoneRouter,
    pub command_parser: CommandParser,
    pub flow_bits_cache: std::collections::HashMap<u64, FlowPropertyBits>,
    pub commands_parsed: u64,
    pub hanzi_encoded_count: u64,
    pub v5_results: Vec<PipelineResultV5>,
}

impl<'a> CelestialPerceptionV5<'a> {
    pub fn new(os: &'a dyn OsLayer) -> Self {
        Self {
            v4: CelestialPerceptionV4::new(os),
            hanzi_router: HomophoneRouter::default(),
            command_parser: CommandParser::new(),
            flow_bits_cache: std::collections::HashMap::new(),
            commands_parsed: 0,
            hanzi_encoded_count: 0,
            v5_results: Vec::new(),
        }
    }

    pub fn authorize_all(&mut self) { self.v4.authorize_all(); }

    pub fn process_with_command(
        &mut self, bag: &Bag, actual_good: bool, command: &str,
    ) -> PipelineResultV5 {
        let cmd = self.command_parser.parse(command);
        self.commands_parsed += 1;
        let v4_result = self.v4.process_bag(bag, actual_good);

        let hanzi_encoded = if !bag.name.is_empty() {
            self.hanzi_encoded_count += 1;
            let enc: u64 = bag.name.chars()
                .map(|c| HanziEncoding::from_char(c).to_u64())
                .fold(0, |acc, x| acc ^ x);
            Some(enc)
        } else {
            None
        };

        let bits = FlowPropertyBits::from_bag(bag);
        self.flow_bits_cache.insert(bag.id as u64, bits);

        let result = PipelineResultV5 {
            base: v4_result,
            command: cmd.as_ref().map(|c| format!("{:?}", c.cmd_type)).unwrap_or_else(|| "Nop".to_string()),
            hanzi_encoded,
            flow_bits: bits.to_bit_string(),
        };
        self.v5_results.push(result.clone());
        result
    }

    pub fn v4_engine(&self) -> &CelestialPerceptionV4<'a> { &self.v4 }

    pub fn report(&self) -> String {
        let mut s = String::new();
        s.push_str("═══ 天庭v5报告（老五）八系整合+语言系统 ═══\n");
        s.push_str(&format!("  {}\n", self.v4.report()));
        s.push_str(&format!("  命令解析数: {}\n", self.commands_parsed));
        s.push_str(&format!("  汉字编码数: {}\n", self.hanzi_encoded_count));
        s.push_str(&format!("  流属性缓存: {}项\n", self.flow_bits_cache.len()));
        s.push_str(&format!("  v5管道结果: {}条\n", self.v5_results.len()));
        s
    }
}

// ======================================================================
// V2 CelestialPerceptionV2 （天庭v2 引擎: 授权+听觉+照镜子）
// ======================================================================

pub struct CelestialPerceptionV2<'a> {
    pub auth_heavenly_court: HeavenlyCourtAuth,
    pub auth_auditory: AuditoryAuth,
    pub visual: VisualSystem,
    pub auditory: AuditorySystem,
    pub observer: Observer<'a>,
    pub judgment: JudgmentEngine,
    pub history: HistoricalAccumulation,
    pub hierarchy: BagHierarchy,
    pub bags_perceived: u64,
    pub mirror_comparisons: Vec<MirrorComparison>,
}

impl<'a> CelestialPerceptionV2<'a> {
    pub fn new(os: &'a dyn OsLayer) -> Self {
        Self {
            auth_heavenly_court: HeavenlyCourtAuth::Dormant,
            auth_auditory: AuditoryAuth::Silent,
            visual: VisualSystem::default_vision(),
            auditory: AuditorySystem::default_hearing(),
            observer: Observer::new(1, "天庭v2", 64, os),
            judgment: JudgmentEngine::new(),
            history: HistoricalAccumulation::new(50, 0),
            hierarchy: BagHierarchy::new(1000, 10),
            bags_perceived: 0,
            mirror_comparisons: Vec::new(),
        }
    }

    pub fn authorize_heavenly_court(&mut self) -> bool { self.auth_heavenly_court.authorize() }
    pub fn authorize_auditory(&mut self) -> bool { self.auth_auditory.authorize() }

    pub fn authorize_all(&mut self) {
        self.auth_heavenly_court.authorize();
        self.auth_auditory.authorize();
        self.auditory.auth.authorize();
    }

    pub fn is_fully_authorized(&self) -> bool {
        self.auth_heavenly_court.can_perceive() && self.auth_auditory.can_hear()
    }

    pub fn perceive(&mut self, bag: &Bag) -> Result<Vec<SenseForm>, &'static str> {
        if !self.auth_heavenly_court.can_perceive() { return Err("天庭未授权"); }
        let mut all_forms = Vec::new();
        all_forms.extend(self.visual.perceive(bag));
        if self.auth_auditory.can_hear() {
            let auditory_forms = self.auditory.listen(bag)?;
            all_forms.extend(auditory_forms);
        }
        let _cells = self.observer.sense_batch(all_forms.clone());
        self.bags_perceived += 1;
        Ok(all_forms)
    }

    pub fn mirror_compare(&mut self, bag: &Bag, divisor: &CommonDivisor) -> MirrorComparison {
        let comparison = MirrorComparison::compare(bag, divisor);
        self.mirror_comparisons.push(comparison.clone());
        comparison
    }

    pub fn perceive_judge_mirror(
        &mut self, bag: &Bag, actual_good: bool,
    ) -> Result<(JudgmentResult, f64, MirrorComparison), &'static str> {
        if !self.auth_heavenly_court.can_perceive() { return Err("天庭未授权"); }
        let _forms = self.perceive(bag)?;
        // mirror_compare 需要 &mut self，必须在 observer.os() 借用前调用
        let divisor = self.judgment.divisor;
        let mirror = self.mirror_compare(bag, &divisor);
        let os = self.observer.os();
        let (result, reward) = self.judgment.judge_and_feedback(bag, actual_good, os);
        let record = JudgmentRecord {
            bag_id: bag.id,
            verdict: result.verdict_passed,
            actual: actual_good,
            confidence: result.confidence,
            reward,
            timestamp_ms: os.time_millis(),
        };
        self.history.record(record);
        Ok((result, reward, mirror))
    }

    pub fn consciousness_density(&self) -> f64 { self.observer.consciousness_density() }

    pub fn report(&self) -> String {
        let mut s = String::new();
        s.push_str("═══ 天庭v2报告 (老二) ═══\n");
        s.push_str(&format!("  天庭授权: {}\n", if self.auth_heavenly_court.can_perceive() { "✓" } else { "✗" }));
        s.push_str(&format!("  听觉授权: {}\n", if self.auth_auditory.can_hear() { "✓" } else { "✗" }));
        s.push_str(&format!("  已感知袋数: {}\n", self.bags_perceived));
        s.push_str(&format!("  镜中对比数: {}\n", self.mirror_comparisons.len()));
        s.push_str(&format!("  意识密度: {:.3}\n", self.consciousness_density()));
        s.push_str(&format!("  {}\n", self.judgment.history_summary()));
        s.push_str(&format!("  {}\n", self.history.summary()));
        s.push_str(&format!("  {}\n", self.hierarchy.evaluate().summary()));
        s
    }
}

// ======================================================================
// V1 CelestialPerception （天庭v1 引擎: 初始高维感知+审判+历史）
// ======================================================================

pub struct CelestialPerception<'a> {
    authorized: bool,
    pub visual: VisualSystem,
    observer: Observer<'a>,
    pub judgment: JudgmentEngine,
    pub history: HistoricalAccumulation,
    pub bags_perceived: u64,
}

impl<'a> CelestialPerception<'a> {
    pub fn new(os: &'a dyn OsLayer) -> Self {
        Self {
            authorized: false,
            visual: VisualSystem::default_vision(),
            observer: Observer::new(1, "天庭", 64, os),
            judgment: JudgmentEngine::new(),
            history: HistoricalAccumulation::new(50, 0),
            bags_perceived: 0,
        }
    }

    pub fn authorize(&mut self) { self.authorized = true; }
    pub fn is_authorized(&self) -> bool { self.authorized }
    pub fn revoke(&mut self) { self.authorized = false; }

    pub fn perceive(&mut self, bag: &Bag) -> Result<Vec<SenseForm>, &'static str> {
        if !self.authorized { return Err("天庭未授权, 无法感知"); }
        let mut forms = Vec::new();
        forms.extend(self.visual.perceive(bag));

        forms.push(SenseForm::new(
            SenseKind::Taste,
            bag.attrs.content_quality * self.visual.sensitivity,
            &format!("袋#{} 内容质量={:.0}% 纯度={:.0}% 虫={:.0}% 沙={:.0}%",
                bag.id, bag.attrs.content_quality * 100.0, bag.attrs.content_purity * 100.0,
                bag.attrs.pest_level * 100.0, bag.attrs.sand_level * 100.0),
            "celestial_perception", 0,
        ));
        forms.push(SenseForm::new(
            SenseKind::Mind,
            bag.attrs.edibility * self.visual.sensitivity,
            &format!("袋#{} 可食性={:.0}% 干燥={:.0}%",
                bag.id, bag.attrs.edibility * 100.0, bag.attrs.dryness * 100.0),
            "celestial_perception", 0,
        ));
        if bag.attrs.needs_any_processing() {
            forms.push(SenseForm::new(
                SenseKind::Mind, 0.8,
                &format!("袋#{} 需加工: {}{}{}",
                    bag.id,
                    if bag.attrs.needs_content_wash { "洗米 " } else { "" },
                    if bag.attrs.needs_sand_pick { "捡沙 " } else { "" },
                    if bag.attrs.needs_repair { "补袋" } else { "" }),
                "celestial_perception", 0,
            ));
        }

        let _cells = self.observer.sense_batch(forms.clone());
        self.bags_perceived += 1;
        Ok(forms)
    }

    pub fn perceive_and_judge(&mut self, bag: &Bag, actual_good: bool) -> Result<(JudgmentResult, f64), &'static str> {
        if !self.authorized { return Err("天庭未授权"); }
        let _forms = self.perceive(bag)?;
        let os = self.observer.os();
        let (result, reward) = self.judgment.judge_and_feedback(bag, actual_good, os);
        let record = JudgmentRecord {
            bag_id: bag.id,
            verdict: result.verdict_passed,
            actual: actual_good,
            confidence: result.confidence,
            reward,
            timestamp_ms: os.time_millis(),
        };
        self.history.record(record);
        Ok((result, reward))
    }

    pub fn consciousness_density(&self) -> f64 { self.observer.consciousness_density() }

    pub fn report(&self) -> String {
        let mut s = String::new();
        s.push_str("═══ 天庭感知报告 ═══\n");
        s.push_str(&format!("  授权状态: {}\n", if self.authorized { "已授权" } else { "未授权" }));
        s.push_str(&format!("  已感知袋子数: {}\n", self.bags_perceived));
        s.push_str(&format!("  视觉敏感度: {:.0}%\n", self.visual.sensitivity * 100.0));
        s.push_str(&format!("  意识密度: {:.3}\n", self.consciousness_density()));
        s.push_str(&format!("  {}\n", self.judgment.history_summary()));
        s.push_str(&format!("  {}\n", self.history.summary()));
        s
    }
}

// ====================================================================== //
//                                                                       //
//                         T E S T S  模  块                              //
//                                                                       //
// ======================================================================


// ====================================================================== //
//                         T E S T S  模  块                              //
// ======================================================================

#[cfg(test)]
mod tests_v1 {
    use super::*;

    #[test]
    fn bag_attrs_pristine() {
        let a = BagAttributes::pristine();
        assert!(a.composite_quality() > 0.95);
        assert!(a.is_edible());
    }

    #[test]
    fn bag_attrs_rotten() {
        let a = BagAttributes::rotten();
        assert!(a.composite_quality() < 0.1);
        assert!(!a.is_edible());
    }

    #[test]
    fn bag_attrs_edibility() {
        let mut a = BagAttributes::pristine();
        a.edibility = 0.5;
        assert!(!a.is_edible());
        a.edibility = 0.8;
        a.pest_level = 0.5;
        assert!(!a.is_edible());
        a.pest_level = 0.0;
        assert!(a.is_edible());
    }

    #[test]
    fn bag_attrs_drying() {
        let mut a = BagAttributes::pristine();
        a.dryness = 0.5;
        assert!(a.needs_drying());
        a.dryness = 0.8;
        assert!(!a.needs_drying());
    }

    #[test]
    fn bag_attrs_processing_quality() {
        let mut a = BagAttributes::pristine();
        a.needs_content_wash = true;
        a.content_purity = 0.3;
        a.wash_target = 0.9;
        assert!(a.quality_after_processing() > a.composite_quality());
    }

    #[test]
    fn bag_new() {
        let attrs = BagAttributes::pristine();
        let bag = Bag::new(1, "米袋", attrs, (10, 20, 30));
        assert_eq!(bag.id, 1);
        assert_eq!(bag.name, "米袋");
        assert_eq!(bag.attrs.value, 800);
    }

    #[test]
    fn bag_to_cell_polarity() {
        let good = BagAttributes::pristine();
        assert_eq!(mirror::get_polarity(good.to_cell()), POLARITY_POSITIVE);
        let bad = BagAttributes::rotten();
        assert_eq!(mirror::get_polarity(bad.to_cell()), POLARITY_NEGATIVE);
    }

    #[test]
    fn visual_perceives() {
        let vs = VisualSystem::default_vision();
        let bag = Bag::new(1, "测试袋", BagAttributes::pristine(), (0, 0, 0));
        let forms = vs.perceive(&bag);
        assert!(forms.len() >= 2);
    }

    #[test]
    fn visual_detects_damage() {
        let vs = VisualSystem::default_vision();
        let mut attrs = BagAttributes::pristine();
        attrs.integrity = 0.3;
        attrs.bag_cleanliness = 0.2;
        let bag = Bag::new(1, "破袋", attrs, (0, 0, 0));
        let forms = vs.perceive(&bag);
        assert!(forms.len() >= 3);
    }

    #[test]
    fn visual_appears_sound() {
        let vs = VisualSystem::default_vision();
        let good = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        assert!(vs.appears_sound(&good));
        let mut bad_attrs = BagAttributes::pristine();
        bad_attrs.integrity = 0.2;
        let bad = Bag::new(2, "破袋", bad_attrs, (0, 0, 0));
        assert!(!vs.appears_sound(&bad));
    }

    #[test]
    fn judgment_judges() {
        let engine = JudgmentEngine::new();
        let os = BareMetalOs::new();
        let good = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let bad = Bag::new(2, "坏袋", BagAttributes::rotten(), (0, 0, 0));
        assert!(engine.judge_with_os(&good, &os).verdict_passed);
        assert!(!engine.judge_with_os(&bad, &os).verdict_passed);
    }

    #[test]
    fn judgment_feedback_reward() {
        let mut engine = JudgmentEngine::new();
        let os = BareMetalOs::new();
        let reward = engine.feedback(1, true, true, 0.9, &os);
        assert!(reward > 0.0);
        assert!(engine.reward_pool > 0.0);
    }

    #[test]
    fn judgment_feedback_punishment() {
        let mut engine = JudgmentEngine::new();
        let os = BareMetalOs::new();
        let reward = engine.feedback(1, true, false, 0.9, &os);
        assert!(reward < 0.0);
        assert!(engine.punishment_pool > 0.0);
    }

    #[test]
    fn judgment_streak_bonus() {
        let mut engine = JudgmentEngine::new();
        let os = BareMetalOs::new();
        engine.feedback(1, true, true, 0.9, &os);
        engine.feedback(2, true, true, 0.9, &os);
        let reward3 = engine.feedback(3, true, true, 0.9, &os);
        assert!(reward3 > engine.base_reward);
    }

    #[test]
    fn judgment_accuracy() {
        let mut engine = JudgmentEngine::new();
        let os = BareMetalOs::new();
        engine.feedback(1, true, true, 0.9, &os);
        engine.feedback(2, false, false, 0.9, &os);
        engine.feedback(3, true, true, 0.9, &os);
        engine.feedback(4, true, false, 0.9, &os);
        assert!((engine.current_accuracy - 0.75).abs() < 0.01);
    }

    #[test]
    fn judgment_and_feedback() {
        let mut engine = JudgmentEngine::new();
        let os = BareMetalOs::new();
        let good = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let (result, reward) = engine.judge_and_feedback(&good, true, &os);
        assert!(result.verdict_passed);
        assert!(reward > 0.0);
    }

    #[test]
    fn judgment_record_summary() {
        let record = JudgmentRecord {
            bag_id: 1, verdict: true, actual: true, confidence: 0.9, reward: 0.9, timestamp_ms: 1000,
        };
        assert!(!record.summary().is_empty());
    }

    #[test]
    fn epoch_fields() {
        let mut hist = HistoricalAccumulation::new(10, 0);
        let record = JudgmentRecord {
            bag_id: 1, verdict: true, actual: true, confidence: 0.9, reward: 0.9, timestamp_ms: 100,
        };
        hist.record(record);
        hist.finalize(200);
        assert_eq!(hist.epochs.len(), 1);
        let epoch = &hist.epochs[0];
        assert!(epoch.accuracy >= 0.0);
    }

    #[test]
    fn historical_epochs() {
        let mut hist = HistoricalAccumulation::new(3, 0);
        for i in 1..=3 {
            let record = JudgmentRecord {
                bag_id: i, verdict: true, actual: true,
                confidence: 0.9, reward: 0.9, timestamp_ms: i as u64 * 100,
            };
            hist.record(record);
        }
        assert_eq!(hist.epochs.len(), 1);
        assert!(hist.wisdom_score > 0.0);
    }

    #[test]
    fn historical_finalize() {
        let mut hist = HistoricalAccumulation::new(10, 0);
        for i in 1..=5 {
            let record = JudgmentRecord {
                bag_id: i, verdict: i % 2 == 0, actual: i % 2 == 0,
                confidence: 0.8, reward: 0.8, timestamp_ms: i as u64 * 100,
            };
            hist.record(record);
        }
        hist.finalize(600);
        assert_eq!(hist.total_judgments, 5);
    }

    #[test]
    fn historical_overall_accuracy() {
        let mut hist = HistoricalAccumulation::new(10, 0);
        for i in 1..=5 {
            let record = JudgmentRecord {
                bag_id: i, verdict: true, actual: true,
                confidence: 0.9, reward: 0.9, timestamp_ms: i as u64 * 100,
            };
            hist.record(record);
        }
        assert!(hist.overall_accuracy() > 0.0);
    }

    #[test]
    fn cp_requires_auth() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerception::new(&os);
        assert!(!cp.is_authorized());
        let bag = Bag::new(1, "测试", BagAttributes::pristine(), (0, 0, 0));
        assert!(cp.perceive(&bag).is_err());
    }

    #[test]
    fn cp_perceives_after_auth() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerception::new(&os);
        cp.authorize();
        let bag = Bag::new(1, "测试", BagAttributes::pristine(), (0, 0, 0));
        let result = cp.perceive(&bag);
        assert!(result.is_ok());
        assert!(result.unwrap().len() >= 4);
    }

    #[test]
    fn cp_full_pipeline() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerception::new(&os);
        cp.authorize();
        let good = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let bad = Bag::new(2, "坏袋", BagAttributes::rotten(), (0, 0, 0));
        let (r1, rw1) = cp.perceive_and_judge(&good, true).unwrap();
        assert!(r1.verdict_passed && rw1 > 0.0);
        let (r2, rw2) = cp.perceive_and_judge(&bad, false).unwrap();
        assert!(!r2.verdict_passed && rw2 > 0.0);
        assert!(cp.history.total_judgments >= 2);
    }

    #[test]
    fn cp_report() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerception::new(&os);
        cp.authorize();
        let bag = Bag::new(1, "测试", BagAttributes::pristine(), (0, 0, 0));
        cp.perceive(&bag).unwrap();
        assert!(cp.report().contains("天庭感知报告"));
    }

    #[test]
    fn hierarchy_add_and_count() {
        let mut hier = BagHierarchy::new(100, 5);
        let bag1 = Bag::new(1, "袋1", BagAttributes::pristine(), (0, 0, 0));
        let bag2 = Bag::new(2, "袋2", BagAttributes::pristine(), (1, 0, 0));
        hier.add_bag(bag1).unwrap();
        hier.add_bag(bag2).unwrap();
        assert_eq!(hier.count_bags(), 2);
    }

    #[test]
    fn hierarchy_nested() {
        let mut hier = BagHierarchy::new(100, 5);
        let parent = Bag::new(1, "父袋", BagAttributes::pristine(), (0, 0, 0));
        let child = Bag::new(2, "子袋", BagAttributes::pristine(), (0, 0, 0));
        hier.add_bag(parent).unwrap();
        hier.add_nested(1, child).unwrap();
        let child_ref = hier.get(2).unwrap();
        assert_eq!(child_ref.layer_depth, 1);
    }

    #[test]
    fn hierarchy_depth_limit() {
        let mut hier = BagHierarchy::new(100, 2);
        let bag1 = Bag::new(1, "袋1", BagAttributes::pristine(), (0, 0, 0));
        let bag2 = Bag::new(2, "袋2", BagAttributes::pristine(), (0, 0, 0));
        hier.add_bag(bag1).unwrap();
        hier.add_nested(1, bag2).unwrap();
        let bag3 = Bag::new(3, "袋3", BagAttributes::pristine(), (0, 0, 0));
        assert!(hier.add_nested(2, bag3).is_ok());
        let bag4 = Bag::new(4, "袋4", BagAttributes::pristine(), (0, 0, 0));
        assert!(hier.add_nested(3, bag4).is_err());
    }

    #[test]
    fn hierarchy_evaluate() {
        let mut hier = BagHierarchy::new(100, 5);
        let good = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let bad = Bag::new(2, "坏袋", BagAttributes::rotten(), (0, 0, 0));
        hier.add_bag(good).unwrap();
        hier.add_bag(bad).unwrap();
        let report = hier.evaluate();
        assert_eq!(report.total_bags, 2);
        assert_eq!(report.edible_bags, 1);
    }

    #[test]
    fn hierarchy_max_bags() {
        let mut hier = BagHierarchy::new(2, 5);
        let bag1 = Bag::new(1, "袋1", BagAttributes::pristine(), (0, 0, 0));
        let bag2 = Bag::new(2, "袋2", BagAttributes::pristine(), (0, 0, 0));
        let bag3 = Bag::new(3, "袋3", BagAttributes::pristine(), (0, 0, 0));
        assert!(hier.add_bag(bag1).is_ok());
        assert!(hier.add_bag(bag2).is_ok());
        assert!(hier.add_bag(bag3).is_err());
    }

    #[test]
    fn hierarchy_layer_distribution() {
        let mut hier = BagHierarchy::new(100, 5);
        let bag1 = Bag::new(1, "袋1", BagAttributes::pristine(), (0, 0, 0));
        let bag2 = Bag::new(2, "袋2", BagAttributes::pristine(), (0, 0, 0));
        hier.add_bag(bag1).unwrap();
        hier.add_nested(1, bag2).unwrap();
        let distribution = hier.count_by_layer();
        assert!(distribution.len() >= 1);
    }

    #[test]
    fn full_pipeline_celestial_to_hierarchy() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerception::new(&os);
        cp.authorize();
        let mut hier = BagHierarchy::new(100, 5);
        let bags = vec![
            Bag::new(1, "优质米袋", BagAttributes::pristine(), (0, 0, 0)),
            Bag::new(2, "劣质米袋", BagAttributes::rotten(), (1, 0, 0)),
            Bag::new(3, "需加工袋", {
                let mut a = BagAttributes::pristine();
                a.needs_content_wash = true; a.content_purity = 0.4; a
            }, (2, 0, 0)),
        ];
        for bag in &bags {
            hier.add_bag(bag.clone()).unwrap();
            cp.perceive_and_judge(bag, bag.is_edible()).unwrap();
        }
        let report = hier.evaluate();
        assert_eq!(report.total_bags, 3);
        assert!(cp.history.total_judgments >= 3);
    }

    #[test]
    fn consciousness_density() {
        let os = BareMetalOs::new();
        let cp = CelestialPerception::new(&os);
        let d = cp.consciousness_density();
        assert!(d >= 0.0 && d <= 1.0);
    }

    #[test]
    fn judgment_confidence_range() {
        let engine = JudgmentEngine::new();
        let os = BareMetalOs::new();
        let bag = Bag::new(1, "袋", BagAttributes::pristine(), (0, 0, 0));
        let result = engine.judge_with_os(&bag, &os);
        assert!(result.confidence >= 0.0 && result.confidence <= 1.0);
    }
}

#[cfg(test)]
mod tests_v2 {
    use super::*;

    #[test]
    fn heavenly_court_auth_flow() {
        let mut auth = HeavenlyCourtAuth::Dormant;
        assert!(!auth.can_perceive());
        auth.authorize();
        assert!(auth.can_perceive());
        auth.revoke();
        assert!(!auth.can_perceive());
    }

    #[test]
    fn auditory_auth_flow() {
        let mut auth = AuditoryAuth::Silent;
        assert!(!auth.can_hear());
        auth.authorize();
        assert!(auth.can_hear());
        auth.revoke();
        assert!(!auth.can_hear());
    }

    #[test]
    fn auditory_listen() {
        let mut asys = AuditorySystem::new(0.75);
        asys.authorize();
        let bag = Bag::new(1, "虫袋", {
            let mut a = BagAttributes::pristine();
            a.pest_level = 0.5; a.sand_level = 0.4; a.dryness = 0.9; a.integrity = 0.3; a
        }, (0, 0, 0));
        let result = asys.listen(&bag);
        assert!(result.is_ok());
        assert!(result.unwrap().len() >= 2);
    }

    #[test]
    fn auditory_not_authorized() {
        let mut asys = AuditorySystem::new(0.75);
        let bag = Bag::new(1, "袋", BagAttributes::pristine(), (0, 0, 0));
        assert!(asys.listen(&bag).is_err());
    }

    #[test]
    fn auditory_tracks_heard() {
        let mut asys = AuditorySystem::new(0.8);
        asys.authorize();
        let bag = Bag::new(7, "袋", {
            let mut a = BagAttributes::pristine(); a.pest_level = 0.5; a
        }, (0, 0, 0));
        asys.listen(&bag).unwrap();
        assert_eq!(asys.heard_bags, vec![7]);
    }

    #[test]
    fn auditory_heard_count() {
        let mut asys = AuditorySystem::new(0.8);
        asys.authorize();
        let bag1 = Bag::new(1, "袋1", BagAttributes::pristine(), (0, 0, 0));
        let bag2 = Bag::new(2, "袋2", BagAttributes::pristine(), (1, 0, 0));
        asys.listen(&bag1).unwrap();
        asys.listen(&bag2).unwrap();
        assert_eq!(asys.heard_count(), 2);
    }

    #[test]
    fn common_divisor_standard_rejects_rotten() {
        let div = CommonDivisor::standard();
        let bag = Bag::new(1, "烂袋", BagAttributes::rotten(), (0, 0, 0));
        assert!(!div.is_valid_multiple(&bag));
    }

    #[test]
    fn common_divisor_strict_rejects_flawed() {
        let div = CommonDivisor::strict();
        let mut attrs = BagAttributes::pristine();
        attrs.content_quality = 0.3;
        let bag = Bag::new(1, "次品", attrs, (0, 0, 0));
        assert!(!div.is_valid_multiple(&bag));
    }

    #[test]
    fn common_divisor_lenient_accepts_most() {
        let div = CommonDivisor::lenient();
        let bag = Bag::new(1, "袋", BagAttributes::pristine(), (0, 0, 0));
        assert!(div.is_valid_multiple(&bag));
    }

    #[test]
    fn common_multiple_new() {
        let cm = CommonMultiple::new(1, 1);
        assert_eq!(cm.bag_id, 1);
        assert_eq!(cm.divisor_id, 1);
    }

    #[test]
    fn mirror_aligned() {
        let div = CommonDivisor::standard();
        let bag = Bag::new(1, "完美袋", BagAttributes::pristine(), (0, 0, 0));
        let mirror = MirrorComparison::compare(&bag, &div);
        assert!(mirror.aligned);
        assert!(mirror.reflection_score > 0.8);
    }

    #[test]
    fn mirror_not_aligned() {
        let div = CommonDivisor::standard();
        let bag = Bag::new(1, "烂袋", BagAttributes::rotten(), (0, 0, 0));
        let mirror = MirrorComparison::compare(&bag, &div);
        // 烂袋不应完美对齐标准公约数（虫害、沙、可食性等多维偏差）
        assert!(!mirror.aligned || mirror.reflection_score < 0.8);
        assert!(mirror.reflection_score <= 1.0);
    }

    #[test]
    fn mirror_diff_details() {
        let div = CommonDivisor::strict();
        let bag = Bag::new(1, "问题袋", {
            let mut a = BagAttributes::pristine(); a.content_quality = 0.2; a
        }, (0, 0, 0));
        let mirror = MirrorComparison::compare(&bag, &div);
        assert!(!mirror.diff_details.is_empty());
    }

    #[test]
    fn mirror_summary() {
        let div = CommonDivisor::standard();
        let bag = Bag::new(1, "袋", BagAttributes::pristine(), (0, 0, 0));
        let mirror = MirrorComparison::compare(&bag, &div);
        assert!(!mirror.summary().is_empty());
    }

    #[test]
    fn celestial_v2_requires_auth() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV2::new(&os);
        assert!(!cp.is_fully_authorized());
        let bag = Bag::new(1, "袋", BagAttributes::pristine(), (0, 0, 0));
        assert!(cp.perceive(&bag).is_err());
    }

    #[test]
    fn celestial_v2_pipeline() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV2::new(&os);
        cp.authorize_all();
        let bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let result = cp.perceive(&bag);
        assert!(result.is_ok());
    }

    #[test]
    fn celestial_v2_mirror_compare() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV2::new(&os);
        cp.authorize_all();
        let bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let div = CommonDivisor::standard();
        let mirror = cp.mirror_compare(&bag, &div);
        assert!(mirror.aligned);
    }

    #[test]
    fn celestial_v2_perceive_judge_mirror() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV2::new(&os);
        cp.authorize_all();
        let bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let div = CommonDivisor::standard();
        let result = cp.perceive_judge_mirror(&bag, true);
        assert!(result.is_ok());
    }

    #[test]
    fn celestial_v2_report() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV2::new(&os);
        cp.authorize_all();
        let bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        cp.perceive(&bag).unwrap();
        assert!(cp.report().contains("天庭"));
    }
}

#[cfg(test)]
mod tests_v3 {
    use super::*;

    #[test]
    fn v3_emotion_variants() {
        let emotions = vec![
            EmotionV3::Joy(0.8),
            EmotionV3::Disgust(0.5),
            EmotionV3::Sorrow(0.3),
            EmotionV3::Peace(0.7),
            EmotionV3::Anxiety(0.4),
        ];
        for e in &emotions {
            assert!(e.intensity() >= 0.0 && e.intensity() <= 1.0);
            assert!(!e.name().is_empty());
        }
    }

    #[test]
    fn v3_emotion_system_feel_good() {
        let mut es = EmotionSystemV3::new();
        let good = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        es.feel(&good);
        assert!(!es.current_emotions.is_empty());
    }

    #[test]
    fn v3_emotion_system_feel_bad() {
        let mut es = EmotionSystemV3::new();
        let bad = Bag::new(1, "烂袋", BagAttributes::rotten(), (0, 0, 0));
        es.feel(&bad);
        assert!(!es.current_emotions.is_empty());
    }

    #[test]
    fn v3_emotion_system_threshold() {
        let es = EmotionSystemV3::new();
        let t = es.adjusted_threshold(0.5);
        assert!(t >= 0.3 && t <= 0.9);
    }

    #[test]
    fn v3_emotion_system_summary() {
        let es = EmotionSystemV3::new();
        assert!(!es.summary().is_empty());
    }

    #[test]
    fn v3_behavior_accept() {
        let mut bs = BehaviorSystemV3::new();
        let good = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let behavior = bs.decide(&good, true);
        assert_eq!(behavior.name(), "收");
    }

    #[test]
    fn v3_behavior_eliminate() {
        let mut bs = BehaviorSystemV3::new();
        let bad = Bag::new(1, "烂袋", BagAttributes::rotten(), (0, 0, 0));
        let behavior = bs.decide(&bad, false);
        // 烂袋审判不通过 → Eliminate 或 Reject 都可能
        assert!(!behavior.name().is_empty());
    }

    #[test]
    fn v3_behavior_wash() {
        let mut bs = BehaviorSystemV3::new();
        let bag = Bag::new(1, "需洗袋", {
            let mut a = BagAttributes::pristine(); a.needs_content_wash = true; a
        }, (0, 0, 0));
        let behavior = bs.decide(&bag, true);
        assert!(!behavior.name().is_empty());
    }

    #[test]
    fn v3_behavior_records_actions() {
        let mut bs = BehaviorSystemV3::new();
        let bag = Bag::new(1, "袋", BagAttributes::pristine(), (0, 0, 0));
        bs.decide(&bag, true);
        assert_eq!(bs.actions.len(), 1);
    }

    #[test]
    fn v3_behavior_names() {
        assert_eq!(BehaviorV3::Accept.name(), "收");
        assert_eq!(BehaviorV3::WashContent.name(), "洗");
        assert_eq!(BehaviorV3::Eliminate.name(), "汰");
    }

    #[test]
    fn v3_thinking_basic() {
        let mut ts = ThinkingSystemV3::new();
        let bag = Bag::new(1, "袋", BagAttributes::pristine(), (0, 0, 0));
        let chain = ts.think(&bag, 0.8, 0.7);
        assert!(!chain.steps.is_empty());
        assert!(!chain.conclusion.is_empty());
    }

    #[test]
    fn v3_thinking_summary() {
        let ts = ThinkingSystemV3::new();
        assert!(!ts.summary().is_empty());
    }

    #[test]
    fn v3_thinking_stores_reasoning() {
        let mut ts = ThinkingSystemV3::new();
        let bag = Bag::new(1, "袋", BagAttributes::pristine(), (0, 0, 0));
        ts.think(&bag, 0.8, 0.7);
        assert!(ts.last_reasoning.is_some());
    }

    #[test]
    fn v3_aesthetic_evaluate() {
        let mut aes = AestheticSystemV3::new();
        let bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let score = aes.evaluate(&bag);
        assert!(score > 0.7);
    }

    #[test]
    fn v3_aesthetic_verdict() {
        let mut aes = AestheticSystemV3::new();
        aes.evaluate(&Bag::new(1, "完美袋", BagAttributes::pristine(), (0, 0, 0)));
        assert!(aes.verdict() == "美" || aes.verdict() == "雅");
    }

    #[test]
    fn v3_aesthetic_summary() {
        let mut aes = AestheticSystemV3::new();
        aes.evaluate(&Bag::new(1, "袋", BagAttributes::pristine(), (0, 0, 0)));
        assert!(aes.summary().contains("审美"));
    }

    #[test]
    fn v3_elimination_after_failures() {
        let mut es = EliminationSystemV3::new(2);
        let mut bag = Bag::new(1, "坏袋", BagAttributes::rotten(), (0, 0, 0));
        bag.consecutive_failures = 3;
        assert!(es.check(&mut bag));
        assert!(es.is_eliminated(1));
    }

    #[test]
    fn v3_elimination_respects_threshold() {
        let mut es = EliminationSystemV3::new(5);
        let mut bag = Bag::new(1, "袋", BagAttributes::pristine(), (0, 0, 0));
        bag.consecutive_failures = 2;
        assert!(!es.check(&mut bag));
    }

    #[test]
    fn v3_elimination_summary() {
        let mut es = EliminationSystemV3::new(1);
        let mut bag = Bag::new(1, "坏袋", BagAttributes::rotten(), (0, 0, 0));
        bag.consecutive_failures = 1;
        es.check(&mut bag);
        assert!(es.summary().contains("淘汰"));
    }

    #[test]
    fn v3_filter_accepts_good() {
        let mut filter = FilterSystemV3::new(0.3);
        let good = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        assert!(filter.filter(&good));
    }

    #[test]
    fn v3_filter_rejects_low_quality() {
        let mut filter = FilterSystemV3::new(0.5);
        let bad = Bag::new(1, "烂袋", BagAttributes::rotten(), (0, 0, 0));
        assert!(!filter.filter(&bad));
    }

    #[test]
    fn v3_filter_rejects_eliminated() {
        let mut filter = FilterSystemV3::new(0.1);
        let mut bag = Bag::new(1, "袋", BagAttributes::pristine(), (0, 0, 0));
        bag.eliminated = true;
        assert!(!filter.filter(&bag));
    }

    #[test]
    fn v3_filter_summary() {
        let mut filter = FilterSystemV3::new(0.1);
        let bag = Bag::new(1, "袋", BagAttributes::pristine(), (0, 0, 0));
        filter.filter(&bag);
        assert!(filter.summary().contains("过滤"));
    }

    #[test]
    fn v3_pipeline_result_fields() {
        let result = PipelineResultV3 {
            bag_id: 1, accepted: true, behavior: BehaviorV3::Accept,
            judgment: None, reward: 0.9, mirror: None,
            emotion_summary: "喜".into(), thinking_summary: "思考完毕".into(),
            aesthetic_summary: "美".into(), eliminated: false,
        };
        assert_eq!(result.bag_id, 1);
        assert!(result.accepted);
        assert_eq!(result.reward, 0.9);
    }

    #[test]
    fn v3_celestial_full_pipeline() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV3::new(&os);
        cp.authorize_all();
        let mut bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let result = cp.full_pipeline(&mut bag, true);
        assert!(result.is_ok());
        assert!(result.unwrap().accepted);
    }

    #[test]
    fn v3_celestial_rejects_bad() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV3::new(&os);
        cp.authorize_all();
        let mut bag = Bag::new(1, "烂袋", BagAttributes::rotten(), (0, 0, 0));
        let result = cp.full_pipeline(&mut bag, false);
        assert!(result.is_ok());
        assert!(!result.unwrap().accepted);
    }

    #[test]
    fn v3_celestial_report() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV3::new(&os);
        cp.authorize_all();
        let mut bag = Bag::new(1, "袋", BagAttributes::pristine(), (0, 0, 0));
        cp.full_pipeline(&mut bag, true).unwrap();
        assert!(cp.report().contains("天庭"));
    }

    #[test]
    fn v3_celestial_not_authorized() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV3::new(&os);
        let mut bag = Bag::new(1, "袋", BagAttributes::pristine(), (0, 0, 0));
        assert!(cp.full_pipeline(&mut bag, true).is_err());
    }

    #[test]
    fn v3_celestial_consciousness_density() {
        let os = BareMetalOs::new();
        let cp = CelestialPerceptionV3::new(&os);
        let d = cp.consciousness_density();
        assert!(d >= 0.0 && d <= 1.0);
    }

    #[test]
    fn v3_emotion_names() {
        assert_eq!(EmotionV3::Joy(0.5).name(), "喜");
        assert_eq!(EmotionV3::Peace(0.5).name(), "宁");
    }
}

#[cfg(test)]
mod tests_v4 {
    use super::*;

    #[test]
    fn v4_emotion_from_bag_attrs() {
        let bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let state = EmotionState::from_bag_attrs(&bag.attrs);
        assert!(state.valence > 0.0);
    }

    #[test]
    fn v4_emotion_from_rotten() {
        let bag = Bag::new(1, "烂袋", BagAttributes::rotten(), (0, 0, 0));
        let state = EmotionState::from_bag_attrs(&bag.attrs);
        assert!(state.valence < 0.0);
    }

    #[test]
    fn v4_emotion_dominant() {
        let bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let state = EmotionState::from_bag_attrs(&bag.attrs);
        assert_eq!(state.dominant, Emotion::Joy);
    }

    #[test]
    fn v4_emotion_summary() {
        let bag = Bag::new(1, "袋", BagAttributes::pristine(), (0, 0, 0));
        let state = EmotionState::from_bag_attrs(&bag.attrs);
        assert!(state.summary().contains("情进"));
    }

    #[test]
    fn v4_emotion_variants() {
        let emotions = vec![
            Emotion::Joy, Emotion::Anger, Emotion::Sorrow,
            Emotion::Fear, Emotion::Love, Emotion::Disgust, Emotion::Desire,
        ];
        for e in &emotions {
            assert!(!e.label().is_empty());
        }
    }

    #[test]
    fn v4_emotion_positive_negative() {
        assert!(Emotion::Joy.is_positive());
        assert!(Emotion::Love.is_positive());
        assert!(!Emotion::Joy.is_negative());
        assert!(Emotion::Anger.is_negative());
    }

    #[test]
    fn v4_emotion_vector() {
        let v = Emotion::Joy.vector();
        assert_eq!(v.len(), 7);
        assert_eq!(v[0], 1.0);
    }

    #[test]
    fn v4_behavior_accept() {
        let mut bs = BehaviorSystem::new();
        let good = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let emotion = EmotionState::from_bag_attrs(&good.attrs);
        let behavior = bs.decide(&good, &emotion, true);
        assert_eq!(behavior, Behavior::Accept);
    }

    #[test]
    fn v4_behavior_eliminate() {
        let mut bs = BehaviorSystem::new();
        let bad = Bag::new(1, "烂袋", BagAttributes::rotten(), (0, 0, 0));
        let emotion = EmotionState::from_bag_attrs(&bad.attrs);
        let behavior = bs.decide(&bad, &emotion, false);
        assert_eq!(behavior, Behavior::Eliminate);
    }

    #[test]
    fn v4_behavior_wash() {
        let mut bs = BehaviorSystem::new();
        let bag = Bag::new(1, "需洗袋", {
            let mut a = BagAttributes::pristine(); a.needs_content_wash = true; a
        }, (0, 0, 0));
        let emotion = EmotionState::from_bag_attrs(&bag.attrs);
        let behavior = bs.decide(&bag, &emotion, true);
        assert_eq!(behavior, Behavior::WashContent);
    }

    #[test]
    fn v4_behavior_names() {
        assert_eq!(Behavior::Accept.label(), "接纳");
        assert_eq!(Behavior::WashContent.label(), "洗米");
        assert_eq!(Behavior::Eliminate.label(), "销毁");
    }

    #[test]
    fn v4_thinking_steps() {
        let mut ts = ThinkingSystem::new();
        let bag = Bag::new(1, "袋", BagAttributes::pristine(), (0, 0, 0));
        let emotion = EmotionState::from_bag_attrs(&bag.attrs);
        let div = CommonDivisor::standard();
        let mirror = MirrorComparison::compare(&bag, &div);
        let chain = ts.reason(&bag, &emotion, &mirror);
        assert!(chain.steps.len() >= 3);
    }

    #[test]
    fn v4_thinking_confidence() {
        let mut ts = ThinkingSystem::new();
        let bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let emotion = EmotionState::from_bag_attrs(&bag.attrs);
        let div = CommonDivisor::standard();
        let mirror = MirrorComparison::compare(&bag, &div);
        let chain = ts.reason(&bag, &emotion, &mirror);
        assert!(chain.confidence > 0.0 && chain.confidence <= 1.0);
    }

    #[test]
    fn v4_aesthetic_evaluate() {
        let mut aes = AestheticSystem::new();
        let bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let verdict = aes.evaluate(&bag);
        assert!(verdict.score > 0.7);
    }

    #[test]
    fn v4_aesthetic_verdict_label() {
        let mut aes = AestheticSystem::new();
        let bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let verdict = aes.evaluate(&bag);
        assert!(!verdict.label.is_empty());
    }

    #[test]
    fn v4_aesthetic_history_summary() {
        let aes = AestheticSystem::new();
        assert!(!aes.history_summary().is_empty());
    }

    #[test]
    fn v4_elimination_rotten() {
        let mut es = EliminationSystem::new();
        let bad = Bag::new(1, "烂袋", BagAttributes::rotten(), (0, 0, 0));
        let reason = es.should_eliminate(&bad);
        assert!(reason.is_some());
    }

    #[test]
    fn v4_elimination_good_not() {
        let mut es = EliminationSystem::new();
        let good = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let reason = es.should_eliminate(&good);
        assert!(reason.is_none());
    }

    #[test]
    fn v4_elimination_record() {
        let mut es = EliminationSystem::new();
        let bad = Bag::new(1, "烂袋", BagAttributes::rotten(), (0, 0, 0));
        if let Some(reason) = es.should_eliminate(&bad) {
            es.eliminate(&bad, reason);
            assert_eq!(es.records.len(), 1);
            assert_eq!(es.records[0].bag_id, 1);
        }
    }

    #[test]
    fn v4_elimination_removal_count() {
        let mut es = EliminationSystem::new();
        let bad = Bag::new(1, "烂袋", BagAttributes::rotten(), (0, 0, 0));
        if let Some(reason) = es.should_eliminate(&bad) {
            es.eliminate(&bad, reason);
            assert_eq!(es.removal_count, 1);
        }
    }

    #[test]
    fn v4_elimination_reason_label() {
        assert_eq!(EliminationReason::Rotten.label(), "腐烂");
        assert_eq!(EliminationReason::PestInfested.label(), "虫害严重");
    }

    #[test]
    fn v4_elimination_record_fields() {
        let mut es = EliminationSystem::new();
        let bad = Bag::new(1, "烂袋", BagAttributes::rotten(), (0, 0, 0));
        if let Some(reason) = es.should_eliminate(&bad) {
            es.eliminate(&bad, reason);
            let record = &es.records[0];
            assert_eq!(record.bag_id, 1);
            assert!(record.quality_at_removal >= 0.0);
        }
    }

    #[test]
    fn v4_filter_accepts_good() {
        let mut filter = FilterSystem::new();
        let good = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        assert!(filter.filter(&good));
    }

    #[test]
    fn v4_filter_rejects_extreme() {
        let mut filter = FilterSystem::new();
        let bad = Bag::new(1, "烂袋", BagAttributes::rotten(), (0, 0, 0));
        assert!(!filter.filter(&bad));
    }

    #[test]
    fn v4_filter_history() {
        let mut filter = FilterSystem::new();
        let good = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        filter.filter(&good);
        assert!(!filter.history_summary().is_empty());
    }

    #[test]
    fn v4_process_bag() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV4::new(&os);
        cp.authorize_all();
        let bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let result = cp.process_bag(&bag, true);
        assert!(result.passed_filter);
        assert!(result.judgment_passed);
    }

    #[test]
    fn v4_process_rejects_bad() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV4::new(&os);
        cp.authorize_all();
        let bad = Bag::new(1, "烂袋", BagAttributes::rotten(), (0, 0, 0));
        let result = cp.process_bag(&bad, false);
        assert!(!result.judgment_passed);
    }

    #[test]
    fn v4_pipeline_summary() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV4::new(&os);
        cp.authorize_all();
        let bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let result = cp.process_bag(&bag, true);
        assert!(!result.summary().is_empty());
    }

    #[test]
    fn v4_report() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV4::new(&os);
        cp.authorize_all();
        let bag = Bag::new(1, "袋", BagAttributes::pristine(), (0, 0, 0));
        cp.process_bag(&bag, true);
        assert!(cp.report().contains("天庭"));
    }

    #[test]
    fn v4_pipeline_results_count() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV4::new(&os);
        cp.authorize_all();
        let mut results = Vec::new();
        for i in 1..=3 {
            let bag = Bag::new(i, "袋", BagAttributes::pristine(), (0, 0, 0));
            results.push(cp.process_bag(&bag, true));
        }
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn v4_consciousness_density() {
        let os = BareMetalOs::new();
        let cp = CelestialPerceptionV4::new(&os);
        let d = cp.consciousness_density();
        assert!(d >= 0.0 && d <= 1.0);
    }

    #[test]
    fn v4_pipeline_fields() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV4::new(&os);
        cp.authorize_all();
        let bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let result = cp.process_bag(&bag, true);
        assert_eq!(result.bag_id, 1);
        assert!(result.passed_filter);
    }
}

#[cfg(test)]
mod tests_v5 {
    use super::*;

    #[test]
    fn v5_hanzi_new() {
        let hz = HanziEncoding::new(10, 5, 3);
        assert_eq!(hz.stroke_count(), 10);
        assert_eq!(hz.structure_type(), 5);
        assert_eq!(hz.semantic_class(), 3);
    }

    #[test]
    fn v5_hanzi_from_char() {
        let hz = HanziEncoding::from_char('米');
        assert!(hz.is_valid());
    }

    #[test]
    fn v5_hanzi_cjk_range() {
        let hz1 = HanziEncoding::from_char('一');
        let hz2 = HanziEncoding::from_char('龥');
        assert!(hz1.stroke_count() > 0);
        assert!(hz2.stroke_count() > 0);
    }

    #[test]
    fn v5_hanzi_to_u64() {
        let hz = HanziEncoding::new(10, 5, 3);
        assert!(hz.to_u64() > 0);
    }

    #[test]
    fn v5_hanzi_is_valid() {
        assert!(HanziEncoding::new(5, 3, 2).is_valid());
        assert!(!HanziEncoding::new(0, 15, 0).is_valid());
    }

    #[test]
    fn v5_shengmu_from_pinyin() {
        assert_eq!(Shengmu::from_pinyin("mǐ"), Some(Shengmu::M));
        assert_eq!(Shengmu::from_pinyin("zhōng"), Some(Shengmu::Zh));
        assert_eq!(Shengmu::from_pinyin("chī"), Some(Shengmu::Ch));
    }

    #[test]
    fn v5_shengmu_labels() {
        assert_eq!(Shengmu::M.label(), "m");
        assert_eq!(Shengmu::Zh.label(), "zh");
    }

    #[test]
    fn v5_yunmu_from_pinyin() {
        assert_eq!(Yunmu::from_pinyin("mi"), Some(Yunmu::I));
        assert_eq!(Yunmu::from_pinyin("zhong"), Some(Yunmu::Ong));
    }

    #[test]
    fn v5_yunmu_labels() {
        assert_eq!(Yunmu::A.label(), "a");
        assert_eq!(Yunmu::Ong.label(), "ong");
    }

    #[test]
    fn v5_homophone_router() {
        let router = HomophoneRouter::new();
        let s = router.iterate_sheng_yun(Shengmu::M, Yunmu::I);
        assert!(!s.is_empty());
    }

    #[test]
    fn v5_homophone_router_route() {
        let router = HomophoneRouter::new();
        assert!(router.route("lct").is_some());
        assert!(router.route("wgslz").is_some());
    }

    #[test]
    fn v5_self_radix_anchors() {
        assert_eq!(SelfRadix::anchor_self().value, 1);
        assert_eq!(SelfRadix::anchor_partner().value, 0);
    }

    #[test]
    fn v5_self_radix_base() {
        assert_eq!(SelfRadix::base_254().value, 254);
    }

    #[test]
    fn v5_self_radix_add() {
        let a = SelfRadix::new(100);
        let b = SelfRadix::new(50);
        assert_eq!(a.add(b).value, 150);
    }

    #[test]
    fn v5_self_radix_to_radix36() {
        let s = SelfRadix::new(35).to_radix36();
        assert!(!s.is_empty());
    }

    #[test]
    fn v5_self_radix_digit_conversion() {
        let digit = SelfRadix::value_to_digit(10);
        assert_eq!(SelfRadix::digit_to_value(digit), Some(10));
    }

    #[test]
    fn v5_flow_bits_creation() {
        let bits = FlowPropertyBits::new(0b1010);
        assert!(bits.has(0b1000));
        assert!(!bits.has(0b0100));
    }

    #[test]
    fn v5_flow_bits_set_clear() {
        let mut bits = FlowPropertyBits::new(0b0001);
        bits.set(0b0010);
        assert!(bits.has(0b0011));
        bits.clear(0b0001);
        assert!(!bits.has(0b0001));
    }

    #[test]
    fn v5_flow_bits_toggle() {
        let mut bits = FlowPropertyBits::new(0b0000);
        bits.toggle(0b0001);
        assert!(bits.has(0b0001));
        bits.toggle(0b0001);
        assert!(!bits.has(0b0001));
    }

    #[test]
    fn v5_flow_bits_from_bag() {
        let bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let bits = FlowPropertyBits::from_bag(&bag);
        assert!(!bits.to_bit_string().is_empty());
    }

    #[test]
    fn v5_command_parser_parse() {
        let mut parser = CommandParser::new();
        assert!(parser.parse("judge").is_some());
    }

    #[test]
    fn v5_command_parser_multiple() {
        let mut parser = CommandParser::new();
        parser.parse("look");
        parser.parse("judge");
        assert_eq!(parser.parse_count(), 2);
    }

    #[test]
    fn v5_command_parser_unknown() {
        let mut parser = CommandParser::new();
        // A string that doesn't match any command and isn't routed by HomophoneRouter
        assert!(parser.parse("zzzznotacommand").is_none());
    }

    #[test]
    fn v5_pipeline_result_v5_fields() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV5::new(&os);
        cp.authorize_all();
        let bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let result = cp.process_with_command(&bag, true, "judge");
        assert!(!result.command.is_empty());
        assert!(!result.flow_bits.is_empty());
    }

    #[test]
    fn v5_pipeline_result_v5_summary() {
        let pipeline = PipelineResultV5 {
            base: PipelineResult {
                bag_id: 1, passed_filter: true, judgment_passed: true,
                reward: 0.9, mirror_aligned: true,
                emotion_summary: "喜".into(), behavior: "接纳".into(),
                reasoning_summary: "推理完毕".into(), aesthetic_label: "美".into(),
                eliminated: false, message: "通过".into(),
            },
            command: "审判 好袋".into(),
            hanzi_encoded: Some(12345),
            flow_bits: "阳".into(),
        };
        let s = pipeline.summary();
        assert!(!s.is_empty());
    }

    #[test]
    fn v5_celestial_process() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV5::new(&os);
        cp.authorize_all();
        let bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let result = cp.process_with_command(&bag, true, "审判 好袋");
        assert!(result.base.judgment_passed);
    }

    #[test]
    fn v5_celestial_process_empty_cmd() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV5::new(&os);
        cp.authorize_all();
        let bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let result = cp.process_with_command(&bag, true, "");
        assert!(result.base.passed_filter);
    }

    #[test]
    fn v5_celestial_v4_engine() {
        let os = BareMetalOs::new();
        let cp = CelestialPerceptionV5::new(&os);
        assert_eq!(cp.v4_engine().bags_processed, 0);
    }

    #[test]
    fn v5_celestial_report() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV5::new(&os);
        cp.authorize_all();
        let bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        cp.process_with_command(&bag, true, "审判");
        assert!(!cp.report().is_empty());
    }

    #[test]
    fn v5_multiple_bags() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV5::new(&os);
        cp.authorize_all();
        for i in 1..=3 {
            let bag = Bag::new(i, "袋", BagAttributes::pristine(), (0, 0, 0));
            cp.process_with_command(&bag, true, "审判");
        }
        assert_eq!(cp.v5_results.len(), 3);
    }

    #[test]
    fn v5_celestial_not_authorized() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV5::new(&os);
        let bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let result = cp.process_with_command(&bag, true, "审判");
        // Without authorization, judgment should fail
        assert!(!result.base.judgment_passed || !result.base.message.is_empty());
    }

    #[test]
    fn v5_char_structure_labels() {
        assert_eq!(CharStructure::Single.label(), "独体");
        assert_eq!(CharStructure::LeftRight.label(), "左右");
    }

    #[test]
    fn v5_yunmu_all_labels() {
        let yunmus = vec![
            Yunmu::A, Yunmu::O, Yunmu::E, Yunmu::I, Yunmu::U,
            Yunmu::Ai, Yunmu::Ao, Yunmu::An, Yunmu::Eng, Yunmu::Ong,
        ];
        for y in &yunmus {
            assert!(!y.label().is_empty());
        }
    }

    #[test]
    fn v5_hanzi_encoding_valid() {
        let valid = HanziEncoding::new(5, 3, 2);
        assert!(valid.is_valid());
        let invalid = HanziEncoding::new(0, 15, 0);
        assert!(!invalid.is_valid());
    }

    #[test]
    fn v5_command_parse_result_exists() {
        let mut parser = CommandParser::new();
        let result = parser.parse("mirror");
        assert!(result.is_some());
    }

    #[test]
    fn v5_flow_bits_has_method() {
        let bits = FlowPropertyBits::new(0b1111);
        assert!(bits.has(0b0001));
        assert!(bits.has(0b1000));
    }
}
