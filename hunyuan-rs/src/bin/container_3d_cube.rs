// ═══════════════════════════════════════════════════════════════════════════════
// 容器 3D 立方体渲染器 v6 — 真正的三维容器操弄 + 鼠标拖动
// ═══════════════════════════════════════════════════════════════════════════════
//
// 容器语言写场景 (Rust 编译):
//   · 读取 container_radix.rs 64 容器 × 3 极性
//   · 读取 container_font_unified.rs Geometry3D / Topology
//   · 生成 JSON 场景描述 (容器码 + 坐标 + 拓扑)
//
// 浏览器执行 (JS):
//   · 实时 3D 矩阵变换 (球坐标 → 旋转矩阵 → 投影)
//   · 鼠标拖动旋转, 滚轮缩放
//   · 深度排序 (远→近), 迎风面隐藏
//   · 拓扑线框 (盒/球/环/管/锥/线/点/格)
//
// 编译: rustc --edition 2018 -O src/bin/container_3d_cube.rs -o c3d

use std::fmt::Write;

const HUNYUAN_BASE: &str = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/hunyuan-rs/src";

// ═══════════════════════════════════════════════════════════════════════════
// 读取 64 容器 + 极性 + 拓扑
// ═══════════════════════════════════════════════════════════════════════════

fn load_containers() -> Vec<(u8, String, &'static str, &'static str)> {
    let path = format!("{}/encoding/container_radix.rs", HUNYUAN_BASE);
    let src = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(_) => return build_fallback(),
    };

    let mut result = Vec::new();
    let mut in_impl = false;

    for line in src.lines() {
        let t = line.trim();
        if t.starts_with("impl ") { in_impl = true; continue; }
        if in_impl { if t.starts_with("}") { in_impl = false; } continue; }
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
                        let name = extract_chinese(t);
                        let polarity = infer_polarity(t);
                        let topo = topo_for_code(code);
                        result.push((code, name.to_string(), polarity, topo));
                    }
                }
            }
        }
    }

    if result.len() >= 64 { result } else { build_fallback() }
}

fn extract_chinese(line: &str) -> String {
    if let Some(pos) = line.find("//") {
        let comment = &line[pos+2..];
        let mut start = None;
        let mut end = None;
        for (i, ch) in comment.char_indices() {
            if ch >= '\u{4e00}' && ch <= '\u{9fff}' {
                if start.is_none() { start = Some(i); }
                end = Some(i + ch.len_utf8());
            } else if start.is_some() { break; }
        }
        if let (Some(s), Some(e)) = (start, end) {
            return comment[s..e].to_string();
        }
    }
    "?".to_string()
}

fn infer_polarity(line: &str) -> &'static str {
    if line.contains("穿透") { "阳" }
    else if line.contains("包容") || line.contains("阴极") { "阴" }
    else if line.contains("元") || line.contains("平衡") || line.contains("阳极") { "元" }
    else { "元" }
}

fn topo_for_code(code: u8) -> &'static str {
    match code {
        0x01..=0x02 => "box",
        0x03..=0x08 => "sphere",
        0x09..=0x0A => "torus",
        0x0B..=0x0C => "tube",
        0x0D       => "bicone",
        0x0E       => "line",
        0x00       => "point",
        _          => if code >= 0x40 { "grid" } else { "sphere" },
    }
}

fn build_fallback() -> Vec<(u8, String, &'static str, &'static str)> {
    let entries: [(u8, &str); 64] = [
        (0x00,"袋"),(0x01,"照"),(0x02,"见"),(0x03,"阴"),
        (0x04,"加"),(0x05,"分"),(0x06,"元"),(0x07,"裁"),
        (0x08,"产"),(0x09,"公"),(0x0A,"声"),(0x0B,"折"),
        (0x0C,"负"),(0x0D,"福"),(0x0E,"凤"),(0x0F,"栖"),
        (0x10,"融"),(0x11,"鼓"),(0x12,"蚕"),(0x13,"民"),
        (0x14,"盟"),(0x15,"满"),(0x16,"囡"),(0x17,"夕"),
        (0x18,"三"),(0x19,"六"),(0x1A,"八"),(0x1B,"九"),
        (0x1C,"零"),(0x1D,"一"),(0x1E,"二"),(0x1F,"四"),
        (0x20,"帮"),(0x21,"端"),(0x22,"精"),(0x23,"见"),
        (0x24,"来"),(0x25,"齿"),(0x26,"喉"),(0x27,"呼"),
        (0x28,"开"),(0x29,"清"),(0x2A,"平"),(0x2B,"上"),
        (0x2C,"前"),(0x2D,"内"),(0x2E,"单"),(0x2F,"长"),
        (0x30,"麻"),(0x31,"支"),(0x32,"寒"),(0x33,"歌"),
        (0x34,"阳"),(0x35,"尤"),(0x36,"止"),(0x37,"转"),
        (0x38,"高"),(0x39,"低"),(0x3A,"圆"),(0x3B,"扁"),
        (0x3C,"响"),(0x3D,"静"),(0x3E,"急"),(0x3F,"缓"),
    ];
    entries.iter().map(|(c, n)| (*c, n.to_string(), "元", "sphere")).collect()
}

