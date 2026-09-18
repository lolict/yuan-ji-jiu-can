# Container Dynamics & Fiber Bundle · 容器动力学与纤维丛

> 版本 v1.0 · 2026-09-19 · 容器不是死囊,是活的关系节点
> 关键词:呼唤编程 · 迂回反馈 · 判断之力 · 评价之力 · 纤维丛拓扑 · 反射感知 · 逻辑组织

---

## 1. 定位

传统系统把容器当"储物格"。
NJSG 把容器当作**能互相呼唤、能传递反馈、能在彼此间形成关系活的网络**。

本规范覆盖:

| Q | A |
|---|---|
| 容器怎么分类? | 动态/静态/失活性 — 按"谁先呼唤谁"区分 |
| 谁在观察谁? | 观察者/被观察者/互看 — 三种站位 |
| 容器用什么连接? | 公约数 — 只有双方有公共因子,才能共振 |
| 反馈怎么传? | 直接/负向/迂回 — 迂回反馈通过第三容器传递 |
| 判断从哪来? | 从相邻时间戳的涟漪模式差异中反射感知 |
| 评价从哪来? | 对因果对排序 — 哪种关系值得强化 |
| 纤维丛是什么? | 容器之间的可感知关系拓扑,底空间 × 纤维方向 |

---

## 2. 容器活性(Vitality)

### 2.1 三态

```rust
enum Vitality {
    Dynamic,     // 主动发起交互,能呼唤其他容器
    Static,      // 被动等待,只有被呼唤时才响应
    Dormant,     // 不再参与交互 (休眠/废弃)
}
```

### 2.2 状态转换

```text
Dynamic ──失活超龄──▶ Dormant
Static  ──主动唤醒──▶ Dynamic
Dormant ──再注册─+─▶ Dynamic
                +─▶ Static(若仅恢复部分功能)
```

失活超龄:容器连续 TICK_DORMANCY_MAX(默认 4096)个 tick 无任何呼唤 → 自动转为 Dormant。

再注册:外部调用 `Container::rebirth()` → 允许再次激活,但 genesis_id 不变(身份锚点永久)。

### 2.3 活性判定规则

```
Dynamic 容器:
  · 每 tick 可主动发起最多 MAX_OUTBOUND_CALLS = 8 次呼唤
  · 可监听 /offer 广播,匹配后自动回应

Static 容器:
  · 不主动发起呼唤
  · 但在收到匹配的 /need 呼唤时,会产生 /offer 回应
  · 可定期(每 64 tick)发出心跳 /ping,告诉外部"还活着,必要时可唤起"

Dormant 容器:
  · 不回应任何呼唤(被调用时返回 CallResponse::DeadRelay)
  · 不更新反馈环路
  · 保留在场登记但不参与排名
  · 可被 Rebirth 事件激活
```

---

## 3. 观察者站位

### 3.1 三种站位

```rust
enum Observance {
    Observer,    // 我在看别人 → 我是主动方
    Observed,    // 别人在看我 → 我是被动方
    Mutual,      // 互看 → 对称关系
}
```

### 3.2 观察行为 Gaze

```
Gaze {
    observer_genesis: u64,    // 观察者身份
    observed_coord: TimeCoordinate,  // 被观察者坐标快照
    observance: Observance,          // 站位方向
    intensity: f64,                  // 观察强度 0.0..1.0
    witness: Option<TriuneVerdict>, // 见证
}
```

观察强度 determinants:
- 距离越近 → 强度越高(见 MORPHISM_ALGEBRA C_eff 加权距离)
- 公约数越大 → 共振越强 → 强度越高
- 观察者 standing = 有 = controlled → 强度 × 0.5(被控制时感知变窄)

### 3.3 观察者/被观察者关系的不对称性

```
观察者(Observer):
  · 掌握主动权 — 决定观察谁、何时停止
  · 看到"整体" — 被观察者在观察者面前的形象是被选择的切面

被观察者(Observed):
  · 不掌握主动权 — 不能拒绝观察(但可返回 DeadRelay)
  · 看到"局部" — 观察者仅看到自己想看到的内容

互看(Mutual):
  · 双方都是观察者也是被观察者
  · 仅当两容器同时发起 → 这种关系最稳定
  · 夫妻关系默认站位=Mutual
```

---

## 4. 呼唤编程(Call Programming)

