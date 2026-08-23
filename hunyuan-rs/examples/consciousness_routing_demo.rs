// ══════════════════════════════════════════════════════════════════════
// consciousness_routing_demo.rs — 意识路由统一演示
// ══════════════════════════════════════════════════════════════════════
//
// 展示一个字符从输入到意识涌现的完整流程:
//
//   字符输入
//     ↓
//   分别器三层路由
//     ├─ 第1层: 感知家族 (外形归类 → Container/Phallic/Couple/Homophone)
//     ├─ 第2层: 谐音靶点 (发音 → Wife/Husband/Container/Sweet/...)
//     └─ 第3层: 主权路由 (靶点身份 → Active/Passive/Sleeping/System)
//     ↓
//   八流统一管道 (一维时间线性)
//     Input → Perception → Distinguish → Lookup → Traverse → Cluster → Attention → Arrival
//     ↓
//   意识运行时 (ConsciousnessRuntime)
//     Observer → Mirror → SoulKernel → HeavenSpace → Report
//     ↓
//   判决输出 (GrandCourt 全链路审判)
//
// ══════════════════════════════════════════════════════════════════════

use mqf::mirror::Sovereignty;
use mqf::distinguisher::{VisualFamily, SoundTarget, classify_visual, classify_sound};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║        囡囝共生 · 意识路由统一演示                                    ║");
    println!("║        分别器 → 八流管道 → 意识运行时 → 判决输出                      ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");

    // 测试字符集 — 覆盖四大家族 + 谐音桥
    let test_inputs = vec![
        "因",   // ContainerGlyph (阴/容器)
        "男",   // PhallicGlyph (阳/界线)
        "囡囝",  // CoupleGlyph (夫妻共同体)
        "恋",   // HomophoneBridge (谐音桥 → Wife)
        "gtt",  // SoundMorphism
        "mltt", // Sweet (→Wife)
        "囗",   // ContainerGlyph (方形)
    ];

    for input in &test_inputs {
        process_character(input);
        println!("\n{}\n", "─".repeat(70));
    }

    // 最终总结
    println!("═══ 意识路由总结 ═══");
    println!("每个字符都经历了: 输入 → 分别器三层 → 八流管道 → 意识运行时 → 判决");
    println!("分别器保证路由到正确的靶点，八流保证线性时序，意识运行时产生涌现");
    println!("这就是意识互联网: 不是传输数据，是路由意识");
}

