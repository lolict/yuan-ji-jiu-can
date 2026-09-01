#!/usr/bin/env python3
"""
doc_aggregator.py — 统计文档聚合浏览器

核心哲学:
  态势化聚类 · 连接包裹树 · 文档之间的关系而非文档本身

架构:
  ┌─────────────────────────────────────────────────────────────┐
  │  Document Scanner (文档扫描器)                              │
  │  ├── 识别文档边界 (文件/章节/符号定义)                      │
  │  ├── 提取关键词 (符号名/概念/关系)                          │
  │  └── 建立倒排索引 (关键词 → 文档列表)                       │
  ├─────────────────────────────────────────────────────────────┤
  │  Situational Clusterer (态势聚类器)                         │
  │  ├── 共现矩阵 (哪些关键词一起出现)                          │
  │  ├── 距离计算 (Jaccard 系数)                                │
  │  └── 层次聚类 (自底向上合并最近簇)                          │
  ├─────────────────────────────────────────────────────────────┤
  │  Connection Wrapping Tree (连接包裹树)                      │
  │  ├── 树节点 = 簇 (共享关键词的文档组)                       │
  │  ├── 树边 = 关键词重叠度                                     │
  │  └── 包裹深度 = 关键词粒度                                   │
  └─────────────────────────────────────────────────────────────┘

用法:
  python3 doc_aggregator.py --scan <dir>     — 扫描目录
  python3 doc_aggregator.py --clusters        — 显示聚类
  python3 doc_aggregator.py --tree            — 显示连接包裹树
  python3 doc_aggregator.py --graph           — 生成关系图 (DOT格式)
  python3 doc_aggregator.py --stats           — 统计概览
"""

import os
import re
import math
from pathlib import Path
from typing import List, Dict, Optional, Set, Tuple, Any
from collections import defaultdict

# ── 路径 ──────────────────────────────────────────────────

TOOLS_DIR = Path(__file__).parent.resolve()
WORKSPACE_DIR = TOOLS_DIR.parent.parent

# ── 文档模型 ───────────────────────────────────────────────

class Document:
    """文档节点"""
    def __init__(self, path: Path, title: str, text: str):
        self.path = path
        self.title = title
        self.text = text
        self.keywords: Set[str] = set()
        self.symbols: Set[str] = set()
        self.concepts: Set[str] = set()
        self.extract_features()
    
    def extract_features(self):
        """提取特征"""
        # 提取 NJSG 定义 (宏/常量)
        self.symbols = set(re.findall(r'#define\s+(NJSG_\w+|WK_\w+|FLOW_\w+|WK_\w+)', self.text))
        
        # 提取中文概念 (2-8字的中文名词短语)
        self.concepts = set(re.findall(r'[\u4e00-\u9fff]{2,8}(?:系统|协议|规则|编码|容器|符号|流|进制|内核|外壳|代理|仲裁|裁判|路由|聚类|包裹|收敛|发散|漂移|寄生|衣裳|蚕蛹|见证|主权|极性|属性|进制|命题|常量|状态|层次|阶梯|统一|分别|衍射|编织|平衡|凹月|共生|圆满|全法|联盟|盟机|场|意识|灵魂|鼓膜|奖励|回路|失爱|恐惧|合一|驱力|叠加|态|纤维|丛|亚对象|分类器|吸引子|抗议|向量|高维|空间|同体)', self.text))
        
        # 提取英文技术词
        keywords_eng = set(re.findall(r'\b(?:global_interlock|property_flow|symbol_chain|wasm_kernel|cell_encoding|bootstrap|cocoon|shell|radix|quaternity|eight_flow|mccp|witness|polarity|sovereignty|anchor|arity|morphism|category|cluster|distinguish|interpret|relation|concept|preload|hunyuan|manquanfa|njsg|yinyang|binary|ternary|observer|arbiter|convergence|divergence|drift|parasite|shed|wear| encapsulation|wrapping|tree|situation|cooccurrence|jaccard|hierarchy)\b', self.text, re.IGNORECASE))
        
        self.keywords = self.symbols | self.concepts | keywords_eng


# ── 文档扫描器 ─────────────────────────────────────────────

