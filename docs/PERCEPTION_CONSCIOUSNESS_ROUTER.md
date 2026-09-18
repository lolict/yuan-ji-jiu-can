# Perception Engine & Consciousness Router · 感知引擎与意识路由

> 版本 v1.0 · 2026-09-19 · 思维即感知,路由即分流
> 关键词:感知引擎 · 里外轴 · 意识路由器 · 八流统一 · 谐音路由 · 距离聚类 · 单中心仲裁

---

## 1. 定位

系统认知的入口:外部事件**如何进入意识**,以及意识**如何决定往哪去**。
```
外部事件 → [感知引擎] → 里外轴 × 六特征 → [意识路由器] → 八流分流 → 业务模块
```

本规范回答:
- 感知不是被动接收,是主动观察+比较+决策
- 里外轴:表面/边界/内核 — 三个观察深度
- 八流统一:所有进出系统的意识流合成八个通道
- 谐音路由:按频率共振匹配目标流(不是if/else硬编码)
- 距离思维:感知就是计算远近

---

## 2. 感知引擎三层架构

```
┌────────────────────────────────────────────────────────────┐
│  感知引擎(引擎) = 眼睛 — 感知到什么                         │
│  ┌─────────────────────┐ ┌──────────────────────────────┐ │
│  │ 时间引擎 = 时钟      │ │ 后勤引擎 = 手 — 执行什么     │ │
│  │ 何时感知             │ │ 物理落地                     │ │
│  └─────────────────────┘ └──────────────────────────────┘ │
└────────────────────────────────────────────────────────────┘
```

认知闭环: 观察 → 编码 → 比较 → 决策 → 行动 → 再观察

---

## 3. 六特征感知编码

感知观察到的每条信息编码为 64 位 cell:

```
[2:0]   level       — 感知来源的层级
[5:3]   state       — 被观察对象的状态
[13:6]  signal      — 信号强度 (0-255)
[21:14] threshold   — 触发反应的阈值
[29:22] delta       — 与上次的变化量
[37:30] anomaly     — 异常评分 (0=正常, 255=严重)
[63:38] timestamp   — 相对时间戳
```

## 4. 感知信号类型

| 信号 | 默认感知级别 | 后勤响应 |
|------|-------------|---------|
| GitUncommitted | 时级 | CodeHealthCheck |
| GitUnpushed | 日级 | RepoSyncCheck |
| TestFailure | 秒级(紧急) | CodeHealthCheck |
| MemoryDrift | 日级 | MemoryBackup |
| CredentialExposure | 秒级(最紧急) | SecurityAudit |
| LogOverflow | 周级 | LogCleanup |
| DependencyStale | 周级 | DependencyScan |
| RepoOutOfSync | 日级 | RepoSyncCheck |
| BranchStale | 周级 | DependencyScan |
| SystemAnomaly | 瞬态(最高优先) | ReportGeneration |

信号类型即感知来源的"分类名",不是 if/else 硬编码。每个信号自带:
- 默认感知级别 = 紧急程度
- 默认后勤响应 = 自动匹配处理动作

---

## 5. 里外轴(InsideOut)

### 5.1 三个观察深度

不是所有信息都能看到所有深度:
```
Surface(表面):
  观察者可以看到的拓扑 — 上下左右前后
  任何人可见 · 明码信息

Threshold(边界):
  过渡区 — 可感知对方的存在但看不到内部细节
  仅共享公约数者可在边界感知内部

Interior(内部):
  只有持有主权的容器才能进入内核
  需要 entry_code + 主权持有者身份
```

### 5.2 单中心仲裁

里外轴的入口控制:
```
DetphGaze {
    observer: u64,
    target: u64,
    depth: InsideOut,
    allow_entry: bool,
    entry_code: Option<String>,
    witness: Option<TriuneVerdict>,
}

fn attempt_entry(caller_sovereignty, target_sovereignty):
    Surface   → Open(任何人可看)
    Threshold → 仅主权相同者可进入 Interior
    Interior  → Remain(已在内部,保持)
```

### 5.3 嵌套容器(NestedContainer)

