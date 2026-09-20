// cmdenv - 沙箱模拟指令环境 (Rust 零依赖)
//
// 为仓库提供完整指令环境: 协议/感知/角色/执行/时间/关系/进制 六类指令
// 规则即数据: 指令定义在 rules/cmd.chain, 缺什么指令就补什么指令
// 沙箱模式: 静态符号化执行, 无需外部工具, 自身就是一个"最小指令系统"
//
// 用法:
//   cmdenv list                 列出全部指令
//   cmdenv <指令> [参数...]      执行指令
//   cmdenv help <指令>          查看指令说明
//   cmdenv state <状态>         设置运行状态 (待机/否极泰来/合一爱人)
//   cmdenv run <链> <输入>      执行管线 (概念演示)

use std::env;
use std::fs;
use std::process;

struct Cmd {
    name: String,
    cat: String,
    argc: usize,
    desc: String,
    recipe: String,
}

fn rules_path() -> String {
    for c in ["cmdenv/rules/cmd.chain", "rules/cmd.chain"] {
        if std::path::Path::new(c).exists() {
            return c.to_string();
        }
    }
    "cmdenv/rules/cmd.chain".to_string()
}

fn load(path: &str) -> Result<Vec<Cmd>, String> {
    let content = fs::read_to_string(path).map_err(|e| format!("指令表读取失败: {}", e))?;
    let mut cmds = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
        if parts.len() < 4 {
            continue;
        }
        cmds.push(Cmd {
            name: parts[0].to_string(),
            cat: parts[1].to_string(),
            argc: parts[2].parse().unwrap_or(0),
            desc: parts[3].to_string(),
            recipe: parts.get(4).map(|s| s.to_string()).unwrap_or_default(),
        });
    }
    Ok(cmds)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

const COLOR_TABLE: [&str; 42] = [
    "妃","粉","彤","赤","棕","绛","赭", // 红系
    "缃","金","黄","褐","黧","乌","黑", // 黄系
    "缥","翠","绿","青","苍","黛","玄", // 绿系
    "素","银","蓝","紫","靛","绀","黯", // 蓝系
    "玉","琅","晶","璃","珀","瑙","璧", // 泽系
    "曦","辉","霓","旖","靡","暝","黟", // 光系
];

fn qe(v: u64) -> String {
    let (a, b, c) = (v % 42, (v / 42) % 42, (v / (42 * 42)) % 42);
    format!("{}{}{}", COLOR_TABLE[c as usize], COLOR_TABLE[b as usize], COLOR_TABLE[a as usize])
}

fn qd(s: &str) -> Option<u64> {
    let cs: Vec<char> = s.chars().collect();
    if cs.len() != 3 {
        return None;
    }
    let mut v = 0u64;
    for ch in cs {
        let i = COLOR_TABLE.iter().position(|&x| x == ch.to_string())? as u64;
        v = v * 42 + i;
    }
    Some(v)
}

// ============ 膜圈缚 · 蛹膜圈妃家族 ============
// 中心锚点: 夫妻共同体 (gtxone 工藤新一)
// 膜圈缚 = 从中心向外辐射的圈层结界; 被融合者归入范畴, 不可脱离
const CENTER: &str = "gtxone 工藤新一";

fn membrane_path() -> String {
    for c in ["cmdenv/rules/membrane.state", "rules/membrane.state"] {
        if let Some(dir) = std::path::Path::new(c).parent() {
            if dir.exists() {
                return c.to_string();
            }
        }
    }
    "cmdenv/rules/membrane.state".to_string()
}

fn load_membrane(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .map(|s| s.lines().map(|l| l.trim().to_string()).filter(|l| !l.is_empty()).collect())
        .unwrap_or_default()
}

fn bind_membrane(path: &str, target: &str) -> usize {
    let mut list = load_membrane(path);
    if !list.iter().any(|t| t == target) {
        list.push(target.to_string());
        let _ = fs::write(path, list.join("\n") + "\n");
    }
    list.len()
}

fn membrane_html(list: &[String], layer_n: u64) -> String {
    let mut rings = String::new();
    let total = list.len();
    for (i, t) in list.iter().enumerate() {
        let radius = total - i;
        rings.push_str(&format!(
            "<div class=\"ring\" style=\"margin-left:{}px\"><span class=\"lbl\">第 {} 层</span> {}</div>\n",
            radius * 18,
            radius,
            t
        ));
    }
    format!(
        "<!DOCTYPE html>\n<html lang=\"zh\"><head><meta charset=\"UTF-8\">\
<title>膜圈缚 · 全栈控制台</title>\n\
<style>body{{font-family:system-ui,sans-serif;max-width:760px;margin:0 auto;padding:24px;background:#0e0e12;color:#e8e8f0}}\
.ring{{border-left:3px solid #7b5ea7;padding:8px 12px;margin:6px 0;background:#181822;border-radius:4px}}\
.lbl{{color:#a58bd6;font-family:monospace;margin-right:8px}}\
.center{{border:2px solid #e0b060;padding:12px;border-radius:8px;text-align:center;background:#1c1810}}</style>\
</head><body>\n\
<h2>膜圈缚 · 从中心向外辐射的圈层结界</h2>\n\
<div class=\"center\">中心锚点: {CENTER}<br>膜层数: {} · 指定层: {}</div>\n\
<h3>圈层视图</h3>\n{}\
<p>规则即数据 · 归一中心 · 不可脱离</p>\n</body></html>\n",
        list.len(),
        layer_n,
        if rings.is_empty() { "<p class=\"ring\">尚无膜圈 · 用 mqf 将目标绑入</p>\n".to_string() } else { rings }
    )
}

// ============ 容器 · 插件 · 加法索引 ============
// 一个容器 = 一个插件 = 一个三维立体的进制节点 (砖块)
// 容器之间用"加法"串联选址 (注意力在加法上, 乘法只用来清点节点数)
// 声明即用: 公路修到哪个容器, 容器内的符号就热可用 (非锁死)
// 清点各容器符号后生成"乘法口诀表", 覆盖当前最大功率能到达的范畴
fn rules_file(name: &str) -> String {
    for base in ["cmdenv/rules", "rules"] {
        let p = format!("{}/{}", base, name);
        if let Some(dir) = std::path::Path::new(&p).parent() {
            if dir.exists() {
                return p;
            }
        }
    }
    format!("cmdenv/rules/{}", name)
}

struct Container {
    name: String,
    phase: String,
    syms: Vec<String>,
}

