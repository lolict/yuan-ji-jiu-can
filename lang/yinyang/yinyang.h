/**
 * 囡囝共生 · 阴阳三进制解释器 API
 * Yinyang Tri-base Interpreter for NJSG OS
 *
 * 三维进制:
 *   每位 = (outer:0/1 边界态, inner:0/1 包含态, flow:0/1/2 流向)
 *   容器 0(1) 的 3D 表示: outer=0(闭) + inner=0(空壳) + flow=0(静止)
 *   内容 1   的 3D 表示: outer=1(开) + inner=1(有物) + flow=1(流出)
 */

#ifndef YINYANG_H
#define YINYANG_H

#include <stdint.h>
#include <stddef.h>

#define YINYANG_VERSION "0.1.0"
#define YY_MAX_DEPTH    64     /* 集装箱嵌套深度 */
#define YY_MAX_SYMBOLS  256    /* 符号表大小 */
#define YY_MAX_STACK    1024   /* 求值栈大小 */

/* 三进制位 */
typedef struct {
    uint8_t outer;   /* 边界态: 0=闭锁 1=开放 */
    uint8_t inner;   /* 包含态: 0=空容器 1=有内容 */
    uint8_t flow;    /* 流向: 0=静止 1=流出 2=回流(循环) */
} yy_trit_t;

/* 值类型 */
typedef enum {
    YY_VAL_NIL = 0,
    YY_VAL_TRIT,     /* 单 trit */
    YY_VAL_CELL,     /* 容器(含子节点) */
    YY_VAL_SYMBOL,   /* 符号 */
    YY_VAL_NUM,      /* 数值 */
    YY_VAL_VERDICT,  /* 裁定结果 */
} yy_valtype_t;

/* 值节点(3D 空间中的一个点) */
typedef struct yy_val {
    yy_valtype_t type;
    yy_trit_t    tri;       /* 3D 坐标 */
    union {
        double   num;       /* YY_VAL_NUM */
        uint32_t sym_id;    /* YY_VAL_SYMBOL */
        struct {
            int verdict;    /* NJ_TRUSTED/NJ_REJECTED */
            double conf;    /* 信任交集 */
        } verdict;
        struct yy_cell *cell;  /* YY_VAL_CELL — 指向容器 */
    };
    struct yy_val *next;    /* 链表(容器内子节点) */
} yy_val_t;

/* 容器(集装箱) */
typedef struct yy_cell {
    uint32_t      depth;      /* 嵌套深度(坐标 zhong) */
    uint32_t      tid;        /* 时间坐标(经度),递增 */
    yy_val_t     *head;       /* 子节点链表 */
    struct yy_cell *parent;   /* 外层容器 */
    uint32_t      child_cnt;  /* 子节点数量(控制用) */
} yy_cell_t;

/* 环境 */
typedef struct {
    yy_cell_t   root;            /* 顶层容器 */
    yy_cell_t  *current;         /* 当前环境指针 */
    yy_val_t   *stack[YY_MAX_STACK];
    int         sp;              /* 栈指针 */
    char        symbols[YY_MAX_SYMBOLS][64];
    uint32_t    sym_count;
} yy_env_t;

/* API */
int         yy_init    (yy_env_t *env);
int         yy_eval    (yy_env_t *env, yy_val_t *expr, yy_val_t **out);
yy_val_t*   yy_make_container(yy_env_t *env, yy_cell_t *parent);
yy_val_t*   yy_make_number  (double n);
yy_val_t*   yy_make_verdict (int verdict, double conf);
yy_val_t*   yy_make_trit    (uint8_t outer, uint8_t inner, uint8_t flow);
int         yy_cell_append  (yy_cell_t *cell, yy_val_t *val);
void        yy_free_val     (yy_val_t *v);
yy_val_t*   yy_judge        (yy_env_t *env, double a, double b, double force, double angle);
yy_val_t*   yy_mirror       (yy_env_t *env, double a, double b, double force, double angle);
void        yy_dump         (yy_env_t *env);

#endif /* YINYANG_H */
