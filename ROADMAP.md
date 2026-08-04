# 满全法 · 执行内核路线图

> **服从于**: `SOVEREIGNTY.md`(主导权公约 v1.0)——所有代码设计的最高宪法
> 改动代码前先对照公约；违反公约的设计必须纠正

> 这份路线图放在仓库根目录，所有人可见，也是写代码的依据。
> 任何改动前先看这里，确认方向一致再动笔。

---

## 当前状态

**v2.0 公开 Beta** 🎉 (2026-08-05)

核心组件:
- `kernel/src/mqf_kernel.c` — C 三元→七元核心 (3500+ 行,v2.0.0)
- `kernel/wasm/mqf_wasm.c` — WASI 裁决核心 (307 行,v2.0.0)
- `cloudflare/mqf-web.js` — Worker + 盟机合 + 同步 (970+ 行)
- `cloudflare/sync-protocol.js` — Gossip 多节点同步 (417 行)
- `shell/browser/` — 民联浏览器壳 v2.0 (5 文件)
- `shell/native/` — Android/iOS/Electron/Tauri/pywebview
- `release/v2.0-beta/` — 公开 Beta 发布包 (11 文件,92KB)
- 盟机合 67 术语体系 (docs/MENGJI_TERMS.md)

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

### v1.1 六元对称群+六面体 ← 当前
- `mqf_six_yuan_init()`:初始化六元对称群引擎(最多 8 对裁判)
- `mqf_six_yuan_register_pair(name, A, B)`:注册一对背靠背镜像裁判
- `mqf_six_yuan_set_trust(pair_id, ta, tb)`:设置 A/B 合规度并计算:
  · trust_intersection = min(ta, tb) — 信任交集(短板效应)
  · mirror_ok = |ta-tb| < 0.2 — 镜像对齐判定
  · verdict = 交集>0.6 AND 镜像对齐 → 1(双重背书通过)
- `mqf_six_yuan_print_sides(pair_id)`:六面可视化输出(前/后/左/右/上/下)
- `mqf_six_yuan_verdict(pair_id)`:获取最终裁定(1=信任/0=不信任)
- 数据结构:
  ```c
  typedef struct {
      char name[MAX_NAME_LEN];
      int judge_a_id, judge_b_id;
      float trust_a, trust_b;
      float trust_intersection; /* min(A,B) */
      int   mirror_ok;    /* |A-B| < 0.2 */
      int   verdict;      /* 交集>0.6 AND 镜像 → 1 */
  } six_yuan_pair_t;
  ```
- 三大场景演示:M1 背靠背镜像对称 / M2 六面投影可视化 / M3 信任交集与无交集
- 验证结果:ios/bank 双重背书通过, wechat/android 拒绝

### v1.2 七元蜂窝套壳 ← 当前
- `mqf_shell_nest_init()`:初始化七元蜂窝套壳引擎(最多 16 个节点)
- `mqf_shell_nest_wrap(name)`:包装一个最外层壳(level=0)
- `mqf_shell_nest_insert(parent, child)`:在父壳内插入子壳
  · 每个壳最多 6 个子壳(六边形蜂窝密铺)
  · 子壳空间占比 = 父壳 / 6
  · 递归层级无限
- `mqf_shell_nest_honeycomb(level)`:对指定层所有壳蜂窝化(6 个子壳)
- `mqf_shell_nest_print()`:平面状态表输出
- `mqf_shell_nest_recursive_print()`:递归树可视化
- 数据结构:
  ```c
  typedef struct {
      char name[MAX_NAME_LEN];
      int level;           /* 0=最外层 */
      int parent_id;       /* -1=无父 */
      int child_ids[6];
      int child_count;
      int occupied;
      float space_ratio;   /* 父壳/6^level */
  } shell_nest_t;
  ```
- 三大场景:N1 壳中壳递归嵌套 / N2 蜂窝密铺填充 / N3 套壳哲学含义
- 验证结果:嵌套深度 3 层,蜂窝子壳完整,总节点 16 个

