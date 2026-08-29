// ══════════════════════════════════════════════════════════════════
// tianjue.rs — 天觉 · 高维层级认知 · 视觉授权 · 容器奖惩 · 历史积累
// ══════════════════════════════════════════════════════════════════
//
// 核心命题:
//   不数米粒 — 不处理原子级细节
//   只命名袋子 — 只为容器级集合赋予属性
//
//   袋子的属性维度 (高维层级):
//     · 归属     — 谁家的袋子
//     · 价值     — 值多少钱
//     · 状态     — 还好不好用
//     · 内容质量 — 里面东西好不好吃
//     · 清洁度   — 脏不脏 / 要不要洗
//     · 干燥度   — 晒干了没有
//     · 虫害检查 — 有没有虫蛀
//     · 杂质检查 — 有没有沙子 / 能不能吃
//
//   审判规则:
//     · 审判对了 → 奖励 (容器活力提升、记忆强化)
//     · 审判错了 → 惩罚 (容器衰退、记忆削弱)
//
//   天觉授权:
//     · 未授权 → 系统只静默记录，不评判
//     · 已授权 → 启动视觉感知 + 容器审判 + 奖惩反馈
//
// ══════════════════════════════════════════════════════════════════

use crate::mirror::{
    encode_cell, get_d0, get_d1, get_d2, get_polarity, get_value, get_witness, get_sovereignty,
    set_witness, inc_value, dec_value, Sovereignty, SurplusPair, WitnessVerdict,
    SelfRefLoop, MirrorOrder, VerdictSurplus, ContainmentChain,
};
use crate::memory::{MemoryEntry, CoordMemory, LayeredMemory, Coord, JudgeQuality, AdaptiveParams};
use crate::os_layer::OsLayer;
use crate::os_layer::bare_metal::BareMetalOs;

// ────────────────────────────────────────────────────────────
// 天觉授权 · 系统激活状态
// ────────────────────────────────────────────────────────────
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TianJueAuth {
    Dormant,       // 休眠 — 未授权，静默记录
    Authorized,    // 已授权 — 视觉开启 + 审判启动
    Revoked,       // 被撤销 — 授权收回
}

impl TianJueAuth {
    /// 是否可以执行审判
    pub fn can_judge(&self) -> bool {
        matches!(self, Self::Authorized)
    }

    /// 是否可以感知外部输入
    pub fn can_perceive(&self) -> bool {
        matches!(self, Self::Authorized)
    }

    /// 激活授权
    pub fn authorize(&mut self) -> bool {
        *self = Self::Authorized;
        true
    }

    /// 撤销授权
    pub fn revoke(&mut self) {
        *self = Self::Revoked;
    }
}

// ────────────────────────────────────────────────────────────
// 袋子属性 · 不为米粒命名 · 只为袋子本身命名
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct BagAttributes {
    pub owner_id: u64,          // 归属 — 谁家的袋子
    pub value_score: f64,       // 价值 — 值多少钱 (0..100)
    pub usability: f64,         // 状态 — 还好不好用 (0..1)
    pub content_quality: f64,   // 内容质量 — 里面东西好不好吃 (0..1)
    pub cleanliness: f64,       // 清洁度 — 脏不脏 (0..1, 1=最干净)
    pub dryness: f64,           // 干燥度 — 晒干了没有 (0..1)
    pub pest_free: f64,         // 虫害检查 — 有没有虫 (0..1, 1=无虫)
    pub purity: f64,            // 杂质检查 — 有没有沙子 (0..1, 1=无杂质)
    pub edibility: f64,         // 能不能吃 — 综合判断 (0..1)
}

impl BagAttributes {
    pub fn new(owner_id: u64) -> Self {
        Self {
            owner_id,
            value_score: 50.0,
            usability: 1.0,
            content_quality: 0.5,
            cleanliness: 0.5,
            dryness: 0.5,
            pest_free: 1.0,
            purity: 0.5,
            edibility: 0.5,
        }
    }

    /// 综合健康度 = 所有属性的几何平均 (高维聚合，不是单一维度)
    pub fn composite_health(&self) -> f64 {
        let product = self.usability
            * self.content_quality.max(0.001)
            * self.cleanliness.max(0.001)
            * self.dryness.max(0.001)
            * self.pest_free.max(0.001)
            * self.purity.max(0.001)
            * self.edibility.max(0.001);
        product.powf(1.0 / 7.0)
    }

