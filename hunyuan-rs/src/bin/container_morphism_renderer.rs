// ═══════════════════════════════════════════════════════════════════════════════
// 容器 MorphismTable 渲染器 v5 — 范畴态射连通图 + 五族完整拓扑
// ═══════════════════════════════════════════════════════════════════════════════
//
// 读取 hunyuan-rs 源文件:
//   · encoding/container_radix.rs  → 80 容器枚举 (4×16 数据 + 16 笔画)
//   · container_font_unified.rs     → MorphismTable (态射规则) + Topology
//
// 五族布局:
//   第0环 0x00-0x0F: 容器族 (袋照见阴加分元裁产公声折负福凤栖)
//   第1环 0x10-0x1F: 操作族 (融鼓蚕民盟囡夕三六八九零一四)
//   第2环 0x20-0x2F: 声母容器 (帮端精来齿喉呼开清平上前后内单长)
//   第3环 0x30-0x3F: 韵母容器 (麻支寒歌阳尤止转高扁圆响静急缓)
//   第4环 0x40-0x4F: 笔画容器族 (桶罐盆盂壶盒瓶盏盎篓筐篮箩箕笼海)
//
// 态射类型 (三种):
//   允许  — 绿色实线
//   条件  — 金色虚线 (需极性匹配)
//   禁止  — 红色虚线
//
// 编译: rustc --edition 2018 -O src/bin/container_morphism_renderer.rs -o cmr

use std::fmt::Write;

const HUNYUAN_BASE: &str = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/hunyuan-rs/src";

// ═══════════════════════════════════════════════════════════════════════════
// 容器名称表 (80 项 = 5 环 × 16)
// ═══════════════════════════════════════════════════════════════════════════

fn load_80_containers() -> Vec<(u8, String, &'static str)> {
    let path = format!("{}/encoding/container_radix.rs", HUNYUAN_BASE);
    let src = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(_) => return build_fallback_80(),
    };

    let mut result: Vec<(u8, String, &str)> = Vec::new();
    let mut in_impl = false;

    for line in src.lines() {
        let t = line.trim();
        if t.starts_with("impl ") { in_impl = true; continue; }
        if in_impl {
            if t.starts_with("}") { in_impl = false; }
            continue;
        }
        if t.is_empty() || t.starts_with('/') || t.starts_with('#') || t.starts_with('}') ||
           t.starts_with("const ") || t.starts_with("use ") || t.starts_with("pub fn") ||
           t.starts_with("pub struct") || t.starts_with("pub enum") { continue; }
        if t.contains("=>") { continue; }

        if let Some(eq_pos) = t.find('=') {
            let ident = t[..eq_pos].trim();
            if ident.chars().next().map_or(false, |c| c.is_ascii_uppercase()) {
                let rest = &t[eq_pos+1..];
                if let Some(pos) = rest.find("0x") {
                    let hex_part = &rest[pos+2..];
                    let hex_clean: String = hex_part.chars().take_while(|c| c.is_alphanumeric()).collect();
                    if let Ok(code) = u8::from_str_radix(&hex_clean, 16) {
                        let name = extract_chinese(t).to_string();
                        let polarity = parse_polarity_in_line(t);
                        result.push((code, name, polarity));
                    }
                }
            }
        }
    }

    if result.len() >= 80 { result } else { build_fallback_80() }
}

