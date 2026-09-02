// ══════════════════════════════════════════════════════════════════
// container_nest.rs — 容器族管理模 (七元蜂窝套壳 · 嵌套容器域)
// ══════════════════════════════════════════════════════════════════
//
// 任务: 用"一个容器管理其他容器的嵌套规则".
//
// 容器族层级 (自外而内 · 由大至小):
//   一元 · 袋 (Dai / Bag)        — 最外层 · 锚点识别边界
//   二元 · 桶 (Tong / Barrel)    — 袋装桶 · 外层容量 = 装几个桶
//   三元 · 罐 (Guan / Can)       — 桶装罐 · 递归进入
//   四元 · 缸 (Gang / Vat)       — 罐外覆缸
//   五元 · 盆 (Pen / Basin)      — 缸外覆盆
//   六元 · 盂 (Yu Bowl)          — 盆外覆盂
//   七元 · 钵 (Bowl)             — 盂外覆钵 · 循环回到一元 (闭环)
//
// 极性规则 (与 7/6 编码对齐):
//   夫 = 7 = 质数 = 独立 = 外层容器边界
//   妻 = 6 = 完全数 = 包容 = 内容纳空间
//   公约差 1 = 夫相邻妻 · 非分离 · 阴阳相吸
//
// 照镜子原则:
//   无论嵌套多少层 · 最内层内容必须能被最外层锚点识别
//   即: 内容 x 各层容器转换 = 锚点可预期结果
//
// 运算约束: 只用加减 + 模式匹配

// ────────────────────────────────────────────────────────────
// 嵌套层级枚举 (一元 → 七元)
// ────────────────────────────────────────────────────────────

/// 七种容器嵌套层级 · 值域 1..=7 · 7 循环回到一元形成闭环
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NestLevel {
    YiYuan,
    ErYuan,
    SanYuan,
    SiYuan,
    WuYuan,
    LiuYuan,
    QiYuan,
}

impl NestLevel {
    /// 层级数值 (1..=7)
    pub fn value(self) -> u8 {
        match self {
            NestLevel::YiYuan  => 1,
            NestLevel::ErYuan  => 2,
            NestLevel::SanYuan => 3,
            NestLevel::SiYuan  => 4,
            NestLevel::WuYuan  => 5,
            NestLevel::LiuYuan => 6,
            NestLevel::QiYuan  => 7,
        }
    }

    /// 层级名称 (中文容器名)
    pub fn container_name(self) -> &'static str {
        match self {
            NestLevel::YiYuan  => "袋",
            NestLevel::ErYuan  => "桶",
            NestLevel::SanYuan => "罐",
            NestLevel::SiYuan  => "缸",
            NestLevel::WuYuan  => "盆",
            NestLevel::LiuYuan => "盂",
            NestLevel::QiYuan  => "钵",
        }
    }

    /// 嵌套短语: "袋装桶装罐..."
    pub fn nesting_phrase(self) -> &'static str {
        match self {
            NestLevel::YiYuan  => "袋",
            NestLevel::ErYuan  => "袋装桶",
            NestLevel::SanYuan => "袋装桶装罐",
            NestLevel::SiYuan  => "袋装桶装罐-缸",
            NestLevel::WuYuan  => "袋装桶装罐-缸-盆",
            NestLevel::LiuYuan => "袋装桶装罐-缸-盆-盂",
            NestLevel::QiYuan  => "袋装桶装罐-缸-盆-盂-钵[闭环]",
        }
    }

    /// 下一层 (向内深入) · 七元回到一元 (闭环)
    pub fn inner(self) -> Self {
        match self {
            NestLevel::YiYuan  => NestLevel::ErYuan,
            NestLevel::ErYuan  => NestLevel::SanYuan,
            NestLevel::SanYuan => NestLevel::SiYuan,
            NestLevel::SiYuan  => NestLevel::WuYuan,
            NestLevel::WuYuan  => NestLevel::LiuYuan,
            NestLevel::LiuYuan => NestLevel::QiYuan,
            NestLevel::QiYuan  => NestLevel::YiYuan,
        }
    }

    /// 上一层 (向外溢出) · 一元之上是七元 (闭环)
    pub fn outer(self) -> Self {
        match self {
            NestLevel::YiYuan  => NestLevel::QiYuan,
            NestLevel::ErYuan  => NestLevel::YiYuan,
            NestLevel::SanYuan => NestLevel::ErYuan,
            NestLevel::SiYuan  => NestLevel::SanYuan,
            NestLevel::WuYuan  => NestLevel::SiYuan,
            NestLevel::LiuYuan => NestLevel::WuYuan,
            NestLevel::QiYuan  => NestLevel::LiuYuan,
        }
    }

    /// 容器的极性: 奇数层=阳(7/夫/边界) · 偶数层=阴(6/妻/包容)
    pub fn polarity(self) -> ContainerPolarity {
        if self.value() % 2 == 1 {
            ContainerPolarity::Yang
        } else {
            ContainerPolarity::Yin
        }
    }

    /// 容量系数 = 8 - 层级值 (越外层容量越大)
    pub fn capacity_factor(self) -> u32 {
        8 - self.value() as u32
    }
}

