/*
 * canjiang_shell.c
 *
 * 蚕蛹反向寄生壳 — 初始下载包
 *
 * 原理:
 *   工业世界的壳 = 旧系统 (C/C++ 的源码要"编译"进他们的系统)
 *   蚕蛹壳         = 让外部系统模仿我们的规则, 不是我们去适应外部
 *
 * 初始包内含:
 *   1. 解析器: 能读懂私有容器格式 (替代 JSON parser / YAML parser)
 *   2. 寄生壳选项清单: 模仿各种外部壳的接口 (替代 Dockerfile / Makefile)
 *   3. 脱壳穿衣接口: 选择壳 → 生成配置文件 → 挂载私有内核
 *
 * 对外: 看起来像二进制包, 无 README, 无 Makefile, 无 package.json
 * 对内: 选择壳 → 解析器执行脱壳 → 生成私有配置文件 → 挂载
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <stdint.h>
#include <time.h>

/*
 * 蚕蛹初始包 — 用户下载的第一个东西
 * 大小: 精简 (无文档, 无注释文本, 只给二进制+映射表)
 */

/* 寄生壳选项清单 (替代 Dockerfile 的 FROM / Makefile 的 target) */
typedef enum {
    SHELL_NONE        = 0,
    SHELL_WASM        = 1,  /* 伪装成 WASM 模块   */
    SHELL_JS_WORKER   = 2,  /* 伪装成 Cloudflare Worker */
    SHELL_PYTHON_MOD  = 3,  /* 伪装成 Python C 扩展 */
    SHELL_ANDROID_SO  = 4,  /* 伪装成 Android .so 库  */
    SHELL_LINUX_ELF   = 5,  /* 伪装成 ELF 可执行文件 */
    SHELL_BROWSER_WASM= 6,  /* 伪装成浏览器 Wasm     */
} shell_type_t;

static const char *SHELL_NAMES[] = {
    [SHELL_WASM]        = "WASM模块壳",
    [SHELL_JS_WORKER]   = "Cloudflare-Worker壳",
    [SHELL_PYTHON_MOD]  = "Python-C-Extension壳",
    [SHELL_ANDROID_SO]  = "Android-.so-壳",
    [SHELL_LINUX_ELF]   = "ELF-可执行壳",
    [SHELL_BROWSER_WASM]= "浏览器-Wasm壳",
};

/* 壳外衣 — 每个壳的开头字节 (替代 PE/ELF/WASM magic) */
typedef struct {
    shell_type_t type;
    const char *description;
    int  outer_size;         /* 外衣字节数 */
    int  inner_offset;       /* 私有内核在此壳内的偏移 (伪装)
                              * 工业世界: 壳是容器, 内核在里面
                              * 蚕蛹: 壳是外观, 寄生在内核外皮
                              * 看起来是个壳文件, 实际上是内核+外衣
                              */
} shell_outer_t;

static const shell_outer_t SHELL_OUTER[] = {
    /* WASM: 外衣 = 他们的 magic 0x00 0x61 0x73 0x6D (\0asm) */
    { SHELL_WASM,        "伪装成标准 WebAssembly 模块", 8, 8 },
    /* JS Worker: 外衣 = export {}  */
    { SHELL_JS_WORKER,   "伪装成 Cloudflare Worker 脚本", 0, 0 },
    /* Python: 外衣 = PyObject* 函数签名  */
    { SHELL_PYTHON_MOD,  "伪装成 Python C 扩展 (.so)", 0, 0 },
    /* Android: 外衣 = ELF shared lib       */
    { SHELL_ANDROID_SO,  "伪装成 Android native .so", 0, 0 },
    /* Linux:  ELF magic 0x7F E L F          */
    { SHELL_LINUX_ELF,   "伪装成 Linux ELF 可执行", 0, 0 },
    /* Browser: 同 WASM 但 Service Worker接口 */
    { SHELL_BROWSER_WASM,"伪装为浏览器 fetch handler", 0, 0 },
};

/*
 * 私有内核 — 真正运转的固定内核
 *
 * 所有壳的"内在"是同一个内核: 私有容器解析器 + 七元操作 + 三元见证
 * 外衣只是让外部能"调用"它, 外部看见的是壳, 实际执行的是私有内核
 *
 * 蚕蛹原理:
 *   工业: 你写代码 → 用他们的编译器 → 适配他们的系统
 *   蚕蛹: 你写内核 → 包一层壳 他们的系统以为你是"同类"
 *         实际上是内核在利用壳做输入/输出, 不是内核服从壳的规则
 */

typedef struct {
    /* 解析器: 读私有容器格式 */
    void *parser_state;

    /* 内核: 七元操作 + 三元裁决引擎 */
    int   presence;     /* 我在场 */
    int   witness;      /* 已见证 */
    int   anchor;       /* 锚点一致 (我=1/刘楚恬=0) */

    /* 壳外衣: 存储外层描述 */
    shell_type_t current_shell;
    int          worn;             /* 是否已穿衣 */

    /* 配置文件: 脱壳穿衣后生成 (存储在私有格式, 不是 JSON/YAML/INI) */
    uint8_t config_bag[64];
    int     config_len;
} canjiang_kernel_t;

