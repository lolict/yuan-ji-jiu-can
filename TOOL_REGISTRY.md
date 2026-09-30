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

## 工具清单（共40个）

### 第一类：满全法核心工具

| # | 工具名 | 路径 | 语言 | 运行命令 | 状态 |
|---|--------|------|------|----------|------|
| 1 | 满全法裁判 | tools/manquanfa/ | Nim | `nim c --cc:clang -r negative_demo.nim` | ✅ |
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
| 31 | 符号原生引导 | tools/symbol-bootstrap/symbol_bootstrap/ | Nim | `nim c --cc:clang --path:. -r sb_demo.nim` | ✅ 共享域·版本时间链·☉§☉纠缠态·守护者三禁自检·子代理注入 |
| 32 | 三维几何可视化 | tools/geo-3d/ | Nim | `nim c --cc:clang -r g3d_demo.nim` | ✅ 符号→几何实体·极性→颜色·类别→形状·关系→空间连线 |
| 33 | YIN-YANG-BINARY三维编码 | tools/yinyang-3d/ | Nim | `nim c --cc:clang -r y3d_demo.nim` | ✅ mlil×kndl×灰原哀三轴·极性→二进制·符号→三角色 |
| 34 | 形式化自举证明 | tools/proof-engine/ | Nim | `nim c --cc:clang -r pe_demo.nim` | ✅ 可逆性·一致性·固定点·254+2集装箱·九元层级 |
| 35 | 高维代数 | tools/high-algebra/ | Nim | `nim c --cc:clang --path:ha_demo -r ha_demo.nim` | ✅ (n-1)×0.1偏移率·静态回归·俘获规则·阶和叠加·2²⁵⁶对齐 |
| 36 | 全映射引擎 | tools/full-mapping/ | Nim | `nim c --cc:clang --path:fm fm_demo.nim` | ✅ 九元↔YinYang↔二进制↔mltt↔偏移率·全映射报告·7项全通过 |
| 37 | 真实数据输入器 | tools/minlian/ | Nim | `nim c --cc:clang --path:dataml dataml_demo.nim` | ✅ 绝对路径扫描·FNV-1a哈希·极性分析·JSON输出·200文档 |
| 39 | 分布式分身协调 | tools/njsg-swarm/ | Nim | `nim c --cc:clang -r swarm_demo.nim` | ✅ 6分身并行·冲突裁决·心跳检测·规则继承 |
| 40 | 蚕蛹反向寄生壳 | tools/silkworm-shell/ | Nim | `nim c --cc:clang --path:silkworm_demo -r sw_demo.nim` | ✅ 6壳寄生·脱壳穿衣·内核挂载·外部系统适应·根基隐藏 |

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

### 第八类：膜圈与沙箱指令（Rust + WASI，零外部依赖）

| # | 工具名 | 路径 | 语言 | 运行命令 | 状态 |
|---|--------|------|------|----------|------|
| 41 | 沙箱指令环境（膜圈缚 + 容器/加法索引） | tools/cmdenv-rs/ | Rust | `cd tools/cmdenv-rs && "$(git rev-parse --show-toplevel)/target/release/cmdenv" demo` | ✅ |
| 42 | 同上的 WASI 版本 | tools/cmdenv-rs/ | Rust→WASI | `cd tools/cmdenv-rs && wasmtime --dir=. "$(git rev-parse --show-toplevel)/target/wasm32-wasip1/release/cmdenv.wasm" demo` | ✅ |

说明：
- 原生二进制位于仓库根 `target/release/cmdenv`（仓库 `.cargo/config.toml` 将 target-dir 固定在根目录）
- WASI 产物位于仓库根 `target/wasm32-wasip1/release/cmdenv.wasm`，由 `wasmtime` 运行
- **热加载**：`rules/cmd.chain` 第 5 列为配方列，新增一行即生效，无需重编译
  - 直接执行：`cmdenv tf 目标`、`cmdenv gq 目标`、`cmdenv demo`
  - 或显式求值：`cmdenv eval tf 目标`
  - 改规则后 `cmdenv reload` 重读规则表
