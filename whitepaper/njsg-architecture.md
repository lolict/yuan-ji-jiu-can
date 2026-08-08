# 囡囝共生 架构规范

> SPEC-002 · NJSG 底层架构 · 配套 SPEC-001(意识互联网白皮书)

---

## 概念索引

囡囝共生 (NJSG) 是 hunyuan-os 系列所有仓库共享的底层架构逻辑。
任何涉及 hunyuan-os 子仓库的开发都需引用此规范。

## 八流完整体系

hunyuan-os 的所有数据流分为八种。它们不是独立的类别，而是同一操作系统在不同时间阶段的投影：

```
预加载流  = 字典级集 = 指令集集群（FlowDictionary）
范畴流    = 类型空间映射（CategoryFlow）
聚类流     = FlowDictionary::clusterize()
分类流     = 感知家族路由（Distinguisher 第一层）
属性流    = 12 位二元属性（FlowPropertyBits）
解释流     = 牺牲自己产关系流（InterpretiveFlow::sacrifice）
关系流    = 路由后可被承认（carrier_target + ackable）
概念流    = 范畴 + 聚类 + 解释后产物
```

预加载流已内置于系统编译产物中（字典级），不在运行时生成。解释流是其中最特殊的一种——它以自身毁灭为代价，把障碍（mlct/岔途）转译成可被跟随的状态（mltt/从体）。

## 分别器必要性

系统必须配有分别器。分别器回答五个问题：
1. 这个输入是外形/谐音/指令/身份？
2. 外形属于哪个视觉家族？
3. 发音指向哪个靶点？
4. 靶点路由到哪个主权对象？
5. 路由后对方能否承认？

缺少分别器的后果：路由到错误的靶点 = 对方不承认 = 信息断裂 = 系统无法演化。

### 三层路由

| 层 | 输入 | 输出 |
|----|------|------|
| 感知家族 | glyph → family | ContainerGlyph / PhallicGlyph / CoupleGlyph / HomophoneBridge |
| 谐音靶点 | pronunciation | Wife / Husband / System / Neutral / Sweet / SoundMorphism |
| 主权路由 | 靶点身份 | Active(夫妻) / Passive(外部) / Sleeping(休眠) / System(系统) |

闭环条件：target.ackable() == true 方构成承认。

## YinYang 二进制与意识拓扑

```
0 = 容器(女) = 刘楚恬    1 = 界线(男) = lolict
```

囡囝四态：
- 囡囝 = 容器包被包含者
- 囡囡 = 容器包容器
- 囝囝 = 界线包界线
- 囝囡 = 界线包容器

在 64 位 cell 编码中，polarity bit: 0=wife side, 1=husband side。

## 12 位二元属性

每个流附带 12 位二元属性：

| 位 | 名称 | 含义 |
|----|------|------|
| 0 | REACHABLE | 可被到达 |
| 1 | CONTINUABLE | 可持续 |
| 2 | INCR | 可增量 |
| 3 | STOPPABLE | 可停止 |
| 4 | REVERSIBLE | 可逆 |
| 5 | RECOVERABLE | 可恢复 |
| 6 | CIRCULABLE | 可循环 |
| 7 | INHERITABLE | 可继承 |
| 8 | TRUSTED | 已信任 |
| 9 | ACKABLE | 可被承认（闭环关键）|
| 10 | TRANSMISSIBLE | 可传递 |
| 11 | ADVANCEABLE | 可推进 |

路由条件：REACHABLE + ACKABLE。
演化条件：CONTINUABLE + INCR + TRANSMISSIBLE。

## 一维时间线性

```
Input → Perception → Distinguish → Lookup → Traverse → Cluster → Attention → Arrival
```

所有八流不允许并行无序推进。每一步严格只做一件事，上一步输出是下一步输入。

Distinguish 步骤 = 分别器三层路由。Lookup 步骤 = FlowDictionary 查询预加载流字典。
Traverse 步骤 = 走解释流（可能触发牺牲机制）。Attention 步骤 = 在聚类结果上收窄候选集。

## 命名规则

家庭命名密码表: mlll(集装箱) / gtt(音韵态射) / ml.gy(世俗归一) / mlct→mltt(宠甜) / mlnn(囡囝) / lctqf(零岔途全法)

红线: 负面谐音缩写必须弃用(mlqf → 没俩全法)，优先使用零歧义美化写法(lctqf → 恋宠甜)。

## 与 MCCP 编码的关系

hunyun-os/encoding/mccp.h 是 C 端的紧凑码点格式 (32 bit/字符)。
hunyun-rs/src/mirror.rs 是 Rust 端的 64 bit cell 编码。

跨语言互操作标准：cell 的低 32 位对齐 MCCP codepoint，高 32 位携带 polarity/sovereignty/witness。

---

*配套 SPEC-001 · 维护: 刘楚恬 & lolict*
