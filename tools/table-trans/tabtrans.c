/*
 * tabtrans.c — 表格编程·全局静态翻译器
 * 目的: 把外来机器码静态对齐编入我们的地址空间(机器码进制 254+2 槽位, 满 254 折叠进位)
 * 允许: wasm / ELF32 / ELF64(arm64/arm32) / 汇编文本 符号直呼
 * 禁止: 乘除求编址(仅加法+折叠), 静默丢弃未译指令
 * 优先级: 真源表 table.chain 更特异掩码排前, 首个命中生效
 * 积累: 表行可持续扩充 ; 突破: 未译显形计数
 * 验收: make test — fact.wasm/arm64/arm32 全译出, 未译=0
 * 真源: tools/table-trans/table.chain ; 符号: SYMBOL_DICTIONARY.md 满全法容器族
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

#define MAXROWS 512
#define MAXLINE 512
#define FOLD_POINT 254

typedef enum { CT_ADD, CT_SPIRAL, CT_MIRROR, CT_FOLD, CT_MAX } CType;
static const char *ct_names[CT_MAX] = { "加法", "螺旋", "镜像", "折叠" };

typedef enum { IM_NONE, IM_U32, IM_S32, IM_S64, IM_MEMARG, IM_BRTABLE, IM_BLOCKTYPE, IM_MAX } ImmKind;
static const char *imm_names[IM_MAX] = { "none", "u32", "s32", "s64", "memarg", "brtable", "blocktype" };

typedef struct {
    char sym[16];
    char cdomain[32];
    CType ctype;
    char fdom[8];
    uint32_t op;
    ImmKind imm;
    uint32_t mask, val;
    char note[64];
    char source[64];
} Row;

static Row rows[MAXROWS];
static int nrows = 0;

static const char *g_table = "table.chain";

static void die(const char *msg) { fprintf(stderr, "tabtrans: %s\n", msg); exit(1); }

static int popcount32(uint32_t x) { int c = 0; while (x) { x &= x - 1; c++; } return c; }

static int row_is_arm(const Row *r) {
    return strcmp(r->fdom, "arm32") == 0 || strcmp(r->fdom, "arm64") == 0;
}

static int row_cmp(const void *a, const void *b) {
    const Row *ra = a, *rb = b;
    int c = strcmp(ra->fdom, rb->fdom);
    if (c != 0) return c;
    return popcount32(rb->mask) - popcount32(ra->mask);
}

static void check_ambiguity(void) {
    for (int i = 0; i < nrows; i++) {
        if (!row_is_arm(&rows[i])) continue;
        for (int j = i + 1; j < nrows; j++) {
            if (!row_is_arm(&rows[j]) || strcmp(rows[i].fdom, rows[j].fdom) != 0) continue;
            uint32_t shared = rows[i].mask & rows[j].mask;
            if ((rows[i].val ^ rows[j].val) & shared) continue;
            uint32_t mi = rows[i].mask, mj = rows[j].mask;
            if (mi == mj) die("载入拒绝: 两行同掩码同值(重复行)");
            if ((mi & ~mj) == 0 || (mj & ~mi) == 0) continue;
            fprintf(stderr, "载入拒绝: 行「%s」与行「%s」部分重叠, 需人工裁决\n",
                    rows[i].note[0] ? rows[i].note : rows[i].sym,
                    rows[j].note[0] ? rows[j].note : rows[j].sym);
            exit(1);
        }
    }
}

static int imm_from_str(const char *s) {
    for (int i = 0; i < IM_MAX; i++)
        if (strcmp(s, imm_names[i]) == 0) return i;
    return -1;
}

static int ctype_from_str(const char *s) {
    for (int i = 0; i < CT_MAX; i++)
        if (strcmp(s, ct_names[i]) == 0) return i;
    return -1;
}

static void load_table(void) {
    FILE *f = fopen(g_table, "r");
    if (!f) { fprintf(stderr, "tabtrans: 打不开真源表 %s\n", g_table); exit(1); }
    char line[MAXLINE];
    while (fgets(line, sizeof line, f)) {
        line[strcspn(line, "\r\n")] = 0;
        if (strncmp(line, "gttx://tab|", 11) != 0) continue;
        char *fields[8]; int nf = 0;
        char *p = line;
        while (nf < 8 && (p = strtok(p, "|")) != NULL) { fields[nf++] = p; p = NULL; }
        if (nf < 8 || fields[7][0] == 0) {
            fprintf(stderr, "tabtrans: 行缺来源列, 载入拒绝: %s\n", line);
            exit(1);
        }
        if (nrows >= MAXROWS) die("表满");
        Row *r = &rows[nrows++];
        memset(r, 0, sizeof *r);
        snprintf(r->sym, sizeof r->sym, "%s", fields[1]);
        snprintf(r->cdomain, sizeof r->cdomain, "%s", fields[2]);
        r->ctype = ctype_from_str(fields[3]);
        snprintf(r->fdom, sizeof r->fdom, "%s", fields[4]);
        r->op = 0; r->imm = IM_NONE; r->mask = 0; r->val = 0;
        snprintf(r->note, sizeof r->note, "%s", fields[6]);
        snprintf(r->source, sizeof r->source, "%s", fields[7]);
        char *enc = fields[5];
        if (strcmp(r->fdom, "wasm") == 0) {
            char *colon = strchr(enc, ':');
            if (!colon) die("wasm 行缺冒号");
            *colon = 0;
            r->op = (uint32_t)strtoul(enc, NULL, 16);
            int ik = imm_from_str(colon + 1);
            if (ik < 0) die("未知立即数种类");
            r->imm = (ImmKind)ik;
        } else {
            char *eq = strchr(enc, '=');
            if (!eq) die("arm 行缺等号");
            *eq = 0;
            r->mask = (uint32_t)strtoul(enc, NULL, 16);
            r->val = (uint32_t)strtoul(eq + 1, NULL, 16);
        }
    }
    fclose(f);
    if (nrows == 0) die("真源表无有效行");
    qsort(rows, (size_t)nrows, sizeof(Row), row_cmp);
    check_ambiguity();
}

static Row *find_wasm(uint32_t op) {
    for (int i = 0; i < nrows; i++)
        if (strcmp(rows[i].fdom, "wasm") == 0 && rows[i].op == op) return &rows[i];
    return NULL;
}

static Row *find_arm(const char *fdom, uint32_t word) {
    for (int i = 0; i < nrows; i++)
        if (strcmp(rows[i].fdom, fdom) == 0 && (word & rows[i].mask) == rows[i].val) return &rows[i];
    return NULL;
}

/* ── 地址空间: 加法串联 + 折叠进位 + 口诀清点格 ── */

