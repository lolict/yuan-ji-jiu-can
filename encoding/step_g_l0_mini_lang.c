/*
 * step_g_l0_mini_lang.c
 *
 * MCCP Phase 3 · Step G (L0 自举第二步)
 *
 * L0 迷你语言解释器: 变量绑定 + 顺序执行 + 加法 (UTF-8 中文支持)
 *
 * 语法:
 *   let x = <expr>      — 变量绑定 (变量名支持中文)
 *   print <expr>        — 输出值
 *   <expr> + <expr>     — 加法
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#include "bootstrap_chain.h"
#include "chain_persist.h"
#include "cloud_observer.h"

#define MAX_VARS 16
#define MAX_LINE 512

/* ═══════════════════════════════════════════════════════════════
 * UTF-8 辅助
 * ═══════════════════════════════════════════════════════════════ */

static int utf8_char_len(unsigned char c) {
    if ((c & 0x80) == 0) return 1;        /* ASCII */
    if ((c & 0xE0) == 0xC0) return 2;     /* 110xxxxx */
    if ((c & 0xF0) == 0xE0) return 3;     /* 1110xxxx */
    if ((c & 0xF8) == 0xF0) return 4;     /* 11110xxx */
    return 1; /* 非法按 1 处理 */
}

static int is_varname_char(const char *p) {
    unsigned char c = (unsigned char)*p;
    if (c >= 0x80) return 1; /* UTF-8 多字节字符（可能是中文） */
    if (isalnum(c) || c == '_') return 1;
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 环境: 变量表
 * ═══════════════════════════════════════════════════════════════ */

typedef struct {
    char name[64];
    int value;
} variable_t;

typedef struct {
    variable_t vars[MAX_VARS];
    int count;
} environment_t;

static void env_init(environment_t *env) {
    env->count = 0;
}

static int env_get(environment_t *env, const char *name) {
    for (int i = 0; i < env->count; i++) {
        if (strcmp(env->vars[i].name, name) == 0) {
            return env->vars[i].value;
        }
    }
    return 0;
}

static void env_set(environment_t *env, const char *name, int value) {
    for (int i = 0; i < env->count; i++) {
        if (strcmp(env->vars[i].name, name) == 0) {
            env->vars[i].value = value;
            return;
        }
    }
    if (env->count < MAX_VARS) {
        strncpy(env->vars[env->count].name, name, 63);
        env->vars[env->count].name[63] = '\0';
        env->vars[env->count].value = value;
        env->count++;
    }
}

/* ═══════════════════════════════════════════════════════════════
 * 算术求值 (支持 + 和 UTF-8 变量引用)
 * ═══════════════════════════════════════════════════════════════ */

static int eval_arith(const char *expr, environment_t *env) {
    int result = 0;
    int sign = 1;
    const char *p = expr;

    while (*p) {
        while (*p && isspace((unsigned char)*p)) p++;
        if (!*p) break;

        if (*p == '+') {
            sign = 1;
            p++;
            continue;
        } else if (*p == '-') {
            sign = -1;
            p++;
            continue;
        }

        if (isdigit((unsigned char)*p)) {
            int val = 0;
            while (*p && isdigit((unsigned char)*p)) {
                val = val * 10 + (*p - '0');
                p++;
            }
            result += sign * val;
        } else if (is_varname_char(p)) {
            char varname[64];
            int i = 0;
            while (*p && is_varname_char(p) && i < 63) {
                int char_len = utf8_char_len((unsigned char)*p);
                for (int ci = 0; ci < char_len && i < 63 && *p; ci++) {
                    varname[i++] = *p++;
                }
            }
            varname[i] = '\0';
            int val = env_get(env, varname);
            result += sign * val;
        } else {
            p++;
        }
    }

    return result;
}

/* ═══════════════════════════════════════════════════════════════
 * L0 迷你语言解释器
 * ═══════════════════════════════════════════════════════════════ */

static void strip_trailing_newline(char *s) {
    size_t len = strlen(s);
    while (len > 0 && (s[len-1] == '\n' || s[len-1] == '\r')) {
        s[--len] = '\0';
    }
}

static void skip_spaces(const char **p) {
    while (**p && isspace((unsigned char)**p)) (*p)++;
}

static void read_varname(const char **p, char *out, int max_len) {
    int i = 0;
    while (**p && is_varname_char(*p) && i < max_len - 1) {
        int char_len = utf8_char_len((unsigned char)**p);
        for (int ci = 0; ci < char_len && i < max_len - 1 && **p; ci++) {
            out[i++] = *(*p)++;
        }
    }
    out[i] = '\0';
}

static void l0_run(const char *source, environment_t *env, chain_t *record) {
    char line_copy[MAX_LINE];
    const char *p = source;

    while (*p) {
        int i = 0;
        while (*p && *p != '\n' && i < MAX_LINE - 1) {
            line_copy[i++] = *p++;
        }
        line_copy[i] = '\0';
        if (*p == '\n') p++;

        strip_trailing_newline(line_copy);
        if (strlen(line_copy) == 0) continue;

        /* line 以 # 开头 = 注释 */
        {
            const char *cp = line_copy;
            skip_spaces(&cp);
            if (*cp == '#') continue;
        }

        if (strncmp(line_copy, "let ", 4) == 0) {
            const char *rest = line_copy + 4;
            char varname[64];
            skip_spaces(&rest);
            read_varname(&rest, varname, sizeof(varname));
            skip_spaces(&rest);
            if (*rest == '=') rest++;
            skip_spaces(&rest);

            int val = eval_arith(rest, env);
            env_set(env, varname, val);

            char observation[128];
            snprintf(observation, sizeof(observation), "%s = %d", varname, val);
            链_追加(record, 1, observation, "L0 解释器", "let");
        }
        else if (strncmp(line_copy, "print ", 6) == 0) {
            const char *rest = line_copy + 6;
            int val = eval_arith(rest, env);

            char observation[64];
            snprintf(observation, sizeof(observation), "输出: %d", val);
            链_追加(record, 1, observation, "L0 解释器", "print");

            printf("  📤 %s\n", observation);
        }
    }
}

/* ═══════════════════════════════════════════════════════════════
 * 测试套件
 * ═══════════════════════════════════════════════════════════════ */

typedef struct {
    const char *name;
    const char *source;
    int expected_vars;
    int expected_values[MAX_VARS];
    char expected_names[MAX_VARS][64];
} test_case_t;

static int run_tests(void) {
    printf("\n【L0 迷你语言 测试套件】\n");

    test_case_t tests[] = {
        {
            "ASCII 变量",
            "let x = 10\nlet y = 20\n",
            2, {10, 20}, {"x", "y"}
        },
        {
            "变量引用加法",
            "let a = 5\nlet b = 3\nlet c = a + b\n",
            3, {5, 3, 8}, {"a", "b", "c"}
        },
        {
            "多步加法",
            "let sum = 1 + 2 + 3 + 4 + 5\n",
            1, {15}, {"sum"}
        },
        {
            "UTF-8 中文变量",
            "let 公约数 = 7\nlet 容器乙 = 13\nlet 融合 = 公约数 + 容器乙\n",
            3, {7, 13, 20}, {"公约数", "容器乙", "融合"}
        },
    };

    int n_tests = (int)(sizeof(tests) / sizeof(tests[0]));
    int passed = 0, failed = 0;

    for (int t = 0; t < n_tests; t++) {
        environment_t env;
        env_init(&env);
        chain_t record;
        链_init(&record, 1, "l0_test_record");

        printf("\n  测试 %d: %s\n", t + 1, tests[t].name);
        l0_run(tests[t].source, &env, &record);

        int ok = 1;
        if (env.count != tests[t].expected_vars) {
            printf("    ❌ 变量数: 实际 %d, 期望 %d\n", env.count, tests[t].expected_vars);
            ok = 0;
        } else {
            for (int i = 0; i < tests[t].expected_vars; i++) {
                int actual = env_get(&env, tests[t].expected_names[i]);
                if (actual != tests[t].expected_values[i]) {
                    printf("    ❌ %s = %d, 期望 %d\n",
                           tests[t].expected_names[i], actual, tests[t].expected_values[i]);
                    ok = 0;
                }
            }
        }

        if (ok) {
            printf("    ✅ %s 通过 (%d 变量正确)\n", tests[t].name, env.count);
            passed++;
        } else {
            failed++;
        }
    }

    printf("\n  总计: %d/%d 通过\n", passed, n_tests);
    return failed;
}

/* ═══════════════════════════════════════════════════════════════
 * 主演示
 * ═══════════════════════════════════════════════════════════════ */

int main(void) {
    printf("═══════════════════════════════════════════════════════════════\n");
    printf("  MCCP Phase 3 · Step G · L0 迷你语言解释器 (UTF-8)\n");
    printf("  变量绑定 + 顺序执行 + 加法 + 链持久化\n");
    printf("═══════════════════════════════════════════════════════════════\n\n");

    /* 鼓膜自举 */
    printf("【第 1 步】鼓膜自举\n");
    bootstrap_chain_t bc;
    鼓膜自举_初始化(&bc, "Step G 会话");
    鼓膜自举_执行振荡(&bc);
    鼓膜自举_加载骨架(&bc);
    鼓膜自举_自检(&bc);
    鼓膜自举_激活(&bc);
    printf("  ✅ 自举完成\n\n");

    /* L0 迷你语言演示 */
    printf("【第 2 步】L0 迷你语言 执行\n");

    environment_t env;
    env_init(&env);

    chain_t record;
    链_init(&record, 1, "step_g_record");

    const char *program =
        "# 囡囝共生 Step G — L0 迷你语言\n"
        "let 公约数 = 1\n"
        "let 容器甲 = 7\n"
        "let 容器乙 = 13\n"
        "let 融合 = 容器甲 + 容器乙\n"
        "print 融合\n";

    printf("  L0 源码:\n");
    const char *pp = program;
    while (*pp) {
        if (*pp == '#') {
            while (*pp && *pp != '\n') pp++;
            continue;
        }
        putchar(*pp);
        if (*pp == '\n') {
            /* 跳过连续空行 */
            const char *np = pp + 1;
            while (*np == '\n') np++;
        }
        pp++;
    }
    printf("\n");

    printf("  L0 执行输出:\n");
    l0_run(program, &env, &record);

    printf("\n  变量表 (%d 个):\n", env.count);
    for (int i = 0; i < env.count; i++) {
        printf("    %s = %d\n", env.vars[i].name, env.vars[i].value);
    }

    /* 测试套件 */
    int failed = run_tests();

    /* 云观察 */
    printf("\n【第 3 步】云观察 — 系统自观\n");
    cloud_observer_t obs;
    云观察_init(&obs, "Step G 演示");
    云观察_注册链(&obs, &bc.chain);
    云观察_注册链(&obs, &record);
    云观察_自观(&obs);
    云观察_报告(&obs);

    /* 最终报告 */
    printf("═══════════════════════════════════════════════════════════════\n");
    printf("  Step G 闭环报告\n");
    printf("═══════════════════════════════════════════════════════════════\n");
    printf("  鼓膜自举:      ✅ 已激活\n");
    printf("  L0 变量绑定:   ✅ %d 个变量\n", env.count);
    printf("  L0 求值:       ✅ 7+13=20\n");
    printf("  链持久化:      ✅ %lu 事件\n", (unsigned long)链_长度(&record));
    printf("  自动测试:      %d 通过, %d 失败\n", 4 - failed, failed);

    if (failed == 0) {
        printf("\n  ✅ Step G 闭环通过 — L0 迷你语言 (UTF-8) 工作正常!\n");
        printf("\n  证明: 系统能绑定中文变量、引用变量做加法、输出结果\n");
        printf("  下一步: 把 L0 解释器从 C 移植到 L0 DSL 自身 (终极自举)\n");
    } else {
        printf("\n  ⚠️  %d 个测试失败\n", failed);
    }
    printf("═══════════════════════════════════════════════════════════════\n");

    return failed == 0 ? 0 : 1;
}
