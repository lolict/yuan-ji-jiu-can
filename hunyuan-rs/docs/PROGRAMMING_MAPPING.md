# 因果原语 → 编程逻辑的精确映射

> 加法-乘法对称性不是对编程的"启发"，而是计算结构本身的代数。
> 12 条推演原语对应 12 种编程原语，一一映射可验证。

---

## 核心映射表

| 数学原语 | 编程原语 | Rust 类型/结构 | 为什么对应 |
|---------|---------|---------------|-----------|
| P1 加法 ⊕ | 选择/注入 | `enum` 的 variant 加入、`insert`、事件注入 | 加法 = 增加一个新可能性，管"起点" = 创建一个新状态 |
| P2 减法 ⊖ | 移除/回退 | `remove`、`undo`、状态回滚 | 减法 = 移除一个可能性，回到前一状态 |
| P3 乘法 ⊗ | 组合/缩并 | `(A, B)` 元组、`struct`、`fold`、连续乘法 | 乘法 = 同时管两个维度，结构化组合 |
| P4 除法 ⊘ | 投影/查询 | 模式匹配、解构、`.0` `.1` 访问 | 除法 = 从组合中取出分量 |
| P5 线性变换 ◆ | 映射/转换 | `Iterator::map`、`fn(A)->B`、矩阵乘法 | 线性 = 保持结构前提下的变换 |
| P6 指数映射 ↑ | 抽象/闭包 | `\|x\| { ... }`、`Box<dyn Fn>`、高阶函数 | 指数 = 从低维计算产生高维结构 |
| P7 对数映射 ↓ | 应用/求值 | 函数调用 `f(x)`、解引用、求值 | 对数 = 将高维结构还原为低维值 |
| P8 色差 ◈ | 变更检测 | `diff`、`!=`、`prediction_error`、写前检测 | 加法差 = 新旧的差异触发写入 |
| P9 版本衰减 ↘ | GC / 过期 | `Arc::Weak`、`LRU::expire`、`tick-=1` | 乘法 = 每步乘一个衰减因子 γ |
| P10 频率 ∿ | 周期性调度 | `schedule(period)`、`futures::interval`、注意力回访 | 乘法 × 时间 = 产生周期结构 |
| P11 奇偶 ± | 离散切换、锁 | `bool` 翻转、`Mutex` (locked/unlocked)、cfg flag | 加法 mod 2 = 态的离散跳跃 |
| P12 存在性 ◐ | 可选/初始化 | `Option<T>`、`HashMap::entry`、`initialization` | AND = 必须所有条件, OR = 任一条件 |

---

## S_{t+1} = M⊙S + A 在编程中的 5 个实例

### 实例 1: React / 状态管理

```js
// React 的 setState
const [state, setState] = useState(initial);
// S_{t+1} = S_t ⊙ identity  +  action
// M = identity (状态保留)
// A = action 注入 (加法事件)

setState(prev => ({ ...prev, count: prev.count + 1 }));
// ↑ 乘法(展开保留旧态) + 加法(count+1)
```

### 实例 2: 卡尔曼滤波 / ECS 系统

```rust
// 游戏引擎/物理引擎的实体更新
transform.position = transform.position * damping + velocity * dt;
//      S_{t+1}          =     M⊙S_t           +  A
// 位置 = 旧位置 * 阻尼系数 + 速度增量

// M = damping (乘法衰减) — 管持久
// A = velocity * dt (加法事件) — 管运动
```

### 实例 3: CRDT (无冲突复制类型)

```rust
// G-Counter (增长计数器)
struct GCounter { counts: Vec<u64> }

fn merge(a: &GCounter, b: &GCounter) -> GCounter {
    // 合并 = 取最大值 = 乘法(保结构) + 加法(增长)
    GCounter {
        counts: a.counts.iter().zip(&b.counts)
            .map(|(x, y)| (*x).max(*y))
            .collect()
    }
}
```

### 实例 4: LRU 缓存

