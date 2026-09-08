/**
 * mlll_isa_table.c — 语义指令 ID → 多 ISA 机器码映射表
 * ------------------------------------------------
 * 由 tools/mcode-stone/gen_isa_table.py 自动生成。
 * 源: riscv32i/arm64/x86_64/mips32 YAML 位域定义。
 */

#ifndef MLLL_ISA_TABLE_C
#define MLLL_ISA_TABLE_C

#define _POSIX_C_SOURCE 200809L
#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include <strings.h>

typedef enum {
    ISA_RV32I = 0, ISA_X86_64, ISA_ARM64, ISA_MIPS32, ISA_COUNT
} isa_t;

static const char* ISA_NAMES[] = { "rv32i", "x86_64", "arm64", "mips32" };

typedef struct {
    isa_t isa; uint32_t opcode; uint32_t funct3; uint32_t funct7;
    const char* asm_ex; uint32_t hex_ex;
} isa_encoding_t;

typedef enum {
    SEM_ID_ADD=0, SEM_ID_SUB, SEM_ID_AND, SEM_ID_OR, SEM_ID_XOR,
    SEM_ID_SLT, SEM_ID_SLTU, SEM_ID_SLL, SEM_ID_SRL, SEM_ID_SRA,
    SEM_ID_ADDI, SEM_ID_LW, SEM_ID_SW, SEM_ID_BEQ, SEM_ID_JAL, SEM_ID_LUI, SEM_ID_COUNT
} sem_id_t;

static const char* SEM_NAMES[] = {
    "ADD","SUB","AND","OR","XOR","SLT","SLTU",
    "SLL","SRL","SRA","ADDI","LW","SW","BEQ","JAL","LUI"
};

typedef struct {
    sem_id_t id; const char* desc;
    isa_encoding_t enc[ISA_COUNT];
} sem_instr_t;

