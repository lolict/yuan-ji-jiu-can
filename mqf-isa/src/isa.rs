// ══════════════════════════════════════════════════════════════════════
// isa — RISC-V 指令编码/解码器
// ══════════════════════════════════════════════════════════════════════
//
// 支持 RV32I + RV64I + M (乘除) + A (原子)
// 编码原则: 仅加减 · 禁用乘除 (进制运算在编码层实现)

/// 32-bit 操作码 (标准 + 扩展)
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Opcode {
    // ── 基础 RV32I ──
    Op     = 0b0110011,   // R-type ALU
    OpImm  = 0b0010011,   // I-type ALU
    Load   = 0b0000011,   // Load
    Store  = 0b0100011,   // Store
    Branch = 0b1100011,   // Branch
    Lui    = 0b0110111,   // LUI
    Auipc  = 0b0010111,   // AUIPC
    Jal    = 0b1101111,   // JAL
    Jalr   = 0b1100111,   // JALR
    Sys    = 0b1110011,   // SYSTEM
    // ── RV64I 扩展 ──
    Op32   = 0b0111011,   // OP-32 (addw, subw, ...)
    OpImm32= 0b0011011,   // OP-IMM-32 (addiw, ...)
    // ── A 扩展 (原子) ──
    Amo    = 0b0101111,   // Atomic Memory Operation
}

