# Kernel 与 NJSG hunyuan-methodology 的关系

> 位置: hunyuan-os/kernel/ (WASM kernel + njsg_kernel.c)

---

## 定位

kernel 是 hunyuan-os 的底层执行引擎（WASM 目标 + C 实现）。
它不直接面向用户，但决定系统的"裁决一致性"。

## 编码标准引用

kernel 在进程间通信和状态存储中与 hunyuan-rs 的 64 bit cell 互操作：

```
Cell 64 bit 的低 32 位 = MCCP 32 bit = kernel 进程描述符的基础字段
polarity bit (bit 39) 区分容器侧/界线侧进程
sovereignty bit (bit 38) 决定进程权限级别
```

kernel/src/njsg_kernel.c 中的进程控制块应与 mirror.rs 的 encode_cell/decode_cell 对齐。

## 裁决引擎的领域无关性

无论部署在哪个域名/服务器上，裁决算法和结果保持一致。
kernel 内部不硬编码任何部署特定的值——裁决逻辑完全基于五规则（阴阳/共生/残差锚点/集装箱/主权）。

## 命名约束

kernel 代码中的符号命名遵循家庭命名密码表：
- `NJSG_*` 前缀 = 囡囝共生宏/函数
- 禁止负面谐音命名

---

*配套: hunyuan-rs v0.14 mirror.rs + hunyuan-methodology*

## 代数基础

`docs/MLLL-ALGEBRAFOUNDATION.md` 定义了 mlll 编码的代数体系：
- 初代数（一元）：时间绝对定位 + 颗粒均匀性 + 偏移量为0
- 偏移量层级化：平行世界尺子 + 齐头并进偏移 + 静态回归
- 256 = 254 + 2 的自我相似叠加律
- 2²⁵⁶ 共同体高位坍缩

编码规范、容器实现、WASM裁定逻辑均基于此代数体系。
