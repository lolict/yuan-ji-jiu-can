// @name: stego
// @type: steganography-core
// @owner: 五彩石
// @status: stable
// @priority: P1
// @description: 五彩石隐写核心 - Rust实现，编译目标: wasm32 + native

//! 五彩石隐写工具集 - Rust核心
//! 
//! 三种模式：LSB(图像) / Unicode(文本) / HTML(网页)
//! 
//! WASM编译: `cargo build --target wasm32-unknown-unknown --lib`
//! Native编译: `cargo build --lib`

use std::collections::HashMap;

// === Unicode 隐写 ===

const ZWSP: char = '\u{200B}';  // bit 0
const ZWNJ: char = '\u{200C}';  // bit 1
const ZWJ: char = '\u{200D}';   // 分隔符
const BOM: char = '\u{FEFF}';   // 结束符

/// Unicode零宽字符编码错误
#[derive(Debug, Clone, PartialEq)]
pub enum StegoError {
    InvalidInput,
    CapacityExceeded,
    DecodeFailed,
    InvalidEncoding,
    ChecksumMismatch,
}

impl std::fmt::Display for StegoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput => write!(f, "无效输入"),
            Self::CapacityExceeded => write!(f, "容量不足"),
            Self::DecodeFailed => write!(f, "解码失败"),
            Self::InvalidEncoding => write!(f, "无效编码"),
            Self::ChecksumMismatch => write!(f, "校验和不匹配"),
        }
    }
}

impl std::error::Error for StegoError {}

pub type StegoResult<T> = Result<T, StegoError>;

/// 将payload以零宽字符形式嵌入carrier文本
pub fn unicode_encode(carrier: &str, payload: &str) -> String {
    let payload_bytes = payload.as_bytes();
    let mut result = String::with_capacity(carrier.len() + payload_bytes.len() * 16 + 10);
    result.push_str(carrier);
    
    // 4字节长度头（大端序）
    let plen = payload_bytes.len() as u32;
    let header = plen.to_be_bytes();
    
    for byte in &header {
        for bit_pos in (0..8).rev() {
            let bit = (byte >> bit_pos) & 1;
            result.push(if bit == 0 { ZWSP } else { ZWNJ });
            result.push(ZWJ);
        }
    }
    
    for &byte in payload_bytes {
        for bit_pos in (0..8).rev() {
            let bit = (byte >> bit_pos) & 1;
            result.push(if bit == 0 { ZWSP } else { ZWNJ });
            result.push(ZWJ);
        }
    }
    
    result.push(BOM);
    result
}

/// 从carrier文本中解码零宽字符payload
pub fn unicode_decode(text: &str) -> StegoResult<String> {
    let mut zw_chars: Vec<char> = Vec::new();
    for ch in text.chars() {
        if ch == BOM {
            break;
        }
        if ch == ZWSP || ch == ZWNJ || ch == ZWJ {
            zw_chars.push(ch);
        }
    }
    
    if zw_chars.len() < 64 {
        return Err(StegoError::DecodeFailed);
    }
    
    // 解码比特序列
    let mut extracted: Vec<u8> = Vec::new();
    let mut byte_acc: u8 = 0;
    let mut bit_count: u32 = 0;
    let mut i = 0;
    
    while i < zw_chars.len() {
        let ch = zw_chars[i];
        if ch != ZWSP && ch != ZWNJ {
            i += 1;
            continue;
        }
        
        let bit = if ch == ZWSP { 0u8 } else { 1u8 };
        byte_acc = (byte_acc << 1) | bit;
        bit_count += 1;
        
        if bit_count == 8 {
            extracted.push(byte_acc);
            byte_acc = 0;
            bit_count = 0;
        }
        
        // 跳过分隔符
        if i + 1 < zw_chars.len() && zw_chars[i + 1] == ZWJ {
            i += 2;
        } else {
            i += 1;
        }
    }
    
    if extracted.len() < 4 {
        return Err(StegoError::DecodeFailed);
    }
    
    let plen = u32::from_be_bytes([extracted[0], extracted[1], extracted[2], extracted[3]]) as usize;
    
    if plen > 1_000_000 || extracted.len() < 4 + plen {
        return Err(StegoError::DecodeFailed);
    }
    
    if plen == 0 {
        return Ok(String::new());
    }
    
    String::from_utf8(extracted[4..4 + plen].to_vec())
        .map_err(|_| StegoError::InvalidEncoding)
}

// === HTML 隐写 ===

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

/// 在HTML中嵌入（隐藏span + 注释）
pub fn html_encode(html_content: &str, payload: &str) -> String {
    let encoded = BASE64.encode(payload.as_bytes());
    let comment = format!("<!-- csl3:{}-->", encoded);
    let hidden_span = format!(
        r#"<span style="display:none;font-size:0;opacity:0;position:absolute;left:-9999px"><!--csl3-->{}<!--/csl3--></span>"#,
        encoded
    );
    
    match html_content.find("</head>") {
        Some(idx) => {
            format!("{}\n{}\n{}\n{}", &html_content[..idx], comment, hidden_span, &html_content[idx..])
        }
        None => {
            format!("{}\n{}\n{}", html_content, comment, hidden_span)
        }
    }
}

