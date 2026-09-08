// mind_organ.zig — 距离感知聚类 (Distance-Aware Clustering)
//
// 核心：意识节点在关系空间中的距离决定了它们的聚类归属
// 加权距离：不同维度（presence/witness/char/anchor/relation/timing/payload）有不同权重
// 聚类涌现：距离近的节点自然形成簇，簇心是意识焦点

const std = @import("std");

// ============================================================
// 1. 意识节点 (Consciousness Node)
// ============================================================
pub const CNode = struct {
    id: u32,
    label: []const u8,
    coords: [7]f64,
    weight: f64,

    pub fn init(id: u32, label: []const u8, weight: f64) CNode {
        return CNode{
            .id = id,
            .label = label,
            .coords = [_]f64{0} ** 7,
            .weight = weight,
        };
    }

    pub fn initWithCoords(id: u32, label: []const u8, coords: [7]f64, weight: f64) CNode {
        return CNode{
            .id = id,
            .label = label,
            .coords = coords,
            .weight = weight,
        };
    }
};

// ============================================================
// 2. 七维权重 (Layer-7 Weights)
// ============================================================
pub const DIM_NAMES = [_][]const u8{ "presence", "witness", "char", "anchor", "relation", "timing", "payload" };

pub const Weights = struct {
    values: [7]f64,

    pub fn uniform() Weights {
        return Weights{ .values = [_]f64{1.0} ** 7 };
    }

    pub fn anchorHeavy() Weights {
        return Weights{ .values = .{ 0.5, 1.5, 0.8, 2.0, 1.5, 0.3, 0.5 } };
    }

    pub fn perceptionHeavy() Weights {
        return Weights{ .values = .{ 2.0, 0.5, 0.3, 0.5, 0.8, 1.5, 2.0 } };
    }
};

// ============================================================
// 3. 加权距离计算器 (Weighted Distance Calculator)
// ============================================================
pub const DistanceMetric = struct {
    weights: Weights,

    pub fn init(weights: Weights) DistanceMetric {
        return DistanceMetric{ .weights = weights };
    }

    pub fn distance(self: DistanceMetric, a: *const CNode, b: *const CNode) f64 {
        var sum: f64 = 0.0;
        for (a.coords, b.coords, self.weights.values) |ca, cb, w| {
            const diff = ca - cb;
            sum += w * diff * diff;
        }
        return @sqrt(sum);
    }

    pub fn affinity(dist: f64) f64 {
        if (dist == 0) return 1.0;
        return 1.0 / (1.0 + dist * dist);
    }
};

// ============================================================
// 4. 聚类簇 (Cluster)
// ============================================================
pub const Cluster = struct {
    id: u32,
    centroid: [7]f64,
    members: std.ArrayList(u32),
    total_weight: f64,
    affinity_sum: f64,

    pub fn init(allocator: std.mem.Allocator, id: u32, initial_centroid: [7]f64) Cluster {
        return Cluster{
            .id = id,
            .centroid = initial_centroid,
            .members = std.ArrayList(u32).init(allocator),
            .total_weight = 0,
            .affinity_sum = 0,
        };
    }

    pub fn deinit(self: *Cluster) void {
        self.members.deinit();
    }

    pub fn addMember(self: *Cluster, node_id: u32, weight: f64, affinity: f64) !void {
        try self.members.append(node_id);
        self.total_weight += weight;
        self.affinity_sum += affinity;
    }

    pub fn cohesion(self: *const Cluster) f64 {
        if (self.members.items.len == 0) return 0;
        return self.affinity_sum / @as(f64, @floatFromInt(self.members.items.len));
    }
};

