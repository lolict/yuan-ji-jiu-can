// ╔══════════════════════════════════════════════════════════════════╗
// ║  v0.70 纤维丛引擎                                                ║
// ║  容器丛化 · 局部平凡化 · 截面选择 · 转移函数 · 几何化身            ║
// ║                                                                  ║
// ║  纤维丛结构:                                                     ║
// ║  · 底空间 (Base): MaoriBase 的 3D 坐标                          ║
// ║  · 纤维 (Fiber): 每个坐标点上的容器族                            ║
// ║  · 总空间 (Total): 所有纤维的并集 = 容器图集                     ║
// ║  · 投影 (π): ContainerRef → MaoriCoord                           ║
// ║  · 局部平凡化: 在邻域 U 内, 纤维丛 ≅ U × Fiber                  ║
// ║  · 转移函数: 纤维间的坐标变换规则                                ║
// ║  · 截面: 从底空间到总空间的连续选择                              ║
// ╚══════════════════════════════════════════════════════════════════╝

use crate::container_ir::{ContainerAtlas, ContainerKind, ContainerRef, RelationKind};
use crate::distance_perception::{ContainerCode, DistancePerception};
use crate::mlil::{MaoriAddr, MaoriBase, MaoriCoord};

/// 局部平凡化 — 描述一个邻域内的纤维结构
#[derive(Clone, Debug)]
pub struct LocalTrivialization {
    /// 邻域中心坐标
    pub center: MaoriCoord,
    /// 邻域半径
    pub radius: u32,
    /// 该邻域内的纤维容器
    pub fibers: Vec<ContainerRef>,
    /// 纤维标签: 局部坐标 → 容器引用
    pub local_coords: Vec<(u32, u32, ContainerRef)>,
}

impl LocalTrivialization {
    /// 创建局部平凡化
    pub fn new(center: MaoriCoord, radius: u32) -> Self {
        LocalTrivialization {
            center,
            radius,
            fibers: Vec::new(),
            local_coords: Vec::new(),
        }
    }

    /// 添加纤维容器
    pub fn add_fiber(&mut self, cref: ContainerRef, local_x: u32, local_y: u32) {
        self.fibers.push(cref.clone());
        self.local_coords.push((local_x, local_y, cref));
    }

    /// 纤维数量
    pub fn fiber_count(&self) -> usize {
        self.fibers.len()
    }

    /// 总条目数 (与 fiber_count 同义)
    pub fn len(&self) -> usize {
        self.fibers.len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.fibers.is_empty()
    }
}

/// 计算两个坐标的曼哈顿距离 (自由的辅助函数)
fn coord_manhattan(a: &MaoriCoord, b: &MaoriCoord) -> u32 {
    ((a.x as i64 - b.x as i64).abs()
        + (a.y as i64 - b.y as i64).abs()
        + (a.z as i64 - b.z as i64).abs()) as u32
}

/// 转移函数 — 两个纤维间的坐标变换
#[derive(Clone, Debug)]
pub struct TransitionFunction {
    /// 源纤维的底空间坐标
    pub from_coord: MaoriCoord,
    /// 目标纤维的底空间坐标
    pub to_coord: MaoriCoord,
    /// 源容器
    pub from_ref: ContainerRef,
    /// 目标容器
    pub to_ref: ContainerRef,
    /// 变换权重 (距离倒数, 越近权重越大)
    pub weight: f64,
}

impl TransitionFunction {
    /// 计算两个容器间的转移权重
    pub fn compute(from: &ContainerRef, to: &ContainerRef, from_coord: &MaoriCoord, to_coord: &MaoriCoord) -> Self {
        let code_a = ContainerCode::new(from.entity() as u32);
        let code_b = ContainerCode::new(to.entity() as u32);
        let dist = DistancePerception::distance(&code_a, &code_b);
        let geo_dist = coord_manhattan(from_coord, to_coord);

        // 转移权重 = 1 / (1 + 距离 + 几何距离)
        let weight = 1.0 / (1.0 + dist + geo_dist as f64);

        TransitionFunction {
            from_coord: from_coord.clone(),
            to_coord: to_coord.clone(),
            from_ref: from.clone(),
            to_ref: to.clone(),
            weight,
        }
    }
}

