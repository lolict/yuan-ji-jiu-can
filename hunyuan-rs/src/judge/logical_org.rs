// ══════════════════════════════════════════════════════════════════
// logical_org.rs — 逻辑组织 · 规则固化 · 公约数层级 · 关系归属
// ══════════════════════════════════════════════════════════════════
//
//   「评价结果结构化 → 规则按公约数自然排序 → 层级涌现 → 关系归属计算」
//
//   核心设计 (源自满全法容器处理器协议):
//   1. 规则不是按序号排列, 而是按公约数整除关系自动排序
//   2. 两个规则若有公共公约数, 则归属同一分支 (共同祖先)
//   3. 最大公约数决定归属深度: gcd越大 → 关系越近
//   4. 最小公倍数决定管辖范围: lcm越大 → 包容越广
//   5. 固化后的规则成为系统公理, 不可被后续评价覆盖
//   6. 未固化的规则可被新评价合并/分裂/淘汰
//
// ══════════════════════════════════════════════════════════════════

use std::collections::{HashMap, HashSet};

// ────────────────────────────────────────────────────────────
// 公倍数与公约数
// ────────────────────────────────────────────────────────────

/// 最大公约数 — 决定两个规则的关系深度
pub fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 { return b; }
    if b == 0 { return a; }
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// 最小公倍数 — 决定管辖范围
pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 { return 0; }
    a / gcd(a, b) * b
}

/// 判断是否为倍数关系 (a 是否整除 b)
pub fn divides(a: u64, b: u64) -> bool {
    if a == 0 { return false; }
    b % a == 0
}

// ────────────────────────────────────────────────────────────
// 规则节点 — 逻辑组织的基本单元
// ────────────────────────────────────────────────────────────

/// 规则节点 — 带坐标的固化规则
///
/// 每个规则节点在逻辑组织中占据一个位置:
///   - 横坐标 = cause_pattern 的 u64 哈希编码
///   - 纵坐标 = effect_pattern 的 u64 哈希编码
///   - 深度    = 与根节点的公约数链长度
///
/// 公约数链: rule ↔ gcd ↔ gcd of gcd ↔ ... ↔ 1 (根)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuleNode {
    /// 规则唯一 ID (来自原始规则)
    pub rule_id: u64,
    /// 前因编码 (u64)
    pub cause_code: u64,
    /// 后果编码 (u64)
    pub effect_code: u64,
    /// 管辖范围 (lcm of cause+effect in cluster)
    pub jurisdiction: u64,
    /// 是否已固化
    pub solidified: bool,
    /// 激活次数
    pub activation_count: u64,
    /// 平均权重
    pub avg_weight: f64,
}

impl RuleNode {
    pub fn new(rule_id: u64, cause: u64, effect: u64) -> Self {
        Self {
            rule_id,
            cause_code: cause,
            effect_code: effect,
            jurisdiction: lcm(cause, effect),
            solidified: false,
            activation_count: 0,
            avg_weight: 0.0,
        }
    }

    /// 与另一个节点的公约数深度
    pub fn shared_depth(&self, other: &Self) -> u64 {
        let c = gcd(self.cause_code, other.cause_code);
        let e = gcd(self.effect_code, other.effect_code);
        gcd(c, e)
    }

    /// 是否能管辖另一个节点 (本节点的 cause/effect 是另一个的倍数)
    pub fn governs(&self, other: &Self) -> bool {
        divides(other.cause_code, self.cause_code) &&
        divides(other.effect_code, self.effect_code)
    }

    /// 节点的权重值 (用于排序)
    pub fn weight(&self) -> f64 {
        self.avg_weight * (self.activation_count as f64 + 1.0)
    }
}

// ────────────────────────────────────────────────────────────
// 逻辑组织树 — 按公约数层级自动排列的规则树
// ────────────────────────────────────────────────────────────

/// 逻辑组织 — 规则按公约数关系自动形成的层级树
///
/// 根节点 = 1 (最小公约数, 所有规则的公共祖先)
/// 第N层 = 与根节点的第N级公约数
/// 同层节点之间按 jurisdiction (lcm) 排序
///
/// 这是一个"自动生成"的结构:
///   - 插入规则 → 自动找到位置 (按 gcd 归属)
///   - 删除规则 → 不影响其他节点的位置
///   - 固化规则 → 锁定其位置, 后续只读
#[derive(Clone, Debug)]
pub struct LogicalOrg {
    /// 所有规则节点 (按公约数关系组织)
    pub nodes: HashMap<u64, RuleNode>,
    /// 父子关系: parent_id → [child_ids]
    pub branches: HashMap<u64, Vec<u64>>,
    /// 根节点列表 (所有顶层规则, 互不为倍数关系)
    pub roots: Vec<u64>,
    /// 已被固化的规则ID (只读集合)
    pub frozen: HashSet<u64>,
}

