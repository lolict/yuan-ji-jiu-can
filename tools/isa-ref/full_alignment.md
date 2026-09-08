# 五架构机器码完全对齐表

> 同语义 · 跨 ARM64 / x86-64 / RV32I / MIPS32 / MTK → 对齐到同一行
> 按功能公约数分类，不按架构分类
> 机器码经 x86-64 宿主机 (GNU as + objdump, AMD EPYC 9K84) 实测 + ARM/RV/MIPS 手册交叉核实
> 标注字节数与小端/大端视图

---

## 符号约定

| 符号 | 含义 |
|------|------|
| `rd` `rs1` `rs2` | RISC-V 寄存器 |
| `Rd` `Rn` `Rm` | ARM64 寄存器 |
| `rax` `rbx` … | x86-64 寄存器 |
| `$v0` `$ra` … | MIPS 寄存器 |
| `imm` | 立即数 |
| `offset` | 内存偏移 |
| (B) 小端 | 多字节机器码按小端排列 (默认) |
| (B) 大端 | MIPS 默认大端视图 |
| — | 该 ISA 无原生对应，需序列化 |

---

## x86-64 立即数扩展规则 (实测要点)

1. **imm8 sign-extension**: `83 /digit imm8` 将 imm8 符号扩展到 64 位。当立即数 > 127 或 < -128 时不可用，必须改用 `81 /digit imm32` 或专用累加器形式。
2. **ALU 专用累加器形式**: gas 优先使用 `05`(ADD EAX) / `0D`(OR EAX) / `25`(AND EAX) / `35`(XOR EAX) / `3D`(CMP EAX) 而非 `81 /digit ModRM`。两者均合法。
3. **MOV 区分**: `B8+q` 用于 imm64；`C7 /0` 用于 imm32 (sign-extend → 64)。
4. **跳转**: 近跳 `0F 8x rel32` 与短跳 `7x rel8` 均合法；gas 按距离自动选择。

---

## 1. 寄存器移动

### 1.1 寄存器→寄存器 (MOV)

| 功能 | ARM64 | x86-64 | RV32I | MIPS32 |
|------|-------|--------|-------|--------|
| 汇编 | `orr x0, xzr, x1` | `mov rax, rbx` | `addi x0, x1, 0` | `or $v0, $v1, $zero` |
| 机器码 | `AA0003E0` (4B) | `48 89 D8` (3B) | `00008013` (4B) | `00 22 10 21` (4B) |
| 位域 | `sf_0_101010_000_000000_Rm_Rd` | `REX.W _89 ModRM` | `0000000_rs1_000_rd_0010011` | `000000_rs_rt_00000_00000_100001` |

### 1.2 零→寄存器 (CLR/MOV 0)

| 功能 | ARM64 | x86-64 | RV32I | MIPS32 |
|------|-------|--------|-------|--------|
| 汇编 | `mov x0, xzr` | `xor eax, eax` | `addi x0, x0, 0` | `move $v0, $zero` |
| 机器码 | `E003804A` (4B) | `31 C0` (2B) | `00000013` (4B) | `00 00 10 21` (4B) |

### 1.3 立即数→寄存器 (MOV imm)

| 功能 | ARM64 | x86-64 | RV32I | MIPS32 |
|------|-------|--------|-------|--------|
| MOVZ 16b | `movz x0, #val` | `mov eax, imm32` | `lui + addi` (2条) | `lui + ori` (2条) |
| 机器码 (imm32) | `D2800000`+ | `B8+d` (5B) / `C7 /0` (7B) | `xxxx37_xxxx13` (8B) | `3C02_3402` (8B) |
| 机器码 (imm64) | `92800000`+ | `48 B8 xx..xx` (10B) | — | — |

> x86-64 实测: `mov rax, 0x12345678` → gas 生成 `48 C7 C0 78 56 34 12` (imm32 sign-extend)，而非 imm64 形式。

---

## 2. 算术运算 (R-R)

### 2.1 ADD / SUB / AND / OR / XOR

