# 满全法 · 执行内核路线图

> **服从于**: `SOVEREIGNTY.md`(主导权公约 v1.0)——所有代码设计的最高宪法
> 改动代码前先对照公约；违反公约的设计必须纠正

> 这份路线图放在仓库根目录，所有人可见，也是写代码的依据。
> 任何改动前先看这里，确认方向一致再动笔。

---

## 当前状态

**已完成**: 空间关系代数 v0.5 (C 语言 + pthread)

- 位置: `kernel/src/mqf_kernel.c`
- 运行: `clang kernel/src/mqf_kernel.c -pthread -o /tmp/mqf -lm && /tmp/mqf`
- 功能:
  - **公理1 identity_enslave**: 声母/韵母/声调/笔画 → auth_code → capability → exec slot
  - **公理4 锥份额**: 看门狗 + 执行槽的时间切片
  - **公理2**: 路径可达(BFS) + 间接力(DP) + 内镜像(分层展开) → 镜像代理的基础

- 宪法: `SOVEREIGNTY.md`(主导权公约 v1.0)已批准并成为所有后续版本的指导

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

### v0.6 反客为主规则引擎 (C + Prolog 子集)
**目标**: 把契约系统(v0.2)升级为可外部加载的声明式规则引擎,实现**公理5(反客为主)**

核心:
- Prolog 子集解释器(内置在 C 运行时,无外部依赖)
- 规则文件 `.pl` 可热加载,增删改无需重编译内核
- **公理5的实现**: 任何外部"规则"被送入系统时,先被我们的裁判通道解析、降级为"被调动者",再按我们的锥份额运行
- **公理1的强化**: identity_enslave() 默认启用,任何进入的实体**强制**接受 MQF 编码
- **准备公理2**: 规则可定义"镜像策略"——裁判者看到外部请求后,可返回替代响应

数据结构:
```c
typedef struct {
    char name[128];        // 规则名
    char pattern[256];     // 匹配模式(类似 Prolog head)
    char action[256];      // 执行动作(Prolog body)
    int priority;          // 优先级(高优先覆盖低优先)
    int loaded_from_file;  // 标记是外部加载还是内置
} rule_t;

// 外部规则进入时先走此函数 → "反客为主"
int rule_enslave(const char* external_rule, rule_t* out);
```

### v0.7 金壳封装格式 (C + Shell Header)
**目标**: 实现**公理3(集装箱封装)**——任何外部二进制先套上我们的壳

- Shell Header 定义: identity/CID, 权限/锥份额, 镜像映射, 01夫妻边界标签
- 壳加载器: 解析壳头 → 内部原二进制送裁判 → 根据锥份额决定是否激活
- 01夫妻边界: 每跳 cross-boundary 行为都要被审计和裁定

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
