# Cross-ISA Execution Layer · 跨指令集执行层

> 版本 v1.0 · 2026-09-19 · 一套语义 × 多份机器码 · 裁决领域无关
> 关键词:跨ISA对齐 · 三维坐标 · Morton距离 · 词汇表 · 容器包 · 状态群轨道 · 领域无关

---

## 1. 定位

上层规范(SMRL 编码、裁定引擎、在场域)描述了"是什么"。
跨 ISA 执行层解决 **"在哪个物理芯片上跑"** 的问题:

```
语义 ID(Load / Addi / Sub)
  │
  ├──▶ x86-64 机器码: 48 89 C3
  ├──▶ AArch64 机器码: 910003E0
  ├──▶ RISC-V   机器码: 00058593
  └──▶ WASM    机器码: 20 00 20 01 6A
```

同一条语义,四个平台四种编码 — 结果 bit-identical(裁决一致)。

---

## 2. 三维坐标(Coord3D)

### 2.1 位布局

```
[X:8位][Y:16位][Z:8位] = 32位容器地址

X(语义ID)   → 指令类型(Load=0x01, Addi=0x04 ...)
Y(操作数)   → 高8位=旋转角, 中4位=X缩放, 低4位=Y缩放
Z(时间窗口) → 高4位=颜色相位对齐, 低4位=脉冲频率
```

### 2.2 编码/解码

```
to_address():
  [X << 24] | [Y << 8] | [Z]

from_address(addr):
  X = (addr >> 24) & 0xFF
  Y = (addr >> 8) & 0xFFFF
  Z = addr & 0xFF
```

### 2.3 为什么三维不是一维

一维地址:指令顺序排列,地址 = 顺序号。
三维地址:**语义 · 参数 · 时间** 三维独立寻址。

好处:
- 相同 Y(操作数)的不同 X(指令)可批量查表
- 相同 Z(时间)的所有指令可同时刻执行
- 距离计算可以按维度加权(语义差异 vs 参数差异 vs 时间差异)

---

## 3. Morton 编码:空间 ↔ 一维互转

### 3.1 Z-order 曲线

三维坐标 (x,y,z) ↔ 一维 Morton 码:
```rust
// 编码:把 x/z 的位交织(每维 8 位), y 的位交织到中间(16 位)
encode_morton3(x, y, z) -> u32
decode_morton3(code) -> Coord3D
```

### 3.2 为什么用 Morton 不用简单拼接

简单拼接 `x << 24 | y << 8 | z =` 空间邻近 → 数值邻近(**仅在固定两维时成立**)。
Morton:空间邻近 → **任何维度滑动都保持数值近似邻近**。

- 欧氏距离近的两个坐标 → Morton 码距离也近
- Morton 码距离 → 三维欧氏距离近似
- 聚类/查找用 Morton 码做范围查询 = 三维空间范围查询

### 3.3 认知距离 vs Morton 距离

| 距离类型 | 公式 | 用途 |
|---------|------|------|
| 认知距离 | `spatial * 0.6 + polarity * 0.25 + witness * 0.15` | 判断"是否同族" |
| Morton 距离 | `|code_a - code_b|` | 判断"是否在同一空间区域" |
| 有效距离 C_eff | C_coord + C_view + C_level | 综合裁定准入 |

三种距离互相备用 — 认知距离做裁定,Morton 做范围查询,C_eff 做最终裁判。

---

## 4. 跨 ISA 对齐表

### 4.1 统一语义 ID

语义 ID = X 坐标(8位),同时是 **指令的"名字"**,不依赖任何具体平台:

```
══ 基础整数指令 ══
Load=0x01, Store=0x02, Add=0x03, Addi=0x04, Sub=0x05
And=0x06, Or=0x07, Xor=0x08
Jmp=0x09, Bne=0x0A, Beq=0x0B
Call=0x0C, Ret=0x0D, Cmp=0x0E, Mov=0x0F

══ 乘除指令(满全法核心禁用,仅编译器内部使用) ══
Mul=0x10, Mulh=0x11, Div=0x12, Divu=0x13, Rem=0x14, Remu=0x15

══ 字宽扩展 ══
Addw=0x16, Addiw=0x17, Mulw=0x18, Divw=0x19, Remw=0x1A

══ 原子操作 ══
Lrw=0x1B, Scw=0x1C, AmoSwapw=0x1D, AmoAddw=0x1E
```

