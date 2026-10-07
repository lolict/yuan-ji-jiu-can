#!/usr/bin/env python3
# 满全法 · 神经突触注册器 (Synapse Registry)
# 功能:
#   遍历工作区全部文件 → 5D地址确定性映射 → 态射(关系)抽取 → 生成注册清单
#
# 输出:
#   synapses.json   — 全量注册 (地址+态射+元数据)
#   synapses_index — 快速查询索引
#
# 架构:
#   Node(节点) = 一个文件 = 一个五维地址
#   Synapse(突射) = 一条态射 = Node A → Node B (关系类型+权重+相位)
#   Field(场空间) = 节点在三维容器中的投影位置

import os, re, json, hashlib, time, sqlite3, sys
from pathlib import Path
from os.path import relpath, basename, dirname, splitext

# ── 工作区根 ──
WORKSPACE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
# WORKSPACE = /.../hunyuan-os
WORKSPACE = os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..'))

# ── 排除路径 ──
SKIP_DIRS = {'.git', 'target', 'node_modules', '__pycache__', '.cargo', '.lib',
             '.bin', '.cache', 'stage0_output', 'stage1_output', 'stage2_output',
             'shared_output', '.github'}
SKIP_EXTS = {'.o', '.a', '.so', '.dylib', '.exe', '.dll',
             '.png', '.jpg', '.jpeg', '.gif', '.bmp', '.ico', '.icns',
             '.ttf', '.otf', '.woff', '.woff2', '.eot',
             '.zip', '.tar', '.gz', '.rar', '.7z',
             '.pdf', '.doc', '.docx', '.xls', '.xlsx',
             '.wasm', '.pyc', '.class', '.jar',
             # 禁用: Python / JS / HTML / CSS / JSON / lock / log / yaml / toml
             '.py', '.js', '.jsx', '.ts', '.tsx', '.mjs', '.cjs',
             '.html', '.htm', '.css', '.scss', '.sass', '.less',
             '.json', '.jsonl', '.json5',
             '.yaml', '.yml', '.toml', '.ini', '.cfg', '.conf',
             '.lock', '.log', '.bak', '.tmp', '.swp', '.swo',
             '.map',  # source map
             }
SKIP_FILES = {'.DS_Store', 'Thumbs.db', '.gitignore', '.gitattributes',
              'Cargo.lock', 'package-lock.json', 'yarn.lock'}

# ── 关系态射类型 ──
# morphism types: import / include / link / ref / call / derive / map
MORPHISM_TYPES = {
    'import':   {'weight': 1.0, 'direction': 'unidirectional', 'phase': 'external'},
    'include':  {'weight': 1.0, 'direction': 'unidirectional', 'phase': 'external'},
    'mod':      {'weight': 0.9, 'direction': 'unidirectional', 'phase': 'internal'},
    'use':      {'weight': 0.7, 'direction': 'unidirectional', 'phase': 'internal'},
    'call':     {'weight': 0.6, 'direction': 'unidirectional', 'phase': 'runtime'},
    'link':     {'weight': 0.8, 'direction': 'bidirectional',   'phase': 'doc'},
    'ref':      {'weight': 0.5, 'direction': 'unidirectional', 'phase': 'knowledge'},
    'derive':   {'weight': 0.9, 'direction': 'unidirectional', 'phase': 'temporal'},
    'map':      {'weight': 1.0, 'direction': 'bidirectional',   'phase': 'encoding'},
    'same_as':  {'weight': 1.0, 'direction': 'bidirectional',   'phase': 'identity'},
}

# ── 五维地址参数 ──
S_DIM = 48   # 声母: shengmu
T_DIM = 60   # 声调: tone
Y_DIM = 31   # 韵母: yunmu (31个已注册韵母)
C_DIM = 42   # 颜色: color42
M_DIM = 100  # 微位置: 10×10

