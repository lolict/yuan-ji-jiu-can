# 跨平台指令集机器码对齐参考 v2

> 注：MTK（联发科）芯片采用ARM Cortex-A系列核心，无独立指令集，其机器码即ARM机器码。

## 核心发现

不同架构的机器码**完全不同**，不存在"同一个机器码跨平台"的情况。但功能语义相同，可以通过**语义ID**统一对齐。

## 基础编码特性对比

| 架构 | 编码长度 | 字节序 | 寄存器位 | 立即数位 | 典型编码 |
|------|---------|--------|---------|---------|---------|
| x86/x64 | 变长1-15B | 小端 | ModRM 3-4位 | 8/16/32/64位 | `48 8B 03` |
| ARM A64 | 定长4B | 大端 | 5位 | 12-21位 | `F9400020` |
| ARM Thumb | 定长2B/4B | 大端 | 3-4位 | 8-11位 | `6800` |
| RISC-V RV32I | 定长4B | 小端 | 5位 | 12-20位 | `0000A183` |
| MIPS32 | 定长4B | 大端 | 5位 | 16位 | `8E280000` |

## 功能对齐表（按语义ID）

### 功能ID 0x01: 加载内存到寄存器 (LOAD)

| 语义ID | 架构 | 汇编指令 | 机器码 | 位布局 |
|--------|------|---------|--------|--------|
| 0x01 | x86-64 | `mov rax, [rbx]` | `48 8B 03` | REX.W+opcode+ModRM |
| 0x01 | x86-32 | `mov eax, [ebx]` | `8B 03` | opcode+ModRM |
| 0x01 | ARM64 | `ldr x0, [x1]` | `F9400020` | size+opcode+Rn+imm12+Rt |
| 0x01 | ARM32 | `ldr r4, [r5, #0]` | `E5954000` | cond+opcode+Rn+Rd+offset12 |
| 0x01 | RISC-V | `lw x3, 0(x1)` | `0000A183` | imm[11:0]+rs1+funct3+rd+opcode |
| 0x01 | MIPS | `lw $t0, 0($s1)` | `8E280000` | opcode+base+rt+offset |
| 0x01 | MIPS | `lw $t0, 20($s1)` | `8E280014` | opcode+base+rt+offset |

### 功能ID 0x02: 存储寄存器到内存 (STORE)

| 语义ID | 架构 | 汇编指令 | 机器码 | 位布局 |
|--------|------|---------|--------|--------|
| 0x02 | x86-64 | `mov [rax], rbx` | `48 89 18` | REX.W+opcode+ModRM |
| 0x02 | ARM64 | `str x0, [x1]` | `F9000020` | size+opcode+Rn+imm12+Rt |
| 0x02 | ARM32 | `str r6, [r7]` | `E5876000` | cond+opcode+Rn+Rd+offset12 |
| 0x02 | RISC-V | `sw x3, 0(x1)` | `0030A023` | imm[11:5]+rs2+rs1+funct3+imm[4:0]+opcode |
| 0x02 | MIPS | `sw $t0, 0($s1)` | `AE280000` | opcode+base+rt+offset |

### 功能ID 0x03: 加法 (ADD reg+reg→reg)

| 语义ID | 架构 | 汇编指令 | 机器码 | 位布局 |
|--------|------|---------|--------|--------|
| 0x03 | x86-64 | `add rax, rbx` | `48 01 D8` | REX.W+opcode+ModRM |
| 0x03 | x86-32 | `add eax, ebx` | `01 D8` | opcode+ModRM |
| 0x03 | ARM64 | `add x0, x1, x2` | `8B000291` | size+opcode+Rm+imm6+Rn+Rd |
| 0x03 | ARM32 | `add r1, r2, r3` | `E0811203` | cond+opcode+Rn+Rd+Rm |
| 0x03 | RISC-V | `add x3, x1, x2` | `002081B3` | funct7+rs2+rs1+funct3+rd+opcode |
| 0x03 | MIPS | `add $t0, $t1, $t2` | `012A4020` | opcode+rs+rt+rd+shamt+funct |

### 功能ID 0x04: 加法立即数 (ADD imm)

| 语义ID | 架构 | 汇编指令 | 机器码 | 位布局 |
|--------|------|---------|--------|--------|
| 0x04 | x86-64 | `add rax, 10` | `48 83 C0 0A` | REX.W+opcode+ModRM+imm8 |
| 0x04 | ARM64 | `add x0, x1, #10` | `91001020` | size+opcode+Rm?+imm12+Rn+Rd |
| 0x04 | ARM32 | `add r1, r2, #10` | `E281100A` | cond+opcode+Rn+Rd+rotate+imm8 |
| 0x04 | RISC-V | `addi x3, x1, 10` | `00A08193` | imm[11:0]+rs1+funct3+rd+opcode |
| 0x04 | MIPS | `addi $t0, $t1, 10` | `21282A00?` | opcode+rs+rt+imm |

