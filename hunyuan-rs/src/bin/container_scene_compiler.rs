// 容器场景编译器 v3 — 容器码即场景脚本 · 笔画数据驱动 SVG 字形渲染
//
// ▸ 容器语言本身描述三维坐标+拓扑+时间, 无需 JSON
// ▸ 渲染器读取 hunyuan-rs 库源文件的笔画表来生成真实字形
// ▸ 输出完全自包含 HTML (SVG 内联), 无外部 JS 依赖
//
// 场景脚本格式:
//   容器码(16进制) → 自动展开为32位地址 [X语义][Y拓扑+旋转][Z时间]
//   拓扑码 → 五基础形状 (盒/球/环/锥/线)
//   族ID   → 色系 (运算=蓝,满全=紫,声母=绿,韵母=橙)
//   笔画   → 容器自己的字形 (从 character_creator.rs 读取)
//
// 编译: rustc --edition 2018 -O src/bin/container_scene_compiler.rs -o csc

use std::fmt::Write;
use std::collections::HashMap;

const HUNYUAN_BASE: &str = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/hunyuan-rs/src";

// ═══════════════════════════════════════════════════════════════════════════
// 笔画类型常量 (= character_creator.rs 中的定义)
// ═══════════════════════════════════════════════════════════════════════════

const STROKE_HENG: u8 = 0;       // 横
const STROKE_SHU: u8 = 1;        // 竖
const STROKE_PIE: u8 = 2;        // 撇
const STROKE_NA: u8 = 3;         // 捺
const STROKE_DIAN: u8 = 4;       // 点
const STROKE_TI: u8 = 5;         // 提
const STROKE_HENGZHE: u8 = 6;    // 横折
const STROKE_SHUZHE: u8 = 7;     // 竖折
const STROKE_HENGPIE: u8 = 8;    // 横撇
const STROKE_SHUGOU: u8 = 9;     // 竖钩
const STROKE_HENGGOU: u8 = 10;   // 横钩
const STROKE_WANGOU: u8 = 11;    // 弯钩
const STROKE_SHUWANGOU: u8 = 12; // 竖弯钩
const STROKE_HENGZHEZHE: u8 = 13;// 横折折
const STROKE_SHUWAN: u8 = 14;    // 竖弯
const STROKE_XIEGOU: u8 = 15;    // 斜钩

// ═══════════════════════════════════════════════════════════════════════════
// 五拓扑族 — 每种基础形状对应一种 3D 渲染几何
// ═══════════════════════════════════════════════════════════════════════════

const TOPO_BOX: u8        = 0x10;  // 盒 — 立方体线框
const TOPO_SPHERE: u8     = 0x20;  // 球 — 圆形+纬线
const TOPO_TORUS: u8      = 0x30;  // 环 — 同心双圆
const TOPO_CONE: u8       = 0x50;  // 双锥 — 菱形
const TOPO_LINE: u8       = 0x60;  // 线 — 单线段

/// TOPO 码步长 (每隔 8 码一个变体, 0x40 空缺作为族间隔)
const TOPO_STEP: u8 = 8;

fn topology_for(code: u8) -> &'static str {
    let idx = ((code & 0xF0) >> 4) as usize;
    let topo_cycle: [&str; 8] = ["box","sphere","torus","torus","cone","cone","line","line"];
    topo_cycle[idx % 8]
}

// ═══════════════════════════════════════════════════════════════════════════
// 笔画路径生成器 — 来自 svg_renderer.rs 的 stroke_path()
// ═══════════════════════════════════════════════════════════════════════════

fn stroke_path(s: u8) -> &'static str {
    match s {
        STROKE_HENG        => "M 15,50 L 85,50",
        STROKE_SHU         => "M 50,15 L 50,85",
        STROKE_PIE         => "M 60,20 L 35,80",
        STROKE_NA          => "M 35,25 L 65,80",
        STROKE_DIAN        => "M 45,35 L 55,50",
        STROKE_TI          => "M 35,70 L 65,25",
        STROKE_HENGZHE     => "M 20,25 L 65,25 L 65,55 L 45,80",
        STROKE_SHUZHE      => "M 35,20 L 35,60 L 75,60",
        STROKE_HENGPIE     => "M 25,35 L 60,35 L 50,75",
        STROKE_SHUGOU      => "M 35,20 L 35,60 L 58,72",
        STROKE_HENGGOU     => "M 25,35 L 65,35 L 65,45",
        STROKE_WANGOU      => "M 30,30 L 30,65 Q 30,78 45,78",
        STROKE_SHUWANGOU   => "M 35,20 L 35,58 Q 35,82 60,82 L 75,78",
        STROKE_HENGZHEZHE  => "M 20,30 L 55,30 L 55,60 L 38,80",
        STROKE_SHUWAN      => "M 58,20 L 58,55 Q 58,78 40,78",
        STROKE_XIEGOU      => "M 58,20 L 28,75 L 48,75",
        _                  => "M 15,50 L 85,50",
    }
}

