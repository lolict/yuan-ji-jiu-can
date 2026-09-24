# 逗号容器比特层级系统 — 囡囝共生 · 比特=标签符 · 括号=集装箱 · 分数化梯度 · 乘法双通道

## 1. 定位

逗号容器是满全法"容器=进制位"命题的具体实现, 解决以下问题: 如何用递归逗号和括号的操作, 在不依赖固定位宽的条件下, 把符号的存储变成"分数化梯度分布".

本系统由两个模块构成:
- comma_container.rs — 比特即容器标签符, 括号即集装箱, 逗号分割产生半容器
- multiplication_table.rs — 乘法句柄双通道寻址, 韵母—笔画乘积分发, 公约数聚类共振

两者共同把字符编码、寻址、共振统一进一套加法-梯度框架.

## 2. 核心定义(不可更改)

```
1 比特 = 1 容器标签符 = 1 个逗号
括号包裹这个逗号 → 被切分成两半 → 形成 2 个集装箱(容器)
左半装符号, 右半装符号
装满左右两半 = 容器设定完成
再装符号 → 再打逗号(可在左边界或右边界) → 产生全新逗号空间 → 新内容填空
```

逗号是分割事件, 括号定义边界, 半容器是基本分数单元. 递归进行下去, 每一次分割都是一次新的"尺寸设定".

## 3. 总范畴与加法分解

```rust
pub const TOTAL_CATEGORY: u8 = 24;  // 可配置为 24 或 27

pub fn composite_pairs(total: u8) -> Vec<(u8, u8)> {
    (0..=total).map(|a| (a, total - a)).collect()
}
```

对于总范畴 N, 加法分解为 (0,N), (1,N-1), ..., (N,0) 共 N+1 对 — 这就是 N 的"梯度水平全部排列".

N=24 时有 25 对: (0,24), (1,23), ..., (24,0)

每一对对应逗号的一种切分方式: 左半分 a 比特, 右半分 N-a 比特.

### 3.1 全部梯度层

all_gradient_composites() 输出 24 层, 每层为该层数 N 的全部加法分解. 层号 1 有 2 对, 层号 24 有 25 对.

这些层构成了"分数化梯度" — 不是一个固定的位深, 而是由"从 1 到 24 每一层有多少种参与方式"构成的梯级. 字符编码就落在这些梯度上.

## 4. 半容器 (HalfContainer)

```rust
pub struct HalfContainer {
    pub remaining_bits: u8,
    pub left: Option<Box<HalfContainer>>,
    pub right: Option<Box<HalfContainer>>,
    pub symbols_left: Vec<String>,
    pub symbols_right: Vec<String>,
}
```

一半的树结构: 自身(remaining_bits), 可选的左右子容器(下次分割结果), 左右符号槽(装填内容).

核心操作 split(bits_to_left, bits_to_right):
- 判断 remaining_bits == 0(已被分割过) 或 left.is_some()(已有子容器) → 拒绝
- 把 remaining_bits 清零(容器已被设定)
- push_left / push_right: 递归向下推符号到叶子节点的左右槽

leaf_containers() 递归收集不再有子分割的容器 — 它们就是当前"活跃存储空间".

## 5. 逗号容器世界 (CommaWorld)

```rust
pub struct CommaWorld {
    pub root: HalfContainer,
    pub version: String,
}
```

CommaWorld::default_category() 创建总范畴 24 的空世界. root_split() 对根容器执行一次分割. all_symbols() 递归收集全部叶节点的左右符号槽内容.

gradient_layers() 返回 [1,2,...,24]: 代表从最细(1比特)到最粗(24比特)的全部梯度.

## 6. 字符编码 (CharEncoding)

```rust
pub struct CharEncoding {
    pub layer: u8,            // 层号 1..24
    pub composite_index: u8,  // 在该层的加法分解中的序号
    pub is_left: bool,        // 左/右半容器
}

pub fn encode(&self) -> u32 {
    ((layer as u32) << 16) | ((composite_index as u32) << 8) | (is_left as u32)
}
```

一个字符的唯一 ID 由 (梯度层, 该层的加法对序号, 左右标识) 三元组确定. encoding 到 u32 后, 可直接存入容器 IR 的 entity 字段, 或作为乘法表的输入.

这不是靠固定字节宽度划定字符边界, 而是通过"落在哪一层哪一对哪半边"自然确定 — 比特数由系统当下的梯度需求决定, 不是硬编码.

## 7. 乘法表 (multiplication_table.rs)