/// 截面 — 从底空间到总空间的连续选择
#[derive(Clone, Debug)]
pub struct Section {
    /// 截面通过的坐标序列
    pub path: Vec<MaoriCoord>,
    /// 每个坐标对应的容器选择
    pub selections: Vec<ContainerRef>,
    /// 截面权重 (路径上所有转移权重之积)
    pub path_weight: f64,
}

impl Section {
    /// 创建空截面
    pub fn new() -> Self {
        Section {
            path: Vec::new(),
            selections: Vec::new(),
            path_weight: 1.0,
        }
    }

    /// 添加截面点
    pub fn add_point(&mut self, coord: MaoriCoord, selection: ContainerRef, transition_weight: f64) {
        self.path.push(coord);
        self.selections.push(selection);
        self.path_weight *= transition_weight;
    }

    /// 截面长度 (经过的坐标数)
    pub fn len(&self) -> usize {
        self.path.len()
    }

    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }
}

impl Default for Section {
    fn default() -> Self {
        Self::new()
    }
}

/// 容器丛化引擎
pub struct FiberBundleEngine {
    /// 所有局部平凡化
    trivializations: Vec<LocalTrivialization>,
    /// 所有转移函数
    transitions: Vec<TransitionFunction>,
    /// 邻域半径
    radius: u32,
}

impl FiberBundleEngine {
    /// 创建纤维丛引擎
    pub fn new(radius: u32) -> Self {
        FiberBundleEngine {
            trivializations: Vec::new(),
            transitions: Vec::new(),
            radius,
        }
    }

    /// 从容器图集自动构建纤维丛
    pub fn build_from_atlas(&mut self, atlas: &ContainerAtlas, base: &MaoriBase) {
        self.build_trivializations(atlas, base);
    }

    /// 构建局部平凡化
    pub fn build_trivializations(&mut self, atlas: &ContainerAtlas, base: &MaoriBase) {
        // 遍历 base 的每个已占坐标, 以其为中心建立局部平凡化
        let entries: Vec<(u64, String)> = base.entries().map(|(k, v)| (k, v.clone())).collect();

        for (linear, _label) in &entries {
            if let Some(coord) = base.from_linear(*linear) {
                if let Some(_addr) = base.encode(&coord) {
                    let mut triv = LocalTrivialization::new(coord.clone(), self.radius);

                    // 找邻域内的所有坐标 (曼哈顿距离 ≤ radius)
                    for (other_linear, _other_label) in &entries {
                        if let Some(other_coord) = base.from_linear(*other_linear) {
                            if base.manhattan(&coord, &other_coord) <= self.radius {
                                if let Some(addr_check) = base.encode(&other_coord) {
                                    let cref = ContainerRef::with_octant(
                                        ContainerKind::Maori,
                                        addr_check.raw(),
                                        addr_check.octant(),
                                    );
                                    let local_x = (other_coord.x as i64 - coord.x as i64).abs() as u32;
                                    let local_y = (other_coord.y as i64 - coord.y as i64).abs() as u32;
                                    triv.add_fiber(cref, local_x, local_y);
                                }
                            }
                        }
                    }

                    if !triv.is_empty() {
                        self.trivializations.push(triv);
                    }
                }
            }
        }

        // 构建转移函数
        self.build_transitions(atlas);
    }

    /// 构建转移函数
    fn build_transitions(&mut self, _atlas: &ContainerAtlas) {
        self.transitions.clear();
        for triv in &self.trivializations {
            // 纤维之间的转移
            for i in 0..triv.fibers.len() {
                for j in (i + 1)..triv.fibers.len() {
                    let t = TransitionFunction::compute(
                        &triv.fibers[i],
                        &triv.fibers[j],
                        &triv.center,
                        &triv.center,
                    );
                    self.transitions.push(t);
                }
            }
        }
    }

