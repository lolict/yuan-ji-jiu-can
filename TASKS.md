# 囡囝共生 · 任务规划(连贯性节点表)
> ManQuanFa · Task Planning · Coherent Reads

## 怎样用这份文档

新会话打开的第一份参考文档就是它。  
读取顺序:**时间节点** → **空间节点** → **当前状态** → **下一步动作**。  
不需要重复已落地的版本,只需从"当前状态"继续。

---

## 时间轴(版本里程碑)

```
v0.1 ── v0.2 ── v0.3 ── v0.4 ── v0.5 ── v0.6 ── v0.7 ── v0.8 ── v0.9 ── v1.0 ── v1.1 ── v1.2 ── v1.3 ── v1.4 ── v1.5 ── v1.6 ── v1.7
 │        │        │        │        │        │        │        │        │        │        │        │        │        │        │         │         │
 编码    契约    看门狗   WASM_v0 空间代数 反客为主 金壳镜像 统计锥  四元面  五元旋钮  六元对称  蜂窝套壳  全栈联动  生态API  对话渠道  盟机合子节点  WASM内核+壳
 完成    完成    完成    完成    完成    完成    完成    完成    完成    完成     完成     完成     完成      完成      完成        完成          进行中
```

### 已交付版本详细说明

| 版本 | 时间 | 核心产出 | 入口文件 | 验证方式 |
|---|---|---|---|---|
| v0.1 | 2026-08-04 | 身份编码 + 守门人 | `kernel/src/njsg_kernel.c` | `njsg_encode("ios")` |
| v0.2 | 2026-08-04 | 契约引擎(规则驱动) | `njsg_contract_add(...)` | main() 场景 A |
| v0.3 | 2026-08-04 | 看门狗(pthread) | `njsg_watchdog()` | 场景超时自动 EXEC_SLEEP |
| v0.4 | 2026-08-04 | WASM 执行槽(Zig) | `kernel/wasm/runner.zig` | `zig run runner.zig` |
| v0.5 | 2026-08-04 | 空间代数(BFS/DP) | `njsg_reachable/inner_mirror` | 场景 K1-K4 |
| v0.6 | 2026-08-05 | 反客为主(rule_enslave) | `njsg_rule_enslave(...)` | 场景 A-F |
| v0.7 | 2026-08-05 | 金壳封装+镜像代理 | `njsg_shell_wrap/proxy` | 场景 G-J |
| v0.8 | 2026-08-05 | 统计锥(cone_share/cone_tick) | `njsg_cone_*/compliance` | 场景 K1-K4 |
| v0.9 | 2026-08-05 | 四元面判断(域+宏微量) | `njsg_domain_*/macronein` | 场景 L1-L3 |

---

## 文件系统空间节点

```
hunyuan-os/
├── TASKS.md                    ← 入口:本文件(新会话读这里)
├── README.md                   ← 项目概览
├── SOVEREIGNTY.md              ← 主导权公约 v1.0(宪法)
├── ROADMAP.md                  ← 技术路线图
├── Dockerfile                  ← Docker 容器
├── docker-compose.yml          ← 一键部署
├── docker-entrypoint.sh        ← 容器入口
├── Makefile                    ← 构建系统(kernel+docs)
│
├── kernel/
│   └── src/njsg_kernel.c        ← ★ 核心文件,所有版本代码都在这(3500+ 行)
│   └── wasm/njsg_wasm.c         ← WASI 裁决内核 v1.7 (307 行)
│   └── wasm/runner.zig         ← Zig WASM 加载器
│
├── cloudflare/
│   └── mqf-web.js              ← Cloudflare Worker + 盟机合注册 (970+ 行)
│
├── shell/                        ← 民联浏览器壳 v1.7c
│   └── browser/                  ← HTML+CSS+JS 裁决台
│   └── native/                   ← Electron/Tauri/pywebview 原生包装
│
├── android/                    ← Android parasitate 辅助服务代码
│   └── src/com/mqf/parasite/   ← 6 个文件:IME/Alarm/Boot/HiddenFS/CoverApp/Accessibility
│
├── docs/
│   ├── MENGJI_TERMS.md         ← 盟机合 67 术语体系
│   ├── WASM_CORE_v17.md        ← WASM 内核技术设计
│   ├── CAST.md                 ← 三元裁判者人格宪法
│   ├── WHITEPAPER.md           ← 对外白皮书
│   └── ARCHITECTURE.md         ← 架构细节
│
├── deploy/
│   ├── FEISHU.md               ← 飞书 Bot 三步对接指南
│   └── MIGRATION_GUIDE.md      ← v0.x→v0.9 迁移+兼容性
│
└── encoding/                   ← 神龙七层编码参考
    └── mccp.md
```

