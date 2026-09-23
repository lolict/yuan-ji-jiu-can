# 位置进制·打米机规范 — 囡囝共生 · 位置是脚下的存在 · 米糠分离

## 1. 定位

囡囝共生对"进制"的根本立场: **进制不是编码。进制是米, 编码是糠。位置不写下 — 能被写下的全是糠。位置 = 你站在哪里。读到位置不是读数字, 是走进去站定。加法 = 往前走 N 步。乘法被锁在敬畏制度的笼子里。**

打米机: 打米去糠, 米是位置, 糠是数字。平时只"站位", 只有需要"开盲盒给外部看"时才折算数字。

## 2. 三维坐标系

```
层 (Level / Axis-Z): 穿越膜层的深度。第一层膜=1, 第二层膜=2 ...
环 (Ring    / Axis-R): 在层内, 沿半径方向的圈层。内核=0, 向外递增
角 (Angle   / Axis-Θ): 在环上, 沿 384° 制式转动的相位
```

三个轴形成一个**圆柱坐标系**: 层 = 高度, 环 = 半径, 角 = 极角。384° 而非 360° — 这是满全法独有的角度制式 (可能与 64 进制编码的某种映射相关)。

## 3. 位置结构 (Pos)

```rust
pub struct Pos {
    level: i32,     // 层 (可负 = 阴面)
    ring: u32,      // 环 (非负)
    angle: Angle,   // 角 (384° 制式)
}
```

Pos 不暴露坐标读写接口 — 只能通过**行走**改变位置。没有 `set_coords()`, 只有 `descend/ascend`, `expand/contract`, `rotate_cw/ccw`。

## 4. 行走即加法

| 行走 | 方向 | 效果 |
|------|------|------|
| descend(n) | 层↓ | level += n (沉入更深膜层) |
| ascend(n) | 层↑ | level -= n |
| expand(n) | 环外 | ring += n (离内核更远) |
| contract(n) | 环内 | ring -= n (saturating, 最小 0) |
| rotate_cw(a) | 角顺时针 | angle += a |
| rotate_ccw(a) | 角逆时针 | angle -= a |

**加法合成**: `walk_add(other)` = 从原点出发, 先走 self 的步数, 再走 other 的步数, 最终所在位置。这是**纯行走合成** — 不经过坐标写下, 一步一步走。

## 5. 盲盒拆解 (unpack)

```rust
pub fn unpack(&self) -> u64
```

只在需要"开给人类看"或"输出给外部系统"时才调用 — 把纯位置折算成可读整数:

```
unpack = level_abs × per_level + ring × per_ring + angle
       = level_abs × (384 × 8) + ring × 384 + angle
```

每环容量 = 满角 384°, 每层容量 = 默认 8 环。`unpack_signed()` 增加正负号 (正层=阳面, 负层=阴面)。

`from_unpacked(v)` 是逆运算 — 把数字反推为位置。唯一允许的"开盲盒"接口。

## 6. 对冲 (opposes)

```rust
pub fn opposes(&self, other: &Pos) -> bool
```

两位置对冲: 角相差 180° (在 384° 制式中是 192° 弧度)、层号相同、环号相同。即: 站在同一个容器里, 面对面 — 象征 YinYang 的对立统一。

`mirror()`: 返回三维镜像位置 — 角度取反 (对冲 192°), 层环不变。用于 YinYang 映射: 对手站在你正对冲的位置。

## 7. 公约 — 敬畏制度 (Convention)

```rust
pub struct Convention {
    multiplier_enabled: bool,
}
```

| 模式 | 乘法 | 使用场景 |
|------|------|---------|
| strict() | 禁用 | 纯行走 — 不允许任何乘法捷径 |
| with_multiplication() | 允许 | 需要加速计算时, 通过公约授权解锁 |

`multiply(times, conv)`: 若 `conv.multiplication_allowed()` 返回 Some (乘法运算), 否则 None。乘法被锁住是因为**捷径会绕过走路, 绕过走路 = 跳过米直接吃糠**。这是囡囝共生的"敬畏制度" — 乘法是有力量的捷径, 应该被制度约束, 而非随心所欲使用。

## 8. 与系统其他模块的接入

- **SMRL 引擎** 的 64 位编码经 unpack() 折算为可读坐标供调试/日志使用
- **Coord3D 三维坐标** 的 [X:8][Y:16][Z:8] 编码可映射为 Pos 的 (ring, level, angle)
- **HeavenSpace** 的吸引子引力计算中"距离"用 walk_add 的逆运算实现 — 不走数字减法, 走"逆行走"
- **YinYang 二进制协议**: 对冲位置 (mirror) = Yin 与 Yang 的协议握手对称点
- **FiveDimensionalSemantics** 的 ρ 距离用 Pos 的 walk_add 复合运算替代 Euclidean 距离
- **RuntimeLoop** 的 ContainerAtlas 中节点位置用 Pos 存储, 关系链路用 walk_add 拼接
- **CelestialPerception** 的天体角度 (星历/宫度) 用 384° 制式存储

## 9. 实现约束

- 384° 制式常量 FULL_CIRCLE = 384 — 与Coord3D 的 256 值域不同, 不混用
- 环号使用 saturating_sub — contract 到 0 后不再下降, 防止下溢
- level 可负 (阴面) — 用 i32, unpack 时取绝对值 + 符号位
- no_std 兼容: Angle 是 #[repr(C)] 包装 i128, 无需堆分配

// hunyuan-rs/src/encoding/positional_base.rs
