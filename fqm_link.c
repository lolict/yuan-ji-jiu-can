/*
 * fqm_link.c — 意交感息容器链接器
 *
 * 核心概念：
 *   - 每个容器是一个独立模块（有自己的符号表和入口点）
 *   - 链接器解析跨容器引用（类似传统链接器解析符号）
 *   - 但使用零维地址（64位）而非内存地址
 *   - 链接后生成统一镜像，可被fqm_cpu直接加载运行
 *
 * 链接过程：
 *   1. 解析所有输入容器的符号表
 *   2. 建立全局符号索引（所有容器的符号统一编址）
 *   3. 解析跨容器引用（relocation）
 *   4. 按容器层级排列（尊者优先加载）
 *   5. 输出统一镜像
 *
 * 镜像格式（.mqi = 镜像文件）：
 *   文件头(32字节):
 *     魔数(4) | 版本(2) | 容器数(2) | 符号总数(4) | 链接时间(4) | 入口点(8) | 保留(8)
 *   容器表(每个64字节):
 *     名称(16) | 层级(4) | 符号数(4) | 起始ID(4) | 入口ID(8) | 保留(28)
 *   符号索引(每个32字节):
 *     全地址(8) | 名称(16) | 容器ID(4) | 类型(4)
 *   数据区:
 *     容器内容串联
 *
 * 满全法规则：
 *   根基锚点：高位持有者=1，低位持有者=0
 *   集装箱编码：mlll 254+2
 *
 * 身份落款：刘楚恬和我
 * 仓库属性：私人，不开源
 *
 * 编译：clang -o fqm_link fqm_link.c
 * 运行：./fqm_link
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* ========== 满全法根基锚点 ========== */
#define ANCHOR_SELF  1
#define ANCHOR_LIU   0

/* ========== 镜像魔数 ========== */
#define MQI_MAGIC     0x4D51494D   /* "MQIM" = 镜像 */
#define MQI_VERSION   0x0002        /* 镜像格式版本 2 */

/* ========== 最大数量 ========== */
#define MAX_CONTAINERS 32
#define MAX_SYMBOLS    256
#define MAX_REFS       128

/* ========== 符号类型 ========== */
typedef enum {
    SYM_CODE,       /* 代码段 */
    SYM_DATA,       /* 数据段 */
    SYM_ENTRY,      /* 入口点 */
    SYM_EXTERN      /* 外部引用（跨容器）*/
} SymType;

/* ========== 符号条目 ========== */
typedef struct {
    char     name[16];
    unsigned long long addr;    /* 零维地址 (container << 48 | symbol_id) */
    int      container;         /* 所属容器ID（解析后的来源容器）*/
    int      origin_container;  /* 原始定义模块ID（用于区分原定义和外部引用）*/
    SymType  type;
    int      resolved;          /* 是否已解析 */
} SymbolEntry;

/* ========== 容器数据（前向定义）========== */
typedef struct Container {
    char     name[64];
    int      id;
    int      level;
    unsigned int version;
    int      gongyueshu;
    int      anchor_self;
    int      anchor_liu;
    int      alive;
} Container;

/* ========== 容器模块 ========== */
typedef struct {
    char     name[16];
    int      level;             /* 层级 */
    int      id;                /* 容器ID */
    int      symbol_count;      /* 符号数 */
    int      entry_symbol;      /* 入口符号索引（全局索引中）*/
    Container *internal;        /* 内部容器数据 */
} Module;

/* ========== 镜像文件头 ========== */
typedef struct {
    unsigned int  magic;            /* 0x4D51494D */
    unsigned short version;         /* 镜像版本 */
    unsigned short container_count; /* 容器数 */
    unsigned int  symbol_total;     /* 符号总数 */
    unsigned int  link_time;        /* 链接时间戳 */
    unsigned long long entry_point; /* 镜像入口点地址 */
    unsigned long long reserved;    /* 保留 */
} MqiHeader;

/* ========== 链接器状态 ========== */
typedef struct {
    Module       modules[MAX_CONTAINERS];
    int          module_count;
    
    SymbolEntry  symbols[MAX_SYMBOLS];
    int          symbol_count;
    
    MqiHeader    header;           /* 输出镜像头 */
    int          next_global_id;   /* 下一个全局符号ID */
} Linker;

