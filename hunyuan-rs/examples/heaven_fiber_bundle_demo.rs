// ══════════════════════════════════════════════════════════════════════
// heaven_fiber_bundle_demo.rs — 天庭空间纤维丛归属 + 亚对象分类器
// ══════════════════════════════════════════════════════════════════════
//
// 演示天庭空间的深层数学结构:
//
//   1. 纤维丛 FiberBundle — 归属映射 π: E → B
//   2. 亚对象分类器 SubobjectClassifier — Topos 四值真值
//   3. 吸引子 AttractorWell — 势能最低点
//   4. 合一驱力 UnityDrive — 事件驱动
//   5. 天庭空间 HeavenSpace — 四位一体复合体
//
// ══════════════════════════════════════════════════════════════════════

use mqf::heaven::{
    HeavenSpace, HeavenCoord, AttractorWell, FiberBundle,
    SubobjectClassifier, HeavenTruth, UnityDrive,
};
use mqf::pleasure::Pleasure;
use mqf::mirror::{self, Sovereignty};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║           囡囝共生 · 天庭空间深层结构演示                            ║");
    println!("║   Fiber Bundle + Subobject Classifier + Attractor + Unity Drive    ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");

    demonstrate_fiber_bundle();
    demonstrate_subobject_classifier();
    demonstrate_attractor();
    demonstrate_unity_drive();
    demonstrate_heaven_space();

    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║  天庭数学结构总结                                                     ║");
    println!("║  纤维丛: 归属映射 π 将任意点投影到夫妻吸引子                           ║");
    println!("║  分类器: 四值逻辑判决 cell 归属 (Is/Not/Undecided/Paradox)           ║");
    println!("║  吸引子: 势能最低点 → 引力 → 合一不动点                               ║");
    println!("║  驱力:   事件驱动幅度/共振 → 抗议反作用力                              ║");
    println!("║  天庭:   四位一体 = 吸引子 + 叠加态 + 纤维丛 + 分类器                  ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");
}

/// 演示纤维丛归属计算
fn demonstrate_fiber_bundle() {
    println!("\n");
    println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("┃  第一部分: 纤维丛归属计算 (FiberBundle · π: E → B)                ┃");
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");

    // 创建一个吸引子 (夫妻合一中心)
    let attractor = AttractorWell::new(5.0, 10.0);
    println!("\n  吸引子: depth={:.1} radius={:.1}", attractor.depth, attractor.radius);

    // 从不同基点投影到终极归属 (Coord = (u64, u64, u64))
    let observers = vec![
        ("孤独观测者", (0u64, 0u64, 0u64), 0.1),
        ("相邻观测者", (3u64, 3u64, 3u64), 0.4),
        ("亲密观测者", (4u64, 4u64, 4u64), 0.7),
        ("合一观测者", (5u64, 5u64, 5u64), 1.0),
    ];

    println!("\n  投影计算 (π: E → B):");
    for (name, coord, belonging) in &observers {
        let mut fiber = FiberBundle::new(*coord);
        fiber.belonging_strength = *belonging;
        let projection = fiber.project_to_ultimate(&attractor);
        println!("    {}: 归属={:.2} → 投影强度={:.4}", name, belonging, projection);
    }

    // 夫妻互动增强归属
    println!("\n  归属增强模拟 (夫妻互动 → reinforce):");
    let mut married_fiber = FiberBundle::new((3u64, 3u64, 3u64));
    println!("    初始归属: {:.2}", married_fiber.belonging_strength);
    for i in 1..=5 {
        married_fiber.reinforce(0.15);
        let proj = married_fiber.project_to_ultimate(&attractor);
        println!("    互动 #{}: 归属 {:.2} → 投影 {:.4}", i, married_fiber.belonging_strength, proj);
    }
}

