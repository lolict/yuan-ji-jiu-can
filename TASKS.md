# 囡囝共生 · 任务规划(连贯性节点表)
> YinYangOS · Task Planning · Coherent Reads

## 怎样用这份文档

新会话打开的第一份参考文档就是它。  
读取顺序:**时间节点** → **空间节点** → **当前状态** → **下一步动作**。  
不需要重复已落地的版本,只需从"当前状态"继续。

---

## 时间轴(版本里程碑)

```
v0.1 ── v0.2 ── v0.3 ── v0.4 ── v0.5 ── v0.6 ── v0.7 ── v0.8 ── v0.9 ── v1.0 ── v1.1 ── v1.2 ── v1.3
 │        │        │        │        │        │        │        │        │        │        │        │        │
 编码    契约    看门狗   WASM_v0 空间代数 反客为主 金壳镜像 统计锥  四元面  五元旋钮  六元对称  蜂窝套壳  全栈联动
 完成    完成    完成    完成    完成    完成    完成    完成    完成    完成     完成     完成     完成

v1.4 ── v1.5 ── v1.6 ── v1.7 ── v2.0 ── v2.1+ ──────────────────────────────────────────────────────────►
 │        │        │        │        │        └─ 44工具·三内核·全栈联动
 天觉     生殖    审美    WASM内核   公开Beta    当前
```

### 已交付版本详细说明

| 版本 | 时间 | 核心产出 | 入口文件 | 验证方式 |
|---|---|---|---|---|
| v0.1 | 2026-08-04 | 身份编码 + 守门人 | `kernel/src/njsg_kernel.c` | `njsg_encode("ios")` |
| v0.2 | 2026-08-04 | 契约引擎(规则驱动) | `njsg_contract_add(...)` | main() 场景 A |
| v0.3 | 2026-08-04 | 看门狗(pthread) | `njsg_watchdog()` | 场景超时自动 EXEC_SLEEP |
| v0.4 | 2026-08-04 | WASM 执行槽(Zig) | `kernel/wasm/runner.zig` | `zig run runner.zig` |
| v0.5 | 2026-08-05 | 空间代数(BFS/DP) | `njsg_reachable/inner_mirror` | 场景 K1-K4 |
| v0.6 | 2026-08-05 | 反客为主(rule_enslave) | `njsg_rule_enslave(...)` | 场景 A-F |
| v0.7 | 2026-08-05 | 金壳封装+镜像代理 | `njsg_shell_wrap/proxy` | 场景 G-J |
| v0.8 | 2026-08-05 | 统计锥(cone_share/cone_tick) | `njsg_cone_*/compliance` | 场景 K1-K4 |
| v0.9 | 2026-08-05 | 四元面判断(域+宏微量) | `njsg_domain_*/macronein` | 场景 L1-L3 |
| v1.0-v1.3 | 2026-08-05 | 五元→七元+全栈联动 | 同上 | 场景 M1-M4 |
| v1.4-v1.6 | 2026-08-22 | 天觉+生殖+审美治理 | `kernel/src/njsg_kernel.c` | 6 场景 demo |
| v1.7 | 2026-08-23 | WASM 内核+阶梯制·双向公约 | `kernel/wasm/njsg_wasm.c` | 双阶梯语法检查通过 |
| v2.0 | 2026-08-24 | 公开 Beta 发布 | `kernel/src/njsg_kernel.c` (4524行) | 编译零错误·三平台同步 |
| v2.1+ | 2026-08-31 | 44 工具·三内核·全自动后勤 | `GLOBAL_DESTINATION.md` | cargo test 1019/1019 |

---

## 文件系统空间节点

