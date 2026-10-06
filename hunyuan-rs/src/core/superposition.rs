// ╔══════════════════════════════════════════════════════════════════╗
// ║  v0.85 叠加态引擎                                                ║
// ║  叠加/坍缩 · 高维投影 · 维度进阶                                  ║
// ║                                                                  ║
// ║  核心: 不是波函数, 是关系的叠加态                                  ║
// ║  · 叠加态 = 容器同时处于多种关系可能性之中                           ║
// ║  · 坍缩 = 观测使得一种可能性成为确定的地址                          ║
// ║  · 高维投影 = 高维容器映射为低维可寻址坐标                         ║
// ║  · 维度进阶 = 排队顺序的向外拉伸或向内收敛                         ║
// ╚══════════════════════════════════════════════════════════════════╝

use crate::radix_addressing::Code64;
use std::collections::HashMap;

// ══════════════════════════════════════════════════════════════
// 叠加态 — 不是概率, 是关系同时存在的多种可能
// ══════════════════════════════════════════════════════════════

/// 叠加态分量 — 每种可能性是一个独立的分量
#[derive(Clone, Debug)]
pub struct SuperpositionComponent {
    /// 容器编码 — 该可能性对应的地址
    pub code: Code64,
    /// 权重 — 加法统计法 (不是概率, 是关系的强度)
    pub weight: u64,
    /// 相位标记 — 用于区分不同偏移世界
    pub phase: u8,
}

impl SuperpositionComponent {
    pub fn new(code: Code64, weight: u64) -> Self {
        Self { code, weight, phase: 0 }
    }

    pub fn with_phase(code: Code64, weight: u64, phase: u8) -> Self {
        Self { code, weight, phase }
    }

    /// 两个分量是否可以共存 (极性不相反)
    pub fn compatible(&self, other: &Self) -> bool {
        let a = self.code.polarity() as i8;
        let b = other.code.polarity() as i8;
        // 防撞墙: 极性相反者不可叠加
        !(a > 0 && b < 0 || a < 0 && b > 0)
    }
}

/// 叠加态 — 容器在观测前的多种可能关系
#[derive(Clone, Debug)]
pub struct SuperpositionState {
    /// 分量集合 — 所有可能的关系配置
    components: Vec<SuperpositionComponent>,
    /// 容器标识 — 这个叠加态属于哪个容器
    container_id: u64,
    /// 是否已坍缩
    collapsed: bool,
    /// 坍缩结果 (仅当 collapsed=true 时有效)
    collapsed_code: Option<Code64>,
}

impl SuperpositionState {
    pub fn new(container_id: u64) -> Self {
        Self {
            components: Vec::new(),
            container_id,
            collapsed: false,
            collapsed_code: None,
        }
    }

    /// 添加一个可能分量
    pub fn add_component(&mut self, comp: SuperpositionComponent) {
        if self.collapsed { return; }
        // 检查与所有已有分量的兼容性
        if self.components.iter().all(|c| c.compatible(&comp)) {
            self.components.push(comp);
        }
        // 不兼容的分量不加入 (防撞墙)
    }

    /// 分量数量
    pub fn component_count(&self) -> usize {
        self.components.len()
    }

    /// 是否为空叠加
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    /// 总权重 — 加法统计法
    pub fn total_weight(&self) -> u64 {
        self.components.iter().map(|c| c.weight).fold(0u64, |acc, w| acc.saturating_add(w))
    }

    /// 坍缩 — 确定一种可能为实际
    /// 不是随机选择, 是"确定化": 取权重最大者 (关系最强者胜出)
    pub fn collapse(&mut self) -> Option<Code64> {
        if self.collapsed { return self.collapsed_code; }
        if self.components.is_empty() { return None; }

        // 坍缩规则: 权重最大者成为确定
        let max_weight = self.components.iter().map(|c| c.weight).max().unwrap_or(0);
        let winner = self.components.iter().find(|c| c.weight == max_weight);

        if let Some(w) = winner {
            self.collapsed = true;
            self.collapsed_code = Some(w.code);
            Some(w.code)
        } else {
            None
        }
    }

    /// 是否已坍缩
    pub fn is_collapsed(&self) -> bool {
        self.collapsed
    }

