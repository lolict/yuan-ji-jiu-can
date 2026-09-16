// 统计文档聚合浏览器 — 流水线全量数据 → 单一自包含 HTML
//
// 输出: 一个 HTML 文件，嵌入 CSS + SVG，可直接在浏览器打开。
// 内容聚合:
//   1. 分类总览表 (状态/类别/轨道/语义)
//   2. 距离矩阵热力图 (SVG)
//   3. 聚类结果 + 连接包裹树 (ASCII + SVG)
//   4. 字体 MCOD 摘要
//   5. 执行裁决验证
//
// 用法:
//   let html = generate_stats_html(&classifications, &font_entries, &verification_results);
//   std::fs::write("stats.html", html).unwrap();

use crate::cross_isa::state_enumerator::*;
use crate::cross_isa::situation_perception::*;
use crate::cross_isa::container_font::FontEntry;

/// 验证结果汇总
pub struct VerificationResult {
    pub check_name: String,
    pub passed: bool,
    pub detail: String,
}

/// 完整流水线输出包
pub struct PipelineOutput {
    pub classifications: Vec<StateClassification>,
    pub font_bytes_len: usize,
    pub font_entries: Vec<FontEntry>,
    pub situation: SituationCluster,
    pub tree: WrapperTree,
    pub verifications: Vec<VerificationResult>,
    pub bit_width: u8,
    pub total_states: usize,
}

/// 生成完整的统计文档 HTML
pub fn generate_stats_html(output: &PipelineOutput) -> String {
    let mut html = String::new();

    html.push_str("<!DOCTYPE html>\n<html lang=\"zh\"><head><meta charset=\"UTF-8\">");
    html.push_str("<title>MQ-OS 统计文档聚合浏览器</title>");
    html.push_str(&CSS_STYLES);
    html.push_str("</head><body>");

    // 顶部标题
    html.push_str(&format!(
        r##"<header><h1> MQ-OS 统计文档聚合浏览器</h1>
        <div class="subtitle">状态空间 {bit} 位 · {states} 状态 · {clusters} 簇 · {fonts} 字体字节</div>
        </header>"##,
        bit = output.bit_width,
        states = output.total_states,
        clusters = output.situation.clusters.len(),
        fonts = output.font_bytes_len
    ));

    // 分类总览
    html.push_str("<section><h2>1. 状态分类总览</h2>");
    html.push_str(&generate_classification_table(&output.classifications));
    html.push_str("</section>");

    // 距离矩阵热力图
    html.push_str("<section><h2>2. 距离矩阵热力图</h2>");
    html.push_str(&generate_heatmap_svg(&output.situation));
    html.push_str("</section>");

    // 聚类结果
    html.push_str("<section><h2>3. 态势聚类</h2>");
    html.push_str(&generate_cluster_section(&output.situation));
    html.push_str("</section>");

    // 包裹树
    html.push_str("<section><h2>4. 连接包裹树</h2>");
    html.push_str(&generate_tree_svg(&output.situation));
    html.push_str("</section>");

    // 字体摘要
    html.push_str("<section><h2>5. 字体 MCOD 摘要</h2>");
    html.push_str(&generate_font_table(&output.font_entries));
    html.push_str("</section>");

    // 验证结果
    html.push_str("<section><h2>6. 执行裁决验证</h2>");
    html.push_str(&generate_verification_table(&output.verifications));
    html.push_str("</section>");

    html.push_str("<footer>容器 = 字体 = 时间 · MQ-OS Aggregation Browser</footer>");
    html.push_str("</body></html>");

    html
}

// ── 分类总览表 ──────────────────────────────────────────────────