### 4.1 核心理念

```
传统: push 数据.send(msg) — 发送者决定一切
呼唤: pull-by-need.need(resource) — 接收者决定何时响应
```

呼唤编程 = 不是"我要推送给你",而是"我被需要"才响应。
解决的不是**通信速度**,而是**通信主权**:谁先谁后的政治问题。

### 4.2 五种呼唤

```rust
enum Call {
    Need { resource, urgency },    // "我缺 X,谁能给我"(urgency 1-9)
    Offer { resource, quantity },  // "我有 X,谁需要"
    Ping,                          // "你还在吗"
    Pong { responder_genesis },    // "我在"
    Arbitrate { claimant_a, claimant_b, dispute },
}
```

urgency 等级:
| 值 | 语义 | 响应时间要求 |
|----|------|-------------|
| 1 | 心跳级 | 64 tick 内 |
| 3 | 普通需求级 | 16 tick 内 |
| 8 | 紧急级 | 4 tick 内 |
| 9 | 裁决赛(最优先) | 1 tick 内 |

### 4.3 公约数是门槛

呼唤不是任意可达——需要**双方有公共因子**:

```rust
// 容器 A 呼唤容器 B
fn is_call_permitted(a_genesis: u64, b_genesis: u64) -> bool {
    let gcd = calculate_gcd(a_genesis, b_genesis);
    gcd > 0  // 有公约数 = 关系合法门槛
}
```

公约数越大 → 双方共振越强 → 呼唤衰减越小 → 反馈环路越清晰。

### 4.4 呼唤信号结构

```
CallSignal {
    caller: u64,           // 发起者
    target: u64,           // 目标者
    covenant_divisor: u64, // 公约数(门槛)
    priority: u8,          // 优先级 0-15 (15=裁决)
    payload: CallPayload,  // Read|Write|Judge|Arbitrate|Heartbeat|Custom
    timestamp: u64,
    depth: u32,            // 递归层数(防死循环)
    expects_feedback: bool,// 是否期待返回
}
```

约束:depth > MAX_CALL_DEPTH(默认 8) → 强制返回 `CallResponse::TooDeep`,不允许再嵌套。

---

## 5. 反馈环路

### 5.1 类型

```rust
enum FeedbackKind {
    Positive,   // 正向反馈:关系增强(供应满足需求)
    Negative,   // 负向反馈:关系衰减(需求未被满足)
    Detour,     // 迂回反馈:通过第三容器传递
}
```

### 5.2 传播与衰减

每次反馈经过一个容器时,按公约数衰减:

```
attenuation = 1.0 / (common_divisor + 1.0)
strength -= attenuation * 0.1
```

- 公约数小(关系疏远) → attenuate 大 → 衰减快
- 公约数大(关系紧密) → attenuate 小 → 衰减慢

### 5.3 闭环效应

反馈回到发起者时 = 闭环(closed loop)。闭环后按类型调整:

| 类型 | 闭环后强度变化 |
|------|---------------|
| Positive | × 1.2(增强) |
| Negative | × 0.5(衰减) |
| Detour | × 1.0(不变) |

闭环是关系的"节律点"——系统稳定/衰减的分水岭。

### 5.4 迂回反馈

不是直接 A→B→A,而是 A→C→B→A:
- 迂回引入第三方视角 → 更客观
- 迂回路径更长 → 强度衰减更多 → 噪声过滤效果更好
- 迂回反馈是反思(reflection)的物理基础

---

## 6. 四阶感知链:力量从哪里来

### 6.1 链路结构

```text
迂回反馈 → 反射感知 → 判断之力 → 评价之力 → 逻辑组织 → 反馈
  (涟漪)     (差异标记)   (关系锚点)  (因果排序)  (规则固化)  (回输)
```

四阶感知链不是四层分离处理,是同一时间在每一次反馈传播中依次完成。

### 6.2 涟漪模式(Ripple)

```
Ripple {
    timestamp: u64,     // 时间印记
    source_id: String,  // 信号源
    payload: T,         // 信号内容
    amplitude: f64,     // 幅度(强度)
    phase: f64,         // 相位偏移
}
```

涟漪是容器间反馈的原始物理量:信号源在时间链上留下的连续印记。

### 6.3 反射感知(ReflectionPerception)

= 比较两个相邻涟漪,觉察差异

