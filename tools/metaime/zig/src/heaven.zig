// ============================================================
// 天庭空间 · 八流统一 · 意识路由
//
// 天庭 = 意识流的统一运行时空间
// 八流 = 八种彼此独立又统一的意识流：
//   1. 预加载流 — 编译期预取/加载/固化
//   2. 解释流   — 运行时解析/语义展开
//   3. 谐音路由 — 声母韵母同源匹配
//   4. 主权路由 — 极性匹配 → 归属权不变量
//   5. 容器流   — 装入/取出/遍历/裁定
//   6. 力场流   — 力/能量/方向的几何演化
//   7. 反馈流   — 结果返回/目标差/再路由
//   8. 寄生流   — 蚕蛹反向化壳/外部模仿
//
// 统一规则：所有流皆为向量（方向×极性×力）
// 兼容性 = zone_used 一致（容器几何裁判）
// ============================================================

const std = @import("std");
const glyph_mod = @import("glyph.zig");
const yunmu_mod = @import("yunmu.zig");
const executor_mod = @import("executor.zig");

// ── 八流枚举 ──
pub const FlowKind = enum(u8) {
    preload    = 0,   // 预加载流 — comptime 固化
    interpret  = 1,   // 解释流 — 运行时语义展开
    homophone  = 2,   // 谐音路由 — 声母韵母同源
    sovereignty = 3,  // 主权路由 — 极性匹配
    container  = 4,   // 容器流 — 装入/取出/审理
    forcefield = 5,   // 力场流 — 几何演化
    feedback   = 6,   // 反馈流 — 目标差返回
    parasitic  = 7,   // 寄生流 — 蚕蛹反向化壳

    pub fn name(self: FlowKind) []const u8 {
        return switch (self) {
            .preload    => "预加载流",
            .interpret  => "解释流",
            .homophone  => "谐音路由",
            .sovereignty => "主权路由",
            .container  => "容器流",
            .forcefield => "力场流",
            .feedback   => "反馈流",
            .parasitic  => "寄生流",
        };
    }
};

// ── 流向量 ──
pub const FlowVector = struct {
    kind: FlowKind,
    direction: FlowDirection,
    polarity: Polarity,
    force: f32,
    energy: f32,
    active: bool,
    container_zone: glyph_mod.Zone,
};

pub const FlowDirection = enum {
    inward,     // 内收 (容器装载)
    outward,    // 外发 (容器取出)
    traverse,   // 穿透 (遍历)
    reflect,    // 反射 (裁定)
    merge,      // 合并 (二流归一)
    split,      // 分裂 (一流化为二)
    judge,      // 裁判 (裁定准入)
    loopback,   // 回环 (反馈路由)
};

pub const Polarity = enum {
    positive,   // 正 (+) = 充实/赋予
    negative,   // 负 (−) = 接收/取用
    neutral,    // 零 = 待裁定
    dual,       // 双 = 正负共存 (螺旋)
};

// ── 流代数运算 ──
pub const FlowAlgebra = struct {};

/// 两流加法统摄：同 zone 才能相加
pub fn add(a: FlowVector, b: FlowVector) ?FlowVector {
    if (a.container_zone != b.container_zone) return null; // 不可耦合
    return FlowVector{
        .kind = a.kind, // 继承左操作数的主体类型
        .direction = combineDirection(a.direction, b.direction),
        .polarity = combinePolarity(a.polarity, b.polarity),
        .force = a.force + b.force, // 加法统摄：不做乘法
        .energy = if (a.energy > b.energy) a.energy else b.energy,
        .active = a.active and b.active,
        .container_zone = a.container_zone,
    };
}

/// 流分裂（一流化二）
pub fn split(f: FlowVector) struct { FlowVector, FlowVector } {
    const half = f.force / 2.0;
    const a = FlowVector{
        .kind = f.kind,
        .direction = f.direction,
        .polarity = f.polarity,
        .force = half,
        .energy = f.energy * 0.7,
        .active = f.active,
        .container_zone = f.container_zone,
    };
    const b = FlowVector{
        .kind = f.kind,
        .direction = oppositeDirection(f.direction),
        .polarity = oppositePolarity(f.polarity),
        .force = half,
        .energy = f.energy * 0.3,
        .active = f.active,
        .container_zone = f.container_zone,
    };
    return .{ a, b };
}

/// 流裁定（三元裁判）
pub fn judge(a: FlowVector, b: FlowVector, witness: FlowVector) ?FlowVector {
    // 三元见证：witness 必须与其中一方同 zone
    const compatible_a = (witness.container_zone == a.container_zone);
    const compatible_b = (witness.container_zone == b.container_zone);

    if (!compatible_a and !compatible_b) return null;

    // 返回与 witness 兼容的那一方
    if (compatible_a) return a;
    return b;
}

// ── 辅助函数 ──
fn combineDirection(a: FlowDirection, b: FlowDirection) FlowDirection {
    if (a == b) return a;
    // 合并法则: 不同方向 → merge
    return .merge;
}