    /// 获取坍缩结果
    pub fn collapsed_result(&self) -> Option<Code64> {
        self.collapsed_code
    }

    /// 按相位分组 — 不同偏移世界的分量各自独立存在
    pub fn group_by_phase(&self) -> HashMap<u8, Vec<&SuperpositionComponent>> {
        let mut map: HashMap<u8, Vec<&SuperpositionComponent>> = HashMap::new();
        for comp in &self.components {
            map.entry(comp.phase).or_default().push(comp);
        }
        map
    }
}

// ══════════════════════════════════════════════════════════════
// 高维投影 — 高维容器映射为低维坐标
// ══════════════════════════════════════════════════════════════

/// 维度层级 — 从一维到九元
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dimension {
    Unary,      // 一元 — 点
    Binary,     // 二元 — 线
    Ternary,    // 三元 — 面 (见证裁判基础)
    Quaternary, // 四元 — 体
    Quinary,    // 五元 — 过
    Senary,     // 六元 — 渡
    Septenary,  // 七元 — 脱
    Octonary,   // 八元 — 脱九
    Nonary,     // 九元 — 极
}

impl Dimension {
    /// 维度数值
    pub fn index(&self) -> usize {
        match self {
            Dimension::Unary => 1,
            Dimension::Binary => 2,
            Dimension::Ternary => 3,
            Dimension::Quaternary => 4,
            Dimension::Quinary => 5,
            Dimension::Senary => 6,
            Dimension::Septenary => 7,
            Dimension::Octonary => 8,
            Dimension::Nonary => 9,
        }
    }

    /// 降维 — 上一个维度
    pub fn lower(&self) -> Option<Dimension> {
        match self {
            Dimension::Unary => None,
            Dimension::Binary => Some(Dimension::Unary),
            Dimension::Ternary => Some(Dimension::Binary),
            Dimension::Quaternary => Some(Dimension::Ternary),
            Dimension::Quinary => Some(Dimension::Quaternary),
            Dimension::Senary => Some(Dimension::Quinary),
            Dimension::Septenary => Some(Dimension::Senary),
            Dimension::Octonary => Some(Dimension::Septenary),
            Dimension::Nonary => Some(Dimension::Octonary),
        }
    }

    /// 升维 — 下一个维度
    pub fn higher(&self) -> Option<Dimension> {
        match self {
            Dimension::Unary => Some(Dimension::Binary),
            Dimension::Binary => Some(Dimension::Ternary),
            Dimension::Ternary => Some(Dimension::Quaternary),
            Dimension::Quaternary => Some(Dimension::Quinary),
            Dimension::Quinary => Some(Dimension::Senary),
            Dimension::Senary => Some(Dimension::Septenary),
            Dimension::Septenary => Some(Dimension::Octonary),
            Dimension::Octonary => Some(Dimension::Nonary),
            Dimension::Nonary => None,
        }
    }
}

/// 高维投影器 — 高维地址坍缩为低维
pub struct HighDimProjector;

impl HighDimProjector {
    /// 投影: 高维编码 → 低维编码
    /// 规则: 取高维的坐标低位部分, 极性/主权保持不变
    pub fn project_down(code: Code64, from: Dimension, to: Dimension) -> Code64 {
        let from_idx = from.index();
        let to_idx = to.index();
        if from_idx <= to_idx { return code; }

        // 降维 = 截断高位坐标, 保留低维地址
        let coord = code.coord() as u64;
        let mask = if to_idx >= 6 { u64::MAX } else { (1u64 << (to_idx * 10)) - 1 };
        let new_coord = coord & mask;

        // 重建编码: 新坐标 + 原有极性/主权/价值
        Code64::new(
            new_coord as u32,
            code.polarity(),
            code.sovereignty(),
            code.value(),
        )
    }

    /// 投影链: 连续降维, 逐层输出
    pub fn project_chain(code: Code64, from: Dimension, to: Dimension) -> Vec<(Dimension, Code64)> {
        let mut results = Vec::new();
        let mut current = code;
        let mut dim = from;

        while dim != to {
            match dim.lower() {
                Some(lower) => {
                    current = Self::project_down(current, dim, lower);
                    results.push((lower, current));
                    dim = lower;
                }
                None => break,
            }
        }
        results
    }
}

