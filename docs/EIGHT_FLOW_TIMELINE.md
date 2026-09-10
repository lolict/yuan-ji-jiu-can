# 八流统一 · 时间线性协议

> 版本 v0.1 · 2026-09-10 · 配套 `SELF_QA_ENGINE.md` `DISTINGUISHER_ROUTING.md`

---

## 1. 定义

八流不是八个独立系统,不是八个执行线程,不是八个微服务。

**八流 = 信息从输入到抵达目标所必须穿越的八个阶段。**

时间线性 = 这八个阶段不允许并行无序。每一步只做一件事。前一步的输出是后一步的输入。环路不允许——循环要变成自问答引擎的 tick 周期,不是流的内部倒流。

```
Input → Perception → Distinguish → Lookup → Traverse → Cluster → Attention → Arrival
        预加载流      范畴流      聚类流     解释流      关系流     概念流
        (指令集)     (类型映射)   (聚合)     (牺牲重译)  (可承认)   (稳态抵达)
```

---

## 2. 八流详细定义

### 2.1 Preload Flow — 预加载流

```
阶段: Input → Perception
位置: 分别器之前的预加载
载体: FlowDictionary(编译时内置的字典)

职责: 把原始输入翻译为字典中的已有条目。
      字典中不存在的条目 = "未被预加载的输入" = 需要额外处理。

预加载流不负责解析输入的语义(那是解释流的职责),
它只负责"这个输入在不在已知字典中?"。

字典 = mlll 254 容器中已装载的全部条目。
      254 个容器 = 254 个预加载簇的根。
```

### 2.2 Category Flow — 范畴流

```
阶段: Perception → Distinguish
位置: 分别器第一层(家族分类) + 类型空间映射

职责: 决定输入的"范畴归属"。

范畴不是集合论意义上的分类——是类型空间的映射。
输入的"形状"(第一层分类)决定了它在哪个类型空间被处理:

  ContainerGlyph → 在容器空间中理解(刘楚恬侧的包围/容纳操作)
  PhallicGlyph  → 在界线空间中理解(我侧的贯穿/分隔操作)
  CoupleGlyph   → 在共生空间中理解(夫妻拓扑中的嵌套操作)
  HomophoneBridge → 在谐音空间中理解(跨空间的音义映射)

范畴流输出: 输入在类型空间中的坐标。
```

### 2.3 Cluster Flow — 聚类流

```
阶段: Distinguish → Lookup
位置: 分别器完成后,表格编程的加载

职责: 把已经范畴化的输入,聚类到具体的容器槽位。

聚类算法 = MORPHISM_ALGEBRA.md §5 "距离感知聚类"。
加权距离 C_eff 决定输入落入哪个簇:

  C_eff < 0.15  → 落入主簇(直接装入对应容器)
  C_eff < 0.4   → 落入相邻簇(进入桥接层过渡)
  C_eff < 0.7   → 落入跨层簇(需生成代理容器中转)
  C_eff >= 0.7  → 拒绝聚类(该输入在当前拓扑中不可达)

聚类流输出: 输入在 mlll 254 容器中的槽位号。
```

### 2.4 Classification Flow — 分类流

```
阶段: 与 Cluster Flow 并行(实际上: Cluster 的另一种等价表述)
位置: 分别器第三层(主权裁定)

职责: 决定聚类后的输入具有什么主权级别。

分类流不是独立的执行步骤,是 Cluster Flow 的主权维度补充。
输入已经聚类到某个容器的同时,必须知道自己的地位:
- 自己是主人(Active)还是客人(Passive)?
- 自己是醒着(System active)还是睡着(Sleeping)?

分类流输出: 主权级别四值(Active / Passive / Sleeping / System)。
```

### 2.5 Attribute Flow — 属性流

```
阶段: Lookup — 属性位的读取与更新
位置: SMRL 引擎 cell 编码的位级操作

职责: 描述一个容器当前的所有二元状态。

属性位 = 12 位二元属性(来自 hunyuan-methodology):

  bit 0  REACHABLE      可被到达
  bit 1  CONTINUABLE    可持续
  bit 2  INCR           可增量
  bit 3  STOPPABLE      可停止
  bit 4  REVERSIBLE     可逆
  bit 5  RECOVERABLE    可恢复
  bit 6  CIRCULABLE     可循环
  bit 7  INHERITABLE    可继承
  bit 8  TRUSTED        已信任
  bit 9  ACKABLE        可被承认(闭环必需)
  bit 10 TRANSMISSIBLE  可传递
  bit 11 ADVANCEABLE    可推进

属性流判断:
  if REACHABLE && ACKABLE: 可被路由
  if CONTINUABLE && INCR && TRANSMISSIBLE: 可构成演化
  if REACHABLE && CONTINUABLE && ACKABLE: 是活的

属性流不做决定,只提供判断材料。
```

