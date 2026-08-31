/*
 * cloud_observer.h — 云观察引擎
 *
 * 系统观察自身的能力:
 *   - 观察自己由什么组成 (自观)
 *   - 观察周围环境 (外观)
 *   - 观察创造过程 (过程观)
 *   - 观察版本变迁 (历史观)
 *
 * 不是外部监控，是系统对自身的感知。
 * 就像人能观察到自己有多少组织、能观察周围、能观察自身。
 */

#ifndef NJSG_CLOUD_OBSERVER_H
#define NJSG_CLOUD_OBSERVER_H

#include "chain_block.h"
#include <stdint.h>

#define OBSERVER_MAX_CHAINS     10      /* 9 族 + 自举链 */
#define OBSERVER_NAME_LEN       32

/* 观察模式 */
#define OBSERVE_SELF            0       /* 自观 — 观察自身组成 */
#define OBSERVE_EXTERNAL        1       /* 外观 — 观察外部输入 */
#define OBSERVE_PROCESS         2       /* 过程观 — 观察创造过程 */
#define OBSERVE_HISTORY         3       /* 历史观 — 观察版本变迁 */

/* 观察者状态 */
typedef struct {
    char        name[OBSERVER_NAME_LEN];        /* 观察者名称 */
    uint8_t     mode;                           /* 当前观察模式 */
    uint64_t    observation_count;              /* 观察次数 */
    uint64_t    last_observed_version;          /* 上次观察的版本 */
    uint8_t     awareness_level;                /* 觉醒层级 0-7 */
} observer_state_t;

/* 云观察引擎 */
typedef struct {
    chain_t*            chains[OBSERVER_MAX_CHAINS];    /* 追踪的 9 条链 */
    uint8_t             chain_count;                     /* 当前链数 */
    observer_state_t    state;                           /* 观察者状态 */
    uint64_t            total_observations;              /* 总观察次数 */
} cloud_observer_t;

/* 操作接口 */
int 云观察_init(cloud_observer_t* obs, const char* name);
int 云观察_注册链(cloud_observer_t* obs, chain_t* chain);
int 云观察_自观(cloud_observer_t* obs);                      /* 观察自身组成 */
int 云观察_外观(cloud_observer_t* obs, const char* target);  /* 观察外部 */
int 云观察_过程(cloud_observer_t* obs, uint64_t version);    /* 观察创造过程 */
int 云观察_历史(cloud_observer_t* obs, uint8_t category);    /* 观察版本变迁 */
void 云观察_报告(cloud_observer_t* obs);                     /* 生成观察报告 */

#endif /* NJSG_CLOUD_OBSERVER_H */
