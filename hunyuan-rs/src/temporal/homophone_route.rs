// ── v0.42 谐音路由 — 声母遍历韵母 · 桥接寻路 ──
// 符号通过共享韵母在网络中跳跃传递
// 满全法协议：谐音桥 = 同韵连接 · 声母距离 = 跳跃代价

use crate::phase_clock2::PhaseClock;
use std::collections::{HashMap, HashSet, VecDeque};

/// 声母距离（满全法协议：表中位置差）
fn initial_distance(a: usize, b: usize) -> usize {
    if a > b { a - b } else { b - a }
}

/// 韵母空间中的一个节点（韵母 = 空间坐标）
#[derive(Debug, Clone)]
pub struct RhymeNode {
    pub rhyme: String,
    /// 此韵母下活跃的声母列表
    pub initials: Vec<usize>,
    /// 到达此节的最小累计代价
    pub min_cost: u64,
}

impl RhymeNode {
    pub fn new(rhyme: &str) -> Self {
        Self {
            rhyme: rhyme.to_string(),
            initials: Vec::new(),
            min_cost: u64::MAX,
        }
    }
}

/// 谐音桥 — 两个韵母之间的连接
#[derive(Debug, Clone)]
pub struct HomophoneBridge {
    pub from_rhyme: String,
    pub to_rhyme: String,
    /// 桥接权重 = 共享声母数 × 声韵协和度
    pub weight: u64,
}

impl HomophoneBridge {
    pub fn new(from: &str, to: &str, weight: u64) -> Self {
        Self {
            from_rhyme: from.to_string(),
            to_rhyme: to.to_string(),
            weight,
        }
    }
}

/// 路由路径
#[derive(Debug, Clone)]
pub struct RoutePath {
    /// 经过的韵母序列
    pub rhyme_chain: Vec<String>,
    /// 经过的声母跳跃序列
    pub initial_hops: Vec<usize>,
    /// 总代价
    pub total_cost: u64,
}

impl RoutePath {
    pub fn new() -> Self {
        Self {
            rhyme_chain: Vec::new(),
            initial_hops: Vec::new(),
            total_cost: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.rhyme_chain.is_empty()
    }

    pub fn hops(&self) -> usize {
        self.initial_hops.len()
    }
}

impl Default for RoutePath {
    fn default() -> Self {
        Self::new()
    }
}

/// 谐音桥网络
#[derive(Debug)]
pub struct HomophoneNetwork {
    /// 韵母 → 声母位置 映射
    pub rhyme_to_initials: HashMap<String, HashSet<usize>>,
    /// 声母位置 → 韵母 反向映射
    pub initial_to_rhymes: HashMap<usize, HashSet<String>>,
    /// 已建立的谐音桥
    pub bridges: Vec<HomophoneBridge>,
    /// 最大跳跃数限制
    pub max_hops: usize,
}

impl HomophoneNetwork {
    pub fn new() -> Self {
        Self {
            rhyme_to_initials: HashMap::new(),
            initial_to_rhymes: HashMap::new(),
            bridges: Vec::new(),
            max_hops: 5,
        }
    }

    pub fn with_max_hops(mut self, hops: usize) -> Self {
        self.max_hops = hops;
        self
    }

    /// 注册符号：建立 声母↔韵母 双向映射
    pub fn register_symbol(&mut self, initial_pos: usize, rhyme: &str) {
        self.rhyme_to_initials
            .entry(rhyme.to_string())
            .or_insert_with(HashSet::new)
            .insert(initial_pos);
        self.initial_to_rhymes
            .entry(initial_pos)
            .or_insert_with(HashSet::new)
            .insert(rhyme.to_string());
    }

    /// 建立桥接：两个韵母共享声母 → 鹊桥连接
    pub fn build_bridge(&mut self, rhyme_a: &str, rhyme_b: &str) -> bool {
        let shared: HashSet<usize> = match (
            self.rhyme_to_initials.get(rhyme_a),
            self.rhyme_to_initials.get(rhyme_b),
        ) {
            (Some(a), Some(b)) => a.intersection(b).copied().collect(),
            _ => return false,
        };

        if shared.is_empty() {
            return false;
        }

        // 权重 = 共享声母数 × 10（越低越优先 → 所以用 1/count）
        let weight = (shared.len() as u64) * 10;
        self.bridges.push(HomophoneBridge::new(rhyme_a, rhyme_b, weight));
        true
    }

