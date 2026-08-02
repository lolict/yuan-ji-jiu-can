#!/usr/bin/env python3
"""
满全法 · 囡囝共生元 OS · 命令行入口
=====================================

用法:
  python3 cli.py dragon <拼音> <声调>
  python3 cli.py paibing <编码>
  python3 cli.py paibing2 <2字编码>
  python3 cli.py paibing3 <3字编码>
  python3 cli.py myth <神仙名>
  python3 cli.py myth list
  python3 cli.py observe binary <起点> <终点>
  python3 cli.py observe ternary <左> <锚> <右>
  python3 cli.py covenant <誓约内容> [签名A] [签名B]
  python3 cli.py protocol handshake [seed]
  python3 cli.py protocol sync [seed]
  python3 cli.py crypto handshake
  python3 cli.py crypto seal <text>
  python3 cli.py log append <kind> <payload>
  python3 cli.py log verify
  python3 cli.py name <中文名>
  python3 cli.py pair <名A> <名B>
  python3 cli.py fsm table
  python3 cli.py fsm test
  python3 cli.py minlian list
  python3 cli.py minlian lookup <字>
  python3 cli.py nanjian <四态>
  python3 cli.py report
  python3 cli.py restore list
  python3 cli.py serve [port]

例子:
  python3 cli.py dragon chuan 2
  python3 cli.py paibing vy
  python3 cli.py paibing2 vy
  python3 cli.py paibing3 lnm
  python3 cli.py myth 太乙真人
  python3 cli.py myth list
  python3 cli.py observe binary 0 7
  python3 cli.py protocol handshake
  python3 cli.py crypto handshake
  python3 cli.py crypto seal "hello"
  python3 cli.py log append test "hello world"
  python3 cli.py log verify
  python3 cli.py name 刘楚恬
  python3 cli.py pair 刘楚恬 lolict
  python3 cli.py fsm table
  python3 cli.py fsm test
  python3 cli.py nanjian nan_jian
  python3 cli.py restore list
"""

import sys
import os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(
    0,
    os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "hunyuan-universe"),
)

from kernel import (
    BinaryObserver, TernaryObserver, CoupleOS,
    MYTHOS, get_full_mythos, lookup,
    LingXiNode, LingXiChannel,
)
from encoding import Dragon, PathologyDiagnosis


# ── LogRing 持久化路径 ──────────────────────────────
_LOG_RING_PATH = os.path.join(
    os.path.dirname(os.path.abspath(__file__)), ".mqf-log-ring.json"
)


def _get_log_ring():
    """获取或创建持久化 LogRing"""
    from kernel.log_ring import LogRing
    if os.path.exists(_LOG_RING_PATH):
        import json
        with open(_LOG_RING_PATH, encoding="utf-8") as f:
            data = json.load(f)
        ring = LogRing.from_snapshot(data)
        return ring
    ring = LogRing(anchor="mqf-cli-log-ring")
    return ring


def _save_log_ring(ring):
    """持久化 LogRing 到磁盘"""
    import json
    snap = ring.snapshot()
    with open(_LOG_RING_PATH, "w", encoding="utf-8") as f:
        json.dump(snap, f, ensure_ascii=False, indent=2)


