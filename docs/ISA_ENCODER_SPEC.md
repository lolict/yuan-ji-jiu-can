# ISA 编码层 · RISC-V 多扩展 + 跨平台语义编码器

> 创建: 2026-09-04 · 更新: 2026-09-21
> 工具编号: hunyuan-rs/src/isa.rs (+ isa16.rs) · vocabulary.rs (跨平台)
> 算法参考: RISC-V Volume I Unprivileged ISA 20191213 + Extensions M/A/C

---

## 1. 设计目标

"最小可运行原型"已跨越 Phase 0 ~ Phase 4 (部分):
- RV32I 基础 32 条指令
- RV64I 字宽扩展
- M 扩展 8 条乘除指令
- A 扩展 20 条原子指令
- 16 位 C 扩展压缩指令
- **Phase 4 新增**: 跨平台语义编码 vocabulary.rs — SemanticId ↔ 四平台原生机器码

统一语义 ID 映射到 RISC-V / x86-64 / ARM64 / MIPS32:

| SemanticId | RISC-V | ARM64 | x86-64 | MIPS32 |
|------------|--------|-------|--------|--------|
| Add | add | ADD 9100.. | 48 01  regs | 0000..100000 |
| Sub | sub | SUB d100.. | 48 29 .. | ..100010 |
| Mul | mul (funct7=1) | MADD | 0F AF | MULT+MFLO |
| Div | div (funct7=1) | SDIV/UDIV | F7 /7 | DIV |
| Rem | rem (funct7=1) | MSUB (rd=temp) | IDIV+MOV RAX,RDX | DIV+MFHI |
| Addw | addw (OP-32) | ADDW | .. | ADD |
| Addiw | addiw (OP-IMM-32) | ADDIW | 48 83 /0 | ADDI |
| Lrw | lr.w | LDAXR | LFENCE | LL |
| Scw | sc.w | STLXR | SFENCE | SC |
| AmoSwapw | amoswap.w (LOCK XCHG) | LOCK XCHG | .. | LL占位 |
| AmoAddw | amoadd.w (LOCK XADD) | LOCK XADD | .. | LL占位 |

三层 API: 构造器 / 字符串汇编编解码 / 编码字段枚举 (ROp/IOp/LoadOp/StoreOp/BranchOp/ROp32/IOp32/MOp/AOp)

---

## 2. 编码约定

- 定长 4 字节 (标准扩展),小端 字节序
- funct7=0000001 在 opcode=OP 时标识 M 扩展
- RV64I 字宽指令使用新 opcode: OP-32 (0111011), OP-IMM-32 (0011011)
- A 扩展 opcode=0101111, funct5 区分子操作, funct3=010/011 区分 32/64 位

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

| funct5 | 操作 |
|--------|------|
| 00010 | lr.w / lr.d |
| 00011 | sc.w / sc.d |
| 00001 | amoswap.w/d |
| 00000 | amoadd.w/d |
| ... | amoxor/amoand/amoor/amomin/amomax/amominu/amomaxu |

---

## 4. 构造器 API

```rust
// RISC-V
Insn::r(IOp, rd, rs1, rs2)        // R-type
Insn::i(IOp, rd, rs1, imm)         // I-type
Insn::load(LoadOp, rd, rs1, off)
Insn::store(StoreOp, rs2, rs1, off)
Insn::branch(BranchOp, rs1, rs2, off)
Insn::lui(rd, imm) / Insn::auipc(rd, imm)
Insn::jal(rd, off) / Insn::jalr(rd, rs1, off)
Insn::r32(ROp32, rd, rs1, rs2)     // OP-32
Insn::i32(IOp32, rd, rs1, shamt)   // OP-IMM-32
Insn::m_op(MOp, rd, rs1, rs2)      // M 扩展
Insn::amo(AOp, rd, rs1, rs2)       // A 扩展

// 字符串编解码
Insn::encode("add x1, x2, x3") -> Option<Insn>
Insn(0x23100B3).decode()

// 跨平台语义
let word = VocabWord::new(SemanticId::Mul, ops6, Platform::ARM64);
let machine_code: Vec<u8> = word.native_encoding;
```

---

## 5. 测试覆盖

- isa.rs: 33 单元测试, + 11 isa16.rs C 扩展
- vocabulary.rs: ~~26~~ 32 跨平台测试 (新增 riscv_m_ext_all_nonempty / arm64_m_ext_all_nonempty / x86_m_ext_all_nonempty / mips_m_ext_all_nonempty / riscv_a_ext_all_nonempty / a_ext_cross_platform)
- 跨平台验证: Add/Sub/And/Or/Xor/Addi 在四平台 round-trip; Mul/Div/Rem 在 RISC-V/ARM64/x86/MIPS 全部产出非空编码

运行: `cargo test --lib cross_isa::vocabulary` · `cargo test --lib isa`

---

## 6. CLI

```shell
cargo run -p mqf-isa --bin isa-encode -- encode "mul x1, x2, x3"
cargo run -p mqf-isa --bin isa-encode -- decode 023100b3
cargo run -p mqf-isa --bin isa-encode -- demo
cargo run -p mqf-isa --bin isa-encode -- test
```

---

## 7. 下一步

- [ ] C 扩展补完 (c.addi4spn / c.andi / c.sub / c.jr 全部形式)
- [ ] 与 MCOD 字体管道联动: 语义ID → 四平台原生码 → 字形轮廓
- [ ] x86-64/ARM64 编码精度提升 (ModRM + SIB 字节处理, 完整指令长度计算)
- [ ] 自举尝试: 简单汇编器 → 链接器 pipeline

END OF SPEC
