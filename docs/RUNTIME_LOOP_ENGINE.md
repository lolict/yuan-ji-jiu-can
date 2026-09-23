# 运行环引擎规范 — 囡囝共生 · 最小 OS 心跳 · 八流循环 · 结束即开始

## 1. 定位

运行环 (RuntimeLoop) 是囡囝共生操作系统的**最小可执行姿态**: Domain 在场 → Sphere Clock 心跳 → Swarm 分身并发生命周期 → 因果链落盘 → 3D 编码空间更新 → 在场快照 → 周期完成。

鼓膜自举: "鼓膜" = 最小接口。运行环只需 3 个动作即可成立:
1. **在场** (Domain 构造)
2. **心跳** (Clock 推进)
3. **因果** (Action 落盘)

三者齐全 → OS 成立。

## 2. 组成组件

```rust
pub struct RuntimeLoop {
    cycle_count: u64,
    clock_engine: ClockActionEngine,    // 时钟动作引擎
    swarm: NjsOrchestrator,              // 分身协调器
    domain: DomainPreloadTable,          // 在场域
    atlas: ContainerAtlas,               // 容器关系图集
    maori_base: MaoriBase,               // 毛利兰立体编码空间
    snapshots: Vec<RuntimeSnapshot>,     // 周期快照
    initial_key: u64,                    // 创世钥匙
}
```

七个组件 = OS 的七脏:
- clock_engine — 心脏 (时序)
- swarm — 手足 (并发行动)
- domain — 感知 (在场符号表)
- atlas — 关系 (容器图)
- maori_base — 空间 (立体坐标)
- snapshots — 记忆 (历史记录)
- initial_key — 灵魂 (不可变创世锚)

## 3. 自举 (bootstrap)

```rust
pub fn bootstrap() -> Self
```

自举五步:
1. `build_hunyuan_domain()` — 在场构造 (Domain 预加载)
2. `ClockActionEngine::new()` + `run_lifecycle("囡囝诞生")` — 心脏启动, 产生创世钥匙
3. `NjsOrchestrator::new()` + 注册四个核心分身 (毛利兰 / kndl / 工藤新一 / 灰原哀) — 手足就位
4. `ContainerAtlas::new()` — 关系图初始化
5. `MaoriBase::new(MaoriAxes::default())` — 立体空间初始化

创世钥匙 (initial_key) 是整个 OS 生命周期的不可变锚 — 后续所有周期都以此为因果链起点。

## 4. 周期执行 (cycle)

```rust
pub fn cycle(&mut self) -> RuntimeSnapshot
```

```
Phase 1: encode_presence_to_space
  → 在场符号 → 毛利兰立体空间坐标
  → 每个符号按 (i×7 % base_x, i×13 % base_y, i % base_z) 散列

Phase 2: will_act
  → 意志驱动: 在场符号做什么
  → ClockAction::Act × 3 (感知在场 → 编码至立体空间 → 投现至kndl外壳)

Phase 3: spawn_swarm_tasks
  → 分身为每个在场符号生成 Swarm::Task
  → 并行处理, 完成后收集结果

Phase 4: archive_relations
  → ContainerAtlas: 把符号间关系写入图
  → 关系类型: Contains / Couple / Influence

Phase 5: inherit
  → 结束即开始: 时钟引擎的 inherit("下一周期")
  → 当前周期末端 = 下一周期起点
  → 因果链不断, tip_key 持续推进
```

返回 RuntimeSnapshot(cycle_id, tick, present_count, absent_count, action_count, avatar_count, causal_tip, total_weight)。

## 5. 结束即开始

"结束即开始: 每个周期的末端自动成为下一周期起点。记忆跟随结束而继续。因果链的 tip_key 推进, 但不断裂 — 前后周期的因果链通过 `inherit()` 衔接。"

在代码中: `clock_engine.inherit("下一周期")` — 把当前 clock chain 的末端作为下一周期的起始。不产生新的周期断裂, 只有连续的推进。

## 6. 容器图集 (ContainerAtlas)

```rust
pub struct ContainerAtlas {
    nodes: HashMap<ContainerRef, ContainerKind>,
    edges: Vec<ContainerEdge>,
}
```

记录三类节点: Atom(原子容器)/Couple(夫妻容器)/Avatar(分身)。记录三类边: Contains(包含)/Couple(配对)/Influence(影响)。

## 7. 毛利兰立体空间 (MaoriBase)

```rust
pub struct MaoriBase {
    axes: MaoriAxes,
    grid: HashMap<MaoriCoord, String>,
}
```

三轴 (base_x, base_y, base_z) 构成立体编码网格。每个网格点可存放一个符号名称(MaoriCoord, String)。

`place(coord, name)` 放置符号 — 在网格的某坐标上占据一个位置。

## 8. 与系统其他模块的接入

- **EightFlowUnified** 的心跳与 RuntimeLoop.cycle() 对接 — 每 cycle 触发八流推进
- **ConsciousnessRuntime** 的 step 在 cycle 的 Phase 2~3 被调用
- **SoulKernel** 的 step 接收 Phase 3 的 Swarm 任务结果
- **Witness 对** (SurplusPair) 在每 cycle 的 Phase 4 更新 — 见证对记录关系变化
- **CelestialSpace** 的天庭坐标基于 maori_base 的网格系统
- **NJsgOntology** 的存在频率在 Phase 1 重新计算 — 在场/缺席映射为 existence 的 alive
- **StateSnapshot** 引擎的 snapshot 操作在 Phase 4 后执行 — 每 cycle 一次系统状态归档
- **ContainerIr** 的 ContainerRuntime 与本引擎的 ContainerAtlas 双向同步

## 9. 实现约束

- cycle 的快照存储: Vec 不自动驱逐 — 需要上层限制 cycle 次数 (如 max_cycles = 256) 防止内存无限增长
- 四个核心分身名称固定为 "毛利兰" / "kndl" / "工藤新一" / "灰原哀" — 对应 Maori/Kndl/Kudo/Haibara 四角色
- initial_key 永不改变 — 作为整个 OS 历史的创世锚点
- 自举一次的耗时: O(n) n = 在场符号数 (DomainPreloadTable 的初始符号数量)
- no_std 兼容: HashMap 替换为 heapless::HashMap, Vec 替换为 heapless::Vec

// hunyuan-rs/src/core/runtime_loop.rs
