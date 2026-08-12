// build.zig — mlil 编译器构建配置
// mlil = 毛利兰编程语言 = 表面/界面语言 → 编译为 WASM
const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // ── mlil 编译器本体 (生成 WASM 的本地工具) ──
    const mlil = b.addExecutable(.{
        .name = "mlil",
        .root_source_file = .{ .src_path = .{ .owner = b, .sub_path = "src/main.zig" } },
        .target = target,
        .optimize = optimize,
    });
    b.installArtifact(mlil);

    // ── 测试 mlil 编译器自身 ──
    const tests = b.addTest(.{
        .root_source_file = .{ .src_path = .{ .owner = b, .sub_path = "src/test_main.zig" } },
        .target = target,
        .optimize = optimize,
    });
    const run_tests = b.addRunArtifact(tests);
    const test_step = b.step("test", "Run mlil compiler tests");
    test_step.dependOn(&run_tests.step);

    // ── 编译示例 (可选) ──
    const examples_step = b.step("examples", "Build example mlil programs");

    b.default_step = examples_step;
    examples_step.dependOn(&mlil.step);
}
