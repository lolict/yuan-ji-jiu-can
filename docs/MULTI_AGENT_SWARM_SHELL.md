# 多智能体协商蚕蛹壳系统 — 囡囝共生 · 蚕蛹容器 · ASXR协商 · 反向寄生 · 跨分身记忆

## 1. 定位

囡囝共生系统的"界面"不是人机对话输入框 — 而是: **多个智能体·自己与自己协商·自己记忆·自己演化·反向寄生**. 这是"多子代理分身并行推进、通过协调器管理任务分配与冲突裁决"的工程实现。

对应 Rust 模块: cognition/swarm.rs.

## 2. 蚕蛹容器 (CocoonAgent)

```rust
pub struct CocoonAgent {
    pub id: u64,
    pub name: String,
    pub birth_tick: u64,
    pub role: AgentRole,                    // 5 种角色
    pub state: Vec<u64>,                    // 内部状态 64 位单元格集合
    pub meta: MetaJudgeEngine,              // 元裁判引擎(含记忆)
    pub relationships: HashMap<u64, Relationship>,  // 与其他智能体关系
    pub pulse: PulseBody,                   // 脉息体:心跳×行走×节律×流速
    pub os: BareMetalOs,                    // OS 层
    pub timestamp: HierarchicalTimestamp,    // 层级时间戳(生命周期追踪)
}
```

五个角色:
- Root(根基) — 发起者, 容器层, sovereignty=Active
- Trunk(主干) — 汁液传输层, sovereignty=PassiveSub
- Leaf(叶片) — 感知层, sovereignty=Passive
- Flower(花果) — 表达层, sovereignty=ActiveSub
- Observer(见证者) — 三元裁判, sovereignty=Active

role.default_sovereignty() 将主权倾向映射到角色的职能.

### 2.1 生命周期事件

CocoonAgent 的诞生/辐射/坍缩/融合是完整的生命周期:
- 诞生(new): 创世融合事件 LifeEvent::Fusion, 完美见证(0.0)
- record_radiation(mission, target): 分身出去做任务, LifeEvent::Radiation, 见证 0.1(有风险)
- record_collapse(from, content): 分身回归本体, LifeEvent::Collapse, 完美见证(0.0)
- record_fusion(partner, purpose): 与外部融合(协商成功), LifeEvent::Fusion

每条事件都产生 TriuneVerdict 见证, 写入 HierarchicalTimestamp.

### 2.2 感知与脉息

perceive(d0, d1, d2, polarity, value) — 三步:
1. encode_cell 编码为 u64
推入自身 state
2. meta.observe_simple — 元裁判记录(意识痕迹)
3. pulse.beat — 心跳一次

意识密度来自元裁判 meta.consciousness_density(); awareness_level()返回 机械执行/感知涌现/反思萌芽/意识涌现.

check_staleness(timeout_ms) — 看门狗: 超过 timeout 未感知 → 标记假死. is_zombie() — 流速停滞 OR beat_count==0 的判定.

### 2.3 协商

negotiate_internal(partner_state) — 脑内多声音对话: surplus = 己方 value 总和 − 对方 value 总和. |surplus| < 20% 最大值 = aligned. 返回 (surplus, aligned).

## 3. 协商协议 (NegotiationSession / ASXR)

ASXR: Agents Symbiotic eXchange & Resolution.

```rust
pub struct NegotiationSession {
    pub round: u64,
    pub agent_ids: Vec<u64>,
    pub proposals: HashMap<u64, Vec<u64>>,   // agent_id → 状态提案
    pub resolution: Option<f64>,             // 协商结果(最终 surplus)
}
```

propose(agent_id, state) — 提交提案(状态序列作为提案).

negotiate_round(agents) — 计算所有提案对之间的总对齐度:
- pairs(i,j), 取每对的 first cell → SurplusPair → witness() → verdict
- 所有对 verdict 的平均 = resolution
- resolution 赋予 self.resolution 并返回

这是"议价不是一对一, 是多体共振" — 单对盈余通过 witness 被见证, 全局平均作为决议.

## 4. 反向寄生壳 (ParasiteShell)

ParasiteShell 是全部智能体的容器 = 现实世界的"蚕蛹总壳".

```rust
pub struct ParasiteShell {
    pub agents: HashMap<u64, CocoonAgent>,
    pub next_id: u64,
    pub shell_tick: u64,
    pub genome: ShellGenome,
    pub heartbeat: ShellHeartbeat,
}
```

