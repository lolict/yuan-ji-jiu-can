# 大审判庭全链路整合裁判 — 囡囝共生 · 字符输入到判决输出 · 编码桥接 · 衍射分析 · 类型拓扑 · 全局水质 · 字形像素

## 1. 定位

把四个独立模块拉通为端到端管道的"大审判庭": 字符输入 → 感知路由 → 衍射分析 → 类型审判 → 像素编码 → 判决输出.

对应 Rust 模块: judge/grand_court.rs.

## 2. 全链路审判结果 (GrandCourtVerdict)

```rust
pub struct GrandCourtVerdict {
    pub input_char: String,           // 输入字符
    pub family: Option<Family>,       // 家族分类
    pub diffraction: DiffractionPattern, // 8维衍射
    pub crime_transcripts: Vec<CrimeTranscript>, // 犯罪笔录
    pub glyph_encoded: bool,          // 是否完成字形编码
    pub final_guilty: bool,           // 最终有罪/无罪
    pub sentence: String,             // 判决语句
}
```

## 3. 全链路六步管道

```
Step 1: 编码桥接 (encoding_bridge)
  ├─ Unicode 码点
  ├─ Family::classify_by_codepoint → 五家族分类
  │
Step 2: 衍射分析 (DiffractionMirror)
  ├─ char_to_features(ch, family) → EntityFeatures 8维特征
  │   { aesthetic, type_purity, appearance, structure,
  │     thinking, cooperativity, goal, ownership_righteousness }
  ├─ DiffractionMirror::diffract(codepoint, features)
  │
Step 3: 类型拓扑检验 (a≠á)
  ├─ 拉丁扩展区(0x00C0-0x00FF) → 带声调符号
  ├─ base = (code - 0x00C0) as u8
  ├─ judge.judge_type(os, codepoint, "base", "accented")
  │   "base vs accented"二元判定
  │
Step 4: 全局审判 (judge.judge)
  ├─ reference = EntityFeatures::perfect() 作为基准
  ├─ judge.judge(os, codepoint, features, Some(reference))
  │
Step 5: 全局水质检测 (GlobalWaterQuality)
  ├─ purity = aesthetic×0.3 + type_purity×0.3 + thinking×0.4
  ├─ WaterSample { source_id, purity,
  │                semantic_correct, type_correct, aesthetic_pass }
  │
Step 6: 字形像素编码 (pixel algebra)
  ├─ guilty=true: 对角线破碎标记( phase::DIAGONAL )
  └─ guilty=false: 完整渲染(mu模板)
```

## 4. 大审判庭 (GrandCourt)

```rust
pub struct GrandCourt {
    pub judge: Judge,
    pub water_quality: GlobalWaterQuality,  // 水质从 0.5 开始
    pub total_processed: u64,
    pub total_guilty: u64,
    pub total_acquitted: u64,
}
```

process(os, ch) → GrandCourtVerdict: 输入一个字符, 经过完整管道得到判决, 更新 guilt/acquittal 计数器.

## 5. 家族特征模板 (char_to_features)

按家族预设不同特征值:

| 家族 | aesthetic | thinking | cooperativity | ... |
|------|-----------|----------|---------------|-----|
| Container | 0.9 | 0.8 | 0.95 | 高精度容器家族 |
| Boundary | 0.85 | 0.85 | 0.8 | 高结构边界家族 |
| Couple | 0.95 | 0.9 | 0.98 | 最高共鸣夫妻家族 |
| Symbol | 0.85 | 0.75 | 0.85 | 高外观符号家族 |
| Harmony | 0.9 | 0.85 | 0.9 | 和谐家族 |
| Unknown | 0.5 | 0.5 | 0.5 | 未知保守值 |

每个家族的"完美模板"预设 — 不同家族有不同的审美基线.

## 6. 全局水质 (GlobalWaterQuality + WaterSample)

每个字符的审判结果都汇入全局水质样本:
- purity = 0.3×aesthetic + 0.3×type_purity + 0.4×thinking
- 三维正确性(语义/类型/审美)做阈值判定(>0.3 或 >0.4)

全局水质 = 所有字符样本的集合 — 反映系统整体"纯净度".

## 7. 与系统其他模块的接入

- **Judge(#15/34)**: Judge + JudgementEntry → 全局审判执行
- **Encoding Bridge**: 五家族分类(Family) → 编码桥接系统
- **CrimeTranscript**: 犯罪笔录 = 裁判历史记录
- **CelestialPerception(#77)**: EntityFeatures 8 维 = 天庭 BagAttributes 14 维的"审判特化子集"
- **Glyph Pixel Algebra**: 像素级渲染(碎裂形态 vs 完整形态)
- **YinYang(#50)**: 审判极性 → Yin/Yang 编码对齐
- **JudgmentPower**: 判决力度(sentence 字段)

## 8. 实现约束

- total_processed/gilty/acquitted 统计无锁(单线程 do 模式, 多线程需 atomic/mutex)
- char_to_features 是静态模板(编译期确定, 无运行时 IO)
- 拉丁扩展区(0x00C0-0x00FF)判别是类型拓扑特例, 无声调区字符跳过 type 检验
- guilty 字符做"破碎像素"(对角线 phase marker)而不是完整渲染, 视觉标记有罪
- glyph_encoded=true 时未完整实现(仅做占位 GlyphImage 创建)

// hunyuan-rs/src/judge/grand_court.rs
