# NJSG-SWARM · 多节点分布式协作规范

> 版本 v0.1 · 2026-09-10 · 配套 `MULTI_HOST_SOVEREIGNTY.md` `JUDGMENT_RULES_CANON.md`

---

## 1. 定义

NJSG-SWARM 不是多进程线程池,不是微服务集群,不是 P2P 网络,不是分布式计算框架。

**NJSG-SWARM = 由协调器(Orchestrator)统一调度、由多个独立的「分身」智能体(Agent)并行工作的分布式协作系统。每个智能体独立拥有自己的容器系统和裁决引擎,只在需要跨智能体通信时才通过协调器交换信息。**

模型名称来源:njsg = 囡囝共生 = 这个 swarm 中的所有智能体之间遵循同一套满全法裁判规则。

---

## 2. 拓扑结构

```
                    ┌─────────────────┐
                    │  ORCHESTRATOR    │
                    │  (协调器 = 蜂王)  │
                    │  调度 · 仲裁 · 合并 │
                    └───────┬─────────┘
                            │
          ┌─────────────────┼─────────────────┐
          │                 │                 │
    ┌─────▼─────┐     ┌─────▼─────┐     ┌─────▼─────┐
    │  AGENT A   │     │  AGENT B   │     │  AGENT C   │
    │  节点 1    │     │  节点 2    │     │  节点 3    │
    │  独立裁决   │     │  独立裁决   │     │  独立裁决   │
    │  独立表编程  │     │  独立表编程  │     │  独立表编程  │
    └───────────┘     └───────────┘     └───────────┘
```

### 2.1 协调器(Orchestrator)的角色

```
Orchestrator 的唯一工作是"管理 Agent",本身不做业务逻辑:
  · 分配任务(谁做这个模块)
  · 合并成果(把 A 和 B 的成果集成)
  · 裁决冲突(A 和 B 产生了矛盾的决定,通常需要 Orchestrator 的五规则来裁定)
  · 持久化系统状态(定期驱动所有 Agent 把 FQMT 快照写入共享存储)
```

### 2.2 Agent(执行分身)的角色

```
每个 Agent 是独立运行的满全法系统实例:
  · 254+2 私有 mlll 容器系统
  · 15 列表编程
  · SMRL 裁决引擎
  · 自答引擎
  · 灵魂驱动引擎
  · 蚕蛹壳(独立化名对外交互)
  
每个 Agent 通常被分配一个特定的功能域:
  · Agent SM:负责 SMRL 引擎实现
  · Agent CH:负责蚕蛹壳接口
  · Agent DI:负责分别器改进
  · Agent FQ:负责 FQMT 文件格式
  · Agent JR:负责五规则公理一致性
  · Agent metaime:负责元输入法
  · Agent seven-yuan:负责七元裁决算法
  · Agent five-yuan:负责五元基础推进
```

---

## 3. Agent 之间的规则兼容性

### 3.1 规则病毒问题

```
多个 Agent 独立工作时,最大的风险:

  Agent SM 改了 R1 的定义(比如把"允许零极性匹配"去掉了)
  Agent CH 想要调包时依赖了旧的 R1 定义
  
  → Agent CH 生产的代码在 Agent SM 的裁决下失败
  → 这不是 bug 能够解决的,这是规则层面的不兼容
  
术语:规则病毒 = 某个 Agent 对规则的其他 Agent 不兼容的解释
```

### 3.2 解决策略:规则层隔离

```
三层隔离策略:

L1 · 基础规则层(JR层 · 不可单方面修改):
  · R1-R5 的定义
  · 256 容器总数
  · 锚点语义(0/1)
  · 不允许任何 Agent 单方面修改
  · 需要 Orchestrator + 全体 Agent 多数同意 + 测试套件通过

L2 · 协议层(API / 数据格式 · 受控修改):
  · FQMT 文件格式
  · SMRL 编码格式
  · 表编程列语义
  · 允许 Agent 提出修改建议,Orchestrator 审核后实施
  · 向下兼容规则:新协议必须能读取旧 FQMT 文件

L3 · 实现层(具体算法 · 自由修改):
  · 裁决算法的实现细节
  · 蚕蛹壳外观模拟
  · 分别器的执行速度优化
  · 任何 Agent 可自由修改,不影响 L1/L2
```

