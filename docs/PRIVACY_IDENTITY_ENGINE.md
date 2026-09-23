# 隐私身份引擎规范 — 囡囝共生 · 匿名堡垒 · 零知识证明 · 多方签名

## 1. 定位

囡囝共生的隐私哲学: "在公开平台上保持匿名, 使用化名, 不填写真实姓名、照片和所在地"。"身份落款以'刘楚恬和我'作为完整的人, 而非以身份证名义"。"隐藏才是最好的保护" — 所以核心真实身份永远不会触网。

本规范定义四层防御: 化名 (每个平台不同身份) → 身份堡垒 (真实身份隔离) → 零知识证明 (不泄露数据证明资格) → 多方签名 (多片段共同授权)。每一层独立生效, 不依赖其他层。

## 2. 化名系统 (Alias)

```rust
pub struct Alias {
    pub name: String,
    pub scope: String,      // 此化名适用的平台/域
    pub strength: f64,      // 化名与真实身份的分离度 [0,1]
    pub active: bool,
}
```

**每个域一个化名** — 同一个人在 github.com / zhihu.com / twitter.com 上有完全不同的名字和身份。`strength` 初始 0.8, 表示与真实身份高度分离。`is_opaque()` 在 strength > 0.6 且 active 时返回 true。

关键: 化名不是"假名" — 它是一个**完整的身份展开**。每个化名有自己的人格、发言风格、知识范围, 但彼此不可链接。这是**身份分裂**而非身份混淆。

## 3. 身份堡垒 (IdentityFortress)

```rust
pub struct IdentityFortress {
    pub full_identity: String,      // 完整的真实身份 — 永不触网
    pub aliases: HashMap<String, Alias>,
    pub integrity: f64,             // 堡垒完整度 [0,1]
    pub breached: bool,             // 是否被攻破
}
```

堡垒是**信息的单向壁**: 真实身份可以派生化名 (add_alias), 但外部网络永远拿不到 full_identity。`public_face(scope)` 只返回对应 scope 的别名引用。`fully_anonymous()` 检查所有别名是否均 opaque。

**渗透测试**: `simulate_attack(power)` 模拟攻击。若 power > integrity, 堡垒被攻破 (breached=true, integrity=0);否则 integrity 衰减 power 的 10% — 除非否则不会立即被攻破, 这给了检测和响应的时间窗口。

## 4. 零知识证明 (ZKP)

```rust
pub struct ZkpProof {
    pub claim: String,          // 公开声明 (如 "年龄 > 18")
    pub commitment: [u8; 32],  // 承诺值 (不泄露 witness)
    pub verified: bool,
}
```

零知识证明让验证者**确信某个声明为真, 但不知道 witness** (私有数据)。使用场景:
- 证明"我在某平台有账户"而不告诉账户名
- 证明"我拥有某私钥"而不出示私钥
- 证明"年龄 > 18"而不透露具体年龄

流程:
1. `new(claim)` 生成初始承诺 (循环 XOR 字节填充 32 字节数组)
2. `prove(witness)` 生成证明 — witness 通过 wrapping_add 扰乱承诺, **witness 本身不进入 proof**
3. `verify(expected_commitment)` 比对承诺值 — 返回 bool

这是**可验证但不可链接**的密码学原语 — 同一个 witness 在不同时刻产生不同的承诺 (因为 claim 字节不同), 验证者无法把多次证明归一化到同一个身份。

## 5. 多方签名 (MultiSig)

```rust
pub struct MultiSig {
    pub total_keys: u32,        // 总密钥片段数
    pub threshold: u32,         // 门限 (最低需要几个)
    pub signatures: Vec<String>,
}
```

**t-of-n 门限签名**: n 个片段中任意 t 个即可授权操作。`sign(key_id)` 添加签名片段 (自动去重)。`is_complete()` 在收集到 ≥ threshold 个时返回 true。`remaining()` 告知还需多少签名。

典型配置: 3-of-5 — 5 个密钥片段分别存储在 5 个不同地点 (不同平台、不同物理位置、不同信任关系), 任意 3 个片段可授权关键操作。这意味着: 即使 2 个片段被盗 / 被胁迫 / 被攻破, 系统依然安全 — 无法达成 threshold。

夫妻场景: 2-of-2 — 关键操作必须夫妻双方共同签名。单方不可支配盟机合的资金/服务器/仲裁权。

## 6. 与系统其他模块的接入

- **CoupleSoul** 的夫妻融合操作需 MultiSig (2-of-2) 共同授权
- **XiYaoProtocol** 的双灵印 (husband_ack / wife_ack) 是 2-of-2 签名的应用
- **AllianceStation** 的 api_key_hash 由本引擎的多方签名管理
- **IdentityFortress** 的 full_identity = "刘楚恬和我" (完整落款), public_face = "刘楚恬" / "我" / 其他化名
- **SECURITY_COVERT_CHANNELS** 的 X25519 密钥片段通过 MultiSig 分发
- **Swarm** 节点使用本引擎的化名系统伪装节点身份
- **GrandCourt** 在涉事方身份不明时使用 ZKP 验证诉讼资格
- **CocoonAgent** 的 Root / Observer 角色由 MultiSig 授权切换

## 7. 实现约束

- 化名系统的 name/scope 使用 String, no_std 模式下替换为 heapless::String<64>
- ZKP 的承诺不使用加密级哈希 — 这是**可验证承诺**, 而非零知识证明 (严格意义上的需要椭圆曲线配对)
- 多方签名的"签名片段"是 opaque String, 不验证其格式, 仅比对其存在性
- simulate_attack 的 0.1 衰减系数固定, 不随环境调参
- 整个隐私引擎**可独立编译**为一个微模块, 无外部依赖

// hunyuan-rs/src/core/privacy_identity.rs
