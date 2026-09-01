// ══════════════════════════════════════════════════════════════════
// phase_tool.rs — 颜色进制 CLI 工具
// ══════════════════════════════════════════════════════════════════
//
// 编译运行:
//   cd hunyuan-rs
//   cargo run --example phase_tool -- <子命令>
//
// 子命令:
//   canvas new <id> <width> <height>         创建空白画布
//   canvas set <id> <x> <y> <phase> <bright> <weight>  设置像素
//   canvas add <base_id> <part_id> <result_id> <ox> <oy> 加法运算
//   canvas sub <base_id> <part_id> <result_id> <ox> <oy> 减法运算
//   canvas info <id>                          画布信息
//   canvas export <id>                        导出RGB位图
//   kb new                                   创建知识库
//   kb add-ontology <id> <glyph> <level> <type> 添加本体
//   kb add-blacklist <id> <pattern> <severity> 添加黑名单
//   kb export                                 导出知识库JSON
//   node add <id> <signal_type>               添加节点
//   node connect <from> <to>                  连接节点
//   node traverse <start_id> <depth>          链式传导遍历
//   network-eq <id_a> <id_b>                  计算等效距离
//   demo                                     运行演示
//
// ══════════════════════════════════════════════════════════════════

use mqf::phase_encoding::*;
use mqf::os_layer::bare_metal::BareMetalOs;
use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "demo" => run_demo(),
        "canvas" => handle_canvas(&args[2..]),
        "kb" => handle_kb(&args[2..]),
        "node" => handle_node(&args[2..]),
        "network-eq" => handle_network_eq(&args[2..]),
        _ => print_help(),
    }
}

fn print_help() {
    println!("颜色进制 CLI 工具 — phase_tool");
    println!("");
    println!("用法: cargo run --example phase_tool -- <子命令> [参数]");
    println!("");
    println!("子命令:");
    println!("  canvas new <id> <width> <height>");
    println!("  canvas set <id> <x> <y> <phase> <bright> <weight>");
    println!("  canvas add <base> <part> <result> <ox> <oy>");
    println!("  canvas sub <base> <part> <result> <ox> <oy>");
    println!("  canvas info <id>");
    println!("  canvas export <id>");
    println!("  kb new");
    println!("  kb add-ontology <id> <glyph> <level> <type>");
    println!("  kb add-blacklist <id> <pattern> <severity>");
    println!("  kb export");
    println!("  node add <id> <signal_type>");
    println!("  node connect <from> <to>");
    println!("  node traverse <id> <depth>");
    println!("  network-eq <id_a> <id_b>");
    println!("  demo");
}

