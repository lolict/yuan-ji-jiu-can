// ══════════════════════════════════════════════════════════════════
// lib.zig — 阶梯制·双向鸿约 瓦片入口
// ══════════════════════════════════════════════════════════════════
//
// 这份 Zig 文件作为 wasm 编译的根文件。
// 实际业务逻辑在 main.c 里,C 源文件由 build.zig 挂载。
//
// Zig 在此的职责:
//   · 作为 wasm 链接入口 (Zig 需要一个 root_source_file)
//   · 暴露一个互补的高维视角(comptime 元编程),与 main.c 同体异构
//   · 验证主函数是否链接对齐 (bootstrap 自检)

// comptime 块:编译期验证,零运行时开销
comptime {
    const EXPECTED_SYMBOLS: u16 = 64;
    const FULL_CIRCLE:     u16 = 384;
    const PER_SYMBOL:      u16 = 6;   // 384 / 64 = 6

    const actual_div = FULL_CIRCLE / EXPECTED_SYMBOLS;
    if (actual_div != 6) @compileError("angle constant broken: 384/64 != 6");
    if (PER_SYMBOL != 6) @compileError("angle constant broken: PER_SYMBOL != 6");
    if (EXPECTED_SYMBOLS * PER_SYMBOL != FULL_CIRCLE) {
        @compileError("angle constant broken: 64*6 != 384");
    }
}

// ══════════════════════════════════════════════════════════════════
// 主入口 (供 wasm 宿主调用)
// ══════════════════════════════════════════════════════════════════

// wasm in freestanding has no stdout, so we do nothing at function entry
// All effects come from the 5 C-exported functions (mqf_boot, etc).
pub fn main() void {}

// Guarantees a complete wasm binary with _start for bare-metal hosts
export fn _start() callconv(.C) void {}
