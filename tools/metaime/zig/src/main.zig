// main.zig - 元输入法工具链 · 统一入口 · 零运行时内存
//
// 所有数据为编译时常量（comptime），嵌入二进制，无运行时分配
// 工具模块：加载器、链接器、翻译器、解释器、三维进制、维度、公约数、元层级

const std = @import("std");

pub const loader = @import("loader.zig");
pub const linker = @import("linker.zig");
pub const translator = @import("translator.zig");
pub const interpreter = @import("interpreter.zig");
pub const radix3d = @import("radix3d.zig");
pub const divisor = @import("divisor.zig");
pub const dimension = @import("dimension.zig");
pub const yuan = @import("yuan.zig");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();

    stdout.print("\n", .{}) catch {};
    printBoxedHeader(stdout, "元输入法工具链 (MetaIME) · Zig 零运行时内存版");
    stdout.print("\n", .{}) catch {};

    // 1. 显示加载结果
    loader.displaySummary();

    // 2. 显示本体论树
    linker.displayOntology();

    // 3. 显示链接解析
    linker.displayResolution();

    // 4. 显示容器查询示例
    printBoxedHeader(stdout, "容器查询示例");
    interpreter.displayContainer("声母");
    interpreter.displayContainer("韵母");
    interpreter.displayContainer("声调");
    stdout.print("\n", .{}) catch {};

    // 5. 显示符号查询示例
    printBoxedHeader(stdout, "符号查询示例");
    interpreter.displaySymbol("声母", "首");
    interpreter.displaySymbol("声母", "次");
    interpreter.displaySymbol("韵母", "首");
    interpreter.displaySymbol("声调", "首");
    stdout.print("\n", .{}) catch {};

    // 6. 显示翻译示例
    printBoxedHeader(stdout, "翻译示例");
    translator.displayTranslation("class");
    translator.displayTranslation("def");
    translator.displayTranslation("import");
    translator.displayTranslation("int");
    translator.displayTranslation("fn");
    translator.displayTranslation("unknown_symbol");
    stdout.print("\n", .{}) catch {};

    // 7. 显示三维进制空间
    radix3d.displaySpace();

    // 8. 显示维度信息
    dimension.displayDimInfo();

    // 9. 显示公约数信息
    divisor.displayDivisors();

    // 10. 显示元层级
    yuan.displayHierarchy();

    // 11. 裁判示例
    printBoxedHeader(stdout, "裁判示例");
    const j1 = yuan.judge("b", "p");
    stdout.print("裁判(b, p): {s}\n", .{j1.message}) catch {};
    const j2 = yuan.judge("b", "b");
    stdout.print("裁判(b, b): {s}\n", .{j2.message}) catch {};
    stdout.print("\n", .{}) catch {};

    // 12. 内存状态
    interpreter.displayMemory();

    printBoxedHeader(stdout, "所有工具模块加载完成 · 编译时验证通过");
    stdout.print("\n", .{}) catch {};
}

fn printBoxedHeader(writer: anytype, comptime title: []const u8) void {
    const sep = "============================================================";
    writer.print("\n{s}\n", .{sep}) catch {};
    writer.print("{s}\n", .{title}) catch {};
    writer.print("{s}\n", .{sep}) catch {};
}

// === 引用所有模块以确保编译 ===

test "all_modules_compile" {
    _ = @import("constants.zig");
    _ = @import("loader.zig");
    _ = @import("linker.zig");
    _ = @import("translator.zig");
    _ = @import("interpreter.zig");
    _ = @import("radix3d.zig");
    _ = @import("dimension.zig");
    _ = @import("divisor.zig");
    _ = @import("yuan.zig");
}

// === 运行所有模块的测试 ===

test "all_module_tests" {
    // constants
    const c = @import("constants.zig");
    try std.testing.expectEqual(@as(usize, 24), c.SHENG_MU_COUNT);
    try std.testing.expectEqual(@as(usize, 24), c.YUN_MU_COUNT);
    try std.testing.expectEqual(@as(usize, 4), c.TONES_COUNT);

    // loader
    try std.testing.expect(loader.fqm_data.container_count >= 3);
    try std.testing.expect(loader.fqm_data.link_count >= 6);

    // linker
    try std.testing.expect(linker.linker_data.entry_count >= 23);
    try std.testing.expect(linker.linker_data.resolved_count >= 6);

    // translator
    try std.testing.expectEqualStrings("袋.定义.容器", translator.translate("class").?);
    try std.testing.expect(translator.translate("unknown") == null);

    // interpreter
    try std.testing.expect(interpreter.queryContainer("声母").ok);
    try std.testing.expect(interpreter.querySymbol("声母", "首").ok);

    // radix3d
    try std.testing.expect(radix3d.space_data.coordinate_count >= 24);
    try std.testing.expectEqual(@as(u8, 1), radix3d.ontologyDistance("首", "次"));

    // dimension
    const dim_info = dimension.getDimInfo();
    try std.testing.expect(dim_info.shengmu_count >= 23);
    try std.testing.expect(dim_info.total_capacity >= 23 * 24 * 4);

    // divisor
    try std.testing.expectEqual(@as(u8, 1), divisor.gcd(23, 24));
    try std.testing.expectEqual(@as(u16, 12), divisor.lcm(3, 4));

    // yuan
    try std.testing.expect(yuan.one("test").ok);
    try std.testing.expect(yuan.judge("b", "p").ok);
}
