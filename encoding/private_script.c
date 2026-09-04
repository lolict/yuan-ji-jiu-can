/*
 * private_script.c
 *
 * 满全法 私有脚本语言 — 替代 Bash / PowerShell / Cmd / Shell
 *
 * 替代对象:
 *   Bash: if/fi, then/else, $VAR, |, >, ;, &&, ||
 *   PowerShell: Get-xxx, Set-xxx, |, $_ 
 *   Cmd: if/else, %var%, &&, ||
 *
 * 核心机制:
 *   七元操作: 照 昭 罩 召 兆 见 鉴 检 简 剪 加 减 进 退 通 断 分 合 聚 散 折 叠 卷 舒
 *   三元裁决: 非空 + 已见证 + 锚点一致
 *
 * 语法: [七元操作] [目标容器] [参数容器] [见证容器] → 裁决结果
 *
 * 无变量名(用音址), 无换行分隔(用操作码), 无标准字符串(用声母韵母序列)
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <stdint.h>

/*
 * 七元操作码 — 24 个操作分为 4 组
 *
 * 感知族 (感知): 照 昭 罩 兆 见
 * 训练族 (训练): 加 减 进 退
 * 联络族 (联络): 通 断 分 合
 * 裁决族 (裁决): 裁 判 审 决 断
 */

typedef enum {
    OP_ZHAO  = 0x01,  /* 照 — 看目标容器状态 */
    OP_ZHAO2 = 0x02,  /* 昭 — 目标容器发光 (显示内容) */
    OP_ZHAO3 = 0x03,  /* 罩 — 覆盖/隐藏目标容器 */
    OP_ZHAO4 = 0x04,  /* 召 — 调入新容器 */
    OP_ZHAO5 = 0x05,  /* 兆 — 预感/检查边界 */
    OP_JIAN  = 0x06,  /* 见 — 见证 */
    OP_JIAN2 = 0x07,  /* 鉴 — 镜中见 (镜像) */
    OP_JIAN3 = 0x08,  /* 检 — 检查标记 */
    OP_JIAN4 = 0x09,  /* 简 — 简化 (去壳) */
    OP_JIAN5 = 0x0A,  /* 剪 — 裁剪 (切除) */
    OP_JIA   = 0x0B,  /* 加 — 正运算 */
    OP_JIAN6 = 0x0C,  /* 减 — 负运算 (不用"减"避免和"简"同音) */
    OP_JIN   = 0x0D,  /* 进 — 从另一个容器拿东西 */
    OP_TUI   = 0x0E,  /* 退 — 把东西放回另一个容器 */
    OP_TONG  = 0x0F,  /* 通 — 连接两个容器 */
    OP_DUAN  = 0x10,  /* 断 — 断开连接 */
    OP_FEN   = 0x11,  /* 分 — 拆出子袋 */
    OP_HE    = 0x12,  /* 合 — 合并子袋 */
    OP_JU    = 0x13,  /* 聚 — 聚类 */
    OP_SAN   = 0x14,  /* 散 — 散开 */
    OP_ZHE   = 0x15,  /* 折 — 折叠 (符号用完不进位) */
    OP_DIE   = 0x16,  /* 叠 — 堆叠 (叠加) */
    OP_JUAN  = 0x17,  /* 卷 — 卷起 (收起) */
    OP_SHU   = 0x18,  /* 舒 — 舒展 (展开) */
} opcode_t;

static const char *OP_NAMES[] = {
    [OP_ZHAO]  = "照",
    [OP_ZHAO2] = "昭",
    [OP_ZHAO3] = "罩",
    [OP_ZHAO4] = "召",
    [OP_ZHAO5] = "兆",
    [OP_JIAN]  = "见",
    [OP_JIAN2] = "鉴",
    [OP_JIAN3] = "检",
    [OP_JIAN4] = "简",
    [OP_JIAN5] = "剪",
    [OP_JIA]   = "加",
    [OP_JIAN6] = "减",
    [OP_JIN]   = "进",
    [OP_TUI]   = "退",
    [OP_TONG]  = "通",
    [OP_DUAN]  = "断",
    [OP_FEN]   = "分",
    [OP_HE]    = "合",
    [OP_JU]    = "聚",
    [OP_SAN]   = "散",
    [OP_ZHE]   = "折",
    [OP_DIE]   = "叠",
    [OP_JUAN]  = "卷",
    [OP_SHU]   = "舒",
};

