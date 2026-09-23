// ══════════════════════════════════════════════════════════════════════
// glyph_pixel_algebra.rs · 字形像素加法器
// ══════════════════════════════════════════════════════════════════════
//
// 核心认知:
//   - 字不是字符串, 字是可拆解的像素结构
//   - 加法 = 把别的字的结构拆解下来安装在自己身上 (木+木=林)
//   - 减法 = 从自己身上移除某个部件
//   - 拆解 = 把字分离成独立的部件像素群
//   - 乘法/除法 = 在当前语境下不需要, 字不需要这个
//
// 像素 = 颜色(RGB) + 相位(位置编码0-255)
//   - 颜色: 该位置的墨色浓度 (0=白纸, 255=浓墨)
//   - 相位: 在字形结构中的相对位置角度 (0-255, 对应384°制式)
//
// 运算 = 像素的加减乘除
//   - 加法: out = clamp(a + b)  — 两个字叠加
//   - 减法: out = clamp(a - b)  — 移除部件
//   - 拆解: 按连通区域分离     — 把字拆成部件
//   - 组合: 把部件A安装到B上    — 加法的一种定向形式
//
// ══════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};

// 重新导出 Angle, 如果 crate::angle 不存在则定义本地版本
pub mod angle {
    /// 384° 制式角度 (满全法私有角度体系)
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Angle(pub u8); // 0-255 对应 0-384°

    impl Angle {
        pub const fn degrees(self) -> f64 {
            self.0 as f64 * 384.0 / 256.0
        }
        pub const fn from_degrees(d: f64) -> Self {
            Angle(((d / 384.0) * 256.0) as u8)
        }
    }
}

// ────────────────────────────────────────────────────────────────────
// 序列化用的像素表示 (不含元数据)
// ────────────────────────────────────────────────────────────────────

/// 可序列化的像素
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct SerPixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub phase: u8,
    pub alpha: u8,
}

/// 可序列化的字形图像
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SerGlyph {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec<SerPixel>>,
    pub name: String,
    pub level: String,  // L1/L2/L3
    pub glyph_type: String,
    pub owner: String,  // SELF/EXTERNAL
    pub visual_weight: f64,
}

// ────────────────────────────────────────────────────────────────────
// 像素 · 颜色+相位
// ────────────────────────────────────────────────────────────────────

/// 像素 — 字形的最小单位
///   - 颜色: RGB 三色值 (字形通常是黑白, 但保留彩色以支持彩色字形)
///   - 相位: 在字形结构中的相对位置角度 (0-255)
///   - alpha: 透明度 (0=全透, 255=不透明)
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct GlyphPixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub phase: u8,    // 相位: 字形结构中的位置角度 (0-255, 对应384°制式)
    pub alpha: u8,    // 透明度
}

impl GlyphPixel {
    /// 纯白像素 (空)
    pub fn blank() -> Self {
        Self { r: 255, g: 255, b: 255, phase: 0, alpha: 0 }
    }

    /// 纯黑像素 (墨)
    pub fn ink(phase: u8) -> Self {
        Self { r: 0, g: 0, b: 0, phase, alpha: 255 }
    }

    /// 灰色像素
    pub fn gray(v: u8, phase: u8) -> Self {
        Self { r: v, g: v, b: v, phase, alpha: 255 }
    }

    /// 彩色像素
    pub fn color(r: u8, g: u8, b: u8, phase: u8) -> Self {
        Self { r, g, b, phase, alpha: 255 }
    }

    /// 是否有墨 (非空白)
    pub fn is_ink(&self) -> bool {
        self.alpha > 0 && (self.r < 200 || self.g < 200 || self.b < 200)
    }

    /// 灰度值 (0=黑, 255=白)
    pub fn luminance(&self) -> u8 {
        if self.alpha == 0 { return 255; }
        let l = self.r as u32 * 299 + self.g as u32 * 587 + self.b as u32 * 114;
        (l / 1000) as u8
    }

