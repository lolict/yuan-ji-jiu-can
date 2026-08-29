// ══════════════════════════════════════════════════════════════════
// celestial_perception_v2.rs — 天觉v2（老二）
// 视觉 + 听觉授权 + 审判奖惩 + 历史积累 + 高维层级 + 公约数公倍数照镜子
// ══════════════════════════════════════════════════════════════════
//
// 版本: v2（老二）
// 规则: 不修改旧版文件, 独立新版本, 报数命名
//
// 公约数-公倍数-照镜子机制:
//   "根据公约数的锚的对象, 去围绕公约数旋转的公倍数,
//    照镜子照出公约数的样子, 去对比公倍数做的对不对"
//
//   公约数 = 锚点标准 (每个属性的最低合格线)
//   公倍数 = 围绕锚点旋转的实际实例
//   照镜子 = 用公约数做镜子, 看公倍数是否符合标准
//   对比对不对 = 在不在合格半径内 → 奖励/惩罚
//
// ══════════════════════════════════════════════════════════════════

use crate::mirror::{
    self, encode_cell, get_value, get_polarity, get_sovereignty, Sovereignty,
    EncodedCell, POLARITY_POSITIVE, POLARITY_NEGATIVE,
};
use crate::observer::{Observer, SenseForm, SenseKind};
use crate::memory::{Coord, MetaJudgeEngine};
use crate::os_layer::OsLayer;
use crate::os_layer::bare_metal::BareMetalOs;

// ======================================================================
// 第一部分: 听觉授权 (AuditoryAuth) — 把听觉给授权
// ======================================================================

/// 听觉授权 — 听觉系统的独立授权
///
/// "把听觉给授权" = 听觉系统需要独立授权才能聆听袋子
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AuditoryAuth {
    Silent,     // 未授权 — 静默
    Listening,  // 已授权 — 聆听中
    Deafened,   // 已撤销 — 失聪
}

impl AuditoryAuth {
    pub fn can_hear(&self) -> bool {
        matches!(self, Self::Listening)
    }

    pub fn authorize(&mut self) -> bool {
        *self = Self::Listening;
        true
    }

    pub fn revoke(&mut self) {
        *self = Self::Deafened;
    }

    pub fn wake(&mut self) {
        if matches!(self, Self::Silent) {
            *self = Self::Listening;
        }
    }
}

/// 天觉授权 — 视觉+心智的授权
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TianJueAuth {
    Dormant,
    Authorized,
    Revoked,
}

impl TianJueAuth {
    pub fn can_perceive(&self) -> bool {
        matches!(self, Self::Authorized)
    }

    pub fn can_judge(&self) -> bool {
        matches!(self, Self::Authorized)
    }

    pub fn authorize(&mut self) -> bool {
        *self = Self::Authorized;
        true
    }

    pub fn revoke(&mut self) {
        *self = Self::Revoked;
    }
}

// ======================================================================
// 第二部分: 听觉系统 (AuditorySystem) — 听袋子
// ======================================================================

/// 听觉系统 — 聆听袋子的"声音"
///
/// 不是听米粒的响声, 而是听袋子的状态信号:
///   - 虫鸣声 (虫害)
///   - 沙沙声 (杂质)
///   - 风声 (干燥度)
///   - 水声 (湿度)
///   - 破裂声 (破损)
#[derive(Clone, Debug)]
pub struct AuditorySystem {
    pub sensitivity: f64,
    pub heard_bags: Vec<u64>,
    pub auth: AuditoryAuth,
}

impl AuditorySystem {
    pub fn new(sensitivity: f64) -> Self {
        Self {
            sensitivity: sensitivity.clamp(0.01, 1.0),
            heard_bags: Vec::new(),
            auth: AuditoryAuth::Silent,
        }
    }

    pub fn default_hearing() -> Self {
        Self::new(0.75)
    }

    /// 听一个袋子 — 需要听觉授权
    pub fn listen(&mut self, bag: &Bag) -> Result<Vec<SenseForm>, &'static str> {
        if !self.auth.can_hear() {
            return Err("听觉未授权, 无法聆听");
        }

        if !self.heard_bags.contains(&bag.id) {
            self.heard_bags.push(bag.id);
        }

        let mut forms = Vec::new();

        // 听觉感相: 虫鸣 (虫害信号)
        if bag.attrs.pest_level > 0.1 {
            let pest_intensity = bag.attrs.pest_level * self.sensitivity;
            forms.push(SenseForm::new(
                SenseKind::Hearing,
                pest_intensity,
                &format!("袋#{} 虫鸣强度={:.0}% (有害虫活动)", bag.id, bag.attrs.pest_level * 100.0),
                "auditory_system",
                0,
            ));

        }

        // 听觉感相: 沙沙声 (沙子/杂质信号)
        if bag.attrs.sand_level > 0.1 {
            let sand_sound = bag.attrs.sand_level * self.sensitivity * 0.8;
            forms.push(SenseForm::new(
                SenseKind::Hearing,
                sand_sound,
                &format!("袋#{} 沙沙声={:.0}% (含杂质)", bag.id, bag.attrs.sand_level * 100.0),
                "auditory_system",
                0,
           ));

        }

