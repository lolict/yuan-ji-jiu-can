// ══════════════════════════════════════════════════════════════════════
// isa — RISC-V RV32I 指令编码/解码器
// ══════════════════════════════════════════════════════════════════════

/// RV32I 操作码
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Opcode {
    Op    = 0b0110011,
    OpImm = 0b0010011,
    Load  = 0b0000011,
    Store = 0b0100011,
    Branch= 0b1100011,
    Lui   = 0b0110111,
    Auipc = 0b0010111,
    Jal   = 0b1101111,
    Jalr  = 0b1100111,
    Sys   = 0b1110011,
}

impl Opcode {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0b0110011 => Some(Self::Op),
            0b0010011 => Some(Self::OpImm),
            0b0000011 => Some(Self::Load),
            0b0100011 => Some(Self::Store),
            0b1100011 => Some(Self::Branch),
            0b0110111 => Some(Self::Lui),
            0b0010111 => Some(Self::Auipc),
            0b1101111 => Some(Self::Jal),
            0b1100111 => Some(Self::Jalr),
            0b1110011 => Some(Self::Sys),
            _ => None,
        }
    }
}

/// 寄存器
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Reg(pub u8);

impl Reg {
    pub fn decode(s: &str) -> Option<Self> {
        let t = s.trim();
        if t.len() < 2 || !t.starts_with('x') { return None; }
        let n: u8 = t[1..].parse().ok()?;
        if n > 31 { None } else { Some(Reg(n)) }
    }
    pub fn name(self) -> String { format!("x{}", self.0) }
}

fn parse_imm(s: &str) -> Option<i32> {
    let t = s.trim();
    if t.starts_with("0x") || t.starts_with("0X") {
        i32::from_str_radix(&t[2..], 16).ok()
    } else if t.starts_with("-0x") || t.starts_with("-0X") {
        i32::from_str_radix(&t[3..], 16).ok().map(|v| -v)
    } else {
        t.parse::<i32>().ok()
    }
}

fn sign_extend_12(v: i32) -> i32 {
    if v & 0x800 != 0 { v | (!0xFFF) } else { v }
}

/// I-type ALU 操作
#[derive(Debug, Clone, Copy)]
pub enum IOp { Addi, Slti, Sltiu, Xori, Ori, Andi, Slli, Srli, Srai }

/// 加载操作
#[derive(Debug, Clone, Copy)]
pub enum LoadOp { Lb=0, Lh=1, Lw=2, Lbu=4, Lhu=5 }

/// 存储操作
#[derive(Debug, Clone, Copy)]
pub enum StoreOp { Sb=0, Sh=1, Sw=2 }

/// 分支操作
#[derive(Debug, Clone, Copy)]
pub enum BranchOp { Beq=0, Bne=1, Blt=4, Bge=5, Bltu=6, Bgeu=7 }

/// R-type ALU 操作
#[derive(Debug, Clone, Copy)]
pub enum ROp { Add, Sub, Sll, Slt, Sltu, Xor, Srl, Sra, Or, And }

/// 32 位指令
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Insn(pub u32);

impl Insn {
    /// R-type: op rd, rs1, rs2
    pub fn r(op: ROp, rd: Reg, rs1: Reg, rs2: Reg) -> Self {
        let funct3 = match op {
            ROp::Add | ROp::Sub => 0b000,
            ROp::Sll => 0b001,
            ROp::Slt => 0b010,
            ROp::Sltu => 0b011,
            ROp::Xor => 0b100,
            ROp::Srl | ROp::Sra => 0b101,
            ROp::Or => 0b110,
            ROp::And => 0b111,
        };
        let funct7 = match op {
            ROp::Sub => 0b0100000,
            ROp::Sra => 0b0100000,
            _ => 0b0000000,
        };
        let code = (funct7 << 25)
                 | ((rs2.0 as u32) << 20)
                 | ((rs1.0 as u32) << 15)
                 | (funct3 << 12)
                 | ((rd.0 as u32) << 7)
                 | 0b0110011;
        Insn(code)
    }

