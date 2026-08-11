// ╔══════════════════════════════════════════════════════════════════╗
// ║  NJSG 统计文档聚合浏览器 — 生成器                                  ║
// ║  满全法(NJSG) | 囡囝共生元                                         ║
// ║  对外公开代号: 援技救残                                            ║
// ║                                                                  ║
// ║  功能:                                                            ║
// ║  1. 任意链接(DeepSeek/飞书/民联) → 分配静态 (x,y,z) 坐标           ║
// ║  2. 邻居(coord diff=1) 自动形成「连接包裹」                         ║
// ║  3. 思维树 = 跨包裹的高相似度连接 (similarity > 0.75)                ║
// ║  4. 位置选址: 从 #1 直接跳到 #100,不需要顺序遍历                     ║
// ║  5. 链接断落不影响索引条目,只是 live 字段变为 false                  ║
// ║                                                                  ║
// ║  Copyright © 刘楚恬 & lolict | 囡囝凹月留                           ║
// ╚══════════════════════════════════════════════════════════════════╝

use std::collections::HashMap;
use std::fs;

// ── 数据模型 ──────────────────────────────────────────────────────
// 坐标直接使用 topo.rs 里 Cube4 映射: i → (i%4, (i/4)%4, i/16)
// 邻居定义: Chebyshev max(|dx|,|dy|,|dz|) ≤ 1
// 曼哈顿距离用于思维树的「差距显示」

#[derive(Clone, serde::Serialize)]
struct Coord {
    x: i8,
    y: i8,
    z: i8,
}

// mlil 映射: 用 mqf::mlil 运行期算出四个角色的激活度与主角色
use mqf::mlil::{MlilRole, MlilRoleAssignment};

#[derive(Clone, serde::Serialize)]
struct ContentItem {
    id: u32,
    symbol_index: u8,    // 0..63, 用于邻居计算
    coord: Coord,
    title: String,
    url: String,
    source: String,      // "deepseek-share" / "feishu-doc" / "feishu-topic" / "minlian"
    layer: u8,           // 时序层 0=源头 1=当下 2=延伸
    summary: String,
    keywords: Vec<String>,
    link_alive: bool,    // false 时索引仍在, 只是链接失效
    mlil_primary: String,          // 主角色 label_cn
    mlil_primary_activation: f64,  // 主角色激活度 0..1
    mlil_roles: Vec<MlilRoleAssignment>, // 四个角色激活度排序
    mlil_summary: String,          // 解剖摘要
}

#[derive(serde::Serialize)]
struct NeighborEdge {
    a: u32,
    b: u32,
    manhattan: u32,
    chebyshev: u32,
    src_to_dst_deg: f64,   // 0..360 「我距它多少度」
    dst_to_src_deg: f64,   // 反向度数(非对称感知)
}

#[derive(serde::Serialize)]
struct ConnectionPackage {
    center_id: u32,
    member_ids: Vec<u32>,
}

#[derive(serde::Serialize)]
struct IndexSnapshot {
    generated_at: String,
    total_items: usize,
    alive_items: usize,
    dead_items: usize,
    balance: f64,
    mood: String,
}

#[derive(serde::Serialize)]
struct IndexFile {
    snapshot: IndexSnapshot,
    items: Vec<ContentItem>,
    neighbors: Vec<NeighborEdge>,
    packages: Vec<ConnectionPackage>,
}

// ── 核心算法 ──────────────────────────────────────────────────────

fn coord_of_symbol(i: u8) -> Coord {
    Coord {
        x: (i % 4) as i8,
        y: ((i / 4) % 4) as i8,
        z: (i / 16) as i8,
    }
}

fn chebyshev(a: &Coord, b: &Coord) -> u32 {
    let dx = (a.x - b.x).abs() as u32;
    let dy = (a.y - b.y).abs() as u32;
    let dz = (a.z - b.z).abs() as u32;
    dx.max(dy).max(dz)
}

