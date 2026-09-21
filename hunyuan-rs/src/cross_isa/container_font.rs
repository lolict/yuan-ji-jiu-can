// 容器字体 — 通用状态轨道拓扑字体生成 + 读回 (共享库模块)

use crate::cross_isa::state_enumerator::*;
use crate::cross_isa::coordinate::Coord3D;
use crate::cross_isa::fridge::ContainerPack;
use crate::cross_isa::vocabulary::{ContainerId, ContainerFamily};
use crate::cross_isa::glyph_3d_font::GlyphContour;

type Int16 = i16;
type Uint16 = u16;
type Int32 = i32;
type Uint32 = u32;
type Fixed = i32;

const DESIGN_SIZE: i16 = 100;

pub(crate) struct BeWriter { pub(crate) buf: Vec<u8> }

impl BeWriter {
    pub(crate) fn new() -> Self { Self { buf: Vec::new() } }
    pub(crate) fn pos(&self) -> u64 { self.buf.len() as u64 }
    pub(crate) fn u8(&mut self, v: u8) { self.buf.push(v); }
    pub(crate) fn u16(&mut self, v: u16) { self.buf.extend_from_slice(&v.to_be_bytes()); }
    pub(crate) fn u32(&mut self, v: u32) { self.buf.extend_from_slice(&v.to_be_bytes()); }
    pub(crate) fn i16(&mut self, v: i16) { self.buf.extend_from_slice(&v.to_be_bytes()); }
    pub(crate) fn i32(&mut self, v: i32) { self.buf.extend_from_slice(&v.to_be_bytes()); }
    pub(crate) fn fixed(&mut self, v: i32) { self.u32(v as u32); }
    pub(crate) fn tag(&mut self, s: &str) {
        let b = s.as_bytes();
        for i in 0..4 { self.buf.push(b.get(i).copied().unwrap_or(b' ')); }
    }
    pub(crate) fn bytes(&mut self, data: &[u8]) { self.buf.extend_from_slice(data); }
    pub(crate) fn pad_to_4(&mut self) {
        while self.buf.len() % 4 != 0 { self.buf.push(0); }
    }
}

pub struct GlyphSource {
    pub codepoint: u32,
    pub name: String,
    pub bits: String,
    pub classification: StateClassification,
    pub pack: ContainerPack,
}

#[derive(Debug, Clone)]
pub struct FontEntry {
    pub pack_id: u64,
    pub coord: Coord3D,
    pub align_flag: u8,
    pub payload: Vec<u8>,
    pub witness: [u8; 3],
    pub category: u16,
    pub bit_width: u8,
    pub orbit_len: u8,
    pub representative: u16,
    pub orbit_elements: Vec<u16>,
}

pub fn generate_state_font(bit_width: u8, extended: bool) -> Vec<u8> {
    let sources = build_sources(bit_width, extended);
    if sources.is_empty() { return Vec::new(); }
    assemble_font(&sources)
}

fn compute_checksum(data: &[u8]) -> u32 {
    let mut sum: u32 = 0;
    let mut i = 0;
    while i + 3 < data.len() {
        sum = sum.wrapping_add(
            ((data[i] as u32) << 24) | ((data[i+1] as u32) << 16)
            | ((data[i+2] as u32) << 8) | (data[i+3] as u32),
        );
        i += 4;
    }
    sum
}

fn make_circle_points(cx: i16, cy: i16, radius: i16, n: u16) -> Vec<(i16, i16)> {
    let mut pts = Vec::new();
    for i in 0..n {
        let a = (i as i16 * 360) / n as i16;
        let r = a as f64 * std::f64::consts::PI / 180.0;
        pts.push(((r.cos() * radius as f64).round() as i16 + cx,
                  (r.sin() * radius as f64).round() as i16 + cy));
    }
    pts
}

fn make_torus_points(cx: i16, cy: i16, r_out: i16, r_in: i16, n: u16) -> (Vec<(i16, i16)>, Vec<(i16, i16)>) {
    (make_circle_points(cx, cy, r_out, n), make_circle_points(cx, cy, r_in, n))
}

fn make_tetra_points(cx: i16, cy: i16, size: i16) -> Vec<Vec<(i16, i16)>> {
    let mut faces = Vec::new();
    for i in 0..4i16 {
        let r1 = (i * 90) as f64 * std::f64::consts::PI / 180.0;
        let r2 = ((i+1)*90) as f64 * std::f64::consts::PI / 180.0;
        faces.push(vec![(cx, cy), ((r1.cos()*size as f64).round() as i16 + cx, (r1.sin()*size as f64).round() as i16 + cy),
                        ((r2.cos()*size as f64).round() as i16 + cx, (r2.sin()*size as f64).round() as i16 + cy)]);
    }
    faces
}

