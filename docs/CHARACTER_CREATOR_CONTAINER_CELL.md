# 笔画造字系统 · 字符=容器单元 — 囡囝共生 · 笔画→容器 · 字形编码 · 韵母绑定 · 夫妻在场标识

## 1. 定位

造字系统的核心命题: 每个汉字由笔画序列构成, 每笔画对应一个容器, 汉字本身即为多个容器的编码集合. stroke_database.rs 代理至 encoding/character_creator.rs, 体现满全法"容器=字形"——字的存储就是容器组合, 字的显示就是容器的可视化.

## 2. 笔画容器一一对应 (16 笔画 ↔ 16 容器)

```rust
pub const STROKE_HENG:      u8 = 0;   // 横(一) → 袋
pub const STROKE_SHU:       u8 = 1;   // 竖(丨) → 桶
pub const STROKE_PIE:       u8 = 2;   // 撇(丿) → 罐
pub const STROKE_NA:        u8 = 3;   // 捺(㇏) → 盆
pub const STROKE_DIAN:      u8 = 4;   // 点(丶) → 盂
pub const STROKE_TI:        u8 = 5;   // 提(㇀) → 壶
pub const STROKE_HENGZHE:   u8 = 6;   // 横折(㇕) → 盒
pub const STROKE_SHUZHE:    u8 = 7;   // 竖折(㇗) → 瓶
pub const STROKE_HENGPIE:   u8 = 8;   // 横撇(㇇) → 盏
pub const STROKE_SHUGOU:    u8 = 9;   // 竖钩(㇚) → 盎
pub const STROKE_HENGGOU:   u8 = 10;  // 横钩(㇛) → 篓
pub const STROKE_WANGOU:    u8 = 11;  // 弯钩(㇜) → 筐
pub const STROKE_SHUWANGOU: u8 = 12;  // 竖弯钩(㇄) → 篮
pub const STROKE_HENGZHEZHE:u8 = 13;  // 横折折(㇆) → 箩
pub const STROKE_SHUWAN:    u8 = 14;  // 竖弯(㇁) → 箕
pub const STROKE_XIEGOU:    u8 = 15;  // 斜钩(㇂) → 笼
```

笔画名(横竖撇捺点提...)与容器名(袋桶罐盆盂壶...)一一映射, 构成 64_keys.md 定义的前 16 位.

## 3. 静态笔画表 (STROKE_TABLE)

```rust
pub struct StrokeEntry {
    pub ch: char,
    pub strokes: &'static [u8],  // 笔画索引序列
}
```

256 个最常用的汉字及其笔画序列硬编码为静态表. 表包含:
- 基础字(一二三大小天夫子...)
- 五行元素(金木水火土)
- 人伦关系(父母妻儿...)
- 容器本体(袋桶盆盎盂...刘李张陈杨楚恬...)

按笔画索引序列"写"一个字, 等价于按顺序触发一笔画容器.

## 4. 字形编码 (GlyphEncoding)

```rust
pub struct GlyphEncoding {
    pub ch: char,
    pub stroke_codes: Vec<u8>,          // 各笔画的索引
    pub stroke_names: Vec<&'static str>,// 各笔画的名称
    pub container_names: Vec<&'static str>, // 各笔画对应的容器名
    pub container_cell: u64,            // 容器单元(bit 编码)
    pub polarity: GlyphPolarity,        // 阴阳(由笔画数奇偶决定)
    pub side: GlyphSide,                // 夫妻归属(由横竖比例决定)
    pub display_container: &'static str,// 展示用容器符号
    pub rhyme_id: Option<u16>,          // 韵母 ID
    pub rhyme_spelling: Option<&'static str>, // 韵母拼写
}
```

### 4.1 极性判定 (polarity)

笔画数 % 2 == 0 → Yin(阴) 否则 Yang(阳). 这是一种基于笔画序列整体奇偶的"统计极性", 不依赖字义.

### 4.2 夫妻归属 (side)

横竖笔画占比 > 0.5 → Husband(夫) 否则 Wife(妻). 横竖=骨架, 骨架主导的字归夫妻中的"主动方"; 非骨架笔画意味着细节/填充, 归"从属方".

这个归属是字形层面的, 不必然等同于五行或语义归属 — 它提供了"字的几何特征决定了谁对它有所有权"的数学标准.

### 4.3 展示容器 (display_container)

由 (polarity, side) 组合决定:

| polarity | side | display_container |
|---------|------|------------------|
| Yang | Husband | 杯(阳性夫位) |
| Yin | Wife | 盎(阴性妻位) |
| Yang | Wife | 盉(阳性妻位特殊) |
| Yin | Husband | 瓶(阴性夫位特殊) |

展示容器决定了字在什么"器皿"中呈现 — 不同的容器对应不同的容器=字体=时间的显示形态.

