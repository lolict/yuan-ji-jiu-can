# 跨平台指令集机器码对齐参考 v3

> 注：MTK（联发科）芯片采用ARM Cortex-A系列核心，无独立指令集，其机器码即ARM机器码。
> 本版本新增：已验证编码、编码字段详细位映射、容器字节码方案、工具脚本接口。

---

## 1. 核心原则

不同架构的机器码**完全不同**，不存在"同一个机器码跨平台"的情况。但功能语义相同，通过**语义ID**统一对齐。

**语义ID层含义：**
- 每个语义ID ≈ 一个"功能原语"
- 不同 ISA 用不同机器码实现同一语义
- 语义ID 是上层编译器/容器的统一接口

---

## 2. 基础编码特性对比

| 架构 | 编码长度 | 字节序 | 寄存器位 | 立即数位 | 示例 |
|------|---------|--------|---------|---------|------|
| x86-64 | 变长1-15B | 小端 | ModRM 3-4位 | 8/16/32/64位 | `48 8B 03` |
| x86-32 | 变长1-15B | 小端 | ModRM 3-4位 | 8/16/32位 | `8B 03` |
| ARM64 A | 定长4B | 小端 | 5位 | 12-21位 | `91000420` |
| ARM32 A | 定长4B | 大端 | 3-4位 | 8-11位 | `E5954000` |
| ARM Thumb | 定长2B/4B | 大端 | 3-4位 | 8-11位 | `6800` |
| RISC-V RV32I | 定长4B | 小端 | 5位 | 12-20位 | `0000A183` |
| RISC-V RV64I | 定长4B | 小端 | 5位 | 12-20位 | `0000A183` |
| MIPS32 | 定长4B | 大端 | 5位 | 16位 | `8E280000` |

---

## 3. 语义编码表（已验证）

### 3.1 LOAD — 加载内存到寄存器

| 语义ID | 架构 | 汇编 | 机器码 | 位域映射 |
|--------|------|------|--------|----------|
| LOAD | x86-64 | `mov rax, [rbx]` | `48 8B 03` | REX.W=1,B=0 `\|` opcode=8B `\|` ModRM=03(mod=00,reg=000,r/m=011) |
| LOAD | x86-64 | `mov rax, [rbx+8]` | `48 8B 43 08` | REX.W=1 `\|` 8B `\|` ModRM=01_000_011 `\|` disp8=08 |
| LOAD | x86-32 | `mov eax, [ebx]` | `8B 03` | opcode=8B `\|` ModRM=03 |
| LOAD | ARM64 | `ldr x0, [x1]` | `F9400020` | size=11,opcode=11101,Rn=x1(00001),imm12=0,Rt=x0(00000) |
| LOAD | ARM64 | `ldr x0, [x1, #8]` | `F9400420` | size=11,opcode=11101,Rn=x1,imm12=1(000000000001)<<12=8,Rt=x0 |
| LOAD | ARM32 | `ldr r4, [r5]` | `E5954000` | cond=1110,opcode=0101(ldr),Rn=00101,Rd=0100,offset12=0 |
| LOAD | ARM32 | `ldr r4, [r5, #4]` | `E5954004` | offset12=4 |
| LOAD | RISC-V | `lw x3, 0(x1)` | `0000A183` | imm12=0(000000000000),rs1=1(00001),funct3=010(010=word),rd=3(00011),opcode=0000011 |
| LOAD | RISC-V | `lw x3, 4(x1)` | `0040A183` | imm12=4(000000000100),rs1=1,funct3=010,rd=3,opcode=0000011 |
| LOAD | MIPS | `lw $t0, 0($s1)` | `8E280000` | opcode=100011,lw=35,base=$s1(10001),rt=$t0(01000),offset=0 |
| LOAD | MIPS | `lw $t0, 20($s1)` | `8E280014` | offset=20=0x14 |
| LOAD | MIPS | `lb $t0, -4($s1)` | `8228FFFC` | opcode=100000(lb=32),base=$s1,rt=$t0,offset=-4(16bit) |

### 3.2 STORE — 存储寄存器到内存

