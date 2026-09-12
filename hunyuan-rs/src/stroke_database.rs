// ═══════════════════════════════════════════════════════════════════════════
// stroke_database.rs — 笔画数据库占位 (Stub)
// ═══════════════════════════════════════════════════════════════════════════
//
// 上游模块引用了 StrokeDatabase::encode_char，但完整实现尚未迁入。
// 此处提供最小接口以保持编译通过；功能实现时替换为真实逻辑。

/// 容器编码占位类型 — 兼容原 ContainerEnc(pub u64) 接口
#[derive(Clone, Copy)]
pub struct ContainerEnc(pub u64);

impl ContainerEnc {
    pub fn consonant_idx(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }
}

impl std::fmt::Debug for ContainerEnc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ContainerEnc(0x{:016X})", self.0)
    }
}

/// 笔画数据库占位类型
pub struct StrokeDatabase;

impl StrokeDatabase {
    /// 字符 → 容器编码 (占位实现，始终返回 None)
    pub fn encode_char(_ch: char) -> Option<ContainerEnc> {
        None
    }
}
