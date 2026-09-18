// ══ 64容器 TrueType 字体生成器（独立不依赖大库）══
// 编译: rustc --edition 2018 ttf_64_gen.rs -o ttf_64_gen
// 直接生成有效的 .ttf 文件，包含64个容器字形 + MCOD元数据

use std::io::Write;

type Int16 = i16;
type Uint16 = u16;
type Int32 = i32;
type Uint32 = u32;
type Tag = [u8; 4];

const DESIGN_SIZE: i16 = 100;
const UNITS_PER_EM: u16 = 1000;
// 64容器名称表
const NAMES: [&str; 64] = [
    "元","加","减","与","或","非","异或","传送","跳转","调用","返回","比较","加载","存储","移位","置位",
    "满","全","法","囡","丱","共","生","伴","侣","赔","偿","合","离","聚","散","归",
    "帮","滂","并","明","端","透","定","泥","知","彻","澄","娘","见","溪","群","疑",
    "东","冬","钟","江","支","脂","之","微","鱼","虞","模","齐","佳","皆","灰","咍",
];
const FAMILY_NAMES: [&str; 4] = ["运算族","操作族","声母族","韵母族"];

fn name_of(code: u8) -> &'static str { NAMES.get(code as usize).copied().unwrap_or("?") }
fn family_of(code: u8) -> &'static str { FAMILY_NAMES.get((code >> 4) as usize).copied().unwrap_or("?") }

fn topology_code(code: u8) -> u8 {
    match code & 0x0F {
        0x00|0x01|0x02 => 0x10, 0x03..=0x08 => 0x20, 0x09|0x0A|0x0B => 0x30,
        0x0C|0x0D => 0x50, 0x0E|0x0F => 0x60, _ => 0x10,
    }
}
fn topology_name(code: u8) -> &'static str {
    match topology_code(code) {
        0x10 => "盒体·立方", 0x20 => "球体·浑圆", 0x30 => "环面·循环",
        0x50 => "双锥·穿刺", 0x60 => "线型·贯通", _ => "未定义",
    }
}

// ══ BE Writer ══
struct BeWriter { buf: Vec<u8> }
impl BeWriter {
    fn new() -> Self { Self { buf: Vec::new() } }
    fn u8(&mut self, v: u8) { self.buf.push(v); }
    fn i16(&mut self, v: i16) { self.buf.extend_from_slice(&v.to_be_bytes()); }
    fn u16(&mut self, v: u16) { self.buf.extend_from_slice(&v.to_be_bytes()); }
    fn i32(&mut self, v: i32) { self.buf.extend_from_slice(&v.to_be_bytes()); }
    fn u32(&mut self, v: u32) { self.buf.extend_from_slice(&v.to_be_bytes()); }
    fn fixed(&mut self, v: i32) { self.u32(v as u32); }
    fn tag(&mut self, s: &str) { let b = s.as_bytes(); for i in 0..4 { self.buf.push(b.get(i).copied().unwrap_or(b' ')); } }
    fn bytes(&mut self, d: &[u8]) { self.buf.extend_from_slice(d); }
    fn pad4(&mut self) { while self.buf.len()%4!=0 { self.buf.push(0); } }
    fn len(&self) -> u64 { self.buf.len() as u64 }
}

fn str_to_u32(s: &str) -> u32 {
    let b = s.bytes().collect::<Vec<_>>();
    ((b.get(0).copied().unwrap_or(0) as u32) << 24)|
    ((b.get(1).copied().unwrap_or(0) as u32) << 16)|
    ((b.get(2).copied().unwrap_or(0) as u32) << 8)|
    (b.get(3).copied().unwrap_or(0) as u32)
}

fn calc_checksum(data: &[u8]) -> u32 {
    let mut sum = 0u32;
    let mut i = 0;
    while i + 3 < data.len() {
        sum = sum.wrapping_add(((data[i] as u32)<<24)|((data[i+1] as u32)<<16)|((data[i+2] as u32)<<8)|(data[i+3] as u32));
        i += 4;
    }
    sum
}

