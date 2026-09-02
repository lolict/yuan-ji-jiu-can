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
// 架构注解:
//   wasm32-wasi — 当前主流
//   wasm64       — 未来地址空间 >4GB 时的主力

#include <stdint.h>
#include <stddef.h>

// ── 固定宽度统一类型: 对 wasm32 和 wasm64 语义一致 ──
typedef uint8_t  mqf_u8;
typedef uint16_t mqf_u16;
typedef uint32_t mqf_u32;
typedef uint64_t mqf_u64;
typedef int32_t  mqf_i32;
typedef int32_t  mqf_ret;

typedef struct { mqf_u64 bits; } mqf_cell;
typedef struct { mqf_u8 x, y, z; } mqf_coord3;

typedef enum {
    MQF_JUDGE_REWARD  =  1,
    MQF_JUDGE_NEUTRAL =  0,
    MQF_JUDGE_PENALTY = -1,
} mqf_judge;

#define MQF_FULL_CIRCLE 384
#define MQF_PER_SYMBOL  6

// ── 阶梯进制 ──
typedef struct {
    mqf_u32 sheet;
    mqf_u8  symbol;
} mqf_stride;

mqf_stride mqf_stride_from_u64(mqf_u64 n) {
    mqf_stride r;
    mqf_u64 td = n * 6;
    r.sheet = (mqf_u32)(td / 384);
    r.symbol = (mqf_u8)((td % 384) / 6);
    return r;
}

mqf_u64 mqf_stride_to_u64(mqf_stride s) {
    return ((mqf_u64)s.sheet * 384 + (mqf_u64)s.symbol * 6) / 6;
}

mqf_stride mqf_stride_add(mqf_stride a, mqf_stride b) {
    mqf_stride r;
    mqf_u16 s = (mqf_u16)a.symbol + (mqf_u16)b.symbol;
    r.sheet = a.sheet + b.sheet + (mqf_u32)(s >> 6);
    r.symbol = (mqf_u8)(s & 0x3F);
    return r;
}

mqf_stride mqf_stride_mul(mqf_stride a, mqf_stride b) {
    mqf_u64 na = mqf_stride_to_u64(a);
    mqf_u64 nb = mqf_stride_to_u64(b);
    mqf_u64 product;
    if (na != 0 && nb > 0xFFFFFFFFFFFFFFFFULL / na)
        product = 0xFFFFFFFFFFFFFFFFULL;
    else
        product = na * nb;
    return mqf_stride_from_u64(product);
}

mqf_i32 mqf_stride_compare(mqf_stride a, mqf_stride b) {
    if (a.sheet < b.sheet) return -1;
    if (a.sheet > b.sheet) return 1;
    if (a.symbol < b.symbol) return -1;
    if (a.symbol > b.symbol) return 1;
    return 0;
}

mqf_u64 mqf_stride_absolute(mqf_stride s) {
    return (mqf_u64)s.sheet * 24 + (mqf_u64)s.symbol * 6;
}

mqf_u32 mqf_symbol_angle(mqf_u8 sym) {
    if (sym >= 64) return 0;
    return (mqf_u32)(sym * MQF_PER_SYMBOL);
}

mqf_u32 mqf_arc_distance(mqf_u32 a, mqf_u32 b) {
    mqf_i32 d = (mqf_i32)(b - a);
    if (d < 0) d = -d;
    if (d > MQF_FULL_CIRCLE / 2) d = MQF_FULL_CIRCLE - d;
    return (mqf_u32)d;
}

mqf_u8 mqf_time_stage_depth(mqf_u8 n) {
    if (n == 0) return 0;
    mqf_u8 depth = 0;
    mqf_u8 cur = n;
    while (cur > 1) {
        mqf_u8 spf = 2;
        while (cur % spf != 0) spf++;
        if (spf > cur) break;
        cur /= spf;
        depth++;
        if (depth >= 8) break;
    }
    return depth;
}

mqf_i32 mqf_is_adjacent(mqf_u8 x, mqf_u8 y, mqf_u8 z,
                        mqf_u8 ax, mqf_u8 ay, mqf_u8 az) {
    mqf_i32 dx = (mqf_i32)x - (mqf_i32)ax;
    mqf_i32 dy = (mqf_i32)y - (mqf_i32)ay;
    mqf_i32 dz = (mqf_i32)z - (mqf_i32)az;
    if (dx < 0) dx = -dx;
    if (dy < 0) dy = -dy;
    if (dz < 0) dz = -dz;
    if ((dx + dy + dz) != 1) return 0;
    if (x > 3 || y > 3 || z > 3) return 0;
    if (ax > 3 || ay > 3 || az > 3) return 0;
    return 1;
}

mqf_ret mqf_boot(void) {
    return 0;
}

// ══════════════════════════════════════════════════════════════════
// FQMCP 管线 WASM 导出
// ══════════════════════════════════════════════════════════════════

static const char STAR_ON[4]  = "\xE2\x98\x86";
static const char STAR_OFF[4] = "\xE2\x98\x85";

static char last_result[8192];
static mqf_u32 last_len = 0;

static void mqf_memcpy(char *dst, const char *src, mqf_u32 n) {
    for (mqf_u32 i = 0; i < n; i++) dst[i] = src[i];
}

void __attribute__((export_name("fqmcp_init"))) fqmcp_init(void) {
    last_len = 0;
}

const char* __attribute__((export_name("fqmcp_unary"))) fqmcp_unary(mqf_u64 v) {
    if (v == 0) {
        mqf_memcpy(last_result, STAR_ON, 3);
        last_result[3] = 0;
        last_len = 3;
        return last_result;
    }
    mqf_u32 idx = 0;
    mqf_u64 n = v;
    while (n > 0 && idx + 3 < sizeof(last_result)) {
        const char* sym = (n % 2) ? STAR_OFF : STAR_ON;
        mqf_memcpy(last_result + idx, sym, 3);
        idx += 3;
        n -= 1;
    }
    last_result[idx] = 0;
    last_len = idx;
    return last_result;
}

const char* __attribute__((export_name("fqmcp_knmlil"))) fqmcp_knmlil(mqf_u8 x, mqf_u8 y, mqf_u8 z) {
    mqf_u32 idx = 0;
    const char* s;
    s = x ? STAR_OFF : STAR_ON; mqf_memcpy(last_result+idx, s, 3); idx += 3;
    last_result[idx] = '|'; idx += 1;
    s = y ? STAR_OFF : STAR_ON; mqf_memcpy(last_result+idx, s, 3); idx += 3;
    last_result[idx] = '|'; idx += 1;
    s = z ? STAR_OFF : STAR_ON; mqf_memcpy(last_result+idx, s, 3); idx += 3;
    last_result[idx] = 0;
    last_len = idx;
    return last_result;
}

const char* __attribute__((export_name("fqmcp_fqmll_header"))) fqmcp_fqmll_header(void) {
    const char* h = "fqmet:knmll\n";
    mqf_u32 i = 0;
    while (h[i]) { last_result[i] = h[i]; i++; }
    last_result[i] = 0;
    last_len = i;
    return last_result;
}

mqf_u32 __attribute__((export_name("fqmcp_last_len"))) fqmcp_last_len(void) {
    return last_len;
}