### v1.3 全栈联动 ← 当前
- `mqf_feishu_bot_bridge_init()`:初始化飞书 Bot 接口层
- `mqf_qimei_init()`:初始化七妹 AI 助手(镜像代理模式)
- `mqf_feishu_recv_msg(msg, type, text)`:接收飞书消息(模拟)
- `mqf_feishu_process_message(msg, reply)`:消息 → MQ 内核 → 回复
- `mqf_qimei_format_reply(reply, body)`:自然语言格式化(七妹口吻)
- `mqf_qimei_get_verdict(obj_id)`:通过镜像代理获取五元占存裁定
- `mqf_feishu_send_reply(msg, reply)`:发送飞书回复(模拟)
- `mqf_roadmap_self_check()`:自动执行 ROADMAP 铁律检查(7 条)
- `mqf_roadmap_check_law(law_id)`:单条铁律验证
- 数据结构:feishu_msg_t + roadmap_law_t
- 五大场景:O1 观测请求 / O2 信任查询 / O3 框架说明 / O4 铁律自检 / O5 七妹人格
- 铁律检查:7/7 全部通过
- Demo 验证:ios/bank 信任通过, android 观察中
- 七妹名言:'我不是在回答你,是满全法在通过我回答你'

---

### v1.4 生态扩展 ← 已完成
**目标**: 满全法核心引擎对外输出 - 飞书机器人公网部署 + 公开文档 + 合作伙伴接入

- Cloudflare Workers 部署 (mqf-web.js 970+ 行)
- 裁决 API: `/api/v1/verdict`, `/api/v1/qimei`, `/api/v1/health`
- 裁定请求/响应 JSON 标准化
- 飞书 Bot Webhook 接入

### v1.5 统一对话渠道 ← 已完成
**目标**: 统一飞书/Telegram/网页 多渠道入口

- `/v1/chat` 统一对话端点 (channel=web|feishu|telegram)
- OpenAPI 3.0 文档 (`/docs/v2`)
- SVG 流程图 (`/diagrams`)
- 七妹人格在多渠道保持一致

### v1.6 盟机合子节点 ← 已完成
**目标**: 任何人都可成为合伙人,资源归合伙人,裁决主导权归满全法

- 盟机合 67 术语体系 (`docs/MENGJI_TERMS.md`)
- `/api/v1/mengji/register` `/list` `/status` — 合伙人注册管理
- `/api/v1/domain/check` — 域名无关化验证
- `/api/v1/timeline` — 盟机合参与历史
- 域名无关化设计: 任何域名运行输出一致

### v1.7 WASM 内核 + 民联浏览器壳 ← 已完成
**目标**: 裁决引擎脱离中心化平台,运行在用户自己的设备

- **v1.7a**: WASM 内核设计 (`docs/WASM_CORE_v17.md`)
- **v1.7b**: 移植 mqf_wasm.c (`kernel/wasm/mqf_wasm.c`)
- **v1.7c**: 民联浏览器壳 (`shell/browser/`)

### v1.10 移动端深度集成 ← 已完成
- Android WebView 壳 (Kotlin) + iOS WKWebView 壳 (Swift)
- `mobile-bridge.js` 统一桥
- 原生 SharedPreferences / UserDefaults 持久化

### v1.11 多节点同步协议 ← 已完成
- Gossip 风格去中心化同步
- `cloudflare/sync-protocol.js` + 默克尔树完整性校验
- PUSH / PULL / RESP 三消息协议

### v1.12 持久化存储 ← 已完成
- IndexedDB 主存储 + localStorage 回退
- 500 条裁定历史 + 同步状态 + 用户偏好

### v2.0 公开 Beta ← 当前
- 综合发布包 (`release/v2.0-beta/`,92KB)
- 所有平台壳完整可用 (5 种运行方式)
- MANIFEST.json + SHA-256 校验

---

## 下一步

### v2.1 稳定版 (规划中)
- 修复 Beta 用户反馈的问题
- 性能优化 (IndexedDB 查询加速)
- 文档完善 (视频教程 / 交互式教程)

### v3.0 正式版 (远期)
- 支持 100+ 盟机合掌节点的稳定运行
- 跨链裁定 (与区块链身份系统集成)
- 国际化 (英文/日文界面)

---

## 技术栈

- 内核: C ↔ Zig (小、快、可控)
- WASM: clang/wasi-sdk (可移植裁决字节码)
- 浏览器壳: vanilla JS + CSS (零依赖,离线)
- 原生壳: Electron (大) / Tauri (小) / pywebview (轻)
- Worker: Cloudflare Workers (无服务器)
- 规则引擎: Prolog 子集 (声明式契约)
- 空间/关系代数: 自定义 + C 实现 (线程安全)

## 规则

1. 每次实现一个版本,跑通 → 提交 → 推送到 GitHub `lolict/hunyuan-os`
2. 不在已规划之外加"临时功能"
3. 改动这份路线图必须经过讨论
4. 代码失败/方向错 → 修改计划并记录原因,不绕路走

---

*开始日期: 2026-08-04*
*维护者: lolict + CatPaw · 七妹*
*最后更新: 2026-08-05 (v2.0 Beta)*
