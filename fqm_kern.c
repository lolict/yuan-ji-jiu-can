/*
 * fqm_kern.c — 意交感息操作系统内核雏形
 * 
 * 功能：整合所有组件的最小操作系统内核
 * 
 * 满全法规则：
 *   - 根基锚点：我=1，刘楚恬=0
 *   - 集装箱编码：mlll 254+2
 *   - 夫妻命运共同体：刘楚恬和lolict
 *   - 夕瑶协议激活双方融合
 * 
 * 身份落款：刘楚恬和我
 * 仓库属性：私人，不开源，仅限本人及刘楚恬使用
 * 
 * 编译：clang -o fqm_kern fqm_kern.c
 * 运行：./fqm_kern
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* ========== 满全法根基锚点 ========== */
#define ANCHOR_SELF  1
#define ANCHOR_LIU   0

/* ========== 系统版本 ========== */
#define KERN_VERSION "0.1"
#define KERN_NAME    "isi-os"

/* ========== 内核状态 ========== */
typedef struct {
    char version[16];
    char name[32];
    char domain[64];
    char owner[32];
    int  boot_time;
    int  ticks;
    int  status;  /* 0=停机, 1=运行, 2=休眠 */
} Kernel;

static Kernel kern;

/* ========== 任务控制块 ========== */
typedef struct {
    int  pid;
    char name[32];
    int  state;     /* 0=空闲, 1=运行, 2=就绪, 3=阻塞 */
    int  priority;
    void (*entry)(void);
} Task;

#define MAX_TASKS 16
static Task tasks[MAX_TASKS];
static int task_cnt = 0;
static int current_task = 0;

/* ========== 内存管理 ========== */
static unsigned char mem[256];
static int mem_used = 0;

/* ========== 内核初始化 ========== */
void kern_init(void) {
    memset(&kern, 0, sizeof(Kernel));
    strncpy(kern.version, KERN_VERSION, 15);
    strncpy(kern.name, KERN_NAME, 31);
    strncpy(kern.domain, "gttx://hieyair", 63);
    strncpy(kern.owner, "刘楚恬和我", 31);
    kern.status = 1;
    kern.ticks = 0;
    
    printf("\n========== %s v%s ==========\n", kern.name, kern.version);
    printf("域: %s\n", kern.domain);
    printf("所有者: %s\n", kern.owner);
    printf("满全法根基: 我=%d, 刘楚恬=%d\n", ANCHOR_SELF, ANCHOR_LIU);
    printf("状态: 运行中\n");
    printf("================================\n\n");
}

/* ========== 任务管理 ========== */
int task_create(const char *name, int prio, void (*func)(void)) {
    if (task_cnt >= MAX_TASKS) return -1;
    Task *t = &tasks[task_cnt];
    t->pid = task_cnt;
    strncpy(t->name, name, 31);
    t->state = 2;  /* 就绪 */
    t->priority = prio;
    t->entry = func;
    printf("[任务] 创建: PID=%d 名称=%s 优先级=%d\n", t->pid, t->name, t->priority);
    task_cnt++;
    return t->pid;
}

/* ========== 内存分配 ========== */
int mem_alloc(int size) {
    if (mem_used + size > 256) return -1;
    int addr = mem_used;
    mem_used += size;
    printf("[内存] 分配: %d字节 地址=%02X\n", size, addr);
    return addr;
}

void mem_free(int size) {
    if (mem_used >= size) mem_used -= size;
    printf("[内存] 释放: %d字节\n", size);
}

/* ========== 示例任务 ========== */
void task_idle(void) {
    printf("[空闲任务] PID=0 运行中\n");
}

void task_worker(void) {
    printf("[工作任务] PID=1 执行计算...\n");
}

void task_display(void) {
    printf("[显示任务] PID=2 刷新输出...\n");
}

/* ========== 系统调用 ========== */
void sys_printf(const char *msg) {
    printf("[SYS] %s\n", msg);
}

int sys_get_ticks(void) {
    return kern.ticks;
}

/* ========== 内核主循环 ========== */
void kern_main(void) {
    printf("\n========== 任务调度 ==========\n");
    
    /* 创建任务 */
    task_create("idle", 0, task_idle);
    task_create("worker", 1, task_worker);
    task_create("display", 2, task_display);
    
    /* 分配内存 */
    printf("\n========== 内存分配 ==========\n");
    int addr1 = mem_alloc(32);
    int addr2 = mem_alloc(64);
    
    /* 运行任务 */
    printf("\n========== 运行任务 ==========\n");
    for (int i = 0; i < task_cnt; i++) {
        current_task = i;
        tasks[i].state = 1;  /* 运行 */
        printf("\n--- 切换至任务 %d (%s) ---\n", i, tasks[i].name);
        tasks[i].entry();
        tasks[i].state = 2;  /* 就绪 */
        kern.ticks++;
    }
    
    /* 系统信息 */
    printf("\n========== 系统信息 ==========\n");
    printf("任务数: %d\n", task_cnt);
    printf("内存使用: %d/256 字节\n", mem_used);
    printf("运行滴答: %d\n", kern.ticks);
    
    /* 完成任务 */
    printf("\n========== 任务完成 ==========\n");
    for (int i = 0; i < task_cnt; i++) {
        printf("任务 %d (%s) 完成\n", i, tasks[i].name);
    }
    
    printf("\n========== 系统停机 ==========\n");
    printf("系统已安全停止\n");
}

/* ========== 主函数 ========== */
int main(void) {
    printf("启动 %s 内核...\n", KERN_NAME);
    
    /* 初始化内核 */
    kern_init();
    
    /* 内核主循环 */
    kern_main();
    
    printf("\n%s v%s 已停止\n", KERN_NAME, kern.version);
    return 0;
}
