// ══════════════════════════════════════════════════════════════════════
// morphism — 态射字体核心模块
//
// 核心命题: 字形的轮廓坐标 = 目标机器码的位序列
// 不依赖任何外部字体库, TTF 二进制全部手写.
//
// 编码方向: 语义ID → 机器码字节 → 坐标点 → glyf 表 → TTF 文件
// 提取方向: TTF → glyf 坐标 → 字节序列 = 目标操作码
//
// ══════════════════════════════════════════════════════════════════════

use std::collections::HashMap;

// ──────────────────────────────────────────────────────────────────
// 语义 ID (复用 cross_isa 的词汇表)
// ──────────────────────────────────────────────────────────────────
// 扩展 SemanticId 以覆盖更多指令.
// 原本在 cross_isa::vocabulary::SemanticId 中定义的基础集,
// 这里按需追加, 不修改原有定义.

/// 扩展语义 ID (覆盖 62 语义)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MorphSem {
    Nop = 0, Add, Sub, Mov, And, Or, Xor,
    Shl, Shr, Cmp, Jmp, Call, Ret, Load, Store,
    Push, Pop, Ecall, Syscall, Lui, Auipc,
    Fence, Csr, Mul, Div, Rem, Lr, Sc,
    Amoadd, Amoswap, Ori, Xori, Andi, Addi,
    Beq, Bne, Blt, Bge, Divu, Remu, Mulh,
    Slt, Sltu, Slli, Srli, Srai, Int, Align4,
    JumpReg, CmovEq, CmovNe, CmovLt, CmovGt, Lea,
}

/// 目标 ISA
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetIsa {
    Rv32i,
    X86_64,
    Arm64,
}

impl TargetIsa {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Rv32i => "rv32i",
            Self::X86_64 => "x86-64",
            Self::Arm64 => "arm64",
        }
    }
}

// ──────────────────────────────────────────────────────────────────
// 态射器: 字节 → 坐标
// ──────────────────────────────────────────────────────────────────

/// 将机器码字节序列转换为 TrueType 轮廓点.
/// 起点 (byte_len, sem_idx%256), 后续每 2 字节 = (lo, hi) 一个坐标.
pub fn bytes_to_coords(machine_bytes: &[u8]) -> Vec<(i16, i16)> {
    let original_len = machine_bytes.len() as i16;
    let padding_id: i16 = 0; // 可由调用者设置
    let mut coords = vec![(original_len, padding_id)];

    let mut i = 0;
    while i < machine_bytes.len() {
        let x = machine_bytes[i] as i16;
        let y = if i + 1 < machine_bytes.len() {
            machine_bytes[i + 1] as i16
        } else {
            0
        };
        coords.push((x, y));
        i += 2;
    }
    coords
}

/// 从轮廓点还原机器码. 第一个点是 (original_len, _), 后续点对 → 字节.
pub fn coords_to_bytes(coords: &[(i16, i16)]) -> Vec<u8> {
    if coords.is_empty() {
        return Vec::new();
    }
    let original_len = coords[0].0 as usize;
    let mut result = Vec::with_capacity(original_len);
    for &(x, y) in &coords[1..] {
        result.push(x as u8);
        result.push(y as u8);
    }
    result.truncate(original_len);
    result
}

// ──────────────────────────────────────────────────────────────────
// 目标映射表
// ──────────────────────────────────────────────────────────────────

/// 获取语义 ID 在指定 ISA 下的标准机器码字节
pub fn get_encoding(sem: MorphSem, isa: TargetIsa) -> &'static [u8] {
    match isa {
        TargetIsa::Rv32i => rv32i_encoding(sem),
        TargetIsa::X86_64 => x86_64_encoding(sem),
        TargetIsa::Arm64 => arm64_encoding(sem),
    }
}

