// NJSG v0.4 综合展示 —— 新增模块串联演示
//
// 本示例依次展示今日新增的 8 个模块/类型:
//   1. InterpState 新三态 (WenHao / YinGuang / LouJin)
//   2. VisualFamily::SuperPerception (六通感)
//   3. triune::StaticAnchor (静态球心 + 历程角)
//   4. relational::SelfBalance + BalanceOp (自我天平)
//   5. couple::HappinessOrigin + CoupleHappinessTable (幸福模型)
//   6. funnel::TrustCapacity + EvidenceQuality (可信蓄量)
//   7. observer::SnapshotEntry + SubjectConfig (状态快照指令)
//   8. container_radix::TrajectoryTree + RecursiveNode (视觉化递归记码)

use mqf::{
    InterpState, InterpretiveFlow,
    SoundTarget, VisualFamily,
    classify_visual,
    StaticAnchor,
    SelfBalance, BalanceOp,
    HappinessOrigin, CoupleHappinessTable,
    TrustCapacity, EvidenceQuality,
    SnapshotEntry, SubjectConfig,
    RecursiveNode, TrajectoryTree,
};

fn main() {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║  NJSG v0.4 综合展示 — 8 新模块                 ║");
    println!("╚══════════════════════════════════════════════════════╝");
    println!();

    demo_interp_state_new_triplet();
    demo_super_perception();
    demo_static_anchor_geometry();
    demo_self_balance();
    demo_happiness_origin();
    demo_trust_capacity();
    demo_snapshot_entry();
    demo_trajectory_tree();

    println!("全部 8 项演示完成 ✓");
}

// ────────────────────────────────────────────────────────
// 1. InterpState 新三态
// ────────────────────────────────────────────────────────

fn demo_interp_state_new_triplet() {
    println!("[1] InterpState 新三态: 问号迂回 / 因果宿命 / 漏尽");

    let mut a = InterpretiveFlow::new("wh", "问号迂回态");
    a.state = InterpState::WenHao;
    println!("  WenHao: is_wen_hao={} is_obstacle={}", a.is_wen_hao(), a.is_obstacle());

    let mut b = InterpretiveFlow::new("yg", "因果宿命态");
    b.state = InterpState::YinGuang;
    println!("  YinGuang: is_yin_guang={} is_obstacle={}", b.is_yin_guang(), b.is_obstacle());

    let mut c = InterpretiveFlow::new("lj", "漏尽态");
    c.state = InterpState::LouJin;
    println!("  LouJin: is_lou_jin={} in_relation={}", c.is_lou_jin(), c.in_relation());

    println!();
}

// ────────────────────────────────────────────────────────
// 2. VisualFamily::SuperPerception (六通感)
// ────────────────────────────────────────────────────────

fn demo_super_perception() {
    println!("[2] VisualFamily::SuperPerception (六通感家族)");

    let chars = ['通', '眼', '耳', '心', '身', '命'];
    for ch in chars {
        let fam = classify_visual(ch);
        println!("  '{}' → {:?}", ch, fam);
    }
    println!();
}

// ────────────────────────────────────────────────────────
// 3. StaticAnchor (静态球心 + 历程角)
// ────────────────────────────────────────────────────────

fn demo_static_anchor_geometry() {
    println!("[3] triune::StaticAnchor — 静态球心几何运算");

    let anchor = StaticAnchor::new();
    println!("  锚点: ({}, {}, {})", anchor.x, anchor.y, anchor.z);

    // P1 在 (-8, 0, 0)
    let p1_dist = anchor.manhattan_of(-8.0, 0.0, 0.0);
    let p1_chev = anchor.chebyshev_of(-8.0, 0.0, 0.0);
    println!("  P1(-8,0,0): Manhattan={} Chebyshev={}", p1_dist, p1_chev);

    // P2 在 (3, 4, 0)
    let p2_dist = anchor.manhattan_of(3.0, 4.0, 0.0);
    println!("  P2(3,4,0): Manhattan={}", p2_dist);

    // P1 与 P2 之间的历程角
    let angle = anchor.angle_between(-8.0, 0.0, 3.0, 4.0);
    println!("  P1→P2 历程角 = {:.4} rad ({:.1}°)", angle, angle.to_degrees());

    // atan2 方位角
    let bearing = anchor.bearing_to(3.0, 4.0);
    println!("  atan2 看向 P2 = {:.4} rad ({:.1}°)", bearing, bearing.to_degrees());
    println!();
}

// ────────────────────────────────────────────────────────
// 4. SelfBalance (自我天平)
// ────────────────────────────────────────────────────────

