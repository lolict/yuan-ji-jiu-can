// ══════════════════════════════════════════════════════════════════
// celestial_perception_v3.rs — 天庭v3（老三）
// 视觉 + 听觉授权 + 审判奖惩 + 历史积累 + 高维层级 + 公约数镜
// + 情进系统 + 行为系统 + 思考系统 + 审美系统 + 淘汰系统 + 过滤系统
// ══════════════════════════════════════════════════════════════════
//
// 版本: v3（老三）
// 规则: 不修改旧版文件, 独立新版本, 报数命名
//
// v3 改进点 (相对老大/老二):
//   1. 情进系统 — 感知触发情感向量, 情感驱动审判偏好
//   2. 行为系统 — 审判输出→行为指令 (收/弃/洗/晒/补)
//   3. 思考系统 — 历史+当前→推理链→决策
//   4. 审美系统 — 袋属性和谐度→审美评分
//   5. 淘汰系统 — 连续不合格→淘汰
//   6. 过滤系统 — 准入过滤, 不让坏袋子进入层级
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

// 百分比显示辅助: 将 0..1 转为 0..100
#[inline]
fn pct(x: f64) -> f64 { x * 100.0 }

// ======================================================================
// 第一部分: 授权体系
// ======================================================================

use crate::heavenly_court::HeavenlyCourtAuth;

// 听觉授权
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AuditoryAuth {
    Silent,
    Listening,
    Deafened,
}

impl AuditoryAuth {
    pub fn can_hear(&self) -> bool { matches!(self, Self::Listening) }
    pub fn authorize(&mut self) -> bool { *self = Self::Listening; true }
    pub fn revoke(&mut self) { *self = Self::Deafened; }
}

// ======================================================================
// 第二部分: 袋子属性 (14维高维感知向量)
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

    pub fn needs_drying(&self) -> bool {
        self.dryness < 0.7
    }

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
}

// ======================================================================
// 第三部分: 袋子 (高维层级基本单元)
// ======================================================================

#[derive(Clone, Debug)]
pub struct Bag {
    pub id: u64,
    pub name: String,
    pub attrs: BagAttributes,
    pub coord: Coord,
    pub layer_depth: u32,
    pub child_bag_ids: Vec<u64>,
    pub judgment_count: u64,
    pub consecutive_failures: u64,
    pub eliminated: bool,
}

impl Bag {
    pub fn new(id: u64, name: &str, attrs: BagAttributes, coord: Coord) -> Self {
        Self {
            id, name: name.to_string(), attrs, coord,
            layer_depth: 0, child_bag_ids: Vec::new(),
            judgment_count: 0, consecutive_failures: 0, eliminated: false,
        }
    }

    pub fn quality(&self) -> f64 { self.attrs.composite_quality() }
    pub fn is_edible(&self) -> bool { self.attrs.is_edible() }
    pub fn aesthetic(&self) -> f64 { self.attrs.aesthetic_score() }
}

// ======================================================================
// 第四部分: 视觉系统
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
            "visual_v3",
                0,
        ));
        forms.push(SenseForm::new(
            SenseKind::Vision,
            (bag.attrs.value as f64 / 1000.0) * self.sensitivity,
            &format!("袋#{} 价值={}", bag.id, bag.attrs.value),
            "visual_v3",
                0,
        ));
        if bag.attrs.integrity < 1.0 - self.threshold {
            forms.push(SenseForm::new(
                SenseKind::Vision,
                (1.0 - bag.attrs.integrity) * self.sensitivity,
                &format!("袋#{} 破损{:.0}%", bag.id, pct(1.0 - bag.attrs.integrity)),
                "visual_v3",
                0,
            ));
        }
        if bag.attrs.bag_cleanliness < 1.0 - self.threshold {
            forms.push(SenseForm::new(
                SenseKind::Vision,
                (1.0 - bag.attrs.bag_cleanliness) * self.sensitivity,
                &format!("袋#{} 脏污{:.0}%", bag.id, pct(1.0 - bag.attrs.bag_cleanliness)),
                "visual_v3",
                0,
            ));
        }
        forms
    }
}

