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
