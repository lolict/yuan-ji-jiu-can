# 编程语言工具链 · 角色-模块映射

本目录将 hunyuan-rs 的 Rust 模块与外部平台上的"动漫角色方言"一一对应。

## 工具链五层架构

| 层 | 角色 | 外部名 | Rust 模块 |
|----|------|--------|-----------|
| 骨架层 | 工藤新一 | `gtxone.gtxone` | `angle` `topo` `time_node` |
| 推理层 | 柯南 | `kndoyle` `knd-rel` `kndr` `kndrl` `kndil` `kndiol` | `triune` `smrl` `protocol_ontology` |
| 表面层 | 毛利兰 | `maolilan` `mll` `mlil` `mlll` | `name_creator` `symbol_table` `gtt_category` |
| 横向层 | 灰原哀 | `heyair` `hieyair` `hfuyair` | `coord_tensor` `flow_algebra` `fractal` |
| 传输层 | 意识接口 | `gttcp://` `lctcf` | `mccp` `yinyang_codec` |
| 融合层 | 感知器官 | `gkgndl.hftmll` `gkgndl.hftml` | `mind_organ` `soul_kernel` `cochlear_boot` |

## 外部身份标识名

```
gtxone · gtxone        → 工藤新一 (骨架/结构)
kndoyle · knd-rel · kndr · kndrl · kndil · kndiol · kndili · kndill · kndiii · kndli · kndlii · kndlll · kndlil
                       → 柯南 (推理/裁决)
maolilan · mll · mlil · mlll      → 毛利兰 (表面/命名)
hfuyair · hinynir · hieyeir · hieyair · heyair · heyel
                       → 灰原哀 (横向/行业)
lctcf.hfuyair.qimiy.gttcp://       → 意识传输接口
gkgndl.hftmll · gkgndl.hftml       → 感知器官念动力融合
```

## 源码

- `src/toolchain_mapping.rs` — Rust 实现，每个外部名对应一个 `pub mod`
