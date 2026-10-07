# 天庭感知系统 v1-v13 合一合并 — 囡囝共生 · 授权 · 袋子属性 · 多维感知 · 感知云 · 活码执行 · 系统引导

## 1. 定位

天庭系统将原 v1-v5 五个独立版本 + v8-v13 七个子模块合并为单一文件,解决"一个概念在五个文件中定义五种模样"的碎片化问题. 冲突解决策略: v4 保留原名, v3 对应冲突类型加 V3 后缀.

对应 Rust 模块: cognition/celestial_perception.rs (v1-v5 主文件 + v8-v13 子模块内联).

## 2. 授权体系

### 2.1 天庭授权 (HeavenlyCourtAuth)

三态: Dormant → Authorized → Revoked. 感知与裁决均要求 Authorized 态. authorize() 只能从 Dormant → Authorized, revoke() 直接到 Revoked.

### 2.2 听觉授权 (AuditoryAuth)

独立于天庭授权的听觉通道授权: Silent → Listening → Deafened. wake() 如处于 Silent 自动升级为 Listening, 类似于"听觉的自动唤醒". can_hear() 仅 Listening 为 true.

### 2.3 双授权模型的意义

天庭授权≠听觉授权; 一个容器可以被授权看见但不被授权听见(隐私容器), 反之亦然.

## 3. 袋子属性 (BagAttributes 14 维)

| # | 字段 | 类型 | 初始值(pristine) | 说明 |
|---|------|------|-----------------|------|
| 1 | sovereignty | u64 | 3(夫妻共有) | 主权归属 |
| 2 | value | u64 | 800 | 价值 |
| 3 | integrity | f64 | 1.0 | 完好性 |
| 4 | content_quality | f64 | 1.0 | 内容质量 |
| 5 | content_purity | f64 | 1.0 | 内容纯度 |
| 6 | bag_cleanliness | f64 | 1.0 | 容器清洁度 |
| 7 | dryness | f64 | 1.0 | 干燥度 |
| 8 | pest_level | f64 | 0.0 | 虫害度 |
| 9 | sand_level | f64 | 0.0 | 杂质度 |
| 10 | edibility | f64 | 1.0 | 可食性 |
| 11 | needs_content_wash | bool | false | 需要内容清洁 |
| 12 | needs_sand_pick | bool | false | 需要除杂 |
| 13 | wash_target | f64 | 1.0 | 加工目标清洁度 |
| 14 | needs_repair | bool | false | 需要修补 |

### 3.1 综合质量计算

`composite_quality()` = quality*0.30 + edibility*0.25 + purity*0.15 + integrity*0.10 + cleanliness*0.10 + dryness*0.05 + (1-pest)*0.025 + (1-sand)*0.025. clamp(0,1).

quality_after_processing() 在 composite_quality 基础上对需要加工的项目使用加工后目标值(purity→wash_target, cleanliness→1.0), 再扣除虫害/杂质惩罚.

审美分数 aesthetic_score(): 平均值×0.6 + 和谐度×0.4, 其中和谐度 = 1 − variance.sqrt(). 高均值+低方差=高审美.

## 4. 袋子 (Bag)

```rust
pub struct Bag {
    id, name, attrs: BagAttributes, coord: Coord,
    layer_depth, child_bag_ids, judgment_count, consecutive_failures, eliminated
}
```

judgment_count/consecutive_failures/eliminated 是 v3/EliminationSystem 的失败计数器: 袋子被判断为低质量时 consecutive_failures 累加, 达到阈值 → eliminated=true(被淘汰).

Bag::nested(id, name, attrs, coord, depth) — 嵌套构造, layer_depth 初始化为 depth(层级). child_bag_ids 用于树形层级.

## 5. 感知子系统

### 5.1 视觉系统 (VisualSystem)

sensitivity(敏感度) + threshold(阈值). perceive(bag) → Vec<SenseForm>: 每个被"看到"的属性产生一条 SenseForm. 感知结果的 detail 来自 BagAttributes 各字段(完好性/清洁度/虫害等).

### 5.2 听觉系统 (AuditorySystem) — v2 引入

AuditoryAuth 授权后才能听. 输入: 音韵信号(来自 YinYang 编码/声音感知). 听觉双授权 HeavenlyCourtAuth + AuditoryAuth: 同属于天庭体系.

### 5.3 感知聚合云 (ConvergenceCloud) — v9

v9 PerceptionReport + ConvergenceCloud: 聚合多个感知源的输出为单一"感知报告". FeatureJudgment 对每个特征做判定(良好/一般/差).

### 5.4 七系汇集 — v4 重构

七个子系统传入 CelestialPerceptionV4:
1. Emotion(七情离散) — 喜怒忧思悲恐惊
2. EmotionState(效价唤醒) — valence × arousal 二维
3. ThinkingSystem — 思维
4. AestheticSystem — 审美
5. EliminationSystem — 连败淘汰
6. FilterSystem — 质量过滤/准入过滤
7. ReasoningChain — 推理链

PipelineResult: len/blocked_count/message: Vec<String> — 管线处理摘要.

### 5.5 创意(活码)执行 — v5

