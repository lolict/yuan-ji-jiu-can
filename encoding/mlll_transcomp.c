/**
 * mlll_transcomp.c — 转移编译器实现
 *
 * 把32/64位程序 → mlll进制可执行文件
 */

#include "mlll_transcomp.h"
#include <string.h>

/* 识别源格式 */
source_format_t transcomp_detect(const uint8_t *data, uint32_t len) {
    if (len < 4) return SRC_RAW_BINARY;

    /* ELF 魔数 0x7F 'E' 'L' 'F' */
    if (data[0] == 0x7F && data[1] == 0x45 && data[2] == 0x4C && data[3] == 0x46) {
        /* 第5字节: 1=32位, 2=64位 */
        if (len > 5 && data[5] == 2) return SRC_ELF64;
        return SRC_ELF32;
    }

    /* PE 魔数 'M' 'Z' */
    if (data[0] == 0x4D && data[1] == 0x5A) {
        /* 简化: 假设 PE32 */
        return SRC_PE32;
    }

    return SRC_RAW_BINARY;
}

/* 把一条32位指令转译成mlll指令 */
static mlll_instruction_t translate_32bit_instr(uint32_t instr) {
    mlll_instruction_t out;
    memset(&out, 0, sizeof(out));

    /* 简化: 取高4位作为操作码 · 其余作为操作数 */
    uint8_t op = (instr >> 28) & 0x0F;
    switch (op) {
    case 0:  out.opcode = MLLL_OP_NOP; break;
    case 1:  out.opcode = MLLL_OP_LOAD; break;
    case 2:  out.opcode = MLLL_OP_STORE; break;
    case 3:  out.opcode = MLLL_OP_MOVE; break;
    case 4:  out.opcode = MLLL_OP_JUMP; break;
    case 5:  out.opcode = MLLL_OP_CALL; break;
    case 6:  out.opcode = MLLL_OP_RETURN; break;
    case 7:  out.opcode = MLLL_OP_YIELD; break;
    default: out.opcode = MLLL_OP_NOP; break;
    }
    out.operand0 = (instr >> 16) & 0xFF;
    out.operand1 = (instr >> 8) & 0xFF;
    out.operand2 = instr & 0xFF;
    return out;
}

/* 把一条64位指令转译成mlll指令 */
static mlll_instruction_t translate_64bit_instr(uint64_t instr) {
    mlll_instruction_t out;
    memset(&out, 0, sizeof(out));

    uint8_t op = (instr >> 60) & 0x0F;
    switch (op) {
    case 0:  out.opcode = MLLL_OP_NOP; break;
    case 1:  out.opcode = MLLL_OP_LOAD; break;
    case 2:  out.opcode = MLLL_OP_STORE; break;
    case 3:  out.opcode = MLLL_OP_MOVE; break;
    case 4:  out.opcode = MLLL_OP_JUMP; break;
    case 5:  out.opcode = MLLL_OP_CALL; break;
    case 6:  out.opcode = MLLL_OP_RETURN; break;
    case 7:  out.opcode = MLLL_OP_YIELD; break;
    default: out.opcode = MLLL_OP_NOP; break;
    }
    out.operand0 = (instr >> 32) & 0xFF;
    out.operand1 = (instr >> 16) & 0xFF;
    out.operand2 = instr & 0xFF;
    return out;
}

transcomp_result_t transcomp_compile(const uint8_t *data, uint32_t len,
                                     mlil_file_t *output) {
    transcomp_result_t res;
    memset(&res, 0, sizeof(res));
    memset(output, 0, sizeof(*output));

    /* 识别源格式 */
    res.src_format = transcomp_detect(data, len);
    res.src_size = len;

    /* 初始化输出文件头 */
    output->header.magic1 = MLIL_MAGIC_1;
    output->header.magic2 = MLIL_MAGIC_2;
    output->header.version = MLIL_VERSION;

    uint32_t pos = 0;
    uint32_t slot = 0;

    switch (res.src_format) {
    case SRC_ELF64:
    case SRC_PE64: {
        /* 每8字节 = 一条64位指令 */
        while (pos + 8 <= len && slot < 254) {
            uint64_t v = 0;
            for (int i = 0; i < 8; i++) v = (v << 8) | data[pos + i];
            mlll_instruction_t instr = translate_64bit_instr(v);
            /* 存入表面槽 */
            output->surfaces[slot].modulus = instr.opcode;
            output->surfaces[slot].face = instr.operand0;
            output->surfaces[slot].name = instr.operand1;
            output->surfaces[slot].container = instr.operand2;
            slot++;
            pos += 8;
        }
        break;
    }
    case SRC_ELF32:
    case SRC_PE32:
    case SRC_RAW_BINARY:
    default: {
        /* 每4字节 = 一条32位指令 */
        while (pos + 4 <= len && slot < 254) {
            uint32_t v = 0;
            for (int i = 0; i < 4; i++) v = (v << 8) | data[pos + i];
            mlll_instruction_t instr = translate_32bit_instr(v);
            output->surfaces[slot].modulus = instr.opcode;
            output->surfaces[slot].face = instr.operand0;
            output->surfaces[slot].name = instr.operand1;
            output->surfaces[slot].container = instr.operand2;
            slot++;
            pos += 4;
        }
        break;
    }
    }

    output->header.slot_count = slot;
    output->header.data_offset = sizeof(mlil_header_t) + slot * sizeof(mlil_surface_t);
    res.instr_count = slot;
    res.container_used = slot;
    res.error = 0;
    return res;
}
