"""
font_builder.py — 字体容器生成器（预加载·容器封装·离线一次性）

将语义 ID + 机器码映射封装为 OpenType TrueType 字体文件：
  - 每个字形占位 = 一个语义 ID 的可视壳（矩形框 + 内部符号区）
  - MCOD 自定义表 = 机器码映射字典（直接二进制嵌入字体）
  - cmap 表 = PUA Unicode → 字形名的双向索引

注意：字体构建是离线一次性步骤（预加载冰箱模型），运行时只读。
构建脚本依赖 fontTools（离线工具链），不含于最终字体运行时。

容器落款：刘楚恬和我 · 私有编码系统
"""

import json
import os
import struct
from fontTools.ttLib import newTable
from fontTools.fontBuilder import FontBuilder
from fontTools.pens.ttGlyphPen import TTGlyphPen

CONTAINER_ROOT = os.path.dirname(os.path.abspath(__file__))
SMC_ROOT = os.path.dirname(CONTAINER_ROOT)

PUA_BASE = 0xE000


def load_mappings(arch: str) -> dict:
    """预加载指定架构的映射 JSON。"""
    path = os.path.join(SMC_ROOT, "mappings", f"{arch}.json")
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def load_semantic_ids() -> dict:
    """预加载所有语义族。"""
    ids_dir = os.path.join(SMC_ROOT, "semantic_ids")
    families = {}
    for f in os.listdir(ids_dir):
        if f.endswith(".json"):
            with open(os.path.join(ids_dir, f), "r", encoding="utf-8") as fp:
                families[f.replace(".json", "")] = json.load(fp)
    return families


def _make_placeholder_glyph():
    """生成一个 512×700 的空心矩形占位字形。"""
    pen = TTGlyphPen(None)
    w, h = 512, 700
    pen.moveTo((64, 64))
    pen.lineTo((64 + w, 64))
    pen.lineTo((64 + w, 64 + h))
    pen.lineTo((64, 64 + h))
    pen.closePath()
    pen.moveTo((96, 96))
    pen.lineTo((96, 96 + h - 64))
    pen.lineTo((96 + w - 64, 96 + h - 64))
    pen.lineTo((96 + w - 64, 96))
    pen.closePath()
    return pen.glyph()


def _op_val(inst: dict, key: str) -> int:
    """返回指令字段的整数值（二进制字符串转整数）。"""
    v = inst.get(key, "0")
    if isinstance(v, int):
        return v
    try:
        return int(v, 2)
    except (ValueError, TypeError):
        return 0


def _op_width(inst: dict, key: str) -> int:
    """返回字段的位宽（根据字符串长度推断）。"""
    v = inst.get(key, "")
    if not v:
        return 0
    if isinstance(v, str) and all(c in "01" for c in v):
        return len(v)
    return 16  # 默认 16 位


