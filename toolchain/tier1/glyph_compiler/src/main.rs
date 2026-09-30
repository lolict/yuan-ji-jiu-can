// 囡囝共生 · 容器编译器 v0.3
// 五维 → 田字格容器 → BMP（零依赖，纯 math）
// 每格 4 分域：声母(左上) · 韵母(右上) · 声调(左下) · 微位(右下特征点)
// 符号 = 几何形态，绝不出现 Latin 字母

const W: i32 = 1920;
const H: i32 = 1080;
const BG: u32 = 0x141414;

// ─── 几何元语 ───────────────────────────────────────────

// Canvas 封装：用 (data, width, height) 三代替代全局 W/H 常量，支持不同尺寸的画布

struct Cv { d: Vec<u32>, w: i32, h: i32 }

fn put(c: &mut Cv, x: i32, y: i32, v: u32) {
    if x >= 0 && x < c.w && y >= 0 && y < c.h {
        c.d[(y as usize) * (c.w as usize) + (x as usize)] = v;
    }
}

fn rect(c: &mut Cv, x0: i32, y0: i32, x1: i32, y1: i32, col: u32) {
    for y in y0..=y1 { for x in x0..=x1 { put(c, x, y, col); } }
}

fn line(c:  &mut Cv, x0: i32, y0: i32, x1: i32, y1: i32, col: u32) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut e = dx + dy;
    let mut x = x0;
    let mut y = y0;
    loop {
        put(c, x, y, col);
        if x == x1 && y == y1 { break; }
        let e2 = 2 * e;
        if e2 >= dy { e += dy; x += sx; }
        if e2 <= dx { e += dx; y += sy; }
    }
}

fn circle(c:  &mut Cv, cx: i32, cy: i32, r: i32, col: u32) {
    let mut x = r;
    let mut y = 0;
    let mut e = 1 - r;
    loop {
        put(c, cx + x, cy + y, col);
        put(c, cx - x, cy + y, col);
        put(c, cx + x, cy - y, col);
        put(c, cx - x, cy - y, col);
        put(c, cx + y, cy + x, col);
        put(c, cx - y, cy + x, col);
        put(c, cx + y, cy - x, col);
        put(c, cx - y, cy - x, col);
        y += 1;
        if e < 0 { e += 2 * y + 1; } else { x -= 1; e += 2 * (y - x) + 1; }
        if x < y { break; }
    }
}

// 菱形（对角线交叉）
fn diamond(c:  &mut Cv, cx: i32, cy: i32, r: i32, col: u32) {
    line(c, cx - r, cy, cx, cy - r, col);
    line(c, cx, cy - r, cx + r, cy, col);
    line(c, cx + r, cy, cx, cy + r, col);
    line(c, cx, cy + r, cx - r, cy, col);
}

// 三角
fn triangle(c:  &mut Cv, cx: i32, cy: i32, s: i32, col: u32) {
    line(c, cx, cy - s, cx + s, cy + s, col);
    line(c, cx + s, cy + s, cx - s, cy + s, col);
    line(c, cx - s, cy + s, cx, cy - s, col);
}

// ─── 声母编码（48 → 几何） ───────────────────────────────
// 0–3:  唇音 b p m f     → 竖条 1–4
// 4–7:  舌尖 d t n l     → 横条 1–4
// 8–11: 舌根 g k h w     → 点 1–4
// 12–15:舌面 j q x y     → 圆形
// 16–19:平舌 z c s r     → 菱形
// 20–23:扩展 ȥ ç ş ŋ    → 三角
// 24–47:大写 = 小写 + 右侧竖线（区分大小写）

fn draw_shengmu_big(c: &mut Cv, sx: i32, sy: i32, size: i32, sid: u32, col: u32) {
    // 大尺寸声母图标
    draw_shengmu(c, sx, sy, size, sid, col);
    // 加外框让它更显眼
    line(c, sx, sy, sx + size, sy, col);
    line(c, sx + size, sy, sx + size, sy + size, col);
    line(c, sx + size, sy + size, sx, sy + size, col);
    line(c, sx, sy + size, sx, sy, col);
}

// 实心竖条（2px 粗）——快 10 倍
fn solid_vbar(c: &mut Cv, cx: i32, cy: i32, h: i32, w: i32, col: u32) {
    rect(c, cx - w/2, cy - h/2, cx + w/2, cy + h/2, col);
}
fn solid_hbar(c: &mut Cv, cx: i32, cy: i32, w: i32, h: i32, col: u32) {
    rect(c, cx - w/2, cy - h/2, cx + w/2, cy + h/2, col);
}
// 实心菱形
fn solid_diamond(c: &mut Cv, cx: i32, cy: i32, r: i32, col: u32) {
    for dy in -r..=r {
        let w = r - dy.abs();
        rect(c, cx - w, cy + dy, cx + w, cy + dy, col);
    }
}
// 实心三角（正）
fn solid_triangle(c: &mut Cv, cx: i32, cy: i32, s: i32, col: u32) {
    for dy in 0..=s {
        let w = dy;
        rect(c, cx - w, cy - s + dy, cx + w, cy - s + dy, col);
    }
}
// 实心圆（中点算法填充）
fn solid_circle(c: &mut Cv, cx: i32, cy: i32, r: i32, col: u32) {
    for dy in -r..=r {
        let w = ((r * r - dy * dy) as f64).sqrt() as i32;
        rect(c, cx - w, cy + dy, cx + w, cy + dy, col);
    }
}