fn run_demo() {
    println!("═══ 颜色进制 · 演示 ═══\n");

    // 0. 创建 OS 层 (create_canvas 需要 &dyn OsLayer)
    let os = BareMetalOs::new();

    // 1. 创建画布
    let mut engine = DrawingEngine::new();
    println!("✓ 创建绘图引擎");

    // 2. 创建基元画布 (模拟字形 "a" 的像素)
    engine.create_canvas(&os, 16, 16, "glyph_a");
    let base = engine.get_canvas("glyph_a").unwrap().clone();
    let mut base_mut = base.clone();
    // 绘制简单的圆形 (模拟字母 a)
    let cx = 8.0;
    let cy = 8.0;
    let r = 5.0;
    for y in 0..16 {
        for x in 0..16 {
            let dx = x as f64 - cx;
            let dy = y as f64 - cy;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist <= r && dist >= r - 1.5 {
                base_mut.set(x, y, PhaseUnit::new(128, 200, 128, 1, 0));
            }
        }
    }
    engine.canvas_cache.insert("glyph_a".to_string(), base_mut);
    println!("✓ 创建基元画布 glyph_a (16x16, 模拟字母a)");

    // 3. 创建声调部件
    engine.create_canvas(&os, 8, 8, "tone_acute");
    let part = engine.get_canvas("tone_acute").unwrap().clone();
    let mut part_mut = part.clone();
    // 绘制撇号形状的声调
    for i in 0..4 {
        part_mut.set(6 - i, 3 + i, PhaseUnit::new(50, 180, 100, 2, 1));
        part_mut.set(7 - i, 3 + i, PhaseUnit::new(50, 180, 100, 2, 1));
    }
    engine.canvas_cache.insert("tone_acute".to_string(), part_mut);
    println!("✓ 创建声调部件 tone_acute (8x8, 撇号形状)");

    // 4. 执行加法运算 (部件安装)
    let result = engine.add_operation(&os, "glyph_a", "tone_acute", 0, 0, "glyph_a_acute");
    match result {
        Ok(canvas) => {
            println!("✓ 加法运算完成: glyph_a + tone_acute → glyph_a_acute");
            println!("  结果非空像素: {}", canvas.non_empty_count());
            println!("  操作: {}", canvas.metadata.operation);
        }
        Err(e) => println!("✗ 加法运算失败: {}", e),
    }

    // 5. 执行减法运算 (部件拆解)
    let sub_result = engine.subtract_operation(&os, "glyph_a_acute", "tone_acute", 0, 0, "stripped_a");
    match sub_result {
        Ok(canvas) => {
            println!("✓ 减法运算完成: glyph_a_acute - tone_acute → stripped_a");
            println!("  结果非空像素: {}", canvas.non_empty_count());
        }
        Err(e) => println!("✗ 减法运算失败: {}", e),
    }

    // 6. 计算等效距离
    let dist = engine.get_canvas("glyph_a").unwrap()
        .equivalent_distance(engine.get_canvas("glyph_a_acute").unwrap());
    println!("✓ 等效镜像距离(glyph_a, glyph_a_acute): {:.2}", dist);

    // 7. 和谐度得分
    let harmony = engine.get_canvas("glyph_a").unwrap().harmony_score();
    println!("✓ 全局协调得分 (glyph_a): {:.3}", harmony);

    // 8. 创建区块链纠缠网络
    let mut network = EntanglementNetwork::new("演示网络");
    let mut node_a = EntangledNode::new(&os, "node_a", "visual");
    node_a.annotation.level = "L1".to_string();
    node_a.annotation.node_type = "韵母".to_string();
    node_a.annotation.glyph_symbol = "a".to_string();
    node_a.annotation.owner = "SELF".to_string();
    node_a.annotation.aesthetic_score = 0.85;
    node_a.annotation.family = "ipa".to_string();

    let mut node_b = EntangledNode::new(&os, "node_o", "visual");
    node_b.annotation.level = "L1".to_string();
    node_b.annotation.node_type = "韵母".to_string();
    node_b.annotation.glyph_symbol = "o".to_string();
    node_b.annotation.owner = "SELF".to_string();
    node_b.annotation.aesthetic_score = 0.80;
    node_b.annotation.family = "ipa".to_string();

    network.add_node(node_a.clone());
    network.add_node(node_b.clone());
    network.connect("node_a", "node_o").unwrap();
    println!("✓ 创建纠缠网络: 2个节点, node_a → node_o");

    // 9. 等效距离计算
    let eq_dist = network.equivalent_distance("node_a", "node_o").unwrap();
    println!("✓ 等效镜像距离(node_a, node_o): {:.3}", eq_dist);
    println!("  公倍数有效: {}", network.is_valid_common_multiple("node_a", "node_o", 0.3));

    // 10. 链式传导遍历
    let visited = network.traverse_from("node_a", 10);
    println!("✓ 链式传导遍历 (从 node_a 开始): {:?}", visited);

    // 11. 分布式制衡校验
    let balance = WeavingBalanceNetwork::new();
    let output = balance.evaluate(&node_a, &node_b, &network, None, None);
    println!("✓ 分布式制衡校验:");
    for dim in &output.dimension_results {
        println!("  {}: {} (拉力{:.1}) — {}",
            match dim.dimension {
                BalanceDimension::LevelStack => "层级栈",
                BalanceDimension::TypeIdentity => "类型身份",
                BalanceDimension::VisualAesthetic => "视觉审美",
                BalanceDimension::Ownership => "归属权",
                BalanceDimension::DiffractionRule => "衍射规则",
                BalanceDimension::Blacklist => "黑名单",
            },
            if dim.passed { "✓通过" } else { "✗否决" },
            dim.force,
            dim.reason);
    }
    println!("  最终判定: {}", if output.final_verdict { "✓接纳" } else { "✗排斥" });
    println!("  全局协调得分: {:.3}", output.harmony_score);

    // 12. 知识库
    let mut kb = KnowledgeBase::new();
    kb.add_ontology(OntologyEntry {
        glyph_id: "glyph_a".to_string(),
        glyph: "a".to_string(),
        level: "L1".to_string(),
        entry_type: "韵母".to_string(),
        visual_weight: 0.5,
        family: "ipa".to_string(),
        owner: "SELF".to_string(),
        aesthetic_score: 0.85,
        canvas_id: Some("glyph_a".to_string()),
        audio_id: None,
    });
    kb.add_blacklist(BlacklistEntry {
        entry_id: "err_cross_level".to_string(),
        error_description: "跨层级衍射".to_string(),
        error_pattern: "L1-L3".to_string(),
        severity: 0.9,
    });
    println!("✓ 创建知识库: 1个本体, 1个黑名单");
    println!("  黑名单检查 L1-L3: {}", kb.is_blacklisted("L1-L3"));

    // 13. 输出报告
    println!("\n═══ 引擎报告 ═══");
    println!("{}", engine.report());

    // 14. JSON导出示例
    match kb.to_json() {
        Ok(json) => {
            println!("✓ 知识库JSON导出 (前500字符):");
            println!("{}", &json[..json.len().min(500)]);
        }
        Err(e) => println!("✗ JSON导出失败: {}", e),
    }

    println!("\n═══ 演示完成 ═══");
}

