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

import sys, os, struct, json
from fontTools.fontBuilder import FontBuilder
from fontTools.pens.ttGlyphPen import TTGlyphPen
from fontTools.ttLib import TTFont

SEMANTIC_ID_BASE = 0xE000

SEMANTIC_IDS = [
    "SEM_NOP", "SEM_ADD", "SEM_SUB", "SEM_MOV", "SEM_AND",
    "SEM_OR",  "SEM_XOR", "SEM_SHL", "SEM_SHR", "SEM_CMP",
    "SEM_JMP", "SEM_CALL", "SEM_RET",  "SEM_LOAD","SEM_STORE",
    "SEM_PUSH", "SEM_POP", "SEM_INT", "SEM_SYSCALL", "SEM_LUI",
    "SEM_AUIPC", "SEM_FENCE", "SEM_CSR", "SEM_MUL", "SEM_DIV",
    "SEM_REM", "SEM_AMOADD", "SEM_AMOSWAP", "SEM_LR", "SEM_SC",
]

TARGET_MAPS = {
    "rv32i": {
        "SEM_NOP":     [0x13, 0x00, 0x00, 0x00],
        "SEM_ADD":     [0xB3, 0x00, 0x30, 0x00],
        "SEM_SUB":     [0xB3, 0x40, 0x30, 0x00],
        "SEM_AND":     [0xB3, 0x70, 0x30, 0x00],
        "SEM_OR":      [0xB3, 0x60, 0x30, 0x00],
        "SEM_XOR":     [0xB3, 0x40, 0x30, 0x00],
        "SEM_SHL":     [0xB3, 0x10, 0x30, 0x00],
        "SEM_SHR":     [0xB3, 0x50, 0x30, 0x00],
        "SEM_JMP":     [0x6F, 0x00, 0x00, 0x00],
        "SEM_LOAD":    [0x83, 0x22, 0x00, 0x00],
        "SEM_STORE":   [0x23, 0x20, 0x53, 0x00],
        "SEM_SYSCALL": [0x73, 0x00, 0x00, 0x00],
        "SEM_LUI":     [0x37, 0x00, 0x00, 0x00],
        "SEM_AUIPC":   [0x17, 0x00, 0x00, 0x00],
        "SEM_FENCE":   [0x0F, 0x00, 0x00, 0x00],
    },
    "x86_64": {
        "SEM_NOP":     [0x90],
        "SEM_ADD":     [0x48, 0x01, 0xD8],
        "SEM_SUB":     [0x48, 0x29, 0xD8],
        "SEM_MOV":     [0x48, 0x89, 0xD8],
        "SEM_XOR":     [0x48, 0x31, 0xC0],
        "SEM_CMP":     [0x48, 0x39, 0xD8],
        "SEM_JMP":     [0xEB, 0xFE],
        "SEM_CALL":    [0xE8, 0x00, 0x00, 0x00, 0x00],
        "SEM_RET":     [0xC3],
        "SEM_LOAD":    [0x48, 0x8B, 0x03],
        "SEM_STORE":   [0x48, 0x89, 0x03],
        "SEM_PUSH":    [0x50],
        "SEM_POP":     [0x58],
        "SEM_INT":     [0xCC],
        "SEM_SYSCALL": [0x0F, 0x05],
        "SEM_AND":     [0x48, 0x21, 0xD8],
        "SEM_OR":      [0x48, 0x09, 0xD8],
        "SEM_SHL":     [0x48, 0xD1, 0xE0],
        "SEM_SHR":     [0x48, 0xD1, 0xE8],
    },
    "arm64": {
        "SEM_NOP":     [0x1F, 0x20, 0x03, 0xD5],
        "SEM_ADD":     [0x8B, 0x02, 0x00, 0x20],
        "SEM_SUB":     [0xCB, 0x02, 0x00, 0x20],
        "SEM_MOV":     [0xE0, 0x03, 0x80, 0xAA],
        "SEM_RET":     [0xC0, 0x03, 0x5F, 0xD6],
        "SEM_LOAD":    [0x83, 0x00, 0x40, 0xF9],
        "SEM_STORE":   [0x81, 0x00, 0x00, 0xF9],
        "SEM_JMP":     [0x14, 0x00, 0x00, 0x00],
        "SEM_SYSCALL": [0x01, 0x00, 0x00, 0xD4],
    },
}


def bytes_to_coords(machine_bytes):
    coords = []
    if len(machine_bytes) % 2 != 0:
        machine_bytes = machine_bytes + [0x00]
    for i in range(0, len(machine_bytes), 2):
        coords.append((machine_bytes[i], machine_bytes[i + 1]))
    return coords


def coords_to_bytes(coords):
    result = []
    for x, y in coords:
        result.append(x & 0xFF)
        result.append(y & 0xFF)
    return result


def _make_notdef_glyph():
    pen = TTGlyphPen(None)
    pen.moveTo((50, 0))
    pen.lineTo((50, 700))
    pen.lineTo((450, 700))
    pen.lineTo((450, 0))
    pen.closePath()
    return pen.glyph()


