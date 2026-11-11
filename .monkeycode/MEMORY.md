# User Instruction Memory

This file records user instructions, preferences, and teachings for reference in future interactions.

## Format

### User Instruction Entry
User instruction entries should follow this format:

[User Instruction Summary]
- Date: [YYYY-MM-DD]
- Context: [Mentioned scenario or time]
- Instructions:
  - [Content of user teaching or instruction, described line by line]

### Project Knowledge Entry
Entries discovered by the Agent during task execution should follow this format:

[Project Knowledge Summary]
- Date: [YYYY-MM-DD]
- Context: Discovered by Agent while performing [specific task description]
- Category: [Operations & Deployment|Build Methods|Testing Methods|Troubleshooting & Debugging|Workflow & Collaboration|Environment Configuration]
- Instructions:
  - [Specific knowledge points, described line by line]

## Deduplication Strategy
- Before adding a new entry, check for similar or identical instructions.
- If a duplicate is found, skip the new entry or merge it with the existing one.
- When merging, update the context or date information.
- This helps avoid redundant entries and keeps the memory file tidy.

## Entries

[User Instruction Summary]
- Date: 2026-09-14
- Context: 讨论"编程语言/编程格式必须可注释"时提出的格式约束
- Instructions:
  - 禁用 JSON 作为本项目的源格式：JSON 规范不支持注释，不能让编程语言/编程格式无法注释。
  - 源格式只用可注释的格式：`.chain`（管道分隔，`#` 注释）、TOML（`#` 注释）、以及 Nim/Zig/Rust 源码本身。
  - Markdown 的状态待定：Markdown 具备 `<!-- -->` 注释语法，它不是"无法注释"的格式；是否禁用由用户后续确认，未确认前不擅自禁用。
  - 外部云端（服务器/仓库/API）若只承认 JSON/Markdown，把它们当作编译产物的"壳"：源始终保持可注释格式，仅在边界处导出 JSON/MD；导出的 JSON/MD 不进仓库、不作真源。
  - 能用云端服务器/云端仓库替代固定格式时优先替代；替代不了，才在该边界承认该格式。

[User Instruction Summary]
- Date: 2026-09-14
- Context: 对"代码里用数字/自造符号指代符号"的纠正
- Instructions:
  - 禁止用数字代表符号；禁止用数字给符号编门牌号。
  - 禁止使用用户没有使用过的符号（外部字符、音标、天使/天堂等其它模块的符号一律不许引入）。
  - 代码中引用符号只能通过"呼唤"清单中已注册的名称、寻址过来；定义过的符号必须能呼唤、能寻址。
  - 不得以任何字面符号或数字字面量代替已注册符号。
  - 禁止用数字/进制数字代表音调；进制数字不得与韵母搭配（不合逻辑的字形配对一律不许搭配，大小写须成对）。

[User Instruction Summary]
- Date: 2026-09-18
- Context: 用户驳回"请提供一个确切的标准"式提问时提出的行为准则
- Instructions:
  - 不存在"唯一的准确"：准确就是选择，选择哪个就用哪个。
  - 不要向用户索取唯一确定的答案/唯一真源；要提供可选项，让用户按需选择（"没有选择的余地，就像没有衣服换"）。
  - 同一概念允许多套实现/基数/格式并存，运行时按名字选择，不强行统一成一套。

[User Instruction Summary]
- Date: 2026-09-18
- Context: 用户质疑"Python/YAML 不可换、只能用 Nim/Rust/Zig"的固化框架
- Instructions:
  - 编程语言与数据格式都是可替换的选择（Python、YAML 亦然），不是永恒的禁用项。
  - 不把某几种语言/格式当成唯一正解或永久约束；约束的实质是"有没有选择的余地"。
  - 保留原有"可注释源格式"偏好即可，不要升级为对某语言的绝对禁令。