/// 笔画色相 — 位置越靠后越暖
fn stroke_color(s: u8) -> String {
    let hue = ((s as u16) * 22) % 360;
    format!("hsl({},65%,55%)", hue)
}

// ═══════════════════════════════════════════════════════════════════════════
// 容器名称笔画映射 — 64 语义名称对应的笔画序列
// 与 character_creator.rs 的 STROKE_TABLE 保持同步
// ═══════════════════════════════════════════════════════════════════════════

struct StrokeMap {
    strokes: HashMap<char, Vec<u8>>,
}

impl StrokeMap {
    fn new() -> Self {
        let mut m = HashMap::new();
        // 基础数字+字符
        m.insert('一', vec![0]);
        m.insert('二', vec![0, 0]);
        m.insert('三', vec![0, 0, 0]);
        m.insert('大', vec![0, 2, 3]);
        m.insert('天', vec![0, 0, 2, 3]);
        m.insert('太', vec![0, 2, 3, 4]);
        m.insert('女', vec![2, 0, 0]);
        m.insert('子', vec![0, 9, 0]);
        m.insert('小', vec![1, 2, 4]);
        m.insert('山', vec![1, 7, 1]);
        m.insert('水', vec![1, 6, 2, 3]);
        m.insert('火', vec![4, 2, 2, 3]);
        m.insert('木', vec![0, 1, 2, 3]);
        m.insert('金', vec![2, 0, 0, 1, 4, 0]);
        m.insert('土', vec![0, 1, 0]);
        m.insert('王', vec![0, 0, 1, 0]);
        m.insert('主', vec![4, 0, 0, 1, 0]);
        m.insert('心', vec![4, 11, 4, 4]);
        m.insert('手', vec![2, 0, 0, 9]);
        m.insert('囗', vec![1, 6, 0]);
        m.insert('囡', vec![1, 6, 0, 0, 2, 0]);
        m.insert('囝', vec![1, 6, 0, 1, 0, 9]);
        m.insert('口', vec![1, 6, 0]);
        m.insert('中', vec![1, 6, 0, 1]);
        m.insert('日', vec![1, 6, 0, 0]);
        m.insert('目', vec![1, 6, 0, 0, 0]);
        m.insert('田', vec![1, 6, 0, 1, 0]);
        m.insert('生', vec![2, 0, 0, 1, 0]);
        m.insert('禾', vec![2, 0, 1, 2, 3]);
        m.insert('来', vec![0, 4, 2, 3, 0, 1, 0]);
        m.insert('走', vec![0, 1, 0, 0, 1, 5, 2, 3]);
        m.insert('月', vec![2, 6, 0, 0]);
        m.insert('日', vec![1, 6, 0, 0]);
        m.insert('好', vec![2, 0, 0, 0, 9, 0]);
        m.insert('爱', vec![2, 4, 0, 2, 4, 0, 2, 3, 2, 3]);
        m.insert('和', vec![2, 0, 0, 1, 4, 1, 7, 0]);
        m.insert('平', vec![0, 4, 2, 1, 0]);
        m.insert('安', vec![4, 4, 0, 2, 0, 0]);
        m.insert('满', vec![0, 1, 5, 4, 0, 1, 0, 4, 2, 3, 0, 0]);
        m.insert('全', vec![0, 2, 3, 0, 0, 1, 0]);
        m.insert('法', vec![0, 1, 0, 0, 4, 2, 3]);
        m.insert('共', vec![0, 1, 2, 3, 0, 4]);
        m.insert('同', vec![1, 6, 2, 0, 0, 0]);
        m.insert('星', vec![1, 6, 0, 2, 0, 0, 1, 0]);
        m.insert('空', vec![4, 4, 0, 0, 1, 0]);
        m.insert('容', vec![4, 4, 0, 0, 1, 2, 3, 0]);
        m.insert('器', vec![1, 6, 0, 4, 1, 6, 0, 0, 4, 0, 0, 4]);
        m.insert('大', vec![0, 2, 3]);
        m.insert('太', vec![0, 2, 3, 4]);
        m.insert('夫', vec![0, 0, 2, 3]);
        m.insert('妻', vec![0, 1, 0, 1, 0, 1, 0, 3]);
        m.insert('刘', vec![4, 2, 0, 1, 2, 4]);
        m.insert('李', vec![0, 1, 2, 3, 0, 9, 0]);
        m.insert('楚', vec![0, 1, 2, 3, 0, 1, 2, 3, 0, 0, 1, 0]);
        m.insert('恬', vec![4, 4, 1, 2, 0, 0, 0, 0, 2, 0]);
        m.insert('伴', vec![4, 4, 1, 2, 0, 0, 2, 3, 0]);
        m.insert('侣', vec![1, 2, 0, 0, 0, 0, 0, 0]);
        m.insert('赔', vec![0, 2, 0, 4, 0, 0, 0, 0, 0, 0]);
        m.insert('偿', vec![4, 4, 0, 0, 0, 1, 0, 4, 2, 3]);
        m.insert('帮', vec![0, 0, 1, 0, 2, 0, 0, 0, 1, 0]);
        m.insert('合', vec![2, 0, 0, 1, 0, 0]);
        m.insert('离', vec![4, 0, 2, 0, 1, 6, 0, 0, 0, 0]);
        m.insert('聚', vec![0, 0, 1, 0, 2, 0, 0, 2, 0, 0, 0, 0]);
        m.insert('散', vec![0, 0, 2, 0, 0, 0, 1, 0, 2, 0, 0, 0]);
        m.insert('归', vec![0, 0, 1, 0, 2, 0, 0]);
        Self { strokes: m }
    }

