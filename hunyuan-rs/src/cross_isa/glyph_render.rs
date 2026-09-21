// ══════════════════════════════════════════════════════════════════════
// glyph_render — 容器=字体=时间 三位一体渲染核心
//
// 供 font-renderer (CLI) 与 pipeline (流水线) 共用
// 纯 Rust · 仅依赖 image crate · 无外部字体工具
// ══════════════════════════════════════════════════════════════════════

use image::{Rgb, RgbImage, imageops};
use image::codecs::gif::GifEncoder;
use image::Frame;
use std::fs;
use std::io::BufWriter;

use super::container_font::FontEntry;

// ── 画布尺寸 ────────────────────────────────────────────────────────
pub const RENDER_W: u32 = 1600;
pub const RENDER_H: u32 = 1200;

// ── 3D 数学 ─────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub struct Vec3 { pub x: f64, pub y: f64, pub z: f64 }

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self { Self { x, y, z } }
    pub fn project_iso(&self) -> (f64, f64) {
        let angle = std::f64::consts::PI / 6.0;
        let sx = (self.x - self.y) * angle.cos();
        let sy = (self.x + self.y) * angle.sin() - self.z;
        (sx, sy)
    }
}

// ── 渲染视图 (从 FontEntry 派生) ─────────────────────────────────────

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
        }
    }
}

impl RenderEntry {
    pub fn category_color(&self) -> Rgb<u8> {
        match self.category {
            1 => Rgb([74, 144, 217]),   // AxialContainer - 蓝
            2 => Rgb([80, 184, 108]),   // ClosedLoop - 绿
            3 => Rgb([217, 140, 74]),   // MirrorEntangled - 橙
            4 => Rgb([160, 100, 200]),  // ExtendedEntangled - 紫
            _ => Rgb([136, 136, 136]),
        }
    }

    pub fn category_name(&self) -> &'static str {
        match self.category {
            1 => "AXIS",
            2 => "LOOP",
            3 => "MIRROR",
            4 => "EXT",
            _ => "????",
        }
    }

    pub fn phase_angle(&self, beat: u8) -> f64 {
        if self.orbit_len == 0 { return 0.0; }
        let step = (beat % self.orbit_len) as f64;
        step * std::f64::consts::TAU / self.orbit_len as f64
    }

    pub fn color_at_phase(&self, beat: u8) -> Rgb<u8> {
        let base = self.category_color();
        let phase = self.phase_angle(beat);
        let lum = 0.7 + 0.3 * phase.sin();
        Rgb([
            (base[0] as f64 * lum).min(255.0) as u8,
            (base[1] as f64 * lum).min(255.0) as u8,
            (base[2] as f64 * lum).min(255.0) as u8,
        ])
    }

    pub fn pos3d(&self) -> Vec3 {
        let cx = (self.pack_id % 8) as f64 * 160.0;
        let cy = (self.representative % 8) as f64 * 120.0;
        let cz = self.z_beat as f64 * 60.0;
        Vec3::new(cx, cy, cz)
    }
}

// ── 绘制工具 ─────────────────────────────────────────────────────────

#[inline]
fn px(x: i32, y: i32, img: &mut RgbImage, color: Rgb<u8>) {
    if x >= 0 && y >= 0 && (x as u32) < RENDER_W && (y as u32) < RENDER_H {
        img.put_pixel(x as u32, y as u32, color);
    }
}

fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, img: &mut RgbImage, color: Rgb<u8>) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;
    let mut x = x0;
    let mut y = y0;
    loop {
        px(x, y, img, color);
        if x == x1 && y == y1 { break; }
        let e2 = 2 * err;
        if e2 > -dy { err -= dy; x += sx; }
        if e2 < dx { err += dx; y += sy; }
    }
}

fn draw_circle(cx: i32, cy: i32, r: i32, img: &mut RgbImage, color: Rgb<u8>) {
    let mut x = 0i32;
    let mut y = r;
    let mut d = 3 - 2 * r;
    while x <= y {
        for &(dx, dy) in &[(x,y),(y,x),(-x,y),(y,-x),(-x,-y),(-y,-x),(x,-y),(-y,x)] {
            px(cx+dx, cy+dy, img, color);
        }
        if d < 0 { d += 4 * x + 6; } else { d += 4 * (x - y) + 10; y -= 1; }
        x += 1;
    }
}

fn draw_filled_rect(x0: i32, y0: i32, x1: i32, y1: i32, img: &mut RgbImage, color: Rgb<u8>) {
    for y in y0.max(0)..=y1.min(RENDER_H as i32 - 1) {
        for x in x0.max(0)..=x1.min(RENDER_W as i32 - 1) {
            img.put_pixel(x as u32, y as u32, color);
        }
    }
}

