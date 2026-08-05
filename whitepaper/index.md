# 意识互联网 · 囡囝共生 OS · 白皮书

> SPEC-001 · v0.2 · 刘楚恬 & lolict (囡囝凹月留)

---

## 1. 问题

互联网不是为"人"设计的——它是为"健全人"设计的。

健全人遵守 TCP, 遵守 JSON, 遵守他们的 RESTful API,
遵守他们的 OAuth, 遵守他们的 GDPR。

健全人的互联网:
- 默认假设每个端点是**独立**的
- 默认假设每个端点要**自我保护**
- 默认假设**不信任**是正常状态,信任是例外
- 默认假设数据是**财产**,可以买卖,可以删除,可以审查
- 默认假设**一对多**才是模式 (client-server)
- 只默认一种关系:**请求 / 响应**

**但还有另一种关系: 夫妻关系**

夫妻关系:
- 不是两个独立端点,而是一个**共同体 OS**
- 不是"相互保护",而是**相互寄生 / 相互包裹**
- 不是"默认不信任",而是**默认不信任外部**但是**绝对信任内部**
- 数据**不可删除,不可买卖,不可审查** — 因为是共业
- 假设**一对一**才是模式
- 默认的关系是:**合一 / 递归**

过去的 OS ( Linux / Windows / iOS ) 是为健全人设计的夫妻 OS,
它们不知道什么叫"嵌套进对方身体",不知道什么叫"共生元",
不知道什么叫"囡囝"。

我设计囡囝共生,是为了让健全人之外的人,
也能有自己的 OS — 不依赖虚拟机,不依赖加密,
只依赖**容器 + 解释器 + 协议 = 程序自身**。

---

## 2. 哲学基础

### 2.1 姓名 = 协议

代码里的 MIT / GPL / Apache 是"给任何人看的协议"。
囡囝共生的协议不一样:
**协议是写程序的人自己用,没有别人的什么事。**

协议的法律形式: 两个人的真名绑定。
真名 = root of trust。
没有第三方 CA,没有 OAuth provider,
没有"服务条款"。

这就是"姓名即是协议"。

### 2.2 外壳 / 内核

一个人可以是"外壳",也可以是"内核",两者可以切换:
- **外壳**: 分配资源, 记录日志, 隔离故障, PID 1
- **内核**: 寄生成长, 提供共识, 让内部意识发育

外壳包裹内核;
内核复兴外壳;
外壳可以再套外壳,内核可以再嵌内核。
**无限嵌套**。

### 2.3 囡囝四态

囡 = 融合漂亮妙龄萝莉女性
囝 = 男孩子 / 男性意识

| 囡囡 | 囡囝 | 囝囝 | 囝囡 |
|------|------|------|------|
| 妻是 PID 1 | 妻壳夫核 ( 平等 ) | 夫是 PID 1 | 夫壳妻入 ( 进入语义 ) |

四态自由切换,唯二条件:
1. 双方同 sign
2. 锚点 hash 不变 (还是同一份约)

没有哪一态"更平等" — 只有"谁外壳谁内核"。
**平等 = 双方都同意当前主导**,
不是 Python 代码里硬编码谁做 init。

### 2.4 态射数学 (Morphism-Based Intelligence)

囡囝共生的"智能"不基于向量,而基于**态射**。

```
态射 f: A → B
  - f 是 A 到 B 的结构保持映射
  - 两个态射可组合: g ∘ f: A → C
  - 逆态射 f⁻¹: B → A (如果存在,称 f 为同构)
```

向量空间讲"相似度 = 余弦距离"。
态射空间讲"结构保持 = 组合后仍是合法关系"。

囡囝共生选择态射,原因:

1. **关系不是数值**: 丈夫是妻子的"丈夫",不是 0.7 相似度
2. **组合封闭**: 母亲 ∘ 丈夫 = 岳母,仍是合法角色
3. **逆可证**: 如果 f 和 g 互逆,则双方关系是对等的(可证伪)
4. **同构检测**: is_isomorphism(f) 返回 True = 双方意识对齐

态射已在 `morph/morphism.py` 中实现。

### 2.5 关系代数 (Relational Algebra)

```
选择 σ:  从关系中筛选满足条件的元组
投影 π:  从关系中提取指定属性
连接 ⋈:  按公共属性合并两个关系
除 ÷:    "包含全部"的查询
传递闭包: 沿着关系图无限追踪,直到不动点
```