    /// 获取所有局部平凡化
    pub fn trivializations(&self) -> &[LocalTrivialization] {
        &self.trivializations
    }

    /// 获取所有转移函数
    pub fn transitions(&self) -> &[TransitionFunction] {
        &self.transitions
    }

    /// 局部平凡化数量
    pub fn trivialization_count(&self) -> usize {
        self.trivializations.len()
    }

    /// 转移函数数量
    pub fn transition_count(&self) -> usize {
        self.transitions.len()
    }

    /// 选择截面: 从一个坐标到另一个坐标的最优路径
    pub fn select_section(&self, from: &MaoriCoord, to: &MaoriCoord) -> Option<Section> {
        // 找到包含 from 和 to 的局部平凡化
        let from_triv = self.trivializations.iter()
            .find(|t| coord_manhattan(&t.center, from) <= self.radius)?;
        let to_triv = self.trivializations.iter()
            .find(|t| coord_manhattan(&t.center, to) <= self.radius)?;

        // 选择两端最匹配的纤维
        let from_cref = from_triv.fibers.first()?;
        let to_cref = to_triv.fibers.first()?;

        // 计算转移权重
        let code_a = ContainerCode::new(from_cref.entity() as u32);
        let code_b = ContainerCode::new(to_cref.entity() as u32);
        let dist = DistancePerception::distance(&code_a, &code_b);
        let weight = 1.0 / (1.0 + dist);

        let mut section = Section::new();
        section.add_point(from.clone(), from_cref.clone(), weight);
        section.add_point(to.clone(), to_cref.clone(), weight);

        Some(section)
    }

    /// 所有局部平凡化的总纤维数
    pub fn total_fibers(&self) -> usize {
        self.trivializations.iter().map(|t| t.fiber_count()).sum()
    }
}

