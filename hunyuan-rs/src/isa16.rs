// ══════════════════════════════════════════════════════════════════════
// isa16 — RISC-V C 扩展 (压缩指令 · 16-bit) 编码器
// ══════════════════════════════════════════════════════════════════════
//
// 压缩指令格式 (16-bit 定长)：Q0/Q1/Q2 三大象限
// Q0 (000): c.addi4spn / c.lw / c.sw
// Q1 (001): c.addi / c.li / c.lui / c.slli / c.j / c.jal / c.beqz / c.bnez /
//           c.srli / c.srai / c.andi / c.sub / c.xor / c.or / c.and
// Q2 (010): c.lwsp / c.swsp / c.jr / c.mv / c.ebreak / c.jalr / c.add

/// 16 位压缩指令
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Insn16(pub u16);

impl Insn16 {
    /// c.nop = addi x0, x0, 0 (1 00000 00000 00 00001
    pub fn nop() -> Self {
        // nzimm=0, rd=x0=0 → bits (0)(00000)(00000)(00)(00001) = 0x0001
        Insn16(0x0001)
    }

    /// c.addi rd, nzimm (rd ≠ 0, nzimm 6-bit signed)
    /// Encoding: nzimm[5] | rd[4:0] | nzimm[4:0] | funct3=000 | op=01
    pub fn addi(rd: u8, nzimm: i8) -> Self {
        assert!(rd <= 31 && rd != 0);
        let u = nzimm as u16 & 0x3F;
        let nz5 = (u >> 5) & 0x1;
        let nz4_0 = u & 0x1F;
        Insn16((nz5 << 12) | ((rd as u16) << 7) | (nz4_0 << 2) | 0x01)
    }

    /// c.li rd, imm (rd ≠ 0)
    /// Encoding: imm[5] | rd[4:0] | imm[4:0] | funct3=010 | op=01
    pub fn li(rd: u8, imm: i8) -> Self {
        assert!(rd <= 31 && rd != 0);
        let u = imm as u16 & 0x3F;
        let imm5 = (u >> 5) & 0x1;
        let imm4_0 = u & 0x1F;
        Insn16((imm5 << 12) | ((rd as u16) << 7) | (imm4_0 << 2) | 0x09)
    }

    /// c.slli rd, shamt (rd ≠ 0, shamt < 32)
    /// Encoding: nzuimm[5] | rd[4:0] | nzuimm[4:0] | funct3=000 | op=10
    pub fn slli(rd: u8, shamt: u8) -> Self {
        assert!(rd <= 31 && rd != 0 && (shamt as u32) < 32);
        let nzuimm = (shamt as u16) & 0x1F;
        let nzuimm5 = ((shamt as u16) >> 5) & 0x1;
        Insn16((nzuimm5 << 12) | ((rd as u16) << 7) | (nzuimm << 2) | 0x21)
    }

    /// c.srli rd', shamt  (rd' 编码在 [9:7] 的伪寄存器 8~15, funct2=00)
    pub fn srli(rd: u8, shamt: u8) -> Self {
        assert!(rd >= 8 && rd <= 15 && (shamt as u32) < 32);
        let rd3 = rd & 0x7;
        let nzuimm = (shamt as u16) & 0x1F;
        let funct1 = ((shamt as u16) >> 5) & 0x1;
        Insn16((funct1 << 12) | (0b00 << 10) | (rd3 << 7) | (nzuimm << 2) | 0x01)
    }

    /// c.srai rd', shamt  (rd' 编码在 [9:7], funct2=01)
    pub fn srai(rd: u8, shamt: u8) -> Self {
        assert!(rd >= 8 && rd <= 15 && (shamt as u32) < 32);
        let rd3 = rd & 0x7;
        let nzuimm = (shamt as u16) & 0x1F;
        let funct1 = (((shamt as u16) >> 5) & 0x1) | 0b01;
        Insn16((funct1 << 12) | (0b01 << 10) | (rd3 << 7) | (nzuimm << 2) | 0x01)
    }

