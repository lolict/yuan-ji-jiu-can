# 五彩石隐写工具集 (Layer 3 混淆层)

将秘密数据嵌入公开内容中，猎取者即使拿到也无法识别。

## 三种模式

### LSB 模式（PNG 图像）
将数据写入 PNG 像素的最低有效位。视觉完全不变，可存储大量数据。

```bash
# 编码
python3 steganography.py encode lsb carrier.png "秘密数据" output.png

# 解码
python3 steganography.py decode lsb output.png
```

容量公式：`宽 × 高 × 3 ÷ 8` 字节（RGB三个通道各1 bit）

### Unicode 模式（文本）
使用零宽字符（ZWSP/ZWNJ）在可见文本中隐藏二进制数据。看起来完全正常的文字。

```bash
# 编码
python3 steganography.py encode unicode "正常博客文本" output.txt -p "秘密"

# 解码
python3 steganography.py decode unicode output.txt
```

特点：可复制到任何文本字段（评论、Issue、文档），视觉不可见。

### HTML 模式（网页）
在 HTML 中嵌入不可见的隐藏 span 和 base64 注释。

```bash
python3 steganography.py encode html page.html output.html -p "秘密"
python3 steganography.py decode output.html
```

## 容量对比

| 模式 | 载体 | 容量 | 持久性 |
|------|------|------|--------|
| LSB | 1000x1000 PNG | ~375 KB | 中（重新压缩会丢失） |
| Unicode | 1000 字文本 | ~105 KB | 高（复制粘贴保留） |
| HTML | 网页 | 无限制 | 高 |

## 安全提示

1. **这不是加密**——只提供混淆。敏感数据请先加密再隐写。
2. **CRC32 校验**——LSB 模式有完整性校验。
3. **LSB 抗压缩**——PNG 是无损的；如果图片被转成 JPEG，数据会丢失。