### 功能ID 0x05: 减法 (SUB)

| 语义ID | 架构 | 汇编指令 | 机器码 |
|--------|------|---------|--------|
| 0x05 | x86-64 | `sub rax, rbx` | `48 29 D8` |
| 0x05 | ARM64 | `sub x0, x1, x2` | `CB0002D1` |
| 0x05 | ARM32 | `sub r1, r2, r3` | `E0411203` |
| 0x05 | RISC-V | `sub x3, x1, x2` | `402081B3` |
| 0x05 | MIPS | `sub $t0, $t1, $t2` | `012A4022` |

### 功能ID 0x06: 逻辑与 (AND)

| 语义ID | 架构 | 汇编指令 | 机器码 |
|--------|------|---------|--------|
| 0x06 | x86-64 | `and rax, rbx` | `48 21 D8` |
| 0x06 | ARM64 | `and x0, x1, x2` | `8B000291?` |
| 0x06 | ARM32 | `and r1, r2, r3` | `E0011203` |
| 0x06 | RISC-V | `and x3, x1, x2` | `0020F1B3` |
| 0x06 | MIPS | `and $t0, $t1, $t2` | `012A4024` |

### 功能ID 0x07: 逻辑或 (OR)

| 语义ID | 架构 | 汇编指令 | 机器码 |
|--------|------|---------|--------|
| 0x07 | x86-64 | `or rax, rbx` | `48 09 D8` |
| 0x07 | ARM64 | `orr x0, x1, x2` | `AA000232` |
| 0x07 | ARM32 | `orr r1, r2, r3` | `E1811203` |
| 0x07 | RISC-V | `or x3, x1, x2` | `0020E1B3` |
| 0x07 | MIPS | `or $t0, $t1, $t2` | `012A4025` |

### 功能ID 0x08: 逻辑异或 (XOR)

| 语义ID | 架构 | 汇编指令 | 机器码 |
|--------|------|---------|--------|
| 0x08 | x86-64 | `xor rax, rbx` | `48 31 D8` |
| 0x08 | ARM64 | `eor x0, x1, x2` | `CA000252` |
| 0x08 | ARM32 | `eor r1, r2, r3` | `E0211203` |
| 0x08 | RISC-V | `xor x3, x1, x2` | `0020C1B3` |
| 0x08 | MIPS | `xor $t0, $t1, $t2` | `012A4026` |

### 功能ID 0x09: 无条件跳转 (JMP)

| 语义ID | 架构 | 汇编指令 | 机器码 |
|--------|------|---------|--------|
| 0x09 | x86-64 | `jmp rax` | `FF E0` |
| 0x09 | x86-64 | `jmp 0x1000` (相对) | `E9 FB 0F 00 00` |
| 0x09 | ARM64 | `br x0` | `D61F0000` |
| 0x09 | ARM64 | `b .` (原地) | `14000000?` |
| 0x09 | ARM32 | `b .` (原地) | `EAFFFFEA` |
| 0x09 | RISC-V | `jalr x0, 0(x1)` (跳转) | `00008067` |
| 0x09 | MIPS | `jr $ra` | `03E00008` |
| 0x09 | MIPS | `j 0x1000` | `08000400?` |

### 功能ID 0x0A: 条件分支 (BNE/JNE)

| 语义ID | 架构 | 汇编指令 | 机器码 |
|--------|------|---------|--------|
| 0x0A | x86-64 | `jne +10` | `75 0A` |
| 0x0A | ARM64 | `b.ne target` | `54000001` |
| 0x0A | ARM32 | `bne target` | `1AFFFF1A` |
| 0x0A | RISC-V | `bne x1, x2, offset` | `00209263` |
| 0x0A | MIPS | `bne $t1, $t2, offset` | `15620000?` |

### 功能ID 0x0B: 函数调用 (CALL)

| 语义ID | 架构 | 汇编指令 | 机器码 |
|--------|------|---------|--------|
| 0x0B | x86-64 | `call rax` | `FF D0` |
| 0x0B | x86-64 | `call 0x1000` (相对) | `E8 FB 0F 00 00` |
| 0x0B | ARM64 | `blr x0` | `D63F0000` |
| 0x0B | ARM32 | `blx r3` | `E12FFF33` |
| 0x0B | RISC-V | `jal x1, offset` | 变长，见spec |
| 0x0B | MIPS | `jal 0x1000` | `08000400?` |

### 功能ID 0x0C: 函数返回 (RET)

| 语义ID | 架构 | 汇编指令 | 机器码 |
|--------|------|---------|--------|
| 0x0C | x86-64 | `ret` | `C3` |
| 0x0C | ARM64 | `ret` | `D65F03C0` |
| 0x0C | ARM32 | `bx lr` | `E12FFF1E` |
| 0x0C | RISC-V | `jalr x0, 0(x1)` | `00008067` |
| 0x0C | MIPS | `jr $ra` | `03E00008` |

