/**
 * global_control.c — 全局控制容器实现
 *
 * 五个子容器协同工作:
 *   容器地图 → 遍历规则 → 判断规则 → 失败记录 → 决策树
 *
 * 核心:
 *   当当前容器符号用完, 不重复重启,
 *   而是从外部容器按排队顺序取符号, 遍历当前容器从0维到1维
 */

#include "global_control.h"
#include <string.h>

#define GCTL_MAGIC  0x4743544C  /* "GCTL" */

/* ═══════════════════════════════════════════════════════════════
 * 初始化
 * ═══════════════════════════════════════════════════════════════ */

void global_control_init(global_control_t *gc) {
    memset(gc, 0, sizeof(*gc));
    gc->magic = GCTL_MAGIC;
    gc->version = 1;
    gc->state = GC_STATE_WORKING;
    
    /* 默认判断规则: 发音位置匹配, 需要极性, 不需要审美, 容差10 */
    gc->judgment.match_type = GC_MATCH_POSITION;
    gc->judgment.require_polarity = 1;
    gc->judgment.require_aesthetic = 0;
    gc->judgment.tolerance = 10;
}

/* ═══════════════════════════════════════════════════════════════
 * 容器地图操作
 * ═══════════════════════════════════════════════════════════════ */

int gc_register_container(global_control_t *gc,
                           uint8_t id, const char *name,
                           uint8_t polarity, uint32_t boundary) {
    if (!gc || gc->magic != GCTL_MAGIC) return -1;
    if (gc->map.count >= GC_MAX_CONTAINERS) return -1;
    
    /* 检查是否已注册 */
    if (gc_find_container(gc, id) >= 0) return -1;
    
    gc_container_entry_t *e = &gc->map.entries[gc->map.count];
    e->container_id = id;
    strncpy(e->name, name, GC_NAME_LEN - 1);
    e->name[GC_NAME_LEN - 1] = '\0';
    e->symbol_count = 0;
    e->dimensionality = 0;
    e->polarity = polarity;
    e->boundary_value = boundary;
    
    gc->map.count++;
    return 0;
}

int gc_find_container(const global_control_t *gc, uint8_t id) {
    if (!gc || gc->magic != GCTL_MAGIC) return -1;
    
    for (uint8_t i = 0; i < gc->map.count; i++) {
        if (gc->map.entries[i].container_id == id) return (int)i;
    }
    return -1;
}

/* ═══════════════════════════════════════════════════════════════
 * 遍历规则操作
 * ═══════════════════════════════════════════════════════════════ */

int gc_add_traversal(global_control_t *gc,
                      uint8_t src, uint8_t dst, uint8_t priority) {
    if (!gc || gc->magic != GCTL_MAGIC) return -1;
    if (gc->traversal.count >= GC_MAX_TRAVERSAL_RULES) return -1;
    
    gc_traversal_rule_t *r = &gc->traversal.rules[gc->traversal.count];
    r->src_container = src;
    r->dst_container = dst;
    r->priority = priority;
    r->direction = 0;
    r->current_src_idx = 0;
    r->current_dst_idx = 0;
    r->active = 0;
    
    gc->traversal.count++;
    return 0;
}

/* 获取下一个应该执行的遍历规则 (按优先级排序) */
int gc_get_next_traversal(const global_control_t *gc,
                           uint8_t *src_out, uint8_t *dst_out) {
    if (!gc || gc->magic != GCTL_MAGIC) return -1;
    if (gc->traversal.count == 0) return -1;
    
    /* 找优先级最高且未完成的规则 */
    int best = -1;
    uint8_t best_pri = 255;
    
    for (uint8_t i = 0; i < gc->traversal.count; i++) {
        if (gc->traversal.rules[i].active) continue;
        if (gc->traversal.rules[i].priority < best_pri) {
            best_pri = gc->traversal.rules[i].priority;
            best = (int)i;
        }
    }
    
    if (best < 0) return -1;
    
    *src_out = gc->traversal.rules[best].src_container;
    *dst_out = gc->traversal.rules[best].dst_container;
    return best;
}