def content_hash_5d(text, path):
    """
    文件内容 → 五维地址 (确定性)
    hash(fpath + content) → (SI, TI, YI, CI, MI)
    """
    blob = (path + "|" + text).encode('utf-8', errors='replace')
    h = hashlib.sha256(blob).hexdigest()
    # 取前20字节 → 5维各4字节
    vals = [int(h[i*8:(i+1)*8], 16) for i in range(5)]
    return (
        vals[0] % S_DIM,   # SI
        vals[1] % T_DIM,   # TI
        vals[2] % Y_DIM,   # YI
        vals[3] % C_DIM,   # CI
        vals[4] % M_DIM,   # MI
    )

def path_to_category(relpath_str):
    """
    文件路径 → 功能分类 (声母48族)
    简化: 取top-dir → 编码族
    """
    top = relpath_str.split(os.sep)[0]
    # 大类映射
    categories = {
        'hunyuan-rs':    0,   # 核心引擎
        'tools':         1,   # 工具链
        'docs':          2,   # 文档
        'android':       3,   # 移动端
        'encoding':      4,   # 编码
        'container':     5,   # 容器
        'font':          6,   # 字体
        'color-base':    7,   # 颜色
        'bootstrap':     8,   # 自举
        'cognition_os':  9,   # 认知OS
        'cloudflare':   10,   # 边缘部署
        'deploy':       11,   # 部署
        'data':         12,   # 数据
        'core':         13,   # 内核
        'glyph_stroke': 14,   # 笔画
        'archive':      15,   # 归档
        'web':          16,   # Web
        'physics':      17,   # 物理
    }
    return categories.get(top, 25)  # 25=未归类

# ── 关系抽取器 ──
class RelationshipExtractor:
    """从源码/文档中抽取态射(关系)链接"""

    @staticmethod
    def extract_rust(src, filepath):
        links = []
        # mod foo;
        for m in re.finditer(r'^\s*mod\s+(\w+)', src, re.M):
            links.append(('mod', m.group(1), filepath))
        # use foo::bar;
        for m in re.finditer(r'^\s*use\s+(\w[\w:]*)', src, re.M):
            links.append(('use', m.group(1), filepath))
        # extern crate foo;
        for m in re.finditer(r'^\s*extern\s+crate\s+(\w+)', src, re.M):
            links.append(('import', m.group(1), filepath))
        return links

    @staticmethod
    def extract_nim(src, filepath):
        links = []
        # import foo
        for m in re.finditer(r'^\s*import\s+(\w[\w/]*)', src, re.M):
            links.append(('import', m.group(1), filepath))
        # include foo
        for m in re.finditer(r'^\s*include\s+(\w[\w/]*)', src, re.M):
            links.append(('include', m.group(1), filepath))
        # from foo import bar
        for m in re.finditer(r'^\s*from\s+(\w[\w/]*)\s+import', src, re.M):
            links.append(('import', m.group(1), filepath))
        return links

    @staticmethod
    def extract_c(src, filepath):
        links = []
        # #include "foo.h"
        for m in re.finditer(r'#include\s+"([^"]+)"', src):
            links.append(('include', m.group(1), filepath))
        return links

    @staticmethod
    def extract_python(src, filepath):
        links = []
        # import foo
        for m in re.finditer(r'^\s*import\s+(\w[\w.]*)', src, re.M):
            links.append(('import', m.group(1), filepath))
        # from foo import bar
        for m in re.finditer(r'^\s*from\s+(\w[\w.]*)\s+import', src, re.M):
            links.append(('import', m.group(1), filepath))
        return links

    @staticmethod
    def extract_markdown(src, filepath):
        links = []
        # [text](path) 或 [text](url)
        for m in re.finditer(r'\[([^\]]+)\]\(([^)]+)\)', src):
            target = m.group(2)
            # 只取本地链接
            if not target.startswith(('http://', 'https://', 'mailto:')):
                links.append(('link', target, filepath))
        # 关联: 引用文件名 (不带路径的 .rs/.nim/.md 名)
        for m in re.finditer(r'\b(\w+\.(?:rs|nim|c|h|md|py|toml|json))\b', src):
            links.append(('ref', m.group(1), filepath))
        return links

    @staticmethod
    def extract_generic(src, filepath):
        """跨文件软链接引用: 纯文件名匹配其他文件"""
        links = []
        # 引用其他源文件的裸名
        for m in re.finditer(r'\b(\w{3,}\.(?:rs|nim|c|h|md|py|toml))\b', src):
            links.append(('ref', m.group(1), filepath))
        return links

