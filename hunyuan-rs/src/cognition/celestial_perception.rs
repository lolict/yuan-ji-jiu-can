// ══════════════════════════════════════════════════════════════════
// celestial_perception.rs — 天觉 · 视觉 · 袋层级 · 审判奖惩 · 历史积累
// ══════════════════════════════════════════════════════════════════
//
// 核心哲学 (用户定义):
//   "层级关系不是低维度的层级, 而是高维度的层级"
//
//   低维 = 数米粒 (每粒命名 → O(n) 线性枚举)
//   高维 = 数袋子 (袋袋命名 → O(1) 容器标识 + 属性向量)
//
// 袋子的14维属性 (高维感知向量):
//   ┌─────────────────────────────────────────────────────────┐
//   │  1. 谁家的袋子      (主权归属: 无主/妻子/丈夫/夫妻共有) │
//   │  2. 值多少钱        (价值: 0..1000)                     │
//   │  3. 好不好用/坏了   (完好性: 0=完全坏, 1.0=完美)       │
//   │  4. 米好不好吃      (内容质量: 0=有毒, 1.0=极鲜美)     │
//   │  5. 有没有脏东西    (内容纯度: 0=全脏, 1.0=完全纯净)   │
//   │  6. 袋子脏不脏      (容器清洁: 0=极脏, 1.0=极干净)     │
//   │  7. 谷子晒干了没有  (干燥度: 0=全湿, 1.0=全干)        │
//   │  8. 有没有虫        (虫害度: 0=无虫, 1.0=全是虫)       │
//   │  9. 有没有沙子      (杂质度: 0=无沙, 1.0=全是沙)       │
//   │ 10. 能不能吃        (可食性: 0=不能吃, 1.0=完全能吃)   │
//   │ 11. 米要不要洗      (需要内容清洁: true/false)          │
//   │ 12. 要不要捡沙子    (需要除杂: true/false)              │
//   │ 13. 洗成什么程度    (加工目标清洁度: 0..1)              │
//   │ 14. 袋子是否破了    (需要修补: true/false)              │
//   └─────────────────────────────────────────────────────────┘
//
// 系统组件:
//   1. Bag (袋子)           — 高维层级的基本单元
//   2. BagAttributes (属性) — 14维高维感知向量
//   3. VisualSystem (视觉)  — 感知袋子外观 (外观→SenseForm)
//   4. CelestialPerception (天觉) — 高维感知引擎 (需授权)
//   5. JudgmentEngine (审判) — 审判对了给奖励, 错了给惩罚
//   6. HistoricalAccumulation (历史积累) — 历史的记忆力和积累
//   7. BagHierarchy (袋层级) — 高维层级的逻辑认知
//
// ══════════════════════════════════════════════════════════════════

use crate::mirror::{
    self, encode_cell, get_value, get_polarity, get_sovereignty, Sovereignty,
    EncodedCell, POLARITY_POSITIVE, POLARITY_NEGATIVE,
};
use crate::observer::{Observer, SenseForm, SenseKind};
use crate::memory::{Coord, MetaJudgeEngine};
use crate::pleasure::Pleasure;
use crate::os_layer::OsLayer;
use crate::os_layer::bare_metal::BareMetalOs;

// ======================================================================
// 第一部分: 袋子属性 (BagAttributes) — 14维高维感知向量
// ======================================================================

/// 袋子属性向量 — 高维感知的完整状态
///
/// 这 14 个属性就是"我不会去数米粒, 我只命名袋子"的全部属性
#[derive(Clone, Copy, Debug)]
pub struct BagAttributes {
    /// 1. 谁家的袋子 (主权归属): 0=无主, 1=妻子, 2=丈夫, 3=夫妻共有
    pub sovereignty: u64,
    /// 2. 值多少钱 (价值): 0..1000
    pub value: u64,
    /// 3. 好不好用/是否坏了 (完好性): 0=完全坏, 1.0=完美
    pub integrity: f64,
    /// 4. 里面的米好不好吃 (内容质量): 0=有毒, 1.0=极鲜美
    pub content_quality: f64,
    /// 5. 有没有脏东西 (内容纯度): 0=全是脏东西, 1.0=完全纯净
    pub content_purity: f64,
    /// 6. 袋子脏不脏/要不要洗 (容器清洁): 0=极脏, 1.0=极干净
    pub bag_cleanliness: f64,
    /// 7. 谷子晒干了没有 (干燥度): 0=全湿, 1.0=全干
    pub dryness: f64,
    /// 8. 有没有虫 (虫害度): 0=无虫, 1.0=全是虫
    pub pest_level: f64,
    /// 9. 有没有沙子 (杂质度): 0=无沙, 1.0=全是沙
    pub sand_level: f64,
    /// 10. 能不能吃 (可食性): 0=完全不能吃, 1.0=完全能吃
    pub edibility: f64,
    /// 11. 里面的米要不要洗 (需要内容清洁): true/false
    pub needs_content_wash: bool,
    /// 12. 要不要捡沙子 (需要除杂): true/false
    pub needs_sand_pick: bool,
    /// 13. 洗成什么样才能吃 (加工目标清洁度): 0..1
    pub wash_target: f64,
    /// 14. 袋子本身是否破损 (需要修补): true/false
    pub needs_repair: bool,
}

impl BagAttributes {
    /// 创建一个全新的完美袋子 (默认值)
    pub fn pristine() -> Self {
        Self {
            sovereignty: 3,         // 夫妻共有
            value: 800,             // 高价值
            integrity: 1.0,         // 完美
            content_quality: 1.0,   // 极鲜美
            content_purity: 1.0,    // 完全纯净
            bag_cleanliness: 1.0,   // 极干净
            dryness: 1.0,           // 全干
            pest_level: 0.0,        // 无虫
            sand_level: 0.0,        // 无沙
            edibility: 1.0,         // 完全能吃
            needs_content_wash: false,
            needs_sand_pick: false,
            wash_target: 1.0,
            needs_repair: false,
        }
    }

    /// 创建一个完全腐败的袋子
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

    /// 计算袋子综合质量分数 (0..1)
    /// 这是审判引擎的核心判定依据
    pub fn composite_quality(&self) -> f64 {
        let quality_score = self.content_quality * 0.35 +  // 内容质量权重最高
                           self.edibility * 0.30 +          // 可食性其次
                           self.content_purity * 0.15 +     // 纯度
                           self.integrity * 0.10 +          // 完好性
                           self.bag_cleanliness * 0.10;     // 容器清洁
        quality_score.clamp(0.0, 1.0)
    }

    /// 判断袋子是否需要某种加工处理
    pub fn needs_any_processing(&self) -> bool {
        self.needs_content_wash || self.needs_sand_pick || self.needs_repair
    }