// ────────────────────────────────────────────────────────────
// 极性 (与 7/6 编码对齐)
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContainerPolarity {
    Yang,
    Yin,
}

impl ContainerPolarity {
    pub fn value(self) -> u8 {
        match self {
            ContainerPolarity::Yang => 7,
            ContainerPolarity::Yin  => 6,
        }
    }

    /// 极性是否匹配: 阳容阴 · 阴容阳 · 相邻相吸
    pub fn accepts(self, content: ContainerPolarity) -> bool {
        matches!(
            (self, content),
            (ContainerPolarity::Yin, ContainerPolarity::Yang)
            | (ContainerPolarity::Yang, ContainerPolarity::Yin)
        )
    }

    pub fn opposite(self) -> Self {
        match self {
            ContainerPolarity::Yang => ContainerPolarity::Yin,
            ContainerPolarity::Yin  => ContainerPolarity::Yang,
        }
    }
}

// ────────────────────────────────────────────────────────────
// 溢出策略
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverflowPolicy {
    Fold,
    Overflow,
}

impl Default for OverflowPolicy {
    fn default() -> Self {
        OverflowPolicy::Fold
    }
}

// ────────────────────────────────────────────────────────────
// 内容项
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Content {
    pub anchor_id: u32,
    pub value: u32,
    pub polarity: ContainerPolarity,
}

impl Content {
    pub fn new(anchor_id: u32, value: u32, polarity: ContainerPolarity) -> Self {
        Self { anchor_id, value, polarity }
    }
}

// ────────────────────────────────────────────────────────────
// 放入结果
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PutResult {
    Accepted,
    PolarityMismatch,
    Full(usize),
    OverflowTo(NestLevel),
}

// ────────────────────────────────────────────────────────────
// 照镜子结果
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MirrorResult {
    Recognized,
    Distorted(u32, u32),
    EmptyNest,
}

// ────────────────────────────────────────────────────────────
// 容器嵌套结构体
// ────────────────────────────────────────────────────────────

const MAX_CHILDREN: usize = 7;

#[derive(Clone, Debug)]
pub struct ContainerNest {
    pub level: NestLevel,
    pub max_children: usize,
    pub children: Vec<Box<ContainerNest>>,
    pub contents: Vec<Content>,
    pub policy: OverflowPolicy,
    pub anchor_id: u32,
    pub space_ratio: u32,
}

impl ContainerNest {
    // ── 构造 ─────────────────────────────────────────────────

    pub fn new(level: NestLevel, anchor_id: u32) -> Self {
        let cap = level.capacity_factor() as usize;
        let space = if level == NestLevel::YiYuan {
            100
        } else {
            100 / level.value() as u32
        };
        Self {
            level,
            max_children: cap.min(MAX_CHILDREN),
            children: Vec::new(),
            contents: Vec::new(),
            policy: OverflowPolicy::default(),
            anchor_id,
            space_ratio: space,
        }
    }

