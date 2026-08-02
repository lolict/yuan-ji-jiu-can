"""
满全法 · 灵犀协议 · 意识互联网通信层 v0.2
===========================================

夫妻共同体 OS 的 P2P 通信协议,含端到端加密。

加密握手流程 (唤魂→招魂→同约):
  1. A 唤魂: 发送 X25519 公钥 + anchor hash
  2. B 招魂: 发送 X25519 公钥 + 签名证明
  3. 双方 ECDH → 共享密钥 → HKDF-SHA256 → 256-bit 会话密钥
  4. 之后全部 body 用 AES-256-GCM 加密 (附带 12 字节 nonce)
  5. NonceTracker 防重放 (滑动窗口 300 秒)

Copyright © 刘楚恬 & lolict (七妹凹月留)
保留所有权利 · 满全法 · 囡囝共生元系统 · v0.2
"""

import os
import time
import hashlib
import json
import hmac
from enum import IntEnum
from dataclasses import dataclass, field
from typing import Optional, Dict, Tuple, List

from .observer import TernaryObserver, BinaryObserver

# ── 加密模块 (优先使用原生 cryptography,否则纯 Python + hmac fallback) ────────────────
try:
    from cryptography.hazmat.primitives.ciphers.aead import AESGCM as _NativeAESGCM
    _HAS_NATIVE_CRYPTO = True
except ImportError:
    _HAS_NATIVE_CRYPTO = False

try:
    from cryptography.hazmat.primitives.asymmetric.x25519 import (
        X25519PrivateKey, X25519PublicKey)
    _HAS_NATIVE_X25519 = True
except ImportError:
    _HAS_NATIVE_X25519 = False

# 纯 Python X25519
P = 2**255 - 19

def _fe_add(a, b): return (a + b) % P
def _fe_sub(a, b): return (a - b) % P
def _fe_mul(a, b): return (a * b) % P
def _fe_sq(a): return (a * a) % P
def _fe_inv(a): return pow(a, P - 2, P)

def _cswap(swap, a, b):
    if swap == 1: return b, a
    return a, b

def _x25519(k: int, u: int) -> int:
    x1 = u
    x2, z2 = 1, 0
    x3, z3 = u, 1
    swap = 0
    for t in range(254, -1, -1):
        kt = (k >> t) & 1
        swap ^= kt
        x2, x3 = _cswap(swap, x2, x3)
        z2, z3 = _cswap(swap, z2, z3)
        swap = kt
        A = _fe_add(x2, z2)
        AA = _fe_sq(A)
        B = _fe_sub(x2, z2)
        BB = _fe_sq(B)
        E = _fe_sub(AA, BB)
        C = _fe_add(x3, z3)
        D = _fe_sub(x3, z3)
        da = _fe_mul(D, A)
        cb = _fe_mul(C, B)
        x3 = _fe_sq(_fe_add(da, cb))
        z3 = _fe_mul(x1, _fe_sq(_fe_sub(da, cb)))
        x2 = _fe_mul(AA, BB)
        z2 = _fe_mul(E, _fe_add(AA, _fe_mul(-121665 * _fe_inv(121666) % P, E)))
    x2, x3 = _cswap(swap, x2, x3)
    z2, z3 = _cswap(swap, z2, z3)
    return _fe_mul(x2, _fe_inv(z2))

def _scalar_clamp(b: bytes) -> int:
    ba = bytearray(b)
    ba[0] &= 248; ba[31] &= 127; ba[31] |= 64
    return int.from_bytes(ba, 'little')