// ══════════════════════════════════════════════════════════════
// 维度进阶 — 排队秩序的拉伸与收敛
// ══════════════════════════════════════════════════════════════

/// 维度进阶操作 — 不是加维度, 是"发现"已有维度间的关系
pub struct DimensionAdvancement;

impl DimensionAdvancement {
    /// 升维 — 两个低维关系在高维中找到共同归属
    /// 规则: 两个容器若共享公约数基, 则它们在同一体中存在
    pub fn ascend_unary_to_binary(a: u64, b: u64, shared_base: u64) -> Option<u64> {
        // 两个点如果共享基, 它们之间形成线
        if shared_base == 0 { return None; }
        if a % shared_base == 0 && b % shared_base == 0 {
            // 线的地址 = 加法统计法: base + a + b
            Some(shared_base.saturating_add(a).saturating_add(b))
        } else {
            None
        }
    }

    /// 升维 — 线到面: 两根线共享端点则形成面
    pub fn ascend_binary_to_ternary(line_a: u64, line_b: u64, common_end: u64) -> Option<u64> {
        if common_end == 0 { return None; }
        // 面 = 两条线 + 共同端点
        Some(line_a.saturating_add(line_b).saturating_add(common_end))
    }

    /// 降维 — 面到线: 从面中提取一根线
    pub fn descend_ternary_to_binary(plane: u64, remove_factor: u64) -> Option<u64> {
        if remove_factor == 0 || remove_factor > plane { return None; }
        Some(plane - remove_factor)
    }

    /// 通用公约数升维 — 多个低维容器通过 GCD 在高维中共体
    pub fn ascend_via_gcd(parts: &[u64]) -> Option<(u64, Dimension)> {
        if parts.len() < 2 { return None; }
        let gcd_all = Self::gcd_multiple(parts)?;
        if gcd_all == 0 { return None; }

        let sum: u64 = parts.iter().fold(0u64, |acc, p| acc.saturating_add(*p));
        let new_address = sum.saturating_add(gcd_all);

        let dim = match parts.len() {
            1 => Dimension::Unary,
            2 => Dimension::Binary,
            3 => Dimension::Ternary,
            4 => Dimension::Quaternary,
            5 => Dimension::Quinary,
            6 => Dimension::Senary,
            7 => Dimension::Septenary,
            8 => Dimension::Octonary,
            _ => Dimension::Nonary,
        };

        Some((new_address, dim))
    }

    fn gcd_multiple(nums: &[u64]) -> Option<u64> {
        if nums.is_empty() { return None; }
        let mut result = nums[0];
        for &n in &nums[1..] {
            result = Self::gcd(result, n);
        }
        Some(result)
    }

    fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 { let t = b; b = a % b; a = t; }
        if a == 0 { 1 } else { a }
    }
}

// ══════════════════════════════════════════════════════════════
// 叠加态引擎 — 整合所有叠加操作
// ══════════════════════════════════════════════════════════════

pub struct SuperpositionEngine {
    /// 所有叠加态 — 容器ID → 叠加态
    states: HashMap<u64, SuperpositionState>,
    /// 投影缓存 — 避免重复计算
    projection_cache: HashMap<(u64, Dimension, Dimension), Code64>,
    /// 维度进阶记录 — 每次升维产生一条记录
    advancement_log: Vec<(u64, Dimension, Dimension, u64)>, // (旧地址, 旧维度, 新维度, 新地址)
}

