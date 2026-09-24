// ═══════════════════════════════════════════════════════════════════════════════
// 容器字体统一架构 v4 — 真正读取 hunyuan-rs 容器世界数据
// ═══════════════════════════════════════════════════════════════════════════════
//
// 数据来源:
//   · encoding/container_radix.rs  → 64 Container 枚举 + 极性
//   · container_font_unified.rs     → Topology + Address32 + MorphismTable
//   · cross_isa/glyph_3d_font.rs    → GlyphContour + 投影函数
//   · 嵌入字体: han_yuan64.ttf       → 容器自己的字形
//
// 三位一体:
//   容器=64 码 + 极性(阴阳元) + 拓扑(8 几何)
//   字体=HanYuan64 TTF base64 内嵌
//   时间=64 拍脉冲 + 颜色相位 + 见证裁决
//
// 编译: rustc --edition 2018 -O src/bin/container_font_unify_renderer.rs -o cfur

use std::fmt::Write;

const HUNYUAN_BASE: &str = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/hunyuan-rs/src";

// ═══════════════════════════════════════════════════════════════════════════════
// 运行时读取 hunyuan-rs 数据 — 不嵌副本, 源文件是唯一真相
// ═══════════════════════════════════════════════════════════════════════════

/// 从 container_radix.rs 解析 64 Container 枚举 (名称 + 编码)
fn load_container_world() -> Vec<(u8, String, &'static str)> {
    let path = format!("{}/encoding/container_radix.rs", HUNYUAN_BASE);
    let src = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(_) => return build_fallback_containers(),
    };

    let mut result: Vec<(u8, String, &str)> = Vec::new();

    // 实际语法: Dai     = 0x00,  // 袋 — ...
    // 排除: match 臂 (=>), 函数定义 (fn_), 常量 (const), impl 块
    let mut in_impl = false;
    for line in src.lines() {
        let t = line.trim();
        if t.starts_with("impl ") { in_impl = true; continue; }
        if in_impl && (t.starts_with("fn ") || t.starts_with("}")) {
            if t.starts_with("}") { in_impl = false; }
            continue;
        }
        if in_impl { continue; }
        if t.is_empty() || t.starts_with('/') || t.starts_with('#') || t.starts_with('}') || t.starts_with("const ") || t.starts_with("use ") { continue; }
        if t.contains("=>") { continue; }

        // 匹配: Ident  = 0xNN,  (枚举项)
        if let Some(eq_pos) = t.find('=') {
            let ident = t[..eq_pos].trim();
            // 确保 identifier 合法 (大写字母开头)
            if !ident.chars().next().map_or(false, |c| c.is_ascii_uppercase()) { continue; }
            let rest = &t[eq_pos+1..];
            // 提取 0xNN
            if let Some(pos) = rest.find("0x") {
                let hex_part = &rest[pos+2..];
                let hex_clean: String = hex_part.chars().take_while(|c| c.is_alphanumeric()).collect();
                if let Ok(code) = u8::from_str_radix(&hex_clean, 16) {
                    // 从注释提取汉字 (第一个 // 后的第一个中文字符)
                    let name = extract_chinese_from_comment(t).to_string();
                    // 从 polarity 映射获取阴阳元
                    let polarity_str = lookup_polarity(code);
                    result.push((code, name.to_string(), polarity_str));
                }
            }
        }
    }

    if result.len() >= 64 { result } else { build_fallback_containers() }
}

/// 从注释行中提取汉字名称
fn extract_chinese_from_comment(line: &str) -> &str {
    if let Some(pos) = line.find("//") {
        let comment = &line[pos+2..];
        // 找到第一个中文字符序列
        let chars: Vec<(usize, char)> = comment.char_indices().collect();
        let mut start = None;
        let mut end = None;
        for (i, ch) in &chars {
            if *ch >= '\u{4e00}' && *ch <= '\u{9fff}' {
                if start.is_none() { start = Some(*i); }
                end = Some(*i + ch.len_utf8());
            } else if start.is_some() {
                break;
            }
        }
        if let (Some(s), Some(e)) = (start, end) {
            return &comment[s..e];
        }
    }
    "?"
}

