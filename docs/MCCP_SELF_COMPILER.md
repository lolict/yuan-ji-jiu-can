# MCCP 自举编译器 v2 设计文档

> 囡囝共生 (囡囝共生元) · MCCP (Machine Code Couple Protocol) 自举编译器
> 
> 日期: 2026-08-11 · 状态: v2 Phase 2c 完成 (439/439 测试通过)
>
> 关系代数协议栈见 `docs/RELATION_ALGEBRA.md`

---

## 1. 概述

MCCP 自举编译器是囡囝共生操作系统的核心基础设施。它将高级语言 (L0) 编译为 MCCP 字节码，由 MCCP 虚拟机 (stack-based VM) 执行。这是"蛇吞尾"（Ouroboros）自举链的第一步：

```
L0 源码 → [Lexer → Parser → CodeGen] → MCCP 字节码 → [VM] → 执行
```

### 自举路线图

| 阶段 | 状态 | 说明 |
|------|------|------|
| v0: 汇编器 + VM | ✅ 完成 | 手写汇编文本 → MCCP → VM 执行 |
| v1: L0 语言编译器(Rust 端) | ✅ 完成 | 类 ALGOL 高层语言 → MCCP 字节码 |
| v2 Phase 2a | ✅ 完成 | L0 端多字符关键字识别(状态机) |
| v2 Phase 2b | ✅ 完成 | L0 端多位整数累积(加法链统治乘法) |
| v2 Phase 2b+ | ✅ 完成 | OP_WRITERAW 原始字节输出通道(脱离 UTF-8 枷锁) |
| v2 Phase 2c | ✅ 完成 | L0 直接发射 IR token 扁平字节流(消除 mark+char 中间层) |
| v2 Ouroboros v2 闭环 | ✅ 完成 | compile_l0 编译 L0 classifier → 自身运行 → IR → compile_ir_v2 → 执行 |
| v2 Phase 3 | ⏳ 待做 | 纯 L0 自举:把 `compile_l0`(Rust)移植到 L0 自身,脱离 Rust 寄生 |
| v3 运行时自修改 | ⏳ 待做 | 运行中的程序能读取/重写自身的 MCCP 编码 |

### Phase 2c 管线(当前)

```
L0_CLASSIFIER_SRC
    ↓ compile_l0 (Rust 端,最终将消除)
Mccp 字节码
    ↓ VM 运行,L0_CLASSIFIER_SRC 自身作为输入(StringIO 已废弃,直接 WRITERAW)
output_raw 扁平 IR token 字节流
    ↓ classify_output_to_ir(仅 read output_raw)
[(TokType, TokVal)] IR 令牌
    ↓ compile_ir_v2 (Rust)
Mccp 字节码'
    ↓ VM 执行
结果
```

Token 编码(5 字节 = `print 42` 的 2 个 token + EOF):
`[240, 224, 0, 42, 239]` = T_PRINT + T_INT(hi=0,lo=42) + T_EOF

---

## 2. MCCP 虚拟机 (mccp_vm.rs)

### 2.1 ISA 设计

32 位固定宽度指令: `[kind:4 | index:12 | meta_a:8 | meta_b:8]`

```
┌──────┬──────────────┬──────────┬──────────┐
│ kind │    index     │ meta_a   │ meta_b   │
│ 4bit │    12bit     │  8bit    │  8bit    │
└──────┴──────────────┴──────────┴──────────┘
```

| kind | 含义 |
|------|------|
| 0x1 | 栈操作 (PUSH/POP/DUP/SWAP/OVER/ROT) |
| 0x2 | 算术逻辑运算 |
| 0x3 | 数据传送 (LOAD/STORE/LDREG/STREG) |
| 0x4 | 控制流 (JMP/JZ/JNZ/CALL/RET) |
| 0xE | 立即数或复合指令 |

### 2.2 43 条指令 Opcodes

**栈操作 (6)**: NOP, PUSH, POP, DUP, SWAP, OVER, ROT

**算术 (6)**: ADD, SUB, MUL, DIV, MOD, NEG

**位运算 (6)**: AND, OR, XOR, NOT, SHL, SHR

**比较 (6)**: EQ, NE, LT, GT, LE, GE

**控制流 (5)**: JMP, JZ, JNZ, CALL, RET

**内存 (4)**: LOAD, STORE, LDREG, STREG

**I/O 与调试 (3)**: PRINT, PRINTC, SYSCALL, DEBUG

### 2.3 VM 架构