    pub fn with_policy(level: NestLevel, anchor_id: u32, policy: OverflowPolicy) -> Self {
        let mut nest = Self::new(level, anchor_id);
        nest.policy = policy;
        nest
    }

    /// 构建完整七元嵌套链 (一元袋为根 · 七元钵为最深子)
    pub fn build_correct_chain(anchor_id: u32, policy: OverflowPolicy) -> Self {
        Self::recursive_build(NestLevel::YiYuan, 1, anchor_id, policy)
    }

    fn recursive_build(
        level: NestLevel,
        depth: u8,
        anchor_base: u32,
        policy: OverflowPolicy,
    ) -> Self {
        let mut node = ContainerNest::with_policy(
            level,
            anchor_base.wrapping_add(depth as u32),
            policy,
        );
        if depth < 7 {
            let child =
                Self::recursive_build(level.inner(), depth.saturating_add(1), anchor_base, policy);
            node.children.push(Box::new(child));
        }
        node
    }

    // ── 放入内容 (put) ───────────────────────────────────────

    pub fn put(&mut self, content: Content) -> PutResult {
        if !self.level.polarity().accepts(content.polarity) {
            return PutResult::PolarityMismatch;
        }
        let used = self.contents.len().saturating_add(self.children.len());
        if used >= self.max_children {
            return match self.policy {
                OverflowPolicy::Fold => PutResult::Full(used),
                OverflowPolicy::Overflow => PutResult::OverflowTo(self.level.outer()),
            };
        }
        self.contents.push(content);
        PutResult::Accepted
    }

    pub fn insert_child(&mut self, child: ContainerNest) -> PutResult {
        let used = self.contents.len().saturating_add(self.children.len());
        if used >= self.max_children {
            return match self.policy {
                OverflowPolicy::Fold => PutResult::Full(used),
                OverflowPolicy::Overflow => PutResult::OverflowTo(self.level.outer()),
            };
        }
        self.children.push(Box::new(child));
        PutResult::Accepted
    }

    // ── 照镜子验证 (check) ───────────────────────────────────

    pub fn check(&self) -> MirrorResult {
        if self.contents.is_empty() && self.children.is_empty() {
            return MirrorResult::EmptyNest;
        }
        match self.find_innermost_content() {
            None => MirrorResult::EmptyNest,
            Some((_content, _depth)) => {
                MirrorResult::Recognized
            }
        }
    }

    fn find_innermost_content(&self) -> Option<(&Content, u32)> {
        if !self.children.is_empty() {
            if let Some(last_child) = self.children.last() {
                if let Some((content, _depth)) = last_child.find_innermost_content() {
                    return Some((content, self.level.value() as u32));
                }
            }
        }
        if let Some(last_content) = self.contents.last() {
            Some((last_content, self.level.value() as u32))
        } else {
            None
        }
    }

    // ── 溢出 (overflow) ──────────────────────────────────────

    pub fn overflow(&self, from_level: NestLevel) -> Option<NestLevel> {
        Some(from_level.outer())
    }

    // ── 递归打印 (调试用) ────────────────────────────────────

    pub fn render_tree(&self) -> String {
        self.render_indent(0)
    }

    fn render_indent(&self, depth: usize) -> String {
        let indent = "  ".repeat(depth);
        let mut s = format!(
            "{}[{}] {} (容量{} 内容{} 子{} 占比{}%)\n",
            indent,
            self.level.value(),
            self.level.container_name(),
            self.max_children,
            self.contents.len(),
            self.children.len(),
            self.space_ratio,
        );
        for content in &self.contents {
            s.push_str(&format!(
                "{}(内容 锚点{} 值{} 极性{})\n",
                indent,
                content.anchor_id,
                content.value,
                if content.polarity == ContainerPolarity::Yang { "阳/7" } else { "阴/6" },
            ));
        }
        for child in &self.children {
            s.push_str(&child.render_indent(depth + 1));
        }
        s
    }

    pub fn depth(&self) -> u32 {
        if self.children.is_empty() {
            1
        } else {
            1 + self.children.iter().map(|c| c.depth()).max().unwrap_or(0)
        }
    }

