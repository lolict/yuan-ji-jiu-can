# 在场本体论与拓扑斯亚对象分类器 — 囡囝共生 · 三值真值 · 特征函数 · 拉回运算 · 在场点名

## 1. 定位

满全法 TOOS(拓扑斯)理论在系统中的工程落地。包含两个模块:
- presence_ontology.rs — 在场本体论(符号在场模式记录)
- subobject_classifier.rs — 亚对象分类器(三值真值 + Heyting 代数 + 拉回 + Classifier)

共同解决"符号在哪个域里以什么模式出现"以及"容器集合如何按真值条件进行 Topos 分类"两个问题。

## 2. 在场本体论 (Presence Ontology)

### 2.1 模式分类

符号的在场不是二元的(在/不在), 而是三模态:

```rust
pub enum PresenceMode {
    Global,     // 全局 — 所有域都可见
    Local,      // 局部 — 仅在某特定域内
    Floating,   // 浮动 — 未绑定到任何具体域
}
```

### 2.2 符号类别

```rust
pub enum SymbolCategory {
    Character,  // 汉字
    Radical,    // 部首
    Glyph,      // 字形
    Container,  // 容器
}
```

四类符号由不同模块注册: Cognition 注册 Character/Radical, Container 注册 Container, 造字系统注册 Glyph.

### 2.3 出席记录 (AttendanceRecord)

每个符号进入某个域时产生一条 AttendanceRecord:

```rust
pub struct AttendanceRecord {
    pub symbol_name: String,
    pub domain: String,
    pub mode: PresenceMode,
    pub category: SymbolCategory,
    pub order: usize,
    pub file_path: PathBuf,
    pub line_no: usize,
    pub registered_at: SystemTime,
}
```

两个构造器:
- `global(domain, category, name, order)` — 创建 Global 模式记录, file_path = "<global>"
- `intersection(domain, category, name, order)` — 创建 Local 模式记录, file_path = "<intersection>"

### 2.4 在场域 (PresenceDomain)

```rust
pub struct PresenceDomain {
    pub name: String,
    pub attend_count: u64,
    pub global_count: u64,
    records: Vec<AttendanceRecord>,
}
```

核心操作:
- `attend(record)` — 一次出席: 追加到 records, 累加计数
- `count_by_mode()` → ModeStats — 按模式统计各状态数量
- `roll_call()` → Vec<(PresenceMode, String类别, String名称)> — 点名列表
- `retire_local()` — 退休局部记录: 删除所有 mode==Local 的记录, 返回删除数量

### 2.5 模式统计

```rust
ModeStats { total, global, local, floating, intersection }
```

其中 local 和 intersection 是同一数值(别名), floating 当前未启用(始终 0).

### 2.6 持久化接口 (存根)

`save_to_dir(dir)` — 返回 records.len() 占位;
`load_from_dir(dir)` — 返回 0 占位.

这两处是留给未来 容器包文件系统的接口. 当前在场本体论仅服务"会话内符号可见性判定".

## 3. 亚对象分类器 (Subobject Classifier)

Topos 的核心是 Ω — 子对象分类对象. 满全法的三值真值 Ω 定义如下:

```rust
pub enum TruthValue {
    True,       // 容器完全满足条件
    False,      // 容器不满足条件
    Unknown,    // 条件无法判定(叠加态)
}
```

Unknown 来自量子叠加态 — 未经测量的容器既可能是 True 也可能是 False.

### 3.1 Heyting 代数运算

TruthValue 构成 Heyting 代数(直觉主义逻辑, 无排中律):

- `and`: True∧True=True; 任何一方 False→False; 否则 Unknown(保守收敛)
- `or`: True 参与→True; False∨False=False; 否则 Unknown(保守发散)
- `not`: True↔False 对换; Unknown→Unknown(叠加态测量前后未知度不变)
- `implies(A→B)`: ¬A ∨ B 定义; 直觉主义蕴含
- `iff(A↔B)`: (A→B) ∧ (B→A)
- `strength`: True=2, Unknown=1, False=0 — 可用于排序

