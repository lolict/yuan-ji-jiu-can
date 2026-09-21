// WASM 阶梯制·双向公约 — 双路编译引擎
//
// 哲学: 同一份源码, 两个目标, 架构无关
//  阶梯 = 源码 → 宏层 → 类型层 → 编译器(wasm32/wasm64) → 输出
//  双向 = 同一语义, 不同位宽, 固定宽度保证等价
//
// 编译: rustc --edition 2018 -O src/bin/wasm_ladder.rs -o wasm_ladder
// 执行: ./wasm_ladder [输出目录]
//
// 输出:
//   wasm_ladder.html — 阶梯制双路编译可视化
//   dual_wasm.json — 双wasm模拟描述

use std::env;
use std::fmt::Write;

// ═══════════════════════════════════════════════════════════════════════════
// 固定宽度类型系统 — C99 stdint + 容器定制
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug)]
struct FixedWidthType {
    name: &'static str,       // 容器名
    c_name: &'static str,     // C typedef
    bits: u8,                 // 位宽
    signed: bool,             // 有符号?
    wasm32_val: &'static str, // wasm32中的表示
    wasm64_val: &'static str, // wasm64中的表示
    container_role: &'static str, // 容器系统角色
}

const FIXED_TYPES: [FixedWidthType; 12] = [
    FixedWidthType { name: "i8",  c_name: "int8_t",   bits: 8,  signed: true,  wasm32_val: "i32(load8_s)", wasm64_val: "i64(load8_s)",  container_role: "状态标志" },
    FixedWidthType { name: "u8",  c_name: "uint8_t",  bits: 8,  signed: false, wasm32_val: "i32(load8_u)", wasm64_val: "i64(load8_u)",  container_role: "YinYang" },
    FixedWidthType { name: "i16", c_name: "int16_t",  bits: 16, signed: true,  wasm32_val: "i32(load16_s)",wasm64_val: "i64(load16_s)", container_role: "声母索引" },
    FixedWidthType { name: "u16", c_name: "uint16_t", bits: 16, signed: false, wasm32_val: "i32(load16_u)",wasm64_val: "i64(load16_u)", container_role: "容器码" },
    FixedWidthType { name: "i32", c_name: "int32_t",  bits: 32, signed: true,  wasm32_val: "i32",         wasm64_val: "i64(ext_s/i32)", container_role: "拓扑码" },
    FixedWidthType { name: "u32", c_name: "uint32_t", bits: 32, signed: false, wasm32_val: "i32",         wasm64_val: "i64(ext_u/i32)", container_role: "TimeSlot" },
    FixedWidthType { name: "i64", c_name: "int64_t",  bits: 64, signed: true,  wasm32_val: "i64(2×i32)",  wasm64_val: "i64",           container_role: "pack_id" },
    FixedWidthType { name: "u64", c_name: "uint64_t", bits: 64, signed: false, wasm32_val: "i64(2×i32)",  wasm64_val: "i64",           container_role: "param_slot" },
    FixedWidthType { name: "f32", c_name: "float",    bits: 32, signed: true,  wasm32_val: "f32",         wasm64_val: "f32",           container_role: "能量值" },
    FixedWidthType { name: "f64", c_name: "double",   bits: 64, signed: true,  wasm32_val: "f64",         wasm64_val: "f64",           container_role: "记忆精度" },
    FixedWidthType { name: "ptr32",c_name: "uint32_t(bits:ptr)",bits:32,signed:false,wasm32_val:"i32",    wasm64_val: "i64(trunc)",   container_role: "wasm32地址" },
    FixedWidthType { name: "ptr64",c_name: "uint64_t(bits:ptr)",bits:64,signed:false,wasm32_val:"N/A",    wasm64_val: "i64",           container_role: "wasm64地址" },
];

// ═══════════════════════════════════════════════════════════════════════════
// 统一宏前缀系统
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Clone)]
struct MacroRule {
    macro_name: &'static str,
    wasm32_def: &'static str,
    wasm64_def: &'static str,
    purpose: &'static str,
}