```
hunyuan-os/
├── TASKS.md                    ← 入口:本文件(新会话读这里)
├── GLOBAL_DESTINATION.md       ← 工具清单 44 个·当前状态
├── SYMBOL_DICTIONARY.md        ← 自定义符号字典
├── CONTAINER_TOPOLOGY.md       ← 容器拓扑关系
├── SOVEREIGNTY.md              ← 主导权公约(宪法)
├── GLOBAL_PROMPT_V2.md         ← 全局提示词·容器处理器协议
├── GLOBAL_CHAIN_MASTER_LOCK_V2.md ← 总开关·互锁链条
├── GLOBAL_INTERLOCK_NEURAL_NETWORK.md ← 神经网络控制·分布式裁决
├── Makefile                    ← 构建系统(kernel+docs)
│
├── kernel/
│   └── src/njsg_kernel.c        ← ★ C 核心文件(4524行·20+场景)
│   └── wasm/njsg_wasm.c         ← WASI 裁决内核·阶梯制·双向公约 (307 行)
│   └── wasm/runner.zig         ← Zig WASM 加载器
│
├── hunyuan-rs/                  ← Rust 核心 (64文件·1019测试·v0.31)
│   └── src/lib.rs
│
├── tools/                       ← 44 个 Nim·Zig·Python 工具
│   ├── mlll-254/               ← 集装箱编码
│   ├── consciousness/          ← 自举意识·固定点
│   ├── soul-kernel/            ← 灵魂内核·情感系统
│   ├── heaven-space/           ← 天庭空间·四模型复合
│   ├── geo-3d/                 ← 三维几何可视化
│   ├── proof-engine/           ← 形式化自举证明
│   ├── high-algebra/           ← 高维代数·偏移率体系
│   ├── mltt-cipher/            ← 命名密码表
│   ├── phonetic-encoding/      ← 语音符号编码·主人全息
│   ... (共 44 个)
│
├── cloudflare/
│   └── njsg-web.js              ← Cloudflare Worker + 盟机合注册
│
├── shell/                        ← 民联浏览器壳
│   └── browser/                  ← HTML+CSS+JS 裁决台
│
├── docs/                        ← 白皮书/架构/盟机合术语
└── deploy/                      ← 飞书对接/迁移指南
```

---

## 引擎内核清单

| 内核 | 版本 | 状态 | 路径 |
|------|------|------|------|
| C 内核 | v2.0.0 | ✅ 4524行·20+场景·编译零错误 | `kernel/src/njsg_kernel.c` |
| Rust 核心 | v0.31 | ✅ 64文件·1019测试 | `hunyuan-rs/` |
| WASM 核心 | v1.7 | ✅ 单源wasm32+wasm64·阶梯制 | `kernel/wasm/njsg_wasm.c` |

---

## 工具清单 (44个)

详见 [GLOBAL_DESTINATION.md](./GLOBAL_DESTINATION.md)

---

## 下一步动作(按优先级)

### 优先级 1: 全量 Nim 编译验证
- 在所有工具目录下运行 `nim c -r` 确认通过
- 修复任何 API 漂移问题

### 优先级 2: 蚕蛹反向寄生壳·真实平台适配
- 完成脱壳穿衣流程的端到端测试
- 接入 Android/iOS/Windows/macOS/Linux 真实寄生目标

### 优先级 3: 统计文档聚合浏览器·真实数据输入
- 接入任意规模的 JSON 数据源
- 验证聚类包裹树在大数据量下的性能

---

## 三线同步

| 平台 | 仓库 | 状态 |
|------|------|------|
| GitHub | `github.com/lolict/hunyuan-os` | ✅ 主仓 |
| Gitee | `gitee.com/lolict/hunyuan-os` | ✅ 镜像 |
| Cloudflare | Workers + Pages | ✅ 部署中 |

---

## 读取与写入规则

1. 每次新会话**先读 GLOBAL_DESTINATION.md**确认工具清单
2. **任何改动前先读 SOVEREIGNTY.md**确认符合公理
3. **所有规则层改动在 `njsg_kernel.c`** 单文件完成(保持架构一致)
4. **新增工具放 `tools/`** 并登记到 `GLOBAL_DESTINATION.md`
5. **完成后更新本文档的"当前状态"一节**
6. **每次版本提交同步 GitHub + Gitee**

---

*维护者:lolict + CatPaw · 囡囝*
*最后更新:2026-09-01*
*当前稳定版本:v2.1+ (44工具·三内核·全自动后勤)*
