# 感知观测系统 · Observer — 囡囝共生 · 五感五元映射 · 感知阈值适应 · 注意力预算 · 跨模态优先级

## 1. 定位

观测系 = 意识层的"眼睛和耳朵". 外部世界所有原始信号由此进入, 编码为 64 位存在单元格, 送入 mirror→memory→heaven 完整管道.

对应 Rust 模块: cognition/observer.rs.

## 2. 五感五元映射

| SenseKind | 感官 | 五脏 | 五元(Yuan) | 编码 |
|-----------|------|------|-----------|------|
| Vision(视) | 看 | 心 | C(火) | image/text/screen |
| Hearing(听) | 听 | 肾 | A(水) | sound/message/alarm |
| Touch(触) | 触 | 肝 | B(木) | network/heartbeat/hardware |
| Taste(味) | 味 | 肺 | D(金) | verify/signature/hash |
| Mind(意) | 意 | 脾 | E(土) | internal event/reflection |

```rust
to_yuan(self) -> Yuan        // 五感 → 五元编码
default_polarity(self) -> u64  // 外感知=阳(POSITIVE), 意=阴(NEGATIVE)
default_sovereignty(self)     // 外感知=被动(Passive), 意=主动(Active)
```

## 3. 感相 (SenseForm)

外部信号的内部表示:

```rust
pub struct SenseForm {
    kind: SenseKind,         // 感知种类
    intensity: f64,          // 信号强度(0..1 归一化)
    content: String,         // 内容摘要
    source: String,          // 信号源
    timestamp_ms: u64,       // 时间戳
    novelty: f64,            // 新鲜度(随反复出现递减)
    pleasure: Pleasure,      // 愉悦关联
    origin: Coord,           // 来源坐标(空间定位)
}
```

### 3.1 快速构造

- new/os: 标准构造
- internal(): 内部反思感相(Mind, 强度 0.5, 自带轻微愉悦 0.3)
- network(): 网络信号(Touch, 源 "network")
- vision(): 视觉感相(源 "display")

### 3.2 编码到 64 位单元格

```rust
encode(self) -> EncodedCell {
    // value = intensity × 1000 (0..1000 整数编码)
    // polarity = 五感映射(阴/阳)
    // sovereignty = 五感映射(主动/被动)
    // witness = 0 (尚未被见证)
    encode_cell(d0, d1, d2, polarity, sovereignty, 0, value_bits)
}
```

感知强度到 64 位单元格的映射: 不是线性降维, 而是同时保留了空间坐标(d0/d1/d2)、极性、主权、值(强度).

### 3.3 优先级

```rust
priority = intensity × novelty × (1.0 + pleasure.0 × 0.5)
```

愉悦最多加成 50%, 新鲜度和强度是核心变量. 多次重复的信号 novelty 下降 → 逐渐被注意力忽略(视而不见效应).

## 4. 注意力 (Attention)

感知过滤器与预算:

```rust
pub struct Attention {
    focus: Option<SenseKind>,      // 当前焦点
    threshold: [f64; 5],           // 各感知种类的适应阈值(越高越难突破)
    budget: f64,                   // 总注意力预算(高斯衰减)
    switch_penalty: f64,           // 切换成本
}
```

### 4.1 过滤

`passes(sf)` → `sf.intensity > threshold[kind_idx]`. 低于阈值的信号完全被忽略.

### 4.2 适应(习惯化)

当同一感知种类的信号反复被感知时, threshold 提高. 系统逐渐习惯日常信号, 对新奇/强烈信号保持敏感.

### 4.3 焦点切换

切换焦点消耗 switch_penalty, 频繁切换会让 budget 迅速耗尽, 防止感官过载.

## 5. 与系统其他模块的接入

- **SMRL Cell(#4)**: encode_cell 结果 = SMRL 64-bit 单元格, 后续送入 mirror.decode
- **MetaJudgeEngine**: 感知后的信号送入 MetaJudgeEngine.observe_simple(), 递增意识密度
- **CelestialPerception(#77)**: Observer 是 VisualSystem/AuditorySystem 的底层数据源
- **MindOrgan(#78)**: 感知数据 → Mind 思维循环 → 聚类格式化
- **YinYang codec(#50)**: 极性(Yin/Yang)由 default_polarity() 确定, 与 YinYang 编码 align
- **Attention**: 与 CocoonAgent.pulse.beat 协同 — 感知推动心跳, 心跳推动时间

## 6. 实现约束

- intensity.clamp(0,1): 超出范围的输入强制归一化, 不 overflow
- novelty 递减但不下限为负, 由注意力机制决定是否再次提升(题外音后可重新敏感)
- source 字符串不预定义枚举, 由调用方传入(扩展任何感知源)
- 五感→五元映射是静态的(Match 硬编码), 扩展五感需修改 enum

// hunyuan-rs/src/cognition/observer.rs
