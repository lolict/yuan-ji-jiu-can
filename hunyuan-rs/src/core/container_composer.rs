// ╔══════════════════════════════════════════════════════════════════╗
// ║  v0.68 容器串联引擎                                              ║
// ║  前字套壳 = 后字入口 · 容器组合 · 加法融合 · 因果链验证            ║
// ║                                                                  ║
// ║  核心概念:                                                       ║
// ║  · 容器串联: 两个容器 A → B 表示 A 的壳成为 B 的入口              ║
// ║  · 加法统计法: 组合后重量 = 前容器重量 + 后容器重量                ║
// ║  · 因果链验证: 所有串联顺序可追溯且不可篡改                       ║
// ╚══════════════════════════════════════════════════════════════════╝

use crate::container_ir::{ContainerAtlas, ContainerKind, ContainerRef, RelationKind};
use std::collections::HashMap;

/// 串联规则类型
#[derive(Clone, Debug, PartialEq)]
pub enum CompositionRule {
    /// 壳入口: shell.container 成为 entry.container 的外壳
    ShellEntry { shell: ContainerRef, entry: ContainerRef },
    /// 加法融合: 两个容器合并，重量相加
    AdditiveMerge { a: ContainerRef, b: ContainerRef, fused_weight: u64 },
    /// 因果延续: 前容器因果末端 → 后容器因果起端
    CausalLink { predecessor: ContainerRef, successor: ContainerRef, causal_key: u64 },
}

/// 串联引擎
pub struct Composer {
    /// 所有已注册的串联规则
    rules: Vec<CompositionRule>,
    /// 壳映射: shell → 被包裹的 entries
    shell_map: HashMap<u128, Vec<u128>>,
    /// 入口映射: entry → 包裹它的 shells
    entry_map: HashMap<u128, Vec<u128>>,
    /// 加法重量表: 容器 raw → 累计加法重量
    additive_weight: HashMap<u128, u64>,
}

impl Composer {
    /// 创建空的串联引擎
    pub fn new() -> Self {
        Composer {
            rules: Vec::new(),
            shell_map: HashMap::new(),
            entry_map: HashMap::new(),
            additive_weight: HashMap::new(),
        }
    }

    /// 注册串联规则: 前字套壳 = 后字入口
    pub fn compose(&mut self, shell: ContainerRef, entry: ContainerRef) -> usize {
        let rule = CompositionRule::ShellEntry {
            shell,
            entry: entry.clone(),
        };
        self.rules.push(rule);

        // 更新映射
        self.shell_map
            .entry(shell.raw())
            .or_default()
            .push(entry.raw());
        self.entry_map
            .entry(entry.raw())
            .or_default()
            .push(shell.raw());

        // 加法融合
        let shell_w = self.additive_weight.get(&shell.raw()).copied().unwrap_or(1);
        let entry_w = self.additive_weight.get(&entry.raw()).copied().unwrap_or(1);
        let fused = shell_w + entry_w;
        self.additive_weight.insert(shell.raw(), fused);
        self.additive_weight.insert(entry.raw(), fused);

        self.rules.len() - 1
    }

    /// 添加因果链链接
    pub fn link_causal(&mut self, pred: ContainerRef, succ: ContainerRef, causal_key: u64) -> usize {
        let rule = CompositionRule::CausalLink {
            predecessor: pred.clone(),
            successor: succ.clone(),
            causal_key,
        };
        // 确保两个节点都在加法表中
        self.additive_weight.entry(pred.raw()).or_insert(1);
        self.additive_weight.entry(succ.raw()).or_insert(1);
        self.rules.push(rule);
        self.rules.len() - 1
    }

    /// 注册加法融合
    pub fn merge_additive(&mut self, a: ContainerRef, b: ContainerRef) -> u64 {
        let wa = self.additive_weight.get(&a.raw()).copied().unwrap_or(1);
        let wb = self.additive_weight.get(&b.raw()).copied().unwrap_or(1);
        let fused = wa + wb;

        self.rules.push(CompositionRule::AdditiveMerge {
            a: a.clone(),
            b: b.clone(),
            fused_weight: fused,
        });

        self.additive_weight.insert(a.raw(), fused);
        self.additive_weight.insert(b.raw(), fused);
        fused
    }

    /// 获取容器的加法累计重量
    pub fn weight_of(&self, r: &ContainerRef) -> u64 {
        self.additive_weight.get(&r.raw()).copied().unwrap_or(1)
    }

