#!/usr/bin/env python3
"""
gen_sdf_font.py — 生成囡囝共生 SDF 字体 atlas

输入:
  - 民联词典 179 句 (data/minlian.json)
  - 代码中所有汉字 (kernel/, morph/)
  - wqy-zenhei.ttc 系统 TrueType 字体

产物:
  - font/mqf_font.bin    (二进制 SDF atlas, < 500 KB)
  - font/mqf_font.png    (调试用的 atlas 可视化)
  - font/mqf_font.h      (C 头文件: 字形表查找结构)

用法:
  python3 encoding/gen_sdf_font.py
"""
import json
import os
import sys
import re
import struct
from PIL import Image, ImageDraw, ImageFont

# ═══════════════════════════════════════════════════════════
# 1. 提取所需字符集
# ═══════════════════════════════════════════════════════════
def extract_charset():
    """收集实际使用的全部唯一汉字。"""
    chars = set()

    # minlian.json (基础词汇)
    data_path = os.path.join(os.path.dirname(__file__), "..", "data", "minlian.json")
    if os.path.exists(data_path):
        with open(data_path) as f:
            items = json.load(f)
        for it in items:
            chars.update(it["word"])
            chars.update(it["source"])
            chars.update(re.findall(r'[\u4e00-\u9fff]', it.get("note", "")))

    # kernel/*.py
    for d in ["kernel", "morph"]:
        dp = os.path.join(os.path.dirname(__file__), "..", d)
        if not os.path.exists(dp):
            continue
        for fn in os.listdir(dp):
            if not fn.endswith(".py"):
                continue
            with open(os.path.join(dp, fn)) as f:
                chars.update(re.findall(r'[\u4e00-\u9fff]', f.read()))

    # android/src/**/*.kt
    for root, _, files in os.walk(os.path.join(os.path.dirname(__file__), "..", "android")):
        for fn in files:
            if fn.endswith((".kt", ".java")):
                with open(os.path.join(root, fn)) as f:
                    chars.update(re.findall(r'[\u4e00-\u9fff]', f.read()))

    # font/ 保底
    chars.update("囡囝囡囝共生元灵犀协议立约合一递归")

    # emoji / 符号
    chars.update("①②③④⑤⑥⑦⑧⑨⑩")
    chars.update("→←↑↓↔↣⇒")

    return sorted(chars)

# ═══════════════════════════════════════════════════════════
# 2. 字体的渲染
# ═══════════════════════════════════════════════════════════
def pick_font(size_pt=36):
    """选择能渲染 CJK 字体备选项> wqy-zenhei 是 Linux 最常见。"""
    candidates = [
        "/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
    ]
    for p in candidates:
        if os.path.exists(p):
            return ImageFont.truetype(p, size_pt, index=0), p
    raise FileNotFoundError("无可用的 CJK 字体。")

FONT_PX = 32       # 渲染分辨率 (高采样, TILE=12 → 32/12 降采样比)
TILE = 12          # atlas tile 尺寸 (SDF 网格, 12×12=144B/glyph)
PAD  = 2           # tile 间 padding (cell = TILE+PAD)
SDF_SCALE = 0.5    # SDF 实际分辨率 = TILE * SDF_SCALE (8×8)
INF  = 9999.0

