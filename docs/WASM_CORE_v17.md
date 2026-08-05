# v1.7 WASM 内核 · 技术设计

> 把囡囝共生七元裁定引擎移植到 WASM,运行在独立壳(民联浏览器)中

---

## 一、目标

```
让裁定引擎脱离 Cloudflare Workers,脱离任何中心化平台
运行在用户自己的设备上(WASM + 独立壳)
```

---

## 二、技术路径

```
┌─────────────────────────────────────────────────────────┐
│  民联浏览器壳                                             │
│  ┌───────────────────────────────────────────────────┐  │
│  │  WASI 运行时(wasmtime / wasmer / WasmEdge)        │  │
│  │  ┌─────────────────────────────────────────────┐  │  │
│  │  │  njsg_core.wasm                              │  │  │
│  │  │  · 三元:编码 + 规则引擎                      │  │  │
│  │  │  · 四元:拓扑域 + 跨域裁定                    │  │  │
│  │  │  · 五元:旋钮步量 + 占存判定                  │  │  │
│  │  │  · 六元:对称群 + 信任交集                    │  │  │
│  │  │  · 七元:蜂窝套壳 + 递归嵌套                  │  │  │
│  │  │  输入:JSON → 裁定请求                        │  │  │
│  │  │  输出:JSON → 裁定结果                        │  │  │
│  │  └─────────────────────────────────────────────┘  │  │
│  │  本地文件系统 · 本地数据库 · 离线运行              │  │
│  └───────────────────────────────────────────────────┘  │
│  跨平台:Windows / macOS / Linux / Android / iOS         │
└─────────────────────────────────────────────────────────┘
```

---

## 三、实现步骤

### Step 1: 提取核心算法 → njsg_wasm.c

从 `njsg_kernel.c` 中提取裁定相关函数:

```c
/* 不依赖 pthread 的裁定核心 */
typedef struct { ... } five_yuan_state_t;
typedef struct { ... } six_yuan_pair_t;
typedef struct { ... } shell_nest_t;

void njsg_wasm_init(void);
int  njsg_wasm_verdict(const char* entity, float a, float b, float force, float angle);
int  njsg_wasm_five_yuan_push(int id, float angle, float force);
int  njsg_wasm_six_yuan_trust(int id, float a, float b);
int  njsg_wasm_shell_nest_wrap(const char* name);
```

### Step 2: WASI 接口层

```c
/* WASI 的 fd_read / fd_write */
#include <wasi/libc.h>

/* 标准 JSON 输入输出 */
int main() {
    char input[4096];
    char output[4096];
    
    /* 从 stdin 读取裁定请求 JSON */
    int len = wasi_fd_read(0, input, sizeof(input));
    
    /* 解析 JSON → 调用裁定引擎 */
    njsg_verdict_t result = njsg_wasm_verdict(input);
    
    /* 输出结果 JSON 到 stdout */
    wasi_fd_write(1, result_json, strlen(result_json));
    
    return 0;
}
```

### Step 3: 编译命令

```bash
# 使用 WASI SDK
/opt/wasi-sdk/bin/clang \
  --target=wasm32-wasi \
  -O2 \
  -z stack-size=1048576 \
  -Wl,--initial-memory=65536 \
  -o njsg_core.wasm \
  njsg_wasm.c
```

### Step 4: 民联浏览器壳

| 平台 | 壳方案 |
|------|--------|
| Windows | WebView2 (Edge Chromium 独立包) |
| macOS | WKWebView (Safari 独立包) |
| Linux | WebKitGTK |
| Android | Android System WebView |
| iOS | WKWebView |

壳的职责:
- 加载并运行 njsg_core.wasm
- 提供本地文件系统访问
- 提供 UI(可选,可以用命令行模式)
- 网络同步(可选)

---

## 四、场域接入模型

```
┌──────────────────────────────────────────────────────────┐
│  主场(lolict.workers.dev)                                 │
│  职责:发布规则 + 签发身份 + 维护盟机合目录                │
└──────────────────────────────────────────────────────────┘
                           ▲
                           │ 同步裁定规则(可选)
        ┌──────────────────┼──────────────────┐
        ▼                  ▼                  ▼
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│  盟机场 A    │  │  萌机场 B    │  │  WASM 场 C   │
│  Worker 节点 │  │  免费节点    │  │  民联浏览器  │
│  自有域名    │  │  萌机合长    │  │  离线运行    │
└──────────────┘  └──────────────┘  └──────────────┘
```

WASM 节点的特点:
- 完全离线可用
- 不需要服务器
- 不依赖任何平台
- 裁定规则从主场同步(可选)

---

## 五、域名无关化

WASM 核心的裁定结果与域名无关:

```
输入: { entity: "ios", compliance_a: 0.9, compliance_b: 0.85 }
输出: { verdict: "TRUSTED", confidence: 0.85 }

无论运行在哪里(Workers/WASM/独立服务器),输出完全一致
```

---

*v1.7 · 规划中 · 2026-08-05 · 囡囝*
