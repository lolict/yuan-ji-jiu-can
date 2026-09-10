// ═══════════════════════════════════════════════════════════════════════════
// cosmos_view.rs — 混元全景视图 · 全子系统统一 SVG 仪表盘 (v0.1)
// ═══════════════════════════════════════════════════════════════════════════
//
// 核心命题：
//
//   "一帧见全貌" — 所有子系统在同一张 SVG 可视化中共同呈现：
//
//   ① ContainerTimeline   — 时间演化层（tick 驱动）
//   ② CelestialSpace      — 三维几何投影层（容器天象体）
//   ③ FiberBundleTopo     — 纤维关系拓扑层（容器间连边）
//   ④ PerceptionField     — 全局感知统计层（语义/空间/时序直方图）
//
//   这四个层次叠加在一张图上 — 斑马的皮（外观）、斑马的骨（结构）、
//   斑马的脉（关系）、斑马的气（演化）。
//
// ═══════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;

use crate::celestial_space::{CelestialBody, CelestialSpace, GeometryForm};
use crate::container_timeline::ContainerTimeline;
use crate::cross_isa::coordinate::Coord3D;
use crate::cross_isa::fridge::ContainerPack;
use crate::fiber_bundle_topo::{FiberBundleTopo, FiberDir};

// ───────────────────────────────────────────────────────────────────────────
// 全景仪表盘 — 单帧 SVG 渲染
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct CosmosFrame {
    pub tick: u64,
    pub space: CelestialSpace,
    pub topo: FiberBundleTopo,
    pub intentions_observed: usize,
    pub dominant_sem: Vec<(u8, usize)>,
    pub dir_hist: HashMap<FiberDir, usize>,
}

impl CosmosFrame {
    /// 从 ContainerTimeline 的当前状态生成一帧
    pub fn from_timeline(tl: &ContainerTimeline) -> Self {
        let space = CelestialSpace::new(&tl.containers, tl.tick);
        let topo = FiberBundleTopo::build(&tl.containers);
        let dir_hist = topo.dir_histogram();

        let mut sem_dist: HashMap<u8, usize> = HashMap::new();
        for pack in &tl.containers {
            *sem_dist.entry(pack.coord().x).or_insert(0) += 1;
        }
        let mut dominant: Vec<(u8, usize)> = sem_dist.iter().map(|(&k, &v)| (k, v)).collect();
        dominant.sort_by(|a, b| b.1.cmp(&a.1));

        Self {
            tick: tl.tick,
            space,
            topo,
            intentions_observed: tl.history.total_observations(),
            dominant_sem: dominant,
            dir_hist: dir_hist,
        }
    }

    /// 渲染为 SVG 全景仪表盘
    pub fn render_svg(&self, width: u32, height: u32) -> String {
        let cx = width as f64 / 2.0;
        let cy = height as f64 / 2.0;
        let r_main = (width.min(height) as f64) * 0.36;

        // 位置分配：按 golden angle 分布在圆周上
        let ids: Vec<u64> = self.space.bodies.iter().map(|b| b.pack_id).collect();
        let n = ids.len().max(1) as f64;
        let tau = std::f64::consts::TAU;
        let mut positions: HashMap<u64, (f64, f64)> = HashMap::new();
        for (i, id) in ids.iter().enumerate() {
            let angle = (*id as f64 * 1.618033988749895) % tau;
            positions.insert(*id, (cx + angle.cos() * r_main, cy + angle.sin() * r_main));
        }

        // 颜色映射
        let color_for = |dir: FiberDir| -> &str {
            match dir {
                FiberDir::Belong => "#4CAF50",
                FiberDir::Invoke => "#2196F3",
                FiberDir::Witness => "#FFC107",
                FiberDir::Opposite => "#F44336",
                FiberDir::Parallel => "#9C27B0",
            }
        };

        let mut svg = String::new();

        // 背景
        svg.push_str(&format!(
            r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}" width="{w}" height="{h}">
<rect width="100%" height="100%" fill="#050510"/>
<defs><radialGradient id="g1" cx="50%" cy="50%" r="50%">
<stop offset="0%" stop-color="#1a1a3a"/><stop offset="100%" stop-color="#050510"/>
</radialGradient></defs>
<rect width="100%" height="100%" fill="url(#g1)" opacity="0.6"/>
"##,
            w = width, h = height,
        ));

        // 中心圆（意识中心 kenar）
        svg.push_str(&format!(
            r##"<circle cx="{cx:.0}" cy="{cy:.0}" r="6" fill="none" stroke="#fff" stroke-width="1" opacity="0.7"/>
<circle cx="{cx:.0}" cy="{cy:.0}" r="2" fill="#fff" opacity="0.9"/>
"##,
            cx = cx, cy = cy,
        ));

