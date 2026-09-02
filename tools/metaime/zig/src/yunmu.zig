// ============================================================
// 韵母向量表 · comptime 嵌入 · 零运行时
// ============================================================

const std = @import("std");

pub const YunmuVector = struct {
    direction: []const u8,
    force_desc: []const u8,
    vx: f32,
    vy: f32,
};

// ── 韵母向量表 (comptime 嵌入) ──
pub const YUNMU_TABLE = [_]YunmuVector{
    // 单韵母
    .{ .direction = "radial_out",     .force_desc = "发散力",     .vx = 0,   .vy = 1   }, // a
    .{ .direction = "circular",       .force_desc = "扭力/向心力", .vx = 1,   .vy = 0   }, // o
    .{ .direction = "diagonal",       .force_desc = "剪切力",     .vx = 1,   .vy = 1   }, // e
    .{ .direction = "linear",         .force_desc = "张力/拉力",  .vx = 0,   .vy = 1   }, // i
    .{ .direction = "pipe",           .force_desc = "压力/推力",  .vx = 1,   .vy = 0   }, // u
    .{ .direction = "narrow",         .force_desc = "聚焦力",     .vx = 0.5, .vy = 0.5 }, // ü
    // 复韵母
    .{ .direction = "radial_to_linear",   .force_desc = "发散→拉力",  .vx = 0,   .vy = 1.2 }, // ai
    .{ .direction = "diagonal_to_linear", .force_desc = "剪切→拉力",  .vx = 0.5, .vy = 1   }, // ei
    .{ .direction = "radial_to_circular", .force_desc = "发散→扭力",  .vx = 1.2, .vy = 1   }, // ao
    .{ .direction = "circular_to_pipe",   .force_desc = "扭力→压力",  .vx = 1,   .vy = 0.8 }, // ou
    .{ .direction = "radial_to_frame",    .force_desc = "发散→比较",  .vx = 0.5, .vy = 1.2 }, // an
    .{ .direction = "diagonal_to_frame",  .force_desc = "剪切→比较",  .vx = 0.8, .vy = 1   }, // en
    .{ .direction = "radial_to_deep",     .force_desc = "发散→承载",  .vx = 0.3, .vy = 1.5 }, // ang
    .{ .direction = "diagonal_to_deep",   .force_desc = "剪切→承载",  .vx = 0.5, .vy = 1.3 }, // eng
    .{ .direction = "circular_to_deep",   .force_desc = "扭力→承载",  .vx = 1,   .vy = 1   }, // ong
    // 介音复韵母
    .{ .direction = "linear_to_radial",   .force_desc = "拉力→发散",  .vx = 0.3, .vy = 1.3 }, // ia
    .{ .direction = "linear_to_diagonal", .force_desc = "拉力→剪切",  .vx = 0.5, .vy = 1   }, // ie
    .{ .direction = "pipe_to_radial",     .force_desc = "压力→发散",  .vx = 0.5, .vy = 1.2 }, // ua
    .{ .direction = "pipe_to_circular",   .force_desc = "压力→扭力",  .vx = 1,   .vy = 1   }, // uo
    .{ .direction = "linear_to_circular", .force_desc = "拉力→扭力",  .vx = 1,   .vy = 1.2 }, // iao
    .{ .direction = "linear_to_pipe",     .force_desc = "拉力→压力",  .vx = 0.8, .vy = 0.8 }, // iu
    .{ .direction = "pipe_to_linear",     .force_desc = "压力→拉力",  .vx = 0.5, .vy = 1.2 }, // uai
    .{ .direction = "pipe_to_diagonal",   .force_desc = "压力→剪切",  .vx = 0.8, .vy = 1   }, // ue
    .{ .direction = "narrow_to_diagonal", .force_desc = "聚焦→剪切",  .vx = 0.7, .vy = 1   }, // üe
};

// ┌─────────────────────────────────────────────────────────┐
// │ 韵母名称表 (与上表索引一致)                             │
// └─────────────────────────────────────────────────────────┘
pub const YUNMU_NAMES = [_][]const u8{
    "a", "o", "e", "i", "u", "v",
    "ai", "ei", "ao", "ou", "an", "en",
    "ang", "eng", "ong",
    "ia", "ie", "ua", "uo",
    "iao", "iu", "uai", "ue", "ve",
};

// 注意: ü 映射为 v (ASCII兼容)

// ┌─────────────────────────────────────────────────────────┐
// │ 通过名称查找韵母索引                                    │
// └─────────────────────────────────────────────────────────┘
pub fn findYunmu(name: []const u8) ?usize {
    for (YUNMU_NAMES, 0..) |ym_name, idx| {
        if (std.mem.eql(u8, ym_name, name)) {
            return idx;
        }
    }
    return null;
}

// ┌─────────────────────────────────────────────────────────┐
// │ 通过索引返回韵母向量                                    │
// └─────────────────────────────────────────────────────────┘
pub fn getYunmu(idx: usize) YunmuVector {
    return YUNMU_TABLE[@min(idx, YUNMU_TABLE.len - 1)];
}

// ┌─────────────────────────────────────────────────────────┐
// │ 声调能量系数 (comptime)                                │
// └─────────────────────────────────────────────────────────┘
pub fn toneEnergy(tone: u8) f32 {
    return switch (tone) {
        1 => 1.0,
        2 => 1.2,
        3 => 0.8,
        4 => 1.5,
        else => 1.0,
    };
}

// ┌─────────────────────────────────────────────────────────┐
// │ 单元测试                                               │
// └─────────────────────────────────────────────────────────┘
test "yunmu_find_a" {
    const idx = findYunmu("a").?;
    try std.testing.expectEqual(@as(usize, 0), idx);
}

test "tone_energy" {
    try std.testing.expectEqual(@as(f32, 1.0), toneEnergy(1));
    try std.testing.expectEqual(@as(f32, 1.5), toneEnergy(4));
}

test "yunmu_table_len" {
    try std.testing.expectEqual(@as(usize, 24), YUNMU_TABLE.len);
}

test "yunmu_a_vector" {
    const v = getYunmu(0);
    try std.testing.expectEqual(@as(f32, 0), v.vx);
    try std.testing.expectEqual(@as(f32, 1), v.vy);
}
