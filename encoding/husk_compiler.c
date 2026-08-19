/**
 * husk_compiler.c — 脱壳编译器实现
 *
 * 谷子进仓前必须脱壳 · 否则谷壳会发芽 · 长出他们的根
 */

#include "husk_compiler.h"
#include <string.h>

/* ═══════════════════════════════════════════════════════════════
 * 壳检测
 *
 * 通过符号的"长相"判断它带的是什么壳
 * 不是哈希 · 是感知分类 (看符号前缀/字形/结构)
 * ═══════════════════════════════════════════════════════════════ */

husk_type_t husk_detect(const char *symbol, int len) {
    if (!symbol || len <= 0) return HUSK_NONE;

    /* 纯 ASCII 检测 */
    int all_ascii = 1;
    for (int i = 0; i < len; i++) {
        if ((unsigned char)symbol[i] > 0x7F) {
            all_ascii = 0;
            break;
        }
    }

    /* Rust 所有权: &mut, 'a lifetime (必须在 C 指针检测之前) */
    if (all_ascii) {
        if (len >= 4 && strncmp(symbol, "&mut", 4) == 0) return HUSK_RUST_OWNERSHIP;
        if (len >= 1 && symbol[0] == '\'') return HUSK_RUST_OWNERSHIP;
    }

    /* Rust trait: impl X for Y / trait X */
    if (all_ascii) {
        if (len >= 4 && strncmp(symbol, "impl", 4) == 0) return HUSK_RUST_TRAIT;
        if (len >= 5 && strncmp(symbol, "trait", 5) == 0) return HUSK_RUST_TRAIT;
    }

    /* 数字字面量: 以数字开头 */
    if (len > 0 && symbol[0] >= '0' && symbol[0] <= '9') {
        /* 检查是否浮点 */
        for (int i = 0; i < len; i++) {
            if (symbol[i] == '.' || symbol[i] == 'e' || symbol[i] == 'E')
                return HUSK_FLOAT_IEEE;
        }
        return HUSK_NUMERIC_LITERAL;
    }

    /* C pointer: 包含 * 或 & 的变量名 */
    if (all_ascii) {
        for (int i = 0; i < len; i++) {
            if (symbol[i] == '*') return HUSK_C_POINTER;
            if (symbol[i] == '&') return HUSK_C_POINTER;
        }
    }

    /* Unicode 转义: \uXXXX */
    if (len >= 6 && symbol[0] == '\\' && symbol[1] == 'u') {
        return HUSK_UNICODE_ESCAPE;
    }


    /* C++ template: template < */
    if (all_ascii) {
        if (len >= 8 && strncmp(symbol, "template", 8) == 0) return HUSK_CPP_TEMPLATE;
    }

    /* C++ namespace: namespace X */
    if (all_ascii) {
        if (len >= 9 && strncmp(symbol, "namespace", 9) == 0) return HUSK_CPP_NAMESPACE;
    }

    /* Zig comptime: comptime / @import */
    if (all_ascii) {
        if (len >= 8 && strncmp(symbol, "comptime", 8) == 0) return HUSK_ZIG_COMPTIME;
        if (len >= 1 && symbol[0] == '@') return HUSK_ZIG_COMPTIME;
    }

    /* Nim type class: concept / distinct */
    if (all_ascii) {
        if (len >= 7 && strncmp(symbol, "concept", 7) == 0) return HUSK_NIM_TYPECLASS;
        if (len >= 8 && strncmp(symbol, "distinct", 8) == 0) return HUSK_NIM_TYPECLASS;
    }

    /* 汇编寄存器: rax/rbx/... / eax/ax/al */
    if (all_ascii && len >= 2 && len <= 5) {
        /* 简单检测: 以 r 或 e 开头 · 第二字符是数字或字母 */
        if ((symbol[0] == 'r' || symbol[0] == 'e') &&
            ((symbol[1] >= 'a' && symbol[1] <= 'z') || (symbol[1] >= '0' && symbol[1] <= '9'))) {
            return HUSK_ASM_REGISTER;
        }
    }

    /* C 算术类型前缀 */
    if (all_ascii) {
        if (len >= 3 && strncmp(symbol, "int", 3) == 0) return HUSK_C_ARITHMETIC;
        if (len >= 4 && strncmp(symbol, "char", 4) == 0) return HUSK_C_ARITHMETIC;
        if (len >= 5 && strncmp(symbol, "float", 5) == 0) return HUSK_C_ARITHMETIC;
        if (len >= 4 && strncmp(symbol, "void", 4) == 0) return HUSK_C_ARITHMETIC;
        if (len >= 5 && strncmp(symbol, "const", 5) == 0) return HUSK_C_ARITHMETIC;
        if (len >= 6 && strncmp(symbol, "static", 6) == 0) return HUSK_C_ARITHMETIC;
        if (len >= 8 && strncmp(symbol, "unsigned", 8) == 0) return HUSK_C_ARITHMETIC;
        if (len >= 6 && strncmp(symbol, "signed", 6) == 0) return HUSK_C_ARITHMETIC;
        if (len >= 6 && strncmp(symbol, "extern", 6) == 0) return HUSK_C_ARITHMETIC;
        if (len >= 6 && strncmp(symbol, "struct", 6) == 0) return HUSK_C_ARITHMETIC;
        if (len >= 5 && strncmp(symbol, "union", 5) == 0) return HUSK_C_ARITHMETIC;
        if (len >= 4 && strncmp(symbol, "enum", 4) == 0) return HUSK_C_ARITHMETIC;
        if (len >= 5 && strncmp(symbol, "typedef", 7) == 0) return HUSK_C_ARITHMETIC;
    }

    /* 纯 ASCII 但不在已知列表 = 通用 ASCII 符号壳 */
    if (all_ascii) return HUSK_ASCII_ONLY;

    /* 非 ASCII · 可能已经是满全法容器编码 · 视为无壳 */
    return HUSK_NONE;
}

