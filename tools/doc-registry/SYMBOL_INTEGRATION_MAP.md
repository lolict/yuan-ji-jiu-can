# 符号复用集成图 · IPA × yunmu64 × tone60

## 重叠符号（复用关系）

| 符号 | IPA 位 | yunmu64 位 | 韵母用途 | 进制归属 |
|------|--------|------------|----------|----------|
| ʊ | #4 | — | IPA u-close | 仅IPA |
| ɐ | #0 | — | IPA a-turned | 仅IPA |
| ɏ / Ɏ | #11 | — | IPA barred-y (ü行7解) | 仅IPA |
| ɛ / Ɛ | #1 | #20,#21 | IPA open-e · er韵母 | IPA + yunmu64 |
| ɔ / Ɔ | #3 | #52,#53 | IPA open-o · ong/声母 | IPA + yunmu64 |
| ʉ / Ʉ | #10 | #26,#27 | IPA barred-u · ua韵母 | IPA + yunmu64 |
| ɵ / Ɵ | #9 | — | IPA barred-o | 仅IPA |
| ə / Ə | #6 | — | IPA schwa | 仅IPA |
| ɜ / Ɜ | #7 | — | IPA reversed-open-e | 仅IPA |
| ɨ / Ɨ | #8 | — | IPA barred-i | 仅IPA |
| ɪ | #2 | — | IPA small-cap-i | 仅IPA |
| Ɐ | #0大写 | — | IPA turned-A | 仅IPA |
| Ʊ | #4大写 | — | IPA upsilon | 仅IPA |
| Ɜ | #7大写 | — | IPA reversed-open-E | 仅IPA |

## 复用逻辑

- **ɛ/Ɛ** (#1 = IPA open-e, #20-21 = yunmu er)：er 发音接近 open-e，自然复用
- **ɔ/Ɔ** (#3 = IPA open-o, #52-53 = yunmu ong相关)：ong 的骨架音，自然复用
- **ʉ/Ʉ** (#10 = IPA barred-u, #26-27 = yunmu ua)：ua 发音终点形态，自然复用

这种复用不是节省，是**符号本体即容器**：一个符号在不同进制里自然找到自己的位置。

## 各进制容量

| 进制 | 文件 | 符号数 | 含 ü？ |
|------|------|--------|--------|
| 48 声母 | phonetic-encoding/symbols.nim | 48 | N/A |
| 60 声调 | tone60.nim | 60 | ✅ 完整 |
| 50 变体 | yunmu50_lower/upper.nim | 50 | ✅ #9 |
| IPA特殊 | ipa_lower/upper.nim | 10+1占位 | ✅ #11 ɏ |
| 66 复韵母 | yunmu66.nim | 66 | 间接(ün/üan) |
| 100 总韵母 | fuzzy_pending计算 | 100 | 补全后 |

---

登记：2026-09-18
砖块：#70 @symbol-integration-map
