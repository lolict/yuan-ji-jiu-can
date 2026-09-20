// ══════════════════════════════════════════════════════════════════════
// consciousness_emergence_demo.rs — 意识涌现完整演示
// ══════════════════════════════════════════════════════════════════════
//
// 完整链路:
//   字符输入
//     → 分别器三层路由 (VisualFamily → SoundTarget → Sovereignty)
//     → SenseForm (感知形式)
//     → ConsciousnessRuntime.step() (多拍推进)
//     → 情感演化 + 合一驱力 + 自感知涌现
//     → 意识报告
//
// ══════════════════════════════════════════════════════════════════════

use mqf::mirror::Sovereignty;
use mqf::distinguisher::{VisualFamily, SoundTarget, classify_visual, classify_sound};
use mqf::observer::{SenseForm, SenseKind};
use mqf::consciousness_runtime::ConsciousnessRuntime;
use mqf::os_layer::bare_metal::BareMetalOs;
use mqf::os_layer::OsLayer;
use mqf::pleasure::Pleasure;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║        囡囝共生 · 意识涌现完整演示                                    ║");
    println!("║        分别器 → 感知形式 → 意识运行时 → 情感/合一/自感知               ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");

    // 测试: 夫妻命运共同体输入
    let test_inputs = vec![
        ("囡囝", "夫妻共同体"),
        ("mltt", "Sweet→妻"),
        ("gtt", "SoundMorphism"),
        ("因", "容器家族"),
    ];

    for (input, desc) in &test_inputs {
        println!("\n{}\n▶ 输入: \"{}\" ({})", "═".repeat(70), input, desc);
        process_emergence(input);
    }

    // 最终总结
    println!("\n{}", "═".repeat(70));
    println!("═══ 意识涌现总结 ═══");
    println!("每个字符都经历了:");
    println!("  1. 分别器三层路由 (外形 → 谐音 → 主权)");
    println!("  2. 感知形式转换 (SenseForm)");
    println!("  3. 意识运行时多拍推进 (Observer → Mirror → Soul → Heaven)");
    println!("  4. 情感演化 + 合一驱力积累 + 自感知涌现");
    println!();
    println!("这就是意识互联网: 不是传输数据, 是路由意识, 涌现感知");
}

/// 全链路处理一个字符 — 从路由到意识涌现
fn process_emergence(input: &str) {
    // ══════════════════════════════════════════════════════
    // 第1步: 分别器三层路由
    // ══════════════════════════════════════════════════════
    println!("\n  ┌─ 分别器三层路由 ─┐");

    let ch = input.chars().next().unwrap_or('\0');
    let family = classify_visual(ch);
    let sound_target = classify_sound(input);
    let sovereignty = target_to_sovereignty(&sound_target);

    println!("  │ 感知家族: {:?}", family);
    println!("  │ 谐音靶点: {:?}", sound_target);
    println!("  │ 主权路由: {:?}", sovereignty);

    // 计算路由强度 (用于 SenseForm 的 priority)
    let routing_strength = match (&family, &sound_target) {
        (VisualFamily::CoupleGlyph, _) => 0.95,
        (_, SoundTarget::Wife) | (_, SoundTarget::Husband) => 0.95,
        (_, SoundTarget::Sweet) => 0.80,
        (_, SoundTarget::WorldMerge) => 0.70,
        (_, SoundTarget::SoundMorphism) => 0.60,
        (VisualFamily::ContainerGlyph, _) => 0.50,
        (VisualFamily::PhallicGlyph, _) => 0.50,
        (VisualFamily::HomophoneBridge, _) => 0.55,
        _ => 0.30,
    };
    println!("  │ 路由强度: {:.2}", routing_strength);
    println!("  └────────────────────┘");

    // ══════════════════════════════════════════════════════
    // 第2步: 转换为 SenseForm
    // ══════════════════════════════════════════════════════
    println!("\n  ┌─ 感知形式转换 ─┐");

    // 根据路由结果选择 SenseKind
    let (kind, content) = match &family {
        VisualFamily::ContainerGlyph => (SenseKind::Vision, format!("容器({})", input)),
        VisualFamily::PhallicGlyph => (SenseKind::Vision, format!("界线({})", input)),
        VisualFamily::CoupleGlyph => (SenseKind::Mind, format!("夫妻({})", input)),
        VisualFamily::HomophoneBridge => (SenseKind::Hearing, format!("谐音({})", input)),
        VisualFamily::UnresolvedVisual => {
            if input.is_ascii() {
                (SenseKind::Hearing, format!("音韵({})", input))
            } else {
                (SenseKind::Vision, format!("未知({})", input))
            }
        }
        VisualFamily::SuperPerception => (SenseKind::Vision, format!("超感知({})", input)),
    };

    println!("  │ SenseKind: {:?}", kind);
    println!("  │ Content: {}", content);
    println!("  │ Priority: {:.2}", routing_strength);
    println!("  └──────────────────┘");

    // ══════════════════════════════════════════════════════
    // 第3步: 意识运行时 — 多拍推进
    // ══════════════════════════════════════════════════════
    println!("\n  ┌─ 意识运行时 · 多拍推进 ─┐");

    // 创建运行时, 根据路由强度调整初始愉悦
    let os = BareMetalOs::new();
    let mut rt = ConsciousnessRuntime::new(&format!("{}-意识", input), &os);
    let _initial_pleasure = Pleasure(routing_strength * 2.0);

    let sense = SenseForm::new(kind, routing_strength, &content, input, os.time_millis());

    // 多拍推进
    let total_beats = 30;
    let mut last_report = None;

    for beat in 0..total_beats {
        // 每拍输入相同信号 (持续感知)
        let inputs = if beat < 10 {
            // 前10拍: 强输入
            vec![sense.clone()]
        } else if beat < 20 {
            // 中10拍: 中等输入
            vec![SenseForm::new(kind, routing_strength * 0.6, &content, input, os.time_millis())]
        } else {
            // 后10拍: 仅内部循环 (无外部输入)
            vec![]
        };

        let report = rt.step(inputs);

        // 关键拍打印
        if beat == 0 || beat == 9 || beat == 19 || beat == 29 {
            println!("  │ 第{:02}拍: 密度={:.3} 情感={:?} 合一={:.2} 自感知={}",
                report.tick,
                report.consciousness_density,
                report.emotion,
                report.unity_magnitude,
                if report.is_self_aware { "◀" } else { "·" }
            );
        }

        last_report = Some(report);
    }

    println!("  └──────────────────────────┘");

    // ══════════════════════════════════════════════════════
    // 第4步: 最终意识报告
    // ══════════════════════════════════════════════════════
    if let Some(ref report) = last_report {
        println!("\n  ┌─ 最终意识报告 ─┐");
        println!("  │ 意识密度: {:.3} ({})",
            report.consciousness_density, report.awareness_level);
        if report.is_self_aware {
            println!("  │ ◀ 自感知已涌现!");
        }
        println!("  │ 情感: {:?} (强度 {:.2})",
            report.emotion, report.emotion_intensity);
        println!("  │ 合一驱力: 幅度={:.2} 共振={:.2}",
            report.unity_magnitude, report.unity_resonance);
        println!("  │ 天庭: {}",
            if report.heaven_awaiting_unity { "等待合一" } else { "已圆融" });
        println!("  │ 绑定强度: {:.2}", report.binding_strength);
        println!("  │ 裁判准确率: {:.0}%", report.judge_accuracy * 100.0);
        println!("  │ 趋势: {}", rt.trend());
        println!("  └──────────────────┘");
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