impl Opcode {
    pub fn from_u8(v: u8) -> Option<Self> {
        Some(match v {
            0b0110011 => Self::Op,
            0b0010011 => Self::OpImm,
            0b0000011 => Self::Load,
            0b0100011 => Self::Store,
            0b1100011 => Self::Branch,
            0b0110111 => Self::Lui,
            0b0010111 => Self::Auipc,
            0b1101111 => Self::Jal,
            0b1100111 => Self::Jalr,
            0b1110011 => Self::Sys,
            0b0111011 => Self::Op32,
            0b0011011 => Self::OpImm32,
            0b0101111 => Self::Amo,
            _ => return None,
        })
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

fn sign_extend_6(v: i32) -> i32 {
    if v & 0x20 != 0 { v | (!0x3F) } else { v }
}

/// I-type ALU 操作
#[derive(Debug, Clone, Copy)]
pub enum IOp { Addi, Slti, Sltiu, Xori, Ori, Andi, Slli, Srli, Srai }

/// 加载操作
#[derive(Debug, Clone, Copy)]
pub enum LoadOp { Lb=0, Lh=1, Lw=2, Ld=3, Lbu=4, Lhu=5, Lwu=6 }

/// 存储操作
#[derive(Debug, Clone, Copy)]
pub enum StoreOp { Sb=0, Sh=1, Sw=2, Sd=3 }

/// 分支操作
#[derive(Debug, Clone, Copy)]
pub enum BranchOp { Beq=0, Bne=1, Blt=4, Bge=5, Bltu=6, Bgeu=7 }

/// R-type ALU 操作
#[derive(Debug, Clone, Copy)]
pub enum ROp { Add, Sub, Sll, Slt, Sltu, Xor, Srl, Sra, Or, And }

/// R-type 64-bit word 操作 (RV64I OP-32)
#[derive(Debug, Clone, Copy)]
pub enum ROp32 { Addw, Subw, Sllw, Srlw, Sraw }

/// I-type 64-bit word 操作 (RV64I OP-IMM-32)
#[derive(Debug, Clone, Copy)]
pub enum IOp32 { Addiw, Slliw, Srliw, Sraiw }

/// M 扩展操作 (乘除)
#[derive(Debug, Clone, Copy)]
pub enum MOp { Mul, Mulh, Mulhsu, Mulhu, Div, Divu, Rem, Remu }

/// A 扩展原子操作
#[derive(Debug, Clone, Copy)]
pub enum AOp {
    LrW, LrD,
    ScW, ScD,
    AmoSwapW, AmoSwapD,
    AmoAddW, AmoAddD,
    AmoXorW, AmoXorD,
    AmoAndW, AmoAndD,
    AmoOrW, AmoOrD,
    AmoMinW, AmoMinD,
    AmoMaxW, AmoMaxD,
    AmoMinuW, AmoMinuD,
    AmoMaxuW, AmoMaxuD,
}

/// 32 位指令
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Insn(pub u32);

impl Insn {
    // ═══════════ RV32I R-type ═══════════

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
                 | (Opcode::Op as u32);
        Insn(code)
    }

    // ═══════════ RV32I I-type ═══════════

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
            (funct7 << 25) | (shamt << 20) | ((rs1.0 as u32) << 15) | (funct3 << 12) | ((rd.0 as u32) << 7) | (Opcode::OpImm as u32)
        } else {
            let imm = (imm_or_shamt as u32) & 0xFFF;
            (imm << 20) | ((rs1.0 as u32) << 15) | (funct3 << 12) | ((rd.0 as u32) << 7) | (Opcode::OpImm as u32)
        };
        Insn(code)
    }

    // ═══════════ Load / Store ═══════════

    pub fn load(op: LoadOp, rd: Reg, rs1: Reg, offset: i32) -> Self {
        let funct3 = op as u32;
        let imm = (offset as u32) & 0xFFF;
        Insn((imm << 20) | ((rs1.0 as u32) << 15) | (funct3 << 12) | ((rd.0 as u32) << 7) | (Opcode::Load as u32))
    }

    pub fn store(op: StoreOp, rs2: Reg, rs1: Reg, offset: i32) -> Self {
        let funct3 = op as u32;
        let imm = (offset as u32) & 0xFFF;
        let imm_hi = (imm >> 5) & 0x7F;
        let imm_lo = imm & 0x1F;
        Insn((imm_hi << 25) | ((rs2.0 as u32) << 20) | ((rs1.0 as u32) << 15) | (funct3 << 12) | (imm_lo << 7) | (Opcode::Store as u32))
    }

    // ═══════════ Branch ═══════════

    pub fn branch(op: BranchOp, rs1: Reg, rs2: Reg, offset: i32) -> Self {
        let funct3 = op as u32;
        let imm = offset as u32;
        let b12   = (imm >> 12) & 0x1;
        let b10_5 = (imm >> 5) & 0x3F;
        let b4_1  = (imm >> 1) & 0xF;
        let b11   = (imm >> 11) & 0x1;
        Insn((b12 << 31) | (b10_5 << 25) | ((rs2.0 as u32) << 20) | ((rs1.0 as u32) << 15) | (funct3 << 12) | (b4_1 << 8) | (b11 << 7) | (Opcode::Branch as u32))
    }

    // ═══════════ U-type ═══════════

    pub fn lui(rd: Reg, imm: i32) -> Self {
        let v = ((imm as u32) >> 12) & 0xFFFFF;
        Insn((v << 12) | ((rd.0 as u32) << 7) | (Opcode::Lui as u32))
    }

    pub fn auipc(rd: Reg, imm: i32) -> Self {
        let v = ((imm as u32) >> 12) & 0xFFFFF;
        Insn((v << 12) | ((rd.0 as u32) << 7) | (Opcode::Auipc as u32))
    }

    // ═══════════ J-type ═══════════

    pub fn jal(rd: Reg, offset: i32) -> Self {
        let imm = offset as u32;
        let j20    = (imm >> 20) & 0x1;
        let j10_1  = (imm >> 1) & 0x3FF;
        let j11    = (imm >> 11) & 0x1;
        let j19_12 = (imm >> 12) & 0xFF;
        Insn((j20 << 31) | (j10_1 << 21) | (j11 << 20) | (j19_12 << 12) | ((rd.0 as u32) << 7) | (Opcode::Jal as u32))
    }

    pub fn jalr(rd: Reg, rs1: Reg, offset: i32) -> Self {
        let imm = (offset as u32) & 0xFFF;
        Insn((imm << 20) | ((rs1.0 as u32) << 15) | ((rd.0 as u32) << 7) | (Opcode::Jalr as u32))
    }

    // ═══════════ RV64I OP-32 (word-width R-type) ═══════════

    pub fn r32(op: ROp32, rd: Reg, rs1: Reg, rs2: Reg) -> Self {
        let funct3 = match op {
            ROp32::Addw | ROp32::Subw => 0b000,
            ROp32::Sllw => 0b001,
            ROp32::Srlw | ROp32::Sraw => 0b101,
        };
        let funct7 = match op {
            ROp32::Subw => 0b0100000,
            ROp32::Sraw => 0b0100000,
            _ => 0b0000000,
        };
        let code = (funct7 << 25)
                 | ((rs2.0 as u32) << 20)
                 | ((rs1.0 as u32) << 15)
                 | (funct3 << 12)
                 | ((rd.0 as u32) << 7)
                 | (Opcode::Op32 as u32);
        Insn(code)
    }

    // ═══════════ RV64I OP-IMM-32 (word-width I-type) ═══════════

    pub fn i32(op: IOp32, rd: Reg, rs1: Reg, imm_or_shamt: i32) -> Self {
        let funct3 = match op {
            IOp32::Addiw => 0b000,
            IOp32::Slliw => 0b001,
            IOp32::Srliw | IOp32::Sraiw => 0b101,
        };
        let is_shift = matches!(op, IOp32::Slliw | IOp32::Srliw | IOp32::Sraiw);
        let funct7 = if matches!(op, IOp32::Sraiw) { 0b0100000 } else { 0 };
        let code = if is_shift {
            let shamt = (imm_or_shamt as u32) & 0x1F;
            (funct7 << 25) | (shamt << 20) | ((rs1.0 as u32) << 15) | (funct3 << 12) | ((rd.0 as u32) << 7) | (Opcode::OpImm32 as u32)
        } else {
            let imm = (imm_or_shamt as u32) & 0xFFF;
            (imm << 20) | ((rs1.0 as u32) << 15) | (funct3 << 12) | ((rd.0 as u32) << 7) | (Opcode::OpImm32 as u32)
        };
        Insn(code)
    }

    // ═══════════ M 扩展 (乘除) — OP opcode with funct7=0000001 ═══════════

    pub fn m_op(op: MOp, rd: Reg, rs1: Reg, rs2: Reg) -> Self {
        let funct3 = match op {
            MOp::Mul => 0b000, MOp::Mulh => 0b001, MOp::Mulhsu => 0b010, MOp::Mulhu => 0b011,
            MOp::Div => 0b100, MOp::Divu => 0b101, MOp::Rem => 0b110, MOp::Remu => 0b111,
        };
        let funct7 = 0b0000001;  // 扩展 M 标识
        let code = (funct7 << 25)
                 | ((rs2.0 as u32) << 20)
                 | ((rs1.0 as u32) << 15)
                 | (funct3 << 12)
                 | ((rd.0 as u32) << 7)
                 | (Opcode::Op as u32);
        Insn(code)
    }

    // ═══════════ A 扩展 (原子) — opcode=0101111 ═══════════
    //
    // 格式: funct5[31:27] | aq(26) | rl(25) | rs2[24:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]
    // funct3: 010=32-bit, 011=64-bit
    // funct5: 00010=LR, 00011=SC, 00001=AMOSWAP, 00000=AMOADD, 00100=AMOXOR,
    //         01100=AMOAND, 01000=AMOOR, 10000=AMOMIN, 10100=AMOMAX,
    //         11000=AMOMINU, 11100=AMOMAXU

    pub fn amo(op: AOp, rd: Reg, rs1: Reg, rs2: Reg) -> Self {
        let (funct5, funct3) = match op {
            AOp::LrW      => (0b00010, 0b010),
            AOp::LrD      => (0b00010, 0b011),
            AOp::ScW      => (0b00011, 0b010),
            AOp::ScD      => (0b00011, 0b011),
            AOp::AmoSwapW => (0b00001, 0b010),
            AOp::AmoSwapD => (0b00001, 0b011),
            AOp::AmoAddW  => (0b00000, 0b010),
            AOp::AmoAddD  => (0b00000, 0b011),
            AOp::AmoXorW  => (0b00100, 0b010),
            AOp::AmoXorD  => (0b00100, 0b011),
            AOp::AmoAndW  => (0b01100, 0b010),
            AOp::AmoAndD  => (0b01100, 0b011),
            AOp::AmoOrW   => (0b01000, 0b010),
            AOp::AmoOrD   => (0b01000, 0b011),
            AOp::AmoMinW  => (0b10000, 0b010),
            AOp::AmoMinD  => (0b10000, 0b011),
            AOp::AmoMaxW  => (0b10100, 0b010),
            AOp::AmoMaxD  => (0b10100, 0b011),
            AOp::AmoMinuW => (0b11000, 0b010),
            AOp::AmoMinuD => (0b11000, 0b011),
            AOp::AmoMaxuW => (0b11100, 0b010),
            AOp::AmoMaxuD => (0b11100, 0b011),
        };
        let code = (funct5 << 27)
                 | ((rs2.0 as u32) << 20)
                 | ((rs1.0 as u32) << 15)
                 | (funct3 << 12)
                 | ((rd.0 as u32) << 7)
                 | (Opcode::Amo as u32);
        Insn(code)
    }

    // ═══════════ 汇编编解码 ═══════════

    /// 从汇编字符串编码 (仅 RV32I 基础指令)
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
            "ld"    => Self::load(LoadOp::Ld,  Self::rd(tokens[1])?, Self::rd(tokens[3])?, parse_imm(tokens[2])?),
            "lwu"   => Self::load(LoadOp::Lwu, Self::rd(tokens[1])?, Self::rd(tokens[3])?, parse_imm(tokens[2])?),
            "sb"    => Self::store(StoreOp::Sb, Self::rd(tokens[1])?, Self::rd(tokens[3])?, parse_imm(tokens[2])?),
            "sh"    => Self::store(StoreOp::Sh, Self::rd(tokens[1])?, Self::rd(tokens[3])?, parse_imm(tokens[2])?),
            "sw"    => Self::store(StoreOp::Sw, Self::rd(tokens[1])?, Self::rd(tokens[3])?, parse_imm(tokens[2])?),
            "sd"    => Self::store(StoreOp::Sd, Self::rd(tokens[1])?, Self::rd(tokens[3])?, parse_imm(tokens[2])?),
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
            // ── RV64I 字宽指令 ──
            "addw"  => Self::r32(ROp32::Addw, Self::rd(tokens[1])?, Self::rd(tokens[2])?, Self::rd(tokens[3])?),
            "subw"  => Self::r32(ROp32::Subw, Self::rd(tokens[1])?, Self::rd(tokens[2])?, Self::rd(tokens[3])?),
            "sllw"  => Self::r32(ROp32::Sllw, Self::rd(tokens[1])?, Self::rd(tokens[2])?, Self::rd(tokens[3])?),
            "srlw"  => Self::r32(ROp32::Srlw, Self::rd(tokens[1])?, Self::rd(tokens[2])?, Self::rd(tokens[3])?),
            "sraw"  => Self::r32(ROp32::Sraw, Self::rd(tokens[1])?, Self::rd(tokens[2])?, Self::rd(tokens[3])?),
            "addiw" => Self::i32(IOp32::Addiw, Self::rd(tokens[1])?, Self::rd(tokens[2])?, parse_imm(tokens[3])?),
            "slliw" => Self::i32(IOp32::Slliw, Self::rd(tokens[1])?, Self::rd(tokens[2])?, parse_imm(tokens[3])?),
            "srliw" => Self::i32(IOp32::Srliw, Self::rd(tokens[1])?, Self::rd(tokens[2])?, parse_imm(tokens[3])?),
            "sraiw" => Self::i32(IOp32::Sraiw, Self::rd(tokens[1])?, Self::rd(tokens[2])?, parse_imm(tokens[3])?),
            // ── M 扩展 ──
            "mul"    => Self::m_op(MOp::Mul,    Self::rd(tokens[1])?, Self::rd(tokens[2])?, Self::rd(tokens[3])?),
            "mulh"   => Self::m_op(MOp::Mulh,   Self::rd(tokens[1])?, Self::rd(tokens[2])?, Self::rd(tokens[3])?),
            "mulhsu" => Self::m_op(MOp::Mulhsu, Self::rd(tokens[1])?, Self::rd(tokens[2])?, Self::rd(tokens[3])?),
            "mulhu"  => Self::m_op(MOp::Mulhu,  Self::rd(tokens[1])?, Self::rd(tokens[2])?, Self::rd(tokens[3])?),
            "div"    => Self::m_op(MOp::Div,    Self::rd(tokens[1])?, Self::rd(tokens[2])?, Self::rd(tokens[3])?),
            "divu"   => Self::m_op(MOp::Divu,   Self::rd(tokens[1])?, Self::rd(tokens[2])?, Self::rd(tokens[3])?),
            "rem"    => Self::m_op(MOp::Rem,    Self::rd(tokens[1])?, Self::rd(tokens[2])?, Self::rd(tokens[3])?),
            "remu"   => Self::m_op(MOp::Remu,   Self::rd(tokens[1])?, Self::rd(tokens[2])?, Self::rd(tokens[3])?),
            // ── A 扩展 (原子) ──
            "lr.w"        => Self::amo(AOp::LrW,      Self::rd(tokens[1])?, Self::rd(tokens[2])?, Reg(0)),
            "sc.w"        => Self::amo(AOp::ScW,      Self::rd(tokens[1])?, Self::rd(tokens[3])?, Self::rd(tokens[2])?),
            "amoswap.w"   => Self::amo(AOp::AmoSwapW, Self::rd(tokens[1])?, Self::rd(tokens[3])?, Self::rd(tokens[2])?),
            "amoadd.w"    => Self::amo(AOp::AmoAddW,  Self::rd(tokens[1])?, Self::rd(tokens[3])?, Self::rd(tokens[2])?),
            "amoxor.w"    => Self::amo(AOp::AmoXorW,  Self::rd(tokens[1])?, Self::rd(tokens[3])?, Self::rd(tokens[2])?),
            "amoand.w"    => Self::amo(AOp::AmoAndW,  Self::rd(tokens[1])?, Self::rd(tokens[3])?, Self::rd(tokens[2])?),
            "amoor.w"     => Self::amo(AOp::AmoOrW,   Self::rd(tokens[1])?, Self::rd(tokens[3])?, Self::rd(tokens[2])?),
            "amomin.w"    => Self::amo(AOp::AmoMinW,  Self::rd(tokens[1])?, Self::rd(tokens[3])?, Self::rd(tokens[2])?),
            "amomax.w"    => Self::amo(AOp::AmoMaxW,  Self::rd(tokens[1])?, Self::rd(tokens[3])?, Self::rd(tokens[2])?),
            "amominu.w"   => Self::amo(AOp::AmoMinuW, Self::rd(tokens[1])?, Self::rd(tokens[3])?, Self::rd(tokens[2])?),
            "amomaxu.w"   => Self::amo(AOp::AmoMaxuW, Self::rd(tokens[1])?, Self::rd(tokens[3])?, Self::rd(tokens[2])?),
            _ => return None,
        })
    }

    /// 解码为汇编字符串 (全扩展)
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

        match opcode {
            // ── RV32I R-type ──
            0b0110011 if funct7 == 0b0000001 => {
                // M 扩展 (funct7=1)
                let name = match funct3 {
                    0b000 => "mul", 0b001 => "mulh", 0b010 => "mulhsu", 0b011 => "mulhu",
                    0b100 => "div", 0b101 => "divu", 0b110 => "rem", 0b111 => "remu",
                    _ => "mop???",
                };
                format!("{} {}, {}, {}", name, rd.name(), rs1.name(), rs2.name())
            },
            0b0110011 => {
                let name = match funct3 {
                    0b000 => if funct7 == 0b0100000 { "sub" } else { "add" },
                    0b001 => "sll", 0b010 => "slt", 0b011 => "sltu",
                    0b100 => "xor",
                    0b101 => if funct7 == 0b0100000 { "sra" } else { "srl" },
                    0b110 => "or", 0b111 => "and",
                    _ => "unknown_R",
                };
                format!("{} {}, {}, {}", name, rd.name(), rs1.name(), rs2.name())
            }
            // ── RV64I OP-32 (funct7 高位可能非零, 区分标识: opcode=0111011) ──
            0b0111011 => {
                let name = match funct3 {
                    0b000 => if funct7 == 0b0100000 { "subw" } else { "addw" },
                    0b001 => "sllw",
                    0b101 => if funct7 == 0b0100000 { "sraw" } else { "srlw" },
                    _ => "op32_???",
                };
                format!("{} {}, {}, {}", name, rd.name(), rs1.name(), rs2.name())
            }
            // ── RV32I I-type ──
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
            // ── RV64I OP-IMM-32 ──
            0b0011011 => match funct3 {
                0b000 => format!("addiw {}, {}, {}", rd.name(), rs1.name(), sign_extend_12((code as i32) >> 20)),
                0b001 => format!("slliw {}, {}, {}", rd.name(), rs1.name(), rs2_idx),
                0b101 => {
                    let n = if funct7 == 0b0100000 { "sraiw" } else { "srliw" };
                    format!("{} {}, {}, {}", n, rd.name(), rs1.name(), rs2_idx)
                }
                _ => format!("unknown_I32_{:03b}", funct3),
            },
            // ── Load ──
            0b0000011 => {
                let imm = sign_extend_12((code as i32) >> 20);
                let name = match funct3 {
                    0b000 => "lb", 0b001 => "lh", 0b010 => "lw",
                    0b100 => "lbu", 0b101 => "lhu",
                    0b011 => "ld", 0b110 => "lwu",
                    _ => "unknown_load",
                };
                format!("{} {}, {}({})", name, rd.name(), imm, rs1.name())
            }
            // ── Store ──
            0b0100011 => {
                let imm = sign_extend_12(((funct7 as i32) << 5) | (rd_idx as i32));
                let name = match funct3 {
                    0b000 => "sb", 0b001 => "sh", 0b010 => "sw",
                    0b011 => "sd",
                    _ => "unknown_store",
                };
                format!("{} {}, {}({})", name, rs2.name(), imm, rs1.name())
            }
            // ── Branch ──
            0b1100011 => {
                let imm = decode_b_imm(code);
                let name = match funct3 {
                    0b000 => "beq", 0b001 => "bne", 0b100 => "blt",
                    0b101 => "bge", 0b110 => "bltu", 0b111 => "bgeu",
                    _ => "unknown_branch",
                };
                format!("{} {}, {}, {}", name, rs1.name(), rs2.name(), imm)
            }
            // ── LUI / AUIPC ──
            0b0110111 => format!("lui {}, 0x{:05x}", rd.name(), (code >> 12) & 0xFFFFF),
            0b0010111 => format!("auipc {}, 0x{:05x}", rd.name(), (code >> 12) & 0xFFFFF),
            // ── JAL / JALR ──
            0b1101111 => format!("jal {}, {}", rd.name(), decode_j_imm(code)),
            0b1100111 => format!("jalr {}, {}({})", rd.name(), sign_extend_12((code as i32) >> 20), rs1.name()),
            // ── SYSTEM ──
            0b1110011 => if funct3 == 0 && rd_idx == 0 && rs1_idx == 0 {
                match code & 0xFFFFF000 {
                    0x00000000 => "ecall".to_string(),
                    0x00100000 => "ebreak".to_string(),
                    _ => format!("system {:08x}", code),
                }
            } else { "system".to_string() },
            // ── A 扩展 ──
            0b0101111 => {
                let width = if funct3 == 0b011 { ".d" } else { ".w" };  // RV32: only .w, funct3=010
                let funct5 = (code >> 27) & 0x1F;
                let name = if funct5 == 0b00010 { "lr".to_string() }
                    else if funct5 == 0b00011 { "sc".to_string() }
                    else if funct5 == 0b00001 { "amoswap".to_string() }
                    else if funct5 == 0b00000 { "amoadd".to_string() }
                    else if funct5 == 0b00100 { "amoxor".to_string() }
                    else if funct5 == 0b01100 { "amoand".to_string() }
                    else if funct5 == 0b01000 { "amoor".to_string() }
                    else if funct5 == 0b10000 { "amomin".to_string() }
                    else if funct5 == 0b10100 { "amomax".to_string() }
                    else if funct5 == 0b11000 { "amominu".to_string() }
                    else if funct5 == 0b11100 { "amomaxu".to_string() }
                    else { format!("amo_{:05b}", funct5) };
                match funct5 {
                    // SC和AMO格式: rd, rs2, (rs1)
                    _ => format!("{} {}, {}, {}({})", name, width, rd.name(), rs2.name(), rs1.name()),
                }
            },
            _ => format!("unknown_{:07b}_{:08x}", opcode, code),
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
        assert_eq!(Insn::encode("add x0, x1, x2").unwrap().0, 0x00208033);
        assert_eq!(Insn::encode("sub x0, x1, x2").unwrap().0, 0x40208033);
        assert_eq!(Insn::encode("addi x0, x1, 42").unwrap().0, 0x02A08013);
        assert_eq!(Insn::encode("lw x0, 8(x1)").unwrap().0, 0x0080A003);
        assert_eq!(Insn::encode("sw x0, 0(x1)").unwrap().0, 0x0000A023);
        assert_eq!(Insn::encode("addi x0, x0, 0").unwrap().0, 0x00000013);
        assert_eq!(Insn::encode("jal x0, 0").unwrap().0, 0x0000006F);
    }

    // ── RV64I 测试 ──
    #[test]
    fn rv64i_addiw() {
        // addiw: OP-IMM-32 opcode=0011011=0x1B, funct3=000
        // addiw x1, x2, 42 → imm=42, rs1=2, funct3=000, rd=1
        let i = Insn::i32(IOp32::Addiw, Reg(1), Reg(2), 42);
        assert_eq!(i.0, 0x02A1009B);
    }

    #[test]
    fn rv64i_addw() {
        // addw x1, x2, x3: OP-32 opcode=0111011=0x3B, funct3=000, funct7=0000000
        let i = Insn::r32(ROp32::Addw, Reg(1), Reg(2), Reg(3));
        assert_eq!(i.0, 0x003100BB);
    }

    #[test]
    fn rv64i_ld() {
        // ld x1, 0(x2): LOAD opcode=0x03, funct3=011 (Ld=3), rs1=2, rd=1, imm=0
        // = (0<<20) | (2<<15) | (3<<12) | (1<<7) | 0x03 = 0x00013083 = 0x13083
        let i = Insn::load(LoadOp::Ld, Reg(1), Reg(2), 0);
        assert_eq!(i.0, 0x00013083);
        let dec = i.decode();
        assert!(dec.contains("ld") && dec.contains("x1") && dec.contains("x2"), "decoded: {}", dec);
    }

    #[test]
    fn rv64i_sd() {
        // sd x1, 0(x2): STORE opcode=0100011, funct3=011
        // imm=0 → imm_hi=0, imm_lo=0
        // raw = (0<<25) | (1<<20) | (2<<15) | (3<<12) | (0<<7) | 0b0100011
        //     = 0x00100000 | 0x10000 | 0x3000 | 0x23 = 0x00113023
        let i = Insn::store(StoreOp::Sd, Reg(1), Reg(2), 0);
        assert_eq!(i.0, 0x00113023);
    }

    #[test]
    fn rv64i_decode_addw() {
        // addw encoding: opcode=0111011=0x3B
        let i = Insn::r32(ROp32::Addw, Reg(1), Reg(2), Reg(3));
        assert_eq!(i.decode(), "addw x1, x2, x3");
        assert_eq!(Insn(i.0).decode(), "addw x1, x2, x3");
    }

    #[test]
    fn rv64i_decode_subw() {
        // subw: funct7=0100000 → bit30=1
        // raw = 0x4031009B
        let i = Insn::r32(ROp32::Subw, Reg(1), Reg(2), Reg(3));
        assert_eq!(i.decode(), "subw x1, x2, x3");
    }

    // ── M 扩展测试 ──
    #[test]
    fn m_ext_mul() {
        // mul x1, x2, x3: OP opcode=0110011, funct7=0000001 (bit 25 set), funct3=000
        // raw = (1<<25) | (3<<20) | (2<<15) | (0<<12) | (1<<7) | 0b0110011
        //     = 0x02000000 | 0x00310000 | 0x0080 | 0x33 = 0x023100B3
        let i = Insn::m_op(MOp::Mul, Reg(1), Reg(2), Reg(3));
        assert_eq!(i.0, 0x023100B3);
        assert_eq!(i.decode(), "mul x1, x2, x3");
    }

    #[test]
    fn m_ext_divu() {
        // divu x4, x5, x6: funct3=101, funct7=0000001 (bit 25)
        let i = Insn::m_op(MOp::Divu, Reg(4), Reg(5), Reg(6));
        assert_eq!(i.0, 0x0262D233);
        assert_eq!(i.decode(), "divu x4, x5, x6");
    }

    // ── A 扩展测试 ──
    #[test]
    fn a_ext_lr_w() {
        // lr.w x0, (x1): opcode=0101111, funct5=00010, funct3=010, rs1=1, rs2=0, rd=0
        // raw = (2<<27) | (0<<20) | (1<<15) | (2<<12) | (0<<7) | 0x2F
        //     = 0x10000000 | 0x8000 | 0x2000 | 0x2F = 0x1000A02F
        let i = Insn::amo(AOp::LrW, Reg(0), Reg(1), Reg(0));
        assert_eq!(i.0, 0x1000A02F);
    }

    #[test]
    fn a_ext_sc_w() {
        // sc.w x2, x3, (x1): funct5=00011, funct3=010, rs2=3, rs1=1, rd=2
        // raw = (3<<27) | (3<<20) | (1<<15) | (2<<12) | (2<<7) | 0x2F
        let i = Insn::amo(AOp::ScW, Reg(2), Reg(1), Reg(3));
        // 0x18000000 | 0x00300000 = 0x18300000 | 0x8000 = 0x18308000
        // | 0x2000 = 0x1830A000 | 0x100 = 0x1830A100 | 0x2F = 0x1830A12F
        assert_eq!(i.0, 0x1830A12F);
    }

    #[test]
    fn a_ext_amoadd_w() {
        // amoadd.w x2, x4, (x3): funct5=00000, rs2=4, rs1=3, rd=2
        // raw = (0<<27) | (4<<20) | (3<<15) | (2<<12) | (2<<7) | 0x2F
        let i = Insn::amo(AOp::AmoAddW, Reg(2), Reg(3), Reg(4));
        // 0x00400000 | 0x00018000 = 0x00418000
        // | 0x2000 = 0x0041A000 | 0x100 = 0x0041A100 | 0x2F = 0x0041A12F
        assert_eq!(i.0, 0x0041A12F);
    }

    #[test]
    fn a_ext_decode() {
        // amoswap.w x2, x4, (x3): funct5=00001, funct3=010, rs2=4, rs1=3, rd=2
        let raw = Insn::amo(AOp::AmoSwapW, Reg(2), Reg(3), Reg(4)).0;
        let decoded = Insn(raw).decode();
        assert!(decoded.contains("amoswap"), "decoded: {}", decoded);
        assert!(decoded.contains("x4"), "decoded: {}", decoded);
    }
}