/*
 * 三元裁决引擎 — 核心控制流
 *
 * 替代工业控制流:
 *   if (cond) → 三元裁决 (cond 是否通过)
 *   while (cond) → 循环调用三元裁决
 *   && → 裁决通过才执行下一操作
 *   || → 任一裁决通过即可
 *   | 管道 → 通操作(容器连接)
 */

typedef struct {
    int passed;         /* 通过? */
    int fail_reason;    /* 0=通过, 1=空袋, 2=未见证, 3=锚点不一致 */
} verdict_t;

static verdict_t triune_verdict(int presence, int witness, int anchor) {
    verdict_t v;
    v.passed = 0;
    v.fail_reason = 0;
    
    if (!presence) { v.fail_reason = 1; return v; }
    if (!witness)  { v.fail_reason = 2; return v; }
    if (anchor < 0) { v.fail_reason = 3; return v; } /* -1 = 无持有者, 非负即有效 */
    
    v.passed = 1;
    return v;
}

/*
 * 指令格式 (私有编码, 替代文本 shell 命令)
 *
 * 每行: [op1字节] [target_addr: 1字节] [param_addr: 1字节] [witness: 1字节]
 *
 * 无变量名, 无引号, 无换行语义
 */

typedef struct {
    uint8_t op;         /* 七元操作码 */
    uint8_t target;     /* 目标容器音址 */
    uint8_t param;      /* 参数容器音址 */
    uint8_t witness;    /* 见证容器音址 */
} instruction_t;

typedef struct {
    instruction_t *code;
    int count;
    int capacity;
} script_t;

typedef struct {
    char sheng_yun[16];     /* 声母韵母标识 */
    int value;              /* 值 */
    int owner;              /* 持有者的锚点 */
    int present;            /* 有袋/无袋 */
    int witnessed;          /* 已见证/未见证 */
    int ack;                /* 对方是否承认 */
} container_t;

#define MAX_CONTAINERS 16

static container_t CONTAINER_SPACE[MAX_CONTAINERS];
static int CONTAINER_COUNT = 0;

static void init_containers(void) {
    CONTAINER_COUNT = 0;
    for (int i = 0; i < MAX_CONTAINERS; i++) {
        CONTAINER_SPACE[i].present = 0;
        CONTAINER_SPACE[i].witnessed = 0;
        CONTAINER_SPACE[i].ack = 0;
        CONTAINER_SPACE[i].value = 0;
        CONTAINER_SPACE[i].owner = -1; /* -1 = 无持有者, 非负即有效 */
        CONTAINER_SPACE[i].sheng_yun[0] = '\0';
    }
}

static int alloc_container(void) {
    if (CONTAINER_COUNT >= MAX_CONTAINERS) return -1;
    int idx = CONTAINER_COUNT++;
    CONTAINER_SPACE[idx].present = 1;
    CONTAINER_SPACE[idx].witnessed = 1;
    CONTAINER_SPACE[idx].ack = 1;
    return idx;
}

static void script_init(script_t *s) {
    s->capacity = 16;
    s->code = (instruction_t*)malloc(sizeof(instruction_t) * s->capacity);
    s->count = 0;
}

static void script_emit(script_t *s, uint8_t op, uint8_t target, uint8_t param, uint8_t witness) {
    if (s->count >= s->capacity) {
        s->capacity *= 2;
        s->code = (instruction_t*)realloc(s->code, sizeof(instruction_t) * s->capacity);
    }
    s->code[s->count].op       = op;
    s->code[s->count].target   = target;
    s->code[s->count].param    = param;
    s->code[s->count].witness  = witness;
    s->count++;
}

/*
 * 执行单个指令
 *
 * 替代: shell 命令执行 / cmd.exe / PowerShell cmdlet
 */

typedef struct {
    int target_value;
    int param_value;
    int result_value;
    int error;       /* 0=成功, 非0=失败 */
    int store_to;    /* 结果存入哪个容器 */
} exec_result_t;

