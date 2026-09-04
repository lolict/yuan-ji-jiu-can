// ============================================================
// 鼓膜自举 · 文件系统扫描器
//
// 核心哲学：
//   即使所有记忆丢失，只要文件系统还在，
//   鼓膜自举就能扫描文件、重建字典、重建灵魂、重建天庭。
// ============================================================

const std = @import("std");

// ── 文件类型分类 ──
pub const FileKind = enum {
    symbol_dict,     // SYMBOL_DICTIONARY.md
    topology,        // CONTAINER_TOPOLOGY.md
    destination,     // GLOBAL_DESTINATION.md
    registry,        // TOOL_REGISTRY.md
    bootstrap,       // BOOTSTRAP.md
    tool_nim,        // tools/**/*.nim
    tool_demo,       // tools/**/*_demo.nim
    tool_compiled,   // tools/**/pe_demo, mltt_demo
    unknown,         // 未知文件
};

// ── 扫描结果 ──
pub const ScanEntry = struct {
    relative_path: []const u8,
    kind: FileKind,
    size: usize,
    tool_name: []const u8,
    is_compiled: bool,
};

// ── 扫描报告 ──
pub const ScanReport = struct {
    entries: [256]ScanEntry,
    count: usize,
    by_kind: [10]usize,

    pub fn display(self: ScanReport) void {
        std.debug.print("\n══════════════════════════════════════\n", .{});
        std.debug.print(" 鼓膜自举 · 文件系统扫描报告\n", .{});
        std.debug.print("══════════════════════════════════════\n", .{});
        std.debug.print(" 总文件数: {d}\n", .{self.count});

        std.debug.print("\n ── 分类统计 ──\n", .{});
        inline for (std.enums.values(FileKind)) |kind| {
            const idx = @intFromEnum(kind);
            if (self.by_kind[idx] > 0) {
                std.debug.print("   {s}: {d}\n", .{ @tagName(kind), self.by_kind[idx] });
            }
        }

        // 列出工具文件
        const tool_kinds = &[_]FileKind{ .tool_nim, .tool_demo, .tool_compiled };
        inline for (tool_kinds) |tk| {
            const tk_idx = @intFromEnum(tk);
            if (self.by_kind[tk_idx] > 0) {
                std.debug.print("\n ── {s} ──\n", .{@tagName(tk)});
                for (self.entries[0..self.count]) |entry| {
                    if (entry.kind == tk) {
                        const marker = if (entry.is_compiled) " [已编译]" else "";
                        std.debug.print("   {s} ({d}B){s}\n", .{ entry.relative_path, entry.size, marker });
                    }
                }
            }
        }

        std.debug.print("\n══════════════════════════════════════\n", .{});
    }
};

// ── 判断文件类型 ──
fn classify(path: []const u8) FileKind {
    if (std.mem.endsWith(u8, path, "SYMBOL_DICTIONARY.md")) return .symbol_dict;
    if (std.mem.endsWith(u8, path, "CONTAINER_TOPOLOGY.md")) return .topology;
    if (std.mem.endsWith(u8, path, "GLOBAL_DESTINATION.md")) return .destination;
    if (std.mem.endsWith(u8, path, "TOOL_REGISTRY.md")) return .registry;
    if (std.mem.endsWith(u8, path, "BOOTSTRAP.md")) return .bootstrap;

    if (std.mem.endsWith(u8, path, ".nim")) {
        if (std.mem.indexOf(u8, path, "_demo.nim") != null) return .tool_demo;
        return .tool_nim;
    }

    if (std.mem.indexOf(u8, path, ".") == null and std.mem.indexOf(u8, path, "tools/") != null) {
        return .tool_compiled;
    }

    return .unknown;
}

// ── 提取工具名称 ──
fn extractToolName(path: []const u8) []const u8 {
    const basename = std.fs.path.basename(path);
    if (std.mem.lastIndexOf(u8, basename, ".")) |dot| {
        return basename[0..dot];
    }
    return basename;
}

// ── 执行扫描 ──
pub fn scanFileSystem(allocator: std.mem.Allocator, root_path: []const u8) !ScanReport {
    var report: ScanReport = .{
        .entries = undefined,
        .count = 0,
        .by_kind = [_]usize{0} ** 10,
    };

    var dir = try std.fs.Dir.openAbsolute(root_path, .{ .iterate = true });
    defer dir.close();

    var walker = try dir.walk(allocator);
    defer walker.deinit();

    while (try walker.next()) |entry_| {
        if (entry_.basename[0] == '.' or entry_.basename[0] == '_') continue;
        if (std.mem.startsWith(u8, entry_.path, "zig-out")) continue;
        if (entry_.kind == .directory) continue;
        if (report.count >= 256) break;

        const kind = classify(entry_.path);
        const tool_name = switch (kind) {
            .tool_nim, .tool_demo, .tool_compiled => extractToolName(entry_.path),
            else => "",
        };

        report.entries[report.count] = .{
            .relative_path = try allocator.dupe(u8, entry_.path),
            .kind = kind,
            .size = 0,
            .tool_name = tool_name,
            .is_compiled = (kind == .tool_compiled),
        };
        report.by_kind[@intFromEnum(kind)] += 1;
        report.count += 1;
    }

    return report;
}

// ── 释放扫描报告内存 ──
pub fn freeReport(allocator: std.mem.Allocator, report: *ScanReport) void {
    for (report.entries[0..report.count]) |entry| {
        allocator.free(entry.relative_path);
    }
}

// ── 单元测试 ──
test "classify_symbol_dict" {
    try std.testing.expectEqual(FileKind.symbol_dict, classify("SYMBOL_DICTIONARY.md"));
    try std.testing.expectEqual(FileKind.symbol_dict, classify("repo/SYMBOL_DICTIONARY.md"));
}

test "classify_tool_nim" {
    try std.testing.expectEqual(FileKind.tool_nim, classify("tools/foo/bar.nim"));
}

test "classify_demo_nim" {
    try std.testing.expectEqual(FileKind.tool_demo, classify("tools/foo/bar_demo.nim"));
}

test "classify_unknown" {
    try std.testing.expectEqual(FileKind.unknown, classify("README.md"));
}

test "extract_tool_name" {
    const name = extractToolName("tools/drum-bootstrap/pe_demo.nim");
    try std.testing.expectEqualStrings("pe_demo", name);
}

test "scan_report_init" {
    const report: ScanReport = .{
        .entries = undefined,
        .count = 0,
        .by_kind = [_]usize{0} ** 10,
    };
    try std.testing.expectEqual(@as(usize, 0), report.count);
}