fn draw_shengmu(c: &mut Cv, sx: i32, sy: i32, size: i32, sid: u32, col: u32) {
    let base = sid % 24;
    let is_upper = sid >= 24;
    let cx = sx + size / 2;
    let cy = sy + size / 2;
    let bar_thick = (size / 6).max(2);   // 笔画粗细

    match base {
        0..=3 => {
            // 竖条 1–4 根
            let n = (base + 1) as i32;
            let spacing = size / (n + 1);
            for i in 1..=n {
                let x = sx + i * spacing;
                solid_vbar(c, x, cy, size - 4, bar_thick, col);
            }
        }
        4..=7 => {
            // 横条 1–4 根
            let n = (base - 3) as i32;
            let spacing = size / (n + 1);
            for i in 1..=n {
                let y = sy + i * spacing;
                solid_hbar(c, cx, y, size - 4, bar_thick, col);
            }
        }
        8..=11 => {
            // 散点 2×2 方块 1–4 个
            let n = (base - 7) as i32;
            let d = bar_thick + 2;
            for i in 0..n {
                let px = cx + ((i % 2) * 2 - 1) * d;
                let py = cy + (((i / 2) % 2) * 2 - 1) * d;
                rect(c, px - 1, py - 1, px + 1, py + 1, col);
            }
        }
        12..=15 => {
            // 实心圆
            let r = (size / 4) + (base - 12) as i32;
            solid_circle(c, cx, cy, r, col);
        }
        16..=19 => {
            // 实心菱形
            let r = (size / 4) + (base - 16) as i32 * 2;
            solid_diamond(c, cx, cy, r, col);
        }
        20..=23 => {
            // 实心三角（4种旋转）
            let s = size / 3;
            let k = (base - 20) as i32;
            if k == 0 { solid_triangle(c, cx, cy + s/3, s, col); }
            else if k == 1 {
                // 倒三角
                for dy in 0..=s {
                    let w = s - dy;
                    rect(c, cx - w, cy + s - dy, cx + w, cy + s - dy, col);
                }
            }
            else if k == 2 {
                // 左三角
                for dx in 0..=s {
                    let h = dx;
                    rect(c, cx - s + dx, cy - h, cx - s + dx, cy + h, col);
                }
            }
            else {
                for dx in 0..=s {
                    let h = s - dx;
                    rect(c, cx + s - dx, cy - h, cx + s - dx, cy + h, col);
                }
            }
        }
        _ => {}
    }

    // 大写标记：右上角小方块
    if is_upper {
        let m = (size / 5).max(2);
        rect(c, sx + size - m - 1, sy + 1, sx + size - 1, sy + m + 1, col);
    }
}

// ─── 韵母编码（66 → 几何） ───────────────────────────────
// 0–5:   基础复韵 → 弧线（不同角度）
// 6–11:  合口     → 双弧
// 12–17: 卷舌     → 折线
// 18–27: 前鼻音   → 方框 + 内点数
// 28–35: 后鼻音   → 大方框
// 36–45: ia/ua组  → 梯形/领结
// 46–65: 剩余     → 复合形状

fn draw_yunmu(c:  &mut Cv, sx: i32, sy: i32, size: u32, yid: u32, col: u32) {
    let cx = sx + size as i32 / 2;
    let cy = sy + size as i32 / 2;
    let r = size as i32 / 3;
    let group = yid / 6;     // 0–10
    let sub = yid % 6;

    match group {
        0 => { // 实心弧带（6 种角度 → 粗条带）
            let angle = (sub * 60) as f64 * std::f64::consts::PI / 180.0;
            let inner = (r - 3).max(1);
            for dy in -inner..=inner {
                let w = ((inner*inner - dy*dy) as f64).sqrt() as i32;
                let rot_x = (angle.cos() * w as f64) as i32;
                let rot_y = (angle.sin() * w as f64) as i32;
                line(c, cx - rot_x, cy - rot_y, cx + rot_x, cy + rot_y, col);
            }
        }
        1 => { // 双层弧带（半径不同）
            for layer in 0..2 {
                let rr = r - layer * 4;
                if rr < 2 { break; }
                let angle = (sub * 60 + layer as u32 * 180) as f64 * std::f64::consts::PI / 180.0;
                for dy in -rr..=rr {
                    let w = ((rr*rr - dy*dy) as f64).sqrt() as i32;
                    let rot_x = (angle.cos() * w as f64) as i32;
                    let rot_y = (angle.sin() * w as f64) as i32;
                    line(c, cx - rot_x, cy - rot_y, cx + rot_x, cy + rot_y, col);
                }
            }
        }
        2 => { // V 形实心（折线 → V 形）
            let s = r;
            let dir = if sub % 2 == 0 { 1i32 } else { -1i32 };
            for i in 0..=s {
                let half_w = i;
                let top = cy - dir * (s - i);
                rect(c, cx - half_w, top, cx + half_w, top, col);
            }
        }
        3 => { // 方框填充，内点数不同
            let m = (3 + sub as i32).min(size as i32 - 2);
            rect(c, sx + m, sy + m, sx + size as i32 - m, sy + size as i32 - m, col);
            // 内部小实心
            if sub < 3 {
                let ir = (sub + 1) as i32 * 2;
                solid_circle(c, cx, cy, ir, col);
            } else {
                let id = (sub - 2) as i32 * 2;
                solid_diamond(c, cx, cy, id, col);
            }
        }
        4 => { // 大方框（填充）
            rect(c, sx + 2, sy + 2, sx + size as i32 - 2, sy + size as i32 - 2, col);
            // 中心留空
            if sub < 3 {
                let inner = (sub + 1) as i32 * 3;
                rect(c, cx - inner, cy - inner, cx + inner, cy + inner, BG);
            }
        }
        5 => { // 梯形实心
            let t = sub as i32 * 2;
            for dy in 0..=r {
                let half_w = (r - t) + t * dy / r;
                rect(c, cx - half_w, sy + 2 + dy, cx + half_w, sy + 2 + dy, col);
            }
        }
        6 => { // 领结（两个三角拼接）
            // 左三角（实心）
            for dx in 0..=r {
                let h = r - dx;
                rect(c, sx + 2 + dx, cy - h, sx + 2 + dx, cy + h, col);
            }
            // 右三角
            for dx in 0..=r {
                let h = dx;
                rect(c, cx + dx, cy - h, cx + dx, cy + h, col);
            }
        }
        _ => { // 其余：实心圆 / 实心菱（不同大小）
            if sub < 3 {
                solid_circle(c, cx, cy, (r - sub as i32).max(2), col);
            } else {
                solid_diamond(c, cx, cy, (r - sub as i32 + 1).max(2), col);
            }
        }
    }
}