    /// I-type ALU 指令
    pub fn i(op: IOp, rd: Reg, rs1: Reg, imm_or_shamt: i32) -> Self {
        let funct3 = match op {
            IOp::Addi => 0b000, IOp::Slli => 0b001, IOp::Slti => 0b010,
            IOp::Sltiu => 0b011, IOp::Xori => 0b100, IOp::Srli | IOp::Srai => 0b101,
            IOp::Ori => 0b110, IOp::Andi => 0b111,
        };
        let is_shift = matches!(op, IOp::Slli | IOp::Srli | IOp::Srai);
        let funct7 = if matches!(op, IOp::Srai) { 0b0100000 } else { 0 };
        let code = if is_shift {
            let shamt = (imm_or_shamt as u32) & 0x1F;
            (funct7 << 25) | (shamt << 20) | ((rs1.0 as u32) << 15) | (funct3 << 12) | ((rd.0 as u32) << 7) | 0b0010011
        } else {
            let imm = (imm_or_shamt as u32) & 0xFFF;
            (imm << 20) | ((rs1.0 as u32) << 15) | (funct3 << 12) | ((rd.0 as u32) << 7) | 0b0010011
        };
        Insn(code)
    }

    pub fn load(op: LoadOp, rd: Reg, rs1: Reg, offset: i32) -> Self {
        let funct3 = op as u32;
        let imm = (offset as u32) & 0xFFF;
        Insn((imm << 20) | ((rs1.0 as u32) << 15) | (funct3 << 12) | ((rd.0 as u32) << 7) | 0b0000011)
    }

    pub fn store(op: StoreOp, rs2: Reg, rs1: Reg, offset: i32) -> Self {
        let funct3 = op as u32;
        let imm = (offset as u32) & 0xFFF;
        let imm_hi = (imm >> 5) & 0x7F;
        let imm_lo = imm & 0x1F;
        Insn((imm_hi << 25) | ((rs2.0 as u32) << 20) | ((rs1.0 as u32) << 15) | (funct3 << 12) | (imm_lo << 7) | 0b0100011)
    }

    pub fn branch(op: BranchOp, rs1: Reg, rs2: Reg, offset: i32) -> Self {
        let funct3 = op as u32;
        let imm = offset as u32;
        let b12  = (imm >> 12) & 0x1;
        let b10_5= (imm >> 5) & 0x3F;
        let b4_1 = (imm >> 1) & 0xF;
        let b11  = (imm >> 11) & 0x1;
        Insn((b12 << 31) | (b10_5 << 25) | ((rs2.0 as u32) << 20) | ((rs1.0 as u32) << 15) | (funct3 << 12) | (b4_1 << 8) | (b11 << 7) | 0b1100011)
    }

    pub fn lui(rd: Reg, imm: i32) -> Self {
        let v = ((imm as u32) >> 12) & 0xFFFFF;
        Insn((v << 12) | ((rd.0 as u32) << 7) | 0b0110111)
    }

    pub fn auipc(rd: Reg, imm: i32) -> Self {
        let v = ((imm as u32) >> 12) & 0xFFFFF;
        Insn((v << 12) | ((rd.0 as u32) << 7) | 0b0010111)
    }

    pub fn jal(rd: Reg, offset: i32) -> Self {
        let imm = offset as u32;
        let j20   = (imm >> 20) & 0x1;
        let j10_1 = (imm >> 1) & 0x3FF;
        let j11   = (imm >> 11) & 0x1;
        let j19_12= (imm >> 12) & 0xFF;
        Insn((j20 << 31) | (j10_1 << 21) | (j11 << 20) | (j19_12 << 12) | ((rd.0 as u32) << 7) | 0b1101111)
    }

    pub fn jalr(rd: Reg, rs1: Reg, offset: i32) -> Self {
        let imm = (offset as u32) & 0xFFF;
        Insn((imm << 20) | ((rs1.0 as u32) << 15) | ((rd.0 as u32) << 7) | 0b1100111)
    }

