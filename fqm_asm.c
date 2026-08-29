/*
 * fqm_asm.c — 意交感息汇编器/反汇编器
 * 
 * 功能：
 *   汇编：将符号文件编译成二进制文件
 *   反汇编：将二进制文件还原为符号文件
 *   查看：显示二进制文件内容
 * 
 * 编译：clang -o fqm_asm fqm_asm.c
 * 运行：
 *   ./fqm_asm asm <输入符号文件> <输出二进制文件>   # 汇编
 *   ./fqm_asm dis <输入二进制文件> <输出符号文件>   # 反汇编
 *   ./fqm_asm show <输入二进制文件>                 # 查看二进制
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define MAX_SYMBOLS 512
#define MAX_LINE 1024
#define MAGIC_NUM 0x46514D00  /* 'FQM\0' */

/* ========== 操作码定义 ========== */
typedef enum {
    OP_NOP     = 0x00,
    OP_READ    = 0x01,
    OP_WRITE   = 0x02,
    OP_LOAD    = 0x03,
    OP_ADD     = 0x04,
    OP_SUB     = 0x05,
    OP_MUL     = 0x06,
    OP_DIV     = 0x07,
    OP_AND     = 0x08,
    OP_OR      = 0x09,
    OP_XOR     = 0x0A,
    OP_NOT     = 0x0B,
    OP_SHL     = 0x0C,
    OP_SHR     = 0x0D,
    OP_JMP     = 0x0E,
    OP_JZ      = 0x0F,
    OP_JNZ     = 0x10,
    OP_CALL    = 0x11,
    OP_RET     = 0x12,
    OP_PUSH    = 0x13,
    OP_POP     = 0x14,
    OP_CMP     = 0x15,
    OP_IN      = 0x16,
    OP_OUT     = 0x17,
    OP_MIRROR  = 0x18,
    OP_EMPTY   = 0x19,
    OP_FILTER  = 0x1A,
    OP_TRANSMIT= 0x1B,
    OP_HALT    = 0x1C,
    OP_UNKNOWN = 0xFF
} OpType;

/* ========== 符号结构 ========== */
typedef struct {
    char name[64];
    char radical[16];
    int  strokes;
    char initial[8];
    char final[16];
    int  tone;
    char wubi[8];
    OpType op;
    int  src_addr;
    int  dst_addr;
    int  mode;
    int  immediate;
} Symbol;

/* ========== 二进制指令格式 ========== */
typedef struct {
    unsigned char magic[4];    /* 'FQM\0' */
    unsigned short version;    /* 版本号 */
    unsigned short num_syms;   /* 符号数量 */
    unsigned short entry_point;/* 入口点 */
    unsigned char reserved[4]; /* 保留 */
} BinHeader;

typedef struct {
    unsigned char op;          /* 操作码 */
    unsigned char src;         /* 源地址 */
    unsigned char dst;         /* 目标地址 */
    unsigned char imm;         /* 立即数 */
} BinInstr;

/* ========== 全局变量 ========== */
static Symbol sym_table[MAX_SYMBOLS];
static int sym_count = 0;

/* ========== 偏旁到操作码 ========== */
OpType radical_to_op(const char *radical) {
    if (radical[0] == '\0') return OP_NOP;
    if (strcmp(radical, "贝") == 0) return OP_READ;
    if (strcmp(radical, "扌") == 0) return OP_WRITE;
    if (strcmp(radical, "讠") == 0) return OP_LOAD;
    if (strcmp(radical, "辶") == 0) return OP_JMP;
    if (strcmp(radical, "口") == 0) return OP_CALL;
    if (strcmp(radical, "囗") == 0) return OP_RET;
    if (strcmp(radical, "人") == 0) return OP_MIRROR;
    if (strcmp(radical, "穴") == 0) return OP_EMPTY;
    if (strcmp(radical, "氵") == 0) return OP_FILTER;
    if (strcmp(radical, "火") == 0) return OP_TRANSMIT;
    if (strcmp(radical, "力") == 0) return OP_ADD;
    if (strcmp(radical, "刀") == 0) return OP_SUB;
    if (strcmp(radical, "又") == 0) return OP_MUL;
    if (strcmp(radical, "厶") == 0) return OP_DIV;
    if (strcmp(radical, "一") == 0) return OP_AND;
    if (strcmp(radical, "二") == 0) return OP_OR;
    if (strcmp(radical, "三") == 0) return OP_XOR;
    if (strcmp(radical, "非") == 0) return OP_NOT;
    if (strcmp(radical, "上") == 0) return OP_SHL;
    if (strcmp(radical, "下") == 0) return OP_SHR;
    if (strcmp(radical, "石") == 0) return OP_JZ;
    if (strcmp(radical, "目") == 0) return OP_JNZ;
    if (strcmp(radical, "丁") == 0) return OP_PUSH;
    if (strcmp(radical, "寸") == 0) return OP_POP;
    if (strcmp(radical, "比") == 0) return OP_CMP;
    if (strcmp(radical, "入") == 0) return OP_IN;
    if (strcmp(radical, "出") == 0) return OP_OUT;
    if (strcmp(radical, "止") == 0) return OP_HALT;
    return OP_UNKNOWN;
}