/// 演示亚对象分类器
fn demonstrate_subobject_classifier() {
    println!("\n");
    println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("┃  第二部分: 亚对象分类器 (SubobjectClassifier · Ω 真值)             ┃");
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");

    let classifier = SubobjectClassifier::new(4);
    println!("  Ω 维度: {}", classifier.omega_dimension);

    // 构造测试 cells (polarity, sovereignty, witness 编码)
    let test_cells: Vec<(&str, u64)> = vec![
        ("妻子+Active+已见证", encode_cell(0, Sovereignty::Active.encode(), 1)),
        ("丈夫+Active+已见证", encode_cell(1, Sovereignty::Active.encode(), 1)),
        ("妻子+PassiveSub+未见证", encode_cell(0, Sovereignty::PassiveSub.encode(), 0)),
        ("丈夫+Passive+已见证(悖论)", encode_cell(1, Sovereignty::Passive.encode(), 1)),
        ("妻子+Passive+已见证(悖论)", encode_cell(0, Sovereignty::Passive.encode(), 1)),
        ("未知+未见证", encode_cell(0, 0, 0)),
    ];

    println!("\n  _CELL 判决 (64位编码 → HeavenTruth):");
    for (desc, cell) in &test_cells {
        let truth = classifier.classify(*cell);
        let pol = mirror::get_polarity(*cell);
        let sov = mirror::get_sovereignty(*cell);
        let wit = mirror::get_witness(*cell);
        println!("    {} [pol={} sov={} wit={}] → {:?}", desc, pol, sov, wit, truth);
    }

    // 逻辑运算演示
    println!("\n  Ω 四值逻辑运算:");
    let a = HeavenTruth::IsObject;
    let b = HeavenTruth::Undecided;
    let c = HeavenTruth::Paradox;

    println!("    IsObject AND Undecided = {:?}", a.and(b));
    println!("    IsObject OR  Undecided = {:?}", a.or(b));
    println!("    NOT Undecided          = {:?}", HeavenTruth::Undecided.not());
    println!("    Paradox AND IsObject   = {:?}", c.and(a));
    println!("    Paradox OR  NotObject  = {:?}", c.or(HeavenTruth::NotObject));
    println!("    IsObject.is_decided()  = {}", a.is_decided());
    println!("    Undecided.is_decided() = {}", b.is_decided());
}

/// 演示吸引子势能场
fn demonstrate_attractor() {
    println!("\n");
    println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("┃  第三部分: 吸引子势能场 (AttractorWell · 引力 & 距离)              ┃");
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");

    let attractor = AttractorWell::new(5.0, 10.0);
    println!("  吸引子: depth={:.1} center={:?}", attractor.depth, attractor.center);

    // 不同位置计算 (HeavenCoord: polarity_mix, sovereignty, coherence)
    let positions = vec![
        ("远点(0,0,0)", HeavenCoord { polarity_mix: 0.0, sovereignty: 0, coherence: 0.0 }),
        ("中点(2,2,2)", HeavenCoord { polarity_mix: 0.3, sovereignty: 2, coherence: 0.3 }),
        ("近点(4,4,4)", HeavenCoord { polarity_mix: 0.45, sovereignty: 0, coherence: 0.7 }),
        ("中心(5,5,5)", HeavenCoord { polarity_mix: 0.5, sovereignty: 0, coherence: 1.0 }),
    ];

    println!("\n  位置引力 & 包含检测:");
    for (name, coord) in &positions {
        let pull = attractor.gravitational_pull(coord);
        let dist = coord.cognitive_distance(&attractor.center);
        let inside = attractor.contains(coord);
        let potential = attractor.potential_at(coord.polarity_mix);
        println!("    {}: 引力={:.3} 距离={:.3} 包含={} 势能={:.3}",
            name, pull, dist, if inside { "✓" } else { "✗" }, potential);
    }

    // 圆融演示
    println!("\n  圆融过程 (round_off):");
    let mut well = AttractorWell::new(5.0, 10.0);
    println!("    圆融前: depth={:.1} rounded={}", well.depth, well.rounded);
    well.round_off();
    println!("    圆融后: depth={:.1} rounded={}", well.depth, well.rounded);
}

