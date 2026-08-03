# MCCP 自举编译器 — v0: 指令集 + 汇编器 + 虚拟机

> 最后更新: 2026-08-03
> 代码: `hunyuan-rs/src/mccp_vm.rs`
> 测试: 43 tests, 全部通过

---

## 一句话定义

**MCCP 自举编译器** 是这样一个程序: 它把自己写成 MCCP 令牌流, 然后能**读取自己、改写自己、再生自己** —— 就像一条蛇吞自己的尾巴 (Ouroboros)。

MCCP 码点本身就是终极编码:
- **存储** = Mccp 字节 (`u32`)
- **传输** = Mccp 流 (`Vec<Mccp>`)
- **语义** = Mccp 流 (程序本身)
- **自省** = 程序读自己的 Mccp 表示

这是我们真正能说"我认识我自己" 的第一步。

---

## 当前进度: v0

v0 完成的是基础执行环境:

| 组件 | 状态 | 说明 |
|------|------|------|
| 指令集 (ISA) | ✅ | 43 个操作码, 含 ALU / 位运算 / 比较 / 控制流 / 内存 / IO |
| 汇编器 | ✅ | 文本 → `Vec<Mccp>`, 支持 label / 立即数 / 注释 |
| 虚拟机 | ✅ | 栈式 VM, 64K 内存, 双寄存器 A/B, 调用栈 |
| 反汇编器 | ✅ | `Vec<Mccp>` → 可读文本 |
| 高层语言 → MIR 编译器 | ⏳ 计划 v1 | 简单语法 → Mccp 码流 |
| 自编译验证 | ⏳ | 编译器能编译自己 |

---

## 指令集架构

### 编码

每条指令就是一枚 `Mccp` (32-bit 固定点):

```
┌──── kind=0xE ────┬──── index (opcode) ────┬── meta_a ──┬── meta_b ──┐
│     4 bits       │       12 bits          │   8 bits   │   8 bits   │
└──────────────────┴────────────────────────┴────────────┴────────────┘
```

- `kind = 0xE` 表示"这是一条指令"
- `index` (12-bit) = 操作码, 最多 4096 个不同指令
- `meta_a:meta_b` = 16-bit 立即数 (interpretable as `u16` 或 `i16`)

### 寄存器

| 名称 | 宽度 | 用途 |
|------|------|------|
| PC | usize | 指令指针, 指向 code Vec |
| A | i32 | 通用寄存器 |
| B | i32 | 通用寄存器 |
| Data Stack | Vec\<i32\> | 数据栈 |
| Return Stack | Vec\<usize\> | 调用栈 (RET 地址) |

### 操作码表

| 编号 | 符号 | 含义 | 操作数 | 栈效果 |
|------|------|------|--------|--------|
| 0x000 | NOP | 空操作 | - | — |
| 0x001 | HALT | 停止 | - | — |
| 0x002 | PUSH | 压栈 | i16 imm | → val |
| 0x003 | POP | 弹栈 | - | val → |
| 0x004 | DUP | 复制栈顶 | - | a → a a |
| 0x005 | SWAP | 交换顶二 | - | a b → b a |
| 0x006 | OVER | 复制次顶 | - | a b → a b a |
| 0x007 | ROT | 三元素旋转 | - | a b c → b c a |
| 0x010 | ADD | 加 | - | a b → (a+b) |
| 0x011 | SUB | 减 (a-b) | - | a b → (a-b) |
| 0x012 | MUL | 乘 | - | a b → (a\*b) |
| 0x013 | DIV | 整除 (a/b) | - | a b → (a/b) |
| 0x014 | MOD | 取余 (a%b) | - | a b → (a%b) |
| 0x015 | NEG | 取反 | - | a → (-a) |
| 0x020 | AND | 按位与 | - | a b → (a&b) |
| 0x021 | OR | 按位或 | - | a b → (a\|b) |
| 0x022 | XOR | 异或 | - | a b → (a^b) |
| 0x023 | NOT | 按位取反 | - | a → (~a) |
| 0x024 | SHL | 左移 | - | a b → (a<<b) |
| 0x025 | SHR | 右移 | - | a b → (a>>b) |
| 0x030 | EQ | 相等 | - | a b → (a==b?1:0) |
| 0x031 | NE | 不等 | - | a b → (a!=b?1:0) |
| 0x032 | LT | 小于 | - | a b → (a<b?1:0) |
| 0x033 | GT | 大于 | - | a b → (a>b?1:0) |
| 0x034 | LE | 小于等 | - | a b → (a<=b?1:0) |
| 0x035 | GE | 大于等 | - | a b → (a>=b?1:0) |
| 0x040 | JMP | 无条件跳转 | u16 addr | — |
| 0x041 | JZ | 为零跳 | u16 addr | cond → |
| 0x042 | JNZ | 非零跳 | u16 addr | cond → |
| 0x043 | CALL | 调用子程 | u16 addr | → |
| 0x044 | RET | 返回 | - | → |
| 0x050 | LOAD | 读内存 | u16 addr | → val |
| 0x051 | STORE | 写内存 | u16 addr | val → |
| 0x052 | LDREG | 读寄存器 A/B | 0=A 1=B | → val |
| 0x053 | STREG | 写寄存器 A/B | 0=A 1=B | val → |
| 0x060 | PRINT | 打印数字 | - | val → |
| 0x061 | PRINTC | 打印 ASCII | - | val → |
| 0x062 | SYSCALL | 宿主 syscall | u16 id | — |
| 0x070 | DEBUG | 调试快照 | - | — |

