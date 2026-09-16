// ═══════════════════════════════════════════════════════════════════════════════
// svg_renderer.rs — 笔画容器编码 → SVG 可视字形渲染
// ═══════════════════════════════════════════════════════════════════════════════
//
// 输入: 字符的笔画容器编码 (来自 CharacterCreator)
// 输出: SVG 文档 — 可视汉字字形
//
// 每个笔画类型在 100×100 设计网格中有标准路径:
//   横 → 水平线, 竖 → 垂直线, 撇 → 左下斜线, 捺 → 右下斜线,
//   点 → 小斜点, 提 → 右上斜线, 折 → 折线, 钩 → 钩形
//
// ═══════════════════════════════════════════════════════════════════════════════

use crate::character_creator::{CharacterCreator, GlyphEncoding, CONTAINER_NAMES, STROKE_NAMES};
use crate::character_creator::{
    STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA, STROKE_DIAN, STROKE_TI,
    STROKE_HENGZHE, STROKE_SHUZHE, STROKE_HENGPIE, STROKE_SHUGOU, STROKE_HENGGOU,
    STROKE_WANGOU, STROKE_SHUWANGOU, STROKE_HENGZHEZHE, STROKE_SHUWAN, STROKE_XIEGOU,
};

/// 设计网格大小
pub const GRID_SIZE: f64 = 100.0;
pub const STROKE_WIDTH: f64 = 4.0;

/// 将单个笔画类型转为 SVG path 数据 (在 grid 单位坐标系内)
pub fn stroke_path(stroke_type: u8) -> &'static str {
    match stroke_type {
        STROKE_HENG       => "M 10,50 L 90,50",                          // 横
        STROKE_SHU        => "M 50,10 L 50,90",                          // 竖
        STROKE_PIE        => "M 60,15 L 35,85",                          // 撇
        STROKE_NA         => "M 35,20 L 65,85",                          // 捺
        STROKE_DIAN       => "M 45,30 L 55,45",                          // 点
        STROKE_TI         => "M 35,70 L 65,20",                          // 提
        STROKE_HENGZHE    => "M 15,20 L 60,20 L 60,50 L 45,80",          // 横折
        STROKE_SHUZHE     => "M 30,15 L 30,55 L 75,55",                  // 竖折
        STROKE_HENGPIE    => "M 20,30 L 60,30 L 45,75",                  // 横撇
        STROKE_SHUGOU     => "M 30,15 L 30,60 L 55,70",                  // 竖钩
        STROKE_HENGGOU    => "M 20,30 L 65,30 L 65,40",                  // 横钩
        STROKE_WANGOU     => "M 25,25 L 25,60 Q 25,75 40,75",            // 弯钩
        STROKE_SHUWANGOU  => "M 30,15 L 30,55 Q 30,80 60,80 L 75,75",     // 竖弯钩
        STROKE_HENGZHEZHE => "M 15,25 L 50,25 L 50,55 L 35,80",          // 横折折
        STROKE_SHUWAN     => "M 55,15 L 55,50 Q 55,75 35,75",            // 竖弯
        STROKE_XIEGOU     => "M 55,15 L 25,75 L 45,75",                  // 斜钩
        _                 => "M 10,50 L 90,50",
    }
}

/// 笔画颜色 (频谱映射: 位位置 → 色相)
pub fn stroke_color(stroke_type: u8) -> String {
    let hue = (stroke_type as u16 * 24) % 360;
    format!("hsl({},70%,45%)", hue)
}

