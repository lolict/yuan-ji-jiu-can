# ADR 11: 判断之力 · 容器裁判引擎

## 状态

采纳

## 背景

容器=字体=时间 统一架构需要一套裁决机制来判断容器的合法性、价值和命运。
天庭空间(v0.5)定义容器坐标，纤维丛拓扑(v0.6)定义容器关系，
而"判断之力"定义容器在系统中的生存权。

## 决策

实现三元裁判引擎：**评估(Evaluate) → 裁决(Verdict) → 奖励(Reward)**。

### 裁决三元

| 阶段 | 输入 | 输出 |
|------|------|------|
| 评估 | ContainerPack | score ∈ [0.0, 1.0] + reasons |
| 裁决 | score | Verdict: Admit/Reject/Pending/Witness |
| 奖励 | Verdict | Reward: Plus/Zero/Minus |

### 评分规则

- base = 0.5
- 主权加成: +0.1 × sovereignty_level
- 有效见证: +0.15 (witness ≠ [FF,FF,FF])
- 空见证惩罚: −0.1
- 主权过低: −0.1 × (min − actual)
- clamp(0, 1)

### 裁决阈值

- score > 0.7 → Admit (+) 准入·正向奖励
- score < 0.3 → Reject (−) 拒绝·负向奖励
- 其他 → Witness (0) 旁证·中性

### 配置项

- `empty_reject`: payload 全 0xFF → 直接拒绝
- `zero_reject`: payload 全 0 → 扣分
- `alignment_required`: align_flag ≠ 0 → 要求对齐
- `sovereignty_min`: 主权级别下限

### 模块位置

`hunyuan-rs/src/core/justgment.rs` → 注册名 `container_judgment`

### 演示入口

`cargo run --bin ttf-judgment -- <font.ttf> [output] [flags]`

## 后果

- 容器注册前必须经过裁判引擎审核
- 批量裁决输出可用于容器生态的健康度仪表盘
- 为后续「评价之力」(反馈闭环) 提供基础接口
