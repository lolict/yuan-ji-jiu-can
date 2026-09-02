// lib.zig — 阶梯制·双向鸿约 瓦片入口
//
// 这份 Zig 文件作为 wasm 编译的根文件。
// FQMCP 管线函数在 main.c 中，由 build.zig 挂载。
// Zig 在此的职责: 作为 wasm 链接入口 + 编译期验证。

comptime {
    const EXPECTED_SYMBOLS: u16 = 64;
    const FULL_CIRCLE: u16 = 384;
    const PER_SYMBOL: u16 = 6;
    const TRADITIONAL_CIRCLE: u16 = 360;
    const OFFSET_PER_LEVEL: u16 = FULL_CIRCLE - TRADITIONAL_CIRCLE;

    if (EXPECTED_SYMBOLS * PER_SYMBOL != FULL_CIRCLE)
        @compileError("公理失败: 64 × 6 ≠ 384");
    if (OFFSET_PER_LEVEL != 24)
        @compileError("公理失败: 偏移量 ≠ 24°");
    if (FULL_CIRCLE % EXPECTED_SYMBOLS != 0)
        @compileError("公理失败: 384 必须被 64 整除");

    const gcd_360 = gcd(OFFSET_PER_LEVEL, TRADITIONAL_CIRCLE);
    const period_360 = TRADITIONAL_CIRCLE / gcd_360;
    const gcd_384 = gcd(OFFSET_PER_LEVEL, FULL_CIRCLE);
    const period_384 = FULL_CIRCLE / gcd_384;

    if (period_360 != 15) @compileError("定理失败: 传统周期 ≠ 15");
    if (period_384 != 16) @compileError("定理失败: 满圆周期 ≠ 16");
    if (period_360 == period_384) @compileError("定理失败: 两周期不能相等");

    if (angle_to_absolute(0, 0) != 0) @compileError("第 0 层 0 号绝对位置 ≠ 0");
    if (angle_to_absolute(1, 0) != 24) @compileError("第 1 层螺距 ≠ 24");
    if (angle_to_absolute(15, 0) != 360) @compileError("第 15 层位置 ≠ 360");
    if (angle_to_absolute(16, 0) != 0) @compileError("第 16 层回归 ≠ 0");
}

fn angle_to_absolute(level: u32, symbol_idx: u32) u16 {
    const FULL_CIRCLE: u32 = 384;
    const OFFSET_PER_LEVEL: u32 = 24;
    const PER_SYMBOL: u32 = 6;
    const symbol_angle = symbol_idx * PER_SYMBOL;
    const level_offset = level * OFFSET_PER_LEVEL;
    return @intCast((level_offset + symbol_angle) % FULL_CIRCLE);
}

fn gcd(a: u16, b: u16) u16 {
    var x = a;
    var y = b;
    while (y != 0) {
        const temp = y;
        y = x % y;
        x = temp;
    }
    return x;
}

pub fn main() void {}
export fn _start() callconv(.C) void {}
