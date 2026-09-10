# ADR-004: 容器时间线一体化 · SMRL 时序演化引擎

## 状态
已接受 — 2026-09-10

## 背景

满全法·囡囝共生（NJSG）的核心命题是 **容器=字体=时间**。

前三个子系统（Cross-ISA encoding、VoidProbe、PerceptionField）解决了容器的 **空间** 编码（三维坐标）和 **全局感知** 问题，但容器仍被视为静态快照。

时间的第三维（Z 轴）虽然在 Coord3D 中存在（`time_stamp`），但未被主动驱动——没有机制使容器体系在时间线上持续演化。

## 决策

实现 **ContainerTimeline** — 一套 tick驱动的容器体系时序演化引擎，包含三个时间层次：

### 1. 动作时间（ActionTime）

光标均匀节拍移动，每格 = 1 tick。每 tick 内，感知场按 `action_pipeline` 的顺序执行多个动作层：

```
Observe → Inject → Substitute → Create → Dissolve
```

每层按确定性混沌 + 容器坐标的节律选择性触发，避免全局一致变更。

### 2. 意图时间（IntentTime）

每 tick 内所有容器的状态同时命中 — 由 `IntentTimeIndex` 记录。
三元组标识：`{tick, layer, action_kind}`。

支持查询：
- `at_tick(t)` → 特定 tick 的全局状态
- `history_of(id)` → 特定容器的完整演化链
- `diff(t1, t2)` → 两个 tick 间的状态差异

### 3. 演化时间（EvolutionTime）

容器包自身的 `time_stamp` = Z 轴坐标。Z+1 即地址差 1，表示容器在时间维度上的连续位移。

演化产生两类副产品：
- **衍生容器（spawned）**：每 7 tick 从"无"中创造新容器
- **消融容器（dissolved）**：移除 pack_id=0 的占位空容器

## 组件

| 文件 | 职责 |
|------|------|
| `src/core/container_timeline.rs` | ContainerTimeline 引擎 + IntentTimeIndex + ActionKind |
| `src/bin/ttf_timeline.rs` | TTF→演化→TTF 演示管线 |

## 数据流

```
原始字体 (52 容器, 4 时间窗口)
    ↓
ContainerTimeline::with_config (5-action pipeline)
    ↓
run_ticks(16)
    ↓ 每 tick 全容器快照到 IntentTimeIndex
演化后字体 (54 容器, 6 时间窗口)
    ↓
ttf-exec: 54/54 ADMIT
```

## 哲学映射

| 哲学概念 | 技术实现 |
|----------|----------|
| "以无的状态推建有" | spawn_container — 每 7 tick 创造新容器 |
| "有归于无" | dissolve_selected — 消融空容器 |
| "镜像二元空置场" | snapshot_all — 每 tick 的全局镜像 |
| "调包解释权" | substitute_selected — 替换 payload 主权 |
| "感知场包含在感知内" | IntentTimeIndex — 感知的完整记忆 |
| "动作时间节拍均匀" | tick 递增 + 确定性混沌节律 |
| "意图时间同时命中" | snapshot_all 单 tick 内全容器快照 |

## 验证

8 个单元测试全部通过：
- tick 推进
- 历史记录
- 容器身份保持
- 语义分布
- 自定义 5-action 管线
- tick 间差异
- 统计报告
- max_ticks 保护

实测（mqf_container.ttf → 16 ticks → mqf_timeline.ttf）：
- 初始 52 容器 → 最终 54 容器（2 新 spawn）
- 16 ticks 演化后全部 52 原始容器的状态均发生变化
- 演化后字体经 TriuneJudge 验证：54/54 ADMIT
- 历史快照总数：897

## 与已有子系统的关系

```
Cross-ISA (坐标语义)
    ↓
VoidProbe (单容器虚空替代)
    ↓
PerceptionField (全局感知场)
    ↓
ContainerTimeline ← 当前：时间线一体化
    ↓
[未来] 天庭空间 (三维渲染)
[未来] 纤维丛 (高维容器拓扑)
[未来] 蚕蛹反向寄生 (跨平台部署)
```