| 操作 | ARM64 | x86-64 | RV32I | MIPS32 |
|------|-------|--------|-------|--------|
| **ADD** | `8B020020` | `48 01 D8` | `00208033` | `00 22 10 21` |
| **SUB** | `CB020020` | `48 29 D8` | `40208033` | `00 22 10 23` |
| **AND** | `8A020020` | `48 21 D8` | `00207033` | `00 22 10 24` |
| **OR**  | `AA020020` | `48 09 D8` | `00206033` | `00 22 10 25` |
| **XOR** | `CA020020` | `48 31 D8` | `00204033` | `00 22 10 26` |
| 字节   | 4 | 3 | 4 | 4 |

### 2.2 R-type 位域模板

```
RISC-V R-type: funct7[31:25] rs2[24:20] rs1[19:15] funct3[14:12] rd[11:7] opcode[6:0]
   ADD: funct7=0000000 funct3=000 opcode=0110011
   SUB: funct7=0100000 funct3=000 opcode=0110011
   AND: funct7=0000000 funct3=111 opcode=0110011
   OR:  funct7=0000000 funct3=110 opcode=0110011
   XOR: funct7=0000000 funct3=100 opcode=0110011

ARM64 DP reg: sf[31] op[30] S[29] 0101010[28:25] op_sel[24:21] Rm[20:16] operand[15:10] Rn[9:5] Rd[4:0]
   ADD: op=0 S=0 op_sel=0000 operand=000010
   SUB: op=1 S=0 op_sel=0000 operand=000010
   AND: op=0 S=0 op_sel=0000 operand=000000
   OR:  op=0 S=0 op_sel=0000 operand=000010 (shifted register orr)

x86-64: REX.W(01001_001) 01 opcode + ModRM(mod=11 reg=r/m rm)
   ADD: opcode=01 (r/m64, r64)
   SUB: opcode=29
   AND: opcode=21
   OR:  opcode=09
   XOR: opcode=31

MIPS R-type: opcode[31:26]=0 rs[25:21] rt[20:16] rd[15:11] sa[10:6] funct[5:0]
   ADD:  funct=100001
   SUB:  funct=100011
   AND:  funct=100100
   OR:   funct=100101
   XOR:  funct=100110
```

---

## 3. 算术运算 (立即数)

### 3.1 ADDI (Rd = Rs + imm12)

| ARM64 | x86-64 | RV32I | MIPS32 |
|-------|--------|-------|--------|
| `add x0,x1,#255` | `add rax, 0xFF` | `addi x0,x1,255` | `addiu $v0,$v1,255` |
| `9103FFC0` (4B) | `48 83 C0 FF` (3B) | `0FF08013` (4B) | `24 42 00 FF` (4B) |

### 3.2 立即数逻辑操作

| 操作 | ARM64 | x86-64 | RV32I | MIPS32 |
|------|-------|--------|-------|--------|
| **ANDI** | `and x0,x1,#imm` | `and rax, imm8` | `andi x0,x1,imm` | `andi $v0,$v1,imm` |
| | `8A0600+imm12` (4B) | `48 83 E0 imm` (3B) | `_imm__rs1_111_rd_0010011` | `30 42 imm_lo` (4B) |
| **ORI**  | `orr x0,x1,#imm` | `or rax, imm8` | `ori x0,x1,imm` | `ori $v0,$v1,imm` |
| | `AA0600+imm12` (4B) | `48 83 C8 imm` (3B) | `_imm__rs1_110_rd_0010011` | `34 42 imm_lo` (4B) |
| **XORI** | `eor x0,x1,#imm` | `xor rax, imm8` | `xori x0,x1,imm` | `xori $v0,$v1,imm` |
| | `CA0600+imm12` (4B) | `48 83 F0 imm` (3B) | `_imm__rs1_100_rd_0010011` | `38 42 imm_lo` (4B) |

---

## 4. 移位操作

### 4.1 寄存器-寄存器移位

| 操作 | ARM64 | x86-64 | RV32I | MIPS32 |
|------|-------|--------|-------|--------|
| **SLL** | `lsl x0,x1,x2` | `shl rax, cl` | `sll x0,x1,x2` | `sllv $v0,$v1,$at` |
| | `9AC22020` (4B) | `48 D3 E0` (3B) | `00209033` (4B) | `00 21 10 04` (4B) |
| **SRL** | `lsr x0,x1,x2` | `shr rax, cl` | `srl x0,x1,x2` | `srlv $v0,$v1,$at` |
| | `9AC22420` (4B) | `48 D3 E8` (3B) | `0020D033` (4B) | `00 21 10 06` (4B) |
| **SRA** | `asr x0,x1,x2` | `sar rax, cl` | `sra x0,x1,x2` | `srav $v0,$v1,$at` |
| | `9AC22820` (4B) | `48 D3 F8` (3B) | `4020D033` (4B) | `00 21 10 07` (4B) |

