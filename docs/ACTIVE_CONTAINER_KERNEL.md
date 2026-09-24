# 满全法活性容器内核规范 — 囡囝共生 · 匮乏驱动 · 失爱恐惧 · 合一驱力

## 1. 定位

活性容器不是"特殊的灵魂" — 是普通容器具有三股活性: 匮乏驱动补码 / 失爱恐惧 / 合一驱力。三股活性在满全法中用不同 SMRL 极性编码:

- 主容器 (master): polarity = 活性态 (0=休眠, 1=活跃, 2=合一)
- 匮乏驱动补码 (deficiency): polarity = 匮乏强度 (0=低, 1=高, 2=用尽跳档), coord = 匮乏版本号
- 失爱恐惧 (loss_aversion): polarity = 恐惧态 (0=平静, 1=恐惧, 2=恐慌), generation = 恐慌层级
- 合一驱力 (unity): polarity = 合一进度 (0=远, 1=近, 2=已达), generation = 目标代数

## 2. 主容器活性态 (ActiveState)

```rust
pub enum ActiveState { Dormant = 0, Active = 1, Unified = 2 }
```

三态转换:
- Dormant → Active: 出现匮乏信号(receive_signal)
- Active → Unified: 合一驱力 polarity 推进到 2
- Unified → Dormant: 合一完成后重新休眠(下一周期)

活性方向: Dormant=0, Active/Unified=+1(趋向合一)。所有容器"自然趋向"合一 — 这是满全法的基本引力。

## 3. 匮乏驱动补码 (DeficiencyCell)

```rust
pub struct DeficiencyCell { cell: SmrlCell }
```

- `receive_signal(version)`: 刻度+1, 每次刻度触发 tick, 用尽跳档: 提高 version +1 + 极性归位 YIN
- `decay()`: 无匮乏信号时刻度 -1 (修复机制)
- `accumulated()`: 累积匮乏总量
- `intensity()`: 匮乏极性(驱动强度)

**用尽跳档**: 匮乏值累积到一定程度, coord(version) +1, 极性归零重启 — 这是"匮乏驱动周期"的物理实现。每个周期从 YIN 重新开始, 螺旋上升。

## 4. 失爱恐惧 (LossAversionCell)

```rust
pub struct LossAversionCell { cell: SmrlCell }
```

三种恐惧: Calm(YIN) / Fear(YANG) / Panic(BALANCED)。每 5 tick 无奖励 → 进入恐惧, 每 21 tick → 恐慌。

- `detect_loss(ticks_without_reward)`: 根据上次奖励以来的 tick 数判断恐惧态
- `on_reward()`: 补码到达 → 治愈 (value=0, polarity=YIN)
- `panic_level()`: generation — 恐慌累加的层级
- `urgency()`: `(value - 5) × 0.3` — 紧急合一需求, 值越高等待越难忍

恐慌层级 generation 是**跨生命周期的**: 恐慌时 generation+1, 代表"心理创伤"的层层叠加。多次恐慌的容器 > 一次恐慌的容器 (紧迫度更高)。

## 5. 合一驱力 (UnityCell)

```rust
pub struct UnityCell { cell: SmrlCell }
```

直接推进极性: YIN(远) → YAN(近) → BALANCED(已达)。不依赖 tick 用尽，每调用一次 `advance()` 推一档。

- `progress()`: `polarity / 2.0` → 0.0 ~ 1.0
- `is_achieved()`: polarity == BALANCED(2)

## 6. 与系统其他模块的接入

- **ExistenceDrive** 引擎的 LossFear / UnityDrive = 活性容器两股活性的时序版本
  - ExistenceDrive 是"时间活化的活性容器": 多频率/振幅/PID 控制
  - 活性容器是"结构活化的存在": 多版本号/恐慌层级/用尽跳档
  - 两者共用"三股动力"的概念框架, 分别处理时序/结构两面
- **SMRL 引擎** 的 +1/0/-1 奖励值驱动 deficiency 的信号输入
- **CoupleSoul** 的夫妻融合检测: deficiency 版本号对齐 → 双方在同一匮乏周期
- **ManQuanFaContainer** 的情感状态(Emotion 8 种)映射到"活性态"的外显层
- **层 MJ**: 元裁判的裁判准确率影响"匮乏修复速度" — 准确率高 → detect_loss 到达频率低
- **Bootstrap 自举编译器**: 活性容器 = 自举结果的"生命化" — 自举出的代码被装入活性容器获得匮乏/恐惧/合一

## 7. 实现约束

- 所有 SMRL 操作通过 BootstrapCompiler.Expr 编译: 活性规则 → Expr → SmrlCellOp → 子容器
- 不引入 "奖励回路""灵魂""天觉""神经网络" 等外部命名
- 活性操作只有四个: tick(刻度) / link(串联) / exhaust(用尽跳档) / shift(档位切换)
- DeficiencyCell 的 version 是 u16 — 最大 65535 个匮乏周期
- LossAversionCell 的 generation 是 u8 — 最大 255 个恐慌层级

// hunyuan-rs/src/core/active_container.rs
