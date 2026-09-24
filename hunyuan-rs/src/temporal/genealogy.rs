// ── v0.42 容器族谱 · 代数层 · 一元至九元层级 ──
// 满全法代数层：加法串联只负责延伸 · 乘法只用于统计
// 族谱 = 容器从一元到九元的生长层 · 见证 = 层间准入

use std::collections::{HashMap, HashSet};

/// 元代数 — 容器在族谱中的代数层级
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GenerationLayer {
    /// 一元 — 根 · 起始 · 生根
    Unary,
    /// 二元 — 分 · 阴阳 · 平行
    Binary,
    /// 三元 — 见证 · 裁定 · 裁判
    Ternary,
    /// 四象 — 关系 · 春夏秋冬
    Quaternary,
    /// 五行 — 容器交互 · 相生相克
    Quinary,
    /// 六合 — 空间 · 三维 × 二极
    Senary,
    /// 七情 — 情感 · 具身情感维度
    Septenary,
    /// 八卦 — 全域 · 覆盖 · 完备
    Octonary,
    /// 九元 — 完成 · 闭合 · 归一
    Novenary,
}

impl GenerationLayer {
    /// 代数值 1-9
    pub fn number(&self) -> u8 {
        match self {
            GenerationLayer::Unary => 1,
            GenerationLayer::Binary => 2,
            GenerationLayer::Ternary => 3,
            GenerationLayer::Quaternary => 4,
            GenerationLayer::Quinary => 5,
            GenerationLayer::Senary => 6,
            GenerationLayer::Septenary => 7,
            GenerationLayer::Octonary => 8,
            GenerationLayer::Novenary => 9,
        }
    }

    /// 从数字创建
    pub fn from_number(n: u8) -> Option<Self> {
        match n {
            1 => Some(GenerationLayer::Unary),
            2 => Some(GenerationLayer::Binary),
            3 => Some(GenerationLayer::Ternary),
            4 => Some(GenerationLayer::Quaternary),
            5 => Some(GenerationLayer::Quinary),
            6 => Some(GenerationLayer::Senary),
            7 => Some(GenerationLayer::Septenary),
            8 => Some(GenerationLayer::Octonary),
            9 => Some(GenerationLayer::Novenary),
            _ => None,
        }
    }

    /// 下一层（加法延伸 · 到九元为止）
    pub fn next(&self) -> Option<Self> {
        Self::from_number(self.number() + 1)
    }

    /// 上一层
    pub fn prev(&self) -> Option<Self> {
        let n = self.number();
        if n > 1 { Self::from_number(n - 1) } else { None }
    }

    /// 是否可生长（非顶元）
    pub fn can_grow(&self) -> bool {
        self.number() < 9
    }
}

/// 容器族谱节点
#[derive(Debug, Clone)]
pub struct GeneNode {
    pub id: u64,
    pub layer: GenerationLayer,
    /// 父代 id（上一代）
    pub parent: Option<u64>,
    /// 子代 ids
    pub children: Vec<u64>,
    /// 见证代数：经过多少次见证裁定才抵达此层
    pub witness_count: u64,
    /// 加法积累（非溢出）
    pub additive_accum: u64,
}

impl GeneNode {
    pub fn new(id: u64, layer: GenerationLayer) -> Self {
        Self {
            id, layer, parent: None, children: Vec::new(), witness_count: 0, additive_accum: 0,
        }
    }

    /// 加法延伸（不到九元不溢出）
    pub fn additive_extend(&mut self, amount: u64) {
        self.additive_accum = self.additive_accum.saturating_add(amount);
    }

    /// 子代数（乘法：仅统计用 → 不参与串联）
    pub fn child_count_factor(&self) -> u64 {
        (self.children.len() as u64).saturating_mul(self.layer.number() as u64)
    }

    /// 是否有子代
    pub fn has_children(&self) -> bool { !self.children.is_empty() }

    /// 层间生长
    pub fn grow(&mut self) -> Option<GenerationLayer> {
        if !self.layer.can_grow() { return None; }
        self.layer.next()
    }
}

/// 加法统计
#[derive(Debug, Clone)]
pub struct AdditiveStats {
    pub total_accum: u64,
    pub node_count: u64,
    /// 乘法口诀表仅用于统计清点（层数 × 节点数）
    pub layer_node_product: u64,
}

/// 族谱引擎
#[derive(Debug)]
pub struct Genealogy {
    pub nodes: HashMap<u64, GeneNode>,
    /// 根代 ids
    pub roots: Vec<u64>,
    /// 代数层 → 该层所有节点
    pub layers: HashMap<u8, Vec<u64>>,
}

