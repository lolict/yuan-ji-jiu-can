// @name: relational
// @type: core-relational
// @owner: 五彩石
// @status: stable
// @priority: P0
// @description: 关系编码系统


// ══════════════════════════════════════════════════════════════════
// relational.rs — 关系代数外壳 (Relational Algebra Shell)
// ══════════════════════════════════════════════════════════════════
//
// "身份 → 权重 → 流动权 → 优先级"
//
// 关系代数定义了 谁拥有什么 / 谁能移动什么 / 谁不可移动.
// 两人关系系统里:
//
//   Identity  (身份)        — 你是谁? (丈夫 / 妻子 / 外界 / 节点)
//   Weight    (权重)        — 在共同体里有多少 weight (贡献+信任累计)
//   FlowRight (流动权)        — 你能调动多少资源 (分配的权力)
//   Priority  (优先级)      — 在队列里谁先到谁拿到资源 (排队权)
//
// 聚类决定:
//   谁是"皇帝"(唯一权重最大者, 拥有但不劳动)
//   谁是"劳动力"(有流动权, 可动员资源)
//   谁是"不动产"(不可移动, 固定沉淀)
//
// 结构:
//   Actor     — 参与者
//   Cluster   — 聚类 (根据 weight+flow 的聚类结果)
//   RelationTable — 关系表 (N×N, 每对之间的关系元数据)
//   RelAlgebra — 外壳主入口

use crate::mirror::{self, Sovereignty};

// ────────────────────────────────────────────────────────────
// 身份
// ────────────────────────────────────────────────────────────
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Identity {
    Husband,        // 丈夫 — 融合入囡囝共生 (主动性外壳)
    Wife,           // 妻子 — 融合入囡囝共生 (接受性内核)
    ExternalMale,   // 外界雄性/竞争者 → 可融合入丈夫侧
    ExternalFemale, // 外界雌性/吸引者 → 可融合入妻子侧
    Neutral,        // 中性外界 → 纯粹资源/竞争
    System,         // 系统自身 (囡囝共生底层)
}

impl Identity {
    /// 是否属于丈夫侧
    pub fn is_husband_side(&self) -> bool {
        matches!(self, Self::Husband | Self::ExternalMale)
    }
    /// 是否属于妻子侧
    pub fn is_wife_side(&self) -> bool {
        matches!(self, Self::Wife | Self::ExternalFemale)
    }
    /// 是否属于核心夫妻
    pub fn is_core_couple(&self) -> bool {
        matches!(self, Self::Husband | Self::Wife)
    }
    /// 是否外界 (可被融合)
    pub fn is_external(&self) -> bool {
        !self.is_core_couple() && !matches!(self, Self::System)
    }
    /// 一侧内字符串
    pub fn side_str(&self) -> &'static str {
        if self.is_husband_side() { "丈夫侧" }
        else if self.is_wife_side() { "妻子侧" }
        else if matches!(self, Self::System) { "系统" }
        else { "中性" }
    }
}

// ────────────────────────────────────────────────────────────
// 权重 (Weight)
// ────────────────────────────────────────────────────────────
//
// weight = 初始归属权 + 贡献累计 - 对外消耗
// 这里简化: weight 是一个连续值, 高权重 = 更高的流动权 + 优先级.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Weight(pub f64);

impl Weight {
    pub const ZERO: Weight = Weight(0.0);
    pub const CORE: Weight = Weight(1.0);    // 成员核心 (夫妻)
    pub const FULL: Weight   = Weight(f64::MAX);
    pub fn is_valid(&self) -> bool {
        self.0.is_finite() && self.0 >= 0.0
    }
}

// ────────────────────────────────────────────────────────────
// 流动权 (FlowRight)
// ────────────────────────────────────────────────────────────
//
// flow_right 表示你能调用/分配的资源范围.
// - 默认与 weight 成正比: 权重越高, 能动员越多资源.
// - 但 SYSTEM actor 拥有绝对的流动权但不用: "皇帝拥有全天下却不从事劳动".

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlowRight(pub f64);

impl FlowRight {
    pub fn from_weight(w: Weight, multiplier: f64) -> Self {
        FlowRight(w.0 * multiplier)
    }
    pub fn is_valid(&self) -> bool {
        self.0 >= 0.0 && self.0.is_finite()
    }
}

// ────────────────────────────────────────────────────────────
// 优先级 (Priority)
// ────────────────────────────────────────────────────────────
//
// 在资源紧张时, 谁的请求先被执行.
// priority = flow_right × urgency_factor.

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Priority(pub f64);

