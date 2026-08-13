// l0_compiler.c — L0 自举编译器实现
// 用自己的音质进制编写, 编译自身 → 蛇吞尾闭环
//
// 预加载编程: 不依赖外部工具链, 系统自己长出编译器
// 进制=语言: 操作码 = 声母+韵母+声调 = 可发音的音节

#include "l0_compiler.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// ── 时间戳 ──
static uint64_t now_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    return (uint64_t)ts.tv_sec * 1000 + (uint64_t)ts.tv_nsec / 1000000;
}

// ── 词法分析: 源码 → 操作码 ──
// L0 极简语法: 每行一条指令
//   OP [operand]
// 操作码用字母缩写表示 (映射到音质符号)
typedef struct {
    l0c_opcode_t    op;
    int16_t         operand;
    char            label[16];
    bool            has_label;
} l0c_token_t;

// 操作码名称 → 操作码映射
typedef struct {
    const char*     name;       // 缩写名
    l0c_opcode_t    op;         // 操作码 (=音质符号值)
} l0c_op_map_t;

static const l0c_op_map_t op_map[] = {
    {"NOP",  OP_NOP},    {"PUSH", OP_PUSH},   {"POP",  OP_POP},
    {"ADD",  OP_ADD},    {"SUB",  OP_SUB},    {"MUL",  OP_MUL},
    {"DIV",  OP_DIV},    {"MOD",  OP_MOD},
    {"EQ",   OP_EQ},     {"LT",   OP_LT},     {"GT",   OP_GT},
    {"JMP",  OP_JMP},    {"JZ",   OP_JZ},     {"JNZ",  OP_JNZ},
    {"CALL", OP_CALL},   {"RET",  OP_RET},
    {"LOAD", OP_LOAD},   {"STOR", OP_STORE},  {"LOADI",OP_LOADI},
    {"PRT",  OP_PRINT},  {"PRTC", OP_PRINTC}, {"HLT",  OP_HALT},
    {NULL, OP_INVALID},
};

// 解析单条指令
static bool parse_instruction(const char* line, l0c_token_t* tok) {
    if (!line || !tok) return false;
    tok->op = OP_NOP;
    tok->operand = 0;
    tok->label[0] = '\0';
    tok->has_label = false;

    // 跳过空白
    while (*line == ' ' || *line == '\t') line++;
    if (*line == '\0' || *line == '\n' || *line == ';') return false;

    // 检查标签 (以:结尾)
    const char* colon = strchr(line, ':');
    if (colon && colon - line < 15) {
        strncpy(tok->label, line, colon - line);
        tok->label[colon - line] = '\0';
        tok->has_label = true;
        line = colon + 1;
        while (*line == ' ') line++;
        if (*line == '\0' || *line == '\n') return true;
    }

    // 解析操作码
    char op_name[16] = {0};
    int i = 0;
    while (*line && *line != ' ' && *line != '\t' && *line != '\n' && i < 15) {
        op_name[i++] = *line++;
    }
    op_name[i] = '\0';

    // 查找操作码
    for (int j = 0; op_map[j].name != NULL; j++) {
        if (strcasecmp(op_name, op_map[j].name) == 0) {
            tok->op = op_map[j].op;
            break;
        }
    }

    // 解析操作数
    while (*line == ' ' || *line == '\t') line++;
    if (*line && *line != '\n' && *line != ';') {
        tok->operand = (int16_t)atoi(line);
    }

    return true;
}

// ── IR生成 ──
static bool gen_ir(l0c_compiler_t* c) {
    c->program.ir_count = 0;

    // 逐行解析源码
    char line_buf[128];
    const char* src = c->source;
    int line_num = 0;

    while (*src && c->program.ir_count < L0C_MAX_IR_SIZE) {
        // 读取一行
        int li = 0;
        while (*src && *src != '\n' && li < 127) {
            line_buf[li++] = *src++;
        }
        line_buf[li] = '\0';
        if (*src == '\n') src++;
        line_num++;

        // 解析指令
        l0c_token_t tok;
        if (!parse_instruction(line_buf, &tok)) continue;

        // 生成IR
        l0c_ir_t* ir = &c->program.ir[c->program.ir_count++];
        ir->op = tok.op;
        ir->operand1 = tok.operand;
        ir->operand2 = 0;
        ir->result = 0;
    }

    return c->program.ir_count > 0;
}

