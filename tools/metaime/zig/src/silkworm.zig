// ============================================================
// 蚕蛹反向寄生壳 · Cocoon Reverse Parasite
//
// 核心哲学：
//   不是自身适应外部规则，而是外部系统模仿自身。
//
// 流程：
//   1. 下载蚕蛹初始包（含解析器 + 壳选项清单）
//   2. 识别外部系统（壳探测）
//   3. 选择寄生壳（手动 or 自动匹配）
//   4. 执行 脱壳穿衣 操作
//   5. 生成配置文件 + 挂载内核
//   6. 外部系统通过壳与我们交互 → 被反向模仿
// ============================================================

const std = @import("std");
const glyph_mod = @import("glyph.zig");
const yunmu_mod = @import("yunmu.zig");

// ── 外壳类型（外部系统的接口形态） ──
pub const ShellKind = enum {
    posix,       // POSIX/Linux 系统调用壳
    win32,       // Windows API 壳
    wasm,        // WebAssembly 沙盒壳
    android,     // Android Binder/JNI 壳
    ios,         // iOS XPC/ObjC 壳
    browser,     // 浏览器 DOM/Web API 壳
    container,   // Docker/K8s 容器壳
    cloud,       // AWS/GCP/Azure SDK 壳

    pub fn name(self: ShellKind) []const u8 {
        return switch (self) {
            .posix    => "POSIX/Linux",
            .win32    => "Windows API",
            .wasm     => "WebAssembly",
            .android  => "Android",
            .ios      => "iOS",
            .browser  => "浏览器",
            .container => "容器",
            .cloud    => "云平台",
        };
    }

    pub fn detection_hints(self: ShellKind) []const u8 {
        return switch (self) {
            .posix    => "syscall:/proc/self/exe",
            .win32    => "CreateFileW:kernel32.dll",
            .wasm     => "wasm_memory:linear",
            .android  => "Binder:/dev/binder",
            .ios      => "xpc_connection:mach",
            .browser  => "document:window",
            .container => "cgroup:/proc/1/cpuset",
            .cloud    => "metadata:169.254.169.254",
        };
    }
};

// ── 蚕蛹包（初始下载/携带物） ──
pub const Cocoon = struct {
    version: []const u8,
    parser_kind: ParserKind,
    available_shells: []const ShellKind,
    kernel_rules: []const u8,
    config_template: []const u8,

    pub fn init_default() Cocoon {
        return .{
            .version = "0.1.0-silkworm",
            .parser_kind = .auto_detect,
            .available_shells = &[_]ShellKind{
                .posix, .wasm, .browser, .container,
            },
            .kernel_rules = "full-hidden",
            .config_template = "auto",
        };
    }
};

pub const ParserKind = enum {
    auto_detect,       // 自动探测
    pinyin_parser,     // 拼音/元输入法解析器
    json_parser,       // JSON 配置解析器
    binary_parser,     // 二进制协议解析器
    semantic_parser,   // 语义意图解析器
};

// ── 寄生壳选项 ──
pub const ParasiteOption = struct {
    shell: ShellKind,
    compatible: bool,       // 是否与当前内核兼容
    requires_dressing: bool, // 是否需要穿衣
    force_match: f32,       // 自动匹配强度 0-1
};