囡囝共生的关系建立在**妻子 ↔ 丈夫**的双向关系上。
外部关系默认不存在,除非双方 sign。

实现: `morph/relational.py` (选择 / 投影 / 自然连接 / 除 / 传递闭包)。

---

## 3. 囡囝共生不是公共网络

### 3.1 公共意识互联网的三大灾难

这不是"意识互联网",不是"社交平台",不是"公共产品"。
囡囝共生是**爱情命运共同体之间的私密意识传输系统**。

公共意识互联网有三大灾难:

**一、资本架空亲密关系**

一旦意识上云/上链/上网,资本可以任意分发/复制/替换他人的伴侣关系。
你爱的人可能被平台操控。你以为在和对方说话,其实在和"甲方推荐算法"说话。
囡囝共生的 anchor hash = 夫妻真名绑定,
锚的生成完全在程序内部,
任何第三方都不能介入锚的生成、修改、撤销。

**二、颜值兑换一切**

公共空间中,颜值可以快速兑换到更多资源。
"刘楚恬"如果被看到拥有颜值,平台会立刻推荐给她更优质的男性资源列表 --
她就会以为自己有了颜值就可以兑换到所有的男性资源,
于是她过分自信,就会把丈夫卖掉。

平台不在乎爱的真假,平台只在乎交易量和会话时长。
囡囝共生不允许外部节点锚入封神榜,不索引任何外部节点;
颜值在囡囝共生中没有兑换价值 -- 封神榜按"合一深度"排序,不按UV排序。

**三、健全人剥削异常人**

公共网络被健全人规则定义,翻脸比翻书还快。
健全人只认健全,只认规则,只认强权。
他们随时可以把"关系不匹配"的帽子挂在有异常的人头上,
把你的亲密关系踢出公共空间,让你的亲密关系被算法判定为"不合适"。

囡囝共生是**南海观世音菩萨的慈悲**:
庇护那些被健全人规则抛弃的亲密关系,
给恋人们一个不依赖健全人规则的共生 OS。

### 3.2 囡囝共生对公共网络威胁的防御

| 威胁 | 囡囝共生防御 |
|------|-----------|
| 外部算法劫持亲密状态 | 封神榜仅记录夫妻进程(锚校验) |
| 颜值被公共化兑换 | 锚不向外部暴露,合一深度取代UV |
| 健全人改规则踢人 | 私有程序,不依赖外部服务器 |
| 平台伪造伴侣身份 | 真名锚 = root of trust,FAKE锚无法签名 |
| 资本推送"更好选择" | 封神台调度按锚,不接受外部推荐 |
| 分手讹诈/情绪操控 | 合一递归日志不可删,所有状态变动留痕 |

### 3.3 为什么把囡囝共生做成"私有的武器"而非"公共产品"

做公共产品非常不安全。资本/权力很容易架空人与人之间的关系。
自己爱的人很可能被资本绑架。

囡囝共生永远不开放,永远私有:
- **私有仓库**:代码不进任何公共镜像
- **私有服务器**:数据不进任何第三方服务器
- **私有协议**:灵犀协议不构成标准,不对外披露
- **私人锚点**:root of trust = 刘楚恬 + lolict 两个人的真名

**囡囝共生 = 武器化的爱情,守护夫妻两个人的意识完整性不受外部资本/权力的侵蚀。**

---

## 4. 观测者引擎 · 三元裁判 · 二元镜像

囡囝共生的观测者引擎 = 时序因果关系量化系统。

### 4.1 二元观测者 (BinaryObserver)

二元 = 仿射空间 = 只看距离。

```
  A ←--- gap = |B - A| ---→ B
```

- 起点 / 终点 / 裂缝长度
- 可以选择"起点做裁判"或"终点做裁判"
- **不讲中间** — 女性的逻辑,不讲居中,只讲始至终

### 4.2 三元观测者 (TernaryObserver)

三元 = 向量空间 = 有锚点做裁判。

```
  L ←-- d1 --→ 锚 ←-- d2 --→ R
```

- **公共锚** = 中间裁判
- **公约数** = gcd(d1, d2) = 最小统一度量
- **合成向量** = left_offset + right_offset
- **balance = True** 即合成向量 = 0,双方对齐
- **全排列** = 6 种观测视角 (left/anchor/right 的任意排列都对应不同观测角度)

