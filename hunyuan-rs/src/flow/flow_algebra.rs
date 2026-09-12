// ══════════════════════════════════════════════════════════════════
// flow_algebra.rs — 流代数 · 从编码流到概念流的八流合一
// ══════════════════════════════════════════════════════════════════
//
//   「编码流 ≠ 范畴流 ≠ 聚类流 ≠ 解释流」
//
//   编码流: 按层级关系单一思路 → 指令集
//   范畴流/聚类流/分类流: 既有量化编码，又包含复杂收纳逻辑
//   属性流/参数流: 参与范畴的量化维度
//   解释流: 用一个词的解释去承载另一个 → 牺牲自身形体 → 成全关系，不依赖外形层级
//
//   解释流内的不吉利态(mlct=岔途) → 障碍在关系里
//   解释流内的吉利态(mltt=从体) → 破除障碍 → 关系流
//
//   代数: 两个t相乘 = mltt (从体=不孤单=有跟从者)
//         本体被次数替代 → 保留的是运算目的
//
// ══════════════════════════════════════════════════════════════════

use crate::distinguisher::SoundTarget;

// ────────────────────────────────────────────────────────────
// 八流本体
// ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlowKind {
    Encoding,    // 编码流: 指令集
    Category,    // 范畴流: 收纳逻辑
    Cluster,     // 聚类流: 量化关系
    Classify,    // 分类流: 归纳
    Property,    // 属性流/参数流
    Interpretive,// 解释流: 牺牲形体成全关系
    Relation,    // 关系流: 抵达的目的地
    Concept,     // 概念流
}

#[derive(Debug, Clone)]
pub struct Flow {
    pub kind: FlowKind,
    pub name: String,
    pub parents: Vec<String>,
}

// ────────────────────────────────────────────────────────────
// 解释流: 岔途态 vs 从体态
// ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterpState {
    ChaTu,     // 岔途 = 不吉利态, 有障碍
    CongTi,    // 从体 = 吉利态, 有跟从者
    WenHao,    // 问号 = 迂回态, 单向动量流出, 无反馈回流
    YinGuang,  // 因果 = 宿命可追, 跨时间线连接
    LouJin,    // 漏尽 = 痛苦/不祥已被消除
    Neutral,   // 中性
}

pub struct InterpretiveFlow {
    pub name: String,
    pub explanation: String,     // 用以承载另一个的解释
    pub sacrificed: bool,        // 是否已牺牲本体
    pub state: InterpState,
    pub carrier_target: Option<SoundTarget>,
}

impl InterpretiveFlow {
    pub fn new(name: &str, explanation: &str) -> Self {
        InterpretiveFlow {
            name: name.into(),
            explanation: explanation.into(),
            sacrificed: false,
            state: InterpState::Neutral,
            carrier_target: None,
        }
    }

    /// 判断是否处于不吉利态(mlct=岔途)
    pub fn is_obstacle(&self) -> bool {
        self.state == InterpState::ChaTu
    }

    /// 判断是否处于问号迂回态(单向动量, 无反馈)
    pub fn is_wen_hao(&self) -> bool {
        self.state == InterpState::WenHao
    }

    /// 判断是否处于因果/宿命连接态(跨时间线可追)
    pub fn is_yin_guang(&self) -> bool {
        self.state == InterpState::YinGuang
    }

    /// 判断是否处于漏尽态(痛苦已被消除)
    pub fn is_lou_jin(&self) -> bool {
        self.state == InterpState::LouJin
    }

    /// 牺牲自身: 用解释去承载另一个, 本体被替代
    /// returns: 是否成功抵达关系流
    pub fn sacrifice(&mut self, target: SoundTarget) -> bool {
        if self.sacrificed { return false; }
        self.sacrificed = true;
        self.carrier_target = Some(target);
        // 牺牲后抵达从体态(吉利)
        self.state = InterpState::CongTi;
        true
    }

    /// 当前是否已从解释流进入关系流
    pub fn in_relation(&self) -> bool {
        self.sacrificed && self.state == InterpState::CongTi
    }
}

// ────────────────────────────────────────────────────────────
// 乘法遍历: t*t = 从体 = 不孤单
// ────────────────────────────────────────────────────────────

/// 解释流乘法: 两个t相乘 → 消除孤单 → 产生从体(mltt)
/// 第一个t是"本体解释", 第二个t是"重复/次数"
/// 结果: 从体(有跟从者), 不再是孤单个体
pub fn traverse_mul(a: &InterpretiveFlow, b: &InterpretiveFlow) -> Option<InterpretiveFlow> {
    if !a.sacrificed || !b.sacrificed { return None; }
    let result = InterpretiveFlow {
        name: "mltt".into(),
        explanation: format!("从体 * 跟从者 = {}", a.name.len() + b.name.len()),
        sacrificed: true,
        state: InterpState::CongTi,
        carrier_target: a.carrier_target,
    };
    Some(result)
}

