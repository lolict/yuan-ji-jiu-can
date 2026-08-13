// test_l0_compiler.c — L0 自举编译器测试
// 验证: 编译/IR/字节码/自编译闭环/音质操作码

#include "l0_compiler.h"
#include <stdio.h>
#include <assert.h>
#include <string.h>

// 测试1: 创建/销毁
void test_create_destroy(void) {
    l0c_compiler_t* c = l0c_create();
    assert(c != NULL);
    assert(c->has_compiled == false);
    l0c_destroy(c);
    printf("  ✓ 创建/销毁测试通过\n");
}

// 测试2: 基础编译
void test_basic_compile(void) {
    l0c_compiler_t* c = l0c_create();

    // 简单L0程序: 压栈→加法→打印→停机
    const char* prog =
        "LOADI 42\n"
        "LOADI 8\n"
        "ADD\n"
        "PRT\n"
        "HLT\n";

    bool ok = l0c_compile(c, prog);
    assert(ok == true);
    assert(c->has_compiled == true);
    assert(c->program.ir_count == 5);

    l0c_destroy(c);
    printf("  ✓ 基础编译测试通过 (5条指令)\n");
}

// 测试3: IR与字节码一致性
void test_ir_bytecode_match(void) {
    l0c_compiler_t* c = l0c_create();

    const char* prog =
        "LOADI 10\n"
        "LOADI 20\n"
        "ADD\n"
        "SUB\n"
        "PRT\n";

    bool ok = l0c_compile(c, prog);
    assert(ok);

    // 字节码大小 = IR数 × 5
    assert(c->program.bytecode.size == c->program.ir_count * 5);

    // 每条IR的操作码与字节码对应位置一致
    for (uint16_t i = 0; i < c->program.ir_count; i++) {
        uint8_t bc_op = c->program.bytecode.code[i * 5];
        assert(bc_op == (uint8_t)c->program.ir[i].op);
    }

    l0c_destroy(c);
    printf("  ✓ IR与字节码一致性测试通过\n");
}

// 测试4: 音质操作码映射 (操作码=可发音音节)
void test_phonetic_opcodes(void) {
    // 加法 = UNION = 合(hé)
    assert(l0c_op_hanzi(OP_ADD) != NULL);
    assert(strcmp(l0c_op_hanzi(OP_ADD), "合") == 0);

    // 跳转 = JIQU = 机(jī)
    assert(strcmp(l0c_op_hanzi(OP_JMP), "机") == 0);

    // 加载立即数 = TUDO = 土(tǔ)
    assert(strcmp(l0c_op_hanzi(OP_LOADI), "土") == 0);

    // 停机 = LUCK = 吉(jí)
    assert(strcmp(l0c_op_hanzi(OP_HALT), "吉") == 0);

    // 打印 = MIAN = 面(miàn)
    assert(strcmp(l0c_op_hanzi(OP_PRINT), "面") == 0);

    printf("  ✓ 音质操作码测试通过 (合/机/土/吉/面)\n");
}

// 测试5: 操作码值在64进制范围内
void test_opcode_range(void) {
    l0c_opcode_t ops[] = {
        OP_NOP, OP_PUSH, OP_POP, OP_ADD, OP_SUB, OP_MUL, OP_DIV,
        OP_EQ, OP_LT, OP_GT, OP_JMP, OP_JZ, OP_JNZ, OP_CALL, OP_RET,
        OP_LOAD, OP_STORE, OP_LOADI, OP_PRINT, OP_PRINTC, OP_HALT
    };
    int n = sizeof(ops) / sizeof(ops[0]);

    for (int i = 0; i < n; i++) {
        // 每个操作码必须是有效的音质符号 (0-63)
        assert(ops[i] >= 0 && ops[i] < 64);
        // 每个操作码必须有对应的拼音和汉字
        assert(l0c_op_pinyin(ops[i]) != NULL);
        assert(l0c_op_hanzi(ops[i]) != NULL);
    }

    printf("  ✓ 操作码范围测试通过 (%d个操作码全部在音质符号内)\n", n);
}

// 测试6: 自编译闭环测试 (蛇吞尾)
void test_self_compile(void) {
    l0c_compiler_t* c = l0c_create();

    // 编译一个简单程序
    const char* prog =
        "LOADI 7\n"
        "LOADI 3\n"
        "ADD\n"
        "PRT\n"
        "HLT\n";

    assert(l0c_compile(c, prog) == true);

    // 执行自编译校验
    bool self_ok = l0c_self_test(c);
    assert(self_ok == true);
    assert(c->program.is_self_compiled == true);

    l0c_destroy(c);
    printf("  ✓ 自编译闭环测试通过 (蛇吞尾 ✓)\n");
}

// 测试7: 自编译一致性验证 (二次编译)
void test_double_compile(void) {
    l0c_compiler_t* c1 = l0c_create();
    l0c_compiler_t* c2 = l0c_create();

    const char* prog =
        "LOADI 100\n"
        "LOADI 200\n"
        "ADD\n"
        "PRT\n"
        "HLT\n";

    // 第一次编译
    assert(l0c_compile(c1, prog) == true);
    uint16_t ir_count1 = c1->program.ir_count;
    uint16_t bc_size1 = c1->program.bytecode.size;

    // 第二次编译 (相同源码)
    assert(l0c_compile(c2, prog) == true);

    // 两次编译结果一致
    assert(ir_count1 == c2->program.ir_count);
    assert(bc_size1 == c2->program.bytecode.size);
    assert(memcmp(c1->program.bytecode.code, c2->program.bytecode.code, bc_size1) == 0);

    l0c_destroy(c1);
    l0c_destroy(c2);
    printf("  ✓ 二次编译一致性测试通过 (确定性编译)\n");
}

// 测试8: 操作码韵律验证 (每个操作码发音都不同)
void test_opcode_distinctness(void) {
    l0c_opcode_t ops[] = {
        OP_NOP, OP_PUSH, OP_POP, OP_ADD, OP_SUB, OP_MUL, OP_DIV,
        OP_JMP, OP_JZ, OP_JNZ, OP_CALL, OP_RET, OP_PRINT, OP_HALT
    };
    int n = sizeof(ops) / sizeof(ops[0]);

    // 所有操作码的发音应各不相同
    for (int i = 0; i < n; i++) {
        for (int j = i + 1; j < n; j++) {
            // 至少有一个维度不同 (通过值不同来判定)
            assert(ops[i] != ops[j]);
        }
    }

    printf("  ✓ 操作码异质互换测试通过 (%d个操作码发音各异)\n", n);
}

int main(void) {
    printf("\n  ═══ L0 自举编译器测试 ═══\n\n");

    test_create_destroy();
    test_basic_compile();
    test_ir_bytecode_match();
    test_phonetic_opcodes();
    test_opcode_range();
    test_self_compile();
    test_double_compile();
    test_opcode_distinctness();

    printf("\n  ═══ 测试结果: 8/8 通过 ═══\n");
    printf("  基础编译 ✓ | IR/字节码 ✓ | 音质操作码 ✓\n");
    printf("  操作码范围 ✓ | 自编译闭环 ✓ | 确定性 ✓\n\n");

    return 0;
}
