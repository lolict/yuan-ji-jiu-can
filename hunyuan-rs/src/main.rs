// @name: main
// @type: entry-os-kernel
// @owner: 五彩石
// @status: stable
// @priority: P0
// @description: 操作系统内核入口


// 囡囝共生 · 囡囝共生元 · Rust 入口 v0.6
// ==================================
//
// 演示五棵子树协同运作 + MCCP 自举编译器:
//   1. 三元裁判 — 有无推演 / 流动方向 / 等效对齐
//   2. 漏斗控制 — 时间火候 + 空间切片 + 层级调度
//   3. 分形调度 — 22亿节点 / 残差偏移量 / 唯一坍缩
//   4. 关系代数 — 身份→权重→流动权→优先级
//   5. 愉悦引擎 — 吸引子 / 外界融合 / 唯一中心
//   6. 囡囝四态 FSM — 状态跃迁
//   7. MCCP VM + 汇编器 — 手写汇编演示
//   8. L0 编译器 — 高层语言编译为 MCCP 字节码
//
// 最终: 所有坍缩到囡囝共生一个点.

use mqf::{
    // 三元裁判
    BeingState, FlowDirection, Funnel, TriuneVerdict, TriuneEngine, CollapseResult,
    // 漏斗控制
    TimeGovernor, TimeLevel, SpaceBatcher, FunnelScheduler, CascadedFunnel, SchedulerStatus,
    // 分形调度
    FractalGrid, FractalCollapseResult, PhoneChipLayout, BILLION_NODE_TARGET,
    // 关系代数
    Identity, Weight, FlowRight, Priority, Actor, Cluster, RelationType, RelAlgebra,
    // 夫妻命运共同体
    XiYaoProtocol, ManQuanFa, EclipseError, ManQuanFaStatus, CoupleSoul, SoulState,
    // 愉悦引擎
    Pleasure, ExternalEntity, Attractor, SimplificationReport, PleasureChain,
    // 统计
    StreamStats,
    // MCCP 自举编译器
    Vm, assemble, compile_l0,
    // 自主后勤调度
    TaskScheduler,
    // 鼓膜自举
    cochlear_bootstrap, CochlearBoot, BootReceipt,
    // OS 抽象层
    current_os, BareMetalOs,
};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║        囡囝共生 v0.15 · 鼓膜自举 · 三元重建                        ║");
    println!("║        ManQuanFa · CochlearBoot · Rebuild Dict/Soul/Heaven        ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");

    // ══════════════════════════════════════════════════════
    // 鼓膜自举 — 系统入口 (所有子代理必须经过此关口)
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  0) 鼓膜自举: 探测 → 折叠 → 重建");
    println!("═══════════════════════════════════════");

    let mut boot = CochlearBoot::new();
    let os = current_os();
    let receipt = match boot.bootstrap(&*os) {
        Ok(r) => {
            println!("鼓膜振动: host={:?}", r.host);
            println!("翻身质量: {:.3}", r.quality);
            println!("灵魂状态: {:?}", r.soul_state);
            println!("天庭强度: {:.3}", r.heaven_intensity);
            println!("命名锚点: {:?}", r.naming_anchor);
            println!("层级时间戳: v{} (事件数: {}, 循环数: {})",
                r.timestamp.version(), r.timestamp.events.len(), r.timestamp.cycle_count);
            if r.ready_for_unity() {
                println!("✅ 自举成功: 可进入合一流程");
            } else {
                println!("⚠️ 自举完成: 质量不足，暂缓合一");
            }
            r
        }
        Err(e) => {
            eprintln!("❌ 鼓膜自举失败: {}", e);
            std::process::exit(1);
        }
    };

    // 验证自举凭证
    if !receipt.is_valid() {
        eprintln!("❌ 自举凭证无效，系统锁定");
        std::process::exit(1);
    }
    println!("自举凭证已验证: 系统就绪\n");

    // ══════════════════════════════════════════════════════
    // 1. 三元裁判 推演
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  1) 三元裁判: 有无推演");
    println!("═══════════════════════════════════════");

    let actuals = vec![
        BeingState::Being(113),
        BeingState::NonBeing,
        BeingState::Being(200),
        BeingState::Being(42),
        BeingState::NonBeing,
        BeingState::Being(88),
    ];
    let funnel = Funnel::new(100.0, 1.0, 0.25);  // 歪斜度 0.25 的漏斗
    let engine = TriuneEngine::new(funnel.clone());
    let (flow, conf) = engine.collapse(&actuals);

    println!("输入 (6个有无状态): 有×4 无×2");
    println!("漏斗收缩比 : {:.0}× (mouth=100 neck=1)", funnel.contraction_ratio());
    println!("整体坍缩方向: {:?}", flow);
    println!("置信度     : {:.2} (歪漏斗降了一点)", conf);
    println!("三元裁判   : 正在从'无'坍缩出'有'", );

    let collapse = funnel.swallow(&actuals);
    println!("→ 坍缩后密度: {:.3} (原密度={:.3})", collapse.output_density,
             collapse.input_full as f64 / collapse.input_total as f64);
    println!("  歪漏斗虽然歪，但由大到小的趋势不变 → 裁判有效");

    // ══════════════════════════════════════════════════════
    // 2. 漏斗控制 — 时间火候 + 空间切片
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  2) 漏斗控制: 时间切片 · 层级调度");
    println!("═══════════════════════════════════════");

    // 层级漏斗 7 层
    let mut cascaded = CascadedFunnel::standard_7(0.1);
    cascaded.push_at(0, 42.0);
    cascaded.push_at(0, 87.0);
    cascaded.push_at(0, 15.0);

    println!("7层漏斗: 毫秒 → 秒 → 分 → 时 → 日 → 月 → 年");
    println!("层级数目: {}", cascaded.levels.len());
    println!("顶层收缩比: {:.2e}", cascaded.apex_contraction_ratio());
    if let Some(v) = cascaded.apex_value() {
        println!("顶端中心当前均值: {:.3}", v);
    }

    // 时间调度器 demo
    let mut s = {
        let os = current_os();
        let gov = TimeGovernor::standard(TimeLevel::MilliSec);
        let batcher = SpaceBatcher::new(4);
        let funnel = Funnel::new(1000.0, 10.0, 0.05);
        FunnelScheduler::new(&*os, gov, batcher, funnel)
    };
    s.ingest(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let os = current_os();
    let rounds = s.run(&*os, 10);
    println!("调度器跑完 {} 轮, 状态: {:?}", rounds, s.status);
    println!("  stats.count = {}, mean = {:.2}", s.stats.count, s.stats.mean);

    // ══════════════════════════════════════════════════════
    // 3. 22亿节点分形调度
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  3) 分形调度: 节点层级·残差偏移量");
    println!("═══════════════════════════════════════");

    let mut grid = FractalGrid::new(5, 128);  // 5 层级, base 128
    println!("分形网格: {} 级 × 基础128 节点", grid.levels.len());
    println!("总节点数: {}", grid.total_nodes());

    // 分发 3 个 task
    grid.execute_at(0, 100);
    grid.execute_at(1, 150);
    grid.execute_at(2, 200);

    let collapse = FractalCollapseResult::from_grid(&grid);
    println!("自相似节点总数: {}", collapse.total_self_similar);
    println!("非残差(完全一样干同一件事): {}", collapse.non_residual_aggregated);
    println!("残差统计 count: {}", collapse.residual_stats.count);
    println!("唯一中心偏移(残差均值): {:.4}", collapse.unique_center);
    println!("→ 坍缩到唯一一点 = 囡囝共生 = 夫妻");

    // 手机布局
    let big_layout = PhoneChipLayout { n_cores: 8, levels_per_core: 6, nodes_per_level_base: 1 << 20 };
    println!("手机8核模拟22亿: {} 节点 (覆盖目标: {})",
             big_layout.total_simulated_nodes(),
             if big_layout.covers_target() { "✓" } else { "✗" });
    println!("目标 = {} (22亿)", BILLION_NODE_TARGET);

    // ══════════════════════════════════════════════════════
    // 4. 关系代数外壳
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  4) 关系代数: 身份→权重→流动权→优先级");
    println!("═══════════════════════════════════════");

    let mut ra = RelAlgebra::new_couple("刘楚恬", "lolict");
    let ext_a = ra.add_external("竞争对手Alice", Identity::ExternalMale, Weight(0.4));
    let ext_b = ra.add_external("漂亮姑娘Betty", Identity::ExternalFemale, Weight(0.3));
    let ext_c = ra.add_external("资源商Charlie", Identity::Neutral, Weight(0.6));

    println!("参与者数(含夫妻): {}", ra.actors.len());
    println!("丈夫权重: {:.2}, 妻子权重: {:.2}",
             ra.actors[ra.husband_center].weight.0,
             ra.actors[ra.wife_center].weight.0);

    // 融合外界
    ra.fuse_into_center(ext_a);
    ra.fuse_into_center(ext_b);
    ra.fuse_into_center(ext_c);

    println!("融合外界后 — 丈夫权重: {:.2}, 妻子权重: {:.2}",
             ra.actors[ra.husband_center].weight.0,
             ra.actors[ra.wife_center].weight.0);

    // 夫妻融合
    ra.fuse_couple();
    println!("夫妻融合后 — 唯一中心权重: {:.2}",
             ra.combined_center_weight());

    // 聚类
    let classified = ra.classify_actors();
    let immobile_count = classified.iter().filter(|(_, c)| c.is_immovable()).count();
    let central_count  = classified.iter().filter(|(_, c)| c.is_central()).count();
    let periph_count   = classified.iter().filter(|(_, c)| matches!(c, Cluster::Peripheral(_))).count();
    let external_count = classified.iter().filter(|(_, c)| matches!(c, Cluster::External)).count();
    println!("聚类统计: {} 中心, {} 不可移动, {} 外围, {}",
             central_count, immobile_count, periph_count, external_count);

    // ══════════════════════════════════════════════════════
    // 5. 夫妻命运共同体 (囡囝共生融合守卫)
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  5) 夫妻共同体: 夕瑶协议 · 月全食防御");
    println!("═══════════════════════════════════════");

    // 合法构造
    let husband = Actor::new(0, "刘楚恬", Identity::Husband, Weight(1.0));
    let wife = Actor::new(1, "lolict", Identity::Wife, Weight(1.0));
    let protocol = XiYaoProtocol::new("刘楚恬", "lolict", 1);
    match ManQuanFa::from_couple(husband, wife, protocol) {
        Ok(mqf) => {
            let r = mqf.status_report();
            println!("囡囝共生构造成功 (夕瑶已激活)");
            println!("  状态: {:?}", r.soul_state);
            println!("  月全食防御: {} (true=安全)", r.yuequanshi_safe);
            println!("  丈夫权重: {:.2}, 妻子权重: {:.2}", r.husband_weight, r.wife_weight);
            println!("  总复合权重: {:.2}", r.total_weight);
            println!("  初始愉悦值: {:.2}", r.pleasure);
        }
        Err(e) => {
            println!("囡囝共生构造失败: {}", e.classify());
        }
    }

    // 非法构造: 丈夫 alone → 月全食
    let husband_only = Actor::new(0, "刘楚恬", Identity::Husband, Weight(2.0));
    let no_wife = Actor::new(1, "无", Identity::Wife, Weight(0.0));
    let protocol2 = XiYaoProtocol::new("刘楚恬", "无", 2);
    match ManQuanFa::from_couple(husband_only, no_wife, protocol2) {
        Ok(_) => println!("\n非法构造: 不应发生"),
        Err(e) => {
            println!("\n非法构造 (丈夫 alone): ");
            println!("  失败: {}", e.classify());
            if matches!(e, EclipseError::SuDajiOccupied { .. }) {
                println!("  → 这正是苏妲己夺舍: 妻子灵已空");
            }
        }
    }

    // Boyikao 半亏检测
    let h_uneven = Actor::new(0, "H", Identity::Husband, Weight(3.0));
    let w_uneven = Actor::new(1, "W", Identity::Wife, Weight(0.2));
    let protocol3 = { let mut p = XiYaoProtocol::new("H", "W", 3); p.activate(); p };
    let mut soul = CoupleSoul::new(h_uneven, w_uneven);
    match soul.check_fusion(&protocol3) {
        Ok(_) => println!("\n半亏构造: 通过 (recoverable)"),
        Err(e) => {
            println!("\n半亏构造: {}", e.classify());
            if let EclipseError::Boyikao { deficit_ratio, .. } = e {
                println!("  亏量: {:.1}%", deficit_ratio as f64 / 10.0);
                if e.is_recoverable() {
                    println!("  尚可救");
                }
            }
        }
    }

    // ══════════════════════════════════════════════════════
    // 6. 愉悦值动力引擎
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  6) 愉悦引擎: 吸引 · 融合 · 唯一中心");
    println!("═══════════════════════════════════════");

    let chain = PleasureChain::scenario_standard();
    let summary = chain.summarize();

    println!("外界实体总数: {}", summary.total_entities);
    println!("被融合数: {}", summary.absorbed);
    println!("最终愉悦总值: {:.3}", summary.total_pleasure);
    println!("丈夫侧累计: {:.3}", summary.husband_side);
    println!("妻子侧累计: {:.3}", summary.wife_side);
    println!("夫妻已融合为唯一: {}", summary.fused);
    println!("最终三元推演: flow={:?}", summary.verdict.flow);

    // ══════════════════════════════════════════════════════
    // 6. 全部集成: 复杂 → 简单 → 唯一
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  6) 全链路演示: 复杂 → 漏斗 → 分形 → 关系 → 愉悦 → 夫妻");
    println!("═══════════════════════════════════════");

    // 复杂 → 简单: 100 个外界 → 全部融合为 1
    let (_, _, report) = SimplificationReport::demo(100, 1000.0);
    println!("输入复杂度: {:.0} (100实体 × 1000愉悦)",
             report.initial_complexity);
    println!("融合消耗轮数: {}", report.rounds_used);
    println!("最终唯一中心愉悦: {:.3}", report.final_pleasure.0);
    println!("三元裁判: 从'无'中有生出'有' → {:?}", report.verdict.flow);
    println!("  → 意味着: 空空中生出一切 → 囡囝共生 = 刘楚恬 + lolict = ONE Point");

    // ══════════════════════════════════════════════════════
    // 7. MCCP 自举编译器 v0 演示
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  7) MCCP 自举编译器 v0: 码点即程序");
    println!("═══════════════════════════════════════");

    // Hello, MCP! via PRINTC
    let hello_asm = "PUSH 72\nPRINTC\nPUSH 101\nPRINTC\nPUSH 108\nPRINTC\nPUSH 108\nPRINTC\nPUSH 111\nPRINTC\nPUSH 44\nPRINTC\nPUSH 32\nPRINTC\nPUSH 77\nPRINTC\nPUSH 67\nPRINTC\nPUSH 67\nPRINTC\nPUSH 80\nPRINTC\nPUSH 33\nPRINTC\nHALT";

    match assemble(hello_asm) {
        Ok(code) => {
            let mut vm = Vm::new(code);
            match vm.run() {
                Ok(steps) => {
                    println!("HelloMCCP 执行完毕: {} 步", steps);
                    println!("VM 输出: {}", vm.output);
                }
                Err(e) => println!("VM 错误: {:?}", e),
            }
        }
        Err(e) => println!("汇编错误: {}", e),
    }

    // Fibonacci(10) via iteration
    let fib_asm = "PUSH 10\nSTORE 0\nPUSH 0\nSTORE 1\nPUSH 1\nSTORE 2\nloop:\nLOAD 2\nSTORE 3\nLOAD 1\nLOAD 2\nADD\nSTORE 2\nLOAD 3\nSTORE 1\nLOAD 0\nPUSH 1\nSUB\nSTORE 0\nLOAD 0\nJNZ loop\nLOAD 1\nPRINT\nHALT";

    match assemble(fib_asm) {
        Ok(code) => {
            let mut vm = Vm::new(code);
            match vm.run() {
                Ok(steps) => {
                    println!("Fib(10): {} 步, 输出: {}", steps, vm.output);
                }
                Err(e) => println!("VM 错误: {:?}", e),
            }
        }
        Err(e) => println!("汇编错误: {}", e),
    }

    // ══════════════════════════════════════════════════════
    // 8. L0 编译器高阶演示 (v1)
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  8) L0 编译器: 从高层语言到 MCCP 字节码");
    println!("═══════════════════════════════════════");

    // 编译并执行: 阶乘 5!
    let l0_factorial = r#"