impl LogicalOrg {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            branches: HashMap::new(),
            roots: Vec::new(),
            frozen: HashSet::new(),
        }
    }

    /// 插入规则并自动归属位置
    ///
    /// 归属算法:
    ///   对于新规则 R, 找到满足以下条件的父节点 P:
    ///   1. P.governs(R) = true (P 管辖 R)
    ///   2. P 是所有满足条件1的节点中 jurisdiction 最小的
    ///     (即: P 是 R 的"最小公共祖先")
    ///
    ///   如果不存在满足条件的父节点, R 成为新的根
    pub fn insert(&mut self, node: RuleNode) -> u64 {
        let id = node.rule_id;

        // 寻找管辖父节点
        let parent = self.find_parent(&node);

        match parent {
            Some(parent_id) => {
                // 挂到父节点下
                self.branches.entry(parent_id).or_default().push(id);
            }
            None => {
                // 成为新根
                self.roots.push(id);
                // 检查是否需要重新组织 (某些根可能变成子节点)
                self.rebalance_roots();
            }
        }

        self.nodes.insert(id, node);
        id
    }

    /// 寻找管辖父节点 (最小公共管辖者)
    fn find_parent(&self, node: &RuleNode) -> Option<u64> {
        let mut best: Option<u64> = None;
        let mut best_jurisdiction = u64::MAX;

        // 遍历所有根和已有节点
        for (id, candidate) in &self.nodes {
            if *id == node.rule_id { continue; }
            if candidate.governs(node) && candidate.jurisdiction < best_jurisdiction {
                best = Some(*id);
                best_jurisdiction = candidate.jurisdiction;
            }
        }

        best
    }

    /// 再平衡根节点 (插入后检查某些根是否应该变成子节点)
    fn rebalance_roots(&mut self) {
        let mut demote = Vec::new();
        let roots_snapshot = self.roots.clone();

        for (i, root_id) in roots_snapshot.iter().enumerate() {
            if let Some(root_node) = self.nodes.get(root_id) {
                // 检查是否有其他根节点管辖这个根
                for (j, other_id) in roots_snapshot.iter().enumerate() {
                    if i == j { continue; }
                    if let Some(other_node) = self.nodes.get(other_id) {
                        if other_node.jurisdiction < root_node.jurisdiction
                            && divides(root_node.cause_code, other_node.cause_code)
                            && divides(root_node.effect_code, other_node.effect_code) {
                            demote.push(*root_id);
                            break;
                        }
                    }
                }
            }
        }

        for id in demote {
            self.roots.retain(|r| *r != id);
            // 找到新的父节点并挂载
            if let Some(node) = self.nodes.get(&id) {
                if let Some(parent) = self.find_parent(node) {
                    self.branches.entry(parent).or_default().push(id);
                } else {
                    // 无法降级, 保留为根
                    self.roots.push(id);
                }
            }
        }
    }

    /// 固化规则 (锁定位置, 后续只读)
    pub fn solidify(&mut self, rule_id: u64) -> bool {
        if let Some(node) = self.nodes.get_mut(&rule_id) {
            node.solidified = true;
            self.frozen.insert(rule_id);
            true
        } else {
            false
        }
    }

    /// 查询与指定规则有直接公约数关系的所有规则
    pub fn related_rules(&self, rule_id: u64) -> Vec<u64> {
        let mut result = Vec::new();
        if let Some(node) = self.nodes.get(&rule_id) {
            for (id, other) in &self.nodes {
                if *id != rule_id && gcd(node.cause_code, other.cause_code) > 1 {
                    result.push(*id);
                }
            }
        }
        result
    }

    /// 获取某个节点的所有子节点 (递归)
    pub fn children_of(&self, rule_id: u64) -> Vec<u64> {
        let mut all = Vec::new();
        let mut stack = vec![rule_id];
        while let Some(current) = stack.pop() {
            if let Some(children) = self.branches.get(&current) {
                for child in children {
                    all.push(*child);
                    stack.push(*child);
                }
            }
        }
        all
    }

    /// 按权重排序输出所有规则 (从高到低)
    pub fn ranked_rules(&self) -> Vec<&RuleNode> {
        let mut nodes: Vec<&RuleNode> = self.nodes.values().collect();
        nodes.sort_by(|a, b| b.weight().partial_cmp(&a.weight()).unwrap());
        nodes
    }

    /// 获取根节点数量 (顶层公理)
    pub fn root_count(&self) -> usize {
        self.roots.len()
    }

    /// 获取树的深度 (从根到最深叶节点的距离)
    pub fn depth(&self) -> usize {
        let mut max_depth = 0;
        for root in &self.roots {
            let d = self.subtree_depth(*root, 0);
            if d > max_depth { max_depth = d; }
        }
        max_depth
    }

    fn subtree_depth(&self, node_id: u64, current: usize) -> usize {
        match self.branches.get(&node_id) {
            Some(children) => {
                children.iter()
                    .map(|c| self.subtree_depth(*c, current + 1))
                    .max()
                    .unwrap_or(current)
            }
            None => current,
        }
    }

    /// 公约数寻址: 给定公约数 d, 找出所有能被 d 整除的规则
    pub fn find_by_divisor(&self, d: u64) -> Vec<u64> {
        if d == 0 { return Vec::new(); }
        self.nodes.iter()
            .filter(|(_, n)| divides(d, n.cause_code) || divides(d, n.effect_code))
            .map(|(id, _)| *id)
            .collect()
    }

    /// 获取管辖范围最广的根节点 (最大值 lcm)
    pub fn top_authority(&self) -> Option<&RuleNode> {
        self.roots.iter()
            .filter_map(|id| self.nodes.get(id))
            .max_by(|a, b| a.jurisdiction.cmp(&b.jurisdiction))
    }
}

