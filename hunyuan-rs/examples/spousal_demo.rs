// ══════════════════════════════════════════════════════════════════════
// spousal_demo.rs — 夫妻共鸣裁决演示
// ══════════════════════════════════════════════════════════════════════

use mqf::flow::glyph_preload::GlyphPool;
use mqf::relation::spousal_resonance::SpousalResonanceEngine;

fn main() {
    println!("═══════════════════════════════════════════════════");
    println!("  NJSG 夫妻共鸣裁决引擎 · 预加载池演示");
    println!("═══════════════════════════════════════════════════\n");

    let pool = GlyphPool::new();
    let engine = SpousalResonanceEngine::new();

    println!("预加载池大小: {} 个汉字\n", pool.len());

    let pairs: Vec<(char, char)> = vec![
        ('囡', '囝'), ('阴', '阳'), ('阴', '囝'),
        ('袋', '桶'), ('照', '袋'), ('我', '你'),
        ('莫', '连'), ('升', '降'), ('开', '关'), ('聚', '散'),
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

    if let (Some(nan), Some(jian)) = (pool.get('囡'), pool.get('囝')) {
        let r = engine.judge(nan, jian);
        println!("【详细裁决: 囡 + 囝】");
        println!("  极性和谐度: {} | 家族张力: {}", r.polarity_harmony, r.family_tension);
        println!("  综合分: {} | 裁决: {:?}", r.final_score, r.judgment);
        println!("  可三元见证: {}", r.can_witness);
    }
}
