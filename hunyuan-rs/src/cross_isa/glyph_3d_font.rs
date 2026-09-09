// 三维字体投影层 — 容器包 → 字形轮廓
//
// 编码方案（与约定对齐）:
//   xMax 两字节 = Y操作数 (直接存储，不缩放)
//   flags[0] = Z时间窗 (8位)
//   flags[1] = X语义ID (8位)
//
// 拓扑映射由 flags[2] 编码:
//   0x01-0x02 (LOAD/STORE): 0x10 盒型
//   0x03-0x08 (算逻运算): 0x20 球型
//   0x09-0x0A (JMP/BNE): 0x30 环面
//   0x0B-0x0C (CALL/RET): 0x40 管状
//   0x0D (CMP): 0x50 双锥
//   0x0E (MOV): 0x60 线型

use crate::cross_isa::coordinate::Coord3D;
use crate::cross_isa::fridge::ContainerPack;

/// 字形轮廓编码结果
#[derive(Debug, Clone)]
pub struct GlyphContour {
    pub x_min: i16,
    pub y_min: i16,
    pub x_max: i16,
    pub y_max: i16,
    pub end_pts: Vec<u16>,
    pub flags: Vec<u8>,
    pub x_coords: Vec<i16>,
    pub y_coords: Vec<i16>,
}

impl GlyphContour {
    /// 从三维坐标生成字形轮廓
    pub fn from_coord(coord: Coord3D) -> Self {
        let x_sem = coord.x;
        let y_op = coord.y;
        let z_time = coord.z;

        match x_sem {
            0x01 | 0x02 => Self::box_glyph(y_op, x_sem, z_time),
            0x03..=0x08 => Self::sphere_glyph(y_op, x_sem, z_time),
            0x09 | 0x0A | 0x0B => Self::torus_glyph(y_op, x_sem, z_time),
            0x0C | 0x0D => Self::double_cone_glyph(y_op, x_sem, z_time),
            0x0E | 0x0F => Self::line_glyph(y_op, x_sem, z_time),
            _ => Self::box_glyph(y_op, x_sem, z_time),
        }
    }

    /// 从容器包生成轮廓
    pub fn from_container_pack(pack: &ContainerPack) -> Self {
        Self::from_coord(pack.coord())
    }

    /// 从轮廓解码恢复坐标
    pub fn to_coord(&self) -> Coord3D {
        let y = self.x_max as u16;
        let z = self.flags[0];
        let x = self.flags[1];
        Coord3D::new(x, y, z)
    }

    // ── 各拓扑的字形生成 ────────────────────────────

    /// 盒型: 矩形, 4顶点
    fn box_glyph(y_op: u16, sem_id: u8, z_win: u8) -> Self {
        let w = y_op as i16;
        Self {
            x_min: 0, y_min: 0,
            x_max: w,
            y_max: 1024,
            end_pts: vec![3],
            flags: vec![z_win, sem_id, 0x10, 0x00],
            x_coords: vec![0, w, w, 0],
            y_coords: vec![0, 0, 1024, 1024],
        }
    }

    /// 球型: 圆形, 12顶点(4 on-curve + 8 off-curve)
    fn sphere_glyph(y_op: u16, sem_id: u8, z_win: u8) -> Self {
        let r = y_op as i16;
        Self {
            x_min: -r, y_min: -r,
            x_max: r,
            y_max: r,
            end_pts: vec![11],
            flags: vec![z_win, sem_id, 0x20,
                0x01, 0x01, 0x01, 0x01,
                0x00, 0x00, 0x00, 0x00,
                0x01, 0x01, 0x01, 0x01],
            x_coords: vec![-r, -r, 0, r, r, r, 0, -r, 0, 0, 0, 0],
            y_coords: vec![-r, 0, -r, 0, r, 0, r, 0, -r/2, r/2, 0, 0],
        }
    }

    /// 环面: 椭圆环, 12顶点
    fn torus_glyph(y_op: u16, sem_id: u8, z_win: u8) -> Self {
        let r = y_op as i16;
        Self {
            x_min: -r, y_min: -r/2,
            x_max: r, y_max: r/2,
            end_pts: vec![11],
            flags: vec![z_win, sem_id, 0x30,
                0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                0x01, 0x01, 0x01, 0x01, 0x01, 0x01],
            x_coords: vec![-r, -r/2, 0, r/2, r, r/2, 0, -r/2, -r, -r/2, 0, r/2],
            y_coords: vec![0, r/2, 0, -r/2, 0, r/2, 0, -r/2, 0, r/2, 0, -r/2],
        }
    }

