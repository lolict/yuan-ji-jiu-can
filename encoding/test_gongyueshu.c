#include <stdio.h>
#include <string.h>
#include "gongyueshu_address.h"

int main(void) {
    gongyueshu_addr_t ga;
    gongyueshu_init(&ga);
    
    printf("─── 公约数地址容器测试 ───\n\n");
    
    /* 编码一系列音节 */
    struct { const char *s, *y; uint8_t t; } tests[] = {
        {"b", "a", 1}, {"m", "a", 3}, {"zh", "ang", 2},
        {"", "i", 4}, {"d", "ong", 1}, {"l", "ai", 2},
        {"sh", "uang", 1}, {"j", "iong", 3}, {"", "ou", 4},
    };
    
    int n = sizeof(tests) / sizeof(tests[0]);
    int ok = 0;
    for (int i = 0; i < n; i++) {
        int ret = gongyueshu_encode(&ga, tests[i].s, tests[i].y, tests[i].t);
        if (ret == 0) ok++;
    }
    
    printf("编码: %d/%d 成功\n\n", ok, n);
    
    /* 显示每个地址的压缩效果 */
    printf("─── 地址压缩对比 ───\n");
    for (uint16_t i = 0; i < ga.entry_count; i++) {
        char sm[32], ym[32];
        uint8_t tone;
        uint16_t freq;
        gongyueshu_decode(&ga, i, sm, ym, &tone, &freq);
        
        uint32_t packed = gongyueshu_get_packed(&ga, i);
        uint8_t bits = gongyueshu_get_bit_length(&ga, i);
        
        /* 计算全长拼写的位宽 */
        int full_bits = (strlen(sm) + strlen(ym)) * 8 + 2;  /* 字母×8bit + 声调2bit */
        
        printf("  %s%s声%d → %dHz  %dbit (全长%dbit, 省%d%%)\n",
               sm, ym, tone, freq, bits, full_bits, (100 - bits*100/full_bits));
    }
    
    /* 完整统计 */
    printf("\n");
    gongyueshu_stats(&ga, stdout);
    
    printf("\n─── 测试完成 ───\n");
    return 0;
}
