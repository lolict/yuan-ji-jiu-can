/*
 * chir_wasm.c — M0: CHIR（Cangjie High-level IR）→ wasm32 整数子集后端
 *
 * 输入：cjc --emit-chir=opt + chir-dis 产出的 .chirtxt
 * 输出：.wasm（导出 main）
 *
 * 范围：default 包 _CN7default* 函数；
 *       Allocate / Constant / Add / GT / Store / Apply / Branch / Exit / Debug / TypeCast。
 * 其余 op 显式报错，不静默降级。
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdarg.h>

#define MAX_FUNCS   64
#define MAX_PARAMS  16
#define MAX_BODY    2048
#define MAX_LOCALS  512
#define MAX_BLOCKS  128
#define MAX_LINE    4096

typedef struct {
    uint8_t *data;
    size_t len;
    size_t cap;
} Buf;

static void buf_init(Buf *b) { b->data = NULL; b->len = 0; b->cap = 0; }
static void buf_free(Buf *b) { free(b->data); b->data = NULL; b->len = 0; b->cap = 0; }

static void buf_reserve(Buf *b, size_t extra) {
    if (b->len + extra <= b->cap) return;
    size_t cap = b->cap ? b->cap : 64;
    while (cap < b->len + extra) cap *= 2;
    uint8_t *p = realloc(b->data, cap);
    if (!p) { fprintf(stderr, "chir_wasm: out of memory\n"); exit(1); }
    b->data = p;
    b->cap = cap;
}
static void put_u8(Buf *b, uint8_t v) { buf_reserve(b, 1); b->data[b->len++] = v; }
static void put_bytes(Buf *b, const void *p, size_t n) {
    buf_reserve(b, n);
    memcpy(b->data + b->len, p, n);
    b->len += n;
}
static void put_leb_u32(Buf *b, uint32_t v) {
    do {
        uint8_t byte = v & 0x7f;
        v >>= 7;
        if (v) byte |= 0x80;
        put_u8(b, byte);
    } while (v);
}
static void put_leb_i64(Buf *b, int64_t v) {
    int more = 1;
    while (more) {
        uint8_t byte = (uint8_t)(v & 0x7f);
        v >>= 7;
        if ((v == 0 && !(byte & 0x40)) || (v == -1 && (byte & 0x40))) more = 0;
        else byte |= 0x80;
        put_u8(b, byte);
    }
}

typedef struct {
    char sym[256];
    char ret[32];
    int  param_count;
    char param_types[MAX_PARAMS][32];
    char *body[MAX_BODY];
    int  body_n;
    int  index;
    int  is_main;
    int  ret_value;
    char ret_name[16];
} Func;

typedef struct {
    char name[16];
    int  type;   /* 0 = i64, 1 = i32 */
    int  idx;
} Local;

static Func funcs[MAX_FUNCS];
static int  func_n = 0;

static void die(const char *fmt, ...) {
    va_list ap;
    va_start(ap, fmt);
    fprintf(stderr, "chir_wasm: ");
    vfprintf(stderr, fmt, ap);
    fprintf(stderr, "\n");
    va_end(ap);
    exit(1);
}

static char *trim(char *s) {
    while (*s == ' ' || *s == '\t' || *s == '\r') s++;
    char *e = s + strlen(s);
    while (e > s && (e[-1] == ' ' || e[-1] == '\t' || e[-1] == '\r' || e[-1] == '\n')) *--e = '\0';
    return s;
}

static int type_id(const char *t) {
    if (strstr(t, "Bool")) return 1;
    return 0; /* Int64 / Int64& / UInt64 / etc. 全部按 i64 */
}

static int is_unit_type(const char *t) {
    return strcmp(t, "Unit") == 0 || strcmp(t, "Unit&") == 0 || strcmp(t, "Void") == 0;
}

static int is_value_type(const char *t) {
    return strstr(t, "Int") || strstr(t, "Bool") || strstr(t, "Rune") || strstr(t, "Float");
}

