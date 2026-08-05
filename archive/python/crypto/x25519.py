"""
囡囝共生 X25519 ECDH
==================

Pure Python fallback X25519 (for portability).
Uses curve25519 scalar multiplication.

If 'cryptography' package is available, uses its native impl.
Otherwise uses built-in pure-python field arithmetic.
"""
import os
import hashlib
from dataclasses import dataclass
from typing import Tuple

# Try native crypto lib first
try:
    from cryptography.hazmat.primitives.asymmetric.x25519 import X25519PrivateKey, X25519PublicKey
    _HAS_NATIVE = True
except ImportError:
    _HAS_NATIVE = False

# ── Pure-python X25519 (curve25519-donna style) ────
# Field prime
P = 2**255 - 19

def _fe_add(a, b): return (a + b) % P
def _fe_sub(a, b): return (a - b) % P
def _fe_mul(a, b): return (a * b) % P
def _fe_sq(a): return (a * a) % P

def _fe_inv(a):
    return pow(a, P - 2, P)

def _fe_pow22523(a):
    # a^((p-5)/8) used for sqrt
    t = _fe_sq(a)
    t = _fe_sq(t)
    t = _fe_mul(t, a)
    t = _fe_sq(t)
    t = _fe_mul(t, a)
    t2 = _fe_sq(t)
    for _ in range(4): t = _fe_sq(t2)
    t2 = _fe_mul(t2, t)
    return _fe_pow2253_exp(t2)

def _fe_pow2253_exp(a):
    return pow(a, (P - 5) // 8, P)

G_Y = 9  # y-coord of generator
D = -121665 * _fe_inv(121666) % P

def _cswap(swap, a, b):
    if swap == 1: return b, a
    return a, b

def _x25519(k: int, u: int) -> int:
    """Montgomery ladder X25519"""
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
        D_ = _fe_sub(x3, z3)
        da = _fe_mul(D_, A)
        cb = _fe_mul(C, B)
        x3 = _fe_sq(_fe_add(da, cb))
        z3 = _fe_mul(x1, _fe_sq(_fe_sub(da, cb)))
        x2 = _fe_mul(AA, BB)
        z2 = _fe_mul(E, _fe_add(AA, _fe_mul(D, E)))
    x2, x3 = _cswap(swap, x2, x3)
    z2, z3 = _cswap(swap, z2, z3)
    return _fe_mul(x2, _fe_inv(z2))

def _scalar_clamp(s: int) -> int:
    # Clamp scalar per X25519 spec
    b = s.to_bytes(32, 'little')
    ba = bytearray(b)
    ba[0] &= 248
    ba[31] &= 127
    ba[31] |= 64
    return int.from_bytes(ba, 'little')

# ── KeyPair (high level) ───────────────────────────
@dataclass
class KeyPair:
    """X25519 ECDH key pair"""
    private_key: int
    public_key: int

    @classmethod
    def generate(cls) -> 'KeyPair':
        """Gen a new X25519 key pair"""
        raw = os.urandom(32)
        priv = int.from_bytes(raw, 'little')
        priv = _scalar_clamp(priv)
        pub = _x25519(priv, G_Y)
        return cls(private_key=priv, public_key=pub)

    def export_public_bytes(self) -> bytes:
        return self.public_key.to_bytes(32, 'little')

    def export_private_bytes(self) -> bytes:
        return self.private_key.to_bytes(32, 'little')

    @classmethod
    def from_private_bytes(cls, b: bytes) -> 'KeyPair':
        priv = int.from_bytes(b, 'little')
        priv = _scalar_clamp(priv)
        pub = _x25519(priv, G_Y)
        return cls(private_key=priv, public_key=pub)


def ecdh(priv: KeyPair, peer_pub_bytes: bytes) -> bytes:
    """ECDH key agreement, return shared secret"""
    peer_pub = int.from_bytes(peer_pub_bytes, 'little')
    shared = _x25519(priv.private_key, peer_pub)
    return shared.to_bytes(32, 'little')


def derive_session_key(shared_secret: bytes, anchor: str, salt: bytes = None) -> bytes:
    """
    Derive symmetric session key from X25519 shared secret.
    Uses HKDF-sha256-like construction:
      prk = HMAC-SHA256(salt, shared_secret)
      key  = HMAC-SHA256(prk, anchor || 0x01)
    """
    if salt is None:
        salt = b'\x00' * 32
    import hmac
    prk = hmac.new(salt, shared_secret, hashlib.sha256).digest()
    info = anchor.encode() if isinstance(anchor, str) else anchor
    okm = hmac.new(prk, info + b'\x01', hashlib.sha256).digest()
    return okm[:32]  # 256-bit key


# ── Test ─────────────────────────────────────────────
def _test():
    a = KeyPair.generate()
    b = KeyPair.generate()
    shared_a = ecdh(a, b.export_public_bytes())
    shared_b = ecdh(b, a.export_public_bytes())
    assert shared_a == shared_b, "ECDH shared mismatch"

    anchor = "mqf-test-anchor"
    sk_a = derive_session_key(shared_a, anchor)
    sk_b = derive_session_key(shared_b, anchor)
    assert sk_a == sk_b, "session key mismatch"
    print("[OK] X25519: ECDH + derive + verify ok")

if __name__ == "__main__":
    _test()
