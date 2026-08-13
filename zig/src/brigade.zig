// brigade.zig — 兵团作战层级调度引擎
//
// 时间线性统治空间相位: layer 方向 = 时间轴(统治), col 方向 = 空间相位(被统治)
// 兵团作战 = 六层兵力协同指挥, 每层既是独立作战单位又是整体的一部分
// 正交集分布式合作 = 层间正交、任务分布式派发、结果聚合

const std = @import("std");
const ColorQuant = @import("color_quant.zig");
const ColorOntology = @import("color_ontology.zig");
const Hierarchy = @import("hierarchy.zig");
const PreloadDict = @import("preload_dict.zig");

// ── 指挥层级 ──
pub const CommandRank = enum(u8) {
    general = 0,    // 统帅 (layer 0: 妃, 夫妻共同体)
    strategist = 1, // 军师 (layer 1)
    commander = 2,  // 将领 (layer 2)
    captain = 3,    // 尉官 (layer 3)
    sergeant = 4,   // 士官 (layer 4)
    soldier = 5,    // 士兵 (layer 5: 黟, 边界执行层)
};

// ── 时间戳 (线性推进) ──
pub const TimeStamp = struct {
    tick: u32,        // 绝对时钟
    layer: u8,        // 当前生效层
    phase: f32,       // 层内相位 [0, 360)

    // 时间线性推进: 每推进一层, 相位重置
    pub fn advance(t: TimeStamp, delta: u32) TimeStamp {
        var nt = t;
        nt.tick += delta;
        nt.phase += @as(f32, @floatFromInt(delta)) * (360.0 / 6.0);
        if (nt.phase >= 360.0) {
            nt.phase -= 360.0;
            nt.layer = @as(u8, @intCast((nt.layer + 1) % 6));
        }
        return nt;
    }
};

// ── 空间相位 (被统治方) ──
pub const SpacePhase = struct {
    col: u8,          // 列 [0, 7)
    hue_angle: f32,   // 色相角
    voice_angle: f32,  // 声相角

    pub fn fromPosition(layer: u8, col: u8) SpacePhase {
        const cs = ColorQuant.ColorState.fromPosition(layer, col);
        return .{ .col = col, .hue_angle = cs.hue_angle, .voice_angle = cs.voice_angle };
    }
};

// ── 作战单元 (每层一个) ──
pub const Unit = struct {
    rank: CommandRank,
    layer: u8,
    strength: f32,        // 战斗力 = 管辖范围内色数 / 42
    authority: f32,       // 权威 = 1/(layer+1), 越窄层权威越高
    members: [7]SpacePhase, // 本层 7 列 (空间相位阵列)

    pub fn deploy(layer: u8) Unit {
        var u: Unit = undefined;
        u.rank = @enumFromInt(layer);
        u.layer = layer;
        u.authority = 1.0 / @as(f32, @floatFromInt(layer + 1));
        u.strength = 7.0 / 42.0;

        for (0..7) |c| {
            u.members[c] = SpacePhase.fromPosition(layer, @intCast(c));
        }
        return u;
    }
};