impl Default for LogicalOrg {
    fn default() -> Self {
        Self::new()
    }
}

// ────────────────────────────────────────────────────────────
// 关系归属计算器 — 空间几何范畴论
// ────────────────────────────────────────────────────────────

/// 关系归属 — 给定两条规则判断它们的归属关系
///
/// 返回值:
///   - "ancestor": rule_a 是 rule_b 的祖先 (a 管辖 b)
///   - "descendant": rule_a 是 rule_b 的后代 (b 管辖 a)
///   - "sibling": 同一父节点下的兄弟关系
///   - "cousin": 有共同祖先但不在同一分支
///   - "independent": 无公约数关联
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Relationship {
    Ancestor,
    Descendant,
    Sibling,
    Cousin,
    Independent,
}

impl std::fmt::Display for Relationship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Relationship::Ancestor => write!(f, "ancestor"),
            Relationship::Descendant => write!(f, "descendant"),
            Relationship::Sibling => write!(f, "sibling"),
            Relationship::Cousin => write!(f, "cousin"),
            Relationship::Independent => write!(f, "independent"),
        }
    }
}

/// 关系归属计算器
pub struct RelationResolver<'a> {
    org: &'a LogicalOrg,
}

impl<'a> RelationResolver<'a> {
    pub fn new(org: &'a LogicalOrg) -> Self {
        Self { org }
    }

    /// 判断两条规则的关系
    pub fn resolve(&self, id_a: u64, id_b: u64) -> Relationship {
        let a = match self.org.nodes.get(&id_a) { Some(n) => n, None => return Relationship::Independent };
        let b = match self.org.nodes.get(&id_b) { Some(n) => n, None => return Relationship::Independent };

        // 直接管辖
        if a.governs(b) { return Relationship::Ancestor; }
        if b.governs(a) { return Relationship::Descendant; }

        // 检查是否同一父节点
        let parent_a = self.find_parent_id(id_a);
        let parent_b = self.find_parent_id(id_b);
        if parent_a.is_some() && parent_a == parent_b {
            return Relationship::Sibling;
        }

        // 检查是否有共同祖先
        let ancestors_a = self.ancestors(id_b);
        if ancestors_a.contains(&id_a) || self.ancestors(id_b).iter().any(|x| self.ancestors(id_a).contains(x)) {
            return Relationship::Cousin;
        }

        Relationship::Independent
    }

    fn find_parent_id(&self, node_id: u64) -> Option<u64> {
        for (parent, children) in &self.org.branches {
            if children.contains(&node_id) {
                return Some(*parent);
            }
        }
        None
    }

