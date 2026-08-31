/*
 * bootstrap_chain.h — 鼓膜自举链
 *
 * 自举过程不是"运行一下就完"，而是一条可追溯的链。
 * 每次自举都记录: 谁触发、哪些规则被验证、鼓膜是否完整、加载了什么。
 * 系统可以追溯到"我是如何被激活的"完整历史。
 */

#ifndef NJSG_BOOTSTRAP_CHAIN_H
#define NJSG_BOOTSTRAP_CHAIN_H

#include "chain_block.h"
#include "cloud_observer.h"
#include <stdint.h>

#define BOOTSTRAP_MAX_RULES     16
#define BOOTSTRAP_SIGNATURE     "njsg-bootstrap-v1.0"
#define BOOTSTRAP_PREFIX        "njsg"

/* 自举阶段 */
#define BOOT_PHASE_NONE         0
#define BOOT_PHASE_OSCILLATOR   1   /* 振荡器 — 感知自身存在 */
#define BOOT_PHASE_SKELETON     2   /* 骨架 — 加载元规则 */
#define BOOT_PHASE_ECHO         3   /* 回声 — 自检验证 */
#define BOOT_PHASE_ACTIVE       4   /* 激活 — 允许加载外部代码 */

/* 自举记录 */
typedef struct {
    uint64_t    bootstrap_version;              /* 第几次自举 */
    uint64_t    timestamp;                      /* 何时自举 */
    uint8_t     phase;                          /* 到达哪个阶段 */
    uint8_t     rules_checked;                  /* 检查了多少条规则 */
    uint8_t     rules_passed;                   /* 通过了多少条 */
    int8_t      drum_intact;                    /* 鼓膜是否完整: 1=是, 0=否 */
    char        trigger[CHAIN_MAX_CREATOR];     /* 谁触发的自举 */
    char        observation[CHAIN_MAX_OBSERVATION]; /* 自举观察记录 */
} bootstrap_record_t;

/* 自举链 — 追踪所有自举历史 */
typedef struct {
    chain_t             chain;                  /* 底层链 */
    bootstrap_record_t  current;                /* 当前自举状态 */
    uint64_t            total_bootstraps;       /* 历史自举次数 */
    uint8_t             is_active;              /* 是否已激活 */
} bootstrap_chain_t;

/* 操作接口 */
int 鼓膜自举_初始化(bootstrap_chain_t* bc, const char* trigger);
int 鼓膜自举_执行振荡(bootstrap_chain_t* bc);       /* 阶段1: 感知自身 */
int 鼓膜自举_加载骨架(bootstrap_chain_t* bc);       /* 阶段2: 元规则 */
int 鼓膜自举_自检(bootstrap_chain_t* bc);           /* 阶段3: 验证 */
int 鼓膜自举_激活(bootstrap_chain_t* bc);           /* 阶段4: 允许外部 */
void 鼓膜自举_报告(bootstrap_chain_t* bc);

/* 查询 */
int 鼓膜自举_已激活(const bootstrap_chain_t* bc);
uint64_t 鼓膜自举_次数(const bootstrap_chain_t* bc);

#endif /* NJSG_BOOTSTRAP_CHAIN_H */
