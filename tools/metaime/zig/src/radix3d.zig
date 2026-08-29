// radix3d.zig - 三维进制空间 · 零运行时内存
//
// 声母×韵母×声调 = 三维空间坐标
// 坐标用本体论路径表示：(声母.首, 韵母.首, 声调.首)

const std = @import("std");
const constants = @import("constants.zig");
const loader = @import("loader.zig");
const linker = @import("linker.zig");

// === 数据结构 ===

pub const Point3D = struct {
    shengmu_identity: []const u8,
    yunmu_identity: []const u8,
    tone_identity: []const u8,

    pub fn format(self: Point3D, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) void {
        _ = fmt;
        _ = options;
        writer.print("(声母.{s}, 韵母.{s}, 声调.{s})", .{
            self.shengmu_identity, self.yunmu_identity, self.tone_identity,
        }) catch {};
    }
};

pub const Coordinate = struct {
    point: Point3D,
    shengmu_symbol: []const u8,
    yunmu_symbol: []const u8,
    tone_symbol: []const u8,
};

// === 三维空间构建 ===

pub const SPACE_CAPACITY = 64; // 4 links × 4 tones × 4 possible shengmu

pub const SpaceData = struct {
    coordinates: [SPACE_CAPACITY]Coordinate,
    coordinate_count: u8,
};

