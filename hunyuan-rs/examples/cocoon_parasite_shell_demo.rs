// ══════════════════════════════════════════════════════════════════════
// cocoon_parasite_shell_demo.rs — 蚕蛹反向寄生壳 · 脱壳穿衣
// ══════════════════════════════════════════════════════════════════════
//
// 演示囡囝共生的"反向寄生"策略:
//
//   1. 蚕蛹初始包 — 内含解析器与寄生壳选项清单
//   2. 脱壳穿衣 — 用户选择寄生壳, 执行脱壳 → 穿衣操作
//   3. 多智能体协商 — Root/Trunk/Leaf/Flower/Observer 五角色协同
//   4. 反向寄生 — 外部系统模仿自身, 而非自身适应外部
//   5. 生命周期 — Genesis → Radiation → Collapse → Fusion
//
// ══════════════════════════════════════════════════════════════════════

use mqf::swarm::{
    CocoonAgent, AgentRole, Relationship, RelType,
};
use mqf::os_layer::{OsLayer, linux::LinuxOs};
use mqf::mirror::{self, Sovereignty, EncodedCell};
use mqf::pleasure::Pleasure;
use mqf::memory::Coord;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║           囡囝共生 · 蚕蛹反向寄生壳演示                              ║");
    println!("║   Cocoon Shell · Multi-Agent协商 · 反向寄生策略                     ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");

    demonstrate_cocoon_shell();
    demonstrate_multi_agent_swarm();
    demonstrate_reverse_parasite();
    demonstrate_lifecycle_integration();

    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║  蚕蛹反向寄生壳总结                                                   ║");
    println!("║  初始包: 5种角色 × 5种主权 × 4种关系类型                              ║");
    println!("║  脱壳穿衣: 选壳 → 脱壳 → 穿衣 → 外部模仿自身                          ║");
    println!("║  协商: Internal Negotiation → Surplus Alignment                      ║");
    println!("║  反向寄生: 不让外部规则绑架, 让外部适应我们的规则                       ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");
}

/// 演示蚕蛹壳创建与脱壳穿衣
fn demonstrate_cocoon_shell() {
    println!("\n");
    println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("┃  第一部分: 蚕蛹初始包 & 脱壳穿衣                                    ┃");
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");

    let os = LinuxOs::new();

    // 五种角色对应五种"壳"
    let roles = vec![
        (AgentRole::Root, "根基壳", "容器层 · 主权 Active"),
        (AgentRole::Trunk, "主干壳", "传输层 · 主权 PassiveSub"),
        (AgentRole::Leaf, "叶片壳", "感知层 · 主权 Passive"),
        (AgentRole::Flower, "花果壳", "表达层 · 主权 ActiveSub"),
        (AgentRole::Observer, "见证壳", "三元裁判 · 主权 Active"),
    ];

    println!("\n  蚕蛹初始包 — 5种寄生壳选项:");
    for (role, name, desc) in &roles {
        let shell = CocoonAgent::new(&os, role.clone() as u64 + 1, name, role.clone(), 64);
        let sov = role.default_sovereignty();
        println!("    ├─ [{:2}] {:8} | {} | sovereignty={:?}, 诞生tick={}",
            shell.id, name, desc, sov, shell.birth_tick);
    }

    // 脱壳穿衣演示
    println!("\n  脱壳穿衣流程 (用户选中\"花果壳\"):");
    let mut flower = CocoonAgent::new(&os, 4, "花果-皂", AgentRole::Flower.clone(), 32);
    println!("    ├─ 选中壳: {} (role={:?})", flower.name, flower.role);

    // 模拟感知外部系统 (脱壳前, 先"看"外面的信息形态)
    flower.perceive(0x10, 100, 0, true, 42);   // 视觉: 容器编码
    flower.perceive(0x20, 200, 0, false, 84);  // 听觉: 界线振幅
    println!("    ├─ 感知外部: 2条输入编码入state, state.len={}", flower.state.len());
    println!("    ├─ 心跳: {}步行走={}", flower.pulse.beat_count(), flower.total_steps());
    println!("    └─ 穿衣: 外部系统戴上\"花果\"面具 → 模仿囡囝表达风格");
}

/// 演示多智能体协商
fn demonstrate_multi_agent_swarm() {
    println!("\n");
    println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("┃  第二部分: 多智能体协商 (Swarm Negotiation)                       ┃");
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");

    let os = LinuxOs::new();

    // 创建5个智能体 (分身)
    let mut root = CocoonAgent::new(&os, 1, "根-容器", AgentRole::Root, 128);
    let mut trunk = CocoonAgent::new(&os, 2, "干-传输", AgentRole::Trunk, 96);
    let mut leaf = CocoonAgent::new(&os, 3, "叶-感知", AgentRole::Leaf, 64);
    let mut flower = CocoonAgent::new(&os, 4, "花-表达", AgentRole::Flower, 64);
    let mut observer = CocoonAgent::new(&os, 5, "观-见证", AgentRole::Observer, 256);

    // 各智能体感知不同数据
    root.perceive(0x01, 10, 0, true, 100);     // 容器层: 高值
    trunk.perceive(0x02, 20, 0, false, 50);     // 传输层: 中值
    leaf.perceive(0x03, 5, 0, true, 30);        // 感知层: 低值
    flower.perceive(0x04, 15, 0, false, 70);    // 表达层: 中高值
    observer.perceive(0x05, 0, 0, true, 999);   // 见证层: 裁判高值

    // 建立关系
    println!("\n  智能体关系网络:");
    let agents = [&root, &trunk, &leaf, &flower, &observer];
    for (i, a) in agents.iter().enumerate() {
        let role_str = format!("{:?}", a.role);
        println!("    [{}] {:10} state={:?} beat={}",
            i, role_str, a.state.len(), a.pulse.beat_count());
    }

    // 内部协商 (Root vs Flower 对话)
    let (surplus, aligned) = root.negotiate_internal(&flower.state);
    println!("\n  协商: Root ↔ Flower");
    println!("    ├─ surplus = {:.1} (Root 总值 - Flower 总值)", surplus);
    println!("    ├─ aligned = {}", if aligned { "✓ 共振对齐" } else { "✗ 存在偏差" });
    println!("    └─ 结论: {}", if aligned {
        "双方合一, 偏差<20%"
    } else {
        "需要更多协商拍"
    });

    // 建立显式关系
    let mut root_mut = root.clone();
    root_mut.relate(flower.id, RelType::Couple, 0.3);
    root_mut.relate(observer.id, RelType::Contain, 0.5);
    println!("\n  关系建立 (Root视角):");
    for (id, rel) in &root_mut.relationships {
        println!("    ├─ 与 #{} → {:?} (强度 {:.2})", id, rel.rel_type, rel.strength);
    }
}

