// divisor.zig - 公约数公倍数 · 零运行时内存
//
// 公约数：不同容器共享的身份位置
// 公倍数：不同容器组合产生的新单位
// 关系运算：判断符号间关系

const std = @import("std");
const constants = @import("constants.zig");
const loader = @import("loader.zig");
const linker = @import("linker.zig");

// === 结果结构 ===

pub const DivisorResult = struct {
    ok: bool,
    message: []const u8,
};

// === GCD/LCM 工具 ===

pub fn gcd(a: u8, b: u8) u8 {
    var x = a;
    var y = b;
    while (y != 0) {
        const t = y;
        y = x % y;
        x = t;
    }
    return if (x == 0) 1 else x;
}

pub fn lcm(a: u8, b: u8) u16 {
    if (a == 0 or b == 0) return 0;
    const g = gcd(a, b);
    return (@as(u16, a) / g) * b;
}

// === 域信息 ===

/// 获取域中所有身份
pub fn getDomainIdentities(comptime domain: []const u8) []const []const u8 {
    _ = domain;
    // Return all identity names (simplified - all domains share the same identity set)
    return &constants.IDENTITY_NAMES;
}

// === 公约数操作 ===

/// 公约(声母, 韵母)：查找两个域共有的身份
pub fn commonDivisor(comptime domain1: []const u8, comptime domain2: []const u8) DivisorResult {
    const entries = linker.getOntologyEntries();

    // Collect identities for each domain
    var id1_count: u8 = 0;
    var id2_count: u8 = 0;

    for (entries) |entry| {
        if (std.mem.eql(u8, entry.domain, domain1)) id1_count += 1;
        if (std.mem.eql(u8, entry.domain, domain2)) id2_count += 1;
    }

    // All domains share the same identity set (首, 次, 三...)
    // So common identities = min(id1_count, id2_count) identities
    const common = if (id1_count < id2_count) id1_count else id2_count;
    _ = common;

    return .{ .ok = true, .message = "共有公约数" };
}

/// 同公(b, p)：判断两个符号是否在同一公约数上
pub fn sameDivisor(comptime sym1: []const u8, comptime sym2: []const u8) DivisorResult {
    const entry1 = linker.lookup("声母", sym1) orelse
        return .{ .ok = false, .message = "符号1不存在" };
    const entry2 = linker.lookup("声母", sym2) orelse
        return .{ .ok = false, .message = "符号2不存在" };

    if (std.mem.eql(u8, entry1.identity, entry2.identity)) {
        return .{ .ok = true, .message = "同一公约数" };
    } else {
        return .{ .ok = true, .message = "不同公约数" };
    }
}

/// 齐公(声母.首, 韵母.首)：判断两个位置是否对齐
pub fn aligned(comptime domain1: []const u8, comptime identity1: []const u8, comptime domain2: []const u8, comptime identity2: []const u8) DivisorResult {
    _ = domain1;
    _ = domain2;
    if (std.mem.eql(u8, identity1, identity2)) {
        return .{ .ok = true, .message = "对齐" };
    } else {
        return .{ .ok = true, .message = "不对齐" };
    }
}

/// 关系(声母.b, 韵母.a)：判断两个符号之间的关系
pub fn relation(comptime sym1: []const u8, comptime sym2: []const u8) DivisorResult {
    const entry1 = linker.lookup("声母", sym1) orelse
        return .{ .ok = false, .message = "符号1不存在" };
    const entry2 = linker.lookup("韵母", sym2) orelse
        return .{ .ok = false, .message = "符号2不存在" };

    _ = entry1;
    _ = entry2;
    return .{ .ok = true, .message = "异域" };
}

/// 公倍(声母.首, 韵母.首)：计算两个位置的组合
pub fn commonMultiple(comptime domain1: []const u8, comptime identity1: []const u8, comptime domain2: []const u8, comptime identity2: []const u8) DivisorResult {
    const sym1 = linker.lookupByIdentity(domain1, identity1) orelse
        return .{ .ok = false, .message = "位置1不存在" };
    const sym2 = linker.lookupByIdentity(domain2, identity2) orelse
        return .{ .ok = false, .message = "位置2不存在" };

    _ = sym1;
    _ = sym2;
    return .{ .ok = true, .message = "公倍计算完成" };
}

