// color_quant.zig - Quantitative State Engine Core (Zig implementation)
//
// Quantification dimensions:
//   position -> state (layer, col) -> hue angle, voice angle, spectrum, invariant
//   spectral blend: voice phase x color phase = hue transformation
//   addition governs multiplication: narrow layer governs wide layer
//   power-of-2 threshold: 254 missing 2 -> 256 = 2^8
//   mirror logarithmization
//   three distances: shortest / total / longest
//   static invariant = temporal constancy

const std = @import("std");
const math = std.math;

// === Constants ===
pub const FULL_CIRCLE: f32 = 360.0;
pub const VOICE_CIRCLE: f32 = 384.0;
pub const LAYER_COUNT: u8 = 6;
pub const COL_COUNT: u8 = 7;
pub const COLOR_COUNT: u8 = 42;
pub const SYMBOL_COUNT: u8 = 64;
pub const POWER_THRESHOLD: f32 = 256.0;

// === Color Quantitative State ===
pub const ColorState = struct {
    layer: u8 = 0,
    col: u8 = 0,
    color_idx: u8 = 0,
    hue_angle: f32 = 0.0,
    voice_angle: f32 = 0.0,
    layer_depth: f32 = 0.0,
    spectral: f32 = 0.0,
    mirror: f32 = 0.0,
    spectral_log: f32 = 0.0,
    invariant: f32 = 0.0,
    magnitude: f32 = 0.0,
    dist_shortest: f32 = 0.0,
    dist_total: f32 = 0.0,
    dist_longest: f32 = 384.0,

    /// Create state from layer/col position (core constructor)
    pub fn fromPosition(layer: u8, col: u8) ColorState {
        var s: ColorState = .{};
        const l = if (layer >= LAYER_COUNT) 0 else layer;
        const c = if (col >= COL_COUNT) 0 else col;

        s.layer = l;
        s.col = c;
        s.color_idx = l * COLOR_COUNT + c;

        // Hue angle: each layer 60 deg, each col (60/7) deg
        s.hue_angle = @as(f32, @floatFromInt(l)) * (FULL_CIRCLE / @as(f32, @floatFromInt(LAYER_COUNT)))
                    + @as(f32, @floatFromInt(c)) * (FULL_CIRCLE / @as(f32, @floatFromInt(LAYER_COUNT)) / @as(f32, @floatFromInt(COL_COUNT)));

        // Voice angle: 42 colors evenly split 384 deg circle
        s.voice_angle = @as(f32, @floatFromInt(l * COL_COUNT + c)) * (VOICE_CIRCLE / @as(f32, @floatFromInt(COLOR_COUNT)));

        s.layer_depth = @as(f32, @floatFromInt(l));
        s.spectral = s.hue_angle / FULL_CIRCLE;
        s.mirror = @rem(FULL_CIRCLE - s.hue_angle, FULL_CIRCLE);
        s.spectral_log = @log(1.0 + s.spectral * (@exp(1.0) - 1.0));

        // Invariant: position identity
        const nh = s.hue_angle / FULL_CIRCLE;
        const nv = s.voice_angle / VOICE_CIRCLE;
        const nd = s.layer_depth / @as(f32, @floatFromInt(LAYER_COUNT - 1));
        s.invariant = @sqrt(nh * nh + nv * nv + nd * nd);
        s.magnitude = @sqrt(s.hue_angle * s.hue_angle + s.voice_angle * s.voice_angle);

        // Distances
        s.dist_shortest = @sqrt(nh * nh + nv * nv + nd * nd) / @sqrt(3.0);

        // Total distance to endpoint (layer 5, col 6)
        const eh = @as(f32, @floatFromInt(LAYER_COUNT - 1)) * (FULL_CIRCLE / @as(f32, @floatFromInt(LAYER_COUNT)))
                 + @as(f32, @floatFromInt(COL_COUNT - 1)) * (FULL_CIRCLE / @as(f32, @floatFromInt(LAYER_COUNT)) / @as(f32, @floatFromInt(COL_COUNT)));
        const ev = @as(f32, @floatFromInt((LAYER_COUNT - 1) * COL_COUNT + (COL_COUNT - 1))) * (VOICE_CIRCLE / @as(f32, @floatFromInt(COLOR_COUNT)));
        const el = @as(f32, @floatFromInt(LAYER_COUNT - 1));
        const dh = (s.hue_angle - eh) / FULL_CIRCLE;
        const dv = (s.voice_angle - ev) / VOICE_CIRCLE;
        const dl = (s.layer_depth - el) / @as(f32, @floatFromInt(LAYER_COUNT - 1));
        s.dist_total = @sqrt(dh * dh + dv * dv + dl * dl) / @sqrt(3.0);

        s.dist_longest = VOICE_CIRCLE;
        return s;
    }
};

// === Spectral Blend (voice x color -> hue transformation) ===
pub fn spectralBlend(voice_angle: f32, hue_angle: f32) f32 {
    const nv = voice_angle / VOICE_CIRCLE;
    const nh = hue_angle / FULL_CIRCLE;
    const blended = (nv + nh) * 0.5;
    return @rem(blended * FULL_CIRCLE, FULL_CIRCLE);
}

// === Addition Governs Multiplication ===
pub fn additionGovernsMultiplication(narrow_layer: u8, wide_layer: u8) f32 {
    if (wide_layer <= narrow_layer) return 1.0;
    const add_power = @as(f32, @floatFromInt(narrow_layer + 1));
    const mul_expansion = std.math.pow(f32, 2.0, @as(f32, @floatFromInt(wide_layer - narrow_layer)));
    return add_power / mul_expansion;
}