fn generate_classification_table(classifications: &[StateClassification]) -> String {
    let mut html = String::new();
    html.push_str("<table><thead><tr><th>状态</th><th>位</th><th>类别</th><th>轨道</th><th>拓扑</th><th>语义</th></tr></thead><tbody>");

    for c in classifications {
        if c.orbit.representative != c.state { continue; }
        let bits_str = format!("{:0w$b}", c.state, w = c.bit_width as usize);
        let topology_icon = match c.category {
            StateCategory::AxialContainer => "◯",
            StateCategory::ClosedLoop => "◎",
            StateCategory::MirrorEntangled => "△",
            StateCategory::ExtendedEntangled => "✦",
        };
        html.push_str(&format!(
            "<tr><td class=\"mono\">{}</td><td>{}</td><td class=\"cat-{}\">{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            bits_str,
            c.bit_width,
            c.category.name_cn().replace(" ", "-"),
            c.category.name_cn(),
            c.orbit.len(),
            topology_icon,
            match c.category {
                StateCategory::AxialContainer => "COUPLE",
                StateCategory::ClosedLoop => "YIN_YANG",
                StateCategory::MirrorEntangled => "HOMOPHONE",
                StateCategory::ExtendedEntangled => "POLYSEMY",
            }
        ));
    }
    html.push_str("</tbody></table>");
    html
}

// ── 距离矩阵热力图 (SVG) ────────────────────────────────────────

fn generate_heatmap_svg(situation: &SituationCluster) -> String {
    let n = situation.matrix.labels.len();
    if n == 0 { return String::from("<p>无数据</p>"); }

    let cell_size = 40u32;
    let margin = 60u32;
    let width = margin + n as u32 * cell_size + 20;
    let height = margin + n as u32 * cell_size + 20;

    // 找最大值用于归一化
    let mut max_dist = 0.01f64;
    for i in 0..n { for j in 0..n {
        if situation.matrix.matrix[i][j] > max_dist { max_dist = situation.matrix.matrix[i][j]; }
    }}

    let mut svg = format!("<svg viewBox=\"0 0 {w} {h}\" class=\"heatmap\">", w = width, h = height);

    for i in 0..n {
        for j in 0..n {
            let dist = situation.matrix.matrix[i][j];
            let normalized = (dist / max_dist).min(1.0);
            let x = margin + j as u32 * cell_size;
            let y = margin + i as u32 * cell_size;
            let color = heatmap_color(normalized);
            let text_color = if normalized > 0.5 { "#fff" } else { "#1a1a2e" };

            svg.push_str(&format!(
                r##"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" rx="2"/>"##,
                x, y, cell_size - 1, cell_size - 1, color
            ));
            if dist < 0.01 {
                svg.push_str(&format!(
                    r##"<text x="{}" y="{}" text-anchor="middle" fill="{}" font-size="12">·</text>"##,
                    x + cell_size / 2, y + cell_size / 2 + 4, text_color
                ));
            } else {
                svg.push_str(&format!(
                    r##"<text x="{}" y="{}" text-anchor="middle" fill="{}" font-size="10">{:.1}</text>"##,
                    x + cell_size / 2, y + cell_size / 2 + 4, text_color, dist
                ));
            }
        }
    }

    // 轴标签
    for i in 0..n {
        let y = margin + i as u32 * cell_size + cell_size / 2 + 4;
        svg.push_str(&format!(
            r##"<text x="{}" y="{}" text-anchor="end" fill="#a0a0b0" font-size="10">S{:02}</text>"##,
            margin - 4, y, situation.matrix.labels[i]
        ));
        let x = margin + i as u32 * cell_size + cell_size / 2;
        svg.push_str(&format!(
            r##"<text x="{}" y="{}" text-anchor="middle" fill="#a0a0b0" font-size="10">S{:02}</text>"##,
            x, margin - 8, situation.matrix.labels[i]
        ));
    }

    svg.push_str("</svg>");
    svg
}

/// 热力颜色映射: 蓝(近) → 黄(中) → 红(远)
fn heatmap_color(t: f64) -> String {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 {
        // 蓝 → 黄
        let r = (t * 2.0 * 255.0) as u8;
        let g = (t * 2.0 * 220.0) as u8;
        let b = ((1.0 - t * 2.0) * 200.0 + 55.0) as u8;
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    } else {
        // 黄 → 红
        let tt = (t - 0.5) * 2.0;
        let r = 255;
        let g = ((1.0 - tt) * 200.0 + 20.0) as u8;
        let b = 30;
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }
}

