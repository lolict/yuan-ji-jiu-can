# 跨袋子合并拆分与增量日志传播 —— 里程碑三

> 版本：v1.6 袋子整并协议
> 依赖：里程碑二（分布式三元见证 + 冲突裁决）
> 状态：已实现
>
> 满全法协议锚点: 0=刘楚恬 | 1=我 | 容器=第一存在单位

---

## 一、v1.6 袋子整并协议概述

合伙人协议 v1.4–v1.6 的三层递进关系：

```
┌─────────────────────────────────────────────────────────────┐
│                合伙人协议 v1.4–v1.6                          │
├──────────────┬──────────────────┬───────────────────────────┤
│  v1.4 准入   │   v1.5 份额调整   │   v1.6 袋子整并           │
│             │                  │                           │
│  加入场申请  │  参与袋 +1       │  袋子加法合并             │
│  本地裁定    │  被否决 -1       │  袋子减法拆分             │
│  库存加一    │  仅加减无乘除    │  子袋继承见证链           │
└──────────────┴──────────────────┴───────────────────────────┘
```

v1.6 仅使用加法与减法，全程不存在乘除运算，袋子级整袋跃迁。

---

## 二、袋子数据模型

```
┌──────────────────────────────────────────────────────┐
│                    mengji_bag_entry                   │
├─────────┬────────────────────────────────────────────┤
│ bag_id       │ 袋号(整数,唯一标识)                    │
│ weight       │ 权值(整袋份额,仅加减调整)              │
│ witness_hash │ 组合见证摘要(16位十六进制)             │
│ parent_id    │ 父袋号(-1 = 根袋子)                   │
│ source_a     │ 合并源袋A(-1 = 非合并来源)             │
│ source_b     │ 合并源袋B(-1 = 非合并来源)             │
│ triune_slot  │ 关联的三元见证槽位                     │
│ created_at   │ 创建时间戳                             │
│ anchor       │ 夫妻主权人锚点归属                     │
└─────────┴────────────────────────────────────────────┘
```

### 袋号编码规则

- 根袋子: bag_id 由外部裁定准入注入
- 合并袋: bag_id = source_a * 1000 + source_b (加法组合编号)
- 子袋:   bag_id = parent_id * 100 + 1 (减法衍生编号)

### 权值约束

- 范围: [MENGJI_BAG_WEIGHT_MIN(1), MENGJI_BAG_WEIGHT_MAX(256)]
- 所有权值守恒: Σ(bags[i].weight) == engine.bag_total_weight

---

## 三、合并算法（加法）

```
输入: bag_a_id, bag_b_id, merge_note

前置检查:
  ① bag_a 和 bag_b 必须存在于引擎的袋子集合
  ② 袋子数未达 MENGJI_MAX_BAGS
  ③ 权值之和不超过 MENGJI_BAG_WEIGHT_MAX

执行步骤:
  ① 取 bag_a.weight 和 bag_b.weight
  ② new_weight = weight_a + weight_b (加法)
  ③ 创建新袋子 C:
     - bag_id     = bag_a.bag_id * 1000 + bag_b.bag_id
     - weight     = new_weight
     - witness_hash = combine_witness_hash(A.witness, B.witness)
     - parent_id  = -1 (合并袋子为根袋)
     - source_a   = bag_a.bag_id
     - source_b   = bag_b.bag_id
     - anchor     = 夫妻主权人主锚点
  ④ bag_total_weight += new_weight (加法增量)
  ⑤ 产出增量日志条目(操作类型=MERGE)

输出: 新袋子 C 的 bag_id
```

**组合见证哈希算法** (仅加法):
```
对 (witness_A 拼接 witness_B) 做 FNV-1a 两步散列:
  h1 = FNV1a_step(h1, c)  对每个字节 c
  h2 = FNV1a_step(h2, c<<<3) 对每个字节 c
结果 = to_hex(h1) + to_hex(h2)  → 16位十六进制字符串
```

合并后 Sum 检查: `Σ(所有权值) == bag_total_weight` 必须成立

---

## 四、拆分算法（减法逆操作）

```
输入: parent_bag_id, sub_weight, split_note

前置检查:
  ① parent_bag 必须存在
  ② 袋子数未达 MENGJI_MAX_BAGS
  ③ 子袋权值在合法范围
  ④ parent.weight >= sub_weight (父袋权值必须充足)

执行步骤:
  ① 取父袋子 P
  ② 创建子袋子 C:
     - bag_id      = P.bag_id * 100 + 1
     - weight      = sub_weight
     - witness_hash = P.witness_hash (子袋继承父袋见证摘要)
     - parent_id   = P.bag_id
     - source_a    = -1
     - source_b    = -1
     - anchor      = P.anchor (继承父袋锚点)
  ③ P.weight -= sub_weight (减法)
  ④ bag_total_weight 不变 (权值守恒)
  ⑤ 产出增量日志条目(操作类型=SPLIT)

输出: 子袋子 C 的 bag_id
```