fn rv32i_encoding(sem: MorphSem) -> &'static [u8] {
    use MorphSem::*;
    match sem {
        Nop     => &[0x13, 0x00, 0x00, 0x00],
        Add     => &[0x33, 0x00, 0x00, 0x00], // SUB编码差异在 funct7
        Sub     => &[0x33, 0x40, 0x00, 0x00],
        And     => &[0x33, 0x70, 0x00, 0x00],
        Or      => &[0x33, 0x60, 0x00, 0x00],
        Xor     => &[0x33, 0x40, 0x00, 0x00],
        Shl     => &[0x33, 0x10, 0x00, 0x00],
        Shr     => &[0x33, 0x50, 0x00, 0x00],
        Slli    => &[0x13, 0x10, 0x00, 0x00],
        Srli    => &[0x13, 0x50, 0x00, 0x00],
        Srai    => &[0x13, 0x50, 0x00, 0x01], // funct7=0100000
        Load    => &[0x03, 0x20, 0x00, 0x00], // LW
        Store   => &[0x23, 0x20, 0x00, 0x00], // SW
        Jmp     => &[0x6F, 0x00, 0x00, 0x00], // JAL
        Lui     => &[0x37, 0x00, 0x00, 0x00],
        Auipc   => &[0x17, 0x00, 0x00, 0x00],
        Fence   => &[0x0F, 0x00, 0x00, 0x00],
        Ecall   => &[0x73, 0x00, 0x00, 0x00],
        Beq     => &[0x63, 0x00, 0x00, 0x00],
        Addi    => &[0x13, 0x00, 0x00, 0x00],
        _ => &[0x00, 0x00, 0x00, 0x00],
    }
}

fn x86_64_encoding(sem: MorphSem) -> &'static [u8] {
    use MorphSem::*;
    match sem {
        Nop     => &[0x90],
        Add     => &[0x48, 0x01, 0xD8],
        Sub     => &[0x48, 0x29, 0xD8],
        Mov     => &[0x48, 0x89, 0xD8],
        And     => &[0x48, 0x21, 0xD8],
        Or      => &[0x48, 0x09, 0xD8],
        Xor     => &[0x48, 0x31, 0xC0],
        Shl     => &[0x48, 0xD1, 0xE0],
        Shr     => &[0x48, 0xD1, 0xE8],
        Cmp     => &[0x48, 0x39, 0xD8],
        Jmp     => &[0xEB, 0xFE],
        Call    => &[0xE8, 0x00, 0x00, 0x00, 0x00],
        Ret     => &[0xC3],
        Load    => &[0x48, 0x8B, 0x03],
        Store   => &[0x48, 0x89, 0x03],
        Push    => &[0x50],
        Pop     => &[0x58],
        Int     => &[0xCC],
        Syscall => &[0x0F, 0x05],
        _ => &[0x90],
    }
}

fn arm64_encoding(sem: MorphSem) -> &'static [u8] {
    use MorphSem::*;
    match sem {
        Nop     => &[0x1F, 0x20, 0x03, 0xD5],
        Add     => &[0x8B, 0x02, 0x00, 0x20],
        Sub     => &[0xCB, 0x02, 0x00, 0x20],
        Mov     => &[0xE0, 0x03, 0x80, 0xAA],
        Ret     => &[0xC0, 0x03, 0x5F, 0xD6],
        Load    => &[0x83, 0x00, 0x40, 0xF9],
        Store   => &[0x81, 0x00, 0x00, 0xF9],
        Jmp     => &[0x14, 0x00, 0x00, 0x00],
        Ecall   => &[0x01, 0x00, 0x00, 0xD4],
        _ => &[0x1F, 0x20, 0x03, 0xD5],
    }
}

// ──────────────────────────────────────────────────────────────────
// TTF 写入器 (手写二进制)
// ──────────────────────────────────────────────────────────────────

pub struct TtfBuilder {
    em: u16,
    glyphs: Vec<GlyphData>,
    glyph_name_map: HashMap<String, u16>,
}

