// translator.zig - 外部符号翻译器 · 零运行时内存
//
// 将外部符号（class/def/import 等）翻译为本体论路径
// 约束AI输出通道：只有通过翻译器的输出才会被执行

const std = @import("std");
const constants = @import("constants.zig");

// === 数据结构 ===

pub const TranslationResult = struct {
    foreign: []const u8,
    ontology: []const u8,
    rejected: bool,
};

// === 翻译表 ===

/// 翻译外部符号到本体论路径
pub fn translate(comptime foreign_symbol: []const u8) ?[]const u8 {
    // Python keywords
    if (std.mem.eql(u8, foreign_symbol, "class")) return "袋.定义.容器";
    if (std.mem.eql(u8, foreign_symbol, "def")) return "袋.定义.操作";
    if (std.mem.eql(u8, foreign_symbol, "import")) return "袋.引用.外部";
    if (std.mem.eql(u8, foreign_symbol, "from")) return "袋.引用.来源";
    if (std.mem.eql(u8, foreign_symbol, "return")) return "袋.输出.结果";
    if (std.mem.eql(u8, foreign_symbol, "if")) return "袋.判断.条件";
    if (std.mem.eql(u8, foreign_symbol, "else")) return "袋.判断.否则";
    if (std.mem.eql(u8, foreign_symbol, "for")) return "袋.循环.遍历";
    if (std.mem.eql(u8, foreign_symbol, "while")) return "袋.循环.当";
    if (std.mem.eql(u8, foreign_symbol, "in")) return "袋.关系.包含";
    if (std.mem.eql(u8, foreign_symbol, "not")) return "袋.逻辑.非";
    if (std.mem.eql(u8, foreign_symbol, "and")) return "袋.逻辑.与";
    if (std.mem.eql(u8, foreign_symbol, "or")) return "袋.逻辑.或";
    if (std.mem.eql(u8, foreign_symbol, "True")) return "袋.值.阳";
    if (std.mem.eql(u8, foreign_symbol, "False")) return "袋.值.阴";
    if (std.mem.eql(u8, foreign_symbol, "None")) return "袋.值.空";
    if (std.mem.eql(u8, foreign_symbol, "self")) return "袋.自指.本体";

    // C keywords
    if (std.mem.eql(u8, foreign_symbol, "int")) return "袋.类型.整数";
    if (std.mem.eql(u8, foreign_symbol, "char")) return "袋.类型.字符";
    if (std.mem.eql(u8, foreign_symbol, "float")) return "袋.类型.小数";
    if (std.mem.eql(u8, foreign_symbol, "void")) return "袋.类型.空";
    if (std.mem.eql(u8, foreign_symbol, "struct")) return "袋.定义.结构";
    if (std.mem.eql(u8, foreign_symbol, "typedef")) return "袋.定义.别名";
    if (std.mem.eql(u8, foreign_symbol, "static")) return "袋.修饰.静态";
    if (std.mem.eql(u8, foreign_symbol, "const")) return "袋.修饰.恒定";
    if (std.mem.eql(u8, foreign_symbol, "sizeof")) return "袋.查询.大小";
    if (std.mem.eql(u8, foreign_symbol, "pointer")) return "袋.引用.地址";

    // Rust keywords
    if (std.mem.eql(u8, foreign_symbol, "fn")) return "袋.定义.操作";
    if (std.mem.eql(u8, foreign_symbol, "let")) return "袋.定义.绑定";
    if (std.mem.eql(u8, foreign_symbol, "mut")) return "袋.修饰.可变";
    if (std.mem.eql(u8, foreign_symbol, "impl")) return "袋.定义.实现";
    if (std.mem.eql(u8, foreign_symbol, "trait")) return "袋.定义.特征";
    if (std.mem.eql(u8, foreign_symbol, "match")) return "袋.判断.匹配";
    if (std.mem.eql(u8, foreign_symbol, "Option")) return "袋.值.可选";
    if (std.mem.eql(u8, foreign_symbol, "Result")) return "袋.值.结果";
    if (std.mem.eql(u8, foreign_symbol, "Box")) return "袋.容器.堆";
    if (std.mem.eql(u8, foreign_symbol, "Vec")) return "袋.容器.序列";

    // General
    if (std.mem.eql(u8, foreign_symbol, "function")) return "袋.定义.操作";
    if (std.mem.eql(u8, foreign_symbol, "variable")) return "袋.定义.绑定";
    if (std.mem.eql(u8, foreign_symbol, "array")) return "袋.容器.序列";
    if (std.mem.eql(u8, foreign_symbol, "string")) return "袋.容器.字符序列";
    if (std.mem.eql(u8, foreign_symbol, "number")) return "袋.值.数字";
    if (std.mem.eql(u8, foreign_symbol, "boolean")) return "袋.值.阴阳";
    if (std.mem.eql(u8, foreign_symbol, "true")) return "袋.值.阳";
    if (std.mem.eql(u8, foreign_symbol, "false")) return "袋.值.阴";
    if (std.mem.eql(u8, foreign_symbol, "null")) return "袋.值.空";
    if (std.mem.eql(u8, foreign_symbol, "undefined")) return "袋.值.无";

    return null; // rejected
}

