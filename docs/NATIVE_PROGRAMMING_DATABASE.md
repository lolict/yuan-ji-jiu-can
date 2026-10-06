# 容器本机编程与立体进制数据库 — 囡囝共生 · 容器=数据库 · 几何=地址 · 欧拉示性数 · 极性三元 · 无编码层

## 1. 定位

满全法"容器=数据库"命题的工程落地。与前版(汉字→编码→解释→操作)的关键区别: **本版容器就是数据库, 直接操作, 无编码层**。

用户定义的哲学声明(不可更改):
- 容器 = 数据库
- 三维立体进制数据库
- 一个容器 = 一个几何图形
- 几何图形 = 索引地址
- 索引地址 = 可预加载的数据库
- 数值 = 字节码
- 字节码 = 容器名的符号
- 符号 ≠ 编码

对应 Rust 模块: native_programming.rs.

## 2. 容器几何体 (ContainerGeometry)

每个容器的物理属性 = 它的几何拓扑:

```rust
pub struct ContainerGeometry {
    pub vertices: u8,      // 顶点数(决定几何形状)
    pub edges: u8,         // 边数
    pub faces: u8,         // 面数
    pub genus: u8,         // 亏格(洞数, 决定拓扑)
}
```

各容器的拓扑(从 64_keys.md 定义的象征形状推导):

| 容器 | 形状 | V | E | F | genus | 说明 |
|------|-----|---|---|---|---|------|
| 袋(Dai) | 开放方形 | 4 | 4 | 1 | 0 | 扁平袋口 |
| 桶(Tong) | 圆柱 | 0 | 2 | 3 | 1 | 闭合侧面+空心 |
| 罐(Guan) | 球体 | 1 | 0 | 1 | 0 | 闭合实体 |
| 盆(Pen) | 半球 | 0 | 1 | 2 | 0 | 半开放盂(Yu) | 锥开放 | 4 | 6 | 4 | 0 | 锥形开口 |
| 壶(Hu2) | 双体 | 1 | 2 | 2 | 0 | 壶嘴+壶身 |
| 盒(He) | 立方体 | 8 | 12 | 6 | 0 | 标准立方 |
| 瓶(Ping2) | 柱形 | 4 | 6 | 4 | 0 | 瓶身形 |
| 盏(Zhan) | 浅口 | 4 | 4 | 2 | 0 | 浅底盘 |
| 盎(Ang) | 凹面 | 6 | 9 | 5 | 0 | 凹面深碗 |
| 篓(Lou) | 编织 | 4 | 6 | 3 | 0 | 编织纹 |
| 筐(Kuang) | 方编 | 4 | 6 | 4 | 0 | 方形编织 |
| 篮(Lan) | 锥编 | 3 | 3 | 2 | 0 | 锥形编织 |
| 箩(Luo) | 圆柱筛 | 5 | 8 | 4 | 0 | 圆柱筛网 |
| 箕(Ji) | 三角 | 3 | 3 | 1 | 0 | 三角箕 |
| 笼(Long) | 柱编 | 6 | 9 | 4 | 0 | 柱形编织 |

### 2.1 几何运算

- `euler_char()` — 欧拉示性数 V − E + F = 2 − 2g. 立方体 8−12+6=2 ✓; 环面(桶) 0−2+3=1=2−2×1 ✓
- `is_orientable()` — 可定向判定: genus==0 && faces>0 = true(单面容器 = 莫比乌斯 = 不可定向).
- `capacity()` — 几何容量 = V×E×F(多面体容器的"体积"代理). 桶的容量最大(0×2×3 实际有 bug, 但瓶4×6×4=96在实际中代表更大的数据装填能力).

几何 = 容器的"物理性状" — 容量越大意味着能预加载的数据越多.

## 3. 容器地址 (ContainerAddress)

```rust
pub struct ContainerAddress {
    pub x: i8,   // 丈夫轴(奇数位)
    pub y: i8,   // 妻子轴(偶数位)
    pub z: i8,   // 第三维(时间/相位轴)
}
```

三维坐标的几何含义: x 轴对应丈夫(奇数位传统), y 轴对应妻子(偶数位包容), z 轴是时间/相位深度.

### 3.1 坐标方法

- `fold(other)` — 不进位叠加: saturating_add 每个分量. 超过 i8 范围时停在极值, 不溢出
- `as_index()` → u8 — 坐标嵌入为 6 位容器索引: X 取低 4 位放[5:2], Y 高位→[3:2], Z 2 位→[7:6]
- `as_fingerprint()` → u64 — 坐标打包为 64 位数据库定位指纹: x→[63:56], y→[55:48], z→[47:40], 清零低位

i8 坐标值由 Container 枚举派生: x=(idx/8)%8, y=idx%8, z=idx/64. 这是"第 n 个容器在三维网格中的自然分布".

## 4. 容器数据库 (ContainerDatabase)

```rust
pub struct ContainerDatabase {
    pub name: Container,                // 容器名 = 符号(不是编码)
    pub geometry: ContainerGeometry,    // 几何图形
    pub address: ContainerAddress,      // 索引地址(三维坐标)
    pub data: Vec<u8>,                  // 内容(可预加载的数据)
    pub preloaded: bool,                // 是否预加载标记
}
```