fn draw_arc(c:  &mut Cv, cx: i32, cy: i32, r: i32, degree: u32, col: u32) {
    let start = degree as f64 * std::f64::consts::PI / 180.0;
    let end = start + std::f64::consts::PI;
    let steps = 20;
    let mut prev_x = -1i32;
    let mut prev_y = -1i32;
    for i in 0..=steps {
        let t = start + (end - start) * (i as f64) / (steps as f64);
        let x = cx + (t.cos() * r as f64) as i32;
        let y = cy + (t.sin() * r as f64) as i32;
        if i > 0 {
            line(c, prev_x, prev_y, x, y, col);
        }
        prev_x = x;
        prev_y = y;
    }
}

// ─── 声调编码（60 → 几何） ───────────────────────────────
// 60 = 12 调型 × 5 调值区域 → 用 12 种方向和 5 种位置

fn draw_tone(c:  &mut Cv, sx: i32, sy: i32, size: i32, tid: u32, col: u32) {
    let cx = sx + size / 2;
    let cy = sy + size / 2;
    let k = (tid % 12) as i32;    // 12 种调型
    let v = (tid / 12) as i32;    // 5 个区域

    let h = size / 4;
    let thick = (h / 2).max(1);
    let px = cx + (v - 2) * h / 2;  // 位置偏移

    match k {
        0 => { solid_hbar(c, px, cy, h * 2, thick, col); }  // 平横条
        1 => { // 升条（左下→右上粗条）
            for i in 0..=h {
                let off = i;
                let x = px - h + off;
                let y = cy + h - off;
                solid_vbar(c, x, y, thick, thick, col);
            }
        }
        2 => { // V 形实心
            for i in 0..=h {
                rect(c, px - i, cy - h + i, px + i, cy - h + i, col);
            }
        }
        3 => { // 降条（左上→右下粗条）
            for i in 0..=h {
                let x = px - h + i;
                let y = cy - h + i;
                solid_vbar(c, x, y, thick, thick, col);
            }
        }
        4 => { // 断升（两段）
            solid_hbar(c, px - h/2, cy - h/2, h, thick, col);
            solid_hbar(c, px + h/2, cy + h/2, h, thick, col);
        }
        5 => { solid_vbar(c, px, cy, h * 2, thick, col); }  // 竖条
        6 => { solid_circle(c, px, cy, h, col); }
        7 => { solid_diamond(c, px, cy, h, col); }
        8 => { solid_triangle(c, px, cy + h/3, h, col); }
        9 => { // 双横条
            solid_hbar(c, px, cy - h/2, h * 2, thick, col);
            solid_hbar(c, px, cy + h/2, h * 2, thick, col);
        }
        10 => { // 十字（实心）
            solid_hbar(c, px, cy, h * 2, thick, col);
            solid_vbar(c, px, cy, h * 2, thick, col);
        }
        11 => { // 点阵（3×3）
            for dy in -1..=1 { for dx in -1..=1 {
                let d = h / 2;
                rect(c, px + dx * d - 1, cy + dy * d - 1, px + dx * d + 1, cy + dy * d + 1, col);
            }}
        }
        _ => {}
    }
}

fn dot_lattice(c:  &mut Cv, cx: i32, cy: i32, n: i32, col: u32) {
    for dy in -1..=1 { for dx in -1..=1 { put(c, cx + dx * n / 2, cy + dy * n / 2, col); } }
}

// ─── 微位置编码（10×10 网格点） ──────────────────────────

fn draw_wz(c:  &mut Cv, sx: i32, sy: i32, size: i32, mx: u32, my: u32, col: u32) {
    let px = sx + (mx as i32) * size / 10 + size / 20;
    let py = sy + (my as i32) * size / 10 + size / 20;
    put(c, px, py, col);
    put(c, px + 1, py, col);
    put(c, px, py + 1, col);
    put(c, px + 1, py + 1, col);
}

// ─── 粗线（2像素）用于容器边框 ─────────────────────────────

