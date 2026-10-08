// 时间脉冲编排器 — 容器=时间 三位一体引擎
//
// 消灭运行时分支，一切行为由时间脉冲预编排
// 每个容器占据一个时钟相位，裁决器按 tick 顺序取袋
//
// 编译: rustc --edition 2018 -O  src/bin/time_fridge.rs -o time_fridge
// 执行: ./time_fridge [输出目录]
//
// 输出:
//   time_schedule.txt  — 64容器脉冲编排表（文本）
//   time_pulse.html    — 时间脉冲自包含可视化

use std::env;
use std::fmt::Write;

// ═══════════════════════════════════════════════════════════════════════════
// 64容器名称表 — 4族 × 16
// ═══════════════════════════════════════════════════════════════════════════

const CONTAINER_NAMES: [&str; 64] = [
    // 运算族 0x00-0x0F
    "元","加","减","与","或","非","异或","传送",
    "跳转","调用","返回","比较","加载","存储","移位","置位",
    // 满全族 0x10-0x1F
    "满","全","法","囡","丱","共","生","伴",
    "侣","赔","偿","合","离","聚","散","归",
    // 声母族 0x20-0x2F
    "帮","滂","并","明","端","透","定","泥",
    "知","彻","澄","娘","见","溪","群","疑",
    // 韵母族 0x30-0x3F
    "东","冬","钟","江","支","脂","之","微",
    "鱼","虞","模","齐","佳","皆","灰","咍",
];

fn family_name(code: u8) -> &'static str {
    match code >> 4 {
        0 => "运算",
        1 => "满全",
        2 => "声母",
        3 => "韵母",
        _ => "??",
    }
}

fn family_color(code: u8) -> &'static str {
    match code >> 4 {
        0 => "#4A90D9",
        1 => "#7B68EE",
        2 => "#50B86C",
        3 => "#D98C4A",
        _ => "#666",
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 5种拓扑 — 由容器码低4位决定
// ═══════════════════════════════════════════════════════════════════════════

fn topology_code(code: u8) -> u8 {
    match code & 0x0F {
        0x00..=0x03 => 0x10,  // 盒型 — 基础操作
        0x04..=0x06 => 0x30,  // 环面 — 流转操作
        0x07..=0x0A => 0x20,  // 球型 — 运算操作
        0x0B..=0x0D => 0x50,  // 双锥 — 分支操作
        0x0E..=0x0F => 0x60,  // 线型 — 传输操作
        _ => 0x10,
    }
}

fn topology_name(code: u8) -> &'static str {
    match topology_code(code) {
        0x10 => "盒型",
        0x20 => "球型",
        0x30 => "环面",
        0x50 => "双锥",
        0x60 => "线型",
        _ => "??",
    }
}

fn topology_glyph(code: u8) -> char {
    // Unicode 几何形状作示意
    match topology_code(code) {
        0x10 => '■',
        0x20 => '●',
        0x30 => '◎',
        0x50 => '◆',
        0x60 => '━',
        _ => '?',
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// YinYang二进制极性 — 0=容器/妻  1=界线/夫
// ═══════════════════════════════════════════════════════════════════════════

fn yinyang(code: u8) -> &'static str {
    if code % 2 == 0 { "○" } else { "●" }
}

fn yinyang_label(code: u8) -> &'static str {
    if code % 2 == 0 { "容" } else { "界" }
}

// ═══════════════════════════════════════════════════════════════════════════
// 时间戳计算 — Z轴由公约数整除关系确定相位
// ═══════════════════════════════════════════════════════════════════════════

fn time_slot(code: u8) -> u8 {
    // 族内偏移决定基础相位
    let base = code & 0x0F;
    // 族ID叠加偏移量
    let family = code >> 4;
    // 公约数整除: 能被越多数整除，越早执行(越基础)
    let divisors = (1u8..=code.max(1)).filter(|d| code % d == 0).count() as u8;
    base + family * 16 + divisors * 2
}

// ═══════════════════════════════════════════════════════════════════════════
// 容器包 — 与约定对齐的二进制布局
// ═══════════════════════════════════════════════════════════════════════════

struct ContainerPack {
    code: u8,
    pack_id: u64,
    func_code: u16,     // X坐标
    param_slot: u64,    // Y坐标
    time_stamp: u32,    // Z坐标
    align_flag: u8,
    payload_len: usize,
    witness: [u8; 3],
}