/* ========== 操作码到偏旁 ========== */
const char* op_to_radical(OpType op) {
    switch (op) {
        case OP_NOP: return "空";
        case OP_READ: return "贝";
        case OP_WRITE: return "扌";
        case OP_LOAD: return "讠";
        case OP_JMP: return "辶";
        case OP_CALL: return "口";
        case OP_RET: return "囗";
        case OP_MIRROR: return "人";
        case OP_EMPTY: return "穴";
        case OP_FILTER: return "氵";
        case OP_TRANSMIT: return "火";
        case OP_ADD: return "力";
        case OP_SUB: return "刀";
        case OP_MUL: return "又";
        case OP_DIV: return "厶";
        case OP_AND: return "一";
        case OP_OR: return "二";
        case OP_XOR: return "三";
        case OP_NOT: return "非";
        case OP_SHL: return "上";
        case OP_SHR: return "下";
        case OP_JZ: return "石";
        case OP_JNZ: return "目";
        case OP_PUSH: return "丁";
        case OP_POP: return "寸";
        case OP_CMP: return "比";
        case OP_IN: return "入";
        case OP_OUT: return "出";
        case OP_HALT: return "止";
        default: return "未";
    }
}

/* ========== 操作码到名称 ========== */
const char* op_to_name(OpType op) {
    switch (op) {
        case OP_NOP: return "NOP";
        case OP_READ: return "READ";
        case OP_WRITE: return "WRITE";
        case OP_LOAD: return "LOAD";
        case OP_ADD: return "ADD";
        case OP_SUB: return "SUB";
        case OP_MUL: return "MUL";
        case OP_DIV: return "DIV";
        case OP_AND: return "AND";
        case OP_OR: return "OR";
        case OP_XOR: return "XOR";
        case OP_NOT: return "NOT";
        case OP_SHL: return "SHL";
        case OP_SHR: return "SHR";
        case OP_JMP: return "JMP";
        case OP_JZ: return "JZ";
        case OP_JNZ: return "JNZ";
        case OP_CALL: return "CALL";
        case OP_RET: return "RET";
        case OP_PUSH: return "PUSH";
        case OP_POP: return "POP";
        case OP_CMP: return "CMP";
        case OP_IN: return "IN";
        case OP_OUT: return "OUT";
        case OP_MIRROR: return "MIRROR";
        case OP_EMPTY: return "EMPTY";
        case OP_FILTER: return "FILTER";
        case OP_TRANSMIT: return "TRANSMIT";
        case OP_HALT: return "HALT";
        default: return "???";
    }
}

/* ========== 声母到地址 ========== */
int initial_to_addr(const char *initial) {
    static const char *initials[] = {
        "b","p","m","f","d","t","n","l",
        "g","k","h","j","q","x","zh","ch",
        "sh","r","z","c","s","y","w", NULL
    };
    for (int i = 0; initials[i] != NULL; i++) {
        if (strcmp(initial, initials[i]) == 0) return i + 1;
    }
    return 0;
}

/* ========== 韵母到地址 ========== */
int final_to_addr(const char *final) {
    static const char *finals[] = {
        "a","o","e","i","u","v",
        "ai","ei","ui","ao","ou","iu",
        "ie","ve","er","an","en","in","un","vn",
        "ang","eng","ing","ong",
        NULL
    };
    for (int i = 0; finals[i] != NULL; i++) {
        if (strcmp(final, finals[i]) == 0) return i + 1;
    }
    return 0;
}