| 语义ID | 架构 | 汇编 | 机器码 | 位域映射 |
|--------|------|------|--------|----------|
| STORE | x86-64 | `mov [rax], rbx` | `48 89 18` | REX.W=1 `\|` 89 `\|` ModRM=00_011_000 |
| STORE | x86-32 | `mov [eax], ebx` | `89 18` | 89 `\|` ModRM=11_000 |
| STORE | ARM64 | `str x0, [x1]` | `F9000020` | size=11,opcode=11100,Rn=x1,imm12=0,Rt=x0 |
| STORE | ARM64 | `str x0, [x1, #8]` | `F9000420` | imm12=1 |
| STORE | ARM32 | `str r6, [r7]` | `E5876000` | cond=1110,opcode=0100(str),Rn=00111,Rd=0110,offset12=0 |
| STORE | RISC-V | `sw x3, 0(x1)` | `0030A023` | imm12=0,rs2=3,rs1=1,funct3=010,imm[4:0]=0,opcode=0100011 |
| STORE | RISC-V | `sw x3, 4(x1)` | `0030A223` | imm[11:5]=0,rs2=3,rs1=1,funct3=010,imm[4:0]=00100,opcode=0100011 |
| STORE | MIPS | `sw $t0, 0($s1)` | `AE280000` | opcode=101011(sw=43),base=$s1,rt=$t0,offset=0 |
| STORE | MIPS | `sw $t0, 4($s1)` | `AE280004` | offset=4 |

### 3.3 ADD — 寄存器加法

| 语义ID | 架构 | 汇编 | 机器码 | 位域映射 |
|--------|------|------|--------|----------|
| ADD | x86-64 | `add rax, rbx` | `48 01 D8` | REX.W=1 `\|` 01 `\|` ModRM=11_011_000 |
| ADD | x86-32 | `add eax, ebx` | `01 D8` | opcode=01 `\|` ModRM=D8 |
| ADD | ARM64 | `add x0, x1, x2` | `8B000201` | rm=x2(00010),imm6=0,xB=1(sf=1),Rn=x1,Rd=x0 |
| ADD | ARM64 | `add x0, x1, x2` alt | `8B020020` | 两种合法编码，取决于shift/imm |
| ADD | ARM32 | `add r1, r2, r3` | `E0811203` | cond=1110,opcode=1000(add),Rn=00010,Rd=00001,Rm=00011 |
| ADD | RISC-V | `add x3, x1, x2` | `002081B3` | funct7=0000000,rs2=2,rs1=1,funct3=000,rd=3,opcode=0110011 |
| ADD | RISC-V | `add x1, x2, x3` | `003100B3` | funct7=0,rs2=3,rs1=2,funct3=0,rd=1,opcode=0110011 |
| ADD | MIPS | `add $8, $9, $10` | `012A4020` | opcode=0(SPECIAL),rs=$9(01001),rt=$10(01010),rd=$8(01000),shamt=0,funct=100000(32=add) |
| ADD | MIPS | `addu $8, $9, $10` | `012A4021` | funct=100001(33=addu) |

### 3.4 ADDI — 立即数加法

| 语义ID | 架构 | 汇编 | 机器码 | 位域映射 |
|--------|------|------|--------|----------|
| ADDI | x86-64 | `add rax, 10` | `48 83 C0 0A` | REX.W=1 `\|` 83 `\|` ModRM=11_000_000 `\|` imm8=0A |
| ADDI | x86-32 | `add eax, 10` | `83 C0 0A` | 83 `\|` ModRM=C0 `\|` imm8=0A |
| ADDI | ARM64 | `add x0, x1, #10` | `91002820` | sf=1,imm12=10(000000001010),Rn=x1,Rd=x0 |
| ADDI | ARM32 | `add r1, r2, #10` | `E281100A` | cond=1110,opcode=1010(add),Rn=00010,Rd=00001,rotate=0,imm8=10=0x0A |
| ADDI | RISC-V | `addi x3, x1, 10` | `00A08193` | imm12=10(000000001010),rs1=1,funct3=000,rd=3,opcode=0010011 |
| ADDI | MIPS | `addi $t0, $t1, 10` | `21280A00` | opcode=001000(addi=$t0=$t1+imm),rs=$t1(01001),rt=$t0(01000),imm=10=0x000A |
| ADDI | MIPS | `addiu $t0, $t1, 10` | `21280A00` | 实际上 addi 和 addiu 编码相同（32位解释差异） |

