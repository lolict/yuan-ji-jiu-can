# 公约坐标张量引擎 — 囡囝共生 · 预加载大客车 · 8×8×384 制式 · 共享坐位 · 范围扫描

## 1. 定位

区分"私约坐标"与"公约坐标"两种策略: 私约 = 一辆跑车拉一件快递(每次 new 独占空间, 资源浪费); 公约 = 一辆大客车拉上万件(预分配, 万千符号共享). 公约坐标张量是满全法"预加载 = 自己定义自己"的物理基础 — 启动时一次性排位置, 不是收到信息后才决定.

对应 Rust 模块: encoding/coord_tensor.rs.

## 2. 制式配置 (TensorConfig)

- 满全法标准制式: 8 层 × 8 环 × 384 角 × 坐位容量 1 = 24576 个独立位置
- 高密度拼车模式: 8×8×384 × 坐位容量 8 = 196608 个符号挂载点

每层 8 环, 每环 384 角(制式=384°). 坐位容量是"公交车拼车" — 同一个槽位住多个符号, 默认 1(纯主权), 可扩展到 8.

## 3. 坐标引用 (CoordRef)

三维离散化引用: (level_idx, ring_idx, angle_idx). 不是编码后的数值, 而是真正的下标三元组, 在 0..8 / 0..8 / 0..383 范围内. 

线性索引 `linear_index(cfg) = level × (num_rings × num_angles) + ring × num_angles + angle`. 这个乘法是预计算阶段的地址数值换算, 由 TensorConfig 提前算出并缓存 — 运行时纯加法偏移.

## 4. 坐位 (Seat)

```rust
struct Seat {
    coord: CoordRef,         // 坐位坐标
    occupants: Vec<u64>,     // 住在坐位上的符号 ID 列表
    capacity: usize,         // 最大容量
    occupied: bool,          // 是否有符号
}
```

board(symbol_id) — 入住, 满座拒绝; depart(symbol_id) — 退房, 按 ID 删除. board 后 occupied = true, depart 后若 occupants 空则 occupied = false.

capacity 控制拼车程度: 容量 1 = 一坐一位(纯主权, 标准制式); 容量 > 1 = 高共享(大客车模式).

## 5. 公约坐标张量 (CoordTensor)

```rust
struct CoordTensor {
    config: TensorConfig,
    seats: Vec<Seat>,         // 所有坐位的平坦数组
    timeline: HashMap<u64, Vec<usize>>, // 时间戳 → 坐位索引(时段占用)
}
```

### 5.1 核心操作

- `seat_at(coord) -> Option<&Seat>` — 直接下标访问(预分配数组)
- `seat_at_mut(coord)` — 可变借用
- `board(coord, symbol_id) -> bool` — 入住指定坐位(委托给 Seat.board)
- `occupants_at(coord) -> &[u64]` — 查看坐位上的所有人
- `find_idle(ring_level) -> Option<CoordRef>` — 在指定层/环中找到第一个空位(顺序扫描)

### 5.2 线性 view

`as_linear() -> &[Seat]` — 把三维张量摊平为一维数组, 用于范围扫描. 三维坐标到线性索引的计算在 CoordRef::linear_index 中已经完成(加法公式, 不涉及乘除运算).

### 5.3 时间不冲突检测

`check_time_conflict(tick_start, tick_end, coord) -> bool` — 查询某时段内某坐位是否已被占用. 时间 → 坐位索引的反向映射表. 判断: `occupants_at(coord).is_empty() && !timeline_overlap`.

这是"公交车时刻表" — 同一个槽位在不同时段可以住不同的符号, 但同时在同一槽位的符号数受限于 capacity.

## 6. 范围扫描 (RangeScan)

```rust
struct RangeScan {
    level_range: (u32, u32),
    ring_range: (u32, u32),
    angle_range: (u16, u16),
}
```

四种扫描模式:
- Rectangle — 长方体范围(逐层逐环逐角全包)
- RingSlice — 整环抽取(层范围固定, 某几环全角度)
- LevelSlice — 整层抽取(某层全环全角)
- ArcSegment — 扇区抽取(某层某环的角度子集)

`execute_on(tensor)` → 送回调到范围内的每个 Seat. 例如: "在第 3-5 层的所有环, 角度 60-120 的范围找到所有空位, 逐一返回".

## 7. 工业化效率原则

- 预分配: 启动时一次性排 24576 个 Seat(约 24576 × sizeof(Seat) ≈ 393KB), 避免每次收到信息才分配
- 共享坐位: 同一坐标可住多个符号(拼车), capacity 控制最高密度
- 主客分离: 知道自己的坐标 ≠ 能定义别人的坐标. 预加载定义自己的位置 = 主权; 不知道坐标 = 被别人定义

`board` 是"我要上这辆车"; capacity 已满 = 车已满(拒绝上车). 不需要 Rust 的 ownership 系统约束, 满座拒绝是业务层判定.

## 8. 与系统其他模块的接入

- **Positions/Base(#58)**: Positional Base 的公约数体系 — 公约坐标系与 Positional Base 的 384° 制式对齐
- **Container IR(#63)**: ContainerRef 的 octant(卦限)字段 = CoordRef 的 level_idx
- **HeavenSpace(#16/天庭空间)**: 天庭的"角落"(HeavenSpace.Coord3D)与 CoordTensor 的 (level, ring, angle)同构
- **Temporal Engine(#时序)**: timeline 哈希表的 key 对应 HierarchicalTimestamp 的 tick
- **MemoryRevolution(#71)**: symbol_id = MemoryEngine 全局索引中的符号 ID

## 9. 实施细节

- 384 = 制式角度数(与 Positional Base 统一), 因为 384 = 2^7 × 3, 被 1/2/3/4/6/8 整除, 方便公约数分层
- max_symbols_per_seat 控制坐位粒度, 默认 1(纯主权). 拼车模式下可达 8(高密度)
- timeline 反向映射支持"哪些坐位同时被占用"的冲突检测, 但不保证强一致性(跨坐位依赖上层事务)
- 预分配后 seats Vec 容量不变, 无运行时 realloc; 容量上限 = num_levels × num_rings × num_angles = 24576

// hunyuan-rs/src/encoding/coord_tensor.rs
