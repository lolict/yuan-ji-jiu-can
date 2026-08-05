# 囡囝共生 v1.6-v1.7 里程碑 · 总结文档

> 从中心化到去中心化 — 囡囝共生生态演化记录

---

## 一、版本回顾

### v1.6 盟机合子节点 — 合伙人架构

**核心理念**: 资源归合伙人,主导权归囡囝共生

**关键成果**:
| 组件 | 文件 | 说明 |
|------|------|------|
| 盟机合术语 | `docs/MENGJI_TERMS.md` | 67 个术语,三层架构(决策/资源/服务) |
| 合伙人注册 | `cloudflare/njsg-web.js` | `apiMengjiRegister/status/list` 三个端点 |
| 域名无关化 | `cloudflare/njsg-web.js` | `apiDomainCheck` + 相同输入必相同输出 |
| 合伙人目录 | in-memory 注册表 | MENGJI_REGISTRY per-isolate |

**三层模型**:
```
┌──────────────────────────────────────────────────────────┐
│  决策层(主权在我们)                                        │
│  盟机合掌 ←→ 盟机合主 ←→ 盟机制心                         │
└──────────────────────────────────────────────────────────┘
         │ 制定规则
         ▼
┌──────────────────────────────────────────────────────────┐
│  资源层(合伙人提供)                                        │
│  盟机合站 ←→ 盟机合址 ←→ 盟机合装                         │
│  合伙人域名/服务器/带宽                                    │
└──────────────────────────────────────────────────────────┘
         │ 服务交付
         ▼
┌──────────────────────────────────────────────────────────┐
│  服务层(协同运作)                                         │
│  盟机合智 ←→ 盟机合志 ←→ 盟机合证                         │
│  裁定引擎 + 审计日志 + 证明签发                            │
└──────────────────────────────────────────────────────────┘
```

**关键决策**: 
- "partner" → "盟机合掌" 术语切换,消除工业化气息
- 域无关裁定: 无论 lolict.workers.dev 还是合伙人自有域名,同样的 ios 输入 → 同样的 TRUSTED 输出

---

### v1.7 WASM 内核 + 民联浏览器壳 — 去中心化运行

**核心理念**: 裁决引擎脱离任何中心化平台,运行在用户自己的设备

#### v1.7a 设计

文档 `docs/WASM_CORE_v17.md` 规划:
- WASI SDK 编译 njsg_wasm.c → njsg_core.wasm
- 纯 stdin/stdout JSON 接口
- 五层跨平台的壳方案 (WebView2/WKWebView/WebKitGTK/Android/iOS)

#### v1.7b WASM 内核移植

文件 `kernel/wasm/njsg_wasm.c` (307 行):

| 维度 | 实现 | 测试 |
|------|------|------|
| 五元旋钮 | step + angle wrap 360° + 占存 >100 → ETERNAL | ✅ 150 ticks, ETERNAL, 角度 45° |
| 六元对称 | min(compliance) + \|a-b\|<0.2 → 双重背书 | ✅ ios/bank TRUSTED, android REJECTED |
| 七元蜂窝 | 递归壳嵌套,每层 6 child | ✅ 1+6+6=13 节点 |
| JSON I/O | stdin 解析 + stdout 生成 | ✅ |

```bash
# 编译
/opt/wasi-sdk/bin/clang --target=wasm32-wasi -O2 -o njsg_core.wasm njsg_wasm.c

# 运行
echo '{"entity":"ios","compliance_a":0.9,"compliance_b":0.85,"force":15,"angle":45}' | wasmtime njsg_core.wasm
# → { "njsg_verdict": { "trust": "TRUSTED", ... } }
```

#### v1.7c 民联浏览器壳

`shell/` 目录结构:
```
shell/browser/                ← HTML/CSS/JS 裁决台 (零依赖)
  index.html                  ← 暗色三栏布局
  njsg-shell.js                ← 裁定引擎 JS + WASM 加载器 + UI 控制器
  shell.css                   ← CSS 变量主题

shell/native/                 ← 跨平台原生包装
  njsg_webview.py              ← Python pywebview (最简单)
  electron-shell/             ← Electron (exe/dmg/AppImage,体积大)
  tauri-shell/                ← Tauri (体积中等,最实用)
  build_wasm.sh               ← WASI SDK 编译脚本
```

**七元算法 JS 实现** 与 C 内核保持完全一致:
- `fiveYuanPush(state, angleInc, forceInc)` → 角度累加 wrap 360°, ticks>100 → ETERNAL
- `sixYuanVerdict(a, b)` → intersection=min(a,b), mirror=|a-b|<0.2, 双条件→TRUSTED
- `ShellNest.honeycomb(level)` → 6 子壳递归填充

**WASM 优先策略**: 
- 首次加载尝试 fetch('njsg_core.wasm')
- 可用则使用 WASI 接口
- 不可用则自动回退到 JS 实现 (算法等价)

