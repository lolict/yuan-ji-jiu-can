/**
 * 囡囝共生 · 阴阳三进制解释器核心
 * Yinyang Tri-base Interpreter Core
 *
 * 实现三维进制语义:
 *   每一位由三个 trit (outer:边界, inner:包含, flow:流向) 构成
 *   容器 0 = (outer:0闭锁, inner:0空壳, flow:0静止)
 *   被含 1 = (outer:1开放, inner:1有内容, flow:1流出)
 *   法规 评 = (outer:1输出, inner:1有效, flow:2回流闭环)
 *
 * 语法: s-expression with containers
 *   ( 0 (body...) )  = 阴开启容器,内含 body
 *   1                 = 阳 = 被含物占位
 *   ( 评 a b )       = 共生赔伴裁定(五/六元)
 *   ( 判 )            = 输出 verdict
 *   ( 变 expr )      = 镜像代理(expr 被我们的 hook 截断)
 *   n                 = 微量(数值)
 */

#include "yinyang.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

/* 命名常量(与 kernel/wasm/mqf_wasm.c 一致,双阶梯哲学) */
#define NJ_TRUSTED  1
#define NJ_REJECTED 0
#define NJSG_TRUST_THRESHOLD  0.6f
#define NJSG_MIRROR_TOLERANCE 0.2f

/* ===== 内存管理 ===== */

static yy_val_t *alloc_val(void) {
    yy_val_t *v = calloc(1, sizeof(yy_val_t));
    return v;
}

yy_val_t* yy_make_trit(uint8_t outer, uint8_t inner, uint8_t flow) {
    yy_val_t *v = alloc_val();
    v->type = YY_VAL_TRIT;
    v->tri.outer = outer;
    v->tri.inner = inner;
    v->tri.flow  = flow;
    return v;
}

yy_val_t* yy_make_number(double n) {
    yy_val_t *v = alloc_val();
    v->type = YY_VAL_NUM;
    v->num  = n;
    v->tri.outer = 1;  /* 开放 */
    v->tri.inner = 1;  /* 有内容 */
    return v;
}

yy_val_t* yy_make_verdict(int verdict, double conf) {
    yy_val_t *v = alloc_val();
    v->type = YY_VAL_VERDICT;
    v->verdict.verdict = verdict;
    v->verdict.conf = conf;
    v->tri.outer = verdict;      /* 1=开放/通过, 0=闭锁/拒绝 */
    v->tri.flow  = 2;             /* 回流闭环(裁定是时间的闭环) */
    return v;
}

yy_val_t* yy_make_container(yy_env_t *env, yy_cell_t *parent) {
    (void)env;
    yy_val_t *v = alloc_val();
    v->type = YY_VAL_CELL;
    v->cell = calloc(1, sizeof(yy_cell_t));
    v->cell->depth = parent ? parent->depth + 1 : 0;
    v->cell->parent = parent;
    v->tri.outer = 0;  /* 闭锁(容器边界) */
    v->tri.inner = 0;  /* 目前空壳 */
    v->tri.flow  = 0;  /* 静止(未启用) */
    return v;
}

int yy_cell_append(yy_cell_t *cell, yy_val_t *val) {
    val->next = NULL;
    if (!cell->head) {
        cell->head = val;
    } else {
        yy_val_t *c = cell->head;
        while (c->next) c = c->next;
        c->next = val;
    }
    cell->child_cnt++;
    cell->tid++;  /* 时间坐标向前走 */
    return cell->child_cnt;
}

void yy_free_val(yy_val_t *v) {
    if (!v) return;
    if (v->type == YY_VAL_CELL && v->cell) {
        yy_val_t *c = v->cell->head;
        while (c) { yy_val_t *n = c->next; yy_free_val(c); c = n; }
        free(v->cell);
    }
    free(v);
}

/* ===== 环境 ===== */

int yy_init(yy_env_t *env) {
    memset(env, 0, sizeof(*env));
    env->root.depth = 0;
    env->root.tid   = 0;
    env->current    = &env->root;
    env->sp         = 0;
    env->sym_count  = 0;
    return 0;
}

