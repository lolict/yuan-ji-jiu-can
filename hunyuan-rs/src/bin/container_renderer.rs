// 容器原生渲染引擎 — 容器语言解释投射到 HTML
//
// 用法: ./container_renderer [场景文件]
// 场景文件: JSON 数组, 每个元素 {code, x?, y?, z?, role?, link?}
//
// 渲染规则完全由容器码决定, 不硬编码:
//   - 族ID决定色系 (运算=蓝,满全=紫,声母=绿,韵母=橙)
//   - 拓扑码决定形状 (盒/球/环/锥/线)
//   - 声母韵母关系决定连线规则
//   - 64进制坐标空间 (Morton/Z-order → 屏幕xy)
//
// 编译: rustc --edition 2018 -O src/bin/container_renderer.rs -o container_renderer

use std::env;
use std::fmt::Write;

const NAMES: [&str; 64] = [
    "元","加","减","与","或","非","异或","传送",
    "跳转","调用","返回","比较","加载","存储","移位","置位",
    "满","全","法","囡","丱","共","生","伴",
    "侣","赔","偿","合","离","聚","散","归",
    "帮","滂","并","明","端","透","定","泥",
    "知","彻","澄","娘","见","溪","群","疑",
    "东","冬","钟","江","支","脂","之","微",
    "鱼","虞","模","齐","佳","皆","灰","咍",
];

// ═══════════════════════════════════════════════════════════════════════════
// 容器解释器
// ═══════════════════════════════════════════════════════════════════════════

struct Container {
    code: u8,
    x: f64, y: f64,
}

fn topo(code: u8) -> u8 { match code & 0x0F { 0..=3 => 0x10, 4..=6 => 0x30, 7..=10 => 0x20, 11..=13 => 0x50, _ => 0x60 } }
fn family(code: u8) -> u8 { code >> 4 }
fn fam_color(code: u8) -> &'static str { match family(code) { 0=>"#4A90D9",1=>"#7B68EE",2=>"#50B86C",3=>"#D98C4A",_=>"#666" } }
fn topo_glyph(code: u8) -> &'static str {
    match topo(code) { 0x10=>"■",0x20=>"●",0x30=>"◎",0x50=>"◆",_=>"━" }
}

// Morton 解码: 16位 → (x,y)
fn morton_decode(z: u16) -> (f64, f64) {
    let mut x = 0u16;
    let mut y = 0u16;
    for i in 0..8 {
        x |= ((z >> (2*i)) & 1) << i;
        y |= ((z >> (2*i+1)) & 1) << i;
    }
    (x as f64, y as f64)
}

// ═══════════════════════════════════════════════════════════════════════════
// 场景构建 — 这是你能改的部分
// ═══════════════════════════════════════════════════════════════════════════

fn default_scene() -> Vec<Container> {
    let mut scene: Vec<Container> = Vec::new();

    // 中心: 囡 0x14 (满全族核心)
    scene.push(Container { code: 0x14, x: 400.0, y: 300.0 });

    // 夫妻对: 囡(0x14) + 伻? 用伴(0x17)
    scene.push(Container { code: 0x17, x: 520.0, y: 300.0 });

    // 四族代表按 Morton 环分布
    for i in 0u16..16 {
        let (mx, my) = morton_decode(i * 4);
        let angle = (i as f64 / 16.0) * std::f64::consts::PI * 2.0;
        let r = 180.0;
        let cx = 400.0 + angle.cos() * r * (1.0 + my * 0.02);
        let cy = 300.0 + angle.sin() * r * (1.0 + mx * 0.02);
        let code = i as u8; // 0x00-0x0F 运算族
        scene.push(Container { code, x: cx, y: cy });
    }

    // 声母环: 0x20-0x2F
    for i in 0..16u16 {
        let angle = (i as f64 / 16.0) * std::f64::consts::PI * 2.0;
        let r = 260.0;
        let cx = 400.0 + angle.cos() * r;
        let cy = 300.0 + angle.sin() * r;
        scene.push(Container { code: 0x20 + i as u8, x: cx, y: cy });
    }

    // 韵母线: 0x30-0x3F
    for i in 0..16u16 {
        let t = i as f64 / 15.0;
        let cx = 100.0 + t * 600.0;
        let cy = if i % 2 == 0 { 80.0 } else { 520.0 };
        scene.push(Container { code: 0x30 + i as u8, x: cx, y: cy });
    }

    scene
}

// ═══════════════════════════════════════════════════════════════════════════
// 渲染管线: 容器 → SVG
// ═══════════════════════════════════════════════════════════════════════════