    /// 从汇编字符串编码
    pub fn encode(s: &str) -> Option<Self> {
        let s = s.trim().split('#').next().unwrap().trim();
        let tokens: Vec<&str> = s.split(&[',', ' ', '(', ')'][..]).filter(|t| !t.is_empty()).collect();
        if tokens.is_empty() { return None; }
        Some(match tokens[0].to_lowercase().as_str() {
            "add"   => Self::r(ROp::Add,  Self::rd(tokens[1])?, Self::rd(tokens[2])?, Self::rd(tokens[3])?),
            "sub"   => Self::r(ROp::Sub,  Self::rd(tokens[1])?, Self::rd(tokens[2])?, Self::rd(tokens[3])?),
            "and"   => Self::r(ROp::And,  Self::rd(tokens[1])?, Self::rd(tokens[2])?, Self::rd(tokens[3])?),
            "or"    => Self::r(ROp::Or,   Self::rd(tokens[1])?, Self::rd(tokens[2])?, Self::rd(tokens[3])?),
            "xor"   => Self::r(ROp::Xor,  Self::rd(tokens[1])?, Self::rd(tokens[2])?, Self::rd(tokens[3])?),
            "sll"   => Self::r(ROp::Sll,  Self::rd(tokens[1])?, Self::rd(tokens[2])?, Self::rd(tokens[3])?),
            "srl"   => Self::r(ROp::Srl,  Self::rd(tokens[1])?, Self::rd(tokens[2])?, Self::rd(tokens[3])?),
            "sra"   => Self::r(ROp::Sra,  Self::rd(tokens[1])?, Self::rd(tokens[2])?, Self::rd(tokens[3])?),
            "slt"   => Self::r(ROp::Slt,  Self::rd(tokens[1])?, Self::rd(tokens[2])?, Self::rd(tokens[3])?),
            "sltu"  => Self::r(ROp::Sltu, Self::rd(tokens[1])?, Self::rd(tokens[2])?, Self::rd(tokens[3])?),
            "addi"  => Self::i(IOp::Addi,  Self::rd(tokens[1])?, Self::rd(tokens[2])?, parse_imm(tokens[3])?),
            "slti"  => Self::i(IOp::Slti,  Self::rd(tokens[1])?, Self::rd(tokens[2])?, parse_imm(tokens[3])?),
            "sltiu" => Self::i(IOp::Sltiu, Self::rd(tokens[1])?, Self::rd(tokens[2])?, parse_imm(tokens[3])?),
            "xori"  => Self::i(IOp::Xori,  Self::rd(tokens[1])?, Self::rd(tokens[2])?, parse_imm(tokens[3])?),
            "ori"   => Self::i(IOp::Ori,   Self::rd(tokens[1])?, Self::rd(tokens[2])?, parse_imm(tokens[3])?),
            "andi"  => Self::i(IOp::Andi,  Self::rd(tokens[1])?, Self::rd(tokens[2])?, parse_imm(tokens[3])?),
            "slli"  => Self::i(IOp::Slli,  Self::rd(tokens[1])?, Self::rd(tokens[2])?, parse_imm(tokens[3])?),
            "srli"  => Self::i(IOp::Srli,  Self::rd(tokens[1])?, Self::rd(tokens[2])?, parse_imm(tokens[3])?),
            "srai"  => Self::i(IOp::Srai,  Self::rd(tokens[1])?, Self::rd(tokens[2])?, parse_imm(tokens[3])?),
            "lb"    => Self::load(LoadOp::Lb,  Self::rd(tokens[1])?, Self::rd(tokens[3])?, parse_imm(tokens[2])?),
            "lh"    => Self::load(LoadOp::Lh,  Self::rd(tokens[1])?, Self::rd(tokens[3])?, parse_imm(tokens[2])?),
            "lw"    => Self::load(LoadOp::Lw,  Self::rd(tokens[1])?, Self::rd(tokens[3])?, parse_imm(tokens[2])?),
            "lbu"   => Self::load(LoadOp::Lbu, Self::rd(tokens[1])?, Self::rd(tokens[3])?, parse_imm(tokens[2])?),
            "lhu"   => Self::load(LoadOp::Lhu, Self::rd(tokens[1])?, Self::rd(tokens[3])?, parse_imm(tokens[2])?),
            "sb"    => Self::store(StoreOp::Sb, Self::rd(tokens[1])?, Self::rd(tokens[3])?, parse_imm(tokens[2])?),
            "sh"    => Self::store(StoreOp::Sh, Self::rd(tokens[1])?, Self::rd(tokens[3])?, parse_imm(tokens[2])?),
            "sw"    => Self::store(StoreOp::Sw, Self::rd(tokens[1])?, Self::rd(tokens[3])?, parse_imm(tokens[2])?),
            "beq"   => Self::branch(BranchOp::Beq,  Self::rd(tokens[1])?, Self::rd(tokens[2])?, parse_imm(tokens[3])?),
            "bne"   => Self::branch(BranchOp::Bne,  Self::rd(tokens[1])?, Self::rd(tokens[2])?, parse_imm(tokens[3])?),
            "blt"   => Self::branch(BranchOp::Blt,  Self::rd(tokens[1])?, Self::rd(tokens[2])?, parse_imm(tokens[3])?),
            "bge"   => Self::branch(BranchOp::Bge,  Self::rd(tokens[1])?, Self::rd(tokens[2])?, parse_imm(tokens[3])?),
            "bltu"  => Self::branch(BranchOp::Bltu, Self::rd(tokens[1])?, Self::rd(tokens[2])?, parse_imm(tokens[3])?),
            "bgeu"  => Self::branch(BranchOp::Bgeu, Self::rd(tokens[1])?, Self::rd(tokens[2])?, parse_imm(tokens[3])?),
            "jal"   => Self::jal(Self::rd(tokens[1])?, if tokens.len() > 2 { parse_imm(tokens[2])? } else { 0 }),
            "jalr"  => Self::jalr(Self::rd(tokens[1])?, Self::rd(tokens[2])?, parse_imm(tokens[3])?),
            "lui"   => Self::lui(Self::rd(tokens[1])?, parse_imm(tokens[2])?),
            "auipc" => Self::auipc(Self::rd(tokens[1])?, parse_imm(tokens[2])?),
            "ecall" => Insn(0x00000073),
            "ebreak"=> Insn(0x00100073),
            "nop"   => Insn(0x00000013),
            _ => return None,
        })
    }