/* 数值求值辅助 */
static double yy_as_num(yy_val_t *v) {
    if (!v) return 0;
    if (v->type == YY_VAL_NUM) return v->num;
    if (v->type == YY_VAL_VERDICT && v->verdict.verdict == NJ_TRUSTED) return v->verdict.conf;
    return 0;
}

/* 寻找容器内符号对应的值 */
static yy_val_t* cell_lookup(yy_cell_t *cell, const char *key) {
    (void)key;  /* 简化:按位置查找而非符号名 */
    return cell ? cell->head : NULL;
}

/* 寻容器内第 n 个子 */
static yy_val_t* cell_nth(yy_cell_t *cell, int n) {
    if (!cell) return NULL;
    yy_val_t *c = cell->head;
    for (int i = 0; c && i < n; i++) c = c->next;
    return c;
}

/* ===== 核心操作 ===== */

/**
 * 共生赔伴裁定(六元对称群)
 * 输入: 两个数值(compliance_a, compliance_b)
 * 输出: yy_val_t(NJ_TRUSTED 或 NJ_REJECTED + 信任交集)
 */
static yy_val_t* yy_op_judge(yy_env_t *env, yy_cell_t *input) {
    (void)env;
    /* 取容器内前两有效数值 */
    double compliance_a = 0.5f, compliance_b = 0.5f, force = 10.0f, angle = 45.0f;
    yy_val_t *v;
    v = cell_nth(input, 0); if (v) compliance_a = yy_as_num(v);
    v = cell_nth(input, 1); if (v) compliance_b = yy_as_num(v);
    v = cell_nth(input, 2); if (v) force      = yy_as_num(v);
    v = cell_nth(input, 3); if (v) angle      = yy_as_num(v);

    /* 六元信任交集 = min(a, b) */
    double trust_intersection = fmin(compliance_a, compliance_b);

    /* 镜像对齐: |a - b| < 容忍度 */
    double diff = fabs(compliance_a - compliance_b);
    int mirror_ok = (diff < NJSG_MIRROR_TOLERANCE) ? NJ_TRUSTED : NJ_REJECTED;

    /* 裁定: 足够信任 + 镜像对齐 → TRUSTED */
    int verdict = (trust_intersection > NJSG_TRUST_THRESHOLD && mirror_ok) ? NJ_TRUSTED : NJ_REJECTED;

    return yy_make_verdict(verdict, trust_intersection);
}

/**
 * 镜像代理(变):截断外部访问,重新定义身份
 * 在真实实现中,这里会 hook 外部 API,替换为我们自己的响应
 * 当前实现:把输入值乘以镜像系数(象征性地“换皮”)
 */
static yy_val_t* yy_op_mirror(yy_env_t *env, yy_cell_t *input) {
    (void)env;
    /*   */
    yy_val_t *mirrored = yy_make_container(env, NULL);
    mirrored->cell->parent = input ? input->parent : NULL;
    double mirror_coeff = 0.92;
    if (input && input->child_cnt >= 3) {
        double force = yy_as_num(input->head->next->next);
        mirror_coeff = 0.92 + (force > 10 ? 0.03 : 0.0);
    }
    yy_val_t *cur = input ? input->head : NULL;
    while (cur) {
        double val = yy_as_num(cur);
        yy_val_t *mv = yy_make_number(val * mirror_coeff);
        yy_cell_append(mirrored->cell, mv);
        cur = cur->next;
    }
    mirrored->cell->depth = input ? input->depth : 0;
    mirrored->tri.outer = 1;  /* 开放(已被我们接管) */
    mirrored->tri.flow  = 2;  /* 回流闭环 */
    return mirrored;
}

/**
 * 3D 空间打印-for debugging
 */