def make_sdf_for_glyph(ch, font):
    """渲染单个字形, 返回 (sdf_grid, advance_w_tiles, bearing_x, bearing_y)。"""
    size = FONT_PX
    img = Image.new("L", (size * 2, size * 2), 0)
    draw = ImageDraw.Draw(img)
    bbox = draw.textbbox((0, 0), ch, font=font)
    x0, y0, x1, y1 = bbox
    w, h = x1 - x0, y1 - y0
    ox, oy = size - x0 - w // 2, size - y0 - h // 2
    draw.text((ox, oy), ch, fill=255, font=font)

    # 降采样到 TILE × TILE
    img_small = img.resize((TILE, TILE), Image.LANCZOS)

    # 二值化 + 计算 SDF
    px = img_small.load()
    sdf = [[0.0] * TILE for _ in range(TILE)]
    for y in range(TILE):
        for x in range(TILE):
            inside = 1 if px[x, y] > 128 else 0
            if inside:
                sdf[y][x] = find_nearest_edge_outside(px, x, y, TILE)
            else:
                sdf[y][x] = -find_nearest_edge_inside(px, x, y, TILE)

    # 归一化到 0-255
    out = [[0] * TILE for _ in range(TILE)]
    for y in range(TILE):
        for x in range(TILE):
            s = sdf[y][x]
            norm = (s + TILE) / (2 * TILE)
            out[y][x] = max(0, min(255, int(norm * 255)))

    # advance in tile units (TILE pixels per tile), clamp to 1-4 → fits u8
    adv_w = max(1, min(255, int((x1 - x0) / TILE * 1.2)))
    bx = max(-127, min(127, 1))
    by = max(-127, min(127, -(y1 - y0) // 2 - 2))
    return out, adv_w, bx, by

def find_nearest_edge_outside(px, x, y, N):
    """向外搜索最近 0 像素的距离。"""
    best = INF
    for dy in range(-TILE, TILE + 1):
        for dx in range(-TILE, TILE + 1):
            nx, ny = x + dx, y + dy
            if nx < 0 or ny < 0 or nx >= N or ny >= N:
                continue
            if px[nx, ny] <= 128:
                d = (dx * dx + dy * dy) ** 0.5
                if d < best:
                    best = d
    return min(best, TILE)

def find_nearest_edge_inside(px, x, y, N):
    """向内搜索最近 255 像素的距离。"""
    best = INF
    for dy in range(-TILE, TILE + 1):
        for dx in range(-TILE, TILE + 1):
            nx, ny = x + dx, y + dy
            if nx < 0 or ny < 0 or nx >= N or ny >= N:
                continue
            if px[nx, ny] > 128:
                d = (dx * dx + dy * dy) ** 0.5
                if d < best:
                    best = d
    return min(best, TILE)

# ═══════════════════════════════════════════════════════════
# 3. 打包 atlas
# ═══════════════════════════════════════════════════════════
ATLAS_COLS = 16
ATLAS_ROWS = 16
ATLAS_W = ATLAS_COLS * (TILE + PAD)
ATLAS_H = ATLAS_ROWS * (TILE + PAD)
GLYPHS_PER_ATLAS = ATLAS_COLS * ATLAS_ROWS

def pack_atlases(glyphs):
    """glyphs = [(char, sdf, adv, bx, by), ...]"""
    atlases = []
    meta = []
    for i in range(0, len(glyphs), GLYPHS_PER_ATLAS):
        chunk = glyphs[i:i + GLYPHS_PER_ATLAS]
        atlas = Image.new("L", (ATLAS_W, ATLAS_H), 128)  # 128 = 中性灰 (SDF=0)
        for j, (ch, sdf, adv, bx, by) in enumerate(chunk):
            cx = (j % ATLAS_COLS) * (TILE + PAD)
            cy = (j // ATLAS_COLS) * (TILE + PAD)
            for py in range(TILE):
                for px in range(TILE):
                    atlas.putpixel((cx + px, cy + py), sdf[py][px])
            meta.append({
                "atlas_ix": i // GLYPHS_PER_ATLAS,
                "col": j % ATLAS_COLS,
                "row": j // ATLAS_COLS,
                "adv": adv,
                "bx": bx,
                "by": by,
            })
        atlases.append(atlas)
    return atlases, meta

# ═══════════════════════════════════════════════════════════
# 4. 文件输出
# ═══════════════════════════════════════════════════════════
def write_bin(atlases, meta, out_path):
    """二进制格式:
       magic(4) GLYPH_COUNT(u32) TILE(u32) ATLAS_W(u32) ATLAS_H(u32) ATLAS_COUNT(u32)
       [per_glyph: atlas_ix u8, col u8, row u8, adv u8, bx i8, by i8, pad u8]
       [atlas_pixels: ATLAS_COUNT * ATLAS_W * ATLAS_H bytes]
    """
    with open(out_path, "wb") as f:
        f.write(b"MQF1")
        f.write(struct.pack("<I", len(meta)))
        f.write(struct.pack("<I", TILE))
        f.write(struct.pack("<I", ATLAS_W))
        f.write(struct.pack("<I", ATLAS_H))
        f.write(struct.pack("<I", len(atlases)))
        for m in meta:
            # atlas_ix,col,row = u8 (max 255 atlases, 255 tiles)
            # adv = u8 (low-half unsigned fits up to 255)
            # bx,by = i8 signed (bearing can be negative)
            f.write(struct.pack("<BBBBbb",
                                m["atlas_ix"], m["col"], m["row"],
                                m["adv"], m["bx"], m["by"]))
            f.write(b"\0")  # 1 byte pad → total 7 bytes/glyph
        for img in atlases:
            f.write(img.tobytes())

def write_h(meta, out_path):
    with open(out_path, "w") as f:
        f.write("/* 自动生成的 SDF 字形表 — 由 encoding/gen_sdf_font.py 生成 */\n")
        f.write("#ifndef MQF_FONT_H\n#define MQF_FONT_H\n\n")
        f.write("#include <stdint.h>\n\n")
        f.write(f"#define MQF_FONT_TILE      {TILE}\n")
        f.write(f"#define MQF_FONT_ATLAS_W   {ATLAS_W}\n")
        f.write(f"#define MQF_FONT_ATLAS_H   {ATLAS_H}\n")
        f.write(f"#define MQF_FONT_PH_COUNT  {len(meta)}\n\n")
        f.write("typedef struct {\n")
        f.write("    uint8_t atlas_ix, col, row;\n")
        f.write("    uint8_t adv;\n")
        f.write("    int8_t  bx, by;\n")
        f.write("    uint8_t pad;\n")
        f.write("};\n\n")
        f.write(f"/* extern const MQF_FONT_ALIGN MqfGlyph MQF_FONT_TABLE[{len(meta)}]; */\n")
        f.write("#endif /* MQF_FONT_H */\n")

# ═══════════════════════════════════════════════════════════
# 5. 主流程
# ═══════════════════════════════════════════════════════════
def main():
    print(f"═══ 囡囝共生 SDF 字体生成 ═══")

    chars = extract_charset()
    print(f"[1] 总字符数: {len(chars)}")

    font, font_path = pick_font(FONT_PX)
    print(f"[2] 字体: {font_path} @{FONT_PX}px")

    glyphs = []; failures = 0
    for ch in chars:
        try:
            sdf, adv, bx, by = make_sdf_for_glyph(ch, font)
            glyphs.append((ch, sdf, adv, bx, by))
        except Exception as e:
            failures += 1
    print(f"[3] 渲染成功: {len(glyphs)}, 失败: {failures}")

    atlases, meta = pack_atlases(glyphs)
    print(f"[4] atlas 数: {len(atlases)} ({ATLAS_W}×{ATLAS_H})")

    out_dir = os.path.join(os.path.dirname(__file__), "..", "font")
    os.makedirs(out_dir, exist_ok=True)

    bin_path = os.path.join(out_dir, "mqf_font.bin")
    write_bin(atlases, meta, bin_path)
    h_path = os.path.join(out_dir, "mqf_font.h")
    write_h(meta, h_path)

    total_bytes = os.path.getsize(bin_path)
    print(f"[5] 二进制: {bin_path} ({total_bytes} bytes = {total_bytes/1024:.1f} KB)")

    # debug PNG
    for i, img in enumerate(atlases):
        img.save(os.path.join(out_dir, f"mqf_font_{i}.png"))
    print(f"[6] debug PNG: font/mqf_font_0..{len(atlases)-1}.png")
    print(f"[7] 头文件: {h_path}")

    return 0 if total_bytes < 500_000 else 1

if __name__ == "__main__":
    sys.exit(main())
