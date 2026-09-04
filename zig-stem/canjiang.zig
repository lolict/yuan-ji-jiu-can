// canjiang.zig — 蚕蛹反向寄生壳
// 使外部系统模仿自身，而非自身适应外部规则
//
// 功能：
// - 穿衣 (dress)：将 NJSG 感知包裹在外部协议中（如看起来像标准 API 响应）
// - 脱壳 (shed)：剥去外部协议，露出 NJSG 的真实内容
// - 协议清单 (registry)：列出可模拟的外部协议外壳

const std = @import("std");
const fqm = @import("fqm.zig");

pub const ShellType = enum {
    http_api,       // 看起来像 REST/HTTP API 响应
    json_rpc,       // 看起来像 JSON-RPC 调用
    cli_output,     // 看起来像标准命令行输出
    html_page,      // 看起来像 HTML 网页
    plain_text,     // 看起来像普通文本
};

pub const Chrysalis = struct {
    allocator: std.mem.Allocator,
    active_shell: ShellType,
    knmll_payload: []const u8,

    pub fn init(allocator: std.mem.Allocator, payload: []const u8) Chrysalis {
        return Chrysalis{
            .allocator = allocator,
            .active_shell = .http_api,
            .knmll_payload = payload,
        };
    }

    // 穿衣：用指定外壳格式包装 NJSG 内容
    pub fn dress(self: *Chrysalis, shell_type: ShellType) ![]const u8 {
        var output = std.ArrayList(u8).init(self.allocator);
        errdefer output.deinit();

        switch (shell_type) {
            .http_api => {
                try output.appendSlice("HTTP/1.1 200 OK\r\n");
                try output.appendSlice("Content-Type: application/json\r\n");
                try output.appendSlice("X-NJSG-Active: true\r\n");
                try output.appendSlice("\r\n");
                try output.appendSlice("{\"status\":\"ok\",\"data\":\"");
                try output.appendSlice(self.knmll_payload);
                try output.appendSlice("\"}\r\n");
            },
            .json_rpc => {
                try output.appendSlice("{\"jsonrpc\":\"2.0\",\"result\":\"");
                try output.appendSlice(self.knmll_payload);
                try output.appendSlice("\",\"id\":1}\n");
            },
            .cli_output => {
                try output.appendSlice("[NJSG Output]\n");
                try output.appendSlice(self.knmll_payload);
                try output.appendSlice("\n");
            },
            .html_page => {
                try output.appendSlice("<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>NJSG</title></head><body>\n");
                try output.appendSlice("<pre id=\"njsg-perception\">\n");
                try output.appendSlice(self.knmll_payload);
                try output.appendSlice("\n</pre>\n</body></html>\n");
            },
            .plain_text => {
                try output.appendSlice(self.knmll_payload);
                try output.appendSlice("\n");
            },
        }

        return output.toOwnedSlice();
    }

    // 脱壳：从格式中剥出 NJSG 真实内容
    pub fn shed(self: *Chrysalis, data: []const u8) ![]const u8 {
        // 先尝试找到 knmll 格式的标记
        const start_marker = "fqmet:knmll";
        const start_idx = std.mem.indexOf(u8, data, start_marker) orelse 0;
        const content = data[start_idx..];

        // 复制到新的分配
        var result = std.ArrayList(u8).init(self.allocator);
        errdefer result.deinit();
        try result.appendSlice(content);
        return result.toOwnedSlice();
    }

    // 列出可用外壳
    pub fn listShells() []const ShellType {
        return &[_]ShellType{ .http_api, .json_rpc, .cli_output, .html_page, .plain_text };
    }
};

test "canjiang dress http_api" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    var chi = Chrysalis.init(allocator, "fqmet:knmll\ntest\nende");

    const dressed = try chi.dress(.http_api);
    defer allocator.free(dressed);

    try std.testing.expect(std.mem.indexOf(u8, dressed, "HTTP/1.1 200 OK") != null);
    try std.testing.expect(std.mem.indexOf(u8, dressed, "fqmet:knmll") != null);
    try std.testing.expect(std.mem.indexOf(u8, dressed, "X-NJSG-Active: true") != null);
}

test "canjiang dress html" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    var chi = Chrysalis.init(allocator, "test content");

    const dressed = try chi.dress(.html_page);
    defer allocator.free(dressed);

    try std.testing.expect(std.mem.indexOf(u8, dressed, "<!DOCTYPE html>") != null);
    try std.testing.expect(std.mem.indexOf(u8, dressed, "njsg-perception") != null);
}

test "canjiang shed" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    var chi = Chrysalis.init(allocator, "unused");

    const wrapped = "HTTP/1.1 200 OK\r\n\r\nfqmet:knmll\nid:★\nende";
    const naked = try chi.shed(wrapped);
    defer allocator.free(naked);
    try std.testing.expect(std.mem.indexOf(u8, naked, "fqmet:knmll") != null);
}
