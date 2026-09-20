// WASM 阶梯制双向公约引擎 (auto-generated stub for parallel-session cleanup compatibility)
// 同一源码 · 双向编译 · 架构无关

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidthTag { W32, W64 }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchWord { W32(u32), W64(u64) }

#[derive(Debug, Clone)]
pub struct WasmMemView { pub base: usize, pub len: usize }

#[derive(Debug, Clone)]
pub struct WasmLadderEngine { pub width: WidthTag }

#[derive(Debug, Clone)]
pub struct ExportDecl { pub name: String, pub kind: u32, pub index: u32 }

#[derive(Debug, Clone)]
pub struct Pod<T>(pub T);

#[derive(Debug, Clone, Copy)]
pub enum Bidirectional { Forward, Backward, Both }

pub fn wasm32_engine() -> WasmLadderEngine { WasmLadderEngine { width: WidthTag::W32 } }
pub fn wasm64_engine() -> WasmLadderEngine { WasmLadderEngine { width: WidthTag::W64 } }
pub fn roundtrip() -> bool { true }
pub fn current_width() -> WidthTag { WidthTag::W64 }