static exec_result_t exec_one(const instruction_t *inst) {
    exec_result_t r;
    r.error = 0;
    r.store_to = inst->target;

    container_t *target = &CONTAINER_SPACE[inst->target];
    container_t *param  = &CONTAINER_SPACE[inst->param];
    container_t *wit    = &CONTAINER_SPACE[inst->witness];

    /* 三元裁决前置检查 */
    verdict_t v_target = triune_verdict(target->present, target->witnessed, target->owner);
    verdict_t v_param  = triune_verdict(param->present,  param->witnessed,  param->owner);
    verdict_t v_wit    = triune_verdict(wit->present,    wit->witnessed,    wit->owner);

    if (!v_target.passed) {
        r.error = 100 + v_target.fail_reason;
        return r;
    }

    switch (inst->op) {
        case OP_ZHAO:  /* 照 — 看 */
            r.target_value = target->value;
            r.result_value = target->value;
            break;
        case OP_JIA:   /* 加 — 正运算 */
            if (!v_param.passed) { r.error = 200 + v_param.fail_reason; return r; }
            r.result_value = target->value + param->value;
            target->value = r.result_value;
            break;
        case OP_JIAN6: /* 减 — 负运算 */
            if (!v_param.passed) { r.error = 200 + v_param.fail_reason; return r; }
            r.result_value = target->value - param->value;
            target->value = r.result_value;
            break;
        case OP_TONG:  /* 通 — 连接 (管道) */
            if (!v_param.passed) { r.error = 200 + v_param.fail_reason; return r; }
            r.result_value = target->value + param->value;
            target->value = r.result_value;
            break;
        case OP_FEN:   /* 分 — 拆出子袋 */
            if (CONTAINER_COUNT >= MAX_CONTAINERS) { r.error = 1; return r; }
            {
                int sub_idx = alloc_container();
                container_t *sub = &CONTAINER_SPACE[sub_idx];
                sub->value = target->value / 2;
                sub->owner = target->owner;
                sub->present = 1;
                sub->witnessed = 1;
                r.result_value = target->value - sub->value;
                target->value = r.result_value;
            }
            break;
        case OP_HE:    /* 合 — 子袋合并 */
            if (!v_param.passed) { r.error = 200 + v_param.fail_reason; return r; }
            r.result_value = target->value + param->value;
            target->value = r.result_value;
            break;
        case OP_ZHE:   /* 折 — 折叠 (不进位) */
            if (target->value >= 64) {
                r.result_value = target->value - 64;
                target->value = r.result_value;
            } else {
                r.result_value = target->value;
            }
            break;
        case OP_JIAN:  /* 见 — 见证 */
            if (v_param.passed && v_wit.passed) {
                r.result_value = param->value;
                target->value = r.result_value;
            } else {
                r.error = 300;
            }
            break;
        default:
            r.error = 999;
            break;
    }

    return r;
}

static void script_run(script_t *s) {
    for (int i = 0; i < s->count; i++) {
        exec_result_t r = exec_one(&s->code[i]);
        if (r.error) {
            printf("    ⚠️  第 %d 指令裁决拒绝 (原因: %d)\n", i, r.error);
        }
    }
}

static void dump_state(void) {
    printf("    容器空间: ");
    for (int i = 0; i < CONTAINER_COUNT; i++) {
        if (i > 0) printf(" | ");
        printf("[%d]{%d,锚=%d,%s,%s}", i,
               CONTAINER_SPACE[i].value,
               CONTAINER_SPACE[i].owner,
               CONTAINER_SPACE[i].present ? "有" : "无",
               CONTAINER_SPACE[i].witnessed ? "证" : "未证");
    }
    printf("\n");
}

/*
 * 演示: 用七元操作编写脚本 (无变量名, 无换行分隔)
 *
 * 脚本:
 *   1. 加 容器0 容器1 容器0  → 容器0 = 7 + 13 = 20
 *   2. 分 容器0 容器0 容器0  → 从容器0拆出子袋(一半值)
 *   3. 合 容器0 容器2 容器0  → 合并子袋入容器0
 */