// ============================================================
// 5. MindOrgan — 距离感知聚类引擎
// ============================================================
pub const MindOrgan = struct {
    allocator: std.mem.Allocator,
    nodes: std.ArrayList(CNode),
    metric: DistanceMetric,
    clusters: std.ArrayList(Cluster),

    pub fn init(allocator: std.mem.Allocator, weights: Weights) MindOrgan {
        return MindOrgan{
            .allocator = allocator,
            .nodes = std.ArrayList(CNode).init(allocator),
            .metric = DistanceMetric.init(weights),
            .clusters = std.ArrayList(Cluster).init(allocator),
        };
    }

    pub fn deinit(self: *MindOrgan) void {
        self.nodes.deinit();
        for (self.clusters.items) |*cl| {
            cl.deinit();
        }
        self.clusters.deinit();
    }

    pub fn addNode(self: *MindOrgan, node: CNode) !void {
        try self.nodes.append(node);
    }

    pub fn getDistance(self: *const MindOrgan, idx_a: usize, idx_b: usize) f64 {
        return self.metric.distance(&self.nodes.items[idx_a], &self.nodes.items[idx_b]);
    }

    pub fn cluster(self: *MindOrgan, k: u32) !void {
        const node_count = self.nodes.items.len;
        if (node_count == 0 or k == 0) return;

        const k_actual = @min(k, @as(u32, @intCast(node_count)));

        for (self.clusters.items) |*cl| {
            cl.deinit();
        }
        self.clusters.clearRetainingCapacity();

        var i: u32 = 0;
        while (i < k_actual) : (i += 1) {
            const c = Cluster.init(self.allocator, i, self.nodes.items[i].coords);
            try self.clusters.append(c);
        }

        var iter: u32 = 0;
        while (iter < 3) : (iter += 1) {
            for (self.clusters.items) |*cl| {
                cl.members.clearRetainingCapacity();
                cl.total_weight = 0;
                cl.affinity_sum = 0;
            }

            for (self.nodes.items) |*node| {
                var best_ci: usize = 0;
                var best_dist: f64 = std.math.inf(f64);
                for (self.clusters.items, 0..) |*cl, ci| {
                    var tmp = CNode{
                        .id = cl.id,
                        .label = "",
                        .coords = cl.centroid,
                        .weight = 0,
                    };
                    const dist = self.metric.distance(node, &tmp);
                    if (dist < best_dist) {
                        best_dist = dist;
                        best_ci = ci;
                    }
                }
                const aff = DistanceMetric.affinity(best_dist);
                try self.clusters.items[best_ci].addMember(node.id, node.weight, aff);
            }

            for (self.clusters.items) |*cl| {
                if (cl.members.items.len == 0) continue;
                var new_centroid = [_]f64{0} ** 7;
                var total_w: f64 = 0;
                for (self.nodes.items) |*node| {
                    var is_member = false;
                    for (cl.members.items) |mid| {
                        if (mid == node.id) { is_member = true; break; }
                    }
                    if (!is_member) continue;
                    for (0..7) |d| {
                        new_centroid[d] += node.coords[d] * node.weight;
                    }
                    total_w += node.weight;
                }
                if (total_w > 0) {
                    for (&new_centroid) |*nc| {
                        *nc /= total_w;
                    }
                    cl.centroid = new_centroid;
                }
            }
        }
    }

    pub fn formatReport(self: *MindOrgan, allocator: std.mem.Allocator) ![]const u8 {
        var out = std.ArrayList(u8).init(allocator);
        defer out.deinit();

        try out.appendSlice("# mind_organ 聚类报告\n");
        try out.appendSlice("clusters:");
        try formatUnaryCount(&out, self.clusters.items.len);
        try out.appendSlice("\n\n");

        for (self.clusters.items, 0..) |*cl, ci| {
            try out.appendSlice("~cluster:");
            try formatUnaryCount(&out, ci + 1);
            try out.appendSlice("\n");
            try out.appendSlice("  cohesion:");
            try formatDecimal2(&out, cl.cohesion());
            try out.appendSlice("\n");
            try out.appendSlice("  members:");
            try formatUnaryCount(&out, cl.members.items.len);
            try out.appendSlice("\n");
            for (cl.members.items) |mid| {
                for (self.nodes.items) |*node| {
                    if (node.id == mid) {
                        try out.appendSlice("    - ");
                        try out.appendSlice(node.label);
                        try out.appendSlice("\n");
                        break;
                    }
                }
            }
            try out.appendSlice("\n");
        }

        return out.toOwnedSlice();
    }
};