static long serial = 0;
static long layer = 0, slot = 0;
static int grid_row = 1, grid_col = 1;
static long grid_count[9][9];
static long grid_round = 0;

#define MAXSYM 64
static char sym_names[MAXSYM][16];
static long sym_counts[MAXSYM];
static int sym_n = 0;
static long untranslated = 0;
static long last_serial_of_sym[MAXSYM];

static int sym_index(const char *s) {
    for (int i = 0; i < sym_n; i++)
        if (strcmp(sym_names[i], s) == 0) return i;
    if (sym_n >= MAXSYM) die("符号种满");
    snprintf(sym_names[sym_n], 16, "%s", s);
    sym_counts[sym_n] = 0;
    last_serial_of_sym[sym_n] = -1;
    return sym_n++;
}

static void grid_advance(void) {
    grid_col++;
    if (grid_col > 9) { grid_col = 1; grid_row++; }
    if (grid_row > 9) { grid_row = 1; grid_round++; }
}

static void emit_container(const char *sym, CType ct, const char *fdom, long off, const unsigned char *bytes, int blen) {
    serial++;
    slot++;
    if (slot >= FOLD_POINT) { layer++; slot = 0; }
    int si = sym_index(sym);
    sym_counts[si]++;
    last_serial_of_sym[si] = serial;
    grid_count[grid_row - 1][grid_col - 1]++;
    printf("#%04ld L%ld 槽%03ld 格%d行%d列 %s %s %s:%ld ",
           serial, layer, slot, grid_row, grid_col, sym, ct_names[ct], fdom, off);
    for (int i = 0; i < blen; i++) printf("%02x", bytes[i]);
    printf("\n");
    grid_advance();
}

static void emit_row(Row *r, const char *fdom, long off, const unsigned char *bytes, int blen) {
    emit_container(r->sym, r->ctype, fdom, off, bytes, blen);
}