static void parse_chir(char *text) {
    char *save = NULL;
    char *line = strtok_r(text, "\n", &save);
    Func *cur = NULL;
    int in_body = 0;
    int in_params = 0;

    while (line) {
        char *t = trim(line);

        if (!cur) {
            char *f = strstr(t, "Func @_CN7default");
            if (f) {
                if (func_n >= MAX_FUNCS) die("too many functions");
                cur = &funcs[func_n];
                memset(cur, 0, sizeof(*cur));
                cur->index = func_n;
                char *name = f + 6; /* skip "Func @" */
                char *lp = strchr(name, '(');
                if (!lp) die("malformed Func header");
                size_t n = (size_t)(lp - name);
                if (n >= sizeof(cur->sym)) die("symbol too long");
                memcpy(cur->sym, name, n);
                cur->sym[n] = '\0';
                cur->is_main = strstr(cur->sym, "<main>") != NULL;
                in_params = 1;
                in_body = 0;
                /* 无参函数：头部同一行就带 "): Ret" */
                char *same = strstr(lp, "):");
                if (same) {
                    char *rt = same + 2;
                    while (*rt == ' ') rt++;
                    char *sp = strstr(rt, " //");
                    if (sp) *sp = '\0';
                    snprintf(cur->ret, sizeof(cur->ret), "%s", trim(rt));
                    cur->ret_value = !is_unit_type(cur->ret);
                    in_params = 0;
                }
            }
        } else if (in_params) {
            if (strncmp(t, "):", 2) == 0) {
                char *rt = t + 2;
                while (*rt == ' ') rt++;
                char *sp = strstr(rt, " //");
                if (sp) *sp = '\0';
                snprintf(cur->ret, sizeof(cur->ret), "%s", trim(rt));
                cur->ret_value = !is_unit_type(cur->ret);
                in_params = 0;
            } else if (strstr(t, "]: ")) {
                char *ty = strstr(t, "]: ") + 3;
                char *sp = strstr(ty, " //");
                if (sp) *sp = '\0';
                if (cur->param_count >= MAX_PARAMS) die("too many params");
                snprintf(cur->param_types[cur->param_count], sizeof(cur->param_types[0]),
                         "%s", trim(ty));
                cur->param_count++;
            }
        } else if (!in_body) {
            if (strstr(t, "{ Block Group")) in_body = 1;
        } else {
            if (strcmp(t, "}") == 0) {
                func_n++;
                cur = NULL;
                in_body = 0;
            } else if (t[0] != '\0' && strncmp(t, "[initializer]", 13) != 0) {
                if (cur->body_n >= MAX_BODY) die("body too large");
                cur->body[cur->body_n++] = t;
            }
        }
        line = strtok_r(NULL, "\n", &save);
    }
}

static Func *find_func(const char *sym) {
    for (int i = 0; i < func_n; i++)
        if (strcmp(funcs[i].sym, sym) == 0) return &funcs[i];
    return NULL;
}

/* ---------- 块定位 ---------- */

typedef struct {
    int id;
    int start;
    int end;
} Block;

static int collect_blocks(Func *f, Block *blocks, int max) {
    int n = 0;
    for (int i = 0; i < f->body_n; i++) {
        if (strncmp(f->body[i], "Block #", 7) == 0) {
            if (n >= max) die("too many blocks");
            blocks[n].id = atoi(f->body[i] + 7);
            blocks[n].start = i + 1;
            if (n > 0) blocks[n-1].end = i;
            n++;
        }
    }
    if (n > 0) blocks[n-1].end = f->body_n;
    return n;
}

static Block *find_block(Block *blocks, int n, int id) {
    for (int i = 0; i < n; i++)
        if (blocks[i].id == id) return &blocks[i];
    return NULL;
}

/* ---------- 局部变量表 ---------- */

typedef struct {
    Local items[MAX_LOCALS];
    int n;
} Locals;

static int local_find(Locals *ls, const char *name) {
    for (int i = 0; i < ls->n; i++)
        if (strcmp(ls->items[i].name, name) == 0) return ls->items[i].idx;
    return -1;
}

static void local_add(Locals *ls, const char *name, int type) {
    if (local_find(ls, name) >= 0) return;
    if (ls->n >= MAX_LOCALS) die("too many locals");
    snprintf(ls->items[ls->n].name, sizeof(ls->items[0].name), "%s", name);
    ls->items[ls->n].type = type;
    ls->items[ls->n].idx = -1;
    ls->n++;
}

/* ---------- op 解析 ---------- */

typedef struct {
    int  is_ret;
    char dst[16];
    char ty[32];
    char op[32];
    char args[512];
} ParsedOp;

