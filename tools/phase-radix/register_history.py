#!/usr/bin/env python3
"""
register_history.py — 将历史文件中的符号纳入全局互锁登记

扫描历史文件 (whitepaper/, 老的会话记录) 提取:
  - FQM 标识与命名规则
  - 容器四态符号
  - YinYang 二进制编码
  - 八流统一体系
  - 三元/二元观测者术语
  - 封神台拟物角色
  - 家庭命名密码表 (mlll/gtt/mlct/mlnn 等)

并将这些符号登记到全局互锁系统中, 防止 AI 遗忘或漂移。

用法:
  python3 register_history.py           # 提取并报告
  python3 register_history.py --commit  # 真正写入互锁登记
"""

import os
import re
import sys
from pathlib import Path
from collections import defaultdict

TOOLS_DIR = Path(__file__).parent.resolve()
# tools/phase-radix/ → hunyuan-os/encoding/
ENCODING_DIR = TOOLS_DIR / ".." / ".." / "encoding"
ENCODING_DIR = ENCODING_DIR.resolve()
WORKSPACE_DIR = ENCODING_DIR.parent

# ── 历史符号定义 (来自 whitepaper + SPEC-002) ──

WHITEPAPER_SYMBOLS = {
    # 囡囝四态
    "QUATERNITY_NJSG": 0x10,   # 囡囝 (平等共生)
    "QUATERNITY_NN":   0x11,   # 囡囡 (妻嵌套夫)
    "QUATERNITY_JJ":   0x12,   # 囝囝 (夫嵌套妻)
    "QUATERNITY_JN":   0x13,   # 囝囡 (夫壳妻入)
    
    # 八流统一
    "FLOW_PRELOAD":    0x20,   # 预加载流
    "FLOW_CATEGORY":   0x21,   # 范畴流
    "FLOW_CLUSTER":    0x22,   # 聚类流
    "FLOW_DISTINGUISH": 0x23,  # 分类流
    "FLOW_PROPERTY":   0x24,   # 属性流
    "FLOW_INTERPRET":  0x25,   # 解释流
    "FLOW_RELATION":   0x26,   # 关系流
    "FLOW_CONCEPT":    0x27,   # 概念流
    
    # 分别器三层
    "ROUTE_PERCEPT":   0x30,   # 感知家族
    "ROUTE_HOMONYM":   0x31,   # 谐音靶点
    "ROUTE_SOVEREIGN": 0x32,   # 主权路由
    
    # YinYang 二进制
    "YINDOYANG_CONTAINER": 0x40,  # 0 = 容器 (妻)
    "YINDOYANG_BOUNDARY":  0x41,  # 1 = 界线 (夫)
    
    # 12 位二元属性
    "PROP_REACHABLE":    0x50,
    "PROP_CONTINUABLE":  0x51,
    "PROP_INCR":        0x52,
    "PROP_STOPPABLE":   0x53,
    "PROP_REVERSIBLE":  0x54,
    "PROP_RECOVERABLE": 0x55,
    "PROP_CIRCULABLE":  0x56,
    "PROP_INHERITABLE": 0x57,
    "PROP_TRUSTED":     0x58,
    "PROP_ACKABLE":     0x59,
    "PROP_TRANSMISSIBLE": 0x5A,
    "PROP_ADVANCEABLE": 0x5B,
    
    # 灵犀协议
    "XIANSHU_HWANHUN":  0x60,  # 唤魂
    "XIANSHU_ZHAOHUN":  0x61,  # 招魂
    "XIANSHU_WENLING":  0x62,  # 问灵
    "XIANSHU_DALING":   0x63,  # 答灵
    "XIANSHU_TONGXIU":  0x64,  # 同修
    "XIANSHU_CHUANGONG": 0x65, # 传功
    "XIANSHU_GUIYUAN":  0x66,  # 归元
    "XIANSHU_JIESHU":   0x67,  # 劫数
    
    # 封神台
    "FENGSHEN_TAIYI":   0x70,  # 太乙真人
    "FENGSHEN_NEZHA":   0x71,  # 哪吒
    "FENGSHEN_QIANKUN": 0x72,  # 乾坤圈
    "FENGSHEN_HUNTIAN": 0x73,  # 混天绫
    "FENGSHEN_BANG":    0x74,  # 封神榜
    "FENGSHEN_TAI":     0x75,  # 封神台
    "FENGSHEN_YUANSHI": 0x76,  # 元始天尊
    "FENGSHEN_LIJING":  0x77,  # 李靖
    "FENGSHEN_JIULONG": 0x78,  # 九龙神火罩
    "FENGSHEN_LINGZHU": 0x79,  # 灵珠子
}

NAMING_CODES = {
    "MLLL": 0xA0,   # 集装箱
    "GTT":  0xA1,   # 音韵态射
    "MLGY": 0xA2,   # 世俗归一
    "MLCT": 0xA3,   # 宠甜 (零岔途)
    "MLTT": 0xA4,   # 从体
    "MLNN": 0xA5,   # 囡囝
    "LCTQF": 0xA6,  # 零岔途全法
}