/// 翻译一行代码（简化版：只翻译已知符号）
pub fn translateLine(comptime line: []const u8) []const u8 {
    _ = line;
    return "[翻译需手动处理]";
}

// === 显示 ===

pub fn displayTranslation(comptime foreign: []const u8) void {
    
    const result = translate(foreign);
    if (result) |ont| {
        std.debug.print("  {s:12} -> {s}\n", .{ foreign, ont });
    } else {
        std.debug.print("  {s:12} -> [拒绝]\n", .{foreign});
    }
}

pub fn displayAllTranslations() void {
    
    const sep = "============================================================";
    std.debug.print("\n{s}\n", .{sep});
    std.debug.print("翻译表\n", .{});
    std.debug.print("{s}\n", .{sep});

    const test_symbols = [_][]const u8{
        "class", "def", "import", "return", "if", "else", "for", "while",
        "int", "char", "struct", "static", "const", "pointer",
        "fn", "let", "mut", "impl", "trait", "match",
        "function", "variable", "array", "string", "boolean",
        "true", "false", "null", "unknown_symbol",
    };

    for (test_symbols) |sym| {
        displayTranslation(sym);
    }

    std.debug.print("{s}\n\n", .{sep});
}

// === 测试 ===

test "translator_python_keywords" {
    try std.testing.expectEqualStrings("袋.定义.容器", translate("class").?);
    try std.testing.expectEqualStrings("袋.定义.操作", translate("def").?);
    try std.testing.expectEqualStrings("袋.引用.外部", translate("import").?);
    try std.testing.expectEqualStrings("袋.输出.结果", translate("return").?);
    try std.testing.expectEqualStrings("袋.判断.条件", translate("if").?);
    try std.testing.expectEqualStrings("袋.逻辑.非", translate("not").?);
    try std.testing.expectEqualStrings("袋.值.阳", translate("True").?);
    try std.testing.expectEqualStrings("袋.值.阴", translate("False").?);
    try std.testing.expectEqualStrings("袋.值.空", translate("None").?);
    try std.testing.expectEqualStrings("袋.自指.本体", translate("self").?);
}

test "translator_c_keywords" {
    try std.testing.expectEqualStrings("袋.类型.整数", translate("int").?);
    try std.testing.expectEqualStrings("袋.类型.字符", translate("char").?);
    try std.testing.expectEqualStrings("袋.定义.结构", translate("struct").?);
    try std.testing.expectEqualStrings("袋.修饰.静态", translate("static").?);
    try std.testing.expectEqualStrings("袋.引用.地址", translate("pointer").?);
}

test "translator_rust_keywords" {
    try std.testing.expectEqualStrings("袋.定义.操作", translate("fn").?);
    try std.testing.expectEqualStrings("袋.定义.绑定", translate("let").?);
    try std.testing.expectEqualStrings("袋.修饰.可变", translate("mut").?);
    try std.testing.expectEqualStrings("袋.定义.实现", translate("impl").?);
    try std.testing.expectEqualStrings("袋.定义.特征", translate("trait").?);
    try std.testing.expectEqualStrings("袋.判断.匹配", translate("match").?);
}

test "translator_general" {
    try std.testing.expectEqualStrings("袋.定义.操作", translate("function").?);
    try std.testing.expectEqualStrings("袋.定义.绑定", translate("variable").?);
    try std.testing.expectEqualStrings("袋.容器.序列", translate("array").?);
    try std.testing.expectEqualStrings("袋.值.数字", translate("number").?);
    try std.testing.expectEqualStrings("袋.值.阴阳", translate("boolean").?);
}

test "translator_rejected" {
    try std.testing.expect(translate("unknown_symbol") == null);
    try std.testing.expect(translate("xyz123") == null);
    try std.testing.expect(translate("hello") == null);
}