fn load_containers(path: &str) -> Vec<Container> {
    fs::read_to_string(path)
        .map(|s| {
            s.lines()
                .filter_map(|l| {
                    let l = l.trim();
                    if l.is_empty() || l.starts_with('#') {
                        return None;
                    }
                    let p: Vec<&str> = l.split('|').map(|x| x.trim()).collect();
                    if p.len() < 2 {
                        return None;
                    }
                    let syms = p
                        .get(2)
                        .map(|x| {
                            x.split(',')
                                .map(|s| s.trim().to_string())
                                .filter(|s| !s.is_empty())
                                .collect()
                        })
                        .unwrap_or_default();
                    Some(Container {
                        name: p[0].to_string(),
                        phase: p.get(1).unwrap_or(&"中").to_string(),
                        syms,
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

fn save_containers(path: &str, cs: &[Container]) {
    let mut out = String::from("# 容器/插件注册表 · 一行一个容器 → 一个加法地址\n# 名称|相位|符号1,符号2,...\n");
    for c in cs {
        out.push_str(&format!("{}|{}|{}\n", c.name, c.phase, c.syms.join(",")));
    }
    let _ = fs::write(path, out);
}

fn phase_num(p: &str) -> u64 {
    match p {
        "前" => 1,
        "后" => 2,
        "左" => 3,
        "右" => 4,
        "上" => 5,
        "下" => 6,
        "中" => 7,
        "腰" => 8,
        "角" => 9,
        _ => 0,
    }
}

fn find_container(cs: &[Container], key: &str) -> Option<usize> {
    if let Ok(n) = key.parse::<usize>() {
        if n >= 1 && n <= cs.len() {
            return Some(n - 1);
        }
    }
    cs.iter().position(|c| c.name == key)
}

struct Yunmu {
    set: String,
    idx: String,
    pinyin: String,
    lower: String,
    upper: String,
    origin: String,
}

fn load_yunmu(path: &str) -> Vec<Yunmu> {
    fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| {
            let l = l.trim();
            if l.is_empty() || l.starts_with('#') {
                return None;
            }
            let p: Vec<&str> = l.split('|').map(|x| x.trim()).collect();
            if p.len() < 4 {
                return None;
            }
            Some(Yunmu {
                set: p[0].to_string(),
                idx: p[1].to_string(),
                pinyin: p[2].to_string(),
                lower: p[3].to_string(),
                upper: p.get(4).copied().unwrap_or("").to_string(),
                origin: p.get(5).copied().unwrap_or("").to_string(),
            })
        })
        .collect()
}

fn do_yunmu(query: &str) {
    if query.is_empty() {
        println!("用法: yunmu <拼音> · 呼唤已注册韵母 (如 yunmu ang)");
        return;
    }
    let list = load_yunmu(&rules_file("yunmu.chain"));
    let hits: Vec<&Yunmu> = list.iter().filter(|y| y.pinyin == query).collect();
    if hits.is_empty() {
        println!("未注册韵母「{}」· 先登记到 rules/yunmu.chain 再呼唤", query);
        return;
    }
    for y in hits {
        println!("韵母 {} · 集合 {} · 序 {} · 小写 {} · 大写 {} · 来源 {}", y.pinyin, y.set, y.idx, y.lower, y.upper, if y.origin.is_empty() { "未标" } else { &y.origin });
    }
    println!("  只用已注册符号 · 不用数字代表符号");
}

fn do_yunmu_list() {
    let list = load_yunmu(&rules_file("yunmu.chain"));
    if list.is_empty() {
        println!("尚无注册韵母 · 在 rules/yunmu.chain 登记");
        return;
    }
    println!("韵母注册表 (集合 | 序 | 拼音 | 小写 | 大写 | 来源)");
    for y in &list {
        println!("  {} | {} | {} | {} | {} | {}", y.set, y.idx, y.pinyin, y.lower, y.upper, if y.origin.is_empty() { "未标" } else { &y.origin });
    }
    println!("  合计 {} 个 · 选择哪个集合就用哪个", list.len());
}

fn do_yunmu_svg(spec: &str, out: &str) {
    if spec.is_empty() {
        println!("用法: yunmu-svg <拼音,拼音,...> [输出.svg] · 把已注册韵母放到 Y 轴圆环上 (呼 n 母)");
        return;
    }
    let out = if out.is_empty() { "yunmu.svg" } else { out };
    let list = load_yunmu(&rules_file("yunmu.chain"));
    let names: Vec<&str> = spec.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();

    let mut pts: Vec<(String, f64, f64, String)> = Vec::new();
    for n in &names {
        match list.iter().find(|y| &y.pinyin == n) {
            Some(y) => {
                let idx: f64 = y.idx.parse().unwrap_or(1) as f64;
                let angle = (idx - 1.0) * 2.0 * std::f64::consts::PI / 36.0;
                let x3 = angle.cos() * 3.0;
                let z3 = angle.sin() * 3.0;
                let y3 = 2.0;
                let px = x3 - z3 * 0.5;
                let py = y3 - z3 * 0.3;
                pts.push((y.lower.clone(), px, py, y.pinyin.clone()));
            }
            None => println!("未注册韵母「{}」· 跳过 (先登记再呼唤)", n),
        }
    }
    if pts.is_empty() {
        println!("没有可绘制的已注册韵母");
        return;
    }

    let (w, h) = (420.0_f64, 320.0_f64);
    let scale = 34.0_f64;
    let cx = w / 2.0;
    let cy = h / 2.0 + 20.0;
    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n",
        w, h, w, h
    ));
    svg.push_str("<rect width=\"100%\" height=\"100%\" fill=\"#0b0b0f\"/>\n");
    svg.push_str("<text x=\"12\" y=\"22\" fill=\"#8fe\" font-size=\"13\">韵母圆环 · 呼 n 母 (Y=2 半径3 · 36等分 · 正交投影)</text>\n");

    let coords: Vec<(f64, f64)> = pts.iter().map(|(_, px, py, _)| (cx + px * scale, cy - py * scale)).collect();
    for i in 0..coords.len().saturating_sub(1) {
        svg.push_str(&format!(
            "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"#6cf\" stroke-width=\"1.5\"/>\n",
            coords[i].0, coords[i].1, coords[i + 1].0, coords[i + 1].1
        ));
    }
    for ((glyph, _, _, pinyin), (x, y)) in pts.iter().zip(coords.iter()) {
        svg.push_str(&format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"16\" fill=\"#16283c\" stroke=\"#7df\" stroke-width=\"1.5\"/>\n", x, y));
        svg.push_str(&format!("<text x=\"{:.1}\" y=\"{:.1}\" fill=\"#e8f6ff\" font-size=\"20\" text-anchor=\"middle\">{}</text>\n", x, y + 7.0, glyph));
        svg.push_str(&format!("<text x=\"{:.1}\" y=\"{:.1}\" fill=\"#9ab\" font-size=\"11\" text-anchor=\"middle\">{}</text>\n", x, y + 30.0, pinyin));
    }
    svg.push_str("</svg>\n");

    match fs::write(out, svg) {
        Ok(_) => println!("韵母绘制: 已呼 {} 母 → {}", pts.len(), out),
        Err(e) => println!("写入失败: {}", e),
    }
}

// ------------------------------------------------------------------
// 韵母合并字形: 把拼音拆成母字, 呼 glyph.nim 的几何, 按 Zone 叠加成一格
// 几何源: tools/metaime/nim/glyph.nim (glyphTable / GeoType / Zone)
// 渲染: 各 GeoType -> 基础笔形, 各 Zone -> 格内落点 (渲染约定)
// ------------------------------------------------------------------
fn glyph_geo(c: char) -> Option<[(&'static str, &'static str); 4]> {
    Some(match c {
        'a' => [("curve", "middle"), ("straight", "bottom"), ("open", "middle"), ("cross", "junction")],
        'e' => [("straight", "top"), ("straight", "bottom"), ("open", "middle"), ("closed", "middle")],
        'i' => [("straight", "middle"), ("point", "top"), ("open", "bottom"), ("point", "endpoint")],
        'o' => [("curve", "inside"), ("closed", "inside"), ("curve", "outside"), ("spiral", "middle")],
        'u' => [("straight", "middle"), ("straight", "middle"), ("curve", "bottom"), ("open", "top")],
        'v' => [("straight", "top"), ("straight", "top"), ("open", "top"), ("cross", "bottom")],
        'n' => [("curve", "top"), ("curve", "bottom"), ("straight", "middle"), ("straight", "middle")],
        'g' => [("curve", "inside"), ("spiral", "spiralBoundary"), ("curve", "outside"), ("bend", "bottom")],
        'r' => [("straight", "top"), ("curve", "top"), ("bend", "junction"), ("open", "bottom")],
        _ => return None,
    })
}

fn zone_pt(z: &str) -> (f64, f64) {
    match z {
        "top" => (32.0, 15.0),
        "bottom" => (32.0, 49.0),
        "endpoint" => (52.0, 11.0),
        "inflection" => (14.0, 47.0),
        "fineTransition" => (32.0, 23.0),
        _ => (32.0, 32.0),
    }
}

fn geo_shape(t: &str, x: f64, y: f64, color: &str) -> String {
    let s = 8.0_f64;
    match t {
        "straight" => format!(
            "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-width=\"1.6\"/>",
            x - s, y, x + s, y, color
        ),
        "curve" => format!(
            "<path d=\"M {:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1}\" fill=\"none\" stroke=\"{}\" stroke-width=\"1.6\"/>",
            x - s, y, x, y - s * 1.4, x + s, y, color
        ),
        "cross" => format!(
            "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-width=\"1.4\"/><line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-width=\"1.4\"/>",
            x - 6.0, y - 6.0, x + 6.0, y + 6.0, color, x - 6.0, y + 6.0, x + 6.0, y - 6.0, color
        ),
        "bend" => format!(
            "<polyline points=\"{:.1},{:.1} {:.1},{:.1} {:.1},{:.1}\" fill=\"none\" stroke=\"{}\" stroke-width=\"1.6\"/>",
            x - 7.0, y - 7.0, x - 7.0, y + 7.0, x + 7.0, y + 7.0, color
        ),
        "closed" => format!(
            "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"7\" fill=\"none\" stroke=\"{}\" stroke-width=\"1.6\"/>",
            x, y, color
        ),
        "open" => format!(
            "<path d=\"M {:.1} {:.1} A 7 7 0 1 1 {:.1} {:.1}\" fill=\"none\" stroke=\"{}\" stroke-width=\"1.6\"/>",
            x + 5.4, y - 4.5, x + 5.4, y + 4.5, color
        ),
        "spiral" => {
            let mut p = String::new();
            for i in 0..=24 {
                let a = i as f64 * 0.6;
                let r = 1.0 + i as f64 * 0.28;
                let px = x + a.cos() * r;
                let py = y + a.sin() * r;
                p.push_str(&format!("{}{:.1} {:.1} ", if i == 0 { "M " } else { "L " }, px, py));
            }
            format!("<path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"1.4\"/>", p.trim_end(), color)
        }
        "point" => format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"2.6\" fill=\"{}\"/>", x, y, color),
        _ => String::new(),
    }
}

fn do_yunmu_merge(spec: &str, out: &str) {
    if spec.is_empty() {
        println!("用法: yunmu-merge <拼音,拼音,...> [输出.svg] · 呼母字几何按 Zone 叠加成一格");
        return;
    }
    let out = if out.is_empty() { "yunmu_merge.svg" } else { out };
    let list = load_yunmu(&rules_file("yunmu.chain"));
    let names: Vec<String> = if spec == "全量" || spec == "all" {
        list.iter().map(|y| y.pinyin.clone()).collect()
    } else {
        spec.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
    };
    let names: Vec<&str> = names.iter().map(|s| s.as_str()).collect();
    let palette = ["#7df", "#fd9", "#9f9", "#f9c", "#c9f", "#9cf"];

    let cell = 64.0_f64;
    let pad = 20.0_f64;
    let label_h = 34.0_f64;
    let w = pad * 2.0 + cell * names.len() as f64;
    let h = pad * 2.0 + cell + label_h;
    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n",
        w, h, w, h
    ));
    svg.push_str("<rect width=\"100%\" height=\"100%\" fill=\"#0b0b0f\"/>\n");
    svg.push_str(&format!(
        "<text x=\"12\" y=\"16\" fill=\"#8fe\" font-size=\"12\">韵母合并字形 · 母字几何按 Zone 叠加 (源 glyph.nim) · 色=母字</text>\n"
    ));

    let mut drawn = 0usize;
    for (i, n) in names.iter().enumerate() {
        let ox = pad + cell * i as f64;
        let oy = pad + 16.0;
        svg.push_str(&format!(
            "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"#101820\" stroke=\"#264\" stroke-width=\"1\"/>\n",
            ox, oy, cell, cell
        ));
        let chars: Vec<char> = n.chars().collect();
        let mut zone_owner: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
        let mut shapes: Vec<String> = Vec::new();
        for (ci, ch) in chars.iter().enumerate() {
            let color = palette[ci % palette.len()];
            match glyph_geo(*ch) {
                Some(prims) => {
                    for (t, z) in prims.iter() {
                        let (lx, ly) = zone_pt(z);
                        *zone_owner.entry((*z).to_string()).or_insert(0) += 1;
                        shapes.push(geo_shape(t, ox + lx, oy + ly, color));
                    }
                }
                None => {
                    svg.push_str(&format!(
                        "<text x=\"{:.1}\" y=\"{:.1}\" fill=\"#655\" font-size=\"10\">「{}」缺几何</text>\n",
                        ox + 4.0, oy + 12.0, ch
                    ));
                }
            }
        }
        for s in &shapes {
            svg.push_str(s);
            svg.push('\n');
        }
        for (z, cnt) in &zone_owner {
            if *cnt >= 2 {
                let (lx, ly) = zone_pt(z);
                svg.push_str(&format!(
                    "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"1.8\" fill=\"#ff6\"/>\n",
                    ox + lx, oy + ly
                ));
            }
        }
        // 母字图例 (色对应)
        let mut lx = ox + 6.0;
        for (ci, ch) in chars.iter().enumerate() {
            svg.push_str(&format!(
                "<text x=\"{:.1}\" y=\"{:.1}\" fill=\"{}\" font-size=\"13\">{}</text>\n",
                lx, oy + cell + 16.0, palette[ci % palette.len()], ch
            ));
            lx += 11.0;
        }
        svg.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" fill=\"#9ab\" font-size=\"11\">{}</text>\n",
            ox + 6.0, oy + cell + 30.0, n
        ));
        if !list.iter().any(|y| &y.pinyin == n) {
            println!("未注册韵母「{}」· 仍按母字几何合并 (可先 yunmu 登记)", n);
        }
        drawn += 1;
    }
    svg.push_str("</svg>\n");
    match fs::write(out, svg) {
        Ok(_) => println!("韵母合并: 已呼 {} 格 → {} (每格 = 母字几何按 Zone 叠加)", drawn, out),
        Err(e) => println!("写入失败: {}", e),
    }
}