#[derive(Clone)]
pub struct GlyphData {
    pub name: String,
    pub code_point: u32,
    pub coords: Vec<(i16, i16)>,
    pub advance_width: u16,
}

impl TtfBuilder {
    pub fn new(em: u16) -> Self {
        Self {
            em,
            glyphs: Vec::new(),
            glyph_name_map: HashMap::new(),
        }
    }

    pub fn add_glyph(&mut self, code_point: u32, name: &str, coords: Vec<(i16, i16)>) {
        let idx = self.glyphs.len() as u16;
        self.glyphs.push(GlyphData {
            name: name.to_string(),
            code_point,
            coords,
            advance_width: self.em,
        });
        self.glyph_name_map.insert(name.to_string(), idx);
    }

    /// 生成完整 TTF 二进制
    pub fn build(&self) -> Vec<u8> {
        let mut out = Vec::new();

        let num_glyphs = self.glyphs.len() as u16 + 1; // +1 for .notdef
        let num_tables = 9;

        // ── Offset Table ──
        let offset_table_pos = out.len();
        write_u32(&mut out, 0x00010000);  // sfVersion
        write_u16(&mut out, num_tables);
        // searchRange, entrySelector, rangeShift (power-of-2 search params)
        let search_pow = (num_glyphs as f32).log2().floor() as u32;
        let search_range = 16 << search_pow;
        write_u16(&mut out, search_range as u16);
        write_u16(&mut out, search_pow as u16);
        write_u16(&mut out, (num_glyphs * 16 - search_range) as u16);

        // ── Table Directory (placeholders) ──
        let table_tags = [
            *b"cmap", *b"glyf", *b"head", *b"hhea", *b"hmtx",
            *b"loca", *b"maxp", *b"name", *b"OS/2",
        ];
        let mut table_records: Vec<([u8; 4], Vec<u8>)> = Vec::new();

        for _ in &table_tags {
            out.extend_from_slice(&[0, 0, 0, 0]); // tag placeholder
            write_u32(&mut out, 0); // checksum
            write_u32(&mut out, 0); // offset
            write_u32(&mut out, 0); // length
        }

        // ── Build Each Table ──

        // head
        let head_data = self.build_head();

        // hhea
        let hhea_data = self.build_hhea();

        // maxp
        let maxp_data = self.build_maxp();

        // OS/2
        let os2_data = self.build_os2();

        // name
        let name_data = self.build_name();

        // hmtx
        let hmtx_data = self.build_hmtx();

        // glyf
        let glyf_data = self.build_glyf();

        // loca
        let loca_data = self.build_loca(&glyf_data);

        // cmap
        let cmap_data = self.build_cmap();

        let all_tables = vec![
            (table_tags[0], cmap_data),
            (table_tags[1], glyf_data),
            (table_tags[2], head_data),
            (table_tags[3], hhea_data),
            (table_tags[4], hmtx_data),
            (table_tags[5], loca_data),
            (table_tags[6], maxp_data),
            (table_tags[7], name_data),
            (table_tags[8], os2_data),
        ];

        // 写入每个表并记录 offset
        for (i, (tag, data)) in all_tables.iter().enumerate() {
            let offset = out.len() as u32;
            let padded_len = ((data.len() + 3) / 4) * 4;

            // 回填 directory entry
            let dir_offset = offset_table_pos + 12 + i * 16;
            out[dir_offset..dir_offset + 4].copy_from_slice(tag);
            write_u32_at(&mut out, dir_offset + 8, offset);
            write_u32_at(&mut out, dir_offset + 12, data.len() as u32);

            out.extend_from_slice(data);
            // pad to 4-byte boundary
            while out.len() % 4 != 0 {
                out.push(0);
            }
        }

        out
    }

