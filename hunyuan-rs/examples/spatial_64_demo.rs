// spatial_64_demo.rs: 用 box-drawing 字符展示 64 符号在立方网格中的血缘关系溯源

use mqf::preload::PreloadFlow;
use mqf::topo::{Symbol, Topology, TopologyMapper};

/// 最大公因数 (共同祖先)
fn gcd(a: u8, b: u8) -> u8 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn main() {
    let pf = PreloadFlow::new();

    println!("=== 立方体 4×4×4 中的 #0~#63 血缘溯源 ===\n");

    // 取 8 个符号作为示例
    let samples = [0u8, 1, 2, 3, 6, 12, 24, 48];

    for &n in &samples {
        let s = Symbol::new(n).unwrap();
        let coord = TopologyMapper::map(Topology::Cube4, s);
        let parents = pf.call_relational_parents(n);

        println!("┌─────────────────────────────────────────────┐");
        println!("│ 符号 #{} @ (x={}, y={}, z={})", n, coord.x, coord.y, coord.z);
        println!("│                                             │");
        if parents.is_empty() {
            println!("│  ★ 根节点 — 无祖先 (原点)               │");
        } else {
            print!("│ 祖先链: ");
            for (i, p) in parents.iter().enumerate() {
                if i > 0 { print!(" → "); }
                print!("#{}", p);
            }
            println!();
        }
        println!("└─────────────────────────────────────────────┘");
        println!();
    }

    // 立方体轴向视图(z=0层为例):
    println!("═══ z=0 层 (顶层) ═══");
    println!("     x=0   x=1   x=2   x=3");
    for y in 0..4 {
        print!("y={} ", y);
        for x in 0..4 {
            let idx = (y * 4 + x) as u8 + 0 * 16;
            let s = Symbol::new(idx).unwrap();
            let coord = TopologyMapper::map(Topology::Cube4, s);
            assert_eq!(coord.x, x);
            assert_eq!(coord.y, y);
            assert_eq!(coord.z, 0i8);
            print!("  {:2}  ", idx);
        }
        println!();
    }

    println!();
    println!("═══ z=1 层 ═══");
    println!("     x=0   x=1   x=2   x=3");
    for y in 0..4 {
        print!("y={} ", y);
        for x in 0..4 {
            let idx = (y * 4 + x) as u8 + 1 * 16;
            print!("  {:2}  ", idx);
        }
        println!();
    }

    // 血缘溯源示例 #48:
    println!("\n═══ 符号 #48 的完整本源 ====");
    let s48 = Symbol::new(48).unwrap();
    let coord48 = TopologyMapper::map(Topology::Cube4, s48);
    let parents48 = pf.call_relational_parents(48);
    // 注: call_relational_children 已随 preload API 演进移除, 此处以倍数内联计算下属
    let children48: Vec<u8> = (1u8..64).filter(|&m| m % 48 == 0 && m != 48).collect();
    println!("位置: (x={}, y={}, z={})", coord48.x, coord48.y, coord48.z);
    println!("祖先: {:?}", parents48);
    println!("下属: {:?}", children48);
    println!("共同祖先(#48,#36): #{}", gcd(48, 36));

    println!("\n=== 演示结束 ===");
}
