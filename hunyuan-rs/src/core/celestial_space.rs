// ═══════════════════════════════════════════════════════════════════════════
// celestial_space.rs — 天庭空间 · 容器三维几何投影引擎 (v0.1)
// ═══════════════════════════════════════════════════════════════════════════
//
// 核心命题（满全法·囡囝共生 NJSG 空间哲学）：
//
//   "天庭"是容器在三维几何空间中的化身（embodiment）。
//   每个容器不再是一个抽象的数据结构，而是一个可感知的几何实体 —
//   有位置、有形态、有颜色、有脉动。
//
//   天庭 = 容器的可感知化身之集合。
//
// 空间几何映射：
//
//   语义 ID (x, 0x01-0x0F) → 极角 (angle) — 语义分布在天庭的圆周上
//   Z 轴 (时间窗口)        → 高度 (height) — 时间维度垂直展开
//   Y 轴 (操作数)          → 半径带 (radius band) — 操作数决定环位置
//   payload hash           → 大小 + 颜色相位 — 内容影响视觉属性
//
//   每个容器的几何形态由语义类别决定（与 ttf-view 五拓扑一致）：
//     LOAD/STORE   → 正方体 box  (memory, 8)
//     ALU          → 球体 sphere (alu, 4)
//     JMP/BNE/BEQ  → 环面 torus  (flow, 4)
//     CMP/CALL/RET → 双锥 bicone (judge, 4)
//     MOV          → 线段 line   (data, 4)
//
// 投影方式：等距圆柱投影（Equirectangular）→ 可输出 SVG 全景图
//
// ═══════════════════════════════════════════════════════════════════════════

use std::f64::consts::TAU;

use crate::cross_isa::coordinate::Coord3D;
use crate::cross_isa::fridge::ContainerPack;

// ───────────────────────────────────────────────────────────────────────────
// 天庭坐标 — 经过投影的三维空间位置
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CelestialPos {
    /// 水平角 [0, TAU) — 语义分布
    pub theta: f64,
    /// 垂直高度（z 维度）
    pub height: f64,
    /// 到天庭中心的距离
    pub radius: f64,
}

impl CelestialPos {
    pub fn new(theta: f64, height: f64, radius: f64) -> Self {
        Self { theta, height, radius }
    }

