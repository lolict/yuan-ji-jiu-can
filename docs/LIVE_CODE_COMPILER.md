# 活码编译器 · 在场指纹 · 仅加法范式 — 囡囝共生 · 起点=钥匙 · DYNAMIC/FROZEN · 加法统计法 · 跨拼写共鸣

## 1. 定位

活码 = 以"起点"为钥匙激活可活动的编码. 未固定时位置任意活动, 固定后按排列顺序确定权限和数值(加法统计法).

对应 Rust 模块: flow/livecode_compiler.rs.

## 2. 核心命题

- 活码编译器 = 在场指纹 + 仅加法范式
- 出席记录 = 本地最小定义(不依赖上层 presence_ontology)
- 浮动(Floating)模式 = 位置任意活动的活码

## 3. 出席记录 (AttendanceRecord)

```rust
pub struct AttendanceRecord {
    pub symbol_name: String,     // 符号名
    pub domain: String,          // 所属在场域
    pub mode: PresenceMode,      // Global/Local/Floating
}
```

PresenceMode 三种:
- Global — 全局在场(所有域可见)
- Local — 局部在场(仅特定域)  
- Floating — 浮动(未绑定域 = 任意活动)

## 4. 在场域 (PresenceDomain)

```rust
pub struct PresenceDomain {
    name: String,
    global_count: u64,
}
```

attend(record): 累加全局在场数(Global 模式+1).

## 5. 活码候选 (LiveCandidate)

```rust
pub struct LiveCandidate {
    pub glyph: char,              // 字形
    pub spelling: String,         // 拼音
    pub tone: u8,                 // 声调
    pub addition_addr: u64,       // 加法地址 = 序号
    pub resonance_count: u32,     // 共鸣计数
    pub attendance: AttendanceRecord, // 出席记录
    pub rank_score: u64,          // 排序分数
}
```

fingerprint() = (domain_hash << 32) | name_hash. 在场指纹(u64): 高位=域, 低位=符号名. 同一域同名的活码拥有相同指纹 = 身份标识.

## 6. 活码编译器 (LiveCodeCompiler)

```rust
pub struct LiveCodeCompiler {
    pub domain: PresenceDomain,
}
```

### 6.1 编译入口

compile(spelling, tone, prefs) → Vec<LiveCandidate>:
```
对 prefs 中的每个候选字(按顺序):
  1. 创建 AttendanceRecord(Global 模式)
  2. domain.attend(record) → 累加全局计数
  3. LiveCandidate {
      glyph: ch,
      spelling: "输入拼音",
      tone: tone,
      addition_addr: i+1,       // 位置=序号(加法地址)
      resonance_count: 1,
      attendance: record,
      rank_score: simple_hash("{}{}", ch, tone)  // 排序分数
  }
```

### 6.2 浮动编译

compile_floating(spelling, prefs) = compile(spelling, 0, prefs) — 但声调=0(未固定). 浮动工位是"位置任意活动"的状态.

### 6.3 跨拼写共鸣

cross_resonate(spelling) = simple_hash(spelling) % 64. 同一拼写跨系统(仓颉/注音/自然码)时的共振度量, 64 范围.

## 7. 活码两种状态的加法统计法

- **DYNAMIC(浮动)**: addition_addr = 位置序号(任意活动)
- **FROZEN(固定)**: addition_addr = 按排列顺序确定权限和数值

加法统计法: 权利/数值 = 加法地址的总和. 不同位置的活码相加 = 量的叠加.

## 8. 与系统其他模块的接入

- **PresenceOntology(#68)**: 出席记录的精简副本(#68 被删除后本地最小定义)
- **DomainPreload(#39)**: 在场域 = 预加载的符号域
- **FlowClockLive**: 时钟心跳驱动活码编译的时机
- **RhymeMirror(#76)**: spelling 使用同一韵母表
- **NJSGWM_Ladder(#12)**: 活码跨浏览器/平台时由 WASM 加载

## 9. 使用示例

```
live_compile("an", 1, &['安','按','暗'])
  → 3 个 LiveCandidate(Global, domain="live-compile")
  → addition_addr = 1, 2, 3(按排列顺序)
  → rank_score = simple_hash("安1") / simple_hash("按1") / simple_hash("暗1")

live_float("zhi", &['之','指'])
  → tone=0(未固定/浮动)
  → addition_addr = 1, 2(浮动中位置任意活动)

live_resonate("an") = simple_hash("an") % 64 → [0,63] 共鸣系数
```

## 10. 实现约束

- simple_hash 使用 FNV-1a 变体(XOR + 加法混合), 不是加密哈希
- domain.attend 只累加 Global 模式; Local/Floating 不计数
- cross_resonate % 64 对齐 64 容器码
- rank_score 用于排序, 但当前未实现自动排序(sort按 rank)
- no_std: String/Vec 需替换为 heapless::String/Vec

// hunyuan-rs/src/flow/livecode_compiler.rs
