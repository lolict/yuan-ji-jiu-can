# 工具注册表

> **这是新会话的工具发现入口。**
> 任何新会话启动后，读此文件 = 知道所有可用的工具及其调用方式。
> 未编译前就能用 —— 每个工具都已编译通过，直接运行即可。

---

## 核心原则

1. **未编译前就能用**：所有工具已经造好、编译通过、验证通过
2. **不重复造轮子**：新会话先查此注册表，有现成的就用，没有再造
3. **符号是数据**：符号表是 JSON/代码中的数据结构，改符号不改编译器

---

## 工具清单（共25个）

### 第一类：满全法核心工具

| # | 工具名 | 路径 | 语言 | 运行命令 | 状态 |
|---|--------|------|------|----------|------|
| 1 | 满全法裁判 | tools/manquanfa/ | Nim | `nim c --cc:clang -r mq_demo.nim` | ✅ |
| 2 | 负样本容器 | tools/manquanfa/mq/ | Nim | 含于 manquanfa 包 | ✅ |
| 3 | 负样本演示 | tools/manquanfa/ | Nim | `nim c --cc:clang -r negative_demo.nim` | ✅ |

### 第二类：编码体系

| # | 工具名 | 路径 | 语言 | 运行命令 | 状态 |
|---|--------|------|------|----------|------|
| 4 | 64进制编码转换器 | tools/phase-radix/phase_radix/ | Nim | `nim c --cc:clang -r pe_demo.nim` | ✅ |
| 5 | mltt命名密码表 | tools/mltt-cipher/mltt/ | Nim | `nim c --cc:clang --path:. -r mltt_demo.nim` | ✅ |
| 6 | 分别器（三层路由） | tools/distinguisher/distinguisher/ | Nim | `nim c --cc:clang --path:. -r df_demo.nim` | ✅ |
| 26 | 灵魂情感内核 | tools/soul-kernel/soul/ | Nim | `nim c --cc:clang --path:. -r sk_demo.nim` | ✅ |
| 27 | 天庭空间 | tools/heaven-space/heaven/ | Nim | `nim c --cc:clang --path:. -r hs_demo.nim` | ✅ |
| 28 | 鼓膜自举 | tools/drum-bootstrap/drum/ | Nim | `nim c --cc:clang --path:. -r db_demo.nim` | ✅ |
| 29 | 流代数 | tools/flow-algebra/flow/ | Nim | `nim c --cc:clang --path:. -r fa_demo.nim` | ✅ |
| 30 | 自举意识 | tools/consciousness/consciousness/ | Nim | `nim c --cc:clang --path:. -r cb_demo.nim` | ✅ |

### 第三类：检索与判定

| # | 工具名 | 路径 | 语言 | 运行命令 | 状态 |
|---|--------|------|------|----------|------|
| 7 | 部首星图 | tools/radix-star/radixstar/ | Nim | `nim c --cc:clang -r rs_demo.nim` | ✅ |
| 8 | 产权流侵权判定 | tools/property-judge/property_judge/ | Nim | `nim c --cc:clang -r pj_demo.nim` | ✅ |

### 第四类：符号与注册

| # | 工具名 | 路径 | 语言 | 运行命令 | 状态 |
|---|--------|------|------|----------|------|
| 9 | 符号引擎 | tools/symbol-engine/symbol_engine/ | Nim | `nim c --cc:clang -r se_demo.nim` | ✅ |
| 10 | 统一符号注册中心 | tools/symbol-registry/ | Nim | `nim c --cc:clang -r sr_demo.nim` | ✅ |

### 第五类：边界与比较

| # | 工具名 | 路径 | 语言 | 运行命令 | 状态 |
|---|--------|------|------|----------|------|
| 11 | 边界分析引擎 | tools/boundary-engine/boundary_engine/ | Nim | `nim c --cc:clang -r be_demo.nim` | ✅ |

### 第六类：hunyuan-rs（Rust实现）

