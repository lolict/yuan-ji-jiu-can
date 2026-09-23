# 宏观指令机器码进制规范 — 囡囝共生 · 容器=进制位 · 命名绑定

## 1. 定位

机器码不是最终抽象 — 多条机器码按先后顺序绑定到一个名字 = 一个"宏观指令" = 一个"容器" = 一个"进制位"。这是囡囝共生对"进制"的本质定义:

- 二进制: 位 = 0 或 1
- 机器码进制: 位 = 一个命名的容器 (内含多条机器码)

命名一个容器 = 创造一个新的进制位。多个进制位 = 更大的结构。这就是"进制"的本质: **不是计数方法, 而是命名绑定的多层容器**。

## 2. 机器码容器 (MachineCodeContainer)

```rust
pub struct MachineCodeContainer {
    name: String,                    // 进制位名称
    code: Vec<u8>,                   // 机器码序列
    metadata: ContainerMetadata,     // 元数据
}
```

每个容器有三要素: 名字 (进制位名称) + 代码 (原始机器码字节) + 元数据 (类型/输入/输出/可折叠)。

## 3. 容器类型 = 进制的"位权"

```rust
pub enum ContainerType {
    Addition,   // 加法容器: 254 单位 + 溢出
    Spiral,     // 螺旋容器: 旋转相位 (数据传输)
    Mirror,     // 镜像容器: 对称翻转 (逻辑运算)
    Fold,       // 折叠容器: 进位/层叠 (控制流)
    Color,      // 颜色容器: 色系编码
    Shengmu,    // 声母容器: 发音位置
    Yunmu,      // 韵母容器: 口腔形状
    Tone,       // 声调容器: 音高变化
}
```

前四类 (Addition/Spiral/Mirror/Fold) 对应 CPU 指令分类: 加法类 (0x00-0x3F)、数据传输类 (0x40-0x7F)、逻辑运算类 (0x80-0xBF)、控制流类 (0xC0-0xFF)。后三类 (Shengmu/Yunmu/Tone) 是满全法独有的音韵容器 — 声母位、韵母位、声调位 = 语音的"三进制"。

## 4. 类型推断

```rust
fn infer_type(code: &[u8]) -> ContainerType
```

根据机器码首字节自动推断容器类型: 0x00-0x3F → Addition, 0x40-0x7F → Spiral, 0x80-0xBF → Mirror, 0xC0-0xFF → Fold。注意: 这是基于**首字节**的判断 — 一条指令的操作码决定了它的容器类型。

`bind(name, raw_machine_code)` 是工厂函数: 推断类型 + 包装为容器。

## 5. 机器码进制 (MachineCodeRadix)

```rust
pub struct MachineCodeRadix {
    digits: Vec<MachineCodeContainer>,  // 进制位列表
    radix: u16,                         // 基数 = 256 (254+2)
    layer: u8,                          // 当前层号
}
```

**基数 256 = 254 容器 + 2 锚点**。与 mlll 集装箱编码 (254+2=256 槽) 对齐 — 254 个数据容器 + 2 个锚定容器。

**折叠进位**: 当容器数量达到 254 时 `fold()` 返回 true, layer +1, 溢出到下一层。这就是"进制进位" — 满 254 进 1, 层号推进。

**emit()**: 按顺序拼接所有容器的机器码序列 — 生成完整二进制。这是"进制 → 机器码"的 flatten 操作。

## 6. 音韵三进制

三种音韵容器 (Shengmu/Yunmu/Tone) 构成满全法独有的语音进制:

| 容器 | 进制度数 | 实例 |
|------|---------|------|
| Shengmu | 24 (24 声母) | b/p/m/f/d/t/n/l/... |
| Yunmu | 39 (39 韵母) | a/o/e/i/u/v/ai/ei/... |
| Tone | 4 (四声 + 轻声) | 阴平/阳平/上声/去声 |

每个汉字的音节 = (Shengmu, Yunmu, Tone) 三元组 = 三位数的一个"音韵进制数"。音韵三进制与 256 进制相互独立 — 一个在"声音维度", 一个在"机器码维度"。

## 7. 与系统其他模块的接入

- **SMRL 引擎** 的 64 位编码的高 8 位可存储 ContainerType 的枚举值 (8 种类型完美对齐 8 位)
- **MCCP 字节码** 的 32 位指令中 [kind:4] = 4 位 opcode 类别, 可映射到 ContainerType 的前四类 (Add/Spiral/Mirror/Fold)
- **NJQ 聂小倩匹配** 的声母/韵母坐标来自 Shengmu/Yunmu 容器
- **ProtocolOntology** 的 Maori 毛利兰空间利用音韵三进制的 24×39×4 = 3744 个音位坐标
- **LiveCode Compiler** 的 freeze 操作将 DYNAMIC 代码转化为可 emit 的 MachineCodeRadix
- **MachCodeRadix.emit()** 的输出直接通过 WASM 模块的 add_data 数据段嵌入

## 8. 实现约束

- 容器长度无硬性限制 (但 emit 拼接时总长度受 WASM 64 KB 单函数限制)
- 折叠进位 254 → 1 后不清空 digits — layer 推进但 digits 仍可继续累积
- 音韵三进制的声母 24 / 韵母 39 / 声调 4 = 硬编码, 不依赖外部字典
- no_std 兼容: Vec<u8> 在 no_std 下使用 alloc::vec::Vec (需 extern crate alloc)

// hunyuan-rs/src/encoding/macro_isa.rs