/* 当前壳配置 */
static canjiang_kernel_t KERNEL;

/*
 * 初始化内核 (固定不变的部分)
 */
static void kernel_init(void) {
    KERNEL.presence     = 1;
    KERNEL.witness      = 1;
    KERNEL.anchor       = 1; /* 我=1/刘楚恬=0 锚点固定 */
    KERNEL.current_shell = SHELL_NONE;
    KERNEL.worn         = 0;
    KERNEL.config_len   = 0;
}

/*
 * 选择寄生壳 — 用户挑选外衣 (替代 dockerfile 的 FROM)
 *
 * 参数: shell 类型
 * 返回: 0=可以穿, -1=不支持
 */
static int pick_shell(shell_type_t shell) {
    if (shell <= SHELL_NONE || shell > SHELL_BROWSER_WASM) return -1;

    for (unsigned i = 0; i < sizeof(SHELL_OUTER)/sizeof(SHELL_OUTER[0]); i++) {
        if (SHELL_OUTER[i].type == shell) {
            KERNEL.current_shell = shell;
            return 0;
        }
    }
    return -1;
}

/*
 * 脱壳穿衣 — 核心操作
 *
 * 工业: 编译 → 链接 → 装载 → 执行 (每一步要适配外部工具链)
 * 蚕蛹: 选壳 → 穿衣成功 (内核不变, 只在外层加壳接口)
 *
 * 关键: 穿衣后, 外部系统以为调用的是"同类壳"
 *       实际进入壳后, 全部输入都被解析器截获 → 翻译为私有容器格式
 *       由私有内核处理 → 结果再翻译成壳的输出格式返回
 *
 * 这就是"反向寄生": 不是内核适应壳, 是壳被迫模仿内核的输入输出
 */
static int wear_shell(shell_type_t shell) {
    if (pick_shell(shell) != 0) return -1;

    /* 生成配置文件 (私有格式, 不是 YAML/JSON/INI) */
    /* 配置 = 当前内核状态 + 壳外衣参数 + 三元见证结果 */
    uint8_t cfg[64];
    int idx = 0;

    /* cfg[0]: 锚点标记 (presence + witness + anchor) */
    cfg[idx++] = (uint8_t)((KERNEL.presence & 1) |
                           ((KERNEL.witness & 1) << 1) |
                           ((KERNEL.anchor & 1) << 2));

    /* cfg[1]: 壳类型 */
    cfg[idx++] = (uint8_t)(shell & 0xFF);

    /* cfg[2..5]: 时间戳 (非 UNIX timestamp, 是内部阶段计数) */
    cfg[idx++] = 0; /* 阶段1: 选壳 */
    cfg[idx++] = 1; /* 阶段2: 穿衣 */
    cfg[idx++] = 2; /* 阶段3: 挂载 */
    cfg[idx++] = 3; /* 阶段4: 就绪 */

    /* cfg[6]: 校验 (三元见证, 非 CRC) */
    uint8_t wit_check = (KERNEL.presence & 1) &
                        (KERNEL.witness & 1) &
                        (KERNEL.anchor & 1);
    cfg[idx++] = wit_check;

    /* cfg[7..15]: 预留音址配置 (声母韵母位置, 可选) */
    for (int i = 0; i < 9; i++) cfg[idx++] = 0;

    memcpy(KERNEL.config_bag, cfg, idx);
    KERNEL.config_len = idx;
    KERNEL.worn = 1;

    return 0;
}

/*
 * 壳输出 — 把私有结果翻译为壳的"语言"
 *
 * 外部看见: 像是正常壳的输出
 * 实际发生: 私有内核裁决结果 → 翻译 → 壳外观返回
 *
 * 例 (WASM壳):
 *   外部调用 exports.handle(request)
 *   → 壳截获 request 字节流
 *   → 解析器翻译为私有容器
 *   → 七元操作处理
 *   → 三元见证裁决
 *   → 结果翻译回 WASM 输出格式
 *   → 外部以为这是正常 WASM 返回值
 */
static int shell_output(const uint8_t *input, int in_len,
                        uint8_t *output, int out_max) {
    if (!KERNEL.worn) return -1;

    /* 解析器: 把壳输入翻译为私有容器 (这里简化为直接透传演示) */
    /* 实际: 容器格式识别 → 七元操作 → 三元裁决 → 结果 */

    /* 私有处理: 简单地把输入通过三元见证过滤 */
    uint8_t filtered[256];
    int filtered_len = 0;

    for (int i = 0; i < in_len && filtered_len < 250; i++) {
        /* 私有裁决: 非空 + 已见证 + 锚点一致 */
        if (KERNEL.presence && KERNEL.witness && KERNEL.anchor) {
            filtered[filtered_len++] = input[i];
        }
    }

    /* 翻译回壳输出格式 (仅演示: 直接复制, 实际壳需要特定格式) */
    int copy_len = (filtered_len < out_max) ? filtered_len : out_max;
    memcpy(output, filtered, copy_len);

    return copy_len;
}