        // 纤维边（按方向分类）
        for edge in &self.topo.edges {
            if let (Some(&(x1, y1)), Some(&(x2, y2))) = (positions.get(&edge.from), positions.get(&edge.to)) {
                let c = color_for(edge.dir);
                let sw = (edge.weight as f64) * 0.2;
                let op = match edge.dir {
                    FiberDir::Witness => 0.25,
                    FiberDir::Parallel => 0.35,
                    FiberDir::Belong => 0.5,
                    FiberDir::Invoke => 0.55,
                    FiberDir::Opposite => 0.45,
                };
                svg.push_str(&format!(
                    r##"<line x1="{x1:.1}" y1="{y1:.1}" x2="{x2:.1}" y2="{y2:.1}" stroke="{c}" stroke-width="{sw:.1}" opacity="{op:.2}"/>
"##
                ));
            }
        }

        // 几何体（CelestialBody）
        for body in &self.space.bodies {
            if let Some(&(x, y)) = positions.get(&body.pack_id) {
                let fill = format!("hsl({:.0}, 60%, {:.0}%)", body.hue, body.lightness);
                let size = 8.0 * body.pulse;
                let x2 = x - size / 2.0;
                let y2 = y - size / 2.0;
                let r = size / 2.0;
                let sw = size / 5.0;
                let op = body.opacity;
                let mut shape_svg = String::new();
                match body.form {
                    GeometryForm::Box => {
                        shape_svg = format!(
                            r##"<rect x="{x2:.1}" y="{y2:.1}" width="{s:.1}" height="{s:.1}" fill="{fill}" opacity="{op:.2}" transform="rotate(45 {x:.1} {y:.1})"/>"##,
                            x2 = x2, y2 = y2, s = size, x = x, y = y, fill = fill, op = op,
                        );
                    }
                    GeometryForm::Sphere => {
                        shape_svg = format!(
                            r##"<circle cx="{x:.1}" cy="{y:.1}" r="{r:.1}" fill="{fill}" opacity="{op:.2}"/>"##,
                            x = x, y = y, r = r, fill = fill, op = op,
                        );
                    }
                    GeometryForm::Torus => {
                        shape_svg = format!(
                            r##"<circle cx="{x:.1}" cy="{y:.1}" r="{r:.1}" fill="none" stroke="{fill}" stroke-width="{sw:.1}" opacity="{op:.2}"/>"##,
                            x = x, y = y, r = r, sw = sw, fill = fill, op = op,
                        );
                    }
                    GeometryForm::Bicone => {
                        let half = size / 2.0;
                        let h_half = size * 0.35;
                        let top = y - h_half;
                        let bot = y + h_half;
                        let left = x - half;
                        let right = x + half;
                        shape_svg = format!(
                            r##"<polygon points="{x:.1},{top:.1} {left:.1},{y:.1} {x:.1},{bot:.1} {right:.1},{y:.1}" fill="{fill}" opacity="{op:.2}"/>"##,
                            x = x, y = y, top = top, bot = bot, left = left, right = right, fill = fill, op = op,
                        );
                    }
                    GeometryForm::Line => {
                        let x1 = x - size / 2.0;
                        let x2l = x + size / 2.0;
                        shape_svg = format!(
                            r##"<line x1="{x1:.1}" y1="{y:.1}" x2="{x2l:.1}" y2="{y:.1}" stroke="{fill}" stroke-width="2" opacity="{op:.2}"/>"##,
                            x1 = x1, y = y, x2l = x2l, fill = fill, op = op,
                        );
                    }
                };
                svg.push_str(&shape_svg);
            }
        }

        // HUD
        let cont_count = self.space.bodies.len();
        let edges_count = self.topo.edges.len();
        let components = self.topo.components.len();
        svg.push_str(&format!(
            r##"<text x="20" y="25" fill="#eef" font-family="monospace" font-size="14" font-weight="bold">mix-universe · tick={tl} · {c}containers · {e}edges · {cm}groups</text>
<text x="20" y="45" fill="#aab" font-family="monospace" font-size="10">intentions_observed={i}</text>
"##,
            tl = self.tick, c = cont_count, e = edges_count, cm = components, i = self.intentions_observed,
        ));

