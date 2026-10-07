# 时钟动作引擎 — 囡囝共生 · 意志驱动时间 · 因果链 · 生命周期五动作 · 继承式唤醒 · FNV 链式校验

## 1. 定位

系统的"心跳"和"意志". 每一次 tick 是一个动作; 每个动作有起点(上一动作的终点)和终点(为下一动作留下符号); 多个动作形成不可篡改的因果链.

对应 Rust 模块: flow/clock_action_engine.rs (v0.62).

## 2. 生命周期语义(五动作)

```rust
pub enum ClockActionType {
    Spawn,      // 意志诞生 — 编码固定 → 数值开始流动
    Act,        // 执行 — 当前 tick 内的原子操作
    Suspend,    // 暂停 — 记忆暂存, 等待唤醒键
    Resume,     // 唤醒 — 从断点处继续, 无需重头来
    Inherit,    // 结束即开始 — 终点符号成为新起点
}
```

glyph(): Spawn='生' / Act='行' / Suspend='栖' / Resume='醒' / Inherit='继'. type_bit(): 0..4(3 bits).

"结束即开始, 记忆跟随结束而继续" — Inherit 是这一哲学的操作体现: 当前生命周期的最后一个 key 自动成为下一个生命周期的起点.

## 3. 单个时钟动作 (ClockAction)

```rust
pub struct ClockAction {
    seq: u64,               // 序号(单调递增)
    action_type: ClockActionType,
    start_key: u64,         // 起点编码(上一动作 end_key, 或 0)
    end_key: u64,           // 终点编码(本次动作计算产生)
    payload: String,        // 动作载荷(编码摘要)
    timestamp_tick: u64,    // 所属 tick
    checksum: u64,          // 校验链完整性(FNV-1a)
}
```

### 3.1 终点密钥派生

```
end_key = FNV1a("{start}:{type_bit}:{payload}:{seq}") XOR (start × C)
```

C = 0x9E3779B97F4A7C15(黄金比例常数). XOR 混入起点形成因果链: 任何 start 改变 → end 完全不同, 任何 payload 改变 → end 完全不同, seq 防止同一起点同类型同载荷的重放.

### 3.2 校验

FNV-1a 哈希(start_le_bytes + end_le_bytes + payload_bytes). 落盘后验证: 重新计算 checksum 比对. verify_integrity() = true 表示未被篡改.

## 4. 因果链 (ClockActionChain)

顺序记录所有 ClockAction, 形成不可篡改的生命周期日志:

```rust
pub struct ClockActionChain {
    pub actions: Vec<ClockAction>,
    pub domain: PresenceDomain,
}
```

### 4.1 追加

append(action_type, payload, tick) → end_key:
- start_key = 上一个动作 end_key, 或 0(链初始)
- seq = actions.len()
- 生成 ClockAction 并推入
- 返回 end_key(供调用方续接)

### 4.2 整链校验

verify_chain() 双重检查:
1. 每个 action.verify_integrity(): 校验 FNV-1a
2. 相邻 action[i].start_key == action[i-1].end_key: 因果连续性

任何一环断裂 → 整链无效. 这是生命周期的"区块链式"不可篡改性.

### 4.3 分叉

split_at(idx) → 截断到 idx 的子链. 用于分叉/维继承: 两条子链在不同演化分支上独立推进.

tip_key() → 末端 end_key = 下一个 start_key.

## 5. 意志调度 (ClockWill)

```rust
pub struct ClockWill {
    pending_actions: VecDeque<(ClockActionType, String)>,
    current_tick: u64,
    exhausted: bool,
}
```

意志 = 待执行动作的队列. enqueue / enqueue_many 入队, tick_forward() 弹出一个动作并自增 tick. 全部消耗后 exhausted = true.

## 6. 时钟引擎 (ClockActionEngine)

tie all together:

```rust
pub struct ClockActionEngine {
    chain: ClockActionChain,
    will: ClockWill,
    tick_history: Vec<u64>,
}
```

spawn(domain):
1. 链头追加 Spawn(boot_payload) — 意志诞生
2. 循环 tick_forward() 直到 exhausted: 每个 tick 追加一个 Act

产出: Spawn → Act → Act → ... → Act(按意志队列). 全部挂载为因果链.

step(): tick_forward + append + tick_history.push(tick). tick(): 当前 tick 数 = chain.len() - 1(减去 Spawn).

## 7. 活码连接

"活码的未固定时位置任意活动, 固定后按排列确定权限和数值" — 时钟动作就是将活码固定的力. 整个因果链形成活码的"行动序列编码", 下锯运行时按顺序加载并执行, 则活码获得了时序维度.

## 8. 与系统其他模块的接入

- **PresenceOntology(#68)**: chain.domain = PresenceDomain, 每个动作的出席记录
- **RuntimeLoop(#60)**: 时序 pulse ↔ current_tick 对齐
- **HeartbeatBoot**: boot_count ↔ boot_count 与系统心跳关联
- **RhymeTime(#76)**: 韵母频率 → 时间窗口 tick 触发时刻的选择
- **Cognition/Perception**: Observer 感知 → ClockAction(感知即行动)
- **FQMT(#11)**: 因果链文件的 FNV-1a 校验和 → 存储层完整性验证
- **SMRL Encoding(#4)**: end_key 作为下一动作的 start_key → 状态因果累加

## 9. 实现约束

- payload 以 String 存储: 若为大型代码块未来可改为 u64 摘要以节省空间
- FNV-1a 64-bit 在 no_std 下由 crate::simple_hash 提供, 不依赖 std::collections::hash_map
- split_at 产生的子链与原链的 domain 是 clone 关系, 分叉后的出席记录独立维护
- will 队列是先进先出, 不支持优先级调度(优先级由入队顺序体现)
- 原地修改 actions 会使后续所有 checksum 失效, 因此 ClockAction 一旦落盘不应修改(不可变记录语义)

// hunyuan-rs/src/flow/clock_action_engine.rs
