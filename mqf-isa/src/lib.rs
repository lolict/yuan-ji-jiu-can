// ══════════════════════════════════════════════════════════════════════
// mqf-isa — 跨平台指令集语义统一层
//
// 容器=字体=时间
//   · 62个语义ID跨平台对齐（RV32I/X86-64/ARM64/MIPS32/MTK）
//   · 态射字体：轮廓坐标 = 机器码字节序列
//   · ISA 编码/解码 CLI
// ══════════════════════════════════════════════════════════════════════

pub mod cross_isa;
pub mod isa;
pub mod isa16;
pub mod morphism;
pub use morphism::{
    MorphSem, TargetIsa,
    bytes_to_coords, coords_to_bytes, bytes_hex,
    generate_font, extract_glyph_coords,
};
