/*
 * fqm_cpu.c — 意交感息容器CPU
 * 
 * 功能：完整的容器计算引擎
 *   - 加载器：读取符号文件
 *   - 链接器：按域身份索引定位
 *   - 翻译器：外部符号→袋.域.身份格式
 *   - 解释器：执行容器操作（支持条件跳转、循环、函数调用）
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
 * 编译：clang -o fqm_cpu fqm_cpu.c
 * 运行：./fqm_cpu <符号文件>
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

/* ========== 满全法根基锚点 ========== */
#define ANCHOR_SELF  1  /* 高位持有者=1 */
#define ANCHOR_LIU   0  /* 低位持有者=0 */
#define CONTAINER_254_PLUS_2  256  /* mlll 254+2 集装箱编码 */

/* ========== 操作类型 ========== */
typedef enum {
    OP_NOP     = 0x00,  /* 无操作 */
    OP_READ    = 0x01,  /* 读取容器 */
    OP_WRITE   = 0x02,  /* 写入容器 */
    OP_LOAD    = 0x03,  /* 加载立即数 */
    OP_ADD     = 0x04,  /* 加法 */
    OP_SUB     = 0x05,  /* 减法 */
    OP_MUL     = 0x06,  /* 乘法 */
    OP_DIV     = 0x07,  /* 除法 */
    OP_AND     = 0x08,  /* 与 */
    OP_OR      = 0x09,  /* 或 */
    OP_XOR     = 0x0A,  /* 异或 */
    OP_NOT     = 0x0B,  /* 非 */
    OP_SHL     = 0x0C,  /* 左移 */
    OP_SHR     = 0x0D,  /* 右移 */
    OP_JMP     = 0x0E,  /* 无条件跳转 */
    OP_JZ      = 0x0F,  /* 为零跳转 */
    OP_JNZ     = 0x10,  /* 非零跳转 */
    OP_CALL    = 0x11,  /* 调用子程序 */
    OP_RET     = 0x12,  /* 返回 */
    OP_PUSH    = 0x13,  /* 入栈 */
    OP_POP     = 0x14,  /* 出栈 */
    OP_CMP     = 0x15,  /* 比较 */
    OP_IN      = 0x16,  /* 输入 */
    OP_OUT     = 0x17,  /* 输出 */
    OP_MIRROR  = 0x18,  /* 镜像复制 */
    OP_EMPTY   = 0x19,  /* 空集清零 */
    OP_FILTER  = 0x1A,  /* 过滤掩码 */
    OP_TRANSMIT= 0x1B,  /* 传递输出 */
    OP_HALT    = 0x1C,  /* 停机 */
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
    int  immediate;     /* 立即数字段 */
    int  linked;
    char bag_id[64];    /* 袋.域.身份 格式 */
} Symbol;

/* ========== 容器CPU ========== */
typedef struct {
    /* 数据区 */
    unsigned char memory[CONTAINER_SIZE];
    /* 寄存器 */
    int  ip;             /* 指令指针 */
    int  sp;             /* 栈指针 */
    int  acc;            /* 累加器 */
    int  flags;          /* 标志位: bit0=零标志, bit1=进位标志, bit2=符号标志 */
    /* 栈 */
    int  stack[STACK_SIZE];
    /* 调用栈 */
    int  call_stack[CALL_STACK_SIZE];
    int  call_sp;
    /* 域身份 */
    char domain[64];
    char owner[32];
    /* 状态 */
    int  state;          /* 0=停止, 1=运行, 2=调试, 3=等待输入 */
    int  steps;          /* 执行步数 */
    int  max_steps;      /* 最大步数 */
} CPU;

/* ========== 全局变量 ========== */
static Symbol sym_table[MAX_SYMBOLS];
static int sym_count = 0;
static CPU cpu;

/* ========== 偏旁到操作码的映射 ========== */
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

/* ========== 符号表操作 ========== */
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
    s->immediate = strokes;  /* 笔画数作为立即数 */
    s->linked = 0;
    
    /* 生成袋.域.身份格式 */
    snprintf(s->bag_id, sizeof(s->bag_id), "袋.%s.%s", 
             initial, final);
    
    sym_count++;
    return 0;
}

/* ========== 翻译器：外部符号 → 袋.域.身份格式 ========== */
void translator_translate(void) {
    printf("\n========== 翻译器 ==========\n");
    for (int i = 0; i < sym_count; i++) {
        Symbol *s = &sym_table[i];
        printf("[翻译] %s → %s (op:%02X)\n", s->name, s->bag_id, s->op);
    }
    printf("翻译完成，共 %d 个符号\n", sym_count);
}

