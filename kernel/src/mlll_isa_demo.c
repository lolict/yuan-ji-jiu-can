/**
 * MLLL ISA 语义指令表 独立演示
 * 编译: cc mlll_isa_demo.c -o mlll_isa_demo
 */

#include <stdio.h>
#include <string.h>
#include <stdint.h>

#include "mlll_isa_table.c"

int main(int argc, char** argv) {
    (void)argc; (void)argv;

    printf("============================================\n");
    printf("  囡囝共生 · MLLL ISA 语义指令表 v1.0\n");
    printf("  跨平台语义 ID → 多 ISA 机器码映射\n");
    printf("============================================\n\n");

    /* 全语义 ID 打印 */
    mlll_isa_dump();

    printf("\n");

    /* 单独查看 RISC-V 编码 */
    mlll_isa_dump_isa(ISA_RV32I);

    printf("\n");

    /* 查一条指令 */
    const sem_instr_t* s = mlll_isa_get_by_name("ADD");
    if (s) {
        printf("查询 ADD:\n");
        printf("  描述: %s\n", s->desc);
        const isa_encoding_t* e = mlll_isa_get_encoding(SEM_ADD, ISA_ARM64);
        if (e) {
            printf("  ARM64: %s → %08X\n", e->asm_ex, e->hex_ex);
        }
    }

    printf("\n");

    /* 自检 */
    int err = mlll_isa_selftest();

    printf("\n[总结] %d 条语义指令, %d 个 ISA 目标, 零错误见上\n", SEM_COUNT, ISA_COUNT);
    return err;
}