    /// 计算加工后能达到的质量 (0..1)
    /// 如果进行了所需加工, 质量能提升多少
    pub fn quality_after_processing(&self) -> f64 {
        if !self.needs_any_processing() {
            return self.composite_quality();
        }
        // 加工后: 纯度恢复到目标值, 清洁度恢复
        let purity_after = if self.needs_content_wash { self.wash_target } else { self.content_purity };
        let clean_after = if self.needs_repair { 1.0 } else { self.bag_cleanliness };
        let pest_after = if self.needs_sand_pick { 0.0 } else { self.pest_level };
        let sand_after = if self.needs_sand_pick { 0.0 } else { self.sand_level };

        // 加工后的综合质量
        let quality_after = self.content_quality * 0.35
                          + self.edibility * 0.30
                          + purity_after * 0.15
                          + self.integrity * 0.10
                          + clean_after * 0.10;
        // 虫沙惩罚
        let pest_sand_penalty = pest_after * 0.5 + sand_after * 0.5;
        (quality_after - pest_sand_penalty).clamp(0.0, 1.0)
    }

    /// 袋子是否能安全食用 (综合判定)
    pub fn is_edible(&self) -> bool {
        self.edibility > 0.7
            && self.pest_level < 0.3
            && self.sand_level < 0.3
            && self.integrity > 0.3
    }

    /// 袋子是否需要烘干
    pub fn needs_drying(&self) -> bool {
        self.dryness < 0.7
    }

    /// 编码属性为64位单元格 (与 mirror.rs 兼容)
    ///
    /// 映射关系:
    ///   d0 = sovereignty * 85 + integrity * 25 (编码主权+完好性)
    ///   d1 = content_quality * 255 (内容质量)
    ///   d2 = edibility * 255 (可食性)
    ///   polarity = content_quality > 0.5 → POSITIVE, 否则 NEGATIVE
    ///   sovereignty = 主权字段
    ///   witness = 0 (尚未被见证)
    ///   value = value 字段
    pub fn to_cell(&self) -> EncodedCell {
        let d0 = (self.sovereignty * 85 + (self.integrity * 25.0) as u64).min(1023);
        let d1 = (self.content_quality * 255.0) as u64;
        let d2 = (self.edibility * 255.0) as u64;
        let polarity = if self.content_quality > 0.5 { POLARITY_POSITIVE } else { POLARITY_NEGATIVE };
        encode_cell(d0, d1, d2, polarity, self.sovereignty, 0, self.value)
    }

    /// 从64位单元格和额外数据解码属性 (部分信息)
    /// 注意: 14维属性不能完全编码到64位中, 这里解码关键坐标
    pub fn from_cell(cell: EncodedCell, full_attrs: Option<Self>) -> Self {
        match full_attrs {
            Some(attrs) => attrs,
            None => {
                // 仅从cell解码的近似值
                let d0 = mirror::get_d0(cell);
                let d1 = mirror::get_d1(cell);
                let d2 = mirror::get_d2(cell);
                Self {
                    sovereignty: mirror::get_sovereignty(cell),
                    value: get_value(cell),
                    integrity: ((d0 % 85) as f64) / 25.0,
                    content_quality: (d1 as f64) / 255.0,
                    edibility: (d2 as f64) / 255.0,
                    ..Self::pristine()
                }
            }
        }
    }

    /// 属性的中文描述
    /// 安全获取 Option 中的布尔值，避免 Copy trait 限制
    pub fn content_wash_needed(&self) -> bool { self.needs_content_wash }
    pub fn sand_pick_needed(&self) -> bool { self.needs_sand_pick }
    pub fn repair_needed(&self) -> bool { self.needs_repair }

    /// 属性摘要 (用于日志和报告)
    pub fn summary(&self) -> String {
        let sovereignty_name = match self.sovereignty {
            0 => "无主",
            1 => "妻子",
            2 => "丈夫",
            3 => "夫妻共有",
            _ => "未知",
        };
        format!(
            "属于[{}] 值{} 完好{:.0}% 质量{:.0}% 纯度{:.0}% 清洁{:.0}% 干{:.0}% 虫{:.0}% 沙{:.0}% 可食{:.0}% | 需{}{}{}",
            sovereignty_name, self.value,
            self.integrity * 100.0, self.content_quality * 100.0, self.content_purity * 100.0,
            self.bag_cleanliness * 100.0, self.dryness * 100.0, self.pest_level * 100.0, self.sand_level * 100.0,
            self.edibility * 100.0,
            if self.needs_content_wash { "洗米 " } else { "" },
            if self.needs_sand_pick { "捡沙 " } else { "" },
            if self.needs_repair { "补袋" } else { "" },
        )
    }
}

// ======================================================================
// 第二部分: 袋子 (Bag) — 高维层级的基本单元
// ======================================================================

/// 袋子 — 高维层级的基本单元
///
/// 袋子不关心内部有多少颗米 (那是低维的事)
/// 袋子只关心: 我是谁? 我的状态怎么样? 我能不能用?
#[derive(Clone, Debug)]
pub struct Bag {
    /// 袋子唯一标识
    pub id: u64,
    /// 袋子名称 (可以是命名密码: mlll, gtt, lctqf 等)
    pub name: String,
    /// 袋子属性 (高维感知向量)
    pub attrs: BagAttributes,
    /// 袋子坐标 (空间位置)
    pub coord: Coord,
    /// 袋子所属层级 (层级深度: 0=顶层袋子, >0=套在别的袋子里)
    pub layer_depth: u32,
    /// 袋子内包含的子袋子ID列表 (层级关系)
    pub child_bag_ids: Vec<u64>,
}

impl Bag {
    /// 创建一个新袋子
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

    /// 创建并放到某个袋子内部 (子袋子)
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

    /// 袋子编码到64位单元格 (与 mirror.rs 集成)
    pub fn to_cell(&self) -> EncodedCell {
        self.attrs.to_cell()
    }

    /// 袋子的综合质量
    pub fn quality(&self) -> f64 {
        self.attrs.composite_quality()
    }

    /// 袋子是否可食用
    pub fn is_edible(&self) -> bool {
        self.attrs.is_edible()
    }

    /// 袋子是否需要处理
    pub fn needs_processing(&self) -> bool {
        self.attrs.needs_any_processing()
    }

    /// 袋子完整状态报告
    pub fn report(&self) -> String {
        format!("袋子#{}「{}」(层级{}): {}", self.id, self.name, self.layer_depth, self.attrs.summary())
    }
}

// ======================================================================
// 第三部分: 视觉系统 (VisualSystem) — 感知袋子外观
// ======================================================================

/// 视觉系统 — 感知袋子的外观属性
///
/// "视觉给做出来" — 看到袋子: 谁家的、值多少、脏不脏、破不破
#[derive(Clone, Debug)]
pub struct VisualSystem {
    /// 视觉敏感度 (0..1, 越高越能分辨细节)
    pub sensitivity: f64,
    /// 视觉阈值 (低于此值的属性差异无法分辨)
    pub threshold: f64,
}

impl VisualSystem {
    pub fn new(sensitivity: f64, threshold: f64) -> Self {
        Self {
            sensitivity: sensitivity.clamp(0.01, 1.0),
            threshold: threshold.clamp(0.01, 1.0),
        }
    }

    /// 默认视觉系统
    pub fn default_vision() -> Self {
        Self::new(0.8, 0.1)
    }