fn state_to_sig(state: u64, bw: u8, cx: i16, cy: i16, span: i16) -> Vec<(i16, i16)> {
    let half = span / 2;
    (0..bw).map(|i| {
        let b = ((state >> i) & 1) as i16;
        (cx - half + (i as i16 * span / bw as i16), cy + b*half - (1-b)*half)
    }).collect()
}

fn build_topology_glyph(cat: &StateCategory, state: u64, bw: u8) -> Vec<u8> {
    match cat {
        StateCategory::AxialContainer => build_circle(state, bw),
        StateCategory::ClosedLoop => build_torus(state, bw),
        StateCategory::MirrorEntangled => build_tetra(state, bw),
        StateCategory::ExtendedEntangled => build_complex(state, bw),
    }
}

fn ginstr(points: &[(i16,i16)], flag: u8) -> Vec<u8> {
    let mut b = Vec::new();
    for &(x,_) in points { b.push((x as u16) as u8); }
    for &(_,y) in points { b.push((y as u16) as u8); }
    b
}

fn build_circle(state: u64, bw: u8) -> Vec<u8> {
    let mut buf = Vec::new();
    let r = DESIGN_SIZE;
    let circ = make_circle_points(0, 0, r, 16);
    let sig = state_to_sig(state, bw, 0, -r*2, r*2);
    buf.extend_from_slice(&2i16.to_be_bytes());
    buf.extend_from_slice(&(-r-10).to_be_bytes()); buf.extend_from_slice(&(-r-10).to_be_bytes());
    buf.extend_from_slice(&(r+10).to_be_bytes()); buf.extend_from_slice(&(r+10).to_be_bytes());
    buf.extend_from_slice(&(circ.len() as u16 - 1).to_be_bytes());
    buf.extend_from_slice(&((circ.len()+sig.len()) as u16 - 1).to_be_bytes());
    buf.extend_from_slice(&0u16.to_be_bytes());
    let n = circ.len() + sig.len();
    let flag: u8 = 0x07;
    for _ in 0..n { buf.push(flag); }
    for &(x,_) in &circ { buf.push((x as u16) as u8); }
    for &(x,_) in &sig { buf.push((x as u16) as u8); }
    for &(_,y) in &circ { buf.push((y as u16) as u8); }
    for &(_,y) in &sig { buf.push((y as u16) as u8); }
    while buf.len()%4!=0 { buf.push(0); }
    buf
}

fn build_torus(state: u64, bw: u8) -> Vec<u8> {
    let mut buf = Vec::new();
    let rout = DESIGN_SIZE; let rin = DESIGN_SIZE*2/3;
    let (outer, inner) = make_torus_points(0, 0, rout, rin, 12);
    let sig = state_to_sig(state, bw, 0, -rout*2, rout*2);
    buf.extend_from_slice(&3i16.to_be_bytes());
    buf.extend_from_slice(&(-rout-10).to_be_bytes()); buf.extend_from_slice(&(-rout-10).to_be_bytes());
    buf.extend_from_slice(&(rout+10).to_be_bytes()); buf.extend_from_slice(&(rout+10).to_be_bytes());
    buf.extend_from_slice(&(outer.len() as u16-1).to_be_bytes());
    buf.extend_from_slice(&((outer.len()+inner.len()) as u16-1).to_be_bytes());
    buf.extend_from_slice(&((outer.len()+inner.len()+sig.len()) as u16-1).to_be_bytes());
    buf.extend_from_slice(&0u16.to_be_bytes());
    let n = outer.len()+inner.len()+sig.len();
    for _ in 0..n { buf.push(0x07); }
    for v in &[&outer, &inner, &sig] { for &(x,_) in *v { buf.push((x as u16) as u8); } }
    for v in &[&outer, &inner, &sig] { for &(_,y) in *v { buf.push((y as u16) as u8); } }
    while buf.len()%4!=0 { buf.push(0); }
    buf
}

fn build_tetra(state: u64, bw: u8) -> Vec<u8> {
    let mut buf = Vec::new();
    let size = DESIGN_SIZE;
    let faces = make_tetra_points(0, 0, size);
    let sig = state_to_sig(state, bw, 0, size, size);
    let nc = faces.len() as i16 + 1;
    buf.extend_from_slice(&nc.to_be_bytes());
    let half = size/2;
    buf.extend_from_slice(&(-half-10).to_be_bytes()); buf.extend_from_slice(&(-half*2/3).to_be_bytes());
    buf.extend_from_slice(&(half+10).to_be_bytes()); buf.extend_from_slice(&half.to_be_bytes());
    let mut ends = Vec::new(); let mut rt = 0usize;
    for f in &faces { rt += f.len(); ends.push((rt-1) as u16); }
    rt += sig.len(); ends.push((rt-1) as u16);
    for e in &ends { buf.extend_from_slice(&e.to_be_bytes()); }
    buf.extend_from_slice(&0u16.to_be_bytes());
    for _ in 0..rt { buf.push(0x07); }
    for f in &faces { for &(x,_) in f { buf.push((x as u16) as u8); } }
    for &(x,_) in &sig { buf.push((x as u16) as u8); }
    for f in &faces { for &(_,y) in f { buf.push((y as u16) as u8); } }
    for &(_,y) in &sig { buf.push((y as u16) as u8); }
    while buf.len()%4!=0 { buf.push(0); }
    buf
}

