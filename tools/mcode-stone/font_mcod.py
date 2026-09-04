#!/usr/bin/env python3
# ============================================================
# MCODE Stone · 字体封装器
#
# 将 MCOD 机器码模板表嵌入 TTF 文件的 OpenType 自定义表中。
# 打包格式: 4-byte magic "MCOD" + 2-byte version + 指令表
#
# 身份落款：刘楚恬和我
# ============================================================

import sys
import os
import struct
import json
import yaml

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
OPS_FILE = os.path.join(SCRIPT_DIR, "riscv32i_ops.yaml")

MCOD_MAGIC = b"MCOD"
MCOD_VERSION = 1

def build_mcod_table():
    with open(OPS_FILE, "r", encoding="utf-8") as f:
        ops_db = yaml.safe_load(f)

    buf = bytearray()
    buf += struct.pack(">4sH", MCOD_MAGIC, MCOD_VERSION)

    inst_list = ops_db["instructions"]
    buf += struct.pack(">H", len(inst_list))

    for inst in inst_list:
        iid = inst["id"].encode("ascii")
        mnemonic = inst["mnemonic"].encode("ascii")
        semantic = inst.get("semantic", "").encode("utf-8")
        encoding = inst.get("encoding", "").encode("ascii")

        buf += struct.pack(">B", len(iid)) + iid
        buf += struct.pack(">B", len(mnemonic)) + mnemonic
        buf += struct.pack(">B", inst["type"].encode("ascii")[0])
        buf += struct.pack(">B", int(inst["opcode"], 2))
        buf += struct.pack(">B", int(inst.get("funct3", "000"), 2))
        buf += struct.pack(">B", int(inst.get("funct7", "0000000"), 2))
        buf += struct.pack(">H", len(semantic)) + semantic
        buf += struct.pack(">H", len(encoding)) + encoding

    return bytes(buf)

def inject_mcod_into_ttf(src_ttf, dst_ttf, mcod_data):
    with open(src_ttf, "rb") as f:
        ttf = bytearray(f.read())

    num_tables = struct.unpack(">H", ttf[4:6])[0]
    old_header_size = 12 + num_tables * 16
    new_header_size = 12 + (num_tables + 1) * 16
    table_data_block = ttf[old_header_size:]

    mcod_padded = mcod_data + b"\x00" * ((4 - len(mcod_data) % 4) % 4)
    mcod_checksum = 0
    for i in range(0, len(mcod_padded), 4):
        mcod_checksum = (mcod_checksum + struct.unpack(">I", mcod_padded[i:i+4])[0]) & 0xFFFFFFFF

    existing_records = []
    for i in range(num_tables):
        off = 12 + i * 16
        existing_records.append(bytearray(ttf[off:off+16]))

    new_num = num_tables + 1
    searchRange = 1
    while searchRange * 2 <= new_num:
        searchRange *= 2
    searchRange *= 16
    rangeShift = new_num * 16 - searchRange

    new_ttf = bytearray()
    new_ttf += struct.pack(">I", 0x00010000)
    new_ttf += struct.pack(">HHH", new_num, searchRange, rangeShift)
    new_ttf += struct.pack(">H", rangeShift)

    delta = new_header_size - old_header_size
    for rec in existing_records:
        old_off = struct.unpack(">I", rec[8:12])[0]
        struct.pack_into(">I", rec, 8, old_off + delta)

    mcod_data_offset = new_header_size + len(table_data_block)
    mcod_rec = bytearray(16)
    mcod_rec[0:4] = MCOD_MAGIC
    struct.pack_into(">I", mcod_rec, 4, mcod_checksum)
    struct.pack_into(">I", mcod_rec, 8, mcod_data_offset)
    struct.pack_into(">I", mcod_rec, 12, len(mcod_data))

    existing_records.append(mcod_rec)
    existing_records.sort(key=lambda r: bytes(r[0:4]))

    for rec in existing_records:
        new_ttf += rec

    new_ttf += table_data_block
    new_ttf += mcod_padded

    with open(dst_ttf, "wb") as f:
        f.write(new_ttf)

    return len(mcod_data), len(new_ttf) - len(ttf)

def extract_mcod_from_ttf(ttf_path):
    with open(ttf_path, "rb") as f:
        data = f.read()

    num_tables = struct.unpack(">H", data[4:6])[0]
    for i in range(num_tables):
        off = 12 + i * 16
        tag = data[off:off+4]
        if tag == MCOD_MAGIC:
            table_offset = struct.unpack(">I", data[off+8:off+12])[0]
            table_len = struct.unpack(">I", data[off+12:off+16])[0]
            return data[table_offset:table_offset+table_len]
    return None

def parse_mcod_table(mcod_data):
    magic = mcod_data[0:4]
    if magic != MCOD_MAGIC:
        raise ValueError(f"Bad magic: {magic}")
    version = struct.unpack(">H", mcod_data[4:6])[0]
    num_inst = struct.unpack(">H", mcod_data[6:8])[0]

    offset = 8
    instructions = []
    for _ in range(num_inst):
        id_len = mcod_data[offset]; offset += 1
        iid = mcod_data[offset:offset+id_len].decode("ascii"); offset += id_len

        mnem_len = mcod_data[offset]; offset += 1
        mnemonic = mcod_data[offset:offset+mnem_len].decode("ascii"); offset += mnem_len

        inst_type = chr(mcod_data[offset]); offset += 1
        opcode = mcod_data[offset]; offset += 1
        funct3 = mcod_data[offset]; offset += 1
        funct7 = mcod_data[offset]; offset += 1

        sem_len = struct.unpack(">H", mcod_data[offset:offset+2])[0]; offset += 2
        semantic = mcod_data[offset:offset+sem_len].decode("utf-8"); offset += sem_len

        enc_len = struct.unpack(">H", mcod_data[offset:offset+2])[0]; offset += 2
        encoding = mcod_data[offset:offset+enc_len].decode("ascii"); offset += enc_len

        instructions.append({
            "id": iid, "mnemonic": mnemonic, "type": inst_type,
            "opcode": format(opcode, "07b"),
            "funct3": format(funct3, "03b"),
            "funct7": format(funct7, "07b"),
            "semantic": semantic, "encoding": encoding,
        })

    return {"version": version, "instructions": instructions}

