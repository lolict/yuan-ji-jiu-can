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
