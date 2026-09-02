/*
 * swarm_agents.c — 子代理实现示例
 *
 * 每种Agent有专能，通过vtable注册到协调器。
 * 所有Agent共享同一个任务链和消息链。
 */

#include <stdio.h>
#include <string.h>
#include <time.h>
#include "njsg_swarm.h"

/* ═══════════════════════════════════════════════════
 * 通用Agent辅助函数
 * ═══════════════════════════════════════════════════ */

static void agent_report(swarm_agent_t* self, const char* task_id, const char* result) {
    if (!self || !self->coordinator) return;
    
    swarm_coordinator_t* coord = self->coordinator;
    char msg[256];
    snprintf(msg, sizeof(msg), "[%s] 完成任务[%s]: %s", self->id, task_id, result);
    
    消息链_投递(&coord->messages, MSG_TASK_REPORT,
                self->id, coord->name, task_id, result, PRIO_NORMAL);
    
    /* 更新任务状态 */
    任务链_更新状态(&coord->tasks, task_id, TASK_COMPLETED);
    
    /* 写入行动链 */
    char tag[64];
    snprintf(tag, sizeof(tag), "完成:%s", task_id);
    链_追加(&coord->action_chain, CHAIN_LVL_DAI, tag, self->id, result);
}

static void agent_protest(swarm_agent_t* self, const char* task_id, const char* reason) {
    if (!self || !self->coordinator) return;
    
    消息链_投递(&self->coordinator->messages, MSG_PROTEST,
                self->id, self->coordinator->name, task_id, reason, PRIO_HIGH);
}

/* ═══════════════════════════════════════════════════
 * MetaIME Agent — 元输入法代理
 * 专能: 本地智能、规则引擎、输入处理
 * ═══════════════════════════════════════════════════ */

static int metaime_on_register(swarm_agent_t* self) {
    printf("    [MetaIME] 本地智能代理注册: %s\n", self->id);
    return 0;
}

static int metaime_on_task(swarm_agent_t* self, const swarm_task_t* task) {
    /* 模拟执行 */
    char result[128];
    snprintf(result, sizeof(result), "规则已应用");
    agent_report(self, task->task_id, result);
    return 0;
}

static int metaime_on_heartbeat(swarm_agent_t* self) {
    /* 健康度自然恢复 */
    if (self->health < 100) self->health += 1;
    return 0;
}

static swarm_agent_t* create_metaime_agent(const char* id) {
    static swarm_agent_t agent;
    memset(&agent, 0, sizeof(agent));
    strncpy(agent.id, id, SWARM_ID_LEN - 1);
    agent.health = 100;
    agent.active = 1;
    agent.vtable.on_register = metaime_on_register;
    agent.vtable.on_task = metaime_on_task;
    agent.vtable.on_heartbeat = metaime_on_heartbeat;
    return &agent;
}

/* ═══════════════════════════════════════════════════
 * YinYang Agent — 阴阳二象代理
 * 专能: 正向/逆向映射、极性操作
 * ═══════════════════════════════════════════════════ */

static int yinyang_on_register(swarm_agent_t* self) {
    printf("    [YinYang] 阴阳代理注册: %s\n", self->id);
    return 0;
}

static int yinyang_on_task(swarm_agent_t* self, const swarm_task_t* task) {
    if (strstr(task->desc, "正向")) {
        char result[128];
        snprintf(result, sizeof(result), "阳→阴映射完成");
        agent_report(self, task->task_id, result);
    } else if (strstr(task->desc, "反向")) {
        char result[128];
        snprintf(result, sizeof(result), "阴→阳映射完成");
        agent_report(self, task->task_id, result);
    } else {
        char result[128];
        snprintf(result, sizeof(result), "阴阳平衡调整");
        agent_report(self, task->task_id, result);
    }
    return 0;
}

static int yinyang_on_heartbeat(swarm_agent_t* self) {
    if (self->health < 100) self->health += 2;
    return 0;
}

