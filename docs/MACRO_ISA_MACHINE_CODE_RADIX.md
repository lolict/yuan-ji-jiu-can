# 机器码容器进制 Macro-ISA — 囡囝共生 · 宏指令=命名容器 · 254+2=256 · 八类容器 · 进位折叠 · 预置指令库

## 1. 定位

机器码进制不是"把机器码编码为数字", 而是"把多条原始机器码绑定到一个名字 = 新的进制位". 多个容器 = 更大的结构 = 更大的"进制位权". 这是满全法在机器码层面的直接应用.

对应 Rust 模块: encoding/macro_isa.rs.

## 2. 核心命题

原始机器码 = 0x48 0x89 0x3C...
多个机器码按顺序绑定 = 一个"宏指令"(容器)
命名这个宏指令 = 新的"进制位"
多个宏指令 = 更大的结构

## 3. 机器码容器 (MachineCodeContainer)

```rust
pub struct MachineCodeContainer {
    name: String,               // 进制位的名字
    code: Vec<u8>,              // 容器内部的原始机器码序列
    metadata: ContainerMetadata,
}
```

ContainerMetadata:
- container_type: ContainerType(8 类之一)
- input_count: 输入参数数量
- output_count: 输出结果数量
- foldable: 是否可折叠

### 3.1 类型自动推断

```rust
fn infer_type(code) -> ContainerType {
    首字节 0x00..0x3F → Addition (加法)
    首字节 0x40..0x7F → Spiral  (螺旋/旋转)
    首字节 0x80..0xBF → Mirror  (镜像/对称)
    首字节 0xC0..0xFF → Fold    (折叠/进位)
}
```

8 类: Addition / Spiral / Mirror / Fold / Color / Shengmu / Yunmu / Tone.

## 4. 机器码进制 (MachineCodeRadix)

```rust
pub struct MachineCodeRadix {
    digits: Vec<MachineCodeContainer>, // 进制位列表
    radix: u16,                        // 254+2=256
    layer: u8,                         // 当前层号
}
```

### 4.1 核心操作

- push(container): 推入一个进制位
- get(index) -> Option<&MachineCodeContainer>: 取指定位置的容器
- emit() -> Vec<u8>: 容器列表→原始机器码序列(连续拼接)
- fold(): 进位折叠(digits.len() ≥ 254 → layer += 1, 返回 true)
- layer(): 当前层号

### 4.2 折叠规则

当容器数量达到 254(满容器阈值, 对应 mlll 254+2 编码), 溢出到下一层(层号 +1). 这与 mlll 254+2=256 的集装箱编码对齐.

## 5. 进制基数 256 = 254+2

```
radix: 256
254 个容器位(max)
+ 2 锚点(起点/终点)
= 256
```

这是 mlll 集装箱编码在宏观机器码层面的直接应用.

## 6. 预置指令库 (preset)

### 6.1 加法容器 container_add

```rust
// x86: ADD register, immediate
code = [0x80, 0xC0 | register, value]
ContainerType::Addition
```

### 6.2 螺旋容器 container_spiral

```rust
// x86: ROL register, angle
code = [0xC1, 0xC0 | register, angle]
ContainerType::Spiral
```

### 6.3 镜像容器 container_mirror

```rust
// x86: XOR register, mask
code = [0x80, 0xF0 | register, mask]
ContainerType::Mirror
```

### 6.4 折叠容器 container_fold

```rust
// x86: CALL rel32
code = [0xE8, lo8, lo16, lo24, hi8]  // 5字节
ContainerType::Fold
```

### 6.5 颜色容器 container_color

```rust
// 自定义编码: 0xCC = Color Container
code = [0xCC, hue, phase, brightness]  // 4字节
ContainerType::Color
```

### 6.6 声母容器 container_shengmu

```rust
// 0x53 = 'S' = Shengmu Set
code = [0x53, position]
ContainerType::Shengmu
```

### 6.7 韵母容器 container_yunmu

```rust
// 0x59 = 'Y' = Yunmu Set
code = [0x59, shape]
ContainerType::Yunmu
```

### 6.8 声调容器

(Tone 类, 编码模式类似 0x?? 前缀)

## 7. 与系统其他模块的接入

- **ISA16(#82)**: isa16 的 16-bit 压缩指令(C 扩展)可原子化嵌入 MachineCodeContainer, 两者是"压缩层 vs 容器层"
- **ContainerRadix(#80)**: 容器进制(6位)的结构 = MachineCodeRadix 每个 digit 内的 code 数组(字节码)
- **Container Trinity(#67)**: ContainerValue 极性 → ContainerType(阴加成法/阳增镜像/元增折叠)
- **SMRL(#4)**: MachineCode 本身作为数字工具载体使用(mirror + 容器 = 机器码经容器化)
- **Position Base(#58)**: 容器位置的进制选择 = MachineCodeRadix 的 digit 层号 × radix

## 8. 实现约束

- 254 满位阈值固定(对应 254+2 编码)
- emit() 将所有容器原始字节连续拼接, 不做重定位; 重定位/符号解析由上层 elf_emitter.rs 处理
- fold() 只累加层号, 不保留历史容器(折叠后 digit 仍保留在 Vec)
- preset 容器中的硬编码 x86 操作码用于演示/占位; 实际跨平台需用 ISA16(RISCV) 或 WASM 目标重建
- 用户自定义容器命名调用 MachineCodeContainer::new, 自动校验 foldable(default=true) 和 output_count(default=1)

// hunyuan-rs/src/encoding/macro_isa.rs
