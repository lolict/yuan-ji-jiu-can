//! Dimension · 维度升降解释器
//! 高维⇄低维映射 · 投影升维

use crate::radix3d::{Point3D, Radix3DInterpreter};

/// 维度层
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dimension {
    Zero,  // 零维
    One,   // 一维
    Two,   // 二维
    Three, // 三维
    Four,  // 四维
}

/// 维度升降解释器
pub struct DimensionInterpreter;

impl DimensionInterpreter {
    /// 创建解释器
    pub const fn new() -> Self {
        DimensionInterpreter
    }

    /// 降维操作: 高维→低维映射
    pub const fn downshift(point: Point3D) -> usize {
        // 三维→零维: 所有坐标折叠为线性索引
        Radix3DInterpreter::to_linear(point)
    }

    /// 升维操作: 低维→高维投影
    pub const fn upshift(index: usize) -> Point3D {
        // 零维→三维: 线性索引展开为坐标
        Radix3DInterpreter::from_linear(index)
    }

    /// 部分降维: 三维→一维 (折叠 y,z)
    pub const fn partial_downshift_x(point: Point3D) -> usize {
        point.x
    }

    /// 部分降维: 三维→一维 (折叠 x,z)
    pub const fn partial_downshift_y(point: Point3D) -> usize {
        point.y
    }

    /// 部分降维: 三维→一维 (折叠 x,y)
    pub const fn partial_downshift_z(point: Point3D) -> usize {
        point.z
    }

    /// 一维升二维: (x) → (x, y)
    pub const fn upshift_1d_to_2d(index: usize, max_y: usize) -> (usize, usize) {
        (index / max_y, index % max_y)
    }

    /// 二维升三维: (x,y) → (x,y,z)
    pub const fn upshift_2d_to_3d(x: usize, y: usize, z: usize) -> Point3D {
        Point3D::new(x, y, z)
    }

    /// 完整降维: 三维→一维 (逐轴取最大维度)
    pub const fn full_downshift(point: Point3D) -> usize {
        point.x * 24 + point.y
    }

    /// 完整降维: 三维→零维 (单一线性索引)
    pub const fn full_downshift_zero(point: Point3D) -> usize {
        point.x * 24 * 4 + point.y * 4 + point.z
    }

    /// 逆操作: 一维→三维
    pub const fn inverse_downshift(index: usize) -> Point3D {
        Point3D {
            x: index / (24 * 4),
            y: (index / 4) % 24,
            z: index % 4,
        }
    }

    /// 投影降维: 三维→二维 (弃z)
    pub const fn project_to_2d(point: Point3D) -> (usize, usize) {
        (point.x, point.y)
    }

    /// 投影降维: 二维→一维 (弃y)
    pub const fn project_to_1d(x: usize, _y: usize) -> usize {
        x
    }

    /// 升维投影: 二维→三维 (补z=0)
    pub const fn expand_z(point_2d: (usize, usize)) -> Point3D {
        Point3D::new(point_2d.0, point_2d.1, 0)
    }

    /// 升维投影: 一维→二维 (补y=0)
    pub const fn expand_y(x: usize) -> (usize, usize) {
        (x, 0)
    }

    /// 计算维度容量
    pub const fn capacity(dim: Dimension) -> usize {
        match dim {
            Dimension::Zero => 0,
            Dimension::One => 24,
            Dimension::Two => 24 * 24,
            Dimension::Three => 24 * 24 * 4,
            Dimension::Four => 24 * 24 * 4 * 9,
        }
    }

    /// 判断坐标是否在指定维度容量内
    pub const fn fits_in(point: Point3D, dim: Dimension) -> bool {
        match dim {
            Dimension::Zero => true,
            Dimension::One => point.x < 24,
            Dimension::Two => point.x < 24 && point.y < 24,
            Dimension::Three => point.x < 24 && point.y < 24 && point.z < 4,
            Dimension::Four => point.x < 24 && point.y < 24 && point.z < 4,
        }
    }
}

/// 静态解释器实例
pub static DEFAULT_DIMENSION: DimensionInterpreter = DimensionInterpreter::new();
