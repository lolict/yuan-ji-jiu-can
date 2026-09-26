# 容器=字体=时间 三位一体引擎 — 囡囝共生 · 冰箱触发 · 几何投影 · 范畴论许可表 · 表格编程

## 1. 定位

满全法核心命题"容器=字体=时间"的工程实现。三个子系统(function语义ID层/时间触发冰箱/三维几何投影)被统一到同一个 32 位地址空间, 通过范畴论许可表和表格编程范式来编排容器之间的转移。

对应 Rust 模块: 根级 container_font_unified.rs (src/, 核心统合入口).

## 2. 32 位统一地址 (Address32)

```
[31:24] = X = 8 bits = 语义类别 / 功能ID / 拓扑类型
[23:8]  = Y = 16 bits = 操作数 / 参数 / 变换
[7:0]   = Z = 8 bits = 时间窗口 / 相位 / 视觉
```

三个子系统共享同一个地址内核, 但解释各不同:
- fridge: X=功能ID, Y=操作数, Z=时间窗口
- machine-code: X=语义ID, Y=操作数编码, Z=相位对齐
- glyph-font: X=拓扑类型, Y=变换参数, Z=视觉属性

这 32 位不是"编码", 就是数值本身。`as_u32` / `from_u32` 是恒等操作, 无编解码损耗。

### 2.1 地址折叠 (fold)

fold 操作 = 饱和加法(self.0.saturating_add(other.0)), 模拟容器包在时间触发下的合并. 不进位折叠意味着超过 2^32 时停在 u32::MAX, 不产生溢出.

### 2.2 莫顿码投影

morton_encode_3d(x,y,z) 把三维坐标按 Z 阶曲线交织为一维数值轴上的行走路径 — 不是编码, 是几何投影. 这样三维空间中的"近"直接对应一维数值上的"近", 方便寻址与范围查询.

## 3. 容器包 (ContainerPack)

```rust
pub struct ContainerPack {
    pub pack_id: Container,           // 容器名 = 符号(不是编码)
    pub address: Address32,           // 32位统一地址
    pub payload: PackPayload,         // 预编码动作序列
    pub witness: WitnessSignature,    // 三元裁决签名
}
```

PackPayload 是动作序列(不是源码, 不是字节码):
- Empty: 空袋, 许可=拒绝执行
- Single: 单步动作(一个容器地址)
- Chain(Vec): 链式动作(多地址序列)
- Branch(Box, Box): 分支(条件决定走哪条链)
- Parallel(Vec): 并行(相同时间窗的多链)

预编码意味着"动作序列在编译期已经确定", 运行时只做裁决与执行, 不重新解析.

## 4. 容器地址 (ContainerAddress)

在 64 容器空间中的精确坐标 (x:i8, y:i8, z:i8) + Container 类型.

```rust
pub fn to_address32(&self) -> Address32 {
    Address32::new(
        self.x as u8,
        ((self.y as u16) << 8) | (self.z as u16),
        self.container as u8,
    )
}
```

ContainerAddress → Address32 是直接数值拼装, 不涉及任何编码转换.

fold 操作: 饱和加法各分量, 主导容器不变. 这是"容器包在空间上靠近但不混合"的操作.

## 5. 三维几何体投影 (Geometry3D)

从 32 位地址**直接**投影为三维几何体, 不经过任何中间编码:

```rust
Geometry3D::project(addr) → {
    topology:  Topology::from_semantic_x(addr.x()),  // X 语义ID → 拓扑形状
    rotation:  从 Y 分量提取 (0-359° 满映射)
    scale_x/y: 从 Y 分量提取 4 bit 各
    color_phase: 从 Z 高 4 bit
    pulse_freq:  从 Z 低 4 bit
}
```

每一种操作码类型对应一种三维几何:

| X 语义ID | 操作类型 | 拓扑形状 | 几何参数(V/E/F) |
|---------|---------|---------|----------------|
| 0x00 | NOP | Point | 1/0/0 |
| 0x01-0x02 | LOAD/STORE | Box | 8/12/6 |
| 0x03-0x08 | 算术逻辑 | Sphere | 1/0/1 |
| 0x09-0x0A | JMP/BNE | Torus | 0/2/1 |
| 0x0B-0x0C | CALL/RET | Tube | 0/2/3 |
| 0x0D | CMP | Bicone | 2/1/2 |
| 0x0E | MOV | Line | 2/1/0 |
| 0x0F | — | Grid | 4/4/1 (表格) |