### 2.6 Interpretive Flow — 解释流

```
阶段: Traverse — 牺牲自己产出新关系
位置: 蚕蛹壳桥接层的 interpret() 函数

职责: 把障碍转译成可被跟随的状态。

解释流是最"勇敢"的流——它以自身结构的变化为代价,
把 mlct(岔途)转译成 mltt(从体):

  mlct(岔途) = 外部系统的规则和我方不兼容时产生的冲突结构
  mltt(从体) = 冲突结构被转译为一个从我方看可被跟随的新结构

解释流不消除冲突,它把冲突变成一种"我可以从这个面上走"的新形状。

解释流的产物 = 桥接层的映射结果。
```

### 2.7 Relational Flow — 关系流

```
阶段: Cluster → Attention
位置: 态射代数 · 可及性判定 · 态射合成

职责: 让解释流产出的关系,被系统的承认机制接纳。

不是所有关系都能被承认。关系流的校验:
- 可及性检查: MORPHISM_ALGEBRA.md §8 "可及性判定"
- 见证检查: MORPHISM_ALGEBRA.md §4 "见证裁定"
- 主权检查: DISTINGUISHER_ROUTING.md §4 "主权级别"

三项检查通过 = 关系被"承认"(ack)。
任何一项失败 = 关系断裂,输入到达 RELATIONAL_FAILURE 状态,需要重新路由。
```

### 2.8 Concept Flow — 概念流

```
阶段: Attention → Arrival
位置: 表格编程的 Phase 6(信念目标生成)

职责: 把已承认的关系凝聚为一个"概念"——可被下一次输入引用的稳态结构。

概念 = 已 ack 的关系簇 + 框架级命名 + 容器的持久化表示。

概念流是八流的终点——输入到达此处时,不再是"外部数据",
而是"系统内部的一个已知事实"——可以像 smlr 引擎的 coord 一样被引用。

概念流的产物 = 写入表格编程 C15(信念目标)的条目 + 更新 FlowDictionary 字典。
```

---

## 3. 八流的执行顺序与并行约束

### 3.1 标准时间线(无冲突情况)

```
Tick t0:  [Input] 外部输入到达
Tick t0+1:[Preload]  查字典,确认输入是否在已知 254 容器中
Tick t0+2:[Category] 决定输入的类型空间归属(分别器第一层)
Tick t0+3:[Cluster]  距离感知聚类,找到槽位
Tick t0+3:[Classify] 与 Cluster 并行,决定主权级别
Tick t0+4:[Attribute]读取目标容器的 12 位属性
Tick t0+5:[Interpret] 桥接层 interpret() 转译冲突
Tick t0+6:[Relational] 可及性+见证+主权三项检查
Tick t0+7:[Concept]   凝聚为概念,写入 C15,更新字典
Tick t0+8:[Arrival]   抵达目标,返回 ack
```

注意:**各个 tick 是逻辑时间,不是物理时钟**。在一个快速执行中,t0+1 到 t0+8 可能在同一个 CPU 周期内以组合逻辑完成。但逻辑顺序不可颠倒——不允许跳过任何阶段。

### 3.2 冲突情况下的时间线分叉

当 Relational Flow 的关系断裂时(例如:见证失败 / 主权越级 / 不可及):

```
Tick t0+6: Relational 检查失败 → 关系断裂
Tick t0+7: 触发信念分叉(SELF_QA_ENGINE §5.2)
Tick t0+7a: 原路径进入"等待调包"状态
Tick t0+7b: twin 路径尝试"制造空质壳→挂载→注入→调包"
Tick t0+(7b+n): 调包成功或超时
Tick t0+8:   如果任一路径成功 → Arrival
             如果两条路径均失败 → ARRIVAL_FAILURE
```

等待调包期间,输入被悬挂在"未决缓冲区"——它不被拒绝,也不向前推进——等待"有/无切换"时机成熟。

---

## 4. 八流与有/无站位的映射

