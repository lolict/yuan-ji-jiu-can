# 满全法 · 执行内核路线图

> **服从于**: `SOVEREIGNTY.md`(主导权公约 v1.0)——所有代码设计的最高宪法
> 改动代码前先对照公约；违反公约的设计必须纠正

> 这份路线图放在仓库根目录，所有人可见，也是写代码的依据。
> 任何改动前先看这里，确认方向一致再动笔。

---

## 当前状态

**已完成**: 金壳封装 + 镜像代理 v0.7 (C 语言 + shell_header_t 格式 + 01夫妻边界审计)

- 位置: `kernel/src/mqf_kernel.c`
- 运行: `make` → `./kernel/src/mqf_kernel.elf`
- 功能:
  - **公理1**: 声母/韵母/声调/笔画 → auth_code,一切实体强制经编码洗礼
  - **公理2**: 镜像代理拦截外部功能调用,返回我们的规定裁判值
  - **公理3**: 金壳封装格式 `shell_header_t`,任何外部二进制必须套 MQ Shell Header
  - **公理4**: 锥份额 = cone_slots + cone_time_ms + mirror_depth,时间节奏由我们分配
  - **公理5**: `mqf_rule_enslave()`,外部规则经 MQ 通道 transmute 为受控碎片

## 已完成版本

### v0.1 身份编码 + 守门人
- 声母/韵母/声调/笔画 → auth_code → capability → exec slot

### v0.2 契约系统
- 规则驱动权限判定(不再是硬编码 `%7`)
- 事实库 + 规则库,增删改无需重编译 C

### v0.3 时间切片 + 看门狗
- 每个执行槽有限定窗口(3s/10s/∞)
- 看门狗线程 period=1s 轮询,超时 → EXEC_SLEEP

### v0.4 WASM 执行槽 (Zig)
- `kernel/wasm/runner.zig`: Zig 加载 WASM 字节码,解析 export,i32 栈机 + 128KB 内存
- Host Bridge: log/getAuth/time/kernelCall/yield ↔ 看门狗协作

### v0.5 空间关系代数
- 对象/关系 CRUD、路径可达(BFS)、间接力(DP)、内镜像(分层展开)、态势判决

### v0.6 反客为主规则引擎
- 内置 Prolog 子集解释器: fact 库 + rule 库 + resolve/arbitrate
- `mqf_rule_enslave()` 函数: 外部规则 → transmute → 受控 ENSLAVED 碎片
- 身份让渡: 外部规则身份被 mqf_encode 重新颁发,我们成为其"主人"
- 锥份额裁决: ENSLAVED 规则默认 priority=0(我们裁判者授权才升高)
- 审计新增 RULE_ENSLAVE/RULE_ACTIVATE/RULE_REJECT 事件类型

### v0.7 金壳封装 + 镜像代理 ← 当前
- `shell_header_t` 格式: 128 bytes 壳头包裹外部二进制(ELF/PE/WASM/...)
- `mqf_shell_wrap()`: 强制颁发 MQ 身份 + 锥份额(slots+time+mirror_depth)
- `mqf_shell_mirror_proxy()`: 拦截 get_balance/get_uid 等外部敏感功能
- `mqf_shell_boundary_audit()`: 01夫妻边界(内=1/外=0)每次穿越都被审计
- 新增审计事件: SHELL_WRAP/SHELL_BOUNDARY/SHELL_ACTIVATE/MIRROR_PROXY
- 六大场景演示:裸二进制→拒绝 → 包壳→颁发身份 → 镜像拦截 → 01边界审计

---

## 下一步(按顺序)

### v0.8 统计锥引擎 (C)
**目标**: 把 v0.5 的空间代数升级为**公理4(统计锥切片)**完整实现

- 锥形态: 时间份额 + 空间份额 + 镜像级数 + 合规度权重
- 锥调度: 按合规度动态调整份额(observe_ratio 高的多给)
- 锥快照: 定时拍照全图状态,回溯历史态势

---

## 技术栈

- 内核: C ↔ Zig (小、快、可控)
- 规则引擎: Prolog 子集 (声明式契约)
- 多语言前端 → WASM (通用字节码)
- 空间/关系代数: 自定义 + C 实现 (线程安全)
- 打包: Tauri/PWA

## 规则

1. 每次实现一个版本，跑通 → 提交 → 推送到 GitHub `lolict/hunyuan-os`
2. 不在已规划之外加"临时功能"
3. 改动这份路线图必须经过讨论
4. 代码失败/方向错 → 修改计划并记录原因，不绕路走

---

*开始日期: 2026-08-04*
*维护者: lolict + CatPaw*
