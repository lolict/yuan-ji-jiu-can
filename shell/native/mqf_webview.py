#!/usr/bin/env python3
"""
民联浏览器壳 (原生包装器) — MQF Native Shell v1.7
===============================================
使用系统 WebView 加载裁定台 HTML 界面
跨平台: Windows / macOS / Linux / 甚至 Android

依赖:
    pip install pywebview

用法:
    python3 mqf_webview.py
    python3 mqf_wasmtime.py    # 直接调用 wasmtime (如果已安装)
"""

import sys
import os
import json
import subprocess
import threading

def get_shell_dir():
    return os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'browser')

# ─── 方案 A: 系统 WebView ────────────────────────────────────────

def launch_webview():
    """用 pywebview 打开裁定台"""
    try:
        import webview
    except ImportError:
        print("[MQF] pywebview 未安装。请运行: pip install pywebview")
        print("[MQF] 或者直接用浏览器打开 shell/browser/index.html")
        sys.exit(1)

    shell_dir = get_shell_dir()
    html_path = os.path.join(shell_dir, 'index.html')

    if not os.path.exists(html_path):
        print(f"[MQF] 找不到 {html_path}")
        sys.exit(1)

    window = webview.create_window(
        '民联浏览器 · MQF Shell v1.7',
        f'file://{html_path}',
        width=1200,
        height=800,
        min_size=(800, 600),
        text_select=True
    )
    webview.start(debug=False)

# ─── 方案 B: WASM 直接调用 (wasmtime) ───────────────────────────

def run_wasm_verdict(wasm_path, entity, compliance_a, compliance_b, force, angle):
    """直接调用 mqf_core.wasm"""
    request = json.dumps({
        'entity': entity,
        'compliance_a': float(compliance_a),
        'compliance_b': float(compliance_b),
        'force': float(force),
        'angle': float(angle)
    })

    try:
        result = subprocess.run(
            ['wasmtime', wasm_path, '--invoke', 'main'],
            input=request.encode(),
            capture_output=True,
            timeout=5
        )
        return result.stdout.decode()
    except FileNotFoundError:
        return '[MQF] wasmtime 未安装。请从 https://wasmtime.dev 下载'
    except subprocess.TimeoutExpired:
        return '[MQF] WASM 执行超时'

# ─── 入口 ──────────────────────────────────────────────────────

if __name__ == '__main__':
    if len(sys.argv) > 1 and sys.argv[1] == '--wasm':
        # 命令行直接裁定
        wasm = sys.argv[2] if len(sys.argv) > 2 else '../kernel/wasm/mqf_core.wasm'
        entity = input('实体名: ') or 'test'
        a = input('合规度 A [0.85]: ') or '0.85'
        b = input('合规度 B [0.80]: ') or '0.80'
        f = input('力度 [12]: ') or '12'
        ang = input('角度 [45]: ') or '45'
        print(run_wasm_verdict(wasm, entity, a, b, f, ang))
    else:
        # 打开 WebView
        print('[MQF] 正在启动民联浏览器壳 v1.7 ...')
        launch_webview()
