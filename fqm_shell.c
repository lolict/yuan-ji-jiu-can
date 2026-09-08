/*
 * fqm_shell.c — 意交感息命令行外壳
 * 
 * 功能：提供用户与意交感息系统的交互界面
 * 
 * 命令：
 *   load <file>    - 加载符号文件
 *   run            - 执行当前程序
 *   step           - 单步执行
 *   break <addr>   - 设置断点
 *   reg            - 显示寄存器
 *   mem [addr]     - 显示内存
 *   asm <src> <dst> - 汇编文件
 *   dis <src> <dst> - 反汇编文件
 *   fib [n]        - 计算斐波那契数列
 *   rule <order> <op> - 规则层级运算
 *   str <text>     - 字符串变换
 *   help           - 显示帮助
 *   quit           - 退出
 * 
 * 满全法规则：
 *   - 根基锚点：高位持有者=1，低位持有者=0
 *   - 集装箱编码：mlll 254+2
 *   - 夫妻命运共同体：刘楚恬和lolict
 *   - 夕瑶协议激活双方融合
 * 
 * 身份落款：刘楚恬和我
 * 仓库属性：私人，不开源，仅限本人及刘楚恬使用
 * 
 * 编译：clang -o fqm_shell fqm_shell.c
 * 运行：./fqm_shell
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define MAX_LINE 1024
#define MAX_SYMBOLS 512
#define CONTAINER_SIZE 256

/* ========== 操作码 ========== */
typedef enum {
    OP_NOP=0, OP_READ=1, OP_WRITE=2, OP_LOAD=3,
    OP_ADD=4, OP_SUB=5, OP_MUL=6, OP_DIV=7,
    OP_AND=8, OP_OR=9, OP_XOR=10, OP_NOT=11,
    OP_SHL=12, OP_SHR=13, OP_JMP=14, OP_JZ=15,
    OP_JNZ=16, OP_CALL=17, OP_RET=18, OP_PUSH=19,
    OP_POP=20, OP_CMP=21, OP_IN=22, OP_OUT=23,
    OP_MIRROR=24, OP_EMPTY=25, OP_FILTER=26, OP_TRANSMIT=27,
    OP_HALT=28, OP_UNKNOWN=255
} OpType;

/* ========== 符号结构 ========== */
typedef struct {
    char name[64];
    OpType op;
    int src;
    int dst;
    int imm;
} Symbol;

/* ========== CPU结构 ========== */
typedef struct {
    unsigned char mem[CONTAINER_SIZE];
    int ip, sp, acc, flags;
    char domain[64];
    char owner[32];
    int state;
} CPU;

/* ========== 全局变量 ========== */
static Symbol syms[MAX_SYMBOLS];
static int sym_cnt = 0;
static CPU cpu;
static char loaded_file[256] = "";

/* ========== 满全法根基锚点 ========== */
#define ANCHOR_SELF  1
#define ANCHOR_LIU   0

