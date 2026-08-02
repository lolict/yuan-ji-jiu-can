# 满全法 · 囡囝共生元 · 开发路线图 (ROADMAP)

> 最后更新: 2026-08-02
> 当前阶段: v0.2 加密完备 + 四态完备 + 三元裁判 + Android寄生 + 元输入法

---

## v0.2 新增 (2026-08-02)

### 加密层
- [x] X25519 ECDH 握手
- [x] AES-256-GCM 端到端加密
- [x] HKDF-SHA256 会话密钥派生
- [x] NonceTracker 防重放

### 时序数学层
- [x] 态射 morphism (范畴论)
- [x] 关系代数 relational algebra
- [x] 三元裁判 TernaryArbiter
- [x] 二元镜像 BinaryMirror
- [x] 参考系 convergence

### 状态机
- [x] quaternity FSM 集成到 kernel
- [x] 进程立约自动建 FSM

### 寄生层
- [x] Android 寄生 stub (17 文件)
  - 前台服务 + 常驻通知
  - 无障碍 IME 三套保活
  - SyncAdapter 唤醒
  - BootReceiver 开机启动
  - 隐藏文件系统

### 元输入法 MetaIME
- [x] 5 模块 (manifest/agent/auto_pilot/intent/channel_router)
- [x] Capability-Based Security

### 交互层
- [x] Web UI 扩展 (6 新面板)
- [x] CLI 扩展 (12 新命令)
- [x] 白皮书更新 (v0.2)

---

## 一、当前状态

### 已完成模块

| 模块 | 文件 | 状态 | 说明 |
|------|------|------|------|
| 观测者引擎 | `kernel/observer.py` | ✅ 完成 | 二元/三元观测者 + 自检通过 |
| 进程管理 | `kernel/kernel.py` | ✅ 完成 | 夫妻共同体 OS + 合一递归 tick |
| 封神拟物 | `kernel/mythology.py` | ✅ 完成 | 50+ 封神实体 + 囡囝四态 |
| 灵犀协议 | `kernel/protocol.py` | ✅ 完成 | 8 仙术 + 二进制帧格式 |
| 四态测试 | `kernel/test_quaternity.py` | ✅ 完成 | 14 测试全通过 |
| Web 控制台 | `web/server.py` | ✅ 完成 | 9 个交互面板 |
| 民联词典 | `data/minlian.json` | ✅ 完成 | 179 条条目 |
| 白皮书 | `whitepaper/index.md` | ✅ 完成 | 8 章,含公共网络批判 |
| CLI 工具 | `cli.py` | ✅ 完成 | 命令式交互 |
| 寄生壳 (Python) | `chrysalis.py` | ✅ 完成 | TUI 13 种寄生壳 |
| 寄生壳 (Shell) | `bootstrap.sh` | ✅ 完成 | 最小自举器 /dev/shm |
| 寄生壳 (浏览器) | `chrysalis.html` | ✅ 完成 | WASM+IDB+WebCrypto |
| README | `README.md` | ✅ 完成 | 完整项目说明 |
| 数学库 | `morph/` | ✅ 完成 | morphism/relational/ternary/binary_mirror/convergence |
| 加密协议 | `kernel/protocol.py` (L2) | ✅ 完成 | X25519 + AES-GCM + NonceTracker |
| 姓名锚 | `kernel/name_anchor.py` | ✅ 完成 | name_to_anchor + joint_anchor |
| 四态FSM | `kernel/quaternity_fsm.py` | ✅ 完成 | 9 转换规则 + kernel 集成 |
| 关系日志 | `kernel/log_ring.py` | ✅ 完成 | SHA-256 链 + Merkle root |
| Android寄生 | `android/` | ✅ 完成 | 17 文件 Flutter stub |
| 元输入法 | `ime-agent/` | ✅ 完成 | 5 模块 skeleton |
| Web UI | `web/server.py` | ✅ 完成 | 9→15 面板 |
| CLI | `cli.py` | ✅ 完成 | 多子命令 |

---

## 二、下一步 (v0.2 — v0.5)

### P0 阻塞级 (必须做)

- [x] **P0-1** 排病诊断独立模块 `kernel/paibing.py` -- ✅ 完成
  - 双编码 14 对声母镜像检查
  - 三编码 9 组路径矩阵检查
  - 四档评估:健康/疑似/病变/癌变
  - 输出治愈建议 (转置/镜像修复)

