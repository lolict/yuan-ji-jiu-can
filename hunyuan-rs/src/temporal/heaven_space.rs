// ── v0.42 天庭空间 · 三维容器空间 · 纤维丛 · 吸引子 ──
// 满全法：空间本身 = 容器 · 坐标 = 数值地址 · 纤维丛 = 关系几何
// 天庭 = 容器生存的几何基底 · 庭 = 庭堂 · 空间 = 三维坐标

use std::collections::HashMap;

/// 三维空间坐标（单数值编码，非 x/y/z 三元组）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpaceCoordinate(pub u64);

impl SpaceCoordinate {
    /// 创建坐标（直接数值编码）
    pub fn new(v: u64) -> Self { Self(v) }

    /// 坐标值
    pub fn value(&self) -> u64 { self.0 }

    /// 曼哈顿距离（用于天庭内寻路）
    pub fn manhattan(&self, other: &SpaceCoordinate) -> u64 {
        if self.0 > other.0 { self.0 - other.0 } else { other.0 - self.0 }
    }

    /// 坐标偏移
    pub fn offset(&self, delta: i64) -> Option<Self> {
        if delta >= 0 {
            Some(Self(self.0.wrapping_add(delta as u64)))
        } else {
            let abs = (-delta) as u64;
            if abs <= self.0 { Some(Self(self.0 - abs)) } else { None }
        }
    }
}

/// 纤维丛：坐标 → 容器族 的纤维化映射
#[derive(Debug, Clone)]
pub struct FiberBundle {
    pub base: SpaceCoordinate,
    pub fiber: String,  // 容器族名称
    pub connection: u64, // 联络强度（关系密度）
}

impl FiberBundle {
    pub fn new(base: SpaceCoordinate, fiber: &str, connection: u64) -> Self {
        Self { base, fiber: fiber.to_string(), connection }
    }

    /// 纤维密度：联络强度 / 坐标范围
    pub fn density(&self, range: u64) -> f64 {
        if range == 0 { return 0.0; }
        self.connection as f64 / range as f64
    }
}

/// 吸引子：空间中自然聚集的容器簇
#[derive(Debug, Clone)]
pub struct Attractor {
    pub center: SpaceCoordinate,
    pub name: String,
    pub member_ids: Vec<u64>,
    pub field_strength: u64,  // 吸引子场强
}

impl Attractor {
    pub fn new(center: SpaceCoordinate, name: &str) -> Self {
        Self { center, name: name.to_string(), member_ids: Vec::new(), field_strength: 1 }
    }

    pub fn add_member(&mut self, id: u64, pos: SpaceCoordinate) -> bool {
        let dist = self.center.manhattan(&pos);
        if dist <= self.field_strength * 100 {
            self.member_ids.push(id);
            true
        } else {
            false
        }
    }

    pub fn member_count(&self) -> usize { self.member_ids.len() }

    pub fn contains(&self, id: u64) -> bool { self.member_ids.contains(&id) }

    /// 增强场强（吸引子成长）
    pub fn strengthen(&mut self, amount: u64) { self.field_strength += amount; }
}

/// 天庭中的容器
#[derive(Debug, Clone)]
pub struct CelestialContainer {
    pub id: u64,
    pub position: SpaceCoordinate,
    pub family: String,
    /// 天位（高天 = 高位 · 低天 = 隐藏侧）
    pub heaven_tier: u8,
    /// 活跃标记
    pub active: bool,
    /// 纤维丛锚定
    pub fiber_name: Option<String>,
}

impl CelestialContainer {
    pub fn new(id: u64, pos: SpaceCoordinate, family: &str) -> Self {
        Self {
            id, position: pos, family: family.to_string(),
            heaven_tier: 0, active: true, fiber_name: None,
        }
    }

    pub fn with_tier(mut self, tier: u8) -> Self { self.heaven_tier = tier; self }

    /// 容器是否可见（非隐藏侧）
    pub fn is_visible(&self) -> bool { self.heaven_tier >= 128 }

    /// 是否在配偶天区（夫妻族核心区）
    pub fn is_couple_zone(&self) -> bool {
        self.family == "couple" || self.family == "夫妻族"
    }
}

