// linker.zig - 本体论索引构建器 · 零运行时内存
//
// 从 FQM 加载数据构建本体论索引
// 位置 = 域.身份 (如 声母.首, 韵母.次)

const std = @import("std");
const constants = @import("constants.zig");
const loader = @import("loader.zig");

// === 数据结构 ===

pub const OntologyEntry = struct {
    domain: []const u8,
    identity: []const u8,
    symbol: []const u8,
};

pub const ResolvedLink = struct {
    from_domain: []const u8,
    from_identity: []const u8,
    from_symbol: []const u8,
    to_domain: []const u8,
    to_identity: []const u8,
    to_symbol: []const u8,
};

pub const UnresolvedLink = struct {
    from_domain: []const u8,
    from_symbol: []const u8,
    to_domain: []const u8,
    to_symbol: []const u8,
};

pub const LinkerData = struct {
    entries: [constants.MAX_IDENTITY_LEN * constants.MAX_CONTAINERS]OntologyEntry,
    entry_count: u8,
    resolved: [constants.MAX_LINKS]ResolvedLink,
    resolved_count: u8,
    unresolved: [constants.MAX_LINKS]UnresolvedLink,
    unresolved_count: u8,
};

// === 编译时构建本体论索引 ===

pub fn buildLinkerData() LinkerData {
    @setEvalBranchQuota(20000);
    comptime var data: LinkerData = undefined;
    data.entry_count = 0;
    data.resolved_count = 0;
    data.unresolved_count = 0;

    const fqm = loader.fqm_data;

    // Build ontology index: for each container, map identity -> symbol
    var ci: u8 = 0;
    while (ci < fqm.container_count) : (ci += 1) {
        const container = fqm.containers[ci];
        var si: u8 = 0;
        while (si < container.symbol_count) : (si += 1) {
            if (si >= constants.MAX_IDENTITY_LEN) break;
            const identity = constants.IDENTITY_NAMES[si];
            const entry_idx = data.entry_count;
            if (entry_idx >= data.entries.len) break;
            data.entries[entry_idx] = .{
                .domain = container.name,
                .identity = identity,
                .symbol = container.symbols[si].symbol,
            };
            data.entry_count = entry_idx + 1;
        }
    }

    // Resolve links
    var li: u8 = 0;
    while (li < fqm.link_count) : (li += 1) {
        const link = fqm.links[li];

        // Find from identity and to identity
        var from_identity: ?[]const u8 = null;
        var to_identity: ?[]const u8 = null;

        var ei: u8 = 0;
        while (ei < data.entry_count) : (ei += 1) {
            const entry = data.entries[ei];
            if (std.mem.eql(u8, entry.domain, link.from_domain) and
                std.mem.eql(u8, entry.symbol, link.from_symbol))
            {
                from_identity = entry.identity;
            }
            if (std.mem.eql(u8, entry.domain, link.to_domain) and
                std.mem.eql(u8, entry.symbol, link.to_symbol))
            {
                to_identity = entry.identity;
            }
        }

        if (from_identity != null and to_identity != null) {
            const ri = data.resolved_count;
            if (ri >= data.resolved.len) break;
            data.resolved[ri] = .{
                .from_domain = link.from_domain,
                .from_identity = from_identity.?,
                .from_symbol = link.from_symbol,
                .to_domain = link.to_domain,
                .to_identity = to_identity.?,
                .to_symbol = link.to_symbol,
            };
            data.resolved_count = ri + 1;
        } else {
            const ui = data.unresolved_count;
            if (ui >= data.unresolved.len) break;
            data.unresolved[ui] = .{
                .from_domain = link.from_domain,
                .from_symbol = link.from_symbol,
                .to_domain = link.to_domain,
                .to_symbol = link.to_symbol,
            };
            data.unresolved_count = ui + 1;
        }
    }

    return data;
}

pub const linker_data = buildLinkerData();

// === 查找函数 ===

pub fn lookup(comptime domain: []const u8, comptime symbol: []const u8) ?OntologyEntry {
    for (linker_data.entries[0..linker_data.entry_count]) |entry| {
        if (std.mem.eql(u8, entry.domain, domain) and std.mem.eql(u8, entry.symbol, symbol))
            return entry;
    }
    return null;
}