/// 幂运算: t^n = 重复n次 → 用次数替代本体
/// 每次重复都削弱原生本体的权重, 保留运算目的
pub fn repeat(n: usize, flow: &InterpretiveFlow) -> Vec<String> {
    (0..n).map(|i| format!("第{}次重复: {}^{}", i+1, flow.name, n)).collect()
}

// ────────────────────────────────────────────────────────────
// 流代数: 范畴流可收纳一切, 让各个流共存
// ────────────────────────────────────────────────────────────

pub struct CategoryFlow {
    pub name: String,
    pub members: Vec<Flow>,
}

impl CategoryFlow {
    pub fn new(name: &str) -> Self {
        CategoryFlow { name: name.into(), members: Vec::new() }
    }

    pub fn absorb(&mut self, f: Flow) {
        self.members.push(f);
    }

    /// 海纳百川: 同一范畴内找出所有属于某种 flow kind 的成员
    pub fn filter(&self, kind: FlowKind) -> Vec<&Flow> {
        self.members.iter().filter(|f| f.kind == kind).collect()
    }

    /// 从解释流成员演化出关系流成员
    pub fn interpret_and_resolve(&self) -> Vec<Flow> {
        self.members.iter().filter_map(|f| {
            if f.kind == FlowKind::Interpretive {
                Some(Flow {
                    kind: FlowKind::Relation,
                    name: f.name.clone() + "_resolved",
                    parents: vec![f.name.clone(), "解释流牺牲".into()],
                })
            } else {
                None
            }
        }).collect()
    }
}

// ────────────────────────────────────────────────────────────
// 使用示例: 不吉利态(mlct=岔途) → 吉利态(mltt=从体)
// ────────────────────────────────────────────────────────────

