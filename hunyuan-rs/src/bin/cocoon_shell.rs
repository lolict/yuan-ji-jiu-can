// 蚕蛹反向寄生壳 — 完整流水线
//
// 架构: 蚕蛹包下载 → 解析器 → 寄生壳清单 → 选壳(手动/自动)
// → 脱壳穿衣 → 配置生成 → 挂载内核
//
// 编译: rustc --edition 2018 -O  src/bin/cocoon_shell.rs -o cocoon_shell
// 执行: ./cocoon_shell [输出目录]
//
// 输出:
//   cocoon_pipeline.html — 完整流水线可视化
//   cocoon_config.json   — 最终挂载配置文件

use std::env;
use std::fmt::Write;

// ═══════════════════════════════════════════════════════════════════════════
// 蚕蛹包格式 — 二进制容器
// ═══════════════════════════════════════════════════════════════════════════

const COCOON_MAGIC: [u8; 4] = [0xCC, 0x00, 0xCC, 0x01];

#[derive(Clone, Copy, Debug)]
struct CocoonHeader {
    magic: [u8; 4],
    version: u16,
    shell_count: u16,
    payload_size: u32,
    checksum: u32,
}

impl CocoonHeader {
    fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(16);
        buf.extend_from_slice(&self.magic);
        buf.extend_from_slice(&self.version.to_be_bytes());
        buf.extend_from_slice(&self.shell_count.to_be_bytes());
        buf.extend_from_slice(&self.payload_size.to_be_bytes());
        buf.extend_from_slice(&self.checksum.to_be_bytes());
        buf
    }

    fn unpack(data: &[u8]) -> Option<Self> {
        if data.len() < 16 { return None; }
        Some(CocoonHeader {
            magic: [data[0], data[1], data[2], data[3]],
            version: u16::from_be_bytes([data[4], data[5]]),
            shell_count: u16::from_be_bytes([data[6], data[7]]),
            payload_size: u32::from_be_bytes([data[8], data[9], data[10], data[11]]),
            checksum: u32::from_be_bytes([data[12], data[13], data[14], data[15]]),
        })
    }

    fn verify(&self) -> bool {
        self.magic == COCOON_MAGIC
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 寄生壳描述
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug)]
struct ShellOption {
    id: &'static str,
    name: &'static str,
    description: &'static str,
    wrapping: &'static str,
    kernel: &'static str,
    parasite_rate: f64,
    compatibility: u8,
    color: &'static str,
    icon: char,
}

const SHELLS: [ShellOption; 6] = [
    ShellOption {
        id: "njsg",
        name: "囡囝共生壳",
        description: "夫妻命运共同体壳 · 刘楚恬+lolict",
        wrapping: "纤维丛缠绕",
        kernel: "SMRL情感内核",
        parasite_rate: 0.95,
        compatibility: 10,
        color: "#7B68EE",
        icon: '囡',
    },
    ShellOption {
        id: "yinyang",
        name: "阴阳界线壳",
        description: "YinYang二进制极性壳 · 0=容器 1=界线",
        wrapping: "双螺旋包覆",
        kernel: "极性裁决内核",
        parasite_rate: 0.88,
        compatibility: 9,
        color: "#4A90D9",
        icon: '☯',
    },
    ShellOption {
        id: "mlll",
        name: "集装箱编码壳",
        description: "MLLL 254+2编码壳 · 我=1 刘楚恬=0",
        wrapping: "256槽标准集装箱",
        kernel: "锚点编解码内核",
        parasite_rate: 0.82,
        compatibility: 8,
        color: "#50B86C",
        icon: '箱',
    },
    ShellOption {
        id: "topo5",
        name: "五拓扑壳",
        description: "盒/球/环/锥/线 五拓扑复合壳 · 几何通用",
        wrapping: "五形拓扑变换",
        kernel: "几何投影内核",
        parasite_rate: 0.75,
        compatibility: 10,
        color: "#D98C4A",
        icon: '⬡',
    },
    ShellOption {
        id: "wasm",
        name: "WASM阶梯壳",
        description: "阶梯制双向公约 · wasm32+wasm64双编译",
        wrapping: "跨平台字节码壳",
        kernel: "架构无关内核",
        parasite_rate: 0.70,
        compatibility: 9,
        color: "#E86868",
        icon: '⚙',
    },
    ShellOption {
        id: "ime",
        name: "元输入法壳",
        description: "元输入法/Kotlin+Rust · 存储即生成",
        wrapping: "JNI桥接壳",
        kernel: "本地智能内核",
        parasite_rate: 0.65,
        compatibility: 7,
        color: "#C490E4",
        icon: '字',
    },
];