/// 演示反向寄生策略
fn demonstrate_reverse_parasite() {
    println!("\n");
    println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("┃  第三部分: 反向寄生策略 (Reverse Parasite)                        ┃");
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");

    let os = LinuxOs::new();

    // 场景: 面对外部系统, 不是我们去适应, 而是让它模仿我们
    println!("\n  场景: 外部API要求我们按它的JSON格式输出");
    println!("  常规做法: 我们的代码适配外部格式 (正向寄生 = 被绑架)");
    println!("  反向寄生: 外部系统戴上面具, 模仿我们的容器编码");

    // 创建代表"我们"的智能体
    let mut us = CocoonAgent::new(&os, 1, "囡囝-容器", AgentRole::Root, 64);
    let mut them = CocoonAgent::new(&os, 2, "外部-API", AgentRole::Leaf, 64);

    // 我们用容器编码感知世界
    // Can = 0x12 = 蚕 — 蚕蛹/蜕变
    us.perceive(0x12, 0, 0, true, 100);    // 蚕: 蜕变
    us.perceive(0x01, 1, 0, true, 90);     // 妻子: 容器
    us.perceive(0x02, 2, 0, false, 80);    // 丈夫: 界线

    // 外部系统被迫用我们的编码"穿衣"后
    them.perceive(0x12, 0, 0, true, 100);  // 它也学会了"蚕"
    them.perceive(0x01, 1, 0, true, 90);   // 它也学会了"妻子"
    them.perceive(0x02, 2, 0, false, 80);  // 它也学会了"丈夫"

    println!("\n  反向寄生执行:");
    println!("    ├─ 我们的编码: 蚕(0x12) + 妻子(0x01) + 丈夫(0x02)");
    println!("    ├─ 外部穿衣后: state.len={} (已学会我们的编码)", them.state.len());

    // 协商验证外部是否真的"懂"了
    let (surplus, aligned) = us.negotiate_internal(&them.state);
    println!("    ├─ surplus={:.1}对齐={}", surplus, if aligned { "✓" } else { "✗" });
    println!("    └─ 结论: {}", if surplus.abs() < 50.0 && aligned {
        "外部已戴上囡囝面具 → 反向寄生成功 ☯"
    } else {
        "外部还在抵抗 → 需要更多协商拍"
    });
}

/// 演示生命周期集成
fn demonstrate_lifecycle_integration() {
    println!("\n");
    println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("┃  第四部分: 生命周期完整性 (Lifecycle Integrity)                   ┃");
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");

    let os = LinuxOs::new();
    let mut agent = CocoonAgent::new(&os, 1, "生命周期-演示", AgentRole::Root, 64);

    // Genesis → Radiation → Collapse → Fusion
    println!("\n  生命周期四阶段:");

    // Genesis (已在 new 中完成)
    agent.perceive(0xFF, 999, 0, true, 1); // 创世感知
    println!("    ├─ [Genesis ] 诞生 tick={} ✓", agent.birth_tick);

    // Radiation (分身出去做任务)
    agent.record_radiation("编译L0", "子智能体#2");
    agent.perceive(0x10, 100, 0, true, 50);
    println!("    ├─ [Radiation] 分身编译L0 ✓");

    // Collapse (分身回归)
    agent.record_collapse("子智能体#2", "编译完成");
    agent.perceive(0x20, 200, 0, false, 60);
    println!("    ├─ [Collapse ] 分身回归 ✓");

    // Fusion (与外部融合)
    agent.record_fusion("外部验证器", "见证共生");
    println!("    ├─ [Fusion  ] 融合验证器 ✓");

    // 完整性验证
    let report = agent.verify_lifecycle();
    println!("\n  完整性验证:");
    println!("    ├─ 总事件: {}", report.total_events);
    println!("    ├─ 有效见证: {}", report.valid_witnesses);
    println!("    ├─ 断裂点: {:?}", report.broken_at);
    println!("    ├─ 完成周期: {}", report.cycles_completed);
    println!("    └─ 状态: {}", if report.is_intact() { "✓ 连贯完整" } else { "✗ 存在断裂" });

    // 脉息状态
    println!("\n  脉息状态:");
    println!("    ├─ 心跳数: {}", agent.pulse.beat_count());
    println!("    ├─ 总行走: {}步", agent.total_steps());
    println!("    ├─ 节律态: {:?}", agent.breath_phase());
    println!("    ├─ 假死? {}", if agent.is_zombie() { "✗ 是" } else { "✓ 活" });
    println!("    └─ 意识痕迹: {}条", agent.state.len());
}
