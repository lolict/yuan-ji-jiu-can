#!/usr/bin/env python3
"""
五彩石隐写工具集 - Layer 3 混淆层
三种模式：LSB(图像) / Unicode(文本) / HTML(网页)

使用方式：
  python3 steganography.py encode <mode> <input> <payload> <output>
  python3 steganography.py decode <mode> <input>

模式：
  lsb    - PNG 图像最低有效位隐写
  unicode - 不可见 Unicode 组合字符编码
  html   - HTML 隐藏 span / 注释
"""

import sys
import argparse
import struct
import zlib
from pathlib import Path


# =============================================================================
# LSB 图像隐写 (PNG)
# =============================================================================

def lsb_encode(image_path: str, payload: str, output_path: str) -> bool:
    """将 payload 编码到 PNG 图像的 LSB 中"""
    try:
        from PIL import Image
    except ImportError:
        print("需要 Pillow: pip install Pillow", file=sys.stderr)
        return False

    img = Image.open(image_path)
    if img.mode != 'RGBA':
        img = img.convert('RGBA')

    pixels = bytearray(img.tobytes())
    # 格式：4字节长度 + payload字节 + 4字节CRC32
    payload_bytes = payload.encode('utf-8')
    header = struct.pack('>I', len(payload_bytes))
    crc = struct.pack('>I', zlib.crc32(payload_bytes) & 0xFFFFFFFF)
    data = header + payload_bytes + crc

    # 检查容量
    max_bits = len(pixels) * 3  # RGB 三个通道的 LSB
    needed_bits = len(data) * 8
    if needed_bits > max_bits:
        print(f"图像太小：需要 {needed_bits} bits，只有 {max_bits} bits 可用",
              file=sys.stderr)
        return False

    # 编码
    data_bits = []
    for byte in data:
        for bit_pos in range(7, -1, -1):
            data_bits.append((byte >> bit_pos) & 1)

    for i, bit in enumerate(data_bits):
        # 清除 RGB 通道的最低位并设置新值
        pixels[i] = (pixels[i] & 0xFE) | bit

    new_img = Image.frombytes('RGBA', img.size, bytes(pixels))
    new_img.save(output_path)
    print(f"已编码 {len(payload_bytes)} 字节到 {output_path}")
    return True


def lsb_decode(image_path: str) -> str | None:
    """从 PNG 图像 LSB 中解码 payload"""
    try:
        from PIL import Image
    except ImportError:
        print("需要 Pillow: pip install Pillow", file=sys.stderr)
        return None

    img = Image.open(image_path)
    if img.mode != 'RGBA':
        img = img.convert('RGBA')

    pixels = bytearray(img.tobytes())

    # 提取前 32 bits (4字节长度头)
    length_bits = []
    for i in range(32):
        length_bits.append(pixels[i] & 1)

    length_bytes = bytearray()
    for byte_idx in range(4):
        val = 0
        for bit_idx in range(8):
            val = (val << 1) | length_bits[byte_idx * 8 + bit_idx]
        length_bytes.append(val)

    payload_len = struct.unpack('>I', bytes(length_bytes))[0]

    if payload_len > 10_000_000:  # 安全限制：10MB
        print(f"异常 payload 长度: {payload_len}，可能不是隐写图像",
              file=sys.stderr)
        return None

    # 提取 payload + CRC (length + 4 bytes)
    total_bits_needed = (4 + payload_len + 4) * 8
    all_bits = []
    for i in range(total_bits_needed):
        all_bits.append(pixels[i] & 1)

    # 跳过 32 bit header，提取 payload 和 CRC
    payload_bits = all_bits[32:32 + payload_len * 8]
    crc_bits = all_bits[32 + payload_len * 8:32 + payload_len * 8 + 32]

    payload_bytes = bytearray()
    for byte_idx in range(payload_len):
        val = 0
        for bit_idx in range(8):
            val = (val << 1) | payload_bits[byte_idx * 8 + bit_idx]
        payload_bytes.append(val)

    crc_bytes = bytearray()
    for byte_idx in range(4):
        val = 0
        for bit_idx in range(8):
            val = (val << 1) | crc_bits[byte_idx * 8 + bit_idx]
        crc_bytes.append(val)

    stored_crc = struct.unpack('>I', bytes(crc_bytes))[0]
    computed_crc = zlib.crc32(bytes(payload_bytes)) & 0xFFFFFFFF

    if stored_crc != computed_crc:
        print("CRC32 校验失败，可能不是隐写图像或已损坏",
              file=sys.stderr)
        return None

    return bytes(payload_bytes).decode('utf-8')