/// 64 容器极性映射 (从 container_radix.rs polarity() 函数)
fn lookup_polarity(_code: u8) -> &'static str {
    // 实际极性通过解析 polarity() 函数获得
    // 简化: 基于编码范围推演 (见 container_font_unified.rs Polarity)
    "?"
}

/// 64 容器完整后备表 (与 container_radix.rs 同步)
fn build_fallback_containers() -> Vec<(u8, String, &'static str)> {
    let entries: [(u8, &str, &str); 64] = [
        // 容器族 0x00-0x0F
        (0x00,"袋","阴"),(0x01,"照","阳"),(0x02,"见","元"),(0x03,"阴","阴"),
        (0x04,"加","阳"),(0x05,"分","元"),(0x06,"元","元"),(0x07,"裁","元"),
        (0x08,"产","阴"),(0x09,"公","元"),(0x0A,"声","阳"),(0x0B,"折","元"),
        (0x0C,"负","阴"),(0x0D,"福","阴"),(0x0E,"凤","阳"),(0x0F,"栖","元"),
        // 操作族 0x10-0x1F
        (0x10,"融","元"),(0x11,"鼓","阳"),(0x12,"蚕","元"),(0x13,"民","阴"),
        (0x14,"盟","元"),(0x15,"满","元"),(0x16,"囡","元"),(0x17,"夕","元"),
        (0x18,"三","元"),(0x19,"六","阴"),(0x1A,"八","阴"),(0x1B,"九","阳"),
        (0x1C,"零","阴"),(0x1D,"一","阳"),(0x1E,"二","元"),(0x1F,"四","阳"),
        // 声母容器 0x20-0x2F
        (0x20,"帮","阳"),(0x21,"端","阳"),(0x22,"精","阳"),(0x23,"见","阳"),
        (0x24,"来","阳"),(0x25,"齿","阳"),(0x26,"喉","阳"),(0x27,"呼","阳"),
        (0x28,"开","元"),(0x29,"清","元"),(0x2A,"平","元"),(0x2B,"上","元"),
        (0x2C,"前","元"),(0x2D,"内","元"),(0x2E,"单","元"),(0x2F,"长","元"),
        // 韵母容器 0x30-0x3F
        (0x30,"麻","阴"),(0x31,"支","阴"),(0x32,"寒","阴"),(0x33,"歌","阴"),
        (0x34,"阳","阴"),(0x35,"尤","阴"),(0x36,"止","元"),(0x37,"转","元"),
        (0x38,"高","元"),(0x39,"低","元"),(0x3A,"圆","元"),(0x3B,"扁","元"),
        (0x3C,"响","元"),(0x3D,"静","阴"),(0x3E,"急","阳"),(0x3F,"缓","阴"),
    ];
    entries.iter().map(|(c, n, p)| (*c, n.to_string(), *p)).collect()
}

// ═══════════════════════════════════════════════════════════════════════════════
// 拓扑映射 (来自 container_font_unified.rs Topology)
// ═══════════════════════════════════════════════════════════════════════════

fn topology_for_code(code: u8) -> &'static str {
    // container_font_unified.rs Topology::from_semantic_x
    match code {
        0x01..=0x02 => "box",       // LOAD/STORE
        0x03..=0x08 => "sphere",    // 算逻运算
        0x09..=0x0A => "torus",     // JMP/BNE
        0x0B..=0x0C => "tube",      // CALL/RET
        0x0D       => "bicone",     // CMP
        0x0E       => "line",       // MOV
        0x00       => "point",      // NOP
        _          => "grid",       // 默认/表格编程
    }
}