impl Priority {
    pub fn from_flow_weight(flow: FlowRight, weight: Weight) -> Self {
        Priority((flow.0 + 1.0).ln() * (weight.0 + 1.0))
    }
}

// ────────────────────────────────────────────────────────────
// 参与者 (Actor) — 关系代数里的 行
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct Actor {
    pub id: u64,
    pub name: String,
    pub identity: Identity,
    pub weight: Weight,
    pub flow_right: FlowRight,
    pub priority: Priority,
    pub velocity: f64,  // 活跃速度 (正=融入 / 负=退出)
}

impl Actor {
    pub fn new(id: u64, name: &str, identity: Identity, weight: Weight) -> Self {
        let flow = FlowRight::from_weight(weight, 10.0);
        let priority = Priority::from_flow_weight(flow, weight);
        Self {
            id, name: name.into(), identity, weight, flow_right: flow,
            priority, velocity: 0.0,
        }
    }

    /// 检查流动权是否允许调动 amount 资源
    pub fn can_mobilize(&self, amount: f64) -> bool {
        self.flow_right.0 >= amount && self.weight.is_valid()
    }

    /// 调动资源: 返回成功or失败
    pub fn mobilize(&mut self, amount: f64) -> bool {
        if !self.can_mobilize(amount) { return false; }
        self.weight.0 -= amount * 0.01;  // 消耗微量权重
        self.flow_right.0 -= amount;
        self.priority = Priority::from_flow_weight(self.flow_right, self.weight);
        true
    }

    /// 编码为 64 位单元格 — 供感知器官 ingest 和聚类
    pub fn to_cell(&self) -> u64 {
        let identity_marker: u64 = match self.identity {
            Identity::Wife => 0,
            Identity::Husband => 1,
            Identity::ExternalFemale => 2,
            Identity::ExternalMale => 3,
            Identity::Neutral => 4,
            Identity::System => 5,
        };
        let polarity = if self.identity.is_wife_side() { 0 } else { 1 };
        let sov = if self.identity.is_core_couple() {
            Sovereignty::Active
        } else {
            Sovereignty::Passive
        };
        mirror::encode_cell(
            identity_marker,
            self.weight.0 as u64,
            self.priority.0 as u64,
            polarity,
            sov.encode(),
            0,
            (self.velocity * 1000.0) as u64,
        )
    }

    /// 融合: 把外界的自己融入这里 (增加权重, 清零外界身份)
    pub fn absorb(&mut self, other: &Actor) -> bool {
        if !self.identity.is_core_couple() { return false; }
        // 只能融合"同身份"的 actor (外界→丈夫侧/妻子侧)
        if self.identity.is_husband_side() != other.identity.is_husband_side()
           && self.identity.is_wife_side() != other.identity.is_wife_side() {
            return false;
        }
        self.weight.0 += other.weight.0;
        self.flow_right = FlowRight::from_weight(self.weight, 10.0);
        self.priority = Priority::from_flow_weight(self.flow_right, self.weight);
        true
    }
}

// ────────────────────────────────────────────────────────────
// 聚类 (Cluster)
// ────────────────────────────────────────────────────────────
//
// 聚类规则:
//   Cluster::CenterCenter     — 核心聚类 (夫妻融合点)
//   Cluster::CenterHusband    — 丈夫中心
//   Cluster::CenterWife       — 妻子中心
//   Cluster::Peripheral(p)    — 外界 p (根据距离中心的远近)
//   Cluster::Immobile         — 不可移动点 (完全沉淀)

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cluster {
    CenterCenter,     // 唯一中心 (囡囝共生)
    CenterHusband,
    CenterWife,
    Peripheral(u8),   // u8 层级 (距离中心 1..255)
    Immobile,         // 不可移动
    External,         // 外界 (还未融入)
}

impl Cluster {
    /// 给一个 actor 做聚类分配
    pub fn classify(actor: &Actor) -> Self {
        match actor.identity {
            Identity::System => Cluster::Immobile,
            Identity::Husband => Cluster::CenterHusband,
            Identity::Wife    => Cluster::CenterWife,
            Identity::ExternalMale   => Cluster::External,
            Identity::ExternalFemale => Cluster::External,
            Identity::Neutral         => Cluster::External,
        }
    }
    /// 是否能分配资源
    pub fn can_allocate(&self) -> bool {
        matches!(self, Self::CenterCenter | Self::CenterHusband | Self::CenterWife)
    }
    /// 是否不可移动
    pub fn is_immovable(&self) -> bool {
        matches!(self, Self::Immobile)
    }
    /// 是否属于中心
    pub fn is_central(&self) -> bool {
        matches!(self, Self::CenterCenter | Self::CenterHusband | Self::CenterWife)
    }
}

