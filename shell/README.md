# 民联浏览器壳 · 囡囝囡囝 Shell v1.7

> 跨平台 WASM/JS 裁定引擎容器 — 七元裁定随时随地运行

---

## 目录结构

```
shell/
├── browser/                  # 浏览器壳 (核心)
│   ├── index.html            # 主界面 (三栏布局)
│   ├── mqf-shell.js          # 裁定核心 + WASM 加载器
│   └── shell.css             # 暗色裁定台样式
├── native/                   # 原生包装器
│   ├── mqf_webview.py        # Python pywebview 启动器
│   ├── electron-shell/       # Electron 包装 (Win/Mac/Linux)
│   │   ├── package.json
│   │   ├── main.js
│   │   └── preload.js
│   └── tauri-shell/          # Tauri 包装 (更轻量)
│       └── src-tauri/
│           ├── Cargo.toml
│           ├── tauri.conf.json
│           └── src/main.rs
└── README.md                 # 本文件
```

---

## 运行方式

### 方式 1: 纯浏览器 (最快,零安装)

```bash
cd shell/browser
python3 -m http.server 8080
# 打开 http://localhost:8080
```

### 方式 2: Python pywebview

```bash
pip install pywebview
python3 shell/native/mqf_webview.py
```

> 注意: Linux 需要安装 WebKitGTK (`apt install libwebkit2gtk-4.0-dev`)

### 方式 3: Electron (独立 exe/dmg/AppImage)

```bash
cd shell/native/electron-shell
npm install
npm start                    # 开发运行
npm run build:win            # 打包 Windows
npm run build:mac            # 打包 macOS
npm run build:linux          # 打包 Linux
```

### 方式 4: Tauri (更小体积,更安全)

```bash
cd shell/native/tauri-shell
cargo tauri dev              # 开发运行
cargo tauri build            # 打包发布
```

### 方式 5: Android WebView

将 `shell/browser/` 内容放入 Android 项目的 `assets/` 目录,
用 `WebView.loadUrl("file:///android_asset/index.html")` 加载。

### 方式 6: iOS WKWebView

同上,iOS 项目加载本地 HTML,使用 `WKWebView.loadFileURL()`。

---

## WASM 内核集成

当 `njsg_core.wasm` 文件放置于 `shell/browser/` 目录下时,
壳会自动检测并使用 WASI 接口调用裁定引擎。

```bash
# 编译 WASM 内核 (需要 WASI SDK)
/opt/wasi-sdk/bin/clang \
  --target=wasm32-wasi \
  -O2 \
  -o shell/browser/njsg_core.wasm \
  kernel/wasm/mqf_wasm.c
```

如 WASM 不可用,壳会自动回退到 JS 实现的裁定引擎 (算法完全一致)。

---

## 界面说明

```
┌──────────────────────────────────────────────────────┐
│  ⚙ 民联浏览器 · 囡囝囡囝 Shell          [WASM | JS 回退]  │
├────────────┬─────────────────────────┬───────────────┤
│  旋钮参数   │       裁定台            │   裁定历史     │
│            │                         │               │
│  实体 [ios]│      ★ / ○             │  ios  TRUSTED │
│  合规 A ═══│   TRUSTED (信赖)        │  and REJECTED │
│  合规 B ═══│   已被铭记 双重背书      │  bank TRUSTED │
│  力度  ═══ │                         │               │
│  角度  ═══ │   A: 0.85 ≈ 对齐 B:0.80│               │
│            │                         │               │
│ [iOS]      │ ticks=150 · ETERNAL    │               │
│ [Android]  │ angle=45° · 交集=0.80  │               │
│ [Bank]     │                         │               │
│            │ 七元蜂窝套壳 · 节点树   │               │
│            │ sys.root                │               │
│            │  sys.root.0 (child)     │               │
│            │  sys.root.1             │               │
│            │ ...                     │               │
│            │                         │               │
│ [⚡ 裁定]   │ [系统日志]              │               │
│ [🔶 蜂窝]   │ [14:32:01] mode=js      │               │
└────────────┴─────────────────────────┴───────────────┘
```

---

## 七元算法对照

| 维度 | 工程含义 | UI 显示 |
|------|---------|--------|
| 五元 | 旋钮步量 + 角度 wrap + 占存 | ticks / ETERNAL·EPHEMERAL / 角度 |
| 六元 | 信任交集 + 镜像对齐 | 交集值 / ALIGNED·OFFSET |
| 七元 | 蜂窝套壳递归 | 节点树 + 空间比 |

---

## 跨域裁定一致性

无论通过哪种方式运行壳,无论域名/IP 是什么,
同样的裁定输入 → 同样的输出:

```json
{
  "entity": "ios",
  "compliance_a": 0.9,
  "compliance_b": 0.85,
  "force": 15,
  "angle": 45
}
→ TRUSTED, ★铭记, 双重背书, ticks=150, ETERNAL
```

---

## 离线运行

浏览器壳完全支持离线:

1. 直接 `file://` 协议打开 `index.html`
2. 无需任何网络请求
3. 裁定引擎为本地 JS/WASM 计算
4. 历史记录保存在浏览器本地 (可扩展 localStorage)

---

*民联浏览器壳 · v1.7 · 2026-08-05*