def create_minimal_ttf(dst_path):
    ttf = bytearray()

    num_tables = 9
    searchRange = 128
    entrySelector = 3
    rangeShift = num_tables * 16 - searchRange

    ttf += struct.pack(">I", 0x00010000)
    ttf += struct.pack(">HHH", num_tables, searchRange, rangeShift)
    ttf += struct.pack(">H", rangeShift)

    tables = [
        b"cmap", b"glyf", b"head", b"hhea",
        b"hmtx", b"loca", b"maxp", b"name", b"post",
    ]
    table_data = {
        b"cmap": b"\x00\x00\x00\x00",
        b"glyf": b"\x00",
        b"head": bytearray(54),
        b"hhea": bytearray(36),
        b"hmtx": b"\x00\x00\x00\x00",
        b"loca": b"\x00\x00",
        b"maxp": bytearray(32),
        b"name": b"\x00\x00\x06\x4d\x43\x4f\x44",
        b"post": b"\x00\x00\x00\x00" + b"\x00\x03\x00\x00",
    }
    table_data[b"head"][12:16] = struct.pack(">I", 0x5F0F3CF5)
    table_data[b"head"][50:52] = struct.pack(">H", 1)
    table_data[b"maxp"][0:4] = struct.pack(">I", 0x00010000)
    table_data[b"maxp"][4:6] = struct.pack(">H", 1)

    header_len = 12 + num_tables * 16
    current_offset = header_len
    table_records = []
    for tag in tables:
        td = table_data[tag]
        padded = td + b"\x00" * ((4 - len(td) % 4) % 4)
        checksum = 0
        for i in range(0, len(padded), 4):
            checksum = (checksum + struct.unpack(">I", padded[i:i+4])[0]) & 0xFFFFFFFF
        table_records.append((tag, checksum, current_offset, len(td)))
        current_offset += len(padded)

    table_records.sort(key=lambda x: x[0])
    for tag, checksum, offset, length in table_records:
        ttf += tag
        ttf += struct.pack(">III", checksum, offset, length)

    for tag in tables:
        td = table_data[tag]
        padded = td + b"\x00" * ((4 - len(td) % 4) % 4)
        ttf += padded

    with open(dst_path, "wb") as f:
        f.write(ttf)

def cli():
    args = sys.argv[1:]
    if not args:
        print("用法: python3 font_mcod.py <build|extract|pack|info>")
        sys.exit(0)

    cmd = args[0]

    if cmd == "build":
        mcod = build_mcod_table()
        print(f"MCOD 表大小: {len(mcod)} bytes")
        print(f"  魔数: {MCOD_MAGIC}")
        print(f"  版本: {MCOD_VERSION}")
        out_path = args[1] if len(args) > 1 else os.path.join(SCRIPT_DIR, "template.mcod")
        with open(out_path, "wb") as f:
            f.write(mcod)
        print(f"输出: {out_path}")

    elif cmd == "pack":
        src_ttf = args[1]
        dst_ttf = args[2] if len(args) > 1 else src_ttf.replace(".ttf", "_mcode.ttf")
        if not os.path.exists(src_ttf):
            print(f"字体不存在，创建最小 TTF: {src_ttf}")
            create_minimal_ttf(src_ttf)
        mcod = build_mcod_table()
        mcode_size, delta = inject_mcod_into_ttf(src_ttf, dst_ttf, mcod)
        print(f"字体重打包完成:")
        print(f"  MCOD 表大小: {mcode_size} bytes")
        print(f"  文件增大: {delta} bytes")
        print(f"  输出: {dst_ttf}")

    elif cmd == "extract":
        ttf_path = args[1]
        mcod = extract_mcod_from_ttf(ttf_path)
        if mcod:
            info = parse_mcod_table(mcod)
            out_json = args[2] if len(args) > 1 else None
            if out_json:
                with open(out_json, "w", encoding="utf-8") as f:
                    json.dump(info, f, indent=2, ensure_ascii=False)
                print(f"已提取 {len(info['instructions'])} 条指令到 {out_json}")
            else:
                print(json.dumps(info, indent=2, ensure_ascii=False))
        else:
            print("未找到 MCOD 表")
            sys.exit(1)

    elif cmd == "info":
        mcod = build_mcod_table()
        info = parse_mcod_table(mcod)
        print(f"MCOD 字体表信息")
        print(f"  总字节数: {len(mcod)}")
        print(f"  指令数:   {info['version']}")
        print(f"  指令列表:")
        for inst in info["instructions"]:
            ops = inst.get("semantic", "?")
            print(f"    {inst['id']:6} {inst['mnemonic']:6} [{inst['type']}] {ops}")

    else:
        print(f"未知命令: {cmd}")
        sys.exit(1)

if __name__ == "__main__":
    cli()
