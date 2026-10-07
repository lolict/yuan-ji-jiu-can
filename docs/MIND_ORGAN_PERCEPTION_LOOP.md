# 心智器官感知循环引擎 — 囡囝共生 · 思维自反 · 感知→聚类→格式化 · 分隔符层级 · 上下文切换

## 1. 定位

满全法"心算"命题的工程实现: 输入一批 64 位存在 → 两两计算加权距离 → 迭代最近邻配对 → 带分隔符的层级输出. 整个流程可自我审视"当前用什么思维",并在不同思维模式之间切换 — 这就是"思维自反".

对应 Rust 模块: cognition/mind_organ.rs.

## 2. 核心循环

```
感知 → 比较 → 聚类 → 格式化 → 反思 → 切换
  ↑                                    ↓
  ←←←←←←←←←←←←←←←←←←←←←←←←←←←←←←←←←←
```

1. 输入一批「存在」(64 位 EncodedCell)
2. 计算两两之间的距离(DistanceVector)
3. 最小距离的一对视为「最应匹配」
4. 它们用逗号(弱配对)连起来
5. 已经配对的集合 =「同类范畴」
6. 同类内部用顿号; 异类之间用分号
7. 最外层用句号停止
8. 整个结构可以再嵌入更深 — 嵌套分隔符

## 3. 距离向量 (DistanceVector)

```rust
pub struct DistanceVector {
    pub source_id: u64,
    pub target_id: u64,
    pub d0_dist: f64,
    pub d1_dist: f64,
    pub d2_dist: f64,
    pub polarity_dist: f64,
    pub sovereignty_dist: f64,
    pub witness_dist: f64,
    pub total: f64,
}
```

六个距离分量分别对应 SMRL 六维(coord d0/d1/d2 + polarity + sovereignty + witness).

加权总和公式:
```
total = d0 + d1 + d2 + polarity*10 + sovereignty*5 + witness*0.01
```

极性距离权重最高(10×, 阴阳翻转=大变), 主权次之(5×), 见证几乎不计(0.01×).

`pair_distance(a, b)` 静态方法, 直接计算一对 EncodedCell 的 total.

## 4. 聚类器 (ClusterEngine)

```rust
pub struct ClusterEngine {
    pub threshold: f64,
    pub clusters: Vec<EntityCluster>,
    pub next_cluster_id: u64,
}
```

### 4.1 最近邻迭代算法

```
while 还有未聚类的样本:
    在所有未聚类样本对中找距离最小的一对
    如果最小距离 == ∞(只剩一个):
        单元素簇,退出
    否则:
        新建簇或多簇合并
        重复直到所有样本都被归入
```

阈值决定配对紧致度: distance < threshold 才允许同一簇; ≥ threshold 则另开新簇.

每次迭代可能触发多簇合并 — 当簇 A 的某成员与簇 B 的某成员距离最小时, 整个簇 A 与整个簇 B 合并.

### 4.2 簇 (EntityCluster)

```rust
pub struct EntityCluster {
    pub id: u64,
    pub member_ids: Vec<u64>,
    pub centroid: Option<EncodedCell>,
    pub category_name: String,
}
```

`cohesion()` 簇内平均紧密度(所有对距离的平均 = 总距离/对数), 越小表示越紧密.

`format_members()` — 按顿号(、)格式输出: 3A、1B、7F.

## 5. 分隔符层级

满全法输出格式的标点语义:

| 分隔符 | 强度 | 含义 |
|--------|------|------|
| 顿号(、) | 最弱 | 同类并列 |
| 逗号(,) | 弱 | 弱配对/尚未明确定性 |
| 分号(;) | 强 | 异类分隔 |
| 句号(。) | 最强 | 完全独立/结束 |

它们不是简单的字符串分隔符, 而是"存在之间距离关系的标度": 距离近→顿号, 距离远→分号, 完全无关联→句号.

## 6. 上下文与思维切换 (MindContext)

