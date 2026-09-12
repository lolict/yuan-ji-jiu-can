#!/usr/bin/env python3
"""
isa-ref · 跨架构语义ID交叉对照器

读取 sem_map.yaml，输出对照表：
  python3 isa_xref.py show             # 默认：输出对照表
  python3 isa_xref.py lookup SEM_ADD_RRR   # 单条语义ID
  python3 isa_xref.py list             # 所有语义ID清单

满全法叠加态 — 同一个语义ID在5个容器里的叠加态共存
"""

import sys, os, json

try:
    import yaml
except ImportError:
    print("PyYAML未安装，请运行: pip install pyyaml")
    print("或使用: python3 -c \"import yaml\" 检查")
    sys.exit(1)

REGISTRY = os.path.join(os.path.dirname(__file__), '..', 'registry', 'sem_map.yaml')

def load():
    with open(REGISTRY) as f:
        return yaml.safe_load(f)

def show_all(db):
    """满全法5项输出 — 跨ISA对照表"""
    isas = ['riscv32', 'arm64', 'x86_64', 'mips32', 'mtk']
    print("满全法 · isa-ref · 跨架构语义ID对照")
    print("=" * 70)
    print(f"{'语义ID':<18} {'riscv32':>10} {'arm64':>10} {'x86_64':>10} {'mips32':>10} {'mtk':>10}")
    print("-" * 70)
    total = 0
    for semid, info in sorted(db.items()):
        vals = [info.get(isa, {}).get('hex', '—')[:8] for isa in isas]
        print(f"{semid:<18} {vals[0]:>10} {vals[1]:>10} {vals[2]:>10} {vals[3]:>10} {vals[4]:>10}")
        total += 1
    print("-" * 70)
    print(f"语义ID总数: {total} × 架构数: 5 = {total*5} 个编码单元")
    print()

def lookup(db, semid):
    if semid not in db:
        print(f"未找到语义ID: {semid}")
        print(f"可用: {', '.join(sorted(db.keys()))}")
        return
    info = db[semid]
    print(f"满全法 · 语义ID · {semid}")
    print(f"  描述: {info['desc']}")
    print(f"  叠加态（5个容器）:")
    for isa in ['riscv32', 'arm64', 'x86_64', 'mips32', 'mtk']:
        if isa in info:
            print(f"    {isa:<8}: {info[isa]['hex']:>12}  ← {info[isa]['asm']}")
    print()

def list_ids(db):
    for semid, info in sorted(db.items()):
        print(f"{semid:<18} # {info['desc']}")

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("用法:")
        print("  isa_xref.py show              # 对照表")
        print("  isa_xref.py lookup SEM_ADD_RRR")
        print("  isa_xref.py list              # 语义ID清单")
        sys.exit(1)

    db = load()
    cmd = sys.argv[1].lower()
    if cmd == 'show':
        show_all(db)
    elif cmd == 'lookup':
        if len(sys.argv) < 3:
            print("用法: isa_xref.py lookup <SEM_ID>")
            sys.exit(1)
        lookup(db, sys.argv[2].upper())
    elif cmd == 'list':
        list_ids(db)
    else:
        print(f"未知命令: {cmd}")
