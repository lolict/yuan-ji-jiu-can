// ============================================================
// 力场演化引擎 · 加法统摄 · 收敛均衡
// ============================================================

const std = @import("std");
const glyph_mod = @import("glyph.zig");
const yunmu_mod = @import("yunmu.zig");

pub const SyllableData = struct {
    shengmu_idx: u8,
    yunmu_idx: u8,
    tone: u8,
    force: f32,
    energy: f32,
    vx: f32,
    vy: f32,
    operation: []const u8,
};

pub const FieldResult = struct {
    syllables: []SyllableData,
    iterations: u32,
    converged: bool,
    final_force: f32,
};

// ┌─────────────────────────────────────────────────────────┐
// │ 解析单个音节 (shengmu + yunmu + tone)                  │
// └─────────────────────────────────────────────────────────┘
pub fn parseSyllable(sm: u8, ym_name: []const u8, tone: u8) ?SyllableData {
    // 声母验证
    const glyph_info = glyph_mod.lookupGlyph(sm) orelse return null;
    _ = glyph_info;

    // 声母索引 (Zig 0.14+ 需要手动追踪索引)
    var sm_idx: u8 = 0;
    var i: u8 = 0;
    inline for (glyph_mod.glyph_registry) |gr| {
        if (gr.symbol == sm) {
            sm_idx = i;
        }
        i += 1;
    }

    // 韵母查找
    const ym_idx = yunmu_mod.findYunmu(ym_name) orelse 0;
    const ymv = yunmu_mod.getYunmu(ym_idx);

    // 计算力
    const gr = glyph_mod.glyph_registry[sm_idx];
    const force = @as(f32, @floatFromInt(glyph_mod.calculateForce(gr))) * yunmu_mod.toneEnergy(tone);

    return SyllableData{
        .shengmu_idx = sm_idx,
        .yunmu_idx = @intCast(ym_idx),
        .tone = tone,
        .force = force,
        .energy = yunmu_mod.toneEnergy(tone),
        .vx = ymv.vx,
        .vy = ymv.vy,
        .operation = gr.operation,
    };
}

// ┌─────────────────────────────────────────────────────────┐
// │ 力场演化: 邻居平均化 → 加法统摄收敛                    │
// └─────────────────────────────────────────────────────────┘
pub fn evolve(allocator: std.mem.Allocator, syllables: []const SyllableData, max_iter: u32) !FieldResult {
    const n = syllables.len;
    if (n == 0) return FieldResult{ .syllables = &.{}, .iterations = 0, .converged = true, .final_force = 0 };

    // 复制到可变数组
    var forces = try allocator.alloc(f32, n);
    defer allocator.free(forces);
    var energies = try allocator.alloc(f32, n);
    defer allocator.free(energies);

    var total_force: f32 = 0;
    for (syllables, 0..) |s, i| {
        forces[i] = s.force;
        energies[i] = s.energy;
        total_force += s.force;
    }

    const initial_force = total_force;
    var converged = false;
    var iter: u32 = 0;

    var new_forces = try allocator.alloc(f32, n);
    defer allocator.free(new_forces);

    while (iter < max_iter) : (iter += 1) {
        // 邻居平均化
        for (forces, 0..) |f, i| {
            var sum = f;
            var count: f32 = 1;
            if (i > 0) {
                sum += forces[i - 1];
                count += 1;
            }
            if (i < n - 1) {
                sum += forces[i + 1];
                count += 1;
            }
            new_forces[i] = sum / count;
        }

        // 写回
        var new_total: f32 = 0;
        for (forces, 0..) |_, i| {
            forces[i] = new_forces[i];
            new_total += forces[i];
        }

        // 收敛判断: 总力变化 < 0.01%
        const delta = if (new_total > initial_force) new_total - initial_force else initial_force - new_total;
        if (delta / initial_force < 0.0001) {
            converged = true;
            break;
        }
    }

    // 构建结果
    var result_syl = try allocator.alloc(SyllableData, n);
    for (syllables, 0..) |s, i| {
        result_syl[i] = s;
        result_syl[i].force = forces[i];
    }

    var final_total: f32 = 0;
    for (forces) |f| final_total += f;

    return FieldResult{
        .syllables = result_syl,
        .iterations = iter,
        .converged = converged,
        .final_force = final_total,
    };
}

// ┌─────────────────────────────────────────────────────────┐
// │ 单元测试                                               │
// └─────────────────────────────────────────────────────────┘
test "parse_syllable_ma" {
    const s = parseSyllable('m', "a", 1).?;
    try std.testing.expect(s.force > 0);
    try std.testing.expectEqual(@as(u8, 1), s.tone);
}

test "parse_syllable_ba" {
    const s = parseSyllable('b', "a", 1).?;
    try std.testing.expectEqual(@as(f32, 14.0), s.force);
}

test "parse_syllable_xi" {
    const s = parseSyllable('x', "i", 1).?;
    try std.testing.expect(s.force > 0);
}

test "parse_syllable_null" {
    try std.testing.expect(parseSyllable('Z', "a", 1) == null);
}

test "evolve_single" {
    const ally = std.testing.allocator;
    const s = parseSyllable('m', "a", 1).?;
    const input = &[_]SyllableData{s};
    const result = try evolve(ally, input, 100);
    defer ally.free(result.syllables);
    try std.testing.expect(result.converged);
    try std.testing.expect(result.final_force > 0);
}