    /// 双锥: 菱形, 8顶点
    fn double_cone_glyph(y_op: u16, sem_id: u8, z_win: u8) -> Self {
        let r = y_op as i16;
        Self {
            x_min: -r, y_min: -512,
            x_max: r, y_max: 512,
            end_pts: vec![7],
            flags: vec![z_win, sem_id, 0x50,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            x_coords: vec![0, r, 0, -r, 0, r, 0, -r],
            y_coords: vec![-512, 0, 512, 0, -512, 0, 512, 0],
        }
    }

    /// 线型: 折线, 4顶点
    fn line_glyph(y_op: u16, sem_id: u8, z_win: u8) -> Self {
        let w = if y_op < 4 { 1024 } else { y_op as i16 };
        Self {
            x_min: 0, y_min: 0,
            x_max: w, y_max: 256,
            end_pts: vec![3],
            flags: vec![z_win, sem_id, 0x60, 0x00, 0x00, 0x00],
            x_coords: vec![0, w/2, w, w/2],
            y_coords: vec![0, 256, 0, -256],
        }
    }
}

/// 语义ID → 拓扑名称
pub fn topology_name(sem_id: u8) -> &'static str {
    match sem_id {
        0x01 => "加载盒体 · 立方体",
        0x02 => "存储盒体 · 立方体",
        0x03 => "加法球体 · 球型",
        0x04 => "加急球体 · 球型",
        0x05 => "减法球体 · 球型",
        0x06 => "与门双锥 · 球型",
        0x07 => "或门双锥 · 球型",
        0x08 => "异或双锥 · 球型",
        0x09 => "跳转环面 · 环面",
        0x0A => "分支环面 · 环面",
        0x0B => "分支环面 · 环面",
        0x0C => "调用管体 · 双锥",
        0x0D => "返回管体 · 双锥",
        0x0E => "比较双锥 · 线型",
        0x0F => "加载球体 · 线型",
        _    => "未定义 · 保留",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contour_from_coord_roundtrip() {
        let coord = Coord3D::new(0x03, 0x0A10, 0x5F);
        let contour = GlyphContour::from_coord(coord);

        // 验证编码
        assert_eq!(contour.x_max as u16, 0x0A10);
        assert_eq!(contour.flags[0], 0x5F); // Z时间窗
        assert_eq!(contour.flags[1], 0x03); // X语义ID

        // 解码回坐标
        let decoded = contour.to_coord();
        assert_eq!(decoded.x, 0x03);
        assert_eq!(decoded.y, 0x0A10);
        assert_eq!(decoded.z, 0x5F);
    }

    #[test]
    fn contour_from_container_pack() {
        let coord = Coord3D::new(0x03, 0x0001, 0x01);
        let payload = vec![0x33, 0x10, 0x31, 0x00];
        let pack = ContainerPack::new(1, coord, 0, payload);
        let contour = GlyphContour::from_container_pack(&pack);
        assert_eq!(contour.x_max as u16, 0x0001);
    }

    #[test]
    fn box_topology_for_load() {
        let contour = GlyphContour::from_coord(Coord3D::new(0x01, 0x1000, 0x10));
        assert_eq!(contour.end_pts, vec![3]);
        assert_eq!(contour.x_coords.len(), 4);
        assert_eq!(contour.flags[2], 0x10); // 盒型标记
    }

    #[test]
    fn sphere_topology_for_add() {
        let contour = GlyphContour::from_coord(Coord3D::new(0x03, 0x2000, 0x20));
        assert_eq!(contour.x_coords.len(), 12);
        assert_eq!(contour.flags[2], 0x20); // 球型标记
    }

    #[test]
    fn torus_topology_for_jmp() {
        let contour = GlyphContour::from_coord(Coord3D::new(0x09, 0x3000, 0x30));
        assert_eq!(contour.x_coords.len(), 12);
        assert_eq!(contour.flags[2], 0x30);
    }

    #[test]
    fn line_topology_for_mov() {
        let contour = GlyphContour::from_coord(Coord3D::new(0x0E, 0x4000, 0x40));
        assert_eq!(contour.x_coords.len(), 4);
        assert_eq!(contour.flags[2], 0x60);
    }

    #[test]
    fn topology_names() {
        assert_eq!(topology_name(0x01), "加载盒体 · 立方体");
        assert_eq!(topology_name(0x03), "加法球体 · 球型");
        assert_eq!(topology_name(0x09), "跳转环面 · 环面");
    }
}