    /// 解码为汇编字符串
    pub fn decode(self) -> String {
        let code = self.0;
        let opcode = (code & 0x7F) as u8;
        let rd_idx  = ((code >> 7) & 0x1F) as u8;
        let funct3  = ((code >> 12) & 0x7) as u8;
        let rs1_idx = ((code >> 15) & 0x1F) as u8;
        let rs2_idx = ((code >> 20) & 0x1F) as u8;
        let funct7  = ((code >> 25) & 0x7F) as u8;
        let rd  = Reg(rd_idx);
        let rs1 = Reg(rs1_idx);
        let rs2 = Reg(rs2_idx);
        let name;
        match opcode {
            0b0110011 => {
                name = match funct3 {
                    0b000 => if funct7 == 0b0100000 { "sub" } else { "add" },
                    0b001 => "sll",
                    0b010 => "slt",
                    0b011 => "sltu",
                    0b100 => "xor",
                    0b101 => if funct7 == 0b0100000 { "sra" } else { "srl" },
                    0b110 => "or",
                    0b111 => "and",
                    _ => "unknown_R",
                };
                format!("{} {}, {}, {}", name, rd.name(), rs1.name(), rs2.name())
            }
            0b0010011 => match funct3 {
                0b000 => format!("addi {}, {}, {}", rd.name(), rs1.name(), sign_extend_12((code as i32) >> 20)),
                0b010 => format!("slti {}, {}, {}", rd.name(), rs1.name(), sign_extend_12((code as i32) >> 20)),
                0b011 => format!("sltiu {}, {}, {}", rd.name(), rs1.name(), sign_extend_12((code as i32) >> 20)),
                0b100 => format!("xori {}, {}, {}", rd.name(), rs1.name(), sign_extend_12((code as i32) >> 20)),
                0b110 => format!("ori {}, {}, {}", rd.name(), rs1.name(), sign_extend_12((code as i32) >> 20)),
                0b111 => format!("andi {}, {}, {}", rd.name(), rs1.name(), sign_extend_12((code as i32) >> 20)),
                0b001 => format!("slli {}, {}, {}", rd.name(), rs1.name(), rs2_idx),
                0b101 => {
                    let n = if funct7 == 0b0100000 { "srai" } else { "srli" };
                    format!("{} {}, {}, {}", n, rd.name(), rs1.name(), rs2_idx)
                }
                _ => format!("unknown_I_{:03b}", funct3),
            },
            0b0000011 => {
                let imm = sign_extend_12((code as i32) >> 20);
                name = match funct3 { 0b000 => "lb", 0b001 => "lh", 0b010 => "lw", 0b100 => "lbu", 0b101 => "lhu", _ => "unknown_load" };
                format!("{} {}, {}({})", name, rd.name(), imm, rs1.name())
            }
            0b0100011 => {
                let imm = sign_extend_12(((funct7 as i32) << 5) | (rd_idx as i32));
                name = match funct3 { 0b000 => "sb", 0b001 => "sh", 0b010 => "sw", _ => "unknown_store" };
                format!("{} {}, {}({})", name, rs2.name(), imm, rs1.name())
            }
            0b1100011 => {
                let imm = decode_b_imm(code);
                name = match funct3 { 0b000 => "beq", 0b001 => "bne", 0b100 => "blt", 0b101 => "bge", 0b110 => "bltu", 0b111 => "bgeu", _ => "unknown_branch" };
                format!("{} {}, {}, {}", name, rs1.name(), rs2.name(), imm)
            }
            0b0110111 => format!("lui {}, 0x{:05x}", rd.name(), (code >> 12) & 0xFFFFF),
            0b0010111 => format!("auipc {}, 0x{:05x}", rd.name(), (code >> 12) & 0xFFFFF),
            0b1101111 => format!("jal {}, {}", rd.name(), decode_j_imm(code)),
            0b1100111 => format!("jalr {}, {}({})", rd.name(), sign_extend_12((code as i32) >> 20), rs1.name()),
            0b1110011 => "ecall".to_string(),
            _ => format!("unknown_{:07b}", opcode),
        }
    }