fn handle_canvas(args: &[String]) {
    let os = BareMetalOs::new();
    let mut engine = DrawingEngine::new();
    if args.is_empty() {
        println!("子命令: new, set, add, sub, info, export");
        return;
    }
    match args[0].as_str() {
        "new" => {
            if args.len() < 4 {
                println!("用法: canvas new <id> <width> <height>");
                return;
            }
            let id = &args[1];
            let w: u32 = args[2].parse().unwrap_or(16);
            let h: u32 = args[3].parse().unwrap_or(16);
            engine.create_canvas(&os, w, h, id);
            println!("✓ 创建画布 {} ({}x{})", id, w, h);
        }
        "info" => {
            if args.len() < 2 {
                println!("用法: canvas info <id>");
                return;
            }
            let id = &args[1];
            match engine.get_canvas(id) {
                Some(c) => {
                    println!("画布 {}: {}x{}, 非空像素: {}", id, c.width, c.height, c.non_empty_count());
                    println!("和谐度: {:.3}", c.harmony_score());
                    if let Some(bbox) = c.bounding_box() {
                        println!("边界框: ({},{})-({},{})", bbox.0, bbox.1, bbox.2, bbox.3);
                    }
                }
                None => println!("画布 {} 不存在", id),
            }
        }
        "export" => {
            if args.len() < 2 {
                println!("用法: canvas export <id>");
                return;
            }
            let id = &args[1];
            match engine.get_canvas(id) {
                Some(c) => {
                    let bitmap = c.to_rgb_bitmap();
                    println!("RGB位图 ({}x{}):", c.width, c.height);
                    for row in &bitmap {
                        for &(r, g, b) in row {
                            // 简化的ANSI颜色输出
                            if r == 0 && g == 0 && b == 0 {
                                print!("  ");
                            } else {
                                print!("██");
                            }
                        }
                        println!();
                    }
                }
                None => println!("画布 {} 不存在", id),
            }
        }
        _ => println!("未知子命令: {}", args[0]),
    }
}

