/*
 * growth_ring.c — 生长环实现
 *
 * 奖励回路驱动链自增长
 * 发现→延伸→共振→深入→新发现... 螺旋上升
 */

#include <stdio.h>
#include <string.h>
#include "growth_ring.h"

/* ═══════════════════════════════════════════════════
 * 生命周期
 * ═══════════════════════════════════════════════════ */

void 生长环_init(growth_ring_t* gr, memory_chain_t* mem) {
    memset(gr, 0, sizeof(growth_ring_t));
    gr->memory = mem;
    gr->event_count = 0;
    gr->chain_count = 0;
    gr->total_reward = 0;
    gr->cycle_count = 0;
    gr->on_growth = NULL;
    gr->ctx = NULL;
    
    for (int i = 0; i < GROWTH_MAX_CHAINS; i++) {
        gr->echo_states[i] = ECHO_DORMANT;
    }
}

int 生长环_注册回调(growth_ring_t* gr,
                    int (*cb)(growth_direction_t, const char*, void*),
                    void* ctx) {
    gr->on_growth = cb;
    gr->ctx = ctx;
    return 0;
}

/* ═══════════════════════════════════════════════════
 * 触发生长
 * ═══════════════════════════════════════════════════ */

uint32_t 生长环_触发(growth_ring_t* gr, growth_trigger_t trigger,
                      uint32_t src_version, const char* description) {
    if (gr->event_count >= GROWTH_MAX_EVENTS) return 0;
    
    /* 确定生长方向 */
    growth_direction_t dir;
    switch (trigger) {
        case TRIGGER_DISCOVERY:   dir = GROWTH_BRANCH; break;
        case TRIGGER_CONFLICT:    dir = GROWTH_INVERT; break;
        case TRIGGER_RESONANCE:   dir = GROWTH_MERGE;  break;
        case TRIGGER_SILENCE:     dir = GROWTH_DEEPEN; break;
        case TRIGGER_RECURSION:   dir = GROWTH_ECHO;   break;
        case TRIGGER_EXTERNAL:    dir = GROWTH_EXTEND; break;
        default:                  dir = GROWTH_EXTEND; break;
    }
    
    growth_event_t* ev = &gr->events[gr->event_count];
    ev->version = gr->event_count + 1;
    ev->trigger_version = src_version;
    ev->direction = dir;
    ev->trigger = trigger;
    ev->reward_score = 0;
    
    if (description) {
        strncpy(ev->description, description, sizeof(ev->description) - 1);
    }
    
    /* 记录到记忆链 */
    if (gr->memory) {
        char mem_content[256];
        const char* trig_name = "?";
        switch (trigger) {
            case TRIGGER_DISCOVERY: trig_name = "发现"; break;
            case TRIGGER_CONFLICT:  trig_name = "冲突"; break;
            case TRIGGER_RESONANCE: trig_name = "共振"; break;
            case TRIGGER_SILENCE:   trig_name = "空启"; break;
            case TRIGGER_RECURSION: trig_name = "递归"; break;
            case TRIGGER_EXTERNAL:  trig_name = "外入"; break;
        }
        snprintf(mem_content, sizeof(mem_content), "生长触发: %s → %s",
                 trig_name, description ? description : "(无描述)");
        记忆链_记录(gr->memory, MEM_DISCOVERY, MEM_DEPTH_TACTIC,
                    mem_content, "生长环");
    }
    
    gr->event_count++;
    return ev->version;
}

/* ═══════════════════════════════════════════════════
 * 执行生长
 * ═══════════════════════════════════════════════════ */

uint32_t 生长环_执行(growth_ring_t* gr, growth_direction_t dir,
                      const char* content, uint32_t base_version) {
    if (gr->event_count >= GROWTH_MAX_EVENTS) return 0;
    
    growth_event_t* ev = &gr->events[gr->event_count];
    ev->version = gr->event_count + 1;
    ev->trigger_version = base_version;
    ev->direction = dir;
    ev->trigger = TRIGGER_RESONANCE;  /* 执行默认由共振驱动 */
    ev->reward_score = 0;
    
    if (content) {
        strncpy(ev->content, content, sizeof(ev->content) - 1);
    }
    
    /* 执行回调 */
    if (gr->on_growth) {
        int rc = gr->on_growth(dir, content, gr->ctx);
        if (rc != 0) {
            return 0;  /* 回调拒绝生长 */
        }
    }
    
    /* 计算奖励 */
    ev->reward_score = 生长环_计算奖励(gr, ev);
    gr->total_reward += ev->reward_score;
    
    /* 检查是否完成一个循环 */
    if (gr->event_count > 4) {
        /* 简化的循环检测: 发现→延伸→深入→共振→新发现 */
        if (dir == GROWTH_BRANCH && 
            gr->events[gr->event_count - 1].direction == GROWTH_MERGE) {
            gr->cycle_count++;
        }
    }
    
    /* 记录到记忆链 */
    if (gr->memory) {
        char mem_content[256];
        const char* dir_name = "?";
        switch (dir) {
            case GROWTH_EXTEND: dir_name = "延伸"; break;
            case GROWTH_DEEPEN: dir_name = "深入"; break;
            case GROWTH_BRANCH: dir_name = "分叉"; break;
            case GROWTH_MERGE:  dir_name = "合并"; break;
            case GROWTH_ECHO:   dir_name = "回声"; break;
            case GROWTH_INVERT: dir_name = "反转"; break;
        }
        snprintf(mem_content, sizeof(mem_content), "生长: %s | 奖励:%u | %s",
                 dir_name, ev->reward_score, content ? content : "");
        记忆链_记录(gr->memory, MEM_REFLECTION, MEM_DEPTH_OPERATION,
                    mem_content, "生长环");
    }
    
    gr->event_count++;
    return ev->version;
}