# =============================================================================
# Unicode 组合字符隐写
# =============================================================================

# 使用零宽字符做二进制编码
# U+200B (ZWSP)     = 0
# U+200C (ZWNJ)     = 1
# U+200D (ZWJ)      分隔符/结束
# U+FEFF (BOM)      停止符

ZWSP = '\u200b'   # 0
ZWNJ = '\u200c'   # 1
ZWJ  = '\u200d'   # 字符分隔
BOM  = '\ufeff'   # 结束


def unicode_encode(carrier: str, payload: str) -> str:
    """将 payload 以零宽字符形式嵌入 carrier 文本中"""
    payload_bytes = payload.encode('utf-8')
    length_header = struct.pack('>I', len(payload_bytes))

    bits = []
    # 4字节长度头
    for byte in length_header:
        for bit_pos in range(7, -1, -1):
            bits.append((byte >> bit_pos) & 1)
    # payload
    for byte in payload_bytes:
        for bit_pos in range(7, -1, -1):
            bits.append((byte >> bit_pos) & 1)

    # 编码为零宽字符序列
    result = [carrier]
    for bit in bits:
        result.append(ZWSP if bit == 0 else ZWNJ)
        result.append(ZWJ)  # 每 bit 后跟分隔符
    result.append(BOM)  # 结束符

    return ''.join(result)


def unicode_decode(text: str) -> str | None:
    """从 carrier 文本中解码零宽字符 payload"""
    # 提取零宽字符直到 BOM
    zw_chars = []
    for ch in text:
        if ch == BOM:
            break
        if ch in (ZWSP, ZWNJ, ZWJ):
            zw_chars.append(ch)

    # 每两个一组：(bit, separator)，提取 bit 部分
    bits = []
    for i in range(0, len(zw_chars), 2):
        if i + 1 < len(zw_chars) and zw_chars[i + 1] == ZWJ:
            bits.append(0 if zw_chars[i] == ZWSP else 1)
        elif zw_chars[i] == BOM:
            break

    if len(bits) < 32:
        print("未检测到隐写数据", file=sys.stderr)
        return None

    # 前 32 bits = 长度
    length = 0
    for bit in bits[:32]:
        length = (length << 1) | bit

    if length > 1_000_000 or length == 0:
        print(f"异常长度: {length}", file=sys.stderr)
        return None

    needed_bits = 32 + length * 8
    if len(bits) < needed_bits:
        print(f"数据不完整：需要 {needed_bits} bits，只有 {len(bits)} bits",
              file=sys.stderr)
        return None

    payload_bits = bits[32:needed_bits]
    payload_bytes = bytearray()
    for byte_idx in range(length):
        val = 0
        for bit_idx in range(8):
            val = (val << 1) | payload_bits[byte_idx * 8 + bit_idx]
        payload_bytes.append(val)

    return bytes(payload_bytes).decode('utf-8')


# =============================================================================
# HTML 隐写（隐藏 span + 注释）
# =============================================================================

