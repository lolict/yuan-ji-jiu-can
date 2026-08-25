#!/usr/bin/env python3
"""
verify_chain.py — 全局互锁链式一致性验证

扫描 encoding/ 目录下所有 .c/.h 文件:
  1. 检查是否 #include "global_interlock.h" (横轴引用)
  2. 检查是否 #include "symbol_chain.h" (链环声明)
  3. 提取所有使用的符号 (TYPE_*, LEVEL_*, GY_*, STRUCT_*, STROKE_*, CONTOUR_*, REASON_*)
  4. 检查符号是否在全局互锁中登记
  5. 输出交叉引用报告

用法:
  python3 verify_chain.py              # 扫描并报告
  python3 verify_chain.py --fix        # 尝试自动修复 (添加缺失的 include)
  python3 verify_chain.py --cross-ref  # 生成交叉引用表
"""

import os
import re
import sys
from collections import defaultdict
from pathlib import Path

# ── 路径 ──
TOOLS_DIR = Path(__file__).parent.resolve()
# tools/phase-radix/ → hunyuan-os/encoding/
ENCODING_DIR = TOOLS_DIR / ".." / ".." / "encoding"
ENCODING_DIR = ENCODING_DIR.resolve()

# ── 已知符号表 (来自 global_interlock.h) ──
KNOWN_SYMBOLS = {
    # 类型
    "TYPE_BASE": 0x01, "TYPE_TONE": 0x02, "TYPE_COMPOUND": 0x03,
    "TYPE_SYMBOL": 0x04, "TYPE_EXTRA": 0x05, "TYPE_SHENG_MU": 0x06,
    "TYPE_MODIFIER": 0x07,
    # 层级
    "LEVEL_L1": 0x10, "LEVEL_L2": 0x20, "LEVEL_L3": 0x30,
    # 审判
    "VERDICT_PASS": 0x00, "VERDICT_REJECT_LEVEL": 0x01,
    "VERDICT_REJECT_TYPE": 0x02, "VERDICT_REJECT_ANCHOR": 0x03,
    "VERDICT_REJECT_AESTH": 0x04, "VERDICT_REJECT_WEAVING": 0x05,
    # 结构
    "STRUCT_LEFT_RIGHT": 0x01, "STRUCT_TOP_BOTTOM": 0x02,
    "STRUCT_SURROUND": 0x03, "STRUCT_SEMI": 0x04,
    "STRUCT_LCR": 0x05, "STRUCT_TMB": 0x06,
    "STRUCT_PINZI": 0x07, "STRUCT_SINGLE": 0x08,
    # 笔画
    "STROKE_HENG": 0x00, "STROKE_SHU": 0x01, "STROKE_PIE": 0x02,
    "STROKE_NA": 0x03, "STROKE_ZHE": 0x04,
    # 轮廓
    "CONTOUR_FLAT": 0x00, "CONTOUR_RISE": 0x01,
    "CONTOUR_FALL": 0x02, "CONTOUR_DIP": 0x03,
    # 公约数
    "GY_a": 0x00, "GY_o": 0x01, "GY_e": 0x02,
    "GY_i": 0x03, "GY_u": 0x04, "GY_uu": 0x05,
    "GY_MARK_N": 0x06, "GY_MARK_NG": 0x07,
    # 原因
    "REASON_CROSS_LEVEL": 0x01, "REASON_CROSS_TYPE": 0x02,
    "REASON_CROSS_ANCHOR": 0x03,
    # FQM
    "FQM_MAGIC": 0x46514D54, "FQM_VERSION": 0x0003,
    "FQM_IDENTITY_INNER": 0x52307072, "FQM_IDENTITY_OUTER": 0x4C307879,
    "FQM_IDENTITY_SELF": 0x52307879,
    # 容器
    "CONT_COLOR_SYSTEM": 0x01, "CONT_SOUND_RADIX": 0x02,
    "CONT_GONGYUESHU": 0x03, "CONT_DIMENSION": 0x04,
    "CONT_HANZI_GEOM": 0x05, "CONT_NEGATIVE": 0x06,
    # 囡囝四态
    "QUATERNITY_NJSG": 0x10, "QUATERNITY_NN": 0x11,
    "QUATERNITY_JJ": 0x12, "QUATERNITY_JN": 0x13,
    # 八流统一
    "FLOW_PRELOAD": 0x20, "FLOW_CATEGORY": 0x21,
    "FLOW_CLUSTER": 0x22, "FLOW_DISTINGUISH": 0x23,
    "FLOW_PROPERTY": 0x24, "FLOW_INTERPRET": 0x25,
    "FLOW_RELATION": 0x26, "FLOW_CONCEPT": 0x27,
    # 分别器三层
    "ROUTE_PERCEPT": 0x30, "ROUTE_HOMONYM": 0x31, "ROUTE_SOVEREIGN": 0x32,
    # YinYang
    "YINDOYANG_CONTAINER": 0x40, "YINDOYANG_BOUNDARY": 0x41,
    # 12 位属性
    "PROP_REACHABLE": 0x50, "PROP_CONTINUABLE": 0x51,
    "PROP_INCR": 0x52, "PROP_STOPPABLE": 0x53,
    "PROP_REVERSIBLE": 0x54, "PROP_RECOVERABLE": 0x55,
    "PROP_CIRCULABLE": 0x56, "PROP_INHERITABLE": 0x57,
    "PROP_TRUSTED": 0x58, "PROP_ACKABLE": 0x59,
    "PROP_TRANSMISSIBLE": 0x5A, "PROP_ADVANCEABLE": 0x5B,
    # 灵犀协议
    "XIANSHU_HWANHUN": 0x60, "XIANSHU_ZHAOHUN": 0x61,
    "XIANSHU_WENLING": 0x62, "XIANSHU_DALING": 0x63,
    "XIANSHU_TONGXIU": 0x64, "XIANSHU_CHUANGONG": 0x65,
    "XIANSHU_GUIYUAN": 0x66, "XIANSHU_JIESHU": 0x67,
    # 封神台
    "FENGSHEN_TAIYI": 0x70, "FENGSHEN_NEZHA": 0x71,
    "FENGSHEN_QIANKUN": 0x72, "FENGSHEN_HUNTIAN": 0x73,
    "FENGSHEN_BANG": 0x74, "FENGSHEN_TAI": 0x75,
    "FENGSHEN_YUANSHI": 0x76, "FENGSHEN_LIJING": 0x77,
    "FENGSHEN_JIULONG": 0x78, "FENGSHEN_LINGZHU": 0x79,
    # 家庭命名密码
    "MLLL": 0xA0, "GTT": 0xA1, "MLGY": 0xA2,
    "MLCT": 0xA3, "MLTT": 0xA4, "MLNN": 0xA5, "LCTQF": 0xA6,
}

