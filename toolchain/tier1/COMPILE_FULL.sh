#!/bin/bash
# 地址14: 满全法五维寻址编译器
# 声母48 × 韵母66 × 声调60 × 颜色42 × 微位置100
set -e

TOOL="/mnt/data/catpaw/home/workspace/硅碳心源-果套循因/toolchain/tier1"
REG="/mnt/data/catpaw/home/workspace/硅碳心源-果套循因/registry"
source ~/.cargo/env 2>/dev/null
export PATH="$HOME/.cargo/bin:$PATH"

LOG() { echo "[$(date '+%H:%M:%S')] addr=14 | $1" | tee -a "$REG/install.log"; }

cd "$TOOL/glyph_compiler"
LOG "开始五维寻址编译器"

cat > src/main.rs << 'RUST_EOF'
/// 满全法五维寻址编译器 v0.2
/// 输入: (声母,韵母,声调) → 输出: 颜色+微位置 → BMP

const W: u32 = 600;
const H: u32 = 400;

struct Address {
    sheng: u32,  // 0-47 (48进制)
    yun: u32,    // 0-65 (66进制)
    tone: u32,   // 0-59 (60进制)
}

struct Output {
    color_id: u32,  // 0-41 (42色)
    micro_x: u32,   // 0-9
    micro_y: u32,   // 0-9
}

const COLORS: [(u8, u8, u8); 42] = [
    (220,80,80),(220,120,60),(220,180,50),(200,220,60),(160,220,70),(120,210,80),
    (80,200,120),(60,190,160),(60,170,200),(70,140,220),(90,110,220),(120,90,220),
    (160,80,210),(200,80,200),(220,90,170),(220,110,140),(210,140,120),(200,170,110),
    (180,200,120),(150,210,140),(120,220,170),(90,210,200),(70,180,220),(80,150,220),
    (100,120,220),(130,100,210),(170,90,200),(210,100,180),(220,130,150),(210,160,130),
    (190,190,130),(160,210,150),(130,220,180),(100,210,210),(80,180,220),(90,160,210),
    (110,140,200),(140,130,190),(180,130,180),(210,140,170),(220,160,150),(200,180,150),
    (170,200,160),(140,210,180),
];

fn compute(addr: &Address) -> Output {
    // 五维 → 输出
    let base = addr.sheng * 60 * 66 + addr.tone * 66 + addr.yun;
    let color_id = (base % 42) as u32;
    let micro_raw = ((addr.sheng * 31 + addr.yun * 17 + addr.tone * 7) % 100) as u32;
    let micro_x = micro_raw % 10;
    let micro_y = micro_raw / 10;
    Output { color_id, micro_x, micro_y }
}

fn draw_line(canvas: &mut [u8], w: u32, h: u32, x0: i32, y0: i32, x1: i32, y1: i32, color: (u8, u8, u8), ch: usize) {
    let dx = (x0 - x1).abs();
    let dy = (y0 - y1).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;
    let mut x = x0;
    let mut y = y0;
    loop {
        if x >= 0 && x < w as i32 && y >= 0 && y < h as i32 {
            let idx = ((y as u32 * w + x as u32) * 3) as usize;
            if idx + 2 < canvas.len() {
                if ch == 0 { canvas[idx] = color.0; }
                else if ch == 1 { canvas[idx+1] = color.1; }
                else { canvas[idx+2] = color.2; }
            }
        }
        if x == x1 && y == y1 { break; }
        let e2 = 2 * err;
        if e2 > -dy { err -= dy; x += sx; }
        if e2 < dx { err += dx; y += sy; }
    }
}