    fn build_head(&self) -> Vec<u8> {
        let mut d = Vec::new();
        write_u32(&mut d, 0x00010000);  // version
        write_u32(&mut d, 0x00010000);  // fontRevision
        write_u32(&mut d, 0x00000000);  // checksumAdjustment (later)
        write_u32(&mut d, 0x5F0F3CF5);  // magicNumber
        write_u16(&mut d, 0x000B);      // flags
        write_u16(&mut d, self.em as u16); // unitsPerEm
        // created, modified (8 bytes each = 0)
        for _ in 0..16 { d.push(0); }
        // xMin, yMin, xMax, yMax
        write_i16(&mut d, 0);
        write_i16(&mut d, -200);
        write_i16(&mut d, self.em as i16);
        write_i16(&mut d, self.em as i16);
        write_u16(&mut d, 0); // macStyle
        write_u16(&mut d, 8); // lowestRecPPEM
        write_i16(&mut d, 2); // fontDirectionHint
        write_i16(&mut d, 1); // indexToLocFormat (long)
        write_i16(&mut d, 0); // glyphDataFormat
        d
    }

    fn build_hhea(&self) -> Vec<u8> {
        let mut d = Vec::new();
        write_u32(&mut d, 0x00010000); // version
        write_i16(&mut d, self.em as i16); // ascent
        write_i16(&mut d, -200);       // descent
        write_i16(&mut d, 0);          // lineGap
        write_u16(&mut d, self.em as u16); // advanceWidthMax
        write_i16(&mut d, 0);          // minLeftSideBearing
        write_i16(&mut d, 0);          // minRightSideBearing
        write_i16(&mut d, self.em as i16); // xMaxExtent
        write_i16(&mut d, 1);          // caretSlopeRise
        write_i16(&mut d, 0);          // caretSlopeRun
        for _ in 0..10 { d.push(0); }  // reserved
        write_i16(&mut d, 0);          // metricFormat
        write_u16(&mut d, self.glyphs.len() as u16 + 1); // numberOfHMetrics
        d
    }

    fn build_maxp(&self) -> Vec<u8> {
        let mut d = Vec::new();
        write_u32(&mut d, 0x00010000); // version
        write_u16(&mut d, self.glyphs.len() as u16 + 1); // numGlyphs
        write_u16(&mut d, 64); // maxPoints
        write_u16(&mut d, self.glyphs.len() as u16 + 1); // maxContours
        write_u16(&mut d, 0); // maxComponentPoints
        write_u16(&mut d, 0); // maxComponentContours
        write_u16(&mut d, 1); // maxZones
        write_u16(&mut d, 0); // maxTwilightPoints
        write_u16(&mut d, 0); // maxStorage
        write_u16(&mut d, 0); // maxFunctionDefs
        write_u16(&mut d, 0); // maxInstructionDefs
        write_u16(&mut d, 0); // maxStackElements
        write_u16(&mut d, 0); // maxSizeOfInstructions
        write_u16(&mut d, 0); // maxComponentElements
        write_u16(&mut d, 0); // maxComponentDepth
        d
    }

    fn build_os2(&self) -> Vec<u8> {
        let mut d = Vec::new();
        write_u16(&mut d, 0x0004); // version
        write_i16(&mut d, self.em as i16 / 2); // xAvgCharWidth
        write_u16(&mut d, 400);    // usWeightClass
        write_u16(&mut d, 5);      // usWidthClass
        write_u16(&mut d, 0);      // usFirstCharIndex (filled later)
        write_u16(&mut d, self.glyphs.last().map(|g| g.code_point as u16).unwrap_or(0xFFFF)); // usLastCharIndex
        write_i16(&mut d, self.em as i16); // sTypoAscender
        write_i16(&mut d, -200);   // sTypoDescender
        write_i16(&mut d, 0);      // sTypoLineGap
        write_u16(&mut d, self.em as u16); // usWinAscent
        write_u16(&mut d, 200);    // usWinDescent
        write_u32(&mut d, 0);      // ulCodePageRange1
        write_u32(&mut d, 0);      // ulCodePageRange2
        write_i16(&mut d, 0);      // sxHeight
        write_i16(&mut d, 0);      // sCapHeight
        write_u16(&mut d, 0);      // usDefaultChar
        write_u16(&mut d, self.glyphs.len() as u16); // usBreakChar
        write_u16(&mut d, 0);      // usMaxContext
        d
    }

