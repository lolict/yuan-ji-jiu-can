/*
 * test_smrl.c — SMRL 引擎测试
 *
 * 验证:
 *   64位单元格编码/解码
 *   几何裁判 (三元见证裁定)
 *   链 → SMRL 编码
 */

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <assert.h>
#include "smrl_engine.h"
#include "chain_block.h"

/* 测试 1: 编码/解码循环 */
static int test_encode_decode(void) {
    printf("\n[测试1] 编码/解码循环\n");

    struct {
        uint8_t coord, polarity, sovereignty, value, level, chain_id;
        uint16_t version;
    } cases[] = {
        {0, 0, 0, 0, 0, 0, 0},
        {255, 255, 255, 255, 7, 255, 65535},
        {64, 128, 200, 50, 3, 10, 1000},
        {128, 64, 0, 255, 5, 7, 42},
    };

    int passed = 0;
    for (int i = 0; i < 4; i++) {
        uint64_t code = smrl_encode(cases[i].coord, cases[i].polarity,
                                     cases[i].sovereignty, cases[i].value,
                                     cases[i].level, cases[i].chain_id,
                                     cases[i].version);
        cell_t c = smrl_decode(code);

        int ok = (c.coord == cases[i].coord &&
                  c.polarity == cases[i].polarity &&
                  c.sovereignty == cases[i].sovereignty &&
                  c.value == cases[i].value &&
                  c.level == cases[i].level &&
                  c.chain_id == cases[i].chain_id &&
                  c.version == cases[i].version);

        if (ok) {
            printf("  [%d] OK: code=0x%016lx\n", i, (unsigned long)code);
            smrl_report(code);
            passed++;
        } else {
            printf("  [%d] FAIL: 编码=0x%016lx\n", i, (unsigned long)code);
        }
    }

    printf("  → %d/4 通过\n", passed);
    return passed == 4;
}

/* 测试 2: 链 → SMRL 编码 */
static int test_chain_encoding(void) {
    printf("\n[测试2] 链 → SMRL 编码\n");

    chain_t chain;
    链_init(&chain, CHAIN_CAT_SHENGMU, "声母");

    链_追加(&chain, CHAIN_LVL_YUAN, "b", "test_smrl", "唇音·爆破");
    链_追加(&chain, CHAIN_LVL_JIE, "p", "test_smrl", "唇音·送气");
    链_追加(&chain, CHAIN_LVL_CENG, "m", "test_smrl", "唇音·鼻音");
    链_追加(&chain, CHAIN_LVL_DAI, "f", "test_smrl", "唇音·擦音");

    uint64_t codes[16];
    int count = smrl_encode_chain(&chain, codes, 16);
    printf("  链 %s: %lu 节点 → %d SMRL 码\n",
           chain.name, (unsigned long)chain.count, count);

    for (int i = 0; i < count && i < 4; i++) {
        printf("  [v%u] ", chain.nodes[i].version);
        smrl_report(codes[i]);
    }

    return count > 0;
}

/* 测试 3: 几何裁判 (三元见证) */
static int test_judgment(void) {
    printf("\n[测试3] 几何裁判 (三元见证)\n");

    /* 创建容器链 */
    chain_t chain;
    链_init(&chain, CHAIN_CAT_JIXING, "极性测试");

    /* 现有容器: 相容极性 */
    cell_t existing1 = {
        .coord = 64, .polarity = 0x55,   /* 阳极性 */
        .sovereignty = 200, .value = 100,
        .level = 3, .chain_id = 1, .version = 1
    };

    /* 候选1: 相容 (应准入) */
    cell_t cand1 = {
        .coord = 64, .polarity = 0x55,   /* 同极性 */
        .sovereignty = 50, .value = 100,
        .level = 3, .chain_id = 1, .version = 2
    };

    /* 候选2: 冲突极性 (应拒绝) */
    cell_t cand2 = {
        .coord = 64, .polarity = 0xAA,   /* 反极性 */
        .sovereignty = 50, .value = 100,
        .level = 3, .chain_id = 1, .version = 3
    };

    /* 候选3: 低主权越权 (应拒绝) */
    cell_t cand3 = {
        .coord = 64, .polarity = 0x55,   /* 同极性 */
        .sovereignty = 200, .value = 100, /* 主权不低反而高 */
        .level = 3, .chain_id = 1, .version = 4
    };

    int result1 = smrl_judge(cand1, existing1);
    int result2 = smrl_judge(cand2, existing1);
    int result3 = smrl_judge(cand3, existing1);

    printf("  候选1 (同极性): %s (期望:准入)\n",
           result1 ? "准入" : "拒绝");
    printf("  候选2 (反极性): %s (期望:拒绝)\n",
           result2 ? "准入" : "拒绝");
    printf("  候选3 (主权越高): %s (期望:拒绝)\n",
           result3 ? "准入" : "拒绝");

    return result1 && !result2 && !result3;
}

/* 测试 4: 层级兼容 */
static int test_level_compatibility(void) {
    printf("\n[测试4] 层级兼容\n");

    cell_t lower = {
        .coord = 32, .polarity = 0x55,
        .sovereignty = 100, .value = 50,
        .level = 2, .chain_id = 1, .version = 1
    };

    cell_t same_level = {
        .coord = 32, .polarity = 0x55,
        .sovereignty = 50, .value = 50,
        .level = 2, .chain_id = 1, .version = 2
    };

    cell_t higher_level = {
        .coord = 32, .polarity = 0x55,
        .sovereignty = 50, .value = 50,
        .level = 3, .chain_id = 1, .version = 3
    };

    int result_same = smrl_judge(same_level, lower);
    int result_higher = smrl_judge(higher_level, lower);

    printf("  同层级 (界→界): %s (期望:准入)\n",
           result_same ? "准入" : "拒绝");
    printf("  高层级 (界→世): %s (期望:准入)\n",
           result_higher ? "准入" : "拒绝");

    return result_same && result_higher;
}

int main(void) {
    printf("╔══════════════════════════════════════╗\n");
    printf("║       SMRL 引擎 · 测试套件            ║\n");
    printf("╚══════════════════════════════════════╝\n");

    int all_pass = 1;

    all_pass &= test_encode_decode();
    all_pass &= test_chain_encoding();
    all_pass &= test_judgment();
    all_pass &= test_level_compatibility();

    printf("\n======================================\n");
    if (all_pass) {
        printf(" 全部测试 PASS ✓\n");
    } else {
        printf(" 存在 FAIL ✗\n");
    }
    printf("======================================\n");

    return all_pass ? 0 : 1;
}