def build_font(output_path: str = None,
               semantic_ids: dict = None,
               arch_mappings: dict = None):
    """
    构建语义机器码字体容器。

    参数：
        output_path: 输出 .ttf 路径（默认 fonts/semantic_code.ttf）
        semantic_ids: {family_name: family_dict}
        arch_mappings: {arch_name: mapping_dict}
    """
    if semantic_ids is None:
        semantic_ids = load_semantic_ids()
    if arch_mappings is None:
        arch_mappings = {}
        mappings_dir = os.path.join(SMC_ROOT, "mappings")
        for f in os.listdir(mappings_dir):
            if f.endswith(".json"):
                arch = f.replace(".json", "")
                arch_mappings[arch] = load_mappings(arch)

    # ── 汇总所有出现过的语义 ID ──
    all_semantics = set()
    for arch, mapping in arch_mappings.items():
        all_semantics.update(mapping.get("instructions", {}).keys())

    # ── 分配 PUA 码点 ──
    glyph_order = [".notdef"]
    cmap = {}
    sid_to_codepoint = {}

    for i, sid in enumerate(sorted(all_semantics)):
        cp = PUA_BASE + i
        glyph_name = f"sem.{sid}"
        glyph_order.append(glyph_name)
        cmap[cp] = glyph_name
        sid_to_codepoint[sid] = cp

    # ── 构建字体骨架 ──
    fb = FontBuilder(1024, isTTF=True)
    fb.setupGlyphOrder(glyph_order)
    fb.setupCharacterMap(cmap)

    placeholder = _make_placeholder_glyph()
    glyf = {name: placeholder for name in glyph_order}
    metrics = {name: (640, 0) for name in glyph_order}

    fb.setupGlyf(glyf)
    fb.setupHorizontalMetrics(metrics)
    fb.setupHorizontalHeader(ascent=800, descent=-200)
    fb.setupOS2(sTypoAscender=800, sTypoDescender=-200, usWinAscent=1000, usWinDescent=200)
    fb.setupNameTable({
        "familyName": "SemanticCode",
        "styleName": "Regular",
        "uniqueFontIdentifier": "1.000;SemanticCode;Private-SemanticMachineCode",
        "fullName": "SemanticCode Regular",
        "version": "Version 1.000",
        "psName": "SemanticCode-Regular",
    })
    fb.setupPost()

    font = fb.font

    # ── 嵌入 MCOD 自定义二进制表 ──
    mcod_data = _build_mcod_table(arch_mappings)
    font["MCOD"] = newTable("MCOD")
    font["MCOD"].data = mcod_data

    if output_path is None:
        output_path = os.path.join(SMC_ROOT, "fonts", "semantic_code.ttf")
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    font.save(output_path)

    print(f"[font_builder] 已生成字体容器: {output_path}")
    print(f"  - 字形数量: {len(glyph_order)}")
    print(f"  - 语义 ID 数: {len(sid_to_codepoint)}")
    print(f"  - 架构数量: {len(arch_mappings)}")
    print(f"  - MCOD 表大小: {len(mcod_data)} bytes")

    # ─ 生成配套 JSON 索引 ─
    index_path = output_path.replace(".ttf", "_index.json")
    index = {
        "family": "SemanticCode",
        "version": "1.000",
        "cmap": {sid: cp for sid, cp in sid_to_codepoint.items()},
        "architectures": list(arch_mappings.keys()),
    }
    with open(index_path, "w", encoding="utf-8") as fp:
        json.dump(index, fp, indent=2, ensure_ascii=False)
    print(f"  - 索引文件: {index_path}")

    return font


def _build_mcod_table(arch_mappings: dict) -> bytes:
    """
    构建 MCOD 二进制表（v3 格式，双模式编码支持）。

    v3 新增:
      - 类型标记 'R' = RISC 位域模式 (opcode/funct3/funct7/shamt)
      - 类型标记 'C' = CISC opkey 引用模式 (变长前缀 + ModRM)

    文件格式：
      [0:4]   magic = b"MCOD"
      [4:6]   version uint16 = 3
      [6:8]   arch_count uint16
      对每个架构（按名称排序）：
        [0]    arch_name_len uint8
        [1:N]  arch_name UTF-8
        [N:N+2] entry_count uint16
        对每个 instruction（按 sid 排序）：
          [0]           sid_len uint8
          [1:M]         sid UTF-8
          [M]           format ASCII (R/I/S/U/B/J/D/X...)
          [M+1]         type_flag: 'R' 或 'C'
          --- 若 type_flag = 'R' (RISC 位域) ---
          [M+2]         opcode_bits uint8
          [M+3:M+7]     opcode_val uint32
          [M+7]         funct3_bits uint8
          [M+8:M+12]    funct3_val uint32
          [M+12]        funct7_bits uint8
          [M+13:M+17]   funct7_val uint32
          [M+17]        shamt_bits uint8
          [M+18:M+22]   shamt_val uint32
          --- 若 type_flag = 'C' (CISC opkey 引用) ---
          [M+2]         opkey_len uint8
          [M+3:M+K]     opkey UTF-8 (如 "ADD_REG_REG")
    """
    buf = bytearray()
    buf += b"MCOD"
    buf += struct.pack("<HH", 3, len(arch_mappings))

    for arch_name, mapping in sorted(arch_mappings.items()):
        name_bytes = arch_name.encode("utf-8")
        buf += struct.pack("<B", len(name_bytes))
        buf += name_bytes

        instructions = mapping.get("instructions", {})
        buf += struct.pack("<H", len(instructions))

        for sid, inst in sorted(instructions.items()):
            sid_bytes = sid.encode("utf-8")
            buf += struct.pack("<B", len(sid_bytes))
            buf += sid_bytes

            fmt_char = inst.get("format", "X")[:1].encode("ascii")
            buf += fmt_char

            # 判断模式：有 opkey 字段且无 opcode 位域 → CISC 模式
            if "opkey" in inst and "opcode" not in inst:
                buf += b"C"
                opkey_bytes = inst["opkey"].encode("utf-8")
                buf += struct.pack("<B", len(opkey_bytes))
                buf += opkey_bytes
            else:
                buf += b"R"
                op = _op_val(inst, "opcode")
                op_w = _op_width(inst, "opcode")
                f3 = _op_val(inst, "funct3")
                f3_w = _op_width(inst, "funct3")
                f7 = _op_val(inst, "funct7")
                f7_w = _op_width(inst, "funct7")
                sh = _op_val(inst, "shamt")
                sh_w = _op_width(inst, "shamt")

                buf += struct.pack("<BI", op_w, op)
                buf += struct.pack("<BI", f3_w, f3)
                buf += struct.pack("<BI", f7_w, f7)
                buf += struct.pack("<BI", sh_w, sh)

    return bytes(buf)