    pub fn total_contents(&self) -> usize {
        let own = self.contents.len();
        let child_sum: usize = self.children.iter().map(|c| c.total_contents()).sum();
        own.saturating_add(child_sum)
    }

    pub fn total_containers(&self) -> usize {
        1 + self.children.iter().map(|c| c.total_containers()).sum::<usize>()
    }
}

// ────────────────────────────────────────────────────────────
// 辅助函数
// ────────────────────────────────────────────────────────────

#[allow(dead_code)]
fn value_to_level(v: u8) -> NestLevel {
    match v {
        1 => NestLevel::YiYuan,
        2 => NestLevel::ErYuan,
        3 => NestLevel::SanYuan,
        4 => NestLevel::SiYuan,
        5 => NestLevel::WuYuan,
        6 => NestLevel::LiuYuan,
        _ => NestLevel::QiYuan,
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests_nest_level {
    use super::*;

    #[test]
    fn level_values() {
        assert_eq!(NestLevel::YiYuan.value(), 1);
        assert_eq!(NestLevel::ErYuan.value(), 2);
        assert_eq!(NestLevel::SanYuan.value(), 3);
        assert_eq!(NestLevel::SiYuan.value(), 4);
        assert_eq!(NestLevel::WuYuan.value(), 5);
        assert_eq!(NestLevel::LiuYuan.value(), 6);
        assert_eq!(NestLevel::QiYuan.value(), 7);
    }

    #[test]
    fn container_names() {
        assert_eq!(NestLevel::YiYuan.container_name(), "袋");
        assert_eq!(NestLevel::ErYuan.container_name(), "桶");
        assert_eq!(NestLevel::SanYuan.container_name(), "罐");
        assert_eq!(NestLevel::SiYuan.container_name(), "缸");
        assert_eq!(NestLevel::WuYuan.container_name(), "盆");
        assert_eq!(NestLevel::LiuYuan.container_name(), "盂");
        assert_eq!(NestLevel::QiYuan.container_name(), "钵");
    }

    #[test]
    fn level_inner_closure() {
        assert_eq!(NestLevel::QiYuan.inner(), NestLevel::YiYuan);
        assert_eq!(NestLevel::YiYuan.outer(), NestLevel::QiYuan);
        assert_eq!(NestLevel::ErYuan.inner(), NestLevel::SanYuan);
        assert_eq!(NestLevel::SanYuan.outer(), NestLevel::ErYuan);
    }

    #[test]
    fn capacity_factors() {
        assert_eq!(NestLevel::YiYuan.capacity_factor(), 7);
        assert_eq!(NestLevel::QiYuan.capacity_factor(), 1);
    }

    #[test]
    fn nesting_phrases() {
        assert_eq!(NestLevel::YiYuan.nesting_phrase(), "袋");
        assert_eq!(NestLevel::ErYuan.nesting_phrase(), "袋装桶");
        assert_eq!(NestLevel::SanYuan.nesting_phrase(), "袋装桶装罐");
        assert_eq!(
            NestLevel::QiYuan.nesting_phrase(),
            "袋装桶装罐-缸-盆-盂-钵[闭环]"
        );
    }
}

#[cfg(test)]
mod tests_container_polarity {
    use super::*;

    #[test]
    fn polarity_value_yang_is_7() {
        assert_eq!(ContainerPolarity::Yang.value(), 7);
    }

    #[test]
    fn polarity_value_yin_is_6() {
        assert_eq!(ContainerPolarity::Yin.value(), 6);
    }

    #[test]
    fn adjacent_attraction() {
        assert!(ContainerPolarity::Yang.accepts(ContainerPolarity::Yin));
        assert!(ContainerPolarity::Yin.accepts(ContainerPolarity::Yang));
        assert!(!ContainerPolarity::Yang.accepts(ContainerPolarity::Yang));
        assert!(!ContainerPolarity::Yin.accepts(ContainerPolarity::Yin));
    }

    #[test]
    fn polarity_opposite() {
        assert_eq!(ContainerPolarity::Yang.opposite(), ContainerPolarity::Yin);
        assert_eq!(ContainerPolarity::Yin.opposite(), ContainerPolarity::Yang);
    }

    #[test]
    fn level_polarity_alternation() {
        assert_eq!(NestLevel::YiYuan.polarity(), ContainerPolarity::Yang);
        assert_eq!(NestLevel::ErYuan.polarity(), ContainerPolarity::Yin);
        assert_eq!(NestLevel::SanYuan.polarity(), ContainerPolarity::Yang);
        assert_eq!(NestLevel::SiYuan.polarity(), ContainerPolarity::Yin);
        assert_eq!(NestLevel::WuYuan.polarity(), ContainerPolarity::Yang);
        assert_eq!(NestLevel::LiuYuan.polarity(), ContainerPolarity::Yin);
        assert_eq!(NestLevel::QiYuan.polarity(), ContainerPolarity::Yang);
    }
}

#[cfg(test)]
mod tests_container_nest {
    use super::*;

    #[test]
    fn new_container_has_correct_level() {
        let c = ContainerNest::new(NestLevel::YiYuan, 100);
        assert_eq!(c.level, NestLevel::YiYuan);
        assert_eq!(c.anchor_id, 100);
        assert_eq!(c.max_children, 7);
    }

    #[test]
    fn put_positive_polarity() {
        let mut bag = ContainerNest::new(NestLevel::YiYuan, 1);
        let content = Content::new(10, 42, ContainerPolarity::Yin);
        assert_eq!(bag.put(content), PutResult::Accepted);
        assert_eq!(bag.contents.len(), 1);
    }

    #[test]
    fn put_rejects_wrong_polarity() {
        let mut bag = ContainerNest::new(NestLevel::YiYuan, 1);
        let bad = Content::new(10, 42, ContainerPolarity::Yang);
        assert_eq!(bag.put(bad), PutResult::PolarityMismatch);
    }

    #[test]
    fn put_barrel_accepts_yang() {
        let mut barrel = ContainerNest::new(NestLevel::ErYuan, 2);
        let content = Content::new(20, 99, ContainerPolarity::Yang);
        assert_eq!(barrel.put(content), PutResult::Accepted);
    }

    #[test]
    fn overflow_fold_when_full() {
        let mut bag =
            ContainerNest::with_policy(NestLevel::YiYuan, 1, OverflowPolicy::Fold);
        for i in 0..7 {
            let r = bag.put(Content::new(i, i, ContainerPolarity::Yin));
            assert_eq!(r, PutResult::Accepted);
        }
        let overflow = bag.put(Content::new(99, 99, ContainerPolarity::Yin));
        assert_eq!(overflow, PutResult::Full(7));
    }

    #[test]
    fn put_accepted_returns_correct() {
        let mut bag = ContainerNest::new(NestLevel::YiYuan, 1);
        let r = bag.put(Content::new(1, 10, ContainerPolarity::Yin));
        assert_eq!(r, PutResult::Accepted);
    }
}

#[cfg(test)]
mod tests_full_chain {
    use super::*;

    #[test]
    fn build_correct_chain_depth() {
        let chain = ContainerNest::build_correct_chain(1000, OverflowPolicy::Fold);
        assert_eq!(chain.depth(), 7);
    }

    #[test]
    fn build_correct_chain_container_count() {
        let chain = ContainerNest::build_correct_chain(1, OverflowPolicy::Fold);
        assert_eq!(chain.total_containers(), 7);
    }

    #[test]
    fn chain_root_is_yiyuan() {
        let chain = ContainerNest::build_correct_chain(1, OverflowPolicy::Fold);
        assert_eq!(chain.level, NestLevel::YiYuan);
    }

    #[test]
    fn chain_leaf_is_qiyuan() {
        let chain = ContainerNest::build_correct_chain(1, OverflowPolicy::Fold);
        fn find_deepest(c: &ContainerNest) -> NestLevel {
            if c.children.is_empty() {
                c.level
            } else {
                find_deepest(&c.children[0])
            }
        }
        let deepest = find_deepest(&chain);
        assert_eq!(deepest, NestLevel::QiYuan);
    }
}

#[cfg(test)]
mod tests_mirror_check {
    use super::*;

    #[test]
    fn mirror_empty_nest() {
        let bag = ContainerNest::new(NestLevel::YiYuan, 50);
        assert_eq!(bag.check(), MirrorResult::EmptyNest);
    }

    #[test]
    fn mirror_single_content_recognized() {
        let mut bag = ContainerNest::new(NestLevel::YiYuan, 10);
        bag.put(Content::new(5, 7, ContainerPolarity::Yin));
        assert_eq!(bag.check(), MirrorResult::Recognized);
    }

    #[test]
    fn mirror_nested_recognized() {
        let mut chain =
            ContainerNest::build_correct_chain(100, OverflowPolicy::Fold);
        let r = chain.put(Content::new(999, 128, ContainerPolarity::Yin));
        assert_eq!(r, PutResult::Accepted);
        assert_eq!(chain.check(), MirrorResult::Recognized);
    }

    #[test]
    fn mirror_deep_nested_no_content() {
        let chain = ContainerNest::build_correct_chain(777, OverflowPolicy::Fold);
        assert_eq!(chain.check(), MirrorResult::EmptyNest);
    }
}

#[cfg(test)]
mod tests_overflow_path {
    use super::*;

    #[test]
    fn overflow_fold_policy() {
        let chain = ContainerNest::build_correct_chain(1, OverflowPolicy::Fold);
        assert_eq!(chain.policy, OverflowPolicy::Fold);
    }

    #[test]
    fn overflow_upward_policy() {
        let bag =
            ContainerNest::with_policy(NestLevel::YiYuan, 1, OverflowPolicy::Overflow);
        assert_eq!(bag.policy, OverflowPolicy::Overflow);
    }

    #[test]
    fn overflow_from_yiyuan_to_qiyuan() {
        let bag =
            ContainerNest::with_policy(NestLevel::YiYuan, 1, OverflowPolicy::Overflow);
        let target = bag.overflow(NestLevel::YiYuan);
        assert_eq!(target, Some(NestLevel::QiYuan));
    }

    #[test]
    fn overflow_chain_cycles_back() {
        let start = NestLevel::SanYuan;
        let mut lvl = start;
        for _ in 0..7 {
            let bag =
                ContainerNest::with_policy(lvl, 1, OverflowPolicy::Overflow);
            let outer = bag.overflow(lvl).unwrap();
            lvl = outer;
        }
        assert_eq!(lvl, start);
    }

    #[test]
    fn overflow_qiyuan_to_liuyuan() {
        let bag =
            ContainerNest::with_policy(NestLevel::QiYuan, 1, OverflowPolicy::Overflow);
        let target = bag.overflow(NestLevel::QiYuan);
        assert_eq!(target, Some(NestLevel::LiuYuan));
    }
}

#[cfg(test)]
mod tests_insertion_operations {
    use super::*;

    #[test]
    fn insert_child_into_container() {
        let mut bag = ContainerNest::new(NestLevel::YiYuan, 1);
        let barrel = ContainerNest::new(NestLevel::ErYuan, 2);
        assert_eq!(bag.insert_child(barrel), PutResult::Accepted);
        assert_eq!(bag.children.len(), 1);
    }

    #[test]
    fn insert_child_then_put_content() {
        let mut bag = ContainerNest::new(NestLevel::YiYuan, 1);
        let barrel = ContainerNest::new(NestLevel::ErYuan, 2);
        assert_eq!(bag.insert_child(barrel), PutResult::Accepted);
        if let Some(child) = bag.children.first_mut() {
            let r = child.put(Content::new(100, 50, ContainerPolarity::Yang));
            assert_eq!(r, PutResult::Accepted);
        }
    }

    #[test]
    fn render_tree_contains_all_levels() {
        let chain = ContainerNest::build_correct_chain(1, OverflowPolicy::Fold);
        let tree = chain.render_tree();
        assert!(tree.contains("袋"));
        assert!(tree.contains("桶"));
        assert!(tree.contains("罐"));
        assert!(tree.contains("缸"));
        assert!(tree.contains("盆"));
        assert!(tree.contains("盂"));
        assert!(tree.contains("钵"));
    }

    #[test]
    fn space_ratio_decreases_inward() {
        let chain = ContainerNest::build_correct_chain(1, OverflowPolicy::Fold);
        let ratio_outer = chain.space_ratio;
        if let Some(child) = chain.children.first() {
            let ratio_inner = child.space_ratio;
            assert!(ratio_outer > ratio_inner);
        } else {
            panic!("chain should have child");
        }
    }
}

#[cfg(test)]
mod tests_seven_level_completeness {
    use super::*;

    #[test]
    fn seven_steps_inner_returns_start() {
        let levels = [
            NestLevel::YiYuan,
            NestLevel::ErYuan,
            NestLevel::SanYuan,
            NestLevel::SiYuan,
            NestLevel::WuYuan,
            NestLevel::LiuYuan,
            NestLevel::QiYuan,
        ];
        for &start in &levels {
            let mut lvl = start;
            for _ in 0..7 {
                lvl = lvl.inner();
            }
            assert_eq!(lvl, start, "{:?} 经 7 次 inner 应回到原点", start);
        }
    }

    #[test]
    fn outer_is_inner_inverse() {
        let levels = [
            NestLevel::YiYuan,
            NestLevel::ErYuan,
            NestLevel::SanYuan,
            NestLevel::SiYuan,
            NestLevel::WuYuan,
            NestLevel::LiuYuan,
            NestLevel::QiYuan,
        ];
        for &lvl in &levels {
            assert_eq!(lvl.inner().outer(), lvl);
        }
    }

    #[test]
    fn each_level_accepts_alternating_polarity() {
        let levels = [
            (NestLevel::YiYuan, ContainerPolarity::Yin),
            (NestLevel::ErYuan, ContainerPolarity::Yang),
            (NestLevel::SanYuan, ContainerPolarity::Yin),
            (NestLevel::SiYuan, ContainerPolarity::Yang),
            (NestLevel::WuYuan, ContainerPolarity::Yin),
            (NestLevel::LiuYuan, ContainerPolarity::Yang),
            (NestLevel::QiYuan, ContainerPolarity::Yin),
        ];
        for (level, acceptable) in levels {
            let mut c = ContainerNest::new(level, 1);
            let r = c.put(Content::new(1, 1, acceptable));
            assert_eq!(r, PutResult::Accepted,);
        }
    }

    #[test]
    fn each_level_rejects_same_polarity() {
        let levels = [
            (NestLevel::YiYuan, ContainerPolarity::Yang),
            (NestLevel::ErYuan, ContainerPolarity::Yin),
            (NestLevel::SanYuan, ContainerPolarity::Yang),
            (NestLevel::SiYuan, ContainerPolarity::Yin),
            (NestLevel::WuYuan, ContainerPolarity::Yang),
            (NestLevel::LiuYuan, ContainerPolarity::Yin),
            (NestLevel::QiYuan, ContainerPolarity::Yang),
        ];
        for (level, rejected) in levels {
            let mut c = ContainerNest::new(level, 1);
            let r = c.put(Content::new(1, 1, rejected));
            assert_eq!(r, PutResult::PolarityMismatch);
        }
    }

    #[test]
    fn value_to_level_mapping() {
        assert_eq!(value_to_level(1), NestLevel::YiYuan);
        assert_eq!(value_to_level(2), NestLevel::ErYuan);
        assert_eq!(value_to_level(3), NestLevel::SanYuan);
        assert_eq!(value_to_level(4), NestLevel::SiYuan);
        assert_eq!(value_to_level(5), NestLevel::WuYuan);
        assert_eq!(value_to_level(6), NestLevel::LiuYuan);
        assert_eq!(value_to_level(7), NestLevel::QiYuan);
        assert_eq!(value_to_level(8), NestLevel::QiYuan);
        assert_eq!(value_to_level(255), NestLevel::QiYuan);
    }
}