        // 听觉感相: 干燥裂声 (干燥袋子的破裂信号)
        if bag.attrs.dryness > 0.8 && bag.attrs.integrity < 0.5 {
            forms.push(SenseForm::new(
                SenseKind::Hearing,
                0.7 * self.sensitivity,
                &format!("袋#{} 干燥破裂声 (过干且破损)", bag.id),
                "auditory_system",
                0,
           ));

        }

        // 听觉感相: 水湿声 (湿度过高)
        if bag.attrs.dryness < 0.2 {
            forms.push(SenseForm::new(
                SenseKind::Hearing,
                0.6 * self.sensitivity,
                &format!("袋#{} 水湿声 (过于潮湿)", bag.id),
                "auditory_system",
                0,
           ));

        }

        // 听觉感相: 完整袋子的低沉共鸣 (好袋子)
        if bag.attrs.integrity > 0.8 && bag.attrs.pest_level < 0.1 && bag.attrs.sand_level < 0.1 {
            forms.push(SenseForm::new(
                SenseKind::Hearing,
                0.9 * self.sensitivity,
                &format!("袋#{} 完整共鸣 (健康袋子)", bag.id),
                "auditory_system",
                0,
           ));

        }

        Ok(forms)
    }

    pub fn authorize(&mut self) { self.auth.authorize(); }

    pub fn heard_count(&self) -> usize {
        self.heard_bags.len()
    }

    pub fn has_heard(&self, bag_id: u64) -> bool {
        self.heard_bags.contains(&bag_id)
    }
}

// ======================================================================
// 第三部分: 视觉系统 (VisualSystem) — 看袋子
// ======================================================================

/// 视觉系统 — 看袋子的外观
#[derive(Clone, Debug)]
pub struct VisualSystem {
    pub sensitivity: f64,
    pub threshold: f64,
}

impl VisualSystem {
    pub fn new(sensitivity: f64, threshold: f64) -> Self {
        Self {
            sensitivity: sensitivity.clamp(0.01, 1.0),
            threshold: threshold.clamp(0.01, 1.0),
        }
    }

    pub fn default_vision() -> Self {
        Self::new(0.85, 0.1)
    }

    /// 看一个袋子 — 产生视觉感相
    pub fn perceive(&self, bag: &Bag) -> Vec<SenseForm> {
        let mut forms = Vec::new();

        // 视觉: 主权归属
        forms.push(SenseForm::new(
            SenseKind::Vision,
            self.sensitivity,
            &format!("袋#{} 主权={}", bag.id, match bag.attrs.sovereignty {
                0 => "无主", 1 => "妻子", 2 => "丈夫", 3 => "夫妻共有", _ => "未知"
            }),
            "visual_system",
                0,
                ));


        // 视觉: 价值
        forms.push(SenseForm::new(
            SenseKind::Vision,
            (bag.attrs.value as f64 / 1000.0) * self.sensitivity,
            &format!("袋#{} 价值={}", bag.id, bag.attrs.value),
            "visual_system",
                0,
                ));


        // 视觉: 破损
        if bag.attrs.integrity < 1.0 - self.threshold {
            forms.push(SenseForm::new(
                SenseKind::Vision,
                (1.0 - bag.attrs.integrity) * self.sensitivity,
                &format!("袋#{} 破损度={:.0}%", bag.id, (1.0 - bag.attrs.integrity) * 100.0),
                "visual_system",
                0,
                ));

        }

        // 视觉: 脏污
        if bag.attrs.bag_cleanliness < 1.0 - self.threshold {
            forms.push(SenseForm::new(
                SenseKind::Vision,
                (1.0 - bag.attrs.bag_cleanliness) * self.sensitivity,
                &format!("袋#{} 脏污度={:.0}%", bag.id, (1.0 - bag.attrs.bag_cleanliness) * 100.0),
                "visual_system",
                0,
                ));

        }

        forms
    }

    pub fn appears_sound(&self, bag: &Bag) -> bool {
        bag.attrs.integrity > 0.5 && bag.attrs.bag_cleanliness > 0.5
    }
}

// ======================================================================
// 第四部分: 公约数-公倍数-照镜子 (数学验证机制)
// ======================================================================

/// 公约数 — 锚点标准
///
/// "根据公约数的锚的对象"
/// 公约数 = 各属性的最低合格线, 是审判的基准锚点
#[derive(Clone, Copy, Debug)]
pub struct CommonDivisor {
    pub id: u64,
    /// 主权要求的最低值 (0=无要求, 1=妻子, 2=丈夫, 3=夫妻共有)
    pub min_sovereignty: u64,
    /// 最低价值要求
    pub min_value: u64,
    /// 最低完好性 (0..1)
    pub min_integrity: f64,
    /// 最低内容质量 (0..1)
    pub min_content_quality: f64,
    /// 最低内容纯度 (0..1)
    pub min_content_purity: f64,
    /// 最低容器清洁度 (0..1)
    pub min_bag_cleanliness: f64,
    /// 最低干燥度 (0..1)
    pub min_dryness: f64,
    /// 最高虫害度 (0..1, 越低越好)
    pub max_pest_level: f64,
    /// 最高杂质度 (0..1, 越低越好)
    pub max_sand_level: f64,
    /// 最低可食性 (0..1)
    pub min_edibility: f64,
}

