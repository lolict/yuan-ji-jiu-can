// ══════════════════════════════════════════════════════════════════
// encoding_bridge_demo.rs — 编码桥接演示
// ══════════════════════════════════════════════════════════════════
//
// 展示 Rust 核心如何通过 FFI 调用 C 编码系统
//
// 运行: cargo run --example encoding_bridge_demo

fn main() {
    println!("╔══════════════════════════════════════════╗");
    println!("║  囡囝共生 · 编码桥接 v0.31               ║");
    println!("║  C encoding → Rust FFI demo             ║");
    println!("╚══════════════════════════════════════════╝\n");

    let chars = [
        "回", "好", "一", "国", "人", "我", "二", "三", "十", "天", "地",
        "网", "开", "关", "时", "间", "的", "是", "不", "在", "有", "和",
        "风", "云", "雨", "电", "末", "未", "之", "予", "即",
        "家", "树", "永", "求", "传", "体", "李", "果",
    ];

    // ── 基本统计 ──
    let count = mqf::encoding_bridge::hanzi_count();
    println!("表内汉字数: {}\n", count);

    // ── 逐字分析 ──
    for ch in &chars {
        let code = mqf::encoding_bridge::utf8_to_container(ch);
        if code == 0 {
            println!("「{}」→ 未收录", ch);
            continue;
        }

        let fam = mqf::encoding_bridge::Family::from_container_code(code);
        let pol = (code >> 28) & 1;
        let flow = mqf::encoding_bridge::compute_flow_bits(0, fam);

        println!(
            "「{}」: code=0x{:08X}  家族={:<12}  极性={}  流=0x{:03X}",
            ch,
            code,
            format!("{}", fam),
            if pol == 0 { "阴" } else { "阳" },
            flow.bits()
        );
    }

    // ── 家族分布统计 ──
    let mut fam_counts = [0u32; 5];
    for ch in &chars {
        let code = mqf::encoding_bridge::utf8_to_container(ch);
        if code != 0 {
            let idx = ((code >> 29) & 0x7) as usize;
            if idx < 5 { fam_counts[idx] += 1; }
        }
    }

    println!("\n── 家族分布统计 (样本) ──");
    let names = ["容器(刘楚恬)", "边界(lolict)", "夫妻(合)", "网状", "中性"];
    for i in 0..5 {
        println!("  {}: {}", names[i], fam_counts[i]);
    }

    println!("\n✅ 编码桥接正常 · C ↔ Rust FFI 通畅");
}