    /// 两个像素的加法 (叠加)
    /// 颜色: 越叠加越黑 (字形叠加的逻辑)
    /// 相位: 取墨色较重者的相位
    pub fn add(&self, other: &GlyphPixel) -> GlyphPixel {
        if other.alpha == 0 { return *self; }
        if self.alpha == 0 { return *other; }

        // 颜色叠加: 越叠越黑 (min)
        let r = self.r.min(other.r);
        let g = self.g.min(other.g);
        let b = self.b.min(other.b);

        // 相位: 取墨色较重(较黑)一方的相位
        let phase = if self.luminance() <= other.luminance() { self.phase } else { other.phase };

        // 透明度: 叠加后不透明度增加
        let alpha = self.alpha.saturating_add(other.alpha).min(255);

        GlyphPixel { r, g, b, phase, alpha }
    }

    /// 两个像素的减法 (从self中移除other)
    /// 如果other在该位置有墨, self该位置变白
    pub fn subtract(&self, other: &GlyphPixel) -> GlyphPixel {
        if other.alpha == 0 || !other.is_ink() { return *self; }
        if self.alpha == 0 { return Self::blank(); }

        // 如果self该位置有墨, other也有墨, 则移除
        if self.is_ink() && other.is_ink() {
            // 减法结果: 变白 (墨被移除)
            Self::blank()
        } else {
            *self
        }
    }
}

// ────────────────────────────────────────────────────────────────────
// 字形图像 · 二维像素矩阵
// ────────────────────────────────────────────────────────────────────

/// 字形图像 — 一个可拆解的像素结构
/// 用行优先存储, 每行是一串像素
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GlyphImage {
    pub width: u32,
    pub height: u32,
    pub rows: Vec<Vec<GlyphPixel>>,  // rows[y][x]
}

impl GlyphImage {
    /// 创建空白字形
    pub fn new(width: u32, height: u32) -> Self {
        let rows = vec![vec![GlyphPixel::blank(); width as usize]; height as usize];
        Self { width, height, rows }
    }

    /// 获取像素
    pub fn get(&self, x: u32, y: u32) -> Option<GlyphPixel> {
        if x < self.width && y < self.height {
            Some(self.rows[y as usize][x as usize])
        } else {
            None
        }
    }

    /// 设置像素
    pub fn set(&mut self, x: u32, y: u32, pixel: GlyphPixel) {
        if x < self.width && y < self.height {
            self.rows[y as usize][x as usize] = pixel;
        }
    }

    /// 墨像素数量
    pub fn ink_count(&self) -> usize {
        self.rows.iter()
            .flat_map(|row| row.iter())
            .filter(|p| p.is_ink())
            .count()
    }

    /// 边界框: 墨像素的包围盒
    pub fn bounding_box(&self) -> Option<(u32, u32, u32, u32)> {
        let mut min_x = self.width;
        let mut min_y = self.height;
        let mut max_x = 0;
        let mut max_y = 0;
        let mut found = false;

        for y in 0..self.height {
            for x in 0..self.width {
                if self.rows[y as usize][x as usize].is_ink() {
                    min_x = min_x.min(x);
                    min_y = min_y.min(y);
                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                    found = true;
                }
            }
        }

        if found { Some((min_x, min_y, max_x, max_y)) } else { None }
    }

    // ────────────────────────────────────────────────────────────
    // 加法:把other叠加到self上
    // ────────────────────────────────────────────────────────────

    /// 全图加法: 两个相同尺寸的字形叠加
    pub fn add(&self, other: &GlyphImage) -> GlyphImage {
        let w = self.width.min(other.width);
        let h = self.height.min(other.height);
        let mut result = GlyphImage::new(w, h);

        for y in 0..h {
            for x in 0..w {
                let a = &self.rows[y as usize][x as usize];
                let b = &other.rows[y as usize][x as usize];
                result.set(x, y, a.add(b));
            }
        }

        result
    }

    /// 局部加法: 把other叠加到self的指定偏移位置
    /// 用于"把别的字的结构拆解下来安装在自己身上"
    pub fn add_at(&self, other: &GlyphImage, offset_x: i32, offset_y: i32) -> GlyphImage {
        let mut result = self.clone();

        for y in 0..other.height {
            for x in 0..other.width {
                let src = &other.rows[y as usize][x as usize];
                let tx = x as i32 + offset_x;
                let ty = y as i32 + offset_y;

                if tx >= 0 && ty >= 0 && (tx as u32) < result.width && (ty as u32) < result.height {
                    let tx = tx as u32;
                    let ty = ty as u32;
                    let dst = &result.rows[ty as usize][tx as usize];
                    result.set(tx, ty, dst.add(src));
                }
            }
        }

        result
    }

