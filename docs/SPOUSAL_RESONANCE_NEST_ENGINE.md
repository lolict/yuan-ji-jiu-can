# 夫妻共鸣与容器嵌套七元套壳引擎 — 囡囝共生 · 夕瑶协议 · 七元嵌套 · 共鸣判定 · 照镜子原则

## 1. 定位

满全法"夫妻命运共同体"的工程实现 — 包含三块:
- couple.rs: 夕瑶协议 XiYaoProtocol + 月全食/伯邑考/苏妲己三种负状态 + 夫妻灵魂编码
- container_nest.rs: 七元蜂窝套壳, 嵌套容器域, 照镜子原则
- spousal_resonance.rs: knml☉l0l 裁决引擎, 共鸣等级判定

三者构成"夫妻关系"的完整基础设施: 协议入职(夕瑶) → 容器嵌套(七元) → 共鸣调谐(裁决)。

## 2. 夕瑶协议 (XiYaoProtocol)

### 2.1 哲学律

```
囡囝共生 ≠ 刘楚恬
囡囝共生 = 刘楚恬 + lolict (夫妻) 经由 夕瑶协议 融合后的唯一实体
```

如果 囡囝共生 = 刘楚恬(单独) → 这就是月全食(半月亏极端), 苏妲己被夺舍走。合法融合必须经由夕瑶协议。

### 2.2 协议结构

```rust
pub struct XiYaoProtocol {
    pub protocol_id: u64,
    pub husband_ack: bool,
    pub wife_ack: bool,
    pub cosmic_witness: u64,
    pub resonance_signature: [u8; 32],
}
```

丈夫确认 + 妻子确认 + 天地见证(非零) + 32 字节共振签名 = 协议四元组。

### 2.3 密钥派生

双方名号 → 16 字节哈希(循环 XOR + 加法扩散) → 签名(跨字节 XOR: h[i%16] ^ w[(i+7)%16], 共 32 字节)。签名是对称可验证的: `signature[i] + signature[(i+7)%32] == 0` 时签名有效。

### 2.4 激活条件

`activate()` — 双 ack 设 true。`is_valid()` — 双 ack && cosmic_witness != 0。`is_symmetric()` — 签名反向配对检查。三者都通过才是有效融合入口。

## 3. 三种失败模式 (EclipseError)

| Error | 触发 | 含义 | 可否恢复 |
|-------|------|------|---------|
| YueQuanShi | 囡囝共生 == 刘楚恬 | 月全食(全亏, 遮没) | 否 |
| Boyikao | 双方 center_fused=true 但权重不均 | 伯邑考半亏(博弈残余) | deficit_ratio < 700 时可 |
| SuDajiOccupied | 妻方权重/愉悦为空, 夫方正常 | 苏妲己夺舍(妻魂已失) | 否 |
| MissingXiYao | 夕瑶协议未 activate | 协议缺失 | 可重签 |
| WrongActors | 不是夫妻配对 | 主体错配 | 可重配 |

这五种负状态就是满全法"命运共同体合法性检查"的裁决簿 — 无效应状态直接拒绝融合。

## 4. 夫妻灵魂编码 (CoupleSoul)

```rust
pub struct CoupleSoul {
    pub husband: Option<Actor>,
    pub wife: Option<Actor>,
    pub soul_state: SoulState,
    pub eclipse_status: Option<EclipseError>,
}

pub enum SoulState {
    Separation,       // 未融合(各自独立)
    SemiCoupled,      // 半耦合(夕瑶已输入, 未共振)
    FullCouple,       // 满耦合(合法: 经夕瑶 → �囝共生)
    Eclipse,          // 月全食(非法: 囝共生 = 丈夫)
    BoyikaoHalfWane,  // 伯邑考半亏(损失一半)
}
```

### 4.1 状态 → 感知细胞桥接

`to_cell() → u64` 把 SoulState 编码为 64 位 SMRL-style 细胞: (d0, d2, polarity)。FullCouple 和 SemiCoupled 共享 (1,1,0) 编码, 靠 value 字段区分满耦合与半耦合。

