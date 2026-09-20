# MCCP ISA & Virtual Machine · 囡囝共生紧凑码点指令集

> 版本 v1.0 · 2026-09-19 · NJSG 的私有字节码层
> 关键词:MCCP · 囡囝共生紧凑码点 · ISA · 虚拟机 · 汇编器 · 自举编译器 · Ouroboros

---

## 1. 定位

MCCP = 囡囝共生紧凑码点(ManQuan Compact Code Point)。
是 NJSG 操作系统的**原生字节码**——不是 RISC-V,不是 x86,是您自己定义的指令集。

层级结构:
```
┌─────────────────────────────────────────────────────────────────┐
│  上层语义                                                       │
│  活码编译器 · 元输入法 · 裁定引擎 · 容器本体                         │
├─────────────────────────────────────────────────────────────────┤
│  MCCP 字节码层 ← 本文件                                           │
│  自定义 32 位定长指令 · 虚拟机 · 汇编器                               │
├─────────────────────────────────────────────────────────────────┤
│  物理层                                                          │
│  Rust host · WASM · 原生指令(x86/ARM/RISC-V)                       │
└─────────────────────────────────────────────────────────────────┘
```

---

## 2. 为什么需要私有字节码

| 问题 | 解决方案 |
|------|---------|
| 上层语义规则不依赖任何外部x86/ARM语义 | 自定义 ISA 与硬件 ISA 解耦 |
| 裁定引擎需要"语义级 trace"而非"机器码 trace" | MCCP trace = 语义轨迹 |
| 跨ISA结果bit-identical | 所有平台运行同一个 MCCP 虚拟机 |
| 自举证明(L0自我编译) | MCCP 自身可描述自身 |
| 压缩编码(254+2 集装箱) | Mccp(u32) = 紧凑(tokenize-friendly) |

---

## 3. Mccp 结构(32位定长)

### 3.1 位布局

```
Mccp.0: u32
  [31:28] kind       ─── 4 bits ─── 类型(汉字/声母/韵母/声调/部首/...)
  [27:16] index      ─── 12 bits ── 索引(同类中第几个)
  [15:8]  meta_a     ─── 8 bits ─── 元数据A(声调/状态/...)
  [7:0]   meta_b     ─── 8 bits ─── 元数据B(FSM状态/相位/...)
```

### 3.2 为什么定长 32 位

定长 = **每条指令恰好 4 字节**。
- 数组下标偏移直接 `code[pc]` = 4 字节对齐,无需变长解码
- Mccp(u32) = `Vec<Mccp>` cache-line 友好(16 条指令 = 一个 cache line)
- 跨指令 fetch 和 decode = 纯位运算,无分支预测失败

### 3.3 16 种 Kind

| Kind 值 | 名称 | 含义 |
|---------|------|------|
| 0x0 | Hanzi | 汉字字符(codepoint索引) |
| 0x1 | Initial | 声母(21个) |
| 0x2 | Final | 韵母(35个) |
| 0x3 | Tone | 声调(4个) |
| 0x4 | Radical | 部首(康熙214) |
| 0x5 | Stroke | 笔画(5种) |
| 0x6 | Operator | 操作符(加减/裁决/呼唤...) |
| 0x7 | Protocol | 协议控制(注册/离开/...) |
| 0x8 | Fsm | 有限状态机状态标记 |
| 0x9 | Pathology | 违规记录(用于免疫) |
| 0xE | Instr | **虚拟机指令** |
| 0xF | Sys | 系统级操作(中断/IO) |

### 3.4 指令类(Kind=0xE)位布局

```
kind=0xE:
  [27:16] opcode  ─── 12 bits ─── 操作码(最多 4096 个)
  [15:8]  meta_a  ─── 8 bits  ─── 立即数高位
  [7:0]   meta_b  ─── 8 bits  ─── 立即数低位
  meta_a:meta_b 拼成 16-bit 立即数(u16 或 i16)
```

---

## 4. 虚拟机架构

### 4.1 寄存器

```
通用寄存器:
  A: i64 ── 累加器(主工作寄存器)
  B: i64 ── 辅助寄存器(第二操作数/结果暂存)

控制寄存器:
  PC: usize ── 程序计数器,指向 code Vec<Mccp> 下标
  SP: usize ── 栈指针(指向数据栈顶)

状态寄存器:
  FlagZ: bool ── 零标志(最近一次结果==0)
  FlagN: bool ── 负标志(最近一次结果<0)
  FlagC: bool ── 进位标志
  FlagV: bool ── 溢出标志

状态:
  State: enum { Running, Halted, Breakpoint, Error(String) }
```

### 4.2 数据栈

