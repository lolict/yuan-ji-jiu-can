# 满全法 · 执行内核路线图

> 这份路线图放在仓库根目录，所有人可见，也是写代码的依据。
> 任何改动前先看这里，确认方向一致再动笔。

---

## 当前状态

**已完成**: 空间关系代数 v0.5 (C 语言 + pthread)

- 位置: `kernel/src/mqf_kernel.c`
- 运行: `clang kernel/src/mqf_kernel.c -pthread -o /tmp/mqf -lm && /tmp/mqf`
- 功能:
  - 身份编码 → 权限分级(0僵尸/1-2只读/3-4读写执行/5-6全权限)
  - 执行守门 → 审计环
  - 时间切片 + 看门狗 (v0.3)
  - **空间关系代数**: 对象/关系 CRUD、路径可达(BFS)、间接力(hop步累积)、内镜像(depth层)、态势判决(稳态/渐变态/逆变态)

---

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
- `mqf_spatial_init / mqf_obj_create / mqf_rel_add`
- `mqf_reachable(from, to): bool` (BFS)
- `mqf_indirect_force(from, to, hops): float` (DP 累积权重)
- `mqf_inner_mirror(center, depth, layers, count)` (按层展开)
- `mqf_assess(): situation_t` (稳态/渐变态/逆变态)
- 线程安全: 单 mutex 保护空间引擎

---

## 下一步(按顺序)

### v0.6 Prolog 声明式规则导出 (C + Plog 子集)
**目标**: 把契约规则(第2层)改成可从外部加载的声明式脚本
- 支持: `grant_level(X, L) :- has_auth(X), trusted(X).`
- 解释器内置在 C 运行时,无需安装外部 Prolog

### v0.7 安卓打包 (Tauri / PWA)
**目标**: 满全法内核打包成安卓 APK，自带运行时、独立公网
- 方案A: Tauri (Rust + WebView, ~10MB)
- 方案B: PWA (纯前端，浏览器内即应用)
- 所有判定在本地，无任何云端回传

### v0.8 空间关系代数 ZooKeeper
**目标**: 图结构持久化 + 变动事件订阅
- 关系变动 → 事件流 → 订阅者(信用分/态势告警)

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
