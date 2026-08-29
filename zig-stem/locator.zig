// 定位器 (Locator)
// 功能：确定空间/关系位置
// 核心：坐标定位 + 关系定位 + 位置计算 + 距离度量

const std = @import("std");
const mapper = @import("mapper.zig");
const aligner = @import("aligner.zig");
const indexer = @import("indexer.zig");
const stepper = @import("stepper.zig");

// ============================================================
// 1. 坐标 (Coordinate)
// ============================================================
pub const Coordinate = struct {
    x: f64,
    y: f64,
    z: f64,

    pub fn init(x: f64, y: f64, z: f64) Coordinate {
        return Coordinate{ .x = x, .y = y, .z = z };
    }

    pub fn distanceTo(self: *const Coordinate, other: *const Coordinate) f64 {
        const dx = self.x - other.x;
        const dy = self.y - other.y;
        const dz = self.z - other.z;
        return std.math.sqrt(dx * dx + dy * dy + dz * dz);
    }

    pub fn midpoint(self: *const Coordinate, other: *const Coordinate) Coordinate {
        return Coordinate{
            .x = (self.x + other.x) / 2,
            .y = (self.y + other.y) / 2,
            .z = (self.z + other.z) / 2,
        };
    }
};

// ============================================================
// 2. 关系位置 (Relation Position)
// ============================================================
pub const RelationPosition = struct {
    source_id: u32,
    target_id: u32,
    relation_type: RelationType,
    weight: f64,            // 关系权重（正=吸引，负=排斥）
    distance: f64,          // 关系距离

    pub fn init(source_id: u32, target_id: u32, relation_type: RelationType, weight: f64, distance: f64) RelationPosition {
        return RelationPosition{
            .source_id = source_id,
            .target_id = target_id,
            .relation_type = relation_type,
            .weight = weight,
            .distance = distance,
        };
    }
};

pub const RelationType = enum {
    positive,   // 正关系：吸引、合作
    negative,   // 负关系：排斥、对抗
    neutral,    // 中性关系：无倾向
    causal,     // 因果关系：因果链
};

// ============================================================
// 3. 定位器核心 (Locator Core)
// ============================================================
pub const Locator = struct {
    allocator: std.mem.Allocator,
    positions: std.ArrayList(PositionEntry),
    relations: std.ArrayList(RelationPosition),
    reference_point: *Coordinate,

    pub fn init(allocator: std.mem.Allocator, reference_point: *Coordinate) Locator {
        return Locator{
            .allocator = allocator,
            .positions = std.ArrayList(PositionEntry).init(allocator),
            .relations = std.ArrayList(RelationPosition).init(allocator),
            .reference_point = reference_point,
        };
    }

    pub fn deinit(self: *Locator) void {
        self.positions.deinit();
        self.relations.deinit();
    }

    // 添加位置
    pub fn addPosition(self: *Locator, entry: PositionEntry) !void {
        try self.positions.append(entry);
    }

    // 添加关系
    pub fn addRelation(self: *Locator, relation: RelationPosition) !void {
        try self.relations.append(relation);
    }

    // 获取指定ID的位置
    pub fn getPosition(self: *const Locator, id: u32) ?PositionEntry {
        for (self.positions.items) |entry| {
            if (entry.id == id) return entry;
        }
        return null;
    }

    // 计算到参考点的距离
    pub fn distanceToReference(self: *const Locator, id: u32) f64 {
        const entry = self.getPosition(id) orelse return std.math.inf(f64);
        return entry.coordinate.distanceTo(self.reference_point);
    }

    // 计算两个位置之间的距离
    pub fn distanceBetween(self: *const Locator, id_a: u32, id_b: u32) f64 {
        const entry_a = self.getPosition(id_a) orelse return std.math.inf(f64);
        const entry_b = self.getPosition(id_b) orelse return std.math.inf(f64);
        return entry_a.coordinate.distanceTo(&entry_b.coordinate);
    }

    // 查找最近的位置
    pub fn findNearest(self: *const Locator, id: u32) ?u32 {
        const entry = self.getPosition(id) orelse return null;
        var min_dist = std.math.inf(f64);
        var nearest_id: ?u32 = null;

        for (self.positions.items) |other| {
            if (other.id == id) continue;
            const dist = entry.coordinate.distanceTo(&other.coordinate);
            if (dist < min_dist) {
                min_dist = dist;
                nearest_id = other.id;
            }
        }

        return nearest_id;
    }

    // 查找指定范围内的位置
    pub fn findInRange(self: *const Locator, id: u32, max_distance: f64) std.ArrayList(u32) {
        var result = std.ArrayList(u32).init(self.allocator);
        const entry = self.getPosition(id) orelse return result;

        for (self.positions.items) |other| {
            if (other.id == id) continue;
            const dist = entry.coordinate.distanceTo(&other.coordinate);
            if (dist <= max_distance) {
                _ = result.append(other.id) catch {};
            }
        }

        return result;
    }

    // 计算关系权重总和
    pub fn getTotalRelationWeight(self: *const Locator, id: u32) f64 {
        var total: f64 = 0;
        for (self.relations.items) |relation| {
            if (relation.source_id == id or relation.target_id == id) {
                total += relation.weight;
            }
        }
        return total;
    }

    // 获取正关系数量
    pub fn getPositiveRelationCount(self: *const Locator, id: u32) u32 {
        var count: u32 = 0;
        for (self.relations.items) |relation| {
            if ((relation.source_id == id or relation.target_id == id) and relation.relation_type == .positive) {
                count += 1;
            }
        }
        return count;
    }

    // 获取负关系数量
    pub fn getNegativeRelationCount(self: *const Locator, id: u32) u32 {
        var count: u32 = 0;
        for (self.relations.items) |relation| {
            if ((relation.source_id == id or relation.target_id == id) and relation.relation_type == .negative) {
                count += 1;
            }
        }
        return count;
    }

    // 计算平衡度（正关系 - 负关系）
    pub fn getBalance(self: *const Locator, id: u32) f64 {
        const positive = self.getPositiveRelationCount(id);
        const negative = self.getNegativeRelationCount(id);
        return @as(f64, @floatFromInt(positive)) - @as(f64, @floatFromInt(negative));
    }
};

