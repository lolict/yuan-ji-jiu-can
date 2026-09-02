// interpreter.zig - 容器操作解释器 · 零运行时内存
//
// 执行容器操作：查询、比较、链接查询
// 所有操作使用本体论路径（袋.域.身份），非代数地址

const std = @import("std");
const constants = @import("constants.zig");
const loader = @import("loader.zig");
const linker = @import("linker.zig");

// === 操作结果 ===

pub const OpResult = struct {
    ok: bool,
    message: []const u8,
};

// === 查询操作 ===

/// 查询容器所有符号: 袋.声母
pub fn queryContainer(comptime domain: []const u8) OpResult {
    const container = loader.getContainer(domain) orelse
        return .{ .ok = false, .message = "容器不存在" };

    // Verify the container has symbols
    if (container.symbol_count == 0) {
        return .{ .ok = false, .message = "容器为空" };
    }
    return .{ .ok = true, .message = "查询成功" };
}

/// 查询具体位置的符号: 袋.声母.首
pub fn querySymbol(comptime domain: []const u8, comptime identity: []const u8) OpResult {
    const sym = linker.lookupByIdentity(domain, identity) orelse
        return .{ .ok = false, .message = "符号不存在" };

    // Verify symbol is non-empty
    if (sym.len == 0) {
        return .{ .ok = false, .message = "符号为空" };
    }
    return .{ .ok = true, .message = "查询成功" };
}

/// 取出操作: 袋.声母.取出(首)
pub fn extract(comptime domain: []const u8, comptime identity: []const u8) OpResult {
    const entries = linker.getOntologyEntries();
    for (entries) |entry| {
        if (std.mem.eql(u8, entry.domain, domain) and
            std.mem.eql(u8, entry.identity, identity))
        {
            if (entry.symbol.len == 0) {
                return .{ .ok = false, .message = "条目为空" };
            }
            return .{ .ok = true, .message = "取出成功" };
        }
    }
    return .{ .ok = false, .message = "符号不存在" };
}

/// 比较操作: 袋.声母.比较(b, p)
pub fn compare(comptime domain: []const u8, comptime sym1: []const u8, comptime sym2: []const u8) OpResult {
    const entry1 = linker.lookup(domain, sym1) orelse
        return .{ .ok = false, .message = "符号1不存在" };
    const entry2 = linker.lookup(domain, sym2) orelse
        return .{ .ok = false, .message = "符号2不存在" };

    if (std.mem.eql(u8, entry1.identity, entry2.identity)) {
        return .{ .ok = true, .message = "同一位置" };
    } else {
        return .{ .ok = true, .message = "不同位置" };
    }
}

/// 链接查询: 袋.声母.引.韵母(首)
pub fn queryLink(comptime from_domain: []const u8, comptime to_domain: []const u8, comptime identity: []const u8) OpResult {
    const entries = linker.getOntologyEntries();
    const resolved = linker.getResolvedLinks();

    // Find the symbol for the given identity
    for (entries) |entry| {
        if (std.mem.eql(u8, entry.domain, from_domain) and
            std.mem.eql(u8, entry.identity, identity))
        {
            // Find a matching resolved link
            for (resolved) |link| {
                if (std.mem.eql(u8, link.from_domain, from_domain) and
                    std.mem.eql(u8, link.from_symbol, entry.symbol) and
                    std.mem.eql(u8, link.to_domain, to_domain))
                {
                    return .{ .ok = true, .message = "链接存在" };
                }
            }
            return .{ .ok = false, .message = "无链接" };
        }
    }

    return .{ .ok = false, .message = "源符号不存在" };
}

/// 装入操作: 袋.声母.装入(b) - 检查符号是否已存在
pub fn load(comptime domain: []const u8, comptime symbol: []const u8) OpResult {
    const existing = linker.lookup(domain, symbol);
    if (existing != null) {
        return .{ .ok = false, .message = "符号已存在" };
    }
    return .{ .ok = true, .message = "可装入" };
}

// === 显示 ===

pub fn displayContainer(comptime domain: []const u8) void {
    
    const container = loader.getContainer(domain) orelse {
        std.debug.print("容器 {s} 不存在\n", .{domain});
        return;
    };

    std.debug.print("袋.{s} = [ ", .{domain});
    for (container.symbols[0..container.symbol_count]) |entry| {
        std.debug.print("{s} ", .{entry.symbol});
    }
    std.debug.print("]\n", .{});
}

pub fn displaySymbol(comptime domain: []const u8, comptime identity: []const u8) void {
    
    const sym = linker.lookupByIdentity(domain, identity) orelse {
        std.debug.print("{s}.{s} = [空]\n", .{ domain, identity });
        return;
    };
    std.debug.print("袋.{s}.{s} = {s}\n", .{ domain, identity, sym });
}

pub fn displayMemory() void {
    
    const sep = "============================================================";
    std.debug.print("\n{s}\n", .{sep});
    std.debug.print("内存状态（本体论路径 -> 符号）\n", .{});
    std.debug.print("{s}\n", .{sep});

    const entries = linker.getOntologyEntries();
    // Group by domain
    var domains: [constants.MAX_CONTAINERS][]const u8 = undefined;
    var domain_count: u8 = 0;

    for (entries) |entry| {
        var found = false;
        var di: u8 = 0;
        while (di < domain_count) : (di += 1) {
            if (std.mem.eql(u8, domains[di], entry.domain)) { found = true; break; }
        }
        if (!found and domain_count < domains.len) {
            domains[domain_count] = entry.domain;
            domain_count += 1;
        }
    }

    var di: u8 = 0;
    while (di < domain_count) : (di += 1) {
        std.debug.print("  [{s}]\n", .{domains[di]});
        for (entries) |entry| {
            if (std.mem.eql(u8, entry.domain, domains[di])) {
                std.debug.print("    {s}.{s} -> {s}\n", .{ entry.domain, entry.identity, entry.symbol });
            }
        }
    }

    std.debug.print("{s}\n\n", .{sep});
}

// === 测试 ===

test "interpreter_query_container" {
    const result = queryContainer("声母");
    try std.testing.expect(result.ok);
}

test "interpreter_query_symbol" {
    const result = querySymbol("声母", "首");
    try std.testing.expect(result.ok);
}

test "interpreter_extract" {
    const result = extract("声母", "首");
    try std.testing.expect(result.ok);
}

test "interpreter_compare_same" {
    const result = compare("声母", "b", "b");
    try std.testing.expect(result.ok);
}

test "interpreter_compare_diff" {
    const result = compare("声母", "b", "p");
    try std.testing.expect(result.ok);
}

test "interpreter_query_link" {
    const result = queryLink("声母", "韵母", "首");
    try std.testing.expect(result.ok);
}

test "interpreter_load_existing" {
    const result = load("声母", "b");
    try std.testing.expect(!result.ok); // already exists
}

test "interpreter_load_new" {
    const result = load("声母", "Q");
    // Q is not in the container, so it should be loadable
    try std.testing.expect(result.ok);
}