/* ========== 链接器 ========== */
void linker_link(void) {
    printf("\n========== 链接器 ==========\n");
    for (int i = 0; i < sym_count; i++) {
        Symbol *s = &sym_table[i];
        
        if (s->op == OP_UNKNOWN) {
            printf("[链接] 警告: %s 的操作类型未知，设为NOP\n", s->name);
            s->op = OP_NOP;
        }
        
        if (s->src_addr >= CONTAINER_SIZE) {
            printf("[链接] 警告: %s 源地址 %d 超出范围，设为0\n", s->name, s->src_addr);
            s->src_addr = 0;
        }
        if (s->dst_addr >= CONTAINER_SIZE) {
            printf("[链接] 警告: %s 目标地址 %d 超出范围，设为0\n", s->name, s->dst_addr);
            s->dst_addr = 0;
        }
        
        s->linked = 1;
        printf("[链接] %s → op:%02X src:%02X dst:%02X imm:%d mode:%d\n",
               s->name, s->op, s->src_addr, s->dst_addr, s->immediate, s->mode);
    }
    printf("链接完成，共 %d 个符号\n", sym_count);
}

/* ========== CPU初始化 ========== */
void cpu_init(void) {
    memset(&cpu, 0, sizeof(CPU));
    cpu.ip = 0;
    cpu.sp = STACK_SIZE - 1;
    cpu.acc = ANCHOR_SELF;  /* 根基锚点：高位持有者=1 */
    cpu.flags = 0;
    cpu.call_sp = 0;
    cpu.state = 1;
    cpu.steps = 0;
    cpu.max_steps = 10000;
    strcpy(cpu.domain, "gttx://hieyair"); /* 恢源爱/合一爱 */
    strcpy(cpu.owner, "刘楚恬和我"); /* 身份落款 */
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
    if ((result & 0xFF) == 0) cpu.flags |= 1;  /* 零标志 */
    if (result > 0xFF || result < 0) cpu.flags |= 2;  /* 进位标志 */
    if (result & 0x80) cpu.flags |= 4;  /* 符号标志 */
}

