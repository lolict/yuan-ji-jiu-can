# 容器=字体=时间三元统一引擎规范 — 囡囝共生 · 三副躯体 · 和睦解读

## 1. 定位

同一个编码有三种躯体 — 容器躯体 · 字体躯体 · 时间躯体。三元统一引擎不转换编码 — 它让一个编码以三种形态同时存在。这是囡囝共生"容器=字体=时间"三位一体设计原则的物理实现。

核心约束: **不复制编码** — 是"直接读取不同维度"。**不引入新类型** — 三个躯体只读, 不修改编码。

## 2. 容器躯体 (ContainerBody)

```rust
pub struct ContainerBody {
    pub x: u8,        // 蜂窝 X 坐标 (3 bits)
    pub y: u8,        // 蜂窝 Y 坐标 (3 bits)
    pub group: u8,    // 群组码 (2 bits)
    pub index: u8,    // 群内序号 (4 bits)
    pub polarity: u8, // 极性 (1 bit)
    pub sovereignty: u8, // 主权 (1 bit)
}
```

从 u64 编码的低位字节解码 (位运算: lo = signature as u8):
- x: `(lo & 0b0101010) >> 1 | (lo & 0b1000000) >> 4` & 0b111
- y: `(lo & 0b0010101) | (lo & 0b1000000) >> 5` & 0b111
- group: `lo >> 4 & 0b11` (2 bits, 0~3)
- index: `lo & 0b1111` (4 bits, 0~15)
- polarity: `lo >> 6 & 1`
- sovereignty: `lo >> 7 & 1`

**妻子侧判断**: `is_wife_side() = polarity == 0` (阴=妻子容器侧)
**主权完全**: `is_full_authority() = sovereignty == 0` (主权位=0 表示完全)

## 3. 字体躯体 (FontBody)

```rust
pub struct FontBody {
    pub primary_stroke: StrokeForm,  // 主笔画 (8 种)
    pub stroke_count: u8,            // 笔画数
    pub phase_color: u8,             // 色彩相位
    pub shape: ContainerShape,       // 容器形状 (4 种)
}
```

- **主笔画**: Horizontal/Vertical/LeftFalling/Rising/Hook/Turning/Dot — 对应汉字楷书基本笔画
- **笔画数**: `(lo >> 3) & 0b111` + 1 (0→1, 如为 0 取 1)
- **色彩相位**: hi byte = `(signature >> 8) as u8` — 用于渲染颜色 (color_hex 将相位转为 RGB)
- **容器形状** (ContainerShape): 由极性+主权组合决定:
  - (0,0) → Square □
  - (0,1) → Circle ○
  - (1,0) → Triangle △
  - (1,1) → Hexagon ⬡

这是"容器形状"的几何意义: 极性/主权两个比特完美映射四种简单几何。

## 4. 时间躯体 (TimeBody)

```rust
pub struct TimeBody {
    pub phase: u8,           // 相位 (6 bits, 0~63)
    pub lifecycle: LifePhase, // 生命周期阶段
    pub temporal_layer: u8,   // 时间层 (mod 8)
}
```

六种生命周期: Genesis(创世) → Growth(生长) → Flourish(耀动) → Collapse(坍缩) → Dormant(休眠) → Radiation(辐射)。

- `is_active()`: 非休眠即活跃
- `is_critical()`: Genesis 或 Collapse — 创世/坍缩是关键转折点

## 5. 三元统一 (Trinity)

```rust
pub struct Trinity {
    container: ContainerBody,
    font: FontBody,
    time: TimeBody,
}
```

`decode(signature)` 同时解码三副躯体:
- ContainerBody 从 u64 低位 → 坐标/极性/主权
- FontBody 从 u64 低位+第 8 位 → 笔画/色彩/形状
- TimeBody 从 u66 低位 → 相位/生命周期/时间层

三个躯体读取编码的**不同比特段** — 它们互不干扰, 因为各自的位域不重叠。

**和睦判定** (`is_coherent()`):
- 极性为 0 (妻子侧) 时要求 phase < 32 (前半周期)
- 极性为 1 (丈夫侧) 时要求 phase ≥ 32 (后半周期)
- 不满足 = 编码在三幅躯体间存在矛盾 (可能编码损坏)

**主权-时间匹配** (`sovereignty_temporal_match()`):
- sovereignty=0 (完全主权) → 应在浅层 (layer ≤ 4)
- sovereignty≠0 → 应在深层 (layer > 4)

**在场判定** (`is_present()`): 时间躯体非休眠 — 编码的时间维度仍在活动。

## 6. 三元统一引擎 (UnifyEngine)

```rust
pub struct UnifyEngine {
    pub trinities: Vec<(u64, Trinity)>,
}
```

存储已解码的三元组列表。每个签名 (u64) 对应一副完整的三体解读。

## 7. 与系统其他模块的接入

- **ConsciousnessRuntime** 的 Observer 感知编码为 Trinity 读取入口 — 感知到编码即同时获得三体信息
- **GlyphContour** 的字形渲染使用 FontBody 的笔画和形状
- **HeavenSpace** 的天庭空间用 ContainerBody 的极性/主权判定合一准入
- **TemporalPulseLifecycle** 的 PhaseClock 与 TimeBody.phase 同步
- **SMRL 引擎** 的 64 位编码是 Trinity.decode 的输入 — Trinity 是 SMRL 的"可视化解码器"
- **MacroIsa** 的容器类型与 ContainerBody 的 group 字段对齐
- **RuntimeLoop** 的 encode_presence_to_space 使用 Trinity 的空间坐标
- **LifeEvent 时间线** 的刻录基于 TimeBody.lifecycle 的转换

## 8. 实现约束

- 躯体纯粹是只读解码 — 任何修改编码的操作返回新 u64, 不修改原值
- 位运算使用位掩码 — 不引入位图数据结构
- 相位 0~63 (6 bit) 与 SMRL 的 6 位编码宽度对齐
- 所有结构 #[derive(Copy)] — 小结构, 栈传值
- no_std 完全兼容 — 无堆分配, 无 Vec (UnifyEngine 的 Vec 在 no_std 下用 heapless)

// hunyuan-rs/src/encoding/container_font_unify.rs