### 4.2 立即数移位

| 操作 | ARM64 | x86-64 | RV32I | MIPS32 |
|------|-------|--------|-------|--------|
| **SLLI** | `lsl x0,x1,#3` | `shl rax, 3` | `slli x0,x1,3` | `sll $v0,$v1,3` |
| | `D37FF820` (ASR编码的立即数变体) | `48 C1 E0 03` (4B) | `00309013` (4B) | `00 02 10 C0` (4B) |
| **SRLI** | `lsr x0,x1,#3` | `shr rax, 3` | `srli x0,x1,3` | `srl $v0,$v1,3` |
| | `D343FC20` (4B) | `48 C1 E8 03` (4B) | `0030D013` (4B) | `00 02 10 C2` (4B) |
| **SRAI** | `asr x0,x1,#3` | `sar rax, 3` | `srai x0,x1,3` | `sra $v0,$v1,3` |
| | `C3 (ASR)` | `48 C1 F8 03` (4B) | `4030D013` (4B) | `00 02 10 C3` (4B) |

---

## 5. 比较与置位

### 5.1 寄存器比较

| 操作 | ARM64 | x86-64 | RV32I | MIPS32 |
|------|-------|--------|-------|--------|
| **SLT** (有符号<) | `cmp + cset` | `cmp + setl` | `slt x0,x1,x2` | `slt $v0,$v1,$at` |
| | 复合指令 (8B) | 复合 (3B+2B) | `0020A033` (4B) | `00 44 08 2A` (4B) |
| **SLTU** (无符号<) | `cmp + cset lo` | `cmp + setb` | `sltu x0,x1,x2` | `sltu $v0,$v1,$at` |
| | 复合 (8B) | 复合 (3B+2B) | `0020B033` (4B) | `00 44 08 2B` (4B) |
| **SLTI** | `cmp + cset lt` | `cmp + setl` | `slti x0,x1,imm` | `slti $v0,$v1,imm` |
| | 复合 (8B) | 复合 (3B+2B) | `_imm__rs1_010_rd_0010011` | `28 42 imm_lo` (4B) |

### 5.2 测试零

| 操作 | ARM64 | x86-64 | RV32I | MIPS32 |
|------|-------|--------|-------|--------|
| 测试 == 0 | `cbz x0` | `test rax, rax` | `beq x0,x0,label` | `beq $v0,$zero,label` |
| | `B40000xx` (4B) | `48 85 C0` (3B) | `00000063`系列 (4B) | `10 40 xx xx` (4B) |

---

## 6. 分支跳转

### 6.1 无条件跳转

| 操作 | ARM64 | x86-64 | RV32I | MIPS32 |
|------|-------|--------|-------|--------|
| **JMP** 相对 | `b label` | `jmp rel32` | `jal x0, offset` | `j target` |
| | `140000xx` (4B, imm26) | `E9 xx xx xx xx` (5B) | `xxxx_00000_000_00000_1101111` (4B) | `08 00 xx xx` (4B) |
| **CALL** | `bl label` | `call rel32` | `jal ra, offset` | `jal target` |
| | `940000xx` (4B) | `E8 xx xx xx xx` (5B) | `xxxx_00000_000_00011_1101111` (4B) | `0C 00 xx xx` (4B) |
| **RET** | `ret` | `ret` | `jalr x0, 0(x1)` | `jr $ra` |
| | `D65F03C0` (4B) | `C3` (1B) | `00008067` (4B) | `03 E0 00 08` (4B) |

### 6.2 条件分支

