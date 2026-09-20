// presence_ontology.rs — 在场本体论 (最小存根)
// 仅提供 container_chain / chain_resonance 需要的类型
// 原始完整实现已被移除, 此为编译兼容性存根

use std::path::Path;

/// 符号类别
#[derive(Clone, Copy, Debug)]
pub enum SymbolCategory { Character, Radical, Glyph, Container }

/// 在场模式
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PresenceMode { Global, Local, Floating }

/// 出席记录
#[derive(Clone, Debug)]
pub struct AttendanceRecord {
    pub symbol_name: String,
    pub domain: String,
    pub mode: PresenceMode,
    pub category: SymbolCategory,
    pub order: usize,
}

impl AttendanceRecord {
    pub fn global(domain: &str, category: SymbolCategory, name: impl Into<String>, _desc: impl Into<String>, order: usize) -> Self {
        Self { symbol_name: name.into(), domain: domain.to_string(), mode: PresenceMode::Global, category, order }
    }
    pub fn intersection(domain: &str, category: SymbolCategory, name: impl Into<String>, _desc: impl Into<String>, order: u64) -> Self {
        Self { symbol_name: name.into(), domain: domain.to_string(), mode: PresenceMode::Local, category, order: order as usize }
    }
}

/// 按模式统计结果
#[derive(Clone, Copy, Debug)]
pub struct ModeStats { pub total: u64, pub global: u64, pub local: u64, pub floating: u64, pub intersection: u64 }

/// 在场域
#[derive(Clone, Debug)]
pub struct PresenceDomain {
    pub name: String,
    pub attend_count: u64,
    pub global_count: u64,
    pub records: Vec<AttendanceRecord>,
}

impl PresenceDomain {
    pub fn new(name: &str) -> Self { Self { name: name.to_string(), attend_count: 0, global_count: 0, records: Vec::new() } }
    pub fn attend(&mut self, record: AttendanceRecord) {
        self.attend_count += 1;
        if record.mode == PresenceMode::Global { self.global_count += 1; }
        self.records.push(record);
    }
    pub fn count_by_mode(&self) -> ModeStats {
        let local = self.records.iter().filter(|r| r.mode == PresenceMode::Local).count() as u64;
        let intersection = local;
        ModeStats { total: self.attend_count, global: self.global_count, local, floating: 0, intersection }
    }
    pub fn roll_call(&self) -> Vec<(PresenceMode, String, String)> {
        self.records.iter().map(|r| (r.mode, format!("{:?}", r.category), r.symbol_name.clone())).collect()
    }
    pub fn retire_local(&mut self) -> usize {
        let before = self.records.len();
        self.records.retain(|r| r.mode != PresenceMode::Local);
        self.attend_count = self.records.len() as u64;
        self.global_count = self.records.iter().filter(|r| r.mode == PresenceMode::Global).count() as u64;
        before - self.records.len()
    }
    pub fn save_to_dir(&self, _dir: &Path) -> std::io::Result<usize> {
        Ok(self.records.len())
    }
    pub fn load_from_dir(&mut self, _dir: &Path) -> std::io::Result<usize> {
        Ok(0)
    }
}