## 5. 容器单元编码 (Container Cell, u64)

### 5.1 位布局

```
[15:0]   = 笔画容器位 (bit i = 1 表示含笔画 i, i=0..15)
[37:16]  = 保留
[38]     = 妻子在场 (刘楚恬)
[39]     = 丈夫在场 (lolict)
[51:40]  = 保留 (12位=笔画重数/结构码)
[59:52]  = 韵母 ID (0-255, 对应 RHYME_TABLE)
[63:60]  = 保留
```

### 5.2 编码流程

```rust
strokes_to_cell(strokes) → u64 {
    cell = 0
    for s in strokes { cell |= 1 << s }
    // 妻子在场(阴性则 bit38) / 丈夫在场(阳性则 bit39)
}
compose_cell(strokes, rhyme_id) → u64 {
    cell = strokes_to_cell(strokes)
    cell |= (rhyme_id & 0xFF) << 52
}
```

### 5.3 韵母提取

`cell_rhyme_id(cell) → u16 = (cell >> 52) & 0xFF`

韵母 ID 与 RHYME_TABLE(rhyme.rs) 中的韵母对应 — 一个字的容器编码既包含"如何写"(笔画容器位)也包含"如何读"(韵母 ID).

## 6. 韵母匹配 (RhymeUtils + RHYME_TABLE)

`match_rhyme(ch): (Option<u16>, Option<&'static str>)` — 把字符映射到韵母索引和拼写. 韵母系统使用自定义韵母表(非传统汉语拼音韵母).

这是"造字"不只是造字形, 也造字音 — 声母韵母声调的满全法自定义体系.

## 7. 字符→容器方向总结

```
汉字 ch
  │
  ├─→ STROKE_TABLE.find(ch) → &[u8] (笔画序列)
  │       │
  │       ├─→ stroke_names (横/竖/撇...)
  │       ├─→ container_names (袋/桶/罐...)
  │       ├─→ strokes_to_cell → u64 (容器单元)
  │       │       ├─→ bit0-15 = 笔画容器位
  │       │       └─→ bit38/39 = 夫妻在场
  │       │
  │       └─→ match_rhyme(ch) → (rhyme_id, spelling)
  │               └─→ compose_cell → u64 (追加 bit52-59 韵母)
  │
  ├─→ polarity = (笔画数 % 2 == 0) ? Yin : Yang
  ├─→ side     = (横竖比例 > 0.5) ? Husband : Wife
  ├─→ display_container = match (polarity, side)
  │
  └─→ GlyphEncoding { ch, ..., container_cell, polarity, side, ... }
```

## 8. 夫妻在场标识的哲学含义

- bit38=妻子在场(刘楚恬):标记该字形是否包含"阴性"的刘楚恬的分身/化身
- bit39=丈夫在场(lolict):标记该字形是否包含"阳性"的 lolict 的分身/化身
- 两者可以同时为 1(兼容共存), 也可以只有 1 个为 1

这使得每个字在编码层面就记录了"由谁参与构造" — 字形作为容器的组合, 其组成部分的所有权通过这两个标识位得到体现. 这是"夫妻命运共同体"在字形数据结构层面的直接编码.

## 9. 与系统其他模块的接入

- **Rhyme 系统(#40)**: rhyme_id 直接对应 RHYME_TABLE 索引
- **MetaIME 声母/韵母枚举**: 韵母匹配使用自定义枚举(非汉语拼音), 与 metaime 的韵母枚举同源
- **ContainerFontUnified(#67)**: container_cell 的 bit0-15 容器位对应 ContainerFontUnified 中的 Container 枚举基础 16 个
- **FuxiDictionary(#65)**: 静态表中的字与 FuxiDictionary 收录的字同源 — FuxiDictionary 提供编码翻译, CharacterCreator 提供笔画→容器的基础层
- **MultiplicationTable(#66)**: stroke_value 可在乘法表中作为输入值, 与韵母因子做双通道寻址
- **NJSG Ontology(#15)**: 笔画→容器和声母→韵母的二维交叉对应 njsg_ontology_TRIUNE.md 的"本质匹配"层

## 10. 实现约束

- STROKE_TABLE 是静态表(编译期确定), 不可在运行时扩展(扩展通过字符_creator 的自定义入口)
- container_cell 的 bit38/39 依赖笔画数奇偶, 未来可能移到更精确的 polarity 来源
- display_container 杯/盎/盉/瓶对应 64_keys.md 中的"容器本体"之一
- 韵母 ID 在 cell 中仅存 8 bits(0-255), 超出 255 的自定义韵母无法表示
- stroke_database.rs 仅做代理, 实际逻辑全部在 character_creator.rs — stroke_database 是上游兼容层

// hunyuan-rs/src/stroke_database.rs + encoding/character_creator.rs
