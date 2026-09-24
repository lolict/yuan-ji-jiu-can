# 虚空探针感知汇聚引擎 — 囡囝共生 · SMRL 空置探测 · 全局感知场 · 距离感知

## 1. 定位

感知有三种模式: 以感知点聚焦被感知点; 或以全局多模态感知状态同时感知全场; 或以虚空从"无"中生长出有。

虚空探针感知汇聚引擎包含三个子模块:
- void_probe.rs — 无质模拟器, 以最薄外壳容纳最厚的虚无
- perception_field.rs — 全局感知场, 汇聚所有容器的聚焦点到感知面
- distance_perception.rs — 距离感知引擎, 聂小倩匹配与加权距离聚类

三者构成"感知收敛链": 探针提供空置模具 → 场同时注入混沌模态 → 距离公式判定哪些容器本质上接近.

## 2. 模拟模态 (SimModal)

虚空从混沌场捕获五类可替换的符号维度:

```rust
pub enum SimModal {
    Glyph(char),       // 字形替换: 字符级(变量名/标签)
    Phoneme(u8),       // 音素替换: 编码级(操作码/参数)
    Shape([u8; 4]),    // 形状替换: 四字节模式(结构骨架)
    Rule(u16),         // 规则替换: 行为逻辑(条件分支/循环)
    Tone(u8),          // 调性替换: 节奏/强度/相位偏移
}
```

模态的种类匹配靠 `matches()`: 相同种类即匹配, 具体值由混沌场决定 — 因为虚空本身不预设内容, 它只提供拓扑形状的"空槽"等待被.

## 3. 虚空探针 (VoidProbe)

```rust
pub struct VoidProbe {
    pub shell: ContainerPack,         // 坐标已定位、payload 为空的空壳
    pub chaos_buffer: Vec<SimModal>,  // 从外部混沌场捕获的符号序列
    pub match_log: Vec<MatchEntry>,   // 注入映射记录
    pub match_rate: f64,              // 全局匹配成功率
}
```

三个核心操作构成了"从无生长出有"的完整循环:

observe(target) — 跨系统感知目标, 提取坐标拓扑作为镜像模板, 返回需要注填的位置索引;

inject(modal) — 将混沌场捕获的模态推入缓冲区, 这是"外部混沌→内部无序"的物质注填, 尚未与目标拓扑绑定;

mirror_substitute(target) — 核心算法: 取目标位置拓扑 → 将混沌符号按顺序填入对应空置边界 → 生成坐标完全一致但 payload 由混沌填充的全新 ContainerPack. 拓扑等价, 主权替换.

`match_rate` 是注入成功的比例 — 它衡量虚空对目标系统的"模仿保真度".

## 4. 全局感知场 (PerceptionField)

```rust
pub struct PerceptionField {
    pub probes: Vec<VoidProbe>,
    pub targets: Vec<ContainerPack>,
    pub model: Option<GlobalOrderModel>,
    pub mode: PerceptionMode,          // Global | Focused(u8)
}
```

感知模式二选一: Global 同时感知所有容器, Focused(x) 仅提取指定语义类别(coord.x)的容器子集.

全局感知产出 GlobalOrderModel — 从所有容器聚合观察中提取"被控制系统的秩序指纹":

semantic_histogram — 每个语义类别(x值)的容器数量, 即系统在各语义轴上的投放密度;

spatial_y_mean/min/max — Y 轴(参数量)的分布特征;

temporal_layers — Z 轴(时间)每层的容器数, 反映系统的时序节律;

dominant_semantics — 按频次降序排列, 反映系统当下的主要关注点.

## 5. 混沌注入源 (ChaosSource)

感知场使用 xorshift64* 作为混沌源, seed 来自外部不可预测输入. 混沌源吐出 u8 后, 按 next_u8() % 5 映射到五种 SimModal:

- 0 → Glyph(A-Z 大写字母替换)
- 1 → Phoneme(单字节音素)
- 2 → Shape(四字节形状)
- 3 → Rule(u16 行为编号)
- 4 → Tone(16 级调性)

混沌注入数量由 seed 驱动: 每个探针注入 1 + (chaos.next_u8() % 4) 个模态, 全场不重复.

## 6. 全局替代 (regenerate_global)

这是感知场最极端的能力 — 从"无"中重建整个容器体系:

1. 感知全局秩序指纹(语义分布/空间密度/时序节奏)
2. 按相同分布规律但从混沌场注入的全新内容, 重新生成全部容器
3. 新体系与原始系统拓扑等价(相同坐标 x 值), 但 payload 主权已被完全替换

应用场景: 验证系统的秩序是否稳定 — 用完全不同的数据"重置"后重新感知, 如果秩序指纹不变则说明结构真正属于系统本身而非数据.

