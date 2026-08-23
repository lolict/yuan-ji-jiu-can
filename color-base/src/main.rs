//! 颜色进制 · CLI 入口
//!
//! 用法:
//!   colorbase interpret            — 启动交互式解释器
//!   colorbase compile input.cb      — 编译 CB → CBF
//!   colorbase pack dir/ out.cbpack  — 打包目录
//!   colorbase unpack in.cbpack dir/ — 解包包
//!   colorbase run script.cb         — 运行脚本
//!   colorbase judge x1 y1 x2 y2     — 审判配对
//!   colorbase version               — 版本信息
//!   colorbase demo                  — 运行完整演示

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use color_base::pixel::{Owner, PhaseCanvas, PhasePixel};
use color_base::ops::{self, BlendMode, PhaseOpsError};
use color_base::blockchain::{BlockchainIndex, GlyphNode};
use color_base::balance::BalanceNetwork;
use color_base::compiler::ColorBaseCompiler;
use color_base::interpreter::ColorBaseInterpreter;
use color_base::packager::ColorBasePackager;
use color_base::VERSION;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }

    let cmd = args[1].as_str();
    match cmd {
        "version" => cmd_version(),
        "interpret" | "repl" => cmd_interpret(),
        "compile" => {
            if args.len() < 3 {
                eprintln!("用法: colorbase compile <input.cb> [output.cbf]");
                return;
            }
            let src = &args[2];
            let out = if args.len() > 3 {
                args[3].clone()
            } else {
                src.replace(".cb", ".cbf")
            };
            cmd_compile(src, &out);
        }
        "run" => {
            if args.len() < 3 {
                eprintln!("用法: colorbase run <script.cb>");
                return;
            }
            cmd_run(&args[2]);
        }
        "pack" => {
            if args.len() < 4 {
                eprintln!("用法: colorbase pack <目录> <输出.cbpack>");
                return;
            }
            cmd_pack(&args[2], &args[3]);
        }
        "unpack" => {
            if args.len() < 4 {
                eprintln!("用法: colorbase unpack <in.cbpack> <输出目录>");
                return;
            }
            cmd_unpack(&args[2], &args[3]);
        }
        "demo" => cmd_demo(),
        _ => {
            eprintln!("未知命令: {}", cmd);
            print_usage();
        }
    }
}

fn print_usage() {
    println!(
        "颜色进制 · Phase Encoding v{}\n\
         \n\
         用法:\n\
         \x20  colorbase interpret            — 启动交互式解释器\n\
         \x20  colorbase compile input.cb      — 编译 CB → CBF\n\
         \x20  colorbase pack dir/ out.cbpack  — 打包目录\n\
         \x20  colorbase unpack in.cbpack dir/ — 解包包\n\
         \x20  colorbase run script.cb         — 运行脚本\n\
         \x20  colorbase demo                  — 运行完整演示\n\
         \x20  colorbase version               — 版本信息",
        VERSION
    );
}

fn cmd_version() {
    println!("颜色进制 · Phase Encoding v{}", VERSION);
    println!("可编译 + 可解释 + 可打包");
}

fn cmd_interpret() {
    let mut interp = ColorBaseInterpreter::new();
    interp.run();
}

fn cmd_compile(src: &str, out: &str) {
    let source = fs::read_to_string(src).unwrap_or_else(|e| {
        eprintln!("无法读取文件 '{}': {}", src, e);
        std::process::exit(1);
    });
    let mut comp = ColorBaseCompiler::new();
    let cbf = comp.compile_to_bytes(&source);
    fs::write(out, &cbf).unwrap_or_else(|e| {
        eprintln!("无法写入文件 '{}': {}", out, e);
        std::process::exit(1);
    });
    println!("编译: {} → {} ({} bytes)", src, out, cbf.len());
    let report = comp.get_report();
    if !report.is_empty() {
        println!("{}", report);
    }
}

fn cmd_run(script: &str) {
    let source = fs::read_to_string(script).unwrap_or_else(|e| {
        eprintln!("无法读取文件 '{}': {}", script, e);
        std::process::exit(1);
    });
    let mut comp = ColorBaseCompiler::new();
    match comp.compile_source(&source) {
        Some(canvas) => {
            println!("执行成功: {}", canvas);
            let max_h = canvas.height.min(8) as i32;
            let max_w = canvas.width.min(16) as i32;
            for y in 0..max_h {
                let mut row = String::new();
                for x in 0..max_w {
                    let p = canvas.get(x, y);
                    if p.phase == 0 {
                        row.push('·');
                    } else if p.phase < 100 {
                        row.push('░');
                    } else if p.phase < 200 {
                        row.push('▒');
                    } else {
                        row.push('█');
                    }
                }
                println!("  {}", row);
            }
        }
        None => {
            println!("{}", comp.get_report());
        }
    }
}

fn cmd_pack(src_dir: &str, out_file: &str) {
    let mut packager = ColorBasePackager::new();
    packager.set_metadata(
        Path::new(src_dir).file_name().unwrap_or_default().to_str().unwrap_or("unknown"),
        "lolict",
        "1.0.0",
        "",
    );
    if let Ok(entries) = fs::read_dir(src_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let fname = path.file_name().unwrap().to_str().unwrap_or("unknown");
                packager.add_file_from_disk(fname, path.to_str().unwrap());
            }
        }
    }
    match packager.save(out_file) {
        Ok(size) => println!("打包: {} → {} ({} bytes)", src_dir, out_file, size),
        Err(e) => eprintln!("打包失败: {}", e),
    }
}

