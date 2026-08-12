// ══════════════════════════════════════════════════════════════════
// main.c — 满全法 · 双向鸿约 C 内核桩
// ══════════════════════════════════════════════════════════════════
//
// 这份 C 源码同时编译为 wasm32 和 wasm64 两份目标,
// 体现满全法 WASM 内核的「阶梯制·双向鸿约」策略:
//   · 固定宽度类型 (u32/u64) 消除架构差异
//   · 统一宏前缀 MQF_ 让两份目标共享同一套语义
//   · 同一份源码、同一份意志、两种表达
//
// 用 Zig 编译:
//   zig build          → 产出 zig-out/wasm32/mqf.wasm + zig-out/wasm64/mqf.wasm
//   zig build -Dtarget=wasm32-wasi
//   zig build -Dtarget=wasm64-wasi (未来,当 zig 支持时)
//
// 架构注解:
//   wasm32-wasi — 当前主流,Anthon 系/K8s/wasmtime 全支持
//   wasm64       — 未来地址空间 >4GB 时的主力 (目前为构想目标)

#include <stdint.h>
#include <stddef.h>

// ── 固定宽度统一类型: 对 wasm32 和 wasm64 语义一致 ──
typedef uint8_t  mqf_u8;
typedef uint16_t mqf_u16;
typedef uint32_t mqf_u32;
typedef uint64_t mqf_u64;
typedef int32_t  mqf_i32;
typedef int32_t  mqf_ret;   // 通用返回值 (0=ok, 非0=err)

// ── 64 位单元格: 满全法坐标/极性/主权/价值编码的载体 ──
// 64 位对齐,wasm32 用两个 u32 模拟,wasm64 直接映射
typedef struct {
    mqf_u64 bits;
} mqf_cell;

// ── 拓扑坐标 (x,y,z 各 8 位) ──
typedef struct {
    mqf_u8 x, y, z;
} mqf_coord3;

// ── 五大判断规则统一返回 ──
typedef enum {
    MQF_JUDGE_REWARD  =  1,   // 正向奖励
    MQF_JUDGE_NEUTRAL =  0,   // 中性
    MQF_JUDGE_PENALTY = -1,   // 负向惩罚
} mqf_judge;

// ── 角距离: 384° 制式 ──
#define MQF_FULL_CIRCLE 384
#define MQF_PER_SYMBOL  6      // 384 / 64

// ══════════════════════════════════════════════════════════════════
// 阶梯进制运算 — 加法主权, 乘法补偿, 螺旋不重叠
// ══════════════════════════════════════════════════════════════════
//
// 阶梯进制数: (sheet, symbol)
//   sheet  — 螺旋圈数 (层级, 从 0 开始)
//   symbol — 该圈内的符号位置 (0..63)
//
// 绝对角度 = sheet × 24 + symbol × 6
//   · 每圈 64 个符号, 每符号 6°
//   · 每圈偏移 24° (螺距), 保证螺旋永不重叠
//   · 绝对角度可以任意大, 不模 384 (保留层级信息)

typedef struct {
    mqf_u32 sheet;      // 螺旋圈数
    mqf_u8  symbol;     // 圈内符号位置 (0..63)
} mqf_stride;

// ── 传统 u64 → 阶梯进制 ──
// 传统数字 N 的绝对角度 = N × 6 (每单位 6°)
// 转换为 (sheet, symbol):
//   total_degrees = N × 6
//   sheet = total_degrees / 384
//   symbol = (total_degrees % 384) / 6
mqf_stride mqf_stride_from_u64(mqf_u64 n) {
    mqf_stride r;
    mqf_u64 total_degrees = n * 6;
    r.sheet = (mqf_u32)(total_degrees / 384);
    r.symbol = (mqf_u8)((total_degrees % 384) / 6);
    return r;
}

// ── 阶梯进制 → 传统 u64 ──
mqf_u64 mqf_stride_to_u64(mqf_stride s) {
    mqf_u64 total_degrees = (mqf_u64)s.sheet * 384 + (mqf_u64)s.symbol * 6;
    return total_degrees / 6;
}