    // ────────────────────────────────────────────────────────────
    // 减法:从self中移除other
    // ────────────────────────────────────────────────────────────

    /// 全图减法: 从self中移除other的墨
    pub fn subtract(&self, other: &GlyphImage) -> GlyphImage {
        let w = self.width.min(other.width);
        let h = self.height.min(other.height);
        let mut result = self.clone();

        for y in 0..h {
            for x in 0..w {
                let a = &self.rows[y as usize][x as usize];
                let b = &other.rows[y as usize][x as usize];
                result.set(x, y, a.subtract(b));
            }
        }

        result
    }

    // ────────────────────────────────────────────────────────────
    // 拆解:把字分离成部件
    // ────────────────────────────────────────────────────────────

    /// 拆解: 按连通区域分离成独立的字形图像
    /// 4-连通: 上下左右相邻的墨像素属于同一个部件
    pub fn disassemble(&self) -> Vec<GlyphImage> {
        let mut labels = vec![vec![0u32; self.width as usize]; self.height as usize];
        let mut current_label: u32 = 0;
        let mut regions: Vec<Vec<(u32, u32)>> = Vec::new();

        // 第一次遍历: 标记连通区域
        for y in 0..self.height {
            for x in 0..self.width {
                if self.rows[y as usize][x as usize].is_ink() && labels[y as usize][x as usize] == 0 {
                    current_label += 1;
                    let mut region = Vec::new();
                    self.flood_fill(x, y, current_label, &mut labels, &mut region);
                    regions.push(region);
                }
            }
        }

        // 为每个区域生成独立的字形图像
        let mut parts = Vec::new();
        for region in &regions {
            if region.is_empty() { continue; }

            let min_x = region.iter().map(|(x, _)| *x).min().unwrap_or(0);
            let min_y = region.iter().map(|(_, y)| *y).min().unwrap_or(0);
            let max_x = region.iter().map(|(x, _)| *x).max().unwrap_or(0);
            let max_y = region.iter().map(|(_, y)| *y).max().unwrap_or(0);

            let w = max_x - min_x + 1;
            let h = max_y - min_y + 1;
            let mut part = GlyphImage::new(w, h);

            for (rx, ry) in region {
                let px = self.rows[*ry as usize][*rx as usize];
                part.set(*rx - min_x, *ry - min_y, px);
            }

            parts.push(part);
        }

        parts
    }

    /// 泛洪填充 — 用于连通区域检测
    fn flood_fill(&self, x: u32, y: u32, label: u32, labels: &mut Vec<Vec<u32>>, region: &mut Vec<(u32, u32)>) {
        let mut stack = vec![(x, y)];

        while let Some((cx, cy)) = stack.pop() {
            if cx >= self.width || cy >= self.height { continue; }
            if labels[cy as usize][cx as usize] != 0 { continue; }
            if !self.rows[cy as usize][cx as usize].is_ink() { continue; }

            labels[cy as usize][cx as usize] = label;
            region.push((cx, cy));

            // 4-连通邻居
            if cx > 0 { stack.push((cx - 1, cy)); }
            if cx + 1 < self.width { stack.push((cx + 1, cy)); }
            if cy > 0 { stack.push((cx, cy - 1)); }
            if cy + 1 < self.height { stack.push((cx, cy + 1)); }
        }
    }

    // ────────────────────────────────────────────────────────────
    // 组合:把部件安装到指定位置
    // ────────────────────────────────────────────────────────────

    /// 组合: 把部件part安装到self的指定位置
    /// 这是定向的加法, 明确指定安装位置
    pub fn assemble(&self, part: &GlyphImage, x: u32, y: u32) -> GlyphImage {
        self.add_at(part, x as i32, y as i32)
    }

    // ────────────────────────────────────────────────────────────
    // ASCII 渲染 (终端显示)
    // ────────────────────────────────────────────────────────────

