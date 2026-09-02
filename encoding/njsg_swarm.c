/*
 * njsg_swarm.c — 囡囝共生 分布式智能体框架实现
 *
 * 协调器管理子代理，通过链式追踪所有行动和通信。
 * 冲突由协调器基于历史贡献（奖励分）裁决。
 */

#include <stdio.h>
#include <string.h>
#include <time.h>
#include "njsg_swarm.h"

/* ═══════════════════════════════════════════════════
 * 消息链
 * ═══════════════════════════════════════════════════ */

void 消息链_init(swarm_msg_chain_t* mc) {
    memset(mc, 0, sizeof(swarm_msg_chain_t));
    mc->next_version = 1;
    mc->next_seq = 1;
}

int 消息链_投递(swarm_msg_chain_t* mc, swarm_msg_type_t type,
                const char* from, const char* to,
                const char* topic, const char* content,
                swarm_priority_t prio) {
    if (mc->count >= SWARM_MAX_MESSAGES) return -1;
    
    swarm_message_t* msg = &mc->messages[mc->count];
    msg->version = mc->next_version++;
    msg->prev_version = (mc->count > 0) ? mc->messages[mc->count - 1].version : 0;
    msg->type = type;
    msg->priority = prio;
    msg->timestamp = (uint64_t)time(NULL);
    msg->seq = mc->next_seq++;
    
    if (from) strncpy(msg->from, from, SWARM_ID_LEN - 1);
    if (to) strncpy(msg->to, to, SWARM_ID_LEN - 1);
    if (topic) strncpy(msg->topic, topic, 63);
    if (content) strncpy(msg->content, content, SWARM_MSG_CONTENT - 1);
    
    mc->count++;
    return 0;
}

swarm_message_t* 消息链_查找(swarm_msg_chain_t* mc, const char* topic, swarm_msg_type_t type) {
    for (uint64_t i = 0; i < mc->count; i++) {
        if (strcmp(mc->messages[i].topic, topic) == 0 && mc->messages[i].type == type) {
            return &mc->messages[i];
        }
    }
    return NULL;
}

uint64_t 消息链_计数(const swarm_msg_chain_t* mc) {
    return mc->count;
}

/* ═══════════════════════════════════════════════════
 * 任务链
 * ═══════════════════════════════════════════════════ */

void 任务链_init(swarm_task_chain_t* tc, chain_t* trace) {
    memset(tc, 0, sizeof(swarm_task_chain_t));
    tc->trace_chain = trace;
}

int 任务链_创建(swarm_task_chain_t* tc, const char* id, const char* desc) {
    if (tc->count >= SWARM_MAX_TASKS) return -1;
    if (任务链_查找(tc, id)) return -1;  /* 已存在 */
    
    swarm_task_t* task = &tc->tasks[tc->count];
    memset(task, 0, sizeof(swarm_task_t));
    strncpy(task->task_id, id, TASK_ID_LEN - 1);
    if (desc) strncpy(task->desc, desc, TASK_DESC_LEN - 1);
    task->status = TASK_PENDING;
    task->create_time = (uint64_t)time(NULL);
    
    /* 写入追踪链 */
    if (tc->trace_chain) {
        char obs[128];
        snprintf(obs, sizeof(obs), "创建任务[%s]: %s", id, desc ? desc : "");
        task->chain_version = tc->trace_chain->next_version;
        链_追加(tc->trace_chain, CHAIN_LVL_DAI, id, "协调器", obs);
    }
    
    tc->count++;
    return 0;
}

int 任务链_分配(swarm_task_chain_t* tc, const char* task_id, const char* agent_id) {
    swarm_task_t* task = 任务链_查找(tc, task_id);
    if (!task) return -1;
    
    strncpy(task->assignee, agent_id, SWARM_ID_LEN - 1);
    task->status = TASK_ASSIGNED;
    task->start_time = (uint64_t)time(NULL);
    
    return 0;
}

int 任务链_更新状态(swarm_task_chain_t* tc, const char* task_id, task_status_t status) {
    swarm_task_t* task = 任务链_查找(tc, task_id);
    if (!task) return -1;
    
    task->status = status;
    if (status == TASK_COMPLETED || status == TASK_FAILED) {
        task->finish_time = (uint64_t)time(NULL);
    }
    return 0;
}

int 任务链_标记冲突(swarm_task_chain_t* tc, const char* task_id) {
    swarm_task_t* task = 任务链_查找(tc, task_id);
    if (!task) return -1;
    task->status = TASK_CONFLICT;
    task->conflict_flag = 1;
    return 0;
}

swarm_task_t* 任务链_查找(swarm_task_chain_t* tc, const char* task_id) {
    for (uint64_t i = 0; i < tc->count; i++) {
        if (strcmp(tc->tasks[i].task_id, task_id) == 0) {
            return &tc->tasks[i];
        }
    }
    return NULL;
}

/* ═══════════════════════════════════════════════════
 * 协调器
 * ═══════════════════════════════════════════════════ */

