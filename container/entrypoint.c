/**
 * 满全法容器入口 — 盟机合引擎独立运行入口
 * ------------------------------------
 * 这个文件不屬於 hunyuan-os 内核范畴,仅作为容器运行时的 main 入口。
 * 编译时与 njsg_mengji.c 链接,生成 njsg-mengji-demo 可执行文件。
 *
 * 环境变量:
 *   MENGJI_SCENARIO  运行哪个独立场景
 *                    triune  → 里程碑二: 分布式三元见证
 *                    bag     → 里程碑三: 跨袋子合并拆分
 *                    offline → 里程碑一: 离线恢复
 *                    all     → 全量 demo (默认)
 *                    (空)    → 全量 demo
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* 来自 kernel */
extern void njsg_mengji_init(void);
extern void njsg_mengji_demo(void);
extern int  njsg_mengji_run_scenario(const char* name);

int main(int argc, char **argv)
{
    (void)argc;
    (void)argv;

    printf("━━━ 满全法容器 · 盟机合引擎启动 ━━━\n");

    const char* scenario = getenv("MENGJI_SCENARIO");
    if (scenario && strlen(scenario) > 0) {
        printf("[容器] MENGJI_SCENARIO=%s\n", scenario);
        /* 独立场景已由 run_scenario 内部调用 init,不需要重复 init */
        int rc = njsg_mengji_run_scenario(scenario);
        if (rc != 0) {
            printf("\n[容器] 场景 '%s' 不存在,运行全量 demo\n", scenario);
            njsg_mengji_init();
            njsg_mengji_demo();
        }
    } else {
        printf("[容器] MENGJI_SCENARIO 未设,运行全量 demo\n");
        njsg_mengji_init();
        njsg_mengji_demo();
    }

    printf("\n━━━ 盟机合引擎运行结束 ━━━\n");
    return 0;
}