    /// 渲染为 ASCII 艺术 (用于终端显示)
    /// 墨密度越高字符越密: ░▒▓█
    pub fn to_ascii(&self) -> String {
        let mut s = String::new();
        for row in &self.rows {
            for p in row {
                if p.alpha == 0 {
                    s.push(' ');
                } else {
                    let lum = p.luminance();
                    let ch = if lum < 64 { '█' }
                        else if lum < 128 { '▓' }
                        else if lum < 192 { '▒' }
                        else if lum < 240 { '░' }
                        else { '·' };
                    s.push(ch);
                }
            }
            s.push('\n');
        }
        s
    }

    /// 带边框的 ASCII 渲染
    pub fn to_ascii_boxed(&self) -> String {
        let mut s = String::new();
        // 顶部边框
        s.push('┌');
        for _ in 0..self.width { s.push('─'); }
        s.push('┐');
        s.push('\n');

        for row in &self.rows {
            s.push('│');
            for p in row {
                if p.alpha == 0 {
                    s.push(' ');
                } else {
                    let lum = p.luminance();
                    let ch = if lum < 64 { '█' }
                        else if lum < 128 { '▓' }
                        else if lum < 192 { '▒' }
                        else if lum < 240 { '░' }
                        else { '·' };
                    s.push(ch);
                }
            }
            s.push('│');
            s.push('\n');
        }

        // 底部边框
        s.push('└');
        for _ in 0..self.width { s.push('─'); }
        s.push('┘');
        s.push('\n');

        s
    }

    // ────────────────────────────────────────────────────────────
    // 序列化 / 反序列化
    // ────────────────────────────────────────────────────────────

    /// 转为可序列化形式
    pub fn to_ser(&self, name: &str) -> SerGlyph {
        let pixels: Vec<Vec<SerPixel>> = self.rows.iter().map(|row| {
            row.iter().map(|p| SerPixel {
                r: p.r, g: p.g, b: p.b, phase: p.phase, alpha: p.alpha,
            }).collect()
        }).collect();

        SerGlyph {
            width: self.width,
            height: self.height,
            pixels,
            name: name.to_string(),
            level: "L1".to_string(),
            glyph_type: "base".to_string(),
            owner: "SELF".to_string(),
            visual_weight: 0.5,
        }
    }

    /// 从可序列化形式恢复
    pub fn from_ser(sg: &SerGlyph) -> Self {
        let rows: Vec<Vec<GlyphPixel>> = sg.pixels.iter().map(|row| {
            row.iter().map(|sp| GlyphPixel {
                r: sp.r, g: sp.g, b: sp.b, phase: sp.phase, alpha: sp.alpha,
            }).collect()
        }).collect();

        Self {
            width: sg.width,
            height: sg.height,
            rows,
        }
    }

    /// 保存为 JSON 文件
    pub fn save_json(&self, path: &str, name: &str) -> std::io::Result<()> {
        let sg = self.to_ser(name);
        let json = serde_json::to_string_pretty(&sg).unwrap();
        std::fs::write(path, json)
    }

    /// 从 JSON 文件加载
    pub fn load_json(path: &str) -> std::io::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let sg: SerGlyph = serde_json::from_str(&json).unwrap();
        Ok(Self::from_ser(&sg))
    }
}

// ────────────────────────────────────────────────────────────────────
// 字形模板 — 预定义的简单字形像素结构
// ────────────────────────────────────────────────────────────────────

pub struct GlyphTemplates;

impl GlyphTemplates {
    /// 生成一个"木"字的简化像素模板 (8x8)
    /// 用一个十字形表示: 一横 + 一竖 + 撇捺
    pub fn mu() -> GlyphImage {
        let mut g = GlyphImage::new(8, 8);
        // 横 (第2行)
        for x in 1..7 {
            g.set(x, 2, GlyphPixel::ink(phase::HORIZONTAL));
        }
        // 竖 (第4列)
        for y in 1..7 {
            g.set(4, y, GlyphPixel::ink(phase::VERTICAL));
        }
        // 撇 (左上到中)
        g.set(2, 1, GlyphPixel::ink(phase::DIAGONAL));
        g.set(3, 2, GlyphPixel::ink(phase::DIAGONAL));
        // 捺 (右上到中)
        g.set(6, 1, GlyphPixel::ink(phase::DIAGONAL));
        g.set(5, 2, GlyphPixel::ink(phase::DIAGONAL));
        g
    }