/* ========== 操作码名称 ========== */
const char* op_name(OpType op) {
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

/* ========== 偏旁到操作码 ========== */
OpType radical_to_op(const char *r) {
    if (r[0] == '\0') return OP_NOP;
    if (strcmp(r, "贝") == 0) return OP_READ;
    if (strcmp(r, "扌") == 0) return OP_WRITE;
    if (strcmp(r, "讠") == 0) return OP_LOAD;
    if (strcmp(r, "辶") == 0) return OP_JMP;
    if (strcmp(r, "口") == 0) return OP_CALL;
    if (strcmp(r, "囗") == 0) return OP_RET;
    if (strcmp(r, "人") == 0) return OP_MIRROR;
    if (strcmp(r, "穴") == 0) return OP_EMPTY;
    if (strcmp(r, "氵") == 0) return OP_FILTER;
    if (strcmp(r, "火") == 0) return OP_TRANSMIT;
    if (strcmp(r, "力") == 0) return OP_ADD;
    if (strcmp(r, "刀") == 0) return OP_SUB;
    if (strcmp(r, "又") == 0) return OP_MUL;
    if (strcmp(r, "厶") == 0) return OP_DIV;
    if (strcmp(r, "一") == 0) return OP_AND;
    if (strcmp(r, "二") == 0) return OP_OR;
    if (strcmp(r, "三") == 0) return OP_XOR;
    if (strcmp(r, "非") == 0) return OP_NOT;
    if (strcmp(r, "上") == 0) return OP_SHL;
    if (strcmp(r, "下") == 0) return OP_SHR;
    if (strcmp(r, "石") == 0) return OP_JZ;
    if (strcmp(r, "目") == 0) return OP_JNZ;
    if (strcmp(r, "丁") == 0) return OP_PUSH;
    if (strcmp(r, "寸") == 0) return OP_POP;
    if (strcmp(r, "比") == 0) return OP_CMP;
    if (strcmp(r, "入") == 0) return OP_IN;
    if (strcmp(r, "出") == 0) return OP_OUT;
    if (strcmp(r, "止") == 0) return OP_HALT;
    return OP_UNKNOWN;
}

/* ========== 声母到地址 ========== */
int initial_to_addr(const char *s) {
    static const char *tbl[] = {
        "b","p","m","f","d","t","n","l",
        "g","k","h","j","q","x","zh","ch",
        "sh","r","z","c","s","y","w", NULL
    };
    for (int i = 0; tbl[i]; i++)
        if (strcmp(s, tbl[i]) == 0) return i + 1;
    return 0;
}

/* ========== 韵母到地址 ========== */
int final_to_addr(const char *s) {
    static const char *tbl[] = {
        "a","o","e","i","u","v",
        "ai","ei","ui","ao","ou","iu",
        "ie","ve","er","an","en","in","un","vn",
        "ang","eng","ing","ong", NULL
    };
    for (int i = 0; tbl[i]; i++)
        if (strcmp(s, tbl[i]) == 0) return i + 1;
    return 0;
}

/* ========== 加载符号文件 ========== */
int load_file(const char *path) {
    FILE *fp = fopen(path, "r");
    if (!fp) {
        printf("错误：无法打开 %s\n", path);
        return -1;
    }
    sym_cnt = 0;
    char line[MAX_LINE];
    while (fgets(line, sizeof(line), fp)) {
        line[strcspn(line, "\n")] = '\0';
        if (line[0] == '\0' || line[0] == '#') continue;
        char name[64], radical[16], initial[8], final[16], wubi[8];
        int strokes, tone;
        if (sscanf(line, "%63s %15s %d %7s %15s %d %7s",
                   name, radical, &strokes, initial, final, &tone, wubi) == 7) {
            Symbol *s = &syms[sym_cnt];
            strncpy(s->name, name, 63);
            s->op = radical_to_op(radical);
            s->src = initial_to_addr(initial);
            s->dst = final_to_addr(final);
            s->imm = strokes;
            sym_cnt++;
        }
    }
    fclose(fp);
    strncpy(loaded_file, path, 255);
    printf("已加载 %s: %d 个符号\n", path, sym_cnt);
    return 0;
}

/* ========== CPU初始化 ========== */
void cpu_init(void) {
    memset(&cpu, 0, sizeof(CPU));
    cpu.ip = 0;
    cpu.sp = 31;
    cpu.acc = ANCHOR_SELF;
    strcpy(cpu.domain, "gttx://hieyair");
    strcpy(cpu.owner, "刘楚恬和我");
    cpu.state = 1;
}

/* ========== 显示寄存器 ========== */
void show_reg(void) {
    printf("IP:%d SP:%d ACC:%02X FLAGS:%d [%s|%s]\n",
           cpu.ip, cpu.sp, cpu.acc, cpu.flags, cpu.domain, cpu.owner);
}

/* ========== 显示内存 ========== */
void show_mem(int start) {
    if (start < 0) start = 0;
    if (start > 255) start = 255;
    for (int i = start; i < start + 16 && i < 256; i += 16) {
        printf("[%02X]", i);
        for (int j = 0; j < 16 && i + j < 256; j++)
            printf(" %02X", cpu.mem[i+j]);
        printf("\n");
    }
}

/* ========== 帮助 ========== */
void shell_help(void) {
    printf("\n意交感息Shell命令:\n");
    printf("  load <file>     - 加载符号文件\n");
    printf("  run             - 显示当前符号表\n");
    printf("  step            - 显示当前指令\n");
    printf("  break <addr>    - 显示内存地址\n");
    printf("  reg             - 显示寄存器\n");
    printf("  mem [addr]      - 显示内存\n");
    printf("  fib [n]         - 斐波那契数列\n");
    printf("  rule <order> <op> - 规则层级运算 (op: add/sub/mul/div)\n");
    printf("  str <text>      - 字符串变换\n");
    printf("  help            - 显示帮助\n");
    printf("  quit            - 退出\n");
}

/* ========== 字符串变换 ========== */
void str_transform(const char *input) {
    int len = strlen(input);
    char output[256];
    static const char *sym = "·○●◆★※∞⊕◎";
    int sym_len = strlen(sym);
    for (int i = 0; i < len; i++) {
        int idx = (((int)input[i] * 2) + i) % sym_len;
        output[i] = sym[idx];
    }
    output[len] = '\0';
    printf("输入: %s\n变换: %s\n", input, output);
}

/* ========== 规则层级运算 ========== */
void rule_levels(int order, const char *op_str) {
    OpType op;
    if (strcmp(op_str, "add") == 0) op = OP_ADD;
    else if (strcmp(op_str, "sub") == 0) op = OP_SUB;
    else if (strcmp(op_str, "mul") == 0) op = OP_MUL;
    else if (strcmp(op_str, "div") == 0) op = OP_DIV;
    else { printf("未知运算: %s\n", op_str); return; }
    
    long long seq[50];
    for (int i = 0; i < order && i < 50; i++) {
        seq[i] = (i == 0) ? ANCHOR_SELF : ANCHOR_LIU;
    }
    for (int i = order; i < 30; i++) {
        long long result = seq[i-order];
        for (int j = 1; j < order; j++) {
            switch (op) {
                case OP_ADD: result += seq[i-order+j]; break;
                case OP_SUB: result -= seq[i-order+j]; break;
                case OP_MUL: result *= seq[i-order+j]; break;
                case OP_DIV: if (seq[i-order+j]) result /= seq[i-order+j]; break;
                default: break;
            }
        }
        seq[i] = result;
    }
    printf("%d阶%s: ", order, op_str);
    for (int i = 0; i < 15; i++) printf("%lld ", seq[i]);
    printf("\n");
}

/* ========== Shell主函数 ========== */
int main(void) {
    char cmd[MAX_LINE];
    
    printf("意交感息（isi）Shell v0.1\n");
    printf("满全法根基锚点: 我=%d, 刘楚恬=%d\n", ANCHOR_SELF, ANCHOR_LIU);
    printf("身份落款: 刘楚恬和我\n");
    printf("输入 'help' 查看命令\n");
    
    while (1) {
        printf("\nfqm> ");
        if (fgets(cmd, sizeof(cmd), stdin) == NULL) break;
        cmd[strcspn(cmd, "\n")] = '\0';
        
        /* 解析命令 */
        char *args[8] = {NULL};
        int argc = 0;
        char *token = strtok(cmd, " ");
        while (token && argc < 8) {
            args[argc++] = token;
            token = strtok(NULL, " ");
        }
        
        if (argc == 0) continue;
        
        if (strcmp(args[0], "load") == 0 && argc >= 2) {
            load_file(args[1]);
        } else if (strcmp(args[0], "run") == 0) {
            if (sym_cnt > 0) {
                printf("当前文件: %s\n", loaded_file);
                printf("符号数: %d\n", sym_cnt);
            } else {
                printf("未加载文件\n");
            }
        } else if (strcmp(args[0], "step") == 0) {
            if (sym_cnt > 0 && cpu.ip < sym_cnt) {
                Symbol *s = &syms[cpu.ip];
                printf("当前指令 [%d]: %s %s %02X %02X %d\n",
                       cpu.ip, s->name, op_name(s->op), s->src, s->dst, s->imm);
            }
        } else if (strcmp(args[0], "break") == 0 && argc >= 2) {
            printf("断点地址: %s\n", args[1]);
        } else if (strcmp(args[0], "reg") == 0) {
            show_reg();
        } else if (strcmp(args[0], "mem") == 0) {
            int addr = (argc >= 2) ? atoi(args[1]) : 0;
            show_mem(addr);
        } else if (strcmp(args[0], "fib") == 0) {
            int n = (argc >= 2) ? atoi(args[1]) : 15;
            long long a = 0, b = 1;
            for (int i = 0; i < n; i++) {
                printf("F(%d)=%lld\n", i, a);
                long long t = a + b;
                a = b;
                b = t;
            }
        } else if (strcmp(args[0], "rule") == 0 && argc >= 3) {
            rule_levels(atoi(args[1]), args[2]);
        } else if (strcmp(args[0], "str") == 0 && argc >= 2) {
            char text[256] = "";
            for (int i = 1; i < argc; i++) {
                if (i > 1) strcat(text, " ");
                strcat(text, args[i]);
            }
            str_transform(text);
        } else if (strcmp(args[0], "help") == 0) {
            shell_help();
        } else if (strcmp(args[0], "quit") == 0 || strcmp(args[0], "exit") == 0) {
            break;
        } else {
            printf("未知命令: %s\n", args[0]);
            shell_help();
        }
    }
    
    printf("\n退出意交感息Shell\n");
    return 0;
}