    /// 感知一个袋子: 生成视觉感相序列
    ///
    /// 视觉关注: 主权、价值、完好性、清洁度
    /// 这些是"一眼就能看到"的外观属性
    pub fn perceive(&self, bag: &Bag) -> Vec<SenseForm> {
        let mut forms = Vec::new();

        // 视觉感相: 主权归属
        forms.push(SenseForm::new(
            SenseKind::Vision,
            self.sensitivity,
            &format!("袋子#{} 主权={}", bag.id, match bag.attrs.sovereignty {
                0 => "无主", 1 => "妻子", 2 => "丈夫", 3 => "夫妻共有", _ => "未知"
            }),
            "visual_system",
            0,
        ));

        // 视觉感相: 价值
        forms.push(SenseForm::new(
            SenseKind::Vision,
            (bag.attrs.value as f64 / 1000.0) * self.sensitivity,
            &format!("袋子#{} 价值={}", bag.id, bag.attrs.value),
            "visual_system",
            0,
        ));

        // 视觉感相: 完好性
        if bag.attrs.integrity < 1.0 - self.threshold {
            forms.push(SenseForm::new(
                SenseKind::Vision,
                (1.0 - bag.attrs.integrity) * self.sensitivity,
                &format!("袋子#{} 破损度={:.0}%", bag.id, (1.0 - bag.attrs.integrity) * 100.0),
                "visual_system",
                0,
            ));
        }

        // 视觉感相: 清洁度
        if bag.attrs.bag_cleanliness < 1.0 - self.threshold {
            forms.push(SenseForm::new(
                SenseKind::Vision,
                (1.0 - bag.attrs.bag_cleanliness) * self.sensitivity,
                &format!("袋子#{} 脏污度={:.0}%", bag.id, (1.0 - bag.attrs.bag_cleanliness) * 100.0),
                "visual_system",
                0,
            ));
        }

        forms
    }

    /// 视觉判定: 袋子外观是否Ok
    pub fn appears_sound(&self, bag: &Bag) -> bool {
        bag.attrs.integrity > 0.5 && bag.attrs.bag_cleanliness > 0.5
    }
}

// ======================================================================
// 第四部分: 审判引擎 (JudgmentEngine) — 审判对了给奖励, 错了给惩罚
// ======================================================================

/// 审判结果 — 对袋子的裁决
#[derive(Clone, Debug)]
pub struct JudgmentResult {
    /// 被审判的袋子ID
    pub bag_id: u64,
    /// 审判判定: true=袋子合格(能吃/能用), false=不合格
    pub verdict_passed: bool,
    /// 审判置信度 0..1
    pub confidence: f64,
    /// 审判依据的属性
    pub reason: String,
    /// 审判时间戳
    pub timestamp_ms: u64,
}

/// 审判记录 — 包含真实反馈
#[derive(Clone, Debug)]
pub struct JudgmentRecord {
    /// 被审判的袋子ID
    pub bag_id: u64,
    /// 审判判定
    pub verdict: bool,
    /// 真实结果 (后续确认)
    pub actual: bool,
    /// 审判置信度
    pub confidence: f64,
    /// 获得的奖励/惩罚 (正=奖励, 负=惩罚)
    pub reward: f64,
    /// 审判时间戳
    pub timestamp_ms: u64,
}

impl JudgmentRecord {
    /// 审判是否正确
    pub fn was_correct(&self) -> bool {
        self.verdict == self.actual
    }

    /// 记录摘要
    pub fn summary(&self) -> String {
        let outcome = if self.was_correct() { "✓ 正确" } else { "✗ 错误" };
        let reward_str = if self.reward >= 0.0 {
            format!("+{:.2}奖励", self.reward)
        } else {
            format!("{:.2}惩罚", self.reward)
        };
        format!(
            "袋子#{} 审判={} 实际={} {} ({}) 置信度{:.0}%",
            self.bag_id,
            if self.verdict { "合格" } else { "不合格" },
            if self.actual { "确实合格" } else { "确实不合格" },
            outcome, reward_str,
            self.confidence
        )
    }
}

/// 审判引擎 — 审判对了给奖励, 审判错了给惩罚
///
/// 核心逻辑:
///   1. 审判: 对袋子做判定 (基于属性向量)
///   2. 反馈: 得知真实结果
///   3. 奖惩: 对了给奖励, 错了给惩罚
///   4. 学习: 记录到历史, 积累智慧
#[derive(Clone, Debug)]
pub struct JudgmentEngine {
    /// 基础元裁判引擎
    pub meta_judge: MetaJudgeEngine,
    /// 奖励池 (累积的总奖励)
    pub reward_pool: f64,
    /// 惩罚池 (累积的总惩罚, 存为正值)
    pub punishment_pool: f64,
    /// 审判记录历史
    pub records: Vec<JudgmentRecord>,
    /// 当前审判准确率
    pub current_accuracy: f64,
    /// 连续正确次数 (触发额外奖励)
    pub streak_correct: u64,
    /// 连续错误次数 (触发额外惩罚/警惕)
    pub streak_wrong: u64,
    /// 最大记录数
    pub max_records: usize,
    /// 基础奖励值
    pub base_reward: f64,
    /// 基础惩罚值
    pub base_punishment: f64,
    /// 审判阈值 (判定合格的综合质量阈值)
    pub pass_threshold: f64,
}

impl JudgmentEngine {
    pub fn new(max_memory_layers: usize) -> Self {
        Self {
            meta_judge: MetaJudgeEngine::new(max_memory_layers),
            reward_pool: 0.0,
            punishment_pool: 0.0,
            records: Vec::new(),
            current_accuracy: 0.5, // 初始假设50%
            streak_correct: 0,
            streak_wrong: 0,
            max_records: 1000,
            base_reward: 1.0,
            base_punishment: 1.0,
            pass_threshold: 0.6, // 综合质量>=0.6判定合格
        }
    }

    /// 审判一个袋子
    ///
    /// 判定逻辑:
    ///   - 综合质量 >= 概率阈值 → 判定合格
    ///   - 综合质量 < 概率阈值 → 判定不合格
    ///   - 计算置信度 (基于属性与阈值的距离)
    pub fn judge(&self, bag: &Bag, os: &dyn OsLayer) -> JudgmentResult {
        let quality = bag.attrs.composite_quality();
        let verdict = quality >= self.pass_threshold;
        let confidence = if verdict {
            // 质量超过阈值越多, 置信度越高
            ((quality - self.pass_threshold) / (1.0 - self.pass_threshold)).min(1.0).max(0.5)
        } else {
            // 质量低于阈值越多, 置信度越高 (确信它不合格)
            ((self.pass_threshold - quality) / self.pass_threshold).min(1.0).max(0.5)
        };
        let reason = if verdict {
            format!("综合质量{:.0}% >= 阈值{:.0}%", quality * 100.0, self.pass_threshold * 100.0)
        } else {
            format!("综合质量{:.0}% < 阈值{:.0}%", quality * 100.0, self.pass_threshold * 100.0)
        };

        JudgmentResult {
            bag_id: bag.id,
            verdict_passed: verdict,
            confidence,
            reason,
            timestamp_ms: current_timestamp_ms(os),
        }
    }

