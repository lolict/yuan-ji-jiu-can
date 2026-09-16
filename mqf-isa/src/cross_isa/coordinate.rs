// 三维坐标层 — [X:8][Y:16][Z:8] = 32位容器地址
// X = 语义ID  Y = 操作数编码  Z = 时间窗口/相位对齐

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord3D {
    pub x: u8,   // 语义ID
    pub y: u16,  // 操作数编码
    pub z: u8,   // 时间窗口
}

impl Coord3D {
    pub fn new(x: u8, y: u16, z: u8) -> Self {
        Self { x, y, z }
    }

    /// 编码为32位地址
    pub fn to_address(self) -> u32 {
        ((self.x as u32) << 24) | ((self.y as u32) << 8) | (self.z as u32)
    }

    /// 从32位地址解码
    pub fn from_address(addr: u32) -> Self {
        Self {
            x: ((addr >> 24) & 0xFF) as u8,
            y: ((addr >> 8) & 0xFFFF) as u16,
            z: (addr & 0xFF) as u8,
        }
    }

    /// Y轴解码：高8位=旋转角, 中4位=X缩放, 低4位=Y缩放
    pub fn y_rotation(&self) -> u8 { ((self.y >> 8) & 0xFF) as u8 }
    pub fn y_scale_x(&self) -> u8 { ((self.y >> 4) & 0x0F) as u8 }
    pub fn y_scale_y(&self) -> u8 { (self.y & 0x0F) as u8 }

    /// Z轴解码：高4位=颜色相位, 低4位=脉冲频率
    pub fn z_color_phase(&self) -> u8 { (self.z >> 4) & 0x0F }
    pub fn z_pulse_freq(&self) -> u8 { self.z & 0x0F }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_roundtrip() {
        let c = Coord3D::new(0x03, 0x0A10, 0x5F);
        let addr = c.to_address();
        let c2 = Coord3D::from_address(addr);
        assert_eq!(c, c2);
    }

    #[test]
    fn y_decoding() {
        let c = Coord3D::new(0x03, 0x0A10, 0x5F);
        assert_eq!(c.y_rotation(), 0x0A);
        assert_eq!(c.y_scale_x(), 0x1);
        assert_eq!(c.y_scale_y(), 0x0);
    }

    #[test]
    fn z_decoding() {
        let c = Coord3D::new(0x03, 0x0A10, 0x5F);
        assert_eq!(c.z_color_phase(), 0x5);
        assert_eq!(c.z_pulse_freq(), 0xF);
    }
}
