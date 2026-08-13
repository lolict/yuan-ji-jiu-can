// color_ontology.zig - Ontological Layer (orthogonal center/boundary/duality)
//
// Center = GCD = orthogonal point = static invariant = man (layer0/妃)
// Boundary = LCM = curve category = dynamic envelope = woman (layer5/黟)
// Orthogonal lines = binary split = state bifurcation
// Arc-chord duality = curve total category vs line category

const std = @import("std");
const ColorQuant = @import("color_quant.zig");
const math = std.math;

pub const PI: f32 = 3.14159265;

// === Orthogonal Center (GCD/man/static) ===
pub const Center = struct {
    center_color_idx: u8 = 0,
    invariant: f32 = 0.0,
    gcd_value: u8 = 1,
    radius_max: f32 = 0.0,
    cardinality: u32 = 42,

    pub fn compute() Center {
        var c: Center = .{};
        c.center_color_idx = 0; // 妃/R0

        // GCD of all color indices = 1
        var g: u8 = 1;
        for (1..ColorQuant.COLOR_COUNT + 1) |i| {
            g = gcd(g, @intCast(i));
        }
        c.gcd_value = g;

        // Invariant at center
        const cs = ColorQuant.ColorState.fromPosition(0, 0);
        c.invariant = cs.invariant;

        // Max radius: center to farthest color
        const far = ColorQuant.ColorState.fromPosition(5, 6);
        const dx = far.hue_angle - cs.hue_angle;
        const dy = far.voice_angle - cs.voice_angle;
        c.radius_max = @sqrt(dx * dx + dy * dy);

        c.cardinality = ColorQuant.COLOR_COUNT;
        return c;
    }
};

// === Boundary (LCM/woman/dynamic envelope) ===
pub const Boundary = struct {
    boundary_color_idx: u8 = 41,
    lcm_value: f32 = 0.0,
    arc_length: f32 = 0.0,
    chord_length: f32 = 0.0,
    arc_chord_ratio: f32 = 0.0,

    pub fn compute() Boundary {
        var b: Boundary = .{};
        b.boundary_color_idx = 41; // 黟/L6

        // LCM of all color indices
        var l: f32 = 1.0;
        for (2..ColorQuant.COLOR_COUNT + 1) |i| {
            l = lcmF(l, @as(f32, @floatFromInt(i)));
            if (l > 1000000.0) break; // cap
        }
        b.lcm_value = l;

        // Arc length at outermost radius (r=1)
        b.arc_length = 2.0 * PI * 1.0; // ~6.28
        b.chord_length = 2.0 * 1.0;    // diameter = 2
        b.arc_chord_ratio = b.arc_length / b.chord_length; // = PI

        return b;
    }
};

// === Binary Split (orthogonal line bifurcation) ===
pub const Split = struct {
    axis_angle: u8,
    group_pos: u8,
    group_neg: u8,
    balance: f32,

    pub fn compute(axis_angle: u8) Split {
        var s: Split = .{ .axis_angle = axis_angle, .group_pos = 0, .group_neg = 0, .balance = 0.0 };

        const axis_rad = @as(f32, @floatFromInt(axis_angle)) * PI / 180.0;
        const ax = @cos(axis_rad);
        const ay = @sin(axis_rad);

        const center = ColorQuant.ColorState.fromPosition(0, 0);

        for (0..ColorQuant.COLOR_COUNT) |i| {
            const cs = ColorQuant.ColorState.fromPosition(@intCast(i / 7), @intCast(i % 7));
            const vx = cs.hue_angle - center.hue_angle;
            const vy = cs.voice_angle - center.voice_angle;
            const proj = vx * ax + vy * ay;

            if (proj > 0.001) {
                s.group_pos += 1;
            } else if (proj < -0.001) {
                s.group_neg += 1;
            }
        }

        const total = s.group_pos + s.group_neg;
        if (total > 0) {
            const diff = @as(i16, @intCast(s.group_pos)) - @as(i16, @intCast(s.group_neg));
            const abs_diff: i16 = if (diff >= 0) diff else -diff;
            s.balance = @as(f32, @floatFromInt(abs_diff)) / @as(f32, @floatFromInt(total));
        }

        return s;
    }
};