---

## 新会话的读取顺序(必读)

1. **`TASKS.md`** — 当前文件,知道时间轴在哪
2. **`ROADMAP.md`** — 看当前版本与下一步
3. **`SOVEREIGNTY.md`** — 5 条公理,所有规则的根基
4. **`kernel/src/njsg_kernel.c`** — 核心代码,从顶部注释即可确认当前版本

**可选读(按需):**
- 要接飞书 → 读 `deploy/FEISHU.md`
- 要部署容器 → 读 `Dockerfile` + `deploy/MIGRATION_GUIDE.md`
- 要对外讲项目 → 读 `docs/WHITEPAPER.md`
- 要定义谁在说话 → 读 `docs/CAST.md`

---

## 当前状态 (2026-08-05 — v2.0 + 阶梯制双向公约)

### 已落地(无需再讨论)
- v0.1-v1.3 全部完成 (三元→七元+全栈联动)
- v1.4-v1.6 生态 API + 对话渠道 + 盟机合
- v1.7 WASM 内核 + 民联浏览器壳
- v1.8-v1.12 术语/移动端/同步/存储/Beta 发布
- v2.0 公开 Beta 发布包 (release/v2.0-beta/)
- v2.0-ladder 阶梯制·双向公约 (2026-08-05)
  - kernel/wasm/mqf_wasm.c 去 32/64 假设:int32_t/size_t/NJSG_* 宏
  - shell/native/build_wasm.sh 单源码双出 (wasm32 + wasm64)
  - docs/DUAL-BINDING-TIER.md 阶梯制文档
  - 双阶梯语法检查通过 (wasm32-wasi ✅, wasm64-wasi ✅)

### 代码文件
- `kernel/wasm/mqf_wasm.c` (WASI 裁定内核,v2.0-ladder,320 行,32/64 同源码)
- `kernel/src/mqf_kernel.c` (本地核心,3500+ 行)
- `cloudflare/mqf-web.js` (Worker + 盟机合 + 同步端点,970+ 行)
- `cloudflare/sync-protocol.js` (多节点同步引擎,417 行)
- `shell/browser/` (民联浏览器壳 v2.0 — HTML+CSS+JS,5 文件 + njsg_core.wasm)
- `shell/native/` (build_wasm.sh + wasm-includes shim + 多平台)

### Demo 验证方式
```bash
make clean && make && make test
```
输出 12 个场景(A-L)的全部审计日志。

---

## 下一步动作(按优先级)

### 优先级 1:v1.0 五元旋钮运动步量
**工程目标**:
- `njsg_five_yuan_push(node, step, angle, force)`:5 步位移累加
- 角度 wrap:0~360°,每步累加并对 360 取模
- 注意力配比:`ratio_in`(进入这个节点的边权重和) vs `ratio_out`(离开的边权重和)
- 占存判定:节点的 accumulated_ticks > 阈值 → "占存"(铭记),否则不占存(可被遗忘)

**入口函数提议**:
```c
typedef struct {
    char name[MAX_NAME_LEN];
    int step;            // 0~4 (五次运动)
    float angle;         // 0.0~360.0
    float force;         // 作用力(= weight 累积)
    float ratio_in;      // 进入配比
    float ratio_out;     // 离开配比
    int   occupied;      // 占存(1) / 不占存(0)
} five_yuan_state_t;

void njsg_five_yuan_init(void);
void njsg_five_yuan_push(int obj_id, int step, float angle_inc, float force);
void njsg_five_yuan_print(void);
```

### 优先级 2:v1.1 六元对称群+六面体
**工程目标**:
- 两个三元裁判背靠背镜像对齐
- 六面三维可视化(每个面一个视角)
- 信任交集判定 vs 不可信任无交集判定

### 优先级 3:v1.2 七元蜂窝套壳
**工程目标**:
- 一套"壳中壳"递归嵌套结构
- 容器-in-容器,无限层级
- 空间填充规则(蜂窝密铺)

---

## 读取与写入规则

1. 每次新会话**先读 ROADMAP.md 顶部**确认当前版本号
2. **任何改动前先读 SOVEREIGNTY.md**确认符合公理
3. **所有规则层改动在 `njsg_kernel.c`** 单文件完成(保持架构一致)
4. **新增文档放 `docs/` 或 `deploy/`**,保持仓库整洁
5. **完成后更新本文档的"当前状态"一节**
6. **每次版本提交更新 ROADMAP.md 的已完成列表**

---

*维护者:lolict + CatPaw · 囡囝*
*最后更新:2026-08-05*
*当前稳定版本:v1.3*
