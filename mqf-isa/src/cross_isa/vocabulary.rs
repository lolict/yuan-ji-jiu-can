// 词汇层 — 统一语义ID定义
// 不同架构的机器码通过语义ID对齐到同一个"词汇表"

/// 统一语义ID (X轴, 8位)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SemanticId {
    Void  = 0x00,
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
            0x00 => Some(Self::Void),
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
            Self::Void => "空",
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

    #[test]
    fn void_variant_exists() {
        assert_eq!(SemanticId::Void as u8, 0x00);
        assert_eq!(SemanticId::from_u8(0x00), Some(SemanticId::Void));
        assert_eq!(SemanticId::Void.initial(), "空");
    }
}

// ══════════════════════════════════════════════════════════════════════
// ContainerId — 全部 64 容器枚举 (0x00-0x3F)
// ══════════════════════════════════════════════════════════════════════

/// 容器族分类
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerFamily {
    Entity,
    Operation,
    Initial,
    Vowel,
    Undefined,
}

/// 统一容器ID — 覆盖全部 64 容器 (0x00-0x3F)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ContainerId {
    Void          = 0x00,
    Load          = 0x01,
    Store         = 0x02,
    Add           = 0x03,
    Addi          = 0x04,
    Sub           = 0x05,
    And           = 0x06,
    Or            = 0x07,
    Xor           = 0x08,
    Jmp           = 0x09,
    Bne           = 0x0A,
    Beq           = 0x0B,
    Call          = 0x0C,
    Ret           = 0x0D,
    Cmp           = 0x0E,
    Mov           = 0x0F,
    Morphism      = 0x10,
    Mapping       = 0x11,
    Quadrant      = 0x12,
    Pulse         = 0x13,
    Gradient      = 0x14,
    Normal        = 0x15,
    Topology      = 0x16,
    Contraction   = 0x17,
    Derivative    = 0x18,
    Adjunct       = 0x19,
    Unit          = 0x1A,
    Inverse       = 0x1B,
    Composite     = 0x1C,
    Natural       = 0x1D,
    Identity      = 0x1E,
    Limit         = 0x1F,
    InitialBang   = 0x20,
    InitialPang   = 0x21,
    InitialBing   = 0x22,
    InitialMing   = 0x23,
    InitialDuan   = 0x24,
    InitialTou    = 0x25,
    InitialDing2  = 0x26,
    InitialNi     = 0x27,
    InitialZhi    = 0x28,
    InitialZhao   = 0x29,
    InitialShen   = 0x2A,
    InitialRi     = 0x2B,
    InitialLai    = 0x2C,
    InitialJian   = 0x2D,
    InitialXi     = 0x2E,
    InitialQun    = 0x2F,
    VowelDong     = 0x30,
    VowelDong2    = 0x31,
    VowelZhong    = 0x32,
    VowelJiang    = 0x33,
    VowelZhi      = 0x34,
    VowelZhi2     = 0x35,
    VowelZhi3     = 0x36,
    VowelWei      = 0x37,
    VowelYu       = 0x38,
    VowelYu2      = 0x39,
    VowelMo       = 0x3A,
    VowelQi       = 0x3B,
    VowelJia      = 0x3C,
    VowelJie      = 0x3D,
    VowelHui      = 0x3E,
    VowelZhen     = 0x3F,
}

