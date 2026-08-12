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
}