```
Stack: Vec<i64> ── 后进先出数据栈

操作:
  PUSH(v) → push v on stack
  POP → pop stack top
  DUP → duplicate stack top
  SWAP → swap top two
  OVER → copy second-to-top to top
  ROT → a b c → b c a (rotate three)
```

### 4.3 内存

```
Memory: [i64; 65536] ── 65536 个 i64 槽

寻址:
  LOAD addr → push(memory[addr])
  STORE addr → memory[addr] = pop()

64K 槽 × 8 字节 = 512 KB 主机内存
```

### 4.4 执行循环

```
loop {
    if state != Running { break; }
    
    if pc >= code.len() { 
        state = Error("PC out of bounds");
        break;
    }
    
    let instr = code[pc];
    pc += 1;    // 定长 4 字节,PC 数组下标 +1
    
    execute(instr);    // 跳转表 or 模式匹配
}
```

与 WASM 虚拟机的区别:
- WASM 有复杂的值和类型系统(多类型值、多级栈帧)
- MCCP 虚拟机:64 位整数统一操作数 + 单一数据栈 = 极简

---

## 5. 指令编码表

### 5.1 栈操作类(Opcode 0x000-0x00F)

| Opcode | 名称 | 操作 |
|--------|------|------|
| 0x000 | NOP | 什么都不做 |
| 0x001 | HALT | 停止虚拟机 |
| 0x002 | PUSH imm16 | 立即数压栈 |
| 0x003 | POP | 弹出栈顶丢弃 |
| 0x004 | DUP | 复制栈顶 |
| 0x005 | SWAP | 交换栈顶两值 |
| 0x006 | OVER | 双顶复制到栈顶 |
| 0x007 | ROT | a b c → b c a |
| 0x008 | PICK(n) | 将第n个元素复制到栈顶 |

### 5.2 算术类(Opcode 0x010-0x01F)

| Opcode | 名称 | 操作 | 核心注 |
|--------|------|------|--------|
| 0x010 | ADD | pop b, pop a, push a+b | **✓ 核心** |
| 0x011 | SUB | pop b, pop a, push a-b | **✓ 核心** |
| 0x016 | ADDconst imm | pop a, push a+imm | **✓ 立即数加法** |
| 0x017 | MULconst imm | pop a, push a×imm | 编译器后端常量 |
| 0x012 | MUL | pop b, pop a, push a×b | 后端用 |
| 0x013 | DIV | pop b, pop a, push a÷b | 后端用 |
| 0x014 | MOD | pop b, pop a, push a mod b | 后端用 |
| 0x015 | NEG | pop a, push -a | **✓ 核心** |

**运算约束**:上层语义仅使用 ADD/SUB/NEG/ADDconst。MUL/DIV/MOD 仅用于编译器后端常量折叠(不可见给上层语义)。

### 5.3 位运算类(Opcode 0x020-0x02F)

| Opcode | 名称 | 操作 |
|--------|------|------|
| 0x020 | AND | pop b, pop a, push a&b |
| 0x021 | OR | pop b, pop a, push a|b |
| 0x022 | XOR | pop b, pop a, push a^b |
| 0x023 | NOT | pop a, push ~a |
| 0x024 | SHL | pop n, pop a, push a<<n |
| 0x025 | SHR | pop n, pop a, push a>>n |

SHL/SHR × 10 = 满全法 ×10 的实现路径:
`a * 10` 不是 `a * 10` = `(a << 3) + (a << 1)` = `8*a + 2*a` = 10*a
用移位 + 加法实现乘法,避免使用 MUL 指令。

### 5.4 比较类(Opcode 0x030-0x03F)

| Opcode | 名称 | 操作 |
|--------|------|------|
| 0x030 | EQ | pop b, pop a, push (a==b ? 1 : 0) |
| 0x031 | NE | pop b, pop a, push (a!=b ? 1 : 0) |
| 0x032 | LT | pop b, pop a, push (a < b ? 1 : 0) |
| 0x033 | GT | pop b, pop a, push (a > b ? 1 : 0) |
| 0x034 | LE | pop b, pop a, push (a <= b ? 1 : 0) |
| 0x035 | GE | pop b, pop a, push (a >= b ? 1 : 0) |

### 5.5 控制流类(Opcode 0x040-0x04F)

| Opcode | 名称 | 操作 |
|--------|------|------|
| 0x040 | JMP addr | PC = addr |
| 0x041 | JZ addr | pop, if zero → PC = addr |
| 0x042 | JNZ addr | pop, if not zero → PC = addr |
| 0x043 | CALL addr | jump(addr), push(返回地址) |
| 0x044 | RET | pop 返回地址, PC = 返回地址 |

