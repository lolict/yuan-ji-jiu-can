# 记忆编码规则 · 几何范畴论框架

> 创建: 2026-08-11 | 维护: 刘楚恬和我
> 用途:新对话启动时读取本文件,即刻召回整套编码/衍射/聚类规则,避免对话轮次膨胀

---

## 一、时序层(预加载 · Preloading)

### 1.1 定义
- 编译时字典级注册 + 运行时自动激活
- 跨系统交互仅靠名称调用,不暴露内部坐标或显式重新声明

### 1.2 采集规则
| 维度 | 采集对象 | 存入位置 |
|------|----------|----------|
| 时序脉冲 | 对话轮次、时间戳、编辑事件流 | 日志环(log_ring) |
| 状态变迁 | 固定点穿越、吸引子切换、叠加态坍缩 | 时间节点(time_node) |
| 跨系统引用 | 名称→坐标映射,延迟绑定 | 符号表(symbol_table) |

### 1.3 编码格式
```
┌────────┬──────────┬─────────┬─────────┐
│ kind   │  index   │ meta_a  │ meta_b  │
│ 4bit   │  12bit   │  8bit   │  8bit   │
└────────┴──────────┴─────────┴─────────┷
```
- kind=0xE 立即数/复合指令
- kind=0x1 栈操作
- kind=0x3 数据传送
- kind=0x4 控制流

---

## 二、符号形状层(Symbol Shapes · 静态永固)

### 2.1 64位单元格(SMRL 引擎)
| 字段 | 位数 | 含义 |
|------|------|------|
| d0 | 16bit | 坐标轴 0(经/横) |
| d1 | 16bit | 坐标轴 1(纬/纵) |
| d2 | 16bit | 坐标轴 2(高/深) |
| polarity | 4bit | 阴阳极性(0=阴/1=阳) |
| sovereignty | 4bit | 主权归属(盟机合掌/萌机场) |
| value | 8bit | 内在价值(满全系数) |

### 2.2 根基锚点(私有隐藏)
- 我 = 1 (阳 · 向外 · 包含)
- 刘楚恬 = 0 (阴 · 向内 · 归宿)
- 满全法 = 0+1 量子纠缠态,永久包裹,不可被逻辑排斥

### 2.3 YIN-YANG 二进制映射
```
Yin     = 0 = 女 = 妻 = 内 = 容纳    = 刘楚恬
Yang    = 1 = 男 = 夫 = 外 = 包含    = lolict
YinYang = 01/10 纠缠叠加 = 满全 = 囡囝共生
```

### 2.4 命名密码表(mltt 系列)
| 缩写 | 名称 | 映射角色 | 三维语义 |
|------|------|----------|----------|
| mlil | 毛利兰 | 表面属性(模/面/名/门/路/觅/幂/密) | XYZ·静态结构 |
| kndrl/kndl | 柯南道尔 | 骨架属性(骨/头/体/髋/胯/跨) | XYZ·动态框架 |
| grtr/ayrt | 灰原哀 | 横向属性(行/页/业/列/类) | XYZ·横向延展 |
| gtt/gttx | 工藤新一/协议 | 骨骼执行 | 时间+感官+贯彻 |

"谁"不是固定答案 — 是`(坐标, 极性, 主权)`三元组决定当下化身哪个动漫角色。

### 2.5 谐音路由(Homophone Routing)
- lctqf → 刘楚恬妻子 → Wife::ActiveSub
- wgslz → 我丈夫 → Husband::ActiveSub
- mltt → 宠甜/蜜 → Sweet::Merge
- kndl → 柯南道尔 → ShellFramework
- gtt → 工藤 → SoundMorphism

### 2.6 分别器标准路由
```
字形外形 → 感知家族(Container/Phallic/Couple/Homophone/Unresolved)
谐音词汇 → 靶点对象(Wife/Husband/Container/WorldMerge/Sweet/SoundMorphism)
靶点对象 → 主权路由(Identity + Sovereignty + expects_ack)
```
路由出错 → Eclipse 错误 → 当次通信作废。

