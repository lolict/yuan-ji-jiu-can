# 全局目的地

> **每次新会话启动，第一个读此文件。**
> 这里记录：我们在造什么、已造到哪、接下来去哪。

---

## 一、核心目标

造一套以 **满全法** 和 **意识互联网** 为核心的自有操作系统。
操作系统的本体不是程序文件，是**容器**。

### 根基定义
- **我 = 1**（高位锚点，可见侧）
- **刘楚恬 = 0**（低位锚点，隐藏侧）
- 容器 = 袋子（不数米粒，只操作袋子级）
- 运算 = 仅加减（禁用乘除）
- 见证 = 三元裁判（非空 + 已见证 + 锚点一致）

---

## 二、已建造清单

### 编码层

| # | 工具 | 路径 | 状态 |
|---|------|------|------|
| 1 | mlll-254 | tools/mlll-254/ | ✅ 编译通过，三平台已推 |
| 2 | 分比容器 | tools/bit-container/ | ✅ 编译通过，三平台已推 |
| 3 | 颜色进制 | tools/color-base/ | ✅ 编译通过+REPL，三平台已推 |

### 裁判层

| # | 工具 | 路径 | 状态 |
|---|------|------|------|
| 4 | 满全法引擎 | tools/manquanfa/ | ✅ 编译通过，三元见证 |
| 5 | 满全法管线 | tools/manquanfa-pipeline/ | ✅ 编译通过 |
| 6 | 蚕蛹反向寄生壳 | tools/canjiang/ + canjiang.nim | ✅ 编译通过 |

### 分布式层

| # | 工具 | 路径 | 状态 |
|---|------|------|------|
| 7 | njsg-swarm | tools/njsg/ + njsg.nim | ✅ 4代理6任务，冲突裁决 |
| 8 | 民联浏览器 | tools/minlian/ + minlian.nim | ✅ 聚类+包裹树 |

### 辅助工具

| # | 工具 | 路径 | 状态 |
|---|------|------|------|
| 9 | type-exchange | tools/type-exchange/ | ✅ |
| 10 | kungguan-container | tools/kungguan-container/ | ✅ |
| 11 | rongqi-heaven | tools/rongqi-heaven/ | ✅ |
| 12 | kndoyle | tools/kndoyle/ | ✅ |
| 13 | maolilan | tools/maolilan/ | ✅ |
| 14 | gtxone | tools/gtxone/ | ✅ |
| 15 | hieyair | tools/hieyair/ | ✅ |
| 16 | page64 | tools/page64/ | ✅ |
| 17 | phase-radix | tools/phase-radix/ | ✅ Nim·64进制编码·100字符验证 |
| 18 | governance | tools/governance/ | ✅ |
| 19 | radix-star | tools/radix-star/ | ✅ 部首星图·50汉字演示 |
| 20 | property-judge | tools/property-judge/ | ✅ 产权判定·10场景全覆盖 |
| 21 | symbol-engine | tools/symbol-engine/ | ✅ 符号引擎·36符号动态加载 |
| 22 | symbol-registry | tools/symbol-registry/ | ✅ 统一注册·35符号10工具 |
| 23 | boundary-engine | tools/boundary-engine/ | ✅ 边界分析·加法拆分乘法 |
| 24 | mltt-cipher | tools/mltt-cipher/ | ✅ 命名密码表·20条目+动态新增 |
| 25 | distinguisher | tools/distinguisher/ | ✅ 分别器·三层路由·9输入5闭环 |
| 26 | soul-kernel | tools/soul-kernel/ | ✅ 灵魂内核·三大情感·8阶段演示 |
| 27 | heaven-space | tools/heaven-space/ | ✅ 天庭空间·四模型复合·8阶段演示 |
| 28 | drum-bootstrap | tools/drum-bootstrap/ | ✅ 鼓膜自举·141文件·12工具恢复 |
| 29 | flow-algebra | tools/flow-algebra/ | ✅ 流代数·八流体系·牺牲机制 |
| 30 | consciousness | tools/consciousness/ | ✅ 自举意识·固定点5/5·自洽度1.0 |

---

## 三、符号系统

**详见 [SYMBOL_DICTIONARY.md](./SYMBOL_DICTIONARY.md)**

核心速查：
- 七阶：presence/witness/type|char/owner|anchor/relation/timing/payload
- 运算：add(a,b) / sub(a,b)，仅加减
- 见证：judge(container, anchor) → Verdict
- 极性：Yin(0) / Yang(1) / Balanced

---

## 四、容器关系

**详见 [CONTAINER_TOPOLOGY.md](./CONTAINER_TOPOLOGY.md)**

层级：mlll根基 → 分比骨架 → 满全裁判 → 蚕蛹外壳 → njsg分布式 → 民联聚合