int 协调器_初始化(swarm_coordinator_t* coord, const char* name) {
    if (!coord) return -1;
    memset(coord, 0, sizeof(swarm_coordinator_t));
    
    if (name) {
        strncpy(coord->name, name, SWARM_ID_LEN - 1);
    } else {
        strcpy(coord->name, "njsg-coordinator");
    }
    
    消息链_init(&coord->messages);
    链_init(&coord->action_chain, 0, "协调器行动");
    链_init(&coord->arbitration_chain, 7, "裁判链");
    
    return 0;
}

int 协调器_注册代理(swarm_coordinator_t* coord, swarm_agent_t* agent) {
    if (!coord || !agent) return -1;
    if (coord->agent_count >= SWARM_MAX_AGENTS) return -1;
    
    /* 检查名称唯一性 */
    for (uint32_t i = 0; i < coord->agent_count; i++) {
        if (strcmp(coord->agents[i]->id, agent->id) == 0) return -1;
    }
    
    coord->agents[coord->agent_count++] = agent;
    agent->coordinator = coord;
    agent->active = 1;
    agent->health = 100;
    
    /* 注册回调 */
    if (agent->vtable.on_register) {
        agent->vtable.on_register(agent);
    }
    
    /* 记录 */
    char obs[128];
    snprintf(obs, sizeof(obs), "注册代理[%s]", agent->id);
    链_追加(&coord->action_chain, CHAIN_LVL_JIE, agent->id, coord->name, obs);
    
    return 0;
}

/* 简单的任务分配：找负载最轻的活跃代理 */
int 协调器_分配任务(swarm_coordinator_t* coord, const char* task_id) {
    if (!coord || !task_id) return -1;
    
    /* 找任务 */
    swarm_task_t* task = 任务链_查找(&coord->tasks, task_id);
    if (!task) return -1;
    
    /* 找最空闲的代理 */
    swarm_agent_t* best = NULL;
    uint32_t min_tasks = 0xFFFFFFFF;
    
    for (uint32_t i = 0; i < coord->agent_count; i++) {
        swarm_agent_t* a = coord->agents[i];
        if (a->active && a->health > 50) {
            if (a->task_count < min_tasks) {
                min_tasks = a->task_count;
                best = a;
            }
        }
    }
    
    if (!best) return -1;
    
    任务链_分配(&coord->tasks, task_id, best->id);
    
    /* 发送消息 */
    消息链_投递(&coord->messages, MSG_TASK_ASSIGN,
                coord->name, best->id, task_id,
                task->desc, PRIO_NORMAL);
    
    /* 执行代理回调 */
    if (best->vtable.on_task) {
        best->vtable.on_task(best, task);
        best->task_count++;
    }
    
    /* 记录追踪链 */
    char obs[256];
    snprintf(obs, sizeof(obs), "分配任务[%s] → 代理[%s]", task_id, best->id);
    链_追加(&coord->action_chain, CHAIN_LVL_CENG, task_id, coord->name, obs);
    
    return 0;
}

int 协调器_上报冲突(swarm_coordinator_t* coord, const char* task_id,
                    const char* agent_a, const char* agent_b) {
    if (!coord) return -1;
    
    任务链_标记冲突(&coord->tasks, task_id);
    
    char content[256];
    snprintf(content, sizeof(content), "任务[%s] 冲突: %s vs %s",
             task_id, agent_a, agent_b);
    
    消息链_投递(&coord->messages, MSG_CONFLICT,
                agent_a, coord->name, task_id, content, PRIO_URGENT);
    
    链_追加(&coord->action_chain, CHAIN_LVL_YU, task_id, agent_a, content);
    
    return 0;
}

int 协调器_裁决冲突(swarm_coordinator_t* coord, const char* task_id,
                    const char* winner, const char* reason) {
    if (!coord) return -1;
    
    coord->total_arbitrations++;
    
    /* 找到任务 */
    swarm_task_t* task = 任务链_查找(&coord->tasks, task_id);
    if (!task) return -1;
    
    /* 赢家获得任务，输家标记不活跃 */
    task->status = TASK_RUNNING;
    task->conflict_flag = 0;
    strncpy(task->assignee, winner, SWARM_ID_LEN - 1);
    
    /* 扣输家的健康分 */
    for (uint32_t i = 0; i < coord->agent_count; i++) {
        if (strcmp(coord->agents[i]->id, winner) != 0 && coord->agents[i]->active) {
            if (coord->agents[i]->health > 10) coord->agents[i]->health -= 10;
            
            if (coord->agents[i]->vtable.on_arbitration) {
                coord->agents[i]->vtable.on_arbitration(coord->agents[i], winner);
            }
        }
    }
    
    /* 赢家加分 */
    swarm_agent_t* win_agent = NULL;
    for (uint32_t i = 0; i < coord->agent_count; i++) {
        if (strcmp(coord->agents[i]->id, winner) == 0) {
            win_agent = coord->agents[i];
            win_agent->reward_score += 10;
            win_agent->task_count++;
            break;
        }
    }
    
    coord->total_resolved++;
    
    /* 记录裁判链 */
    char content[512];
    snprintf(content, sizeof(content), "裁判: 任务[%s] 胜者[%s] 理由:%s",
             task_id, winner, reason ? reason : "贡献更高");
    链_追加(&coord->arbitration_chain, CHAIN_LVL_SHI, task_id, coord->name, content);
    
    /* 发送消息给双方 */
    消息链_投递(&coord->messages, MSG_ARBITRATION,
                coord->name, winner, task_id, "胜利", PRIO_HIGH);
    
    return 0;
}

