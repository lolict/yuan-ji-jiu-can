/*
 * sync_engine.c — 全局符号同步引擎
 *
 * 全量空间同步替换:
 *   改一处 = 改全部 · 跨链 · 跨文件 · 跨模块
 *
 * 核心机制:
 *   符号注册表 → 引用追踪 → 同步替换
 *   替换后所有引用点自动指向新符号
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "sync_engine.h"

/* ═══════════════════════════════════════════════════
 * 内部辅助函数
 * ═══════════════════════════════════════════════════ */

static sync_symbol_t* find_symbol(sync_engine_t* engine, const char* name) {
    if (!engine || !name) return NULL;
    for (uint32_t i = 0; i < engine->symbol_count; i++) {
        if (strcmp(engine->symbols[i].name, name) == 0) {
            return &engine->symbols[i];
        }
    }
    return NULL;
}

static int find_symbol_idx(sync_engine_t* engine, const char* name) {
    if (!engine || !name) return -1;
    for (uint32_t i = 0; i < engine->symbol_count; i++) {
        if (strcmp(engine->symbols[i].name, name) == 0) {
            return (int)i;
        }
    }
    return -1;
}

/* ═══════════════════════════════════════════════════
 * API 实现
 * ═══════════════════════════════════════════════════ */

int sync_engine_init(sync_engine_t* engine) {
    if (!engine) return -1;
    memset(engine, 0, sizeof(sync_engine_t));
    engine->magic = SYNC_MAGIC;
    engine->symbol_count = 0;
    engine->ref_count = 0;
    engine->total_syncs = 0;
    engine->total_replacements = 0;
    return 0;
}

int sync_register(sync_engine_t* engine,
                  const char* name, uint8_t level, uint8_t category,
                  uint64_t smrl_code) {
    if (!engine || !name) return -1;
    if (engine->symbol_count >= SYNC_MAX_SYMBOLS) return -1;

    /* 已存在则更新 */
    int idx = find_symbol_idx(engine, name);
    if (idx >= 0) {
        engine->symbols[idx].smrl_code = smrl_code;
        engine->symbols[idx].level = level;
        engine->symbols[idx].category = category;
        return idx;
    }

    /* 新建 */
    sync_symbol_t* sym = &engine->symbols[engine->symbol_count];
    strncpy(sym->name, name, SYNC_MAX_NAME - 1);
    sym->name[SYNC_MAX_NAME - 1] = '\0';
    sym->replacement[0] = '\0';
    sym->smrl_code = smrl_code;
    sym->level = level;
    sym->category = category;
    sym->ref_count = 0;
    sym->flags = 0;

    return (int)engine->symbol_count++;
}

int sync_register_ref(sync_engine_t* engine,
                      const char* symbol_name,
                      uint32_t chain_id, uint32_t version,
                      const char* context) {
    if (!engine || !symbol_name) return -1;
    if (engine->ref_count >= SYNC_MAX_REFS) return -1;

    /* 找到对应符号 */
    int sym_idx = find_symbol_idx(engine, symbol_name);
    if (sym_idx < 0) {
        /* 自动注册未知符号 */
        sym_idx = sync_register(engine, symbol_name, 0, 0, 0);
        if (sym_idx < 0) return -1;
    }

    /* 添加引用 */
    sync_ref_t* ref = &engine->refs[engine->ref_count];
    ref->symbol_idx = (uint32_t)sym_idx;
    ref->chain_id = chain_id;
    ref->node_version = version;
    if (context) {
        strncpy(ref->context, context, SYNC_MAX_NAME - 1);
        ref->context[SYNC_MAX_NAME - 1] = '\0';
    } else {
        ref->context[0] = '\0';
    }

    engine->symbols[sym_idx].ref_count++;
    engine->ref_count++;

    return 0;
}

int sync_replace(sync_engine_t* engine,
                 const char* old_name, const char* new_name) {
    if (!engine || !old_name || !new_name) return -1;

    sync_symbol_t* sym = find_symbol(engine, old_name);
    if (!sym) return -1;

    /* 更新符号名 */
    strncpy(sym->name, new_name, SYNC_MAX_NAME - 1);
    sym->name[SYNC_MAX_NAME - 1] = '\0';
    sym->flags |= 0x01;  /* 已替换标记 */

    engine->total_syncs++;
    engine->total_replacements += sym->ref_count;

    return (int)sym->ref_count;
}

int sync_replace_in_chain(chain_t* chain,
                          const char* old_content, const char* new_content) {
    if (!chain || !old_content || !new_content) return -1;

    int replaced = 0;
    for (uint64_t i = 0; i < chain->count; i++) {
        if (strcmp(chain->nodes[i].content, old_content) == 0) {
            strncpy(chain->nodes[i].content, new_content, CHAIN_MAX_CONTENT - 1);
            chain->nodes[i].content[CHAIN_MAX_CONTENT - 1] = '\0';
            replaced++;
        }
    }
    return replaced;
}

int sync_replace_all_chains(chain_t* chains, uint32_t chain_count,
                            const char* old_content, const char* new_content) {
    if (!chains || !old_content || !new_content) return -1;

    int total = 0;
    for (uint32_t c = 0; c < chain_count; c++) {
        int n = sync_replace_in_chain(&chains[c], old_content, new_content);
        if (n > 0) {
            total += n;
        }
    }
    return total;
}

int sync_collect_from_chains(sync_engine_t* engine,
                             chain_t* chains, uint32_t chain_count) {
    if (!engine || !chains) return -1;

    int collected = 0;
    for (uint32_t c = 0; c < chain_count; c++) {
        for (uint64_t i = 0; i < chains[c].count; i++) {
            int idx = sync_register(engine,
                                    chains[c].nodes[i].content,
                                    chains[c].nodes[i].level,
                                    chains[c].category,
                                    0);
            if (idx >= 0) {
                sync_register_ref(engine,
                                  chains[c].nodes[i].content,
                                  c,
                                  chains[c].nodes[i].version,
                                  chains[c].name);
                collected++;
            }
        }
    }
    return collected;
}

void sync_engine_report(const sync_engine_t* engine) {
    if (!engine) return;
    printf("\n");
    printf("  ┌─ 符号同步引擎 ──────────────────────┐\n");
    printf("  │ 注册表: %u/%u 符号                │\n",
           engine->symbol_count, (unsigned int)SYNC_MAX_SYMBOLS);
    printf("  │ 引用追踪: %u/%u 引用              │\n",
           engine->ref_count, (unsigned int)SYNC_MAX_REFS);
    printf("  │ 累计同步: %u 次                     │\n", engine->total_syncs);
    printf("  │ 累计替换: %u 引用                   │\n", engine->total_replacements);
    if (engine->symbol_count > 0) {
        printf("  │\n");
        printf("  │ 注册表 (前10):\n");
        uint32_t limit = engine->symbol_count < 10 ? engine->symbol_count : 10;
        for (uint32_t i = 0; i < limit; i++) {
            const sync_symbol_t* s = &engine->symbols[i];
            printf("  │   [%s] L%u C%u 引用:%u%s\n",
                   s->name, (unsigned int)s->level, (unsigned int)s->category,
                   s->ref_count, (s->flags & 0x01) ? " [已替换]" : "");
        }
    }
    printf("  └────────────────────────────────────┘\n");
}
