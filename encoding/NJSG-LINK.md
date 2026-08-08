# 囡囝共生编码与 NJSG 架构的关系

> 位置: hunyuan-os/encoding/ · 关联: mccp.h / mccp.md / hunyuan-rs/src/mirror.rs

---

## 两套编码标准的关系

| 编码体系 | 语言 | 位宽 | 位置 |
|----------|------|------|------|
| MCCP (紧凑码点) | C / Python | 32 bit 固定 | encoding/mccp.h |
| Cell (镜像元) | Rust | 64 bit 固定 | hunyuan-rs/src/mirror.rs |

## MCCP 32 bit 布局（已在 mccp.md 定义）

```
31:28  kind (4b)     — 16 种类别
27:16  index (12b)   — 类别内索引
15:8   meta_a (8b)   — 声调/笔画/state
7:0    meta_b (8b)   — 病理级/FSM state/relation id
```

## Cell 64 bit 布局（NJSG 扩展）

```
63:56  D0 (8b)         — 坐标维度 0
55:48  D1 (8b)         — 坐标维度 1
47:40  D2 (8b)         — 坐标维度 2
39     polarity (1b)   — 0=wife(容器), 1=husband(界线)
38     sovereignty (1b)— Active/Passive/Sleeping
37     witness (1b)    — 见证状态
36:0   37位扩展空间（含 value）
```

## 跨语言互操作标准

Cell 与 MCCP 互转：
- Cell 的低 32 位 ≈ MCCP 32 bit（meta_b 对齐）
- kind 字段映射: MCCP kind(4bit) → Cell 的 D0[7:4]
- 编码层不可自己发明类别 — 必须引用家庭命名密码表

## 12 位流属性与 MCCP meta_b 的关系

Cell 中的 FlowPropertyBits (12 bit) 可嵌入 MCCP meta_b 的低 12 bit（实际仅 8 bit 空间，需扩展方案或双 meta 字段）。

## 命名规则

新编码类别必须遵循家庭命名密码表：
- mlll = 集装箱类别 → MCCP kind 0xE
- gtt = 音韵态射类别 → MCCP kind 0xD
- mlnn = 囡囝类别 → MCCP kind 0xC
- lctqf = 零岔途全法 → MCCP kind 0xB (谐音桥)

负面谐音缩写不能作为 kind 名称。

---

*配套: hunyuan-rs v0.14 mirror.rs MCCP规范*