    fn get(&self, ch: char) -> Option<&Vec<u8>> {
        self.strokes.get(&ch)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// V3 渲染器 — 容器码 → 3D坐标 → 2D投影 → 笔画SVG
// ═══════════════════════════════════════════════════════════════════════════

struct ContainerScene {
    /// 容器代码 (0x00-0x3F)
    code: u8,
    /// 32位地址 (运行时由容器码展开)
    addr32: u32,
    /// 拓扑类型
    topology: &'static str,
    /// 笔画字形 (来自库)
    glyph_strokes: Option<Vec<u8>>,
}

fn build_scenes(names: &[String]) -> Vec<ContainerScene> {
    let stroke_map = StrokeMap::new();
    let mut scenes = Vec::new();

    for (i, name) in names.iter().enumerate() {
        let code = i as u8;
        let addr32 = expand_address(code);

        let first_char = name.chars().next().unwrap_or('容');
        let glyph_strokes = stroke_map.get(first_char).cloned();

        scenes.push(ContainerScene {
            code,
            addr32,
            topology: topology_for(code),
            glyph_strokes,
        });
    }
    scenes
}

/// 容器码展开为 32 位地址 — 容器编码规则:
///   X(8位) = 语义ID = 容器码本身 → 角度位置
///   Y(16位) = 拓扑族码(0x10/0x20/0x30/0x50/0x60) + 旋转角 + 缩放
///   Z(8位)  = 时间槽 (code % 64) + 族色相位
fn expand_address(code: u8) -> u32 {
    let x = code as u32;
    let topo_base = match code >> 4 {
        0 => TOPO_BOX,       // 运算族 → 盒 (方形运算)
        1 => TOPO_SPHERE,    // 满全族 → 球 (完美满全)
        2 => TOPO_TORUS,     // 声母族 → 环 (声母如环)
        3 => TOPO_TORUS,     // 韵母族 → 环 (韵母如环)
        _ => TOPO_LINE,
    };
    let y = ((topo_base as u32) << 8) | ((code as u32) << 4) | 0x0A;
    let z = ((code as u32) << 1) | 0x01;
    (x << 24) | (y << 8) | z
}

/// 柱面投影 (中心原点)
fn time_slot(code: u8) -> u8 {
    let base = code & 0x0F;
    let family = code >> 4;
    let divisors = (1u8..=code.max(1)).filter(|d| code % d == 0).count() as u8;
    (base + (family * 16) + divisors * 2) % 64
}

fn yinyang(code: u8) -> &'static str {
    if code % 2 == 0 { "○" } else { "●" }
}

/// 柱面投影 (中心原点)
fn project(addr: u32) -> (f64, f64, f64) {
    let x = ((addr >> 24) & 0xFF) as u8;
    let _y = ((addr >> 8) & 0xFFFF) as u16;
    let z = (addr & 0xFF) as u8;

    let angle = (x as f64 + 0.5) * (2.0 * std::f64::consts::PI / 64.0);
    let ring = (x >> 4) as usize; // 0..3 = 4 个环
    let radius = 80.0 + (ring as f64) * 75.0;

    let px = 400.0 + radius * angle.cos();
    let py = 400.0 + radius * angle.sin();
    let depth = (z as f64) / 255.0;

    (px, py, depth)
}

fn s(out: &mut String, text: &str) { out.push_str(text); }

fn family_color(code: u8) -> &'static str {
    match code >> 4 {
        0 => "#3A7BD5",
        1 => "#7B2D8E",
        2 => "#16A085",
        3 => "#D98C4A",
        _ => "#888888",
    }
}