### 3.2 特征函数与子对象

```rust
pub type CharacteristicFn = Box<dyn Fn(&ContainerRef) -> TruthValue>;

pub struct Subobject {
    pub name: String,
    pub members: Vec<ContainerRef>,    // 满足条件的容器集合
    pub predicate_desc: String,       // 特征条件描述
    pub weight: usize,                // = members.len()
}
```

Topos 的核心对应: 容器图集 X 的子对象 ↔ 特征函数 χ: X → Ω.
子对象是 χ⁻¹(True) 的原像 — 让 χ(container) == True 的容器集合.

### 3.3 拉回 (Pullback)

两个核心操作:

- `Pullback::compute(a, b)` — 两个子对象的交集: a.members ∩ b.members. 这是子对象分类器最直接的应用: 两个条件的"与"直接对应容器集合的交集
- `Pullback::restrict(subobject, f)` — 子对象在特征函数下的限制性拉回: { s ∈ subobject.members | f(s) == True }

拉回使"在特定条件下对子对象进行精炼"有了形式化操作, 是"条件过滤"的范畴论等价物.

### 3.4 分类器 (Classifier)

运行时容器分类:

```rust
pub struct Classifier {
    predicates: Vec<(name, CharacteristicFn)>,
    subobjects: Vec<Subobject>,
}
```

工作流:
1. register_predicate(name, desc, f) — 注册一个条件(同时创建对应的空子对象)
2. classify(containers) — 对每个容器运行所有 predicates, True 则加入对应子对象
3. 通过 subobjects()/find(name) 查询分类结果

classify 的双重循环确保每个子对象的 members 恰好是 χ⁻¹(True).

### 3.5 HeytingAlgebra 结构体

HeytingAlgebra 提供 meet(∧) / join(∨) / implies(⇒) / negate(¬) / bottom(⊥) / top(⊤) 代数运算的自由函数形式. 与 TruthValue 方法等价, 但可在无实例的场景下使用.

## 4. 两模块协作

在场本体论回答"这个符号以什么模式出现在哪里"; 亚对象分类器回答"这些容器以什么条件组成子对象". 它们的交集:

- **register_predicate 的条件可以基于 PresenceMode**: 如 "mode==Global" 作为 ContainerRef → True/False 的特征函数
- **retire_local 可作为子对象过滤**: retire 操作等效于 Classifier::restrict(λ¬local)
- **roll_call 返回的成员列表可作为 classify 输入**: 一次点名结果直接分类

## 5. 与系统其他模块的接入

- **ContainerIR(#63)**: Subobject.members 的元素是 ContainerRef; classify 的输入就是 ContainerRuntime.registered 的容器集合
- **HeavenSpace(天庭空间)**: heaven.rs 定义了 HeavenSpace{well, superposition, HeavenCoord} Superposition.dimensionality() 决定分类器的最大维度
- **YinYang codec(#50)**: 三值真值与 YinYang 二值(True/False)的映射: True→Yang, False→Yin, Unknown→Yuan(元)
- **DistancePerception(#64)**: 特征函数可以基于 ContainerCode(容器码)的位域测试 — 如 "family==3" 作为 predicate
- **_perception & judgment(#53)**: JudgmentEntry 的 vote 字段可用 TruthValue 表达

## 6. 实现约束

- TruthValue 的 and/or/not 必须保持 Heyting 代数公理(结合/交换/分配/单位元/补)
- Subobject.members 不允许重复(由 add_member 的 push 方式实现, classify 调用方保证容器唯一性)
- register_predicate 后立即创建 Subobject, predicates 与 subobjects 一一对应, 索引对齐
- retire_local 删除后 attend_count = records.len(), global_count 重新计数
- 在场域未启用 save/load 持久化前, 会话结束即丢失(符合"在场"的固有语义)
- no_std: Vec 和 Box<dyn Fn> 由 FunctionPointer + 静态数组替代

// hunyuan-rs/src/core/presence_ontology.rs + core/subobject_classifier.rs