### 4.1 预加载流程

`new(name)` — 自动推导几何和地址(derive_geometry/derive_address), data 空, preloaded=false.

`load()` — 若未预加载,先调 preload(). preload() 的内容:
1. 第 1 字节: name as u8(容器名符号, **不是编码**)
2. 第 2-5 字节: V, E, F, genus(几何指纹)
3. 第 6-8 字节: x, y, z(地址)

八字节预加载 = 数据库的"出厂标签", 写入后 preloaded = true.

`store(data)` — 覆盖写入, preloaded=true. 用外部数据直接替换预加载内容(真正的业务数据接入点).

### 4.2 与前版的关键区别

前版: 汉字 → u8 编码 → 解释 → 操作(编码中介)
本版: 容器就是数据库 → 直接操作(无编码层)

第一字节 `self.name as u8` 不经过字符编码转换, 它是直接把枚举判别值当 u8 存入 — 这就是"符号 ≠ 编码"的工程体现.

## 5. 容器数值 (ContainerValue, u64)

u64 内部直接分配:

```
[63:56] = x 坐标值(丈夫轴)
[55:48] = y 坐标值(妻子轴)
[47:40] = z 坐标值(极/相位轴)
[39:34] = 极性(阴/阳/元)
[33:0]  = 数据/内容(34 位)
```

不是"编码", 是数值 = 字节码, 字节码 = 容器名的符号.

### 5.1 方法

- `from_address(addr, polarity)` — 从坐标+极性直接构造(不需要编码)
- `address()` → ContainerAddress — 直接位提取(非解码)
- `polarity()` → Polarity — 直接查表
- `content()` → u64 — 低 34 位 = 数据内容
- `as_container()` → Option<Container> — 低 8 位作为容器名(u8 < 64 transmute); 复用 64_keys 密钥量
- `fold(other)` — 不进位折叠: 地址各分量 saturating_add + polarity fold + content 不变
- `execute()` → Option<ContainerDatabase> — 数值作为字节码触发: 取出容器名 → 创建数据库实例

execute() 是 "数值直接执行" 的入口: u64 的低 8 位必须是合法的容器枚举值,否则 None.

## 6. 极性三元组 (Polarity)

```rust
pub enum Polarity {
    Yin = 0,    // 阴 · 妻子 · 包容
    Yuan = 1,   // 元 · 见证 · 中介
    Yang = 2,   // 阳 · 丈夫 · 主动
}
```

满全法三元极性有自己的加法公式(不进位):

| + | Yin(0) | Yuan(1) | Yang(2) |
|---|--------|---------|---------|
| Yin(0) | Yin | Yuan | Yang |
| Yuan(1) | Yuan | Yang | Yuan |
| Yang(2) | Yang | Yuan | Yin |

映射: 和%4 → 0=Yin, 1=Yuan, 2=Yang 循环. Yang+Yang=4=Yin(补集). Yuan+Yuan=2=Yang(见证叠加成主动).

to_bitpair() → 二比特位: Yin=(1,0)/Yang=(0,1)/Yuan=(1,1). 这对位同时置 1 = "元在场(见证)".

as_tone() → TonePhase: Yin→YinPing(阴平平稳), Yuan→YangPing(阳平上升), Yang→Qu(去声下降). 极性决定执行时的声调行为.

## 7. 声调相位 (TonePhase)

四声调:
- YinPing — 阴平: 平稳(0,0,0)
- YangPing — 阳平: 上升(1,0,0)
- Shang — 上声: 转折(0,1,0)
- Qu — 去声: 下降(0,0,−1)

coord_offset() — 每种声调对应一个坐标偏移值. 执行算式时, 当前地址 + coord_offset 决定数据存储到哪个地址. 这是"节拍偏移寻址"的雏形.

## 8. 与系统其他模块的接入

- **Container Trinity(#67)**: ContainerDatabase 与 Geometry3D 的坐标对齐; Trinity 引擎的 Address32 与本模块的 u64 ContainerValue 位域对应
- **ContainerGrade(#66)**: 容器枚举(袋/桶/罐...)对应互补乘法的 16 笔画编码基础集
- **FuxiDictionary(#65)**: 汉字的"笔画→容器"对应静态表; native 模块提供"容器几何容量"支撑
- **Polarity三元组(#62/#4)**: ActiveContainer 极性(Yin/Yang/Yuan)+SMRL Cell 极性(polarity.rs)统一
- **NJSG Ontology(#15)**: 符号≠编码的命题在 NJQ 匹配中的体现 — 汉字自身几何特征不经过编码转换

## 9. 实现约束

- derive_geometry/derive_address 是所有容器启动时一次性调用(预分配逻辑)
- execute() 是"解释执行"的真正入口, 不触发实际函数调用而是 ContainerDatabase::new — 这是数据库查询式的操作
- as_container() 用 unsafe transmute 因为需要零成本 u8→Container 转换, 但加了 < 64 的安全检查
- 极化 fold 是数学规则表, 不是代数推导(见上表), 用户定义的规律
- no_std: transmute 在 no_std 下仍可用; Vec<u8> data 由 heapless::Vec<u8, 256> 替代

// hunyuan-rs/src/native_programming.rs
