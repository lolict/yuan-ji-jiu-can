#!/bin/bash
# 地址13: 编译 Rust 容器编码器 → 生成点阵字体BMP
set -e

TOOL="/mnt/data/catpaw/home/workspace/硅碳心源-果套循因/toolchain/tier1"
REG="/mnt/data/catpaw/home/workspace/硅碳心源-果套循因/registry"
cd "$TOOL"

LOG() { echo "[$(date '+%H:%M:%S')] addr=13 | $1" | tee -a "$REG/install.log"; }

LOG "开始编译容器编码器"

# 检查 Rust
source ~/.cargo/env 2>/dev/null
if ! command -v rustc &>/dev/null; then
    LOG "Rust 未安装，先安装..."
    RUSTUP_DIST_SERVER="https://rsproxy.cn" RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup" \
        bash /tmp/rustup-init.sh -y --default-toolchain stable --profile minimal 2>&1 | tail -3
    source ~/.cargo/env
fi
LOG "Rust $(rustc --version) 就绪"

# 创建 Rust 项目
LOG "创建 glyph_compiler 项目"
cargo init --name glyph_compiler glyph_compiler 2>&1 | tail -3
cd glyph_compiler

# 写入源码
LOG "写入编译器源码"
cat > src/main.rs << 'RUST_EOF'
/// 满全法容器编码器 v0.1
/// 输入: 符号表(声母+韵母) → 输出: BMP点阵
/// 零外部依赖，纯数学像素操作

const GLYPH_SIZE: u32 = 64;   // 每个字形 64x64 像素
const COLS: u32 = 12;         // 每行12个字
const CELL_PAD: u32 = 4;      // 格间距

/// 声母48 (24小写+24大写)
const SHENGMU: [&str; 48] = [
    "b","p","m","f","d","t","n","l","g","k","h","w",
    "j","q","x","y","z","c","s","r","ȥ","ç","ş","ŋ",
    "B","P","M","F","D","T","N","L","G","K","H","W",
    "J","Q","X","Y","Z","C","S","R","Ȥ","Ç","Ŋ","Ş",
];

/// 从一个符号名字生成几何地址（确定性哈希 → x,y 笔画参数）
fn symbol_to_geometry(name: &str) -> Vec<(u32, u32, u32, u32)> {
    let bytes = name.as_bytes();
    let mut strokes = Vec::new();
    // 用符号的字节值确定性生成笔画坐标
    let seed = if bytes.is_empty() { 0u32 } else {
        bytes.iter().fold(0u32, |acc, &b| acc.wrapping_mul(31).wrapping_add(b as u32))
    };
    // 生成1~4个笔画
    let n_strokes = ((seed % 4) + 1) as usize;
    for i in 0..n_strokes {
        let s = seed.wrapping_add(i as u32 * 137);
        let x0 = (s % 30) + 5;
        let y0 = ((s / 7) % 30) + 5;
        let x1 = ((s / 3) % 30) + 5;
        let y1 = ((s / 11) % 30) + 5;
        strokes.push((x0, y0, x1, y1));
    }
    strokes
}

/// 画线（Bresenham）
fn draw_line(canvas: &mut [u8], w: u32, h: u32, x0: i32, y0: i32, x1: i32, y1: i32) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;
    let mut x = x0;
    let mut y = y0;
    loop {
        if x >= 0 && x < w as i32 && y >= 0 && y < h as i32 {
            let idx = (y as u32 * w + x as u32) as usize;
            if idx < canvas.len() { canvas[idx] = 255; }
        }
        if x == x1 && y == y1 { break; }
        let e2 = 2 * err;
        if e2 > -dy { err -= dy; x += sx; }
        if e2 < dx { err += dx; y += sy; }
    }
}

