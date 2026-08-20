/**
 * mlll_vm.h — mlll 解释器
 *
 * 直接执行mlll编码程序 · 不依赖32/64位系统
 *
 * 核心: "用我们的进制运行程序"
 *   不调用系统API · 不依赖外部运行时
 *   所有操作都在mlll容器内完成
 */

#ifndef NJSG_MLLL_VM_H
#define NJSG_MLLL_VM_H

#include <stdint.h>
#include "mlll_transcomp.h"
#include "mlll_cart.h"

/* ═══════════════════════════════════════════════════════════════
 * 虚拟机状态
 * ═══════════════════════════════════════════════════════════════ */

#define MLLL_VM_REGS    16   /* 16个通用寄存器 */
#define MLLL_VM_STACK   64   /* 调用栈深度 */

typedef struct {
    /* 寄存器 = 16个容器槽 */
    uint32_t  reg[MLLL_VM_REGS];

    /* 程序计数器 = 当前执行槽 */
    uint32_t  pc;

    /* 栈 */
    uint32_t  stack[MLLL_VM_STACK];
    uint32_t  sp;          /* 栈指针 */

    /* 状态 */
    uint8_t   running;
    uint8_t   last_admit;  /* 上次裁判结果 */

    /* 装载的程序 */
    mlil_file_t *program;
    mlll_whole_t *box;     /* 字车集装箱 */
} mlll_vm_t;

/* ═══════════════════════════════════════════════════════════════
 * API · mlll 解释器
 * ═══════════════════════════════════════════════════════════════ */

/**
 * 初始化虚拟机
 */
void mlll_vm_init(mlll_vm_t *vm, mlil_file_t *program, mlll_whole_t *box);

/**
 * 执行一条指令
 *   返回: 0=正常 · 1=停止 · -1=错误
 */
int mlll_vm_step(mlll_vm_t *vm);

/**
 * 连续执行直到停止
 */
void mlll_vm_run(mlll_vm_t *vm);

/**
 * 获取寄存器值
 */
uint32_t mlll_vm_getreg(mlll_vm_t *vm, uint8_t r);

#endif /* NJSG_MLLL_VM_H */
