// 自动生成: hunyuan-os/tools/glyph-container/generate_full_font.py
// 包含全部 64 容器的 coord round-trip 测试
#[cfg(test)]
mod container64_roundtrip {
    use super::*;

    #[test]
    fn container_00_roundtrip() {
        let coord = Coord3D::new(0, 0x0000, 0x00);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x00 (box) coord round-trip fail");
    }

    #[test]
    fn container_01_roundtrip() {
        let coord = Coord3D::new(1, 0x0100, 0x01);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x01 (box) coord round-trip fail");
    }

    #[test]
    fn container_02_roundtrip() {
        let coord = Coord3D::new(2, 0x0200, 0x02);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x02 (double_cone) coord round-trip fail");
    }

    #[test]
    fn container_03_roundtrip() {
        let coord = Coord3D::new(3, 0x0300, 0x03);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x03 (sphere) coord round-trip fail");
    }

    #[test]
    fn container_04_roundtrip() {
        let coord = Coord3D::new(4, 0x0400, 0x04);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x04 (sphere) coord round-trip fail");
    }

    #[test]
    fn container_05_roundtrip() {
        let coord = Coord3D::new(5, 0x0500, 0x05);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x05 (torus) coord round-trip fail");
    }

    #[test]
    fn container_06_roundtrip() {
        let coord = Coord3D::new(6, 0x0600, 0x06);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x06 (line) coord round-trip fail");
    }

    #[test]
    fn container_07_roundtrip() {
        let coord = Coord3D::new(7, 0x0700, 0x07);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x07 (double_cone) coord round-trip fail");
    }

    #[test]
    fn container_08_roundtrip() {
        let coord = Coord3D::new(8, 0x0800, 0x08);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x08 (sphere) coord round-trip fail");
    }

    #[test]
    fn container_09_roundtrip() {
        let coord = Coord3D::new(9, 0x0900, 0x09);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x09 (sphere) coord round-trip fail");
    }

    #[test]
    fn container_0a_roundtrip() {
        let coord = Coord3D::new(10, 0x0A00, 0x0A);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x0A (torus) coord round-trip fail");
    }

    #[test]
    fn container_0b_roundtrip() {
        let coord = Coord3D::new(11, 0x0B00, 0x0B);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x0B (torus) coord round-trip fail");
    }

    #[test]
    fn container_0c_roundtrip() {
        let coord = Coord3D::new(12, 0x0C00, 0x0C);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x0C (box) coord round-trip fail");
    }

    #[test]
    fn container_0d_roundtrip() {
        let coord = Coord3D::new(13, 0x0D00, 0x0D);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x0D (sphere) coord round-trip fail");
    }

    #[test]
    fn container_0e_roundtrip() {
        let coord = Coord3D::new(14, 0x0E00, 0x0E);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x0E (torus) coord round-trip fail");
    }

    #[test]
    fn container_0f_roundtrip() {
        let coord = Coord3D::new(15, 0x0F00, 0x0F);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x0F (line) coord round-trip fail");
    }

    #[test]
    fn container_10_roundtrip() {
        let coord = Coord3D::new(16, 0x0000, 0x10);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x10 (torus) coord round-trip fail");
    }

    #[test]
    fn container_11_roundtrip() {
        let coord = Coord3D::new(17, 0x0100, 0x11);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x11 (sphere) coord round-trip fail");
    }

    #[test]
    fn container_12_roundtrip() {
        let coord = Coord3D::new(18, 0x0200, 0x12);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x12 (torus) coord round-trip fail");
    }

    #[test]
    fn container_13_roundtrip() {
        let coord = Coord3D::new(19, 0x0300, 0x13);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x13 (sphere) coord round-trip fail");
    }

    #[test]
    fn container_14_roundtrip() {
        let coord = Coord3D::new(20, 0x0400, 0x14);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x14 (box) coord round-trip fail");
    }

    #[test]
    fn container_15_roundtrip() {
        let coord = Coord3D::new(21, 0x0500, 0x15);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x15 (sphere) coord round-trip fail");
    }

    #[test]
    fn container_16_roundtrip() {
        let coord = Coord3D::new(22, 0x0600, 0x16);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x16 (torus) coord round-trip fail");
    }

    #[test]
    fn container_17_roundtrip() {
        let coord = Coord3D::new(23, 0x0700, 0x17);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x17 (line) coord round-trip fail");
    }

    #[test]
    fn container_18_roundtrip() {
        let coord = Coord3D::new(24, 0x0800, 0x18);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x18 (double_cone) coord round-trip fail");
    }

    #[test]
    fn container_19_roundtrip() {
        let coord = Coord3D::new(25, 0x0900, 0x19);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x19 (sphere) coord round-trip fail");
    }

    #[test]
    fn container_1a_roundtrip() {
        let coord = Coord3D::new(26, 0x0A00, 0x1A);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x1A (torus) coord round-trip fail");
    }

    #[test]
    fn container_1b_roundtrip() {
        let coord = Coord3D::new(27, 0x0B00, 0x1B);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x1B (double_cone) coord round-trip fail");
    }

    #[test]
    fn container_1c_roundtrip() {
        let coord = Coord3D::new(28, 0x0C00, 0x1C);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x1C (line) coord round-trip fail");
    }

    #[test]
    fn container_1d_roundtrip() {
        let coord = Coord3D::new(29, 0x0D00, 0x1D);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x1D (line) coord round-trip fail");
    }

    #[test]
    fn container_1e_roundtrip() {
        let coord = Coord3D::new(30, 0x0E00, 0x1E);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x1E (torus) coord round-trip fail");
    }

    #[test]
    fn container_1f_roundtrip() {
        let coord = Coord3D::new(31, 0x0F00, 0x1F);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x1F (box) coord round-trip fail");
    }

