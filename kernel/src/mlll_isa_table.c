/**
 * mlll_isa_table.c — 语义指令 ID → 多 ISA 机器码映射表
 * ------------------------------------------------
 * 对标文章里的"跨平台机器码使用同一个机器码定义同一个功能"。
 *
 * 设计原则：
 *   - 语义层是统一的（ADD / SUB / AND / OR / LW / SW / BEQ / JAL ...）
 *   - 每个语义 ID 挂载所有目标 ISA 的编码模板
 *   - 编码模板 = opcode + 功能位域 + 操作数槽位
 *   - 汇编器后端根据目标 ISA 选择对应模板生成机器码
 *
 * 这也是一套"底层符号注册表"，和 mlll_symbols 的七维符号系统互补：
 *   mlll_symbols  = 汉字的几何/声调/笔画属性
 *   mlll_isa      = 指令的语义/位域/寄存器属性
 */

#ifndef MLLL_ISA_TABLE_C
#define MLLL_ISA_TABLE_C

#define _POSIX_C_SOURCE 200809L
#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include <strings.h>   /* strcasecmp on Linux */

/* ========== ISA 目标枚举 ========== */

typedef enum {
    ISA_RV32I = 0,   /* RISC-V 32 位基础整数 */
    ISA_RV64I,       /* RISC-V 64 位基础整数 */
    ISA_X86_32,      /* x86 32 位 */
    ISA_X86_64,      /* x86 64 位 */
    ISA_ARM32,       /* ARM 32 位 (AArch32) */
    ISA_ARM64,       /* ARM 64 位 (AArch64) */
    ISA_MIPS32,      /* MIPS 32 位 */
    ISA_COUNT
} isa_t;

static const char* ISA_NAMES[] = {
    "rv32i", "rv64i", "x86", "x86_64", "arm32", "arm64", "mips32"
};

/* ========== 指令格式类型 ========== */

typedef enum {
    FMT_R = 0,   /* Register: funct7 rs2 rs1 funct3 rd opcode */
    FMT_I,       /* Immediate: imm[11:0] rs1 funct3 rd opcode */
    FMT_S,       /* Store: imm[11:5] rs2 rs1 funct3 imm[4:0] opcode */
    FMT_B,       /* Branch: imm[12|10:5] rs2 rs1 funct3 imm[4:1|11] opcode */
    FMT_U,       /* Upper: imm[31:12] rd opcode */
    FMT_J,       /* Jump: imm[20|10:1|11|19:12] rd opcode */
    FMT_X86,     /* x86 变长 opcode+modrm+sib+imm */
    FMT_ARM,     /* ARM 固定 32 位 condition+op+fields */
    FMT_MIPS     /* MIPS 固定 32 位 opcode+rs+rt+rd+shamt+funct */
} fmt_t;

/* ========== ISA 编码模板 ========== */

typedef struct {
    isa_t      isa;
    uint32_t   opcode;       /* 主操作码基值 */
    uint32_t   funct3;       /* RISC-V funct3 / x86 扩展 / MIPS funct */
    uint32_t   funct7;       /* RISC-V funct7 */
    fmt_t      format;
    const char* asm_ex;      /* 汇编示例 */
    /* 十六进制示例 (常用操作数组合) */
    uint32_t   hex_ex;
} isa_encoding_t;

/* ========== 语义指令 ID ========== */

typedef enum {
    SEM_ADD = 0,
    SEM_SUB,
    SEM_AND,
    SEM_OR,
    SEM_XOR,
    SEM_SLT,
    SEM_SLTU,
    SEM_SLL,
    SEM_SRL,
    SEM_SRA,
    SEM_ADDI,
    SEM_LW,
    SEM_SW,
    SEM_BEQ,
    SEM_JAL,
    SEM_LUI,
    SEM_COUNT
} sem_id_t;

static const char* SEM_NAMES[] = {
    "ADD", "SUB", "AND", "OR", "XOR", "SLT", "SLTU",
    "SLL", "SRL", "SRA", "ADDI", "LW", "SW", "BEQ", "JAL", "LUI"
};

/* ========== 指令描述 ========== */

typedef struct {
    sem_id_t         id;
    const char*      desc;        /* 自然语言含义 */
    isa_encoding_t   enc[ISA_COUNT];
    int              enc_count;
} sem_instr_t;

