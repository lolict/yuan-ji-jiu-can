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
// 导出函数 (供宿主/wasm runtime 调用)
// ══════════════════════════════════════════════════════════════════

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