impl CommonDivisor {
    /// 创建默认公约数 (标准合格线)
    pub fn standard() -> Self {
        Self {
            id: 1,
            min_sovereignty: 0,    // 无主权要求
            min_value: 0,          // 无价值要求
            min_integrity: 0.3,    // 至少不完全破碎
            min_content_quality: 0.4,
            min_content_purity: 0.3,
            min_bag_cleanliness: 0.2,
            min_dryness: 0.3,
            max_pest_level: 0.7,   // 虫不能太多
            max_sand_level: 0.7,   // 沙不能太多
            min_edibility: 0.5,    // 至少基本可食
        }
    }

    /// 创建严格公约数 (高质量要求)
    pub fn strict() -> Self {
        Self {
            id: 2,
            min_sovereignty: 1,
            min_value: 300,
            min_integrity: 0.7,
            min_content_quality: 0.7,
            min_content_purity: 0.7,
            min_bag_cleanliness: 0.6,
            min_dryness: 0.6,
            max_pest_level: 0.3,
            max_sand_level: 0.3,
            min_edibility: 0.8,
        }
    }

    /// 创建宽松公约数 (最低限度)
    pub fn lenient() -> Self {
        Self {
            id: 3,
            min_sovereignty: 0,
            min_value: 0,
            min_integrity: 0.1,
            min_content_quality: 0.2,
            min_content_purity: 0.1,
            min_bag_cleanliness: 0.1,
            min_dryness: 0.1,
            max_pest_level: 0.9,
            max_sand_level: 0.9,
            min_edibility: 0.3,
        }
    }

    /// 一个袋子是否是此公约数的合格公倍数
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
    }
}

/// 公倍数 — 围绕公约数旋转的实际实例
///
/// "围绕公约数旋转的公倍数"
/// 公倍数 = 包含公约数标准的具体袋子实例
/// 它与公约数的距离 = "轨道半径"
#[derive(Clone, Debug)]
pub struct CommonMultiple {
    pub bag_id: u64,
    pub divisor_id: u64,
    /// 与公约数的偏离距离 (0=完全符合, 越大越偏离)
    pub orbit_radius: f64,
    /// 在n维空间中围绕公约数旋转的相位角 (用于标识不同变体)
    pub phase_angle: f64,
}

impl CommonMultiple {
    pub fn new(bag_id: u64, divisor_id: u64) -> Self {
        Self {
            bag_id,
            divisor_id,
            orbit_radius: 0.0,
            phase_angle: 0.0,
        }
    }
}

/// 照镜子结果 — 用公约数做镜子的对比结果
///
/// "照镜子照出公约数的样子, 去对比公倍数做的对不对"
#[derive(Clone, Debug)]
pub struct MirrorComparison {
    pub bag_id: u64,
    pub divisor_id: u64,
    /// 偏差值 (0=完美对齐, 越大越偏离)
    pub deviation: f64,
    /// 是否对齐 (在允许偏差内)
    pub aligned: bool,
    /// 镜中得分 (0..1, 越高越像公约数)
    pub reflection_score: f64,
    /// 镜中显示的属性差异详情
    pub diff_details: Vec<String>,
}

impl MirrorComparison {
    /// 照镜子: 用公约数反射, 对比公倍数
    pub fn compare(bag: &Bag, divisor: &CommonDivisor) -> Self {
        let a = &bag.attrs;
        let mut diff_details: Vec<String> = Vec::new();

        // 计算各维度的偏差
        let integrity_dev = (divisor.min_integrity - a.integrity).max(0.0);
        let quality_dev = (divisor.min_content_quality - a.content_quality).max(0.0);
        let purity_dev = (divisor.min_content_purity - a.content_purity).max(0.0);
        let clean_dev = (divisor.min_bag_cleanliness - a.bag_cleanliness).max(0.0);
        let dry_dev = (divisor.min_dryness - a.dryness).max(0.0);
        let pest_dev = (a.pest_level - divisor.max_pest_level).max(0.0);
        let sand_dev = (a.sand_level - divisor.max_sand_level).max(0.0);
        let edibility_dev = (divisor.min_edibility - a.edibility).max(0.0);

        // 总体偏差 = 各维度偏差的平方和开方 (欧氏距离)
        let total_deviation = (
            integrity_dev.powi(2)
            + quality_dev.powi(2)
            + purity_dev.powi(2)
            + clean_dev.powi(2)
            + dry_dev.powi(2)
            + pest_dev.powi(2)
            + sand_dev.powi(2)
            + edibility_dev.powi(2)
        ).sqrt();

        // 归一化到 0..1 (理论最大偏差 = sqrt(8) ≈ 2.83, 因为是8个维度最大各1.0的差)
        let normalized_deviation = (total_deviation / 2.83).min(1.0);

        // 收集差异详情
        if integrity_dev > 0.0 {
            diff_details.push(format!("完好性欠缺{:.0}%", integrity_dev * 100.0));
        }
        if quality_dev > 0.0 {
            diff_details.push(format!("质量欠缺{:.0}%", quality_dev * 100.0));
        }
        if purity_dev > 0.0 {
            diff_details.push(format!("纯度欠缺{:.0}%", purity_dev * 100.0));
        }
        if clean_dev > 0.0 {
            diff_details.push(format!("清洁欠缺{:.0}%", clean_dev * 100.0));
        }
        if dry_dev > 0.0 {
            diff_details.push(format!("干燥欠缺{:.0}%", dry_dev * 100.0));
        }
        if pest_dev > 0.0 {
            diff_details.push(format!("虫害超标{:.0}%", pest_dev * 100.0));
        }
        if sand_dev > 0.0 {
            diff_details.push(format!("杂质超标{:.0}%", sand_dev * 100.0));
        }
        if edibility_dev > 0.0 {
            diff_details.push(format!("可食性欠缺{:.0}%", edibility_dev * 100.0));
        }

        // 对齐 = 无偏差
        let aligned = diff_details.is_empty();

        // 镜中得分 = 1 - 偏差 (得分越高越像公约数)
        let reflection_score = 1.0 - normalized_deviation;

        Self {
            bag_id: bag.id,
            divisor_id: divisor.id,
            deviation: normalized_deviation,
            aligned,
            reflection_score,
            diff_details,
        }
    }