// ── 阶梯加法 (主权运算) ──
// 阶梯进制加法 = u64 加法 (保持一一对应, zero 是单位元)
// 等价于 64 进制加法: 低位 symbol 相加, 进位到高位 sheet
mqf_stride mqf_stride_add(mqf_stride a, mqf_stride b) {
    mqf_stride r;
    mqf_u16 sym_sum = (mqf_u16)a.symbol + (mqf_u16)b.symbol;
    r.sheet = a.sheet + b.sheet + (mqf_u32)(sym_sum >> 6);  // sym_sum / 64 进位
    r.symbol = (mqf_u8)(sym_sum & 0x3F);                     // sym_sum % 64
    return r;
}

// ── 阶梯乘法 (补偿运算) ──
// a × b: 转 u64, 相乘, 再转回阶梯进制
// 乘法作为加法的补偿: 用 u64 乘法的高效替代 O(b) 的连加
mqf_stride mqf_stride_mul(mqf_stride a, mqf_stride b) {
    mqf_u64 na = mqf_stride_to_u64(a);
    mqf_u64 nb = mqf_stride_to_u64(b);
    mqf_u64 product;
    
    // 检测溢出: 如果 na > UMAX/nb, 乘积会溢出
    if (na != 0 && nb > 0xFFFFFFFFFFFFFFFFULL / na) {
        product = 0xFFFFFFFFFFFFFFFFULL;  // 饱和到最大值
    } else {
        product = na * nb;
    }
    return mqf_stride_from_u64(product);
}

// ── 阶梯比较 ──
// 先比 sheet, 再比 symbol
mqf_i32 mqf_stride_compare(mqf_stride a, mqf_stride b) {
    if (a.sheet < b.sheet) return -1;
    if (a.sheet > b.sheet) return 1;
    if (a.symbol < b.symbol) return -1;
    if (a.symbol > b.symbol) return 1;
    return 0;
}

// ── 绝对角度 (用于调试和验证) ──
mqf_u64 mqf_stride_absolute(mqf_stride s) {
    return (mqf_u64)s.sheet * 24 + (mqf_u64)s.symbol * 6;
}

// 符号 → 角 (符号 N 占 N*6°)
mqf_u32 mqf_symbol_angle(mqf_u8 sym) {
    if (sym >= 64) return 0;
    return (mqf_u32)(sym * MQF_PER_SYMBOL);
}

// two angles → shortest arc distance (0..192)
mqf_u32 mqf_arc_distance(mqf_u32 a, mqf_u32 b) {
    mqf_i32 d = (mqf_i32)(b - a);
    if (d < 0) d = -d;
    if (d > MQF_FULL_CIRCLE / 2) d = MQF_FULL_CIRCLE - d;
    return (mqf_u32)d;
}

// 8 阶段时间链: 符号 N 在第几阶段到达 ancestors[M-N] (简化版)
mqf_u8  mqf_time_stage_depth(mqf_u8 n) {
    if (n == 0) return 0;
    mqf_u8 depth = 0;
    mqf_u8 cur = n;
    while (cur > 1) {
        // spf (最小质因数)
        mqf_u8 spf = 2;
        while (cur % spf != 0) spf++;
        if (spf > cur) break;
        cur /= spf;
        depth++;
        if (depth >= 8) break;   // 共 8 阶段
    }
    return depth;
}

// (x,y,z) 在 4×4×4 立方体中是否与 (ax,ay,az) 6-邻接 (topo adj)
mqf_i32 mqf_is_adjacent(mqf_u8 x, mqf_u8 y, mqf_u8 z,
                       mqf_u8 ax, mqf_u8 ay, mqf_u8 az) {
    mqf_i32 dx = (mqf_i32)x - (mqf_i32)ax;
    mqf_i32 dy = (mqf_i32)y - (mqf_i32)ay;
    mqf_i32 dz = (mqf_i32)z - (mqf_i32)az;
    if (dx < 0) dx = -dx;
    if (dy < 0) dy = -dy;
    if (dz < 0) dz = -dz;
    // 6-邻接: 曼哈顿距离 = 1 且仅一维动
    mqf_i32 manhattan = dx + dy + dz;
    if (manhattan != 1) return 0;
    // 确保坐标在 [0,3] 范围
    if (x > 3 || y > 3 || z > 3) return 0;
    if (ax > 3 || ay > 3 || az > 3) return 0;
    return 1;
}

// 主入口 (被 wasm 宿主调用)
mqf_ret mqf_boot(void) {
    return 0;   // 自举成功
}