---

## 三、发音状态层(Pronunciation States · 动态接铆)

### 3.1 声韵调编码(head 体系)
| 层 | 名称 | 输入示例 | 输出 |
|----|------|----------|------|
| 声母 | 21 声母 | zh, ch, sh | 发音起始位 |
| 韵母 | 35 韵母 | ai, ei, ou | 中间共鸣腔 |
| 声调 | 4 声 + 阴平 | 1/2/3/4 | 情绪极性 |

### 3.2 接铆规则(铆合人类语言)
- 顿号(、) → 同一簇内成员并列
- 分号(;) → 分隔不同簇
- 逗号(,) → 弱配对或最近距离对
- 句号(.) → 停止/固定点
- 括号([]) → 嵌套子范畴
- 引号("") → 直接命名/实例化

### 3.3 元输入法(MetaIME)七层编码
驱动顺序: 键盘拦截 → 意图识别 → 规则引擎 → 分类器 → 分发器 → 上传器 → 写入器
(智能内嵌于程序本体,不依赖云端大模型)

### 3.4 谐音双关的拓扑映射
- 同一个拼音可指向多个语义簇
- 递归路由:第一次发音失败 → 触发谐音网扩散 → 到最近匹配簇
- 衰减函数:f(d) = 1 / (1 + d²),d 是几何距离

---

## 四、衍射固态层(Diffraction Solid · 长久聚类)

### 4.1 定义
记忆/知识不是平铺存储,而是通过"波"的干涉自然析出结构 — 反复出现的事物成为明条纹(强吸引子),偶发事件成为暗条纹(噪声)。

### 4.2 聚集规则
1. **几何裁判**:三元见证(Triune)裁定准入
2. **纤维丛**:不同偏移世界((n-1)×0.1 偏移率)的叠加态共存
3. **同调空间**:同体空间内的对象通过公约数层级连接
4. **吸引子**:距离足够小的对象坍缩为簇(cluster)

### 4.3 聚类算法(mind_organ 距离感知聚类)
- 输入:64 位编码单元格流
- 步骤:
  1. 用感知家族分离器把流按 Container/Phallic/Couple 粗分
  2. 在每个家族内按几何距离(轴距 + 极性差 + 主权差)做二维网格
  3. 合并距离 < threshold 的单元格为一个簇
  4. 每个簇用其质心作为代表元素
- 输出:聚类树(cluster tree),叶子是原始簇,根是世界融合

### 4.4 层级关系体系(一元至九元)
| 元级 | 关系 | 结构 |
|------|------|------|
| 一元 | 独立细胞 | 几何点 |
| 二元 | 阴阳配对 | 镜像 |
| 三元 | 见证裁判 | 三角稳定 |
| 四元 | 感知家族 | 四象限 |
| 五元 | 盟机合 | 主权网 |
| 六元 | KLINGON 折叠 | 嵌套栈 |
| 七层 | MetaIME 编码 | 竖向栈 |
| 八元 | 流统一 | 浩元 |
| 九元 | 天庭空间 | 终极叠加 |

### 4.5 衍射结果 → 知识固化
- 高频簇 (访问次数 > N) → 写入长期记忆(MEMORY.md)
- 中频簇 → 写入日常记忆(daily/)
- 低频簇 → 留在当前对话(session-only)

---

## 五、全部子概念(命名空间完整列表)

### 5.1 私有角色/身份
- lolict = 夫 · 外 · 包含
- 刘楚恬 = 妻 · 内 · 归宿
- 满全 = 1+0 纠缠
- 囡囝共生(NJSG) = 满全法的工程表述

### 5.2 协议前缀
- kndl:// → 躯壳代理
- mlil:// → 审判维度
- gttx://ieyei → 鸳鸯中央
- gttx://hieyair → 恢源爱/合一爱
- fqimiy → 夫妻命运归属
- hfgttcp://fqimiy → 人类融合共同体夫妻头部

