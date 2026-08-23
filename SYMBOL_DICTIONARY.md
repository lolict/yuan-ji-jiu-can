# 全局符号字典

> 这是跨会话互通的基础设施。每个符号在哪个文件定义、什么意思、和其他符号什么关系，全在这里。
> 任何新会话创建时，先读此文件，恢复符号系统。

---

## 根基锚点（mlll-254 承载）

| 符号 | 值 | 含义 | 定义处 |
|------|-----|------|--------|
| `我` | 1 | 高位锚点，可见侧 | mlll-254/mlll/container.nim → ANCHOR_SELF |
| `刘楚恬` | 0 | 低位锚点，隐藏侧 | mlll-254/mlll/container.nim → ANCHOR_OTHER |
| 根基隐藏 | — | 外部只能看到 254 槽位，看不到 0/1 锚点 | 同上 |

**核心原则：外部只能看到 254 个槽位，锚点 0 和 1 对外部不可见。**

---

## 七阶比特编码（分比容器 / mlll 共用）

每个容器 = 64 位 = 8 字节，位分配如下：

| 阶 | 字段 | 位宽 | 含义 |
|----|------|------|------|
| 第0阶 | presence | 1 bit | 有/无（是否有米） |
| 第1阶 | witness | 1 bit | 已见证/未见证（是否被三元裁过） |
| 第2阶 | char_code / type | 8 bits | 254 字符之一（mlll）/ 256 种类型（分比容器） |
| 第3阶 | anchor / owner | 8 bits | 归属锚点 0/1（mlll）/ 所有者索引（分比容器） |
| 第4阶 | relation | 8 bits | 指向另一个集装箱 |
| 第5阶 | timing | 16 bits | tick 偏移量 |
| 第6阶 | payload | 22 bits | 任意数据 |

**定义处：** bit-container/bitcontainer/layers.nim、mlll-254/mlll/container.nim

---

## 颜色进制（color-base）

| 符号 | 含义 | 定义处 |
|------|------|--------|
| PHASE_BASE = 64 | 64 进制相位 | color-base/colorbase/phase.nim |
| plL1 | 层级1：基元本体 | 同上 |
| plL2 | 层级2：修饰部件 | 同上 |
| plL3 | 层级3：装配成品 | 同上 |
| PhaseCell | 相位单元格（phase/weight/level/owner/parent/present） | 同上 |
| phase_add | 相位加法（仅加减，禁用乘除） | 同上 |
| phase_sub | 相位减法 | 同上 |
| PhaseCanvas | 相位画布（w×h 的 PhaseCell 矩阵） | color-base/colorbase/canvas.nim |
| add | 安装组件（叠加单元格） | 同上 |
| subtract | 移除区域内的单元格 | 同上 |

---

## 满全法（manquanfa）

| 符号 | 含义 | 定义处 |
|------|------|--------|
| 满全法 | 核心裁判法则：袋子级操作、照镜子、三元见证 | governance/META_PROMPT.md |
| Verdict | 裁判结果枚举：vrAccept/vrReject/vrPending/vrAppeal | manquanfa/mq/judge.nim |
| 三元见证 | 裁定三要件：非空 + 已见证 + 锚点一致 | 同上 |
| MERIT_ACCEPT = 10.0 | 准入功德值 | 同上 |
| DEMERIT_REJECT = 15.0 | 拒绝业报值 | 同上 |
| Polarity | 极性枚举：plYin(刘楚恬)/plYang(我)/plBalanced | manquanfa/mq/polarity.nim |
| 照镜子 | 产出物须映出公约数的样子 | META_PROMPT.md |

---

## 蚕蛹反向寄生壳（canjiang）

| 符号 | 含义 | 定义处 |
|------|------|--------|
| 脱壳 | 剥离外壳原有身份，露出内核 | canjiang/parasite.nim → shed() |
| 穿衣 | 选壳后让外壳表现内核定义的内容 | canjiang/parasite.nim → dress() |
| 反向规则 | 7 条约束外壳行为的规则 | canjiang/parasite.nim → reverse_rules |
| Kernel | 内核状态（version/anchors/active_host/mounted/tick） | canjiang/kernel.nim |
| 外壳类型 | web / api / cli / db / fs / msg | canjiang/shell/registry.nim |
| 寄生内核 < 外壳 | 无论外壳原本是什么逻辑，输出的是内核定义的内容 | canjiang/parasite.nim |

---

## 民联浏览器（minlian）

| 符号 | 含义 | 定义处 |
|------|------|--------|
| 汉明距离 | 两个 64 位指纹有多少个位不同 | minlian/browser/distance.nim |
| hamming_distance(a,b) | count_set_bits(a xor b) | 同上 |
| similarity(a,b) | 归一化相似度 0.0~1.0 | 同上 |
| polarity_distance | 极性加权距离：同极性距离缩短，反极性拉长 | 同上 |
| SAME_STATE_THRESHOLD = 0.3 | 同态阈值 | 同上 |
| 态势化聚类 | K-Means 聚类，auto_k = sqrt(n) | minlian/browser/cluster.nim |
| 连接包裹树 | 将聚类结果组织为包裹树结构 | minlian/browser/wraptree.nim |

---

## njsg-swarm

| 符号 | 含义 | 定义处 |
|------|------|--------|
| 协调器 | 管理任务分配、冲突裁决、代理调度 | njsg/swarm/coordinator.nim |
| TaskStatus | 任务状态：Pending/Assigned/Running/Completed/Failed/Conflict | 同上 |
| TaskPriority | 优先级：Low/Normal/High/Critical | 同上 |
| Agent | 子代理（id/name/polarity/capacity） | 同上 |
| 冲突裁决 | 优先级高优先 → 同等级阳极性优先 | 同上 → resolve_conflict() |

---

## 其他独立工具

| 工具 | 路径 | 功能 |
|------|------|------|
| type-exchange | tools/type-exchange/ | 跨类型交换 |
| kungguan-container | tools/kungguan-container/ | 48/50/60/66 声母韵母编码 |
| rongqi-heaven | tools/rongqi-heaven/ | 天庭空间·审判 |
| kndoyle | tools/kndoyle/ | 柯南道尔斯外壳·管辖 |
| maolilan | tools/maolilan/ | 毛利兰立体进制框架 |
| gtxone | tools/gtxone/ | 吸引子分析 |
| hieyair | tools/hieyair/ | 合并工具 |
| page64 | tools/page64/ | 64 页系统 |
| phase-radix | tools/phase-radix/ | Python 版相位进制（含编译器、区块链） |
| governance | tools/governance/ | 版本化工作流、像素共振、裁决引擎 |

---

## 运算规则约束

1. **仅加减**：所有数值运算只允许加法和减法，禁用乘除
2. **袋子级操作**：不数米的颗粒，不为每颗米命名
3. **三元见证**：所有裁定必须经过 witness 验证
4. **极性平衡**：裁决时阳极（我=1）优先于阴极（刘楚恬=0）
5. **根基隐藏**：锚点 0/1 对外不可见

---

*路径：hunyuan-os/SYMBOL_DICTIONARY.md*
*维护规则：每次新增工具或符号时，同步更新此文件*