fn extract_chinese(line: &str) -> &str {
    if let Some(pos) = line.find("//") {
        let comment = &line[pos+2..];
        let mut start = None;
        let mut end = None;
        for (i, ch) in comment.char_indices() {
            if ch >= '\u{4e00}' && ch <= '\u{9fff}' {
                if start.is_none() { start = Some(i); }
                end = Some(i + ch.len_utf8());
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

fn parse_polarity_in_line(line: &str) -> &'static str {
    // 从注释中的描述推断极性
    if line.contains("阳") { "阳" }
    else if line.contains("阴") { "阴" }
    else if line.contains("元") || line.contains("平衡") { "元" }
    else { "元" }  // 默认
}

fn build_fallback_80() -> Vec<(u8, String, &'static str)> {
    let entries: [(u8, &str, &str); 80] = [
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
        // 笔画容器族 0x40-0x4F
        (0x40,"桶","阳"),(0x41,"罐","阴"),(0x42,"盆","阳"),(0x43,"盂","阴"),
        (0x44,"壶","元"),(0x45,"盒","元"),(0x46,"瓶","元"),(0x47,"盏","阳"),
        (0x48,"盎","阴"),(0x49,"篓","阳"),(0x4A,"筐","阴"),(0x4B,"篮","阴"),
        (0x4C,"箩","阳"),(0x4D,"箕","阴"),(0x4E,"笼","阳"),(0x4F,"海","元"),
    ];
    entries.iter().map(|(c, n, p)| (*c, n.to_string(), *p)).collect()
}

// ═══════════════════════════════════════════════════════════════════════════
// MorphismTable (来自 container_font_unified.rs)
// ═══════════════════════════════════════════════════════════════════════════

struct Morphism {
    src_code: u8,
    dst_code: u8,
    kind: MorphKind,
}

#[derive(Clone, Copy)]
enum MorphKind { Allowed, Conditional, Denied, Unknown }

fn load_morphisms() -> Vec<Morphism> {
    // 直接从源码解析 add(源, 目标, 类型)
    let path = format!("{}/container_font_unified.rs", HUNYUAN_BASE);
    let src = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(_) => return vec![],
    };

    let mut morphisms = Vec::new();
    for line in src.lines() {
        let t = line.trim();
        if t.starts_with("self.add(") {
            if let Some(args_start) = t.find('(') {
                if let Some(args_end) = t.rfind(')') {
                    let args = &t[args_start+1..args_end];
                    let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
                    if parts.len() >= 3 {
                        // 源和目标是 Container::XXX 形式
                        // 需要从名字查编码
                        let src_name = parts[0].split("::").nth(1).unwrap_or(parts[0]);
                        let dst_name = parts[1].split("::").nth(1).unwrap_or(parts[1]);
                        let kind_str = parts[2];

                        // 解析 MorphKind
                        let kind = if kind_str.contains("Denied") { MorphKind::Denied }
                        else if kind_str.contains("Conditional") { MorphKind::Conditional }
                        else if kind_str.contains("Allowed") { MorphKind::Allowed }
                        else { MorphKind::Unknown };

                        // 名字转编码
                        let src_code = name_to_code(src_name);
                        let dst_code = name_to_code(dst_name);
                        morphisms.push(Morphism { src_code, dst_code, kind });
                    }
                }
            }
        }
    }
    morphisms
}

fn name_to_code(name: &str) -> u8 {
    // Container 枚举名 → 编码
    let table: [(&str, u8); 80] = [
        ("Dai",0x00),("Zhao",0x01),("Jian",0x02),("Yin",0x03),
        ("Jia",0x04),("Fen",0x05),("Yuan",0x06),("Cai",0x07),
        ("Chan",0x08),("Gong",0x09),("Sheng",0x0A),("Zhe",0x0B),
        ("Fu",0x0C),("Fú",0x0D),("Feng",0x0E),("Qi",0x0F),
        ("Rong",0x10),("Gu",0x11),("Can",0x12),("Min",0x13),
        ("Meng",0x14),("Man",0x15),("Nan",0x16),("Xi",0x17),
        ("San",0x18),("Liu",0x19),("Ba",0x1A),("Jiu",0x1B),
        ("Ling",0x1C),("Yi",0x1D),("Er",0x1E),("Sì",0x1F),
        ("Bang",0x20),("Duan",0x21),("Jing",0x22),("Jian2",0x23),
        ("Lai",0x24),("Chi",0x25),("Hou",0x26),("Hu",0x27),
        ("Kai",0x28),("Qing",0x29),("Ping",0x2A),("Shang",0x2B),
        ("Qian",0x2C),("Nei",0x2D),("Dan",0x2E),("Chang",0x2F),
        ("Ma",0x30),("Zhī",0x31),("Han",0x32),("Ge",0x33),
        ("Yang",0x34),("You",0x35),("Zhǐ",0x36),("Zhuan",0x37),
        ("Gao",0x38),("Dī",0x39),("Yuán",0x3A),("Bian",0x3B),
        ("Xiǎng",0x3C),("Jìng",0x3D),("Jí",0x3E),("Huan",0x3F),
        ("Tong",0x40),("Guan",0x41),("Pen",0x42),("Yu",0x43),
        ("Hu2",0x44),("He",0x45),("Ping2",0x46),("Zhan",0x47),
        ("Ang",0x48),("Lou",0x49),("Kuang",0x4A),("Lan",0x4B),
        ("Luo",0x4C),("Ji",0x4D),("Long",0x4E),("Hai",0x4F),
    ];
    for &(n, c) in &table {
        if n == name { return c; }
    }
    0
}

// ═══════════════════════════════════════════════════════════════════════════
// 投影 (5 环)
// ═══════════════════════════════════════════════════════════════════════════

fn expand_addr(code: u8) -> u32 {
    let x = code as u32;
    let topo: u16 = match code >> 4 {
        0 => 0x10, 1 => 0x20, 2 => 0x30, 3 => 0x30, 4 => 0x00, _ => 0x00,
    };
    let y = ((code & 0x0F) as u16) << 4;
    let z = ((code >> 4) as u32) * 6 + (code as u32 & 0x0F);
    (x << 24) | ((topo | y) as u32) << 8 | z
}

fn project(addr: u32) -> (f64, f64) {
    let x = ((addr >> 24) & 0xFF) as u8;
    let angle = (x as f64 + 0.5) * (2.0 * std::f64::consts::PI / 80.0);
    let ring = (x >> 4) as usize;
    let radius = 50.0 + (ring as f64) * 65.0;
    let px = 400.0 + radius * angle.cos();
    let py = 400.0 + radius * angle.sin();
    (px, py)
}

fn time_slot(code: u8) -> u8 { (code + (code >> 3)) % 64 }

fn s(out: &mut String, text: &str) { out.push_str(text); }

fn code_to_hsl(code: u8, polarity: &str) -> String {
    let ring = (code >> 4) as u8;
    let hue = match ring {
        0 => 220, // 容器族蓝紫
        1 => 280, // 操作族紫
        2 => 160, // 声母族青绿
        3 => 35,  // 韵母族橙
        4 => 100, // 笔画族绿
        _ => 0,
    };
    match polarity {
        "阳" => format!("hsl({},80%,65%)", hue+15),
        "元" => format!("hsl(50,90%,65%)"),
        _    => format!("hsl({},60%,50%)", hue),
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// HTML 构建
// ═══════════════════════════════════════════════════════════════════════════

fn build_html(containers: &[(u8, String, &str)], morphisms: &[Morphism]) -> String {
    let mut out = String::new();
    s(&mut out, "<!DOCTYPE html><html lang=\"zh\">\n<head><meta charset=\"UTF-8\">\n");
    s(&mut out, "<title>容器 MorphismTable v5 — 范畴态射连通图</title>\n");
    s(&mut out, "<style>");
    s(&mut out, "body{background:#050508;color:#eee;font-family:'Segoe UI',sans-serif;margin:0;padding:20px;}\n");
    s(&mut out, "h1{text-align:center;font-size:22px;margin:6px 0 2px;letter-spacing:3px;color:#9b8bff;}\n");
    s(&mut out, "p.sub{text-align:center;color:#888;font-size:11px;margin:0 0 12px;}\n");
    s(&mut out, "#stage{width:800px;height:800px;margin:0 auto;background:radial-gradient(ellipse at center,#0c0a1a 0%,#030306 80%);border-radius:16px;position:relative;overflow:hidden;}\n");
    s(&mut out, "#scene{position:absolute;top:0;left:0;width:800px;height:800px;}\n");
    s(&mut out, "#legend{display:flex;gap:12px;justify-content:center;margin:10px 0;flex-wrap:wrap;font-size:10px;}\n");
    s(&mut out, ".leg{display:flex;align-items:center;gap:4px;color:#999;}\n");
    s(&mut out, "@keyframes tick{0%,100%{opacity:0.4}5%{opacity:1}10%{opacity:0.4}}\n");
    s(&mut out, ".tick{animation:tick 64s infinite ease-in-out;}\n");

    // 嵌入 TTF字体
    if let Some(b64) = std::fs::read_to_string(format!("{}/bin/font_data.txt", HUNYUAN_BASE)).ok() {
        let _ = write!(out, "@font-face{{font-family:'HanYuan64';src:url(data:font/ttf;base64,{})format('truetype');}}\n", b64.trim());
    }
    s(&mut out, ".cf{font-family:'HanYuan64',sans-serif;}\n");
    s(&mut out, "#info{margin-top:14px;text-align:center;font-size:10px;color:#444;font-family:monospace;}\n");
    s(&mut out, "</style></head><body>\n");

    s(&mut out, "<h1>容器 MorphismTable · 范畴态射连通图</h1>\n");
    s(&mut out, "<p class=\"sub\">五族 × 16 = 80 容器 · 允许(绿实线) · 条件(金虚线) · 禁止(红虚线)</p>\n");

    // 图例
    s(&mut out, "<div id=\"legend\">");
    s(&mut out, "<div class=\"leg\"><b style=\"background:#4CAF50;width:18px;height:2px;display:inline-block\"></b>允许</div>");
    s(&mut out, "<div class=\"leg\"><b style=\"background:#FFD700;width:18px;height:2px;display:inline-block;border-top:2px dashed #FFD700\"></b>条件</div>");
    s(&mut out, "<div class=\"leg\"><b style=\"background:#FF4444;width:18px;height:2px;display:inline-block\"></b>禁止</div>");
    for &(label, hue) in &[("容器族",220),("操作族",280),("声母",160),("韵母",35),("笔画",100)] {
        let _ = write!(out, "<div class=\"leg\"><b style=\"background:hsl({},60%,50%);width:10px;height:10px;border-radius:50%;display:inline-block\"></b>{}</div>",
            hue, label);
    }
    s(&mut out, "</div>\n");

    // SVG
    s(&mut out, "<div id=\"stage\"><svg id=\"scene\" viewBox=\"0 0 800 800\">\n");

    // 5 个基环
    for ring in 0..5 {
        let r = 50.0 + ring as f64 * 65.0;
        let _ = write!(out, "<circle cx=\"400\" cy=\"400\" r=\"{}\" fill=\"none\" stroke=\"#ffffff10\" stroke-width=\"0.6\" stroke-dasharray=\"3,6\"/>\n", r);
    }

    // 态射线 (先画, 在最底层)
    for m in morphisms {
        let src_addr = expand_addr(m.src_code);
        let dst_addr = expand_addr(m.dst_code);
        let (sx, sy) = project(src_addr);
        let (dx, dy) = project(dst_addr);

        let (color, dash) = match m.kind {
            MorphKind::Allowed => ("#4CAF50", ""),
            MorphKind::Conditional => ("#FFD700", "stroke-dasharray=\"6,3\""),
            MorphKind::Denied => ("#FF4444", "stroke-dasharray=\"3,3\""),
            MorphKind::Unknown => ("#666666", "stroke-dasharray=\"2,4\""),
        };

        // 态射带箭头
        let mid_x = (sx + dx) / 2.0;
        let mid_y = (sy + dy) / 2.0;
        let _ = write!(out, "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-width=\"1.5\" opacity=\"0.7\" {}>\n",
            sx, sy, mid_x, mid_y, color, dash);
        // 动画: 流动效果
        s(&mut out, "<animate attributeName=\"stroke-dashoffset\" values=\"0;20\" dur=\"3s\" repeatCount=\"indefinite\"/>\n");
        s(&mut out, "</line>\n");
        let _ = write!(out, "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-width=\"0.8\" opacity=\"0.5\" stroke-dasharray=\"2,8\">\n",
            mid_x, mid_y, dx, dy, color);
        s(&mut out, "<animate attributeName=\"stroke-dashoffset\" values=\"0;-20\" dur=\"3s\" repeatCount=\"indefinite\"/>\n");
        s(&mut out, "</line>\n");

        // 中点标记 (条件态射特别标记)
        match m.kind {
            MorphKind::Conditional => {
                let _ = write!(out, "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"3\" fill=\"{}\" opacity=\"0.8\"/>\n", mid_x, mid_y, color);
            }
            MorphKind::Denied => {
                // 禁止标记 (×)
                let nx = (dy - sy) * 0.15;
                let ny = (sx - dx) * 0.15;
                let _ = write!(out, "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-width=\"1.5\"/>\n",
                    mid_x-nx, mid_y-ny, mid_x+nx, mid_y+ny, color);
                let _ = write!(out, "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-width=\"1.5\"/>\n",
                    mid_x+nx, mid_y-ny, mid_x-nx, mid_y+ny, color);
            }
            _ => {}
        }
    }

    // 80 容器节点
    for &(code, ref name, polarity) in containers {
        let addr = expand_addr(code);
        let (px, py) = project(addr);
        let color = code_to_hsl(code, polarity);
        let t_slot = time_slot(code);
        let is_core = code == 0x16 || code == 0x17;
        let is_polarity_元 = polarity == "元";
        let r = if is_core { 22.0 } else if is_polarity_元 { 17.0 } else { 14.0 };

        let opacity = match polarity { "阳" => 1.0, "元" => 0.95, _ => 0.85 };

        let _ = write!(out, "<g class=\"tick\" style=\"animation-delay:-{}s;transform-origin:{:.1}px {:.1}px\">\n", t_slot, px, py);

        // 光晕 (阳更亮)
        if polarity == "阳" || is_core {
            let _ = write!(out, "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"{}\" opacity=\"0.15\"/>\n",
                px, py, r * 2.0, color);
        }

        // 节点圆
        let _ = write!(out, "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"{}\" opacity=\"{:.2}\"/>\n",
            px, py, r, color, opacity);

        // 名称
        let _ = write!(out, "<text x=\"{:.1}\" y=\"{:.1}\" fill=\"{}\" font-size=\"{:.1}\" text-anchor=\"middle\" dominant-baseline=\"middle\" class=\"cf\">{}</text>\n",
            px, py, color, (r * 0.75_f64).max(7.0), name);

        s(&mut out, "</g>\n");
    }

    s(&mut out, "</svg></div>\n");
    
    // 态射列表
    s(&mut out, "<div id=\"info\">态射表:<br>");
    for m in morphisms {
        let src_name = containers.iter().find(|(c,_,_)| *c == m.src_code).map(|(_,n,_)| n.as_str()).unwrap_or("?");
        let dst_name = containers.iter().find(|(c,_,_)| *c == m.dst_code).map(|(_,n,_)| n.as_str()).unwrap_or("?");
        let kind_str = match m.kind {
            MorphKind::Allowed => "允许",
            MorphKind::Conditional => "条件",
            MorphKind::Denied => "禁止",
            MorphKind::Unknown => "未知",
        };
        let _ = write!(out, "{}→{}: {}<br>", src_name, dst_name, kind_str);
    }
    s(&mut out, "</div>\n");
    s(&mut out, "</body></html>\n");

    out
}

fn main() {
    let containers = load_80_containers();
    let morphisms = load_morphisms();
    println!("▸ 80 Container: {} 条目", containers.len());
    println!("▸ Morphisms: {} 条态射", morphisms.len());

    let html = build_html(&containers, &morphisms);
    let out = "/tmp/container_morphism.html";
    std::fs::write(out, &html).expect("写入失败");
    println!("▸ 输出: {} ({}字节)", out, html.len());
}