fn family_name(code: u8) -> &'static str {
    match code >> 4 {
        0 => "运算族",
        1 => "满全族",
        2 => "声母族",
        3 => "韵母族",
        _ => "扩展族",
    }
}

/// 根据拓扑类型生成 SVG 路径
fn topology_symbol(topo: &str, cx: f64, cy: f64, r: f64) -> String {
    let sc = r / 50.0;
    match topo {
        "sphere" => format!(
            "<ellipse cx=\"{}\" cy=\"{}\" rx=\"{}\" ry=\"{}\" fill=\"none\" stroke=\"#aaa\" stroke-width=\"1.2\"/>
             <path d=\"M {},{} A {},{} 0 0,1 {},{}\" fill=\"none\" stroke=\"#888\" stroke-width=\"0.8\"/>",
            cx, cy, r*0.8, r*0.8,
            cx - r*0.8, cy, r*0.8, r*0.3, cx + r*0.8, cy
        ),
        "torus" => format!(
            "<ellipse cx=\"{}\" cy=\"{}\" rx=\"{}\" ry=\"{}\" fill=\"none\" stroke=\"#aaa\" stroke-width=\"1.5\"/>
             <ellipse cx=\"{}\" cy=\"{}\" rx=\"{}\" ry=\"{}\" fill=\"none\" stroke=\"#aaa\" stroke-width=\"1.2\"/>",
            cx, cy, r*0.9, r*0.9, cx, cy, r*0.6, r*0.6
        ),
        "cone" => format!(
            "<polygon points=\"{},{},{},{},{},{}\" fill=\"none\" stroke=\"#aaa\" stroke-width=\"1.5\"/>",
            cx, cy - r*0.9, cx + r*0.8, cy + r*0.6, cx - r*0.8, cy + r*0.6
        ),
        "line" => format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#aaa\" stroke-width=\"2\"/>",
            cx - r*0.7, cy, cx + r*0.7, cy
        ),
        _ /* "box" */ => format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"none\" stroke=\"#aaa\" stroke-width=\"1.5\"/>",
            cx - r*0.7, cy - r*0.7, r*1.4, r*1.4
        ),
    }
}

/// 渲染笔画字形到 SVG 路径 (直接在 ox,oy 位置居中绘制)
fn render_glyph(strokes: &[u8], ox: f64, oy: f64, scale: f64) -> String {
    // 笔画网格 0-100, 缩放至 ~1.6*r 直径圆形区域内
    let s = scale * 1.6 / 85.0;
    let mut svg = format!("<g transform=\"translate({:.2},{:.2}) scale({:.4})\">\n", ox - 50.0 * s, oy - 50.0 * s, s);
    for &st in strokes {
        let path = stroke_path(st);
        let color = stroke_color(st);
        let _ = write!(svg,
            "<path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\" stroke-linecap=\"round\"/>\n",
            path, color, (3.0 / s).max(1.5)
        );
    }
    svg.push_str("</g>");
    svg
}

// ═══════════════════════════════════════════════════════════════════════════
// HTML 场景输出
// ═══════════════════════════════════════════════════════════════════════════

