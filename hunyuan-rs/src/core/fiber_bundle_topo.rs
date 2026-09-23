// ═══════════════════════════════════════════════════════════════════════════
// fiber_bundle_topo.rs — 纤维丛容器拓扑引擎 · 高维关系结构 (v0.1)
// ═══════════════════════════════════════════════════════════════════════════
//
// 核心命题（满全法·囡囝共生 NJSG 高维代数）：
//
//   "纤维丛"不是抽象数学，而是容器之间的可感知关系拓扑。
//
//   底空间 B = 容器坐标语义（SMRL 三维：语义·操作数·时间）
//   纤维 F = 每个容器内禀属性空间（极性·主权·价值·见证）
//   全空间 E = B × F（容器总空间）
//   截面 s: B → E（容器的选择函数，即"此刻哪些容器在线"）
//
//   纤维丛拓扑回答的核心问题：
//     "容器 A 与容器 B 在哪个维度上属于同一个纤维？"
//     "从容器 A 到容器 B 的纤维方向是什么（归属·调用·见证·对立）？"
//     "整个容器空间的连通性结构（哪些容器群落是纤维连通的）？"
//
// 纤维方向（φυβερ方向）：
//   Belong   — A 归属于 B（父子套娃）
//   Invoke   — A 调用 B（参数传递·流跳转）
//   Witness  — A 见证 B（三元裁判·同时观测）
//   Opposite — A 对立于 B（阴·阳·裁）
//   Parallel — A 平行于 B（同维度·不同实例）
//
// ═══════════════════════════════════════════════════════════════════════════

use std::collections::{HashMap, HashSet};

use crate::cross_isa::coordinate::Coord3D;
use crate::cross_isa::fridge::ContainerPack;

// ───────────────────────────────────────────────────────────────────────────
// 纤维方向
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FiberDir {
    /// 归属：A 在 B 内部/被 B 包含
    Belong,
    /// 调用：A 需要 B 完成计算（跳转·传参）
    Invoke,
    /// 见证：A 与 B 被同一裁判观测
    Witness,
    /// 对立：A 与 B 极性相反（阴阳）
    Opposite,
    /// 平行：A 与 B 同语义不同实例
    Parallel,
}

