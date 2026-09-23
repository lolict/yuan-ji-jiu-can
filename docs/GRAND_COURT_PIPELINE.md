# 大审判庭全链路裁判规范 — 囡囝共生 · 编码→衍射→审判→像素→判决

## 1. 定位

大审判庭 (GrandCourt) 把四个独立裁判模块拉通为端到端管道:编码桥接 → 衍射分析 → 类型拓扑 → 像素编码 → 判决输出。每个字符从输入到最终判决经历完整流水线,不是单点判断。大审判庭同时维护全局水质 (GlobalWaterQuality),记录系统整体的审判公正度。

这不是实验室组件 — 大审判庭是囡囝共生"有表达就有审判"原则的物理实现。

## 2. 四模块管道

**编码桥接 (encoding_bridge)**:

```rust
use crate::encoding_bridge::{Family, FlowBits};
```

按 Unicode 码点将字符粗分类为五家族:

| 码点范围 | 家族 | 含义 |
|---------|------|------|
| 0x4E00..=0x9FFF | Boundary | CJK统一汉字 — 我侧边界家族 |
| 0x0041..=0x007A | Container | 基本拉丁 — 容器家族 |
| 0x00C0..=0x00FF | Couple | 拉丁扩展(带声调) — 夫妻家族 |
| 0x2E80..=0x2EFF | Web | CJK部首扩展 — 网状家族 |
| 0xFF00..=0xFFEF | Container | 全角符号 — 容器家族 |
| 其他 | Neutral | 中性 |

关键设计:带声调的拉丁字母 (á=0x00E1) 归入 Couple 家族,与基本拉丁 a=0x061 (Container) 严格分开。这就是 CrimeTranscript "a≠á 就是犯罪"的大审判庭物理实现 — 不同家族字面量被视作不同类型,跨家族运算是类型错误。

**衍射分析 (DiffractionMirror)**:

将字符的 8 维特征 (审美·纯度·相·结构·思维·协作·目标·义) 投影为 DiffractionPattern。`char_to_features` 对不同家族赋予不同特征值 — Couple 家族在 ownership_righteousness 维度置 0.8 (强调产权正当性),Neutral 家族全 0.5,Broken 视角下全部 0.0。

**类型拓扑审判 (Judge + CrimeTranscript)**:

对 Latin 扩展区字符调用 `judge.judge_type(os, cp, "base", "accented")` — 验证 base → accented 是否满足类型拓扑的衍射条件。a→á 是跨家族,视为拓扑错误 ;á→â 是同家族(均为 Latin 扩展),视为合法蜕变。

**像素编码 (glyph_pixel_algebra)**:

审判通过的表达式经 `phase` 函数编码为像素相位。未通过的字形将以高对比警示色呈现(红色 + 震动波形)。

## 3. 全局水质跟踪

```rust
pub struct GrandCourt {
    pub judge: Judge,
    pub water_quality: GlobalWaterQuality,
    pub total_processed: u64,
    pub total_guilty: u64,
    pub total_acquitted: u64,
}
```

`GlobalWaterQuality` 初始化为 0.5 (中性)。每次审判结果反馈到水质:
- 定罪且事后确认 (`verdict_accepted = true`) → 水质上升
- 定罪但被推翻 (`verdict_accepted = false`) → 水质下降
- 无罪释放且事后确认 → 水质微升

水质报告由 `water_report()` 生成。水质低于 0.3 时判定为"系统裁判可信度低",触发 MetaJudgeEngine 的裁判漂移告警。

## 4. 全链路判决结果

```rust
pub struct GrandCourtVerdict {
    pub input_char: String,
    pub family: Option<Family>,
    pub diffraction: DiffractionPattern,
    pub crime_transcripts: Vec<CrimeTranscript>,
    pub glyph_encoded: bool,
    pub final_guilty: bool,
    pub sentence: String,
}
```

`final_guilty` 由以下规则裁定:
- 跨家族字面量 → guilty
- 同家族但衍射差异超阈值 → guilty
- 同家族且衍射差异在阈值内 → not guilty
- 未通过主权流校验 → guilty

`sentence` 是刑罚/矫正方案的字符串描述,如"折射调整"、"类型归一化"、"家族平移"。

## 5. 综合报告与统计

```rust
pub fn report(&self) -> String
```

输出格式:
```
═══ 大审判庭综合报告 ═══
  已处理字符: N
  有罪: X | 无罪: Y
  定罪率: Z%
  [水质报告]
  [犯罪统计]
```

`crime_stats()` 输出各类型犯罪计数 (如 "跨家族:3,衍射超阈:1,主权拒绝:0")。定罪率是系统自审的硬指标 — 过高说明裁判过严 (可能是 MetaJudgeEngine 出偏差),过低说明裁判过松。

## 6. 与系统其他模块的接入

- **JudgmentApparatus** 的 DiffractionMirror / CrimeTranscript 两组件被本管道直接调用
- **DiffractionMirror** 的 8 维特征来自 PerceptionEngine 的感知编码
- **ConsciousnessRuntime** 每步 tick 将 GrandCourtVerdict 写入 ConsciousnessChronicle
- **CocoonAgent** 的 Observer 角色在处理外部输入时调用 GrandCourt.process()
- **Swarm** 多智能体的协商结果经过 GrandCourt 全链路验证后才被视为系统级合法输出
- **ManQuanFaContainer** 的感知奖励/损失受 verdict_accepted 反馈影响 (确认 → 奖励,推翻 → 损失)
- **Funnel 漏斗节律** 受定罪率反馈:定罪率过高 → 漏斗收紧 (减小节律周期),过低 → 漏斗放宽

## 7. 实现约束

- 五家族的 Unicode 码点范围分类是硬编码匹配 — 不依赖外部 unicode 数据表
- EntityFeatures 的 8 维度在编译期确定,不可运行时增删
- 审判管道的全链路耗时约束:单字符 < 100μs (不含 I/O)
- 水质值的浮点计算使用 f64,比较时使用 ε = 1e-9
- no_std 模式下 Vec<String> 替换为 heapless

// hunyuan-rs/src/judge/grand_court.rs
