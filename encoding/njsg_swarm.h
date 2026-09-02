/*
 * njsg_swarm.h — 囡囝共生 分布式智能体框架
 *
 * 本体-分身架构:
 *   协调器 (Orchestrator) — 任务分配、冲突裁决、链式追踪
 *   子代理 (SubAgents)    — 并行执行，各有专能
 *
 * 子代理类型:
 *   bootstrap-agent    — 鼓膜自举，序列化器-解释器-裁判器三元组
 *   meta-ime-agent     — 元输入法，本地智能规则引擎
 *   five-yuan-agent    — 五元仲裁，五维审判
 *   seven-yuan-agent   — 七元演化，版本分叉仲裁
 *   yinyang-agent      — 阴阳正向/逆向映射
 *
 * 所有Agent的行动都写入全局任务链，确保可追溯。
 * 所有通信都写入消息链，确保可审计。
 * 冲突由协调器裁决，裁决结果写入审判链。
 */

#ifndef NJSG_SWARM_H
#define NJSG_SWARM_H

#include <stdint.h>
#include <stddef.h>
#include "chain_block.h"

/* ═══════════════════════════════════════════════════
 * 消息系统
 * ═══════════════════════════════════════════════════ */

/* 消息类型 */
typedef enum {
    MSG_TASK_ASSIGN     = 0x10,  /* 协调器 → Agent: 分配任务 */
    MSG_TASK_REPORT     = 0x11,  /* Agent → 协调器: 任务报告 */
    MSG_CONFLICT        = 0x20,  /* Agent → 协调器: 冲突上报 */
    MSG_ARBITRATION     = 0x21,  /* 协调器 → Agent: 裁决胜者 */
    MSG_HEARTBEAT       = 0x30,  /* Agent → 协调器: 存活信号 */
    MSG_SYNC            = 0x40,  /* 协调器 → All: 状态同步 */
    MSG_PROTEST         = 0x50,  /* Agent → 协调器: 抗议向量 */
    MSG_REWARD          = 0x60,  /* 协调器 → Agent: 奖励分发 */
} swarm_msg_type_t;

/* 消息优先级 */
typedef enum {
    PRIO_LOW    = 0,
    PRIO_NORMAL = 1,
    PRIO_HIGH   = 2,
    PRIO_URGENT = 3,
} swarm_priority_t;

/* 消息结构 */
#define SWARM_MSG_CONTENT   256
#define SWARM_ID_LEN        32

typedef struct {
    uint64_t            version;            /* 链式版本 */
    uint64_t            prev_version;       /* 上一版本 */
    swarm_msg_type_t    type;               /* 消息类型 */
    swarm_priority_t    priority;           /* 优先级 */
    char                from[SWARM_ID_LEN]; /* 发送者 */
    char                to[SWARM_ID_LEN];   /* 接收者 */
    char                topic[64];          /* 主题/任务ID */
    char                content[SWARM_MSG_CONTENT]; /* 载荷 */
    uint64_t            timestamp;          /* 时间戳 */
    uint32_t            seq;                /* 序列号（防重放） */
} swarm_message_t;

/* 消息链 */
#define SWARM_MAX_MESSAGES  4096

typedef struct {
    swarm_message_t     messages[SWARM_MAX_MESSAGES];
    uint64_t            count;
    uint64_t            next_version;
    uint32_t            next_seq;
} swarm_msg_chain_t;

/* 消息链操作 */
void    消息链_init(swarm_msg_chain_t* mc);
int     消息链_投递(swarm_msg_chain_t* mc, swarm_msg_type_t type,
                    const char* from, const char* to,
                    const char* topic, const char* content,
                    swarm_priority_t prio);
swarm_message_t* 消息链_查找(swarm_msg_chain_t* mc, const char* topic, swarm_msg_type_t type);
uint64_t 消息链_计数(const swarm_msg_chain_t* mc);

/* ═══════════════════════════════════════════════════
 * 任务系统
 * ═══════════════════════════════════════════════════ */

/* 任务状态 */
typedef enum {
    TASK_PENDING    = 0,  /* 待分配 */
    TASK_ASSIGNED   = 1,  /* 已分配 */
    TASK_RUNNING    = 2,  /* 执行中 */
    TASK_COMPLETED  = 3,  /* 已完成 */
    TASK_FAILED     = 4,  /* 失败 */
    TASK_CONFLICT   = 5,  /* 冲突待裁决 */
    TASK_BLOCKED    = 6,  /* 阻塞 */
} task_status_t;

/* 任务 */
#define TASK_ID_LEN     64
#define TASK_DESC_LEN   256