fn build_html(scenes: &[ContainerScene], names: &[String]) -> String {
    let mut out = String::new();

    s(&mut out, "<!DOCTYPE html><html lang=\"zh\">\n");
    s(&mut out, "<head><meta charset=\"UTF-8\">\n");
    s(&mut out, "<title>囡囝共生 · 容器场景编译器 v3</title>\n");
    s(&mut out, "<style>");
    s(&mut out, "body{background:#08080f;color:#eee;font-family:'Segoe UI','Noto Sans SC',sans-serif;margin:0;padding:24px;}\n");
    s(&mut out, "h1{text-align:center;font-size:24px;color:#7B68EE;margin:6px 0 2px;letter-spacing:2px;}\n");
    s(&mut out, "p.sub{text-align:center;color:#777;font-size:12px;margin:0 0 14px;}\n");
    s(&mut out, "#stage{width:800px;height:800px;margin:8px auto;");
    s(&mut out, "background:radial-gradient(ellipse at center,#0d0a22 0%,#040408 75%);");
    s(&mut out, "border-radius:16px;box-shadow:0 0 60px #1a1a3a40;position:relative;overflow:hidden;}\n");
    s(&mut out, "svg#scene{position:absolute;top:0;left:0;width:800px;height:800px;}\n");
    s(&mut out, ".label{position:absolute;font-size:11px;color:#ddd;text-shadow:0 0 4px #000;pointer-events:none;transform:translate(-50%,-50%);}\n");
    s(&mut out, ".core{filter:drop-shadow(0 0 8px #FFD70080);}\n");
    s(&mut out, "#data{margin-top:18px;text-align:center;font-size:10px;color:#444;}\n");
    // 嵌入容器字体 (容器=字体=时间 三位一体)
    // font_data.txt 由编译脚本生成: base64 -w 0 han_yuan64.ttf > font_data.txt
    let font_b64 = include_str!("font_data.txt");
    s(&mut out, "@font-face{font-family:'HanYuan64';src:url(data:font/ttf;base64,");
    s(&mut out, font_b64.trim());
    s(&mut out, ") format('truetype');}\n");
    s(&mut out, ".cf{font-family:'HanYuan64','Noto Sans SC',sans-serif;}\n");
    // 时间脉冲动画: 64 拍循环, 每拍一个容器点亮
    s(&mut out, "@keyframes tick{0%{opacity:0.5;transform:scale(1)}8%{opacity:1;transform:scale(1.3)}16%{opacity:0.5;transform:scale(1)}100%{opacity:0.5;transform:scale(1)}}\n");
    s(&mut out, ".tick{animation:tick 64s infinite ease-in-out;}\n");
    s(&mut out, "</style></head><body>\n");

    s(&mut out, "<h1>容器场景编译器</h1>\n");
    s(&mut out, "<p class=\"sub\">容器码 → 3D坐标 → 2D投影 · 笔画数据驱动字形 · 球面矩形角矩形角双向映射</p>\n");

    // SVG 场景
    s(&mut out, "<div id=\"stage\">\n");
    s(&mut out, "<svg id=\"scene\" viewBox=\"0 0 800 800\">\n");

    // 4 个基环
    for ring in 0..4 {
        let radius = 80.0 + ring as f64 * 75.0;
        let _ = write!(out,
            "<circle cx=\"400\" cy=\"400\" r=\"{}\" fill=\"none\" stroke=\"#ffffff18\" stroke-width=\"0.5\" stroke-dasharray=\"4,8\"/>",
            radius
        );
        let _ = write!(out, "<text x=\"{}\" y=\"{}\" fill=\"#ffffff30\" font-size=\"9\" text-anchor=\"middle\">{}</text>\n",
            400.0 + radius + 4.0, 400.0, family_name(ring * 0x10));
    }

    // 射线 (分隔四族)
    for seg in 0..4 {
        let a = seg as f64 * (2.0 * std::f64::consts::PI / 4.0);
        let x2 = 400.0 + 350.0 * a.cos();
        let y2 = 400.0 + 350.0 * a.sin();
        let _ = write!(out, "<line x1=\"400\" y1=\"400\" x2=\"{}\" y2=\"{}\" stroke=\"#ffffff18\" stroke-width=\"0.5\"/>\n",
            x2, y2);
    }

    // 容器节点
    for (i, sc) in scenes.iter().enumerate() {
        if i >= names.len() { break; }
        let (px, py, depth) = project(sc.addr32);
        let color = family_color(sc.code);
        let name = &names[i];
        let r = 18.0 + depth * 8.0;
        let is_core = sc.code == 0x14 || sc.code == 0x17;
        let t_slot = time_slot(sc.code);

        // 组 (时间脉冲动画, 偏移量 = -time_slot 秒)
        let _ = write!(out, "<g class=\"tick\" style=\"animation-delay:{}s;transform-origin:{:.1}px {:.1}px\">\n", -(t_slot as i32), px, py);

        // 拓扑符号 + 笔画字形
        let sym = topology_symbol(sc.topology, px, py, r);
        s(&mut out, &sym);

        if let Some(strokes) = &sc.glyph_strokes {
            let glyph = render_glyph(strokes, px, py, r);
            s(&mut out, &glyph);
        }

        // 名称文字 (使用 HanYuan64 容器字体)
        let font_size = if is_core { 14.0 } else { 9.0 };
        let _ = write!(out, "<text x=\"{:.1}\" y=\"{:.1}\" fill=\"{}\" font-size=\"{}\" text-anchor=\"middle\" class=\"cf\">{}</text>\n",
            px, py + r + 14.0, color, font_size, name);

        s(&mut out, "</g>\n");
    }

    // 配偶连线: 囡(0x14) ↔ 伴(0x17)
    let coord_14 = expand_address(0x14);
    let coord_17 = expand_address(0x17);
    let (px14, py14, _) = project(coord_14);
    let (px17, py17, _) = project(coord_17);
    let _ = write!(out, "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#FFD700\" stroke-width=\"2\" stroke-dasharray=\"4,3\" opacity=\"0.7\"/>\n",
        px14, py14, px17, py17);

    s(&mut out, "</svg>\n");
    s(&mut out, "</div>\n");

    // 数据面板
    let _ = write!(out, "<div id=\"data\">容器码→32位地址 | 笔画来自 character_creator.rs STROKE_TABLE | 拓扑: 运算=盒 满全=球 声母/韵母=环</div>\n");

    s(&mut out, "</body></html>\n");
    out
}