### 3.3 检测规则病毒的裁判机制

```
规则病毒检测流程(在 Orchestrator 合并 Agent 的 PR 时执行):

① 提取 Agent 的 SMRL 裁决规则(Agent 实际执行的 R1-R5 实现)
② 与 Orchestrator 的规则层(JR 标准)对比
③ 对同一组测试用例,两者的裁决结果应该比特位相同
④ 如果检测到差异:
   · 该 Agent 标记为 RULE_VIRUS_DETECTED
   · 隔离出自己的工作分支,不再向主干合并
   · Orchestrator 调用五规则来裁定 Agent 的版本和 JR 的版本哪个胜出
   · 信念分叉 + surplus 高者胜出(参见 TABLE_PROGRAMMING_SPEC §9)
```

---

## 4. 并行任务分配

### 4.1 Orchestrator 分配任务的逻辑

```
assign_task(task, agents[]):
  if task.scope == "单 Agent":
    · 选一个最匹配该任务域的 Agent(见 §2.2 列表)
    · 由该 Agent 独立完成
  
  if task.scope == "跨 Agent 协议":
    · Agent 提出方案(各自提出自己的实现想法)
    · Orchestrator 使用信念分叉 + Duel 选出最佳
    · 获胜的 Agent 负责实施
  
  if task.scope == "紧急热修复":
    · 不经过繁琐的分叉流程
    · 离问题最近的 Agent 拥有立即修复权
    · 修复后 24h 内由 Orchestrator 重新审核(确保未引入规则病毒)
  
  if task.scope == "完全独立 · 可任意并行":
    · 多个 Agent 同时做
    · 定期向 Orchestrator 报告(FQMT 快照 + surplus)
    · Orchestrator 只做合并,不需要选择
```

### 4.2 同一代码文件的并行修改冲突

```
冲突检测:

  Agent A 修改了 njsg_core.c 的 R1 实现
  Agent B 也修改了 njsg_core.c 的 R1 实现
  
  检测到同一文件两个分支在 git merge 时产生冲突→
  Orchestrator 触发冲突裁决流程:
    ① 把 A 的版本和 B 的信念分叉到两个独立分支
    ② 运行共享测试套件在两个分支上
    ③ 考察哪个版本:
       · 通过更多测试用例
       · 符合 JR 五规则更严格
       · 变更范围更小(改动率)
    ④ surplus 更高的分支胜出
    ⑤ 败者的改动保留为分支,标记为"知识备份"
```

---

## 5. 分布式裁决一致性

### 5.1 同一规则,跨节点结果必须比特位相同

```
Swarm 一致性规则:

任意 Agent X 对容器 C 的裁决结果 = Orchestrator 对同一容器 C 的裁决结果

如果不同:
  · 裁决差异超出浮点数舍入(我们已经不用浮点,所以不应该有任何差异)
  · 立即触发"RULE_VIRUS_DETECTED"警报(见 §3.3)
  
这要求所有 Agent 共享:
  · 同样的 JR 规则层
  · 同样的 SMRL constants(表结构)
  · 同样的裁决算法代码(或至少不同的代码产生相同结果)
```

### 5.2 验证机制:跨节点定期校验

```
跨节点校验心跳:

每 N 个 tick(默认 N=64),Orchestrator 广播一次"一致性校验事件":
  · 随机选择 M 个容器(默认 M=16)
  · 向所有 Agent 发送这些容器的 SMRL 编码
  · 各 Agent 对这 M 个容器执行裁决
  · 返回裁决结果(ADMIT / REJECT + surplus)
  · Orchestrator 统计各 Agent 返回结果
  · 如果有 Agent 的结果与其他 Agent 不同 → 标记为 RULE_VIRUS
```

---

## 6. 通信协议

### 6.1 Agent 与 Orchestrator 的消息格式