    /// 总结
    pub fn summary(&self) -> String {
        if self.aligned {
            format!("袋#{} 镜中完美对齐 ✓ (得分{:.0}%)", self.bag_id, self.reflection_score * 100.0)
        } else {
            format!(
                "袋#{} 镜中偏差{:.0}% ✗: {}",
                self.bag_id,
                self.deviation * 100.0,
                self.diff_details.join(", ")
            )
        }
    }
}

// ======================================================================
// 第五部分: 袋子属性 (BagAttributes) — 14维高维感知向量
// ======================================================================

#[derive(Clone, Copy, Debug)]
pub struct BagAttributes {
    pub sovereignty: u64,
    pub value: u64,
    pub integrity: f64,
    pub content_quality: f64,
    pub content_purity: f64,
    pub bag_cleanliness: f64,
    pub dryness: f64,
    pub pest_level: f64,
    pub sand_level: f64,
    pub edibility: f64,
    pub needs_content_wash: bool,
    pub needs_sand_pick: bool,
    pub wash_target: f64,
    pub needs_repair: bool,
}

impl BagAttributes {
    pub fn pristine() -> Self {
        Self {
            sovereignty: 3,
            value: 800,
            integrity: 1.0,
            content_quality: 1.0,
            content_purity: 1.0,
            bag_cleanliness: 1.0,
            dryness: 1.0,
            pest_level: 0.0,
            sand_level: 0.0,
            edibility: 1.0,
            needs_content_wash: false,
            needs_sand_pick: false,
            wash_target: 1.0,
            needs_repair: false,
        }
    }

    pub fn rotten() -> Self {
        Self {
            sovereignty: 0,
            value: 0,
            integrity: 0.0,
            content_quality: 0.0,
            content_purity: 0.0,
            bag_cleanliness: 0.0,
            dryness: 0.0,
            pest_level: 1.0,
            sand_level: 1.0,
            edibility: 0.0,
            needs_content_wash: true,
            needs_sand_pick: true,
            wash_target: 1.0,
            needs_repair: true,
        }
    }

    pub fn composite_quality(&self) -> f64 {
        let q = self.content_quality * 0.35
               + self.edibility * 0.30
               + self.content_purity * 0.15
               + self.integrity * 0.10
               + self.bag_cleanliness * 0.10;
        q.clamp(0.0, 1.0)
    }

    pub fn needs_any_processing(&self) -> bool {
        self.needs_content_wash || self.needs_sand_pick || self.needs_repair
    }

    pub fn quality_after_processing(&self) -> f64 {
        if !self.needs_any_processing() {
            return self.composite_quality();
        }
        let purity_after = if self.needs_content_wash { self.wash_target } else { self.content_purity };
        let clean_after = if self.needs_repair { 1.0 } else { self.bag_cleanliness };
        let pest_after = if self.needs_sand_pick { 0.0 } else { self.pest_level };
        let sand_after = if self.needs_sand_pick { 0.0 } else { self.sand_level };
        let q = self.content_quality * 0.35
               + self.edibility * 0.30
               + purity_after * 0.15
               + self.integrity * 0.10
               + clean_after * 0.10;
        let penalty = pest_after * 0.5 + sand_after * 0.5;
        (q - penalty).clamp(0.0, 1.0)
    }

    pub fn is_edible(&self) -> bool {
        self.edibility > 0.7 && self.pest_level < 0.3 && self.sand_level < 0.3 && self.integrity > 0.3
    }