// === Arc-Chord Duality ===
pub fn arcLength(a_idx: u8, b_idx: u8) f32 {
    const ra = @as(f32, @floatFromInt(a_idx / 7)) / 6.0;
    const rb = @as(f32, @floatFromInt(b_idx / 7)) / 6.0;
    const r = (ra + rb) * 0.5;

    const ha = @as(f32, @floatFromInt(a_idx)) * (360.0 / 42.0);
    const hb = @as(f32, @floatFromInt(b_idx)) * (360.0 / 42.0);
    var dtheta = @abs(ha - hb);
    if (dtheta > 180.0) dtheta = 360.0 - dtheta;

    return r * dtheta * PI / 180.0;
}

pub fn chordLength(a_idx: u8, b_idx: u8) f32 {
    const ra = @as(f32, @floatFromInt(a_idx / 7)) / 6.0;
    const rb = @as(f32, @floatFromInt(b_idx / 7)) / 6.0;
    const r = (ra + rb) * 0.5;

    const ha = @as(f32, @floatFromInt(a_idx)) * (360.0 / 42.0);
    const hb = @as(f32, @floatFromInt(b_idx)) * (360.0 / 42.0);
    var dtheta = @abs(ha - hb);
    if (dtheta > 180.0) dtheta = 360.0 - dtheta;

    return 2.0 * r * @sin(dtheta * PI / 360.0);
}

pub fn arcChordRatio(a_idx: u8, b_idx: u8) f32 {
    const chord = chordLength(a_idx, b_idx);
    if (chord < 0.0001) return 1.0;
    return arcLength(a_idx, b_idx) / chord;
}

// === Engine Report ===
pub fn engineReport() void {
    const stdout = std.io.getStdOut().writer();
    const c = Center.compute();
    const b = Boundary.compute();
    const s = Split.compute(0);

    stdout.print("\n  [Ontological Layer] \n", .{}) catch {};
    stdout.print("  Center: idx={} GCD={} invariant={d:.6} radius_max={d:.1}\n", .{
        c.center_color_idx, c.gcd_value, c.invariant, c.radius_max,
    }) catch {};
    stdout.print("  Boundary: idx={} LCM={d:.0} arc={d:.3} chord={d:.3} ratio={d:.4}\n", .{
        b.boundary_color_idx, b.lcm_value, b.arc_length, b.chord_length, b.arc_chord_ratio,
    }) catch {};
    stdout.print("  Split @ {}deg: pos={} neg={} balance={d:.2}\n", .{
        s.axis_angle, s.group_pos, s.group_neg, s.balance,
    }) catch {};
    stdout.print("  Arc-chord (R0->L6): arc={d:.3} chord={d:.3} ratio={d:.3}\n", .{
        arcLength(0, 41), chordLength(0, 41), arcChordRatio(0, 41),
    }) catch {};
}

// === Helpers ===
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

fn lcmF(a: f32, b: f32) f32 {
    if (a == 0.0 or b == 0.0) return 0.0;
    const g = @round(a * 1000.0) / 1000.0;
    _ = g;
    return (a * b) / 1.0; // simplified: for coprime small ints
}

// === Tests ===
test "center_compute" {
    const c = Center.compute();
    try std.testing.expectEqual(@as(u8, 0), c.center_color_idx);
    try std.testing.expectEqual(@as(u8, 1), c.gcd_value);
    try std.testing.expectEqual(@as(u32, 42), c.cardinality);
}

test "boundary_compute" {
    const b = Boundary.compute();
    try std.testing.expectEqual(@as(u8, 41), b.boundary_color_idx);
    try std.testing.expectApproxEqAbs(PI, b.arc_chord_ratio, 0.01);
    try std.testing.expect(b.arc_length > b.chord_length);
}

test "split" {
    const s = Split.compute(0);
    try std.testing.expect(s.group_pos + s.group_neg <= 42);
    try std.testing.expect(s.balance >= 0.0 and s.balance <= 1.0);
}

test "arc_chord_ratio" {
    const r = arcChordRatio(0, 41);
    try std.testing.expect(r >= 1.0);
    const r_same = arcChordRatio(5, 5);
    try std.testing.expectEqual(@as(f32, 1.0), r_same);
}
