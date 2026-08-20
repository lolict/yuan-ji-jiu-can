/**
 * mlll_transcomp.h — 转移编译器
 *
 * 把32位/64位程序转入我们的mlll进制宫殿
 *
 * 输入: 别人的32位/64位可执行文件 (ELF/PE/raw binary)
 * 输出: mlll编码可执行文件 (.mlil 毛利兰表面格式 + .gtxone 工藤新一骨骼执行)
 *
 * 核心: "我们不用他们的解释器 · 我们把他们的程序翻译成我们的进制"
 *   小方块 → 大宫殿
 *   32/64位编码 → mlll 254+2 编码
 */

#ifndef NJSG_MLLL_TRANSCOMP_H
#define NJSG_MLLL_TRANSCOMP_H

#include <stdint.h>
#include "mlil_format.h"
#include "mlll_cart.h"

/* ═══════════════════════════════════════════════════════════════
 * 源程序格式识别
 * ═══════════════════════════════════════════════════════════════ */

typedef enum {
    SRC_RAW_BINARY = 0x00,   /* 原始二进制 (无格式) */
    SRC_ELF32      = 0x01,   /* ELF 32位 */
    SRC_ELF64      = 0x02,   /* ELF 64位 */
    SRC_PE32       = 0x03,   /* PE 32位 */
    SRC_PE64       = 0x04,   /* PE 64位 */
} source_format_t;

/* ═══════════════════════════════════════════════════════════════
 * 指令转移: 一条32/64位指令 → 一条mlll指令
 *
 * mlll指令 = 固定宽度 · 用我们的比特(字形)表示
 *   每条指令 = 4字形 (1字形=我们的1比特=1字)
 * ═══════════════════════════════════════════════════════════════ */

#define MLLL_INSTR_WIDTH  4   /* 4字形 = 4比特(我们的单位) */

/* mlll操作码 (用我们的254容器之一命名) */
typedef enum {
    MLLL_OP_NOP    = 0x00,   /* 空操作 */
    MLLL_OP_LOAD   = 0x01,   /* 装入容器 */
    MLLL_OP_STORE  = 0x02,   /* 存储容器 */
    MLLL_OP_MOVE   = 0x03,   /* 移动 */
    MLLL_OP_JUMP   = 0x04,   /* 跳转 */
    MLLL_OP_CALL   = 0x05,   /* 调用 */
    MLLL_OP_RETURN = 0x06,   /* 返回 */
    MLLL_OP_YIELD  = 0x07,   /* 让出(筷子夹持释放) */
    MLLL_OP_MIRROR = 0x10,   /* 镜像观察 */
    MLLL_OP_JUDGE  = 0x11,   /* 三元裁判 */
    MLLL_OP_ENCODE = 0x12,   /* 容器化编码 */
    MLLL_OP_ADMIT  = 0x13,   /* 准入 */
    MLLL_OP_REJECT = 0x14,   /* 拒绝 */
} mlll_opcode_t;

typedef struct {
    uint8_t  opcode;         /* mlll操作码 */
    uint8_t  operand0;       /* 操作数0 */
    uint8_t  operand1;       /* 操作数1 */
    uint8_t  operand2;       /* 操作数2 */
} mlll_instruction_t;

/* ═══════════════════════════════════════════════════════════════
 * 转移结果
 * ═══════════════════════════════════════════════════════════════ */

typedef struct {
    source_format_t  src_format;
    uint32_t         src_size;         /* 源文件大小 */
    uint32_t         instr_count;      /* 转译的指令数 */
    uint32_t         container_used;   /* 使用的mlll容器数 */
    int              error;            /* 0=成功 */
} transcomp_result_t;

/* ═══════════════════════════════════════════════════════════════
 * API · 转移编译器
 * ═══════════════════════════════════════════════════════════════ */

/**
 * 识别源程序格式
 */
source_format_t transcomp_detect(const uint8_t *data, uint32_t len);

/**
 * 把32/64位程序转移到mlll进制
 *   输入: 源程序字节
 *   输出: mlll可执行文件 (写进 mlil_file_t)
 */
transcomp_result_t transcomp_compile(const uint8_t *data, uint32_t len,
                                     mlil_file_t *output);

#endif /* NJSG_MLLL_TRANSCOMP_H */