def main():
    args = sys.argv[1:]
    if not args:
        print(__doc__)
        return
    cmd = args[0]
    rest = args[1:]

    if cmd == "dragon" and len(rest) >= 2:
        py, tn = rest[0], rest[1]
        from encoding.compress import compress
        chain = compress(py, tn)
        if chain:
            d = Dragon(chain=chain)
            import json
            print(json.dumps(d.summon(), ensure_ascii=False, indent=2))
        else:
            print(f"[error] cannot parse: {py}{tn}")

    elif cmd == "paibing" and rest:
        code = rest[0]
        if len(code) == 2:
            import json
            print(json.dumps(PathologyDiagnosis.diagnose_double(code), ensure_ascii=False, indent=2))
        elif len(code) == 3:
            import json
            print(json.dumps(PathologyDiagnosis.diagnose_triple(code), ensure_ascii=False, indent=2))
        else:
            print("[error] code must be 2 or 3 chars")

    elif cmd == "paibing2" and rest:
        code = rest[0]
        from kernel.paibing import check_double_code
        import json
        result = check_double_code([code])
        print(json.dumps(result, ensure_ascii=False, indent=2))

    elif cmd == "paibing3" and rest:
        code = rest[0]
        from kernel.paibing import check_triple_code
        import json
        result = check_triple_code([code])
        print(json.dumps(result, ensure_ascii=False, indent=2))

    elif cmd == "myth":
        if not rest or rest[0] != "name":
            if rest and rest[0] == "list":
                data = get_full_mythos()
                for k, v in data["entities"].items():
                    print(f"  {k:20} {v.get('name',''):8} {v.get('role','')}")
            elif rest:
                name = rest[0]
                r = lookup(name)
                import json
                print(json.dumps(r, ensure_ascii=False, indent=2))
            else:
                data = get_full_mythos()
                for k, v in data["entities"].items():
                    print(f"  {k:20} {v.get('name',''):8} {v.get('role','')}")

    elif cmd == "observe" and len(rest) >= 4:
        mode, vals = rest[0], [float(x) for x in rest[1:]]
        import json
        if mode == "binary" and len(vals) >= 2:
            obs = BinaryObserver(start=vals[0], end=vals[1])
            print(json.dumps({
                "repr": repr(obs), "gap": obs.gap, "abs_gap": obs.abs_gap,
                "rate_start": obs.to_rate("start"),
                "rate_end": obs.to_rate("end"),
            }, ensure_ascii=False, indent=2))
        elif mode == "ternary" and len(vals) >= 3:
            obs = TernaryObserver(left=vals[0], anchor=vals[1], right=vals[2])
            print(json.dumps({
                "repr": repr(obs),
                "to_anchor": obs.to_anchor,
                "distance_via_anchor": obs.distance_via_anchor,
                "common_divisor": obs.common_divisor(),
                "synthesis": obs.synthesize(),
            }, ensure_ascii=False, indent=2))

    elif cmd == "covenant" and rest:
        content = rest[0]
        sa = rest[1] if len(rest) > 1 else "刘楚恬"
        sb = rest[2] if len(rest) > 2 else "lolict"
        os_inst = CoupleOS()
        p = os_inst.立约(content, sa, sb)
        import json
        print(json.dumps({
            "state": p.state.value,
            "observer": repr(p.observer),
            "sync": p.sync(),
        }, ensure_ascii=False, indent=2))

    elif cmd == "protocol" and len(rest) >= 2:
        mode, seed = rest[0], rest[1] if len(rest) > 1 else "hunyuan"
        a = LingXiNode("刘楚恬", seed)
        b = LingXiNode("lolict", seed)
        anchor = a.anchor_hash
        a.set_state(TernaryObserver(anchor-3, anchor, anchor+3))
        b.set_state(TernaryObserver(anchor-3, anchor, anchor+3))
        channel = LingXiChannel(a, b)
        if mode == "handshake":
            channel.handshake()
            channel.heartbeat_round()
        elif mode == "sync":
            a.set_state(TernaryObserver(anchor-10, anchor, anchor+5))
            channel.handshake()
            channel.sync_flow()
        else:
            print(f"[error] unknown mode: {mode}")
            return
        print("== transcript ==")
        for t in channel.get_transcript():
            print(f"  {t['from']:8} --[{t['type']}]-->")
        print()
        print("== 刘楚恬 log ==")
        for l in a.log[-4:]:
            print(f"  {l}")
        print()
        print("== lolict log ==")
        for l in b.log[-4:]:
            print(f"  {l}")

    elif cmd == "crypto" and rest:
        mode = rest[0]
        if mode == "handshake":
            from kernel.protocol import X25519KeyPair, x25519_ecdh, hkdf_derive
            import hashlib
            a_kp = X25519KeyPair.generate()
            b_kp = X25519KeyPair.generate()
            a_pub = a_kp.public_bytes().hex()
            b_pub = b_kp.public_bytes().hex()
            shared_a = x25519_ecdh(a_kp, b_kp.public_bytes())
            shared_b = x25519_ecdh(b_kp, a_kp.public_bytes())
            assert shared_a == shared_b, "ECDH mismatch"
            anchor_str = f"mqf-handshake-{a_pub[:16]}-{b_pub[:16]}"
            session_key = hkdf_derive(shared_a, anchor_str)
            import json
            print(json.dumps({
                "A_pub": a_pub,
                "B_pub": b_pub,
                "shared_key_fingerprint": hashlib.sha256(shared_a).hexdigest()[:16],
                "session_key": session_key.hex(),
            }, ensure_ascii=False, indent=2))
        elif mode == "seal" and len(rest) >= 2:
            from kernel.protocol import aes_gcm_encrypt
            import os as _os
            plaintext = rest[1].encode("utf-8")
            key = _os.urandom(32)
            nonce = _os.urandom(12)
            ciphertext = aes_gcm_encrypt(key, nonce, plaintext)
            import json
            print(json.dumps({
                "key_hex": key.hex(),
                "nonce_hex": nonce.hex(),
                "ciphertext_hex": ciphertext.hex(),
            }, ensure_ascii=False, indent=2))
        else:
            print("[error] crypto usage: crypto handshake | crypto seal <text>")

    elif cmd == "log" and rest:
        mode = rest[0]
        if mode == "append" and len(rest) >= 3:
            kind = rest[1]
            payload_str = rest[2]
            ring = _get_log_ring()
            try:
                import json as _json
                payload = _json.loads(payload_str)
            except Exception:
                payload = {"text": payload_str}
            entry = ring.append(kind, payload)
            _save_log_ring(ring)
            import json
            print(json.dumps({
                "seq": entry.seq,
                "kind": entry.kind,
                "hash": entry.hash[:16] + "...",
                "total_entries": len(ring.entries),
            }, ensure_ascii=False, indent=2))
        elif mode == "verify":
            ring = _get_log_ring()
            valid = ring.verify()
            root = ring.merkle_root()
            import json
            print(json.dumps({
                "entries": len(ring.entries),
                "merkle_root": root,
                "valid": valid,
            }, ensure_ascii=False, indent=2))
        else:
            print("[error] log usage: log append <kind> <payload> | log verify")

    elif cmd == "name" and rest:
        name = rest[0]
        from kernel.name_anchor import name_to_anchor
        anchor = name_to_anchor(name)
        import json
        print(json.dumps({
            "name": name,
            "anchor": anchor,
            "anchor_hex": anchor.replace("mqf_name_", ""),
        }, ensure_ascii=False, indent=2))

    elif cmd == "pair" and len(rest) >= 2:
        name_a, name_b = rest[0], rest[1]
        from kernel.name_anchor import name_to_anchor, joint_anchor
        a_anchor = name_to_anchor(name_a)
        b_anchor = name_to_anchor(name_b)
        joint = joint_anchor(name_a, name_b)
        import json
        print(json.dumps({
            "name_a": name_a,
            "anchor_a": a_anchor,
            "name_b": name_b,
            "anchor_b": b_anchor,
            "joint_anchor": joint,
        }, ensure_ascii=False, indent=2))

    elif cmd == "fsm" and rest:
        mode = rest[0]
        if mode == "table":
            from kernel.quaternity_fsm import QuaternityState, TRANSITIONS
            import json
            states = [{"name": s.name, "value": s.value} for s in QuaternityState]
            transitions = [
                {"from": k[0].value, "event": k[1], "to": v.value}
                for k, v in TRANSITIONS.items()
            ]
            print(json.dumps({
                "states": states,
                "transitions": transitions,
            }, ensure_ascii=False, indent=2))
        elif mode == "test":
            from kernel.quaternity_fsm import QuaternityFSM, QuaternityState
            import json
            fsm = QuaternityFSM()
            events = ["wife_deepen", "husband_take", "deepen", "wife_push"]
            results = []
            for ev in events:
                ok = fsm.trigger(ev)
                results.append({
                    "event": ev,
                    "success": ok,
                    "state": fsm.current.value,
                })
            print(json.dumps({
                "initial_state": QuaternityState.NAN_JIAN.value,
                "events": results,
                "final_state": fsm.current.value,
                "diagnosis": fsm.diagnose(),
            }, ensure_ascii=False, indent=2))
        else:
            print("[error] fsm usage: fsm table | fsm test")

    elif cmd == "nanjian" and rest:
        state = rest[0]
        r = MYTHOS.get("nan_jian_topology", {}).get("quaternity", {}).get(state, {})
        if r:
            import json
            print(json.dumps(r, ensure_ascii=False, indent=2))
        else:
            print(f"[error] state must be nan_nan / nan_jian / jian_jian / jian_nan")

    elif cmd == "report":
        os_inst = CoupleOS()
        os_inst.立约("莫刘连理萝莉兰...", "刘楚恬", "lolict")
        os_inst.立约("民联集合", "liu", "lol")
        import json
        print(json.dumps(os_inst.运行(), ensure_ascii=False, indent=2))

    elif cmd == "restore" and rest:
        if rest[0] == "list":
            import subprocess
            result = subprocess.run(
                [sys.executable, os.path.join(os.path.dirname(os.path.abspath(__file__)), "restore.py"), "--list"],
                capture_output=True, text=True
            )
            print(result.stdout, end="")
            if result.returncode != 0:
                print(result.stderr, end="")
        else:
            print("[error] restore usage: restore list")

    elif cmd == "serve":
        port = int(rest[0]) if rest else 8080
        from web.server import PORT
        import importlib
        m = importlib.import_module("web.server")
        m.PORT = port
        print(f"\n满全法 · 囡囝共生元 OS")
        print(f"  监听: http://localhost:{port}")
        server = __import__("http.server").server.HTTPServer(("0.0.0.0", port), m.Handler)
        try:
            server.serve_forever()
        except KeyboardInterrupt:
            server.shutdown()

    elif cmd == "minlian":
        if not rest or rest[0] == "list":
            data = _load_minlian()
            print(f"民联词典 ({len(data)} 条)")
            for item in data[:50]:
                print(f"  {item['word']:8} = {item['source']:8} {item['note']}")
            if len(data) > 50:
                print(f"  ... 共 {len(data)} 条")
        elif rest[0] == "lookup" and len(rest) >= 2:
            q = rest[1]
            data = _load_minlian()
            hits = [x for x in data if q in x["word"] or q in x["source"] or q in x.get("note", "")]
            for h in hits[:20]:
                print(f"  {h['word']:8} = {h['source']:8} {h['note']}")
            if not hits:
                print("  未找到")

    else:
        print(f"[error] unknown command: {cmd}")
        print(__doc__)