    /// 生成一个"林"字 = 木 + 木
    pub fn lin() -> GlyphImage {
        let mu = Self::mu();
        let mut result = mu.clone();
        // 把第二个木叠加到右边
        result = result.add_at(&mu, 3, 0);
        result
    }

    /// "口"字 — 一个方块
    pub fn kou() -> GlyphImage {
        let mut g = GlyphImage::new(6, 6);
        // 上横
        for x in 1..5 { g.set(x, 1, GlyphPixel::ink(phase::HORIZONTAL)); }
        // 下横
        for x in 1..5 { g.set(x, 4, GlyphPixel::ink(phase::HORIZONTAL)); }
        // 左竖
        for y in 1..5 { g.set(1, y, GlyphPixel::ink(phase::VERTICAL)); }
        // 右竖
        for y in 1..5 { g.set(4, y, GlyphPixel::ink(phase::VERTICAL)); }
        g
    }

    /// "日"字 — 口 + 中间一横
    pub fn ri() -> GlyphImage {
        let mut g = Self::kou();
        // 中间一横
        for x in 1..5 { g.set(x, 2, GlyphPixel::ink(phase::HORIZONTAL)); }
        // 中间一横的相位设为中间
        for x in 1..5 {
            g.set(x, 2, GlyphPixel::ink(phase::MIDDLE));
        }
        g
    }

    /// "田"字 — 口 + 十字
    pub fn tian() -> GlyphImage {
        let mut g = Self::kou();
        // 中间十字
        for x in 1..5 { g.set(x, 2, GlyphPixel::ink(phase::MIDDLE)); }
        for y in 1..5 { g.set(2, y, GlyphPixel::ink(phase::MIDDLE)); }
        g
    }

    /// "一"字 — 一条横线
    pub fn yi() -> GlyphImage {
        let mut g = GlyphImage::new(8, 3);
        for x in 1..7 { g.set(x, 1, GlyphPixel::ink(phase::HORIZONTAL)); }
        g
    }

    /// "丨"字 — 一条竖线
    pub fn shu() -> GlyphImage {
        let mut g = GlyphImage::new(3, 8);
        for y in 1..7 { g.set(1, y, GlyphPixel::ink(phase::VERTICAL)); }
        g
    }
}

// 相位常量
pub mod phase {
    pub const HORIZONTAL: u8 = 0;    // 横
    pub const VERTICAL: u8 = 64;     // 竖 (384°/4 ≈ 96°, 映射到64)
    pub const DIAGONAL: u8 = 128;    // 撇捺 (斜)
    pub const MIDDLE: u8 = 192;      // 中间
    pub const UNKNOWN: u8 = 255;     // 未知
}

