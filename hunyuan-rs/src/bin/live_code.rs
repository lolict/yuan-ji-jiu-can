// ═════════════════════════════════════════════════════════════════════════
// live-code.rs — 活码编译器命令行工具 (在场本体论版)
// ═════════════════════════════════════════════════════════════════════════
//
//   活码 = 符号在场登记,不是数值。
//   拼音"起点"激活在场登记查询,声调锁定登记。
//
// 用法:
//   live-code <韵母> [声调]          编译在场候选
//   live-code --float <韵母>         自由浮动 (未锁定声调)
//   live-code --resonate <韵母>      跨韵母共振查询
//   live-code --code <韵母> <字符>   单字符在场指纹
//   live-code --demo                 演示模式
//
// ═════════════════════════════════════════════════════════════════════════

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("用法: {} <韵母> [声调]", args[0]);
        eprintln!("      {} --float <韵母>", args[0]);
        eprintln!("      {} --resonate <韵母>", args[0]);
        eprintln!("      {} --code <韵母> <字符>", args[0]);
        eprintln!("      {} --demo", args[0]);
        eprintln!("示例: {} an 1", args[0]);
        eprintln!("      {} ong", args[0]);
        eprintln!("      {} --demo", args[0]);
        process::exit(1);
    }

    match args[1].as_str() {
        "--float" => {
            if args.len() < 3 { eprintln!("缺少韵母参数"); process::exit(1); }
            cmd_float(&args[2]);
        }
        "--resonate" => {
            if args.len() < 3 { eprintln!("缺少韵母参数"); process::exit(1); }
            cmd_resonate(&args[2]);
        }
        "--code" => {
            if args.len() < 4 { eprintln!("缺少参数: --code <韵母> <字符>"); process::exit(1); }
            cmd_code(&args[2], &args[3]);
        }
        "--demo" => cmd_demo(),
        spelling => {
            let tone = if args.len() > 2 {
                args[2].parse().unwrap_or(0)
            } else { 0 };
            cmd_compile(spelling, tone);
        }
    }
}

fn cmd_compile(spelling: &str, tone: u8) {
    let cands = mqf::livecode_compiler::live_compile(spelling, tone, &[]);
    if cands.is_empty() {
        println!("未找到候选 (可能韵母不在表中)");
        return;
    }

    println!("╔══════════════════════════════════════════════════════╗");
    println!("║  活码编译 · {} @{}                                   ║", spelling, tone);
    println!("╠══════════════════════════════════════════════════════╣");

    for (i, c) in cands.iter().enumerate().take(10) {
        let mark = if c.attendance.mode == mqf::presence_ontology::PresenceMode::Global { "全" } else { "局" };
        println!("║  {}. {}  @{}  共振={}  加={}  指纹={:016x} {} ║",
            i + 1,
            c.glyph,
            c.tone,
            c.resonance_count,
            c.addition_addr,
            c.fingerprint(),
            mark,
        );
    }

    println!("╠══════════════════════════════════════════════════════╣");
    println!("║  域: {:<12}  模式: {}                      ║",
        cands[0].attendance.domain,
        cands[0].attendance.mode);
    println!("╚══════════════════════════════════════════════════════╝");
}

fn cmd_float(spelling: &str) {
    let cands = mqf::livecode_compiler::live_float(spelling);

    println!("自由浮动 · {} (未锁定声调, {} 候选)", spelling, cands.len());
    for (i, c) in cands.iter().enumerate().take(8) {
        print!("{}@{}={:08x}  ", c.glyph, c.tone, c.fingerprint() % 100000);
        if (i + 1) % 4 == 0 { println!(); }
    }
    println!();
}

fn cmd_resonate(spelling: &str) {
    let result = mqf::livecode_compiler::live_resonate(spelling);

    println!("跨韵母共振 · {}", spelling);
    if result.is_empty() {
        println!("  (无共振韵母)");
        return;
    }

    for (sp, chars) in &result {
        println!("  {} → {}", sp, chars.iter().collect::<String>());
    }
}

fn cmd_code(spelling: &str, ch_str: &str) {
    let ch = ch_str.chars().next().unwrap();
    let cands = mqf::livecode_compiler::live_compile(spelling, 1, &[ch]);

    if let Some(c) = cands.first() {
        println!("字符: {}", c.glyph);
        println!("拼音: {}", c.spelling);
        println!("声调: {}", c.tone);
        println!("加法寻址: {}", c.addition_addr);
        println!("共振兄弟数: {}", c.resonance_count);
        println!("在场指纹: {:016x}", c.fingerprint());
        println!("在场域: {}", c.attendance.domain);
        println!("到场模式: {}", c.attendance.mode);
        println!("排名分: {}", c.rank_score);
    } else {
        println!("未找到候选");
    }
}

fn cmd_demo() {
    for spelling in &["an", "ong", "ing", "i", "u"] {
        println!("\n═══ {} ═══", spelling);
        let cands = mqf::livecode_compiler::live_compile(*spelling, 1, &[]);
        for (i, c) in cands.iter().enumerate().take(3) {
            println!("  {}. {} @{} 共振={} 指纹={:08x}",
                i + 1, c.glyph, c.tone, c.resonance_count, c.fingerprint() % 100000);
        }
    }
}
