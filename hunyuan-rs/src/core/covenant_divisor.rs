// ╔══════════════════════════════════════════════════════════════════╗
// ║  v0.84 公约数引擎                                                ║
// ║  公约数编程 · 产权流 · 同体空间 · 迂回封装 · 加法统计法           ║
// ║                                                                  ║
// ║  核心: 公约 = 固定标题 + 基准值; 排队者在其下列队; 总和 = 基准 + 排队 ║
// ║  · 公约数编程: 通过建立公约数来组织关系                          ║
// ║  · 产权流: 产权沿关系流动, 通过公倍数扩散                        ║
// ║  · 同体空间: 多个容器共享同一身体                                ║
// ║  · 迂回封装: 信号绕行多容器后到达目标                            ║
// ╚══════════════════════════════════════════════════════════════════╝

use std::collections::HashMap;
use crate::container_ir::{ContainerKind, ContainerRef};

/// 公约 — 固定标题 + 基准值, 排队者在其下列队
#[derive(Clone, Debug)]
pub struct Covenant {
    /// 公约标题 (固定)
    pub title: String,
    /// 公约基准值
    pub base_value: u64,
    /// 排队者值列表 (顺序 = 优先级)
    pub queue: Vec<u64>,
    /// 总和 (缓存: 基准 + 排队者之和)
    total: u64,
}

impl Covenant {
    pub fn new(title: &str, base_value: u64) -> Self {
        Covenant {
            title: title.to_string(),
            base_value,
            queue: Vec::new(),
            total: base_value,
        }
    }

    /// 排队 (追加新值, 总和自动更新)
    pub fn enqueue(&mut self, value: u64) {
        self.queue.push(value);
        self.total = self.base_value + self.queue.iter().sum::<u64>();
    }

    /// 总和 = 基准 + 排队者值之和
    pub fn total(&self) -> u64 {
        self.total
    }

    /// 排队者数量
    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }

    /// 是否有效 (基准 > 0 或至少有一个排队者)
    pub fn is_active(&self) -> bool {
        self.base_value > 0 || !self.queue.is_empty()
    }

    /// 排队者均值
    pub fn queue_average(&self) -> f64 {
        if self.queue.is_empty() {
            return 0.0;
        }
        self.queue.iter().sum::<u64>() as f64 / self.queue.len() as f64
    }

    /// 取出首个排队者
    pub fn dequeue(&mut self) -> Option<u64> {
        if self.queue.is_empty() {
            return None;
        }
        let val = self.queue.remove(0);
        self.total = self.base_value + self.queue.iter().sum::<u64>();
        Some(val)
    }
}

/// 公倍数 — 从基数向外扩张的边界
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CommonMultiple {
    /// 基数
    pub base: u64,
    /// 倍数
    pub factor: u64,
}

impl CommonMultiple {
    pub fn new(base: u64, factor: u64) -> Self {
        CommonMultiple { base, factor: factor.max(1) }
    }

    /// 当前值 = base * factor
    pub fn value(&self) -> u64 {
        self.base * self.factor
    }

    /// 扩张 (倍数 +1)
    pub fn expand(&mut self) {
        self.factor += 1;
    }

    /// 是否能被 base 整除
    pub fn contains(&self, val: u64) -> bool {
        val % self.base == 0
    }
}

/// 产权流 — 沿容器关系扩散的所有权
#[derive(Clone, Debug)]
pub struct PropertyFlow {
    /// 源持有者
    pub source: ContainerRef,
    /// 当前边界 (公倍数)
    pub boundary: CommonMultiple,
    /// 已扩散到的容器
    pub reached: Vec<ContainerRef>,
    /// 扩散深度
    pub depth: u32,
}

impl PropertyFlow {
    pub fn new(source: ContainerRef, base_value: u64) -> Self {
        PropertyFlow {
            source,
            boundary: CommonMultiple::new(base_value, 1),
            reached: Vec::new(),
            depth: 0,
        }
    }

    /// 向外扩散一步
    pub fn diffuse(&mut self) {
        self.boundary.expand();
        self.depth += 1;
    }

    /// 记录已到达
    pub fn mark_reached(&mut self, container: ContainerRef) {
        if !self.reached.contains(&container) {
            self.reached.push(container);
        }
    }