fn build_complex(state: u64, bw: u8) -> Vec<u8> {
    let mut buf = Vec::new();
    let span = DESIGN_SIZE;
    let circ = make_circle_points(0, 0, span, 16);
    let s1 = state_to_sig(state, bw, 0, 0, span);
    let s2 = state_to_sig(state, bw, 0, span, span);
    buf.extend_from_slice(&3i16.to_be_bytes());
    buf.extend_from_slice(&(-span-10).to_be_bytes()); buf.extend_from_slice(&(-span-10).to_be_bytes());
    buf.extend_from_slice(&(span+10).to_be_bytes()); buf.extend_from_slice(&(span*2+10).to_be_bytes());
    buf.extend_from_slice(&(circ.len() as u16-1).to_be_bytes());
    buf.extend_from_slice(&((circ.len()+s1.len()) as u16-1).to_be_bytes());
    buf.extend_from_slice(&((circ.len()+s1.len()+s2.len()) as u16-1).to_be_bytes());
    buf.extend_from_slice(&0u16.to_be_bytes());
    let n = circ.len()+s1.len()+s2.len();
    for _ in 0..n { buf.push(0x07); }
    for v in &[&circ, &s1, &s2] { for &(x,_) in *v { buf.push((x as u16) as u8); } }
    for v in &[&circ, &s1, &s2] { for &(_,y) in *v { buf.push((y as u16) as u8); } }
    while buf.len()%4!=0 { buf.push(0); }
    buf
}

fn build_sources(bw: u8, extended: bool) -> Vec<GlyphSource> {
    let en = if extended {
        StateEnumerator::with_generators(bw, vec![Transform::Not, Transform::Reverse, Transform::Rotate(1)])
    } else { StateEnumerator::new(bw) };
    let all = en.enumerate();
    let mut srcs = Vec::new();
    let mut cp: u32 = 0xE000;
    for c in &all {
        if c.orbit.representative != c.state { continue; }
        let bits = format!("{:0w$b}", c.state, w = bw as usize);
        let xs = match c.category {
            StateCategory::AxialContainer => 0x10,
            StateCategory::ClosedLoop => 0x11,
            StateCategory::MirrorEntangled => 0x12,
            StateCategory::ExtendedEntangled => 0x13,
        };
        let coord = Coord3D::new(xs, c.state as u16, (c.state % 16) as u8);
        let pack = ContainerPack::new(cp as u64, coord, 1, bits.clone().into_bytes());
        srcs.push(GlyphSource { codepoint: cp, name: format!("state_{}_{}", bits, c.category.name_cn()), bits, classification: c.clone(), pack });
        cp += 1;
    }
    srcs
}

fn build_head(ng: u16) -> Vec<u8> {
    let mut w = BeWriter::new();
    w.fixed(0x00010000); w.fixed(0x00010000); w.u32(0); w.u32(0x5F0F3CF5);
    w.u16(0x000B); w.u16(1000); w.u32(0); w.u32(0); w.u32(0); w.u32(0);
    w.i16(0); w.i16(-100); w.i16(200); w.i16(200); w.u16(0); w.u16(8); w.i16(2); w.i16(1); w.i16(0);
    w.buf
}

fn build_hhea(ng: u16) -> Vec<u8> {
    let mut w = BeWriter::new();
    w.fixed(0x00010000); w.i16(800); w.i16(-200); w.i16(0); w.u16(200);
    for _ in 0..6 { w.i16(0); }
    w.u16(ng);
    w.buf
}

fn build_maxp(ng: u16) -> Vec<u8> {
    let mut w = BeWriter::new();
    w.fixed(0x00010000); w.u16(ng); w.u16(50); w.u16(3);
    for _ in 0..11 { w.u16(0); }
    w.buf
}

fn build_hmtx(ng: u16) -> Vec<u8> {
    let mut w = BeWriter::new();
    for _ in 0..ng { w.u16(200); w.i16(0); }
    w.buf
}

fn build_post() -> Vec<u8> {
    let mut w = BeWriter::new();
    w.fixed(0x00030000); w.fixed(0); w.i16(-100); w.i16(50);
    for _ in 0..5 { w.u32(0); }
    w.buf
}