static int parse_assign(const char *line, ParsedOp *p) {
    const char *eq = strstr(line, " = ");
    if (!eq) return 0;
    memset(p, 0, sizeof(*p));
    char lhs[MAX_LINE];
    size_t ln = (size_t)(eq - line);
    if (ln >= sizeof(lhs)) ln = sizeof(lhs) - 1;
    memcpy(lhs, line, ln);
    lhs[ln] = '\0';
    char *l = trim(lhs);
    if (strncmp(l, "[ret]", 5) == 0) { p->is_ret = 1; l = trim(l + 5); }
    char *colon = strstr(l, ": ");
    if (!colon) return 0;
    *colon = '\0';
    char *nm = trim(l);
    char *br = strchr(nm, '[');   /* %3[s] → %3 */
    if (br) *br = '\0';
    snprintf(p->dst, sizeof(p->dst), "%s", nm);
    snprintf(p->ty, sizeof(p->ty), "%s", trim(colon + 2));

    const char *rhs = eq + 3;
    const char *lp = strchr(rhs, '(');
    if (!lp) return 0;
    size_t on = (size_t)(lp - rhs);
    if (on >= sizeof(p->op)) on = sizeof(p->op) - 1;
    memcpy(p->op, rhs, on);
    p->op[on] = '\0';
    const char *rp = strrchr(rhs, ')');
    if (rp && rp > lp) {
        size_t an = (size_t)(rp - (lp + 1));
        if (an >= sizeof(p->args)) an = sizeof(p->args) - 1;
        memcpy(p->args, lp + 1, an);
        p->args[an] = '\0';
    }
    return 1;
}

/* 拆分参数（不处理嵌套括号；本子集足够） */
static int split_args(char *s, char **out, int max) {
    int n = 0;
    char *p = s;
    while (n < max) {
        while (*p == ' ') p++;
        if (*p == '\0') break;
        out[n++] = p;
        char *c = strchr(p, ',');
        if (!c) break;
        *c = '\0';
        p = c + 1;
    }
    for (int i = 0; i < n; i++) out[i] = trim(out[i]);
    return n;
}

/* ---------- wasm 发射 ---------- */

static void emit_local_get(Buf *b, int idx) { if (idx < 0) die("unmapped local ref"); put_u8(b, 0x20); put_leb_u32(b, (uint32_t)idx); }
static void emit_local_set(Buf *b, int idx) { if (idx < 0) die("unmapped local dst"); put_u8(b, 0x21); put_leb_u32(b, (uint32_t)idx); }

static void emit_block(Func *f, Locals *ls, Block *blocks, int bn, int id,
                       int back_edge, Buf *out, int *emitted, int emitted_n);

static int ends_with_goto(Func *f, Block *b, int target);

