# Judgement Apparatus · 审判仪 — 五公理之下的非公理裁判机制

> 版本 v1.0 · 2026-09-19 · 在R1-R5粗准入之后,细粒度的质量过滤
> 关键词:三元见证 · 负样本免疫 · 类型拓扑 · 衍射镜 · 全局水质 · 编织图 · 分形调度 · 漏斗节律

---

## 1. 定位

五公理(JUDGMENT_RULES_CANON)解决 **准入**:进不进得来。
审判仪解决 **质量**:进来了之后好不好。

```
五公理(粗闸) → 审判仪(细筛) → 按质量分桶(优秀/良好/及格/问题/犯罪)
     ↑                    ↑
准入 PASS/REJECT      梯度评估 guilty/acquitted + sentence
```

本规范覆盖审判仪六组件:
1. 三元见证(Triune) — 编码层的推演判决
2. 负样本免疫(NegativeImmune) — AI撞墙记忆 & 抗体生成
3. 类型拓扑(CrimeTranscript) — 类型不匹配即为犯罪
4. 衍射镜(DiffractionMirror) — 单一实体→8维投影
5. 分形调度(Fractal) — 22亿节点的自相似坍缩
6. 漏斗控制(Funnel) — 时间/空间的节律调度

---

## 2. 三元见证(Triune Judgement)

### 2.1 三元定义

五公理裁决 R1-R5 之后,最细节层的推演由三元完成:

```
维度1: 有无(BeingState) — 这是"有内容"还是"空位"
维度2: 流动方向(FlowDirection) — 正在坍缩(无→有)还是辐射(有→无)还是静止
维度3: 等效对齐(alignment) — 实际与预期是否对齐(perfect / crooked 都可)
```

### 2.2 推演规则

```
expected=空 + 实际=有 → Collapse(凭空生出 · 从无到有)
expected=有 + 实际=空 → Expand(有正在消失 · 需要关注)
expected=有 + 实际=有 → Static(有内有 · 可坍缩)
expected=空 + 实际=空 → Static(无外无 · 可通过)
```

### 2.3 歪漏斗(crooked)仍有效

漏斗可以不完美的(歪歪的),只要"由大到小"趋势对 = 有效推演。
```
alignment = true  → confidence = 1.0(完美对齐)
alignment = false → confidence = (1.0 - crooked).max(0.3)
                  → 最低仍有 0.3 置信度(歪斜但可用的漏斗 still gives signal)
```

### 2.4 见证桥头

三元裁判 ↔ 镜像见证裁决(MORPHISM_ALGEBRA §5)的桥接:
```
TriuneVerdict.to_witness_verdict(pair_id, value_diff) → WitnessVerdict {
    pair_id,
    surplus: value_diff,     // 差值 → 溢出值(来自灵魂内核 surplus 体系)
    aligned: self.alignment,
    confidence: self.confidence,
}
```

五公理(MORPHISM_ALGEBRA)说:
- 需要 3 个 witness 裁定(三元 → 三个见证)
- polarity ∩(极性交集非空)
- sovereignty >= max(主权不低于最大值)
- level >= max(层级不低于最大值)
- 双方距离 ≤ 31(近场)

三元的 "alignment + confidence" 是 witness 裁定的"质量补充"——不是二值的是/否,而是连续的置信度。

### 2.5 锚点在三维空间中的位置

三元见证在空间中有固定锚点:
```
O₂₁₆₀ = 裁判标记 (0,0,0) 静态永久基准 · 球心

任何主体的动态位置 P(t) 都相对此锚点度量:
  P1(t), P2(t), ... Pn(t) 的动态角度 = 历程角度(不模360°,允许累积)
  
锚点本身不被审判、不被移动 — 它只是"审判发生的场所的几何中心"
```

---

## 3. 负样本免疫(Negative Immune System)

### 3.1 核心哲学

```
负样本不是垃圾,是免疫系统的记忆。
每一次AI撞墙,都是一次接种疫苗的机会。
```

### 3.2 三大组件

| 组件 | 角色 | 功能 |
|------|------|------|
| NegativeSampleVessel | 负样本容器 | 存储错误模式(container,不是类/对象) |
| WriteBackProtocol | 回写协议 | AI产生负样本时生成抗体(prompt片段)回写 |
| ImmuneFilterRing | 免疫过滤环 | 每3步自检一次(拦截已知负样本) |