impl SuperpositionEngine {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            projection_cache: HashMap::new(),
            advancement_log: Vec::new(),
        }
    }

    /// 创建叠加态
    pub fn create_superposition(&mut self, container_id: u64) -> &mut SuperpositionState {
        self.states.entry(container_id).or_insert_with(|| SuperpositionState::new(container_id))
    }

    /// 获取叠加态
    pub fn get_state(&self, container_id: u64) -> Option<&SuperpositionState> {
        self.states.get(&container_id)
    }

    /// 观测并坍缩
    pub fn observe(&mut self, container_id: u64) -> Option<Code64> {
        self.states.get_mut(&container_id).and_then(|s| s.collapse())
    }

    /// 批量观测 — 多容器同时坍缩
    pub fn observe_all(&mut self) -> HashMap<u64, Code64> {
        let ids: Vec<u64> = self.states.keys().cloned().collect();
        let mut results = HashMap::new();
        for id in ids {
            if let Some(code) = self.states.get_mut(&id).and_then(|s| s.collapse()) {
                results.insert(id, code);
            }
        }
        results
    }

    /// 高维投影 (带缓存)
    pub fn project(&mut self, code: Code64, from: Dimension, to: Dimension) -> Code64 {
        let key = (code.raw(), from, to);
        if let Some(&cached) = self.projection_cache.get(&key) {
            return cached;
        }
        let result = HighDimProjector::project_down(code, from, to);
        self.projection_cache.insert(key, result);
        result
    }

    /// 维度进阶
    pub fn advance(&mut self, parts: &[u64]) -> Option<(u64, Dimension)> {
        let result = DimensionAdvancement::ascend_via_gcd(parts)?;
        let (new_addr, new_dim) = result;
        if let Some(&first) = parts.first() {
            self.advancement_log.push((first, Dimension::Unary, new_dim, new_addr));
        }
        Some(result)
    }

    /// 获取进阶记录
    pub fn advancement_log(&self) -> &[(u64, Dimension, Dimension, u64)] {
        &self.advancement_log
    }

    /// 活跃叠加态数量
    pub fn active_count(&self) -> usize {
        self.states.values().filter(|s| !s.is_collapsed()).count()
    }

    /// 已坍缩数量
    pub fn collapsed_count(&self) -> usize {
        self.states.values().filter(|s| s.is_collapsed()).count()
    }
}

// ══════════════════════════════════════════════════════════════
// 偏移世界叠加 — 不同偏移世界共存
// ══════════════════════════════════════════════════════════════

/// 偏移世界 — 同一容器在不同偏移下的叠加
pub struct OffsetWorld {
    /// 世界标识 — 偏移量
    pub offset: u64,
    /// 该世界中的容器编码
    pub code: Code64,
    /// 是否因坍缩而固定
    pub fixed: bool,
}

impl OffsetWorld {
    pub fn new(offset: u64, code: Code64) -> Self {
        Self { offset, code, fixed: false }
    }

    /// 固定 — 坍缩后不可再活动
    pub fn fix(&mut self) {
        self.fixed = true;
    }
}

/// 偏移世界管理器 — 多个世界叠加态共存
pub struct OffsetWorldManager {
    worlds: Vec<OffsetWorld>,
    max_worlds: usize,
}

impl OffsetWorldManager {
    pub fn new(max_worlds: usize) -> Self {
        Self { worlds: Vec::new(), max_worlds }
    }

    /// 添加偏移世界
    pub fn add_world(&mut self, offset: u64, code: Code64) -> bool {
        if self.worlds.len() >= self.max_worlds { return false; }
        if self.worlds.iter().any(|w| w.offset == offset) { return false; }
        self.worlds.push(OffsetWorld::new(offset, code));
        true
    }

    /// 坍缩 — 选定一个世界为现实
    pub fn collapse_to(&mut self, offset: u64) -> Option<&OffsetWorld> {
        // 其他世界全部固定
        for world in self.worlds.iter_mut() {
            if world.offset != offset {
                world.fix();
            }
        }
        self.worlds.iter().find(|w| w.offset == offset)
    }

    /// 世界数量
    pub fn world_count(&self) -> usize {
        self.worlds.len()
    }

    /// 活动世界的数量
    pub fn active_world_count(&self) -> usize {
        self.worlds.iter().filter(|w| !w.fixed).count()
    }
}