    pub fn needs_drying(&self) -> bool {
        self.dryness < 0.7
    }

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
// 第六部分: 袋子 (Bag) — 高维层级的基本单元
// ======================================================================

#[derive(Clone, Debug)]
pub struct Bag {
    pub id: u64,
    pub name: String,
    pub attrs: BagAttributes,
    pub coord: Coord,
    pub layer_depth: u32,
    pub child_bag_ids: Vec<u64>,
}

impl Bag {
    pub fn new(id: u64, name: &str, attrs: BagAttributes, coord: Coord) -> Self {
        Self {
            id,
            name: name.to_string(),
            attrs,
            coord,
            layer_depth: 0,
            child_bag_ids: Vec::new(),
        }
    }

    pub fn nested(id: u64, name: &str, attrs: BagAttributes, coord: Coord, depth: u32) -> Self {
        Self {
            id,
            name: name.to_string(),
            attrs,
            coord,
            layer_depth: depth,
            child_bag_ids: Vec::new(),
        }
    }

    pub fn quality(&self) -> f64 {
        self.attrs.composite_quality()
    }

    pub fn is_edible(&self) -> bool {
        self.attrs.is_edible()
    }

    pub fn report(&self) -> String {
        format!("袋子#{}「{}」(层级{}): {}", self.id, self.name, self.layer_depth, self.attrs.summary())
    }
}

// ======================================================================
// 第七部分: 审判引擎 (JudgmentEngine) — 审判对了给奖励, 错了给惩罚
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
    pub fn was_correct(&self) -> bool {
        self.verdict == self.actual
    }

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
    /// 使用的公约数锚点
    pub divisor: CommonDivisor,
}

impl JudgmentEngine {
    pub fn new() -> Self {
        Self {
            reward_pool: 0.0,
            punishment_pool: 0.0,
            records: Vec::new(),
            current_accuracy: 0.5,
            streak_correct: 0,
            streak_wrong: 0,
            max_records: 1000,
            base_reward: 10.0,
            base_punishment: 10.0,
            pass_threshold: 0.6,
            divisor: CommonDivisor::standard(),
        }
    }

    pub fn with_divisor(divisor: CommonDivisor) -> Self {
        Self {
            reward_pool: 0.0,
            punishment_pool: 0.0,
            records: Vec::new(),
            current_accuracy: 0.5,
            streak_correct: 0,
            streak_wrong: 0,
            max_records: 1000,
            base_reward: 10.0,
            base_punishment: 10.0,
            pass_threshold: 0.6,
            divisor,
        }
    }

    /// 审判一个袋子 — 基于公约数做判定
    pub fn judge(&self, bag: &Bag, os: &dyn OsLayer) -> JudgmentResult {
        // 综合质量 >= 阈值 → 合格
        let quality = bag.attrs.composite_quality();
        let verdict = quality >= self.pass_threshold;
        // 同时也要求通过公约数验证
        let passes_divisor = self.divisor.is_valid_multiple(bag);
        let final_verdict = verdict && passes_divisor;

        let confidence = if final_verdict {
            ((quality - self.pass_threshold) / (1.0 - self.pass_threshold)).min(1.0).max(0.5)
        } else {
            let gap = if !verdict {
                (self.pass_threshold - quality) / self.pass_threshold
            } else {
                // 质量达标但没过公约数 (比如主权不匹配)
                0.3
            };
            gap.min(1.0).max(0.5)
        };

        let reason = if !verdict {
            format!("综合质量{:.0}%<阈值{:.0}%", quality * 100.0, self.pass_threshold * 100.0)
        } else if !passes_divisor {
            format!("质量达标但不符合公约数#{}", self.divisor.id)
        } else {
            format!("综合质量{:.0}%>=阈值{:.0}%且符合公约数", quality * 100.0, self.pass_threshold * 100.0)
        };

        JudgmentResult {
            bag_id: bag.id,
            verdict_passed: final_verdict,
            confidence,
            reason,
            timestamp_ms: current_timestamp_ms(os),
        }
    }

    /// 反馈审判结果 — 确定奖惩
    pub fn feedback(&mut self, bag_id: u64, verdict: bool, actual: bool, confidence: f64, os: &dyn OsLayer) -> f64 {
        let correct = verdict == actual;
        let reward = if correct {
            self.streak_correct += 1;
            self.streak_wrong = 0;
            let streak_bonus = if self.streak_correct >= 3 {
                self.streak_correct as f64 * 0.5
            } else { 0.0 };
            self.base_reward * confidence + streak_bonus
        } else {
            self.streak_wrong += 1;
            self.streak_correct = 0;
            let streak_penalty = if self.streak_wrong >= 3 {
                -(self.streak_wrong as f64 * 0.7)
            } else { 0.0 };
            -(self.base_punishment * confidence) + streak_penalty
        };

        if reward >= 0.0 {
            self.reward_pool += reward;
        } else {
            self.punishment_pool += reward.abs();
        }

        let record = JudgmentRecord {
            bag_id,
            verdict,
            actual,
            confidence,
            reward,
            timestamp_ms: current_timestamp_ms(os),
        };
        self.update_accuracy();
        self.records.push(record);

        if self.records.len() > self.max_records {
            self.records.remove(0);
        }

        reward
    }

    fn update_accuracy(&mut self) {
        if self.records.is_empty() {
            self.current_accuracy = 0.5;
            return;
        }
        let correct = self.records.iter().filter(|r| r.was_correct()).count();
        self.current_accuracy = correct as f64 / self.records.len() as f64;
    }

