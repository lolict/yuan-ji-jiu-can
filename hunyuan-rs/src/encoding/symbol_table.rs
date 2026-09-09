// ══════════════════════════════════════════════════════════════════
// symbol_table.rs — 符号表 (Symbol Table)
// ══════════════════════════════════════════════════════════════════
//
// 满全系统的"地球仪":每个 #0..#63 Symbol 对应一个具体的字形载体。
// 载体可来自:
//   · 预定义汉字 (诗经/道德经源头字形)
//   · 用户自定义的私有字形 (未来可从 64-1.txt 加载)
//   · 默认占位符 (Unicode 几何符号,纯视觉锚点)
//
// 关键设计:符号表本身可自举。即便没有任何外部字体,
// 系统内建的 fallback 字形仍然保证每个 Symbol 有一枚视觉代表。

use crate::topo::Symbol;

/// 字形载体 — 用 String 承载,兼容任何 Unicode 符号
pub type Glyph = String;

/// 符号表:索引 ↔ 字形
pub struct SymbolTable {
    entries: Vec<Glyph>,   // index 0..63
}

impl SymbolTable {
    /// 默认初始化:生成 fallback 字形表
    ///   当没有汉字可用时,使用几何符号占位,保证视觉锚点
    pub fn new() -> Self {
        let entries: Vec<Glyph> = (0..64u8).map(|i| Self::fallback_glyph(i)).collect();
        SymbolTable { entries }
    }

    /// 从外部 64 行输入加载 (未来对接 64-1.txt)
    /// 要求: 恰好 64 行,每行一个 UTF-8 字形,按 #0..#63 排列
    pub fn from_lines(lines: &[&str]) -> Result<Self, SymbolTableError> {
        if lines.len() != 64 {
            return Err(SymbolTableError::WrongCount(lines.len()));
        }
        let entries: Vec<Glyph> = lines.iter().map(|l| l.trim().to_string()).collect();
        Ok(SymbolTable { entries })
    }

    /// 查: Symbol → 字形
    pub fn glyph(&self, s: Symbol) -> &str {
        &self.entries[s.as_u8() as usize]
    }

    /// 反向查: 字形 → Option<Symbol>
    pub fn find(&self, ch: &str) -> Option<Symbol> {
        self.entries.iter()
            .position(|g| g == ch)
            .and_then(|idx| Symbol::new(idx as u8))
    }

    /// 单符号取字形
    pub fn get(&self, idx: u8) -> Option<&str> {
        Symbol::new(idx).map(|s| self.glyph(s))
    }

    /// 把符号序列拼成字符串
    pub fn render(&self, symbols: &[Symbol]) -> String {
        symbols.iter().map(|s| self.glyph(*s).to_string()).collect()
    }

    /// 迭代器
    pub fn iter(&self) -> impl Iterator<Item = (Symbol, &str)> + '_ {
        self.entries.iter()
            .enumerate()
            .map(|(i, g)| (Symbol::new(i as u8).unwrap(), g.as_str()))
    }

    // ── fallback 字形生成 ──
    // 几何符号 + 干支 + 序数,保证即使没有任何外部字体也能渲染

    fn fallback_glyph(idx: u8) -> Glyph {
        // 前 22 用天干地支 (私有编码,不需外部字库)
        const HEAVENLY:  [char; 10] = ['甲','乙','丙','丁','戊','己','庚','辛','壬','癸'];
        const EARTHLY:   [char; 12] = ['子','丑','寅','卯','辰','巳','午','未','申','酉','戌','亥'];
        // 后续用 Unicode 几何符号
        const GEOMETRIC: [char; 32] = [
            '●','○','◐','◑','◒','◓','◔','◕','◖','◗','◴','◵','◶','◷','◌','◍',
            '◎','◉','◦','○','◯','⬤','⭕','◐','◑','◒','◓','◔','◕','◖','◗','◙',
        ];
        match idx {
            0..=9   => HEAVENLY[idx as usize].to_string(),
            10..=21 => EARTHLY[(idx - 10) as usize].to_string(),
            22..=53 => GEOMETRIC[(idx - 22) as usize].to_string(),
            _       => format!("[{idx:02}]"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SymbolTableError {
    WrongCount(usize),
}

impl std::fmt::Display for SymbolTableError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SymbolTableError::WrongCount(n) => write!(f, "符号表需要恰好 64 行,得到 {n}"),
        }
    }
}

impl std::error::Error for SymbolTableError {}

// ═══════════════════════════════════════════════════════════════
// 测试
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_table_has_64_entries() {
        let st = SymbolTable::new();
        for i in 0..64u8 {
            let s = Symbol::new(i).unwrap();
            assert!(!st.glyph(s).is_empty(), "#{i} 字形不应为空");
        }
    }

    #[test]
    fn first_10_are_heavenly_stems() {
        let st = SymbolTable::new();
        // #0 = 甲, #9 = 癸
        assert_eq!(st.get(0).unwrap(), "甲");
        assert_eq!(st.get(9).unwrap(), "癸");
    }

    #[test]
    fn next_12_are_earthly_branches() {
        let st = SymbolTable::new();
        // #10 = 子, #21 = 亥
        assert_eq!(st.get(10).unwrap(), "子");
        assert_eq!(st.get(21).unwrap(), "亥");
    }

    #[test]
    fn from_lines_requires_64() {
        let lines: Vec<&str> = (0..10).map(|i| "X").collect();
        let r = SymbolTable::from_lines(&lines);
        assert_eq!(r.err().unwrap(), SymbolTableError::WrongCount(10));
    }

    #[test]
    fn from_lines_64_ok() {
        let lines: Vec<&str> = (0..64).map(|i| {
            // 用天干 + 地支 + 几何 + 序号作为替身
            "●"
        }).collect();
        let r = SymbolTable::from_lines(&lines);
        assert!(r.is_ok());
    }

    #[test]
    fn render_syntax() {
        let st = SymbolTable::new();
        let syms = vec![
            Symbol::new(0).unwrap(),   // 甲
            Symbol::new(10).unwrap(),  // 子
            Symbol::new(22).unwrap(),  // ●
        ];
        let s = st.render(&syms);
        assert_eq!(s, "甲子●");
    }

    #[test]
    fn find_glyph() {
        let st = SymbolTable::new();
        // "甲" 应能找到 #0
        let found = st.find("甲");
        assert_eq!(found.unwrap().as_u8(), 0);
    }

    #[test]
    fn find_missing_returns_none() {
        let st = SymbolTable::new();
        assert!(st.find("✗").is_none());
    }

    #[test]
    fn iter_yields_64() {
        let st = SymbolTable::new();
        let collected: Vec<(Symbol, &str)> = st.iter().collect();
        assert_eq!(collected.len(), 64);
    }
}
