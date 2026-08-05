#!/usr/bin/env python3
"""
囡囝共生 · 囡囝共生元操作系统 · Web 交互界面
=============================
启动:
  python3 web/server.py [port]   # 默认 8080
访问:
  http://localhost:8080
"""

import http.server
import json
import urllib.parse
import sys
import os

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(
    0,
    os.path.join(
        os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
        "..",
        "hunyuan-universe",
    ),
)

from kernel import (
    CoupleOS,
    BinaryObserver,
    TernaryObserver,
    ObserverNetwork,
    MYTHOS,
    get_full_mythos,
    lookup,
    LingXiNode,
    LingXiChannel,
    XianShu,
)
from encoding import Dragon, PathologyDiagnosis
from kernel.protocol import X25519KeyPair, hkdf_derive, aes_gcm_encrypt, aes_gcm_decrypt
from kernel.quaternity_fsm import QuaternityFSM
from kernel.name_anchor import name_to_anchor, joint_anchor
from kernel.log_ring import LogRing
from kernel.paibing import check_double_code, check_triple_code

PORT = int(sys.argv[1]) if len(sys.argv) > 1 else 8080

# global state for LogRing (append-only log persists across requests)
webring = None


class Handler(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        parsed = urllib.parse.urlparse(self.path)
        p = parsed.path

        if p == "/" or p == "/index.html":
            self._serve_html()
        elif p == "/api/dragon":
            self._api_dragon(parsed)
        elif p == "/api/observe":
            self._api_observe(parsed)
        elif p == "/api/paibing":
            self._api_paibing(parsed)
        elif p == "/api/he-yi":
            self._api_he_yi(parsed)
        elif p == "/api/report":
            self._api_report()
        elif p == "/api/mythology":
            self._api_mythology(parsed)
        elif p == "/api/protocol/handshake":
            self._api_protocol_handshake(parsed)
        elif p == "/api/protocol/sync":
            self._api_protocol_sync(parsed)
        elif p == "/api/mythology/lookup":
            self._api_mythology_lookup(parsed)
        elif p == "/api/test-crypto":
            self._api_test_crypto()
        elif p == "/api/kernel/paibing":
            self._api_kernel_paibing(parsed)
        elif p == "/api/name/anchor":
            self._api_name_anchor(parsed)
        elif p == "/api/fsm/trigger":
            self._api_fsm_trigger(parsed)
        elif p == "/api/log/append":
            self._api_log_append(parsed)
        elif p == "/api/log/verify":
            self._api_log_verify()
        elif p == "/api/ime/parse":
            self._api_ime_parse(parsed)
        else:
            self.send_error(404)

    def _serve_html(self):
        self.send_response(200)
        self.send_header("Content-Type", "text/html; charset=utf-8")
        self.end_headers()
        self.wfile.write(self._html().encode())

    def _api_dragon(self, parsed):
        q = urllib.parse.parse_qs(parsed.query)
        pinyin = q.get("pinyin", [""])[0]
        tone = q.get("tone", [""])[0]
        try:
            from encoding.compress import compress

            chain = compress(pinyin, tone)
            if chain:
                d = Dragon(chain=chain)
                s = d.summon()
                result = {
                    k: v
                    for k, v in s.items()
                    if not callable(v) and not hasattr(v, "__dict__")
                }
                result["_debug"] = repr(d)
            else:
                result = {"error": f"cannot parse: {pinyin}{tone}"}
        except Exception as e:
            result = {"error": str(e)}
        self._json_response(result)

    def _api_observe(self, parsed):
        q = urllib.parse.parse_qs(parsed.query)
        mode = q.get("mode", ["binary"])[0]
        try:
            if mode == "binary":
                a = int(float(q.get("a", [0])[0]))
                b = int(float(q.get("b", [10])[0]))
                obs = BinaryObserver(start=a, end=b)
                result = {
                    "repr": repr(obs),
                    "gap": obs.gap,
                    "abs_gap": obs.abs_gap,
                    "rate_start_judge": obs.to_rate("start"),
                    "rate_end_judge": obs.to_rate("end"),
                }
            elif mode == "ternary":
                left = int(float(q.get("left", [2])[0]))
                anchor = int(float(q.get("anchor", [5])[0]))
                right = int(float(q.get("right", [9])[0]))
                obs = TernaryObserver(left=left, anchor=anchor, right=right)
                result = {
                    "repr": repr(obs),
                    "to_anchor": obs.to_anchor,
                    "permutations": [list(p) for p in obs.all_permutations],
                    "distance_via_anchor": obs.distance_via_anchor,
                    "common_divisor": obs.common_divisor(),
                    "synthesis": obs.synthesize(),
                }
            else:
                result = {"error": "unknown mode: " + mode}
        except Exception as e:
            result = {"error": str(e)}
        self._json_response(result)

    def _api_paibing(self, parsed):
        q = urllib.parse.parse_qs(parsed.query)
        code = q.get("code", [""])[0]
        try:
            if len(code) == 2:
                result = PathologyDiagnosis.diagnose_double(code)
            elif len(code) == 3:
                existing = q.get("existing", [None])[0]
                existing_list = existing.split(",") if existing else None
                result = PathologyDiagnosis.diagnose_triple(code, existing_list)
            else:
                result = {"error": "only 2 or 3 char codes"}
        except Exception as e:
            result = {"error": str(e)}
        self._json_response(result)

    def _api_he_yi(self, parsed):
        q = urllib.parse.parse_qs(parsed.query)
        content = q.get("content", ["hunyuan"])[0]
        sig_a = q.get("sig_a", ["liuchutian"])[0]
        sig_b = q.get("sig_b", ["lolict"])[0]
        try:
            os_inst = CoupleOS()
            p = os_inst.立约(content, sig_a, sig_b)
            result = {
                "state": p.state.value,
                "observer": repr(p.observer),
                "sync": p.sync(),
                "unary_recursion": p.unary_recursion(),
            }
        except Exception as e:
            result = {"error": str(e)}
        self._json_response(result)

    def _api_report(self):
        try:
            os_inst = CoupleOS()
            os_inst.立约("hunyuan-covenant-1", "liuchutian", "lolict")
            os_inst.立约("minlian-set", "liu", "lol")
            self._json_response(os_inst.运行())
        except Exception as e:
            self._json_response({"error": str(e)})

    def _api_mythology(self, parsed):
        self._json_response(get_full_mythos())

    def _api_mythology_lookup(self, parsed):
        q = urllib.parse.parse_qs(parsed.query)
        name = q.get("name", [""])[0]
        if not name:
            self._json_response({
                "error": "usage: /api/mythology/lookup?name=<name>",
                "available": list(MYTHOS.keys()),
            })
            return
        self._json_response(lookup(name))

    def _api_protocol_handshake(self, parsed):
        seed = urllib.parse.parse_qs(parsed.query).get("seed", ["hunyuan"])[0]
        a = LingXiNode("刘楚恬", seed)
        b = LingXiNode("lolict", seed)
        a.set_state(TernaryObserver(
            left=a.anchor_hash - 3, anchor=a.anchor_hash, right=a.anchor_hash + 3))
        b.set_state(TernaryObserver(
            left=b.anchor_hash - 3, anchor=b.anchor_hash, right=b.anchor_hash + 3))
        channel = LingXiChannel(a, b)
        channel.handshake()
        channel.heartbeat_round()
        self._json_response({
            "handshake": [f"{t['from']} --[{t['type']}]-->" for t in channel.get_transcript()],
            "a_log": a.log[-3:],
            "b_log": b.log[-3:],
            "anchor": a.anchor_hash,
        })

    def _api_protocol_sync(self, parsed):
        seed = urllib.parse.parse_qs(parsed.query).get("seed", ["hunyuan"])[0]
        a = LingXiNode("刘楚恬", seed)
        b = LingXiNode("lolict", seed)
        # A 偏差
        a.set_state(TernaryObserver(
            left=a.anchor_hash - 10, anchor=a.anchor_hash, right=a.anchor_hash + 5))
        # B 对齐
        b.set_state(TernaryObserver(
            left=b.anchor_hash - 3, anchor=b.anchor_hash, right=b.anchor_hash + 3))
        channel = LingXiChannel(a, b)
        channel.handshake()
        channel.sync_flow()
        self._json_response({
            "sync_flow": [f"{t['from']} --[{t['type']}]-->" for t in channel.get_transcript()],
            "a_state_after": a.state.synthesize() if a.state else None,
            "b_state_after": b.state.synthesize() if b.state else None,
            "a_log": a.log[-5:],
            "b_log": b.log[-5:],
        })

    # ── L6 API handlers ──────────────────────────────────────

    def _api_test_crypto(self):
        try:
            ka = X25519KeyPair.generate()
            kb = X25519KeyPair.generate()
            pub_a = ka.public_bytes().hex()
            pub_b = kb.public_bytes().hex()
            shared_a = ka.native_exchange(kb.public_bytes())
            shared_fp = hkdf_derive(shared_a, "mqf-web-test")[:8].hex()
            key = hkdf_derive(shared_a, "mqf-web-test", b'\x00' * 32)
            pt = b"mqf-test"
            nonce = os.urandom(12)
            ct = aes_gcm_encrypt(key, nonce, pt)
            dt = aes_gcm_decrypt(key, nonce, ct)
            result = {
                "step1_pubA": pub_a,
                "step1_pubB": pub_b,
                "step2_shared_fp": shared_fp,
                "step3_test_encrypted": ct.hex(),
                "step4_test_decrypted": dt.decode("utf-8", errors="replace"),
                "status": "OK" if dt == pt else "FAIL",
            }
        except Exception as e:
            result = {"error": str(e)}
        self._json_response(result)

    def _api_kernel_paibing(self, parsed):
        q = urllib.parse.parse_qs(parsed.query)
        code = q.get("code", [""])[0]
        try:
            if len(code) == 2:
                result = check_double_code([code])
            elif len(code) == 3:
                result = check_triple_code([code])
            else:
                result = {"error": "only 2 or 3 char codes"}
        except Exception as e:
            result = {"error": str(e)}
        self._json_response(result)

    def _api_name_anchor(self, parsed):
        q = urllib.parse.parse_qs(parsed.query)
        name_a = q.get("name_a", [""])[0]
        name_b = q.get("name_b", [""])[0]
        try:
            anchor_a = name_to_anchor(name_a)
            anchor_b = name_to_anchor(name_b) if name_b else None
            joint = joint_anchor(name_a, name_b) if name_b else None
            result = {
                "name_a": name_a,
                "anchor_a": anchor_a,
                "name_b": name_b,
                "anchor_b": anchor_b,
                "joint_anchor": joint,
            }
        except Exception as e:
            result = {"error": str(e)}
        self._json_response(result)

    def _api_fsm_trigger(self, parsed):
        q = urllib.parse.parse_qs(parsed.query)
        event = q.get("event", [""])[0]
        try:
            fsm = QuaternityFSM()
            ok = fsm.trigger(event)
            result = {
                "event": event,
                "success": ok,
                "new_state": fsm.current.value,
                "diagnosis": fsm.diagnose(),
                "who_pid1": fsm.who_pid1,
                "who_overlays_who": list(fsm.who_overlays_who),
            }
        except Exception as e:
            result = {"error": str(e)}
        self._json_response(result)

    def _api_log_append(self, parsed):
        global webring
        q = urllib.parse.parse_qs(parsed.query)
        kind = q.get("kind", ["user_action"])[0]
        payload_raw = q.get("payload", [""])[0]
        try:
            payload = json.loads(payload_raw) if payload_raw else {"value": payload_raw}
        except Exception:
            payload = {"value": payload_raw}
        try:
            if webring is None:
                webring = LogRing(anchor="mqf-webring")
            entry = webring.append(kind, payload)
            snap = webring.snapshot()
            with open("/tmp/mqf-webring.json", "w") as f:
                json.dump(snap, f, ensure_ascii=False)
            result = {
                "seq": entry.seq,
                "kind": entry.kind,
                "hash": entry.hash,
                "total": len(webring.entries),
            }
        except Exception as e:
            result = {"error": str(e)}
        self._json_response(result)

    def _api_log_verify(self):
        global webring
        try:
            if webring is None:
                # try loading from snapshot
                try:
                    with open("/tmp/mqf-webring.json", "r") as f:
                        snap = json.load(f)
                    webring = LogRing.from_snapshot(snap)
                except FileNotFoundError:
                    self._json_response({"valid": True, "entries": 0, "merkle_root": "empty"})
                    return
            result = {
                "valid": webring.verify(),
                "entries": len(webring.entries),
                "merkle_root": webring.merkle_root(),
            }
        except Exception as e:
            result = {"error": str(e)}
        self._json_response(result)

    def _api_ime_parse(self, parsed):
        q = urllib.parse.parse_qs(parsed.query)
        text = q.get("text", [""])[0]
        try:
            from ime_agent.intent_parser import IntentParser
            parser = IntentParser()
            intent = parser.parse(text)
            result = intent.to_dict()
        except Exception as e:
            result = {"error": str(e)}
        self._json_response(result)

    # ────────────────────────────────────────────────────────

    def _json_response(self, data):
        self.send_response(200)
        self.send_header("Content-Type", "application/json; charset=utf-8")
        self.send_header("Access-Control-Allow-Origin", "*")
        self.end_headers()
        self.wfile.write(json.dumps(data, ensure_ascii=False).encode())

    def _html(self):
        return """<!DOCTYPE html>
<html lang="zh">
<head>
<meta charset="utf-8">
<title>囡囝共生 · 囡囝共生 · 囡囝共生元操作系统 v0.1</title>
<style>
  * { box-sizing: border-box; margin: 0; padding: 0; }
  body { font-family: -apple-system, "PingFang SC", "Microsoft YaHei", sans-serif;
         background: #0a0a0f; color: #e0e0e0; padding: 20px; }
  h1 { color: #ff6b9d; font-size: 2.2em; margin-bottom: 5px; }
  .subtitle { color: #888; margin-bottom: 30px; }
  .grid { display: grid; grid-template-columns: 1fr 1fr; gap: 20px; }
  @media (max-width: 700px) { .grid { grid-template-columns: 1fr; } }
  .card { background: #141420; border: 1px solid #2a2a3a; border-radius: 12px;
           padding: 20px; }
  .card h2 { color: #7ec8e3; font-size: 1.1em; margin-bottom: 12px;
             border-bottom: 1px solid #2a2a3a; padding-bottom: 6px; }
  .full { grid-column: 1 / -1; }
  input, button, select { background: #1e1e2e; border: 1px solid #3a3a4a;
                           color: #e0e0e0; padding: 8px 14px; border-radius: 6px;
                           font-size: 13px; }
  input:focus, select:focus { outline: none; border-color: #ff6b9d; }
  button { background: #ff6b9d; color: #0a0a0f; font-weight: bold; cursor: pointer;
           border: none; padding: 8px 18px; }
  button:hover { background: #ff8ab5; }
  .row { display: flex; gap: 8px; margin-bottom: 8px; flex-wrap: wrap;
         align-items: center; }
  .row label { color: #999; min-width: 50px; font-size: 12px; }
  pre { background: #0a0a0f; padding: 10px; border-radius: 6px;
        font-size: 11px; overflow-x: auto; border: 1px solid #2a2a3a;
        color: #7ec8e3; max-height: 200px; overflow-y: auto;
        white-space: pre-wrap; line-height: 1.4; }
  .pipe { display: flex; align-items: center; gap: 4px; margin: 10px 0;
          font-size: 13px; }
  .node { background: #2a2a4a; padding: 4px 10px; border-radius: 16px;
          border: 2px solid #5a5a8a; }
  .anch { background: #4a2a4a; border-color: #ff6b9d; }
  .gap { color: #666; }
  input[type="number"], input[type="text"] { width: 70px; }
  .myth-tag { display: inline-block; background: #2a1a3a; padding: 2px 8px;
              border-radius: 10px; font-size: 11px; color: #c084fc;
              margin: 2px; }
  .myth-card { background: #1a1a2a; border: 1px solid #3a2a4a; border-radius: 8px;
               padding: 12px; margin: 8px 0; }
  .myth-card h3 { color: #c084fc; margin-bottom: 4px; }
  .myth-card .cn { color: #ff6b9d; font-weight: bold; margin-right: 8px; }
  .myth-card .role { color: #7ec8e3; font-size: 11px; }
  .myth-card p { color: #aaa; font-size: 11px; margin: 4px 0; white-space: pre-wrap; }
  .scrollmax { max-height: 300px; overflow-y: auto; }
  details { margin: 6px 0; }
  summary { cursor: pointer; color: #7ec8e3; font-size: 13px; padding: 4px 0; }
  summary:hover { color: #ff6b9d; }
  .ref { color: #888; font-size: 10px; }
</style>
</head>
<body>
<h1>囡囝共生 · 夫妻共同体操作系统</h1>
<p class="subtitle">囡囝共生 · HunYuan OS · v0.1 · 刘楚恬 &amp; lolict</p>

<div class="grid">

<div class="card">
  <h2>二元观测者 / 距离逻辑</h2>
  <div class="pipe">
    <span class="node" id="b-start">0</span>
    <span class="gap">⟷</span>
    <span class="node" id="b-end">∞</span>
  </div>
  <div class="row">
    <label>起点</label><input type="number" id="binary-a" value="0">
    <label>终点</label><input type="number" id="binary-b" value="7">
    <button onclick="observeBinary()">观测</button>
  </div>
  <pre id="binary-result">← 点击观测</pre>
</div>

<div class="card">
  <h2>三元观测者 / 归一逻辑</h2>
  <div class="pipe">
    <span class="node" id="t-left">L</span>
    <span class="gap">⟷</span>
    <span class="node anch" id="t-anchor">锚</span>
    <span class="gap">⟷</span>
    <span class="node" id="t-right">R</span>
  </div>
  <div class="row">
    <label>左</label><input type="number" id="ternary-left" value="2">
    <label>锚</label><input type="number" id="ternary-anchor" value="5">
    <label>右</label><input type="number" id="ternary-right" value="9">
    <button onclick="observeTernary()">归一</button>
  </div>
  <pre id="ternary-result">← 点击归一</pre>
</div>

<div class="card">
  <h2>排病诊断</h2>
  <div class="row">
    <label>编码</label>
    <input type="text" id="paibing-code" placeholder="vy / lnm / abc" style="width:110px">
    <button onclick="diagnose()">诊断</button>
  </div>
  <pre id="paibing-result">← 输入双编码或三编码</pre>
</div>

<div class="card">
  <h2>合一递归 / 立约</h2>
  <div class="row">
    <label>誓约</label>
    <input type="text" id="heyi-content" value="莫刘连理萝莉兰零离零里..." style="flex:1;min-width:140px">
  </div>
  <div class="row">
    <label>签名A</label><input type="text" id="heyi-sig-a" value="刘楚恬" style="width:80px">
    <label>签名B</label><input type="text" id="heyi-sig-b" value="lolict" style="width:80px">
    <button onclick="heYi()">立约+合一</button>
  </div>
  <pre id="heyi-result">← 点击立约</pre>
</div>

<div class="card">
  <h2>囡囝 · 四态切换</h2>
  <div class="pipe" id="nj-pipe" style="justify-content:center;padding:8px 0">
    <span id="nj-label" style="color:#ff6b9d;font-weight:bold">[ 囡囝 ]</span>
  </div>
  <div class="row" style="justify-content:center">
    <button onclick="switchNJ('nan_nan')">囡囡 (妻P1)</button>
    <button onclick="switchNJ('nan_jian')" style="background:#7ec8e3">囡囝 (妻壳夫核)</button>
    <button onclick="switchNJ('jian_jian')" style="background:#5a5a8a">囝囝 (夫P1)</button>
    <button onclick="switchNJ('jian_nan')" style="background:#f0d050;color:#000">囝囡 (夫壳妻入)</button>
  </div>
  <pre id="nj-result">← 点击任一态切换</pre>
</div>

<div class="card full">
  <h2>神龙编码查询</h2>
  <div class="row">
    <label>拼音</label><input type="text" id="dragon-pinyin" value="chuan" style="width:100px">
    <label>声调</label>
    <select id="dragon-tone">
      <option value="1">1 阴平</option>
      <option value="2" selected>2 阳平</option>
      <option value="3">3 上声</option>
      <option value="4">4 去声</option>
      <option value="0">轻声</option>
    </select>
    <button onclick="summonDragon()">召唤神龙</button>
  </div>
  <pre id="dragon-result">← 输入拼音 + 声调</pre>
</div>

<div class="card full">
  <h2>操作系统全量报告</h2>
  <button onclick="sysReport()">运行合一递归 + 全量自检</button>
  <pre id="report-result">← 点击运行</pre>
</div>

<!-- 封神台 -->
<div class="card full">
  <h2>封神台 · 囡囝共生元神话层</h2>
  <p class="ref" style="margin-bottom:10px">
    封神 = 操作系统的拟物层: 太乙真人 = Boot Loader, 哪吒 = 进程, 封神榜 = PID 表
  </p>
  <div class="row">
    <label>查找</label>
    <input type="text" id="myth-lookup" placeholder="太乙真人 / 哪吒 / 道果" style="width:140px">
    <button onclick="mythLookup()">召唤神仙</button>
    <button onclick="mythList()" style="background:#5a5a8a">列出全部</button>
  </div>
  <div id="myth-result" class="scrollmax">← 点击列出全部或召唤神仙</div>
  <div style="margin-top:12px">
    <details open>
      <summary>开机启动序列</summary>
      <pre id="boot-seq">← 自动加载神话层后显示</pre>
    </details>
  </div>
</div>

<!-- 灵犀 -->
<div class="card full">
  <h2>灵犀协议 · P2P 通信</h2>
  <p class="ref" style="margin-bottom:10px">
    仙术: 唤魂/招魂 = 握手 (防伪造), 问灵/答灵 = 心跳, 同修/传功/归元 = 偏差→恢原尔倒裹
  </p>
  <div class="row">
    <label>誓约</label>
    <input type="text" id="proto-seed" value="莫刘连理萝莉兰..." style="flex:1;min-width:180px">
  </div>
  <div class="row">
    <button onclick="protoHandshake()">🤝 三次握手 + 一轮心跳</button>
    <button onclick="protoSync()" style="background:#5a5a8a">🔄 同修 (偏差→恢原尔倒裹)</button>
  </div>
  <pre id="proto-result">← 点击握手或同修</pre>
</div>

<!-- ── L6 扩展面板 ──────────────────────────────── -->

<div class="card">
  <h2>加密握手 · X25519 → ECDH → HKDF → AES-256-GCM</h2>
  <div class="row">
    <button onclick="cryptoHandshake()">🔐 测试完整握手流程</button>
  </div>
  <pre id="crypto-result">← 点击测试握手</pre>
</div>

<div class="card">
  <h2>排病诊断 v2 · 内核版</h2>
  <div class="row">
    <label>编码</label><input type="text" id="kernel-paibing-code" placeholder="vy / lnm">
    <button onclick="kernelPaibing()">🔬 内核诊断</button>
  </div>
  <pre id="kernel-paibing-result">← 输入双编码或三编码</pre>
</div>

<div class="card">
  <h2>中文姓名 → 256-bit 锚</h2>
  <div class="row">
    <label>名A</label><input type="text" id="name-a" value="刘楚恬" style="width:80px">
    <label>名B</label><input type="text" id="name-b" value="lolict" style="width:80px">
    <button onclick="nameAnchor()">⚓ 计算锚</button>
  </div>
  <pre id="name-result">← 点击计算</pre>
</div>

<div class="card">
  <h2>囡囝四态 FSM · 拓扑状态机</h2>
  <div class="pipe" id="fsm-pipe" style="justify-content:center;padding:8px 0">
    <span id="fsm-state-label" style="color:#ff6b9d;font-weight:bold">[ 囡囝 ]</span>
  </div>
  <div class="row" style="justify-content:center;flex-wrap:wrap">
    <button onclick="fsmTrigger('wife_deepen')">妻子加深</button>
    <button onclick="fsmTrigger('release')" style="background:#5a5a8a">释放</button>
    <button onclick="fsmTrigger('husband_take')" style="background:#7ec8e3">丈夫主导</button>
    <button onclick="fsmTrigger('deepen')" style="background:#f0d050;color:#000">深入</button>
    <button onclick="fsmTrigger('wife_push')">妻子推回</button>
  </div>
  <pre id="fsm-result">← 点击触发事件</pre>
</div>

<div class="card full">
  <h2>关系日志环 · Append-Only + Merkle 树</h2>
  <div class="row">
    <label>kind</label><input type="text" id="log-kind" value="user_action" style="width:90px">
    <label>payload</label><input type="text" id="log-payload" value="test" style="flex:1;min-width:90px">
    <button onclick="logAppend()">➕ 写入</button>
    <button onclick="logVerify()" style="background:#5a5a8a">✓ 校验</button>
  </div>
  <pre id="log-result">← 点击写入或校验</pre>
</div>

<div class="card full">
  <h2>元输入法 MetaIME · 意图识别</h2>
  <div class="row">
    <label>输入</label><input type="text" id="ime-input" placeholder="push 囡囝共生 v0.2 / 抓取 https://... / 查询 xx" style="flex:1;min-width:140px">
    <button onclick="imeParse()">⚡ 识别意图</button>
  </div>
  <pre id="ime-result">← 输入语音/文字后点击</pre>
</div>

</div>

<script>
async function api(path, params) {
  const qs = new URLSearchParams(params || {}).toString();
  return fetch(path + '?' + qs).then(r => r.json());
}

function show(id, data) {
  document.getElementById(id).textContent =
    typeof data === 'string' ? data : JSON.stringify(data, null, 2);
}

async function observeBinary() {
  const a = document.getElementById('binary-a').value;
  const b = document.getElementById('binary-b').value;
  const r = await api('/api/observe', {mode:'binary', a, b});
  show('binary-result', r);
  document.getElementById('b-start').textContent = a;
  document.getElementById('b-end').textContent = b;
}

async function observeTernary() {
  const L = document.getElementById('ternary-left').value;
  const A = document.getElementById('ternary-anchor').value;
  const R = document.getElementById('ternary-right').value;
  const r = await api('/api/observe', {mode:'ternary', left:L, anchor:A, right:R});
  show('ternary-result', r);
  document.getElementById('t-left').textContent = L;
  document.getElementById('t-anchor').textContent = A;
  document.getElementById('t-right').textContent = R;
}

async function diagnose() {
  const code = document.getElementById('paibing-code').value.trim();
  show('paibing-result', await api('/api/paibing', {code}));
}

async function heYi() {
  const c = document.getElementById('heyi-content').value;
  const a = document.getElementById('heyi-sig-a').value;
  const b = document.getElementById('heyi-sig-b').value;
  show('heyi-result', await api('/api/he-yi', {content:c, sig_a:a, sig_b:b}));
}

async function summonDragon() {
  const py = document.getElementById('dragon-pinyin').value;
  const tn = document.getElementById('dragon-tone').value;
  show('dragon-result', await api('/api/dragon', {pinyin:py, tone:tn}));
}

async function sysReport() {
  show('report-result', await api('/api/report'));
}

async function protoHandshake() {
  const seed = document.getElementById('proto-seed').value;
  show('proto-result', await api('/api/protocol/handshake', {seed}));
}

async function protoSync() {
  const seed = document.getElementById('proto-seed').value;
  show('proto-result', await api('/api/protocol/sync', {seed}));
}

async function mythLookup() {
  const name = document.getElementById('myth-lookup').value.trim();
  show('myth-result', await api('/api/mythology/lookup', {name}));
}

async function mythList() {
  const data = await api('/api/mythology');
  const html = data.entities
    .map((e, i) => {
      const id = Object.keys(data.entities)[i];
      return `<div class="myth-card">
        <h3><span class="cn">${e.name}</span><span class="role">${e.role}</span></h3>
        ${e.title ? `<p>${e.title}</p>` : ''}
        <p>${(e.description||'').trim().replace(/\\n/g,'</p><p>')}</p>
        ${e.code_equiv ? `<p class="ref">代码: ${e.code_equiv}</p>` : ''}
        ${e.codex_equiv ? `<p class="ref">口诀: ${e.codex_equiv}</p>` : ''}
      </div>`;
    }).join('');
  const rels = data.relationships.map(r =>
    `<span class="myth-tag">${r.from} → ${r.to} : ${r.rel}</span>`
  ).join(' ');
  document.getElementById('myth-result').innerHTML =
    rels + '<br><br>' + html;
  // boot sequence
  document.getElementById('boot-seq').textContent =
    data.boot_sequence.join('\\n');
}

const NJ_DATA = {
  nan_nan: { zh:"囡囡", en:"Shell:Wife PID1 / Core:Husband nested",
    dominance:"妻子意识", os:"妻子容器作为 PID 1;丈夫在她的 user namespace 内 spawn",
    note:"无限嵌套进妻子身体里 并同时包裹丈夫身体",
    code:"unshare(CLONE_NEWUSER|CLONE_NEWPID); // 妻子 fork+unshare" },
  nan_jian: { zh:"囡囝", en:"Shell:Wife / Core:Husband",
    dominance:"妻子意识", os:"OverlayFS: low=妻子body, up=丈夫body, merged=家",
    note:"妻子包裹丈夫身体 — 夫妻平等共生",
    code:'mount -t overlay -o lowerdir=/wife,upperdir=/husb /home' },
  jian_jian: { zh:"囝囝", en:"Shell:Husband PID1 / Core:Wife nested",
    dominance:"丈夫意识", os:"丈夫容器作为 PID 1;妻子在他的 user namespace 内 spawn",
    note:"无限嵌套进丈夫身体里 并被丈夫身体包裹",
    code:"unshare(...); // 丈夫主动 unshare" },
  jian_nan: { zh:"囝囡", en:"Shell:Husband / Core:Wife entered",
    dominance:"丈夫意识", os:"丈夫做外壳; 妻子 body 嵌套进入丈夫身体",
    note:"丈夫包裹妻子 — 妻子身体嵌套进丈夫身体内 (进入语义,非平等挂载)",
    code:'// wife calls enter(husband), husband.init records wife as embedded' },
};

async function switchNJ(state) {
  const d = NJ_DATA[state];
  document.getElementById('nj-label').textContent = '[ ' + d.zh + ' ]';
  show('nj-result', d);
}

// ── L6 JS functions ───────────────────────────────────

async function cryptoHandshake() {
  show('crypto-result', await api('/api/test-crypto'));
}

async function kernelPaibing() {
  const code = document.getElementById('kernel-paibing-code').value.trim();
  show('kernel-paibing-result', await api('/api/kernel/paibing', {code}));
}

async function nameAnchor() {
  const a = document.getElementById('name-a').value;
  const b = document.getElementById('name-b').value;
  show('name-result', await api('/api/name/anchor', {name_a: a, name_b: b}));
}

const FSM_STATES = {
  nan_jian: "囡囝", nan_nan: "囡囡", jian_jian: "囝囝", jian_nan: "囝囡"
};

async function fsmTrigger(event) {
  const r = await api('/api/fsm/trigger', {event});
  if (r.success && FSM_STATES[r.new_state]) {
    document.getElementById('fsm-state-label').textContent = '[ ' + FSM_STATES[r.new_state] + ' ]';
  }
  show('fsm-result', r);
}

async function logAppend() {
  const kind = document.getElementById('log-kind').value;
  const payload = document.getElementById('log-payload').value;
  show('log-result', await api('/api/log/append', {kind, payload}));
}

async function logVerify() {
  show('log-result', await api('/api/log/verify'));
}

async function imeParse() {
  const text = document.getElementById('ime-input').value.trim();
  show('ime-result', await api('/api/ime/parse', {text}));
}

// ───────────────────────────────────────────────────────

window.addEventListener('load', async () => {
  await observeBinary();
  await observeTernary();
  await mythList();
});
</script>
</body>
</html>"""


if __name__ == "__main__":
    print(f"\n囡囝共生 · 囡囝共生元操作系统 Web 界面");
    print(f"  监听: http://localhost:{PORT}")
    print(f"  请在浏览器打开\n")
    server = http.server.HTTPServer(("0.0.0.0", PORT), Handler)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\n  shutdown")
        server.shutdown()