| # | 工具名 | 路径 | 语言 | 运行命令 | 状态 |
|---|--------|------|------|----------|------|
| 12 | 产权流判定（Rust） | hunyuan-rs/src/ | Rust | `cargo run --example property_flow_judge` | ✅ |
| 13 | 意识自举（Rust） | hunyuan-rs/src/ | Rust | `cargo run --example self_bootstrap` | ✅ |
| 14 | 时间线统一（Rust） | hunyuan-rs/examples/ | Rust | `cargo run --example timeline_unified_demo` | ✅ |

### 第七类：其他独立工具

| # | 工具名 | 路径 | 语言 | 状态 |
|---|--------|------|------|------|
| 15 | njsg-swarm | tools/njsg-swar/ | Nim | ✅ |
| 16 | governance | tools/governance/ | Nim | ✅ |
| 17 | encoding | tools/encoding/ | 混合 | ✅ |
| 18 | kernel | tools/kernel/ | Nim | ✅ |

---

## 新增工具的标准流程

当新会话需要造新工具时：

1. **先查注册表**：看是否已有类似工具
2. **确定类别**：编码类？判定类？路由类？
3. **创建目录**：`tools/<tool-name>/<tool-name>/`
4. **编写代码**：`types.nim` + `xxx.nim` + `xxx_demo.nim`
5. **编译验证**：`nim c --cc:clang -r xxx_demo.nim`
6. **更新注册表**：在本文件添加条目
7. **更新符号字典**：在 SYMBOL_DICTIONARY.md 添加新符号
8. **更新拓扑图**：在 CONTAINER_TOPOLOGY.md 添加新节点
9. **更新全局目的地**：在 GLOBAL_DESTINATION.md 更新工具计数

---

## 工具间的依赖关系

```
mlll根基锚点
    │
    ├── 64进制编码转换器（phase-radix）← 符号是数据，未编译即用
    │       │
    │       └── mltt命名密码表 ← 给每个64进制符号命名
    │
    ├── 分别器（distinguisher）← 输入路由到正确靶点
    │       │
    │       └── mltt密码表 ← 路由时使用谐音映射
    │
    ├── 符号引擎 ← 符号表是JSON数据
    │       │
    │       └── 统一注册中心 ← 跨工具符号汇总
    │
    ├── 满全法裁判 ← 非空+已见证+锚点一致
    │       │
    │       └── 负样本容器 ← 记录被拒绝的容器
    │
    ├── 产权流判定 ← 侵权自动判定树
    │       │
    │       └── 边界分析 ← 加法vs乘法边界比较
    │
    └── 部首星图 ← 汉字按部首/笔画归入星图
```

---

## 快速启动命令

```bash
cd /mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os

# 运行全部核心demo（验证所有工具可用）
cd tools/phase-radix && nim c --cc:clang -r pe_demo.nim
cd tools/mltt-cipher && nim c --cc:clang --path:. -r mltt_demo.nim
cd tools/distinguisher && nim c --cc:clang --path:. -r df_demo.nim
cd tools/manquanfa && nim c --cc:clang -r mq_demo.nim
cd tools/radix-star && nim c --cc:clang -r rs_demo.nim
cd tools/property-judge && nim c --cc:clang -r pj_demo.nim
cd tools/symbol-engine && nim c --cc:clang -r se_demo.nim
cd tools/symbol-registry && nim c --cc:clang -r sr_demo.nim
cd tools/boundary-engine && nim c --cc:clang -r be_demo.nim
```

---

## 编译注意事项

1. **Nim版本**：使用仓库内置的 `nim-2.2.2/bin/nim`
2. **编译命令**：`nim c --cc:clang`（c=compile, clang=使用clang编译器）
3. **运行**：加 `-r` 参数（compile and run）
4. **路径**：模块在子目录时需要 `--path:.`
5. **func vs proc**：有副作用（I/O、全局状态）用 `proc`，纯函数用 `func`

---

*路径：hunyuan-os/TOOL_REGISTRY.md*
*维护规则：每造一个新工具，必须在此文件添加条目*
*创建日期：2026-08-26*
