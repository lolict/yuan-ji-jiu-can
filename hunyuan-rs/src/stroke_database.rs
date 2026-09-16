// ═══════════════════════════════════════════════════════════════════════════
// stroke_database.rs — 笔画数据库接入层 (代理至 character_creator)
// ═══════════════════════════════════════════════════════════════════════════
//
// 上游模块通过本文件访问笔画容器编码。
// 实际逻辑在 crate::character_creator 中实现，本模块提供兼容代理。

pub use crate::character_creator::{ContainerEnc, StrokeDatabase};