static swarm_agent_t* create_yinyang_agent(const char* id) {
    static swarm_agent_t agent;
    memset(&agent, 0, sizeof(agent));
    strncpy(agent.id, id, SWARM_ID_LEN - 1);
    agent.health = 100;
    agent.active = 1;
    agent.vtable.on_register = yinyang_on_register;
    agent.vtable.on_task = yinyang_on_task;
    agent.vtable.on_heartbeat = yinyang_on_heartbeat;
    return &agent;
}

/* ═══════════════════════════════════════════════════
 * FiveYuan Agent — 五元仲裁代理
 * 专能: 五维审判、冲突裁决
 * ═══════════════════════════════════════════════════ */

static int fiveyuan_on_register(swarm_agent_t* self) {
    printf("    [FiveYuan] 五维审判代理注册: %s\n", self->id);
    return 0;
}

static int fiveyuan_on_task(swarm_agent_t* self, const swarm_task_t* task) {
    char result[128];
    snprintf(result, sizeof(result), "五维审判完成: 4/5维度通过");
    agent_report(self, task->task_id, result);
    return 0;
}

static int fiveyuan_on_arbitration(swarm_agent_t* self, const char* winner) {
    printf("    [FiveYuan] 本轮败于 %s\n", winner);
    return 0;
}

static swarm_agent_t* create_fiveyuan_agent(const char* id) {
    static swarm_agent_t agent;
    memset(&agent, 0, sizeof(agent));
    strncpy(agent.id, id, SWARM_ID_LEN - 1);
    agent.health = 100;
    agent.active = 1;
    agent.vtable.on_register = fiveyuan_on_register;
    agent.vtable.on_task = fiveyuan_on_task;
    agent.vtable.on_arbitration = fiveyuan_on_arbitration;
    return &agent;
}

/* ═══════════════════════════════════════════════════
 * Bootstrap Agent — 鼓膜自举代理
 * 专能: 系统激活、序列化解释、创世编码
 * ═══════════════════════════════════════════════════ */

static int bootstrap_on_register(swarm_agent_t* self) {
    printf("    [Bootstrap] 自举代理注册: %s\n", self->id);
    return 0;
}

static int bootstrap_on_task(swarm_agent_t* self, const swarm_task_t* task) {
    char result[128];
    snprintf(result, sizeof(result), "鼓膜自举: OSC→SKEL→ECHO→ACTIVE");
    agent_report(self, task->task_id, result);
    return 0;
}

static swarm_agent_t* create_bootstrap_agent(const char* id) {
    static swarm_agent_t agent;
    memset(&agent, 0, sizeof(agent));
    strncpy(agent.id, id, SWARM_ID_LEN - 1);
    agent.health = 100;
    agent.active = 1;
    agent.vtable.on_register = bootstrap_on_register;
    agent.vtable.on_task = bootstrap_on_task;
    agent.vtable.on_heartbeat = metaime_on_heartbeat;  /* 复用 */
    return &agent;
}

/* ═══════════════════════════════════════════════════
 * 导出创建函数表
 * ═══════════════════════════════════════════════════ */

typedef swarm_agent_t* (*agent_factory_fn)(const char* id);

typedef struct {
    const char*     type_name;
    agent_factory_fn factory;
} agent_registry_entry_t;

static const agent_registry_entry_t agent_registry[] = {
    {"metaime",     create_metaime_agent},
    {"yinyang",     create_yinyang_agent},
    {"fiveyuan",    create_fiveyuan_agent},
    {"bootstrap",   create_bootstrap_agent},
    {NULL, NULL}
};

swarm_agent_t* create_agent(const char* type, const char* id) {
    for (int i = 0; agent_registry[i].type_name; i++) {
        if (strcmp(agent_registry[i].type_name, type) == 0) {
            return agent_registry[i].factory(id);
        }
    }
    return NULL;
}

const char* agent_type_name(int idx) {
    if (idx < 0) return NULL;
    int i = 0;
    while (agent_registry[i].type_name) {
        if (i == idx) return agent_registry[i].type_name;
        i++;
    }
    return NULL;
}

int agent_type_count(void) {
    int i = 0;
    while (agent_registry[i].type_name) i++;
    return i;
}