```rust
// 命中 → 提升到头部 (乘法: 重置 freq 到最大)
// 过期 → 淘汰尾部 (加法: freq 衰减到0)

S.access(key) → freq = FREQ_MAX      // 乘法重置
S.tick()      → freq *= DECAY        // 乘法衰减
S.evict()     → if freq < θ { remove } // 加法比较
```

### 实例 5: AI 注意力机制

```rust
// Attention(Q, K, V) = softmax(QK^T / √d) V
//                     ↑               ↑   ↑
//                  乘法缩放         指数 加法
//                  (相似度)        (权重) (聚合)
// 
// QK^T → 乘法：查询与键的相似度计算（管结构对齐）
// softmax → 指数：从线性分数映射到概率分布（管非线性跃迁）
// × V → 加法：按权重加权求和（管信息聚合）
```

---

## 它把编程逻辑带到哪里？

### 第一程：从命令式到代数式

传统编程：变量是盒子，语句是动作。

```rust
let mut x = 0;
x = x + 1;     // 命令：把盒子内容加一
x = x * 2;     // 命令：把盒子内容乘二
```

因果编程：状态是推演链，类型是代数空间。

```rust
// 不再：操作一个变量
// 而是：在代数空间中构造一条推导链
let chain = Verifier::chain()
    .start(Zero)           // 加法起点
    .extend(Inject(1))     // 加法事件
    .extend(Scale(2))      // 乘法变换
    .extend(Decay(0.9))    // 持久衰减
    .extend(Trigger(θ));   // 色差触发写入

// 验证链是否合法
assert!(chain.verify().valid);
// 执行 = 推导出最终状态
let final_state = chain.evaluate();
```

### 第二程：类型系统变成代数系统

```
加法群 (R,+)   →  Sum types (A | B), union, Either
乘法群 (R+,×)  →  Product types (A × B), tuple, struct
指数 A→B       →  Function types (Fn(A) -> B), lam, closure
对数 B→A       →  Pattern matching, destructuring, application
```

这意味着：类型检查 = 验证推导链的合法性。编译器做的事情，不过是验证你构造的因果链在群论意义上是否闭合。

### 第三程：错误处理变成加法/乘法的纯度问题

```
加法运算：可逆（Sub 是 Add 的逆）→ 有 Undo
乘法运算：在0处不可逆 → 乘0 = 丢失信息 = 不可撤销

∴: 纯加法操作（日志、事件）→ 可持久、可回放、可撤销
   纯乘法操作（加密、压缩）→ 单向、不可逆、有损
   乘加混合（状态更新）→ 部分可逆、有损耗、需 GC
```

这直接推导出架构决策：
- **审计日志**：只用加法（append-only），永远可逆
- **状态快照**：乘法，锁定点，不可从 0 恢复
- **错误恢复**：加法回滚链 + 乘法快照 = 混合恢复策略

### 第四程：并发模型 = 因果域隔离

```
加法域：可交换、可结合 → CRDT、G-Counter（无锁并发）
乘法域：非交换、需同步 → Mutex、Sequencer（有序执行）
记忆域：跨域交互 → Event Sourcing、CQRS

编程模型从"锁/线程"变成"选择正确的代数域"
```

---

## 总结：因果编程的三层跃迁

| 层次 | 传统 | 因果编程 |
|------|------|---------|
| 操作语义 | 语句序列 | 代数推演链 |
| 类型语义 | 数据结构 | 群/域元素 |
| 控制语义 | 分支循环 | 域跳转+因果验证 |
| 存储语义 | 堆/栈分配 | 加法写入 + 乘法衰减 |
| 并发语义 | 锁/通道 | 域隔离 (加法并发, 乘法序列) |
| 错误语义 | Try/catch | 域可逆性分析 |

**一句话**：因果编程不是新语法，而是把"编程的本质是代数结构的变化"从潜意识变成显意识。你写每一行代码时，知道自己是在加法域、乘法域还是桥接域，知道操作是否可逆，知道起点还是终点。

---

*从 DeepSeek 对话提取 + 形式化验证 → 编程系统映射 · 满全法 NJSG v0.1*