/// 天庭空间引擎
#[derive(Debug)]
pub struct HeavenSpace {
    pub containers: HashMap<u64, CelestialContainer>,
    pub fibers: Vec<FiberBundle>,
    pub attractors: HashMap<String, Attractor>,
    /// 坐标轴范围
    pub axis_range: u64,
    /// 鼓膜自举标志：空间是否自动从几何中涌现
    pub auto_bootstrap: bool,
}

impl HeavenSpace {
    pub fn new() -> Self {
        Self {
            containers: HashMap::new(),
            fibers: Vec::new(),
            attractors: HashMap::new(),
            axis_range: u64::MAX,
            auto_bootstrap: true,
        }
    }

    pub fn place_container(&mut self, container: CelestialContainer) -> &mut Self {
        self.containers.insert(container.id, container);
        self
    }

    pub fn create_attractor(&mut self, center: SpaceCoordinate, name: &str) -> &mut Self {
        let a = Attractor::new(center, name);
        self.attractors.insert(name.to_string(), a);
        self
    }

    /// 放置纤维丛
    pub fn lay_fiber(&mut self, base: SpaceCoordinate, fiber: &str, connection: u64) -> &mut Self {
        self.fibers.push(FiberBundle::new(base, fiber, connection));
        self
    }

    /// 吸引子聚类：将所有未归属容器纳入最近吸引子
    pub fn cluster(&mut self) -> usize {
        let mut clustered = 0;
        // 先收集最佳匹配（避免同时借用）
        let mut assignments: Vec<(u64, String)> = Vec::new();
        for (_, container) in self.containers.iter() {
            if !container.active { continue; }
            let mut best: Option<String> = None;
            let mut best_dist = u64::MAX;
            for (_name, attractor) in self.attractors.iter() {
                let dist = attractor.center.manhattan(&container.position);
                if dist < best_dist {
                    best_dist = dist;
                    best = Some(attractor.name.clone());
                }
            }
            if let Some(name) = best {
                assignments.push((container.id, name));
            }
        }
        // 应用赋值
        for (id, name) in assignments {
            if let Some(attractor) = self.attractors.get_mut(&name) {
                if let Some(container) = self.containers.get(&id) {
                    if attractor.add_member(id, container.position) {
                        clustered += 1;
                    }
                }
            }
        }
        clustered
    }

        /// 天庭区域统计
    pub fn tier_distribution(&self) -> (usize, usize) {
        let visible = self.containers.values().filter(|c| c.is_visible()).count();
        let hidden = self.containers.len() - visible;
        (visible, hidden)
    }

    /// 寻找容器邻居（距离 < threshold）
    pub fn neighbors(&self, id: u64, threshold: u64) -> Vec<u64> {
        let pos = match self.containers.get(&id) {
            Some(c) => c.position,
            None => return Vec::new(),
        };
        self.containers.iter()
            .filter(|(other_id, c)| {
                **other_id != id && pos.manhattan(&c.position) < threshold
            })
            .map(|(id, _)| *id)
            .collect()
    }

    /// 容器总数
    pub fn count(&self) -> usize { self.containers.len() }

    /// 吸引子总数
    pub fn attractor_count(&self) -> usize { self.attractors.len() }

    /// 纤维总数
    pub fn fiber_count(&self) -> usize { self.fibers.len() }
}

impl Default for HeavenSpace {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heaven_new() {
        let h = HeavenSpace::new();
        assert_eq!(h.count(), 0);
        assert!(h.auto_bootstrap);
    }

    #[test]
    fn test_coordinate() {
        let c = SpaceCoordinate::new(42);
        assert_eq!(c.value(), 42);
    }

    #[test]
    fn test_coordinate_distance() {
        let a = SpaceCoordinate::new(100);
        let b = SpaceCoordinate::new(150);
        assert_eq!(a.manhattan(&b), 50);
        assert_eq!(b.manhattan(&a), 50);
    }