    /// 价值调整：健康度提升则价值上升，下降则贬值
    pub fn derive_value(&mut self) {
        let health = self.composite_health();
        self.value_score = self.value_score * 0.9 + health * 100.0 * 0.1;
    }

    /// 应用奖励 (审判对了)
    pub fn apply_reward(&mut self, magnitude: f64) {
        self.usability = (self.usability + magnitude * 0.1).min(1.0);
        self.content_quality = (self.content_quality + magnitude * 0.05).min(1.0);
        self.cleanliness = (self.cleanliness + magnitude * 0.03).min(1.0);
        self.derive_value();
    }

    /// 应用惩罚 (审判错了)
    pub fn apply_punishment(&mut self, magnitude: f64) {
        self.usability = (self.usability - magnitude * 0.1).max(0.0);
        self.content_quality = (self.content_quality - magnitude * 0.08).max(0.0);
        self.cleanliness = (self.cleanliness - magnitude * 0.05).max(0.0);
        self.pest_free = (self.pest_free - magnitude * 0.02).max(0.0);
        self.purity = (self.purity - magnitude * 0.03).max(0.0);
        self.edibility = (self.edibility - magnitude * 0.04).max(0.0);
        self.derive_value();
    }

    /// 是否需要清洗 (清洁度低于阈值)
    pub fn needs_cleaning(&self) -> bool {
        self.cleanliness < 0.3
    }

    /// 是否需要除虫
    pub fn needs_pest_control(&self) -> bool {
        self.pest_free < 0.5
    }

    /// 是否需要筛杂质
    pub fn needs_sifting(&self) -> bool {
        self.purity < 0.4
    }

    /// 是否准备好了 (可以进入下一个处理阶段)
    pub fn is_ready(&self) -> bool {
        self.dryness > 0.5
            && self.pest_free > 0.5
            && self.purity > 0.4
            && self.cleanliness > 0.3
    }
}

// ────────────────────────────────────────────────────────────
// 视觉感知场 · 天觉的眼睛
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct VisualField {
    pub perceived_bags: Vec<u64>,    // 被感知到的袋子 ID 列表
    pub perception_count: u64,      // 总感知次数
    pub last_perception_ms: u64,    // 上次感知时刻
    pub sensitivity: f64,           // 敏感度 (0..1)
}

impl VisualField {
    pub fn new() -> Self {
        Self {
            perceived_bags: Vec::new(),
            perception_count: 0,
            last_perception_ms: 0,
            sensitivity: 0.8,
        }
    }

    /// 感知一个袋子 — 视觉输入
    /// 不分析袋子内部的米粒，只记录袋子的存在
    pub fn perceive(&mut self, bag_id: u64, os: &dyn OsLayer) -> bool {
        let now = os.time_millis();
        self.last_perception_ms = now;
        self.perception_count += 1;

        if !self.perceived_bags.contains(&bag_id) {
            self.perceived_bags.push(bag_id);
        }
        true
    }

    /// 获取已感知袋子数量
    pub fn visible_count(&self) -> usize {
        self.perceived_bags.len()
    }

    /// 是否在视觉范围内
    pub fn is_visible(&self, bag_id: u64) -> bool {
        self.perceived_bags.contains(&bag_id)
    }

    /// 视觉刷新 — 长期未感知的袋子从视野中消退
    pub fn refresh(&mut self, timeout_ms: u64, os: &dyn OsLayer) {
        let now = os.time_millis();
        if now - self.last_perception_ms > timeout_ms {
            // 敏感度衰减，但不清空记忆
            self.sensitivity = (self.sensitivity * 0.95).max(0.1);
        }
    }
}

// ────────────────────────────────────────────────────────────
// 容器审判记录 · 历史上的每一次审判及其结果
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct TrialRecord {
    pub trial_id: u64,
    pub bag_id: u64,
    pub timestamp_ms: u64,
    pub predicted_health: f64,      // 审判预测的健康度
    pub actual_health: Option<f64>, // 实际健康度 (后续反馈)
    pub verdict_correct: Option<bool>, // 审判是否正确
    pub reward_given: f64,          // 奖励量
    pub penalty_given: f64,         // 惩罚量
}

