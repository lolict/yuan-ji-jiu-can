// yuan.zig - 元层级 · 零运行时内存
//
// 一元 = 时间因果 — 代表目标、红利、受益者（中心不变量）
// 二元 = 盈亏镜像 — 围绕中心展开的辅助变化（可统计、可镜像）
// 三元 = 见证镜像 — 裁判观测者，公约化二元（静态公约化、动态收敛）
// 四元 = 包含镜像 — 面包裹其内/归属权（容积内部再套容积）

const std = @import("std");
const constants = @import("constants.zig");
const linker = @import("linker.zig");

// === 结果结构 ===

pub const YuanResult = struct {
    ok: bool,
    message: []const u8,
};

// === 一元操作 ===

/// 一元(目标)：声明中心目标
pub fn one(comptime target: []const u8) YuanResult {
    _ = target;
    return .{ .ok = true, .message = "一元声明" };
}

// === 二元操作 ===

/// 二元(中心, 辅助)：声明中心与辅助的镜像关系
pub fn two(comptime center: []const u8, comptime auxiliary: []const u8) YuanResult {
    _ = center;
    _ = auxiliary;
    return .{ .ok = true, .message = "二元声明" };
}

// === 三元操作 ===

/// 三元(观测, A, B)：声明见证者对A和B的裁判
pub fn three(comptime observer: []const u8, comptime a: []const u8, comptime b: []const u8) YuanResult {
    _ = observer;
    _ = a;
    _ = b;
    return .{ .ok = true, .message = "三元声明" };
}

// === 四元操作 ===

/// 四元(外壳, 内容)：声明包含关系
pub fn four(comptime shell: []const u8, comptime content: []const u8) YuanResult {
    _ = shell;
    _ = content;
    return .{ .ok = true, .message = "四元声明" };
}

// === 裁判操作 ===

/// 裁判(A, B)：裁定A与B的关系
pub fn judge(comptime a: []const u8, comptime b: []const u8) YuanResult {
    // Look up both symbols
    const entry1 = linker.lookup("声母", a) orelse
        return .{ .ok = false, .message = "符号A不存在" };
    const entry2 = linker.lookup("声母", b) orelse
        return .{ .ok = false, .message = "符号B不存在" };

    if (std.mem.eql(u8, entry1.identity, entry2.identity)) {
        return .{ .ok = true, .message = "同域同位" };
    } else {
        return .{ .ok = true, .message = "同域异位" };
    }
}

// === 升元/降元 ===

/// 升元：从低元升到高元
pub fn upgrade(comptime element: []const u8) YuanResult {
    _ = element;
    return .{ .ok = true, .message = "升元路径: 一元 -> 二元 -> 三元 -> 四元" };
}

/// 降元：从高元降到低元
pub fn downgrade(comptime element: []const u8) YuanResult {
    _ = element;
    return .{ .ok = true, .message = "降元路径: 四元 -> 三元 -> 二元 -> 一元" };
}

// === 层级显示 ===

pub fn displayHierarchy() void {
    const stdout = std.io.getStdOut().writer();
    const sep = "============================================================";
    stdout.print("\n{s}\n", .{sep}) catch {};
    stdout.print("元层级结构\n", .{}) catch {};
    stdout.print("{s}\n", .{sep}) catch {};
    stdout.print("  【一元】(时间因果) - 中心不变量、受益者\n", .{}) catch {};
    stdout.print("  【二元】(盈亏镜像) - 可统计、可镜像\n", .{}) catch {};
    stdout.print("  【三元】(见证镜像) - 裁判/观测者\n", .{}) catch {};
    stdout.print("  【四元】(包含镜像) - 归属权、容积包裹\n", .{}) catch {};
    stdout.print("{s}\n\n", .{sep}) catch {};
}

// === 测试 ===

test "yuan_one" {
    const result = one("囝囝共生");
    try std.testing.expect(result.ok);
}

test "yuan_two" {
    const result = two("目标", "声母b");
    try std.testing.expect(result.ok);
}

test "yuan_three" {
    const result = three("满全法", "声母.b", "韵母.a");
    try std.testing.expect(result.ok);
}

test "yuan_four" {
    const result = four("声母容器", "b");
    try std.testing.expect(result.ok);
}

test "yuan_judge_same" {
    const result = judge("b", "b");
    try std.testing.expectEqualStrings("同域同位", result.message);
}

test "yuan_judge_diff" {
    const result = judge("b", "p");
    try std.testing.expectEqualStrings("同域异位", result.message);
}

test "yuan_upgrade" {
    const result = upgrade("test");
    try std.testing.expect(result.ok);
}

test "yuan_downgrade" {
    const result = downgrade("test");
    try std.testing.expect(result.ok);
}