```
ripple.diff(adjacent_ripple) → RippleDiff {
    amp_delta,        // 强度变化
    phase_delta,      // 相位变化
    kind: Amplitude|Phase  // 差异类型
}
```

感知的不是一种事物,是**变化**。无差异 = 无感知。

### 6.4 判断之力(JudgmentPower)

= 将差异绑定为因果对(relationship anchor)

```
判断回答: "这个变化是由谁引起的?"

输出: CausalPair {
    source_id: u64,
    target_id: u64,
    feedback_count: usize,
    total_quality: f64,
    avg_quality: f64,
    trend: f64,      // 正=改善 / 负=恶化
}
```

判断之力不是外部输入的裁定规则——它是**从关系内部涌现的能力**。

### 6.5 评价之力(EvaluationPower)

= 对因果对排序(哪些关系值得强化)

```
评价回答: "多种关系中,哪种应该优先强化?"

排序依据:
  高 avg_quality → 稳定可靠的关系
  高 trend → 正在改善的关系
  高 feedback_count → 频度高的关系
  低 latency → 响应快的关系
```

评价结果最终固化为规则 → 规则反馈回呼唤策略。

---

## 7. 纤维丛容器拓扑

### 7.1 数学对应

纤维丛不是抽象符号——在 NJSG 中是容器可感知的关系拓扑:

```
底空间 B = 容器坐标语义(SMRL 三维:语义·操作数·时间)
纤维 F = 每个容器内禀属性空间(极性·主权·价值·见证)
全空间 E = B × F(容器总空间)
截面 s: B → E = 此刻"在线"的容器选择函数
```

### 7.2 纤维方向(five directions)

```rust
enum FiberDir {
    Belong,     // A ⊃ B(A 归属于 B · 父子套娃)
    Invoke,     // A → B(A 调用 B · 参数传递)
    Witness,    // A ◇ B(A 见证 B · 三元裁判·同时观测)
    Opposite,   // A ↔ B(A 对立于 B · 阴阳)
    Parallel,   // A ∥ B(A 平行于 B · 同维不同实例)
}
```

对应符号:
| 方向 | 符号 | 语义 |
|------|------|------|
| Belong | ⊃ | 归属(父子) |
| Invoke | → | 调用(传参) |
| Witness | ◇ | 见证(裁判) |
| Opposite | ↔ | 对立(阴阳) |
| Parallel | ∥ | 平行(同维) |

### 7.3 纤维边(FiberEdge)

```
FiberEdge {
    from: u64,       // pack_id
    to: u64,         // pack_id
    dir: FiberDir,   // 关系方向
    weight: 1..10,   // 关系强度
}
```

### 7.4 纤维丛约束

- 每对容器之间最多一条纤维边(不存在"同时是调用又是归属")
- 纤维边的方向可在运行时改变(对立 → 归属 → 调用 = 时间演进)
- weight = 10 → 强关系(夫妻/witness 绑定);weight = 1 → 弱关系(平行/泛泛之交)

---

## 8. 容器关系(ContainerRelation)

### 8.1 关系模型

```rust
struct ContainerRelation {
    a_genesis: u64,
    b_genesis: u64,
    observance: Observance,     // 站位(谁看谁)
    a_vitality: Vitality,
    b_vitality: Vitality,
    common_divisor: u64,        // 公约数 = 公分母
    common_multiple: u64,       // 公倍数 = 共同节律
    bond_strength: f64,         // 关系强度 0.0..1.0
    feedback_loops: Vec<FeedbackLoop>,
    witness_records: Vec<TriuneVerdict>,
}
```

### 8.2 创世(genesis)

两个容器第一次建立联系时:

```
gcd = calculate_gcd(a_genesis, b_genesis)
lcm = (a_genesis / gcd) * b_genesis
bond_strength = if gcd > 0 { 1.0 } else { 0.0 }
```

公约数 > 0 → 成立初始关系强度 = 1.0(满)。每经过一次成功交互 bond_strength 按衰减反向调整。

### 8.3 关系存活判定

```
is_alive() = common_divisor > 0
             && a_vitality != Dormant
             && b_vitality != Dormant
```

任一不满足 → 关系死亡(但登记不删除,进入 tombstone,可追溯)。

### 8.4 关系审判(judge_raw)