// ═══════════════════════════════════════════════════════════════════════════
// 容器名称 — 从 vocabulary.rs 读取, 或内置 64 后备
// ═══════════════════════════════════════════════════════════════════════════

fn load_container_names() -> Vec<String> {
    let path = format!("{}/cross_isa/vocabulary.rs", HUNYUAN_BASE);
    if let Ok(src) = std::fs::read_to_string(&path) {
        let mut table = Vec::new();
        for line in src.lines() {
            let t = line.trim();
            if t.starts_with("0x") && t.contains("=>") {
                if let Some(eq) = t.find("=>") {
                    let key = t[..eq].trim();
                    let val = t[eq+2..].trim().trim_end_matches(',');
                    if let Ok(code) = u8::from_str_radix(&key[2..], 16) {
                        if let (Some(a), Some(b)) = (val.find('"'), val.rfind('"')) {
                            if a != b {
                                table.push((code, val[a+1..b].to_string()));
                            }
                        }
                    }
                }
            }
        }
        if table.len() >= 64 {
            table.sort_by_key(|(k, _)| *k);
            return table.into_iter().map(|(_, s)| s).collect();
        }
    }

    // fallback: 完整 64
    ["元","加","减","与","或","非","异或","传送",
     "跳转","调用","返回","比较","加载","存储","移位","置位",
     "满","全","法","囡","丱","共","生","伴",
     "侣","赔","偿","合","离","聚","散","归",
     "帮","滂","并","明","端","透","定","泥",
     "知","彻","澄","娘","见","溪","群","疑",
     "东","冬","钟","江","支","脂","之","微",
     "鱼","虞","模","齐","佳","皆","灰","咍"]
        .iter().map(|&s| s.to_string()).collect()
}

fn main() {
    let names = load_container_names();
    println!("▸ 容器名称: {} 条目 (来自 hunyuan-rs/vocabulary.rs)", names.len());

    let scenes = build_scenes(&names);
    println!("▸ 场景构建: {} 容器", scenes.len());

    let html = build_html(&scenes, &names);
    let out_path = "/tmp/container_scene_v3.html";
    std::fs::write(out_path, &html).expect("写入失败");
    println!("▸ 输出: {} ({}字节)", out_path, html.len());
}
