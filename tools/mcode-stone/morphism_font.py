#!/usr/bin/env python3
# ============================================================
# 态射字体 (Morphism Font)
#
# 核心命题: 字形的轮廓坐标 = 目标机器码的位序列
#
# 不存储机器码, 不封装元数据, 不做翻译.
# 字体的 glyph outline 经过标准渲染器解析后,
# 输出的 int16 坐标值拼接起来就是目标 ISA 的操作码.
#
# 这就是"态射": 不是 A 翻译成 B, 而是 A 的结构本身就是 B.
#
# 身份落款: 刘楚恬和我
# ============================================================

import sys
import os
import struct
import json
from fontTools.fontBuilder import FontBuilder
from fontTools.pens.ttGlyphPen import TTGlyphPen
from fontTools.ttLib import TTFont

# ────────────────────────────────────────────────────────────
# 第 一层: 自己的语义 (SMRL 公约数)
# ────────────────────────────────────────────────────────────
# 每个语义 ID 对应一个字形. 字形编号 = 语义 ID (U+E000 起).
# 语义 ID 的定义在 hunyuan-os 的容器编码体系中,
# 不依赖任何外部 ISA 规范.

SEMANTIC_ID_BASE = 0xE000  # 私有使用区起始

SEMANTIC_IDS = [
    "SEM_NOP", "SEM_ADD", "SEM_SUB", "SEM_MOV", "SEM_AND",
    "SEM_OR",  "SEM_XOR", "SEM_SHL", "SEM_SHR", "SEM_CMP",
    "SEM_JMP", "SEM_CALL", "SEM_RET",  "SEM_LOAD","SEM_STORE",
    "SEM_PUSH", "SEM_POP", "SEM_INT", "SEM_SYSCALL", "SEM_LUI",
    "SEM_AUIPC", "SEM_FENCE", "SEM_CSR", "SEM_MUL", "SEM_DIV",
    "SEM_REM", "SEM_AMOADD", "SEM_AMOSWAP", "SEM_LR", "SEM_SC",
]

# ────────────────────────────────────────────────────────────
# 第 二层: 目标映射 (态射窗口)
# ────────────────────────────────────────────────────────────
# 每个语义 ID 对应各 ISA 的一条指令及其机器码.
# 这里存放的是「经过态射变换后会成为目标操作码」的源字节序列.
# 源字节 → 字体坐标 → 标准渲染器输出 → 位序列 = 目标操作码.
#
# 映射表是可替换的: 换一个目标芯片, 只换映射表,
# 自己的语义空间完全不动.