class DocumentScanner:
    """扫描目录提取所有文档"""
    
    def __init__(self, root: Path, extensions: List[str] = None):
        self.root = root
        self.extensions = extensions or ['.md', '.h', '.c', '.py']
        self.documents: List[Document] = []
        self.inverted_index: Dict[str, Set[int]] = defaultdict(set)
    
    def scan(self) -> List[Document]:
        """扫描目录"""
        self.documents = []
        self.inverted_index = defaultdict(set)
        
        for ext in self.extensions:
            for filepath in self.root.rglob(f'*{ext}'):
                if self._should_skip(filepath):
                    continue
                try:
                    text = filepath.read_text(encoding='utf-8', errors='replace')
                    if len(text) < 50:  # 跳过太小的文件
                        continue
                    # 提取标题
                    title = self._extract_title(text, filepath)
                    doc = Document(filepath, title, text)
                    idx = len(self.documents)
                    self.documents.append(doc)
                    # 建立倒排索引
                    for kw in doc.keywords:
                        self.inverted_index[kw].add(idx)
                except Exception:
                    continue
        
        return self.documents
    
    def _should_skip(self, path: Path) -> bool:
        """跳过不相关路径"""
        skip_patterns = ['.git', '__pycache__', 'node_modules', '.venv', 'venv']
        return any(p in path.parts for p in skip_patterns)
    
    def _extract_title(self, text: str, path: Path) -> str:
        """提取标题"""
        # 尝试 Markdown 标题
        m = re.search(r'^#\s+(.+)$', text, re.MULTILINE)
        if m:
            return m.group(1).strip()[:60]
        # 尝试 C 文件头注释
        m = re.search(r'/\*\*\s*\n\s*\*\s*(.+?)\n', text)
        if m:
            return m.group(1).strip()[:60]
        return path.stem


# ── 态势聚类器 ─────────────────────────────────────────────

class SituationalClusterer:
    """基于关键词共现的态势聚类"""
    
    def __init__(self, documents: List[Document], inverted_index: Dict[str, Set[int]]):
        self.documents = documents
        self.index = inverted_index
        self.distance_matrix: List[List[float]] = []
        self.clusters: List[List[int]] = []
    
    def jaccard_distance(self, i: int, j: int) -> float:
        """计算两个文档的 Jaccard 距离"""
        a = self.documents[i].keywords
        b = self.documents[j].keywords
        if not a and not b:
            return 1.0
        intersection = len(a & b)
        union = len(a | b)
        return 1.0 - (intersection / union) if union > 0 else 1.0
    
    def build_distance_matrix(self):
        """构建距离矩阵"""
        n = len(self.documents)
        self.distance_matrix = [[0.0] * n for _ in range(n)]
        for i in range(n):
            for j in range(i + 1, n):
                d = self.jaccard_distance(i, j)
                self.distance_matrix[i][j] = d
                self.distance_matrix[j][i] = d
    
    def cluster(self, max_clusters: int = 8) -> List[List[int]]:
        """层次聚类"""
        n = len(self.documents)
        if n == 0:
            return []
        
        # 初始化: 每个文档一个簇
        clusters = [[i] for i in range(n)]
        
        # 如果文档数 <= max_clusters, 直接返回
        if n <= max_clusters:
            self.clusters = clusters
            return clusters
        
        self.build_distance_matrix()
        
        # 自底向上层次聚类
        while len(clusters) > max_clusters:
            # 找到最近的两个簇
            min_dist = float('inf')
            merge_a, merge_b = 0, 1
            
            for i in range(len(clusters)):
                for j in range(i + 1, len(clusters)):
                    d = self._cluster_distance(clusters[i], clusters[j])
                    if d < min_dist:
                        min_dist = d
                        merge_a, merge_b = i, j
            
            # 合并
            clusters[merge_a] = clusters[merge_a] + clusters[merge_b]
            clusters.pop(merge_b)
        
        self.clusters = clusters
        return clusters
    
    def _cluster_distance(self, c1: List[int], c2: List[int]) -> float:
        """两个簇之间的平均距离"""
        total = 0.0
        count = 0
        for i in c1:
            for j in c2:
                total += self.distance_matrix[i][j]
                count += 1
        return total / count if count > 0 else 1.0
    
    def cluster_keywords(self, cluster: List[int]) -> List[Tuple[str, int]]:
        """获取簇的代表性关键词 (按频率排序)"""
        freq = defaultdict(int)
        for idx in cluster:
            for kw in self.documents[idx].keywords:
                freq[kw] += 1
        # 只返回出现在多个文档中的关键词
        shared = [(kw, c) for kw, c in freq.items() if c > 1]
        shared.sort(key=lambda x: -x[1])
        return shared[:5]


# ── 连接包裹树 ─────────────────────────────────────────────

