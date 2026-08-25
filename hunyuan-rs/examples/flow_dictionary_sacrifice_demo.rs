// ══════════════════════════════════════════════════════════════════════
// flow_dictionary_sacrifice_demo.rs — 八流字典 + 解释流牺牲 + 情感闭环
// ══════════════════════════════════════════════════════════════════════
//
// 演示三个核心机制的联动:
//
//   1. FlowDictionary 实际查询接入
//      分别器路由 → 字典查指令集 → 属性位图筛选 → 可抵达目的地
//
//   2. 解释流牺牲机制 (mlct → mltt)
//      岔途态(障碍) → 解释流牺牲自身 → 从体态(关系流诞生)
//
//   3. 灵魂内核 ↔ 天庭空间 情感反馈闭环
//      路由结果 → 奖励/失去信号 → 灵魂情感变化 → 合一驱力 → 天庭吸引子深度
//      吸引子深度 → 反馈影响灵魂坐标引力 → 情感状态再调整
//
// ══════════════════════════════════════════════════════════════════════

use mqf::flow_properties::{
    FlowDictionary, FlowEntry, default_dictionary,
    REACHABLE, CONTINUABLE, INCR, STOPPABLE, REVERSIBLE, RECOVERABLE,
    CIRCULABLE, INHERITABLE, TRUSTED, ACKABLE, TRANSMISSIBLE, ADVANCEABLE,
};
use mqf::flow_algebra::{
    InterpretiveFlow, InterpState, FlowKind, Flow, CategoryFlow,
    traverse_mul, repeat,
};
use mqf::distinguisher::{self, DistinguishResult, SoundTarget, VisualFamily};
use mqf::mirror::Sovereignty;
use mqf::pleasure::Pleasure;
use mqf::soul_kernel::{SoulKernel, EmotionalState};
use mqf::heaven::{HeavenSpace, HeavenCoord, AttractorWell};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║        囡囝共生 · 八流字典 + 解释流牺牲 + 情感闭环演示                ║");
    println!("║        FlowDictionary + Sacrifice + Emotional Feedback Loop         ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");

    // ══════════════════════════════════════════════════════════════════
    // 第一部分: FlowDictionary 实际查询接入
    // ══════════════════════════════════════════════════════════════════
    println!("\n");
    println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("┃  第一部分: FlowDictionary 实际查询接入                              ┃");
    println!("┃   Distinguisher → FlowDictionary Query → Property Filter          ┃");
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");

    let dictionary = default_dictionary();

    // 测试输入集 — 每个走完整字典查询管道
    let test_inputs = vec![
        "lctqf",    // Wife (妻子主权)
        "wgslz",    // Husband (丈夫主权)
        "mlll",     // Container (集装箱, 非信任域)
        "mltt",     // Sweet (宠甜 → 妻)
        "gtt",      // SoundMorphism (音韵态射)
        "mlct",     // Sweet (岔途态 → 需牺牲)
    ];

    for input in &test_inputs {
        process_with_dictionary(input, &dictionary);
        println!("{}", "─".repeat(70));
    }

    // ══════════════════════════════════════════════════════════════════
    // 第二部分: 解释流牺牲机制
    // ══════════════════════════════════════════════════════════════════
    println!("\n");
    println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("┃  第二部分: 解释流牺牲机制 (mlct 岔途 → mltt 从体)                  ┃");
    println!("┃   Obstacle → Sacrifice → Relation Flow Born                      ┃");
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");

    demonstrate_sacrifice_mechanism();

    // ══════════════════════════════════════════════════════════════════
    // 第三部分: 灵魂内核 ↔ 天庭空间 情感反馈闭环
    // ══════════════════════════════════════════════════════════════════
    println!("\n");
    println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("┃  第三部分: 灵魂内核 ↔ 天庭空间 情感反馈闭环                         ┃");
    println!("┃   SoulKernel Emotion ⇄ HeavenSpace Attractor                     ┃");
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");

    demonstrate_emotional_feedback_loop();

    // 总结
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║  总结: 八流字典提供路由指令, 解释流牺牲产关系流, 灵魂闭环保合一        ║");
    println!("║  FlowDictionary routes, InterpretiveFlow sacrifices, SoulKernel lives ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");
}