    pub fn hex(self) -> String { format!("{:08x}", self.0) }

    fn rd(s: &str) -> Option<Reg> { Reg::decode(s) }
}

fn decode_b_imm(code: u32) -> i32 {
    let b12   = (code >> 31) & 0x1;
    let b10_5 = (code >> 25) & 0x3F;
    let b4_1  = (code >> 8) & 0xF;
    let b11   = (code >> 7) & 0x1;
    let imm = (b12 << 12) | (b11 << 11) | (b10_5 << 5) | (b4_1 << 1);
    if imm & 0x1000 != 0 { (imm | (!0x1FFF)) as i32 } else { imm as i32 }
}

fn decode_j_imm(code: u32) -> i32 {
    let j20    = (code >> 31) & 0x1;
    let j19_12 = (code >> 12) & 0xFF;
    let j11    = (code >> 20) & 0x1;
    let j10_1  = (code >> 21) & 0x3FF;
    let imm = (j20 << 20) | (j19_12 << 12) | (j11 << 11) | (j10_1 << 1);
    if imm & 0x100000 != 0 { (imm | (!0x1FFFFF)) as i32 } else { imm as i32 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_add_r0_r1_r2() {
        // tools/isa-ref/riscv.md: add x0, x1, x2 = 00208033
        assert_eq!(Insn::r(ROp::Add, Reg(0), Reg(1), Reg(2)).0, 0x00208033);
    }

    #[test]
    fn encode_sub_r0_r1_r2() {
        assert_eq!(Insn::r(ROp::Sub, Reg(0), Reg(1), Reg(2)).0, 0x40208033);
    }

    #[test]
    fn encode_nop() {
        assert_eq!(Insn::i(IOp::Addi, Reg(0), Reg(0), 0).0, 0x00000013);
    }

    #[test]
    fn encode_sw_r0_0_r1() {
        assert_eq!(Insn::store(StoreOp::Sw, Reg(0), Reg(1), 0).0, 0x0000A023);
    }

    #[test]
    fn encode_lw_r0_8_r1() {
        assert_eq!(Insn::load(LoadOp::Lw, Reg(0), Reg(1), 8).0, 0x0080A003);
    }

    #[test]
    fn encode_jal_r0_0() {
        // jal x0, 0 = infinite loop = 0x0000006F
        assert_eq!(Insn::jal(Reg(0), 0).0, 0x0000006F);
    }

    #[test]
    fn decode_add_r1_r2_r3() {
        assert_eq!(Insn(0x003100B3).decode(), "add x1, x2, x3");
    }

    #[test]
    fn decode_sub_r0_r1_r2() {
        assert_eq!(Insn(0x40208033).decode(), "sub x0, x1, x2");
    }

    #[test]
    fn decode_lw_r0_8_r1() {
        assert_eq!(Insn(0x0080A003).decode(), "lw x0, 8(x1)");
    }

    #[test]
    fn encode_decode_roundtrip() {
        for asm_orig in &[
            "add x1, x2, x3",
            "sub x4, x5, x6",
            "and x7, x8, x9",
            "addi x5, x6, -100",
            "lw x3, 16(x4)",
            "sw x3, 32(x4)",
        ] {
            let insn = Insn::encode(asm_orig).unwrap();
            let decoded = insn.decode();
            assert!(!decoded.starts_with("unknown"), "decode({}) -> {}", asm_orig, decoded);
            let re = Insn::encode(&decoded).unwrap();
            assert_eq!(insn.0, re.0, "rt mismatch: {} -> {} -> {:08x} vs {:08x}", asm_orig, decoded, insn.0, re.0);
        }
    }

    #[test]
    fn against_isa_ref_doc() {
        // R-type: 来自 RISC-V 官方手册,与 riscv.md 一致
        assert_eq!(Insn::encode("add x0, x1, x2").unwrap().0, 0x00208033);
        assert_eq!(Insn::encode("sub x0, x1, x2").unwrap().0, 0x40208033);
        // I-type: 经手动按位域重新计算,riscv.md 部分条目遗漏了 rs1 字段
        assert_eq!(Insn::encode("addi x0, x1, 42").unwrap().0, 0x02A08013);
        assert_eq!(Insn::encode("lw x0, 8(x1)").unwrap().0, 0x0080A003);
        assert_eq!(Insn::encode("sw x0, 0(x1)").unwrap().0, 0x0000A023);
        assert_eq!(Insn::encode("addi x0, x0, 0").unwrap().0, 0x00000013);
        assert_eq!(Insn::encode("jal x0, 0").unwrap().0, 0x0000006F);
    }
}