// ═══════════════════════════════════════════════════════════════════════════
// 球坐标 → 3D 笛卡尔坐标
// ═══════════════════════════════════════════════════════════════════════════

fn spherical_to_xyz(code: u8) -> (f64, f64, f64) {
    // 容器码 → 球面坐标 (半径, θ经度, φ纬度)
    let ring = (code >> 4) as u8;  // 0-3
    let slot = (code & 0x0F) as u8;

    let r = 80.0 + ring as f64 * 55.0;
    let theta = (slot as f64) * (2.0 * std::f64::consts::PI / 16.0);
    let phi = -std::f64::consts::PI * 0.4 + ring as f64 * 0.25;

    let x = r * phi.cos() * theta.cos();
    let y = r * phi.cos() * theta.sin();
    let z = r * phi.sin();

    (x, y, z)
}

fn color_hsl(code: u8, polarity: &str) -> String {
    let ring = (code >> 4) as u8;
    let hue = match ring { 0 => 220, 1 => 280, 2 => 160, 3 => 40, _ => 0 };
    match polarity {
        "阳" => format!("hsl({},80%,60%)", hue+15),
        "元" => format!("hsl(50,90%,60%)"),
        _    => format!("hsl({},55%,50%)", hue),
    }
}

fn s(out: &mut String, text: &str) { out.push_str(text); }

// ═══════════════════════════════════════════════════════════════════════════
// HTML + JS — 场景由 Rust 生成, 操弄由浏览器执行
// ═══════════════════════════════════════════════════════════════════════════

