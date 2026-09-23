// ╔══════════════════════════════════════════════════════════════════╗
// ║  v0.71 亚对象分类器                                              ║
// ║  Topos理论 · 三值真值 · 特征函数 · 子对象 · 拉回运算               ║
// ║                                                                  ║
// ║  Topos 核心: Ω (真值对象)                                        ║
// ║  · 子对象分类器: 每个对象 X 的子对象 ↔ 态射 X → Ω                ║
// ║  · 特征函数: χ: X → Ω, 标记子对象的「是/否/未知」                 ║
// ║  · 子对象: χ⁻¹(True) 的原像                                     ║
// ║  · 拉回 (Pullback): 子对象在态射下的限制                          ║
// ╚══════════════════════════════════════════════════════════════════╝

use crate::container_ir::{ContainerAtlas, ContainerKind, ContainerRef, RelationKind};
use crate::distance_perception::ContainerCode;

/// 三值真值 — Ω (Topos 真值对象)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TruthValue {
    /// 真 — 容器完全满足条件
    True,
    /// 假 — 容器不满足条件
    False,
    /// 未知 — 条件对该容器无法判定 (叠加态)
    Unknown,
}

impl TruthValue {
    /// 逻辑合取 (AND)
    pub fn and(self, other: Self) -> Self {
        match (self, other) {
            (TruthValue::True, TruthValue::True) => TruthValue::True,
            (TruthValue::False, _) | (_, TruthValue::False) => TruthValue::False,
            _ => TruthValue::Unknown,
        }
    }

    /// 逻辑析取 (OR)
    pub fn or(self, other: Self) -> Self {
        match (self, other) {
            (TruthValue::True, _) | (_, TruthValue::True) => TruthValue::True,
            (TruthValue::False, TruthValue::False) => TruthValue::False,
            _ => TruthValue::Unknown,
        }
    }

    /// 逻辑否定 (NOT)
    pub fn not(self) -> Self {
        match self {
            TruthValue::True => TruthValue::False,
            TruthValue::False => TruthValue::True,
            TruthValue::Unknown => TruthValue::Unknown,
        }
    }

    /// 蕴含 (A → B)
    pub fn implies(self, other: Self) -> Self {
        self.not().or(other)
    }

    /// 等价 (A ↔ B)
    pub fn iff(self, other: Self) -> Self {
        self.implies(other).and(other.implies(self))
    }

    /// 是否为真
    pub fn is_true(self) -> bool {
        matches!(self, TruthValue::True)
    }

    /// 是否为假
    pub fn is_false(self) -> bool {
        matches!(self, TruthValue::False)
    }

    /// 真值强度排序: True > Unknown > False
    pub fn strength(self) -> u8 {
        match self {
            TruthValue::True => 2,
            TruthValue::Unknown => 1,
            TruthValue::False => 0,
        }
    }
}

impl Default for TruthValue {
    fn default() -> Self {
        TruthValue::Unknown
    }
}

/// 特征函数 — 从容器到真值的映射
pub type CharacteristicFn<'a> = Box<dyn Fn(&ContainerRef) -> TruthValue + 'a>;

/// 子对象 — 满足特征函数的容器集合
#[derive(Clone, Debug)]
pub struct Subobject {
    /// 子对象名称
    pub name: String,
    /// 满足条件的容器
    pub members: Vec<ContainerRef>,
    /// 特征函数描述
    pub predicate_desc: String,
    /// 子对象权重 (成员数)
    pub weight: usize,
}

impl Subobject {
    pub fn new(name: &str, predicate_desc: &str) -> Self {
        Subobject {
            name: name.to_string(),
            members: Vec::new(),
            predicate_desc: predicate_desc.to_string(),
            weight: 0,
        }
    }

    /// 添加成员
    pub fn add_member(&mut self, cref: ContainerRef) {
        self.members.push(cref);
        self.weight = self.members.len();
    }

    /// 是否包含
    pub fn contains(&self, cref: &ContainerRef) -> bool {
        self.members.iter().any(|m| m.raw() == cref.raw())
    }

    /// 成员数
    pub fn len(&self) -> usize {
        self.members.len()
    }

    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }
}

/// 分类器 — 对容器图集进行分类
pub struct Classifier<'a> {
    /// 特征函数列表
    predicates: Vec<(String, CharacteristicFn<'a>)>,
    /// 分类结果
    subobjects: Vec<Subobject>,
}

