# ü 符号完整性审计 — 全系统结论

## ✅ 已实现：IPA 编码文件已生成

- `tools/kungguan-container/ipa_lower.nim` — 11个小写符号（含行7补 ɏ）
- `tools/kungguan-container/ipa_upper.nim` — 11个大写符号（含行7补 Ɏ）
- `ipa_upper` import `ipa_lower`，提供 `ipaPair()` 配对查询
- 新增砖块：#68 @ipa-lower · #69 @ipa-upper

---

## 审计范围：工作区所有含 ü 的编码文件

| 文件 | 含 ü？ | 详情 |
|------|--------|------|
| tone60.nim | ✅ 完整 | ǖǘǚǜ + 轻声ü，大小写全 |
| yunmu50_lower.nim | ✅ #9 | ü (U+00FC, u+分音) |
| yunmu50_upper.nim | ✅ #9 | Ü (U+00DC) |
| phonetic-encoding/symbols.nim | ✅ #3,#4 | Ӱ/ӱ (Cyrillic), Ü/ü 二套标记 |
| yunmu66.nim | N/A | 复韵母系统，ün=ỹ, üan=ѫ — 无单ü位 |
| encoding/phono_codec.c | ✅ | ʊ̈ 作为 ue变体 |
| ipa_lower.nim | ✅ #11 | ɏ = ü行7杠系列解 |
| ipa_upper.nim | ✅ #11 | Ɏ = ü行7杠系列解 |

## IPA 符号配对（行6-7 旧缺 · 已补全）

行7 ü位 (idx=11): ɏ (U+024F) ↔ Ɏ (U+024E) — 已编码
行6 ü位 (idx=5): 仍留空待补（无合适 Unicode 候选）

---

登记时间：2026-09-18
砖块：#67 @missing-u-pairs · #68 @ipa-lower · #69 @ipa-upper