def _load_minlian():
    import json
    data_path = os.path.join(
        os.path.dirname(os.path.abspath(__file__)),
        "data",
        "minlian.json",
    )
    if not os.path.exists(data_path):
        return _builtin_minlian()
    with open(data_path, encoding="utf-8") as f:
        return json.load(f)


def _builtin_minlian():
    """内置兜底民联数据 -- 详细版见 data/minlian.json"""
    return [
        {"word": "攀", "source": "潘", "note": "攀龙附凤 → 潘江陆海, 基6.4"},
        {"word": "廉", "source": "连", "note": "清廉 → 连理"},
        {"word": "莫", "source": "刘", "note": "莫刘连理"},
        {"word": "刘", "source": "留", "note": "刘 = 留 (谐音)"},
        {"word": "安", "source": "桉", "note": "平安 → 桉树长青"},
        {"word": "花", "source": "华", "note": "花容月貌 → 华章"},
        {"word": "诗", "source": "思", "note": "诗意 → 思念"},
        {"word": "情", "source": "晴", "note": "情深 → 晴朗"},
        {"word": "枫", "source": "风", "note": "枫叶 → 风骨"},
        {"word": "蝶", "source": "叠", "note": "蝶舞 → 叠韵"},
    ]


if __name__ == "__main__":
    main()