fn render_svg(scene: &[Container]) -> String {
    let mut svg = String::with_capacity(24000);
    writeln!(svg, "<svg viewBox=\"0 0 800 600\" xmlns=\"http://www.w3.org/2000/svg\" width=\"800\" height=\"600\">");
    writeln!(svg, "<rect width=\"800\" height=\"600\" fill=\"#0a0a1a\"/>");

    // 1. 连线 — 同族容器自动连
    for i in 0..scene.len() {
        for j in i+1..scene.len() {
            if family(scene[i].code) == family(scene[j].code) {
                let dist = ((scene[i].x - scene[j].x).hypot(scene[i].y - scene[j].y));
                if dist < 200.0 {
                    let col = fam_color(scene[i].code);
                    writeln!(svg, "<line x1=\"{:.0}\" y1=\"{:.0}\" x2=\"{:.0}\" y2=\"{:.0}\" stroke=\"{col}\" stroke-width=\"1\" opacity=\"0.3\"/>",
                        scene[i].x, scene[i].y, scene[j].x, scene[j].y);
                }
            }
        }
    }

    // 2. 韵母 — 声母匹配线
    for c in scene {
        if family(c.code) == 3 && (c.code & 0x0F) < 16 {
            // 韵母连接到对应声母
            let sm_code = 0x20 + (c.code & 0x0F);
            if let Some(sm) = scene.iter().find(|s| s.code == sm_code) {
                writeln!(svg, "<line x1=\"{:.0}\" y1=\"{:.0}\" x2=\"{:.0}\" y2=\"{:.0}\" stroke=\"#D98C4A\" stroke-width=\"0.5\" opacity=\"0.2\" stroke-dasharray=\"2,2\"/>",
                    c.x, c.y, sm.x, sm.y);
            }
        }
    }

    // 3. 容器形状
    for c in scene {
        let color = fam_color(c.code);
        let r = 14.0 + (topo(c.code) as f64 / 0x60 as f64) * 6.0;

        match topo(c.code) {
            0x10 => { // 盒
                writeln!(svg, "<rect x=\"{:.0}\" y=\"{:.0}\" width=\"{:.0}\" height=\"{:.0}\" fill=\"none\" stroke=\"{}\" stroke-width=\"2\" rx=\"3\"/>",
                    c.x - r, c.y - r, r*2.0, r*2.0, color);
            }
            0x20 => { // 球
                writeln!(svg, "<circle cx=\"{:.0}\" cy=\"{:.0}\" r=\"{:.0}\" fill=\"none\" stroke=\"{color}\" stroke-width=\"2\"/>",
                    c.x, c.y, r);
            }
            0x30 => { // 环
                writeln!(svg, "<circle cx=\"{:.0}\" cy=\"{:.0}\" r=\"{:.0}\" fill=\"none\" stroke=\"{color}\" stroke-width=\"2\"/>", c.x, c.y, r);
                let r2 = r * 0.55;
                writeln!(svg, "<circle cx=\"{:.0}\" cy=\"{:.0}\" r=\"{:.0}\" fill=\"none\" stroke=\"{color}\" stroke-width=\"1\"/>", c.x, c.y, r2);
            }
            0x50 => { // 双锥
                writeln!(svg, "<polygon points=\"{:.0},{:.0} {:.0},{:.0} {:.0},{:.0}\" fill=\"none\" stroke=\"{}\" stroke-width=\"2\"/>",
                    c.x, c.y - r, c.x - r, c.y + r, c.x + r, c.y + r, color);
            }
            _ => { // 线
                writeln!(svg, "<line x1=\"{:.0}\" y1=\"{:.0}\" x2=\"{:.0}\" y2=\"{:.0}\" stroke=\"{}\" stroke-width=\"3\"/>",
                    c.x - r, c.y, c.x + r, c.y, color);
            }
        }

        // 标签
        writeln!(svg, "<text x=\"{:.0}\" y=\"{:.0}\" text-anchor=\"middle\" font-size=\"10\" fill=\"{}\" dy=\"{:.0}\">{}</text>",
            c.x, c.y + 3.0, color, r + 12.0, NAMES[c.code as usize]);

        // hex
        writeln!(svg, "<text x=\"{:.0}\" y=\"{:.0}\" text-anchor=\"middle\" font-size=\"8\" fill=\"#666\" dy=\"{:.0}\">0x{:02X}</text>",
            c.x, c.y + 3.0, r + 22.0, c.code);
    }

    writeln!(svg, "</svg>");
    svg
}

// ═══════════════════════════════════════════════════════════════════════════
// HTML 外壳
// ═══════════════════════════════════════════════════════════════════════════

fn wrap_html(svg: &str, scene: &[Container]) -> String {
    let mut h = String::new();
    writeln!(h, "<!DOCTYPE html><html lang=\"zh\"><head><meta charset=\"UTF-8\"><title>容器原生渲染</title>").unwrap();
    writeln!(h, "<style>body{{background:#0a0a1a;color:#ddd;font-family:sans-serif;margin:0;padding:20px}}svg{{border:1px solid #223}}</style></head><body>").unwrap();
    writeln!(h, "<b>容器原生渲染</b> — 容器语言解释投射到 SVG").unwrap();
    writeln!(h, "<p style=\"color:#888;font-size:12px\">{} 容器 · 同族自动连线 · 韵→声匹配 · 拓扑决定形状 · 族决定颜色</p>", scene.len()).unwrap();
    write!(h, "{}", svg).unwrap();
    writeln!(h, "</body></html>").unwrap();
    h
}

fn main() {
    let _args: Vec<String> = env::args().collect();
    let scene = default_scene();
    let svg = render_svg(&scene);
    let html = wrap_html(&svg, &scene);
    std::fs::write("/tmp/container_native.html", &html).expect("write");
    println!("容器原生渲染: {} containers → {} bytes", scene.len(), html.len());
    println!("容器分布:");
    for fam in 0u8..4 {
        let count = scene.iter().filter(|c| family(c.code) == fam).count();
        println!("  {}族 {} 个", match fam { 0=>"运算", 1=>"满全", 2=>"声母", _=>"韵母" }, count);
    }
}