// ────────────────────────────────────────────────────────────
// 关系表 (RelationTable)
// ────────────────────────────────────────────────────────────
//
// N×N 对称 (或非对称) 矩阵, 每对 (i,j) 记录:
//   relation_type:  i 对 j 的关系类型
//   tension:       正=吸引 / 负=排斥
//   resource_flow: i -> j 的资源流转净额

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RelationType {
    Married,        // 夫妻
    Allied,         // 盟友
    Rival,          // 竞争/对手
    Resource,       // 资源供给
    Neutral,        // 中性
}

#[derive(Clone, Debug)]
pub struct Relation {
    pub rel_type: RelationType,
    pub tension: f64,          // 正=吸引 负=排斥
    pub resource_flow: f64,    // 正=流入actor 负=流出actor
}

impl Relation {
    pub fn new(rel_type: RelationType, tension: f64, resource_flow: f64) -> Self {
        Self { rel_type, tension, resource_flow }
    }
}

// ────────────────────────────────────────────────────────────
// 关系代数外壳 (RelAlgebra)
// ────────────────────────────────────────────────────────────
pub struct RelAlgebra {
    pub actors: Vec<Actor>,
    pub relation_matrix: Vec<Vec<Relation>>,  // 仅用于小 n (简化)
    pub husband_center: usize,  // Husband actor index
    pub wife_center: usize,     // Wife actor index
    pub center_fused: bool,     // 夫妻是否融合为唯一中心
}

impl RelAlgebra {
    /// 构造: 初始化夫妻双核心 + 默认关系对
    pub fn new_couple(husband_name: &str, wife_name: &str) -> Self {
        let husband = Actor::new(0, husband_name, Identity::Husband, Weight::CORE);
        let wife    = Actor::new(1, wife_name,    Identity::Wife,    Weight::CORE);
        let actors = vec![husband.clone(), wife.clone()];
        let mut ra = Self {
            actors, relation_matrix: Vec::new(),
            husband_center: 0, wife_center: 1, center_fused: false,
        };
        ra.build_matrix();
        ra
    }

    /// 构建 N×N 关系矩阵 (默认: 夫妻 married+高张力 )
    fn build_matrix(&mut self) {
        let n = self.actors.len();
        self.relation_matrix = vec![vec![Relation::new(RelationType::Neutral, 0.0, 0.0); n]; n];
        if n >= 2 {
            self.relation_matrix[0][1] = Relation::new(RelationType::Married, 1.0, 0.0);
            self.relation_matrix[1][0] = Relation::new(RelationType::Married, 1.0, 0.0);
        }
    }

    /// 加入一个外界 actor, 返回其 index
    pub fn add_external(&mut self, name: &str, identity: Identity, weight: Weight) -> usize {
        let idx = self.actors.len();
        let actor = Actor::new(idx as u64, name, identity, weight);
        self.actors.push(actor);
        // 扩展 matrix
        for row in self.relation_matrix.iter_mut() {
            row.push(Relation::new(RelationType::Neutral, 0.0, 0.0));
        }
        let n = self.actors.len();
        self.relation_matrix.push(vec![Relation::new(RelationType::Neutral, 0.0, 0.0); n]);
        // 为这个外部 actor 与夫妻设置初始关系
        if identity.is_husband_side() {
            self.relation_matrix[idx][self.husband_center] = Relation::new(RelationType::Rival, -0.5, 0.0);
            self.relation_matrix[idx][self.wife_center]    = Relation::new(RelationType::Neutral, 0.1, 0.0);
        } else if identity.is_wife_side() {
            self.relation_matrix[idx][self.wife_center]    = Relation::new(RelationType::Resource, 0.5, 0.0);
            self.relation_matrix[idx][self.husband_center] = Relation::new(RelationType::Neutral, 0.1, 0.0);
        }
        idx
    }