def build_morphism_glyph(machine_bytes, sem_id, em=1024):
    original_len = len(machine_bytes)
    coords = bytes_to_coords(machine_bytes)
    pen = TTGlyphPen(None)
    start_x = original_len
    start_y = sem_id % 256
    pen.moveTo((start_x, start_y))
    for x, y in coords:
        pen.lineTo((x, y))
    pen.closePath()
    return glyph_to_simple(pen.glyph()), coords


def glyph_to_simple(g):
    return g


def generate_morphism_font(target_isa, sem_ids=None, out_path=None, verbose=True):
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
            print(f"  [MAP] {sem_id:12s} -> U+{code_point:04X} -> "
                  f"{target_isa}: {bytes_to_hex(machine_bytes)} "
                  f"({len(coords)} coords)")
    fb = FontBuilder(em, isTTF=True)
    fb.setupGlyphOrder(glyph_order)
    fb.setupGlyf(glyphs)
    fb.setupCharacterMap(cmap)
    fb.setupHorizontalMetrics(metrics)
    fb.setupHorizontalHeader(ascent=em, descent=-200)
    fb.setupNameTable({"familyName": f"Morphism-{target_isa}", "styleName": "Regular"})
    fb.setupOS2(sTypoAscender=em, sTypoDescender=-200, usWinAscent=int(em * 1.2), usWinDescent=200)
    fb.setupPost(formatType=2.0)
    if out_path is None:
        out_path = f"morphism_{target_isa}.ttf"
    fb.font.save(out_path)
    if verbose:
        print(f"\n[OK] {out_path} ({len(mapping_info)} glyphs, ISA={target_isa})")
    return out_path, mapping_info


def extract_morph_mapping(ttf_path, isa_name):
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
        coord_pairs = list(glyph.coordinates)
        if len(coord_pairs) > 1:
            original_len = coord_pairs[0][0]
            machine_coords = coord_pairs[1:]
        else:
            original_len = 0
            machine_coords = coord_pairs
        raw_bytes = coords_to_bytes(machine_coords)[:original_len]
        results.append({
            "glyph_name": glyph_name,
            "code_point": f"U+{code_point:04X}",
            "isa": isa_name,
            "extracted_hex": bytes_to_hex(raw_bytes).rstrip(),
            "coords": machine_coords[:6],
        })
    print(f"\n  === 态射验证: {ttf_path} ({isa_name}) ===")
    print(f"  {'字形':20s} {'码点':10s} {'提取 hex':30s}")
    print(f"  {'─' * 60}")
    for r in results:
        print(f"  {r['glyph_name']:20s} {r['code_point']:10s} {r['extracted_hex']:30s}")
    print(f"\n  === 与原始 TARGET_MAPS 比对 ===")
    ok_count = 0
    for sem_id in SEMANTIC_IDS:
        if sem_id not in tmap:
            continue
        expected_hex = bytes_to_hex(tmap[sem_id]).rstrip()
        matching = [r for r in results if r["glyph_name"] == sem_id.lower()]
        if not matching:
            continue
        extracted_hex = matching[0]["extracted_hex"]
        match = extracted_hex.upper().replace(" ", "") == expected_hex.upper().replace(" ", "")
        mark = "PASS" if match else "FAIL"
        if match:
            ok_count += 1
        print(f"  {sem_id:14s} exp: {expected_hex:30s} got: {extracted_hex:30s} {mark}")
    total = len([s for s in SEMANTIC_IDS if s in tmap])
    print(f"\n  比对结果: {ok_count}/{total} PASS")
    return ok_count == total


def bytes_to_hex(byte_list):
    return " ".join(f"{b:02X}" for b in byte_list)


def main():
    import argparse
    parser = argparse.ArgumentParser(description="态射字体生成与验证 (刘楚恬和我)")
    sub = parser.add_subparsers(dest="cmd")
    g = sub.add_parser("generate")
    g.add_argument("isa", choices=["rv32i", "x86_64", "arm64"])
    g.add_argument("--out", default=None)
    g.add_argument("--quiet", action="store_true")
    v = sub.add_parser("verify")
    v.add_argument("ttf_path")
    v.add_argument("isa")
    e = sub.add_parser("demo")
    e.add_argument("--isa", default="rv32i")
    args = parser.parse_args()
    if args.cmd == "generate":
        generate_morphism_font(args.isa, out_path=args.out, verbose=not args.quiet)
    elif args.cmd == "verify":
        extract_morph_mapping(args.ttf_path, args.isa)
    elif args.cmd == "demo":
        print("=== 态射字体 Demo ===\n")
        for isa in ["rv32i", "x86_64", "arm64"]:
            print(f"\n--- {isa} ---")
            path, info = generate_morphism_font(isa, verbose=False)
            if path:
                all_ok = extract_morph_mapping(path, isa)
                print(f"  [{isa}] 态射验证: {'PASS' if all_ok else 'FAIL'}\n")
    else:
        parser.print_help()


if __name__ == "__main__":
    main()