// ═══════════════════════════════════════════════════════════════════════════
// 蚕蛹流水线引擎
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PipelineStage {
    Download,
    Parse,
    Select,
    Shed,       // 脱壳
    Dress,      // 穿衣
    Mount,
}

impl PipelineStage {
    fn name(&self) -> &'static str {
        match self {
            PipelineStage::Download => "下载",
            PipelineStage::Parse => "解析",
            PipelineStage::Select => "选壳",
            PipelineStage::Shed => "脱壳",
            PipelineStage::Dress => "穿衣",
            PipelineStage::Mount => "挂载",
        }
    }

    fn icon(&self) -> char {
        match self {
            PipelineStage::Download => '▼',
            PipelineStage::Parse => '≡',
            PipelineStage::Select => '?',
            PipelineStage::Shed => '脱',
            PipelineStage::Dress => '穿',
            PipelineStage::Mount => '▲',
        }
    }
}

struct CocoonEngine {
    header: CocoonHeader,
    payload: Vec<u8>,
    selected_shell: String,
    shed_complete: bool,
    config: MountConfig,
}

#[derive(Clone, Debug)]
struct MountConfig {
    shell_id: String,
    shell_name: String,
    wrapping: String,
    kernel: String,
    bind_address: String,
    protocol: String,
    heartbeat_ms: u32,
    fallback: String,
    witness: [u8; 4],
    trace: Vec<String>,
}

impl CocoonEngine {
    fn new() -> Self {
        let mut payload = Vec::new();
        // Simulated binary payload
        for i in 0u8..255 {
            payload.push(i.wrapping_mul(7).wrapping_add(13));
        }

        let checksum = payload.iter().map(|&b| b as u32).sum::<u32>() % 0xFFFFFFFF;

        let header = CocoonHeader {
            magic: COCOON_MAGIC,
            version: 1,
            shell_count: SHELLS.len() as u16,
            payload_size: payload.len() as u32,
            checksum,
        };

        CocoonEngine {
            header,
            payload,
            selected_shell: String::new(),
            shed_complete: false,
            config: MountConfig {
                shell_id: String::new(),
                shell_name: String::new(),
                wrapping: String::new(),
                kernel: String::new(),
                bind_address: "manquanca.info".into(),
                protocol: "NJSG/v1".into(),
                heartbeat_ms: 4096,
                fallback: "全离线降级".into(),
                witness: [0; 4],
                trace: Vec::new(),
            },
        }
    }