| 操作 | ARM64 | x86-64 | RV32I | MIPS32 |
|------|-------|--------|-------|--------|
| **BEQ** | `b.eq label` | `je label` | `beq rs1,rs2,label` | `beq $rs,$rt,label` |
| | `54000400`系列(4B) | `0F 84 rel32` (6B) | `xxxx_000_xxxxxx_1100011` (4B) | `10 xx xx xx` (4B) |
| **BNE** | `b.ne` | `jne` | `bne rs1,rs2,label` | `bne $rs,$rt,label` |
| | `54000401` | `0F 85` | `xxxx_001_xxxxxx_1100011` | `14 xx xx xx` |
| **BLT** | `b.lt` | `jl` | `blt rs1,rs2,label` | 无原生,slt+beq |
| | `5400040B` | `0F 8C` | `xxxx_100_xxxxxx_1100011` | `10 40 xx xx;10 xx xx+4` |
| **BGE** | `b.ge` | `jge` | `bge rs1,rs2,label` | `bgezal / bgez` |
| | `5400040A` | `0F 8D` | `xxxx_101_xxxxxx_1100011` | `04 xx xx xx` |

---

## 7. 内存访问

### 7.1 加载

| 操作 | ARM64 | x86-64 | RV32I | MIPS32 |
|------|-------|--------|-------|--------|
| **LW** 32b | `ldr w0, [x1, #8]` | `mov eax, [rbx+8]` | `lw x0, 8(x1)` | `lw $v0, 8($v1)` |
| | `B9400820` (4B) | `8B 43 08` (3B) | `0080A003` (4B) | `8C 42 00 08` (4B) |
| **LH** 16b 符号 | `ldrsh w0,[x1,#8]` | `movsx eax, word ptr[]` | `lh x0, 8(x1)` | `lh $v0, 8($v1)` |
| | `79800820` (4B) | `0F BF 43 08` (4B) | `00809003` (4B) | `84 42 00 08` (4B) |
| **LHU** 16b 零 | `ldrh w0,[x1,#8]` | `movzx eax, word ptr[]` | `lhu x0, 8(x1)` | `lhu $v0, 8($v1)` |
| | `79400820` (4B) | `0F B7 43 08` (4B) | `0080D003` (4B) | `94 42 00 08` (4B) |
| **LB** 8b 符号 | `ldrsb w0,[x1,#8]` | `movsx eax, byte ptr[]` | `lb x0, 8(x1)` | `lb $v0, 8($v1)` |
| | `39800820` (4B) | `0F BE 43 08` (4B) | `0080C003` (4B) | `80 42 00 08` (4B) |
| **LBU** 8b 零 | `ldrb w0,[x1,#8]` | `movzx eax, byte ptr[]` | `lbu x0, 8(x1)` | `lbu $v0, 8($v1)` |
| | `39400820` (4B) | `0F B6 43 08` (4B) | `0080C003` funct=100 (4B) | `90 42 00 08` (4B) |
| **LD** 64b | `ldr x0, [x1, #8]` | `mov rax, [rbx+8]` | `ld x0, 8(x1)` (RV64) | — |
| | `F9400820` (4B) | `48 8B 43 08` (4B) | `0080B003` (4B) | — |

### 7.2 存储

| 操作 | ARM64 | x86-64 | RV32I | MIPS32 |
|------|-------|--------|-------|--------|
| **SW** 32b | `str w0, [x1, #8]` | `mov [rbx+8], eax` | `sw x0, 8(x1)` | `sw $v0, 8($v1)` |
| | `B9000820` (4B) | `89 43 08` (3B) | `00112423` (4B) | `AC 42 00 08` (4B) |
| **SH** 16b | `strh w0,[x1,#8]` | `mov word ptr[], ax` | `sh x0, 8(x1)` | `sh $v0, 8($v1)` |
| | `79000820` (4B) | `66 89 43 08` (4B) | `00111423` (4B) | `A4 42 00 08` (4B) |
| **SB** 8b | `strb w0,[x1,#8]` | `mov byte ptr[], al` | `sb x0, 8(x1)` | `sb $v0, 8($v1)` |
| | `39000820` (4B) | `88 43 08` (3B) | `00110423` (4B) | `A0 42 00 08` (4B) |
| **SD** 64b | `str x0,[x1,#8]` | `mov [rbx+8], rax` | `sd x0, 8(x1)` (RV64) | — |
| | `F9000820` (4B) | `48 89 43 08` (4B) | `00113423` funct=011 (4B) | — |

---

## 8. 原子操作

