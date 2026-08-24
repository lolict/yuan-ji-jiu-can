#include <stdio.h>
#include "global_interlock.h"
#include "symbol_chain.h"

int main(void) {
    printf("═══ 全局互锁横轴 — 齿轮转动演示 ═══\n\n");
    
    /* 初始化横轴 */
    interlock_init();
    printf("横轴初始化: FQM v%04X\n\n", FQM_VERSION);
    
    /* 注册6个容器齿轮 */
    interlock_register_container(CONT_COLOR_SYSTEM, "颜色进制");
    interlock_register_container(CONT_SOUND_RADIX, "声音进制");
    interlock_register_container(CONT_GONGYUESHU, "公约数地址");
    interlock_register_container(CONT_DIMENSION, "多维几何");
    interlock_register_container(CONT_HANZI_GEOM, "汉字几何");
    interlock_register_container(CONT_NEGATIVE, "负样本");
    printf("6个容器齿轮已套上横轴\n\n");
    
    /* 登记符号圆珠 */
    interlock_register_symbol(TYPE_BASE, CONT_COLOR_SYSTEM);
    interlock_register_symbol(TYPE_TONE, CONT_COLOR_SYSTEM);
    interlock_register_symbol(GY_a, CONT_GONGYUESHU);
    interlock_register_symbol(GY_i, CONT_GONGYUESHU);
    interlock_register_symbol(STRUCT_LEFT_RIGHT, CONT_HANZI_GEOM);
    interlock_register_symbol(STROKE_HENG, CONT_HANZI_GEOM);
    interlock_register_symbol(REASON_CROSS_LEVEL, CONT_NEGATIVE);
    printf("符号圆珠已入圈\n\n");
    
    /* 互锁检查 */
    printf("─── 互锁检查 ───\n");
    printf("TYPE_BASE (0x01): %s\n",
           interlock_check_symbol(TYPE_BASE) ? "已登记" : "未登记");
    printf("0x99 (未登记): %s\n",
           interlock_check_symbol(0x99) ? "已登记" : "未登记");
    
    /* 激活互锁 */
    printf("\n─── 激活互锁 ───\n");
    interlock_activate();
    printf("横轴互锁已激活 · 所有齿轮必须遵守\n\n");
    
    /* 横轴状态 */
    interlock_stats(stdout);
    
    printf("\n═══ 演示完成 ═══\n");
    return 0;
}