    /// c.andi rd', imm (imm 6-bit signed)
    pub fn andi(rd: u8, imm: i8) -> Self {
        assert!(rd >= 8 && rd <= 15);
        let rd3 = rd & 0x7;
        let u = imm as u16 & 0x3F;
        let imm5 = (u >> 5) & 0x1;
        let imm4_0 = u & 0x1F;
        Insn16((imm5 << 12) | (0b10 << 10) | (rd3 << 7) | (imm4_0 << 2) | 0x01)
    }

    /// c.sub rd', rs2'
    pub fn sub(rd: u8, rs2: u8) -> Self {
        assert!(rd >= 8 && rd <= 15 && rs2 >= 8 && rs2 <= 15);
        let rd3 = rd & 0x7;
        let rs2_3 = rs2 & 0x7;
        Insn16(((rd3 as u16) << 7) | ((rs2_3 as u16) << 2) | 0x01)
    }

    /// c.xor rd', rs2'
    pub fn xor(rd: u8, rs2: u8) -> Self {
        assert!(rd >= 8 && rd <= 15 && rs2 >= 8 && rs2 <= 15);
        let rd3 = rd & 0x7;
        let rs2_3 = rs2 & 0x7;
        Insn16((0b1001 << 13) | (0b01u16 << 10) | ((rd3 as u16) << 7)
             | ((rs2_3 as u16) << 2) | 0x01)
    }

    /// c.or rd', rs2'
    pub fn or(rd: u8, rs2: u8) -> Self {
        assert!(rd >= 8 && rd <= 15 && rs2 >= 8 && rs2 <= 15);
        let rd3 = rd & 0x7;
        let rs2_3 = rs2 & 0x7;
        Insn16((0b1001 << 13) | (0b10u16 << 10) | ((rd3 as u16) << 7)
             | ((rs2_3 as u16) << 2) | 0x01)
    }

    /// c.and rd', rs2'
    pub fn and(rd: u8, rs2: u8) -> Self {
        assert!(rd >= 8 && rd <= 15 && rs2 >= 8 && rs2 <= 15);
        let rd3 = rd & 0x7;
        let rs2_3 = rs2 & 0x7;
        Insn16((0b1001 << 13) | (0b11u16 << 10) | ((rd3 as u16) << 7)
             | ((rs2_3 as u16) << 2) | 0x01)
    }

    /// c.lw rd', uimm(rs1')  uimm: 7-bit (2|6|5:3)
    /// Encoding: funct3=010 | uimm[5:3] | rs1'[9:7] | uimm[2|6] | rd'[4:2] | op=00
    pub fn lw(rd: u8, rs1: u8, uimm: u8) -> Self {
        assert!(rd >= 8 && rd <= 15 && rs1 >= 8 && rs1 <= 15);
        let rd3 = rd & 0x7;
        let rs1_3 = rs1 & 0x7;
        let uimm5_3 = (uimm >> 3) & 0x7;
        let uimm2   = (uimm >> 2) & 0x1;
        let uimm6   = (uimm >> 6) & 0x1;
        Insn16((0b010u16 << 13) | (uimm5_3 << 10) | ((rs1_3 as u16) << 7)
             | (uimm2 << 6) | (uimm6 << 5) | ((rd3 as u16) << 2) | 0x00)
    }

    /// c.sw rs2', uimm(rs1')
    pub fn sw(rs2: u8, rs1: u8, uimm: u8) -> Self {
        assert!(rs2 >= 8 && rs2 <= 15 && rs1 >= 8 && rs1 <= 15);
        let rs2_3 = rs2 & 0x7;
        let rs1_3 = rs1 & 0x7;
        let uimm5_3 = (uimm >> 3) & 0x7;
        let uimm2   = (uimm >> 2) & 0x1;
        let uimm6   = (uimm >> 6) & 0x1;
        Insn16((0b110u16 << 13) | (uimm5_3 << 10) | ((rs1_3 as u16) << 7)
             | (uimm2 << 6) | (uimm6 << 5) | ((rs2_3 as u16) << 2) | 0x00)
    }

