# ADR 6 — 纤维丛拓扑 · 容器间五向关系图

## 状态

接受 (Accepted)

## 日期

2026-09-11

## 背景

CosmosView 之前缺少容器之间的关系模型。仅有静态坐标不够 — 需要描述容器之间的"力"。

## 决策

### 五向纤维关系

| 方向 | 条件 | 语义 |
|------|------|------|
| Belong (归属) | 同 X, Y 向有序 | 同族内的层级 |
| Invoke (调用) | 跳转型语义 ID (如 JMP/CALL) | 跨族调用 |
| Witness (见证) | 同 Z (第三维) | 共时性 |
| Opposite (对立) | 极性互反 (yin/yang) | 矛盾 |
| Parallel (平行) | 同语义 ID 不同实例 | 同义重复 |

### 结构

```
FiberBundleTopo
├── nodes: HashMap<u64, FiberNode>
├── edges: Vec<FiberEdge>           (双向, 每对产生 2 条有向边)
└── components: Vec<HashSet<u64>>    (DFS 连通分量)
```

每条边标记方向 + 权重。全量 O(n²) 构建, n=52 时 1924 条边。

### 连通分量

通过 DFS 划分群落。每个分量内部全连通, 分量间无连接。
增加 BFS `path_between(a, b)` 支持两点间路径检索。

## 验证

```
cargo test --lib fiber_bundle_topo → 9 tests pass
ttf-fiber → 52 nodes, 1924 edges, 4 components, 输出 SVG/DOT
```