/* ========== 寄存器名表 ========== */

static const char* RV_ABI[32] = {
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2",
    "s0", "s1", "a0", "a1", "a2", "a3", "a4", "a5",
    "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7",
    "s8", "s9", "s10", "s11", "t3", "t4", "t5", "t6"
};

/* ========== 核心语义-机器码表 ========== */

static sem_instr_t INSTR_TABLE[] = {
    /* ADD rd, rs1, rs2 — 整数加法 */
    [SEM_ADD] = {
        .id = SEM_ADD, .desc = "整数加法 rd = rs1 + rs2",
        .enc = {
            /* RISC-V: add rd, rs1, rs2 = 0000000 rs2 rs1 000 rd 0110011 */
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x33, .funct3=0, .funct7=0,
                .format=FMT_R, .asm_ex="add x1, x2, x3", .hex_ex=0x003100B3 },
            [ISA_RV64I] = { .isa=ISA_RV64I, .opcode=0x33, .funct3=0, .funct7=0,
                .format=FMT_R, .asm_ex="add x1, x2, x3", .hex_ex=0x003100B3 },
            /* x86: add r/m32, r32 = 01 /r  (mod=3 reg=src rm=dst) */
            [ISA_X86_32] = { .isa=ISA_X86_32, .opcode=0x01, .funct3=0, .funct7=0,
                .format=FMT_X86, .asm_ex="add eax, ebx", .hex_ex=0xD801 },
            [ISA_X86_64] = { .isa=ISA_X86_64, .opcode=0x01, .funct3=0, .funct7=0,
                .format=FMT_X86, .asm_ex="add rax, rbx", .hex_ex=0xD801 },
            /* ARM64: add xd, xn, xm = 1000 1011 0000 0000 0000 0000 0000 0000 */
            [ISA_ARM64] = { .isa=ISA_ARM64, .opcode=0x8B000000, .funct3=0, .funct7=0,
                .format=FMT_ARM, .asm_ex="add x1, x2, x3", .hex_ex=0x8B030041 },
            /* MIPS: add rd, rs, rt = 0000 00ss ssst tttt dddd d000 0010 0000 */
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0, .funct3=0, .funct7=0x20,
                .format=FMT_MIPS, .asm_ex="add $1, $2, $3", .hex_ex=0x00430820 },
        }, .enc_count = ISA_COUNT
    },

    /* SUB rd, rs1, rs2 — 整数减法 */
    [SEM_SUB] = {
        .id = SEM_SUB, .desc = "整数减法 rd = rs1 - rs2",
        .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x33, .funct3=0, .funct7=0x20,
                .format=FMT_R, .asm_ex="sub x1, x2, x3", .hex_ex=0x403100B3 },
            [ISA_RV64I] = { .isa=ISA_RV64I, .opcode=0x33, .funct3=0, .funct7=0x20,
                .format=FMT_R, .asm_ex="sub x1, x2, x3", .hex_ex=0x403100B3 },
            [ISA_X86_32] = { .isa=ISA_X86_32, .opcode=0x29, .funct3=0, .funct7=0,
                .format=FMT_X86, .asm_ex="sub eax, ebx", .hex_ex=0xD829 },
            [ISA_ARM64] = { .isa=ISA_ARM64, .opcode=0xCB000000, .funct3=0, .funct7=0,
                .format=FMT_ARM, .asm_ex="sub x1, x2, x3", .hex_ex=0xCB030041 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0, .funct3=0, .funct7=0x22,
                .format=FMT_MIPS, .asm_ex="sub $1, $2, $3", .hex_ex=0x00430822 },
        }, .enc_count = ISA_COUNT
    },

    /* AND rd, rs1, rs2 — 按位与 */
    [SEM_AND] = {
        .id = SEM_AND, .desc = "按位与",
        .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x33, .funct3=7, .funct7=0,
                .format=FMT_R, .asm_ex="and x1, x2, x3", .hex_ex=0x003170B3 },
            [ISA_ARM64] = { .isa=ISA_ARM64, .opcode=0x8A000000, .funct3=0, .funct7=0,
                .format=FMT_ARM, .asm_ex="and x1, x2, x3", .hex_ex=0x8A030041 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0, .funct3=0, .funct7=0x24,
                .format=FMT_MIPS, .asm_ex="and $1, $2, $3", .hex_ex=0x00430824 },
        }, .enc_count = ISA_COUNT
    },

    /* OR rd, rs1, rs2 — 按位或 */
    [SEM_OR] = {
        .id = SEM_OR, .desc = "按位或",
        .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x33, .funct3=6, .funct7=0,
                .format=FMT_R, .asm_ex="or x1, x2, x3", .hex_ex=0x003160B3 },
            [ISA_ARM64] = { .isa=ISA_ARM64, .opcode=0xAA000000, .funct3=0, .funct7=0,
                .format=FMT_ARM, .asm_ex="orr x1, x2, x3", .hex_ex=0xAA030041 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0, .funct3=0, .funct7=0x25,
                .format=FMT_MIPS, .asm_ex="or $1, $2, $3", .hex_ex=0x00430825 },
        }, .enc_count = ISA_COUNT
    },

    /* XOR rd, rs1, rs2 — 按位异或 */
    [SEM_XOR] = {
        .id = SEM_XOR, .desc = "按位异或",
        .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x33, .funct3=4, .funct7=0,
                .format=FMT_R, .asm_ex="xor x1, x2, x3", .hex_ex=0x003140B3 },
            [ISA_ARM64] = { .isa=ISA_ARM64, .opcode=0xCA000000, .funct3=0, .funct7=0,
                .format=FMT_ARM, .asm_ex="eor x1, x2, x3", .hex_ex=0xCA030041 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0, .funct3=0, .funct7=0x26,
                .format=FMT_MIPS, .asm_ex="xor $1, $2, $3", .hex_ex=0x00430826 },
        }, .enc_count = ISA_COUNT
    },

    /* SLT rd, rs1, rs2 — 有符号小于置位 */
    [SEM_SLT] = {
        .id = SEM_SLT, .desc = "有符号小于置位 rd=(rs1<rs2)?1:0",
        .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x33, .funct3=2, .funct7=0,
                .format=FMT_R, .asm_ex="slt x1, x2, x3", .hex_ex=0x003120B3 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0, .funct3=0, .funct7=0x2A,
                .format=FMT_MIPS, .asm_ex="slt $1, $2, $3", .hex_ex=0x0043082A },
        }, .enc_count = ISA_COUNT
    },

    /* SLTU rd, rs1, rs2 — 无符号小于置位 */
    [SEM_SLTU] = {
        .id = SEM_SLTU, .desc = "无符号小于置位",
        .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x33, .funct3=3, .funct7=0,
                .format=FMT_R, .asm_ex="sltu x1, x2, x3", .hex_ex=0x003130B3 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0, .funct3=0, .funct7=0x2B,
                .format=FMT_MIPS, .asm_ex="sltu $1, $2, $3", .hex_ex=0x0043082B },
        }, .enc_count = ISA_COUNT
    },

    /* SLL rd, rs1, rs2 — 逻辑左移 */
    [SEM_SLL] = {
        .id = SEM_SLL, .desc = "逻辑左移",
        .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x33, .funct3=1, .funct7=0,
                .format=FMT_R, .asm_ex="sll x1, x2, x3", .hex_ex=0x003110B3 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0, .funct3=0, .funct7=0x00,
                .format=FMT_MIPS, .asm_ex="sll $1, $2, $3", .hex_ex=0x00030800 },
        }, .enc_count = ISA_COUNT
    },

    /* SRL rd, rs1, rs2 — 逻辑右移 */
    [SEM_SRL] = {
        .id = SEM_SRL, .desc = "逻辑右移",
        .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x33, .funct3=5, .funct7=0,
                .format=FMT_R, .asm_ex="srl x1, x2, x3", .hex_ex=0x003150B3 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0, .funct3=0, .funct7=0x02,
                .format=FMT_MIPS, .asm_ex="srl $1, $2, $3", .hex_ex=0x00030802 },
        }, .enc_count = ISA_COUNT
    },

    /* SRA rd, rs1, rs2 — 算术右移 */
    [SEM_SRA] = {
        .id = SEM_SRA, .desc = "算术右移",
        .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x33, .funct3=5, .funct7=0x20,
                .format=FMT_R, .asm_ex="sra x1, x2, x3", .hex_ex=0x403150B3 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0, .funct3=0, .funct7=0x03,
                .format=FMT_MIPS, .asm_ex="sra $1, $2, $3", .hex_ex=0x00030803 },
        }, .enc_count = ISA_COUNT
    },

    /* ADDI rd, rs1, imm — 立即数加法 */
    [SEM_ADDI] = {
        .id = SEM_ADDI, .desc = "立即数加法 rd = rs1 + imm",
        .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x13, .funct3=0, .funct7=0,
                .format=FMT_I, .asm_ex="addi x1, x2, 5", .hex_ex=0x00510093 },
            [ISA_X86_64] = { .isa=ISA_X86_64, .opcode=0x83, .funct3=0, .funct7=0,
                .format=FMT_X86, .asm_ex="add rax, 5", .hex_ex=0x054883 },
            [ISA_ARM64] = { .isa=ISA_ARM64, .opcode=0x91000000, .funct3=0, .funct7=0,
                .format=FMT_ARM, .asm_ex="add x1, x2, #5", .hex_ex=0x91001441 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0x08, .funct3=0, .funct7=0,
                .format=FMT_MIPS, .asm_ex="addi $1, $2, 5", .hex_ex=0x20410005 },
        }, .enc_count = ISA_COUNT
    },

    /* LW rd, offset(rs1) — 加载字 */
    [SEM_LW] = {
        .id = SEM_LW, .desc = "从内存加载 32 位字",
        .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x03, .funct3=2, .funct7=0,
                .format=FMT_I, .asm_ex="lw x1, 0(x2)", .hex_ex=0x00012083 },
            [ISA_ARM64] = { .isa=ISA_ARM64, .opcode=0xB9400000, .funct3=0, .funct7=0,
                .format=FMT_ARM, .asm_ex="ldr w1, [x2]", .hex_ex=0xB9400041 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0x23, .funct3=0, .funct7=0,
                .format=FMT_MIPS, .asm_ex="lw $1, 0($2)", .hex_ex=0x8C410000 },
        }, .enc_count = ISA_COUNT
    },

    /* SW rs2, offset(rs1) — 存储字 */
    [SEM_SW] = {
        .id = SEM_SW, .desc = "将 32 位字存入内存",
        .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x23, .funct3=2, .funct7=0,
                .format=FMT_S, .asm_ex="sw x1, 0(x2)", .hex_ex=0x00112023 },
            [ISA_ARM64] = { .isa=ISA_ARM64, .opcode=0xB9000000, .funct3=0, .funct7=0,
                .format=FMT_ARM, .asm_ex="str w1, [x2]", .hex_ex=0xB9000041 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0x2B, .funct3=0, .funct7=0,
                .format=FMT_MIPS, .asm_ex="sw $1, 0($2)", .hex_ex=0xAC410000 },
        }, .enc_count = ISA_COUNT
    },

    /* BEQ rs1, rs2, label — 相等则跳转 */
    [SEM_BEQ] = {
        .id = SEM_BEQ, .desc = "相等则分支",
        .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x63, .funct3=0, .funct7=0,
                .format=FMT_B, .asm_ex="beq x1, x2, label", .hex_ex=0x00208063 },
            [ISA_ARM64] = { .isa=ISA_ARM64, .opcode=0x54000000, .funct3=0, .funct7=0,
                .format=FMT_ARM, .asm_ex="b.eq label", .hex_ex=0x54000040 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0x04, .funct3=0, .funct7=0,
                .format=FMT_MIPS, .asm_ex="beq $1, $2, label", .hex_ex=0x10220000 },
        }, .enc_count = ISA_COUNT
    },

    /* JAL rd, label — 跳转并链接 */
    [SEM_JAL] = {
        .id = SEM_JAL, .desc = "跳转并保存返回地址",
        .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x6F, .funct3=0, .funct7=0,
                .format=FMT_J, .asm_ex="jal x1, label", .hex_ex=0x000000EF },
            [ISA_X86_64] = { .isa=ISA_X86_64, .opcode=0xE8, .funct3=0, .funct7=0,
                .format=FMT_X86, .asm_ex="call label", .hex_ex=0xE8000000 },
            [ISA_ARM64] = { .isa=ISA_ARM64, .opcode=0x94000000, .funct3=0, .funct7=0,
                .format=FMT_ARM, .asm_ex="bl label", .hex_ex=0x94000000 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0x03, .funct3=0, .funct7=0,
                .format=FMT_MIPS, .asm_ex="jal label", .hex_ex=0x0C000000 },
        }, .enc_count = ISA_COUNT
    },

    /* LUI rd, imm — 加载高位立即数 */
    [SEM_LUI] = {
        .id = SEM_LUI, .desc = "加载高位立即数 rd = imm << 12",
        .enc = {
            [ISA_RV32I] = { .isa=ISA_RV32I, .opcode=0x37, .funct3=0, .funct7=0,
                .format=FMT_U, .asm_ex="lui x1, 0x12345", .hex_ex=0x123450B7 },
            [ISA_MIPS32] = { .isa=ISA_MIPS32, .opcode=0x0F, .funct3=0, .funct7=0,
                .format=FMT_MIPS, .asm_ex="lui $1, 0x1234", .hex_ex=0x3C011234 },
        }, .enc_count = ISA_COUNT
    },
};

