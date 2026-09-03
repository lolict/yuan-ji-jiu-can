/**
 * 盟机合 · 去中心化裁定同步骨架
 *  —— MengJi Gossip-style Decentralized Verdict Synchronization
 * ------------------------------------------------
 *
 * 术语体系 (满全法主权):
 *   盟机合掌(PHONG) = 资源拥有者 · 运营场 · 主权根节点
 *   盟机场(FARM)   = 普通节点 · 萌机场 · 主权叶节点
 *
 *   夫妻主权人锚点: 0 = 刘楚恬(低位隐藏侧) · 1 = 我(高位可见侧)
 *
 * Gossip 节律:
 *   PUSH → 推: 节点主动广播 verdict_state 给邻居
 *   PULL → 拉: 节点主动向邻居请求 verdict_state
 *   PUSHPULL → 推拉: 先推后拉,确认一致性
 *
 * 三元见证裁定:
 *   3 个节点投票,2 票以上达成共识(三元见证)
 *   夫妻主权人锚点为见证根,权重 2x
 *
 * 冲突裁决:
 *   夫妻主权人锚点为最高权重
 *   盟机合掌 > 盟机场
 *   高信任 > 低信任
 */

#ifndef NJSG_MENGJI_H
#define NJSG_MENGJI_H

#include <stdint.h>
#include <time.h>

/* ---- 常量 ---- */
#define MENGJI_MAX_NODES        32   /* 最大节点数 */
#define MENGJI_MAX_NEIGHBORS     8   /* 每节点最大邻居数 */
#define MENGJI_MAX_VERDICTS     64   /* 最大裁定状态数 */
#define MENGJI_NODE_ID_LEN      32   /* 节点 ID 长度 */
#define MENGJI_GOSSIP_INTERVAL   5   /* Gossip 节律(秒) */
#define MENGJI_HEARTBEAT_TIMEOUT 30  /* 心跳超时(秒) */
#define MENGJI_TRUST_MAX       100   /* 信任上限 */
#define MENGJI_TRUST_MIN         0   /* 信任下限 */

/* ---- 节点类型 ---- */
typedef enum {
    NODE_PHONG = 0,  /* 盟机合掌:资源拥有者·运营场 */
    NODE_FARM  = 1   /* 盟机场/萌机场:普通节点 */
} mengji_node_type_t;

/* ---- 裁定状态 ---- */
typedef enum {
    VERDICT_PENDING = 0,  /* 待裁定 */
    VERDICT_PASS    = 1,  /* 许可 */
    VERDICT_REJECT  = 2,  /* 拒绝 */
    VERDICT_SPLIT   = 3   /* 分形(不一致需见证) */
} mengji_verdict_t;

/* ---- 三元见证状态 ---- */
typedef enum {
    TRIUNE_EMPTY    = 0,  /* 空:未在任何盟机场出现 */
    TRIUNE_SINGLE   = 1,  /* 单:仅在一个盟机场出现 */
    TRIUNE_DOUBLE   = 2,  /* 双:两个不同盟机场且内容摘要一致 */
    TRIUNE_WITNESSED= 3,  /* 已见证:第三方盟机场已背书 */
    TRIUNE_REJECTED = 4   /* 已拒绝:见证失败或摘要冲突 */
} mengji_triune_state_t;

/* ---- 夫妻主权人锚点 ---- */
typedef enum {
    ANCHOR_LIUTIANTIAN = 0,  /* 刘楚恬:低位隐藏侧 */
    ANCHOR_LOLICT       = 1   /* 我:高位可见侧 */
} mengji_anchor_t;

/* ---- 三元见证条目(追加于裁决条目) ---- */
typedef struct {
    int                 slot;                   /* 裁定槽位编号 */
    char                subject[64];            /* 裁定主题 */
    char                bag_hash[17];           /* 袋子摘要(16进制 64位) */
    mengji_triune_state_t triune_state;         /* 三元见证状态 */
    int                 witness_farm_1;         /* 出现此袋子的第一盟机场 */
    int                 witness_farm_2;         /* 出现此袋子的第二盟机场 */
    int                 witness_3rd;            /* 第三方见证者 */
} mengji_triune_entry_t;