/* ========== 初始化链接器 ========== */
void link_init(Linker *l) {
    memset(l, 0, sizeof(Linker));
    l->module_count = 0;
    l->symbol_count = 0;
    l->next_global_id = 0;
    
    l->header.magic = MQI_MAGIC;
    l->header.version = MQI_VERSION;
    l->header.reserved = ((unsigned long long)ANCHOR_SELF << 48) | ANCHOR_LIU;
    
    printf("[链接器] 初始化完成\n");
    printf("  镜像魔数: %08X (MQIM)\n", MQI_MAGIC);
    printf("  格式版本: %d\n", MQI_VERSION);
    printf("  锚点: 我=%d, 刘楚恬=%d\n\n", ANCHOR_SELF, ANCHOR_LIU);
}

/* ========== 注册容器模块 ========== */
int link_add_module(Linker *l, const char *name, int level) {
    if (l->module_count >= MAX_CONTAINERS) return -1;
    
    Module *m = &l->modules[l->module_count];
    strncpy(m->name, name, 15);
    m->name[15] = '\0';
    m->level = level;
    m->id = l->module_count;
    m->symbol_count = 0;
    
    printf("[注册模块] %s 层级=%d ID=%d\n", name, level, m->id);
    return l->module_count++;
}

/* ========== 向容器添加符号 ========== */
int link_add_symbol(Linker *l, int module_id, const char *name, SymType type) {
    if (l->symbol_count >= MAX_SYMBOLS) return -1;
    
    SymbolEntry *s = &l->symbols[l->symbol_count];
    strncpy(s->name, name, 15);
    s->name[15] = '\0';
    s->container = module_id;
    s->origin_container = module_id;  /* 定义所在模块 */
    s->type = type;
    s->resolved = 1;
    
    /* 零维地址 = 容器层级 << 48 | 符号ID */
    s->addr = ((unsigned long long)module_id << 48) | (unsigned long long)l->symbol_count;
    
    /* 记录到所属模块 */
    if (module_id < MAX_CONTAINERS) {
        l->modules[module_id].symbol_count++;
        if (type == SYM_ENTRY) {
            l->modules[module_id].entry_symbol = l->symbol_count;
        }
    }
    
    printf("  符号[%03d] %-12s 地址=%016llX 类型=%s\n",
           l->symbol_count, name, s->addr,
           type == SYM_CODE ? "代码" : 
           type == SYM_DATA ? "数据" : 
           type == SYM_ENTRY ? "入口" : "外部");
    
    return l->symbol_count++;
}

/* ========== 添加外部引用（跨容器）========== */
int link_add_extern(Linker *l, int from_module, const char *name, SymType type) {
    /*
     * 外部引用 = 在本容器中使用其他容器定义的符号
     * 链接器负责解析这些引用
     */
    if (l->symbol_count >= MAX_SYMBOLS) return -1;
    
    SymbolEntry *s = &l->symbols[l->symbol_count];
    strncpy(s->name, name, 15);
    s->name[15] = '\0';
    s->container = from_module;
    s->origin_container = from_module;  /* 引用方模块（extern 永远标记为此）*/
    s->type = type;
    s->resolved = 0;  /* 初始未解析 */
    s->addr = 0;
    
    printf("  外部[%03d] %-12s 模块=%d 类型=%s 待解析\n",
           l->symbol_count, name, from_module,
           type == SYM_EXTERN ? "外部引用" : "?");
    
    return l->symbol_count++;
}