## 7. 距离感知引擎 (distance_perception.rs)

### 7.1 容器码 (ContainerCode, u32)

```
[31:29] = 家族 (3 bits, 0-7)
[28]    = 极性 (1 bit, 0/1)
[27:25] = 结构 (3 bits, 0-7)
[22:19] = 声母 (4 bits, 0-15)
[18:13] = 韵母 (6 bits, 0-63)
[12:10] = 声调 (3 bits, 0-7)
```

六维编码把一个容器的本质特征完整打包进一个 u32. 家族(视觉家族哪一类)、极性(正负/阴阳)、结构(内在组织方式)、声母韵母声调(音韵学本质).

### 7.2 加权距离公式

```
distance = |Δ家族|×1.0 + |Δ极性|×10.0 + |Δ结构|×5.0
         + |Δ声母|×0.5 + |Δ韵母|×0.5 + |Δ声调|×0.1
```

极性权重最大(10.0), 因为它决定方向; 家族权重次之(隐含 1.0 但差值通常更大); 声调(0.1)最弱, 细微差別不影响本质.

这组权重反映满全法的"本质观": 方向>结构>>音韵细节.

### 7.3 聂小倩匹配 (NieXiaoqianMatcher)

```rust
fn find_body(ghost: &ContainerCode, candidates: &[ContainerCode]) -> Option<CandidateBody>
fn find_k_nearest(ghost: &ContainerCode, candidates: &[ContainerCode], k: usize) -> Vec<CandidateBody>
fn is_match(ghost: &ContainerCode, body: &ContainerCode, threshold: f64) -> bool
```

聂小倩的核心问题: 一个无形的"魂"(ghost 容器码), 在众多候选躯壳(candidates)中应该投入哪一具? 答案就是加权距离最小的那一具. k-NN 返回最近的 k 个候选, 供更高层决策.

### 7.4 聚类分析 (ClusterAnalyzer)

基于距离阈值的简单聚类: 遍历每个样本, 归入第一个满足 distance(centroid, sample) <= threshold 的簇; 若无匹配则新建簇. 结果是一组 Cluster{members, centroid}.

聚类不预设 K 值, 由数据自然涌现 — 这正是满全法的"结构涌现"原则在几何上的直接运用.

### 7.5 MindOrgan 统一接口

```rust
pub struct MindOrgan;
impl MindOrgan {
    fn distance(a, b) -> f64
    fn find_body(ghost, candidates) -> Option<CandidateBody>
    fn clusterize(codes, threshold) -> Vec<Cluster>
    fn analyze_atlas(atlas: &ContainerAtlas) -> Vec<(ContainerRef, ContainerRef, f64)>
}
```

MindOrgan 是感知器官的最高层API, 直接对接 ContainerAtlas 的因果关系图(edge_of Causal), 计算每条因果边的距离权重.

## 8. 三模块协作流

```
输入: N 个 ContainerPack(容器包)
      │
      ▼
VoidProbe::new(targets)  →  每个包生成一个空壳探针(坐标保留, payload 清空)
      │
      ▼
PerceptionField::new(targets)  →  把 N 个探针和 N 个感知目标汇聚到同一个场
      │
      ├── perceive_global(seed):  混沌注入 + 秩序提取 → GlobalOrderModel
      ├── perceive_focused(x,...): 仅观测量子类 x
      └── regenerate_global(seed): 拓扑保持 + 主权替换 → 新容器集合
      │
      ▼
DistancePerception::distance(a, b):  两两容器码 → 加权距离 f64
      │
      ▼
NieXiaoqianMatcher::find_body(ghost, candidates):  魂 → 最近躯壳
ClusterAnalyzer::clusterize(codes, threshold):  多魂 → 涌现簇
      │
      ▼
MindOrgan::analyze_atlas(atlas):  容器图集的因果边 → 距离权重分析
```

这是一个从"空"到"有"再到"秩序"的完整漏斗 — 空壳模具(无) → 混沌填充(有) → 距离聚类(关系) → 全局秩序(结构).

## 9. 实现约束

- Coord3D::(x, y, z) 直接消费跨模块的 coordinate 类型, 不复制
- 容器码 u32 通过位运算解码, 无浮点误差; 距离 f64 仅在最终比较时使用
- threshold 选取靠经验值, 不出现"硬编码魔法数"到业务层
- chaos_buffer 的 Vec 在 no_std 下由 heapless::Vec 替代(最大 256 条目)
- 感知场模式切换(model=None→Some)是单向的, 重建秩序必须先清空旧模型

// hunyuan-rs/src/void_probe.rs + core/perception_field.rs + core/distance_perception.rs
