#!/usr/bin/env python3
"""
MetaIME · 能力声明 (Capability Manifest)
========================================

声明 ime-agent 能做什么 + 不能做什么。
Capability-Based Security:内生权限,不靠操作系统。
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


@dataclass
class CapabilityToken:
    """能力令牌:每张代表一种被授权的行为"""
    cap_id: str
    scope: CapabilityScope
    resource: str
    allowed: bool
    issued_by: str        # 签发者 (刘楚恬 / lolict / auto)
    issued_at: float
    expires_at: float = 0.0  # 0 = 永久
    proof: str = ""       # SHA-256(HMAC) 签名

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


class Manifest:
    """ime-agent 能力注册表"""

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
        """检查是否持有某能力"""
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
        return {
            "active_caps": len(self.list_active()),
            "total_caps": len(self.caps),
            "actions_logged": len(self.action_log),
        }


if __name__ == "__main__":
    m = Manifest()
    print("[manifest] active caps:", m.list_active())
    print("[manifest] check network_out github.com:",
        m.check(CapabilityScope.NETWORK, "github.com"))
    print("[manifest] check network_out evil.com:",
        m.check(CapabilityScope.NETWORK, "evil.com"))
    print("[manifest]:", json.dumps(m.report(), indent=2, ensure_ascii=False))