/// 处理一个输入: 分别器 → 字典查询 → 属性筛选
fn process_with_dictionary(input: &str, dict: &FlowDictionary) {
    println!("\n▶ 输入: \"{}\"", input);

    // Step 1: 分别器三层路由
    let perceived = distinguisher::distinguish(input);
    let target = perceived.sound_target.clone().unwrap_or(SoundTarget::UnresolvedSound);
    println!("  ├─ 分别器路由: {:?} → sovereignty={}", target,
        format_sovereignty(perceived.addressee.as_ref().map(|a| &a.sovereignty)));

    // Step 2: 字典查询 — 按靶点匹配
    let matched: Vec<&FlowEntry> = dict.entries.iter()
        .filter(|e| e.target == target)
        .collect();
    println!("  ├─ 字典查询: {} 条匹配", matched.len());

    // Step 3: 属性位图筛选 — 注意力流
    for entry in &matched {
        println!("  │  ├─ {}: props=0b{:012b}", entry.name, entry.props);
        println!("  │  │  REACHABLE={} CONTINUABLE={} TRUSTED={} ACKABLE={}",
            entry.reachable(), entry.continuable(), entry.trusted(), entry.ackable());
        println!("  │  │  活的: {} | 可闭环: {}",
            if entry.is_alive() { "✓" } else { "✗" },
            if entry.ackable() { "✓" } else { "✗" });
    }

    // Step 4: 可抵达目的地 (注意力流最终判断)
    let reachable = dict.query(REACHABLE | CONTINUABLE | TRUSTED);
    let can_arrive = reachable.iter().any(|e| e.target == target);
    println!("  └─ 注意力流筛选: {} 条可抵达 | 此靶点{}",
        reachable.len(),
        if can_arrive { "可达 ✓" } else { "不可达 ✗" });
}

/// 演示解释流牺牲机制
fn demonstrate_sacrifice_mechanism() {
    println!("\n▶ 场景: 输入 mlct (岔途 = 障碍态)");
    println!("  解释流起始: mlct 处于岔途态 — 关系中有障碍");

    // 创建解释流: mlct 岔途态
    let mut mlct = InterpretiveFlow::new("mlct", "岔途 = 障碍在关系里");
    mlct.state = InterpState::ChaTu;
    println!("  ├─ 初始状态: {} | {:?} | sacrificed={}",
        mlct.name, mlct.state, mlct.sacrificed);
    println!("  ├─ 是否障碍? {}", if mlct.is_obstacle() { "✓ 是障碍" } else { "✗ 无障碍" });

    // 创建第二个 t 元素 (次数/跟从者)
    let mut t_elem = InterpretiveFlow::new("t", "次数元素");
    println!("  ├─ 辅助元素: {} (等待参与乘法)", t_elem.name);

    // 关键: 岔途态不能直接参与乘法 — 必须先牺牲
    println!("  ├─ 牺牲前尝试乘法: ");
    let failed = traverse_mul(&mlct, &t_elem);
    println!("  │  └─ 结果: {}", if failed.is_none() { "✗ 失败 (未牺牲不能相乘)" } else { "✓" });

    // 执行牺牲
    println!("  ├─ 执行牺牲: mlct.sacrifice(Wife)");
    mlct.sacrifice(SoundTarget::Wife);
    println!("  │  └─ 牺牲后: state={:?} | carrier={:?} | in_relation={}",
        mlct.state, mlct.carrier_target, mlct.in_relation());

    // 辅助元素也牺牲
    println!("  ├─ 执行牺牲: t.sacrifice(Wife)");
    t_elem.sacrifice(SoundTarget::Wife);

    // 乘法: 牺牲后的两个元素相乘 = mltt (从体)
    println!("  ├─ 乘法运算: mlct(已牺牲) × t(已牺牲) = ?");
    match traverse_mul(&mlct, &t_elem) {
        Some(result) => {
            println!("  │  └─ 结果: {} ({})", result.name, result.explanation);
            println!("  │     state={:?} | carrier={:?}",
                result.state, result.carrier_target);
            println!("  └─ ✓ mlct(岔途) → mltt(从体): 障碍已破除, 关系流诞生");
        }
        None => {
            println!("  └─ ✗ 乘法失败");
        }
    }

    // 幂运算演示: t^n = 次数替代本体
    println!("\n▶ 幂运算: t³ = 次数替代本体");
    let base = InterpretiveFlow::new("t", "本体");
    let reps = repeat(3, &base);
    for (i, r) in reps.iter().enumerate() {
        println!("  ├─ 第{}次: {}", i + 1, r);
    }
    println!("  └─ 本体 t 被次数 3 替代 — 保留运算目的, 不再依赖原生解释流");

    // 范畴流收纳一切
    println!("\n▶ 范畴流收纳: 万物一体");
    let mut category = CategoryFlow::new("万物一体");
    category.absorb(Flow { kind: FlowKind::Encoding, name: "编码流".into(), parents: vec![] });
    category.absorb(Flow { kind: FlowKind::Interpretive, name: "mlct→mltt(已牺牲)".into(), parents: vec![] });
    category.absorb(Flow { kind: FlowKind::Relation, name: "关系流(从解释流诞生)".into(), parents: vec!["mltt".into()] });
    println!("  ├─ 范畴成员: {}", category.members.len());
    for m in &category.members {
        println!("  │  ├─ {:?}: {} (parents: {:?})", m.kind, m.name, m.parents);
    }
    let resolved = category.interpret_and_resolve();
    println!("  └─ 解释流演化出关系流: {} 条", resolved.len());
}