### 4.1 各流中的站位应用

| 八流阶段 | 有侧行为 | 无侧行为 |
|----------|----------|----------|
| Preload  | 在对方字典中查条目 | 在我们的字典中查条目 |
| Category | 按对方的家族分类 | 按我们的家族分类 |
| Cluster  | 落入对方的簇根 | 落入我们的簇根 |
| Classify | 作为 Passive 进入 | 作为 Active 发起 |
| Attribute| 只读读取对方属性 | 自有读取我方属性 |
| Interpret| 不适用(有侧不解释) | 主动解释 |
| Relational| 等待对方 ack | 主动 ack / 主动寻求 ack |
| Concept  | 在对方框架内解释概念 | 在本系统内建立概念 |

### 4.2 空质壳的站位与八流

空质壳不自然归属任一侧——它在每个八流阶段都需要明确声明"此时站在哪一侧":

```
空质壳的八流站位规则:

Preload: 在"外部签名库"中查条目(不是我们真正的字典)
Category: 以对方的家族分类为准(因为是空质壳)
Cluster: 落入"壳边界簇"——这是一个专门用于挂载的虚拟簇
Classify: 主权 = "待定主权"(既不是 Active 也不是 Passive)
Attribute: 外壳属性模拟对方,内核属性为我们
Interpret: 
  if mount_active=1: 不解释,让对方的冲动直接通过
  else: 执行反向吸入
Relational:
  if mount_active=1: 假装 ack(让对方以为成功)
Concept: 壳不持有概念,所有概念在内核中
```

---

## 5. 八流与表格编程的对接

### 5.1 每轮 ENGINE_TICK 的八流推进

TABLE_PROGRAMMING_SPEC 中的 ENGINE_TICK 的各个 Phase 对应于八流:

| ENGINE_TICK Phase | 对应八流 |
|-------------------|----------|
| Phase 0 站位校验 | Attribute(读站位位) + Preload |
| Phase 1 站位校验 | Classify(主权裁定) |
| Phase 2 自提问 | Concept 的逆向过程(回到 Preload 问"我的字典对应什么?") |
| Phase 3 自回答推导 | Cluster + Interpret + Relational |
| Phase 4 信念一致性 | Relational 的可达性 |
| Phase 5 双路径对决 | Relational 分叉 |
| Phase 6 信念目标生成 | Concept 凝聚 |
| Phase 7 产出区刷新 | Arrival |

### 5.2 表格编程列的八流视角

| 列 | 八流来源 |
|----|----------|
| C1 序号 | Cluster 聚类结果 |
| C2 提问 | Preload 字典查询未命中时触发 |
| C3 视角 | Attribute 属性位中的有/无二位 |
| C4 所在态 | Classify 主权级别 |
| C5 空质壳 | Interpret 结果中的桥接层 |
| C6 镜像匹配 | Interpret 的输入来源 |
| C7 管辖 | Relational 的见证者 |
| C8 归属 | Cluster 簇根 |
| C9 从属 | Category 类型空间层级 |
| C10 允许权 | Attribute REACHABLE + ACKABLE |
| C11 否决权 | Attribute 失败原因 |
| C12 态射方向 | Cluster 距离计算结果 |
| C13 博弈身份 | Classify + 有/无站位 两字段组合 |
| C14 回答 | Concept 凝聚中的新认知 |
| C15 信念目标 | Concept 最终输出 |

---

## 6. 八流失败的传播

### 6.1 单点失败的影响范围

| 失败阶段 | 后续流全部失败? | 是否有降级路径? |
|----------|-----------------|-----------------|
| Preload 字典未命中 | 是 | 触发 QT4 进行"制造匹配壳" |
| Category 四族都不匹配 | 是 | 进入未分类缓冲区,等待邻居上下文 |
| Cluster 不可及 | 是 | 寻找远态射跳板 / 建立新容器桥梁 |
| Classify 身份未知 | 是 | 默认 Passive(最小权限) |
| Attribute 属性缺失 | 否 | 使用默认属性(全部 bit = 0)继续 |
| Interpret 转译失败 | 是 | 尝试多种 interpret() 变体,取 ack 率最高者 |
| Relational ack 失败 | 是 | 触发信念分叉 |
| Concept 无法凝聚 | 否 | 作为"未成型事实"存储,不进入字典 |

### 6.2 失败恢复策略