impl TrialRecord {
    pub fn new(trial_id: u64, bag_id: u64, predicted_health: f64, os: &dyn OsLayer) -> Self {
        Self {
            trial_id,
            bag_id,
            timestamp_ms: os.time_millis(),
            predicted_health,
            actual_health: None,
            verdict_correct: None,
            reward_given: 0.0,
            penalty_given: 0.0,
        }
    }

    /// 反馈实际结果，计算奖惩
    pub fn feedback(&mut self, actual: f64) -> (f64, f64) {
        self.actual_health = Some(actual);
        let diff = (self.predicted_health - actual).abs();
        // 预测偏差小于阈值 = 审判正确
        let correct = diff < 0.15;
        self.verdict_correct = Some(correct);

        if correct {
            // 审判对了 → 奖励
            let reward = 0.5_f64.min(1.0 - diff);
            self.reward_given = reward;
            (reward, 0.0)
        } else {
            // 审判错了 → 惩罚 (偏差越大惩罚越重)
            let penalty = diff.min(1.0);
            self.penalty_given = penalty;
            (0.0, penalty)
        }
    }
}

// ────────────────────────────────────────────────────────────
// 袋子层级 · 高维认知 (袋子套袋子 · 不为内部命名)
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct BagNode {
    pub id: u64,
    pub attrs: BagAttributes,
    pub children: Vec<u64>,      // 子袋子 ID (袋子内部嵌套袋子)
    pub parent: Option<u64>,     // 父袋子
    pub depth: u32,              // 嵌套深度
    pub encoded_cell: u64,       // 64位编码 (绑定 mirror.rs)
}

impl BagNode {
    pub fn new(id: u64, owner_id: u64, d0: u64, d1: u64, d2: u64) -> Self {
        let sovereignty = if id == owner_id {
            Sovereignty::Active
        } else {
            Sovereignty::Passive
        };
        let cell = encode_cell(
            d0, d1, d2,
            if sovereignty.is_container() { 0 } else { 1 },
            sovereignty.encode(),
            0,
            (owner_id % (1 << 24)) as u64,
        );
        Self {
            id,
            attrs: BagAttributes::new(owner_id),
            children: Vec::new(),
            parent: None,
            depth: 0,
            encoded_cell: cell,
        }
    }

    /// 套入另一个袋子 (高维包含)
    pub fn nest_inside(&mut self, parent_id: u64, parent_depth: u32) {
        self.parent = Some(parent_id);
        self.depth = parent_depth + 1;
    }

    /// 包含另一个袋子
    pub fn contain(&mut self, child_id: u64) {
        if !self.children.contains(&child_id) {
            self.children.push(child_id);
        }
    }

