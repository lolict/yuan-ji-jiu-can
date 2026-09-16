# 机器码容器规范 · Machine Container Spec

> 把机器码装进字体容器 · 让指令即容器 · 让容器即字体

---

## 一、设计原则

1. **容器第一存在单位** — 机器码不是数值，是容器
2. **语义 ID 跨平台对齐** — 不同 ISA 共享同一语义 ID
3. **64位单元编码** — 每个容器坐标/极性/主权/价值四位一体
4. **字体文件即仓库** — TTF 文件存储所有容器映射
5. **仅加减运算** — 容器拼接只用加法

---

## 二、容器结构（64位）

```
┌────────────┬────────────┬────────────┬────────────┐
│  坐标(16)  │  极性(8)   │  主权(8)   │  价值(32)  │
│  platform  │ polarity   │ ownership  │   value    │
│  ISA 编码   │ 主/外/中性  │ 公有/私有   │ 机器码本体  │
└────────────┴────────────┴────────────┴────────────┘
```

- **坐标(16位)**: ISA 平台标识
  - `0x0001` = RISC-V
  - `0x0002` = ARM64
  - `0x0003` = x86-64
  - `0x0004` = MIPS
- **极性(8位)**: `0x00`=外人, `0x01`=主人, `0xFF`=中性
- **主权(8位)**: `0x00`=私有, `0x01`=公有
- **价值(32位)**: 机器码本体（RISC-V 指令为 32 位）

---

## 三、语义 ID 表（跨平台统一）

| 语义 ID | 操作 | RISC-V 助记符 | ARM64 助记符 | x86-64 助记符 | MIPS 助记符 |
|---------|------|---------------|--------------|---------------|-------------|
| ADD | 寄存器加法 | `add` | `add` | `add` | `add` |
| SUB | 寄存器减法 | `sub` | `sub` | `sub` | `sub` |
| AND | 按位与 | `and` | `and` | `and` | `and` |
| OR | 按位或 | `or` | `orr` | `or` | `or` |
| XOR | 按位异或 | `xor` | `eor` | `xor` | `xor` |
| SLT | 有符号比较 | `slt` | `cmp`+`cset` | `cmp`+`setl` | `slt` |
| SLTU | 无符号比较 | `sltu` | `cmp`+`cset` | `cmp`+`setb` | `sltu` |
| SLL | 逻辑左移 | `sll` | `lsl` | `shl` | `sll` |
| SRL | 逻辑右移 | `srl` | `lsr` | `shr` | `srl` |
| SRA | 算术右移 | `sra` | `asr` | `sar` | `sra` |
| ADDI | 立即数加法 | `addi` | `add` | `add` | `addi` |
| LW | 加载字 | `lw` | `ldr` | `mov` | `lw` |
| SW | 存储字 | `sw` | `str` | `mov` | `sw` |
| BEQ | 相等分支 | `beq` | `cmp`+`beq` | `cmp`+`je` | `beq` |
| JAL | 跳转并链接 | `jal` | `bl` | `call` | `jal` |
| LUI | 加载高位 | `lui` | `movz` | `mov` | `lui` |
| AUIPC | PC 加立即数 | `auipc` | `adr` | `lea` | `lui`+`ori` |

---

## 四、字体文件内 MCOD 表结构

```
MCOD 表 (SFNT Tag: "MCOD")
├── version:        UInt16    版本号 (1)
├── entryCount:     UInt16    条目数
├── entries:        entry[]   条目数组
│   ├── semanticId: UInt16    语义 ID 索引
│   ├── platform:   UInt16    平台编码 (同坐标字段)
│   ├── machineCode: UInt32   机器码本体
│   ├── format:     UInt8     指令格式 (R/I/S/B/U/J)
│   ├── opcode:     UInt8     操作码
│   ├── funct3:     UInt8     功能码 3
│   ├── funct7:     UInt8     功能码 7
│   └── operandCnt: UInt8     操作数数量
```

---

## 五、工作流程

### 编码：语义 ID + 操作数 → 机器码容器

```
输入: ADD, [r1, r2, r3]
  → 查表得 ADD 语义模板 (opcode=0110011, funct3=000, funct7=0000000)
  → 从容器字体读 RISC-V 坐标 (0x0001)
  → 加法拼接: opcode | rs2<<20 | rs1<<15 | funct3<<12 | rd<<7 | funct7<<25
  → 输出: 容器(坐标=0x0001, 极性=0x01, 主权=0x01, 价值=0x003100B3)
```

### 解码：机器码 → 语义 ID + 操作数

```
输入: 0x003100B3
  → 提取 opcode[6:0] = 0110011, funct3[14:12] = 000, funct7[31:25] = 0000000
  → 查 MCOD 表: (opcode=0110011, funct3=000, funct7=0000000) → ADD
  → 输出: ADD, rd=x1, rs1=x2, rs2=x3
```

---

## 六、指令格式（RISC-V）

| 格式 | 位域布局 |
|------|----------|
| R | `funct7[31:25] \| rs2[24:20] \| rs1[19:15] \| funct3[14:12] \| rd[11:7] \| opcode[6:0]` |
| I | `imm[31:20] \| rs1[19:15] \| funct3[14:12] \| rd[11:7] \| opcode[6:0]` |
| S | `imm[31:25] \| rs2[24:20] \| rs1[19:15] \| funct3[14:12] \| imm[11:7] \| opcode[6:0]` |
| B | `imm[12]\|imm[10:5] \| rs2[24:20] \| rs1[19:15] \| funct3[14:12] \| imm[4:1]\|imm[11] \| opcode[6:0]` |
| U | `imm[31:12] \| rd[11:7] \| opcode[6:0]` |
| J | `imm[20]\|imm[10:1]\|imm[11]\|imm[19:12] \| rd[11:7] \| opcode[6:0]` |

---

*维护者: lolict + 刘楚恬*
*版本: 1.0.0*
*状态: 进行中*
