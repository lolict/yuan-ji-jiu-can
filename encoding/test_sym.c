#include <stdio.h>
#include "symbol_table.h"

int main(void) {
    printf("符号数量: %d\n", 符号数量());
    printf("b → %d\n", 查地址("b"));
    printf("袋 → %d\n", 查地址("袋"));
    printf("照 → %d\n", 查地址("照"));
    printf("阴 → %d\n", 查地址("阴"));
    printf("元 → %d\n", 查地址("元"));
    printf("裁 → %d\n", 查地址("裁"));
    printf("产 → %d\n", 查地址("产"));
    printf("∅ → %d\n", 查地址("∅"));
    printf("地址0 → %s\n", 查符号(0));
    printf("地址24 → %s\n", 查符号(24));
    printf("地址65 → %s\n", 查符号(65));
    printf("存在'袋': %d\n", 存在("袋"));
    printf("存在'xyz': %d\n", 存在("xyz"));
    printf("声母族: 起始=%d, 数量=%d\n", 族起始(0), 族数量(0));
    printf("韵母族: 起始=%d, 数量=%d\n", 族起始(1), 族数量(1));
    printf("产权族: 起始=%d, 数量=%d\n", 族起始(8), 族数量(8));
    return 0;
}