# ── 主注册器 ──
class SynapseRegistry:
    def __init__(self, root_dir):
        self.root = root_dir
        self.nodes = {}       # {relpath: {address, meta, synpases}}
        self.name_index = {}  # {filename: [relpath, ...]}  用于name→file解析
        self.stats = {
            'total_files': 0,
            'total_synapses': 0,
            'by_ext': {},
            'by_category': {},
            'skipped': 0,
        }

    def scan(self):
        """遍历工作区 → 节点注册"""
        print(f"扫描工作区: {self.root}")
        for dirpath, dirnames, filenames in os.walk(self.root):
            # 跳过目录
            dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS
                           and not d.startswith('.git')]
            for fn in filenames:
                if fn in SKIP_FILES:
                    self.stats['skipped'] += 1
                    continue
                ext = splitext(fn)[1].lower()
                if ext in SKIP_EXTS:
                    self.stats['skipped'] += 1
                    continue
                fullpath = os.path.join(dirpath, fn)
                rel = relpath(fullpath, self.root)
                self._register_file(fullpath, rel, ext, fn)
        print(f"  节点注册: {len(self.nodes)}")

    def _register_file(self, fullpath, rel, ext, fn):
        """单文件 → 节点注册"""
        try:
            with open(fullpath, 'r', encoding='utf-8', errors='replace') as f:
                content = f.read(65536)  # 前64KB用于测速
            full_len = os.path.getsize(fullpath)
            mtime = int(os.path.getmtime(fullpath))
        except Exception:
            return

        # 5D地址
        si, ti, yi, ci, mi = content_hash_5d(content[:4096], rel)

        # 分类
        cat = path_to_category(rel)

        node = {
            'path': rel,
            'filename': fn,
            'ext': ext,
            'category': cat,
            'mtime': mtime,
            'size': full_len,
            'address': {'si': si, 'ti': ti, 'yi': yi, 'ci': ci, 'mi': mi},
            'address_str': f"{si}:{ti}:{yi}:{ci}:{mi}",
            'synapses': [],  # 待填充
        }

        self.nodes[rel] = node
        self.stats['total_files'] += 1
        self.stats['by_ext'][ext] = self.stats['by_ext'].get(ext, 0) + 1
        self.stats['by_category'][cat] = self.stats['by_category'].get(cat, 0) + 1

        # 文件名索引 (用于后续态射解析)
        base = splitext(fn)[0]
        if base not in self.name_index:
            self.name_index[base] = []
        self.name_index[base].append(rel)

    def extract_all_synapses(self):
        """对所有源码/文档文件 → 抽取态射"""
        print("抽取态射...")
        extractor = RelationshipExtractor()
        n_extracted = 0

        for rel, node in self.nodes.items():
            fullpath = os.path.join(self.root, rel)
            ext = node['ext']
            try:
                with open(fullpath, 'r', encoding='utf-8', errors='replace') as f:
                    src = f.read(65536)
            except Exception:
                continue

            raw_links = []
            if ext == '.rs':
                raw_links = extractor.extract_rust(src, rel)
            elif ext == '.nim':
                raw_links = extractor.extract_nim(src, rel)
            elif ext in ('.c', '.h'):
                raw_links = extractor.extract_c(src, rel)
            elif ext == '.py':
                raw_links = extractor.extract_python(src, rel)
            elif ext == '.md':
                raw_links = extractor.extract_markdown(src, rel)

            # 目标链接解析: name → 目标文件路径
            for mtype, target, srcpath in raw_links:
                dest = self._resolve_target(target, srcpath)
                if dest:
                    morph_info = MORPHISM_TYPES.get(mtype, MORPHISM_TYPES['ref'])
                    node['synapses'].append({
                        'type': mtype,
                        'to': dest,
                        'weight': morph_info['weight'],
                        'phase': morph_info['phase'],
                    })
                    n_extracted += 1

        self.stats['total_synapses'] = n_extracted
        print(f"  突触总数: {n_extracted}")

    def _resolve_target(self, target_name, src_relpath):
        """
        目标名 → 文件路径解析
        尝试多种匹配: 直接 / 同名 / 路径前缀 / 大小写不敏感
        """
        # 1. 绝对路径拼接
        src_dir = dirname(src_relpath)

        # 2. 在name_index中找同名文件
        base_name = basename(target_name)
        if base_name in self.name_index:
            candidates = self.name_index[base_name]
            if len(candidates) == 1:
                return candidates[0]
            # 多候选: 选与源文件同目录或最近
            for c in candidates:
                if dirname(c) == src_dir:
                    return c
            return candidates[0]

        # 3. target_name带路径: import njsg/foo → tools/njsg/foo.nim
        if '/' in target_name:
            for ext_try in ['', '.nim', '.rs', '.md', '.py', '.c', '.h']:
                cand = target_name + ext_try
                if cand in self.nodes:
                    return cand

        # 4. 暴力搜索 (最后手段)
        base_noext = splitext(base_name)[0]
        if base_noext in self.name_index:
            return self.name_index[base_noext][0]

        return None  # 外部依赖或未解析

    def query_by_address(self, si=None, ti=None, yi=None, ci=None, mi=None):
        """按5D地址(可部分)检索"""
        results = []
        for rel, node in self.nodes.items():
            addr = node['address']
            if si is not None and addr['si'] != si: continue
            if ti is not None and addr['ti'] != ti: continue
            if yi is not None and addr['yi'] != yi: continue
            if ci is not None and addr['ci'] != ci: continue
            if mi is not None and addr['mi'] != mi: continue
            results.append(node)
        return results

    def query_by_path_fragment(self, fragment):
        """路径模糊匹配"""
        return [n for r, n in self.nodes.items() if fragment in r]

    def query_by_category(self, cat_id):
        """按功能分类族检索"""
        return [n for n in self.nodes.values() if n['category'] == cat_id]

    def query_outgoing(self, path):
        """某节点的出射态射"""
        if path in self.nodes:
            return self.nodes[path]['synapses']
        return []

    def query_incoming(self, path):
        """入射态射 (谁引用了我)"""
        incoming = []
        for rel, node in self.nodes.items():
            for syn in node['synapses']:
                if syn['to'] == path:
                    incoming.append({'from': rel, 'type': syn['type'], 'weight': syn['weight']})
        return incoming

    def query_path(self, src, dst, max_depth=4):
        """思维链路径查找 (DFS深度的优先)"""
        visited = set()
        path = []

        def dfs(current, depth):
            if depth > max_depth:
                return False
            if current in visited:
                return False
            visited.add(current)
            syns = self.query_outgoing(current)
            for syn in syns:
                nxt = syn['to']
                path.append({'from': current, 'to': nxt, 'type': syn['type']})
                if nxt == dst:
                    return True
                if dfs(nxt, depth + 1):
                    return True
                path.pop()
            return False

        if dfs(src, 0):
            return path
        return None

    def cluster_by_proximity(self, threshold=3):
        """
        相位感知聚类: 5D地址中的汉明距离 ≤ threshold 的文件为一簇
        返回:  {cluster_id: [path, ...]}
        """
        clusters = {}
        assigned = {}
        cluster_id = 0

        for rel, node in self.nodes.items():
            addr = tuple(node['address'].values())
            # 找最近的簇
            best_cid = None
            best_dist = threshold + 1
            for cid, members in clusters.items():
                for m in members[:5]:  # sampling compare
                    maddr = tuple(self.nodes[m]['address'].values())
                    dist = sum(1 for a, b in zip(addr, maddr) if a != b)
                    if dist < best_dist:
                        best_dist = dist
                        best_cid = cid
                        break
            if best_cid is not None and best_dist <= threshold:
                clusters[best_cid].append(rel)
                assigned[rel] = best_cid
            else:
                clusters[cluster_id] = [rel]
                assigned[rel] = cluster_id
                cluster_id += 1
        return clusters

    def compute_field_projection(self, path):
        """三维容器投影: 五维地址 → (x, y, z) 几何坐标
        公式 (来自 container_radix.rs 六边形蜂窝):
          偶数位 → y, 奇数位 → x, 时间版本 → z
        简化: (CI, MI) → (x, y), YI → z
        """
        node = self.nodes.get(path)
        if not node: return None
        a = node['address']
        # 相位投影
        x = a['ci'] % 7             # 7位色阶 → x
        y = (a['ci'] >> 3) % 6      # 6系 → y
        z = a['yi'] * 10 + a['mi']  # 韵母×微位置 → 深度轴
        return {'x': x, 'y': y, 'z': z, 'phase': (a['si'] + a['ti']) % 12}

    def generate_report(self):
        """生成注册与统计"""
        out = {
            'meta': {
                'system': '满全法 · 神经突触注册',
                'root': self.root,
                'generated_at': int(time.time()),
                'version': '1.0',
                'address_space': f"{S_DIM}×{T_DIM}×{Y_DIM}×{C_DIM}×{M_DIM} = {S_DIM*T_DIM*Y_DIM*C_DIM*M_DIM:,}",
                'architecture': 'Node=文件=5D地址, Synapse=态射=有向关系, Field=三维投影=相位空间',
            },
            'stats': self.stats,
            'nodes': {rel: {
                'path': n['path'],
                'filename': n['filename'],
                'ext': n['ext'],
                'category': n['category'],
                'size': n['size'],
                'mtime': n['mtime'],
                'address': n['address_str'],
                'synapse_count': len(n['synapses']),
                'synapses': n['synapses'][:50],  # 单节点限50
            } for rel, n in self.nodes.items()},
        }
        return out

