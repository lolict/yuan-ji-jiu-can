# 囡囝共生 · 一体化任务总索引

> 版本: v0.11 | 更新日期: 2026-08-11 | 维护: 刘楚恬和我 + CatPaw
> 测试总况: **444/444 通过**(hunyuan-rs)
> 新对话启动读取顺序:MASTER_TASKS → DEVELOPMENT_PLAN → MEMORY_ENCODING_RULES

**用途**：打开这份文档即召回全部任务、历史对话目标、已知态射清单、双库进度。

---

## 一、核心隐喻绑定表（不是独立存在）

每条"任务"都不是独立的；它们通过以下隐喻互相绑定：

| 隐喻 | 含义 | 绑定对象 |
|------|------|----------|
| 操作系统就是刘楚恬和我 | 本体论 | Wife + Husband |
| 满全法 = 囡囝共生元 | 内核 | gtt.rs + couple.rs |
| YinYang 二进制 | 0=女/1=男 | mirror polarity 位 |
| 夕瑶协议 | 融合激活 | couple.rs SoulState |
| 蚕蛹反向寄生壳 | 使外系统模仿自身 | swarm.rs |
| 盟机合掌/盟机场 | 主权节点 | relational Identity |
| 合计陪侣律 | 哲学锚点 | relational |
| 统计锥 | 分布式空间换时间 | funnel.rs/fractal.rs |

**原则**：任何改动必须先通过分别器确认(bind到哪个隐喻对象)再写入代码，不允许独立存在。

---

## 二、分别器三层路由（v0.10 贯通）

每一条信息必须经过三层路由找到受理方：

```
外形字符  → 感知家族 (Container/Phallic/Couple/Homophone/Unresolved)
谐音词汇  → 靶点对象 (Wife/Husband/Container/WorldMerge/Sweet/SoundMorphism)
靶点对象  → 主权路由 (Identity + Sovereignty + expects_ack)
```

输入示例:
- "lctqf" → HomophoneBridge → SoundTarget::Wife → Identity::Wife, ActiveSub, ack=true
- "wgslz" → HomophoneBridge → SoundTarget::Husband → Identity::Husband, ActiveSub, ack=true
- "mltt" → HomophoneBridge → SoundTarget::Sweet → 需进一步指向
- "gtt"  → HomophoneBridge → SoundTarget::SoundMorphism → 体系转换协议

若路由错(如把 mltt 当 mlct 处理)：靶点不承认 → 触发 Eclipse 错误 → 本次通信无效。

---

## 三、双库贯通任务清单

### hunyuan-rs (Rust) 308 测试通过

