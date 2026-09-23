# 产权流侵权自动判定树规范 — 囡囝共生 · 容器产权 · 态射授权

## 1. 定位

容器不是无主物 — 囡囝共生中每个容器都有主权归属。本规范定义容器产权状态(个人私有/夫妻共有/集体共有/待裁定/无主)、流的八种类型(创建/转移/变换/合并/拆分/销毁/借用/租赁),以及基于决策树的自动侵权判定逻辑。产权流判定是 GrandCourt 在"经济维度"的裁定能力。

核心哲学:产权 = 容器的所有权归属,流 = 容器之间的态射(转移/变换/交易),侵权 = 未经授权的容器占用/挪用/篡改。

## 2. 容器主权状态 (PropertyState)

```rust
pub enum PropertyState {
    Ownerless,               // 无主容器 — 可自由占用
    Private(String),         // 个人私有 (owner_id)
    CoupleShared(String, String), // 夫妻共有 (owner1, owner2)
    Collective(Vec<String>), // 集体共有 (owner_ids)
    Disputed(Vec<String>),   // 待裁定 (claimant_ids — 主权争议中)
}
```

关键:**无主容器创建后,创建者获得主权**。夫妻共有容器需双方共同操作(与 MultiSig 2-of-2 联动)。待裁定容器在主权争议期间不得流转 — 需 Triune 仲裁裁定后才能解冻。

## 3. 产权流类型 (FlowType)

```rust
pub enum FlowType {
    Creation,        // 创建 — 从虚无中创建新容器
    Transfer,        // 转移 — 所有权变更
    Transformation,  // 变换 — 容器形态改变
    Merge,           // 合并 — 多个容器合一
    Split,           // 拆分 — 一个容器分裂
    Destruction,     // 销毁 — 容器消亡
    Borrow,          // 借用 — 临时使用(不转移主权)
    Lease,           // 租赁 — 有偿使用
}
```

八种流类型覆盖了容器的全生命周期:创建→变换/借用/租赁→合并/拆分→转移→销毁。借用与租赁的差异:借用是无偿临时,租赁是有偿(可能触发愉悦值转移)。

## 4. 授权级别 (Authorization)

```rust
pub enum Authorization {
    Full,            // 完全授权 — 无任何限制
    Bounded(String), // 部分授权 — 有边界限制
    Conditional(String), // 条件授权 — 需要满足条件
    None,            // 未授权
    Revoked,         // 授权被撤回
}
```

授权的有效性判定路径:
- Full → 合法(无需进一步检查)
- Bounded → 需验证未超出边界
- Conditional → 需验证条件已满足
- None → 非主人则进入侵权判定
- Revoked → 任何操作均属盗用

## 5. 侵权类型 (InfringementType)

```rust
pub enum InfringementType {
    None,             // 无侵权
    Misappropriation, // 挪用侵权 — 将容器用于未经授权的目的
    Overboundary,     // 越界侵权 — 超出授权边界使用
    Concealment,     // 隐匿侵权 — 隐藏容器来源/历史
    Tampering,       // 篡改侵权 — 擅自修改容器编码/属性
    Theft,           // 盗用侵权 — 完全窃取他人容器
    Destruction,     // 破坏侵权 — 损坏或销毁他人容器
    Disputed(String),// 需要进一步仲裁
}
```

应急响应级别:
- 需立即停止: Theft / Destruction / Misappropriation
- 需赔偿: 除 None 和 Disputed 外的所有侵权
- 应记录为负样本: 所有非 None 侵权

## 6. 判定树 (七层决策)

```
节点1: 容器是否存在?
  └─ 否 → Theft (非法构造, 容器不存在)

节点2: 容器是否有主?
  └─ Ownerless → 无侵权 (创建者获主权)

节点3: 操作人是否为主人?
  ├─ 是 → 检查自身授权
  │   ├─ Revoked → Misappropriation
  │   └─ 其他 → 无侵权
  └─ 否 → 节点4

节点4: 授权级别?
  ├─ Full → 无侵权
  ├─ Bounded → 是否超边界?
  │   └─ 是 → Overboundary
  ├─ Conditional → 条件是否满足?
  │   └─ 否 → Misappropriation
  └─ None/Revoked → 节点5

节点5 (Revoked → 直接 Theft)

节点6 (None): 具体侵权特异性排序
  ├─ 隐藏来源 → Concealment
  ├─ 擅自修改 → Tampering
  ├─ 未经授权销毁 → Destruction
  └─ 未经授权转移 → Theft

节点7: 全部通过 → 无侵权 (合理使用)
```

**特异性排序原则**:越具体的侵权类型越优先判定。隐匿侵权(隐藏来源)比盗用更具体,所以先检查隐匿 — 这避免了把所有无授权操作都归为盗用。

## 7. 容器注册表与判定器

```rust
pub struct ContainerRegistry {
    pub containers: HashMap<String, ContainerInfo>,
}

pub struct PropertyFlowJudge {
    pub registry: ContainerRegistry,
    pub history: Vec<(PropertyFlow, Judgment)>, // 所有判定历史
}
```

判定结果 `Judgment` 包含: infringement 类型, path (判定路径节点列表), confidence (默认 0.85), explanation (文字说明), recommendation (建议措施)。

历史记录用于 NegativeImmune — 重复的侵权模式被提取为抗体,Tampering 和 Theft 的高频出现会触发 33% 采样自检。

## 8. 与系统其他模块的接入

- **NegativeImmune** 的事故模式从本模块的 history 提取
- **GrandCourt** 将 PropertyFlowJudgment 作为"经济审判"维度
- **SpousalResonance** 的 CoupleShared 状态需要本模块的夫妻共同授权验证
- **LiveCode Compiler** 的 activate/freeze 操作是 Creation/Transformation 流
- **MCCP 字节码** 的 CALL/RET 指令是 Transfer 流的安全封装
- **WASM 沙箱** 的 ContainerRegistry 通过 extern "C" 共享给 WASM 侧
- **AllianceStation** 的多方签名是 Conditional 授权的实现 — t-of-n 满足才放行
- **LogicalOrg 规则树** 的管辖关系可视为容器主权的编译期判定版本

## 9. 实现约束

- 判定树的每层只做一件事 — 不混合判定逻辑
- 决策路径 path 最多 7 层深度 (树高固定)
- Judgment.recommendation 是硬编码的字符串 — 每种侵权类型一条建议,不用动态生成
- no_std 模式: HashMap 替换为 heapless::HashMap,G history 使用 ring buffer (定长)
- 判定耗时 O(1) — 树的深度固定,无循环

// hunyuan-rs/src/judge/property_flow_judge.rs