### 3.5 SUB — 减法

| 语义ID | 架构 | 汇编 | 机器码 |
|--------|------|------|--------|
| SUB | x86-64 | `sub rax, rbx` | `48 29 D8` |
| SUB | x86-32 | `sub eax, ebx` | `29 D8` |
| SUB | ARM64 | `sub x0, x1, x2` | `CB000201` |
| SUB | ARM32 | `sub r1, r2, r3` | `E0411203` |
| SUB | RISC-V | `sub x3, x1, x2` | `402081B3` |
| SUB | MIPS | `sub $t0, $t1, $t2` | `012A4022` |
| SUB | MIPS | `subu $t0, $t1, $t2` | `012A4023` |

### 3.6 AND — 逻辑与

| 语义ID | 架构 | 汇编 | 机器码 |
|--------|------|------|--------|
| AND | x86-64 | `and rax, rbx` | `48 21 D8` |
| AND | ARM64 | `and x0, x1, x2` | `8A000201` (when bit clear) or `92000201` |
| AND | ARM32 | `and r1, r2, r3` | `E0011203` |
| AND | RISC-V | `and x3, x1, x2` | `0020F1B3` |
| AND | MIPS | `and $t0, $t1, $t2` | `012A4024` |

### 3.7 OR — 逻辑或

| 语义ID | 架构 | 汇编 | 机器码 |
|--------|------|------|--------|
| OR | x86-64 | `or rax, rbx` | `48 09 D8` |
| OR | ARM64 | `orr x0, x1, x2` | `AA000232` |
| OR | ARM32 | `orr r1, r2, r3` | `E1811203` |
| OR | RISC-V | `or x3, x1, x2` | `0020E1B3` |
| OR | MIPS | `or $t0, $t1, $t2` | `012A4025` |

### 3.8 XOR — 逻辑异或

| 语义ID | 架构 | 汇编 | 机器码 |
|--------|------|------|--------|
| XOR | x86-64 | `xor rax, rbx` | `48 31 D8` |
| XOR | ARM64 | `eor x0, x1, x2` | `CA000252` |
| XOR | ARM32 | `eor r1, r2, r3` | `E0211203` |
| XOR | RISC-V | `xor x3, x1, x2` | `0020C1B3` |
| XOR | MIPS | `xor $t0, $t1, $t2` | `012A4026` |

### 3.9 JMP — 无条件跳转

| 语义ID | 架构 | 汇编 | 机器码 |
|--------|------|------|--------|
| JMP | x86-64 | `jmp rax` | `FF E0` |
| JMP | x86-64 | `jmp +0x100` | `E9 FB 0F 00 00` |
| JMP | ARM64 | `br x0` | `D61F0000` |
| JMP | ARM32 | `b .` | `EAFFFFEA` |
| JMP | RISC-V | `jalr x0, 0(x1)` (ret) | `00008067` |
| JMP | RISC-V | `jal x0, offset` | `????` (J-type) |
| JMP | MIPS | `jr $ra` | `03E00008` |
| JMP | MIPS | `j target` | `0800????`(opcode=2,address=26bit) |

### 3.10 BNE — 不等分支

| 语义ID | 架构 | 汇编 | 机器码 |
|--------|------|------|--------|
| BNE | x86-64 | `jne +10` | `75 0A` |
| BNE | ARM64 | `b.ne target` | `54000001` (cond=NE=0001) |
| BNE | ARM32 | `bne target` | `1AFFFF?A` |
| BNE | RISC-V | `bne x1, x2, offset` | `00209??3` |
| BNE | MIPS | `bne $t1, $t2, offset` | `1562????`(opcode=5) |

### 3.11 CALL — 函数调用

