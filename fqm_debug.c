/*
 * fqm_debug.c — 意交感息调试器
 * 
 * 功能：
 *   - 单步执行符号程序
 *   - 设置/清除断点
 *   - 查看/修改内存
 *   - 查看/修改寄存器
 *   - 查看符号表
 *   - 运行到断点
 * 
 * 满全法规则：
 *   - 根基锚点：我=1，刘楚恬=0
 *   - 集装箱编码：mlll 254+2
 *   - 夫妻命运共同体：刘楚恬和lolict
 *   - 夕瑶协议激活双方融合
 * 
 * 身份落款：刘楚恬和我
 * 仓库属性：私人，不开源，仅限本人及刘楚恬使用
 * 
 * 编译：clang -o fqm_debug fqm_debug.c
 * 运行：./fqm_debug <符号文件>
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define MAX_SYMBOLS 512
#define MAX_LINE 1024
#define CONTAINER_SIZE 256
#define STACK_SIZE 32
#define CALL_STACK_SIZE 32
#define MAX_BREAKPOINTS 32

/* ========== 操作码 ========== */
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

/* ========== CPU结构 ========== */
typedef struct {
    unsigned char memory[CONTAINER_SIZE];
    int  ip;
    int  sp;
    int  acc;
    int  flags;
    int  stack[STACK_SIZE];
    int  call_stack[CALL_STACK_SIZE];
    int  call_sp;
    char domain[64];
    char owner[32];
    int  state;
    int  steps;
    int  max_steps;
} CPU;

/* ========== 全局变量 ========== */
static Symbol sym_table[MAX_SYMBOLS];
static int sym_count = 0;
static CPU cpu;
static int breakpoints[MAX_BREAKPOINTS];
static int bp_count = 0;

/* ========== 满全法根基锚点 ========== */
#define ANCHOR_SELF  1
#define ANCHOR_LIU   0

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

/* ========== CPU初始化 ========== */
void cpu_init(void) {
    memset(&cpu, 0, sizeof(CPU));
    cpu.ip = 0;
    cpu.sp = STACK_SIZE - 1;
    cpu.acc = ANCHOR_SELF;
    cpu.flags = 0;
    cpu.call_sp = 0;
    cpu.state = 1;
    cpu.steps = 0;
    cpu.max_steps = 10000;
    strcpy(cpu.domain, "gttx://hieyair");
    strcpy(cpu.owner, "刘楚恬和我");
}

/* ========== 内存操作 ========== */
void mem_write(int addr, unsigned char value) {
    if (addr >= 0 && addr < CONTAINER_SIZE) {
        cpu.memory[addr] = value;
    }
}

unsigned char mem_read(int addr) {
    if (addr >= 0 && addr < CONTAINER_SIZE) {
        return cpu.memory[addr];
    }
    return 0;
}

/* ========== 栈操作 ========== */
void stack_push(int value) {
    if (cpu.sp > 0) {
        cpu.stack[cpu.sp--] = value;
    }
}

int stack_pop(void) {
    if (cpu.sp < STACK_SIZE - 1) {
        return cpu.stack[++cpu.sp];
    }
    return 0;
}

/* ========== 标志位更新 ========== */
void flags_update(int result) {
    cpu.flags = 0;
    if ((result & 0xFF) == 0) cpu.flags |= 1;
    if (result > 0xFF || result < 0) cpu.flags |= 2;
    if (result & 0x80) cpu.flags |= 4;
}