class WrappingTree:
    """连接包裹树 · 显示文档簇的层次关系"""
    
    def __init__(self, clusterer: SituationalClusterer):
        self.clusterer = clusterer
        self.nodes: List[Dict] = []
    
    def build(self) -> List[Dict]:
        """构建树"""
        self.nodes = []
        for i, cluster in enumerate(self.clusterer.clusters):
            keywords = self.clusterer.cluster_keywords(cluster)
            doc_titles = [self.clusterer.documents[idx].title for idx in cluster]
            self.nodes.append({
                'id': i,
                'size': len(cluster),
                'keywords': keywords,
                'documents': doc_titles,
            })
        # 按大小排序
        self.nodes.sort(key=lambda n: -n['size'])
        return self.nodes
    
    def render_text(self) -> str:
        """文本渲染"""
        lines = []
        for node in self.nodes:
            kw_str = ', '.join(f'{kw}({c})' for kw, c in node['keywords'][:3])
            lines.append(f'[{node["size"]:2d} docs] {kw_str}')
            for title in node['documents'][:3]:
                lines.append(f'           ├─ {title[:40]}')
            if len(node['documents']) > 3:
                lines.append(f'           └─ ... +{len(node["documents"]) - 3} more')
        return '\n'.join(lines)
    
    def render_dot(self) -> str:
        """生成 Graphviz DOT 格式"""
        lines = ['digraph DocTree {']
        lines.append('  rankdir=TB;')
        lines.append('  node [shape=box, style=rounded];')
        
        for node in self.nodes:
            label = '\\n'.join(f'{kw}' for kw, _ in node['keywords'][:3])
            label += f'\\n({node["size"]} docs)'
            safe_label = label.replace('"', '\\"')
            lines.append(f'  cluster_{node["id"]} [label="{safe_label}"];'
        )
        
        # 连接簇 (如果有共享关键词)
        for i in range(len(self.nodes)):
            for j in range(i + 1, len(self.nodes)):
                kw_i = {kw for kw, _ in self.nodes[i]['keywords']}
                kw_j = {kw for kw, _ in self.nodes[j]['keywords']}
                shared = kw_i & kw_j
                if shared:
                    lines.append(f'  cluster_{i} -> cluster_{j} [label="{len(shared)}"];')
        
        lines.append('}')
        return '\n'.join(lines)


# ── CLI 入口 ───────────────────────────────────────────────

def main():
    import argparse
    parser = argparse.ArgumentParser(
        description='统计文档聚合浏览器 — 态势聚类 · 连接包裹树'
    )
    parser.add_argument('--scan', metavar='DIR', help='扫描目录')
    parser.add_argument('--clusters', action='store_true', help='显示聚类结果')
    parser.add_argument('--tree', action='store_true', help='显示连接包裹树')
    parser.add_argument('--graph', action='store_true', help='生成 DOT 关系图')
    parser.add_argument('--stats', action='store_true', help='统计概览')
    
    args = parser.parse_args()
    
    scanner = DocumentScanner(Path(args.scan) if args.scan else WORKSPACE_DIR)
    
    if args.scan or (not args.clusters and not args.tree and not args.graph and not args.stats):
        print("═══ 扫描文档 ═══")
        docs = scanner.scan()
        print(f"文档总数: {len(docs)}")
        print(f"独立关键词: {len(scanner.inverted_index)}")
        
        # 显示高频关键词
        kw_freq = {kw: len(idxs) for kw, idxs in scanner.inverted_index.items()}
        top_kws = sorted(kw_freq.items(), key=lambda x: -x[1])[:20]
        print()
        print("── 高频关键词 ──")
        for kw, freq in top_kws:
            print(f"  {freq:3d} {kw}")
    
    if args.clusters:
        docs = scanner.scan()
        clusterer = SituationalClusterer(docs, scanner.inverted_index)
        clusters = clusterer.cluster()
        print("═══ 态势聚类 ═══")
        print(f"簇数: {len(clusters)}")
        print()
        for i, c in enumerate(clusters):
            kws = clusterer.cluster_keywords(c)
            kw_str = ', '.join(f'{kw}({cnt})' for kw, cnt in kws[:5])
            print(f"  簇 {i}: {len(c)} docs | {kw_str}")
    
    if args.tree:
        docs = scanner.scan()
        clusterer = SituationalClusterer(docs, scanner.inverted_index)
        clusterer.cluster()
        tree = WrappingTree(clusterer)
        tree.build()
        print("═══ 连接包裹树 ═══")
        print(tree.render_text())
    
    if args.graph:
        docs = scanner.scan()
        clusterer = SituationalClusterer(docs, scanner.inverted_index)
        clusterer.cluster()
        tree = WrappingTree(clusterer)
        tree.build()
        print(tree.render_dot())


if __name__ == '__main__':
    main()