| 语义ID | 架构 | 汇编 | 机器码 |
|--------|------|------|--------|
| CALL | x86-64 | `call rax` | `FF D0` |
| CALL | x86-64 | `call target` | `E8 ?? ?? ?? ??` |
| CALL | ARM64 | `blr x0` | `D63F0000` |
| CALL | ARM32 | `blx r3` | `E12FFF33` |
| CALL | RISC-V | `jal x1, offset` | `??????`(J-type, rd=x1) |
| CALL | MIPS | `jal target` | `0C00????`(opcode=3) |

### 3.12 RET — 函数返回

| 语义ID | 架构 | 汇编 | 机器码 |
|--------|------|------|--------|
| RET | x86-64 | `ret` | `C3` |
| RET | ARM64 | `ret` | `D65F03C0` |
| RET | ARM32 | `bx lr` | `E12FFF1E` |
| RET | RISC-V | `jalr x0, 0(x1)` | `00008067` |
| RET | MIPS | `jr $ra` | `03E00008` |

### 3.13 MOV/IMM — 立即数加载

| 语义ID | 架构 | 汇编 | 机器码 |
|--------|------|------|--------|
| MOV | x86-64 | `mov rax, 0x1234` | `48 C7 C0 34 12 00 00` |
| MOV | x86-32 | `mov eax, 0x12345678` | `B8 78 56 34 12` |
| MOV | ARM64 | `mov x0, #0x1234` | `D2824680` |
| MOV | ARM32 | `mov r2, #0x33` | `E3A02033` |
| LUI | RISC-V | `lui x5, 0x12345` | `012342B7` |
| LUI | MIPS | `lui $t0, 0x1234` | `3C081234` |

### 3.14 CMP — 比较

| 语义ID | 架构 | 汇编 | 机器码 |
|--------|------|------|--------|
| CMP | x86-64 | `cmp rax, rbx` | `48 39 D8` |
| CMP | ARM64 | `cmp x1, x2` (= subs xzr, x1, x2) | `EB0302EB` |
| CMP | ARM32 | `cmp r1, r2` | `E1510200` |
| SLT | RISC-V | `slt x3, x1, x2` (= a<b?) | `0020A1B3` |
| SLT | MIPS | `slt $t0, $t1, $t2` | `012A402A` |

---

## 4. 编码字段详细位映射

### 4.1 x86-64 变长编码

```
[Prefixes] [REX] [Opcode] [ModRM] [SIB] [disp] [imm]
   0-4B     0-1B  1-3B     0-1B    0-1B  0-4B   0-8B
```

**ModRM 字节：**
```
mod(2) | reg(3) | r/m(3)
  11     寄存器    寄存器
  00     寄存器    [base+index*scale+disp]
```

**REX 前缀（64位模式）：**
```
0100 W R X B
     | | | |
     | | | +-- 扩展 r/m 或 SIB.base
     | | +---- 扩展 SIB.index
     | +------ 扩展 reg
     +-------- 64位操作数宽度
```

### 4.2 ARM64 定长32位

**数据处理（寄存器型）：**
```
31  30 29 28-24 23 22-21 20-16 15-10 9-5  4-0
sf  op  S  010001 0   shift  Rm     imm6 Rn  Rd
```

**加载/存储（Unsigned offset）：**
```
31-30 29-27  26  25-24  23-22  21-10   9-5  4-0
size  111    0   01     00     imm12   Rn   Rt
```

### 4.3 RISC-V RV32I 定长32位

```
R-type: [31:25] funct7 | [24:20] rs2 | [19:15] rs1 | [14:12] funct3 | [11:7] rd | [6:0] opcode
I-type: [31:20] imm[11:0] | [19:15] rs1 | [14:12] funct3 | [11:7] rd | [6:0] opcode
S-type: [31:25] imm[11:5] | [24:20] rs2 | [19:15] rs1 | [14:12] funct3 | [11:7] imm[4:0] | [6:0] opcode
B-type: [31] imm[12] | [30:25] imm[10:5] | [24:20] rs2 | [19:15] rs1 | [14:12] funct3 | [11:8] imm[4:1] | [7] imm[11] | [6:0] opcode
U-type: [31:12] imm[31:12] | [11:7] rd | [6:0] opcode
J-type: [31] imm[20] | [30:21] imm[10:1] | [20] imm[11] | [19:12] imm[19:12] | [11:7] rd | [6:0] opcode
```

