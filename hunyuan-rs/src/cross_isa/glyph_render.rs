// glyph_render — 字体样张渲染器

use image::{Rgb, RgbImage, imageops};
use image::codecs::gif::GifEncoder;
use image::Frame;
use std::fs;
use std::io::BufWriter;
use super::container_font::FontEntry;

pub const RW: u32 = 1600;
pub const RH: u32 = 1200;

#[derive(Clone)]
pub struct RenderEntry {
    pub pack_id: u64,
    pub category: u8,
    pub bit_width: u8,
    pub orbit_len: u8,
    pub representative: u16,
    pub z_beat: u8,
    pub x_sem: u8,
    pub y_op: u16,
    pub orbit_elems: Vec<u16>,
}

impl From<&FontEntry> for RenderEntry {
    fn from(e: &FontEntry) -> Self {
        Self {
            pack_id: e.pack_id,
            category: e.category as u8,
            bit_width: e.bit_width,
            orbit_len: e.orbit_len,
            representative: e.representative,
            z_beat: e.coord.z,
            x_sem: e.coord.x,
            y_op: e.coord.y,
            orbit_elems: e.orbit_elements.clone(),
        }
    }
}

impl RenderEntry {
    pub fn cat_color(&self) -> Rgb<u8> {
        match self.category {
            1 => Rgb([74, 144, 217]),
            2 => Rgb([80, 184, 108]),
            3 => Rgb([217, 140, 74]),
            4 => Rgb([160, 100, 200]),
            _ => Rgb([136, 136, 136]),
        }
    }
    pub fn cat_name(&self) -> &'static str {
        match self.category {
            1 => "轴心容器", 2 => "闭环态", 3 => "镜像纠缠", 4 => "扩展纠缠", _ => "???",
        }
    }
    pub fn cat_code(&self) -> &'static str {
        match self.category {
            1 => "AXIS", 2 => "LOOP", 3 => "MIRROR", 4 => "EXT", _ => "???",
        }
    }
    fn bits_str(&self, val: u16) -> String {
        let mut s = String::new();
        for i in (0..self.bit_width).rev() {
            s.push(if (val >> i) & 1 == 1 { '1' } else { '0' });
        }
        s
    }
    pub fn rep_bits(&self) -> String { self.bits_str(self.representative) }
    pub fn orbit_bits(&self) -> Vec<String> {
        self.orbit_elems.iter().map(|&e| self.bits_str(e)).collect()
    }
}

// Monochrome bitmap font
const FONT_5X7: [[u8; 5]; 37] = [
    [0x0E,0x11,0x13,0x15,0x0E],[0x04,0x0C,0x04,0x04,0x0E],[0x0E,0x01,0x0E,0x10,0x1F],
    [0x0E,0x01,0x06,0x01,0x0E],[0x11,0x11,0x1F,0x01,0x01],[0x1F,0x10,0x0E,0x01,0x0E],
    [0x0E,0x10,0x1E,0x11,0x0E],[0x1F,0x01,0x02,0x04,0x04],[0x0E,0x11,0x0E,0x11,0x0E],
    [0x0E,0x11,0x0F,0x01,0x0E],[0x0E,0x11,0x1F,0x11,0x11],[0x1E,0x11,0x1E,0x11,0x1E],
    [0x0F,0x10,0x10,0x10,0x0F],[0x1E,0x11,0x11,0x11,0x1E],[0x1F,0x10,0x1E,0x10,0x1F],
    [0x1F,0x10,0x1E,0x10,0x10],[0x0F,0x10,0x17,0x11,0x0F],[0x11,0x11,0x1F,0x11,0x11],
    [0x0E,0x04,0x04,0x04,0x0E],[0x07,0x02,0x02,0x12,0x0C],[0x11,0x12,0x1C,0x12,0x11],
    [0x10,0x10,0x10,0x10,0x1F],[0x11,0x1B,0x15,0x11,0x11],[0x11,0x19,0x15,0x13,0x11],
    [0x0E,0x11,0x11,0x11,0x0E],[0x1E,0x11,0x1E,0x10,0x10],[0x0E,0x11,0x15,0x12,0x0D],
    [0x1E,0x11,0x1E,0x12,0x11],[0x0F,0x10,0x0E,0x01,0x1E],[0x1F,0x04,0x04,0x04,0x04],
    [0x11,0x11,0x11,0x11,0x0E],[0x11,0x11,0x11,0x0A,0x04],[0x11,0x11,0x15,0x1B,0x11],
    [0x11,0x0A,0x04,0x0A,0x11],[0x11,0x0A,0x04,0x04,0x04],[0x1F,0x02,0x04,0x08,0x1F],
    [0x00,0x00,0x00,0x00,0x00],
];

