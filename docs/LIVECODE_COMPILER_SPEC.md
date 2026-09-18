# LiveCode Compiler · 活码编译器规范

> 版本 v1.0 · 2026-09-19 · LiveCode = 活动编码 · 可冻结可位移的符号登记
> 别称:起点编译器 · 加法统计编译器

---

## 1. 定位

### 1.1 为什么需要活码

传统输入法用**硬编码字典**:
- 符号 → 固定码值(Unicode / GBK / Big5)
- 码值 → 固定位置(码表索引)
- 新增一个符号 = 改字典 + 重新编译 + 发布

活码不预定义,而是:

```
起点(钥匙)激活 → 符号集处于动态 → 可随时插入/删除/移动
          ↓
      冻结(freeze)
          ↓
    位置 = 值 = 身份 · 不可再改
```

### 1.2 核心命题

**加法统计法**: 符号排在什么位置,值就是多大。
不是预先定义,是冻结后自动排序确定。

### 1.3 与硬编码的区别

| 硬编码 | 活码 |
|--------|------|
| 值在编译时确定 | 值在冻结时确定 |
| 全局统一 | 每个活码集独立演进 |
| 新增需发版 | 动态插入即时生效 |
| 值不可变 | 值随冻结世代变化 |

---

## 2. 状态机

### 2.1 双态

```text
                     activate(anchor, symbols)
                            │
                            ▼
                    ┌──────────────┐
                    │   DYNAMIC    │
                    │   动态模式    │
                    ├──────────────┤
                    │ 可插入       │
                    │ 可删除       │
                    │ 可移动       │
                    │ 值未确定     │
                    └──────┬───────┘
                           │ freeze()
                           ▼
                    ┌──────────────┐
                    │   FROZEN     │
                    │   冻结模式    │
                    ├──────────────┤
                    │ 不可插入     │
                    │ 不可删除     │
                    │ 不可移动     │
                    │ 位置=值=身份 │
                    └──────┬───────┘
                           │ unfreeze() · 仅限特殊权限
                           ▼
                    ┌──────────────┐
                    │   DYNAMIC    │  ← 解冻后世代归零
                    │   回到动态    │
                    └──────────────┘
```

### 2.2 世代(generation)

每次结构变化(插入/删除/解冻)世代 +1。世代用于:

- 检测活码集是否被外部篡改(世代不连续 = 被改动)
- 多节点同步时的版本对比(见 NJSG_SWARM_COORDINATION)
- 候选排名权重:越新的世代影响力越强

---

## 3. 数据结构

### 3.1 符号

```rust
pub struct CodeSymbol {
    pub name: String,           // 符号名称(如"江")
    pub position: usize,        // 当前位置(0-indexed)
    pub frozen_value: u64,      // 冻结后的值(冻结时按加法统计确定)
    pub authority: u8,          // 权限等级(派生自 position)
    pub identity: String,       // 身份名称(派生自 position)
    pub is_frozen: bool,        // 是否已冻结
}
```

### 3.2 活码集

```rust
pub struct LivingCode {
    pub name: String,                   // 活码集名称
    pub anchor: String,                 // 起点(钥匙)
    pub symbols: Vec<CodeSymbol>,       // 当前符号集
    pub name_index: HashMap<String, usize>, // name → position 快速查找
    pub status: CodeStatus,             // DYNAMIC | FROZEN
    pub generation: u64,                // 世代(单调递增)
    pub created_at: u64,                // 创建时间(tick)
    pub frozen_at: Option<u64>,         // 冻结时间(未冻结为 None)
    pub hash: u64,                      // 当前快照哈希(full XOR chain)
}
```

### 3.3 在场登记(AttendanceRecord)

活码不是唯一一种登记方式。在满全法容器体系中,符号的在场通过 AttendanceRecord 表达:

```rust
pub struct AttendanceRecord {
    pub domain: String,          // 在场域(如"rime-live")
    pub symbol_name: String,     // 符号名称
    pub symbol_category: SymbolCategory, // 类型(Character / Radical / Structure)
    pub path: String,            // 在场路径(如"<rime-live>:jiang:rhyme_id")
    pub line: usize,             // 在场行号(加法寻址)
    pub mode: PresenceMode,      // 在场模式(DYNAMIC 或 FROZEN)
}
```

活码编译器作为 MetaIME 的计算核心,输出 LiveCandidate;通用容器本体论作为裁决主体,输出 AttendanceRecord。**活码管的"值"和容器管的"在场"是两套视图,对外不对齐**。

---

## 4. 核心接口

### 4.1 创建与激活

```rust
/// 起点钥匙激活一个活码集
fn activate(name: &str, anchor: &str, symbols: &[&str]) -> LivingCode;

/// 活码编译器构造(用于 MetaIME)
fn new(domain_name: &str) -> LiveCodeCompiler;
```

