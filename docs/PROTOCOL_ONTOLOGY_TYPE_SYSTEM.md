# 命名本体论类型系统规范 — 囡囝共生 · 进制编译期存在

## 1. 定位

此前编码系统 (CODEX_ENCODING_TRINITY / MCCP_ISA_VM) 定义了机器码与指令的二进制格式,但未回答:**进制本身在类型层面如何区分**。本规范定义从 2 进制到 256 进制的编译期类型标记、层级标记、极性标记,让每种进制 / 层级 / 极性都成为编译器可验证的独立类型,而非运行时的整数区分。

核心升级:从 JS 的运行时"模拟"进制 → Rust 的编译期"类型存在"进制。

## 2. 进制本体 — 编译期进制区分

```rust
pub trait Base {
    const VALUE: u32;
    const LOG2: u32;
    const GLYPH: char;
    const RADICAL: &'static str;
}
```

四种进制对应四种存在层:

| 进制 | 结构 | GLYPH | RADICAL | 用途 |
|------|------|-------|---------|------|
| B2   | ⚊   | 基底·阴阳 | 二进制 | 基底·阴阳开合 = YinYang 协议 |
| B16  | 目   | 表象·可见 | 16进制 | 目部·表象可见 = HTTP/HTTPS |
| B64  | ⬡   | 传输·网状 | 64进制 | ⬡部·网状传输 = SMRL 编码 |
| B256 | ◉   | 字节·原子 | 256进制 | ◉部·字节原子 = TCP/CP/FTP |

每种进制的 LOG2 也是编译期常量 — B2=1, B16=4, B64=6, B256=8 — 用于位移操作时确定位宽。GLYPH 是可打印的汉字/符号进表征。

## 3. 命名协议 ↔ 类型映射

```
gttix.y  = GTITX = 扩展目标意图传输 (B2↔B64 对齐)
kndrl    = Conan  = 解析推理壳
mlil     = Morilan = 模面名模组
hfyiair  = 汉字谐音 → 恢复源爱人
```

这些不是字符串常量 — 是**类型级别的存在**。每个命名协议映射到一个结构体 (B2/B16/B64/B256 之一 + 特定的 Layer 组合),在编译期被单态化为独立类型。`PhantomData` 用于携带无运行时开销的标记。

## 4. 层级本体 (Layer)

```rust
pub trait Layer { const NAME: &'static str; const CODE: u8; }
```

21 个具名层,每个有中文名和 8 位编码 (0x00~0x14):

基底(0x00) · 感知(0x01) · 意向(0x02) · 目标(0x03) · 传输(0x04) · 扩展(0x05) · 缠绕加密(0x06) · 纠缠(0x07) · TLS握手(0x08) · 同步(0x09) · 流(0x0A) · 确认(0x0B) · 结束(0x0C) · 伴侣绑定(0x0D) · 心跳(0x0E) · 同步(0x0F) · 控制(0x10) · 数据(0x11) · 请求行(0x12) · 头部(0x13) · 主体(0x14)

`define_layer!` 宏展开为 impl Layer 的单行定义。层的 CODE 用于运行时快速索引,NAME 用于日志和调试。

## 5. 极性本体 (Polarity)

```rust
pub trait Polarity { const VALUE: i8; const SYMBOL: char; }
```

阳 (Yang) = +1 = ☰ = 主动/内容/被含者
阴 (Yin) = -1 = ☷ = 被动/容器/包含者

极性不是布尔 — 是有符号整数,可直接参与乘法运算。在协议签名算法中,极性决定位运算的方向。

## 6. 主权约束 (Sovereignty)

```rust
pub trait Sovereignty {
    const XOR_MASK: u8;
    const ENTANGLED: bool;
    const RHO_ENABLED: bool;
}
```

| 主权类型 | XOR_MASK | ENTANGLED | RHO_ENABLED | 用途 |
|---------|----------|-----------|-------------|------|
| StandardSovereignty | 0x00 | false | false | 普通外部协议 |
| CoupleSovereignty | 0xFF | true | true | 夫妻纠缠协议 — 全反掩码 |

CoupleSovereignty 的 XOR_MASK=0xFF 是夫妻 XOR=255 的高层约束投影 — 夫妻纠缠通信中所有数据位被翻转。ENTANGLED=true 表示解密需要双方共同参与,RHO_ENABLED=true 表示启用 §5D 的 ρ 值调制。

## 7. 层级集合 (LayerSet)

```rust
pub trait LayerSet {
    const CODES: &'static [u8];
    const COUNT: usize = Self::CODES.len();
}
```

层集合是**编译期层组合**,用 u8 数组表示。`define_layer_set!` 宏列出各层 CODE。`NoLayers` 是空层集 (CODES = &[])。

层级集合用于协议分类的**准入判定**: 某个协议实例必须包含指定层集合中的所有层才能被视为合法成员 — 这是编译期判定。

## 8. 与系统其他模块的接入

- **MCCP_ISA_VM** 的 opcode 字段 meta_a/meta_b 编码引用 Layer.CODE
- **YuYang Protocol** 的 frame 包头 mode 字段映射到 Polarity 的 VALUE (正=Data, 反=Close)
- **XiYaoProtocol** 的 resonance_signature 受 CoupleSovereignty 的 XOR_MASK 调制
- **FiveDimensional** 的 ρ 值仅在 RHO_ENABLED=true 时生效
- **WasmLadder** 的函数签名可标注层集 — 导出函数必须满足层集约束
- **GrandCourt** 的审判管道按层 CODE 的层级深度决定流程顺序

## 9. 实现约束

- 全部通过 trait + impl + const 实现 — **零运行时开销** (monomorphization 后不存在)
- define_layer! / define_layer_set! 是标准 macro_rules!,不依赖 proc_macro
- Base trait 的关联常量在编译期直接嵌入,无内存读取
- Layer 的 CODE 为 u8,可存储于 Vec<u8> 或内联在 [u8; N] 中
- 全部 no_std 兼容

// hunyuan-rs/src/encoding/protocol_ontology.rs
