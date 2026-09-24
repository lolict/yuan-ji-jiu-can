// ╔══════════════════════════════════════════════════════════════════╗
// ║  v0.74 伏羲字典                                                  ║
// ║  汉字转容器编码 · 三维寻址 · 声母韵母声调 · 全表征统一查找         ║
// ║                                                                  ║
// ║  伏羲 = 字形 ↔ 数字的桥梁                                        ║
// ║  · 汉字 → ContainerCode (容器码: 家族/极性/结构/声母/韵母/声调)  ║
// ║  · 汉字 → MaoriCoord (毛利兰坐标: x/y/z 三维)                    ║
// ║  · 汉字 → ContainerRef (统一中间表示: u128)                       ║
// ║  · 反向查找: 容器码/坐标/引用 → 汉字                             ║
// ╚══════════════════════════════════════════════════════════════════╝

use crate::container_ir::{ContainerKind, ContainerRef};
use crate::distance_perception::ContainerCode;
use crate::mlil::{MaoriAddr, MaoriCoord};
use std::collections::HashMap;

/// 汉字全表征 — 一个汉字对应的所有编码形式
#[derive(Clone, Debug)]
pub struct FuxiChar {
    /// 汉字本身
    pub hanzi: String,
    /// Unicode 标量值
    pub unicode: u32,
    /// 容器编码 (家族/极性/结构/声母/韵母/声调)
    pub code: ContainerCode,
    /// 毛利兰立体坐标
    pub coord: MaoriCoord,
    /// 容器中间表示
    pub cref: ContainerRef,
    /// 汉字标签 (用于显示)
    pub label: String,
}

impl FuxiChar {
    pub fn new(hanzi: &str, unicode: u32, code: ContainerCode, coord: MaoriCoord, cref: ContainerRef) -> Self {
        FuxiChar {
            hanzi: hanzi.to_string(),
            unicode,
            code,
            coord,
            cref,
            label: hanzi.to_string(),
        }
    }

    /// 容器码原始值
    pub fn code_raw(&self) -> u32 {
        self.code.raw()
    }

    /// 容器引用原始值
    pub fn cref_raw(&self) -> u128 {
        self.cref.raw()
    }

    /// 毛利兰地址的线性值
    pub fn linear(&self) -> u64 {
        self.cref.entity()
    }

    /// 简短描述
    pub fn describe(&self) -> String {
        format!(
            "{} [U+{:04X}] ML[{}:{}:{}](oct={}) code={:08x}",
            self.hanzi,
            self.unicode,
            self.coord.x,
            self.coord.y,
            self.coord.z,
            self.cref.octant(),
            self.code_raw()
        )
    }
}

/// FNV-1a 32-bit hash
fn fnv1a_32(data: &[u8]) -> u32 {
    let mut hash: u32 = 0x811c9dc5;
    for &b in data {
        hash ^= b as u32;
        hash = hash.wrapping_mul(0x01000193);
    }
    hash
}

/// 汉字 → 3D 坐标
pub fn hanzi_to_coord(s: &str) -> MaoriCoord {
    let bytes = s.as_bytes();
    let h = fnv1a_32(bytes);
    let h2 = fnv1a_32(&[&[h.to_le_bytes()[0]], bytes].concat());
    let h3 = fnv1a_32(&[&[h.to_le_bytes()[1], h2.to_le_bytes()[0]], bytes].concat());

    MaoriCoord::new(
        (h % 32) as u32,
        (h2 % 32) as u32,
        (h3 % 32) as u32,
    )
}

/// 汉字 → 容器码 (从 Unicode 派生各字段)
pub fn hanzi_to_code(s: &str) -> ContainerCode {
    let bytes = s.as_bytes();
    let h = fnv1a_32(bytes);
    let h2 = fnv1a_32(&[bytes, &[1]].concat());

    // 从哈希值派生各字段, 保持分布均匀
    let family = ((h >> 29) & 0x7) as u8;        // 0-7
    let polarity = ((h >> 28) & 0x1) as u8;       // 0-1
    let structure = ((h >> 25) & 0x7) as u8;      // 0-7
    let shengmu = ((h >> 19) & 0xF) as u8;        // 0-15
    let yunmu = ((h2 >> 13) & 0x3F) as u8;        // 0-63
    let tone = ((h2 >> 10) & 0x7) as u8;          // 0-7

    ContainerCode::compose(family, polarity, structure, shengmu, yunmu, tone)
}