    /// 投影到 2D 平面（极坐标 → 笛卡尔）
    /// theta=0 → 正右方（天庭东方）；height=0 → 水平中线。
    pub fn project_2d(&self, cam_theta: f64, cam_height: f64) -> (f64, f64) {
        let theta_rel = self.theta - cam_theta;
        let eff_r = (self.radius + 1.0) * 50.0;  // 半径 2-5 → 150-300 px
        let x = theta_rel.cos() * eff_r;
        let y = (self.height - cam_height) - theta_rel.sin() * eff_r * 0.15;
        (x, y)
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 几何形态 — 容器在天庭中的显现形态
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GeometryForm {
    Box,      // LOAD / STORE
    Sphere,   // ALU (ADD, SUB, AND, OR, XOR)
    Torus,    // JMP, BNE, BEQ
    Bicone,   // CMP, CALL, RET
    Line,     // MOV
}

impl GeometryForm {
    pub fn from_sem_id(x: u8) -> Self {
        match x {
            0x01 | 0x02 => GeometryForm::Box,
            0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 => GeometryForm::Sphere,
            0x09 | 0x0A | 0x0B => GeometryForm::Torus,
            0x0C | 0x0D | 0x0E => GeometryForm::Bicone,
            0x0F => GeometryForm::Line,
            _ => GeometryForm::Sphere,
        }
    }

    pub fn svg_element(&self, cx: f64, cy: f64, size: f64, fill: &str, opacity: f64) -> String {
        match self {
            GeometryForm::Box => {
                let x = cx - size / 2.0;
                let y = cy - size / 2.0;
                format!(r#"<rect x="{x:.1}" y="{y:.1}" width="{size:.1}" height="{size:.1}" fill="{fill}" opacity="{opacity:.2}" rx="2"/>"#)
            }
            GeometryForm::Sphere => {
                let r = size / 2.0;
                format!(r#"<circle cx="{cx:.1}" cy="{cy:.1}" r="{r:.1}" fill="{fill}" opacity="{opacity:.2}"/>"#)
            }
            GeometryForm::Torus => {
                let r_outer = size / 2.0;
                let r_inner = size / 4.0;
                format!(r#"<circle cx="{cx:.1}" cy="{cy:.1}" r="{r_outer:.1}" fill="none" stroke="{fill}" stroke-width="{r_inner:.1}" opacity="{opacity:.2}"/>"#)
            }
            GeometryForm::Bicone => {
                let r = size / 2.0;
                let h = size * 0.7;
                let top = cy - h / 2.0;
                let bot = cy + h / 2.0;
                let left = cx - r;
                let right = cx + r;
                format!(r#"<polygon points="{cx:.1},{top:.1} {left:.1},{cy:.1} {cx:.1},{bot:.1} {right:.1},{cy:.1}" fill="{fill}" opacity="{opacity:.2}"/>"#)
            }
            GeometryForm::Line => {
                let x1 = cx - size / 2.0;
                let x2 = cx + size / 2.0;
                format!(r#"<line x1="{x1:.1}" y1="{cy:.1}" x2="{x2:.1}" y2="{cy:.1}" stroke="{fill}" stroke-width="2" opacity="{opacity:.2}"/>"#)
            }
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 天象参数 — 容器的视觉属性
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct CelestialBody {
    /// 容器包 ID
    pub pack_id: u64,
    /// 几何位置
    pub pos: CelestialPos,
    /// 形态
    pub form: GeometryForm,
    /// 颜色相位 (hue 0-360)
    pub hue: f64,
    /// 大小脉动 [0.5, 2.0]
    pub pulse: f64,
    /// 亮度 [0, 100]
    pub lightness: f64,
    /// 不透明度
    pub opacity: f64,
}

impl CelestialBody {
    pub fn from_pack(pack: &ContainerPack, cycle: u64, container_count: usize) -> Self {
        let coord = pack.coord();
        let cx = coord.x;
        let cz = coord.z;
        let payload_hash = hash_payload(&pack.payload);

        // 语义 → 极角（15 个语义环分布）
        let theta = (cx as f64 - 1.0) / 15.0 * TAU;

        // Z → 高度（时间层），归一化到 [-100, 100]
        let z_norm = (cz as f64) / 255.0;
        let height = (z_norm - 0.5) * 200.0;

        // Y 高 8 位 → 半径带位置 [2, 5]
        let y_hi = ((coord.y >> 8) & 0xFF) as f64;
        let radius = 2.0 + (y_hi / 255.0) * 3.0;

        // payload hash → 颜色相位
        let hue = ((payload_hash % 360) as f64 + cycle as f64 * 13.0) % 360.0;

        // hash → 大小脉动（周期性呼吸） [0.5, 1.5]
        let pulse_phase = ((payload_hash as f64 * 0.05) + (cycle as f64 * 0.3)) % TAU;
        let pulse = 0.7 + pulse_phase.sin() * 0.3 + 0.5;

        // Z → 亮度 [30, 70]
        let lightness = 30.0 + z_norm * 40.0;

        // 不透明度 [0.4, 1.0]
        let opacity = 0.4 + z_norm * 0.6;

        Self {
            pack_id: pack.pack_id,
            pos: CelestialPos::new(theta, height, radius),
            form: GeometryForm::from_sem_id(cx),
            hue,
            pulse,
            lightness,
            opacity,
        }
    }

    /// 在当前位置下的 SVG 渲染字符串
    pub fn render_svg(&self, cam_theta: f64, cam_height: f64, size_base: f64) -> String {
        let (px, py) = self.pos.project_2d(cam_theta, cam_height);
        let size = size_base * self.pulse;
        let fill = format!("hsl({:.0}, 70%, {:.0}%)", self.hue, self.lightness);
        self.form.svg_element(px, py, size, &fill, self.opacity)
    }
}

fn hash_payload(payload: &[u8]) -> u32 {
    let mut h: u32 = 0x811c9dc5;
    for b in payload {
        h ^= *b as u32;
        h = h.wrapping_mul(0x01000193);
    }
    h
}

// ───────────────────────────────────────────────────────────────────────────
// 天庭空间 — 全部容器的三维几何集合
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CelestialSpace {
    /// 当前天象体集合
    pub bodies: Vec<CelestialBody>,
    /// 演化周期序号
    pub cycle: u64,
    /// 相机的水平角
    pub camera_theta: f64,
    /// 相机的垂直高度
    pub camera_height: f64,
}

impl CelestialSpace {
    pub fn new(containers: &[ContainerPack], cycle: u64) -> Self {
        let n = containers.len().max(1);
        let bodies: Vec<CelestialBody> = containers
            .iter()
            .map(|pack| CelestialBody::from_pack(pack, cycle, n))
            .collect();
        Self {
            bodies,
            cycle,
            camera_theta: 0.0,
            camera_height: 0.0,
        }
    }

    /// 从 tick 演化更新：按 ContainerTimeline 的 tick 容器集重建天庭
    pub fn update(&mut self, containers: &[ContainerPack], cycle: u64) {
        let n = containers.len().max(1);
        self.bodies = containers
            .iter()
            .map(|pack| CelestialBody::from_pack(pack, cycle, n))
            .collect();
        self.cycle = cycle;
    }

    /// 设置相机位置
    pub fn set_camera(&mut self, theta: f64, height: f64) {
        self.camera_theta = theta;
        self.camera_height = height;
    }

    /// 渲染为 SVG
    pub fn render_svg(&self, width: u32, height: u32) -> String {
        let mut elements: Vec<String> = Vec::with_capacity(self.bodies.len());

        // 按 Y 分层排序（从底到顶），用于伪深度排序
        let mut sorted_indices: Vec<usize> = (0..self.bodies.len()).collect();
        sorted_indices.sort_by(|&a, &b| {
            let da = self.bodies[a].pos.radius;
            let db = self.bodies[b].pos.radius;
            da.partial_cmp(&db).unwrap()
        });

        for &idx in &sorted_indices {
            let body = &self.bodies[idx];
            elements.push(body.render_svg(self.camera_theta, self.camera_height, 12.0));
        }

        let cx = width as f64 / 2.0;
        let cy = height as f64 / 2.0;
        let elements_svg = elements.join("\n  ");

        format!(r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}" width="{w}" height="{h}">
  <rect width="100%" height="100%" fill="#0a0a1a"/>
  <g transform="translate({cx:.1},{cy:.1})">
    {elements}
  </g>
  <text x="10" y="{ty}" fill="#888" font-family="monospace" font-size="12">天庭空间 · cycle={cycle} · {n} 容器 · theta={theta}</text>
</svg>"##,
            w = width, h = height, cx = cx, cy = cy,
            elements = elements_svg,
            ty = height as f64 - 14.0,
            cycle = self.cycle, n = self.bodies.len(),
            theta = self.camera_theta,
        )
    }

    /// 生成演化动画 SVG 序列（每 cycle 一帧）
    pub fn render_evolution_frames(&self, container_snapshots: &[Vec<ContainerPack>], width: u32, height: u32) -> Vec<String> {
        container_snapshots.iter().enumerate().map(|(i, packs)| {
            let mut space = CelestialSpace::new(packs, i as u64);
            space.camera_theta = self.camera_theta;
            space.camera_height = self.camera_height;
            space.render_svg(width, height)
        }).collect()
    }

    /// 统计天庭状态
    pub fn stats(&self) -> String {
        let form_counts = {
            let mut m = [0u32; 5];
            for b in &self.bodies {
                match b.form {
                    GeometryForm::Box => m[0] += 1,
                    GeometryForm::Sphere => m[1] += 1,
                    GeometryForm::Torus => m[2] += 1,
                    GeometryForm::Bicone => m[3] += 1,
                    GeometryForm::Line => m[4] += 1,
                }
            }
            m
        };
        format!(
            "CelestialSpace[cycle={}, bodies={}, box={}, sphere={}, torus={}, bicone={}, line={}]",
            self.cycle, self.bodies.len(),
            form_counts[0], form_counts[1], form_counts[2], form_counts[3], form_counts[4],
        )
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 测试
// ───────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_packs(n: usize) -> Vec<ContainerPack> {
        (0..n).map(|i| {
            let x = (i % 15 + 1) as u8;
            let y = (0x1000 + i as u16) as u16;
            let z = (i % 4 + 1) as u8;
            ContainerPack::new(i as u64, Coord3D::new(x, y, z), 0, vec![0xAA, 0xBB, 0xCC, 0xDD])
        }).collect()
    }

    #[test]
    fn celestial_space_new_creates_bodies() {
        let packs = mock_packs(10);
        let space = CelestialSpace::new(&packs, 0);
        assert_eq!(space.bodies.len(), 10);
        assert_eq!(space.cycle, 0);
    }

    #[test]
    fn celestial_pos_project_2d() {
        let pos = CelestialPos::new(0.0, 0.0, 3.0);
        let (x, y) = pos.project_2d(0.0, 0.0);
        // theta=0, radius=3 → eff_r=200, cos(0)=1 → x ≈ 200
        assert!((x - 200.0).abs() < 1.0, "x={}", x);
        assert!((y - 0.0).abs() < 1.0, "y={}", y);
    }

    #[test]
    fn celestial_body_form_mapping() {
        // LOAD → Box
        let p = ContainerPack::new(0, Coord3D::new(0x01, 0, 1), 0, vec![1, 2, 3, 4]);
        let body = CelestialBody::from_pack(&p, 0, 1);
        assert_eq!(body.form, GeometryForm::Box);

        // ALU → Sphere
        let p = ContainerPack::new(1, Coord3D::new(0x03, 0, 1), 0, vec![1, 2, 3, 4]);
        let body = CelestialBody::from_pack(&p, 0, 1);
        assert_eq!(body.form, GeometryForm::Sphere);

        // MOV → Line
        let p = ContainerPack::new(2, Coord3D::new(0x0F, 0, 1), 0, vec![1, 2, 3, 4]);
        let body = CelestialBody::from_pack(&p, 0, 1);
        assert_eq!(body.form, GeometryForm::Line);
    }

    #[test]
    fn celestial_render_svg_contains_elements() {
        let packs = mock_packs(5);
        let space = CelestialSpace::new(&packs, 0);
        let svg = space.render_svg(400, 300);
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("天庭空间"));
        // 应该有 5 个形状
        let shape_count = svg.matches("rect").count()
            + svg.matches("circle").count()
            + svg.matches("polygon").count()
            + svg.matches("line ").count();
        assert!(shape_count >= 5, "expected at least 5 shapes, got {}", shape_count);
    }

    #[test]
    fn celestial_space_update_changes_cycle() {
        let packs = mock_packs(8);
        let mut space = CelestialSpace::new(&packs, 0);
        assert_eq!(space.cycle, 0);

        space.update(&packs, 5);
        assert_eq!(space.cycle, 5);
        assert_eq!(space.bodies.len(), 8);
    }

    #[test]
    fn celestial_geometry_svg_elements() {
        // Box (LOAD/STORE)
        let svg = GeometryForm::Box.svg_element(10.0, 10.0, 8.0, "#fff", 0.9);
        assert!(svg.contains("rect"));

        // Sphere (ALU)
        let svg = GeometryForm::Sphere.svg_element(10.0, 10.0, 8.0, "#fff", 0.9);
        assert!(svg.contains("circle"));

        // Torus (JMP)
        let svg = GeometryForm::Torus.svg_element(10.0, 10.0, 8.0, "#fff", 0.9);
        assert!(svg.contains("circle"));
        assert!(svg.contains("stroke"));

        // Bicone (CMP)
        let svg = GeometryForm::Bicone.svg_element(10.0, 10.0, 8.0, "#fff", 0.9);
        assert!(svg.contains("polygon"));

        // Line (MOV)
        let svg = GeometryForm::Line.svg_element(10.0, 10.0, 8.0, "#fff", 0.9);
        assert!(svg.contains("line"));
    }

    #[cfg(test)]
    fn celestial_space_camera_perspective() {
        let packs = mock_packs(3);
        let mut space = CelestialSpace::new(&packs, 0);
        space.set_camera(1.0, 50.0);

        // After moving camera, positions should reflect new perspective
        let body = &space.bodies[0];
        let (x, y) = body.pos.project_2d(space.camera_theta, space.camera_height);
        // Just ensure no NaN
        assert!(!x.is_nan());
        assert!(!y.is_nan());
    }

    #[test]
    fn celestial_stats_format() {
        let packs = mock_packs(15);
        let space = CelestialSpace::new(&packs, 3);
        let s = space.stats();
        assert!(s.contains("cycle=3"));
        assert!(s.contains("bodies=15"));
    }
}
