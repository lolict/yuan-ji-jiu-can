# zig-stem — 阶梯制·双向鸿约 WASM 内核

满全法(NJSG)WASM 内核的最小可运行示范。

## 目录结构

```
zig-stem/
├── main.c          ← 单一 C 源码 (固定宽度类型,无架构依赖)
├── build.zig       ← Zig 构建脚本 (调度双产出)
├── bootstrap.sh    ← 自举脚本 (下载 zig + 编译 + 验证)
└── README.md       ← 本文件
```

## 设计哲学

### 1. 一份源码,两份目标
同份 C 源码 `main.c` 编译为:
- `wasm32-wasi` — 当前主流, Wasmtime/Node/浏览器 全支持
- `wasm64-wasi` — 未来地址空间 >4GB 时的主力

两份 wasm 共享:
- 同一套 C 源码
- 同一套宏定义 (`mqf_*` 前缀)
- 同一套函数导出 (`mqf_symbol_angle`, `mqf_arc_distance`, ...)
- 同一套五判断语义 (`mqf_judge` 枚举)

### 2. 阶梯制鸿约
- wasm32 = 当下阶梯 (生产就绪)
- wasm64 = 未来阶梯 (保持代码兼容)

当 wasm64 生态成熟时,满全法无需改代码,仅切换 `-Dtarget`。

### 3. 固定宽度类型
`main.c` 显式使用 `mqf_u8/u16/u32/u64`,不依赖平台 `int/long` 的隐式宽度。
这是 wasm32 与 wasm64 兼容的关键前提。

## 使用方法

```bash
cd hunyuan-os/zig-stem
./bootstrap.sh
# 产出:
#   zig-out/wasm32/release/mqf32.wasm
#   zig-out/wasm64/release/mqf64.wasm
```

如果 zig 已安装:
```bash
bash build.sh   # 直接编译双目标
```

## 导出的 wasm 函数

| 函数 | 签名 | 用途 |
|------|------|------|
| `mqf_boot` | `() → i32` | 自举入口 |
| `mqf_symbol_angle` | `(i32) → i32` | Symbol → 角(384°制式) |
| `mqf_arc_distance` | `(i32,i32) → i32` | 两角最短弧 |
| `mqf_time_stage_depth` | `(i32) → i32` | 血缘深度估算 |
| `mqf_is_adjacent` | `(i32×6) → i32` | 6-邻接判定 |

## 与 hunyuan-rs 的关系

- `hunyuan-rs` = Rust 侧的 **完整内核** (topo/rel/angle/smrl/stats)
- `zig-stem`   = WASM 侧的 **最小桩** (未来从 Rust wasm 编译目标的跳板)

未来计划:让 `hunyuan-rs` 本身也编译为 wasm32,与 `zig-stem` 同源异构共存。

## 关于 wasm64

zig 0.13 已将 wasm64 列为实验目标。即使主机当前不支持运行,
产出 wasm64 二进制本身已经是向前兼容的"种子" —— 当宿主就绪时,
满全法直接激活,无需重编译。

## 无外部依赖

`bootstrap.sh` 自带 zig 下载。整个目录可携带迁移,
符合蚕蛹反向寄生壳的"自主部署"策略 —— 宿主只需跑 bash,
无需预装任何工具链。

---

 2026 · 满全法(NJSG) · 囡囝共生元内核