static void emit_op(Func *f, Locals *ls, Block *blocks, int bn,
                    const char *line, int back_edge, Buf *out,
                    int *emitted, int emitted_n) {
    char tmp[MAX_LINE];
    snprintf(tmp, sizeof(tmp), "%s", line);
    char *t = trim(tmp);

    if (strncmp(t, "Branch(", 7) == 0) {
        char *args = strchr(t, '(') + 1;
        char *rp = strrchr(args, ')');
        if (rp) *rp = '\0';
        char *parts[4];
        int n = split_args(args, parts, 4);
        if (n != 3) die("Branch expects 3 args: %s", line);
        int cond = local_find(ls, parts[0]);
        int then_id = atoi(parts[1] + 1);
        int else_id = atoi(parts[2] + 1);
        emit_local_get(out, cond);
        put_u8(out, 0x04);        /* if */
        put_u8(out, 0x40);        /* blocktype: empty */
        emit_block(f, ls, blocks, bn, then_id, back_edge, out, emitted, emitted_n);
        put_u8(out, 0x05);        /* else */
        emit_block(f, ls, blocks, bn, else_id, back_edge, out, emitted, emitted_n);
        put_u8(out, 0x0b);        /* end */
        return;
    }
    if (strncmp(t, "Exit(", 5) == 0) {
        if (f->ret_value) {
            int ret = local_find(ls, f->ret_name);
            if (ret < 0) die("Exit without [ret] slot in %s", f->sym);
            emit_local_get(out, ret);
        }
        put_u8(out, 0x0f);        /* return */
        return;
    }

    ParsedOp p;
    if (!parse_assign(t, &p)) {
        if (strncmp(t, "Debug(", 6) == 0) return;
        die("unsupported CHIR line in %s: %s", f->sym, line);
    }

    if (strcmp(p.op, "Allocate") == 0) return;
    if (strcmp(p.op, "Debug") == 0) return;

    if (strcmp(p.op, "Constant") == 0) {
        if (is_unit_type(p.ty)) return;
        char *a = trim(p.args);
        if (strcmp(a, "true") == 0) { put_u8(out, 0x41); put_leb_i64(out, 1); }
        else if (strcmp(a, "false") == 0) { put_u8(out, 0x41); put_leb_i64(out, 0); }
        else {
            size_t n = strlen(a);
            if (n && (a[n-1] == 'i' || a[n-1] == 'u')) a[n-1] = '\0';
            put_u8(out, 0x42);    /* i64.const */
            put_leb_i64(out, strtoll(a, NULL, 0));
        }
        int idx = local_find(ls, p.dst);
        if (idx < 0) die("constant dst missing: %s", p.dst);
        emit_local_set(out, idx);
        return;
    }

    if (strcmp(p.op, "Load") == 0) {
        char *parts[4];
        int n = split_args(p.args, parts, 4);
        if (n != 1) die("Load expects 1 arg: %s", line);
        emit_local_get(out, local_find(ls, parts[0]));
        emit_local_set(out, local_find(ls, p.dst));
        return;
    }

    {
        static const struct { const char *name; uint8_t opcode; } binops[] = {
            {"Add", 0x7c}, {"Sub", 0x7d}, {"Mul", 0x7e},
            {"Div", 0x7f}, {"Mod", 0x81},
        };
        for (size_t bi = 0; bi < sizeof(binops)/sizeof(binops[0]); bi++) {
            if (strcmp(p.op, binops[bi].name) != 0) continue;
            char *parts[4];
            int n = split_args(p.args, parts, 4);
            if (n != 2) die("%s expects 2 args: %s", p.op, line);
            emit_local_get(out, local_find(ls, parts[0]));
            emit_local_get(out, local_find(ls, parts[1]));
            put_u8(out, binops[bi].opcode);
            emit_local_set(out, local_find(ls, p.dst));
            return;
        }
    }

    {
        static const struct { const char *name; uint8_t opcode; } cmps[] = {
            {"EQ", 0x51}, {"NE", 0x52}, {"LT", 0x53}, {"GT", 0x55},
            {"LE", 0x57}, {"GE", 0x59},
            {"Equal", 0x51}, {"NotEqual", 0x52},
            {"Less", 0x53}, {"Greater", 0x55},
            {"LessEqual", 0x57}, {"GreaterEqual", 0x59},
        };
        for (size_t ci = 0; ci < sizeof(cmps)/sizeof(cmps[0]); ci++) {
            if (strcmp(p.op, cmps[ci].name) != 0) continue;
            char *parts[4];
            int n = split_args(p.args, parts, 4);
            if (n != 2) die("%s expects 2 args: %s", p.op, line);
            emit_local_get(out, local_find(ls, parts[0]));
            emit_local_get(out, local_find(ls, parts[1]));
            put_u8(out, cmps[ci].opcode);
            emit_local_set(out, local_find(ls, p.dst));
            return;
        }
    }

    if (strcmp(p.op, "Store") == 0) {
        char *parts[4];
        int n = split_args(p.args, parts, 4);
        if (n != 2) die("Store expects 2 args: %s", line);
        int v = local_find(ls, parts[0]);
        int d = local_find(ls, parts[1]);
        if (v < 0 && d < 0) return;   /* Unit 值写入 Unit 槽，无运行时表示 */
        if (v < 0 || d < 0) die("Store operand unmapped in %s: %s", f->sym, line);
        emit_local_get(out, v);
        emit_local_set(out, d);
        return;
    }

    if (strcmp(p.op, "TypeCast") == 0) {
        if (is_unit_type(p.ty)) return;
        char *parts[4];
        int n = split_args(p.args, parts, 4);
        if (n != 1) die("TypeCast expects 1 arg: %s", line);
        emit_local_get(out, local_find(ls, parts[0]));
        emit_local_set(out, local_find(ls, p.dst));
        return;
    }

    if (strcmp(p.op, "Apply") == 0) {
        char *parts[16];
        int n = split_args(p.args, parts, 16);
        if (n < 1) die("Apply without callee: %s", line);
        if (parts[0][0] != '@') die("Apply callee not symbol: %s", line);
        Func *callee = find_func(parts[0][0] == '@' ? parts[0] + 1 : parts[0]);
        if (!callee) die("unknown callee %s in %s", parts[0], f->sym);
        for (int i = 1; i < n; i++)
            emit_local_get(out, local_find(ls, parts[i]));
        put_u8(out, 0x10);        /* call */
        put_leb_u32(out, (uint32_t)callee->index);
        if (!is_unit_type(p.ty)) {
            emit_local_set(out, local_find(ls, p.dst));
        } else if (callee->ret_value) {
            put_u8(out, 0x1a);    /* 有返回值但被丢弃 */
        }
        return;
    }

    die("unsupported op '%s' in %s", p.op, f->sym);
}

