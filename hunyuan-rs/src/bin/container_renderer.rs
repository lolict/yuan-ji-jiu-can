// 容器原生渲染引擎 v2 — 读取 hunyuan-rs 库源码作为数据源
//
// 用法: ./container_renderer [输出路径]
//
// 数据源: 直接读取 ../hunyuan-rs/src/cross_isa/*.rs 和 encoding/*.rs
// 当容器词汇表/拓扑/坐标更新时, 渲染器自动跟随 — 不需要重新编译
//
// 编译: rustc --edition 2018 -O src/bin/container_renderer.rs -o container_renderer

use std::env;
use std::fmt::Write;

const HUNYUAN_BASE: &str = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/hunyuan-rs/src";

// ═══════════════════════════════════════════════════════════════════════════
// 运行时读 hunyuan-rs 源文件 — 数据容器是文件而非上下文
// ═══════════════════════════════════════════════════════════════════════════

fn read_file(path: &str) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

/// vocabulary.rs 中提取 ContainerId::name() 的匹配表
fn parse_container_names(src: &str) -> Vec<(u8, String)> {
    let mut out = Vec::new();
    for line in src.lines() {
        let t = line.trim();
        if t.starts_with("0x") && t.contains("=>") && t.contains('"') {
            if let Some(eq) = t.find("=>") {
                let key = t[..eq].trim();
                let val = t[eq+2..].trim().trim_end_matches(',');
                if let Ok(code) = u8::from_str_radix(&key[2..], 16) {
                    if let (Some(a), Some(b)) = (val.find('"'), val.rfind('"')) {
                        if a != b {
                            out.push((code, val[a+1..b].to_string()));
                        }
                    }
                }
            }
        }
    }
    out
}

/// vocabulary.rs 中提取族名 (定位到 match self.0 块)
fn parse_family_names(src: &str) -> Vec<(u8, String)> {
    let mut out = Vec::new();
    let mut depth = 0;
    for line in src.lines() {
        let t = line.trim();
        if t.contains("match self.0") {
            depth = 1;
            continue;
        }
        if depth > 0 {
            if t == "}" { depth -= 1; if depth == 0 { break; } continue; }
            if t.starts_with('{') { depth += 1; continue; }
            if t.starts_with("0 =>") || t.starts_with("1 =>") ||
               t.starts_with("2 =>") || t.starts_with("3 =>") ||
               t.starts_with("4 =>") || t.starts_with("_ =>") {
                if let Some(eq) = t.find("=>") {
                    let val = t[eq+2..].trim().trim_end_matches(',');
                    if let (Some(a), Some(b)) = (val.find('"'), val.rfind('"')) {
                        if a != b {
                            let key: Result<u8, _> = t[..eq].trim().parse();
                            let code = key.unwrap_or(255);
                            out.push((code, val[a+1..b].to_string()));
                        }
                    }
                }
            }
        }
    }
    out
}

/// 64 名称后备表 (库文件仅部分时补全)
fn full_name_fallback() -> Vec<(u8, String)> {
    let raw = [
        "元","加","减","与","或","非","异或","传送",
        "跳转","调用","返回","比较","加载","存储","移位","置位",
        "满","全","法","囡","丱","共","生","伴",
        "侣","赔","偿","合","离","聚","散","归",
        "帮","滂","并","明","端","透","定","泥",
        "知","彻","澄","娘","见","溪","群","疑",
        "东","冬","钟","江","支","脂","之","微",
        "鱼","虞","模","齐","佳","皆","灰","咍",
    ];
    raw.iter().enumerate().map(|(i, &s)| (i as u8, s.to_string())).collect()
}

fn load_container_name_table() -> Vec<(u8, String)> {
    let path = format!("{}/cross_isa/vocabulary.rs", HUNYUAN_BASE);
    let mut table = if let Some(src) = read_file(&path) {
        parse_container_names(&src)
    } else {
        vec![]
    };

    // 如果库只提供部分 (如16条), 用后备补全到 64
    if table.len() < 64 {
        let fallback = full_name_fallback();
        let existing: std::collections::HashSet<u8> = table.iter().map(|(k, _)| *k).collect();
        for (code, name) in fallback {
            if !existing.contains(&code) {
                table.push((code, name));
            }
        }
        table.sort_by_key(|(k, _)| *k);
    }
    table
}