pub fn lookupByIdentity(comptime domain: []const u8, comptime identity: []const u8) ?[]const u8 {
    for (linker_data.entries[0..linker_data.entry_count]) |entry| {
        if (std.mem.eql(u8, entry.domain, domain) and std.mem.eql(u8, entry.identity, identity))
            return entry.symbol;
    }
    return null;
}

pub fn getOntologyEntries() []const OntologyEntry {
    return linker_data.entries[0..linker_data.entry_count];
}

pub fn getResolvedLinks() []const ResolvedLink {
    return linker_data.resolved[0..linker_data.resolved_count];
}

pub fn getUnresolvedLinks() []const UnresolvedLink {
    return linker_data.unresolved[0..linker_data.unresolved_count];
}

// === 显示 ===

pub fn displayOntology() void {
    const stdout = std.io.getStdOut().writer();
    printLine(stdout, 60, '=');
    stdout.print("本体论树\n", .{}) catch {};
    printLine(stdout, 60, '-');

    // Group by domain
    var domains: [constants.MAX_CONTAINERS][]const u8 = undefined;
    var domain_count: u8 = 0;

    for (linker_data.entries[0..linker_data.entry_count]) |entry| {
        var found = false;
        var di: u8 = 0;
        while (di < domain_count) : (di += 1) {
            if (std.mem.eql(u8, domains[di], entry.domain)) { found = true; break; }
        }
        if (!found) {
            if (domain_count < domains.len) {
                domains[domain_count] = entry.domain;
                domain_count += 1;
            }
        }
    }

    var di: u8 = 0;
    while (di < domain_count) : (di += 1) {
        stdout.print("  {s}\n", .{domains[di]}) catch {};
        for (linker_data.entries[0..linker_data.entry_count]) |entry| {
            if (std.mem.eql(u8, entry.domain, domains[di])) {
                stdout.print("    {s}: {s}\n", .{ entry.identity, entry.symbol }) catch {};
            }
        }
    }

    printLine(stdout, 60, '=');
}

pub fn displayResolution() void {
    const stdout = std.io.getStdOut().writer();
    printLine(stdout, 60, '=');
    stdout.print("本体论链接\n", .{}) catch {};
    printLine(stdout, 60, '-');
    stdout.print("已解析: {}\n", .{linker_data.resolved_count}) catch {};
    stdout.print("未解析: {}\n", .{linker_data.unresolved_count}) catch {};

    for (linker_data.resolved[0..linker_data.resolved_count]) |link| {
        stdout.print("  {s}.{s}({s}) -> {s}.{s}({s})\n", .{
            link.from_domain, link.from_identity, link.from_symbol,
            link.to_domain, link.to_identity, link.to_symbol,
        }) catch {};
    }

    printLine(stdout, 60, '=');
}

fn printLine(writer: anytype, comptime n: usize, comptime ch: u8) void {
    comptime var i: usize = 0;
    inline while (i < n) : (i += 1) {
        writer.print("{c}", .{ch}) catch {};
    }
    writer.print("\n", .{}) catch {};
}

// === 测试 ===

test "linker_build_ontology" {
    const data = linker_data;
    try std.testing.expect(data.entry_count >= 23);
    try std.testing.expect(data.resolved_count >= 6);
}

test "linker_lookup" {
    const entry = lookup("声母", "b").?;
    try std.testing.expectEqualStrings("声母", entry.domain);
    try std.testing.expectEqualStrings("首", entry.identity);
    try std.testing.expectEqualStrings("b", entry.symbol);
}

test "linker_lookup_by_identity" {
    const sym = lookupByIdentity("声母", "首").?;
    try std.testing.expectEqualStrings("b", sym);
}

test "linker_resolved_links" {
    const links = getResolvedLinks();
    try std.testing.expect(links.len >= 6);
    try std.testing.expectEqualStrings("b", links[0].from_symbol);
    try std.testing.expectEqualStrings("a", links[0].to_symbol);
    try std.testing.expectEqualStrings("首", links[0].from_identity);
    try std.testing.expectEqualStrings("首", links[0].to_identity);
}
