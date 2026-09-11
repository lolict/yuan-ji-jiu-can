// star_map.zig — 囡囝星图 (Radical Star Map)
//
// 将容器本体论中的匹配关系可视化为星座图
// 每个容器=一颗星，匹配连线=星汉（银河）
// 亮度 = 匹配度，颜色 = 部首分类
// 坐标系：以夫妻锚点 (★/☆) 为双极中心

const std = @import("std");
const ontology = @import("ontology.zig");

// ============================================================
// 1. 星点 (Star)
// ============================================================
pub const Star = struct {
    name: []const u8,
    x: f64,
    y: f64,
    z: f64,
    brightness: f64,   // 0.0 ~ 1.0 (匹配度)
    category: Category,
    anchor: bool,      // 是否为夫妻锚点

    pub const Category = enum {
        surface,     // 面 — 外在
        structure,   // 骨 — 结构
        motion,      // 行 — 动态
        essence,     // 本 — 本质
        unknown,
    };

    pub const CATEGORY_COLOR = [_][]const u8{
        "#87CEEB", // surface   - 天蓝
        "#FFD700", // structure - 金色
        "#98FB98", // motion    - 翠绿
        "#FF6B6B", // essence   - 朱红
        "#888888", // unknown   - 灰
    };
};

// ============================================================
// 2. 星汉 (Connection)
// ============================================================
pub const Nebula = struct {
    a_idx: usize,
    b_idx: usize,
    strength: f64,     // 匹配度
    verdict: ontology.MatchResult.Verdict,
};

