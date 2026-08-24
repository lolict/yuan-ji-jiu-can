/**
 * negative_sample.h — 负样本容器 (免疫记忆库)
 *
 * 核心: 记录被排斥的异类 · 形成免疫记忆
 *
 * 负样本 = 公约数不在其中的容器 (异类/敌对/免疫排斥)
 *
 * 三个组成部分:
 *   1. 记忆库 — 存储所有被拒绝的组合
 *   2. 回写协议 — 更新排斥准则
 *   3. 裁定器 — 判断新输入是否属于负样本
 *
 * 记忆库结构:
 *   ┌────────┬────────┬──────────┬──────────┐
 *   │ 符号A  │ 符号B  │ 拒绝原因 │ 时间戳   │
 *   │ 2字节  │ 2字节  │ 1字节    │ 4字节    │
 *   └────────┴────────┴──────────┴──────────┘
 *
 * 裁定器:
 *   输入新组合 → 查记忆库 → 已知负样本 = 立即拒绝
 *              → 未知组合 → 送审判 → 拒绝后回写记忆库
 *
 * 法条化:
 *   vA + vB == 2·v_core → 通过
 *   vA + vB != 2·v_core → 负样本 · 入记忆库
 */

#ifndef NJSG_NEGATIVE_SAMPLE_H
#define NJSG_NEGATIVE_SAMPLE_H

#include <stdint.h>
#include <stdio.h>
/* global_interlock.h 在 enum 定义之后包含 · 避免宏展开破坏枚举 */

#define NEGATIVE_SAMPLE_MAGIC       0x4E534D50  /* "NSMP" */
#define NEGATIVE_MEMORY_SIZE        254         /* 记忆库容量 */
#define NEGATIVE_MAX_REASON         16          /* 拒绝原因类型数 */

/* 拒绝原因枚举 */
typedef enum {
    REASON_NONE             = 0,
    REASON_CROSS_LEVEL      = 1,    /* 跨层级 */
    REASON_CROSS_TYPE       = 2,    /* 跨类型 */
    REASON_CROSS_ANCHOR     = 3,    /* 跨锚点 */
    REASON_AESTHETIC_FAIL   = 4,    /* 审美不通过 */
    REASON_WEAVING_FAIL     = 5,    /* 编织不通过 */
    REASON_BLACKLIST        = 6,    /* 黑名单 */
    REASON_REPEAT           = 7,    /* 重复拒绝 */
    REASON_DEGRADATION      = 8,    /* 降解产物 */
} reject_reason_t;

/* 记忆条目 */
typedef struct {
    uint32_t        symbol_a;       /* 符号A的编码 */
    uint32_t        symbol_b;       /* 符号B的编码 */
    uint8_t         reason;         /* 拒绝原因 */
    uint8_t         count;          /* 重复次数 */
    uint32_t        timestamp;      /* 首次拒绝时间 */
    uint8_t         frozen;         /* 是否冻结 (不再接受申诉) */
} negative_entry_t;

/* 回写协议 */
typedef struct {
    uint32_t        write_count;        /* 总回写次数 */
    uint32_t        frozen_count;       /* 冻结条目数 */
    uint32_t        evict_count;        /* 驱逐次数 */
    uint8_t         auto_freeze;        /* 重复多少次后冻结 */
    uint8_t         memory_full;        /* 记忆库是否已满 */
} writeback_proto_t;

/* 裁定结果 */
typedef enum {
    VERDICT_PASS        = 0,    /* 通过 */
    VERDICT_KNOWN_NEG   = 1,    /* 已知负样本 (记忆库命中) */
    VERDICT_NEW_REJECT  = 2,    /* 新拒绝 (需审判后入记忆库) */
    VERDICT_APPEAL      = 3,    /* 申诉 (要求重新审判) */
} verdict_t;

/* ── 全局互锁 (在所有 enum 定义之后 · 避免宏展开破坏枚举) ── */
#include "global_interlock.h"
#include "symbol_chain.h"

/* 负样本容器 */
typedef struct {
    uint32_t            magic;
    uint16_t            version;
    
    /* 记忆库 */
    negative_entry_t    memory[NEGATIVE_MEMORY_SIZE];
    uint16_t            memory_count;      /* 当前记忆条目数 */
    uint16_t            memory_head;       /* 写入位置 (循环缓冲) */
    
    /* 回写协议 */
    writeback_proto_t   protocol;
    
    /* 统计 */
    uint32_t            total_rejects;     /* 总拒绝数 */
    uint32_t            total_hits;        /* 记忆库命中数 */
    uint32_t            total_misses;      /* 记忆库未命中 */
    
    /* v_core 锚点 (产权核心值) */
    uint32_t            v_core;            /* 公约数核心价值 */
    uint32_t            tolerance;         /* 容差范围 */
} negative_sample_t;

/* ── API ───────────────────────────────────────────────────── */

void        negative_init(negative_sample_t *ns);
void        negative_set_core(negative_sample_t *ns, uint32_t v_core, uint32_t tol);

/* 裁定: 判断一对符号是否负样本 */
verdict_t   negative_judge(negative_sample_t *ns,
                            uint32_t sym_a, uint32_t sym_b,
                            reject_reason_t *reason_out);

/* 回写: 将新拒绝加入记忆库 */
int         negative_writeback(negative_sample_t *ns,
                                uint32_t sym_a, uint32_t sym_b,
                                reject_reason_t reason);

/* 查记忆库 */
int         negative_lookup(const negative_sample_t *ns,
                             uint32_t sym_a, uint32_t sym_b,
                             reject_reason_t *reason_out);

/* 冻结条目 */
int         negative_freeze(negative_sample_t *ns, uint16_t idx);

/* 驱逐最旧条目 */
int         negative_evict(negative_sample_t *ns);

/* 判定树: vA + vB == 2·v_core ? */
int         negative_law_judge(const negative_sample_t *ns,
                                int32_t vA, int32_t vB);

/* 统计 */
void        negative_stats(const negative_sample_t *ns, FILE *out);

#endif /* NJSG_NEGATIVE_SAMPLE_H */