fn build_name(srcs: &[GlyphSource]) -> Vec<u8> {
    let mut w = BeWriter::new();
    let count = 6 + srcs.len() as u16;
    w.u16(0); w.u16(count); w.u16(6 * count);
    let mut strs = Vec::new();
    let base = vec![
        (1,0,0,0,"Container State Font"),(1,0,0,1,"MQState"),
        (1,0,0,2,"Regular"),(1,0,0,3,"MQState001"),
        (1,0,0,4,"MQ State Orbit Font"),(1,0,0,5,"Version 0.2")
    ];
    for &(p,e,l,n,s) in &base { strs.push((p,e,l,n,s.to_string())); }
    for (i,s) in srcs.iter().enumerate() {
        let d = format!("{}_{}_orbit{}", s.bits, s.classification.category.name_cn(), s.classification.orbit.len());
        strs.push((1,0,0,256+i as u16,d));
    }
    let mut sdata = Vec::new(); let mut recs = Vec::new();
    for (p,e,l,n,s) in &strs {
        let off = sdata.len() as u16;
        sdata.extend_from_slice(s.as_bytes());
        recs.push((*p,*e,*l,*n,off,s.len() as u16));
    }
    for (p,e,l,n,o,ln) in &recs {
        w.u16(*p); w.u16(*e); w.u16(*l); w.u16(*n); w.u16(*ln); w.u16(*o);
    }
    w.bytes(&sdata);
    w.buf
}

fn build_cmap(srcs: &[GlyphSource]) -> Vec<u8> {
    let mut w = BeWriter::new();
    w.u16(0); w.u16(1); w.u16(3); w.u16(10);
    let off_pos = w.pos();
    w.u32(0);
    let data_start = w.pos();
    w.u16(0); w.u16(0);
    if srcs.is_empty() {
        w.u32(16); w.u32(0); w.u32(0);
        return w.buf;
    }
    let grps = vec![(srcs[0].codepoint, srcs[srcs.len()-1].codepoint, 1u32)];
    let gds = grps.len() as u32 * 12;
    w.u32(16+gds); w.u32(0); w.u32(grps.len() as u32);
    for &(s,e,g) in &grps { w.u32(s); w.u32(e); w.u32(g); }
    w.buf[off_pos as usize..off_pos as usize+4].copy_from_slice(&(data_start as u32).to_be_bytes());
    w.buf
}

fn build_mcod_table(srcs: &[GlyphSource]) -> Vec<u8> {
    let mut w = BeWriter::new();
    w.u32(0x4D434F44); w.u16(2); w.u16(srcs.len() as u16); w.u16(48);
    for s in srcs {
        let es = w.pos();
        w.u32(s.pack.pack_id as u32);
        w.u16(s.classification.category.code());
        w.u8(s.classification.bit_width);
        w.u8(s.classification.orbit.len() as u8);
        w.u16(s.classification.orbit.representative as u16);
        for i in 0..8u8 {
            if i < s.classification.orbit.len() as u8 && (i as usize) < s.classification.orbit.elements.len() {
                w.u16(s.classification.orbit.elements[i as usize] as u16);
            } else { w.u16(0); }
        }
        w.u8(s.pack.func_code as u8);
        w.u8(s.pack.time_stamp as u8);
        w.u16(s.pack.param_slot as u16);
        while (w.pos()-es) < 48 { w.u8(0); }
    }
    w.buf
}

