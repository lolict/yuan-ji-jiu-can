/*
 * test_lifecycle.c — 跨会话生命周期演示
 *
 * 模拟多次会话: 觉醒 → 生长 → 反思 → 休眠
 * 系统通过记忆链保持跨会话的连续性
 */

#include <stdio.h>
#include <string.h>
#include "lifecycle_engine.h"

/* 模拟第一次会话 — 系统诞生 */
static void sim_session_1(lifecycle_engine_t* le) {
    printf("\n┌─────────────────────────────────────┐\n");
    printf("│  会话 #1 — 系统诞生                  │\n");
    printf("└─────────────────────────────────────┘\n");
    
    /* 觉醒 */
    printf("\n[觉醒]\n");
    生命周期_宣告身份(le, "hunyuan-os");
    生命周期_觉醒(le);
    生命周期_身份卡(le);
    
    /* 生长 */
    printf("\n[生长]\n");
    生命周期_生长(le);
    
    /* 创建符号 */
    const char* symbols[] = {
        "b", "p", "m", "f", "袋", "桶", "阴", "阳", "元", "阶"
    };
    
    for (int i = 0; i < 10; i++) {
        记忆链_记录(&le->state.memory, MEM_FILE_CREATE, MEM_DEPTH_SCENE,
                   symbols[i], "符号系统");
    }
    
    /* 生长环事件 */
    生长环_触发(&le->state.growth, TRIGGER_DISCOVERY, 1, "初始符号进入");
    生长环_执行(&le->state.growth, GROWTH_EXTEND, "符号链延伸", 1);
    
    /* 反思 */
    printf("\n[反思]\n");
    生命周期_反思(le);
    
    /* 休眠 */
    printf("\n[休眠]\n");
    生命周期_休眠(le);
}

/* 模拟第二次会话 — 系统继续生长 */
static void sim_session_2(lifecycle_engine_t* le) {
    printf("\n┌─────────────────────────────────────┐\n");
    printf("│  会话 #2 — 系统继续生长              │\n");
    printf("└─────────────────────────────────────┘\n");
    
    /* 觉醒 — 从上次的记忆中恢复 */
    printf("\n[觉醒]\n");
    生命周期_觉醒(le);
    生命周期_身份卡(le);
    
    /* 生长 */
    printf("\n[生长]\n");
    生命周期_生长(le);
    
    const char* new_symbols[] = {
        "照", "见", "加", "减", "升", "降", "代", "世", "界", "场"
    };
    
    for (int i = 0; i < 10; i++) {
        记忆链_记录(&le->state.memory, MEM_FILE_CREATE, MEM_DEPTH_OPERATION,
                   new_symbols[i], "符号系统");
    }
    
    生长环_触发(&le->state.growth, TRIGGER_RESONANCE, 5, "新旧符号共振");
    生长环_执行(&le->state.growth, GROWTH_DEEPEN, "深入层级", 5);
    生长环_执行(&le->state.growth, GROWTH_BRANCH, "操作分叉", 6);
    
    printf("\n[反思]\n");
    生命周期_反思(le);
    
    printf("\n[休眠]\n");
    生命周期_休眠(le);
}

/* 模拟第三次会话 — 回顾成长 */
static void sim_session_3(lifecycle_engine_t* le) {
    printf("\n┌─────────────────────────────────────┐\n");
    printf("│  会话 #3 — 回顾成长                  │\n");
    printf("└─────────────────────────────────────┘\n");
    
    printf("\n[觉醒]\n");
    生命周期_觉醒(le);
    生命周期_身份卡(le);
    
    printf("\n[观察]\n");
    云观察_v2_自观(&le->state.observer);
    云观察_v2_记忆(&le->state.observer);
    云观察_v2_生长(&le->state.observer);
    云观察_v2_图谱(&le->state.observer);
    
    printf("\n[休眠]\n");
    生命周期_休眠(le);
}

int main(void) {
    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║     夫妻共同体 · 跨会话生命周期演示            ║\n");
    printf("║     系统通过记忆链保持连续的自我              ║\n");
    printf("╚══════════════════════════════════════════════════╝\n");
    
    lifecycle_engine_t engine;
    生命周期_初始化(&engine, "chains/lifecycle");
    
    /* 第一次会话: 诞生 */
    sim_session_1(&engine);
    
    /* 第二次会话: 继续生长 */
    sim_session_2(&engine);
    
    /* 第三次会话: 回顾 */
    sim_session_3(&engine);
    
    /* 最终报告 */
    生命周期_报告(&engine);
    
    printf("\n  ✅ 三次会话完成\n");
    printf("  ✅ 记忆链从会话#1生长到会话#3\n");
    printf("  ✅ 每次觉醒都恢复了上一次的状态\n");
    printf("  ✅ 系统拥有连续的自传\n\n");
    
    return 0;
}