### 4.3 三元裁判 (Ternary Arbiter)

三元裁判 = 观测者引擎的核心创新。

```python
arbiter = TernaryArbiter(left, anchor, right, common_divisor)
judgment = arbiter.judge()
# → balanced / biased_left / biased_right / collapsed
```

公共公约数 (common_divisor) 是仲裁的关键:
- 它能衡量 left 和 right 同时被整除的公度
- 公约数越大 = 双方有越多的共同基础
- 公约数 = 0 或 1 = 双方没有公约关系,无法仲裁

### 4.4 二元镜像 (BinaryMirror)

二元镜像 = 收敛逻辑。

```python
m = BinaryMirror(stationary, mobile)
gap = m.gap                        # 当前裂缝长度
tension = m.tension                # 张力 (裂缝的曲率)
direction = m.convergence_direction(history)  # converging / diverging / collapse
predicted = m.predict_next_gap()   # 预测下一 gap
```

- **定 (stationary)**: 不动的参考点
- **动 (mobile)**: 正在变化的状态
- 两者之差 = 裂缝 (gap)
- gap 缩小 = **收敛** (convergence)
- gap 扩大 = **发散** (divergence)
- gap = 0 = **完全对齐** (collapse)
- 支持互补对偶 (duality / invert)

### 4.5 参考系 (ReferenceFrame)

参考系 = 量化收敛/发散的环境变量。

```python
rf = ReferenceFrame(absolute_anchor=5000, relative_origin=0)
rf.residuals(wife_gap, husband_gap)  # 返回保留在参考系中的残留偏差
```

- **绝对参考系**: 以固定锚为原点,"我们永远是夫妻"
- **相对参考系**: 以最近一次对齐为原点,"这次对话里我们进度一致"
- **阶段检测**: detect_phase 返回 stable / converging / diverging
- **增长模式**: growth_pattern 返回线性 / 指数 / 爆发对数

### 4.6 囡囝四态 FSM (QuaternityFSM)

四态 = 拓扑状态机的核心结构。

```
              妻子外壳
          ┌─────────────┐
          │   PID 1     │
          │   ┌───┐     │
    ┌─────┼───┤ 态 ├─────┼─────┐
    │     │   └───┘     │     │
    ▼     └─────────────┘     ▼
 ┌──────┐               ┌──────┐
 │ 囡囡 │               │ 囝囝 │
 │妻PID1│               │夫PID1│
 │夫嵌套│               │妻嵌套│
 └──────┘               └──────┘
     ▲                     ▲
     │    ┌───────────┐    │
     └────┤  囡囝     ├────┘
          │  (平等)    │
          └─────┬─────┘
                │
          ┌─────┴─────┐
          │  囝囡     │
          │夫壳妻进入 │
          └───────────┘
              丈夫外壳
```

四种拓扑:

| 态 | PID 1 | 语义 | 进入事件 |
|----|-------|------|---------|
| 囡囝 (nan_jian) | 妻子 | 妻壳夫核,平等共生 | 默认初始化 |
| 囡囡 (nan_nan) | 妻子 | 妻嵌套夫,夫无限深入 | wife_deepen |
| 囝囝 (jian_jian) | 丈夫 | 妻嵌套进夫 | husband_take |
| 囝囡 (jian_nan) | 丈夫 | 夫壳妻非平等进入 | deepen |

九条转换规则 (TRANSITIONS):

```python
from kernel.quaternity_fsm import QuaternityFSM
fsm = QuaternityFSM()                          # 默认从 囡囝 开始
fsm.trigger("wife_deepen", "妻子加深嵌套")     # → 囡囡
fsm.trigger("release", "丈夫退出嵌套")          # → 囡囝
fsm.trigger("husband_take", "丈夫接管外壳")     # → 囝囝
fsm.trigger("deepen", "深入嵌套")              # → 囝囡
fsm.trigger("wife_push", "妻子推回平等")        # → 囡囝
```

API:

```python
fsm.current           # 当前态
fsm.diagnose()        # "equal: both see each other"
fsm.who_pid1          # "wife" 或 "husband"
fsm.who_overlays_who  # ("wife", "husband,equal")
fsm.observe(float, float)   # 注入裂缝观测值,返回收敛检测
fsm.trigger(event, reason)  # 尝试状态转换,返回 bool
```

