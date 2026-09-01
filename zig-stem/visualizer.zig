// 可视化器 (Visualizer)
// 功能：将协调器输出转换为三维坐标和HTML可视化
// 核心：三维坐标生成 + 颜色映射 + HTML输出

const std = @import("std");
const orchestrator = @import("orchestrator.zig");
const indexer = @import("indexer.zig");
const locator = @import("locator.zig");
const phaser = @import("phaser.zig");
const angler = @import("angler.zig");
const frequencer = @import("frequencer.zig");

// ============================================================
// 1. 三维坐标点 (3D Point)
// ============================================================
pub const Point3D = struct {
    x: f64,
    y: f64,
    z: f64,
    color: [3]u8,        // RGB颜色
    label: []const u8,
    size: f64,          // 点的大小

    pub fn init(x: f64, y: f64, z: f64, color: [3]u8, label: []const u8, size: f64) Point3D {
        return Point3D{
            .x = x,
            .y = y,
            .z = z,
            .color = color,
            .label = label,
            .size = size,
        };
    }
};

// ============================================================
// 2. 三维连接线 (3D Line)
// ============================================================
pub const Line3D = struct {
    start: Point3D,
    end: Point3D,
    color: [3]u8,
    width: f64,

    pub fn init(start: Point3D, end: Point3D, color: [3]u8, width: f64) Line3D {
        return Line3D{
            .start = start,
            .end = end,
            .color = color,
            .width = width,
        };
    }
};