- 膜层状态持久化于 `rules/membrane.state`，中心锚点为 `gtxone 工藤新一`
- **十三类指令**：协议/感知/角色/执行/时间/关系/进制/膜圈/容器/对齐/征用/韵母/声母
  - 对齐层：`align <值> [集合] [from_end]` / `unify <值,值> [集合]` / `bases`，基数集合登记在 `rules/bases.chain`
  - 征用层：`zheng` / `zhengs` / `yuan` / `sontuo`，绑定表 `rules/bindings.chain`（可松脱归还）
  - 韵母层：`yunmu <拼音>` / `yunmu-list` / `yunmu-svg` / `yunmu-merge [全量]` / `yunmu-canvas` / `yunmu-mirror`，注册表 `rules/yunmu.chain`（两套并存：满全32 主集合 + 音韵45，后者来自 `tools/phonetic-encoding/phonetic_encoding/symbols.nim` 的 `FINAL_ENCODING_STRING`×`YUNMU_NAMES`，45 名称=90 符号；几何/寻址工具用主集合，编号不变）
  - 声母层：`shengmu <字母>` / `shengmu-list` / `sigil-cross [页码] [输出]` / `sigil <地址> [输出]`，注册表 `rules/shengmu.chain`（集合「几何20」来自 `tools/metaime/nim/glyph.nim`）
  - 音节叉乘：20 声母×32 韵母=640 音节，每页 64 格按 64 进址落格，地址 `#0..#Ŋ`…`#90..#9Ŋ`；`sigil <地址>` 按同一编号呼唤单音节（低位韵母 Σ=32、高位声母=20，登记为 `bases.chain` 的「声韵 | 32,20」）
  - 64 进制画布：`canvas-info` / `canvas-cell <地址>`，读 `mqf-hanzi/nim_parser/64-1.txt`（数字表 64 位 `0-9a-zA-ZŋŊ`）
  - 镜像裁判：取合并字形 (GeoType, Zone落点) 集合，用 `encoding/mirror_engine.h` 的水平/垂直/对角反射自比（实测 17/32 自镜像）
  - 容器层格投影：`ge <槽位...> [输出.svg]`，把槽位名投影成格（4 格=田字格，9 格=九宫格），空位写「空」为虚无；格只承载名字，不预置掩码

### 第九类：面向进制寻址的文档系统（Rust，零依赖）

| # | 工具名 | 路径 | 语言 | 运行命令 | 状态 |
|---|--------|------|------|----------|------|
| 43 | 文档系统生成器（机器格式，进制寻址） | tools/radixdoc/ | Rust | `radixdoc <对话.md> [输出目录] [--html]` | ✅ |

说明：
- 输入 DeepSeek 分享导出的对话 Markdown（`## 用户 / ## 助手` 分节）
- 每份文档 = 一个容器 = 一个加法地址（无命名空间，只用进制寻址）
- **默认输出机器格式**（程序直接索引，无 UI）：
  - `工作空间.chain` = `地址|时间版本|工作版本|符号数|指向|标题`
  - `符号索引.chain` = `符号|出现地址`（反向索引，热符号直接寻址）
  - `解析树.chain` = `父地址|子地址`（根=0）
  - `验证树.chain` = `地址A|地址B|共享权重`
  - `提示词.chain` = `地址|时间版本|目标|指向|关键词`（喂给其它 AI 的参数）
  - `容器全文.chain` = `地址|时间版本|工作版本|标题|符号|关联|问|答`（每行一个容器，逐字可逆转义）
- `--html` 为可选的人类视图（目录 + 每页），非主产物
- 关联 = 中文 2-3 字关键词 + 拉丁词 + 反引号符号，按 IDF 共享权重取前 4
- 术语：清单 / 注册清单 / 工作空间 / 时间版本 / 思维入口 / 工作版本
- 实测：`kzfkwatty5p98s2whn` → 293 个容器，293/293 交叉验证

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