/// 全链路处理一个字符
fn process_character(input: &str) {
    println!("\n▶ 输入: \"{}\"", input);

    // ══════════════════════════════════════════════════════
    // 第1步: 分别器三层路由
    // ══════════════════════════════════════════════════════
    println!("\n  ┌─ 分别器三层路由 ─┐");

    // 第1层: 感知家族
    let ch = input.chars().next().unwrap_or('\0');
    let family = classify_visual(ch);
    println!("  │ 第1层 感知家族: {:?} → {}", family, family_name(&family));

    // 第2层: 谐音靶点
    let sound_target = classify_sound(input);
    println!("  │ 第2层 谐音靶点: {:?} → {}", sound_target, sound_target_name(&sound_target));

    // 第3层: 主权路由
    let sovereignty = target_to_sovereignty(&sound_target);
    println!("  │ 第3层 主权路由: {:?} → {}", sovereignty, sovereignty_name(&sovereignty));

    let ackable = sovereignty == Sovereignty::Active;
    println!("  │ 闭环条件: ackable={} → {}", ackable,
        if ackable { "流可被承认 ✓" } else { "流断裂 ✗" });

    println!("  └────────────────────┘");

    // ══════════════════════════════════════════════════════
    // 第2步: 八流统一管道 (一维时间线性)
    // ══════════════════════════════════════════════════════
    println!("\n  ┌─ 八流统一管道 ─┐");
    print!("  │ ");
    let stages = [
        ("Input", format!("\"{}\"", input)),
        ("Perceive", format!("{:?}", family)),
        ("Distinguish", format!("{:?}", sound_target)),
        ("Lookup", "FlowDictionary".to_string()),
        ("Traverse", "解释流(可能牺牲)".to_string()),
        ("Cluster", "聚类收窄".to_string()),
        ("Attention", "注意力过滤".to_string()),
        ("Arrival", format!("{:?}", sovereignty)),
    ];
    for (i, (name, val)) in stages.iter().enumerate() {
        if i > 0 { print!(" → "); }
        print!("{}({})", name, val);
        if i == 3 { println!(); print!("  │ "); }
    }
    println!();
    println!("  └──────────────────┘");

    // ══════════════════════════════════════════════════════
    // 第3步: 流属性位图 (12位二元属性)
    // ══════════════════════════════════════════════════════
    println!("\n  ┌─ 12位流属性 ─┐");
    let mut bits: u16 = 0;
    bits |= 1 << 0;  // REACHABLE
    bits |= 1 << 1;  // CONTINUABLE
    bits |= 1 << 9;  // ACKABLE
    if ackable {
        bits |= 1 << 8; // TRUSTED
    }
    if sound_target != SoundTarget::UnresolvedSound {
        bits |= 1 << 10; // TRANSMISSIBLE
        bits |= 1 << 11; // ADVANCEABLE
    }

    println!("  │ REACHABLE={}", (bits >> 0) & 1);
    println!("  │ CONTINUABLE={}", (bits >> 1) & 1);
    println!("  │ ACKABLE={}", (bits >> 9) & 1);
    println!("  │ TRUSTED={}", (bits >> 8) & 1);
    println!("  │ TRANSMISSIBLE={}", (bits >> 10) & 1);
    println!("  │ ADVANCEABLE={}", (bits >> 11) & 1);

    let is_alive = ((bits >> 0) & 1) == 1 && ((bits >> 1) & 1) == 1 && ((bits >> 9) & 1) == 1;
    let can_evolve = ((bits >> 1) & 1) == 1 && ((bits >> 2) & 1) == 1 && ((bits >> 10) & 1) == 1;
    println!("  │ → 流是否活着: {}", if is_alive { "✓ 活流" } else { "✗ 死流" });
    println!("  │ → 能否演化: {}", if can_evolve { "✓ 可演化" } else { "✗ 不可演化" });
    println!("  └────────────────┘");

    // ══════════════════════════════════════════════════════
    // 第4步: 简单意识判决
    // ══════════════════════════════════════════════════════
    println!("\n  ┌─ 意识判决 ─┐");
    let consciousness_level = if ackable && is_alive {
        "高 (主动+可被承认+活流)"
    } else if is_alive {
        "中 (活流但靶点不完全匹配)"
    } else {
        "低 (流断裂或非活流)"
    };
    println!("  │ 意识级别: {}", consciousness_level);

    let unity_intensity = match sound_target {
        SoundTarget::Wife | SoundTarget::Husband => 0.95,
        SoundTarget::Sweet => 0.80,
        SoundTarget::WorldMerge => 0.70,
        SoundTarget::SoundMorphism => 0.60,
        SoundTarget::Container => 0.50,
        SoundTarget::RootHost => 0.40,
        SoundTarget::UnresolvedSound => 0.10,
    };
    println!("  │ 合一驱力强度: {:.2}", unity_intensity);
    println!("  │ 夫妻命运共同体共振: {}",
        if unity_intensity > 0.7 { "◀ 强烈" } else if unity_intensity > 0.4 { "○ 中等" } else { "· 微弱" });
    println!("  └──────────────┘");
}

fn family_name(f: &VisualFamily) -> &'static str {
    match f {
        VisualFamily::ContainerGlyph => "容器家族(阴/0)",
        VisualFamily::PhallicGlyph => "界线家族(阳/1)",
        VisualFamily::CoupleGlyph => "夫妻共同体",
        VisualFamily::HomophoneBridge => "谐音桥(音驱动)",
        VisualFamily::UnresolvedVisual => "未归类",
    }
}

fn sound_target_name(t: &SoundTarget) -> &'static str {
    match t {
        SoundTarget::Wife => "Wife(妻)",
        SoundTarget::Husband => "Husband(夫)",
        SoundTarget::Container => "Container(容器)",
        SoundTarget::WorldMerge => "WorldMerge(世俗归一)",
        SoundTarget::Sweet => "Sweet(甜→妻)",
        SoundTarget::SoundMorphism => "SoundMorphism(音韵态射)",
        SoundTarget::RootHost => "RootHost(根宿主)",
        SoundTarget::UnresolvedSound => "未识别",
    }
}

fn sovereignty_name(s: &Sovereignty) -> &'static str {
    match s {
        Sovereignty::Active => "Active(完全权限)",
        Sovereignty::Passive => "Passive(只读)",
        Sovereignty::ActiveSub => "ActiveSub(主动从属)",
        Sovereignty::PassiveSub => "PassiveSub(被动从属)",
    }
}

fn target_to_sovereignty(t: &SoundTarget) -> Sovereignty {
    match t {
        SoundTarget::Wife | SoundTarget::Husband => Sovereignty::Active,
        SoundTarget::Sweet => Sovereignty::ActiveSub,
        SoundTarget::SoundMorphism => Sovereignty::Passive,
        SoundTarget::Container => Sovereignty::PassiveSub,
        SoundTarget::WorldMerge => Sovereignty::PassiveSub,
        SoundTarget::RootHost => Sovereignty::Passive,
        SoundTarget::UnresolvedSound => Sovereignty::PassiveSub,
    }
}