    /// c.addi4spn rd', nzuimm[5|4|9:6|2|3]
    pub fn addi4spn(rd: u8, nzuimm: u16) -> Self {
        assert!(rd >= 8 && rd <= 15);
        let imm9_6 = (nzuimm >> 6) & 0xF;
        let imm5_4 = (nzuimm >> 4) & 0x3;
        let imm3   = (nzuimm >> 3) & 0x1;
        let imm2   = (nzuimm >> 2) & 0x1;
        let rd3 = rd & 0x7;
                Insn16((imm5_4 << 11) | (imm9_6 << 7) | (imm3 << 6) | (imm2 << 5) | ((rd3 as u16) << 2))
    }

    /// c.lwsp rd, uimm(x2)  rd ≠ 0, uimm[7:2|7:6]
    /// Encoding: funct3=010 | uimm[5] | rd[4:0] | uimm[4:2|7:6] | op=10
    pub fn lwsp(rd: u8, uimm: u8) -> Self {
        assert!(rd <= 31 && rd != 0);
        let imm5    = (uimm >> 5) & 0x1;
        let imm4_2  = (uimm >> 2) & 0x7;
        let imm7_6  = (uimm >> 6) & 0x3;
        Insn16((0b010u16 << 13) | ((imm5 as u16) << 12) | ((rd as u16) << 7)
             | ((imm4_2 as u16) << 4) | ((imm7_6 as u16) << 2) | 0x02)
    }

    /// c.swsp rs2, uimm(x2)
    pub fn swsp(rs2: u8, uimm: u8) -> Self {
        assert!(rs2 <= 31);
        let imm5_2 = (uimm >> 2) & 0x1F;
        let imm7_6 = (uimm >> 6) & 0x3;
        Insn16((0b110u16 << 13) | ((imm5_2 as u16) << 7) | ((rs2 as u16) << 2) | 0x02)
    }

    /// c.jr rs1 (rs1 ≠ 0)
    /// Encoding: 0[12] | rs1[11:7] | 00000 | funct3=100 | op=10
    pub fn jr(rs1: u8) -> Self {
        assert!(rs1 <= 31 && rs1 != 0);
        Insn16(((rs1 as u16) << 7) | 0x0021)
    }

    /// c.jalr rs1 (rs1 ≠ 0)
    pub fn jalr(rs1: u8) -> Self {
        assert!(rs1 <= 31 && rs1 != 0);
        Insn16(((rs1 as u16) << 7) | 0x8025)  // bit 5 distinguishes from jr
    }

    /// c.add rd, rs2  (rd ≠ 0, rs2 ≠ 0)
    pub fn add(rd: u8, rs2: u8) -> Self {
        assert!(rd <= 31 && rd != 0 && rs2 <= 31 && rs2 != 0);
        Insn16(((rd as u16) << 7) | ((rs2 as u16) << 2) | 0x25 | (1 << 5))
    }

    /// c.mv rd, rs2 (rd ≠ 0, rs2 ≠ 0)
    pub fn mv(rd: u8, rs2: u8) -> Self {
        assert!(rd <= 31 && rs2 <= 31);
        Insn16(((rd as u16) << 7) | ((rs2 as u16) << 2) | 0x21)
    }

    /// c.ebreak
    pub fn ebreak() -> Self {
        Insn16(0x9002)  // 1001 00000 00000010
    }

    /// c.j offset (RV32 only, 11-bit signed)
    pub fn j(offset: i16) -> Self {
        let o = offset as u16 & 0x7FF;
        let imm10_1 = (o >> 1) & 0x3FF;
        let imm11   = (o >> 11) & 0x1;
        let imm5    = (o >> 5) & 0x1;
        let imm4_3  = (o >> 3) & 0x3;
        let imm7    = (o >> 7) & 0x1;
        let imm9_8  = (o >> 8) & 0x3;
        let imm6    = (o >> 6) & 0x1;
        Insn16((0b101u16 << 13)
             | (imm11 << 12)
             | (imm10_1 << 6)
             | 0x05)
    }

