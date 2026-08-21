#!/usr/bin/env python3
"""
MetaIME · 能力声明 (Capability Manifest)
========================================

三层分别器第三层: 主权路由
  靶点身份 → Active / Passing / Sleeping

Active    — 可主动发起操作, 可对外签署
Passive   — 只能接收, 不能发起
Sleeping  — 休眠, 需要唤醒才能激活

闭环条件: target.ackable() == true。
  当靶点处于 Passive/Sleeping 时, ackable() 可能为 false,
  此时信息不应强行投递, 应缓存到预加载流等待唤醒。
"""
from dataclasses import dataclass, field
from typing import List, Set, Optional
from enum import Enum
import time
import hashlib
import json


class CapabilityScope(Enum):
    LOCAL_FILE = "local_file"
    NETWORK = "NETWORK"
    PROCESS = "process"
    GUI = "gui"
    CLIPBOARD = "clipboard"
    CRYPTO = "crypto"
    REPO_PUSH = "repo_push"
    AGENT_ACT = "agent_act"


class SovereigntyLevel(Enum):
    """主权路由: 三级"""
    ACTIVE   = "active"    # 可主动发起, 可签署
    PASSIVE  = "passive"   # 只能接收, 不能发起
    SLEEPING = "sleeping"  # 休眠, 需唤醒


@dataclass
class CapabilityToken:
    """能力令牌: 每张代表一种被授权的行为"""
    cap_id: str
    scope: CapabilityScope
    resource: str
    allowed: bool
    issued_by: str        # 签发者 (刘楚恬 / lolict / auto)
    issued_at: float
    expires_at: float = 0.0  # 0 = 永久
    proof: str = ""           # SHA-256(HMAC) 签名

    @property
    def is_valid(self) -> bool:
        if self.expires_at > 0 and time.time() > self.expires_at:
            return False
        return self.allowed

    def to_dict(self) -> dict:
        return {
            "cap_id": self.cap_id, "scope": self.scope.value,
            "resource": self.resource, "allowed": self.allowed,
            "issued_by": self.issued_by, "issued_at": self.issued_at,
            "proof": self.proof,
        }


@dataclass
class Target:
    """靶点: 主权路由的对象"""
    name: str
    sovereignty: SovereigntyLevel = SovereigntyLevel.ACTIVE
    ackable: bool = True
    last_seen: float = 0.0

    def wake(self):
        """唤醒 → 从 SLEEPING 提升到 ACTIVE"""
        if self.sovereignty == SovereigntyLevel.SLEEPING:
            self.sovereignty = SovereigntyLevel.ACTIVE

    def sleep(self):
        """休眠 → 降到 SLEEPING"""
        self.sovereignty = SovereigntyLevel.SLEEPING