    /// 获取容器的壳 (所有包裹它的容器)
    pub fn shells_of(&self, r: &ContainerRef) -> &[u128] {
        static EMPTY: &[u128] = &[];
        self.entry_map.get(&r.raw()).map(|v| v.as_slice()).unwrap_or(EMPTY)
    }

    /// 获取容器的入口 (它包裹的所有容器)
    pub fn entries_of(&self, r: &ContainerRef) -> &[u128] {
        static EMPTY: &[u128] = &[];
        self.shell_map.get(&r.raw()).map(|v| v.as_slice()).unwrap_or(EMPTY)
    }

    /// 获取所有规则
    pub fn rules(&self) -> &[CompositionRule] {
        &self.rules
    }

    /// 规则数量
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// 总加法重量 (所有容器的加法统计)
    pub fn total_additive_weight(&self) -> u64 {
        self.additive_weight.values().sum()
    }

    /// 验证因果链完整: 所有因果链接中没有断链
    pub fn verify_causal_chain(&self) -> bool {
        for rule in &self.rules {
            if let CompositionRule::CausalLink { predecessor, successor, .. } = rule {
                // 前驱必须是已注册的壳或入口
                let pred_known = self.additive_weight.contains_key(&predecessor.raw());
                let succ_known = self.additive_weight.contains_key(&successor.raw());
                if !pred_known || !succ_known {
                    return false;
                }
            }
        }
        true
    }

    /// 获取完整的串联顺序 (DFS 拓扑排序 — 保证壳在入口前)
    pub fn composition_order(&self) -> Vec<u128> {
        let mut visited = std::collections::HashSet::new();
        let mut order = Vec::new();
        let mut temp = std::collections::HashSet::new();

        // 收集所有节点
        let mut all_nodes: Vec<u128> = self.shell_map.keys().copied().collect();
        for entries in self.shell_map.values() {
            for e in entries {
                if !all_nodes.contains(e) {
                    all_nodes.push(*e);
                }
            }
        }
        for rule in &self.rules {
            if let CompositionRule::CausalLink { predecessor, successor, .. } = rule {
                if !all_nodes.contains(&predecessor.raw()) {
                    all_nodes.push(predecessor.raw());
                }
                if !all_nodes.contains(&successor.raw()) {
                    all_nodes.push(successor.raw());
                }
            }
        }

        // DFS topological sort (reverse postorder — 壳在入口前)
        for node in &all_nodes {
            if !visited.contains(node) {
                self.dfs_topo(*node, &mut visited, &mut temp, &mut order);
            }
        }

        // 反向 = 壳在前
        order.reverse();
        order
    }

    fn dfs_topo(
        &self,
        node: u128,
        visited: &mut std::collections::HashSet<u128>,
        temp: &mut std::collections::HashSet<u128>,
        order: &mut Vec<u128>,
    ) {
        if visited.contains(&node) {
            return;
        }
        temp.insert(node);

        // 遍历壳的入口 (壳 → 入口的关系意味着壳必须在前)
        if let Some(entries) = self.shell_map.get(&node) {
            for entry in entries {
                if !visited.contains(entry) {
                    self.dfs_topo(*entry, visited, temp, order);
                }
            }
        }

        // 遍历因果链接的后继
        for rule in &self.rules {
            if let CompositionRule::CausalLink { predecessor, successor, .. } = rule {
                if predecessor.raw() == node && !visited.contains(&successor.raw()) {
                    self.dfs_topo(successor.raw(), visited, temp, order);
                }
            }
        }

        temp.remove(&node);
        visited.insert(node);
        order.push(node);
    }

    /// 将串联规则同步到容器图集
    pub fn sync_to_atlas(&self, atlas: &mut ContainerAtlas) {
        // 注册所有容器
        for rule in &self.rules {
            match rule {
                CompositionRule::ShellEntry { shell, entry } => {
                    atlas.register(shell.clone());
                    atlas.register(entry.clone());
                    atlas.link(shell.clone(), entry.clone(), RelationKind::ContainerOf, 1);
                }
                CompositionRule::AdditiveMerge { a, b, .. } => {
                    atlas.register(a.clone());
                    atlas.register(b.clone());
                    atlas.link(a.clone(), b.clone(), RelationKind::Resonance, 1);
                }
                CompositionRule::CausalLink { predecessor, successor, .. } => {
                    atlas.register(predecessor.clone());
                    atlas.register(successor.clone());
                    atlas.link(predecessor.clone(), successor.clone(), RelationKind::Causal, 1);
                }
            }
        }
    }
}

