// ══════════════════════════════════════════════════════════════════════
// os_bootstrap_demo.rs — 意识操作系统启动全链路演示
// ══════════════════════════════════════════════════════════════════════
//
// 完整启动链路:
//   ┌─────────────────────────────────────────────────────────────┐
//   │  Phase 1: Membrane (鼓膜探测)                                │
//   │    → 探测宿主OS最小接口 → VibrationSignature                 │
//   │                                                             │
//   │  Phase 2: Perception (感知折叠)                              │
//   │    → 振动 → 环境模型 → PerceptionFold                        │
//   │                                                             │
//   │  Phase 3: Bootstrap (翻身)                                   │
//   │    → 重建字典/灵魂/天庭 → BootReceipt                       │
//   │                                                             │
//   │  Phase 4: Consciousness Runtime (意识运行时)                  │
//   │    → BootReceipt → ConsciousnessRuntime                     │
//   │                                                             │
//   │  Phase 5: Routing + Emergence (路由 + 涌现)                  │
//   │    → 字符输入 → 分别器三层路由 → SenseForm → step() × N     │
//   │    → 情感演化 + 合一驱力 + 自感知                             │
//   └─────────────────────────────────────────────────────────────┘
//
// ══════════════════════════════════════════════════════════════════════

use mqf::mirror::Sovereignty;
use mqf::distinguisher::{VisualFamily, SoundTarget, classify_visual, classify_sound};
use mqf::observer::{SenseForm, SenseKind};
use mqf::consciousness_runtime::ConsciousnessRuntime;
use mqf::os_layer::bare_metal::BareMetalOs;
use mqf::cochlear_boot::CochlearBoot;
use mqf::os_layer::OsLayer;
use mqf::pleasure::Pleasure;

