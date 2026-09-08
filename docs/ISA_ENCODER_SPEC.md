# ISA 编码层 · RISC-V 多扩展编码器/解码器

> 创建: 2026-09-04 · 更新: 2026-09-08
> 工具编号: hunyuan-rs/src/isa.rs (+ isa16.rs)
> 算法参考: RISC-V Volume I Unprivileged ISA 20191213 + Extensions M/A/C

---

## 1. 设计目标

"最小可运行原型"已跨越 Phase 0 ~ Phase 2：
- 覆盖 RV32I 基础 32 条指令
- 覆盖 RV64I 字宽扩展 (addw/addiw/sllw/srlw/sraw/slliw/srliw/sraiw + ld/lwu/sd)
- 覆盖 M 扩展 8 条乘除指令
- 覆盖 A 扩展 20 条原子指令 (LR/SC + 8 种 AMO × {W,D})
- 16 位 C 扩展压缩指令 (c.lw/c.sw/c.addi/c.li/c.lui/c.jr/c.jalr/c.ebreak/c.mv/c.add/c.slli/c.lwsp/c.swsp/c.j)

三层 API: 构造器 / 字符串汇编编解码 / 编码字段枚举 (ROp/IOp/LoadOp/StoreOp/BranchOp/ROp32/IOp32/MOp/AOp)

---

## 2. 编码约定

- 定长 4 字节 (标准扩展),小端
- 符号扩展: I-type 12 位立即数有符号扩展; B/J-type 重组位字段为无符号偏移量再符号扩展
- funct7=0000001 在 opcode=OP 时标识 M 扩展
- RV64I 字宽指令使用新 opcode: OP-32 (0111011), OP-IMM-32 (0011011)
- A 扩展使用 opcode 0101111,funct5 区分子操作,funct3=010/011 区分 32/64 位

---

## 3. 各扩展编码明细

### 3.1 RV64I 字宽 (opcode 0111011 和 0011011)

| 指令 | funct3 | funct7 | 备注 |
|------|--------|--------|------|
| addw | 000 | 0000000 | 结果符号扩展到 64 位 |
| subw | 000 | 0100000 | |
| sllw | 001 | 0000000 | |
| srlw | 101 | 0000000 | |
| sraw | 101 | 0100000 | |
| addiw | 000 | 0000000 | 立即数 12 位有符号 |
| slliw | 001 | 0000000 | shamt 5 位 |
| srliw | 101 | 0000000 | |
| sraiw | 101 | 0100000 | |

Load/Store 在原有 opcode 基础上新增 funct3:
- ld: LOAD funct3=011
- lwu: LOAD funct3=110
- sd: STORE funct3=011

### 3.2 M 扩展 (opcode 0110011 + funct7=0000001)

| 指令 | funct3 | 语义 |
|------|--------|------|
| mul | 000 | 低位乘法 |
| mulh | 001 | 高位有符号 × 有符号 |
| mulhsu | 010 | 高位有符号 × 无符号 |
| mulhu | 011 | 高位无符号 |
| div | 100 | 有符号除法 |
| divu | 101 | 无符号除法 |
| rem | 110 | 有符号余数 |
| remu | 111 | 无符号余数 |

### 3.3 A 扩展 (opcode 0101111)

格式: funct5[31:27] | aq(26) | rl(25) | rs2[24:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]

| funct5 | 操作 | 助记符 |
|--------|------|--------|
| 00010 | 加载预留 | lr.w / lr.d |
| 00011 | 条件存储 | sc.w / sc.d |
| 00001 | 交换 | amoswap.w/d |
| 00000 | 加法 | amoadd.w/d |
| 00100 | 异或 | amoxor.w/d |
| 01100 | 与 | amoand.w/d |
| 01000 | 或 | amoor.w/d |
| 10000 | 有符号最小 | amomin.w/d |
| 10100 | 有符号最大 | amomax.w/d |
| 11000 | 无符号最小 | amominu.w/d |
| 11100 | 无符号最大 | amomaxu.w/d |

funct3=010 表示 32 位操作,011 表示 64 位。

---

## 4. 编码构造器 API

```rust
// RV32I
Insn::r(ROp, rd, rs1, rs2)       // R-type
Insn::i(IOp, rd, rs1, imm)        // I-type
Insn::load(LoadOp, rd, rs1, off)  // Load
Insn::store(StoreOp, rs2, rs1, off) // Store
Insn::branch(BranchOp, rs1, rs2, off) // Branch
Insn::lui(rd, imm) / Insn::auipc(rd, imm)
Insn::jal(rd, off) / Insn::jalr(rd, rs1, off)

// RV64I
Insn::r32(ROp32, rd, rs1, rs2)    // OP-32
Insn::i32(IOp32, rd, rs1, shamt)  // OP-IMM-32

// M 扩展
Insn::m_op(MOp, rd, rs1, rs2)

// A 扩展
Insn::amo(AOp, rd, rs1, rs2)

// 字符串编解码
Insn::encode("add x1, x2, x3") -> Option<Insn>
Insn(0x003100B3).decode() -> String
Insn(0x003100B3).hex() -> String
```

---

## 5. 测试覆盖

- isa.rs: 33 个单元测试,含 12 个新增 RV64I/M/A 测试
- isa16.rs: 11 个 C 扩展测试
- 权威数据源: RISC-V Unprivileged ISA 手册 + riscv.md 内部参照文档

运行: `cargo test --lib isa`

---

## 6. CLI 工具

```shell
cargo run --bin isa-encode -- encode "add x1, x2, x3"
cargo run --bin isa-encode -- decode 003100b3
cargo run --bin isa-encode -- demo
cargo run --bin isa-encode -- test
```

---

## 7. 下一步

- [ ] C 扩展增加更多指令 (c.addi4spn / c.andi / c.sub / c.xor / c.or / c.and / c.j / c.beqz / c.bnez)
- [ ] 与字体封装工具联动: 符号→机器码→字体码点映射
- [ ] 添加 x86-64/ARM64/MIPS 编码表,按功能对齐到同一语义 ID
- [ ] 自举尝试: 用本编码器引导出简单的汇编器→链接器 pipeline

END OF SPEC
