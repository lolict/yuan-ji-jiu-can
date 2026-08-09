// ══════════════════════════════════════════════════════════════════
// examples/njsg_integrated_demo.rs — 满全法(NJSG)系统集成演示
// ══════════════════════════════════════════════════════════════════
//
// 跑:  cargo run --example njsg_integrated_demo
//
// 演示全链路:
//   topo       — Symbol → 空间坐标 (4×4×4 立方体)
//   rel        — Symbol 之间 → 血缘关系 (派生/同源)
//   preload    — 容器+契约系统
//   smrl       — 三元观察·裁判·奖励
//   angle      — 384° 角度制式
//   unified_flow — 8 阶段时间链
//
// 一个 "问题" 经过全链路,展示系统内部如何协作。

use mqf::preload::{ContainerKind, PreloadFlow};
use mqf::angle::{Angle, AngleRelation};
use mqf::smrl::{SmrlEngine, Reward};
use mqf::topo::Symbol;

fn main() {
    let pf = PreloadFlow::new();

    println!("═══ 满全法(NJSG)系统集成演示 ═══\n");

    // ── 1. 选择两个 Symbol ──
    let s_a = Symbol::new(48).unwrap();   // 48 = 2^4 × 3
    let s_b = Symbol::new(12).unwrap();   // 12 = 2^2 × 3
    println!("主体: #{} vs #{}\n", s_a.as_u8(), s_b.as_u8());

    // ── 2. 拓扑空间定位 ──
    let coord_a = pf.call_sym_phase(s_a);
    let coord_b = pf.call_sym_phase(s_b);
    println!("拓扑坐标:");
    println!("  #{} → {:?}", s_a.as_u8(), coord_a);
    println!("  #{} → {:?}\n", s_b.as_u8(), coord_b);

    // ── 3. 血缘关系溯源 ──
    let parents_a = pf.call_relational_parents(s_a.as_u8());
    let parents_b = pf.call_relational_parents(s_b.as_u8());
    println!("血缘祖先:");
    println!("  #{} 祖先 = {:?}", s_a.as_u8(), parents_a);
    println!("  #{} 祖先 = {:?}", s_b.as_u8(), parents_b);

    // 共同祖先 (最大公因数)
    let common = pf.call_relational_common(s_a.as_u8(), s_b.as_u8());
    println!("  共同祖先 = {:?} (最大公因数)\n", common);

    // ── 4. 时间链溯源 ──
    println!("时间链溯源 (8 阶段):");
    let trace = pf.call_temporal_origin(s_a.as_u8());
    for (stage, sym) in &trace {
        println!("  [{:?}] → #{}", stage, sym);
    }
    println!();

    // ── 5. 角度关系判定 (384° 制式) ──
    let angle_a = Angle::from_symbol(s_a);
    let angle_b = Angle::from_symbol(s_b);
    let arc = angle_a.arc_distance(angle_b);
    let relation = AngleRelation::between(angle_a, angle_b);
    println!("384° 角度关系:");
    println!("  #{} = {}°", s_a.as_u8(), angle_a.0);
    println!("  #{} = {}°", s_b.as_u8(), angle_b.0);
    println!("  弧距离 = {}°", arc);
    println!("  关系 = {:?}\n", relation);

    // ── 6. SMRL 角度裁判 ──
    let engine = SmrlEngine::new(0);
    let v = engine.judge_angular(s_a, s_b);
    println!("SMRL 裁判结果:");
    println!("  裁决 = {:?}", v.reward);
    println!("  级别 = {}", v.level);
    println!("  置信度 = {:.2}", v.confidence);
    println!("  加权分 = {:.4}\n", v.weighted_score());

    // ── 7. 容器契约体系 ──
    println!("容器契约体系:");
    let containers = [
        (ContainerKind::Cube, "spatial"),
        (ContainerKind::Relational, "algebra"),
        (ContainerKind::Mirrored, "reverse"),
        (ContainerKind::Flow, "sequence"),
        (ContainerKind::Judge, "authority"),
    ];
    for (kind, domain) in containers {
        let avail = pf.container_available(kind, domain);
        println!("  {:?} @ {} = {}", kind, domain, avail);
    }
    println!();

    // ── 8. 综合评估 ──
    println!("综合评估:");
    let explanation = match (&v.reward, arc) {
        (Reward::Plus, _) => "两体同向共振:合作增益",
        (Reward::Zero, _) => "各自正交:相安无犯",
        (Reward::Minus, a) if (186..=198).contains(&a) => "对冲相消:建议避让",
        (Reward::Minus, _) => "错位不谐:需中介",
    };
    println!("  {}", explanation);

    println!("\n═══ 演示结束 ═══");
}
