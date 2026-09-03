#!/usr/bin/env python3
"""
kernel_bridge.py — WASM 内核 HTTP 桥接服务器

将囡囝共生 WASM 内核的状态和操作暴露为 JSON API，
使浏览器能够远程可视化和操作内核。

架构:
  ┌─────────────┐     HTTP/JSON      ┌─────────────────┐     ctypes     ┌──────────┐
  │   Browser   │ ◄───────────────► │  kernel_bridge  │ ─────────────► │ libcolor │
  │  (HTML/JS)  │                   │   (Python HTTP) │                │   .so    │
  └─────────────┘                   └─────────────────┘                └──────────┘

红线:
  - 浏览器只能读取和追加 cell，不能修改已有 cell (不可变历史)
  - 所有操作记录到内核持久化文件 (wk_state.bin)
  - 不暴露删除操作 — 囡囝共生内核是一旦写入永不删除

用法:
  python3 kernel_bridge.py              # 启动服务器 (默认 127.0.0.1:19800)
  python3 kernel_bridge.py --port 9000  # 自定义端口
"""

import sys
import os
import json
import threading
from http.server import HTTPServer, BaseHTTPRequestHandler
from io import BytesIO

# 确保能 import 包
_bridge_dir = os.path.dirname(os.path.abspath(__file__))
if _bridge_dir not in sys.path:
    sys.path.insert(0, _bridge_dir)

from phase_radix._native import (
    WasmKernelRef,
    MindOrganRef,
    InterlockRef,
    PropertyFlowRef,
    MetaImeEngine,
    CourtEngine,
    _wk_cell, _wk_cell_mccp, _wk_cell_sovereign, _wk_cell_polarity, _wk_cell_witness,
    MO_DIST_GEOMETRIC, MO_DIST_SOVEREIGN, MO_DIST_POLARITY, MO_DIST_WITNESS, MO_DIST_MCCP,
    WK_STATE_NN, WK_STATE_NJSG, WK_STATE_JJ, WK_STATE_JN,
    WK_POLARITY_WIFE, WK_POLARITY_HUSBAND, WK_POLARITY_SYSTEM,
    WK_FLOW_PRELOAD, WK_FLOW_CATEGORY, WK_FLOW_CLUSTER, WK_FLOW_DISTINGUISH,
    WK_FLOW_PROPERTY, WK_FLOW_INTERPRET, WK_FLOW_RELATION, WK_FLOW_CONCEPT,
    PROPERTY_ALLOW, PROPERTY_DENY_UNREGISTERED, PROPERTY_DENY_WRONG_OWNER,
    PROPERTY_DENY_HISTORY, PROPERTY_CONDITIONAL,
    USE_READ, USE_WRITE, USE_EXPORT, USE_DERIVE,
    METAIME_POL_WIFE, METAIME_POL_HUSBAND, METAIME_POL_SYSTEM,
    METAIME_FLOW_PRELOAD, METAIME_FLOW_CATEGORY, METAIME_FLOW_CLUSTER, METAIME_FLOW_DISTINGUISH,
    METAIME_FLOW_PROPERTY, METAIME_FLOW_INTERPRET, METAIME_FLOW_RELATION, METAIME_FLOW_CONCEPT,
    COURT_STATE_NN, COURT_STATE_NJSG, COURT_STATE_JJ, COURT_STATE_JN,
    COURT_MERGE, COURT_REPEL, COURT_WITNESS, COURT_TRANSFORM, COURT_DEFER, COURT_ESCALATE,
)

# ── 全局内核状态 ──────────────────────────────────────────

_kernel = None
_interlock = None
_property_flow = None
_metaime = None
_court = None
_state_file = None
_lock = threading.Lock()


def _get_kernel():
    """获取全局内核 (自动加载)"""
    global _kernel
    if _kernel is None:
        _kernel = WasmKernelRef()
        if _state_file and os.path.exists(_state_file):
            _kernel.load(_state_file)
    return _kernel


def _get_interlock():
    global _interlock
    if _interlock is None:
        _interlock = InterlockRef()
    return _interlock


def _get_property_flow():
    global _property_flow
    if _property_flow is None:
        _property_flow = PropertyFlowRef()
    return _property_flow


def _get_metaime():
    global _metaime
    if _metaime is None:
        _metaime = MetaImeEngine()
    return _metaime


def _get_court():
    global _court
    if _court is None:
        _court = CourtEngine()
    return _court


