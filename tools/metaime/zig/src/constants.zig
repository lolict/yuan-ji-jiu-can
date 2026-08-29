// constants.zig - 共享常量 · 零运行时内存
//
// 所有数据为编译时常量（comptime），嵌入二进制，无运行时分配

const std = @import("std");

// === 声母表（24个，含空声母 ∅）===
pub const SHENG_MU = [_][]const u8{
    "b", "p", "m", "f", "d", "t", "n", "l",
    "g", "k", "h", "j", "q", "x", "zh", "ch",
    "sh", "r", "z", "c", "s", "y", "w", "∅",
};
pub const SHENG_MU_COUNT = SHENG_MU.len;

// === 韵母表（24个）===
pub const YUN_MU = [_][]const u8{
    "a", "o", "e", "i", "u", "v", "ai", "ei",
    "ui", "ao", "ou", "iu", "ie", "ve", "er", "an",
    "en", "in", "un", "vn", "ang", "eng", "ing", "ong",
};
pub const YUN_MU_COUNT = YUN_MU.len;

// === 声调表（4个）===
pub const TONES = [_][]const u8{ "1", "2", "3", "4" };
pub const TONES_COUNT = TONES.len;

// === 身份名表（首、次、三...二十四）===
pub const IDENTITY_NAMES = [_][]const u8{
    "首", "次", "三", "四", "五", "六", "七", "八", "九", "十",
    "十一", "十二", "十三", "十四", "十五", "十六", "十七", "十八", "十九", "二十",
    "二十一", "二十二", "二十三", "二十四",
};
pub const IDENTITY_COUNT = IDENTITY_NAMES.len;

// === 容器名常量 ===
pub const DOMAIN_SHENGMU = "声母";
pub const DOMAIN_YUNMU = "韵母";
pub const DOMAIN_TONE = "声调";

// === 域名匹配：按域名字符串返回索引 ===
pub const DomainIdx = enum(u8) {
    shengmu = 0,
    yunmu = 1,
    tone = 2,
};

/// 域名字符串 → 域索引
pub fn domainToIndex(comptime name: []const u8) ?DomainIdx {
    if (std.mem.eql(u8, name, DOMAIN_SHENGMU)) return .shengmu;
    if (std.mem.eql(u8, name, DOMAIN_YUNMU)) return .yunmu;
    if (std.mem.eql(u8, name, DOMAIN_TONE)) return .tone;
    return null;
}

/// 域索引 → 符号表
pub fn domainSymbols(comptime idx: DomainIdx) []const []const u8 {
    return switch (idx) {
        .shengmu => &SHENG_MU,
        .yunmu => &YUN_MU,
        .tone => &TONES,
    };
}

/// 域索引 → 域名字符串
pub fn domainName(comptime idx: DomainIdx) []const u8 {
    return switch (idx) {
        .shengmu => DOMAIN_SHENGMU,
        .yunmu => DOMAIN_YUNMU,
        .tone => DOMAIN_TONE,
    };
}

// === 身份名查找：身份字符串 → 位置索引 ===
pub fn identityToIndex(comptime identity: []const u8) ?u8 {
    comptime var i: u8 = 0;
    inline while (i < IDENTITY_COUNT) : (i += 1) {
        if (std.mem.eql(u8, IDENTITY_NAMES[i], identity)) return i;
    }
    return null;
}

// === 符号查找：在指定符号表中查找符号位置 ===
pub fn symbolToIndex(comptime table: []const []const u8, comptime symbol: []const u8) ?u8 {
    comptime var i: u8 = 0;
    inline while (i < table.len) : (i += 1) {
        if (std.mem.eql(u8, table[i], symbol)) return i;
    }
    return null;
}