    /// 自动发现所有可能的桥接
    pub fn auto_discover_bridges(&mut self) -> usize {
        let rhymes: Vec<String> = self.rhyme_to_initials.keys().cloned().collect();
        let mut count = 0;
        for i in 0..rhymes.len() {
            for j in (i + 1)..rhymes.len() {
                if self.build_bridge(&rhymes[i], &rhymes[j]) {
                    count += 1;
                }
            }
        }
        count
    }

    /// BFS 寻路：从源声母到目标声母的最低代价路径
    pub fn find_route(
        &self,
        source_initial: usize,
        target_initial: usize,
        phase: &PhaseClock,
    ) -> Option<RoutePath> {
        // 1. 获取源声母可达的韵母集合
        let source_rhymes = self.initial_to_rhymes.get(&source_initial)?;
        let target_rhymes = self.initial_to_rhymes.get(&target_initial)?;

        // 2. 快速路径：源和目标共享同一个韵母 → 直接连接
        for rhyme in source_rhymes {
            if target_rhymes.contains(rhyme) {
                let dist = initial_distance(source_initial, target_initial);
                let mut path = RoutePath::new();
                path.rhyme_chain.push(rhyme.clone());
                path.initial_hops.push(source_initial);
                path.initial_hops.push(target_initial);
                path.total_cost = dist as u64;
                return Some(path);
            }
        }

        // 3. BFS 在韵母图中寻路
        let mut visited: HashSet<String> = HashSet::new();
        let mut queue: VecDeque<(String, Vec<String>, u64)> = VecDeque::new();

        // 初始化队列：从源声母可达的所有韵母出发
        for rhyme in source_rhymes {
            let chain = vec![rhyme.clone()];
            visited.insert(rhyme.clone());
            queue.push_back((rhyme.clone(), chain, 0));
        }

        // 通过桥接传播
        while let Some((current_rhyme, chain, cost)) = queue.pop_front() {
            if chain.len() > self.max_hops {
                continue;
            }

            if target_rhymes.contains(&current_rhyme) {
                // 找到路径 → 构建 RoutePath
                let mut path = RoutePath::new();
                path.rhyme_chain = chain;
                path.initial_hops.push(source_initial);
                path.initial_hops.push(target_initial);
                path.total_cost = cost;
                // 加上目标声母距离代价
                path.total_cost += initial_distance(source_initial, target_initial) as u64;
                return Some(path);
            }

            // 通过邻接桥接扩展
            for bridge in &self.bridges {
                let next = if bridge.from_rhyme == current_rhyme {
                    Some(bridge.to_rhyme.clone())
                } else if bridge.to_rhyme == current_rhyme {
                    Some(bridge.from_rhyme.clone())
                } else {
                    None
                };

                if let Some(next_rhyme) = next {
                    if !visited.contains(&next_rhyme) {
                        visited.insert(next_rhyme.clone());
                        let mut new_chain = chain.clone();
                        new_chain.push(next_rhyme.clone());
                        queue.push_back((next_rhyme, new_chain, cost + bridge.weight));
                    }
                }
            }
        }

        None
    }

    /// 获取韵母的网络度（连接数）
    pub fn degree(&self, rhyme: &str) -> usize {
        self.rhyme_to_initials.get(rhyme).map_or(0, |s| s.len())
    }

    /// 获取网络中所有韵母数量
    pub fn rhyme_count(&self) -> usize {
        self.rhyme_to_initials.len()
    }

    /// 获取网络中所有声母数量
    pub fn initial_count(&self) -> usize {
        self.initial_to_rhymes.len()
    }
}

impl Default for HomophoneNetwork {
    fn default() -> Self {
        Self::new()
    }
}

// ── 单元测试 ──
#[cfg(test)]
mod tests {
    use super::*;
    use crate::phase_clock2::PhaseClock;