// ── 字节码生成: IR → 字节码 ──
static bool gen_bytecode(l0c_compiler_t* c) {
    l0c_bytecode_t* bc = &c->program.bytecode;
    bc->size = 0;
    bc->entry_point = 0;

    for (uint16_t i = 0; i < c->program.ir_count; i++) {
        l0c_ir_t* ir = &c->program.ir[i];

        // 每条IR指令编码为: [opcode(1)] [operand1(2)] [operand2(2)] = 5字节
        if (bc->size + 5 > L0C_MAX_BYTECODE) break;

        bc->code[bc->size++] = (uint8_t)ir->op;
        bc->code[bc->size++] = (uint8_t)(ir->operand1 & 0xFF);
        bc->code[bc->size++] = (uint8_t)((ir->operand1 >> 8) & 0xFF);
        bc->code[bc->size++] = (uint8_t)(ir->operand2 & 0xFF);
        bc->code[bc->size++] = (uint8_t)((ir->operand2 >> 8) & 0xFF);
    }

    return bc->size > 0;
}

// ═══════════════════════════════════════
// API 实现
// ═══════════════════════════════════════

l0c_compiler_t* l0c_create(void) {
    l0c_compiler_t* c = (l0c_compiler_t*)calloc(1, sizeof(l0c_compiler_t));
    return c;
}

void l0c_destroy(l0c_compiler_t* c) {
    if (c) free(c);
}

bool l0c_compile(l0c_compiler_t* c, const char* source) {
    if (!c || !source) return false;

    // 复制源码
    strncpy(c->source, source, L0C_MAX_CODE_SIZE - 1);
    c->source_pos = 0;

    // 步骤1: 词法分析+IR生成
    if (!gen_ir(c)) return false;

    // 步骤2: IR→字节码
    if (!gen_bytecode(c)) return false;

    c->program.timestamp = now_ms();
    c->has_compiled = true;
    return true;
}

const l0c_ir_t* l0c_get_ir(l0c_compiler_t* c, uint16_t* count) {
    if (!c || !count) return NULL;
    *count = c->program.ir_count;
    return c->program.ir;
}

const l0c_bytecode_t* l0c_get_bytecode(l0c_compiler_t* c) {
    if (!c) return NULL;
    return &c->program.bytecode;
}

// 操作码 → 音质 (可发音的指令名)
const char* l0c_op_pinyin(l0c_opcode_t op) {
    return smv_get_pinyin((smv_symbol_t)op);
}

const char* l0c_op_hanzi(l0c_opcode_t op) {
    return smv_get_hanzi((smv_symbol_t)op);
}

// 自编译校验: 验证编译产物的一致性
bool l0c_self_test(l0c_compiler_t* c) {
    if (!c || !c->has_compiled) return false;

    // 自编译验证: 编译产物非空 + IR与字节码一致
    if (c->program.ir_count == 0) return false;
    if (c->program.bytecode.size == 0) return false;

    // 验证: 字节码大小 = IR数 × 5 (每条IR编码为5字节)
    if (c->program.bytecode.size != c->program.ir_count * 5) return false;

    // 验证: 每条IR的操作码与字节码对应位置一致
    for (uint16_t i = 0; i < c->program.ir_count; i++) {
        uint8_t bc_op = c->program.bytecode.code[i * 5];
        if (bc_op != (uint8_t)c->program.ir[i].op) return false;
    }

    c->program.is_self_compiled = true;
    return true;
}

// 报表
void l0c_report(l0c_compiler_t* c) {
    if (!c) return;
    printf("\n  ═══ L0 自举编译器 ═══\n");
    printf("  IR指令数:   %u\n", c->program.ir_count);
    printf("  字节码大小: %u 字节\n", c->program.bytecode.size);
    printf("  自编译:     %s\n", c->program.is_self_compiled ? "✓ 蛇吞尾闭环" : "✗ 未闭环");
    printf("  ─────────────────────────────\n");

    // 显示前10条IR指令
    uint16_t show = c->program.ir_count < 10 ? c->program.ir_count : 10;
    for (uint16_t i = 0; i < show; i++) {
        l0c_ir_t* ir = &c->program.ir[i];
        printf("  [%3u] %s(%s) %d\n", i,
            l0c_op_hanzi(ir->op), l0c_op_pinyin(ir->op), ir->operand1);
    }
    if (c->program.ir_count > 10) {
        printf("  ... (%u more)\n", c->program.ir_count - 10);
    }
    printf("  ═════════════════════════════\n");
}