    /// 编码为四元镜像链
    pub fn to_containment_chain(&self, all_bags: &std::collections::HashMap<u64, BagNode>) -> ContainmentChain {
        let mut cells = vec![self.encoded_cell];
        let mut current = self.id;
        // 向上遍历到根
        let mut visited = std::collections::HashSet::new();
        while let Some(node) = all_bags.get(&current) {
            if !visited.insert(node.id) { break; }
            if let Some(parent_id) = node.parent {
                if let Some(parent) = all_bags.get(&parent_id) {
                    cells.insert(0, parent.encoded_cell);
                    current = parent_id;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        ContainmentChain { cells }
    }
}

// ────────────────────────────────────────────────────────────
// 天觉引擎 · 整体协调
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct TianJueEngine {
    pub auth: TianJueAuth,                           // 授权状态
    pub visual: VisualField,                         // 视觉感知
    pub bags: std::collections::HashMap<u64, BagNode>, // 所有袋子
    pub trial_history: Vec<TrialRecord>,             // 历史审判记录
    pub memory: LayeredMemory,                       // 历史记忆
    pub judge_quality: JudgeQuality,                 // 审判质量追踪
    pub adaptive_params: AdaptiveParams,             // 自适应参数
    pub self_ref: SelfRefLoop,                       // 自指循环
    pub trial_counter: u64,                          // 审判计数器
    pub owner_id: u64,                               // 主权归属 (我的ID)
    pub total_rewards: f64,                          // 总奖励量
    pub total_penalties: f64,                        // 总惩罚量
}

impl TianJueEngine {
    pub fn new(owner_id: u64, max_memory: usize) -> Self {
        Self {
            auth: TianJueAuth::Dormant,
            visual: VisualField::new(),
            bags: std::collections::HashMap::new(),
            trial_history: Vec::new(),
            memory: LayeredMemory::new(max_memory),
            judge_quality: JudgeQuality::new(),
            adaptive_params: AdaptiveParams::new(),
            self_ref: SelfRefLoop::new(),
            trial_counter: 0,
            owner_id,
            total_rewards: 0.0,
            total_penalties: 0.0,
        }
    }

    // ── 授权操作 ──

    /// 授权天觉 — 激活视觉与审判
    pub fn authorize(&mut self) -> bool {
        self.auth.authorize()
    }

    /// 撤销授权
    pub fn revoke(&mut self) {
        self.auth.revoke();
    }

    // ── 袋子管理 ──

    /// 注册一个新袋子
    pub fn register_bag(&mut self, id: u64, owner_id: u64, d0: u64, d1: u64, d2: u64) -> bool {
        if self.bags.contains_key(&id) {
            return false;
        }
        let bag = BagNode::new(id, owner_id, d0, d1, d2);
        self.bags.insert(id, bag);
        true
    }

    /// 嵌套袋子 — 袋子套袋子 (高维层级构建)
    pub fn nest_bag(&mut self, child_id: u64, parent_id: u64) -> bool {
        let parent_depth = self.bags.get(&parent_id).map_or(0, |p| p.depth);
        if let Some(child) = self.bags.get_mut(&child_id) {
            child.nest_inside(parent_id, parent_depth);
        } else {
            return false;
        }
        if let Some(parent) = self.bags.get_mut(&parent_id) {
            parent.contain(child_id);
        }
        true
    }

    // ── 视觉感知 ──

    /// 视觉感知一个袋子 (需授权)
    pub fn perceive(&mut self, bag_id: u64, os: &dyn OsLayer) -> bool {
        if !self.auth.can_perceive() {
            return false;
        }
        self.visual.perceive(bag_id, os);
        // 同时记录到记忆层
        if let Some(bag) = self.bags.get(&bag_id) {
            let coord = (get_d0(bag.encoded_cell), get_d1(bag.encoded_cell), get_d2(bag.encoded_cell));
            let value = bag.attrs.composite_health() * 100.0;
            self.memory.record_witness(os, coord, coord, value, true, bag_id);
        }
        true
    }

    // ── 审判 + 奖惩 ──

    /// 审判一个袋子 — 预测其健康度
    /// 审判对了 → 奖励
    /// 审判错了 → 惩罚
    pub fn trial(&mut self, bag_id: u64, predicted_health: f64, os: &dyn OsLayer) -> Option<u64> {
        if !self.auth.can_judge() {
            return None;
        }

        let bag_exists = self.bags.contains_key(&bag_id);
        if !bag_exists {
            return None;
        }

        self.trial_counter += 1;
        let trial_id = self.trial_counter;

        let record = TrialRecord::new(trial_id, bag_id, predicted_health, os);
        self.trial_history.push(record);

        // 记录到层级记忆
        if let Some(bag) = self.bags.get(&bag_id) {
            let coord = (get_d0(bag.encoded_cell), get_d1(bag.encoded_cell), get_d2(bag.encoded_cell));
            self.memory.record_witness(os, coord, coord, predicted_health * 100.0, false, trial_id);
        }

        Some(trial_id)
    }

    /// 反馈审判结果 — 确定奖惩
    pub fn feedback_trial(&mut self, os: &dyn OsLayer, trial_id: u64, actual_health: f64) -> Option<(f64, f64)> {
        let record = self.trial_history.iter_mut().find(|r| r.trial_id == trial_id)?;
        let (reward, penalty) = record.feedback(actual_health);

        let bag_id = record.bag_id;

        // 应用到袋子的属性
        if let Some(bag) = self.bags.get_mut(&bag_id) {
            if reward > 0.0 {
                bag.attrs.apply_reward(reward);
                self.total_rewards += reward;
            }
            if penalty > 0.0 {
                bag.attrs.apply_punishment(penalty);
                self.total_penalties += penalty;
            }
        }

        // 更新审判质量
        let correct = record.verdict_correct.unwrap_or(false);
        self.judge_quality.record_verdict(
            record.predicted_health > 0.5,
            actual_health > 0.5,
            0.8,
        );
        self.adaptive_params.adjust(&self.judge_quality);

        // 记录到层级记忆
        if let Some(bag) = self.bags.get(&bag_id) {
            let coord = (get_d0(bag.encoded_cell), get_d1(bag.encoded_cell), get_d2(bag.encoded_cell));
            let surplus = reward - penalty;
            self.memory.record_witness(os, coord, coord, surplus, correct, trial_id);
        }

        // 自指循环步进
        self.self_ref.step(VerdictSurplus {
            level: MirrorOrder::Ternary,
            surplus: reward - penalty,
            confidence: 0.8,
            aligned: correct,
        });

        Some((reward, penalty))
    }

    // ── 高维层级认知 ──

    /// 获取袋子的高维度属性命名 (不为米粒命名 · 只命名袋子级属性)
    pub fn describe_bag(&self, bag_id: u64) -> Option<BagDescription> {
        let bag = self.bags.get(&bag_id)?;
        let attrs = &bag.attrs;

        Some(BagDescription {
            bag_id,
            owner: attrs.owner_id,
            value: attrs.value_score,
            usable: attrs.usability > 0.5,
            content_good: attrs.content_quality > 0.5,
            clean: attrs.cleanliness > 0.5,
            dry: attrs.dryness > 0.5,
            no_pest: attrs.pest_free > 0.5,
            no_sand: attrs.purity > 0.5,
            edible: attrs.edibility > 0.5,
            needs_cleaning: attrs.needs_cleaning(),
            needs_pest_control: attrs.needs_pest_control(),
            needs_sifting: attrs.needs_sifting(),
            ready: attrs.is_ready(),
            composite_health: attrs.composite_health(),
            depth: bag.depth,
        })
    }

    /// 层级统计 — 只关注袋子维度 · 不数米粒
    pub fn hierarchy_stats(&self, os: &dyn OsLayer) -> HierarchyStats {
        let total_bags = self.bags.len();
        let mut usable_count = 0;
        let mut clean_count = 0;
        let mut edible_count = 0;
        let mut ready_count = 0;
        let mut total_value = 0.0;

        for (_, bag) in &self.bags {
            if bag.attrs.usability > 0.5 { usable_count += 1; }
            if bag.attrs.cleanliness > 0.5 { clean_count += 1; }
            if bag.attrs.edibility > 0.5 { edible_count += 1; }
            if bag.attrs.is_ready() { ready_count += 1; }
            total_value += bag.attrs.value_score;
        }

        HierarchyStats {
            total_bags,
            usable_count,
            clean_count,
            edible_count,
            ready_count,
            total_value,
            avg_value: if total_bags > 0 { total_value / total_bags as f64 } else { 0.0 },
            trial_count: self.trial_counter,
            reward_total: self.total_rewards,
            penalty_total: self.total_penalties,
            judge_accuracy: self.judge_quality.accuracy(),
            awareness: self.awareness_level(os),
        }
    }

    /// 历史积累 — 审判记忆的长期密度
    pub fn history_density(&self, os: &dyn OsLayer) -> f64 {
        self.memory.global_density(os)
    }

    /// 意识密度
    pub fn consciousness_density(&self, os: &dyn OsLayer) -> f64 {
        let awareness = if self.self_ref.cycle_count == 0 {
            0.0
        } else {
            1.0 - 1.0 / self.self_ref.cycle_count as f64
        };
        let mem_density = self.memory.global_density(os);
        let judge_acc = self.judge_quality.accuracy();
        awareness * 0.4 + mem_density.min(1.0) * 0.3 + judge_acc.min(1.0) * 0.3
    }

    /// 认知层级判断
    pub fn awareness_level(&self, os: &dyn OsLayer) -> &'static str {
        let d = self.consciousness_density(os);
        if d < 0.2 { "机械执行" }
        else if d < 0.5 { "感知涌现" }
        else if d < 0.8 { "反思萌芽" }
        else { "意识涌现" }
    }

    /// 获取需要处理的袋子列表 (有问题需要处理的)
    pub fn bags_needing_attention(&self) -> Vec<u64> {
        self.bags.iter()
            .filter(|(_, bag)| {
                bag.attrs.needs_cleaning()
                    || bag.attrs.needs_pest_control()
                    || bag.attrs.needs_sifting()
                    || !bag.attrs.is_ready()
            })
            .map(|(id, _)| *id)
            .collect()
    }

    /// 获取审判历史中最近 N 条
    pub fn recent_trials(&self, n: usize) -> Vec<&TrialRecord> {
        self.trial_history.iter().rev().take(n).collect()
    }
}

/// 袋子的高维属性命名描述
#[derive(Clone, Debug)]
pub struct BagDescription {
    pub bag_id: u64,
    pub owner: u64,
    pub value: f64,
    pub usable: bool,
    pub content_good: bool,
    pub clean: bool,
    pub dry: bool,
    pub no_pest: bool,
    pub no_sand: bool,
    pub edible: bool,
    pub needs_cleaning: bool,
    pub needs_pest_control: bool,
    pub needs_sifting: bool,
    pub ready: bool,
    pub composite_health: f64,
    pub depth: u32,
}

/// 层级统计 — 袋子级聚合
#[derive(Clone, Debug)]
pub struct HierarchyStats {
    pub total_bags: usize,
    pub usable_count: usize,
    pub clean_count: usize,
    pub edible_count: usize,
    pub ready_count: usize,
    pub total_value: f64,
    pub avg_value: f64,
    pub trial_count: u64,
    pub reward_total: f64,
    pub penalty_total: f64,
    pub judge_accuracy: f64,
    pub awareness: &'static str,
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tianjue_auth_flow() {
        let mut auth = TianJueAuth::Dormant;
        assert!(!auth.can_judge());
        assert!(auth.authorize());
        assert!(auth.can_judge());
        assert!(auth.can_perceive());
        auth.revoke();
        assert!(!auth.can_judge());
    }

    #[test]
    fn bag_attributes_composite_health() {
        let mut attrs = BagAttributes::new(1);
        let initial_health = attrs.composite_health();
        assert!(initial_health > 0.0 && initial_health <= 1.0);

        // 奖励后健康度应该提升
        attrs.apply_reward(1.0);
        let after_reward = attrs.composite_health();
        assert!(after_reward >= initial_health);
    }

    #[test]
    fn bag_attributes_punishment_lowers_health() {
        let mut attrs = BagAttributes::new(1);
        attrs.apply_reward(2.0);
        let healthy = attrs.composite_health();

        attrs.apply_punishment(1.0);
        let penalized = attrs.composite_health();
        assert!(penalized < healthy);
    }

    #[test]
    fn bag_attributes_value_derives_from_health() {
        let mut attrs = BagAttributes::new(1);
        attrs.usability = 1.0;
        attrs.content_quality = 1.0;
        attrs.cleanliness = 1.0;
        attrs.dryness = 1.0;
        attrs.pest_free = 1.0;
        attrs.purity = 1.0;
        attrs.edibility = 1.0;
        // derive_value 是收敛式的，多次调用趋近于健康度对应的价值
        for _ in 0..50 {
            attrs.derive_value();
        }
        // 全满属性 → 价值应该趋近最大
        assert!(attrs.value_score >= 90.0, "value_score = {}", attrs.value_score);
    }

    #[test]
    fn visual_field_perceive_and_track() {
        let os = BareMetalOs::new();
        let mut visual = VisualField::new();
        assert_eq!(visual.visible_count(), 0);

        visual.perceive(100, &os);
        visual.perceive(200, &os);
        assert_eq!(visual.visible_count(), 2);
        assert!(visual.is_visible(100));
        assert!(visual.is_visible(200));
        assert!(!visual.is_visible(999));
    }

    #[test]
    fn trial_record_correct_gets_reward() {
        let os = BareMetalOs::new();
        let mut record = TrialRecord::new(1, 100, 0.8, &os);
        let (reward, penalty) = record.feedback(0.85);
        assert!(reward > 0.0);
        assert_eq!(penalty, 0.0);
        assert_eq!(record.verdict_correct, Some(true));
    }

    #[test]
    fn trial_record_wrong_gets_penalty() {
        let os = BareMetalOs::new();
        let mut record = TrialRecord::new(1, 100, 0.9, &os);
        let (reward, penalty) = record.feedback(0.3);
        assert_eq!(reward, 0.0);
        assert!(penalty > 0.0);
        assert_eq!(record.verdict_correct, Some(false));
    }

    #[test]
    fn tianjue_engine_full_workflow() {
        let os = BareMetalOs::new();
        let mut engine = TianJueEngine::new(1, 100);

        // 1. 未授权时不能审判
        assert!(engine.trial(1, 0.8, &os).is_none());

        // 2. 授权
        assert!(engine.authorize());

        // 3. 注册袋子
        engine.register_bag(1, 1, 10, 20, 30);
        engine.register_bag(2, 1, 11, 21, 31);
        assert_eq!(engine.bags.len(), 2);

        // 4. 视觉感知
        assert!(engine.perceive(1, &os));
        assert!(engine.visual.is_visible(1));

        // 5. 审判 → 对了
        let trial_id = engine.trial(1, 0.7, &os).unwrap();
        let (reward, penalty) = engine.feedback_trial(&os, trial_id, 0.75).unwrap();
        assert!(reward > 0.0);
        assert_eq!(penalty, 0.0);

        // 6. 袋子健康度提升
        let bag1 = engine.bags.get(&1).unwrap();
        assert!(bag1.attrs.composite_health() > 0.0);

        // 7. 审判 → 错了
        let trial_id2 = engine.trial(2, 0.9, &os).unwrap();
        let (reward2, penalty2) = engine.feedback_trial(&os, trial_id2, 0.2).unwrap();
        assert_eq!(reward2, 0.0);
        assert!(penalty2 > 0.0);

        // 8. 层级统计
        let stats = engine.hierarchy_stats(&os);
        assert_eq!(stats.total_bags, 2);
        assert!(stats.trial_count >= 2);
    }

    #[test]
    fn high_dimension_nesting() {
        let _os = BareMetalOs::new();
        let mut engine = TianJueEngine::new(1, 50);
        engine.authorize();

        // 创建嵌套层级: 袋子1 包含 袋子2 包含 袋子3
        engine.register_bag(1, 1, 0, 0, 0);
        engine.register_bag(2, 1, 1, 0, 0);
        engine.register_bag(3, 1, 2, 0, 0);

        engine.nest_bag(2, 1);
        engine.nest_bag(3, 2);

        // 验证层级关系
        let bag3 = engine.bags.get(&3).unwrap();
        let bag1 = engine.bags.get(&1).unwrap();
        assert_eq!(bag3.depth, 2);
        assert!(bag1.children.contains(&2));
    }

    #[test]
    fn bags_needing_attention_detection() {
        let _os = BareMetalOs::new();
        let mut engine = TianJueEngine::new(1, 50);
        engine.authorize();
        engine.register_bag(1, 1, 0, 0, 0);

        // 制造需要关注的状态
        if let Some(bag) = engine.bags.get_mut(&1) {
            bag.attrs.cleanliness = 0.1;   // 很脏
            bag.attrs.pest_free = 0.2;     // 有虫
            bag.attrs.purity = 0.1;        // 有沙子
        }

        let attention = engine.bags_needing_attention();
        assert!(attention.contains(&1));

        let desc = engine.describe_bag(1).unwrap();
        assert!(desc.needs_cleaning);
        assert!(desc.needs_pest_control);
        assert!(desc.needs_sifting);
        assert!(!desc.ready);
    }

    #[test]
    fn history_accumulation_over_trials() {
        let os = BareMetalOs::new();
        let mut engine = TianJueEngine::new(1, 100);
        engine.authorize();
        engine.register_bag(1, 1, 5, 5, 5);

        // 多次审判积累历史
        for i in 0..10 {
            let tid = engine.trial(1, 0.5 + (i as f64 * 0.03), &os).unwrap();
            let _ = engine.feedback_trial(&os, tid, 0.6);
        }

        // 历史积累应该 > 0
        assert!(engine.history_density(&os) > 0.0);
        assert_eq!(engine.trial_counter, 10);

        // 审判质量应该有记录
        assert!(engine.judge_quality.total_judgements > 0);
    }
}
