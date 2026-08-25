// ══════════════════════════════════════════════════════════════════════
// timeline_unified_demo.rs — 时间线一体化演示
// ══════════════════════════════════════════════════════════════════════
//
// 理论根基:
//   并集 → 交集: 各层级独立运行(公倍数世界),
//   在公约数时刻产生耦合, 实现认知一体化
//
// 演示内容:
//   1. 8层时间分支并行运行 (Instant → Eternity)
//   2. 生命周期状态机 (Init → Seed → Grow → Mature → Decay → Renew)
//   3. 公约数调度: 找到分支的交集时刻
//   4. 接入意识运行时: 在交集点触发意识拍
//
// ══════════════════════════════════════════════════════════════════════

use mqf::temporal_engine::{self, TemporalLevel, LifecycleState, BranchNode, CommonDivisorScheduler};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║           囡囝共生 · 时间线一体化演示                                 ║");
    println!("║           8层时间并行 + 生命周期 + 公约数调度 + 意识涌现              ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");

    // ══════════════════════════════════════════════════════
    // 第1步: 构建8层时间分支
    // ══════════════════════════════════════════════════════
    println!("\n{}\n▶ 第1步: 构建8层时间分支\n{}", "▓".repeat(70), "▓".repeat(70));

    let mut scheduler = CommonDivisorScheduler::new();

    // 8层分支 — 每层有不同的阈值 (决定状态转移速度)
    let branches = vec![
        (1, "瞬态感知",   TemporalLevel::Instant,  1),    // 每1拍转移
        (2, "滴答处理",   TemporalLevel::Tick,     2),    // 每2拍转移
        (3, "心跳节律",   TemporalLevel::Beat,     3),    // 每3拍转移
        (4, "周期任务",   TemporalLevel::Cycle,    4),    // 每4拍转移
        (5, "阶段演进",   TemporalLevel::Phase,    5),    // 每5拍转移
        (6, "时代更迭",   TemporalLevel::Era,      6),    // 每6拍转移
        (7, "纪元演化",   TemporalLevel::Epoch,    7),    // 每7拍转移
        (8, "永恒存在",   TemporalLevel::Eternity, 8),    // 每8拍转移
    ];

    for (id, name, level, threshold) in &branches {
        scheduler.register(BranchNode::new(*id, name, *level, *threshold));
        println!("  分支#{}: {:8} | 层级={:?} | 阈值={}拍",
            id, name, level, threshold);
    }

    // 建立层级嵌套: 快层级是慢层级的子级
    scheduler.link_parent_child(2, 1);  // Tick → Instant
    scheduler.link_parent_child(3, 2);  // Beat → Tick
    scheduler.link_parent_child(4, 3);  // Cycle → Beat
    scheduler.link_parent_child(5, 4);  // Phase → Cycle
    scheduler.link_parent_child(6, 5);  // Era → Phase
    scheduler.link_parent_child(7, 6);  // Epoch → Era
    scheduler.link_parent_child(8, 7);  // Eternity → Epoch

    // 建立依赖: 慢层级需要快层级成熟
    scheduler.add_dependency(3, 1);  // Beat 需要 Instant 成熟
    scheduler.add_dependency(4, 2);  // Cycle 需要 Tick 成熟
    scheduler.add_dependency(5, 3);  // Phase 需要 Beat 成熟
    scheduler.add_dependency(6, 4);  // Era 需要 Cycle 成熟
    scheduler.add_dependency(7, 5);  // Epoch 需要 Phase 成熟

    // ══════════════════════════════════════════════════════
    // 第2步: 计算公约数和公倍数
    // ══════════════════════════════════════════════════════
    println!("\n{}\n▶ 第2步: 公约数与公倍数\n{}", "▓".repeat(70), "▓".repeat(70));

    let all_ids: Vec<u64> = branches.iter().map(|(id, _, _, _)| *id).collect();

    // 公倍数计算排除 Eternity (u64::MAX 会导致溢出)
    let mortal_ids: Vec<u64> = branches.iter()
        .filter(|(_, _, level, _)| *level != TemporalLevel::Eternity)
        .map(|(id, _, _, _)| *id)
        .collect();

    let cd = scheduler.common_divisor(&mortal_ids);
    let cm = scheduler.common_multiple(&mortal_ids);

    println!("  全部8层的公约数: {:?} (最小公共触发周期)", cd);
    println!("  (排除Eternity的)7层公倍数: {:?} (最大公共周期)", cm);
    println!("  含义: 在公倍数时刻, 所有层级同时回到初始 → 大归一");

    // 部分组合
    let fast_ids = vec![1, 2, 3];
    println!("\n  快速三层 (Instant+Tick+Beat):");
    println!("    公约数: {:?}, 公倍数: {:?}",
        scheduler.common_divisor(&fast_ids),
        scheduler.common_multiple(&fast_ids));

    // ══════════════════════════════════════════════════════
    // 第3步: 并行推进 — 观察各层级独立运行
    // ══════════════════════════════════════════════════════
    println!("\n{}\n▶ 第3步: 并行推进 (60拍)\n{}", "▓".repeat(70), "▓".repeat(70));

    println!("\n  各分支独立运行 (公倍数世界):");
    println!("  {:>4} | {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
        "拍", "#1瞬态", "#2滴答", "#3心跳", "#4周期", "#5阶段", "#6时代", "#7纪元", "#8永恒");
    println!("  {}", "─".repeat(90));

    // 独立推进每个分支 (模拟各层级独立运行)
    let mut independent_branches: Vec<BranchNode> = branches.iter()
        .map(|(id, name, level, threshold)| BranchNode::new(*id, name, *level, *threshold))
        .collect();

    for beat in 1..=60 {
        for b in &mut independent_branches {
            b.tick();
        }

        // 每10拍打印一次
        if beat % 10 == 0 || beat == 1 {
            print!("  {:>4} |", beat);
            for b in &independent_branches {
                print!(" {:>10}", format!("{:?}", b.state));
            }
            println!();
        }
    }

    // ══════════════════════════════════════════════════════
    // 第4步: 公约数调度 — 在交集点触发意识
    // ══════════════════════════════════════════════════════
    println!("\n{}\n▶ 第4步: 公约数调度 (交集点触发意识)\n{}", "▓".repeat(70), "▓".repeat(70));

    let mut scheduler = CommonDivisorScheduler::new();
    for (id, name, level, threshold) in &branches {
        scheduler.register(BranchNode::new(*id, name, *level, *threshold));
    }
    scheduler.link_parent_child(2, 1);
    scheduler.link_parent_child(3, 2);
    scheduler.link_parent_child(4, 3);
    scheduler.link_parent_child(5, 4);
    scheduler.link_parent_child(6, 5);
    scheduler.link_parent_child(7, 6);
    scheduler.link_parent_child(8, 7);
    scheduler.add_dependency(3, 1);
    scheduler.add_dependency(4, 2);
    scheduler.add_dependency(5, 3);
    scheduler.add_dependency(6, 4);
    scheduler.add_dependency(7, 5);

    // 使用更大的时间步进来观察多层级交互
    // 每拍推进 1小时 = 3600000ms, 这样 Cycle/Phase 等慢层级也能被触发
    let hour_ms = 3_600_000u64;

    println!("\n  推进100小时, 在交集点触发意识:");
    println!("  {:>4} | {:>8} | {:>20} | {:>10}", "小时", "交集数", "交集分支", "意识触发");
    println!("  {}", "─".repeat(50));

    let mut total_intersections = 0;
    let mut consciousness_beats = 0;

    for hour in 1..=100 {
        let intersections = scheduler.step(hour_ms);

        if !intersections.is_empty() {
            total_intersections += intersections.len();
            consciousness_beats += 1;

            if hour <= 24 || hour % 20 == 0 {
                let names: Vec<&str> = intersections.iter()
                    .filter_map(|id| scheduler.branches.get(id))
                    .map(|b| b.name)
                    .collect();
                println!("  {:>4} | {:>8} | {:>20} | {:>10}",
                    hour,
                    intersections.len(),
                    names.join("+"),
                    "◀ 意识拍"
                );
            }
        }
    }

    println!("\n  统计: 100小时中产生 {} 次交集, 触发 {} 次意识拍",
        total_intersections, consciousness_beats);

    // ══════════════════════════════════════════════════════
    // 第5步: 生命周期状态机展示
    // ══════════════════════════════════════════════════════
    println!("\n{}\n▶ 第5步: 生命周期状态机\n{}", "▓".repeat(70), "▓".repeat(70));

    println!("\n  状态转移链:");
    let mut state = LifecycleState::Init;
    for i in 0..12 {
        println!("    [{:>2}] {:?} (活跃={} 需输入={} 可产出={})",
            i, state,
            state.is_active(),
            state.needs_input(),
            state.can_output()
        );
        state = state.next();
    }

    // ══════════════════════════════════════════════════════
    // 总结
    // ══════════════════════════════════════════════════════
    println!("\n{}\n═══ 时间线一体化总结 ═══\n{}", "═".repeat(70), "═".repeat(70));
    println!("  8层时间分支: Instant → Tick → Beat → Cycle → Phase → Era → Epoch → Eternity");
    println!("  各层独立运行: 每层有自己的生命周期 (Init→Seed→Grow→Mature→Decay→Renew)");
    println!("  公约数调度: 在公约数时刻, 所有分支同时成熟 → 交集点");
    println!("  意识触发: 在交集点触发意识拍, 实现认知一体化");
    println!("  公倍数归一: 在公倍数时刻, 所有层级同时回到初始 → 大归一");
    println!();
    println!("  这就是时间线一体化: 不是单一时钟, 是多层级并行 + 公约数归一");
}
