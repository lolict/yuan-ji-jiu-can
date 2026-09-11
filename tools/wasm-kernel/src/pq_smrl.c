/* ============================================================
 * pq_smrl.c — SMRL 引擎实现
 * 同一份源码, 同时编译 wasm32 / wasm64 / native
 * ============================================================ */

#include "mqf_smrl.h"

/* 跨架构兼容的熵池 */
static uint64_t smrl_entropy = 0x9E3779B97F4A7C15ULL; /* 黄金比例 */

/* xorshift64* 伪随机 */
static inline uint64_t smrl_xorshift(void) {
    uint64_t x = smrl_entropy;
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    smrl_entropy = x;
    return x * 0x2545F4914F6CDD1DULL;
}

/* 单元格哈希 (跨架构一致) */
uint64_t mqf_smrl_cell_hash(MqfCell c) {
    uint64_t h = c.cell ^ 0x9E3779B97F4A7C15ULL;
    h ^= h >> 33;
    h *= 0xFF51AFD7ED558CCDULL;
    h ^= h >> 33;
    h *= 0xC4CEB9FE1A85EC53ULL;
    h ^= h >> 33;
    return h;
}

/* 裁决引擎原生入口 */
MqfCell mqf_smrl_builtin_admit(
    MqfCell agent, MqfCell space, uint64_t rule)
{
    MqfCell result = agent;
    MqfWitness w = mqf_smrl_admit(agent, space, rule);
    result.cell &= ~MQF_CELL_MASK_WITNESS;
    result.cell |= ((uint64_t)w << 36) & MQF_CELL_MASK_WITNESS;
    return result;
}

/* 交互函数 (供 JS/WASM 导入) */
#ifdef __EMSCRIPTEN__
#include <emscripten.h>
EMSCRIPTEN_KEEPALIVE
#endif
MqfCell* mqf_smrl_create_cell(
    uint64_t yuan, uint64_t x, uint64_t y, uint64_t z,
    uint64_t polar, uint64_t sovereign, uint64_t witness, uint64_t value)
{
    static MqfCell cell;
    cell = mqf_cell_encode(yuan, x, y, z, polar, sovereign, witness, value);
    return &cell;
}

#ifdef __EMSCRIPTEN__
EMSCRIPTEN_KEEPALIVE
#endif
uint64_t mqf_smrl_cell_extract(MqfCell* c, uint64_t field) {
    switch (field) {
        case 0: return mqf_cell_get_yuan(*c);
        case 1: return mqf_cell_get_x(*c);
        case 2: return mqf_cell_get_y(*c);
        case 3: return mqf_cell_get_z(*c);
        case 4: return mqf_cell_get_polar(*c);
        case 5: return mqf_cell_get_sovereign(*c);
        case 6: return mqf_cell_get_witness(*c);
        case 7: return mqf_cell_get_value(*c);
        default: return 0;
    }
}