fn manhattan(a: &Coord, b: &Coord) -> u32 {
    ((a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()) as u32
}

fn src_to_dst_deg(a: &Coord, b: &Coord) -> f64 {
    let dx = (b.x - a.x) as f64;
    let dy = (b.y - a.y) as f64;
    let raw = dy.atan2(dx).to_degrees();
    if raw < 0.0 { raw + 360.0 } else { raw }
}

fn keyword_similarity(a: &[String], b: &[String]) -> f64 {
    if a.is_empty() || b.is_empty() { return 0.0; }
    let mut common = 0u32;
    for ka in a {
        if b.iter().any(|kb| kb == ka) {
            common += 1;
        }
    }
    let denom = (a.len() as f64).max(b.len() as f64);
    common as f64 / denom
}

fn build_data() -> IndexFile {
    // ── 内容清单 ──────────────────────────────
    // 直接由调用方维护。任何链接断落只改 link_alive, 不删条目。
    // 条目标题/优先使用 deepseek-share 原始会话中的第一行用户输入作为锚点(可人工替换)。
    let items_vec: Vec<(&str, &str, &str, u8, &str, Vec<&str>)> = vec![
        // 0 - 古文 OS 隐喻锚(deepseek-share 第一轮用户古文)
        (
            "弥勒佛·共生·OS 隐喻 — 古文散句",
            "https://chat.deepseek.com/share/xkn9h436aktrkl2wsc",
            "deepseek-share",
            1,
            "弥勒佛=未来佛=共生 OS; 燃灯古灯=当代热点引擎; 如来=现世日历翻; 弥勒佛说: 我弥补世界的操作系统漏洞, 你勒紧世界的窟窿欲望眼",
            vec!["弥勒佛", "共生", "OS", "隐喻", "古文", "燃灯", "如来", "漏洞"],
        ),
        // 1 - 古文通诊互动 — 用户反复求诊断哪些句子通
        (
            "古文情绪散句·通诊互动",
            "https://chat.deepseek.com/share/dphie2xw8oedmbnjjx",
            "deepseek-share",
            1,
            "AI(DeepSeek)对用户古文散句的四轮诊断: 哪些通、哪些不通、情绪乱码区段、建议把口语金句单独保留",
            vec!["古文", "情绪", "诊断", "乱码", "弥陀", "通诊", "金句"],
        ),
        // 2 - 谷歌安卓分析 — deepshare 第二轮三段对话
        (
            "谷歌安卓控制论·AOSP/GMS",
            "https://chat.deepseek.com/share/jz5s060ppcbj92a5gj",
            "deepseek-share",
            1,
            "AOSP 地基 + GMS 钥匙; 国内大厂国内市场暂不受影响但出海必守谷歌规; 华为鸿蒙 = 被卡脖子后的求生系统",
            vec!["谷歌", "安卓", "AOSP", "GMS", "华为", "鸿蒙", "出海"],
        ),
        // 3 - 数字红军概念 — 用户自述在 jz5s 会话末尾提出
        (
            "数字红军·红色标签体系",
            "https://chat.deepseek.com/share/jz5s060ppcbj92a5gj",
            "deepseek-share",
            2,
            "用户提出: 数字红军首长/红字示警(既得利益者的红 vs 自救的红); 序列包含红科红工红机红编红媒红捐红法红车红电红店红递红飞红乡红航红程红城红T红宿红房红饮红购红学红教红AI红政红婚红债红时红空红孤红能红源红光红歌红版红权红职红金红令",
            vec!["红军", "红字", "红色", "示警", "自救", "共生", "数字"],
        ),
        // 4 - 刘楚恬拍摄(保持私有 URL,不需要公网可达)
        (
            "刘楚恬 & 凝灯拍摄时序",
            "feishu://docx/abc123",
            "feishu-doc",
            2,
            "刘楚恬本人凝灯拍摄记录, 时序上下层索引到弥勒佛散句创作日期; 私人链接不保证公网可达",
            vec!["刘楚恬", "凝灯", "拍摄", "时序", "共生"],
        ),
        // 5 - 快手符号排版
        (
            "快手符号排版优化",
            "feishu://topic/kuaishou-typeset",
            "feishu-topic",
            2,
            "短视频字幕/符号/排版走型的跨平台保真问题; 私有设备附加符号集的显示一致性",
            vec!["快手", "符号", "排版", "视频", "跨平台"],
        ),
        // 6 - MHT 容器
        (
            "MHT → TXT/DOCX/PDF 容器",
            "feishu://topic/mht-container",
            "feishu-topic",
            0,
            "邮件网页存档(MHT)拆解重组为目标容器格式(TXT/DOCX/PDF); 保留层级化锚点",
            vec!["MHT", "格式", "容器", "转换", "锚点"],
        ),
        // 7 - 创业资源入局
        (
            "创业资源 & 入局路径",
            "feishu://docx/ent2025",
            "feishu-doc",
            2,
            "外部创业资源盘点 + 满全法视角下的入局路径与共生联盟; 借力他人服务器/域名/资金",
            vec!["创业", "资源", "入局", "共生", "满全法"],
        ),
        // 8 - 本地 AI vs 云端
        (
            "本地 AI vs 云端大模型",
            "feishu://topic/local-ai",
            "feishu-topic",
            0,
            "本地推理(手机芯片)能力边界 vs 云端积分制商业模式; 元输入法作为本地智能载体",
            vec!["本地", "云端", "AI", "推理", "积分", "元输入法"],
        ),
        // 9 - 范畴论
        (
            "范畴论·智能电路构建",
            "feishu://topic/catgr-build",
            "feishu-topic",
            1,
            "满全法底层范畴引擎: 智能即电路, 通道即吸引子; 空间几何范畴论实现关系归属与身份准入",
            vec!["范畴论", "电路", "吸引子", "智能", "几何"],
        ),
        // 10 - 民联浏览器对外通道
        (
            "民联浏览器·对外通道",
            "https://minlian.example/njsg",
            "minlian",
            0,
            "民联浏览器作为聚合对外域名; 不暴露满全法内核; 仅输出统计文档聚合视图",
            vec!["民联", "浏览器", "NJSG", "满全法", "通道", "统计"],
        ),
        // 11 - 模型制作
        (
            "模型制作教程资源(毛利兰/kndl/灰原哀)",
            "feishu://model/make2026",
            "feishu-doc",
            2,
            "3D/概念模型制作教程/资源索引; 毛利兰=模/立体, kndl=外壳, 工藤新一=旋翼, 灰原哀=横/行",
            vec!["模型", "教程", "毛利兰", "kndl", "3D", "工藤新一", "灰原哀"],
        ),
    ];

    let items: Vec<ContentItem> = items_vec
        .into_iter()
        .enumerate()
        .map(|(i, (title, url, source, layer, summary, kws))| {
            let coord = coord_of_symbol((i % 64) as u8);
            let kws_str: Vec<String> = kws.iter().map(|s| s.to_string()).collect();
            let anatomy = mqf::mlil::assign_roles(
                layer,
                &(coord.x, coord.y, coord.z),
                &kws_str,
                source,
            );
            let primary = anatomy.roles.first().cloned().unwrap_or_else(|| MlilRoleAssignment {
                role: mqf::mlil::MlilRole::Maori,
                activation: 0.25,
                note: "未分".to_string(),
            });
            ContentItem {
                id: i as u32,
                symbol_index: (i % 64) as u8,
                coord: coord.clone(),
                title: title.to_string(),
                url: url.to_string(),
                source: source.to_string(),
                layer,
                summary: summary.to_string(),
                keywords: kws_str,
                link_alive: true,
                mlil_primary: primary.role.label_cn().to_string(),
                mlil_primary_activation: primary.activation,
                mlil_roles: anatomy.roles,
                mlil_summary: anatomy.summary,
            }
        })
        .collect();

    // ── 邻居(Chebyshev == 1) ─────────────────
    let mut neighbors: Vec<NeighborEdge> = Vec::new();
    for i in 0..items.len() {
        for j in (i + 1)..items.len() {
            let c = chebyshev(&items[i].coord, &items[j].coord);
            if c <= 1 {
                neighbors.push(NeighborEdge {
                    a: items[i].id,
                    b: items[j].id,
                    manhattan: manhattan(&items[i].coord, &items[j].coord),
                    chebyshev: c,
                    src_to_dst_deg: src_to_dst_deg(&items[i].coord, &items[j].coord),
                    dst_to_src_deg: src_to_dst_deg(&items[j].coord, &items[i].coord),
                });
            }
        }
    }

    // ── 连接包裹 (高相似邻居聚合) ─────────────
    let mut adjacency: HashMap<u32, Vec<u32>> = HashMap::new();
    for n in &neighbors {
        adjacency.entry(n.a).or_default().push(n.b);
        adjacency.entry(n.b).or_default().push(n.a);
    }
    let mut packages: Vec<ConnectionPackage> = Vec::new();
    let mut seen = HashMap::new();
    for item in &items {
        let mut members: Vec<u32> = vec![item.id];
        if let Some(ns) = adjacency.get(&item.id) {
            for n in ns {
                let sim = keyword_similarity(&item.keywords, &items[*n as usize].keywords);
                if sim >= 0.12 {
                    members.push(*n);
                }
            }
        }
        members.sort();
        let key = members.clone();
        if !seen.contains_key(&key) {
            seen.insert(key, true);
            if members.len() >= 2 {
                packages.push(ConnectionPackage { center_id: item.id, member_ids: members });
            }
        }
    }

    // ── 统计锥·平衡·情绪 ─────────────────────
    let total = items.len();
    let alive = items.iter().filter(|i| i.link_alive).count();
    let dead = total - alive;
    let balance = if total == 0 { 0.0 } else { alive as f64 / total as f64 };
    let mood = if balance >= 0.9 {
        "稳定"
    } else if balance >= 0.6 {
        "韧性"
    } else if balance >= 0.3 {
        "勉强"
    } else {
        "危急"
    };

    let generated_at = "2026-08-10T12:00:00Z".to_string();

    IndexFile {
        snapshot: IndexSnapshot {
            generated_at,
            total_items: total,
            alive_items: alive,
            dead_items: dead,
            balance,
            mood: mood.to_string(),
        },
        items,
        neighbors,
        packages,
    }
}

fn render_html(index: &IndexFile) -> String {
    let items_json = serde_json::to_string_pretty(&index.items).unwrap();
    let neighbors_json = serde_json::to_string_pretty(&index.neighbors).unwrap();
    let packages_json = serde_json::to_string_pretty(&index.packages).unwrap();
    let snapshot_json = serde_json::to_string_pretty(&index.snapshot).unwrap();

    format!(
        r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>民联 · 统计文档聚合浏览器 · NJSG</title>
<style>
:root {{
  --bg: #0a0e1a; --panel: #111827; --edge: #1e293b;
  --text: #e2e8f0; --dim: #94a3b8; --accent: #38bdf8;
  --plus: #10b981; --zero: #f59e0b; --minus: #ef4444;
}}
* {{ box-sizing: border-box; margin:0; padding:0; }}
body {{ font-family: -apple-system, "PingFang SC", "Helvetica Neue", sans-serif;
  background: var(--bg); color: var(--text); line-height: 1.5; }}
header {{ padding: 16px 24px; border-bottom: 1px solid var(--edge);
  display: flex; align-items: center; gap: 16px; flex-wrap: wrap; }}
header h1 {{ font-size: 18px; font-weight: 600; }}
header .meta {{ color: var(--dim); font-size: 13px; }}
header .mood {{ background: var(--edge); padding: 4px 12px; border-radius: 12px;
  font-size: 12px; }}
main {{ display: grid; grid-template-columns: 320px 1fr 320px; gap: 0;
  height: calc(100vh - 57px); }}
.panel {{ border-right: 1px solid var(--edge); overflow-y: auto; padding: 16px; }}
.panel:last-child {{ border-right: none; }}
.panel h2 {{ font-size: 13px; color: var(--dim); margin-bottom: 12px;
  text-transform: uppercase; letter-spacing: 1px; }}
.content-card {{ background: var(--panel); border: 1px solid var(--edge);
  border-radius: 8px; padding: 10px 12px; margin-bottom: 8px; cursor: pointer; }}
.content-card:hover {{ border-color: var(--accent); }}
.content-card .row1 {{ display: flex; justify-content: space-between; align-items: center; }}
.content-card .id {{ font-size: 11px; color: var(--dim); }}
.content-card .coord {{ font-size: 11px; color: var(--accent); font-family: monospace; }}
.content-card .title {{ font-size: 14px; font-weight: 500; margin: 4px 0 2px; }}
.content-card .summary {{ font-size: 12px; color: var(--dim); line-height: 1.4;
  overflow: hidden; display: -webkit-box; -webkit-line-clamp: 2;
  -webkit-box-orient: vertical; }}
.content-card .src {{ font-size: 10px; color: var(--dim); margin-top: 4px;
  display: flex; gap: 8px; align-items: center; }}
.src-tag {{ padding: 1px 6px; border-radius: 4px; background: var(--edge); }}
.card-active {{ border-color: var(--accent) !important; background: #0c1425; }}
.detail-line {{ font-size: 13px; padding: 6px 0; border-bottom: 1px solid var(--edge);
  display: flex; justify-content: space-between; }}
.detail-line .k {{ color: var(--dim); }}
.neighbor-row {{ font-size: 12px; padding: 6px 8px; border-radius: 4px;
  background: var(--panel); margin-bottom: 4px; cursor: pointer;
  display: flex; justify-content: space-between; align-items: center; }}
.neighbor-row:hover {{ background: var(--edge); }}
.deg {{ color: var(--accent); font-family: monospace; font-size: 11px; }}
.pkg-card {{ background: var(--panel); border-radius: 8px; padding: 10px;
  margin-bottom: 12px; border: 1px solid var(--edge); }}
.pkg-title {{ font-size: 12px; color: var(--dim); margin-bottom: 6px; }}
.pkg-members {{ display: flex; flex-wrap: wrap; gap: 4px; }}
.pkg-chip {{ background: var(--edge); border-radius: 12px; padding: 2px 8px;
  font-size: 11px; cursor: pointer; }}
.pkg-chip:hover {{ background: var(--accent); color: var(--bg); }}
input[type="text"] {{ width: 100%; padding: 8px 12px; background: var(--panel);
  border: 1px solid var(--edge); border-radius: 6px; color: var(--text);
  font-size: 13px; margin-bottom: 12px; }}
input:focus {{ outline: none; border-color: var(--accent); }}
.stat-grid {{ display: grid; grid-template-columns: 1fr 1fr; gap: 8px; margin-bottom: 16px; }}
.stat-cell {{ background: var(--panel); border-radius: 6px; padding: 8px; text-align: center; }}
.stat-cell .v {{ font-size: 20px; font-weight: 600; color: var(--accent); }}
.stat-cell .k {{ font-size: 11px; color: var(--dim); }}
</style>
</head>
<body>

<header>
  <h1>民联 · 统计文档聚合浏览器</h1>
  <span class="meta">NJSG · 囡囝共生元 · 援技救残</span>
  <span class="meta" id="snapshot-meta"></span>
  <span class="mood" id="mood-tag"></span>
</header>

<main>
  <!-- 左: 内容清单 -->
  <section class="panel" id="panel-left">
    <h2>内容清单 (静态坐标)</h2>
    <input type="text" id="search" placeholder="搜索标题/关键词……">
    <div id="content-list"></div>
  </section>

  <!-- 中: 详情 + 位置选址 -->
  <section class="panel" id="panel-mid">
    <h2>位置选址 · 详情</h2>
    <div id="detail-view" style="color:var(--dim);font-size:13px;">
      点击左侧条目查看
    </div>
  </section>

  <!-- 右: 邻居 + 包裹 -->
  <section class="panel" id="panel-right">
    <h2>连接包裹 · 思维树</h2>
    <div id="package-view" style="color:var(--dim);font-size:13px;">
      点击左侧条目查看
    </div>
  </section>
</main>

<script>
const ITEMS = {items_json};
const NEIGHBORS = {neighbors_json};
const PACKAGES = {packages_json};
const SNAPSHOT = {snapshot_json};

// ── 统计锥头部 ──
document.getElementById('snapshot-meta').textContent =
  `共 ${{SNAPSHOT.total_items}} 条 · ${{SNAPSHOT.alive_items}} 活 / ${{SNAPSHOT.dead_items}} 断`;
document.getElementById('mood-tag').textContent = '系统情绪: ' + SNAPSHOT.mood;

// ── 建立邻居索引 ──
const neighborIdx = {{}};
NEIGHBORS.forEach(n => {{
  if (!neighborIdx[n.a]) neighborIdx[n.a] = [];
  if (!neighborIdx[n.b]) neighborIdx[n.b] = [];
  neighborIdx[n.a].push(n);
  neighborIdx[n.b].push({{...n, a: n.b, b: n.a,
    src_to_dst_deg: n.dst_to_src_deg, dst_to_src_deg: n.src_to_dst_deg}});
}});

// ── 内容卡片渲染 ──
function renderList(filter = '') {{
  const root = document.getElementById('content-list');
  root.innerHTML = '';
  const f = filter.toLowerCase().trim();
  ITEMS.forEach(item => {{
    const match = !f ||
      item.title.toLowerCase().includes(f) ||
      item.summary.toLowerCase().includes(f) ||
      item.keywords.some(k => k.toLowerCase().includes(f));
    if (!match) return;
    const card = document.createElement('div');
    card.className = 'content-card';
    card.dataset.id = item.id;
    card.innerHTML = `
      <div class="row1">
        <span class="id">#${{item.id}}</span>
        <span class="coord">(${{item.coord.x}},${{item.coord.y}},${{item.coord.z}})</span>
      </div>
      <div class="title">${{item.title}}</div>
      <div class="summary">${{item.summary}}</div>
      <div class="src">
        <span class="src-tag">${{item.source}}</span>
        <span class="src-tag">层${{item.layer}}</span>
        ${{!item.link_alive ? '<span class="src-tag" style="color:var(--minus)">断</span>' : ''}}
      </div>`;
    card.onclick = () => selectItem(item.id);
    root.appendChild(card);
  }});
}}

let selectedId = null;
function selectItem(id) {{
  selectedId = id;
  document.querySelectorAll('.content-card').forEach(c => {{
    c.classList.toggle('card-active', parseInt(c.dataset.id) === id);
  }});
  renderDetail(id);
  renderPackages(id);
}}

function renderDetail(id) {{
  const item = ITEMS.find(i => i.id === id);
  if (!item) return;
  const ns = neighborIdx[id] || [];
  const root = document.getElementById('detail-view');
  root.innerHTML = `
    <div class="detail-line"><span class="k">标题</span><span>${{item.title}}</span></div>
    <div class="detail-line"><span class="k">坐标</span><span style="color:var(--accent);font-family:monospace">
      (${{item.coord.x}}, ${{item.coord.y}}, ${{item.coord.z}})</span></div>
    <div class="detail-line"><span class="k">symbol_index</span><span>${{item.symbol_index}}</span></div>
    <div class="detail-line"><span class="k">来源</span><span>${{item.source}}</span></div>
    <div class="detail-line"><span class="k">时序层</span><span>${{item.layer}}</span></div>
    <div class="detail-line"><span class="k">关键词</span>${{item.keywords.join(', ')}}</div>
    <div class="detail-line"><span class="k">URL</span><a href="${{item.url}}" target="_blank"
      style="color:var(--accent);text-decoration:none;word-break:break-all">${{item.url}}</a></div>
    <div style="margin-top:8px;font-size:12px;color:var(--dim);">${{item.summary}}</div>
    <h2 style="margin-top:20px;">直接邻居 (距离 1)</h2>
    ${{ns.length === 0 ? '<div style="color:var(--dim);font-size:12px;">无坐标相邻邻居</div>' : ''}}
    ${{ns.map(n => `<div class="neighbor-row" onclick="selectItem(${{n.b}})">
      <span>${{ITEMS.find(i=>i.id===n.b)?.title || ('#'+n.b)}}</span>
      <span class="deg">M.${{n.manhattan}}° C.${{n.chebyshev}}° →${{n.src_to_dst_deg.toFixed(1)}}° ←${{n.dst_to_src_deg.toFixed(1)}}°</span>
    </div>`).join('')}}`;
}}

function renderPackages(id) {{
  const root = document.getElementById('package-view');
  const relevant = PACKAGES.filter(p => p.member_ids.includes(id));
  if (relevant.length === 0) {{
    root.innerHTML = '<div style="color:var(--dim);font-size:12px;">无包裹</div>';
    return;
  }}
  root.innerHTML = relevant.map(p => `
    <div class="pkg-card">
      <div class="pkg-title">包裹中心: #${{p.center_id}} (${{ITEMS.find(i=>i.id===p.center_id)?.title||''}})</div>
      <div class="pkg-members">${{p.member_ids.map(m =>
        `<span class="pkg-chip" onclick="selectItem(${{m}})">#${{m}} ${{ITEMS.find(i=>i.id===m)?.title||''}}</span>`
      ).join(' ')}}</div>
    </div>`).join('');
}}

// 搜索框
document.getElementById('search').addEventListener('input', e => renderList(e.target.value));

// 初始化
renderList();
if (ITEMS.length > 0) selectItem(0);
</script>
</body>
</html>"#
    )
}

fn main() {{
    let index = build_data();

    // 输出 JSON (工作区)
    let json_out = serde_json::to_string_pretty(&index).unwrap();
    let json_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../njsg-browser/njsg_index.json");
    fs::write(json_path, &json_out).expect("write json failed");
    eprintln!("✓ JSON 索引写入 {} ({:.2} KB)", json_path, json_out.len() as f64 / 1024.0);

    // 输出 HTML (工作区)
    let html_out = render_html(&index);
    let html_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../njsg-browser/index.html");
    fs::write(html_path, &html_out).expect("write html failed");
    eprintln!("✓ 浏览器 HTML 写入 {} ({:.2} KB)", html_path, html_out.len() as f64 / 1024.0);

    // 统计锥快照
    eprintln!("统计锥: total={} alive={} dead={} balance={:.2} mood={}",
        index.snapshot.total_items, index.snapshot.alive_items, index.snapshot.dead_items,
        index.snapshot.balance, index.snapshot.mood);
    eprintln!("邻居边数: {} · 包裹数: {}", index.neighbors.len(), index.packages.len());
}}