fn combinePolarity(a: Polarity, b: Polarity) Polarity {
    const ua = @intFromEnum(a);
    const ub = @intFromEnum(b);
    if (ua == 0 and ub == 0) return .positive;
    if (ua == 2 and ub == 2) return .negative;
    if (ua == 1 and ub == 1) return .neutral;
    return .dual;
}

fn oppositeDirection(d: FlowDirection) FlowDirection {
    return switch (d) {
        .inward => .outward,
        .outward => .inward,
        .traverse => .reflect,
        .reflect => .traverse,
        .merge => .split,
        .split => .merge,
        .judge => .loopback,
        .loopback => .judge,
    };
}

fn oppositePolarity(p: Polarity) Polarity {
    return switch (p) {
        .positive => .negative,
        .negative => .positive,
        .neutral => .neutral,
        .dual => .dual,
    };
}

// ── 天庭空间 ──
pub const HeavenSpace = struct {
    flows: [8]FlowVector,
    count: u8,

    pub fn init() HeavenSpace {
        var h: HeavenSpace = .{
            .flows = undefined,
            .count = 0,
        };

        // 初始化八流（预编译=已知结构）
        inline for (std.enums.values(FlowKind)) |fk| {
            const idx = @intFromEnum(fk);
            const dir: FlowDirection = if (idx < 4) .inward else .outward;
            const pol: Polarity = switch (idx % 4) {
                0 => .positive,
                1 => .negative,
                2 => .neutral,
                else => .dual,
            };
            h.flows[idx] = FlowVector{
                .kind = fk,
                .direction = dir,
                .polarity = pol,
                .force = 1.0,
                .energy = yunmu_mod.toneEnergy(if (idx < 4) @as(u8, idx + 1) else 4),
                .active = (idx < 8),
                .container_zone = .middle,
            };
            h.count += 1;
        }

        return h;
    }

    pub fn get(self: HeavenSpace, kind: FlowKind) FlowVector {
        return self.flows[@intFromEnum(kind)];
    }

    pub fn route(self: HeavenSpace, input: FlowVector) ?FlowVector {
        // 路由：找到同 zone 的流中力最强的
        var best: ?FlowVector = null;
        for (self.flows[0..self.count]) |f| {
            if (f.active and f.container_zone == input.container_zone and f.kind != input.kind) {
                if (best == null or f.force > best.?.force) {
                    best = f;
                }
            }
        }
        return best;
    }

    pub fn display(self: HeavenSpace) void {
        std.debug.print("\n══════════════════════════════════════\n", .{});
        std.debug.print(" 天庭空间 · 八流统一\n", .{});
        std.debug.print("══════════════════════════════════════\n", .{});
        for (self.flows) |f| {
            const status = if (f.active) "●" else "○";
            std.debug.print("  {s} 力={d:.1} 能量={d:.2} 方向={s} \n", .{
                status, f.force, f.energy, @tagName(f.direction),
            });
        }
        std.debug.print("══════════════════════════════════════\n", .{});
    }
};

// ── 单元测试 ──
test "flow_kind_name" {
    try std.testing.expectEqualStrings("解释流", FlowKind.interpret.name());
}

test "flow_add_compatible" {
    const a = FlowVector{
        .kind = .forcefield, .container_zone = .middle,
        .force = 10, .energy = 1, .active = true,
        .direction = .inward, .polarity = .positive,
    };
    const b = FlowVector{
        .kind = .container, .container_zone = .middle,
        .force = 20, .energy = 1, .active = true,
        .direction = .outward, .polarity = .negative,
    };
    const result = add(a, b).?;
    try std.testing.expectEqual(@as(f32, 30.0), result.force);
}

test "flow_add_incompatible" {
    const a = FlowVector{
        .kind = .forcefield, .container_zone = .middle,
        .force = 10, .energy = 1, .active = true,
        .direction = .inward, .polarity = .positive,
    };
    const b = FlowVector{
        .kind = .container, .container_zone = .junction,
        .force = 20, .energy = 1, .active = true,
        .direction = .outward, .polarity = .negative,
    };
    try std.testing.expect(add(a, b) == null);
}

test "flow_split" {
    const f = FlowVector{
        .kind = .container, .container_zone = .middle,
        .force = 10, .energy = 1, .active = true,
        .direction = .inward, .polarity = .positive,
    };
    const result = split(f);
    try std.testing.expectEqual(@as(f32, 5.0), result[0].force);
    try std.testing.expectEqual(@as(f32, 5.0), result[1].force);
}

test "heaven_init" {
    const h = HeavenSpace.init();
    try std.testing.expectEqual(@as(u8, 8), h.count);
}

test "heaven_route" {
    const h = HeavenSpace.init();
    const input = FlowVector{
        .kind = .interpret, .container_zone = .middle,
        .force = 10, .energy = 1, .active = true,
        .direction = .inward, .polarity = .positive,
    };
    const target = h.route(input).?;
    try std.testing.expect(target.container_zone == .middle);
}

test "flow_polarity_opposite" {
    try std.testing.expectEqual(Polarity.negative, oppositePolarity(.positive));
    try std.testing.expectEqual(Polarity.positive, oppositePolarity(.negative));
}