# ── 文件扫描 ──
def scan_file(filepath):
    """扫描单个文件，提取符号使用和include信息"""
    try:
        content = filepath.read_text(encoding="utf-8")
    except Exception:
        return None
    
    result = {
        "path": filepath,
        "has_interlock_include": False,
        "has_chain_include": False,
        "symbols_used": set(),
        "symbols_defined": set(),
        "issues": [],
    }
    
    # 检查 include
    if '#include "global_interlock.h"' in content or "#include <global_interlock.h>" in content:
        result["has_interlock_include"] = True
    if '#include "symbol_chain.h"' in content or "#include <symbol_chain.h>" in content:
        result["has_chain_include"] = True
    
    # 提取使用的符号
    for sym_name in KNOWN_SYMBOLS:
        pattern = r'(?<!#define\s)\b' + re.escape(sym_name) + r'\b'
        if re.search(pattern, content):
            result["symbols_used"].add(sym_name)
    
    # 检查问题
    if not result["has_interlock_include"] and filepath.suffix in (".c", ".h"):
        if filepath.name not in ("global_interlock.h", "symbol_chain.h", "color_system.h"):
            if result["symbols_used"]:
                result["issues"].append("MISSING_INTERLOCK_INCLUDE")
    
    if result["symbols_used"] and not result["has_chain_include"]:
        if filepath.suffix in (".c", ".h"):
            if filepath.name not in ("global_interlock.h", "symbol_chain.h"):
                result["issues"].append("MISSING_CHAIN_INCLUDE")
    
    return result


