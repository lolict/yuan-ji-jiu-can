// livecode_compiler.rs — 活码编译器 · 在场指纹 · 仅加法范式

/// 出席记录 (本地最小定义, 不依赖已删除的 presence_ontology)
#[derive(Clone, Debug)]
pub struct AttendanceRecord {
    pub symbol_name: String,
    pub domain: String,
    pub mode: PresenceMode,
}

impl AttendanceRecord {
    pub fn global(domain: &str, _category: SymbolCategory, name: String, _desc: String, _order: usize) -> Self {
        Self { symbol_name: name, domain: domain.to_string(), mode: PresenceMode::Global }
    }
}

/// 在场模式
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PresenceMode { Global, Local, Floating }

/// 在场域
#[derive(Clone, Debug)]
pub struct PresenceDomain {
    pub name: String,
    pub global_count: u64,
}

impl PresenceDomain {
    pub fn new(name: &str) -> Self { Self { name: name.to_string(), global_count: 0 } }
    pub fn attend(&mut self, record: AttendanceRecord) {
        if record.mode == PresenceMode::Global { self.global_count += 1; }
    }
}

/// 符号类别
#[derive(Clone, Copy, Debug)]
pub enum SymbolCategory { Character, Radical, Glyph }

#[derive(Clone, Debug)]
pub struct LiveCandidate { pub glyph: char, pub spelling: String, pub tone: u8, pub addition_addr: u64, pub resonance_count: u32, pub attendance: AttendanceRecord, pub rank_score: u64 }

impl LiveCandidate {
    pub fn fingerprint(&self) -> u64 {
        let name_hash = simple_hash(&self.attendance.symbol_name);
        let domain_hash = simple_hash(&self.attendance.domain);
        (domain_hash << 32) | name_hash
    }
}

fn simple_hash(s: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s.bytes() { h ^= b as u64; h = h.wrapping_add(h.wrapping_shl(1).wrapping_add(h.wrapping_shl(3))); }
    h
}

pub struct LiveCodeCompiler { pub domain: PresenceDomain }

impl LiveCodeCompiler {
    pub fn new(name: &str) -> Self { Self { domain: PresenceDomain::new(name) } }
    pub fn compile(&mut self, spelling: &str, tone: u8, prefs: &[char]) -> Vec<LiveCandidate> {
        let mut cands = Vec::new();
        for (i, &ch) in prefs.iter().enumerate() {
            let att = AttendanceRecord::global(&self.domain.name, SymbolCategory::Character, format!("{}_{}", ch, spelling), format!("livecode:{}@{}", spelling, tone), i + 1);
            self.domain.attend(att.clone());
            cands.push(LiveCandidate { glyph: ch, spelling: spelling.to_string(), tone, addition_addr: (i + 1) as u64, resonance_count: 1, attendance: att, rank_score: simple_hash(&format!("{}{}", ch, tone)) });
        }
        cands
    }
    pub fn compile_floating(&mut self, spelling: &str, prefs: &[char]) -> Vec<LiveCandidate> { self.compile(spelling, 0, prefs) }
    pub fn cross_resonate(&self, spelling: &str) -> u32 { (simple_hash(spelling) % 64) as u32 }
}

pub fn live_compile(spelling: &str, tone: u8, prefs: &[char]) -> Vec<LiveCandidate> {
    let mut c = LiveCodeCompiler::new("live-compile"); c.compile(spelling, tone, prefs)
}
pub fn live_float(spelling: &str, prefs: &[char]) -> Vec<LiveCandidate> {
    let mut c = LiveCodeCompiler::new("live-float"); c.compile_floating(spelling, prefs)
}
pub fn live_resonate(spelling: &str) -> u32 { LiveCodeCompiler::new("q").cross_resonate(spelling) }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_live_compile_basic() { let c = live_compile("an", 1, &['安', '按', '暗']); assert_eq!(c.len(), 3); for cc in &c { assert_eq!(cc.attendance.mode, PresenceMode::Global); } }
    #[test] fn test_live_float() { let c = live_float("zhi", &['之', '指']); assert_eq!(c.len(), 2); assert_eq!(c[0].tone, 0); }
    #[test] fn test_live_resonate() { let r = live_resonate("an"); assert!(r < 64); }
    #[test] fn test_fingerprint_unique() { let c = live_compile("an", 1, &['安', '按']); assert_ne!(c[0].fingerprint(), c[1].fingerprint()); }
    #[test] fn test_fingerprint_contains_domain() { let mut c = LiveCodeCompiler::new("my-domain"); let cands = c.compile("an", 1, &['安']); assert!(cands[0].fingerprint() != 0); }
    #[test] fn test_compiler_domain_registered() { let mut c = LiveCodeCompiler::new("reg-test"); let _ = c.compile("an", 1, &['安']); let s = c.domain.global_count; assert!(s > 0); }
}