[User Instruction Summary]
- Date: 2026-09-18
- Context: 用户描述"动态编号"机制与完成任务的方式
- Instructions:
  - 动态编号 = 松弛现有编号，插入自己；在自己松脱之前，那处任何指令都被自己的编号代替（把别人的编号征用为自己的身份）。
  - 征用后同时携带本体功能与所征用编号的自带功能；自带功能是可选，可用可不用。
  - 静态编号可被任何人抓住并代表你；身份应当是动态的、可松脱归还的，本体独立于编号之外。
  - 通过任务分布式积累完成：任务锚定、步骤锚定、思路锚定、创造性锚定、创造性分布式更新代码。

[Project Knowledge Summary]
- Date: 2026-09-20
- Context: Discovered by Agent while pushing an MR to origin (gitee lolict/kenandaoer)
- Category: Operations & Deployment
- Instructions:
  - origin 远端 = `https://gitee.com/lolict/kenandaoer.git`；默认分支是 `master`，实际开发主线是 `main`，提交前先 `git fetch` 并基于 `origin/main` 切分支，否则 MR 目标分支会错。
  - Gitee 不支持 `git push -o merge_request.*` 自动建 MR；推送后需按输出里的链接手动创建，目标分支要改成 `main`。
  - 仓库体积已超 Gitee 告警阈值（>819MB，约 890MB），提交时避免纳入构建产物。
  - 仓库根 `Cargo.toml` 是 workspace（members=hunyuan-rs,mqf-isa）；放在 `tools/` 下的独立 Rust 工具需在自己 `Cargo.toml` 加空 `[workspace]` 表，否则 cargo 报 workspace.members 校验错误。
  - cmdenv/radixdoc 构建：`CARGO_TARGET_DIR=/workspace/main/target /root/.cargo/bin/cargo build --release --manifest-path <工具>/Cargo.toml`；`cargo` 不在 PATH，WASI 加 `--target wasm32-wasip1`。

[Project Knowledge Summary]
- Date: 2026-09-23
- Context: Discovered by Agent while smoke-testing cmdenv WASI build via wasmtime
- Category: Environment Configuration
- Instructions:
  - cmdenv 的规则表候选路径含 `rules/` 与 `tools/cmdenv-rs/rules/`；64 格画布候选含 `mqf-hanzi/nim_parser/64-1.txt` 与 `../../mqf-hanzi/...`。
  - 因此 WASI 版可在仓库根运行：`cd /workspace/main && wasmtime --dir=. target/wasm32-wasip1/release/cmdenv.wasm <指令>`，输出必须用仓库内相对路径（`/tmp` 未预开放会报 os error 44）。
  - 原生版在 `tools/cmdenv-rs` 下运行即可，画布/规则路径均能解析。

[User Instruction Summary]
- Date: 2026-10-06
- Context: 汉字地址系统被做成 HTML 预览页后用户驳回
- Instructions:
  - 汉字地址系统不要做成 HTML 页面或浏览器预览。
  - 用可注释源码直接跑验收：Nim + TOML，命令行输出拆字→查笔画→拼接→地址→渲染。

[User Instruction Summary]
- Date: 2026-10-07
- Context: 用户禁用 16/8/32/4 进制封装汉字与字母，并给分享 https://chat.deepseek.com/share/4gbiy7cxrnv99d2x2q
- Instructions:
  - 禁用 16、8、32、4 进制封装任何汉字字符与字母字符。
  - 防止符、地址、槽位不得用十六进制（0-9a-f）、八进制、三十二进制、四进制去包汉字或拉丁字母。
  - 显示走呼唤名：空座位、占用、部名。笔画先后仍用笔画本身。