    /// c.jal offset (RV32 only, 11-bit signed)
    pub fn jal(offset: i16) -> Self { Self::j(offset) } // same bits, Q1 vs Q0

    /// c.beqz rs1', offset (8-bit signed)
    pub fn beqz(rs1: u8, offset: i8) -> Self {
        assert!(rs1 >= 8 && rs1 <= 15);
        let rs1_3 = rs1 & 0x7;
        let o = offset as u8;
        let imm8   = (o >> 8) as u8 & 0x1;
        let imm7_6 = (o >> 6) & 0x3;
        let imm5   = (o >> 5) & 0x1;
        let imm4_3 = (o >> 3) & 0x3;
        let imm2_1 = (o >> 1) & 0x3;
        Insn16((0b110u16 << 13)
             | (imm8 << 12)
             | (imm7_6 << 10)
             | ((rs1_3 as u16) << 7)
             | (imm5 << 6)
             | (imm4_3 << 5)
             | (imm2_1 << 3)
             | 0x01)
    }

    /// c.bnez rs1', offset (8-bit signed)
    pub fn bnez(rs1: u8, offset: i8) -> Self {
        assert!(rs1 >= 8 && rs1 <= 15);
        let rs1_3 = rs1 & 0x7;
        let o = offset as u8;
        let imm8   = (o >> 8) as u8 & 0x1;
        let imm7_6 = (o >> 6) & 0x3;
        let imm5   = (o >> 5) & 0x1;
        let imm4_3 = (o >> 3) & 0x3;
        let imm2_1 = (o >> 1) & 0x3;
        Insn16((0b111u16 << 13)
             | (imm8 << 12)
             | (imm7_6 << 10)
             | ((rs1_3 as u16) << 7)
             | (imm5 << 6)
             | (imm4_3 << 5)
             | (imm2_1 << 3)
             | 0x01)
    }

