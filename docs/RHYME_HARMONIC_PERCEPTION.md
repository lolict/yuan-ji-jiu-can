# Rhyme Harmonic & Four-Stage Perception · 韵母谐音与四阶感知引擎

> 版本 v1.0 · 2026-09-19 · 声母遍历韵母 · 韵即地址
> 关键词:韵母进制 · 超韵母 · 镜像遍历 · 四阶感知 · 声母遍历韵母 · DiffMarker · 加法位置

---

## 1. 定位

NJSG 容器体系的一个核心操作:**A 容器呼唤 B 容器**。呼唤路径不是 IP 路由,是**韵母路径**。

```
A 的韵母 = a
B 的韵母 = an
路径 = a → ai → an (正韵:开口→闭合)
```

韵母 = 地址。不是编码——它的"值"就是它的发音位置。

---

## 2. 韵母进制(Rhyme Radix)

### 2.1 不是编码,是进制

传统:每个符号有固定编码(Unicode / GBK)。
韵母进制:韵母 = 进制位,位置即值。

```
韵母 a   = 位 0 (最开)
韵母 o   = 位 3 (半合)
韵母 e   = 位 2 (半开)
韵母 i   = 位 1 (齐齿)
韵母 u   = 位 5 (合口)
韵母 ü   = 位 4 (撮口)
韵母 n/ng= 位 7 (鼻音)
```

### 2.2 10 个基础音素(Phoneme)

| 音素 | 名称 | 位置 | 口诀 |
|------|------|------|------|
| A | 开口 a | 0 | 最开 |
| I | 齐齿 i | 1 | |
| E | 半开 e | 2 | |
| O | 半合 o | 3 | |
| V | 撮口 ü | 4 | |
| U | 合口 u | 5 | |
| N | 鼻音 n/ng | 7 | 最闭 |

空位 6 — 保留(满全法:位 6 留作"袋子"位,不分配音素)。

### 2.3 音素的加法与乘法

```
加法: position(a) + position(b) mod 8
  = 韵尾连接(音A + 音B = 新的口腔位置)
  = a(0) + o(3) = 3 → O
  = 开口 + 半合 = 半合

乘法: position(a) × position(b) mod 8
  = 韵头共鸣放大
  = i(1) × u(5) = 5 → U
  = 齐齿 × 合口 = 合口(共鸣放大)
```

**只有加减**。乘法仅在"共鸣"语义下使用,不用于核心数值运算——乘法在满全法核心中被视为"放大",只用于编译器后端。

---

## 3. 三种韵母遍历

### 3.1 正韵(Forward)

```
a → ai → an → ang → ao
开口 → 齐齿 → 前鼻 → 后鼻 → 开口圆唇
```

方向:从开到闭合。对应容器 A 呼唤容器 B(A 开始,B 是目标)。

### 3.2 反韵(Backward / Mirror)

```
ang → an → ai → a
后鼻 → 前鼻 → 齐齿 → 开口
```

方向:从闭合回开口。对应 B 的反馈返回 A。

### 3.3 圆遍历(Circular / Mirror-fold)

```
ei ↔ ie ↔ oi ↔ io ↔ ai ↔ ia
```

镜像对折,双向迂回回路。对应迂回反馈(见 CONTAINER_DYNAMICS §5.4)。

---

## 4. 超韵母(SuperVowel)

所有韵母遍历后的合一:
```
遍历 a → ai → an → ang → ao → ... → ün → un
所有韵母的并集 = 超韵母 = 所有口腔位置的统一 = 合一

超韵母 ≠ 某个实际存在的韵母
超韵母 = 所有韵母的"公约数"(共同基底)
```

设计用途:
- 候选排名中的"默认保底韵":找不到精确实,回退到超韵母
- GTT(天庭感知总线)的全局广播地址 = 超韵母
- 容器之间的"泛呼唤": /need resource 广播给所有容器 = 超韵母地址

---

## 5. 四阶感知引擎(Four-Stage Perception)

### 5.1 四阶链路

```
迂回反馈  →  反射感知  →  判断之力  →  评价之力
(Ripple)     (DiffMarker)   (RelationAnchor)  (EvalRecord)
```

四阶 = 一次完整的反馈传播过程中经过的四个认知阶段。

### 5.2 各阶产物

**第一阶:迂回反馈(Ripple)**

反馈从 A 出发,经过 C、D 等迂回路径回到 A。原始物理量。

**第二阶:反射感知(DiffMarker)**

比较两个相邻时间戳的反馈差异:
```
DiffMarker {
    from_rhyme_id, to_rhyme_id,
    linear_delta,      // 加法值变化
    resonance_delta,   // 共振值变化
    phoneme_shift,     // 音素位移量(听觉感知到的变化)
}

magnitude = |linear_delta| + |resonance_delta| + phoneme_shift
```

感知 = 觉察变化。无变化 = 无感知。

**第三阶:判断之力(RelationAnchor)**

"这个变化是由谁引起的?"——绑定因果:
```
RelationAnchor {
    cause_rhyme_id,    // 因容器
    effect_rhyme_id,   // 果容器
    diff: DiffMarker,  // 具体差异
    confidence: u16,   // 置信度
}
```

