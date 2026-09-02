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

// ────────────────────────────────────────────────────────────
// 一、传统进制 vs 阶梯进制
// ────────────────────────────────────────────────────────────
//
// 传统进制: 数字 = Σ(符号 × 位置权值)
//           乘法是主权，进位是对有限符号集的妥协
//
// 阶梯进制: 数字 = (阶梯层级 × 偏移角) + 角度位置
//           加法是主权，层级是自然涌现（不对齐，不用进位）
//
// 关键: 阶梯进制不需要"进位",因为它不在一条直线上走,
//       它在螺旋线上走——每走满一圈,进入下一层,
//       下一层相对于上一层有一个"叠加态偏移"。
//
// ────────────────────────────────────────────────────────────
// 二、384° 满全法的阶梯结构
// ────────────────────────────────────────────────────────────
//
//   384° = 64 符号 × 6°/符号
//   384° - 360° = 24° = 每层偏移量 (螺旋上升的"螺距")
//
//   走完第 0 层的 64 个符号 (384°) 后,
//   进入第 1 层, 但第 1 层的"0°"位置
//   相对于第 0 层的"0°"位置偏移了 24°。
//
//   这就是"回归圆心,但世界已经不同了"。

// comptime 块:编译期验证,零运行时开销
comptime {
    // ── 公理层 ──
    const EXPECTED_SYMBOLS: u16 = 64;    // 每阶梯符号数
    const FULL_CIRCLE:     u16 = 384;    // 满圆格点数
    const PER_SYMBOL:      u16 = 6;      // 384 / 64 = 6, 每符号度数
    const TRADITIONAL_CIRCLE: u16 = 360; // 传统圆周度数 (被故意不取整)
    const OFFSET_PER_LEVEL: u16 = FULL_CIRCLE - TRADITIONAL_CIRCLE; // 24°

    // ── 公理验证 ──
    if (EXPECTED_SYMBOLS * PER_SYMBOL != FULL_CIRCLE) {
        @compileError("公理失败: 64 × 6 ≠ 384");
    }
    if (OFFSET_PER_LEVEL != 24) {
        @compileError("公理失败: 偏移量 ≠ 24°");
    }
    if (FULL_CIRCLE % EXPECTED_SYMBOLS != 0) {
        @compileError("公理失败: 384 必须被 64 整除");
    }

    // ── 阶梯进制核心定理: 螺旋双周期不重复 ──
    // 在 360° 坐标系中: 走 15 层后回到初始方向
    //    gcd(24, 360) = 24, 周期 = 360/24 = 15
    // 在 384° 坐标系中: 走 16 层后回到初始位置
    //    gcd(24, 384) = 24, 周期 = 384/24 = 16
    // 关键: 15 ≠ 16 !!
    // 这意味着阶梯进制在两个坐标系中永远不会同时重合,
    // 螺旋永远不会完美重叠 —— 这是真正的"非周期覆盖"
    const gcd_360 = gcd(OFFSET_PER_LEVEL, TRADITIONAL_CIRCLE); // gcd(24,360)=24
    const period_360 = TRADITIONAL_CIRCLE / gcd_360;          // 360/24 = 15

    const gcd_384 = gcd(OFFSET_PER_LEVEL, FULL_CIRCLE);       // gcd(24,384)=24
    const period_384 = FULL_CIRCLE / gcd_384;                 // 384/24 = 16

    if (period_360 != 15) {
        @compileError("定理失败: 传统周期 ≠ 15");
    }
    if (period_384 != 16) {
        @compileError("定理失败: 满圆周期 ≠ 16");
    }
    if (period_360 == period_384) {
        @compileError("定理失败: 两周期不能相等,否则螺旋会重叠");
    }

    // ── 结构验证: 阶梯进制的表示唯一性 ──
    // 第 0 层 0 号符号: 在 384° 网格中的绝对位置 = 0
    const abs_0_0 = angle_to_absolute(0, 0);
    if (abs_0_0 != 0) {
        @compileError("验证失败: 第 0 层 0 号绝对位置 ≠ 0");
    }

    // 第 1 层 0 号符号: 绝对位置 = 24 (螺旋螺距)
    const abs_1_0 = angle_to_absolute(1, 0);
    if (abs_1_0 != 24) {
        @compileError("验证失败: 第 1 层螺距 ≠ 24");
    }

    // 第 15 层 0 号符号: 在 360° 系中回到 0°, 但在 384° 系中是 360
    const abs_15_0 = angle_to_absolute(15, 0);
    if (abs_15_0 != 360) {
        @compileError("验证失败: 第 15 层在 384° 系中的位置 ≠ 360");
    }

    // 第 16 层 0 号符号: 在 384° 系中回到 0 (真正满圆回归)
    const abs_16_0 = angle_to_absolute(16, 0);
    if (abs_16_0 != 0) {
        @compileError("验证失败: 第 16 层满圆回归 ≠ 0");
    }
}

/// 计算阶梯数字的"绝对角度"
/// 绝对角度 = (层级 × 偏移量 + 符号角度) mod 满圆
fn angle_to_absolute(level: u32, symbol_idx: u32) u16 {
    const FULL_CIRCLE: u32 = 384;
    const OFFSET_PER_LEVEL: u32 = 24;
    const PER_SYMBOL: u32 = 6;

    const symbol_angle = symbol_idx * PER_SYMBOL;        // 该符号所在的角度
    const level_offset = level * OFFSET_PER_LEVEL;      // 该层的偏移量
    const raw = level_offset + symbol_angle;
    return @intCast(raw % FULL_CIRCLE);
}

/// 最大公约数 (欧几里得算法)
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

// ══════════════════════════════════════════════════════════════════
// 主入口 (供 wasm 宿主调用)
// ══════════════════════════════════════════════════════════════════

// FQMCP WASM 绑定层
const wasm_fqmcp = @import("wasm_fqmcp.zig");

// 编译期验证：确保 FQMCP 函数可用
comptime {
    _ = wasm_fqmcp.fqmcp_init;
    _ = wasm_fqmcp.fqmcp_unary;
    _ = wasm_fqmcp.fqmcp_knmlil;
}

// wasm in freestanding has no stdout, so we do nothing at function entry
// All effects come from the C-exported functions (mqf_boot + fqmcp_*).
pub fn main() void {}

// Guarantees a complete wasm binary with _start for bare-metal hosts
export fn _start() callconv(.C) void {}