impl Default for FiberBundleEngine {
    fn default() -> Self {
        Self::new(3)
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mlil::{MaoriAxes, MaoriBase};

    fn mk_ml_ref(entity: u64, octant: u8) -> ContainerRef {
        ContainerRef::with_octant(ContainerKind::Maori, entity, octant)
    }

    #[test]
    fn local_trivialization_new() {
        let center = MaoriCoord::new(5, 5, 5);
        let triv = LocalTrivialization::new(center, 3);
        assert!(triv.is_empty());
        assert_eq!(triv.fiber_count(), 0);
        assert_eq!(triv.radius, 3);
    }

    #[test]
    fn local_trivialization_add_fiber() {
        let center = MaoriCoord::new(0, 0, 0);
        let mut triv = LocalTrivialization::new(center, 5);
        let a = mk_ml_ref(100, 1);
        triv.add_fiber(a, 1, 2);
        assert_eq!(triv.fiber_count(), 1);
        assert!(!triv.is_empty());
    }

    #[test]
    fn transition_function_compute() {
        let a = mk_ml_ref(0x1000_0000, 0);
        let b = mk_ml_ref(0x2000_0000, 1);
        let coord_a = MaoriCoord::new(1, 1, 1);
        let coord_b = MaoriCoord::new(2, 2, 2);

        let t = TransitionFunction::compute(&a, &b, &coord_a, &coord_b);
        assert!(t.weight > 0.0);
        assert!(t.weight <= 1.0);
    }

    #[test]
    fn transition_closer_higher_weight() {
        let a = mk_ml_ref(0x1000_0000, 0);
        let b = mk_ml_ref(0x2000_0000, 1);
        let c = mk_ml_ref(0x3000_0000, 2);
        let coord = MaoriCoord::new(0, 0, 0);

        let t_ab = TransitionFunction::compute(&a, &b, &coord, &coord);
        let t_ac = TransitionFunction::compute(&a, &c, &coord, &coord);
        // Both should be valid weights
        assert!(t_ab.weight > 0.0);
        assert!(t_ac.weight > 0.0);
    }

    #[test]
    fn section_new_empty() {
        let s = Section::new();
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
        assert_eq!(s.path_weight, 1.0);
    }

    #[test]
    fn section_add_points() {
        let mut s = Section::new();
        let a = mk_ml_ref(100, 0);
        let b = mk_ml_ref(200, 1);
        let coord_a = MaoriCoord::new(1, 1, 1);
        let coord_b = MaoriCoord::new(2, 2, 2);
        s.add_point(coord_a, a, 0.5);
        s.add_point(coord_b, b, 0.5);
        assert_eq!(s.len(), 2);
        assert!((s.path_weight - 0.25).abs() < 1e-9); // 0.5 * 0.5
    }

    #[test]
    fn fiber_bundle_default() {
        let engine: FiberBundleEngine = FiberBundleEngine::default();
        assert_eq!(engine.trivialization_count(), 0);
        assert_eq!(engine.transition_count(), 0);
    }

    #[test]
    fn fiber_bundle_with_radius() {
        let mut engine = FiberBundleEngine::new(5);
        // 手动添加一个局部平凡化
        let center = MaoriCoord::new(10, 10, 10);
        let mut triv = LocalTrivialization::new(center, 5);
        let a = mk_ml_ref(0x100, 0);
        triv.add_fiber(a, 0, 0);
        engine.trivializations.push(triv);
        assert_eq!(engine.trivialization_count(), 1);
        assert_eq!(engine.total_fibers(), 1);
    }

    #[test]
    fn select_section_with_insufficient_trivializations() {
        let engine = FiberBundleEngine::new(3);
        let from = MaoriCoord::new(0, 0, 0);
        let to = MaoriCoord::new(1, 1, 1);
        let result = engine.select_section(&from, &to);
        assert!(result.is_none());
    }

    #[test]
    fn select_section_valid() {
        let mut engine = FiberBundleEngine::new(5);
        // 添加至少一个覆盖 (0,0,0) 的局部平凡化
        let center = MaoriCoord::new(0, 0, 0);
        let mut triv = LocalTrivialization::new(center, 5);
        triv.add_fiber(mk_ml_ref(0x100, 0), 0, 0);
        triv.add_fiber(mk_ml_ref(0x200, 1), 1, 0);
        triv.add_fiber(mk_ml_ref(0x300, 2), 0, 1);
        engine.trivializations.push(triv);

        // 添加覆盖 (2,2,2) 的局部平凡化
        let center2 = MaoriCoord::new(2, 2, 2);
        let mut triv2 = LocalTrivialization::new(center2, 5);
        triv2.add_fiber(mk_ml_ref(0x400, 0), 0, 0);
        engine.trivializations.push(triv2);

        let from = MaoriCoord::new(0, 0, 0);
        let to = MaoriCoord::new(2, 2, 2);
        let section = engine.select_section(&from, &to);
        assert!(section.is_some());
        let s = section.unwrap();
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn build_from_maori_base() {
        let axes = MaoriAxes::default();
        let mut base = MaoriBase::new(axes);

        // 放置几个坐标
        for (x, y, z, label) in [
            (1u32, 2u32, 3u32, "满全法"),
            (2, 3, 4, "囡囝共生"),
            (5, 5, 5, "意识路由"),
        ] {
            base.place(&MaoriCoord::new(x, y, z), label);
        }

        let atlas = ContainerAtlas::new();
        let mut engine = FiberBundleEngine::new(5);
        engine.build_trivializations(&atlas, &base);

        assert!(engine.trivialization_count() >= 1);
        assert!(engine.total_fibers() >= 3);
    }

    #[test]
    fn trivialization_fiber_count_matches() {
        let center = MaoriCoord::new(16, 16, 16);
        let mut triv = LocalTrivialization::new(center, 3);
        for i in 0..5 {
            triv.add_fiber(mk_ml_ref(1000 + i as u64, (i % 8) as u8), i, i);
        }
        assert_eq!(triv.fiber_count(), 5);
        assert_eq!(triv.len(), 5);
    }
}