fn thick_line(c: &mut Cv, x0: i32, y0: i32, x1: i32, y1: i32, col: u32, w: i32) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    if dx >= dy {
        // 水平走向：沿 Y 偏移画 w 条平行线
        for o in -w/2..=w/2 {
            line(c, x0, y0 + o, x1, y1 + o, col);
        }
    } else {
        for o in -w/2..=w/2 {
            line(c, x0 + o, y0, x1 + o, y1, col);
        }
    }
}

// ─── 田字格容器 ───────────────────────────────────────────

const C_ID: [(u8,u8,u8); 42] = [
    (220,80,80),(220,120,60),(220,180,50),(200,220,60),(160,220,70),(120,210,80),
    (80,200,120),(60,190,160),(60,170,200),(70,140,220),(90,110,220),(120,90,220),
    (160,80,210),(200,80,200),(220,90,170),(220,110,140),(210,140,120),(200,170,110),
    (180,200,120),(150,210,140),(120,220,170),(90,210,200),(70,180,220),(80,150,220),
    (100,120,220),(130,100,210),(170,90,200),(210,100,180),(220,130,150),(210,160,130),
    (190,190,130),(160,210,150),(130,220,180),(100,210,210),(80,180,220),(90,160,210),
    (110,140,200),(140,130,190),(180,130,180),(210,140,170),(220,160,150),(200,180,150),
];

fn draw_container_mono(canvas:  &mut Cv, ox: i32, oy: i32, cw: i32, ch: i32,
                        sid: u32, yid: u32, tid: u32, mx: u32, my: u32, fg: u32) {
    // 田字格外框 2px —— 用前景色
    thick_line(canvas, ox, oy, ox + cw, oy, fg, 2);
    thick_line(canvas, ox + cw, oy, ox + cw, oy + ch, fg, 2);
    thick_line(canvas, ox + cw, oy + ch, ox, oy + ch, fg, 2);
    thick_line(canvas, ox, oy + ch, ox, oy, fg, 2);
    // 十字分割 1px
    line(canvas, ox + cw / 2, oy, ox + cw / 2, oy + ch, fg);
    line(canvas, ox, oy + ch / 2, ox + cw, oy + ch / 2, fg);

    let half_w = cw / 2;
    let half_h = ch / 2;

    // 左上：声母
    draw_shengmu(canvas, ox + 2, oy + 2, half_w - 4, sid, fg);
    // 右上：韵母
    draw_yunmu(canvas, ox + half_w + 1, oy + 2, (half_w - 4) as u32, yid, fg);
    // 左下：声调
    draw_tone(canvas, ox + 2, oy + half_h + 1, half_w - 4, tid, fg);
    // 右下：微位置（白色 2×2 点，但在 mono 下用 RGB(255,0,0) 做对比）
    draw_wz(canvas, ox + half_w + 1, oy + half_h + 1, half_w - 4, mx, my, 0x0000FF);
}

fn draw_container(canvas:  &mut Cv, ox: i32, oy: i32, cw: i32, ch: i32,
                   sid: u32, yid: u32, tid: u32, mx: u32, my: u32) {
    let base = sid * 3960 + tid * 66 + yid;
    let cid = (base % 42) as usize;
    let (cr, cg, cb) = C_ID[cid];
    let col = cr as u32 | ((cg as u32) << 8) | ((cb as u32) << 16);

    // 田字格外框 2px
    thick_line(canvas, ox, oy, ox + cw, oy, col, 2);
    thick_line(canvas, ox + cw, oy, ox + cw, oy + ch, col, 2);
    thick_line(canvas, ox + cw, oy + ch, ox, oy + ch, col, 2);
    thick_line(canvas, ox, oy + ch, ox, oy, col, 2);
    // 十字分割 1px
    line(canvas, ox + cw / 2, oy, ox + cw / 2, oy + ch, col);
    line(canvas, ox, oy + ch / 2, ox + cw, oy + ch / 2, col);

    let half_w = cw / 2;
    let half_h = ch / 2;

    // 左上：声母
    draw_shengmu(canvas, ox + 2, oy + 2, half_w - 4, sid, col);
    // 右上：韵母
    draw_yunmu(canvas, ox + half_w + 1, oy + 2, (half_w - 4) as u32, yid, col);
    // 左下：声调
    draw_tone(canvas, ox + 2, oy + half_h + 1, half_w - 4, tid, col);
    // 右下：微位置（白色 2×2 点）
    draw_wz(canvas, ox + half_w + 1, oy + half_h + 1, half_w - 4, mx, my, 0xFFFFFF);
}

// ─── BMP 输出 ───────────────────────────────────────────

fn write_bmp_canvas(c: &Cv, out: &str) {
    let w = c.w as usize;
    let h = c.h as usize;
    let rs = (w * 3 + 3) & !3;
    let mut buf: Vec<u8> = Vec::with_capacity(54 + rs * h);

    buf.extend_from_slice(b"BM");
    let fsize: u32 = 54 + (rs as u32) * (h as u32);
    buf.extend_from_slice(&fsize.to_le_bytes());
    buf.extend_from_slice(&[0u8; 4]);
    buf.extend_from_slice(&54u32.to_le_bytes());
    buf.extend_from_slice(&40u32.to_le_bytes());
    buf.extend_from_slice(&(w as i32).to_le_bytes());
    buf.extend_from_slice(&(h as i32).to_le_bytes());
    buf.extend_from_slice(&1u16.to_le_bytes());
    buf.extend_from_slice(&24u16.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&(rs as u32 * h as u32).to_le_bytes());
    buf.extend_from_slice(&2835i32.to_le_bytes());
    buf.extend_from_slice(&2835i32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());

    for yy in (0..h as i32).rev() {
        for x in 0..w as i32 {
            let c2 = c.d[(yy as usize * w + x as usize)];
            buf.push((c2 & 0xFF) as u8);
            buf.push(((c2 >> 8) & 0xFF) as u8);
            buf.push(((c2 >> 16) & 0xFF) as u8);
        }
        for _ in 0..(rs - w * 3) { buf.push(0); }
    }

    std::fs::write(out, &buf).ok();
}

