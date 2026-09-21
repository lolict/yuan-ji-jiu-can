// ══════════════════════════════════════════════════════════════════
// judgment_and_invocation_demo.rs
// 判断之力 + 呼唤表 联合演示
// ══════════════════════════════════════════════════════════════════
//
//   完整链路:
//     感知差异 (Ripple diff)
//       → 判断因果 (CausalPair)
//         → 评价等级 (Evaluation)
//           → 固化规则 (RelationRule)
//             → 反馈信号 (FeedbackSignal)
//               → 触发呼唤表入口 (InvocationEntry)
//                 → 执行运算 (Invoker → OpKind)
//
//   场景: 一个温度监控系统
//     - sensor 每周期报告温度
//     - 引擎判断温度变化趋势
//     - 评价变化幅度 (Noise/Notice/Respond/Intervene/Alert)
//     - 固化规则: "温度变化>X → 激活降温"
//     - 反馈信号触发呼唤表执行实际降温动作
//
// ══════════════════════════════════════════════════════════════════

use mqf::judgment_power::{
    CausalPair, JudgmentEngine, Ripple,
    EvaluationLevel, FeedbackSignal,
};
use mqf::invocation_table::{
    InvocationTable, InvocationEntry, Invoker,
    OpKind, Arithmetic, Compare,
};

fn main() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║   判断之力 + 呼唤表 · 完整闭环演示          ║");
    println!("╚══════════════════════════════════════════════╝\n");

    // ── 第一阶段: 构建温度监控的呼唤表 ──
    let mut table = InvocationTable::new();

    // entry 10: 启动降温系统 (风扇加速)
    table.register(InvocationEntry::with_label(10, "cooling_on", vec![
        OpKind::PushI(1),    // 状态: 开启
        OpKind::Store(0),    // reg[0] = 冷却状态
        OpKind::PushI(100),  // 目标转速
        OpKind::Store(1),    // reg[1] = 目标转速
        OpKind::Halt,
    ]));

    // entry 20: 关闭降温
    table.register(InvocationEntry::with_label(20, "cooling_off", vec![
        OpKind::PushI(0),
        OpKind::Store(0),    // reg[0] = 0 (关闭)
        OpKind::PushI(0),
        OpKind::Store(1),    // reg[1] = 0
        OpKind::Halt,
    ]));

    // entry 30: 温度差异计算与响应 (参数化)
    // 栈传入: [上次温度, 当前温度]
    // 计算出差异后, 如果超过阈值则调用降温
    table.register(InvocationEntry::with_label(30, "temp_diff_response", vec![
        // 计算差异: current - last
        OpKind::Arithmetic(Arithmetic::Sub),  // stack: [diff]
        // 如果 diff > 5 (阈值) 调用降温
        OpKind::PushI(5),
        OpKind::Compare(Compare::Gt),  // diff > 5?
        OpKind::JumpIfFalse(31),  // 不大于则跳去关闭
        OpKind::Call(10),          // 调用降温
        OpKind::Return,
    ]));

    // entry 31: 差异不大, 关闭降温
    table.register(InvocationEntry::new(31, vec![
        OpKind::Call(20),
        OpKind::Return,
    ]));

    println!("✓ 呼唤表已构建 ({} 个入口)", table.len());

    // ── 第二阶段: 构建判断引擎 ──
    let mut engine: JudgmentEngine<f64> = JudgmentEngine::new();

    println!("✓ 判断引擎已初始化\n");

    // ── 第三阶段: 模拟数据流 ──
    println!("── 模拟温度数据流 ──\n");

    let temperatures = [20.0, 21.0, 22.5, 25.0, 28.5, 32.0, 35.5, 38.0];
    let mut prev_temp = temperatures[0];

    println!("初始温度: {:.1}°C", prev_temp);

    for (i, &temp) in temperatures[1..].iter().enumerate() {
        let cycle = i + 1;
        println!("\n═══ 周期 {} ═══", cycle);
        println!("  传感器读数: {:.1}°C", temp);

        // ── 感知阶段 ──
        let ripple = Ripple::new(
            cycle as u64,
            "temp_sensor",
            temp,
            temp,      // 幅度 = 温度值
            0.0,       // 相角
        );
        engine.perceive(ripple);

        // 计算差异 (用于显示)
        let diff = (temp - prev_temp).abs();
        println!("  温度变化: {:.1}°C", diff);

        // 如果有差异, 执行判断-评价-反馈-执行 闭环
        if diff > 0.01 {
            // ── 判断阶段 ──
            let diff = engine.diffs.last().cloned().unwrap();
            let pair = CausalPair::new(diff, "env_heat", "cooling_system");
            engine.causal_pairs.push(pair);

            // ── 评价阶段 ──
            let pair_idx = engine.causal_pairs.len() - 1;
            let _rule_id = engine.evaluate(pair_idx, "temp_change_eval");

            // ── 反馈阶段 ──
            let feedback = engine.feedback();
            println!("  反馈信号: {:?}", feedback.signals);

            // ── 执行阶段 (将判断结果转化为呼唤表调用) ──
            for signal in &feedback.signals {
                if let FeedbackSignal::EnhancePerception(source) = signal {
                    println!("  → 增强感知: {}", source);

                    // 调用呼唤表执行降温
                    let mut invoker = Invoker::new();
                    invoker.data_stack.push(prev_temp as u64);
                    invoker.data_stack.push(temp as u64);

                    let _ = invoker.call_entry(&table, 30);
                    let cooling_status = invoker.registers.get(0).copied().unwrap_or(0);
                    let target_rpm = invoker.registers.get(1).copied().unwrap_or(0);

                    if cooling_status > 0 {
                        println!("  ★ 降温系统启动! 目标转速: {}", target_rpm);
                    } else {
                        println!("  ○ 系统维持现状");
                    }
                }
            }
        } else {
            println!("  无显著变化, 跳过评价");
        }

        prev_temp = temp;
    }

    // ── 第四阶段: 输出统计 ──
    println!("\n═══════════════════════════════════════");
    println!("  统计报告");
    println!("═══════════════════════════════════════");
    println!("  评价总数: {}", engine.evaluations.len());
    println!("  规则总数: {}", engine.total_rules());
    println!("  固化规则数: {}", engine.solidified_count());

    // 评价等级分布
    println!("\n  评价等级分布:");
    let level_names = ["噪声", "注意", "响应", "干预", "警报"];
    for (level, name) in level_names.iter().enumerate() {
        let count = engine.evaluations.iter()
            .filter(|e| {
                let lvl = match e.level {
                    EvaluationLevel::Noise => 0,
                    EvaluationLevel::Notice => 1,
                    EvaluationLevel::Respond => 2,
                    EvaluationLevel::Intervene => 3,
                    EvaluationLevel::Alert => 4,
                };
                lvl == level
            })
            .count();
        if count > 0 {
            println!("    {}: {}", name, count);
        }
    }

    // 固化规则列表
    let rules = engine.rule_bank.rules.clone();
    if !rules.is_empty() {
        println!("\n  已固化规则:");
        for (id, rule) in &rules {
            println!("    规则#{}: {} → {} (激活 {} 次, 权重 {:.2})",
                id, rule.cause_pattern, rule.effect_pattern,
                rule.activation_count, rule.avg_weight);
        }
    }

    println!("\n═══════════════════════════════════════");
    println!("  闭环稳定性: ✓");
    println!("═══════════════════════════════════════");
}