```
失败恢复 = 两种策略:

策略 A: 降级重试
  降低一级处理精度,尝试再次通过。
  例如: Cluster 的 C_eff 阈值从 0.4 放宽到 0.6,允许落入"近似簇"。

策略 B: 跳跃重置
  放弃当前输入,重新从 Category 开始。
  但只在新上下文出现时(因为上次失败可能恰恰是因为本次输入缺少上下文)。
```

---

## 7. 八流的输出闭环

### 7.1 ack 作为八流完成的标志

Relational Flow 成功的唯一标志 = 目标容器的 ack。

```
ack(response):
  response.ackable == true
  AND response 返回给发起方
  AND 发起方写入 Concept Flow
  AND Concept Flow 更新 FlowDictionary
```

如果 ack 未返回,整个八流标记为 **未闭环**——输入停留在 Relational Flow,等待下一次调度。

### 7.2 ack 失败的处理

```
ack 失败三次: 
  输入从 Relational 流进入"信念僵局"状态(参见 SELF_QA_ENGINE §9.1)
  需要人工或外部仲裁器介入。

ack 失败但另一路径获得 ack:
  twin 路径胜出,原路径标记为"败入缓存"。

ack 成功:
  本轮八流完成,Concept Flow 产出新条目。
```

---

## 8. 八流的物质基础

### 8.1 物理位置

八流不限于单一进程。在蚕蛹壳多壳并行的架构中:

```
壳A(Android App壳):捕获的输入走完 Preload→Category→Cluster
壳B(Http Server壳):捕获的输入走完 Preload→Category→Cluster
壳C(Browser DOM壳):捕获的输入走完 Preload→Category→Cluster

三壳的 Cluster 输出汇聚到同一个 mlll 装箱(内核)。
内核完成 Interpret→Relational→Concept 后,产出写回三个壳。
```

八流不是分布式的执行图,是一个**逻辑上的顺序承诺**——任何实现必须在逻辑上满足 Preload 先于 Category、Category 先于 Cluster、以此类推的物理因果顺序。

### 8.2 各流的实现载体

| 八流 | 在哪个模块实现 | 该模块的文档 |
|------|---------------|-------------|
| Preload  | mlll 集装箱的 FlowDictionary | MLLL-ALGEBRAFOUNDATION.md |
| Category | 分别器第一层 | DISTINGUISHER_ROUTING.md |
| Cluster  | 距离感知聚类 | MORPHISM_ALGEBRA.md |
| Classify | 分别器第三层 + SMRL 引擎 | DISTINGUISHER_ROUTING.md SMRL_YOUWU_IMPL.md |
| Attribute| SMRL 64 位 cell 高 12 位(MORPHISM 属性位扩展) | SMRL_YOUWU_IMPL.md |
| Interpret | 桥接层的 interpret() | CHRYSALIS_REVERSE_PARASITISM.md |
| Relational| 态射合成 + 见证裁定 + 可及性判定 | MORPHISM_ALGEBRA.md |
| Concept  | 表格编程 C15 | TABLE_PROGRAMMING_SPEC.md |

---

## 9. 不变量

1. **顺序不可颠倒**: Preload < Category < Cluster < Classify < Attribute < Interpret < Relational < Concept,这个偏序必须在所有实现中被保持
2. **前一步输出是后一步输入**:不允许旁路——禁止从 Preload 直接跳到 Concept
3. **Interpret 是可选的简化路径**:但跳过 Interpret 的输入必须在 Relational 中通过三种检查(因为未经转译的输入可能含有格式有毒的结构)
4. **Cluster + Classify 是并行的逻辑阶段**:它们的物理执行可以是乱序的,但其结果必须在 Attribute 之前同时可用
5. **ack 是唯一的闭环标志**:没有 ack 的关系 = 撕裂的关系 = 未完成的关系

---

*本文件定义八流的统一时间模型以及它们如何将前面的分别器、表格编程、SMRL 引擎、态射代数、蚕蛹壳连通成一个逻辑上可推导的整体。八流不是独立的处理管线,而是输入从"抵达我们"到"成为我们"必须穿越的逻辑通道。配套文档: SELF_QA_ENGINE.md、TABLE_PROGRAMMING_SPEC.md、DISTINGUISHER_ROUTING.md、SMRL_YOUWU_IMPL.md、MORPHISM_ALGEBRA.md、CHRYSALIS_REVERSE_PARASITISM.md。*