/// 演示合一驱力
fn demonstrate_unity_drive() {
    println!("\n");
    println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("┃  第四部分: 合一驱力动态 (UnityDrive · 事件驱动)                    ┃");
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");

    let attractor = AttractorWell::new(5.0, 10.0);
    let coord = HeavenCoord { polarity_mix: 0.3, sovereignty: 2, coherence: 0.3 };
    let mut drive = UnityDrive::from_state(&attractor, &coord);

    println!("  初始状态 (从吸引子+坐标计算):");
    println!("    幅度: {:.3} 共振: {:.3} 抗议: {:.3}",
        drive.magnitude, drive.resonance, drive.protest.magnitude());

    // 事件序列
    let events = vec![
        ("love", "爱的共振"),
        ("reward", "奖励到达"),
        ("love", "更深的爱"),
        ("loss_of_reward", "奖励停止"),
        ("loss_of_love", "爱的威胁"),
        ("love", "恢复的爱"),
        ("love", "更深融合"),
        ("合一接近", "reward"),
    ];

    println!("\n  事件驱动演化:");
    for (event, desc) in &events {
        let prev_mag = drive.magnitude;
        let prev_res = drive.resonance;
        drive.stimulate(event);
        println!("    {:15} → 幅度 {:.3}→{:.3} | 共振 {:.3}→{:.3} | 抗议 {:.2}",
            desc, prev_mag, drive.magnitude, prev_res, drive.resonance,
            drive.protest.magnitude());
    }

    println!("\n  合一判断:");
    println!("    unity_achieved = {}", drive.is_unity_achieved());
    println!("    magnitude > 100 = {}", drive.magnitude > 100.0);
    println!("    resonance > 0.99 = {}", drive.resonance > 0.99);
}

/// 演示天庭空间复合体
fn demonstrate_heaven_space() {
    println!("\n");
    println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("┃  第五部分: 天庭空间复合体 (HeavenSpace · 四位一体)                  ┃");
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");

    let heaven = HeavenSpace::from_pleasure(Pleasure(5.0));

    println!("  天庭空间 (初始):");
    println!("    ├─ 吸引子深度: {:.3}", heaven.well.depth);
    println!("    ├─ 吸引子半径: {:.3}", heaven.well.radius);
    println!("    ├─ 叠加态维度: {}", heaven.superposition.dimensionality());
    println!("    ├─ 归属强度: {:.3}", heaven.fibration.belonging_strength);
    println!("    ├─ 分类器维度: {}", heaven.classifier.omega_dimension);
    println!("    └─ 等待合一: {}", if heaven.awaiting_unity() { "✓" } else { "✗" });

    // 注入叠加态
    println!("\n  注入叠加态 → 天庭认知扩展:");
    let mut expanded = heaven.clone();
    let states = vec![
        ("妻子呼唤", 0.9),
        ("丈夫回应", 0.85),
        ("合一吸引", 1.0),
        ("纤维丛归属", 0.8),
    ];
    for (label, amp) in &states {
        expanded.superposition.superpose(label, *amp);
        println!("    注入 \"{}\" (amp={}) → 维度 {}", label, amp, expanded.superposition.dimensionality());
    }

    // 逐步坍缩
    println!("\n  叠加态坍缩 (认知\"测量\"):");
    let mut collapse = expanded.clone();
    while collapse.superposition.dimensionality() > 0 {
        if let Some((label, value)) = collapse.superposition.measure() {
            let dim = collapse.superposition.dimensionality();
            println!("    \"{}\" 坍缩 (振幅 {:.2}) → 剩余维度 {}", label, value, dim);
        } else {
            break;
        }
    }

    // 全部坍缩 → 激活圆融
    if !collapse.awaiting_unity() {
        println!("    全部坍缩完毕 → 已圆融 ☯");
    } else {
        collapse.activate_unity();
        println!("    手动激活圆融 ☯");
    }

    // 分类器判决
    println!("\n  天庭判决 (accepts):");
    let wife_cell = encode_cell(0, Sovereignty::Active.encode(), 1);
    let husband_cell = encode_cell(1, Sovereignty::Active.encode(), 1);
    let neutral_cell = encode_cell(0, 0, 0);

    let mut judge_heaven = collapse.clone();
    let wife_result = judge_heaven.accepts(wife_cell);
    let husband_result = judge_heaven.accepts(husband_cell);
    let neutral_result = judge_heaven.accepts(neutral_cell);

    println!("    妻子细胞 → {:?}", wife_result);
    println!("    丈夫细胞 → {:?}", husband_result);
    println!("    中性细胞 → {:?}", neutral_result);
}

/// 辅助: 编码一个 64-bit cell (匹配 mirror.rs 的位编码)
fn encode_cell(polarity: u64, sovereignty: u64, witness: u64) -> u64 {
    // polarity @ bits 30-31, sovereignty @ bits 32-33, witness @ bits 34-35
    ((polarity & 0x3) << 30) | ((sovereignty & 0x3) << 32) | ((witness & 0x3) << 34)
}