// ─── Main ───────────────────────────────────────────────

fn draw_label(canvas:  &mut Cv, x: i32, y: i32, s: u32, yy: u32, t: u32, mx: u32, my: u32) {
    // 用简单象形数字（矩形柱状）标注参数
    let col = 0x888888;
    let cx = x + 20;

    // 声母序号 — 短竖条堆叠
    for i in 0..(s % 8) as i32 {
        line(canvas, cx, y + i * 6, cx + 30, y + i * 6, col);
    }
    // 韵母序号 — 点列
    for i in 0..(yy % 8) as i32 {
        put(canvas, cx + 35 + i * 4, y + 10, col);
        put(canvas, cx + 36 + i * 4, y + 10, col);
    }
    // 声调序号 — 横条
    for i in 0..(t % 8) as i32 {
        line(canvas, cx + 65, y + i * 5 + 2, cx + 100, y + i * 5 + 2, col);
    }
}

fn draw_row(sid: u32, out: &str) {
    // 行视图：每容器 120×120，真正能看清
    // X轴=韵母取前12列，Y轴=声调取前10行 → 120音节样品
    let cell = 90i32;
    let cols = 12i32;
    let rows = 10i32;
    let label_w = 120i32;     // 左侧声母标注
    let title_h = 50i32;      // 顶部韵母标注
    let margin = 8i32;
    let pix_w = cell;
    let pix_h = cell;
    let cw = cell - 4;         // 容器实际大小
    let ch = cell - 4;
    let canvas_w = label_w + cols * pix_w + margin * 2;
    let canvas_h = title_h + rows * pix_h + margin * 2;

    let mut cv = Cv { d: vec![BG; (canvas_w * canvas_h) as usize], w: canvas_w, h: canvas_h };

    // 声母大图标（左侧）
    draw_shengmu_big(&mut cv, 10, title_h + rows * pix_h / 2 - 45, 90, sid, 0xCCCCCC);

    // 韵母列标注条（顶部 0–11 号）
    for i in 0..cols {
        let px = label_w + i * pix_w;
        // 用韵母几何小图标做列头
        draw_yunmu(&mut cv, px + pix_w/2 - 12, 4, 24, i as u32, 0x666666);
        line(&mut cv, px, title_h, px, title_h + rows * pix_h, 0x333333);
    }
    line(&mut cv, label_w + cols * pix_w, title_h, label_w + cols * pix_w, title_h + rows * pix_h, 0x333333);

    // 声调行标注条（左侧）
    for j in 0..rows {
        let py = title_h + j * pix_h;
        // 用声调几何做行头
        draw_tone(&mut cv, 2, py + 2, label_w - 8, j as u32, 0x666666);
        line(&mut cv, label_w, py, label_w + cols * pix_w, py, 0x333333);
    }
    line(&mut cv, label_w, title_h + rows * pix_h, label_w + cols * pix_w, title_h + rows * pix_h, 0x333333);
    // 声母底部横线
    line(&mut cv, 0, title_h, label_w, title_h, 0x555555);

    // 画中容器
    let mut drawn = 0u32;
    for yy in 0..12u32 {
        for t in 0..10u32 {
            let ox = label_w + yy as i32 * pix_w + 2;
            let oy = title_h + t as i32 * pix_h + 2;
            let mx = ((sid * 31 + yy * 10 + t * 7) % 10) as u32;
            let my = ((sid * 13 + yy * 7 + t * 11) % 10) as u32;
            draw_container(&mut cv, ox, oy, cw, ch, sid, yy, t, mx, my);
            drawn += 1;
        }
    }

    write_bmp_canvas(&cv, out);
    let non_bg = cv.d.iter().filter(|&&c| c != BG).count();
    println!("行视图 声母#{}({}) {}音节 画布:{}x{} 像素:{}({:.1}%) → {}",
             sid, ['b','p','m','f','d','t','n','l','g','k','h','w',
                   'j','q','x','y','z','c','s','r','ȥ','ç','ş','ņ',
                   'B','P','M','F','D','T','N','L','G','K','H','W',
                   'J','Q','X','Y','Z','C','S','R','Ȥ','Ç','Ş','Ņ'][sid as usize],
             drawn, canvas_w, canvas_h,
             non_bg, non_bg as f64 / (canvas_w * canvas_h) as f64 * 100.0, out);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "one" {
        let s: u32 = args.get(2).and_then(|x| x.parse().ok()).unwrap_or(0);
        let yy: u32 = args.get(3).and_then(|x| x.parse().ok()).unwrap_or(0);
        let t: u32 = args.get(4).and_then(|x| x.parse().ok()).unwrap_or(0);
        let big_w = 800i32;
        let big_h = 800i32;
        let mut cv = Cv { d: vec![0xFFFFFF; (big_w * big_h) as usize], w: big_w, h: big_h };

        let mx = ((s * 31 + yy * 17 + t * 7) % 10) as u32;
        let my = ((s * 13 + yy * 7 + t * 11) % 10) as u32;

        // 白色背景，黑色笔画，最大对比度
        let m = 40i32;
        draw_container_mono(&mut cv, m, m, big_w - 2*m, big_h - 2*m, s, yy, t, mx, my, 0x000000);

        let out = format!("/mnt/data/catpaw/home/workspace/硅碳心源-果套循因/toolchain/tier1/mqf_glyph_one_s{}_y{}_t{}.bmp", s, yy, t);
        write_bmp_canvas(&cv, &out);

        let non_black = cv.d.iter().filter(|&&c| c != 0xFFFFFF).count();
        println!("单音节: s={} y={} t={} mx={} my={} → {} 笔画像素:{} ({:.1}%)",
                 s, yy, t, mx, my, out, non_black, non_black as f64 / (big_w * big_h) as f64 * 100.0);
    } else if args.len() > 1 && args[1] == "row" {
        let sid: u32 = args.get(2).and_then(|x| x.parse().ok()).unwrap_or(0);
        let out = format!("/mnt/data/catpaw/home/workspace/硅碳心源-果套循因/toolchain/tier1/mqf_glyph_row_s{}.bmp", sid);
        draw_row(sid, &out);
    } else if args.len() > 1 && args[1] == "bias" {
        // 偏置可视化：bias s y t ±1.-2 1±.±3 ...
        // 把偏置记号的每一步画成从中心辐射的彩色条带
        let s: u32 = args.get(2).and_then(|x| x.parse().ok()).unwrap_or(0);
        let yy: u32 = args.get(3).and_then(|x| x.parse().ok()).unwrap_or(0);
        let t: u32 = args.get(4).and_then(|x| x.parse().ok()).unwrap_or(0);

        // 收集记号参数（第5个参数起）
        let mut notations = Vec::new();
        for i in 5..args.len() {
            notations.push(args[i].clone());
        }
        if notations.is_empty() {
            notations.push("+1".to_string());
        }

        let big_w = 800i32;
        let big_h = 800i32;
        let mut cv = Cv { d: vec![0x111111; (big_w * big_h) as usize], w: big_w, h: big_h };

        let m = 60i32;
        let cont_w = big_w - 2 * m;
        let cont_h = big_h - 2 * m - 80;
        // 先画容器本体（彩色）
        draw_container(&mut cv, m, m, cont_w, cont_h, s, yy, t,
                       ((s*31+yy*17+t*7)%10) as u32,
                       ((s*13+yy*7+t*11)%10) as u32);

        // 用 grid_notation 解析记号
        let cx = m + cont_w / 2;
        let cy = m + cont_h / 2;
        let cols: [u32; 6] = [0xFF4444, 0x44FF44, 0x4444FF, 0xFFFF44, 0xFF44FF, 0x44FFFF];

        for (idx, nota) in notations.iter().enumerate() {
            let col = cols[idx % 6];
            // 解析：Fine(d) → 中心微调, Grid(d) → 格间1/3宽, Row(d) → 行间1/3高
            let mut x = cx;
            let mut y = cy;

            // 拆解 "±1.-2" → ["±1", "-2"]
            for step in nota.split('.') {
                let step = step.trim();
                // 取量级数字
                let num_s: String = step.chars().filter(|c| c.is_ascii_digit()).collect();
                let mag: i32 = num_s.parse().unwrap_or(1);
                // 方向
                let has_neg = step.contains('-');
                let sign: i32 = if has_neg { -1 } else { 1 };

                let dx = match mag {
                    1 => sign * 8,                    // Fine：微调 ±8px
                    2 => sign * cont_w / 6,           // Grid：格间 1/6 宽度
                    3 => 0,                           // Row：Y方向
                    _ => sign * 8,
                };
                let dy = match mag {
                    3 => sign * cont_h / 6,           // Row：行间 1/6 高度
                    4 => sign * cont_h / 4,           // Fam：大跳
                    _ => 0,
                };

                let nx = x + dx;
                let ny = y + dy;
                // 画箭头带（2px 粗条）
                thick_line(&mut cv, x, y, nx, ny, col, 3);
                // 画端点方块
                rect(&mut cv, nx-3, ny-3, nx+3, ny+3, col);

                x = nx;
                y = ny;
            }
            // 在最终落点画一个带编号的圆
            let r = 12;
            for dy in -r..=r {
                let w = ((r*r - dy*dy) as f64).sqrt() as i32;
                rect(&mut cv, x - w, y + dy, x + w, y + dy, col);
            }
        }

        let nota_str = notations.join(" ");
        let out = format!("/mnt/data/catpaw/home/workspace/硅碳心源-果套循因/toolchain/tier1/mqf_glyph_bias_s{}_y{}_t{}_{}.bmp", s, yy, t, nota_str.replace([' ', '.', '±', '+', '-'], "_"));
        write_bmp_canvas(&cv, &out);

        let non_bg = cv.d.iter().filter(|&&c| c != 0x111111).count();
        println!("偏置: s={} y={} t={} 记号:[{}] → {} 像素({:.1}%)",
                 s, yy, t, nota_str, out, non_bg as f64 / (big_w * big_h) as f64 * 100.0);
    } else if args.len() > 1 && args[1] == "big" {
        let s: u32 = args.get(2).and_then(|x| x.parse().ok()).unwrap_or(0);
        let yy: u32 = args.get(3).and_then(|x| x.parse().ok()).unwrap_or(0);
        let t: u32 = args.get(4).and_then(|x| x.parse().ok()).unwrap_or(0);

        let big_w = 800i32;
        let big_h = 800i32;
        let mut cv = Cv { d: vec![BG; (big_w * big_h) as usize], w: big_w, h: big_h };

        let mx = ((s * 31 + yy * 17 + t * 7) % 10) as u32;
        let my = ((s * 13 + yy * 7 + t * 11) % 10) as u32;

        let m = 50i32;
        draw_container(&mut cv, m, m, big_w - 2 * m, big_h - 2 * m - 60, s, yy, t, mx, my);
        draw_label(&mut cv, m, big_h - 55, s, yy, t, mx, my);

        let out = format!("/mnt/data/catpaw/home/workspace/硅碳心源-果套循因/toolchain/tier1/mqf_glyph_big_s{}_y{}_t{}.bmp", s, yy, t);
        write_bmp_canvas(&cv, &out);

        let non_bg = cv.d.iter().filter(|&&c| c != BG).count();
        println!("放大: s={} y={} t={} mx={} my={} → {} 像素({:.1}%)",
                 s, yy, t, mx, my, out, non_bg as f64 / (big_w * big_h) as f64 * 100.0);
    } else if args.len() > 1 && args[1] == "tone" {
        // tone s y：固定声母+韵母，画 60 个声调变化（6×10 网格）
        let s: u32 = args.get(2).and_then(|x| x.parse().ok()).unwrap_or(0);
        let yy: u32 = args.get(3).and_then(|x| x.parse().ok()).unwrap_or(0);

        let cell = 96i32;
        let cols = 10i32;
        let rows = 6i32;
        let label_left = 80i32;
        let title_top = 30i32;
        let gap = 2i32;
        let cw = cell - 2 * gap - 2;
        let ch = cell - 2 * gap - 2;

        let canvas_w = label_left + cols * cell + 10;
        let canvas_h = title_top + rows * cell + 10;

        let mut cv = Cv { d: vec![BG; (canvas_w * canvas_h) as usize], w: canvas_w, h: canvas_h };

        // 列号（顶部）
        for c in 0..cols {
            let px = label_left + c * cell + cell / 2 - 3;
            draw_tone(&mut cv, px, 5, 20, c as u32, 0x666666);
        }
        // 行号（左侧）
        for r in 0..rows {
            let py = title_top + r * cell + cell / 2 - 3;
            draw_tone(&mut cv, 5, py, 60, (r * 10) as u32, 0x666666);
        }

        let mut drawn = 0u32;
        for t in 0..60u32 {
            let col_num = (t % 10) as i32;
            let row_num = (t / 10) as i32;
            let ox = label_left + col_num * cell + gap + 1;
            let oy = title_top + row_num * cell + gap + 1;
            let mx = ((s * 31 + yy * 17 + t * 7) % 10) as u32;
            let my = ((s * 13 + yy * 7 + t * 11) % 10) as u32;
            draw_container(&mut cv, ox, oy, cw, ch, s, yy, t, mx, my);
            drawn += 1;
        }

        let out = format!("/mnt/data/catpaw/home/workspace/硅碳心源-果套循因/toolchain/tier1/mqf_glyph_tone_s{}_y{}.bmp", s, yy);
        write_bmp_canvas(&cv, &out);
        let non_bg = cv.d.iter().filter(|&&c| c != BG).count();
        println!("声调视图 s={} y={} t:0-59 画布:{}x{} 像素:{}({:.1}%) → {}",
                 s, yy, canvas_w, canvas_h, non_bg, non_bg as f64 / (canvas_w * canvas_h) as f64 * 100.0, out);
    } else if args.len() > 2 && args[1] == "pipe" {
        // pipeline：一个音节一键生成 4 张图
        let s: u32 = args.get(2).and_then(|x| x.parse().ok()).unwrap_or(0);
        let y: u32 = args.get(3).and_then(|x| x.parse().ok()).unwrap_or(0);
        let t: u32 = args.get(4).and_then(|x| x.parse().ok()).unwrap_or(0);
        let prefix = format!("/mnt/data/catpaw/home/workspace/硅碳心源-果套循因/toolchain/tier1/mqf_pipe_s{}_y{}_t{}", s, y, t);

        draw_pipe_one(s, y, t, &prefix);
        draw_pipe_tones(s, y, &prefix);
        draw_pipe_row(s, &prefix);
        draw_pipe_bias(s, y, t, &prefix);

        println!("pipeline done: s={} y={} t={} → {}_*.bmp", s, y, t, prefix);
    } else {
        // ─── 默认：8×8 索引图（64 个音节，每个 80×80 看清细节）───
        let cell = 80i32;
        let cols = 8i32;
        let rows = 8i32;
        let label_left = 70i32;
        let title_top = 30i32;

        let cw = cell - 4;
        let ch = cell - 4;
        let canvas_w = label_left + cols * cell + 8;
        let canvas_h = title_top + rows * cell + 8;
        let mut cv = Cv { d: vec![BG; (canvas_w * canvas_h) as usize], w: canvas_w, h: canvas_h };

        for c in 0..cols {
            let px = label_left + c * cell + cell/2 - 12;
            draw_yunmu(&mut cv, px, 5, 24, c as u32, 0x888888);
            for dx in 0..(c as i32) {
                put(&mut cv, px + 15 + dx*2, 10, 0x666666);
            }
        }
        for r in 0..rows {
            let py = title_top + r * cell + cell/2 - 8;
            draw_shengmu(&mut cv, 8, py, 24, r as u32, 0xAAAAAA);
        }

        let tone = 5u32;
        let mut drawn = 0u32;
        for s in 0..rows as u32 {
            for y in 0..cols as u32 {
                let ox = label_left + y as i32 * cell + 2;
                let oy = title_top + s as i32 * cell + 2;
                let mx = ((s * 31 + y * 17 + tone * 7) % 10) as u32;
                let my = ((s * 13 + y * 7 + tone * 11) % 10) as u32;
                draw_container(&mut cv, ox, oy, cw, ch, s, y, tone, mx, my);
                drawn += 1;
            }
        }

        let out = "/mnt/data/catpaw/home/workspace/硅碳心源-果套循因/toolchain/tier1/mqf_glyph_v0_3.bmp";
        write_bmp_canvas(&cv, out);
        let non_bg = cv.d.iter().filter(|&&c| c != BG).count();
        println!("索引图 声母:{}×韵母:{}={}音节 @ 画布:{}x{} 像素:{}({:.1}%) → {}",
                 rows, cols, drawn, canvas_w, canvas_h,
                 non_bg, non_bg as f64 / (canvas_w * canvas_h) as f64 * 100.0, out);
    }
}