    /// 解码 16 位压缩指令
    pub fn decode(self) -> String {
        let raw = self.0;
        let op = raw & 0x3;
        let funct3 = (raw >> 13) & 0x7;

        match (op, funct3) {
            // ── Q0 ──
            (0b00, 0b000) => {
                let rd3 = ((raw >> 2) & 0x7) + 8;
                let imm = ((raw >> 5) & 0x3 << 4)
                        | ((raw >> 11) & 0xF << 6)
                        | ((raw >> 6) & 0x1 << 2)
                        | ((raw >> 5) & 0x1 << 3);
                format!("c.addi4spn x{}, {}", rd3, imm)
            },
            (0b00, 0b001) => "c.flw ...".to_string(),  // RV32DC only
            (0b00, 0b010) => {
                let rd3 = ((raw >> 2) & 0x7) + 8;
                let rs1_3 = ((raw >> 7) & 0x7) + 8;
                let uimm = ((raw >> 10) & 0x7 << 3)
                         | ((raw >> 6) & 0x1 << 2)
                         | ((raw >> 5) & 0x1 << 6);
                format!("c.lw x{}, {}(x{})", rd3, uimm as i32, rs1_3)
            },
            (0b00, 0b011) => "c.flw ...".to_string(),
            (0b00, 0b110) => {
                let rs2_3 = ((raw >> 2) & 0x7) + 8;
                let rs1_3 = ((raw >> 7) & 0x7) + 8;
                let uimm = ((raw >> 10) & 0x7 << 3)
                         | ((raw >> 6) & 0x1 << 2)
                         | ((raw >> 5) & 0x1 << 6);
                format!("c.sw x{}, {}(x{})", rs2_3, uimm as i32, rs1_3)
            },
            (0b00, 0b111) => "c.fsw ...".to_string(),

            // ── Q1 ──
            (0b01, 0b000) => {
                let rd = (raw >> 7) & 0x1F;
                let nzimm5 = (raw as i16 >> 12) as i8 & 0x1;
                let nzimm4_0 = ((raw >> 2) & 0x1F) as i8;
                let nzimm = (nzimm5 << 5) | nzimm4_0;
                format!("c.addi x{}, {}", rd, nzimm)
            },
            (0b01, 0b001) => "c.jal ...".to_string(),  // RV32 only
            (0b01, 0b010) => {
                let rd = (raw >> 7) & 0x1F;
                let imm5 = (raw as i16 >> 12) as i8 & 0x1;
                let imm4_0 = ((raw >> 2) & 0x1F) as i8;
                let imm = (imm5 << 5) | imm4_0;
                format!("c.li x{}, {}", rd, imm)
            },
            (0b01, 0b011) => {
                let rd = (raw >> 7) & 0x1F;
                if rd == 2 {
                    let nzimm9 = (raw as i16 >> 12) as i8 & 0x1;
                    let nzimm8_7 = ((raw >> 3) & 0x3) as i8;
                    let nzimm6   = ((raw >> 5) & 0x1) as i8;
                    let nzimm5   = ((raw >> 2) & 0x1) as i8;
                    let nzimm4   = ((raw >> 6) & 0x1) as i8;
                    let nzimm = (nzimm9 << 9) | (nzimm8_7 << 7) | (nzimm6 << 6) |
                                 (nzimm5 << 5) | (nzimm4 << 4);
                    format!("c.addi16sp {}", nzimm)
                } else {
                    format!("c.lui x{}, 0x{:04x}", rd, (raw as u32) >> 12 & 0x1F)
                }
            },
            (0b01, 0b100) => {
                let funct2 = (raw >> 10) & 0x3;
                let rd3 = ((raw >> 7) & 0x7) + 8;
                let shamt = (raw >> 2) & 0x1F;
                match funct2 {
                    0b00 => format!("c.srli x{}, {}", rd3, shamt),
                    0b01 => format!("c.srai x{}, {}", rd3, shamt),
                    0b10 => {
                        let imm5 = ((raw >> 12) & 0x1) as i8;
                        let imm4_0 = ((raw >> 2) & 0x1F) as i8;
                        let imm = (imm5 << 5) | imm4_0;
                        format!("c.andi x{}, {}", rd3, imm)
                    },
                    _ => {
                        let op2 = (raw >> 5) & 0x3;
                        let rs2_3 = ((raw >> 2) & 0x7) + 8;
                        match op2 {
                            0b00 => format!("c.sub x{}, x{}", rd3, rs2_3),
                            0b01 => format!("c.xor x{}, x{}", rd3, rs2_3),
                            0b10 => format!("c.or x{}, x{}", rd3, rs2_3),
                            _   => format!("c.and x{}, x{}", rd3, rs2_3),
                        }
                    },
                }
            },
            (0b01, 0b101) => "c.j ...".to_string(),
            (0b01, 0b110) => format!("c.beqz x{}, ...", ((raw >> 7) & 0x7) + 8),
            (0b01, 0b111) => format!("c.bnez x{}, ...", ((raw >> 7) & 0x7) + 8),

            // ── Q2 ──
            (0b10, 0b000) => {
                let rd = (raw >> 7) & 0x1F;
                let shamt5 = (raw >> 12) & 0x1;
                let shamt = (((raw >> 6) & 0x1) << 5) | shamt5 << 5 | (raw >> 2) & 0x1F;
                format!("c.slli x{}, {}", rd, shamt)
            },
            (0b10, 0b001) => "c.flwsp ...".to_string(),
            (0b10, 0b010) => {
                let rd = (raw >> 7) & 0x1F;
                let uimm5 = (raw >> 12) & 0x1;
                let uimm4_2 = (raw >> 4) & 0x7;
                let uimm7_6 = (raw >> 2) & 0x3;
                let uimm = (uimm5 << 5) | (uimm4_2 << 2) | (uimm7_6 << 6);
                format!("c.lwsp x{}, {}", rd, uimm)
            },
            (0b10, 0b011) => "c.flsp ...".to_string(),
            (0b10, 0b100) => {
                let bit12 = (raw >> 12) & 0x1;
                let rs1 = (raw >> 7) & 0x1F;
                let rs2 = (raw >> 2) & 0x1F;
                match (bit12, rs1, rs2) {
                    (0, 0, _) => unreachable!(),  // reserved
                    (0, _, 0) => format!("c.jr x{}", rs1),
                    (0, _, _) => format!("c.mv x{}, x{}", rs1, rs2),
                    (1, 0, 0) => "c.ebreak".to_string(),
                    (1, _, 0) => format!("c.jalr x{}", rs1),
                    (1, _, _) => format!("c.add x{}, x{}", rs1, rs2),
                    _ => unreachable!(),
                }
            },
            (0b10, 0b101) => "c.fswsp ...".to_string(),
            (0b10, 0b110) => {
                let rs2 = (raw >> 2) & 0x1F;
                let uimm = (raw >> 7) & 0x3F;
                format!("c.swsp x{}, {}(x2)", rs2, uimm)
            },

            _ => format!("unknown_c 0x{:04x}", raw),
        }
    }