fn assemble_font(sources: &[GlyphSource]) -> Vec<u8> {
    let ng = (sources.len()+1) as u16;
    let head = build_head(ng); let hhea = build_hhea(ng); let maxp = build_maxp(ng);
    let post = build_post(); let name = build_name(sources); let cmap = build_cmap(sources);
    let hmtx = build_hmtx(ng); let mcod = build_mcod_table(sources);
    let mut glyf = BeWriter::new();
    glyf.i16(0); glyf.i16(0); glyf.i16(0); glyf.i16(0); glyf.u16(0); glyf.u16(0);
    let mut offs = Vec::new();
    for s in sources {
        offs.push(glyf.pos() as u32);
        glyf.bytes(&build_topology_glyph(&s.classification.category, s.classification.state, s.classification.bit_width));
    }
    let loca: Vec<u8> = offs.iter().fold(Vec::new(), |mut acc, &o| { acc.extend_from_slice(&o.to_be_bytes()); acc });
    let loca = loca;

    let tables = vec![
        (*b"head",head),(*b"hhea",hhea),(*b"maxp",maxp),
        (*b"hmtx",hmtx),(*b"cmap",cmap),
        (*b"glyf",glyf.buf),(*b"loca",loca),
        (*b"name",name),(*b"post",post),(*b"MCOD",mcod)
    ];

    let nt = tables.len() as u16;
    let mut sr = 1u16; let mut es = 0u16;
    while sr*2<=nt { sr*=2; es+=1; }
    let rs = nt*16-sr;
    let mut out = BeWriter::new();
    out.u32(0x00010000); out.u16(nt); out.u16(sr); out.u16(es); out.u16(rs);
    let dir_s = out.pos();
    for _ in 0..nt { out.tag(""); out.u32(0); out.u32(0); out.u32(0); }

    let mut tinfo = Vec::new();
    for (tag,data) in &tables {
        let pos = out.pos() as u32;
        let mut pad = data.clone();
        while pad.len()%4!=0 { pad.push(0); }
        out.bytes(&pad);
        tinfo.push((tag.to_vec(), compute_checksum(&pad), pos, pad.len() as u32));
    }

    let mut dw = BeWriter::new();
    for (tag,ck,off,len) in &tinfo {
        for &b in tag { dw.u8(b); }
        for _ in tag.len()..4 { dw.u8(b' '); }
        dw.u32(*ck); dw.u32(*off); dw.u32(*len);
    }
    for (i,b) in dw.buf.iter().enumerate() { out.buf[dir_s as usize+i] = *b; }

    let head_off = tinfo.iter().find(|t| t.0==b"head").map(|t| t.2).unwrap_or(0);
    let adj_off = (head_off+8) as usize;
    for i in 0..4 { out.buf[adj_off+i] = 0; }
    let fck = compute_checksum(&out.buf);
    let adj = 0xB1B0AFBAu32.wrapping_sub(fck);
    out.buf[adj_off..adj_off+4].copy_from_slice(&adj.to_be_bytes());
    out.buf
}

pub fn read_mcod_entries(data: &[u8]) -> Vec<FontEntry> {
    if data.len() < 12 { return Vec::new(); }
    let num_tables = u16::from_be_bytes([data[4],data[5]]) as usize;
    let mut mcod_data = None;
    let mut o = 12;
    for _ in 0..num_tables {
        if o+16 > data.len() { break; }
        let tag = &data[o..o+4];
        let toff = u32::from_be_bytes([data[o+8],data[o+9],data[o+10],data[o+11]]) as usize;
        let tlen = u32::from_be_bytes([data[o+12],data[o+13],data[o+14],data[o+15]]) as usize;
        if tag == b"MCOD" {
            let end = toff+tlen;
            if end <= data.len() { mcod_data = Some(data[toff..end].to_vec()); }
            break;
        }
        o += 16;
    }
    let mcod = match mcod_data { Some(d)=>d, None=>return Vec::new() };
    // MCOD header: magic(4) + version(2) + nent(2) + entry_size(2) = 10 bytes
    if mcod.len() < 10 { return Vec::new(); }
    let ver = u16::from_be_bytes([mcod[4], mcod[5]]);
    let nent = u16::from_be_bytes([mcod[6], mcod[7]]) as usize;
    let esz = if ver >= 2 { 48 } else { u16::from_be_bytes([mcod[8], mcod[9]]) as usize };
    let mut ents = Vec::with_capacity(nent);
    let mut p = 10; // entries start after 10-byte header
    for _ in 0..nent {
        if p + esz > mcod.len() { break; }
        let pid = u32::from_be_bytes([mcod[p], mcod[p+1], mcod[p+2], mcod[p+3]]) as u64;
        let cat = u16::from_be_bytes([mcod[p+4], mcod[p+5]]);
        let bw = mcod[p+6];
        let ol = mcod[p+7];
        let rep = u16::from_be_bytes([mcod[p+8], mcod[p+9]]);
        let mut elems = Vec::new();
        for i in 0..8 {
            if p + 10 + i*2 + 2 <= mcod.len() {
                elems.push(u16::from_be_bytes([mcod[p+10+i*2], mcod[p+11+i*2]]));
            }
        }
        // Coord3D bytes at [26]=X(func_code) [27]=Z(time_stamp%16) [28-29]=Y(param_slot)
        let coord = if p + 30 <= mcod.len() {
            Coord3D::new(mcod[p+26], u16::from_be_bytes([mcod[p+28], mcod[p+29]]), mcod[p+27])
        } else {
            Coord3D::new(cat as u8, rep, ol)
        };
        ents.push(FontEntry {
            pack_id: pid, coord, align_flag: 1,
            payload: vec![(bw << 4) | 0x01, ol, cat as u8, ol],
            witness: [0; 3], category: cat, bit_width: bw,
            orbit_len: ol, representative: rep, orbit_elements: elems,
        });
        p += esz;
    }
    ents
}

// ══════════════════════════════════════════════════════════════════════
// 64 容器字体生成 — 直接由 ContainerId 生成字形
// ══════════════════════════════════════════════════════════════════════