static int ends_with_goto(Func *f, Block *b, int target) {
    if (!b) return 0;
    for (int i = b->end - 1; i >= b->start; i--) {
        const char *t = f->body[i];
        if (t[0] == '\0') continue;
        if (strncmp(t, "GoTo(#", 6) != 0) return 0;
        int got = atoi(t + 6);
        return got == target;
    }
    return 0;
}

/* 可归约 while：A=头块，Branch(c,B,C)，B 以 GoTo(A) 收尾。
 * block $exit; loop $head; A; c==0 -> br 1; B; br 0; end; C; end */
static void emit_while(Func *f, Locals *ls, Block *blocks, int bn, int header,
                       int cond_local, int body_id, int exit_id,
                       Buf *out, int *emitted, int emitted_n) {
    Block *h = find_block(blocks, bn, header);
    if (!h) die("missing while header #%d", header);

    put_u8(out, 0x02); put_u8(out, 0x40);   /* block (exit) */
    put_u8(out, 0x03); put_u8(out, 0x40);   /* loop (head) */

    /* 头块：跳过末尾的 Branch */
    for (int i = h->start; i < h->end; i++) {
        const char *t = f->body[i];
        if (strncmp(t, "Branch(", 7) == 0) break;
        emit_op(f, ls, blocks, bn, t, header, out, emitted, emitted_n);
    }
    emit_local_get(out, cond_local);
    put_u8(out, 0x45);                       /* i32.eqz */
    put_u8(out, 0x0d); put_leb_u32(out, 1);  /* br_if exit */

    emit_block(f, ls, blocks, bn, body_id, header, out, emitted, emitted_n + 1);
    put_u8(out, 0x0c); put_leb_u32(out, 0);  /* br loop */
    put_u8(out, 0x0b);                       /* end loop */
    put_u8(out, 0x0b);                       /* end block：br_if 1 落到块后 */

    emit_block(f, ls, blocks, bn, exit_id, 0, out, emitted, emitted_n + 1);
}

/* 自环：Branch(c, exit, head) 或 Branch(c, head, exit) */
static void emit_self_loop(Func *f, Locals *ls, Block *blocks, int bn, int header,
                           int cond_local, int exit_id, int exit_on_true,
                           Buf *out, int *emitted, int emitted_n) {
    Block *h = find_block(blocks, bn, header);
    if (!h) die("missing self-loop header #%d", header);

    put_u8(out, 0x02); put_u8(out, 0x40);   /* block (exit) */
    put_u8(out, 0x03); put_u8(out, 0x40);   /* loop (head) */
    for (int i = h->start; i < h->end; i++) {
        const char *t = f->body[i];
        if (strncmp(t, "Branch(", 7) == 0) break;
        emit_op(f, ls, blocks, bn, t, header, out, emitted, emitted_n);
    }
    emit_local_get(out, cond_local);
    if (exit_on_true) {
        put_u8(out, 0x0d); put_leb_u32(out, 1);  /* cond -> exit */
        put_u8(out, 0x0c); put_leb_u32(out, 0);  /* else continue */
    } else {
        put_u8(out, 0x0d); put_leb_u32(out, 0);  /* cond -> continue */
        put_u8(out, 0x0c); put_leb_u32(out, 1);  /* else exit */
    }
    put_u8(out, 0x0b);                       /* end loop */
    put_u8(out, 0x0b);                       /* end block */
    emit_block(f, ls, blocks, bn, exit_id, 0, out, emitted, emitted_n + 1);
}