impl Default for Composer {
    fn default() -> Self {
        Self::new()
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_ref(entity: u64) -> ContainerRef {
        ContainerRef::new(ContainerKind::Maori, entity)
    }

    #[test]
    fn compose_creates_shell_entry() {
        let mut c = Composer::new();
        let a = mk_ref(100);
        let b = mk_ref(200);
        let idx = c.compose(a.clone(), b.clone());
        assert_eq!(idx, 0);
        assert_eq!(c.rule_count(), 1);
        assert!(c.shells_of(&b).contains(&a.raw()));
        assert!(c.entries_of(&a).contains(&b.raw()));
    }

    #[test]
    fn additive_weight_sums() {
        let mut c = Composer::new();
        let a = mk_ref(10);
        let b = mk_ref(20);
        c.compose(a.clone(), b.clone());
        // 前字重量 = 1 + 1 = 2, 后字重量 = 1 + 1 = 2
        assert_eq!(c.weight_of(&a), 2);
        assert_eq!(c.weight_of(&b), 2);
        assert_eq!(c.total_additive_weight(), 4);
    }

    #[test]
    fn multiple_compositions_accumulate() {
        let mut c = Composer::new();
        let a = mk_ref(1);
        let b = mk_ref(2);
        let d = mk_ref(3);

        c.compose(a.clone(), b.clone());  // a → b, w(a)=2, w(b)=2
        c.compose(b.clone(), d.clone());  // b → d, w(b)=2+1=3, w(d)=3

        assert!(c.weight_of(&a) >= 2);
        assert!(c.weight_of(&b) >= 2);
        assert!(c.weight_of(&d) >= 2);
    }

    #[test]
    fn causal_link_registered() {
        let mut c = Composer::new();
        let a = mk_ref(50);
        let b = mk_ref(60);
        c.link_causal(a.clone(), b.clone(), 0xDEAD);
        assert_eq!(c.rule_count(), 1);
        assert!(c.verify_causal_chain());
    }

    #[test]
    fn causal_chain_always_complete_after_link() {
        let mut c = Composer::new();
        let a = mk_ref(70);
        let b = mk_ref(80);
        c.link_causal(a.clone(), b.clone(), 0xBEEF);
        // link_causal 自动注册节点到加法表, 因果链始终完整
        assert!(c.verify_causal_chain());
    }

    #[test]
    fn merge_additive() {
        let mut c = Composer::new();
        let a = mk_ref(1);
        let b = mk_ref(2);
        let w = c.merge_additive(a.clone(), b.clone());
        assert_eq!(w, 2);
        assert_eq!(c.weight_of(&a), 2);
        assert_eq!(c.weight_of(&b), 2);
    }

    #[test]
    fn composition_order_topological() {
        let mut c = Composer::new();
        let a = mk_ref(1);
        let b = mk_ref(2);
        let d = mk_ref(3);
        c.compose(a.clone(), b.clone());
        c.compose(b.clone(), d.clone());

        let order = c.composition_order();
        // a 应该在 b 前, b 应该在 d 前
        let pos_a = order.iter().position(|x| *x == a.raw()).unwrap();
        let pos_b = order.iter().position(|x| *x == b.raw()).unwrap();
        let pos_d = order.iter().position(|x| *x == d.raw()).unwrap();
        assert!(pos_a < pos_b);
        assert!(pos_b < pos_d);
    }

    #[test]
    fn sync_to_atlas_creates_nodes_and_edges() {
        let mut c = Composer::new();
        let a = mk_ref(100);
        let b = mk_ref(200);
        c.compose(a.clone(), b.clone());

        let mut atlas = ContainerAtlas::new();
        c.sync_to_atlas(&mut atlas);
        assert_eq!(atlas.count(), 2);
        assert_eq!(atlas.edge_count(), 1);
    }

    #[test]
    fn compose_keyword_pipeline() {
        // 模拟关键词串联: 满全法 → 囡囝共生 → 意识路由
        let mut c = Composer::new();
        let kw1 = mk_ref(0x6211_0001); // 满全法
        let kw2 = mk_ref(0x56F2_0002); // 囡囝共生
        let kw3 = mk_ref(0x610F_0003); // 意识路由

        c.compose(kw1.clone(), kw2.clone());
        c.compose(kw2.clone(), kw3.clone());
        c.link_causal(kw3.clone(), kw1.clone(), 0xC1A1); // 因果末端回到起端

        assert_eq!(c.rule_count(), 3);
        assert!(c.verify_causal_chain());
    }
}