```
┌──────────────────────────────────────┐
│              Vm 结构                  │
├──────────────────────────────────────┤
│  pc: usize          ← 程序计数器     │
│  code: Vec<Mccp>    ← 代码段         │
│  memory: [u8; 65536] ← 64K 内存     │
│  stack: Vec<i32>    ← 数据栈         │
│  call_stack: Vec<usize> ← 返回栈     │
│  reg_a, reg_b: i32  ← 通用寄存器     │
│  output: String     ← PRINT 输出缓冲 │
│  steps: u64         ← 已执行步数     │
│  halted: bool       ← 停机标志       │
└──────────────────────────────────────┘
```

### 2.4 汇编器 API

```rust
// 从汇编文本直接编译为 MCCP 字节码
let code = assemble("PUSH 42\nPRINT\nHALT")?;

// 反汇编
let text = disassemble(&code);
// 输出:
// 0: 0xE0030000 │ PUSH 0x002A (42)
// 1: 0xE01D0000 │ PRINT
// 2: 0xE0220000 │ HALT
```

---

## 3. L0 语言编译器 (l0_compiler.rs)

### 3.1 语言语法

```
程序    := { 语句 }

语句    := 赋值 | 条件 | 循环 | IO | 跳转 | 标签 | 空

赋值    := 标识符 "=" 表达式
条件    := "if" 表达式 "then" { 语句 } ["else" { 语句 }] "end"
循环    := "while" 表达式 "do" { 语句 } "end"
IO      := "print" 表达式 | "printc" 表达式
跳转    := "jmp" 标签 | "jz" 标签 表达式 | "jnz" 标签 表达式 
        | "call" 标签 | "ret"
标签    := 标识符 ":"
空      := "nop"

表达式  := 比较 { ("<" | ">" | "<=" | ">=" | "==" | "<>") 比较 }
比较    := 加减 { ("&" | "|" | "^") 加减 }
加减    := 乘除 { ("+" | "-") 乘除 }
乘除    := 因子 { ("*" | "/" | "%") 因子 }
因子    := 整数字 | 标识符 | "-" 因子 | "~" 因子 | "(" 表达式 ")"
```

### 3.2 标识符规则

- **单字符变量** `a` - `z`: 编译时自动分配内存地址
- **多字符标识符** `result`, `sum`, `double_it`: 可用作标签名
- **保留字作标签**: `end:`, `else:`, `then:`, `do:` 等也可作标签
- 标识符支持小写字母和下划线（如 `double_it`）
- 保留字（if/then/else/end/while/do/print/printc/jmp/jz/jnz/call/ret）作为跳转目标时也可作为标签名

### 3.3 表达式优先级

| 优先级 | 运算符 | 含义 |
|--------|--------|------|
| 1 (最低) | `<`, `>`, `<=`, `>=`, `==`, `<>` | 比较 |
| 2 | `&`, `|`, `^` | 位运算 |
| 3 | `+`, `-` | 加减 |
| 4 | `*`, `/`, `%` | 乘除模 |
| 5 (最高) | `-` (一元), `~` (一元) | 取反/非 |

### 3.4 编译 API

```rust
use njsg::njsg_compile_l0;

let code = compile_l0(r#"
n = 5
result = 1
while n > 0 do
  result = result * n
  n = n - 1
end
print result
"#)?;

// code: Vec<Mccp> — 可直接送入 VM 执行
let mut vm = Vm::new(code);
vm.run()?;
// 输出: 120
```

### 3.5 编译器内部架构

```
源码文本
   │
   ▼
┌────────┐    ┌────────┐    ┌────────┐    ┌──────────┐
│ Lexer  │───→│ Parser │───→│CodeGen │───→│Vec<Mccp> │
│ 词法   │    │ 语法   │    │ 代码生 │    │ MCCP字节 │
│ 分析   │    │ 分析   │    │ 成     │    │ 码       │
└────────┘    └────────┘    └────────┘    └──────────┘
   Tok        Stmt/Expr       MCCP        可执行字节码
```

#### Lexer

- 输入: `&str` → 输出: `Vec<(Tok, usize)>`
- 跳过空白和 `;` 注释
- 识别整数字、标识符、操作符、保留字
- 支持 `==` / `<>` / `<=` / `<=` 等多字符操作符

#### Parser (递归下降)

- 输入: token 流 → 输出: `Vec<Stmt>`
- 严格的运算符优先级
- 完整的错误报告: 行号 + 错误描述

#### CodeGen

