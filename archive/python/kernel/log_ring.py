"""
满全法 · 关系日志环 (Log Ring)
================================

append-only 关系状态日志,配合 Merkle 树历史锚定。
 -- 任何状态变化都留痕
 -- 不可篡改 (每一条 hash 串联)
 -- 可审计 (可证明某时某刻有某事)
 -- 为结构性安全感提供数据基础
"""
import json
import hashlib
import time
from typing import List, Dict, Any, Optional


class LogEntry:
    def __init__(self, seq: int, kind: str, payload: dict, prev_hash: str, ts: float = None):
        self.seq = seq
        self.kind = kind
        self.payload = payload
        self.prev_hash = prev_hash
        self.ts = ts or time.time()
        self.hash = self._compute_hash()

    def _compute_hash(self) -> str:
        material = f"{self.seq}|{self.kind}|{json.dumps(self.payload, sort_keys=True)}|{self.prev_hash}|{self.ts}"
        return hashlib.sha256(material.encode()).hexdigest()

    def to_dict(self) -> dict:
        return {
            "seq": self.seq,
            "kind": self.kind,
            "payload": self.payload,
            "ts": self.ts,
            "prev_hash": self.prev_hash,
            "hash": self.hash,
        }


class MerkleNode:
    def __init__(self, hash_val: str, left=None, right=None):
        self.hash = hash_val
        self.left = left
        self.right = right

    @staticmethod
    def build(leaves: List[str]) -> Optional['MerkleNode']:
        if not leaves:
            return None
        if len(leaves) == 1:
            return MerkleNode(leaves[0])
        nodes = [MerkleNode(l) for l in leaves]
        while len(nodes) > 1:
            next_level = []
            for i in range(0, len(nodes), 2):
                left = nodes[i]
                right = nodes[i + 1] if i + 1 < len(nodes) else nodes[i]
                combined = hashlib.sha256((left.hash + right.hash).encode()).hexdigest()
                next_level.append(MerkleNode(combined, left, right))
            nodes = next_level
        return nodes[0]


class LogRing:
    """
    关系日志环
    anchor: 创世锚
    """

    def __init__(self, anchor: str):
        self.anchor = anchor
        self.entries: List[LogEntry] = []
        self._genesis_hash = hashlib.sha256(anchor.encode()).hexdigest()

    def append(self, kind: str, payload: dict = None) -> LogEntry:
        seq = len(self.entries)
        prev_hash = self.entries[-1].hash if self.entries else self._genesis_hash
        entry = LogEntry(seq, kind, payload or {}, prev_hash)
        self.entries.append(entry)
        return entry

    def last_hash(self) -> str:
        if not self.entries:
            return self._genesis_hash
        return self.entries[-1].hash

    def verify(self) -> bool:
        prev = self._genesis_hash
        for e in self.entries:
            if e.prev_hash != prev:
                return False
            if e.hash != e._compute_hash():
                return False
            prev = e.hash
        return True

    def merkle_root(self) -> str:
        hashes = [e.hash for e in self.entries]
        if not hashes:
            return self._genesis_hash
        root = MerkleNode.build(hashes)
        return root.hash if root else self._genesis_hash

    def find_by_kind(self, kind: str) -> List[LogEntry]:
        return [e for e in self.entries if e.kind == kind]

    def timeline(self) -> List[dict]:
        return [e.to_dict() for e in self.entries]

    def snapshot(self) -> dict:
        return {
            "anchor": self.anchor,
            "entry_count": len(self.entries),
            "merkle_root": self.merkle_root(),
            "last_hash": self.last_hash(),
            "entries": [e.to_dict() for e in self.entries],
        }

    @classmethod
    def from_snapshot(cls, snapshot: dict) -> 'LogRing':
        ring = cls(snapshot["anchor"])
        for e_data in snapshot.get("entries", []):
            entry = LogEntry(
                seq=e_data["seq"],
                kind=e_data["kind"],
                payload=e_data["payload"],
                prev_hash=e_data["prev_hash"],
                ts=e_data["ts"],
            )
            ring.entries.append(entry)
        return ring


def _test():
    log = LogRing(anchor="mqf-test-anchor")
    log.append("he-yi-tick", {"balance": 0})
    log.append("he-yi-tick", {"balance": 1})
    log.append("wear-shell", {"shell": "linux_systemd"})
    log.append("jieshu-alarm", {"severity": 2, "reason": "anchor_mismatch"})
    assert log.verify(), "log ring verify failed"
    assert len(log.find_by_kind("he-yi-tick")) == 2
    assert log.merkle_root() is not None
    snap = log.snapshot()
    ring2 = LogRing.from_snapshot(snap)
    assert ring2.verify(), "restored ring verify failed"
    print(f"[OK] log_ring: {len(log.entries)} entries, merkle_root={log.merkle_root()[:16]}...")


if __name__ == "__main__":
    _test()