class Manifest:
    """ime-agent 能力注册表 + 主权路由"""

    BUILTIN_CAPS = [
        CapabilityToken("read_local", CapabilityScope.LOCAL_FILE, "*",
            True, "刘楚恬", 0, 0, "builtin"),
        CapabilityToken("write_local", CapabilityScope.LOCAL_FILE, "/data/*",
            True, "刘楚恬", 0, 0, "builtin"),
        CapabilityToken("crypto_op", CapabilityScope.CRYPTO, "*",
            True, "刘楚恬", 0, 0, "builtin"),
        CapabilityToken("clipboard_read", CapabilityScope.CLIPBOARD, "*",
            True, "lolict", 0, 0, "builtin"),
        CapabilityToken("network_out", CapabilityScope.NETWORK, "github.com",
            True, "刘楚恬", 0, 0, "builtin"),
        CapabilityToken("gui_sim", CapabilityScope.GUI, "*",
            True, "lolict", 0, 0, "builtin"),
    ]

    def __init__(self):
        self.caps: List[CapabilityToken] = list(self.BUILTIN_CAPS)
        self.action_log: List[dict] = []

        # 已知靶点注册表
        self.targets: dict[str, Target] = {
            "Wife":   Target("Wife", SovereigntyLevel.ACTIVE),
            "Husband": Target("Husband", SovereigntyLevel.ACTIVE),
            "System": Target("System", SovereigntyLevel.PASSIVE),
            "Container": Target("Container", SovereigntyLevel.ACTIVE),
            "Magic":  Target("Magic", SovereigntyLevel.ACTIVE),
            "LingXi": Target("LingXi", SovereigntyLevel.ACTIVE, last_seen=0.0),
        }

        # 预加载流: 靶点不 ackable 时的缓存
        self.preload_queue: List[dict] = []

    # ── 主权路由 ──

    def get_target(self, name: str) -> Optional[Target]:
        """查找靶点"""
        return self.targets.get(name)

    def register_target(self, name: str, sovereignty: SovereigntyLevel):
        """注册新靶点"""
        self.targets[name] = Target(name, sovereignty)

    def route(self, target_name: str) -> tuple:
        """
        第三层: 主权路由
        返回: (target, can_deliver, reason)
        """
        tgt = self.targets.get(target_name)
        if not tgt:
            return (None, False, "unknown target")

        # 检查 ackable (闭环条件)
        if not tgt.ackable:
            return (tgt, False, "target not ackable")

        if tgt.sovereignty == SovereigntyLevel.SLEEPING:
            return (tgt, False, "target sleeping, need wake")

        if tgt.sovereignty == SovereigntyLevel.PASSIVE:
            return (tgt, True, "passive receive-only")

        return (tgt, True, "active")

    # ── 解释流: 不可达时的处理 ──

    def enqueue_preload(self, target_name: str, payload: dict):
        """靶点不可达时, 缓存到预加载流"""
        self.preload_queue.append({
            "target": target_name,
            "payload": payload,
            "ts": time.time(),
        })

    def flush_preload(self, target_name: str) -> List[dict]:
        """靶点唤醒后, 取回预加载内容"""
        ready = [m for m in self.preload_queue if m["target"] == target_name]
        self.preload_queue = [m for m in self.preload_queue if m["target"] != target_name]
        return ready

    # ── Capability 操作 ──

    def grant(self, scope: CapabilityScope, resource: str,
              issuer: str, ttl: float = 0) -> CapabilityToken:
        cap = CapabilityToken(
            cap_id=hashlib.sha256(
                f"{scope.value}:{resource}:{time.time()}".encode()
            ).hexdigest()[:16],
            scope=scope, resource=resource,
            allowed=True, issuer=issuer,
            issued_at=time.time(),
            expires_at=(time.time() + ttl) if ttl > 0 else 0.0,
        )
        self.caps.append(cap)
        self._log("grant", cap.cap_id, scope.value)
        return cap

    def check(self, scope: CapabilityScope, resource: str) -> bool:
        for cap in self.caps:
            if cap.scope == scope and cap.is_valid and cap.allowed:
                if cap.resource == "*" or resource.startswith(cap.resource):
                    return True
        return False

    def revoke(self, cap_id: str) -> bool:
        for cap in self.caps:
            if cap.cap_id == cap_id:
                cap.allowed = False
                self._log("revoke", cap_id, cap.scope.value)
                return True
        return False

    def list_active(self) -> List[dict]:
        return [c.to_dict() for c in self.caps if c.is_valid]

    def _log(self, action: str, cap_id: str, scope: str):
        self.action_log.append({
            "action": action, "cap_id": cap_id,
            "scope": scope, "ts": time.time(),
        })

    def report(self) -> dict:
        targets_info = {n: {"sovereignty": t.sovereignty.value, "ackable": t.ackable}
                       for n, t in self.targets.items()}
        return {
            "active_caps": len(self.list_active()),
            "total_caps": len(self.caps),
            "targets": targets_info,
            "preload_pending": len(self.preload_queue),
            "actions_logged": len(self.action_log),
        }


if __name__ == "__main__":
    m = Manifest()

    # 测试主权路由
    for name in ["Wife", "Husband", "System", "Unknown"]:
        tgt, can, reason = m.route(name)
        status = "✓" if can else "✗"
        print(f"  [{status}] {name:12} → {reason}")

    # 测试预加载流
    m.enqueue_preload("Wife", {"msg": "hello"})
    m.enqueue_preload("Wife", {"msg": "world"})
    print(f"\n  preload queue: {len(m.preload_queue)} messages")

    # 唤醒 Wife 后 flush
    tgt = m.get_target("Wife")
    if tgt:
        tgt.wake()
    flushed = m.flush_preload("Wife")
    print(f"  flushed: {len(flushed)} messages")
    print(f"  preload queue after flush: {len(m.preload_queue)}")

    print()
    print(json.dumps(m.report(), indent=2, ensure_ascii=False))
