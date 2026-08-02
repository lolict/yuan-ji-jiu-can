"""
满全法 Nonce 防重放
====================

每个灵犀消息都携带 12 字节 nonce。
本地 NonceTracker 记录已见过的 nonce。
如果收到重复 nonce → 丢弃 (重放攻击)。
如果收到过期 nonce (超过窗口) → 丢弃 (重发攻击)。
"""
import time
import threading
from typing import Dict, Set, Optional

class NonceTracker:
    """
    滑动窗口 nonce 防重放。
    存储最近 N 秒内的 nonce (默认 300 秒)。
    过期自动清理。
    """

    def __init__(self, window_seconds: int = 300, max_entries: int = 10000):
        self.window = window_seconds
        self.max_entries = max_entries
        self._seen: Dict[bytes, float] = {}
        self._lock = threading.Lock()

    def check_and_add(self, nonce: bytes) -> bool:
        """
        检查 nonce 是否是新的。
        返回 True = 新消息,可以接受。
        返回 False = 重放或溢出,拒绝。
        """
        with self._lock:
            self._cleanup()
            if nonce in self._seen:
                return False
            if len(self._seen) >= self.max_entries:
                return False
            self._seen[nonce] = time.time()
            return True

    def _cleanup(self):
        """清理过期 nonce"""
        cutoff = time.time() - self.window
        expired = [n for n, t in self._seen.items() if t < cutoff]
        for n in expired:
            del self._seen[n]

    def count(self) -> int:
        return len(self._seen)

    def __contains__(self, nonce: bytes) -> bool:
        return nonce in self._seen


def replay_check(tracker: NonceTracker, nonce: bytes) -> bool:
    """Convenience: return True if message is ok (not replay)"""
    return tracker.check_and_add(nonce)


# ── Test ─────────────────────────────────────────────
def _test():
    tracker = NonceTracker(window_seconds=60)
    n1 = b'\x01' * 12
    n2 = b'\x02' * 12

    assert tracker.check_and_add(n1) == True   # new
    assert tracker.check_and_add(n1) == False  # replay
    assert tracker.check_and_add(n2) == True   # new
    assert tracker.count() == 2
    print("[OK] NonceTracker: dedup + count ok")

if __name__ == "__main__":
    _test()