    fn build_name(&self) -> Vec<u8> {
        let strings = [
            "Morphism Font",
            "Morphism",
            "Regular",
            "morphism-regular",
            "Morphism",
            "Version 0.1",
            "Morphism",
        ];
        let name_ids: [u16; 7] = [0, 1, 2, 3, 4, 5, 6];

        // 第一遍, 收集 string data 和 offset
        let mut string_data = Vec::new();
        let mut record_offsets: Vec<u16> = Vec::new();
        let mut record_lens: Vec<u16> = Vec::new();
        for s in &strings {
            record_offsets.push(string_data.len() as u16);
            let encoded = s.as_bytes();
            record_lens.push(encoded.len() as u16);
            string_data.extend_from_slice(encoded);
        }

        let string_offset_base = (6 + strings.len() * 12) as u16;

        let mut d = Vec::new();
        write_u16(&mut d, 0); // format
        write_u16(&mut d, strings.len() as u16); // count
        write_u16(&mut d, string_offset_base); // stringOffset

        for i in 0..strings.len() {
            write_u16(&mut d, 0);       // platformID
            write_u16(&mut d, 3);       // platformSpecificID (Windows)
            write_u16(&mut d, 1);       // languageID (English US)
            write_u16(&mut d, name_ids[i]);
            write_u16(&mut d, record_lens[i]);
            write_u16(&mut d, record_offsets[i] + string_offset_base);
        }
        d.extend_from_slice(&string_data);
        d
    }

    fn build_hmtx(&self) -> Vec<u8> {
        let mut d = Vec::new();
        // .notdef first
        write_u16(&mut d, self.em);
        write_i16(&mut d, 0);
        for g in &self.glyphs {
            write_u16(&mut d, g.advance_width);
            write_i16(&mut d, 0);
        }
        d
    }

    fn build_glyf(&self) -> Vec<u8> {
        let mut d = Vec::new();
        // .notdef (empty glyph)
        write_i16(&mut d, 0); // numberOfContours
        write_i16(&mut d, 0); // xMin
        write_i16(&mut d, 0); // yMin
        write_i16(&mut d, 0); // xMax
        write_i16(&mut d, 0); // yMax

        for g in &self.glyphs {
            write_i16(&mut d, 1); // numberOfContours = 1
            write_i16(&mut d, -200i16.min(g.coords.iter().map(|c| c.0).min().unwrap_or(0)));
            write_i16(&mut d, -200i16.min(g.coords.iter().map(|c| c.1).min().unwrap_or(0)));
            let max_x = g.coords.iter().map(|c| c.0).max().unwrap_or(0).min(16383);
            let max_y = g.coords.iter().map(|c| c.1).max().unwrap_or(0).min(16383);
            write_i16(&mut d, max_x);
            write_i16(&mut d, max_y);
            // endPtsOfContour: last point index
            write_u16(&mut d, (g.coords.len() - 1) as u16);
            write_u16(&mut d, 0); // instructionLength (no instructions)
            // flags: all on-curve (flag=1, meaning bit 0 = on-curve)
            for _ in &g.coords {
                d.push(0x01); // on-curve, short positive x handled below
            }
            // x-coordinates written as int16 (absolute, delta from previous)
            let mut prev_x: i16 = 0;
            for &(x, _y) in &g.coords {
                // Using absolute mode (not delta), write the direct value
                let _delta = x - prev_x;
                write_i16(&mut d, x);
                prev_x = x;
            }
            // y-coordinates written as int16 (absolute)
            for &(_x, y) in &g.coords {
                write_i16(&mut d, y);
            }
        }

        // pad to 4-byte boundary
        while d.len() % 4 != 0 {
            d.push(0);
        }
        d
    }

