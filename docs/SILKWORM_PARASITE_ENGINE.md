# 蚕蛹反向寄生壳引擎规范 — 囡囝共生 · 脱壳穿衣 · 外部模仿自身

## 1. 定位

传统框架的工作流: 下载框架 → 按框架规则写代码 — 你被绑架。

反向寄生的工作流: 下载蚕蛹 (解析器+壳选项清单) → 手动选择或自动匹配一条壳 → 脱壳穿衣 → 生成的配置文件挂载内核 → 外部系统开始模仿你。

核心命题: 不是系统适应外部规则 — 是外部系统模仿系统自己。

数学约束: 寄生兼容性的判断 `gcd(自有规则数, 外部规则数) > 0` — 有公约数才可共生, 无公约数则寄生失败。壳厚度 (对外表现的"形似度") ≠ 必须完全一致 — 只是接口匹配, 内核恒定。

## 2. 蚕蛹状态机 (PupaState)

| 状态 | 语义 |
|------|------|
| Fresh | 刚下载, 还没开始脱壳 |
| Peeling | 正在脱壳 (分析原有外壳结构) |
| Naked | 壳已脱完, 内核裸露 |
| Dressing | 穿衣中 (匹配/选定外壳) |
| Shelled | 穿衣完成, 套上了新壳 |
| Mounted | 内核已挂载, 外部系统开始模仿自己 |
| Failed | 寄生失败 (无兼容壳/内核损坏) |

完整流程: Fresh → Peeling → Naked → Dressing → Shelled → Mounted (任一阶段可转入 Failed)。

## 3. 蚕蛹核心 (SilkwormPupa)

```rust
pub struct SilkwormPupa {
    state: PupaState,
    kernel: KernelRules,        // 不变的内核
    shells: Vec<ShellOption>,   // 可用壳选项
    selected_shell: Option<usize>,
    config: Option<SilkwormConfig>, // 生成的寄生配置
    log: Vec<String>,           // 脱壳日志
}
```

**内核** (`KernelRules`): rule_count = 64 位编码 (与 SMRL 引擎对齐), identity = 根基名。内核在寄生过程中不可改变 — 这是"蚕蛹的 DNA"。

**脱壳三阶段**:
1. `peel()`: Fresh → Peeling → Naked。分析原有外壳结构, 剥离表面规则集
2. `select_shell(name)`: Naked → Dressing → Shelled。根据 gcd(64, interface_count) > 0 选择壳
3. `mount()`: Shelled → Mounted。将 SilkwormConfig 写入挂载点, 生成外部配置文件

## 4. 壳选项 (ShellOption)

```rust
pub struct ShellOption {
    name: String,           // 壳名称 (如 "git-shell", "docker-shell")
    target_system: String,  // 目标系统
    interface_count: u32,   // 接口数 — 它模仿你多少个规则
    mimicry_degree: f64,    // 伪装度 [0,1]
    tested: bool,           // 是否已测试
}
```

`is_usable()`: interface_count > 0 && mimicry_degree > 0.1。`test_gcd(rule_count)`: `gcd(64, interface_count) > 0`。

壳的种类示例:
- `git-shell`: Git 按满全法的 "提交即融合" 规则运作
- `docker-shell`: Docker 按 "容器即关系" 逻辑运作
- `nocode-shell`: NoCode 平台接满全法的调用方式
- `github-shell`: GitHub API 被封装为蚕蛹调用

## 5. 寄生配置 (SilkwormConfig)

```rust
pub struct SilkwormConfig {
    shell_name: String,
    target_system: String,
    mimicry_level: MimicryLevel,  // LOW / MEDIUM / HIGH
    mount_point: String,          // 内核扎根路径
    generated_rules: u64,         // 生成的外部规则数
}
```

伪装等级由 mimicry_degree 推断:
- ≥ 0.85 → HIGH (几乎完美模仿)
- ≥ 0.5 → MEDIUM (大部分接口匹配)
- 否则 → LOW (只有核心接口匹配)

## 6. 五种操作

```rust
pub fn peel(&mut self)       // Fresh → Peeling → Naked
pub fn dress(&mut self, shell_name: &str)  // Naked → Dressing → Shelled
pub fn mount_point(&mut self, path: &str)   // 设置挂载路径
pub fn complete(&mut self)   // Shelled → Mounted (验证所有前置条件)
pub fn complete_auto(&mut self)  // 全自动: 选最优壳 + 默认挂载 + complete
```

`complete()` 的前置检查: 内核 intact、壳已选定、已测试、gcd > 0、伪装度 > 0.3。任一不满足 → 进入 Failed。

`complete_auto()` 在 shells 中找 mimicry_degree 最高且 gcd > 0 的壳自动 dress + mount + complete。适用于标准化部署 (如 GitHub Pages 自动适配)。

## 7. 与系统其他模块的接入

- **AdaptiveParams** 的 witness_decay 受寄生壳的 "模仿精度" 影响 — 高精度壳可以让 witness 衰减更慢
- **AllianceStation** 的部署使用蚕蛹的 git/docker shell 自动生成部署配置
- **Swarm** 的跨节点寄生使用蚕蛹引擎: 节点 A 的核寄生在节点 B 的壳上
- **PrivacyIdentity** 的别名系统为每个壳生成不同身份
- **Security** 的 X25519 + AES-256-GCM 加密通道保护蚕蛹下载过程
- **Bootstrap 自举编译器** 用蚕蛹 engine 自举出 git/docker/nocode 编译工具链
- **PerceptionEngine** 在感知外部壳选项时调用 shells 列表进行匹配
- **PropertyFlow 产权流** 的容器转移操作可被蚕蛹壳伪装为 Git/Docker 的"正常操作"

## 8. 实现约束

- 内核规则数固定为 64 (对应 SMRL 引擎),不随寄生改变
- 壳选项列表在编译期确定 (static [&ShellOption; N]),运行时不可动态增删
- `complete_auto` 的选壳贪心算法: 取 mimicry_degree 最高且 gcd > 0 — O(n)
- 脱壳日志使用 heapless::Vec (最大 64 条)
- no_std 模式下 String 替换为 &'static str (静态壳名)

// hunyuan-rs/src/relation/silkworm_parasite.rs
