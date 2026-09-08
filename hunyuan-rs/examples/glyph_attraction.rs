// ══════════════════════════════════════════════════════════════════════
// glyph_attraction.rs — 汉字活容器示例: 囡囝夫妻相遇演示
// ══════════════════════════════════════════════════════════════════════
//
// 运行: cargo run --example glyph_attraction
//
// 演示:
//   1. 创建"囡"和"囝"两个活容器
//   2. 计算它们之间的"关系距离"
//   3. 判断是否"相吸" (即是否匹配)
//   4. 输出交互结果

use mqf::glyph_living::{LivingGlyph, GlyphFamily, Shengmu, Yunmu, Tone};

fn main() {
    println!("═══════════════════════════════════════════════");
    println!("  汉字活容器 · 囡囝相遇演示");
    println!("═══════════════════════════════════════════════\n");

    // ── 第一步: 创建两个活容器 ──

    // "囡" = 妻子 (低气压/包容/阴)
    let nan = LivingGlyph::new(
        '囡',
        Shengmu::N,       // 声母: N
        Yunmu::An,        // 韵母: an
        Tone::T1,         // 一声
        GlyphFamily::Couple, // 夫妻家族
    );

    // "囝" = 丈夫 (高气压/主动/阳)
    let jian = LivingGlyph::new(
        '囝',
        Shengmu::J,       // 声母: J
        Yunmu::Iou,       // 韵母: iou (实际发音接近 "iu")
        Tone::T1,         // 一声
        GlyphFamily::Couple, // 夫妻家族
    );

    // ── 第二步: 自我感知 ──

    let nan_report = nan.gauge_self();
    let jian_report = jian.gauge_self();

    println!("【自我感知】");
    println!("┌─────────┬──────────┬──────────┬──────────┬──────────┐");
    println!("│  汉字   │ Unicode  │  声母    │  韵母    │  家族    │");
    println!("├─────────┼──────────┼──────────┼──────────┼──────────┤");
    println!(
        "│ {}      │ U+{:04X}   │ {:<8} │ {:<8} │ {:<8} │",
        nan_report.glyph, nan_report.code,
        format!("{:?}", nan_report.shengmu),
        format!("{:?}", nan_report.yunmu),
        "Couple"
    );
    println!(
        "│ {}      │ U+{:04X}   │ {:<8} │ {:<8} │ {:<8} │",
        jian_report.glyph, jian_report.code,
        format!("{:?}", jian_report.shengmu),
        format!("{:?}", jian_report.yunmu),
        "Couple"
    );
    println!("└─────────┴──────────┴──────────┴──────────┴──────────┘\n");

    // ── 第三步: 谐音匹配检测 ──

    println!("【谐音桥检测】");
    println!("  声母匹配: {}", nan.same_shengmu(&jian));
    println!("  韵母匹配: {}", nan.same_yunmu(&jian));
    println!("  发音完全相同: {}", nan.same_pronunciation(&jian));

    // 囡囝不是谐音 — 声母不同(N vs J), 韵母不同(an vs iou)
    // 但它们在"夫妻家族"中极性相同 → 不是靠声音, 靠"家族引力"
    println!("  → 囡囝不是靠谐音, 而是靠夫妻家族的同极性引力\n");

    // ── 第四步: 计算关系距离 ──

    let dist = nan.distance_to(&jian);

    println!("【关系距离计算】");
    println!("  d0 (声韵调空间): {:.2}", dist.d0);
    println!("  d1 (韵母接收空间): {:.2}", dist.d1);
    println!("  d2 (层级环空间): {:.2}", dist.d2);
    println!("  极性差异: {:.2} (0 = 同极性, 10 = 反极性)", dist.polarity_diff);
    println!("  ─────────────────────────────");
    println!("  总距离: {:.2}\n", dist.total);

    // ── 第五步: 相吸判断 ──

    let threshold = 100.0; // 阈值
    let attracted = nan.is_attracted(&jian, threshold);

    println!("【相吸判断】");
    println!("  阈值: {:.2}", threshold);
    println!("  实际距离: {:.2}", dist.total);
    println!("  结果: {}", if attracted { "✓ 相吸 — 可建立关系" } else { "✗ 相斥 — 距离过远" });
    println!();

    // ── 第六步: 编码为 64 位镜像 ──

    let nan_cell = nan.to_cell();
    let jian_cell = jian.to_cell();

    println!("【64位镜像编码】");
    println!("  囡 = 0b{:064b}", nan_cell);
    println!("  囝 = 0b{:064b}", jian_cell);
    println!("  (可直接被 mind_organ 处理)\n");

    // ── 总结 ──

    println!("═══════════════════════════════════════════════");
    println!("  结论: 囡(N)与囝(J)通过夫妻家族同极性");
    println!("        即使声韵不同, 仍能建立引力关系");
    println!("═══════════════════════════════════════════════");
}