fn ci(c: char) -> usize {
    match c { '0'..='9' => c as usize - 48, 'A'..='Z' => c as usize - 55, _ => 36 }
}

fn draw_char(img: &mut RgbImage, ch: char, ox: i32, oy: i32, s: i32, color: Rgb<u8>) {
    let p = FONT_5X7[ci(ch)];
    for r in 0..5i32 {
        for c in 0..5i32 {
            if (p[r as usize] >> (4 - c as u8)) & 1 == 1 {
                for dy in 0..s { for dx in 0..s {
                    let px = ox + c * s + dx; let py = oy + r * s + dy;
                    if px>=0 && py>=0 && px<RW as i32 && py<RH as i32 { img.put_pixel(px as u32, py as u32, color); }
                }}
            }
        }
    }
}

fn draw_text(img: &mut RgbImage, text: &str, ox: i32, oy: i32, s: i32, color: Rgb<u8>) {
    let mut cx = ox;
    for ch in text.chars() { draw_char(img, ch, cx, oy, s, color); cx += 6 * s; }
}

fn fr(img: &mut RgbImage, x0: i32, y0: i32, x1: i32, y1: i32, c: Rgb<u8>) {
    for y in y0.max(0)..=y1.min(RH as i32 - 1) { for x in x0.max(0)..=x1.min(RW as i32 - 1) { img.put_pixel(x as u32, y as u32, c); } }
}

fn br(img: &mut RgbImage, x0: i32, y0: i32, x1: i32, y1: i32, c: Rgb<u8>) {
    for x in x0..=x1 { if x>=0 && x<RW as i32 { if y0>=0&&y0<RH as i32 { img.put_pixel(x as u32,y0 as u32,c); } if y1>=0&&y1<RH as i32 { img.put_pixel(x as u32,y1 as u32,c); } } }
    for y in y0..=y1 { if y>=0 && y<RH as i32 { if x0>=0&&x0<RW as i32 { img.put_pixel(x0 as u32,y as u32,c); } if x1>=0&&x1<RW as i32 { img.put_pixel(x1 as u32,y as u32,c); } } }
}

