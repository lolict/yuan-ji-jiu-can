# ADR 8 — 纤维丛操作 · Transport · Holonomy · Curvature

## 状态

接受 (Accepted)

## 日期

2026-09-12

## 背景

FiberBundleTopo (ADR 6) 构建完毕，但仅有静态连接 — 缺乏沿路径平移、绕圈累积、曲面弯曲三件套。

## 决策

### Phase — 复数近似

```rust
pub struct Phase { pub re: f64, pub im: f64 }  // cos θ + i sin θ
```

### 五向相位映射

| 方向 | 角度 | 含义 |
|------|------|------|
| Parallel | 0 | 无旋转 |
| Belong | π/4 | 归属聚合 |
| Invoke | π/2 | 正交跳转 |
| Witness | π | 镜像翻转 |
| Opposite | -π/2 | 反向跳转 |

### 三件套

1. **Transport (平行移动)**: 沿路径搬运属性，每边按方向旋转相位
2. **Holonomy (和乐)**: 绕闭合回路总相位偏移；deficit > 0 → 有曲率
3. **Curvature (曲率)**: 三角形回路的 holonomy deficit 平均

## 验证

```
cargo test --lib fiber_bundle_ops → 5 tests pass
ttf-holonomy → 52 containers, 全局曲率=1.219, 42/50 triangles 非平凡
```