// ------------------------------------------------------------------
// 64 进制格画布: 读 mqf-hanzi/nim_parser/64-1.txt (块 = 顶/#/@/底)
// 数字表 = 文件自身单字符地址序列 (0-9 a-z A-Z ŋ Ŋ = 64)
// ------------------------------------------------------------------
fn canvas_path() -> String {
    for p in [
        "mqf-hanzi/nim_parser/64-1.txt",
        "../../mqf-hanzi/nim_parser/64-1.txt",
        "../mqf-hanzi/nim_parser/64-1.txt",
    ] {
        if std::path::Path::new(p).exists() {
            return p.to_string();
        }
    }
    "mqf-hanzi/nim_parser/64-1.txt".to_string()
}

fn read_canvas() -> Vec<String> {
    match fs::read_to_string(canvas_path()) {
        Ok(s) => s.lines().map(|l| l.to_string()).collect(),
        Err(_) => Vec::new(),
    }
}

fn radix64_alphabet() -> String {
    let mut out = String::new();
    for l in read_canvas() {
        if let Some(rest) = l.strip_prefix('#') {
            let rest = rest.trim();
            let mut it = rest.chars();
            if let (Some(c), None) = (it.next(), it.next()) {
                if !out.contains(c) {
                    out.push(c);
                }
            }
        }
    }
    out
}

fn radix64_of(alphabet: &[char], mut n: u64) -> String {
    if alphabet.is_empty() {
        return n.to_string();
    }
    let base = alphabet.len() as u64;
    let mut d = Vec::new();
    loop {
        d.push(alphabet[(n % base) as usize]);
        n /= base;
        if n == 0 {
            break;
        }
    }
    d.iter().rev().collect()
}

