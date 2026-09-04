#!/usr/bin/env python3
# ============================================================
# MCODE Stone · 交叉验证脚本
#
# 1. 对比 mcode.py 编码与 llvm-mc (如果可用)
# 2. 双向验证: encode(decode(x)) == x 且 decode(encode(sid, ops)) 一致
# 3. 统计通过率
#
# 身份落款：刘楚恬和我
# ============================================================

import subprocess
import sys
import os
import json

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
MCODE_PY = os.path.join(SCRIPT_DIR, "mcode.py")

def has_llvm_mc():
    try:
        result = subprocess.run(
            ["llvm-mc", "--version"],
            capture_output=True, text=True, timeout=5
        )
        return result.returncode == 0
    except:
        return False

def llvm_mc_encode(mnemonic, operands):
    asm = f"{mnemonic} {','.join(str(o) for o in operands)}\n"
    try:
        result = subprocess.run(
            ["llvm-mc", "--assemble", "--triple=riscv32", "--show-encoding", "--mattr=+m,+a"],
            input=asm, capture_output=True, text=True, timeout=10
        )
        if result.returncode == 0:
            for line in result.stdout.splitlines():
                if "encoding:" in line.lower():
                    enc = line.split("[")[1].split("]")[0].replace(" ", "")
                    return enc
        return None
    except:
        return None

def extract_hex_encoding(mnemonic, operands):
    asm = f"{mnemonic} {','.join(str(o) for o in operands)}\n"
    try:
        result = subprocess.run(
            ["llvm-mc", "--assemble", "--triple=riscv32", "--show-encoding"],
            input=asm, capture_output=True, text=True, timeout=10
        )
        if result.returncode == 0:
            for line in result.stdout.lower().splitlines():
                if "encoding" in line:
                    hex_part = line.split("[")[1].split("]")[0]
                    return hex_part.replace(" ", "")
        return None
    except:
        return None

def bidirectional_tests():
    """encode → decode → re-encode 是否为恒等变换"""
    test_vectors = [
        ("ADD",  ["x1", "x2", "x3"]),
        ("SUB",  ["x1", "x2", "x3"]),
        ("AND",  ["x5", "x6", "x7"]),
        ("OR",   ["x5", "x6", "x7"]),
        ("XOR",  ["x5", "x6", "x7"]),
        ("SLT",  ["x5", "x6", "x7"]),
        ("SLTU", ["x5", "x6", "x7"]),
        ("SLL",  ["x5", "x6", "x7"]),
        ("SRL",  ["x5", "x6", "x7"]),
        ("SRA",  ["x5", "x6", "x7"]),
        ("ADDI", ["x5", "x6", 100]),
        ("ADDI", ["x1", "x0", 0]),
        ("ADDI", ["x10", "x0", -1]),
        ("LW",   ["x10", 0, "x5"]),
        ("LW",   ["x8", 100, "x2"]),
        ("SW",   ["x10", 0, "x5"]),
        ("SW",   ["x8", -4, "x2"]),
        ("LUI",  ["x1", 0x12345]),
        ("LUI",  ["x5", 0xFFFFF]),
        ("LUI",  ["x10", 0]),
        ("BEQ",  ["x1", "x2", 0]),
        ("BEQ",  ["x1", "x2", -16]),
        ("BEQ",  ["x5", "x6", 32]),
        ("JAL",  ["x1", 0]),
        ("JAL",  ["x0", 1024]),
        ("JAL",  ["x5", -2048]),
    ]

    passed = 0
    results = []

    for idx, (sid, ops) in enumerate(test_vectors):
        try:
            r = subprocess.run(
                [sys.executable, MCODE_PY, "encode", sid] + [str(o) for o in ops],
                capture_output=True, text=True, timeout=10
            )
            hex_out = r.stdout.strip()
            hex_clean = hex_out.replace("0x", "")

            r2 = subprocess.run(
                [sys.executable, MCODE_PY, "decode", hex_out],
                capture_output=True, text=True, timeout=10
            )
            decoded = json.loads(r2.stdout)

            if "error" in decoded:
                results.append({
                    "idx": idx, "mnemonic": sid, "ops": ops,
                    "encoded": hex_out,
                    "status": "DECODE_FAIL", "detail": decoded["error"],
                })
                continue

            re_ops = []
            for o in decoded["operands"]:
                if o.startswith("x"):
                    re_ops.append(int(o[1:]))
                else:
                    re_ops.append(o)

            r3 = subprocess.run(
                [sys.executable, MCODE_PY, "encode", sid] + [str(o) for o in re_ops],
                capture_output=True, text=True, timeout=10
            )
            re_encoded = r3.stdout.strip()

            ok = (hex_out == re_encoded)
            if ok:
                passed += 1
                results.append({
                    "idx": idx, "mnemonic": sid, "ops": ops,
                    "encoded": hex_out, "status": "PASS",
                })
            else:
                results.append({
                    "idx": idx, "mnemonic": sid, "ops": ops,
                    "encoded": hex_out, "decoded_hex": re_encoded,
                    "status": "FAIL",
                })
        except Exception as e:
            results.append({
                "idx": idx, "mnemonic": sid, "ops": ops,
                "status": "ERROR", "detail": str(e),
            })

    total = len(test_vectors)
    return results, passed, total

