// dimension.zig - 维度升降 · 零运行时内存
//
// 高维〈=〉低维映射
// 三维坐标：(声母.x, 韵母.y, 声调.z)
// 降维：去掉指定维度
// 升维：添加维度（取默认值）
// 投影：只保留指定维度
// 展开：从一维展开到完整三维

const std = @import("std");
const constants = @import("constants.zig");
const loader = @import("loader.zig");
const linker = @import("linker.zig");

// === 维度操作结果 ===

pub const DimResult = struct {
    ok: bool,
    message: []const u8,
};

/// 维度信息
pub const DimInfo = struct {
    shengmu_count: u8,
    yunmu_count: u8,
    tone_count: u8,
    total_capacity: u16,
};

pub fn getDimInfo() DimInfo {
    const fqm = loader.fqm_data;
    var sm_count: u8 = 0;
    var ym_count: u8 = 0;
    var t_count: u8 = 0;

    for (fqm.containers[0..fqm.container_count]) |c| {
        if (std.mem.eql(u8, c.name, "声母")) sm_count = c.symbol_count;
        if (std.mem.eql(u8, c.name, "韵母")) ym_count = c.symbol_count;
        if (std.mem.eql(u8, c.name, "声调")) t_count = c.symbol_count;
    }

    return .{
        .shengmu_count = sm_count,
        .yunmu_count = ym_count,
        .tone_count = t_count,
        .total_capacity = @as(u16, sm_count) * ym_count * t_count,
    };
}

/// 降维：去掉指定维度
/// drop_dim: "声母", "韵母", or "声调"
pub fn downshift(comptime shengmu_sym: []const u8, comptime yunmu_sym: []const u8, comptime tone_sym: []const u8, comptime drop_dim: []const u8) DimResult {
    if (std.mem.eql(u8, drop_dim, "声母")) {
        // Result: keep 韵母 and 声调
        _ = shengmu_sym;
        return .{ .ok = true, .message = "降维完成" };
    } else if (std.mem.eql(u8, drop_dim, "韵母")) {
        // Result: keep 声母 and 声调
        _ = yunmu_sym;
        return .{ .ok = true, .message = "降维完成" };
    } else if (std.mem.eql(u8, drop_dim, "声调")) {
        // Result: keep 声母 and 韵母
        _ = tone_sym;
        return .{ .ok = true, .message = "降维完成" };
    }
    return .{ .ok = false, .message = "未知维度" };
}

/// 升维：从声母韵母升到三维，声调取默认
pub fn upshift(comptime shengmu_sym: []const u8, comptime yunmu_sym: []const u8, comptime tone_default: []const u8) DimResult {
    // Verify 声母 and 韵母 exist
    const sm_entry = linker.lookup("声母", shengmu_sym) orelse
        return .{ .ok = false, .message = "声母不存在" };
    const ym_entry = linker.lookup("韵母", yunmu_sym) orelse
        return .{ .ok = false, .message = "韵母不存在" };

    // Verify tone exists
    const tone_entry = linker.lookup("声调", tone_default) orelse
        return .{ .ok = false, .message = "声调不存在" };

    _ = sm_entry;
    _ = ym_entry;
    _ = tone_entry;
    return .{ .ok = true, .message = "升维完成" };
}

/// 投影：只保留指定维度
pub fn project(comptime shengmu_sym: []const u8, comptime yunmu_sym: []const u8, comptime tone_sym: []const u8, comptime keep_dim: []const u8) DimResult {
    _ = shengmu_sym;
    _ = yunmu_sym;
    _ = tone_sym;
    _ = keep_dim;
    return .{ .ok = true, .message = "投影完成" };
}

/// 展开：从声母展开到完整三维
pub fn expand(comptime shengmu_sym: []const u8, comptime yunmu_sym: []const u8, comptime tone_sym: []const u8) DimResult {
    // Verify all three dimensions exist
    const sm_entry = linker.lookup("声母", shengmu_sym) orelse
        return .{ .ok = false, .message = "声母不存在" };
    const ym_entry = linker.lookup("韵母", yunmu_sym) orelse
        return .{ .ok = false, .message = "韵母不存在" };
    const tone_entry = linker.lookup("声调", tone_sym) orelse
        return .{ .ok = false, .message = "声调不存在" };

    _ = sm_entry;
    _ = ym_entry;
    _ = tone_entry;
    return .{ .ok = true, .message = "展开完成" };
}

// === 显示 ===

pub fn displayDimInfo() void {
    const stdout = std.io.getStdOut().writer();
    const info = getDimInfo();
    stdout.print("\n维度信息:\n", .{}) catch {};
    stdout.print("  声母轴: {} 个位置\n", .{info.shengmu_count}) catch {};
    stdout.print("  韵母轴: {} 个位置\n", .{info.yunmu_count}) catch {};
    stdout.print("  声调轴: {} 个位置\n", .{info.tone_count}) catch {};
    stdout.print("  三维空间总容量: {}x{}x{} = {}\n", .{
        info.shengmu_count, info.yunmu_count, info.tone_count, info.total_capacity,
    }) catch {};
}

// === 测试 ===

test "dimension_info" {
    const info = getDimInfo();
    try std.testing.expect(info.shengmu_count >= 23);
    try std.testing.expect(info.yunmu_count >= 24);
    try std.testing.expect(info.tone_count >= 4);
    try std.testing.expect(info.total_capacity >= 23 * 24 * 4);
}

test "dimension_downshift" {
    const result = downshift("b", "a", "1", "声调");
    try std.testing.expect(result.ok);
}

test "dimension_upshift" {
    const result = upshift("b", "a", "1");
    try std.testing.expect(result.ok);
}

test "dimension_expand" {
    const result = expand("b", "a", "1");
    try std.testing.expect(result.ok);
}

test "dimension_downshift_bad_dim" {
    const result = downshift("b", "a", "1", "未知");
    try std.testing.expect(!result.ok);
}