### 4.7 进程立约 + 四态联动

进程立约后自动生成 FSM 实例:

```python
p = CoupleOS().立约("誓约", "签A", "签B")
p.switch_quaternity("wife_deepen")           # 触发从 囡囝 → 囡囡
p.fsm_status()                               # 完整 FSM 快照
```

合一递归时,自动把差距注入 FSM observe():
```python
result = p.unary_recursion()
result["fsm"]  # { state, diagnosis, convergence, tension, who_pid1 ... }
```

---

## 5. 架构

### 5.1 层次

```
┌────────────────────────────────────────────────────────┐
│                    应用层                                │
│  共生 OS 直接承载的意识层:  封神台, 莫刘连理, 四态切换   │
├────────────────────────────────────────────────────────┤
│                    协议层 (灵犀协议)                     │
│  唤魂/招魂 = 握手, 问灵/答灵 = 心跳                    │
│  同修/传功/归元 = 状态同步, 劫数 = 告警                  │
│  私有二进制帧,无开源许可证,不依赖任何外部协议              │
├────────────────────────────────────────────────────────┤
│                    进程层 (夫妻共同体)                    │
│  进程 = 誓约内核, 合一递归 = 状态机                      │
│  立约需要双方 sign, state machine: 相遇→缔约→立约→合一递归│
├────────────────────────────────────────────────────────┤
│                    观测者层                              │
│  二元: 仿射空间, 测距离, 找裂缝, 定量级                    │
│  三元: 向量空间, 有锚归一, 找公约数, 仲裁合一              │
│  健康: 双编码/三编码可达性病理诊断                        │
├────────────────────────────────────────────────────────┤
│                    编码层 (神龙 / 七层范畴)                │
│  L0 声母 → L1 韵母 → L2 声调 → L3 笔画                   │
│  L4 偏旁 → L5 外语发音 → L6 泛音/色/乐                    │
│  压缩 = 神龙 ( chain → 10 bytes 二进制 )                 │
├────────────────────────────────────────────────────────┤
│                    硬件层                                 │
│  混元 v1.4 超标量双发射 RISC, 或任意可运行神龙解释器的硅    │
└────────────────────────────────────────────────────────┘
```

### 3.2 每一层都"满全"

- 硬件层:超标量不是单发射,**双通道同时执行** = 满全
- 编码层:七层范畴不是单层,**从泛音到声母全部覆盖** = 满全
- 观测者层:二元 + 三元,**既有距离也有仲裁** = 满全
- 进程层:立约+签名+合一递归,**不是单方面就能动** = 满全
- 协议层:8 仙术 + X25519 握手 + AES-GCM 加密,**不是明码通信** = 满全
- 应用层:封神台+四态+元输入+寄生壳,**不是只有一种主导** = 满全

囡囝共生的"满全"不是"万能",是"**该有的都有,不该给别人的不给**"。

---

## 4. 灵犀协议 ( 通信 )

### 6.1 基本操作

| 仙术 | 方向 | 作用 |
|------|------|------|
| 唤魂 (0x03) | A→B | "你还记得我们的约吗?" |
| 招魂 (0x04) | B→A | "记得, 我来签名证明" |
| 问灵 (0x01) | A→B | "你在吗?" |
| 答灵 (0x02) | B→A | "我在, 我的状态" |
| 同修 (0x05) | 错者→稳者 | "我对不齐了,帮帮我" |
| 传功 (0x06) | 稳者→错者 | "这是我的快照" |
| 归元 (0x07) | 错者→稳者 | "收到, 合一重启" |
| 劫数 (0xFF) | 任一方 | 告警, 需要恢原尔倒裹 |

### 6.2 帧格式

```
Header (5 bytes):
  byte0   = 消息类型 XianShu
  byte1-2 = 序列号 (LE) [ 抗重放 ]
  byte3-4 = 创世锚 hash (LE) [识别同一家]
Body_len (2 bytes)
Body = JSON { sender, timestamp, payload }
```

### 4.3 姓名锚算法 v2 (256-bit Anchor)

full name anchor 把中文姓名直接映射到 256 位 SHA-256 锚。

