// browser.zig — 统计文档聚合浏览器 (Statistical Document Aggregation Browser)
//
// 功能：
// ├─ 态势化聚类视图 — 容器/位置/关系按聚类展示
// ├─ 连接包裹树 — 层级嵌套结构
// ├─ 裁决汇总 — 满全法裁判结果统计
// ├─ 八流热度图 — 各流 merit 可视化
// ├─ 蚕蛹壳状态 — 协议翻译层状态
// └─ 本体匹配矩阵 — 聶小聶匹配度热力图

const std = @import("std");

// ============================================================
// 1. 浏览器结构
// ============================================================
pub const AggBrowser = struct {
    allocator: std.mem.Allocator,
    sections: std.ArrayList(Section),

    pub const Section = struct {
        title: []const u8,
        content: []const u8,
        subsection_level: u8,  // 0=top, 1=section, 2=subsection
    };

    pub fn init(allocator: std.mem.Allocator) AggBrowser {
        return AggBrowser{
            .allocator = allocator,
            .sections = std.ArrayList(Section).init(allocator),
        };
    }

    pub fn deinit(self: *AggBrowser) void {
        // sections 中的字符串由外部管理
        self.sections.deinit();
    }

    pub fn addSection(self: *AggBrowser, title: []const u8, content: []const u8, level: u8) !void {
        try self.sections.append(Section{
            .title = title,
            .content = content,
            .subsection_level = level,
        });
    }

    pub fn generateHTML(self: *const AggBrowser) ![]const u8 {
        var out = std.ArrayList(u8).init(self.allocator);
        defer out.deinit();

        try out.appendSlice(
            \\<!DOCTYPE html>
            \\<html lang="zh">
            \\<head>
            \\<meta charset="utf-8">
            \\<title>囡囝共生 · 聚合浏览器</title>
            \\<style>
            \\body{font-family:monospace;max-width:960px;margin:0 auto;padding:24px;background:#0a0a14;color:#e0e0f0}
            \\h1{color:#ffd700;border-bottom:2px solid #ffd700;padding-bottom:8px}
            \\h2{color:#87ceeb;margin-top:24px}
            \\h3;color:#98fb98}
            \\.card{background:#151520;border:1px solid #333;border-radius:8px;padding:16px;margin:12px 0}
            \\.metric{display:inline-block;background:#1a1a2e;border:1px solid #444;border-radius:4px;padding:4px 12px;margin:4px}
            \\.accept{color:#4caf50}.reject{color:#f44336}.warn{color:#ff9800}
            \\table{border-collapse:collapse;width:100%;margin:8px 0}
            \\th,td{border:1px solid #333;padding:6px 12px;text-align:left}
            \\th{background:#1a1a2e;color:#ffd700}
            \\tr:nth-child(even){background:#12121c}
            \\.bar{display:inline-block;height:12px;background:linear-gradient(90deg,#ffd700,#ff8c00);border-radius:2px}
            \\pre{background:#0d0d18;padding:12px;border-radius:6px;overflow-x:auto;font-size:11px}
            \\</style>
            \\</head>
            \\<body>
            \\<h1>🌀 囡囝共生 聚合浏览器</h1>
            \\<p>满全法操作系统 · 统计文档聚合视图</p>
        );

        for (self.sections.items) |sec| {
            if (sec.subsection_level == 0) {
                try out.appendSlice("<h1>");
            } else if (sec.subsection_level == 1) {
                try out.appendSlice("<h2>");
            } else {
                try out.appendSlice("<h3>");
            }
            try out.appendSlice(sec.title);
            if (sec.subsection_level == 0) {
                try out.appendSlice("</h1>");
            } else if (sec.subsection_level == 1) {
                try out.appendSlice("</h2>");
            } else {
                try out.appendSlice("</h3>");
            }

            try out.appendSlice("<div class=\"card\">");
            try out.appendSlice(sec.content);
            try out.appendSlice("</div>");
        }

        try out.appendSlice(
            \\<hr>
            \\<p style="text-align:center;color:#666">满全法操作系统 · NJSG · fqmcp</p>
            \\</body>
            \\</html>
        );

        return out.toOwnedSlice();
    }
};

// ============================================================
// 2. HTML 模板辅助
// ============================================================
pub fn htmlMetric(allocator: std.mem.Allocator, label: []const u8, value: []const u8) ![]const u8 {
    return std.fmt.allocPrint(allocator, "<span class=\"metric\">{s}: {s}</span>", .{ label, value });
}

pub fn htmlTableHeader(allocator: std.mem.Allocator, headers: []const []const u8) ![]const u8 {
    var out = std.ArrayList(u8).init(allocator);
    defer out.deinit();
    try out.appendSlice("<table><tr>");
    for (headers) |h| {
        try out.appendSlice("<th>");
        try out.appendSlice(h);
        try out.appendSlice("</th>");
    }
    try out.appendSlice("</tr>");
    return out.toOwnedSlice();
}

pub fn bar(allocator: std.mem.Allocator, fraction: f64) ![]const u8 {
    const width: usize = @intFromFloat(@max(0.0, @min(1.0, fraction)) * 100.0);
    const pct: usize = @intFromFloat(@round(fraction * 100.0));
    return std.fmt.allocPrint(allocator, "<span class=\"bar\" style=\"width:{d}%\"></span> {d}%", .{ width, pct });
}

// ============================================================
// 测试
// ============================================================
test "browser HTML generation" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var browser = AggBrowser.init(allocator);
    defer browser.deinit();

    const content = "测试内容 <b>容器数:</b> ★★★";
    try browser.addSection("测试标题", content, 1);

    const html = try browser.generateHTML();
    defer allocator.free(html);

    try std.testing.expect(std.mem.indexOf(u8, html, "<!DOCTYPE html>") != null);
    try std.testing.expect(std.mem.indexOf(u8, html, "囡囝共生") != null);
    try std.testing.expect(std.mem.indexOf(u8, html, "测试标题") != null);
}

test "browser metric formatting" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const metric = try htmlMetric(allocator, "容器", "★★★");
    defer allocator.free(metric);

    try std.testing.expect(std.mem.eql(u8, metric, "<span class=\"metric\">容器: ★★★</span>"));
}

test "browser bar visualization" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const b = try bar(allocator, 0.75);
    defer allocator.free(b);

    try std.testing.expect(std.mem.indexOf(u8, b, "75%") != null);
}
