# 满全法容器编码系统

## 架构总览

```
FQ-ROOT (始祖)
├── FQ-TONE60 ─── 音调类型(60进制 · 6音×5声调)
│   └── FQ-ATONE50 ── 高级音调(100进制 · 5层×10符号)
├── FQ-YM66 ──── 韵母层(66进制)
│   ├── FQ-YM100 ── 韵母-100(小写50+大写50)
│   │   ├── FQ-YM-CMPD ── 复韵母(49符号)
│   │   └── FQ-NASAL ─── 鼻韵母(85符号)
│   ├── FQ-YM-a ──── a韵母族(28变体)
│   ├── FQ-YM-o ──── o韵母族(28变体)
│   ├── FQ-YM-e ──── e韵母族(26变体)
│   ├── FQ-YM-i ──── i韵母族(26变体)
│   ├── FQ-YM-u ──── u韵母族(26变体)
│   ├── FQ-YM-ü ──── ü韵母族(7变体)
│   └── FQ-CLONE ─── 分身(11同形符号)
│       ├── FQ-SPARE1 ── 备用1(70鼻韵)
│       ├── FQ-SPARE2 ── 备用2(29复韵)
│       └── FQ-SPARE3 ── 备用3(31特殊)
├── FQ-SM48 ───── 声母层(48进制)
│   ├── FQ-SM50P ─── 声母-50(含v/V)
│   └── FQ-SPCON ─── 特殊辅音(60符号)
├── Clock-60 ──── 时钟(60进制=秒/分)
├── Color-42 ──── 颜色(42进制=7色×6阶)
├── Space-18 ──── 空间(18进制=6方位+4斜角+8天地)
├── Mirror-10 ─── 镜像(10进制=10符号对射)
└── Music-3 ───── 音乐(3进制=原子/短语/段落)
```

## 核心原则

- **容器化编码**: 每个编码系统作为独立容器存在
- **无中央统治**: 没有单一编码统治所有容器
- **公约数关联**: 跨容器通过gcd自然桥接
- **去中心化**: 权力指数0.143 · 去中心化0.857

## 模块清单

| 文件 | 符号数 | 功能 |
|------|-------|------|
| encoding_container.nim | - | 7容器独立编码系统 |
| categorical_bridge.nim | - | 跨容器同构桥接(12条桥) |
| multi_encoding_society.nim | - | 多编码社会模拟 |
| dynamic_discovery.nim | - | 5维度自主发现同构 |
| cross_query.nim | - | 跨容器统一查询(去中心0.999) |
| fairness_check.nim | - | 公平性验证(权力0.143) |
| symbol_classifier.nim | 497 | 14容器分类(音调/韵母×6/分身/备用×3/辅音) |
| container_special.nim | 230 | 专业版容器(100/50/复韵/鼻韵/辅音/分身) |
| container_dna.nim | - | 四代进化血统树 |
| master_registry.nim | 867 | 22容器元索引 |
| cross_compile.nim | - | 跨容器编译桥(直接/桥接/辐射) |
| compile_demo.nim | - | 4种编译实战演示 |
| full_integration_test.nim | - | 全容器集成测试(5/5 PASS) |

## 进制对照

| 容器 | 进制 | 用途 |
|------|------|------|
| FQ-ATONE50 | 100 | 高级拓扑编码 |
| FQ-SPARE1 | 70 | 视觉匹配备用 |
| FQ-NASAL | 85 | 鼻韵母 |
| Clock-60 | 60 | 时钟 |
| FQ-SPCON | 60 | 特殊辅音 |
| FQ-YM-CMPD | 49 | 复韵母 |
| FQ-SM50P | 50 | 声母扩展 |
| Color-42 | 42 | 颜色 |
| FQ-SM48 | 48 | 基础声母 |
| FQ-SPARE3 | 31 | 特殊音节 |
| FQ-TONE60 | 60 | 音调 |
| FQ-YM-SPARE2 | 29 | 复韵备用 |
| FQ-YM-a | 28 | a韵母族 |
| FQ-YM-o | 28 | o韵母族 |
| FQ-YM-e | 26 | e韵母族 |
| FQ-YM-i | 26 | i韵母族 |
| FQ-YM-u | 26 | u韵母族 |
| FQ-YM-ü | 7 | ü韵母族 |
| Space-18 | 18 | 空间坐标 |
| Mirror-10 | 10 | 镜像对射 |
| FQ-CLONE | 11 | 同形异用 |
| Music-3 | 3 | 节奏层级 |

## 使用

编译单个模块:
```bash
nim c --cc:clang -o:xxx xxx.nim && ./xxx
```

编译集成测试:
```bash
nim c --cc:clang -o:full_integration_test full_integration_test.nim && ./full_integration_test
```

## 提交历史

| Commit | 内容 |
|--------|------|
| 91bb921 | 七层统一社会模拟(声母48+韵母66+时钟60+颜色42+空间18+镜像10+音乐3) |
| 4ecec3f | 容器化编码系统·六模块·无中央统治 |
| 793590d | 符号分类系统·14容器497符号 |
| 26d0f6d | 容器DNA血统树·全容器元索引(22容器867符号) |
| ca22a78 | 跨容器范畴编译桥·编译实战演示 |
| 2a9c992 | 全容器集成测试·22容器5/5 PASS |