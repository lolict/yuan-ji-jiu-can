//! 区块链式纠缠索引网络
//!
//! 每个节点携带：
//!   - 小纸条（元数据）: 层级/类型/审美/归属权/黑名单
//!   - 前缀指针列表: 可以接在它前面的节点
//!   - 后缀指针列表: 可以接在它后面的节点
//!
//! 传导逻辑：从任意节点出发，沿 prefix/suffix 遍历全部合法组合

use std::collections::HashMap;
use std::fmt;
use crate::pixel::PhasePixel;

/// 字形/声音区块链节点
#[derive(Debug, Clone)]
pub struct GlyphNode {
    pub glyph_id: String,
    pub pixel: PhasePixel,
    pub symbol: String,
    pub prefix: Vec<String>,
    pub suffix: Vec<String>,
    pub annotate: HashMap<String, String>,
}

impl GlyphNode {
    pub fn new(glyph_id: &str, pixel: PhasePixel) -> GlyphNode {
        GlyphNode {
            glyph_id: glyph_id.to_string(),
            pixel,
            symbol: String::new(),
            prefix: Vec::new(),
            suffix: Vec::new(),
            annotate: HashMap::new(),
        }
    }

    pub fn can_connect_before(&self, node_id: &str) -> bool {
        self.prefix.iter().any(|p| p == node_id)
    }

    pub fn can_connect_after(&self, node_id: &str) -> bool {
        self.suffix.iter().any(|s| s == node_id)
    }
}

/// 区块链纠缠索引 — 全局节点网络
#[derive(Debug, Clone)]
pub struct BlockchainIndex {
    pub nodes: HashMap<String, GlyphNode>,
    pub connection_count: usize,
}

impl BlockchainIndex {
    pub fn new() -> BlockchainIndex {
        BlockchainIndex {
            nodes: HashMap::new(),
            connection_count: 0,
        }
    }

    pub fn register(&mut self, node: GlyphNode) {
        self.nodes.insert(node.glyph_id.clone(), node);
    }

    pub fn connect(&mut self, from_id: &str, to_id: &str, direction: &str) -> bool {
        if !self.nodes.contains_key(from_id) || !self.nodes.contains_key(to_id) {
            return false;
        }
        if direction == "suffix" {
            let node = self.nodes.get_mut(from_id).unwrap();
            if !node.suffix.iter().any(|s| s == to_id) {
                node.suffix.push(to_id.to_string());
            }
        } else {
            let node = self.nodes.get_mut(from_id).unwrap();
            if !node.prefix.iter().any(|p| p == to_id) {
                node.prefix.push(to_id.to_string());
            }
        }
        self.connection_count += 1;
        true
    }

    /// 从起始节点遍历，生成全部合法组合路径
    pub fn traverse_from(&self, start_id: &str, direction: &str, max_depth: usize) -> Vec<Vec<String>> {
        if !self.nodes.contains_key(start_id) {
            return Vec::new();
        }
        let mut results = Vec::new();
        let mut path = vec![start_id.to_string()];
        self._dfs(start_id, &mut path, direction, max_depth, &mut results);
        results
    }

    fn _dfs(
        &self,
        current: &str,
        path: &mut Vec<String>,
        direction: &str,
        depth: usize,
        results: &mut Vec<Vec<String>>,
    ) {
        if depth == 0 {
            results.push(path.clone());
            return;
        }
        let node = match self.nodes.get(current) {
            Some(n) => n,
            None => {
                results.push(path.clone());
                return;
            }
        };
        let neighbors = if direction == "suffix" {
            &node.suffix
        } else {
            &node.prefix
        };
        if neighbors.is_empty() {
            results.push(path.clone());
            return;
        }
        for nb in neighbors {
            if !path.iter().any(|p| p == nb) {
                path.push(nb.clone());
                self._dfs(nb, path, direction, depth - 1, results);
                path.pop();
            }
        }
    }

    /// 区块链式传导：从起始节点向外辐射影响力（距离越远衰减）
    pub fn propagate(&self, start_id: &str) -> HashMap<String, f32> {
        let mut influence = HashMap::new();
        let mut visited = std::collections::HashSet::new();
        let mut queue: Vec<(String, f32)> = vec![(start_id.to_string(), 1.0)];
        while let Some((node_id, strength)) = queue.pop() {
            if visited.contains(&node_id) || strength < 0.01 {
                continue;
            }
            visited.insert(node_id.clone());
            influence.insert(node_id.clone(), strength);
            if let Some(node) = self.nodes.get(&node_id) {
                for nb in &node.suffix {
                    if !visited.contains(nb) {
                        queue.push((nb.clone(), strength * 0.7));
                    }
                }
            }
        }
        influence
    }

    /// 找到所有与当前节点可组合的节点
    pub fn find_composable(&self, node_id: &str) -> Vec<String> {
        if !self.nodes.contains_key(node_id) {
            return Vec::new();
        }
        let node = &self.nodes[node_id];
        self.nodes
            .iter()
            .filter(|(nid, _)| *nid != node_id)
            .filter(|(_, other)| node.pixel.compatible_with(&other.pixel).0)
            .map(|(nid, _)| nid.clone())
            .collect()
    }

    pub fn stats(&self) -> HashMap<String, f64> {
        let mut s = HashMap::new();
        s.insert("total_nodes".to_string(), self.nodes.len() as f64);
        s.insert("total_connections".to_string(), self.connection_count as f64);
        s.insert(
            "avg_connections".to_string(),
            if self.nodes.is_empty() {
                0.0
            } else {
                self.connection_count as f64 / self.nodes.len() as f64
            },
        );
        s
    }
}

impl Default for BlockchainIndex {
    fn default() -> BlockchainIndex {
        BlockchainIndex::new()
    }
}

impl fmt::Display for BlockchainIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Blockchain({} nodes, {} links)",
            self.nodes.len(),
            self.connection_count
        )
    }
}