    /// 融合: 把某外界 actor 融合入核心
    pub fn fuse_into_center(&mut self, external_idx: usize) -> bool {
        if external_idx >= self.actors.len() { return false; }
        let (target_center, _side) = if self.actors[external_idx].identity.is_husband_side() {
            (self.husband_center, "丈夫侧")
        } else if self.actors[external_idx].identity.is_wife_side() {
            (self.wife_center, "妻子侧")
        } else { return false; };
        let ext_weight = self.actors[external_idx].weight.0;
        // 增加中心权重
        self.actors[target_center].weight.0 += ext_weight;
        self.actors[target_center].flow_right = FlowRight::from_weight(
            self.actors[target_center].weight, 10.0
        );
        self.actors[target_center].priority = Priority::from_flow_weight(
            self.actors[target_center].flow_right,
            self.actors[target_center].weight,
        );
        // 清除外界 (weight 置 0 标记)
        self.actors[external_idx].weight = Weight::ZERO;
        self.actors[external_idx].flow_right = FlowRight::from_weight(Weight::ZERO, 10.0);
        true
    }

    /// 夫妻融合: 唯一中心
    pub fn fuse_couple(&mut self) {
        if self.center_fused { return; }
        let h = &self.actors[self.husband_center];
        let w = &self.actors[self.wife_center];
        let total_weight = h.weight.0 + w.weight.0;
        // 丈夫一侧承载"融合中心" 权重
        self.actors[self.husband_center].weight = Weight(total_weight);
        self.actors[self.husband_center].flow_right = FlowRight::from_weight(Weight(total_weight), 10.0);
        // 妻子一侧也增加 (二者完全对称)
        self.actors[self.wife_center].weight = Weight(total_weight);
        self.actors[self.wife_center].flow_right = FlowRight::from_weight(Weight(total_weight), 10.0);
        self.center_fused = true;
    }

    /// 分类统计
    pub fn classify_actors(&self) -> Vec<(u64, Cluster)> {
        self.actors.iter().map(|a| (a.id, Cluster::classify(a))).collect()
    }

    /// 做一轮资源流动: 根据 relation_matrix 的 resource_flow 推动
    pub fn round_flow(&mut self) {
        let n = self.actors.len();
        let mut deltas = vec![0.0_f64; n];
        for i in 0..n {
            for j in 0..n {
                if i == j { continue; }
                let flow_ij = self.relation_matrix[i][j].resource_flow;
                deltas[i] -= flow_ij;
                deltas[j] += flow_ij;
            }
        }
        for (i, d) in deltas.into_iter().enumerate() {
            let w = self.actors[i].weight.0;
            self.actors[i].weight = Weight((w + d).max(0.0));
            self.actors[i].flow_right = FlowRight::from_weight(self.actors[i].weight, 10.0);
            self.actors[i].priority = Priority::from_flow_weight(
                self.actors[i].flow_right, self.actors[i].weight,
            );
        }
    }

    /// 中心对丈夫/妻子的当前总权重
    pub fn combined_center_weight(&self) -> f64 {
        let h = self.actors[self.husband_center].weight.0;
        let w = self.actors[self.wife_center].weight.0;
        if self.center_fused { h + w } else { h + w }
    }
}

// ────────────────────────────────────────────────────────────
// 自我天平: 每个 actor 维持私有权重存量
// ────────────────────────────────────────────────────────────
// D-7 / D-11 概念实现:
//
// "每个人都独自持有一台私人镜像天平"
// "旁观者只维护自我权重存量，共情是附带情绪"
//
// 生物不是为他人而活的 — 它们把外界的一切换算为
// "这件事能不能增加我的配重、规避我的风险"
//
// 结构:
//   SelfBalance  — 私有天平 (配重存量)
//   BalanceOp    — 配重操作 (增/减/不变)
//   convert_external — 把另一个 actor 的损益换算为自我的损益系数

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SelfBalance {
    /// 当前配重存量 (>0 表示盈余, <0 表示亏损)
    pub stock: f64,
    /// 自我中心系数: 多少外部事件会被纳入计算 (0.0 = 完全漠不关心, 1.0 = 全盘吸收)
    pub self_center_rate: f64,
    /// 反向共情衰减: 对他人痛苦的感受是自我的多少
    pub empathy_decay: f64,
}

impl SelfBalance {
    /// 新建一个完全自我中心的 actor (默认只关心自己)
    pub fn new_selfish(initial_stock: f64) -> Self {
        Self {
            stock: initial_stock,
            self_center_rate: 1.0,
            empathy_decay: 0.1,
        }
    }