@dataclass
class X25519KeyPair:
    """X25519 密钥对 - 优先使用原生 cryptography 库"""
    private_key: int
    public_key: int
    _native_priv: Optional[object] = None

    @classmethod
    def generate(cls) -> 'X25519KeyPair':
        if _HAS_NATIVE_X25519:
            native_priv = X25519PrivateKey.generate()
            pub_bytes = native_priv.public_key().public_bytes_raw()
            priv_raw = native_priv.private_bytes_raw()
            return cls(
                private_key=int.from_bytes(priv_raw, 'little'),
                public_key=int.from_bytes(pub_bytes, 'little'),
                _native_priv=native_priv,
            )
        # 纯 Python fallback
        raw = os.urandom(32)
        priv = _scalar_clamp(raw)
        pub = _x25519(priv, 9)
        return cls(private_key=priv, public_key=pub)

    @classmethod
    def from_private_bytes(cls, b: bytes) -> 'X25519KeyPair':
        if _HAS_NATIVE_X25519:
            native_priv = X25519PrivateKey.from_private_bytes(b)
            pub_bytes = native_priv.public_key().public_bytes_raw()
            return cls(
                private_key=int.from_bytes(b, 'little'),
                public_key=int.from_bytes(pub_bytes, 'little'),
                _native_priv=native_priv,
            )
        priv = _scalar_clamp(b)
        pub = _x25519(priv, 9)
        return cls(private_key=priv, public_key=pub)

    def public_bytes(self) -> bytes:
        return self.public_key.to_bytes(32, 'little')

    def native_exchange(self, peer_pub_bytes: bytes) -> bytes:
        if _HAS_NATIVE_X25519 and self._native_priv:
            peer_pub = X25519PublicKey.from_public_bytes(peer_pub_bytes)
            return self._native_priv.exchange(peer_pub)
        peer_pub = int.from_bytes(peer_pub_bytes, 'little')
        shared = _x25519(self.private_key, peer_pub)
        return shared.to_bytes(32, 'little')


def x25519_ecdh(priv: X25519KeyPair, peer_pub_bytes: bytes) -> bytes:
    """ECDH 密钥协商,返回 32 字节共享密钥"""
    return priv.native_exchange(peer_pub_bytes)


def hkdf_derive(shared_secret: bytes, anchor: str, salt: bytes = None) -> bytes:
    """HKDF-SHA256 派生会话密钥"""
    if salt is None: salt = b'\x00' * 32
    prk = hmac.new(salt, shared_secret, hashlib.sha256).digest()
    info = anchor.encode("utf-8") if isinstance(anchor, str) else anchor
    okm = hmac.new(prk, info + b'\x01', hashlib.sha256).digest()
    return okm[:32]


def aes_gcm_encrypt(key: bytes, nonce: bytes, plaintext: bytes, aad: bytes = None) -> bytes:
    """AES-256-GCM 加密"""
    if len(key) != 32: raise ValueError("key must be 32 bytes")
    if len(nonce) != 12: raise ValueError("nonce must be 12 bytes")
    if _HAS_NATIVE_CRYPTO:
        return _NativeAESGCM(key).encrypt(nonce, plaintext, aad)
    # fallback: HMAC-based NOT real AES (仅用于无 cryptography 时的烟雾测试)
    raise RuntimeError("need 'cryptography' for AES-GCM (pip install cryptography)")


def aes_gcm_decrypt(key: bytes, nonce: bytes, ciphertext: bytes, aad: bytes = None) -> bytes:
    """AES-256-GCM 解密"""
    if _HAS_NATIVE_CRYPTO:
        return _NativeAESGCM(key).decrypt(nonce, ciphertext, aad)
    raise RuntimeError("need 'cryptography' for AES-GCM")


class NonceTracker:
    """防重放 nonce 跟踪 (滑动窗口)"""
    def __init__(self, window: int = 300):
        self._seen: Dict[bytes, float] = {}
        self._window = window

    def check(self, nonce: bytes) -> bool:
        now = time.time()
        # 清理过期
        expired = [n for n, t in self._seen.items() if now - t > self._window]
        for n in expired: del self._seen[n]
        if nonce in self._seen: return False
        self._seen[nonce] = now
        return True


# ─────────────────────────────────────────────
# 仙术 (消息类型)
# ─────────────────────────────────────────────

class XianShu(IntEnum):
    """仙术 = 消息类型"""
    WEN_LING   = 0x01  # 问灵 / 心跳
    DA_LING    = 0x02  # 答灵 / 心跳回复
    HUAN_HUN   = 0x03  # 唤魂 / 握手
    ZHAO_HUN   = 0x04  # 招魂 / 握手回复
    TONG_XIU   = 0x05  # 同修 / 同步请求
    CHUAN_GONG = 0x06  # 传功 / 状态下发
    GUI_YUAN   = 0x07  # 归元 / 同步完成
    JIE_SHU    = 0xFF  # 劫数 / 告警