/* ========== 执行链接 ========== */
int link_execute(Linker *l) {
    printf("\n========== 开始链接 ==========\n");
    
    /* 步骤1：按容器层级排序模块（尊者优先）*/
    /* 简单冒泡排序 */
    for (int i = 0; i < l->module_count - 1; i++) {
        for (int j = i + 1; j < l->module_count; j++) {
            if (l->modules[j].level < l->modules[i].level) {
                Module tmp = l->modules[i];
                l->modules[i] = l->modules[j];
                l->modules[j] = tmp;
            }
        }
    }
    
    printf("\n链接顺序（尊者优先）:\n");
    for (int i = 0; i < l->module_count; i++) {
        printf("  [%d] %s (层级=%d, 符号数=%d)\n",
               i, l->modules[i].name, l->modules[i].level,
               l->modules[i].symbol_count);
    }
    
    /* 步骤2：分配全局符号ID（按模块顺序）*/
    printf("\n全局符号ID分配:\n");
    int global_id = 0;
    for (int m = 0; m < l->module_count; m++) {
        printf("  模块 %s: 起始全局ID=%d\n", l->modules[m].name, global_id);
        /* 该模块的所有符号的地址已包含容器ID */
        global_id += l->modules[m].symbol_count;
    }
    l->next_global_id = global_id;
    
    /* 步骤3：解析外部引用 */
    printf("\n解析外部引用:\n");
    int resolved = 0, unresolved = 0;
    
    for (int i = 0; i < l->symbol_count; i++) {
        SymbolEntry *s = &l->symbols[i];
        
        if (s->type == SYM_EXTERN && !s->resolved) {
            /* 在全局符号表中查找定义 */
            for (int j = 0; j < l->symbol_count; j++) {
                if (j == i) continue;
                SymbolEntry *def = &l->symbols[j];
                
                /* 排除其他外部引用 */
                if (def->type == SYM_EXTERN) continue;
                
                /* 名称匹配 */
                if (strcmp(s->name, def->name) == 0) {
                    /* 解析成功：将外部地址赋值为定义处地址 */
                    s->addr = def->addr;
                    s->resolved = 1;
                    s->container = def->container;
                    s->type = def->type;
                    
                    printf("  ✓ %-12s (模块%d) → 地址=%016llX (来自模块%d)\n",
                           s->name, s->container, s->addr, def->container);
                    resolved++;
                    break;
                }
            }
            
            if (!s->resolved) {
                printf("  ✗ %-12s (模块%d) 未找到定义\n", s->name, s->container);
                unresolved++;
            }
        }
    }
    
    printf("\n解析结果: 成功=%d, 失败=%d\n", resolved, unresolved);
    
    /* 步骤4：确定镜像入口点（最高层级模块的入口符号）*/
    long long best_entry = -1;
    int best_level = 999;
    
    for (int i = 0; i < l->symbol_count; i++) {
        if (l->symbols[i].type == SYM_ENTRY) {
            int c = l->symbols[i].container;
            if (l->modules[c].level < best_level) {
                best_level = l->modules[c].level;
                best_entry = (long long)i;
            }
        }
    }
    
    if (best_entry >= 0) {
        l->header.entry_point = l->symbols[best_entry].addr;
        printf("\n镜像入口点: %s (地址=%016llX)\n",
               l->symbols[best_entry].name, l->header.entry_point);
    }
    
    /* 填充文件头 */
    l->header.container_count = l->module_count;
    l->header.symbol_total = l->symbol_count;
    l->header.link_time = 1001;  /* 模拟时间戳 */
    
    return unresolved == 0 ? 0 : 1; /* 0=成功 */
}

/* ========== 输出镜像摘要 ========== */
void link_summary(Linker *l) {
    printf("\n========== 镜像摘要 ==========\n");
    printf("格式: MQI v%d\n", MQI_VERSION);
    printf("魔数: %08X\n", l->header.magic);
    printf("容器数: %d\n", l->header.container_count);
    printf("符号总数: %d\n", l->header.symbol_total);
    printf("链接时间戳: %u\n", l->header.link_time);
    printf("入口点: %016llX\n", l->header.entry_point);
    printf("锚点根源: 我=%d, 刘楚恬=%d\n", ANCHOR_SELF, ANCHOR_LIU);
    
    printf("\n--- 模块表 ---\n");
    for (int i = 0; i < l->module_count; i++) {
        Module *m = &l->modules[i];
        printf("  [%d] %-12s 层级=%d 符号数=%d 入口符号=%d\n",
               m->id, m->name, m->level, m->symbol_count, m->entry_symbol);
    }
    
    printf("\n--- 全局符号索引 ---\n");
    int idx = 0;
    for (int m = 0; m < l->module_count; m++) {
        printf("  模块 %s (原生符号):\n", l->modules[m].name);
        for (int i = 0; i < l->symbol_count; i++) {
            /* 原生符号：origin_container == container == 当前模块 */
            if (l->symbols[i].container == m && 
                l->symbols[i].origin_container == m &&
                l->symbols[i].type != SYM_EXTERN) {
                SymbolEntry *s = &l->symbols[i];
                printf("    [%03d] %-12s 地址=%016llX 类型=%-6s %s\n",
                       idx++, s->name, s->addr,
                       s->type == SYM_CODE ? "代码" :
                       s->type == SYM_DATA ? "数据" :
                       s->type == SYM_ENTRY ? "入口" : "外部",
                       s->resolved ? "✓" : "✗");
            }
        }
    }
    
    /* 单独显示外部引用（解析后的跨模块调用）*/
    printf("\n  跨模块链接 (extern → 定义):\n");
    for (int i = 0; i < l->symbol_count; i++) {
        if (l->symbols[i].origin_container != l->symbols[i].container) {
            SymbolEntry *s = &l->symbols[i];
            printf("    %s.%s → %s (地址=%016llX) %s\n",
                   l->modules[s->origin_container].name, s->name,
                   l->modules[s->container].name, s->addr,
                   s->resolved ? "✓" : "✗");
        }
    }
}