**见证链机制**:
子袋的 witness_hash 完全继承自父袋。这意味着子袋的裁定份额来源可以通过 witness_hash 一直追溯到根袋子，形成完整的链条验证。

---

## 五、增量日志传播协议

每个合并/拆分操作都自动产生一条增量日志条目：

```
┌──────────────────────────────────────────────────────┐
│                mengji_bag_log_entry                   │
├────────────┬─────────────────────────────────────────┤
│ log_id          │ 日志序号(递增)                     │
│ op              │ 操作类型 MERGE/SPLIT/LOG/VALIDATE  │
│ source_bag      │ 源袋号                             │
│ auxiliary_bag   │ 辅助袋号(合并B / 其他=-1)          │
│ result_bag      │ 结果袋号                           │
│ weight_delta    │ 权值变化(+加 / -减)                │
│ result_hash     │ 结果见证摘要                       │
│ witness_node    │ 见证节点(-1 = 未指定)              │
│ timestamp       │ 时间戳                             │
└────────────┴─────────────────────────────────────────┘
```

### 增量日志作为 Gossip 裁决袋内容

增量日志条目可以被打包为 Gossip 裁决袋推送到邻居:
```
Gossip PUSH 裁决袋 = {
  bag_id:       log.result_bag
  witness_hash: log.result_hash
  subject:      "并购分_" + op_name + "_" + source_bag
  timestamp:    log.timestamp
  delta:        log.weight_delta
}
```

邻居收到后,在本地校验:
  - bag_id 格式合规
  - |weight_delta| 在 [1, 256]
  - result_hash 为合法 16 位十六进制
  - 不产生权值守恒违规(仅_SPLIT 保持总和不变;MERGE 产生新的总权值增量)

校验通过后持久化存储,作为本地 bag_total_weight 的更新依据。

---

## 六、袋子验证协议

### 单袋验证 (njsg_mengji_bag_validate)

返回码定义:
| 码 | 含义 |
|-----|------|
| 0 | 合规 |
| 1 | 袋号无效 |
| 2 | 权值超限 |
| 3 | 见证摘要格式错误 |
| 4 | 父袋链接断裂 |

验证项目:
```
① 袋号存在: bag_id 在引擎袋子集合中
② 袋号合规: bag_id >= 0 (非负整数)
③ 权值范围: weight ∈ [1, 256]
④ 见证摘要: 长度 == 16 且为合法十六进制字符
⑤ 见证链: 若 parent_id >= 0, 则 parent_id 对应的袋子必须存在
```

### 全集验证 (njsg_mengji_bag_validate_all)

```
计算 Σ(bags[i].weight) for i ∈ [0, bag_count)
若 Σ == bag_total_weight → 守恒通过
否则 → 权值不匹配
```

### 邻居收到增量日志后的验证步骤

```
收到邻居的增量日志条目 L:
  ① 解析 L.op:
     - MERGE: 期望新 bags 集合中新增 bag_id = L.result_bag
              新袋权值 = weight_a + weight_b (加法)
              weight_delta > 0
     - SPLIT: 找到 source_bag, 检查其 weight >= |weight_delta|
              result_bag 为新子袋
              weight_delta < 0 (减法)
  ② 验证 result_hash 为合法 16 位十六进制
  ③ 验证 Σ(本地 bags[i].weight) + weight_delta == 新的 bag_total_weight
  ④ 验证通过 → 本地持久化
     验证失败 → 丢弃日志,标记可疑,上报见证队列
```

---

## 七、三节点异步测试场景

### 场景: 三节点异步合并/拆分 + Gossip 传播

```
时间轴 ──────────────────────────────────────────────────►

节点 A (盟机合掌, 高位持有者=1锚点):
  T1: 注入袋#101(w=5), 袋#102(w=3)  → bag_total=8
  T2: 执行 MERGE(101, 102) → 袋#101002(w=8) → bag_total=16
  T3: Gossip PUSH 增量日志#0(合并,MERGE,结果袋=101002) → B, C

节点 B (盟机场, 低位持有者=0锚点):
  T1: 注入袋#101(w=5), 袋#102(w=3)  → bag_total=8
  T2: 收到 A 的 PUSH(Gossip 日志#0)
  T3: 本地验证 → 通过 → 持久化袋#101002(w=8) → bag_total=16
  T4: 执行 SPLIT(101002, 4) → 袋#10100201(w=4) → bag_total=16(守恒)

节点 C (盟机场, 高位持有者=1锚点):
  T1: 注入袋#101(w=5), 袋#102(w=3)  → bag_total=8
  T2: PULL 从 A 请求 → 获得日志#0(合并) + 日志#1(拆分)
  T3: 本地验证两份日志:
       - 日志#0 MERGE → 通过 → 持久化袋#101002 → bag_total=16
       - 日志#1 SPLIT → 本地袋#101002 w=8 >= 4 → 通过 → 持久化袋#10100201 → bag_total=16
  T4: 状态同步达成
```