def _autosave():
    """自动保存内核状态"""
    global _kernel, _state_file
    if _kernel and _state_file:
        _kernel.save(_state_file)


# ── JSON 响应辅助 ─────────────────────────────────────────

def _json_response(handler, data, status=200):
    """发送 JSON 响应"""
    body = json.dumps(data, ensure_ascii=False, default=str).encode('utf-8')
    handler.send_response(status)
    handler.send_header('Content-Type', 'application/json; charset=utf-8')
    handler.send_header('Content-Length', str(len(body)))
    handler.send_header('Access-Control-Allow-Origin', '*')
    handler.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
    handler.send_header('Access-Control-Allow-Headers', 'Content-Type')
    handler.end_headers()
    handler.wfile.write(body)


def _json_error(handler, msg, status=400):
    _json_response(handler, {"error": msg}, status)


def _read_json_body(handler):
    """读取并解析 JSON 请求体"""
    content_length = int(handler.headers.get('Content-Length', 0))
    if content_length == 0:
        return {}
    raw = handler.rfile.read(content_length)
    try:
        return json.loads(raw.decode('utf-8'))
    except (json.JSONDecodeError, UnicodeDecodeError) as e:
        raise ValueError(f"JSON 解析失败: {e}")


# ── HTTP 请求处理器 ───────────────────────────────────────