    pub fn hex(self) -> String { format!("{:04x}", self.0) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_nop_encode() {
        assert_eq!(Insn16::nop().0, 0x0001);
    }

    #[test]
    fn c_nop_decode() {
        let d = Insn16(0x0001).decode();
        assert!(d.contains("c.addi x0, 0"), "{}", d);
    }

    #[test]
    fn c_jr_x1() {
        // c.jr x1 = 0x01 00001 00000 000 00001 = 0x8082
        assert_eq!(Insn16::jr(1).0, 0x8082);
        assert_eq!(Insn16(0x8082).decode(), "c.jr x1");
    }

    #[test]
    fn c_jalr_x1() {
        // c.jalr x1 = 0x9082
        assert_eq!(Insn16::jalr(1).0, 0x9082);
        assert_eq!(Insn16(0x9082).decode(), "c.jalr x1");
    }

    #[test]
    fn c_ebreak() {
        assert_eq!(Insn16::ebreak().0, 0x9002);
        assert_eq!(Insn16(0x9002).decode(), "c.ebreak");
    }

    #[test]
    fn c_slli() {
        // c.slli x1, 5: nzuimm[5]=0, nzuimm[4:0]=5, rd=x1, funct3=000, op=10
        let i = Insn16::slli(1, 5);
        let d = i.decode();
        assert!(d.contains("c.slli x1, 5"), "{}", d);
    }

    #[test]
    fn c_addi_roundtrip() {
        // c.addi x1, 4
        let i = Insn16::addi(1, 4);
        let d = i.decode();
        assert!(d.contains("c.addi x1, 4"), "decoded: {}", d);
    }

    #[test]
    fn c_lw() {
        // c.lw x10, 0(x10): rd3=2 (x10), rs1_3=2 (x10), uimm=0
        let i = Insn16::lw(10, 10, 0);
        let d = i.decode();
        assert!(d.contains("c.lw x10, 0(x10)"), "decoded: {}", d);
    }

    #[test]
    fn c_sw() {
        // c.sw x10, 0(x10)
        let i = Insn16::sw(10, 10, 0);
        let d = i.decode();
        assert!(d.contains("c.sw x10, 0(x10)"), "decoded: {}", d);
    }

    #[test]
    fn c_mv() {
        // c.mv x1, x2
        let i = Insn16::mv(1, 2);
        let d = i.decode();
        assert_eq!(d, "c.mv x1, x2");

        // c.add x1, x2  (bit 5 = 1)
        let i2 = Insn16::add(1, 2);
        let d2 = i2.decode();
        assert_eq!(d2, "c.add x1, x2");
    }

    #[test]
    fn c_lwsp() {
        // c.lwsp x5, 0(x2)
        let i = Insn16::lwsp(5, 0);
        let d = i.decode();
        assert!(d.contains("c.lwsp x5"), "{}", d);
    }

    #[test]
    fn c_swsp() {
        // c.swsp x3, 0(x2)
        let i = Insn16::swsp(3, 0);
        let d = i.decode();
        assert!(d.contains("c.swsp x3"), "{}", d);
    }
}
