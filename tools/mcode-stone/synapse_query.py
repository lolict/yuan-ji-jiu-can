#!/usr/bin/env python3
# 满全法 · 神经突触查询终端 (Synapse Query Terminal)
# 用法:
#   python3 synapse_query.py addr 22:35:10:0:77    # 按地址查
#   python3 synapse_query.py find container_radix    # 模糊查找
#   python3 synapse_query.py out hunyuan-rs/src/encoding/container_radix.rs  # 出射
#   python3 synapse_query.py in container_radix.rs   # 入射
#   python3 synapse_query.py path <src> <dst> [depth] # 思维链路径
#   python3 synapse_query.py cluster                  # 聚类统计
#   python3 synapse_query.py stats                    # 总体统计
#   python3 synapse_query.py proj container_radix.rs  # 三维投影
#   python3 synapse_query.py color ci=0               # 按color分量查
#   python3 synapse_query.py yunmu yi=10              # 按韵母分量查

import json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
SYNAPSES = os.path.join(HERE, 'synapses.json')

def load():
    with open(SYNAPSES, 'r', encoding='utf-8') as f:
        return json.load(f)

def cmd_addr(data, addr_str):
    """按地址字符串查找"""
    for path, node in data['nodes'].items():
        if node['address'] == addr_str:
            _print_node(node, full=True)
            return
    print(f"地址 {addr_str}: 无匹配节点")

def cmd_find(data, name):
    """文件名模糊匹配"""
    matches = [(p, n) for p, n in data['nodes'].items()
               if name.lower() in n['filename'].lower()
               or name.lower() in n['path'].lower()]
    print(f"查询 \"{name}\" → {len(matches)} 个结果:")
    for path, node in matches[:30]:
        _print_node(node, full=False)
    if len(matches) > 30:
        print(f"  ... 省略 {len(matches)-30}")

def cmd_out(data, path):
    """出射: 谁被我引用"""
    node = data['nodes'].get(path)
    if not node:
        # 模糊匹配
        matches = [p for p in data['nodes'] if path in p]
        if matches:
            path = matches[0]
            node = data['nodes'][path]
        else:
            print(f"路径 {path}: 未找到")
            return
    syns = node.get('synapses', [])
    print(f"[出射] {path} ({len(syns)} 条突触):")
    for s in sorted(syns, key=lambda x: -x['weight']):
        print(f"  [{s['type']}] w={s['weight']:.1f} phase={s['phase']} → {s['to']}")

def cmd_in(data, path):
    """入射: 谁引用了我"""
    incoming = []
    for p, node in data['nodes'].items():
        for s in node.get('synapses', []):
            target = s['to']
            if path in target or target == path:
                incoming.append({'from': p, 'type': s['type'], 'weight': s['weight']})
    print(f"[入射] {path} ({len(incoming)} 条反向链接):")
    for inc in sorted(incoming, key=lambda x: -x['weight'])[:50]:
        print(f"  [{inc['type']}] w={inc['weight']:.1f} ← {inc['from']}")

def cmd_path(data, src, dst, max_depth):
    """DFS路径查找"""
    visited = set()
    path_stack = []

    # 模糊匹配
    def resolve(name):
        if name in data['nodes']:
            return name
        matches = [p for p in data['nodes'] if name in p or p.endswith(name)]
        return matches[0] if matches else None

    src_path = resolve(src)
    dst_path = resolve(dst)
    if not src_path or not dst_path:
        print("路径解析失败")
        return

    def dfs(current, depth):
        if depth > max_depth: return False
        if current in visited: return False
        visited.add(current)
        node = data['nodes'].get(current, {})
        for s in node.get('synapses', []):
            nxt = s['to']
            path_stack.append({'from': current, 'to': nxt, 'type': s['type']})
            if nxt == dst_path or dst_path in nxt:
                return True
            if dfs(nxt, depth + 1):
                return True
            path_stack.pop()
        return False

    print(f"查找路径: {src_path} → {dst_path} (最深 {max_depth} 跳)...")
    if dfs(src_path, 0):
        print(f"  ✓ 找到 {len(path_stack)} 跳路径:")
        for step in path_stack:
            print(f"    [{step['type']}] {step['from']} → {step['to']}")
    else:
        print(f"  ✗ {max_depth} 跳内无路径")

def cmd_cluster(data, top_n=8):
    """简单聚类统计"""
    # 按地址首维度 (SI=声母族) 分组统计
    si_dist = {}
    yi_dist = {}
    for _, node in data['nodes'].items():
        addr = node.get('address', '')
        parts = addr.split(':')
        if len(parts) >= 3:
            si, yi = parts[0], parts[2]
            si_dist[si] = si_dist.get(si, 0) + 1
            yi_dist[yi] = yi_dist.get(yi, 0) + 1

    print("── 声母族(SI)分布 (Top 10) ──")
    for si, cnt in sorted(si_dist.items(), key=lambda x: -x[1])[:10]:
        print(f"  SI={int(si):2d}: {'█' * min(cnt // 100, 80)} {cnt}")

    print("── 韵母(YI)分布 (Top 10) ──")
    for yi, cnt in sorted(yi_dist.items(), key=lambda x: -x[1])[:10]:
        print(f"  YI={int(yi):2d}: {'█' * min(cnt // 100, 80)} {cnt}")

    print(f"\n总节点: {data['stats']['total_files']}")
    print(f"总突触: {data['stats']['total_synapses']}")