impl<'a> Classifier<'a> {
    pub fn new() -> Self {
        Classifier {
            predicates: Vec::new(),
            subobjects: Vec::new(),
        }
    }

    /// 注册特征函数 (条件)
    pub fn register_predicate<F>(&mut self, name: &str, desc: &str, f: F)
    where
        F: Fn(&ContainerRef) -> TruthValue + 'a,
    {
        self.predicates.push((name.to_string(), Box::new(f)));
        self.subobjects.push(Subobject::new(name, desc));
    }

    /// 对所有容器运行分类
    pub fn classify(&mut self, containers: &[ContainerRef]) {
        for (i, (_name, pred)) in self.predicates.iter().enumerate() {
            for cref in containers {
                if pred(cref).is_true() {
                    self.subobjects[i].add_member(cref.clone());
                }
            }
        }
    }

    /// 获取分类结果
    pub fn subobjects(&self) -> &[Subobject] {
        &self.subobjects
    }

    /// 按名称查找子对象
    pub fn find(&self, name: &str) -> Option<&Subobject> {
        self.subobjects.iter().find(|s| s.name == name)
    }

    /// 分类总数
    pub fn subobject_count(&self) -> usize {
        self.subobjects.len()
    }

    /// 总分类成员数
    pub fn total_members(&self) -> usize {
        self.subobjects.iter().map(|s| s.len()).sum()
    }
}

impl<'a> Default for Classifier<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// 拉回 — 子对象在态射下的限制
pub struct Pullback;

impl Pullback {
    /// 计算两个子对象的拉回 (交集)
    pub fn compute(a: &Subobject, b: &Subobject) -> Subobject {
        let mut result = Subobject::new(
            &format!("{} ⋂ {}", a.name, b.name),
            &format!("拉回({}, {})", a.predicate_desc, b.predicate_desc),
        );

        for member in &a.members {
            if b.contains(member) && !result.contains(member) {
                result.add_member(member.clone());
            }
        }

        result
    }

    /// 子对象在特征函数下的拉回: f⁻¹(True) ∩ S
    pub fn restrict<F>(subobject: &Subobject, f: F) -> Subobject
    where
        F: Fn(&ContainerRef) -> TruthValue,
    {
        let mut result = Subobject::new(
            &format!("f⁻¹(True) ∩ {}", subobject.name),
            "限制性拉回",
        );

        for member in &subobject.members {
            if f(member).is_true() {
                result.add_member(member.clone());
            }
        }

        result
    }
}

/// 真值常量
pub const TRUE: TruthValue = TruthValue::True;
pub const FALSE: TruthValue = TruthValue::False;
pub const UNKNOWN: TruthValue = TruthValue::Unknown;

/// Topos 真值对象 — Heyting 代数运算
pub struct HeytingAlgebra;

impl HeytingAlgebra {
    /// 合取 (∧)
    pub fn meet(a: TruthValue, b: TruthValue) -> TruthValue {
        a.and(b)
    }

    /// 析取 (∨)
    pub fn join(a: TruthValue, b: TruthValue) -> TruthValue {
        a.or(b)
    }

    /// 蕴含 (⇒)
    pub fn implies(a: TruthValue, b: TruthValue) -> TruthValue {
        a.implies(b)
    }

    /// 否定 (¬)
    pub fn negate(a: TruthValue) -> TruthValue {
        a.not()
    }

    /// 底 (⊥)
    pub fn bottom() -> TruthValue {
        TruthValue::False
    }

    /// 顶 (⊤)
    pub fn top() -> TruthValue {
        TruthValue::True
    }
}

/// 便捷: 基于容器码字段的特征函数生成器
pub struct Predicates;

impl Predicates {
    /// 家族判定
    pub fn family_is(target: u8) -> impl Fn(&ContainerRef) -> TruthValue {
        move |cref| {
            let code = ContainerCode::new(cref.entity() as u32);
            if code.family() == target {
                TruthValue::True
            } else {
                TruthValue::False
            }
        }
    }

    /// 极性判定
    pub fn polarity_is(target: u8) -> impl Fn(&ContainerRef) -> TruthValue {
        move |cref| {
            let code = ContainerCode::new(cref.entity() as u32);
            if code.polarity() == target {
                TruthValue::True
            } else {
                TruthValue::False
            }
        }
    }

