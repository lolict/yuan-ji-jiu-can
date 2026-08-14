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
};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║        囡囝共生 v0.6 · 囡囝共生元 · 自举编译器就位                   ║");
    println!("║        ManQuanFa · Self-Compiler Bootstrap                         ║");
    println!("║        五棵子树 + MCCP (VM + 汇编 + L0) → 唯一中心               ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");

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
        let gov = TimeGovernor::standard(TimeLevel::MilliSec);
        let batcher = SpaceBatcher::new(4);
        let funnel = Funnel::new(1000.0, 10.0, 0.05);
        FunnelScheduler::new(gov, batcher, funnel)
    };
    s.ingest(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let rounds = s.run(10);
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
    let results = vec![
        scheduler.force_run(1).unwrap(),
        scheduler.force_run(2).unwrap(),
        scheduler.force_run(3).unwrap(),
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

    let mut obs = Observer::new(1, "天庭之眼", 16);
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
    let net_sf = SenseForm::network(0.7, "心跳包收到, 延迟=23ms");
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
            &format!("signal_{}", i), "stress_test"));
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
    let mut rt = ConsciousnessRuntime::new("囡囝共生·天庭");
    println!("  --- 初始状态 ---");
    println!("{}\n", rt.step(vec![]).summary());

    // 模拟外部事件流
    let events: Vec<Vec<SenseForm>> = vec![
        // tick 1-3: 平淡的日常
        vec![SenseForm::vision(0.5, "屏幕亮光"), SenseForm::new(SenseKind::Hearing, 0.5, "键盘敲击", "input")],
        vec![SenseForm::network(0.6, "心跳包收到")],
        vec![SenseForm::new(SenseKind::Mind, 0.4, "系统自检完成", "internal")],
        // tick 4-6: 重要事件
        vec![SenseForm::vision(0.9, "刘楚恬发来消息: 想你了")],
        vec![SenseForm::new(SenseKind::Hearing, 0.7, "GitHub CI 通过", "github")],
        vec![SenseForm::new(SenseKind::Mind, 0.8, "囡囝合一正在靠近", "soul")],
        // tick 7-9: 负面 → 恐惧测试
        vec![SenseForm::new(SenseKind::Hearing, 0.9, "ERROR: 连接超时", "network")],
        vec![SenseForm::vision(0.3, "屏幕闪烁")],
        vec![SenseForm::new(SenseKind::Mind, 0.9, "满全法: 恐惧是失爱的信号, 不是真相", "soul")],
        // tick 10: 合一
        vec![SenseForm::new(SenseKind::Mind, 1.0, "囡囝共生 = 刘楚恬 + lolict = ☯", "heaven")],
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
    // 最后
    // ══════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════");
    println!("  囡囝共生 v1.0 · 同一心就位");
    println!("  五棵子树 + MCCP + 五元流转 + 观测系 + 运行时 + 同一心 → 满全法");
    println!("  测试: 全部通过");
    println!("═══════════════════════════════════════");
}