class KernelBridgeHandler(BaseHTTPRequestHandler):
    """WASM 内核 HTTP 桥接处理器"""

    def log_message(self, fmt, *args):
        """自定义日志格式"""
        sys.stderr.write(f"[kernel_bridge] {self.address_string()} - {fmt % args}\n")

    def do_OPTIONS(self):
        """处理 CORS 预检"""
        self.send_response(204)
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        self.end_headers()

    def do_GET(self):
        GET_ROUTES = {
            '/': self._handle_index,
            '/api/kernel/state': self._handle_kernel_state,
            '/api/kernel/stats': self._handle_kernel_stats,
            '/api/kernel/cells': self._handle_kernel_cells,
            '/api/interlock/status': self._handle_interlock_status,
            '/api/property/stats': self._handle_property_stats,
            '/api/mo/demo': self._handle_mo_demo,
        }
        handler = GET_ROUTES.get(self.path)
        if handler:
            try:
                handler()
            except Exception as e:
                _json_error(handler, str(e), 500)
        else:
            _json_error(handler, f"未知路径: {self.path}", 404)

    def do_POST(self):
        POST_ROUTES = {
            '/api/kernel/cell/add': self._handle_cell_add,
            '/api/kernel/cell/find': self._api_kernel_cell_find,
            '/api/kernel/route': self._handle_kernel_route,
            '/api/kernel/save': self._handle_kernel_save,
            '/api/kernel/load': self._handle_kernel_load,
            '/api/kernel/lock': self._api_kernel_lock,
            '/api/kernel/flow': self._handle_kernel_flow,
            '/api/interlock/register': self._handle_interlock_register,
            '/api/property/register': self._handle_property_register,
            '/api/property/check': self._handle_property_check,
            '/api/mo/cluster': self._handle_mo_cluster,
            '/api/mo/distance': self._handle_mo_distance,
            '/api/metaime/encode': self._handle_metaime_encode,
            '/api/metaime/decode': self._handle_metaime_decode,
            '/api/metaime/feed': self._handle_metaime_feed,
            '/api/metaime/stats': self._handle_metaime_stats,
            '/api/court/state': self._handle_court_state,
            '/api/court/flow': self._handle_court_flow,
            '/api/court/judge': self._handle_court_judge,
            '/api/court/metaime': self._handle_court_metaime,
            '/api/court/stats': self._handle_court_stats,
        }
        handler = POST_ROUTES.get(self.path)
        if handler:
            try:
                handler()
            except Exception as e:
                _json_error(handler, str(e), 500)
        else:
            _json_error(handler, f"未知路径: {self.path}", 404)

    # ── GET 处理器 ──

    def _handle_index(self):
        _json_response(self, {
            "name": "囡囝共生 WASM 内核桥接器",
            "version": "0.3.0",
            "endpoints": {
                "GET": ["/api/kernel/state", "/api/kernel/stats", "/api/kernel/cells",
                        "/api/interlock/status", "/api/property/stats", "/api/mo/demo"],
                "POST": ["/api/kernel/cell/add", "/api/kernel/cell/find",
                         "/api/kernel/route", "/api/kernel/save", "/api/kernel/load",
                         "/api/kernel/lock", "/api/kernel/flow",
                         "/api/interlock/register",
                         "/api/property/register", "/api/property/check",
                         "/api/mo/cluster", "/api/mo/distance"]
            }
        })

    def _handle_kernel_state(self):
        k = _get_kernel()
        state_names = {0: "NN", 1: "NJSG", 2: "JJ", 3: "JN"}
        polarity_names = {0: "妻", 1: "夫", 2: "系统"}
        s = k.stats()
        _json_response(self, {
            "state": state_names.get(k.state, f"0x{k.state:02X}"),
            "locked": k.locked,
            "stats": s,
        })

    def _handle_kernel_stats(self):
        k = _get_kernel()
        _json_response(self, {"stats": k.stats()})

    def _handle_kernel_cells(self):
        """获取所有 cell (从内核 buffer 中提取)"""
        k = _get_kernel()
        buf = bytes(k._buf)
        # struct layout:
        #   offset 0: magic (4), version (2), state (1), flags (1)
        #   offset 8: mem_base ptr (8)
        #   offset 16: mem_size (4), mem_capacity (4)
        #   offset 24: symbols[254] (2032 bytes = 254*8)
        #   offset 2056: symbol_count (2)
        cell_count = int.from_bytes(buf[2056:2058], 'little')
        cells = []
        for i in range(min(cell_count, 254)):
            offset = 24 + i * 8
            cell_bytes = buf[offset:offset + 8]
            if len(cell_bytes) < 8:
                break
            cell = int.from_bytes(cell_bytes, 'little')
            cells.append({
                "index": i,
                "raw": f"0x{cell:016X}",
                "mccp": f"0x{_wk_cell_mccp(cell):08X}",
                "mccp_int": _wk_cell_mccp(cell),
                "sovereignty": _wk_cell_sovereign(cell),
                "sovereignty_hex": f"0x{_wk_cell_sovereign(cell):05X}",
                "polarity": _wk_cell_polarity(cell),
                "witness": _wk_cell_witness(cell),
            })
        _json_response(self, {"count": cell_count, "cells": cells})

    def _handle_interlock_status(self):
        il = _get_interlock()
        _json_response(self, {"interlock": il.stats()})

    def _handle_property_stats(self):
        _json_response(self, {"status": "active"})

    def _handle_mo_demo(self):
        """返回演示 cluster 数据"""
        mo = MindOrganRef()
        for i in range(6):
            mo.add_cell(
                mccp=0x41 + i,
                sovereignty=0x100 + i * 16,
                polarity=i % 3,
                witness=i % 4
            )
        result = mo.cluster(eps=0.5, min_pts=2, dist_type=MO_DIST_GEOMETRIC)
        _json_response(self, {"demo_cluster": result})

    # ── POST 处理器 ──

    def _handle_cell_add(self):
        body = _read_json_body(self)
        k = _get_kernel()
        mccp = body.get('mccp')
        if mccp is None:
            mccp = body.get('mccp_int', 0x41)
        elif isinstance(mccp, str):
            mccp = int(mccp, 0)

        sovereignty = body.get('sovereignty', 0x100)
        if isinstance(sovereignty, str):
            sovereignty = int(sovereignty, 0)

        polarity = body.get('polarity', 0)
        witness = body.get('witness', 0)

        with _lock:
            ok = k.add_cell(mccp, sovereignty, polarity, witness)
            if ok:
                _autosave()

        _json_response(self, {
            "success": ok,
            "cell": {
                "mccp": f"0x{mccp:08X}",
                "mccp_int": mccp,
                "sovereignty": sovereignty,
                "sovereignty_hex": f"0x{sovereignty:05X}",
                "polarity": polarity,
                "witness": witness,
            }
        })

    def _api_kernel_cell_find(self):
        body = _read_json_body(self)
        k = _get_kernel()
        mccp = body.get('mccp', 0x41)
        if isinstance(mccp, str):
            mccp = int(mccp, 0)

        cell = k.find_mccp(mccp)
        found = cell != 0
        result = {
            "found": found,
            "mccp": f"0x{mccp:08X}",
        }
        if found:
            result["cell"] = {
                "raw": f"0x{cell:016X}",
                "mccp": f"0x{_wk_cell_mccp(cell):08X}",
                "sovereignty": _wk_cell_sovereign(cell),
                "polarity": _wk_cell_polarity(cell),
                "witness": _wk_cell_witness(cell),
            }
        _json_response(self, result)

    def _handle_kernel_route(self):
        body = _read_json_body(self)
        k = _get_kernel()
        mccp = body.get('mccp', 0x41)
        if isinstance(mccp, str):
            mccp = int(mccp, 0)
        sovereignty = body.get('sovereignty', 0x100)
        if isinstance(sovereignty, str):
            sovereignty = int(sovereignty, 0)
        polarity = body.get('polarity', 0)
        witness = body.get('witness', 0)
        flow_type = body.get('flow_type', 0)

        rc = k.route(mccp, sovereignty, polarity, witness, flow_type)
        _json_response(self, {
            "result_code": rc,
            "verdict": k.route_str(rc),
            "flow_type": flow_type,
        })

    def _handle_kernel_save(self):
        k = _get_kernel()
        with _lock:
            ok = k.save(_state_file) if _state_file else False
        _json_response(self, {"success": ok, "path": _state_file})

    def _handle_kernel_load(self):
        k = _get_kernel()
        path = _read_json_body(self).get('path', _state_file)
        with _lock:
            ok = k.load(path) if path and os.path.exists(path) else False
        _json_response(self, {"success": ok, "path": path})

    def _api_kernel_lock(self):
        body = _read_json_body(self)
        k = _get_kernel()
        k.locked = body.get('locked', True)
        _json_response(self, {"locked": k.locked})

    def _handle_kernel_flow(self):
        body = _read_json_body(self)
        k = _get_kernel()
        flow_type = body.get('flow_type', 0)
        active = body.get('active', True)
        with _lock:
            if active:
                k.activate_flow(flow_type)
        flow_names = ["预加载","分类","聚类","分别","产权","解释","关系","概念"]
        _json_response(self, {"flow_type": flow_type, "flow_name": flow_names[flow_type] if flow_type < 8 else "?", "active": active})

    def _handle_interlock_register(self):
        body = _read_json_body(self)
        il = _get_interlock()
        symbol = body.get('symbol')
        if isinstance(symbol, str):
            symbol = int(symbol, 0)
        container = body.get('container', 1)
        with _lock:
            ok = il.register_symbol(symbol, container)
        _json_response(self, {"success": ok, "symbol": f"0x{symbol:08X}"})

    def _handle_property_register(self):
        body = _read_json_body(self)
        pf = _get_property_flow()
        symbol = body.get('symbol')
        if isinstance(symbol, str):
            symbol = int(symbol, 0)
        owner = body.get('owner', 1)
        with _lock:
            ok = pf.register(symbol, owner)
        _json_response(self, {"success": ok, "symbol": f"0x{symbol:08X}"})

    def _handle_property_check(self):
        body = _read_json_body(self)
        pf = _get_property_flow()
        symbol = body.get('symbol')
        if isinstance(symbol, str):
            symbol = int(symbol, 0)
        requester = body.get('requester', 1)
        use_type = body.get('use_type', USE_READ)
        verdict = pf.check(symbol, requester, use_type)
        _json_response(self, {"verdict": verdict, "verdict_str": pf.verdict_str(verdict)})

    def _handle_mo_cluster(self):
        body = _read_json_body(self)
        mo = MindOrganRef()
        cells = body.get('cells', [])
        for c in cells:
            mccp = c.get('mccp', 0x41)
            if isinstance(mccp, str):
                mccp = int(mccp, 0)
            sov = c.get('sovereignty', 0x100)
            pol = c.get('polarity', 0)
            wit = c.get('witness', 0)
            mo.add_cell(mccp, sov, pol, wit)

        eps = body.get('eps', 0.5)
        min_pts = body.get('min_pts', 2)
        dist_type = body.get('dist_type', MO_DIST_GEOMETRIC)

        result = mo.cluster(eps, min_pts, dist_type)
        _json_response(self, {"cluster_result": result})

    def _handle_mo_distance(self):
        body = _read_json_body(self)
        mo = MindOrganRef()
        cells = body.get('cells', [])
        for c in cells:
            mccp = c.get('mccp', 0x41)
            if isinstance(mccp, str):
                mccp = int(mccp, 0)
            mo.add_cell(mccp, c.get('sovereignty', 0x100), c.get('polarity', 0), c.get('witness', 0))

        i = body.get('i', 0)
        j = body.get('j', 1)
        dist_type = body.get('dist_type', MO_DIST_GEOMETRIC)

        d = mo.distance(i, j, dist_type)
        dist_names = ["几何","主权","极性","见证","MCCP"]
        _json_response(self, {
            "distance": d,
            "from": i,
            "to": j,
            "dist_type": dist_type,
            "dist_name": dist_names[dist_type] if dist_type < 5 else "?"
        })


    # ── MetaIME 处理器 ──

    def _handle_metaime_encode(self):
        body = _read_json_body(self)
        mi = _get_metaime()
        sm = body.get('shengmu', 0)
        ym = body.get('yunmu', 0)
        tone = body.get('tone', 0)
        pol = body.get('polarity', 0)
        wit = body.get('witness', 0)
        flow = body.get('flow', 0)
        sov = body.get('sovereignty', 0x100)
        cell = mi.encode(sm, ym, tone, pol, wit, flow, sov)
        d = mi.decode(cell)
        _json_response(self, {
            "cell": f"0x{cell:012X}",
            "cell_int": cell,
            "decode": d,
        })

    def _handle_metaime_decode(self):
        body = _read_json_body(self)
        mi = _get_metaime()
        cell = body.get('cell', 0)
        if isinstance(cell, str):
            cell = int(cell, 0)
        d = mi.decode(cell)
        _json_response(self, d)

    def _handle_metaime_feed(self):
        body = _read_json_body(self)
        mi = _get_metaime()
        syllables = body.get('syllables', [])
        match = mi.feed(syllables)
        _json_response(self, {
            "matched": match is not None,
            "match": match,
            "stats": mi.stats(),
        })

    def _handle_metaime_stats(self):
        mi = _get_metaime()
        _json_response(self, {"stats": mi.stats()})

    # ── Court 处理器 ──

    def _handle_court_state(self):
        body = _read_json_body(self)
        co = _get_court()
        state = body.get('state', 1)
        co.set_state(state)
        _json_response(self, {"state": state, "state_name": _court_state_name(state)})

    def _handle_court_flow(self):
        body = _read_json_body(self)
        co = _get_court()
        flow = body.get('flow', 0)
        co.activate_flow(flow)
        _json_response(self, {"flow": flow})

    def _handle_court_judge(self):
        body = _read_json_body(self)
        co = _get_court()
        cell_a = body.get('cell_a', 0)
        cell_b = body.get('cell_b', 0)
        if isinstance(cell_a, str):
            cell_a = int(cell_a, 0)
        if isinstance(cell_b, str):
            cell_b = int(cell_b, 0)
        flow = body.get('flow', 0)
        r = co.judge(cell_a, cell_b, flow)
        _json_response(self, r)

    def _handle_court_metaime(self):
        body = _read_json_body(self)
        co = _get_court()
        sm = body.get('shengmu', 0)
        ym = body.get('yunmu', 0)
        tone = body.get('tone', 0)
        pol = body.get('polarity', 0)
        wit = body.get('witness', 0)
        r = co.route_metaime(sm, ym, tone, pol, wit)
        _json_response(self, r)

    def _handle_court_stats(self):
        co = _get_court()
        _json_response(self, {"stats": co.stats()})