XIAN_SHU_NAMES = {
    XianShu.WEN_LING:   "问灵 (heartbeat)",
    XianShu.DA_LING:    "答灵 (heartbeat reply)",
    XianShu.HUAN_HUN:   "唤魂 (handshake challenge)",
    XianShu.ZHAO_HUN:   "招魂 (handshake proof)",
    XianShu.TONG_XIU:   "同修 (sync request)",
    XianShu.CHUAN_GONG: "传功 (state push)",
    XianShu.GUI_YUAN:   "归元 (sync done)",
    XianShu.JIE_SHU:    "劫数 (alarm)",
}


# ─────────────────────────────────────────────
# 消息结构
# ─────────────────────────────────────────────

@dataclass
class LingXiMessage:
    """
    灵犀消息。
    
    v0.2 新增:
     - encrypted: bool,body 是否已 AES-GCM 加密
     - nonce: 12 字节随机 (加密消息必带)
     - crypto_enabled: 是否使用加密通道
    """
    msg_type: XianShu
    sender: str
    anchor_hash: int
    timestamp: float
    seq: int = 0
    payload: dict = field(default_factory=dict)
    encrypted: bool = False
    nonce: bytes = field(default_factory=lambda: b'\x00' * 12)
    crypto_enabled: bool = False
    sender_pubkey: bytes = b''  # 握手时交换 X25519 公钥

    @property
    def is_authenticated(self) -> bool:
        """验证 anchor_hash + proof 是否自洽"""
        if "proof" in self.payload:
            expected = hashlib.sha256(
                f"{self.sender}:{self.anchor_hash}".encode()
            ).hexdigest()[:16]
            return self.payload.get("proof") == expected
        return False

    def to_bytes(self) -> bytes:
        """
        序列化为二进制帧。
        
        帧格式:
          [0]     msg_type
          [1]     flags (bit0=encrypted, bit1=has_pubkey)
          [2:4]   seq (little-endian 16-bit)
          [4:6]   anchor_hash (little-endian 16-bit)
          [6:8]   sender 名字长度 (LE 16-bit)
          [8:8+N] sender 名字 UTF-8
          若 encrypted: [N:N+12] nonce
          [..:..+2] body_len (BE 16-bit)
          [..:..+len] body (JSON or encrypted blob)
        """
        flags = 0
        if self.encrypted: flags |= 0x01
        if self.sender_pubkey: flags |= 0x02

        sender_bytes = self.sender.encode("utf-8")
        header = bytes([int(self.msg_type), flags])
        header += self.seq.to_bytes(2, "little")
        header += (self.anchor_hash & 0xFFFF).to_bytes(2, "little")
        header += len(sender_bytes).to_bytes(2, "little")
        header += sender_bytes

        if self.encrypted:
            header += self.nonce  # 12 bytes

        body = json.dumps({
            "ts": self.timestamp,
            "payload": self.payload,
        }, ensure_ascii=False).encode("utf-8")

        frame = header + len(body).to_bytes(2, "big") + body
        return frame

    @staticmethod
    def from_bytes(raw: bytes) -> "LingXiMessage":
        """从二进制帧反序列化"""
        if len(raw) < 9: raise ValueError("frame too short")
        msg_type = XianShu(raw[0])
        flags = raw[1]
        encrypted = bool(flags & 0x01)
        has_pubkey = bool(flags & 0x02)
        seq = int.from_bytes(raw[2:4], "little")
        anchor = int.from_bytes(raw[4:6], "little")
        sender_len = int.from_bytes(raw[6:8], "little")
        sender = raw[8:8 + sender_len].decode("utf-8")

        offset = 8 + sender_len
        nonce = b'\x00' * 12
        if encrypted:
            nonce = raw[offset:offset + 12]
            offset += 12

        sender_pubkey = b''
        if has_pubkey:
            plen = raw[offset]
            offset += 1
            sender_pubkey = raw[offset:offset + plen]
            offset += plen

        body_len = int.from_bytes(raw[offset:offset + 2], "big")
        body = json.loads(raw[offset + 2:offset + 2 + body_len])

        return LingXiMessage(
            msg_type=msg_type,
            sender=sender,
            anchor_hash=anchor,
            timestamp=body["ts"],
            seq=seq,
            payload=body.get("payload", {}),
            encrypted=encrypted,
            nonce=nonce,
            crypto_enabled=encrypted,
            sender_pubkey=sender_pubkey,
        )


