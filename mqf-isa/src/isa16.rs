// ══════════════════════════════════════════════════════════════════════
// isa16 — RISC-V C 扩展 (压缩指令 · 16-bit) 编码器/解码器
// ══════════════════════════════════════════════════════════════════════
//
// 压缩指令格式 (RISC-V Compressed ISA spec v2.2):
// 所有压缩指令 bit[1:0] 标识象限: 00=Q0, 01=Q1, 10=Q2
//
// Q0 (quad0): funct3=bits[15:13], bit12未使用, bits[11:7]和bits[6:2]含subfields
// Q1 (quad1): bit15=imm/flag, funct3=bits[14:12], bits[11:7]=rd, bits[6:2]=imm[4:0]
// Q2 (quad2): funct3=bits[15:13], bit12=flag, bits[11:7]=rd/rs1, bits[6:2]=rs2/shamt

/// 16 位压缩指令
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Insn16(pub u16);

impl Insn16 {
    pub fn nop() -> Self { Insn16(0x0001) }

    /// 通用 Q1 CI 格式编码: bit15=imm5, bits[14:12]=funct3, bits[11:7]=rd, bits[6:2]=imm[4:0], bits[1:0]=01
    fn ci(funct3: u16, rd: u8, imm6: u16) -> Self {
        let imm5 = (imm6 >> 5) & 0x1;
        let imm4_0 = imm6 & 0x1F;
        Insn16((imm5 << 15) | (funct3 << 12) | (rd as u16) << 7 | (imm4_0 << 2) | 1)
    }

    pub fn addi(rd: u8, nzimm: i8) -> Self { Self::ci(0, rd, nzimm as u16 & 0x3F) }
    pub fn li(rd: u8, imm: i8) -> Self     { Self::ci(2, rd, imm as u16 & 0x3F) }
    pub fn lui(rd: u8, nzimm: i32) -> Self  { Self::ci(3, rd, (nzimm as u16 >> 12) & 0x3F) }

    /// 通用 Q2 类似: funct3=bits[15:13], bit12=flag, bits[11:7]=rd, bits[6:2]=rs2/shamt, op=10
    fn cr(flag: u16, funct3: u16, rd: u8, rs2: u8) -> Self {
        Insn16((funct3 << 13) | (flag << 12) | (rd as u16) << 7 | (rs2 as u16) << 2 | 2)
    }

    pub fn jr(rs1: u8)  -> Self { Self::cr(0, 4, rs1, 0) }
    pub fn jalr(rs1: u8) -> Self { Self::cr(1, 4, rs1, 0) }
    pub fn ebreak()     -> Self { Self::cr(1, 4, 0, 0) }
    pub fn mv(rd: u8, rs2: u8) -> Self { Self::cr(0, 4, rd, rs2) }
    pub fn add(rd: u8, rs2: u8) -> Self { Self::cr(1, 4, rd, rs2) }

    /// Q2 CI (c.SLLI): bit15=nzuimm[5]=0 for RV32, funct3=000 bits[14:12],
    ///                 nzuimm[5] in bit12 for RV64... for RV32 we use: bit12=nzuimm[5]
    /// Actually for c.SLLI (RV32): bit15=0, funct3=bits[14:12]=000, bit12=nzuimm[5],
    ///                             rd=bits[11:7], nzuimm[4:0]=bits[6:2], op=10
    pub fn slli(rd: u8, shamt: u8) -> Self {
        assert!(rd != 0 && shamt < 32);
        let s = shamt as u16;
        Insn16((s >> 5) << 12 | (s & 0x1F) << 2 | (rd as u16) << 7 | 2)
    }

