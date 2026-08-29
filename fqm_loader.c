/*
 * fqm_loader.c — 意交感息最小加载器
 * 
 * 功能：读取符号文件，按容器-进制规则解析，执行/显示
 * 依赖：无外部库，只用标准C
 * 编译：gcc -o fqm_loader fqm_loader.c
 * 运行：./fqm_loader <符号文件路径>
 * 
 * 规则（基于你的定义）：
 *   - 偏旁 = 操作类型
 *   - 笔画数 = 操作数长度
 *   - 声母 = 源地址
 *   - 韵母 = 目标地址
 *   - 声调 = 模式切换
 *   - 结构 = 内存布局
 *   - 五笔 = 寄存器映射
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

/* ========== 符号表 ========== */

#define MAX_SYMBOLS 256
#define MAX_LINE 1024

/* 操作类型（偏旁映射） */
typedef enum {
    OP_READ    = 0x01,  /* 贝 — 读取 */
    OP_WRITE   = 0x02,  /* 扌 — 写入 */
    OP_EXEC    = 0x03,  /* 讠 — 执行 */
    OP_JUMP    = 0x04,  /* 辶 — 跳转 */
    OP_CALL    = 0x05,  /* 口 — 调用 */
    OP_RETURN  = 0x06,  /* 囗 — 返回 */
    OP_MIRROR  = 0x07,  /* 人 — 镜像 */
    OP_EMPTY   = 0x08,  /* 穴 — 空集 */
    OP_FILTER  = 0x09,  /* 氵 — 过滤 */
    OP_TRANSMIT= 0x0A,  /* 火 — 传递 */
    OP_UNKNOWN = 0xFF
} OpType;

/* 符号结构 */
typedef struct {
    char name[64];       /* 符号名称 */
    char radical[16];    /* 偏旁 */
    int  strokes;        /* 笔画数 */
    char initial[8];     /* 声母 */
    char final[16];      /* 韵母 */
    int  tone;           /* 声调 1-4 */
    char wubi[8];        /* 五笔 */
    OpType op;           /* 操作类型 */
    int  src_addr;       /* 源地址（声母映射） */
    int  dst_addr;       /* 目标地址（韵母映射） */
    int  mode;           /* 模式（声调映射） */
} Symbol;

/* 符号表 */
static Symbol sym_table[MAX_SYMBOLS];
static int sym_count = 0;

/* ========== 偏旁到操作码的映射 ========== */

OpType radical_to_op(const char *radical) {
    if (strcmp(radical, "贝") == 0) return OP_READ;
    if (strcmp(radical, "扌") == 0) return OP_WRITE;
    if (strcmp(radical, "讠") == 0) return OP_EXEC;
    if (strcmp(radical, "辶") == 0) return OP_JUMP;
    if (strcmp(radical, "口") == 0) return OP_CALL;
    if (strcmp(radical, "囗") == 0) return OP_RETURN;
    if (strcmp(radical, "人") == 0) return OP_MIRROR;
    if (strcmp(radical, "穴") == 0) return OP_EMPTY;
    if (strcmp(radical, "氵") == 0) return OP_FILTER;
    if (strcmp(radical, "火") == 0) return OP_TRANSMIT;
    return OP_UNKNOWN;
}

/* ========== 声母到地址的映射 ========== */

int initial_to_addr(const char *initial) {
    /* 声母表：b p m f d t n l g k h j q x zh ch sh r z c s y w */
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

/* ========== 韵母到地址的映射 ========== */

int final_to_addr(const char *final) {
    /* 基础韵母表 */
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

/* ========== 声调到模式的映射 ========== */

int tone_to_mode(int tone) {
    switch (tone) {
        case 1: return 0;  /* 阴平 — 默认模式 */
        case 2: return 1;  /* 阳平 — 执行模式 */
        case 3: return 2;  /* 上声 — 调试模式 */
        case 4: return 3;  /* 去声 — 特权模式 */
        default: return 0;
    }
}

/* ========== 符号表操作 ========== */

/* 添加符号 */
int add_symbol(const char *name, const char *radical, int strokes,
               const char *initial, const char *final, int tone,
               const char *wubi) {
    if (sym_count >= MAX_SYMBOLS) return -1;
    
    Symbol *s = &sym_table[sym_count];
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
    
    sym_count++;
    return 0;
}

/* 查找符号 */
Symbol* find_symbol(const char *name) {
    for (int i = 0; i < sym_count; i++) {
        if (strcmp(sym_table[i].name, name) == 0) {
            return &sym_table[i];
        }
    }
    return NULL;
}

/* ========== 文件解析 ========== */

/* 解析一行符号定义：名称 偏旁 笔画 声母 韵母 声调 五笔 */
int parse_line(char *line) {
    char name[64], radical[16], initial[8], final[16], wubi[8];
    int strokes, tone;
    
    if (sscanf(line, "%63s %15s %d %7s %15s %d %7s",
               name, radical, &strokes, initial, final, &tone, wubi) == 7) {
        return add_symbol(name, radical, strokes, initial, final, tone, wubi);
    }
    return -1;
}

/* 加载符号文件 */
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
        
        /* 去掉末尾换行 */
        line[strcspn(line, "\n")] = '\0';
        
        /* 跳过空行和注释 */
        if (line[0] == '\0' || line[0] == '#') continue;
        
        if (parse_line(line) != 0) {
            fprintf(stderr, "警告：第%d行解析失败: %s\n", line_num, line);
        }
    }
    
    fclose(fp);
    return 0;
}

