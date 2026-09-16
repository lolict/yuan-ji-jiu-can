// Morton编码 — 三维坐标与一维地址互转
// Z-order curve for spatial indexing

use super::coordinate::Coord3D;

/// 将三维坐标编码为 Morton 码（Z序交织）
pub fn encode_morton3(x: u8, y: u16, z: u8) -> u32 {
    let mut result: u32 = 0;
    for i in 0..8 {
        result |= ((((x >> i) & 1) as u32) << (3 * i));
        result |= ((((z >> i) & 1) as u32) << (3 * i + 2));
    }
    for i in 0..16 {
        result |= ((((y >> i) & 1) as u32) << (3 * i + 1));
    }
    result & 0xFFFF_FFFF
}

/// Morton 码解码回三维坐标
pub fn decode_morton3(code: u32) -> Coord3D {
    let mut x: u8 = 0;
    let mut y: u16 = 0;
    let mut z: u8 = 0;
    for i in 0..8 {
        x |= (((code >> (3 * i)) & 1) as u8) << i;
        z |= (((code >> (3 * i + 2)) & 1) as u8) << i;
    }
    for i in 0..16 {
        y |= (((code >> (3 * i + 1)) & 1) as u16) << i;
    }
    Coord3D::new(x, y, z)
}

/// 三维坐标直接转 Morton 码
pub fn coord_to_morton(coord: &Coord3D) -> u32 {
    encode_morton3(coord.x, coord.y, coord.z)
}

/// Morton 码转三维坐标
pub fn morton_to_coord(code: u32) -> Coord3D {
    decode_morton3(code)
}