fn draw_rect_border(x0: i32, y0: i32, x1: i32, y1: i32, img: &mut RgbImage, color: Rgb<u8>) {
    draw_line(x0, y0, x1, y0, img, color);
    draw_line(x1, y0, x1, y1, img, color);
    draw_line(x1, y1, x0, y1, img, color);
    draw_line(x0, y1, x0, y0, img, color);
}

// ── 5x7 位图字体 ─────────────────────────────────────────────────────

const FONT_5X7: [[u8; 5]; 37] = [
    [0b11110,0b10001,0b10001,0b10001,0b10001], // 0
    [0b00100,0b01100,0b00100,0b00100,0b01110], // 1
    [0b11110,0b00001,0b11110,0b10000,0b11111], // 2
    [0b11110,0b00001,0b11110,0b00001,0b11110], // 3
    [0b10001,0b10001,0b11111,0b00001,0b00001], // 4
    [0b11111,0b10000,0b11110,0b00001,0b11110], // 5
    [0b11110,0b10000,0b11110,0b10001,0b11110], // 6
    [0b11111,0b00001,0b00010,0b00100,0b00100], // 7
    [0b11110,0b10001,0b11110,0b10001,0b11110], // 8
    [0b11110,0b10001,0b11111,0b00001,0b11110], // 9
    [0b01110,0b10001,0b11111,0b10001,0b10001], // A
    [0b11110,0b10001,0b11110,0b10001,0b11110], // B
    [0b01111,0b10000,0b10000,0b10000,0b01111], // C
    [0b11110,0b10001,0b10001,0b10001,0b11110], // D
    [0b11111,0b10000,0b11110,0b10000,0b11111], // E
    [0b11111,0b10000,0b11110,0b10000,0b10000], // F
    [0b01111,0b10000,0b10111,0b10001,0b01111], // G
    [0b10001,0b10001,0b11111,0b10001,0b10001], // H
    [0b01110,0b00100,0b00100,0b00100,0b01110], // I
    [0b00111,0b00010,0b00010,0b10010,0b01100], // J
    [0b10001,0b10010,0b11100,0b10010,0b10001], // K
    [0b10000,0b10000,0b10000,0b10000,0b11111], // L
    [0b10001,0b11011,0b10101,0b10001,0b10001], // M
    [0b10001,0b11001,0b10101,0b10011,0b10001], // N
    [0b01110,0b10001,0b10001,0b10001,0b01110], // O
    [0b11110,0b10001,0b11110,0b10000,0b10000], // P
    [0b01110,0b10001,0b10001,0b10101,0b01111], // Q
    [0b11110,0b10001,0b11110,0b10010,0b10001], // R
    [0b01111,0b10000,0b01110,0b00001,0b11110], // S
    [0b11111,0b00100,0b00100,0b00100,0b00100], // T
    [0b10001,0b10001,0b10001,0b10001,0b01110], // U
    [0b10001,0b10001,0b10001,0b01010,0b00100], // V
    [0b10001,0b10001,0b10101,0b11011,0b10001], // W
    [0b10001,0b01010,0b00100,0b01010,0b10001], // X
    [0b10001,0b01010,0b00100,0b00100,0b00100], // Y
    [0b11111,0b00010,0b00100,0b01000,0b11111], // Z
    [0b00000,0b00000,0b00000,0b00000,0b00000], // space
];

fn char_index(c: char) -> usize {
    match c {
        '0'..='9' => c as usize - '0' as usize,
        'A'..='Z' => 10 + (c as usize - 'A' as usize),
        'a'..='z' => 10 + (c as usize - 'a' as usize),
        _ => 36, // space fallback
    }
}

fn draw_char_5x7(c: char, x: i32, y: i32, scale: i32, img: &mut RgbImage, color: Rgb<u8>) {
    let idx = char_index(c);
    if idx >= FONT_5X7.len() { return; }
    let pat = &FONT_5X7[idx];
    for row in 0..pat.len() {
        for col in 0..5 {
            if (pat[row] >> (4 - col)) & 1 == 1 {
                draw_filled_rect(
                    x + col as i32 * scale, y + row as i32 * scale,
                    x + (col + 1) as i32 * scale - 1, y + (row + 1) as i32 * scale - 1,
                    img, color,
                );
            }
        }
    }
}

pub fn draw_string_5x7(text: &str, x: i32, y: i32, scale: i32, img: &mut RgbImage, color: Rgb<u8>) {
    let mut cx = x;
    for ch in text.chars() {
        draw_char_5x7(ch, cx, y, scale, img, color);
        cx += 6 * scale;
    }
}