### 4.2 各平台编码(以 Addi 为例)

| 语义 ID | x86-64 | AArch64 | RISC-V | WASM |
|---------|--------|---------|--------|------|
| Addi(r1,r0,10) | `48 83 C0 0A` | `11008421` | `00A50513` | `20 00 41 6A 04` |

同一条语义 ID = 同一种"行为"。不同平台的字节序列完全不同,但 **裁决后的结果 bit-identical**。

### 4.3 约束

语义 ID 不是无限扩展的:
- 8 位 → 最多 254 个语义(保留 0x00 = Nop, 0xFF = Halt)
- 每个语义 ID 必须有至少一个平台的真实 CPU 指令对应
- 不能在"无真实 CPU 对应"的情况下虚构语义 ID
- 因为 FQMT 魔数采用真实陷阱指令(非 ASCII),这是同一设计理念:不虚构、用真的

---

## 5. 句子:指令的加法拼接

### 5.1 Sentence = Vec\<VocabWord\>

```
sentence = word1 + word2 + word3 + ...
        = "加载 r0, 加 10, 跳转到顶部"
        = Load | Addi | Jmp  (语义 ID 序列)
```

对应二进制:`word1.native_bytes + word2.native_bytes + ...`

### 5.2 句子跨平台

```
同一句子在不同平台的二进制:

[sentence = Addi → Jmp]

x86-64:  48 83 C0 0A  EB FE
AArch64: 11008421  17FFFFFE
RISC-V: 00A50513  A0000067
```

所有平台的句子语义相同 → 裁定结果 bit-identical。

---

## 6. ContainerPack: 冰箱式时间触发包

### 6.1 布局

```
ContainerPack {
    pack_id: u64,         // 8B 公约数(=容器ID)
    func_code: u16,       // 2B X坐标 = 语义ID
    param_slot: u64,      // 8B Y坐标 = 操作数
    time_stamp: u32,      // 4B Z坐标 = 触发时间窗
    align_flag: u8,       // 1B 相位对齐
    payload: Vec<u8>,     // NB 预编码机器码(4B对齐)
    witness: [u8; 3],     // 3B 三元见证签名
}
```

### 6.2 为什么叫"冰箱"

```
冰箱 = 格子(container 包)存放,定时(time_stamp)取出来"烹饪"(执行)
执行 = 从格子取出 → 裁决验证(witness pass) → 装入袋子(运行时栈) → 输出

┌──────────────────────────────────────────────────┐
│  冰箱(Fridge)                                     │
│  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐            │
│  │Pack 1│ │Pack 2│ │Pack 3│ │Pack 4│  n packs   │
│  │ t=10 │ │ t=20 │ │ t=30 │ │ t=40 │   pre-timed │
│  └──────┘ └──────┘ └──────┘ └──────┘            │
└──────────────────────────────────────────────────┘
      │ time advances → t=20 reached → Pack 2 取出执行
```

### 6.3 时间触发编译(time-triggered)

不是运行时编译,不是提前编译一次运行 forever — 是 **执行之前所有行为编译**(fridge 中预编排)。

执行时:查表(找 pack_id) → 裁决(witness check) → 取袋(payload) → 输出。

无解析,无解释,无运行时编译。

### 6.4 见证签名(witness)

witness = [sum % 251, xor, (sum >> 8) + cksum + align_flag]

三种状态:
- 空袋: pack_id=0xFFFF → [0xFF, 0xFF, 0xFF] → 拒绝
- 全零陷阱: payload 全零但 pack_id 有效 → [0xFF, 0xFE, 0xFD] → 拒绝
- 有效: 三个字节由 payload 计算 → 校验和匹配

签名验证: 取袋时立即验证,任何篡改 = 拒绝执行。

---

## 7. 状态群轨道(State Enumeration)

### 7.1 核心算法