static void emit_untranslated(const char *fdom, long off, const unsigned char *bytes, int blen) {
    untranslated++;
    printf("#%04ld ---- 未译 ---- %s:%ld ", serial + 1, fdom, off);
    for (int i = 0; i < blen; i++) printf("%02x", bytes[i]);
    printf("\n");
}

/* ── verify 模式: 精确清点覆盖, 无采样无概率 ── */

static void run_verify(const char *fdom) {
    if (strcmp(fdom, "wasm") == 0) {
        int covered = 0;
        for (int i = 0; i < nrows; i++)
            if (strcmp(rows[i].fdom, "wasm") == 0) covered++;
        printf("== verify %s ==\n", fdom);
        printf("覆盖操作码: %d / 256\n", covered);
        printf("歧义: 0 (字节码精确匹配, 无掩码)\n");
        return;
    }
    long sym_cnt[MAXSYM] = {0};
    for (int i = 0; i < sym_n; i++) sym_cnt[i] = 0;
    unsigned long long total = 0;
    for (int i = 0; i < nrows; i++) {
        Row *r = &rows[i];
        if (strcmp(r->fdom, fdom) != 0) continue;
        uint32_t fb = ~r->mask;
        unsigned long long row_hits = 0;
        for (uint32_t s = fb; ; s = (s - 1) & fb) {
            uint32_t w = r->val | s;
            Row *win = find_arm(fdom, w);
            if (win == r) {
                row_hits++;
                sym_cnt[sym_index(r->sym)]++;
            }
            if (s == 0) break;
        }
        total += row_hits;
    }
    printf("== verify %s ==\n", fdom);
    for (int i = 0; i < sym_n; i++)
        if (sym_cnt[i] > 0) printf("%s: %ld\n", sym_names[i], sym_cnt[i]);
    printf("覆盖字数: %llu / 4294967296\n", total);
    printf("覆盖率: %.4f%%\n", (double)total * 100.0 / 4294967296.0);
    printf("歧义: 0 (载入检查通过: 重叠行均严格特异有序)\n");
}

/* ── LEB128 ── */

static uint32_t read_uleb(const unsigned char *buf, long *p, long size) {
    uint32_t v = 0; int shift = 0;
    while (*p < size) {
        unsigned char b = buf[(*p)++];
        v |= (uint32_t)(b & 0x7F) << shift;
        if (!(b & 0x80)) break;
        shift += 7;
    }
    return v;
}

static int64_t read_sleb64(const unsigned char *buf, long *p, long size) {
    int64_t v = 0; int shift = 0; unsigned char b;
    while (*p < size) {
        b = buf[(*p)++];
        v |= (int64_t)(uint64_t)(b & 0x7F) << shift;
        shift += 7;
        if (!(b & 0x80)) break;
    }
    if (shift < 64 && (b & 0x40)) v |= (int64_t)(~(uint64_t)0 << shift);
    return v;
}

/* ── wasm 模式 ── */