TARGET_MAPS = {
    "rv32i": {
        # 语义 ID → RV32I 机器码 (4 字节, 小端来源到大端排列)
        "SEM_NOP":     [0x13, 0x00, 0x00, 0x00],  # addi x0,x0,0
        "SEM_ADD":     [0xB3, 0x00, 0x30, 0x00],  # ADD x1,x2,x3 → 0x003100B3 → 大端: 00 31 00 B3 → 反序
        "SEM_SUB":     [0xB3, 0x40, 0x30, 0x00],  # SUB
        "SEM_AND":     [0xB3, 0x70, 0x30, 0x00],  # AND
        "SEM_OR":      [0xB3, 0x60, 0x30, 0x00],  # OR
        "SEM_XOR":     [0xB3, 0x40, 0x30, 0x00],  # XOR (实际 funct3=4 → opcode=03 rs2 00 funct3 00 rd 33)
        "SEM_SHL":     [0xB3, 0x10, 0x30, 0x00],  # SLL
        "SEM_SHR":     [0xB3, 0x50, 0x30, 0x00],  # SRL
        "SEM_LOAD":    [0x83, 0x22, 0x00, 0x00],  # LW x5,0(x6) → 0x00032283
        "SEM_STORE":   [0x23, 0x20, 0x53, 0x00],  # SW x5,0(x6)
        "SEM_JMP":     [0x6F, 0x00, 0x00, 0x00],  # JAL x0,offset
        "SEM_LUI":     [0x37, 0x00, 0x00, 0x00],  # LUI x0,imm
        "SEM_AUIPC":   [0x17, 0x00, 0x00, 0x00],  # AUIPC x0,imm
        "SEM_FENCE":   [0x0F, 0x00, 0x00, 0x00],  # FENCE
        "SEM_SYSCALL": [0x73, 0x00, 0x00, 0x00],  # ECALL
    },
    "x86_64": {
        # 语义 ID → x86-64 机器码 (1-15 字节, 按实际指令长度)
        "SEM_NOP":     [0x90],
        "SEM_ADD":     [0x48, 0x01, 0xD8],              # add rax, rbx
        "SEM_SUB":     [0x48, 0x29, 0xD8],              # sub rax, rbx
        "SEM_MOV":     [0x48, 0x89, 0xD8],              # mov rax, rbx
        "SEM_XOR":     [0x48, 0x31, 0xC0],              # xor rax, rax
        "SEM_CMP":     [0x48, 0x39, 0xD8],              # cmp rax, rbx
        "SEM_JMP":     [0xEB, 0xFE],                    # jmp short self
        "SEM_CALL":    [0xE8, 0x00, 0x00, 0x00, 0x00],  # call rel32
        "SEM_RET":     [0xC3],
        "SEM_LOAD":    [0x48, 0x8B, 0x03],              # mov rax, [rbx]
        "SEM_STORE":   [0x48, 0x89, 0x03],              # mov [rbx], rax
        "SEM_PUSH":    [0x50],                          # push rax
        "SEM_POP":     [0x58],                          # pop rax
        "SEM_INT":     [0xCC],                          # int3
        "SEM_SYSCALL": [0x0F, 0x05],                    # syscall
        "SEM_AND":     [0x48, 0x21, 0xD8],
        "SEM_OR":      [0x48, 0x09, 0xD8],
        "SEM_SHL":     [0x48, 0xD1, 0xE0],              # shl rax, 1
        "SEM_SHR":     [0x48, 0xD1, 0xE8],              # shr rax, 1
    },
    "arm64": {
        "SEM_NOP":     [0x1F, 0x20, 0x03, 0xD5],        # NOP
        "SEM_ADD":     [0x8B, 0x02, 0x00, 0x20],        # ADD x0, x1, x2 (低字节在前)
        "SEM_SUB":     [0xCB, 0x02, 0x00, 0x20],
        "SEM_MOV":     [0xE0, 0x03, 0x80, 0xAA],        # ORR x0, xzr, x1
        "SEM_RET":     [0xC0, 0x03, 0x5F, 0xD6],        # RET
        "SEM_LOAD":    [0x83, 0x00, 0x40, 0xF9],        # LDR x1, [x2]
        "SEM_STORE":   [0x81, 0x00, 0x00, 0xF9],        # STR x1, [x2]
        "SEM_JMP":     [0x14, 0x00, 0x00, 0x00],        # B offset (未重定位)
        "SEM_SYSCALL": [0x01, 0x00, 0x00, 0xD4],        # SVC #0
    },
}

# ────────────────────────────────────────────────────────────
# 第 三层: 态射器 (字节 → 坐标)
# ────────────────────────────────────────────────────────────
# 把机器码字节序列转换为 TrueType 轮廓点.
# 每 2 字节 = 一个 (x, y) 坐标对.
# 如果字节序列为奇数长度, 末尾补 0.
#
# 标准 TrueType 渲染器解析 glyph 时:
#   1. 读取 flag 数组 (on-curve/off-curve)
#   2. 读取 x 坐标 (delta 或绝对)
#   3. 读取 y 坐标
# 我们使用最简单编码: 所有点 on-curve, 绝对坐标.