**第四阶:评价之力(EvalRecord)**

"这个关系值得强化还是抑制?":
```
EvalRecord {
    anchor: RelationAnchor,
    weight: i32,           // 正=强化, 负=抑制
    reason: EvalReason,    // Harmony | Dissonance | MirrorPair | ZeroSum
}
```

评价结果 → 反馈回呼唤策略(调整下次路径选择)。

---

## 6. 韵母感知.Y 通道(Y-Channel / 韵通道)

### 6.1 两个值

| 值 | 含义 | 计算 |
|----|------|------|
| linear_val | 加法位置(线性) | 音素 position 直接映射 |
| resonance_val | 共振强度 | 声母 + 韵母共同决定 |

### 6.2 声母进入韵母

声母遍历韵母:
```
声母 b + 韵母 a = ba
声母 b + 韵母 o = bo
声母 b + 韵母 i = bi
声母 b + 韵母 u = bu
声母 b + 韵母 λ = bü
...遍历全部 35 韵母
```

每种组合产生一对 (linear_val, resonance_val):
- linear_val = 韵母开口度(0-7)
- resonance_val = 声母 × 韵母共鸣度

### 6.3 谐音路由

在分别器(Distinguisher)层,谐音路径与耦合路径形成**双重检查**:
```
感知家族 = 韵母空间(声学感知)
分别器路径 = x/y/r/L 四条路径(见 DISTINGUISHER_ROUTING §1.3)

谐音路径(DualPath::Rhyme):仅看韵母空间的耦合
分别器路径(y/L/x/r):看容器内拓扑结构

双重检查:谐音匹配 + 分别器匹配 都 PASS → 放行
任一失败 → 拒绝
```

---

## 7. 负面谐音(Blacklist)

某些韵母组合激活"有害谐音"过滤器:
- `mlqf` → 过滤为"没良钱"
- `mlct` → 过滤为"没良心"
- `dss` → 过滤为"第三声"近似"低死"...

处理流程:
```
输入音节 → 查负面谐音表 → 如果命中:
  1. 在 DiffMarker 中标记 phoneme_shift = 0xFF(无效)
  2. EvalRecord.reason = Dissonance
  3. weight = -1(抑制)
  4. 在候选排名中降序到最底
  5. 不参与见证(witness = None)
```

---

## 8. 在元输入法中的映射

### 8.1 七层编码路与韵母进制的对应

| MetaIME 层 | 韵母进制映射 |
|------------|-------------|
| L1 声母 | 声母 21 → 声母索引 (0-20) |
| L2 韵母 | 韵母 35 → linear_val (0-7) + resonance_val |
| L3 声调 | 1-4 → 声调位 |
| L4 部首 | 部首 214 → radical_container 偏移 |
| L5 语义 | 韵母空间 → 在场域定位(AttendanceRecord) |
| L6 意图 | 六特征中的"语义特征" = 韵母 × 声母耦合 |
| L7 动作 | 候选字上屏/rank 输出 |

### 8.2 排名公式

候选排名 = resonance_count × 10 - divergence_penalty

为什么 × 10? = SHL 3 + SHL 1 = (resonance_count << 3) + (resonance_count << 1)。
避免使用乘法指令,改用移位 + 加法。

---

## 9. 与本系列规范的关系

| 话题 | 主参考 |
|------|--------|
| 分别器相声母空间 | DISTINGUISHER_ROUTING §3 |
| MetaIME 七层编码 | METASIME_LOCAL_INTELLIGENCE §4 |
| 四阶感知之迂回反馈 | CONTAINER_DYNAMICS_FIBER_BUNDLE §5.4 |
| 判断之力/评价之力 | CONTAINER_DYNAMICS_FIBER_BUNDLE §6 |
| 在场登记(语义定位) | BOOTSTRAP_SELF_COMPILER §6 |
| 容器距离 | MORPHISM_ALGEBRA |

---

## 10. 已知限制

1. **声母 × 韵母共振 = 仅经验映射**:当前 resonance_val 是静态表(基于语言学家的经验值)。未来可接入 MetaIME 使用者的实际候选选择数据,动态调整共振表。

2. **七位口腔位置(0-7) vs 实际发音学**:普通话韵母远不止 7 个位置。当前 7 位是简化模型,用于地址对齐。不直接对应 IPA(国际音标)的精确舌位。

3. **负面谐音表需人工维护**:mlqf, mlct, dss 等词表需要随语言演变扩展。当前 utf-8 精确匹配,不支持谐音模糊匹配(如 "miben" 不一定算"美笨蛋")。

4. **加法溢出模 8**:`Phoneme::add` 使用 `% 8`。加法超出位置 7 时回绕到 0。这正确模拟了口腔位置是环状结构(最闭→最开)的语音学事实。

5. **超韵母没有固定编码**:超韵母 = 所有韵母合一,但不是一个具体的线性地址。它只在 GTT 广播和保底逻辑中使用,不可作为单独容器的韵母。

---

*本规范的解释权归 刘楚恬和我 所有 · 归属 hunyuan-os/RhymeRadix 模块*