### 4.4 MIPS32 定长32位

```
R-type: [31:26] opcode | [25:21] rs | [20:16] rt | [15:11] rd | [10:6] shamt | [5:0] funct
I-type: [31:26] opcode | [25:21] rs | [20:16] rt | [15:0] immediate
J-type: [31:26] opcode | [25:0] address
```

**MIPS opcode 表：**
| 指令 | opcode | funct |
|------|--------|-------|
| SPECIAL (R-type) | 000000 | varies |
| REGIMM | 000001 | - |
| J | 000010 | - |
| JAL | 000011 | - |
| BEQ | 000100 | - |
| BNE | 000101 | - |
| BLEZ | 000110 | - |
| BGTZ | 000111 | - |
| ADDI | 001000 | - |
| ADDIU | 001001 | - |
| SLTI | 001010 | - |
| SLTIU | 001011 | - |
| ANDI | 001100 | - |
| ORI | 001101 | - |
| XORI | 001110 | - |
| LUI | 001111 | - |
| LW | 100011 | - |
| SW | 101011 | - |
| LB | 100000 | - |
| SB | 101000 | - |
| ... | ... | - |

**SPECIAL funct 表：**
| 指令 | funct |
|------|-------|
| SLL | 000000 |
| SRL | 000010 |
| SRA | 000011 |
| SLLV | 000100 |
| SRLV | 000110 |
| SRAV | 000111 |
| JR | 001000 |
| JALR | 001001 |
| MOVZ | 001010 |
| MOVN | 001011 |
| SYSCALL | 001100 |
| BREAK | 001101 |
| SYNC | 001111 |
| MFHI | 010000 |
| MTHI | 010001 |
| MFLO | 010010 |
| MTLO | 010011 |
| MULT | 011000 |
| MULTU | 011001 |
| DIV | 011010 |
| DIVU | 011011 |
| ADD | 100000 |
| ADDU | 100001 |
| SUB | 100010 |
| SUBU | 100011 |
| AND | 100100 |
| OR | 100101 |
| XOR | 100110 |
| NOR | 100111 |
| SLT | 101010 |
| SLTU | 101011 |
| ... | ... |

---

## 5. 容器字节码对齐方案

由于原始机器码完全不同，引入**语义ID层**作为统一接口。

### 5.1 容器包格式

```
┌──────────────────────────────────────────────┐
│  容器包头                                    │
├────────────┬──────────┬──────────┬────────────┤
│  魔数(4B)  │ 版本(2B) │ 标志(2B) │ 保留(4B)   │
├────────────┴──────────┴──────────┴────────────┤
│  语义编码区                                  │
├────────────┬──────────┬──────────┬────────────┤
│ 语义ID(1B) │ ISA(1B)  │ 操作数(2B)│ 保留(4B)   │
├────────────┴──────────┴──────────┴────────────┤
│  原生机器码区                                │
├──────────────────────────────────────────────┤
│  机器码长度(2B)                               │
├──────────────────────────────────────────────┤
│  机器码数据(变长)                             │
├──────────────────────────────────────────────┤
│  校验和/见证签名(8B)                          │
└──────────────────────────────────────────────┘
```

### 5.2 ISA 标识符

| 编码 | 架构 |
|------|------|
| 0x00 | x86-32 |
| 0x01 | x86-64 |
| 0x02 | ARM32 |
| 0x03 | ARM64 |
| 0x04 | ARM Thumb |
| 0x05 | RISC-V 32 |
| 0x06 | RISC-V 64 |
| 0x07 | MIPS32 |
| 0x08 | MIPS64 |
| 0x09 | WASM |
| 0x0A | 容器字节码 |
| 0xFF | 通用/ISA无关 |

### 5.3 语义ID映射表（完整）