static sem_instr_t INSTR_TABLE[] = {
    [SEM_ID_ADD] = { .id=SEM_ID_ADD, .desc="整数加法", .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x3100B3, .asm_ex="auto", .hex_ex=0x003100B3 },
            [ISA_X86_64] = { .isa=ISA_X86_64, .opcode=0xCA01, .asm_ex="auto", .hex_ex=0x0000CA01 },
            [ISA_ARM64] = { .isa=ISA_ARM64, .opcode=0x8B030041, .asm_ex="auto", .hex_ex=0x8B030041 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0x430820, .asm_ex="auto", .hex_ex=0x00430820 },
        },
    },

    [SEM_ID_SUB] = { .id=SEM_ID_SUB, .desc="整数减法", .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x403100B3, .asm_ex="auto", .hex_ex=0x403100B3 },
            [ISA_X86_64] = { .isa=ISA_X86_64, .opcode=0xCA29, .asm_ex="auto", .hex_ex=0x0000CA29 },
            [ISA_ARM64] = { .isa=ISA_ARM64, .opcode=0xCB030041, .asm_ex="auto", .hex_ex=0xCB030041 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0x430822, .asm_ex="auto", .hex_ex=0x00430822 },
        },
    },

    [SEM_ID_AND] = { .id=SEM_ID_AND, .desc="按位与", .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x7372B3, .asm_ex="auto", .hex_ex=0x007372B3 },
            [ISA_X86_64] = { .isa=ISA_X86_64, .opcode=0xCA21, .asm_ex="auto", .hex_ex=0x0000CA21 },
            [ISA_ARM64] = { .isa=ISA_ARM64, .opcode=0x8A030041, .asm_ex="auto", .hex_ex=0x8A030041 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0x430824, .asm_ex="auto", .hex_ex=0x00430824 },
        },
    },

    [SEM_ID_OR] = { .id=SEM_ID_OR, .desc="按位或", .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x7362B3, .asm_ex="auto", .hex_ex=0x007362B3 },
            [ISA_X86_64] = { .isa=ISA_X86_64, .opcode=0xCA09, .asm_ex="auto", .hex_ex=0x0000CA09 },
            [ISA_ARM64] = { .isa=ISA_ARM64, .opcode=0xAA030041, .asm_ex="auto", .hex_ex=0xAA030041 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0x430825, .asm_ex="auto", .hex_ex=0x00430825 },
        },
    },

    [SEM_ID_XOR] = { .id=SEM_ID_XOR, .desc="按位异或", .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x7342B3, .asm_ex="auto", .hex_ex=0x007342B3 },
            [ISA_X86_64] = { .isa=ISA_X86_64, .opcode=0xCA31, .asm_ex="auto", .hex_ex=0x0000CA31 },
            [ISA_ARM64] = { .isa=ISA_ARM64, .opcode=0xCA030041, .asm_ex="auto", .hex_ex=0xCA030041 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0x430826, .asm_ex="auto", .hex_ex=0x00430826 },
        },
    },

    [SEM_ID_SLT] = { .id=SEM_ID_SLT, .desc="有符号小于置位", .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x7322B3, .asm_ex="auto", .hex_ex=0x007322B3 },
            [ISA_X86_64] = { .isa=ISA_X86_64, .opcode=0xCA39, .asm_ex="auto", .hex_ex=0x0000CA39 },
            [ISA_ARM64] = { .isa=ISA_ARM64, .opcode=0x91001421, .asm_ex="auto", .hex_ex=0x91001421 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0x43082A, .asm_ex="auto", .hex_ex=0x0043082A },
        },
    },

    [SEM_ID_SLTU] = { .id=SEM_ID_SLTU, .desc="无符号小于置位", .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x7332B3, .asm_ex="auto", .hex_ex=0x007332B3 },
            [ISA_X86_64] = { .isa=ISA_X86_64, .opcode=0xCA39, .asm_ex="auto", .hex_ex=0x0000CA39 },
            [ISA_ARM64] = { .isa=ISA_ARM64, .opcode=0x91001421, .asm_ex="auto", .hex_ex=0x91001421 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0x43082B, .asm_ex="auto", .hex_ex=0x0043082B },
        },
    },

    [SEM_ID_SLL] = { .id=SEM_ID_SLL, .desc="逻辑左移", .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x3110B3, .asm_ex="auto", .hex_ex=0x003110B3 },
            [ISA_X86_64] = { .isa=ISA_X86_64, .opcode=0xE1C104, .asm_ex="auto", .hex_ex=0x00E1C104 },
            [ISA_ARM64] = { .isa=ISA_ARM64, .opcode=0x93407C41, .asm_ex="auto", .hex_ex=0x93407C41 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0x430004, .asm_ex="auto", .hex_ex=0x00430004 },
        },
    },

    [SEM_ID_SRL] = { .id=SEM_ID_SRL, .desc="逻辑右移", .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x3150B3, .asm_ex="auto", .hex_ex=0x003150B3 },
            [ISA_X86_64] = { .isa=ISA_X86_64, .opcode=0xE9C104, .asm_ex="auto", .hex_ex=0x00E9C104 },
            [ISA_ARM64] = { .isa=ISA_ARM64, .opcode=0x93407C81, .asm_ex="auto", .hex_ex=0x93407C81 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0x430006, .asm_ex="auto", .hex_ex=0x00430006 },
        },
    },

    [SEM_ID_SRA] = { .id=SEM_ID_SRA, .desc="算术右移", .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x403150B3, .asm_ex="auto", .hex_ex=0x403150B3 },
            [ISA_X86_64] = { .isa=ISA_X86_64, .opcode=0xF9C104, .asm_ex="auto", .hex_ex=0x00F9C104 },
            [ISA_ARM64] = { .isa=ISA_ARM64, .opcode=0x93407CC1, .asm_ex="auto", .hex_ex=0x93407CC1 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0x430007, .asm_ex="auto", .hex_ex=0x00430007 },
        },
    },

    [SEM_ID_ADDI] = { .id=SEM_ID_ADDI, .desc="立即数加法", .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x510093, .asm_ex="auto", .hex_ex=0x00510093 },
            [ISA_X86_64] = { .isa=ISA_X86_64, .opcode=0x4883C005, .asm_ex="auto", .hex_ex=0x4883C005 },
            [ISA_ARM64] = { .isa=ISA_ARM64, .opcode=0x91001441, .asm_ex="auto", .hex_ex=0x91001441 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0x20410010, .asm_ex="auto", .hex_ex=0x20410010 },
        },
    },

    [SEM_ID_LW] = { .id=SEM_ID_LW, .desc="加载字", .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x12083, .asm_ex="auto", .hex_ex=0x00012083 },
            [ISA_X86_64] = { .isa=ISA_X86_64, .opcode=0x8B0A, .asm_ex="auto", .hex_ex=0x00008B0A },
            [ISA_ARM64] = { .isa=ISA_ARM64, .opcode=0xB9400041, .asm_ex="auto", .hex_ex=0xB9400041 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0x8C410000, .asm_ex="auto", .hex_ex=0x8C410000 },
        },
    },

    [SEM_ID_SW] = { .id=SEM_ID_SW, .desc="存储字", .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x112023, .asm_ex="auto", .hex_ex=0x00112023 },
            [ISA_X86_64] = { .isa=ISA_X86_64, .opcode=0x8901, .asm_ex="auto", .hex_ex=0x00008901 },
            [ISA_ARM64] = { .isa=ISA_ARM64, .opcode=0xB9000041, .asm_ex="auto", .hex_ex=0xB9000041 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0xAC410000, .asm_ex="auto", .hex_ex=0xAC410000 },
        },
    },

    [SEM_ID_BEQ] = { .id=SEM_ID_BEQ, .desc="相等则分支", .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x208063, .asm_ex="auto", .hex_ex=0x00208063 },
            [ISA_X86_64] = { .isa=ISA_X86_64, .opcode=0x7400, .asm_ex="auto", .hex_ex=0x00007400 },
            [ISA_ARM64] = { .isa=ISA_ARM64, .opcode=0x54000040, .asm_ex="auto", .hex_ex=0x54000040 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0x10220000, .asm_ex="auto", .hex_ex=0x10220000 },
        },
    },

    [SEM_ID_JAL] = { .id=SEM_ID_JAL, .desc="跳转并链接", .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0xEF, .asm_ex="auto", .hex_ex=0x000000EF },
            [ISA_X86_64] = { .isa=ISA_X86_64, .opcode=0xE8000000, .asm_ex="auto", .hex_ex=0xE8000000 },
            [ISA_ARM64] = { .isa=ISA_ARM64, .opcode=0x14000000, .asm_ex="auto", .hex_ex=0x14000000 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0xC100000, .asm_ex="auto", .hex_ex=0x0C100000 },
        },
    },

    [SEM_ID_LUI] = { .id=SEM_ID_LUI, .desc="加载高位立即数", .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x800000B7, .asm_ex="auto", .hex_ex=0x800000B7 },
            [ISA_X86_64] = { .isa=ISA_X86_64, .opcode=0x48B80000, .asm_ex="auto", .hex_ex=0x48B80000 },
            [ISA_ARM64] = { .isa=ISA_ARM64, .opcode=0xD2800000, .asm_ex="auto", .hex_ex=0xD2800000 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0x3C011000, .asm_ex="auto", .hex_ex=0x3C011000 },
        },
    },

};