HanziEncoding — 汉字编码对接全(?)_nyang 二进制; CommandParser 命令解析; FlowPropertyBits 流属性位; SelfRadix 自进制计算器.

## 6. 灵活动码/编译/执行 (LivingCode/JIT)

### 6.1 v12 指令包

```rust
pub struct InstructionBag { ... }
pub struct BagContainer { ... }
pub struct BagPool { ... }
pub enum BagStatus { Active, Dormant, Poisoned, Purged }
pub struct JitExecutionEngine { ... }
pub struct FrameRecord { ... }
```

InstructionBag 是容器化指令包 — 把机器码封装到容器中. BagPool 管理所有容器, JitExecutionEngine 做即时编译执行, FrameRecord 保留每帧的执行快照.

BagIntrospection 在运行时检查自身状态 — 容器对其内部机器码的自反射. LivingCodeCompiler 从源码/容器生成 InstructionBag.

### 6.2 活码编译器 (LiveCode)

v12 与 flow/livecode_compiler.rs(活码编译器#20) 的关系: v12 是"运行时容器层面的活码执行", #20 是"编译器层面的活码编译". v12 消费 #20 产出的字节码.

## 7. 安全命名空间 (Cap/Namespace)

v10 引入能力+命名空间模型(类似 POSIX capabilities):

```rust
pub struct CapSet { ... }        // 能力集
pub struct CapRegistry { ... }   // 全局能力注册表
pub struct Namespace { ... }     // 命名空间隔离
pub struct NsManager { ... }     // 命名空间管理器
pub struct BagSyscallRequest { ... }  // 容器调用请求
pub struct BagSyscallResult { ... }   // 容器调用结果
pub struct PolicyEngine { ... }       // 策略引擎
pub struct ProcContext { ... }        // 进程上下文(容器隔离)
```

PolicyDecision/PolicyEngine 实现"容器化系统调用拦截" — 每个 BagSyscallRequest 经 PolicyEngine 决策后才能执行. 这与 parasite_shell(蚕蛹壳)的脱壳穿衣流程形成正交互补.

## 8. 系统总线引导

### 8.1 v11 事件总线

CelestialBus — 系统内各子系统通过 BusMessage 通信:
- TimelineEvent/Timeline — 时间线事件追录
- BootStage/BootStepRecord — 引导步骤记录
- UnifiedBootstrap — 统一引导器

### 8.2 系统健康

SystemHealth 聚合所有子系统状态(正常/降级/故障). CelestialObserver 订阅 SystemHealth 变化.

SubSystem 枚举各子系统(视觉/听觉/思维/审美/淘汰/过滤/推理/活码/JIT/总线等).

## 9. WASM/蚕蛹寄生壳

### 9.1 v13 WASM 编译目标

WasmTarget/WasmModule/WasmCompiler — 天庭系统的 WASM 跨编译支持(与 wasm_ladder.rs #12 互补).

### 9.2 寄生壳

ShellPhase/ShellKind/ShellConfig/ParasiteShell — 蚕蛹反向寄生壳的运行时实现(与 chrysalis.md #6 理论层互补). ParasiteManager 管理多个寄生壳.

## 10. 多版本合并后的类型命名

保留两版:
- v4(最新版)/Emotion/Behavior/BehaviorSystem/EliminationSystem = 主用
- v3/v1 对应冲突类型 → EmotionV3/BehaviorV3/EliminationSystemV3

解决: v3 与 v1 中与 v4 重名的类型在 v3/v1 侧加 V3 后缀, 主用 v4 版本.

## 11. 与系统其他模块的接入

- **SMRL(#4)**: EncodedCell = mirror::encode_cell(d0,d1,d2,polarity,sovereignty,witness,value), BagAttributes.to_cell 编码 14 维属性到 u64
- **ContainerIR(#63)**: Bag 可序列化为 ContainerRef(kind=Relation)
- **MindOrgan(#78)**: VisualSystem/AuditorySystem 提供感知数据; MindOrgan 做聚类
- **MemoryRevolution(#71)**: Timeline/Event 持久化; BagPool 注册
- **CelestialPerceptionV8**: 盟机合袋子网络(AllianceNetwork 家族)
- **Judgment(#15/34)**: EliminationSystem 连败淘汰 → triune.rs WitnessSignature
- **CocoonAgent(#74)**: ParasiteShell 运行在 CocoonAgent 中

## 12. 实现约束

- v1-v5 合并后单一文件 ~146KB(大量内联测试), 是系统中最长的源文件
- v10 POSIX/syscall 模型仅在外向系统调用层(interface)使用, 内部仍然是满全法的 SMRL/容器语义
- v12 JIT Execution 需要 容器=机器码=内存 三重对齐(参见#49 TI 宏观指令)
- v13 ParasiteShell import 来自 #44/eco/silkworm_parasite.rs, 蚕蛹壳的"脱壳穿衣"全权由 v13 承接
- Bag 淘汰机制(consecutive_failures/eliminated)是"失败容忍度"的量化, 区别于 triune Witness 的"一致性判定"
- SystemHealth 枚举的"子系统"是正向健康报告, 区别于 CrimeTranscript( \#64 犯罪笔录)的反向异常记录

// hunyuan-rs/src/cognition/celestial_perception.rs
