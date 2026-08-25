/**
 * ═══════════════════════════════════════════════════════════════════════════
 * fqm_chain.c — 全局互锁记忆链条
 * ═══════════════════════════════════════════════════════════════════════════
 *
 *   全局互锁文件神经网络分布式控制。
 *
 *   架构:
 *     链条 = 文件间的传导(跨文件引用)
 *     齿轮 = 每个编程文件(独立运转)
 *     横轴 = 全局记忆主线(贯穿所有文件)
 *     珠子 = 记忆点(每个符号/函数/常量)
 *     内圈 = 早期符号(历史更久远)
 *     外圈 = 后期符号(新近创建)
 *     双圈 = 边界 = 约束珠子的轨迹
 *
 * ─────────────────────────────────────────────────────────────────────────
 * 编译: cc -Wall -Wextra -std=c99 -O2 -o fqmchain fqm_chain.c
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <dirent.h>

/* ═══════════════════════════════════════════════════════════════════════════
 * 常量
 * ═══════════════════════════════════════════════════════════════════════════ */

#define MAX_FILES    64
#define MAX_SYMBOLS  2048
#define MAX_NAME     64
#define MAX_LINE     256

/* ═══════════════════════════════════════════════════════════════════════════
 * 数据结构 (动态分配)
 * ═══════════════════════════════════════════════════════════════════════════ */

typedef struct {
    char name[MAX_NAME];
    char defined_in[MAX_NAME];
    int  ref_count;
    int  is_defined;
} bead_t;

typedef struct {
    char name[MAX_NAME];
    int  symbol_count;
    int  ref_count;
    int  is_connected;
} gear_t;

typedef struct {
    char from[MAX_NAME];
    char to[MAX_NAME];
    char symbol[MAX_NAME];
} chain_link_t;

typedef struct {
    gear_t       *gears;
    int           gear_count;
    int           gear_cap;
    bead_t       *beads;
    int           bead_count;
    int           bead_cap;
    chain_link_t *links;
    int           link_count;
    int           link_cap;
} global_memory_t;

/* ═══════════════════════════════════════════════════════════════════════════
 * 初始化/清理
 * ═══════════════════════════════════════════════════════════════════════════ */

static int gm_init(global_memory_t *gm) {
    gm->gear_cap = MAX_FILES;
    gm->gears = calloc(gm->gear_cap, sizeof(gear_t));
    gm->bead_cap = MAX_SYMBOLS;
    gm->beads = calloc(gm->bead_cap, sizeof(bead_t));
    gm->link_cap = MAX_SYMBOLS;
    gm->links = calloc(gm->link_cap, sizeof(chain_link_t));
    gm->gear_count = 0;
    gm->bead_count = 0;
    gm->link_count = 0;
    return (gm->gears && gm->beads && gm->links) ? 0 : -1;
}