fn draw_number_5x7(mut n: u64, x: i32, y: i32, scale: i32, img: &mut RgbImage, color: Rgb<u8>) {
    if n == 0 { draw_string_5x7("0", x, y, scale, img, color); return; }
    let mut digits = Vec::new();
    while n > 0 { digits.push((n % 10) as u8); n /= 10; }
    digits.reverse();
    let mut cx = x;
    for d in digits {
        draw_string_5x7(&d.to_string(), cx, y, scale, img, color);
        cx += 6 * scale;
    }
}

// ══════════════════════════════════════════════════════════════════════
// 公开 API
// ══════════════════════════════════════════════════════════════════════

/// PNG 静态全容器快照
pub fn render_snapshot(font_entries: &[FontEntry], out_path: &str) {
    let entries: Vec<RenderEntry> = font_entries.iter().map(RenderEntry::from).collect();
    let mut img = RgbImage::from_pixel(RENDER_W, RENDER_H, Rgb([15, 15, 25]));

    // Header
    draw_filled_rect(0, 0, RENDER_W as i32 - 1, 70, &mut img, Rgb([25, 25, 45]));
    draw_string_5x7("MQ-OS Container Font  Phase-Space Snapshot", 20, 15, 2, &mut img, Rgb([220, 220, 255]));
    draw_string_5x7(&format!("{} containers  |  Z=beat  |  orbit=phase  |  color=category + phase",
                              entries.len()), 20, 42, 1, &mut img, Rgb([140, 140, 180]));

    let threshold = 5.0_f64;

    // Relationships
    for i in 0..entries.len() {
        for j in (i+1)..entries.len() {
            let pi = entries[i].pos3d();
            let pj = entries[j].pos3d();
            let (sx1, sy1) = pi.project_iso();
            let (sx2, sy2) = pj.project_iso();
            let sx1 = (sx1 + 800.0) as i32;
            let sy1 = (sy1 + 450.0) as i32;
            let sx2 = (sx2 + 800.0) as i32;
            let sy2 = (sy2 + 450.0) as i32;
            // Simple distance metric based on category + orbit + z differences
            let cat_diff = if entries[i].category == entries[j].category { 0.0 } else { 8.0 };
            let orb_diff = if entries[i].orbit_len == entries[j].orbit_len { 0.0 } else { 3.0 };
            let z_diff = ((entries[i].z_beat as i32 - entries[j].z_beat as i32).abs()) as f64;
            let d = cat_diff + orb_diff + z_diff * 0.5;
            if d < threshold {
                let alpha = ((threshold - d) / threshold * 200.0) as u8;
                draw_line(sx1, sy1, sx2, sy2, &mut img, Rgb([alpha / 3, alpha / 2, alpha]));
            }
        }
    }

    // Containers
    for c in &entries {
        let pos3 = c.pos3d();
        let (sx, sy) = pos3.project_iso();
        let sx = (sx + 800.0) as i32;
        let sy = (sy + 450.0) as i32;
        let color = c.color_at_phase(0);
        let box_size = 20 + c.orbit_len as i32 * 6;
        draw_filled_rect(sx - box_size, sy - box_size, sx + box_size, sy + box_size, &mut img, color);
        draw_rect_border(sx - box_size, sy - box_size, sx + box_size, sy + box_size, &mut img, Rgb([200, 200, 220]));

        let ring_r = 8 + c.orbit_len as i32 * 5;
        draw_circle(sx, sy, ring_r, &mut img, Rgb([80, 80, 100]));

        draw_string_5x7(c.category_name(), sx - box_size, sy + box_size + 4, 1, &mut img, color);
        draw_number_5x7(c.pack_id, sx - box_size, sy - box_size - 9, 1, &mut img, Rgb([180, 180, 200]));

        let bar_w = (c.z_beat as i32).min(30);
        draw_filled_rect(sx - 15, sy - box_size - 5, sx - 15 + bar_w, sy - box_size - 3, &mut img, Rgb([100, 200, 100]));
    }

    // Legend
    let ly = RENDER_H as i32 - 120;
    draw_filled_rect(20, ly, RENDER_W as i32 - 20, ly + 100, &mut img, Rgb([25, 25, 40]));
    draw_rect_border(20, ly, RENDER_W as i32 - 20, ly + 100, &mut img, Rgb([60, 60, 80]));
    draw_string_5x7("LEGEND", 30, ly + 8, 1, &mut img, Rgb([200, 200, 240]));

    let items = [
        (Rgb([74, 144, 217]), "AXIS  AxialContainer (c=1)"),
        (Rgb([80, 184, 108]), "LOOP  ClosedLoop (c=2)"),
        (Rgb([217, 140, 74]), "MIRROR MirrorEntangled (c=3)"),
        (Rgb([160, 100, 200]), "EXT   ExtendedEntangled (c=4)"),
    ];
    for (i, (color, label)) in items.iter().enumerate() {
        let y = ly + 28 + i as i32 * 16;
        draw_filled_rect(30, y + 2, 44, y + 10, &mut img, *color);
        draw_string_5x7(label, 50, y, 1, &mut img, Rgb([160, 160, 180]));
    }

    imageops::flip_vertical_in_place(&mut img);
    img.save(out_path).expect("cannot write PNG");
}