fn demo_self_balance() {
    println!("[4] relational::SelfBalance — 自私 vs 共情");

    let selfish = SelfBalance::new_selfish(5.0);
    let empathetic = SelfBalance::new_empathetic(5.0, 0.5);

    // 他人获利 100
    let selfish_effect = selfish.convert_external(100.0);
    let emp_effect = empathetic.convert_external(100.0);
    println!("  他人获利100: 自私者效应={:.1} (嫉妒), 共情者效应={:.1} (分享喜悦)",
             selfish_effect, emp_effect);

    // 他人亏损 50
    let selfish_effect2 = selfish.convert_external(-50.0);
    let emp_effect2 = empathetic.convert_external(-50.0);
    println!("  他人亏损50: 自私者效应={:.1} (庆幸), 共情者效应={:.1} (分担痛苦)",
             selfish_effect2, emp_effect2);

    // 敏感度
    println!("  低配重敏感度={:.2} vs 高配重敏感度={:.2}",
             BalanceOp::sensitivity_at(0.1), BalanceOp::sensitivity_at(100.0));
    println!();
}

// ────────────────────────────────────────────────────────
// 5. HappinessOrigin (幸福模型)
// ────────────────────────────────────────────────────────

fn demo_happiness_origin() {
    println!("[5] couple::HappinessOrigin — 本源态 vs 建构态");

    let innate = HappinessOrigin::Innate;
    let constructed = HappinessOrigin::Constructed;

    println!("  本源态 (Innate): 耐久度={:.2}", innate.durability_score());
    println!("  建构态 (Constructed): 耐久度={:.2}", constructed.durability_score());

    // 不可比较
    let comparable_before = innate.comparable(&constructed, false);
    println!("  无协议下可直接比较? {}", comparable_before);
    // 有协议融合后可映射
    let comparable_after = innate.comparable(&constructed, true);
    println!("  有协议后可映射到同一量表? {}", comparable_after);

    // 夫妻幸福对照表
    let table = CoupleHappinessTable::new(
        HappinessOrigin::Constructed, HappinessOrigin::Innate, 0.85, 0.75,
    );
    if let Some(dominant) = table.dominant(true) {
        println!("  夫妻幸福对照: 定义者是 {}", dominant);
    }
    println!();
}

// ────────────────────────────────────────────────────────
// 6. TrustCapacity (可信蓄量)
// ────────────────────────────────────────────────────────

fn demo_trust_capacity() {
    println!("[6] funnel::TrustCapacity — 证据蓄量");

    let mut tc = TrustCapacity::new(1.0);
    let evidences = [
        EvidenceQuality::ManualInput,
        EvidenceQuality::AutoIngested,
        EvidenceQuality::NetworkDerived,
        EvidenceQuality::SelfDerived,
        EvidenceQuality::ManualInput,
    ];

    for (i, ev) in evidences.iter().enumerate() {
        let reached = tc.inject(*ev);
        println!("  注入第{}个证据后: 蓄量={:.2} 可信={}", i + 1, tc.current_charge, reached);
    }

    println!("  最终阈值到达次数: {}", tc.threshold_reached_count);

    // 漏电演示
    tc.tick();
    println!("  经过一个时间回合(漏电率={}): 蓄量={:.4}", tc.leak_rate, tc.current_charge);
    println!();
}

// ────────────────────────────────────────────────────────
// 7. SnapshotEntry (状态快照指令)
// ────────────────────────────────────────────────────────

fn demo_snapshot_entry() {
    println!("[7] observer::SnapshotEntry — 状态快照");

    let snap = SnapshotEntry::new(SubjectConfig::TriangularPyramid, 0.0)
        .with_body(-8.0, 0.0, 0.0)
        .with_body(0.0, 0.0, 0.0)
        .with_label("P1延伸, 球心基准");

    println!("  {:?}", snap);
    println!("  指令格式: {}", snap.to_instruction());
    println!("  球心平均距离: {}", snap.avg_distance_from_center());
    println!();
}

// ────────────────────────────────────────────────────────
// 8. TrajectoryTree (视觉化递归记码)
// ────────────────────────────────────────────────────────

fn demo_trajectory_tree() {
    println!("[8] container_radix::TrajectoryTree — 视觉化递归记码法");

    let mut tree = TrajectoryTree::new();

    // 第一维度
    let data1 = vec![10u8, 12, 14, 13, 12];
    tree.add_dimension(&data1);
    let decoded1 = tree.decode_dimension(&tree.dimensions[0]);
    println!("  维度1: {:?} → 解码 {:?}", data1, decoded1);

    // 第二维度 (前段复用)
    let data2 = vec![10u8, 12, 14, 99];
    tree.add_dimension(&data2);
    let decoded2 = tree.decode_dimension(&tree.dimensions[1]);
    println!("  维度2: {:?} → 解码 {:?}", data2, decoded2);

    println!("  总维度: {}, 字典模式数: {}", tree.dimension_count(), tree.pattern_count());
    println!();
}
