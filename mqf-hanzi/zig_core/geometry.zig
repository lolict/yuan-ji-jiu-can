// 满全法 - Zig 底层几何运算 (English identifiers - Zig supports ASCII only)
// Static compilation, zero overhead, direct machine code generation

const std = @import("std");

/// 64-bit cell coordinate
pub const Cell = extern struct {
    x: u16,        // left-right axis
    y: u16,        // up-down axis
    z: u16,        // depth axis
    polarity: u8,  // polarity (yin/yang)
    _pad: u8 = 0,
};

/// Component pair (left-right relationship)
pub const CompPair = struct {
    left: Cell,
    right: Cell,
    affinity: f64,
};

/// 3x3 transformation matrix
pub const Mat3x3 = [3][3]f64;

/// Manhattan distance
pub fn manhattan(a: Cell, b: Cell) u32 {
    const dx = if (a.x > b.x) a.x - b.x else b.x - a.x;
    const dy = if (a.y > b.y) a.y - b.y else b.y - a.y;
    const dz = if (a.z > b.z) a.z - b.z else b.z - a.z;
    return dx + dy + dz;
}

/// Euclidean distance squared
pub fn euclideanSq(a: Cell, b: Cell) f64 {
    const dx = @as(f64, @floatFromInt(a.x)) - @as(f64, @floatFromInt(b.x));
    const dy = @as(f64, @floatFromInt(a.y)) - @as(f64, @floatFromInt(b.y));
    const dz = @as(f64, @floatFromInt(a.z)) - @as(f64, @floatFromInt(b.z));
    return dx * dx + dy * dy + dz * dz;
}

/// Affinity (0.0 ~ 1.0)
pub fn affinity(a: Cell, b: Cell) f64 {
    const dist = manhattan(a, b);
    return 1.0 / (1.0 + @as(f64, @floatFromInt(dist)) / 1000.0);
}

/// Matrix multiply
pub fn matMul(a: Mat3x3, b: Mat3x3) Mat3x3 {
    var r: Mat3x3 = undefined;
    var i: usize = 0;
    while (i < 3) : (i += 1) {
        var j: usize = 0;
        while (j < 3) : (j += 1) {
            r[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j];
        }
    }
    return r;
}

/// Matrix apply to vector
pub fn matApply(m: Mat3x3, v: [3]f64) [3]f64 {
    var r: [3]f64 = undefined;
    var i: usize = 0;
    while (i < 3) : (i += 1) {
        r[i] = m[i][0] * v[0] + m[i][1] * v[1] + m[i][2] * v[2];
    }
    return r;
}

/// Identity matrix
pub fn identity() Mat3x3 {
    return .{
        .{ 1.0, 0.0, 0.0 },
        .{ 0.0, 1.0, 0.0 },
        .{ 0.0, 0.0, 1.0 },
    };
}

/// Scale matrix
pub fn scale(sx: f64, sy: f64, sz: f64) Mat3x3 {
    return .{
        .{ sx, 0.0, 0.0 },
        .{ 0.0, sy, 0.0 },
        .{ 0.0, 0.0, sz },
    };
}

/// Level constants
pub const Level = enum(u8) {
    yuan1 = 1,
    yuan2 = 2,
    yuan3 = 3,
    yuan4 = 4,
    yuan5 = 5,
    yuan6 = 6,
    yuan7 = 7,
    yuan8 = 8,
    yuan9 = 9,
};

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    
    try stdout.print("=== Manquanfa Zig Geometry Core ===\n\n", .{});
    
    const nv = Cell{ .x = 100, .y = 200, .z = 50, .polarity = 0 };
    const zi = Cell{ .x = 300, .y = 200, .z = 50, .polarity = 1 };
    const ri = Cell{ .x = 150, .y = 180, .z = 40, .polarity = 1 };
    const yue = Cell{ .x = 320, .y = 180, .z = 40, .polarity = 0 };
    
    try stdout.print("Distance test:\n", .{});
    try stdout.print("  nv-zi (good): manhattan={d} euclidSq={d:.2}\n", .{ manhattan(nv, zi), euclideanSq(nv, zi) });
    try stdout.print("  ri-yue (bright): manhattan={d} euclidSq={d:.2}\n", .{ manhattan(ri, yue), euclideanSq(ri, yue) });
    try stdout.print("  nv-ri (cross): manhattan={d} euclidSq={d:.2}\n", .{ manhattan(nv, ri), euclideanSq(nv, ri) });
    
    try stdout.print("\nAffinity:\n", .{});
    try stdout.print("  good(nv+zi) affinity={d:.6}\n", .{ affinity(nv, zi) });
    try stdout.print("  bright(ri+yue) affinity={d:.6}\n", .{ affinity(ri, yue) });
    
    try stdout.print("\nMatrix transform:\n", .{});
    const s = scale(2.0, 0.5, 1.0);
    const v = [3]f64{ 100.0, 200.0, 50.0 };
    const tv = matApply(s, v);
    try stdout.print("  original: [{d:.1}, {d:.1}, {d:.1}]\n", .{ v[0], v[1], v[2] });
    try stdout.print("  scaled:   [{d:.1}, {d:.1}, {d:.1}]\n", .{ tv[0], tv[1], tv[2] });
    
    try stdout.print("\nLevel system:\n", .{});
    const levels = [_]Level{
        .yuan1, .yuan2, .yuan3, .yuan4, .yuan5,
        .yuan6, .yuan7, .yuan8, .yuan9,
    };
    for (levels) |level| {
        const depth = @intFromEnum(level);
        const bits = @as(u16, depth) * 64;
        try stdout.print("  {}: depth={d}, bits={d}-bit\n", .{ level, depth, bits });
    }
    
    try stdout.print("\n=== Geometry Core Ready ===\n", .{});
}