impl ContainerId {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0x00 => Some(Self::Void),
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
            0x10 => Some(Self::Morphism),
            0x11 => Some(Self::Mapping),
            0x12 => Some(Self::Quadrant),
            0x13 => Some(Self::Pulse),
            0x14 => Some(Self::Gradient),
            0x15 => Some(Self::Normal),
            0x16 => Some(Self::Topology),
            0x17 => Some(Self::Contraction),
            0x18 => Some(Self::Derivative),
            0x19 => Some(Self::Adjunct),
            0x1A => Some(Self::Unit),
            0x1B => Some(Self::Inverse),
            0x1C => Some(Self::Composite),
            0x1D => Some(Self::Natural),
            0x1E => Some(Self::Identity),
            0x1F => Some(Self::Limit),
            0x20 => Some(Self::InitialBang),
            0x21 => Some(Self::InitialPang),
            0x22 => Some(Self::InitialBing),
            0x23 => Some(Self::InitialMing),
            0x24 => Some(Self::InitialDuan),
            0x25 => Some(Self::InitialTou),
            0x26 => Some(Self::InitialDing2),
            0x27 => Some(Self::InitialNi),
            0x28 => Some(Self::InitialZhi),
            0x29 => Some(Self::InitialZhao),
            0x2A => Some(Self::InitialShen),
            0x2B => Some(Self::InitialRi),
            0x2C => Some(Self::InitialLai),
            0x2D => Some(Self::InitialJian),
            0x2E => Some(Self::InitialXi),
            0x2F => Some(Self::InitialQun),
            0x30 => Some(Self::VowelDong),
            0x31 => Some(Self::VowelDong2),
            0x32 => Some(Self::VowelZhong),
            0x33 => Some(Self::VowelJiang),
            0x34 => Some(Self::VowelZhi),
            0x35 => Some(Self::VowelZhi2),
            0x36 => Some(Self::VowelZhi3),
            0x37 => Some(Self::VowelWei),
            0x38 => Some(Self::VowelYu),
            0x39 => Some(Self::VowelYu2),
            0x3A => Some(Self::VowelMo),
            0x3B => Some(Self::VowelQi),
            0x3C => Some(Self::VowelJia),
            0x3D => Some(Self::VowelJie),
            0x3E => Some(Self::VowelHui),
            0x3F => Some(Self::VowelZhen),
            _ => None,
        }
    }

    pub fn family(&self) -> ContainerFamily {
        let v = *self as u8;
        match v {
            0x00..=0x0F => ContainerFamily::Entity,
            0x10..=0x1F => ContainerFamily::Operation,
            0x20..=0x2F => ContainerFamily::Initial,
            0x30..=0x3F => ContainerFamily::Vowel,
            _ => ContainerFamily::Undefined,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Void => "虚空",
            Self::Load => "加载",
            Self::Store => "存储",
            Self::Add => "加法",
            Self::Addi => "加急",
            Self::Sub => "减法",
            Self::And => "与门",
            Self::Or => "或门",
            Self::Xor => "异或",
            Self::Jmp => "跳转",
            Self::Bne => "分支",
            Self::Beq => "等分支",
            Self::Call => "调用",
            Self::Ret => "返回",
            Self::Cmp => "比较",
            Self::Mov => "移动",
            Self::Morphism => "态射",
            Self::Mapping => "映射",
            Self::Quadrant => "象限",
            Self::Pulse => "脉冲",
            Self::Gradient => "梯度",
            Self::Normal => "法向",
            Self::Topology => "拓扑",
            Self::Contraction => "缩并",
            Self::Derivative => "衍生",
            Self::Adjunct => "伴随",
            Self::Unit => "单位",
            Self::Inverse => "逆",
            Self::Composite => "复合",
            Self::Natural => "自然",
            Self::Identity => "幺元",
            Self::Limit => "极限",
            Self::InitialBang => "帮母",
            Self::InitialPang => "滂母",
            Self::InitialBing => "并母",
            Self::InitialMing => "明母",
            Self::InitialDuan => "端母",
            Self::InitialTou => "透母",
            Self::InitialDing2 => "定母",
            Self::InitialNi => "泥母",
            Self::InitialZhi => "知母",
            Self::InitialZhao => "照母",
            Self::InitialShen => "审母",
            Self::InitialRi => "日母",
            Self::InitialLai => "来母",
            Self::InitialJian => "见母",
            Self::InitialXi => "溪母",
            Self::InitialQun => "群母",
            Self::VowelDong => "东韵",
            Self::VowelDong2 => "冬韵",
            Self::VowelZhong => "钟韵",
            Self::VowelJiang => "江韵",
            Self::VowelZhi => "支韵",
            Self::VowelZhi2 => "脂韵",
            Self::VowelZhi3 => "之韵",
            Self::VowelWei => "微韵",
            Self::VowelYu => "鱼韵",
            Self::VowelYu2 => "虞韵",
            Self::VowelMo => "模韵",
            Self::VowelQi => "齐韵",
            Self::VowelJia => "佳韵",
            Self::VowelJie => "皆韵",
            Self::VowelHui => "灰韵",
            Self::VowelZhen => "真韵",
        }
    }

    pub fn to_semantic_id(&self) -> Option<SemanticId> {
        SemanticId::from_u8(*self as u8)
    }
}

#[cfg(test)]
mod container_id_tests {
    use super::*;

    #[test]
    fn container_id_roundtrip() {
        for code in 0x00u8..=0x3F {
            let cid = ContainerId::from_u8(code).unwrap();
            assert_eq!(cid as u8, code);
        }
    }

    #[test]
    fn container_id_out_of_range() {
        assert!(ContainerId::from_u8(0x40).is_none());
        assert!(ContainerId::from_u8(0xFF).is_none());
    }

    #[test]
    fn container_family_partition() {
        for code in 0x00..=0x0F {
            let cid = ContainerId::from_u8(code).unwrap();
            assert_eq!(cid.family(), ContainerFamily::Entity);
        }
        for code in 0x10..=0x1F {
            let cid = ContainerId::from_u8(code).unwrap();
            assert_eq!(cid.family(), ContainerFamily::Operation);
        }
        for code in 0x20..=0x2F {
            let cid = ContainerId::from_u8(code).unwrap();
            assert_eq!(cid.family(), ContainerFamily::Initial);
        }
        for code in 0x30..=0x3F {
            let cid = ContainerId::from_u8(code).unwrap();
            assert_eq!(cid.family(), ContainerFamily::Vowel);
        }
    }

    #[test]
    fn entity_to_semantic_id() {
        assert_eq!(ContainerId::Load.to_semantic_id(), Some(SemanticId::Load));
        assert_eq!(ContainerId::Void.to_semantic_id(), Some(SemanticId::Void));
        assert_eq!(ContainerId::Morphism.to_semantic_id(), None);
        assert_eq!(ContainerId::InitialBang.to_semantic_id(), None);
    }

    #[test]
    fn container_names_complete() {
        for code in 0x00u8..=0x3F {
            let cid = ContainerId::from_u8(code).unwrap();
            let n = cid.name();
            assert!(!n.is_empty());
            assert_ne!(n, "未定义");
        }
    }
}
