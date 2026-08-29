/*
 * fqm_engine.c — 意交感息容器引擎
 * 
 * 功能：加载符号 → 链接定位 → 执行容器操作
 * 编译：clang -o fqm_engine fqm_engine.c
 * 运行：./fqm_engine <符号文件>
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define MAX_SYMBOLS 256
#define MAX_LINE 1024
#define CONTAINER_SIZE 256

/* ========== 操作类型 ========== */
typedef enum {
    OP_NOP     = 0x00,  /* 无操作 */
    OP_READ    = 0x01,  /* 读取容器 */
    OP_WRITE   = 0x02,  /* 写入容器 */
    OP_EXEC    = 0x03,  /* 执行跳转 */
    OP_JUMP    = 0x04,  /* 无条件跳转 */
    OP_CALL    = 0x05,  /* 调用子程序 */
    OP_RETURN  = 0x06,  /* 返回 */
    OP_MIRROR  = 0x07,  /* 镜像复制 */
    OP_EMPTY   = 0x08,  /* 空集清零 */
    OP_FILTER  = 0x09,  /* 过滤掩码 */
    OP_TRANSMIT= 0x0A,  /* 传递输出 */
    OP_ADD     = 0x0B,  /* 加法 */
    OP_SUB     = 0x0C,  /* 减法 */
    OP_AND     = 0x0D,  /* 与 */
    OP_OR      = 0x0E,  /* 或 */
    OP_XOR     = 0x0F,  /* 异或 */
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
    int  linked;        /* 是否已链接 */
} Symbol;

/* ========== 容器 ========== */
typedef struct {
    unsigned char data[CONTAINER_SIZE];
    int  ip;             /* 指令指针 */
    int  sp;             /* 栈指针 */
    int  state;          /* 容器状态: 0=停止, 1=运行, 2=调试 */
    char domain[32];     /* 域身份 */
    char owner[32];      /* 归属 */
} Container;

/* ========== 全局变量 ========== */
static Symbol sym_table[MAX_SYMBOLS];
static int sym_count = 0;
static Container container;
static int call_stack[32];
static int call_sp = 0;

/* ========== 偏旁映射 ========== */
OpType radical_to_op(const char *radical) {
    if (strcmp(radical, "贝") == 0) return OP_READ;
    if (strcmp(radical, "扌") == 0) return OP_WRITE;
    if (strcmp(radical, "讠") == 0) return OP_EXEC;
    if (strcmp(radical, "辶") == 0) return OP_JUMP;
    if (strcmp(radical, "口") == 0) return OP_CALL;
    if (radical[0] == '\0') return OP_NOP;
    if (strcmp(radical, "囗") == 0) return OP_RETURN;
    if (strcmp(radical, "人") == 0) return OP_MIRROR;
    if (strcmp(radical, "穴") == 0) return OP_EMPTY;
    if (strcmp(radical, "氵") == 0) return OP_FILTER;
    if (strcmp(radical, "火") == 0) return OP_TRANSMIT;
    if (strcmp(radical, "力") == 0) return OP_ADD;
    if (strcmp(radical, "刀") == 0) return OP_SUB;
    if (strcmp(radical, "一") == 0) return OP_AND;
    if (strcmp(radical, "二") == 0) return OP_OR;
    if (strcmp(radical, "三") == 0) return OP_XOR;
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
        case 1: return 0;  /* 默认 */
        case 2: return 1;  /* 执行 */
        case 3: return 2;  /* 调试 */
        case 4: return 3;  /* 特权 */
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
    s->linked = 0;
    
    sym_count++;
    return 0;
}