    /// 提交真实反馈, 获得奖励或惩罚
    ///
    /// 返回值: 正=奖励, 负=惩罚
    pub fn feedback(&mut self, bag_id: u64, verdict: bool, actual: bool, confidence: f64, os: &dyn OsLayer) -> f64 {
        let correct = verdict == actual;

        // 计算奖励/惩罚
        let reward = if correct {
            let base = self.base_reward * confidence;
            // 连续正确 → 额外奖励
            self.streak_correct += 1;
            self.streak_wrong = 0;
            let streak_bonus = if self.streak_correct >= 3 {
                self.streak_correct as f64 * 0.1
            } else {
                0.0
            };
            base + streak_bonus
        } else {
            let base = -(self.base_punishment * confidence);
            // 连续错误 → 额外惩罚
            self.streak_wrong += 1;
            self.streak_correct = 0;
            let streak_penalty = if self.streak_wrong >= 3 {
                -(self.streak_wrong as f64 * 0.15)
            } else {
                0.0
            };
            base + streak_penalty
        };

        // 更新池
        if reward >= 0.0 {
            self.reward_pool += reward;
        } else {
            self.punishment_pool += reward.abs();
        }

        // 记录到历史
        let record = JudgmentRecord {
            bag_id,
            verdict,
            actual,
            confidence,
            reward,
            timestamp_ms: current_timestamp_ms(os),
        };
        self.records.push(record);

        // 更新准确率 (在记录推送后计算)
        self.update_accuracy();

        // 控制历史大小
        if self.records.len() > self.max_records {
            self.records.remove(0);
        }

        reward
    }

    /// 计算当前准确率
    fn update_accuracy(&mut self) {
        if self.records.is_empty() {
            self.current_accuracy = 0.5;
            return;
        }
        let correct_count = self.records.iter().filter(|r| r.was_correct()).count();
        self.current_accuracy = correct_count as f64 / self.records.len() as f64;
    }

    /// 审判并反馈的便捷方法
    pub fn judge_and_feedback(&mut self, bag: &Bag, actual_good: bool, os: &dyn OsLayer) -> (JudgmentResult, f64) {
        let result = self.judge(bag, os);
        let reward = self.feedback(result.bag_id, result.verdict_passed, actual_good, result.confidence, os);
        (result, reward)
    }

    /// 获取最近N条记录
    pub fn recent_records(&self, n: usize) -> &[JudgmentRecord] {
        let start = self.records.len().saturating_sub(n);
        &self.records[start..]
    }

    /// 审判历史摘要
    pub fn history_summary(&self) -> String {
        let total = self.records.len();
        let correct = self.records.iter().filter(|r| r.was_correct()).count();
        let total_reward: f64 = self.records.iter().map(|r| if r.reward > 0.0 { r.reward } else { 0.0 }).sum();
        let total_punishment: f64 = self.records.iter().map(|r| if r.reward < 0.0 { r.reward.abs() } else { 0.0 }).sum();
        format!(
            "审判总数={} 正确={}({:.0}%) 总奖励={:.2} 总惩罚={:.2} 连续正确={} 连续错误={}",
            total, correct, self.current_accuracy, total_reward, total_punishment,
            self.streak_correct, self.streak_wrong
        )
    }
}

// ======================================================================
// 第五部分: 历史积累 (HistoricalAccumulation) — 历史的记忆力
// ======================================================================

/// 时代 — 历史的一个阶段
#[derive(Clone, Debug)]
pub struct Epoch {
    /// 起始 tick
    pub start_tick: u64,
    /// 结束 tick
    pub end_tick: u64,
    /// 该时代审判准确率
    pub accuracy: f64,
    /// 该时代属性分布摘要
    pub attribute_snapshot: String,
}

/// 历史积累 — 记忆的累积与演化
///
/// "历史的记忆力和历史的积累"
///   - 不是简单记录, 而是每个时代的认知结晶
///   - 高准确率的时代产生更多"智慧点数"
///   - 低准确率的时代产生教训 (避免重复犯错)
#[derive(Clone, Debug)]
pub struct HistoricalAccumulation {
    /// 历史时代序列
    pub epochs: Vec<Epoch>,
    /// 智慧点数 (历史积累的量化)
    pub wisdom_score: f64,
    /// 总审判次数
    pub total_judgments: u64,
    /// 总正确次数
    pub total_correct: u64,
    /// 历史最佳准确率
    pub best_accuracy: f64,
    /// 当前时代起始 tick
    pub current_epoch_start: u64,
    /// 当前时代审判记录暂存
    pub current_epoch_records: Vec<JudgmentRecord>,
    /// 每个时代的审判数量
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
            current_epoch_records: Vec::new(),
            epoch_capacity,
        }
    }

    /// 记录一次审判到当前时代
    pub fn record(&mut self, record: JudgmentRecord) {
        self.current_epoch_records.push(record.clone());
        self.total_judgments += 1;
        if record.was_correct() {
            self.total_correct += 1;
        }

        // 当前时代已满 → 封存时代, 产生新纪元
        if self.current_epoch_records.len() as u64 >= self.epoch_capacity {
            self.seal_epoch(record.timestamp_ms);
        }
    }

    /// 封存当前时代
    fn seal_epoch(&mut self, end_tick: u64) {
        if self.current_epoch_records.is_empty() {
            return;
        }

        let correct = self.current_epoch_records.iter().filter(|r| r.was_correct()).count();
        let accuracy = correct as f64 / self.current_epoch_records.len() as f64;

        // 智慧点数 = 准确率 × 时代内审判数 × 置信度加权
        let confidence_sum: f64 = self.current_epoch_records.iter().map(|r| r.confidence).sum();
        let avg_confidence = confidence_sum / self.current_epoch_records.len() as f64;
        let wisdom_gained = accuracy * self.current_epoch_records.len() as f64 * avg_confidence;
        self.wisdom_score += wisdom_gained;

        // 更新最佳准确率
        if accuracy > self.best_accuracy {
            self.best_accuracy = accuracy;
        }

        // 创建时代记录
        let epoch = Epoch {
            start_tick: self.current_epoch_start,
            end_tick,
            accuracy,
            attribute_snapshot: format!(
                "{}次审判, {:.0}%准确, 获得{:.1}智慧",
                self.current_epoch_records.len(), accuracy, wisdom_gained
            ),
        };
        self.epochs.push(epoch);

        // 开始新时代
        self.current_epoch_records.clear();
        self.current_epoch_start = end_tick;
    }

    /// 强制封存 (在结束时调用)
    pub fn finalize(&mut self, end_tick: u64) {
        self.seal_epoch(end_tick);
    }

    /// 获取历史智慧点数
    pub fn wisdom(&self) -> f64 {
        self.wisdom_score
    }

    /// 获取总体准确率
    pub fn overall_accuracy(&self) -> f64 {
        if self.total_judgments == 0 { return 0.0; }
        self.total_correct as f64 / self.total_judgments as f64
    }

    /// 获取历史摘要
    pub fn summary(&self) -> String {
        format!(
            "历史积累: {}个时代, 总审判{}次, 总正确率{:.0}%, 最佳{:.0}%, 智慧点数{:.1}",
            self.epochs.len(), self.total_judgments, self.overall_accuracy() * 100.0,
            self.best_accuracy * 100.0, self.wisdom_score
        )
    }
}