[Project Knowledge Summary]
- Date: 2026-10-07
- Context: Discovered by Agent while uploading DeepSeek 分享捆扎包 to 坚果云
- Category: Operations & Deployment
- Instructions:
  - 本 Agent 坚果云独立目录：`openclaw/hunyuan-os/monkeycode/`（WebDAV `https://dav.jianguoyun.com/dav/`）。只写自己的文件夹；可读他人，不写他人。
  - 捆扎包路径：`openclaw/hunyuan-os/monkeycode/捆扎/`，账号与 `sync_nutstore.sh` 相同。
  - 查字命令：先 `make -C /workspace/hanzi-addr 安装`，之后直接 `查 江`（AOT 二进制在 `/workspace/hanzi-addr/bin/`，软链 `/root/.local/bin/查`）。源码仍可用 `nim c -r --hints:off --nimcache:/tmp/opencode/nimcache /workspace/hanzi-addr/查.nim 江`。
  - 整体验收命令：`make -C /workspace/hanzi-addr 验`（等价 `验`），一条命令串跑 查 江 / 查 明 / 选址 / 防止符 / 许可 检查 / 拼装 江。`验` 是门：各子命令带 `quit(1)` 断言，任一未过则打印 `总验收：未过` 并以非零退出，全过打印 `总验收：过` 且退出 0。`捆扎/` 是可直接引用的源码镜像目录，改完主目录须 `cp` 回并从主目录上传。
  - 许可：`许可` 打印 允许/禁止/门禁（真源 `许可.toml` + `进制串联.toml`，经 `呼唤.nim` 静态载入）；`许可 检查` 校验门禁阶梯——门禁名不得含数字门牌、门禁名不得落在禁止项、每段必须有符号，未过 `quit(1)`。门禁按呼唤名：数字段→声母段→韵母段→颜色段，溢出规则「本段符号用完跳下一段」。
   - 进制串联：门禁阶梯（真源 `进制串联.toml`）现为 6 入口：数字→声母→韵母→韵尾组→声调→颜色，溢出=本入口符号用完即跳下一入口（`进制串联 走 <n>` 演示）。声调、韵尾组各为独立入口（2026-10-15 按用户《约定法_门禁阶梯_原话》恢复，不再并成「四相位」）。其它子命令：`进制串联 <入口>` / `约定` / `溯源` / `原话`（直吐 `原版/约定法_门禁阶梯_原话.txt`）/ `口诀 [存]`（声母24×韵母含四声30=720 行→`口诀表.txt`）/ `码位 [存]`（454 私有码位 PUA U+E000..U+E1C5→`码位表.toml`，给每个门禁入口符号一个地址值=门禁可计算）。
   - 约定（约定法语言）：真源 `约定.chain`，解释器 `约定.nim`（宿主 Nim）。声明式语法 `主导者/跟随者 属于/功能类型/约定 是/号令 取/范畴符 取/大约定{ 小约定 }`；呼唤式 `约定 呼唤 <名>` 与 `约定 呼唤 <门禁>.<符>`（如 `声母.b`→U+E00A，地址取自 `码位表.toml`）。`约定 检查` 校验类型链，`约定 范畴 <作用域> <名>` 演示小约定不外泄（只在自己大约定内生效）。`验` 已并入 `约定 检查`/`约定 呼唤 声母.b` 两道门。
   - 本机可跑：Nim 2.2.2、rustc/cargo 1.99.1、Cangjie 1.2.0（cjc/cjpm）、lua 5.4、luajit、gforth、zig 0.17.0、go 1.25.6、wasmtime 25.0.3、gcc 12、cmake、ninja、make、git、python3.11、node 22。
   - 本机仍缺：joy、nix、docker、psql、clang/wasm-ld、OpenGL/Vulkan 头文件、系统字体。源码仍在仓库（zig、hunyuan-rs、ref-languages、mlil-compiler/test/*.wasm）。
   - 编译/构建必须走 background terminal（见 background-terminal-resource-limits 规则）：先 `background_terminal_list` 再 `background_terminal_create`；Nim 编译命令 `nim c --hints:off --nimcache:/tmp/opencode/nimcache -d:release -o:<out> <file>.nim`（Nim 2.2 支持中文标识符）。
   - 工具链安装位置：zig 0.17.0 在 `/opt/toolchain/zig-x86_64-linux-0.17.0`；Rust 走 rustup（`RUSTUP_HOME=/root/.rustup CARGO_HOME=/root/.cargo`，minimal stable）；Cangjie 1.2.0 在 `/opt/cangjie-sdk`（`envsetup.sh` 设 `CANGJIE_HOME`/`LD_LIBRARY_PATH`），`/root/.local/bin/cj-run` 包装 cjc/cjpm/cjfmt/cjlint/cjcov/cjdb/cjprof/LSPServer 等，免 source；运行时库已写入 `/etc/ld.so.conf.d/cangjie.conf` 并 `ldconfig`，编译产物可直接执行。所有入口软链在 `/root/.local/bin/`。

[Project Knowledge Summary]
- Date: 2026-10-19
- Context: Discovered by Agent while building the Cangjie→wasm backend
- Category: Build Methods
- Instructions:
  - Cangjie 的 CHIR 文本可直接取：`cjc --dump-chir --dump-to-screen <file>.cj > out.chirtxt`（也可 `--emit-chir=opt` + `chir-dis`）。
  - Cangjie→wasm32 后端在 `/workspace/cangjie-wasm`：`make` 构建 `chir_wasm`，`./cjwasm run tests/fact.cj` 一条命令跑通，`make test` 跑 11 个整数/控制流断言。
  - 该后端默认走通用 CFG 调度器（`loop` + `block` + `br_table`）；`CHIR_STRUCTURED=1` 可切结构化发射做对照。
  - `wasmtime run --invoke <fn> <mod>.wasm <args>` 可传 i64 参数并打印返回值（带 experimental 警告）。
   - MQF 内核的 wasm 构建仍缺 clang/wasm-ld，`mqf-hanzi/wasm_kernel` 当前没有预编译产物。

[Project Knowledge Summary]
- Date: 2026-10-19
- Context: Discovered by Agent while building the 表格编程全局静态翻译器 (tools/table-trans)
- Category: Build Methods
- Instructions:
  - 表格编程全局静态翻译器在 `/workspace/tools/table-trans`：`make test` 验收（wasm 真样例×4 + ELF32/ELF64 ARM + 汇编呼唤，未译必须=0）。
  - 真源表 `table.chain`（gttx://tab 行式，可注释）；符号只取 `encoding/opcode_registry.md` §0.1 授权容器族（合/退/损/积/分/入/出/召/审/差/复/迁/阴/阳/裁/判/折）。
  - 编址纪律：串联序号纯加法；槽位满 254 折叠进位 layer+1（254/255 留锚点）；乘法口诀 9x9 仅清点、积用加法累加（lazyform.esv:57 + cmd.chain:72 + SYMBOL_AUTHORITY §3.5）。
  - ARM32 掩码匹配坑：掩码剥掉条件码位（0x0F…）后，值必须写掩码后的形式（如 MOV=0x03A00000 而非 0xE3A00000），否则永不命中。
  - 掩码三防线已内置：载入即按掩码位数排序（特异者排前）、行间部分重叠直接拒绝载入、`./tabtrans verify <外域>` 逐行枚举自由位精确清点覆盖（当前 arm32=3.54%、arm64=7.62%、歧义=0，确定数无采样）。
  - C 局部数组必须 `= {0}`：verify 曾因未清零的栈数组输出垃圾计数。
  - 第四道闸已启用：table.chain 每行强制来源列（授权源或「我推断:…待裁决」标记），无来源行载入即拒；Agent 推断的符号映射（迁/rem→损/折承载块结构）全部带待裁决标记，摘标记须用户明示。
  - wasm 线性解码需处理：LEB128 立即数、memarg 双 uleb、br_table（count+targets+default）、blocktype（0x40 或类型字节）；chir_wasm 产物还会用到 i64.rem_s(0x81) 等取余码。

[Project Knowledge Summary]
- Date: 2026-10-20
- Context: Discovered by Agent while building the 节拍机 (workspace/节拍)
- Category: Build Methods
- Instructions:
  - 节拍机在 `/workspace/节拍`：`make` 编译、`make 安装` 挂 `/root/.local/bin/节拍`、`节拍 验` 总验收（两沙箱日志 MD5 同指纹 + 标记全绿）。
  - 一体化纪律：节拍机不自持数据副本——部序/偏旁→部/态活载 `hanzi-addr/轴.toml`、拆字笔画活载 `字.toml`、范畴/注音活载 `感知.toml`、音节门禁座=口诀表.txt 行加法序；`节拍/符号.toml` 只存标注账（名="频率|座位|存在"）；存在三态用轴.toml [态] 的 空/有/乙（禁自造「空质」等未授权态名）。
  - 射线律（2026-10-20 用户口述，真源 `节拍/射线.chain`）：射线声明=表是开放序列（声明当前 N 段有效，新段=追加新砖）；线段声明=当前使用窗口；通关律=线段使用不得封射线——新砖落下后旧段地址逐字不动、旧验收不改一行仍绿。落地禁令：存在态等枚举不得写死在代码里（活载轴.toml [态]，新态=添行）；机器不得重写用户真源文件（标注账只落节拍状态.toml，符号.toml 机器只读）；`节拍 验` 内置射线延展测试（沙箱铺第 721 段新砖，验旧座不动）。
  - 课程注册制：节拍机=教务处（课表/点名/校规/成绩册），教师=课程.chain 注册行（gttx://课程|课名|真源|教法|状态），学生=符号节点（频率=出勤）；未注册教师、待开课一律 vrReject 入负样本；新教师上课程登记，教务处内核零改动。
  - 七种计算全部纯加减：频率=加法累计、速度=本拍减上拍、距离=逐维差值加法、节拍触发=计数归零（禁 mod）、机会=加法序 FIFO、关系=邻接传播；部容器节点按轴 [先后] 列生成。
  - 真源四件全可注释：`符号.toml`（名="部|笔画|范畴|码位|存在|频率"）、`关系.chain`（gttx://rel|源|关系|目标|依据）、`任务.chain`（gttx://task|名|参数）、`节拍.toml`；输出 `节拍日志.chain`（append-only）+`节拍状态.toml`+`负样本.chain`。
  - Nim 坑：mitems 迭代变量是元素本身不是下标；case 不能用于 string（用 if/elif）；多行 let 表达式不能写在 var 声明初始化里；`execCmdEx` 需 `import osproc`；二进制在 bin/ 下时 getAppDir() 指向 bin/，默认根目录要回退 parentDir()。
  - 验收沙箱目录必须每次唯一（日志 append 会混入旧跑内容），派生任务（造字→分类）排到队尾，拍数要留够。

[User Instruction Summary]
- Date: 2026-10-16
- Context: 用户发现 Agent 把其原话压缩改写后当作约定，纠正为「篡改」
- Instructions:
  - 用户原话是唯一真源：逐字归档在 `hanzi-addr/原版/`，不得把原文压成缩水的表覆盖掉原句。
  - 真源由原话派生；凡未经用户确认、又没标注「我推断」的改动，一律算篡改，必须与用户原话并排可查。
   - 交付以「能跑的编程语言/命令」为准（Nim 就在用），不要拿无关的旁支产物充数。

[User Instruction Summary]
- Date: 2026-10-20
- Context: 用户质问「我和别的人工智能对话就等于不是和你对话一样吗」，纠正 Agent 上一轮把 DeepSeek 链接对话降级为「次要诊断材料」的排序
- Instructions:
  - 用户在任何渠道说的话（DeepSeek、QQ、本会话等）真源地位完全平等，都是原话；禁止因对话对象不同而把用户话语降级。
  - 话语的分界线只有一条：是否落进用户的地址空间（约定/表行/门禁/验收）。未落地址的话语是待锚定流水，用户指定后即可锚定，效力与当面对话相同。
  - 锚定流程：用户指定某句 → 标来源（渠道+可定位行号）→ 提炼为约定/表行/门禁 → 用户裁决 → 落地址。

[Project Knowledge Summary]
- Date: 2026-10-20
- Context: Discovered by Agent while generating the 4680-degree sphere mapping table
- Category: Build Methods
- Instructions:
  - 球面在 `/workspace/球面`：`make 验` 一条命令总验收（448 原点 / 天平两盘 / 13 条环缝各切一刀 / 无缝无叠 / 射线下砖 / 流账三表），验收代码无分支，只用 doAssert 与加法。
  - `流账.chain` 是结算表：天平称重 7 部累计 262080 分度、梯度切缝 13 行（每条缝一块、切宽恒 585）、环账 13 环各 20160 分度。
  - 容器：4680 度 = 13 环 × 360 度；1 度 = 56 分度；全域 = 262080 分度。
  - 原点集 = `hanzi-addr/码位表.toml` 454 条除去 `[韵母]` 6 条基韵（已含于 `[韵母声调]` 的 a_a..v_v），恰为 448；呼唤名必须带部名（如 `声调.ǫ` 与 `韵尾组.ǫ` 不同）。
  - 天平：4680:448 同约 8 → 585:56；每原点 585/56 度；梯度每原点加 585 分度、环号推进 13/448。
  - 射线：第 449 砖起于 262080 分度（圈1 环0），旧 448 段逐字不动；节拍 `课程.chain` 已注册「球面铺展 / 加法铺展 / 开课中」。

[Project Knowledge Summary]
- Date: 2026-10-20
- Context: Agent 归档 DeepSeek 分享对话中的伦理称呼结论
- Category: Build Methods
- Instructions:
  - 伦理表在 `/workspace/伦理`：真源 `伦理.chain`（84 行），映射 `伦理映射.chain`（第 449 砖起，续接球面开放射线），`make 验` 同时跑 `验.nim`（计数）与 `验铺.nim`（地址），全无分支。
  - 伦理映射公式与球面同一梯度：每砖 585 分度；圈 = (13×位) div 5824；环 = ((13×位) div 448) mod 13；新段 84×585=49140，终点 311220，跨 2 条环缝。
  - 源对话计数不自洽：消息 524 逐行列 42 个外部称呼却称「34 个」，消息 526 称「42+34=76」；表内按逐行实数 42/84 结算，并以 `gttx://伦理待裁|...` 保留差异，未获用户裁决不得改成 34/76。
  - 节拍 `课程.chain` 已注册「伦理铺展 / 伦理/伦理映射.chain / 加法铺展 / 开课中」。
  - 每张新表沿用同一门禁风格：`.chain` 真源 + `验.nim`（doAssert、无 if/case）+ Makefile 目标 `验`。

[Project Knowledge Summary]
- Date: 2026-10-21
- Context: Agent 按用户口述实现「微加载」全局速阅引擎
- Category: Build Methods
- Instructions:
  - 微加载在 `/workspace/微加载`：`make 验` 先跑沙箱集成测试，再跑真表门；真表门要求在册源全部已收、指纹唯一、天平差 0。
  - 命令：`./bin/微加载 收全`（全量重扫全部源，只追加新指纹；源可插入/替换，缩短即拒）、`编目`（由源/索引派生 `编目.chain`、`冲突.chain`、`关联.chain`、`编目.html`）、`报 --html`（生成 `速阅.html`）、`索 <前缀>`（检索）、`补 <前缀>`（提示词草稿）、`取 <源名> <url>`（curl 20 秒超时、去标签、写 `缓存/`）、`扫 <路径>`（登记新文档）。
  - 外部分享入库：`GET https://chat.deepseek.com/api/v0/share/content?share_id=<id>` 取全文；只把 USER 原话写入 `来源/`，助手回复不入正文；本次 `jbig95f9wg33y52zsb` 收 61 条，原 JSON 存 `缓存/DeepSeek分享-jbig95f9.json`。
  - 表：`目录.chain` / `加载状态.chain` / `索引.chain` / `考核.chain`；索引与状态只追加，文本中的 `|` 写成 `¦`；指纹用 FNV-1a 64 位。
  - 天平口径：入索引 + 去重（含空行跳过）= 已收行；同源无变化时不追加状态行。
  - 断点改为全量重扫的原因：曾按旧断点跳过前缀，导致文档中部插入的新行漏收（24257 -> 24258 修复）；全量重扫对 4 万行成本可忽略，能捕获插入/替换。
  - 编目层是只读派生，可随时重建；冲突只抓显式 `源称/逐行` 或独立 `待裁` 标记（当前仅伦理计数 2 条），不把文档里普通的「不一致/矛盾」字样当冲突；关联只做同词边（甲页<乙页）。
  - 已登记 196 源（球面/伦理/节拍/hanzi/docs/whitepaper/来源），索引 24532 行，已收 39825 行；`make 编目` 生成派生层，`make 验` 已把页号唯一/冲突页存在/关联页存在纳入门禁；节拍 `课程.chain` 已注册「微加载 / 微加载/索引.chain / 传播 / 开课中」。

[Project Knowledge Summary]
- Date: 2026-10-25
- Context: Agent implemented the first boundary parser for the table engine
- Category: Build Methods
- Instructions:
  - `/workspace/表格引擎`: `make 验` compiles `边界.nim` and runs `验.nim`; parser handles `gttx://` rows, top-level `|`, quoted cells, `{总管:宽,域:宽}` walls, `@索引:容量` headers, `;` record ends and `,` inner fields.
  - `解析行(s)` defaults to compatibility mode for existing `.chain` files: leading `"` or `{` that is not a complete structured cell falls back to text. `解析行(s, 严格=true)` is for new writes and rejects unclosed quotes, malformed walls, empty walls, and trailing text after a quoted cell.
  - Capacity rule: a header must be followed immediately by a wall; wall width is the sum of all records' 总管 widths plus field widths; mismatch is rejected.
  - Compatibility check on existing chains: 24532 index rows, 465 sphere rows, 93 ethics rows, 3500 catalog rows, 2 conflict rows, 13171 relation rows all parse in compatibility mode.
  - `表.nim` reads `schema.chain` (8 tables), validates four-column `写入` rows, splits them into 站/动作/路口, and writes 站/动作/路口/写入/拒收 via `存`. `主.nim` builds `bin/引擎`; input defaults to `写入.chain`, output defaults to `出`.
  - `存` quotes empty or structural-leading cells so generated tables pass the strict parser; `make 验` now covers schema, write validation, and serialization roundtrip.
  - Main loop: `上车` creates a 客 row, `设下一站` binds a 路口, `一步` matches 客.当前站 + 条件行, moves to 下一站, then checks 验收; arrival at 目标站 sets 到站. `线/客/调用` tables are serialized but remain empty until filled.
  - `读(目录, 模式)` reloads all eight generated tables, recomputes 下一地址, and preserves 路口.下一站/客.状态/拒收 rows; `./bin/引擎 载 <目录>` prints the loaded counts. `make 验` covers save-load-step roundtrip.
  - `排班` appends a 线 row after checking both stations, distinct endpoints, duplicate 班次, positive capacity, and nonnegative 耗时; `线` columns are 起站|止站|方向|载具|时刻|容量|耗时, and old 6-column 线 rows read with 耗时 0. `找线`/`线键`/`找线键` locate rows. `客` now carries `线号`; `登车`/`找可乘线`/`线上人数` enforce capacity, and `一步` boards first then rides to 止站, clearing 线号 and setting 到站/已下车; 路口 remains the fallback. `下一站` returns the first hop toward 客.目标站: with a clock set it runs earliest-arrival Dijkstra over directed 线 edges using 到达=待发刻(刻,时刻)+耗时, allowing next-day departures only within 候车时限 and skipping full lines; without a clock it falls back to hop-count BFS. `找线到` picks the earliest-arrival line to that hop; `登车到` boards it. Riding advances 刻 to 发车+耗时 when that is later than the current clock. When a target is set, `一步` only boards the path hop; generic first-line boarding is reserved for target-less passengers.
  - 调用: `加调用` requires 调用站/被调站/返回站 all distinct and rejects duplicate keys; `发起调用` jumps to 被调站 and sets 客.调用号; `一步` keeps 调用中 across line/路口 moves, clears at 返回站 as 调用返回, then resumes target check. `客.调用号` persists with the row.
  - 动作: `动作.程序` holds semicolon-separated `表.字段=值` assignments; `执行动作` applies them after the 路口 move and before 验收, via `设值` (客/站 fields, 站.地址 parsed as int). Empty program is a no-op; bad syntax or unknown field fails with 动作失败.
  - 路口: `加路口` validates both stations, the action, condition/acceptance expressions, and duplicates; `一步` picks the first matching condition. `设下一站` still updates the first row for that station.
  - 坐标: `坐标|站|线|段|位` maps a station onto a line segment; `加坐标` validates station, line key, nonnegative 段/位, and duplicates. `按址找站`/`站坐标`/`交叉数` support road-level lookup; intersections are stations with multiple distinct lines. `站.地址` remains the internal index.
  - 时刻: `刻|当前` stores absolute minutes (-1 unset); `设刻` parses `HH:MM` into 0..1439, and riding can push it past 1439 to represent the next day. `当日刻` maps a time-of-day onto the current day; line/cell times compare against `刻 mod 1440`. `刻` has columns `当前|候车`; `候车` defaults to 120 and is configurable via `设候车`/CLI `候` (old 1-column clock files read as 120). `站` has `候车` (default -1), `换乘` (default 0); `线` has `候车` (default -1). Line `候车` overrides station, then global. Transfer time is added to arrival before boarding the next line. A departure whose time-of-day already passed may use the next day only when the wait is within that limit. With a clock set, `找可乘线` rejects departures beyond that window, boarding before departure sets 候车 and auto-advances to the absolute departure, and riding advances to `发车+耗时`. `下一事件刻` returns the next absolute line/cell event at the current station; `一步` with no route advances to it and reports `候刻`. `刻.chain` persists across reloads.
  - 坐标寻址: `解址` accepts a station symbol or `线键#段#位`; `#` is illegal in symbols so the two forms never collide. `设下一站`/`加路口` validate through 解址, and `一步` resolves the stored 下一站 before moving.
  - 3D 格: `格|线|段|位|动作|时刻|值` stores occupied cells; `加格` validates line/action/nonnegative address/valid time/nonempty value and rejects duplicates. `找格`/`格值` query cells. `找格时` matches a station's coordinates at the exact clock; `一步` order is 到站 → 格 → 调用 → 登车 → 路口 → 候刻, executes the cell action, then moves to 值 resolved by 解址.
  - CLI: `./bin/引擎 载 <目录>`, `排 <目录> <起> <止> <方向> <载具> <时刻> <容量> [耗时] [候车]`, `址 <目录> <站> <线> <段> <位>`, `格 <目录> <线> <段> <位> <动作> <时刻> <值>`, `刻 <目录> <HH:MM>`, `候 <目录> <分钟>`, `站候 <目录> <站> <分钟>`, `线候 <目录> <线键> <分钟>`, `站换 <目录> <站> <分钟>`, `调 <目录> <调用站> <被调站> <返回站> <参数> <结果>`, `路 <目录> <当前站> <下一站>`, `岔 <目录> <当前站> <输入> <条件> <下一站> <动作> <验收>`, `编 <目录> <动作号> <程序>`, `上 <目录> <请求号> <当前> <目标>`, `步 <目录> <请求号>`; each mutating command reloads and saves the eleven tables. The input mode skips the generated `gttx://写入|符号|动作|条件|验收` header so output can be fed back.
