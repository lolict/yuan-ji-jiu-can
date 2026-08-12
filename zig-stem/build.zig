// build.zig — 阶梯制·双向鸿约 (zig 0.13)
const std = @import("std");

pub fn build(b: *std.Build) void {
    // ── 目标查询 ──
    const wasm32_query = std.Target.Query{
        .cpu_arch = .wasm32,
        .os_tag = .freestanding,
    };
    const wasm64_query = std.Target.Query{
        .cpu_arch = .wasm64,
        .os_tag = .freestanding,
    };

    const wasm32_target = b.resolveTargetQuery(wasm32_query);
    const wasm64_target = b.resolveTargetQuery(wasm64_query);

    // ── wasm32 产物 ──
    const wasm32 = b.addExecutable(.{
        .name = "mqf32",
        .root_source_file = .{ .src_path = .{ .owner = b, .sub_path = "lib.zig" } },
        .target = wasm32_target,
        .optimize = .ReleaseSmall,
    });
    wasm32.addCSourceFile(.{ .file = .{ .src_path = .{ .owner = b, .sub_path = "main.c" } }, .flags = &[_][]const u8{ "-Wall", "--std=c11" } });
    wasm32.linkLibC();
    b.installArtifact(wasm32);

    // ── wasm64 产物 ──
    const wasm64 = b.addExecutable(.{
        .name = "mqf64",
        .root_source_file = .{ .src_path = .{ .owner = b, .sub_path = "lib.zig" } },
        .target = wasm64_target,
        .optimize = .ReleaseSmall,
    });
    wasm64.addCSourceFile(.{ .file = .{ .src_path = .{ .owner = b, .sub_path = "main.c" } }, .flags = &[_][]const u8{ "-Wall", "--std=c11" } });
    wasm64.linkLibC();
    b.installArtifact(wasm64);

    // ── 构建步骤 ──
    const s32 = b.step("wasm32", "Build wasm32");
    s32.dependOn(&wasm32.step);
    const s64 = b.step("wasm64", "Build wasm64");
    s64.dependOn(&wasm64.step);
    const s_all = b.step("all", "Build all");
    s_all.dependOn(&wasm32.step);
    s_all.dependOn(&wasm64.step);
    b.default_step = s_all;

    // ── 阶梯进制测试 (原生编译,非 wasm) ──
    const test_exe = b.addExecutable(.{
        .name = "test_stride",
        .root_source_file = .{ .src_path = .{ .owner = b, .sub_path = "test_stride.c" } },
        .target = b.resolveTargetQuery(.{ .cpu_arch = .x86_64, .os_tag = .linux }),
        .optimize = .Debug,
    });
    test_exe.addCSourceFile(.{ .file = .{ .src_path = .{ .owner = b, .sub_path = "main.c" } }, .flags = &[_][]const u8{ "-Wall", "--std=c11", "-DTEST_MODE" } });
    test_exe.linkLibC();
    b.installArtifact(test_exe);

    const test_step = b.step("test", "Run stride base tests");
    const run_test = b.addRunArtifact(test_exe);
    test_step.dependOn(&run_test.step);
}