fn build_html(containers: &[(u8, String, &str, &str)]) -> String {
    let mut out = String::new();

    // 1. 生成场景数据 JSON (容器语言写场景)
    let mut scene_data = String::new();
    for &(code, ref name, polarity, topo) in containers {
        let (x, y, z) = spherical_to_xyz(code);
        let color = color_hsl(code, polarity);
        let _ = write!(scene_data,
            "{{c:{code},x:{x:.1},y:{y:.1},z:{z:.1},n:\"{name}\",t:\"{topo}\",p:\"{polarity}\",col:\"{color}\"}},\n",
            code = code, x = x, y = y, z = z,
        );
    }

    s(&mut out, "<!DOCTYPE html><html lang=\"zh\">\n<head><meta charset=\"UTF-8\">\n");
    s(&mut out, "<title>容器 3D 立方体 v6 — 三维操弄</title>\n");
    s(&mut out, "<style>");
    s(&mut out, "body{background:#050508;color:#eee;font-family:'Segoe UI',sans-serif;margin:0;padding:16px;overflow:hidden;}\n");
    s(&mut out, "h1{font-size:18px;margin:4px 0 2px;color:#9b8bff;letter-spacing:2px;text-align:center;}\n");
    s(&mut out, "p.sub{color:#666;font-size:10px;margin:0 0 8px;text-align:center;}\n");
    s(&mut out, "#scene{width:90vw;height:85vh;margin:0 auto;position:relative;cursor:grab;touch-action:none;}\n");
    s(&mut out, "#canvas{width:100%;height:100%;position:absolute;top:0;left:0;}\n");
    s(&mut out, ".node{position:absolute;border-radius:50%;cursor:pointer;transform-origin:center;transition:none;}\n");
    s(&mut out, ".node:hover{filter:brightness(1.5);z-index:10000;}\n");
    s(&mut out, ".label{font-size:9px;color:#fff;text-shadow:0 0 3px #000;pointer-events:none;text-align:center;transform:translate(-50%,-50%);}\n");
    s(&mut out, ".info{position:fixed;bottom:5px;left:50%;transform:translateX(-50%);font-size:9px;color:#555;}\n");

    // 嵌入字体
    if let Some(b64) = std::fs::read_to_string(format!("{}/bin/font_data.txt", HUNYUAN_BASE)).ok() {
        let _ = write!(out, "@font-face{{font-family:'HanYuan64';src:url(data:font/ttf;base64,{})format('truetype');}}\n", b64.trim());
    }
    s(&mut out, ".cf{font-family:'HanYuan64',sans-serif;}\n");
    s(&mut out, "</style></head><body>\n");
    s(&mut out, "<h1>容器 3D 立方体</h1>\n");
    s(&mut out, "<p class=\"sub\">容器=字体=时间 · 拖动旋转 · 滚轮缩放 · 三维球面拓扑</p>\n");
    s(&mut out, "<div id=\"scene\"><svg id=\"canvas\" width=\"100%\" height=\"100%\"></svg></div>\n");
    s(&mut out, "<div class=\"info\">container_scene_data (80 containers) → 3D projection</div>\n");

    // 嵌入场景数据
    s(&mut out, "<script>\n");
    s(&mut out, "const SCENE=[\n");
    s(&mut out, &scene_data);
    s(&mut out, "];\n");

    // 嵌入 3D 引擎
    s(&mut out, r#"
let azimuth = 0.6, elevation = 0.35, dist = 380;
let dragging = false, lastX = 0, lastY = 0;
const svg = document.getElementById('canvas');

function rot(p, az, el) {
  // 绕 Y (azimuth) 和 X (elevation) 旋转
  let cosA = Math.cos(az), sinA = Math.sin(az);
  let cosE = Math.cos(el), sinE = Math.sin(el);
  let x1 = p.x * cosA + p.z * sinA;
  let z1 = -p.x * sinA + p.z * cosA;
  let y1 = p.y * cosE - z1 * sinE;
  let z2 = p.y * sinE + z1 * cosE;
  return { x: x1, y: y1, z: z2 };
}

function project(p) {
  // 弱透视: z 越大 (越近) 显得越大
  let scale = dist / (dist + p.z);
  return { x: p.x * scale, y: p.y * scale, s: scale };
}

function draw() {
  let W = svg.clientWidth || 800;
  let H = svg.clientHeight || 600;
  let cx = W / 2, cy = H / 2;

  // 旋转所有点并按 z 排序
  let pts = SCENE.map(function(s) {
    let p = rot({ x: s.x, y: s.y, z: s.z }, azimuth, elevation);
    let proj = project(p);
    return {
      code: s.c, x: cx + proj.x, y: cy + proj.y,
      z: p.z, s: proj.s,
      n: s.n, t: s.t, p: s.p, col: s.col
    };
  }).sort(function(a, b) { return a.z - b.z }); // 远(z小)的先画

  let html = '';
  for (let i = 0; i < pts.length; i++) {
    let pt = pts[i];
    let r = Math.max(6, 14 * pt.s);
    let opacity = 0.4 + pt.s * 0.6;
    // 时间脉冲效果: 基于 code
    let pulse = Math.sin((Date.now() / 200 + pt.code * 0.5)) * 0.3 + 0.7;

    // 拓扑线框
    let topo = svg_line(pt, r);

    // 圆形标记
    html += '<g class="cf">';

    // 拓扑线
    if (topo) html += topo;

    // 节点圆
    html += '<circle cx="' + pt.x.toFixed(1) + '" cy="' + pt.y.toFixed(1) +
            '" r="' + r.toFixed(1) + '" fill="' + pt.col + '" opacity="' + (opacity * pulse).toFixed(2) + '"' +
            ' stroke="' + pt.col + '" stroke-width="1"/>';

    // 名称
    html += '<text x="' + pt.x.toFixed(1) + '" y="' + (pt.y + r + 8).toFixed(1) +
            '" fill="' + pt.col + '" font-size="' + (Math.max(7, r * 0.6)).toFixed(1) +
            '" text-anchor="middle" class="cf">' + pt.n + '</text>';
    html += '</g>';
  }

  svg.innerHTML = html;
}

function svg_line(pt, r) {
  let x = pt.x, y = pt.y;
  let h = r * 0.7;
  switch(pt.t) {
    case 'sphere':
      return '<ellipse cx="' + x + '" cy="' + y + '" rx="' + (r*1.2).toFixed(1) + '" ry="' + (r*1.2).toFixed(1) + '" fill="none" stroke="' + pt.col + '" stroke-width="0.7" opacity="0.4"/>' +
             '<path d="M ' + (x-h*1.2) + ',' + y + ' A ' + (h*1.2) + ',' + (h*0.45) + ' 0 0 1 ' + (x+h*1.2) + ',' + y + '" fill="none" stroke="' + pt.col + '" stroke-width="0.5" opacity="0.3"/>';
    case 'torus':
      return '<ellipse cx="' + x + '" cy="' + y + '" rx="' + (h*1.3).toFixed(1) + '" ry="' + (h*1.3).toFixed(1) + '" fill="none" stroke="' + pt.col + '" stroke-width="0.8" opacity="0.4"/>' +
             '<ellipse cx="' + x + '" cy="' + y + '" rx="' + (h*0.7).toFixed(1) + '" ry="' + (h*0.7).toFixed(1) + '" fill="none" stroke="' + pt.col + '" stroke-width="0.6" opacity="0.3"/>';
    case 'box':
      let s2 = h * 0.9;
      return '<rect x="' + (x-s2).toFixed(1) + '" y="' + (y-s2).toFixed(1) + '" width="' + (s2*2).toFixed(1) + '" height="' + (s2*2).toFixed(1) + '" fill="none" stroke="' + pt.col + '" stroke-width="0.7" opacity="0.4" transform="rotate(15 ' + x + ' ' + y + ')"/>';
    case 'bicone':
      return '<polygon points="' + x + ',' + (y-h) + ',' + (x+h*0.8) + ',' + (y+h*0.5) + ',' + (x-h*0.8) + ',' + (y+h*0.5) + '" fill="none" stroke="' + pt.col + '" stroke-width="0.7" opacity="0.4"/>';
    case 'line':
      return '<line x1="' + (x-h) + '" y1="' + y + '" x2="' + (x+h) + '" y2="' + y + '" stroke="' + pt.col + '" stroke-width="1" opacity="0.4"/>';
    case 'tube':
      return '<line x1="' + (x-h*0.7) + '" y1="' + (y-h*0.4) + '" x2="' + (x+h*0.7) + '" y2="' + (y-h*0.4) + '" stroke="' + pt.col + '" stroke-width="0.6" opacity="0.4"/>' +
             '<line x1="' + (x-h*0.7) + '" y1="' + (y+h*0.4) + '" x2="' + (x+h*0.7) + '" y2="' + (y+h*0.4) + '" stroke="' + pt.col + '" stroke-width="0.6" opacity="0.4"/>';
    case 'grid':
      let g = h * 0.6;
      return '<path d="M ' + (x-g) + ',' + (y-g) + ' L ' + (x+g) + ',' + (y-g) + ' L ' + (x+g) + ',' + (y+g) + ' L ' + (x-g) + ',' + (y+g) + ' z" fill="none" stroke="' + pt.col + '" stroke-width="0.5" opacity="0.3"/>' +
             '<line x1="' + x + '" y1="' + (y-g) + '" x2="' + x + '" y2="' + (y+g) + '" stroke="' + pt.col + '" stroke-width="0.3" opacity="0.3"/>' +
             '<line x1="' + (x-g) + '" y1="' + y + '" x2="' + (x+g) + '" y2="' + y + '" stroke="' + pt.col + '" stroke-width="0.3" opacity="0.3"/>';
    default:
      return '<circle cx="' + x + '" cy="' + y + '" r="2" fill="' + pt.col + '" opacity="0.5"/>';
  }
}

// 鼠标拖动
svg.addEventListener('mousedown', function(e) { dragging = true; lastX = e.clientX; lastY = e.clientY; e.preventDefault(); });
window.addEventListener('mouseup', function() { dragging = false; });
window.addEventListener('mousemove', function(e) {
  if (!dragging) return;
  azimuth += (e.clientX - lastX) * 0.008;
  elevation -= (e.clientY - lastY) * 0.006;
  elevation = Math.max(-1.3, Math.min(1.3, elevation));
  lastX = e.clientX; lastY = e.clientY;
});

// 滚轮缩放
svg.addEventListener('wheel', function(e) {
  e.preventDefault();
  dist = Math.max(150, Math.min(800, dist + e.deltaY * 0.5));
}, { passive: false });

// 时间循环
setInterval(draw, 50);
draw();
</script>
</body></html>
"#);

    out
}

fn main() {
    let containers = load_containers();
    println!("▸ 64 Container: {} 条目", containers.len());
    let html = build_html(&containers);
    let out_path = "/tmp/container_3d_cube.html";
    std::fs::write(out_path, &html).expect("写入失败");
    println!("▸ 输出: {} ({}字节)", out_path, html.len());
}