/* ========== 链接器 ========== */
void linker_link(void) {
    printf("\n========== 链接器 ==========\n");
    for (int i = 0; i < sym_count; i++) {
        Symbol *s = &sym_table[i];
        
        /* 检查操作类型是否已知 */
        if (s->op == OP_UNKNOWN) {
            printf("[链接] 警告: %s 的操作类型未知，设为NOP\n", s->name);
            s->op = OP_NOP;
        }
        
        /* 检查地址范围 */
        if (s->src_addr >= CONTAINER_SIZE) {
            printf("[链接] 警告: %s 源地址 %d 超出容器范围，设为0\n", s->name, s->src_addr);
            s->src_addr = 0;
        }
        if (s->dst_addr >= CONTAINER_SIZE) {
            printf("[链接] 警告: %s 目标地址 %d 超出容器范围，设为0\n", s->name, s->dst_addr);
            s->dst_addr = 0;
        }
        
        s->linked = 1;
        printf("[链接] %s → op:%02X src:%02X dst:%02X mode:%d\n",
               s->name, s->op, s->src_addr, s->dst_addr, s->mode);
    }
    printf("链接完成，共 %d 个符号\n", sym_count);
}

/* ========== 容器初始化 ========== */
void container_init(void) {
    memset(&container, 0, sizeof(Container));
    container.ip = 0;
    container.sp = 0;
    container.state = 1;
    strcpy(container.domain, "gttx://self");
    strcpy(container.owner, "公约数");
}

/* ========== 容器操作 ========== */
void container_write(int addr, unsigned char value) {
    if (addr >= 0 && addr < CONTAINER_SIZE) {
        container.data[addr] = value;
    }
}

unsigned char container_read(int addr) {
    if (addr >= 0 && addr < CONTAINER_SIZE) {
        return container.data[addr];
    }
    return 0;
}

/* ========== 执行引擎 ========== */
void exec_symbol(Symbol *s) {
    unsigned char src_val, dst_val, result;
    
    printf("\n[执行 %s] op:%02X src:%02X dst:%02X mode:%d stroke:%d\n",
           s->name, s->op, s->src_addr, s->dst_addr, s->mode, s->strokes);
    
    src_val = container_read(s->src_addr);
    dst_val = container_read(s->dst_addr);
    
    switch (s->op) {
        case OP_NOP:
            printf("  → 无操作\n");
            break;
            
        case OP_READ:
            printf("  → 读取: 地址[%02X] = %02X (%d)\n", s->src_addr, src_val, src_val);
            break;
            
        case OP_WRITE:
            result = (unsigned char)(s->strokes & 0xFF);
            container_write(s->dst_addr, result);
            printf("  → 写入: 地址[%02X] = %02X (%d) [笔画数]\n", s->dst_addr, result, result);
            break;
            
        case OP_EXEC:
            if (s->dst_addr < sym_count) {
                printf("  → 执行跳转: 跳到符号[%d]\n", s->dst_addr);
                container.ip = s->dst_addr;
            } else {
                printf("  → 执行跳转: 地址超出范围\n");
            }
            break;
            
        case OP_JUMP:
            if (s->dst_addr < sym_count) {
                printf("  → 无条件跳转: 跳到符号[%d]\n", s->dst_addr);
                container.ip = s->dst_addr;
            } else {
                printf("  → 无条件跳转: 地址超出范围\n");
            }
            break;
            
        case OP_CALL:
            if (call_sp < 32) {
                call_stack[call_sp++] = container.ip;
                if (s->dst_addr < sym_count) {
                    printf("  → 调用: 跳到符号[%d]，返回地址=%d\n", s->dst_addr, container.ip);
                    container.ip = s->dst_addr;
                }
            }
            break;
            
        case OP_RETURN:
            if (call_sp > 0) {
                container.ip = call_stack[--call_sp];
                printf("  → 返回: 回到符号[%d]\n", container.ip);
            } else {
                printf("  → 返回: 调用栈为空\n");
            }
            break;
            
        case OP_MIRROR:
            container_write(s->dst_addr, src_val);
            printf("  → 镜像: [%02X]=%02X → [%02X]=%02X\n", s->src_addr, src_val, s->dst_addr, src_val);
            break;
            
        case OP_EMPTY:
            for (int i = 0; i < s->strokes && i < CONTAINER_SIZE; i++) {
                container.data[i] = 0;
            }
            printf("  → 空集: 清除前%d个单元\n", s->strokes);
            break;
            
        case OP_FILTER:
            result = src_val & (unsigned char)(s->strokes & 0xFF);
            container_write(s->dst_addr, result);
            printf("  → 过滤: %02X & %02X = %02X\n", src_val, (unsigned char)(s->strokes & 0xFF), result);
            break;
            
        case OP_TRANSMIT:
            printf("  → 传递输出: 地址[%02X] = %02X (%d)\n", s->src_addr, src_val, src_val);
            printf("    >>> 输出值: %d <<<\n", src_val);
            break;
            
        case OP_ADD:
            result = (src_val + dst_val) & 0xFF;
            container_write(s->dst_addr, result);
            printf("  → 加法: %02X + %02X = %02X\n", src_val, dst_val, result);
            break;
            
        case OP_SUB:
            result = (src_val - dst_val) & 0xFF;
            container_write(s->dst_addr, result);
            printf("  → 减法: %02X - %02X = %02X\n", src_val, dst_val, result);
            break;
            
        case OP_AND:
            result = src_val & dst_val;
            container_write(s->dst_addr, result);
            printf("  → 与: %02X & %02X = %02X\n", src_val, dst_val, result);
            break;
            
        case OP_OR:
            result = src_val | dst_val;
            container_write(s->dst_addr, result);
            printf("  → 或: %02X | %02X = %02X\n", src_val, dst_val, result);
            break;
            
        case OP_XOR:
            result = src_val ^ dst_val;
            container_write(s->dst_addr, result);
            printf("  → 异或: %02X ^ %02X = %02X\n", src_val, dst_val, result);
            break;
            
        default:
            printf("  → 未知操作: %02X\n", s->op);
            break;
    }
}