def scan_whitepaper_symbols():
    """扫描白皮书文件, 提取引用的符号"""
    whitepaper_dir = WORKSPACE_DIR / "whitepaper"
    if not whitepaper_dir.exists():
        # 尝试备用路径
        whitepaper_dir = Path("/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/whitepaper")
    if not whitepaper_dir.exists():
        return {}
    
    found = defaultdict(list)
    for f in whitepaper_dir.glob("*.md"):
        content = f.read_text(encoding="utf-8")
        for sym_name in list(WHITEPAPER_SYMBOLS.keys()) + list(NAMING_CODES.keys()):
            if sym_name in content:
                found[sym_name].append(f.name)
    
    return found


def generate_history_header():
    """生成历史符号注册头文件"""
    header_path = ENCODING_DIR / "history_symbols.h"
    
    lines = [
        "/**",
        " * history_symbols.h — 历史符号登记表",
        " *",
        " * 本文件由 register_history.py 自动生成",
        " * 将白皮书/SPEC 中的符号正式纳入全局互锁体系",
        " *",
        " * 确保 AI 不会遗忘或漂移这些核心符号",
        " */",
        "",
        "#ifndef NJSG_HISTORY_SYMBOLS_H",
        "#define NJSG_HISTORY_SYMBOLS_H",
        "",
        "#include \"global_interlock.h\"",
        "",
        "/* ── 囡囝四态 ── */",
    ]
    
    # 囡囝四态
    for name, val in sorted(WHITEPAPER_SYMBOLS.items(), key=lambda x: x[1]):
        if name.startswith("QUATERNITY_"):
            lines.append(f"#define {name:30s} 0x{val:02X}")
    
    lines.extend(["", "/* ── 八流统一 ── */"])
    for name, val in sorted(WHITEPAPER_SYMBOLS.items(), key=lambda x: x[1]):
        if name.startswith("FLOW_"):
            lines.append(f"#define {name:30s} 0x{val:02X}")
    
    lines.extend(["", "/* ── 分别器三层 ── */"])
    for name, val in sorted(WHITEPAPER_SYMBOLS.items(), key=lambda x: x[1]):
        if name.startswith("ROUTE_"):
            lines.append(f"#define {name:30s} 0x{val:02X}")
    
    lines.extend(["", "/* ── YinYang 二进制 ── */"])
    for name, val in sorted(WHITEPAPER_SYMBOLS.items(), key=lambda x: x[1]):
        if name.startswith("YINDOYANG_"):
            lines.append(f"#define {name:30s} 0x{val:02X}")
    
    lines.extend(["", "/* ── 12 位二元属性 ── */"])
    for name, val in sorted(WHITEPAPER_SYMBOLS.items(), key=lambda x: x[1]):
        if name.startswith("PROP_"):
            lines.append(f"#define {name:30s} 0x{val:02X}")
    
    lines.extend(["", "/* ── 灵犀协议 ── */"])
    for name, val in sorted(WHITEPAPER_SYMBOLS.items(), key=lambda x: x[1]):
        if name.startswith("XIANSHU_"):
            lines.append(f"#define {name:30s} 0x{val:02X}")
    
    lines.extend(["", "/* ── 封神台 ── */"])
    for name, val in sorted(WHITEPAPER_SYMBOLS.items(), key=lambda x: x[1]):
        if name.startswith("FENGSHEN_"):
            lines.append(f"#define {name:30s} 0x{val:02X}")
    
    lines.extend(["", "/* ── 家庭命名密码 ── */"])
    for name, val in sorted(NAMING_CODES.items(), key=lambda x: x[1]):
        lines.append(f"#define {name:30s} 0x{val:02X}")
    
    lines.extend([
        "",
        "#endif /* NJSG_HISTORY_SYMBOLS_H */",
        "",
    ])
    
    header_path.write_text('\n'.join(lines), encoding="utf-8")
    return header_path


def main():
    import argparse
    parser = argparse.ArgumentParser(description="将历史文件符号登记到全局互锁")
    parser.add_argument("--commit", action="store_true", help="生成头文件")
    args = parser.parse_args()
    
    print("═══ 历史文件符号扫描 ═══\n")
    
    found = scan_whitepaper_symbols()
    print(f"扫描 whitepaper/ 目录\n")
    print(f"发现历史符号引用: {len(found)} 个\n")
    
    for sym, files in sorted(found.items()):
        print(f"  {sym:25s} ← {', '.join(files)}")
    
    if args.commit:
        header_path = generate_history_header()
        print(f"\n已生成: {header_path}")
        print(f"共 {len(WHITEPAPER_SYMBOLS) + len(NAMING_CODES)} 个历史符号已登记")
    else:
        print(f"\n使用 --commit 正式生成 history_symbols.h")
    
    return 0


if __name__ == "__main__":
    sys.exit(main())