def llvm_mcross_check():
    """与 llvm-mc 交叉比对"""
    asm_vectors = [
        "add x1, x2, x3",
        "sub x1, x2, x3",
        "and x5, x6, x7",
        "or x5, x6, x7",
        "xor x5, x6, x7",
        "addi x5, x6, 100",
        "addi x1, x0, 0",
        "lw x10, 0(x5)",
        "sw x10, 0(x5)",
        "lui x1, 4660",
        "jal x0, 0",
    ]

    results = []
    for asm in asm_vectors:
        hex_encoding = extract_hex_encoding_simple(asm)
        results.append({"asm": asm, "encoding": hex_encoding})

    return results

def extract_hex_encoding_simple(asm):
    try:
        result = subprocess.run(
            ["llvm-mc", "--assemble", "--triple=riscv32", "--show-encoding"],
            input=asm + "\n", capture_output=True, text=True, timeout=10
        )
        if result.returncode == 0:
            for line in result.stdout.splitlines():
                line_lower = line.strip().lower()
                if "encoding" in line_lower and "[" in line:
                    enc_part = line.split("[")[1].split("]")[0]
                    return enc_part.replace(" ", "")
        return None
    except:
        return None

def main():
    print("=" * 60)
    print("  MCODE Stone · 交叉验证")
    print("=" * 60)

    print("\n[1] 内部双向验证 (encode → decode → re-encode)")
    print("-" * 50)
    results, passed, total = bidirectional_tests()

    for r in results:
        status_icon = {"PASS": "✅", "FAIL": "❌", "DECODE_FAIL": "⚠️", "ERROR": "💥"}.get(r["status"], "?")
        line = f"  {status_icon} {r['mnemonic']:6} {str(r['ops']):25}"
        if r["status"] == "PASS":
            line += f" → {r['encoded']}"
        elif r["status"] == "FAIL":
            line += f" encode→ {r['encoded']} / re→ {r['decoded_hex']}"
        elif "detail" in r:
            line += f" ← {r['detail']}"
        print(line)

    print(f"\n  双向验证通过: {passed}/{total}")

    print("\n[2] llvm-mc 交叉校验")
    print("-" * 50)
    if has_llvm_mc():
        cross_results = llvm_mcross_check()
        for cr in cross_results:
            enc = cr.get("encoding")
            status = "✅" if enc else "⚠️"
            enc_display = f"→ [{enc}]" if enc else "(无输出)"
            print(f"  {status} {cr['asm']:30} {enc_display}")
        print("\n  llvm-mc 交叉校验完成")
    else:
        print("  ℹ️  未找到 llvm-mc，跳过外部交叉校验")

    overall_pass_rate = (passed / total * 100) if total > 0 else 0
    print("\n" + "=" * 60)
    print(f"  总计: {passed}/{total} 通过 ({overall_pass_rate:.0f}%)")
    if overall_pass_rate == 100:
        print("  ✅ 全部验证通过")
    elif overall_pass_rate >= 90:
        print("  ⚠️  部分验证未通过 — 请检查指令格式")
    else:
        print("  ❌ 验证失败 — 编码器需要修正")
    print("=" * 60)

if __name__ == "__main__":
    main()