// ─── Pipeline 子函数 ─────────────────────────────────────

fn draw_pipe_one(s: u32, y: u32, t: u32, prefix: &str) {
    let big_w = 800i32;
    let big_h = 800i32;
    let mut cv = Cv { d: vec![0xFFFFFF; (big_w * big_h) as usize], w: big_w, h: big_h };
    let m = 40i32;
    draw_container_mono(&mut cv, m, m, big_w - 2*m, big_h - 2*m, s, y, t,
                        ((s*31+y*17+t*7)%10) as u32,
                        ((s*13+y*7+t*11)%10) as u32, 0x000000);
    let out = format!("{}_one.bmp", prefix);
    write_bmp_canvas(&cv, &out);
    println!("  ✓ 单音节 mono → {}", out);
}

fn draw_pipe_tones(s: u32, y: u32, prefix: &str) {
    let cell = 96i32;
    let cols = 10i32;
    let rows = 6i32;
    let label_left = 80i32;
    let title_top = 30i32;
    let cw = cell - 6;
    let canvas_w = label_left + cols * cell + 10;
    let canvas_h = title_top + rows * cell + 10;
    let mut cv = Cv { d: vec![BG; (canvas_w * canvas_h) as usize], w: canvas_w, h: canvas_h };
    for c in 0..cols {
        draw_tone(&mut cv, label_left + c * cell + cell/2 - 10, 5, 20, c as u32, 0x666666);
    }
    for r in 0..rows {
        draw_tone(&mut cv, 5, title_top + r * cell + cell/2 - 8, 60, (r*10) as u32, 0x666666);
    }
    for t in 0..60u32 {
        let ox = label_left + (t as i32 % 10) * cell + 3;
        let oy = title_top + (t as i32 / 10) * cell + 3;
        draw_container(&mut cv, ox, oy, cw, cw, s, y, t,
                       ((s*31+y*17+t*7)%10) as u32,
                       ((s*13+y*7+t*11)%10) as u32);
    }
    let out = format!("{}_tones.bmp", prefix);
    write_bmp_canvas(&cv, &out);
    println!("  ✓ 声调视图 {} → {}", 60, out);
}