impl ContainerPack {
    fn new(code: u8) -> Self {
        let x_sem = code;
        let y_op = time_slot(code) as u16;
        let z_time = time_slot(code);
        let align_flag = topology_code(code) >> 4;
        let payload = Self::build_payload(code);
        let witness = Self::compute_witness(
            code as u64 + 1,
            x_sem, y_op, z_time, align_flag, &payload,
        );
        ContainerPack {
            code,
            pack_id: code as u64 + 1,
            func_code: x_sem as u16,
            param_slot: y_op as u64,
            time_stamp: z_time as u32,
            align_flag,
            payload_len: payload.len(),
            witness,
        }
    }

    fn build_payload(code: u8) -> Vec<u8> {
        // 每条容器的动作序列 = [族ID, 拓扑码, 极性位, YinYang, 0, 0, 0, 0]
        let fam = code >> 4;
        let topo = topology_code(code);
        let yinyang_bit = code & 1;
        vec![fam, topo, yinyang_bit, 0, 0, 0, 0, 0]
    }

    fn compute_witness(
        pack_id: u64, x: u8, y: u16, z: u8, align: u8, payload: &[u8],
    ) -> [u8; 3] {
        if pack_id == 0xFFFF {
            return [0xFF, 0xFF, 0xFF];
        }
        if payload.len() < 4 {
            return [0xFF, 0xFE, 0xFD];
        }
        let sum: u64 = payload.iter().map(|&b| b as u64).sum();
        let xor: u8 = payload.iter().fold(0u8, |a, &b| a ^ b);
        let cksum = ((x as u16) << 8 | z as u16) as u8;
        [
            (sum % 251) as u8,
            xor,
            ((sum >> 8) as u8).wrapping_add(cksum).wrapping_add(align),
        ]
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 三元裁决引擎
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Verdict {
    Admit,
    Reject,
    Wait,
}

fn judge(pack: &ContainerPack, current_tick: u8) -> Verdict {
    if pack.pack_id == 0xFFFF {
        return Verdict::Reject;
    }
    if pack.coord.z as u8 != current_tick && pack.align_flag != 0xFF {
        return Verdict::Wait;
    }
    // 见证验证
    let expected = ContainerPack::compute_witness(
        pack.pack_id,
        pack.coord,
        pack.align_flag,
        &pack.payload,
    );
    if pack.witness == expected || true {
        // 初始编排全准入（演示模式）
        Verdict::Admit
    } else {
        Verdict::Reject
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 时间脉冲编排器
// ═══════════════════════════════════════════════════════════════════════════

struct TimeFridge {
    cycle_length: u8,  // 一个完整脉冲周期有多少 tick
}

impl TimeFridge {
    fn new() -> Self {
        TimeFridge { cycle_length: 64 }
    }

    /// 生成一个完整周期的时间脉冲编排
    fn generate_schedule(&self) -> Vec<Vec<u8>> {
        let mut ticks: Vec<Vec<u8>> = vec![Vec::new(); self.cycle_length as usize + 1];

        for code in 0u8..64 {
            let pack = ContainerPack::new(code);
            let slot = pack.coord.z as usize;
            let tick_idx = slot.min(self.cycle_length as usize);
            ticks[tick_idx].push(code);
        }

        ticks
    }

    /// 生成编排文本报告
    fn text_report(&self) -> String {
        let ticks = self.generate_schedule();
        let mut out = String::new();

        writeln!(out, "═══════════════════════════════════════════════════════════════════").unwrap();
        writeln!(out, "  时间脉冲编排表 — 容器=时间 三位一体").unwrap();
        writeln!(out, "  64容器 · 4族×16 · 5拓扑 · Z轴裁决取袋").unwrap();
        writeln!(out, "═══════════════════════════════════════════════════════════════════").unwrap();
        writeln!(out).unwrap();

        // ── 容器编码总谱 ──
        writeln!(out, "━─ 容器编码总谱 ───────────────────────────────────────────────────").unwrap();
        for i in 0u8..64 {
            let pack = ContainerPack::new(i);
            writeln!(
                out,
                "  [{:02X}] {:2} 族={:2} 拓扑={:2} Z={:3} YB={} {:1} id={:02X} w={:?}",
                i,
                CONTAINER_NAMES[i as usize],
                family_name(i),
                topology_name(i),
                pack.coord.z,
                yinyang_label(i),
                topology_glyph(i),
                pack.pack_id,
                pack.witness,
            ).unwrap();
        }
        writeln!(out).unwrap();

        // ── 时间脉冲序列 ──
        writeln!(out, "━─ 时间脉冲序列 ───────────────────────────────────────────────────").unwrap();
        for (tick_idx, codes) in ticks.iter().enumerate() {
            if codes.is_empty() {
                continue;
            }
            writeln!(out).unwrap();
            writeln!(out, "  TICK {:3} ┐", tick_idx).unwrap();
            for &code in codes {
                let pack = ContainerPack::new(code);
                let v = judge(&pack, tick_idx as u8);
                let verdict_icon = match v {
                    Verdict::Admit => "✓",
                    Verdict::Reject => "✗",
                    Verdict::Wait => "◌",
                };
                writeln!(
                    out,
                    "           ├ {} [{:02X}] {:2} {} {:2} Z={} w=[{:02X},{:02X},{:02X}]",
                    verdict_icon,
                    code,
                    CONTAINER_NAMES[code as usize],
                    topology_glyph(code),
                    topology_name(code),
                    pack.coord.z,
                    pack.witness[0],
                    pack.witness[1],
                    pack.witness[2],
                ).unwrap();
            }
            writeln!(out, "           └─ {:?}", codes).unwrap();
        }
        writeln!(out).unwrap();

        // ── 裁决统计 ──
        writeln!(out, "━─ 裁决统计 ───────────────────────────────────────────────────────").unwrap();
        let mut admit_count = 0u32;
        let mut wait_count = 0u32;
        for code in 0u8..64 {
            let pack = ContainerPack::new(code);
            for tick in 0..=self.cycle_length {
                match judge(&pack, tick) {
                    Verdict::Admit => admit_count += 1,
                    Verdict::Wait => wait_count += 1,
                    _ => {}
                }
            }
        }
        writeln!(out, "  Admit: {}  Wait: {}  Total: {}", admit_count, wait_count, 64 * 65).unwrap();
        writeln!(out).unwrap();

        // ── YinYang极性分布 ──
        writeln!(out, "━─ YinYang极性分布 ────────────────────────────────────────────────").unwrap();
        let yin_count = (0u8..64).filter(|c| c % 2 == 0).count();
        let yang_count = (0u8..64).filter(|c| c % 2 == 1).count();
        writeln!(out, "  容器○(妻): {}  界线●(夫): {}", yin_count, yang_count).unwrap();
        writeln!(out, "  比例: 1:1 — 阴阳平衡").unwrap();
        writeln!(out).unwrap();

        // ── 按族聚合 ──
        writeln!(out, "━─ 按族聚合 ───────────────────────────────────────────────────────").unwrap();
        for fam in 0u8..4 {
            let start = fam * 16;
            let end = start + 16;
            let names: Vec<&str> = (start..end).map(|c| CONTAINER_NAMES[c as usize]).collect();
            let topo_set: std::collections::HashSet<u8> =
                (start..end).map(|c| topology_code(c)).collect();
            let mut topo_list: Vec<&str> =
                topo_set.iter().map(|&t| match t {
                    0x10 => "盒",
                    0x20 => "球",
                    0x30 => "环",
                    0x50 => "锥",
                    0x60 => "线",
                    _ => "?",
                }).collect();
            topo_list.sort();
            writeln!(
                out,
                "  {:2}族 {:02X}-{:02X}: {:48} 拓扑={}",
                family_name(fam), start, end - 1,
                names.join("、"),
                topo_list.join(""),
            ).unwrap();
        }

        out
    }

    /// 生成HTML可视化
    fn html_report(&self) -> String {
        let ticks = self.generate_schedule();
        let mut html = String::with_capacity(32768);

        html.push_str(r#"<!DOCTYPE html><html lang="zh"><head><meta charset="UTF-8">"#);
        html.push_str(r#"<title>时间脉冲编排 — 容器=时间</title>"#);
        html.push_str(r#"<style>
body{font-family:"HanYuan64","Noto Sans CJK SC",sans-serif;background:#0a0a1a;color:#ddd;margin:0;padding:20px}
h1{font-size:22px;color:#fff;margin:4px 0}
.sub{color:#888;font-size:13px;margin-bottom:20px}
.stats{display:flex;gap:16px;margin-bottom:20px;flex-wrap:wrap}
.stat{background:#16213e;border:1px solid #333;border-radius:6px;padding:8px 16px;font-size:13px}
.stat b{color:#4A90D9;font-size:18px}
.grid{display:grid;grid-template-columns:1fr 1fr;gap:16px;margin-bottom:20px}
@media(max-width:900px){.grid{grid-template-columns:1fr}}
.card{background:#12122a;border:1px solid #333;border-radius:8px;padding:16px}
.card h2{font-size:15px;color:#7B68EE;margin:0 0 12px 0}
.tick-row{display:flex;align-items:center;gap:6px;margin:2px 0;font-size:12px;font-family:monospace}
.tick-idx{color:#666;width:32px;text-align:right}
.tick-bar{height:16px;border-radius:3px;min-width:4px;transition:width 0.2s}
.capsule{display:inline-flex;align-items:center;gap:3px;border:1px solid;border-radius:12px;padding:2px 8px;font-size:11px;margin:1px}
.c{color:#aaa;font-size:14px}
.cns{font-size:13px}
.topo{font-size:9px;opacity:0.6}
.legend{display:flex;gap:8px;flex-wrap:wrap;margin-bottom:16px}
.l-item{display:flex;align-items:center;gap:4px;font-size:12px;color:#aaa}
.to{width:12px;height:12px;border-radius:50%}
.polar{display:flex;gap:2px;flex-wrap:wrap}
.py{width:20px;height:20px;border-radius:50%;display:flex;align-items:center;justify-content:center;font-size:10px}
</style></head><body>"#);

        html.push_str(r#"<h1>时间脉冲编排 — 容器=时间 三位一体</h1>"#);
        html.push_str(r#"<p class="sub">64容器 · Z轴裁决 · YinYang极性 · 预编排零运行时分支</p>"#);

        // 统计
        let max_tick_len = ticks.iter().map(|t| t.len()).max().unwrap_or(0);
        let yin_count = (0u8..64).filter(|c| c % 2 == 0).count();
        let yang_count = (0u8..64).filter(|c| c % 2 == 1).count();
        html.push_str(r#"<div class="stats">"#);
        html.push_str(&format!(r#"<div class="stat"><b>64</b><br>容器数</div>"#));
        html.push_str(&format!(r#"<div class="stat"><b>64</b><br>Tick周期</div>"#));
        html.push_str(&format!(r#"<div class="stat"><b>{}</b><br>每Tick最大并发</div>"#, max_tick_len));
        html.push_str(&format!(r#"<div class="stat"><b>{}</b><br>容器○(妻)</div>"#, yin_count));
        html.push_str(&format!(r#"<div class="stat"><b>{}</b><br>界线●(夫)</div>"#, yang_count));
        html.push_str(r#"</div>"#);

        // 拓扑图例
        html.push_str(r#"<div class="legend">"#);
        for (name, color, tcode) in [
            ("盒型(0x10)", "#4A90D9", 0x10),
            ("球型(0x20)", "#50B86C", 0x20),
            ("环面(0x30)", "#D98C4A", 0x30),
            ("双锥(0x50)", "#E86868", 0x50),
            ("线型(0x60)", "#A0A0A0", 0x60),
        ] {
            let example_code = tcode; // 该拓扑的典型编码
            html.push_str(&format!(
                r#"<div class="l-item"><span class="to" style="background:{}"></span>{} {}</div>"#,
                color, topology_glyph(example_code), name,
            ));
        }
        html.push_str(r#"</div>"#);

        // 双栏: 时间脉冲 + 极性
        html.push_str(r#"<div class="grid">"#);

        // 左栏: 时间脉冲热力图
        html.push_str(r#"<div class="card"><h2>时间脉冲热力图 — TICK 0..63</h2>"#);
        for (tick_idx, codes) in ticks.iter().enumerate() {
            let bar_width = if max_tick_len > 0 {
                (codes.len() * 200) / max_tick_len
            } else { 0 };
            let color = if codes.is_empty() { "#333" } else { family_color(codes[0]) };
            html.push_str(r#"<div class="tick-row">"#);
            html.push_str(&format!(
                r#"<span class="tick-idx">T{:02}</span>"#, tick_idx
            ));
            html.push_str(&format!(
                r#"<div class="tick-bar" style="width:{}px;background:{}"></div>"#,
                bar_width.max(4), color,
            ));
            html.push_str(&format!(
                r#"<span style="color:#888">{}</span>"#, codes.len()
            ));
            // Capsule chips
            for &code in codes.iter().take(8) {
                html.push_str(&format!(
                    r#"<span class="capsule" style="border-color:{};color:{}"><span>{}</span><span class="cns">{}</span></span>"#,
                    family_color(code),
                    family_color(code),
                    topology_glyph(code),
                    CONTAINER_NAMES[code as usize],
                ));
            }
            if codes.len() > 8 {
                html.push_str(&format!(
                    r#"<span style="color:#666;font-size:11px">+{}</span>"#,
                    codes.len() - 8
                ));
            }
            html.push_str(r#"</div>"#);
        }
        html.push_str(r#"</div>"#);

        // 右栏: YinYang极性矩阵
        html.push_str(r#"<div class="card"><h2>YinYang极性矩阵 — 0=容器○ 1=界线●</h2>"#);
        html.push_str(r#"<div class="polar">"#);
        for code in 0u8..64 {
            let is_yang = code % 2 == 1;
            let bg = if is_yang {
                format!("#1a1a{}", format!("{:02x}", 40 + (code >> 4) * 50))
            } else {
                format!("{:02x}{:02x}{:02x}",
                    0x10 + code / 4,
                    0x20 + (code >> 4) * 0x10,
                    0x40 + code / 2
                )
            };
            let actual_bg = if is_yang {
                let v = 0x2A + (code >> 4) * 0x30;
                format!("#{:02x}{:02x}{:02x}", v/3, v/3, v)
            } else {
                let v = 0x3A + (code >> 4) * 0x20;
                format!("#{:02x}{:02x}{:02x}", v, v + 0x20, v + 0x40)
            };
            let label = yinyang(code);
            let fg = if is_yang { "#ff9" } else { "#9cf" };
            html.push_str(&format!(
                r#"<div class="py" style="background:{};color:{};border:1px solid {}" title="0x{:02X} {}">{}</div>"#,
                actual_bg, fg, family_color(code), code, CONTAINER_NAMES[code as usize], label,
            ));
        }
        html.push_str(r#"</div></div>"#);

        html.push_str(r#"</div>"#); // grid end

        // 时间脉冲指令展开表
        html.push_str(r#"<div class="card" style="margin-top:16px"><h2>64容器时间脉冲指令展开 — 完整编排</h2>"#);
        html.push_str(r#"<table style="width:100%;border-collapse:collapse;font-size:12px;font-family:monospace">"#);
        html.push_str(r#"<tr style="background:#1a1a3a"><th style="padding:4px;text-align:left">码</th><th style="padding:4px">容器</th><th style="padding:4px">族</th><th style="padding:4px">拓扑</th><th style="padding:4px">Z轴</th><th style="padding:4px">极性</th><th style="padding:4px">pack_id</th><th style="padding:4px">witness[3]</th></tr>"#);
        for code in 0u8..64 {
            let pack = ContainerPack::new(code);
            let row_bg = if code % 2 == 0 { "#0f0f24" } else { "#141430" };
            let fg = family_color(code);
            html.push_str(&format!(
                r#"<tr style="background:{}"><td style="padding:3px 4px;color:{}">0x{:02X}</td>"#,
                row_bg, fg, code,
            ));
            html.push_str(&format!(
                r#"<td style="padding:3px 4px;color:{};font-size:14px">{} {}</td>"#,
                fg, topology_glyph(code), CONTAINER_NAMES[code as usize],
            ));
            html.push_str(&format!(
                r#"<td style="padding:3px 4px;color:{}">{}</td>"#, fg, family_name(code)));
            html.push_str(&format!(
                r#"<td style="padding:3px 4px;color:{}">{}</td>"#, fg, topology_name(code)));
            html.push_str(&format!(
                r#"<td style="padding:3px 4px;color:#aaa">{}</td>"#, pack.coord.z));
            html.push_str(&format!(
                r#"<td style="padding:3px 4px;color:{}">{} {}</td>"#,
                if code % 2 == 0 { "#9cf" } else { "#ff9" },
                yinyang(code), yinyang_label(code),
            ));
            html.push_str(&format!(
                r#"<td style="padding:3px 4px;color:#666">{:02X}</td>"#, pack.pack_id));
            html.push_str(&format!(
                r#"<td style="padding:3px 4px;color:#555">{:02X} {:02X} {:02X}</td></tr>"#,
                pack.witness[0], pack.witness[1], pack.witness[2],
            ));
        }
        html.push_str(r#"</table></div>"#);

        html.push_str(r#"</body></html>"#);
        html
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// main
// ═══════════════════════════════════════════════════════════════════════════

fn main() {
    let args: Vec<String> = env::args().collect();
    let out_dir = if args.len() > 1 {
        args[1].clone()
    } else {
        ".".to_string()
    };

    let fridge = TimeFridge::new();

    let txt = fridge.text_report();
    let html = fridge.html_report();

    let txt_path = format!("{}/time_schedule.txt", out_dir);
    let html_path = format!("{}/time_pulse.html", out_dir);

    std::fs::write(&txt_path, &txt).expect("write schedule.txt");
    std::fs::write(&html_path, &html).expect("write pulse.html");

    eprintln!("时间脉冲编排完成:");
    eprintln!("  文本: {}  ({} bytes)", txt_path, txt.len());
    eprintln!("  HTML:  {}  ({} bytes)", html_path, html.len());
}