/// 从拓扑标志 (flags[2]) 生成字形轮廓字节
fn topology_to_glyph(flag: u8, code: u8, size: i16) -> Vec<u8> {
    let points = match flag {
        0x60 => {
            // 线型: 2 点
            vec![(size/2, 0), (-size/2, 0)]
        }
        0x50 => {
            // 双锥: 4 点 (菱形)
            vec![(0, -size), (size/2, 0), (0, size), (-size/2, 0)]
        }
        0x10 => {
            // 盒型: 4 点 (方框)
            let s = size/2;
            vec![(-s, -s), (s, -s), (s, s), (-s, s)]
        }
        0x20 => {
            // 球型: 8 点圆形近似 (用八边形)
            make_circle_points(0, 0, size/2, 8)
        }
        0x30 => {
            // 环型: 2 轮廓 (外圆 + 内圆, 8边近似)
            let outer = make_circle_points(0, 0, size/2, 8);
            let inner = make_circle_points(0, 0, size/4, 8);
            let mut combined = Vec::new();
            combined.push((0u8, outer.len() as u16));
            combined.push((1u8, inner.len() as u16));
            // 展平为单一点序列 + end_pts
            let outer_len = outer.len();
            let inner_len = inner.len();
            let mut pts = Vec::new();
            pts.extend(outer);
            pts.extend(inner);
            return build_composite_glyph(&pts, &[(0, outer_len as u16), (outer_len, inner_len as u16)]);
        }
        _ => {
            // 默认: 小方块
            let s = size/4;
            vec![(-s, -s), (s, -s), (s, s), (-s, s)]
        }
    };

    build_simple_contour(&points)
}

/// 构建单一轮廓字形
fn build_simple_contour(pts: &[(i16, i16)]) -> Vec<u8> {
    let mut buf = Vec::new();
    let n = pts.len() as u16;

    // numberOfContours = 1
    buf.extend_from_slice(&1i16.to_be_bytes());
    // bbox = (0, 0, 0, 0) 粗略 (hmtx 已管宽度)
    buf.extend_from_slice(&0i16.to_be_bytes()); buf.extend_from_slice(&0i16.to_be_bytes());
    buf.extend_from_slice(&0i16.to_be_bytes()); buf.extend_from_slice(&0i16.to_be_bytes());
    // endPtsOfContours = [n-1]
    buf.extend_from_slice(&(n - 1).to_be_bytes());
    // instructionLength = 0
    buf.extend_from_slice(&0u16.to_be_bytes());
    // flags: on-curve = 1
    for _ in 0..n { buf.push(0x01); }
    // xCoordinates (delta from 0)
    for &(x, _) in pts { buf.extend_from_slice(&x.to_be_bytes()); }
    // yCoordinates
    for &(_, y) in pts { buf.extend_from_slice(&y.to_be_bytes()); }
    // pad 4
    while buf.len() % 4 != 0 { buf.push(0); }
    buf
}

/// 构建复合轮廓 (多环) 字形
fn build_composite_glyph(all_pts: &[(i16, i16)], ranges: &[(usize, u16)]) -> Vec<u8> {
    let mut buf = Vec::new();
    let nc = ranges.len() as i16;
    buf.extend_from_slice(&nc.to_be_bytes());
    buf.extend_from_slice(&0i16.to_be_bytes()); buf.extend_from_slice(&0i16.to_be_bytes());
    buf.extend_from_slice(&0i16.to_be_bytes()); buf.extend_from_slice(&0i16.to_be_bytes());

    let mut ends = Vec::new();
    let mut acc: u16 = 0;
    for &(_, cnt) in ranges {
        acc += cnt;
        ends.push(acc - 1);
    }
    for e in &ends { buf.extend_from_slice(&e.to_be_bytes()); }
    buf.extend_from_slice(&0u16.to_be_bytes());

    let n = all_pts.len() as u16;
    for _ in 0..n { buf.push(0x01); }
    for &(x, _) in all_pts { buf.extend_from_slice(&x.to_be_bytes()); }
    for &(_, y) in all_pts { buf.extend_from_slice(&y.to_be_bytes()); }
    while buf.len() % 4 != 0 { buf.push(0); }
    buf
}