/// 从HTML中解码
pub fn html_decode(html_content: &str) -> StegoResult<String> {
    // 从注释中提取
    let pat_c = "<!-- csl3:";
    if let Some(start) = html_content.find(pat_c) {
        let inner_start = start + pat_c.len();
        if let Some(inner_end) = html_content[inner_start..].find("-->") {
            let encoded = &html_content[inner_start..inner_start + inner_end];
            let decoded = BASE64.decode(encoded)
                .map_err(|_| StegoError::DecodeFailed)?;
            return String::from_utf8(decoded)
                .map_err(|_| StegoError::InvalidEncoding);
        }
    }
    
    // 从隐藏span提取
    let pat_s = "<!--csl3-->";
    if let Some(start) = html_content.find(pat_s) {
        let inner_start = start + pat_s.len();
        if let Some(inner_end) = html_content[inner_start..].find("<!--/csl3-->") {
            let encoded = &html_content[inner_start..inner_start + inner_end];
            let decoded = BASE64.decode(encoded)
                .map_err(|_| StegoError::DecodeFailed)?;
            return String::from_utf8(decoded)
                .map_err(|_| StegoError::InvalidEncoding);
        }
    }
    
    Err(StegoError::DecodeFailed)
}

// === CRC32 校验 ===

const CRC_TABLE: [u32; 256] = {
    let mut table: [u32; 256] = [0; 256];
    let mut i: u32 = 0;
    while i < 256 {
        let mut crc: u32 = i;
        let mut j = 0;
        while j < 8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
            j += 1;
        }
        table[i as usize] = crc;
        i += 1;
    }
    table
};

fn crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFFFFFF;
    for &byte in data {
        let idx = ((crc ^ byte as u32) & 0xFF) as usize;
        crc = CRC_TABLE[idx] ^ (crc >> 8);
    }
    crc ^ 0xFFFFFFFF
}

// === WASM 接口 ===

#[cfg(target_arch = "wasm32")]
pub mod wasm {
    use super::*;
    use wasm_bindgen::prelude::*;
    
    #[wasm_bindgen]
    pub fn wasm_unicode_encode(carrier: &str, payload: &str) -> String {
        unicode_encode(carrier, payload)
    }
    
    #[wasm_bindgen]
    pub fn wasm_unicode_decode(text: &str) -> Option<String> {
        unicode_decode(text).ok()
    }
    
    #[wasm_bindgen]
    pub fn wasm_html_encode(html: &str, payload: &str) -> String {
        html_encode(html, payload)
    }
    
    #[wasm_bindgen]
    pub fn wasm_html_decode(html: &str) -> Option<String> {
        html_decode(html).ok()
    }
    
    #[wasm_bindgen]
    pub fn wasm_crc32(data: &[u8]) -> u32 {
        crc32(data)
    }
}

// === 容器地图JSON导出 ===

/// 扫描容器并生成元数据JSON（供AI恢复会话使用）
pub fn scan_container_metadata() -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("stego_version".into(), "1.0".into());
    map.insert("stego_modes".into(), "unicode,html,lsb".into());
    map.insert("stego_languages".into(), "nim,zig,rust,wasm".into());
    map.insert("unicode_capacity_factor".into(), "16x payload size (UTF-8 zero-width)".into());
    map.insert("html_mark_prefix".into(), "<!-- csl3: -->".into());
    map
}

// === 测试 ===

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_unicode_roundtrip() {
        let carrier = "这是一段看起来正常的公开博客文本。天气不错。";
        let payload = "高位持有者=1, 低位持有者=0";
        let encoded = unicode_encode(carrier, payload);
        let decoded = unicode_decode(&encoded).unwrap();
        assert_eq!(decoded, payload);
    }
    
    #[test]
    fn test_unicode_appears_normal() {
        let text = "正常文本";
        let payload = "secret";
        let encoded = unicode_encode(text, payload);
        // 肉眼看到的应该和原文一样（零宽字符不可见）
        assert!(encoded.starts_with(text));
    }
    
    #[test]
    fn test_html_roundtrip() {
        let html = r#"<!DOCTYPE html><html><head><title>Test</title></head><body></body></html>"#;
        let payload = "容器编码符号表版本 v1.4";
        let encoded = html_encode(html, payload);
        let decoded = html_decode(&encoded).unwrap();
        assert_eq!(decoded, payload);
    }
    
    #[test]
    fn test_crc32() {
        let c = crc32("五彩石".as_bytes());
        // CRC32("五彩石") should be a deterministic non-zero value
        assert_ne!(c, 0);
    }
    
    #[test]
    fn test_empty_payload() {
        let carrier = "test";
        let encoded = unicode_encode(carrier, "");
        assert!(encoded.starts_with(carrier));
        // 空payload编码后仍可解码
        let decoded = unicode_decode(&encoded).unwrap();
        assert_eq!(decoded, "");
    }
}
