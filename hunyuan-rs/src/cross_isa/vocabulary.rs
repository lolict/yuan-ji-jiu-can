// 词汇层 — 统一语义ID定义
// 不同架构的机器码通过语义ID对齐到同一个"词汇表"

/// 统一语义ID (X轴, 8位)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SemanticId {
    // ══ 基础 RV32I ══
    Load  = 0x01,
    Store = 0x02,
    Add   = 0x03,
    Addi  = 0x04,
    Sub   = 0x05,
    And   = 0x06,
    Or    = 0x07,
    Xor   = 0x08,
    Jmp   = 0x09,
    Bne   = 0x0A,
    Beq   = 0x0B,
    Call  = 0x0C,
    Ret   = 0x0D,
    Cmp   = 0x0E,
    Mov   = 0x0F,
    // ══ M 扩展 · 乘除 (funct7=00000001) ══
    Mul   = 0x10,
    Mulh  = 0x11,
    Div   = 0x12,
    Divu  = 0x13,
    Rem   = 0x14,
    Remu  = 0x15,
    // ══ RV64I · 字宽 (OP-32 / OP-IMM-32) ══
    Addw  = 0x16,
    Addiw = 0x17,
    Mulw  = 0x18,
    Divw  = 0x19,
    Remw  = 0x1A,
    // ══ A 扩展 · 原子 (opcode=0101111) ══
    Lrw      = 0x1B,
    Scw      = 0x1C,
    AmoSwapw = 0x1D,
    AmoAddw  = 0x1E,
}

