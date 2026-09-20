// ══════════════════════════════════════════════════════════════════════
// chain_resonance.rs — 链路共振 · 壳组寻址 (v0.45)
// ══════════════════════════════════════════════════════════════════════

use crate::presence_ontology::{AttendanceRecord, PresenceDomain, SymbolCategory};
use crate::container_chain::{ContainerChain, ContainerCode, ChamberId, chamber_name};

#[derive(Clone, Debug)]
pub struct ChamberGroup {
    pub chamber_id: ChamberId,
    pub chamber_name: String,
    pub members: Vec<ContainerCode>,
}

#[derive(Clone, Debug)]
pub struct ResonanceGraph {
    pub groups: Vec<ChamberGroup>,
    pub total: usize,
}

impl ResonanceGraph {
    pub fn build(input: &[ContainerCode]) -> Self {
        let mut map: std::collections::HashMap<u8, Vec<ContainerCode>> = std::collections::HashMap::new();
        for code in input {
            map.entry(code.chamber).or_default().push(*code);
        }
        let mut groups: Vec<ChamberGroup> = map.into_iter()
            .map(|(chamber, members)| {
                let chamber_id = chamber as ChamberId;
                ChamberGroup {
                    chamber_id,
                    chamber_name: chamber_name(chamber_id).to_string(),
                    members,
                }
            })
            .collect();
        groups.sort_by(|a, b| b.members.len().cmp(&a.members.len()));
        Self { groups, total: input.len() }
    }

    pub fn largest_group(&self) -> Option<&ChamberGroup> {
        self.groups.first()
    }

    pub fn build_longest_chain(&self) -> Option<ContainerChain> {
        self.largest_group().map(|g| ContainerChain::build(&g.members))
    }

    pub fn build_all_chains(&self) -> Vec<ContainerChain> {
        self.groups.iter()
            .map(|g| ContainerChain::build(&g.members))
            .collect()
    }

    pub fn report(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("  共 {} 组共振 (总 {} 容器)\n", self.groups.len(), self.total));
        for g in &self.groups {
            let chars: String = g.members.iter().map(|c| c.ch).collect();
            out.push_str(&format!("  壳[{}] {}: [{}]\n", g.chamber_id, g.chamber_name, chars));
        }
        out
    }
}

pub fn resonate_and_register(input: &[ContainerCode], domain: &mut PresenceDomain) -> ResonanceGraph {
    let graph = ResonanceGraph::build(input);
    for chain in graph.build_all_chains() {
        let _ = chain;
        for code in input {
            domain.attend(AttendanceRecord::global(
                &domain.name,
                SymbolCategory::Character,
                &format!("{}_{}", code.ch, chamber_name(code.chamber as ChamberId)),
                "resonate",
                0,
            ));
        }
    }
    graph
}

#[derive(Clone, Copy, Debug)]
pub struct ResonanceStats {
    pub total_chars: usize,
    pub chamber_count: usize,
    pub max_chain_len: usize,
    pub fully_linked_groups: usize,
    pub isolated_chars: usize,
}

pub fn resonance_stats(graph: &ResonanceGraph) -> ResonanceStats {
    let total_chars = graph.total;
    let chamber_count = graph.groups.len();
    let max_chain_len = graph.groups.first().map(|g| g.members.len()).unwrap_or(0);
    let fully_linked_groups = graph.groups.iter()
        .filter(|g| {
            let chain = ContainerChain::build(&g.members);
            chain.fully_linked
        })
        .count();
    let isolated_chars = graph.groups.iter().filter(|g| g.members.len() == 1).count();
    ResonanceStats { total_chars, chamber_count, max_chain_len, fully_linked_groups, isolated_chars }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn code(ch: char, chamber: u8) -> ContainerCode {
        ContainerCode::new(ch, chamber)
    }

    #[test]
    fn test_resonance_graph_build() {
        let input = vec![code('囡', 3), code('囝', 3), code('刘', 1), code('楚', 3), code('恬', 3)];
        let graph = ResonanceGraph::build(&input);
        assert_eq!(graph.total, 5);
        assert_eq!(graph.groups.len(), 2);
        // largest should be chamber=3 (4 members)
        assert_eq!(graph.largest_group().unwrap().chamber_id, 3);
        assert_eq!(graph.largest_group().unwrap().members.len(), 4);
    }

    #[test]
    fn test_build_longest_chain() {
        let input = vec![code('A', 3), code('B', 3), code('C', 3)];
        let graph = ResonanceGraph::build(&input);
        let chain = graph.build_longest_chain().unwrap();
        assert_eq!(chain.encodings.len(), 3);
    }

    #[test]
    fn test_build_all_chains() {
        let input = vec![code('A', 0), code('B', 3), code('C', 3)];
        let graph = ResonanceGraph::build(&input);
        let chains = graph.build_all_chains();
        assert_eq!(chains.len(), 2);
    }

    #[test]
    fn test_resonance_stats() {
        let input = vec![code('囡', 3), code('囝', 3), code('刘', 1)];
        let graph = ResonanceGraph::build(&input);
        let stats = resonance_stats(&graph);
        assert_eq!(stats.total_chars, 3);
        assert_eq!(stats.chamber_count, 2);
        assert_eq!(stats.max_chain_len, 2);
    }

    #[test]
    fn test_isolated_chars() {
        let input = vec![code('A', 0), code('B', 1), code('C', 2)];
        let graph = ResonanceGraph::build(&input);
        let stats = resonance_stats(&graph);
        assert_eq!(stats.isolated_chars, 3);
    }

    #[test]
    fn test_empty_resonance() {
        let graph = ResonanceGraph::build(&[]);
        assert_eq!(graph.total, 0);
        assert_eq!(graph.groups.len(), 0);
        assert!(graph.largest_group().is_none());
    }

    #[test]
    fn test_resonate_and_register() {
        let mut domain = PresenceDomain::new("test-reso");
        let input = vec![code('囡', 3), code('囝', 3)];
        let graph = resonate_and_register(&input, &mut domain);
        assert_eq!(graph.groups.len(), 1);
        let s = domain.count_by_mode();
        assert!(s.global >= 2);
    }

    #[test]
    fn test_fully_linked_groups() {
        let input = vec![code('A', 3), code('B', 3), code('C', 3)];
        let graph = ResonanceGraph::build(&input);
        let stats = resonance_stats(&graph);
        assert_eq!(stats.fully_linked_groups, 1);
    }
}