fn make_circle(cx: i16, cy: i16, r: i16, n: u16) -> Vec<(i16,i16)> {
    (0..n).map(|i| {
        let a = (i as f64) * 2.0 * std::f64::consts::PI / (n as f64);
        ((a.cos() * r as f64).round() as i16 + cx, (a.sin() * r as f64).round() as i16 + cy)
    }).collect()
}

fn build_simple_contour(topo: u8, _code: u8, size: i16) -> Vec<u8> {
    let mut buf = Vec::new();
    let pts = match topo {
        0x10 => { // box
            let s = size;
            vec![(-s,-s),(s,-s),(s,s),(-s,s),(-s,-s)]
        }
        0x20 => make_circle(0, 0, size, 16),
        0x30 => { // torus: outer + inner
            let outer = make_circle(0, 0, size, 16);
            let inner = make_circle(0, 0, size/2, 12);
            let mut all = outer; all.extend(inner); all
        }
        0x50 => { // double cone
            let s = size;
            vec![(0,-s),(-s,s),(s,s),(0,-s)]
        }
        0x60 => { // line
            let s = size;
            vec![(-s,0),(s,0)]
        }
        _ => make_circle(0, 0, size, 16),
    };
    let n_contours: i16 = if topo == 0x30 { 2 } else { 1 };
    let x_min = pts.iter().map(|p| p.0).min().unwrap_or(0);
    let y_min = pts.iter().map(|p| p.1).min().unwrap_or(0);
    let x_max = pts.iter().map(|p| p.0).max().unwrap_or(0);
    let y_max = pts.iter().map(|p| p.1).max().unwrap_or(0);

    buf.extend_from_slice(&n_contours.to_be_bytes());
    buf.extend_from_slice(&x_min.to_be_bytes());
    buf.extend_from_slice(&y_min.to_be_bytes());
    buf.extend_from_slice(&x_max.to_be_bytes());
    buf.extend_from_slice(&y_max.to_be_bytes());

    if topo == 0x30 {
        // torus: 2 contours
        let outer_end = 16u16 - 1;
        let inner_end = 16u16 + 12u16 - 1;
        buf.extend_from_slice(&outer_end.to_be_bytes());
        buf.extend_from_slice(&inner_end.to_be_bytes());
        buf.extend_from_slice(&(inner_end as i16 + 1).to_be_bytes()); // instructionLength
    } else {
        let last = (pts.len() as u16) - 1;
        buf.extend_from_slice(&last.to_be_bytes());
        buf.extend_from_slice(&0u16.to_be_bytes()); // instructionLength
    }

    // endPtsOfContours
    let flag: u8 = 0x03; // ON_CURVE
    if topo == 0x30 {
        for _ in 0..16u16 { buf.extend_from_slice(&[flag, 0x00]); } // flags + x
        for _ in 0..12u16 { buf.extend_from_slice(&[flag, 0x00]); }
    } else {
        for _ in 0..pts.len() { buf.push(flag); }
    }

    // coordinates (simplified: all zero in x for scalar, and in y if scalar format)
    if topo == 0x30 {
        let outer_i = make_circle(0, 0, size, 16);
        let inner_i = make_circle(0, 0, size/2, 12);
        // x coordinates
        for &(x,_) in &outer_i { let d = x; buf.extend_from_slice(&(d as i16).to_be_bytes()); }
        for &(x,_) in &inner_i { let d = x; buf.extend_from_slice(&(d as i16).to_be_bytes()); }
        // y coordinates
        for &(_,y) in &outer_i { let d = y; buf.extend_from_slice(&(d as i16).to_be_bytes()); }
        for &(_,y) in &inner_i { let d = y; buf.extend_from_slice(&(d as i16).to_be_bytes()); }
        // pad
        while buf.len() % 4 != 0 { buf.push(0); }
    } else {
        // short format flags
        let flags_pts = pts.len();
        let f2 = if topo == 0x10 { 0x11 } else { 0x01 };
        for _ in 0..flags_pts {
            buf.push(flag);
        }
        // x coordinates (short positive = on-curve dx)
        let mut prev_x = 0i16;
        for &(x,_) in &pts {
            let dx = x - prev_x;
            buf.extend_from_slice(&dx.to_be_bytes());
            prev_x = x;
        }
        // y coordinates
        let mut prev_y = 0i16;
        for &(_,y) in &pts {
            let dy = y - prev_y;
            buf.extend_from_slice(&dy.to_be_bytes());
            prev_y = y;
        }
        while buf.len() % 4 != 0 { buf.push(0); }
    }
    buf
}