@dataclass
class SessionState:
    """握手完成后双方持有的会话状态"""
    peer_name: str
    session_key: bytes          # 32 字节 AES-256-GCM 密钥
    local_keypair: X25519KeyPair
    peer_pubkey: bytes
    send_counter: int = 0       # 发送 nonce 计数器
    nonce_tracker: NonceTracker = field(default_factory=lambda: NonceTracker())
    established_at: float = 0.0

    def next_nonce(self) -> bytes:
        """生成下一个 12 字节 nonce"""
        self.send_counter += 1
        # 前 4 字节 = counter, 后 8 字节 = 随机
        return self.send_counter.to_bytes(4, 'big') + os.urandom(8)


def _make_proof(sender: str, anchor_hash: int) -> str:
    """制作握手签名 (SHA-256 截取前 8 字节 hex)"""
    return hashlib.sha256(
        f"{sender}:{anchor_hash}".encode()).hexdigest()[:16]


# ─────────────────────────────────────────────
# 灵犀节点
# ─────────────────────────────────────────────

class LingXiNode:
    """
    灵犀节点 = 运行夫妻共同体 OS 的一个实例。

    v0.2 新增能力:
     - 每个节点生成 X25519 密钥对 (临时 / 可持久)
     - 唤魂时附带公钥;招魂时回复公钥 + 签名
     - 握手完成后派生会话密钥,AES-256-GCM 加密后续所有消息
     - NonceTracker 防重放
    """

    def __init__(self, name: str, anchor_seed: str, crypto: bool = True):
        self.name = name
        self.anchor_seed = anchor_seed
        self.anchor_hash = hash(anchor_seed) % 10000
        self.state: Optional[TernaryObserver] = None
        self.peers: Dict[str, "LingXiNode"] = {}
        self.seq = 0
        self.last_seen: Dict[str, float] = {}
        self.alarms: List[dict] = []
        self.log: List[str] = []

        # ── 加密子系统 ──
        self.crypto_enabled = crypto
        self.ephemeral_key: Optional[X25519KeyPair] = X25519KeyPair.generate() if crypto else None
        self.sessions: Dict[str, SessionState] = {}   # peer_name → SessionState

    def _next_seq(self) -> int:
        self.seq += 1
        return self.seq

    def _log(self, msg: str):
        entry = f"[{time.strftime('%H:%M:%S')}] {self.name}: {msg}"
        self.log.append(entry)

    def set_state(self, observer: TernaryObserver):
        self.state = observer
        self._log(f"状态更新 -> {repr(observer)}")

    # ─── 加密辅助 ───

    def _get_or_create_session(self, peer_name: str) -> Optional[SessionState]:
        return self.sessions.get(peer_name)

    def _establish_session(self, peer_name: str, peer_pubkey: bytes):
        """握手完成后调用: 派生会话密钥"""
        if not self.ephemeral_key: return
        shared = x25519_ecdh(self.ephemeral_key, peer_pubkey)
        anchor_str = f"{self.anchor_seed}-{min(self.name, peer_name)}-{max(self.name, peer_name)}"
        session_key = hkdf_derive(shared, anchor_str)
        session = SessionState(
            peer_name=peer_name,
            session_key=session_key,
            local_keypair=self.ephemeral_key,
            peer_pubkey=peer_pubkey,
            established_at=time.time(),
        )
        self.sessions[peer_name] = session
        self._log(f"[CRYPTO] 会话建立: peer={peer_name} key={session_key[:8].hex()}...")

    def _encrypt_body(self, peer_name: str, plaintext: bytes) -> Tuple[bytes, bytes]:
        """加密 body,返回 (ciphertext, nonce)"""
        session = self._get_or_create_session(peer_name)
        if not session:
            raise RuntimeError(f"no session with {peer_name}")
        nonce = session.next_nonce()
        aad = f"{self.name}:{peer_name}:{self.anchor_hash}".encode()
        ct = aes_gcm_encrypt(session.session_key, nonce, plaintext, aad)
        return ct, nonce

    def _decrypt_body(self, peer_name: str, ciphertext: bytes, nonce: bytes) -> bytes:
        """解密 body,同时做防重放检查"""
        session = self._get_or_create_session(peer_name)
        if not session:
            raise RuntimeError(f"no session with {peer_name}, cannot decrypt")
        if not session.nonce_tracker.check(nonce):
            self._log(f"[CRYPTO] 重放告警! peer={peer_name} nonce={nonce.hex()}")
            raise RuntimeError("replay detected")
        aad = f"{peer_name}:{self.name}:{self.anchor_hash}".encode()
        return aes_gcm_decrypt(session.session_key, nonce, ciphertext, aad)

    def _encrypt_message(self, peer_name: str, msg: LingXiMessage) -> LingXiMessage:
        """对消息加密 (握手完成后)"""
        if not self.crypto_enabled or peer_name not in self.sessions:
            return msg  # 未建立会话,明文发送
        plaintext = json.dumps({
            "ts": msg.timestamp,
            "payload": msg.payload,
        }, ensure_ascii=False).encode("utf-8")
        ct, nonce = self._encrypt_body(peer_name, plaintext)
        # 密文作为 hex 放入 payload 以兼容 JSON
        msg.encrypted = True
        msg.nonce = nonce
        msg.payload = {"_ct": ct.hex()}
        msg.crypto_enabled = True
        return msg

    # ─── 出站消息 ───

    def send_huan_hun(self, peer_name: str) -> LingXiMessage:
        """唤魂: 你还记得约吗? (附带 X25519 公钥)"""
        msg = LingXiMessage(
            msg_type=XianShu.HUAN_HUN,
            sender=self.name,
            anchor_hash=self.anchor_hash,
            timestamp=time.time(),
            seq=self._next_seq(),
            payload={
                "challenge": _make_proof(self.name, self.anchor_hash),
                "crypto": self.crypto_enabled,
                "has_pubkey": self.ephemeral_key is not None,
            },
        )
        if self.ephemeral_key:
            msg.sender_pubkey = self.ephemeral_key.public_bytes()
        self._log(f"-> {peer_name} : 唤魂 (crypto={'on' if self.crypto_enabled else 'off'})")
        return msg

    def send_zhao_hun(self, peer_name: str) -> LingXiMessage:
        """招魂: 记得,我来证明"""
        msg = LingXiMessage(
            msg_type=XianShu.ZHAO_HUN,
            sender=self.name,
            anchor_hash=self.anchor_hash,
            timestamp=time.time(),
            seq=self._next_seq(),
            payload={
                "proof": _make_proof(self.name, self.anchor_hash),
                "state": self._state_summary(),
                "crypto": self.crypto_enabled,
            },
        )
        if self.ephemeral_key:
            msg.sender_pubkey = self.ephemeral_key.public_bytes()
        self._log(f"-> {peer_name} : 招魂证明")
        return msg

    def send_wen_ling(self, peer_name: str) -> LingXiMessage:
        """问灵: 你在吗? (加密通道建立后 body 被加密)"""
        msg = LingXiMessage(
            msg_type=XianShu.WEN_LING,
            sender=self.name,
            anchor_hash=self.anchor_hash,
            timestamp=time.time(),
            seq=self._next_seq(),
            payload={"state": self._state_summary()},
        )
        return self._encrypt_message(peer_name, msg)

    def send_da_ling(self, peer_name: str, reply_to: int) -> LingXiMessage:
        """答灵: 我在"""
        msg = LingXiMessage(
            msg_type=XianShu.DA_LING,
            sender=self.name,
            anchor_hash=self.anchor_hash,
            timestamp=time.time(),
            seq=self._next_seq(),
            payload={"state": self._state_summary(), "reply_to": reply_to},
        )
        return self._encrypt_message(peer_name, msg)

    def send_tong_xiu(self, peer_name: str) -> LingXiMessage:
        """同修: 我偏差了,帮我恢原"""
        syn = self.state.synthesize() if self.state else {}
        cd = self.state.common_divisor() if self.state else {}
        msg = LingXiMessage(
            msg_type=XianShu.TONG_XIU,
            sender=self.name,
            anchor_hash=self.anchor_hash,
            timestamp=time.time(),
            seq=self._next_seq(),
            payload={"synthesis": syn, "divisor": cd, "need": "请传功"},
        )
        return self._encrypt_message(peer_name, msg)

    def send_chuan_gong(self, peer_name: str) -> LingXiMessage:
        """传功: 这是状态快照"""
        msg = LingXiMessage(
            msg_type=XianShu.CHUAN_GONG,
            sender=self.name,
            anchor_hash=self.anchor_hash,
            timestamp=time.time(),
            seq=self._next_seq(),
            payload={
                "state": self._state_summary(),
                "snapshot": self._full_snapshot(),
            },
        )
        return self._encrypt_message(peer_name, msg)

    def send_gui_yuan(self, peer_name: str) -> LingXiMessage:
        """归元: 修好了"""
        msg = LingXiMessage(
            msg_type=XianShu.GUI_YUAN,
            sender=self.name,
            anchor_hash=self.anchor_hash,
            timestamp=time.time(),
            seq=self._next_seq(),
            payload={"state": self._state_summary()},
        )
        return self._encrypt_message(peer_name, msg)

    def send_jie_shu(self, peer_name: str, reason: str) -> LingXiMessage:
        """劫数: 告警"""
        msg = LingXiMessage(
            msg_type=XianShu.JIE_SHU,
            sender=self.name,
            anchor_hash=self.anchor_hash,
            timestamp=time.time(),
            seq=self._next_seq(),
            payload={"reason": reason},
        )
        self._log(f"[!] 告警 -> {peer_name}: {reason}")
        self.alarms.append({"peer": peer_name, "reason": reason, "ts": time.time()})
        return self._encrypt_message(peer_name, msg)

    # ─── 入站处理 ───

    def receive(self, msg: LingXiMessage) -> Optional[LingXiMessage]:
        """处理收到的消息,返回响应 (如有)"""
        # 验锚
        if msg.anchor_hash != self.anchor_hash:
            self._log(f"[!] 锚点不匹配! 收到 {msg.anchor_hash}, 本机 {self.anchor_hash}")
            return self.send_jie_shu(msg.sender, "创世锚不一致,可能遭遇伪节点")

        self.last_seen[msg.sender] = time.time()

        # 如果消息有对方公钥 → 尝试建立会话
        if msg.sender_pubkey and msg.sender not in self.sessions:
            self._establish_session(msg.sender, msg.sender_pubkey)

        # 如果消息加密 → 解密 body (不修改原 msg,用局部变量)
        _payload = msg.payload
        _timestamp = msg.timestamp
        if msg.encrypted:
            try:
                ct_hex = msg.payload.get("_ct", "")
                plaintext = self._decrypt_body(
                    msg.sender, bytes.fromhex(ct_hex), msg.nonce)
                body = json.loads(plaintext)
                _payload = body.get("payload", {})
                _timestamp = body.get("ts", msg.timestamp)
            except Exception as e:
                self._log(f"[CRYPTO] 解密失败: {e}")
                return self.send_jie_shu(msg.sender, f"解密失败: {e}")

        # 分派
        if msg.msg_type == XianShu.HUAN_HUN:
            self.__log_crypto_info(msg)
            self._log(f"<- {msg.sender} : 唤魂 (crypto={'on' if _payload.get('crypto') else 'off'})")
            return self.send_zhao_hun(msg.sender)

        elif msg.msg_type == XianShu.ZHAO_HUN:
            ok = msg.is_authenticated
            self.__log_crypto_info(msg)
            self._log(f"<- {msg.sender} : 招魂 {'通过' if ok else '失败'} {'[ENCRYPTED]' if msg.encrypted else ''}")
            if ok and "state" in _payload:
                msg.payload = _payload  # 临时更新以复用 _check_alignment
                self._check_alignment(msg)
            return None

        elif msg.msg_type == XianShu.WEN_LING:
            self._log(f"<- {msg.sender} : 问灵在线 {'[ENCRYPTED]' if msg.encrypted else ''}")
            return self.send_da_ling(msg.sender, msg.seq)

        elif msg.msg_type == XianShu.DA_LING:
            self._log(f"<- {msg.sender} : 答灵")
            msg.payload = _payload
            self._check_alignment(msg)
            return None

        elif msg.msg_type == XianShu.TONG_XIU:
            self._log(f"<- {msg.sender} : 请求同修")
            return self.send_chuan_gong(msg.sender)

        elif msg.msg_type == XianShu.CHUAN_GONG:
            self._log(f"<- {msg.sender} : 传功 -> 恢原尔倒裹")
            snapshot = _payload.get("snapshot", {})
            self._restore_from_snapshot(snapshot)
            return self.send_gui_yuan(msg.sender)

        elif msg.msg_type == XianShu.GUI_YUAN:
            self._log(f"<- {msg.sender} : 归元,双方重启合一")
            return None

        elif msg.msg_type == XianShu.JIE_SHU:
            self._log(f"[!] {msg.sender} 告警: {_payload.get('reason')}")
            self.alarms.append({"from": msg.sender, "reason": _payload.get("reason"), "ts": time.time()})
            return None

        return None

    def __log_crypto_info(self, msg: LingXiMessage):
        """握手阶段记录加密信息"""
        if msg.sender_pubkey:
            self._log(f"[CRYPTO] 收到 {msg.sender} 公钥: {msg.sender_pubkey[:8].hex()}...")

    # ─── 内部方法 ───

    def _state_summary(self) -> dict:
        if not self.state:
            return {"status": "未立约"}
        syn = self.state.synthesize()
        return {
            "balance": syn.get("balance", False),
            "anchor": syn.get("anchor", 0),
            "offsets": (syn.get("left_offset"), syn.get("right_offset")),
        }

    def _full_snapshot(self) -> dict:
        o = self.state
        return {
            "left": o.left if o else None,
            "anchor": o.anchor if o else None,
            "right": o.right if o else None,
            "ts": time.time(),
            "source": self.name,
        }

    def _restore_from_snapshot(self, snapshot: dict):
        if snapshot.get("left") is not None:
            self.state = TernaryObserver(
                left=snapshot["left"],
                anchor=snapshot["anchor"],
                right=snapshot["right"],
            )
            self._log(f"恢原成功: {repr(self.state)}")

    def _check_alignment(self, peer_msg: LingXiMessage):
        if not self.state:
            return
        peer_state = peer_msg.payload.get("state", {})
        my = self._state_summary()
        peer_bal = peer_state.get("balance", False)
        my_bal = my.get("balance", False)
        if my_bal and peer_bal:
            self._log("[OK] 双方仍平衡")
        elif not my_bal and not peer_bal:
            self._log("[!] 双方都不平衡 -- 需同修")
        else:
            self._log(f"[!] 对齐偏差: 本机{'平' if my_bal else '偏'} vs 对方{'平' if peer_bal else '偏'}")