def bytes_to_coords(machine_bytes):
    """将机器码字节序列 → [(x,y), ...] 坐标列表

    配对规则:
      byte[0],byte[1] → point[0].(hi=x, lo=y) ── 实际存储为小端 int16
      即 0x48 0x01 → int16 = 0x0148 (小端字节序)

    TrueType glyf 表使用 big-endian int16 存储坐标,
    但我们通过 FontBuilder 的高级 API, 自动处理字节序.

    返回: list of (x, y) int pairs
    """
    coords = []
    n = len(machine_bytes)
    # 确保偶数长度
    if n % 2 != 0:
        machine_bytes = machine_bytes + [0x00]

    for i in range(0, len(machine_bytes), 2):
        b0 = machine_bytes[i]
        b1 = machine_bytes[i + 1]
        # 小端 int16 → 还原为自然顺序 (b0 为低字节, b1 为高字节)
        val = b0 | (b1 << 8)
        # 拆分回 x = 低字节, y = 高字节
        x = b0
        y = b1
        coords.append((x, y))
    return coords


def coords_to_bytes(coords):
    """将坐标列表还原为机器码字节序列"""
    result = []
    for x, y in coords:
        result.append(x & 0xFF)
        result.append(y & 0xFF)
    return result


def build_morphism_glyph(machine_bytes, sem_id, em=1024):
    """构建单个态射字形.

    策略:
      - 机器码字节 → 等量的坐标点
      - 每个坐标点画一条线段 (从上一个点到此点)
      - 最终闭合路径
      - 所有点 on-curve (flag=1), 无 off-curve 控制点

    输出: 可被任意标准 TTF 渲染器解析的 glyph 对象
    """
    coords = bytes_to_coords(machine_bytes)

    # 使用 fontTools 的 TTGlyphPen 绘制轮廓
    from fontTools.pens.ttGlyphPen import TTGlyphPen
    pen = TTGlyphPen(None)

    # 轮廓起点 (padding point)
    start_x = 64  # 非零起点, 区分语义
    start_y = sem_id % 256  # 语义 ID 编码到 y 坐标

    pen.moveTo((start_x, start_y))
    for x, y in coords:
        # 每个字节对成为一个坐标点 (直线段)
        pen.lineTo((x, y))
    pen.closePath()

    glyph = pen.glyph()
    return glyph, coords


def generate_morphism_font(target_isa, sem_ids=None, out_path=None, verbose=True):
    """生成一个完整的态射字体.

    参数:
      target_isa: "rv32i" | "x86_64" | "arm64"
      sem_ids: 要包含的语义 ID 列表 (None = 全部)
      out_path: 输出 TTF 路径

    返回: (font_path, 映射信息列表)
    """
    if target_isa not in TARGET_MAPS:
        print(f"[ERROR] 不支持目标 ISA: {target_isa}")
        return None, None

    tmap = TARGET_MAPS[target_isa]
    if sem_ids is None:
        sem_ids = [s for s in SEMANTIC_IDS if s in tmap]

    em = 1024
    glyph_order = [".notdef"]
    glyphs = {".notdef": _make_notdef_glyph()}
    cmap = {}
    metrics = {".notdef": (500, 0)}
    mapping_info = []

    for idx, sem_id in enumerate(sem_ids):
        if sem_id not in tmap:
            continue

        machine_bytes = tmap[sem_id]
        code_point = SEMANTIC_ID_BASE + idx
        glyph_name = sem_id.lower()

        glyph_order.append(glyph_name)
        cmap[code_point] = glyph_name
        metrics[glyph_name] = (em, 0)

        glyph, coords = build_morphism_glyph(machine_bytes, idx, em)
        glyphs[glyph_name] = glyph

        rec = {
            "sem_id": sem_id,
            "code_point": f"U+{code_point:04X}",
            "glyph_name": glyph_name,
            "isa": target_isa,
            "machine_bytes_hex": bytes_to_hex(machine_bytes),
            "byte_len": len(machine_bytes),
            "coords_count": len(coords),
            "coords_preview": coords[:4],
        }
        mapping_info.append(rec)

        if verbose:
            print(f"  [MAP] {sem_id:12s} → U+{code_point:04X} → "
                  f"{target_isa}: {bytes_to_hex(machine_bytes)} "
                  f"({len(coords)} coords)")

    # 构建字体
    fb = FontBuilder(em, isTTF=True)
    fb.setupGlyphOrder(glyph_order)
    fb.setupGlyf(glyphs)
    fb.setupCharacterMap(cmap)
    fb.setupHorizontalMetrics(metrics)
    fb.setupHorizontalHeader(ascent=em, descent=-200)
    fb.setupNameTable({
        "familyName": f"Morphism-{target_isa}",
        "styleName": "Regular",
    })
    fb.setupOS2(
        sTypoAscender=em,
        sTypoDescender=-200,
        usWinAscent=int(em * 1.2),
        usWinDescent=200,
    )
    fb.setupPost(formatType=2.0)

    if out_path is None:
        out_path = f"morphism_{target_isa}.ttf"

    fb.font.save(out_path)

    if verbose:
        print(f"\n[OK] 态射字体: {out_path}")
        print(f"     包含字形: {len(mapping_info)}")
        print(f"     目标 ISA: {target_isa}")

    return out_path, mapping_info


