/**
 * 满全法容器入口 — 盟机合引擎独立运行入口
 * ------------------------------------
 * 这个文件不屬於 hunyuan-os 内核范畴,仅作为容器运行时的 main 入口。
 * 编译时与 njsg_mengji.c 链接,生成 njsg-mengji-demo 可执行文件。
 */

#include <stdio.h>
#include <stdlib.h>

/* 来自 kernel */
extern void njsg_mengji_init(void);
extern void njsg_mengji_demo(void);

int main(int argc, char **argv)
{
    (void)argc;
    (void)argv;

    printf("━━━ 满全法容器 · 盟机合引擎启动 ━━━\n");
    printf("容器内 fid 不适用,进程 PID 仅供调试。\n\n");

    njsg_mengji_init();
    njsg_mengji_demo();

    printf("\n━━━ 盟机合引擎运行结束 ━━━\n");
    return 0;
}
