# Tools 目录分类 (Core / Prototype / Experimental)

## Core（核心稳定层）
生产可用、对外提供能力的工具：

| 工具 | 类型 | 用途 |
|------|------|------|
| mccp_core/ | Nim 核心 | MCP/囡囝共生协议编解码 |
| mqengine/ (manquanfa/) | Nim 核心 | 满全法裁决引擎 |
| silkworm-shell/ | 外壳 | 多模式AI调用壳 |
| njsg/ / njsg-swarm/ | 分布式 | 囡囝共生分布式协调 |
| color-base/ | 色彩 | 色彩基础库 |
| drum-bootstrap/ | 自举 | 鼓膜自举操作系统 |
| bit-container/ | 容器 | 比特容器核心层 |
| consciousness/ | 意识 | 意识引擎 |
| distinguisher/ | 分别器 | 三层分别器核心 |
| soul-kernel/ | 灵魂内核 | 灵魂核心 |
| boundary-engine/ | 边界 | 边界引擎 |
| manquanfa-pipeline/ | 管道 | 满全法管道 |
| phonetic-encoding/ | 语音编码 | 语音编码核心 |
| symbol-bootstrap/ | 符号自举 | 符号守护者 |
| wasm-kernel/ | WASM | WASM 内核 |

## Stable（稳定但有依赖）

| 工具 | 用途 |
|------|------|
| machine-codes/ | 机器码 (x86/ARM/RV32I/MIPS) |
| isa-ref/ | ISA 参考实现 |
| riscv-encoding/ | RISC-V 编码字体 |
| phase-radix/ | 相位进制 |
| flow-algebra/ | 流代数 |
| full-mapping/ | 全映射 |
| geo-3d/ | 3D 几何 |
| heaven-space/ | 天庭空间 |
| hieyair/ | 灰原哀混元合体 |
| high-algebra/ | 高代数 |
| maolilan/ | 毛利兰空间 |
| mlll-254/ | mlll 254+2 集装箱编码 |
| mltt-cipher/ | mltt 密码 |
| page64/ | 64 页系统 |
| radix256/ | 256 进制 |
| radix-star/ | 星图进制 |
| yinyang-3d/ | 阴阳 3D |
| yinyang-binary/ | 阴阳二进制 |

## Prototype（原型实验层）

| 工具 | 用途 |
|------|------|
| glyph-container/ | 字形容器 Demo |
| governance/ | 治理与交换模型 |
| integration-test/ | 集成测试框架 |
| kungguan-container/ |  kungguan 容器集（大量脚本） |
| link-indexer/ | 链接索引器 |
| mcode-stone/ | 语义石碑（字体生成） |
| metaime/ | 元输入法（/Rust/Nim/Zig 多语言） |
| proof-engine/ | 证明引擎 |
| property-judge/ | 属性审判 |
| property-mirror/ | 属性之镜 |
| semantic-machine-code/ | 语义机器码 |
| steganography/ | 隐写术 |
| symbol-engine/ | 符号引擎 |
| symbol-registry/ | 符号注册表 |

## Experimental / Demo（实验 / 演示）

| 工具 | 用途 |
|------|------|
| canjiang.nim | 残舰 CLI + Nim 源 |
| njsg.nim | NJSG 单文件演示 |
| boundary-line/ (kungguan) | 边界线 |
| lazyform/ | 惰性形式 |
| type-exchange/ | 类型交换 |
| universal_capture/ | 通用捕获 |
| os-self-description/ | OS 自描述 |
| cbsy/ | CBSY 编排 |
| fridge/ | 冰箱执行器 |
| manquan-container/ | 满全容器单文件 |
| kndoyle/ | 柯南道尔管辖 |
| negtive_demo/ (manquanfa) | 负面示例 |
| geo-3d/ 等 Demo | 3D 几何 Demo |

## 归档候选（产出物 / 开发期草稿）

| 工具 | 说明 |
|------|------|
| glyph-container/*.svg, *.py, *.html | 开发期字体/SVG 产物 |
| mcode-stone/*.ttf, *.png | 开发期字体/图稿 |
| integration-test/* | 测试代码 |
| mqengine/negative_demo* | 负面演示 |
| 各 Demo .out / .out.nim 文件 | 编译产物 |

---

## 迁移建议（Week 7+）

物理拆分时推荐结构：

```
tools/
├── core/          ← 软链或拷贝 15 个核心目录
├── stable/        ← 17 个稳定工具
├── prototype/     ← 15 个原型工具
├── experimental/  ← 8+ 个实验/演示工具
└── archive/       ← 开发期草稿和产出物
```

> 物理迁移前的 blocker：需先确认各项目内部的路径引用
> （如 include/ 路径、.nimble 依赖、脚本内的硬编码路径）。