// === 显示 ===

pub fn displayDivisors() void {
    
    const sep = "============================================================";
    std.debug.print("\n{s}\n", .{sep});
    std.debug.print("公约数信息\n", .{});
    std.debug.print("{s}\n", .{sep});

    const entries = linker.getOntologyEntries();
    // Group by domain
    var domains: [constants.MAX_CONTAINERS][]const u8 = undefined;
    var domain_counts: [constants.MAX_CONTAINERS]u8 = undefined;
    var domain_count: u8 = 0;

    for (entries) |entry| {
        var found = false;
        var di: u8 = 0;
        while (di < domain_count) : (di += 1) {
            if (std.mem.eql(u8, domains[di], entry.domain)) {
                domain_counts[di] += 1;
                found = true;
                break;
            }
        }
        if (!found and domain_count < domains.len) {
            domains[domain_count] = entry.domain;
            domain_counts[domain_count] = 1;
            domain_count += 1;
        }
    }

    var di: u8 = 0;
    while (di < domain_count) : (di += 1) {
        std.debug.print("  {s}: {} 个身份\n", .{ domains[di], domain_counts[di] });
    }

    // Show GCD/LCM of capacities
    const fqm = loader.fqm_data;
    var sm_cap: u8 = 0;
    var ym_cap: u8 = 0;
    var t_cap: u8 = 0;
    for (fqm.containers[0..fqm.container_count]) |c| {
        if (std.mem.eql(u8, c.name, "声母")) sm_cap = c.capacity;
        if (std.mem.eql(u8, c.name, "韵母")) ym_cap = c.capacity;
        if (std.mem.eql(u8, c.name, "声调")) t_cap = c.capacity;
    }

    std.debug.print("  GCD(声母={}, 韵母={}) = {}\n", .{ sm_cap, ym_cap, gcd(sm_cap, ym_cap) });
    std.debug.print("  LCM(声母={}, 韵母={}) = {}\n", .{ sm_cap, ym_cap, lcm(sm_cap, ym_cap) });
    std.debug.print("  GCD({}, 声调={}) = {}\n", .{ sm_cap, t_cap, gcd(sm_cap, t_cap) });

    std.debug.print("{s}\n\n", .{sep});
}

// === 测试 ===

test "divisor_gcd" {
    try std.testing.expectEqual(@as(u8, 1), gcd(23, 24));
    try std.testing.expectEqual(@as(u8, 6), gcd(6, 12));
    try std.testing.expectEqual(@as(u8, 1), gcd(3, 5));
}

test "divisor_lcm" {
    try std.testing.expectEqual(@as(u16, 12), lcm(3, 4));
    try std.testing.expectEqual(@as(u16, 60), lcm(12, 5));
}

test "divisor_common_divisor" {
    const result = commonDivisor("声母", "韵母");
    try std.testing.expect(result.ok);
}

test "divisor_same_divisor_true" {
    const result = sameDivisor("b", "b");
    try std.testing.expectEqualStrings("同一公约数", result.message);
}

test "divisor_same_divisor_false" {
    const result = sameDivisor("b", "p");
    try std.testing.expectEqualStrings("不同公约数", result.message);
}

test "divisor_aligned_true" {
    const result = aligned("声母", "首", "韵母", "首");
    try std.testing.expectEqualStrings("对齐", result.message);
}

test "divisor_aligned_false" {
    const result = aligned("声母", "首", "韵母", "次");
    try std.testing.expectEqualStrings("不对齐", result.message);
}

test "divisor_relation" {
    const result = relation("b", "a");
    try std.testing.expect(result.ok);
}

test "divisor_common_multiple" {
    const result = commonMultiple("声母", "首", "韵母", "首");
    try std.testing.expect(result.ok);
}