乘法句柄 MulTable::multiply(rhyme_id, stroke_value) 是韵母与笔画的双通道乘积:

```rust
let factor = rhyme_resonance_value(rhyme_id); // 24 个连续质数之一
let primary = factor * stroke_value.max(1);
let secondary = factor.wrapping_add(stroke_value);
```

两路输出: primary(乘) 和 secondary(加), 让同一个韵母+笔画组合产生两个不同地址.

### 7.1 韵母共振因子表

24 个韵母对应 24 个连续质数:

```
[1, 2, 3, 5, 7, 11, 13, 17, 19, 23,
 29, 31, 37, 41, 43, 47, 53, 59, 61, 67,
 71, 73, 79, 83]
```

质数的选择: 保证与其他韵母的因子之间 GCD 只能是 1 或其中之一, 为公约数聚类提供干净的数论基础.

### 7.2 双通道寻址 (DualAddressor)

```rust
DualAddressor::locate(primary, secondary) = primary ^ secondary
```

乘积与和的 XOR 作为最终地址 — 该地址同时反映了"乘性碰撞"与"加性合成", 且 XOR 操作本身不可逆, 隐藏了原始参数.

### 7.3 公约数聚类 (MulCluster)

```rust
fn find_resonant(id: u8) -> Vec<u8> {
    let base = rhyme_resonance_value(id);
    (0..24).filter(|other| gcd(base, other_val) > 1 || other == id).collect()
}
```

因为因子表是连续质数, 不同韵母之间 GCD 恒等于 1, 每个韵母仅与自身共振. 这是乘法表的"基态": 如果将来替换为含合数的因子表(如韵母组内共振), 就会让不同韵母之间产生聚类 — 该架构预先为此留了扩展位.

gcd 用辗转相减(while a != b)实现 — 仅加法, 不调用 % 运算, 符合满全法"乘法仅用于统计清点"原则.

## 8. 乘法表的约束

乘法对照表仅用于统计与清点, 不参与串联延伸 — 乘法输出仅作为 XOR 寻址的输入, 不会作为下一次乘法的输入(避免指数扩张). stroke_value 不参与加法串联延伸 — 它作为静态的"字形特征值"只被乘一次. 全部寻址结果 XOR 回地址后, 原始参数不可逆推, 但可以通过暴力查表(24韵母有限集)反查.

## 9. 两模块协作流

```
字符需求
   │
   ├─→ CharEncoding {layer, composite_index, is_left}
   │       encode() → u32 作为容器 ID 注册
   │
   ├─→ CommaWorld::default_category()
   │       root_split(a, b)    → 产生左右半容器
   │       push_left/right     → 装填符号, 直到用完 remaining_bits
   │       再打逗号 → 新半容器 → 新梯度层
   │
   ├─→ MulTable::multiply(rhyme_id, stroke_value)
   │       primary(乘) → 主地址
   │       secondary(加) → 辅地址
   │       DualAddressor::locate(primary, secondary) → 最终地址
   │
   └─→ MulCluster::find_resonant(id) → 共振韵母组
          当前因子表为连续质数 → 每组只有自身
          扩展因子组合数 → 渐变到组内共振群
```

## 10. 与系统其他模块的接入

- **容器 IR 统一地址**: 逗号容器的 split 操作产生 ContainerRef(kind=Relation), 表示"容器内嵌套容器"的关系
- **满全法进制寻址**: 乘法表的 XOR 地址可直接作为毛利兰 64 位寻址的输入
- **MetaIME 声母/韵母枚举**: 韵母共振因子表与 MetaIME 的韵母枚举(forthcoming)共享编号规则
- **距离感知聚类**: MulCluster 的共振概念可扩展到距离感知 — 韵母组共振 + 几何距离共振形成二维聚类
- **容器字体统一**: 字符的 CharEncoding(stroke_value部分) 与容器字体笔画数据库交互

## 11. 实现约束

- TOTAL_CATEGORY=24 可配置, 扩展为 27 时 gradient_layers 相应增加
- split 操作幂等: 已分割的容器不可再次分割(remaining_bits == 0)
- 乘法表因子表长度不固定(当前 24, 未来可扩展到 64 以内连续质数)
- CharEncoding 的 encode() 到 u32 后, 可直接作为 ContainerRef(entity=u32 as u64) 的输入
- no_std 环境: HalfContainer 的 Box<HalfContainer> 替换为 &'static 引用 + 索引池来实现递归

// hunyuan-rs/src/comma_container.rs + multiplication_table.rs