// ── 聚类结果 ──────────────────────────────────────────────────────

fn generate_cluster_section(situation: &SituationCluster) -> String {
    let mut html = String::new();
    html.push_str("<div class=\"cluster-info\">");
    html.push_str(&format!(
        "<div class=\"stat-box\"><span class=\"stat-num\">{}</span><span class=\"stat-lbl\">簇总数</span></div>",
        situation.clusters.len()
    ));
    html.push_str(&format!(
        "<div class=\"stat-box\"><span class=\"stat-num\">{}</span><span class=\"stat-lbl\">状态维度</span></div>",
        situation.classifications.len()
    ));
    if let Some((i, j, dist)) = situation.matrix.nearest_pair() {
        html.push_str(&format!(
            "<div class=\"stat-box\"><span class=\"stat-num sm\">d={:.1}</span><span class=\"stat-lbl\">最近邻(S{}↔S{})</span></div>",
            dist, situation.matrix.labels[i], situation.matrix.labels[j]
        ));
    }
    if let Some((i, j, dist)) = situation.matrix.farthest_pair() {
        html.push_str(&format!(
            "<div class=\"stat-box\"><span class=\"stat-num sm\">d={:.1}</span><span class=\"stat-lbl\">最远对(S{}↔S{})</span></div>",
            dist, situation.matrix.labels[i], situation.matrix.labels[j]
        ));
    }
    html.push_str("</div>");

    // 每个簇的信息
    html.push_str("<div class=\"cluster-list\">");
    for (ci, cluster) in situation.clusters.iter().enumerate() {
        let cat_name = cluster.member_ids.first().and_then(|id| {
            situation.classifications.iter()
                .find(|c| {
                    use crate::cross_isa::situation_perception::encode_classification;
                    encode_classification(c) == *id
                })
                .map(|c| c.category.name_cn())
        }).unwrap_or("未知");

        html.push_str(&format!(r##"<div class="cluster-card cat-border-{}">"##, cat_name.replace(" ", "-")));
        html.push_str(&format!("<h4>簇 {} <span class=\"cat-cat\">{}</span></h4>", ci, cat_name));
        html.push_str("<div class=\"cluster-members\">");
        for &member_id in cluster.member_ids.iter() {
            if let Some(c) = situation.classifications.iter().find(|c| {
                use crate::cross_isa::situation_perception::encode_classification;
                encode_classification(c) == member_id
            }) {
                html.push_str(&format!(
                    "<span class=\"state-tag\">{:0w$b}</span>",
                    c.state, w = c.bit_width as usize
                ));
            }
        }
        html.push_str("</div></div>");
    }
    html.push_str("</div>");
    html
}

// ── 包裹树 SVG ──────────────────────────────────────────────────

fn generate_tree_svg(situation: &SituationCluster) -> String {
    let n = situation.clusters.len();
    if n == 0 { return String::from("<p>无数据</p>"); }

    let width = 600u32;
    let height = (n as u32 * 50 + 60).max(200);
    let mut svg = format!("<svg viewBox=\"0 0 {w} {h}\" class=\"tree\">", w = width, h = height);

    // 中心垂直线
    let cx = width / 2;
    svg.push_str(&format!(
        r##"<line x1="{}" y1="20" x2="{}" y2="{}" stroke="#4a4a6a" stroke-width="2"/>"##,
        cx, cx, height - 20
    ));

    // 每个簇一个分支
    for i in 0..n {
        let y = 30 + i as u32 * ((height - 60) / n.max(1) as u32).min(50);
        let cluster = &situation.clusters[i];
        let cat_color = cluster.member_ids.first().and_then(|id| {
            situation.classifications.iter()
                .find(|c| {
                    use crate::cross_isa::situation_perception::encode_classification;
                    encode_classification(c) == *id
                })
                .map(|c| match c.category {
                    StateCategory::AxialContainer => "#4ecdc4",
                    StateCategory::ClosedLoop => "#f7b731",
                    StateCategory::MirrorEntangled => "#e74c3c",
                    StateCategory::ExtendedEntangled => "#a55eea",
                })
        }).unwrap_or("#888");

        // 横线
        svg.push_str(&format!(
            r##"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="2"/>"##,
            cx, y, cx + 80, y, cat_color
        ));
        // 节点圆
        svg.push_str(&format!(
            r##"<circle cx="{}" cy="{}" r="{}" fill="{}" stroke="#fff" stroke-width="1"/>"##,
            cx + 80, y, 6, cat_color
        ));
        // 标签
        svg.push_str(&format!(
            r##"<text x="{}" y="{}" fill="#ddd" font-size="11">簇{}[{}]</text>"##,
            cx + 92, y + 4, i, cluster.size()
        ));
    }

    svg.push_str("</svg>");
    svg
}

// ── 字体摘要表 ──────────────────────────────────────────────────

fn generate_font_table(entries: &[FontEntry]) -> String {
    let mut html = String::new();
    html.push_str("<table><thead><tr><th>Pack ID</th><th>类别</th><th>位宽</th><th>轨道</th><th>代表元</th></tr></thead><tbody>");
    for e in entries.iter().take(16) {
        let cat_name = match e.category {
            1 => "轴心容器态",
            2 => "闭环态",
            3 => "镜像纠缠态",
            4 => "扩展纠缠态",
            _ => "未知",
        };
        html.push_str(&format!(
            "<tr><td class=\"mono\">0x{:04X}</td><td>{}</td><td>{}</td><td>{}</td><td class=\"mono\">0x{:04X}</td></tr>",
            e.pack_id, cat_name, e.bit_width, e.orbit_len, e.representative
        ));
    }
    if entries.len() > 16 {
        html.push_str(&format!("<tr><td colspan=\"5\" class=\"more\">... 共 {} 条</td></tr>", entries.len()));
    }
    html.push_str("</tbody></table>");
    html
}

// ── 验证结果表 ──────────────────────────────────────────────────

fn generate_verification_table(verifications: &[VerificationResult]) -> String {
    let mut html = String::new();
    html.push_str("<table><thead><tr><th>检查项</th><th>结果</th><th>说明</th></tr></thead><tbody>");
    for v in verifications {
        let (cls, sym) = if v.passed {
            ("pass", "✓")
        } else {
            ("fail", "✗")
        };
        html.push_str(&format!(
            "<tr><td>{}</td><td class=\"{}\">{}</td><td class=\"detail\">{}</td></tr>",
            v.check_name, cls, sym, v.detail
        ));
    }
    html.push_str("</tbody></table>");
    html
}

// ── CSS 样式 ──────────────────────────────────────────────────────

const CSS_STYLES: &str = r##"
<style>
* { margin: 0; padding: 0; box-sizing: border-box; }
body {
    font-family: 'SF Mono', 'Fira Code', 'Cascadia Code', monospace;
    background: #0a0a1a;
    color: #e0e0f0;
    line-height: 1.6;
    padding: 0;
}
header {
    background: linear-gradient(135deg, #1a1a3e 0%, #2d1b4e 100%);
    padding: 30px 40px;
    border-bottom: 2px solid #4ecdc4;
}
header h1 { font-size: 1.5em; color: #fff; }
header .subtitle { color: #9090b0; font-size: 0.9em; margin-top: 6px; }
section {
    margin: 20px 40px;
    padding: 20px;
    background: #12122a;
    border-radius: 8px;
    border: 1px solid #2a2a4a;
}
section h2 {
    color: #4ecdc4;
    font-size: 1.1em;
    margin-bottom: 16px;
    padding-bottom: 8px;
    border-bottom: 1px solid #2a2a4a;
}
table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.85em;
}
th {
    background: #1a1a3e;
    color: #9090b0;
    text-align: left;
    padding: 8px 10px;
    font-weight: normal;
    border-bottom: 2px solid #4ecdc4;
}
td {
    padding: 7px 10px;
    border-bottom: 1px solid #1a1a3e;
}
tr:hover td { background: #1a1a3e; }
.mono { font-family: monospace; color: #f7b731; }
.cat-轴心容器态 { color: #4ecdc4; }
.cat-闭环态 { color: #f7b731; }
.cat-镜像纠缠态 { color: #e74c3c; }
.cat-扩展纠缠态 { color: #a55eea; }
.pass { color: #4ecdc4; font-weight: bold; }
.fail { color: #e74c3c; font-weight: bold; }
.detail { color: #7070a0; font-size: 0.9em; }
.cluster-info {
    display: flex;
    gap: 12px;
    flex-wrap: wrap;
    margin-bottom: 20px;
}
.stat-box {
    flex: 1;
    min-width: 120px;
    padding: 14px;
    background: #1a1a3e;
    border-radius: 6px;
    text-align: center;
    border: 1px solid #2a2a4a;
}
.stat-num {
    display: block;
    font-size: 1.8em;
    color: #4ecdc4;
    font-weight: bold;
}
.stat-num.sm { font-size: 1.2em; }
.stat-lbl {
    display: block;
    font-size: 0.75em;
    color: #7070a0;
    margin-top: 4px;
}
.cluster-list {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 10px;
    margin-top: 14px;
}
.cluster-card {
    padding: 12px;
    background: #1a1a3e;
    border-radius: 6px;
    border-left: 3px solid #4ecdc4;
}
.cluster-card h4 {
    font-size: 0.9em;
    margin-bottom: 6px;
    color: #ddd;
}
.cat-cat { color: #9090b0; font-weight: normal; font-size: 0.8em; }
.cluster-members {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
    margin-top: 6px;
}
.state-tag {
    padding: 2px 6px;
    background: #2a2a4a;
    border-radius: 3px;
    font-size: 0.75em;
    color: #f7b731;
    font-family: monospace;
}
.heatmap { width: 100%; max-width: 400px; margin: 0 auto; display: block; }
.tree { width: 100%; max-width: 600px; margin: 0 auto; display: block; }
.more { text-align: center; color: #5050a0; font-style: italic; }
footer {
    text-align: center;
    padding: 20px;
    color: #5050a0;
    font-size: 0.8em;
    border-top: 1px solid #2a2a4a;
    margin-top: 30px;
}
</style>
"##;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_valid_html() {
        let output = PipelineOutput {
            classifications: Vec::new(),
            font_bytes_len: 0,
            font_entries: Vec::new(),
            situation: SituationCluster {
                clusters: Vec::new(),
                matrix: DistanceMatrix::new(&[]),
                classifications: Vec::new(),
            },
            tree: WrapperTree { root_nodes: Vec::new() },
            verifications: vec![
                VerificationResult { check_name: "测试".into(), passed: true, detail: "通过".into() },
            ],
            bit_width: 3,
            total_states: 8,
        };
        let html = generate_stats_html(&output);
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("</html>"));
        assert!(html.contains("MQ-OS"));
    }

    #[test]
    fn heatmap_has_svg() {
        let situation = SituationCluster {
            clusters: vec![],
            matrix: DistanceMatrix {
                labels: vec![0, 1],
                matrix: vec![vec![0.0, 5.0], vec![5.0, 0.0]],
            },
            classifications: vec![],
        };
        let svg = generate_heatmap_svg(&situation);
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("<rect"));
    }

    #[test]
    fn verify_pass_row_class() {
        let v = vec![
            VerificationResult { check_name: "A".into(), passed: true, detail: "ok".into() },
            VerificationResult { check_name: "B".into(), passed: false, detail: "bad".into() },
        ];
        let html = generate_verification_table(&v);
        assert!(html.contains("class=\"pass\""));
        assert!(html.contains("class=\"fail\""));
    }
}