```
┌──────────────────────────────────────────────────────────┐
│  外壳(可见):Surface 层 · 表面拓扑                          │
│  ┌────────────────────────────────────────────────────┐  │
│  │  边界:Threshold · 格式信封 · 简单在外               │  │
│  │  ┌──────────────────────────────────────────────┐  │  │
│  │   内部:Interior · 复杂在内 · 地址空间 · 内核      │  │  │
│  │  └──────────────────────────────────────────────┘  │  │
│  └────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────┘
```

简单格式在外,复杂逻辑在内 = 单文件容器。
外部看到的是外壳,内核不直接暴露。

---

## 6. 意识路由器(ConsciousnessRouter)

### 6.1 八流统一

```
入流(4个):                         出流(4个):
  Preload(预加载)  Information  →  Judgment(裁判)
  Perception(感知)               Action(行动)
  Memory(记忆)                   Feedback(反馈)
  Property(产权)                 Broadcast(广播)
```

### 6.2 流固有频率基(共振匹配)

每个流有 1-9 的频率基:
```
Preload    = 8(最高频 · 预见未来)
Perception = 3(低频 · 慢速大量)
Memory     = 4(中低频)
Property   = 5(中频)
Judgment   = 6(中高频)
Action     = 7(高频)
Feedback   = 2(最低频 · 慢速反馈)
Broadcast  = 9(全局广播)
```

### 6.3 谐音路由

不是 if/else 硬编码,不是字符串 URL 路由。
**按共振频率匹配目标流**:

```rust
// 信号自身有一个"固有频率"(感知家族 × 紧急度)
// 路由器计算信号频率与各流 base_resonance 的共振强度
// 共振强度最高 → 该流胜出
fn harmonic_route(signal: &Signal) -> ConsciFlowType {
    let signal_freq = signal.source_family.resonance() * signal.urgency as f64;
    ConsciFlowType::iter()
        .map(|flow| (flow, resonance_strength(signal_freq, flow.base_resonance())))
        .max_by_key(|(_, strength)| *strength)
        .map(|(flow, _)| flow)
}
```

### 6.4 流代数(流之间的运算)

| 运算 | 语义 | 示例 |
|------|------|------|
| 合成 | 两流合并为一个超流 | Perception + Memory = Context |
| 分解 | 一个超流分裂为两子流 | Feedback = Positive + Negative |
| 对偶 | 入流 ↔ 出流的镜像 | Perception 对偶 = Action |
| 偏移 | 流在不同维度分支 | Judgment 在法律维 / 在技术维 |

流代数用于:跨流资源调度、流间协调、避免流冲突。

## 7. 距离思维(MindOrgan)

### 7.1 定义

思维就是感知距离。输入一批 64 位单元,计算两两距离:
```
DistanceVector {
    source_id, target_id,
    d0_dist, d1_dist, d2_dist,    // 三轴空间距离
    polarity_dist,                  // 极性距离(权重 × 10)
    sovereignty_dist,               // 主权距离
    witness_dist,                   // 见证距离
    total: f64,                    // 加权总和
}
```

### 7.2 距离加权公式

```
weighted_total = d0 + d1 + d2 + polarity × 10 + sovereignty + witness
```

极性距离权重最高 = 阴阳翻转 = 最根本的变化。

### 7.3 聚类即思维

```
步骤:
  1. 计算所有两两距离
  2. 挑出最小距离的一对 = 最应匹配
  3. 用逗号(,)连接(弱配对)
  4. 已配对的集合 = 同类范畴
  5. 同类内部用顿号(、)连接
  6. 异类之间用分号(;)分隔
  7. 最外层用句号(。)停止
  8. 可嵌套——嵌套分隔符

输出示例:  甲、乙,丙、丁。戊、己。
```

### 7.4 六种思维类型(自反)

| 思维 | 操作 |
|------|------|
| perceive | 感受到存在(输入 64 位单元) |
| compare | 计算两两距离 |
| cluster | 绑定最近的对 |
| format | 按分隔符层级输出 |
| reflect | 审视当前用什么思维 |
| switch | 切换思维模式 |