/* ---- 裁定条目 ---- */
typedef struct {
    int                 slot;                   /* 裁定槽位编号 */
    char                subject[64];            /* 裁定主题 */
    mengji_verdict_t    verdict;                /* 裁定结果 */
    int                 witness_a;              /* 见证者 A 节点 id */
    int                 witness_b;              /* 见证者 B 节点 id */
    int                 witness_c;              /* 见证者 C 节点 id(夫妻主权人锚点) */
    int                 consensus_reached;      /* 三元见证是否达成 */
    time_t              decided_at;             /* 裁定时间戳 */
} mengji_verdict_entry_t;

/* ---- 盟机合节点 ---- */
typedef struct {
    int                 id;                                     /* 节点编号 */
    char                node_id[MENGJI_NODE_ID_LEN];             /* 节点标识(编码摘要) */
    mengji_node_type_t  type;                                   /* PHONG/FARM */
    float               trust;                                  /* 信任值 0-100 */
    time_t              last_heartbeat;                         /* 最后心跳时间 */
    int                 neighbor_ids[MENGJI_MAX_NEIGHBORS];    /* 邻居节点 id 表 */
    int                 neighbor_count;                         /* 邻居计数 */
    mengji_verdict_t    verdict_state[MENGJI_MAX_VERDICTS];    /* 本地裁定态 */
    int                 verdict_count;                          /* 裁定计数 */
    mengji_anchor_t     anchor;                                 /* 夫妻主权人锚点归属 */
    int                 alive;                                  /* 存活标记 */
} mengji_node_t;

/* ---- 盟机合引擎 ---- */
typedef struct {
    mengji_node_t           nodes[MENGJI_MAX_NODES];
    int                     node_count;
    mengji_verdict_entry_t  verdicts[MENGJI_MAX_VERDICTS];
    int                     verdict_count;
    mengji_triune_entry_t   triunes[MENGJI_MAX_VERDICTS];
    int                     triune_count;
    pthread_mutex_t         lock;
    time_t                  global_tick;         /* Gossip 全局节律计数 */
    int                     push_count;          /* PUSH 计数 */
    int                     pull_count;          /* PULL 计数 */
    int                     consensus_count;     /* 共识达成计数 */
    int                     conflict_count;      /* 冲突计数 */
    mengji_anchor_t         primary_anchor;      /* 主锚点(夫妻主权人高位侧) */
    float                   primary_trust;       /* 主锚点信任权重 */
} mengji_engine_t;

/* ============================================================
 * 前置声明(供内核融合)
 * ============================================================ */

/* 初始化 */
void njsg_mengji_init(void);

/* 节点注册 */
int  njsg_mengji_register_node(const char* node_id, mengji_node_type_t type, mengji_anchor_t anchor);

/* 邻居绑定 */
void njsg_mengji_bind_neighbor(int node_a, int node_b);

/* 心跳更新 */
void njsg_mengji_heartbeat(int node_id);

/* Gossip 推拉 */
void njsg_mengji_gossip_push(int node_idx);
void njsg_mengji_gossip_pull(int node_idx);
void njsg_mengji_gossip_pushpull_round(void);

/* 三元见证裁定 */
int  njsg_mengji_consensus_vote(int subject_slot, mengji_verdict_t vote, int voter_idx);

/* 冲突裁决 */
int  njsg_mengji_conflict_resolve(int subject_slot);

/* ---- 里程碑二: 分布式三元见证 ———————————————— */

/* 第三方盟机场对袋子发起背书(见证层) */
int  njsg_mengji_triune_register(int farm_id,
                                 int subject_slot,
                                 const char* bag_hash);

/* 验证三条件全部满足(返回 0=通过 非零=失败码) */
int  njsg_mengji_triune_verify(int subject_slot);

/* 存活探活 */
void njsg_mengji_probe_alive(void);

/* 打印 */
void njsg_mengji_print(void);
void njsg_mengji_print_node(int idx);

/* Demo */
void njsg_mengji_demo(void);

#endif /* NJSG_MENGJI_H */