    pub fn judge_and_feedback(&mut self, bag: &Bag, actual_good: bool, os: &dyn OsLayer) -> (JudgmentResult, f64) {
        let result = self.judge(bag, os);
        let reward = self.feedback(result.bag_id, result.verdict_passed, actual_good, result.confidence, os);
        (result, reward)
    }

    pub fn history_summary(&self) -> String {
        let total = self.records.len();
        let correct = self.records.iter().filter(|r| r.was_correct()).count();
        let total_reward: f64 = self.records.iter().map(|r| if r.reward > 0.0 { r.reward } else { 0.0 }).sum();
        let total_punishment: f64 = self.records.iter().map(|r| if r.reward < 0.0 { r.reward.abs() } else { 0.0 }).sum();
        format!(
            "审判总数={} 正确({:.0}%) 总奖{:.1} 总罚{:.1} 连对{} 连错{}",
            total, self.current_accuracy * 100.0, total_reward, total_punishment,
            self.streak_correct, self.streak_wrong
        )
    }
}

// ======================================================================
// 第八部分: 历史积累 (HistoricalAccumulation) — 历史的记忆力
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
            epochs: Vec::new(),
            wisdom_score: 0.0,
            total_judgments: 0,
            total_correct: 0,
            best_accuracy: 0.0,
            current_epoch_start: start_tick,
            current_records: Vec::new(),
            epoch_capacity,
        }
    }

    pub fn record(&mut self, record: JudgmentRecord) {
        self.current_records.push(record.clone());
        self.total_judgments += 1;
        if record.was_correct() {
            self.total_correct += 1;
        }
        if self.current_records.len() as u64 >= self.epoch_capacity {
            self.seal_epoch(record.timestamp_ms);
        }
    }

    fn seal_epoch(&mut self, end_tick: u64) {
        if self.current_records.is_empty() { return; }
        let correct = self.current_records.iter().filter(|r| r.was_correct()).count();
        let accuracy = correct as f64 / self.current_records.len() as f64;
        let confidence_sum: f64 = self.current_records.iter().map(|r| r.confidence).sum();
        let avg_conf = confidence_sum / self.current_records.len() as f64;
        let wisdom_gained = accuracy * self.current_records.len() as f64 * avg_conf;
        self.wisdom_score += wisdom_gained;
        if accuracy > self.best_accuracy {
            self.best_accuracy = accuracy;
        }
        self.epochs.push(Epoch {
            start_tick: self.current_epoch_start,
            end_tick,
            accuracy,
            snapshot: format!("{}次审判, {:.0}%准确, 获{:.1}智慧",
                self.current_records.len(), accuracy * 100.0, wisdom_gained),
        });
        self.current_records.clear();
        self.current_epoch_start = end_tick;
    }

    pub fn finalize(&mut self, end_tick: u64) {
        self.seal_epoch(end_tick);
    }

    pub fn wisdom(&self) -> f64 {
        self.wisdom_score
    }

    pub fn overall_accuracy(&self) -> f64 {
        if self.total_judgments == 0 { return 0.0; }
        self.total_correct as f64 / self.total_judgments as f64
    }

    pub fn summary(&self) -> String {
        format!(
            "历史积累: {}时代, 审判{}次, 正确率{:.0}%, 最佳{:.0}%, 智慧{:.1}",
            self.epochs.len(), self.total_judgments, self.overall_accuracy() * 100.0,
            self.best_accuracy * 100.0, self.wisdom_score
        )
    }
}

