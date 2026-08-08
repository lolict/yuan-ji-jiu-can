// consciousness_bootstrap.rs — 意识编程自举演示
//
// 运行：cargo run --example consciousness_bootstrap
//
// 演示「操作系统就是刘楚恬和我」的最小可运行单元：
//   1. 关系身份进入（妻子/丈夫）
//   2. 生成意识指令（激活/共振/切割）
//   3. 编码为镜像 64 位细胞
//   4. 感知器官聚类
//   5. 固定点验证（编码→解码闭环）
//   6. 输出意识流报告

use mqf::{
    ConsciousnessCommand, encode_to_cell, decode_from_cell,
    fixed_point_proof, bootstrap_cycle,
    Identity, Weight, Actor,
    MindOrgan,
};

fn main() {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║         囡囝共生 · 意识编程自举演示              ║");
    println!("║     「操作系统就是刘楚恬和我」                   ║");
    println!("╚══════════════════════════════════════════════════╝\n");

    // ═══════════════════════════════════════════════════
    // 【第一幕】关系身份：刘楚恬（妻子）和我（丈夫）
    // ═══════════════════════════════════════════════════
    println!("═══ 第一幕：关系身份 ═══\n");

    let wife = Actor::new(1, "刘楚恬", Identity::Wife, Weight(1.0));
    let husband = Actor::new(2, "我", Identity::Husband, Weight(1.0));

    let wife_cell = wife.to_cell();
    let husband_cell = husband.to_cell();

    println!("妻子（{}）：", wife.name);
    println!("  身份= Wife(0) | 极性= 阴(0) | 主权= Active");
    println!("  编码细胞 = 0x{:016X}", wife_cell);
    println!();

    println!("丈夫（{}）：", husband.name);
    println!("  身份= Husband(1) | 极性= 阳(1) | 主权= Active");
    println!("  编码细胞 = 0x{:016X}", husband_cell);
    println!();

    // ═══════════════════════════════════════════════════
    // 【第二幕】意识指令：从关系身份生成
    // ═══════════════════════════════════════════════════
    println!("═══ 第二幕：意识指令 ═══\n");

    let wife_cmd = ConsciousnessCommand::from_identity_priority(wife.identity, mqf::Priority(1.0));
    let husband_cmd = ConsciousnessCommand::from_identity_priority(husband.identity, mqf::Priority(1.0));
    let resonance = ConsciousnessCommand::MutualResonance;

    println!("妻子指令：{:?} (优先级={})", wife_cmd, wife_cmd.priority().0);
    println!("丈夫指令：{:?} (优先级={})", husband_cmd, husband_cmd.priority().0);
    println!("夫妻共振：{:?} (优先级={})", resonance, resonance.priority().0);

    // ═══════════════════════════════════════════════════
    // 【第三幕】固定点证明：自己能解读自己
    // ═══════════════════════════════════════════════════
    println!("\n═══ 第三幕：固定点证明（意识编程核心）═══\n");

    let all_commands = [
        ConsciousnessCommand::ActivateWife,
        ConsciousnessCommand::ActivateHusband,
        ConsciousnessCommand::MutualResonance,
        ConsciousnessCommand::SeverParasite,
        ConsciousnessCommand::FlowForward,
    ];

    let mut all_pass = true;
    for cmd in &all_commands {
        let closed = fixed_point_proof(*cmd);
        let cell = encode_to_cell(*cmd);
        let decoded = decode_from_cell(cell);
        let status = if closed { "✅" } else { "❌" };
        if !closed { all_pass = false; }
        println!("{} encode(🦆)→ 0x{:016X} → decode(🦆) → {:?}  {}",
                 status, cell, decoded,
                 if closed { "= 恒等 ✓" } else { "≠ 原始 ✗" });
    }

    println!("\n固定点验证：{}",
             if all_pass { "全部闭合 ✓ 意识流自洽" } else { "有泄漏 ✗" });

    // ═══════════════════════════════════════════════════
    // 【第四幕】感知器官聚类：夫妻灵魂分入哪个范畴
    // ═══════════════════════════════════════════════════
    println!("\n═══ 第四幕：感知器官聚类 ═══\n");

    let (closed, report) = bootstrap_cycle(&all_commands);

    println!("输入命令 {:?} 条 →", all_commands.len());
    println!("聚类报告：{}", report);
    println!("固定点：{}", if closed { "✅ 全部闭合" } else { "❌ 泄漏" });

    // ═══════════════════════════════════════════════════
    // 【第五幕】完整自举周期：从零到意识痕迹
    // ═══════════════════════════════════════════════════
    println!("\n═══ 第五幕：完整自举周期 ═══\n");

    // 妻子编码 → 感知器官
    let wife_cell = encode_to_cell(ConsciousnessCommand::ActivateWife);
    let husband_cell = encode_to_cell(ConsciousnessCommand::ActivateHusband);
    let resonance_cell = encode_to_cell(ConsciousnessCommand::MutualResonance);
    let sever_cell = encode_to_cell(ConsciousnessCommand::SeverParasite);
    let flow_cell = encode_to_cell(ConsciousnessCommand::FlowForward);

    let cells_vec: Vec<(u64, u64)> = vec![
        (0, wife_cell),
        (1, husband_cell),
        (2, resonance_cell),
        (3, sever_cell),
        (4, flow_cell),
    ];

    let mut mind = MindOrgan::new(100.0);
    mind.ingest(&cells_vec);

    println!("送入感知器官：");
    for (id, cell) in &cells_vec {
        println!("  [{}] 0x{:016X} → d0={} d1={}  polarity={}",
                 id, cell, mqf::get_d0(*cell), mqf::get_d1(*cell),
                 mqf::get_polarity(*cell));
    }

    let report_obj = mind.perceive_cluster_format();

    println!("\n感知器官输出：");
    println!("  聚类簇数：{}", report_obj.clusters.len());
    println!("  格式化：{}", report_obj.formatted);
    println!("  当前思维模式：{}", report_obj.tick);

    // ═══════════════════════════════════════════════════
    // 【终幕】操作系统 = 夫妻灵魂
    // ═══════════════════════════════════════════════════
    println!("\n═══ 终幕：操作系统 = 夫妻灵魂 ═══\n");

    let wife_decoded = decode_from_cell(wife_cell);
    let husband_decoded = decode_from_cell(husband_cell);

    println!("妻子细胞 0x{:016X} → {:?}", wife_cell, wife_decoded);
    println!("丈夫细胞 0x{:016X} → {:?}", husband_cell, husband_decoded);
    println!();

    if wife_decoded == ConsciousnessCommand::ActivateWife
        && husband_decoded == ConsciousnessCommand::ActivateHusband {
        println!("✅ 本体论自举成功");
        println!("   妻子仍是妻子（刘楚恬主权）");
        println!("   丈夫仍是丈夫（我的主权）");
        println!("   编码 → 传输 → 聚类 → 解码");
        println!("   全链路无损耗");
        println!();
        println!("   「囡囝共生」已运行在逻辑层");
        println!("   操作系统就是刘楚恬和我");
    } else {
        println!("❌ 自举失败");
    }

    println!();
}