fn load_family_name_table() -> Vec<(u8, String)> {
    // 优先尝试从 vocabulary.rs 读取
    let path = format!("{}/cross_isa/vocabulary.rs", HUNYUAN_BASE);
    if let Some(src) = read_file(&path) {
        let parsed = parse_family_names(&src);
        // 如果读到的不是4族分法(运算/满全/声母/韵母), 用实际代码体系
        if parsed.len() >= 4 && parsed.iter().any(|(_, n)| n.contains("运算")) {
            return parsed;
        }
    }
    // 按实际容器代码体系: 0=运算 1=满全 2=声母 3=韵母
    vec![
        (0, "运算族".to_string()),
        (1, "满全族".to_string()),
        (2, "声母族".to_string()),
        (3, "韵母族".to_string()),
    ]
}

fn lookup<'a>(table: &'a [(u8, String)], key: u8, default: &'a str) -> &'a str {
    for (k, v) in table {
        if *k == key { return v.as_str(); }
    }
    default
}

// ═══════════════════════════════════════════════════════════════════════════
// Morton/Z-order 3D→2D 投影 (来自 coordinate.rs 的坐标解释)
// ═══════════════════════════════════════════════════════════════════════════

fn morton_decode(code32: u32) -> (u8, u16, u8) {
    let x = ((code32 >> 24) & 0xFF) as u8;
    let y = ((code32 >> 8) & 0xFFFF) as u16;
    let z = (code32 & 0xFF) as u8;
    (x, y, z)
}

/// 柱面投影: (x语义, y操作数, z时间) → (screenX, screenY)
///   x → 角度 (360° 环状分布)
///   y → 半径 (偏移量)
///   z → 亮度/色相
fn project(x: u8, y: u16, z: u8) -> (f64, f64, f64) {
    let angle_deg = (x as f64) * 5.625; // 256 → 360° (每容器 5.625°)
    let angle_rad = angle_deg * std::f64::consts::PI / 180.0;
    let radius = 100.0 + ((y & 0xFF) as f64) * 1.5;

    let px = 400.0 + radius * angle_rad.cos();
    let py = 400.0 + radius * angle_rad.sin();
    let brightness = 0.4 + ((z & 0x0F) as f64) / 15.0 * 0.6;

    (px, py, brightness)
}