/// 汉字 → 容器引用
pub fn hanzi_to_cref(s: &str, kind: ContainerKind) -> ContainerRef {
    let bytes = s.as_bytes();
    let h = fnv1a_32(bytes);
    let entity = (h as u64) << 32 | (fnv1a_32(&[bytes, &[2]].concat()) as u64);
    let octant = ((h >> 29) & 0x7) as u8;
    ContainerRef::with_octant(kind, entity, octant)
}

/// 伏羲字典 — 统一的汉字↔编码查找表
pub struct FuxiDictionary {
    /// 汉字 → 全表征
    by_hanzi: HashMap<String, FuxiChar>,
    /// 容器码原始值 → 汉字索引列表
    by_code: HashMap<u32, Vec<String>>,
    /// 坐标 → 汉字索引列表
    by_coord: HashMap<u64, Vec<String>>,
    /// 容器引用原始值 → 汉字索引列表
    by_cref: HashMap<u128, Vec<String>>,
    /// 条目总数
    count: usize,
}

impl FuxiDictionary {
    pub fn new() -> Self {
        FuxiDictionary {
            by_hanzi: HashMap::new(),
            by_code: HashMap::new(),
            by_coord: HashMap::new(),
            by_cref: HashMap::new(),
            count: 0,
        }
    }

    /// 注册单个汉字
    pub fn register(&mut self, hanzi: &str) -> &FuxiChar {
        let unicode = hanzi.chars().next().map(|c| c as u32).unwrap_or(0);
        let code = hanzi_to_code(hanzi);
        let coord = hanzi_to_coord(hanzi);
        let cref = hanzi_to_cref(hanzi, ContainerKind::Maori);

        let fchar = FuxiChar::new(hanzi, unicode, code, coord, cref);

        self.by_code.entry(fchar.code_raw()).or_default().push(hanzi.to_string());
        self.by_coord.entry(((coord.x as u64) << 40) | ((coord.y as u64) << 20) | coord.z as u64)
            .or_default().push(hanzi.to_string());
        self.by_cref.entry(fchar.cref_raw()).or_default().push(hanzi.to_string());

        self.by_hanzi.insert(hanzi.to_string(), fchar);
        self.count = self.by_hanzi.len();
        self.by_hanzi.get(hanzi).unwrap()
    }

    /// 批量注册
    pub fn register_batch(&mut self, hanzis: &[&str]) {
        for &h in hanzis {
            self.register(h);
        }
    }

    /// 按汉字查找
    pub fn find_by_hanzi(&self, hanzi: &str) -> Option<&FuxiChar> {
        self.by_hanzi.get(hanzi)
    }

    /// 按容器码查找
    pub fn find_by_code(&self, code: u32) -> Vec<&FuxiChar> {
        self.by_code.get(&code)
            .map(|names| names.iter()
                .filter_map(|n| self.by_hanzi.get(n))
                .collect())
            .unwrap_or_default()
    }

    /// 按坐标查找
    pub fn find_by_coord(&self, coord: &MaoriCoord) -> Vec<&FuxiChar> {
        let key = ((coord.x as u64) << 40) | ((coord.y as u64) << 20) | coord.z as u64;
        self.by_coord.get(&key)
            .map(|names| names.iter()
                .filter_map(|n| self.by_hanzi.get(n))
                .collect())
            .unwrap_or_default()
    }

    /// 按坐标近似查找 (曼哈顿距离 ≤ radius)
    pub fn find_nearby(&self, coord: &MaoriCoord, radius: u32) -> Vec<&FuxiChar> {
        let mut results = Vec::new();
        for fchar in self.by_hanzi.values() {
            let md = ((fchar.coord.x as i64 - coord.x as i64).abs()
                + (fchar.coord.y as i64 - coord.y as i64).abs()
                + (fchar.coord.z as i64 - coord.z as i64).abs()) as u32;
            if md <= radius {
                results.push(fchar);
            }
        }
        results
    }