// ============================================================
// 4. 位置条目 (Position Entry)
// ============================================================
pub const PositionEntry = struct {
    id: u32,
    name: []const u8,
    coordinate: Coordinate,
    container_id: u32,

    pub fn init(id: u32, name: []const u8, coordinate: Coordinate, container_id: u32) PositionEntry {
        return PositionEntry{
            .id = id,
            .name = name,
            .coordinate = coordinate,
            .container_id = container_id,
        };
    }
};

// ============================================================
// 5. 空间定位器 (Spatial Locator)
// ============================================================
pub const SpatialLocator = struct {
    locator: *Locator,

    pub fn init(locator: *Locator) SpatialLocator {
        return SpatialLocator{
            .locator = locator,
        };
    }

    // 计算重心（所有位置的平均）
    pub fn calculateCentroid(self: *const SpatialLocator) Coordinate {
        var sum_x: f64 = 0;
        var sum_y: f64 = 0;
        var sum_z: f64 = 0;
        const count = self.locator.positions.items.len;

        if (count == 0) return Coordinate.init(0, 0, 0);

        for (self.locator.positions.items) |entry| {
            sum_x += entry.coordinate.x;
            sum_y += entry.coordinate.y;
            sum_z += entry.coordinate.z;
        }

        return Coordinate.init(sum_x / @as(f64, @floatFromInt(count)), sum_y / @as(f64, @floatFromInt(count)), sum_z / @as(f64, @floatFromInt(count)));
    }

    // 计算离散度（位置分布的分散程度）
    pub fn calculateDispersion(self: *const SpatialLocator) f64 {
        const centroid = self.calculateCentroid();
        var total_dist: f64 = 0;
        const count = self.locator.positions.items.len;

        if (count == 0) return 0;

        for (self.locator.positions.items) |entry| {
            total_dist += entry.coordinate.distanceTo(&centroid);
        }

        return total_dist / @as(f64, @floatFromInt(count));
    }
};