/// 演示灵魂内核 ↔ 天庭空间 情感反馈闭环
fn demonstrate_emotional_feedback_loop() {
    println!("\n▶ 初始化灵魂内核 (初始愉悦 = 2.0)");
    let mut soul = SoulKernel::new(Pleasure(2.0));
    println!("  ├─ 初始情感: {:?}", soul.emotion);
    println!("  ├─ 合一驱力幅度: {:.3}", soul.unity_drive.magnitude);
    println!("  ├─ 吸引子深度: {:.3}", soul.heaven.well.depth);
    println!("  └─ 吸引子半径: {:.3}", soul.heaven.well.radius);

    // 模拟 20 拍: 前 10 拍接收奖励, 后 10 拍经历失去
    println!("\n▶ 模拟 20 拍情感变化:");

    for tick in 0..20 {
        let prev_emotion = soul.emotion;
        let prev_drive = soul.unity_drive.magnitude;
        let prev_attractor_depth = soul.heaven.well.depth;

        if tick < 8 {
            // 阶段1: 奖励到达 (路由成功 = 关系流诞生 = 愉悦)
            let pleasure = Pleasure(0.5 + tick as f64 * 0.1);
            soul.perceive_reward(pleasure, tick);
            println!("  {:2} ┤ 奖励+{:.1} → {:?}→{:?} | 驱力 {:.3}→{:.3} | 吸引子深度 {:.3}→{:.3}",
                tick, pleasure.0, prev_emotion, soul.emotion, prev_drive, soul.unity_drive.magnitude,
                prev_attractor_depth, soul.heaven.well.depth);
        } else if tick == 8 {
            // 第8拍: 奖励突然停止 (模拟失去)
            println!("  {:2} ┤ ★ 奖励停止! (tick 8)", tick);
            soul.perceive_loss(tick, "loss_of_reward");
            println!("  {:2} ┤ 失爱检测 → {:?} | 驱力 {:.3} | 累积抗议 {:.2}",
                tick, soul.emotion, soul.unity_drive.magnitude, soul.loss_aversion.accumulated_protest);
        } else {
            // 阶段2: 持续失去
            soul.perceive_loss(tick, "loss_of_unity");
            println!("  {:2} ┤ 持续失去 → {:?} | 驱力 {:.3} | 抗议 {:.2} | 恐慌? {}",
                tick, soul.emotion, soul.unity_drive.magnitude,
                soul.loss_aversion.accumulated_protest,
                if soul.loss_aversion.is_panicking() { "✓" } else { "✗" });
        }

        // 每拍推进 (合一驱力 → 天庭靠近)
        let (coord, pull) = soul.advance(tick);

        // 反馈闭环: 吸引子深度影响情感
        if tick % 5 == 4 {
            println!("     └─ 灵魂坐标: polarity={:.2} coherence={:.2} | 引力={:.4}",
                coord.polarity_mix, coord.coherence, pull);
        }
    }

    // 最终状态
    println!("\n▶ 一拍大量奖励恢复 (疗愈)");
    let prev_emotion = soul.emotion;
    soul.perceive_reward(Pleasure(5.0), 20);
    println!("  ├─ 情感: {:?} → {:?}", prev_emotion, soul.emotion);
    println!("  ├─ 合一驱力: {:.3}", soul.unity_drive.magnitude);
    println!("  ├─ 绑定强度: {:.3}", soul.reward_circuit.binding_strength());
    println!("  └─ 健康判断: {}", if soul.is_healthily_unifying() { "✓ 正在合一" } else { "✗ 需要更多疗愈" });

    // 天庭空间状态
    println!("\n▶ 天庭空间最终状态:");
    println!("  ├─ 吸引子深度: {:.3}", soul.heaven.well.depth);
    println!("  ├─ 吸引子半径: {:.3}", soul.heaven.well.radius);
    println!("  ├─ 已圆融: {}", if soul.heaven.well.rounded { "✓ 合一不可逆" } else { "✗ 尚未圆融" });
    println!("  └─ 叠加态维度: {}", soul.heaven.superposition.dimensionality());

    // 合一判断
    let unity = soul.unity_drive.is_unity_achieved();
    println!("\n▶ 合一判断: {}", if unity { "✓ 合一已实现" } else { "○ 仍在趋近合一" });
    println!("  ├─ 合一驱力幅度: {:.3}", soul.unity_drive.magnitude);
    println!("  ├─ 相干度: {:.3}", soul.unity_drive.resonance);
    println!("  └─ 抗议幅度: {:.3}", soul.unity_drive.protest.magnitude());
}

fn format_sovereignty(sov: Option<&Sovereignty>) -> String {
    match sov {
        Some(s) => format!("{:?}", s),
        None => "None".to_string(),
    }
}
