// 角度器 (Angler)
// 功能：计算方向差异
// 核心：角度计算 + 方向判定 + 角度差 + 角度同步

const std = @import("std");
const mapper = @import("mapper.zig");
const aligner = @import("aligner.zig");
const indexer = @import("indexer.zig");
const stepper = @import("stepper.zig");
const locator = @import("locator.zig");
const phaser = @import("phaser.zig");

// ============================================================
// 1. 角度 (Angle)
// ============================================================
pub const Angle = struct {
    degrees: f64,           // 角度（0-360）

    pub fn init(degrees: f64) Angle{
        return Angle{ .degrees = @mod(degrees, 360.0) };
    }

    pub fn fromRadians(radians: f64) Angle {
        return Angle.init(radians * 180.0 / std.math.pi);
    }

    pub fn toRadians(self: *const Angle) f64 {
        return self.degrees * std.math.pi / 180.0;
    }

    // 计算角度差（最短路径）
    pub fn difference(self: *const Angle, other: *const Angle) f64 {
        var diff = @abs(self.degrees - other.degrees);
        if (diff > 180.0) {
            diff = 360.0 - diff;
        }
        return diff;
    }

    // 角度是否在容差范围内
    pub fn isWithinTolerance(self: *const Angle, other: *const Angle, tolerance: f64) bool {
        return self.difference(other) <= tolerance;
    }
};

// ============================================================
// 2. 方向 (Direction)
// ============================================================
pub const Direction = struct {
    angle: Angle,
    name: []const u8,

    pub fn init(degrees: f64, name: []const u8) Direction {
        return Direction{
            .angle = Angle.init(degrees),
            .name = name,
        };
    }
};

// ============================================================
// 3. 角度器核心 (Angler Core)
// ============================================================
pub const Angler = struct {
    allocator: std.mem.Allocator,
    directions: std.ArrayList(Direction),
    tolerance: f64,

    pub fn init(allocator: std.mem.Allocator, tolerance: f64) Angler {
        return Angler{
            .allocator = allocator,
            .directions = std.ArrayList(Direction).init(allocator),
            .tolerance = tolerance,
        };
    }

    pub fn deinit(self: *Angler) void {
        self.directions.deinit();
    }

    // 添加方向
    pub fn addDirection(self: *Angler, direction: Direction) !void {
        try self.directions.append(direction);
    }

    // 计算两个方向之间的角度差
    pub fn calculateAngleDiff(self: *const Angler, index_a: u32, index_b: u32) f64 {
        if (index_a >= self.directions.items.len or index_b >= self.directions.items.len) {
            return 0;
        }
        return self.directions.items[index_a].angle.difference(&self.directions.items[index_b].angle);
    }

    // 检查两个方向是否对齐
    pub fn areAligned(self: *const Angler, index_a: u32, index_b: u32) bool {
        const diff = self.calculateAngleDiff(index_a, index_b);
        return diff <= self.tolerance;
    }

    // 查找最接近的方向
    pub fn findClosestDirection(self: *const Angler, target: *const Angle) ?u32 {
        if (self.directions.items.len == 0) return null;

        var min_diff = std.math.inf(f64);
        var closest_index: ?u32 = null;

        for (self.directions.items, 0..) |direction, i| {
            const diff = direction.angle.difference(target);
            if (diff < min_diff) {
                min_diff = diff;
                closest_index = @intCast(i);
            }
        }

        return closest_index;
    }

    // 计算平均方向
    pub fn calculateAverageDirection(self: *const Angler) Angle {
        if (self.directions.items.len == 0) return Angle.init(0);

        var sum_x: f64 = 0;
        var sum_y: f64 = 0;

        for (self.directions.items) |direction| {
            const rad = direction.angle.toRadians();
            sum_x += std.math.cos(rad);
            sum_y += std.math.sin(rad);
        }

        const avg_rad = std.math.atan2(sum_y, sum_x);
        return Angle.fromRadians(avg_rad);
    }

    // 计算方向离散度
    pub fn calculateDispersion(self: *const Angler) f64 {
        if (self.directions.items.len == 0) return 0;

        const avg = self.calculateAverageDirection();
        var total_diff: f64 = 0;

        for (self.directions.items) |direction| {
            total_diff += direction.angle.difference(&avg);
        }

        return total_diff / @as(f64, @floatFromInt(self.directions.items.len));
    }

    // 获取指定索引的角度
    pub fn getAngle(self: *const Angler, index: u32) Angle {
        if (index >= self.directions.items.len) return Angle.init(0);
        return self.directions.items[index].angle;
    }

    // 设置指定索引的角度
    pub fn setAngle(self: *Angler, index: u32, angle: Angle) void {
        if (index >= self.directions.items.len) return;
        self.directions.items[index].angle = angle;
    }
};

