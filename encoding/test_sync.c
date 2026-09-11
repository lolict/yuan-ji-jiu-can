/*
 * test_sync.c — 全局符号同步引擎测试
 *
 * 验证:
 *   符号注册
 *   引用追踪
 *   同步替换 (改一处 = 改全部)
 *   跨链全量替换
 *   符号收集
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "sync_engine.h"
#include "chain_block.h"

/* 测试 1: 符号注册与查找 */
static int test_register(void) {
    printf("\n[测试1] 符号注册与查找\n");

    sync_engine_t engine;
    sync_engine_init(&engine);

    int idx1 = sync_register(&engine, "天庭", 3, 0, 0x100);
    int idx2 = sync_register(&engine, "容器", 3, 3, 0x200);
    int idx3 = sync_register(&engine, "进制", 2, 6, 0x300);

    printf("  注册 [天庭] idx=%d\n", idx1);
    printf("  注册 [容器] idx=%d\n", idx2);
    printf("  注册 [进制] idx=%d\n", idx3);

    /* 重复注册应返回已有索引 */
    int idx_dup = sync_register(&engine, "天庭", 3, 0, 0x999);
    printf("  重复注册 [天庭] idx=%d (应=%d)\n", idx_dup, idx1);

    int pass = (idx1 >= 0 && idx2 >= 0 && idx3 >= 0 && idx_dup == idx1);
    printf("  → %s\n", pass ? "PASS ✓" : "FAIL ✗");
    return pass;
}

/* 测试 2: 引用追踪 */
static int test_ref_tracking(void) {
    printf("\n[测试2] 引用追踪\n");

    sync_engine_t engine;
    sync_engine_init(&engine);

    sync_register(&engine, "天庭", 3, 0, 0x100);
    sync_register(&engine, "容器", 3, 3, 0x200);

    /* 注册引用关系 */
    sync_register_ref(&engine, "天庭", 0, 1, "声母链");
    sync_register_ref(&engine, "天庭", 4, 2, "操作链");
    sync_register_ref(&engine, "容器", 3, 5, "容器链");
    sync_register_ref(&engine, "天庭", 7, 3, "审判链");

    printf("  注册 4 条引用\n");

    /* 验证引用数 */
    int idx = -1;
    for (uint32_t i = 0; i < engine.symbol_count; i++) {
        if (strcmp(engine.symbols[i].name, "天庭") == 0) {
            idx = (int)i;
            break;
        }
    }

    int pass = (idx >= 0 && engine.symbols[idx].ref_count == 3);
    if (idx >= 0) {
        printf("  [天庭] 引用数=%u (期望:3)\n", engine.symbols[idx].ref_count);
    }
    printf("  → %s\n", pass ? "PASS ✓" : "FAIL ✗");
    return pass;
}

/* 测试 3: 同步替换 (改一处 = 改全部) */
static int test_sync_replace(void) {
    printf("\n[测试3] 同步替换\n");

    sync_engine_t engine;
    sync_engine_init(&engine);

    sync_register(&engine, "天庭", 3, 0, 0x100);
    sync_register_ref(&engine, "天庭", 0, 1, "声母链");
    sync_register_ref(&engine, "天庭", 4, 2, "操作链");
    sync_register_ref(&engine, "天庭", 7, 3, "审判链");

    printf("  替换前: [天庭] 引用=%u\n",
           engine.symbols[0].ref_count);

    /* 执行替换 */
    int affected = sync_replace(&engine, "天庭", "容器");
    printf("  sync_replace(天庭 → 容器) 影响: %d 引用\n", affected);

    int pass = (affected == 3 && strcmp(engine.symbols[0].name, "容器") == 0);
    printf("  替换后符号名: %s (期望:容器)\n", engine.symbols[0].name);
    printf("  累计替换数: %u (期望:3)\n", engine.total_replacements);
    printf("  → %s\n", pass ? "PASS ✓" : "FAIL ✗");
    return pass;
}

/* 测试 4: 跨链全量替换 */
static int test_cross_chain_replace(void) {
    printf("\n[测试4] 跨链全量替换\n");

    chain_t chains[3];
    链_init(&chains[0], 0, "声母");
    链_init(&chains[1], 4, "操作");
    链_init(&chains[2], 7, "审判");

    /* 在多条链中插入 "天庭" */
    链_追加(&chains[0], 3, "天庭", "test_sync", "测试符号");
    链_追加(&chains[0], 3, "b", "test_sync", "声母符号");
    链_追加(&chains[1], 3, "天庭", "test_sync", "测试符号");
    链_追加(&chains[1], 3, "照", "test_sync", "操作符号");
    链_追加(&chains[2], 3, "天庭", "test_sync", "测试符号");

    printf("  替换前:\n");
    for (int i = 0; i < 3; i++) {
        printf("    %s: ", chains[i].name);
        for (uint64_t j = 0; j < chains[i].count; j++) {
            printf("%s ", chains[i].nodes[j].content);
        }
        printf("\n");
    }

    /* 全量替换 */
    int total = sync_replace_all_chains(chains, 3, "天庭", "容器");
    printf("  sync_replace_all_chains(天庭 → 容器): %d 处被替换\n", total);

    printf("  替换后:\n");
    for (int i = 0; i < 3; i++) {
        printf("    %s: ", chains[i].name);
        for (uint64_t j = 0; j < chains[i].count; j++) {
            printf("%s ", chains[i].nodes[j].content);
        }
        printf("\n");
    }

    int pass = (total == 3);
    printf("  → %s\n", pass ? "PASS ✓" : "FAIL ✗");
    return pass;
}

/* 测试 5: 从链收集符号 */
static int test_collect(void) {
    printf("\n[测试5] 从链收集符号\n");

    sync_engine_t engine;
    sync_engine_init(&engine);

    chain_t chains[2];
    链_init(&chains[0], 0, "声母");
    链_init(&chains[1], 3, "容器");

    链_追加(&chains[0], 3, "b", "test", "符号");
    链_追加(&chains[0], 3, "p", "test", "符号");
    链_追加(&chains[1], 3, "袋", "test", "容器符号");
    链_追加(&chains[1], 3, "桶", "test", "容器符号");
    链_追加(&chains[1], 3, "罐", "test", "容器符号");

    int collected = sync_collect_from_chains(&engine, chains, 2);
    printf("  收集: %d 符号 (期望:5)\n", collected);
    printf("  注册表: %u 符号\n", engine.symbol_count);
    printf("  引用: %u 追踪\n", engine.ref_count);

    sync_engine_report(&engine);

    int pass = (collected == 5 && engine.symbol_count >= 5);
    printf("  → %s\n", pass ? "PASS ✓" : "FAIL ✗");
    return pass;
}

int main(void) {
    printf("╔══════════════════════════════════════╗\n");
    printf("║     全局符号同步引擎 · 测试套件       ║\n");
    printf("╚══════════════════════════════════════╝\n");

    int all_pass = 1;

    all_pass &= test_register();
    all_pass &= test_ref_tracking();
    all_pass &= test_sync_replace();
    all_pass &= test_cross_chain_replace();
    all_pass &= test_collect();

    printf("\n======================================\n");
    if (all_pass) {
        printf(" 全部测试 PASS ✓\n");
    } else {
        printf(" 存在 FAIL ✗\n");
    }
    printf("======================================\n");

    return all_pass ? 0 : 1;
}