# ── 主程序 ──
def main():
    reg = SynapseRegistry(WORKSPACE)
    reg.scan()
    reg.extract_all_synapses()

    # 输出JSON
    out_path = os.path.join(os.path.dirname(__file__), 'synapses.json')
    report = reg.generate_report()
    with open(out_path, 'w', encoding='utf-8') as f:
        json.dump(report, f, ensure_ascii=False, indent=1)
    print(f"\n✓ 生成 {out_path}")
    print(f"  节点: {report['stats']['total_files']}")
    print(f"  突触: {report['stats']['total_synapses']}")

    # 统计
    print(f"\n── 按扩展名 ──")
    for ext, cnt in sorted(report['stats']['by_ext'].items(), key=lambda x: -x[1])[:10]:
        print(f"  {ext or '(no ext)'}: {cnt}")
    print(f"\n── 按分类族 ──")
    for cat, cnt in sorted(report['stats']['by_category'].items(), key=lambda x: -x[1])[:10]:
        print(f"  族{cat}: {cnt}")

    # 思维链路径示例
    print("\n── 路径查找示例 ──")
    # 找一个有突触的节点作为起点
    sample_start = None
    sample_end = None
    for rel, node in reg.nodes.items():
        if node['synapses']:
            sample_start = rel
            sample_end = node['synapses'][0]['to']
            break
    if sample_start and sample_end:
        print(f"  起点: {sample_start}")
        print(f"  目标: {sample_end}")
        path = reg.query_path(sample_start, sample_end, max_depth=3)
        if path:
            for step in path:
                print(f"    → [{step['type']}] {step['to']}")
        else:
            print(f"  3跳内无路径")

    # 三维投影示例
    print("\n── 三维容器投影示例 ──")
    for rel in list(reg.nodes.keys())[:5]:
        proj = reg.compute_field_projection(rel)
        n = reg.nodes[rel]
        print(f"  {rel}: addr={n['address_str']} → field={proj}")

    print("\n── 全部完成 ──")
    return reg

if __name__ == '__main__':
    main()