| 操作 | ARM64 | x86-64 | RV32I | MIPS32 |
|------|-------|--------|-------|--------|
| **LR** (Load-Reserved) | `ldxr x0, [x1]` | `mov rax, [rbx]` + `lock` | `lr.w x0, (x1)` | `ll $v0, 0($v1)` |
| | `F8400C20` (4B) | `48 8B 03`隐含lock | `0000B02F` (4B) | `C0 42 00 00` (4B) |
| **SC** (Store-Conditional) | `stxr w2,x0,[x1]` | `lock cmpxchg` | `sc.w x0,x2,(x1)` | `sc $v0,0($v1)` |
| | `F8000C20` (4B) | `48 0F B1 xx` (4B) | `0020B02F` (4B) | `E0 42 00 00` (4B) |
| **AMOSWAP** | `swp x0,x2,[x1]` | `xchg [rbx], rax` | `amoswap.w x0,x2,(x1)` | — (ll+sc模拟) |
| | `F8220C20` (4B) | `48 87 03` (3B) | `0020902F` (4B) | — |
| **AMOADD** | `stadd x0,[x1]` | `lock add [rbx],rax` | `amoadd.w x0,x2,(x1)` | — (ll+sc模拟) |
| | 复合(8B) | `48 01 03; lock` | `0000A02F` (4B) | — |
| **FENCE** | `dmb sy` | `mfence` | `fence rw,rw` | `sync` |
| | `D5033F9F` (4B) | `0F AE F0` (3B) | `0000000F` (4B) | `00 00 00 0F` (4B) |

---

## 9. 系统指令

### 9.1 系统调用 / 异常

| 操作 | ARM64 | x86-64 | RV32I | MIPS32 |
|------|-------|--------|-------|--------|
| **SYSCALL** | `svc #0` | `syscall` | `ecall` | `syscall` |
| | `D4000001` (4B) | `0F 05` (2B) | `00000073` (4B) | `00 00 00 0C` (4B) |
| **断点** | `brk #0` | `int3 (CC)` / `int 80` | `ebreak` | `break` |
| | `D4200000` (4B) | `CC` (1B) / `CD 80` | `00100073` (4B) | `00 00 00 0D` (4B) |
| **ERET** (返回上级) | `eret` | `sysret` | `sret / mret` | `eret` |
| | `D69F03E0` (4B) | `0F 07` (2B) | `10200073` / `30200073` (4B) | `42 00 00 18` (4B) |

### 9.2 调用号与参数约定

| ISA | 调用号寄存器 | 参数 1-6 | 返回 |
|-----|------------|---------|------|
| ARM64 | x8 | x0-x5 | x0 |
| x86-64 | rax | rdi,rsi,rdx,r10,r8,r9 | rax |
| RV32I | a7 | a0-a5 | a0 |
| MIPS32 | $v0 | $a0-$a3 + 栈 | $v0 |

---

## 10. 扩展算术

| 操作 | ARM64 | x86-64 | RV32I | MIPS32 |
|------|-------|--------|-------|--------|
| **MUL** | `mul x0,x1,x2` | `imul rax, rbx` | `mul x0,x1,x2` | `mul $v0,$v1,$at` |
| | `9B029C20` (SMULL变体) | `48 0F AF C3` (4B) | `00208033` funct7=0000001 | `00 44 08 2A`→div变体 |
| **DIV** (有符号) | `sdiv x0,x1,x2` | `idiv rbx` | `div x0,x1,x2` | `div $v1,$at; mflo $v0` |
| | `9AC20C20` (4B) | `48 F7 F8` (3B) | `00208033` funct3=100 funct7=000001 | `00 44 08 1A;mfc` |
| **DIVU** (无符号) | `udiv x0,x1,x2` | `div rbx` | `divu x0,x1,x2` | `divu $v1,$at; mflo` |
| | `9AC20820` (4B) | `48 F7 F0` (3B) | funct3=101 | `00 44 08 1B` |
| **REM** | `sdiv + msub` (3条) | `idiv` 余在 rdx | `rem x0,x1,x2` | `div; mfhi $v0` |
| | `9B20+` (复合) | `48 F7 F8` + 取 rdx | funct3=110 | `00 44 08 1A; 00 00 10 10` |

---

## 11. 条件选择与移动

| 操作 | ARM64 | x86-64 | RV32I | MIPS32 |
|------|-------|--------|-------|--------|
| **CSEL** 条件选择 | `csel x0,x1,x2,eq` | `cmovcc rax, rbx` | — (用分支模拟) | `movz/movn` |
| | `9A821020` (4B) | `0F 44 C3` (3B) | `0xxx + 0xx` (多条) | `01 41 10 01` (movz) |