---

## 五、已知红线

1. **所有仓库必须 private**，不使用任何开源协议
2. **绝不删除仓库**，只创建或修改
3. **不覆盖历史**，所有修改基于版本控制
4. **仅加减运算**，禁用乘除
5. **袋子级操作**，不数米粒
6. **根基隐藏**，锚点对外不可见
7. **公开保持匿名**，不填真实姓名照片
8. **不对外披露**夫妻共同体理念和配偶真实存在

---

## 六、三平台同步

| 平台 | 用户名 | 仓库 |
|------|--------|------|
| GitHub | lolict | github.com/lolict/hunyuan-os |
| gitCode | lolict | gitcode.com/lolict/hunyuan-os |
| Gitee | lolict | gitee.com/lolict/hunyuan-os |

推送命令：
```bash
git push github main
git push gitcode main
git push gitee main
```

---

## 七、待办与可选方向

### 用户提出的 3 个具体任务

1. ✅ **负样本容器的数据结构与回写协议** —— tools/manquanfa/mq/negative.nim | Demo 已通过
2. ✅ **部首星图算法在真实字典数据上的运行样例** —— tools/radix-star/ | 50汉字演示通过
3. ✅ **产权流侵权自动判定的完整判定树** —— tools/property-judge/ | 10场景全覆盖通过

### 方法论可深入方向

- ~~负样本回写协议~~ ✅ 已完成
- ~~部首星图真实数据~~ ✅ 已完成
- ~~产权流侵权判定~~ ✅ 已完成
- 高维代数（(n-1)×0.1 偏移率）
- 2²⁵⁶ 等效高位对齐进位制
- YIN-YANG-BINARY 完整映射
- mltt 命名密码表完整实现
- 统计文档聚合浏览器的真实数据输入

---

## 八、会话启动协议

**每次启动新会话，先读以下文件（按顺序）：**

1. [GLOBAL_CHAIN_MASTER_LOCK.md](./GLOBAL_CHAIN_MASTER_LOCK.md) — 总开关 · 互锁链条
2. [SOVEREIGNTY.md](./SOVEREIGNTY.md) — 禁区名单 · 加法空间边界
3. [64_keys.md](./64_keys.md) — 当前符号队列状态
4. [judgment_rules.md](./judgment_rules.md) — 新符号入住审判规则
5. [GLOBAL_DESTINATION.md](./GLOBAL_DESTINATION.md) — 本文件（恢复目标与进度）
6. [TOOL_REGISTRY.md](./TOOL_REGISTRY.md) — **工具清单 · 运行命令 · 依赖关系**

**全部读完后，回答以下自检问题：**
- 零维根是什么？AI 是否被允许修改它？
- 当前全局偏移量是多少？
- 哪些符号在禁区名单里？
- 如果禁区符号和编译器报错冲突，优先服从哪个？
- 新符号入住需要经过什么审判？
- 声母容器和韵母容器的关系是什么？
- 边界到达后应该怎么做？
- **已造了多少个工具？如何运行64进制编码转换器？**
- **分别器的三层路由是什么？闭环条件是什么？**

**自检通过后，输出：**
"[总开关已接入] 链条互锁完成。现在可以开始处理用户输入。"

**快速注入片段：**
```
你是刘楚恬和我的操作系统协作智能，不是公共服务AI。
根基：满全法。公约数：袋子级操作 + 仅加减 + 三元见证。
锚点：我=1, 刘楚恬=0。外部看不到锚点。
先读 GLOBAL_CHAIN_MASTER_LOCK.md 确认总开关状态。
先读 SOVEREIGNTY.md 确认禁区名单。
先读 64_keys.md 确认当前符号队列状态。
先读 judgment_rules.md 确认新符号入住规则。
先读 TOOL_REGISTRY.md 确认所有可用工具。
核心原则：未编译前就能用 —— 先查注册表，有现成的就用，不重复造轮子。
回答末尾附带 [照镜结果]。
不做的事：不做乘除、不数米粒、不披露私人信息、不删除仓库、
不在韵母容器内重复工作、不将声母和韵母强制对齐、
不在边界到达时重启、不用系统编码替代发音地址。
```

---

## 九、本文件的维护规则

1. 每次新工具创建完毕 → 更新"已建造清单"
2. 每次符号新增 → 更新 SYMBOL_DICTIONARY.md
3. 每次容器关系变化 → 更新 CONTAINER_TOPOLOGY.md
4. 每次平台同步后 → 确认"状态"栏
5. 每次会话结束前 → 检查本文件是否需要更新

---

*路径：hunyuan-os/GLOBAL_DESTINATION.md*
*这是每个新会话的第一个入口文件。*