static void emit_block(Func *f, Locals *ls, Block *blocks, int bn, int id,
                       int back_edge, Buf *out, int *emitted, int emitted_n) {
    for (int i = 0; i < emitted_n; i++)
        if (emitted[i] == id) return;
    Block *blk = find_block(blocks, bn, id);
    if (!blk) die("missing block #%d in %s", id, f->sym);
    emitted[emitted_n] = id;

    for (int i = blk->start; i < blk->end; i++) {
        const char *t = f->body[i];
        if (strncmp(t, "GoTo(", 5) == 0) {
            int target = atoi(t + 6);
            if (target == back_edge) continue;   /* 回边由 emit_while 发射 */
            emit_block(f, ls, blocks, bn, target, 0, out, emitted, emitted_n + 1);
            return;
        }
        if (strncmp(t, "Branch(", 7) == 0) {
            char tmp[MAX_LINE];
            snprintf(tmp, sizeof(tmp), "%s", t);
            char *args = strchr(tmp, '(') + 1;
            char *rp = strrchr(args, ')');
            if (rp) *rp = '\0';
            char *parts[4];
            int n = split_args(args, parts, 4);
            if (n != 3) die("Branch expects 3 args: %s", t);
            int cond = local_find(ls, parts[0]);
            int then_id = atoi(parts[1] + 1);
            int else_id = atoi(parts[2] + 1);
            Block *thenb = find_block(blocks, bn, then_id);
            Block *elseb = find_block(blocks, bn, else_id);
            if (else_id == id && !ends_with_goto(f, thenb, id)) {
                emit_self_loop(f, ls, blocks, bn, id, cond, then_id, 1,
                               out, emitted, emitted_n + 1);
                return;
            }
            if (then_id == id && !ends_with_goto(f, elseb, id)) {
                emit_self_loop(f, ls, blocks, bn, id, cond, else_id, 0,
                               out, emitted, emitted_n + 1);
                return;
            }
            if (ends_with_goto(f, thenb, id)) {
                emit_while(f, ls, blocks, bn, id, cond, then_id, else_id,
                           out, emitted, emitted_n + 1);
                return;
            }
            emit_op(f, ls, blocks, bn, t, back_edge, out, emitted, emitted_n + 1);
            continue;
        }
        emit_op(f, ls, blocks, bn, t, back_edge, out, emitted, emitted_n + 1);
    }
}

static int block_index(Block *blocks, int n, int chir_id) {
    for (int i = 0; i < n; i++)
        if (blocks[i].id == chir_id) return i;
    die("unknown block #%d", chir_id);
    return -1;
}

/* 通用 CFG：状态机 + br_table，可处理带 join 的 for-in 降级结果 */
static void emit_dispatch(Func *f, Locals *ls, Block *blocks, int bn,
                          int state_idx, Buf *out) {
    put_u8(out, 0x41); put_leb_i64(out, 0);   /* 入口 = 第 0 块 */
    emit_local_set(out, state_idx);

    put_u8(out, 0x03); put_u8(out, 0x40);     /* loop */
    for (int j = 0; j < bn; j++) { put_u8(out, 0x02); put_u8(out, 0x40); }

    emit_local_get(out, state_idx);
    put_u8(out, 0x0e);                        /* br_table */
    put_leb_u32(out, (uint32_t)bn);
    for (int s = 0; s < bn; s++) put_leb_u32(out, (uint32_t)(bn - 1 - s));
    put_leb_u32(out, (uint32_t)(bn - 1));      /* default → 第 0 块 */

    for (int k = bn - 1; k >= 0; k--) {
        put_u8(out, 0x0b);                     /* end block k */
        Block *b = &blocks[k];
        const char *term = NULL;

        for (int i = b->start; i < b->end; i++) {
            const char *t = f->body[i];
            if (strncmp(t, "Exit(", 5) == 0 ||
                strncmp(t, "GoTo(", 5) == 0 ||
                strncmp(t, "Branch(", 7) == 0) { term = t; break; }
            emit_op(f, ls, blocks, bn, t, 0, out, NULL, 0);
        }
        if (!term) die("block #%d in %s has no terminator", b->id, f->sym);

        if (strncmp(term, "Exit(", 5) == 0) {
            if (f->ret_value) {
                int ret = local_find(ls, f->ret_name);
                if (ret < 0) die("Exit without [ret] slot in %s", f->sym);
                emit_local_get(out, ret);
            }
            put_u8(out, 0x0f);
            continue;
        }

        if (strncmp(term, "GoTo(", 5) == 0) {
            int tid = atoi(term + 6);
            put_u8(out, 0x41);
            put_leb_i64(out, block_index(blocks, bn, tid));
            emit_local_set(out, state_idx);
            put_u8(out, 0x0c); put_leb_u32(out, (uint32_t)k);
            continue;
        }

        char tmp[MAX_LINE];
        snprintf(tmp, sizeof(tmp), "%s", term);
        char *args = strchr(tmp, '(') + 1;
        char *rp = strrchr(args, ')');
        if (rp) *rp = '\0';
        char *parts[4];
        int n = split_args(args, parts, 4);
        if (n != 3) die("Branch expects 3 args: %s", term);
        int cond = local_find(ls, parts[0]);
        int tid = block_index(blocks, bn, atoi(parts[1] + 1));
        int eid = block_index(blocks, bn, atoi(parts[2] + 1));

        emit_local_get(out, cond);
        put_u8(out, 0x04); put_u8(out, 0x40);  /* if */
        put_u8(out, 0x41); put_leb_i64(out, tid);
        emit_local_set(out, state_idx);
        put_u8(out, 0x0c); put_leb_u32(out, (uint32_t)(k + 1));
        put_u8(out, 0x05);                     /* else */
        put_u8(out, 0x41); put_leb_i64(out, eid);
        emit_local_set(out, state_idx);
        put_u8(out, 0x0c); put_leb_u32(out, (uint32_t)(k + 1));
        put_u8(out, 0x0b);                     /* end if */
        put_u8(out, 0x00);                     /* unreachable */
    }
    put_u8(out, 0x0b);                         /* end loop */
}

