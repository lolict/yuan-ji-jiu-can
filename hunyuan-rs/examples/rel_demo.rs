// rel_demo.rs: 用 topo 和 rel 子系统演示关系的建立、溯源、分叉和归属判断。
//
// 展示二分法和多分法如何在符号集合上运行。

use mqf::topo::{Symbol, Topology, TopologyMapper};
use mqf::rel::{RelGraph, RelKind};

fn main() {
    println!("=== 64进制符号拓扑与关系演示 ===\n");

    // 1. 显示符号在立方体 4×4×4 中的坐标
    println!("--- 立方体拓扑映射 ---");
    for i in [0, 1, 4, 5, 16, 63].iter() {
        let s = Symbol::new(*i).unwrap();
        let c = TopologyMapper::map(Topology::Cube4, s);
        println!("符号 #{} → (x={}, y={}, z={})", i, c.x, c.y, c.z);
    }

    // 2. 建立关系示例（线性链）
    println!("\n--- 关系建立（派生树） ---");
    println!("用 6 个符号 0~5,建立纵向关系");
    println!("派生树: #0 派生 #1/#2, #1 派生 #3/#4, #3 派生 #5\n");

    let graph = RelGraph::from_pairs(&[
        (0, 1, RelKind::PaiSheng),
        (0, 2, RelKind::PaiSheng),
        (1, 3, RelKind::PaiSheng),
        (1, 4, RelKind::PaiSheng),
        (3, 5, RelKind::PaiSheng),
    ]);

    // 显示每个节点的出边
    for i in 0..6u8 {
        let outs: Vec<u8> = graph.out_edges(i).iter().map(|r| r.to).collect();
        if outs.is_empty() {
            println!("  #{} → 叶节点", i);
        } else {
            println!("  #{} → {:?}", i, outs);
        }
    }

    // 3. 溯源查询
    println!("\n--- 溯源 ---");
    let ancestors_5 = graph.trace_origin(5);
    println!("  #5 的完整祖先链: {:?}", ancestors_5);
    println!("  (说明 #5 由 #3→#1→#0 派生而来)");

    let ancestors_4 = graph.trace_origin(4);
    println!("  #4 的完整祖先链: {:?}", ancestors_4);

    // 4. 二分法 → 多分法（传递闭包）
    println!("\n--- 多分法（传递闭包） ---");
    for i in 0..3u8 {
        let reachable = graph.reachable(i);
        println!(
            "  #{} 的派生范围: 共 {} 个符号 → {:?}",
            i,
            reachable.len(),
            &reachable[..reachable.len().min(10)]
        );
    }

    // 5. 分叉判断
    println!("\n--- 分叉判断 ---");
    for i in 0..6u8 {
        let degree = graph.out_degree(i);
        let branching = graph.is_branch(i);
        if branching {
            println!("  #{} 是分叉点 (出度={}),派生两条以上线", i, degree);
        } else {
            println!("  #{} 非分叉点 (出度={})", i, degree);
        }
    }

    // 6. 归属判断
    println!("\n--- 归属判断 ---");
    let test_cases = [(2, 0), (4, 0), (5, 1), (2, 1)];
    for (a, b) in test_cases {
        let owned = graph.is_owned_by(a, b);
        println!("  #{} {} 归属 #{}", a, if owned { "✓" } else { "✗" }, b);
    }

    // 7. 分叉同源判断
    println!("\n--- 同源判断 ---");
    let test_pairs = [(3, 4), (2, 5), (4, 5)];
    for (a, b) in test_pairs {
        match graph.common_origin(a, b) {
            Some(origin) => println!("  #{} 和 #{} 有共同祖先: #{}", a, b, origin),
            None => println!("  #{} 和 #{} 没有共同祖先", a, b),
        }
    }

    println!("\n演示完成。关系代数已就位。");
}
