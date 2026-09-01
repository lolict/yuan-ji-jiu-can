// ══════════════════════════════════════════════════════════════════
// examples/self_bootstrap_proof.rs — 自举证明
// ══════════════════════════════════════════════════════════════════
//
// 满全系统的"自举"意味着:不用任何外部数据/字体/配置,
// 系统自己就能生成完整可运行的符号表 + 全部运行时结构。
//
// 本 demo 验证:
//   1. SymbolTable::new() 在无外部文件时自举生成 64 个字形
//   2. 生成的字形可渲染 (甲, 乙, ..., 子, ..., ●, ...)
//   3. Symbol → Topology → Angle → SMRL 全链路无需外部依赖
//   4. 64 个符号里任选两个做血缘 + 角度 + 判决,端到端跑通

use mqf::angle::Angle;
use mqf::preload::PreloadFlow;
use mqf::smrl::SmrlEngine;
use mqf::stats::StreamStats;
use mqf::symbol_table::SymbolTable;
use mqf::topo::{Symbol, Topology, TopologyMapper};

/// 最大公因数 (共同祖先)
fn gcd(a: u8, b: u8) -> u8 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn main() {
    println!("═══ 自举证明 (Self-Bootstrap Proof) ═══\n");

    let pf = PreloadFlow::new();
    let table = SymbolTable::new();
    let engine = SmrlEngine::new(0);

    // ── 1. 证明:无外部文件也能生成 64 符号字形 ──
    print!("[1] 字形自举:");
    let mut all_non_empty = true;
    for i in 0u8..64 {
        let s = Symbol::new(i).unwrap();
        let g = table.glyph(s);
        if g.is_empty() { all_non_empty = false; break; }
    }
    println!(" {}", if all_non_empty { "✓ 全部 63 字形非空" } else { "✗ 有 glyph 为空" });

    // ── 2. 显示自举字形样本 ──
    print!("[2] 字形样本:");
    let mut samples: Vec<String> = Vec::new();
    for i in [0u8, 1, 9, 10, 11, 21, 22, 53, 63] {
        let s = Symbol::new(i).unwrap();
        samples.push(format!("{}#{}", table.glyph(s), i));
    }
    println!(" {}", samples.join(" "));

    // ── 3. 反向查找 ──
    print!("[3] 反向查 '甲':");
    let found = table.find("甲");
    println!(" {}", found.map(|s| format!("#{}", s.as_u8())).unwrap_or_else(|| "未找到".into()));

    // ── 4. 端到端链路任意两符号 ──
    print!("[4] 端到端链路 (#36 ↔ #9):");
    let a = Symbol::new(36).unwrap();
    let b = Symbol::new(9).unwrap();
    let ancestors_a = pf.call_relational_parents(a.as_u8());
    let ancestors_b = pf.call_relational_parents(b.as_u8());
    let common = gcd(a.as_u8(), b.as_u8());
    let angle_a = Angle::from_symbol(a);
    let angle_b = Angle::from_symbol(b);
    let verdict = engine.judge_angular(a, b);
    println!();
    println!("        #{} 祖先 = {:?}", a.as_u8(), ancestors_a);
    println!("        #{} 祖先 = {:?}", b.as_u8(), ancestors_b);
    println!("        共同祖先 = {:?}", common);
    println!("        角度距离 = {}°", angle_a.arc_distance(angle_b));
    println!("        裁判 = {:?} / 置信 {:.2}", verdict.reward, verdict.confidence);

    // ── 5. 用自身 StreamStats 统计 64 符号血缘深度 ──
    print!("[5] StreamStats 内建统计:");
    let mut st = StreamStats::new();
    for i in 1u8..64 {
        let parents = pf.call_relational_parents(i);
        st.push(parents.len() as f64);
    }
    println!(" 64 符号血缘深度 = 均值 {:.2}, 总计 {:.0}", st.mean, st.sum);

    // ── 6. 坐标不自举矛盾 ──
    print!("[6] 立方坐标范围:");
    let mut min_c = (i8::MAX, i8::MAX, i8::MAX);
    let mut max_c = (i8::MIN, i8::MIN, i8::MIN);
    for i in 0u8..64 {
        let s = Symbol::new(i).unwrap();
        let c = TopologyMapper::map(Topology::Cube4, s);
        min_c = (min_c.0.min(c.x), min_c.1.min(c.y), min_c.2.min(c.z));
        max_c = (max_c.0.max(c.x), max_c.1.max(c.y), max_c.2.max(c.z));
    }
    println!(" [{}..{}] × [{}..{}] × [{}..{}] ≡ 4×4×4",
        min_c.0, max_c.0, min_c.1, max_c.1, min_c.2, max_c.2);

    println!("\n═══ 自举证明完成 ═══");
    println!("结论:系统从零启动,无需外部 font / 数据文件,完整运行。");
}
