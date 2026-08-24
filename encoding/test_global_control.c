/**
 * test_global_control.c — 全局控制容器端到端测试
 *
 * 演示: 声母容器(48个) 遍历 韵母容器(66个) 的全过程
 * 这是用户描述的跨容器训练过程:
 *   从声母容器按排队顺序取一个声母,
 *   遍历韵母容器从0维到1维的所有宽度,
 *   遍历完后取下一个声母, 直到所有声母遍历完毕,
 *   然后折叠形成新层。
 */

#include <stdio.h>
#include "global_control.h"

int main(void) {
    global_control_t gc;
    global_control_init(&gc);
    
    printf("═══ 全局控制容器 — 声母遍历韵母 端到端演示 ═══\n\n");
    
    /* ── 第一步: 注册容器 ── */
    printf("─── 第一步: 注册容器 ───\n");
    
    /* 声母容器: 48个, lolict侧(阳) */
    gc_register_container(&gc, 0x10, "声母", GC_POLARITY_YANG, 48);
    
    /* 韵母容器: 66个, 刘楚恬侧(阴) */
    gc_register_container(&gc, 0x20, "韵母", GC_POLARITY_YIN, 66);
    
    /* 声调容器: 5个, 中性 */
    gc_register_container(&gc, 0x30, "声调", GC_POLARITY_NEUTRAL, 5);
    
    printf("  声母容器: 48个 (阳/lolict侧)\n");
    printf("  韵母容器: 66个 (阴/刘楚恬侧)\n");
    printf("  声调容器:  5个 (中性)\n\n");
    
    /* ── 第二步: 设置遍历规则 ── */
    printf("─── 第二步: 添加遍历规则 ───\n");
    
    /* 规则1: 声母遍历韵母 (优先级1, 最高) */
    gc_add_traversal(&gc, 0x10, 0x20, 1);
    printf("  声母(0x10) → 韵母(0x20)  优先级:1\n");
    
    /* 规则2: 声调遍历(声母+韵母) (优先级2) */
    gc_add_traversal(&gc, 0x30, 0x10, 2);
    printf("  声调(0x30) → 声母(0x10)  优先级:2\n\n");
    
    /* ── 第三步: 设置判断规则 ── */
    gc_set_judgment(&gc, GC_MATCH_POSITION, 1, 0, 10);
    printf("─── 第三步: 判断规则 ───\n");
    printf("  匹配方式: 发音位置匹配\n");
    printf("  容差: ±10\n");
    printf("  极性要求: 是\n\n");
    
    /* ── 第四步: 执行遍历 ── */
    printf("─── 第四步: 执行跨容器遍历 ───\n");
    printf("  (声母逐个遍历韵母, 每个声母都从0维到1维)\n\n");
    
    int step = 0;
    int max_steps = 20;  /* 演示前20步 */
    
    while (step < max_steps) {
        uint8_t src, dst;
        int next = gc_get_next_traversal(&gc, &src, &dst);
        if (next < 0) {
            printf("  所有遍历规则已完成\n");
            break;
        }
        
        /* 获取源容器和目标容器信息 */
        int src_idx = gc_find_container(&gc, src);
        int dst_idx = gc_find_container(&gc, dst);
        if (src_idx < 0 || dst_idx < 0) break;
        
        gc_traversal_rule_t *r = &gc.traversal.rules[next];
        
        /* 显示当前步骤 */
        if (r->current_dst_idx == 0 && step > 0) {
            printf("\n  ┌── 新声母[%u] 进入遍历 ──┐\n", r->current_src_idx);
        }
        
        if (step < 10 || step >= max_steps - 5) {
            printf("  第%2d步: %s[%2u] × %s[%2u] → ",
                    step + 1,
                    gc.map.entries[src_idx].name, r->current_src_idx,
                    gc.map.entries[dst_idx].name, r->current_dst_idx);
            
            /* 判断这对是否匹配 */
            uint32_t sym_a = ((uint32_t)src << 8) | r->current_src_idx;
            uint32_t sym_b = ((uint32_t)dst << 8) | r->current_dst_idx;
            int match = gc_judge_pair(&gc, sym_a, sym_b);
            
            if (match) {
                printf("匹配 (发音位置在容差内)\n");
            } else {
                printf("不匹配\n");
            }
        } else if (step == 10) {
            printf("  ... (省略中间步骤) ...\n");
        }
        
        /* 标记当前规则索引 */
        gc.traversal.current_idx = (uint8_t)next;
        
        /* 推进遍历 */
        gc_advance_traversal(&gc);
        step++;
        
        /* 检查是否所有规则都完成了 */
        int all_done = 1;
        for (uint8_t i = 0; i < gc.traversal.count; i++) {
            if (!gc.traversal.rules[i].active) {
                all_done = 0;
                break;
            }
        }
        if (all_done) break;
    }
    
    /* ── 第五步: 状态报告 ── */
    printf("\n─── 第五步: 遍历后状态报告 ───\n");
    gc_report(&gc, stdout);
    
    /* ── 第六步: 聂小倩演示 ── */
    printf("\n─── 聂小倩演示: 为鬼魂找躯壳 ───\n");
    printf("  聂小倩(符鬼): 声=0x10, 韵=0x20, 调=0x30\n");
    printf("  凡人寿壳: 从韵母容器中找与声乳化匹配的\n\n");
    
    /* 演示: 在给定的声母下, 找匹配的韵母 */
    {
        uint32_t nie_xiaoqian = 0x102040;  /* 鬼魂的编码: 声母+韵母+声调 */
        
        printf("  鬼魂编码: 0x%06X\n", nie_xiaoqian);
        printf("  寻找躯壳过程:\n");
        
        int found = 0;
        for (uint16_t yunmu = 0; yunmu < 66 && !found; yunmu++) {
            uint32_t candidate = 0x200000 | yunmu;
            int match = gc_judge_pair(&gc, nie_xiaoqian, candidate);
            
            if (yunmu < 5 || match) {
                printf("    候选韵母[%2u]: 0x%06X → %s\n",
                       yunmu, candidate, match ? "匹配! 借尸还魂成功" : "不匹配");
            } else if (yunmu == 5) {
                printf("    ... (省略不匹配项) ...\n");
            }
            
            if (match) {
                printf("\n  ✦ 聂小倩借尸还魂成功!\n");
                printf("    鬼魂(声母+韵母+声调) 匹配 躯壳(韵母[%u])\n", yunmu);
                found = 1;
            }
        }
        
        if (!found) {
            printf("\n  所有韵母试完了, 没有匹配的躯壳\n");
            printf("  进入恢复流程...\n");
            
            int action = gc_recover(&gc);
            const char *action_name[] = {"", "查看外部", "跨容器遍历", "折叠新层", "记录失败"};
            printf("  恢复策略: %s\n", action_name[action]);
        }
    }
    
    /* ── 最终报告 ── */
    printf("\n═══ 演示完成 ═══\n");
    
    return 0;
}