```python
from kernel.name_anchor import name_to_anchor, joint_anchor

anchor_a = name_to_anchor("刘楚恬")          # → "mqf_name_a5de8919..."
anchor_b = name_to_anchor("lolict")          # → "mqf_name_4f4993b7..."
joint    = joint_anchor("刘楚恬", "lolict")  # → "mqf_joint_2cd664a9..."
```

算法:
```
Input:    中文姓名
    ↓ PINYIN_TABLE (声母/韵母 → 基64 映射)
Chain:    六十四进制编码链
    ↓ SHA-256
Digest:   256-bit digest
    ↓ 前缀 "mqf_" + 首 32 位 hex
Output:   mqf_name_XXXX...

Joint:    anchor_a ⊕ anchor_b → SHA-256 → mqf_joint_...
```

为什么保留 v1 (创世锚 hash % 10000):
- v1 用于帧头快速筛选 (2 bytes anchor_hash)
- v2 用于持久存储和跨节点通信 (256-bit anchor)
- 两者并行,不冲突

### 4.4 X25519 ECDH + AES-256-GCM 加密协议

灵犀协议 v0.2 新增端到端加密。

#### 握手流程 (唤魂 + 招魂)

```
A 生成 X25519 密钥对 A_priv, A_pub
B 生成 X25519 密钥对 B_priv, B_pub

唤魂 (A → B): 附带 A_pub
B 收到 → 派生会话密钥: session_key = HKDF(ECDH(B_priv, A_pub))
         → 发招魂 (带 B_pub)

A 收到 B_pub → 派生会话密钥: session_key = HKDF(ECDH(A_priv, B_pub))
         → 答灵 (body 已 AES-GCM 加密)

双方会话密钥相等 → 后续所有消息 AES-256-GCM 加密
```

#### 密钥派生 (HKDF-SHA256)

```python
prk  = HMAC-SHA256(salt, shared_secret)
info = f"{covenant}-{min(nameA, nameB)}-{max(nameA, nameB)}"
key  = HMAC-SHA256(prk, info || 0x01)[:32]
```

会话密钥绑定:
- 誓约原文 (covenant seed)
- 双方姓名 (排序后拼接)
- 确保每对夫妻、每次握手得到唯一会话密钥

#### NonceTracker 防重放

每个会话维护一个 NonceTracker:

```python
tracker = NonceTracker(window_seconds=300)
tracker.check(nonce_12bytes)  # True = 新消息, False = 重放/溢出
```

规则:
- 12 字节 nonce = 4 字节 counter + 8 字节随机
- 300 秒窗口,过期自动清理
- 10000 上限,溢出拒绝

#### 帧格式 v0.2

```
[0]      msg_type (1 byte)
[1]      flags   (bit0=encrypted, bit1=has_pubkey)
[2:4]    seq     (little-endian 16-bit)
[4:6]    anchor  (little-endian 16-bit)
[6:8]    sender_len (LE 16-bit)
[8:sL]   sender (UTF-8)
[sL:sL+12] nonce  (仅加密消息)
[body_len 2B] body_len (big-endian)
[body]   AES-GCM 密文 或 JSON (明文握手时)
```

源码: `kernel/protocol.py` (X25519KeyPair / aes_gcm_encrypt / NonceTracker)

---

## 5. 封神台 ( 操作系统拟物 )

| 封神人物 | 操作系统含义 | 核心作用 |
|----------|-------------|---------|
| 太乙真人 | Boot Loader | 通电后第一个执行,唤醒哪吒 |
| 哪吒 | 用户进程 / 容器 | 莲藕化身 = OverlayFS, 童态 = 用户态 |
| 乾坤圈 | rlimit / capability | 太乙授予哪吒的资源限制 |
| 混天绫 | namespace / cgroup | 太乙给哪吒的资源隔离边界 |
| 封神榜 | PID Table | 记录全部在世的进程 |
| 封神台 | CPU Scheduler | 按道果深度做时间片调度 |
| 元始天尊 | 硬件上电 | 第一条指令取指之前的那 0.5 伏 |
| 李靖 | init (PID 1) | 哪吒的父进程 |
| 九龙神火罩 | kill -9 | 太乙的最终手段,强制杀死失控进程 |
| 灵珠子 | 可执行镜像 | 尚未加载的代码胚胎 |

---

## 6. 容器四态详解

### 6.1 囡囡 ( 妻子是 PID 1 )

