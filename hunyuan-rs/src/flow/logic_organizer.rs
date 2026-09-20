// ══════════════════════════════════════════════════════════════════════
// logic_organizer.rs — 逻辑组织引擎 · 因果路径排序·呼唤策略·路径叠加态 (v0.46)
// ══════════════════════════════════════════════════════════════════════
//
//   「符号的秩序不来自声明, 来自公约数整除的层级涌现」
//
//   逻辑组织引擎解决三个问题:
//
//     1. 因果路径排序 — 给定一组呼唤入口, 按依赖关系拓扑排序
//        入口 A 呼唤入口 B → B 必须在 A 之后执行
//        不是声明顺序, 是拓扑偏序 → 结构自然涌现
//
//     2. 呼唤策略 — 当多个路径同时可供时, 按公约数共振强度选取
//        GCD(入口A的签名, 入口B的签名) > 1 → 构成共存候选
//        最大公约数路径 = 最大公倍数节律 = 最稳定的执行序列
//
//     3. 路径叠加态 — 未观测前, 多条因果路径同时存在
//        每一条都是一条完整的执行链 (独立序)
//        一旦观测(分别器裁决), 坍缩为单一链
//        叠加态不是不确定, 是"多个确定态的共存"
//
//   设计约束:
//     - 不引入乘除 (仅加法 + GCD 公约数)
//     - 路径标识用呼唤表编号, 不是字符串
//     - 排序结果不可变(编译期约束, 运行时只读)
//

use crate::flow_algebra::InterpState;
use crate::resonance_discovery::{gcd_additive, is_resonant};

// ────────────────────────────────────────────────────────────
// 因果节点 — 呼唤表的依赖图顶点
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct CausalNode {
    /// 入口编号 (呼唤表索引)
    pub entry_id: u32,
    /// 入口签名 (公约数共振的计算基底)
    pub signature: u64,
    /// 本节点调用的下游节点编号
    pub calls: Vec<u32>,
    /// 本节点的解释流状态
    pub interp_state: InterpState,
}

impl CausalNode {
    pub fn new(entry_id: u32, signature: u64) -> Self {
        Self {
            entry_id,
            signature,
            calls: Vec::new(),
            interp_state: InterpState::Neutral,
        }
    }

    pub fn with_calls(entry_id: u32, signature: u64, calls: Vec<u32>) -> Self {
        Self {
            entry_id,
            signature,
            calls,
            interp_state: InterpState::Neutral,
        }
    }

    /// 判断下游节点是否与本节点共振 (GCD > 1)
    pub fn resonates_with(&self, other_sig: u64) -> bool {
        is_resonant(self.signature, other_sig, 1)
    }

    /// 与下游节点的共振强度 (公约数大小)
    pub fn resonance_strength(&self, other_sig: u64) -> u64 {
        gcd_additive(self.signature, other_sig)
    }
}

// ────────────────────────────────────────────────────────────
// 因果路径 — 一条完整的执行链
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct CausalPath {
    /// 路径上的节点序列 (有序)
    pub nodes: Vec<CausalNode>,
    /// 路径总共振强度 (各相邻节点间共振之和)
    pub total_resonance: u64,
}

impl CausalPath {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            total_resonance: 0,
        }
    }

    pub fn push(&mut self, node: CausalNode) {
        if let Some(last) = self.nodes.last() {
            self.total_resonance += gcd_additive(last.signature, node.signature);
        }
        self.nodes.push(node);
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// 路径是否完全共振 (所有相邻节点 GCD > 1)
    pub fn fully_resonant(&self) -> bool {
        if self.nodes.len() < 2 {
            return true;
        }
        self.nodes.windows(2).all(|w| {
            gcd_additive(w[0].signature, w[1].signature) > 1
        })
    }

    /// 路径条目编号序列
    pub fn entry_ids(&self) -> Vec<u32> {
        self.nodes.iter().map(|n| n.entry_id).collect()
    }

    /// 路径签名序列
    pub fn signatures(&self) -> Vec<u64> {
        self.nodes.iter().map(|n| n.signature).collect()
    }
}

impl Default for CausalPath {
    fn default() -> Self {
        Self::new()
    }
}

