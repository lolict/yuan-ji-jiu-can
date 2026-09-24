// ════════════════════════════════════════════════════════════════
// container_runtime.rs — 容器操作系统运行时
// ════════════════════════════════════════════════════════════════
// 独立编译 · 不依赖 hunyuan-rs 库

use std::collections::HashMap;

/// 容器状态：64-bit SMRL 容器单元 + 壳路径 + 历史
#[derive(Clone)]
pub struct ContainerOS {
    cell: u64,
    depth: u32,
    shell_path: Vec<u8>,
    history: Vec<char>,
}

impl ContainerOS {
    pub fn new() -> Self {
        Self { cell: 0, depth: 0, shell_path: Vec::new(), history: Vec::new() }
    }

    pub fn cell(&self) -> u64 { self.cell }
    pub fn depth(&self) -> u32 { self.depth }
    pub fn shell_path(&self) -> &[u8] { &self.shell_path }
    pub fn history(&self) -> &[char] { &self.history }

    pub fn absorb(&mut self, container_bit: u8) {
        self.cell |= 1u64 << container_bit;
    }

    pub fn ingest(&mut self, strokes: &[u8]) -> u64 {
        if strokes.is_empty() { return self.cell; }
        let mut char_cell: u64 = 0;
        for &s in strokes { char_cell |= 1u64 << s; }
        self.cell |= char_cell;
        self.shell_path.push(strokes[0]);
        self.depth += 1;
        self.cell
    }

    pub fn merge(&mut self, other: &Self) {
        self.cell |= other.cell;
        self.depth = self.depth.max(other.depth);
    }

    pub fn arbitrate(&self, other: &Self, witness: &Self) -> ArbitrationResult {
        let shared_self = self.cell & witness.cell;
        let shared_other = other.cell & witness.cell;

        if shared_self == 0 && shared_other == 0 {
            return ArbitrationResult::Neutral;
        }

        let cnt_self = shared_self.count_ones();
        let cnt_other = shared_other.count_ones();

        if cnt_self > cnt_other { ArbitrationResult::FavorSelf }
        else if cnt_other > cnt_self { ArbitrationResult::FavorOther }
        else { ArbitrationResult::Symphony }
    }

    pub fn self_describe(&self) -> String {
        let active: Vec<u8> = (0..16u8).filter(|&i| (self.cell >> i) & 1 == 1).collect();
        format!(
            "ContainerOS{{ depth={}, active={}, shell={}, cell=0x{:016X} }}",
            self.depth, active.len(), self.shell_path.len(), self.cell
        )
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.cell.to_le_bytes());
        out.extend_from_slice(&self.depth.to_le_bytes());
        out.push(self.shell_path.len() as u8);
        out.extend_from_slice(&self.shell_path);
        // 历史字符以 UTF-8 String 形式持久化 (避免逐字节转码乱码)
        let hist_str: String = self.history.iter().collect();
        let hist_bytes = hist_str.as_bytes();
        out.extend_from_slice(&(hist_bytes.len() as u32).to_le_bytes());
        out.extend_from_slice(hist_bytes);
        out
    }

    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 16 { return None; }
        let cell = u64::from_le_bytes(bytes[0..8].try_into().ok()?);
        let depth = u32::from_le_bytes(bytes[8..12].try_into().ok()?);
        let shell_len = bytes[12] as usize;
        if bytes.len() < 13 + shell_len { return None; }
        let shell_path = bytes[13..13 + shell_len].to_vec();
        let hist_start = 13 + shell_len;
        if bytes.len() < hist_start + 4 { return None; }
        let hist_len = u32::from_le_bytes(
            bytes[hist_start..hist_start + 4].try_into().ok()?
        ) as usize;
        let mut history = Vec::new();
        let hist_bytes = &bytes[hist_start + 4 .. hist_start + 4 + hist_len];
        match std::str::from_utf8(hist_bytes) {
            Ok(s) => { history.extend(s.chars()); }
            Err(_) => { /* 历史丢失, 继续 */ }
        }
        Some(Self { cell, depth, shell_path, history })
    }

    pub fn record_char(&mut self, ch: char) {
        self.history.push(ch);
    }

    pub fn active_count(&self) -> u32 {
        (self.cell & 0xFFFF).count_ones()
    }

    pub fn active_containers(&self) -> Vec<u8> {
        (0..16u8).filter(|&i| (self.cell >> i) & 1 == 1).collect()
    }

    pub fn reset(&mut self) {
        self.cell = 0;
        self.depth = 0;
        self.shell_path.clear();
        self.history.clear();
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArbitrationResult {
    Neutral,
    FavorSelf,
    FavorOther,
    Symphony,
}

impl std::fmt::Display for ArbitrationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Neutral => write!(f, "中立"),
            Self::FavorSelf => write!(f, "偏向甲方"),
            Self::FavorOther => write!(f, "偏向乙方"),
            Self::Symphony => write!(f, "共鸣"),
        }
    }
}

pub const OS_FORMAT_VERSION: u8 = 1;

pub struct OsHeader {
    pub version: u8,
    pub os: ContainerOS,
}

impl OsHeader {
    pub fn new(os: ContainerOS) -> Self {
        Self { version: OS_FORMAT_VERSION, os }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = vec![self.version];
        out.extend_from_slice(&self.os.serialize());
        out
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.is_empty() { return None; }
        let version = bytes[0];
        let os = ContainerOS::deserialize(&bytes[1..])?;
        Some(Self { version, os })
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        std::fs::write(path, self.to_bytes())
    }

    pub fn load(path: &str) -> std::io::Result<Option<Self>> {
        let bytes = std::fs::read(path)?;
        Ok(Self::from_bytes(&bytes))
    }
}
