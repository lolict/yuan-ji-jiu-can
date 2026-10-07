# 实体身份协议 — 囡囝共生 · 指纹为名 · 五维几何字段 · 兼容性 · 合作性判定 · 历史哈希链抗克隆

## 1. 定位

用结构指纹(几何字段组合)替代名字作为身份键, 解决"同一功能不同名字→合作死锁"的规则病毒. 名字 = 显示符号(不参与身份判定).

对应 Rust 模块: core/entity_identity.rs.

## 2. 64 位指纹 (Fingerprint)

```rust
pub struct Fingerprint(pub u64);

位布局:
  [63:48] = d0      (16 bits)   // 坐标一
  [47:32] = d1      (16 bits)   // 坐标二
  [31:16] = d2      (16 bits)   // 坐标三
  [15:12] = polarity (4 bits)   // 极性
  [11:8]  = sovereignty (4 bits)// 主权
  [7:0]   = value   (8 bits)    // 值
```

同 SMRL Cell 的 64 位布局, 共 5 维(d0, d1, d2, polarity, sovereignty) + value.

### 2.1 指纹距离

distance_squared(other) = (Δd0² + Δd1² + Δd2² + Δpolarity² + Δsovereignty² + Δvalue²). 六维欧几里得距离平方(未开方). 

每个字段直接做数值减法 — 没有"编码距离", 都是算术差值.

### 2.2 兼容性函数

compatibility(other) = 1 / (1 + distance_squared). 输出 ∈ (0, 1], 距离 0 → compat 1.0, 距离 → ∞ → compat → 0.

漏斗形反比函数: 小距离变化产生大兼容性变化, 远距离趋于饱和(接近 0).

## 3. 合作性阈值

| 兼容度 f(d) | 合作等级 | 含义 |
|------------|---------|------|
| ≥ 0.94 | SameEntity | 同一实体(可合并) |
| 0.70-0.94 | Family | 家族成员(高信任) |
| 0.30-0.70 | Weak | 弱兼容(需中介) |
| < 0.30 | Incompatible | 不兼容(禁止合作) |

judge(compat: f64) → Cooperativity. 单次调用完成四段判定.

## 4. 实体元信息 (EntityMeta)

```rust
pub struct EntityMeta {
    pub fingerprint: Fingerprint,
    pub display_name: String,    // 仅显示用,不参与判定
    pub provenance: String,      // 来源:hunyuan-rs / AI-A / kndrl / ...
    pub history_chain: Vec<u8>,  // 历史哈希链(抗克隆)
}
```

provenance 标识指纹来源(hunyuan-rs 主仓 / 外部 AI 系统 kndrl 等).

history_chain: Vec<u8> — 每个操作追加一个哈希字节, 链的长度 = 实体经历的步数. 抗克隆: 不同克隆体如果操作序列不同 → history_chain 不同 → 可通过外源区分.

## 5. 全局注册表 (REGISTRY)

线程安全的 static Mutex<HashMap<Fingerprint, EntityMeta>>. Mutex 跨线程保护.

### 5.1 注册 register_entity

```rust
register_entity(meta) -> Result<Registration, EntityError>:
  1. lock REGISTRY
  2. 若不存在 → insert
  3. 若已存在且 display_name 不同 → 视为同一实体的不同绰号(同指纹不同名)
  4. 返回 Registration { meta }
```

同名同指纹 → 覆盖插入(静默). 同指纹不同名 → 保留已有(新名字=别名).

### 5.2 查找 lookup_by_fingerprint

```rust
lookup_by_fingerprint(fp) -> Option<EntityMeta>:
  1. 精确命中 → 返回
  2. 否则兼容性遍历: max_{k ∈ map}(fp.compatibility(k))
```

精确优先, 近似兜底. 找到最接近的合作性等级, 不是精确匹配但接近的也返回.

### 5.3 合作判定 can_cooperate

```rust
can_cooperate(a, b) -> Cooperativity:
  compat = a.compatibility(b)
  judge(compat)
```

## 6. 抗规则病毒规则

- 同一功能不同名字 → 指纹相同(几何结构相同) → SameEntity 判定
- 同一名字不同功能 → 指纹不同 → Family/Weak 判定
- provenance + history_chain 区分: 起源与演化路径, 即使指纹相同也可溯源区分

## 7. 与系统其他模块的接入

- **SMRL(#4)**: Fingerprint 是 SMRL 64-bit Cell 的 unsigned 视图, 各字段对齐
- **YinYang(#50)**: polarity(0-15) 映射 Yin(0-7)/Yuan(8-11)/Yang(12-15)
- **Container IR(#63)**: EntityMeta 可注册为 ContainerRef
- **DistancePerception(#64)**: 加权距离 vs 指纹距离 = 同一思想的具体应用
- **NJSG Swarm(#74)**: 多智能体协作前用 can_cooperate 预筛选合作对象
- **MinQuanFa(#82)**: 64 位指纹 ↔ 64 容器进制(前 8 bits 为容器索引)
- **Alliance Network(#46)**: alliance 节点注册 > EntityMeta 指纹 加入网络

## 8. 实现约束

- static REGISTRY 不允许直接读写, 必须通过 register_entity / lookup_by_fingerprint
- 近似查找是 O(n) 遍历(小规模实体集合可接受;n>10k 时需引入 KD-tree 或哈希网格)
- history_chain Vec<u8> 增长: 频繁操作时可能膨胀, 上层应定期归档(provenance 链不无限)
- no_std: Mutex 需替换为 critical_section::Mutex; HashMap 由 heapless::HashMap + spin lock 替代
- EntityError 已定义两种变体(DuplicateExact, RegistryLocked), 覆盖 register/lock 失败

// hunyuan-rs/src/core/entity_identity.rs