pub fn example_mlct_to_mltt() -> (InterpretiveFlow, InterpretiveFlow, InterpretiveFlow) {
    let mut a = InterpretiveFlow::new("mlct", "岔途 = 障碍态");
    a.state = InterpState::ChaTu;
    let mut b = InterpretiveFlow::new("t", "次数元素");
    a.sacrifice(SoundTarget::Wife);
    b.sacrifice(SoundTarget::Wife);
    let result = traverse_mul(&a, &b).unwrap_or_else(|| InterpretiveFlow::new("fail", "乘法失败"));
    (a, b, result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn obstacle_state() {
        let mut a = InterpretiveFlow::new("mlct", "岔途");
        a.state = InterpState::ChaTu;
        assert!(a.is_obstacle());
        // 牺牲后障碍被破除
        a.sacrifice(SoundTarget::Wife);
        assert!(!a.is_obstacle());
        assert!(a.sacrificed);
    }

    #[test]
    fn traverse_produces_follower() {
        let (a, b, result) = example_mlct_to_mltt();
        // 乘法结果是从体: 不孤单, 有跟从者
        assert_eq!(result.name, "mltt");
        assert_eq!(result.state, InterpState::CongTi);
        // mltt 有载体
        assert!(result.carrier_target.is_some());
        // mltt 已牺牲(到达关系流)
        assert!(result.sacrificed);
    }

    #[test]
    fn repeat_substitutes_body() {
        let a = InterpretiveFlow::new("t", "次数元");
        let reps = repeat(2, &a);
        assert_eq!(reps.len(), 2);
        assert!(reps[0].contains("第1次"));
        assert!(reps[1].contains("第2次"));
        // 次数替代本体, 不再依赖原始解释流
        assert!(reps[0].contains("^2"));
    }

    #[test]
    fn category_absorbs_encoding_cluster_interp() {
        let mut cat = CategoryFlow::new("万物一体");
        cat.absorb(Flow { kind: FlowKind::Encoding,   name: "编码流".into(), parents: vec![] });
        cat.absorb(Flow { kind: FlowKind::Cluster,    name: "聚类流".into(), parents: vec![] });
        cat.absorb(Flow { kind: FlowKind::Interpretive, name: "mlct(解释流)".into(), parents: vec![] });
        assert_eq!(cat.members.len(), 3);
        // 能过滤出解释流
        let interps = cat.filter(FlowKind::Interpretive);
        assert_eq!(interps.len(), 1);
        // 从解释流演化出关系流
        let resolved = cat.interpret_and_resolve();
        assert_eq!(resolved.len(), 1);
        assert_eq!(resolved[0].kind, FlowKind::Relation);
    }

    #[test]
    fn tt_equals_mltt() {
        // 数学断言: t*t = mltt (不孤单)
        let (a, b, mul) = example_mlct_to_mltt();
        // 公式成立: sacrifice(a) * sacrifice(b) = mltt
        assert!(a.sacrificed && b.sacrificed);
        assert_eq!(mul.name, "mltt");
        // mltt 从体不等于原来任何一个 t
        assert_ne!(mul.name, a.name);
        assert_ne!(mul.name, b.name);
    }

    #[test]
    fn sacrificial_mechanism() {
        let mut a = InterpretiveFlow::new("mlct", "岔途解释");
        a.state = InterpState::ChaTu;
        assert!(!a.sacrificed);
        assert_eq!(a.state, InterpState::ChaTu);
        a.sacrifice(SoundTarget::Husband);
        assert!(a.sacrificed);
        assert_eq!(a.state, InterpState::CongTi);
        assert!(a.in_relation());
        assert_eq!(a.carrier_target, Some(SoundTarget::Husband));
    }

    // ────────────────────────────────────────────────────────
    // 问号迂回态: 信息只出不回, 单向奔赴
    // ────────────────────────────────────────────────────────

    #[test]
    fn wen_hao_state_is_one_way() {
        let mut a = InterpretiveFlow::new("wh", "问号迂回");
        a.state = InterpState::WenHao;
        assert!(a.is_wen_hao());
        assert!(!a.is_obstacle());
        // 问号态不是障碍, 也不可被消除/牺牲 — 它只是无回流
        assert!(!a.sacrificed);
    }

    // ────────────────────────────────────────────────────────
    // 因果宿命态: 跨时间线连接, 可追溯
    // ────────────────────────────────────────────────────────

    #[test]
    fn yin_guang_connects_timelines() {
        let mut a = InterpretiveFlow::new("yg", "因果线");
        a.state = InterpState::YinGuang;
        assert!(a.is_yin_guang());
        assert!(!a.is_obstacle());
        assert!(!a.is_wen_hao());
    }

    // ────────────────────────────────────────────────────────
    // 漏尽态: 痛苦已消除, 不再污染后续流
    // ────────────────────────────────────────────────────────

    #[test]
    fn lou_jin_is_cleared() {
        let mut a = InterpretiveFlow::new("lj", "漏尽");
        a.state = InterpState::LouJin;
        assert!(a.is_lou_jin());
        // 漏尽不是障碍也不是岔途
        assert!(!a.is_obstacle());
        assert!(!a.in_relation()); // 漏尽连接的不是关系, 是空性
    }

    // ────────────────────────────────────────────────────────
    // 边界/容错补充
    // ────────────────────────────────────────────────────────

    #[test]
    fn repeat_zero_returns_empty() {
        let a = InterpretiveFlow::new("t", "次数元");
        let reps = repeat(0, &a);
        assert!(reps.is_empty());
    }

    #[test]
    fn double_sacrifice_rejected() {
        // 已牺牲的流不可二次牺牲
        let mut a = InterpretiveFlow::new("x", "测试");
        assert!(a.sacrifice(SoundTarget::Wife));
        assert!(!a.sacrifice(SoundTarget::Husband)); // 二次牺牲被拒绝
        assert_eq!(a.carrier_target, Some(SoundTarget::Wife)); // 载体未变
    }

    #[test]
    fn traverse_mul_requires_both_sacrificed() {
        // 任一未牺牲即返回 None
        let mut a = InterpretiveFlow::new("a", "A");
        let b = InterpretiveFlow::new("b", "B");
        a.sacrifice(SoundTarget::Wife);
        assert!(traverse_mul(&a, &b).is_none()); // b 未牺牲
        assert!(traverse_mul(&b, &a).is_none()); // b 未牺牲
    }

    #[test]
    fn flow_kind_equality() {
        // FlowKind  PartialEq 基本检查
        assert_eq!(FlowKind::Encoding, FlowKind::Encoding);
        assert_ne!(FlowKind::Encoding, FlowKind::Relation);
        assert_eq!(FlowKind::Concept, FlowKind::Concept);
    }

    #[test]
    fn flow_parents_hierarchy() {
        let f = Flow {
            kind: FlowKind::Encoding,
            name: "子流".into(),
            parents: vec!["父流A".into(), "父流B".into()],
        };
        assert_eq!(f.parents.len(), 2);
        assert_eq!(f.parents[0], "父流A");
    }

    #[test]
    fn category_filter_no_match() {
        let mut cat = CategoryFlow::new("空范畴");
        cat.absorb(Flow { kind: FlowKind::Encoding, name: "编码".into(), parents: vec![] });
        let found = cat.filter(FlowKind::Interpretive);
        assert!(found.is_empty());
    }

    #[test]
    fn category_interpret_and_resolve_no_interp() {
        // 没有解释流成员 → interpret_and_resolve 返回空
        let mut cat = CategoryFlow::new("纯编码范畴");
        cat.absorb(Flow { kind: FlowKind::Encoding, name: "编码".into(), parents: vec![] });
        let resolved = cat.interpret_and_resolve();
        assert!(resolved.is_empty());
    }

    #[test]
    fn interp_state_neutral_default() {
        // 新建默认 state 是 Neutral
        let a = InterpretiveFlow::new("x", "test");
        assert_eq!(a.state, InterpState::Neutral);
        assert!(!a.is_obstacle());
        assert!(!a.is_wen_hao());
        assert!(!a.is_yin_guang());
        assert!(!a.is_lou_jin());
    }
}