/// 渲染单个字符为 SVG
pub fn render_glyph_svg(ch: char) -> Option<String> {
    let enc = CharacterCreator::encode(ch)?;

    let mut paths = String::new();
    let strokes = &enc.stroke_codes;

    // 布局: 根据笔画数决定排列方式
    let n = strokes.len();
    if n == 0 {
        return None;
    }

    // 按笔画顺序渲染，每个笔画在其"自然位置"
    // 使用字符的容器单元 bit 模式推断布局
    for (i, &stroke) in strokes.iter().enumerate() {
        let path = stroke_path(stroke);
        let color = stroke_color(stroke);

        // 偏移: 根据笔画序列位置做微小偏移，模拟真实书写
        let (dx, dy) = stroke_layout_offset(i, n, stroke);

        paths.push_str(&format!(
            r#"<path d="{}" stroke="{}" stroke-width="{}" fill="none" stroke-linecap="round" stroke-linejoin="round" transform="translate({},{})"/>"#,
            path, color, STROKE_WIDTH, dx, dy
        ));
        paths.push('\n');
    }

    // 信息标注
    let info = format!(
        "字符: {} | 笔画: {} | 容器单元: 0x{:016X} | 极性: {}",
        enc.ch,
        n,
        enc.container_cell,
        match enc.polarity {
            crate::character_creator::GlyphPolarity::Yin => "阴",
            crate::character_creator::GlyphPolarity::Yang => "阳",
        }
    );

    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="-10 -10 120 120" width="300" height="300">
  <rect x="-10" y="-10" width="120" height="120" fill="#fafafa" stroke="#ddd"/>
  <text x="50" y="-2" text-anchor="middle" font-size="8" fill="#666">{}</text>
  <g>{}</g>
  <text x="50" y="115" text-anchor="middle" font-size="5" fill="#999">{}</text>
</svg>"##,
        info,
        paths,
        enc.container_names.join("+")
    );

    Some(svg)
}