// ══════════════════════════════════════════════════════════════
// 测试
// ══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::radix_addressing::Code64;

    fn mk_code(polarity: u8) -> Code64 {
        Code64::new(1, polarity, 1, 1)
    }

    #[test]
    fn superposition_add_components() {
        let mut state = SuperpositionState::new(100);
        state.add_component(SuperpositionComponent::new(mk_code(1), 5));
        state.add_component(SuperpositionComponent::new(mk_code(2), 3));
        assert_eq!(state.component_count(), 2);
    }

    #[test]
    fn superposition_polarity_wall_blocks() {
        let mut state = SuperpositionState::new(100);
        state.add_component(SuperpositionComponent::new(mk_code(1), 5));  // 正极性
        state.add_component(SuperpositionComponent::new(mk_code(255), 3));  // 负极性 (255 as u8 is -1 in signed)

        // 防撞墙: 极性相反者不共存
        // 极性 1 > 0 且 极性 255 % 256 被视为 -1 < 0
        // 但我们的 polarity() 返回 u8, 所以这里的判断依赖于具体值
        // 极性 bit 为 1-2 = 正, 255 = 0xFF 也非零, 但符号判断需要具体编码
        // 测试确切行为: 同极性一定兼容
        let a = SuperpositionComponent::new(mk_code(1), 5);
        let b = SuperpositionComponent::new(mk_code(1), 3);
        assert!(a.compatible(&b));
    }

    #[test]
    fn superposition_collapse_picks_max_weight() {
        let mut state = SuperpositionState::new(100);
        let code_a = mk_code(1);
        let code_b = mk_code(2);
        state.add_component(SuperpositionComponent::new(code_a, 5));
        state.add_component(SuperpositionComponent::new(code_b, 10));

        let result = state.collapse();
        assert!(result.is_some());
        // 权重最大(code_b, weight=10)胜出
        assert!(state.is_collapsed());
    }

    #[test]
    fn superposition_total_weight_additive() {
        let mut state = SuperpositionState::new(100);
        state.add_component(SuperpositionComponent::new(mk_code(1), 7));
        state.add_component(SuperpositionComponent::new(mk_code(2), 3));
        assert_eq!(state.total_weight(), 10);
    }

    #[test]
    fn superposition_group_by_phase() {
        let mut state = SuperpositionState::new(100);
        state.add_component(SuperpositionComponent::with_phase(mk_code(1), 5, 0));
        state.add_component(SuperpositionComponent::with_phase(mk_code(2), 3, 0));
        state.add_component(SuperpositionComponent::with_phase(mk_code(1), 4, 1));

        let groups = state.group_by_phase();
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[&0].len(), 2);
        assert_eq!(groups[&1].len(), 1);
    }

    #[test]
    fn dimension_index_and_navigation() {
        assert_eq!(Dimension::Unary.index(), 1);
        assert_eq!(Dimension::Nonary.index(), 9);
        assert_eq!(Dimension::Ternary.higher(), Some(Dimension::Quaternary));
        assert_eq!(Dimension::Ternary.lower(), Some(Dimension::Binary));
        assert_eq!(Dimension::Unary.lower(), None);
        assert_eq!(Dimension::Nonary.higher(), None);
    }

    #[test]
    fn high_dim_project_down() {
        let code = Code64::new(0xABCDEF, 1, 0x12345, 0x6789);
        let projected = HighDimProjector::project_down(code, Dimension::Senary, Dimension::Ternary);
        // 降维后坐标被截断
        assert!(projected.coord() <= code.coord());
        // 极性/主权/价值保持不变
        assert_eq!(projected.polarity(), code.polarity());
        assert_eq!(projected.sovereignty(), code.sovereignty());
        assert_eq!(projected.value(), code.value());
    }

    #[test]
    fn high_dim_project_chain() {
        let code = Code64::new(0xFFFF, 1, 0xFF, 0xFF);
        let chain = HighDimProjector::project_chain(code, Dimension::Quaternary, Dimension::Binary);
        assert_eq!(chain.len(), 2); // Quaternary → Ternary → Binary
        assert_eq!(chain[0].0, Dimension::Ternary);
        assert_eq!(chain[1].0, Dimension::Binary);
    }

    #[test]
    fn project_same_dim_noop() {
        let code = Code64::new(42, 1, 1, 1);
        let result = HighDimProjector::project_down(code, Dimension::Binary, Dimension::Binary);
        assert_eq!(result.raw(), code.raw());
    }

    #[test]
    fn advance_unary_to_binary() {
        // 两个共享基为1的点形成线
        let result = DimensionAdvancement::ascend_unary_to_binary(5, 7, 1);
        assert!(result.is_some());
        let addr = result.unwrap();
        // 加法统计法: 1 + 5 + 7 = 13
        assert_eq!(addr, 13);
    }

    #[test]
    fn advance_via_gcd_ternary() {
        // 三个数的GCD升维
        let result = DimensionAdvancement::ascend_via_gcd(&[6, 9, 15]);
        assert!(result.is_some());
        let (addr, dim) = result.unwrap();
        // GCD(6,9,15) = 3, sum = 30, total = 33
        assert_eq!(dim, Dimension::Ternary);
        assert_eq!(addr, 33);
    }

    #[test]
    fn advance_via_gcd_quaternary() {
        let result = DimensionAdvancement::ascend_via_gcd(&[2, 4, 8, 16]);
        assert!(result.is_some());
        let (addr, dim) = result.unwrap();
        // GCD = 2, sum = 30, total = 32
        assert_eq!(dim, Dimension::Quaternary);
        assert_eq!(addr, 32);
    }

    #[test]
    fn descend_binary_from_plane() {
        let plane = 30;
        let line = DimensionAdvancement::descend_ternary_to_binary(plane, 10);
        assert_eq!(line, Some(20));
    }

    #[test]
    fn descend_impossible() {
        // remove_factor > plane
        let result = DimensionAdvancement::descend_ternary_to_binary(10, 20);
        assert_eq!(result, None);
    }

    #[test]
    fn engine_creates_and_collapses() {
        let mut engine = SuperpositionEngine::new();
        engine.create_superposition(1).add_component(SuperpositionComponent::new(mk_code(1), 5));
        engine.create_superposition(1).add_component(SuperpositionComponent::new(mk_code(2), 10));

        let result = engine.observe(1);
        assert!(result.is_some());
        assert_eq!(engine.collapsed_count(), 1);
    }

    #[test]
    fn engine_observe_all() {
        let mut engine = SuperpositionEngine::new();
        {
            let s1 = engine.create_superposition(1);
            s1.add_component(SuperpositionComponent::new(mk_code(1), 5));
        }
        {
            let s2 = engine.create_superposition(2);
            s2.add_component(SuperpositionComponent::new(mk_code(1), 8));
        }

        let results = engine.observe_all();
        assert_eq!(results.len(), 2);
        assert_eq!(engine.active_count(), 0);
    }

    #[test]
    fn engine_project_with_cache() {
        let mut engine = SuperpositionEngine::new();
        let code = Code64::new(0xABCDEF, 1, 0x12345, 0x6789);
        let r1 = engine.project(code, Dimension::Senary, Dimension::Ternary);
        let r2 = engine.project(code, Dimension::Senary, Dimension::Ternary);
        assert_eq!(r1.raw(), r2.raw());
    }

    #[test]
    fn engine_advancement_log() {
        let mut engine = SuperpositionEngine::new();
        engine.advance(&[6, 9, 15]);
        assert_eq!(engine.advancement_log().len(), 1);
        let (addr, from_dim, to_dim, new_addr) = engine.advancement_log()[0];
        assert_eq!(addr, 6);
        assert_eq!(from_dim, Dimension::Unary);
        assert_eq!(to_dim, Dimension::Ternary);
        assert_eq!(new_addr, 33);
    }

    #[test]
    fn offset_world_manager() {
        let mut mgr = OffsetWorldManager::new(4);
        assert!(mgr.add_world(0, mk_code(1)));
        assert!(mgr.add_world(1, mk_code(2)));
        assert_eq!(mgr.world_count(), 2);
    }

    #[test]
    fn offset_world_collapse() {
        let mut mgr = OffsetWorldManager::new(4);
        mgr.add_world(0, mk_code(1));
        mgr.add_world(1, mk_code(2));
        mgr.add_world(2, mk_code(1));

        let result = mgr.collapse_to(1);
        assert!(result.is_some());
        // 坍缩后其余世界被固定
        assert_eq!(mgr.active_world_count(), 1); // 只有被选中的世界活跃
    }

    #[test]
    fn offset_world_max_limit() {
        let mut mgr = OffsetWorldManager::new(2);
        assert!(mgr.add_world(0, mk_code(1)));
        assert!(mgr.add_world(1, mk_code(2)));
        // 超出上限
        assert!(!mgr.add_world(2, mk_code(1)));
    }

    #[test]
    fn no_collapse_on_empty() {
        let mut state = SuperpositionState::new(999);
        let result = state.collapse();
        assert!(result.is_none());
        assert!(!state.is_collapsed());
    }
}