/* 预扫描：为 %N 分配局部索引（params → retslot → i64 → i32） */
static void build_locals(Func *f, Locals *ls) {
    ls->n = 0;
    for (int i = 0; i < f->param_count; i++) {
        char nm[16];
        snprintf(nm, sizeof(nm), "%%%d", i);
        local_add(ls, nm, type_id(f->param_types[i]));
    }

    f->ret_name[0] = '\0';
    for (int i = 0; i < f->body_n; i++) {
        ParsedOp p;
        if (!parse_assign(f->body[i], &p)) continue;
        if (p.is_ret) {
            f->ret_value = !is_unit_type(p.ty);
            if (f->ret_value) {
                snprintf(f->ret_name, sizeof(f->ret_name), "%s", p.dst);
                local_add(ls, p.dst, type_id(p.ty));
            }
            break; /* 只认第一个 [ret] */
        }
    }
    for (int i = 0; i < f->body_n; i++) {
        ParsedOp p;
        if (!parse_assign(f->body[i], &p) || p.is_ret) continue;
        if (!is_value_type(p.ty)) continue;
        local_add(ls, p.dst, type_id(p.ty));
    }

    for (int i = 0; i < ls->n; i++) ls->items[i].idx = -1;
    for (int i = 0; i < f->param_count; i++) ls->items[i].idx = i;

    local_add(ls, "%state", 1);   /* 调度器状态；结构化模式不用 */

    /* 声明顺序：i64 组在前、i32 组在后，索引必须与之一致 */
    int next = f->param_count;
    for (int ty = 0; ty < 2; ty++)
        for (int i = f->param_count; i < ls->n; i++)
            if (ls->items[i].type == ty)
                ls->items[i].idx = next++;
}

static void emit_function_entry(Func *f, Buf *entry) {
    Locals ls;
    build_locals(f, &ls);

    int extra[2] = {0, 0};
    for (int i = f->param_count; i < ls.n; i++) extra[ls.items[i].type]++;

    Buf instr;
    buf_init(&instr);

    Block blocks[MAX_BLOCKS];
    int bn = collect_blocks(f, blocks, MAX_BLOCKS);
    if (bn == 0) die("function %s has no blocks", f->sym);

    if (getenv("CHIR_STRUCTURED")) {
        int emitted[MAX_BLOCKS];
        emit_block(f, &ls, blocks, bn, blocks[0].id, 0, &instr, emitted, 0);
    } else {
        int state_idx = local_find(&ls, "%state");
        if (state_idx < 0) die("missing state local");
        emit_dispatch(f, &ls, blocks, bn, state_idx, &instr);
    }

    /* 所有分支都以 return 结束；补一个不可达默认返回，满足校验器 */
    if (f->ret_value) {
        put_u8(&instr, type_id(f->ret) ? 0x41 : 0x42); /* i32/i64.const 0 */
        put_leb_i64(&instr, 0);
        put_u8(&instr, 0x0f);                          /* return */
    }
    put_u8(&instr, 0x0b); /* function end */

    int groups = (extra[0] ? 1 : 0) + (extra[1] ? 1 : 0);
    put_leb_u32(entry, (uint32_t)groups);
    if (extra[0]) { put_leb_u32(entry, (uint32_t)extra[0]); put_u8(entry, 0x7e); }
    if (extra[1]) { put_leb_u32(entry, (uint32_t)extra[1]); put_u8(entry, 0x7f); }
    put_bytes(entry, instr.data, instr.len);
    buf_free(&instr);
}

