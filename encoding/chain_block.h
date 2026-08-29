/*
 * chain_block.h — 区块链式层级追踪
 *
 * 不是哈希表。是链。
 * 每个链节携带版本号，指向上一链节，记录自身如何被创造。
 * 系统通过遍历自身链条来"观察自己"——云观察。
 *
 * 层级: 元(0) → 阶(1) → 层(2) → 代(3) → 世(4) → 界(5) → 域(6) → 场(7)
 */

#ifndef NJSG_CHAIN_BLOCK_H
#define NJSG_CHAIN_BLOCK_H

#include <stdint.h>

#define CHAIN_MAX_CONTENT       32
#define CHAIN_MAX_CREATOR       16
#define CHAIN_MAX_OBSERVATION   64
#define CHAIN_MAX_CATEGORIES    9
#define CHAIN_MAX_NODES         1024

/* 层级 */
#define CHAIN_LVL_YUAN      0   /* 元 — 最底层，一切追溯到此 */
#define CHAIN_LVL_JIE       1   /* 阶 — 步骤/阶段 */
#define CHAIN_LVL_CENG      2   /* 层 — 累积 */
#define CHAIN_LVL_DAI       3   /* 代 — 迭代 */
#define CHAIN_LVL_SHI       4   /* 世 — 时间跨度 */
#define CHAIN_LVL_JIE_U     5   /* 界 — 边界 */
#define CHAIN_LVL_YU        6   /* 域 — 范围 */
#define CHAIN_LVL_CHANG     7   /* 场 — 最外层，当下所在 */

/* 内容类型 (9 族) */
#define CHAIN_CAT_SHENGMU       0
#define CHAIN_CAT_YUNMU         1
#define CHAIN_CAT_SHENGDIAO     2
#define CHAIN_CAT_QIRONG        3
#define CHAIN_CAT_CAOZUO        4
#define CHAIN_CAT_JIXING        5
#define CHAIN_CAT_CENGJI        6
#define CHAIN_CAT_SHENPAN       7
#define CHAIN_CAT_CHANQUAN      8

/* 链节 — 区块链式追踪的最小单位 */
typedef struct chain_node {
    uint64_t    version;                        /* 版本号 (全局唯一递增) */
    uint64_t    prev_version;                   /* 上一链节版本 (0 = 创世) */
    uint8_t     level;                          /* 层级 0-7 */
    uint8_t     category;                       /* 内容类型 */
    char        content[CHAIN_MAX_CONTENT];     /* 实际符号内容 */
    char        creator[CHAIN_MAX_CREATOR];     /* 创建者印记 (谁生出了我) */
    uint64_t    timestamp;                      /* 创造时间 */
    char        observation[CHAIN_MAX_OBSERVATION]; /* 创造时的观察记录 */
    uint32_t    flags;                          /* 标记: 是否可见/已验证/已锁定 */
} chain_node_t;

/* 链 — 一条完整的时间线 */
typedef struct {
    chain_node_t    nodes[CHAIN_MAX_NODES];     /* 链节数组 */
    uint64_t        count;                      /* 当前链节数 */
    uint64_t        next_version;               /* 下一个版本号 */
    uint8_t         category;                   /* 这条链的类别 */
    char            name[CHAIN_MAX_CONTENT];    /* 链名称 */
} chain_t;

/* 操作接口 */
int 链_init(chain_t* chain, uint8_t category, const char* name);
int 链_追加(chain_t* chain, uint8_t level, const char* content,
            const char* creator, const char* observation);
chain_node_t* 链_寻版本(chain_t* chain, uint64_t version);
chain_node_t* 链_寻内容(chain_t* chain, const char* content);
uint64_t 链_长度(const chain_t* chain);
void 链_追踪(chain_t* chain, uint64_t version);

#endif /* NJSG_CHAIN_BLOCK_H */