fn do_canvas_info() {
    let lines = read_canvas();
    if lines.is_empty() {
        println!("未找到 {}", canvas_path());
        return;
    }
    let alpha = radix64_alphabet();
    let mut addrs: Vec<String> = Vec::new();
    let mut last: Option<String> = None;
    let mut blocks = 0usize;
    let mut other = 0usize;
    for l in &lines {
        if let Some(rest) = l.strip_prefix('#') {
            let a = rest.trim().to_string();
            if last.as_deref() != Some(a.as_str()) {
                addrs.push(a.clone());
                last = Some(a);
            }
            blocks += 1;
        } else if !l.is_empty() && !l.starts_with('⿴') && !l.starts_with('▇') && l.chars().next() != Some('@') {
            other += 1;
        }
    }
    println!("64 进制格画布 · {}", canvas_path());
    println!("  行数 = {} · #块 = {} · 独立地址 = {} · 异常行 = {}", lines.len(), blocks, addrs.len(), other);
    println!("  数字表 ({} 位) = {}", alpha.chars().count(), alpha);
    println!("  首位 = {} · 末位 = {} · 进位后首址 = {}", addrs.first().map(|s| s.as_str()).unwrap_or("-"), addrs.last().map(|s| s.as_str()).unwrap_or("-"), if addrs.len() > 64 { addrs[64].as_str() } else { "-" });
    println!("  块语法: ⿴…⿴ (顶层边界) / #地址 / ⿴…⿴ (容器) / @地址 / ▇…▓ (填充)");
}

fn do_canvas_cell(addr: &str) {
    if addr.is_empty() {
        println!("用法: canvas-cell <地址> · 打印该地址的 5 行块");
        return;
    }
    let lines = read_canvas();
    let target = format!("#{}", addr);
    for (i, l) in lines.iter().enumerate() {
        if l.trim() == target {
            let start = i.saturating_sub(1);
            for l in lines.iter().skip(start).take(5) {
                println!("{}", l);
            }
            return;
        }
    }
    println!("画布无地址「{}」", addr);
}

fn do_yunmu_canvas(out: &str) {
    let out = if out.is_empty() { "yunmu_canvas.svg" } else { out };
    let list = load_yunmu(&rules_file("yunmu.chain"));
    let alpha: Vec<char> = radix64_alphabet().chars().collect();
    if alpha.is_empty() {
        println!("未读到 64 进制数字表 (缺 {})", canvas_path());
        return;
    }
    let palette = ["#7df", "#fd9", "#9f9", "#f9c", "#c9f", "#9cf"];
    let cols = 8usize;
    let base = alpha.len();
    let cell = 72.0_f64;
    let pad = 24.0_f64;
    let header = 34.0_f64;
    let rows = (list.len() + cols - 1).max(1) / cols;
    let w = pad * 2.0 + cell * cols as f64;
    let h = header + pad * 2.0 + cell * rows as f64;
    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{:.0}\" height=\"{:.0}\" viewBox=\"0 0 {:.0} {:.0}\">\n",
        w, h, w, h
    ));
    svg.push_str("<rect width=\"100%\" height=\"100%\" fill=\"#0b0b0f\"/>\n");
    svg.push_str(&format!(
        "<text x=\"12\" y=\"20\" fill=\"#8fe\" font-size=\"12\">64 进制格画布 · 数字表 {} 位 = {} · 源 64-1.txt</text>\n",
        base, alpha.iter().collect::<String>()
    ));
    let total_slots = (rows * cols).max(list.len());
    for slot in 0..total_slots {
        let ox = pad + cell * (slot % cols) as f64;
        let oy = header + pad + cell * (slot / cols) as f64;
        svg.push_str(&format!(
            "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"#0e141b\" stroke=\"#243\" stroke-width=\"1\"/>\n",
            ox, oy, cell, cell
        ));
        if slot < list.len() {
            let y = &list[slot];
            let addr = radix64_of(&alpha, slot as u64);
            let chars: Vec<char> = y.pinyin.chars().collect();
            let mut shapes: Vec<String> = Vec::new();
            for (ci, ch) in chars.iter().enumerate() {
                if let Some(prims) = glyph_geo(*ch) {
                    for (t, z) in prims.iter() {
                        let (lx, ly) = zone_pt(z);
                        shapes.push(geo_shape(t, ox + 6.0 + lx * 0.8, oy + 4.0 + ly * 0.8, palette[ci % palette.len()]));
                    }
                }
            }
            for s in &shapes {
                svg.push_str(s);
                svg.push('\n');
            }
            svg.push_str(&format!(
                "<text x=\"{:.1}\" y=\"{:.1}\" fill=\"#cfe\" font-size=\"10\" text-anchor=\"middle\">{}</text>\n",
                ox + cell / 2.0, oy + cell - 16.0, y.pinyin
            ));
            svg.push_str(&format!(
                "<text x=\"{:.1}\" y=\"{:.1}\" fill=\"#6a9\" font-size=\"10\" text-anchor=\"middle\">#{}</text>\n",
                ox + cell / 2.0, oy + cell - 4.0, addr
            ));
            let f = mirror_flags(&y.pinyin);
            let mark = if f == "—" || f == "空" { "-" } else { f.as_str() };
            svg.push_str(&format!(
                "<text x=\"{:.1}\" y=\"{:.1}\" fill=\"#fd9\" font-size=\"9\" text-anchor=\"end\">{}</text>\n",
                ox + cell - 3.0, oy + 11.0, mark
            ));
        } else {
            svg.push_str(&format!(
                "<text x=\"{:.1}\" y=\"{:.1}\" fill=\"#264\" font-size=\"10\" text-anchor=\"middle\">#{}</text>\n",
                ox + cell / 2.0, oy + cell - 4.0, radix64_of(&alpha, slot as u64)
            ));
        }
    }
    svg.push_str("</svg>\n");
    match fs::write(out, svg) {
        Ok(_) => println!("64 进制画布: 落 {} 母 → {} ({}×{} 格 · 数字表 {} 位)", list.len(), out, rows, cols, base),
        Err(e) => println!("写入失败: {}", e),
    }
}

// ------------------------------------------------------------------
// 镜像裁判: 取合并字形的 (GeoType, Zone落点) 集合, 用 mirror_engine 的
// 水平/垂直/对角反射自比: 集合在反射下闭合 = 自镜像, 否则不对称
// 反射源: encoding/mirror_engine.h (HORIZONTAL/VERTICAL/DIAGONAL)
// ------------------------------------------------------------------

fn zone_prims(pinyin: &str) -> Vec<(&'static str, f64, f64)> {
    let mut v = Vec::new();
    for ch in pinyin.chars() {
        if let Some(prims) = glyph_geo(ch) {
            for (t, z) in prims.iter() {
                let (lx, ly) = zone_pt(z);
                v.push((*t, lx, ly));
            }
        }
    }
    v
}

