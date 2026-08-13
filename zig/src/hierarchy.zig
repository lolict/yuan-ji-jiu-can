// hierarchy.zig - Hierarchical Governance (time-linear governing space-phase)
//
// Core principle:
//   Time is linear/sequential/causal => GOVERNS => space (phase/distributed/parallel)
//   Narrow layer (addition) governs wide layer (multiplication)
//   This is the "corps warfare" hierarchy: time-line commander dispatches space observers
//
// Hierarchy structure:
//   Layer 0 (couple) = commander innermost = GCD = man = static
//   Layer 1 (center) = staff officer
//   Layer 2 (perception) = scout/recon
//   Layer 3 (backbone) = infantry
//   Layer 4 (horizontal) = artillery
//   Layer 5 (light/high-dim) = outermost boundary = LCM = woman = dynamic envelope

const std = @import("std");
const ColorQuant = @import("color_quant.zig");

pub const LAYER_COUNT: u8 = 6;

// === Layer descriptor ===
pub const Layer = struct {
    index: u8,
    name: []const u8,
    role: []const u8,          // military role
    operation: []const u8,     // add or multiply
    governs: ?u8,               // which wider layer it governs
    governed_by: ?u8,           // which narrower layer governs it
    gcd_with_next: u8,          // GCD with next layer
    lcm_with_next: u16,         // LCM with next layer
};

// === Full hierarchy descriptor ===
pub const Hierarchy = struct {
    layers: [LAYER_COUNT]Layer,
    center_color_idx: u8,
    boundary_color_idx: u8,
    center_gcd: u8,
    boundary_lcm: u16,
};

/// Compute the full hierarchy
pub fn compute() Hierarchy {
    var h: Hierarchy = undefined;

    const layer_names = [LAYER_COUNT][]const u8{
        "夫妻层", "中心层", "感知层", "骨干层", "行页层", "高维层",
    };
    const roles = [LAYER_COUNT][]const u8{
        "将军/统帅", "参谋/副将", "侦察/先锋", "步兵/主力", "炮兵/支援", "边界/斥候",
    };
    const ops = [LAYER_COUNT][]const u8{
        "加法", "加法->乘法", "乘法", "乘法", "乘法", "乘法->幂",
    };

    for (0..LAYER_COUNT) |i| {
        const idx: u8 = @intCast(i);
        const next: u8 = @intCast((i + 1) % LAYER_COUNT);
        h.layers[i] = .{
            .index = idx,
            .name = layer_names[i],
            .role = roles[i],
            .operation = ops[i],
            .governs = if (i < LAYER_COUNT - 1) next else null,
            .governed_by = if (i > 0) idx - 1 else null,
            .gcd_with_next = gcd8(idx + 1, next + 1),
            .lcm_with_next = lcmU16(@intCast(i + 1), @intCast(next + 1)),
        };
    }

    h.center_color_idx = 0;     // R0 = 妃
    h.boundary_color_idx = 41;  // L6 = 黟

    // Compute GCD of all layers
    var g: u8 = 1;
    for (1..LAYER_COUNT + 1) |i| {
        g = gcd8(g, @intCast(i));
    }
    h.center_gcd = g;

    // Compute LCM of all layers
    var l: u16 = 1;
    for (1..LAYER_COUNT + 1) |i| {
        l = lcmU16(l, @intCast(i));
    }
    h.boundary_lcm = l;

    return h;
}

/// Get governance chain (who governs whom)
pub fn governanceChain(h: *const Hierarchy, from_layer: u8) []const u8 {
    _ = h;
    _ = from_layer;
    // Return the chain of command from this layer outward
    // For now return the role names
    return &.{};
}

fn gcd8(a: u8, b: u8) u8 {
    var x = a;
    var y = b;
    while (y != 0) {
        const t = y;
        y = x % y;
        x = t;
    }
    return if (x == 0) 1 else x;
}

fn lcmU16(a: u16, b: u16) u16 {
    if (a == 0 or b == 0) return 0;
    const g = gcdU16(a, b);
    return (a / g) * b;
}

fn gcdU16(a: u16, b: u16) u16 {
    var x = a;
    var y = b;
    while (y != 0) {
        const t = y;
        y = x % y;
        x = t;
    }
    return if (x == 0) 1 else x;
}

/// Report hierarchy
pub fn report() void {
    const stdout = std.io.getStdOut().writer();
    const h = compute();
    stdout.print("\n  [Hierarchical Governance] \n", .{}) catch {};
    stdout.print("  center: layer0 (妃/R0) GCD={}\n", .{h.center_gcd}) catch {};
    stdout.print("  boundary: layer5 (黟/L6) LCM={}\n", .{h.boundary_lcm}) catch {};

    for (h.layers) |layer| {
        stdout.print("    Layer {}: {s} | role: {s} | op: {s}", .{
            layer.index, layer.name, layer.role, layer.operation,
        }) catch {};
        if (layer.governs) |g| {
            stdout.print(" | governs: {}", .{g}) catch {};
        }
        stdout.print("\n", .{}) catch {};
    }
    stdout.print("  chain: 妃(将军) -> 参谋 -> 侦察 -> 步兵 -> 炮兵 -> 黟(边界)\n", .{}) catch {};
}

test "hierarchy_compute" {
    const h = compute();
    try std.testing.expectEqual(@as(u8, 0), h.layers[0].index);
    try std.testing.expectEqual(@as(u8, 5), h.layers[5].index);
    try std.testing.expectEqual(@as(u8, 1), h.center_gcd);
}

test "gcd_lcm" {
    try std.testing.expectEqual(@as(u8, 1), gcd8(3, 5));
    try std.testing.expectEqual(@as(u8, 6), gcd8(6, 12));
    try std.testing.expectEqual(@as(u16, 12), lcmU16(3, 4));
    try std.testing.expectEqual(@as(u16, 60), lcmU16(12, 5));
}

test "governance_chain" {
    const h = compute();
    try std.testing.expect(h.layers[0].governs.? == 1);
    try std.testing.expect(h.layers[5].governs == null);
}
