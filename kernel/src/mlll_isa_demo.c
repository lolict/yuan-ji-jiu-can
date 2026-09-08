/**
 * MLLL ISA 语义指令表 独立演示 (适配自动生成版)
 * 编译: cc mlll_isa_demo.c -o mlll_isa_demo
 */
#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include "mlll_isa_table.c"

int main(void) {
    printf("=== MLLL ISA 语义指令表 (自动生成版) ===\n");
    printf("ISA 目标: ISA_RV32I, ISA_X86_64, ISA_ARM64, ISA_MIPS32\n");
    printf("语义 ID 数: %d\n\n", SEM_ID_COUNT);

    #define N_ISA 4
    const char* ISA_STRS[N_ISA] = {"rv32i","x86_64","arm64","mips32"};

    /* 遍历打印非零项 */
    int printed = 0;
    for (int s = 0; s < SEM_ID_COUNT; s++) {
        const sem_instr_t* si = mlll_isa_get((sem_id_t)s);
        if (!si) continue;
        printf("%-5s: ", SEM_NAMES[s]);
        for (int i = 0; i < N_ISA; i++) {
            const isa_encoding_t* e = &si->enc[i];
            if (e->hex_ex != 0)
                printf("%-7s=%08X  ", ISA_STRS[i], e->hex_ex);
            else
                printf("%-7s=------  ", ISA_STRS[i]);
        }
        printf("\n");
        printed++;
    }
    printf("\n");
    /* 验证命名查询 */
    const sem_instr_t* add = mlll_isa_get_by_name("ADD");
    const isa_encoding_t* add_rv = mlll_isa_get_encoding(SEM_ID_ADD, ISA_RV32I);
    const isa_encoding_t* add_arm = mlll_isa_get_encoding(SEM_ID_ADD, ISA_ARM64);
    printf("查询 ADD: RV32I=%08X  ARM64=%08X\n",
           add_rv ? add_rv->hex_ex : 0,
           add_arm ? add_arm->hex_ex : 0);
    printf("按名查询 ADD: %s\n", add ? add->desc : "(miss)");

    /* 自检 */
    int err = mlll_isa_selftest();
    printf("\n共展示 %d 条语义指令, 自检错误=%d\n", printed, err);
    return err;
}
