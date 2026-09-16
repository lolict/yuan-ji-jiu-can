# 五彩石隐写工具集 (Layer 3 混淆层)

将秘密数据嵌入公开内容中，猎取者即使拿到也无法识别。

## 实现版本

| 语言 | 文件 | 模式 | 编译目标 |
|------|------|------|----------|
| Nim | `stego_nim.nim` | Unicode + HTML | Native |
| Zig | `stego_zig.zig` | Unicode + HTML | Native |
| Rust | `hunyuan-rs/src/stego.rs` | Unicode + HTML + CRC32 | Native + WASM |
| Python | `steganography.py` | Unicode + HTML + LSB | 参考实现 |

> Python 文件为原始参考实现。新开发优先使用 Nim/Zig/Rust。

## 三种模式

### Unicode 模式（文本）
使用零宽字符（ZWSP/ZWNJ）在可见文本中隐藏二进制数据。看起来完全正常的文字。

```bash
# Nim
nim c -r stego_nim.nim

# Zig
zig build-exe stego_zig.zig

# Rust (已集成到 hunyuan-rs 库)
cargo test --lib stego
```

特点：可复制到任何文本字段（评论、Issue、文档），视觉不可见。

### HTML 模式（网页）
在 HTML 中嵌入不可见的隐藏 span 和 base64 注释。

```rust
// Rust 内嵌使用
use mqf::stego::{html_encode, html_decode};
let encoded = html_embed(html_content, payload);
let decoded = html_extract(&encoded).unwrap();
```

### LSB 模式（PNG 图像）
将数据写入 PNG 像素的最低有效位。视觉完全不变，可存储大量数据。
仅在 Python 参考实现中提供。

## Rust WASM 编译

```bash
# 添加 wasm32 目标
rustup target add wasm32-unknown-unknown

# 编译为 WASM
cd hunyuan-rs
cargo build --target wasm32-unknown-unknown --lib
```

WASM 绑定接口（通过 `wasm-bindgen`）：

- `wasm_unicode_encode(carrier, payload) → String`
- `wasm_unicode_decode(text) → String | null`
- `wasm_html_encode(html, payload) → String`
- `wasm_html_decode(html) → String | null`
- `wasm_crc32(data) → u32`

## 容量对比

| 模式 | 载体 | 容量 | 持久性 |
|------|------|------|--------|
| Unicode | 1000 字文本 | ~105 KB | 高（复制粘贴保留） |
| HTML | 网页 | 无限制 | 高 |
| LSB | 1000x1000 PNG | ~375 KB | 中（重新压缩会丢失）— 仅 Python |

## 安全提示

1. **这不是加密**——只提供混淆。敏感数据请先加密再隐写。
2. **CRC32 校验**——Rust 版本带完整性校验。
3. **LSB 抗压缩**——PNG 是无损的；如果图片被转成 JPEG，数据会丢失。