/* 推进当前遍历: src_idx++, 到达边界后折叠 */
int gc_advance_traversal(global_control_t *gc) {
    if (!gc || gc->magic != GCTL_MAGIC) return -1;
    if (gc->traversal.current_idx >= gc->traversal.count) return -1;
    
    gc_traversal_rule_t *r = &gc->traversal.rules[gc->traversal.current_idx];
    
    /* 获取目标容器的边界 */
    int dst_idx = gc_find_container(gc, r->dst_container);
    if (dst_idx < 0) return -1;
    gc_container_entry_t *dst = &gc->map.entries[dst_idx];
    
    /* 推进目标索引 */
    r->current_dst_idx++;
    
    /* 如果遍历完目标容器所有符号 */
    if (r->current_dst_idx >= dst->boundary_value) {
        /* 折叠: 目标容器维度+1, 重置索引 */
        dst->dimensionality++;
        r->current_dst_idx = 0;
        
        /* 推进源索引 */
        r->current_src_idx++;
        
        /* 检查源容器是否也遍历完 */
        int src_entry = gc_find_container(gc, r->src_container);
        if (src_entry < 0) { r->active = 1; return 0; }
        
        gc_container_entry_t *src = &gc->map.entries[src_entry];
        if (r->current_src_idx >= src->boundary_value) {
            r->active = 1;  /* 这条遍历规则完成了 */
        }
    }
    
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 判断规则操作
 * ═══════════════════════════════════════════════════════════════ */

void gc_set_judgment(global_control_t *gc, uint8_t match_type,
                      uint8_t require_pol, uint8_t require_aes, uint8_t tol) {
    if (!gc || gc->magic != GCTL_MAGIC) return;
    gc->judgment.match_type = match_type;
    gc->judgment.require_polarity = require_pol;
    gc->judgment.require_aesthetic = require_aes;
    gc->judgment.tolerance = tol;
}

/* 判断一对符号是否匹配 */
int gc_judge_pair(global_control_t *gc,
                   uint32_t sym_a, uint32_t sym_b) {
    if (!gc || gc->magic != GCTL_MAGIC) return 0;
    
    gc->hit_count++;
    
    switch (gc->judgment.match_type) {
        case GC_MATCH_ENCODING:
            /* 禁止使用的编码关系 → 默认拒绝 */
            return 0;
            
        case GC_MATCH_POSITION:
            /* 发音位置匹配:
             * 需要满足 |pos_a - pos_b| <= tolerance
             * sym 的高位 = 容器ID, 低位 = 容器内偏移 */
            {
                uint8_t pos_a = (uint8_t)(sym_a & 0xFF);
                uint8_t pos_b = (uint8_t)(sym_b & 0xFF);
                int diff = (int)pos_a - (int)pos_b;
                if (diff < 0) diff = -diff;
                return (diff <= (int)gc->judgment.tolerance) ? 1 : 0;
            }
            
        case GC_MATCH_AESTHETIC:
            /* 审美品质匹配:
             * 暂时返回0 — 需要视觉数据才能判断 */
            return 0;
            
        case GC_MATCH_EQUIVALENT:
            /* 等效对齐: vA + vB == 2·v_core */
            {
                int32_t sum = (int32_t)sym_a + (int32_t)sym_b;
                int32_t expected = 2 * (int32_t)(sym_a | sym_b);  /* v_core 近似 */
                int32_t diff = sum - expected;
                if (diff < 0) diff = -diff;
                return (diff <= (int32_t)gc->judgment.tolerance) ? 1 : 0;
            }
            
        default:
            return 0;
    }
}

/* ═══════════════════════════════════════════════════════════════
 * 决策树: 卡住时的恢复
 * ═══════════════════════════════════════════════════════════════ */

int gc_recover(global_control_t *gc) {
    if (!gc || gc->magic != GCTL_MAGIC) return -1;
    
    gc->recover_count++;
    
    switch (gc->state) {
        case GC_STATE_STUCK:
            /* 卡住了 → 看看外部容器有什么 */
            {
                int next = gc_get_next_traversal(gc, NULL, NULL);
                if (next >= 0) {
                    gc->state = GC_STATE_RECOVERING;
                    return GC_RECOVER_TRAVERSE;
                }
                return GC_RECOVER_FOLD;
            }
            
        case GC_STATE_RECOVERING:
            /* 恢复中 → 推进遍历 */
            gc_advance_traversal(gc);
            {
                int next = gc_get_next_traversal(gc, NULL, NULL);
                if (next < 0) {
                    gc->state = GC_STATE_DEGRADED;
                    return GC_RECOVER_FOLD;
                }
            }
            return GC_RECOVER_TRAVERSE;
            
        case GC_STATE_DEGRADED:
            /* 退化了 → 折叠 */
            {
                for (uint8_t i = 0; i < gc->map.count; i++) {
                    gc->map.entries[i].dimensionality++;
                }
            }
            gc->state = GC_STATE_WORKING;
            return GC_RECOVER_FOLD;
            
        default:
            return GC_RECORD_FAIL;
    }
}

/* ═══════════════════════════════════════════════════════════════
 * 状态报告
 * ═══════════════════════════════════════════════════════════════ */

void gc_report(const global_control_t *gc, FILE *out) {
    if (!gc || gc->magic != GCTL_MAGIC) return;
    
    fprintf(out, "─── 全局控制容器 ───\n");
    
    const char *state_name[] = {"工作中", "卡住了", "恢复中", "退化了"};
    fprintf(out, "状态: %s\n", state_name[gc->state]);
    fprintf(out, "内容器: %d\n", gc->map.count);
    
    for (uint8_t i = 0; i < gc->map.count; i++) {
        const gc_container_entry_t *e = &gc->map.entries[i];
        const char *pol = (e->polarity == GC_POLARITY_YIN) ? "阴" :
                          (e->polarity == GC_POLARITY_YANG) ? "阳" : "中";
        fprintf(out, "  [%d] %s | 极性:%s | 边界:%u | 维度:%u\n",
                e->container_id, e->name, pol, e->boundary_value, e->dimensionality);
    }
    
    fprintf(out, "遍历规则: %d\n", gc->traversal.count);
    for (uint8_t i = 0; i < gc->traversal.count; i++) {
        const gc_traversal_rule_t *r = &gc->traversal.rules[i];
        fprintf(out, "  [%d] %d→%d 优先级:%d 进度:%u/%u %s\n",
                i, r->src_container, r->dst_container, r->priority,
                r->current_src_idx, r->current_dst_idx,
                r->active ? "[完成]" : "");
    }
    
    const char *judge_name[] = {"编码(禁用)", "发音位置", "审美品质", "等效对齐"};
    fprintf(out, "判断方式: %s\n", judge_name[gc->judgment.match_type]);
    
    fprintf(out, "失败:%u | 恢复:%u | 命中:%u\n",
            gc->fail_count, gc->recover_count, gc->hit_count);
}
