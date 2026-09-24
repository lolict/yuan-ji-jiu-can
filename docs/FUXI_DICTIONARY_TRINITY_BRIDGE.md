# 伏羲字典字形三元桥梁 — 囡囝共生 · 汉字 ↔ 容器编码 · 三维寻址 · 全表征统一查找

## 1. 定位

系统中存在三个平行的编码空间: 家族/极性/结构/声母/韵母/声调(ContainerCode/u32)、毛利兰立体坐标(MaoriCoord/x,y,z)、统一中间表示(ContainerRef/u128). 它们分别服务于不同的子系统: 距离感知用 ContainerCode, 毛利兰寻址用 MaoriCoord, 全局注册用 ContainerRef.

伏羲字典的命题是: **一个汉字必须能够同时在其所有编码形式之间自由转换**. 不是靠 to_string + re-parse(代数选址), 而是一次性从同一个 FNV-1a 种子哈希派生出全部字段, 保证三空间的一致性.

## 2. 汉字全表征 (FuxiChar)

```rust
pub struct FuxiChar {
    pub hanzi: String,        // 汉字本身
    pub unicode: u32,         // Unicode 标量值
    pub code: ContainerCode,  // 容器码(家族/极性/结构/声母/韵母/声调)
    pub coord: MaoriCoord,    // 毛利兰立体坐标
    pub cref: ContainerRef,   // 容器中间表示
    pub label: String,        // 显示标签
}
```

一枚汉字就是一座六路桥梁: visual form → unicode scalar → distance code → 3D space → universal IR → label. 任意一路改变, 其余五路都能通过重新计算 FNV-1a 立即看出变动.

## 3. 哈希派生流水线 (FNV-1a 32-bit)

```rust
fn fnv1a_32(data: &[u8]) -> u32 {
    let mut hash: u32 = 0x811c9dc5;
    for &b in data {
        hash ^= b as u32;
        hash = hash.wrapping_mul(0x01000193);
    }
    hash
}
```

FNV-1a 被选中是因为: 单遍遍历, 无浮点, 对单字节变化极敏感(雪崩效应), 同时在 wasm32 和有符号环境下行为一致.

三次哈希产生三个独立的派生值:

h1 = fnv1a_32(utf8_bytes)
h2 = fnv1a_32([h1.lo, utf8_bytes, 1])    // 加盐 1 → 独立
h3 = fnv1a_32([h1.hi, h2.lo, utf8_bytes, 2]) // 加盐 2 → 独立

三个独立哈希值分别驱动三条派生链, 互不影响.

## 4. 三条派生链

### 4.1 汉字 → 毛利兰坐标 (hanzi_to_coord)

```
x = h1 % 32   → 横轴: 语义类别(0-31)
y = h2 % 32   → 纵轴: 参数量/强度(0-31)
z = h3 % 32   → 竖轴: 时间/层级(0-31)
```

x/y/z 各自在 0-31 内均匀分布(前提是 FNV-1a 的质量保证), 构成一个 32×32×32 的毛利兰离散实体网格.

### 4.2 汉字 → 容器码 (hanzi_to_code)

从 h1/h2 按位域切出 6 个字段:

```rust
family    = (h1 >> 29) & 0x7     // [31:29] 3 bits, 0-7
polarity  = (h1 >> 28) & 0x1     // [28]    1 bit,  0/1
structure = (h1 >> 25) & 0x7     // [27:25] 3 bits, 0-7
shengmu   = (h1 >> 19) & 0xF     // [22:19] 4 bits, 0-15
yunmu     = (h2 >> 13) & 0x3F    // [18:13] 6 bits, 0-63
tone      = (h2 >> 10) & 0x7     // [12:10] 3 bits, 0-7
```

这些位域直接对应 ContainerCode::compose 的布局.

### 4.3 汉字 → 容器引用 (hanzi_to_cref)

```rust
entity = (h1 as u64) << 64 | (h2 as u64)  // 完整 64 位实体编码
octant = (h1 >> 29) & 0x7                  // 卦限 = 家族号
ContainerRef::with_octant(Maori, entity, octant)
```

ContainerRef 的 octant 字段直接复用家族号 — 卦限与视觉家族对齐, 系统与距离感知引擎的家族划分.

## 5. 伏羲字典 (FuxiDictionary)

四重索引结构:

```rust
pub struct FuxiDictionary {
    by_hanzi: HashMap<String, FuxiChar>,     // 正向: 汉字 → 全表征
    by_code: HashMap<u32, Vec<String>>,      // 反向: 容器码 → [汉字]
    by_coord: HashMap<u64, Vec<String>>,     // 反向: 打包坐标 → [汉字]
    by_cref: HashMap<u128, Vec<String>>,     // 反向: 容器引用 → [汉字]
    count: usize,
}
```

四个 HashMap 分别对应四条查询路径:

find_by_hanzi — 知道是什么字, 查询全部编码形式(核心 path);

find_by_code — 距离感知容器码, 反查对应哪几个字(多对一, 哈希碰撞允许);

find_by_coord — 毛利兰坐标反查(坐标打包: x<<40 | y<<20 | z);

find_by_cref — 容器引用反查(用于统一 IR 注册后的字还原).

### 5.1 注册法则

register() 把一个字一次性落地四重索引. 先派生 code/coord/cref, 再分别推入 by_code/by_coord/by_cref/的 Vec<String>, 最后插入 by_hanzi. `count = by_hanzi.len()` — count 始终跟随正向表, 不单独分配.

批量注册 register_batch() 遍历调用 register, 保持幂等: 同一字再次注册先覆盖 by_hanzi, 后刷新三个反向索引(旧条目先被推入, 相同项可多份; 反向查询结果用 filter_map 去 None).

### 5.2 坐标打包公式

coord_key = (x as u64) << 40 | (y as u64) << 20 | z as u64

每个轴占 20 bits — 足够覆盖 0-31 的范围, 且三轴唯一组合. 该打包与毛利兰寻址系统的 64 位线性寻址对齐.

## 6. 与系统其他模块的接入

- **NieXiaoqianMatcher**: find_body 时的 ghost/candidates 就是 ContainerCode, 由伏羲字典从汉字派生
- **DistancePerception::distance**: distance_ref_to_code 通过 ContainerRef.entity() 反推 ContainerCode, 路径为 ContainerRef → entity 低 32 位 → 位域切分 → 容器码
- **FuxiDictionary → ContainerIR 双向同步**: 每次 register, ContainerRef 同时注册进 ContainerRuntime
- **SymbolTable(#60汉字→字形)**: SymbolTable 提供 fallback 汉字(干支/Unicode几何符号), 伏羲字典把这些 fallback 汉字也编码为全表征
- **PerceptionField**: 感知目标的 coord.x 由 hanzi_to_coord 而来, 感知模式 Focused(x) 的 x 与坐标系中语义类别对齐

## 7. 实现约束

- FNV-1a 32-bit 已在 wasm32 实测, 无端序问题
- HashMap 在 no_std 下替换为 HashMap 的 hashbrown 实现(已是 std 内建)
- register_batch 大量插入时建议预分配容量
- code_raw/cref_raw 用于跨模块比对时先用 u32/u128 raw, 避免 String 重新分配
- describe() 返回格式含坐标/卦限/容器码, 便于调试期可读

// hunyuan-rs/src/core/fuxi_dictionary.rs