    #[test]
    fn test_coordinate_offset() {
        let c = SpaceCoordinate::new(100);
        assert_eq!(c.offset(10).unwrap().value(), 110);
        assert_eq!(c.offset(-10).unwrap().value(), 90);
        assert!(c.offset(-200).is_none());
    }

    #[test]
    fn test_place_container() {
        let mut h = HeavenSpace::new();
        h.place_container(CelestialContainer::new(1, SpaceCoordinate::new(0), "base"));
        assert_eq!(h.count(), 1);
    }

    #[test]
    fn test_container_visibility() {
        let vis = CelestialContainer::new(1, SpaceCoordinate::new(0), "x").with_tier(200);
        assert!(vis.is_visible());
        let hid = CelestialContainer::new(2, SpaceCoordinate::new(0), "x").with_tier(50);
        assert!(!hid.is_visible());
    }

    #[test]
    fn test_couple_zone() {
        let c = CelestialContainer::new(1, SpaceCoordinate::new(0), "couple");
        assert!(c.is_couple_zone());
        let c2 = CelestialContainer::new(2, SpaceCoordinate::new(0), "base");
        assert!(!c2.is_couple_zone());
    }

    #[test]
    fn test_fiber_bundle() {
        let fb = FiberBundle::new(SpaceCoordinate::new(0), "test", 100);
        assert_eq!(fb.density(1000), 0.1);
    }

    #[test]
    fn test_attractor_add_member() {
        let mut a = Attractor::new(SpaceCoordinate::new(0), "center");
        a.strengthen(10);
        // field_strength=11, threshold=1100
        let nearby = SpaceCoordinate::new(100);
        assert!(a.add_member(1, nearby));
        let far = SpaceCoordinate::new(99999);
        assert!(!a.add_member(2, far));
        assert_eq!(a.member_count(), 1);
    }

    #[test]
    fn test_attractor_contains() {
        let mut a = Attractor::new(SpaceCoordinate::new(0), "test");
        a.strengthen(100);
        a.add_member(42, SpaceCoordinate::new(50));
        assert!(a.contains(42));
        assert!(!a.contains(99));
    }

    #[test]
    fn test_lay_fiber() {
        let mut h = HeavenSpace::new();
        h.lay_fiber(SpaceCoordinate::new(0), "夫妻族", 61);
        assert_eq!(h.fiber_count(), 1);
    }

    #[test]
    fn test_neighbors() {
        let mut h = HeavenSpace::new();
        h.place_container(CelestialContainer::new(1, SpaceCoordinate::new(0), "base"));
        h.place_container(CelestialContainer::new(2, SpaceCoordinate::new(5), "base"));
        h.place_container(CelestialContainer::new(3, SpaceCoordinate::new(999), "base"));
        let n = h.neighbors(1, 10);
        assert!(n.contains(&2));
        assert!(!n.contains(&3));
    }

    #[test]
    fn test_cluster() {
        let mut h = HeavenSpace::new();
        h.create_attractor(SpaceCoordinate::new(50), "hub");
        h.place_container(CelestialContainer::new(1, SpaceCoordinate::new(45), "x"));
        h.place_container(CelestialContainer::new(2, SpaceCoordinate::new(55), "x"));
        h.place_container(CelestialContainer::new(3, SpaceCoordinate::new(9999), "x"));
        let clustered = h.cluster();
        assert!(clustered >= 1);
    }

    #[test]
    fn test_attractor_strengthen() {
        let mut a = Attractor::new(SpaceCoordinate::new(0), "growing");
        assert_eq!(a.field_strength, 1);
        a.strengthen(9);
        assert_eq!(a.field_strength, 10);
    }

    #[test]
    fn test_tier_distribution() {
        let mut h = HeavenSpace::new();
        h.place_container(CelestialContainer::new(1, SpaceCoordinate::new(0), "x").with_tier(200));
        h.place_container(CelestialContainer::new(2, SpaceCoordinate::new(0), "x").with_tier(50));
        h.place_container(CelestialContainer::new(3, SpaceCoordinate::new(0), "x").with_tier(128));
        let (vis, hid) = h.tier_distribution();
        assert_eq!(vis, 2);  // tier 200, 128
        assert_eq!(hid, 1);  // tier 50
    }
}
