// main.zig - 元输入法工具链 · 统一入口 · 零运行时内存
//
// 所有数据为编译时常量（comptime），嵌入二进制，无运行时分配
// 工具模块：加载器、链接器、翻译器、解释器、三维进制、维度、公约数、元层级
// 新增：韵母向量、字形几何、力场演化

const std = @import("std");

pub const loader = @import("loader.zig");
pub const linker = @import("linker.zig");
pub const translator = @import("translator.zig");
pub const interpreter = @import("interpreter.zig");
pub const radix3d = @import("radix3d.zig");
pub const divisor = @import("divisor.zig");
pub const dimension = @import("dimension.zig");
pub const yuan = @import("yuan.zig");
pub const yunmu = @import("yunmu.zig");
pub const glyph = @import("glyph.zig");
pub const executor = @import("executor.zig");

// ── 统一输出宏 (Zig 0.14+ 兼容) ──
fn print(comptime fmt: []const u8, args: anytype) void {
    std.debug.print(fmt, args);
}

fn printHeader(comptime title: []const u8) void {
    const sep = "============================================================";
    print("\n{s}\n", .{sep});
    print("{s}\n", .{title});
    print("{s}\n", .{sep});
}

pub fn main() !void {
    print("\n", .{});
    printHeader("元输入法工具链 (MetaIME) · Zig 零运行时");
    print("\n", .{});

    // 1. 显示加载结果
    loader.displaySummary();

    // 2. 显示本体论树
    linker.displayOntology();
    linker.displayResolution();

    // 3. 容器查询示例
    printHeader("容器查询示例");
    interpreter.displayContainer("声母");
    interpreter.displayContainer("韵母");
    interpreter.displayContainer("声调");
    print("\n", .{});

    // 4. 符号查询示例
    printHeader("符号查询示例");
    interpreter.displaySymbol("声母", "首");
    interpreter.displaySymbol("韵母", "首");
    interpreter.displaySymbol("声调", "首");
    print("\n", .{});

    // 5. 翻译示例
    printHeader("翻译示例");
    translator.displayTranslation("class");
    translator.displayTranslation("def");
    translator.displayTranslation("import");
    print("\n", .{});

    // 6. 韵母向量示例
    printHeader("韵母向量");
    displayYunmu("a");
    displayYunmu("i");
    print("\n", .{});

    // 7. 字形几何示例
    printHeader("字形几何");
    displayGlyph('b');
    displayGlyph('m');
    displayGlyph('x');
    displayGlyph('g');
    print("\n", .{});

    // 8. 力场演化示例
    printHeader("力场演化");
    print("解析: ba ma pa\n", .{});
    const gpa = executor.parseSyllable('b', "a", 1) orelse return;
    const gpb = executor.parseSyllable('m', "a", 1) orelse return;
    const gpc = executor.parseSyllable('p', "a", 1) orelse return;
    print("  声母力: b={d:.0} m={d:.0} p={d:.0}\n", .{ gpa.force, gpb.force, gpc.force });
    print("  (力场演化需要运行时分配器，见 executor.zig)\n", .{});
    print("\n", .{});

    // 9. 三维进制空间
    radix3d.displaySpace();

    // 10. 元层级
    yuan.displayHierarchy();

    // 11. 裁判示例
    printHeader("裁判示例");
    const j1 = yuan.judge("b", "p");
    print("裁判(b, p): {s}\n", .{j1.message});
    print("\n", .{});

    // 12. 内存状态
    interpreter.displayMemory();

    printHeader("所有工具模块加载完成 · 编译时验证通过");
    print("\n", .{});
}

fn displayYunmu(name: []const u8) void {
    const idx = yunmu.findYunmu(name) orelse {
        print("  未找到韵母: {s}\n", .{name});
        return;
    };
    const v = yunmu.getYunmu(idx);
    print("  {s}: 方向={s}, 力={s}, 向量=({d:.1},{d:.1})\n", .{ name, v.direction, v.force_desc, v.vx, v.vy });
}

fn displayGlyph(ch: u8) void {
    const info = glyph.lookupGlyph(ch) orelse {
        print("  未找到字母: {c}\n", .{ch});
        return;
    };
    const force = glyph.calculateForce(info);
    print("  {c}: 力={d}, 操作={s}\n", .{ ch, force, info.operation });
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
    _ = @import("yunmu.zig");
    _ = @import("glyph.zig");
    _ = @import("executor.zig");
}

// === 运行所有模块的测试 ===

test "all_module_tests" {
    const c = @import("constants.zig");
    try std.testing.expectEqual(@as(usize, 24), c.SHENG_MU_COUNT);
    try std.testing.expectEqual(@as(usize, 24), c.YUN_MU_COUNT);

    try std.testing.expect(loader.fqm_data.container_count >= 3);
    try std.testing.expect(linker.linker_data.entry_count >= 23);
    try std.testing.expectEqualStrings("袋.定义.容器", translator.translate("class").?);
    try std.testing.expect(interpreter.queryContainer("声母").ok);
    try std.testing.expect(radix3d.ontologyDistance("首", "次") == 1);

    // 新增
    try std.testing.expect(yunmu.findYunmu("a") != null);
    try std.testing.expectEqual(@as(f32, 1.0), yunmu.toneEnergy(1));
    try std.testing.expect(glyph.lookupGlyph('b') != null);
    try std.testing.expect(yuan.judge("b", "p").ok);
}