// ======================================================================
// 第五部分: 听觉系统 (需授权)
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

    pub fn listen(&mut self, bag: &Bag) -> Result<Vec<SenseForm>, &'static str> {
        if !self.auth.can_hear() { return Err("听觉未授权"); }
        if !self.heard_bags.contains(&bag.id) { self.heard_bags.push(bag.id); }

        let mut forms = Vec::new();
        if bag.attrs.pest_level > 0.1 {
            forms.push(SenseForm::new(
                SenseKind::Hearing, bag.attrs.pest_level * self.sensitivity,
                &format!("袋#{} 虫鸣{:.0}%", bag.id, pct(bag.attrs.pest_level)),
                "auditory_v3",
                0,
            ));
        }
        if bag.attrs.sand_level > 0.1 {
            forms.push(SenseForm::new(
                SenseKind::Hearing, bag.attrs.sand_level * self.sensitivity * 0.8,
                &format!("袋#{} 沙沙声{:.0}%", bag.id, pct(bag.attrs.sand_level)),
                "auditory_v3",
                0,
            ));
        }
        if bag.attrs.dryness > 0.8 && bag.attrs.integrity < 0.5 {
            forms.push(SenseForm::new(
                SenseKind::Hearing, 0.7 * self.sensitivity,
                &format!("袋#{} 干燥破裂声", bag.id),
                "auditory_v3",
                0,
            ));
        }
        if bag.attrs.dryness < 0.2 {
            forms.push(SenseForm::new(
                SenseKind::Hearing, 0.6 * self.sensitivity,
                &format!("袋#{} 水湿声", bag.id),
                "auditory_v3",
                0,
            ));
        }
        if bag.attrs.integrity > 0.8 && bag.attrs.pest_level < 0.1 && bag.attrs.sand_level < 0.1 {
            forms.push(SenseForm::new(
                SenseKind::Hearing, 0.9 * self.sensitivity,
                &format!("袋#{} 健康共鸣", bag.id),
                "auditory_v3",
                0,
            ));
        }
        Ok(forms)
    }
}

// ======================================================================
// 第六部分: 公约数-公倍数-照镜子
// ======================================================================

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
// 第七部分: 审判引擎
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
    pub divisor: CommonDivisor,
    pub elimination_threshold: u64,
}

