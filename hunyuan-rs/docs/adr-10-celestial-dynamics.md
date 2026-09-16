# ADR 10 — 天庭碰撞动力学 · 容器宇宙模拟

## 状态

接受 (Accepted)

## 日期

2026-09-12

## 背景

CelestialSpace (ADR 5) 提供静态投影, CosmosView (ADR 7) 提供统一视图。
但容器是死的星点 — 缺少力的交互。NJSG 命题: 容器不仅是存在, 它们是活的。

## 决策

### PhysicsBody

质量 = payload 和 × (主权 + 1)
极性: +1 (阳) / -1 (阴)，由 payload 首字节最低位决定

### 三件套

1. 引力: F = G × m1 × m2 / r² × polarity_factor
   异性 pol=+1 吸引, 同性 pol=-0.3 弱排斥
2. 碰撞: distance < r1 + r2 + MERGE → 合并 (动量守恒)
3. 轨道: Euler 积分, DT=0.01, 阻尼, 限速, 边界反弹

## 验证

ttf-dynamics: 52 容器, 200 步, 最终 8 体束缚系统, 能量 -517631