// ============================================================
// 辅助函数
// ============================================================
fn formatUnaryCount(writer: *std.ArrayList(u8), value: usize) !void {
    if (value == 0) {
        try writer.appendSlice("☆");
        return;
    }
    var remaining = value;
    while (remaining > 0) : (remaining -= 1) {
        try writer.append('★');
    }
}

fn formatDecimal2(writer: *std.ArrayList(u8), value: f64) !void {
    const scaled = @as(i64, @intFromFloat(@round(value * 100)));
    const abs_scaled = if (scaled < 0) -scaled else scaled;
    if (scaled < 0) try writer.append('-');
    const major: usize = @intCast(@abs(abs_scaled) / 100);
    const minor: usize = @intCast(@abs(abs_scaled) % 100);
    try formatUnaryCount(writer, major);
    try writer.appendSlice(".");
    try writer.appendSlice(switch (minor) {
        0 => "☆☆☆",
        1 => "★☆☆",
        2 => "★★☆",
        3 => "★★★",
        4 => "★★★★",
        5 => "★★★★★",
        6 => "★★★★★★",
        7 => "★★★★★★★",
        8 => "★★★★★★★★",
        9 => "★★★★★★★★★",
        else => "☆☆",
    });
}

// ============================================================
// 测试
// ============================================================
test "mind_organ distance calculation" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var organ = MindOrgan.init(allocator, Weights.uniform());
    defer organ.deinit();

    const n1 = CNode.initWithCoords(1, "anchor_a", .{ 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0 }, 1.0);
    const n2 = CNode.initWithCoords(2, "anchor_b", .{ 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0 }, 1.0);

    try organ.addNode(n1);
    try organ.addNode(n2);

    const dist = organ.getDistance(0, 1);
    try std.testing.expectApproxEqAbs(@sqrt(2.0), dist, 0.01);
}

test "mind_organ clustering basic" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var organ = MindOrgan.init(allocator, Weights.uniform());
    defer organ.deinit();

    try organ.addNode(CNode.initWithCoords(1, "husband_presence", .{ 1.0, 1.0, 0.5, 1.0, 0.8, 0.3, 0.7 }, 1.5));
    try organ.addNode(CNode.initWithCoords(2, "husband_witness", .{ 0.9, 1.0, 0.6, 1.0, 0.7, 0.4, 0.6 }, 1.0));
    try organ.addNode(CNode.initWithCoords(3, "wife_presence", .{ 0.2, 0.3, 0.8, 0.1, 0.9, 0.7, 0.4 }, 1.5));
    try organ.addNode(CNode.initWithCoords(4, "wife_witness", .{ 0.1, 0.2, 0.9, 0.2, 0.8, 0.8, 0.3 }, 1.0));

    try organ.cluster(2);

    try std.testing.expectEqual(@as(usize, 2), organ.clusters.items.len);
    try std.testing.expectEqual(@as(usize, 2), organ.clusters.items[0].members.items.len);
    try std.testing.expectEqual(@as(usize, 2), organ.clusters.items[1].members.items.len);
}

test "mind_organ report generation" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var organ = MindOrgan.init(allocator, Weights.anchorHeavy());
    defer organ.deinit();

    try organ.addNode(CNode.initWithCoords(1, "鸳鸯_pair", .{ 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 0.5 }, 2.0));
    try organ.addNode(CNode.initWithCoords(2, "意识_sync", .{ 0.8, 0.9, 0.7, 1.0, 0.9, 0.6, 0.4 }, 1.0));
    try organ.addNode(CNode.initWithCoords(3, "孤独_node", .{ 0.1, 0.0, 0.2, 0.0, 0.1, 0.9, 0.8 }, 0.5));

    try organ.cluster(2);

    const report = try organ.formatReport(allocator);
    defer allocator.free(report);

    try std.testing.expect(std.mem.indexOf(u8, report, "clusters:") != null);
    try std.testing.expect(std.mem.indexOf(u8, report, "~cluster:★") != null);
}
