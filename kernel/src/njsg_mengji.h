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
#define MENGJI_MAX_BAGS         64   /* 最大袋子数(里程碑三) */
#define MENGJI_MAX_BAG_LOGS     128  /* 最大增量日志条目数 */
#define MENGJI_BAG_WEIGHT_MIN    1   /* 袋子权值下限 */
#define MENGJI_BAG_WEIGHT_MAX  256   /* 袋子权值上限 */

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

/* ---- 袋子操作类型 (里程碑三 · v1.6) ---- */
typedef enum {
    MJ_BAG_MERGE    = 0,  /* 加法合并 */
    MJ_BAG_SPLIT    = 1,  /* 减法拆分 */
    MJ_BAG_LOG      = 2,  /* 增量日志 */
    MJ_BAG_VALIDATE = 3   /* 袋子验证 */
} mengji_bag_op_t;

/* ---- 袋子条目 ---- */
typedef struct {
    int                 bag_id;             /* 袋号 */
    int                 weight;             /* 权值(整袋份额) */
    char                witness_hash[17];   /* 组合见证摘要(16进制64位) */
    int                 parent_id;          /* 父袋号(-1=根袋子) */
    int                 source_a;           /* 合并源袋A(-1=非合并来源) */
    int                 source_b;           /* 合并源袋B(-1=非合并来源) */
    int                 triune_slot;        /* 关联的三元见证槽位 */
    time_t              created_at;         /* 创建时间戳 */
    mengji_anchor_t     anchor;             /* 夫妻主权人锚点归属 */
} mengji_bag_entry_t;

/* ---- 袋子增量日志条目 ---- */
typedef struct {
    int                 log_id;             /* 日志序号 */
    mengji_bag_op_t     op;                 /* MERGE/SPLIT/LOG/VALIDATE */
    int                 source_bag;         /* 源袋号(合并A/拆分父袋子) */
    int                 auxiliary_bag;      /* 辅助袋号(合并B) */
    int                 result_bag;         /* 结果袋号(合并C/拆分子袋子) */
    int                 weight_delta;       /* 权值变化量(+加-减) */
    char                result_hash[17];    /* 结果见证摘要 */
    int                 witness_node;       /* 见证节点 */
    time_t              timestamp;          /* 时间戳 */
} mengji_bag_log_entry_t;

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
    mengji_bag_entry_t      bags[MENGJI_MAX_BAGS];
    int                     bag_count;
    mengji_bag_log_entry_t  bag_logs[MENGJI_MAX_BAG_LOGS];
    int                     bag_log_count;
    int                     bag_total_weight;
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

/* 里程碑一 · 种子场握手: 通过种子节点自动发现邻居 */
int  njsg_mengji_discover_from_seed(int node_idx, int seed_idx);

/* 里程碑一 · 离线恢复完整测试场景 */
void njsg_mengji_scenario_M15_offline_recovery(void);

/* 打印 */
void njsg_mengji_print(void);
void njsg_mengji_print_node(int idx);

/* ---- 里程碑三 · 跨袋子合并拆分 -------------------------------- */

/* 加法合并: bag_a + bag_b → 新袋子(权值累加,见证摘要组合) */
int  njsg_mengji_bag_merge(int bag_a, int bag_b, const char* merge_note);

/* 减法拆分: parent_bag → parent_bag' + child_bag(子袋承接部分权值与见证) */
int  njsg_mengji_bag_split(int parent_bag, int sub_weight, const char* split_note);

/* 产出增量日志条目(作为Gossip裁决袋内容) */
int  njsg_mengji_bag_incremental_log(mengji_bag_op_t op,
                                      int source, int auxiliary, int result,
                                      int weight_delta, const char* result_hash,
                                      int witness_node);

/* 验证袋子合规(袋号/权值/见证链) */
int  njsg_mengji_bag_validate(int bag_id);

/* 验证袋子全集守恒(所有袋子权值总和一致性) */
int  njsg_mengji_bag_validate_all(void);

/* 打印 */
void njsg_mengji_print_bag(int bag_id);
void njsg_mengji_print_bag_log(int log_id);

/* ---- 独立场景入口 (容器环境变量 MENGJI_SCENARIO 控制) ---- */

/* 按名称调度独立场景: triune / bag / offline / all */
int  njsg_mengji_run_scenario(const char* name);

/* 里程碑二 · 独立三元见证验证 */
void njsg_mengji_scenario_M16_triune_only(void);

/* 里程碑三 · 独立袋子合并拆分验证 */
void njsg_mengji_scenario_M17_bag_only(void);

/* 里程碑一 · 独立离线恢复验证 */
void njsg_mengji_scenario_M18_offline_only(void);

/* Demo */
void njsg_mengji_demo(void);

#endif /* NJSG_MENGJI_H */