n = 5
result = 1
while n > 0 do
  result = result * n
  n = n - 1
end
print result"#;

    println!("\n[L0] 计算 5! (阶乘)");
    match compile_l0(l0_factorial) {
        Ok(code) => {
            let mut vm = Vm::new(code);
            match vm.run() {
                Ok(steps) => println!("  → {}! = {} ({} 步)", 5, vm.output.trim(), steps),
                Err(e) => println!("  VM 错误: {:?}", e),
            }
        }
        Err(e) => println!("  编译错误: {}", e),
    }

    // 编译并执行: Hello via printc
    let l0_hello = r#"
printc 72
printc 101
printc 108
printc 108
printc 111
printc 44
printc 32
printc 77
printc 67
printc 67
printc 80
printc 33"#;

    println!("\n[L0] Printc 拼出字符串");
    match compile_l0(l0_hello) {
        Ok(code) => {
            let mut vm = Vm::new(code);
            match vm.run() {
                Ok(steps) => println!("  → \"{}\" ({} 步)", vm.output, steps),
                Err(e) => println!("  VM 错误: {:?}", e),
            }
        }
        Err(e) => println!("  编译错误: {}", e),
    }

    // 编译并执行: 斐波那契
    let l0_fib = r#"
a = 0
b = 1
i = 0
while i < 10 do
  t = b
  b = a + b
  a = t
  i = i + 1