// ── 兵团调度器 (核心) ──
pub const Brigade = struct {
    time: TimeStamp,
    units: [6]Unit,
    center: ColorOntology.Center,
    boundary: ColorOntology.Boundary,

    pub fn boot() Brigade {
        var b: Brigade = undefined;
        b.time = .{ .tick = 0, .layer = 0, .phase = 0.0 };
        b.center = ColorOntology.Center.compute();
        b.boundary = ColorOntology.Boundary.compute();

        for (0..6) |i| {
            b.units[i] = Unit.deploy(@intCast(i));
        }
        return b;
    }

    // ── 时间线性统治空间相位 ──
    pub fn govern(b: *Brigade, target_layer: u8, target_col: u8) f32 {
        const unit = b.units[target_layer];
        const phase = unit.members[target_col];

        const layer_dist = if (target_layer > b.time.layer)
            target_layer - b.time.layer
        else
            b.time.layer - target_layer;
        const time_decay = 1.0 / @as(f32, @floatFromInt(layer_dist + 1));

        const phase_diff = @abs(phase.hue_angle - b.time.phase);
        const phase_align = @cos(phase_diff * std.math.pi / 180.0);

        return unit.authority * time_decay * phase_align;
    }

    // ── 兵团作战: 六层协同计算 ──
    pub fn corpsCompute(b: *Brigade, target_layer: u8, target_col: u8) f32 {
        var total: f32 = 0.0;
        var weight_sum: f32 = 0.0;

        for (0..6) |i| {
            const w = b.govern(@intCast(i), 0);
            const cs = ColorQuant.ColorState.fromPosition(target_layer, target_col);
            const contribution = w * (cs.hue_angle / 360.0);
            total += contribution;
            weight_sum += w;
        }

        if (weight_sum > 0.001) {
            return total / weight_sum;
        }
        return 0.0;
    }

    // ── 正交集分布式合作 ──
    pub fn orthogonalCooperate(b: *Brigade, symbol: u8) CooperationResult {
        const color_idx = PreloadDict.symbolToColor(symbol) orelse 0;
        const target_layer = color_idx / 7;
        const target_col = color_idx % 7;

        var result: CooperationResult = undefined;
        result.symbol = symbol;
        result.color_idx = color_idx;

        for (0..6) |i| {
            result.layer_contrib[i] = b.govern(@intCast(i), target_col);
        }

        result.aggregate = b.corpsCompute(target_layer, target_col);
        result.witness = (result.layer_contrib[1] + result.layer_contrib[3] + result.layer_contrib[5]) / 3.0;

        return result;
    }

    // ── 推进时间线 ──
    pub fn tick(b: *Brigade, delta: u32) void {
        b.time = b.time.advance(delta);
    }
};

// ── 合作结果 ──
pub const CooperationResult = struct {
    symbol: u8,
    color_idx: u8,
    layer_contrib: [6]f32,
    aggregate: f32,
    witness: f32,
};

// ── 测试 ──
test "brigade_boot" {
    const b = Brigade.boot();
    try std.testing.expectEqual(@as(u32, 0), b.time.tick);
    try std.testing.expectEqual(@as(u8, 0), b.time.layer);
}

test "unit_deploy" {
    const u = Unit.deploy(0);
    try std.testing.expectEqual(@as(u8, 0), u.layer);
    try std.testing.expect(u.authority > 0.0 and u.authority <= 1.0);
    try std.testing.expectEqual(@as(f32, 7.0 / 42.0), u.strength);
}

test "time_advance" {
    var t = TimeStamp{ .tick = 0, .layer = 0, .phase = 0.0 };
    t = t.advance(1);
    try std.testing.expectEqual(@as(u32, 1), t.tick);
    try std.testing.expectApproxEqAbs(60.0, t.phase, 0.01);
}

test "govern_decay" {
    var b = Brigade.boot();
    const g0 = b.govern(0, 0);
    const g5 = b.govern(5, 0);
    try std.testing.expect(g0 > g5);
}

test "corps_compute" {
    var b = Brigade.boot();
    const result = b.corpsCompute(0, 0);
    try std.testing.expect(result >= 0.0 and result <= 1.0);
}

test "orthogonal_cooperate" {
    var b = Brigade.boot();
    const r = b.orthogonalCooperate(0);
    try std.testing.expectEqual(@as(u8, 0), r.symbol);
    try std.testing.expectEqual(@as(u8, 0), r.color_idx);
    try std.testing.expect(r.witness >= -1.0 and r.witness <= 1.0);
}

test "tick_advances_time" {
    var b = Brigade.boot();
    b.tick(6);
    try std.testing.expectEqual(@as(u32, 6), b.time.tick);
}