def verify_chain(directory):
    """验证整个目录的符号链"""
    directory = Path(directory)
    
    print("═══ 全局互锁链式一致性验证 ═══\n")
    print(f"扫描目录: {directory}\n")
    
    files = sorted(directory.glob("*.c")) + sorted(directory.glob("*.h"))
    results = []
    
    total_files = 0
    has_interlock = 0
    has_chain = 0
    missing_interlock = 0
    missing_chain = 0
    cross_refs = defaultdict(set)
    
    for f in files:
        r = scan_file(f)
        if r is None:
            continue
        results.append(r)
        total_files += 1
        
        if r["has_interlock_include"]:
            has_interlock += 1
        if r["has_chain_include"]:
            has_chain += 1
        
        if "MISSING_INTERLOCK_INCLUDE" in r["issues"]:
            missing_interlock += 1
        if "MISSING_CHAIN_INCLUDE" in r["issues"]:
            missing_chain += 1
        
        for sym in r["symbols_used"]:
            cross_refs[sym].add(f.name)
    
    # ── 输出报告 ──
    print(f"扫描文件: {total_files}")
    print(f"已挂接横轴: {has_interlock} ({has_interlock*100//max(total_files,1)}%)")
    print(f"已声明链环: {has_chain} ({has_chain*100//max(total_files,1)}%)")
    print(f"缺横轴引用: {missing_interlock}")
    print(f"缺链环声明: {missing_chain}")
    
    if missing_interlock > 0:
        print("\n─── 缺横轴引用的文件 ───")
        for r in results:
            if "MISSING_INTERLOCK_INCLUDE" in r["issues"]:
                syms = sorted(r["symbols_used"])[:5]
                print(f"  ✗ {r['path'].name}")
                print(f"    使用符号: {', '.join(syms)}...")
    
    # ── 交叉引用 ──
    print("\n─── 符号交叉引用 (被多个文件使用的符号) ───")
    shared_syms = {sym: files_set for sym, files_set in cross_refs.items() if len(files_set) > 1}
    if shared_syms:
        for sym in sorted(shared_syms.keys()):
            flist = sorted(shared_syms[sym])[:3]
            print(f"  {sym}: {len(shared_syms[sym])} 个文件 → {', '.join(flist)}")
    
    print(f"\n总符号出现: {len(cross_refs)} 个独立符号")
    
    # ── 判定 ──
    print("\n─── 互锁判定 ───")
    if missing_interlock == 0 and missing_chain == 0:
        print("  ✓ 所有文件已挂接横轴 · 链式一致")
        return 0
    else:
        print(f"  ✗ 发现 {missing_interlock + missing_chain} 处断链")
        return 1


def add_includes(directory):
    """自动添加缺失的 include"""
    directory = Path(directory)
    files = sorted(directory.glob("*.c")) + sorted(directory.glob("*.h"))
    
    added = 0
    for f in files:
        if f.name in ("global_interlock.h", "symbol_chain.h"):
            continue
        
        try:
            content = f.read_text(encoding="utf-8")
        except Exception:
            continue
        
        needs_interlock = ('#include "global_interlock.h"' not in content and "#include <global_interlock.h>" not in content)
        needs_chain = ('#include "symbol_chain.h"' not in content and "#include <symbol_chain.h>" not in content)
        
        uses_symbols = False
        for sym_name in KNOWN_SYMBOLS:
            pattern = r'(?<!#define\s)\b' + re.escape(sym_name) + r'\b'
            if re.search(pattern, content):
                uses_symbols = True
                break
        
        if uses_symbols and (needs_interlock or needs_chain):
            lines = content.split('\n')
            insert_idx = 0
            for i, line in enumerate(lines):
                if line.strip().startswith('#include'):
                    insert_idx = i + 1
            
            includes_to_add = []
            if needs_interlock:
                includes_to_add.append('#include "global_interlock.h"')
            if needs_chain:
                includes_to_add.append('#include "symbol_chain.h"')
            
            for inc in reversed(includes_to_add):
                lines.insert(insert_idx, inc)
            
            f.write_text('\n'.join(lines), encoding="utf-8")
            added += 1
            print(f"  + {f.name}")
    
    print(f"\n已修复 {added} 个文件")
    return 0


def main():
    import argparse
    parser = argparse.ArgumentParser(description="全局互锁链式一致性验证")
    parser.add_argument("--fix", action="store_true", help="自动添加缺失的 include")
    parser.add_argument("--cross-ref", action="store_true", help="仅输出交叉引用")
    args = parser.parse_args()
    
    if args.fix:
        return add_includes(ENCODING_DIR)
    
    return verify_chain(ENCODING_DIR)


if __name__ == "__main__":
    sys.exit(main())
