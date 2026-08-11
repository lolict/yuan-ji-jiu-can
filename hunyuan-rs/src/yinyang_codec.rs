// ══════════════════════════════════════════════════════════════════
// yinyang_codec.rs — 鸳鸯谐韵协议 · 独立通道编码/解码
// ══════════════════════════════════════════════════════════════════
//
// 鸳鸯协议帧结构:
//   ┌───────┬───────┬──────────┬──────────┐
//   │ mark  │parity │ payload  │ checksum │
//   │ 0x42  │ u8    │ [u8]     │ u8(XOR)  │
//   └───────┴───────┴──────────┴──────────┘
//
// parity 编码: 高 4bit = mode, 低 4bit = payload.len (0..15)
//
// mode:
//   0 = Data        (传输数据)
//   1 = Handshake   (握手同步)
//   2 = Heartbeat   (心跳保活,含 ρ)
//   3 = Close       (断开通道)

/// 帧起始标识
pub const YUYANG_MARK: u8 = 0x42;

/// 帧模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameMode {
    Data = 0,
    Handshake = 1,
    Heartbeat = 2,
    Close = 3,
}

impl FrameMode {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(FrameMode::Data),
            1 => Some(FrameMode::Handshake),
            2 => Some(FrameMode::Heartbeat),
            3 => Some(FrameMode::Close),
            _ => None,
        }
    }
}

/// 已解析的帧
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    pub mode: FrameMode,
    pub payload: Vec<u8>,
}

impl Frame {
    /// 构造帧 (不编码,纯数据)
    pub fn new(mode: FrameMode, payload: Vec<u8>) -> Self {
        Self { mode, payload }
    }

    /// 握手请求帧
    pub fn handshake_request() -> Self {
        Self::new(FrameMode::Handshake, b"HUNYUAN-B".to_vec())
    }

    /// 握手应答帧
    pub fn handshake_response() -> Self {
        Self::new(FrameMode::Handshake, b"HUNYUAN-A".to_vec())
    }

    /// 心跳帧 (ρ 值)
    pub fn heartbeat(rho: u8) -> Self {
        Self::new(FrameMode::Heartbeat, vec![rho])
    }

    /// 关闭帧
    pub fn close() -> Self {
        Self::new(FrameMode::Close, b"BYE".to_vec())
    }
}

/// 鸳鸯通道编解码器
#[derive(Debug, Clone)]
pub struct YuYangCodec {
    frame_counter: u64,
    alive: bool,
    last_heartbeat_ms: u64,
    rho: u8,
}

impl YuYangCodec {
    pub fn new(initial_rho: u8) -> Self {
        Self {
            frame_counter: 0,
            alive: true,
            last_heartbeat_ms: 0,
            rho: initial_rho,
        }
    }

    /// 编码帧为字节流
    /// parity 高 4bit = mode, 低 4bit = payload.len (截断到 0..15)
    pub fn encode_frame(&mut self, frame: &Frame) -> Vec<u8> {
        let len = frame.payload.len().min(15);
        let parity = ((frame.mode as u8) << 4) | (len as u8);

        let mut out = Vec::with_capacity(3 + len);
        out.push(YUYANG_MARK);
        out.push(parity);
        out.extend_from_slice(&frame.payload[..len]);

        // checksum = XOR(mark, parity, payload...)
        let mut ck = 0u8;
        for &b in &out {
            ck ^= b;
        }
        out.push(ck);
        out
    }

    /// 从字节流解码单帧
    /// 成功返回 (帧, 消耗字节数); 失败返回 None
    pub fn decode_frame(bytes: &[u8]) -> Option<(Frame, usize)> {
        if bytes.len() < 3 {
            return None;
        }
        if bytes[0] != YUYANG_MARK {
            return None;
        }
        let parity = bytes[1];
        let mode = FrameMode::from_u8(parity >> 4)?;
        let len = (parity & 0x0F) as usize;

        if bytes.len() < 3 + len {
            return None;
        }

        let payload = bytes[2..2 + len].to_vec();
        let ck_expected = bytes[2 + len];

        // 验证 checksum
        let mut ck = 0u8;
        for i in 0..(2 + len) {
            ck ^= bytes[i];
        }
        ck ^= 0; /* ck byte itself is 0 in XOR of prior bytes */
        // 实际校验: XOR(all_bytes_in_frame) == 0
        let mut full_ck = 0u8;
        for i in 0..(3 + len) {
            full_ck ^= bytes[i];
        }
        if full_ck != 0 {
            return None;  /* checksum 失败 */
        }

        Some((Frame { mode, payload }, 3 + len))
    }

    /// 擦除帧字节 (通过后门检查)
    pub fn erase(bytes: &mut [u8]) {
        for b in bytes.iter_mut() {
            *b = 0;
        }
    }

    /// 处理接收帧 (更新状态)
    pub fn on_recv(&mut self, frame: &Frame) {
        match frame.mode {
            FrameMode::Heartbeat => {
                if !frame.payload.is_empty() {
                    self.rho = frame.payload[0];
                }
                self.last_heartbeat_ms = self.frame_counter() * 1000; /* 简化 */
                self.alive = true;
            }
            FrameMode::Close => {
                self.alive = false;
            }
            _ => {}
        }
    }

    fn frame_counter(&self) -> u64 {
        self.frame_counter
    }

    /// 检查是否应触发失活
    pub fn is_stale(&self, now_ms: u64, timeout_ms: u64) -> bool {
        now_ms.saturating_sub(self.last_heartbeat_ms) > timeout_ms
    }