typedef struct {
    uint64_t        chain_version;          /* 写入链时的版本 */
    char            task_id[TASK_ID_LEN];
    char            desc[TASK_DESC_LEN];
    task_status_t   status;
    char            assignee[SWARM_ID_LEN]; /* 分配给哪个Agent */
    uint64_t        create_time;
    uint64_t        start_time;
    uint64_t        finish_time;
    uint32_t        retry_count;
    uint32_t        reward;                 /* 完成奖励 */
    uint8_t         conflict_flag;          /* 是否有冲突 */
} swarm_task_t;

/* 任务链 */
#define SWARM_MAX_TASKS     2048

typedef struct {
    swarm_task_t        tasks[SWARM_MAX_TASKS];
    uint64_t            count;
    chain_t*            trace_chain;        /* 追踪链 */
} swarm_task_chain_t;

/* 任务链操作 */
void    任务链_init(swarm_task_chain_t* tc, chain_t* trace);
int     任务链_创建(swarm_task_chain_t* tc, const char* id, const char* desc);
int     任务链_分配(swarm_task_chain_t* tc, const char* task_id, const char* agent_id);
int     任务链_更新状态(swarm_task_chain_t* tc, const char* task_id, task_status_t status);
int     任务链_标记冲突(swarm_task_chain_t* tc, const char* task_id);
swarm_task_t* 任务链_查找(swarm_task_chain_t* tc, const char* task_id);

/* ═══════════════════════════════════════════════════
 * Agent 接口
 * ═══════════════════════════════════════════════════ */

/* 前向声明 */
struct swarm_agent;
struct swarm_coordinator;

/* Agent行为接口（虚函数表） */
typedef struct {
    int (*on_register)(struct swarm_agent* self);       /* 注册到协调器 */
    int (*on_task)(struct swarm_agent* self, const swarm_task_t* task); /* 执行任务 */
    int (*on_message)(struct swarm_agent* self, const swarm_message_t* msg); /* 收消息 */
    int (*on_heartbeat)(struct swarm_agent* self);      /* 心跳响应 */
    int (*on_arbitration)(struct swarm_agent* self, const char* loser); /* 被裁决失败 */
} swarm_agent_vtable_t;

/* Agent实体 */
typedef struct swarm_agent {
    char                id[SWARM_ID_LEN];       /* 名称标识 */
    uint32_t            health;                 /* 健康度 0-100 */
    uint32_t            task_count;             /* 已完成任务数 */
    uint32_t            reward_score;           /* 累计奖励 */
    uint8_t             active;                 /* 是否活跃 */
    swarm_agent_vtable_t vtable;                /* 行为 */
    struct swarm_coordinator* coordinator;      /* 所属协调器 */
    void*               priv;                   /* 私有数据 */
} swarm_agent_t;

/* ═══════════════════════════════════════════════════
 * 协调器
 * ═══════════════════════════════════════════════════ */

#define SWARM_MAX_AGENTS    16

typedef struct swarm_coordinator {
    char                name[SWARM_ID_LEN];
    uint32_t            round;                  /* 当前轮次 */
    
    /* 子系统 */
    swarm_msg_chain_t   messages;
    swarm_task_chain_t  tasks;
    
    /* 子代理 */
    swarm_agent_t*      agents[SWARM_MAX_AGENTS];
    uint32_t            agent_count;
    
    /* 追踪链 */
    chain_t             action_chain;           /* 所有行动记录 */
    chain_t             arbitration_chain;      /* 裁判链 */
    
    /* 运行状态 */
    uint8_t             running;
    uint64_t            total_arbitrations;
    uint64_t            total_resolved;
} swarm_coordinator_t;

/* 协调器操作 */
int     协调器_初始化(swarm_coordinator_t* coord, const char* name);
int     协调器_注册代理(swarm_coordinator_t* coord, swarm_agent_t* agent);
int     协调器_分配任务(swarm_coordinator_t* coord, const char* task_id);
int     协调器_上报冲突(swarm_coordinator_t* coord, const char* task_id, 
                        const char* agent_a, const char* agent_b);
int     协调器_裁决冲突(swarm_coordinator_t* coord, const char* task_id,
                        const char* winner, const char* reason);
int     协调器_广播(swarm_coordinator_t* coord, const char* topic, const char* content);
int     协调器_一轮迭代(swarm_coordinator_t* coord);
int     协调器_心跳检测(swarm_coordinator_t* coord);
void    协调器_报告(swarm_coordinator_t* coord);

#endif /* NJSG_SWARM_H */
