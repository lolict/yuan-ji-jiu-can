#include <stdio.h>
#include <string.h>
#include "symbol_table.h"

int main(int argc, char *argv[]) {
    if (argc < 2) {
        printf("用法: %s <符号>\n", argv[0]);
        return 1;
    }
    
    const char* sym = argv[1];
    int addr = 查地址(sym);
    
    if (addr >= 0) {
        printf("%s -> 地址%d\n", sym, addr);
    } else {
        printf("%s -> 不在符号表里\n", sym);
    }
    
    return 0;
}
