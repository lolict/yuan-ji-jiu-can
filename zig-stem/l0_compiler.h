// l0_compiler.h — L0 自举编译器 (Self-Compiling Compiler)
// hunyuan-os 的自举核心 — 用自己的音质进制编写, 编译自身
//
// 哲学:
//   预加载编程: 不是按系统规则索求内容, 而是符号自身的声音就是规则
//   自举(蛇吞尾): L0 源码用 L0 编译器编译 → 产出能编译 L0 的字节码
//   进制=语言: 操作码 = 声母+韵母+声调 = 可发音的音节
//   自给自足: 不依赖外部工具链, 系统自己长出编译器
//
// L0 极简语言:
//   操作码映射到64音质符号的操作子集
//   变量: 单字母 a-z → 内存槽位
//   指令: 赋值/跳转/运算/打印/返回
//   自编译: L0 源码描述自身 → 编译产出 → 再编译 → 验证一致性

#ifndef HUNYUAN_L0_COMPILER_H
#define HUNYUAN_L0_COMPILER_H

#include "smrrll_voice.h"
#include <stdint.h>
#include <stdbool.h>

#define L0C_VERSION         0x40    // 版本=64
#define L0C_MAX_CODE_SIZE   1024    // 最大源码长度
#define L0C_MAX_IR_SIZE     512     // 最大中间表示
#define L0C_MAX_BYTECODE    256     // 最大字节码
#define L0C_MAX_VARS        26      // 变量数 (a-z)
#define L0C_MAX_LABELS      32      // 标签数

// ── 操作码类型 (用音质符号的子集定义) ──
typedef enum {
    // 基础运算 (夫妻层起家)
    OP_NOP      = SMV_YMBA,     // 啊(ā) = 空操作
    OP_PUSH     = SMV_KNDO,     // 工(gōng) = 压栈
    OP_POP      = SMV_NVFM,     // 空(kōng) = 出栈

    // 算术运算
    OP_ADD      = SMV_UNION,    // 合(hé) = 加法
    OP_SUB      = SMV_QI,       // 七(qī) = 减法
    OP_MUL      = SMV_WU,       // 五(wǔ) = 乘法
    OP_DIV      = SMV_LIU,      // 六(liù) = 除法
    OP_MOD      = SMV_BA,       // 八(bā) = 取模

    // 比较运算
    OP_EQ       = SMV_SI,       // 四(sì) = 等于
    OP_LT       = SMV_YI,       // 一(yī) = 小于
    OP_GT       = SMV_ER,       // 二(èr) = 大于

    // 控制流
    OP_JMP      = SMV_JIQU,     // 机(jī) = 跳转
    OP_JZ       = SMV_XUBI,     // 序(xù) = 为零跳
    OP_JNZ      = SMV_FUHR,     // 福(fú) = 非零跳
    OP_CALL     = SMV_GUAX,     // 挂(guà) = 调用
    OP_RET      = SMV_SHOU,     // 收(shōu) = 返回

    // 数据存取
    OP_LOAD     = SMV_LIAM,     // 量(liàng) = 加载变量
    OP_STORE    = SMV_DUEN,     // 端(duān) = 存储变量
    OP_LOADI    = SMV_TUDO,     // 土(tǔ) = 加载立即数

    // IO
    OP_PRINT    = SMV_MIAN,     // 面(miàn) = 打印数值
    OP_PRINTC   = SMV_LING,     // 令(lìng) = 打印字符
    OP_HALT     = SMV_LUCK,     // 吉(jí) = 停机

    OP_INVALID  = SMV_INVALID,
} l0c_opcode_t;

// ── IR中间表示 (三地址码) ──
typedef struct {
    l0c_opcode_t    op;         // 操作码 (=音质符号值)
    int16_t         operand1;   // 操作数1
    int16_t         operand2;   // 操作数2
    int16_t         result;     // 结果
} l0c_ir_t;

// ── 字节码 ──
typedef struct {
    uint8_t     code[L0C_MAX_BYTECODE]; // 字节码数组
    uint16_t    size;                   // 字节码大小
    uint16_t    entry_point;            // 入口点地址
} l0c_bytecode_t;

// ── 编译产物 ──
typedef struct {
    l0c_ir_t        ir[L0C_MAX_IR_SIZE];    // IR
    uint16_t        ir_count;               // IR指令数
    l0c_bytecode_t  bytecode;               // 字节码
    uint64_t        timestamp;              // 编译时间
    bool            is_self_compiled;       // 是否自编译
} l0c_program_t;

// ── 编译器状态 ──
typedef struct {
    char            source[L0C_MAX_CODE_SIZE];  // 源码缓冲
    uint32_t        source_pos;                 // 当前源码位置
    l0c_program_t   program;                    // 编译产物
    bool            has_compiled;               // 是否已编译
} l0c_compiler_t;

// ── 自举编译器 API ──

// 创建编译器实例
l0c_compiler_t* l0c_create(void);
void l0c_destroy(l0c_compiler_t* c);

// 编译L0源码 → 字节码
bool l0c_compile(l0c_compiler_t* c, const char* source);

// 获取IR中间表示
const l0c_ir_t* l0c_get_ir(l0c_compiler_t* c, uint16_t* count);

// 获取字节码
const l0c_bytecode_t* l0c_get_bytecode(l0c_compiler_t* c);

// 获取操作码对应的音质 (=可发音的指令名)
const char* l0c_op_pinyin(l0c_opcode_t op);
const char* l0c_op_hanzi(l0c_opcode_t op);

// 自编译校验: 验证编译产物能正确编译自身
bool l0c_self_test(l0c_compiler_t* c);

// 编译报表
void l0c_report(l0c_compiler_t* c);

#endif // HUNYUAN_L0_COMPILER_H