static void run_wasm(const char *path) {
    FILE *f = fopen(path, "rb");
    if (!f) die("打不开 wasm 文件");
    fseek(f, 0, SEEK_END); long size = ftell(f); fseek(f, 0, SEEK_SET);
    unsigned char *buf = malloc((size_t)size);
    if (fread(buf, 1, (size_t)size, f) != (size_t)size) die("读 wasm 失败");
    fclose(f);
    if (size < 8 || memcmp(buf, "\0asm", 4) != 0) die("wasm 魔数不符");
    printf("== 域: wasm %s (%ld 字节) ==\n", path, size);
    long p = 8;
    while (p < size) {
        uint8_t id = buf[p++];
        uint32_t sz = read_uleb(buf, &p, size);
        long sec_start = p;
        if (id == 10) {
            uint32_t fn = read_uleb(buf, &p, size);
            printf("-- 代码段: %u 个函数 --\n", fn);
            for (uint32_t fi = 0; fi < fn && p < sec_start + (long)sz; fi++) {
                uint32_t body_sz = read_uleb(buf, &p, size);
                long body_end = p + (long)body_sz;
                uint32_t loc_n = read_uleb(buf, &p, size);
                for (uint32_t li = 0; li < loc_n; li++) {
                    read_uleb(buf, &p, size);
                    if (p < body_end) p++;
                }
                long ip = p;
                while (ip < body_end) {
                    long off = ip;
                    uint8_t op = buf[ip++];
                    Row *r = find_wasm(op);
                    if (!r) {
                        emit_untranslated("wasm", off, buf + off, 1);
                        continue;
                    }
                    switch (r->imm) {
                    case IM_NONE: break;
                    case IM_U32: read_uleb(buf, &ip, body_end); break;
                    case IM_S32: read_sleb64(buf, &ip, body_end); break;
                    case IM_S64: read_sleb64(buf, &ip, body_end); break;
                    case IM_MEMARG:
                        read_uleb(buf, &ip, body_end);
                        read_uleb(buf, &ip, body_end);
                        break;
                    case IM_BRTABLE: {
                        uint32_t n = read_uleb(buf, &ip, body_end);
                        for (uint32_t t = 0; t <= n; t++) read_uleb(buf, &ip, body_end);
                        break;
                    }
                    case IM_BLOCKTYPE:
                        if (ip < body_end) {
                            uint8_t bt = buf[ip];
                            if (bt == 0x40 || bt <= 0x7F) ip++;
                        }
                        break;
                    default: break;
                    }
                    emit_row(r, "wasm", off, buf + off, (int)(ip - off));
                }
                p = body_end;
            }
        }
        p = sec_start + (long)sz;
    }
    free(buf);
}

/* ── ARM 定长解码 ── */

static void run_arm_words(const char *fdom, const unsigned char *code, long len, const char *label) {
    printf("== 域: %s %s (%ld 字节) ==\n", fdom, label, len);
    long off = 0;
    while (off + 4 <= len) {
        uint32_t w = (uint32_t)code[off] | ((uint32_t)code[off + 1] << 8) |
                     ((uint32_t)code[off + 2] << 16) | ((uint32_t)code[off + 3] << 24);
        Row *r = find_arm(fdom, w);
        if (r) emit_row(r, fdom, off, code + off, 4);
        else emit_untranslated(fdom, off, code + off, 4);
        off += 4;
    }
}

/* ── ELF 模式 ── */

#define EM_ARM 40
#define EM_AARCH64 183
#define PT_LOAD 1
#define PF_X 1

static void run_elf(const char *path) {
    FILE *f = fopen(path, "rb");
    if (!f) die("打不开 ELF 文件");
    fseek(f, 0, SEEK_END); long size = ftell(f); fseek(f, 0, SEEK_SET);
    unsigned char *buf = malloc((size_t)size);
    if (fread(buf, 1, (size_t)size, f) != (size_t)size) die("读 ELF 失败");
    fclose(f);
    if (size < 52 || memcmp(buf, "\x7f" "ELF", 4) != 0) die("ELF 魔数不符");
    int cls = buf[4];
    int little = buf[5] == 1;
    if (!little) die("仅支持小端");
    uint16_t machine;
    long found = 0;
    if (cls == 2) {
        machine = (uint16_t)(buf[18] | (buf[19] << 8));
        uint64_t phoff; uint16_t phentsize, phnum;
        memcpy(&phoff, buf + 32, 8);
        memcpy(&phentsize, buf + 54, 2);
        memcpy(&phnum, buf + 56, 2);
        printf("== ELF64 machine=%u phnum=%u ==\n", machine, phnum);
        for (int i = 0; i < phnum; i++) {
            long ph = (long)phoff + (long)i * phentsize;
            if (ph + 56 > size) break;
            uint32_t p_type, p_flags; uint64_t p_offset, p_filesz;
            memcpy(&p_type, buf + ph, 4);
            memcpy(&p_flags, buf + ph + 4, 4);
            memcpy(&p_offset, buf + ph + 8, 8);
            memcpy(&p_filesz, buf + ph + 32, 8);
            if (p_type == PT_LOAD && (p_flags & PF_X)) {
                run_arm_words(machine == EM_AARCH64 ? "arm64" : "arm32",
                              buf + p_offset, (long)p_filesz, "可执行段");
                found = 1;
            }
        }
    } else if (cls == 1) {
        machine = (uint16_t)(buf[18] | (buf[19] << 8));
        uint32_t phoff; uint16_t phentsize, phnum;
        memcpy(&phoff, buf + 28, 4);
        memcpy(&phentsize, buf + 42, 2);
        memcpy(&phnum, buf + 44, 2);
        printf("== ELF32 machine=%u phnum=%u ==\n", machine, phnum);
        for (int i = 0; i < phnum; i++) {
            long ph = (long)phoff + (long)i * phentsize;
            if (ph + 32 > size) break;
            uint32_t p_type, p_offset, p_filesz, p_flags;
            memcpy(&p_type, buf + ph, 4);
            memcpy(&p_offset, buf + ph + 4, 4);
            memcpy(&p_filesz, buf + ph + 16, 4);
            memcpy(&p_flags, buf + ph + 24, 4);
            if (p_type == PT_LOAD && (p_flags & PF_X)) {
                run_arm_words(machine == EM_AARCH64 ? "arm64" : "arm32",
                              buf + p_offset, (long)p_filesz, "可执行段");
                found = 1;
            }
        }
    } else die("未知 ELF 类");
    if (!found) die("ELF 无可执行段");
    free(buf);
}

