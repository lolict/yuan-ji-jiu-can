//! Radix3D · 三维进制解释器
//! 声母×韵母×声调=坐标

use crate::constants::*;
use crate::loader::const_str_eq;

/// 三维坐标点
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point3D {
    pub x: usize,  // 声母索引
    pub y: usize,  // 韵母索引
    pub z: usize,  // 声调索引
}

impl Point3D {
    /// 创建新坐标点
    pub const fn new(x: usize, y: usize, z: usize) -> Self {
        Point3D { x, y, z }
    }

    /// 创建零点
    pub const fn zero() -> Self {
        Point3D { x: 0, y: 0, z: 0 }
    }

    /// 验证坐标是否在有效范围内
    pub const fn is_valid(&self) -> bool {
        self.x < 24 && self.y < 24 && self.z < 4
    }
}

/// 三维进制解释器
pub struct Radix3DInterpreter;

impl Radix3DInterpreter {
    /// 创建解释器
    pub const fn new() -> Self {
        Radix3DInterpreter
    }

    /// 编译时坐标查询: 根据声母/韵母/声调获取坐标
    pub const fn coordinate(sheng: &str, yun: &str, tone: &str) -> Option<Point3D> {
        let x = match Self::sheng_index(sheng) {
            Some(v) => v,
            None => return None,
        };
        let y = match Self::yun_index(yun) {
            Some(v) => v,
            None => return None,
        };
        let z = match Self::tone_index(tone) {
            Some(v) => v,
            None => return None,
        };
        Some(Point3D::new(x, y, z))
    }

    /// 编译时声母索引
    pub const fn sheng_index(sheng: &str) -> Option<usize> {
        let mut i = 0;
        while i < 24 {
            if const_str_eq(SHENG_MU[i], sheng) {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    /// 编译时韵母索引
    pub const fn yun_index(yun: &str) -> Option<usize> {
        let mut i = 0;
        while i < 24 {
            if const_str_eq(YUN_MU[i], yun) {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    /// 编译时声调索引
    pub const fn tone_index(tone: &str) -> Option<usize> {
        let mut i = 0;
        while i < 4 {
            if const_str_eq(TONES[i], tone) {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    /// 计算曼哈顿距离
    pub fn manhattan_distance(a: Point3D, b: Point3D) -> usize {
        let dx = if a.x > b.x { a.x - b.x } else { b.x - a.x };
        let dy = if a.y > b.y { a.y - b.y } else { b.y - a.y };
        let dz = if a.z > b.z { a.z - b.z } else { b.z - a.z };
        dx + dy + dz
    }

    /// 计算最大维距离
    pub fn max_dim_distance(a: Point3D, b: Point3D) -> usize {
        let dx = if a.x > b.x { a.x - b.x } else { b.x - a.x };
        let dy = if a.y > b.y { a.y - b.y } else { b.y - a.y };
        let dz = if a.z > b.z { a.z - b.z } else { b.z - a.z };
        let max = if dx > dy { dx } else { dy };
        if dz > max { dz } else { max }
    }

    /// 计算邻接关系（曼哈顿距离=1为严格邻接）
    pub fn are_adjacent(a: Point3D, b: Point3D) -> bool {
        Self::manhattan_distance(a, b) == 1
    }

    /// 查找指定坐标的所有严格邻接坐标
    pub fn neighbors_6(point: Point3D) -> [Option<Point3D>; 6] {
        let mut result = [None; 6];
        let mut count = 0;

        // 6个方向: ±x, ±y, ±z
        let deltas: [(isize, isize, isize); 6] = [
            (-1, 0, 0), (1, 0, 0),
            (0, -1, 0), (0, 1, 0),
            (0, 0, -1), (0, 0, 1),
        ];

        let mut i = 0;
        while i < 6 {
            let (dx, dy, dz) = deltas[i];
            let nx = point.x as isize + dx;
            let ny = point.y as isize + dy;
            let nz = point.z as isize + dz;

            if nx >= 0 && nx < 24 && ny >= 0 && ny < 24 && nz >= 0 && nz < 4 {
                result[count] = Some(Point3D::new(nx as usize, ny as usize, nz as usize));
                count += 1;
            }
            i += 1;
        }

        result
    }

    /// 查找指定坐标的所有宽松邻接坐标（曼哈顿距离≤2）
    pub fn neighbors_12(point: Point3D) -> [Option<Point3D>; 26] {
        let mut result = [None; 26];
        let mut count = 0;

        let mut dx = -1isize;
        while dx <= 1 {
            let mut dy = -1isize;
            while dy <= 1 {
                let mut dz = -1isize;
                while dz <= 1 {
                    if dx == 0 && dy == 0 && dz == 0 {
                        dz += 1;
                        continue;
                    }
                    let nx = point.x as isize + dx;
                    let ny = point.y as isize + dy;
                    let nz = point.z as isize + dz;

                    if nx >= 0 && nx < 24 && ny >= 0 && ny < 24 && nz >= 0 && nz < 4 {
                        let dist = (dx.abs() + dy.abs() + dz.abs()) as usize;
                        if dist <= 2 && count < 26 {
                            result[count] = Some(Point3D::new(nx as usize, ny as usize, nz as usize));
                            count += 1;
                        }
                    }
                    dz += 1;
                }
                dy += 1;
            }
            dx += 1;
        }

        result
    }

    /// 计算路径（逐维移动）
    pub fn path(from: Point3D, to: Point3D) -> [Option<Point3D>; 72] {
        let mut result = [None; 72];
        let mut count = 0;

        // 先沿 x 移动
        let mut cx = from.x as isize;
        let tx = to.x as isize;
        while cx != tx {
            if cx < tx { cx += 1; } else { cx -= 1; }
            if count < 72 {
                result[count] = Some(Point3D::new(cx as usize, from.y, from.z));
                count += 1;
            }
        }

        // 再沿 y 移动
        let mut cy = from.y as isize;
        let ty = to.y as isize;
        while cy != ty {
            if cy < ty { cy += 1; } else { cy -= 1; }
            if count < 72 {
                result[count] = Some(Point3D::new(cx as usize, cy as usize, from.z));
                count += 1;
            }
        }

        // 最后沿 z 移动
        let mut cz = from.z as isize;
        let tz = to.z as isize;
        while cz != tz {
            if cz < tz { cz += 1; } else { cz -= 1; }
            if count < 72 {
                result[count] = Some(Point3D::new(cx as usize, cy as usize, cz as usize));
                count += 1;
            }
        }

        result
    }

    /// 坐标转线性索引
    pub const fn to_linear(point: Point3D) -> usize {
        point.x * 24 * 4 + point.y * 4 + point.z
    }

    /// 线性索引转坐标
    pub const fn from_linear(index: usize) -> Point3D {
        Point3D {
            x: index / (24 * 4),
            y: (index / 4) % 24,
            z: index % 4,
        }
    }

    /// 获取声母名称
    pub const fn sheng_name(index: usize) -> &'static str {
        if index < 24 { SHENG_MU[index] } else { "" }
    }

    /// 获取韵母名称
    pub const fn yun_name(index: usize) -> &'static str {
        if index < 24 { YUN_MU[index] } else { "" }
    }

    /// 获取声调名称
    pub const fn tone_name(index: usize) -> &'static str {
        if index < 4 { TONES[index] } else { "" }
    }
}

/// 静态解释器实例
pub static DEFAULT_RADIX3D: Radix3DInterpreter = Radix3DInterpreter::new();