"语义→形状"是满全法"容器=字体"命题的最直接工程表达 — 一个函数在三维空间中的几何形态完全由其语义类型决定, 没有主观"设计".

## 6. 许可态射表 (MorphismTable)

采用范畴论许可表实现"容器之间的态射(转移)判定":

```rust
pub enum MorphismKind {
    Allowed,                // 许可 — 路径连通
    Denied,                 // 不许 — 路径断裂
    Conditional(Polarity),  // 条件许可 — 需特定极性才允许
}
```

query(from, to) → 在 morphism 表中查找从 from 容器到 to 容器的态射类型. 默认 Allowed(没在表中的转移视为允许).

init_default_morphisms 定义了容器等级链的基底态射:
- 袋→桶→罐→盆→盂→壶→盒→瓶 全 Allowed — 这是"容器进位"的阶梯式允许
- 某些跨级转移为 Conditional(需特定极性)
- 反向(如笼→袋)为 Denied — 容器位不可逆向流失

permits() 是 MorphismTable 到布尔的简化: Conditional(p) 满足当 p == Yuan 或 p == Yang.

## 7. 统一容器世界 (UnifiedContainerWorld)

把所有子系统整合到一个运行上下文:

```rust
pub struct UnifiedContainerWorld {
    packs: Vec<(Address32, ContainerPack)>,         // 容器包存储
    geometries: Vec<(Address32, Geometry3D)>,       // 几何体缓存
    morphisms: MorphismTable,                        // 范畴论许可
    time_cursor: u8,                                 // 当前时间光标(Z 轴)
}
```

核心操作:

register_pack(addr, pack) — 注册容器包;

get_pack / get_geometry / geometry_or_project — 读存储或直接投影生成几何体;

execute_at(addr) — 时间触发检查: 仅有 pack.address.z() == time_cursor 的容器包才能被执行, 返回 witness.verdict()(Recognized/Rejected/Waiting);

advance_time() — 推进 time_cursor, Z 轴循环;

scan_executable() — 沿 Z 轴扫描, 返回所有当前时间窗口匹配的可执行容器包.

PackBuilder 提供标准的容器包构造器, 默认 WitnessSignature: sovereignty=ActiveSub, soul_state=FullCouple, polarity=Yuan.

## 8. 三维表格编程单元 (TableCell3D)

```rust
pub struct TableCell3D {
    pub address: Address32,
    pub permission: MorphismKind,
    pub content: Option<Container>,
    pub geometry_topology: Topology,
}
```

三维表格编程的导航方法:
- next_on_x() → 沿 X 轴(语义类别)移动
- next_on_y() → 沿 Y 轴(参数/变换)移动
- advance_z() → 沿 Z 轴(时间光标)推进

acts_on(other, world) — 查询两个单元格的内容容器之间是否存在允许的态射. 两个单元格都有内容容器时, 调用 world.permits(我的容器, 对方容器).

这是满全法"表格编程"概念的形式化: 表格中的每个单元格是一个三维地址+容器, 单元格之间的"作用关系"由范畴论许可表决定.

## 9. 与系统其他模块的接入

- **ContainerIR(#63)**: ContainerRef(kind=Maori) 的本体坐标与 Address32 对齐, ContainerPack 可序列化为 ContainerState
- **InvocationTable(#61)**: 呼唤表执行的指令地址 = Address32(Nop/PushI/Load/Store等)
- **FUMT(#51/5)**: 三元裁决(WitnessSignature)的 Verdict 输出决定是否准入 FlowEntry
- **FuxiDictionary(#65)**: Topology::from_semantic_x 的 X 字段的语义类别与 FuxiDictionary 的家族字段同源
- **ActiveContainer(#62)**: polarity 三元(Yin/Yuan/Yang)与本引擎的 Conditional(polarity)对接
- **Cognition/HeavenlySpaceModule(天庭)**: Geometry3D 的坐标与 HeavenSpace 的 well/attractor 概念共享"三维几何定位"

## 10. 实现约束

- Address32 的 32 位统一地址与(u8, u16, u8)三元组互转无精度损失
- MorphismTable 初始化态射不可变(运行期新增用 add())
- time_cursor 在 u8 范围内循环(0-255), 跨容器包依靠 Z 相同同步触发
- geometry_or_project 是 lazy evaluation: 首次访问时投影生成, 之后从缓存读取
- pack.payload 的 Chain/Parallel 在 Chain 中嵌套深度不受限(但实际应用 ≤ 4 层)
- no_std: Vec 替换为 heapless::Vec (packs 最大 256 条目)

// hunyuan-rs/src/container_font_unified.rs
