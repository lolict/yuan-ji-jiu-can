#!/bin/bash
# 满全法 · 公开 Beta 构建脚本 v2.0
# ==================================
# 把 shell/browser/ + docs + license 打包为发布包

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
SRC_DIR="${PROJECT_DIR}/shell/browser"
OUT_DIR="${SCRIPT_DIR}/v2.0-beta"

echo "=== 满全法 v2.0 公开 Beta 打包 ==="
echo "源: ${SRC_DIR}"

# 清理并重建
rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

# ─── 核心文件 ────────────────────────────────────────
echo ">> 复制浏览器壳核心文件"
cp "${SRC_DIR}/index.html" "$OUT_DIR/"
cp "${SRC_DIR}/mqf-shell.js" "$OUT_DIR/"
cp "${SRC_DIR}/mqf-storage.js" "$OUT_DIR/"
cp "${SRC_DIR}/mobile-bridge.js" "$OUT_DIR/"
cp "${SRC_DIR}/shell.css" "$OUT_DIR/"

# ─── 文档 ──────────────────────────────────────────
echo ">> 复制文档"
mkdir -p "$OUT_DIR/docs"
cp "${PROJECT_DIR}/docs/MENGJI_TERMS.md" "$OUT_DIR/docs/" 2>/dev/null || true
cp "${PROJECT_DIR}/docs/MILESTONE_v17.md" "$OUT_DIR/docs/" 2>/dev/null || true
cp "${PROJECT_DIR}/docs/WASM_CORE_v17.md" "$OUT_DIR/docs/" 2>/dev/null || true

# ─── MANIFEST ──────────────────────────────────────
echo ">> 生成 manifest"
cat > "$OUT_DIR/MANIFEST.json" << EOF
{
  "name": "民联浏览器 · MQF Shell",
  "version": "v2.0-beta",
  "codename": "民联公开Beta",
  "release_date": "$(date +%Y-%m-%d)",
  "min_platforms": {
    "browser": "Chrome 90+, Firefox 90+, Safari 15+, Edge 90+",
    "android": "Android 8.0+ (API 26)",
    "ios": "iOS 16.0+",
    "desktop": "Windows 10+, macOS 12+, Linux (glibc 2.31+)"
  },
  "files": {
    "index.html": "入口 HTML",
    "mqf-shell.js": "裁定引擎 + UI 控制器",
    "mqf-storage.js": "持久化存储 (IndexedDB/localStorage)",
    "mobile-bridge.js": "移动端桥 (Android/iOS/浏览器)",
    "shell.css": "暗色主题样式"
  },
  "engine": {
    "five_yuan": "旋钮步量 + 角度 wrap + 占存 (>100 ticks)",
    "six_yuan": "对称群信任交集 + 镜像对齐 (<0.2)",
    "seven_yuan": "蜂窝套壳递归 (6 子壳)"
  },
  "features": [
    "离线运行 (file:// / assets)",
    "域无关: 同样的裁决结果",
    "移动端原生桥 (WebView / WKWebView)",
    "多节点 Gossip 同步",
    "裁定历史持久化 (500 条)",
    "七元蜂窝嵌套可视化",
    "旋钮/力度/角度实时控制",
    "多种预设 (ios/bank/android)"
  ],
  "integrity": "SHA-256 哈希见 SUMS.txt"
}
EOF

# ─── 快速上手 README ──────────────────────────────
cat > "$OUT_DIR/README.md" << 'EOF'
# 民联浏览器 · 满全法公开 Beta v2.0

七元裁定引擎 — 可在任何设备离线运行的智能裁决系统。

## 快速开始

### 方式 1: 浏览器直接打开
```
用浏览器打开 index.html
```
或
```bash
cd this_directory
python3 -m http.server 8080
# 访问 http://localhost:8080
```

### 方式 2: Android 壳
```bash
cd shell/native/mobile/android
./gradlew assembleDebug
# 输出: app/build/outputs/apk/debug/app-debug.apk
```

### 方式 3: iOS 壳
```bash
# 在 Xcode 中打开 ios/MqfShell.xcodeproj
# Product → Archive → Distribute
```

### 方式 4: 桌面 (Electron / Tauri)
```bash
# Electron
cd shell/native/electron-shell
npm install && npm start

# Tauri
cd shell/native/tauri-shell
cargo tauri dev
```

### 方式 5: Python WebView
```bash
pip install pywebview
python3 shell/native/mqf_webview.py
```

## WASM 内核 (可选)

将 mqf_core.wasm 放置于 index.html 旁边,浏览器会自动检测并使用。

```bash
# 编译
/opt/wasi-sdk/bin/clang --target=wasm32-wasi -O2 -o index.html.dir/mqf_core.wasm kernel/wasm/mqf_c
```

## 裁定 API

```javascript
// 在浏览器控制台直接调用:
mqfShell.runVerdict()

// 或直接调用核心:
const result = mqfVerdictEngine({
  entity: 'test',
  complianceA: 0.9,
  complianceB: 0.85,
  force: 15,
  angle: 45
});
// → { mqf_verdict: { trust: 'TRUSTED', confidence: 0.85, ... } }
```

## 多节点同步

```javascript
const { MqfSyncEngine } = require('cloudflare/sync-protocol.js');
const engine = new MqfSyncEngine({ peers: ['https://node-b.example.com'] });
await engine.syncTo('https://node-b.example.com');
```

## 盟机合术语

参见 `docs/MENGJI_TERMS.md`

## 许可证

内部使用 — lolict 保留所有权利。
EOF

# ─── 校验和 ────────────────────────────────────────
echo ">> 生成 SHA-256 校验"
if command -v sha256sum &> /dev/null; then
  (cd "$OUT_DIR" && sha256sum *.html *.js *.css > SUMS.txt 2>/dev/null)
elif command -v shasum &> /dev/null; then
  (cd "$OUT_DIR" && shasum -a 256 *.html *.js *.css > SUMS.txt 2>/dev/null)
fi

# ─── 统计 ──────────────────────────────────────────
FILE_COUNT=$(find "$OUT_DIR" -type f | wc -l)
TOTAL_SIZE=$(du -sh "$OUT_DIR" | cut -f1)

echo ""
echo "=== 构建完成 ==="
echo "输出目录: ${OUT_DIR}"
echo "文件数量: ${FILE_COUNT}"
echo "总大小:   ${TOTAL_SIZE}"
echo ""
echo "文件清单:"
find "$OUT_DIR" -type f | sort | while read f; do
  size=$(wc -c < "$f" | tr -d ' ')
  echo "  $(basename "$f") ($size bytes)"
done