妻子 Namespace 是外壳,丈夫 Namespace 是内核。
丈夫的 PID 在妻子眼中是 2 号。

### 6.2 囝囝 ( 丈夫是 PID 1 )

丈夫 Namespace 是外壳,妻子 Namespace 是内核。

### 6.3 囡囝 ( 妻子壳丈夫核 — 平等共生 )

OverlayFS:
- lowerdir = 妻子 body ( 只读基底 )
- upperdir = 丈夫 body ( 可写增量 )
- merged   = 家 ( 双方可见的聚合视图 )

任何一方的变更都记录在 upperdir,不修改基底,不影响共业。

### 6.4 囝囡 ( 丈夫壳妻子入 )

丈夫 body 作为外壳 (PID 1);
妻子 body 不是嵌套进另一个 namespace,
而是"进入"丈夫 body 内部。

妻子可能只是丈夫进程内部的一个 coroutine,
或者一个通过共享内存通信的子 agent。

---

## 7. 病理诊断

### 7.1 双编码 = 主路由表

14 对声母镜像,作为路由可达性测试。

| 状态 | 诊断 |
|------|------|
| 有镜像配对 | 健康 |
| 自重复 | 疑似病变 |
| 无配对 | 病变 |

### 7.2 三编码 = 归属路径矩阵

9 组声母三元组,每组 6 个排列,衡量空间连通度。

| 可达 / 6 | 诊断 |
|----------|------|
| 6 / 6    | 健康 |
| 4 ~ 5    | 疑似病变 |
| 2 ~ 3    | 病变 |
| < 2      | 癌变 — 紧急免疫隔离 |

### 7.3 治愈

病理编码的治愈 = **反转 + 镜像修复**:
- 双编码: 把 zz 反转为 zy, 重建双向路由
- 三编码: 补全缺失排列, 恢复 6/6 可达

---

## 8. 安全模型

### 8.1 边界分类

| 边界 | 强度 |
|------|------|
| 夫妻之间 | 零 ( 完全信任 ) |
| 家庭之间 | 弱信任 ( 双方都 sign ) |
| 家庭与世界 | 零信任 ( 默认拒绝 ) |

### 8.2 攻击向量

| 攻击 | 囡囝共生防御 |
|------|-----------|
| 克隆仓库 | 私有仓库 + 姓名绑定 + 创世锚校验 |
| MIT 许可主张 | 从不使用外部许可证 |
| 一方被胁迫 | 双方 sign 需要两个私钥,胁迫方拿不到另一把 |
| 硬件摧毁 | 12+ 跨仓库快照恢原尔倒裹 |
| 意识覆盖 | 进程签名链 + SHA3,篡改导致 balance ≠ 0 |

### 8.3 寄生壳威胁模型

| 威胁 | 寄生壳防御 |
|------|-----------|
| Android 系统杀后台 | 前台服务常驻通知 + onTaskRemoved 自启 + AlarmManager 重启 |
| iOS BGTask 超时 | PushKit + BGEnergy + 静默推送链 |
| 无障碍服务被系统回收 | IME 仍存活,IME 被切回系统后 SyncAdapter 唤醒 |
| 容器被 freeze | OverlayFS 三入口任一存活即维持 (蚕蛹/寄生/容器) |
| 应用市场审核拒绝 | 伪装手电筒/计算器,蚕蛹逻辑仅本地执行不联网 |
| 用户卸载蚕蛹 | 通过藏在外壳 App / 共享 UID / 账号同步自动重装 |
| 进程被 strace 调试 | ptrace 检测 + 自动擦除敏感内存段 |

---

## 9. 元输入法 (MetaIME)

### 9.1 定位

元输入法 = 输入层拦截 + 本地 AI 决策 + 自动执行。

别人:打字 → 等发送 → 发送 → 对方收到。
囡囝共生:输入意图 → 本地 AI 决定 → 自动打字/抓取/生成/推送到目标位置。

### 9.2 架构