def parse_mcod_table(mcod_data: bytes) -> dict:
    """从 MCOD 二进制数据反向解析出架构→指令→位域字典（支持 v2/v3）。"""
    if mcod_data[:4] != b"MCOD":
        raise ValueError("无效 MCOD 数据")
    version, arch_count = struct.unpack_from("<HH", mcod_data, 4)
    if version not in (2, 3):
        raise ValueError(f"不支持 MCOD 版本: {version}（只支持 v2/v3）")

    arch_mappings = {}
    offset = 8
    for _ in range(arch_count):
        name_len = mcod_data[offset]; offset += 1
        arch_name = mcod_data[offset:offset + name_len].decode("utf-8"); offset += name_len
        entry_count = struct.unpack_from("<H", mcod_data, offset)[0]; offset += 2

        inst_map = {}
        for _ in range(entry_count):
            sid_len = mcod_data[offset]; offset += 1
            sid = mcod_data[offset:offset + sid_len].decode("utf-8"); offset += sid_len
            fmt_char = chr(mcod_data[offset]); offset += 1

            entry = {"format": fmt_char}

            if version >= 3:
                type_flag = chr(mcod_data[offset]); offset += 1
                if type_flag == "C":
                    # CISC opkey 引用模式
                    ok_len = mcod_data[offset]; offset += 1
                    entry["opkey"] = mcod_data[offset:offset + ok_len].decode("utf-8")
                    offset += ok_len
                    inst_map[sid] = entry
                    continue
                # type_flag == "R' 继续下面位域解析

            # RISC 位域模式 (v2 默认 / v3 'R' 标记)
            op_w = mcod_data[offset]; offset += 1
            op = struct.unpack_from("<I", mcod_data, offset)[0]; offset += 4
            f3_w = mcod_data[offset]; offset += 1
            f3 = struct.unpack_from("<I", mcod_data, offset)[0]; offset += 4
            f7_w = mcod_data[offset]; offset += 1
            f7 = struct.unpack_from("<I", mcod_data, offset)[0]; offset += 4
            sh_w = mcod_data[offset]; offset += 1
            sh = struct.unpack_from("<I", mcod_data, offset)[0]; offset += 4

            if op_w: entry["opcode"] = op
            if f3_w: entry["funct3"] = f3
            if f7_w: entry["funct7"] = f7
            if sh_w: entry["shamt"] = sh
            inst_map[sid] = entry

        arch_mappings[arch_name] = {"instructions": inst_map}
    return arch_mappings


# ── CLI 入口 ──
if __name__ == "__main__":
    import sys
    output = sys.argv[1] if len(sys.argv) > 1 else None
    build_font(output_path=output)
