// ═══════════════════════════════════════════════════════════════════════════
// stroke_database.rs — 笔画数据库占位 (Stub)
// ═══════════════════════════════════════════════════════════════════════════
//
// 上游模块引用了 StrokeDatabase::encode_char，但完整实现尚未迁入。
// 此处提供最小接口以保持编译通过；功能实现时替换为真实逻辑。

use crate::comma_container::ContainerEnc;

/// 笔画数据库占位类型
pub struct StrokeDatabase;

impl StrokeDatabase {
    /// 字符 → 容器编码 (占位实现，始终返回 None)
    pub fn encode_char(_ch: char) -> Option<ContainerEnc> {
        None
    }
}
