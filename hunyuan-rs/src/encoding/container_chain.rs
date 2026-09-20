// ══════════════════════════════════════════════════════════════════════
// container_chain.rs — 容器串联引擎 · 前字套壳=后字入口 (v0.45)
// ══════════════════════════════════════════════════════════════════════

use crate::presence_ontology::{AttendanceRecord, PresenceDomain, SymbolCategory};

pub type ChamberId = u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContainerCode {
    pub ch: char,
    pub chamber: u8,
}

impl ContainerCode {
    pub fn new(ch: char, chamber: u8) -> Self {
        assert!(chamber <= 3);
        Self { ch, chamber }
    }
    pub fn entry_chamber(&self) -> ChamberId { self.chamber as ChamberId }
    pub fn exit_chamber(&self) -> ChamberId { self.chamber as ChamberId }
}

pub fn chamber_name(id: ChamberId) -> &'static str {
    match id {
        0 => "杯(阳·夫)",
        1 => "盎(阴·妻)",
        2 => "盉(阳·妻)",
        3 => "瓶(阴·夫)",
        _ => "未知",
    }
}

pub fn container_entry_addr(code: &ContainerCode) -> ChamberId { code.entry_chamber() }
pub fn container_exit_addr(code: &ContainerCode) -> ChamberId { code.exit_chamber() }

/// 从字符串构建容器编码 (用于 river 管线等场景)
pub fn encode_chars_to_codes(input: &str) -> Vec<ContainerCode> {
    input.chars().filter(|c| !c.is_whitespace()).enumerate().map(|(i, ch)| {
        let chamber = (i % 4) as u8;
        ContainerCode::new(ch, chamber)
    }).collect()
}

/// 基于字符串构建容器链 (便捷函数)
pub fn build_chain_from_str(input: &str) -> ContainerChain {
    let codes = encode_chars_to_codes(input);
    ContainerChain::build(&codes)
}

/// 基于字符串获取入口/出口 (便捷函数)
pub fn glyph_entry_addr(ch: char) -> ChamberId { (ch as u64 % 4) as ChamberId }
pub fn glyph_exit_addr(ch: char) -> ChamberId { (ch as u64 % 4) as ChamberId }

#[derive(Clone, Debug)]
pub struct ContainerLink {
    pub prev: ContainerCode,
    pub next: ContainerCode,
    pub link_addr: ChamberId,
    pub intersection: AttendanceRecord,
}

#[derive(Clone, Debug)]
pub struct ContainerChain {
    pub encodings: Vec<ContainerCode>,
    pub links: Vec<ContainerLink>,
    pub total_intersections: u32,
    pub fully_linked: bool,
}

impl ContainerChain {
    pub fn build(input: &[ContainerCode]) -> Self {
        let mut links = Vec::new();
        for w in input.windows(2) {
            let prev = w[0];
            let next = w[1];
            let link_addr = container_exit_addr(&prev);
            let intersection = AttendanceRecord::intersection(
                "container-chain", SymbolCategory::Container,
                &format!("{}+{}", prev.ch, next.ch), "chain-link", links.len() as u64,
            );
            links.push(ContainerLink { prev, next, link_addr, intersection });
        }
        let fully_linked = links.iter().all(|l| {
            container_exit_addr(&l.prev) == container_entry_addr(&l.next)
        });
        Self { encodings: input.to_vec(), links, total_intersections: input.len().saturating_sub(1) as u32, fully_linked }
    }

    pub fn build_and_register(input: &[ContainerCode], domain: &mut PresenceDomain) -> Self {
        let chain = Self::build(input);
        for code in &chain.encodings {
            domain.attend(AttendanceRecord::global(
                &domain.name, SymbolCategory::Character,
                &format!("{}_{}", code.ch, chamber_name(code.chamber as ChamberId)),
                "<chain>", 0,
            ));
        }
        for link in &chain.links { domain.attend(link.intersection.clone()); }
        chain
    }
}

#[derive(Clone, Debug)]
pub struct LinkValidation { pub prev_char: char, pub next_char: char, pub link_addr: ChamberId, pub valid: bool, pub idx: usize }
#[derive(Clone, Debug)]
pub struct ChainValidation { pub results: Vec<LinkValidation>, pub chain_len: usize, pub all_valid: bool }

pub fn verify_chain(chain: &ContainerChain) -> ChainValidation {
    let mut results = Vec::new();
    for (i, link) in chain.links.iter().enumerate() {
        let valid = container_exit_addr(&link.prev) == container_entry_addr(&link.next);
        results.push(LinkValidation { prev_char: link.prev.ch, next_char: link.next.ch, link_addr: link.link_addr, valid, idx: i });
    }
    let all_valid = results.iter().all(|r| r.valid);
    let chain_len = chain.encodings.len();
    ChainValidation { results, chain_len, all_valid }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn code(ch: char, chamber: u8) -> ContainerCode { ContainerCode::new(ch, chamber) }

    #[test]
    fn test_chamber_name() {
        assert_eq!(chamber_name(0), "杯(阳·夫)");
        assert_eq!(chamber_name(3), "瓶(阴·夫)");
    }
    #[test]
    fn test_container_code() {
        let c = code('囡', 3);
        assert_eq!(c.ch, '囡');
        assert_eq!(c.chamber, 3);
        assert_eq!(container_entry_addr(&c), 3);
        assert_eq!(container_exit_addr(&c), 3);
    }
    #[test]
    fn test_build_chain_pair() {
        let chain = ContainerChain::build(&[code('囡', 3), code('囝', 3)]);
        assert_eq!(chain.encodings.len(), 2);
        assert_eq!(chain.links.len(), 1);
    }
    #[test]
    fn test_fully_linked_same() {
        let chain = ContainerChain::build(&[code('A', 3), code('B', 3), code('C', 3)]);
        assert!(chain.fully_linked);
    }
    #[test]
    fn test_not_fully_linked() {
        let chain = ContainerChain::build(&[code('A', 0), code('B', 3)]);
        assert!(!chain.fully_linked);
    }
    #[test]
    fn test_verify() {
        let chain = ContainerChain::build(&[code('A', 3), code('B', 3)]);
        let v = verify_chain(&chain);
        assert!(v.all_valid);
        assert_eq!(v.results.len(), 1);
    }
    #[test]
    fn test_build_and_register() {
        let mut d = PresenceDomain::new("t");
        let chain = ContainerChain::build_and_register(&[code('囡', 3), code('囝', 3)], &mut d);
        assert!(d.count_by_mode().total >= 3);
        assert_eq!(chain.links.len(), 1);
    }
    #[test]
    fn test_empty() {
        let chain = ContainerChain::build(&[]);
        assert_eq!(chain.links.len(), 0);
        assert!(chain.fully_linked);
    }
    #[test]
    fn test_single() {
        let chain = ContainerChain::build(&[code('X', 1)]);
        assert_eq!(chain.links.len(), 0);
        assert!(chain.fully_linked);
    }
    #[test]
    fn test_intersection_record() {
        let chain = ContainerChain::build(&[code('A', 2), code('B', 2)]);
        assert_eq!(chain.links[0].link_addr, 2);
    }
}