// ────────────────────────────────────────────────────────────
// 呼唤策略 — 多路径间的选择规则
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InvocationStrategy {
    /// 最大共振优先 — 选 total_resonance 最高的路径
    MaxResonance,
    /// 最长路径优先 — 覆盖最多入口
    LongestPath,
    /// 最短路径优先 — 最小代价
    ShortestPath,
    /// 完全共振优先 — 只选 fully_resonant 的路径
    FullyResonant,
}

// ────────────────────────────────────────────────────────────
// 路径叠加态 — 多条因果路径同时存在
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct PathSuperposition {
    /// 叠加中的候选路径
    pub paths: Vec<CausalPath>,
    /// 是否已坍缩
    pub collapsed: bool,
    /// 坍缩后的路径(如果已坍缩)
    pub collapsed_path: Option<CausalPath>,
}

impl PathSuperposition {
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
            collapsed: false,
            collapsed_path: None,
        }
    }

    pub fn add_path(&mut self, path: CausalPath) {
        if !self.collapsed {
            self.paths.push(path);
        }
    }

    pub fn len(&self) -> usize {
        self.paths.len()
    }

    pub fn is_empty(&self) -> bool {
        self.paths.is_empty()
    }

    /// 分别器裁决 — 按策略坍缩为单一路径
    pub fn collapse(&mut self, strategy: InvocationStrategy) -> Option<&CausalPath> {
        if self.paths.is_empty() {
            return None;
        }

        let best_idx = match strategy {
            InvocationStrategy::MaxResonance => {
                self.paths.iter()
                    .enumerate()
                    .max_by_key(|(_, p)| p.total_resonance)
                    .map(|(i, _)| i)
            }
            InvocationStrategy::LongestPath => {
                self.paths.iter()
                    .enumerate()
                    .max_by_key(|(_, p)| p.len())
                    .map(|(i, _)| i)
            }
            InvocationStrategy::ShortestPath => {
                self.paths.iter()
                    .enumerate()
                    .min_by_key(|(_, p)| p.len())
                    .map(|(i, _)| i)
            }
            InvocationStrategy::FullyResonant => {
                // 先尝试找完全共振路径; 不存在则退化为 MaxResonance
                let fully: Vec<_> = self.paths.iter()
                    .enumerate()
                    .filter(|(_, p)| p.fully_resonant())
                    .collect();
                if fully.is_empty() {
                    self.paths.iter()
                        .enumerate()
                        .max_by_key(|(_, p)| p.total_resonance)
                        .map(|(i, _)| i)
                } else {
                    fully.into_iter()
                        .max_by_key(|(_, p)| p.total_resonance)
                        .map(|(i, _)| i)
                }
            }
        };

        if let Some(idx) = best_idx {
            let chosen = self.paths[idx].clone();
            self.collapsed = true;
            self.collapsed_path = Some(chosen);
            self.collapsed_path.as_ref()
        } else {
            None
        }
    }

    /// 未观测时的路径数量 (叠加度)
    pub fn superposition_degree(&self) -> usize {
        if self.collapsed { 1 } else { self.paths.len() }
    }
}

impl Default for PathSuperposition {
    fn default() -> Self {
        Self::new()
    }
}

// ────────────────────────────────────────────────────────────
// 因果图 — 呼唤表的依赖拓扑
// ────────────────────────────────────────────────────────────

pub struct CausalGraph {
    /// 所有节点 (entry_id → CausalNode)
    nodes: Vec<CausalNode>,
}

impl CausalGraph {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: CausalNode) {
        self.nodes.push(node);
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// 拓扑排序 — Kahn 算法 (按入度归零顺序)
    /// 返回排序后的 entry_id 序列; 如果有循环依赖则返回部分排序
    pub fn topological_sort(&self) -> Vec<u32> {
        let n = self.nodes.len();
        if n == 0 {
            return Vec::new();
        }

        // 建立 entry_id → 索引映射
        let id_to_idx: std::collections::HashMap<u32, usize> = self.nodes.iter()
            .enumerate()
            .map(|(i, node)| (node.entry_id, i))
            .collect();

        // 计算入度
        let mut in_degree = vec![0u32; n];
        for (i, node) in self.nodes.iter().enumerate() {
            for &called_id in &node.calls {
                if let Some(&j) = id_to_idx.get(&called_id) {
                    // node[i] 呼唤 node[j] → node[j] 入度+1
                    in_degree[j] += 1;
                }
            }
        }

        // 入度为 0 的节点入队
        let mut queue: std::collections::VecDeque<usize> = in_degree.iter()
            .enumerate()
            .filter(|(_, &d)| d == 0)
            .map(|(i, _)| i)
            .collect();

        let mut result = Vec::new();

        while let Some(idx) = queue.pop_front() {
            result.push(self.nodes[idx].entry_id);

            // 减少被呼唤节点的入度
            for &called_id in &self.nodes[idx].calls {
                if let Some(&j) = id_to_idx.get(&called_id) {
                    in_degree[j] = in_degree[j].saturating_sub(1);
                    if in_degree[j] == 0 {
                        queue.push_back(j);
                    }
                }
            }
        }

        result
    }