fn mcod_entry(pack_id: u64, code: u8) -> Vec<u8> {
    let mut e = Vec::new();
    e.extend_from_slice(&pack_id.to_be_bytes()); // 8B
    func_code_bytes(code, &mut e); // 2B
    e.extend_from_slice(&[0u8; 8]); // param_slot
    e.extend_from_slice(&[0u8; 4]); // time_stamp
    e.push(0); // align_flag
    e.extend_from_slice(&[0xDE, 0xAD, 0xBE]); // witness
    while e.len() % 4 != 0 { e.push(0); }
    e
}

fn func_code_bytes(code: u8, out: &mut Vec<u8>) {
    out.extend_from_slice(&(code as u16).to_be_bytes());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out_path = args.get(1).map(|s| s.as_str()).unwrap_or("han_yuan64.ttf");
    let html_path = format!("{}.html", out_path.strip_suffix(".ttf").unwrap_or(out_path));

    let n = 64u16;
    let mut glyph_data: Vec<Vec<u8>> = Vec::new();
    let mut advance_widths: Vec<u16> = Vec::new();
    let mut code_points: Vec<u32> = Vec::new();

    for i in 0u8..64 {
        let topo = topology_code(i);
        let contour = build_simple_contour(topo, i, DESIGN_SIZE);
        glyph_data.push(contour);
        advance_widths.push(UNITS_PER_EM);
        code_points.push(0xE000 + i as u32);
    }

    let mut w = BeWriter::new();

    // ══ Table Directory ══
    let num_tables: u16 = 10;
    w.u16(0x0001); // sfVersion
    w.u16(num_tables);
    let search_range = (num_tables as f64).log2().floor() as u32 * 16;
    w.u16(search_range as u16);
    w.u16((num_tables as f64).log2().floor() as u16);
    w.u16(num_tables * 16 - search_range as u16);

    // directory entries (filled later)
    let dir_pos = w.len();
    for _ in 0..num_tables as usize { w.tag("    "); w.u32(0); w.u32(0); w.u32(0); }

    let tables_start = w.len();
    fn write_table(w: &mut BeWriter, tag: &str, data: &[u8]) {
        w.tag(tag);
        w.u32(calc_checksum(data));
        w.u32(w.len() as u32); // offset (placeholder, will be fixed)
        w.u32(data.len() as u32);
    }

    // ══ Build each table's data ══
    // head
    let mut head = BeWriter::new();
    head.fixed(0x00010000); // version
    head.fixed(0x00010000); // fontRevision
    head.u32(0); // checksumAdjustment
    head.u32(0x5F0F3CF5); // magicNumber
    head.u16(0x000B); // flags
    head.u16(UNITS_PER_EM);
    head.bytes(&[0u8;8]); // created
    head.bytes(&[0u8;8]); // modified
    head.i16(-DESIGN_SIZE as i16); head.i16(-DESIGN_SIZE as i16);
    head.i16(DESIGN_SIZE as i16); head.i16(DESIGN_SIZE as i16);
    head.u8(0); // macStyle
    head.u8(8); // lowestRecPPEM
    head.i16(2); // fontDirectionHint
    head.i16(1); // indexToLocFormat = long
    head.i16(0); // glyphDataFormat

    // hhea
    let mut hhea = BeWriter::new();
    hhea.fixed(0x00010000);
    hhea.i16(DESIGN_SIZE as i16); // ascent
    hhea.i16(-DESIGN_SIZE as i16); // descent
    hhea.i16(0); // lineGap
    hhea.u16(UNITS_PER_EM); // advanceWidthMax
    hhea.i16(0); // minLeftSideBearing
    hhea.i16(0); // minRightSideBearing
    hhea.i16(DESIGN_SIZE as i16); // xMaxExtent
    hhea.i16(1); hhea.i16(0); // caretSlopeRise/Run
    hhea.bytes(&[0u8;10]); // reserved
    hhea.i16(0); // metricDataFormat
    hhea.u16(n); // numberOfHMetrics

    // maxp
    let mut maxp = BeWriter::new();
    maxp.fixed(0x00010000);
    maxp.u16(n); // numGlyphs
    maxp.u16(128); maxp.u16(0); maxp.u16(1); maxp.u16(0); maxp.u16(0);
    maxp.u16(0); maxp.u16(0); maxp.u16(0); maxp.u16(0);
    maxp.u16(if n > 1 { 2 } else { 1 }); // maxComponentElements/ComponentDepth

    // cmap format 4
    let seg_count = 1u16;
    let mut cmap = BeWriter::new();
    cmap.u16(0); // version
    cmap.u16(1); // numTables
    cmap.u16(3); // platformID
    cmap.u16(1); // encodingID
    cmap.u32(12); // offset to subtable

    // format 4 subtable
    let search_range = 2u16;
    cmap.u16(4); // format
    let length_pos = cmap.len();
    cmap.u16(0); // length (placeholder)
    cmap.u16(0); // language
    cmap.u16(seg_count * 2); // segCountX2
    cmap.u16(search_range);
    cmap.u16(0); // entrySelector
    cmap.u16(0); // rangeShift
    cmap.u16(0x3F + 1); // endCode[0] = last cp
    cmap.u16(0xFFFF); // endCode[1] (sentinel)
    cmap.u16(0); // reservedPad
    cmap.u16(0); // startCode[0] = 0
    cmap.u16(0xFFFF); // startCode[1] (sentinel)
    cmap.i16(-0x3F); // idDelta[0] (cp 0 → gid 0, but startCp=0 so idDelta=1; cp 0x3F → gid 0x3F means idDelta=1)
    cmap.i16(1); // idDelta[1] + sentinel
    cmap.u16(0); // idRangeOffset[0]
    cmap.u16(0); // idRangeOffset[1]

    // Fix cmap length
    let cmap_sub_len = cmap.buf.len() as u16;
    cmap.buf[length_pos as usize] = (cmap_sub_len >> 8) as u8;
    cmap.buf[length_pos as usize + 1] = (cmap_sub_len & 0xFF) as u8;

    // loca (long)
    let mut loca = BeWriter::new();
    let mut offset: u32 = 0;
    for gd in &glyph_data {
        loca.u32(offset);
        offset += gd.len() as u32;
    }
    loca.u32(offset);

    // glyf
    let mut glyf = BeWriter::new();
    for gd in &glyph_data {
        glyf.bytes(gd);
        glyf.pad4();
    }

    // name
    let mut name = BeWriter::new();
    name.u16(0); // format
    name.u16(1); // count
    name.u16(18); // stringOffset
    // NameRecord
    name.u16(3); name.u16(1); name.u16(0x0409); name.u16(1); // fontFamily en-US
    {
        let s = b"HanYuan64";
        name.u16(s.len() as u16);
        name.bytes(s);
    }

    // post
    let mut post = BeWriter::new();
    post.fixed(0x00030000); // version 3.0 (no glyph names)
    post.i32(0); // italicAngle
    post.i16(-50); // underlinePosition
    post.i16(50); // underlineThickness
    post.u32(0); // isFixedPitch
    post.u32(0); post.u32(0); post.u32(0); post.u32(0); // minMemType42 etc.

    // MCOD (custom)
    let mcod_tag = *b"MCOD";
    let mut mcod = BeWriter::new();
    mcod.u32(0x00010000); // version
    mcod.u32(n as u32); // count
    for i in 0u8..64 {
        let entry = mcod_entry(i as u64, i);
        mcod.bytes(&entry);
        mcod.pad4();
    }

    // hmtx
    let mut hmtx = BeWriter::new();
    for &aw in &advance_widths {
        hmtx.u16(aw);
        hmtx.i16(0); // lsb
    }

    // ══ Assemble ══
    let table_data = [
        ("head", head.buf.as_slice()),
        ("hhea", hhea.buf.as_slice()),
        ("maxp", maxp.buf.as_slice()),
        ("cmap", cmap.buf.as_slice()),
        ("glyf", glyf.buf.as_slice()),
        ("loca", loca.buf.as_slice()),
        ("name", name.buf.as_slice()),
        ("post", post.buf.as_slice()),
        ("MCOD", mcod.buf.as_slice()),
        ("hmtx", hmtx.buf.as_slice()),
    ];

    // Go back and fix directory with actual offsets
    let mut actual_dir = vec![0u8; num_tables as usize * 16];
    let table_data_clone: Vec<Vec<u8>> = table_data.iter().map(|(_, d)| d.to_vec()).collect();

    // ══ Build all table data ══
    let table_list: [(&str, Vec<u8>); 10] = [
        ("head", head.buf.clone()),
        ("hhea", hhea.buf.clone()),
        ("maxp", maxp.buf.clone()),
        ("cmap", cmap.buf.clone()),
        ("glyf", glyf.buf.clone()),
        ("loca", loca.buf.clone()),
        ("name", name.buf.clone()),
        ("post", post.buf.clone()),
        ("MCOD", mcod.buf.clone()),
        ("hmtx", hmtx.buf.clone()),
    ];

    // Compute offset for each table (after 12-byte header + 10*16-byte directory)
    let dir_size = 12 + 10 * 16;
    let mut offsets: Vec<(String, u32, u32)> = Vec::new();
    let mut acc: u32 = dir_size as u32;
    for (tag, data) in &table_list {
        let padded = ((data.len() + 3) / 4 * 4) as u32;
        offsets.push((tag.to_string(), acc, data.len() as u32));
        acc += padded;
    }

    // ══ Assemble final file ══
    let mut out_buf = Vec::new();

    // Offset Table (12 bytes)
    out_buf.extend_from_slice(&0x00010000u32.to_be_bytes()); // sfVersion
    out_buf.extend_from_slice(&10u16.to_be_bytes());         // numTables
    out_buf.extend_from_slice(&128u16.to_be_bytes());        // searchRange
    out_buf.extend_from_slice(&3u16.to_be_bytes());          // entrySelector
    out_buf.extend_from_slice(&(10u16 * 16 - 128).to_be_bytes()); // rangeShift

    // Table Directory (10 × 16 bytes), sorted by tag
    let mut sorted_offsets = offsets.clone();
    sorted_offsets.sort_by(|a, b| a.0.cmp(&b.0));
    for (tag, offset, length) in &sorted_offsets {
        let data = table_list.iter().find(|(t,_)| t == tag).map(|(_,d)| d.clone()).unwrap_or_default();
        out_buf.extend_from_slice(tag.as_bytes());
        out_buf.extend_from_slice(&calc_checksum(&data).to_be_bytes());
        out_buf.extend_from_slice(&offset.to_be_bytes());
        out_buf.extend_from_slice(&length.to_be_bytes());
    }

    // Table Data (in original order)
    for (tag, data) in &table_list {
        let off = offsets.iter().find(|(t,_,_)| t == tag).map(|(_,o,_)| *o).unwrap_or(0);
        // Pad to reach correct offset
        while out_buf.len() < off as usize { out_buf.push(0); }
        out_buf.extend_from_slice(data);
        while out_buf.len() % 4 != 0 { out_buf.push(0); }
    }

    // ══ Fix head checksum ══
    // Find head table's offset from the directory we just wrote
    let head_entry_off = 12 + sorted_offsets.iter().position(|(t,_,_)| t == "head").unwrap_or(0) * 16;
    // Directory entry: tag(4) + checksum(4) + offset(4) + length(4)
    let head_table_offset = u32::from_be_bytes([out_buf[head_entry_off+8], out_buf[head_entry_off+9], out_buf[head_entry_off+10], out_buf[head_entry_off+11]]);
    let csa_pos = (head_table_offset + 8) as usize;
    let whole_checksum = calc_checksum(&out_buf);
    let bafn_sum = 0xB1B0AFu32;
    let adjustment = bafn_sum.wrapping_sub(whole_checksum);
    out_buf[csa_pos] = ((adjustment >> 24) & 0xFF) as u8;
    out_buf[csa_pos+1] = ((adjustment >> 16) & 0xFF) as u8;
    out_buf[csa_pos+2] = ((adjustment >> 8) & 0xFF) as u8;
    out_buf[csa_pos+3] = (adjustment & 0xFF) as u8;

    // Write file
    let mut file = std::fs::File::create(out_path).expect("Cannot create file");
    file.write_all(&out_buf).expect("Write failed");

    println!("[容器字体生成器] 64 容器 → {}", out_path);
    println!("[成功] 字体已写入: {} ({} 字节)", out_path, out_buf.len());
    println!("  MCOD 条目: {}", n);
    for i in 0u8..64 {
        println!("  0x{:02X} {:4} → {} (拓扑码={})", i, name_of(i), topology_name(i), topology_code(i));
    }

    // Generate HTML
    let mut html = String::from(r#"<!DOCTYPE html><html><head><meta charset="utf-8"><title>HanYuan64 容器字体</title>
<style>
body{font-family:sans-serif;background:#1a1a2e;color:#eee;padding:20px}
@font-face{font-family:"HanYuan64";src:url("data:font/ttf;base64,"#);
    // embed base64 font
    let b64 = base64_encode(&out_buf);
    html.push_str(&b64);
    html.push_str(r#"")}
.disp{font-family:"HanYuan64",sans-serif;font-size:48px;line-height:1.2}
.grid{display:grid;grid-template-columns:repeat(8,1fr);gap:8px;margin-top:16px}
.cell{border:1px solid #444;border-radius:6px;padding:8px;text-align:center;background:#16213e}
.cp{font-size:36px;display:block}
.id{font-size:10px;color:#aaa}
.nm{font-size:14px;color:#fff}
h1{font-size:24px;margin-bottom:8px}
</style></head><body><h1>HanYuan64 — 64容器字体</h1>
<div class="disp"#);
    for i in 0u8..64 {
        let cp = 0xE000u32 + i as u32;
        if let Some(c) = char::from_u32(cp) {
            html.push(c);
        }
    }
    html.push_str("</div>");
    html.push_str(&format!("<p>{} 字形 · 4 族 × 16 · 5 种拓扑 · {} 字节</p>", n, out_buf.len()));
    html.push_str("<div class=\"grid\">");
    let colors = ["#4A90D9", "#7B68EE", "#50B86C", "#D98C4A"];
    for i in 0u8..64 {
        let cp = 0xE000u32 + i as u32;
        let ch = char::from_u32(cp).unwrap_or('?');
        let fam = (i >> 4) as usize;
        html.push_str(&format!(
            r#"<div class="cell" style="border-color:{}"><span class="cp">{}</span><span class="id">0x{:02X}</span><span class="nm">{}</span></div>"#,
            colors[fam], ch, i, name_of(i)
        ));
    }
    html.push_str("</div></body></html>");

    let mut html_file = std::fs::File::create(&html_path).expect("Cannot create html");
    html_file.write_all(html.as_bytes()).expect("Write html failed");
    println!("  HTML 预览: {}", html_path);
}

// Simple base64 encoder (no external crates)
const B64_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
fn base64_encode(data: &[u8]) -> String {
    let mut out = String::new();
    let mut i = 0;
    while i < data.len() {
        let b0 = data[i];
        let b1 = if i+1 < data.len() { data[i+1] } else { 0 };
        let b2 = if i+2 < data.len() { data[i+2] } else { 0 };
        out.push(B64_TABLE[(b0 >> 2) as usize] as char);
        out.push(B64_TABLE[(((b0 & 3) << 4) | (b1 >> 4)) as usize] as char);
        if i+1 < data.len() {
            out.push(B64_TABLE[(((b1 & 0xF) << 2) | (b2 >> 6)) as usize] as char);
        } else { out.push('='); }
        if i+2 < data.len() {
            out.push(B64_TABLE[(b2 & 0x3F) as usize] as char);
        } else { out.push('='); }
        i += 3;
    }
    out
}