def html_encode(html_content: str, payload: str) -> str:
    """
    使用两种方式在 HTML 中嵌入 hidden payload：
    1. HTML 注释中的 base64
    2. display:none 的隐藏 span
    """
    import base64

    payload_bytes = payload.encode('utf-8')
    encoded = base64.b64encode(payload_bytes).decode('ascii')

    # 方式1: 隐藏注释
    comment = f"<!-- csl3:{encoded} -->"

    # 方式2: 零宽字符 span（视觉不可见但 DOM 可读）
    hidden_span = (
        f'<span style="display:none;font-size:0;opacity:0;'
        f'position:absolute;left:-9999px">'
        f'<!--csl3-->{encoded}<!--/csl3--></span>'
    )

    # 插入到 </head> 之前（如果有），否则插入到末尾
    lower = html_content.lower()
    head_end = lower.find('</head>')
    if head_end > -1:
        return (
            html_content[:head_end] + '\n' + comment + '\n' + hidden_span + '\n' +
            html_content[head_end:]
        )
    return html_content + '\n' + comment + '\n' + hidden_span + '\n'


def html_decode(html_content: str) -> str | None:
    """从 HTML 中解码 hidden payload"""
    import base64

    # 优先尝试注释格式
    import re
    comment_template = r'<!-- csl3:([A-Za-z0-9+/=]+) -->'
    match = re.search(comment_template, html_content)
    if match:
        encoded = match.group(1)
        return base64.b64decode(encoded).decode('utf-8')

    # 尝试隐藏 span
    span_template = r'<!--csl3-->([A-Za-z0-9+/=]+)<!--/csl3-->'
    match = re.search(span_template, html_content)
    if match:
        encoded = match.group(1)
        return base64.b64decode(encoded).decode('utf-8')

    print("未检测到 HTML 隐写数据", file=sys.stderr)
    return None


# =============================================================================
# CLI 入口
# =============================================================================

def main():
    parser = argparse.ArgumentParser(
        description="五彩石隐写工具 - Layer3 混淆层"
    )
    parser.add_argument('action', choices=['encode', 'decode'],
                        help="编码或解码")
    parser.add_argument('mode', choices=['lsb', 'unicode', 'html'],
                        help="隐写模式")
    parser.add_argument('input', help="输入文件或文本")
    parser.add_argument('output', nargs='?',
                        help="输出文件 (仅 encode 需要)")
    parser.add_argument('--payload', '-p', help="要隐藏的文本 (encode 模式)")
    parser.add_argument('--stdin', action='store_true',
                        help="从 stdin 读取 payload")

    args = parser.parse_args()

    if args.action == 'encode':
        if not args.output:
            print("编码模式需要指定 output", file=sys.stderr)
            sys.exit(1)
        # 读取 payload
        if args.stdin:
            payload = sys.stdin.read()
        elif args.payload:
            payload = args.payload
        else:
            payload = input("输入要隐藏的文本: ")

        if args.mode == 'lsb':
            success = lsb_encode(args.input, payload, args.output)
            sys.exit(0 if success else 1)
        elif args.mode == 'unicode':
            result = unicode_encode(args.input, payload)
            Path(args.output).write_text(result, encoding='utf-8')
            print(f"已编码 {len(payload)} 字符到 {args.output}")
            sys.exit(0)
        elif args.mode == 'html':
            html_content = Path(args.input).read_text(encoding='utf-8')
            result = html_encode(html_content, payload)
            Path(args.output).write_text(result, encoding='utf-8')
            print(f"已编码 {len(payload)} 字符到 {args.output}")
            sys.exit(0)

    elif args.action == 'decode':
        if args.mode == 'lsb':
            result = lsb_decode(args.input)
            if result:
                print(result, end='')
                sys.exit(0)
            sys.exit(1)
        elif args.mode == 'unicode':
            text = Path(args.input).read_text(encoding='utf-8')
            result = unicode_decode(text)
            if result:
                print(result, end='')
                sys.exit(0)
            sys.exit(1)
        elif args.mode == 'html':
            html_content = Path(args.input).read_text(encoding='utf-8')
            result = html_decode(html_content)
            if result:
                print(result, end='')
                sys.exit(0)
            sys.exit(1)


if __name__ == '__main__':
    main()