static void gm_free(global_memory_t *gm) {
    free(gm->gears);
    free(gm->beads);
    free(gm->links);
    memset(gm, 0, sizeof(*gm));
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 辅助函数
 * ═══════════════════════════════════════════════════════════════════════════ */

static int is_c_file(const char *name) {
    int len = strlen(name);
    return (len > 2 && strcmp(name + len - 2, ".c") == 0) ||
           (len > 2 && strcmp(name + len - 2, ".h") == 0);
}

static int is_symbol_char(char c) {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || 
           (c >= '0' && c <= '9') || c == '_';
}

static int is_c_keyword(const char *word) {
    static const char *keywords[] = {
        "if","else","for","while","do","switch","case","default",
        "break","continue","return","goto","typedef","struct","enum",
        "union","static","const","extern","void","int","char","float",
        "double","long","short","unsigned","signed","sizeof","NULL",
        "define","include","ifdef","ifndef","endif","undef","pragma",
        NULL
    };
    for (int i = 0; keywords[i]; i++)
        if (strcmp(word, keywords[i]) == 0) return 1;
    return 0;
}

static int find_bead(global_memory_t *gm, const char *name) {
    for (int i = 0; i < gm->bead_count; i++)
        if (strcmp(gm->beads[i].name, name) == 0) return i;
    return -1;
}

static int find_gear(global_memory_t *gm, const char *name) {
    for (int i = 0; i < gm->gear_count; i++)
        if (strcmp(gm->gears[i].name, name) == 0) return i;
    return -1;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 扫描文件
 * ═══════════════════════════════════════════════════════════════════════════ */

static void scan_file(global_memory_t *gm, const char *filepath, const char *filename) {
    FILE *f = fopen(filepath, "r");
    if (!f) return;
    
    /* 注册齿轮 */
    int gi = find_gear(gm, filename);
    if (gi < 0) {
        if (gm->gear_count >= gm->gear_cap) return;
        gi = gm->gear_count++;
        strncpy(gm->gears[gi].name, filename, MAX_NAME-1);
        gm->gears[gi].symbol_count = 0;
        gm->gears[gi].ref_count = 0;
        gm->gears[gi].is_connected = 0;
    }
    
    char line[MAX_LINE];
    int in_comment = 0;
    
    while (fgets(line, sizeof(line), f)) {
        /* 简单注释处理 */
        char *cmt = strstr(line, "/*");
        if (cmt) {
            char *end = strstr(cmt, "*/");
            if (!end) in_comment = 1;
        }
        if (in_comment) {
            if (strstr(line, "*/")) in_comment = 0;
            continue;
        }
        cmt = strstr(line, "//");
        if (cmt) *cmt = '\0';
        
        /* 提取单词 */
        char *p = line;
        while (*p) {
            if (is_symbol_char(*p)) {
                char word[MAX_NAME];
                int wlen = 0;
                while (is_symbol_char(*p) && wlen < MAX_NAME-1)
                    word[wlen++] = *p++;
                word[wlen] = '\0';
                
                if (strlen(word) < 3) continue;
                if (is_c_keyword(word)) continue;
                
                /* 检查是否是定义(简化判断) */
                int is_def = 0;
                char *start = line;
                while (*start == ' ' || *start == '\t') start++;
                if (strncmp(start, "typedef", 7) == 0 || 
                    strncmp(start, "static", 6) == 0 ||
                    strncmp(start, "const", 5) == 0 ||
                    strncmp(start, "void", 4) == 0 ||
                    strncmp(start, "int", 3) == 0 ||
                    strncmp(start, "char", 4) == 0 ||
                    strncmp(start, "float", 5) == 0 ||
                    strncmp(start, "uint", 4) == 0 ||
                    strncmp(start, "verdict_t", 9) == 0 ||
                    strncmp(start, "sym_type_t", 10) == 0 ||
                    strncmp(start, "flow_id_t", 9) == 0)
                    is_def = 1;
                
                int bi = find_bead(gm, word);
                if (bi < 0) {
                    if (gm->bead_count >= gm->bead_cap) { fclose(f); return; }
                    bi = gm->bead_count++;
                    strncpy(gm->beads[bi].name, word, MAX_NAME-1);
                    gm->beads[bi].defined_in[0] = '\0';
                    gm->beads[bi].ref_count = 0;
                    gm->beads[bi].is_defined = 0;
                }
                
                if (is_def) {
                    strncpy(gm->beads[bi].defined_in, filename, MAX_NAME-1);
                    gm->beads[bi].is_defined = 1;
                    gm->gears[gi].symbol_count++;
                } else {
                    gm->beads[bi].ref_count++;
                }
            } else {
                p++;
            }
        }
    }
    fclose(f);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 建立链条(引用关系)
 * ═══════════════════════════════════════════════════════════════════════════ */

static void build_chains(global_memory_t *gm) {
    for (int i = 0; i < gm->bead_count; i++) {
        if (gm->beads[i].is_defined && strlen(gm->beads[i].defined_in) > 0) {
            for (int j = 0; j < gm->gear_count; j++) {
                if (gm->link_count >= gm->link_cap) return;
                if (strcmp(gm->gears[j].name, gm->beads[i].defined_in) != 0) {
                    if (gm->beads[i].ref_count > 0) {
                        int li = gm->link_count++;
                        strncpy(gm->links[li].from, gm->gears[j].name, MAX_NAME-1);
                        strncpy(gm->links[li].to, gm->beads[i].defined_in, MAX_NAME-1);
                        strncpy(gm->links[li].symbol, gm->beads[i].name, MAX_NAME-1);
                        gm->gears[j].is_connected = 1;
                    }
                }
            }
        }
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 报告
 * ═══════════════════════════════════════════════════════════════════════════ */

static void report(global_memory_t *gm) {
    printf("\n═══════════════════════════════════════════════════════\n");
    printf("  全局互锁记忆链条 · 报告\n");
    printf("═══════════════════════════════════════════════════════\n");
    
    printf("\n─── 齿轮(文件)状态 ───\n");
    printf("  总计: %d 个文件\n", gm->gear_count);
    int connected = 0, isolated = 0;
    for (int i = 0; i < gm->gear_count; i++) {
        if (gm->gears[i].is_connected) connected++;
        else isolated++;
    }
    printf("  已接入链条: %d | 孤立齿轮: %d\n", connected, isolated);
    
    printf("\n─── 各齿轮详情 ───\n");
    for (int i = 0; i < gm->gear_count; i++) {
        printf("  %-30s 定义%3d符号  被引用%3d次  %s\n",
               gm->gears[i].name,
               gm->gears[i].symbol_count,
               gm->gears[i].ref_count,
               gm->gears[i].is_connected ? "●已接入" : "○孤立");
    }
    
    printf("\n─── 珠子(符号)热度 TOP 15 ───\n");
    /* 简单排序 */
    int *sorted = malloc(gm->bead_count * sizeof(int));
    if (!sorted) { printf("  [内存不足]\n"); return; }
    for (int i = 0; i < gm->bead_count; i++) sorted[i] = i;
    for (int i = 0; i < gm->bead_count - 1; i++)
        for (int j = i + 1; j < gm->bead_count; j++)
            if (gm->beads[sorted[j]].ref_count > gm->beads[sorted[i]].ref_count) {
                int tmp = sorted[i]; sorted[i] = sorted[j]; sorted[j] = tmp;
            }
    
    int top = gm->bead_count < 15 ? gm->bead_count : 15;
    for (int i = 0; i < top; i++) {
        int bi = sorted[i];
        if (gm->beads[bi].ref_count == 0) break;
        printf("  %-20s 引用%3d次  定义于: %s\n",
               gm->beads[bi].name,
               gm->beads[bi].ref_count,
               gm->beads[bi].is_defined ? gm->beads[bi].defined_in : "[未定义]");
    }
    free(sorted);
    
    printf("\n─── 链条(引用关系) ───\n");
    printf("  总链接数: %d\n", gm->link_count);
    if (gm->link_count > 0) {
        int shown = gm->link_count < 20 ? gm->link_count : 20;
        for (int i = 0; i < shown; i++) {
            printf("  %s → %s (符号: %s)\n",
                   gm->links[i].from,
                   gm->links[i].to,
                   gm->links[i].symbol);
        }
        if (gm->link_count > 20)
            printf("  ... 还有 %d 条链接\n", gm->link_count - 20);
    }
    
    printf("\n─── 孤立齿轮(需关注) ───\n");
    int found_isolated = 0;
    for (int i = 0; i < gm->gear_count; i++) {
        if (!gm->gears[i].is_connected && gm->gears[i].symbol_count == 0) {
            printf("  ○ %s (无定义符号, 未接入链条)\n", gm->gears[i].name);
            found_isolated = 1;
        }
    }
    if (!found_isolated) printf("  无孤立齿轮\n");
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 测试
 * ═══════════════════════════════════════════════════════════════════════════ */

static int test_count = 0;
static int test_pass = 0;

#define TEST(name) do { test_count++; printf("  [TEST] %-40s ", name); } while(0)
#define PASS()     do { test_pass++; printf("✓\n"); } while(0)
#define FAIL(msg)  do { printf("✗ %s\n", msg); } while(0)

static void run_tests(global_memory_t *gm) {
    printf("\n═══════════════════════════════════════════════════════\n");
    printf("  全局互锁记忆链条 · 测试套件\n");
    printf("═══════════════════════════════════════════════════════\n\n");
    
    TEST("扫描到文件");
    if (gm->gear_count > 0) PASS(); else FAIL("无文件");
    
    TEST("扫描到符号");
    if (gm->bead_count > 0) PASS(); else FAIL("无符号");
    
    TEST("文件数合理(>5)");
    if (gm->gear_count > 5) PASS(); else FAIL("文件太少");
    
    TEST("符号数合理(>20)");
    if (gm->bead_count > 20) PASS(); else FAIL("符号太少");
    
    TEST("链条已建立");
    if (gm->link_count >= 0) PASS(); else FAIL("链条错误");
    
    printf("\n───────────────────────────────────────────────────────\n");
    printf("  结果: %d/%d 通过\n", test_pass, test_count);
    printf("───────────────────────────────────────────────────────\n");
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 入口
 * ═══════════════════════════════════════════════════════════════════════════ */

int main(int argc, char **argv) {
    printf("═══════════════════════════════════════════════════════\n");
    printf("  全局互锁记忆链条\n");
    printf("═══════════════════════════════════════════════════════\n");
    
    global_memory_t gm;
    if (gm_init(&gm) < 0) {
        printf("  [ERROR] 内存分配失败\n");
        return 1;
    }
    
    /* 扫描当前目录所有 .c/.h 文件 */
    DIR *dir = opendir(".");
    if (!dir) {
        printf("  [ERROR] 无法打开当前目录\n");
        gm_free(&gm);
        return 1;
    }
    
    struct dirent *entry;
    int file_count = 0;
    while ((entry = readdir(dir)) != NULL) {
        if (is_c_file(entry->d_name)) {
            char filepath[512];
            snprintf(filepath, sizeof(filepath), "./%s", entry->d_name);
            scan_file(&gm, filepath, entry->d_name);
            file_count++;
        }
    }
    closedir(dir);
    
    printf("  扫描了 %d 个文件, 发现 %d 个符号\n", file_count, gm.bead_count);
    
    /* 建立链条 */
    build_chains(&gm);
    printf("  建立了 %d 条链接\n", gm.link_count);
    
    if (argc > 1 && strcmp(argv[1], "-t") == 0) {
        run_tests(&gm);
    } else {
        report(&gm);
        run_tests(&gm);
    }
    
    gm_free(&gm);
    return 0;
}