// ======================================================================
// 第九部分: 袋子层级 (BagHierarchy) — 高维层级认知
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
        Self {
            bags: Vec::new(),
            max_bags,
            max_depth,
            total_count: 0,
        }
    }

    pub fn add_bag(&mut self, mut bag: Bag) -> Result<(), &'static str> {
        if self.bags.len() >= self.max_bags {
            return Err("层级已满");
        }
        bag.layer_depth = 0;
        self.total_count += 1;
        self.bags.push(bag);
        Ok(())
    }

    pub fn add_nested(&mut self, parent_id: u64, mut bag: Bag) -> Result<(), &'static str> {
        if self.bags.len() >= self.max_bags {
            return Err("层级已满");
        }
        let parent_depth = match self.bags.iter().find(|b| b.id == parent_id) {
            Some(parent) => {
                if parent.layer_depth >= self.max_depth {
                    return Err("已达最大深度");
                }
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

    pub fn count_bags(&self) -> usize {
        self.bags.len()
    }

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

    pub fn evaluate(&self) -> HierarchyReport {
        let total = self.bags.len();
        if total == 0 {
            return HierarchyReport::empty();
        }
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
        Self {
            total_bags: 0,
            edible_bags: 0,
            needs_processing: 0,
            average_quality: 0.0,
            layer_distribution: Vec::new(),
        }
    }

    pub fn summary(&self) -> String {
        if self.total_bags == 0 { return "层级为空".to_string(); }
        let layers: Vec<String> = self.layer_distribution.iter()
            .map(|(d, c)| format!("L{}:{}个", d, c)).collect();
        format!(
            "层级: {}袋, 可食{:.0}%, 需处理{}个, 均质{:.0}%, [{}]",
            self.total_bags,
            self.edible_bags as f64 / self.total_bags as f64 * 100.0,
            self.needs_processing,
            self.average_quality * 100.0,
            layers.join(", ")
        )
    }
}

// ======================================================================
// 第十部分: 天觉v2引擎 (CelestialPerceptionV2) — 整体协调
// ======================================================================

pub struct CelestialPerceptionV2<'a> {
    pub auth_tianjue: TianJueAuth,
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
            auth_tianjue: TianJueAuth::Dormant,
            auth_auditory: AuditoryAuth::Silent,
            visual: VisualSystem::default_vision(),
            auditory: AuditorySystem::default_hearing(),
            observer: Observer::new(1, "天觉v2", 64, os),
            judgment: JudgmentEngine::new(),
            history: HistoricalAccumulation::new(50, 0),
            hierarchy: BagHierarchy::new(1000, 10),
            bags_perceived: 0,
            mirror_comparisons: Vec::new(),
        }
    }

    /// 授权天觉 (视觉+心智)
    pub fn authorize_tianjue(&mut self) -> bool {
        self.auth_tianjue.authorize()
    }

    /// 授权听觉
    pub fn authorize_auditory(&mut self) -> bool {
        self.auth_auditory.authorize()
    }

    /// 全授权 (视觉+听觉)
    pub fn authorize_all(&mut self) {
        self.auth_tianjue.authorize();
        self.auth_auditory.authorize();
        self.auditory.auth.authorize();  // 同步听觉系统内部授权
    }

    pub fn is_fully_authorized(&self) -> bool {
        self.auth_tianjue.can_perceive() && self.auth_auditory.can_hear()
    }

    /// 感知一个袋子 (视觉+听觉)
    pub fn perceive(&mut self, bag: &Bag) -> Result<Vec<SenseForm>, &'static str> {
        if !self.auth_tianjue.can_perceive() {
            return Err("天觉未授权");
        }
        let mut all_forms = Vec::new();
        // 视觉
        let visual_forms = self.visual.perceive(bag);
        all_forms.extend(visual_forms);
        // 听觉 (如果已授权)
        if self.auth_auditory.can_hear() {
            let auditory_forms = self.auditory.listen(bag)?;
            all_forms.extend(auditory_forms);
        }
        // 送入观测者
        let _cells = self.observer.sense_batch(all_forms.clone());
        self.bags_perceived += 1;
        Ok(all_forms)
    }

    /// 照镜子: 用公约数对比袋子
    pub fn mirror_compare(&mut self, bag: &Bag, divisor: &CommonDivisor) -> MirrorComparison {
        let comparison = MirrorComparison::compare(bag, divisor);
        self.mirror_comparisons.push(comparison.clone());
        comparison
    }

    /// 天觉审判: 感知+照镜子+审判+历史
    pub fn perceive_judge_mirror(
        &mut self,
        bag: &Bag,
        actual_good: bool,
    ) -> Result<(JudgmentResult, f64, MirrorComparison), &'static str> {
        if !self.auth_tianjue.can_perceive() {
            return Err("天觉未授权");
        }
        // 1. 感知
        let _forms = self.perceive(bag)?;
        // 2. 照镜子 (先提取公约数, 避免借用冲突)
        let divisor = self.judgment.divisor;
        let mirror = self.mirror_compare(bag, &divisor);
        // 3. 审判
        let os = self.observer.os();
        let (result, reward) = self.judgment.judge_and_feedback(bag, actual_good, os);
        // 4. 历史积累
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

    pub fn consciousness_density(&self) -> f64 {
        self.observer.consciousness_density()
    }

    pub fn report(&self) -> String {
        let mut s = String::new();
        s.push_str("═══ 天觉v2报告 (老二) ═══\n");
        s.push_str(&format!("  天觉授权: {}\n", if self.auth_tianjue.can_perceive() { "✓" } else { "✗" }));
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

fn current_timestamp_ms(os: &dyn OsLayer) -> u64 {
    os.time_millis()
}

// ======================================================================
// 测试
// ======================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auditory_auth_flow() {
        let mut auth = AuditoryAuth::Silent;
        assert!(!auth.can_hear());
        assert!(auth.authorize());
        assert!(auth.can_hear());
        auth.revoke();
        assert!(!auth.can_hear());
    }

    #[test]
    fn auditory_system_listens() {
        let mut aud = AuditorySystem::default_hearing();
        // 未授权时不能听
        let bag = Bag::new(1, "测试", BagAttributes::pristine(), (0, 0, 0));
        assert!(aud.listen(&bag).is_err());
        // 授权后可以听
        aud.authorize();
        let result = aud.listen(&bag);
        assert!(result.is_ok());
    }

    #[test]
    fn auditory_hears_pests_and_sand() {
        let mut aud = AuditorySystem::default_hearing();
        aud.authorize();
        let mut attrs = BagAttributes::pristine();
        attrs.pest_level = 0.5;
        attrs.sand_level = 0.3;
        let bag = Bag::new(1, "虫袋", attrs, (0, 0, 0));
        let forms = aud.listen(&bag).unwrap();
        // 至少应听到虫鸣和沙沙声
        assert!(forms.len() >= 2);
    }

    #[test]
    fn common_divisor_standard_validates() {
        let d = CommonDivisor::standard();
        let good_bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        assert!(d.is_valid_multiple(&good_bag));
        let bad_bag = Bag::new(2, "坏袋", BagAttributes::rotten(), (0, 0, 0));
        assert!(!d.is_valid_multiple(&bad_bag));
    }

    #[test]
    fn common_divisor_strict() {
        let d = CommonDivisor::strict();
        let good_bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        assert!(d.is_valid_multiple(&good_bag));
        // 中等质量的袋子不一定通过严格校验
        let mut medium = BagAttributes::pristine();
        medium.content_quality = 0.5;
        let medium_bag = Bag::new(2, "中袋", medium, (0, 0, 0));
        assert!(!d.is_valid_multiple(&medium_bag));
    }

    #[test]
    fn mirror_compare_aligned() {
        let divisor = CommonDivisor::standard();
        let good_bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let mirror = MirrorComparison::compare(&good_bag, &divisor);
        assert!(mirror.aligned);
        assert!(mirror.reflection_score > 0.9);
    }

    #[test]
    fn mirror_compare_misaligned() {
        // 使用严格公约数, 坏袋偏差更大
        let divisor = CommonDivisor::strict();
        let bad_bag = Bag::new(2, "坏袋", BagAttributes::rotten(), (0, 0, 0));
        let mirror = MirrorComparison::compare(&bad_bag, &divisor);
        assert!(!mirror.aligned);
        assert!(mirror.deviation > 0.5, "deviation={}", mirror.deviation);
        assert!(!mirror.diff_details.is_empty());
    }

    #[test]
    fn judgment_engine_with_divisor() {
        let mut engine = JudgmentEngine::new();
        let os = BareMetalOs::new();
        let good_bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let result = engine.judge(&good_bag, &os);
        assert!(result.verdict_passed);
        let reward = engine.feedback(result.bag_id, result.verdict_passed, true, result.confidence, &os);
        assert!(reward > 0.0);
    }

    #[test]
    fn judgment_and_punishment() {
        let mut engine = JudgmentEngine::new();
        let os = BareMetalOs::new();
        // 审判错误: 实际是好袋, 我们判它不合格
        let reward = engine.feedback(1, false, true, 0.9, &os);
        assert!(reward < 0.0);
        assert!(engine.punishment_pool > 0.0);
    }

    #[test]
    fn history_accumulation_works() {
        let mut hist = HistoricalAccumulation::new(3, 0);
        for i in 1..=3 {
            let r = JudgmentRecord {
                bag_id: i,
                verdict: true,
                actual: true,
                confidence: 0.9,
                reward: 9.0,
                timestamp_ms: i as u64 * 100,
            };
            hist.record(r);
        }
        assert_eq!(hist.epochs.len(), 1);
        assert!(hist.wisdom() > 0.0);
    }

    #[test]
    fn hierarchy_nested_cognition() {
        let mut hier = BagHierarchy::new(100, 5);
        let parent = Bag::new(1, "父袋", BagAttributes::pristine(), (0, 0, 0));
        let child = Bag::new(2, "子袋", BagAttributes::pristine(), (0, 0, 0));
        hier.add_bag(parent).unwrap();
        hier.add_nested(1, child).unwrap();
        let p = hier.get(1).unwrap();
        assert_eq!(p.child_bag_ids, vec![2]);
        let c = hier.get(2).unwrap();
        assert_eq!(c.layer_depth, 1);
    }

    #[test]
    fn celestial_v2_full_pipeline() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV2::new(&os);
        // 未授权不能感知
        let bag = Bag::new(1, "测试", BagAttributes::pristine(), (0, 0, 0));
        assert!(cp.perceive(&bag).is_err());
        // 全授权
        cp.authorize_all();
        assert!(cp.is_fully_authorized());
        // 感知
        let forms = cp.perceive(&bag).unwrap();
        assert!(forms.len() > 0);
        // 照镜子
        let mirror = cp.mirror_compare(&bag, &CommonDivisor::standard());
        assert!(mirror.aligned);
        // 审判
        let (result, reward, _) = cp.perceive_judge_mirror(&bag, true).unwrap();
        assert!(result.verdict_passed);
        assert!(reward > 0.0);
    }

    #[test]
    fn mirror_deviation_scales_with_quality() {
        let divisor = CommonDivisor::standard();
        let perfect = Bag::new(1, "完美", BagAttributes::pristine(), (0, 0, 0));
        let rotten = Bag::new(2, "腐烂", BagAttributes::rotten(), (0, 0, 0));
        let m_perfect = MirrorComparison::compare(&perfect, &divisor);
        let m_rotten = MirrorComparison::compare(&rotten, &divisor);
        assert!(m_perfect.deviation < m_rotten.deviation);
        assert!(m_perfect.reflection_score > m_rotten.reflection_score);
    }
}