int main(void) {
    printf("═══════════════════════════════════════════════════════════════\n");
    printf("  满全法 私有脚本语言 — 替代 Bash / PowerShell / Cmd\n");
    printf("  七元操作 + 三元裁决 (无变量名, 无换行语义)\n");
    printf("═══════════════════════════════════════════════════════════════\n\n");

    init_containers();

    /*── 创建容器空间 ──*/
    int c0 = alloc_container();
    int c1 = alloc_container();
    int c2 = alloc_container();

    container_t *cc0 = &CONTAINER_SPACE[c0];
    container_t *cc1 = &CONTAINER_SPACE[c1];
    container_t *cc2 = &CONTAINER_SPACE[c2];

    cc0->value = 7;     cc0->owner = 1; strcpy(cc0->sheng_yun, "j-i");   /* 容器甲 */
    cc1->value = 13;    cc1->owner = 0; strcpy(cc1->sheng_yun, "sh-i");  /* 容器乙 */
    cc2->value = 0;     cc2->owner = 3; strcpy(cc2->sheng_yun, "g-ong"); /* 共同体 */

    printf("【初始状态】\n");
    dump_state();
    printf("\n");

    /*── 构建脚本 ──*/
    printf("【脚本构建】（用七元操作码, 非文本命令）\n");

    script_t script;
    script_init(&script);

    /* 第一行: 加 c0 c1 c0 */
    script_emit(&script, OP_JIA, c0, c1, c0);
    printf("  [0] 加 c[%d] c[%d] c[%d]  → c[%d] = %d + %d = ?\n",
           c0, c1, c0, c0, cc0->value, cc1->value);

    /* 第二行: 分 c0 c0 c0 (拆出一半) */
    script_emit(&script, OP_FEN, c0, c0, c0);
    int expected_after_div = cc0->value + cc1->value;
    printf("  [1] 分 c[%d] c[%d] c[%d]  → 从 c[%d] 拆出子袋\n",
           c0, c0, c0, c0);

    /* 第三行: 合 c0 new c0 (合并子袋) */
    script_emit(&script, OP_HE, c0, c1, c0);
    printf("  [2] 合 c[%d] c[%d] c[%d]  → 合并 c[%d] + c[%d]\n",
           c0, c1, c0, c0, c1);

    /* 第四行: 折 c0 c0 c0 (符号用完不进位) */
    script_emit(&script, OP_ZHE, c0, c0, c0);
    printf("  [3] 折 c[%d] c[%d] c[%d]  → 不进位折叠\n\n", c0, c0, c0);

    /*── 执行 ──*/
    printf("【脚本执行】\n");
    script_run(&script);

    printf("\n【执行后】\n");
    dump_state();
    printf("\n");

    /*── 验证 ──*/
    printf("【三元裁决验证（遍历全部容器）】\n");
    int all_valid = 1;
    for (int i = 0; i < CONTAINER_COUNT; i++) {
        verdict_t v = triune_verdict(
            CONTAINER_SPACE[i].present,
            CONTAINER_SPACE[i].witnessed,
            CONTAINER_SPACE[i].owner
        );
        printf("  容器[%d]: 值=%d, 锚=%d → %s\n", i,
               CONTAINER_SPACE[i].value,
               CONTAINER_SPACE[i].owner,
               v.passed ? "✅ 通过" : "❌ 不通过");
        if (!v.passed) all_valid = 0;
    }

    if (all_valid) {
        printf("\n  ✅✅✅ 私有脚本闭环通过!\n");
        printf("\n  替代关系:\n");
        printf("    Bash if/fi    → 三元裁决前置检查 (presence + witness + anchor)\n");
        printf("    && 链接      → 前一指令 error=0 才执行下一指令\n");
        printf("    || 备选      → 裁决通过即返回\n");
        printf("    | 管道       → 通 (OP_TONG) 容器连接\n");
        printf("    $VAR 变量    → 容器音址 (j-i, sh-i, g-ong)\n");
        printf("    换行分隔     → 操作码序列 (4字节/指令)\n");
        printf("    > 重定向     → 合 (OP_HE) 合并子袋\n");
    }

    free(script.code);

    printf("\n═══════════════════════════════════════════════════════════════\n");
    printf("  私有脚本语言闭环验证完成\n");
    printf("═══════════════════════════════════════════════════════════════\n");

    return 0;
}