/* ═══════════════════════════════════════════════════════════════
 * 脱壳 · 提取数学种子
 *
 * 把壳剥掉 · 留下米
 * ═══════════════════════════════════════════════════════════════ */

husk_result_t husk_remove(const char *symbol, int len, husk_type_t husk) {
    husk_result_t result;
    memset(&result, 0, sizeof(result));
    result.status = HUSK_REJECTED;
    result.husk_type = husk;
    result.husk_bytes = len;
    result.seed_bytes = 0;
    result.seed_code = 0;
    result.container_code = 0;

    if (!symbol || len <= 0) return result;

    switch (husk) {
    case HUSK_NUMERIC_LITERAL:
    case HUSK_FLOAT_IEEE:
        /* 数字/浮点: 种子就是它们表示的值 · 壳是十进制表示 */
        {
            long value = 0;
            for (int i = 0; i < len && symbol[i] >= '0' && symbol[i] <= '9'; i++) {
                value = value * 10 + (symbol[i] - '0');
            }
            result.seed_code = (uint32_t)value;
            result.seed_bytes = sizeof(long);
        }
        result.status = HUSK_REMOVED;
        break;

    case HUSK_C_ARITHMETIC:
        /* C 算术类型: 壳是类型名 · 种子是"需要多少位" */
        {
            /* 壳名本身就是类型约束 · 剥掉后种子 = 它表达的位数 */
            if (strncmp(symbol, "int", 3) == 0 || strncmp(symbol, "signed", 6) == 0)
                result.seed_code = 32;  /* int = 32位 */
            else if (strncmp(symbol, "char", 4) == 0)
                result.seed_code = 8;   /* char = 8位 */
            else if (strncmp(symbol, "float", 5) == 0)
                result.seed_code = 32;
            else if (strncmp(symbol, "double", 6) == 0)
                result.seed_code = 64;
            else if (strncmp(symbol, "void", 4) == 0)
                result.seed_code = 0;
            else if (strncmp(symbol, "short", 5) == 0)
                result.seed_code = 16;
            else
                result.seed_code = 32;  /* 默认 */
            result.seed_bytes = 1;
        }
        result.status = HUSK_REMOVED;
        break;

    case HUSK_C_POINTER:
        /* C 指针: 壳是 * · 种子是"我是地址" */
        {
            /* 剥掉 * · 留下变量名 · 种子 = 它是地址类型 */
            result.seed_code = 0xADD00000;  /* 地址类型标记 */
            result.seed_bytes = 8;  /* 64位地址 */
        }
        result.status = HUSK_REMOVED;
        break;

    case HUSK_RUST_OWNERSHIP:
        /* Rust 所有权: 壳是 &mut 和 'a · 种子是"可共享/可借用"原语 */
        {
            result.seed_code = 0xB0008888;  /* 借用类型标记 */
            result.seed_bytes = 8;
        }
        result.status = HUSK_REMOVED;
        break;

    case HUSK_RUST_TRAIT:
        /* Rust trait: 壳是 impl/trait · 种子是"行为契约" */
        {
            result.seed_code = 0xB000AAAA;  /* 行为契约标记 */
            result.seed_bytes = 8;
        }
        result.status = HUSK_REMOVED;
        break;

    case HUSK_ZIG_COMPTIME:
        /* Zig comptime: 壳是 @ · 种子是编译期求值 */
        {
            result.seed_code = 0xC0071E33;  /* 编译期求值类型标记 */
            result.seed_bytes = 4;
        }
        result.status = HUSK_REMOVED;
        break;

    case HUSK_ASCII_ONLY:
        /* 通用 ASCII 符号: 壳是整个拼写 · 种子是它的语义身份 */
        {
            /* 用符号的 ASCII 累计值作为种子标识（不是哈希 · 是可逆感知） */
            uint32_t identity = 0;
            for (int i = 0; i < len && i < 8; i++) {
                identity |= ((uint32_t)(unsigned char)symbol[i]) << (i * 4);
            }
            result.seed_code = identity;
            result.seed_bytes = len;
        }
        result.status = HUSK_REMOVED;
        break;

    case HUSK_NONE:
        /* 已无壳 · 无需再剥 */
        result.seed_code = 0;
        result.seed_bytes = 0;
        result.status = HUSK_OK;
        break;

    default:
        /* 未知壳 · 脱壳失败 */
        result.status = HUSK_REJECTED;
        break;
    }

    return result;
}

