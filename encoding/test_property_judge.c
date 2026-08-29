/**
 * test_property_judge.c — 产权流侵权判定树端到端测试
 *
 * 测试核心公式 vA + vB == 2·v_core 的各种情况
 */

#include <stdio.h>
#include "property_judge.h"

int main(void) {
    printf("产权流侵权自动判定树 · 端到端测试\n\n");
    
    /* 运行内置演示 */
    property_judge_demo();
    
    /* 额外测试: 边界情况 */
    printf("\n─── 边界测试 ───\n\n");
    
    /* 零核心 */
    {
        property_verdict_t v = property_judge(0, 0, 0);
        printf("零核心(0,0,0): %s\n", v.balanced ? "平衡" : "失衡");
    }
    
    /* 大数值 */
    {
        property_verdict_t v = property_judge(1000000, 1000000, 1000000);
        printf("大数值(1M,1M,1M): %s delta=%d\n",
               v.balanced ? "平衡" : "失衡", v.delta);
    }
    
    /* 极端偏离 */
    {
        property_verdict_t v = property_judge(100, 200, 0);
        printf("极端偏离(100,200,0): 法条=第%d条 %s\n",
               v.clause, v.verdict_text);
    }
    
    printf("\n测试完成.\n");
    return 0;
}