pub fn render_snapshot(entries: &[FontEntry], out_path: &str) {
    let es: Vec<RenderEntry> = entries.iter().map(RenderEntry::from).collect();
    let mut img = RgbImage::from_pixel(RW, RH, Rgb([10, 10, 18]));

    // Banner
    fr(&mut img, 0, 0, RW as i32 - 1, 55, Rgb([25, 20, 45]));
    draw_text(&mut img, "MQ-OS CONTAINER FONT SPECIMEN", 20, 8, 3, Rgb([230, 210, 255]));
    draw_text(&mut img, &format!("{} CONTAINERS — STD MODE — BEAT=Z-AXIS", es.len()), 20, 38, 1, Rgb([160, 150, 180]));

    // Grid layout
    let n = es.len() as i32;
    let cols = if n <= 4 { 2 } else if n <= 9 { 3 } else if n <= 16 { 4 } else { 5 };
    let rows = (n + cols - 1) / cols;
    let mx = 30i32; let my = 80i32; let gap = 20i32;
    let cw = (RW as i32 - mx * 2 - gap * (cols - 1)) / cols;
    let ch = (RH as i32 - my - 50 - gap * (rows - 1)) / rows;

    for (idx, e) in es.iter().enumerate() {
        let col = idx as i32 % cols;
        let row = idx as i32 / cols;
        let cx = mx + col * (cw + gap);
        let cy = my + row * (ch + gap);

        // Card background
        fr(&mut img, cx, cy, cx + cw, cy + ch, Rgb([18, 18, 30]));
        br(&mut img, cx, cy, cx + cw, cy + ch, e.cat_color());
        fr(&mut img, cx, cy, cx + 8, cy + ch, e.cat_color());

        // Title: codepoint + category code
        draw_text(&mut img, &format!("U+{:04X}  {}", e.pack_id, e.cat_code()), cx + 18, cy + 10, 2, Rgb([240, 240, 255]));

        // Category name (short English label)
        draw_text(&mut img, e.cat_name(), cx + 18, cy + 38, 1, e.cat_color());

        // ====== MAIN VISUAL: Binary pattern (big) ======
        let bits: Vec<char> = e.rep_bits().chars().collect();
        let bfs = if e.bit_width <= 4 { 6 } else if e.bit_width <= 6 { 5 } else { 4 };
        let total_w = bits.len() as i32 * 6 * bfs;
        let sx = cx + (cw - total_w) / 2;
        let by = cy + 60;
        for (bi, &ch) in bits.iter().enumerate() {
            let bx = sx + bi as i32 * 6 * bfs;
            let color = if ch == '1' { e.cat_color() } else { Rgb([60, 60, 80]) };
            draw_char(&mut img, ch, bx, by, bfs, color);
        }

        // Orbit length label
        draw_text(&mut img, &format!("ORBIT LEN={}", e.orbit_len), cx + 18, by + 48, 1, Rgb([150, 150, 180]));

        // Orbit elements as mini binary
        let orbits = e.orbit_bits();
        let oby = by + 65;
        let mut obx = cx + 18i32;
        for (oi, ob) in orbits.iter().enumerate() {
            if oi >= 8 { break; }
            for (bi, ch) in ob.chars().enumerate() {
                let px = obx + bi as i32 * 4;
                let color = if ch == '1' { Rgb([200, 200, 220]) } else { Rgb([40, 40, 60]) };
                draw_char(&mut img, ch, px, oby, 1, color);
            }
            obx += ob.len() as i32 * 6 + 8;
        }

        // Footer: beat + representative
        let fy = cy + ch - 28;
        draw_text(&mut img, &format!("BEAT Z={}  REP={}", e.z_beat, e.representative), cx + 18, fy, 1, Rgb([120, 200, 150]));
    }

    // Bottom legend
    let ly = RH as i32 - 35;
    fr(&mut img, 0, ly, RW as i32 - 1, RH as i32 - 1, Rgb([20, 20, 35]));
    let cats = [(Rgb([74,144,217]),"AXIS 轴心容器"),(Rgb([80,184,108]),"LOOP 闭环态"),
                (Rgb([217,140,74]),"MIRROR 镜像纠缠"),(Rgb([160,100,200]),"EXT 扩展纠缠")];
    let mut lx = 30;
    for (c, label) in &cats { fr(&mut img, lx, ly + 8, lx + 15, ly + 22, *c); draw_text(&mut img, label, lx + 22, ly + 8, 1, Rgb([200, 200, 220])); lx += 300; }

    imageops::flip_vertical_in_place(&mut img);
    img.save(out_path).ok();
}

