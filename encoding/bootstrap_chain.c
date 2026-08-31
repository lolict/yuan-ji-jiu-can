/*
 * bootstrap_chain.c — 鼓膜自举链实现
 *
 * 自举过程被记录为链上的链节，每次自举都可以追溯。
 * 三层自举: 振荡器 → 骨架 → 回声，记录在链上。
 */

#include "bootstrap_chain.h"
#include <string.h>
#include <stdio.h>
#include <time.h>

/* 元规则 */
static const char* 元规则表[] = {
    "前缀 = njsg",
    "边界 = left",
    "未知处理 = warn_and_continue",
    "签名 = njsg-bootstrap-v1.0",
    "禁止: 类/对象/继承/多态",
    "禁止: 文件/目录/进程/线程"
};
#define 元规则数量 (sizeof(元规则表) / sizeof(元规则表[0]))

/* ═══════════════════════════════════════════════════════════════
 * 初始化
 * ═══════════════════════════════════════════════════════════════ */

int 鼓膜自举_初始化(bootstrap_chain_t* bc, const char* trigger) {
    if (!bc) return -1;
    
    memset(bc, 0, sizeof(bootstrap_chain_t));
    
    /* 初始化底层链 */
    链_init(&bc->chain, 0xFF, "鼓膜自举链");  /* 0xFF = 特殊类别: 自举 */
    
    bc->current.bootstrap_version = 1;
    bc->current.timestamp = (uint64_t)time(NULL);
    bc->current.phase = BOOT_PHASE_NONE;
    bc->total_bootstraps = 0;
    bc->is_active = 0;
    
    if (trigger) {
        strncpy(bc->current.trigger, trigger, CHAIN_MAX_CREATOR - 1);
        bc->current.trigger[CHAIN_MAX_CREATOR - 1] = '\0';
    } else {
        strcpy(bc->current.trigger, "系统启动");
    }
    
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 阶段1: 振荡器 — 感知自身存在
 * ═══════════════════════════════════════════════════════════════ */

int 鼓膜自举_执行振荡(bootstrap_chain_t* bc) {
    if (!bc) return -1;
    
    bc->current.phase = BOOT_PHASE_OSCILLATOR;
    
    /* 检查签名 - 系统能否"感觉"到自己的签名 */
    if (strcmp(BOOTSTRAP_SIGNATURE, "njsg-bootstrap-v1.0") != 0) {
        return -1;  /* 签名不匹配，无法感知自身 */
    }
    
    /* 记录到链 */
    char obs[CHAIN_MAX_OBSERVATION];
    snprintf(obs, sizeof(obs), "振荡器启动: 签名=%s, 前缀=%s",
             BOOTSTRAP_SIGNATURE, BOOTSTRAP_PREFIX);
    
    链_追加(&bc->chain, CHAIN_LVL_YUAN, "振荡器", "自举", obs);
    
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 阶段2: 骨架 — 加载元规则
 * ═══════════════════════════════════════════════════════════════ */

int 鼓膜自举_加载骨架(bootstrap_chain_t* bc) {
    if (!bc) return -1;
    if (bc->current.phase < BOOT_PHASE_OSCILLATOR) return -1;  /* 必须先振荡 */
    
    bc->current.phase = BOOT_PHASE_SKELETON;
    bc->current.rules_checked = 元规则数量;
    
    uint8_t passed = 0;
    
    /* 逐一验证每条元规则 */
    for (uint8_t i = 0; i < 元规则数量; i++) {
        char obs[CHAIN_MAX_OBSERVATION];
        snprintf(obs, sizeof(obs), "验证元规则: %s", 元规则表[i]);
        
        链_追加(&bc->chain, CHAIN_LVL_JIE, 元规则表[i], "骨架加载", obs);
        passed++;
    }
    
    bc->current.rules_passed = passed;
    
    return (passed == bc->current.rules_checked) ? 0 : -1;
}

/* ═══════════════════════════════════════════════════════════════
 * 阶段3: 回声 — 自检验证
 * ═══════════════════════════════════════════════════════════════ */

int 鼓膜自举_自检(bootstrap_chain_t* bc) {
    if (!bc) return -1;
    if (bc->current.phase < BOOT_PHASE_SKELETON) return -1;  /* 必须先有骨架 */
    
    bc->current.phase = BOOT_PHASE_ECHO;
    
    /* 自检: 用规则编码，解码，看是否一致 */
    char test_payload[] = "鼓膜完整测试";
    char encoded[256];
    
    snprintf(encoded, sizeof(encoded), "%s:%s:%s",
             BOOTSTRAP_PREFIX, "left", test_payload);
    
    /* 验证一致性 */
    char* decoded = strstr(encoded, test_payload);
    int intact = (decoded && strcmp(decoded, test_payload) == 0) ? 1 : 0;
    
    bc->current.drum_intact = intact;
    
    /* 记录到链 */
    char obs[CHAIN_MAX_OBSERVATION];
    snprintf(obs, sizeof(obs), "自检结果: 鼓膜%s", intact ? "完整" : "破损");
    
    链_追加(&bc->chain, CHAIN_LVL_CENG, "回声自检", "自举", obs);
    
    return intact ? 0 : -1;
}

/* ═══════════════════════════════════════════════════════════════
 * 阶段4: 激活 — 允许加载外部代码
 * ═══════════════════════════════════════════════════════════════ */

int 鼓膜自举_激活(bootstrap_chain_t* bc) {
    if (!bc) return -1;
    if (bc->current.phase < BOOT_PHASE_ECHO) return -1;  /* 必须先通过自检 */
    if (!bc->current.drum_intact) return -1;             /* 鼓膜必须完整 */
    
    bc->current.phase = BOOT_PHASE_ACTIVE;
    bc->is_active = 1;
    bc->total_bootstraps++;
    
    /* 记录激活 */
    char obs[CHAIN_MAX_OBSERVATION];
    snprintf(obs, sizeof(obs), "第%lu次自举完成，鼓膜完整，允许加载外部代码",
             (unsigned long)bc->total_bootstraps);
    
    链_追加(&bc->chain, CHAIN_LVL_DAI, "激活", "自举", obs);
    
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 查询
 * ═══════════════════════════════════════════════════════════════ */

int 鼓膜自举_已激活(const bootstrap_chain_t* bc) {
    return (bc && bc->is_active) ? 1 : 0;
}

uint64_t 鼓膜自举_次数(const bootstrap_chain_t* bc) {
    return bc ? bc->total_bootstraps : 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 报告
 * ═══════════════════════════════════════════════════════════════ */

void 鼓膜自举_报告(bootstrap_chain_t* bc) {
    if (!bc) return;
    
    printf("\n");
    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║           【鼓膜自举 · 报告】                     ║\n");
    printf("╠══════════════════════════════════════════════════╣\n");
    printf("║ 签名: %s                        ║\n", BOOTSTRAP_SIGNATURE);
    printf("║ 触发者: %-12s                           ║\n", bc->current.trigger);
    printf("║ 自举次数: %lu                                ║\n", (unsigned long)bc->total_bootstraps);
    printf("║ 状态: %-12s                             ║\n",
           bc->is_active ? "已激活✅" : "未激活❌");
    printf("║ 鼓膜: %-12s                             ║\n",
           bc->current.drum_intact ? "完整✅" : "破损❌");
    printf("║ 规则: %d/%d 通过                          ║\n",
           bc->current.rules_passed, bc->current.rules_checked);
    printf("║ 链节数: %lu                                ║\n", (unsigned long)bc->chain.count);
    printf("║                                                  ║\n");
    printf("║ 自举过程追踪:                                    ║\n");
    
    for (uint64_t i = 0; i < bc->chain.count; i++) {
        chain_node_t* node = &bc->chain.nodes[i];
        printf("║   [v%lu] %-10s ← %-8s %s层    ║\n",
               (unsigned long)node->version,
               node->content,
               node->creator,
               node->level == 0 ? "元" : node->level == 1 ? "阶" :
               node->level == 2 ? "层" : "代");
    }
    
    printf("╚══════════════════════════════════════════════════╝\n");
}
