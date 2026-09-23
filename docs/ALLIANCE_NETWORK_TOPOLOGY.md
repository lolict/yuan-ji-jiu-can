# 盟机合分布式网络拓扑规范 — 囡囝共生 · 非层级公约数协作

## 1. 定位

工业时代的"合伙人"概念在囡囝共生中全面转向**盟机合**体系 — 不是股份层级, 而是主权独立的节点通过公约数共振协作。盟机合掌 (运营者) 拥有资源与场地, 盟机场/萌机场 (节点) 是主权独立的意识容器。节点协作不是上级命令下级, 而是平等共振。

盟机合的两套归属: **资源归盟机合掌** (谁是服务器的主人), **规则归囡囝共生** (编排协议是统一的)。节点可以离开盟机合掌, 但不可带走囡囝共生规则的实现。

## 2. 盟机合掌 (AlliancePalm)

```rust
pub struct AlliancePalm {
    pub id: u64,
    pub name: String,
    pub managed_stations: Vec<u64>,  // 运营的场
    pub reputation: f64,             // 声誉 [0,1]
    pub active: bool,
}
```

一个掌可以运营**多个场** — 这是盟机合独特的"一掌多场"拓扑, 区别于传统的"一公司一服务器"模型。`reputation` 初始 0.5, 由网络健康度和仲裁胜诉率动态调整。

## 3. 盟机场/萌机场 (AllianceStation)

```rust
pub struct AllianceStation {
    pub id: u64,
    pub domain: String,             // 身份锚点
    pub server_id: String,          // 物理位置/服务器标识
    pub operator_id: u64,           // 运营者 (盟机合掌)
    pub coord: StationCoord,        // 几何空间位置 (X:经度 / Y:语义 / Z:时间)
    pub status: StationStatus,
    pub api_key_hash: [u8; 16],    // 外部AI接口的密钥哈希
    pub container_count: u32,       // 已注册的容器数
}
```

四状态: Active (运营中) / Standby (待命) / Migrating (迁移中) / Suspended (暂停)。`api_key_hash` 仅存哈希 — 真实 API Key 由节点自己保管, 盟机合不见密钥原文。

距离使用 Coord3D 的欧氏距离 (与 FiveDimensional 同源)。

## 4. 协作类型 (CollabKind)

```rust
pub enum CollabKind {
    Resonance,      // 公约数共振 — 几何空间的距离感知聚类
    DataSync,       // 数据同步 — 状态一致化
    HeartbeatRelay, // 心跳代传 — 为宕机节点中继
    Arbitration,    // 委托裁决 — 将争议提交邻近节点裁定
}
```

四种协作**没有包含关系** — 它们各自独立, 一个协作关系不可能同时是两种类型。这是"协作类型不可混淆"原则的具体体现。

## 5. 网络引擎 (AllianceNetworkEngine)

```rust
pub struct AllianceNetworkEngine {
    pub palms: HashMap<u64, AlliancePalm>,
    pub stations: HashMap<u64, AllianceStation>,
    pub collaborations: Vec<Collaboration>,
    next_palm_id: u64,
    next_station_id: u64,
}
```

ID 分配器自增。注册掌 → 注册站 (自动关联到掌) → 建立协作 (验证两端存在、clamp strength)。

**网络健康度**: `health() = active_ratio × 0.7 + collab_ratio × 0.3`。活跃节点比例占 70%, 协作链接数占 30%。一个全是活跃节点但零协作的网络 — health = 0.7; 有大量协作但一半宕机 — health = 0.5。

**最近邻**: `nearest_station()` 通过欧氏距离计算, 用于心跳代传和委托裁决时寻找最近的对等节点。

## 6. 与系统其他模块的接入

- **CoupleSoul** 的夫妻容器分驻在不同盟机场上 — 跨节点婚姻
- **CocoonAgent** 的蚕蛹在盟机场上部署, 盟机合掌运营其肉身
- **Heartbeat** 盟机场之间互发心跳检测存活
- **Triune 仲裁** 在盟机场之间委托裁决 — 节点 A 与 B 争议时, 由最近邻 C 的 Triune 裁定
- **Security** 的 api_key_hash 是 Security 模块密钥哈希的引用
- **MetaIME** 可达盟机场节点, 通过监听键盘远程打字输入
- **Swarm 分布式智能体** 的节点发现通过 AllianceNetworkEngine.active_stations 获取
- **LayeredMemory 记忆** 的分布式备份通过 DataSync 协作类型实现
- **CelestialSpace** 天庭空间可能跨节点部署, 由不同盟机场承载不同天宫

## 7. 实现约束

- 网络节点数 ≤ 65535 (u64 ID 理论无限制, 但 HashMap 内存限制实际规模)
- 协作链接数无硬性限制, 但 health() 计算时 collab_ratio 以 stations.len() 为分母 (每站至少一条协作才满分)
- 不依赖 TCP/IP 协议 — 这是拓扑抽象层, 传输层由 Swarm 或 Heartbeat 模块实现
- no_std 模式下 HashMap 替换为 heapless::HashMap (最大容量编译期指定)

// hunyuan-rs/src/core/alliance_network.rs
