// 词汇层 — 统一语义ID定义
// 不同架构的机器码通过语义ID对齐到同一个"词汇表"

/// 统一语义ID (X轴, 8位)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SemanticId {
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
            _ => None,
        }
    }

    pub fn initial(&self) -> &'static str {
        match self {
            Self::Add | Self::Addi => "加",
            Self::Sub => "减",
            Self::And | Self::Or | Self::Xor => "合",
            Self::Load | Self::Store => "传",
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
        use crate::isa::{Insn, Reg, ROp, IOp, LoadOp, StoreOp, BranchOp};
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
            SemanticId::Jmp => {
                match ops[0].label() {
                    Some(off) => Insn::jal(Reg(0), off as i32),
                    None => return vec![],
                }
            }
            SemanticId::Ret => Insn(0x00008067),
            SemanticId::Mov if ops.len() >= 2 => {
                match (ops[0].reg(), ops[1].imm()) {
                    (Some(rd), Some(imm)) => Insn::i(IOp::Addi, Reg(rd), Reg(0), imm as i32),
                    _ => return vec![],
                }
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
            SemanticId::Addi if ops.len() >= 3 => {
                let rd = match ops[0].reg() { Some(v) => v as u32, None => return vec![] };
                let rn = match ops[1].reg() { Some(v) => v as u32, None => return vec![] };
                let imm = match ops[2].imm() { Some(v) => (v as u32) & 0xFFF, None => return vec![] };
                (0b10010001 << 24) | (imm << 10) | (rn << 5) | rd
            }
            _ => match id {
                SemanticId::Load  => 0xF9400020,
                SemanticId::Store => 0xF9000020,
                SemanticId::And   => 0x8A000040,
                SemanticId::Or    => 0xAA000240,
                SemanticId::Xor   => 0xCA000040,
                SemanticId::Jmp   => 0xD61F0000,
                SemanticId::Bne   => 0x54000001,
                SemanticId::Call  => 0xD63F0000,
                SemanticId::Ret   => 0xD65F03C0,
                SemanticId::Cmp   => 0xEB0302EB,
                SemanticId::Mov   => 0xD2800080,
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
            SemanticId::Addi => {
                out.push(0x48); out.push(0x83);
                let rm = match ops[0].reg() { Some(v) => v, None => return vec![] };
                out.push(0xC0 | rm);
                let imm = match ops[1].imm() { Some(v) => v as u8, None => return vec![] };
                out.push(imm);
            }
            _ => match id {
                SemanticId::Load  => out.extend_from_slice(&[0x48, 0x8B, 0x03]),
                SemanticId::Store => out.extend_from_slice(&[0x48, 0x89, 0x18]),
                SemanticId::And   => out.extend_from_slice(&[0x48, 0x21, 0xD8]),
                SemanticId::Or    => out.extend_from_slice(&[0x48, 0x09, 0xD8]),
                SemanticId::Xor   => out.extend_from_slice(&[0x48, 0x31, 0xD8]),
                SemanticId::Jmp   => out.extend_from_slice(&[0xFF, 0xE0]),
                SemanticId::Bne   => out.extend_from_slice(&[0x75, 0x0A]),
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
}