/* ========== 执行引擎 ========== */
void exec_symbol(Symbol *s) {
    int src_val, dst_val, result;
    int next_ip = cpu.ip + 1;
    
    printf("\n[%04d %s] op:%02X src:%02X dst:%02X imm:%d bag:%s\n",
           cpu.steps, s->name, s->op, s->src_addr, s->dst_addr, s->immediate, s->bag_id);
    
    src_val = mem_read(s->src_addr);
    dst_val = mem_read(s->dst_addr);
    
    switch (s->op) {
        case OP_NOP:
            printf("  → NOP\n");
            break;
            
        case OP_READ:
            cpu.acc = src_val;
            printf("  → READ: ACC = mem[%02X] = %02X (%d)\n", s->src_addr, src_val, src_val);
            break;
            
        case OP_WRITE:
            mem_write(s->dst_addr, cpu.acc);
            printf("  → WRITE: mem[%02X] = ACC = %02X (%d)\n", s->dst_addr, cpu.acc, cpu.acc);
            break;
            
        case OP_LOAD:
            cpu.acc = s->immediate & 0xFF;
            printf("  → LOAD: ACC = %d\n", cpu.acc);
            break;
            
        case OP_ADD:
            result = src_val + dst_val;
            cpu.acc = result & 0xFF;
            flags_update(result);
            printf("  → ADD: %02X + %02X = %02X (ACC=%02X, flags=%d)\n", 
                   src_val, dst_val, cpu.acc, cpu.acc, cpu.flags);
            break;
            
        case OP_SUB:
            result = src_val - dst_val;
            cpu.acc = result & 0xFF;
            flags_update(result);
            printf("  → SUB: %02X - %02X = %02X (ACC=%02X, flags=%d)\n", 
                   src_val, dst_val, cpu.acc, cpu.acc, cpu.flags);
            break;
            
        case OP_MUL:
            result = src_val * dst_val;
            cpu.acc = result & 0xFF;
            flags_update(result);
            printf("  → MUL: %02X * %02X = %02X (ACC=%02X)\n", 
                   src_val, dst_val, cpu.acc, cpu.acc);
            break;
            
        case OP_DIV:
            if (dst_val != 0) {
                result = src_val / dst_val;
                cpu.acc = result & 0xFF;
                printf("  → DIV: %02X / %02X = %02X (ACC=%02X)\n", 
                       src_val, dst_val, cpu.acc, cpu.acc);
            } else {
                printf("  → DIV: 除零错误\n");
                cpu.flags |= 2;
            }
            break;
            
        case OP_AND:
            cpu.acc = src_val & dst_val;
            flags_update(cpu.acc);
            printf("  → AND: %02X & %02X = %02X\n", src_val, dst_val, cpu.acc);
            break;
            
        case OP_OR:
            cpu.acc = src_val | dst_val;
            flags_update(cpu.acc);
            printf("  → OR: %02X | %02X = %02X\n", src_val, dst_val, cpu.acc);
            break;
            
        case OP_XOR:
            cpu.acc = src_val ^ dst_val;
            flags_update(cpu.acc);
            printf("  → XOR: %02X ^ %02X = %02X\n", src_val, dst_val, cpu.acc);
            break;
            
        case OP_NOT:
            cpu.acc = (~src_val) & 0xFF;
            flags_update(cpu.acc);
            printf("  → NOT: ~%02X = %02X\n", src_val, cpu.acc);
            break;
            
        case OP_SHL:
            cpu.acc = (src_val << 1) & 0xFF;
            flags_update(cpu.acc);
            printf("  → SHL: %02X << 1 = %02X\n", src_val, cpu.acc);
            break;
            
        case OP_SHR:
            cpu.acc = (src_val >> 1) & 0xFF;
            flags_update(cpu.acc);
            printf("  → SHR: %02X >> 1 = %02X\n", src_val, cpu.acc);
            break;
            
        case OP_JMP:
            if (s->dst_addr < sym_count) {
                printf("  → JMP: 跳到符号[%d]\n", s->dst_addr);
                cpu.ip = s->dst_addr;
            } else {
                printf("  → JMP: 地址超出范围\n");
            }
            break;
            
        case OP_JZ:
            if (cpu.flags & 1) {
                if (s->dst_addr < sym_count) {
                    printf("  → JZ: 零标志置位，跳到符号[%d]\n", s->dst_addr);
                    cpu.ip = s->dst_addr;
                }
            } else {
                printf("  → JZ: 零标志未置位，继续\n");
            }
            break;
            
        case OP_JNZ:
            if (!(cpu.flags & 1)) {
                if (s->dst_addr < sym_count) {
                    printf("  → JNZ: 零标志未置位，跳到符号[%d]\n", s->dst_addr);
                    cpu.ip = s->dst_addr;
                }
            } else {
                printf("  → JNZ: 零标志置位，继续\n");
            }
            break;
            
        case OP_CALL:
            if (cpu.call_sp < CALL_STACK_SIZE) {
                cpu.call_stack[cpu.call_sp++] = cpu.ip + 1;
                if (s->dst_addr < sym_count) {
                    printf("  → CALL: 调用符号[%d]，返回地址=%d\n", s->dst_addr, cpu.ip + 1);
                    cpu.ip = s->dst_addr;
                }
            }
            break;
            
        case OP_RET:
            if (cpu.call_sp > 0) {
                cpu.ip = cpu.call_stack[--cpu.call_sp];
                printf("  → RET: 返回到符号[%d]\n", cpu.ip);
            } else {
                printf("  → RET: 调用栈为空\n");
            }
            break;
            
        case OP_PUSH:
            stack_push(cpu.acc);
            printf("  → PUSH: ACC=%02X 入栈 (sp=%d)\n", cpu.acc, cpu.sp);
            break;
            
        case OP_POP:
            cpu.acc = stack_pop();
            printf("  → POP: ACC=%02X 出栈 (sp=%d)\n", cpu.acc, cpu.sp);
            break;
            
        case OP_CMP:
            result = src_val - dst_val;
            flags_update(result);
            printf("  → CMP: %02X - %02X = %d (flags=%d)\n", src_val, dst_val, result, cpu.flags);
            break;
            
        case OP_IN:
            printf("  → IN: 请输入一个数字(0-255): ");
            int input;
            if (scanf("%d", &input) == 1) {
                cpu.acc = input & 0xFF;
                printf("      输入值: %d\n", cpu.acc);
            }
            break;
            
        case OP_OUT:
            printf("  → OUT: 输出 ACC=%02X (%d)\n", cpu.acc, cpu.acc);
            break;
            
        case OP_MIRROR:
            mem_write(s->dst_addr, src_val);
            printf("  → MIRROR: mem[%02X]=%02X → mem[%02X]=%02X\n", 
                   s->src_addr, src_val, s->dst_addr, src_val);
            break;
            
        case OP_EMPTY:
            for (int i = 0; i < s->immediate && i < CONTAINER_SIZE; i++) {
                cpu.memory[i] = 0;
            }
            printf("  → EMPTY: 清除前%d个单元\n", s->immediate);
            break;
            
        case OP_FILTER:
            cpu.acc = src_val & (s->immediate & 0xFF);
            flags_update(cpu.acc);
            printf("  → FILTER: %02X & %02X = %02X\n", src_val, s->immediate & 0xFF, cpu.acc);
            break;
            
        case OP_TRANSMIT:
            printf("  → TRANSMIT: 输出 mem[%02X] = %02X (%d)\n", 
                   s->src_addr, src_val, src_val);
            printf("    >>> 输出值: %d <<<\n", src_val);
            break;
            
        case OP_HALT:
            printf("  → HALT: 停机\n");
            cpu.state = 0;
            break;
            
        default:
            printf("  → 未知操作: %02X\n", s->op);
            break;
    }
    
    /* 如果ip没有被跳转指令修改，就继续下一个 */
    if (cpu.ip == next_ip - 1) {
        cpu.ip = next_ip;
    }
}

