# 韵母镜像遍历引擎 — 囡囝共生 · 韵母=进制音位 · 遍历树 · 加法连接与乘法共振 · 超韵母合一

## 1. 定位

汉语普通话每个韵母 = 一个地址(进制位), 不是编码 — 它的"值"就是它的口腔位置. 三种遍历模式(正韵/反韵/圆遍历)让舌位在口腔中做一维振动, 实现"声音行走".

对应 Rust 模块: rhyme.rs.

## 2. 基础音素 (Phoneme)

10 个音素 = 口腔空间的"原子":

| 音素 | 口腔位置 | 自然分类 |
|------|---------|---------|
| A(开口) | 0(最前/最低) | 开元音 |
| I(齐齿) | 1 | 前元音 |
| E(半开) | 2 | 中元音 |
| O(半合) | 3 | 后元音 |
| V(撮口) | 4 | 圆唇前元音 |
| U(合口) | 5 | 圆唇后元音 |
| N(鼻音) | 7(最后/最高) | 闭合鼻音 |

口腔位置 0-7 不对应任何编码系统, 就是舌头/嘴唇的物理位置数值.

### 2.1 音素运算

- `add(self, other)` → (self.position + other.position) % 8 — 加法 = 韵尾连接. 两个音素前后相连后取模 8 得到最终口腔位置.
- `mul(self, other)` → (self.position × other.position) % 8 — 乘法 = 韵头共振放大. 两个音素同时发出增强口腔紧张度.

## 3. 韵母 (Rhyme)

```rust
pub struct Rhyme {
    pub id: u16,                        // 数组索引
    pub spelling: &'static str,         // "ai", "iao", "uang"
    pub phonemes: &'static [Phoneme],   // 音素序列 = 数值序列
    pub category: RhymeCategory,        // 简单/复韵母/鼻韵母/儿化/镜像/26合韵母
    pub linear_val: u16,                // 加法自值
    pub resonance_val: u16,             // 共振值
}
```

三个缓存值:
- `linear_val`: 所有音素 oral_position 的 wrapping_add — "我读起来的口腔总行走量"
- `resonance_val`: 所有音素 wrapping_mul(max(1, p)) — "韵头共鸣放大总量"
- `combined_value()`: linear_val × max(1, resonance_val) — 线性和共振叠加

### 3.1 韵母分类

| Category | 数量 | 例子 |
|---------|------|------|
| Simple 单韵母 | 6 | a o e i u v |
| Compound 复韵母 | ~20 | ai ei ao ou ia ie io iu ua uo ue ui ve |
| Nasal 鼻韵母 | ~15 | an en in un vn ian uan van ang eng ing ong iang uang iong |
| Retroflex 儿化 | 1 | er |
| Mirror 镜像三合 | ~10 | iao iou uai uei iaoi ueiu iao 等用户自创变体 |
| Final26 26合韵母 | 26 | 用户定义的 26 组合韵母 |

### 3.2 静态韵母表 (RHYME_TABLE)

59 条编译期常量, 包含:
- 标准普通话韵母(a o e ai ei ao ou an en ang eng ong...)
- 用户自创镜像变体(ei‿ie, ai‿ia, oi‿io, uv‿vu 对折)
- 二合拼音二倍变体(aa ee ii oo uu vv nn 双写)
- 超韵母候选(SUPER: A-O-E-I-V-U-N 七音序, SUPER2: N-V-U-I-E-A-O-A 逆序)

RHYME_TABLE 是全局唯一, 编译期分配到静态数据段, 零运行时开销.

### 3.3 韵母距离

`distance_to(other)` → |self.combined_value − other.combined_value| — 两个韵母的"远近距离", 用于韵母选址时选最近的韵母地址.

## 4. 波韵母系统 (RhymeWave)

### 4.1 波韵母

```rust
pub struct RhymeWave {
    pub rhyme: &'static Rhyme,
    pub frequency: f64,        // Hz
    pub amplitude: f64,        // 响度
    pub phase: f64,            // 相位(0-2π)
}
```