def extract_morph_mapping(ttf_path, isa_name, sem_ids=None):
    """从态射字体中提取: 字形坐标 → 机器码

    验证路径: TTF → glyf 表 → 坐标 → 字节序列
    如果提取结果与原始 TARGET_MAPS 一致 → 态射成立
    """
    font = TTFont(ttf_path)
    glyf_table = font["glyf"]
    cmap_table = font.getBestCmap()

    tmap = TARGET_MAPS.get(isa_name, {})
    results = []

    for code_point, glyph_name in sorted(cmap_table.items()):
        if glyph_name not in glyf_table:
            continue
        glyph = glyf_table[glyph_name]
        if glyph.numberOfContours == 0:
            continue

        coords = []
        # 提取所有端点坐标 (接收绝对坐标)
        end_pts = glyph.endPtsOfContours
        x_coords = glyph.coordinates.x
        y_coords = glyph.coordinates.y

        for i in range(len(x_coords)):
            if i in end_pts or i == 0:
                coords.append((x_coords[i], y_coords[i]))

        # 去掉第一个 padding 点 (start_x, start_y=sem_id%256)
        if len(coords) > 1:
            machine_coords = coords[1:]
        else:
            machine_coords = coords

        raw_bytes = coords_to_bytes(machine_coords)

        # 查找对应的语义 ID
        sem_idx = code_point - SEMANTIC_ID_BASE
        SEMANTIC_IDS[sem_idx] if 0 <= sem_idx < len(SEMANTIC_IDS) else "?"

        results.append({
            "glyph_name": glyph_name,
            "code_point": f"U+{code_point:04X}",
            "isa": isa_name,
            "extracted_hex": bytes_to_hex(raw_bytes).rstrip(),
            "coords": machine_coords[:6],
        })

    return results


def _make_notdef_glyph():
    from fontTools.pens.ttGlyphPen import TTGlyphPen
    pen = TTGlyphPen(None)
    pen.moveTo((50, 0))
    pen.lineTo((50, 700))
    pen.lineTo((450, 700))
    pen.lineTo((450, 0))
    pen.closePath()
    return pen.glyph()


def _extract_fill_raw(ttf_path):
    """直接读取 TTF glyf 表的原始字节, 验证态射位模式."""
    font = TTFont(ttf_path)
    glyf = font["glyf"]
    raw_report = []

    for gn in font.getGlyphOrder():
        if gn == ".notdef":
            continue
        if gn not in glyf:
            continue
        g = glyf[gn]
        if g.numberOfContours <= 0:
            continue
        end_pts = g.endPtsOfContours
        xs = list(g.coordinates.x)
        ys = list(g.coordinates.y)
        # 拼接 x,y 字节 (大端 int16)
        buf = bytearray()
        for i in range(len(xs)):
            buf += struct.pack(">hh", xs[i], ys[i])
        raw_report.append({
            "glyph": gn,
            "end_pts": list(end_pts),
            "num_coords": len(xs),
            "raw_hex": buf[:32].hex(),
        })
    return raw_report


