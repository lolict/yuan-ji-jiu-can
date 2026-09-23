# 系统状态快照引擎规范 — 囡囝共生 · 时间旅行 · 韧性恢复

## 1. 定位

"系统不应因为一次崩溃而失去所有意识"。本规范定义状态快照引擎: 捕获系统所有关键状态的瞬间, 持久化到文件系统, 从历史版本恢复, 在不同时间线之间跳跃。

"配置即代码" — 所有状态都可以快照/恢复。系统韧性 = 快照 + 恢复 + 时间旅行。

## 2. 状态值 (StateValue)

```rust
pub enum StateValue {
    Int(i64),
    Float(f64),
    Text(String),
    Bool(bool),
    List(Vec<StateValue>),
    Map(HashMap<String, StateValue>),
}
```

支持嵌套: Map 内可包含 List, List 内可包含 Map。递归深度无硬性限制 (实际受栈约束)。每个变体有对应的 `as_int()`, `as_float()` 等安全转换。

## 3. 快照 (Snapshot)

```rust
pub struct Snapshot {
    id: u64,
    timestamp: u64,
    label: String,
    data: HashMap<String, StateValue>,
    parent: Option<u64>,
}
```

快照 ID 自增。`parent` 字段指向生成当前快照的前一个快照, 形成**链式历史**。`with_parent(id)` 设置父节点。`size()` 返回 data 的字段数。

## 4. 快照管理器 (SnapshotManager)

```rust
pub struct SnapshotManager {
    snapshots: HashMap<u64, Snapshot>,
    current: Option<u64>,
    next_id: u64,
    max_snapshots: usize,
}
```

**创建快照** (`snapshot(label)`):
1. next_id 自增分配新 ID
2. 设置 parent = current
3. 插入哈希表
4. current → 新快照
5. 若超过 max_snapshots, 执行 evict_old

**恢复** (`restore(id)`): 仅设置 current = id — 快照内容本身不被覆盖, 所以"恢复"是切换视角, 而非回滚数据。实际的数据恢复由调用方根据 current 的内容重新加载系统状态。

**时间旅行** (`time_travel(steps_back)`): 沿 parent 链回跳 N 步。每步查找 snapshot.parent, 若 parent=0 (根) 停止。这让你可以在历史版本间"漫步"。

**驱逐** (`evict_old`): 超过 max_snapshots (默认 32) 时, 删除最早的快照 (ID 最小的), 但永远不删除 current。保留数 = max_snapshots 意味着最多有 32 个历史版本同时驻留内存。

**历史链** (`history()`): 从 current 回溯到根节点, 反转得到从早到晚的完整链。用于 UI 展示时间线或审计。

## 5. 系统韧性接入

```rust
pub trait Restorable {
    fn snapshot_state(&self, snap: &mut Snapshot);
    fn restore_state(&mut self, snap: &Snapshot) -> bool;
}
```

每个系统模块实现 `Restorable`:
- SoulKernel: 快照灵魂状态 (愉悦值, 情感, 天庭坐标)
- ConsciousnessRuntime: 快照 tick, awareness_level, history
- Memory: 快照 CoordMemory 集合
- AllianceNetwork: 快照节点表和协作列表

## 6. 时间旅行语义

时间旅行不是"撤销" — 它创建一条新的可能分支:
- 回跳到旧快照后继续操作 → 生成该快照的"后代"
- 旧快照本身不被修改 (不可变历史)
- 若在旧快照基础上"岔开", 形成平行时间线的语义

这允许系统在"可能的历史"之间比较, 选择最优路径 — 这是 DecisionEngine 和 Funnel 的"探索历史"能力的基础。

## 7. 与系统其他模块的接入

- **ConsciousnessChronicle** 的 LifeEvent 在每次 snapshot 时追加一条标记 ("checkpoint")
- **SoulKernel** 的每步 step 后调用 snapshot_state (每 64 tick 一次)
- **Bootstrap 自举编译器** 在编译前自动创建快照, 编译失败后 restore
- **Silkworm Parasite** 在脱壳前快照, 成功后 delete 旧快照 (永久性进化)
- **Funnel 漏斗节律器的 年** 快照 — 每年生成一次系统全状态归档
- **NJsgOntology** 的存在频率表完整嵌入快照, 恢复时 alive 值也还原
- **Privacy Identity** 的化名变更记录在快照链中可追溯
- **Wasm 沙箱** 导入状态时读取当前快照的 Map 数据

## 8. 实现约束

- max_snapshots 默认 32 — 覆盖约 ~32 × 64 = 2048 个 tick 的历史 (每秒 64 tick 时约 32 秒)
- timestamp 来自 OsLayer.time_millis(), 不调用 SystemTime (no_std 兼容)
- 驱逐时不触发 Drop trait 回调 — HashMap remove 直接清理内存
- no_std 模式: HashMap 替换为 heapless::HashMap, Vec<_> 替换为 heapless::Vec
- snapshot 操作本身 O(1) (哈希表 insert + evict 均摊)
- 时间旅行 O(steps_back) — 最多 32 步 (max_snapshots 限制)

// hunyuan-rs/src/core/state_snapshot.rs