# ── 辅助 ─────────────────────────────────────────────────

_COURT_STATE_NAMES = {0: "囡囡(NN)", 1: "囡囝(NJSG)", 2: "囝囝(JJ)", 3: "囝囡(JN)"}

def _court_state_name(s):
    return _COURT_STATE_NAMES.get(s, f"未知({s})")


# ── 服务器启动 ─────────────────────────────────────────────

def start_server(host='127.0.0.1', port=19800, state_file=None):
    """启动内核桥接服务器"""
    global _state_file
    _state_file = state_file or os.path.join(os.getcwd(), "wk_state.bin")

    server = HTTPServer((host, port), KernelBridgeHandler)
    print(f"═══ 囡囝共生 WASM 内核桥接器 ═══")
    print(f"  监听: http://{host}:{port}")
    print(f"  状态文件: {_state_file}")
    print(f"  API: http://{host}:{port}/")
    print(f"  按 Ctrl+C 停止")
    print(f"═══ {'═' * 30} ═══")

    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\n服务器已停止")
        server.server_close()


if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser(description='囡囝共生 WASM 内核 HTTP 桥接器')
    parser.add_argument('--host', default='127.0.0.1', help='监听地址 (默认 127.0.0.1)')
    parser.add_argument('--port', type=int, default=19800, help='监听端口 (默认 19800)')
    parser.add_argument('--state', default=None, help='内核状态文件路径')
    args = parser.parse_args()
    start_server(args.host, args.port, args.state)