// === 外来语映射表 ===
pub const FOREIGN_MAP_KEYS = [_][]const u8{
    "class", "def", "import", "from", "return", "if", "else", "elif",
    "for", "while", "break", "continue", "pass", "try", "except", "finally",
    "with", "as", "raise", "assert", "yield", "lambda", "global", "nonlocal",
    "del", "in", "is", "not", "and", "or", "True", "False",
    "None", "print", "range", "len", "int", "str", "float", "list",
    "dict", "set", "tuple", "bool", "type",
};
pub const FOREIGN_MAP_VALS = [_][]const u8{
    "袋.定义.容器", "袋.定义.操作", "袋.定义.引入", "袋.定义.来源", "袋.定义.返回", "袋.判断.条件", "袋.判断.否则", "袋.判断.否则若",
    "袋.循环.遍历", "袋.循环.当", "袋.循环.中断", "袋.循环.继续", "袋.循环.空", "袋.异常.尝试", "袋.异常.捕获", "袋.异常.最终",
    "袋.上下文.伴随", "袋.上下文.作为", "袋.异常.抛出", "袋.验证.断言", "袋.生成.产出", "袋.定义.匿名", "袋.作用域.全局", "袋.作用域.非局部",
    "袋.定义.删除", "袋.运算.包含", "袋.运算.同一", "袋.运算.非", "袋.运算.且", "袋.运算.或", "袋.常量.真", "袋.常量.假",
    "袋.常量.空", "袋.输出.打印", "袋.序列.范围", "袋.查询.长度", "袋.类型.整数", "袋.类型.字符串", "袋.类型.浮点", "袋.类型.列表",
    "袋.类型.字典", "袋.类型.集合", "袋.类型.元组", "袋.类型.布尔", "袋.类型.类型",
};
pub const FOREIGN_MAP_COUNT = FOREIGN_MAP_KEYS.len;

// === 最大容量 ===
pub const MAX_CONTAINERS = 8;
pub const MAX_SYMBOLS = 64;
pub const MAX_LINKS = 32;
pub const MAX_SYNONYMS = 16;
pub const MAX_IDENTITY_LEN = 24;
pub const MAX_SYMBOL_LEN = 4;
pub const MAX_DOMAIN_LEN = 16;

// === 容量常量（来自 containers.fqm）===
pub const SHENGMU_CAPACITY = 23;
pub const YUNMU_CAPACITY = 24;
pub const TONE_CAPACITY = 4;
pub const GLOBAL_OFFSET = 0x00;

// === 极性常量 ===
pub const POLARITY_YANG = 1;
pub const POLARITY_YIN = 0;

test "constants_shengmu_count" {
    try std.testing.expectEqual(@as(usize, 24), SHENG_MU_COUNT);
}

test "constants_yunmu_count" {
    try std.testing.expectEqual(@as(usize, 24), YUN_MU_COUNT);
}

test "constants_tones_count" {
    try std.testing.expectEqual(@as(usize, 4), TONES_COUNT);
}

test "domain_index_lookup" {
    try std.testing.expectEqual(DomainIdx.shengmu, domainToIndex("声母").?);
    try std.testing.expectEqual(DomainIdx.yunmu, domainToIndex("韵母").?);
    try std.testing.expectEqual(DomainIdx.tone, domainToIndex("声调").?);
    try std.testing.expect(domainToIndex("未知") == null);
}

test "identity_index_lookup" {
    try std.testing.expectEqual(@as(u8, 0), identityToIndex("首").?);
    try std.testing.expectEqual(@as(u8, 1), identityToIndex("次").?);
    try std.testing.expectEqual(@as(u8, 2), identityToIndex("三").?);
    try std.testing.expect(identityToIndex("不存在") == null);
}

test "symbol_index_lookup" {
    try std.testing.expectEqual(@as(u8, 0), symbolToIndex(&SHENG_MU, "b").?);
    try std.testing.expectEqual(@as(u8, 22), symbolToIndex(&SHENG_MU, "w").?);
    try std.testing.expectEqual(@as(u8, 0), symbolToIndex(&YUN_MU, "a").?);
    try std.testing.expectEqual(@as(u8, 23), symbolToIndex(&YUN_MU, "ong").?);
}

test "domain_symbols_match" {
    try std.testing.expectEqual(&SHENG_MU, domainSymbols(.shengmu));
    try std.testing.expectEqual(&YUN_MU, domainSymbols(.yunmu));
    try std.testing.expectEqual(&TONES, domainSymbols(.tone));
}