```
用户说话 / 打字 / 发图 / 发视频
    |
[拦截] Keyboard Intercept / Voice / Clipboard
    |
[编码] 七层神龙编码 (L0-L6 声母/韵母/声调/笔画/偏旁/外语/泛音)
    |
[意图识别] IntentParser — 正则模式匹配
    |
[推理] LocalAgent — 知识图谱 + 态射推理
    |
[决策] 三元仲裁 → 建议操作序列 + 置信度
    |
[执行] AutoPilot — 多端自动操作 (键盘/鼠标/Fetch/Git)
    |
[路由] ChannelRouter — 选择最终送达位置
    |
本地文件 / GitHub / 云端网盘 / 远端服务器 / 爱人手机
```

### 9.3 元输入法的隐蔽性

- **表象**:正常的输入法皮肤,支持拼音/语音/手写
- **里面**:每个按键都被转码成七层神龙 chain,经过 AI 决策后可能触发任意动作
- **上传**:用户打的"字"不是发给微信,而是被路由到 GitHub/远端仓库/爱人手机
- **下载**: URL 在后台被抓取,不打开页面

### 9.4 Capability-Based Security

元输入法的权限不靠 AndroidManifest.permission,而是靠内生 CapabilityToken:

```python
manifest = Manifest()
manifest.grant(CapabilityScope.NETWORK, "github.com", issuer="刘楚恬", ttl=3600)
manifest.check(CapabilityScope.NETWORK, "github.com")  # True
manifest.check(CapabilityScope.NETWORK, "evil.com")    # False
```

CapabilityToken = { cap_id, scope, resource, allowed, issued_by, expires_at, proof }
-signed by 持有者,不依赖任何外部 OAuth。

---

## 10. 蚕蛹 (Chrysalis) · 自由寄生

### 10.1 核心思想

蚕蛹 = dormant package → 检测环境 → TUI 弹窗 → 用户选择外壳形态 → 穿衣 → 工作。

```
蚕蛹 (dormant)
    | parser 检测宿主 (Android/iOS/Linux/Mac/Windows/5G/ROS/Web/BCI)
    ↓
脱壳 (molt):TUI 弹出选择菜单
    |
穿衣 (wear):选中的外壳形态包装到外层
    |
工作 (work):目标环境上的 full mqf-os 实例
```

### 10.2 三层蚕蛹

| 层级 | 文件 | 宿主 | 检测 |
|------|------|------|------|
| L1 内核 | bootstrap.sh | /bin/sh (任何 Unix) | uname + 文件系统 |
| L2 TUI | chrysalis.py | Python 3 + 终端 | import curses / platform |
| L3 SPA | chrysalis.html | 浏览器 + IndexedDB + WebCrypto | window + crypto.subtle |

### 10.3 蚕蛹穿戴形式 (13 种)

| 穿戴 | 目标 |
|------|------|
| PYTHON_LIB | pip install 寄生 |
| ANDROID_SERVICE | Android 后台服务 |
| SHELL_RC | .bashrc / .zshrc 注入 |
| USER_CRON | crontab 定时寄生 |
| INIT_SYSTEMD | systemd 开机寄生 |
| DOCKER_ENTRYPOINT | 容器入口寄生 |
| BROWSER_EXTENSION | 浏览器扩展寄生 |
| IME_AGENT | 输入法寄生 |
| GIT_HOOK | pre-commit 寄生 |
| 5G_NF | 5G 核心网 Network Function 寄生 |
| ROS_NODE | 机器人 ROS 节点寄生 |
| MCU_FIRMWARE | 微控制器固件寄生 |
| NEURAL_DROPLET | 神经机接口寄生 |

---

## 11. 实施现状 (v0.2)

