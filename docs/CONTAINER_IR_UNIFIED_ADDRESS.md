# 容器中间表示统一地址空间规范 — 囡囝共生 · 容器本体论 · 代数选址替代

## 1. 定位

系统中有五种地址系统各自为政: 毛利兰用 MaoriAddr(u64), 时钟用 ClockActionKey(u64), 分身用 AvatarId(String), 域用 SymbolId(u32). 跨模块转换靠 to_string + re-parse — 这就是满全法所称的"代数选址"。

容器本体论替代: 一切地址都是一个 ContainerRef — 编码为 u128 的统一容器标识。ContainerRef 不关心你用什么语言产生它, 只关心它指向什么"在场实体"。

## 2. 容器类型 (ContainerKind)

```rust
pub enum ContainerKind {
    Maori,      // 毛利兰立体编码
    Clock,      // 时钟因果密钥
    Avatar,     // 分身身份
    Symbol,     // 域符号 ID
    Relation,   // 串联/因果/纠缠关系
}
```

5 种类型 = 5 个"卦限世界"。每个 ContainerRef 的高 4 bit 标记所属的卦限:

| type_bit | 类型 | 实体编码 |
|---------|------|---------|
| 0 | Maori | 毛利兰立体坐标 |
| 1 | Clock | HierarchicalTimestamp.tick_key |
| 2 | Avatar | 4 分身 ID 编码 |
| 3 | Symbol | Flow 流符号编号 |
| 4 | Relation | 串联/因果/纠缠关系 |

## 3. ContainerRef 布局 (u128)

```
[127:124] = 容器类型 (4 bits)
[123:116] = 卦限/世界 (8 bits)
[115:64]  = 上层路径 (52 bits)
[63:0]    = 实体编码 (64 bits, 直接嵌入毛利兰/时钟编码)
```

低 64 位直接嵌入原有编码 — MaoriAddr / ClockActionKey / SymbolId 零转换嵌入。高 64 位是**统一的元数据**: 类型(4) + 卦限(8) + 命名空间路径(52)。

这是一个**纤维丛的投影**: 底空间(Entity, 64 位) + 纤维(Type+Octant+Namespace, 64 位) = 全空间(128 位)。

## 4. 构造方法

```rust
ContainerRef::new(kind, entity)              // 基础: 类型 + 实体
ContainerRef::with_octant(kind, entity, oct) // + 卦限
ContainerRef::compose(kind, octant, ns, entity) // 完整: 四元组
```

提取方法:
- `kind()`: 高 4 bit → ContainerKind
- `octant()`: 接着 8 bit → 卦限号
- `entity()`: 低 64 bit → 原始实体编码
- `namespace()`: 中间 52 bit → 上层路径

反向: `to_maori_addr()` / `to_avatar_id()` / 等 — 从 ContainerRef 恢复原有地址格式。

## 5. 容器运行时 (ContainerRuntime)

```rust
pub struct ContainerRuntime {
    pub containers: HashMap<ContainerRef, ContainerState>,
    pub epoch: u64,
}
```

`ContainerState` 三类: Atom(原子容器) / Couple(夫妻容器) / Avatar(分身容器)。

核心操作:
- `register(kind, entity) → ContainerRef`: 自动分配 epoch, 注册新容器
- `relate(a, b, RelationType)`: 建立容器间关系(Contain/Couple/Influence/Witness)
- `dissolve(ref)`: 销毁容器(从注册表中移除)

关系类型 × 容器类型 = 完整的关系代数:
- Atom + Couple → 容器内部包含夫妻对
- Couple + Avatar → 夫妻双方各派出分身
- Atom + Atom → 包含/被包含/平行

## 6. 域符号集合 (DomainSym)

```rust
pub struct DomainSym {
    pub table: HashMap<u32, String>,
}
```

5 个域的符号表: Hunyuan(囡囝) / Dragon(东方青龙) / Phoenix(南方朱雀) / Tortoise(西方白虎) / Qilin(北方玄武)。每域 20 个符号, 共 100 个五行符号。

`domain_symbol(domain, index): Option<&str>` — 通过域编号 + 偏移量查找符号, 用于 NJQ 聂小倩匹配的初始加载。

## 7. 与系统其他模块的接入

- **FiveFlowUnifiedEngine** 的八流符号通过 ContainerRef::with_octant(kind=Symbol) 统一引用
- **RuntimeLoop** 的 ContainerAtlas 与本模块的 ContainerRuntime 双向同步 — Atlas 是 IR 的"运行态实例"
- **CoupleSoul** 的夫妻容器对用 ContainerRef::compose(kind=Couple) 编码
- **AllianceStation** 的节点地址用 ContainerRef 统一(涵盖所有物理节点/盟机场)
- **Witness 对**(SurplusPair)的关系用 rel_anchor ContainerRef 表示
- **HeavenSpace** 的天庭坐标用 ContainerRef(kind=Maori) 嵌入
- **Distinguisher 分别器** 的四分身用 ContainerRef(kind=Avatar) 注册

## 8. 实现约束

- u128 在 wasm32 下运算较慢(需软件模拟 128 位), 但不影响映射表查找
- HashMap 在 no_std 下替换为 heapless::HashMap (最大 1024 条目)
- 容器注册后 epoch 单调递增, 永不回退
- dissolve 不真正"删除"容器(历史上已有记录), 只是标记为 Inactive

// hunyuan-rs/src/core/container_ir.rs