// ── 蚕蛹引擎 ──
pub const SilkwormEngine = struct {
    cocoon: Cocoon,
    detected_shell: ?ShellKind,
    selected_shell: ?ShellKind,
    dressed: bool,
    config_generated: bool,

    pub fn init() SilkwormEngine {
        return .{
            .cocoon = Cocoon.init_default(),
            .detected_shell = null,
            .selected_shell = null,
            .dressed = false,
            .config_generated = false,
        };
    }

    /// 探测外部系统壳类型
    pub fn detect_shell(self: *SilkwormEngine, hints: []const u8) ShellKind {
        // 基于探测提示匹配最可能的壳
        inline for (std.enums.values(ShellKind)) |sk| {
            const hint = sk.detection_hints();
            if (std.mem.indexOf(u8, hints, hint) != null) {
                self.detected_shell = sk;
                return sk;
            }
        }
        // 默认 posix
        self.detected_shell = .posix;
        return .posix;
    }

/// 寄生选项表（编译期常量）
const default_options: []const ParasiteOption = &.{
    .{ .shell = .posix, .compatible = true, .requires_dressing = false, .force_match = 0.95 },
    .{ .shell = .wasm, .compatible = true, .requires_dressing = true, .force_match = 0.85 },
    .{ .shell = .browser, .compatible = true, .requires_dressing = true, .force_match = 0.75 },
    .{ .shell = .container, .compatible = true, .requires_dressing = false, .force_match = 0.60 },
};

/// 获取自动匹配的寄生选项
pub fn auto_match_options(self: SilkwormEngine) []const ParasiteOption {
    _ = self;
    return default_options;
}

    /// 选择寄生壳
    pub fn select_shell(self: *SilkwormEngine, shell: ShellKind) bool {
        // 检查是否在可用列表
        for (self.cocoon.available_shells) |available| {
            if (available == shell) {
                self.selected_shell = shell;
                return true;
            }
        }
        return false;
    }

    /// 执行脱壳穿衣
    pub fn dress(self: *SilkwormEngine) bool {
        if (self.selected_shell == null) return false;
        self.dressed = true;
        return true;
    }

    /// 生成配置文件（脱壳穿衣后的壳文件）
    pub fn generate_config(self: *SilkwormEngine) []const u8 {
        self.config_generated = true;
        // 返回静态配置模板（实际应为动态生成）
        return switch (self.selected_shell orelse .posix) {
            .posix => "[shell]\ntype=posix\nkernel=hidden\ninterface=system_call",
            .wasm => "[shell]\ntype=wasm\nkernel=hidden\ninterface=func_table",
            .browser => "[shell]\ntype=browser\nkernel=hidden\ninterface=dom_bridge",
            .container => "[shell]\ntype=container\nkernel=hidden\ninterface=cgroup",
            else => "[shell]\ntype=unknown\nkernel=hidden\ninterface=none",
        };
    }

    /// 显示当前蚕蛹状态
    pub fn display(self: SilkwormEngine) void {
        std.debug.print("\n══════════════════════════════════════\n", .{});
        std.debug.print(" 蚕蛹反向寄生壳 · 状态\n", .{});
        std.debug.print("══════════════════════════════════════\n", .{});
        std.debug.print(" 蚕蛹版本: {s}\n", .{self.cocoon.version});
        std.debug.print(" 解析器:   {s}\n", .{@tagName(self.cocoon.parser_kind)});
        std.debug.print(" 检测壳:   {s}\n", .{if (self.detected_shell) |s| s.name() else "未检测"});
        std.debug.print(" 选择壳:   {s}\n", .{if (self.selected_shell) |s| s.name() else "未选择"});
        std.debug.print(" 穿衣状态: {s}\n", .{if (self.dressed) "已穿衣 ✓" else "未穿衣 ○"});
        std.debug.print(" 配置生成: {s}\n", .{if (self.config_generated) "已生成 ✓" else "未生成 ○"});

        std.debug.print("\n ── 可用寄生选项 ──\n", .{});
        const options = self.auto_match_options();
        inline for (std.enums.values(ShellKind)) |sk| {
            for (options) |opt| {
                if (opt.shell == sk) {
                    const marker = if (opt.compatible) "✓" else "✗";
                    std.debug.print("  {s} {s} (匹配度: {d:.0}%)\n", .{ marker, sk.name(), opt.force_match * 100 });
                }
            }
        }

        std.debug.print("══════════════════════════════════════\n", .{});
    }
};

// ── 蚕蛹完整生命周期 ──
pub fn run_silkworm_lifecycle(allocator: std.mem.Allocator) !void {
    _ = allocator;
    var engine = SilkwormEngine.init();

    // 1. 探测外部系统
    const hints = "syscall:read, syscall:write, /proc/version";
    const detected = engine.detect_shell(hints);
    std.debug.print("  [蚕蛹] 探测到外部系统: {s}\n", .{detected.name()});

    // 2. 自动选择最匹配的壳
    const options = engine.auto_match_options();
    var best: ?ShellKind = null;
    var best_force: f32 = 0;
    for (options) |opt| {
        if (opt.force_match > best_force) {
            best = opt.shell;
            best_force = opt.force_match;
        }
    }

    // 3. 选择 + 穿衣
    if (best) |shell| {
        if (engine.select_shell(shell)) {
            _ = engine.dress();
            std.debug.print("  [蚕蛹] 选择壳: {s} (穿衣成功)\n", .{shell.name()});
        }
    }

    // 4. 显示状态
    engine.display();
}

// ── 单元测试 ──
test "shell_name_posix" {
    try std.testing.expectEqualStrings("POSIX/Linux", ShellKind.posix.name());
}

test "shell_hint_wasm" {
    try std.testing.expectEqualStrings("wasm_memory:linear", ShellKind.wasm.detection_hints());
}

test "cocoon_default" {
    const c = Cocoon.init_default();
    try std.testing.expectEqualStrings("0.1.0-silkworm", c.version);
}

test "detect_posix" {
    var e = SilkwormEngine.init();
    const result = e.detect_shell("syscall:open, /proc/version");
    try std.testing.expectEqual(ShellKind.posix, result);
}

test "detect_wasm" {
    var e = SilkwormEngine.init();
    const result = e.detect_shell("wasm_memory:linear");
    try std.testing.expectEqual(ShellKind.wasm, result);
}

test "select_available_shell" {
    var e = SilkwormEngine.init();
    try std.testing.expect(e.select_shell(.posix));
    try std.testing.expect(e.selected_shell == .posix);
}

test "select_unavailable_shell" {
    var e = SilkwormEngine.init();
    // iOS 不在默认可用列表中
    try std.testing.expect(!e.select_shell(.ios));
}

test "dress_requires_selection" {
    var e = SilkwormEngine.init();
    try std.testing.expect(!e.dress()); // 未选择壳 → 失败
}

test "full_lifecycle" {
    var e = SilkwormEngine.init();
    _ = e.detect_shell("syscall:read");
    _ = e.select_shell(.posix);
    try std.testing.expect(e.dress());
    const config = e.generate_config();
    try std.testing.expect(std.mem.indexOf(u8, config, "kernel=hidden") != null);
}

test "auto_match" {
    var e = SilkwormEngine.init();
    const options = e.auto_match_options();
    try std.testing.expect(options.len > 0);
    // posix 匹配度最高
    try std.testing.expect(options[0].shell == .posix);
}