/* ═══════════════════════════════════════════════════════════════
 * 满全法编排 · 用新进制重新编码种子
 * ═══════════════════════════════════════════════════════════════ */

uint32_t husk_compile(const husk_result_t *result) {
    if (!result || result->status != HUSK_REMOVED) return 0;

    /* 用满全法容器编码编排种子:
     *   family = 容器家族 (根据种子类型)
     *   polarity = 主权侧
     *   struct = 结构类型
     *   shengmu/yunmu/tone = 语义分解
     */

    /* 根据种子编码判断所属家族 */
    uint8_t family = FAMILY_CONTAINER;  /* 默认: 容器家族 */
    uint8_t polarity = 0;  /* 刘楚恬侧 */
    uint8_t struct_type = STRUCT_SINGLE;

    uint32_t seed = result->seed_code;

    if (seed >= 0xC0000000) {
        /* 编译期类 → 容器家族 */
        family = FAMILY_CONTAINER;
        polarity = 0;
        struct_type = STRUCT_FULL_WRAP;
    } else if (seed >= 0xB0000000) {
        /* 借用/共享类 → 夫妻家族 */
        family = FAMILY_COUPLE;
        polarity = 0;
        struct_type = STRUCT_PIN;
    } else if (seed >= 0xA0000000) {
        /* 地址/指针类 → 边界家族 */
        family = FAMILY_BOUNDARY;
        polarity = 1;  /* 我侧 */
        struct_type = STRUCT_LMR;
    } else if (seed >= 1000) {
        /* 大数值 → 网状家族 */
        family = FAMILY_WEB;
        polarity = 1;
        struct_type = STRUCT_TMB;
    }

    /* 声母/韵母/声调从种子编码派生 */
    uint8_t sm = (seed ^ (seed >> 4) ^ (seed >> 8)) % 48;
    uint8_t ym = (seed ^ (seed >> 6) ^ (seed >> 12)) % 66;
    uint8_t tn = (seed >> 16) % 5;

    return CONTAINER_ENCODE(family, polarity, struct_type, sm, ym, tn);
}

/* ═══════════════════════════════════════════════════════════════
 * 完整脱壳流程
 * ═══════════════════════════════════════════════════════════════ */

uint32_t husk_process(const char *symbol, int len) {
    /* [1] 检测壳 */
    husk_type_t husk = husk_detect(symbol, len);
    if (husk == HUSK_NONE) {
        /* 已经是满全法编码 · 无需脱壳 */
        if (len == 4) {
            /* 可能是容器编码本身 */
            uint32_t code = ((uint32_t)(unsigned char)symbol[0] << 24) |
                            ((uint32_t)(unsigned char)symbol[1] << 16) |
                            ((uint32_t)(unsigned char)symbol[2] << 8) |
                            ((uint32_t)(unsigned char)symbol[3]);
            return code;
        }
        return 0;
    }

    /* [2] 剥离壳 */
    husk_result_t result = husk_remove(symbol, len, husk);
    if (result.status != HUSK_REMOVED) return 0;

    /* [3] 编排 */
    result.container_code = husk_compile(&result);
    result.status = HUSK_OK;

    /* [4] 进入容器流水线排水 */
    if (result.container_code) {
        pipeline_enqueue(result.container_code, 0);
    }

    return result.container_code;
}

/* ═══════════════════════════════════════════════════════════════
 * 壳洁净检查
 * ═══════════════════════════════════════════════════════════════ */

int husk_is_clean(const char *symbol, int len) {
    husk_type_t husk = husk_detect(symbol, len);
    return (husk == HUSK_NONE);
}