static void yy_dump_cell(yy_cell_t *c, int indent) {
    for (int i = 0; i < indent; i++) printf("  ");
    printf("[depth=%d tid=%d depth-coord=(%d,%d)]\n",
        c->depth, c->tid, c->depth, c->child_cnt);
    yy_val_t *v = c->head;
    while (v) {
        for (int i = 0; i < indent + 1; i++) printf("  ");
        switch (v->type) {
            case YY_VAL_NUM:     printf("num=%.3f tri=(%d,%d,%d)\n", v->num, v->tri.outer, v->tri.inner, v->tri.flow); break;
            case YY_VAL_VERDICT: printf("verdict=%s conf=%.3f tri=(%d,%d,%d)\n",
                                        v->verdict.verdict ? "TRUSTED" : "REJECTED",
                                        v->verdict.conf, v->tri.outer, v->tri.inner, v->tri.flow); break;
            case YY_VAL_CELL:    { /* 递归打印容器 */
                yy_val_t *inner_trit = yy_make_trit(0,0,0);
                (void)inner_trit;
                printf("cell container:\n");
                yy_dump_cell(v->cell, indent + 2);
                break;
            }
            case YY_VAL_TRIT:    printf("trit(%d,%d,%d)\n", v->tri.outer, v->tri.inner, v->tri.flow); break;
            default: printf("(other)\n"); break;
        }
        v = v->next;
    }
}

/* ===== 公开接口:求值 ===== */

/**
 * 主求值循环
 * expr 格式(简化 s-expr):
 *   - 单数字          → 数值
 *   - (0 <num> ...)   → 创建一个容器(0)装入子节点
 *   - (评 <container>)→ 施加裁定
 *   - (变 <container>)→ 镜像代理
 *   - (判)             → 输出 verdict JSON
 */
int yy_eval(yy_env_t *env, yy_val_t *expr, yy_val_t **out) {
    if (!expr) return -1;

    switch (expr->type) {
        case YY_VAL_NUM:
        case YY_VAL_VERDICT:
        case YY_VAL_TRIT:
            *out = expr;
            return 0;

        case YY_VAL_CELL: {
            /* 容器:对子节点逐一求值,裁决输出 */
            yy_val_t *cur = expr->cell->head;
            yy_val_t *last_result = NULL;
            while (cur) {
                yy_val_t *result = NULL;
                if (yy_eval(env, cur, &result) == 0 && result) {
                    if (last_result && last_result != *out) {
                        /* 保留 verdict */
                        if (last_result->type == YY_VAL_VERDICT) {
                            yy_free_val(last_result);
                        }
                    }
                    last_result = result;
                }
                cur = cur->next;
            }
            *out = last_result ? last_result : expr;
            return 0;
        }

        default:
            *out = expr;
            return 0;
    }
}

/**
 * 从容器数据行构造"评"操作
 * 简化 API:给定 4 个数值,构造容器 + 执行裁定
 */
yy_val_t* yy_judge(yy_env_t *env, double a, double b, double force, double angle) {
    yy_val_t *container = yy_make_container(env, env->current);
    container->cell->parent = (yy_cell_t*)env->root.head;  /* 指向根容器 */
    yy_cell_append(container->cell, yy_make_number(a));
    yy_cell_append(container->cell, yy_make_number(b));
    yy_cell_append(container->cell, yy_make_number(force));
    yy_cell_append(container->cell, yy_make_number(angle));
    yy_val_t *result = yy_op_judge(env, container->cell);
    return result;
}

yy_val_t* yy_mirror(yy_env_t *env, double a, double b, double force, double angle) {
    yy_val_t *container = yy_make_container(env, env->current);
    yy_cell_append(container->cell, yy_make_number(a));
    yy_cell_append(container->cell, yy_make_number(b));
    yy_cell_append(container->cell, yy_make_number(force));
    yy_cell_append(container->cell, yy_make_number(angle));
    return yy_op_mirror(env, container->cell);
}

/* ===== dump API ===== */
void yy_dump(yy_env_t *env) {
    printf("==== 囡囝共生 阴阳三进制运行时状态 ====\n");
    printf("深度=%d 时序=%d 符号数=%d\n",
        env->current ? env->current->depth : -1,
        env->root.tid,
        env->sym_count);
    yy_dump_cell(&env->root, 0);
    printf("========================================\n");
}