对 N 位空间的每个状态 s,计算其在群作用下的轨道(Orbit):
```
群生成元 G = {NOT, REV, ROT}
Orbit(s) = {g(s) | g ∈ <G>}
```

### 7.2 四种分类

| 类别 | 中文 | 轨道大小 | 语义 |
|------|------|---------|------|
| AxialContainer | 轴心容器态 | 2 | 自包含,镜像不变(010 ↔ 010) |
| ClosedLoop | 闭环态 | 2 | 反回文,内部自我指涉(0101,R→1010=NOT) |
| MirrorEntangled | 镜像纠缠态 | 4 | 必须与三个兄弟态共同参与才完整 |
| ExtendedEntangled | 扩展纠缠态 | 8+ | 更大对称性,更多生成元 |

### 7.3 哲学:容器不是被命名的,是自己在几何中显现的

不靠外部给容器取名,靠群轨道的对称性计算出容器的"性质"。
轴心 = 最独立,无需外部参照;
镜像纠缠 = 无法独立存在,必须三兄弟同时在场。

---

## 8. 态势感知(Situation Perception)

### 8.1 五维距离

| 维度 | 名称 | 权重 | 计算 |
|------|------|------|------|
| d0 | 位汉明距离 | 1.0 | bit-level diff |
| d1 | 轨道大小差 | 3.0 | |len_a - len_b| |
| d2 | 分类距离 | 8.0 | 类别不同 = 根本不同 |
| d3 | 代表元偏移 | 0.5 | representative diff |
| d4 | 补码距离 | 2.0 | NOT(a) vs b |

权重:**分类距离权重最高 = 阴阳翻转 = 根本不同容器类型**(呼应极性距离 × 10 在 C_eff 中的地位)。

### 8.2 聚类流程

```
输入:一批状态
  ↓
encode_to_cell (SMRL 格式 64位)
  ↓
DistanceCalculator::between(a, b) 计算五维距离
  ↓
ClusterEngine::cluster() 距离感知聚类
  ↓
EntityCluster → WrapperTree → pipeline 输出
```

### 8.3 与裁定引擎的关系

态势感知不裁决,只做 **距离聚类**。
聚类结果输入给裁定引擎 → 五公理(R1-R5) → 输出最终 verdict。

---

## 9. Container Font:容器字体统一

### 9.1 问题

系统中有 64 个容器符号,不能依赖系统字体,必须自渲染。

### 9.2 TrueType 字体生成

根据状态分类(轴心/闭环/纠缠)自动生成 TrueType 字体:
```
轴心容器态 → 对称字形(左右对称)          → 稳定视觉锚点
闭环态     → 循环/圆角字形(封闭环路)       → "圆满"视觉
镜像纠缠态 → 配对字形(左半与右半镜像)      → "必须配对"视觉
扩展纠缠态 → 放射状(多方向扩展)            → "连接网络"视觉
```

### 9.3 GlyphSource

每个字形源:
```
GlyphSource {
    codepoint: u32,
    name: String,
    bits: String,
    classification: StateClassification,
    pack: ContainerPack,
}
```

字形 + 分类 + 包 = 视觉 + 语义 + 执行一体化。

### 9.4 FontEntry

内存中的字体条目:
```
FontEntry {
    pack_id: u64,
    coord: Coord3D,
    align_flag: u8,
    payload: Vec<u8],
    witness: [u8; 3],
    category: u16,
    bit_width: u8,
    orbit_len: u8,
    representative: u16,
    orbit_elements: Vec<u16>,
}
```

---

## 10. 领域无关(Domain-Independent)

### 10.1 定义

领域无关 = 无论部署在哪个域名/服务器上,裁决算法和结果一致,不对特定基础设施产生依赖。

### 10.2 为什么能做到

```
裁决依赖:
  · SMRL 64 位编码(在 CPU 上跑,不依赖域名)
  · Witness 三元签名(三份字节,不需要 DNS)
  · 语义 ID → 多平台机器码(结果 bit-identical,不需要特定 ISA)

裁决不依赖:
  · 域名(HTTP URL)
  · 服务器 IP
  · 操作系统类型
  · 第三方 CA 证书
  · 网络拓扑
```