// ============================================================
// 6. 关系定位器 (Relation Locator)
// ============================================================
pub const RelationLocator = struct {
    locator: *Locator,

    pub fn init(locator: *Locator) RelationLocator {
        return RelationLocator{
            .locator = locator,
        };
    }

    // 查找关系链（从source到target的路径）
    pub fn findRelationChain(self: *const RelationLocator, source_id: u32, target_id: u32, max_depth: u32) std.ArrayList(u32) {
        var path = std.ArrayList(u32).init(self.locator.allocator);
        var visited = std.ArrayList(u32).init(self.locator.allocator);
        defer visited.deinit();

        _ = self.findPath(source_id, target_id, max_depth, &path, &visited);
        return path;
    }

    fn findPath(self: *const RelationLocator, current: u32, target: u32, depth_remaining: u32, path: *std.ArrayList(u32), visited: *std.ArrayList(u32)) bool {
        if (depth_remaining == 0) return false;

        _ = path.append(current) catch {};
        _ = visited.append(current) catch {};

        if (current == target) return true;

        for (self.locator.relations.items) |relation| {
            var next_id: ?u32 = null;
            if (relation.source_id == current) {
                next_id = relation.target_id;
            } else if (relation.target_id == current) {
                next_id = relation.source_id;
            }

            if (next_id) |nid| {
                // 检查是否已访问
                var already_visited = false;
                for (visited.items) |v| {
                    if (v == nid) {
                        already_visited = true;
                        break;
                    }
                }

                if (!already_visited) {
                    if (self.findPath(nid, target, depth_remaining - 1, path, visited)) {
                        return true;
                    }
                }
            }
        }

        // 回溯
        if (path.items.len > 0) {
            path.items.len -= 1;
        }

        return false;
    }
};

// ============================================================
// 测试
// ============================================================
test "locator basic" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var ref = Coordinate.init(0, 0, 0);
    var locator = Locator.init(allocator, &ref);
    defer locator.deinit();

    // 添加位置
    try locator.addPosition(PositionEntry.init(1, "point_a", Coordinate.init(3, 4, 0), 1));
    try locator.addPosition(PositionEntry.init(2, "point_b", Coordinate.init(6, 8, 0), 1));

    // 计算距离
    const dist = locator.distanceBetween(1, 2);
    try std.testing.expectApproxEqAbs(@as(f64, 5.0), dist, 0.001);

    // 到参考点距离
    const dist_ref = locator.distanceToReference(1);
    try std.testing.expectApproxEqAbs(@as(f64, 5.0), dist_ref, 0.001);
}

test "locator nearest" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var ref = Coordinate.init(0, 0, 0);
    var locator = Locator.init(allocator, &ref);
    defer locator.deinit();

    try locator.addPosition(PositionEntry.init(1, "origin", Coordinate.init(0, 0, 0), 1));
    try locator.addPosition(PositionEntry.init(2, "near", Coordinate.init(1, 0, 0), 1));
    try locator.addPosition(PositionEntry.init(3, "far", Coordinate.init(10, 0, 0), 1));

    const nearest = locator.findNearest(1);
    try std.testing.expectEqual(@as(u32, 2), nearest.?);
}

test "locator relations" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var ref = Coordinate.init(0, 0, 0);
    var locator = Locator.init(allocator, &ref);
    defer locator.deinit();

    try locator.addPosition(PositionEntry.init(1, "a", Coordinate.init(0, 0, 0), 1));
    try locator.addPosition(PositionEntry.init(2, "b", Coordinate.init(1, 0, 0), 1));
    try locator.addPosition(PositionEntry.init(3, "c", Coordinate.init(2, 0, 0), 1));

    try locator.addRelation(RelationPosition.init(1, 2, .positive, 1.0, 1.0));
    try locator.addRelation(RelationPosition.init(2, 3, .negative, -1.0, 1.0));

    const balance = locator.getBalance(2);
    try std.testing.expectEqual(@as(f64, 0.0), balance);
}

test "spatial locator" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var ref = Coordinate.init(0, 0, 0);
    var locator = Locator.init(allocator, &ref);
    defer locator.deinit();

    try locator.addPosition(PositionEntry.init(1, "a", Coordinate.init(0, 0, 0), 1));
    try locator.addPosition(PositionEntry.init(2, "b", Coordinate.init(2, 0, 0), 1));
    try locator.addPosition(PositionEntry.init(3, "c", Coordinate.init(4, 0, 0), 1));

    var spatial = SpatialLocator.init(&locator);
    const centroid = spatial.calculateCentroid();

    try std.testing.expectApproxEqAbs(@as(f64, 2.0), centroid.x, 0.001);
}