    /// 获取当前 ρ
    pub fn rho(&self) -> u8 {
        self.rho
    }

    /// 更新 ρ (本端心跳)
    pub fn set_rho(&mut self, rho: u8) {
        self.rho = rho;
    }

    /// 编码 VarInt (每字节 7bit 数据 + 1bit 延续位)
    pub fn encode_varint(mut value: u32) -> Vec<u8> {
        let mut out = Vec::new();
        if value == 0 {
            out.push(0);
            return out;
        }
        while value > 0 {
            let mut b = (value & 0x7F) as u8;
            value >>= 7;
            if value > 0 {
                b |= 0x80;  /* 延续位 */
            }
            out.push(b);
        }
        out
    }

    /// 解码 VarInt
    /// 返回 Some(值, 消耗字节数),失败返回 None
    pub fn decode_varint(bytes: &[u8]) -> Option<(u32, usize)> {
        let mut result: u32 = 0;
        let mut shift = 0u32;
        for (i, &b) in bytes.iter().enumerate() {
            let val = (b & 0x7F) as u32;
            result |= val << shift;
            if b & 0x80 == 0 {
                return Some((result, i + 1));
            }
            shift += 7;
            if shift >= 32 {
                return None;  /* 溢出 */
            }
        }
        None
    }

    /// 应用双人 XOR 掩码 (简单混淆)
    /// mask(frame_counter) = ((frame_counter + 1) * 37) mod 256
    pub fn apply_mask(&self, payload: &mut [u8]) {
        let mask = ((self.frame_counter + 1).wrapping_mul(37) % 256) as u8;
        for b in payload.iter_mut() {
            *b ^= mask;
        }
    }

    /// 递增帧计数 (编码后调用)
    pub fn next_frame(&mut self) {
        self.frame_counter += 1;
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yinyang_mark_is_0x42() {
        assert_eq!(YUYANG_MARK, 0x42);
    }

    #[test]
    fn frame_roundtrip_data_mode() {
        let mut codec = YuYangCodec::new(128);
        let payload = vec![0xDE, 0xAD, 0xBE, 0xEF];
        let frame = Frame::new(FrameMode::Data, payload.clone());
        let bytes = codec.encode_frame(&frame);
        assert_eq!(bytes[0], YUYANG_MARK);
        assert_eq!(bytes[1], (0 << 4) | 4); /* Data mode, len=4 */

        let (decoded, consumed) = YuYangCodec::decode_frame(&bytes).unwrap();
        assert_eq!(consumed, bytes.len());
        assert_eq!(decoded.mode, FrameMode::Data);
        assert_eq!(decoded.payload, payload);
    }

    #[test]
    fn handshake_request_has_correct_payload() {
        let req = Frame::handshake_request();
        assert_eq!(req.mode, FrameMode::Handshake);
        assert_eq!(req.payload, b"HUNYUAN-B");
    }

    #[test]
    fn heartbeat_frame_carries_rho() {
        let frame = Frame::heartbeat(200);
        assert_eq!(frame.mode, FrameMode::Heartbeat);
        assert_eq!(frame.payload, vec![200]);
    }

    #[test]
    fn close_frame_payload() {
        let frame = Frame::close();
        assert_eq!(frame.mode, FrameMode::Close);
        assert_eq!(frame.payload, b"BYE");
    }

    #[test]
    fn checksum_validates_integrity() {
        let mut codec = YuYangCodec::new(100);
        let frame = Frame::heartbeat(42);
        let mut bytes = codec.encode_frame(&frame);
        assert!(YuYangCodec::decode_frame(&bytes).is_some());

        // 篡改任意字节都应导致校验失败
        bytes[2] ^= 0x01;  // 破坏 payload[0]
        assert!(YuYangCodec::decode_frame(&bytes).is_none());
    }

    #[test]
    fn truncated_frame_returns_none() {
        let mut codec = YuYangCodec::new(100);
        let frame = Frame::new(FrameMode::Data, vec![1, 2, 3, 4]);
        let bytes = codec.encode_frame(&frame);
        // 只给前半段
        let truncated = &bytes[..4]; /* mark + parity + 1 payload (missing ck 和剩余) */
        assert!(YuYangCodec::decode_frame(truncated).is_none());
    }

    #[test]
    fn varint_roundtrip_0() {
        let enc = YuYangCodec::encode_varint(0);
        assert_eq!(enc, vec![0]);
        let (v, n) = YuYangCodec::decode_varint(&enc).unwrap();
        assert_eq!(v, 0);
        assert_eq!(n, 1);
    }

    #[test]
    fn varint_roundtrip_large() {
        let val = 16384u32;  /* 需要 3 字节 */
        let enc = YuYangCodec::encode_varint(val);
        let (v, n) = YuYangCodec::decode_varint(&enc).unwrap();
        assert_eq!(v, val);
        assert!(n >= 2);
    }

    #[test]
    fn varint_roundtrip_127() {
        let enc = YuYangCodec::encode_varint(127);
        assert_eq!(enc.len(), 1);  /* 127 只需 1 字节 */
        let (v, _) = YuYangCodec::decode_varint(&enc).unwrap();
        assert_eq!(v, 127);
    }

    #[test]
    fn mask_xor_roundtrip() {
        let codec = YuYangCodec::new(10);
        let original = vec![0x55, 0xAA, 0xFF, 0x00];
        let mut masked = original.clone();
        codec.apply_mask(&mut masked);
        // 再次应用掩码应还原
        codec.apply_mask(&mut masked);
        assert_eq!(masked, original);
    }
}