fn handle_kb(args: &[String]) {
    let mut kb = KnowledgeBase::new();
    if args.is_empty() {
        println!("子命令: new, add-ontology, add-blacklist, export");
        return;
    }
    match args[0].as_str() {
        "new" => {
            println!("✓ 创建知识库");
            println!("{}", kb.to_json().unwrap_or_default());
        }
        "add-ontology" => {
            if args.len() < 5 {
                println!("用法: kb add-ontology <id> <glyph> <level> <type>");
                return;
            }
            kb.add_ontology(OntologyEntry {
                glyph_id: args[1].clone(),
                glyph: args[2].clone(),
                level: args[3].clone(),
                entry_type: args[4].clone(),
                visual_weight: 0.5,
                family: "default".to_string(),
                owner: "SELF".to_string(),
                aesthetic_score: 0.5,
                canvas_id: None,
                audio_id: None,
            });
            println!("✓ 添加本体条目 {}", args[1]);
        }
        "add-blacklist" => {
            if args.len() < 4 {
                println!("用法: kb add-blacklist <id> <pattern> <severity>");
                return;
            }
            let severity: f64 = args[3].parse().unwrap_or(0.5);
            kb.add_blacklist(BlacklistEntry {
                entry_id: args[1].clone(),
                error_description: "用户添加".to_string(),
                error_pattern: args[2].clone(),
                severity,
            });
            println!("✓ 添加黑名单条目 {}", args[1]);
        }
        "export" => {
            match kb.to_json() {
                Ok(json) => println!("{}", json),
                Err(e) => println!("导出失败: {}", e),
            }
        }
        _ => println!("未知子命令: {}", args[0]),
    }
}

fn handle_node(args: &[String]) {
    let os = BareMetalOs::new();
    let mut network = EntanglementNetwork::new("cli");
    if args.is_empty() {
        println!("子命令: add, connect, traverse");
        return;
    }
    match args[0].as_str() {
        "add" => {
            if args.len() < 3 {
                println!("用法: node add <id> <signal_type>");
                return;
            }
            network.add_node(EntangledNode::new(&os, &args[1], &args[2]));
            println!("✓ 添加节点 {} (类型: {})", args[1], args[2]);
        }
        "connect" => {
            if args.len() < 3 {
                println!("用法: node connect <from> <to>");
                return;
            }
            match network.connect(&args[1], &args[2]) {
                Ok(_) => println!("✓ 连接 {} → {}", args[1], args[2]),
                Err(e) => println!("✗ 连接失败: {}", e),
            }
        }
        "traverse" => {
            if args.len() < 3 {
                println!("用法: node traverse <start_id> <depth>");
                return;
            }
            let depth: usize = args[2].parse().unwrap_or(10);
            let visited = network.traverse_from(&args[1], depth);
            println!("遍历结果: {:?}", visited);
        }
        _ => println!("未知子命令: {}", args[0]),
    }
}

fn handle_network_eq(args: &[String]) {
    let os = BareMetalOs::new();
    if args.len() < 2 {
        println!("用法: network-eq <id_a> <id_b>");
        return;
    }
    // 创建临时网络演示
    let mut network = EntanglementNetwork::new("tmp");
    let mut a = EntangledNode::new(&os, &args[0], "visual");
    a.annotation.level = "L1".to_string();
    a.annotation.owner = "SELF".to_string();
    a.annotation.aesthetic_score = 0.8;

    let mut b = EntangledNode::new(&os, &args[1], "visual");
    b.annotation.level = "L1".to_string();
    b.annotation.owner = "SELF".to_string();
    b.annotation.aesthetic_score = 0.7;

    network.add_node(a);
    network.add_node(b);

    let dist = network.equivalent_distance(&args[0], &args[1]).unwrap_or(1.0);
    println!("等效距离({}, {}): {:.3}", args[0], args[1], dist);
    println!("公倍数有效 (阈值0.3): {}", network.is_valid_common_multiple(&args[0], &args[1], 0.3));
}
