// build.zig — 阶梯制·双向鸿约 (zig 0.13)
const std = @import("std");

pub fn build(b: *std.Build) void {
    const tgt32 = std.zig.CrossTarget{ .cpu_arch = .wasm32, .os_tag = .freestanding };
    const tgt64 = std.zig.CrossTarget{ .cpu_arch = .wasm64, .os_tag = .freestanding };

    const wasm32 = b.addExecutable(.{
        .name = "mqf32",
        .root_source_file = .{ .src_path = .{ .owner = b, .sub_path = "lib.zig" } },
        .target = tgt32,
        .optimize = .ReleaseSmall,
    });
    wasm32.addCSourceFile(.{ .file = .{ .src_path = .{ .owner = b, .sub_path = "main.c" } }, .flags = &[_][]const u8{ "-Wall", "--std=c11" } });
    wasm32.linkLibC();
    wasm32.install();

    const wasm64 = b.addExecutable(.{
        .name = "mqf64",
        .root_source_file = .{ .src_path = .{ .owner = b, .sub_path = "lib.zig" } },
        .target = tgt64,
        .optimize = .ReleaseSmall,
    });
    wasm64.addCSourceFile(.{ .file = .{ .src_path = .{ .owner = b, .sub_path = "main.c" } }, .flags = &[_][]const u8{ "-Wall", "--std=c11" } });
    wasm64.linkLibC();
    wasm64.install();

    const s32 = b.step("wasm32", "Build wasm32");
    s32.dependOn(&wasm32.step);
    const s64 = b.step("wasm64", "Build wasm64");
    s64.dependOn(&wasm64.step);
    const s_all = b.step("all", "Build all");
    s_all.dependOn(&wasm32.step);
    s_all.dependOn(&wasm64.step);
    b.default_step = s_all;
}