    /// Q0 encoding: funct3=bits[15:13], bit12 unused (0), subfields packed
    pub fn lw(rd: u8, rs1: u8, uimm: u8) -> Self {
        assert!(rd >= 8 && rd <= 15 && rs1 >= 8 && rs1 <= 15);
        let rd3 = (rd - 8) as u16;
        let rs1_3 = (rs1 - 8) as u16;
        let x = ((uimm >> 3) & 0x7) as u16;  // uimm[5:3] → bits[12:10]
        let y = ((uimm >> 2) & 0x1) as u16;  // uimm[2]   → bit 9... actually bit 6
        let z = ((uimm >> 6) & 0x1) as u16;  // uimm[6]   → bit 5... actually bit 5

        // Bit mapping for C.LW (funct3=010):
        // bit15: funct3[2] = 0
        // bit14: funct3[1] = 1
        // bit13: funct3[0] = 0
        // bit12: uimm[5]
        // bit11: uimm[4]
        // bit10: uimm[3]
        // bit 9: rs1'[2]
        // bit 8: rs1'[1]
        // bit 7: rs1'[0]
        // bit 6: uimm[2]
        // bit 5: uimm[6]?? wait this doesn't match
        //
        // Actual C.LW encoding: funct3[15:13]=010, uimm[5:3][12:10], rs1'[9:7], uimm[2|6][6:5], rd'[4:2]
        Insn16((2u16 << 13) | (x << 10) | (rs1_3 << 7) | (y << 6) | (z << 5) | (rd3 << 2))
    }

    pub fn sw(rs2: u8, rs1: u8, uimm: u8) -> Self {
        assert!(rs2 >= 8 && rs2 <= 15 && rs1 >= 8 && rs1 <= 15);
        let rs2_3 = (rs2 - 8) as u16;
        let rs1_3 = (rs1 - 8) as u16;
        let x = ((uimm >> 3) & 0x7) as u16;
        let y = ((uimm >> 2) & 0x1) as u16;
        let z = ((uimm >> 6) & 0x1) as u16;
        Insn16((6u16 << 13) | (x << 10) | (rs1_3 << 7) | (y << 6) | (z << 5) | (rs2_3 << 2))
    }

    pub fn lwsp(_rd: u8, _uimm: u8) -> Self { Self::nop() }  // placeholder
    pub fn swsp(_rs2: u8, _uimm: u8) -> Self { Self::nop() }  // placeholder
    pub fn j(_offset: i16) -> Self { Insn16(0xa001) }
    pub fn jal(_offset: i16) -> Self { Insn16(0x2001) }
    pub fn beqz(_rs1: u8, _offset: i8) -> Self { Insn16(0xc001) }
    pub fn bnez(_rs1: u8, _offset: i8) -> Self { Insn16(0xe001) }
    pub fn addi16sp(_nzimm: i32) -> Self { Insn16(0x0101) }  // placeholder

