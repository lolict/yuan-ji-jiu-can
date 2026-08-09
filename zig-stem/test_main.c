// 测试 main.c 的导出函数 (用系统 cc 跑,无需 zig)
#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

typedef uint8_t  mqf_u8;
typedef uint32_t mqf_u32;
typedef int32_t  mqf_i32;

extern mqf_u32 mqf_symbol_angle(mqf_u8 sym);
extern mqf_u32 mqf_arc_distance(mqf_u32 a, mqf_u32 b);
extern mqf_u8  mqf_time_stage_depth(mqf_u8 n);
extern mqf_i32 mqf_is_adjacent(mqf_u8 x,mqf_u8 y,mqf_u8 z,mqf_u8 ax,mqf_u8 ay,mqf_u8 az);
extern mqf_i32 mqf_boot(void);

int main(void) {
    printf("═══ zig-stem main.c 测试结果 ═══\n");
    printf("mqf_boot             = %d  (expect 0)\n", mqf_boot());
    printf("angle(#9)            = %3u° (expect 54)\n", mqf_symbol_angle(9));
    printf("angle(#32)           = %3u° (expect 192,对径)\n", mqf_symbol_angle(32));
    printf("arc(0,192)           = %3u° (expect 192)\n", mqf_arc_distance(0, 192));
    printf("arc(350,10)          = %3u° (expect 44)\n", mqf_arc_distance(350, 10));
    printf("depth(#48)           = %u   (expect 5)\n", mqf_time_stage_depth(48));
    printf("depth(#12)           = %u   (expect 3)\n", mqf_time_stage_depth(12));
    printf("depth(#1)            = %u   (expect 0)\n", mqf_time_stage_depth(1));
    printf("adj(0,0,0 ; 1,0,0)  = %d   (expect 1)\n", mqf_is_adjacent(0,0,0, 1,0,0));
    printf("adj(0,0,0 ; 2,0,0)  = %d   (expect 0)\n", mqf_is_adjacent(0,0,0, 2,0,0));
    printf("adj(3,3,3 ; 3,3,2)  = %d   (expect 1)\n", mqf_is_adjacent(3,3,3, 3,3,2));
    printf("════════════════════════════════\n");
    return 0;
}