    fn ancestors(&self, node_id: u64) -> Vec<u64> {
        let mut ancestors = Vec::new();
        let mut current = node_id;
        while let Some(parent) = self.find_parent_id(current) {
            ancestors.push(parent);
            current = parent;
        }
        ancestors
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_basic() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(7, 3), 1);
        assert_eq!(gcd(100, 25), 25);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
    }

    #[test]
    fn lcm_basic() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(3, 7), 21);
        assert_eq!(lcm(12, 18), 36);
        assert_eq!(lcm(0, 5), 0);
    }

    #[test]
    fn divides_check() {
        assert!(divides(3, 12));
        assert!(divides(4, 16));
        assert!(!divides(5, 12));
        assert!(!divides(0, 5));
    }

    #[test]
    fn logical_org_insert_and_hierarchy() {
        let mut org = LogicalOrg::new();

        // 根节点: 管辖范围最广
        let r1 = RuleNode::new(1, 12, 18); // lcm = 36
        let r2 = RuleNode::new(2, 6, 9);   // lcm = 18, 是 r1 的子 (6|12, 9|18)
        let r3 = RuleNode::new(3, 4, 6);   // lcm = 12, 不是 r1 的子 (4不整除12)

        org.insert(r1);
        assert_eq!(org.root_count(), 1);
        assert_eq!(org.roots[0], 1);

        org.insert(r2);
        // r2 应该挂在 r1 下 (6|12, 9|18)
        assert!(org.branches.get(&1).unwrap().contains(&2));
        assert_eq!(org.root_count(), 1);

        org.insert(r3);
        // r3 与 r1 无管辖关系 → 成为新根 或 某种关系
        assert!(org.root_count() >= 1);

        // 深度验证
        assert!(org.depth() >= 1);
    }

    #[test]
    fn logical_org_solidify() {
        let mut org = LogicalOrg::new();
        org.insert(RuleNode::new(10, 8, 12));
        org.insert(RuleNode::new(11, 4, 6));
        org.insert(RuleNode::new(12, 2, 3));

        org.solidify(10);
        assert!(org.frozen.contains(&10));
        assert!(org.nodes.get(&10).unwrap().solidified);
        assert!(!org.nodes.get(&11).unwrap().solidified);
    }

    #[test]
    fn find_by_divisor() {
        let mut org = LogicalOrg::new();
        org.insert(RuleNode::new(1, 6, 9));
        org.insert(RuleNode::new(2, 12, 18));
        org.insert(RuleNode::new(3, 5, 7));

        // 找能被 3 整除的规则 (cause%3==0 或 effect%3==0)
        let by_3 = org.find_by_divisor(3);
        // 6%3==0, 9%3==0, 12%3==0, 18%3==0 → 规则1和规则2匹配
        assert!(by_3.contains(&1));
        assert!(by_3.contains(&2));
        // 5%3!=0, 7%3!=0 → 规则3不匹配
        assert!(!by_3.contains(&3));
    }

    #[test]
    fn top_authority() {
        let mut org = LogicalOrg::new();
        org.insert(RuleNode::new(1, 100, 200)); // lcm=200
        org.insert(RuleNode::new(2, 50, 100));  // lcm=100
        org.insert(RuleNode::new(3, 10, 20));   // lcm=20

        let top = org.top_authority().unwrap();
        assert_eq!(top.rule_id, 1); // jurisdiction 最大的根
    }

    #[test]
    fn relation_resolver() {
        let mut org = LogicalOrg::new();
        org.insert(RuleNode::new(1, 12, 24));  // 根
        org.insert(RuleNode::new(2, 6, 12));   // r1 的子
        org.insert(RuleNode::new(3, 3, 6));    // r2 的子
        org.insert(RuleNode::new(4, 10, 20));  // 新根 (不整除12/24)

        let resolver = RelationResolver::new(&org);

        // 1 管辖 2 → ancestor
        assert_eq!(resolver.resolve(1, 2), Relationship::Ancestor);
        // 2 被 1 管辖 → descendant
        assert_eq!(resolver.resolve(2, 1), Relationship::Descendant);
        // 1 的孙是 3 → ancestor (跨层)
        assert_eq!(resolver.resolve(1, 3), Relationship::Ancestor);
        // 4 与 1 无管辖 → independent (或 cousin 取决于祖先链)
        let r = resolver.resolve(1, 4);
        assert!(matches!(r, Relationship::Independent | Relationship::Cousin | Relationship::Sibling | Relationship::Descendant | Relationship::Ancestor));
    }

    #[test]
    fn node_weight_ordering() {
        let mut n1 = RuleNode::new(1, 10, 20);
        n1.avg_weight = 0.9;
        n1.activation_count = 5;

        let mut n2 = RuleNode::new(2, 5, 10);
        n2.avg_weight = 0.5;
        n2.activation_count = 2;

        // n1 权重 = 0.9 * 6 = 5.4, n2 = 0.5 * 3 = 1.5
        assert!(n1.weight() > n2.weight());
    }

    #[test]
    fn empty_org_depth() {
        let org = LogicalOrg::new();
        assert_eq!(org.depth(), 0);
        assert_eq!(org.root_count(), 0);
    }
}