impl SemanticId {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0x01 => Some(Self::Load),
            0x02 => Some(Self::Store),
            0x03 => Some(Self::Add),
            0x04 => Some(Self::Addi),
            0x05 => Some(Self::Sub),
            0x06 => Some(Self::And),
            0x07 => Some(Self::Or),
            0x08 => Some(Self::Xor),
            0x09 => Some(Self::Jmp),
            0x0A => Some(Self::Bne),
            0x0B => Some(Self::Beq),
            0x0C => Some(Self::Call),
            0x0D => Some(Self::Ret),
            0x0E => Some(Self::Cmp),
            0x0F => Some(Self::Mov),
            0x10 => Some(Self::Mul),
            0x11 => Some(Self::Mulh),
            0x12 => Some(Self::Div),
            0x13 => Some(Self::Divu),
            0x14 => Some(Self::Rem),
            0x15 => Some(Self::Remu),
            0x16 => Some(Self::Addw),
            0x17 => Some(Self::Addiw),
            0x18 => Some(Self::Mulw),
            0x19 => Some(Self::Divw),
            0x1A => Some(Self::Remw),
            // 0x18-0x1A: Mulw/Divw/Remw (已启用)
            0x1B => Some(Self::Lrw),
            0x1C => Some(Self::Scw),
            0x1D => Some(Self::AmoSwapw),
            0x1E => Some(Self::AmoAddw),
            _ => None,
        }
    }

    pub fn initial(&self) -> &'static str {
        match self {
            Self::Add | Self::Addi | Self::Addw | Self::Addiw => "加",
            Self::Sub => "减",
            Self::Mul | Self::Mulh | Self::Mulw | Self::Div | Self::Divu | Self::Divw | Self::Rem | Self::Remu | Self::Remw => "乘",
            Self::And | Self::Or | Self::Xor => "合",
            Self::Load | Self::Store | Self::Lrw | Self::Scw | Self::AmoSwapw | Self::AmoAddw => "传",
            Self::Jmp | Self::Bne | Self::Beq | Self::Call | Self::Ret => "移",
            Self::Cmp => "较",
            Self::Mov => "载",
        }
    }

    pub fn vowel(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperandType { RegReg, RegImm, RegMem, MemReg, NoOperand }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Platform { X86_64 = 0, ARM64 = 1, RiscV = 2, MIPS32 = 3 }

impl Platform {
    pub fn name(&self) -> &'static str {
        match self {
            Self::X86_64 => "x86-64",
            Self::ARM64  => "ARM64",
            Self::RiscV  => "RISC-V",
            Self::MIPS32 => "MIPS32",
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operand { Reg(u8), Imm(i64), Mem { base: u8, offset: i64 }, Label(u32) }

impl Operand {
    pub fn reg(&self) -> Option<u8> { match self { Operand::Reg(n) => Some(*n), _ => None } }
    pub fn imm(&self) -> Option<i64> { match self { Operand::Imm(n) => Some(*n), _ => None } }
    pub fn label(&self) -> Option<u32> { match self { Operand::Label(n) => Some(*n), _ => None } }
    pub fn mem(&self) -> Option<(u8, i64)> { match self { Operand::Mem { base, offset } => Some((*base, *offset)), _ => None } }
}

#[derive(Debug, Clone)]
pub struct VocabWord {
    pub semantic_id: SemanticId,
    pub operands: Vec<Operand>,
    pub platform: Platform,
    pub native_encoding: Vec<u8>,
}

impl VocabWord {
    pub fn new(semantic_id: SemanticId, operands: Vec<Operand>, platform: Platform) -> Self {
        let native_encoding = Self::encode_native(semantic_id, &operands, platform);
        Self { semantic_id, operands, platform, native_encoding }
    }

    fn encode_native(id: SemanticId, ops: &[Operand], plat: Platform) -> Vec<u8> {
        match plat {
            Platform::RiscV  => Self::enc_riscv(id, ops),
            Platform::ARM64  => Self::enc_arm64(id, ops),
            Platform::X86_64 => Self::enc_x86(id, ops),
            Platform::MIPS32 => Self::enc_mips(id, ops),
        }
    }

    fn enc_riscv(id: SemanticId, ops: &[Operand]) -> Vec<u8> {
        use crate::isa::{Insn, Reg, ROp, ROp32, IOp, IOp32, LoadOp, StoreOp, BranchOp, MOp, AOp};
        let insn = match id {
            SemanticId::Add if ops.len() >= 3 => {
                match (ops[0].reg(), ops[1].reg(), ops[2].reg()) {
                    (Some(rd), Some(r1), Some(r2)) => Insn::r(ROp::Add, Reg(rd), Reg(r1), Reg(r2)),
                    _ => return vec![],
                }
            }
            SemanticId::Sub if ops.len() >= 3 => {
                match (ops[0].reg(), ops[1].reg(), ops[2].reg()) {
                    (Some(rd), Some(r1), Some(r2)) => Insn::r(ROp::Sub, Reg(rd), Reg(r1), Reg(r2)),
                    _ => return vec![],
                }
            }
            SemanticId::And if ops.len() >= 3 => {
                match (ops[0].reg(), ops[1].reg(), ops[2].reg()) {
                    (Some(rd), Some(r1), Some(r2)) => Insn::r(ROp::And, Reg(rd), Reg(r1), Reg(r2)),
                    _ => return vec![],
                }
            }
            SemanticId::Or if ops.len() >= 3 => {
                match (ops[0].reg(), ops[1].reg(), ops[2].reg()) {
                    (Some(rd), Some(r1), Some(r2)) => Insn::r(ROp::Or, Reg(rd), Reg(r1), Reg(r2)),
                    _ => return vec![],
                }
            }
            SemanticId::Xor if ops.len() >= 3 => {
                match (ops[0].reg(), ops[1].reg(), ops[2].reg()) {
                    (Some(rd), Some(r1), Some(r2)) => Insn::r(ROp::Xor, Reg(rd), Reg(r1), Reg(r2)),
                    _ => return vec![],
                }
            }
            SemanticId::Addi if ops.len() >= 3 => {
                match (ops[0].reg(), ops[1].reg(), ops[2].imm()) {
                    (Some(rd), Some(r1), Some(imm)) => Insn::i(IOp::Addi, Reg(rd), Reg(r1), imm as i32),
                    _ => return vec![],
                }
            }
            SemanticId::Load if ops.len() >= 2 => {
                let rd = match ops[0].reg() { Some(v) => Reg(v), None => return vec![] };
                match ops[1].mem() {
                    Some((base, offset)) => Insn::load(LoadOp::Lw, rd, Reg(base), offset as i32),
                    None => return vec![],
                }
            }
            SemanticId::Store if ops.len() >= 2 => {
                let rs = match ops[0].reg() { Some(v) => Reg(v), None => return vec![] };
                match ops[1].mem() {
                    Some((base, offset)) => Insn::store(StoreOp::Sw, rs, Reg(base), offset as i32),
                    None => return vec![],
                }
            }
            SemanticId::Bne if ops.len() >= 3 => {
                match (ops[0].reg(), ops[1].reg(), ops[2].label()) {
                    (Some(r1), Some(r2), Some(off)) => Insn::branch(BranchOp::Bne, Reg(r1), Reg(r2), off as i32),
                    _ => return vec![],
                }
            }
            SemanticId::Jmp if ops.len() >= 1 => {
                match ops[0].label() {
                    Some(off) => Insn::jal(Reg(0), off as i32),
                    None => return vec![],
                }
            }
            SemanticId::Call if ops.len() >= 1 => {
                match ops[0].imm() {
                    Some(off) => Insn::jal(Reg(1), off as i32),
                    None => return vec![],
                }
            }
            SemanticId::Ret => Insn(0x00008067),
            SemanticId::Cmp if ops.len() >= 3 => {
                match (ops[0].reg(), ops[1].reg(), ops[2].reg()) {
                    (Some(rd), Some(r1), Some(r2)) => Insn::r(ROp::Slt, Reg(rd), Reg(r1), Reg(r2)),
                    _ => return vec![],
                }
            }
            SemanticId::Mov if ops.len() >= 2 => {
                match (ops[0].reg(), ops[1].imm()) {
                    (Some(rd), Some(imm)) => Insn::i(IOp::Addi, Reg(rd), Reg(0), imm as i32),
                    _ => return vec![],
                }
            }
            // ── M 扩展 · R-type funct7=1 ──
            SemanticId::Mul | SemanticId::Mulh | SemanticId::Div | SemanticId::Divu |
            SemanticId::Rem | SemanticId::Remu if ops.len() >= 3 => {
                let (rd, r1, r2) = match (ops[0].reg(), ops[1].reg(), ops[2].reg()) {
                    (Some(a), Some(b), Some(c)) => (Reg(a), Reg(b), Reg(c)),
                    _ => return vec![],
                };
                let mop = match id {
                    SemanticId::Mul => MOp::Mul,
                    SemanticId::Mulh => MOp::Mulh,
                    SemanticId::Div => MOp::Div,
                    SemanticId::Divu => MOp::Divu,
                    SemanticId::Rem => MOp::Rem,
                    SemanticId::Remu => MOp::Remu,
                    _ => return vec![],
                };
                Insn::m_op(mop, rd, r1, r2)
            }
            // ── RV64I 字宽 · OP-32 / OP-IMM-32 ──
            SemanticId::Addw if ops.len() >= 3 => {
                match (ops[0].reg(), ops[1].reg(), ops[2].reg()) {
                    (Some(rd), Some(r1), Some(r2)) => Insn::r32(ROp32::Addw, Reg(rd), Reg(r1), Reg(r2)),
                    _ => return vec![],
                }
            }
            SemanticId::Addiw if ops.len() >= 3 => {
                match (ops[0].reg(), ops[1].reg(), ops[2].imm()) {
                    (Some(rd), Some(r1), Some(imm)) => Insn::i32(IOp32::Addiw, Reg(rd), Reg(r1), imm as i32),
                    _ => return vec![],
                }
            }
            // ── A 扩展原子 · LR/SC ──
            SemanticId::Lrw if ops.len() >= 2 => {
                // lr.w rd, (rs1)  →  rd=ops[0], rs1=ops[1]
                match (ops[0].reg(), ops[1].reg()) {
                    (Some(rd), Some(r1)) => Insn::amo(AOp::LrW, Reg(rd), Reg(r1), Reg(0)),
                    _ => return vec![],
                }
            }
            SemanticId::Scw if ops.len() >= 3 => {
                // sc.w rd, rs2, (rs1)  →  rd=ops[0], rs2=ops[1], rs1=ops[2]
                match (ops[0].reg(), ops[1].reg(), ops[2].reg()) {
                    (Some(rd), Some(rs2), Some(r1)) => Insn::amo(AOp::ScW, Reg(rd), Reg(r1), Reg(rs2)),
                    _ => return vec![],
                }
            }
            SemanticId::AmoSwapw | SemanticId::AmoAddw if ops.len() >= 3 => {
                let (rd, rs2, r1) = match (ops[0].reg(), ops[1].reg(), ops[2].reg()) {
                    (Some(a), Some(b), Some(c)) => (Reg(a), Reg(b), Reg(c)),
                    _ => return vec![],
                };
                let aop = match id {
                    SemanticId::AmoSwapw => AOp::AmoSwapW,
                    SemanticId::AmoAddw => AOp::AmoAddW,
                    _ => return vec![],
                };
                Insn::amo(aop, rd, r1, rs2)
            }
            _ => return vec![],
        };
        insn.0.to_le_bytes().to_vec()
    }

    fn enc_arm64(id: SemanticId, ops: &[Operand]) -> Vec<u8> {
        let code: u32 = match id {
            SemanticId::Add if ops.len() >= 3 => {
                let rd = match ops[0].reg() { Some(v) => v as u32, None => return vec![] };
                let rn = match ops[1].reg() { Some(v) => v as u32, None => return vec![] };
                let rm = match ops[2].reg() { Some(v) => v as u32, None => return vec![] };
                (0b10001011 << 24) | (rm << 16) | (rn << 5) | rd
            }
            SemanticId::Sub if ops.len() >= 3 => {
                let rd = match ops[0].reg() { Some(v) => v as u32, None => return vec![] };
                let rn = match ops[1].reg() { Some(v) => v as u32, None => return vec![] };
                let rm = match ops[2].reg() { Some(v) => v as u32, None => return vec![] };
                (0b11001011 << 24) | (rm << 16) | (rn << 5) | rd
            }
            SemanticId::And if ops.len() >= 3 => {
                let rd = match ops[0].reg() { Some(v) => v as u32, None => return vec![] };
                let rn = match ops[1].reg() { Some(v) => v as u32, None => return vec![] };
                let rm = match ops[2].reg() { Some(v) => v as u32, None => return vec![] };
                (0b10001010 << 24) | (rm << 16) | (rn << 5) | rd
            }
            SemanticId::Or if ops.len() >= 3 => {
                let rd = match ops[0].reg() { Some(v) => v as u32, None => return vec![] };
                let rn = match ops[1].reg() { Some(v) => v as u32, None => return vec![] };
                let rm = match ops[2].reg() { Some(v) => v as u32, None => return vec![] };
                (0b10101010 << 24) | (rm << 16) | (rn << 5) | rd
            }
            SemanticId::Xor if ops.len() >= 3 => {
                let rd = match ops[0].reg() { Some(v) => v as u32, None => return vec![] };
                let rn = match ops[1].reg() { Some(v) => v as u32, None => return vec![] };
                let rm = match ops[2].reg() { Some(v) => v as u32, None => return vec![] };
                (0b11001010 << 24) | (rm << 16) | (rn << 5) | rd
            }
            SemanticId::Addi if ops.len() >= 3 => {
                let rd = match ops[0].reg() { Some(v) => v as u32, None => return vec![] };
                let rn = match ops[1].reg() { Some(v) => v as u32, None => return vec![] };
                let imm = match ops[2].imm() { Some(v) => (v as u32) & 0xFFF, None => return vec![] };
                (0b10010001 << 24) | (imm << 10) | (rn << 5) | rd
            }
            _ => match id {
                SemanticId::Load  => 0xF9400020,
                SemanticId::Store => 0xF9000020,
                SemanticId::Jmp   => 0xD61F0000,
                SemanticId::Bne   => 0x54000001,
                SemanticId::Beq   => 0x54000000,
                SemanticId::Call  => 0xD63F0000,
                SemanticId::Ret   => 0xD65F03C0,
                SemanticId::Cmp   => 0xEB0302EB,
                SemanticId::Mov   => 0xD2800080,
                // ── M 扩展 · ARM64 等价 ──
                SemanticId::Mul | SemanticId::Mulh => {
                    // MADD (×+0 = ×): 9B020011 << (rm<<16) | (rn<<5) | rd
                    // 简化: 用 MADD × rm, 1F (zero reg)
                    if ops.len() >= 3 {
                        let rd = match ops[0].reg() { Some(v) => v as u32, None => return vec![] };
                        let rn = match ops[1].reg() { Some(v) => v as u32, None => return vec![] };
                        let rm = match ops[2].reg() { Some(v) => v as u32, None => return vec![] };
                        (0x9B020000u32) | (rm << 16) | (rn << 5) | rd
                    } else { return vec![]; }
                },
                SemanticId::Div | SemanticId::Divu => {
                    // SDIV/UDIV: 9AC20800 << (rm<<16) | (rn<<5) | rd
                    if ops.len() >= 3 {
                        let rd = match ops[0].reg() { Some(v) => v as u32, None => return vec![] };
                        let rn = match ops[1].reg() { Some(v) => v as u32, None => return vec![] };
                        let rm = match ops[2].reg() { Some(v) => v as u32, None => return vec![] };
                        if id == SemanticId::Divu {
                            (0x8B020000u32) | (rm << 16) | (rn << 5) | rd  // UDIV form
                        } else {
                            (0x9AC20800u32) | (rm << 16) | (rn << 5) | rd  // SDIV
                        }
                    } else { return vec![]; }
                },
                SemanticId::Rem | SemanticId::Remu => {
                    // MSUB: rd - rn × rm → remainder 在 ARM 需要多条,简化编码为 MSUB
                    if ops.len() >= 3 {
                        let rd = match ops[0].reg() { Some(v) => v as u32, None => return vec![] };
                        let rn = match ops[1].reg() { Some(v) => v as u32, None => return vec![] };
                        let rm = match ops[2].reg() { Some(v) => v as u32, None => return vec![] };
                        (0x9B220000u32) | (rm << 16) | (rn << 5) | rd  // MSUB as placeholder
                    } else { return vec![]; }
                },
                SemanticId::Addw => {
                    // ADD W (32-bit): 0B020000
                    if ops.len() >= 3 {
                        let rd = match ops[0].reg() { Some(v) => v as u32, None => return vec![] };
                        let rn = match ops[1].reg() { Some(v) => v as u32, None => return vec![] };
                        let rm = match ops[2].reg() { Some(v) => v as u32, None => return vec![] };
                        (0x0B020000u32) | (rm << 16) | (rn << 5) | rd
                    } else { return vec![]; }
                }
                SemanticId::Addiw => {
                    // ADDI W (32-bit): 11000000 << imm12
                    if ops.len() >= 2 {
                        let rd = match ops[0].reg() { Some(v) => v as u32, None => return vec![] };
                        let rn = match ops[1].reg() { Some(v) => v as u32, None => return vec![] };
                        let imm = match ops[2].imm() { Some(v) => (v as u32) & 0xFFF, None => return vec![] };
                        (0x11000000u32) | (imm << 10) | (rn << 5) | rd
                    } else { return vec![]; }
                }
                // ── A 扩展 · ARM64 原子 ──
                SemanticId::Lrw => 0xC87F0400,         // LDAXR
                SemanticId::Scw => 0xC803FC00,         // STLXR
                SemanticId::AmoSwapw => 0x38206820,    // LD64B (simplified)
                SemanticId::AmoAddw => 0x38201820,     // STADDL
                _ => return vec![],
            },
        };
        code.to_be_bytes().to_vec()
    }

    fn enc_x86(id: SemanticId, ops: &[Operand]) -> Vec<u8> {
        let mut out = Vec::new();
        match id {
            SemanticId::Add => {
                out.push(0x48); out.push(0x01);
                let reg = match ops[2].reg() { Some(v) => v, None => return vec![] };
                let rm  = match ops[0].reg() { Some(v) => v, None => return vec![] };
                out.push(0xC0 | (reg << 3) | rm);
            }
            SemanticId::Sub => {
                out.push(0x48); out.push(0x29);
                let reg = match ops[2].reg() { Some(v) => v, None => return vec![] };
                let rm  = match ops[0].reg() { Some(v) => v, None => return vec![] };
                out.push(0xC0 | (reg << 3) | rm);
            }
            SemanticId::And => {
                out.push(0x48); out.push(0x21);
                let reg = match ops[2].reg() { Some(v) => v, None => return vec![] };
                let rm  = match ops[0].reg() { Some(v) => v, None => return vec![] };
                out.push(0xC0 | (reg << 3) | rm);
            }
            SemanticId::Or => {
                out.push(0x48); out.push(0x09);
                let reg = match ops[2].reg() { Some(v) => v, None => return vec![] };
                let rm  = match ops[0].reg() { Some(v) => v, None => return vec![] };
                out.push(0xC0 | (reg << 3) | rm);
            }
            SemanticId::Xor => {
                out.push(0x48); out.push(0x31);
                let reg = match ops[2].reg() { Some(v) => v, None => return vec![] };
                let rm  = match ops[0].reg() { Some(v) => v, None => return vec![] };
                out.push(0xC0 | (reg << 3) | rm);
            }
            SemanticId::Addi => {
                out.push(0x48); out.push(0x83);
                let rm = match ops[0].reg() { Some(v) => v, None => return vec![] };
                out.push(0xC0 | rm);
                let imm = match ops[1].imm() { Some(v) => v as u8, None => return vec![] };
                out.push(imm);
            }
            // ── M 扩展 · x86 等价 ──
            SemanticId::Mul | SemanticId::Mulh => {
                // IMUL r,r (REX.W 0F AF /r)
                out.push(0x48); out.push(0x0F); out.push(0xAF);
                let rd = match ops[0].reg() { Some(v) => v, None => return vec![] };
                let r2 = match ops[1].reg() { Some(v) => v, None => return vec![] };
                out.push(0xC0 | ((rd & 7) << 3) | (r2 & 7));
            }
            SemanticId::Div | SemanticId::Divu => {
                // IDIV r (REX.W F7 /7) 或 DIV (REX.W F7 /6)
                let r = match ops[0].reg() { Some(v) => v, None => return vec![] };
                let slash = if id == SemanticId::Div { 7u8 } else { 6u8 };
                out.push(0x48); out.push(0xF7);
                out.push(0xC0 | ((slash & 7) << 3) | (r & 7));
            }
            SemanticId::Rem | SemanticId::Remu => {
                // x86 的余数在同一条 IDIV 中产生 (→RDX). 编码为 IDIV + 取 DX 探针
                let r = match ops[0].reg() { Some(v) => v, None => return vec![] };
                let slash = if id == SemanticId::Rem { 7u8 } else { 6u8 };
                out.push(0x48); out.push(0xF7);
                out.push(0xC0 | ((slash & 7) << 3) | (r & 7));
                out.push(0x48); out.push(0x89); out.push(0xD0);  // MOV RAX,RDX (提取余数)
            }
            SemanticId::Addw => {
                // x86-64 对 32 位操作: MOVSXD + ADD (跳过,直接用 ADD REX.W)
                out.push(0x48); out.push(0x01);
                let reg = match ops[2].reg() { Some(v) => v, None => return vec![] };
                let rm  = match ops[0].reg() { Some(v) => v, None => return vec![] };
                out.push(0xC0 | (reg << 3) | rm);
            }
            SemanticId::Addiw => {
                out.push(0x48); out.push(0x83);
                let rm = match ops[0].reg() { Some(v) => v, None => return vec![] };
                out.push(0xC0 | rm);
                let imm = match ops[1].imm() { Some(v) => v as u8, None => return vec![] };
                out.push(imm);
            }
            // ── A 扩展原子 · x86 用 LOCK+XCHG / LOCK+XADD ──
            SemanticId::AmoSwapw => {
                // LOCK XCHG: F0 87 /r
                let rd = match ops[0].reg() { Some(v) => v, None => return vec![] };
                let r2 = match ops[1].reg() { Some(v) => v, None => return vec![] };
                out.push(0xF0); out.push(0x48); out.push(0x87);
                out.push(0xC0 | ((rd & 7) << 3) | (r2 & 7));
            }
            SemanticId::AmoAddw => {
                // LOCK XADD: F0 0F C1 /r
                let rd = match ops[0].reg() { Some(v) => v, None => return vec![] };
                let r2 = match ops.get(1).and_then(|o| o.reg()).unwrap_or(0);
                out.push(0xF0); out.push(0x48); out.push(0x0F); out.push(0xC1);
                out.push(0xC0 | ((rd & 7) << 3) | (r2 & 7));
            }
            SemanticId::Lrw => {
                out.extend_from_slice(&[0x0F, 0xAE, 0xE8]);  // LFENCE (原子读)
            }
            SemanticId::Scw => {
                out.extend_from_slice(&[0x0F, 0xAE, 0xF8]);  // SFENCE (原子写)
            }
            _ => match id {
                SemanticId::Load  => out.extend_from_slice(&[0x48, 0x8B, 0x03]),
                SemanticId::Store => out.extend_from_slice(&[0x48, 0x89, 0x18]),
                SemanticId::Jmp   => out.extend_from_slice(&[0xFF, 0xE0]),
                SemanticId::Bne   => out.extend_from_slice(&[0x75, 0x0A]),
                SemanticId::Beq   => out.extend_from_slice(&[0x74, 0x0A]),
                SemanticId::Call  => out.extend_from_slice(&[0xFF, 0xD0]),
                SemanticId::Ret   => out.push(0xC3),
                SemanticId::Cmp   => out.extend_from_slice(&[0x48, 0x39, 0xD8]),
                SemanticId::Mov   => out.extend_from_slice(&[0xB8, 0x78, 0x56, 0x34, 0x12]),
                _ => return vec![],
            },
        }
        out
    }

    fn enc_mips(id: SemanticId, ops: &[Operand]) -> Vec<u8> {
        let code: u32 = match id {
            SemanticId::Add if ops.len() >= 3 => {
                let rd = match ops[0].reg() { Some(v) => v as u32, None => return vec![] };
                let rs = match ops[1].reg() { Some(v) => v as u32, None => return vec![] };
                let rt = match ops[2].reg() { Some(v) => v as u32, None => return vec![] };
                (rs << 21) | (rt << 16) | (rd << 11) | 0b100000
            }
            SemanticId::Sub if ops.len() >= 3 => {
                let rd = match ops[0].reg() { Some(v) => v as u32, None => return vec![] };
                let rs = match ops[1].reg() { Some(v) => v as u32, None => return vec![] };
                let rt = match ops[2].reg() { Some(v) => v as u32, None => return vec![] };
                (rs << 21) | (rt << 16) | (rd << 11) | 0b100010
            }
            // ── M 扩展 · MIPS (mult → R-type 011000 + mflo) ──
            SemanticId::Mul if ops.len() >= 3 => {
                let rs = match ops[1].reg() { Some(v) => v as u32, None => return vec![] };
                let rt = match ops[2].reg() { Some(v) => v as u32, None => return vec![] };
                let rd = match ops[0].reg() { Some(v) => v as u32, None => return vec![] };
                // MULT + MFLO 编码合并为单 4 字节: MULT=011000, MFLO=010010
                // 实际为两条指令,此处用 MFLO 编码占位 (高位 0x0000_010F)
                (rs << 21) | (rt << 16) | (rd << 11) | 0b010010
            }
            SemanticId::Div | SemanticId::Divu if ops.len() >= 3 => {
                let rs = match ops[1].reg() { Some(v) => v as u32, None => return vec![] };
                let rt = match ops[2].reg() { Some(v) => v as u32, None => return vec![] };
                let rd = match ops[0].reg() { Some(v) => v as u32, None => return vec![] };
                // DIV=011010, MFLO=010010 (商) 或 MFHI=010000 (余)
                if id == SemanticId::Divu {
                    (rs << 21) | (rt << 16) | (rd << 11) | 0b011011  // DIVU
                } else {
                    (rs << 21) | (rt << 16) | (rd << 11) | 0b011010  // DIV
                }
            }
            SemanticId::Rem | SemanticId::Remu if ops.len() >= 3 => {
                let rs = match ops[1].reg() { Some(v) => v as u32, None => return vec![] };
                let rt = match ops[2].reg() { Some(v) => v as u32, None => return vec![] };
                let rd = match ops[0].reg() { Some(v) => v as u32, None => return vec![] };
                // 余数 = DIV + MFHI (010000)
                (rs << 21) | (rt << 16) | (rd << 11) | 0b010000
            }
            SemanticId::Mulh => {
                // MULH (高位乘法) 在 MIPS32 = MULT + MFHI
                let rs = match ops.get(1).and_then(|o| o.reg()).unwrap_or(1) as u32;
                let rt = match ops.get(2).and_then(|o| o.reg()).unwrap_or(2) as u32;
                let rd = match ops.get(0).and_then(|o| o.reg()).unwrap_or(3) as u32;
                (rs << 21) | (rt << 16) | (rd << 11) | 0b010000  // MFHI
            }
            SemanticId::Addw if ops.len() >= 3 => {
                // MIPS ADD Word (32 位操作) = SLL + ADD + SRA 简化 → 直接用 ADD
                let rd = match ops[0].reg() { Some(v) => v as u32, None => return vec![] };
                let rs = match ops[1].reg() { Some(v) => v as u32, None => return vec![] };
                let rt = match ops[2].reg() { Some(v) => v as u32, None => return vec![] };
                (rs << 21) | (rt << 16) | (rd << 11) | 0b100000
            }
            SemanticId::Addiw => {
                let rs = match ops[0].reg() { Some(v) => v as u32, None => return vec![] };
                let imm = match ops[1].imm() { Some(v) => (v as u32) & 0xFFFF, None => return vec![] };
                0x20000000u32 | (rs << 21) | (rs << 16) | imm  // ADDI rs,rs,imm
            }
            // ── A 扩展原子 · MIPS LL/SC ──
            SemanticId::Lrw if ops.len() >= 2 => {
                let rd = match ops[0].reg() { Some(v) => v as u32, None => return vec![] };
                let rs = match ops[1].reg() { Some(v) => v as u32, None => return vec![] };
                // LL ( Load Linked ): 0xC0000000 + (rs<<21) + (rd<<16)
                0xC0000000u32 | (rs << 21) | (rd << 16)
            }
            SemanticId::Scw if ops.len() >= 3 => {
                let rd = match ops[0].reg() { Some(v) => v as u32, None => return vec![] };
                let rs = match ops[2].reg() { Some(v) => v as u32, None => return vec![] };
                let rt = match ops[1].reg() { Some(v) => v as u32, None => return vec![] };
                // SC ( Store Conditional ): 0xE0000000 + (rs<<21) + (rt<<16) + (rd<<11)
                0xE0000000u32 | (rs << 21) | (rt << 16) | (rd << 11)
            }
            SemanticId::AmoSwapw if ops.len() >= 3 => {
                // MIPS: LL → MOVE → SC 循环 → 简化编码为 LL + SW + SC 占位
                let rd = match ops[0].reg() { Some(v) => v as u32, None => return vec![] };
                let rs = match ops[2].reg() { Some(v) => v as u32, None => return vec![] };
                0xC0000000u32 | (rs << 21) | (rd << 16)  // LL 占位
            }
            SemanticId::AmoAddw if ops.len() >= 3 => {
                let rd = match ops[0].reg() { Some(v) => v as u32, None => return vec![] };
                let rs = match ops[2].reg() { Some(v) => v as u32, None => return vec![] };
                let rt = match ops[1].reg() { Some(v) => v as u32, None => return vec![] };
                // LL + ADD + SC 循环 → 编码为 LL 占位
                0xC0000000u32 | (rs << 21) | (rd << 16) | (rt << 11) | 0b100000
            }
            _ => match id {
                SemanticId::Addi => 0x21282A00,
                SemanticId::Load  => 0x8E280000,
                SemanticId::Store => 0xAE280000,
                SemanticId::And   => 0x012A4024,
                SemanticId::Or    => 0x012A4025,
                SemanticId::Xor   => 0x012A4026,
                SemanticId::Jmp   => 0x03E00008,
                SemanticId::Bne   => 0x15620000,
                SemanticId::Call  => 0x08000400,
                SemanticId::Ret   => 0x03E00008,
                SemanticId::Cmp   => 0x012A402A,
                SemanticId::Mov   => 0x3C081234,
                _ => return vec![],
            },
        };
        code.to_be_bytes().to_vec()
    }

    pub fn len(&self) -> usize { self.native_encoding.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_riscv_roundtrip() {
        let w = VocabWord::new(
            SemanticId::Add,
            vec![Operand::Reg(1), Operand::Reg(2), Operand::Reg(3)],
            Platform::RiscV,
        );
        assert_eq!(w.native_encoding, vec![0xB3, 0x00, 0x31, 0x00]);
    }

    #[test]
    fn addi_riscv() {
        let w = VocabWord::new(
            SemanticId::Addi,
            vec![Operand::Reg(1), Operand::Reg(1), Operand::Imm(42)],
            Platform::RiscV,
        );
        assert_eq!(w.native_encoding, vec![0x93, 0x80, 0xA0, 0x02]);
    }

    #[test]
    fn add_arm64_length() {
        let w = VocabWord::new(
            SemanticId::Add,
            vec![Operand::Reg(0), Operand::Reg(1), Operand::Reg(2)],
            Platform::ARM64,
        );
        assert_eq!(w.native_encoding.len(), 4);
    }

    #[test]
    fn add_x86() {
        let w = VocabWord::new(
            SemanticId::Add,
            vec![Operand::Reg(0), Operand::Reg(0), Operand::Reg(3)],
            Platform::X86_64,
        );
        assert_eq!(w.native_encoding, vec![0x48, 0x01, 0xD8]);
    }

    #[test]
    fn add_mips() {
        let w = VocabWord::new(
            SemanticId::Add,
            vec![Operand::Reg(8), Operand::Reg(9), Operand::Reg(10)],
            Platform::MIPS32,
        );
        assert_eq!(w.native_encoding, vec![0x01, 0x2A, 0x40, 0x20]);
    }

    #[test]
    fn sub_riscv() {
        let w = VocabWord::new(
            SemanticId::Sub,
            vec![Operand::Reg(5), Operand::Reg(6), Operand::Reg(7)],
            Platform::RiscV,
        );
        // SUB x5,x6,x7 → funct7=0100000, rs2=7, rs1=6, funct3=000, rd=5, op=0110011
        assert_eq!(w.native_encoding.len(), 4);
    }

    #[test]
    fn and_or_xor_riscv() {
        for (sid, expected_funct7) in [
            (SemanticId::And, 0b0000000u32),
            (SemanticId::Or,  0b0000000u32),
            (SemanticId::Xor, 0b0000000u32),
        ] {
            let w = VocabWord::new(
                sid,
                vec![Operand::Reg(5), Operand::Reg(6), Operand::Reg(7)],
                Platform::RiscV,
            );
            assert_eq!(w.native_encoding.len(), 4, "{:?} must be 4 bytes", sid);
            // funct7 is upper 7 bits of 32-bit instruction
            let insn = u32::from_le_bytes([
                w.native_encoding[0], w.native_encoding[1],
                w.native_encoding[2], w.native_encoding[3],
            ]);
            let funct7 = insn >> 25;
            assert_eq!(funct7, expected_funct7, "{:?} funct7 mismatch", sid);
        }
    }

    #[test]
    fn load_store_riscv() {
        let load = VocabWord::new(
            SemanticId::Load,
            vec![Operand::Reg(3), Operand::Mem { base: 2, offset: 8 }],
            Platform::RiscV,
        );
        assert_eq!(load.native_encoding.len(), 4);

        let store = VocabWord::new(
            SemanticId::Store,
            vec![Operand::Reg(3), Operand::Mem { base: 2, offset: 8 }],
            Platform::RiscV,
        );
        assert_eq!(store.native_encoding.len(), 4);
    }

    #[test]
    fn ret_riscv() {
        let w = VocabWord::new(
            SemanticId::Ret,
            vec![],
            Platform::RiscV,
        );
        assert_eq!(w.native_encoding, vec![0x67, 0x80, 0x00, 0x00]);
    }

    #[test]
    fn mov_riscv() {
        let w = VocabWord::new(
            SemanticId::Mov,
            vec![Operand::Reg(10), Operand::Imm(0x42)],
            Platform::RiscV,
        );
        assert_eq!(w.native_encoding.len(), 4);
    }

    #[test]
    fn sub_arm64() {
        let w = VocabWord::new(
            SemanticId::Sub,
            vec![Operand::Reg(0), Operand::Reg(1), Operand::Reg(2)],
            Platform::ARM64,
        );
        assert_eq!(w.native_encoding.len(), 4);
    }

    #[test]
    fn addi_arm64() {
        let w = VocabWord::new(
            SemanticId::Addi,
            vec![Operand::Reg(0), Operand::Reg(0), Operand::Imm(100)],
            Platform::ARM64,
        );
        assert_eq!(w.native_encoding.len(), 4);
    }

    #[test]
    fn sub_x86() {
        let w = VocabWord::new(
            SemanticId::Sub,
            vec![Operand::Reg(0), Operand::Reg(0), Operand::Reg(3)],
            Platform::X86_64,
        );
        assert_eq!(w.native_encoding, vec![0x48, 0x29, 0xD8]);
    }

    #[test]
    fn addi_x86() {
        let w = VocabWord::new(
            SemanticId::Addi,
            vec![Operand::Reg(0), Operand::Imm(5)],
            Platform::X86_64,
        );
        assert_eq!(w.native_encoding, vec![0x48, 0x83, 0xC0, 0x05]);
    }

    #[test]
    fn sub_mips() {
        let w = VocabWord::new(
            SemanticId::Sub,
            vec![Operand::Reg(8), Operand::Reg(9), Operand::Reg(10)],
            Platform::MIPS32,
        );
        assert_eq!(w.native_encoding.len(), 4);
    }

    #[test]
    fn all_riscv_alu_have_encoding() {
        // R-type (rd, rs1, rs2): Add / Sub / And / Or / Xor
        for sid in [SemanticId::Add, SemanticId::Sub, SemanticId::And, SemanticId::Or, SemanticId::Xor] {
            let w = VocabWord::new(
                sid,
                vec![Operand::Reg(5), Operand::Reg(6), Operand::Reg(7)],
                Platform::RiscV,
            );
            assert!(w.native_encoding.len() > 0, "{:?} RISC-V encoding empty", sid);
        }

        // I-type (rd, rs1, imm): Addi
        let w = VocabWord::new(
            SemanticId::Addi,
            vec![Operand::Reg(5), Operand::Reg(6), Operand::Imm(100)],
            Platform::RiscV,
        );
        assert!(w.native_encoding.len() > 0, "ADDI encoding empty");

        // Jmp: label operand
        let w_jmp = VocabWord::new(SemanticId::Jmp, vec![Operand::Label(0x100)], Platform::RiscV);
        assert!(w_jmp.native_encoding.len() > 0, "JMP RISC-V encoding empty");

        // Ret: no operands
        let w_ret = VocabWord::new(SemanticId::Ret, vec![], Platform::RiscV);
        assert!(w_ret.native_encoding.len() > 0, "RET RISC-V encoding empty");

        // ADD 必须全平台覆盖
        for plat in [Platform::X86_64, Platform::ARM64, Platform::RiscV, Platform::MIPS32] {
            let w = VocabWord::new(
                SemanticId::Add,
                vec![Operand::Reg(1), Operand::Reg(2), Operand::Reg(3)],
                plat,
            );
            assert!(w.native_encoding.len() > 0, "ADD on {:?} empty", plat);
        }
    }

    #[test]
    fn and_or_xor_arm64_operand_driven() {
        for sid in [SemanticId::And, SemanticId::Or, SemanticId::Xor] {
            let w = VocabWord::new(
                sid,
                vec![Operand::Reg(10), Operand::Reg(11), Operand::Reg(12)],
                Platform::ARM64,
            );
            assert_eq!(w.native_encoding.len(), 4, "{:?} ARM64 must be 4 bytes", sid);
            let insn = u32::from_be_bytes([
                w.native_encoding[0], w.native_encoding[1],
                w.native_encoding[2], w.native_encoding[3],
            ]);
            let rd = insn & 0x1F;
            let rn = (insn >> 5) & 0x1F;
            let rm = (insn >> 16) & 0x1F;
            assert_eq!(rd, 10, "{:?} rd mismatch", sid);
            assert_eq!(rn, 11, "{:?} rn mismatch", sid);
            assert_eq!(rm, 12, "{:?} rm mismatch", sid);
        }
    }

    #[test]
    fn and_or_xor_x86_operand_driven() {
        for (sid, expected_opcode) in [
            (SemanticId::And, 0x21u8),
            (SemanticId::Or,  0x09u8),
            (SemanticId::Xor, 0x31u8),
        ] {
            let w = VocabWord::new(
                sid,
                vec![Operand::Reg(0), Operand::Reg(0), Operand::Reg(3)],
                Platform::X86_64,
            );
            assert_eq!(w.native_encoding.len(), 3, "{:?} x86-64 must be 3 bytes", sid);
            assert_eq!(w.native_encoding[0], 0x48, "{:?} REX prefix mismatch", sid);
            assert_eq!(w.native_encoding[1], expected_opcode, "{:?} opcode mismatch", sid);
        }
    }

    #[test]
    fn beq_arm64() {
        let w = VocabWord::new(
            SemanticId::Beq,
            vec![Operand::Reg(0), Operand::Reg(1), Operand::Label(4)],
            Platform::ARM64,
        );
        assert_eq!(w.native_encoding.len(), 4);
    }

    #[test]
    fn beq_x86() {
        let w = VocabWord::new(
            SemanticId::Beq,
            vec![Operand::Reg(0), Operand::Reg(1), Operand::Label(4)],
            Platform::X86_64,
        );
        assert_eq!(w.native_encoding, vec![0x74, 0x0A]);
    }

    // ── RV64I/M/A 新指令编码验证 ──

    #[test]
    fn m_ext_riscv_funct7_is_1() {
        for sid in [SemanticId::Mul, SemanticId::Divu, SemanticId::Remu] {
            let w = VocabWord::new(
                sid,
                vec![Operand::Reg(5), Operand::Reg(6), Operand::Reg(7)],
                Platform::RiscV,
            );
            assert_eq!(w.native_encoding.len(), 4, "{:?} RISC-V must be 4 bytes", sid);
            let insn = u32::from_le_bytes([
                w.native_encoding[0], w.native_encoding[1],
                w.native_encoding[2], w.native_encoding[3],
            ]);
            let funct7 = insn >> 25;
            assert_eq!(funct7, 1, "{:?} funct7 must be 1 for M ext, got {}", sid, funct7);
        }
    }

    #[test]
    fn rv64i_addw_uses_op32_opcode() {
        let w = VocabWord::new(
            SemanticId::Addw,
            vec![Operand::Reg(1), Operand::Reg(2), Operand::Reg(3)],
            Platform::RiscV,
        );
        assert_eq!(w.native_encoding.len(), 4);
        let insn = u32::from_le_bytes([
            w.native_encoding[0], w.native_encoding[1],
            w.native_encoding[2], w.native_encoding[3],
        ]);
        let opcode = insn & 0x7F;
        assert_eq!(opcode, 0b0111011, "addw must use OP-32 opcode 0111011, got {:#09b}", opcode);
    }

    #[test]
    fn rv64i_addiw_uses_opimm32_opcode() {
        let w = VocabWord::new(
            SemanticId::Addiw,
            vec![Operand::Reg(1), Operand::Reg(2), Operand::Imm(42)],
            Platform::RiscV,
        );
        assert_eq!(w.native_encoding.len(), 4);
        let insn = u32::from_le_bytes([
            w.native_encoding[0], w.native_encoding[1],
            w.native_encoding[2], w.native_encoding[3],
        ]);
        let opcode = insn & 0x7F;
        assert_eq!(opcode, 0b0011011, "addiw must use OP-IMM-32 opcode 0011011, got {:#09b}", opcode);
    }

    #[test]
    fn a_ext_amo_uses_amo_opcode() {
        for sid in [SemanticId::Lrw, SemanticId::Scw, SemanticId::AmoSwapw, SemanticId::AmoAddw] {
            let ops = match sid {
                SemanticId::Lrw => vec![Operand::Reg(0), Operand::Reg(1)],
                _ => vec![Operand::Reg(0), Operand::Reg(3), Operand::Reg(1)],
            };
            let w = VocabWord::new(sid, ops, Platform::RiscV);
            assert_eq!(w.native_encoding.len(), 4, "{:?} A-ext must be 4 bytes", sid);
            let insn = u32::from_le_bytes([
                w.native_encoding[0], w.native_encoding[1],
                w.native_encoding[2], w.native_encoding[3],
            ]);
            let opcode = insn & 0x7F;
            assert_eq!(opcode, 0b0101111, "{:?} must use AMO opcode 0101111, got {:#09b}", sid, opcode);
        }
    }

    #[test]
    fn m_ext_mul_encoding_matches_isa_crate() {
        // VocabWord mul encoding must agree with direct isa::Insn::m_op
        use crate::isa::{Insn, Reg, MOp};
        let direct = Insn::m_op(MOp::Mul, Reg(5), Reg(6), Reg(7)).0;
        let via_vocab = VocabWord::new(
            SemanticId::Mul,
            vec![Operand::Reg(5), Operand::Reg(6), Operand::Reg(7)],
            Platform::RiscV,
        );
        let v = u32::from_le_bytes([
            via_vocab.native_encoding[0], via_vocab.native_encoding[1],
            via_vocab.native_encoding[2], via_vocab.native_encoding[3],
        ]);
        assert_eq!(direct, v, "VocabWord(Mul) must equal Insn::m_op(Mul)");
    }

    #[test]
    fn a_ext_lrw_encoding_matches_isa_crate() {
        use crate::isa::{Insn, Reg, AOp};
        let direct = Insn::amo(AOp::LrW, Reg(0), Reg(1), Reg(0)).0;
        let via_vocab = VocabWord::new(
            SemanticId::Lrw,
            vec![Operand::Reg(0), Operand::Reg(1)],
            Platform::RiscV,
        );
        let v = u32::from_le_bytes([
            via_vocab.native_encoding[0], via_vocab.native_encoding[1],
            via_vocab.native_encoding[2], via_vocab.native_encoding[3],
        ]);
        assert_eq!(direct, v, "VocabWord(Lrw) must equal Insn::amo(LrW)");
    }
}

/// 容器ID — 64进制容器编码的直接表示
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContainerId(pub u8);

impl ContainerId {
    pub fn from_u8(v: u8) -> Option<Self> {
        if v <= 0x3F { Some(Self(v)) } else { None }
    }

    pub fn name(&self) -> &'static str {
        match self.0 {
            0x00 => "元",
            0x01 => "加",
            0x02 => "减",
            0x03 => "与",
            0x04 => "或",
            0x05 => "非",
            0x06 => "异或",
            0x07 => "传送",
            0x08 => "跳转",
            0x09 => "调用",
            0x0A => "返回",
            0x0B => "比较",
            0x0C => "加载",
            0x0D => "存储",
            0x0E => "移位",
            0x0F => "置位",
            _ => "容",
        }
    }

    pub fn code(self) -> u8 { self.0 }
}

/// 容器族 — 按语义功能分组的容器集合
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContainerFamily(pub u8);

impl ContainerFamily {
    pub fn from_u8(v: u8) -> Option<Self> {
        if v < 16 { Some(Self(v)) } else { None }
    }

    pub fn name(&self) -> &'static str {
        match self.0 {
            0 => "运算族",
            1 => "逻辑族",
            2 => "控制族",
            3 => "数据族",
            4 => "通信族",
            _ => "扩展族",
        }
    }
}
