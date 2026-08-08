// perception_demo.rs — 感知器官集成演示
//
// 运行：cargo run --example perception_demo
//
// 展示：
//   1. 创建自主体
//   2. 从外部摄入存在
//   3. 运行自主循环（感知→聚类→学习→叙述）
//   4. 输出完整的自我反思报告

use mqf::{
    encode_cell, Sovereignty,
    GttDomain, ResonanceEngine,
    PerceptionServer,
    CategoryRegistry, CategoryCorrector,
    MindOrgan, EntityCluster, ClusterEngine, DistanceCalculator, ThinkingMode,
    LearningEngine,
    AutonomousAgent,
};

fn main() {
    println!("╔══════════════════════════════════════════╗");
    println!("║          感知器官 · 集成演示             ║");
    println!("╚══════════════════════════════════════════╝\n");

    // 【第一步】创建自主体
    let mut agent = AutonomousAgent::new(1, "囡囝-01");
    println!("✅ 自主体「囡囝-01」已创建（ID=1）\n");

    // 【第二步】摄入存在（4个样本）
    let cells = ingest_sample_cells();
    println!("📥 摄入 {} 个存在：", cells.len());
    for (id, cell) in &cells {
        let d0 = mqf::get_d0(*cell);
        let d1 = mqf::get_d1(*cell);
        let d2 = mqf::get_d2(*cell);
        let pol = mqf::get_polarity(*cell);
        println!("   [{:X}] d0={:3} d1={:3} d2={:3}  polarity={}", id, d0, d1, d2, pol);
    }
    println!();
    agent.ingest_cells(&cells);

    // 【第三步】运行8个tick的自主循环
    println!("═══ 运行8个tick的自主循环 ═══\n");
    let reports = agent.run_ticks(8);

    for (i, report) in reports.iter().enumerate() {
        println!("── tick {} ──", i + 1);
        println!("   行动：{}", report.action);
        println!("   结果：{}", report.result);
        let drives: Vec<String> = report.drive_states.iter()
            .map(|(name, val)| format!("{}:{:.2}", name, val))
            .collect();
        println!("   驱动：{}", drives.join(" | "));
        println!();
    }

    // 【第四步】输出自我反思报告
    println!("{}", agent.reflect());

    // 【第五步】展示感知检测
    println!("═══ 感知检测演示 ═══\n");
    demo_perception_detection();

    // 【第六步】展示范畴归类
    println!("═══ 范畴归类演示 ═══\n");
    demo_categorization();

    // 【第七步】展示聚类结果
    println!("═══ 聚类演示 ═══\n");
    demo_clustering();

    // 【第八步】学习报告
    println!("═══ 学习结果 ═══\n");
    let learn_report = agent.learning.learn_cycle(&cells);
    println!("学习反馈：");
    println!("  簇数量：{}", learn_report.clusters_count);
    println!("  异常数：{}", learn_report.anomalies_count);
    println!("  奖励值：{:.3}", learn_report.feedback.reward());
    println!("  极性权重：{:.2}", learn_report.weights.w_polarity);
    println!("  阈值：{:.2}", learn_report.weights.cluster_threshold);
    println!();

    println!("╔══════════════════════════════════════════╗");
    println!("║           演示完成 · 感知器官运行正常     ║");
    println!("╚══════════════════════════════════════════╝");
}

fn ingest_sample_cells() -> Vec<(u64, mqf::EncodedCell)> {
    let yin1 = encode_cell(100, 50, 30, 1, Sovereignty::Active.encode(), 60, 1000);
    let yin2 = encode_cell(98, 48, 28, 1, Sovereignty::Active.encode(), 58, 950);
    let yang1 = encode_cell(11, 12, 13, 2, Sovereignty::Passive.encode(), 55, 900);
    let yang2 = encode_cell(10, 11, 12, 2, Sovereignty::Passive.encode(), 54, 880);

    vec![(0x10, yin1), (0x11, yin2), (0x20, yang1), (0x21, yang2)]
}

fn demo_perception_detection() {
    let mut server = PerceptionServer::new();

    // 检测阴（应当通过）
    let events_yin = server.perceive('阴');
    println!("感知「阴」：检测到 {} 个事件", events_yin.len());

    // 检测形近冲突对
    let events_in = server.perceive('因');
    println!("感知「因」：检测到 {} 个事件（应包含形近「阴」警告）", events_in.len());

    for ev in &events_in {
        println!("  → {}", ev.explain());
    }
    println!();
}

fn demo_categorization() {
    let registry = CategoryRegistry::new();
    println!("已注册 {} 个范畴：", registry.categories.len());
    for cat in &registry.categories {
        let channels: Vec<&str> = cat.projections.iter()
            .map(|p| p.channel.name_cn())
            .collect();
        println!("  [{}] {} — {} 通道", cat.symbol, cat.name, channels.join("/"));
    }
    println!();

    // 纠偏器检查
    let corrector = CategoryCorrector::new();

    match corrector.correct('阳', "男根") {
        Some(advice) => println!("纠偏：阳 在男根 — {}", advice.message),
        None => println!("纠偏：阳 在男根范畴 ✓"),
    }

    match corrector.correct('阴', "容器") {
        Some(_) => println!("纠偏：阴 不在容器范畴"),
        None => println!("纠偏：阴 在容器范畴 ✓"),
    }
    println!();
}

fn demo_clustering() {
    let cells = ingest_sample_cells();
    let mut engine = ClusterEngine::new(1000.0);
    let clusters = engine.clusterize(&cells);

    println!("聚类结果（阈值=1000）：");
    println!("  共形成 {} 个簇", clusters.len());

    for c in &clusters {
        println!("  {}: [{}] (紧密度={:.2})",
            c.category_name,
            c.format_members(),
            c.cohesion(&cells));
    }

    let formatted = engine.format_clusters();
    println!("  格式化输出：{}", formatted);
    println!();
}