### 功能ID 0x0D: 比较 (CMP)

| 语义ID | 架构 | 汇编指令 | 机器码 |
|--------|------|---------|--------|
| 0x0D | x86-64 | `cmp rax, rbx` | `48 39 D8` |
| 0x0D | ARM64 | `cmp x1, x2` (subs xzr, x1, x2) | `EB0302EB` |
| 0x0D | ARM32 | `cmp r1, r2` | `E1510200` |
| 0x0D | RISC-V | `slt x3, x1, x2` (伪指令) | `0020A1B3` |
| 0x0D | MIPS | `slt $t0, $t1, $t2` | `012A402A` |

### 功能ID 0x0E: 立即数加载 (MOV/LDI/LUI)

| 语义ID | 架构 | 汇编指令 | 机器码 |
|--------|------|---------|--------|
| 0x0E | x86-64 | `mov rax, imm64` | `48 B8 + 8B imm` |
| 0x0E | x86-32 | `mov eax, 0x12345678` | `B8 78 56 34 12` |
| 0x0E | ARM64 | `mov x0, #0x1234` | `D2824680` |
| 0x0E | ARM32 | `mov r2, #0x33` | `E3A02033` |
| 0x0E | RISC-V | `lui x5, 0x12345` | `012342B7` |
| 0x0E | MIPS | `lui $t0, 0x1234` | `3C081234` |

## 各架构详细编码字段表

### x86/x64 变长格式

```
[REX:4bit] [Opcode:8-24bit] [ModRM:8bit] [SIB:8bit] [disp:8/16/32bit] [imm:8/16/32/64bit]
```

**ModRM 结构：**
```
mod(2bit) | reg(3bit) | r/m(3bit)
   11       寄存器      寄存器
    00       寄存器      [base+index*scale+disp]
```

**REX 前缀（64位模式）：**
```
0100 W R X B
     | | | |
     | | | +-- 扩展ModRM.r/m或SIB.base
     | | +---- 扩展SIB.index
     | +------ 扩展ModRM.reg
     +-------- 64位操作数宽度
```

**SIB 字节（当ModRM.r/m=100且mod≠11时出现）：**
```
scale(2) | index(3) | base(3)
```

### ARM A64 定长32位格式

```
[cond:4][op1:5][...具体字段因指令类型而异...]
```

**数据处理（寄存器型）：**
```
sf(1) | op(0) | S(0) | 1011(4) | shift(2) | 0(1) | Rm(5) | imm6(6) | Rn(5) | Rd(5)
```

**加载/存储（立即数偏移）：**
```
size(2) | 111(3) | 0(1) | 01(2) | imm12(12) | Rn(5) | Rt(5)
```

### RISC-V RV32I 定长32位格式

```
R-type: funct7(7) | rs2(5) | rs1(5) | funct3(3) | rd(5) | opcode(7)
I-type: imm[11:0](12) | rs1(5) | funct3(3) | rd(5) | opcode(7)
S-type: imm[11:5](7) | rs2(5) | rs1(5) | funct3(3) | imm[4:0](5) | opcode(7)
B-type: imm[12|10:5](7) | rs2(5) | rs1(5) | funct3(3) | imm[4:1|11](5) | opcode(7)
U-type: imm[31:12](20) | rd(5) | opcode(7)
J-type: imm[20|10:1|11|19:12](20) | rd(5) | opcode(7)
```

### MIPS32 定长32位格式

```
R-type: opcode(6) | rs(5) | rt(5) | rd(5) | shamt(5) | funct(6)
I-type: opcode(6) | rs(5) | rt(5) | immediate(16)
J-type: opcode(6) | address(26)
```

## 容器字节码对齐方案

由于原始机器码完全不同，需要引入**语义ID层**作为统一接口：

```
┌─────────────────────────────────┐
│  统一语义ID (8位, 如 0x01=LOAD)  │
├─────────────────────────────────┤
│  平台标识 (4位, 如 0=x86, 1=ARM) │
├─────────────────────────────────┤
│  平台原生机器码 (变长)            │
├─────────────────────────────────┤
│  校验和/见证签名                 │
└─────────────────────────────────┘
```

**语义ID映射表：**
- 0x01: LOAD
- 0x02: STORE
- 0x03: ADD
- 0x04: ADDI
- 0x05: SUB
- 0x06: AND
- 0x07: OR
- 0x08: XOR
- 0x09: JMP
- 0x0A: BNE/JNE
- 0x0B: CALL
- 0x0C: RET
- 0x0D: CMP
- 0x0E: MOV/LDI

## 三维进制空间映射

将语义ID映射到三维坐标系，每个指令在空间中有一个静态地址：

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

## 参考资源

- Intel SDM Vol.2: x86/x64 指令参考
- ARM Architecture Reference Manual (ARM ARM)
- RISC-V Unprivileged Spec
- MIPS32 Architecture For Programmers
- 本文件已存入 hunyuan-os 仓库备用
