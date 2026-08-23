"""
区块链式纠缠索引网络

每个信号节点携带元数据小纸条：
  - prefix: 可以接在它前面的节点列表
  - suffix: 可以接在它后面的节点列表
  - annotate: 层级、类型、归属权、审美打分、黑名单标记

从任意节点出发，沿 prefix/suffix 链式传导，
生成全部合法组合。全局连续性由此实现。

这不是加密区块链，是借用它的：
  - 链式传导
  - 互相锚定
  - 每条节点携带元数据标注
  - 全局可回溯索引
"""

from dataclasses import dataclass, field
from typing import Dict, List, Optional, Set, Tuple
import json


@dataclass
class SignalNode:
    """
    信号节点 = 一个声音/字形单元 + 元数据纸条
    
    主体是信号（声波/像素），不是文字。
    文字只是 annotate 里的备注标签。
    """
    signal_id: str                          # 唯一标识
    signal_type: str                        # "audio" 或 "glyph"
    
    # 链式传导指针
    prefix: List[str] = field(default_factory=list)   # 可以接在前面的节点
    suffix: List[str] = field(default_factory=list)   # 可以接在后面的节点
    
    # 元数据纸条（annotate）
    annotate: Dict = field(default_factory=lambda: {
        "level": "L1",           # L1/L2/L3
        "type": "unknown",       # 声母/韵母/声调/修饰零件
        "owner": "SELF",         # SELF/EXTERNAL
        "visual_weight": 5.0,    # 视觉重量
        "family": "latin",       # 字形家族
        "aesthetic_score": 0.0,  # 审美打分
        "blacklist": False,      # 是否在黑名单
        "goal": "system",        # 目标锚点
        "glyph_label": "",       # 文字标签（只是备注）
        "unicode_char": "",      # Unicode 字符（只是备注）
    })
    
    def can_precede(self, other_id: str) -> bool:
        """这个节点能不能放在 other 前面"""
        return other_id in self.suffix
    
    def can_follow(self, other_id: str) -> bool:
        """这个节点能不能放在 other 后面"""
        return other_id in self.prefix
    
    def to_dict(self) -> Dict:
        return {
            "signal_id": self.signal_id,
            "signal_type": self.signal_type,
            "prefix": self.prefix,
            "suffix": self.suffix,
            "annotate": self.annotate,
        }
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'SignalNode':
        node = cls(
            signal_id=data["signal_id"],
            signal_type=data.get("signal_type", "glyph"),
            prefix=data.get("prefix", []),
            suffix=data.get("suffix", []),
        )
        node.annotate = {**node.annotate, **data.get("annotate", {})}
        return node


