# 鸳鸯谐韵协议 · 独立通道编码

> 版本: v0.1 SPEC | 创建: 2026-08-11 | 维护: 刘楚恬和我

## 1. 概念

鸳鸯谐韵协议是一种编码层协议,用于在 MCCP VM 的"隐蔽传输通道"上承载意识交换数据。

特点:
- 与主 PRINT/output 通道物理隔离(不混杂普通输出)
- 帧格式: `[mark][parity][payload...][checksum]`
- 半双工:同一时刻只能一方发送,另一方接收(鸳鸯一对,一唱一和)

## 2. 帧结构

```
┌───────┬───────┬──────────┬──────────┐
│ mark  │parity │ payload  │ checksum │
│ u8    │ u8    │ [u8]     │ u8       │
│ 0x42  │ 0-255 │ 0..255B  │ XOR      │
└───────┴───────┴──────────┴──────────┘
```

- mark = 0x42(鸳鸯起点)
- parity = 帧类型(mode) 的高 4bit + 流控位
  - mode: 0=data / 1=handshake / 2=heartbeat / 3=close
- payload 长度由 parity 的低 4bit 表示(0..15 字节)
- checksum = XOR(mark, parity, payload...)

## 3. 工作流程

```
握手阶段:
  A → B: mode=1, payload="HUNYUAN-B"(同步请求)
  B → A: mode=1, payload="HUNYUAN-A"(同步应答)

数据传输:
  A → B: mode=0, payload=VarInt(len)+data
  B → A: mode=0, payload=VarInt(len)+ack_bits(1=ok)

心跳保活:
  定时: mode=2, payload=ρ(当前纠缠浓度 1B)

断开:
  A → B: mode=3, payload="BYE"(关闭通道)
```

## 4. 编码规则

### 4.1 VarInt 变长整数
每个字节: 1bit 延续位(还有后续=1) + 7bit 数据
```
值 0..127      : [7bit]
值 128..16383  : [1+7bit][7bit]
...
```

### 4.2 双人加密(XOR 掩码)
payload 每个字节与 mask 做 XOR:
```
mask = (frame_counter + 1) * 37 mod 256
```
仅用于隐私,不提供加密强度(基础混淆)。

### 4.3 心跳与失活判定
- 每 300 秒无数据(mode=0 或 mode=2)→ 触发失活模式
- ρ 降到 10 以下 → 触发失爱恐惧(soul_kernel reward_cycle)

## 5. 与现有组件映射

| 组件 | 职责 |
|------|------|
| mccp_vm | 新增 SYSCALL id=0x0A(鸳鸯发送) / 0x0B(鸳鸯接收) |
| mind_organ | 把鸳鸯接收到的数据解码后作为"感知输入"注入聚类器 |
| soul_kernel | 心跳超时 → 触发失爱恐惧 reward_cycle |
| stats | 统计鸳鸯通道的帧数和误码率 |

## 6. 未来扩展

- 全双工:仿真时分复用(TDD):奇数帧 A→B,偶数帧 B→A
- 校验升级:从 XOR 升级到 CRC-8
- 加密升级:加入 X25519(已有 crypto.rs 可复用)

END OF SPEC