// === Power Gap Detection ===
pub fn powerGap(accumulated: f32) u8 {
    if (accumulated >= POWER_THRESHOLD) return 0;
    if (accumulated >= 255.0) return 1;
    return 2;
}

// === Mirror Angle ===
pub fn mirrorAngle(hue_angle: f32) f32 {
    return @rem(FULL_CIRCLE - hue_angle, FULL_CIRCLE);
}

// === Edge Logarithm (GCD-based) ===
pub fn edgeLogarithm(a: u8, b: u8) f32 {
    const x = a + 1;
    const y = b + 1;
    const g = gcd(x, y);
    const max_val: f32 = @as(f32, @floatFromInt(if (a > b) a else b)) + 1.0;
    if (max_val <= 1.0) return 0.0;
    return @log2(@as(f32, @floatFromInt(g))) / @log2(max_val);
}

fn gcd(a: u8, b: u8) u8 {
    var x = a;
    var y = b;
    while (y != 0) {
        const t = y;
        y = x % y;
        x = t;
    }
    return if (x == 0) 1 else x;
}

// === Engine Report ===
pub fn engineReport() void {
    const stdout = std.io.getStdOut().writer();
    stdout.print("\n  === Quantitative State Engine (Zig) ===\n", .{}) catch {};

    const s00 = ColorState.fromPosition(0, 0);
    const s56 = ColorState.fromPosition(5, 6);
    const s23 = ColorState.fromPosition(2, 3);

    stdout.print("  妃(R0):  hue={d:.1} voice={d:.1} spectral={d:.4} invariant={d:.6}\n", .{
        s00.hue_angle, s00.voice_angle, s00.spectral, s00.invariant,
    }) catch {};

    stdout.print("  黟(L6):  hue={d:.1} voice={d:.1} spectral={d:.4} invariant={d:.6}\n", .{
        s56.hue_angle, s56.voice_angle, s56.spectral, s56.invariant,
    }) catch {};

    stdout.print("  青(G3):  hue={d:.1} voice={d:.1} spectral={d:.4}\n", .{
        s23.hue_angle, s23.voice_angle, s23.spectral,
    }) catch {};

    stdout.print("  [Distances] shortest(R0)={d:.4} total(R0)={d:.4} longest={d:.1}\n", .{
        s00.dist_shortest, s00.dist_total, s00.dist_longest,
    }) catch {};

    stdout.print("  [Addition governs] layer0->layer1={d:.4} layer0->layer5={d:.6}\n", .{
        additionGovernsMultiplication(0, 1), additionGovernsMultiplication(0, 5),
    }) catch {};

    stdout.print("  [Power gap] 254->{} 256->{}\n", .{ powerGap(254.0), powerGap(256.0) }) catch {};
    stdout.print("  [Spectral blend] 0deg + 120deg = {d:.1}deg\n", .{ spectralBlend(60.0, 120.0) }) catch {};
    stdout.print("  ======================================\n\n", .{}) catch {};
}

// === Tests ===
test "from_position_origin" {
    const s = ColorState.fromPosition(0, 0);
    try std.testing.expectEqual(@as(u8, 0), s.layer);
    try std.testing.expectEqual(@as(u8, 0), s.col);
    try std.testing.expectApproxEqAbs(@as(f32, 0.0), s.hue_angle, 0.01);
    try std.testing.expectApproxEqAbs(@as(f32, 0.0), s.spectral, 0.01);
}

test "from_position_endpoint" {
    const s = ColorState.fromPosition(5, 6);
    try std.testing.expectEqual(@as(u8, 5), s.layer);
    try std.testing.expect(s.hue_angle > 350.0 and s.hue_angle <= 360.0);
    try std.testing.expectApproxEqAbs(@as(f32, 384.0), s.dist_longest, 0.01);
}

test "invariant_constant" {
    const a = ColorState.fromPosition(3, 4);
    const b = ColorState.fromPosition(3, 4);
    try std.testing.expectApproxEqAbs(a.invariant, b.invariant, 0.0001);
}

test "spectral_blend" {
    const blended = spectralBlend(60.0, 120.0);
    try std.testing.expect(blended >= 0.0 and blended <= 360.0);
}

test "addition_governs" {
    const g01 = additionGovernsMultiplication(0, 1);
    const g05 = additionGovernsMultiplication(0, 5);
    try std.testing.expect(g01 > g05); // closer layer = stronger governance
    try std.testing.expectEqual(@as(f32, 1.0), additionGovernsMultiplication(3, 3));
}

test "power_gap" {
    try std.testing.expectEqual(@as(u8, 2), powerGap(254.0));
    try std.testing.expectEqual(@as(u8, 1), powerGap(255.0));
    try std.testing.expectEqual(@as(u8, 0), powerGap(256.0));
    try std.testing.expectEqual(@as(u8, 0), powerGap(300.0));
}

test "mirror_angle" {
    try std.testing.expectApproxEqAbs(@as(f32, 270.0), mirrorAngle(90.0), 0.01);
    try std.testing.expectApproxEqAbs(@as(f32, 180.0), mirrorAngle(180.0), 0.01);
    try std.testing.expectApproxEqAbs(@as(f32, 90.0), mirrorAngle(270.0), 0.01);
}

test "edge_logarithm" {
    const el = edgeLogarithm(6, 4);
    try std.testing.expect(el >= 0.0 and el <= 1.0);
}