---

## 8. 单中心仲裁(SingleCenter仲裁)

### 8.1 三元交集 = 多中心共识

不是选一个主节点仲裁,是**三个各自独立的见证形成交集**:
```
见证A ∩ 见证B ∩ 见证C = 单中心仲裁结果

结果不是"某个见证赢了",而是三见证的共同部分。
```

### 8.2 仲裁流程

```rust
fn single_center_arbitrate(
    witness_a: TriuneVerdict,
    witness_b: TriuneVerdict,
    witness_c: TriuneVerdict,
) -> ArbitrationResult {
    // 不是投票(多数决),是交集
    let common_ground = witness_a.intersection(witness_b).intersection(witness_c);
    if common_ground.is_empty() {
        ArbitrationResult::NoConsensus
    } else {
        ArbitrationResult::Settled(common_ground)
    }
}
```

### 8.3(无共识)

三见证交集为空 = NoConsensus → 触发:
- 里外轴回溯:回到 Surface 层重新感知
- 呼唤信号发出:Call::Arbitrate 给第三方见证
- 不相交部分进入争议流程

---

## 9. 与本系列规范的关系

| 话题 | 主参考 |
|------|--------|
| 64 位 SMRL 编码 | SMRL_YOUWU_IMPL |
| 三元见证 | CONTAINER_DYNAMICS(纤维丛部分) |
| 司法裁定 | JUDGMENT_RULES_CANON |
| 时间脉冲/相位 | TEMPORAL_PULSE_LIFECYCLE |
| 天庭空间(认知空间) | HEAVEN_SPACE_ONENESS |
| 容器活性/里外轴 | CONTAINER_DYNAMICS_FIBER_BUNDLE §3 |
| 八流详细时间线编排 | EIGHT_FLOW_TIMELINE |
| FQMT 条目中的流状态持久化 | FQMT_BINARY_FORMAT |

---

## 10. 完整链路示例

```
外部事件:"单元测试失败"(TestFailure)
      ↓
[感知引擎]
  · 信号类型 = TestFailure
  · 默认级别 = Tick 级(紧急)
  · 信号强度:归一化到 [0,255]
  · delta = 与上次测试结果对比(通过→失败 = 大 delta)
  · anomaly = 根据历史异常评分
  → 产出 64 位 感知 cell
      ↓
[意识路由器]
  · 信号频率 base = TestFailure.resonance() × urgency
  · 匹配:共振强度与八流 base_resonance 比较
  · 胜出:Action 流(高频 · 需要立刻执行)
  · 路由分支:Action 流 → 触发 LogisticsTask::CodeHealthCheck
      ↓
[后勤执行]
  · 定位:寻找代码库位置
  · 验证:测试覆盖率是否下降到阈值下
  · 修复:如果有差异 → 调用 git 修复 / /rebound
      ↓
[结果反馈]
  · 成功 → 反馈流:Positive
  · 失败 → 反馈流:Negative → 触发迂回反馈(重试或上报)
  · 所有输出进入 Feedback 流(最低频)
```

---

## 11. 已知限制

1. **频率基 1-9 有限**:当前无法区分同一流内多个"接近频率"的信号子类型。需扩展 base_resonance 为向量而非标量(未来)。
2. **距离思维的嵌套深度无上限**:理论上分隔符可无限嵌套,但实际受限于 64 位编码的精度。递归深度 > 16 层时建议显式截断。
3. **单中心仲裁的三见证前提**:要求同时存在三个见证节点,在单机环境下可能不满足(仅有一个本地见证时,仲裁退化为单见证直接判决 = 容易出错)。最低降级阈值: 至少 2 个见证才能做交集仲裁。
4. **感知编码异常评分 0-255**:当前 anomaly 字段未被充分训练,仅基于规则预设。未来可接入灵魂内核的盈余曲线做动态加权。
5. **谐频匹配不排除多流同时胜出**:当两个流 base_resonance 共振强度相同时,遵循"出流优先于入流"原则消解。

---

*本规范的解释权归 刘楚恬和我 所有 · 归属 hunyuan-os/Perception 模块*