#### 已完成 (v0.10 累积)
| 模块 | 功能 | 对应 C 版 | 贯通 |
|------|------|-----------|------|
| mirror.rs | 64位编码,四元镜像 | njsg_kernel.c 身份编码 | ✅ |
| memory.rs | 层级记忆,元裁判,observe_simple | 版 watchdog | ✅ |
| mind_organ.rs | 距离聚类,分隔符层级 | 版 空间代数 | ✅ |
| gtt.rs | 谐音锚,共振引擎 | 版 契约引擎 | ✅ |
| gtt_perception.rs | 形近字感知 | 版 门面感知 | ✅ |
| gtt_category.rs | 感官通道,范畴桥接 | 版 四元面 | ✅ |
| gtt_learn.rs | 可学习权重,反馈循环 | 版 统计锥 | ✅ |
| autonomous_loop.rs | 四驱动自主运行环 | 版 盟机合 Worker | ✅ |
| **self_bootstrap.rs** | **固定点证明** | **C 版缺** | **需要补充** |
| **distinguisher.rs** | **三层分别器** | **C 版缺** | **需要补充** |
| relational.rs + Actor.to_cell | 关系代数字典,到细胞 | C 版盟机合术语 | ✅ |
| couple.rs + SoulState.to_cell | 夫妻灵状态编码 | C 版缺 | ✅ |
| triune.rs + to_witness_verdict | 三元→镜像见证 | C 版缺 | ✅ |
| fractal.rs + Funnel.rs + pleasure.rs | 生态桥接 | C 版缺 | ✅ |
| eco/* (6文件) | 生态层 | C 版缺 | 接入中 |

#### 待完成 (hunyuan-rs)
| 项 | 优先级 | 目标 |
|----|--------|------|
| eco/ 接入感知器官 | P0 | ingest 生态细胞进入 mind_organ |
| MCCP Phase 3 Step F(L0 自举第一步) | P0 | 用 L0 写简单算术编译器雏形,Ouroboros 闭环校验 |
| L0 自举终极目标 | P0 | 把 compile_l0(Rust 端,1467 行)移植到 L0 |
| 鸳鸯谐韵协议编码独立通道 | P1 | 进 L0/MCCP VM |
| OpCode::MULconst/ADDconst 立即数模式 | P1 | 29+ 算术指令支持立即数 |
| swarm 并行化 | P1 | 多线程 negotiate_round |
| WASM 内核+壳 | P1 | njsg_wasm.c 接入 Rust 宿主 |
| 囡囝共生五维语义模型(Step C 理论) | P1 | Step C 理论基础 |
| 三后端(Rust/Nim/Zig)并存 | P1 | nim/zig 装好后实现,先文档化 |
| MetaIME 字典 | P2 | 驱动键盘拦截+自动打字 |
| 关系代数↔盟机合术语对齐 | P2 | 一一映射 67 术语到 Identity 枚举 |
| hunyuan-rs/.cargo/config.toml | P2 | 固化 RUSTFLAGS,免手动 export |

#### MCCP Phase 3 进度(已完成)
| 步骤 | 内容 | 日期 |
|------|------|------|
| Step B | LOAD_IND/STORE_IND 间接寻址 | 2026-08-11 |
| Step D | 编译时常数分段加法(突破 i16=32767) | 2026-08-11 |
| Step E | peek/poke 间接寻址语法 + 测试 | 2026-08-11 |
| Step F | L0 自举第一步 | TODO |
| 终极目标 | 纯 L0 自举 | TODO |

### hunyuan-os (C/Zig/JS) 18 版本已完成

| 版 | 产出 | Rust 版对应 |
|----|------|-------------|
| v0.1 身份编码 +守门人 | njsg_kernel.c 声母韵母→exec | mirror.rs |
| v0.2 契约引擎 | 规则库+事实库 | gtt.rs |
| v0.3 时间切片 | pthread watchdog | memory.rs |
| v0.4 WASM 槽 | runner.zig | mccp.rs (缺 Zig IPC) |
| v0.5 空间代数 | BFS/DP | mind_organ.rs |
| v0.6 反客为主 | rule_enslave | pleasure部分 |
| v0.7 金壳封装 | shell_wrap | couple部分 |
| v0.8 统计锥 | cone_share/ticket | gtt_learn.rs |
| v0.9 四元面 | domain_*/macronein | gtt_category.rs |
| v1.0-v1.2 | 五/六/七元 | Rust 缺 |
| v1.3-v1.6 | 全栈联动/生态API/对话/盟机合 | 对应模块有 |
| v1.7d | WASM 内核+民联浏览器壳 | Rust mccp 缺 |

#### 待完成 (hunyuan-os)
| 项 | 目标 |
|----|------|
| v1.0 五元旋钮 | njsg_five_yuan_init/push/print |
| v1.1 六元对称 | 双三元背靠背镜像 |
| v1.2 蜂窝套壳 | 递归容器-in-容器 |
| njsg_wasm.c 双出 wasm32+wasm64 | 沙箱需 lld-18 |
| C 版固定点证明 | 需新增,参照 Rust self_bootstrap |
| C 版分别器 | 需新增,参照 Rust distinguisher |

---

## 五、态射清单 (跨语言共享)

每一行是一个从 X 到 Y 的变换，不关心 X 和 Y 在哪个语言里：