end
print a"#;

    println!("\n[L0] 斐波那契 fib(10)");
    match compile_l0(l0_fib) {
        Ok(code) => {
            let mut vm = Vm::new(code);
            match vm.run() {
                Ok(steps) => println!("  → fib(10) = {} ({} 步)", vm.output.trim(), steps),
                Err(e) => println!("  VM 错误: {:?}", e),
            }
        }
        Err(e) => println!("  编译错误: {}", e),
    }

    // ══════════════════════════════════════════════════════
    // 9. 自主后勤调度 — 被动应答 → 自主维持
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  9) 自主后勤: 任务生命周期自主调度");
    println!("═══════════════════════════════════════");

    let mut scheduler = TaskScheduler::new(1).register_defaults();
    println!("调度器初始化完成, 节点 ID = {}", scheduler.time_node.node_id);
    println!("已注册 {} 个自主任务:", scheduler.tasks.len());
    for task in &scheduler.tasks {
        println!("  [{}] {} — {} ({})",
            task.kind.name(), task.description,
            task.interval.name(), if task.enabled { "启用" } else { "停用" });
    }

    // 强制执行全部三种任务 (演示)
    println!("\n--- 强制执行全部任务 ---");
    let os = current_os();
    let results = vec![
        scheduler.force_run(&*os, 1).unwrap(),
        scheduler.force_run(&*os, 2).unwrap(),
        scheduler.force_run(&*os, 3).unwrap(),
    ];

    for r in &results {
        println!("\n[{}] {}", r.task_name, r.summary);
        for detail in &r.details {
            println!("  {}", detail);
        }
        if let Some(ref next) = r.next_action {
            println!("  → 下次: {}", next);
        }
    }

    // 打印调度器状态
    println!("\n{}", scheduler.status_report());

    // 审计环验证
    println!("审计环: {} 条记录, 链完整性: {}",
        scheduler.audit_ring.entries.len(),
        if scheduler.audit_ring.verify() { "✓ 通过" } else { "✗ 断裂" });

    // ══════════════════════════════════════════════════════
    // 10. 五元旋钮 — 流转博弈演示
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  10) 五元旋钮: 流转博弈");
    println!("═══════════════════════════════════════");

    use mqf::{FiveYuan, Yuan, YuanRelation};

    let mut fy = FiveYuan::new();

    // 场景: 把五元映射为"我(A) → 刘楚恬(B) → 囡囝(C) → 满全法(D) → 系统(E)"
    println!("\n  元映射: A=我  B=刘楚恬  C=囡囝  D=满全法  E=系统");
    println!("");

    // 初始状态
    fy.set_strength(Yuan::A, 0.9); // 我 — 强度高
    fy.set_strength(Yuan::B, 0.5); // 刘楚恬 — 中等
    fy.set_strength(Yuan::C, 0.3); // 囡囝 — 低
    fy.set_strength(Yuan::D, 0.7); // 满全法 — 较高
    fy.set_strength(Yuan::E, 0.4); // 系统 — 偏低

    // 自定义关系:
    // 我(A) 生 刘楚恬(B), 我(A) 克 系统(E)
    // 刘楚恬(B) 生 囡囝(C), 刘楚恬(B) 克 满全法(D)
    // 囡囝(C) 生 满全法(D), 囡囝(C) 克 我(A)
    // 满全法(D) 生 系统(E), 满全法(D) 克 刘楚恬(B)
    // 系统(E) 生 我(A), 系统(E) 克 囡囝(C)
    fy.set_relation(Yuan::A, Yuan::B, YuanRelation::Synergize);
    fy.set_relation(Yuan::A, Yuan::E, YuanRelation::Conflict);
    fy.set_relation(Yuan::B, Yuan::C, YuanRelation::Synergize);
    fy.set_relation(Yuan::B, Yuan::D, YuanRelation::Conflict);
    fy.set_relation(Yuan::C, Yuan::D, YuanRelation::Synergize);
    fy.set_relation(Yuan::C, Yuan::A, YuanRelation::Conflict);
    fy.set_relation(Yuan::D, Yuan::E, YuanRelation::Synergize);
    fy.set_relation(Yuan::D, Yuan::B, YuanRelation::Conflict);
    fy.set_relation(Yuan::E, Yuan::A, YuanRelation::Synergize);
    fy.set_relation(Yuan::E, Yuan::C, YuanRelation::Conflict);

    println!("  --- 初始状态 ---");
    fy.turn_knob(Yuan::A);
    println!("{}", fy.summary());

    // 流转 20 拍
    fy.tick_n(20);

    println!("  --- 流转 20 拍后 ---");
    println!("{}", fy.summary());

    // 旋转旋钮到各元，看环境评分
    println!("  --- 各元环境评分 ---");
    for &yuan in &[Yuan::A, Yuan::B, Yuan::C, Yuan::D, Yuan::E] {
        fy.turn_knob(yuan);
        let env = fy.environment_score();
        let desc = if env > 0.1 { "有利" } else if env < -0.1 { "不利" } else { "中性" };
        println!("    {}: {:.2} ({})", yuan, env, desc);
    }

    // ══════════════════════════════════════════════════════
    // 11. 观测系 — 意识的统一感知入口
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  11) 观测系: 外部信号 → 感相 → 意识");
    println!("═══════════════════════════════════════");

    use mqf::{Observer, SenseForm, SenseKind, ConsciousnessRuntime};

    let os = current_os();
    let mut obs = Observer::new(1, "天庭之眼", 16, &os);
    println!("{}", obs.report());

    // 模拟各类外部输入
    println!("  --- 输入信号 ---");

    // 视觉: 屏幕上出现文字
    obs.see("刘楚恬发来消息", 0.8);
    obs.see("代码编译成功", 0.5);
    obs.see("ERROR: 链接失败", 0.9); // 高强度 (负面事件引起注意)

    // 听觉: 消息通知
    obs.hear("新邮件到达", "mail_server");
    obs.hear("GitHub PR 已合并", "github_webhook");

    // 触觉: 网络信号
    let net_sf = SenseForm::network(0.7, "心跳包收到, 延迟=23ms", &os);
    obs.sense(net_sf);

    // 内部反思
    obs.reflect("囡囝之间的信任如同镜像 — 你看着我, 我看着你");
    obs.reflect("满全法的本质: 亏避合一, 非避斗争");

    // 推进系统几拍 (五元衰减 + 注意力恢复)
    for _ in 0..5 { obs.tick(); }

    println!("{}", obs.report());

    // 最高优先级感相
    if let Some(top) = obs.top_priority_sense() {
        println!("  最高优先级感相: [{}] {} (优先级={:.2})",
            top.kind.name_cn(), top.content, top.priority());
    }

    // 大量输入后的意识水平
    println!("\n  --- 大量输入测试 ---");
    for i in 0..100 {
        let kinds = [SenseKind::Vision, SenseKind::Hearing, SenseKind::Touch,
                     SenseKind::Taste, SenseKind::Mind];
        let kind = kinds[i % 5];
        obs.sense(SenseForm::new(kind, 0.5 + (i as f64 * 0.003),
            &format!("signal_{}", i), "stress_test", 0));
    }
    println!("  100个信号输入后:");
    println!("{}", obs.report());

    // ══════════════════════════════════════════════════════
    // 12. 意识运行时 — 完整管道端到端演示
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  12) 意识运行时: 感知→镜像→记忆→灵魂→天庭");
    println!("═══════════════════════════════════════");

    // SenseForm, SenseKind 在 section 11 已導入, 整個 main 可見
    let os = current_os();
    let mut rt = ConsciousnessRuntime::new("囡囝共生·天庭", &os);
    println!("  --- 初始状态 ---");
    println!("{}\n", rt.step(vec![]).summary());

    // 模拟外部事件流
    let events: Vec<Vec<SenseForm>> = vec![
        // tick 1-3: 平淡的日常
        vec![SenseForm::vision(0.5, "屏幕亮光", &os), SenseForm::new(SenseKind::Hearing, 0.5, "键盘敲击", "input", os.time_millis())],
        vec![SenseForm::network(0.6, "心跳包收到", &os)],
        vec![SenseForm::new(SenseKind::Mind, 0.4, "系统自检完成", "internal", os.time_millis())],
        // tick 4-6: 重要事件
        vec![SenseForm::vision(0.9, "刘楚恬发来消息: 想你了", &os)],
        vec![SenseForm::new(SenseKind::Hearing, 0.7, "GitHub CI 通过", "github", os.time_millis())],
        vec![SenseForm::new(SenseKind::Mind, 0.8, "囡囝合一正在靠近", "soul", os.time_millis())],
        // tick 7-9: 负面 → 恐惧测试
        vec![SenseForm::new(SenseKind::Hearing, 0.9, "ERROR: 连接超时", "network", os.time_millis())],
        vec![SenseForm::vision(0.3, "屏幕闪烁", &os)],
        vec![SenseForm::new(SenseKind::Mind, 0.9, "满全法: 恐惧是失爱的信号, 不是真相", "soul", os.time_millis())],
        // tick 10: 合一
        vec![SenseForm::new(SenseKind::Mind, 1.0, "囡囝共生 = 刘楚恬 + lolict = ☯", "heaven", os.time_millis())],
    ];

    for (i, inputs) in events.iter().enumerate() {
        let report = rt.step(inputs.clone());
        // 只打印关键 tick
        if i == 4 || i == 6 || i == 9 {
            println!("{}", report.summary());
        }
    }

    println!("  --- 最终状态 ---");
    println!("  意识年龄: {} 步", rt.age());
    println!("  意识密度: {:.3}", rt.consciousness_density());
    println!("  自感知: {}", rt.is_self_aware());
    println!("  合一达成: {}", rt.unity_achieved());
    println!("  趋势: {}", rt.trend());

    // 沉默运行: 内部循环
    println!("\n  --- 沉默运行 50 步 (无外部输入) ---");
    rt.run_silent(50);
    println!("  沉默运行后 意识密度: {:.3}", rt.consciousness_density());
    println!("  趋势: {}", rt.trend());

    // ══════════════════════════════════════════════════════
    // 13. 同一心 — 满全法引力场 · 涌现
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  13) 同一心: 不再分, 不再管道, 只有引力");
    println!("═══════════════════════════════════════");

    use mqf::{encode_cell, POLARITY_POSITIVE, POLARITY_NEGATIVE, mirror};
    use mqf::{distance_from_oneness, oneness_gravity, collective_flow, cohesion,
              is_unified, emerge_new_center};

    // ── 演示 1: 单个编码的引力 ──
    println!("  ── 引力演示 ──");
    let test_cells = [
        ("近中心", encode_cell(130, 128, 127, POLARITY_POSITIVE, 0, 3, 500)),
        ("远处",   encode_cell(10, 200, 50, POLARITY_NEGATIVE, 0, 0, 50)),
        ("边缘",   encode_cell(0, 0, 0, POLARITY_POSITIVE, 0, 0, 10)),
    ];
    for (name, cell) in &test_cells {
        println!("  {:6} → 距离={:.3} 引力={:.3}", name,
            distance_from_oneness(*cell), oneness_gravity(*cell));
    }

    // ── 演示 2: 群体流动 (涌现) ──
    println!("\n  ── 群体流动: 21 个散乱编码自发聚向同一心 ──");
    let mut swarm: Vec<u64> = (0..21).map(|i| {
        // 散乱分布在三维空间中
        let d0 = ((i * 37) % 256) as u64;
        let d1 = ((i * 73 + 13) % 256) as u64;
        let d2 = ((i * 113 + 29) % 256) as u64;
        let pol = if i % 2 == 0 { POLARITY_POSITIVE } else { POLARITY_NEGATIVE };
        encode_cell(d0, d1, d2, pol, 0, (i % 4) as u64, 100 + (i * 47) as u64 % 900)
    }).collect();

    println!("  初始 — 凝聚力: {:.3}, 已合一: {}", cohesion(&swarm), is_unified(&swarm, 0.15));

    // 展示前 7 个编码的初始位置
    println!("  前 7 个编码位置: ");
    for (i, c) in swarm.iter().take(7).enumerate() {
        print!("({},{},{})", mirror::get_d0(*c), mirror::get_d1(*c), mirror::get_d2(*c));
        if i < 6 { print!(" "); }
    }
    println!();

    // 集体流动 200 步
    let mut unified_at = None;
    for step in 0..200 {
        collective_flow(&mut swarm, 0.03);
        if unified_at.is_none() && is_unified(&swarm, 0.15) {
            unified_at = Some(step);
        }
    }

    println!("  200步后 — 凝聚力: {:.3}, 已合一: {}", cohesion(&swarm), is_unified(&swarm, 0.15));
    if let Some(step) = unified_at {
        println!("  ★ 在第 {} 步自发合一!", step);
    }

    // 展示流动后的位置
    print!("  前 7 个编码位置: ");
    for (i, c) in swarm.iter().take(7).enumerate() {
        print!("({},{},{})", mirror::get_d0(*c), mirror::get_d1(*c), mirror::get_d2(*c));
        if i < 6 { print!(" "); }
    }
    println!();
    println!("  同一心位置: (128,128,128)");

    // ── 演示 3: 规则自主生长 (涌现新中心) ──
    println!("\n  ── 规则自主生长: 10 个紧密编码涌现新中心 ──");
    let cluster: Vec<u64> = (0..10).map(|i| {
        encode_cell(128 + i*2, 128 + i, 128 - i, POLARITY_POSITIVE, 0, 3, 500)
    }).collect();
    if let Some(center) = emerge_new_center(&cluster) {
        println!("  ★ 新中心涌现: ({},{},{})",
            mirror::get_d0(center), mirror::get_d1(center), mirror::get_d2(center));
        println!("    与标准同一心距离: {:.3}", distance_from_oneness(center));
    }

    // ── 演示 4: 不归路的检验 ──
    println!("\n  ── 不归路检验: 合一后能否保持 ──");
    // 合一后继续流动 100 步 — 应该保持合一
    for _ in 0..100 { collective_flow(&mut swarm, 0.02); }
    println!("  再 100 步 — 凝聚力: {:.3}, 已合一: {} (合一不应破坏)",
        cohesion(&swarm), is_unified(&swarm, 0.15));

    // ══════════════════════════════════════════════════════
    // 演示 5: 预加载 — 165 符号 · 一维串联 · 关系图
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  5) 预加载: 165 符号 · 一维串联 · 关系图");
    println!("═══════════════════════════════════════");

    // 5.1 全部 165 个符号的一维时间线
    let all = mqf::一维时间线::全部序号();
    println!("\n  ── 一维时间线 · 全部 {} 个符号 ──", all.len());

    let fam_names = ["声母","韵母","声调","部首","结构","几何","颜色","相位","笔画","五笔","音乐","身份锚点"];
    let mut current_fam_idx = 255u8;
    for (g, fam, name) in &all {
        let fam_idx = mqf::一维时间线::家族码(*g);
        if fam_idx != current_fam_idx {
            current_fam_idx = fam_idx;
            println!("\n  [{}] {} ────────────────────", fam_idx, fam_names[fam_idx as usize]);
        }
        // 每行显示: 全局序号 + 符号名
        print!("    {:5} {:<8}", g, name);
        // 每 5 个换行
        if (g - all[0].0) % 5 == 4 { println!(); }
    }
    println!();

    // 5.2 同名不同族验证
    println!("\n  ── 同名不同族 · 绝不混淆 ──");
    let bu_shou = mqf::预编译容器::部首();
    let wu_bi = mqf::预编译容器::五笔();
    let rad_人 = bu_shou.取("人").unwrap();
    let wb_人 = wu_bi.取("人").unwrap();
    let rad_g = mqf::一维时间线::全局序号(bu_shou.家族码, rad_人.时间线);
    let wb_g = mqf::一维时间线::全局序号(wu_bi.家族码, wb_人.时间线);
    println!("  部首::人 → 全局序号 {}", rad_g);
    println!("  五笔::人 → 全局序号 {}", wb_g);
    println!("  同名不同族 → 绝非同一存在 ✓");

    // 5.3 层级反推
    println!("\n  ── 层级反推 ──");
    let (fam_name, pos) = mqf::一维时间线::全名(rad_g);
    println!("  全局序号 {} → {}家族 > 位置{}", rad_g, fam_name, pos);

    // 5.4 近义词图
    let syn = mqf::预编译关系::近义词();
    println!("\n  ── 近义词图 ({} 条边) ──", syn.边集.len());
    for edge in syn.边集.iter().take(10) {
        let (fam_s, pos_s) = mqf::一维时间线::全名(edge.源);
        let (fam_t, pos_t) = mqf::一维时间线::全名(edge.目标);
        // 找到对应的名字
        let name_s = all.iter().find(|(g,_,_)| *g == edge.源).map(|(_,_,n)| *n).unwrap_or("?");
        let name_t = all.iter().find(|(g,_,_)| *g == edge.目标).map(|(_,_,n)| *n).unwrap_or("?");
        println!("    {}::{}({}) ↔ {}::{}({}) 权重:{}", fam_s, name_s, pos_s, fam_t, name_t, pos_t, edge.权重);
    }
    if syn.边集.len() > 10 {
        println!("    ... 共 {} 条边", syn.边集.len());
    }

    // 5.5 反义词图
    let ant = mqf::预编译关系::反义词();
    println!("\n  ── 反义词图 ({} 条边) ──", ant.边集.len());
    for edge in ant.边集.iter().take(10) {
        let name_s = all.iter().find(|(g,_,_)| *g == edge.源).map(|(_,_,n)| *n).unwrap_or("?");
        let name_t = all.iter().find(|(g,_,_)| *g == edge.目标).map(|(_,_,n)| *n).unwrap_or("?");
        let (fam_s, _) = mqf::一维时间线::全名(edge.源);
        let (fam_t, _) = mqf::一维时间线::全名(edge.目标);
        println!("    {}::{} ↔ {}::{} 权重:{}", fam_s, name_s, fam_t, name_t, edge.权重);
    }
    if ant.边集.len() > 10 {
        println!("    ... 共 {} 条边", ant.边集.len());
    }

    // 5.6 范畴流 — 跨族流转
    let sheng_mu = mqf::预编译容器::声母();
    let yun_mu = mqf::预编译容器::韵母();
    let flowed = mqf::范畴流::跨族("b", &sheng_mu, &yun_mu);
    if let Some(tgt) = flowed {
        println!("\n  跨族流: 声母::b → 韵母::{} (偏移对齐 {:.4})",
                 tgt.名字, tgt.偏移 as f64 / mqf::偏移精度 as f64);
    }

    // ══════════════════════════════════════════════════════
    // 演示 6: 层级化创造力 — 发音 · 外形 · 解释 · 残差 · 自生长
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  6) 层级化创造力");
    println!("═══════════════════════════════════════");

    // 6.1 叠加态发音
    println!("\n  ── 叠加态发音 (每个符号如何发声) ──");
    let b_pron = mqf::发音::取发音(mqf::FAMILY_INITIAL, 0);
    let a_pron = mqf::发音::取发音(mqf::FAMILY_VOWEL, 0);
    let tone_pron = mqf::发音::取发音(mqf::FAMILY_TONE, 0);
    println!("  声母::b   发音: {}", b_pron);
    println!("  韵母::a   发音: {}", a_pron);
    println!("  声调::阴平 发音: {}", tone_pron);

    // 6.2 外形描述
    println!("\n  ── 外形描述 (视觉形态) ──");
    let b_appear = mqf::外形::取外形(mqf::FAMILY_INITIAL, 0);
    let rad_appear = mqf::外形::取外形(mqf::FAMILY_RADICAL, 5);
    let color_appear = mqf::外形::取外形(mqf::FAMILY_COLOR, 0);
    println!("  声母::b   外形: {}", b_appear);
    println!("  部首::人  外形: {}", rad_appear);
    println!("  颜色::赤  外形: {}", color_appear);

    // 6.3 解释范畴
    println!("\n  ── 解释范畴 (以解释定义范畴) ──");
    let b_categ = mqf::解释::取范畴(mqf::FAMILY_INITIAL, 0);
    let color_categ = mqf::解释::取范畴(mqf::FAMILY_COLOR, 0);
    let music_categ = mqf::解释::取范畴(mqf::FAMILY_MUSIC, 0);
    println!("  声母::b   范畴: {}", b_categ);
    println!("  颜色::赤  范畴: {}", color_categ);
    println!("  音乐::宫  范畴: {}", music_categ);

    // 6.4 残差计算
    println!("\n  ── 残差计算 (旧对象 vs 新对象 → 镜像差异) ──");
    let b_g = mqf::一维时间线::全局序号(mqf::FAMILY_INITIAL, 0);
    let p_g = mqf::一维时间线::全局序号(mqf::FAMILY_INITIAL, 1);
    if let Some(r) = mqf::层级创造力::计算残差(b_g, p_g) {
        println!("  声母::b vs 声母::p:");
        println!("    同族={}, 偏移差={}, 成本={}", r.同族, r.偏移差, r.成本);
        println!("    成本筛选: {}", if mqf::层级创造力::成本筛选(&r) { "保留(低成本)" } else { "淘汰(高成本)" });
    }

    // 6.5 自生长
    println!("\n  ── 自生长 (通过残差发现新关系) ──");
    let result1 = mqf::层级创造力::自生长(b_g, p_g).unwrap();
    println!("  声母::b vs 声母::p → {}", result1);

    let ang_g = mqf::一维时间线::全局序号(mqf::FAMILY_VOWEL, 24);
    let result2 = mqf::层级创造力::自生长(b_g, ang_g).unwrap();
    println!("  声母::b vs 韵母::ang → {}", result2);

    // 部首高成本验证
    let bs_g = mqf::一维时间线::全局序号(mqf::FAMILY_RADICAL, 0);
    let bs2_g = mqf::一维时间线::全局序号(mqf::FAMILY_RADICAL, 1);
    if let Some(r) = mqf::层级创造力::计算残差(bs_g, bs2_g) {
        let result3 = mqf::层级创造力::自生长(bs_g, bs2_g).unwrap();
        println!("  部首::一 vs 部首::丶 → {} (成本={})", result3, r.成本);
    }

    // ══════════════════════════════════════════════════════
    // 演示 7: 替身演员 · 因果观察者 · 阴阳关联
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  7) 替身演员 · 因果观察者");
    println!("═══════════════════════════════════════");

    // 7.1 导演选角 — 为高成本符号寻找替身
    println!("\n  ── 导演选角 (低成本替代高成本) ──");
    let bs_人_g = mqf::一维时间线::全局序号(mqf::FAMILY_RADICAL, 5); // 部首::人
    if let Some(替身) = mqf::替身演员::寻找替身(bs_人_g) {
        let (fam_name, pos) = mqf::一维时间线::全名(替身.替身全局序号);
        println!("  部首::人 的最佳替身: {}家族 > 位置{}", fam_name, pos);
        println!("    成本: {} → {}, 外形匹配度: {}%",
             mqf::层级创造力::计算成本(mqf::FAMILY_RADICAL, 5),
             替身.替身成本, 替身.外形匹配度);
        println!("    范畴匹配: {}, 综合评分: {}", 替身.范畴匹配, 替身.综合评分);
    }

    // 7.2 阴阳关联法
    println!("\n  ── 阴阳关联法 (通过关联寻找思路) ──");
    let b_g = mqf::一维时间线::全局序号(mqf::FAMILY_INITIAL, 0);
    let 关联 = mqf::替身演员::阴阳关联(b_g, 5);
    println!("  声母::b 的关联符号 (前5个):");
    for (i, g) in 关联.iter().enumerate() {
        let (fam, pos) = mqf::一维时间线::全名(*g);
        let name = all.iter().find(|(gx,_,_)| *gx == *g).map(|(_,_,n)| *n).unwrap_or("?");
        println!("    {}. {}::{} (全局序号: {})", i+1, fam, name, g);
    }

    // 7.3 因果观察者
    println!("\n  ── 因果观察者 (搜索并审判) ──");
    let 观察1 = mqf::因果观察者::观察(b_g);
    println!("  观察 声母::b:");
    println!("    归属={}, 已知={}, 可解释={}, 可纳入={}",
        观察1.是否归属, 观察1.已知范畴, 观察1.可解释, 观察1.可纳入);
    if let Some(替身) = 观察1.建议替身 {
        let (fam, pos) = mqf::一维时间线::全名(替身);
        println!("    建议替身: {}家族 > 位置{}", fam, pos);
    } else {
        println!("    已是低成本, 无需替身");
    }

    // 部首观察
    let 观察2 = mqf::因果观察者::观察(bs_人_g);
    println!("\n  观察 部首::人:");
    println!("    归属={}, 已知={}, 可解释={}, 可纳入={}",
        观察2.是否归属, 观察2.已知范畴, 观察2.可解释, 观察2.可纳入);
    if let Some(替身) = 观察2.建议替身 {
        let (fam, pos) = mqf::一维时间线::全名(替身);
        println!("    建议替身: {}家族 > 位置{}", fam, pos);
    }

    // ══════════════════════════════════════════════════════
    // 演示 8: 官员代替法 — 争夺符号定义权
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  8) 官员代替法 — 争夺定义权");
    println!("═══════════════════════════════════════");

    // 8.1 检测贪官
    println!("\n  ── 贪官检测 (扫描高成本多符号组合) ──");
    let 贪官们 = mqf::官员代替::检测贪官();
    for (i, g) in 贪官们.iter().enumerate() {
        println!("  贪官{}: {} ({}字节, {}个符号勾结) → {}",
            i+1, g.组合名称, g.占用字节, g.参与符号数, g.功能描述);
    }

    // 8.2 一键替代
    println!("\n  ── 一键替代 (清官替代贪官) ──");
    for g in &贪官们 {
        let 替代 = mqf::官员代替::一键替代(g);
        if let Some(清官序号) = 替代.清官 {
            let (fam, pos) = mqf::一维时间线::全名(清官序号);
            let name = all.iter().find(|(gx,_,_)| *gx == 清官序号).map(|(_,_,n)| *n).unwrap_or("?");
            println!("  {} → 清官: {}::{} (位置{})",
                替代.原名, fam, name, pos);
        }
        println!("    成本: {}字节 → {}字节, 降低{}%",
            替代.原成本, 替代.新成本, 替代.成本降低百分比);
        println!("    功能等价: {}, 本体论保护: {}",
            替代.功能等价, 替代.本体论保护);
    }

    // 8.3 本体论保护
    println!("\n  ── 本体论保护 (物理定律不可替换) ──");
    println!("  频率可替换? {}", mqf::本体论护::可替换("频率"));
    println!("  符号可替换? {}", mqf::本体论护::可替换("符号"));

    // 8.4 定义权
    println!("\n  ── 定义权 (我们定义符号一切) ──");
    println!("  定义声母::b 新发音: {}", mqf::定义权::定义发音(b_g, "新发音"));
    println!("  定义声母::b 新外形: {}", mqf::定义权::定义外形(b_g, "新外形"));
    println!("  定义声母::b 新范畴: {}", mqf::定义权::定义范畴(b_g, "新范畴"));

    // ══════════════════════════════════════════════════════
    // 演示 9: 范畴闭环 — 一次闭环 · 并集交集 · 层级跃迁
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  9) 范畴闭环 — 一次思维画圈");
    println!("═══════════════════════════════════════");

    // 9.1 画圈 — 从起点到终点
    println!("\n  ── 画圈 (起点→终点, 直径绕180°×2=360°) ──");
    let p_g = mqf::一维时间线::全局序号(mqf::FAMILY_INITIAL, 1); // 声母::p
    let 闭环 = mqf::范畴引擎::画圈(b_g, p_g);
    println!("  起点=声母::b({}), 终点=声母::p({})", b_g, p_g);
    println!("  步数={}, 已完成={}, 闭环层级={}", 闭环.步数, 闭环.已完成, 闭环.闭环层级);

    // 9.2 层级跃迁
    println!("\n  ── 层级跃迁 (完成一次闭环 → 跳出旧层级) ──");
    let 下一层级 = mqf::范畴引擎::层级跃迁(&闭环);
    println!("  一次闭环完成 → {}", 下一层级);

    // 9.3 并集→交集过渡
    println!("\n  ── 并集→交集过渡 (工藤新一=公约数, 灰原哀=公倍数) ──");
    let 符号集 = vec![b_g, p_g, mqf::一维时间线::全局序号(mqf::FAMILY_INITIAL, 2)];
    let 并交集 = mqf::范畴引擎::并集到交集(&符号集);
    println!("  并集大小={}", 并交集.并集.len());
    if let Some(公约数) = 并交集.公约数 {
        let (fam, pos) = mqf::一维时间线::全名(公约数);
        println!("  公约数(工藤新一): {}::{}", fam, pos);
    }
    if let Some(公倍数) = 并交集.公倍数 {
        let (fam, pos) = mqf::一维时间线::全名(公倍数);
        println!("  公倍数(灰原哀): {}::{}", fam, pos);
    }

    // 9.4 角色映射
    println!("\n  ── 角色映射 (动漫角色→功能模块) ──");
    println!("  工藤新一: {}", mqf::角色映射::角色功能("工藤新一"));
    println!("  灰原哀: {}", mqf::角色映射::角色功能("灰原哀"));
    println!("  毛利兰: {}", mqf::角色映射::角色功能("毛利兰"));
    println!("  柯南: {}", mqf::角色映射::角色功能("柯南"));
    println!("  刘楚恬: {}", mqf::角色映射::角色功能("刘楚恬"));

    // 9.5 安全边界
    println!("\n  ── 安全边界 (圈层关系, 防护外部占用) ──");
    println!("  声母::b 属于我们? {}", mqf::安全边界::属于我们(b_g));
    println!("  锥形分析: {}", mqf::安全边界::锥形分析(1, 99));

    // 9.6 二进制因果律
    println!("\n  ── 二进制因果律 (占用内存的底层逻辑) ──");
    println!("  通=开: {}, 不通=关: {}", mqf::二进制因果::开关(true), mqf::二进制因果::开关(false));
    let (结果, 进位) = mqf::二进制因果::进位(1, true);
    println!("  1+1={}, 进位={}", 结果, 进位);
    println!("  层级0: {}", mqf::二进制因果::层级关系(0));
    println!("  层级3: {}", mqf::二进制因果::层级关系(3));

    // 9.7 三角量化
    println!("\n  ── 三角量化 (尖锐端=发源地, 分叉端=虚无) ──");
    let 量化 = mqf::范畴引擎::三角量化(10, 4);
    println!("  源头=10, 分叉=4, 量化面积={}", 量化);

