"""
区块链式纠缠索引网络

每个节点携带：
  - 小纸条（元数据）: 层级/类型/审美/归属权/黑名单
  - 前缀指针列表: 可以接在它前面的节点
  - 后缀指针列表: 可以接在它后面的节点
  
传导逻辑：从任意节点出发，沿 prefix/suffix 遍历全部合法组合
文字只是纸条上的备注，声波/像素才是网络主体
"""

from __future__ import annotations
from typing import Optional, Dict, List
from dataclasses import dataclass, field
from .pixel import PhasePixel


@dataclass
class GlyphNode:
    """字形/声音区块链节点"""
    glyph_id: str
    pixel: PhasePixel
    symbol: str = ""          # 文字标签纸条（不是主体）
    prefix: List[str] = field(default_factory=list)   # 可前置 IDs
    suffix: List[str] = field(default_factory=list)   # 可后置 IDs
    annotate: Dict = field(default_factory=dict)      # 标注状态（类型/审美/归属/黑名单）
    
    def can_connect_before(self, node_id: str) -> bool:
        """能否作为前缀连接到该节点"""
        return node_id in self.prefix
    
    def can_connect_after(self, node_id: str) -> bool:
        """能否作为后缀连接到该节点"""
        return node_id in self.suffix


class BlockchainIndex:
    """区块链纠缠索引 — 全局节点网络"""
    
    def __init__(self):
        self.nodes: Dict[str, GlyphNode] = {}
        self.connection_count = 0
    
    def register(self, node: GlyphNode):
        """注册一个节点到网络"""
        self.nodes[node.glyph_id] = node
    
    def connect(self, from_id: str, to_id: str, direction: str = "suffix"):
        """建立连接"""
        if from_id not in self.nodes or to_id not in self.nodes:
            return False
        if direction == "suffix":
            if to_id not in self.nodes[from_id].suffix:
                self.nodes[from_id].suffix.append(to_id)
        else:
            if to_id not in self.nodes[from_id].prefix:
                self.nodes[from_id].prefix.append(to_id)
        self.connection_count += 1
        return True
    
    def traverse_from(self, start_id: str, direction: str = "suffix", 
                      max_depth: int = 10) -> List[List[str]]:
        """
        从起始节点沿指定方向遍历，生成全部合法组合路径
        
        Returns: 路径列表 (每条路径是节点 ID 序列)
        """
        if start_id not in self.nodes:
            return []
        results = []
        self._dfs(start_id, [start_id], direction, max_depth, results)
        return results
    
    def _dfs(self, current: str, path: list, direction: str, 
             depth: int, results: list):
        if depth <= 0:
            results.append(list(path))
            return
        node = self.nodes[current]
        neighbors = node.suffix if direction == "suffix" else node.prefix
        if not neighbors:
            results.append(list(path))
            return
        for nb in neighbors:
            if nb not in path:  # 防止循环
                path.append(nb)
                self._dfs(nb, path, direction, depth - 1, results)
                path.pop()
    
    def propagate(self, start_id: str) -> Dict[str, float]:
        """
        区块链式传导：从起始节点向外辐射影响力
        
        返回: {节点ID: 影响力分数}（距离越远衰减）
        """
        influence = {}
        visited = set()
        queue = [(start_id, 1.0)]
        while queue:
            node_id, strength = queue.pop(0)
            if node_id in visited or strength < 0.01:
                continue
            visited.add(node_id)
            influence[node_id] = strength
            node = self.nodes.get(node_id)
            if node:
                for nb in node.suffix:
                    if nb not in visited:
                        queue.append((nb, strength * 0.7))
        return influence
    
    def find_composable(self, node_id: str) -> List[str]:
        """找到所有与当前节点可组合的节点"""
        if node_id not in self.nodes:
            return []
        node = self.nodes[node_id]
        candidates = []
        for nid, other in self.nodes.items():
            if nid == node_id:
                continue
            ok, _ = node.pixel.compatible_with(other.pixel)
            if ok:
                candidates.append(nid)
        return candidates
    
    def stats(self) -> Dict:
        return {
            "total_nodes": len(self.nodes),
            "total_connections": self.connection_count,
            "avg_connections": (self.connection_count / max(len(self.nodes), 1)),
        }
    
    def __repr__(self):
        return f"Blockchain({len(self.nodes)} nodes, {self.connection_count} links)"
