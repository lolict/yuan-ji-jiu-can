# 联发科 (MTK) — 基于 ARM 的扩展

> MTK 手机芯片（天玑系列）基于 ARM 公版架构，指令编码与 ARM 一致
> 本文件只记录与 ARM 公版的差异和专有扩展

---

## 1. 基础 ISA

- 天玑 700 (OPPO PHJ110): Cortex-A76 + A55 → 支持 ARM v8.2-A
- 天玑 800/900/9000 系列: 逐步支持 ARM v8.4 / v9.0
- **基础整数编码与 ARM64 完全一致**

---

## 2. MTK 专有扩展

### 2.1 协处理器 (COPROC)

MTK 芯片在 CP10/CP11 (FPU) 之外，有自己的 CP14/CP15 扩展:

```
MRC p15, 0, <Rt>, c15, c0, 0  → 读取 MTK 专有配置
MCR p15, 0, <Rt>, c15, c0, 0  → 写入 MTK 专有配置
```

编码:
```
MRC:  EE 1x 0x 0x  RT  CRn CRm 0x  (CP15)
MCR:  EE 0x 0x 0x  RT  CRn CRm 0x  (CP15)
```

### 2.2 电源管理 (与 ARM 不同)

```
WFI MTK 自定义:
  → D5 03 20 3F  (与 ARM 标准一致，但 VE 版本不同)

WFE 等待事件:
  → D5 03 20 3E

SEV 发送事件:
  → D5 03 20 3D
```

### 2.3 缓存维护 (MTK 扩展)

```
CCSIDR 读取 → 获取缓存拓扑
    MRC p15, 1, <Rt>, c0, c0, 0

Cache Invalidate:
    MCR p15, 0, <Rt>, c7, c5, 0  (ICIALLU)
    MCR p15, 0, <Rt>, c7, c6, 1  (ICIMVAU)
```

MTK 天玑 700 的特殊之处:
- L1: 64KB I-cache + 64KB D-cache (每核)
- L2: 512KB (共享)
- L3: 无 (天玑700) / 4MB (天玑900)

---

## 3. GPU 相关

MTK 天玑 700 使用 Mali-G57 MC2:
- 不是 Adreno，**不支持 CUDA**
- 只支持 GLES 3.2 / Vulkan 1.1
- 没有 OpenCL compute (MTK 专有驱动不支持)

MTK 天玑 9000+ 使用 Mali-G710:
- 支持 Valhall 架构
- 允许有限的 OpenCL 2.0 (需专有驱动)

---

## 4. AI 处理器 — APU

联发科叫 **APU** (AI Processing Unit)，不是 NPU:

天玑 700: 无独立 APU
天玑 900: 有 APU 3.0 (联发科专有)
天玑 9000: APU 590 (6 核 APU)

APU 访问通过专有驱动:
```
/dev/apusys           → APU 电源/时钟
/dev/apu_dfs          → APU 调频
```

**注意:** APU 只在相机/AI 相机场景开放通用计算，不是 CUDA/NVIDIA 那样的通用加速器。

---

## 5. 在内核 (手机) 上的差异

### 5.1 系统调用 ABI

与 ARM 一致: `svc #0`，参数 `x0-x5`，调用号 `x8`，返回 `x0`

### 5.2 io_uring 支持

Linux 5.10+ 已支持 io_uring，天玑 700 内核 5.4+ 部分支持 (`IORING_SETUP_SQPoll`)

### 5.3 BPF / eBPF

ARM64 eBPF 编码:
```
mov x0, #0     → E0 03 80 4A  (orr x0,xzr,xzr)
call bpf_func  → 94 00 00 00  (bl bpf_func)
exit           → D6 5F 03 C0  (ret)
```

---

## 6. MTK 特有: 物理地址扩展 (PAE)

天玑 700 实际支持 36 位物理地址 (64GB)，虽然用户可见只有 6-12GB:

```
AT S1E1R, x0    → 地址转换
    EE 30 2x 0x x0  (System register)
```

---

## 7. 编译 MTK 手机程序时的实际限制

| 维度 | 天玑700 (OPPO PHJ110) | Exynos880 (vivo Y51s) |
|------|----------------------|----------------------|
| 架构 | ARM v8.2-A | ARM v8.2-A |
| 最大并发线程 | 8 | 8 |
| RAM 可用 | 实际可用 ~8GB (含扩展) | 实际可用 ~4GB |
| io_uring | 部分支持 (内核 5.4) | 需内核 5.10+ |
| GPU 计算 | 无 | 无 |
| NPU | 无独立 APU | 无 |
| 最大频率 | 2.2GHz (A76核) | 2.0GHz (Deimos核) |

---

## 8. 在 MTK 上运行服务器程序的可行性

- **DNS 递归解析**: 内存 200MB 足够，CPU 安全
- **HTTP 静态站**: C + io_uring 可支持 500+ 并发连接
- **WebSocket**: 单连接内存 < 100KB，总连接数受 RAM 限制 (~8000 并发)
- **加密 (TLS)**: OpenSSL ARM 已优化，无硬件加速也可承受 ~100 req/s