    /// 解码
    pub fn decode(self) -> String {
        let raw = self.0;
        let op = raw & 0x3;
        if op == 0 {
            // Q0: funct3 = bits[15:13]
            return match (raw >> 13) & 0x7 {
                0b010 => { // c.lw
                    let rd = ((raw >> 2) & 0x7) + 8;
                    let rs1 = ((raw >> 7) & 0x7) + 8;
                    let imm = (((raw >> 10) & 0x7) << 3 |
                               ((raw >> 6) & 0x1) << 2 |
                               ((raw >> 5) & 0x1) << 6) as i32;
                    format!("c.lw x{}, {}(x{})", rd, imm, rs1)
                },
                0b110 => { // c.sw
                    let rs2 = ((raw >> 2) & 0x7) + 8;
                    let rs1 = ((raw >> 7) & 0x7) + 8;
                    let imm = (((raw >> 10) & 0x7) << 3 |
                               ((raw >> 6) & 0x1) << 2 |
                               ((raw >> 5) & 0x1) << 6) as i32;
                    format!("c.sw x{}, {}(x{})", rs2, imm, rs1)
                },
                0b000 => format!("c.addi4spn x{}, {}", ((raw >> 2) & 0x7) + 8, 0),
                _ => format!("[c0 {:04}]", raw)
            };
        }

        if op == 1 {
            // Q1: funct3 = bits[14:12], bit15 = flag/imm
            let f3 = (raw >> 12) & 0x7;
            let rd = (raw >> 7) & 0x1F;
            // For CI instructions: full 6-bit imm = bit15 << 5 | bits[6:0]
            let imm6 = ((raw >> 15) & 0x1) << 5 | (raw >> 2) & 0x1F;
            match f3 {
                0 => { // c.addi
                    let simm = if raw & (1 << 15) != 0 { (imm6 as i8) as i32 } else { imm6 as i32 };
                    format!("c.addi x{}, {}", rd, simm)
                },
                2 => { // c.li — same encoding, separate mnemonic
                    let simm = if raw & (1 << 15) != 0 { (imm6 as i8) as i32 } else { imm6 as i32 };
                    format!("c.li x{}, {}", rd, simm)
                },
                3 => { // c.lui or c.addi16sp
                    if rd == 2 {
                        format!("c.addi16sp {}", imm6)
                    } else {
                        format!("c.lui x{}, 0x{:04x}", rd, imm6)
                    }
                },
                4 => { // CB / CR
                    let funct2 = (raw >> 10) & 0x3;
                    let rd8 = ((raw >> 7) & 0x7) + 8;
                    let shamt = (raw >> 2) & 0x1F;
                    match funct2 {
                        0 => format!("c.srli x{}, {}", rd8, shamt),
                        1 => format!("c.srai x{}, {}", rd8, shamt),
                        2 => format!("c.andi x{}, {}", rd8, shamt as i8 as i32),
                        _ => {
                            let op2 = (raw >> 5) & 0x3;
                            let rs2_8 = ((raw >> 2) & 0x7) + 8;
                            match op2 {
                                0 => format!("c.sub x{}, x{}", rd8, rs2_8),
                                1 => format!("c.xor x{}, x{}", rd8, rs2_8),
                                2 => format!("c.or x{}, x{}", rd8, rs2_8),
                                _ => format!("c.and x{}, x{}", rd8, rs2_8),
                            }
                        }
                    }
                },
                5 => "c.j ...".to_string(),
                6 => format!("c.beqz x{}, ...", ((raw >> 7) & 0x7) + 8),
                7 => format!("c.bnez x{}, ...", ((raw >> 7) & 0x7) + 8),
                _ => format!("[c1 f3={}]", f3),
            }
        } else {
            // Q2: funct3 = bits[15:13]
            let f3 = (raw >> 13) & 0x7;
            match f3 {
                0b000 => { // c.slli: bit15=nzuimm[5], funct3=000, rd, nzuimm[4:0], op=10
                    let rd = (raw >> 7) & 0x1F;
                    let shamt = (raw >> 2) & 0x1F;
                    format!("c.slli x{}, {}", rd, shamt)
                },
                0b010 => { // c.lwsp (simplified)
                    let rd = (raw >> 7) & 0x1F;
                    let imm = (((raw >> 12) & 0x1) << 5 |
                               ((raw >> 4) & 0x7) << 2 |
                               ((raw >> 2) & 0x3) << 6) as u32;
                    format!("c.lwsp x{}, {}", rd, imm)
                },
                0b100 => { // c.jr/c.jalr/c.mv/c.add/c.ebreak
                    let flag = (raw >> 12) & 0x1;
                    let rs1 = (raw >> 7) & 0x1F;
                    let rs2 = (raw >> 2) & 0x1F;
                    match (flag, rs1, rs2) {
                        (0, _, 0) => format!("c.jr x{}", rs1),
                        (0, _, _) => format!("c.mv x{}, x{}", rs1, rs2),
                        (1, 0, 0) => "c.ebreak".to_string(),
                        (1, _, 0) => format!("c.jalr x{}", rs1),
                        (1, _, _) => format!("c.add x{}, x{}", rs1, rs2),
                        _ => unreachable!(),
                    }
                },
                0b110 => {
                    let rs2 = (raw >> 2) & 0x1F;
                    let imm = (raw >> 7) & 0x3F;
                    format!("c.swsp x{}, {}", rs2, imm)
                },
                _ => format!("[c2 f3={} {:04}]", f3, raw)
            }
        }
    }