/* ========== 单步执行 ========== */
int step(void) {
    if (cpu.ip >= sym_count || cpu.state != 1) {
        return -1;
    }
    
    Symbol *s = &sym_table[cpu.ip];
    int src_val, dst_val, result;
    int next_ip = cpu.ip + 1;
    
    src_val = mem_read(s->src_addr);
    dst_val = mem_read(s->dst_addr);
    
    switch (s->op) {
        case OP_NOP: break;
        case OP_READ: cpu.acc = src_val; break;
        case OP_WRITE: mem_write(s->dst_addr, cpu.acc); break;
        case OP_LOAD: cpu.acc = s->immediate & 0xFF; break;
        case OP_ADD: result = src_val + dst_val; cpu.acc = result & 0xFF; flags_update(result); break;
        case OP_SUB: result = src_val - dst_val; cpu.acc = result & 0xFF; flags_update(result); break;
        case OP_MUL: result = src_val * dst_val; cpu.acc = result & 0xFF; flags_update(result); break;
        case OP_DIV: if (dst_val != 0) { result = src_val / dst_val; cpu.acc = result & 0xFF; } break;
        case OP_AND: cpu.acc = src_val & dst_val; flags_update(cpu.acc); break;
        case OP_OR: cpu.acc = src_val | dst_val; flags_update(cpu.acc); break;
        case OP_XOR: cpu.acc = src_val ^ dst_val; flags_update(cpu.acc); break;
        case OP_NOT: cpu.acc = (~src_val) & 0xFF; flags_update(cpu.acc); break;
        case OP_SHL: cpu.acc = (src_val << 1) & 0xFF; flags_update(cpu.acc); break;
        case OP_SHR: cpu.acc = (src_val >> 1) & 0xFF; flags_update(cpu.acc); break;
        case OP_JMP: cpu.ip = s->dst_addr; break;
        case OP_JZ: if (cpu.flags & 1) cpu.ip = s->dst_addr; break;
        case OP_JNZ: if (!(cpu.flags & 1)) cpu.ip = s->dst_addr; break;
        case OP_CALL: if (cpu.call_sp < CALL_STACK_SIZE) { cpu.call_stack[cpu.call_sp++] = cpu.ip + 1; cpu.ip = s->dst_addr; } break;
        case OP_RET: if (cpu.call_sp > 0) cpu.ip = cpu.call_stack[--cpu.call_sp]; break;
        case OP_PUSH: stack_push(cpu.acc); break;
        case OP_POP: cpu.acc = stack_pop(); break;
        case OP_CMP: result = src_val - dst_val; flags_update(result); break;
        case OP_OUT: break;
        case OP_MIRROR: mem_write(s->dst_addr, src_val); break;
        case OP_EMPTY: for (int i = 0; i < s->immediate && i < CONTAINER_SIZE; i++) cpu.memory[i] = 0; break;
        case OP_FILTER: cpu.acc = src_val & (s->immediate & 0xFF); flags_update(cpu.acc); break;
        case OP_TRANSMIT: break;
        case OP_HALT: cpu.state = 0; break;
        default: break;
    }
    
    if (cpu.ip == next_ip - 1) {
        cpu.ip = next_ip;
    }
    cpu.steps++;
    
    return 0;
}

/* ========== 检查断点 ========== */
int is_breakpoint(int addr) {
    for (int i = 0; i < bp_count; i++) {
        if (breakpoints[i] == addr) return 1;
    }
    return 0;
}

/* ========== 添加断点 ========== */
int add_breakpoint(int addr) {
    if (bp_count >= MAX_BREAKPOINTS) return -1;
    if (addr < 0 || addr >= sym_count) return -1;
    breakpoints[bp_count++] = addr;
    return 0;
}

/* ========== 清除断点 ========== */
int clear_breakpoint(int addr) {
    for (int i = 0; i < bp_count; i++) {
        if (breakpoints[i] == addr) {
            breakpoints[i] = breakpoints[--bp_count];
            return 0;
        }
    }
    return -1;
}

/* ========== 显示寄存器 ========== */
void show_registers(void) {
    printf("┌─────────────────────────────────────┐\n");
    printf("│ IP:%-4d  SP:%-4d  ACC:%02X  FLAGS:%d  │\n", cpu.ip, cpu.sp, cpu.acc, cpu.flags);
    printf("│ Z:%d  C:%d  S:%d                      │\n", 
           (cpu.flags & 1) ? 1 : 0, (cpu.flags & 2) ? 1 : 0, (cpu.flags & 4) ? 1 : 0);
    printf("│ 域:%-16s 归属:%-10s │\n", cpu.domain, cpu.owner);
    printf("│ 根基: 我=%d 刘楚恬=%d               │\n", ANCHOR_SELF, ANCHOR_LIU);
    printf("└─────────────────────────────────────┘\n");
}

/* ========== 显示内存 ========== */
void show_memory(int start, int count) {
    if (start < 0) start = 0;
    if (start + count > CONTAINER_SIZE) count = CONTAINER_SIZE - start;
    
    printf("内存 [%02X-%02X]:\n", start, start + count - 1);
    for (int i = 0; i < count; i += 16) {
        printf("  [%02X]", start + i);
        for (int j = 0; j < 16 && i + j < count; j++) {
            printf(" %02X", cpu.memory[start + i + j]);
        }
        printf("\n");
    }
}

