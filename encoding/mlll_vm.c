/**
 * mlll_vm.c — mlll 解释器实现
 *
 * 直接执行mlll编码程序 · 不依赖32/64位系统
 */

#include "mlll_vm.h"
#include <string.h>

void mlll_vm_init(mlll_vm_t *vm, mlil_file_t *program, mlll_whole_t *box) {
    memset(vm, 0, sizeof(*vm));
    vm->program = program;
    vm->box = box;
    vm->running = 1;
    vm->sp = 0;
    vm->pc = 0;
}

int mlll_vm_step(mlll_vm_t *vm) {
    if (!vm->running) return 1;
    if (!vm->program) return -1;

    mlil_file_t *f = vm->program;
    if (vm->pc >= f->header.slot_count) {
        vm->running = 0;
        return 1;  /* 程序结束 */
    }

    /* 取当前槽的mlll指令 */
    mlil_surface_t *slot = &f->surfaces[vm->pc];
    uint8_t op = slot->modulus;
    uint8_t a0 = slot->face;
    uint8_t a1 = slot->name;

    switch (op) {
    case MLLL_OP_NOP:
        break;

    case MLLL_OP_LOAD:
        /* 从字车取容器到寄存器 */
        if (a0 < MLLL_VM_REGS && a1 < 254) {
            mlll_cart_entry_t *cart = &vm->box->cart[a1];
            if (cart->occupied) {
                vm->reg[a0] = cart->container;
            }
        }
        break;

    case MLLL_OP_STORE:
        /* 寄存器存入字车 */
        if (a0 < MLLL_VM_REGS && a1 < 254) {
            vm->box->cart[a1].container = vm->reg[a0];
            vm->box->cart[a1].occupied = 1;
            vm->box->cart[a1].slot = a1;
        }
        break;

    case MLLL_OP_MOVE:
        /* 寄存器间移动 */
        if (a0 < MLLL_VM_REGS && a1 < MLLL_VM_REGS) {
            vm->reg[a0] = vm->reg[a1];
        }
        break;

    case MLLL_OP_JUMP:
        /* 跳转到槽 */
        vm->pc = a0;
        return 0;

    case MLLL_OP_CALL:
        /* 调用: 保存返回地址 · 跳转 */
        if (vm->sp < MLLL_VM_STACK) {
            vm->stack[vm->sp++] = vm->pc + 1;
        }
        vm->pc = a0;
        return 0;

    case MLLL_OP_RETURN:
        /* 返回: 恢复返回地址 */
        if (vm->sp > 0) {
            vm->pc = vm->stack[--vm->sp];
        } else {
            vm->running = 0;
            return 1;
        }
        return 0;

    case MLLL_OP_YIELD:
        /* 让出: 暂停执行 */
        vm->running = 0;
        return 1;

    case MLLL_OP_MIRROR:
        /* 镜像观察: 比较两个容器 */
        if (a0 < MLLL_VM_REGS && a1 < MLLL_VM_REGS) {
            vm->last_admit = (vm->reg[a0] == vm->reg[a1]) ? 1 : 0;
        }
        break;

    case MLLL_OP_JUDGE:
        /* 三元裁判: 检查容器是否准入 */
        if (a0 < MLLL_VM_REGS) {
            uint32_t c = vm->reg[a0];
            /* 简化: 非0即准入 */
            vm->last_admit = (c != 0) ? 1 : 0;
        }
        break;

    case MLLL_OP_ENCODE:
        /* 容器化编码: 把寄存器值编码进字车 */
        if (a0 < MLLL_VM_REGS) {
            mlll_load(vm->box, vm->reg[a0], (uint8_t)(a1 ? a1 : CUNIT_64));
        }
        break;

    case MLLL_OP_ADMIT:
        /* 准入: 设置状态 */
        vm->last_admit = 1;
        break;

    case MLLL_OP_REJECT:
        /* 拒绝: 设置状态 */
        vm->last_admit = 0;
        break;

    default:
        /* 未知操作码 · 忽略 */
        break;
    }

    vm->pc++;
    return 0;
}

void mlll_vm_run(mlll_vm_t *vm) {
    while (vm->running) {
        int r = mlll_vm_step(vm);
        if (r != 0) break;
    }
}

uint32_t mlll_vm_getreg(mlll_vm_t *vm, uint8_t r) {
    if (r < MLLL_VM_REGS) return vm->reg[r];
    return 0;
}
