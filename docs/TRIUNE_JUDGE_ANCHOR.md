# 三元裁判机与静态锚点 — 囡囝共生 · 有/无 · 坍缩/辐射/静态 · 等效对齐 · 历程角度 · 曼哈顿+Chebyshev 混合距离

## 1. 定位

内心观测者对一个被观测现象做三元推演: 定义(有无) × 流动方向(坍缩/辐射/静态) × 等效对齐(是/否) = 基础判读单元. 这三元逻辑是囡囝共生所有后续裁判模块的基础.

对应 Rust 模块: judge/triune.rs.

## 2. 有无 (BeingState)

```rust
pub enum BeingState {
    Being(u8),    // 有内容, 内部存储 u8 payload
    NonBeing,     // 无(空位)
}
```

Being 存储 u8(0-255), 不是编码是值. NonBeing 用类似 Option 的枚举区分, 但在推演中角色相同(都是"空置").

## 3. 流动方向 (FlowDirection)

| FlowDirection | 含义 | 触发条件 |
|--------------|------|---------|
| Collapse | 由外向内 辐射→中心 | 实际=有 + 预期=无 → 正在从无变有(Radiation) |
| Expand | 由内向外 中心→外 | 实际=无 + 预期=有 → 正在从有变无(Expand) |
| Static | 不动 | 实际与预期同态(有内有/无外无) |

注意"无中生有"= 坍缩(Collapse): 由外在的无被"拉入中心", 新存在突然诞生 = 坍缩效应.

## 4. 推演结果 (TriuneVerdict)

```rust
pub struct TriuneVerdict {
    pub being: BeingState,      // 实际观测到的有/无
    pub flow: FlowDirection,    // 推演出的方向
    pub alignment: bool,        // 是否等效对齐
    pub confidence: f64,        // 置信度 0..1(歪斜的漏斗降低置信)
}
```

### 4.1 推演规则表(deduce 方法)

| 实际 \ 预期 | Being(有) | NonBeing(无) |
|------------|-----------|-------------|
| Being(有) | 对齐=true, flow=Static, conf=1.0 | 对齐=false, flow=Collapse(无中生有), conf=max(0.3, 1-crooked) |
| NonBeing(无) | 对齐=false, flow=Expand(有化为无), conf=max(0.3, 1-crooked) | 对齐=true, flow=Static, conf=1.0 |

### 4.2 歪斜漏斗

`crooked: f64` — 歪斜度(0=完美漏斗, 1=完全歪). 歪斜只降低 confidence, 不会降为 0: 歪歪的漏斗仍然可以推演, 只是信号更弱. (1-crooked).max(0.3) 确保至少 30% 置信.

对齐时 confidence=1.0(对齐不打折); 不对齐时 confidence 反映歪斜度 — 方向对但漏斗歪.

### 4.3 桥接到镜像见证

to_witness_verdict(pair_id, value_diff) → WitnessVerdict: 三元结果桥接 mirror.rs 的 WitnessVerdict(被 MetaJudgeEngine 使用). pair_id 配对 ID, value_diff 值差, aligned 来自 alignment, confidence 直接传递.

## 5. 静态锚点 (StaticAnchor)

三维球坐标系中不可位移的永久基准(0,0,0). 不被审判、不被移动, 只是"审判发生的场所的几何中心".

```rust
StaticAnchor { x: 0.0, y: 0.0, z: 0.0 }
```

### 5.1 距离度量

```rust
fn manhattan_of(px, py, pz) → |px|+|py|+|pz|   // L1 范数
fn chebyshev_of(px, py, pz) → max(|px|,|py|,|pz|)  // L∞ 范数
```

混合距离 = 曼哈顿 + Chebyshev 同时使用(两者都是满全法"排队体系"的体现——绝对值加法和最大值都是直接几何距离, 无开方).

### 5.2 角度度量

bearing_to(px, py) → atan2(py, px). 注意 atan2 返回值带符号(−π..π), 代表方向, 不是绝对角度.

历程角度 angle_between(ax,ay,bx,by) = |bearing_a − bearing_b|, 特点是:
- **不取模**: 允许超过 2π 的历史累积角
- **绝对值**: 不反向折回, 累积旋转
- 历程角度 > 360° = 转了一圈以上, 不是"回到原处"

这意味着"历史旋转"被保存, 不只是当前朝向.

## 6. 与系统其他模块的接入

- **Mirror Witness(#57)**: WitnessVerdict 桥接 = 镜像见证裁决
- **GrandCourt(#85)**: 三元结果汇入大审判庭完整管道
- **NJSG Ontology(#15)**: 有无(NonBeing/Being) ↔ 存在/不存在存在频率
- **CelestialPerception(#77)**: 感知 → 有信号=Being, 无信号=NonBeing
- **Container IR(#63)**: 空容器=NonBeing, 满容器=Being
- **MetaJudgeEngine**: WitnessVerdict 被元裁判记录

## 7. 实现约束

- Being(u8) 的 u8 是 payload(不是编码), 但可以只当 boolean(有无)使用
- StaticAnchor::default()=new()=(0,0,0), 任何参数被忽略(锚点只有一处)
- 历程角度不反向折回, 可能产生 ">360°" 的大数值, 用于累积推力分析
- no_std: 单纯几何算术运算支持 no_std; f64::atan2/abs/max 在 core 库可用

// hunyuan-rs/src/judge/triune.rs