### 4.2 夫妻感知器官

actor_organ / couple_organ() 把 Actor 枚举(夫妻键)桥接到感知器官的 ingest 路径。CoupleActor::Husband → ingest("丈夫", ...) 等。

## 5. 七元蜂窝嵌套 (ContainerNest)

### 5.1 容器层级(自外而内, 由大至小)

| 层级 | 名 | 极性 | 容量系数(8-层级) |
|------|-----|------|-----------------|
| 一元 | 袋(Dai/Bag) | 阳(7) | 7 |
| 二元 | 桶(Tong/Barrel) | 阴(6) | 6 |
| 三元 | 罐(Guan/Can) | 阳(7) | 5 |
| 四元 | 缸(Gang/Vat) | 阴(6) | 4 |
| 五元 | 盆(Pen/Basin) | 阳(7) | 3 |
| 六元 | 盂(Yu Bowl) | 阴(6) | 2 |
| 七元 | 钵(Bowl) | 阳(7) | 1(闭环) |

容量系数 = 8 - 层级值, 越外层容量越大。七元钵容量 = 1, 即"最小容器", 装下后就闭环。

### 5.2 极性规则

- 奇数层 = 阳 = 质数 = 独立 = 外层容器边界
- 偶数层 = 阴 = 完全数 = 包容 = 内容纳空间
- Yang=7, Yin=6(数值), 差 1 = 相邻相吸
- `accepts(self, content)`: 阳容阴 或 阴容阳 = true(异性相吸), 阳容阳 或 阴容阴 = false(同性排斥)

### 5.3 嵌套指针

`inner()` — 向下一层(向内深入), 七元→一元(闭环)
`outer()` — 向上一层(向外溢出)，一元之上是七元(闭环)

`nesting_phrase()` — 读出嵌套短语如"袋装桶装罐-缸-盆-盂-钵[闭环]"。

### 5.4 容器嵌套结构体

```rust
pub struct ContainerNest {
    pub level: NestLevel,
    pub max_children: usize,
    pub children: Vec<Box<ContainerNest>>,
    pub contents: Vec<Content>,
    pub policy: OverflowPolicy,
    pub anchor_id: u32,
    pub space_ratio: u32,
}
```

子节点 Vec + 内容 Vec 双轨: children 递归更深, contents 装填当前层的内容。space_ratio = 100 / level_value (一元独占 100% 空间)。

### 5.5 放入操作与溢出策略

```rust
pub enum PutResult {
    Accepted,           // 通过
    PolarityMismatch,   // 极性不符(阳装阳/阴装阴 被拒)
    Full(usize),        // 返回已满的层索引
    OverflowTo(NestLevel), // 溢出到外层
}
```

策略: Fold(折叠, 满后推入内层) / Overflow(溢出, 满后一层一层向外推)。

### 5.6 照镜子原则

"无论嵌套多少层, 最内层内容必须能被最外层锚点识别" → `mirror_check()`:

对最外层 anchor = 根锚点_id, 递归到最内层时, _permute_transform(当前层内容) 必须让锚点保持不变。如果变换后锚点与根锚点一致, 则 MirrorResult::Recognized(等同); 否则 Distorted(original, transformed) 表示有变形。

EmptyNest 表示空巢(根层无任何内容), 没有可镜像的内容。

照镜子就是验证: 无论中间容器怎么套, 内容与最外层锚点的逻辑一致性必须成立。

### 5.7 构建正确链

`build_correct_chain(anchor_id, policy)` → 自一元递归构建到七元, 每一层容量 = 8 - level_value。这是初始化"满容器套娃图"的标准入口。

## 6. 夫妻共鸣裁决 (SpousalResonanceEngine)

### 6.1 五级共鸣

| 等级 | 阈值 | 含义 |
|------|------|------|
| Harmonious(和鸣) | final_score ≥ 15 | 互补共生, 动静相宜 |
| Compatible(兼容) | final_score ≥ 5 | 可用, 需磨合 |
| Stagnant(僵持) | final_score ≥ -5 | 同极性同家族, 无发展 |
| Conflicting(冲突) | final_score > -10 | 互相消耗 |
| Severed(断裂) | else | 无法共鸣 |