// ============================================================
// 3. 可视化场景 (Visualization Scene)
// ============================================================
pub const Scene = struct {
    allocator: std.mem.Allocator,
    points: std.ArrayList(Point3D),
    lines: std.ArrayList(Line3D),

    pub fn init(allocator: std.mem.Allocator) Scene {
        return Scene{
            .allocator = allocator,
            .points = std.ArrayList(Point3D).init(allocator),
            .lines = std.ArrayList(Line3D).init(allocator),
        };
    }

    pub fn deinit(self: *Scene) void {
        for (self.points.items) |point| {
            self.allocator.free(point.label);
        }
        self.points.deinit();
        self.lines.deinit();
    }

    // 添加点
    pub fn addPoint(self: *Scene, point: Point3D) !void {
        try self.points.append(point);
    }

    // 添加线
    pub fn addLine(self: *Scene, line: Line3D) !void {
        try self.lines.append(line);
    }

    // 从协调器生成三维场景
    pub fn generateFromOrchestrator(self: *Scene, orch: *const orchestrator.Orchestrator) !void {
        // 为每个容器生成一个区域
        const stats = orch.getContainerStats();

        var index: u32 = 0;
        for (stats) |container| {
            // 根据容器ID分配区域（不同高度）
            const base_y = @as(f64, @floatFromInt(index)) * 2.0;

            // 根据净收益设置颜色
            const net = container.netBenefit();
            const color = if (net > 0) [3]u8{ 0, 255, 0 }      // 绿色：正收益
                         else if (net < 0) [3]u8{ 255, 0, 0 }  // 红色：负收益
                         else [3]u8{ 128, 128, 128 };          // 灰色：中性

            // 根据项数设置点的大小
            const size = @as(f64, @floatFromInt(container.item_count)) * 0.5 + 0.5;

            // 添加容器中心点
            const center_point = Point3D{
                .x = 0,
                .y = base_y,
                .z = 0,
                .color = color,
                .label = try self.allocator.dupe(u8, container.name),
                .size = size,
            };
            try self.addPoint(center_point);

            // 为正关系添加绿色点
            if (container.positive_count > 0) {
                const pos_point = Point3D{
                    .x = 1.0,
                    .y = base_y + 0.5,
                    .z = 0,
                    .color = [3]u8{ 0, 200, 0 },
                    .label = try self.allocator.dupe(u8, "positive"),
                    .size = @as(f64, @floatFromInt(container.positive_count)) * 0.3,
                };
                try self.addPoint(pos_point);

                try self.addLine(Line3D.init(center_point, pos_point, [3]u8{ 0, 150, 0 }, 1.0));
            }

            // 为负关系添加红色点
            if (container.negative_count > 0) {
                const neg_point = Point3D{
                    .x = -1.0,
                    .y = base_y + 0.5,
                    .z = 0,
                    .color = [3]u8{ 200, 0, 0 },
                    .label = try self.allocator.dupe(u8, "negative"),
                    .size = @as(f64, @floatFromInt(container.negative_count)) * 0.3,
                };
                try self.addPoint(neg_point);

                try self.addLine(Line3D.init(center_point, neg_point, [3]u8{ 150, 0, 0 }, 1.0));
            }

            index += 1;
        }

        // 为定位器的位置添加点
        for (orch.locator.positions.items) |pos| {
            const point = Point3D{
                .x = pos.coordinate.x,
                .y = pos.coordinate.y + 5.0,
                .z = pos.coordinate.z,
                .color = [3]u8{ 0, 100, 255 },
                .label = try self.allocator.dupe(u8, pos.name),
                .size = 0.5,
            };
            try self.addPoint(point);
        }
    }

    // 生成HTML输出
    pub fn generateHTML(self: *const Scene, allocator: std.mem.Allocator) ![]const u8 {
        var output = std.ArrayList(u8).init(allocator);
        defer output.deinit();

        // HTML头部
        try output.appendSlice(
            \\<!DOCTYPE html>
            \\<html>
            \\<head>
            \\<meta charset="UTF-8">
            \\<title>囡囝共生 - 三维可视化</title>
            \\<style>
            \\body { margin: 0; font-family: monospace; background: #1a1a2e; color: #eee; }
            \\#container { display: flex; }
            \\#canvas { flex: 1; height: 100vh; }
            \\#sidebar { width: 300px; padding: 20px; background: #16213e; }
            \\h1 { color: #e94560; font-size: 1.2em; }
            \\.stat { margin: 10px 0; padding: 10px; background: #0f3460; border-radius: 4px; }
            \\.positive { color: #4ecca3; }
            \\.negative { color: #e94560; }
            \\</style>
            \\</head>
            \\<body>
            \\<div id="container">
            \\<canvas id="canvas"></canvas>
            \\<div id="sidebar">
            \\<h1>系统状态</h1>
        );

        // 统计信息
        const stat_line = try std.fmt.allocPrint(allocator, "<div class='stat'>总点数: {d}<br>总线数: {d}</div>", .{ self.points.items.len, self.lines.items.len });
        try output.appendSlice(stat_line);
        allocator.free(stat_line);

        // 点列表
        try output.appendSlice("<h2>坐标点</h2>");
        for (self.points.items, 0..) |point, i| {
            const r = point.color[0];
            const g = point.color[1];
            const b = point.color[2];
            const color_hex = try std.fmt.allocPrint(allocator, "#{x:0>2}{x:0>2}{x:0>2}", .{r, g, b});
            const line = try std.fmt.allocPrint(allocator, "<div class='stat' style='border-left: 3px solid {s};'>{d}. {s} ({d:.1}, {d:.1}, {d:.1})</div>", .{
                color_hex, i + 1, point.label, point.x, point.y, point.z,
            });
            try output.appendSlice(line);
            allocator.free(color_hex);
            allocator.free(line);
        }

        // HTML脚部和JavaScript
        try output.appendSlice(
            \\</div>
            \\</div>
            \\<script>
            \\const canvas = document.getElementById('canvas');
            \\const ctx = canvas.getContext('2d');
            \\canvas.width = canvas.offsetWidth;
            \\canvas.height = canvas.offsetHeight;
            \\
            \\const points = [
        );

        // 输出点数据为JavaScript
        for (self.points.items, 0..) |point, i| {
            if (i > 0) try output.appendSlice(",\n");
            const js_point = try std.fmt.allocPrint(allocator, "  {{x: {d:.2}, y: {d:.2}, z: {d:.2}, r: {d}, g: {d}, b: {d}, label: '{s}', size: {d:.1}}}", .{
                point.x, point.y, point.z,
                point.color[0], point.color[1], point.color[2],
                point.label, point.size,
            });
            try output.appendSlice(js_point);
            allocator.free(js_point);
        }

        try output.appendSlice(
            \\];
            \\
            \\function project(x, y, z) {
            \\  const scale = 50;
            \\  const cx = canvas.width / 2;
            \\  const cy = canvas.height / 2;
            \\  const perspective = 300;
            \\  const factor = perspective / (perspective + z * scale);
            \\  return {
            \\    x: cx + x * scale * factor,
            \\    y: cy - y * scale * factor,
            \\    scale: factor
            \\  };
            \\}
            \\
            \\function render() {
            \\  ctx.fillStyle = '#1a1a2e';
            \\  ctx.fillRect(0, 0, canvas.width, canvas.height);
            \\
            \\  // 绘制网格
            \\  ctx.strokeStyle = '#333';
            \\  ctx.lineWidth = 0.5;
            \\  for (let i = -5; i <= 5; i++) {
            \\    const p1 = project(i, 0, -5);
            \\    const p2 = project(i, 0, 5);
            \\    const p3 = project(-5, 0, i);
            \\    const p4 = project(5, 0, i);
            \\    ctx.beginPath();
            \\    ctx.moveTo(p1.x, p1.y);
            \\    ctx.lineTo(p2.x, p2.y);
            \\    ctx.moveTo(p3.x, p3.y);
            \\    ctx.lineTo(p4.x, p4.y);
            \\    ctx.stroke();
            \\  }
            \\
            \\  // 绘制点
            \\  for (const p of points) {
            \\    const proj = project(p.x, p.y, p.z);
            \\    const size = Math.max(2, p.size * 8 * proj.scale);
            \\    ctx.fillStyle = `rgb(${p.r}, ${p.g}, ${p.b})`;
            \\    ctx.beginPath();
            \\    ctx.arc(proj.x, proj.y, size, 0, Math.PI * 2);
            \\    ctx.fill();
            \\    ctx.fillStyle = '#fff';
            \\    ctx.font = '10px monospace';
            \\    ctx.fillText(p.label, proj.x + size + 2, proj.y + 3);
            \\  }
            \\}
            \\
            \\let angle = 0;
            \\function animate() {
            \\  angle += 0.005;
            \\  for (const p of points) {
            \\    const cos = Math.cos(angle);
            \\    const sin = Math.sin(angle);
            \\    const nx = p.x * cos - p.z * sin;
            \\    const nz = p.x * sin + p.z * cos;
            \\    p.x = nx;
            \\    p.z = nz;
            \\  }
            \\  render();
            \\  requestAnimationFrame(animate);
            \\}
            \\
            \\animate();
            \\</script>
            \\</body>
            \\</html>
        );

        return output.toOwnedSlice();
    }
};

// ============================================================
// 测试
// ============================================================
test "visualizer basic" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var scene = Scene.init(allocator);
    defer scene.deinit();

    // 添加测试点
    const point1 = Point3D.init(0, 0, 0, .{ 255, 0, 0 }, "origin", 1.0);
    const point2 = Point3D.init(1, 1, 0, .{ 0, 255, 0 }, "point", 0.5);

    try scene.addPoint(point1);
    try scene.addPoint(point2);
    try scene.addLine(Line3D.init(point1, point2, .{ 128, 128, 128 }, 1.0));

    try std.testing.expectEqual(@as(usize, 2), scene.points.items.len);
    try std.testing.expectEqual(@as(usize, 1), scene.lines.items.len);
}