    /// 扩散范围
    pub fn range(&self) -> u64 {
        self.boundary.value()
    }

    /// 已到达数量
    pub fn reached_count(&self) -> usize {
        self.reached.len()
    }
}

/// 迂回路径 — 信号绕行的封装
#[derive(Clone, Debug)]
pub struct DetourPath {
    /// 起点
    pub origin: ContainerRef,
    /// 终点
    pub destination: ContainerRef,
    /// 中间路径 (容器顺序)
    pub hops: Vec<ContainerRef>,
    /// 路径权重 = 1/(1+跳数)
    pub weight: f64,
}

impl DetourPath {
    pub fn new(origin: ContainerRef, destination: ContainerRef) -> Self {
        DetourPath {
            origin,
            destination,
            hops: Vec::new(),
            weight: 1.0,
        }
    }

    /// 添加一跳
    pub fn add_hop(&mut self, hop: ContainerRef) {
        self.hops.push(hop);
        self.recalc_weight();
    }

    /// 重新计算权重
    fn recalc_weight(&mut self) {
        let jumps = self.hops.len() as u64 + 1; // +1 for final hop
        self.weight = 1.0 / (1.0 + jumps as f64);
    }

    /// 跳数
    pub fn jump_count(&self) -> usize {
        self.hops.len() + 1
    }

    /// 路径长度 (考虑权重)
    pub fn effective_length(&self) -> f64 {
        self.jump_count() as f64 / self.weight
    }

    /// 是否有效 (非空路径)
    pub fn is_viable(&self) -> bool {
        self.weight > 0.01
    }
}

/// 同体空间 — 多个容器共享同一身体
#[derive(Clone, Debug)]
pub struct CoBodySpace {
    /// 空间标识
    pub space_id: u64,
    /// 成员容器
    pub members: Vec<ContainerRef>,
    /// 共享公倍数基
    pub shared_base: u64,
    /// 体内关系: (a,b) → 关系强度
    pub relations: HashMap<(u64, u64), f64>,
}

impl CoBodySpace {
    pub fn new(space_id: u64, shared_base: u64) -> Self {
        CoBodySpace {
            space_id,
            members: Vec::new(),
            shared_base,
            relations: HashMap::new(),
        }
    }

    /// 加入空间
    pub fn join(&mut self, member: ContainerRef) {
        if !self.members.contains(&member) {
            // 与所有现有成员建立关系
            for existing in &self.members {
                let strength = self.compute_strength(existing.entity(), member.entity());
                self.relations.insert((existing.entity(), member.entity()), strength);
            }
            self.members.push(member);
        }
    }

    /// 计算两容器在同体空间中的关系强度
    fn compute_strength(&self, a: u64, b: u64) -> f64 {
        let g = gcd64(a, b) as f64;
        if self.shared_base == 0 {
            return g;
        }
        if g >= self.shared_base as f64 {
            g / self.shared_base as f64
        } else {
            0.0
        }
    }

    /// 离开空间
    pub fn leave(&mut self, member: &ContainerRef) -> bool {
        if let Some(pos) = self.members.iter().position(|m| m == member) {
            // 移除关系
            self.relations.retain(|(a, _), _| *a != member.entity());
            self.members.remove(pos);
            true
        } else {
            false
        }
    }

    /// 空间内是否存在关系
    pub fn has_relation(&self, a: u64, b: u64) -> bool {
        self.relations.contains_key(&(a, b)) || self.relations.contains_key(&(b, a))
    }

    /// 关系强度
    pub fn relation_strength(&self, a: u64, b: u64) -> f64 {
        self.relations
            .get(&(a, b))
            .or_else(|| self.relations.get(&(b, a)))
            .copied()
            .unwrap_or(0.0)
    }

    /// 成员数
    pub fn member_count(&self) -> usize {
        self.members.len()
    }
}

/// 公约数引擎 — 整合公约 · 产权流 · 迂回路径 · 同体空间
pub struct CovenantDivisorEngine {
    /// 公约表
    pub covenants: HashMap<String, Covenant>,
    /// 产权流表
    pub property_flows: Vec<PropertyFlow>,
    /// 迂回路径表
    pub detours: Vec<DetourPath>,
    /// 同体空间表
    pub cobodies: HashMap<u64, CoBodySpace>,
}