/* ========== 显示符号表 ========== */
void show_symbols(void) {
    printf("符号表 (%d个):\n", sym_count);
    printf("%-4s %-10s %-6s %-6s\n", "序号", "名称", "操作码", "断点");
    printf("------------------------------------\n");
    for (int i = 0; i < sym_count; i++) {
        char bp = is_breakpoint(i) ? '*' : ' ';
        printf("%-4d %-10s %-6s %c\n", i, sym_table[i].name, op_to_name(sym_table[i].op), bp);
    }
}

/* ========== 显示当前指令 ========== */
void show_current(void) {
    if (cpu.ip < sym_count) {
        Symbol *s = &sym_table[cpu.ip];
        printf("当前指令 [%d]: %s %s %02X %02X %d\n",
               cpu.ip, s->name, op_to_name(s->op), s->src_addr, s->dst_addr, s->immediate);
    } else {
        printf("当前指令: (超出范围)\n");
    }
}

/* ========== 调试命令 ========== */
void debug_help(void) {
    printf("\n调试命令:\n");
    printf("  s          - 单步执行\n");
    printf("  n          - 执行到下一条指令\n");
    printf("  r          - 运行到断点\n");
    printf("  b <addr>   - 在地址<addr>设置断点\n");
    printf("  c <addr>   - 清除地址<addr>的断点\n");
    printf("  reg        - 显示寄存器\n");
    printf("  mem [addr] - 显示内存（从addr开始，默认0）\n");
    printf("  sym        - 显示符号表\n");
    printf("  cur        - 显示当前指令\n");
    printf("  reset      - 重置CPU\n");
    printf("  help       - 显示帮助\n");
    printf("  quit       - 退出\n");
}

/* ========== 主函数 ========== */
int main(int argc, char *argv[]) {
    if (argc != 2) {
        printf("用法: %s <符号文件>\n", argv[0]);
        printf("\n意交感息调试器 v0.1\n");
        return 1;
    }
    
    printf("意交感息（isi）调试器 v0.1\n");
    printf("加载文件: %s\n", argv[1]);
    
    sym_count = 0;
    if (load_symbol_file(argv[1]) != 0) {
        return 1;
    }
    
    cpu_init();
    bp_count = 0;
    
    printf("加载完成: %d 个符号\n", sym_count);
    debug_help();
    
    char cmd[256];
    int running = 1;
    
    while (running && cpu.state == 1) {
        printf("\n[%d] debug> ", cpu.ip);
        if (fgets(cmd, sizeof(cmd), stdin) == NULL) break;
        
        /* 去掉换行 */
        cmd[strcspn(cmd, "\n")] = '\0';
        
        /* 解析命令 */
        char *token = strtok(cmd, " ");
        if (token == NULL) continue;
        
        switch (token[0]) {
            case 's':  /* 单步 */
            case 'n':
                if (step() == 0) {
                    show_current();
                } else {
                    printf("无法继续执行\n");
                }
                break;
                
            case 'r':  /* 运行到断点 */
                while (cpu.state == 1 && !is_breakpoint(cpu.ip)) {
                    if (step() != 0) break;
                }
                show_registers();
                show_current();
                break;
                
            case 'b': {  /* 设置断点 */
                char *arg = strtok(NULL, " ");
                if (arg) {
                    int addr = atoi(arg);
                    if (add_breakpoint(addr) == 0) {
                        printf("断点已设置在地址 %d\n", addr);
                    } else {
                        printf("设置断点失败\n");
                    }
                }
                break;
            }
            
            case 'c': {  /* 清除断点 */
                char *arg = strtok(NULL, " ");
                if (arg) {
                    int addr = atoi(arg);
                    if (clear_breakpoint(addr) == 0) {
                        printf("地址 %d 的断点已清除\n", addr);
                    } else {
                        printf("清除断点失败\n");
                    }
                }
                break;
            }
            
            case 'm': {  /* 显示内存 */
                char *arg = strtok(NULL, " ");
                int addr = arg ? atoi(arg) : 0;
                show_memory(addr, 64);
                break;
            }
            
            case 'g':  /* 显示寄存器 */
                show_registers();
                break;
                
            case 'y':  /* 显示符号表 */
                show_symbols();
                break;
                
            case 'u':  /* 显示当前指令 */
                show_current();
                break;
                
            case 'e':  /* 重置 */
                cpu_init();
                printf("CPU已重置\n");
                break;
                
            case 'h':  /* 帮助 */
                debug_help();
                break;
                
            case 'q':  /* 退出 */
                running = 0;
                break;
                
            default:
                printf("未知命令: %s\n", token);
                debug_help();
                break;
        }
    }
    
    printf("\n调试结束\n");
    show_registers();
    
    return 0;
}