/* ── 汇编文本模式: 符号直呼 ── */

static void run_asm(const char *path) {
    FILE *f = fopen(path, "r");
    if (!f) die("打不开汇编文件");
    printf("== 域: 汇编文本 %s ==\n", path);
    char line[MAXLINE]; long ln = 0;
    while (fgets(line, sizeof line, f)) {
        ln++;
        line[strcspn(line, "\r\n")] = 0;
        char *s = line;
        while (*s == ' ' || *s == '\t') s++;
        if (*s == 0 || *s == '#') continue;
        char sym[16]; int si = 0;
        int is_call = 0;
        if (*s == '@') { is_call = 1; s++; }
        while (*s && *s != ' ' && *s != '\t' && si < 15) sym[si++] = *s++;
        sym[si] = 0;
        if (is_call) {
            int k = sym_index(sym);
            if (last_serial_of_sym[k] < 0) {
                fprintf(stderr, "tabtrans: 第 %ld 行呼唤未编址符号 %s\n", ln, sym);
                exit(1);
            }
            printf("@呼唤 %s -> #%04ld\n", sym, last_serial_of_sym[k]);
            continue;
        }
        Row *r = NULL;
        for (int i = 0; i < nrows; i++)
            if (strcmp(rows[i].sym, sym) == 0) { r = &rows[i]; break; }
        if (!r) {
            fprintf(stderr, "tabtrans: 第 %ld 行符号 %s 未授权\n", ln, sym);
            exit(1);
        }
        emit_container(sym, r->ctype, "asm", ln, (const unsigned char *)"", 0);
    }
    fclose(f);
}

/* ── 清点输出 ── */

static void print_stats(void) {
    printf("== 符号清点 ==\n");
    for (int i = 0; i < sym_n; i++)
        printf("%s: %ld\n", sym_names[i], sym_counts[i]);
    printf("串联总数: %ld\n", serial);
    printf("未译: %ld\n", untranslated);
    printf("== 乘法口诀清点格 (行i列j, 积=加法累加, 仅清点) ==\n");
    for (int i = 1; i <= 9; i++) {
        long acc = 0;
        printf("行%d:", i);
        for (int j = 1; j <= 9; j++) {
            acc += i;
            printf(" %2ld=%2ld/%ld", (long)j, acc, grid_count[i - 1][j - 1]);
        }
        printf("\n");
    }
    printf("口诀轮次: %ld\n", grid_round);
    printf("折叠层号: %ld (满%d折叠)\n", layer, FOLD_POINT);
}

int main(int argc, char **argv) {
    if (argc < 3) {
        fprintf(stderr, "用法: %s <wasm|elf|asm|verify> <文件或外域> [--table 路径]\n", argv[0]);
        return 2;
    }
    for (int i = 1; i < argc; i++)
        if (strcmp(argv[i], "--table") == 0 && i + 1 < argc) g_table = argv[++i];
    load_table();
    if (strcmp(argv[1], "wasm") == 0) run_wasm(argv[2]);
    else if (strcmp(argv[1], "elf") == 0) run_elf(argv[2]);
    else if (strcmp(argv[1], "asm") == 0) run_asm(argv[2]);
    else if (strcmp(argv[1], "verify") == 0) run_verify(argv[2]);
    else die("未知模式");
    print_stats();
    return 0;
}