// ============================================================
// 4. 角度同步器 (Angle Synchronizer)
// ============================================================
pub const AngleSynchronizer = struct {
    angler: *Angler,
    target_angle: Angle,

    pub fn init(angler: *Angler, target_angle: Angle) AngleSynchronizer {
        return AngleSynchronizer{
            .angler = angler,
            .target_angle = target_angle,
        };
    }

    // 同步所有方向到目标角度
    pub fn synchronizeAll(self: *AngleSynchronizer) void {
        for (0..self.angler.directions.items.len) |i| {
            self.angler.directions.items[i].angle = self.target_angle;
        }
    }

    // 渐进同步（按速率逐步靠近）
    pub fn gradualSynchronize(self: *AngleSynchronizer, rate: f64) u32 {
        var synced_count: u32 = 0;

        for (0..self.angler.directions.items.len) |i| {
            var current = &self.angler.directions.items[i].angle;
            const diff = self.target_angle.difference(current);

            if (diff <= self.angler.tolerance) {
                synced_count += 1;
                continue;
            }

            // 按速率调整
            const adjustment = diff * rate;
            const new_degrees = current.degrees + adjustment;
            current.degrees = @mod(new_degrees, 360.0);

            if (current.difference(&self.target_angle) <= self.angler.tolerance) {
                synced_count += 1;
            }
        }

        return synced_count;
    }
};

// ============================================================
// 5. 方向分类器 (Direction Classifier)
// ============================================================
pub const DirectionClassifier = struct {
    angler: *Angler,

    pub fn init(angler: *Angler) DirectionClassifier {
        return DirectionClassifier{
            .angler = angler,
        };
    }

    // 按角度范围分类
    pub fn classifyByRange(self: *const DirectionClassifier, index: u32) []const u8 {
        if (index >= self.angler.directions.items.len) return "unknown";

        const angle = self.angler.directions.items[index].angle.degrees;

        if (angle < 45 or angle >= 315) return "east";
        if (angle < 135) return "south";
        if (angle < 225) return "west";
        return "north";
    }

    // 按八方位分类
    pub fn classifyBy8Directions(self: *const DirectionClassifier, index: u32) []const u8 {
        if (index >= self.angler.directions.items.len) return "unknown";

        const angle = self.angler.directions.items[index].angle.degrees;

        if (angle < 22.5 or angle >= 337.5) return "E";
        if (angle < 67.5) return "SE";
        if (angle < 112.5) return "S";
        if (angle < 157.5) return "SW";
        if (angle < 202.5) return "W";
        if (angle < 247.5) return "NW";
        if (angle < 292.5) return "N";
        return "NE";
    }
};

// ============================================================
// 测试
// ============================================================
test "angler basic" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var angler = Angler.init(allocator, 5.0);
    defer angler.deinit();

    try angler.addDirection(Direction.init(0, "east"));
    try angler.addDirection(Direction.init(90, "south"));
    try angler.addDirection(Direction.init(180, "west"));

    // 计算角度差
    const diff_01 = angler.calculateAngleDiff(0, 1);
    try std.testing.expectApproxEqAbs(@as(f64, 90.0), diff_01, 0.001);

    const diff_02 = angler.calculateAngleDiff(0, 2);
    try std.testing.expectApproxEqAbs(@as(f64, 180.0), diff_02, 0.001);
}

test "angler cyclic difference" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var angler = Angler.init(allocator, 5.0);
    defer angler.deinit();

    try angler.addDirection(Direction.init(350, "NNW"));
    try angler.addDirection(Direction.init(10, "NNE"));

    // 350度和10度的差应该是20度（循环最短路径）
    const diff = angler.calculateAngleDiff(0, 1);
    try std.testing.expectApproxEqAbs(@as(f64, 20.0), diff, 0.001);
}

test "angler closest direction" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var angler = Angler.init(allocator, 5.0);
    defer angler.deinit();

    try angler.addDirection(Direction.init(0, "east"));
    try angler.addDirection(Direction.init(120, "SE"));
    try angler.addDirection(Direction.init(240, "NW"));

    // 查找最接近90度的方向
    const target = Angle.init(90);
    const closest = angler.findClosestDirection(&target);
    try std.testing.expectEqual(@as(u32, 1), closest.?);
}

test "angle synchronizer" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var angler = Angler.init(allocator, 5.0);
    defer angler.deinit();

    try angler.addDirection(Direction.init(0, "east"));
    try angler.addDirection(Direction.init(120, "SE"));
    try angler.addDirection(Direction.init(240, "NW"));

    const target = Angle.init(90);
    var sync = AngleSynchronizer.init(&angler, target);
    sync.synchronizeAll();

    // 所有方向应该都同步到90度
    for (0..3) |i| {
        try std.testing.expect(angler.areAligned(@intCast(i), 0));
    }
}