int 协调器_广播(swarm_coordinator_t* coord, const char* topic, const char* content) {
    if (!coord) return -1;
    
    for (uint32_t i = 0; i < coord->agent_count; i++) {
        if (coord->agents[i]->active) {
            消息链_投递(&coord->messages, MSG_SYNC,
                        coord->name, coord->agents[i]->id,
                        topic, content, PRIO_NORMAL);
        }
    }
    
    return 0;
}

int 协调器_一轮迭代(swarm_coordinator_t* coord) {
    if (!coord) return -1;
    coord->round++;
    
    /* 1. 分配待处理任务 */
    for (uint64_t i = 0; i < coord->tasks.count; i++) {
        if (coord->tasks.tasks[i].status == TASK_PENDING) {
            协调器_分配任务(coord, coord->tasks.tasks[i].task_id);
        }
    }
    
    /* 2. 心跳检测 */
    协调器_心跳检测(coord);
    
    return 0;
}

int 协调器_心跳检测(swarm_coordinator_t* coord) {
    static uint64_t last_check = 0;
    uint64_t now = (uint64_t)time(NULL);
    
    /* 每5秒做一次简化心跳 */
    if (now - last_check < 5) return 0;
    last_check = now;
    
    for (uint32_t i = 0; i < coord->agent_count; i++) {
        swarm_agent_t* a = coord->agents[i];
        if (!a->active) continue;
        
        if (a->vtable.on_heartbeat) {
            a->vtable.on_heartbeat(a);
        }
        
        /* 健康度低于30，标记不活跃 */
        if (a->health < 30) {
            a->active = 0;
            char buf[128];
            snprintf(buf, sizeof(buf), "代理[%s] 健康度过低，已下线", a->id);
            链_追加(&coord->action_chain, CHAIN_LVL_JIE_U, a->id, coord->name, buf);
        }
    }
    
    return 0;
}

/* ═══════════════════════════════════════════════════
 * 报告
 * ═══════════════════════════════════════════════════ */

void 协调器_报告(swarm_coordinator_t* coord) {
    printf("\n");
    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║        囡囝共生 SWARM · 协 调 器              ║\n");
    printf("╠══════════════════════════════════════════════════╣\n");
    printf("║ 协调器: %-42s║\n", coord->name);
    printf("║ 轮次: %-44u║\n", coord->round);
    printf("║ 注册代理: %-40u║\n", coord->agent_count);
    printf("║ 任务总数: %-40lu║\n", (unsigned long)coord->tasks.count);
    printf("║ 消息总数: %-40lu║\n", (unsigned long)coord->messages.count);
    printf("║ 冲突裁决: %-40lu║\n", (unsigned long)coord->total_arbitrations);
    printf("║ 成功解决: %-40lu║\n", (unsigned long)coord->total_resolved);
    printf("╚══════════════════════════════════════════════════╝\n");
    
    /* 代理列表 */
    printf("\n  子代理 (%u):\n", coord->agent_count);
    for (uint32_t i = 0; i < coord->agent_count; i++) {
        swarm_agent_t* a = coord->agents[i];
        const char* status = a->active ? "活跃" : "离线";
        printf("    [%s] 健康:%u 任务:%u 奖励:%u %s\n",
               a->id, a->health, a->task_count, a->reward_score, status);
    }
    
    /* 任务列表 */
    if (coord->tasks.count > 0) {
        printf("\n  任务 (%lu):\n", (unsigned long)coord->tasks.count);
        for (uint64_t i = 0; i < coord->tasks.count; i++) {
            swarm_task_t* t = &coord->tasks.tasks[i];
            const char* status = "?";
            switch (t->status) {
                case TASK_PENDING:    status = "待分配"; break;
                case TASK_ASSIGNED:   status = "已分配"; break;
                case TASK_RUNNING:    status = "执行中"; break;
                case TASK_COMPLETED:  status = "已完成"; break;
                case TASK_FAILED:     status = "失败"; break;
                case TASK_CONFLICT:   status = "冲突"; break;
                case TASK_BLOCKED:    status = "阻塞"; break;
            }
            printf("    [%s] %-20s → %-15s [%s]\n",
                   t->task_id, t->desc, t->assignee, status);
        }
    }
    
    /* 行动链摘要 */
    printf("\n  行动链: %lu 节点\n", (unsigned long)coord->action_chain.next_version - 1);
    printf("  裁判链: %lu 节点\n", (unsigned long)coord->arbitration_chain.next_version - 1);
    
    printf("\n");
}