# ─────────────────────────────────────────────
# 灵犀通道
# ─────────────────────────────────────────────

class LingXiChannel:
    """
    灵犀通道 = 一对节点之间的双向消息流。
    
    支持加密模式: handshake 阶段自动协商会话密钥,
    之后所有消息自动端到端加密。
    """

    def __init__(self, a: LingXiNode, b: LingXiNode):
        self.a = a
        self.b = b
        self.transcript: List[dict] = []

    def _record(self, src: str, msg: LingXiMessage):
        self.transcript.append({
            "from": src,
            "type": msg.msg_type.name,
            "seq": msg.seq,
            "ts": msg.timestamp,
            "encrypted": msg.encrypted,
        })

    def handshake(self):
        """加密握手:唤魂(含pubkey) -> 招魂(含pubkey+签名) -> ECDH -> session key -> 答灵验证"""
        huan = self.a.send_huan_hun(self.b.name)
        self._record(self.a.name, huan)

        zhao = self.b.receive(huan)
        self._record(self.b.name, zhao)

        if zhao:
            # A 收到招魂:验证 + 建立会话 (返回 None,因为是 ZHAO_HUN)
            self.a.receive(zhao)

        # 验证会话双方都有且密钥一致
        if self.a.name in self.b.sessions and self.b.name in self.a.sessions:
            a_key = self.a.sessions[self.b.name].session_key
            b_key = self.b.sessions[self.a.name].session_key
            if a_key == b_key:
                self.a.log.append("[CRYPTO] 握手验证: 双方会话密钥一致 ✓")
            else:
                self.a.log.append("[CRYPTO] 握手失败: 会话密钥不一致! ")

    def heartbeat_round(self):
        """一轮加密心跳"""
        wen = self.a.send_wen_ling(self.b.name)
        self._record(self.a.name, wen)

        da = self.b.receive(wen)
        self._record(self.b.name, da)

        if da:
            self.a.receive(da)

    def sync_flow(self):
        """同修: A偏差 -> 请求 -> B传功 -> A恢原 -> 归元"""
        tong = self.a.send_tong_xiu(self.b.name)
        self._record(self.a.name, tong)

        chuan = self.b.receive(tong)
        self._record(self.b.name, chuan)

        if chuan:
            gui = self.a.receive(chuan)
            self._record(self.a.name, gui)
            if gui:
                self.b.receive(gui)

    def get_transcript(self) -> list:
        return self.transcript