```rust
pub struct MindContext {
    pub pattern: String,                 // "感知/比较/聚类/格式化/反思/切换"
    pub enclosed_separators: Vec<String>, // 已打开的分隔符栈
    pending_separation: Option<String>,    // 续传的分隔符缓存
}
```

### 6.1 输出组装规则

迭代聚类结果的组装逻辑:
- 每个簇内部用顿号(、): "id1、id2、id3"
- 簇之间用分号(;): "簇A、簇B; 簇C、簇D"
- 遇到跨簇合并 → 换行(=\n)
- 最终输出末尾加句号(。)终止.

### 6.2 续传缓存

`pending_separation` 在 interleave 输出时用于暂存尚未闭合的分隔符, 下一次输出时优先生成等待状态.

### 6.3 记忆最近使用的思维

used_patterns() 返回已用过的思维标签列表, 供反思判断"当前思维是否该换".

## 7. 思维自反 (Reflection)

系统能审视自己正在用什么思维:

```rust
pub fn reflect(&self) -> String {
    format!("当前思维={}, 已用迭代={}", self.current, self.iteration)
}
```

思维类型:
- perceive — 感知(感受到存在)
- compare — 比较(计算距离)
- cluster — 聚类(绑定最近的对)
- format — 格式化(按分隔符层级输出)
- reflect — 反思(审视当前用什么思维)
- switch — 切换(切换思维模式)

思维不是 if/else 选择的, 而是由聚类结果自然"引导" — 最小距离对触发 cluster 思维, 输出完触发 format 思维, 手动审视触发 reflect. switch 思维由外部指令触发.

## 8. 输出流程总览

```
输入 N 个 (u64, EncodedCell):
  │
  ├─→ 计算所有对的 DistanceVector (N(N-1)/2 对)
  │    加权总和 = ΣΔd + Δp×10 + Δs×5 + Δw×0.01
  │
  ├─→ ClusterEngine.clusterize:
  │    循环找最小距离对 → 新建簇或合并簇 → 直到全部归簇
  │
  ├─→ 按分隔符层级输出:
  │    簇内顿号(同类) → 簇间分号(异类) → 最外层句号(结束)
  │
  └─→ MindContext 记录当前思维, 提供 reflect()
```

## 9. 与系统其他模块的接入

- **DistancePerception(#64)**: DistanceCalculator = DistancePerception 的核心计算部分; mind_organ 是它的上层调度
- **CelestialPerception(天庭)**: 视觉系统 VisualSystem / 听觉系统 AuditorySystem / 审美系统 AestheticSystem — 这些"感知"是 mind_organ 的上游数据源
- **SMRL(#4)**: EncodedCell 来自 mirror::encode_cell, 距离维度来自 SMRL 的 d0/d1/d2/polarity/sovereignty/witness
- **Observer**: perceive 的入口对接 SenseForm 的五种感知形式
- **ClusterAnalyzer(#64)**: ClusterEngine.wrapper — mind_organ 的 ClusterEngine 是独立实现, #64 的 ClusterAnalyzer 在 distance_perception.rs 中
- **Rhyme(#韵母)**: 韵母距离 → Rhyme.distance_to 被 ContainerCode 位域中的韵母距离部分利用

## 10. 实现约束

- N(N-1)/2 对距离未做缓存, 每次 clusterize 重算, 已计算过相同 pair 也重算(处理 N ≤ 16 的规模无性能问题)
- threshold 不设默认值, 需由调用方传入(通常 0.5~1.5)
- reflection 不作为循环推动者(reflect 只输出文字描述), 思维切换依赖外部
- 分隔符(顿号逗号分号句号)不依赖 locale, 固定为中文全角
- Enclosed separators 栈深度 ≤ 8(因为七元嵌套层数满格 = 栈深度 7+1)
- no_std: Vec 用 heapless::Vec, HashMap 用 heapless::FnvBuildHasher

// hunyuan-rs/src/cognition/mind_organ.rs