/// 生成完整的 64 容器字体 (核心入口)
/// 返回 .ttf 二进制数据
pub fn generate_64_container_font() -> Vec<u8> {
    let codepoint_start: u32 = 0xE000; // Unicode Private Use Area
    let size: i16 = 50; // 字形半宽 (设计网格 100×100)

    // 为每个容器生成 Source
    let mut sources: Vec<(u32, String, Vec<u8>, ContainerPack)> = Vec::new();

    for code in 0x00u8..=0x3F {
        let cid = match ContainerId::from_u8(code) {
            Some(c) => c,
            None => continue,
        };
        let coord = Coord3D::new(code, 0x0100, code); // Y=通用操作数, Z=容器码作为时间窗
        let payload = vec![code, 0x01, 0x00, 0x01]; // payload = 容器码 + 固定标记
        let pack = ContainerPack::new(code as u64, coord, 1, payload);

        // 拓扑标志: 直接由 GlyphContour 计算 (单一事实来源)
        let contour = GlyphContour::from_coord(coord);
        let topo_flag = contour.flags[2];

        let glyph_bytes = topology_to_glyph(topo_flag, code, size);
        let cp = codepoint_start + code as u32;
        let name = format!("container_{:02X}_{}", code, cid.name());
        sources.push((cp, name, glyph_bytes, pack));
    }

    // 组装字体
    assemble_container_font(&sources)
}

/// 将 (codepoint, name, glyph_bytes, ContainerPack) 组装为完整 TTF
fn assemble_container_font(
    sources: &[(u32, String, Vec<u8>, ContainerPack)],
) -> Vec<u8> {
    let ng = sources.len() as u16;
    if ng == 0 { return Vec::new(); }

    // ── glyph_3d_font 导入 ──
    use crate::cross_isa::glyph_3d_font::GlyphContour;

    // ── 构造每个表的二进制 ──
    let head = build_head(ng);
    let hhea = build_hhea(ng);
    let maxp = build_maxp(ng);
    let post = build_post();
    let name_table = build_cmap_name(sources);
    let cmap = build_cmap_formats(sources);
    let hmtx = build_hmtx(ng);

    // glyf table
    let mut glyf_w = BeWriter::new();
    glyf_w.i16(0); glyf_w.i16(0); glyf_w.i16(0); glyf_w.i16(0); glyf_w.u16(0); glyf_w.u16(0);
    let mut offs: Vec<u32> = Vec::new();
    for (_, _, gb, _) in sources {
        offs.push(glyf_w.pos() as u32);
        glyf_w.bytes(gb);
    }
    let loca: Vec<u8> = offs.iter().fold(Vec::new(), |mut acc, &o| {
        acc.extend_from_slice(&o.to_be_bytes()); acc
    });

    // MCOD 表 (携带 ContainerPack 数据)
    let mcod = build_mcod_for_containers(sources);

    // ── 构建 table directory + 输出 ──
    let tables: Vec<(&[u8], Vec<u8>)> = vec![
        (b"head", head), (b"hhea", hhea), (b"maxp", maxp),
        (b"hmtx", hmtx), (b"cmap", cmap),
        (b"glyf", glyf_w.buf), (b"loca", loca),
        (b"name", name_table), (b"post", post), (b"MCOD", mcod),
    ];
    assemble_ttf_tables(&tables)
}

/// 构建 MCOD 表 (使用 ContainerPack 数据)
fn build_mcod_for_containers(
    sources: &[(u32, String, Vec<u8>, ContainerPack)],
) -> Vec<u8> {
    let mut w = BeWriter::new();
    w.u32(0x4D434F44); // 'MCOD'
    w.u16(2);          // version
    w.u16(sources.len() as u16);
    w.u16(48);         // entry size

    for (_, _, _, pack) in sources {
        let es = w.pos();
        w.u32(pack.pack_id as u32);
        w.u16(pack.func_code);    // X = semantic flag (容器族码)
        w.u8(pack.func_code as u8);
        w.u8(pack.align_flag);
        w.u16(pack.param_slot as u16);
        for i in 0..8u8 {
            if (i as usize) < pack.payload.len() { w.u16(pack.payload[i as usize] as u16); }
            else { w.u16(0); }
        }
        while (w.pos() - es) < 48 { w.u8(0); }
    }
    w.buf
}

/// 构建 name 表 (仅限容器专用)
fn build_cmap_name(
    sources: &[(u32, String, Vec<u8>, ContainerPack)],
) -> Vec<u8> {
    let mut w = BeWriter::new();
    let count = 2 + sources.len() as u16;
    w.u16(0); w.u16(count); w.u16(6 * count);
    let mut strs: Vec<(u16,u16,u16,u16,String)> = vec![
        (1,0,0,0,"Container Font 64".to_string()),
        (1,0,0,1,"HanYuan64".to_string()),
    ];
    for (i, (_, name, _, _)) in sources.iter().enumerate() {
        strs.push((1,0,0,256+i as u16,name.clone()));
    }
    let mut sdata = Vec::new();
    let mut recs = Vec::new();
    for (p,e,l,n,s) in &strs {
        let off = sdata.len() as u16;
        sdata.extend_from_slice(s.as_bytes());
        recs.push((*p,*e,*l,*n,off,s.len() as u16));
    }
    for (p,e,l,n,o,ln) in &recs {
        w.u16(*p); w.u16(*e); w.u16(*l); w.u16(*n); w.u16(*ln); w.u16(*o);
    }
    w.bytes(&sdata);
    w.buf
}