/* ========== 执行所有符号 ========== */
void exec_all(void) {
    printf("\n========== 开始执行 ==========\n");
    printf("容器域: %s\n", container.domain);
    printf("归属: %s\n", container.owner);
    printf("符号总数: %d\n", sym_count);
    
    int steps = 0;
    int max_steps = sym_count * 2;  /* 防止无限循环 */
    
    while (container.ip < sym_count && steps < max_steps && container.state == 1) {
        Symbol *s = &sym_table[container.ip];
        int next_ip = container.ip + 1;
        
        exec_symbol(s);
        
        /* 如果ip没有被跳转指令修改，就继续下一个 */
        if (container.ip == next_ip - 1) {
            container.ip = next_ip;
        }
        steps++;
    }
    
    printf("\n========== 执行完成 ==========\n");
    printf("执行步数: %d\n", steps);
    printf("最终IP: %d\n", container.ip);
}

/* ========== 显示容器状态 ========== */
void display_container(void) {
    printf("\n========== 容器状态 ==========\n");
    printf("IP: %d  SP: %d  状态: %d\n", container.ip, container.sp, container.state);
    printf("域: %s  归属: %s\n", container.domain, container.owner);
    
    printf("\n容器数据（前64单元）:\n");
    for (int i = 0; i < 64; i += 16) {
        printf("  [%02X]", i);
        for (int j = 0; j < 16; j++) {
            printf(" %02X", container.data[i + j]);
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
        printf("\n意交感息容器引擎 v0.2\n");
        return 1;
    }
    
    printf("意交感息（isi）容器引擎 v0.2\n");
    printf("加载文件: %s\n", argv[1]);
    
    /* 加载符号文件 */
    if (load_symbol_file(argv[1]) != 0) {
        return 1;
    }
    
    /* 显示符号表 */
    display_symbols();
    
    /* 链接器：按域身份索引定位 */
    linker_link();
    
    /* 初始化容器 */
    container_init();
    
    /* 执行所有符号 */
    exec_all();
    
    /* 显示容器最终状态 */
    display_container();
    
    return 0;
}