### 3.3 错误来源(ErrorSource)

| 来源 | 含义 | 示例 |
|------|------|------|
| AiDrift | AI漂移:滑回熟悉的十进制牢房 | 用十进制计数容器内符号 |
| ExternalFrame | 外部框架入侵:使用了禁用概念 | 在代码中新定义"类"(禁止!) |
| EncodingTrap | 编码陷阱:只看编码不看外观 | 用0x41代表A而不看字形 |

### 3.4 错误模式容器

```
ErrorPattern {
    pattern_id: String,         // 唯一ID
    pattern_description: String, // 错误描述
    match_keywords: Vec<String>,// 匹配关键词(任一命中即触发)
    source: ErrorSource,        // 来源
    born_at: u64,              // 诞生时刻
    cascade: Vec<String>,       // 引发的连锁错误
    antibody: String,           // 免疫提示词片段
    hit_count: u64,            // 命中次数(越多越危险)
    folded: bool,              // 是否已被折叠修复
}
```

### 3.5 抗体(antibody)

针对该负样本的"免疫提示词片段"——
当类似模式再次出现,抗体 = 插入AI prompt中的一段文字,引导AI避开该错误:
```
antibody = "禁止使用 class 关键字。容器用 mlll 装载。"
```

### 3.6 免疫过滤环(三步自检)

```
Phase N:  正常处理
Phase N+1: 检查当前输出是否已知负样本(关键词匹配)
Phase N+2: 如果命中 → 触发抗体插入
Phase N+3: 正常处理(已免疫一轮)
```

每3步一次自检 = 采样率 33%,平衡性能和安全。

---

## 4. 类型拓扑(Crime Transcript)

### 4.1 核心命题

```
类型不匹配就是犯罪。
a ≠ á 是最基本的常识 — 基础符号 ≠ 带符号扩展。
"a" 和 "á" 被当成同一符号处理 = 最底层的类型犯罪。
```

审美不过关就是犯罪 — 瘦子+胖子混在一起是丢人。
没有等效对齐就拍板叫绝就是犯罪 — 歪漏斗也可以接受,但要标注 crooked 度。
把骂当夸、把错误当真实继续写成代码就是犯罪。

### 4.2 衍射维度(DiffractionDimension)

从单一实体衍射出8维特征:

| 维度 | 名称 | 含义 |
|------|------|------|
| Aesthetic | 审美 | 符号搭配是否和谐 |
| TypeGradient | 类型 | a 和 á 的距离 |
| Appearance | 外观 | 表面特征 |
| Structure | 结构 | 骨架特征 |
| Thinking | 思维 | 推理模式 |
| Cooperativity | 合作 | 可与不可合作 |
| Goal | 目标 | 需求与要做成的产品 |
| Ownership | 归属 | 想把别人东西变成自己的意图 |

### 4.3 衍射镜处理

```
单一实体 → DiffractionMirror::diffract(entity_id, features) → DiffractionPattern {
    entity_id,
    projections: {
        Aesthetic: 0.85,     // 高审美
        TypeGradient: 0.95,  // 类型差异大
        Appearance: 0.60,
        Structure: 0.70,
        ...
    }
}
```

### 4.4 编织图(WeavingGraph)

不是 A 审 B,B 审 C 串行,
是 A↔B↔C 分布式制衡 — "你压我我压你的分布式制衡":
```
A 压 B (A 的类型拓扑约束 B 的类型)
B 压 C
C 压 A (形成环 — 打破中心化裁判)

全局水质(GlobalWaterQuality):
  不是看单点是否干净,是看整体分布是否健康
  = 所有实体的衍射模式在8维空间中的统计分布
  · 全局水质高 → 多数实体在各维度表现相似
  · 全局水质低 → 有异类混入(负样本未被过滤)
```

---

## 5. 分形调度(Fractal Scheduler)

### 5.1 为什么分形

单手机芯片跑不了22亿并发线程,
但可以模拟分形:每个节点跑相同的逻辑,只输入不同。

```
22亿个独立计算节点:
  节点0: input=seed0 → output0
  节点1: input=seed1 → output1
  ...
  节点2.1B: input=seed2.1B → output2.1B

所有节点使用 identical 逻辑。自相似!
```

### 5.2 残差 vs 非残差