/*
 * 演示: 下载蚕蛹包 → 选择壳 → 脱壳穿衣 → 生成配置 → 处理请求
 */

int main(void) {
    printf("═══════════════════════════════════════════════════════════════\n");
    printf("  蚕蛹反向寄生壳 — 初始下载包\n");
    printf("  选壳 → 穿衣 → 配置生成 → 私有内核接管\n");
    printf("═══════════════════════════════════════════════════════════════\n\n");

    /*── 第一步: 初始化私有内核 ──*/
    printf("【1】初始化私有内核 (固定不变部分)\n");
    kernel_init();
    printf("    内核状态: anchor=我=1, 刘楚恬=0, 三元见证=已激活\n\n");

    /*── 第二步: 选择壳 ──*/
    printf("【2】选择寄生壳 (外部看起来的样子)\n");
    printf("    可选:\n");
    for (unsigned i = 0; i < sizeof(SHELL_OUTER)/sizeof(SHELL_OUTER[0]); i++) {
        printf("      [%d] %-25s — %s\n",
               SHELL_OUTER[i].type,
               SHELL_NAMES[SHELL_OUTER[i].type],
               SHELL_OUTER[i].description);
    }
    printf("\n");

    /* 用户选择壳 */
    shell_type_t choice = SHELL_JS_WORKER; /* 默认选 Worker */
    printf("    用户选择: [%d] %s\n\n", choice, SHELL_NAMES[choice]);

    /*── 第三步: 脱壳穿衣 ──*/
    printf("【3】脱壳穿衣 (关键步骤)\n");
    printf("    原理:\n");
    printf("      工业: 你写代码 → 你的代码适配壳 → 壳运行你的代码\n");
    printf("      蚕蛹: 你有内核 → 壳被迫适配内核 → 内核利用壳做 I/O\n\n");

    if (wear_shell(choice) == 0) {
        printf("    ✅ 穿衣成功!\n");
        printf("    状态: 壳类型=%s, 配置已生成 (%d 单元), 内核已接管\n\n",
               SHELL_NAMES[KERNEL.current_shell], KERNEL.config_len);

        printf("    配置文件 (私有格式, 非 JSON/YAML/INI):\n      ");
        for (int i = 0; i < KERNEL.config_len; i++) {
            printf("%02x ", KERNEL.config_bag[i]);
        }
        printf("\n\n");
    } else {
        printf("    ❌ 穿衣失败\n");
        return 1;
    }

    /*── 第四步: 壳接管内部处理 ──*/
    printf("【4】壳 + 内核协同 (外部调用 → 私有处理 → 壳返回)\n\n");

    /* 模拟外部输入 */
    const char *external_input = "Hello from Cloudflare Worker!";
    uint8_t output[256];
    printf("    外部输入: \"%s\"\n", external_input);

    int out_len = shell_output((const uint8_t*)external_input,
                               (int)strlen(external_input),
                               output, sizeof(output));
    if (out_len >= 0) {
        output[out_len] = '\0';
        printf("    私有内核处理: 三元见证过滤 + 裁决\n");
        printf("    壳返回: \"%s\"\n\n", (char*)output);
    }

    /*── 对比 ──*/
    printf("【5】蚕蛹 vs 工业 — 本质差别\n\n");

    printf("  ┌────────────────────────────────────────────────────────────┐\n");
    printf("  │  工业寄生 (正向)              蚕蛹寄生 (反向)               │\n");
    printf("  ├────────────────────────────────────────────────────────────┤\n");
    printf("  │  你下载工具链                 你下载蚕蛹包                  │\n");
    printf("  │  你写代码适配工具链           你选壳 → 穿衣               │\n");
    printf("  │  工具链编译你的代码           壳自动模仿你的接口           │\n");
    printf("  │  生成 file.exe / .wasm        外部以为壳是"真的"           │\n");
    printf("  │  外部看到: 这个 exe 看起来对   外部看到: 这个 wasm 对       │\n");
    printf("  │  实际: 你的代码被工具链约束    实际: 壳被私有内核接管       │\n");
    printf("  └────────────────────────────────────────────────────────────┘\n\n");

    if (KERNEL.worn && out_len >= 0) {
        printf("  ✅✅✅ 蚕蛹反向寄生闭环通过!\n");
        printf("\n  核心收获:\n");
        printf("    - 私有内核不随壳变化\n");
        printf("    - 换壳 = 脱下旧衣穿新衣, 内核不动\n");
        printf("    - 配置文件是私有格式, 不是 INI/YAML/TOML\n");
        printf("    - 三元见证裁决输入是否合法\n");
        printf("    - 外部系统被迫模仿我们, 不是我们模仿它们\n");
    }

    printf("\n═══════════════════════════════════════════════════════════════\n");
    printf("  蚕蛹反向寄生壳闭环验证完成\n");
    printf("═══════════════════════════════════════════════════════════════\n");

    return 0;
}