pub fn render_beat_animation(entries: &[FontEntry], out_path: &str) {
    if entries.is_empty() { return; }
    let es: Vec<RenderEntry> = entries.iter().map(RenderEntry::from).collect();
    let mut beats: Vec<u8> = entries.iter().map(|e| e.coord.z).collect();
    beats.sort_unstable(); beats.dedup();
    if beats.is_empty() { return; }

    let file = fs::File::create(out_path).expect("cannot create GIF");
    let mut encoder = GifEncoder::new(BufWriter::new(file));

    for &beat in &beats {
        let mut img = RgbImage::from_pixel(RW, RH, Rgb([8, 8, 16]));

        fr(&mut img, 0, 0, RW as i32 - 1, 50, Rgb([35, 15, 55]));
        draw_text(&mut img, &format!("BEAT {} / Z-WINDOW", beat), 20, 10, 3, Rgb([255, 230, 200]));
        draw_text(&mut img, &format!("{} active container(s)", es.iter().filter(|e| e.z_beat == beat).count()),
                  20, 38, 1, Rgb([180, 160, 180]));

        let n = es.len() as i32;
        let cols = if n <= 4 { 2 } else if n <= 9 { 3 } else if n <= 16 { 4 } else { 5 };
        let rows = (n + cols - 1) / cols;
        let mx = 30i32; let my = 75i32; let gap = 18i32;
        let cw = (RW as i32 - mx * 2 - gap * (cols - 1)) / cols;
        let ch = (RH as i32 - my - 55 - gap * (rows - 1)) / rows;

        for (idx, e) in es.iter().enumerate() {
            let col = idx as i32 % cols; let row = idx as i32 / cols;
            let cx = mx + col * (cw + gap); let cy = my + row * (ch + gap);
            let active = e.z_beat == beat;
            let cat_c = e.cat_color();
            let alpha = if active { 1.0 } else { 0.2 };
            let dim_c = Rgb([(cat_c[0] as f64 * alpha) as u8, (cat_c[1] as f64 * alpha) as u8, (cat_c[2] as f64 * alpha) as u8]);

            fr(&mut img, cx, cy, cx + cw, cy + ch, Rgb([15, 15, 25]));
            if active { fr(&mut img, cx, cy, cx + 6, cy + ch, dim_c); }
            br(&mut img, cx, cy, cx + cw, cy + ch, if active { dim_c } else { Rgb([35, 35, 50]) });

            draw_text(&mut img, &format!("U+{:04X} {}", e.pack_id, e.cat_code()),
                      cx + 12, cy + 8, 2, if active { Rgb([255,255,255]) } else { Rgb([100,100,130]) });

            if active {
                draw_text(&mut img, e.cat_name(), cx + 12, cy + 32, 1, dim_c);
            }

            // Big bits
            let bits: Vec<char> = e.rep_bits().chars().collect();
            let bfs = if e.bit_width <= 4 { 5 } else { 3 };
            let total_w = bits.len() as i32 * 6 * bfs;
            let sx = cx + (cw - total_w) / 2; let by = cy + 55;
            for (bi, &ch) in bits.iter().enumerate() {
                let bx = sx + bi as i32 * 6 * bfs;
                let c = if !active { Rgb([50,40,60]) } else if ch == '1' { dim_c } else { Rgb([40,40,60]) };
                draw_char(&mut img, ch, bx, by, bfs, c);
            }

            if active {
                let iy = by + 42;
                draw_text(&mut img, &format!("BEAT={} REP={}", e.z_beat, e.representative),
                          cx + 12, iy, 1, Rgb([140, 220, 160]));
            }
        }

        // Bottom beat bar
        let by = RH as i32 - 30;
        fr(&mut img, 20, by, RW as i32 - 20, by + 15, Rgb([25, 25, 35]));
        for b in &beats {
            let bx = 20 + (*b as i32 * (RW as i32 - 40) / 255).min(RW as i32 - 42);
            let bh: i32 = if *b == beat { 15 } else { 6 };
            fr(&mut img, bx, by + 15 - bh, bx + 4, by + 14, Rgb([100, 230, 255]));
        }

        imageops::flip_vertical_in_place(&mut img);
        let rgba = image::DynamicImage::ImageRgb8(img).to_rgba8();
        if encoder.encode_frame(Frame::new(rgba)).is_err() { break; }
    }
}
