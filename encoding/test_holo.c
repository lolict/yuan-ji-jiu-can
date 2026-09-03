/*
 * test_holo.c — 全息链浏览器演示
 *
 * 整合所有子系统的统一全景视图:
 *   1. 创建多链
 *   2. 启动 Swarm
 *   3. 初始化生命周期
 *   4. 截图查看全息状态
 */

#include <stdio.h>
#include <string.h>
#include "holo_viewer.h"
#include "swarm_agents.h"

/* Forward declarations if header not seen */
extern void holo_set_globals(void* coord, void* mem, void* life);
extern void holo_set_stats(uint32_t symbols, uint32_t chains, uint32_t growth, uint32_t nodes, uint32_t diam);

int main(void) {
    printf("╔════════════════════════════════════════════════════════════════╗\n");
    printf("║                                                                ║\n");
    printf("║           全 息 链 浏 览 器 · 最 终 演 示                    ║\n");
    printf("║                                                                ║\n");
    printf("║     整合所有子系统的统一全景视图                              ║\n");
    printf("║     区块链 · 记忆 · 生长 · 图谱 · 云观察 · Swarm           ║\n");
    printf("║                                                                ║\n");
    printf("╚════════════════════════════════════════════════════════════════╝\n");
    
    /* ══════════════════════════════════════════════════
     * 1. 创建符号链
     * ══════════════════════════════════════════════════ */
    printf("\n═══ 阶段 1: 创建符号链 ═══\n");
    
    chain_t chain[3];
    链_init(&chain[0], 0, "天干");
    链_init(&chain[1], 1, "地支");
    链_init(&chain[2], 2, "八卦");
    
    const char* tiangan[] = {"甲","乙","丙","丁","戊","己","庚","辛","壬","癸"};
    const char* dizhi[] = {"子","丑","寅","卯","辰","巳","午","未","申","酉","戌","亥"};
    const char* bagua[] = {"乾","坤","震","巽","坎","离","艮","兑"};
    
    for (int i = 0; i < 10; i++) 链_追加(&chain[0], (uint8_t)(i % 8), tiangan[i], "天", "干");
    for (int i = 0; i < 12; i++) 链_追加(&chain[1], (uint8_t)(i % 8), dizhi[i], "地", "支");
    for (int i = 0; i < 8; i++) 链_追加(&chain[2], (uint8_t)(i % 8), bagua[i], "卦", "象");
    
    uint32_t total = (uint32_t)(chain[0].count + chain[1].count + chain[2].count);
    printf("  ✅ 天干%d + 地支%d + 八卦%d = %u 符号\n",
           (int)chain[0].count, (int)chain[1].count, (int)chain[2].count, total);
    
    /* ══════════════════════════════════════════════════
     * 2. 3D 投影
     * ══════════════════════════════════════════════════ */
    printf("\n═══ 阶段 2: 三维投射 ═══\n");
    
    mlil_scene_t scene;
    多链场景(chain, 3, &scene);
    printf("  ✅ %u 节点投射到 3D 空间\n", scene.count);
    
    /* ══════════════════════════════════════════════════
     * 3. 启动生命周期
     * ══════════════════════════════════════════════════ */
    printf("\n═══ 阶段 3: 启动生命周期 ═══\n");
    
    lifecycle_engine_t life;
    生命周期_初始化(&life, "chains/lifecycle");
    生命周期_宣告身份(&life, "hunyuan");
    生命周期_觉醒(&life);
    
    printf("  ✅ 系统觉醒，世代: %u\n", life.state.identity.generation);
    
    /* ══════════════════════════════════════════════════
     * 4. 构建 Swarm
     * ══════════════════════════════════════════════════ */
    printf("\n═══ 阶段 4: 构建分布式智能体 ═══\n");
    
    swarm_coordinator_t coord;
    协调器_初始化(&coord, "全息协调器");
    
    任务链_init(&coord.tasks, &coord.action_chain);
    
    swarm_agent_t* agents[4];
    agents[0] = create_agent("bootstrap", "自举");
    agents[1] = create_agent("metaime", "元入");
    agents[2] = create_agent("yinyang", "阴阳");
    agents[3] = create_agent("fiveyuan", "五元");
    
    for (int i = 0; i < 4; i++) {
        协调器_注册代理(&coord, agents[i]);
    }
    
    /* 创建任务 */
    任务链_创建(&coord.tasks, "J01", "编码链验证");
    任务链_创建(&coord.tasks, "J02", "层级追溯");
    任务链_创建(&coord.tasks, "J03", "版本回溯");
    
    协调器_一轮迭代(&coord);
    
    printf("  ✅ %u 代理在线, %u 任务就绪\n", coord.agent_count, (uint32_t)coord.tasks.count);
    
    /* ══════════════════════════════════════════════════
     * 5. 全息截图
     * ══════════════════════════════════════════════════ */
    printf("\n═══ 阶段 5: 全息截图 ═══\n");
    
    holo_set_globals(&coord, &life.state.memory, &life);
    holo_set_stats(total, 3, life.state.growth.event_count, scene.count, 24);
    
    holo_snapshot_t snap;
    全息截图(&snap, "系统启动快照");
    全息视图(&snap);
    
    /* ══════════════════════════════════════════════════
     * 6. 推进系统后再截图
     * ══════════════════════════════════════════════════ */
    printf("\n═══ 阶段 6: 推进系统 → 第二张快照 ═══\n");
    
    /* 让系统生长 */
    生长环_触发(&life.state.growth, TRIGGER_DISCOVERY, 1, "全息启动");
    生长环_执行(&life.state.growth, GROWTH_EXTEND, "链延伸", 1);
    
    /* 记忆更多 */
    记忆链_记录(&life.state.memory, MEM_MILESTONE, MEM_DEPTH_INSTINCT,
               "全息浏览器启动", "全息系统");
    
    holo_snapshot_t snap2;
    全息截图(&snap2, "系统生长快照");
    全息视图(&snap2);
    
    /* 对比 */
    全息对比(&snap, &snap2);
    
    /* ══════════════════════════════════════════════════
     * 7. 3D视图
     * ══════════════════════════════════════════════════ */
    printf("\n═══ 阶段 7: 3D 投影视图 ═══\n");
    JSON导出(&scene, "chains/holo_3d.json");
    printf("  ✅ 导出: chains/holo_3d.json (供外部渲染)\n");
    
    /* 休眠 */
    生命周期_休眠(&life);
    
    printf("\n");
    printf("╔════════════════════════════════════════════════════════════════╗\n");
    printf("║                                                                ║\n");
    printf("║              全 息 浏 览 器 · 演 示 完 成                    ║\n");
    printf("║                                                                ║\n");
    printf("║  所有子系统已整合:                                            ║\n");
    printf("║    ✓ 区块链式层级追踪                                         ║\n");
    printf("║    ✓ 记忆链 (跨会话自传)                                      ║\n");
    printf("║    ✓ 生长环 (奖励驱动)                                        ║\n");
    printf("║    ✓ 版本图 (分叉/合并)                                       ║\n");
    printf("║    ✓ 云观察 (7种感知)                                         ║\n");
    printf("║    ✓ Swarm (分布式智能体)                                     ║\n");
    printf("║    ✓ 自举编译器 (规则→代码)                                   ║\n");
    printf("║    ✓ 蚕蛹寄生壳 (多平台适应)                                  ║\n");
    printf("║    ✓ mLIL 3D投影 (静态坐标)                                   ║\n");
    printf("║    ✓ 生命周期 (觉醒→休眠)                                     ║\n");
    printf("║    ✓ 全息浏览器 (全景状态)                                    ║\n");
    printf("║                                                                ║\n");
    printf("║   系统能够观察自身、感知外界、回溯历史、追踪生长            ║\n");
    printf("║   这就是组成自身的过程                                       ║\n");
    printf("║                                                                ║\n");
    printf("╚════════════════════════════════════════════════════════════════╝\n\n");
    
    return 0;
}