        let hist_x = width as f64 - 160.0;
        let mut hist_y: f64 = 30.0;
        svg.push_str(&format!(
            r##"<text x="{hx:.0}" y="{hy:.0}" fill="#ccc" font-family="monospace" font-size="10">semantics</text>
"##,
            hx = hist_x, hy = hist_y,
        ));
        hist_y += 15.0;
        for (x, count) in self.dominant_sem.iter().take(8) {
            svg.push_str(&format!(
                r##"<text x="{hx:.0}" y="{hy:.0}" fill="#9ae" font-family="monospace" font-size="9">0x{x:02X} {c}</text>
"##,
                hx = hist_x, hy = hist_y, x = x, c = count,
            ));
            hist_y += 12.0;
        }

        let dir_x = width as f64 - 160.0;
        let mut dir_y = height as f64 - 120.0;
        svg.push_str(&format!(
            r##"<text x="{dx:.0}" y="{dy:.0}" fill="#ccc" font-family="monospace" font-size="10">fiber_dirs</text>
"##,
            dx = dir_x, dy = dir_y,
        ));
        dir_y += 15.0;
        for (dir, label, glyph) in [
            (FiberDir::Belong, "belong", "&#x2283;"),
            (FiberDir::Invoke, "invoke", "&#x2192;"),
            (FiberDir::Witness, "witness", "&#x25C7;"),
            (FiberDir::Opposite, "opposite", "&#x2194;"),
            (FiberDir::Parallel, "parallel", "&#x2225;"),
        ] {
            let count = self.dir_hist.get(&dir).unwrap_or(&0);
            let c = color_for(dir);
            svg.push_str(&format!(
                r##"<text x="{dx:.0}" y="{dy:.0}" fill="{c}" font-family="monospace" font-size="9">{glyph} {label} {n}</text>
"##,
                dx = dir_x, dy = dir_y, c = c, glyph = glyph, label = label, n = count,
            ));
            dir_y += 12.0;
        }

        svg.push_str("\n</svg>");
        svg
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 测试
// ───────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::container_timeline::{ActionKind, EvolutionConfig};

    fn mock_packs(n: usize) -> Vec<ContainerPack> {
        (0..n).map(|i| {
            let x = (i % 15 + 1) as u8;
            let y = (0x1000 + i as u16) as u16;
            let z = (i % 4 + 1) as u8;
            ContainerPack::new(i as u64, Coord3D::new(x, y, z), 0, vec![0xAA, 0xBB, 0xCC, 0xDD])
        }).collect()
    }

    #[test]
    fn cosmos_frame_builds_from_timeline() {
        let packs = mock_packs(10);
        let tl = ContainerTimeline::new(packs);
        let frame = CosmosFrame::from_timeline(&tl);
        assert_eq!(frame.tick, 0);
        assert_eq!(frame.space.bodies.len(), 10);
        assert!(frame.topo.nodes.len() >= 10);
    }

    #[test]
    fn cosmos_frame_svg_contains_all_layers() {
        let packs = mock_packs(8);
        let tl = ContainerTimeline::new(packs);
        let frame = CosmosFrame::from_timeline(&tl);
        let svg = frame.render_svg(800, 600);
        assert!(svg.contains("mix-universe"));
        assert!(svg.contains("semantics"));
        assert!(svg.contains("fiber_dirs"));
        assert!(svg.contains("witness"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn cosmos_frame_svg_contains_shapes() {
        let packs = mock_packs(6);
        let tl = ContainerTimeline::new(packs);
        let frame = CosmosFrame::from_timeline(&tl);
        let svg = frame.render_svg(600, 400);
        // 至少有一种几何元素
        let shapes = svg.matches("<circle").count()
            + svg.matches("<rect").count()
            + svg.matches("<polygon").count()
            + svg.matches("<line").count();
        assert!(shapes >= 10, "expected shape elements, got {}", shapes);
    }

    #[test]
    fn cosmos_after_evolution_changes_tick() {
        let packs = mock_packs(8);
        let config = EvolutionConfig {
            action_pipeline: vec![ActionKind::Observe, ActionKind::Substitute, ActionKind::Create],
            base_seed: 42,
            max_ticks: 10,
        };
        let mut tl = ContainerTimeline::with_config(packs, config);
        tl.advance();
        let frame = CosmosFrame::from_timeline(&tl);
        assert_eq!(frame.tick, 1);
    }

    #[test]
    fn cosmos_dominant_sem_sorted() {
        let packs = mock_packs(20);
        let tl = ContainerTimeline::new(packs);
        let frame = CosmosFrame::from_timeline(&tl);
        // 主导语义应按频次降序
        for w in frame.dominant_sem.windows(2) {
            assert!(w[0].1 >= w[1].1);
        }
    }
}