fn save_bmp(path: &str, data: &[u8], width: u32, height: u32) {
    let row_size = (width * 3 + 3) & !3;
    let pixel_size = row_size * height;
    let file_size = 54 + pixel_size;
    let mut buf = Vec::new();
    buf.extend_from_slice(b"BM");
    buf.extend_from_slice(&file_size.to_le_bytes());
    buf.extend_from_slice(&[0u8; 4]);
    buf.extend_from_slice(&54u32.to_le_bytes());
    buf.extend_from_slice(&40u32.to_le_bytes());
    buf.extend_from_slice(&(width as i32).to_le_bytes());
    buf.extend_from_slice(&(height as i32).to_le_bytes());
    buf.extend_from_slice(&1u16.to_le_bytes());
    buf.extend_from_slice(&24u16.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&pixel_size.to_le_bytes());
    buf.extend_from_slice(&2835i32.to_le_bytes());
    buf.extend_from_slice(&2835i32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    for y in (0..height).rev() {
        for x in 0..width {
            let idx = (y * width + x) * 3;
            buf.push(data[(idx + 2) as usize]); // B
            buf.push(data[(idx + 1) as usize]); // G
            buf.push(data[idx] as usize);       // R
        }
        for _ in 0..(row_size - width * 3) { buf.push(0); }
    }
    std::fs::write(path, &buf).ok();
}

fn main() {
    let total = (W * H * 3) as usize;
    let mut canvas = vec![30u8; total];  // 深色背景
    
    let mut count = 0;
    
    // 遍历所有声母×韵母×声调，把结果映射到画布上
    for s in 0..24u32 {       # 前24声母
        for y in 0..11u32 {   # 前11韵母
            for t in 0..6u32 { # 6个基础声调组合
                let addr = Address { sheng: s, yun: y, tone: t };
                let out = compute(&addr);
                let color = COLORS[out.color_id as usize];
                
                # 把(声母,韵母,声调)映射到画布网格位置
                let grid_x = count % 30;
                let grid_y = count / 30;
                if grid_y >= 12 { break; }
                
                let cx = 30 + grid_x * 18;
                let cy = 40 + grid_y * 28;
                
                # 画一个色块 + 微位置标记
                for dy in 0..16 {
                    for dx in 0..10 {
                        let px = cx + dx;
                        let py = cy + dy;
                        if px < W && py < H {
                            let idx = ((py * W + px) * 3) as usize;
                            canvas[idx] = color.0;
                            canvas[idx+1] = color.1;
                            let blend = 255 - (dy * 8);
                            canvas[idx+2] = ((color.2 as u16 + blend) / 2) as u8;
                        }
                    }
                }
                
                # 微位置：中心点小点标记
                let mx = cx + out.micro_x + 3;
                let my = cy + out.micro_y;
                if mx < W && my < H {
                    for dy in 0..2 {
                        for dx in 0..2 {
                            let idx = (((my+dy) * W + (mx+dx)) * 3) as usize;
                            canvas[idx] = 255;
                            canvas[idx+1] = 255;
                            canvas[idx+2] = 255;
                        }
                    }
                }
                
                count += 1;
                if count >= 360 { break; }
            }
        }
    }
    
    let out_path = "/mnt/data/catpaw/home/workspace/硅碳心源-果套循因/toolchain/tier1/mqf_address_map.bmp";
    save_bmp(out_path, &canvas, W, H);
    
    let non_bg = canvas.chunks(3).filter(|p| p[0] > 50 || p[1] > 50 || p[2] > 50).count();
    let rate = non_bg as f64 / (W * H) as f64 * 100.0;
    
    println!("五维寻址编译器 v0.2");
    println!("声母×韵母×声调 → 颜色+微位置 → 色块地图");
    println!("映射组合数: {} / {} 画布覆盖 {:.2}%", count, W*H, rate);
    println!("输出: {}", out_path);
}
RUST_EOF

# 修复 Rust syntax（Python f-string style # 注释在Rust里不行，重写）
cat > src/main.rs << 'RUST_EOF'
const W: u32 = 600;
const H: u32 = 400;

fn compute(sheng: u32, yun: u32, tone: u32) -> (u32, u32, u32) {
    let base = sheng * 60 * 66 + tone * 66 + yun;
    let color_id = base % 42;
    let m = (sheng * 31 + yun * 17 + tone * 7) % 100;
    (color_id, m % 10, m / 10)
}

const COLORS: [(u8, u8, u8); 42] = [
    (220,80,80),(220,120,60),(220,180,50),(200,220,60),(160,220,70),(120,210,80),
    (80,200,120),(60,190,160),(60,170,200),(70,140,220),(90,110,220),(120,90,220),
    (160,80,210),(200,80,200),(220,90,170),(220,110,140),(210,140,120),(200,170,110),
    (180,200,120),(150,210,140),(120,220,170),(90,210,200),(70,180,220),(80,150,220),
    (100,120,220),(130,100,210),(170,90,200),(210,100,180),(220,130,150),(210,160,130),
    (190,190,130),(160,210,150),(130,220,180),(100,210,210),(80,180,220),(90,160,210),
    (110,140,200),(140,130,190),(180,130,180),(210,140,170),(220,160,150),(200,180,150),
    (170,200,160),(140,210,180),
];

fn main() {
    let total = (W * H * 3) as usize;
    let mut canvas = vec![30u8; total];
    let mut count = 0u32;

    for s in 0..24u32 {
        for y in 0..11u32 {
            for t in 0..6u32 {
                let (cid, mx, my) = compute(s, y, t);
                let color = COLORS[cid as usize];
                let gx = count % 30;
                let gy = count / 30;
                if gy >= 12 { break; }
                let cx = 30 + gx * 18;
                let cy = 40 + gy * 28;
                for dy in 0..16u32 {
                    for dx in 0..10u32 {
                        let px = cx + dx;
                        let py = cy + dy;
                        if px < W && py < H {
                            let idx = ((py * W + px) * 3) as usize;
                            canvas[idx] = color.0;
                            canvas[idx+1] = color.1;
                            canvas[idx+2] = color.2 / 2 + 128 - dy * 5;
                        }
                    }
                }
                for ey in 0..3u32 {
                    for ex in 0..3u32 {
                        let px = cx + mx + ex + 2;
                        let py = cy + my + ey;
                        if px < W && py < H {
                            let idx = ((py * W + px) * 3) as usize;
                            canvas[idx] = 255;
                            canvas[idx+1] = 255;
                            canvas[idx+2] = 255;
                        }
                    }
                }
                count += 1;
            }
        }
    }

    // BMP输出
    let row_size = (W * 3 + 3) & !3;
    let pixel_size = row_size * H;
    let file_size = 54 + pixel_size;
    let mut buf: Vec<u8> = Vec::with_capacity(file_size as usize);
    buf.extend_from_slice(b"BM");
    buf.extend_from_slice(&file_size.to_le_bytes());
    buf.extend_from_slice(&[0u8; 4]);
    buf.extend_from_slice(&54u32.to_le_bytes());
    buf.extend_from_slice(&40u32.to_le_bytes());
    buf.extend_from_slice(&(W as i32).to_le_bytes());
    buf.extend_from_slice(&(H as i32).to_le_bytes());
    buf.extend_from_slice(&1u16.to_le_bytes());
    buf.extend_from_slice(&24u16.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&pixel_size.to_le_bytes());
    buf.extend_from_slice(&2835i32.to_le_bytes());
    buf.extend_from_slice(&2835i32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    for yy in (0..H).rev() {
        for x in 0..W {
            let idx = (yy * W + x) * 3;
            buf.push((canvas[(idx+2) as usize]).min(255));
            buf.push((canvas[(idx+1) as usize]).min(255));
            buf.push((canvas[idx as usize]).min(255));
        }
        for _ in 0..(row_size - W * 3) { buf.push(0); }
    }

    let out = "/mnt/data/catpaw/home/workspace/硅碳心源-果套循因/toolchain/tier1/mqf_address_map.bmp";
    std::fs::write(out, &buf).ok();

    let non_bg = canvas.chunks(3).filter(|p| {
        (p[0] as u16 + p[1] as u16 + p[2] as u16) > 90
    }).count();
    let rate = non_bg as f64 / (W * H) as f64 * 100.0;
    println!("五维寻址编译器 v0.2");
    println!("组合 {} / {} 像素 覆盖 {:.2}%", count, W*H, rate);
    println!("输出: {}", out);
}
RUST_EOF

LOG "编译五维寻址..."
cargo build --release 2>&1 | tail -3
LOG "运行..."
./target/release/glyph_compiler 2>&1

ls -lh /mnt/data/catpaw/home/workspace/硅碳心源-果套循因/toolchain/tier1/mqf_address_map.bmp 2>&1
LOG "完成"