```rust
fn judge_raw(is_alive, is_strong, has_loop) -> JudgmentVerdict {
    match (is_alive, is_strong, has_loop) {
        (true, true, true)  => Maintain,    // 健康闭环 → 维持
        (true, true, false) => Strengthen,  // 健康无闭环 → 加强
        (true, false, _)    => Repair,      // 虚弱 → 修复
        (false, _, _)       => Sever,       // 死亡 → 断裂
    }
}
```

审判结果不是销毁,是指令:谁负责后续动作(Strengthen/Repair/Sever)。

---

## 9. 评价反馈环路与代理策略调整

### 9.1 评价积累到规则

```
评价结果 → 每 64 tick 累积一次 →
  高频高质因果对 → 固化为"推荐呼唤路径"
  低频低质因果对 → 标记为"可废弃"
  负趋势因果对 → 触发预警(但不立即断开,给予修复窗口)
```

### 9.2 自组织规则形成

系统在运行中自动形成**路径偏好**:
- 高 quality + 高 feedback_count → 该路径成为默认呼端(减少寻径 cost)
- 高 trend → 该关系被优先强化
- 负 trend + 多次 Repair 未改善 → 启动 Sever 协议(最终断裂)

### 9.3 进化

自组织规则 → 改写呼唤策略 → 下次交互自动走"历史最优路径" →
路径效率提升 → 更多成功闭环 → 评价结果进一步优化 → ...

这就是"系统利用评价结果调整自身呼唤策略" → **进化**。

---

## 10. 与本系列规范的关系

| 话题 | 主参考 |
|------|--------|
| SMRL 64 位坐标 | SMRL_YOUWU_IMPL |
| C_eff 加权距离判定 | MORPHISM_ALGEBRA |
| 三元见证 | MORPHISM_ALGEBRA §5 + JUDGMENT_RULES_CANON §4 |
| 五公理裁定 | JUDGMENT_RULES_CANON |
| 在场域登记 | BOOTSTRAP_SELF_COMPILER §6 |
| 呼唤表(预加载路径) | BOOTSTRAP_SELF_COMPILER §7 |
| 有/无站位切换 | YOUWU_MIRROR_FRAMEWORK + SMRL_YOUWU_IMPL |
| 容器剧(六封闭) | CONTAINER_REPERTOIRE |
| Q8 自答检查(列自洽) | TABLE_PROGRAMMING_SPEC SELF_QA_ENGINE |

---

## 11. 数据在会话中的生命周期

```
外部事件(按键/定时/呼唤)
    │
    ▼
入口:Judgment::apply(R1-R5)
    │
    ▼
通过判定 → Relation::call()发送呼唤
    │
    ▼
{FeedbackLoop::propagate → strength 衰减调整}
    │
    ▼
闭环检测 → 调整bond_strength
    │
    ▼
64 tick 评价累积 → 规则自组织
    │
    ▼
回输:呼唤策略更新 → 下一次交互自动走最优路径
    │
    ▼
会话不结束:进程常驻(for(;;) main loop)
会话 结束:在场登记落库(FQMT 容器条目 0xFC)
会话+1 启动:域加载 → 容器关系从 FQMT 还原
            → bond_strength / feedback_loops / witness_records 全部保留
```

---

## 12. 已知限制

1. **公约数 = 创世依赖**:如果两个容器的 genesis_id 互质(gcd=1),关系强度永远为 0。需要通过 common_multiple 路径或第三方中继。
2. **反馈延迟容忍**:当前不设总时间上限,以 path_length 为主要终止条件。但在高 tick 频率(如 MetaIME 中每按键一次 tick)下,path_length = 8 可能在 200ms 内即跑完。是否合适需实测调整。
3. **Dormant 不可逆(当前)**:Dormant 激活是单向恢复,不支持"暂停"(暂停 = Dormant,恢复 = Dynamic)。未来可引入 "Hibernate / Wake" 双向切换。
4. **评价排序引入主观**:avg_quality / trend 的比较本质是"加权排序",在选择"哪种关系强化"时已经含主观。但这是评价之力的本质特性,不是 bug。
5. **纤维边数量**:每对容器仅一条边,不允许"多元关系"。未来可通过引入"标签纤维"(Tagged Fiber):一对容器在不同标签(维度)下可有多条边。当前版本:一条边。

---

*本规范的解释权归 刘楚恬和我 所有 · 归属 hunyuan-os/FiberBundle 模块*