/// GIF 逐拍动画
pub fn render_beat_animation(font_entries: &[FontEntry], out_path: &str) {
    if font_entries.is_empty() { return; }
    let entries: Vec<RenderEntry> = font_entries.iter().map(RenderEntry::from).collect();

    let mut beats: Vec<u8> = entries.iter().map(|e| e.z_beat).collect();
    beats.sort_unstable();
    beats.dedup();
    if beats.is_empty() { return; }

    let file = fs::File::create(out_path).expect("cannot create GIF file");
    let writer = BufWriter::new(file);
    let mut encoder = GifEncoder::new(writer);

    for &beat in &beats {
        let mut img = RgbImage::from_pixel(RENDER_W, RENDER_H, Rgb([12, 12, 22]));

        // Header
        draw_filled_rect(0, 0, RENDER_W as i32 - 1, 50, &mut img, Rgb([30, 20, 50]));
        draw_string_5x7(&format!("BEAT {} / Z-Window", beat), 20, 10, 2, &mut img, Rgb([255, 220, 180]));
        let max_orbit = entries.iter().filter(|c| c.z_beat == beat).map(|c| c.orbit_len).max().unwrap_or(0);
        draw_string_5x7(&format!("Active: orbit_len={}", max_orbit), 20, 32, 1, &mut img, Rgb([160, 140, 180]));

        // Non-active (dimmed)
        for c in &entries {
            if c.z_beat == beat { continue; }
            let pos3 = c.pos3d();
            let (sx, sy) = pos3.project_iso();
            let sx = (sx + 800.0) as i32;
            let sy = (sy + 450.0) as i32;
            let box_size = 15 + c.orbit_len as i32 * 3;
            draw_filled_rect(sx - box_size, sy - box_size, sx + box_size, sy + box_size, &mut img, Rgb([20, 20, 30]));
        }

        // Active (bright)
        for c in entries.iter().filter(|c| c.z_beat == beat) {
            let pos3 = c.pos3d();
            let (sx, sy) = pos3.project_iso();
            let sx = (sx + 800.0) as i32;
            let sy = (sy + 450.0) as i32;
            let color = c.color_at_phase(beat);
            let box_size = 25 + c.orbit_len as i32 * 8;

            let bright = Rgb([
                (color[0] as u16 + 60).min(255) as u8,
                (color[1] as u16 + 60).min(255) as u8,
                (color[2] as u16 + 60).min(255) as u8,
            ]);
            draw_filled_rect(sx - box_size, sy - box_size, sx + box_size, sy + box_size, &mut img, bright);
            draw_rect_border(sx - box_size, sy - box_size, sx + box_size, sy + box_size, &mut img, Rgb([255, 255, 255]));

            let phase = c.phase_angle(beat);
            let ring_r = 12 + c.orbit_len as i32 * 5;
            draw_circle(sx, sy, ring_r, &mut img, Rgb([120, 120, 150]));
            let px = sx + (phase.cos() * ring_r as f64) as i32;
            let py = sy + (phase.sin() * ring_r as f64) as i32;
            draw_filled_rect(px - 3, py - 3, px + 3, py + 3, &mut img, Rgb([255, 255, 100]));

            draw_string_5x7(c.category_name(), sx - box_size, sy + box_size + 5, 1, &mut img, bright);
            draw_number_5x7(c.pack_id, sx - box_size, sy - box_size - 10, 1, &mut img, Rgb([220, 220, 255]));
        }

        // Beat indicator strip
        let bar_total = RENDER_W as i32 - 40;
        let bar_step = bar_total / 256;
        if bar_step > 0 {
            draw_filled_rect(20, RENDER_H as i32 - 30, RENDER_W as i32 - 20, RENDER_H as i32 - 10, &mut img, Rgb([30, 30, 40]));
            for b in &beats {
                let bx = 20 + (*b as i32) * bar_step;
                let bh = if *b == beat { 18 } else { 8 };
                draw_filled_rect(bx, RENDER_H as i32 - 10 - bh, bx + bar_step.max(1) - 1, RENDER_H as i32 - 11, &mut img, Rgb([100, 200, 255]));
            }
        }

        let rgba_img = image::DynamicImage::ImageRgb8(img).to_rgba8();
        let frame = Frame::new(rgba_img);
        if let Err(e) = encoder.encode_frame(frame) {
            eprintln!("[!] GIF encode error: {}", e);
            break;
        }
    }
}