class BlockchainIndex:
    """
    区块链式索引网络 = 所有信号节点的纠缠关联图
    
    从任意节点出发，沿 prefix/suffix 链式传导，
    可以遍历所有合法组合。
    """
    
    def __init__(self):
        self.nodes: Dict[str, SignalNode] = {}
        self.goal: str = "满全法·私有语音编码体系"  # 全局公约数
    
    def add_node(self, node: SignalNode) -> None:
        """添加节点到网络"""
        self.nodes[node.signal_id] = node
    
    def get_node(self, signal_id: str) -> Optional[SignalNode]:
        """获取节点"""
        return self.nodes.get(signal_id)
    
    def link(self, from_id: str, to_id: str, bidirectional: bool = False) -> None:
        """
        建立连接：from 可以接在 to 前面
        即 from.suffix += to, to.prefix += from
        """
        from_node = self.nodes.get(from_id)
        to_node = self.nodes.get(to_id)
        
        if not from_node or not to_node:
            return
        
        if to_id not in from_node.suffix:
            from_node.suffix.append(to_id)
        if from_id not in to_node.prefix:
            to_node.prefix.append(from_id)
        
        if bidirectional:
            if from_id not in to_node.suffix:
                to_node.suffix.append(from_id)
            if to_id not in from_node.prefix:
                from_node.prefix.append(to_id)
    
    def traverse_from(self, start_id: str, max_depth: int = 5) -> List[List[str]]:
        """
        从起始节点出发，链式传导遍历所有合法路径
        
        返回：所有可能的节点序列（每个序列是一个合法组合）
        """
        if start_id not in self.nodes:
            return []
        
        paths = []
        self._traverse_recursive(start_id, [start_id], max_depth, paths)
        return paths
    
    def _traverse_recursive(self, current_id: str, path: List[str], 
                            max_depth: int, paths: List[List[str]]):
        if len(path) >= max_depth:
            paths.append(path[:])
            return
        
        node = self.nodes.get(current_id)
        if not node or not node.suffix:
            paths.append(path[:])
            return
        
        for next_id in node.suffix:
            if next_id not in path:  # 避免循环
                path.append(next_id)
                self._traverse_recursive(next_id, path, max_depth, paths)
                path.pop()
    
    def find_combinations(self, start_id: str, target_length: int = 3) -> List[Dict]:
        """
        从起始节点出发，找到所有指定长度的合法组合
        
        返回：每个组合包含节点序列和综合元数据
        """
        paths = self.traverse_from(start_id, max_depth=target_length)
        combinations = []
        
        for path in paths:
            if len(path) != target_length:
                continue
            
            nodes = [self.nodes.get(pid) for pid in path]
            if any(n is None for n in nodes):
                continue
            
            # 综合元数据
            combined_annotate = {
                "levels": [n.annotate.get("level", "L1") for n in nodes],
                "types": [n.annotate.get("type", "unknown") for n in nodes],
                "owners": [n.annotate.get("owner", "SELF") for n in nodes],
                "blacklist_flags": [n.annotate.get("blacklist", False) for n in nodes],
                "glyph_labels": [n.annotate.get("glyph_label", "") for n in nodes],
            }
            
            # 检查是否有黑名单节点
            has_blacklist = any(combined_annotate["blacklist_flags"])
            
            # 检查层级一致性
            levels_consistent = len(set(combined_annotate["levels"])) == 1
            
            combinations.append({
                "path": path,
                "annotate": combined_annotate,
                "valid": not has_blacklist,
                "levels_consistent": levels_consistent,
                "reason": "包含黑名单节点" if has_blacklist else "合法组合",
            })
        
        return combinations
    
    def global_presence(self, source_id: str) -> Dict[str, float]:
        """
        全局在场：从源节点出发，计算所有节点的在场强度
        
        沿 prefix/suffix 链式传导，每跳衰减。
        不是"检查"全局，而是全局自然在场。
        """
        if source_id not in self.nodes:
            return {}
        
        presence = {source_id: 1.0}
        current_level = {source_id}
        decay = 0.6
        
        for hop in range(4):  # 最多4跳
            next_level = set()
            for node_id in current_level:
                node = self.nodes.get(node_id)
                if not node:
                    continue
                
                for neighbor_id in node.suffix + node.prefix:
                    if neighbor_id not in presence:
                        presence[neighbor_id] = decay ** (hop + 1)
                        next_level.add(neighbor_id)
            
            current_level = next_level
            if not current_level:
                break
        
        return presence
    
    def to_json(self) -> str:
        """序列化为 JSON"""
        data = {
            "goal": self.goal,
            "nodes": {nid: node.to_dict() for nid, node in self.nodes.items()},
        }
        return json.dumps(data, ensure_ascii=False, indent=2)
    
    @classmethod
    def from_json(cls, json_str: str) -> 'BlockchainIndex':
        """从 JSON 反序列化"""
        data = json.loads(json_str)
        index = cls()
        index.goal = data.get("goal", index.goal)
        
        for nid, node_data in data.get("nodes", {}).items():
            index.add_node(SignalNode.from_dict(node_data))
        
        return index
    
    def statistics(self) -> Dict:
        """网络统计"""
        return {
            "total_nodes": len(self.nodes),
            "total_connections": sum(
                len(n.prefix) + len(n.suffix) for n in self.nodes.values()
            ),
            "by_type": self._count_by_type(),
            "by_level": self._count_by_level(),
            "blacklist_count": sum(
                1 for n in self.nodes.values() if n.annotate.get("blacklist", False)
            ),
        }
    
    def _count_by_type(self) -> Dict[str, int]:
        counts = {}
        for node in self.nodes.values():
            t = node.annotate.get("type", "unknown")
            counts[t] = counts.get(t, 0) + 1
        return counts
    
    def _count_by_level(self) -> Dict[str, int]:
        counts = {}
        for node in self.nodes.values():
            l = node.annotate.get("level", "L1")
            counts[l] = counts.get(l, 0) + 1
        return counts