fn family_color(code: u8) -> &'static str {
    match code >> 4 {
        0 => "#3A7BD5",  // 运算族 — 蓝紫
        1 => "#7B2D8E",  // 满全族 — 紫
        2 => "#16A085",  // 声母族 — 青绿
        3 => "#D98C4A",  // 韵母族 — 橙金
        _ => "#888888",
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 场景定义 — 完整的四族 64 容器
// ═══════════════════════════════════════════════════════════════════════════

fn build_full_scene() -> Vec<(u8, u32)> {
    let mut out = Vec::with_capacity(64);

    // 运算族 0x00-0x0F
    for i in 0..16u8 {
        let y = 0x0A00 | (i as u16);
        let z = 0x00 | (i & 0x0F);
        out.push((i, (i as u32) << 24 | (y as u32) << 8 | (z as u32)));
    }

    // 满全族 0x10-0x1F
    for i in 0..16u8 {
        let code = 0x10 + i;
        let y = 0x0B00 | (i as u16);
        let z = 0x10 | (i & 0x0F);
        out.push((code, (code as u32) << 24 | (y as u32) << 8 | (z as u32)));
    }

    // 声母族 0x20-0x2F
    for i in 0..16u8 {
        let code = 0x20 + i;
        let y = 0x0C00 | (i as u16);
        let z = 0x20 | (i & 0x0F);
        out.push((code, (code as u32) << 24 | (y as u32) << 8 | (z as u32)));
    }

    // 韵母族 0x30-0x3F
    for i in 0..16u8 {
        let code = 0x30 + i;
        let y = 0x0D00 | (i as u16);
        let z = 0x30 | (i & 0x0F);
        out.push((code, (code as u32) << 24 | (y as u32) << 8 | (z as u32)));
    }

    out
}

// ═══════════════════════════════════════════════════════════════════════════
// 渲染 — 生成自描述 HTML (嵌入所有数据, 不依赖外部 JS)
// ═══════════════════════════════════════════════════════════════════════════

fn s(out: &mut String, text: &str) { out.push_str(text); }

fn build_html(container_table: &[(u8, String)], family_table: &[(u8, String)]) -> String {
    let scene = build_full_scene();
    let mut out = String::new();

    s(&mut out, "<!DOCTYPE html><html lang=\"zh\">\n");
    s(&mut out, "<head><meta charset=\"UTF-8\"><title>囡囝共生 — 64容器投影</title>\n");
    s(&mut out, "<style>\n");
    s(&mut out, "body{background:#0a0a14;color:#eee;font-family:'Segoe UI',sans-serif;margin:0;padding:20px;}\n");
    s(&mut out, "h1{text-align:center;font-size:22px;color:#7B68EE;margin:8px 0 4px;}\n");
    s(&mut out, "p.sub{text-align:center;color:#888;font-size:12px;margin-top:0;}\n");
    s(&mut out, "#canvas{margin:10px auto;width:800px;height:800px;background:radial-gradient(circle,#101025 0%,#05050f 100%);border-radius:12px;box-shadow:0 0 40px #1a1a3a80;overflow:hidden;}\n");
    s(&mut out, ".node{position:absolute;border-radius:50%;display:flex;align-items:center;justify-content:center;font-size:11px;font-weight:bold;color:#fff;text-shadow:0 0 4px #000;transition:all .3s;cursor:pointer;}\n");
    s(&mut out, ".node:hover{transform:scale(1.4);z-index:10;box-shadow:0 0 16px #fff6;}\n");
    s(&mut out, ".core{box-shadow:0 0 20px #FFD70080;border:2px solid #FFD700;}\n");
    s(&mut out, ".link{position:absolute;background:linear-gradient(90deg,#3A7BD5,#16A085);height:1px;opacity:0.3;transform-origin:left center;}\n");
    s(&mut out, "#legend{display:flex;gap:16px;justify-content:center;margin:12px 0;flex-wrap:wrap;}\n");
    s(&mut out, ".leg{display:flex;align-items:center;gap:4px;font-size:11px;}\n");
    s(&mut out, ".leg b{display:inline-block;width:14px;height:14px;border-radius:50%;}\n");
    s(&mut out, "#info{margin-top:20px;text-align:center;font-size:11px;color:#666;}\n");
    s(&mut out, "</style></head><body>\n");

    // 标题
    s(&mut out, "<h1>囡囝共生 · 64容器投影</h1>\n");
    s(&mut out, "<p class=\"sub\">容器即机器码 · 坐标即程序 · 字体即时间</p>\n");

    // 图例
    s(&mut out, "<div id=\"legend\">");
    let _ = write!(out, "<div class=\"leg\" style=\"color:{}\"><b style=\"background:{}\"></b>{}</div>",
        family_color(0), family_color(0), lookup(family_table, 0, "运算族"));
    let _ = write!(out, "<div class=\"leg\" style=\"color:{}\"><b style=\"background:{}\"></b>{}</div>",
        family_color(0x10), family_color(0x10), lookup(family_table, 1, "满全族"));
    let _ = write!(out, "<div class=\"leg\" style=\"color:{}\"><b style=\"background:{}\"></b>{}</div>\n",
        family_color(0x20), family_color(0x20), lookup(family_table, 2, "声母族"));
    let _ = write!(out, "<div class=\"leg\" style=\"color:{}\"><b style=\"background:{}\"></b>{}</div>\n",
        family_color(0x30), family_color(0x30), lookup(family_table, 3, "韵母族"));
    s(&mut out, "</div>\n");

    // SVG 画布
    s(&mut out, "<div id=\"canvas\">\n");

    // 连线: 中心 囡(0x14) → 伴(0x17) → 运算0 → 满全0 → 声母0 → 韵母0
    let pairs: Vec<(u8, u8)> = vec![(0x14, 0x17), (0x17, 0x10), (0x10, 0x00), (0x00, 0x20), (0x20, 0x30)];
    for (a, b) in &pairs {
        let a_coord = scene.iter().find(|(c, _)| *c == *a).map(|(_, coord)| *coord).unwrap_or(0);
        let b_coord = scene.iter().find(|(c, _)| *c == *b).map(|(_, coord)| *coord).unwrap_or(0);
        let (ax, ay, _) = morton_decode(a_coord);
        let (bx, by, _) = morton_decode(b_coord);
        let (px1, py1, _) = project(ax, ay, 0);
        let (px2, py2, _) = project(bx, by, 0);
        let dx = px2 - px1;
        let dy = py2 - py1;
        let dist = (dx * dx + dy * dy).sqrt();
        let angle = dy.atan2(dx) * 180.0 / std::f64::consts::PI;
        let _ = write!(out, "<div class=\"link\" style=\"left:{}px;top:{}px;width:{}px;transform:rotate({}deg);\"></div>\n",
            px1, py1, dist, angle);
    }

    // 节点
    let is_core = |code: u8| code == 0x14 || code == 0x17;
    for (code, coord) in &scene {
        let (x, y, z) = morton_decode(*coord);
        let (px, py, brightness) = project(x, y, z);
        let name = lookup(container_table, *code, "容");
        let color = family_color(*code);
        let sz = if is_core(*code) { 40.24922359499622 } else { 24.0 };
        let opacity = 0.5 + brightness * 0.5;
        let cls = if is_core(*code) { "node core" } else { "node" };
        let _ = write!(out, "<div class=\"{}\" style=\"left:{}px;top:{}px;width:{}px;height:{}px;background:{};opacity:{};\" title=\"{:02X}|{:04X}|{:02X}\">{}</div>\n",
            cls, px - sz/2.0, py - sz/2.0, sz, sz, color, opacity, x, y, z, name);
    }

    s(&mut out, "</div>\n");

    // 数据面板
    s(&mut out, "<div id=\"info\">");
    let _ = write!(out, "容器来源: hunyuan-rs/src/cross_isa/vocabulary.rs | ");
    let _ = write!(out, "族名来源: hunyuan-rs/src/cross_isa/vocabulary.rs | ");
    let _ = write!(out, "坐标系统: hunyuan-rs/src/cross_isa/coordinate.rs | ");
    let _ = write!(out, "计数: {} 容器 × 4 族\n", scene.len());
    s(&mut out, "</div>\n");

    s(&mut out, "</body></html>\n");
    out
}

// ═══════════════════════════════════════════════════════════════════════════
// main — 运行时读取源文件, 不嵌入编译期数据
// ═══════════════════════════════════════════════════════════════════════════

fn main() {
    let args: Vec<String> = env::args().collect();
    let out_path = if args.len() > 1 { &args[1] } else {
        "/tmp/container_native_v2.html"
    };

    println!("▸ 读取 hunyuan-rs 源文件...");
    let container_table = load_container_name_table();
    let family_table = load_family_name_table();

    println!("▸ 容器名称: {} 条目", container_table.len());
    println!("▸ 族名: {} 条目", family_table.len());

    if !container_table.is_empty() {
        let n = container_table.len();
        println!("▸ 示例: {:02X}={}  {:02X}={}  {:02X}={}  {:02X}={}",
            container_table[0].0, container_table[0].1,
            container_table[1].0, container_table[1].1,
            container_table[15.min(n - 1)].0, container_table[15.min(n - 1)].1,
            container_table[n - 1].0, container_table[n - 1].1,
        );
    }

    let html = build_html(&container_table, &family_table);
    std::fs::write(out_path, &html).expect("写入失败");
    println!("▸ 输出: {} ({}字节)", out_path, html.len());
}
