# 层级时间戳系统 — 囡囝共生 · 三元见证替代哈希链 · 生命周期事件 · 完整性证明 · 持久化协议

## 1. 定位

基于三元见证的时间层级追踪, 替代传统哈希链. 核心差异: 不需要密码学哈希 — 三元见证本身 = 完整性证明.

对应 Rust 模块: temporal/hierarchical_time.rs.

## 2. 核心思想

- 不需要密码学哈希 — 三元见证本身 = 完整性证明
- 坍缩(入) + 辐射(出) 的循环 = 时间的自然方向
- 验证: 不是靠哈希比对 — 靠"见证链是否连续合法"
- 每一个新事件必须被上一个三元见证裁定为"可坍缩/可辐射"
- 篡改一个事件 = 打破见证连续性 = 立刻被发现

## 3. 生命周期事件 (LifeEvent)

```rust
pub enum LifeEvent {
    Fusion { subject, object, purpose, witness },    // 融合: 外部进入本体
    Collapse { from, content, witness },              // 坍缩: 分身回归本体
    Radiation { to, mission, witness },               // 辐射: 本体向外分配
}
```

每条事件携带一个三元见证(TriuneVerdict).

- timestamp(os) → os.time_millis(): 依赖 OS 层获取毫秒时间
- flow() → 事件内的 witness.flow
- is_witnessed() → witness.confidence > 0.5: 弱置信(<0.5)视为"不可信事件"

## 4. 层级时间戳 (HierarchicalTimestamp)

```rust
pub struct HierarchicalTimestamp {
    pub genesis: Vec<TriuneVerdict>,   // 创世见证链(不可变根)
    pub events: Vec<LifeEvent>,        // 生命周期事件序列
    pub anchor: NamingAnchor,          // 命名锚点
    pub cycle_count: u64,              // 坍缩-辐射循环次数
}
```

### 4.1 创世(genesis)

创世见证 = 无中有(系统从"无"坍缩出"有"):
```rust
TriuneVerdict::deduce(BeingState::Being(1), BeingState::NonBeing, 0.0)
```
实际=有, 预期=无, 无歪斜 → Collapse(坍缩创世).

version() = events.len + 1.

### 4.2 记录(record)

```rust
record(event) -> Result<(), String>:
  assert event.is_witnessed()   // 置信度必须 > 0.5
  events.push(event)
  cycle_count += 1
```

未被合法见证的事件直接拒绝, 不进入时间链.

### 4.3 完整性验证

```rust
verify_integrity() -> IntegrityReport:
  for (i, event) in events:
      if !event.is_witnessed():
          broken_at = Some(i); break
```

遍历所有事件, 任何一个置信度 < 0.5 = 链断裂, 找到第一个断裂点即停止.

is_intact() = broken_at.is_none() && valid_witnesses == total.

### 4.4 当前相位

current_phase() → 最后一条事件的 flow = 当前系统在坍缩还是辐射.

## 5. 持久化协议 (HTS_V1 文本协议)

```
HTS_V1
version:N
cycles:N
FUSION|"subject"|"object"|"purpose"|being_val|flow|align|conf
RADIATION|"to"|"mission"|being_val|flow|align|conf
COLLAPSE|"from"|"content"|being_val|flow|align|conf
```

零外部依赖, 纯文本, 可人工阅读/编辑.

## 6. 整体数据结构

```
HierarchicalTimestamp
├── genesis: [TriuneVerdict]     (创世见证链, 不可变根)
├── events:  [LifeEvent]         (生命周期事件序列)
├── anchor:  NamingAnchor        (命名锚点)
└── cycle_count: u64             (坍缩-辐射循环计数)
```

## 7. 与其他时间模块的关系

- ClockActionEngine(#81): ClockAction 是"单步动作"; LifeEvent 是"生命周期大动作"
- HierarchicalTime vs Clock 引擎: Clock 引擎面向"操作时序", HT 面向"宇宙历史"
- "创世"不是"启动",而是"从无到有的坍缩" — 这一哲学差异影响整个时间观

## 8. 实现约束

- 创世见证链目前只有 1 个元素(single), 未来可扩展为多见证 GenesisQuorum
- anchor 字段暂时未在 genesis()/record() 中主动使用, 作为扩展点的命名锚点保留
- record() 的 error 用 String, 未定义专门错误类型(简化)
- HTS_V1 文本协议是"导出格式", 源码侧 HierarchicalTimestamp 仍在内存中
- 持久化 IO 由上层 FQMT(#11) 或 MetaIME 文件系统执行

// hunyuan-rs/src/temporal/hierarchical_time.rs
