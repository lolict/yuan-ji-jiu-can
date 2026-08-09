// ══════════════════════════════════════════════════════════════════
// build.zig — 阶梯制·双向鸿约编译脚本
// ══════════════════════════════════════════════════════════════════
//
// 同份 C 源码 main.c → 双出 wasm32 + wasm64 两份 .wasm
// Zig 在此充当:
//   1. C 编译器 (内部绑定 clang,无外部 CC 依赖)
//   2. 交叉编译器 (单命令切到 wasm32/wasm64)
//   3. 构建调度器 (多产出统一管理)
//
// 用法:
//   zig build                     → 产出全部目标
//   zig build wasm32              → 仅 wasm32
//   zig build wasm64              → 仅 wasm64
//   zig build run -Dtarget=wasm32-wasi   → wasmedge/wasmtime 跑结果
//
// 产出:
//   zig-out/wasm32/release/mqf.wasm
//   zig-out/wasm64/release/mqf.wasm

const std = @import("std");

pub fn build(b: *std.Build) void {
    const target_wasm32 = std.zig.CrossTarget{
        .cpu_arch = .wasm32,
        .os_tag = .freestanding,    // 无 OS,裸 wasm
    };
    const target_wasm64 = std.zig.CrossTarget{
        .cpu_arch = .wasm64,
        .os_tag = .freestanding,
    };

    const opt = b.standardOptimizeOption(.{
        .prefer_debug = false,      // release 体型
    });

    // ── wasm32 目标 (当前生产就绪) ──
    const wasm32 = b.addExecutable(.{
        .name = "mqf",
        .root_source_file = null,   // C 源码
        .target = target_wasm32,
        .optimize = opt,
    });
    wasm32.addCSourceFiles(&.{
        .files = &[_][]const u8{"main.c"},
        .flags = &[_][]const u8{ "-Wall", "-Wextra", "--std=c11" },
    });
    wasm32.linkLibC();              // wasm 版 libc (wasi)
    wasm32.install();

    // ── wasm64 目标 (未来地址空间 >4GB 时的主力) ──
    // zig 当前 0.11+ 已经实验性支持 wasm64
    const wasm64 = b.addExecutable(.{
        .name = "mqf",
        .root_source_file = null,
        .target = target_wasm64,
        .optimize = opt,
    });
    wasm64.addCSourceFiles(&.{
        .files = &[_][]const u8{"main.c"},
        .flags = &[_][]const u8{ "-Wall", "-Wextra", "--std=c11" },
    });
    wasm64.linkLibC();
    wasm64.install();

    // ── 命名 step 让 zig build 可单选 ──
    const step32 = b.step("wasm32", "产出 wasm32-wasi mqf.wasm");
    const step64 = b.step("wasm64", "产出 wasm64-wasi mqf.wasm");
    const step_all = b.step("all", "产出全部 wasm 目标");
    step32.dependOn(&wasm32.step);
    step64.dependOn(&wasm64.step);
    step_all.dependOn(&wasm32.step);
    step_all.dependOn(&wasm64.step);
    b.default_step = step_all;

    // ── 验证 step: 两份产出是否都生成 ──
    const verify = b.addSystemCommand(&[_][]const u8{
        "echo", "✓ 双 wasm 产出就绪",
    });
    verify.step.dependOn(&wasm32.step);
    verify.step.dependOn(&wasm64.step);
    const verify_step = b.step("verify", "验证双产出");
    verify_step.dependOn(&verify.step);
}
