# glyph-container · 容器字体统一架构工具集

hunyuan-os 容器字体操作系统的 Python 参考层 + 数据生成器。

## 快速开始

```bash
# 1. 检查协议自检
python3 protocol.py
# 预期: [OK] × 10, "所有自检通过"

# 2. 生成完整 64 容器字体
python3 generate_full_font.py
# 预期: han_yuan64.svg (45KB), container64_tests.rs

# 3. 冰箱调度执行闭环
python3 generate_font.py         # 先生成 8 样本
python3 fridge_executor.py       # 读 SVG → 建构时间表 → 裁决 → 发射
```

## 文件说明

| 文件 | 用途 | 引用 |
|------|------|------|
| `protocol.py` | 满全法编码参考层 (64容器/极性/主权/三层) | hunyuan-rs/src/encoding/container_radix.rs |
| `generate_font.py` | 8 样本 SVG 字体生成 | encoding/z-api.txt |
| `generate_full_font.py` | 完整 64 容器 SVG 字体生成 | container_radix.rs + glyph-3d-font SKILL |
| `han_yuan_container.svg` | 8 样本输出 | — |
| `han_yuan64.svg` | 64 容器完整字体 | — |
| `fridge_executor.py` | 冰箱调度: 读 SVG → Z 轴排序 → 三元裁决 → 发射 | cross_isa/fridge.rs |
| `container_demo.html` | 浏览器可视化 (64 容器网格 + 详情面板 + 时间轴) | — |
| `container64_tests.rs` | hunyuan-rs 测试模板 (64 个 round-trip) | cross_isa/glyph_3d_font.rs |
| `interface-alignment.md` | 三子系统接口冲突分析 | 三个 SKILL.md |
| `alignment-proof.md` | 九项初心对齐证明 | hunyuan-methodology + njsg-ontology |

## 协议架构

```
32 位地址空间 = [容器群:2 + 容器索引:4 + 极性:2 + 主权:2 + 操作数:8 + 时间窗:8 + 层级:6]
             → 分解为 ContainerAddr { group, index, polarity, sovereignty, operand, time_slot, depth }

三层协议 (对齐 glyph-container 三个 Skill):
  Layer 1 ContainerByteCode  → machine-code-container    → 容器字节码 + witness
  Layer 2 ScheduleEnvelope   → time-fridge-execution     → 调度信封 (16B, pack_id + 容器地址 + 对齐)
  Layer 3 GlyphRecord        → glyph-3d-font             → GRPH 表记录 (独立表, 不隐写 glyf)

闭环流程:
  protocol.py (编码) → generate_font.py (包装入 SVG) → fridge_executor.py (解码+调度+执行)
                                  ↓
                           container_demo.html (可视化验证)
```

## 满全法约束

- [x] 所有命名是容器语义名 (袋/裁/满/囡/夕), 不是 uint 位宽
- [x] 进制通过加法循环实现, 不直接写 `×16`
- [x] 不使用工业 typedef (uint8_t/uint16_t/uint32_t)
- [x] 无十六进制常量裸出现
- [x] 无乘除运算符
- [x] 极性 = 夫妻二元 (YIN=妻/0, YANG=夫/1, NEUTRAL=见证/0.5)
- [x] 主权 = 包含关系 (ACTIVE=包含他人, PASSIVE=被包含, ACTIVE_SUB=从属, PASSIVE_SUB=受约束)
- [x] 三元裁决 (空袋拒绝/全零陷阱/数值准入)

## 浏览器演示

打开 `container_demo.html` 可看到:
- 64 容器网格, 颜色按拓扑类型区分, 透明度按主权区分
- 点击容器显示详情 (编码公式/极性语义/主权描述/时间槽/Z轴深度)
- Z 轴时间轴将所有容器按时间槽排列
- 裁决栏显示每容器的三元裁决结果

## Rust 集成

`container64_tests.rs` 可直接粘贴到 `hunyuan-rs/src/cross_ia/glyph_3d_font.rs` 的测试模块。
`protocol.py` 与 `hunyuan-rs/src/encoding/container_radix.rs` 完全对齐, 可直接作为编码参考。

## 引用源

- hunyuan-rs/src/encoding/container_radix.rs — 64 容器枚举
- hunyuan-rs/src/cross_isa/glyph_3d_font.rs — 拓扑路径生成
- hunyuan-rs/src/cross_isa/fridge.rs — ContainerPack + witness
- encoding/z-api.txt — 48 声母 + 108 韵母音标全集
- encoding/symbols_list.txt — 复合韵母 Unicode 映射
- skills/hunyuan-methodology/SKILL.md — NJSG 操作系统方法论
- skills/njsg-ontology/SKILL.md — 容器本体论
- skills/glyph-3d-font/SKILL.md — 字体三维投影
- skills/machine-code-container/SKILL.md — 机器码容器封装
- skills/time-fridge-execution/SKILL.md — 时间触发执行