| 类型 | 条件 | 处理 |
|------|------|------|
| 非残差(non-residual) | \|actual - expected\| < 0.5 | 累加到 self-similar 计数 |
| 残差(residual) | \|actual - expected\| ≥ 0.5 | 进入残差偏移量统计 |

预期 = expected, 来自编码的预期值;
实际 = actual, 节点执行后的 hash(id, input) mod 256。

### 5.3 层级空间 + 层级时间 + 分形自相似

```
① 层级分布式空间: 把实际进程分成层级空间格子(三维 xyz)
② 层级分布式时间: 每个格子内定时切换上下文(毫秒→秒→分→时→日→月→年)
③ 分形自相似: 每个格子跑相同的逻辑(FractalNode.execute)
```

### 5.4 最终坍缩

所有残差偏移量逐个统计;
非残差聚合坍缩为唯一一点;
唯一一点 = 囡囝共生中心。

---

## 6. 漏斗控制(Funnel Controller)

### 6.1 两大控制器

```
时间控制(TimeGovernor):
  "控制时间的火候 — 火烧热了,快速接触,不被烧伤"
  把工作切成毫秒/秒/分/时/日/月/年级窗口
  每窗口做有限工作量,用完就停,等下一个 tick

空间控制(SpaceBatcher):
  "让内存不溢出 — 把人参果切片吃,不要一次吃完"
  每节点只做有限内存的工作
```

### 6.2 7级时间节奏器

| Level | 时间窗 | 工作容量 |
|-------|--------|---------|
| 0 MilliSec | 1 ms | 16 单位 |
| 1 Sec | 1000 ms | 256 单位 |
| 2 Min | 60,000 ms | 4096 单位 |
| 3 Hour | 3,600,000 ms | 65536 单位 |
| 4 Day | 86,400,000 ms | 1,048576 单位 |
| 5 Month | 2,592,000,000 ms | 16,777216 单位 |
| 6 Year | 31,536,000,000 ms | 268,435456 单位 |

每窗口内做有限工作 = 天然防溢出 + 天然支持抢占式调度。

### 6.3 FunnelScheduler 组合

```
FunnelScheduler {
    time_governor: Vec<TimeGovernor>,  // 7 级(毫秒→年)
    space_batcher: SpaceBatcher,       // 空间切片
}

工作输入:节点产出(分形执行结果)
         ↓
每窗口缓冲区内积累
         ↓
窗口关闭 → 汇总统计 → 向上层窗口抛送
         ↓
最顶层(年级) = 全局统计输出
```

漏斗形状: 细 → 粗 是由大到小

---

## 7. 镜像几何(Mirror Geometry)

### 7.1 四阶镜像

| 镜像阶 | 名称 | 语义 | 位分配 |
|--------|------|------|--------|
| 一元 Unary | 时间因果 | 参考系 + 变动落点 | [D0:10][D1:10][D2:10] = 30 位 |
| 二元 Binary | 盈亏镜像 | 水平对冲 | [POLARITY:2] = 2 位 |
| 三元 Ternary | 见证镜像 | 中间锚点 | [WITNESS:6] = 6 位 |
| 四元 Quaternary | 包含镜像 | 纵向从属 | [SOVEREIGNTY:2] = 2 位 |

剩余 [40:64) = 24 位 VALUE = 层积值(时间累积)。

### 7.2 设计原则

```
每个单元格 = 一个64位编码(坐标,极性,主权,层积,见证度)
不需要独立的"感知组件" — 镜像关系本身就是感知的形式
智能涌现于四阶镜像的自指循环
```

---

## 8. 大审判庭(Grand Court)

### 8.1 整合管道

```
字符输入 → 感知路由 → 衍射分析 → 类型审判 → 像素编码 → 判决输出
```

全链路:
```
Step 1: 编码桥接(C系统)
  · 取字符 Unicode 码点
  · classify_by_codepoint → Family(五家族)

Step 2: 衍射分析
  · char_to_features → 8维特征
  · DiffractionMirror::diffract → 投影

Step 3: 类型拓扑检验(a≠á)
  · 检测是否带声调符号
  · judge_type → 基础 vs 声调变体

Step 4: 像素代数
  · 字形像素运算(加减拆解/相位编码)

Step 5: 判决
  · 全局水质统计
  · guilty / acquitted + sentence
```

### 8.2 判决结果