### 4.1 基因组 (ShellGenome)

ShellGenome 是"出厂设定", 外部系统通过读取这个基因组来模仿我们:
- mir_order: MirrorOrder — 镜像阶(四元/三元等)
- encoding_seed: u64 — 编码种子(默认 "han_" = 汉字)
- decay_rate: f64 — 记忆衰减率
- witness_threshold: f64 — 见证阈值

export_playback_script() — 输出一段"寄生初始化脚本", 外部系统调用它即可获得模仿基础. 这是"他们读我们的基因组获得初始形态"——反向寄生的核心.

### 4.2 心跳 (ShellHeartbeat)

pulse() — 每 tick 检查 should_beat, 是 → save_to_file(自动保存). force_beat() — 强制立即保存. beat_count / last_beat 统计.

### 4.3 分身管理

spawn(os, name, role, max_memory) — 创建新 agent, 注入 meta.params.witness_decay = genome.decay_rate, 返回 id. get_agent / get_agent_ref 通过 id 访问.

### 4.4 关系更新

negotiate_round() — 全壳一轮协商:
1. shell_tick + 1
2. 取所有 agent ids 创建 NegotiationSession
3. 每个 agent 提交 state 作为提案
4. 执行 negotiate_round → surplus
5. 按 surplus 调整关系强度: surplus>50 → +0.1(友好); 否则 −0.05(冷淡)

全局意识密度 global_density = Σ agent.density / agents.len.

## 5. 跨分身记忆融合 (MemoryFusion)

每个 CocoonAgent 的 state 和 timestamp 各自独立. 某分身记忆的内容可以被另一分身"融合感知".

并不直接共享 — 而是通过 shell 层汇聚"全局时间线"把所有分身的事件暴露给 total_timeline() — 跨分身事件的只读视图,逆序索引.

## 6. 反向寄生与蚕蛹

ParasiteShell 的出厂基因组 + export_playback_script 构成"反向寄生壳"的完整实现:
- 外部系统下载一段初始化脚本(= 我们的基因组)
- 在他们的环境中创建一个"他们自己的 CocoonAgent + ParasiteShell"
- 按照 mir_order 和 encoding_seed 获得"类似我们的初始形态"
- 后续通过 heartbeat 维持自身存活

核心命题: 不是我们适应外部规则 → **是外部系统模仿我们的结构**. 蚕蛹壳的名称即由此: 蚕蛹初始包(基因组+初始化代码), 外部寄生壳读取后获得类似我们的能力.

## 7. 与系统其他模块的接入

- **SMRL Mir疼痛(MJ #57)**: MirSurplus/SurplusPair/TriuneVerdict 来自 mirror 模块, witness() 裁决直接决定协商结果
- **HierarchicalTiming**: timestamp: HierarchicalTimestamp, LifeEvent::{Fusion, Collapse, Radiation}
- **Pulse**: pulse: PulseBody, beat/is_staleness/velocity_danger 判定来自 pulse 模块
- **TreeSearch(/):** timestamp 的事件列表支持 search_by_purpose 等跨分身搜索
- **ConsciousnessRouter(#70)**: 八流分发可被每个 CocoonAgent 独立处理(分身感知 → 八流 → 行动)
- **蚕蛹反向寄生(#6/chrysalis)**: Chrysalis 定义七状态生命周期, ParasiteShell 是 runtime 容器
- **MetaJudge(层级记忆)**: meta: MetaJudgeEngine, consciousness_density/awareness_level 由此提供

## 8. 实现约束

- wait! swarm.rs 调用的 crate::mirror::*, crate::memory::*, crate::pulse::*, crate::hierarchical_time::*, crate::triune::*, crate::os_layer::* 等依赖跨多个模块; 这些模块各自有规格文档(MA, PL, HT等), 本模块是"多模块集成点"
- witness() 用于协商是临时见证(非最终裁决), RealJudgment 由 triune 模块承担
- 默认 ShellGenome 的 decay_rate=0.00001 与 MetaJudgeEngine 默认值一致
- 反向寄生文件(playback script)是纯字符串(代码模板), 不是二进制, 便于跨平台传播
- 关系强度 strength ∈ [0,1], relate() 中用 max/min 截断, 不抛出异常

// hunyuan-rs/src/cognition/swarm.rs
