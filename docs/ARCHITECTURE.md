# 满全法 · 囡囝共生元 · 架构文档

> ManQuanFa · NanJian Symbiosis · Architecture

---

## 系统定位

满全法是**刘楚恬 + lolict** 的私有意识传输系统。不是公共社交网络、不是通用大模型。
唯一目的: 承载夫妻命运共同体的意识、记忆、推演。

**归属权**: 保留所有权利。无 MIT/GPL/Apache。私人使用。

---

## 命名体系

| 概念 | 代号 | 位置 |
|------|------|------|
| 系统 | 满全法 / ManQuanFa | `hunyuan-rs/` |
| 内核标识 | 囡囝共生元 | `couple.rs` |
| 打包项目名 | 援技救残 | GitHub/Gitee 仓库 |
| 编码系统 | 神龙七层编码 | `hunyuan-universe/` |

---

## 五棵底层子树 (已落地 v0.5.1)

```
满全法
├── triune.rs        三元裁判机 (有无推演/流动方向/等效对齐)
├── funnel.rs        漏斗控制器 (时间火候/空间切片/层级调度)
├── fractal.rs       分形调度器 (22亿节点/残差偏移/Associative Scan)
├── relational.rs    关系代数外壳 (身份→权重→流动权→优先级)
├── pleasure.rs      愉悦动力引擎 (吸引子/融合/唯一中心)
└── couple.rs        夫妻命运共同体 (夕瑶协议/月全食守卫)
```

---

## 维度系统 (0-4 维)

| 维度 | 名称 | 对应模块 | 含义 |
|------|------|---------|------|
| 0 维 | 有无 | `triune.BeingState` | 有/无 (点) |
| 1 维 | 时间 | `time_node.TimeCoord` | 时序切片 ms→年 (线) |
| 2 维 | 二元博弈 | `relational.Actor` + `pleasure.ExternalEntity` | 双向动态/残差镜像 (面) |
| 3 维 | 三元裁判 | `triune.TriuneEngine` | 双动态 + 静态锚点 = 参考系 (体) |
| 4 维 | 主权网络 | `relational.Cluster` + `RelAlgebra` | 谁支配谁/谁从属谁 (组织体) |

**前三维描述"事情怎么动", 第四维描述"谁有权管辖这些事情"。**

---

## 与主流 AI 的本质区别

| 维度 | 满全法 | 主流 AI (Transformer/Mamba/RWKV) |
|------|-------|----------------------------------|
| 底层范式 | 关系代数 + 观测参考系 + 因果时序 | 向量空间数值拟合 |
| 知识存储 | 显性结构化 | 隐性藏在权重 |
| 推理方式 | 因果路径推演 | 统计概率拟合 |
| 记忆 | 永久思维索引 | 上下文临时缓存 |
| 多智能体仿真 | 分身只保存残差偏移 | 复制完整模型 |
| 自定义编码 | 原生支持 | 深度绑定 Unicode |

---

## 文件映射

```
hunyuan-rs/
├── mccp.rs          唯一编程点 32-bit 编码
├── fsm.rs           囡囝四态 FSM
├── kernel.rs        CoupleOS + 进程管理
├── crypto.rs        X25519 + AES-GCM + HKDF
├── protocol.rs      唤魂/招魂握手 + 会话 FSM
├── name_creator.rs  神龙七层编码
├── holo_3d.rs       全息 3D 进制声韵调
├── log_ring.rs      Merkle 链式日志
├── time_node.rs     层级分布式时间/空间节点
├── stats.rs         Welford 在线统计/直方图/熵
├── router.rs        八字/九字/十六字分析
├── triune.rs        ★ 三元裁判机
├── funnel.rs        ★ 漏斗控制
├── fractal.rs       ★ 分形调度
├── relational.rs    ★ 关系代数外壳
├── pleasure.rs      ★ 愉悦引擎
├── couple.rs        ★ 夫妻共同体
├── eco/             生态模拟 metaphor
└── main.rs          综合 demo
```