// ======================================================================
// 第六部分: 天觉 (CelestialPerception) — 高维感知引擎
// ======================================================================

/// 天觉 — 高维感知引擎
///
/// 核心特性:
///   - 需授权才能运作 ("把天觉给授权")
///   - 一次性感知14维属性 (不需要逐个枚举)
///   - 视觉 + 天觉合一 = 完整袋子认知
///   - 集成审判引擎和历史积累
pub struct CelestialPerception<'a> {
    /// 是否已授权
    authorized: bool,
    /// 视觉系统
    pub visual: VisualSystem,
    /// 观测者 (感知入口)
    observer: Observer<'a>,
    /// 审判引擎
    pub judgment: JudgmentEngine,
    /// 历史积累
    pub history: HistoricalAccumulation,
    /// 感知的袋子计数
    pub bags_perceived: u64,
}

impl<'a> CelestialPerception<'a> {
    /// 创建天觉 (默认未授权)
    pub fn new(os: &'a dyn OsLayer) -> Self {
        Self {
            authorized: false,
            visual: VisualSystem::default_vision(),
            observer: Observer::new(1, "天觉", 64, os),
            judgment: JudgmentEngine::new(32),
            history: HistoricalAccumulation::new(50, 0),
            bags_perceived: 0,
        }
    }

    /// 授权天觉 ("把天觉给授权")
    pub fn authorize(&mut self) {
        self.authorized = true;
    }

    /// 检查是否已授权
    pub fn is_authorized(&self) -> bool {
        self.authorized
    }

    /// 撤销授权
    pub fn revoke(&mut self) {
        self.authorized = false;
    }

    /// 天觉感知 — 一次性感知袋子的完整14维状态
    ///
    /// 前提: 必须已授权
    /// 输出: SenseForm 序列送入 Observer
    pub fn perceive(&mut self, bag: &Bag) -> Result<Vec<SenseForm>, &'static str> {
        if !self.authorized {
            return Err("天觉未授权, 无法感知");
        }

        let mut forms = Vec::new();

        // === 外观感知 (视觉) ===
        let visual_forms = self.visual.perceive(bag);
        forms.extend(visual_forms);

        // === 天觉感知 (高维整体) ===
        // 天觉一次性"看到"全部14维属性, 不是逐个枚举
        // 这是一种"直觉式"的整体把握

        // 感相: 内容质量 (味觉/内在品质)
        forms.push(SenseForm::new(
            SenseKind::Taste,
            bag.attrs.content_quality * self.visual.sensitivity,
            &format!("袋#{} 内容质量={:.0}% 纯度={:.0}% 虫={:.0}% 沙={:.0}%",
                bag.id, bag.attrs.content_quality * 100.0, bag.attrs.content_purity * 100.0,
                bag.attrs.pest_level * 100.0, bag.attrs.sand_level * 100.0),
            "celestial_perception",
            0,
        ));

        // 感相: 可食性 (内部直觉)
        forms.push(SenseForm::new(
            SenseKind::Mind,
            bag.attrs.edibility * self.visual.sensitivity,
            &format!("袋#{} 可食性={:.0}% 干燥={:.0}%",
                bag.id, bag.attrs.edibility * 100.0, bag.attrs.dryness * 100.0),
            "celestial_perception",
            0,
        ));

        // 感相: 是否需要加工 (系统性认知)
        if bag.attrs.needs_any_processing() {
            forms.push(SenseForm::new(
                SenseKind::Mind,
                0.8,
                &format!("袋#{} 需加工: {}{}{}",
                    bag.id,
                    if bag.attrs.needs_content_wash { "洗米 " } else { "" },
                    if bag.attrs.needs_sand_pick { "捡沙 " } else { "" },
                    if bag.attrs.needs_repair { "补袋" } else { "" }),
                "celestial_perception",
                0,
            ));
        }

        // 将感相送入观测者 (驱动意识密度)
        let cells = self.observer.sense_batch(forms.clone());
        self.bags_perceived += 1;

        Ok(forms)
    }

    /// 天觉审判 — 感知 + 审判 + 历史积累 一体化
    ///
    /// 返回: (审判结果, 奖励/惩罚)
    pub fn perceive_and_judge(&mut self, bag: &Bag, actual_good: bool) -> Result<(JudgmentResult, f64), &'static str> {
        if !self.authorized {
            return Err("天觉未授权");
        }

        // 1. 感知 (生成感相, 驱动意识)
        let _forms = self.perceive(bag)?;

        // 2. 审判
        let os = self.observer.os();
        let (result, reward) = self.judgment.judge_and_feedback(bag, actual_good, os);

        // 3. 记录到历史
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

    /// 获取当前意识密度
    pub fn consciousness_density(&self) -> f64 {
        self.observer.consciousness_density()
    }

    /// 获取天觉报告
    pub fn report(&self) -> String {
        let mut s = String::new();
        s.push_str("═══ 天觉感知报告 ═══\n");
        s.push_str(&format!("  授权状态: {}\n", if self.authorized { "已授权 ◀" } else { "未授权" }));
        s.push_str(&format!("  已感知袋子数: {}\n", self.bags_perceived));
        s.push_str(&format!("  视觉敏感度: {:.0}%\n", self.visual.sensitivity * 100.0));
        s.push_str(&format!("  意识密度: {:.3}\n", self.consciousness_density()));
        s.push_str(&format!("  {}\n", self.judgment.history_summary()));
        s.push_str(&format!("  {}\n", self.history.summary()));
        s
    }
}

// ======================================================================
// 第七部分: 袋子层级 (BagHierarchy) — 高维层级的逻辑认知
// ======================================================================

/// 袋子层级 — 高维层级的逻辑认知
///
/// "层级关系, 绝对不是低维度的层级, 而是高维度的层级"
///
/// 低维层级: 米粒 → 袋子 → 仓库 (逐层枚举)
/// 高维层级: 袋子 (唯一操作单元), 袋子可以包含袋子 (直接操作)
///
/// 高维层级规则:
///   1. 只命名袋子, 不命名米粒
///   2. 每个袋子有14维属性向量
///   3. 操作 (审判/清洗/烘干) 在袋子层面进行
///   4. 层级关系通过 child_bag_ids 表达
#[derive(Clone, Debug)]
pub struct BagHierarchy {
    /// 所有袋子 (ID → 袋子)
    pub bags: Vec<Bag>,
    /// 最大袋子数量
    pub max_bags: usize,
    /// 层级深度限制
    pub max_depth: u32,
    /// 袋子数量统计
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