    #[test]
    fn test_network_new() {
        let net = HomophoneNetwork::new();
        assert_eq!(net.rhyme_count(), 0);
        assert_eq!(net.initial_count(), 0);
    }

    #[test]
    fn test_register_symbol() {
        let mut net = HomophoneNetwork::new();
        net.register_symbol(10, "ao");
        assert_eq!(net.rhyme_count(), 1);
        assert_eq!(net.initial_count(), 1);
    }

    #[test]
    fn test_build_bridge_shared_initial() {
        let mut net = HomophoneNetwork::new();
        net.register_symbol(10, "ao");  // 声母 10 ↔ 韵母 ao
        net.register_symbol(10, "an");  // 声母 10 ↔ 韵母 an
        let built = net.build_bridge("ao", "an");
        assert!(built);
    }

    #[test]
    fn test_build_bridge_no_shared() {
        let mut net = HomophoneNetwork::new();
        net.register_symbol(10, "ao");
        net.register_symbol(20, "an");
        let built = net.build_bridge("ao", "an");
        assert!(!built);
    }

    #[test]
    fn test_auto_discover() {
        let mut net = HomophoneNetwork::new();
        net.register_symbol(10, "ao");
        net.register_symbol(10, "an");
        net.register_symbol(10, "ai");
        // 三个韵母共享声母 10 → 3 个桥接: (ao,an), (ao,ai), (an,ai)
        let count = net.auto_discover_bridges();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_find_route_direct() {
        let mut net = HomophoneNetwork::new();
        net.register_symbol(10, "ao");
        net.register_symbol(11, "ao");  // 同韵母
        let phase = PhaseClock::new();
        let path = net.find_route(10, 11, &phase);
        assert!(path.is_some());
        let p = path.unwrap();
        assert_eq!(p.rhyme_chain.len(), 1);
        assert_eq!(p.total_cost, 1); // distance(10,11) = 1
    }

    #[test]
    fn test_find_route_via_bridge() {
        let mut net = HomophoneNetwork::new();
        // 路径: 声母10 -韵母a- 声母10 -韵母b- 声母11
        net.register_symbol(10, "a");
        net.register_symbol(10, "b");
        net.register_symbol(11, "b");
        net.build_bridge("a", "b");
        let phase = PhaseClock::new();
        let path = net.find_route(10, 11, &phase);
        assert!(path.is_some());
    }

    #[test]
    fn test_no_route_without_bridge() {
        let mut net = HomophoneNetwork::new();
        net.register_symbol(10, "a");
        net.register_symbol(11, "b");
        // 不建立桥接 → 寻路失败
        let phase = PhaseClock::new();
        let path = net.find_route(10, 11, &phase);
        assert!(path.is_none());
    }

    #[test]
    fn test_initial_distance_fn() {
        assert_eq!(initial_distance(10, 11), 1);
        assert_eq!(initial_distance(20, 10), 10);
    }

    #[test]
    fn test_degree() {
        let mut net = HomophoneNetwork::new();
        net.register_symbol(10, "ao");
        net.register_symbol(11, "ao");
        net.register_symbol(12, "ao");
        assert_eq!(net.degree("ao"), 3);
    }

    #[test]
    fn test_path_hops() {
        let mut net = HomophoneNetwork::new();
        net.register_symbol(10, "ao");
        net.register_symbol(11, "ao");
        let phase = PhaseClock::new();
        let p = net.find_route(10, 11, &phase).unwrap();
        assert_eq!(p.hops(), 2);
    }

    #[test]
    fn test_phase_unaffected() {
        // 相位不影响路由代价（仅作为上下文传入）
        let mut net = HomophoneNetwork::new();
        net.register_symbol(10, "ao");
        net.register_symbol(11, "ao");
        let phase = PhaseClock::new();
        let p = net.find_route(10, 11, &phase).unwrap();
        assert!(p.total_cost > 0);
    }
}