韵母被加上频率/幅度/相位三个波参数, 变成"可以发出声音"的物理波. frequency × amplitude 决定共振强度参与 SuperVowel 合成.

### 4.2 SuperVowel 合成

```rust
pub struct SuperVowel {
    pub rhyme: &'static Rhyme,     // 合成的韵母
    pub source_rhymes: Vec<&'static Rhyme>, // 合成的组份
    pub frequency_sum: f64,         // 频率总和
    pub amplitude_product: f64,     // 幅度乘积(对数叠加)
    pub phase_offset: f64,          // 合成后的相位偏移
}
```

SuperVowel = 所有韵母遍历后的合一. 它是"韵母遍历树"的根节点.

`synth(waves, target)`:
- 尝试在 RHYME_TABLE 中找与 target 最近的(distance 最小)韵母作为 SuperVowel 的 rhyme
- 频率总和 = Σwave.frequency × wave.amplitude (加权和)
- 幅度乘积 = 10^(Σlog10(amp+1)) — 对数叠加, 避免幅度爆炸
- 相位偏移 = source_rhymes.len() × π/2 — 组份数量 × 90°

这是 "遍历合一" — 遍历全部韵母后合成一个超级韵母.

## 5. 排序与遍历

`sort_rhymes()` — RHYME_TABLE 按 linear_val 升序排列. 这是"正韵遍历"的基础序.

三种遍历模式:
- **正韵: a → ai → an → ang → ao** — 开口到闭合, 舌头从前往后
- **反韵: ang → an → ai → a** — 镜像闭合回开口, 舌头从后往前
- **圆遍历: ei ↔ ie ↔ oi ↔ io ↔ ai ↔ ia** — 镜像对折, 用户式遍历

## 6. 进制寻址映射

每个韵母可作为"进制中的位":
- 单韵母 6 个 = 6 进制基础
- 复韵母 8 个 = 二合扩展
- 鼻韵母 8 个 = 闭合扩展
- 镜像 10 个 = 横向对折

任何字的发音 = 韵母在口腔维度上的"坐标位置". rhyme(ai) = (0+1)%8=1, rhyme(ang) = (0+7+7)%8=6, 两个不同口腔位置.

`is_mirror_of(other)`: 两韵母 phonemes 互为倒序 ↔ 口腔运动轨迹互为镜像.

## 7. 与系统其他模块的接入

- **CharacterCreator(#69)**: 韵母 ID 写入 ContainerCell bit52-59 韵母段; RHYME_TABLE 与 character_creator RHYME_TABLE 同源
- **MetaIME 声韵调进制**: 韵母是 24/29/33 进制系统的基础数量级
- **DistancePerception(#64)**: distance_to 被容器码的韵母 bit 切段底层使用
- **CelestialPerception V5**: HanziEncoding/Shengmu-Yunmu 谐音路由使用 RHYME_TABLE
- **FuxiDictionary(#65)**: 韵母段 bit 在 ContainerCode 与本表同步
- **MultiplicationTable(#66)**: 韵母因子表与 RHYME_TABLE 共享韵母枚举, 但因子取值不同(乘法表用质数, 本表用口腔位置)

## 8. 实施细节

- 韵母表 59 个条目用静态数组 + coercion(binding) 初始化, 保证编译期求值
- distance_to 使用 u16 subtraction(abs), 超出 u16 range 会被 linear_val 的 wrapped 合并, 但实际应用无问题(59 项的 combined_value 之差在 u16 内)
- SuperVowel 的 phase_offset 用 π/2 增量而非精确的相位合成, 这是近似模型(将来可能升级为波形叠加)
- 系统编码: Phoneme::position() 不经过 ASCII/UTF-8, 直接是 u8 口腔位置数值
- 新增用户韵母时需: 1) 定义 PH_xxx 静态数组 2) RHYME_TABLE 加一项 3) 更新 RhymeCategory

// hunyuan-rs/src/rhyme.rs