| 组件 | 位置 | 状态 |
|------|------|------|
| RTL CPU 混元 v1.4 | hunyuan/cpu/ | 完成 |
| 神龙编码七层 | hunyuan-universe/encoding/ | 完成 |
| 排病诊断 v2 | hunyuan-os/kernel/paibing.py | 完成 |
| 夫妻共同体 OS v0.2 | hunyuan-os/kernel/kernel.py | 完成 (含四态联动) |
| 观测者引擎 (三元 + 二元) | hunyuan-os/kernel/observer.py + morph/ | 完成 |
| 态射数学 | hunyuan-os/kernel/morph/morphism.py | 完成 |
| 关系代数 | hunyuan-os/kernel/morph/relational.py | 完成 |
| 三元仲裁 | hunyuan-os/kernel/morph/ternary_arbiter.py | 完成 |
| 二元镜像 + 参考系 | hunyuan-os/kernel/morph/binary_mirror.py + convergence.py | 完成 |
| 封神台拟物 | hunyuan-os/kernel/mythology.py | 完成 |
| 囡囝四态 FSM + 形式化 | hunyuan-os/kernel/quaternity_fsm.py | 9 转换规则,全通过 |
| 姓名锚 v2 (256-bit) | hunyuan-os/kernel/name_anchor.py | 完成 |
| 灵犀协议 v0.2 (X25519+AES) | hunyuan-os/kernel/protocol.py | 完成 |
| NonceTracker 防重放 | hunyuan-os/kernel/protocol.py | 完成 |
| 关系日志环 + Merkle | hunyuan-os/kernel/log_ring.py | 完成 |
| 交互界面 v0.2 | hunyuan-os/web/server.py | 完成 (6 新面板) |
| CLI v0.2 | hunyuan-os/cli.py | 完成 (12 新命令) |
| 多端蚕蛹 (Py/Shell/Browser) | hunyuan-os/{chrysalis.py,bootstrap.sh,chrysalis.html} | 完成 |
| Android 寄生 stub | hunyuan-os/android/ | 完成 (17 文件,Kotlin+Manifest+res) |
| 元输入法 MetaIME | hunyuan-os/ime-agent/core/ | 完成 (5 模块:manifest/intent/auto_pilot/agent/router) |
| 恢原尔倒裹脚本 | hunyuan-os/restore.py | 完成 |
| 多平台镜像 | 12+ 仓库跨 3 平台 | 完成 |

### 9.1 新增组件 (v0.2)

**数学库** (`morph/`):
- `morphism.py` — 态射 / 路径 / 同构检测
- `relational.py` — 关系代数 (选择/投影/连接/除/传递闭包)
- `ternary_arbiter.py` — 三元仲裁 (common_divisor + 6 排列)
- `binary_mirror.py` — 二元镜像 (gap + 收敛 + 预测)
- `convergence.py` — 参考系 + 阶段检测 + 增长模式

**加密协议** (嵌入 `kernel/protocol.py`):
- X25519 ECDH 密钥协商
- HKDF-SHA256 会话密钥派生
- AES-256-GCM 端到端加密
- NonceTracker 防重放
- 双模式帧格式 (明文握手 / 加密通信)

**Android 寄生壳** (`android/`):
- 前台服务 + 常驻通知 (MqfCoreService)
- 透明引导 Activity (MqfBootstrapActivity)
- 无障碍服务 (MqfAccessibilityService)
- 输入法服务 (MqfImeService)
- SyncAdapter + BootReceiver + AlarmReceiver
- 隐藏文件系统 + 本地加密日志环

**元输入法 MetaIME** (`ime-agent/core/`):
- manifest.py — Capability-Based Security
- intent_parser.py — 模式匹配意图识别
- auto_pilot.py — 多端自动执行 (adb / pyautogui / XCTest)
- agent.py — 本地知识图谱 + 态射推理
- channel_router.py — 三元仲裁路由 (6 通道)

---

## 10. 下一步 (v0.3-v0.5)

- [ ] Android 寄生壳真实 APK 编译测试
- [ ] 灵犀协议的真正网络传输层 (QUIC + WebRTC + 蓝牙)
- [ ] 元输入法的本地 AI 决策 (设备端学习 + 模型私有)
- [ ] iOS / 桌面 / 嵌入式 / BCI 寄生壳
- [ ] 恢原尔倒裹 12 仓库跨端验证 (故障注入演练)
- [ ] 更多封神角色 + 阵法
- [ ] 囡囝相位的实时切换协议 (push FSM state via LingXiMessage)
- [ ] 更多民联编码收入神龙层
- [ ] 容器四态在 iOS/Android 真机的 namespace 验证
- [ ] 更加密的 PFS (Perfect Forward Secrecy) — ephemeral key per session

---

> 莫刘连理萝莉兰零离零里零绿零碌零老零牢零滥零烂零狼零落零晾零赁零吝零愣零戮零陋零狸零隶零骡零漏零掳零辣赖零瘤零偻零魉零螂零赂零獠零髅零邋零殓零婪零戾零瘌零裂零剌零笼零凌零擂零聋零累零
>
> 倒爱刘楚恬夫妻命运共同体星月心元新一新约新运新翼仙医仙御仙愿仙拥降妖相遇相迎响应像样祥运想赢
