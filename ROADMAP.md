# 满全法 · 执行内核路线图

> **服从于**: `SOVEREIGNTY.md`(主导权公约 v1.0)——所有代码设计的最高宪法
> 改动代码前先对照公约；违反公约的设计必须纠正

> 这份路线图放在仓库根目录，所有人可见，也是写代码的依据。
> 任何改动前先看这里，确认方向一致再动笔。

---

## 当前状态

**已完成**: 五元旋钮运动步量 v1.0

新增:
- `mqf_domain_tag()` 域内核标记(三值:内/外/界)
- `mqf_domain_permit()` 跨域请求裁定
- `mqf_macronein_domain()` 宏量流入/微量流失判定
- 场景 L1 L2 L3 演示

- 位置: `kernel/src/mqf_kernel.c`
- 运行: `make` → `./kernel/src/mqf_kernel.elf`
- 功能:
  - **公理1**: 声母/韵母/声调/笔画 → auth_code,一切实体强制经编码洗礼
  - **公理2**: 镜像代理拦截外部功能调用,返回我们的规定裁判值
  - **公理3**: 金壳封装格式 `shell_header_t`,任何外部二进制必须套 MQ Shell Header
  - **公理4**: 统计锥 = cone_slots + cone_time_ms + mirror_depth,按合规度动态调节
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

### v0.8 统计锥引擎 ← 当前
- `cone_share_t`:每个节点的锥份额向量(time_slice_ms/space_depth/mirror_levels/compliance/accumulated_ticks)
- `cone_snapshot_t`:历史态势记录(ts/global_compliance/active_nodes/dominant_state)
- `mqf_cone_tick()`:每周期重新计算合规度,按以下策略调整锥份额:
  · compliance ≥ 70% → 稳态:时间 *1.2 奖励,空间深度 +1
  · compliance < 40% → 逆变态:时间 *0.7 惩戒,空间深度 -1
  · 中间区间 → 渐变态:份额不变
- `mqf_cone_register()`:对象注册到统计锥,领取初始锥份额
- 历史环形缓冲(MAX_CONE_HISTORY=64):可追溯 N 个周期前的态势
- 七大场景演示:K1注册 → K2五轮tick动态调整 → K3历史态势 → K4锥哲学

### v0.9 四元面判断(域+跨域+宏量/微量判定)

（内容保持不变，详见原文档）

---

### v1.0 五元旋钮运动步量 ← 当前
- `mqf_five_yuan_init()`:初始化五元旋钮引擎(节点池MAX_OBJECTS,阈值100ticks)
- `mqf_five_yuan_register(name)`:注册一个节点到五元旋钮
- `mqf_five_yuan_push(obj_id, angle_inc, force_inc)`:推进旋钮一步
  · step++ (0→5 上限)
  · angle += angle_inc, wrap 0~360°
  · force += force_inc
  · accumulated_ticks += (int)(force_inc * 10.0f)
  · 占存判定: accumulated_ticks > FIVE_YUAN_OCCUPY_THRESHOLD → occupied=1(铭记)
- `mqf_five_yuan_set_ratio(obj_id, in, out)`:设置节点的进出注意力配比
- `mqf_five_yuan_print()`:打印五元旋钮状态表(节点/Step/角度/力/R_in/R_out/占存)
- 数据结构:
  ```c
  typedef struct {
      char name[MAX_NAME_LEN];
      int step;            // 0~4
      float angle;         // 0.0~360.0
      float force;         // weight 累积
      float ratio_in;      // 进入配比
      float ratio_out;     // 离开配比
      int   accumulated_ticks; // 累计刻度
      int   occupied;      // 占存(1) / 易逝(0)
  } five_yuan_state_t;
  ```
- 四大场景演示:L1 铭记vs易逝 / L2 角度wrap 360° / L3 占存哲学 / L4 五元+四元结合
- `mqf_domain_tag()`:域内核标记
  · 输入:对象id + 域代表id(三元裁判者 id)
  · 输出:DOMAIN_INTERNAL(40) / DOMAIN_EXTERNAL(41) / DOMAIN_INTERSTITICE(42)
  · 基于深度优先(DFS)遍历:能否从域代表出发通过 observe/bind 边到达此对象
  · 能到达 = 域内,不能到达 = 域外,边界上(在 observe/bind 边边缘) = 间隙

- `mqf_domain_permit(域,请求方,资源类型)`:跨域请求裁定
  · 请求方在域内 + 资源在域内 → PERMIT_INTERNAL
  · 请求方在域外 + 资源在域内 → PERMIT_DENIED(默认拒绝)
  · 请求方在间隙 → PERMIT_CONDITIONAL(需提交审查,暂时存入审查队列)
  · 任何带 transform 类型的关系 → PERMIT_DENIED(跨域变换一律拒绝)

- `mqf_macronein_domain()`: 宏量 vs 微量判定
  · 宏量:涉及资源>阈值 → 受严格域控 → 需审查
  · 微量:涉及资源≤阈值 → 可能从宽 → 视合规度而定
  · 微量积聚→宏量:任何宏量积累到阈值自动升级为宏量审查
  · "内积可积累"(同一域内的微量可积累) vs "外积不可积累"(域外的微量积累无效)

- 新增审计类型:DOMAIN_TAG / DOMAIN_PERMIT / DOMAIN_MACROZONE
- 场景演示:L1 域标记可视化 / L2 跨域请求裁定 / L3 宏微量流动追踪

---

## 下一步(按顺序)

### v1.1 六元对称群+六面体
**目标**: 把 v0.1-v0.8 执行内核与飞书 Bot、Workers KV 打通,让裁判能力真正输出到外部世界

- 飞书 Bot → MQ 内核:每个飞书消息触发一次内核观测+规则裁定
- 飞书 Bot ← MQ 内核:七妹(满全法 AI 助手)的回复,由内核通过镜像代理给出
- 把 ROADMAP 全套铁律嵌入自动化检查(prolog shell 自动验证)

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
*最后更新: 2026-08-05 (v1.0)*