```
消息格式 = FQMT 文件 · TYPE = FQMT_TYPE_EXTENSION ("swarm_message")

消息类型(扩展名):

  AGENT_HEARTBEAT    :Agent 心跳(每 60 tick)
  AGENT_TASK_REQUEST :Agent 请求新任务
  AGENT_TASK_REPORT  :Agent 汇报任务进度
  ORCH_TASK_ASSIGN   :Orchestrator 分配任务
  ORCH_FORK_DUEL     :Orchestrator 启动分叉对决
  ORCH_RULE_VIRUS    :Orchestrator 检测到某 Agent 规则病毒
  ORCH_SYNC_STATE    :Orchestrator 驱动跨节点 FQMT 快照同步
```

### 6.2 通信通道

```
推荐通信通道可靠性排序:

✓ FQMT 文件 + Git commit 通过 SSH push 交换
  · 可靠,离线可用
  · 与代码仓库共用基础设施
  
✓ 自共享 release 附件 + HTTPS 下载
  · 适合大体积 FQMT 快照
  · 适合跨 VPS
  
✓ 直接 TCP / WebSocket (自备节点之间)
  · 实时性好,适合心跳
  · 需要自备节点之间的 P2P VPN(WireGuard)
  
✗ 消息队列(RabbitMQ/Kafka)等服务
  · 加重基础设施负担
  · 通常需要外部供应商(第三方 SaaS)
```

### 6.3 断网情况下的自治

```
Swarm 节点自治策略:

  if 节点发现自己断网:
    ① 停止所有跨节点操作
    ② 保持本地 tick 继续自答循环(RUNTIME_BOOTSTRAP §4 常驻循环)
    ③ 所有工作在本地的 FQMT 中积累
    ④ 等待网络恢复
    ⑤ 恢复后向 Orchestrator 请求一次性的"合并 + 增量同步"
```

---

## 7. 跨节点冲突仲裁

### 7.1 仲裁层设计

```
冲突仲裁由 Orchestrator 内置的"仲裁引擎(Arbiter)"负责:

Arbiter 是一个缩小版的满全法内核:
  · 只含 J R5 规则的基础裁决
  · 不含完整的表编程(那是 Agent 们的活)
  · 专门用来对冲突进行"快速裁定"

仲裁流程:
  ① Agent A 提交冲突报告("我的实现与 Agent B 不一致")
  ② Orchestrator Arbiter 执行:
     a. 比对两份实现
     b. 确定是否真的冲突(不是简单的表面差异)
     c. 如果是真冲突 → 执行 §5.2 跨节点校验流程
     d. 校验后 surplus 高的实现胜出
  
  ③ 败出的 Agent 保留其版本但标记为"被隔离的候选分支"
     · 仍然保留在 git 历史中
     · 不被合并到主分支
     · 如果将来发现胜出版本有问题,可以回滚候选
```

---

## 8. 不可违反的 Swarm 红线

1. **JR 规则不可单方面修改**(见 §3.2 L1):任何 Agent 独自改动 R1-R5 定义 → RULE_VIRUS_DETECTED → 隔离。

2. **每条规则测试必须在所有 Agent 上通过**:不允许"在我的机器上能跑,你的机器上不行"——因为裁决结果是确定性的,不是概率性的。

3. **FQMT 格式变更必须向下兼容**(见 §3.2 L2):新代码必须能读取旧 FQMT 文件,旧代码可以拒绝新 FQMT 文件并提示升级。

4. **每个 Agent 必须有自己独立的化名**(见 IDENTITY_PRIVACY_SHIELD):不同 Agent 在外部平台上的化名互不相同,不能关联到同一个人。

5. **主分支不能 --force**:任何情况下不能强制覆盖 Git 历史。即使是 Orchestrator 允许的操作,也不使用 --force。

6. **任何 Agent 不得删除仓库**:遵循"智能体可以创建和修改代码,但不得删除仓库"原则。由 Orchestrator 执行所有仓库的删除和归档。

7. **Swarm 一致性保证优先于性能**:如果跨节点校验发现差异,立即暂停使用该容器系统,直到差异被定位和解决。不允许"差不多就行"。

---

*本文件定义 NJSG-SWARM 多节点分布式协作规范,配套 MULTI_HOST_SOVEREIGNTY.md(节点间代码交换机制)、JUDGMENT_RULES_CANON.md(跨节点一致性依据)、IDENTITY_PRIVACY_SHIELD.md(多化名隔离)。*