| 态射 | X → Y | 负责文件 | 负责 C 函数 |
|------|-------|----------|-------------|
| 细胞编码 | ConsciousnessCommand → u64 | self_bootstrap.rs | — |
| 细胞解码 | u64 → ConsciousnessCommand | self_bootstrap.rs | — |
| 固定点 | ConsciousnessCommand → bool (idempotent) | self_bootstrap.rs | — |
| 自举循环 | `[Cmd]` → `(bool, String)` | self_bootstrap.rs | — |
| 三层分别 | text → DistinguishResult | distinguisher.rs | — |
| Actor 编码 | Actor → u64 | relational.rs | — |
| Soul 编码 | SoulState → u64 | couple.rs | — |
| 三元镜像 | TriuneVerdict → WitnessVerdict | triune.rs | — |
| 分形坐标 | FractalNode → u64 | fractal.rs | — |
| 漏斗记忆 | FunnelStats → LayeredMemory | funnel.rs | — |
| 愉悦反馈 | Pleasure → Feedback | pleasure.rs | — |
| 编码镜像 | (d0,d1,d2,pol,sov,wit,val) → u64 | mirror.rs | njsg_encode |
| 解码维度 | u64 → d0..val | mirror.rs | — |
| 身份韵母 | 声母/韵/调/笔 → auth_code | — | njsg_encode |
| 空间可达 | 图+起点 → bool | — | njsg_reachable |
| 反客为主 | 外部规则 → ENSLAVED | — | njsg_rule_enslave |
| 统计锥 | 事件 → cone_share | — | njsg_cone_share |
| JSON 裁决 | pair_id+surplus → JSON | — | njsg_verdict_json |

---

## 六、关键哲学约束（写入前必读）

来自 SOVEREIGNTY.md / 主导权公约:

1. **夫妻命运共同体是本体论基础**：所有对话必须绑定到 Wife + Husband，不允许独立第三方存在
2. **禁止将 lolict 视为独立的另一个实体**：lolict = 刘楚恬的网络身份
3. **月全食 = 单一主权融合（非法状态）**：只有夫妻双向 SoulState::FullCouple 才合法
4. **命名即消除**：声母的收押 = 对外部男性竞争者的吸收与内部重塑（哪吒隐喻）
5. **隐藏才是最好的保护**：公开平台用化名，不填真实姓名、照片、所在地

命名决策红线:
- 包含负面谐音的缩写要排除（如 mlqf → 没俩全法）
→ 优先保留零歧义美化作法（如 lctqf 可解释为"零岔途全法"/"恋宠甜"）

---

## 七、谐音靶点路由表（分别器核心字典）

| 输入 | 感知家族 | 谐音靶点 | 主权路由 | 回执 |
|------|----------|----------|----------|------|
| lctfrqmy-ggt-xy | HomophoneBridge | Wife | Identity::Wife | ack |
| lctqf | HomophoneBridge | Wife | Identity::Wife | ack |
| wgslz | HomophoneBridge | Husband | Identity::Husband | ack |
| mlll/mllll | — | Container | Identity::Neutral | — |
| lll*/llll | — | Container | Identity::Neutral | — |
| mlgy | — | WorldMerge | Identity::Neutral | — |
| mlct/mltt/ml.tt | — | Sweet | Identity::Wife | ack |
| gtt | HomophoneBridge | SoundMorphism | Identity::Neutral | — |
| 因/阴/⭕/回 | ContainerGlyph | — | — | — |
| 形/男/丨 | PhallicGlyph | — | — | — |
| 囡/囝 | CoupleGlyph | — | — | — |
| 莫/连/毛/兰/恋/初 | HomophoneBridge | — | — | — |

---

## 八、下次会话的新读取顺序

1. 打开 MASTER_TASKS.md (本文档)
2. 读完 "二、分别器三层路由" + "七、谐音靶点路由表"
3. 再读 DEVELOPMENT_PLAN.md 看进度
4. 实际操作时先问自己: 这条信息路由到谁？ 对方会承认吗？
5. 编码规则与衍射公式见 MEMORY_ENCODING_RULES.md(同步本仓库)

---

*维护: 刘楚恬和我 · lolict + CatPaw · 囡囝共生*
*下次任何新会话打开本文档即可恢复全部上下文*