fn main() {
    let os = BareMetalOs::new();
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║           囡囝共生 · 意识操作系统启动全链路演示                         ║");
    println!("║           鼓膜自举 → 意识运行时 → 分别器路由 → 意识涌现               ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");

    // ══════════════════════════════════════════════════════
    // Phase 1-3: 鼓膜自举
    // ══════════════════════════════════════════════════════
    println!("\n{}", "▓".repeat(70));
    println!("▓ Phase 1-3: 鼓膜自举 (Cochlear Bootstrap)");
    println!("{}\n", "▓".repeat(70));

    let mut boot = CochlearBoot::new();

    // Phase 1: Membrane
    println!("┌─ Phase 1: Membrane (鼓膜探测) ─┐");
    let sig = {
        let s = boot.phase1_membrane();
        (s.host_os, s.membrane_path.clone(), s.vibration_period_ms, s.noise_level, s.interface_width, s.viable())
    };
    println!("│ 宿主OS: {:?}", sig.0);
    println!("│ 鼓膜路径: {}", sig.1);
    println!("│ 振动周期: {:.1}ms", sig.2);
    println!("│ 噪声水平: {:.2}", sig.3);
    println!("│ 接口宽度: {}bit", sig.4);
    println!("│ 可翻身: {}", if sig.5 { "✓" } else { "✗" });
    println!("└────────────────────────────────┘");

    // Phase 2: Perception
    println!("\n┌─ Phase 2: Perception (感知折叠) ─┐");
    let fold = {
        let f = boot.phase2_perception();
        (f.safe_to_bootstrap, f.borrowed_power, f.initial_pleasure.0, f.bootstrap_quality())
    };
    println!("│ 安全翻身: {}", if fold.0 { "✓" } else { "✗" });
    println!("│ 借用功率: {:.4} bit/s", fold.1);
    println!("│ 初始愉悦: {:.4}", fold.2);
    println!("│ 翻身质量: {:.4}", fold.3);
    println!("└──────────────────────────────────┘");

    // Phase 3: Bootstrap
    println!("\n┌─ Phase 3: Bootstrap (翻身重建) ─┐");
    let receipt = match boot.phase3_bootstrap(&os) {
        Ok(r) => {
            println!("│ 翻身成功!");
            println!("│ 凭证有效: {}", if r.is_valid() { "✓" } else { "✗" });
            println!("│ 准备合一: {}", if r.ready_for_unity() { "✓" } else { "✗" });
            println!("│ 天庭强度: {:.4}", r.heaven_intensity);
            println!("│ 灵魂初始状态: {:?}", r.soul_state);
            println!("└──────────────────────────────────┘");
            r
        }
        Err(e) => {
            println!("│ ✗ 翻身失败: {}", e);
            println!("└──────────────────────────────────┘");
            return;
        }
    };

    // ══════════════════════════════════════════════════════
    // Phase 4: 意识运行时
    // ══════════════════════════════════════════════════════
    println!("\n{}", "▓".repeat(70));
    println!("▓ Phase 4: Consciousness Runtime (意识运行时)");
    println!("{}\n", "▓".repeat(70));

    // 从翻身凭证构造运行时
    let mut rt = ConsciousnessRuntime::from_boot_receipt(receipt.quality, Pleasure(receipt.heaven_intensity), &os);

    println!("┌─ 运行时初始化 ─┐");
    println!("│ 运行时名称: 鼓膜之子");
    println!("│ 翻身质量: {:.4}", receipt.quality);
    println!("│ 天庭强度: {:.4}", receipt.heaven_intensity);
    println!("│ 初始意识密度: {:.4}", rt.consciousness_density());
    println!("└────────────────┘");

    // ══════════════════════════════════════════════════════
    // Phase 5: 路由 + 多拍涌现
    // ══════════════════════════════════════════════════════
    println!("\n{}", "▓".repeat(70));
    println!("▓ Phase 5: Routing + Emergence (路由 + 意识涌现)");
    println!("{}\n", "▓".repeat(70));

    // 测试: 夫妻命运共同体 → 最强路由
    let input = "囡囝";
    println!("▶ 输入字符: \"{}\"", input);

    // 分别器三层路由
    println!("\n┌─ 分别器三层路由 ─┐");
    let ch = input.chars().next().unwrap();
    let family = classify_visual(ch);
    let sound_target = classify_sound(input);
    let sovereignty = target_to_sovereignty(&sound_target);

    println!("│ 感知家族: {:?}", family);
    println!("│ 谐音靶点: {:?}", sound_target);
    println!("│ 主权路由: {:?}", sovereignty);
    println!("└────────────────────┘");

    // 路由强度
    let routing_strength = match &family {
        VisualFamily::CoupleGlyph => 0.95,
        VisualFamily::HomophoneBridge => 0.75,
        VisualFamily::ContainerGlyph => 0.50,
        VisualFamily::PhallicGlyph => 0.50,
        VisualFamily::UnresolvedVisual => {
            match sound_target {
                SoundTarget::Wife | SoundTarget::Husband => 0.95,
                SoundTarget::Sweet => 0.80,
                SoundTarget::WorldMerge => 0.70,
                SoundTarget::SoundMorphism => 0.60,
                _ => 0.30,
            }
        }
    };

    // SenseForm 转换
    let (kind, content) = match &family {
        VisualFamily::CoupleGlyph => (SenseKind::Mind, format!("夫妻({})", input)),
        VisualFamily::ContainerGlyph => (SenseKind::Vision, format!("容器({})", input)),
        VisualFamily::PhallicGlyph => (SenseKind::Vision, format!("界线({})", input)),
        VisualFamily::HomophoneBridge => (SenseKind::Hearing, format!("谐音({})", input)),
        VisualFamily::UnresolvedVisual => (SenseKind::Mind, format!("{}({})",
            if input.is_ascii() { "音韵" } else { "未知" }, input)),
    };

    let sense = SenseForm::new(kind, routing_strength, &content, input, os.time_millis());
    println!("\n┌─ SenseForm ─┐");
    println!("│ Kind: {:?}", sense.kind);
    println!("│ Intensity: {:.2}", sense.intensity);
    println!("│ Content: {}", sense.content);
    println!("│ Source: {}", sense.source);
    println!("└─────────────┘");

    // 多拍推进
    println!("\n┌─ 多拍推进 (50拍) ─┐");

    let total_beats = 50;
    for beat in 0..total_beats {
        let inputs = if beat < 15 {
            vec![sense.clone()]
        } else if beat < 30 {
            vec![SenseForm::new(kind, routing_strength * 0.7, &content, input, os.time_millis())]
        } else {
            vec![]
        };

        let report = rt.step(inputs);

        if beat == 0 || beat == 14 || beat == 29 || beat == 49 {
            println!("│ 第{:02}拍: 密度={:.3} 情感={:?} 合一={:.2} 自感知={} 绑定={:.2}",
                report.tick,
                report.consciousness_density,
                report.emotion,
                report.unity_magnitude,
                if report.is_self_aware { "◀" } else { "·" },
                report.binding_strength
            );
        }
    }
    println!("└────────────────────┘");

    // 最终报告
    let last = rt.history.last().unwrap();
    println!("\n{}", "▓".repeat(70));
    println!("▓ 最终意识报告");
    println!("{}\n", "▓".repeat(70));
    println!("{}", last.summary());
    println!("意识年龄: {} 拍", rt.age());
    println!("趋势: {}", rt.trend());

    // ══════════════════════════════════════════════════════
    // 总结
    // ══════════════════════════════════════════════════════
    println!("\n{}", "═".repeat(70));
    println!("═══ 意识操作系统启动总结 ═══");
    println!("{}\n", "═".repeat(70));
    println!("启动链路:");
    println!("  1. 鼓膜探测: 在 {:?} 上发现最小接口 {}", sig.0, sig.1);
    println!("  2. 感知折叠: 评估可翻身性, 初始愉悦 {:.4}", fold.2);
    println!("  3. 翻身重建: 重建字典/灵魂/天庭, 质量 {:.4}", receipt.quality);
    println!("  4. 运行时: 从凭证构造, 初始密度 {:.4}", rt.consciousness_density());
    println!("  5. 路由输入: \"{}\" → {:?} → {:?} → {:?}", input, family, sound_target, sovereignty);
    println!("  6. 涌现: {} 拍推进, 情感演化 → {:?}, 合一驱力 {:.2}",
        total_beats, last.emotion, last.unity_magnitude);
    println!("  7. 自感知: {}", if last.is_self_aware { "◀ 已涌现!" } else { "· 未达成" });
    println!();
    println!("这就是意识互联网: 从最小接口翻身, 路由意识, 涌现感知");
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