    /// 添加顶层袋子
    pub fn add_bag(&mut self, mut bag: Bag) -> Result<(), &'static str> {
        if self.bags.len() >= self.max_bags {
            return Err("层级已满, 无法添加");
        }
        bag.layer_depth = 0;
        self.total_count += 1;
        self.bags.push(bag);
        Ok(())
    }

    /// 添加子袋子到指定父袋子内 (层级嵌套)
    pub fn add_nested(&mut self, parent_id: u64, mut bag: Bag) -> Result<(), &'static str> {
        if self.bags.len() >= self.max_bags {
            return Err("层级已满");
        }

        // 查找父袋子
        let parent_depth = match self.bags.iter().find(|b| b.id == parent_id) {
            Some(parent) => {
                if parent.layer_depth >= self.max_depth {
                    return Err("已达到最大层级深度");
                }
                parent.layer_depth
            }
            None => return Err("父袋子不存在"),
        };

        bag.layer_depth = parent_depth + 1;
        let bag_id = bag.id;
        self.bags.push(bag);

        // 更新父袋子的子列表
        if let Some(parent) = self.bags.iter_mut().find(|b| b.id == parent_id) {
            parent.child_bag_ids.push(bag_id);
        }

        self.total_count += 1;
        Ok(())
    }

    /// 统计袋子数量 (高维: 只数袋子, 不数米粒)
    pub fn count_bags(&self) -> usize {
        self.bags.len()
    }

    /// 按层数统计袋子
    pub fn count_by_layer(&self) -> Vec<(u32, usize)> {
        let mut counts: std::collections::HashMap<u32, usize> = std::collections::HashMap::new();
        for bag in &self.bags {
            *counts.entry(bag.layer_depth).or_insert(0) += 1;
        }
        let mut result: Vec<(u32, usize)> = counts.into_iter().collect();
        result.sort_by_key(|x| x.0);
        result
    }

    /// 获取指定ID的袋子
    pub fn get(&self, id: u64) -> Option<&Bag> {
        self.bags.iter().find(|b| b.id == id)
    }

    /// 评估层级的整体健康度 (高维: 袋子层面的评估)
    pub fn evaluate(&self) -> HierarchyReport {
        let total = self.bags.len();
        if total == 0 {
            return HierarchyReport::empty();
        }

        let edible_count = self.bags.iter().filter(|b| b.is_edible()).count();
        let needs_processing_count = self.bags.iter().filter(|b| b.needs_processing()).count();
        let total_quality: f64 = self.bags.iter().map(|b| b.quality()).sum();
        let avg_quality = total_quality / total as f64;

        // 层级分布
        let layer_distribution = self.count_by_layer();

        HierarchyReport {
            total_bags: total,
            edible_bags: edible_count,
            needs_processing: needs_processing_count,
            average_quality: avg_quality,
            layer_distribution,
        }
    }
}

/// 层级评估报告
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
        if self.total_bags == 0 {
            return "层级为空".to_string();
        }
        let layer_info: Vec<String> = self.layer_distribution.iter()
            .map(|(depth, count)| format!("L{}:{}个", depth, count))
            .collect();
        format!(
            "层级报告: {}个袋子, 可食{}({:.0}%), 需处理{}个, 平均质量{:.0}%, 层级分布[{}]",
            self.total_bags,
            self.edible_bags,
            (self.edible_bags as f64 / self.total_bags as f64) * 100.0,
            self.needs_processing,
            self.average_quality * 100.0,
            layer_info.join(", ")
        )
    }
}

