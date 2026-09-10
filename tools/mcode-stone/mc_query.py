#!/usr/bin/env python3
# ============================================================
# mc_query.py · 跨 ISA 机器码查询工具
#
# 用法:
#   python3 mc_query.py list              — 列出所有语义 ID
#   python3 mc_query.py align <语义ID>     — 查询单条指令的全 ISA 编码
#   python3 mc_query.py encode <ISA> <语义ID> [操作数 ...]
#   python3 mc_query.py decode <ISA> <机器码>
#   python3 mc_query.py table              — 输出完整对齐表
#
# 身份落款：刘楚恬和我
# ============================================================

import sys, json
from cross_isa import CROSS_ALIGN, encode, decode, query_all_isa, get_isa_db

ISAS = ['riscv32', 'arm64', 'mips32', 'x86_64']

def cmd_list():
    print('语义 ID 列表:')
    for sem in sorted(CROSS_ALIGN.keys()):
        count = len(CROSS_ALIGN[sem])
        print(f'  {sem:<12} ({count} ISA)')

def cmd_align(sem_id):
    info = query_all_isa(sem_id)
    print(f'{sem_id} 跨 ISA 对齐:')
    if not info['isas']:
        print('  未找到')
        return
    for isa, d in info['isas'].items():
        inst = d['inst_id']
        ops_count = '?'
        tmpl = get_isa_db(isa)
        for inst_entry in tmpl['instructions']:
            if inst_entry['id'] == inst:
                ops_count = len(inst_entry.get('operands', []))
                break
        print(f'  isa={isa:<8} inst_id={inst:<12} ops={ops_count}')

def cmd_encode(isa, sem_id, ops_raw):
    ops = []
    for o in ops_raw:
        try: ops.append(int(o, 0))
        except ValueError: ops.append(o)
    try:
        mc = encode(isa, sem_id, ops)
        print(f'  0x{mc:08X}')
        # 二进制展示
        # print(f'  ({mc:032b})')
    except Exception as e:
        print(f'  ERR: {e}')

def cmd_decode(isa, hex_str):
    word = int(hex_str, 16)
    r = decode(isa, word)
    print(json.dumps(r, indent=2, ensure_ascii=False))

def cmd_table():
    from cross_isa import RV32I_DB, ARM64_DB, X64_DB, MIPS_DB
    TEST_OPS_DEFAULT = {
        'ADD': {'riscv32':['x1','x2','x3'],'arm64':['x1','x2','x3'],'mips32':['1','2','3'],'x86_64':['rcx','rdx']},
        'SUB': {'riscv32':['x1','x2','x3'],'arm64':['x1','x2','x3'],'mips32':['1','2','3']},
        'MOV': {'riscv32':['x1','x0',0],'arm64':['x1','x2'],'mips32':['1','0','0'],'x86_64':['rcx','rdx']},
    }
    hdr = f"{'语义ID':<10}"
    print(hdr, end='')
    for isa in ISAS: print(f'{isa:<16}', end='')
    print()
    print('-'*74)
    for sem in sorted(CROSS_ALIGN.keys()):
        line = f'{sem:<10}'
        for isa in ISAS:
            inst_id = CROSS_ALIGN[sem].get(isa)
            if not inst_id:
                line += f"{'—':<16}"
                continue
            ops_map = {
                'riscv32':['x1','x2','x3'],
                'arm64':['x1','x2','x3'],
                'mips32':['1','2','3'],
                'x86_64':['rcx','rdx']
            }
            try:
                mc = encode(isa, sem, ops_map.get(isa,['x1','x2','x3']))
                line += f'0x{mc:08X}    '
            except:
                line += f"{'ERR':<16}"
        print(line)

def main():
    args = sys.argv[1:]
    if not args:
        cmd_list()
        return
    cmd = args[0]
    if cmd == 'list':
        cmd_list()
    elif cmd == 'align' and len(args) >= 2:
        cmd_align(args[1])
    elif cmd == 'encode' and len(args) >= 3:
        cmd_encode(args[1], args[2], args[3:])
    elif cmd == 'decode' and len(args) >= 3:
        cmd_decode(args[1], args[2])
    elif cmd == 'table':
        cmd_table()
    else:
        print('用法: mc_query.py <list|align|encode|decode|table> [args]')
        sys.exit(1)

if __name__ == '__main__':
    main()