    /// 结构判定
    pub fn structure_in_range(min: u8, max: u8) -> impl Fn(&ContainerRef) -> TruthValue {
        move |cref| {
            let code = ContainerCode::new(cref.entity() as u32);
            let s = code.structure();
            if s >= min && s <= max {
                TruthValue::True
            } else {
                TruthValue::False
            }
        }
    }

    /// 复合条件 (AND)
    pub fn and<F, G>(f: F, g: G) -> impl Fn(&ContainerRef) -> TruthValue
    where
        F: Fn(&ContainerRef) -> TruthValue,
        G: Fn(&ContainerRef) -> TruthValue,
    {
        move |cref| {
            let a = f(cref);
            let b = g(cref);
            if a.is_true() && b.is_true() {
                TruthValue::True
            } else if a.is_false() || b.is_false() {
                TruthValue::False
            } else {
                TruthValue::Unknown
            }
        }
    }

    /// 复合条件 (OR)
    pub fn or<F, G>(f: F, g: G) -> impl Fn(&ContainerRef) -> TruthValue
    where
        F: Fn(&ContainerRef) -> TruthValue,
        G: Fn(&ContainerRef) -> TruthValue,
    {
        move |cref| {
            let a = f(cref);
            let b = g(cref);
            if a.is_true() || b.is_true() {
                TruthValue::True
            } else if a.is_false() && b.is_false() {
                TruthValue::False
            } else {
                TruthValue::Unknown
            }
        }
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_ref(entity: u64) -> ContainerRef {
        ContainerRef::new(ContainerKind::Maori, entity)
    }

    #[test]
    fn truth_value_and() {
        assert_eq!(TRUE.and(TRUE), TRUE);
        assert_eq!(TRUE.and(FALSE), FALSE);
        assert_eq!(FALSE.and(TRUE), FALSE);
        assert_eq!(FALSE.and(FALSE), FALSE);
    }

    #[test]
    fn truth_value_or() {
        assert_eq!(TRUE.or(TRUE), TRUE);
        assert_eq!(TRUE.or(FALSE), TRUE);
        assert_eq!(FALSE.or(TRUE), TRUE);
        assert_eq!(FALSE.or(FALSE), FALSE);
    }

    #[test]
    fn truth_value_not() {
        assert_eq!(TRUE.not(), FALSE);
        assert_eq!(FALSE.not(), TRUE);
        assert_eq!(UNKNOWN.not(), UNKNOWN);
    }

    #[test]
    fn truth_value_implies() {
        assert_eq!(TRUE.implies(TRUE), TRUE);
        assert_eq!(TRUE.implies(FALSE), FALSE);
        assert_eq!(FALSE.implies(TRUE), TRUE);
        assert_eq!(FALSE.implies(FALSE), TRUE);
    }

    #[test]
    fn truth_value_iff() {
        assert_eq!(TRUE.iff(TRUE), TRUE);
        assert_eq!(FALSE.iff(FALSE), TRUE);
        assert_eq!(TRUE.iff(FALSE), FALSE);
        assert_eq!(FALSE.iff(TRUE), FALSE);
    }

    #[test]
    fn truth_value_with_unknown() {
        // True AND Unknown = Unknown
        assert_eq!(TRUE.and(UNKNOWN), UNKNOWN);
        // False AND Unknown = False
        assert_eq!(FALSE.and(UNKNOWN), FALSE);
        // True OR Unknown = True
        assert_eq!(TRUE.or(UNKNOWN), TRUE);
        // False OR Unknown = Unknown
        assert_eq!(FALSE.or(UNKNOWN), UNKNOWN);
    }

    #[test]
    fn subobject_new() {
        let s = Subobject::new("test", "测试条件");
        assert_eq!(s.name, "test");
        assert_eq!(s.predicate_desc, "测试条件");
        assert!(s.is_empty());
    }

    #[test]
    fn subobject_add_and_contains() {
        let mut s = Subobject::new("Maori家族", "家族==3");
        let a = mk_ref(0xE000_0000); // family=7
        let b = mk_ref(0x6000_0000); // family=3
        s.add_member(a.clone());
        s.add_member(b.clone());

        assert_eq!(s.len(), 2);
        assert!(s.contains(&a));
        assert!(s.contains(&b));
        assert_eq!(s.weight, 2);
    }

    #[test]
    fn classifier_register_and_classify() {
        let mut c = Classifier::new();
        // 家族 == 3 的容器
        c.register_predicate("家族3", "家族字段==3", |cref| {
            let code = ContainerCode::new(cref.entity() as u32);
            if code.family() == 3 { TRUE } else { FALSE }
        });

        let containers = vec![
            mk_ref(0x6000_0000), // family=3
            mk_ref(0xE000_0000), // family=7
            mk_ref(0x6000_0001), // family=3
        ];

        c.classify(&containers);
        let sub = c.find("家族3").unwrap();
        assert_eq!(sub.len(), 2);
    }

    #[test]
    fn classifier_multiple_predicates() {
        let mut c = Classifier::new();
        c.register_predicate("极性0", "极性==0(阴)", |cref| {
            let code = ContainerCode::new(cref.entity() as u32);
            if code.polarity() == 0 { TRUE } else { FALSE }
        });
        c.register_predicate("极性1", "极性==1(阳)", |cref| {
            let code = ContainerCode::new(cref.entity() as u32);
            if code.polarity() == 1 { TRUE } else { FALSE }
        });

        let containers = vec![
            mk_ref(0x6000_0000), // polarity=0 (bit 28 = 0)
            mk_ref(0x7000_0000), // polarity=1 (bit 28 = 1)
        ];

        c.classify(&containers);
        assert_eq!(c.subobject_count(), 2);
    }

    #[test]
    fn pullback_intersection() {
        let mut a = Subobject::new("A", "条件A");
        let r1 = mk_ref(100);
        let r2 = mk_ref(200);
        let r3 = mk_ref(300);
        a.add_member(r1.clone());
        a.add_member(r2.clone());

        let mut b = Subobject::new("B", "条件B");
        b.add_member(r2.clone());
        b.add_member(r3.clone());

        let result = Pullback::compute(&a, &b);
        assert_eq!(result.len(), 1);
        assert!(result.contains(&r2));
    }

    #[test]
    fn pullback_disjoint() {
        let mut a = Subobject::new("A", "A");
        let mut b = Subobject::new("B", "B");
        a.add_member(mk_ref(1));
        b.add_member(mk_ref(2));

        let result = Pullback::compute(&a, &b);
        assert!(result.is_empty());
    }

    #[test]
    fn pullback_restrict() {
        let mut s = Subobject::new("源子对象", "基础条件");
        s.add_member(mk_ref(0x6000_0000)); // family=3
        s.add_member(mk_ref(0xE000_0000)); // family=7

        let result = Pullback::restrict(&s, |cref| {
            let code = ContainerCode::new(cref.entity() as u32);
            if code.family() == 3 { TRUE } else { FALSE }
        });

        assert_eq!(result.len(), 1);
        assert!(result.contains(&mk_ref(0x6000_0000)));
    }

    #[test]
    fn heyting_algebra() {
        assert_eq!(HeytingAlgebra::meet(TRUE, FALSE), FALSE);
        assert_eq!(HeytingAlgebra::join(TRUE, FALSE), TRUE);
        assert_eq!(HeytingAlgebra::implies(FALSE, TRUE), TRUE);
        assert_eq!(HeytingAlgebra::negate(TRUE), FALSE);
        assert_eq!(HeytingAlgebra::bottom(), FALSE);
        assert_eq!(HeytingAlgebra::top(), TRUE);
    }

    #[test]
    fn predicate_family_is() {
        let containers = vec![
            mk_ref(0x6000_0000), // family=3
            mk_ref(0xE000_0000), // family=7
        ];

        let f_is_3 = Predicates::family_is(3);
        assert_eq!(f_is_3(&containers[0]), TRUE);
        assert_eq!(f_is_3(&containers[1]), FALSE);
    }

    #[test]
    fn predicate_composition() {
        let containers = vec![
            mk_ref(0x6000_0000), // family=3, polarity=0
            mk_ref(0x7000_0000), // family=3, polarity=1
            mk_ref(0xE000_0000), // family=7, polarity=0
        ];

        let family3 = Predicates::family_is(3);
        let polarity0 = Predicates::polarity_is(0);

        let combined = Predicates::and(family3, polarity0);
        assert_eq!(combined(&containers[0]), TRUE);  // family=3 AND polarity=0
        assert_eq!(combined(&containers[1]), FALSE); // family=3 AND polarity=1
        assert_eq!(combined(&containers[2]), FALSE); // family=7 AND polarity=0
    }

    #[test]
    fn truth_value_strength_ordering() {
        assert!(TRUE.strength() > UNKNOWN.strength());
        assert!(UNKNOWN.strength() > FALSE.strength());
    }
}
