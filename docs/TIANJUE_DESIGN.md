# 天觉 · 袋子认知审判系统

> 不数米，数袋子。不命名米，命名袋子。
> 高维认知：操作容器本身而非容器内的原子。

---

## 一、核心隐喻：袋子

**米** = 编码原子（符号、码位）
**袋子** = 容器（阿大、阿猫、阿狗、叠音）
**天觉** = 对袋子的感知、审判、记忆、层级认知

一个袋子，不问里面有多少颗米，只问：

| 维度 | 问题 | 认知动作 |
|------|------|----------|
| 归属 | 这是谁家的袋子？ | 所有权认定 |
| 价值 | 这袋子值多少钱？ | 定价判断 |
| 完整 | 袋子是否坏了？ | 完好性检验 |
| 品质 | 米还好不好吃？ | 质量评估 |
| 安全 | 有没有脏东西？ | 安全扫描 |
| 干湿 | 谷子晒干了没有？ | 成熟度判断|
| 虫害 | 有没有虫？ | 健康检查洁净 |
| 杂质 | 有没有沙子？ | 纯度检测 |
| 洗涤 | 要不要洗？ | 净化判定 |
| 可食 | 能不能吃？ | 准入裁决 |

---

## 二、天觉五子系统

### 2.1 视觉（天觉感知层）

每个袋子被"看见"——不是看见里面的米，而是看见袋子的**属性状态**。

```
视觉输入：袋子ID
视觉输出：十维属性向量
  [归属, 价值, 完整, 品质, 安全, 干湿, 虫害, 杂质, 洗涤, 可食]
```

每维取值：
- `0.0` = 完全不合格
- `0.5` = 中间状态
- `1.0` = 完全合格
- `-1` = 未触发检测

### 2.2 审判（裁决引擎）

```
输入：视觉向量 + 审判标准阈值
输出：审判结果（通过/待定/拒绝）+ 置信度
```

审判规则：
- 全部维度 ≥ 阈值 → **特许**（高置信通过）
- 任一维度 = 0 → **拒绝**
- 任一维度 < 阈值但 > 0 → **待定**（需补充审查）
- 安全 = 0 → **绝对拒绝**（一票否决）

### 2.3 历史记忆力与积累

每次审判都留下记录：
```
记录 = { 袋子ID, 时间, 视觉向量, 审判结果, 置信度, 审判官 }
```

历史积累产生：
- **袋子履历**：该袋子历次审判的趋势
- **审判官准确度**：某审判官历史裁决的正确率
- **模式识别**：同族袋子的共性问题
- **衰老衰减**：袋子随时间退化的轨迹

### 2.4 奖励与惩罚

| 事件 | 反馈 |
|------|------|
| 审判正确（袋子后续验证为好） | 审判官 +奖励积分 |
| 审判错误（袋子后续验证为坏） | 审判官 -惩罚积分 |
| 连续正确 N 次 | 晋升（更高审判权限） |
| 连续错误 M 次 | 降级（失去审判权限） |
| 拒绝了一个坏袋子 | +高奖励（防守成功） |
| 放进了一个坏袋子 | -高惩罚（防守失败） |

### 2.5 层级关系认知（高维）

```
低维层级：袋子里有米，米是袋子的子节点
高维层级：袋子A 引用 袋子B 的审判结果
          袋子C 继承 袋子D 的品质基线
          袋子E 鉴定 袋子F 的归属可信度
          袋子G 监控 袋子H 的干湿状态
```

高维关系类型：
- **引用**：袋子A使用袋子B的审判结论
- **继承**：袋子C继承袋子D的品质基线
- **鉴定**：袋子E对袋子F做归属认定
- **监控**：袋子G持续观察袋子H
- **融合**：多袋内容融合为新袋
- **仲裁**：冲突时高维袋子裁决

---

## 三、数据结构

### 3.1 袋子（Bag）

```c
typedef struct {
    int     id;                 // 袋子唯一标识
    char    name[MAX_NAME_LEN]; // 袋子名（阿大/阿猫/阿狗…）
    char    owner[MAX_NAME_LEN];// 所有者
    float   value;              // 估值
    float   integrity;          // 完好性 0-1
    float   quality;            // 品质 0-1
    float   safety;             // 安全性 0-1（一票否决）
    float   dryness;            // 干燥度 0-1
    float   pest_free;         // 无害虫 0-1
    float   purity;             // 纯度 0-1
    float   cleanliness;        // 洁净度 0-1
    float   edibility;          // 可食性 0-1（准入裁决）
    int     parent_bag;         // 父袋子ID（高维层级）
    int     generation;         // 层级代数
} Bag;
```