    /// 构建依赖子图 — 从指定入口开始, 沿 calls 展开所有可达节点
    pub fn subgraph_from(&self, start_id: u32) -> Vec<u32> {
        let id_set: std::collections::HashSet<u32> = self.nodes.iter()
            .map(|n| n.entry_id)
            .collect();

        let mut visited = std::collections::HashSet::new();
        let mut stack = vec![start_id];
        let mut result = Vec::new();

        while let Some(id) = stack.pop() {
            if visited.insert(id) {
                result.push(id);
                if let Some(node) = self.nodes.iter().find(|n| n.entry_id == id) {
                    for &called in &node.calls {
                        if id_set.contains(&called) && !visited.contains(&called) {
                            stack.push(called);
                        }
                    }
                }
            }
        }

        result
    }

    /// 从图中提取一条共振路径 (沿共振最强的分支走)
    pub fn extract_resonant_path(&self, start_id: u32) -> CausalPath {
        let mut path = CausalPath::new();
        let id_to_node: std::collections::HashMap<u32, &CausalNode> = self.nodes.iter()
            .map(|n| (n.entry_id, n))
            .collect();

        let mut current_id = start_id;
        let mut visited = std::collections::HashSet::new();

        while let Some(&node) = id_to_node.get(&current_id) {
            if !visited.insert(current_id) {
                break; // 防止循环
            }
            path.push((*node).clone());

            // 找下游中共振强度最强的
            let best_next = node.calls.iter()
                .filter_map(|cid| id_to_node.get(cid)
                    .map(|n| (*cid, gcd_additive(node.signature, n.signature))))
                .filter(|(_, strength)| *strength > 1)
                .max_by_key(|(_, strength)| *strength);

            match best_next {
                Some((next_id, _)) => current_id = next_id,
                None => break,
            }
        }

        path
    }

    /// 枚举所有可能的共振路径 (从入度为 0 的节点开始)
    pub fn enumerate_paths(&self) -> PathSuperposition {
        let mut superposition = PathSuperposition::new();

        let topo = self.topological_sort();
        if topo.is_empty() {
            return superposition;
        }

        // 找入度为 0 的起始节点
        let mut in_degree_map: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
        for node in &self.nodes {
            for &called in &node.calls {
                *in_degree_map.entry(called).or_insert(0) += 1;
            }
        }

        let roots: Vec<u32> = topo.iter()
            .filter(|id| in_degree_map.get(*id).copied().unwrap_or(0) == 0)
            .copied()
            .collect();

        for root_id in roots {
            let path = self.extract_resonant_path(root_id);
            if !path.is_empty() {
                superposition.add_path(path);
            }
        }

        // 如果没有找到起点, 把所有节点各自作为单节点路径
        if superposition.is_empty() {
            for node in &self.nodes {
                let mut path = CausalPath::new();
                path.push(node.clone());
                superposition.add_path(path);
            }
        }

        superposition
    }
}

impl Default for CausalGraph {
    fn default() -> Self {
        Self::new()
    }
}

// ────────────────────────────────────────────────────────────
// 逻辑组织引擎 — 统一入口
// ────────────────────────────────────────────────────────────

pub struct LogicOrganizer {
    /// 因果依赖图
    pub graph: CausalGraph,
    /// 当前叠加态
    pub superposition: PathSuperposition,
    /// 选取策略
    pub strategy: InvocationStrategy,
}

impl LogicOrganizer {
    pub fn new(strategy: InvocationStrategy) -> Self {
        Self {
            graph: CausalGraph::new(),
            superposition: PathSuperposition::new(),
            strategy,
        }
    }

    /// 添加因果节点
    pub fn add(&mut self, entry_id: u32, signature: u64, calls: Vec<u32>) {
        self.graph.add_node(CausalNode::with_calls(entry_id, signature, calls));
    }