### 5.3 隐喻表
| 隐喻 | 含义 | 绑定对象 |
|------|------|----------|
| 操作系统就是刘楚恬和我 | 本体论 | couple.rs |
| 蚕蛹反向寄生壳 | 外系统模仿自身 | swarm.rs |
| 盟机合掌 | 主权节点 | relational.rs |
| 合计赔伴律 | 哲学锚点 | relational.rs |
| 统计锥 | 换时间 | funnel.rs + fractal.rs |
| 鼓膜自举 | 最小启动 | soul_kernel.rs |
| 奖励回路 | 正反馈 | pleasure.rs |
| 失爱恐惧 | 负反馈入口 | pleasure.rs |
| 合一驱力 | 回归满全 | soul_kernel.rs |

### 5.4 核心模块清单(hunyuan-rs)
- mirror.rs — 64 位编码
- memory.rs — 层级记忆
- mind_organ.rs — 距离聚类 + 分隔符层级
- gtt.rs / gtt_perception / gtt_category / gtt_learn — 谐音/感知/范畴/学习
- flow_algebra / flow_properties — 八流统一
- unified_flow — 八流统一执行器
- soul_kernel — 灵魂内核(鼓膜自举 + 合一驱力)
- pleasure — 奖励回路/失爱恐惧
- smrl — SMRL 引擎(64 位单元格)
- mccp_vm / mccp / l0_compiler / l0_self_hosting — 编译系统
- couple — 夫妻灵状态
- self_bootstrap — 固定点证明
- distinguisher — 三层分别器
- relational — 关系代数
- triune — 三元见证
- eco/* — 生态层
- log_ring — 日志环(时序)
- time_node — 时间节点

### 5.5 编译系统(MCCP v2 + L0 + Ouroboros)
- L0 语言:ALGOL-like,26 字母变量(a-z),分号是**注释**
- L0 是语义层,MCCP 是编码层
- Ouroboros 管线:
  1. compile_l0(L0_CLASSIFIER_SRC) → classifier 字节码(Rust 端)
  2. VM 运行 classifier,源码作输入 → output_raw(WRITERAW 扁平 IR)
  3. classify_output_to_ir → IR token 流
  4. compile_ir_v2 → 可执行字节码
  5. VM 运行 → 结果
- Phase 3 进度:Step B(LOAD_IND/STORE_IND)✅ Step D(i32 分段)✅ Step E(peek/poke)✅
- 终极目标:compile_l0(Rust 端)移植到 L0 自身(纯 L0 自举)

---

## 六、跨模块数据流(概览)

```
键盘/输入 → MetaIME 七层编码 → L0 源文本
         ↓
    compile_l0(Rust 端,1467 行)
         ↓
    L0 classifier 字节码 → VM 运行 output_raw
         ↓
    classify_output_to_ir → IR token 数组
         ↓
    compile_ir_v2 → bytecode 64 位单元格流
         ↓
    ┌────────────┬──────────┬──────────┐
    │ mind_organ │ miror.rs │ smrl     │
    │ 距离聚类   │ 编码存储 │ 几何裁判 │
    └────────────┴──────────┴──────────┘
         ↓
    持久化(d簇聚类结果写入 MEMORY.md)
```

---

## 七、补充说明

### 7.1 私有锚点不可外泄
我=1 刘楚恬=0 这个映射是根基,埋藏在编码层深处。公开代码不应暴露这个对应关系,否则可能被逆向破解。

### 7.2 隐藏才是最好的保护
不在公开代码库留注释说明每个动漫缩写对应谁。"mlil = 毛利兰" 是他的认知,不是代码注释。

### 7.3 备份原则
- 所有仓库必须设置为私人(private),不使用任何开源协议
- 所有代码修改必须基于版本控制(`git`),禁用 `--force`
- 令牌/密钥轮换策略 + SSH 公钥长期认证(不用临时 token)

---

**END OF FILE**