/// 写 BMP 文件
fn save_bmp(path: &str, data: &[u8], width: u32, height: u32) {
    let row_size = (width + 3) & !3;  // 8bit灰度，4字节对齐
    let pixel_size = row_size * height;
    let file_size = 54 + 1024 + pixel_size;  // header + palette + pixels
    
    let mut buf = Vec::new();
    // BMP header
    buf.extend_from_slice(b"BM");
    buf.extend_from_slice(&file_size.to_le_bytes());
    buf.extend_from_slice(&[0u8; 4]);  // reserved
    let offset: u32 = 54 + 1024;
    buf.extend_from_slice(&offset.to_le_bytes());
    // DIB header
    buf.extend_from_slice(&40u32.to_le_bytes());  // header size
    buf.extend_from_slice(&(width as i32).to_le_bytes());
    buf.extend_from_slice(&(height as i32).to_le_bytes());
    buf.extend_from_slice(&1u16.to_le_bytes());   // planes
    buf.extend_from_slice(&8u16.to_le_bytes());   // bpp (8-bit grayscale)
    buf.extend_from_slice(&0u32.to_le_bytes());   // compression
    buf.extend_from_slice(&pixel_size.to_le_bytes());
    buf.extend_from_slice(&2835i32.to_le_bytes());  // x ppm
    buf.extend_from_slice(&2835i32.to_le_bytes());  // y ppm
    buf.extend_from_slice(&0u32.to_le_bytes());     // colors used
    buf.extend_from_slice(&0u32.to_le_bytes());     // important
    // Grayscale palette (256 entries)
    for i in 0u32..256 {
        buf.push(i as u8); buf.push(i as u8); buf.push(i as u8); buf.push(0);
    }
    // Pixel data (bottom-up, row-padded)
    for y in (0..height).rev() {
        for x in 0..width {
            buf.push(data[(y * width + x) as usize]);
        }
        for _ in 0..(row_size - width) {
            buf.push(0);
        }
    }
    std::fs::write(path, &buf).ok();
}

fn main() {
    let glyphs = SHENGMU.len() as u32;
    let rows = (glyphs + COLS - 1) / COLS;
    let cell = GLYPH_SIZE + CELL_PAD * 2;
    let width = COLS * cell;
    let height = rows * cell;
    
    let total = (width * height) as usize;
    let mut canvas = vec![18u8; total];  // 灰底=18 (深灰)
    
    // 画每个声母
    for (i, &name) in SHENGMU.iter().enumerate() {
        let col = (i as u32) % COLS;
        let row = (i as u32) / COLS;
        let ox = col * cell + CELL_PAD;
        let oy = row * cell + CELL_PAD;
        
        // 画边框
        for x in 0..GLYPH_SIZE {
            canvas[(oy * width + ox + x) as usize] = 40;
            canvas[((oy + GLYPH_SIZE - 1) * width + ox + x) as usize] = 40;
        }
        for y in 0..GLYPH_SIZE {
            canvas[((oy + y) * width + ox) as usize] = 40;
            canvas[((oy + y) * width + ox + GLYPH_SIZE - 1) as usize] = 40;
        }
        
        // 画中分线（田字格）
        for x in 0..GLYPH_SIZE {
            canvas[((oy + GLYPH_SIZE/2) * width + ox + x) as usize] = 35;
        }
        for y in 0..GLYPH_SIZE {
            canvas[((oy + y) * width + ox + GLYPH_SIZE/2) as usize] = 35;
        }
        
        // 根据符号名生成笔画
        let geo = symbol_to_geometry(name);
        for &(x0, y0, x1, y1) in &geo {
            draw_line(&mut canvas, width, height,
                     (ox + x0 as u32 % (GLYPH_SIZE-10) + 5) as i32,
                     (oy + y0 as u32 % (GLYPH_SIZE-10) + 5) as i32,
                     (ox + x1 as u32 % (GLYPH_SIZE-10) + 5) as i32,
                     (oy + y1 as u32 % (GLYPH_SIZE-10) + 5) as i32);
        }
    }
    
    let out_path = "/mnt/data/catpaw/home/workspace/硅碳心源-果套循因/toolchain/tier1/mqf_glyph_compiler_out.bmp";
    save_bmp(out_path, &canvas, width, height);
    
    let non_bg = canvas.iter().filter(|&&p| p > 50).count();
    let rate = non_bg as f64 / total as f64 * 100.0;
    
    println!("容器编码器 v0.1");
    println!("字形数: {}", glyphs);
    println!("画布: {}x{} ({} 字节)", width, height, total);
    println!("非背景像素: {} ({:.2}%)", non_bg, rate);
    println!("输出: {}", out_path);
}
RUST_EOF

LOG "编译中..."
cargo build --release 2>&1 | tail -5

LOG "运行..."
./target/release/glyph_compiler 2>&1

LOG "编译完成"

# 检查输出
ls -lh /mnt/data/catpaw/home/workspace/硅碳心源-果套循因/toolchain/tier1/mqf_glyph_compiler_out.bmp 2>&1