static const int INSTR_TABLE_SIZE = sizeof(INSTR_TABLE) / sizeof(INSTR_TABLE[0]);

/* ========== 查询接口 ========== */

/**
 * 通过语义 ID 取指令描述
 */
const sem_instr_t* mlll_isa_get(sem_id_t id) {
    if (id < 0 || id >= SEM_COUNT) return NULL;
    return &INSTR_TABLE[id];
}

/**
 * 通过语义名称取指令描述 (不区分大小写)
 */
const sem_instr_t* mlll_isa_get_by_name(const char* name) {
    for (int i = 0; i < SEM_COUNT; i++) {
        if (strcasecmp(SEM_NAMES[i], name) == 0) {
            return &INSTR_TABLE[i];
        }
    }
    return NULL;
}

/**
 * 找到某 ISA 的编码模板
 */
const isa_encoding_t* mlll_isa_get_encoding(sem_id_t id, isa_t isa) {
    const sem_instr_t* s = mlll_isa_get(id);
    if (!s) return NULL;
    if (isa < 0 || isa >= ISA_COUNT) return NULL;
    /* 通过比较 hex_ex 判是否填充 (0 表示未定义该指令对该 ISA) */
    if (s->enc[isa].hex_ex == 0 && s->enc[isa].opcode == 0) return NULL;
    return &s->enc[isa];
}