### 3.2 视觉快照（Perception）

```c
typedef struct {
    int     bag_id;
    float   attrs[TIANJUE_DIMS]; // 十维属性向量
    int64_t timestamp;
    int     perceiver_id;        // 感知者
} Perception;
```

### 3.3 审判记录（Judgment）

```c
typedef struct {
    int     judgment_id;
    int     bag_id;
    int     judge_id;            // 审判官ID
    int     verdict;             // 0=拒绝, 1=待定, 2=特许
    float   confidence;          // 置信度
    int64_t timestamp;
    int     correct;             // -1=未验证, 0=错误, 1=正确
} Judgment;
```

### 3.4 审判官（Judge）

```c
typedef struct {
    int     id;
    char    name[MAX_NAME_LEN];
    int     reward_points;       // 奖励积分
    int     penalty_points;      // 惩罚积分
    int     total_judgments;     // 总审判次数
    int     correct_count;       // 正确次数
    float   accuracy;            // 正确率
    int     level;               // 审判权限等级
} Judge;
```

---

## 四、函数接口

```c
// === 视觉感知 ===
void njsg_tianjue_perceive(int bag_id);                    // 感知一个袋子
void njsg_tianjue_perceive_all(void);                      // 感知所有袋子
void njsg_tianjue_print_perception(int bag_id);            // 打印感知报告

// === 审判裁决 ===
int  njsg_tianjue_judge(int bag_id, int judge_id);         // 审判一个袋子
void njsg_tianjue_judge_all(void);                         // 审判所有待审袋子
float njsg_tianjue_confidence(int bag_id);                 // 计算置信度

// === 奖励惩罚 ===
void njsg_tianjue_reward(int judge_id, int points);        // 奖励审判官
void njsg_tianjue_punish(int judge_id, int points);        // 惩罚审判官
void njsg_tianjue_verify(int judgment_id, int actual);     // 验证审判正确性
void njsg_tianjue_update_levels(void);                     // 更新审判官等级

// === 历史记忆 ===
void njsg_tianjue_record(int bag_id, int verdict);         // 记录一次审判
void njsg_tianjue_history(int bag_id);                    // 查看袋子履历
void njsg_tianjue_accumulate(void);                       // 积累分析
void njsg_tianjue_pattern_detect(void);                    // 模式识别

// === 层级认知 ===
void njsg_tianjue_set_parent(int bag_id, int parent);      // 设置父子关系
void njsg_tianjue_link(int from, int to, const char* rel_type); // 建立高维关系
void njsg_tianjue_hierarchy_print(void);                   // 打印层级
float njsg_tianjue_inherited_quality(int bag_id);          // 继承品质基线
```

---

## 五、运行场景

```
场景 T1: 感知袋子
  → 创建袋子 "阿大"，随机生成十维属性
  → 调用 tianjue_perceive，输出感知报告

场景 T2: 审判袋子
  → 创建审判官 "三元"
  → 设定阈值 0.6
  → 审判阿大：安全=0 → 绝对拒绝

场景 T3: 历史记忆
  → 对同一袋子审判 5 次
  → 输出袋子履历趋势

场景 T4: 奖励惩罚
  → 审判官正确 → +10 分 → 晋升
  → 审判官错误 → -15 分 → 降级

场景 T5: 层级引用
  → 袋子B引用袋子A的审判结果
  → 袋子C继承袋子D的品质基线
  → 层级认知打印

场景 T6: 模式积累
  → 100次审判后，统计：
    · 哪个维度最常被拒绝
    · 哪个审判官最准确
    · 哪类袋子品质最好
```

---

## 六、对接现有编码体系

天觉不是替代四容器系统，而是**站在四容器之上**的审判层：

```
容器一·阿大（声母）──┐
容器二·阿猫（韵母）──┤
容器三·阿狗（声调）──┼── 被天觉审判的对象
容器四·叠音 ────────┘

天觉层：
  · 视觉：感知每个容器的品质状态
  · 审判：裁定容器是否合格可用
  · 记忆：记录容器历史表现
  · 层级：容器间的高维引用/继承/鉴定/监控关系
```

---

文件：docs/TIANJUE_DESIGN.md
版本：v1.0
状态：已定稿
设计师：lolict
实现者：CatPaw