def cmd_stats(data):
    """全量统计"""
    stats = data['stats']
    addr_space = data['meta']['address_space']
    print("═══ 满全法 · 神经突触统计 ═══")
    print(f"地址空间: {addr_space}")
    print(f"注册节点: {stats['total_files']:,}")
    print(f"注册突触: {stats['total_synapses']:,}")
    print(f"跳过文件: {stats['skipped']}")
    print(f"节点密度: {stats['total_files']/float(addr_space.split('=')[-1].replace(',',''))*100:.6f}%")
    print()
    print("── 文件类型 Top 10 ──")
    for ext, cnt in sorted(stats['by_ext'].items(), key=lambda x: -x[1])[:10]:
        print(f"  {ext or '(无扩展名)'}: {cnt:,}")
    print()
    print("── 功能分类族 ──")
    for cat, cnt in sorted(stats['by_category'].items(), key=lambda x: -x[1])[:10]:
        print(f"  族{cat}: {cnt:,}")

def cmd_proj(data, path):
    """三维容器投影"""
    node = data['nodes'].get(path)
    if not node:
        matches = [p for p in data['nodes'] if path in p]
        if matches:
            path = matches[0]
            node = data['nodes'][path]
    if not node:
        print(f"路径: 未找到")
        return

    a = node['address']
    parts = a.split(':')
    if len(parts) != 5:
        return
    si, ti, yi, ci, mi = int(parts[0]), int(parts[1]), int(parts[2]), int(parts[3]), int(parts[4])

    # 相位投影
    x = ci % 7
    y = (ci >> 3) % 6
    z = yi * 10 + mi
    phase = (si + ti) % 12
    layer = ci // 7

    print(f"三维投影: {path}")
    print(f"  五维地址: {a}")
    print(f"  几何坐标: x={x} y={y} z={z}")
    print(f"  相位角度: {phase * 30}° (六边形 {phase}/12)")
    print(f"  容器层: 层{layer}")
    print(f"  声母族: SI={si} (声母{si % 24}/{'小写' if si < 24 else '大写'})")
    print(f"  韵母:   YI={yi}")
    print(f"  色阶:   CI={ci}")
    print(f"  微位置: MI={mi} (x={mi % 10}, y={mi // 10})")

def cmd_color(data, ci_val):
    """按COLOR分量查找"""
    matches = [(p, n) for p, n in data['nodes'].items()
               if n['address'].split(':')[3] == str(ci_val)]
    print(f"CI={ci_val} → {len(matches)} 节点")
    for p, n in matches[:20]:
        print(f"  {p} ({n['filename']})")

def cmd_yunmu(data, yi_val):
    """按韵母YI分量查找"""
    matches = [(p, n) for p, n in data['nodes'].items()
               if n['address'].split(':')[2] == str(yi_val)]
    print(f"YI={yi_val} → {len(matches)} 节点")
    for p, n in matches[:20]:
        print(f"  {p} ({n['filename']})")

def _print_node(node, full=False):
    if full:
        print(f"路径:   {node['path']}")
        print(f"文件名: {node['filename']}")
        print(f"扩展名: {node['ext']}")
        print(f"地址:   {node['address']}")
        print(f"大小:   {node['size']} bytes")
        print(f"修改:   {node['mtime']}")
        print(f"突触数: {node['synapse_count']}")
        if node.get('synapses'):
            print("  出射:")
            for s in node['synapses'][:10]:
                print(f"    [{s['type']}] → {s['to']}")
    else:
        print(f"  [{node['address']}] {node['path']} ({node['filename']}) ⇄{node['synapse_count']}")

def main():
    data = load()
    args = sys.argv[1:]
    if not args:
        print("用法:")
        print("  addr <SI:TI:YI:CI:MI>    按地址查")
        print("  find <name>              文件名模糊查")
        print("  out <path>               出射态射")
        print("  in <path>                入射反向链接")
        print("  path <src> <dst> [depth] 思维链路径")
        print("  cluster                  聚类统计")
        print("  stats                    总体统计")
        print("  proj <path>              三维投影")
        print("  color <ci>               按颜色分量")
        print("  yunmu <yi>               按韵母分量")
        return

    cmd = args[0]
    if cmd == 'addr' and len(args) > 1:
        cmd_addr(data, args[1])
    elif cmd == 'find' and len(args) > 1:
        cmd_find(data, args[1])
    elif cmd == 'out' and len(args) > 1:
        cmd_out(data, args[1])
    elif cmd == 'in' and len(args) > 1:
        cmd_in(data, args[1])
    elif cmd == 'path' and len(args) > 3:
        cmd_path(data, args[1], args[2], int(args[3]))
    elif cmd == 'path' and len(args) > 2:
        cmd_path(data, args[1], args[2], 4)
    elif cmd == 'cluster':
        cmd_cluster(data)
    elif cmd == 'stats':
        cmd_stats(data)
    elif cmd == 'proj' and len(args) > 1:
        cmd_proj(data, args[1])
    elif cmd == 'color' and len(args) > 1:
        cmd_color(data, int(args[1]))
    elif cmd == 'yunmu' and len(args) > 1:
        cmd_yunmu(data, int(args[1]))
    else:
        print(f"未知命令或参数不足: {cmd}")

if __name__ == '__main__':
    main()