/* ========== 声调到模式 ========== */
int tone_to_mode(int tone) {
    switch (tone) {
        case 1: return 0;
        case 2: return 1;
        case 3: return 2;
        case 4: return 3;
        default: return 0;
    }
}

/* ========== 添加符号 ========== */
int add_symbol(const char *name, const char *radical, int strokes,
               const char *initial, const char *final, int tone,
               const char *wubi) {
    if (sym_count >= MAX_SYMBOLS) return -1;
    
    Symbol *s = &sym_table[sym_count];
    memset(s, 0, sizeof(Symbol));
    strncpy(s->name, name, sizeof(s->name) - 1);
    strncpy(s->radical, radical, sizeof(s->radical) - 1);
    s->strokes = strokes;
    strncpy(s->initial, initial, sizeof(s->initial) - 1);
    strncpy(s->final, final, sizeof(s->final) - 1);
    s->tone = tone;
    strncpy(s->wubi, wubi, sizeof(s->wubi) - 1);
    
    s->op = radical_to_op(radical);
    s->src_addr = initial_to_addr(initial);
    s->dst_addr = final_to_addr(final);
    s->mode = tone_to_mode(tone);
    s->immediate = strokes;
    
    sym_count++;
    return 0;
}

/* ========== 解析符号文件 ========== */
int parse_line(char *line) {
    char name[64], radical[16], initial[8], final[16], wubi[8];
    int strokes, tone;
    
    if (sscanf(line, "%63s %15s %d %7s %15s %d %7s",
               name, radical, &strokes, initial, final, &tone, wubi) == 7) {
        return add_symbol(name, radical, strokes, initial, final, tone, wubi);
    }
    return -1;
}

int load_symbol_file(const char *path) {
    FILE *fp = fopen(path, "r");
    if (!fp) {
        fprintf(stderr, "错误：无法打开文件 %s\n", path);
        return -1;
    }
    
    char line[MAX_LINE];
    int line_num = 0;
    
    while (fgets(line, sizeof(line), fp)) {
        line_num++;
        line[strcspn(line, "\n")] = '\0';
        if (line[0] == '\0' || line[0] == '#') continue;
        
        if (parse_line(line) != 0) {
            fprintf(stderr, "警告：第%d行解析失败: %s\n", line_num, line);
        }
    }
    
    fclose(fp);
    return 0;
}

/* ========== 汇编：符号文件 → 二进制文件 ========== */
int assemble(const char *input, const char *output) {
    printf("汇编: %s → %s\n", input, output);
    
    sym_count = 0;
    if (load_symbol_file(input) != 0) {
        return -1;
    }
    
    FILE *fp = fopen(output, "wb");
    if (!fp) {
        fprintf(stderr, "错误：无法创建文件 %s\n", output);
        return -1;
    }
    
    /* 写入头部 */
    BinHeader header;
    memcpy(header.magic, "FQM", 3);
    header.magic[3] = 0;
    header.version = 0x0001;
    header.num_syms = sym_count;
    header.entry_point = 0;
    memset(header.reserved, 0, sizeof(header.reserved));
    
    fwrite(&header, sizeof(header), 1, fp);
    
    /* 写入指令 */
    for (int i = 0; i < sym_count; i++) {
        Symbol *s = &sym_table[i];
        BinInstr instr;
        instr.op = s->op;
        instr.src = s->src_addr;
        instr.dst = s->dst_addr;
        instr.imm = s->immediate;
        
        fwrite(&instr, sizeof(instr), 1, fp);
    }
    
    fclose(fp);
    
    printf("汇编完成: %d 个符号 → %ld 字节\n", 
           sym_count, sizeof(BinHeader) + sym_count * sizeof(BinInstr));
    
    return 0;
}

