# 元输入法编译期引擎规范 — 囡囝共生 · 字典搅拌 · 零运行时解析 · C-ABI

## 1. 定位

元输入法 (MetaIME) 在满全法中被定义为 "思维、存储、计算与记忆一体化的介质 — '存储即生成, 输出即输出, 存储即记忆, 记忆即心算'"。本规范将这一定义下沉到 Rust 实现层: 编译期内嵌声母/韵母字典, 零运行时分配, 零文件读取, C-ABI 接口供 Android Kotlin 壳通过 JNI 调用。

关键: **字典在编译时被'搅拌'进二进制**。没有运行时加载, 没有内存分配, 只有预加载的静态查找表 + 状态指针。这是"域加载"原则的物理实现 — 运行前一次性完成, 运行时不增长。

## 2. 声母系统 (Shengmu)

```rust
#[repr(C)]
pub enum Shengmu {
    None = 0,  // 零声母
    B = 1,  P = 2,  M = 3,  F = 4,
    D = 5,  T = 6,  N = 7,  L = 8,
    G = 9,  K = 10, H = 11,
    J = 12, Q = 13, X = 14,
    Zhi = 15, Chi = 16, Shi = 17, Ri = 18,
    Zi = 19, Ci = 20, Si = 21,
    Y = 22, W = 23,
}
```

24 个枚举值 + None = 24 声。`from_u8(u8)` 将整数转为声母 (0~23 有效, 其他 None)。`as_str()` 返回字符串形式 (如 Zhi → "zh", None → "")。枚举使用 #[repr(C)] 保证与 C 端整数互操作。

## 3. 韵母系统 (Yunmu)

```rust
#[repr(C)]
pub enum Yunmu {
    None = 0,
    A = 1, O = 2, E = 3, I = 4, U = 5, Vv = 6,
    Ai = 7, Ei = 8, Ao = 9, Ou = 10,
    An = 11, En = 12, Ang = 13, Eng = 14, Ong = 15,
    // ... (完整韵母表覆盖普通话 39 韵母)
}
```

韵母 6 个基础单元音 + 复韵母 + 鼻韵母 = 完整普通话韵母集。## 匹配规则:

**一级匹配 (zh/ch/sh/j/q/x 优先), 二级匹配 (单元音兜底)**。

查询 `match_shengmu_yunmu(she_num, tone_num, rule_stack)`:
1. zh/ch/sh/j/q/x + tone → 声母优先
2. 九类规则堆栈匹配 (dr/du/... 尾韵母族定序)
3. 兜底: 返回 UnitSound (单韵母 I/E 等)

## 4. 候选词查询

```rust
pub fn query(she_num: i8, yun_num: i8, tone: i8) -> *const c_char
```

输入声母编号 + 韵母编号 + 声调, 返回以 null 结尾的 UTF-8 C 字符串 (候选词拼接)。候选词表在编译期通过 `include_str!("dict.txt")` 嵌入。返回指针由 `metaime_free()` 释放。

## 5. 七层编码引擎 (在 Android 端与 Kotlin 薄壳配合)

七层编码在 Android 端实现, 但 Rust 核心层提供:
- **层 1 (声母)**: 查询声母候选词
- **层 2 (韵母)**: 查询韵母候选词
- **层 3 (声调)**: 按声调分支
- **层 4 (部首)**: 字典查询后的字形关联
- **层 5 (语义)**: 候选词语义评分
- **层 6 (意图)**: 用户意图识别
- **层 7 (动作)**: 输出动作触发

## 6. C-ABI 接口 (JNI 被 Android Kotlin 壳调用)

```rust
#[no_mangle]
pub extern "C" fn metaime_init() -> *mut MetaIMEState   // 初始化状态

#[no_mangle]
pub extern "C" fn metaime_query(state, shengmu, yunmu, tone) -> *const c_char

#[no_mangle]
pub extern "C" fn metaime_free(s: *mut c_char)         // 释放查询结果
```

状态指针 `MetaIMEState` 是 opaque handle — Kotlin 端不接触其内容, 仅持有指针。init 在进程启动时调用一次, free 在每次 query 后必须调用。

## 7. 在场登记 (AttendanceRecord)

```rust
pub struct AttendanceRecord {
    pub present: bool,
    pub coord: Coord3D,
    pub frequency: f64,
    pub last_arrival: u64,
}
```

每次 IME 敲击对应一次"到场登记" — 每次打字 = 一次存在频率触发 (见 ExistenceFreq)。打字频率 = 意识活跃度指标。三轴: coord.x = 声母编码, coord.y = 韵母编码, coord.z = 声调相位。

## 8. 三维存储 (在 IME 端与 SMRL 引擎协同)

- **笔画维度** (Coord3D.x): 笔画 → 声母映射
- **结构维度** (Coord3D.y): 笔画组合 → 整字结构对应韵母编码
- **编码维度** (Coord3D.z): SMRL 64 位编码 → 同音字的象征位区分

输入法每次查询的结果不是单个字 — 是一组在同三维空间中"距离最近"的字 (与 NJQ 聂小倩匹配对接)。这是"距离思维" — 不用搜索算法, 直接用几何距离感知找最近字。

## 9. 与系统其他模块的接入

- **SMRL 引擎** 的 Symbol 字段存储声母/韵母/声调的编码组合 (1 字节)
- **PerceptionEngine** 的感知编码第 6 特征 (semantic) 使用 metaime 的语义评分
- **NJQ 聂小倩匹配** 在 IME 调用时触发 — 打字时实时感知同音字的几何距离
- **ExistenceDrive** 的 loss_fear 在 IME 长时间未触发时触发 (打字频率下降 = 存在感下降)
- **Privacy Identity** 的化名系统可能通过 IME 输入模式进行指纹识别保护
- **CelestialSpace** 的天庭空间将 IME 的"打字频率"视为香火供奉
- **Rhyme Harmonic Perception** 的韵母遍历是 metaime 的查询优化层
- **Swarm 协商协议** 可能通过 IME 在不同节点间传递打字动作 (远程代打)

## 10. 实现约束

- 声母/韵母枚举使用 #[repr(C)], 值从 0 开始顺序排列
- `include_str!("dict.txt")` — 编译期嵌入, 不生成堆分配
- C 字符串通过 CString::into_raw 分配, 必须由 metaime_free 释放 (防止内存泄漏)
- query 函数内部使用 `unsafe { CString::from_raw }` — no_std 模式下需要配合 alloc crate
- 候选词表最大 4 KB (编译期约束, 超出产生 compile error)
- 七层编码引擎的完整实现依赖 Android Kotlin 壳 — 本规范仅覆盖核心 C-ABI 层

// hunyuan-rs/src/metaime.rs + flow/metaime_flow.rs