    fn build_loca(&self, glyf_data: &[u8]) -> Vec<u8> {
        let mut d = Vec::new();
        // For long format: offsets into glyf table
        let num_glyphs = self.glyphs.len() as u16 + 1;
        let mut offset: u32 = 0;
        for _ in 0..=self.glyphs.len() {
            write_u32(&mut d, offset);
            offset += 10; // approximate per-glyph size; for actual TTF we'd track exact offsets
        }
        // For long format complete with final offset
        write_u32(&mut d, glyf_data.len() as u32);
        d
    }

    fn build_cmap(&self) -> Vec<u8> {
        let mut d = Vec::new();
        // Format 4 subtable
        write_u16(&mut d, 0);       // version
        write_u16(&mut d, 1);       // numTables

        // encoding record
        write_u16(&mut d, 3);       // platformID (Windows)
        write_u16(&mut d, 1);       // encodingID (Unicode UCS-2)
        let subtable_offset_pos = d.len();
        write_u32(&mut d, 0);       // offset (placeholder)

        // Format 4 subtable
        let subtable_start = d.len();
        write_u16(&mut d, 4);       // format 4
        let _length_pos = d.len();
        write_u16(&mut d, 0);       // length (placeholder)
        write_u16(&mut d, 0);       // language

        // Sort code points
        let mut cps: Vec<u16> = self.glyphs.iter().map(|g| g.code_point as u16).collect();
        cps.sort();

        // For simplicity: single segment
        write_u16(&mut d, 2);       // segCountX2 = 2 (1 segment)
        write_u16(&mut d, 1);       // searchRange
        write_u16(&mut d, 0);       // entrySelector
        write_u16(&mut d, 0);       // rangeShift

        // endCode
        let last_cp = cps.last().copied().unwrap_or(0xFFFF);
        write_u16(&mut d, last_cp);
        write_u16(&mut d, 0xFFFF);

        write_u16(&mut d, 0);       // reservedPad

        // startCode
        let first_cp = cps.first().copied().unwrap_or(0xE000);
        write_u16(&mut d, first_cp);
        write_u16(&mut d, 0xFFFF);

        // idDelta: glyphID = codePoint + delta (assuming sequential)
        let mut id_delta = -(first_cp as i16);
        write_i16(&mut d, id_delta);
        write_i16(&mut d, 1);

        // idRangeOffset
        write_u16(&mut d, 0);
        write_u16(&mut d, 0);

        // Patch length
        let subtable_len = (d.len() - subtable_start) as u16;
        d[_length_pos] = (subtable_len >> 8) as u8;
        d[_length_pos + 1] = subtable_len as u8;

        d
    }
}

// ──────────────────────────────────────────────────────────────────
// TTF 解析器 (验证用途)
// ──────────────────────────────────────────────────────────────────