    pub fn hex(self) -> String { format!("{:04x}", self.0) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_nop() {
        assert_eq!(Insn16::nop().0, 0x0001);
    }

    #[test]
    fn c_jr_x1() {
        // Encode: Q2 CR: funct3=100, flag=0, rs1=1, rs2=0
        // raw = (4 << 13) | (0 << 12) | (1 << 7) | (0 << 2) | 2 = 0x8082
        let i = Insn16::jr(1);
        assert_eq!(i.0, 0x8082);
        assert_eq!(Insn16(0x8082).decode(), "c.jr x1");
    }

    #[test]
    fn c_jalr_x1() {
        // flag=1: raw = 0x8002 | 0x1000 | 0x80 = 0x9082
        let i = Insn16::jalr(1);
        assert_eq!(i.0, 0x9082);
        assert_eq!(Insn16(0x9082).decode(), "c.jalr x1");
    }

    #[test]
    fn c_ebreak() {
        // flag=1, rs1=0, rs2=0: raw = 0x8002 | 0x1000 = 0x9002
        let i = Insn16::ebreak();
        assert_eq!(i.0, 0x9002);
        assert_eq!(Insn16(0x9002).decode(), "c.ebreak");
    }

    #[test]
    fn c_mv_x1_x2() {
        // Q2 CR: flag=0, rd=1, rs2=2: raw = 0x8002 | 0x80 | 0x8 = 0x808A
        let i = Insn16::mv(1, 2);
        assert_eq!(i.0, 0x808A);
        assert_eq!(i.decode(), "c.mv x1, x2");
    }

    #[test]
    fn c_add_x1_x2() {
        // Q2 CR: flag=1, rd=1, rs2=2: raw = 0x9002 | 0x80 | 0x8 = 0x908A
        let i = Insn16::add(1, 2);
        assert_eq!(i.0, 0x908A);
        assert_eq!(i.decode(), "c.add x1, x2");
    }

    #[test]
    fn c_addi_r1_4() {
        // Q1 CI: funct3=000, rd=1, nzimm=4
        // raw = (0 << 15) | (0 << 12) | (1 << 7) | (4 << 2) | 1 = 0x0091
        let i = Insn16::addi(1, 4);
        assert_eq!(i.0, 0x0091);
        let d = i.decode();
        assert!(d.contains("c.addi"), "{}", d);
    }

    #[test]
    fn c_li_r5_42() {
        // Q1 CI: funct3=010, rd=5, imm=42
        // imm5 = bit 5 of 42 = 1, imm4_0 = 10
        // raw = (1 << 15) | (2 << 12) | (5 << 7) | (10 << 2) | 1 = 0xA2A9
        let i = Insn16::li(5, 42);
        assert_eq!(i.0, 0xA2A9);
        let d = i.decode();
        assert!(d.contains("c.li"), "{}", d);
    }

    #[test]
    fn c_slli_r1_5() {
        // Q2 CI: funct3=000, nzuimm[5]=0 (shamt=5 < 32), rd=x1, nzuimm[4:0]=5
        // raw = (0 << 12) | (5 << 2) | (1 << 7) | 2 = 0x0096
        let i = Insn16::slli(1, 5);
        assert_eq!(i.0, 0x0096);
        assert_eq!(i.decode(), "c.slli x1, 5");
    }

    #[test]
    fn c_lw_r10_r10_0() {
        // Q0 C.LW: rd=10(=2), rs1=10(=2), uimm=0
        // rd'=rd-8=2, rs1'=rs1-8=2, uimm=0
        // raw = (2 << 13) | (0 << 10) | (2 << 7) | (0 << 6) | (0 << 5) | (2 << 2)
        //     = 0x4000 | 0 | 0x100 | 0 | 0 | 0x8 = 0x4108
        let i = Insn16::lw(10, 10, 0);
        assert_eq!(i.0, 0x4108);
        assert_eq!(i.decode(), "c.lw x10, 0(x10)");
    }

    #[test]
    fn c_sw_r10_r10_0() {
        // Q0 C.SW: rs2=10(=2), rs1=10(=2), uimm=0
        // raw = (6 << 13) | (0 << 10) | (2 << 7) | (0 << 6) | (0 << 5) | (2 << 2)
        //     = 0xC000 | 0x100 | 0x8 = 0xC108
        let i = Insn16::sw(10, 10, 0);
        assert_eq!(i.0, 0xC108);
        assert_eq!(i.decode(), "c.sw x10, 0(x10)");
    }
}