def bytes_to_hex(byte_list):
    return " ".join(f"{b:02X}" for b in byte_list)


# ────────────────────────────────────────────────────────────
# CLI 入口
# ────────────────────────────────────────────────────────────
def main():
    import argparse
    parser = argparse.ArgumentParser(description="态射字体生成与验证")
    sub = parser.add_subparsers(dest="cmd")

    g = sub.add_parser("generate", help="生成态射字体")
    g.add_argument("isa", choices=["rv32i", "x86_64", "arm64"])
    g.add_argument("--out", default=None)
    g.add_argument("--quiet", action="store_true")

    v = sub.add_parser("verify", help="字体验证 (坐标 → 机器码)")
    v.add_argument("ttf_path")
    v.add_argument("isa")

    r = sub.add_parser("raw", help="查看 glyf 原始位模式")
    r.add_parser("raw", help="查看 glyf 原始位模式")

    args = parser.parse_args()

    if args.cmd == "generate":
        generate_morphism_font(
            args.isa,
            out_path=args.out,
            verbose=not args.quiet,
        )
    elif args.cmd == "verify":
        results = extract_morph_mapping(args.ttf_path, args.isa)
        print(f"\n  === 态射验证: {args.ttf_path} ({args.isa}) ===")
        print(f"  {'字形':20s} {'码点':10s} {'提取 hex':30s}")
        print(f"  {'─' * 60}")
        for r in results:
            print(f"  {r['glyph_name']:20s} {r['code_point']:10s} {r['extracted_hex']:30s}")
            print(f"    coords preview: {r['coords'][:4]}")

        # 与已知原始字节交叉比对
        tmap = TARGET_MAPS.get(args.isa, {})
        print(f"\n  === 与原始 TARGET_MAPS 比对 ===")
        ok_count = 0
        for sem_id in SEMANTIC_IDS:
            if sem_id not in tmap:
                continue
            expected_bytes = tmap[sem_id]
            expected_hex = bytes_to_hex(expected_bytes).rstrip()
            # 在 results 中查找 matching glyph
            matching = [r for r in results if r["glyph_name"] == sem_id.lower()]
            if not matching:
                continue
            extracted_hex = matching[0]["extracted_hex"]
            match = "✅" if extracted_hex.upper().replace(" ", "") == expected_hex.upper().replace(" ", "") else "❌"
            if "✅" in match:
                ok_count += 1
            print(f"  {sem_id:14s} 期望: {expected_hex:30s} 提取: {extracted_hex:30s} {match}")
        total = len([s for s in SEMANTIC_IDS if s in tmap])
        print(f"\n  比对结果: {ok_count}/{total} 通过")

    elif args.cmd == "raw":
        ttf_path = args.ttf_path if hasattr(args, 'ttf_path') else (sys.argv[2] if len(sys.argv) > 2 else None)
        raw = _extract_fill_raw(ttf_path)
        for r in raw:
            print(f"  {r['glyph']:20s} pts={r['num_coords']:3d} end_pts={r['end_pts']} raw={r['raw_hex']}")
    else:
        parser.print_help()
        print("\n高层概念:")
        print("  1. 自己的语义空间 (SEMANTIC_IDS) — 完全独立, 不依赖外部")
        print("  2. 目标映射表 (TARGET_MAPS) — 可替换, 不同芯片用不同表")
        print("  3. 态射器 (bytes_to_coords) — 字节 → 轮廓坐标")
        print("  4. 字体的 glyf 表是载体, 标准渲染器是执行通道")
        print("  5. 芯片的字体解析器读出坐标 = 直接获得操作码")


if __name__ == "__main__":
    main()