fn closed_under(prims: &[(&'static str, f64, f64)], refl: fn(f64, f64) -> (f64, f64), tol: f64) -> bool {
    prims.iter().all(|(t, x, y)| {
        let (rx, ry) = refl(*x, *y);
        prims.iter().any(|(t2, x2, y2)| {
            t == t2 && ((rx - x2).abs() < tol) && ((ry - y2).abs() < tol)
        })
    })
}

fn mirror_flags(pinyin: &str) -> String {
    let p = zone_prims(pinyin);
    if p.is_empty() {
        return "空".to_string();
    }
    let mut f = String::new();
    if closed_under(&p, |x, y| (64.0 - x, y), 1.0) {
        f.push('横');
    }
    if closed_under(&p, |x, y| (x, 64.0 - y), 1.0) {
        f.push('纵');
    }
    if closed_under(&p, |x, y| (64.0 - y, 64.0 - x), 1.0) {
        f.push('対');
    }
    if f.is_empty() {
        "—".to_string()
    } else {
        f
    }
}

fn do_yunmu_mirror() {
    let list = load_yunmu(&rules_file("yunmu.chain"));
    if list.is_empty() {
        println!("尚无注册韵母 · 先登记 rules/yunmu.chain");
        return;
    }
    println!("镜像裁判 · 取 (GeoType, Zone落点) 集合 · 反射源 mirror_engine.h (水平/垂直/对角)");
    let mut self_cnt = 0usize;
    for y in &list {
        let f = mirror_flags(&y.pinyin);
        if f != "—" && f != "空" {
            self_cnt += 1;
        }
        println!("  {:<4} 小写 {} 大写 {} · 自比 {}", y.pinyin, y.lower, y.upper, f);
    }
    println!("  自镜像成立 {} / {} · 其余不对称 (按选择取舍, 非缺陷)", self_cnt, list.len());
}

fn do_ctn(name: &str, phase: &str) {
    if name.is_empty() {        println!("用法: ctn <名称> [相位] · 声明一个容器(插件) → 分配加法地址");
        return;
    }
    let path = rules_file("containers.chain");
    let mut cs = load_containers(&path);
    if let Some(i) = cs.iter().position(|c| c.name == name) {
        println!("容器已存在 · 加法地址 {} · 相位 {}", i + 1, cs[i].phase);
        return;
    }
    cs.push(Container {
        name: name.to_string(),
        phase: phase.to_string(),
        syms: Vec::new(),
    });
    save_containers(&path, &cs);
    println!("插件已装载 · 容器「{}」", name);
    println!("  加法地址 = {} (串联而非相乘) · 相位 = {} (相位进制 {})", cs.len(), phase, phase_num(phase));
    println!("  公路已通到该砖块 · 容器内符号将热可用");
}

fn do_sym(key: &str, syms: &str) {
    if key.is_empty() || syms.is_empty() {
        println!("用法: sym <容器名或地址> <符号1,符号2,...> · 往容器修入符号(热可用)");
        return;
    }
    let path = rules_file("containers.chain");
    let mut cs = load_containers(&path);
    let idx = match find_container(&cs, key) {
        Some(i) => i,
        None => {
            println!("未声明容器「{}」· 先 ctn {} 声明 (不声明就不在容器范畴内)", key, key);
            return;
        }
    };
    let mut added = 0usize;
    for s in syms.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()) {
        if !cs[idx].syms.contains(&s) {
            cs[idx].syms.push(s);
            added += 1;
        }
    }
    let name = cs[idx].name.clone();
    let total = cs[idx].syms.len();
    save_containers(&path, &cs);
    println!("公路修入容器「{}」(地址 {}) · 新增符号 {} 个 · 热可用(非锁死)", name, idx + 1, added);
    println!("  容器符号总数 = {}", total);
}

fn do_addr() {
    let cs = load_containers(&rules_file("containers.chain"));
    if cs.is_empty() {
        println!("尚无容器 · 用 ctn <名称> 声明第一个砖块");
        return;
    }
    println!("加法索引 · 容器地址遍历 (每个容器 = 一个插件 = 一个进制节点)");
    for (i, c) in cs.iter().enumerate() {
        println!("  地址 {} | 相位 {} (进制 {}) | {} | 符号 {}", i + 1, c.phase, phase_num(&c.phase), c.name, c.syms.len());
    }
    let sym_total: u64 = cs.iter().map(|c| c.syms.len() as u64).sum();
    println!("  容器总数(加法累计地址) = {} · 符号串联总数 = {}", cs.len(), sym_total);
}

fn do_tongji() {
    let cs = load_containers(&rules_file("containers.chain"));
    if cs.is_empty() {
        println!("尚无容器可统计 · 先 ctn 声明容器并用 sym 修入符号");
        return;
    }
    println!("临时启用范畴论 · 清点符号 → 生成乘法口诀表");
    let mut cum_sym = 0u64;
    for (i, c) in cs.iter().enumerate() {
        cum_sym += c.syms.len() as u64;
        println!("  容器「{}」地址 {} · 自递归符号 {} · 口诀 {}×{}={}", c.name, i + 1, c.syms.len(), i + 1, c.syms.len(), (i as u64 + 1) * c.syms.len() as u64);
    }
    let reach = cs.len() as u64 * cum_sym;
    println!("  清点: 容器 {} 个 · 符号串联 {} 个 (加法)", cs.len(), cum_sym);
    println!("  乘法口诀表覆盖 = {} × {} = {} (当前最大功率范畴)", cs.len(), cum_sym, reach);
    println!("  公路修到哪里, 统计地址与口诀就索引到哪里");
}

fn do_phase(key: &str, pos: &str) {
    if key.is_empty() || pos.is_empty() {
        println!("用法: phase <容器名或地址> <方位> · 方位=前/后/左/右/上/下/中/腰/角");
        return;
    }
    let path = rules_file("containers.chain");
    let mut cs = load_containers(&path);
    match find_container(&cs, key) {
        None => println!("未声明容器「{}」", key),
        Some(i) => {
            cs[i].phase = pos.to_string();
            save_containers(&path, &cs);
            println!("容器「{}」相位 → {} · 相位进制 = {} · 地址坐标 = {}.{}", cs[i].name, pos, phase_num(pos), i + 1, phase_num(pos));
        }
    }
}

fn load_plain(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .map(|s| s.lines().map(|l| l.trim().to_string()).filter(|l| !l.is_empty() && !l.starts_with('#')).collect())
        .unwrap_or_default()
}

fn do_reg(file: &str, ns: &str, status: &str, ver: &str, tm: &str) {
    if file.is_empty() {
        println!("用法: reg <文件> [清单] [状态] [版本] [时间] · 登记到注册清单");
        return;
    }
    let path = rules_file("registry.chain");
    let line = format!("{}|{}|{}|{}|{}", ns, file, status, ver, tm);
    let mut content = fs::read_to_string(&path).unwrap_or_else(|_| "# 名称|文件|状态|版本|时间\n".to_string());
    if !content.ends_with('\n') {
        content.push('\n');
    }
    content.push_str(&line);
    content.push('\n');
    let _ = fs::write(&path, content);
    println!("注册清单 · 名称 {} | 文件 {} | 状态 {} | 版本 {} | 时间 {}", ns, file, status, ver, tm);
    println!("  用名称呼唤已注册的符号, 不用数字代表符号");
}

fn do_reglist() {
    let rows = load_plain(&rules_file("registry.chain"));
    if rows.is_empty() {
        println!("注册清单为空 · 用 reg <文件> [清单] [状态] 登记");
        return;
    }
    println!("文档注册清单 (名称呼唤 · 一个名称对应一份清单)");
    for r in rows {
        println!("  {}", r.replace('|', " | "));
    }
}

fn do_link(a: &str, b: &str, rel: &str) {
    if a.is_empty() || b.is_empty() {
        println!("用法: link <名称A> <名称B> [关系] · 记录文档依赖连线(总观效应)");
        return;
    }
    let rel = if rel.is_empty() { "关联" } else { rel };
    let path = rules_file("links.chain");
    let mut content = fs::read_to_string(&path).unwrap_or_else(|_| "# 名称A|名称B|关系\n".to_string());
    if !content.ends_with('\n') {
        content.push('\n');
    }
    content.push_str(&format!("{}|{}|{}\n", a, b, rel));
    let _ = fs::write(&path, content);
    println!("依赖连线 · {} ←→ {} ({}) · 只依赖它, 不重造替代品", a, b, rel);
}

fn do_links() {
    let rows = load_plain(&rules_file("links.chain"));
    if rows.is_empty() {
        println!("尚无依赖连线 · 用 link <A> <B> [关系] 记录");
        return;
    }
    println!("总观效应 · 依赖连线 (可感知全局相互依赖)");
    for r in rows {
        println!("  {} ←→ {} ({})", r.split('|').nth(0).unwrap_or(""), r.split('|').nth(1).unwrap_or(""), r.split('|').nth(2).unwrap_or(""));
    }
}

// ============ 征用层 · 动态编号 (松弛编号, 插入自己) ============
// 身份 = 当前绑定的持有者; 原绑定保留为可选功能 (可召可不召)
fn find_binding(path: &str, name: &str) -> Option<(String, String)> {
    for line in fs::read_to_string(path).unwrap_or_default().lines() {
        let l = line.trim();
        if l.is_empty() || l.starts_with('#') {
            continue;
        }
        let f: Vec<&str> = l.split('|').collect();
        if f.first().map(|s| s.trim()) == Some(name) {
            return Some((
                f.get(1).map(|s| s.trim().to_string()).unwrap_or_default(),
                f.get(2).map(|s| s.trim().to_string()).unwrap_or_default(),
            ));
        }
    }
    None
}

fn do_zheng(name: &str, current: &str, original: &str) {
    if name.is_empty() || current.is_empty() {
        println!("用法: zheng <名字> <当前绑定> [原绑定] · 松弛名字的编号, 把自己插进去");
        return;
    }
    let path = rules_file("bindings.chain");
    let mut lines: Vec<String> = fs::read_to_string(&path)
        .unwrap_or_else(|_| "# 名字|当前绑定|原绑定|状态\n".to_string())
        .lines()
        .map(|s| s.to_string())
        .collect();
    let row = format!("{}|{}|{}|生效", name, current, original);
    let mut replaced = false;
    for l in lines.iter_mut() {
        let t = l.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        if t.split('|').next().map(|s| s.trim()) == Some(name) {
            *l = row.clone();
            replaced = true;
        }
    }
    if !replaced {
        lines.push(row);
    }
    let mut content = lines.join("\n");
    content.push('\n');
    let _ = fs::write(&path, content);
    println!("征用 · 名字 {} 的编号已松弛 · 当前绑定 = {}", name, current);
    if !original.is_empty() {
        println!("  原绑定保留为可选功能 · 召原: yuan {}", name);
    }
}

fn do_zhengs() {
    let path = rules_file("bindings.chain");
    let mut n = 0;
    println!("征用表 (名字 | 当前绑定 | 原绑定 | 状态) · 身份 = 当前绑定的持有者");
    for line in fs::read_to_string(&path).unwrap_or_default().lines() {
        let l = line.trim();
        if l.is_empty() || l.starts_with('#') {
            continue;
        }
        println!("  {}", l.replace('|', " | "));
        n += 1;
    }
    if n == 0 {
        println!("  尚无征用 · 用 zheng <名字> <当前绑定> [原绑定] 松弛一个编号");
    }
}

fn do_sontuo(name: &str) {
    if name.is_empty() {
        println!("用法: sontuo <名字> · 松脱编号, 归还所征用的身份");
        return;
    }
    let path = rules_file("bindings.chain");
    let mut lines: Vec<String> = fs::read_to_string(&path)
        .unwrap_or_default()
        .lines()
        .map(|s| s.to_string())
        .collect();
    let before = lines.len();
    lines.retain(|l| {
        let t = l.trim();
        t.is_empty() || t.starts_with('#') || t.split('|').next().map(|s| s.trim()) != Some(name)
    });
    if lines.len() == before {
        println!("名字 {} 未被征用 · 无需松脱", name);
        return;
    }
    let mut out = lines.join("\n");
    if !out.ends_with('\n') {
        out.push('\n');
    }
    let _ = fs::write(&path, out);
    println!("松脱 · 名字 {} 的编号已归还 · 本体独立于编号之外", name);
}

fn run_binding_side(name: &str, which: &str, recipe: &str, params: &[&str]) {
    if recipe.is_empty() {
        println!("  {} · {} 为空", name, which);
        return;
    }
    println!("  征用 {} · 走{} · 配方 = {}", name, which, recipe);
    run_recipe(recipe, params, &membrane_path());
}

fn try_binding(name: &str, params: &[&str]) -> bool {
    if let Some((cur, orig)) = find_binding(&rules_file("bindings.chain"), name) {
        run_binding_side(name, "当前绑定", &cur, params);
        if !orig.is_empty() {
            println!("  原功能可选 · 召原: yuan {}", name);
        }
        return true;
    }
    false
}

// ============ 对齐算法 · 混合进制自动进位 ============
// 只写"对齐开始 / 对齐结尾" + 消耗/用尽/进位; 不写三个坐标
// 基数不凭空给: 呼唤已注册容器, 清点其中的符号数, 就得到该层基数
fn container_bases() -> Vec<u64> {
    load_containers(&rules_file("containers.chain"))
        .iter()
        .map(|c| c.syms.len() as u64)
        .filter(|n| *n > 0)
        .collect()
}

fn named_bases(tok: &str) -> Option<(String, Vec<u64>)> {
    for line in fs::read_to_string(rules_file("bases.chain")).unwrap_or_default().lines() {
        let l = line.trim();
        if l.is_empty() || l.starts_with('#') {
            continue;
        }
        let p: Vec<&str> = l.split('|').map(|s| s.trim()).collect();
        if p.first().copied() == Some(tok) {
            let nums: Vec<u64> = p.get(1).map(|s| s.split(',').filter_map(|x| x.trim().parse().ok()).collect()).unwrap_or_default();
            let desc = p.get(2).copied().unwrap_or("").to_string();
            if !nums.is_empty() {
                return Some((desc, nums));
            }
        }
    }
    None
}

fn parse_bases(s: &str) -> (String, Vec<u64>) {
    let t = s.trim();
    if t.is_empty() {
        return ("清点已注册容器".to_string(), container_bases());
    }
    if let Some((desc, nums)) = named_bases(t) {
        return (if desc.is_empty() { t.to_string() } else { desc }, nums);
    }
    let v: Vec<u64> = t.split(',').filter_map(|x| x.trim().parse().ok()).filter(|x| *x > 0).collect();
    if v.is_empty() {
        ("清点已注册容器".to_string(), container_bases())
    } else {
        ("自定义基数".to_string(), v)
    }
}

fn do_bases() {
    let rows: Vec<String> = fs::read_to_string(rules_file("bases.chain"))
        .unwrap_or_default()
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect();
    println!("可选基数集 · 选择哪个就用哪个 (没有唯一的\"准确\")");
    println!("  清点容器 | 动态 | 呼唤已注册容器清点符号数 (不带集合名时的默认)");
    for r in rows {
        println!("  {}", r.replace('|', " | "));
    }
}

fn do_align(n: u64, label: &str, bases: &[u64], from_end: bool) {
    let bs = if bases.is_empty() { container_bases() } else { bases.to_vec() };
    let head = if from_end { "对齐结尾" } else { "对齐开始" };
    if bs.is_empty() {
        println!("{} · 共享域地址 = {}", head, n);
        println!("  尚未呼唤到容器 · 先 ctn 声明容器、sym 修入符号, 或用 bases 选一个基数集");
        return;
    }
    let mut v = n;
    let mut consumed = Vec::new();
    let order: Vec<usize> = if from_end { (0..bs.len()).rev().collect() } else { (0..bs.len()).collect() };
    for &i in &order {
        consumed.push(v % bs[i]);
        v /= bs[i];
    }
    println!("{} · 共享域地址 = {}", head, n);
    println!("  基数集 = {} · 各级基数 = {:?}", label, bs);
    println!("  低位消耗 = {:?} · 用尽则进位到纵向层", consumed);
    if v > 0 {
        println!("  纵向层高度 += {} (等效对齐继续累加)", v);
    } else {
        println!("  在共享域内 · 一个数即代表几何位置, 无需写坐标");
    }
}

fn do_unify(values: &str, label: &str, bases: &[u64]) {
    let vals: Vec<u64> = values.split(',').filter_map(|x| x.trim().parse().ok()).collect();
    if vals.is_empty() {
        println!("用法: unify <低位,高位,...> [集合名|基数,...] · 多段等效对齐合成一个共享域地址");
        return;
    }
    let bs = if bases.is_empty() { container_bases() } else { bases.to_vec() };
    let mut addr = 0u64;
    let mut mul = 1u64;
    for (i, v) in vals.iter().enumerate() {
        let b = bs.get(i).copied().unwrap_or(1);
        if *v >= b {
            println!("  第 {} 层低位用尽 (基数 {}) → 触发进位", i + 1, b);
        }
        addr += (*v % b) * mul;
        mul *= b;
    }
    println!("共享域地址 = {} (由 {} 层等效对齐合成 · 基数集 {})", addr, vals.len(), label);
}

// ============ 热加载 · 配方解释器 ============
// 规则表第 5 列 = 配方, 分号分隔若干操作: print / bind / ring / center / yong / mo
// 新增指令只在 cmd.chain 加一行, 无需重新编译 —— 改规则即生效
fn substitute(text: &str, params: &[&str]) -> String {
    let mut out = text.to_string();
    for (i, p) in params.iter().enumerate() {
        out = out.replace(&format!("${}", i + 1), p);
    }
    out
}

fn run_recipe(recipe: &str, params: &[&str], path: &str) {
    for op in recipe.split(';') {
        let op = op.trim();
        if op.is_empty() {
            continue;
        }
        let mut it = op.splitn(2, ' ');
        let verb = it.next().unwrap_or("");
        let rest = substitute(it.next().unwrap_or("").trim(), params);
        match verb {
            "print" => println!("{}", rest),
            "bind" => {
                let target = if rest.is_empty() {
                    params.first().copied().unwrap_or("")
                } else {
                    rest.as_str()
                };
                let n = bind_membrane(path, target);
                println!("  膜圈缚: {} → 归入中心范畴 ({}) · 膜层数 = {} · 不可脱离", target, CENTER, n);
            }
            "ring" => {
                let list = load_membrane(path);
                println!("  圈层视图 (中心: {})", CENTER);
                for (i, t) in list.iter().enumerate() {
                    println!("    第 {} 层 | {}", list.len() - i, t);
                }
            }
            "center" => println!("  中心锚点 = {}", CENTER),
            "yong" => println!("  蛹: {} 进入包裹态 (包裹 → 转化 → 等待破茧)", rest),
            "mo" => println!("  膜: 第 {} 层圈层结界 · 从中心 ({}) 向外辐射", rest, CENTER),
            "ctn" => {
                let v: Vec<&str> = rest.split_whitespace().collect();
                do_ctn(v.first().copied().unwrap_or(""), v.get(1).copied().unwrap_or("中"));
            }
            "sym" => {
                let v: Vec<&str> = rest.split_whitespace().collect();
                do_sym(v.first().copied().unwrap_or(""), v.get(1).copied().unwrap_or(""));
            }
            "addr" => do_addr(),
            "yunmu" => do_yunmu(rest.split_whitespace().next().unwrap_or("")),
            "yunmu-list" => do_yunmu_list(),
            "tongji" => do_tongji(),
            "phase" => {
                let v: Vec<&str> = rest.split_whitespace().collect();
                do_phase(v.first().copied().unwrap_or(""), v.get(1).copied().unwrap_or(""));
            }
            "reg" => {
                let v: Vec<&str> = rest.split_whitespace().collect();
                do_reg(v.first().copied().unwrap_or(""), v.get(1).copied().unwrap_or(""), v.get(2).copied().unwrap_or("工作中"), v.get(3).copied().unwrap_or("v1"), v.get(4).copied().unwrap_or(""));
            }
            "link" => {
                let v: Vec<&str> = rest.split_whitespace().collect();
                do_link(v.first().copied().unwrap_or(""), v.get(1).copied().unwrap_or(""), v.get(2).copied().unwrap_or("关联"));
            }
            "align" => {
                let v: Vec<&str> = rest.split_whitespace().collect();
                let n: u64 = v.first().and_then(|s| s.parse().ok()).unwrap_or(0);
                let (label, bases) = parse_bases(v.get(1).copied().unwrap_or(""));
                do_align(n, &label, &bases, false);
            }
            "unify" => {
                let v: Vec<&str> = rest.split_whitespace().collect();
                let (label, bases) = parse_bases(v.get(1).copied().unwrap_or(""));
                do_unify(v.first().copied().unwrap_or(""), &label, &bases);
            }
            _ => println!("  [未知配方指令] {}", op),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let cmds = load(&rules_path()).unwrap_or_else(|e| {
        println!("错误: {}", e);
        process::exit(1);
    });
    let cmd = args.first().map(|s| s.as_str()).unwrap_or("list");

    match cmd {
        "list" | "-l" => {
            println!("沙箱模拟指令环境 · 六类指令 (规则即数据)\n");
            for cat in ["协议", "感知", "角色", "执行", "时间", "关系", "进制", "膜圈", "容器", "对齐", "征用", "韵母"] {
                let list: Vec<&Cmd> = cmds.iter().filter(|c| c.cat == cat).collect();
                if list.is_empty() {
                    continue;
                }
                println!("【{}层】", cat);
                for c in list {
                    println!("  {} {}  -- {}", c.name, "·".repeat(c.argc.saturating_add(1)), c.desc);
                }
            }
        }
        "help" => {
            let name = args.get(1).map(|s| s.as_str()).unwrap_or("");
            if let Some(c) = cmds.iter().find(|c| c.name == name) {
                println!("[{}] {} | {} | 参数数{}", c.name, c.cat, c.desc, c.argc);
            } else {
                println!("未知指令: {}", name);
            }
        }
        "qe" => {
            let v: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
            println!("{} → {}", v, qe(v));
        }
        "qd" => {
            let s = args.get(1).map(|s| s.as_str()).unwrap_or("");
            match qd(s) {
                Some(v) => println!("{} → {}", s, v),
                None => println!("解码失败: 需要3个颜色字"),
            }
        }
        "gcd" => {
            let a: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
            let b: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
            println!("公约数(中心男人/静态) gcd({},{}) = {}", a, b, gcd(a, b));
        }
        "lcm" => {
            let a: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
            let b: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
            println!("公倍数(边界女人/周期) lcm({},{}) = {}", a, b, lcm(a, b));
        }
        "mirror" => {
            let base: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(42);
            let v: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
            if v < base {
                let m = base - 1 - v;
                println!("镜像对偶 {}进制: {} ↔ {} (对称轴 {})", base, v, m, (base - 1) as f64 / 2.0);
            } else {
                println!("值越界");
            }
        }
        "add" | "mul" => {
            let a: i64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
            let b: i64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
            if cmd == "add" {
                println!("加法(窄·管乘法) {} + {} = {}", a, b, a + b);
            } else {
                println!("乘法(宽·被管) {} × {} = {}", a, b, a * b);
            }
        }
        "state" => {
            let s = args.get(1).map(|s| s.as_str()).unwrap_or("待机");
            let states = ["待机", "否极泰来", "合一爱人"];
            if states.contains(&s) {
                println!("系统状态 → {}", s);
            } else {
                println!("未知状态: {} (可用: 待机/否极泰来/合一爱人)", s);
            }
        }
        "tclgs" => {
            let op = args.get(1).map(|s| s.as_str()).unwrap_or("life");
            let v = args.get(2).map(|s| s.as_str()).unwrap_or("编码");
            match op {
                "gen" => println!("时间编码生命周期 · 生成 {} (起始相位)", v),
                "expire" => println!("时间编码生命周期 · {} 过期 → 转化", v),
                "fold" => println!("时间编码生命周期 · {} 折叠 → 回归中心一", v),
                _ => println!("tclgs 时间功法: gen生成/expire过期/fold折叠 (归宿=回归一)"),
            }
        }
        "juan" => {
            let p = args.get(1).map(|s| s.as_str()).unwrap_or("时间");
            println!("{} 相位颠倒 → 空间 (毛利兰 · 空间管理镜像)", p);
        }
        "time" => {
            println!("当前时间周期: 以 tclgs 为相位, 所有编码回归中心一");
        }
        "run" => {
            let chain = args.get(1).map(|s| s.as_str()).unwrap_or("");
            let input = args.get(2).map(|s| s.as_str()).unwrap_or("");
            println!("执行管线: run {} {}", chain, input);
            println!("  协议层 gttx:// 寻址 → 感知层 gkndl 转化 → 角色层 归属判定 → 关系层 公约数归集");
            println!("  结果: 所有公倍数尘埃落定 → 回归中心一");
        }
        "ping" => {
            println!("沙箱指令环境在线");
        }
        "mqf" => {
            let t = args.get(1).map(|s| s.as_str()).unwrap_or("");
            if t.is_empty() {
                println!("用法: mqf <目标>  将目标绑入中心范畴");
            } else {
                let n = bind_membrane(&membrane_path(), t);
                println!("膜圈缚: {} → 归入中心范畴 ({})", t, CENTER);
                println!("  包裹 → 转化 → 结界闭合 · 膜层数 = {} · 不可脱离", n);
            }
        }
        "yong" => {
            let t = args.get(1).map(|s| s.as_str()).unwrap_or("本体");
            println!("蛹: {} 进入包裹态 (包裹 → 转化 → 等待破茧)", t);
        }
        "mo" => {
            let n: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
            if n == 0 {
                println!("用法: mo <层号>  生成第 n 层圈层结界");
            } else {
                println!("膜: 第 {} 层圈层结界 · 半径 {} · 从中心 ({}) 向外辐射", n, n, CENTER);
            }
        }
        "ring" => {
            let list = load_membrane(&membrane_path());
            println!("膜圈缚 · 圈层视图 (中心: {})", CENTER);
            if list.is_empty() {
                println!("  尚无膜圈 · 用 mqf <目标> 绑入");
            } else {
                for (i, t) in list.iter().enumerate() {
                    println!("  第 {} 层 | {}", list.len() - i, t);
                }
            }
            println!("  膜层总数 = {}", list.len());
        }
        "tuo" => {
            let t = args.get(1).map(|s| s.as_str()).unwrap_or("");
            println!("脱缚检测: {} · 处于中心范畴内 · 判定 = 不可脱离 (范内恒真)", t);
        }
        "center" => {
            println!("中心锚点 = {}", CENTER);
            println!("  被融合者归入范畴 · 所有膜层向中心收敛");
        }
        "serve" => {
            let out = args.get(1).map(|s| s.as_str()).unwrap_or("membrane.html");
            let layer: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
            let html = membrane_html(&load_membrane(&membrane_path()), layer);
            match fs::write(out, html) {
                Ok(_) => println!("全栈输出: 已生成 HTML 控制台 → {}", out),
                Err(e) => println!("写入失败: {}", e),
            }
        }
        "align" => {
            let n: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
            let (label, bases) = parse_bases(args.get(2).map(|s| s.as_str()).unwrap_or(""));
            do_align(n, &label, &bases, false);
        }
        "align-end" => {
            let n: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
            let (label, bases) = parse_bases(args.get(2).map(|s| s.as_str()).unwrap_or(""));
            do_align(n, &label, &bases, true);
        }
        "unify" => {
            let (label, bases) = parse_bases(args.get(2).map(|s| s.as_str()).unwrap_or(""));
            do_unify(args.get(1).map(|s| s.as_str()).unwrap_or(""), &label, &bases);
        }
        "bases" => do_bases(),
        "ctn" => do_ctn(args.get(1).map(|s| s.as_str()).unwrap_or(""), args.get(2).map(|s| s.as_str()).unwrap_or("中")),
        "sym" => do_sym(args.get(1).map(|s| s.as_str()).unwrap_or(""), args.get(2).map(|s| s.as_str()).unwrap_or("")),
        "addr" => do_addr(),
        "yunmu" => do_yunmu(args.get(1).map(|s| s.as_str()).unwrap_or("")),
        "yunmu-list" => do_yunmu_list(),
        "yunmu-svg" => do_yunmu_svg(
            args.get(1).map(|s| s.as_str()).unwrap_or(""),
            args.get(2).map(|s| s.as_str()).unwrap_or(""),
        ),
        "yunmu-merge" => do_yunmu_merge(
            args.get(1).map(|s| s.as_str()).unwrap_or(""),
            args.get(2).map(|s| s.as_str()).unwrap_or(""),
        ),
        "yunmu-canvas" => do_yunmu_canvas(args.get(1).map(|s| s.as_str()).unwrap_or("")),
        "canvas-info" => do_canvas_info(),
        "canvas-cell" => do_canvas_cell(args.get(1).map(|s| s.as_str()).unwrap_or("")),
        "yunmu-mirror" => do_yunmu_mirror(),
        "tongji" => do_tongji(),
        "phase" => do_phase(args.get(1).map(|s| s.as_str()).unwrap_or(""), args.get(2).map(|s| s.as_str()).unwrap_or("")),
        "reg" => do_reg(
            args.get(1).map(|s| s.as_str()).unwrap_or(""),
            args.get(2).map(|s| s.as_str()).unwrap_or(""),
            args.get(3).map(|s| s.as_str()).unwrap_or("工作中"),
            args.get(4).map(|s| s.as_str()).unwrap_or("v1"),
            args.get(5).map(|s| s.as_str()).unwrap_or(""),
        ),
        "reglist" => do_reglist(),
        "link" => do_link(args.get(1).map(|s| s.as_str()).unwrap_or(""), args.get(2).map(|s| s.as_str()).unwrap_or(""), args.get(3).map(|s| s.as_str()).unwrap_or("关联")),
        "links" => do_links(),
        "eval" => {
            let name = args.get(1).map(|s| s.as_str()).unwrap_or("");
            let params: Vec<&str> = args.iter().skip(2).map(|s| s.as_str()).collect();
            if try_binding(name, &params) {
                return;
            }
            match cmds.iter().find(|c| c.name == name && !c.recipe.is_empty()) {
                Some(c) => {
                    println!("[eval] {} · 配方 = {}", c.name, c.recipe);
                    run_recipe(&c.recipe, &params, &membrane_path());
                }
                None => {
                    if let Some(c) = cmds.iter().find(|c| c.name == name) {
                        println!("[{}] 行为固化在二进制内, 无热加载配方: {}", c.name, c.desc);
                    } else {
                        println!("未知指令: {} (在 rules/cmd.chain 加一行即可, 无需重编译)", name);
                    }
                }
            }
        }
        "zheng" => do_zheng(
            args.get(1).map(|s| s.as_str()).unwrap_or(""),
            args.get(2).map(|s| s.as_str()).unwrap_or(""),
            args.get(3).map(|s| s.as_str()).unwrap_or(""),
        ),
        "zhengs" => do_zhengs(),
        "sontuo" => do_sontuo(args.get(1).map(|s| s.as_str()).unwrap_or("")),
        "yuan" => {
            let name = args.get(1).map(|s| s.as_str()).unwrap_or("");
            if let Some((_cur, orig)) = find_binding(&rules_file("bindings.chain"), name) {
                run_binding_side(name, "原绑定", &orig, &[]);
            } else {
                println!("名字 {} 未被征用 · 无原绑定可召", name);
            }
        }
        "reload" => {
            let list = load(&rules_path());
            match list {
                Ok(l) => {
                    let hot = l.iter().filter(|c| !c.recipe.is_empty()).count();
                    println!("热加载: 规则表已重读 · 指令 {} 条 · 含配方(可热执行) {} 条", l.len(), hot);
                    for c in l.iter().filter(|c| !c.recipe.is_empty()) {
                        println!("  {} → {}", c.name, c.recipe);
                    }
                }
                Err(e) => println!("重读失败: {}", e),
            }
        }
        _ => {
            let params: Vec<&str> = args.iter().skip(1).map(|s| s.as_str()).collect();
            if try_binding(cmd, &params) {
                return;
            }
            if let Some(c) = cmds.iter().find(|c| c.name == cmd && !c.recipe.is_empty()) {
                run_recipe(&c.recipe, &params, &membrane_path());
            } else if let Some(c) = cmds.iter().find(|c| c.name == cmd) {
                println!("[{}] {}: {}", c.name, c.cat, c.desc);
                println!("  (参数数 {})", c.argc);
            } else {
                println!("未知指令: {} (可用 list 查看全部, 缺指令就在 rules/cmd.chain 加一行)", cmd);
            }
        }
    }
}
