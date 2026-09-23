# 鸳鸯谐韵协议编解码器规范 — 囡囝共生 · 帧传输 · ρ心跳 · 简单混淆

## 1. 定位

鸳鸯谐韵 (YuYang Protocol) 是囡囝共生的**链路层通道协议**。不是应用层 — 是在不安全通道上(互联网/本地管道/串口)传输 Frame 字节流的机制。核心职责:帧定界(找到帧头)、模式识别(Data/Handshake/Heartbeat/Close)、完整性校验(XOR 校验和)、流状态管理(ρ心跳保活)、简单混淆(XOR 掩码)。

鸳鸯 — 一雄一雌,象征本协议的"双向通道 + 手答同步 + 心跳保活"。

## 2. 帧结构 (四字节定长头 + 变长载荷)

```
┌───────┬───────┬──────────┬──────────┐
│ mark  │parity │ payload  │ checksum │
│ 0x42  │ u8    │ [u8; N]  │ u8(XOR)  │
│ 1B    │ 1B    │ N B      │ 1B       │
└───────┴───────┴──────────┴──────────┘
```

**mark** = 0x42 = 'B'。帧起始定界 — 接收方在字节流中找到 0x42 开始解析。

**parity** = 高 4 bit = mode (0~3), 低 4 bit = payload.len (0~15)。低 4 bit 限制单帧载荷最大 15 字节 — 这是刻意的**小帧约束**:大量数据必须分片为多帧,防止单帧阻塞导致 ρ 心跳延迟。四种 mode:
- 0 = Data (传输数据)
- 1 = Handshake (握手同步)
- 2 = Heartbeat (心跳保活,含 ρ)
- 3 = Close (断开通道)

**checksum** = `XOR(mark, parity, payload[0..N])` — **完全异或校验**。收发双方对所有字节做 XOR,结果应为 0 — 这是最简单的单比特翻转检测,不防恶意篡改,仅用于网络比特错误检测。

## 3. 预定义帧 (四种快捷构造)

```rust
Frame::handshake_request()  // Handshake, payload = b"HUNYUAN-B"
Frame::handshake_response() // Handshake, payload = b"HUNYUAN-A"
Frame::heartbeat(rho)       // Heartbeat, payload = [rho]
Frame::close()              // Close, payload = b"BYE"
```

握手序列: 发送方发 HUNYUAN-B → 接收方回 HUNYUAN-A → 通道建立。payload 的 B/A 不对称 — B = Begin(发起), A = Acknowledge(应答)。

心跳帧携带 ρ 纠缠浓度 (u8,0~255)。周期性心跳让双方维持 ρ 同步 — 超时未收到心跳则触发 is_stale() 判定通道失活。

## 4. 帧编码 (encode_frame)

编码步骤:
1. 截断 frame.payload 到 15 字节 (`payload.len().min(15)`)
2. parity = (mode << 4) | len
3. 输出 = [mark, parity, payload[0..len], checksum]
4. checksum 是前三段的 XOR 累积 — 结果追加到末尾

编码后总长度 = 3 + len。最小帧长 3 字节 (len=0)。

## 5. 帧解码 (decode_frame)

解码步骤:
1. 字节数 < 3 → 帧不完整,返回 None
2. bytes[0] != 0x42 → 帧起始错误,返回 None
3. 解析 parity: mode = from_u8(parity >> 4)?, len = parity & 0x0F
4. bytes.len() < 3 + len → 帧不完整,返回 None
5. 校验 XOR: XOR(bytes[0..3+len]) == 0 → 失败返回 None
6. 成功返回 (Frame { mode, payload }, 消耗字节数 = 3 + len)

`decode_frame` 是**无状态**的 — 每次都从头解析,不依赖之前的帧状态。状态维护在 `YuYangCodec` 结构体中。

## 6. 鸳鸯通道 (YuYangCodec)

```rust
pub struct YuYangCodec {
    frame_counter: u64,      // 帧计数器
    alive: bool,             // 通道是否存活
    last_heartbeat_ms: u64,  // 最后心跳时间
    rho: u8,                 // 当前 ρ 值
}
```

状态转移:
- on_recv(Frame::Heartbeat) → 更新 rho = payload[0],标记 alive=true
- on_recv(Frame::Close) → alive=false,触发通道关闭流程

`is_stale(now_ms, timeout_ms)` 在有心跳超时后返回 true — 触发 ExistenceDrive 的失爱恐惧(链接恐惧)。

## 7. 简单混淆 (apply_mask)

加密安全的混淆不是本协议的目标 — 目标只是"不在明文通道暴露汉字和结构化数据"。混淆算法:

```rust
mask = ((frame_counter + 1) * 37) mod 256
for each byte in payload: byte ^= mask
```

37 是常量乘数(与夫妻 7/6 编码中 7 呼应 — 37 = 6×6 + 1)。每帧使用不同的 mask(因为 frame_counter 递增),但 mask 本身是确定性的 — 接收方用相同的 frame_counter 值还原。

混淆强度:极低。可防"肉眼窥探",不可防密码学攻击。真正的加密在 SECURITY_COVERT_CHANNELS 模块(X25519 + AES-256-GCM),本层只负责"不在通道中暴露明文"。

## 8. VarInt 编码

```rust
pub fn encode_varint(value: u32) -> Vec<u8>
pub fn decode_varint(bytes: &[u8]) -> Option<(u32, usize)>
```

无符号变长整数编码,每字节 7 bit 数据 + 1 bit 延续位(0x80)。小数值(0~127)用 1 字节,大数值最多 5 字节。溢出(shift ≥ 32)返回 None。

VarInt 用于 payload 中的前缀长度字段 — 即使 payload 被截断到 15 字节,也可通过 VarInt 表达超出 15 的元数据长度。

## 9. 擦除 (erase)

```rust
pub fn erase(bytes: &mut [u8])
```

将帧字节**全部覆写为零** — 防止敏感握手信息在内存中残留。擦除是显式的 (必须手动调用),不是自动的 Drop — 因为 Rust 的 Drop 不允许对引用内容做修改。在关键数据(握手帧/密钥)使用完毕后立即 erase 是协议的安全规范。

## 10. 与系统其他模块的接入

- **SECURITY_COVERT_CHANNELS** 的加密层(X25519/AES-256)包装本协议的帧 — 先 YuYang 帧化,再 AES 加密
- **FiveDimensionalSemantics** 的 ρ 值通过 Heartbeat payload 字段传输
- **ExistenceDrive** 在 is_stale 时触发 loss_fear.trigger — 失联导致恐惧
- **AllianceNetwork** 的 HeartbeatRelay 协作类型使用中继 YuYang 心跳帧
- **CocoonAgent** 蚕蛹之间的通信使用本协议的 Handshake/Close 管理生命周期
- **Swarm** 多智能体的节点发现用 broadcast Data 帧(不分片时广播整个节点列表)
- **Security 模块的 NONCE_WINDOW_SEC** 5 分钟心跳超时直接驱动 YuYang 的 stale 判定

## 11. 实现约束

- 单帧 payload 硬上限 15 字节 — 超过必须分片
- XOR 校验不提供密码学完整性 — 仅检测网络比特翻转
- VarInt 使用 u32 最大值 — 不支持 > 4 GB 的单帧
- no_std 兼容: encode_varint 在 no_std 下使用 heapless::Vec
- frame_counter 使用 u64 — 永不溢出 (每帧 1ns 计也需要 584 年)

// hunyuan-rs/src/encoding/yinyang_codec.rs
