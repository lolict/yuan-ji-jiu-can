# YIN-YANG 夫妻进制映射 — 囡囝共生 · 我=1/妻=0 · 加法统摄律 · 双锚主权 · 32-bit 容器 · 不进位加法

## 1. 定位

私有进制 = 去除外壳的量化核心. "我"(lolict) = 1 (主动极); "刘楚坦"(配偶) = 0 (锚定极). 不是计算机科学, 是满全法私有进制.

对应 Rust 模块: encoding/yinyang_binary.rs.

## 2. 核心定义(不可更改)

我 = lolict = AnchorBit::LoLict = 1 (主动)
妻 = 刘楚坦 = AnchorBit::ChuTao = 0 (锚定)

flip() = 取反, 配偶互换.

## 3. 加法统摄律

| + | 0(锚定) | 1(主动) |
|---|---------|---------|
| 0(锚定) | 0(纯锚定,不动) | 1(锚定召唤主动) |
| 1(主动) | 1(主动依附锚定) | 10(夫妻合一,升维) |

OR 而非 XOR → 主从和合, 非相互抵消. 1+1=10 表示夫妻合一产生新维度(进位).

## 4. 夫妻数 (YinYangNumber)

```rust
位布局(每字节):
  bit7 = 锚点(1=我发起, 0=妻发起)
  bit6..0 = 加法统摄值(7位)

value() = 所有字节低7位按大端序组合 → u64
anchor() = 首字节高位
```

### 4.1 构造

from_value(value, anchor): 逐 7bit 分解值域 → Vec<u8> → 首字节加锚点.

zero() = 锚定为刘楚坦(vec![0]); one() = 锚定为我(vec![0x80]).

### 4.2 加法统摄(absorb)

```rust
absorb(self, other) → from_value(self.value + other.value, LoLict)
absorbed_by(self, other) → from_value(self.value + other.value, ChuTao)
```

absorb: 我吸纳你的值, 主权保留为我. absorbed_by: 你吸纳我的值, 主权保留为你.

### 4.3 夫妻双锚加法

spousal_add → 结果有两个锚头字节: [首字节=0x80(我), 次字节=0x00(刘楚坦), 后续=和值]. 两个主权同时标记.

anchor_len(): 双锚检测(首字节高位1 + 次字节高位0) = 2, 否则 = 1.

## 5. 32-bit YIN-YANG 容器

```rust
bit31   = 主权锚 (1=我, 0=刘楚坦)
bit30   = 极性 (1=正, 0=负) 
bit29   = 进位位 (上一步加法是否溢出)
bit28..0 = 29 位值域 (最大 0x1FFFFFFF = 536,870,911)
add(other) → Option<Self>: 值域相加,超 bit28 → None(溢出)
shift_left(n) → 31bits 内有效(移位过程中丢弃溢出位) → 不进位
inv() → 按位取反(补数)
inv_add_29(a, b) → 不进位加法 = a+b, 大于 2²⁹−1 时留在 0x1FFFFFFF
```

32 位容器是 YinYangNumber 的紧凑单字版本 — 适合嵌入 u32 容器码.

## 6. 与系统其他模块的接入

- **Soul(#49)**: 夫妻二元锚定 → SoulState 的 wife/husband bit
- **Binary(#50)**: YinYang 二进制帧协议的底层 bit 语义对齐
- **Couple(#72)**: 夫妻双方主权分别存储, 非零和博弈
- **ContainerRadix(#82)**: 六位 bitmask + 极性 ↔ YinYang bit31 主权 + bit30 极性

## 7. 实现约束

- 加法不进位(OR 语义)是核心语义, 不能改为 XOR
- 溢出时 shift_left 丢弃溢出位(不进位到其他字节), 数学上等价于 mod 2^29
- anchor_len 检测依赖位模式(0x80 + 0x00), 若零值填充可能被误判
- no_std: Vec<u8> 在分配后不再扩容(由 from_value 一次性分配)

// hunyuan-rs/src/encoding/yinyang_binary.rs