/* ═══════════════════════════════════════════════════
 * 回声反馈
 * ═══════════════════════════════════════════════════ */

void 生长环_回声(growth_ring_t* gr, uint32_t chain_idx) {
    if (chain_idx >= GROWTH_MAX_CHAINS) return;
    
    echo_state_t* s = &gr->echo_states[chain_idx];
    
    switch (*s) {
        case ECHO_DORMANT:
            *s = ECHO_OSCILLATE;
            break;
        case ECHO_OSCILLATE:
            *s = ECHO_RESONATE;
            break;
        case ECHO_RESONATE:
            *s = ECHO_AMPLIFY;
            break;
        case ECHO_AMPLIFY:
            *s = ECHO_DECAY;
            break;
        case ECHO_DECAY:
            *s = ECHO_DORMANT;
            /* 完成一个回声循环 */
            gr->cycle_count++;
            break;
    }
}

echo_state_t 生长环_回声状态(growth_ring_t* gr, uint32_t chain_idx) {
    if (chain_idx >= GROWTH_MAX_CHAINS) return ECHO_DORMANT;
    return gr->echo_states[chain_idx];
}

/* ═══════════════════════════════════════════════════
 * 奖励计算
 * ═══════════════════════════════════════════════════ */

uint32_t 生长环_计算奖励(growth_ring_t* gr, const growth_event_t* ev) {
    uint32_t score = 50;  /* 基础分 */
    
    /* 方向加分 */
    switch (ev->direction) {
        case GROWTH_DEEPEN: score += 20; break;  /* 深入更有价值 */
        case GROWTH_MERGE:  score += 15; break;
        case GROWTH_BRANCH: score += 10; break;
        case GROWTH_INVERT: score += 10; break;
        case GROWTH_ECHO:   score += 5;  break;
        case GROWTH_EXTEND: score += 0;  break;
    }
    
    /* 如果前面有反转生长（新视角），加分 */
    if (gr->event_count >= 2) {
        growth_event_t* prev = &gr->events[gr->event_count - 2];
        if (prev->direction == GROWTH_INVERT) {
            score += 10;
        }
    }
    
    /* 内容长度适中加分 */
    if (ev->content[0] && strlen(ev->content) > 10 && strlen(ev->content) < 100) {
        score += 5;
    }
    
    /* 封顶 */
    if (score > 100) score = 100;
    
    return score;
}

/* ═══════════════════════════════════════════════════
 * 循环检测
 * ═══════════════════════════════════════════════════ */

int 生长环_检测循环(growth_ring_t* gr, uint32_t from_version) {
    /* 检测是否在短时间内重复同方向生长（可能导致无限递归） */
    if (gr->event_count < 4) return 0;
    
    growth_direction_t recent[4];
    for (int i = 0; i < 4; i++) {
        recent[i] = gr->events[gr->event_count - 1 - i].direction;
    }
    
    /* 连续4次同方向 = 循环风险 */
    if (recent[0] == recent[1] && recent[1] == recent[2] && recent[2] == recent[3]) {
        return 1;  /* 检测到循环 */
    }
    return 0;
}

/* ═══════════════════════════════════════════════════
 * 报告
 * ═══════════════════════════════════════════════════ */

void 生长环_报告(growth_ring_t* gr) {
    printf("\n");
    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║           生 长 环 · 回 路                      ║\n");
    printf("╠══════════════════════════════════════════════════╣\n");
    printf("║ 生长事件: %-6u  循环次数: %-6u             ║\n",
           gr->event_count, gr->cycle_count);
    printf("║ 累积奖励: %-6u                                ║\n",
           gr->total_reward);
    printf("╚══════════════════════════════════════════════════╝\n");
    
    if (gr->event_count > 0) {
        printf("\n  生长轨迹 (最近10次):\n");
        uint32_t start = (gr->event_count > 10) ? gr->event_count - 10 : 0;
        for (uint32_t i = start; i < gr->event_count; i++) {
            growth_event_t* ev = &gr->events[i];
            const char* dir_icon = "?";
            switch (ev->direction) {
                case GROWTH_EXTEND: dir_icon = "→"; break;
                case GROWTH_DEEPEN: dir_icon = "↓"; break;
                case GROWTH_BRANCH: dir_icon = "⇉"; break;
                case GROWTH_MERGE:  dir_icon = "⊕"; break;
                case GROWTH_ECHO:   dir_icon = "◌"; break;
                case GROWTH_INVERT: dir_icon = "⟲"; break;
            }
            printf("    [%u] %s 奖励:%u  v%u←v%u\n",
                   i + 1, dir_icon, ev->reward_score,
                   ev->version, ev->trigger_version);
            if (ev->content[0]) {
                printf("         └ %s\n", ev->content);
            }
        }
    }
    
    /* 回声状态 */
    printf("\n  回声状态:\n");
    for (uint32_t i = 0; i < gr->chain_count && i < GROWTH_MAX_CHAINS; i++) {
        const char* state_name = "?";
        switch (gr->echo_states[i]) {
            case ECHO_DORMANT:   state_name = "休眠"; break;
            case ECHO_OSCILLATE: state_name = "振荡"; break;
            case ECHO_RESONATE:  state_name = "共振"; break;
            case ECHO_AMPLIFY:   state_name = "放大"; break;
            case ECHO_DECAY:     state_name = "衰减"; break;
        }
        printf("    链路#%u: %s\n", i, state_name);
    }
}
