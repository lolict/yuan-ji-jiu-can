# ADR 9 — 蚕蛹反向寄生壳 · 跨平台部署引擎

## 状态

接受 (Accepted)

## 日期

2026-09-12

## 背景

满全法·囡囝共生 (NJSG) 操作系统的核心命题之一: "规则不应适应外部世界, 而是让外部世界模仿规则"。
蚕蛹反向寄生壳架构就是这一命题的工程实现。

传统寄生模型: 寄生虫改变自身以适应宿主 (失去自我)。
反向寄生模型: 让宿主环境承载寄生虫的规则编码 (保持自我)。

## 决策

### 核心概念

```
蚕蛹包 (CocoonPackage)  →  初始下载单元
  ├── parsers: 各平台解析器清单
  ├── shell_options: 可注入的壳选项
  └── kernel_digest: 内核摘要 (不可变)

寄生壳 (ShellKind)  →  目标平台标识
  ├── Cargo (Rust 规则寄生)
  ├── npm (JavaScript 流式寄生)
  ├── PyPI (Python hook 寄生)
  ├── Docker (镜像层寄生)
  ├── WASM (WASI 接口寄生)
  ├── ELF (.so 动态库寄生)
  └── APK (Android 应用壳寄生)
```

### 四阶段生命周期

| 阶段 | 名 | 操作 | 输出 |
|------|-----|------|------|
| ① 生 | spawn | 离线生成蚕蛹包 | parsers + shells + digest |
| ② 长 | grow | 分析目标 → 选定壳 | ShellKind + parser |
| ③ 收 | harvest | 脱壳穿衣 → 生成配置 | DressingConfig + mount_path |
| ④ 藏 | hide | 验证 → 隐形评分 | HideReport + stealth_score |

### 目标平台分析

通过文件存在度和匹配模式推断壳类型:

```
Cargo:  Cargo.toml + Cargo.lock + src/*.rs  → confidence 75%+
npm:    package.json + node_modules/         → confidence 100%
PyPI:   pyproject.toml + setup.py            → confidence ~80%
Docker: Dockerfile + docker-compose.yml      → confidence ~80%
WASM:   pkg/ + wasm-pack configuration       → confidence ~70%
ELF:    Makefile + *.so                      → confidence ~60%
APK:    AndroidManifest.xml + build.gradle   → confidence ~75%
```

### 脱壳穿衣

根据选定壳类型, 生成对应结构的注入文件 + 入口点声明。所有注入路径以 `.njsg/` 前缀隐藏:

```
Cargo:      Cargo.toml / src/lib.rs / .njsg/digest.bin
npm:        package.json / index.js / .njsg/internal.mjs
PyPI:       pyproject.toml / njsg_core/__init__.py / .njsg/_internal.pyd
Docker:     Dockerfile / .njsg/shim.sh
WASM:       pkg/*.wasm / pkg/*.d.ts
ELF:        libnjsg.so / .njsg/elf_tramp.S
APK:        njsm.kotlin / .njsg/dex_shim.dex
```

### 隐形验证

stealth_score = base(0.5) + hidden_dir(0.2) - file_count × 0.05 - translation_penalty(0.1) + strategy_bonus(0.05~0.2)

| Score | Status | 含义 |
|-------|--------|------|
| >80% | Invisible | 完全隐形 |
| 50~80% | Blended | 融入环境 |
| <50% | Detectable | 可被检测 |

## 验证

```
ttf-parasite (demo mode):
  🦀 cargo project (confidence=75%)
  → Grow → Cargo selected
  → Harvest: Cargo.toml / src/lib.rs / .njsg/digest.bin
  → Hide: score=75% status=Blended

ttf-parasite (npm target):
  📦 npm project (confidence=100%)
  → Grow → Npm selected (requires translation)
  → Harvest: package.json / index.js / .njsg/internal.mjs
  → Hide: score=60% status=Blended (translation penalty)
```

## 后续方向

- 离线 cocoon 包升级为网络下载
- 交互壳选择界面 (手动 override 自动匹配)
- 实际代码注入 (生成真的 Cargo.toml / package.json)
- 加壳倒计时: 对外暴露的时间受控 (定时解密)
- 多宿主协同寄生