impl Genealogy {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            roots: Vec::new(),
            layers: HashMap::new(),
        }
    }

    /// 添加根代
    pub fn add_root(&mut self, id: u64) -> &mut Self {
        let node = GeneNode::new(id, GenerationLayer::Unary);
        self.nodes.insert(id, node);
        self.roots.push(id);
        self.layers.entry(1).or_insert_with(Vec::new).push(id);
        self
    }

    /// 添加子代（加法延伸）
    pub fn add_child(&mut self, parent_id: u64, child_id: u64, child_layer: GenerationLayer) -> bool {
        if !self.nodes.contains_key(&parent_id) { return false; }
        let mut child = GeneNode::new(child_id, child_layer);
        child.parent = Some(parent_id);
        child.additive_accum = self.nodes[&parent_id].additive_accum;
        let child_id_copied = child.id;
        self.nodes.insert(child_id, child);
        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.children.push(child_id_copied);
        }
        self.layers.entry(child_layer.number()).or_insert_with(Vec::new).push(child_id_copied);
        true
    }

    /// 节点向上见证计
    pub fn witness(&mut self, id: u64) -> u64 {
        let node = match self.nodes.get_mut(&id) {
            Some(n) => n,
            None => return 0,
        };
        node.witness_count += 1;
        node.witness_count
    }

    /// 尝试生长到下一层
    pub fn try_grow(&mut self, id: u64) -> Option<GenerationLayer> {
        let node = self.nodes.get(&id)?;
        if node.witness_count < self.threshold_for(&node.layer) { return None; }
        let next_layer = node.layer.next()?;
        let node = self.nodes.get_mut(&id)?;
        let new_layer = node.layer.next()?;
        node.layer = new_layer;
        node.witness_count = 0; // 重置见证计数
        Some(new_layer)
    }

    /// 层生长阈值（见证次数需够）
    fn threshold_for(&self, layer: &GenerationLayer) -> u64 {
        layer.number() as u64 * 2
    }

    /// 获取某层所有节点
    pub fn get_layer(&self, layer: GenerationLayer) -> Vec<&GeneNode> {
        self.layers.get(&layer.number())
            .map(|ids| ids.iter().filter_map(|id| self.nodes.get(id)).collect())
            .unwrap_or_default()
    }

    /// 加法统计
    pub fn additive_stats(&self) -> AdditiveStats {
        let total: u64 = self.nodes.values().map(|n| n.additive_accum).sum();
        let node_count = self.nodes.len() as u64;
        let layer_node_product = self.layers.iter()
            .map(|(layer_num, ids)| (*layer_num as u64) * (ids.len() as u64))
            .sum();
        AdditiveStats {
            total_accum: self.nodes.values().map(|n| n.additive_accum).sum(),
            node_count,
            layer_node_product,
        }
    }

    /// 节点路径（id → 根）
    pub fn path_to_root(&self, id: u64) -> Vec<u64> {
        let mut path = Vec::new();
        let mut current = Some(id);
        while let Some(curr) = current {
            path.push(curr);
            current = self.nodes.get(&curr).and_then(|n| n.parent);
        }
        path.reverse();
        path
    }

    /// 深度（节点距根的层数）
    pub fn depth(&self, id: u64) -> u64 {
        self.path_to_root(id).len() as u64 - 1
    }

    /// 统计节点总数
    pub fn total_nodes(&self) -> usize { self.nodes.len() }

    /// 根代数量
    pub fn root_count(&self) -> usize { self.roots.len() }
}

impl Default for Genealogy {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generation_number() {
        assert_eq!(GenerationLayer::Unary.number(), 1);
        assert_eq!(GenerationLayer::Novenary.number(), 9);
    }

    #[test]
    fn test_generation_next() {
        assert_eq!(GenerationLayer::Unary.next(), Some(GenerationLayer::Binary));
        assert_eq!(GenerationLayer::Novenary.next(), None);
    }

    #[test]
    fn test_generation_prev() {
        assert_eq!(GenerationLayer::Binary.prev(), Some(GenerationLayer::Unary));
        assert_eq!(GenerationLayer::Unary.prev(), None);
    }

    #[test]
    fn test_from_number() {
        assert_eq!(GenerationLayer::from_number(1), Some(GenerationLayer::Unary));
        assert_eq!(GenerationLayer::from_number(9), Some(GenerationLayer::Novenary));
        assert_eq!(GenerationLayer::from_number(10), None);
    }

