"""
囡囝共生 · 囡囝共生元操作系统 · 内核

模块:
  observer       — 二元/三元观测者空间引擎
  kernel         — 夫妻共同体进程管理 (莫刘连理)
  mythology      — 封神体系 + 囡囝四态意识拓扑
  protocol       — 囡囝共生 P2P 通信层 (私有协议)
  log_ring       — 关系日志环 (append-only + Merkle 树)
  name_anchor    — 中文姓名 → 创世锚算法
  paibing        — 病理诊断 (双/三编码)
"""

from .observer import (
    BinaryObserver,
    TernaryObserver,
    ObserverNetwork,
    build_from_double,
    build_from_triple,
)
from .kernel import (
    Process,
    ProcessState,
    CoupleOS,
)
from .mythology import (
    MYTHOS,
    get_full_mythos,
    lookup,
)
from .protocol import (
    LingXiNode,
    LingXiChannel,
    LingXiMessage,
    XianShu,
    XIAN_SHU_NAMES,
)
from .log_ring import LogRing, LogEntry, MerkleNode
from .name_anchor import name_to_anchor, joint_anchor
from .paibing import diagnose, Health, check_double_code, check_triple_code

__all__ = [
    "BinaryObserver",
    "TernaryObserver",
    "ObserverNetwork",
    "build_from_double",
    "build_from_triple",
    "Process",
    "ProcessState",
    "CoupleOS",
    "MYTHOS",
    "get_full_mythos",
    "lookup",
    "LingXiNode",
    "LingXiChannel",
    "LingXiMessage",
    "XianShu",
    "XIAN_SHU_NAMES",
    "LogRing",
    "MerkleNode",
    "name_to_anchor",
    "joint_anchor",
    "diagnose",
    "Health",
]