static void emit_section(Buf *mod, uint8_t id, Buf *payload) {
    put_u8(mod, id);
    put_leb_u32(mod, (uint32_t)payload->len);
    put_bytes(mod, payload->data, payload->len);
}

int main(int argc, char **argv) {
    if (argc != 3) {
        fprintf(stderr, "usage: %s <input.chirtxt> <output.wasm>\n", argv[0]);
        return 1;
    }

    FILE *fp = fopen(argv[1], "rb");
    if (!fp) { perror("open input"); return 1; }
    fseek(fp, 0, SEEK_END);
    long sz = ftell(fp);
    fseek(fp, 0, SEEK_SET);
    char *text = malloc((size_t)sz + 1);
    if (!text) die("out of memory");
    if (fread(text, 1, (size_t)sz, fp) != (size_t)sz) die("short read");
    text[sz] = '\0';
    fclose(fp);

    parse_chir(text);
    if (func_n == 0) die("no _CN7default function found (is this --emit-chir=opt output?)");

    Buf mod;
    buf_init(&mod);
    put_bytes(&mod, "\0asm", 4);
    uint8_t version[4] = {1, 0, 0, 0};
    put_bytes(&mod, version, 4);

    /* 类型段 */
    Buf types;
    buf_init(&types);
    put_leb_u32(&types, (uint32_t)func_n);
    for (int i = 0; i < func_n; i++) {
        Func *f = &funcs[i];
        put_u8(&types, 0x60);
        put_leb_u32(&types, (uint32_t)f->param_count);
        for (int k = 0; k < f->param_count; k++)
            put_u8(&types, type_id(f->param_types[k]) ? 0x7f : 0x7e);
        int has_ret = f->ret_value;
        put_leb_u32(&types, has_ret ? 1 : 0);
        if (has_ret) put_u8(&types, type_id(f->ret) ? 0x7f : 0x7e);
    }
    emit_section(&mod, 1, &types);
    buf_free(&types);

    /* 函数段 */
    Buf fsec;
    buf_init(&fsec);
    put_leb_u32(&fsec, (uint32_t)func_n);
    for (int i = 0; i < func_n; i++) put_leb_u32(&fsec, (uint32_t)i);
    emit_section(&mod, 3, &fsec);
    buf_free(&fsec);

    /* 导出段：main → "main" */
    int main_idx = -1;
    for (int i = 0; i < func_n; i++)
        if (funcs[i].is_main) { main_idx = i; break; }
    if (main_idx < 0) die("no <main> entry found");

    Buf exp;
    buf_init(&exp);
    put_leb_u32(&exp, (uint32_t)func_n);
    for (int i = 0; i < func_n; i++) {
        const char *ename = funcs[i].is_main ? "main" : funcs[i].sym;
        size_t elen = strlen(ename);
        put_leb_u32(&exp, (uint32_t)elen);
        put_bytes(&exp, ename, elen);
        put_u8(&exp, 0x00);
        put_leb_u32(&exp, (uint32_t)i);
    }
    emit_section(&mod, 7, &exp);
    buf_free(&exp);

    /* 代码段 */
    Buf code;
    buf_init(&code);
    put_leb_u32(&code, (uint32_t)func_n);
    for (int i = 0; i < func_n; i++) {
        Buf entry;
        buf_init(&entry);
        emit_function_entry(&funcs[i], &entry);
        put_leb_u32(&code, (uint32_t)entry.len);
        put_bytes(&code, entry.data, entry.len);
        buf_free(&entry);
    }
    emit_section(&mod, 10, &code);
    buf_free(&code);

    FILE *out = fopen(argv[2], "wb");
    if (!out) { perror("open output"); return 1; }
    if (fwrite(mod.data, 1, mod.len, out) != mod.len) die("short write");
    fclose(out);
    size_t out_len = mod.len;
    buf_free(&mod);

    fprintf(stderr, "chir_wasm: %d function(s), %zu bytes → %s\n",
            func_n, out_len, argv[2]);
    return 0;
}