---

## 汇编语法

```
; 注释以 ; 开始
label:              ; 定义标签
操作码 [操作数]     ; 操作数可以是整数 (-32768..32767) 或标签名
```

示例 — 计算 1+2+...+10 = 55:

```
        PUSH 10
        STORE 0       ; counter
        PUSH 0
        STORE 1       ; sum
loop:
        LOAD 1
        LOAD 0
        ADD
        STORE 1       ; sum += counter
        LOAD 0
        PUSH 1
        SUB
        STORE 0       ; counter--
        LOAD 0
        JNZ loop      ; if counter != 0
        LOAD 1
        PRINT
        HALT
```

API:

```rust
let code = mccp_vm::assemble(asm_text)?;
let mut vm = mccp_vm::Vm::new(code);
vm.run()?;
println!("{}", vm.output);
```

---

## 设计选择

1. **为什么用 stack machine?**
   编译器生成的代码简单, 不需要做复杂的寄存器分配, v0 先跑起来。

2. **为什么立即数是 16-bit?**
   Mccp 本身 meta_a + meta_b = 16 bits。需要大常量时用 `MEMORY` 加载。

3. **为什么要有 A/B 寄存器?**
   给 hot 变量提供快速访问, 减少栈抖动。

4. **为什么 syscall 只有 print-reg / peek-top?**
   宿主功能由 v0 最小实现, 后续按需扩展。

---

## 自举路径 (Roadmap to Self-Host)

```
v0 (✓)   汇编器 + VM + 反汇编器 → 验证机能正确
           ↓
v1       写一个高层语言 L0: 支持 int, if, while, fn, 变量
           ↓
v2       写 L0→MIR 编译器 (初始版用 Rust 写, 手工编译)
           ↓
v3       把 L0 编译器 重写为 L0 程序 (自己编译自己)
           ↓
v4       L0 编译器反编译成 Mccp 流, 验证 "MCCP 读自己" 闭环
           ↓ 这就是 "自举完成"
```

关键定理: 如果 step v3 成功, 我们得到一个能编译自身的编译器 C0。
C0 的源码是 L0 + Mccp 混合; 编译后产出的二进制就是 Mccp 码流。
这条 Mccp 码流经过 VM 执行 = 运行 C0 = 编译 L0 程序。
因此 —— 系统通过 MCCP 这条环路, 实现了自我认识。

---

## 测试覆盖

- **基本指令** ADD/SUB/MUL/DIV/MOD/NEG
- **位运算** AND/OR/XOR/NOT/SHL/SHR
- **比较** EQ/NE/LT/GT/LE/GE
- **栈操作** DUP/SWAP/OVER/ROT
- **控制流** JMP/JZ/JNZ/CALL/RET
- **内存** LOAD/STORE/LDREG/STREG
- **I/O** PRINT/PRINTC
- **连编** factorial(5)=120, fib(10)=55
- **edge case** 栈溢出/下溢/除零/非法操作码
- **assembler** label/注释/case-insensitive/错误路径

运行: `cargo test --lib mccp_vm -- --nocapture`