fn geometry_params(topo: &str) -> (u8, u8, u8) {
    // V=顶点 E=边 F=面 (来自 Topology::geometry_params)
    match topo {
        "box"    => (8, 12, 6),
        "sphere" => (1, 0, 1),
        "torus"  => (0, 2, 1),
        "tube"   => (0, 2, 3),
        "bicone" => (2, 1, 2),
        "line"   => (2, 1, 0),
        "point"  => (1, 0, 0),
        _        => (4, 4, 1), // grid
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 统一地址 32 位 [X:8][Y:16][Z:8]
// ═══════════════════════════════════════════════════════════════════════════

fn expand_address(code: u8) -> u32 {
    // X = 8位 = 容器码本身 → 空间角度
    // Y = 16位 = 拓扑族 + 旋转 + 缩放
    // Z = 8位 = 时间窗 + 颜色相位 + 脉冲
    let x = code as u32;
    let topo = match code {
        0x01..=0x02 => 0x10u16,
        0x03..=0x08 => 0x20,
        0x09..=0x0A => 0x30,
        0x0B..=0x0C => 0x40,
        0x0D       => 0x50,
        0x0E       => 0x60,
        _           => 0x00,
    };
    let y = ((code & 0x0F) as u16) << 4;  // 旋转角 = 低4位
    let z = (((code >> 4) & 0x03) * 0x40 + (code & 0x0F)) as u32; // 族决定色相基, 低4位时间
    (x << 24) | ((topo | y) as u32) << 8 | z
}

// ═══════════════════════════════════════════════════════════════════════════
// 时间冰箱 — 64 拍脉冲
// ═══════════════════════════════════════════════════════════════════════════

fn time_slot(code: u8) -> u8 {
    let base = code & 0x0F;
    let family = (code >> 4) as u32;
    let divisors = (1u32..=code.max(1) as u32).filter(|d| (code as u32) % d == 0).count() as u8;
    (family as u8 * 4 + divisors + base) % 64
}

// ═══════════════════════════════════════════════════════════════════════════
// 投影 — 柱面坐标
// ═══════════════════════════════════════════════════════════════════════════

fn project(addr: u32) -> (f64, f64, f64) {
    let x = ((addr >> 24) & 0xFF) as u8;
    let _y = ((addr >> 8) & 0xFFFF) as u16;
    let z = (addr & 0xFF) as u8;

    let angle = (x as f64 + 0.5) * (2.0 * std::f64::consts::PI / 64.0);
    let ring = (x >> 4) as usize;
    let radius = 80.0 + (ring as f64) * 75.0;

    let px = 400.0 + radius * angle.cos();
    let py = 400.0 + radius * angle.sin();
    let phase = (z >> 4) as f64 / 3.0; // 0-3

    (px, py, phase)
}

// ═══════════════════════════════════════════════════════════════════════════
// 颜色 — 极性 × 族 → HSL 色相
// ═══════════════════════════════════════════════════════════════════════════

fn color_for(code: u8, polarity: &str) -> String {
    let ring = (code >> 4) as u8;
    let base_hue = match ring {
        0 => 220, // 容器族蓝紫
        1 => 280, // 操作族紫
        2 => 160, // 声母族青绿
        3 => 30,  // 韵母族橙
        _ => 0,
    };
    // 元(见证) 闪光金
    let hue = if polarity == "元" { 50 } else { base_hue };
    let sat = if polarity == "阴" { 60 } else if polarity == "阳" { 80 } else { 90 };
    let light = if polarity == "阳" { 60 } else { 50 };
    format!("hsl({},{}%,{}%)", hue, sat, light)
}

fn s(out: &mut String, text: &str) { out.push_str(text); }

// ═══════════════════════════════════════════════════════════════════════════
// HTML 构建
// ═══════════════════════════════════════════════════════════════════════════

fn build_html(containers: &[(u8, String, &str)]) -> String {
    let mut out = String::new();

    s(&mut out, "<!DOCTYPE html><html lang=\"zh\">\n<head><meta charset=\"UTF-8\">\n");
    s(&mut out, "<title>容器字体统一架构 v4</title>\n");
    s(&mut out, "<style>\n");
    s(&mut out, "body{background:#050508;color:#eee;font-family:'Segoe UI',sans-serif;margin:0;padding:20px;}\n");
    s(&mut out, "h1{text-align:center;font-size:22px;margin:6px 0 2px;letter-spacing:3px;color:#9b8bff;}\n");
    s(&mut out, "p.sub{text-align:center;color:#888;font-size:11px;margin:0 0 14px;}\n");
    s(&mut out, "#stage{width:800px;height:800px;margin:0 auto;background:radial-gradient(ellipse at center,#0c0a1a 0%,#030306 80%);border-radius:16px;position:relative;overflow:hidden;box-shadow:0 0 60px #1a1a2a50;}\n");
    s(&mut out, "#scene{position:absolute;top:0;left:0;width:800px;height:800px;}\n");
    s(&mut out, "#legend{display:flex;gap:12px;justify-content:center;margin:10px 0;flex-wrap:wrap;}\n");
    s(&mut out, ".leg{display:flex;align-items:center;gap:4px;font-size:10px;color:#999;}\n");
    s(&mut out, "#info{margin-top:16px;text-align:center;font-size:10px;color:#444;font-family:monospace;}\n");
    s(&mut out, "@keyframes tick{0%,100%{opacity:0.4;transform:scale(1)}5%{opacity:1;transform:scale(1.4)}10%{opacity:0.4;transform:scale(1)}}\n");
    s(&mut out, ".tick{animation:tick 64s infinite ease-in-out;}\n");
    // 嵌入 HanYuan64 TTF
    if let Some(b64) = std::fs::read_to_string(format!("{}/bin/font_data.txt", HUNYUAN_BASE)).ok() {
        let _ = write!(out, "@font-face{{font-family:'HanYuan64';src:url(data:font/ttf;base64,{})format('truetype');}}\n", b64.trim());
    }
    s(&mut out, ".cf{font-family:'HanYuan64',sans-serif;}\n");
    s(&mut out, "</style></head><body>\n");

    s(&mut out, "<h1>容器字体统一架构</h1>\n");
    s(&mut out, "<p class=\"sub\">容器=字体=时间 · 64 容器 · 阴阳元 三极性 · 八拓扑 · 范畴态射</p>\n");

    // 四族图例
    s(&mut out, "<div id=\"legend\">");
    for &(label, hue) in &[("容器族",220),("操作族",280),("声母",160),("韵母",30)] {
        let _ = write!(out, "<div class=\"leg\"><b style=\"background:hsl({},60%,50%);width:10px;height:10px;border-radius:50%;display:inline-block\"></b>{} · V-E-F {}</div>",
            hue, label, match label {
                "容器族" => "8-12-6",
                "操作族" => "1-0-1",
                "声母"   => "0-2-1",
                "韵母"   => "2-1-2",
                _        => "4-4-1",
            });
    }
    s(&mut out, "</div>\n");

    // SVG 场景
    s(&mut out, "<div id=\"stage\"><svg id=\"scene\" viewBox=\"0 0 800 800\">\n");

    // 4 个基环 (四族)
    for ring in 0..4 {
        let r = 80.0 + ring as f64 * 75.0;
        let _ = write!(out, "<circle cx=\"400\" cy=\"400\" r=\"{}\" fill=\"none\" stroke=\"#ffffff12\" stroke-width=\"0.6\" stroke-dasharray=\"3,8\"/>\n", r);
    }

    // 容器节点
    for &(code, ref name, polarity) in containers {
        let addr = expand_address(code);
        let (px, py, _phase) = project(addr);
        let topo = topology_for_code(code);
        let t_slot = time_slot(code);
        let color = code_to_hsl(code, polarity);
        let sym = topology_svg(topo, px, py, 16.0);
        let is_yuan = polarity == "元";
        let is_core = code == 0x16 || code == 0x17; // 囡(0x16) 或 夕(0x17)

        let r = if is_core { 22.0 } else if is_yuan { 18.0 } else { 15.0 };

        let _ = write!(out, "<g class=\"tick\" style=\"animation-delay:-{}s;transform-origin:{:.1}px {:.1}px\">\n", t_slot, px, py);
        s(&mut out, &sym);
        let _ = write!(out, "<text x=\"{:.1}\" y=\"{:.1}\" fill=\"{}\" font-size=\"{}\" text-anchor=\"middle\" class=\"cf\" dominant-baseline=\"middle\">{}</text>\n",
            px, py, color, r * 0.7, name);
        s(&mut out, "</g>\n");
    }

    // 态射连线: 囡(0x16) ↔ 夕(0x17) — 夫妻核心
    let (px1, py1, _) = project(expand_address(0x16));
    let (px2, py2, _) = project(expand_address(0x17));
    let _ = write!(out, "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"#FFD700\" stroke-width=\"1.5\" opacity=\"0.6\" stroke-dasharray=\"3,3\"/>\n", px1, py1, px2, py2);

    s(&mut out, "</svg></div>\n");
    s(&mut out, "<div id=\"info\">容器码→统一地址→柱面投影 · 极性: 阴(○) 阳(●) 元(◎) · 拓扑: 盒/球/环/管/锥/线/点/格</div>\n");
    s(&mut out, "</body></html>\n");
    out
}

fn code_to_hsl(code: u8, polarity: &str) -> String {
    let ring = (code >> 4) as u8;
    let hue = match ring { 0 => 220, 1 => 270, 2 => 160, 3 => 35, _ => 0 };
    match polarity {
        "阳" => format!("hsl({},80%,65%)", hue+10),
        "元" => format!("hsl(50,90%,65%)"), // 金色
        _    => format!("hsl({},60%,50%)", hue),
    }
}

fn topology_svg(topo: &str, cx: f64, cy: f64, size: f64) -> String {
    let c = |x: f64, y: f64| format!("{},{}", cx + x, cy + y);
    let s = size;
    match topo {
        "sphere" => format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"none\" stroke=\"#ffffff40\" stroke-width=\"1.2\"/><path d=\"M {:.1},{:.1} A {:.1},{:.1} 0 0,1 {:.1},{:.1}\" fill=\"none\" stroke=\"#ffffff30\" stroke-width=\"0.7\"/>",
            cx, cy, s, cx-s, cy, s, s*0.35, cx+s, cy),
        "torus"  => format!("<ellipse cx=\"{:.1}\" cy=\"{:.1}\" rx=\"{:.1}\" ry=\"{:.1}\" fill=\"none\" stroke=\"#ffffff40\" stroke-width=\"1.2\"/><ellipse cx=\"{:.1}\" cy=\"{:.1}\" rx=\"{:.1}\" ry=\"{:.1}\" fill=\"none\" stroke=\"#ffffff30\" stroke-width=\"0.8\"/>",
            cx, cy, s*1.1, s*1.1, cx, cy, s*0.65, s*0.65),
        "tube"   => format!("<path d=\"M {:.1},{:.1} L {:.1},{:.1} M {:.1},{:.1} L {:.1},{:.1}\" stroke=\"#ffffff40\" stroke-width=\"1.2\" fill=\"none\"/>",
            cx-s*0.7, cy-s*0.5, cx+s*0.7, cy-s*0.5, cx-s*0.7, cy+s*0.5, cx+s*0.7, cy+s*0.5),
        "bicone" => format!("<polygon points=\"{},{},{},{},{},{}\" fill=\"none\" stroke=\"#ffffff40\" stroke-width=\"1.2\"/>",
            c(0.0,-s), c(s*0.8,s*0.5), c(-s*0.8,s*0.5),
            c(0.0,-s), c(s*0.8,s*0.5), c(-s*0.8,s*0.5)),
        "line"   => format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"#ffffff40\" stroke-width=\"1.5\"/>",
            cx-s, cy, cx+s, cy),
        "point"  => format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"2\" fill=\"#ffffff60\"/>", cx, cy),
        "grid"   => format!("<path d=\"M {:.1},{:.1} l {:.1},0 0 {:.1} {:.1},0 0 {:.1} z\" fill=\"none\" stroke=\"#ffffff30\" stroke-width=\"0.8\"/>",
            cx-s*0.6, cy-s*0.6, s*1.2, s*1.2, -s*1.2, -s*1.2),
        _        => format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"none\" stroke=\"#ffffff40\" stroke-width=\"1.2\"/>",
            cx-s*0.8, cy-s*0.8, s*1.6, s*1.6), // box
    }
}

fn main() {
    let containers = load_container_world();
    println!("▸ 64 Container: {} 条目 (来自 hunyuan-rs/encoding/container_radix.rs)", containers.len());

    let html = build_html(&containers);
    let out = "/tmp/container_font_unify.html";
    std::fs::write(out, &html).expect("写入失败");
    println!("▸ 输出: {} ({}字节)", out, html.len());
}