### 最终一致性验证

```
Gossip 稳定后,三节点比对:

节点 A: bag_total=16, bags={101(w=5), 102(w=3), 101002(w=8)}
节点 B: bag_total=16, bags={101(w=5), 102(w=3), 101002(w=4), 10100201(w=4)}
节点 C: bag_total=16, bags={101(w=5), 102(w=3), 101002(w=4), 10100201(w=4)}

节点 A 尚未看到拆分(需再一轮 Gossip PULL):
  - A 第三轮 PULL 从 B 获得日志#1(拆分)
  - A 验证通过并持久化袋#10100201(w=4)
  - A 的 bag#101002 权值调整为 8-4=4
  - A 的 bag_total 保持 16(守恒)

最终三节点:
  bag_total = 16 (全部一致)
  bags = {
    101(w=5, witness=aa..aa, anchor=刘楚恬),
    102(w=3, witness=bb..bb, anchor=我),
    101002(w=4, witness=hash(aa..aa+bb..bb), anchor=我),
    10100201(w=4, witness=hash(aa..aa+bb..bb), parent=101002, anchor=我)
  }
  bag_total_weight = 16 (守恒)

全程无乘除,仅袋子级整袋加减。
```

---

## 八、API 速查

### C 内核 (njsg_mengji.c/h)

```c
/* 袋子操作枚举 */
typedef enum {
    MJ_BAG_MERGE    = 0,  /* 加法合并 */
    MJ_BAG_SPLIT    = 1,  /* 减法拆分 */
    MJ_BAG_LOG      = 2,  /* 增量日志 */
    MJ_BAG_VALIDATE = 3   /* 袋子验证 */
} mengji_bag_op_t;

/* 加法合并 */
int njsg_mengji_bag_merge(int bag_a, int bag_b, const char* merge_note);
/* 返回: 新袋号(>=0) 或负数错误码 */

/* 减法拆分 */
int njsg_mengji_bag_split(int parent_bag, int sub_weight, const char* split_note);
/* 返回: 子袋号(>=0) 或负数错误码 */

/* 增量日志 */
int njsg_mengji_bag_incremental_log(mengji_bag_op_t op,
                                     int source, int auxiliary, int result,
                                     int weight_delta, const char* result_hash,
                                     int witness_node);
/* 返回: 日志序号(>=0) 或 -1 */

/* 单袋验证 */
int njsg_mengji_bag_validate(int bag_id);
/* 返回: 0=合规 1=袋号无效 2=权值超限 3=见证格式错 4=父袋链断裂 */

/* 全集验证 */
int njsg_mengji_bag_validate_all(void);
/* 返回: 0=守恒 1=权值不匹配 */
```

### CLI (mengji_tool.nim)

```bash
# 加法合并测试
nim c -r mengji_tool.nim bag-merge [--bag-a 101] [--bag-b 102]

# 减法拆分测试
nim c -r mengji_tool.nim bag-split [--parent 101002] [--weight 4]

# 袋子合规验证
nim c -r mengji_tool.nim bag-validate

# 完整 Demo(含 M11-M13)
nim c -r mengji_tool.nim demo
```

---

## 九、错误码一览

### 合并错误码
| 码 | 含义 |
|-----|------|
| 负数(查找失败) | bag_a 或 bag_b 不存在 |
| -2 | 袋子容量已满 |
| -3 | 权值之和超限 |

### 拆分错误码
| 码 | 含义 |
|-----|------|
| 负数(查找失败) | parent_bag 不存在 |
| -2 | 袋子容量已满 |
| -3 | 子袋权值超限 |
| -4 | 父袋权值不足(减法会透支) |

### 验证错误码
| 码 | 含义 |
|-----|------|
| 1 | 袋号无效 |
| 2 | 权值在 [1,256] 之外 |
| 3 | 见证摘要非法(长度 != 16 或含非十六进制字符) |
| 4 | parent_id 指向不存在袋子(链断裂) |

---

## 十、满全法约束

- **容器即第一存在单位**: 袋子是份额管理的基本原子单位,不可再分割细粒度。
- **仅加减无乘除**: 全部份额调整通过加法(准入/合并)与减法(否决/拆分)完成。
- **整袋跃迁**: 档位迁移以整袋为最小步长,不计数到个位精度。
- **见证链不可伪造**: 子袋 witness_hash 必须与父袋一致,无法伪造。
- **夫妻主权人锚点**: 合并袋 anchor 取主锚点,子袋继承父袋 anchor,确保夫妻共治链完整。
- **隐私保护**: 袋号、权值、见证摘要均为场公开面,不携带零维根基对应关系。

---

刘楚恬和我