    /// 构建叠加态 (枚举所有路径)
    pub fn build(&mut self) {
        self.superposition = self.graph.enumerate_paths();
    }

    /// 分别器裁决 — 坍缩为单一执行链
    pub fn decide(&mut self) -> Option<&CausalPath> {
        self.superposition.collapse(self.strategy)
    }

    /// 直接取拓扑排序结果 (不考虑共振)
    pub fn topo_order(&self) -> Vec<u32> {
        self.graph.topological_sort()
    }

    /// 从主控入口提取共振路径
    pub fn resonant_path(&self, start_id: u32) -> CausalPath {
        self.graph.extract_resonant_path(start_id)
    }

    /// 统计
    pub fn stats(&self) -> LogicStats {
        LogicStats {
            node_count: self.graph.node_count(),
            path_count: self.superposition.len(),
            collapsed: self.superposition.collapsed,
            superposition_degree: self.superposition.superposition_degree(),
        }
    }
}

impl Default for LogicOrganizer {
    fn default() -> Self {
        Self::new(InvocationStrategy::MaxResonance)
    }
}

// ────────────────────────────────────────────────────────────
// 统计
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LogicStats {
    pub node_count: usize,
    pub path_count: usize,
    pub collapsed: bool,
    pub superposition_degree: usize,
}

// ────────────────────────────────────────────────────────────
// 便捷函数
// ────────────────────────────────────────────────────────────

/// 从一组 (entry_id, signature, calls) 构建逻辑组织
pub fn organize(nodes: &[(u32, u64, Vec<u32>)], strategy: InvocationStrategy) -> LogicOrganizer {
    let mut org = LogicOrganizer::new(strategy);
    for &(id, sig, ref calls) in nodes {
        org.add(id, sig, calls.clone());
    }
    org.build();
    org
}

/// 拓扑排序独立函数
pub fn causal_sort(nodes: &[(u32, Vec<u32>)]) -> Vec<u32> {
    let mut graph = CausalGraph::new();
    for &(id, ref calls) in nodes {
        // 签名用 entry_id 的 FNV 风格散列
        let sig = simple_hash(&id.to_le_bytes());
        graph.add_node(CausalNode::with_calls(id, sig, calls.clone()));
    }
    graph.topological_sort()
}

