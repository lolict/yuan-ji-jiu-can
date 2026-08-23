//! 相位像素 · 颜色进制底层对象
//!
//! 每个像素不是 RGB，而是：
//!   - phase: 相位编码值（进制编码，不是颜色）
//!   - weight: 视觉权重（笔画密度/胖瘦）
//!   - brightness: 亮度
//!   - metadata: 元数据纸条（来源母体、层级、归属权、黑名单标记）

use std::fmt;

/// 归属权
#[derive(Debug, Clone, PartialEq)]
pub enum Owner {
    SelfOwned,
    External,
}

impl Default for Owner {
    fn default() -> Owner {
        Owner::SelfOwned
    }
}

/// 相位像素 — 颜色进制的最小单元
#[derive(Debug, Clone)]
pub struct PhasePixel {
    /// 相位编码值（64进制/256进制等）
    pub phase: u16,
    /// 视觉权重 0-1（笔画密度/胖瘦）
    pub weight: f32,
    /// 亮度 0-1
    pub brightness: f32,
    /// 归属权
    pub owner: Owner,
    /// 层级: 1=基元本体 / 2=修饰部件 / 3=装配成品
    pub level: u8,
    /// 字形家族（用于审美配对）
    pub family: String,
    /// 母体部件来源（记录加减运算）
    pub parent_source: Vec<String>,
    /// 黑名单标记（犯罪笔录）
    pub blacklisted: bool,
    /// 可前置连接的节点 IDs
    pub prefix_ids: Vec<String>,
    /// 可后置连接的节点 IDs
    pub suffix_ids: Vec<String>,
    /// RGB 翻译层（输出时才用，不是底层本体）
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Default for PhasePixel {
    fn default() -> PhasePixel {
        PhasePixel {
            phase: 0,
            weight: 0.5,
            brightness: 1.0,
            owner: Owner::SelfOwned,
            level: 1,
            family: "default".to_string(),
            parent_source: Vec::new(),
            blacklisted: false,
            prefix_ids: Vec::new(),
            suffix_ids: Vec::new(),
            r: 0,
            g: 0,
            b: 0,
        }
    }
}

impl PhasePixel {
    /// 序列化为二进制（CBF 格式）— 12 字节定长
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(12);
        // phase: 2 bytes
        buf.extend_from_slice(&self.phase.to_be_bytes());
        // weight: 1 byte (0-255)
        buf.push((self.weight * 255.0).min(255.0) as u8);
        // brightness: 4 bytes (f32)
        buf.extend_from_slice(&self.brightness.to_be_bytes());
        // level: 1 byte
        buf.push(self.level);
        // owner flag: 1 byte
        buf.push(if self.owner == Owner::SelfOwned { 1 } else { 0 });
        // blacklist: 1 byte
        buf.push(if self.blacklisted { 1 } else { 0 });
        // padding to 12 bytes
        buf.extend_from_slice(&[0u8; 2]);
        buf
    }

    /// 从二进制反序列化
    pub fn from_bytes(data: &[u8]) -> Option<PhasePixel> {
        if data.len() < 12 {
            return None;
        }
        let phase = u16::from_be_bytes([data[0], data[1]]);
        let weight = data[2] as f32 / 255.0;
        let brightness = f32::from_be_bytes([data[3], data[4], data[5], data[6]]);
        let level = data[7];
        let owner = if data[8] == 1 {
            Owner::SelfOwned
        } else {
            Owner::External
        };
        let blacklisted = data[9] == 1;
        Some(PhasePixel {
            phase,
            weight,
            brightness,
            owner,
            level,
            blacklisted,
            ..Default::default()
        })
    }

    /// 相位 → RGB 翻译（输出的最后一步）
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        if self.r == 0 && self.g == 0 && self.b == 0 {
            let h = (self.phase % 360) as f32 / 360.0;
            let s = self.weight;
            let v = self.brightness;
            hsv_to_rgb(h, s, v)
        } else {
            (self.r, self.g, self.b)
        }
    }

    /// 等效镜像距离检查 — 两个节点能否配对
    pub fn compatible_with(&self, other: &PhasePixel) -> (bool, String) {
        if self.level != other.level {
            return (false, format!("层级不匹配: L{} vs L{}", self.level, other.level));
        }
        if self.family != other.family {
            return (false, format!("家族不同: {} vs {}", self.family, other.family));
        }
        let wdiff = (self.weight - other.weight).abs();
        if wdiff > 0.35 {
            return (false, format!("视觉重量差过大: {:.2}", wdiff));
        }
        if self.blacklisted || other.blacklisted {
            return (false, "黑名单标记".to_string());
        }
        (true, "等效对齐".to_string())
    }
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    if s == 0.0 {
        let val = (v * 255.0) as u8;
        return (val, val, val);
    }
    let i = (h * 6.0) as i32;
    let f = h * 6.0 - i as f32;
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);
    let rgb_table: [(f32, f32, f32); 6] = [
        (v, t, p),
        (q, v, p),
        (p, v, t),
        (p, q, v),
        (t, p, v),
        (v, p, q),
    ];
    let (r, g, b) = rgb_table[(i % 6) as usize];
    (
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
    )
}

impl fmt::Display for PhasePixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Pxl(φ={},w={:.2},L{},{},owner={:?})",
            self.phase, self.weight, self.level, self.family, self.owner
        )
    }
}

/// 相位画布 — 二维 PhasePixel 网格
#[derive(Debug, Clone)]
pub struct PhaseCanvas {
    pub width: u16,
    pub height: u16,
    pub name: String,
    pub grid: Vec<Vec<PhasePixel>>,
}

impl PhaseCanvas {
    pub fn new(width: u16, height: u16, name: &str) -> PhaseCanvas {
        let grid = vec![vec![PhasePixel::default(); width as usize]; height as usize];
        PhaseCanvas {
            width,
            height,
            name: name.to_string(),
            grid,
        }
    }

    pub fn get(&self, x: i32, y: i32) -> PhasePixel {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.grid[y as usize][x as usize].clone()
        } else {
            PhasePixel::default()
        }
    }

    pub fn set(&mut self, x: i32, y: i32, pixel: PhasePixel) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.grid[y as usize][x as usize] = pixel;
        }
    }

    /// 翻译为 RGB 矩阵（最后输出用）
    pub fn to_rgb_matrix(&self) -> Vec<Vec<(u8, u8, u8)>> {
        self.grid
            .iter()
            .map(|row| row.iter().map(|p| p.to_rgb()).collect())
            .collect()
    }
}

impl fmt::Display for PhaseCanvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Canvas({},{},'{}')", self.width, self.height, self.name)
    }
}