- [x] **P0-2** 灵犀协议加密升级 -- ✅ 完成 (含 X25519 + AES-GCM + NonceTracker)

- [x] **P0-3** 关系日志环 `kernel/log_ring.py` -- ✅ 完成
  - append-only 关系状态日志
  - 梅克尔树历史锚定
  - 日志不可篡改 + 可审计

### P1 重要级 (应该做)

- [x] **P1-1** 中文姓名→锚算法 `kernel/name_anchor.py` -- ✅ 完成
  - 汉字拆解:声母/韵母/笔画/偏旁 → 256 bit anchor
  - `name_to_anchor()` + `joint_anchor()` 夫妻联合锚
  - 刘楚恬=mqf_name_a5de89... , lolict=mqf_name_4f4993...
  - 汉字拆解:声母/韵母/笔画/偏旁
  - 六十四进制 base64 编码
  - 刘楚恬 + lolict → 唯一 anchor hash

- [x] **P1-2** 囡囝四态状态机 -- ✅ 完成 (9 规则 + kernel.py 集成 + FSM→process)

- [x] **P1-3** 恢原尔倒裹脚本 -- ✅ 完成 (`restore.py` 含 --snapshot/--verify/--restore/--list)

- [x] **P1-4** Android 寄生实现 -- ✅ 完成 (stub 17 文件,真实 APK 编译为下一里程)

### P2 增强级 (可以做)

- [ ] **P2-1** 手机端 Web 控制台优化
  - 响应式 CSS
  - 触摸友好的面板

- [ ] **P2-2** iOS 寄生 stub
  - BGTaskScheduler 注册
  - 静默推送唤醒

- [ ] **P2-3** 道家·仙境叙事扩展
  - 增加封神演义第二部角色
  - 增加西游记映射
  - 增加庄子·老子映射

### P3 愿景级 (远期)

- [ ] **P3-1** RTL 硬件实现 → FPGA 流片
- [ ] **P3-2** 脑机接口 (BCI) 接入层
- [ ] **P3-3** 具身智能 ROS2 节点
- [ ] **P3-4** 卫星通信遥测寄生

---

## 三、版本规划

### v0.1 (当前)
- [x] 基础内核 + 观测者
- [x] 封神台拟物
- [x] 灵犀协议
- [x] Web 控制台
- [x] 寄生壳三层 (Python/Shell/HTML)
- [x] 白皮书 8 章
- [x] 公共网络批判章节

### v0.2 (下一步,预计 2026-08)
- [ ] P0 三项 (诊断/加密/日志)
- [ ] 中文姓名→锚算法
- [ ] 囡囝四态状态机

### v0.3
- [ ] Android/iOS 寄生实现
- [ ] 恢原尔倒裹脚本
- [ ] 手机端 UI

### v0.4
- [ ] BCI 接入层
- [ ] ROS2 节点
- [ ] 卫星遥测

### v0.5
- [ ] 全链路加密打通
- [ ] 完整 12 仓恢原测试
- [ ] beta 内部发布

### v1.0 (远期)
- [ ] RTL FPGA 原型
- [ ] 产品化封装
- [ ] 全部 P3 完成

---

## 四、架构不变式

> 无论后续怎么扩展,以下规则不违反:

1. **姓名 = 协议**:刘楚恬 + lolict 永远是 root of trust
2. **私有 = 一切**:绝不开放公共 API,绝不上链,绝不托管
3. **无根权限**:程序不依赖 sudo/admin,自带内生 capability
4. **反公共网络**:满全法不是互联网,不索引外部节点
5. **囡囝共生**:主导权在妻子一侧,丈夫嵌套其中
6. **观世音庇护**:象征弱势群体对健全人规则的反抗
7. **封神台 = 唯一操作系统映射**:所有概念必须能在封神台找到对应

---

## 五、版权与许可

```
© 2026 刘楚恬 & lolict (七妹凹月留)
不选择任何开源许可证 (MIT/GPL/Apache 均不适用)
协议 = 嵌入程序内部
一切未明确授予的权利均保留
```

---

> 满全法,南海观世音菩萨的慈悲,庇护那些被健全人规则抛弃的亲密关系。