    #[test]
    fn container_20_roundtrip() {
        let coord = Coord3D::new(32, 0x0000, 0x20);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x20 (sphere) coord round-trip fail");
    }

    #[test]
    fn container_21_roundtrip() {
        let coord = Coord3D::new(33, 0x0100, 0x21);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x21 (sphere) coord round-trip fail");
    }

    #[test]
    fn container_22_roundtrip() {
        let coord = Coord3D::new(34, 0x0200, 0x22);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x22 (sphere) coord round-trip fail");
    }

    #[test]
    fn container_23_roundtrip() {
        let coord = Coord3D::new(35, 0x0300, 0x23);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x23 (sphere) coord round-trip fail");
    }

    #[test]
    fn container_24_roundtrip() {
        let coord = Coord3D::new(36, 0x0400, 0x24);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x24 (sphere) coord round-trip fail");
    }

    #[test]
    fn container_25_roundtrip() {
        let coord = Coord3D::new(37, 0x0500, 0x25);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x25 (sphere) coord round-trip fail");
    }

    #[test]
    fn container_26_roundtrip() {
        let coord = Coord3D::new(38, 0x0600, 0x26);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x26 (sphere) coord round-trip fail");
    }

    #[test]
    fn container_27_roundtrip() {
        let coord = Coord3D::new(39, 0x0700, 0x27);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x27 (sphere) coord round-trip fail");
    }

    #[test]
    fn container_28_roundtrip() {
        let coord = Coord3D::new(40, 0x0800, 0x28);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x28 (sphere) coord round-trip fail");
    }

    #[test]
    fn container_29_roundtrip() {
        let coord = Coord3D::new(41, 0x0900, 0x29);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x29 (sphere) coord round-trip fail");
    }

    #[test]
    fn container_2a_roundtrip() {
        let coord = Coord3D::new(42, 0x0A00, 0x2A);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x2A (line) coord round-trip fail");
    }

    #[test]
    fn container_2b_roundtrip() {
        let coord = Coord3D::new(43, 0x0B00, 0x2B);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x2B (box) coord round-trip fail");
    }

    #[test]
    fn container_2c_roundtrip() {
        let coord = Coord3D::new(44, 0x0C00, 0x2C);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x2C (line) coord round-trip fail");
    }

    #[test]
    fn container_2d_roundtrip() {
        let coord = Coord3D::new(45, 0x0D00, 0x2D);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x2D (box) coord round-trip fail");
    }

    #[test]
    fn container_2e_roundtrip() {
        let coord = Coord3D::new(46, 0x0E00, 0x2E);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x2E (sphere) coord round-trip fail");
    }

    #[test]
    fn container_2f_roundtrip() {
        let coord = Coord3D::new(47, 0x0F00, 0x2F);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x2F (line) coord round-trip fail");
    }

    #[test]
    fn container_30_roundtrip() {
        let coord = Coord3D::new(48, 0x0000, 0x30);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x30 (torus) coord round-trip fail");
    }

    #[test]
    fn container_31_roundtrip() {
        let coord = Coord3D::new(49, 0x0100, 0x31);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x31 (torus) coord round-trip fail");
    }

    #[test]
    fn container_32_roundtrip() {
        let coord = Coord3D::new(50, 0x0200, 0x32);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x32 (torus) coord round-trip fail");
    }

    #[test]
    fn container_33_roundtrip() {
        let coord = Coord3D::new(51, 0x0300, 0x33);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x33 (torus) coord round-trip fail");
    }

    #[test]
    fn container_34_roundtrip() {
        let coord = Coord3D::new(52, 0x0400, 0x34);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x34 (torus) coord round-trip fail");
    }

    #[test]
    fn container_35_roundtrip() {
        let coord = Coord3D::new(53, 0x0500, 0x35);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x35 (torus) coord round-trip fail");
    }

    #[test]
    fn container_36_roundtrip() {
        let coord = Coord3D::new(54, 0x0600, 0x36);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x36 (sphere) coord round-trip fail");
    }

    #[test]
    fn container_37_roundtrip() {
        let coord = Coord3D::new(55, 0x0700, 0x37);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x37 (torus) coord round-trip fail");
    }

    #[test]
    fn container_38_roundtrip() {
        let coord = Coord3D::new(56, 0x0800, 0x38);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x38 (line) coord round-trip fail");
    }

    #[test]
    fn container_39_roundtrip() {
        let coord = Coord3D::new(57, 0x0900, 0x39);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x39 (line) coord round-trip fail");
    }

    #[test]
    fn container_3a_roundtrip() {
        let coord = Coord3D::new(58, 0x0A00, 0x3A);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x3A (sphere) coord round-trip fail");
    }

    #[test]
    fn container_3b_roundtrip() {
        let coord = Coord3D::new(59, 0x0B00, 0x3B);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x3B (box) coord round-trip fail");
    }

    #[test]
    fn container_3c_roundtrip() {
        let coord = Coord3D::new(60, 0x0C00, 0x3C);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x3C (sphere) coord round-trip fail");
    }

    #[test]
    fn container_3d_roundtrip() {
        let coord = Coord3D::new(61, 0x0D00, 0x3D);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x3D (sphere) coord round-trip fail");
    }

    #[test]
    fn container_3e_roundtrip() {
        let coord = Coord3D::new(62, 0x0E00, 0x3E);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x3E (sphere) coord round-trip fail");
    }

    #[test]
    fn container_3f_roundtrip() {
        let coord = Coord3D::new(63, 0x0F00, 0x3F);
        let contour = GlyphContour::from_coord(coord);
        let restored = contour.to_coord();
        assert_eq!(coord, restored, "容器0x3F (line) coord round-trip fail");
    }

}