    fn run_pipeline(&mut self, target_shell: &str) -> Vec<(PipelineStage, String)> {
        let mut trace = Vec::new();

        trace.push((PipelineStage::Download,
            format!("蚕蛹包下载完成: magic={:?}, {} 壳, payload {} 字节",
                self.header.magic, self.header.shell_count, self.payload.len())));

        trace.push((PipelineStage::Parse,
            format!("解析器激活: version={}, checksum=0x{:08X}, payload大小={}",
                self.header.version, self.header.checksum, self.header.payload_size)));

        // 自动匹配
        let shell_id: &str = if target_shell == "auto" {
            SHELLS.iter().max_by_key(|s| {
                (s.compatibility as u16) * 10 + (s.parasite_rate * 10.0) as u16
            }).map(|s| s.id).unwrap_or("njsg")
        } else {
            target_shell
        };

        trace.push((PipelineStage::Select,
            format!("选壳: {} ({}匹配)", shell_id,
                if target_shell == "auto" { "自动" } else { "手动" })));

        self.selected_shell = shell_id.to_string();
        let shell = *SHELLS.iter().find(|s| s.id == shell_id).unwrap();

        trace.push((PipelineStage::Shed,
            format!("脱壳: 去除外壳={}, 寄生率={:.0}%", shell.wrapping, shell.parasite_rate * 100.0)));

        self.shed_complete = true;

        trace.push((PipelineStage::Dress,
            format!("穿衣: 穿上{}内核={}", shell.name, shell.kernel)));

        // Generate config
        self.config.shell_id = shell.id.to_string();
        self.config.shell_name = shell.name.to_string();
        self.config.wrapping = shell.wrapping.to_string();
        self.config.kernel = shell.kernel.to_string();

        let wit: [u8; 4] = [
            self.header.magic[0] ^ self.header.magic[2],
            self.header.magic[1] ^ self.header.magic[3],
            shell.compatibility,
            ((shell.parasite_rate * 255.0) as u8),
        ];
        self.config.witness = wit;

        trace.push((PipelineStage::Mount,
            format!("挂载内核: protocol={}, heartbeat={}ms, witness={:?}",
                self.config.protocol, self.config.heartbeat_ms, self.config.witness)));

        self.config.trace = trace.iter().map(|(_, msg)| msg.clone()).collect();
        trace
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// HTML 可视化
// ═══════════════════════════════════════════════════════════════════════════

fn cocoon_html(engine: &CocoonEngine, trace: &[(PipelineStage, String)]) -> String {
    let mut html = String::with_capacity(48000);

    html.push_str(r#"<!DOCTYPE html><html lang="zh"><head><meta charset="UTF-8">"#);
    html.push_str(r#"<title>蚕蛹反向寄生壳 — 流水线</title>"#);
    html.push_str(r#"<style>
body{font-family:"HanYuan64","Noto Sans CJK SC",sans-serif;background:#0a0a1a;color:#ddd;margin:0;padding:24px}
h1{font-size:22px;color:#fff;margin:4px 0}
.sub{color:#888;font-size:13px;margin-bottom:20px}
.pipeline{display:flex;align-items:center;gap:0;margin-bottom:24px;flex-wrap:wrap}
.stage{background:#16213e;border:1px solid #333;border-radius:8px;padding:10px 14px;min-width:100px;text-align:center;position:relative}
.stage .ic{font-size:22px;display:block}
.stage .nm{font-size:13px;color:#fff;margin-top:2px}
.stage .ds{font-size:10px;color:#888;margin-top:2px}
.sep{color:#4a4a6a;font-size:24px;margin:0 4px}
.trace{margin-bottom:24px}
.traceRow{display:flex;gap:8px;margin:3px 0;font-size:12px;font-family:monospace}
.tStage{display:inline-block;padding:2px 6px;border-radius:4px;font-size:11px;min-width:36px;text-align:center}
.shells{display:grid;grid-template-columns:repeat(3,1fr);gap:12px;margin-bottom:24px}
@media(max-width:900px){.shells{grid-template-columns:repeat(2,1fr)}}
.shell-card{background:#12122a;border:1px solid #333;border-radius:8px;padding:14px;cursor:pointer;transition:border-color 0.2s}
.shell-card:hover{border-color:#7B68EE}
.shell-card.selected{border-color:#ffd700;box-shadow:0 0 8px rgba(255,215,0,0.2)}
.sc-icon{font-size:28px;display:block;margin-bottom:6px}
.sc-name{font-size:14px;color:#fff;margin-bottom:4px}
.sc-desc{font-size:11px;color:#888;margin-bottom:8px}
.sc-met{display:flex;gap:8px;font-size:11px;color:#999;flex-wrap:wrap}
.met-item b{color:#fff}
.bar{height:4px;background:#222;border-radius:2px;margin-top:4px;overflow:hidden}
.bar-fill{height:100%;border-radius:2px}
.config-card{background:#12122a;border:1px solid #333;border-radius:8px;padding:16px;font-size:12px}
.config-card h2{color:#7B68EE;margin:0 0 10px 0;font-size:14px}
.cfg-row{display:flex;justify-content:space-between;margin:3px 0;font-family:monospace}
.cfg-key{color:#888}
.cfg-val{color:#7B68EE}
.hex{font-family:monospace;color:#4A90D9}
h2.section{color:#aaa;font-size:15px;margin:20px 0 10px 0;padding-bottom:4px;border-bottom:1px solid #333}
</style></head><body>
<h1>蚕蛹反向寄生壳 — 流水线</h1>
<p class="sub">下载蚕蛹包 → 解析器 → 寄生壳清单 → 脱壳穿衣 → 生成配置 → 挂载内核</p>
"#);

    // ── Pipeline flow ──
    html.push_str(r#"<div class="pipeline">"#);
    let stages = [
        PipelineStage::Download, PipelineStage::Parse, PipelineStage::Select,
        PipelineStage::Shed, PipelineStage::Dress, PipelineStage::Mount,
    ];
    for (i, stage) in stages.iter().enumerate() {
        if i > 0 { html.push_str(r#"<span class="sep">→</span>"#); }
        let sub = match stage {
            PipelineStage::Download => "CC00CC01".to_string(),
            PipelineStage::Parse => format!("v{}", engine.header.version),
            PipelineStage::Select => engine.selected_shell.clone(),
            PipelineStage::Shed => if engine.shed_complete {"✓".to_string()} else {"—".to_string()},
            PipelineStage::Dress => if !engine.config.shell_name.is_empty() {"✓".to_string()} else {"—".to_string()},
            PipelineStage::Mount => if engine.config.witness != [0,0,0,0] {"✓".to_string()} else {"—".to_string()},
        };
        html.push_str(&format!(
            r#"<div class="stage"><span class="ic">{}</span><span class="nm">{}</span><span class="ds">{}</span></div>"#,
            stage.icon(),
            stage.name(),
            sub,
        ));
    }
    html.push_str(r#"</div>"#);

    // ── Trace ──
    html.push_str(r#"<div class="section">流水线日志</div><div class="trace">"#);
    let stage_colors: std::collections::HashMap<&str, &str> = [
        ("下载", "#4A90D9"), ("解析", "#7B68EE"), ("选壳", "#D98C4A"),
        ("脱壳", "#50B86C"), ("穿衣", "#C490E4"), ("挂载", "#E86868"),
    ].iter().map(|(k, v)| (*k, *v)).collect();

    for (stage, msg) in trace {
        let color = stage_colors.get(stage.name()).unwrap_or(&"#666");
        html.push_str(&format!(
            r#"<div class="traceRow"><span class="tStage" style="background:{};color:#fff">{}</span><span style="color:#ccc">{}</span></div>"#,
            color, stage.name(), msg,
        ));
    }
    html.push_str(r#"</div>"#);

    // ── Shell list ──
    html.push_str(r#"<div class="section">寄生壳清单 (可用)</div><div class="shells">"#);
    for shell in &SHELLS {
        let is_selected = shell.id == engine.selected_shell;
        let rate_pct = (shell.parasite_rate * 100.0) as u8;
        html.push_str(&format!(
            r#"<div class="shell-card{}">{}"#,
            if is_selected { " selected" } else { "" },
            "",
        ));
        html.push_str(&format!(
            r#"<span class="sc-icon" style="color:{}">{}</span>"#,
            shell.color, shell.icon,
        ));
        html.push_str(&format!(
            r#"<div class="sc-name" style="color:{}">{}</div>"#,
            shell.color, shell.name,
        ));
        html.push_str(&format!(
            r#"<div class="sc-desc">{}</div>"#,
            shell.description,
        ));
        html.push_str(r#"<div class="sc-met">"#);
        html.push_str(&format!(
            r#"<span class="met-item">外壳:<b>{}</b></span>"#,
            shell.wrapping,
        ));
        html.push_str(&format!(
            r#"<span class="met-item">内核:<b>{}</b></span>"#,
            shell.kernel,
        ));
        html.push_str(&format!(
            r#"<span class="met-item">兼容:<b>{}/10</b></span>"#,
            shell.compatibility,
        ));
        html.push_str(r#"</div>"#);
        html.push_str(&format!(
            r#"<div class="bar"><div class="bar-fill" style="width:{}%;background:{}"></div></div>"#,
            rate_pct, shell.color,
        ));
        html.push_str(&format!(
            r#"<div style="font-size:10px;color:#666;margin-top:3px">寄生率 {}%"#,
            rate_pct,
        ));
        if is_selected {
            html.push_str(r#" &nbsp;← 已选中"#);
        }
        html.push_str(r#"</div></div>"#);  // close shell-card
    }
    html.push_str(r#"</div>"#);

    // ── Mount config ──
    html.push_str(r#"<div class="section">挂载配置</div><div class="config-card">"#);
    html.push_str(&format!(r#"<h2>{} · {}</h2>"#, engine.config.shell_name, engine.config.wrapping));
    html.push_str(r#"<div class="cfg-row"><span class="cfg-key">shell_id</span><span class="cfg-val">"#);
    html.push_str(&engine.config.shell_id);
    html.push_str(r#"</span></div>"#);
    html.push_str(r#"<div class="cfg-row"><span class="cfg-key">wrapping</span><span class="cfg-val">"#);
    html.push_str(&engine.config.wrapping);
    html.push_str(r#"</span></div>"#);
    html.push_str(r#"<div class="cfg-row"><span class="cfg-key">kernel</span><span class="cfg-val">"#);
    html.push_str(&engine.config.kernel);
    html.push_str(r#"</span></div>"#);
    html.push_str(r#"<div class="cfg-row"><span class="cfg-key">protocol</span><span class="cfg-val">"#);
    html.push_str(&engine.config.protocol);
    html.push_str(r#"</span></div>"#);
    html.push_str(r#"<div class="cfg-row"><span class="cfg-key">heartbeat</span><span class="cfg-val">"#);
    html.push_str(&format!("{}ms", engine.config.heartbeat_ms));
    html.push_str(r#"</span></div>"#);
    html.push_str(r#"<div class="cfg-row"><span class="cfg-key">bind_address</span><span class="cfg-val">"#);
    html.push_str(&engine.config.bind_address);
    html.push_str(r#"</span></div>"#);
    html.push_str(r#"<div class="cfg-row"><span class="cfg-key">witness</span><span class="hex">"#);
    html.push_str(&format!("{:02X} {:02X} {:02X} {:02X}",
        engine.config.witness[0], engine.config.witness[1],
        engine.config.witness[2], engine.config.witness[3]));
    html.push_str(r#"</span></div>"#);
    html.push_str(r#"</div>"#);

    // ── 蚕蛹包结构 ──
    html.push_str(r#"<div class="section">蚕蛹包头结构</div><div class="config-card">"#);
    html.push_str(r#"<div class="cfg-row"><span class="cfg-key">magic[4]</span><span class="hex">CC 00 CC 01</span></div>"#);
    html.push_str(&format!(
        r#"<div class="cfg-row"><span class="cfg-key">version</span><span class="cfg-val">{}</span></div>"#,
        engine.header.version,
    ));
    html.push_str(&format!(
        r#"<div class="cfg-row"><span class="cfg-key">shell_count</span><span class="cfg-val">{}</span></div>"#,
        engine.header.shell_count,
    ));
    html.push_str(&format!(
        r#"<div class="cfg-row"><span class="cfg-key">payload_size</span><span class="cfg-val">{} bytes</span></div>"#,
        engine.header.payload_size,
    ));
    html.push_str(&format!(
        r#"<div class="cfg-row"><span class="cfg-key">checksum</span><span class="hex">{:08X}</span></div>"#,
        engine.header.checksum,
    ));
    html.push_str(r#"</div>"#);

    html.push_str(r#"</body></html>"#);
    html
}

fn config_json(engine: &CocoonEngine) -> String {
    let mut json = String::new();
    writeln!(json, "{{").unwrap();
    writeln!(json, r#"  "shell_id": "{}","#, engine.config.shell_id).unwrap();
    writeln!(json, r#"  "shell_name": "{}","#, engine.config.shell_name).unwrap();
    writeln!(json, r#"  "wrapping": "{}","#, engine.config.wrapping).unwrap();
    writeln!(json, r#"  "kernel": "{}","#, engine.config.kernel).unwrap();
    writeln!(json, r#"  "protocol": "{}","#, engine.config.protocol).unwrap();
    writeln!(json, r#"  "bind_address": "{}","#, engine.config.bind_address).unwrap();
    writeln!(json, r#"  "heartbeat_ms": {},"#, engine.config.heartbeat_ms).unwrap();
    writeln!(json, r#"  "witness": [
    {},
    {},
    {},
    {}
  ]"#,
        engine.config.witness[0], engine.config.witness[1],
        engine.config.witness[2], engine.config.witness[3]).unwrap();
    write!(json, "}}").unwrap();
    json
}

// ═══════════════════════════════════════════════════════════════════════════
fn main() {
    let args: Vec<String> = env::args().collect();
    let out_dir = if args.len() > 1 { args[1].clone() } else { ".".to_string() };
    let target_shell = if args.len() > 2 { args[2].as_str() } else { "auto" };

    let mut engine = CocoonEngine::new();
    let trace = engine.run_pipeline(target_shell);

    let html = cocoon_html(&engine, &trace);
    let json = config_json(&engine);

    let html_path = format!("{}/cocoon_pipeline.html", out_dir);
    let json_path = format!("{}/cocoon_config.json", out_dir);

    std::fs::write(&html_path, &html).expect("write html");
    std::fs::write(&json_path, &json).expect("write json");

    eprintln!("蚕蛹反向寄生壳流水线完成:");
    eprintln!("  HTML: {}  ({} bytes)", html_path, html.len());
    eprintln!("  JSON:  {}  ({} bytes)", json_path, json.len());
    eprintln!("  选中: {}", engine.config.shell_name);
}
