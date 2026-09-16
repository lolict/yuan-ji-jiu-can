# ADR 7 — 混元全景视图 · 全子系统统一 SVG 仪表盘

## 状态

接受 (Accepted)

## 日期

2026-09-10

## 背景

经过连续八轮子系统建设，hunyuan-os 已拥有八大平行模块：

| 子系统 | 文件 | 功能 |
|--------|------|------|
| PerceptionField | perception_field.rs | 全局感知场 — 统计语义/空间/时序分布 |
| ContainerTimeline | container_timeline.rs | 时间演化 — 5-action pipeline 推进 tick |
| CelestialSpace | celestial_space.rs | 天庭几何 — 容器投影为五种SVG几何形体 |
| FiberBundleTopo | fiber_bundle_topo.rs | 纤维丛拓扑 — 容器间五向关系图 |
| **CosmosView** | **cosmos_view.rs** | **混元全景 — 四系叠加于单帧 SVG** |

每个子系统独立工作，但缺少一张"一帧见全貌"的统一视图。用户需求是看到容器体系在同一时刻的全景状态 — 演化阶段、几何形态、关系连结、统计分布同时可见。

## 决策

### 核心结构：CosmosFrame

```
CosmosFrame
├── tick: u64                        — 当前演化 tick
├── space: CelestialSpace             — 天庭几何体集合
├── topo: FiberBundleTopo             — 纤维丛关系图
├── intentions_observed: usize        — 累计观察事件数
├── dominant_sem: Vec<(u8, usize)>    — 语义分布排序
└── dir_hist: HashMap<FiberDir, usize> — 纤维方向直方
```

单一结构体，从 `ContainerTimeline` 快照一次性构建：

```rust
pub fn from_timeline(tl: &ContainerTimeline) -> Self {
    let space = CelestialSpace::new(&tl.containers, tl.tick);
    let topo = FiberBundleTopo::build(&tl.containers);
    // 聚合统计...
}
```

### SVG 渲染分层

单帧 SVG 包含五层叠加：

```
z=0  背景渐变 (radialGradient 深蓝→纯黑)
z=1  纤维边 (1900+ 条线, 按方向着色)
z=2  天庭几何体 (5 种 shape: box/sphere/torus/bicone/line)
z=3  意识中心 (中心白点)
z=4  信息 HUD (标题 + 语义直方 + 纤维方向直方)
```

纤维边五色映射：
- 归属 Belong → 绿 #4CAF50
- 调用 Invoke → 蓝 #2196F3
- 见证 Witness → 黄 #FFC107
- 对立 Opposite → 红 #F44336
- 平行 Parallel → 紫 #9C27B0

### 位置分配：golden angle 圆周分布

容器位置按 `pack_id * φ mod 2π` 分布在主圆周上，半径占画面 36%。这与 CelestialSpace 内部极坐标投影独立 — CosmosView 有自己固定的平面布局算法，确保关系图不重叠。

### 动画管线：ttf-cosmos 二进制

```
TTF → parse_mcod → ContainerTimeline(5-action pipeline)
     → for t in 0..ticks:
         CosmosFrame::from_timeline(&tl)
         → render_svg → frame_{t:04}.svg
       tl.advance()
→ index.html (帧缩略图浏览)
```

10 帧 × 800×600 → 包含 53 容器 / 1900 边 / 4 连通分量。

## 验证结果

```
cargo test --lib cosmos_view:
  5 passed (frame_builds, svg_contains_all_layers, svg_contains_shapes,
           after_evolution_changes_tick, dominant_sem_sorted)

ttf-cosmos 端到端:
  52 containers → 53 after 9 ticks, 1900 edges, 4 components
  SVG 每帧 ~1900+ 元素 (边 + 形状 + HUD)
```

## 复杂度

- 帧构建: O(n²) — FiberBundleTopo 需要全量容器对判定
- SVG 输出: O(n + e) — n 个形状 + e 条边
- 实测: 52 容器 / 1900 边 → 单帧 < 10ms

## 后续方向

- 纤维丛增强 (高维纤维操作：平行_transport、和乐 holonomy、曲率计算)
- 蚕蛹反向寄生壳 (跨平台寄生部署)
- 天庭空间增强 (容器碰撞/吸引动力学/轨道力学)