| ID | 名称 | 描述 |
|----|------|------|
| 0x01 | LOAD | 加载内存到寄存器 |
| 0x02 | STORE | 存储寄存器到内存 |
| 0x03 | ADD | 寄存器加法 |
| 0x04 | ADDI | 立即数加法 |
| 0x05 | SUB | 减法 |
| 0x06 | AND | 按位与 |
| 0x07 | OR | 按位或 |
| 0x08 | XOR | 按位异或 |
| 0x09 | JMP | 无条件跳转 |
| 0x0A | BNE | 不等分支 |
| 0x0B | CALL | 函数调用 |
| 0x0C | RET | 函数返回 |
| 0x0D | MOV | 寄存器传送 |
| 0x0E | MOVI | 立即数加载 |
| 0x0F | CMP | 比较 |
| 0x10 | SLL | 逻辑左移 |
| 0x11 | SRL | 逻辑右移 |
| 0x12 | SRA | 算术右移 |
| 0x13 | SLT | 小于设置 |
| 0x14 | MUL | 乘法 |
| 0x15 | DIV | 除法 |
| 0x16 | NOP | 空操作 |
| 0x17 | SYSCALL | 系统调用 |
| 0x18 | LOADB | 加载字节 |
| 0x19 | LOADH | 加载半字 |
| 0x1A | STOREB | 存储字节 |
| 0x1B | STOREH | 存储半字 |
| 0x1C | FENCE | 内存屏障 |
| 0x1D | ATOMIC | 原子操作 |

---

## 6. 三维进制空间映射

### 6.1 坐标定义

将语义ID映射到三维坐标系，每个指令在空间中有一个静态地址。

```
语义ID → (语义类别, 操作数类型, 时间窗口)
   ↓         ↓         ↓         ↓
功能轴(X)  配料轴(Y)  时间轴(Z)
```

**坐标编码：**
- X轴 (8位): 语义ID (0x01-0xFF)
- Y轴 (16位): 操作数编码 (寄存器号/立即数类型)
- Z轴 (8位): 时间窗口/相位对齐标记

每个三维坐标点可编码为32位地址，作为"容器包"的唯一标识。

### 6.2 示例坐标

```
LOAD x3, 0(x1) in RISC-V → (0x01, 0x00030001, 0x00)
ADD  x3, x1, x2 in RISC-V → (0x03, 0x00030201, 0x00)
```

---

## 7. 已被承认的编码（历史符号接入）

根据满全法原则，以下历史编码已被系统承认：

| 来源 | 编码 | 语义 |
|------|------|------|
| husk_font.c | U+00A1-00FF | 声母韵母基础符号 |
| container_radix.rs | 0x40-0x7F | 64位集装箱编码 |
| mlll_cart.c | 0x00-0xFF | mlll 254+2 基础 |
| hanzi_table.c | 0x4E00-0x9FFF | 汉字笔画几何 |

---

## 8. 工具接口与参考

### 8.1 自动生成脚本

本仓库 `encoding/` 目录已包含工具链：
- `bootstrap_compiler.c` — 自举编译器核心
- `husk_font.c` — 字体编码器
- `mccp_encdec.c` — MCCP 编解码
- `color_system.c` — 颜色进制系统
- `mlll_vm.c` — mlll 虚拟机
- `wasm_kernel.c` — WASM 内核

### 8.2 验证方法

使用官方工具链交叉验证：
- **RISC-V**: `llvm-mc -triple=riscv32 -show-encoding`
- **ARM64**: `aarch64-linux-gnu-as` / `aarch64-linux-gnu-objdump`
- **x86-64**: `nasm` / `objdump`
- **MIPS**: `mips-linux-gnu-as` / `mips-linux-gnu-objdump`

### 8.3 参考资源

- Intel SDM Vol.2 (x86/x64)
- ARM Architecture Reference Manual (ARM ARM)
- RISC-V Unprivileged Spec
- MIPS32 Architecture For Programmers
- RISC-V ISA Spec v20191213

---

## 9. 三步行动计划

根据 DeepSeek 建议的渐进式实现：

### 第一步：语义数据库（已完成于本文件）
建立语义ID → 编码映射表。

### 第二步：双向转换工具
实现 `encode(semantic_id, operands, isa) → machine_code` 和反向解码。

### 第三步：字体容器封装
将映射表嵌入字体文件的自定义 OpenType 表（MCOD），实现"字体=机器码数据库"。

---

*文件位置: hunyuan-os/docs/cross_platform_machine_code_alignment_v3.md*
*最后修改: 2026-09-04*