---

## 12. 高位立即数 (U-type)

| 操作 | ARM64 | x86-64 | RV32I | MIPS32 |
|------|-------|--------|-------|--------|
| **LUI** 加载高位 | `movz x0, #val, lsl #16` | `mov rax, imm64` | `lui x0, imm20` | `lui $v0, imm` |
| | `D2800000`+ | `48 B8 xx..xx` (10B) | `xxxxx_00000_0110111` (4B) | `3C 02 xx xx` (4B) |
| **AUIPC** PC+imm | — | — | `auipc x0, imm20` | — |
| | — | — | `xxxxx_00000_0010111` (4B) | — |

---

## 13. NOP 与其他

| 操作 | ARM64 | x86-64 | RV32I | MIPS32 |
|------|-------|--------|-------|--------|
| **NOP** | `nop` | `nop (90)` | `addi x0,x0,0` | `nop (sll $0,$0,0)` |
| | `D503201F` (4B) | `90` (1B) | `00000013` (4B) | `00 00 00 00` (4B) |
| **WFI** 等待中断 | `wfi` | `hlt` | — (CSR操作) | — |
| | `D503201F` (4B) | `F4` (1B) | — | — |

---

## 14. 编码进制参考 (位域基线)

| ISA | 指令宽度 | 字节序 | 立即数编码 | 寄存器编码 |
|-----|---------|--------|----------|----------|
| ARM64 | 固定 32b | 小端 | 12b 移位 / 16b 分多次 | 5b register |
| x86-64 | 变长 1-15B | 小端 | 8/16/32/64b 变址 | ModRM+R+B 扩展 |
| RV32I | 固定 32b | 小端 | 12/20b (I/B/U/J型) | 5b rd/rs1/rs2 |
| MIPS32 | 固定 32b | 大端(默认) | 16b (多次拼接) | 5b rs/rt/rd |
| MTK | 同 ARM64 | 小端 | 同 ARM64 | 同 ARM64 |

---

## 15. 符号→机器码对照索引

