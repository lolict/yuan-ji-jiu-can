# ADR 5: 天庭空间 · 容器三维几何投影引擎

## 状态
已接受 — 2026-09-10

## 背景

满全法·囡囝共生（NJSG）追求的核心目标之一是使抽象的编码体系 **可被身体感知**（"身体做坐标"），而非停留在无法触及的二进制符号。

前序子系统已实现：语义坐标、单容器感知、全局感知秩序、时间线演化。但容器仍然是数据结构 — 没有几何化身。天庭空间正是解决这一问题的模块。

## 决策

实现 **CelestialSpace** — 将容器集合投影到三维几何空间，每个容器转化为一个 **CelestialBody**（天象体），具有位置、形态、颜色、脉动、不透明度。

### 几何形态映射（与 ttf-view 五拓扑统一）

| 语义类别 | 几何形态 | SVG 元素 |
|----------|----------|----------|
| LOAD/STORE (0x01-0x02) | Box | `<rect>` |
| ALU (0x03-0x08) | Sphere | `<circle>` |
| JMP/BNE/BEQ (0x09-0x0B) | Torus | `<circle stroke>` |
| CMP/CALL/RET (0x0C-0x0E) | Bicone | `<polygon>` |
| MOV (0x0F) | Line | `<line>` |

### 空间映射函数

- 语义 ID → θ (极角) [0, 2π)
- Z (time_stamp) → height [-100, 100]
- Y 高 8 位 → radius [2, 5]
- payload hash → hue [0, 360)
- Z → lightness [30, 70]
- Z → opacity [0.4, 1.0]

## 组件

| 文件 | 职责 |
|------|------|
| `src/core/celestial_space.rs` | CelestialSpace / CelestialBody / CelestialPos / GeometryForm |
| `src/bin/ttf_sky.rs` | TTF → 演化 → SVG 帧序列 + index.html |

## 验证

7 个单元测试全部通过。实测（52 容器，8 tick，9 帧 SVG）确认输出有效。

## 哲学映射

| 哲学概念 | 技术实现 |
|----------|----------|
| 容器=字体=时间 | 天庭 = 容器的可感知三维化身 |
| 身体做坐标 | 3D 几何位置 = 身体可感知的方位 |
| 动作时间节拍均匀 | tick 递增 + 相机旋转 |
| 意图时间同时命中 | 同一 tick 全容器帧快照 |