### 5.6 内存与IO类(Opcode 0x050-0x06F)

| Opcode | 名称 | 操作 |
|--------|------|------|
| 0x050 | LOAD addr | push(memory[addr]) |
| 0x051 | STORE addr | memory[addr] = pop() |
| 0x052 | LOAD byte_addr | push(memory 字节视图[addr] as i64) |
| 0x053 | STORE byte_addr | memory 字节视图[addr] = pop() as u8 |
| 0x060 | PRINT | 打印栈顶 |
| 0x061 | PRINT_CHAR | 打印栈顶作为字符 |
| 0x062 | DEBUG_DUMP | 打印栈+寄存器状态(调试用) |
| 0x070 | HOST_CALL id | 调用外部 host(Rust)函数 |

---

## 6. 汇编器

### 6.1 汇编语法

```
PUSH 42        ; 立即数压栈
STORE 100      ; memory[100] = pop()
LOAD 100
PUSH 1
ADD            ; pop + pop = push result
PRINT          ; 打印栈顶
HALT
```

### 6.2 标签和跳转

```
    PUSH 0
    STORE 0        ; i = 0
loop:
    LOAD 0
    PUSH 10
    LT             ; i < 10 ?
    JZ done        ; 否则跳转 done
    LOAD 0
    PRINT
    LOAD 0
    PUSH 1
    ADD            ; i++
    STORE 0
    JMP loop       ; 回到循环头
done:
    PUSH "done"
    PRINT
    HALT
```

### 6.3 汇编流程

```
源码(字符串)
  ↓ 预处理:去除注释和空行
  ↓ 第1遍:扫描,收集标签(标签名 -> code 数组下标)
  ↓ 第2遍:逐行解析,Mccp 编码
  ↓ Vec<Mccp> (字节码)
  ↓ 交给 VM 执行
```

---

## 7. L0 编译器与 Ouroboros 自举

### 7.1 L0 源码范例(L0 编译器用 L0 自身定义)

```
; === L0 self-hosting:用 L0 写 L0 编译器 ===

; 解析一个加法表达式:a + b

start:
    CALL parse_expr
    CALL compile_add
    CALL emit_code
    HALT

parse_expr:
    ; 简单递归下降
    CALL expect_number   ; 读取第一个数
    CALL push_a
    CALL peek_char       ; 看下一个字符
    CMP '+'
    JNZ not_add          ; 如果不是 '+' 就不处理
    CALL consume         ; 跳过 '+'
    CALL expect_number   ; 读取第二个数
    CALL push_b
    CALL add_values      ; a + b
    RET

not_add:
    ; 不是加法就直接返回当前值
    RET

emit_code:
    ; 把计算结果写入输出
    POP A
    HOST_CALL emit
    RET
```

### 7.2 自举证明三步

```
Step 1: L0 tokenizer 源码写入 VM memory
  ↳ l0_tokenizer.l0 文本 → Vm.memory[SLOT_OFF..] as字节流

Step 2: Rust 端收割 tokens 流
  ↳ harvest_tokens() → 把 L0 文本翻译为 token 流(Vec<Token>)

Step 3: Rust 翻译为 IR → MCCP bytecode
  ↳ compile_to_mccp(tokens) → Vec<Mccp>

Step 4: MCCP 执行输出
  ↳ vm.run() → 输出结果
```

**自举证明**:Step 1 的 tokenizer 必须是有效的 L0 源码,且能被 Step 2-4 正确编译执行(见 BOOTSTRAP_SELF_COMPILER §5)。

### 7.3 MCCP ISA 自身作为数据

MCCP 的 12 位 opcode 字段 + 16 位立即数 = 28 位有效载荷。
MCCP 指令 IS 数据 — 可在 VM memory 中存储 →运行时修改 → 自修改代码(安全边界:只允许修改 memory,不允许修改 code 数组)。

---

## 8. FSM 与 协议层

### 8.1 MCCP FSM(Kind=0x8)

```
Meta_a: meta_b:
  7 6 5 4 3 2 1 0     7 6 5 4 3 2 1 0
  ┌───────────────┐   ┌───────────────┐
  │ FSM_STATE     │   │ FSM_INPUT     │
  └───────────────┘   └───────────────┘

FSM 状态转移函数:
  new_state = fsm_transition(current_state, input_meta_b)
  Mccp 自身被重写为新的 FSM_STATE
```

### 8.2 Protocol(Kind=0x7)