    #[test]
    fn test_new_genealogy() {
        let g = Genealogy::new();
        assert_eq!(g.total_nodes(), 0);
        assert_eq!(g.root_count(), 0);
    }

    #[test]
    fn test_add_root() {
        let mut g = Genealogy::new();
        g.add_root(1);
        assert_eq!(g.total_nodes(), 1);
        assert_eq!(g.root_count(), 1);
    }

    #[test]
    fn test_add_child() {
        let mut g = Genealogy::new();
        g.add_root(1);
        assert!(g.add_child(1, 2, GenerationLayer::Binary));
        assert_eq!(g.total_nodes(), 2);
        assert!(g.nodes[&1].has_children());
    }

    #[test]
    fn test_witness() {
        let mut g = Genealogy::new();
        g.add_root(1);
        assert_eq!(g.witness(1), 1);
        assert_eq!(g.witness(1), 2);
    }

    #[test]
    fn test_try_grow() {
        let mut g = Genealogy::new();
        g.add_root(1);
        // 需要 1*2=2 次见证才能生长
        g.witness(1);
        g.witness(1);
        let new_layer = g.try_grow(1);
        assert_eq!(new_layer, Some(GenerationLayer::Binary));
    }

    #[test]
    fn test_try_grow_insufficient_witness() {
        let mut g = Genealogy::new();
        g.add_root(1);
        g.witness(1);
        assert!(g.try_grow(1).is_none()); // 见证不足
    }

    #[test]
    fn test_child_count_factor() {
        let mut node = GeneNode::new(1, GenerationLayer::Ternary);
        node.children = vec![2, 3];
        assert_eq!(node.child_count_factor(), 6); // 2 * 3
    }

    #[test]
    fn test_additive_accum() {
        let mut node = GeneNode::new(1, GenerationLayer::Unary);
        node.additive_extend(42);
        node.additive_extend(58);
        assert_eq!(node.additive_accum, 100);
    }

    #[test]
    fn test_path_to_root() {
        let mut g = Genealogy::new();
        g.add_root(1);
        g.add_child(1, 2, GenerationLayer::Binary);
        g.add_child(2, 3, GenerationLayer::Ternary);
        let path = g.path_to_root(3);
        assert_eq!(path, vec![1, 2, 3]);
    }

    #[test]
    fn test_depth() {
        let mut g = Genealogy::new();
        g.add_root(1);
        g.add_child(1, 2, GenerationLayer::Binary);
        g.add_child(2, 3, GenerationLayer::Ternary);
        assert_eq!(g.depth(1), 0);
        assert_eq!(g.depth(2), 1);
        assert_eq!(g.depth(3), 2);
    }

    #[test]
    fn test_additive_stats() {
        let mut g = Genealogy::new();
        g.add_root(1);
        g.add_child(1, 2, GenerationLayer::Binary);
        let stats = g.additive_stats();
        assert_eq!(stats.node_count, 2);
        assert_eq!(stats.layer_node_product, 3); // 1*1 + 1*2
    }

    #[test]
    fn test_get_layer() {
        let mut g = Genealogy::new();
        g.add_root(1);
        g.add_child(1, 2, GenerationLayer::Binary);
        let unary = g.get_layer(GenerationLayer::Unary);
        assert_eq!(unary.len(), 1);
        let binary = g.get_layer(GenerationLayer::Binary);
        assert_eq!(binary.len(), 1);
    }

    #[test]
    fn test_generation_can_grow() {
        assert!(GenerationLayer::Unary.can_grow());
        assert!(!GenerationLayer::Novenary.can_grow());
    }

    #[test]
    fn test_node_grow() {
        let mut node = GeneNode::new(1, GenerationLayer::Binary);
        let next = node.grow();
        assert_eq!(next, Some(GenerationLayer::Ternary));
    }

    #[test]
    fn test_node_grow_max() {
        let mut node = GeneNode::new(1, GenerationLayer::Novenary);
        assert!(node.grow().is_none());
    }

    #[test]
    fn test_layer_enum_iterate() {
        let layers = vec![
            GenerationLayer::Unary,
            GenerationLayer::Binary,
            GenerationLayer::Ternary,
            GenerationLayer::Quaternary,
            GenerationLayer::Quinary,
            GenerationLayer::Senary,
            GenerationLayer::Septenary,
            GenerationLayer::Octonary,
            GenerationLayer::Novenary,
        ];
        for (i, layer) in layers.iter().enumerate() {
            assert_eq!(layer.number(), (i + 1) as u8);
        }
    }
}