### 10.3 部署保证

```
同一裁定算法:
  · 部署在 self-hosted 服务器(x86 Linux)
  · 部署在 Cloudflare Workers(x86 wasm)
  · 部署在 Raspberry Pi wasm(ARM)
  → 同一个语义 ID → 三种平台的裁决结果 bit-identical
```

---

## 11. 与本系列规范的关系

| 话题 | 主参考 |
|------|--------|
| 五公理裁定 | JUDGMENT_RULES_CANON |
| WASM 双向公约 | WASM_COMMON_DIVISOR |
| FQMT 二进制 | FQMT_BINARY_FORMAT |
| SMRL 64 位编码 | SMRL_YOUWU_IMPL |
| 在场本体论 | BOOTSTRAP_SELF_COMPILER §6 |
| 时间触发相位 | TEMPORAL_PULSE_LIFECYCLE §3 |
| 盟机合节点部署 | MENGJI_IDENTITY_PROTOCOL |

---

## 12. 完整链路示例

```
外部输入:Addi(r1, r0, 10)
  │
  ▼
[跨ISA语义匹配]
  SemanticId::Addi = 0x04
  │
  ▼
[三维坐标计算]
  X = 0x04 (语义ID)
  Y = (r1 << 16) | (r0 << 8) | 10  (操作数编码)
  Z = current_phase (当前脉冲相位)
  Address = X << 24 | Y << 8 | Z
  │
  ▼
[Morton 编码]
  morton = encode_morton3(X.y.x, X.y.y, Z)
  │
  ▼
[选择平台 machine code]
  Target = AArch64
  Native = 11008421 ✓
  │
  ▼
[构建 ContainerPack]
  pack_id = 0x04_00_0A_00_00_00_00_00
  payload = [11 00 84 21] (AArch64 Addi)
  witness = compute_witness(...)
  │
  ▼
[状态群轨道]
  encode_to_state_classification()
  Category = MirrorEntangled (非闭环,需要兄弟态)
  │
  ▼
[态势感知]
  与其他 pack 的距离聚类
  C_eff 加权 → 是否允许合并裁定
  │
  ▼
[裁决输出]
  R1(阴阳), R2(共生), R3(因果), R4(容器), R5(主主)
  Pass / Reject / Reject_Level
```

---

## 13. 已知限制

1. **语义 ID 254 上限**:8 位地址空间,扣除 Nop(0x00)和 Halt(0xFF),实际可用 254 个。当前使用了 30 个,剩余 224 个。扩展需提升 X 轴位宽 = 整个三维地址格式改动。

2. **乘除指令保留但核心价值禁用**:Mul/Div 系列在词汇表中存在,因为编译器后端需要(常量折叠)。但上层"满全法核心"禁用乘除。这与 SMRL 极性距离 × 10 权重("极性翻转 = 根本变化")同源:乘除在"关系几何"中被视为"破坏性放大",仅用于编译器内部。

3. **状态空间指数增长**:N 位状态空间 = 2^N 状态。N=8 → 256 状态;N=16 → 65536;N=32 → 40 亿。群轨道枚举算法 O(2^N × |G|)。当前仅支持 N ≤ 16(实测 65536 状态枚举 < 1s)。N=32+ 需要近似算法(采样轨道而非全枚举)。

4. **容器字体仅支持静态位图字形**:当前 TrueType 生成根据状态分类输出位图轮廓(圆/方/对称)。不支持矢量曲线轮廓(二次/三次贝塞尔曲线)。未来可引入"容器矢量字形生成器"(用 Coord3D XYZ 控制控制点)。

5. **领域无关依赖 "同源裁定算法" 假设**:如果两个节点的裁定算法 hash 不同(例如版本不一致),领域无关保证失效。见 MENGJI_IDENTITY_PROTOCOL §6.1 注册流程和 NJSG_SWARM_COORDINATION §5 跨节点一致性。

---

*本规范的解释权归 刘楚恬和我 所有 · 归属 hunyuan-os/CrossISA 模块*