impl CovenantDivisorEngine {
    pub fn new() -> Self {
        CovenantDivisorEngine {
            covenants: HashMap::new(),
            property_flows: Vec::new(),
            detours: Vec::new(),
            cobodies: HashMap::new(),
        }
    }

    /// 建立公约
    pub fn establish_covenant(&mut self, title: &str, base_value: u64) {
        if !self.covenants.contains_key(title) {
            self.covenants.insert(title.to_string(), Covenant::new(title, base_value));
        }
    }

    /// 排队入公约
    pub fn enqueue(&mut self, title: &str, value: u64) {
        if let Some(cov) = self.covenants.get_mut(title) {
            cov.enqueue(value);
        }
    }

    /// 获取公约总和
    pub fn covenant_total(&self, title: &str) -> Option<u64> {
        self.covenants.get(title).map(|c| c.total())
    }

    /// 创建产权流
    pub fn create_flow(&mut self, source: ContainerRef, base: u64) -> usize {
        let idx = self.property_flows.len();
        self.property_flows.push(PropertyFlow::new(source, base));
        idx
    }

    /// 扩散产权流
    pub fn diffuse_flow(&mut self, idx: usize) {
        if idx < self.property_flows.len() {
            self.property_flows[idx].diffuse();
        }
    }

    /// 创建迂回路径
    pub fn create_detour(&mut self, origin: ContainerRef, dest: ContainerRef) -> usize {
        let idx = self.detours.len();
        self.detours.push(DetourPath::new(origin, dest));
        idx
    }

    /// 添加跳跃
    pub fn add_hop(&mut self, idx: usize, hop: ContainerRef) {
        if idx < self.detours.len() {
            self.detours[idx].add_hop(hop);
        }
    }

    /// 创建同体空间
    pub fn create_cobody(&mut self, space_id: u64, base: u64) {
        if !self.cobodies.contains_key(&space_id) {
            self.cobodies.insert(space_id, CoBodySpace::new(space_id, base));
        }
    }

    /// 加入空间
    pub fn join_cobody(&mut self, space_id: u64, member: ContainerRef) {
        if let Some(space) = self.cobodies.get_mut(&space_id) {
            space.join(member);
        }
    }

    /// 全局 GCD
    pub fn global_gcd(&self, values: &[u64]) -> u64 {
        if values.is_empty() { return 1; }
        let mut result = values[0];
        for &v in &values[1..] {
            result = gcd64(result, v);
            if result == 1 { break; }
        }
        if result == 0 { 1 } else { result }
    }
}

impl Default for CovenantDivisorEngine {
    fn default() -> Self {
        Self::new()
    }
}