impl JudgmentEngine {
    pub fn new() -> Self {
        Self {
            reward_pool: 0.0, punishment_pool: 0.0, records: Vec::new(),
            current_accuracy: 0.5, streak_correct: 0, streak_wrong: 0,
            max_records: 1000, base_reward: 10.0, base_punishment: 10.0,
            pass_threshold: 0.6, divisor: CommonDivisor::standard(),
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
// 第八部分: 情进系统 (情感引擎)
// ======================================================================

#[derive(Clone, Debug)]
pub enum Emotion {
    Joy(f64),
    Disgust(f64),
    Sorrow(f64),
    Peace(f64),
    Anxiety(f64),
}

impl Emotion {
    pub fn intensity(&self) -> f64 {
        match self {
            Emotion::Joy(v) | Emotion::Disgust(v) | Emotion::Sorrow(v)
            | Emotion::Peace(v) | Emotion::Anxiety(v) => *v,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Emotion::Joy(_) => "喜",
            Emotion::Disgust(_) => "恶",
            Emotion::Sorrow(_) => "哀",
            Emotion::Peace(_) => "宁",
            Emotion::Anxiety(_) => "忧",
        }
    }
}

#[derive(Clone, Debug)]
pub struct EmotionSystem {
    pub current_emotions: Vec<Emotion>,
    pub threshold_modifier: f64,
}

impl EmotionSystem {
    pub fn new() -> Self {
        Self { current_emotions: Vec::new(), threshold_modifier: 0.0 }
    }

    pub fn feel(&mut self, bag: &Bag) {
        self.current_emotions.clear();
        let q = bag.quality();

        if q > 0.8 {
            self.current_emotions.push(Emotion::Joy(q));
        } else if q < 0.3 {
            self.current_emotions.push(Emotion::Disgust(1.0 - q));
        } else if bag.consecutive_failures > 0 {
            self.current_emotions.push(Emotion::Sorrow(bag.consecutive_failures as f64 * 0.2));
        }

        if bag.attrs.needs_any_processing() {
            self.current_emotions.push(Emotion::Anxiety(0.3));
        }

        if !self.current_emotions.iter().any(|e| matches!(e, Emotion::Anxiety(_))) && q > 0.6 {
            self.current_emotions.push(Emotion::Peace(q * 0.5));
        }

        self.threshold_modifier = self.calculate_threshold_modifier();
    }

    fn calculate_threshold_modifier(&self) -> f64 {
        let mut modifier = 0.0;
        for emotion in &self.current_emotions {
            match emotion {
                Emotion::Joy(v) => modifier -= v * 0.05,
                Emotion::Disgust(v) => modifier += v * 0.1,
                Emotion::Sorrow(v) => modifier += v * 0.05,
                Emotion::Peace(v) => modifier -= v * 0.02,
                Emotion::Anxiety(v) => modifier += v * 0.08,
            }
        }
        modifier.clamp(-0.2, 0.2)
    }

    pub fn adjusted_threshold(&self, base_threshold: f64) -> f64 {
        (base_threshold + self.threshold_modifier).clamp(0.3, 0.9)
    }

    pub fn summary(&self) -> String {
        if self.current_emotions.is_empty() {
            "情进: 平静".to_string()
        } else {
            let emotions: Vec<String> = self.current_emotions.iter()
                .map(|e| format!("{}{:.0}", e.name(), pct(e.intensity())))
                .collect();
            format!("情进: {} (阈值修正{:+.0}%)", emotions.join(" "), pct(self.threshold_modifier))
        }
    }
}

// ======================================================================
// 第九部分: 行为系统 — 审判输出→行为指令
// ======================================================================

#[derive(Clone, Debug)]
pub enum Behavior {
    Accept,
    Reject,
    WashContent,
    DryContent,
    PickSand,
    Repair,
    Eliminate,
    Wait,
}

impl Behavior {
    pub fn name(&self) -> &'static str {
        match self {
            Behavior::Accept => "收",
            Behavior::Reject => "弃",
            Behavior::WashContent => "洗",
            Behavior::DryContent => "晒",
            Behavior::PickSand => "捡",
            Behavior::Repair => "补",
            Behavior::Eliminate => "汰",
            Behavior::Wait => "等",
        }
    }
}

#[derive(Clone, Debug)]
pub struct BehaviorSystem {
    pub actions: Vec<(u64, Behavior, String)>,
}

impl BehaviorSystem {
    pub fn new() -> Self { Self { actions: Vec::new() } }

    pub fn decide(&mut self, bag: &Bag, verdict_passed: bool) -> Behavior {
        self.actions.clear();

        let action = if verdict_passed {
            Behavior::Accept
        } else if bag.consecutive_failures >= 3 {
            Behavior::Eliminate
        } else if bag.attrs.content_quality < 0.2 {
            Behavior::Reject
        } else if bag.attrs.needs_content_wash && bag.attrs.content_quality > 0.5 {
            Behavior::WashContent
        } else if bag.attrs.needs_drying() {
            Behavior::DryContent
        } else if bag.attrs.needs_sand_pick && bag.attrs.content_quality > 0.4 {
            Behavior::PickSand
        } else if bag.attrs.needs_repair && bag.attrs.content_quality > 0.5 {
            Behavior::Repair
        } else {
            Behavior::Wait
        };

        self.actions.push((bag.id, action.clone(), format!("袋#{}→{}", bag.id, action.name())));
        action
    }

    pub fn summary(&self) -> String {
        if self.actions.is_empty() { "行为: 无".to_string() }
        else { format!("行为: {}", self.actions.iter().map(|(_, b, _)| b.name()).collect::<Vec<_>>().join(" ")) }
    }
}

// ======================================================================
// 第十部分: 思考系统 — 历史+当前→推理→决策
// ======================================================================

#[derive(Clone, Debug)]
pub struct ReasoningChain {
    pub steps: Vec<String>,
    pub conclusion: String,
}

#[derive(Clone, Debug)]
pub struct ThinkingSystem {
    pub last_reasoning: Option<ReasoningChain>,
}

impl ThinkingSystem {
    pub fn new() -> Self { Self { last_reasoning: None } }

    pub fn think(&mut self, bag: &Bag, history_accuracy: f64, hierarchy_health: f64) -> ReasoningChain {
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

        let chain = ReasoningChain { steps, conclusion: conclusion.to_string() };
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

// ======================================================================
// 第十一部分: 审美系统
// ======================================================================

#[derive(Clone, Debug)]
pub struct AestheticSystem {
    pub last_score: f64,
}

impl AestheticSystem {
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

// ======================================================================
// 第十二部分: 淘汰系统
// ======================================================================

#[derive(Clone, Debug)]
pub struct EliminationSystem {
    pub eliminated_bags: Vec<u64>,
    pub threshold: u64,
}

impl EliminationSystem {
    pub fn new(threshold: u64) -> Self {
        Self { eliminated_bags: Vec::new(), threshold }
    }

    pub fn check(&mut self, bag: &mut Bag) -> bool {
        if bag.consecutive_failures >= self.threshold && !bag.eliminated {
            bag.eliminated = true;
            self.eliminated_bags.push(bag.id);
            true
        } else {
            false
        }
    }

    pub fn is_eliminated(&self, bag_id: u64) -> bool {
        self.eliminated_bags.contains(&bag_id)
    }

    pub fn summary(&self) -> String {
        format!("淘汰: {}个袋子已被淘汰 ({:?})", self.eliminated_bags.len(), self.eliminated_bags)
    }
}

// ======================================================================
// 第十三部分: 过滤系统
// ======================================================================

#[derive(Clone, Debug)]
pub struct FilterSystem {
    pub min_quality: f64,
    pub filtered_bags: Vec<u64>,
    pub filter_reasons: Vec<(u64, String)>,
}

impl FilterSystem {
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
// 第十四部分: 历史积累
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
// 第十五部分: 袋子层级
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

// ======================================================================
// 第十六部分: 天庭v3总引擎
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
    pub emotion: EmotionSystem,
    pub behavior: BehaviorSystem,
    pub thinking: ThinkingSystem,
    pub aesthetic: AestheticSystem,
    pub elimination: EliminationSystem,
    pub filter: FilterSystem,
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
            emotion: EmotionSystem::new(),
            behavior: BehaviorSystem::new(),
            thinking: ThinkingSystem::new(),
            aesthetic: AestheticSystem::new(),
            elimination: EliminationSystem::new(3),
            filter: FilterSystem::new(0.0),
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

    pub fn full_pipeline(&mut self, bag: &mut Bag, actual_good: bool) -> Result<PipelineResult, &'static str> {
        if !self.auth_heavenly_court.can_perceive() { return Err("天庭未授权"); }

        self.emotion.feel(bag);
        self.aesthetic.evaluate(bag);

        if !self.filter.filter(bag) {
            return Ok(PipelineResult {
                bag_id: bag.id, accepted: false, behavior: Behavior::Reject,
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

        Ok(PipelineResult {
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

#[derive(Clone, Debug)]
pub struct PipelineResult {
    pub bag_id: u64,
    pub accepted: bool,
    pub behavior: Behavior,
    pub judgment: Option<JudgmentResult>,
    pub reward: f64,
    pub mirror: Option<MirrorComparison>,
    pub emotion_summary: String,
    pub thinking_summary: String,
    pub aesthetic_summary: String,
    pub eliminated: bool,
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
    fn v3_full_pipeline_accepts_good_bag() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV3::new(&os);
        cp.authorize_all();

        let mut bag = Bag::new(1, "优质袋", BagAttributes::pristine(), (0, 0, 0));
        let result = cp.full_pipeline(&mut bag, true).unwrap();

        assert!(result.accepted);
        assert!(matches!(result.behavior, Behavior::Accept));
        assert!(result.reward > 0.0);
        assert!(!result.eliminated);
    }

    #[test]
    fn v3_full_pipeline_rejects_rotten_bag() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV3::new(&os);
        cp.authorize_all();

        let mut bag = Bag::new(2, "腐烂袋", BagAttributes::rotten(), (0, 0, 0));
        let result = cp.full_pipeline(&mut bag, false).unwrap();
        assert!(!result.accepted);
        assert!(result.reward > 0.0);
    }

    #[test]
    fn v3_filter_blocks_bad_bag() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV3::new(&os);
        cp.authorize_all();
        cp.filter.min_quality = 0.3;

        let mut bag = Bag::new(1, "极坏袋", BagAttributes::rotten(), (0, 0, 0));
        let result = cp.full_pipeline(&mut bag, false).unwrap();
        assert!(!result.accepted);
    }

    #[test]
    fn v3_elimination_after_consecutive_failures() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV3::new(&os);
        cp.authorize_all();
        cp.elimination.threshold = 2;

        let mut bag = Bag::new(1, "次品袋", BagAttributes::rotten(), (0, 0, 0));

        let r1 = cp.full_pipeline(&mut bag, false).unwrap();
        assert!(!r1.eliminated);

        let r2 = cp.full_pipeline(&mut bag, false).unwrap();
        assert!(r2.eliminated);
    }

    #[test]
    fn v3_emotion_system_responds() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV3::new(&os);
        cp.authorize_all();

        let mut good_bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        cp.full_pipeline(&mut good_bag, true).unwrap();
        assert!(cp.emotion.current_emotions.iter().any(|e| matches!(e, Emotion::Joy(_))));

        let mut bad_bag = Bag::new(2, "坏袋", BagAttributes::rotten(), (0, 0, 0));
        cp.full_pipeline(&mut bad_bag, false).unwrap();
        assert!(cp.emotion.current_emotions.iter().any(|e| matches!(e, Emotion::Disgust(_))));
    }

    #[test]
    fn v3_behavior_system_decides() {
        let mut bs = BehaviorSystem::new();
        let good_bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let action = bs.decide(&good_bag, true);
        assert!(matches!(action, Behavior::Accept));

        let bad_bag = Bag::new(2, "坏袋", BagAttributes::rotten(), (0, 0, 0));
        let action = bs.decide(&bad_bag, false);
        assert!(matches!(action, Behavior::Reject | Behavior::Eliminate));
    }

    #[test]
    fn v3_thinking_system_reasons() {
        let mut ts = ThinkingSystem::new();
        let bag = Bag::new(1, "袋", BagAttributes::pristine(), (0, 0, 0));
        let chain = ts.think(&bag, 0.9, 0.8);
        assert!(!chain.steps.is_empty());
        assert!(chain.conclusion.contains("可接纳"));
    }

    #[test]
    fn v3_aesthetic_system_evaluates() {
        let mut aesthetic = AestheticSystem::new();
        let perfect_bag = Bag::new(1, "完美袋", BagAttributes::pristine(), (0, 0, 0));
        let score = aesthetic.evaluate(&perfect_bag);
        assert!(score > 0.7);
    }

    #[test]
    fn v3_mirror_comparison_works() {
        let divisor = CommonDivisor::standard();
        let perfect = Bag::new(1, "完美", BagAttributes::pristine(), (0, 0, 0));
        let rotten = Bag::new(2, "腐烂", BagAttributes::rotten(), (0, 0, 0));
        let m1 = MirrorComparison::compare(&perfect, &divisor);
        let m2 = MirrorComparison::compare(&rotten, &divisor);
        assert!(m1.aligned);
        assert!(!m2.aligned);
        assert!(m1.reflection_score > m2.reflection_score);
    }

    #[test]
    fn v3_hierarchy_health_score() {
        let mut hier = BagHierarchy::new(100, 5);
        assert_eq!(hier.health_score(), 0.5);
        let bag = Bag::new(1, "袋", BagAttributes::pristine(), (0, 0, 0));
        hier.add_bag(bag).unwrap();
        assert!(hier.health_score() > 0.5);
    }

    #[test]
    fn v3_unauthorized_pipeline_fails() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV3::new(&os);
        let mut bag = Bag::new(1, "袋", BagAttributes::pristine(), (0, 0, 0));
        assert!(cp.full_pipeline(&mut bag, true).is_err());
    }

    #[test]
    fn v3_punishment_for_wrong_judgment() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV3::new(&os);
        cp.authorize_all();

        let mut bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let result = cp.full_pipeline(&mut bag, false).unwrap();
        assert!(result.reward < 0.0);
    }
}