static const int INSTR_TABLE_SIZE = sizeof(INSTR_TABLE) / sizeof(INSTR_TABLE[0]);

const sem_instr_t* mlll_isa_get(sem_id_t id) {
    if (id < 0 || id >= SEM_ID_COUNT) return NULL;
    return &INSTR_TABLE[id];
}

const sem_instr_t* mlll_isa_get_by_name(const char* name) {
    for (int i = 0; i < SEM_ID_COUNT; i++)
        if (strcasecmp(SEM_NAMES[i], name) == 0) return &INSTR_TABLE[i];
    return NULL;
}

const isa_encoding_t* mlll_isa_get_encoding(sem_id_t id, isa_t isa) {
    const sem_instr_t* s = mlll_isa_get(id);
    if (!s || isa < 0 || isa >= ISA_COUNT) return NULL;
    if (s->enc[isa].hex_ex == 0) return NULL;
    return &s->enc[isa];
}

int mlll_isa_selftest(void) {
    int errors = 0;
    for (int i = 0; i < SEM_ID_COUNT; i++) {
        const isa_encoding_t* e = &INSTR_TABLE[i].enc[ISA_RV32I];
        if (!e->asm_ex || e->hex_ex == 0) {
            printf("[自检] 错误: %s 缺失 RV32I 定义\n", SEM_NAMES[i]);
            errors++;
        }
    }
    if (errors == 0) printf("[自检] %d 条语义指令 RV32I 全部就绪\n", SEM_ID_COUNT);
    return errors;
}

#endif /* MLLL_ISA_TABLE_C */
