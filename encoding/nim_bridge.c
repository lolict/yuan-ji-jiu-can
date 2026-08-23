/**
 * nim_bridge.c — Nim ↔ C 桥接实现
 * 
 * C 层对外暴露桥供 Nim 调用。
 * Nim 层提供声韵调数据 + 42色 + 镜像，
 * C 层提供容器框架 + 审判系统 + 编织平衡。
 */

#include "nim_bridge.h"
#include <string.h>
#include <stdio.h>

/* ═══════════════════════════════════════════════════════════════
 * 全局状态
 * ═══════════════════════════════════════════════════════════════ */

static color_system_t g_system;          /* C 层颜色审判系统 */
static container_codec_t g_codec;        /* C 层容器框架 */
static int g_initialized = 0;

/* 已注册符号表（等待封装进容器） */
#define MAX_SYMBOLS 512
static struct {
    char name[32];
    int type;
    int level;
    float weight;
    int family;
} g_symbols[MAX_SYMBOLS];
static int g_symbol_count = 0;

/* ═══════════════════════════════════════════════════════════════
 * 桥接函数实现
 * ═══════════════════════════════════════════════════════════════ */

int bridge_init(void) {
    if (g_initialized) return 0;
    
    /* 初始化颜色审判系统 */
    color_system_init(&g_system);
    
    /* 初始化容器框架: 254容器, 每容器16字, 3层 */
    codec_init(&g_codec, FRAME_FULL, 254, 16, 3);
    
    g_symbol_count = 0;
    g_initialized = 1;
    return 0;
}

int bridge_trial(const char *name_a, const char *name_b) {
    if (!g_initialized) bridge_init();
    pair_verdict_t verdict = color_system_trial(&g_system, name_a, name_b);
    return (int)verdict;
}

const char* bridge_verdict_str(int verdict) {
    switch (verdict) {
        case PAIR_PASS:               return "PASS (通过)";
        case PAIR_REJECT_CROSS_LEVEL: return "REJECT (跨层级)";
        case PAIR_REJECT_MODIFIER:    return "REJECT (修饰零件)";
        case PAIR_REJECT_CROSS_TONE:  return "REJECT (跨基调)";
        case PAIR_REJECT_EXTRA:       return "REJECT (溢出符号)";
        case PAIR_REJECT_UNKNOWN:     return "REJECT (未分类)";
        case PAIR_REJECT_DIFFERENT:   return "REJECT (不同类型)";
        default:                      return "UNKNOWN";
    }
}

void bridge_diffract(const char *name, char *buf, int bufsize) {
    if (!g_initialized) bridge_init();
    
    diffraction_result_t result;
    color_system_diffract(&g_system, name, &result);
    
    snprintf(buf, bufsize,
             "符号:%s 出力:%.2f 受力:%.2f 失衡:%.2f",
             name, result.pressure_given, result.pressure_received,
             result.equilibrium_delta);
}

int bridge_register_symbol(const char *name, int type, int level,
                            float weight, int family) {
    if (!g_initialized) bridge_init();
    if (g_symbol_count >= MAX_SYMBOLS) return -1;
    
    /* 同时注册到容器表 */
    strncpy(g_symbols[g_symbol_count].name, name, 31);
    g_symbols[g_symbol_count].name[31] = '\0';
    g_symbols[g_symbol_count].type = type;
    g_symbols[g_symbol_count].level = level;
    g_symbols[g_symbol_count].weight = weight;
    g_symbols[g_symbol_count].family = family;
    g_symbol_count++;
    
    /* 同时注册到 type_discipline 注册表（用于审判） */
    symbol_type_t st = (symbol_type_t)type;
    symbol_level_t sl = (symbol_level_t)level;
    type_discipline_register(&g_system.police, name, st, sl);
    
    return 0;
}

int bridge_register_batch(const char **names, const int *types,
                           const int *levels, const float *weights,
                           int count) {
    int registered = 0;
    for (int i = 0; i < count; i++) {
        if (bridge_register_symbol(names[i], types[i], levels[i], weights[i], 0) == 0) {
            registered++;
        }
    }
    return registered;
}

int bridge_seal_all(void) {
    if (!g_initialized) bridge_init();
    
    /* 把已注册的符号按每16个一组封装进字框 */
    int frame_idx = 0;
    int idx = 0;
    
    while (idx < g_symbol_count) {
        /* 收集最多16个字的ID */
        uint32_t chars[16];
        int count = 0;
        
        while (count < 16 && idx < g_symbol_count) {
            /* 用符号名的哈希作为字ID */
            uint32_t hash = 0;
            const char *p = g_symbols[idx].name;
            while (*p) {
                hash = hash * 31 + (unsigned char)(*p);
                p++;
            }
            chars[count] = hash;
            count++;
            idx++;
        }
        
        /* 封装进字框（1bit） */
        if (codec_seal_chars(&g_codec, chars, (uint8_t)count) == 0) {
            frame_idx++;
        }
    }
    
    return frame_idx;
}

int bridge_render_frame(int frame_idx, int *x, int *y, int *z,
                          uint8_t *color, uint8_t *phase_left, uint8_t *phase_right) {
    if (!g_initialized) bridge_init();
    
    cylinder_coord_t coord;
    int result = codec_render_frame(&g_codec, (uint16_t)frame_idx,
                                      &coord, color, phase_left, phase_right);
    if (result == 0) {
        *x = coord.x;
        *y = coord.y;
        *z = coord.z;
    }
    return result;
}

void bridge_get_stats(char *buf, int bufsize) {
    if (!g_initialized) {
        snprintf(buf, bufsize, "系统未初始化");
        return;
    }
    
    snprintf(buf, bufsize,
             "注册符号: %d | 容器字框: %d | 总审判: %u | 通过: %u | 拒绝: %u",
             g_symbol_count, g_codec.frame_count,
             g_system.total_trials, g_system.total_passed, g_system.total_rejected);
}

void bridge_get_counts(int *total, int *passed, int *rejected) {
    if (!g_initialized) {
        *total = 0; *passed = 0; *rejected = 0;
        return;
    }
    *total = (int)g_system.total_trials;
    *passed = (int)g_system.total_passed;
    *rejected = (int)g_system.total_rejected;
}