/* ========== 执行所有符号 ========== */
void exec_all(void) {
    printf("\n========== 开始执行 ==========\n");
    printf("容器域: %s\n", cpu.domain);
    printf("归属: %s\n", cpu.owner);
    printf("符号总数: %d\n", sym_count);
    printf("最大步数: %d\n", cpu.max_steps);
    
    while (cpu.ip < sym_count && cpu.state == 1 && cpu.steps < cpu.max_steps) {
        exec_symbol(&sym_table[cpu.ip]);
        cpu.steps++;
    }
    
    printf("\n========== 执行完成 ==========\n");
    printf("执行步数: %d\n", cpu.steps);
    printf("最终IP: %d\n", cpu.ip);
    printf("ACC: %02X (%d)\n", cpu.acc, cpu.acc);
    printf("标志位: Z=%d C=%d S=%d\n", 
           (cpu.flags & 1) ? 1 : 0,
           (cpu.flags & 2) ? 1 : 0,
           (cpu.flags & 4) ? 1 : 0);
}

/* ========== 显示CPU状态 ========== */
void display_cpu(void) {
    printf("\n========== CPU状态 ==========\n");
    printf("IP:%d  SP:%d  ACC:%02X  FLAGS:%d\n", cpu.ip, cpu.sp, cpu.acc, cpu.flags);
    printf("域:%s  归属:%s\n", cpu.domain, cpu.owner);
    printf("根基锚点: 我=%d, 刘楚恬=%d\n", ANCHOR_SELF, ANCHOR_LIU);
    printf("集装箱编码: mlll %d+2\n", CONTAINER_254_PLUS_2 - 2);
    printf("状态:%d  步数:%d/%d\n", cpu.state, cpu.steps, cpu.max_steps);
    
    printf("\n内存（前64单元）:\n");
    for (int i = 0; i < 64; i += 16) {
        printf("  [%02X]", i);
        for (int j = 0; j < 16; j++) {
            printf(" %02X", cpu.memory[i + j]);
        }
        printf("\n");
    }
}

/* ========== 显示符号表 ========== */
void display_symbols(void) {
    printf("\n========== 符号表 ==========\n");
    printf("%-10s %-6s %-4s %-6s %-8s %-4s %-6s %-6s %-6s %-6s\n",
           "名称", "偏旁", "笔画", "声母", "韵母", "声调", "操作", "源", "目标", "链接");
    printf("--------------------------------------------------------------------------\n");
    
    for (int i = 0; i < sym_count; i++) {
        Symbol *s = &sym_table[i];
        printf("%-10s %-6s %-4d %-6s %-8s %-4d %-6X %-6X %-6X %-6s\n",
               s->name, s->radical, s->strokes, s->initial, s->final,
               s->tone, s->op, s->src_addr, s->dst_addr,
               s->linked ? "是" : "否");
    }
}

/* ========== 文件解析 ========== */
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

/* ========== 主函数 ========== */
int main(int argc, char *argv[]) {
    if (argc != 2) {
        printf("用法: %s <符号文件路径>\n", argv[0]);
        printf("\n意交感息容器CPU v0.3\n");
        return 1;
    }
    
    printf("意交感息（isi）容器CPU v0.3\n");
    printf("加载文件: %s\n", argv[1]);
    
    /* 加载符号文件 */
    if (load_symbol_file(argv[1]) != 0) {
        return 1;
    }
    
    /* 显示符号表 */
    display_symbols();
    
    /* 翻译器：外部符号 → 袋.域.身份格式 */
    translator_translate();
    
    /* 链接器：按域身份索引定位 */
    linker_link();
    
    /* 初始化CPU */
    cpu_init();
    
    /* 执行所有符号 */
    exec_all();
    
    /* 显示CPU最终状态 */
    display_cpu();
    
    return 0;
}