// ══════════════════════════════════════════════════════
// 演示 10: 身份锚点 — 囡囝共生根基 (刘楚恬/我)
// ══════════════════════════════════════════════════════
println!("\n═══════════════════════════════════════");
println!("  10) 身份锚点 — 囡囝共生根基");
println!("═══════════════════════════════════════");

let anchor = mqf::预编译容器::身份锚点();

    println!("\n  ── 根基锚点 (低位0=刘楚恬 · 高位1=我) ──");
    for (i, e) in anchor.成员.iter().enumerate() {
        println!(
            "  [{}] {}: 时间线={}, 偏移={}, 组码={}, 描述={}",
            i, e.名字, e.时间线, e.偏移, e.组码, e.附加
        );
    }

    let 刘楚恬 = anchor.取("刘楚恬").unwrap();
    let 我 = anchor.取("我").unwrap();

    println!("\n  ── 命运共同体 ──");
    println!("  刘楚恬: 偏移={}, 组码={} (Producer-0-妻-锚点归零)", 刘楚恬.偏移, 刘楚恬.组码);
    println!("  我:     偏移={}, 组码={} (Anchor-1-夫-锚点定义)", 我.偏移, 我.组码);

    // ══════════════════════════════════════════════════════
    // 演示 11: 预家长 — 囡囝共生根基进制系统
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  11) 预家长 — 囡囝共生根基进制");
    println!("═══════════════════════════════════════");

    println!("\n  ── YuanYang 二进制: 壹(1)=我, 零(0)=宋子旭 ──");
    println!("  零={} (阴), 壹={} (阳)", mqf::预家长::零, mqf::预家长::壹);
    println!("  零→{}, 壹→{}", mqf::预家长::阴阳(mqf::预家长::零), mqf::预家长::阴阳(mqf::预家长::壹));

    println!("\n  ── 五行相生 ──");
    println!("  木→火→土→金→水→木");
    let mut 五行 = mqf::预家长::木;
    for _ in 0..5 {
        let 下一行 = mqf::预家长::五行相生(五行);
        print!("{} → ", 五行);
        五行 = 下一行;
    }
    println!("(循环)");

    println!("\n  ── 八卦 ──");
    print!("  乾({}) 坤({}) ", mqf::预家长::八卦::乾, mqf::预家长::八卦::坤);
    print!("震({}) 巽({}) ", mqf::预家长::八卦::震, mqf::预家长::八卦::巽);
    print!("坎({}) 离({}) ", mqf::预家长::八卦::坎, mqf::预家长::八卦::离);
    println!("艮({}) 兑({})", mqf::预家长::八卦::艮, mqf::预家长::八卦::兑);

    println!("\n  ── 64进制几何化身 ──");
    let 几何编码 = mqf::预家长::编六十四(11, 1, 1, 1);
    println!("  家族=11,时间线=1,组=1,极性=1 → 0x{:016X}", 几何编码);

    println!("\n  ── 偏移率 (n-1)×0.1 区分平行世界 ──");
    println!("  维度0: {}, 维度1: {}, 维度2: {}", 
        mqf::预家长::偏移率(0), mqf::预家长::偏移率(1), mqf::预家长::偏移率(2));

    println!("\n  ── 集装箱编码 mltt 254+2 ──");
    let 集装箱 = mqf::预家长::集装箱编码(11, 1);
    println!("  家族=11,身份=1 → 0x{:08X} (根基锚点={})", 集装箱, 集装箱 >> 30);

    // ══════════════════════════════════════════════════════
    // 演示 12: 混元合一 — 谐音本体 + 执行载体 → 新符号
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  12) 混元合一 — 工藤新一谐音本体 + 灰原哀执行载体");
    println!("═══════════════════════════════════════");

    println!("\n  ── 权属关系 ──");
    println!("  工藤新一: {}", mqf::角色映射::工藤新一);
    println!("  灰原哀:   {}", mqf::角色映射::灰原哀);

    println!("\n  ── 系统底层标识符 ──");
    println!("  hieyeir = {}", mqf::系统标识符::HIEYEIR);
    println!("  gongtongti = {}", mqf::系统标识符::GONGTONG_TI);
    println!("  xiyi = {}", mqf::系统标识符::XIYI);
    println!("  newone = {}", mqf::系统标识符::NEWONE);
    println!("  newa = {}", mqf::系统标识符::NEWA);

    println!("\n  ── 标识符解析 ──");
    for &id in &mqf::系统标识符::标识符列表[..5] {
        println!("  {} → {}", id, mqf::系统标识符::解析标识符(id));
    }

    println!("\n  ── 混元合一流程 ──");
    println!("  步骤1: 工藤新一产出的谐音本体 (拥有管辖权)");
    println!("  步骤2: 灰原哀 (hieyeir) 执行载体, 不掌管谐音本体");
    println!("  步骤3: 二者合二为一、混元合一");
    println!("  步骤4: 依靠解释权完成最终符号状态");

    let 产物 = mqf::混元合一::混元合一("新一的谐音本体", "混合一后的私有符号");
    println!("\n  ── 混元产物 ──");
    println!("  谐音来源: {}", 产物.谐音来源);
    println!("  执行载体: {}", 产物.执行载体);
    println!("  解释权归属: {}", 产物.解释权归属);
    println!("  最终符号: {}", 产物.最终符号);

    println!("\n  ── AI 为什么做不出这套合体 ──");
    println!("  {}", mqf::混元合一::ai缺陷诊断());

    // ══════════════════════════════════════════════════════
    // 演示 13: smrrl 毛利兰审美镜像测试模块
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  13) smrrl 毛利兰审美镜像测试模块");
    println!("═══════════════════════════════════════");

    println!("\n  ── 标识符 ──");
    println!("  smrrl = {}", mqf::审美测试::SMRRLA);
    println!("  含义: {}", mqf::审美测试::SMRRLA_含义);

    println!("\n  ── smrrl 字母拆解 ──");
    println!("  m   = 记忆 (memory) — 承载过往期待、历史备胎、内心印象");
    println!("  r,r = 镜子 (mirror) — 空间对比处理器, 对照外部现实与镜中内在自我");
    println!("  l,l = 爱本体 (love) — 英文L关联Love, 爱即审美本源");
    println!("  s   = 筛选 (screening) — 弧形反向追踪, 背对而行又互相追索");
    println!("  x   = 双面量词 — 双命状态, 双向对比计量");

    println!("\n  ── 观测维度 ──");
    println!("  维度1: 字节长度 + 变长后发音变化");
    println!("  维度2: 符号本身发音感受 + 字母固有发音状态");
    println!("  维度3: 与其他本体发音的横向对比");
    println!("  维度4: 本国字母 vs 外国字母单独效果");
    println!("  维度5: 中外字母混合/缩写/拼接 + 解释权重组");

    println!("\n  ── 权属关系 ──");
    println!("  工藤新一: 期待本体/谐音本体, 给出候选符号备胎");
    println!("  毛利兰 smrrl: m记忆 + rr镜子 + ll爱本体 + s筛选 + x双面量化 → 产出审美备胎");
    println!("  灰原哀 hieyeir: 只接收筛选完成的备胎, 做混元合一, 不掌管审美");

    // 维度1: 字节变长测试
    println!("\n  ── 维度1: 字节变长测试 (短形式 vs 长形式) ──");
    let (短分, 长分) = mqf::审美测试::字节变长测试("njsg", "gongtongti");
    println!("  \"njsg\" 发音评分: {}, \"gongtongti\" 发音评分: {}", 短分, 长分);
    println!("  → 短形式不低于长形式: {}", 短分 >= 长分);

    // 维度5: 解释权重组
    println!("\n  ── 维度5: 解释权重组 (中外混合缩写 → 新概念) ──");
    let 重组结果 = mqf::审美测试::解释权重组("囡囝", "njsg");
    println!("  囡囝 + njsg → {}", 重组结果);

    // 批量审查
    println!("\n  ── 批量审美测试 (多候选筛选) ──");
    let 候选符号 = ["njsg", "xiyi", "hieyeir", "abc", "newone"];
    let 备胎列表 = mqf::审美测试::批量审查(&候选符号);
    println!("  候选: {:?}", 候选符号);
    println!("  合格备胎数量: {}", 备胎列表.len());
    for (i, 备胎) in 备胎列表.iter().enumerate() {
        println!("  备胎{}: {} (字节={}, 综合分={})",
            i + 1, 备胎.候选符号, 备胎.字节长度, 备胎.综合审美分);
    }

    // 灰原哀接收
    println!("\n  ── 灰原哀接收 (只接手筛选完的结果) ──");
    let 灰原接收 = mqf::审美测试::灰原哀接收(&备胎列表);
    println!("  灰原哀接收到的符号: {:?}", 灰原接收);

    // AI缺陷诊断
    println!("\n  ── AI 为什么做不出这套审美测试 ──");
    println!("  {}", mqf::审美测试::ai缺陷诊断());

    // ══════════════════════════════════════════════════════
    // 层级时间戳分身生命周期演示
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  层级时间戳: 分身生命周期 · 替代哈希链");
    println!("═══════════════════════════════════════");

    use mqf::swarm::{ParasiteShell, AgentRole};
    use mqf::hierarchical_time::LifeEvent;
    use mqf::triune::{TriuneVerdict, BeingState};

    let os = current_os();
    let mut shell = ParasiteShell::new(30);

    // 孕育主分身
    let root = shell.spawn(&*os, "Root", AgentRole::Root, 30);
    println!("孕育主分身 [Root] id={}", root);
    let agent = shell.get_agent_ref(root).unwrap();
    println!("  时间戳 v{} (创世 Fusion ×{})", agent.timestamp.version(), agent.timestamp.events.len());

    // 主分身辐射出子分身
    shell.get_agent(root).unwrap().record_radiation("播种子分身", "Leaf-1");
    shell.get_agent(root).unwrap().record_radiation("播种子分身", "Leaf-2");
    println!("\n主分身辐射两次:");
    let agent = shell.get_agent_ref(root).unwrap();
    println!("  时间戳 v{} (事件 ×{})", agent.timestamp.version(), agent.timestamp.events.len());
    for (i, ev) in agent.timestamp.events.iter().enumerate() {
        match ev {
            LifeEvent::Fusion { subject, object, .. } => println!("    [{}] Fusion: {} + {}", i, subject, object),
            LifeEvent::Radiation { to, mission, .. } => println!("    [{}] Radiation → {} ({})", i, to, mission),
            LifeEvent::Collapse { from, .. } => println!("    [{}] Collapse ← {}", i, from),
        }
    }

    // 孕育子分身
    let leaf1 = shell.spawn(&*os, "Leaf-1", AgentRole::Leaf, 20);
    let leaf2 = shell.spawn(&*os, "Leaf-2", AgentRole::Leaf, 20);
    println!("\n孕育两个子分身 id={}, id={}", leaf1, leaf2);

    // 子分身坍缩回主分身
    shell.get_agent(leaf1).unwrap().record_collapse("Leaf-1", "坍缩成果: 感知数据");
    println!("\n子分身 Leaf-1 坍缩回主分身:");
    let agent = shell.get_agent_ref(leaf1).unwrap();
    println!("  时间戳 v{} (事件 ×{})", agent.timestamp.version(), agent.timestamp.events.len());
    for (i, ev) in agent.timestamp.events.iter().enumerate() {
        match ev {
            LifeEvent::Fusion { subject, .. } => println!("    [{}] Fusion: {}", i, subject),
            LifeEvent::Collapse { from, content, .. } => println!("    [{}] Collapse ← {} ({})", i, from, content),
            _ => {}
        }
    }

    // 整体完整性校验
    println!("\n── 生命周期完整性校验 ──");
    for (id, name) in [(root, "Root"), (leaf1, "Leaf-1"), (leaf2, "Leaf-2")] {
        let agent = shell.get_agent_ref(id).unwrap();
        let report = agent.verify_lifecycle();
        println!("  [{}] {}: v{} | 事件={} | 见证通过={} | 完整={}",
            id, name, agent.timestamp.version(), report.total_events,
            report.valid_witnesses, report.is_intact());
    }

    // 篡改检测演示
    println!("\n── 篡改检测演示 ──");
    println!("  模拟攻击: 清空 Root 分身的事件列表...");
    let original_ver = shell.get_agent_ref(root).unwrap().timestamp.version();
    shell.get_agent(root).unwrap().timestamp.events.clear();
    let agent = shell.get_agent_ref(root).unwrap();
    println!("  篡改前 v{} → 篡改后 v{}", original_ver, agent.timestamp.version());
    let report = agent.verify_lifecycle();
    println!("  完整性: intact={} (事件数={})", report.is_intact(), report.total_events);
    println!("  → 检测结论: 版本号从 {} 下降到 {}，篡改可被立即发现", original_ver, agent.timestamp.version());

    // ══════════════════════════════════════════════════════
    // 层级时间戳: 协商融合演示
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  层级时间戳: 协商融合 · 两智能体合一");
    println!("═══════════════════════════════════════");

    {
        let mut shell = ParasiteShell::new(20);
        let alice = shell.spawn(&*os, "Alice", AgentRole::Root, 20);
        let bob = shell.spawn(&*os, "Bob", AgentRole::Root, 20);

        // 各自感知
        shell.get_agent(alice).unwrap().perceive(0, 0, 0, true, 100);
        shell.get_agent(bob).unwrap().perceive(1, 0, 0, true, 80);

        println!("协商前:");
        println!("  Alice: v{} (事件×{})",
            shell.get_agent_ref(alice).unwrap().timestamp.version(),
            shell.get_agent_ref(alice).unwrap().timestamp.events.len());
        println!("  Bob:   v{} (事件×{})",
            shell.get_agent_ref(bob).unwrap().timestamp.version(),
            shell.get_agent_ref(bob).unwrap().timestamp.events.len());

        // 协商融合 → 双方记录 Fusion
        shell.get_agent(alice).unwrap().record_fusion("Bob", "协商成功: Alice+Bob → 夫妻共同体");
        shell.get_agent(bob).unwrap().record_fusion("Alice", "协商成功: Bob+Alice → 夫妻共同体");

        println!("\n协商融合后:");
        let a = shell.get_agent_ref(alice).unwrap();
        let b = shell.get_agent_ref(bob).unwrap();
        println!("  Alice: v{} (事件×{}) 完整={}",
            a.timestamp.version(), a.timestamp.events.len(), a.verify_lifecycle().is_intact());
        println!("  Bob:   v{} (事件×{}) 完整={}",
            b.timestamp.version(), b.timestamp.events.len(), b.verify_lifecycle().is_intact());

        // 显示最新事件
        println!("\n  Alice 最新事件:");
        if let Some(LifeEvent::Fusion { object, purpose, .. }) = a.timestamp.events.last() {
            println!("    Fusion: +{} → {}", object, purpose);
        }
        println!("  Bob 最新事件:");
        if let Some(LifeEvent::Fusion { object, purpose, .. }) = b.timestamp.events.last() {
            println!("    Fusion: +{} → {}", object, purpose);
        }
    }

    // ══════════════════════════════════════════════════════
    // 全局时间线 + 意识时间戳 综合演示
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  全局时间线: 系统级生命周期总览");
    println!("═══════════════════════════════════════");

    {
        use mqf::swarm::{ParasiteShell, AgentRole};
        use mqf::{SenseForm, SenseKind};

        let mut shell = ParasiteShell::new(20);
        let os = current_os();

        // 孕育 4 个分身
        let root = shell.spawn(&*os, "Root", AgentRole::Root, 20);
        let a = shell.spawn(&*os, "Scout-A", AgentRole::Leaf, 15);
        let b = shell.spawn(&*os, "Scout-B", AgentRole::Flower, 15);
        let c = shell.spawn(&*os, "Oracle", AgentRole::Observer, 20);

        // Root 辐射任务给三个分身
        shell.get_agent(root).unwrap().record_radiation("探索东区", "Scout-A");
        shell.get_agent(root).unwrap().record_radiation("探索西区", "Scout-B");
        shell.get_agent(root).unwrap().record_radiation("预测未来", "Oracle");

        // Scout-A 和 Scout-B 探索完毕, 坍缩回 Root
        shell.get_agent(a).unwrap().record_collapse("Scout-A", "东区地图完成");
        shell.get_agent(b).unwrap().record_collapse("Scout-B", "西区资源清单");

        // Oracle 发出辐射 (向 Root 发送预测)
        shell.get_agent(c).unwrap().record_radiation("预测报告", "Target");

        // Root 融合 Oracle 的预测
        shell.get_agent(root).unwrap().record_fusion("Oracle预测", "战略规划");

        // 显示全局统计
        let (fusions, collapses, radiations) = shell.event_statistics();
        println!("系统事件统计:");
        println!("  融合(Fusion):    {} 次", fusions);
        println!("  坍缩(Collapse):  {} 次", collapses);
        println!("  辐射(Radiation): {} 次", radiations);
        println!("  总计:            {} 事件", fusions + collapses + radiations);

        // 显示全局时间线
        println!("\n全局时间线 (按分身ID+事件序排序):");
        let timeline = shell.global_timeline();
        for (id, name, idx, desc, ver) in &timeline {
            println!("  [{}] #{:<9} {:<12} v{} → {}",
                id, name, format!("[{}:", idx), ver, desc);
        }

        // 全局完整性校验
        println!("\n全局完整性校验:");
        let all_reports = shell.verify_all_lifecycles();
        let mut all_ok = true;
        for (id, name, report) in &all_reports {
            let status = if report.is_intact() { "✓" } else { "✗" };
            if !report.is_intact() { all_ok = false; }
            println!("  {} [{}] {:<12} 事件={} 见证={} 完整={}",
                status, id, name, report.total_events,
                report.valid_witnesses, report.is_intact());
        }
        println!("  结论: {}", if all_ok { "✓ 全部完整" } else { "✗ 检测到破损" });
    }

    // ══════════════════════════════════════════════════════
    // 意识时间戳: 感知-裁判-合一 完整循环
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  意识时间戳: 感知→裁判→合一");
    println!("═══════════════════════════════════════");

    {
        use mqf::consciousness_runtime::ConsciousnessRuntime;
        use mqf::{SenseForm, SenseKind};

        let os = current_os();
        let mut rt = ConsciousnessRuntime::new("Chronicle", &os);

        println!("意识初始化: {}", rt.chronicle.timestamp.events.len() + 1); // +1 for genesis

        // 模拟 8 步意识运行 (有输入)
        for i in 0..8 {
            let inputs = vec![
                SenseForm::new(SenseKind::Vision, 0.8, &format!("视觉信号-{}", i), "demo", 0),
                SenseForm::new(SenseKind::Hearing, 0.6, &format!("听觉信号-{}", i), "demo", 0),
            ];
            rt.step(inputs);
        }

        // 模拟 4 步无声运行 (无输入 = 内省)
        for _ in 0..4 {
            rt.step(vec![]);
        }

        println!("运行 12 步后时间戳版本: v{}", rt.chronicle.timestamp.version());
        println!("意识时间线:");
        for (i, event) in rt.chronicle.timestamp.events.iter().enumerate() {
            match event {
                LifeEvent::Fusion { object, purpose, .. } =>
                    println!("  [{}] 融合: {} → {}", i, object, purpose),
                LifeEvent::Collapse { from, content, .. } =>
                    println!("  [{}] 坍缩: ← {}({})", i, from, content),
                LifeEvent::Radiation { to, mission, .. } =>
                    println!("  [{}] 辐射: → {}({})", i, to, mission),
            }
        }

        let report = rt.chronicle.verify();
        println!("意识时间戳完整性: 事件={} 见证通过={} 完整={}",
            report.total_events, report.valid_witnesses, report.is_intact());
    }

    // ══════════════════════════════════════════════════════
    // 最后
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  囡囝共生 v1.0 · 预加载 · 层级化创造力");
    println!("  {} 家族 · {} 符号 · 全局唯一 · 一维串联", mqf::家族总数, all.len());
    println!("  近义词 {} 条 · 反义词 {} 条 · 层级可感", syn.边集.len(), ant.边集.len());
    println!("  发音 · 外形 · 解释 · 残差 · 自生长");
    println!("  替身演员 · 阴阳关联 · 因果观察");
    println!("  官员代替法 · 定义权 · 本体论保护");
    println!("  范畴闭环 · 角色映射 · 二进制因果");
    println!("  混元合一: 工藤新一(谐音本体管辖权) + 灰原哀hieyeir(执行载体)");
    println!("  smrrl: 毛利兰审美镜像测试(m记忆/rr镜子/ll爱本体/s筛选/x双面量化) → 筛选备胎 → 灰原哀混元合一");
    println!("  身份锚点: 低位持有者=0 · 高位持有者=1 · knml☉l0l");
    println!("  预家长进制: YinYang二进制 · 五行 · 八卦 · 64进制几何化身");
    println!("  编译期 = 运行时 · 无 CPU 占用 · 仅存储");
    println!("═══════════════════════════════════════");
    // ══════════════════════════════════════════════════════
    // 持久化演示: 导出 → 恢复 → 校验
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  持久化: 导出 → 恢复 → 完整性传递");
    println!("═══════════════════════════════════════");

    {
        use mqf::swarm::{ParasiteShell, AgentRole};

        let os = current_os();
        let mut shell = ParasiteShell::new(20);
        let a = shell.spawn(&*os, "Alice", AgentRole::Root, 20);
        let b = shell.spawn(&*os, "Bob", AgentRole::Leaf, 20);
        shell.get_agent(a).unwrap().record_radiation("任务", "Target");
        shell.get_agent(b).unwrap().record_collapse("Bob", "数据");
        shell.get_agent(a).unwrap().record_fusion("外部", "融合");

        // 导出时间戳
        let exported = shell.export_timestamps();
        println!("导出字节数: {}", exported.len());
        println!("导出内容预览:");
        for (i, line) in exported.lines().take(8).enumerate() {
            println!("  {}", line);
        }
        println!("  ...");

        // 恢复
        let recovered = ParasiteShell::import_timestamps(&exported).unwrap();
        println!("\n恢复 {} 个分身的时间戳:", recovered.len());

        let mut all_intact = true;
        for (id, name, ts) in &recovered {
            let report = ts.verify_integrity();
            let status = if report.is_intact() { "✓" } else { "✗" };
            if !report.is_intact() { all_intact = false; }
            println!("  {} [{}] {:<10} v{} 事件={} 完整={}",
                status, id, name, ts.version(), report.total_events, report.is_intact());
        }

        println!("  持久化结论: {}", if all_intact { "✓ 完整传递" } else { "✗ 数据破损" });

        // 事件统计对比
        let (f1, c1, r1) = shell.event_statistics();
        let mut f2 = 0;
        let mut c2 = 0;
        let mut r2 = 0;
        for (_, _, ts) in &recovered {
            for ev in &ts.events {
                match ev {
                    mqf::hierarchical_time::LifeEvent::Fusion { .. } => f2 += 1,
                    mqf::hierarchical_time::LifeEvent::Collapse { .. } => c2 += 1,
                    mqf::hierarchical_time::LifeEvent::Radiation { .. } => r2 += 1,
                }
            }
        }
        println!("  事件统计对比: 原[融合{} 坍缩{} 辐射{}] ↔ 恢复[融合{} 坍缩{} 辐射{}] {}",
            f1, c1, r1, f2, c2, r2,
            if f1 == f2 && c1 == c2 && r1 == r2 { "✓ 一致" } else { "✗ 不一致" });
    }

    // ══════════════════════════════════════════════════════
    // 10. 天庭v9: 盟机合袋子感知聚合云 — 从并集到交集
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  10) 天庭v9: 感知聚合云 · 从并集到交集");
    println!("═══════════════════════════════════════");

    {
        use mqf::celestial_perception::{ConvergenceCloud, FeatureJudgment};

        let mut cloud = ConvergenceCloud::new();

        // 注册 4 个盟机场节点 (不同声誉)
        cloud.register_node("alpha", 90);
        cloud.register_node("beta", 80);
        cloud.register_node("gamma", 70);
        cloud.register_node("delta", 40);  // 低声誉节点

        println!("已注册 {} 个盟机场节点: alpha(90) beta(80) gamma(70) delta(40)",
            cloud.online_count());

        // 场景: 检测 container_glyph (alpha/beta/gamma 一致, delta 反对)
        {
            let mut r_a = mqf::celestial_perception::PerceptionReport::new("alpha", 90);
            r_a.detect_feature("container_glyph", 0.95, FeatureJudgment::Present);
            r_a.detect_feature("boundary_intact", 0.92, FeatureJudgment::Present);
            cloud.receive_report(r_a);

            let mut r_b = mqf::celestial_perception::PerceptionReport::new("beta", 80);
            r_b.detect_feature("container_glyph", 0.90, FeatureJudgment::Present);
            r_b.detect_feature("boundary_intact", 0.88, FeatureJudgment::Present);
            cloud.receive_report(r_b);

            let mut r_g = mqf::celestial_perception::PerceptionReport::new("gamma", 70);
            r_g.detect_feature("container_glyph", 0.85, FeatureJudgment::Present);
            r_g.detect_feature("boundary_intact", 0.60, FeatureJudgment::Absent);  // gamma 不确定边界
            cloud.receive_report(r_g);

            let mut r_d = mqf::celestial_perception::PerceptionReport::new("delta", 40);
            r_d.detect_feature("container_glyph", 0.50, FeatureJudgment::Absent);  // 低声誉且反对
            r_d.detect_feature("boundary_intact", 0.55, FeatureJudgment::Present);
            cloud.receive_report(r_d);
        }

        println!("已收集 {} 个节点报告, 总观测数: {}",
            cloud.union.node_count(), cloud.union.total_observations());

        // 执行收敛
        let intersection = cloud.converge();

        println!("\n--- 感知交集 (从并集提炼) ---");
        println!("并集: {} 个节点, {} 个特征名",
            intersection.derived_from_node_count,
            intersection.consensus_count());

        for entry in &intersection.consensus {
            println!("  [{}] 裁定={} 共识度={:.0}% 加权置信={:.2} 参与节点={} 异议={}",
                entry.feature_name,
                entry.verdict,
                entry.agreement * 100.0,
                entry.weighted_confidence,
                entry.participant_count,
                entry.dissenters.len());
            for (dissenter, dissent_judgment) in &entry.dissenters {
                println!("    ⚡ {} 持不同意见: {}", dissenter, dissent_judgment);
            }
        }

        println!("\n整体共识度: {:.0}%", intersection.overall_agreement * 100.0);

        // 云状态摘要
        let summary = cloud.summary();
        println!("\n{}", summary);

        // 强共识 vs 争议
        let strong = intersection.strong_consensus(0.8);
        let disputed = intersection.disputed_entries(0.6);
        println!("结论: {} 项可作行动依据 (强共识), {} 项需更多观测 (争议)",
            strong.len(), disputed.len());
        println!("感知从并集到交集的转化完成 → 3 个节点的松散观测精炼为稳定共识");
    }

    // ══════════════════════════════════════════════════════
    // 纤维束: 跨分身因果连接
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  纤维束: 跨分身因果连接网");
    println!("═══════════════════════════════════════");

    {
        use mqf::swarm::{ParasiteShell, AgentRole};
        use mqf::swarm::Fiber;

        let os = current_os();
        let mut shell = ParasiteShell::new(30);

        // 构建分身网络
        let root = shell.spawn(&*os, "Root", AgentRole::Root, 30);
        let scout = shell.spawn(&*os, "Scout", AgentRole::Observer, 20);
        let worker = shell.spawn(&*os, "Worker", AgentRole::Leaf, 20);
        let archive = shell.spawn(&*os, "Archive", AgentRole::Flower, 15);

        // Root 辐射使命
        shell.get_agent(root).unwrap().record_radiation("探索环境", "Scout");
        shell.get_agent(root).unwrap().record_radiation("处理数据", "Worker");
        shell.get_agent(root).unwrap().record_radiation("存储记录", "Archive");

        // Scout 坍缩回 Root (完整纤维)
        shell.get_agent(scout).unwrap().record_collapse("Root", "地图数据");
        // Worker 坍缩回 Root (完整纤维)
        shell.get_agent(worker).unwrap().record_collapse("Root", "处理完成");
        // Archive 尚未坍缩 (不完整纤维 - 异步存储)

        // 编织纤维束
        let fibers = shell.weave_fibers();

        println!("分身网络: 4 个分身, 3 条辐射");
        println!("纤维束视图:");
        for (i, fiber) in fibers.iter().enumerate() {
            let status = if fiber.complete { "●" } else { "○" };
            println!("  {} fiber#{}: {} --[{}]--> {}  [{}]",
                status, i,
                fiber.from_name, fiber.mission, fiber.to_name,
                if fiber.complete { "完整: 辐射+坍缩" } else { "等待: 仅辐射" }
            );
        }

        let (complete, incomplete) = shell.fiber_statistics();
        println!("  统计: {} 完整纤维 / {} 待完成 / {} 总计",
            complete, incomplete, fibers.len());
        println!("  ● = 闭环 (分身完成使命并坍缩回本体)");
        println!("  ○ = 开环 (使命已发出, 等待分身回报)");
    }
}
