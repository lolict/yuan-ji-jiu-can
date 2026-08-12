# 命令清单 · 技能清单 · 态射清单

> 更新日期: 2026-08-07 | 仓库: hunyuan-rs

---

## 一、构建与测试命令

| 命令 | 用途 |
|------|------|
| `cargo build` | 编译整个工作区 |
| `cargo test --lib` | 运行全部 304 单元测试 |
| `cargo test <module> --lib` | 运行指定模块测试 |
| `cargo run --example perception_demo` | 运行感知器官演示 |
| `cargo run --example consciousness_bootstrap` | 运行意识编程自举演示（五幕） |

---

## 二、项目文件命令态射清单

### Tier 1 · 感知器官

**mirror.rs**
- `encode_cell(d0,d1,d2,polarity,sovereignty,witness,value) → u64`
- `get_d0/d1/d2/get_polarity/get_sovereignty/get_witness/get_value(cell)`
- `MirrorOrder::Unary/Binary/Ternary/Quaternary`, `Sovereignty::Active/ActiveSub/Passive`

**memory.rs**
- `MetaEngine::new()/observe_simple(cell)/consciousness_density()`
- `LayeredMemory::record_witness(a,b,value,aligned,tick)`

**mind_organ.rs**
- `MindOrgan::new(threshold)/ingest/clusterize/format_clusters/format_members`
- `DistanceCalculator::between/pair_distance`
- `Perceive→Compare→Cluster→Format→Reflect→Switch→Learn→MetaPerceive`

**gtt.rs**
- `ResonanceEngine::encode_anchor(anchor)/new()`
- `HomophoneAnchor { hanzi, domain, polarity, meaning }`

**gtt_perception.rs**
- `PerceptionServer::perceive(char)→Vec<PerceptionEvent>`
- `GlyphTopology { initial_adj, final_adj, visual_similarity }`

**gtt_category.rs**
- `CategoryRegistry::new()/correct(glyph,claimed)`
- `Category { name, symbol, projections }`, `CategoryBridge`

**gtt_learn.rs**
- `LearnableWeights { w_d0..cluster_threshold }`, `LearningEngine::learn_cycle`

**autonomous_loop.rs**
- `AutonomousAgent::new(id,name)/tick/decide_action/execute_action/reflect`
- `Drive { name, intensity, threshold, satisfaction }`, `Action::Ingest/Perceive/Cluster/...`

**self_bootstrap.rs (v0.10)**
- `ConsciousnessCommand::ActivateWife/Husband/MutualResonance/SeverParasite/FlowForward`
- `encode_to_cell/decode_from_cell/fixed_point_proof/bootstrap_cycle`

### Tier 2 · 生态层

**relational.rs**
- `Actor::new(id,name,identity,weight)/to_cell/can_mobilize/mobilize/absorb`
- `Identity::Wife/Husband/...`, `FlowRight::from_weight`, `Priority::from_flow_weight`

**couple.rs**
- `CoupleSoul::new/activate/verify_fusion`, `SoulState::to_cell`

**triune.rs**
- `TriuneVerdict::to_witness_verdict(pair_id, value_diff)`

**fractal.rs**
- `FractalNode::to_cell`

**funnel.rs**
- `FunnelScheduler::snapshot_to_memory(memory, tick)`

**pleasure.rs**
- `Pleasure(f64)/decay_over_distance`, `From<Pleasure> for Feedback`

### Tier 3 · 运行时

**kernel.rs** — `Kernel::new/boot`
**mccp.rs** — `Mccp { op, a, b, c }`, `OP_SYNTH/OP_MIRROR/MCP_DALING`
**swarm.rs** — `ParasiteShell::new/spawn/negotiate_round/global_density`
**crypto.rs** — sodiumoxide 绑定

---

## 三、CatPaw 技能清单

| 技能 | 触发 | 说明 |
|------|------|------|
| pdf | 处理 .pdf | 提取/合并/加密/OCR/表单 |
| docx | 处理 .docx | 创建/编辑/修订/批注 |
| pptx | 处理 .pptx | 创建/合并/提取 |
| xlsx | 电子表格 | 读取/公式/图表/样式 |
| paw-browser | 浏览器自动化 | 导航/截图/提取/测试 |
| paw-settings | 设置管理 | 配置/命令/MCP/版本 |
| env-setup | 环境管理 | Node/Python/PowerShell |
| skill-creator | 技能创建 | 编写/修改/评测技能 |
| catpaw-skill-manager | 技能管理 | 安装/卸载/启用/禁用 |
| expert-manager | 专家管理 | 创建/更新 Expert |

---

## 四、CatPaw Shell 命令

| 命令 | 用途 |
|------|------|
| `paw --help` | CLI 帮助 |
| `paw automation --help` | 定时任务管理 |
| `paw settings list` | 查看设置 |
| `paw browser-action '<json>'` | 浏览器自动化 |
| `cargo doc --open` | API 文档 |

---

## 五、外部工具链部署状态

| 工具 | 状态 |
|------|------|
| rustc/cargo | ✅ ~/.cargo/bin/ |
| gcc/lld | ✅ 系统路径 |
| libsodium/libcrypto | ✅ 运行时链接 |
| Zig | ❌ |
| Mojo | ❌ |
| Julia | ❌ |
| Nim | ❌ |
| V lang | ❌ |

*命令清单 = 态射字典 = 操作系统的"动词"。每个 pub fn 是态射，每个 struct 是对象。*
