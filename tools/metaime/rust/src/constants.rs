//! Shared constants · 共享常量
//! All data is compile-time const, zero runtime memory.

/// 声母表 · 23+1 (含零声母)
pub const SHENG_MU: [&str; 24] = [
    "b", "p", "m", "f", "d", "t", "n", "l",
    "g", "k", "h", "j", "q", "x", "zh", "ch",
    "sh", "r", "z", "c", "s", "y", "w", "∅",
];

/// 韵母表 · 24
pub const YUN_MU: [&str; 24] = [
    "a", "o", "e", "i", "u", "v", "ai", "ei",
    "ui", "ao", "ou", "iu", "ie", "ve", "er", "an",
    "en", "in", "un", "vn", "ang", "eng", "ing", "ong",
];

/// 声调表 · 4
pub const TONES: [&str; 4] = ["1", "2", "3", "4"];

/// 身份序数名 · 首到二十四
pub const IDENTITY_NAMES: [&str; 24] = [
    "首", "次", "三", "四", "五", "六", "七", "八",
    "九", "十", "十一", "十二", "十三", "十四", "十五", "十六",
    "十七", "十八", "十九", "二十", "二十一", "二十二", "二十三", "二十四",
];

/// 外部符号映射表 · (外部符号, 本体论路径)
pub const FOREIGN_MAP: [(&str, &str); 47] = [
    // Python
    ("class", "袋.定义.容器"),
    ("def", "袋.定义.操作"),
    ("import", "袋.引用.外部"),
    ("from", "袋.引用.来源"),
    ("return", "袋.输出.结果"),
    ("if", "袋.判断.条件"),
    ("else", "袋.判断.否则"),
    ("for", "袋.循环.遍历"),
    ("while", "袋.循环.当"),
    ("in", "袋.关系.包含"),
    ("not", "袋.逻辑.非"),
    ("and", "袋.逻辑.与"),
    ("or", "袋.逻辑.或"),
    ("True", "袋.值.阳"),
    ("False", "袋.值.阴"),
    ("None", "袋.值.空"),
    ("self", "袋.自指.本体"),
    // C
    ("int", "袋.类型.整数"),
    ("char", "袋.类型.字符"),
    ("float", "袋.类型.小数"),
    ("void", "袋.类型.空"),
    ("struct", "袋.定义.结构"),
    ("typedef", "袋.定义.别名"),
    ("static", "袋.修饰.静态"),
    ("const", "袋.修饰.恒定"),
    ("sizeof", "袋.查询.大小"),
    ("pointer", "袋.引用.地址"),
    // Rust
    ("fn", "袋.定义.操作"),
    ("let", "袋.定义.绑定"),
    ("mut", "袋.修饰.可变"),
    ("impl", "袋.定义.实现"),
    ("trait", "袋.定义.特征"),
    ("match", "袋.判断.匹配"),
    ("Option", "袋.值.可选"),
    ("Result", "袋.值.结果"),
    ("Box", "袋.容器.堆"),
    ("Vec", "袋.容器.序列"),
    // 通用
    ("function", "袋.定义.操作"),
    ("variable", "袋.定义.绑定"),
    ("array", "袋.容器.序列"),
    ("string", "袋.容器.字符序列"),
    ("number", "袋.值.数字"),
    ("boolean", "袋.值.阴阳"),
    ("true", "袋.值.阳"),
    ("false", "袋.值.阴"),
    ("null", "袋.值.空"),
    ("undefined", "袋.值.无"),
];

/// 最大容器数量
pub const MAX_CONTAINERS: usize = 16;
/// 每个容器最大符号数
pub const MAX_SYMBOLS_PER_CONTAINER: usize = 64;
/// 最大链接数量
pub const MAX_LINKS: usize = 64;
/// 最大近义词数
pub const MAX_SYNONYMS: usize = 32;
/// 近义词最大变体数
pub const MAX_VARIANTS: usize = 8;