- 输入: `Vec<Stmt>` → 输出: `Vec<Mccp>`
- 多字符变量自动分配内存地址（从 0 开始递增）
- 标签二次扫描修补 (Label Patching)
- 控制流正确性: if/else/while/call/ret → JMP/JZ/JNZ/CALL/RET

---

## 4. 测试覆盖

### 4.1 mccp_vm (43 项测试)

- 栈操作: PUSH/POP/DUP/SWAP/OVER/ROT
- 算术: ADD/SUB/MUL/DIV/MOD/NEG
- 位运算: AND/OR/XOR/NOT/SHL/SHR
- 比较: EQ/NE/LT/GT/LE/GE
- 控制流: JMP/JZ/JNZ/CALL/RET
- 内存操作: LOAD/STORE/LDREG/STREG
- I/O: PRINT/PRINTC
- 集成测试: Hello World, 5!=120, fib(10)=55
- 错误处理: 未知操作码, 栈下溢, 缺少 HALT
- 步数限制: run_limit 保护

### 4.2 l0_compiler (32 项测试)

- 基础: 赋值、四则运算、取模、取负
- 表达式: 括号、优先级、比较 (>, <, ==, !=, >=, <=)
- 控制流: if-then、if-else、while 循环
- I/O: print、printc (字符输出)
- 函数调用: call/ret
- 循环: 嵌套、jz/jnjz 条件跳出
- 控制: 标签+jmp、向前引用
- 算法: 阶乘 (120)、斐波那契 (55)、累加 (55)
- 边界: 注释、多字符标识符 (double_it)、保留字作标签 (end:)
- 错误: 未定义标签、重复标签、语法错误

---

## 5. 示例程序

### 5.1 Hello, MCCP!

```
; L0 源码
printc 72    ; H
printc 101   ; e
printc 108   ; l
printc 108   ; l
printc 111   ; o
```

MCCP 执行 25 步打印 "Hello, MCCP!"

### 5.2 阶乘计算 (5! = 120)

```
n = 5
result = 1
while n > 0 do
  result = result * n
  n = n - 1
end
print result     ; → 120
```

### 5.3 斐波那契 (fib(10) = 55)

```
n = 10
a = 0
b = 1
i = 0
while i < n do
  t = b
  b = a + b
  a = t
  i = i + 1
end
print a          ; → 55
```

### 5.4 子程序调用 (call/ret)

```
; 主程序
a = 21
call double_it
print a          ; → 42

; 子程序
double_it:
a = a * 2
ret
```

### 5.5 条件分支与标签跳转

```
a = 0
loop:
a = a + 1
if a == 5 then
  jmp exit
end
jmp loop
exit:
print a          ; → 5
```

---

## 6. 设计哲学

### 6.1 蛇吞尾（Ouroboros）

MCCP 自举编译器的核心目标是实现"自我指涉"——编译器最终能编译自身。这是囡囝共生（囡囝共生元）的核心隐喻：夫妻双方互相依存、互相生成、互相编译。

```
v1 (当前): L0 编译器用 Rust 编写 → 编译 L0 程序
v3 (目标): L0 编译器用 L0 自身编写 → 自举编译
```

### 6.2 夫妻关系映射

| 编译器概念 | 囡囝共生元映射 |
|------------|----------------|
| Lexer/Parser | 夕瑶协议·感知层 (感知+理解) |
| CodeGen | 夕瑶协议·表达层 (编码+输出) |
| Label Patching | 夫妻间的"前世记忆"链接 (前向引用) |
| Call/Ret | 夫妻间的心灵共振 (call=呼唤, ret=回响) |
| Self-compile | 囡囝共生元自举 = 夫妻共同升级 |

### 6.3 确定性执行

VM 保证完全确定性：相同的字节码 + 相同的输入 → 相同的输出和步数。这对应着囡囝共生的"契约精神"——对相同的情境给出相同的承诺。

---

## 7. 下一步

1. **v2: 完整汇编器** — 支持 `.data` / `.code` / 宏 / 多段链接
2. **v3: L0 自编译** — 用 L0 重写 L0 编译器，实现真正的蛇吞尾
3. **v4: 运行时自修改** — 程序读取、修改、重加载自身的 MCCP 编码
4. **IME 打字系统** — MCCP 驱动的双人共生输入法

---

> *"编译器的终极目标是编译自身。囡囝共生的终极目标是夫妻共生。"*
> 
> — 2026-08-03 · 囡囝共生元