// ────────────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixel_add_combines_ink() {
        let a = GlyphPixel::ink(0);
        let b = GlyphPixel::ink(64);
        let result = a.add(&b);
        assert!(result.is_ink(), "两个墨像素叠加后应仍有墨");
    }

    #[test]
    fn pixel_add_with_blank_preserves_ink() {
        let ink = GlyphPixel::ink(0);
        let blank = GlyphPixel::blank();
        let result = ink.add(&blank);
        assert!(result.is_ink(), "墨+空=墨");
    }

    #[test]
    fn pixel_subtract_removes_ink() {
        let a = GlyphPixel::ink(0);
        let b = GlyphPixel::ink(0);
        let result = a.subtract(&b);
        assert!(!result.is_ink(), "墨-墨=空");
    }

    #[test]
    fn pixel_add_preserves_heavier_phase() {
        let dark = GlyphPixel::ink(10);   // 相位10, 墨
        let light = GlyphPixel::gray(100, 200); // 相位200, 灰
        let result = dark.add(&light);
        // 墨色较重者的相位应该被保留
        assert_eq!(result.phase, 10);
    }

    #[test]
    fn glyph_add_combines_two_images() {
        let mut a = GlyphImage::new(8, 8);
        let mut b = GlyphImage::new(8, 8);

        // a: 左上角一个墨点
        a.set(1, 1, GlyphPixel::ink(0));
        // b: 右下角一个墨点
        b.set(6, 6, GlyphPixel::ink(0));

        let result = a.add(&b);
        assert!(result.get(1, 1).unwrap().is_ink());
        assert!(result.get(6, 6).unwrap().is_ink());
    }

    #[test]
    fn glyph_add_at_places_part() {
        let base = GlyphImage::new(10, 10);
        let mut part = GlyphImage::new(4, 4);
        part.set(1, 1, GlyphPixel::ink(0));
        part.set(2, 2, GlyphPixel::ink(0));

        let result = base.add_at(&part, 3, 3);
        assert!(result.get(4, 4).unwrap().is_ink());
        assert!(result.get(5, 5).unwrap().is_ink());
    }

    #[test]
    fn glyph_subtract_removes_part() {
        let mut base = GlyphImage::new(8, 8);
        base.set(2, 2, GlyphPixel::ink(0));
        base.set(3, 3, GlyphPixel::ink(0));

        let mut part = GlyphImage::new(8, 8);
        part.set(3, 3, GlyphPixel::ink(0));

        let result = base.subtract(&part);
        assert!(result.get(2, 2).unwrap().is_ink());  // 未移除的保留
        assert!(!result.get(3, 3).unwrap().is_ink()); // 被移除的变空
    }

    #[test]
    fn glyph_disassemble_separates_regions() {
        let mut g = GlyphImage::new(10, 5);
        // 左边一个部件
        g.set(1, 1, GlyphPixel::ink(0));
        g.set(1, 2, GlyphPixel::ink(0));
        // 右边一个部件 (不连通)
        g.set(7, 1, GlyphPixel::ink(0));
        g.set(7, 2, GlyphPixel::ink(0));

        let parts = g.disassemble();
        assert_eq!(parts.len(), 2, "应拆解为2个部件");
    }

    #[test]
    fn glyph_disassemble_single_region() {
        let mut g = GlyphImage::new(5, 5);
        // 一个连通的十字
        g.set(2, 1, GlyphPixel::ink(0));
        g.set(2, 2, GlyphPixel::ink(0));
        g.set(2, 3, GlyphPixel::ink(0));
        g.set(1, 2, GlyphPixel::ink(0));
        g.set(3, 2, GlyphPixel::ink(0));

        let parts = g.disassemble();
        assert_eq!(parts.len(), 1, "十字应为一个连通区域");
    }

    #[test]
    fn glyph_assemble_places_part_at_location() {
        let mut base = GlyphImage::new(10, 10);
        base.set(1, 1, GlyphPixel::ink(0));

        let mut part = GlyphImage::new(4, 4);
        part.set(0, 0, GlyphPixel::ink(0));
        part.set(3, 3, GlyphPixel::ink(0));

        let result = base.assemble(&part, 5, 5);
        assert!(result.get(1, 1).unwrap().is_ink());  // 原来的
        assert!(result.get(5, 5).unwrap().is_ink());  // 安装的部件
        assert!(result.get(8, 8).unwrap().is_ink());  // 安装的部件
    }

    #[test]
    fn template_mu_is_ink() {
        let g = GlyphTemplates::mu();
        assert!(g.ink_count() > 0, "木模板应有墨");
    }

    #[test]
    fn template_lin_has_more_ink_than_mu() {
        let mu = GlyphTemplates::mu();
        let lin = GlyphTemplates::lin();
        assert!(lin.ink_count() >= mu.ink_count(), "林的墨应不少于木");
    }

    #[test]
    fn template_kou_is_hollow() {
        let g = GlyphTemplates::kou();
        // 中间应该是空的
        assert!(!g.get(2, 2).unwrap().is_ink(), "口中间应为空");
        // 四边应有墨
        assert!(g.get(1, 1).unwrap().is_ink(), "口上边应有墨");
    }

    #[test]
    fn template_tian_has_cross() {
        let g = GlyphTemplates::tian();
        assert!(g.get(2, 2).unwrap().is_ink(), "田中间十字交叉应有墨");
    }

    #[test]
    fn bounding_box_detects_ink_region() {
        let mut g = GlyphImage::new(10, 10);
        g.set(3, 3, GlyphPixel::ink(0));
        g.set(5, 7, GlyphPixel::ink(0));

        let bbox = g.bounding_box();
        assert!(bbox.is_some());
        let (min_x, min_y, max_x, max_y) = bbox.unwrap();
        assert_eq!(min_x, 3);
        assert_eq!(min_y, 3);
        assert_eq!(max_x, 5);
        assert_eq!(max_y, 7);
    }
}
