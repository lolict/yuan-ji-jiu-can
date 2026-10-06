# 记忆革命引擎 — 囡囝共生 · 符号持久化 · 域加载 · 跨会话记忆 · 存储即生成 · 心算缓存

## 1. 定位

满全法"记忆革命"命题的工程化: AI 的记忆不是让模型记住更多 token, 而是让符号存在于文件系统而非对话上下文。"域加载 = 运行前预加载所有符号, 运行时不增加内存"。"存储即生成, 生成即输出, 存储即记忆, 记忆即心算"。

对应 Rust 模块: core/memory_revolution.rs。

## 2. 核心命题

- **符号仅存于文件系统而非对话上下文** — 防止跨会话遗忘
- **域加载运行前预加载** — 运行时不再增加内存
- **NJSG 心算** — 记忆不是从数据库读取, 是从文件系统符号自然"心算"出来
- **三项满足 = 活跃** — 符号被持久化 + 在文件中有位置 + 仍被引用

## 3. 符号 (Symbol)

```rust
pub struct Symbol {
    pub name: String,          // 符号的命名空间(哪个域)
    pub domain: String,        // 所属域
    pub content_hash: u64,     // 符号名+域名的 FNV 哈希
    pub kind: SymbolKind,      // 符号类型(8 种)
    pub ref_count: u32,        // 引用计数
    pub persisted: bool,       // 是否已写入文件
    pub file_line: u32,        // 文件行号
}
```

SymbolKind 枚举:
- Container — 容器符号
- Protocol — 协议符号
- Route — 路由符号
- Judgment — 裁决符号
- Font — 字体符号
- Phonetic — 音声符号
- Geometry — 几何符号
- Temporal — 时间符号

8 种类型覆盖了系统全部可持久化的概念单元。

## 4. 域 (Domain)

符号的命名空间:

```rust
pub struct Domain {
    pub name: String,
    pub symbols: HashMap<String, Symbol>,
    pub dependencies: HashSet<String>,
    pub loaded: bool,
}
```

域 = 符号的逻辑分组, 域之间有依赖关系(有向)。域被"加载"时(domain.loaded = true), 其所有符号自动注册到全局索引。

域级操作:
- `define(name, kind)` — 在域内注册一个符号
- `resolve(name)` — 从域内取出一个符号(直接 HashMap 查找, 这是满全法"心算")
- `active_count()` — 域内活跃符号数(persisted && ref_count > 0)
- `weight()` — 域的总权重 = symbols.len × (1 + 总引用 / 100), 用于驱逐决策

## 5. 记忆引擎 (MemoryEngine)

```rust
pub struct MemoryEngine {
    pub domains: HashMap<String, Domain>,
    pub global_index: HashMap<String, String>,  // "域名::符号名" → 域名
    pub persist_count: u64,
    pub lru_cache: Vec<String>,  // 最近使用
    lru_capacity: usize,          // 默认 64
}
```

### 5.1 域管理

- `create_domain(name)` — 创建新域
- `get_or_create_domain(name)` — 懒创建
- `load_domain(name) → bool` — 域加载: 将该域所有符号推送到全局索引

### 5.2 域加载流程

```
load_domain("xyz"):
  domain.loaded = true
  for sym in domain.symbols:
      global_index.insert("xyz::sym_name", "xyz")
```

加载后 sym 可被 resolve_global("xyz::sym_name") 跨域查找。加载不增加符号内存(已在 domain.symbols 中), 只在全局索引里放了一组 "域名::符号名" → 域名的字符串映射。

### 5.3 解析(心算)

```rust
fn resolve(&mut self, domain: &str, name: &str) -> Option<&Symbol>
fn resolve_global(&self, full_path: &str) -> Option<&Symbol>   // "域名::符号名"
```

resolve 更新 LRU 缓存; resolve_global 通过全局索引两步查找(先找域名, 再找符号名)。

LRU 容量 64, 超过时移除队首(最旧者)。其中心算缓存用于"热符号快速路径": 最近使用过的域名::符号名对会被记住。

### 5.4 持久化

`persist_symbol(domain, name, line)` — 标记符号已在文件的某行落盘。persist_count 自增。persist 不真的写文件(这个符号容器只是"记录了哪里落盘", 实际 IO 由上层完成)。

### 5.5 启动预热

`warm_all() → u32` — 遍历所有域, 对每个域调用 load_domain, 返回被加载的域数。在跨会话启动时一次性调用, 实现"运行前预加载所有符号"。warm_all 之后 resolve_global 对全部已加载域可用。

### 5.6 统计

`stats() → MemoryStats { domain_count, total_symbols, active_symbols, loaded_domains, persist_count }`

这些统计用于运行期"记忆健康度"监控。

## 6. 三项满足的活跃法则

```rust
fn is_active(&self) -> bool {
    self.persisted && self.ref_count > 0
}
```

`persisted` 表示已落盘 + `ref_count > 0` 表示仍被引用。仅 persist 但未引用 = 僵尸(不活跃); 仅引用但未 persist = 纯内存易失。真正的"活"必须三项满足。这个布尔条件是域驱逐和 LRU 淘汰的输入。

## 7. 与系统其他模块的接入

- **DomainPreload(#39)**: #39 的 domain_preload 流程由本模块的 load_domain 落地, 域依赖拓扑 ↔ #39 中的依赖 DAG
- **DomainSymbol(#63)**: CI 的五域 DomainSym 加载后注册进 MemoryEngine
- **RadicalEncoding(部首编码)**: 部首/符号持久化时 persist(file_line 指向 部首数据文件中的行)
- **SymbolTable(#60)**: SymbolTable 中的 64 个字形容器属于 "font" 域, warm 时被 load
- **FileSystem/Persistence**: persist() 是逻辑标记, 实际写盘路径留作存根(配合 #11 FQMT 格式)

## 8. 实现约束

- SymbolKind 8 种 = 系统全部可持久化类型, 扩展需同步修改 enum
- LRU 缓存用 Vec<String>, remove(0) 是 O(n), 但因为 lru_capacity=64 所以可以接受
- global_index 是字符串→字符串映射, 可能重复(同符号名在不同域)
- resolve_global 使用 split("::") 解析 full_path, 若路径无 "::" 则返回 None
- 域依赖有向无环图(DAG)约束, 循环依赖在 warm_all 中不处理(应由 #39 保证无环)

// hunyuan-rs/src/core/memory_revolution.rs