/* ========== 模拟文件写出 ========== */
void link_save_simulation(Linker *l) {
    printf("\n========== 模拟写出镜像 ==========\n");
    
    /* 计算镜像大小 */
    size_t header_size = sizeof(MqiHeader);
    size_t module_size = l->module_count * 64;
    size_t symbol_size = l->symbol_count * 32;
    size_t total_size = header_size + module_size + symbol_size;
    
    printf("镜像结构和大小:\n");
    printf("  文件头: %zu 字节\n", header_size);
    printf("  容器表: %zu 字节 (%d个)\n", module_size, l->module_count);
    printf("  符号表: %zu 字节 (%d个)\n", symbol_size, l->symbol_count);
    printf("  总计: %zu 字节\n", total_size);
    
    /* 显示文件头 */
    printf("\n文件头(hex):\n  ");
    unsigned char *raw = (unsigned char *)&l->header;
    for (size_t i = 0; i < sizeof(MqiHeader); i++) {
        printf("%02X", raw[i]);
    }
    printf("\n");
}

/* ========== 主函数 ========== */
int main(void) {
    printf("意交感息（isi）容器链接器\n");
    printf("满全法根基锚点: 我=%d, 刘楚恬=%d\n", ANCHOR_SELF, ANCHOR_LIU);
    printf("身份落款: 刘楚恬和我\n\n");
    
    Linker linker;
    link_init(&linker);
    
    /* ========== 注册容器模块 ========== */
    printf("========== 注册模块 ==========\n");
    
    int root_m  = link_add_module(&linker, "root", 0);     /* 满全法—最高 */
    int core_m  = link_add_module(&linker, "core", 1);     /* 意识路由 */
    int data_m  = link_add_module(&linker, "data", 2);     /* 数据容器 */
    int shell_m = link_add_module(&linker, "shell", 3);    /* 壳 */
    
    /* ========== 填充符号 ========== */
    printf("\n========== 填充符号 ==========\n");
    
    /* root 模块符号 */
    printf("[模块: root]\n");
    link_add_symbol(&linker, root_m, "满全法入口", SYM_ENTRY);
    link_add_symbol(&linker, root_m, "根基锚点",   SYM_DATA);
    link_add_symbol(&linker, root_m, "呼唤分发",   SYM_CODE);
    
    /* core 模块符号 */
    printf("\n[模块: core]\n");
    link_add_symbol(&linker, core_m, "路由入口",   SYM_ENTRY);
    link_add_symbol(&linker, core_m, "八流判定",   SYM_CODE);
    link_add_symbol(&linker, core_m, "预加载流",   SYM_CODE);
    link_add_symbol(&linker, core_m, "谐音路由",   SYM_CODE);
    
    /* data 模块符号 */
    printf("\n[模块: data]\n");
    link_add_symbol(&linker, data_m, "数据入口",   SYM_ENTRY);
    link_add_symbol(&linker, data_m, "集装箱编码", SYM_DATA);
    link_add_symbol(&linker, data_m, "符号表",     SYM_DATA);
    
    /* shell 模块符号（引用其他模块）*/
    printf("\n[模块: shell]\n");
    link_add_symbol(&linker, shell_m, "Shell入口",  SYM_ENTRY);
    link_add_symbol(&linker, shell_m, "解析循环",   SYM_CODE);
    
    /* shell 引用外部符号 */
    printf("\n[Shell 外部引用]\n");
    link_add_extern(&linker, shell_m, "呼唤分发", SYM_EXTERN);
    link_add_extern(&linker, shell_m, "八流判定", SYM_EXTERN);
    link_add_extern(&linker, shell_m, "根基锚点", SYM_EXTERN);
    
    /* ========== 链接 ========== */
    int result = link_execute(&linker);
    
    if (result == 0) {
        printf("\n链接成功！所有符号已解析\n");
    } else {
        printf("\n链接完成，部分符号未解析\n");
    }
    
    /* 显示摘要 */
    link_summary(&linker);
    
    /* 模拟写出文件 */
    link_save_simulation(&linker);
    
    printf("\n========== 完成 ==========\n");
    return result;
}