    /// 按容器引用查找
    pub fn find_by_cref(&self, cref: &ContainerRef) -> Vec<&FuxiChar> {
        self.by_cref.get(&cref.raw())
            .map(|names| names.iter()
                .filter_map(|n| self.by_hanzi.get(n))
                .collect())
            .unwrap_or_default()
    }

    /// 按家族字段过滤
    pub fn filter_by_family(&self, family: u8) -> Vec<&FuxiChar> {
        self.by_hanzi.values()
            .filter(|c| c.code.family() == family)
            .collect()
    }

    /// 按极性过滤
    pub fn filter_by_polarity(&self, polarity: u8) -> Vec<&FuxiChar> {
        self.by_hanzi.values()
            .filter(|c| c.code.polarity() == polarity)
            .collect()
    }

    /// 按声母过滤
    pub fn filter_by_shengmu(&self, shengmu: u8) -> Vec<&FuxiChar> {
        self.by_hanzi.values()
            .filter(|c| c.code.shengmu() == shengmu)
            .collect()
    }

    /// 字典条目数
    pub fn count(&self) -> usize {
        self.count
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// 全部条目
    pub fn all(&self) -> impl Iterator<Item = &FuxiChar> {
        self.by_hanzi.values()
    }

    /// 全部汉字
    pub fn all_hanzi(&self) -> impl Iterator<Item = &String> {
        self.by_hanzi.keys()
    }
}

impl Default for FuxiDictionary {
    fn default() -> Self {
        Self::new()
    }
}

/// 常用汉字表 — 囡囝共生哲学关键词
pub const NJSG_KEYWORDS: &[&str] = &[
    "满", "全", "法", "囡", "因", "共", "生",
    "夫", "妻", "命", "运", "同", "体", "心",
    "意", "识", "互", "联", "网", "路", "由",
    "时", "钟", "动", "作", "容", "器", "编",
    "码", "寻", "址", "进", "制", "判", "断",
    "评", "价", "观", "察", "反", "馈", "迂",
    "回", "跳", "蚕", "蛹", "寄", "神", "话",
    "伏", "羲", "八", "卦", "阴", "阳", "二",
    "加", "法", "统", "计", "验", "证", "序",
];

/// 预建囡囝共生字典
pub fn build_njsg_dictionary() -> FuxiDictionary {
    let mut dict = FuxiDictionary::new();
    dict.register_batch(NJSG_KEYWORDS);
    dict
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::container_ir::ContainerKind;

    #[test]
    fn hanzi_to_coord_deterministic() {
        let a = hanzi_to_coord("满");
        let b = hanzi_to_coord("满");
        assert_eq!(a.x, b.x);
        assert_eq!(a.y, b.y);
        assert_eq!(a.z, b.z);
    }

    #[test]
    fn hanzi_to_coord_different() {
        let a = hanzi_to_coord("满");
        let b = hanzi_to_coord("全");
        // 大概率不同 (小概率碰撞忽略)
        assert!((a.x != b.x) || (a.y != b.y) || (a.z != b.z));
    }

    #[test]
    fn hanzi_to_code_fields_in_range() {
        let code = hanzi_to_code("法");
        assert!(code.family() <= 7);
        assert!(code.polarity() <= 1);
        assert!(code.structure() <= 7);
        assert!(code.shengmu() <= 15);
        assert!(code.yunmu() <= 63);
        assert!(code.tone() <= 7);
    }

    #[test]
    fn hanzi_to_cref_valid() {
        let cref = hanzi_to_cref("生", ContainerKind::Maori);
        assert_eq!(cref.kind(), Some(ContainerKind::Maori));
        assert!(cref.entity() > 0);
    }

    #[test]
    fn fuxi_char_describe() {
        let code = hanzi_to_code("满");
        let coord = hanzi_to_coord("满");
        let cref = hanzi_to_cref("满", ContainerKind::Maori);
        let fc = FuxiChar::new("满", 0x6EE1, code, coord, cref);
        let desc = fc.describe();
        assert!(desc.contains("满"));
        assert!(desc.contains("6EE1") || desc.contains("6ee1") || desc.contains("U+"));
    }

    #[test]
    fn dictionary_register_and_find() {
        let mut dict = FuxiDictionary::new();
        dict.register("满");
        dict.register("全");
        dict.register("法");

        assert_eq!(dict.count(), 3);

        let fc = dict.find_by_hanzi("满").unwrap();
        assert_eq!(fc.hanzi, "满");
        assert_eq!(fc.code.family(), hanzi_to_code("满").family());
    }

    #[test]
    fn dictionary_find_empty() {
        let dict = FuxiDictionary::new();
        assert!(dict.find_by_hanzi("满").is_none());
        assert!(dict.is_empty());
    }

    #[test]
    fn dictionary_filter_by_family() {
        let mut dict = FuxiDictionary::new();
        dict.register_batch(&["满", "全", "法", "囡", "共", "生"]);

        // 按家族 3 过滤 (应该有一些命中的)
        let result = dict.filter_by_family(3);
        // 不断言数量, 因为哈希分布随机, 但至少不断言 panic
        let _ = result.len();
    }

    #[test]
    fn dictionary_filter_by_polarity() {
        let mut dict = FuxiDictionary::new();
        dict.register_batch(&["满", "全", "法"]);

        let yin = dict.filter_by_polarity(0);
        let yang = dict.filter_by_polarity(1);
        // 两个极性加起来应该等于总数 (每个字非阴即阳)
        assert_eq!(yin.len() + yang.len(), 3);
    }

    #[test]
    fn dictionary_find_by_code() {
        let mut dict = FuxiDictionary::new();
        dict.register("测");
        let code = hanzi_to_code("测");

        let results = dict.find_by_code(code.raw());
        assert!(results.iter().any(|c| c.hanzi == "测"));
    }

    #[test]
    fn dictionary_find_by_coord() {
        let mut dict = FuxiDictionary::new();
        dict.register("试");
        let coord = hanzi_to_coord("试");

        let results = dict.find_by_coord(&coord);
        assert!(results.iter().any(|c| c.hanzi == "试"));
    }

    #[test]
    fn dictionary_find_nearby() {
        let mut dict = FuxiDictionary::new();
        dict.register_batch(&["A", "B", "C"]);

        // 查找一个坐标附近
        let center = hanzi_to_coord("A");
        let results = dict.find_nearby(&center, 100); // 大半径
        assert!(results.len() >= 1);
    }

    #[test]
    fn dictionary_find_by_cref() {
        let mut dict = FuxiDictionary::new();
        dict.register("字");
        let cref = hanzi_to_cref("字", ContainerKind::Maori);

        let results = dict.find_by_cref(&cref);
        assert!(results.iter().any(|c| c.hanzi == "字"));
    }

    #[test]
    fn dictionary_register_batch() {
        let mut dict = FuxiDictionary::new();
        dict.register_batch(&["囡", "囝", "共", "生"]);
        assert_eq!(dict.count(), 4);
    }

    #[test]
    fn build_njsg_dictionary_has_entries() {
        let dict = build_njsg_dictionary();
        assert!(dict.count() > 0);
    }

    #[test]
    fn dictionary_all_iter() {
        let mut dict = FuxiDictionary::new();
        dict.register_batch(&["一", "二", "三"]);

        let mut count = 0;
        for _ in dict.all() {
            count += 1;
        }
        assert_eq!(count, 3);
    }

    #[test]
    fn code_fields_consistent_after_register() {
        let mut dict = FuxiDictionary::new();
        let fc = dict.register("验");

        // 注册后的表征应一致
        let code = hanzi_to_code("验");
        assert_eq!(fc.code_raw(), code.raw());

        let coord = hanzi_to_coord("验");
        assert_eq!(fc.coord.x, coord.x);
        assert_eq!(fc.coord.y, coord.y);
        assert_eq!(fc.coord.z, coord.z);
    }
}