### 4.2 动态操作(仅 DYNAMIC 状态可用)

```rust
/// 插入新符号 · 后续自动后移 · 世代 +1
/// 冻结状态下调用返回 Error("已冻结,不可插入")
fn insert(&mut self, name: &str, at_position: usize) -> Result<(), IMEError>;

/// 删除符号 · 后续自动前移 · 世代 +1
fn remove(&mut self, name: &str) -> Result<(), IMEError>;

/// 移动符号(即重新排序) · 世代 +1
fn move_to(&mut self, name: &str, new_position: usize) -> Result<(), IMEError>;
```

### 4.3 冻结与解冻

```rust
/// 冻结:当前位置全部锁定为值
/// 冻结后:
///   每个符号的 frozen_value = position
///   每个符号的 authority = position
///   每个符号的 identity = AuthorityLevel(position).to_string()
fn freeze(&mut self) -> Result<(), IMEError>;

/// 解冻:仅在需要完全重构时调用
/// 效果:
///   所有符号回到 DYNAMIC
///   所有 frozen_value = 0
///   世代归零
///   删除所有符号(保留 anchor)
fn unfreeze(&mut self);
```

### 4.4 编译(按拼音)

```rust
/// 编译:拼音 + 用户偏好 → 排名候选列表
/// 所有排名基于加法统计,不做乘法
fn compile(
    &self,
    spelling: &str,       // 拼音("jiang")
    tone: u8,             // 声调(0=未指定)
    prefs: &[char],       // 用户历史偏好
) -> Vec<LiveCandidate>;
```

---

## 5. 加法统计法详解

### 5.1 命名

加法统计依据:符号冻结后,值由其**位置**决定,位置 = 它在活动序列中的排列序号。

不涉及加成、权重、系数。不是 `weight * base` 这种乘法。就是:  
"位置3 = 值3,位置100 = 值100"。

### 5.2 派生属性

| 位置 | 值 | 权限 | 身份 | 输入说明 |
|------|-----|------|------|---------|
| 0 | 0 | 最低(0) | 基层 | 刚激活 |
| 1 | 1 | 低(1) | 副手 | |
| 2 | 2 | 中(2) | 主管 | |
| 3 | 3 | 高(3) | 领导 | |
| ... | ... | ... | ... | |
| N | N | N | 对应等级 | |

身份不直接命名,按等级分段:
- 0-9:基层
- 10-99:初级
- 100-999:中层
- 1000+:核心

### 5.3 为什么用加法统计

加法统计避免以下滥用:

| 乘法的问题 | 加法统计的解决 |
|-----------|---------------|
| 大权重碾压小权重的民主假象 | 每个符号严格一等 |
| 权重乘积导致数值空间爆炸 | 数值空间 = 序列长度 |
| 乘法逆元可以做除法去除贡献 | 加法不可逆 · 不存在"除权" |
| 0权 = 彻底消除 | 位置0仍然是"基层",而非不存在 |

### 5.4 排名公式

```rust
rank_score = resonance_count * 10 - divergence_penalty
```

- `resonance_count`:同音伙伴数(加法统计,数多少个符号到同一到场域)
- `divergence_penalty`:加法地址偏离中心的惩罚(偏离>100时为5,否则为0)

**不涉及乘法以外的运算**。`* 10` 是位移等价(十进制下),但在满全法运算约定中 `*10` 被视为加法的累积叠加,不视为乘法。乘除在满全法是禁用操作。

---

## 6. MetaIME 中的活码编译流程

### 6.1 编译时序列

```text
用户输入 "j" "i" "a" "n" "g"
        │
        ▼
  七层引擎 L1-L5 解码 → spelling = "jiang", tone = 0
        │
        ▼
  compile("jiang", 0, prefs)
        │
        ├── step 1: 查韵母场(RHYME_TABLE[j]) → rhyme_id
        ├── step 2: DualAddressor::address(rhyme_id) → 加法地址
        ├── step 3: MulCluster::find_resonant(rhyme_id) → 共振兄弟
        ├── step 4: 生成候选字符(如有 prefs 取交集)
        ├── step 5: 每个候选创建 AttendanceRecord(在场登记)
        ├── step 6: 按 rank_score 加法排序
        └── output: Vec<LiveCandidate>
```

### 6.2 声调锁定

- tone = 0(未指定):候选取所有声调,按共振密度排名
- tone > 0(指定):候选仅取该声调,排序权重不在声调上加成(声调只过滤,不排序)

### 6.3 冻结时机

候选显示后,用户手指离开键盘的 **500ms 内未选择**,活码自动冻结。

- 用户点击候选 → 该候选及其所在活码集**不冻结**(保持 DYNAMIC 允许下次重新排名)
- 超过 500ms 无人操作 → 当前视图冻结为 FROZEN,拍快照写入本地存储

