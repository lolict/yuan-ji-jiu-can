/**
 * 阴阳三进制解释器测试
 * 编译: cc -o test_yy test_yy.c yinyang_interp.c -lm
 */
#include "yinyang.h"
#include <stdio.h>
#include <math.h>

int main(void) {
    yy_env_t env;
    yy_init(&env);

    printf("囡囝共生 阴阳三进制解释器测试\n");
    printf("============================\n\n");

    /* 测试 1:裁定 ios 高信任 */
    printf("[测试 1] ios 高信任: a=0.85 b=0.90 force=15 angle=45\n");
    yy_val_t *r1 = yy_judge(&env, 0.85, 0.90, 15.0, 45.0);
    if (r1 && r1->type == YY_VAL_VERDICT) {
        printf("  结果: %s confidence=%.3f tri=(%d,%d,%d)\n",
            r1->verdict.verdict ? "TRUSTED" : "REJECTED",
            r1->verdict.conf,
            r1->tri.outer, r1->tri.inner, r1->tri.flow);
    }

    /* 测试 2:裁定 android 低信任 */
    printf("\n[测试 2] android 低信任: a=0.3 b=0.7 force=2 angle=15\n");
    yy_val_t *r2 = yy_judge(&env, 0.30, 0.70, 2.0, 15.0);
    if (r2 && r2->type == YY_VAL_VERDICT) {
        printf("  结果: %s confidence=%.3f tri=(%d,%d,%d)\n",
            r2->verdict.verdict ? "TRUSTED" : "REJECTED",
            r2->verdict.conf,
            r2->tri.outer, r2->tri.inner, r2->tri.flow);
    }

    /* 测试 3:镜像代理 */
    printf("\n[测试 3] 镜像代理 ios: a=0.85 b=0.90 force=15 angle=45\n");
    yy_val_t *r3 = yy_mirror(&env, 0.85, 0.90, 15.0, 45.0);
    if (r3 && r3->type == YY_VAL_CELL) {
        printf("  镜像后链:\n");
        yy_val_t *c = r3->cell->head;
        int idx = 0;
        while (c) {
            printf("    [%d] num=%.3f tri=(%d,%d,%d)\n", idx++, c->num,
                c->tri.outer, c->tri.inner, c->tri.flow);
            c = c->next;
        }
    }

    /* 测试 4:容器嵌套(3D 层级) */
    printf("\n[测试 4] 3D 嵌套容器 (0(0(1)))\n");
    yy_val_t *outer = yy_make_container(&env, &env.root);
    yy_val_t *mid   = yy_make_container(&env, outer->cell);
    yy_val_t *inner = yy_make_number(1.0);
    inner->tri.outer = 1; inner->tri.inner = 1; inner->tri.flow = 1;  /* 阳 */
    yy_cell_append(mid->cell, inner);
    yy_cell_append(outer->cell, mid);
    printf("  外层容器: depth=%d tid=%d\n", outer->cell->depth, outer->cell->tid);
    printf("  中层容器: depth=%d tid=%d children=%d\n",
        mid->cell->depth, mid->cell->tid, mid->cell->child_cnt);
    printf("  内层阳物: value=%.3f tri=(%d,%d,%d)\n",
        inner->num, inner->tri.outer, inner->tri.inner, inner->tri.flow);

    /* 释放 */
    yy_free_val(r1);
    yy_free_val(r2);
    yy_free_val(r3);
    yy_free_val(outer);

    printf("\n[完成] 阴阳三进制解释器运行正常\n");
    return 0;
}