/// 渲染容器单元 bit 图 (显示 16 位笔画空间)
pub fn render_cell_bitmap(cell: u64) -> String {
    let mut svg = String::new();
    svg.push_str(r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 400 210" width="400" height="210">"##);

    for bit in 0..16u8 {
        let x = (bit % 4) as f64 * 100.0;
        let y = (bit / 4) as f64 * 50.0;
        let set = (cell >> bit) & 1 == 1;
        let fill = if set { stroke_color(bit) } else { "#eee".to_string() };
        let stroke_name = STROKE_NAMES.get(bit as usize).unwrap_or(&"?");
        let container_name = CONTAINER_NAMES.get(bit as usize).unwrap_or(&"?");

        svg.push_str(&format!(
            r##"<rect x="{}" y="{}" width="95" height="45" fill="{}" stroke="#333" stroke-width="2" rx="4"/>"##,
            x + 2.0, y + 2.0, fill
        ));
        svg.push_str(&format!(
            r##"<text x="{}" y="{}" text-anchor="middle" font-size="14" fill="white" font-weight="bold">{} {}</text>"##,
            x + 50.0, y + 25.0, bit, container_name
        ));
        svg.push_str(&format!(
            r##"<text x="{}" y="{}" text-anchor="middle" font-size="8" fill="rgba(255,255,255,0.8)">{}</text>"##,
            x + 50.0, y + 38.0, stroke_name
        ));
    }

    // bit38/39 标注
    for bit in [38u8, 39u8] {
        let x = (bit - 36) as f64 * 100.0;
        let y = 160.0;
        let set = (cell >> bit) & 1 == 1;
        let fill = if set { "#e91e63" } else { "#ddd" };
        let label = if bit == 38 { "妻子" } else { "丈夫" };
        svg.push_str(&format!(
            r##"<rect x="{}" y="{}" width="95" height="35" fill="{}" stroke="#333" stroke-width="2" rx="4"/>"##,
            x + 2.0, y + 2.0, fill
        ));
        svg.push_str(&format!(
            r##"<text x="{}" y="{}" text-anchor="middle" font-size="12" fill="{}" font-weight="bold">{} bit{}</text>"##,
            x + 50.0, y + 22.0,
            if set { "white" } else { "#999" },
            label, bit
        ));
    }

    svg.push_str("</svg>");
    svg
}

/// 计算笔画在序列中的布局偏移
fn stroke_layout_offset(index: usize, total: usize, stroke_type: u8) -> (f64, f64) {
    match total {
        1 => (0.0, 0.0),
        2 => {
            // 二分: 左/右 or 上/下
            if index == 0 { (-12.0, 0.0) } else { (12.0, 0.0) }
        }
        3 => {
            // 三分
            let offsets = [(-15.0, -5.0), (15.0, -5.0), (0.0, 15.0)];
            offsets[index % 3]
        }
        4 => {
            // 四分 (田字格)
            let offsets = [(-12.0, -12.0), (12.0, -12.0), (-12.0, 12.0), (12.0, 12.0)];
            offsets[index % 4]
        }
        n => {
            // 多笔画: 环绕分布
            let angle = (index as f64) * 2.0 * std::f64::consts::PI / (n as f64);
            let r = 15.0_f64.min(n as f64 * 2.0);
            (angle.cos() * r, angle.sin() * r * 0.6)
        }
    }
}

/// 渲染一个字符 + 其容器单元图为完整文档
pub fn render_character_sheet(ch: char) -> Option<String> {
    let glyph_svg = render_glyph_svg(ch)?;
    let enc = CharacterCreator::encode(ch)?;
    let bitmap_svg = render_cell_bitmap(enc.container_cell);

    Some(format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html>
<head><meta charset="utf-8"><title>造字系统 · {}</title>
<style>
  body {{ font-family: monospace; margin: 20px; background: #f5f5f5; }}
  .container {{ display: flex; gap: 20px; flex-wrap: wrap; }}
  .card {{ background: white; padding: 15px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); }}
  h1 {{ font-size: 18px; margin: 0 0 10px; }}
  .info {{ font-size: 12px; color: #666; margin: 8px 0; }}
</style>
</head>
<body>
<h1>造字系统 · 字形可视化 · {}</h1>
<div class="container">
  <div class="card">
    <div class="info">笔画路径 (容器→可视)</div>
    {}
  </div>
  <div class="card">
    <div class="info">容器单元 ({}-bit)</div>
    {}
  </div>
</div>
<div class="info">
  笔画序列: {}<br/>
  容器集合: {}<br/>
  显示容器: {} · 极性: {} · 侧面: {}
</div>
</body>
</html>"##,
        ch, ch,
        glyph_svg,
        64, bitmap_svg,
        enc.stroke_names.join(" → "),
        enc.container_names.join(" + "),
        enc.display_container,
        match enc.polarity {
            crate::character_creator::GlyphPolarity::Yin => "阴",
            crate::character_creator::GlyphPolarity::Yang => "阳",
        },
        match enc.side {
            crate::character_creator::GlyphSide::Wife => "妻子侧",
            crate::character_creator::GlyphSide::Husband => "丈夫侧",
        }
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stroke_path_all_16() {
        for i in 0..16u8 {
            let p = stroke_path(i);
            assert!(!p.is_empty());
            assert!(p.starts_with("M"));
        }
    }

    #[test]
    fn test_stroke_color_valid() {
        for i in 0..16u8 {
            let c = stroke_color(i);
            assert!(c.starts_with("hsl("));
        }
    }

    #[test]
    fn test_render_glyph_svg_nan() {
        let svg = render_glyph_svg('囡');
        assert!(svg.is_some());
        let s = svg.unwrap();
        assert!(s.contains("<svg"));
        assert!(s.contains("</svg>"));
        assert!(s.contains("囡"));
    }

    #[test]
    fn test_render_cell_bitmap() {
        let cell = CharacterCreator::strokes_to_cell(&[STROKE_HENG, STROKE_SHU, STROKE_PIE]);
        let svg = render_cell_bitmap(cell);
        assert!(svg.contains("<svg"));
        assert!(svg.contains("袋"));
        assert!(svg.contains("桶"));
        assert!(svg.contains("罐"));
    }

    #[test]
    fn test_render_character_sheet() {
        let html = render_character_sheet('囡');
        assert!(html.is_some());
        let h = html.unwrap();
        assert!(h.contains("<!DOCTYPE html>"));
        assert!(h.contains("<svg"));
    }

    #[test]
    fn test_all_strokes_renderable() {
        // 确保所有笔画都能编码并渲染
        for ch in "一二人八大天夫女子火山木金土王心" .chars() {
            let svg = render_glyph_svg(ch);
            assert!(svg.is_some(), "should render {}", ch);
        }
    }
}