fn simple_hash(bytes: &[u8]) -> u64 {
    // FNV-1a 64-bit
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

use std::collections::VecDeque;

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topological_sort_linear() {
        // A → B → C 线性依赖
        let mut graph = CausalGraph::new();
        graph.add_node(CausalNode::with_calls(1, 100, vec![2]));
        graph.add_node(CausalNode::with_calls(2, 200, vec![3]));
        graph.add_node(CausalNode::with_calls(3, 300, vec![]));

        let order = graph.topological_sort();
        assert_eq!(order, vec![1, 2, 3]);
    }

    #[test]
    fn test_topological_sort_diamond() {
        //   1
        //  / \
        // 2   3
        //  \ /
        //   4
        let mut graph = CausalGraph::new();
        graph.add_node(CausalNode::with_calls(1, 100, vec![2, 3]));
        graph.add_node(CausalNode::with_calls(2, 200, vec![4]));
        graph.add_node(CausalNode::with_calls(3, 300, vec![4]));
        graph.add_node(CausalNode::with_calls(4, 400, vec![]));

        let order = graph.topological_sort();
        // 1 必须在 2,3 前; 2,3 必须在 4 前
        assert!(order.iter().position(|&x| x == 1) < order.iter().position(|&x| x == 2));
        assert!(order.iter().position(|&x| x == 1) < order.iter().position(|&x| x == 3));
        assert!(order.iter().position(|&x| x == 2) < order.iter().position(|&x| x == 4));
        assert!(order.iter().position(|&x| x == 3) < order.iter().position(|&x| x == 4));
    }

    #[test]
    fn test_causal_path_resonance() {
        // 6 和 9 的 GCD = 3 > 1 → 共振
        let mut path = CausalPath::new();
        path.push(CausalNode::new(1, 6));
        path.push(CausalNode::new(2, 9));
        path.push(CausalNode::new(3, 12));

        assert!(path.fully_resonant()); // GCD(6,9)=3, GCD(9,12)=3
        assert_eq!(path.total_resonance, 6); // 3 + 3
    }

    #[test]
    fn test_superposition_collapse_max_resonance() {
        let mut superpos = PathSuperposition::new();

        let mut path_a = CausalPath::new();
        path_a.push(CausalNode::new(1, 100));
        path_a.push(CausalNode::new(2, 200));

        let mut path_b = CausalPath::new();
        path_b.push(CausalNode::new(3, 6));
        path_b.push(CausalNode::new(4, 9));
        path_b.push(CausalNode::new(5, 12));
        path_b.push(CausalNode::new(6, 15));

        superpos.add_path(path_a);
        superpos.add_path(path_b);

        let chosen = superpos.collapse(InvocationStrategy::MaxResonance);
        assert!(chosen.is_some());
        // path_b 共振更强 (GCDs: 3+3+3=9) > path_a (GCD(100,200)=100, 但100和200的GCD...)
        // Actually GCD(100,200) = 100, so path_a.total_resonance = 100
        // path_b.total_resonance = 3 + 3 + 3 = 9
        // MaxResonance should pick path_a
        assert_eq!(chosen.unwrap().total_resonance, 100);
    }

    #[test]
    fn test_superposition_collapse_longest() {
        let mut superpos = PathSuperposition::new();

        let mut path_a = CausalPath::new();
        path_a.push(CausalNode::new(1, 10));
        path_a.push(CausalNode::new(2, 20));

        let mut path_b = CausalPath::new();
        path_b.push(CausalNode::new(3, 30));
        path_b.push(CausalNode::new(4, 40));
        path_b.push(CausalNode::new(5, 50));

        superpos.add_path(path_a);
        superpos.add_path(path_b);

        let chosen = superpos.collapse(InvocationStrategy::LongestPath);
        assert_eq!(chosen.unwrap().len(), 3);
    }

    #[test]
    fn test_extract_resonant_path() {
        // 1(sig=6) → [2(sig=9), 3(sig=7)]
        // GCD(6,9)=3 > GCD(6,7)=1 → 选 2
        let mut graph = CausalGraph::new();
        graph.add_node(CausalNode::with_calls(1, 6, vec![2, 3]));
        graph.add_node(CausalNode::with_calls(2, 9, vec![]));
        graph.add_node(CausalNode::with_calls(3, 7, vec![]));

        let path = graph.extract_resonant_path(1);
        assert_eq!(path.entry_ids(), vec![1, 2]);
    }

    #[test]
    fn test_organize_simple() {
        let nodes = vec![
            (1u32, 6u64, vec![2u32]),
            (2, 9, vec![3]),
            (3, 12, vec![]),
        ];

        let mut org = organize(&nodes, InvocationStrategy::FullyResonant);
        let stats = org.stats();
        assert_eq!(stats.node_count, 3);
        assert!(stats.path_count >= 1);

        let chosen = org.decide();
        assert!(chosen.is_some());
    }

    #[test]
    fn test_subgraph_from() {
        let mut graph = CausalGraph::new();
        graph.add_node(CausalNode::with_calls(1, 100, vec![2, 3]));
        graph.add_node(CausalNode::with_calls(2, 200, vec![4]));
        graph.add_node(CausalNode::with_calls(3, 300, vec![]));
        graph.add_node(CausalNode::with_calls(4, 400, vec![]));
        graph.add_node(CausalNode::with_calls(5, 500, vec![])); // 孤立

        let sub = graph.subgraph_from(1);
        assert!(sub.contains(&1));
        assert!(sub.contains(&2));
        assert!(sub.contains(&3));
        assert!(sub.contains(&4));
        assert!(!sub.contains(&5));
    }

    #[test]
    fn test_causal_sort() {
        let nodes = vec![
            (1u32, vec![2u32, 3u32]),
            (2, vec![4]),
            (3, vec![4]),
            (4, vec![]),
        ];

        let order = causal_sort(&nodes);
        assert_eq!(order.len(), 4);
        // 1 在最前, 4 在最后
        assert_eq!(order[0], 1);
        assert_eq!(order[3], 4);
    }

    #[test]
    fn test_superposition_degree() {
        let mut superpos = PathSuperposition::new();
        let mut path = CausalPath::new();
        path.push(CausalNode::new(1, 100));
        superpos.add_path(path.clone());
        superpos.add_path(path);

        assert_eq!(superpos.superposition_degree(), 2);

        superpos.collapse(InvocationStrategy::MaxResonance);
        assert_eq!(superpos.superposition_degree(), 1); // 坍缩后 = 1
    }
}