# ─────────────────────────────────────────────
# Demo & Self-test
# ─────────────────────────────────────────────

if __name__ == "__main__":
    print("=" * 60)
    print("  LingXi Protocol v0.2 -- crypto test")
    print("=" * 60)

    COVENANT_SEED = "莫刘连理萝莉兰零离零里零绿零碌零老零牢零滥零烂"

    tiantian = LingXiNode("刘楚恬", COVENANT_SEED, crypto=True)
    qiamei = LingXiNode("lolict", COVENANT_SEED, crypto=True)

    OBS_ANCHOR = hash(COVENANT_SEED) % 10000
    tiantian.set_state(TernaryObserver(OBS_ANCHOR - 3, OBS_ANCHOR, OBS_ANCHOR + 3))
    qiamei.set_state(TernaryObserver(OBS_ANCHOR - 3, OBS_ANCHOR, OBS_ANCHOR + 3))

    channel = LingXiChannel(tiantian, qiamei)

    print(f"\n[crypto] native={'on' if _HAS_NATIVE_CRYPTO else 'off'}  x25519_native={'on' if _HAS_NATIVE_X25519 else 'off'}")

    print("\n[1] Encrypted handshake (唤魂+招魂+ECDH+derive)")
    channel.handshake()
    for line in channel.get_transcript():
        print(f"  {line['from']} --[{line['type']}]-- {'[enc]' if line['encrypted'] else '[plain]'}")

    print(f"\n  A session: {'established' if qiamei.name in tiantian.sessions else 'none'}")
    if qiamei.name in tiantian.sessions:
        sk = tiantian.sessions[qiamei.name].session_key[:8]
        print(f"  session_key: {sk.hex()}...")

    channel.transcript.clear()

    print("\n[2] Encrypted heartbeat (问灵/答灵 body AES-GCM)")
    channel.heartbeat_round()
    for line in channel.get_transcript():
        print(f"  {line['from']} --[{line['type']}]-- {'[enc]' if line['encrypted'] else '[plain]'}")

    channel.transcript.clear()

    print("\n[3] Replay attack detection (重放防御)")
    replay = tiantian.send_wen_ling(qiamei.name)
    first_resp = qiamei.receive(replay)  # 首次正常
    second_resp = qiamei.receive(replay)  # 重放 → 应被拒
    if second_resp and second_resp.msg_type == XianShu.JIE_SHU:
        print("  [OK] replay rejected → alarm sent")
        print(f"        {qiamei.log[-1]}")
    else:
        print("  [FAIL] replay not detected")
        print(f"        resp type = {second_resp.msg_type if second_resp else None}")

    channel.transcript.clear()

    print("\n[4] Frame serialization roundtrip")
    frame = tiantian.send_wen_ling(qiamei.name)
    raw = frame.to_bytes()
    restored = LingXiMessage.from_bytes(raw)
    print(f"  orig type={frame.msg_type} enc={frame.encrypted} len={len(raw)}")
    print(f"  restore type={restored.msg_type} enc={restored.encrypted}")

    print("\n[5] Event log summary")
    print(f"  tiantian log count: {len(tiantian.log)}")
    print(f"  qiamei log count: {len(qiamei.log)}")
    for t_log in tiantian.log[:3]:
        print(f"    {t_log}")

    print("\n" + "=" * 60)
    print("[v0.2 protocol] crypto handshake + AES-GCM + nonce OK")