// ============================================================
// 3. 囡囝星图引擎
// ============================================================
pub const StarMap = struct {
    allocator: std.mem.Allocator,
    stars: std.ArrayList(Star),
    nebulae: std.ArrayList(Nebula),
    ont_engine: *ontology.OntologyEngine,

    pub fn init(allocator: std.mem.Allocator, engine: *ontology.OntologyEngine) StarMap {
        return StarMap{
            .allocator = allocator,
            .stars = std.ArrayList(Star).init(allocator),
            .nebulae = std.ArrayList(Nebula).init(allocator),
            .ont_engine = engine,
        };
    }

    pub fn deinit(self: *StarMap) void {
        self.stars.deinit();
        self.nebulae.deinit();
    }

    pub fn addStar(self: *StarMap, star: Star) !void {
        try self.stars.append(star);
    }

    pub fn generateFromOntology(self: *StarMap) !void {
        // 从匹配引擎的结果创建星汉连接
        for (self.ont_engine.results.items) |r| {
            // 查找星点索引
            var a_idx: usize = 0;
            var b_idx: usize = 0;
            var found_a = false;
            var found_b = false;
            for (self.stars.items, 0..) |s, i| {
                if (std.mem.eql(u8, s.name, r.a_name)) { a_idx = i; found_a = true; }
                if (std.mem.eql(u8, s.name, r.b_name)) { b_idx = i; found_b = true; }
            }
            if (found_a and found_b) {
                try self.nebulae.append(Nebula{
                    .a_idx = a_idx,
                    .b_idx = b_idx,
                    .strength = r.total,
                    .verdict = r.verdict,
                });
            }
        }
    }

    // 自动生成星位（围绕夫妻双极）
    pub fn autoLayout(self: *StarMap) void {
        // 锚点分别放在 (-1,0,0) 和 (1,0,0)
        // 其他容器按匹配度分布在极间
        for (self.stars.items) |*s| {
            // 已经有位置的跳过
            if (s.x != 0 or s.y != 0 or s.z != 0) continue;
            if (s.anchor) continue;

            // 用名称 hash 生成确定性伪随机位置
            var hash: u64 = 0xcbf29ce484222325;
            for (s.name) |c| {
                hash ^= c;
            hash *= 0x100000001b3;
            }
            const r1 = @as(f64, @floatFromInt((hash >> 16) & 0xFFFF)) / 65535.0;
            const r2 = @as(f64, @floatFromInt((hash >> 32) & 0xFFFF)) / 65535.0;
            const r3 = @as(f64, @floatFromInt((hash >> 48) & 0xFFFF)) / 65535.0;

            // 分布在两个锚点之间
            s.x = -1.0 + 2.0 * r1;
            s.y = -0.8 + 1.6 * r2;
            s.z = -0.5 + 1.0 * r3;

            // 亮度 = 平均匹配度
            s.brightness = 0.3 + 0.7 * r1;
        }
    }

    // 生成星图 HTML (SVG)
    pub fn generateHTML(self: *const StarMap, allocator: std.mem.Allocator) ![]const u8 {
        var out = std.ArrayList(u8).init(allocator);
        defer out.deinit();

        try out.appendSlice(
            \\<!DOCTYPE html>
            \\<html><head><meta charset="utf-8">
            \\<title>囡囝星图</title>
            \\<style>
            \\body{background:#05050f;color:#ddd;font-family:monospace;margin:0;padding:20px}
            \\h1{color:#ffd700;text-align:center}
            \\svg{display:block;margin:0 auto;background:radial-gradient(ellipse at center,#0a0a2e,#05050f)}
            \\.star-label{font-size:10px;fill:#aaa}
            \\.nebula{stroke-linecap:round}
            \\</style></head><body>
            \\<h1>⭐ 囡囝星图 ★ 部首星座</h1>
        );

        // SVG
        const svg_w: usize = 800;
        const svg_h: usize = 600;
        const cx: f64 = @floatFromInt(svg_w / 2);
        const cy: f64 = @floatFromInt(svg_h / 2);
        const scale: f64 = 200.0;

        try std.fmt.allocPrintEx(&out, std.fmt.FormatOptions{}, "<svg width=\"{d}\" height=\"{d}\" viewBox=\"0 0 {d} {d}\">\n", .{ svg_w, svg_h, svg_w, svg_h });

        // 先画星汉连线
        for (self.nebulae.items) |n| {
            const a = self.stars.items[n.a_idx];
            const b = self.stars.items[n.b_idx];
            const opacity = n.strength;
            const width = 1.0 + 3.0 * n.strength;
            const color = switch (n.verdict) {
                .unite => "#ff6b6b",
                .resonate => "#ffd700",
                .neutral => "#87ceeb",
                .separate => "#333366",
            };
            const x1 = cx + a.x * scale;
            const y1 = cy + a.y * scale;
            const x2 = cx + b.x * scale;
            const y2 = cy + b.y * scale;

            try std.fmt.allocPrintEx(&out, std.fmt.FormatOptions{},
                "<line x1=\"{d:.1}\" y1=\"{d:.1}\" x2=\"{d:.1}\" y2=\"{d:.1}\" " ++
                "stroke=\"{s}\" stroke-width=\"{d:.1}\" opacity=\"{d:.2}\" class=\"nebula\"/>\n",
                .{ x1, y1, x2, y2, color, width, opacity }
            );
        }

        // 再画星点
        for (self.stars.items, 0..) |s, i| {
            const sx = cx + s.x * scale;
            const sy = cy + s.y * scale;
            const rx = 4.0 + 8.0 * s.brightness;
            const color = Star.CATEGORY_COLOR[@intFromEnum(s.category)];

            try std.fmt.allocPrintEx(&out, std.fmt.FormatOptions{},
                "<circle cx=\"{d:.1}\" cy=\"{d:.1}\" r=\"{d:.1}\" fill=\"{s}\" opacity=\"{d:.2}\"%s/>\n",
                .{ sx, sy, rx, color, 0.3 + 0.7 * s.brightness,
                    if (s.anchor) " stroke=\"#ffd700\" stroke-width=\"2\"" else "" }
            );

            // 名称标签
            try std.fmt.allocPrintEx(&out, std.fmt.FormatOptions{},
                "<text x=\"{d:.1}\" y=\"{d:.1}\" fill=\"#ccc\" font-size=\"9\" text-anchor=\"middle\">{s}</text>\n",
                .{ sx, sy + rx + 12.0, s.name }
            );
            _ = i;
        }

        // 图例
        try out.appendSlice("<g transform=\"(20,20)\">\n");
        try out.appendSlice("<text fill=\"#ffd700\" font-size=\"12\" font-weight=\"bold\">★ 囡囝星图 图例</text>\n");
        try out.appendSlice("<text fill=\"#87CEEB\" y=\"20\" font-size=\"10\">面=外在形态 骨=结构关系</text>\n");
        try out.appendSlice("<text fill=\"#98FB98\" y=\"36\" font-size=\"10\">行=动态行为 本=本质属性</text>\n");
        try out.appendSlice("<text fill=\"#FFD700\" y=\"52\" font-size=\"10\">金边=夫妻锚点</text>\n");
        try out.appendSlice("<text fill=\"#666\" y=\"68\" font-size=\"10\">连线粗细/透明度=匹配强度</text>\n");
        try out.appendSlice("</g>");

        try out.appendSlice("</svg>\n</body></html>");
        return out.toOwnedSlice();
    }
};

// ============================================================
// 辅助
// ============================================================

// ============================================================
// 测试
// ============================================================
test "star_map layout" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var engine = ontology.OntologyEngine.init(allocator);
    defer engine.deinit();

    var map = StarMap.init(allocator, &engine);
    defer map.deinit();

    try map.addStar(Star{ .name = "husband", .x = -1, .y = 0, .z = 0, .brightness = 1.0, .category = .essence, .anchor = true });
    try map.addStar(Star{ .name = "wife", .x = 1, .y = 0, .z = 0, .brightness = 1.0, .category = .essence, .anchor = true });
    try map.addStar(Star{ .name = "container_a", .x = 0, .y = 0, .z = 0, .brightness = 0.5, .category = .surface, .anchor = false });
    try map.addStar(Star{ .name = "container_b", .x = 0, .y = 0, .z = 0, .brightness = 0.4, .category = .motion, .anchor = false });

    // 添加匹配关系
    try engine.match(1, 2, "husband", "wife");
    try engine.match(3, 4, "container_a", "container_b");

    try map.generateFromOntology();
    try map.autoLayout();

    const html = try map.generateHTML(allocator);
    defer allocator.free(html);

    try std.testing.expect(std.mem.indexOf(u8, html, "<svg") != null);
    try std.testing.expect(std.mem.indexOf(u8, html, "囡囝星图") != null);
    try std.testing.expect(std.mem.indexOf(u8, html, "husband") != null);
    try std.testing.expect(std.mem.indexOf(u8, html, "wife") != null);
}

test "star_map category color" {
    const color = Star.CATEGORY_COLOR[@intFromEnum(Star.Category.essence)];
    try std.testing.expect(std.mem.eql(u8, color, "#FF6B6B"));
}