**界面特色**:
- 旋钮滑块实时控制 (力度/角度 0-360)
- 六元镜像对齐可视化 (A/B 柱 + ALIGNED/OFFSET 标签)
- 七元蜂窝节点树 (level 层级 + spaceRatio)
- 裁定历史 (最多 50 条,左右色条区分 TRUSTED/REJECTED)
- 系统日志终端
- 一键预设 (ios/bank/android)

---

## 二、演化路径图

```
              v1.5                      v1.6                     v1.7
         ┌──────────┐            ┌──────────────┐         ┌──────────────────┐
         │ 中心化    │            │   生态扩展    │         │   去中心化        │
         │ 单一域名  │ ────────► │  合伙人子节点  │ ──────► │  WASM 本地运行   │
         │ CF Worker │            │  多域名部署    │         │  民联浏览器壳    │
         └──────────┘            └──────────────┘         └──────────────────┘
              │                        │                          │
              ▼                        ▼                          ▼
           lolict.workers.dev    合伙人自有域名A/B/C       离线 file://
           单点故障风险          域无关保证                  用户设备
                                资源可扩展                   零服务器
                                                         防审查/抗封锁
```

**演化原则**: 不是替换,而是叠加 — v1.5+v1.6+v1.7 同时运行,互相不冲突

---

## 三、关键技术决策记录 (ADR)

### ADR-001: WASI SDK vs 自定义 wasm32

**选择**: WASI SDK (标准方案)
**理由**: wasmtime/wasmer/WasmEdge 都支持,生态系统成熟;自定义 wasm32 需要重新实现 ABI,长期维护成本高

### ADR-002: JS 回退策略

**选择**: WASM 优先 + JS 自动回退
**理由**: 民联浏览器壳需在不可靠环境下运行(WebView 兼容性、离线场景);JS 实现保证"最坏情况下也能裁定";WASM 性能提升留给未来优化

### ADR-003: Tauri vs Electron

**选择**: 同时保留两个方案
**理由**: Electron 开发者熟悉度高,调试快;Tauri 体积小适合生产;不预判用户环境,两条路径都提供

### ADR-004: 盟机合 vs 联盟/合伙人

**选择**: 创建独立的"盟机合"词汇系统(67 术语)
**理由**: "联盟"偏政治化,"合伙人"偏工业化;两者都已有强联想;"盟机合"是中性新造词,准确表达"机器协同,盟约共守"

### ADR-005: 域无关作为核心约束

**选择**: "域无关"写入内核设计原则
**理由**: 绑定特定域名 = 丧失主权;囡囝共生裁定逻辑必须与域名完全解耦;同样的输入在 lolict.workers.dev / mqf.合伙人.com / file:///本地.html 输出完全一致

---

## 四、待解决问题 (未来)

1. **WASM 加载性能**: fetch + instantiate 在大型 .wasm 文件下可能有延迟
2. **离线状态下的持久化**: 裁定历史目前内存存储,需 localStorage/IndexedDB
3. **多节点同步**: 多个盟机合节点间的裁定结果一致性
4. **WASI 文件系统**: njsg_wasm.c 目前只支持 stdin/stdout,未来需要本地文件持久化
5. **移动壳细化**: iOS WKWebView 和 Android System WebView 的具体胶水代码
6. **WASM 签名验证**: 防止 njsg_core.wasm 被篡改 (代码签名或哈希校验)

---

## 五、文件索引

| 维度 | 文件 |
|------|------|
| 内核 C | `kernel/src/njsg_kernel.c` (三元~七元+全栈,3500+ 行) |
| WASM C | `kernel/wasm/njsg_wasm.c` (WASI 裁定核心,307 行) |
| Worker | `cloudflare/njsg-web.js` (Cloudflare Worker,970+ 行) |
| 浏览器壳 | `shell/browser/njsg-shell.js` (JS 引擎+UI,~470 行) |
| 术语集 | `docs/MENGJI_TERMS.md` |
| WASM 设计 | `docs/WASM_CORE_v17.md` |
| Python 壳 | `shell/native/njsg_webview.py` |
| Electron 壳 | `shell/native/electron-shell/main.js` |
| Tauri 壳 | `shell/native/tauri-shell/src-tauri/src/main.rs` |
| WASM 编译 | `shell/native/build_wasm.sh` |

---

## 六、当前稳定版本

```
NJSG_VERSION                 "1.6.0"   (kernel/src/njsg_kernel.c 第 79 行)
kernel ver (K.ver)             12      (kernel/src/njsg_kernel.c)
NJSG_WASM_VERSION            "1.7.0"   (kernel/wasm/njsg_wasm.c 第 19 行)
NJSG_SHELL_VERSION           "1.7.0"   (shell/browser/njsg-shell.js)
Worker version display      "v1.5"    (实际包含 v1.6-v1.7b 端点)
```

---

*文档: v1.8 里程碑总结*
*日期: 2026-08-05*
*维护者: lolict + CatPaw · 囡囝*