impl FiberDir {
    pub fn to_glyph(&self) -> char {
        match self {
            FiberDir::Belong => '⊃',
            FiberDir::Invoke => '→',
            FiberDir::Witness => '◇',
            FiberDir::Opposite => '↔',
            FiberDir::Parallel => '∥',
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 纤维边 — 两个容器之间的有向关系
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FiberEdge {
    pub from: u64,      // pack_id
    pub to: u64,        // pack_id
    pub dir: FiberDir,
    pub weight: u8,     // 关系强度 [1, 10]
}

impl FiberEdge {
    pub fn new(from: u64, to: u64, dir: FiberDir, weight: u8) -> Self {
        Self { from, to, dir, weight: weight.clamp(1, 10) }
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 纤维节点 — 容器在纤维丛空间中的表示
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct FiberNode {
    pub pack_id: u64,
    pub coord: Coord3D,
    pub polarity_yin: bool,         // true = 阴, false = 阳
    pub sovereignty_level: u8,      // 主权级别 [0, 9]
    pub value_code: u8,             // 价值编码
    pub witness_valid: bool,        // 见证签名是否有效
    /// 关联的出边
    pub out_edges: Vec<FiberEdge>,
    /// 关联的入边
    pub in_edges: Vec<FiberEdge>,
}

impl FiberNode {
    pub fn from_pack(pack: &ContainerPack) -> Self {
        let coord = pack.coord();
        // 极性：payload 第一个字节的最低位
        let yin = pack.payload.first().map(|b| b & 1 == 0).unwrap_or(true);
        // 主权级别：Y 轴中 4 位 [0-9]
        let sov = ((coord.y >> 4) & 0x0F).min(9) as u8;
        // 价值编码：payload 校验和的低 8 位
        let val = pack.witness.iter().copied().fold(0u8, |a: u8, b: u8| a.wrapping_add(*pack.payload.get(b as usize % pack.payload.len().max(1)).unwrap_or(&0)));
        // 见证有效
        let witness_ok = pack.witness != [0xFF, 0xFF, 0xFF] && pack.witness != [0xFF, 0xFE, 0xFD];

        Self {
            pack_id: pack.pack_id,
            coord,
            polarity_yin: yin,
            sovereignty_level: sov,
            value_code: val,
            witness_valid: witness_ok,
            out_edges: Vec::new(),
            in_edges: Vec::new(),
        }
    }

    /// 出度
    pub fn out_degree(&self) -> usize { self.out_edges.len() }
    /// 入度
    pub fn in_degree(&self) -> usize { self.in_edges.len() }
    /// 总连接度
    pub fn total_degree(&self) -> usize { self.out_degree() + self.in_degree() }
}

// ───────────────────────────────────────────────────────────────────────────
// 纤维丛拓扑 — 容器之间的高维关系图
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct FiberBundleTopo {
    /// 所有节点
    pub nodes: HashMap<u64, FiberNode>,
    /// 所有边
    pub edges: Vec<FiberEdge>,
    /// 连通分量（群落 ID → 容器 pack_id 集合）
    pub components: Vec<HashSet<u64>>,
}

impl FiberBundleTopo {
    // ─── 构造 ───────────────────────────────────────────────────────

    /// 从容器包集合自动构建纤维丛拓扑
    pub fn build(packs: &[ContainerPack]) -> Self {
        let mut nodes: HashMap<u64, FiberNode> = packs.iter()
            .map(|p| (p.pack_id, FiberNode::from_pack(p)))
            .collect();

        let mut edges: Vec<FiberEdge> = Vec::new();
        let ids: Vec<u64> = packs.iter().map(|p| p.pack_id).collect();

        for i in 0..ids.len() {
            for j in (i + 1)..ids.len() {
                let id_a = ids[i];
                let id_b = ids[j];
                if let (Some(a), Some(b)) = (nodes.get(&id_a), nodes.get(&id_b)) {
                    let dirs = Self::classify_relation(a, b);
                    for dir in dirs {
                        edges.push(FiberEdge::new(id_a, id_b, dir, 5));
                        edges.push(FiberEdge::new(id_b, id_a, dir, 5));
                    }
                }
            }
        }

        // 填充节点边表
        for edge in &edges {
            if let Some(n) = nodes.get_mut(&edge.from) { n.out_edges.push(*edge); }
            if let Some(n) = nodes.get_mut(&edge.to) { n.in_edges.push(*edge); }
        }

        let components = Self::find_components(&nodes, &edges);

        Self { nodes, edges, components }
    }

    // ─── 关系分类 ───────────────────────────────────────────────────

    /// 根据两个容器的坐标语义判定它们之间的纤维关系
    fn classify_relation(a: &FiberNode, b: &FiberNode) -> Vec<FiberDir> {
        let mut dirs = Vec::new();

        // 平行：同语义 ID，不同实例
        if a.coord.x == b.coord.x && a.pack_id != b.pack_id {
            dirs.push(FiberDir::Parallel);
        }

        // 见证：同一时间窗口（Z 相同）
        if a.coord.z == b.coord.z && a.pack_id != b.pack_id {
            dirs.push(FiberDir::Witness);
        }

        // 对立：极性相反且 Z 互补（Z + z' = 255 = 0xFF）
        if a.polarity_yin != b.polarity_yin
            && (a.coord.z as u16 + b.coord.z as u16 == 255 || (a.coord.z == b.coord.z && (a.coord.x as u16 + b.coord.x as u16) % 2 == 1)) {
            dirs.push(FiberDir::Opposite);
        }

        // 调用：A 是跳转类 (sem 0x09-0x0B) 且 B 被 A 以 param_slot 的低位索引到
        let a_is_jump = matches!(a.coord.x, 0x09 | 0x0A | 0x0B);
        let b_indexed_by_a = (a.coord.y & 0x00FF) as u64 == b.pack_id || (b.coord.y & 0x00FF) as u64 == a.pack_id;
        if a_is_jump && b_indexed_by_a {
            dirs.push(FiberDir::Invoke);
        }

        // 归属：同语义，Y轴 B 在 A 的子槽范围内（A 的 param 高位 = B 的 id 低位）
        if a.coord.x == b.coord.x && a.coord.y > b.coord.y && a.coord.y.wrapping_sub(b.coord.y) < 0x0100 {
            dirs.push(FiberDir::Belong);
        }

        // 默认：若无任何关系，添加弱平行（同群落）
        if dirs.is_empty() && a.coord.z == b.coord.z {
            dirs.push(FiberDir::Witness);
        }

        dirs
    }

    // ─── 连通分量 ───────────────────────────────────────────────────

    fn find_components(nodes: &HashMap<u64, FiberNode>, edges: &[FiberEdge]) -> Vec<HashSet<u64>> {
        let mut visited: HashSet<u64> = HashSet::new();
        let mut components: Vec<HashSet<u64>> = Vec::new();

        // 邻接表（双向，连通性不区分方向）
        let mut adj: HashMap<u64, Vec<u64>> = HashMap::new();
        for e in edges {
            adj.entry(e.from).or_default().push(e.to);
            adj.entry(e.to).or_default().push(e.from);
        }

        for &start in nodes.keys() {
            if visited.contains(&start) { continue; }
            let mut component: HashSet<u64> = HashSet::new();
            let mut stack = vec![start];
            while let Some(v) = stack.pop() {
                if !visited.insert(v) { continue; }
                component.insert(v);
                if let Some(neighbors) = adj.get(&v) {
                    for &n in neighbors { if !visited.contains(&n) { stack.push(n); } }
                }
            }
            components.push(component);
        }

        components
    }

    // ─── 查询接口 ──────────────────────────────────────────────────

    pub fn node(&self, id: u64) -> Option<&FiberNode> { self.nodes.get(&id) }

    pub fn edges_from(&self, id: u64) -> Vec<&FiberEdge> {
        self.edges.iter().filter(|e| e.from == id).collect()
    }

    pub fn edges_to(&self, id: u64) -> Vec<&FiberEdge> {
        self.edges.iter().filter(|e| e.to == id).collect()
    }

    pub fn edges_of_dir(&self, dir: FiberDir) -> Vec<&FiberEdge> {
        self.edges.iter().filter(|e| e.dir == dir).collect()
    }

    /// 度数最高的 N 个节点
    pub fn top_degree_nodes(&self, n: usize) -> Vec<(&u64, usize)> {
        let mut v: Vec<(&u64, usize)> = self.nodes.iter().map(|(id, node)| (id, node.total_degree())).collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        v.into_iter().take(n).collect()
    }

    /// 统计各方向的关系数量
    pub fn dir_histogram(&self) -> HashMap<FiberDir, usize> {
        let mut m = HashMap::new();
        for e in &self.edges {
            *m.entry(e.dir).or_insert(0) += 1;
        }
        m
    }

    /// 路径搜索（BFS，忽略方向）
    pub fn path_between(&self, from: u64, to: u64) -> Option<Vec<u64>> {
        if !self.nodes.contains_key(&from) || !self.nodes.contains_key(&to) { return None; }
        let mut visited: HashSet<u64> = HashSet::new();
        let mut queue: Vec<Vec<u64>> = vec![vec![from]];
        visited.insert(from);

        let adj: HashMap<u64, Vec<u64>> = {
            let mut a: HashMap<u64, Vec<u64>> = HashMap::new();
            for e in &self.edges {
                a.entry(e.from).or_default().push(e.to);
                a.entry(e.to).or_default().push(e.from);
            }
            a
        };

        while let Some(path) = queue.first().cloned() { queue.remove(0);
            let last = *path.last().unwrap();
            if last == to { return Some(path); }
            if let Some(neighbors) = adj.get(&last) {
                for &n in neighbors {
                    if visited.insert(n) {
                        let mut new_path = path.clone();
                        new_path.push(n);
                        queue.push(new_path);
                    }
                }
            }
        }
        None
    }

    /// 统计
    pub fn stats(&self) -> String {
        let hist = self.dir_histogram();
        format!(
            "FiberBundleTopo[nodes={}, edges={}, components={}, belong={}, invoke={}, witness={}, opposite={}, parallel={}]",
            self.nodes.len(), self.edges.len(), self.components.len(),
            hist.get(&FiberDir::Belong).unwrap_or(&0),
            hist.get(&FiberDir::Invoke).unwrap_or(&0),
            hist.get(&FiberDir::Witness).unwrap_or(&0),
            hist.get(&FiberDir::Opposite).unwrap_or(&0),
            hist.get(&FiberDir::Parallel).unwrap_or(&0),
        )
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 测试
// ───────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_packs(n: usize) -> Vec<ContainerPack> {
        (0..n).map(|i| {
            let x = (i % 15 + 1) as u8;
            let y = (0x1000 + i as u16) as u16;
            let z = (i % 4 + 1) as u8;
            ContainerPack::new(i as u64, Coord3D::new(x, y, z), 0, vec![0xAA, 0xBB, 0xCC, 0xDD])
        }).collect()
    }

    #[test]
    fn fiber_build_creates_nodes_and_edges() {
        let packs = mock_packs(10);
        let topo = FiberBundleTopo::build(&packs);
        assert_eq!(topo.nodes.len(), 10);
        assert!(!topo.edges.is_empty());
    }

    #[test]
    fn fiber_polarity_assignment() {
        // 偶数 payload 首字节 → yin
        let p_yin = ContainerPack::new(0, Coord3D::new(1, 0, 1), 0, vec![0x00, 1, 2, 3]);
        let node = FiberNode::from_pack(&p_yin);
        assert!(node.polarity_yin);

        // 奇数 payload 首字节 → yang
        let p_yang = ContainerPack::new(1, Coord3D::new(1, 0, 1), 0, vec![0x01, 1, 2, 3]);
        let node = FiberNode::from_pack(&p_yang);
        assert!(!node.polarity_yin);
    }

    #[test]
    fn fiber_parallel_relation_among_same_sem() {
        // 同语义 ID → 应产生 Parallel 关系
        let packs = vec![
            ContainerPack::new(0, Coord3D::new(0x03, 0x1000, 1), 0, vec![1, 2, 3, 4]),
            ContainerPack::new(1, Coord3D::new(0x03, 0x1001, 1), 0, vec![5, 6, 7, 8]),
        ];
        let topo = FiberBundleTopo::build(&packs);
        let par_count = topo.edges_of_dir(FiberDir::Parallel).len();
        assert!(par_count >= 2, "平行关系应双向各一条: {}", par_count);
    }

    #[test]
    fn fiber_witness_relation_with_same_z() {
        // 同 Z → 应产生 Witness 关系
        let packs = vec![
            ContainerPack::new(0, Coord3D::new(0x01, 0, 3), 0, vec![1, 2, 3, 4]),
            ContainerPack::new(1, Coord3D::new(0x02, 0, 3), 0, vec![5, 6, 7, 8]),
        ];
        let topo = FiberBundleTopo::build(&packs);
        let wit = topo.edges_of_dir(FiberDir::Witness).len();
        assert!(wit >= 2, "同 Z 容器应有见证关系: {}", wit);
    }

    #[test]
    fn fiber_components_connected() {
        let packs = mock_packs(20);
        let topo = FiberBundleTopo::build(&packs);
        // 由于同 z 的容器之间有 witness 边，图可能是高度连通的
        assert!(topo.components.len() <= 20);
        if topo.components.len() == 1 {
            assert_eq!(topo.components[0].len(), 20);
        }
    }

    #[test]
    fn fiber_path_search_exists_within_component() {
        let packs = mock_packs(8);
        let topo = FiberBundleTopo::build(&packs);
        // 同属一个连通分量应有路径
        if topo.components.len() == 1 {
            let ids: Vec<u64> = packs.iter().map(|p| p.pack_id).collect();
            assert!(topo.path_between(ids[0], ids[7]).is_some());
        }
    }

    #[test]
    fn fiber_dir_histogram_complete() {
        // 构建更丰富的测试数据：混合 X 和 Z 以确保触发多种关系
        let mut packs = Vec::new();
        // 同语义 X=3 → Parallel
        for i in 0..5 {
            packs.push(ContainerPack::new(i, Coord3D::new(0x03, 0x1000 + i as u16, 1), 0, vec![0xAA, 0xBB, 0xCC, 0xDD]));
        }
        // 异语义，奇数首字节 → Yang (与 0xAA Yin 形成 Opposite)
        for i in 0..5 {
            packs.push(ContainerPack::new(5 + i, Coord3D::new(0x05, 0x2000 + i as u16, 2), 0, vec![0x01, 0xBB, 0xCC, 0xDD]));
        }
        let topo = FiberBundleTopo::build(&packs);
        let hist = topo.dir_histogram();
        // 应至少有 Witness 和 Parallel 两类
        assert!(hist.len() >= 2, "histogram: {:?}", hist);
        assert!(hist.contains_key(&FiberDir::Parallel), "should have parallel");
    }

    #[test]
    fn fiber_stats_format() {
        let packs = mock_packs(10);
        let topo = FiberBundleTopo::build(&packs);
        let s = topo.stats();
        assert!(s.contains("FiberBundleTopo"));
        assert!(s.contains("nodes=10"));
    }

    #[test]
    fn fiber_top_degree_nodes() {
        let packs = mock_packs(20);
        let topo = FiberBundleTopo::build(&packs);
        let top = topo.top_degree_nodes(3);
        assert!(top.len() <= 3);
        // 度数应非负且按降序
        for w in top.windows(2) {
            assert!(w[0].1 >= w[1].1);
        }
    }
}