/* ========== 反汇编：二进制文件 → 符号文件 ========== */
int disassemble(const char *input, const char *output) {
    printf("反汇编: %s → %s\n", input, output);
    
    FILE *fp = fopen(input, "rb");
    if (!fp) {
        fprintf(stderr, "错误：无法打开文件 %s\n", input);
        return -1;
    }
    
    /* 读取头部 */
    BinHeader header;
    if (fread(&header, sizeof(header), 1, fp) != 1) {
        fprintf(stderr, "错误：读取头部失败\n");
        fclose(fp);
        return -1;
    }
    
    if (memcmp(header.magic, "FQM", 3) != 0) {
        fprintf(stderr, "错误：不是有效的FQM文件\n");
        fclose(fp);
        return -1;
    }
    
    printf("FQM文件: 版本=%d, 符号数=%d, 入口=%d\n",
           header.version, header.num_syms, header.entry_point);
    
    /* 读取指令 */
    FILE *out = fopen(output, "w");
    if (!out) {
        fprintf(stderr, "错误：无法创建文件 %s\n", output);
        fclose(fp);
        return -1;
    }
    
    fprintf(out, "# 反汇编输出\n");
    fprintf(out, "# 源文件: %s\n", input);
    fprintf(out, "# 符号数: %d\n\n", header.num_syms);
    
    for (int i = 0; i < header.num_syms; i++) {
        BinInstr instr;
        if (fread(&instr, sizeof(instr), 1, fp) != 1) {
            fprintf(stderr, "警告：指令读取提前结束\n");
            break;
        }
        
        const char *radical = op_to_radical(instr.op);
        const char *name = op_to_name(instr.op);
        
        fprintf(out, "%-8s %-6s %-4d %-6s %-8s %-4d %-6s\n",
                name, radical, instr.imm, "a", "o", 1, "xxx");
    }
    
    fclose(fp);
    fclose(out);
    
    printf("反汇编完成: %d 个符号\n", header.num_syms);
    
    return 0;
}

/* ========== 查看二进制文件 ========== */
int show_binary(const char *input) {
    printf("查看二进制文件: %s\n", input);
    
    FILE *fp = fopen(input, "rb");
    if (!fp) {
        fprintf(stderr, "错误：无法打开文件 %s\n", input);
        return -1;
    }
    
    /* 读取头部 */
    BinHeader header;
    if (fread(&header, sizeof(header), 1, fp) != 1) {
        fprintf(stderr, "错误：读取头部失败\n");
        fclose(fp);
        return -1;
    }
    
    if (memcmp(header.magic, "FQM", 3) != 0) {
        fprintf(stderr, "错误：不是有效的FQM文件\n");
        fclose(fp);
        return -1;
    }
    
    printf("\n========== FQM文件头 ==========\n");
    printf("Magic: %c%c%c\\0\n", header.magic[0], header.magic[1], header.magic[2]);
    printf("版本: %d\n", header.version);
    printf("符号数: %d\n", header.num_syms);
    printf("入口点: %d\n", header.entry_point);
    printf("文件大小: %ld 字节\n", sizeof(BinHeader) + header.num_syms * sizeof(BinInstr));
    
    printf("\n========== 指令表 ==========\n");
    printf("%-4s %-8s %-6s %-6s %-6s %-6s\n",
           "序号", "名称", "操作码", "源", "目标", "立即数");
    printf("----------------------------------------------\n");
    
    for (int i = 0; i < header.num_syms; i++) {
        BinInstr instr;
        if (fread(&instr, sizeof(instr), 1, fp) != 1) {
            break;
        }
        
        printf("%-4d %-8s %-6X %-6X %-6X %-6X\n",
               i, op_to_name(instr.op), instr.op, instr.src, instr.dst, instr.imm);
    }
    
    fclose(fp);
    return 0;
}

/* ========== 主函数 ========== */
int main(int argc, char *argv[]) {
    if (argc < 3) {
        printf("意交感息汇编器 v0.1\n\n");
        printf("用法:\n");
        printf("  ./fqm_asm asm <输入符号文件> <输出二进制文件>\n");
        printf("  ./fqm_asm dis <输入二进制文件> <输出符号文件>\n");
        printf("  ./fqm_asm show <输入二进制文件>\n");
        return 1;
    }
    
    const char *cmd = argv[1];
    
    if (strcmp(cmd, "asm") == 0) {
        if (argc != 4) {
            printf("用法: ./fqm_asm asm <输入符号文件> <输出二进制文件>\n");
            return 1;
        }
        return assemble(argv[2], argv[3]);
    } else if (strcmp(cmd, "dis") == 0) {
        if (argc != 4) {
            printf("用法: ./fqm_asm dis <输入二进制文件> <输出符号文件>\n");
            return 1;
        }
        return disassemble(argv[2], argv[3]);
    } else if (strcmp(cmd, "show") == 0) {
        if (argc != 3) {
            printf("用法: ./fqm_asm show <输入二进制文件>\n");
            return 1;
        }
        return show_binary(argv[2]);
    } else {
        printf("未知命令: %s\n", cmd);
        return 1;
    }
}