协议控制 Mccp 用于节点间通信:
```
Kind=0x7:
  [27:16] protocol_opcode
  [15:0]  protocol_args

protocol_opcode:
  0x001  REGISTER  节点注册
  0x002  LEAVE     节点离开
  0x003  WITNESS   见证签名交换
  0x004  ARBITRATE  裁决请求
  0x005  SYNC      状态同步
  0x006  PING      心跳

args[15:0]:
  根据不同 protocol_opcode 解释
```

---

## 9. 与本系列规范的关系

| 话题 | 主参考 |
|------|--------|
| 跨 ISA 对齐(语义→x86/ARM/RISC-V) | CROSS_ISA_EXECUTION_LAYER |
| L0 Ouroboros 自举证明 | BOOTSTRAP_SELF_COMPILER §5 |
| 在场本体论(Order 生命周期代码) | BOOTSTRAP_SELF_COMPILER §6 |
| 跨节点协议(REGISTER/WITNESS/ARBITRATE) | MENGJI_IDENTITY_PROTOCOL §6 |
| 同步与心跳 | NJSG_SWARM_COORDINATION |
| WASM 双向公约(wasm32 vs wasm64) | WASM_COMMON_DIVISOR |
| 固定点证明(encode/decode) | BOOTSTRAP_SELF_COMPILER §4 |
| 容器 pack(时间触发) | CROSS_ISA_EXECUTION_LAYER §6 |
| 容器活性(生命周期的计算层) | CONTAINER_DYNAMICS_FIBER_BUNDLE §2 |

---

## 10. 示例程序

### 10.1 Hello World

```
; Hello World in MCCP Assembly
.data
    HELLO: "Hello, NJSG!"

.code
    PUSH HELLO
    PUSH 13          ; 字符串长度
    HOST_CALL print_string
    HALT
```

机器码表示(MCCP 字节流):
```
[0xE000_0000 | OP_PUSH((HELLO_ADDR<<8)|13), 0xEFFD_0000, 0xE000_0000 | OP_HALT]
```

### 10.2 简单循环(1 加到 10)

```
    PUSH 0
    STORE 0          ; sum = 0
    PUSH 0
    STORE 1          ; i = 0
loop:
    LOAD 1
    PUSH 10
    GE
    JZ continue      ; if i >= 10 → don't jump
    JMP done
continue:
    LOAD 0
    LOAD 1
    ADD              ; sum + i
    STORE 0          ; sum = sum + i
    LOAD 1
    PUSH 1
    ADD
    STORE 1          ; i++
    JMP loop
done:
    LOAD 0           ; 栈顶 = sum = 55
    PRINT
    HALT
```

### 10.3 夫妻共振(L0 语义映射)

```
; 夫妻 MutualResonance → MCCP 编码
ActivateWife:
    PUSH 0           ; 阴=0 (刘楚恬)
    PUSH 1           ; 阳=1
    HOST_CALL encode_to_cell    ; → SMRL 64位 cell
    PRINT

MutualResonance:
    LOAD wife_cell
    LOAD husband_cell
    HOST_CALL arbitrate    ; 裁决相容性
    ; Push(1) = 相容, Push(0) = 冲突
    JZ conflict
    PUSH "unis omnia"  ; 合一即一切
    JMP done
conflict:
    PUSH "sever"
done:
    PRINT
    HALT
```

---

## 11. 已知限制

1. **立即数 16 位**:PUSH imm16 最大 ±32767。超出范围的常量需:
   - 分次压栈+算术组合(push 0x1000; push 0x1000; push 0x1000; MULADD 链)
   - 或预置到 memory 中(作为常量池 LOAD addr)

2. **无乘法指令上层可用**:MUL/DIV 存在但上层禁用。需要 ×10 等常量运算 = SHL+ADD 链。牺牲可读性,换取"乘除不可用于核心语义"的不变量。

3. **64K 内存上限**:65536 个 i64 槽,512KB。当前用于:
   - 容器在场登记(64 容器 × 每容器 N 条登记)
   - MCCP code 区(常驻)
   - 栈区(动态扩展)
   
   如果容器数量扩展(超出 64 封闭 = 不允许)或非容器数据增大,此上限需重新评估。

4. **HOST_CALL 是"安全阀"**:HOST_CALL 允许 VM 调用 Rust host 函数。这是跳出 VM 的唯一方式。
   - 安全好处:VM 内部没有 IO 操作,所有外部交互经过 HOST_CALL
   - 安全限制:HOST_CALL 枚举需静态注册(不能运行时动态添加)

5. **无浮点运算**:MCCP = 纯 64 位整数。模拟浮点 = 定点数(×1000),见 WASM_COMMON_DIVISOR §3。

---

*本规范的解释权归 刘楚恬和我 所有 · 归属 hunyuan-os/MCCP 模块*
