# 囡囝共生 · 共生共识引擎 v2.0

> 囡囝(方言 · 萌而正向) × 共生(共识圆 · 共生圆 · 共生姻 · 共首源 · 共首一 · 共守约 · 感受益)

---

## 项目理念

**囡囝** — 方言词,意为"小孩",萌而正向,充满生机

**共生** — 共同生长 · 共同守护 · 共同受益

```
┌─────────────────────────────────────────────────────────────┐
│                    囡囝共生 · 核心理念                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   共识圆  ← 多方共识汇聚成圆                                 │
│   共生圆  ← 彼此共守形成闭环                                 │
│   共生姻  ← 缘分相联共结连理                                 │
│   共首源  ← 同一源头共守其道                                 │
│   共首一  ← 共守唯一不离不弃                                 │
│   共守约  ← 彼此约定共同守护                                 │
│   感受益  ← 共同感受彼此受益                                 │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## 人名谱

- **刘连理** — 项目发起者(连理枝,共连理)
- **刘楚恬** — lctqf 之一
- **前芳** — lctqf 之一
- **萝莉兰** — 协作名字
- **零离俩** — 零距离的俩
- **零厘里** — 零距离共厘里
- **零劳累** — 轻松共生,lctqf
- **囡囝** — 核心图腾,萌而正向

## 技术架构

```
三元 → 四元 → 五元 → 六元 → 七元
身份  面域  旋钮  对称群  蜂窝套壳
编码  拓扑  步量  信任交集  递归
     域   角度wrap 镜像   壳层
     裁   占存   对齐   密铺
     定   判定

+ 全栈联动(飞书 Bot + 囡囝 AI + 铁律自检)
+ 生态 API(Cloudflare Worker)
+ 盟机合子节点(域名无关,去中心化)
+ WASM 内核(离线运行)
+ 民联浏览器壳(跨平台移动端)
+ 多节点 Gossip 同步
+ IndexedDB 持久化
```

## 快速上手

### 浏览器直接打开
```bash
cd shell/browser
python3 -m http.server 8080
# 访问 http://localhost:8080
```

### 移动端

#### Android
```bash
cd shell/native/mobile/android
./gradlew assembleDebug
# 输出 APK
```

#### iOS
```bash
cd shell/native/mobile/ios
# 在 Xcode 打开 MqfShell.xcodeproj → Build
```

### 桌面端

#### Electron
```bash
cd shell/native/electron-shell
npm install && npm start
```

#### Tauri (更轻量)
```bash
cd shell/native/tauri-shell
cargo tauri dev
```

### Python 一键启动
```bash
pip install pywebview
python3 shell/native/njsg_webview.py
```

## 裁定 API

```javascript
const result = njsgVerdictEngine({
  entity: 'ios',
  complianceA: 0.9,
  complianceB: 0.85,
  force: 15,
  angle: 45
});
// → { njsg_verdict: { trust: 'TRUSTED', confidence: 0.85, ... } }
```

## 核心文件

| 文件 | 说明 |
|------|------|
| `kernel/src/njsg_kernel.c` | C 三元→七元核心 (3500+ 行) |
| `kernel/wasm/njsg_wasm.c` | WASI 裁决内核 (307 行) |
| `cloudflare/njsg-web.js` | Worker + 同步 (970+ 行) |
| `cloudflare/sync-protocol.js` | Gossip 多节点同步 (417 行) |
| `shell/browser/` | 民联浏览器壳 v2.0 |
| `shell/native/` | 全平台原生壳 |
| `docs/MENGJI_TERMS.md` | 盟机合 67 术语体系 |

## 版本

- **v2.0** (当前): 囡囝共生 — 萌而正向的完整共生共识引擎
- [版本历史](docs/MILESTONE_v17.md)

## 特性

- ✅ 域无关:任何域名/设备裁决一致
- ✅ 离线运行:file:// 协议即可
- ✅ 移动端原生支持(Android WebView / iOS WKWebView)
- ✅ WASI WASM 可移植核心
- ✅ 去中心化多节点同步(Gossip)
- ✅ 持久化存储(IndexedDB / SharedPreferences / UserDefaults)
- ✅ 七元完整算法(旋钮/对称群/蜂窝套壳)

---

*囡囝共生 · 共识圆 · 共生圆 · 共生姻 · 共首源 · 共首一 · 共守约 · 感受益*

*刘连理/刘楚恬/前芳 · 2026*