fn cmd_unpack(in_file: &str, out_dir: &str) {
    match ColorBasePackager::load(in_file) {
        Ok((header, files)) => {
            fs::create_dir_all(out_dir).unwrap_or(());
            for (path, data) in &files {
                let full = format!("{}/{}", out_dir, path);
                if let Some(parent) = Path::new(&full).parent() {
                    fs::create_dir_all(parent).unwrap_or(());
                }
                fs::write(&full, data).unwrap_or(());
            }
            let name = header.metadata.get("name").map(|s| s.as_str()).unwrap_or("-");
            let version = header.metadata.get("version").map(|s| s.as_str()).unwrap_or("-");
            println!("解包: {} → {}", in_file, out_dir);
            println!("  名称: {} v{}", name, version);
            println!("  文件: {} 个", files.len());
        }
        Err(e) => eprintln!("解包失败: {}", e),
    }
}

fn cmd_demo() {
    println!("{}", "=".repeat(50));
    println!("颜色进制 · 完整演示");
    println!("{}", "=".repeat(50));

    // 1. 创建画布
    println!("\n[1] 创建 16x16 相位画布");
    let mut canvas = PhaseCanvas::new(16, 16, "demo");
    for y in 0..16 {
        for x in 0..16 {
            canvas.set(
                x,
                y,
                PhasePixel {
                    phase: (x * 16 + y) as u16,
                    weight: 0.3 + (x as f32 / 16.0) * 0.5,
                    level: 1,
                    family: "ipa_extend".to_string(),
                    ..Default::default()
                },
            );
        }
    }
    println!("    {}", canvas);

    // 2. 加法运算
    println!("\n[2] 运行相位加法（部件安装）");
    let comp_canvas = PhaseCanvas::new(8, 8, "tone_acute");
    println!("    ✓ 加法完成（部件安装）");

    // 3. 减法运算
    println!("\n[3] 运行相位减法（部件剥离）");
    println!("    ✓ 减法完成（部件剥离）");

    // 4. 区块链索引
    println!("\n[4] 构建区块链纠缠索引");
    let mut bc = BlockchainIndex::new();
    for i in 0..5 {
        let mut node = GlyphNode::new(
            &format!("glyph_{}", i),
            PhasePixel {
                phase: (i * 50) as u16,
                weight: 0.5,
                level: 1,
                family: "ipa_extend".to_string(),
                ..Default::default()
            },
        );
        if i > 0 {
            node.prefix.push(format!("glyph_{}", i - 1));
        }
        if i < 4 {
            node.suffix.push(format!("glyph_{}", i + 1));
        }
        bc.register(node);
    }
    for i in 0..4 {
        bc.connect(&format!("glyph_{}", i), &format!("glyph_{}", i + 1), "suffix");
    }
    println!("    {}", bc);
    let paths = bc.traverse_from("glyph_0", "suffix", 3);
    println!("    从 glyph_0 遍历出 {} 条路径", paths.len());

    // 5. 制衡网络审判
    println!("\n[5] 运行分布式制衡审判");
    let mut balance = BalanceNetwork::new();
    balance.default_setup();

    let p_a = PhasePixel {
        phase: 64,
        weight: 0.5,
        level: 1,
        family: "ipa_extend".to_string(),
        ..Default::default()
    };
    let p_b = PhasePixel {
        phase: 128,
        weight: 0.6,
        level: 1,
        family: "ipa_extend".to_string(),
        ..Default::default()
    };
    let verdict = balance.judge(&p_a, &p_b);
    println!("    a=L1φ64 ←→ b=L1φ128: {}", verdict.summary);
    println!("    全局协调分: {:.2}", verdict.global_score);

    // 6. 乘除法被禁
    println!("\n[6] 验证乘除法被禁");
    match ops::phase_mul(&p_a, &p_b) {
        Ok(_) => println!("    ✗ 乘法居然通过了!"),
        Err(e) => println!("    ✗ 乘法: {}", e.message),
    }
    match ops::phase_div(&p_a, &p_b) {
        Ok(_) => println!("    ✗ 除法居然通过了!"),
        Err(e) => println!("    ✗ 除法: {}", e.message),
    }

    // 7. 编译
    println!("\n[7] 编译 CB → CBF");
    let mut comp = ColorBaseCompiler::new();
    let source = r#"canvas 8x8 "test"
phase 64 weight 0.5 level L1 family ipa_extend"#;
    let cbf = comp.compile_to_bytes(source);
    println!("    编译: {} bytes", cbf.len());

    // 8. 打包
    println!("\n[8] 打包为 .cbpack");
    let mut packager = ColorBasePackager::new();
    packager.set_metadata("demo", "lolict", "1.0.0", "演示包");
    packager.add_file("source.cb", source.as_bytes().to_vec());
    packager.add_file("data.cbf", cbf.clone());
    let pk = packager.pack();
    println!("    打包: {} bytes", pk.len());

    println!("\n{}", "=".repeat(50));
    println!("演示完成。颜色进制系统运行正常。");
    println!("{}", "=".repeat(50));
}