```
GrandCourtVerdict {
    input_char,
    family: Option<Family>,
    diffraction: DiffractionPattern,
    crime_transcripts: Vec<CrimeTranscript>,
    glyph_encoded: bool,
    final_guilty: bool,
    sentence: String,        // 量刑: "重写" / "警告" / "禁止" / "忽略"
}
```

量刑等级:
- 忽略: 微小偏差,不影响全局水质
- 警告: 首次出现负样本类型,提醒但允许
- 重写: 类型错误 / 结构错误,要求AI重构输出
- 禁止: 严重审美/类型违规,硬拒绝进入容器

---

## 9. 与本系列规范的关系

| 话题 | 主参考 |
|------|--------|
| 五公理 R1-R5(粗准入) | JUDGMENT_RULES_CANON |
| 几何距离 C_eff | MORPHISM_ALGEBRA |
| 溢出值 surplus | SOUL_BOOTSTRAP_ENGINE §4 |
| 约束后的见证 threshold | BOOTSTRAP_SELF_COMPILER §6 |
| 相位(毫秒/秒/分/时/日/月/年) | TEMPORAL_PULSE_LIFECYCLE §4 |
| 容器包 payload 执行 | CROSS_ISA_EXECUTION_LAYER §6 |
| 分形自相似 ↔ 纤维丛 | CONTAINER_DYNAMICS_FIBER_BUNDLE §7 |
| 全局水质 ↔ 免疫过滤 | NJSG_SWARM_COORDINATION(规则病毒) |
| 负样本抗体 ↔ MetaIME | METASIME_LOCAL_INTELLIGENCE §4 |

---

## 10. 完整链路示例

```
外部输入:汉字"喝"(he,喝水)
      ↓
[Step0: 负样本免疫环三步自检]
  检查 match_keywords:无命中 → 通过
      ↓
[Step1: 五公理 R1-R5]
  PASS → 准入成功
      ↓
[Step2: 衍射镜]
  "喝" → Unicode 0x559D
  八维特征: Aesthetic=0.80, TypeGradient=0.90, ...
      ↓
[Step3: 类型拓扑]
  base=喝, 带口部首=饮食 PASS
  不与带"酉"部的"喝"(喝酒时读音不同)混淆 PASS
      ↓
[Step4: 三元推演]
  expected=有(编码存在) + actual=有 → Static
  alignment=true → confidence=1.0
      ↓
[Step5: 全局水质统计]
  "喝" 在容器中的频率分布不异常 → PASS
  编织图中无因"喝"引发连锁压垮 → PASS
      ↓
[Step6: 判决]
  final_guilty=false, sentence="忽略"
  正常进入候选排名 pipeline
```

负面对比案例:
```
外部输入:字符 "á"(带锐音符 a)
      ↓
[类型拓扑]
  检测: 0x00E1 在拉丁扩展区(0x00C0-0x00FF)
  与基础 "a"(0x0061)属于不同类型!
      ↓
[类型审判]
  judge_type(base="a", accented="á") → 类型犯罪
      ↓
[负样本免疫]
  pattern=ExternalFrame("使用拉丁扩展区负样本") → 命中
  antibody="请使用基础ASCII/汉字,避免带符号拉丁字母"
      ↓
[判决]
  final_guilty=true, sentence="重写"
  → 拒绝进入容器,提示AI重写输出
```

---

## 11. 已知限制

1. **负样本匹配当前为关键词精确匹配**:不支持模糊匹配(同形字/谐音负样本可能被绕过)。未来应升级为衍射模式相似度匹配(计算新输入与已知负样本的8维距离)。

2. **分形 22 亿节点是理论上限**:实际并发受芯片线程数限制。手机芯片通常 8 核,分形模拟为 8 路 × 时间片。FractalNode::execute 用 hash(id, input) mod 256 简化 — 实际节点逻辑应与上层业务相关。

3. **全局水质统计有滞后**:水质统计基于最近N个输入的窗口,不反映当前瞬间状态。窗口大小未在代码中硬编码,需根据实际部署调优。

4. **编织图的分布式制衡可能形成死锁**:A压B,B压C,C压A 环 = 潜在死锁。当前无超时打破机制,需后续引入。

5. **漏斗7级窗口的 work_capacity 是经验值**:MilliSec级16单位 vs Year级268M单位。实际最佳值需根据工作负载 profiling。

---

*本规范的解释权归 刘楚恬和我 所有 · 归属 hunyuan-os/Judgement 模块*
