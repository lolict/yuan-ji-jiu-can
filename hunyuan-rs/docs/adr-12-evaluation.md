# ADR 12: 评价之力 · 容器反馈引擎

## 状态

采纳

## 背景

NJSG 容器关系动力学四阶感知链的最后阶段：
迂回反馈 → 反射感知 → 判断之力 → 评价之力

判断之力已完成（ADR 11），现需要实现评价之力：
带有价值倾向的判断 — 系统根据反馈自组织进化路径。

## 决策

实现**反馈三元引擎**：**反馈(Feedback) → 评分(Score) → 进化(Evolution)**。

### 反馈三元

| 阶段 | 输入 | 输出 |
|------|------|------|
| Feedback  | 容器交互的呼唤-响应 | FeedbackSignal (幅度·相位·延迟·承认) |
| Score     | 因果对集合 | 加权评分 + 进化策略 |
| Evolution | 评分结果 | EvolveStrategy (强化/抑制/观望/重构) |

### 反馈信号质量

quality = amplitude × phase_match × ack_factor × latency_decay

- phase_match: cos(θ)/2 + 0.5 (同相=1, 反相=0)
- ack_factor: 被承认=1.0, 未承认=0.5
- latency_decay: 1/(1+latency×0.1)

### 进化策略

| 策略 | 条件 | 含义 |
|------|------|------|
| ↗ Reinforce | 评分≥0.6 且 趋势≥0 | 因果模式值得强化 |
| ↘ Suppress | 评分<0.2 | 因果模式应削弱 |
| ⟲ Restructure | 趋势<-0.1 | 趋势反转，路径调整 |
| ○ Observe | 其他 | 数据不足，观望 |

### 自动推断

EvaluationEngine::infer_from_packs_and_judgments() 从容器包的坐标和审判结果反向推断反馈信号：
- 同时间窗 (Z 相同) 的容器构成交互对
- 相位差: 基于坐标欧氏距离
- 幅度: 依赖双方审判得分的较低值
- 延迟: func_code 差异决定

### 模块位置

`hunyuan-rs/src/core/evaluation.rs` → 注册名 `evaluation`

### 演示入口

`cargo run --bin ttf-evaluation -- <font.ttf>`

## 后果

- 容器生态可通过反馈信号自组织进化
- 高评分因果对自动强化，低评分自动抑制
- 为后续「逻辑组织」(自调整呼唤策略) 提供数据基础