fn gcd64(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    if a == 0 { 1 } else { a }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_ref(entity: u64) -> ContainerRef {
        ContainerRef::new(ContainerKind::Maori, entity);
    }

    #[test]
    fn covenant_new() {
        let c = Covenant::new("测试", 100);
        assert_eq!(c.base_value, 100);
        assert_eq!(c.total(), 100);
        assert_eq!(c.queue_len(), 0);
    }

    #[test]
    fn covenant_enqueue() {
        let mut c = Covenant::new("测试", 100);
        c.enqueue(50);
        c.enqueue(30);

        assert_eq!(c.total(), 180);
        assert_eq!(c.queue_len(), 2);
    }

    #[test]
    fn covenant_dequeue() {
        let mut c = Covenant::new("测试", 100);
        c.enqueue(50);
        c.enqueue(30);

        assert_eq!(c.dequeue(), Some(50));
        assert_eq!(c.queue_len(), 1);
        assert_eq!(c.total(), 130);
    }

    #[test]
    fn covenant_active() {
        let c1 = Covenant::new("有效", 10);
        assert!(c1.is_active());

        let c2 = Covenant::new("空", 0);
        assert!(!c2.is_active());
    }

    #[test]
    fn covenant_queue_average() {
        let mut c = Covenant::new("测试", 100);
        c.enqueue(10);
        c.enqueue(20);

        assert!((c.queue_average() - 15.0).abs() < 1e-9);
    }

    #[test]
    fn common_multiple() {
        let mut m = CommonMultiple::new(12, 3);
        assert_eq!(m.value(), 36);

        m.expand();
        assert_eq!(m.value(), 48);
    }

    #[test]
    fn common_multiple_contains() {
        let m = CommonMultiple::new(6, 10);
        assert!(m.contains(60));
        assert!(m.contains(120));
        assert!(!m.contains(70));
    }

    #[test]
    fn property_flow_diffuse() {
        let mut flow = PropertyFlow::new(mk_ref(100), 50);
        assert_eq!(flow.range(), 50);

        flow.diffuse();
        assert_eq!(flow.range(), 100);
        assert_eq!(flow.depth, 1);
    }

    #[test]
    fn property_flow_reached() {
        let mut flow = PropertyFlow::new(mk_ref(100), 50);
        flow.mark_reached(mk_ref(200));
        flow.mark_reached(mk_ref(300));

        assert_eq!(flow.reached_count(), 2);
    }

    #[test]
    fn detour_path_weight() {
        let origin = mk_ref(1);
        let dest = mk_ref(2);
        let mut path = DetourPath::new(origin, dest);

        assert_eq!(path.weight, 1.0);

        path.add_hop(mk_ref(100));
        // weight = 1 / (1 + 2 hops)
        assert!((path.weight - 1.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn detour_viable() {
        let mut path = DetourPath::new(mk_ref(1), mk_ref(2));
        path.add_hop(mk_ref(100));
        path.add_hop(mk_ref(200));

        // weight = 1/(1+3) = 0.25, still > 0.01
        assert!(path.is_viable());
    }

    #[test]
    fn detour_jump_count() {
        let origin = mk_ref(1);
        let dest = mk_ref(2);
        let mut path = DetourPath::new(origin, dest);

        path.add_hop(mk_ref(10));
        path.add_hop(mk_ref(20));

        // 2 intermediate + 1 final = 3
        assert_eq!(path.jump_count(), 3);
    }

    #[test]
    fn cobody_space_join() {
        let mut space = CoBodySpace::new(1, 10);
        space.join(mk_ref(100));
        space.join(mk_ref(200));

        assert_eq!(space.member_count(), 2);
        // 100 和 200 的公倍数基为 10, gcd=100 > 10, 有关系
        assert!(space.has_relation(100, 200));
    }

    #[test]
    fn cobody_space_relation_strength() {
        let mut space = CoBodySpace::new(1, 1);
        space.join(mk_ref(12));
        space.join(mk_ref(18));

        let strength = space.relation_strength(12, 18);
        // gcd(12,18)=6, base=1, strength >= 6
        assert!(strength >= 6.0);
    }

    #[test]
    fn cobody_space_leave() {
        let mut space = CoBodySpace::new(1, 1);
        space.join(mk_ref(100));
        assert_eq!(space.member_count(), 1);

        space.leave(&mk_ref(100));
        assert_eq!(space.member_count(), 0);
    }

    #[test]
    fn engine_covenant_lifecycle() {
        let mut engine = CovenantDivisorEngine::new();
        engine.establish_covenant("决心", 10);
        engine.enqueue("决心", 5);
        engine.enqueue("决心", 3);

        assert_eq!(engine.covenant_total("决心"), Some(18));
    }

    #[test]
    fn engine_flow_lifecycle() {
        let mut engine = CovenantDivisorEngine::new();
        let idx = engine.create_flow(mk_ref(100), 50);
        engine.diffuse_flow(idx);

        assert_eq!(engine.property_flows[idx].depth, 1);
    }

    #[test]
    fn engine_cobody_lifecycle() {
        let mut engine = CovenantDivisorEngine::new();
        engine.create_cobody(1, 10);
        engine.join_cobody(1, mk_ref(100));
        engine.join_cobody(1, mk_ref(200));

        assert_eq!(engine.cobodies[&1].member_count(), 2);
    }

    #[test]
    fn engine_global_gcd() {
        let engine = CovenantDivisorEngine::new();
        assert_eq!(engine.global_gcd(&[12, 18, 24]), 6);
        assert_eq!(engine.global_gcd(&[7, 11]), 1);
    }

    #[test]
    fn covenant_accumulate_no_overflow_check() {
        let mut c = Covenant::new("累加", 0);
        for i in 1..=100 {
            c.enqueue(i);
        }

        // 1+2+...+100 = 5050
        assert_eq!(c.total(), 5050);
    }
}