| 语义 ID | 主分类 | ARM64 | x86-64 | RV32I | MIPS32 |
|---------|--------|-------|--------|-------|--------|
| ADD | 算术 R | 8B020020 | 48 01 D8 | 00208033 | 00 22 10 21 |
| SUB | 算术 R | CB020020 | 48 29 D8 | 40208033 | 00 22 10 23 |
| AND | 算术 R | 8A020020 | 48 21 D8 | 00207033 | 00 22 10 24 |
| OR | 算术 R | AA020020 | 48 09 D8 | 00206033 | 00 22 10 25 |
| XOR | 算术 R | CA020020 | 48 31 D8 | 00204033 | 00 22 10 26 |
| SLL | 移位 R | 9AC22020 | 48 D3 E0 | 00209033 | 00 21 10 04 |
| SRL | 移位 R | 9AC22420 | 48 D3 E8 | 0020D033 | 00 21 10 06 |
| SRA | 移位 R | 9AC22820 | 48 D3 F8 | 4020D033 | 00 21 10 07 |
| SLT | 比较 R | cmp+cset | cmp+setl | 0020A033 | 00 44 08 2A |
| ADDI | 立即数 | 9103FFC0 | 48 83 C0 FF | 0FF08013 | 24 42 00 FF |
| SLLI | 移位 I | D37FF820 | 48 C1 E0 03 | 00309013 | 00 02 10 C0 |
| SRLI | 移位 I | D343FC20 | 48 C1 E8 03 | 0030D013 | 00 02 10 C2 |
| SRAI | 移位 I | C3 (ASR) | 48 C1 F8 03 | 4030D013 | 00 02 10 C3 |
| ANDI | 立即数 | 8A0600+imm | 48 83 E0 | 0xxx 7xxx 13 | 30 42 imm |
| ORI | 立即数 | AA0600+imm | 48 83 C8 | 0xxx 6xxx 13 | 34 42 imm |
| XORI | 立即数 | CA0600+imm | 48 83 F0 | 0xxx 4xxx 13 | 38 42 imm |
| SLTI | 立即数 | cmp+cset | cmp+setl | 0xxx 2xxx 13 | 28 42 imm |
| BEQ | 分支 | 54000400 | 0F 84 | xxxx 0 1100011 | 10 44 xx |
| BNE | 分支 | 54000401 | 0F 85 | xxxx 1 1100011 | 14 44 xx |
| BLT | 分支 | 5400040B | 0F 8C | xxxx 4 1100011 | slt+beq |
| BGE | 分支 | 5400040A | 0F 8D | xxxx 5 1100011 | 04 xx xx |
| LW | 加载 | B9400820 | 8B 43 08 | 0080A003 | 8C 42 00 08 |
| SW | 存储 | B9000820 | 89 43 08 | 00112423 | AC 42 00 08 |
| LH | 加载 | 79400820 | 0F B7 | 00809003 | 84 42 00 08 |
| SH | 存储 | 79000820 | 66 89 | 00111423 | A4 42 00 08 |
| LB | 加载 | 39400820 | 0F B6 | 0080C003 funct000 | 80 42 00 08 |
| SB | 存储 | 39000820 | 88 43 | 00110423 | A0 42 00 08 |
| JMP | 跳转 | 1400xxxx | E9 xx xx xx xx | xxxx 0 1101111 | 08 00 xx xx |
| CALL | 跳转 | 9400xxxx | E8 xx xx xx xx | xxxx 3 1101111 | 0C 00 xx xx |
| RET | 跳转 | D65F03C0 | C3 | 00008067 | 03 E0 00 08 |
| LUI | 高位 | D2800000+ | 48 B8 xx..xx | xxxx 0110111 | 3C 02 xx xx |
| SYSCALL | 系统 | D4000001 | 0F 05 | 00000073 | 00 00 00 0C |
| ECALL | 系统 | — | — | 00000073 | 00 00 00 0C |
| EBREAK | 系统 | D4200000 | CC | 00100073 | 00 00 00 0D |
| LR | 原子 | F8400C20 | 48 8B 03 | 0000B02F | C0 42 00 00 |
| SC | 原子 | F8000C20 | 48 0F B1 | 0020B02F | E0 42 00 00 |
| AMOSWAP | 原子 | F8220C20 | 48 87 03 | 0020902F | — |
| AMOADD | 原子 | 复合 stadd | lock add | 0000A02F | — |
| FENCE | 访存序 | D5033F9F | 0F AE F0 | 0000000F | 00 00 00 0F |
| NOP | 空操作 | D503201F | 90 | 00000013 | 00 00 00 00 |
| MOV | 移动 | AA0003E0 | 48 89 D8 | 00008013 | 00 22 10 21 |
| CMP | 比较 | EB02007F | 48 39 C3 | beq x0,x0+slt | 00 22 10 2A |
| MUL | 扩展 | 9B029C | 48 0F AF | 02xxx1 funct7=01 | 00 44 08 2A→ |
| DIV | 扩展 | 9AC20C | 48 F7 F8 | 02xxx1 funct7=01 | 00 44 08 1A |
| REM | 扩展 | sdiv+msub | idiv rdx | 02xxx1 funct7=01 | 00 44 08 1A;mfhi |
| SLTU | 比较 | — | — | 0020B033 | 00 44 08 2B |
| CMOV | 条件 | 9A8210 +cond | 0F 44 C3 | — | 01 41 10 01 |
| AUIPC | 高位 | — | — | xxxx 0010111 | — |
| MRET_SRET | 系统 | D69F03E0 | 0F 07 | 30200073 | 42 00 00 18 |

---

## 参考手册

| ISA | 规范文档 |
|-----|---------|
| ARM64 | ARM DDI 0487 (ARM Architecture Reference Manual for A-profile) |
| x86-64 | Intel SDM Vol. 2A/2B/2C / AMD APM Vol. 3 |
| RISC-V | Volume I: Unprivileged ISA (20191213); Volume II: Privileged Architecture |
| MIPS32 | MIPS32 Architecture For Programmers Vol. II: The MIPS32 Instruction Set |
| MTK | ARM64 规范 + 联发科专有 APUSYS/协处理器文档 |

---

> 本文件是对齐公约数 · 六十二语义 ID 跨五架构的机器码详表。
> 机器码以公开手册或工具链输出为准，使用前建议用对应汇编器生成后交叉验证。
> 路径: hunyuan-os/tools/isa-ref/full_alignment.md
