/*
 * smrl_engine.c — SMRL 引擎
 *
 * 64位单元格: 坐标 · 极性 · 主权 · 价值
 * 位分配:
 *   [63:56] 坐标 (256 唯一位置 = 静态绝对地址)
 *   [55:48] 极性 (阴/阳/盈/亏/虚/实/表/里/...)
 *   [47:40] 主权 (0=无主, 255=全主)
 *   [39:32] 价值 (0-255 量化值)
 *   [31:24] 层级 (元/阶/层/代/世/界/域/场)
 *   [23:16] 链ID
 *   [15:8]  版本高位
 *   [7:0]   版本低位 → 65536 版本/链
 *
 * 几何裁判: 三元见证 → 准入裁定
 *   三个独立维度同时满足 = 准入
 */

#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include <time.h>
#include "smrl_engine.h"

/* ═══════════════════════════════════════════════════
 * 64位编码/解码
 * ═══════════════════════════════════════════════════ */

uint64_t smrl_encode(uint8_t coord, uint8_t polarity, uint8_t sovereignty,
                     uint8_t value, uint8_t level, uint8_t chain_id,
                     uint16_t version) {
    return ((uint64_t)coord    << 56) |
           ((uint64_t)polarity << 48) |
           ((uint64_t)sovereignty << 40) |
           ((uint64_t)value    << 32) |
           ((uint64_t)level    << 24) |
           ((uint64_t)chain_id << 16) |
           (uint64_t)version;
}

cell_t smrl_decode(uint64_t code) {
    cell_t c;
    c.coord       = (uint8_t)(code >> 56);
    c.polarity    = (uint8_t)(code >> 48);
    c.sovereignty = (uint8_t)(code >> 40);
    c.value       = (uint8_t)(code >> 32);
    c.level       = (uint8_t)(code >> 24);
    c.chain_id    = (uint8_t)(code >> 16);
    c.version     = (uint16_t)(code & 0xFFFF);
    return c;
}

/* ═══════════════════════════════════════════════════
 * 几何裁判: 三元见证 + 准入裁定
 * ═══════════════════════════════════════════════════ */

int smrl_judge(cell_t candidate, cell_t existing) {
    /* 三元见证: 极性相容 + 层级兼容 + 主权不侵 */
    int polarity_ok   = (candidate.polarity & existing.polarity) != 0 ||
                       (candidate.polarity == 0) || (existing.polarity == 0);
    int level_ok     = (candidate.level >= existing.level);  /* 不降级侵入 */
    int sovereignty_ok = (candidate.sovereignty == 0) || (existing.sovereignty == 0) ||
                        (candidate.sovereignty < existing.sovereignty); /* 严格低于 */

    /* 三裁全部通过 = 准入 */
    return polarity_ok && level_ok && sovereignty_ok;
}

/* ═══════════════════════════════════════════════════
 * 链 → SMRL 编码
 * ═══════════════════════════════════════════════════ */

int smrl_encode_chain(const chain_t* chain, uint64_t* codes, int max_codes) {
    if (!chain || !codes) return -1;
    
    int n = (int)chain->count;
    if (n > max_codes) n = max_codes;
    
    for (int i = 0; i < n; i++) {
        const chain_node_t* node = &chain->nodes[i];
        
        uint8_t coord = (uint8_t)(i % 256);
        uint8_t polarity = (uint8_t)(node->level * 32);  /* 层级映射到极性 */
        uint8_t sovereignty = (node->version == 1) ? 255 : 128; /* 根节点=全主 */
        uint8_t value = (uint8_t)((i * 7 + 13) % 256);   /* 简单哈希 */
        
        codes[i] = smrl_encode(coord, polarity, sovereignty, value,
                               node->level, (uint8_t)chain->category,
                               (uint16_t)node->version);
    }
    
    return n;
}

void smrl_report(uint64_t code) {
    cell_t c = smrl_decode(code);
    const char* lvl[] = {"元","阶","层","代","世","界","域","场"};
    
    printf("  SMRL[%016lx] ", (unsigned long)code);
    printf("坐标:%u 极性:%u 主权:%u 价值:%u %s 链#%u v%u\n",
           c.coord, c.polarity, c.sovereignty, c.value,
           lvl[c.level < 8 ? c.level : 7], c.chain_id, c.version);
}

void smrl_engine_report(void) {
    printf("\n");
    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║              SMRL 引 擎 报 告                  ║\n");
    printf("╠══════════════════════════════════════════════════╣\n");
    printf("║ 64位单元格: [坐标|极性|主权|价值|层级|链|版本] ║\n");
    printf("║ 几何裁判: 极性相容 · 层级兼容 · 主权不侵     ║\n");
    printf("║ 三元同时满足 = 准入                           ║\n");
    printf("╚══════════════════════════════════════════════════╝\n");
}