pub fn buildSpaceData() SpaceData {
    @setEvalBranchQuota(20000);
    comptime var data: SpaceData = undefined;
    data.coordinate_count = 0;

    const fqm = loader.fqm_data;
    const entries = linker.getOntologyEntries();
    const resolved = linker.getResolvedLinks();

    // For each resolved link (声母→韵母), create coordinates for each tone
    for (resolved) |link| {
        // Find identity of from_symbol in from_domain
        var from_identity: ?[]const u8 = null;
        var to_identity: ?[]const u8 = null;

        for (entries) |entry| {
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

        const fi = from_identity orelse continue;
        const ti = to_identity orelse continue;

        // Create a coordinate for each tone
        for (fqm.containers[2].symbols[0..fqm.containers[2].symbol_count]) |tone_entry| {
            // Find tone identity
            var tone_identity: []const u8 = undefined;
            if (tone_entry.position < constants.IDENTITY_COUNT) {
                tone_identity = constants.IDENTITY_NAMES[tone_entry.position];
            } else {
                tone_identity = "?";
            }

            if (data.coordinate_count < data.coordinates.len) {
                data.coordinates[data.coordinate_count] = .{
                    .point = .{
                        .shengmu_identity = fi,
                        .yunmu_identity = ti,
                        .tone_identity = tone_identity,
                    },
                    .shengmu_symbol = link.from_symbol,
                    .yunmu_symbol = link.to_symbol,
                    .tone_symbol = tone_entry.symbol,
                };
                data.coordinate_count += 1;
            }
        }
    }

    return data;
}

pub const space_data = buildSpaceData();

// === 查找函数 ===

pub fn getCoordinates() []const Coordinate {
    return space_data.coordinates[0..space_data.coordinate_count];
}

/// 符号转身份
pub fn symbolToIdentity(comptime domain: []const u8, comptime symbol: []const u8) ?[]const u8 {
    return linker.lookup(domain, symbol).?.identity;
}

/// 查找坐标
pub fn findCoordinate(comptime shengmu_sym: []const u8, comptime yunmu_sym: []const u8, comptime tone_sym: []const u8) ?Coordinate {
    for (space_data.coordinates[0..space_data.coordinate_count]) |coord| {
        if (std.mem.eql(u8, coord.shengmu_symbol, shengmu_sym) and
            std.mem.eql(u8, coord.yunmu_symbol, yunmu_sym) and
            std.mem.eql(u8, coord.tone_symbol, tone_sym))
        {
            return coord;
        }
    }
    return null;
}

/// 计算两个身份之间的距离（身份差距之和）
pub fn ontologyDistance(comptime id1: []const u8, comptime id2: []const u8) u8 {
    var idx1: u8 = 0;
    var idx2: u8 = 0;
    var found1 = false;
    var found2 = false;

    var i: u8 = 0;
    while (i < constants.IDENTITY_COUNT) : (i += 1) {
        if (std.mem.eql(u8, constants.IDENTITY_NAMES[i], id1)) { idx1 = i; found1 = true; }
        if (std.mem.eql(u8, constants.IDENTITY_NAMES[i], id2)) { idx2 = i; found2 = true; }
    }

    if (!found1 or !found2) return 0;

    if (idx1 > idx2) return idx1 - idx2;
    return idx2 - idx1;
}

/// 计算两个三维点之间的距离
pub fn distance(comptime p1: Point3D, comptime p2: Point3D) u16 {
    const d1 = ontologyDistance(p1.shengmu_identity, p2.shengmu_identity);
    const d2 = ontologyDistance(p1.yunmu_identity, p2.yunmu_identity);
    const d3 = ontologyDistance(p1.tone_identity, p2.tone_identity);
    return d1 + d2 + d3;
}

/// 查找邻近坐标（距离为1）
pub fn findNeighbors(comptime shengmu_sym: []const u8, comptime yunmu_sym: []const u8, comptime tone_sym: []const u8) []const Coordinate {
    _ = shengmu_sym;
    _ = yunmu_sym;
    _ = tone_sym;
    // Full implementation would check all adjacent coordinates
    // For now, return empty (zero-runtime constraint)
    return space_data.coordinates[0..0];
}

// === 显示 ===

pub fn displaySpace() void {
    const stdout = std.io.getStdOut().writer();
    const sep = "============================================================";
    stdout.print("\n{s}\n", .{sep}) catch {};
    stdout.print("三维进制空间\n", .{}) catch {};
    stdout.print("{s}\n", .{sep}) catch {};

    const coords = getCoordinates();
    if (coords.len == 0) {
        stdout.print("  空间为空\n", .{}) catch {};
    } else {
        for (coords) |coord| {
            stdout.print("  (声母.{s}, 韵母.{s}, 声调.{s})\n", .{
                coord.point.shengmu_identity,
                coord.point.yunmu_identity,
                coord.point.tone_identity,
            }) catch {};
            stdout.print("    -> 声母.{s}, 韵母.{s}, 声调.{s}\n", .{
                coord.shengmu_symbol, coord.yunmu_symbol, coord.tone_symbol,
            }) catch {};
        }
    }

    stdout.print("{s}\n\n", .{sep}) catch {};
}

// === 测试 ===

test "radix3d_space_build" {
    const data = space_data;
    // 6 links × 4 tones = 24 coordinates
    try std.testing.expect(data.coordinate_count >= 24);
}

test "radix3d_find_coordinate" {
    const coord = findCoordinate("b", "a", "1").?;
    try std.testing.expectEqualStrings("b", coord.shengmu_symbol);
    try std.testing.expectEqualStrings("a", coord.yunmu_symbol);
    try std.testing.expectEqualStrings("1", coord.tone_symbol);
}

test "radix3d_ontology_distance" {
    const d = ontologyDistance("首", "次");
    try std.testing.expectEqual(@as(u8, 1), d);

    const d2 = ontologyDistance("首", "三");
    try std.testing.expectEqual(@as(u8, 2), d2);
}

test "radix3d_distance" {
    const p1 = Point3D{ .shengmu_identity = "首", .yunmu_identity = "首", .tone_identity = "首" };
    const p2 = Point3D{ .shengmu_identity = "次", .yunmu_identity = "次", .tone_identity = "次" };
    const d = distance(p1, p2);
    try std.testing.expectEqual(@as(u16, 3), d);
}

test "radix3d_symbol_to_identity" {
    const id = symbolToIdentity("声母", "b").?;
    try std.testing.expectEqualStrings("首", id);

    const id2 = symbolToIdentity("声母", "p").?;
    try std.testing.expectEqualStrings("次", id2);
}