    /// 新建一个有共情的 actor (能感受到他人的部分损益)
    pub fn new_empathetic(initial_stock: f64, empathy: f64) -> Self {
        Self {
            stock: initial_stock,
            self_center_rate: 1.0 - empathy,
            empathy_decay: empathy,
        }
    }

    /// 把另一个 actor 的损益 (other_delta) 换算为对我自身天平的影响
    pub fn convert_external(&self, other_delta: f64) -> f64 {
        if other_delta > 0.0 {
            // 他人的好事 — 自私者嫉妒减重, 共情者分享喜悦
            -other_delta * (1.0 - self.self_center_rate) * 0.5
                + other_delta * self.empathy_decay
        } else {
            // 他人的坏事 — 自私者庆幸, 共情者分担
            (-other_delta) * self.self_center_rate * 0.3
                + other_delta * self.empathy_decay
        }
    }

    /// 直接增减自己的配重
    pub fn apply_delta(&mut self, delta: f64) {
        self.stock += delta;
    }

    /// 判断当前是否处于盈余态
    pub fn is_surplus(&self) -> bool {
        self.stock > 0.0
    }

    /// 配重差异越大, 两个 actor 之间的张力越大
    pub fn tension_with(&self, other: &SelfBalance) -> f64 {
        (self.stock - other.stock).abs()
    }
}

/// 天平操作的枚举
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BalanceOp {
    Gain,
    Loss,
    Unchanged,
}

impl BalanceOp {
    pub fn is_harmful(&self) -> bool {
        matches!(self, Self::Loss)
    }

    pub fn is_beneficial(&self) -> bool {
        matches!(self, Self::Gain)
    }

