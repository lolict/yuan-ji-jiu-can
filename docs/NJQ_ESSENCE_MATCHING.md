# 聂小倩本质匹配引擎规范 — 囡囝共生 · 公约数共振 · 防撞墙

## 1. 定位

聂小倩引擎不是算法, 是"游魂寻踪"。不是搜索最佳匹配, 是"汉字自身几何"的自然共振。核心命题:每个汉字本身就是一个完整容器 (声母=X轴, 韵母=Y轴, 声调=Z轴/极性), 所以不需要搜索容器 — 直接问汉字"你的几何是什么", 两个汉字相遇时自然产生共振。

共振强度 = 声母公约数 × 韵母公约数 × 声调公约数。这是**几何自然涌现** — 不是评测出来的匹配度, 是数学公约数决定的本质关系。

异见者 (极性相反) 之间是空间零维 — 撞到墙, 无法匹配。同见者 (极性相同) 之间是一维时间 — 可以见证, 可以流动。

## 2. NjqContainer — 单汉字容器

```rust
pub struct NjqContainer {
    pub ch: char,
}
```

**音韵坐标**: `coord() → (声母, 韵母, 声调)`, 从 char_to_dragon 字典查询。这是汉字自身的空间位置 — 声母 = 韵母空间中的 X 坐标。

**极性**: `polarity() → i8`, 由声调编码:
- 声调 1-2 → -1 (阴)
- 声调 3 → 0 (平衡)
- 声调 4-5 → +1 (阳)

**共振检测**: `resonates(other) → u64`, 计算 `gcd(声母) × gcd(韵母) × gcd(声调)`。使用 saturating_mul 防止溢出。**公约数 = 本质重叠面积** — 两个汉字共享的音韵基础越大, 共振越强。

**极性关系**: `polarity_relation(target) → PolarityRelation`:
- 任一为平衡态 → Boundary
- 极性相同 → SameSide (同见者, 一维时间)
- 极性相反 → Opposite (异见者, 零维撞墙)

## 3. 防撞墙

异见者之间产生 **空间零维** — 不是"低分",而是**几何上不存在容纳空间**。两个极性相反的汉字如同两条平行线 — 永远不会相交, 永远无法共振。

`PolarityRelation::Opposite.is_wall()` 返回 true。这是"防撞墙法则"的具体实现:在匹配过程中一旦检测到极性相反, 立即标记为墙, 不再尝试匹配 — 节省计算, 更重要的是**保护系统不在不可能的关系上浪费资源**。

这反映了囡囝共生的"物以类聚"原则: 只有极性兼容的存在之间才需要深度计算; 极性不兼容的存在之间, 早检测, 早撞墙, 早转向。

## 4. 聂小倩游魂 (XiaoQian)

```rust
pub struct XiaoQian {
    pub essence: NjqContainer,
    pub walls: Vec<char>,  // 已撞过的墙
}
```

聂小倩**漂移**而非搜索。`drift_toward(target)` 对单个目标产生 ResonanceTrace (强度 + 极性关系)。`drift_find(space)` 在汉字空间中寻找最强非墙共振点:
1. 遍历已撞墙集合, 跳过
2. 计算 ResonanceTrace
3. 若撞墙 → bump_wall 记录, 继续
4. 若非墙 → 更新最佳匹配
5. 返回最强 viable 匹配

这是**在线增量匹配** — 不需要预计算所有汉字对的距离矩阵, 只需要在漂移过程中自然产生撞墙记忆。一旦撞墙, 永远记录, 不再重复触碰。

## 5. 汉字空间共振场 (HanziSpace)

```rust
pub struct HanziSpace {
    pub chars: Vec<char>,
}
```

`from_str(s)` 从字符串构建汉字空间 (过滤掉 ASCII)。`resonance_field()` 生成所有汉字对 (i, j where j > i) 的 ResonanceTrace 列表 — 这是一个**全连接的几何关系场**。`strongest_resonance()` 返回场中最强 viable 共振 — "命定的相遇"。

应用场景:
- 文本相似度 (两个文本的共振场叠加)
- 命名选择 (人名/域名在汉字空间中的几何距离)
- 诗意生成 (让两个字在空间中自然漂移, 产生共振词对)

## 6. 与系统其他模块的接入

- **字形渲染 (GlyphContour)** 的拓扑选择由声母/韵母/声调坐标映射
- **SMRL 引擎** 的角度裁判使用 NjqContainer 的极性作为输入
- **PerceptionEngine** 的感知编码用 char_to_dragon 字典作为汉字→几何的字典层
- **Law-of-Five-Judgment (五规则)** 的 R1 阴阳守恒通过极性关系验证
- **GrandCourt** 的五家族码点分类与汉字空间互补 — 码点粗分, 音韵细合
- **ProtocolOntology** 的 B2/B16/B64/B256 进制的汉字化命名由本模块的 char_to_dragon 实现
- **RhymeHarmonicPerception** 的韵母遍历与本模块的韵母坐标对接

## 7. 实现约束

- gcd 在 gcd=0 时返回 1 (避免零除)
- saturating_mul 防止三个 255 公约数相乘时溢出 (255³ = 16,581,375 < u64::MAX, 但仍以 saturating 保安全)
- HanziSpace 的 resonance_field 是 O(n²) — 适用于 n < 1000 的小规模文本
- no_std 兼容: 使用 heapless::Vec 替代 Vec

// hunyuan-rs/src/core/njq_matching.rs
