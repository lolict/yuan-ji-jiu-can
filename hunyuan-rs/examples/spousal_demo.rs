// ══════════════════════════════════════════════════════════════════════
// spousal_demo.rs — 夫妻共鸣裁决演示 (从预加载池取样)
// ══════════════════════════════════════════════════════════════════════
//
// 运行: cargo run --example spousal_demo
//
// 演示: 从 GlyphPool 取出汉字, 对调 spousal_resonance 引擎裁决
//       输出每对汉字的共鸣等级和见证结果

use mqf::glyph_preload::GlyphPool;
use mqf::spousal_resonance::SpousalResonanceEngine;

fn main() {
    println!("═══════════════════════════════════════════════════");
    println!("  NJSG 夫妻共鸣裁决引擎 · 预加载池演示");
    println!("═══════════════════════════════════════════════════\n");

    let pool = GlyphPool::new();
    let engine = SpousalResonanceEngine::new();

    println!("预加载池大小: {} 个汉字\n", pool.len());

    // ── 待测试的配对 ──
    let pairs: Vec<(char, char)> = vec![
        ('囡', '囝'),   // 夫妻
        ('阴', '阳'),   // 阴阳
        ('阴', '囝'),   // 阴阳+夫妻
        ('袋', '桶'),   // 容器+容器
        ('照', '袋'),   // 操作+容器
        ('我', '你'),   // 谐音
        ('莫', '连'),   // 谐音桥
        ('升', '降'),   // 极性
        ('开', '关'),   // 操作反向
        ('聚', '散'),   // 操作反向
    ];

    println!("┌──────┬──────┬────────────┬──────────┬──────────┐");
    println!("│  A   │  B   │  共鸣等级  │  综合分  │  见证    │");
    println!("├──────┼──────┼────────────┼──────────┼──────────┤");

    for (a, b) in &pairs {
        match (pool.get(*a), pool.get(*b)) {
            (Some(ga), Some(gb)) => {
                let report = engine.judge(ga, gb);
                println!(
                    "│  {}   │  {}   │ {:<10} │ {:>6}   │ {:<8} │",
                    report.glyph_a, report.glyph_b,
                    format!("{:?}", report.level),
                    report.final_score,
                    if report.can_witness { "✓" } else { "✗" }
                );
            }
            _ => {
                println!("│  {}   │  {}   │ (未在池中) │    ─     │    ─     │", a, b);
            }
        }
    }
    println!("└──────┴──────┴────────────┴──────────┴──────────┘\n");

    // ── 详细裁决一对 ──
    if let (Some(nan), Some(jian)) = (pool.get('囡'), pool.get('囝')) {
        let r = engine.judge(nan, jian);
        println!("【详细裁决示例: 囡 + 囝】");
        println!("  极性和谐度: {}", r.polarity_harmony);
        println!("  家族张力: {}", r.family_tension);
        println!("  距离得分: {:.4}", r.distance_score);
        println!("  综合分: {}", r.final_score);
        println!("  共鸣等级: {:?} — {}", r.level, r.level.description());
        println!("  容器裁决: {:?}", r.judgment);
        println!("  可三元见证: {}", r.can_witness);
    }

    println!("\n═══════════════════════════════════════════════════");
}