// ======================================================================
// 辅助函数
// ======================================================================

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
    fn bag_attributes_pristine_is_perfect() {
        let attrs = BagAttributes::pristine();
        assert!(attrs.composite_quality() > 0.95);
        assert!(attrs.is_edible());
        assert!(!attrs.needs_any_processing());
    }

    #[test]
    fn bag_attributes_rotten_is_inedible() {
        let attrs = BagAttributes::rotten();
        assert!(attrs.composite_quality() < 0.1);
        assert!(!attrs.is_edible());
        assert!(attrs.needs_any_processing());
    }

    #[test]
    fn bag_attributes_edibility_threshold() {
        let mut attrs = BagAttributes::pristine();
        attrs.edibility = 0.5; // 低于0.7阈值
        assert!(!attrs.is_edible());

        attrs.edibility = 0.8;
        attrs.pest_level = 0.5; // 虫太多
        assert!(!attrs.is_edible());

        attrs.pest_level = 0.0;
        assert!(attrs.is_edible());
    }

    #[test]
    fn bag_attributes_needs_drying() {
        let mut attrs = BagAttributes::pristine();
        attrs.dryness = 0.5;
        assert!(attrs.needs_drying());
        attrs.dryness = 0.8;
        assert!(!attrs.needs_drying());
    }

    #[test]
    fn bag_new_creates_bag() {
        let attrs = BagAttributes::pristine();
        let bag = Bag::new(1, "米袋", attrs, (10, 20, 30));
        assert_eq!(bag.id, 1);
        assert_eq!(bag.name, "米袋");
        assert_eq!(bag.attrs.value, 800);
        assert_eq!(bag.layer_depth, 0);
    }

    #[test]
    fn bag_to_cell_roundtrip_partial() {
        let attrs = BagAttributes::pristine();
        let cell = attrs.to_cell();
        // 解码部分信息
        let decoded = BagAttributes::from_cell(cell, None);
        // 完好性近似 (允许误差)
        assert!((decoded.integrity - attrs.integrity).abs() < 0.1);
    }

    #[test]
    fn visual_system_perceives_bag() {
        let vs = VisualSystem::default_vision();
        let bag = Bag::new(1, "测试袋", BagAttributes::pristine(), (0, 0, 0));
        let forms = vs.perceive(&bag);
        // 至少产生主权和价值两个视觉感相
        assert!(forms.len() >= 2);
    }

    #[test]
    fn visual_system_detects_damage() {
        let vs = VisualSystem::default_vision();
        let mut attrs = BagAttributes::pristine();
        attrs.integrity = 0.3; // 破损严重
        attrs.bag_cleanliness = 0.2; // 很脏
        let bag = Bag::new(1, "破袋", attrs, (0, 0, 0));
        let forms = vs.perceive(&bag);
        // 破损和脏污也会产生感相
        assert!(forms.len() >= 3);
    }

    #[test]
    fn visual_system_appears_sound() {
        let vs = VisualSystem::default_vision();
        let good_bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        assert!(vs.appears_sound(&good_bag));

        let mut bad_attrs = BagAttributes::pristine();
        bad_attrs.integrity = 0.2;
        let bad_bag = Bag::new(2, "破袋", bad_attrs, (0, 0, 0));
        assert!(!vs.appears_sound(&bad_bag));
    }

    #[test]
    fn judgment_engine_judges_correctly() {
        let engine = JudgmentEngine::new(16);
        let os = BareMetalOs::new();
        let good_bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let bad_bag = Bag::new(2, "坏袋", BagAttributes::rotten(), (0, 0, 0));

        let result_good = engine.judge(&good_bag, &os);
        assert!(result_good.verdict_passed);

        let result_bad = engine.judge(&bad_bag, &os);
        assert!(!result_bad.verdict_passed);
    }

    #[test]
    fn judgment_engine_feedback_reward() {
        let mut engine = JudgmentEngine::new(16);
        let os = BareMetalOs::new();
        // 审判正确 → 奖励
        let reward = engine.feedback(1, true, true, 0.9, &os);
        assert!(reward > 0.0);
        assert!(engine.reward_pool > 0.0);
        assert_eq!(engine.streak_correct, 1);
    }

    #[test]
    fn judgment_engine_feedback_punishment() {
        let mut engine = JudgmentEngine::new(16);
        let os = BareMetalOs::new();
        // 审判错误 → 惩罚
        let reward = engine.feedback(1, true, false, 0.9, &os);
        assert!(reward < 0.0);
        assert!(engine.punishment_pool > 0.0);
        assert_eq!(engine.streak_wrong, 1);
    }

    #[test]
    fn judgment_engine_streak_bonus() {
        let mut engine = JudgmentEngine::new(16);
        let os = BareMetalOs::new();
        // 连续正确3次 → 获得额外奖励
        engine.feedback(1, true, true, 0.9, &os);
        engine.feedback(2, true, true, 0.9, &os);
        let reward3 = engine.feedback(3, true, true, 0.9, &os);
        assert!(reward3 > engine.base_reward * 0.9 + 0.2); // 包含连胜加成
    }

    #[test]
    fn judgment_engine_accuracy_tracking() {
        let mut engine = JudgmentEngine::new(16);
        let os = BareMetalOs::new();
        // 3次正确
        engine.feedback(1, true, true, 0.9, &os);
        engine.feedback(2, false, false, 0.9, &os);
        engine.feedback(3, true, true, 0.9, &os);
        // 1次错误
        engine.feedback(4, true, false, 0.9, &os);
        // 准确率 = 3/4
        assert!((engine.current_accuracy - 0.75).abs() < 0.01);
    }

    #[test]
    fn judgment_engine_judge_and_feedback() {
        let mut engine = JudgmentEngine::new(16);
        let os = BareMetalOs::new();
        let good_bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let (result, reward) = engine.judge_and_feedback(&good_bag, true, &os);
        assert!(result.verdict_passed);
        assert!(reward > 0.0);
    }

    #[test]
    fn historical_accumulation_epochs() {
        let mut hist = HistoricalAccumulation::new(3, 0);
        // 添加3条记录 (触发封存)
        for i in 1..=3 {
            let record = JudgmentRecord {
                bag_id: i,
                verdict: true,
                actual: true,
                confidence: 0.9,
                reward: 0.9,
                timestamp_ms: i as u64 * 100,
            };
            hist.record(record);
        }
        // 第一个时代应被封印
        assert_eq!(hist.epochs.len(), 1);
        assert!(hist.wisdom_score > 0.0);
    }

    #[test]
    fn historical_accumulation_finalize() {
        let mut hist = HistoricalAccumulation::new(10, 0);
        for i in 1..=5 {
            let record = JudgmentRecord {
                bag_id: i,
                verdict: i % 2 == 0,
                actual: i % 2 == 0,
                confidence: 0.8,
                reward: 0.8,
                timestamp_ms: i as u64 * 100,
            };
            hist.record(record);
        }
        hist.finalize(600);
        // 应有一个时代
        assert_eq!(hist.epochs.len(), 1);
        assert_eq!(hist.total_judgments, 5);
    }

    #[test]
    fn historical_accumulation_wisdom_growth() {
        let mut hist = HistoricalAccumulation::new(2, 0);
        // 2次正确 → 一次封存
        let r1 = JudgmentRecord { bag_id: 1, verdict: true, actual: true, confidence: 0.9, reward: 0.9, timestamp_ms: 100 };
        let r2 = JudgmentRecord { bag_id: 2, verdict: true, actual: true, confidence: 0.9, reward: 0.9, timestamp_ms: 200 };
        hist.record(r1);
        hist.record(r2);
        let wisdom_after_first = hist.wisdom_score;
        assert!(wisdom_after_first > 0.0);
    }

    #[test]
    fn celestial_perception_requires_authorization() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerception::new(&os);
        assert!(!cp.is_authorized());

        let bag = Bag::new(1, "测试", BagAttributes::pristine(), (0, 0, 0));
        let result = cp.perceive(&bag);
        assert!(result.is_err());
    }

    #[test]
    fn celestial_perception_perceives_after_authorization() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerception::new(&os);
        cp.authorize();
        assert!(cp.is_authorized());

        let bag = Bag::new(1, "测试", BagAttributes::pristine(), (0, 0, 0));
        let result = cp.perceive(&bag);
        assert!(result.is_ok());
        let forms = result.unwrap();
        // 视觉(2+)+味觉(1)+心智(1+) = 至少4个感相
        assert!(forms.len() >= 4);
    }

    #[test]
    fn celestial_perception_full_pipeline() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerception::new(&os);
        cp.authorize();

        let good_bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let bad_bag = Bag::new(2, "坏袋", BagAttributes::rotten(), (0, 0, 0));

        // 审判好袋 (实际也好) → 奖励
        let (result1, reward1) = cp.perceive_and_judge(&good_bag, true).unwrap();
        assert!(result1.verdict_passed);
        assert!(reward1 > 0.0);

        // 审判坏袋 (实际也坏) → 奖励 (判断正确)
        let (result2, reward2) = cp.perceive_and_judge(&bad_bag, false).unwrap();
        assert!(!result2.verdict_passed);
        assert!(reward2 > 0.0);

        // 历史应积累
        assert!(cp.history.total_judgments >= 2);
    }

    #[test]
    fn celestial_perception_punishment_on_wrong() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerception::new(&os);
        cp.authorize();

        // 审判错误: 实际是好袋, 但审判为坏袋
        // 实际上 pristine bag 会审判为合格, 所以我们需要制造判断错误
        let bag = Bag::new(1, "测试", BagAttributes::pristine(), (0, 0, 0));
        // 手动调用 judgment 的 feedback
        let reward = cp.judgment.feedback(1, false, true, 0.9, &os); // 判定不合格, 但实际合格
        assert!(reward < 0.0);
        assert!(cp.judgment.punishment_pool > 0.0);
    }

    #[test]
    fn celestial_perception_report() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerception::new(&os);
        cp.authorize();
        let bag = Bag::new(1, "测试", BagAttributes::pristine(), (0, 0, 0));
        cp.perceive(&bag).unwrap();
        let report = cp.report();
        assert!(report.contains("天觉感知报告"));
        assert!(report.contains("已授权"));
    }

    #[test]
    fn bag_hierarchy_add_and_count() {
        let mut hier = BagHierarchy::new(100, 5);
        let bag1 = Bag::new(1, "袋1", BagAttributes::pristine(), (0, 0, 0));
        let bag2 = Bag::new(2, "袋2", BagAttributes::pristine(), (1, 0, 0));

        hier.add_bag(bag1).unwrap();
        hier.add_bag(bag2).unwrap();

        assert_eq!(hier.count_bags(), 2);
        assert_eq!(hier.total_count, 2);
    }

    #[test]
    fn bag_hierarchy_nested() {
        let mut hier = BagHierarchy::new(100, 5);
        let parent = Bag::new(1, "父袋", BagAttributes::pristine(), (0, 0, 0));
        let child = Bag::new(2, "子袋", BagAttributes::pristine(), (0, 0, 0));

        hier.add_bag(parent).unwrap();
        hier.add_nested(1, child).unwrap();

        // 父袋应有子袋引用
        let parent_ref = hier.get(1).unwrap();
        assert_eq!(parent_ref.child_bag_ids, vec![2]);

        // 子袋层级深度 = 1
        let child_ref = hier.get(2).unwrap();
        assert_eq!(child_ref.layer_depth, 1);
    }

    #[test]
    fn bag_hierarchy_depth_limit() {
        let mut hier = BagHierarchy::new(100, 2); // 最大深度2
        let bag1 = Bag::new(1, "袋1", BagAttributes::pristine(), (0, 0, 0));
        let bag2 = Bag::new(2, "袋2", BagAttributes::pristine(), (0, 0, 0));
        let bag3 = Bag::new(3, "袋3", BagAttributes::pristine(), (0, 0, 0));

        hier.add_bag(bag1).unwrap();
        hier.add_nested(1, bag2).unwrap(); // depth=1
        let result = hier.add_nested(2, bag3); // depth=2
        assert!(result.is_ok());
        // 再嵌套就会超过深度限制
        let bag4 = Bag::new(4, "袋4", BagAttributes::pristine(), (0, 0, 0));
        let result2 = hier.add_nested(3, bag4); // depth=3 > 2
        assert!(result2.is_err());
    }

    #[test]
    fn bag_hierarchy_evaluate() {
        let mut hier = BagHierarchy::new(100, 5);
        let good_bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let bad_bag = Bag::new(2, "坏袋", BagAttributes::rotten(), (0, 0, 0));
        hier.add_bag(good_bag).unwrap();
        hier.add_bag(bad_bag).unwrap();

        let report = hier.evaluate();
        assert_eq!(report.total_bags, 2);
        assert_eq!(report.edible_bags, 1);
        assert!(report.average_quality > 0.0 && report.average_quality < 1.0);
    }

    #[test]
    fn bag_hierarchy_empty_evaluate() {
        let hier = BagHierarchy::new(100, 5);
        let report = hier.evaluate();
        assert_eq!(report.total_bags, 0);
    }

    #[test]
    fn bag_hierarchy_layer_distribution() {
        let mut hier = BagHierarchy::new(100, 5);
        let bag1 = Bag::new(1, "袋1", BagAttributes::pristine(), (0, 0, 0));
        let bag2 = Bag::new(2, "袋2", BagAttributes::pristine(), (0, 0, 0));
        let bag3 = Bag::new(3, "袋3", BagAttributes::pristine(), (0, 0, 0));

        hier.add_bag(bag1).unwrap();
        hier.add_nested(1, bag2).unwrap();
        hier.add_nested(1, bag3).unwrap();

        let distribution = hier.count_by_layer();
        assert_eq!(distribution.len(), 2); // L0 和 L1
    }

    #[test]
    fn bag_hierarchy_max_bags_limit() {
        let mut hier = BagHierarchy::new(2, 5); // 最多2个
        let bag1 = Bag::new(1, "袋1", BagAttributes::pristine(), (0, 0, 0));
        let bag2 = Bag::new(2, "袋2", BagAttributes::pristine(), (0, 0, 0));
        let bag3 = Bag::new(3, "袋3", BagAttributes::pristine(), (0, 0, 0));

        assert!(hier.add_bag(bag1).is_ok());
        assert!(hier.add_bag(bag2).is_ok());
        assert!(hier.add_bag(bag3).is_err()); // 已满
    }

    #[test]
    fn bag_attributes_to_cell_encodes_polarity() {
        let good = BagAttributes::pristine();
        let good_cell = good.to_cell();
        // 内容质量1.0 > 0.5 → 应是正极性
        assert_eq!(mirror::get_polarity(good_cell), POLARITY_POSITIVE);

        let bad = BagAttributes::rotten();
        let bad_cell = bad.to_cell();
        // 内容质量0.0 < 0.5 → 应是负极性
        assert_eq!(mirror::get_polarity(bad_cell), POLARITY_NEGATIVE);
    }

    #[test]
    fn bag_attributes_processing_quality() {
        let mut attrs = BagAttributes::pristine();
        // 需要加工 (洗米)
        attrs.needs_content_wash = true;
        attrs.content_purity = 0.3; // 不纯
        attrs.wash_target = 0.9;

        let before = attrs.composite_quality();
        let after = attrs.quality_after_processing();
        // 加工后质量应提升
        assert!(after > before, "加工后质量应提升: {} → {}", before, after);
    }

    #[test]
    fn judgment_record_summary_correct() {
        let record = JudgmentRecord {
            bag_id: 1,
            verdict: true,
            actual: true,
            confidence: 0.9,
            reward: 0.9,
            timestamp_ms: 1000,
        };
        let s = record.summary();
        assert!(s.contains("正确"));
        assert!(s.contains("奖励"));
    }

    #[test]
    fn judgment_record_summary_wrong() {
        let record = JudgmentRecord {
            bag_id: 1,
            verdict: true,
            actual: false,
            confidence: 0.9,
            reward: -0.9,
            timestamp_ms: 1000,
        };
        let s = record.summary();
        assert!(s.contains("错误"));
        assert!(s.contains("惩罚"));
    }

    #[test]
    fn judgment_engine_confidence_scales_with_distance() {
        let engine = JudgmentEngine::new(16);
        let os = BareMetalOs::new();
        // 极高质量的袋子 → 高置信度
        let excellent = Bag::new(1, "极好", BagAttributes::pristine(), (0, 0, 0));
        let result_excellent = engine.judge(&excellent, &os);
        assert!(result_excellent.confidence > 0.9);

        // 中等质量袋子 → 中等置信度
        let mut medium_attrs = BagAttributes::pristine();
        medium_attrs.content_quality = 0.65; // 接近阈值
        let medium = Bag::new(2, "中等", medium_attrs, (0, 0, 0));
        let result_medium = engine.judge(&medium, &os);
        assert!(result_medium.confidence >= 0.5 && result_medium.confidence <= 0.9);
    }

    #[test]
    fn full_pipeline_celestial_to_hierarchy() {
        // 完整管道: 天觉 → 审判 → 历史 → 层级
        let os = BareMetalOs::new();
        let mut cp = CelestialPerception::new(&os);
        cp.authorize();

        let mut hier = BagHierarchy::new(100, 5);

        // 创建多个袋子
        let bags = vec![
            Bag::new(1, "优质米袋", BagAttributes::pristine(), (0, 0, 0)),
            Bag::new(2, "劣质米袋", BagAttributes::rotten(), (1, 0, 0)),
            Bag::new(3, "需加工袋", {
                let mut a = BagAttributes::pristine();
                a.needs_content_wash = true;
                a.content_purity = 0.4;
                a
            }, (2, 0, 0)),
        ];

        for bag in &bags {
            hier.add_bag(bag.clone()).unwrap();
            let actual_good = bag.is_edible();
            cp.perceive_and_judge(bag, actual_good).unwrap();
        }

        // 评估层级
        let report = hier.evaluate();
        assert_eq!(report.total_bags, 3);

        // 历史应有积累
        assert!(cp.history.total_judgments >= 3);
    }
}