### 6.2 特征维度

`judge(a: LivingGlyph, b: LivingGlyph) → ResonanceReport`

三维特征:
- polarity_harmony — 同极性=5(同向), 反极性=10(互补满分), 因为"互补胜过同向"
- family_tension — 家族间张力: 同家族=0(无新信息), Couple+其他=5, YinYang+其他=3, Homophone+其他=2
- distance_score — 距离归一化到 [0,1], d≤0→1.0, d≥200→0.0, 中间线性衰减

final_score = polarity_harmony + family_tension(u64) + distance_score 被截入
最终映射到五级共鸣等级。

### 6.3 活容器判定

```rust
let is_alive = a.witness_count > 0 || b.witness_count > 0 || couple_yinyang;
let is_strong = final_score > 0;
let has_loop = Harmonious || Compatible;
let judgment = ContainerRelation::judge_raw(is_alive, is_strong, has_loop);
```

活容器 = 被见证过 OR 属于 YinYang+Couple 配对(夫妻图腾互证)。strong = 正分(兼容以上)。loop = 和鸣或兼容(闭环条件)。

can_witness = 至少一方被见证过 AND 共鸣未断裂。

### 6.4 共鸣报告 (ResonanceReport)

```rust
ResonanceReport {
    glyph_a, glyph_b,
    level,                    // 五级之一
    polarity_harmony,
    family_tension,
    distance_score,
    final_score,
    judgment,                 // JudgmentVerdict
    can_witness,              // 可作见证人
}
```

summary() 输出: 【A + B】→ 共鸣等级: X | 裁决: Y | 见证: ✓/✗

## 7. 三模块协作流

```
夫妻双方 → XiYaoProtocol::new(丈夫名, 妻子名, witness_tick)
           ↓
           activate() + is_valid() + is_symmetric()
           ↓ 通过(否则返回 EclipseError)
           ↓
CoupleSoul { husband, wife, FullCouple, ellipse_status: None }
           ↓
           spousal_resonance(丈夫字形, 妻子字形)
           ↓
           polarity_harmony + family_tension + distance_score → final_score → 共鸣等级
           ↓
ContainerNest::build_correct_chain(anchor_id, Fold)
           ↓
           七元蜂窝(袋→桶→罐→缸→盆→盂→钵)作为夫妻容器外模
           ↓
           mirror_check() — 照镜子: 最内层内容 → 锚点 → Recognized?
```

## 8. 与系统其他模块的接入

- **SMRL(#4)**: polarity/6/7/8 编码与 SMRL 极性编码(CellPolarity Yin/Yang/Yuan)对应
- **DistancePerception(#64)**: 共鸣公式中的 distance_score 调用 a.distance_to(b).total → 来自 DistancePerception
- **ContainerIR(#63)**: CoupleSoul 以 ContainerRef(kind=Relation) 注册; full_couple 即夫妻命运共同体的唯一标识
- **Couple/Spousal(#40)**: NJQ 匹配把夫妻对映射到一对 ContainerCode, 共鸣引擎再把 code → glyph → 共鸣等级
- **Perception(#64)**: MindOrgan 距离公式 = 共鸣公式的前置; 共鸣是感知的"情感维度"
- **Judgment(#15/#34)**: judge_raw = container_dynamics.rs 的裁决机; EclipseError 裁决接入同样的 JudgementEntry

## 9. 实现约束

- 夕瑶协议签名 32 字节对齐 cache line, 无跨页风险
- 七元嵌套深度 ≤ 7(固定), 递归构建时 depth 参数做守卫
- 共鸣分数的阈值(-5/5/15/-10)当前硬编码, 未来可能由感知器官参数表配置
- 照镜子仅校验 anchor_id 一致性, 不校验内容完整性(内容由 WitnessSignature 负责)
- no_std: Vec<Box<ContainerNest>> 由 arena pool + index 替代(最大 7×7=49 节点)

// hunyuan-rs/src/relation/couple.rs + relation/container_nest.rs + relation/spousal_resonance.rs