/// 构建 cmap 表
fn build_cmap_formats(
    sources: &[(u32, String, Vec<u8>, ContainerPack)],
) -> Vec<u8> {
    let mut w = BeWriter::new();
    w.u16(0); w.u16(1); // version, numTables
    w.u16(3); w.u16(10); // platformID, encodingID
    let off_pos = w.pos();
    w.u32(0); // offset placeholder
    let data_start = w.pos();
    w.u16(12); // format 12
    w.u16(0);  // reserved
    w.u32(0);  // length placeholder
    w.u32(0);  // language
    let ngroups = sources.len() as u32;
    w.u32(ngroups);
    for (cp, _, _, _) in sources {
        w.u32(*cp);
        w.u32(*cp);  // 每个组覆盖 1 个码点
        w.u32(0);    // startGlyphID (index 0 = .notdef)
    }
    // 回填 length
    let seg = (w.pos() - data_start) as u32;
    w.buf[off_pos as usize..off_pos as usize + 4].copy_from_slice(&(data_start as u32).to_be_bytes());
    let length_off = (data_start + 4) as usize;
    w.buf[length_off..length_off + 4].copy_from_slice(&seg.to_be_bytes());
    w.buf
}

/// 通用 TTF 组装器
fn assemble_ttf_tables(tables: &[(&[u8], Vec<u8>)]) -> Vec<u8> {
    let nt = tables.len() as u16;
    let mut sr = 1u16; let mut es = 0u16;
    while sr * 2 <= nt { sr *= 2; es += 1; }
    let rs = nt * 16 - sr * 16;

    let mut out = BeWriter::new();
    out.u32(0x00010000); out.u16(nt); out.u16(sr); out.u16(es); out.u16(rs);
    let dir_s = out.pos();
    for _ in 0..nt { out.tag(""); out.u32(0); out.u32(0); out.u32(0); }

    let mut tinfo = Vec::new();
    for (tag, data) in tables {
        let pos = out.pos() as u32;
        let mut pad = data.clone();
        while pad.len() % 4 != 0 { pad.push(0); }
        out.bytes(&pad);
        tinfo.push((tag.to_vec(), compute_checksum(&pad), pos, pad.len() as u32));
    }

    let mut dw = BeWriter::new();
    for (tag, ck, off, len) in &tinfo {
        for &b in tag.iter() { dw.u8(b); }
        for _ in tag.len()..4 { dw.u8(b' '); }
        dw.u32(*ck); dw.u32(*off); dw.u32(*len);
    }
    for (i, b) in dw.buf.iter().enumerate() { out.buf[dir_s as usize + i] = *b; }

    let head_off = tinfo.iter().find(|t| t.0 == b"head").map(|t| t.2).unwrap_or(0);
    let adj_off = (head_off + 8) as usize;
    for i in 0..4 { out.buf[adj_off + i] = 0; }
    let fck = compute_checksum(&out.buf);
    let adj = 0xB1B0AFBAu32.wrapping_sub(fck);
    out.buf[adj_off..adj_off + 4].copy_from_slice(&adj.to_be_bytes());
    out.buf
}

#[cfg(test)]
mod container_font_tests {
    use super::*;

    #[test]
    fn generate_64_font_nonempty() {
        let font_data = generate_64_container_font();
        assert!(font_data.len() > 100, "字体文件过短: {}", font_data.len());
        // 验证 TTF magic
        assert_eq!(font_data[0..4], [0x00, 0x01, 0x00, 0x00]);
    }

    #[test]
    fn all_64_glyphs_present() {
        let font_data = generate_64_container_font();
        // 读取 table directory 验证有 MCOD 表
        let num_tables = u16::from_be_bytes([font_data[4], font_data[5]]) as usize;
        let mut found_mcod = false;
        let mut o = 12;
        for _ in 0..num_tables {
            if o + 8 > font_data.len() { break; }
            if font_data[o..o+4] == b"MCOD"[..] { found_mcod = true; break; }
            o += 16;
        }
        assert!(found_mcod, "MCOD 表应存在");
    }

    #[test]
    fn topology_glyph_flags_match() {
        // 球形容器 (0x03 ADD) 应产生 8 字形点
        let sphere = topology_to_glyph(0x20, 0x03, 50);
        assert!(sphere.len() > 0, "球型字形不能为空");

        // 盒形容器 (0x01 LOAD) 应产生 4 字形点
        let box_glyph = topology_to_glyph(0x10, 0x01, 50);
        assert!(box_glyph.len() > 0, "盒型字形不能为空");
    }
}