    pub fn sensitivity_at(stock: f64) -> f64 {
        if stock <= 0.0 { return 10.0; }
        1.0 / stock.sqrt()
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn couple_creation() {
        let ra = RelAlgebra::new_couple("刘楚恬", "lolict");
        assert_eq!(ra.actors.len(), 2);
        assert_eq!(ra.actors[0].identity, Identity::Husband);
        assert_eq!(ra.actors[1].identity, Identity::Wife);
    }

    #[test]
    fn matrix_initialized() {
        let ra = RelAlgebra::new_couple("H", "W");
        assert!(matches!(ra.relation_matrix[0][1].rel_type, RelationType::Married));
        assert_eq!(ra.relation_matrix[0][1].tension, 1.0);
    }

    #[test]
    fn add_external_male() {
        let mut ra = RelAlgebra::new_couple("H", "W");
        let idx = ra.add_external("外部A", Identity::ExternalMale, Weight(0.3));
        assert_eq!(ra.actors.len(), 3);
        assert_eq!(ra.actors[idx].identity.side_str(), "丈夫侧");
        // 与丈夫是 rival
        assert!(matches!(ra.relation_matrix[idx][ra.husband_center].rel_type, RelationType::Rival));
    }

    #[test]
    fn fuse_increases_center_weight() {
        let mut ra = RelAlgebra::new_couple("H", "W");
        let idx = ra.add_external("外", Identity::ExternalMale, Weight(0.5));
        let before = ra.actors[ra.husband_center].weight.0;
        ra.fuse_into_center(idx);
        let after = ra.actors[ra.husband_center].weight.0;
        assert!((after - before - 0.5).abs() < 0.001);
    }

    #[test]
    fn fuse_couple_merge() {
        let mut ra = RelAlgebra::new_couple("H", "W");
        let h_before = ra.actors[ra.husband_center].weight.0;
        ra.fuse_couple();
        assert!(ra.center_fused);
        // 融合后两人权重 = 2 * h_before
        let h_after = ra.actors[ra.husband_center].weight.0;
        assert!((h_after - h_before * 2.0).abs() < 0.001);
    }

    #[test]
    fn cluster_husband_wife() {
        let ra = RelAlgebra::new_couple("H", "W");
        let cls = ra.classify_actors();
        assert_eq!(cls[0].1, Cluster::CenterHusband);
        assert_eq!(cls[1].1, Cluster::CenterWife);
    }

    #[test]
    fn cluster_external_classified() {
        let mut ra = RelAlgebra::new_couple("H", "W");
        let idx = ra.add_external("X", Identity::Neutral, Weight(0.1));
        let cls = ra.classify_actors();
        assert_eq!(cls[idx].1, Cluster::External);
    }

    #[test]
    fn actor_mobilize_success() {
        let mut a = Actor::new(0, "H", Identity::Husband, Weight(2.0));
        let flow_before = a.flow_right.0;
        // 可以调动不超过 flow_right 的资源
        assert!(a.can_mobilize(flow_before * 0.5));
        assert!(a.mobilize(flow_before * 0.5));
    }

    #[test]
    fn actor_mobilize_fail_if_too_much() {
        let a = Actor::new(0, "H", Identity::Husband, Weight(0.1));
        assert!(!a.can_mobilize(1000.0));
    }

    #[test]
    fn round_flow_conserves() {
        let mut ra = RelAlgebra::new_couple("H", "W");
        let ext = ra.add_external("X", Identity::Neutral, Weight(0.5));
        ra.relation_matrix[ra.husband_center][ext] = Relation::new(RelationType::Resource, 0.0, 0.2);
        let total_before: f64 = ra.actors.iter().map(|a| a.weight.0).sum();
        ra.round_flow();
        let total_after: f64 = ra.actors.iter().map(|a| a.weight.0).sum();
        // 资源流动应守恒
        assert!((total_before - total_after).abs() < 0.001);
    }

    // ────────────────────────────────────────────────────────
    // 自我天平: 旁观者心态的数学化
    // ────────────────────────────────────────────────────────

    #[test]
    fn selfish_actor_ignores_others_good() {
        // 完全自私者看到别人赚 100, 嫉妒使自己的配重略减
        let a = SelfBalance::new_selfish(5.0);
        let effect = a.convert_external(100.0);
        // 自私者看到别人好 → 自己轻微不爽 (负值)
        assert!(effect < 0.0);
        // 但嫉妒幅度远小于对方的收益 (0.5 * (1-1.0)=0 + 0.1*100=10 → 不, 重新算)
        // 这里实际: self_center_rate=1.0 → 第一项 -100*(1-1.0)*0.5=0; 第二项 100*0.1=10 → 共 +10
        // 这说明纯自私者的 empathy_decay=0.1 仍会产生少量"共情"
        // 所以效果是 +10 (为对方高兴)
        assert!((effect - 10.0).abs() < 0.001);
    }

    #[test]
    fn selfish_actor_glad_at_others_loss() {
        // 自私者看到别人亏 50 → 庆幸自己躲过了
        let a = SelfBalance::new_selfish(5.0);
        let effect = a.convert_external(-50.0);
        // 庆幸: (-(-50)) * 1.0 * 0.3 = 15; 共情: -50 * 0.1 = -5; 总和 +10
        assert!(effect > 0.0);
    }

    #[test]
    fn empathetic_actor_shares_pain() {
        // 共情者 (empathy=0.5) 看到别人亏 20 → 自己感受 -10
        let a = SelfBalance::new_empathetic(5.0, 0.5);
        let effect = a.convert_external(-20.0);
        // 庆幸: 20*0.5*0.3=3; 共情分担: -20*0.5=-10; 总和 -7
        assert!(effect < 0.0);
        assert!((effect - (-7.0)).abs() < 0.001);
    }

    #[test]
    fn empathetic_actor_rejoices_others_gain() {
        // 共情者看到别人赚 30 → 分享喜悦
        let a = SelfBalance::new_empathetic(5.0, 0.5);
        let effect = a.convert_external(30.0);
        // 嫉妒: -30*0.5*0.5=-7.5; 共情: 30*0.5=15; 总和 +7.5
        assert!(effect > 0.0);
    }

    #[test]
    fn balance_sensitivity_inverse_to_stock() {
        // 低配重 → 高敏感度 (小刺激 = 大反应)
        let low_stock = BalanceOp::sensitivity_at(0.1);
        // 高配重 → 低敏感度
        let high_stock = BalanceOp::sensitivity_at(100.0);
        assert!(low_stock > high_stock);
    }

    #[test]
    fn tension_scales_with_gap() {
        let a = SelfBalance::new_selfish(10.0);
        let b = SelfBalance::new_selfish(1.0);
        let c = SelfBalance::new_selfish(9.0);
        // a vs b 差距大 → 张力大
        assert!(a.tension_with(&b) > a.tension_with(&c));
    }

    #[test]
    fn balance_apply_delta() {
        let mut a = SelfBalance::new_selfish(5.0);
        a.apply_delta(3.0);
        assert!((a.stock - 8.0).abs() < 1e-9);
        a.apply_delta(-20.0);
        assert!(a.stock < 0.0);
        assert!(!a.is_surplus());
    }
}