fn draw_pipe_row(s: u32, prefix: &str) {
    draw_row(s, &format!("{}_row.bmp", prefix));
}

fn draw_pipe_bias(s: u32, y: u32, t: u32, prefix: &str) {
    let big_w = 800i32;
    let big_h = 800i32;
    let mut cv = Cv { d: vec![0x111111; (big_w * big_h) as usize], w: big_w, h: big_h };
    let m = 60i32;
    let cont_w = big_w - 2 * m;
    let cont_h = big_h - 2 * m - 80;
    draw_container(&mut cv, m, m, cont_w, cont_h, s, y, t,
                   ((s*31+y*17+t*7)%10) as u32,
                   ((s*13+y*7+t*11)%10) as u32);
    let cx = m + cont_w / 2;
    let cy = m + cont_h / 2;
    let examples = vec![
        ("+1", 0xFF4444, vec![(1i32, 0i32, 8i32, 0i32)]),
        ("±2", 0x44FF44, vec![(2, 0, cont_w/6, 0)]),
        ("-1.2+", 0x4444FF, vec![(1, 0, -8, 0), (2, 0, cont_w/6, 0)]),
        ("±3", 0xFFFF44, vec![(3, 1, 0, cont_h/6)]),
    ];
    for (_name, col, steps) in &examples {
        let (mut x, mut y_pos) = (cx, cy);
        for &(mag, _dir, dx, dy) in steps {
            let nx = x + dx;
            let ny = y_pos + dy;
            thick_line(&mut cv, x, y_pos, nx, ny, *col, 3);
            rect(&mut cv, nx-3, ny-3, nx+3, ny+3, *col);
            x = nx;
            y_pos = ny;
        }
    }
    let out = format!("{}_bias.bmp", prefix);
    write_bmp_canvas(&cv, &out);
    println!("  ✓ 偏置演示 → {}", out);
}