/* ========== 执行引擎 ========== */

/* 执行一个符号 */
void exec_symbol(Symbol *s) {
    printf("[执行] %s | 操作:%02X | 源:%02X | 目标:%02X | 模式:%d | 笔画:%d\n",
           s->name, s->op, s->src_addr, s->dst_addr, s->mode, s->strokes);
    
    switch (s->op) {
        case OP_READ:
            printf("  → 读取操作：从地址 %02X 读取 %d 字节\n", s->src_addr, s->strokes);
            break;
        case OP_WRITE:
            printf("  → 写入操作：向地址 %02X 写入 %d 字节\n", s->dst_addr, s->strokes);
            break;
        case OP_EXEC:
            printf("  → 执行操作：跳转到地址 %02X\n", s->dst_addr);
            break;
        case OP_JUMP:
            printf("  → 跳转到地址 %02X\n", s->dst_addr);
            break;
        case OP_CALL:
            printf("  → 调用子程序：地址 %02X\n", s->dst_addr);
            break;
        case OP_RETURN:
            printf("  → 返回\n");
            break;
        case OP_MIRROR:
            printf("  → 镜像操作：源 %02X → 目标 %02X\n", s->src_addr, s->dst_addr);
            break;
        case OP_EMPTY:
            printf("  → 空集：在独立环境中自主演化\n");
            break;
        case OP_FILTER:
            printf("  → 过滤操作：保留可用功能\n");
            break;
        case OP_TRANSMIT:
            printf("  → 传递操作：输出到地址 %02X\n", s->dst_addr);
            break;
        default:
            printf("  → 未知操作\n");
            break;
    }
}

/* 执行所有符号 */
void exec_all(void) {
    printf("\n========== 开始执行 ==========\n");
    printf("符号总数: %d\n\n", sym_count);
    
    for (int i = 0; i < sym_count; i++) {
        exec_symbol(&sym_table[i]);
    }
    
    printf("\n========== 执行完成 ==========\n");
}

/* ========== 显示符号表 ========== */

void display_symbols(void) {
    printf("\n========== 符号表 ==========\n");
    printf("%-8s %-6s %-4s %-6s %-8s %-4s %-6s %-6s %-6s %-6s\n",
           "名称", "偏旁", "笔画", "声母", "韵母", "声调", "五笔", "操作", "源", "目标");
    printf("--------------------------------------------------------------------------\n");
    
    for (int i = 0; i < sym_count; i++) {
        Symbol *s = &sym_table[i];
        printf("%-8s %-6s %-4d %-6s %-8s %-4d %-6s %-6X %-6X %-6X\n",
               s->name, s->radical, s->strokes, s->initial, s->final,
               s->tone, s->wubi, s->op, s->src_addr, s->dst_addr);
    }
    
    printf("--------------------------------------------------------------------------\n");
    printf("共 %d 个符号\n", sym_count);
}

/* ========== 主函数 ========== */

void print_usage(const char *prog) {
    printf("用法: %s <符号文件路径>\n", prog);
    printf("\n符号文件格式（每行一个符号）：\n");
    printf("  名称 偏旁 笔画数 声母 韵母 声调 五笔\n");
    printf("\n示例：\n");
    printf("  读取 贝 4 贝 贝 1 gm\n");
    printf("  写入 扌 3 扌 扌 1 rfh\n");
}

int main(int argc, char *argv[]) {
    if (argc != 2) {
        print_usage(argv[0]);
        return 1;
    }
    
    printf("意交感息（isi）最小加载器 v0.1\n");
    printf("加载文件: %s\n", argv[1]);
    
    /* 加载符号文件 */
    if (load_symbol_file(argv[1]) != 0) {
        return 1;
    }
    
    /* 显示符号表 */
    display_symbols();
    
    /* 执行所有符号 */
    exec_all();
    
    return 0;
}