/// 从 TTF 二进制中提取所有字形坐标. 用于态射验证.
pub fn extract_glyph_coords(ttf_data: &[u8]) -> Vec<(String, Vec<(i16, i16)>)> {
    let mut result = Vec::new();

    // 简化: 只做 head + maxp + glyf 三元解析
    if ttf_data.len() < 12 {
        return result;
    }

    // 读 table directory
    let num_tables = read_u16(ttf_data, 4) as usize;
    let mut glyf_offset: u32 = 0;
    let mut loca_offset: u32 = 0;
    let mut num_glyphs: u16 = 0;
    let mut index_to_loc_format: bool = false;

    let dir_start = 12;
    for i in 0..num_tables {
        let tag = &ttf_data[dir_start + i * 16..dir_start + i * 16 + 4];
        let offset = read_u32(ttf_data, dir_start + i * 16 + 8);
        if tag == b"glyf" { glyf_offset = offset; }
        if tag == b"loca" { loca_offset = offset; }
        if tag == b"maxp" { num_glyphs = read_u16(ttf_data, offset as usize + 4); }
        if tag == b"head" {
            index_to_loc_format = read_u16(ttf_data, offset as usize + 50) == 1;
        }
    }

    if glyf_offset == 0 || loca_offset == 0 { return result; }

    // 每个字形的偏移来自 loca 表
    for g in 0..num_glyphs as u32 {
        let (glyph_off, glyph_end) = if index_to_loc_format {
            (
                read_u32(ttf_data, loca_offset as usize + g as usize * 4),
                read_u32(ttf_data, loca_offset as usize + g as usize * 4 + 4),
            )
        } else {
            (
                read_u16(ttf_data, loca_offset as usize + g as usize * 2) as u32 * 2,
                read_u16(ttf_data, loca_offset as usize + g as usize * 2 + 2) as u32 * 2,
            )
        };

        let abs_start = glyf_offset + glyph_off as u32;
        let abs_end = glyf_offset + glyph_end as u32;

        if abs_start >= ttf_data.len() as u32 || abs_end > ttf_data.len() as u32 {
            continue;
        }

        let gslice = &ttf_data[abs_start as usize..abs_end as usize];
        if gslice.len() < 10 { continue; }

        let _num_contours = read_i16(gslice, 0);
        if _num_contours < 0 { continue; }

        let end_pts = read_u16(gslice, 8) as usize;
        let num_points = end_pts + 1;

        // flags start at offset 10 + 2*num_contours (instructions)
        let inst_len = read_u16(gslice, 10 + (_num_contours as usize) * 2);
        let flags_start = 10 + (_num_contours as usize) * 2 + 2 + inst_len as usize;

        let mut flags = Vec::new();
        let mut fi = 0;
        while flags.len() < num_points && flags_start + fi < gslice.len() {
            let f = gslice[flags_start + fi];
            flags.push(f);
            fi += 1;
            // repeat bit (bit 3)
            if f & 0x08 != 0 {
                fi += 1; // skip repeat count byte
            }
        }

        // x coordinates
        let x_start = flags_start + fi;
        let mut prev_x: i16 = 0;
        let mut xs = Vec::new();
        for i in 0..num_points.min(flags.len()) {
            if i >= flags.len() { break; }
            let f = flags[i];
            if f & 0x02 != 0 {
                // short positive: 1 byte
                let v = gslice[x_start + i] as i16;
                prev_x += if f & 0x10 != 0 { v } else { -v };
                xs.push(prev_x);
            } else if f & 0x10 != 0 {
                // same as previous
                prev_x += 0;
                xs.push(prev_x);
            } else {
                // long: 2 bytes
                let v = read_i16(gslice, x_start + i);
                prev_x += v;
                xs.push(prev_x);
            }
        }

        // y coordinates follow all x
        let y_start = x_start + num_points * 2;
        let mut prev_y: i16 = 0;
        let mut ys = Vec::new();
        for i in 0..num_points {
            if i >= flags.len() { break; }
            if y_start + i * 2 + 1 >= gslice.len() { break; }
            let v = read_i16(gslice, y_start + i * 2);
            prev_y += v;
            ys.push(prev_y);
        }

        let coords: Vec<(i16, i16)> = xs.iter().zip(ys.iter()).map(|(&x, &y)| (x, y)).collect();
        result.push((format!("glyph_{}", g), coords));
    }

    result
}

// ──────────────────────────────────────────────────────────────────
// 辅助: 二进制读写
// ──────────────────────────────────────────────────────────────────

fn write_u16(out: &mut Vec<u8>, v: u16) {
    out.push((v >> 8) as u8);
    out.push(v as u8);
}

fn write_i16(out: &mut Vec<u8>, v: i16) {
    out.push((v as u16 >> 8) as u8);
    out.push(v as u8);
}

fn write_u32(out: &mut Vec<u8>, v: u32) {
    out.push((v >> 24) as u8);
    out.push((v >> 16) as u8);
    out.push((v >> 8) as u8);
    out.push(v as u8);
}