/* ========== 打印 ========== */

void mlll_isa_dump(void) {
    printf("=== MLLL ISA 语义指令表 (共 %d 条) ===\n", SEM_COUNT);
    for (int i = 0; i < SEM_COUNT; i++) {
        const sem_instr_t* s = &INSTR_TABLE[i];
        printf("\n[%s] %s\n", SEM_NAMES[i], s->desc);
        for (int j = 0; j < ISA_COUNT; j++) {
            const isa_encoding_t* e = &s->enc[j];
            if (e->asm_ex) {
                printf("  %-8s : %-20s hex=%08X\n",
                    ISA_NAMES[j], e->asm_ex, e->hex_ex);
            }
        }
    }
}

void mlll_isa_dump_isa(isa_t isa) {
    printf("=== %s 指令编码表 ===\n", ISA_NAMES[isa]);
    for (int i = 0; i < SEM_COUNT; i++) {
        const isa_encoding_t* e = &INSTR_TABLE[i].enc[isa];
        if (e->asm_ex) {
            printf("  %-5s  %-22s  %08X  op=%02X\n",
                SEM_NAMES[i], e->asm_ex, e->hex_ex, e->opcode);
        }
    }
}

/* ========== 自检 ========== */

int mlll_isa_selftest(void) {
    /* 检查所有 SEM_ID 都有 RV32I 定义 */
    int errors = 0;
    for (int i = 0; i < SEM_COUNT; i++) {
        const isa_encoding_t* e = &INSTR_TABLE[i].enc[ISA_RV32I];
        if (!e->asm_ex || e->hex_ex == 0) {
            printf("[自检] 错误: %s 缺失 RV32I 定义\n", SEM_NAMES[i]);
            errors++;
        }
    }
    if (errors == 0) {
        printf("[自检] %d 条语义指令 RV32I 编码全部就绪\n", SEM_COUNT);
    }
    return errors;
}

#endif /* MLLL_ISA_TABLE_C */