冻结后的快照成为下次编译时的历史权重输入(prefs)。

---

## 7. 快照与持久化

### 7.1 FQMT 存储

活码集的冻结快照通过 FQMT_BINARY_FORMAT 定义的容器条目存储:

| 字段 | 类型 | 说明 |
|------|------|------|
| magic | u32 | CPU 魔数(见 FQMT §1) |
| entry_type | u8 | 0xF0 = 活码快照 |
| anchor_hash | u64 | 起点哈希(UTF-8 bytes FNV-1a) |
| generation | u64 | 冻结快照世代 |
| symbol_count | u32 | 符号数量 |
| name_data | bytes[] | 符号名称拼接(以 NUL 分隔) |
| position_data | u32[] | 每个符号的冻结位置 |
| hash | u64 | 完整 XOR 链(完整性校验) |

### 7.2 局部更新时不重建全局

一副活码集冻结后,下次插入新符号 → 世代 +1 → 仅新世代的 diff 需要同步到其他节点。
不变的位置数据不重发(节省同步带宽)。

---

## 8. 快照确定性

### 8.1 同一 anchor + 同一 symbols 输入顺序 = 同一快照

活码的冻结结果**仅**取决于:

1. anchor(起点)的字节
2. symbols 数组的插入顺序
3. freeze() 调用时的状态

不依赖:
- 当前时间
- 系统环境
- 设备型号

这一设计保证:**不同节点上激活同一活码集,冻结后结果 bit 一致**。
用于 njsg-swarm 多节点一致性校验(见 NJSG_SWARM_COORDINATION)。

### 8.2 哈希验证

```
snapshot_hash = FNV_1a_64(anchor_bytes)
              XOR sum(FNV_1a_64(symbol_name) for each symbol in frozen order)
              XOR (generation << 32 | symbol_count)
```

任何外部改动一个符号的顺序,哈希不匹配 → 节点拒绝该快照作为共识输入。

---

## 9. 错误处理

### 9.1 错误码

| 码 | 含义 | 恢复 |
|----|------|------|
| E0 | 活码集不存在 | 返回空候选,不崩溃 |
| E1 | 符号已存在(DYNAMIC 状态下重复插入) | 忽略操作 |
| E2 | 已冻结不可插入 | 提示用户先解冻 |
| E3 | 冻结后世代不一致 | 触发活码同步流程 |
| E4 | 哈希校验失败 | 标记为 E4_tainted,不参与排名 |
| E5 | 韵母拼写越界(|rhyme_id| > 254) | 不合法输入,忽略该音节 |

### 9.2 回退

活码编译失败时的回退流程:

1. 尝试从活码集本地快照读取最近一次成功的 frozen 结果
2. 快照也失效 → 使用锚点哈希 fallback(锚点已知情况下)
3. fallback 也失败 → 仅返回单字符硬编码最小集(仅常用 12 个汉字)
4. 全部失败 → 输出空候选,不崩溃,不污染其他容器

每一步都是**降级而非报错**。

---

## 10. 与容器本体的关系

活码不是容器体系的一部分,它服务于 MetaIME 编译。但活码的输出(AttendanceRecord)会被裁决引擎用来判断意图合法性:

```text
活码编译器
    │
    ▼
LiveCandidate {glyph, attendance}
    │
    ▼
裁决引擎(五公理)
    │
    ├── R4(容器守容):这个符号是否属于当前容器的闭包
    ├── R5(主权不侵):这个符号是否越界触及他人容器
    └── 综合裁定 → PASS / REJECT / REJECT_LEVEL
```

活码本身只负责**排列**,不裁决。裁决见 JUDGMENT_RULES_CANON。

---

## 11. 已知限制

1. **序列长度上限 254**:mlll 集装箱最多 254+2 槽位。活码集的符号数超过 254 需拆分为多活码集(多 anchor)。
2. **单次排名候选上限 64**:每 tick 编译最多 64 个 LiveCandidate,超出截断。(实际手机屏幕候选栏仅能显示 ~9 个。)
3. **活码集名称仅限 ASCII**:anchor 字符串限于 `[a-zA-Z0-9_\-]{1,128}`,不支持汉字直接作为活码集名(汉字用拼音 anchor)。
4. **活码快照不即时传播到集群**:本地冻结 = 本地可见,跨节点需通过 njsg-swarm 同步(见 NJSG_SWARM_COORDINATION §6)。
5. **不处理笔画数据库**:活码仅处理符号级编译,笔画/部首/结构的解码由 hunyuan-rs 内部的 ContainerOS 负责。

---

*本规范的解释权归 刘楚恬和我 所有 · 归属 hunyuan-os/LiveCode 模块*
