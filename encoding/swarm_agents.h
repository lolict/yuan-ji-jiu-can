/*
 * swarm_agents.h — 子代理创建接口
 */

#ifndef NJSG_SWARM_AGENTS_H
#define NJSG_SWARM_AGENTS_H

#include "njsg_swarm.h"

/* 创建指定类型的代理 */
swarm_agent_t* create_agent(const char* type, const char* id);

/* 查询已注册的代理类型 */
const char* agent_type_name(int idx);
int agent_type_count(void);

#endif /* NJSG_SWARM_AGENTS_H */
