# MQF WASM 内核 · 阶梯制 · 双向公约

## 概述

满全法（MQF）操作系统的可移植执行基底。同一份 C 源码同时编译为四种目标，通过固定宽度类型和统一宏前缀消除架构差异。

```
               同一份 C 源码
                    │
        ┌───────────┼───────────┐
        ▼           ▼           ▼
    kernel_64   wasm32     wasm64
    (本地 x64)  (浏览器)   (未来)
```

## 设计原则

| 原则 | 实现 |
|------|------|
| 固定宽度类型 | `uint32_t` / `uint64_t`，禁用 `int`/`long` |
| 统一宏前缀 | `MQF_` 全部大写，按 `MQF_BITWIDTH` 编译期切换 |
| 架构无关容器 | 坐标/极性/主权/价值编码通过宏派发，不依赖目标字长 |
| 跨会话继承 | 编译产物可纳入版本控制，二进制一致 |
| 阶梯制·双向公约 | 同一源码 · 32/64位 · 两个平台 · 一套规则 |

## 文件结构

```
wasm_kernel/
├── mqf_wasm.h           # 头文件：类型定义、宏派发、API 原型
├── mqf_container.c      # SMRL 引擎核心（容器操作 + 物理公理层）
├── mqf_bridge.c         # 域桥接层（默认域规则 + 容器服务 API）
├── 阶梯构建器.nim       # Nim 构建脚本，输出四目标矩阵
└── README.md            # 本文档
```

## 编译矩阵

| 目标 | 命令 | 状态 |
|------|------|------|
| kernel_64 | `clang -m64 -DMQF_BITWIDTH=64 -o kernel_64 mqf_container.c` | ✓ |
| kernel_32 | `clang -m32 -DMQF_BITWIDTH=32 -o kernel_32 mqf_container.c` | 需32位头文件 |
| wasm32 | `clang --target=wasm32 -nostdlib -c mqf_container.c` | ✓ |
| wasm64 | `clang --target=wasm64 -nostdlib -c mqf_container.c` | ✓ |

## WASM 导出符号

```c
mqf_container_init      容器构造（坐标，极性）
mqf_distance            测距
mqf_consume             吃（能量消耗）
mqf_merge               合一（极性判决 → 吸引/排斥）
mqf_settle              沉淀（能量积累）
mqf_drive               驱动（能量转换坐标进位）
mqf_cycle               推进周期
mqf_judge_polarity      极性判决（吸/斥）
mqf_container_energy    查能量
mqf_container_coord     查坐标
mqf_container_phase     查相位
mqf_container_layer     查层级
mqf_container_valid     有效性检查
mqf_bridge_init         注入域规则
mqf_bridge_domain_const 域常量加载
mqf_bridge_judge        容器极性判决（桥接入口）
mqf_bridge_relate       完整生命周期（测距→吃→合一→沉淀→驱动）
```

## 容器字段

```c
typedef struct {
    mqf_cell_t  coord;     // 单值线性坐标（相位│层级│行│列 一体编码）
    uint8_t     polarity;  // 0=阴(刘楚恬)  1=阳(我)
    uint8_t     phase;     // 0-8 九宫相位（Z 层投影）
    uint8_t     layer;     // 1-9 层级（一元到九元）
    uint8_t     reserved;  // 对齐填充
    mqf_cell_t  valence;   // 量化值
    mqf_cell_t  energy;    // 推动力
} mqf_container_t;
```

## 物理公理层

```
测距 → 吃 → 合一 → 沉淀 → 驱动
 │      │      │       │      │
物质  消除   距离    公约数  公倍数
=时间 空间  归零    刻度    推动力
```

## 运行构建器

```bash
nim c --cc:clang -r 阶梯构建器.nim
```

输出四目标 PASS/SKIP 矩阵。

## 实现注意事项

- wasm 链接器（wasm-ld）未安装时只能编译对象文件，不能生成完整 wasm 二进制
- 可通过 `apt install lld-18` 或 `apt install wabt` 补充 wasm 链接工具
- 在浏览器中运行需 JS 胶水层调用导出的容器裁判 API