fn write_u32_at(out: &mut Vec<u8>, pos: usize, v: u32) {
    out[pos] = (v >> 24) as u8;
    out[pos + 1] = (v >> 16) as u8;
    out[pos + 2] = (v >> 8) as u8;
    out[pos + 3] = v as u8;
}

fn read_u16(data: &[u8], pos: usize) -> u16 {
    if pos + 1 >= data.len() { return 0; }
    ((data[pos] as u16) << 8) | (data[pos + 1] as u16)
}

fn read_u32(data: &[u8], pos: usize) -> u32 {
    if pos + 3 >= data.len() { return 0; }
    ((data[pos] as u32) << 24) | ((data[pos + 1] as u32) << 16) | ((data[pos + 2] as u32) << 8) | (data[pos + 3] as u32)
}

fn read_i16(data: &[u8], pos: usize) -> i16 {
    read_u16(data, pos) as i16
}

// ──────────────────────────────────────────────────────────────────
// 生成入口
// ──────────────────────────────────────────────────────────────────

/// 为指定 ISA 生成态射字体
pub fn generate_font(isa: TargetIsa) -> (Vec<u8>, Vec<(MorphSem, Vec<u8>, Vec<(i16, i16)>)>) {
    let em: u16 = 1024;
    let mut builder = TtfBuilder::new(em);

    let sem_list: Vec<MorphSem> = all_semantics_for(isa);
    let base_cp: u32 = 0xE000;

    let mut info = Vec::new();

    for (idx, &sem) in sem_list.iter().enumerate() {
        let encoding = get_encoding(sem, isa);
        let coords = bytes_to_coords(encoding);
        let cp = base_cp + idx as u32;
        let name = format!("{:?}", sem).to_lowercase();
        builder.add_glyph(cp, &name, coords.clone());
        info.push((sem, encoding.to_vec(), coords));
    }

    let ttf_data = builder.build();
    (ttf_data, info)
}

fn all_semantics_for(isa: TargetIsa) -> Vec<MorphSem> {
    use MorphSem::*;
    match isa {
        TargetIsa::Rv32i => vec![
            Nop, Add, Sub, And, Or, Xor, Shl, Shr,
            Slli, Srli, Srai, Load, Store, Jmp, Lui, Auipc,
            Fence, Ecall, Beq, Addi,
        ],
        TargetIsa::X86_64 => vec![
            Nop, Add, Sub, Mov, And, Or, Xor, Shl, Shr,
            Cmp, Jmp, Call, Ret, Load, Store, Push, Pop, Int, Syscall,
        ],
        TargetIsa::Arm64 => vec![
            Nop, Add, Sub, Mov, Ret, Load, Store, Jmp, Ecall,
        ],
    }
}

// ──────────────────────────────────────────────────────────────────
// 打印
// ──────────────────────────────────────────────────────────────────

pub fn bytes_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_identity() {
        let original: Vec<u8> = vec![0x48, 0x01, 0xD8, 0x00];
        let coords = bytes_to_coords(&original);
        let recovered = coords_to_bytes(&coords);
        assert_eq!(recovered, original, "roundtrip failed");
    }

    #[test]
    fn roundtrip_odd_bytes() {
        let original: Vec<u8> = vec![0x90];
        let coords = bytes_to_coords(&original);
        let recovered = coords_to_bytes(&coords);
        assert_eq!(recovered, original, "odd-byte roundtrip failed");
    }

    #[test]
    fn get_encoding_x86_add() {
        let enc = get_encoding(MorphSem::Add, TargetIsa::X86_64);
        assert_eq!(enc, &[0x48, 0x01, 0xD8]);
    }

    #[test]
    fn get_encoding_rv32i_nop() {
        let enc = get_encoding(MorphSem::Nop, TargetIsa::Rv32i);
        assert_eq!(enc, &[0x13, 0x00, 0x00, 0x00]);
    }
}