fn macro_rules() -> Vec<MacroRule> {
    vec![
        MacroRule { macro_name: "MQF_TYPE",   wasm32_def: "#define intptr_t i32",  wasm64_def: "#define intptr_t i64",  purpose: "架构指针宽度" },
        MacroRule { macro_name: "MQF_SIZE",   wasm32_def: "#define SIZE 4",         wasm64_def: "#define SIZE 8",         purpose: "字节宽度常量" },
        MacroRule { macro_name: "MQF_LOAD",   wasm32_def: "#define LOAD(p) i32.load",wasm64_def: "#define LOAD(p) i64.load", purpose: "内存加载指令" },
        MacroRule { macro_name: "MQF_STORE",  wasm32_def: "#define STORE(p,v) i32.store",wasm64_def:"#define STORE(p,v) i64.store", purpose: "内存存储指令" },
        MacroRule { macro_name: "MQF_ALIGN",  wasm32_def: "#define ALIGN 2",        wasm64_def: "#define ALIGN 3",        purpose: "对齐指数(2^2=4, 2^3=8)" },
        MacroRule { macro_name: "MQF_CAST",   wasm32_def: "#define CAST(v) v",      wasm64_def: "#define CAST(v) i64.extend_i32_u(v)", purpose: "位宽转换" },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════
// 宏观容器源码 — 一份, 两个目标
// ═══════════════════════════════════════════════════════════════════════════

fn shared_source_lines() -> Vec<&'static str> {
    vec![
        "/* 容器运算 — 架构无关源码 */",
        "#include <stdint.h>",
        "#include <mqf_prefix.h>",
        "",
        "typedef uint32_t ContainerCode;",
        "typedef uint64_t PackId;",
        "typedef uint16_t Topology;",
        "",
        "struct Container {",
        "    MQF_TYPE code;",
        "    MQF_TYPE topology;",
        "    MQF_TYPE time_slot;",
        "    uint8_t yinyang_bit;",
        "    uint8_t align_flag;",
        "};",
        "",
        "int64_t route(Container* c, int64_t memory) {",
        "    MQF_TYPE x = MQF_LOAD(&c->code);",
        "    MQF_TYPE y = MQF_LOAD(&c->topology);",
        "",
        "    /* 公约数裁判 */",
        "    for (MQF_TYPE d = 2; d <= x; d++) {",
        "        if (x % d == 0 && y % d == 0)",
        "            return MQF_CAST(d);",
        "    }",
        "    return memory;  /* 记忆跟随结束而继续 */",
        "}",
    ]
}

// ═══════════════════════════════════════════════════════════════════════════
// HTML
// ═══════════════════════════════════════════════════════════════════════════

fn ladder_html() -> String {
    let mut html = String::with_capacity(40960);

    // ── header ──
    s(&mut html, "<!DOCTYPE html><html lang=\"zh\"><head><meta charset=\"UTF-8\">\n");
    s(&mut html, "<title>WASM 阶梯制·双向公约 — 双路编译</title>\n");
    s(&mut html, "<style>\n");
    s(&mut html, "body{font-family:\"HanYuan64\",\"Noto Sans CJK SC\",monospace;background:#0a0a1a;color:#ddd;margin:0;padding:20px}\n");
    s(&mut html, "h1{font-size:22px;color:#fff;margin:4px 0}\n");
    s(&mut html, ".sub{color:#888;font-size:13px;margin-bottom:16px}\n");
    s(&mut html, ".ladder{display:flex;flex-direction:column;gap:0;align-items:center;margin-bottom:24px}\n");
    s(&mut html, ".step{width:100%;max-width:800px;text-align:center;padding:10px 16px;border:1px solid #333;border-radius:6px;position:relative}\n");
    s(&mut html, ".step-source{background:#0d1a35;border-color:#4A90D9}\n");
    s(&mut html, ".step-macro{background:#1a1a35;border-color:#7B68EE}\n");
    s(&mut html, ".step-type{background:#1a2a1a;border-color:#50B86C}\n");
    s(&mut html, ".step-dual{display:grid;grid-template-columns:1fr auto 1fr;gap:0;align-items:stretch}\n");
    s(&mut html, ".dual-left{background:#0d1a0d;border:1px solid #50B86C;border-radius:0 0 0 6px;margin-right:0}\n");
    s(&mut html, ".dual-mid{background:#2a2a10;border:1px solid #D98C4A;text-align:center;padding:10px 8px;display:flex;align-items:center;justify-content:center;min-width:80px}\n");
    s(&mut html, ".dual-right{background:#1a0d1a;border:1px solid #E86868;border-radius:0 0 6px 0}\n");
    s(&mut html, ".step-output{background:#1a1a1a;border-color:#D98C4A}\n");
    s(&mut html, ".arrow{color:#666;text-align:center;font-size:18px;padding:2px 0}\n");
    s(&mut html, ".step .label{font-size:11px;color:#888;display:block;margin-bottom:4px}\n");
    s(&mut html, ".step .title{font-size:15px;color:#fff;font-weight:bold}\n");
    s(&mut html, ".step .desc{font-size:11px;color:#888;margin-top:4px}\n");
    s(&mut html, "pre{margin:0;padding:8px;font-size:11px;text-align:left;color:#bbb;background:#05050f;border-radius:4px;overflow-x:auto;line-height:1.4}\n");
    s(&mut html, ".table{margin-bottom:24px}\n");
    s(&mut html, "table{width:100%;max-width:800px;border-collapse:collapse;font-size:11px}\n");
    s(&mut html, "th{background:#16213e;color:#7B68EE;padding:6px 8px;text-align:left;border-bottom:2px solid #333}\n");
    s(&mut html, "td{padding:4px 8px;border-bottom:1px solid #1a1a3a;font-family:monospace}\n");
    s(&mut html, "tr:nth-child(even)td{background:#0f0f24}\n");
    s(&mut html, "tr:hover td{background:#1a1a40}\n");
    s(&mut html, ".bits{color:#4A90D9}\n");
    s(&mut html, ".wasm32{color:#50B86C}\n");
    s(&mut html, ".wasm64{color:#E86868}\n");
    s(&mut html, ".rule{background:#12122a;border:1px solid #333;padding:6px 8px;border-radius:4px;font-size:11px;font-family:monospace}\n");
    s(&mut html, ".rule-m{background:#1a1a35;border-left:3px solid #7B68EE}\n");
    s(&mut html, ".rule-32{background:#0d1a0d;border-left:3px solid #50B86C}\n");
    s(&mut html, ".rule-64{background:#1a0d1a;border-left:3px solid #E86868}\n");
    s(&mut html, ".section-title{color:#7B68EE;font-size:14px;margin:20px 0 10px 0;border-bottom:1px solid #333;padding-bottom:4px}\n");
    s(&mut html, ".stats{display:flex;gap:12px;margin-bottom:20px;flex-wrap:wrap}\n");
    s(&mut html, ".stat{background:#12122a;border:1px solid #333;border-radius:6px;padding:8px 14px;font-size:12px}\n");
    s(&mut html, ".stat b{color:#4A90D9;font-size:16px;display:block}\n");
    s(&mut html, "</style></head><body>\n");

    s(&mut html, "<h1>WASM 阶梯制 · 双向公约</h1>\n");
    s(&mut html, "<p class=\"sub\">同一份源码 → 宏前缀统一 → 固定宽度类型 → 双路编译(wasm32+wasm64) → 等价验证</p>\n");

    s(&mut html, "<div class=\"stats\">\n");
    w(&mut html, "<div class=\"stat\"><b>1</b>源码</div>\n");
    w(&mut html, "<div class=\"stat\"><b>6</b>宏规则</div>\n");
    w(&mut html, "<div class=\"stat\"><b>12</b>固定宽度类型</div>\n");
    w(&mut html, "<div class=\"stat\"><b>2</b>编译目标</div>\n");
    w(&mut html, "<div class=\"stat\"><b>1</b>等价语义</div>\n");
    s(&mut html, "</div>\n");

    // ── Ladder ──
    s(&mut html, "<div class=\"ladder\">\n");

    // Step 1: Source
    s(&mut html, "<div class=\"step step-source\">\n");
    s(&mut html, "<span class=\"label\">第0阶 · 架构无关源码</span>\n");
    s(&mut html, "<span class=\"title\">containers.c</span>\n");
    s(&mut html, "<div class=\"desc\">一份源码, 同时编译 wasm32 和 wasm64</div>\n");
    s(&mut html, "<pre>\n");
    for line in shared_source_lines() {
        w(&mut html, &format!("{}\n", line));
    }
    s(&mut html, "</pre>\n");
    s(&mut html, "</div>\n");

    s(&mut html, "<div class=\"arrow\">▼ 宏前缀展开 ▼</div>\n");

    // Step 2: Macros
    s(&mut html, "<div class=\"step step-macro\">\n");
    s(&mut html, "<span class=\"label\">第1阶 · 宏前缀统一层</span>\n");
    s(&mut html, "<span class=\"title\">mqf_prefix.h</span>\n");
    s(&mut html, "<div class=\"desc\">MQF_ 宏前缀由预处理器按目标架构展开</div>\n");
    let rules = macro_rules();
    for rule in &rules {
        s(&mut html, "<div class=\"rule rule-m\">\n");
        w(&mut html, &format!("<b>{}</b>: ", rule.macro_name));
        w(&mut html, &format!("<span class=\"wasm32\">{:25}</span> │ <span class=\"wasm64\">{:25}</span>\n",
            rule.wasm32_def, rule.wasm64_def));
        w(&mut html, &format!("  <span style=\"color:#888\">{}</span>\n", rule.purpose));
        s(&mut html, "</div>\n");
    }
    s(&mut html, "</div>\n");

    s(&mut html, "<div class=\"arrow\">▼ 类型宽度分派 ▼</div>\n");

    // Step 3: Type Width
    s(&mut html, "<div class=\"step step-type\">\n");
    s(&mut html, "<span class=\"label\">第2阶 · 固定宽度保证</span>\n");
    s(&mut html, "<span class=\"title\">stdint · 跨平台等价</span>\n");
    s(&mut html, "<div class=\"desc\">同一类型名在不同位宽下保证字面语义一致</div>\n");
    s(&mut html, "<pre style=\"font-size:10px\">\n");
    w(&mut html, "类型          C          WASM32       WASM64        角色\n");
    w(&mut html, "─────────────────────────────────────────────────────────\n");
    for t in &FIXED_TYPES {
        w(&mut html, &format!("{:6} {:14} {:14} {:14} {}\n",
            t.name, t.c_name, t.wasm32_val, t.wasm64_val, t.container_role));
    }
    s(&mut html, "</pre>\n");
    s(&mut html, "</div>\n");

    s(&mut html, "<div class=\"arrow\">▼ 双向编译 ▼</div>\n");

    // Step 4: Dual compilation
    s(&mut html, "<div class=\"step step-dual\">\n");
    s(&mut html, "<div class=\"dual-left\">\n");
    s(&mut html, "<span class=\"label\" style=\"color:#50B86C\">wasm32 目标</span>\n");
    s(&mut html, "<pre style=\"font-size:10px\">\n");
    w(&mut html, ";; memory: 1 page × 64KB\n");
    w(&mut html, "(module\n");
    w(&mut html, "  (func $route\n");
    w(&mut html, "    (param $code i32)\n");
    w(&mut html, "    (result i64)\n");
    w(&mut html, "    i32.load (local.get $code)\n");
    w(&mut html, "    ;; i32 = ptr/unit size\n");
    w(&mut html, "  )\n");
    w(&mut html, "  (memory (export \"mem\") 1)\n");
    w(&mut html, ")\n");
    s(&mut html, "</pre>\n");
    s(&mut html, "</div>\n");

    s(&mut html, "<div class=\"dual-mid\">\n");
    s(&mut html, "⇅<br>一<br>份<br>源<br>码<br>⇅\n");
    s(&mut html, "</div>\n");

    s(&mut html, "<div class=\"dual-right\">\n");
    s(&mut html, "<span class=\"label\" style=\"color:#E86868\">wasm64 目标</span>\n");
    s(&mut html, "<pre style=\"font-size:10px\">\n");
    w(&mut html, ";; memory: 1 page × 64KB\n");
    w(&mut html, "(module\n");
    w(&mut html, "  (func $route\n");
    w(&mut html, "    (param $code i64)\n");
    w(&mut html, "    (result i64)\n");
    w(&mut html, "    i64.load (local.get $code)\n");
    w(&mut html, "    ;; i64 = ptr/unit size\n");
    w(&mut html, "  )\n");
    w(&mut html, "  (memory (export \"mem\") 1)\n");
    w(&mut html, ")\n");
    s(&mut html, "</pre>\n");
    s(&mut html, "</div>\n");
    s(&mut html, "</div>\n");  // step-dual end

    s(&mut html, "<div class=\"arrow\">▼ 等价验证 ▼</div>\n");

    // Step 5: Verification
    s(&mut html, "<div class=\"step step-output\">\n");
    s(&mut html, "<span class=\"label\">第3阶 · 等价输出</span>\n");
    s(&mut html, "<span class=\"title\">containers.wasm32.wasm ‖ containers.wasm64.wasm</span>\n");
    s(&mut html, "<div class=\"desc\">两个字码对 (0x5D, 0x00, 0x83, 0x0D) 验证位宽语义一致</div>\n");
    s(&mut html, "<div style=\"display:flex;gap:16px;justify-content:center;margin-top:8px\">\n");
    s(&mut html, "<div style=\"background:#0d1a0d;border:1px solid #50B86C;border-radius:4px;padding:8px 12px;font-size:11px\">\n");
    s(&mut html, "<b style=\"color:#50B86C\">wasm32</b><br>\n");
    w(&mut html, "header: <span style=\"color:#aaa\">00 61 73 6D 01 00 00 00</span><br>\n");
    w(&mut html, "type  : <span style=\"color:#aaa\">60 01 7F 01 7E</span><br>\n");
    w(&mut html, "memory: <span style=\"color:#aaa\">00 01 (1 page)</span><br>\n");
    w(&mut html, "func  : <span style=\"color:#aaa\">i32 → i64 common_divisor</span>\n");
    s(&mut html, "</div>\n");
    s(&mut html, "<div style=\"background:#1a0d1a;border:1px solid #E86868;border-radius:4px;padding:8px 12px;font-size:11px\">\n");
    s(&mut html, "<b style=\"color:#E86868\">wasm64</b><br>\n");
    w(&mut html, "header: <span style=\"color:#aaa\">00 61 73 6D 01 00 00 00</span><br>\n");
    w(&mut html, "type  : <span style=\"color:#aaa\">60 01 7E 01 7E</span><br>\n");
    w(&mut html, "memory: <span style=\"color:#aaa\">00 01 (1 page)</span><br>\n");
    w(&mut html, "func  : <span style=\"color:#aaa\">i64 → i64 common_divisor</span>\n");
    s(&mut html, "</div>\n");
    s(&mut html, "</div>\n");
    s(&mut html, "</div>\n");

    s(&mut html, "</div>\n"); // ladder end

    // ── Type table ──
    w(&mut html, "<div class=\"section-title\">完整类型分派表</div>\n");
    s(&mut html, "<div class=\"table\">\n");
    s(&mut html, "<table><thead><tr>\n");
    s(&mut html, "<th>类型</th><th>C typedef</th><th>位宽</th><th>符号</th>\n");
    s(&mut html, "<th>wasm32 表示</th><th>wasm64 表示</th><th>角色</th>\n");
    s(&mut html, "</tr></thead><tbody>\n");
    for t in &FIXED_TYPES {
        s(&mut html, "<tr>");
        w(&mut html, &format!("<td><b>{}</b></td>", t.name));
        w(&mut html, &format!("<td>{}</td>", t.c_name));
        w(&mut html, &format!("<td><span class=\"bits\">{}b</span></td>", t.bits));
        w(&mut html, &format!("<td>{}</td>", if t.signed { "有" } else { "无" }));
        w(&mut html, &format!("<td class=\"wasm32\">{}</td>", t.wasm32_val));
        w(&mut html, &format!("<td class=\"wasm64\">{}</td>", t.wasm64_val));
        w(&mut html, &format!("<td style=\"color:#888\">{}</td>", t.container_role));
        s(&mut html, "</tr>\n");
    }
    s(&mut html, "</tbody></table>\n");
    s(&mut html, "</div>\n");

    s(&mut html, "</body></html>\n");
    html
}

// ═══════════════════════════════════════════════════════════════════════════
// JSON — dual wasm descriptor
// ═══════════════════════════════════════════════════════════════════════════

fn dual_json() -> String {
    let mut j = String::new();
    writeln!(j, "{{").unwrap();
    writeln!(j, r#"  "strategy": "阶梯制·双向公约","#).unwrap();
    writeln!(j, r#"  "source": "containers.c (单源码)","#).unwrap();

    writeln!(j, "  \"fixed_width_types\": [").unwrap();
    for (i, t) in FIXED_TYPES.iter().enumerate() {
        if i > 0 { j.push(','); }
        writeln!(j, "").unwrap();
        writeln!(j, "    {{").unwrap();
        writeln!(j, r#"      "name": "{}","#, t.name).unwrap();
        writeln!(j, r#"      "c_typedef": "{}","#, t.c_name).unwrap();
        writeln!(j, r#"      "bits": {},"#, t.bits).unwrap();
        writeln!(j, r#"      "signed": {},"#, t.signed).unwrap();
        writeln!(j, r#"      "wasm32": "{}","#, t.wasm32_val).unwrap();
        writeln!(j, r#"      "wasm64": "{}","#, t.wasm64_val).unwrap();
        writeln!(j, r#"      "role": "{}""#, t.container_role).unwrap();
        write!(j, "    }}").unwrap();
    }
    writeln!(j, "").unwrap();
    writeln!(j, "  ],").unwrap();

    writeln!(j, "  \"macro_rules\": [").unwrap();
    let rules = macro_rules();
    for (i, r) in rules.iter().enumerate() {
        if i > 0 { j.push(','); }
        writeln!(j, "").unwrap();
        writeln!(j, "    {{").unwrap();
        writeln!(j, r#"      "name": "{}","#, r.macro_name).unwrap();
        writeln!(j, r#"      "wasm32": "{}","#, r.wasm32_def).unwrap();
        writeln!(j, r#"      "wasm64": "{}""#, r.wasm64_def).unwrap();
        write!(j, "    }}").unwrap();
    }
    writeln!(j, "").unwrap();
    writeln!(j, "  ]").unwrap();

    write!(j, "}}").unwrap();
    j
}

fn s(html: &mut String, text: &str) { html.push_str(text); }
fn w(html: &mut String, text: &str) { html.push_str(text); }

fn main() {
    let args: Vec<String> = env::args().collect();
    let out_dir = if args.len() > 1 { args[1].clone() } else { ".".to_string() };

    let html = ladder_html();
    let json = dual_json();

    let hp = format!("{}/wasm_ladder.html", out_dir);
    let jp = format!("{}/dual_wasm.json", out_dir);
    std::fs::write(&hp, &html).expect("write html");
    std::fs::write(&jp, &json).expect("write json");

    eprintln!("WASM 阶梯制·双向公约:");
    eprintln!("  HTML: {} ({} bytes)", hp, html.len());
    eprintln!("  JSON:  {} ({} bytes)", jp, json.len());
}
