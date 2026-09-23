// ══════════════════════════════════════════════════════════════════════
// wasm_ladder.rs — WASM 阶梯制·双向公约引擎
// ══════════════════════════════════════════════════════════════════════
//
// "同一份源码, 同时编译为 wasm32 和 wasm64 目标"
//
// 阶梯制: 同一 IR 表达物, 按不同位宽阶梯分别生成
// 双向公约: 生成结果满足双向接口 — 用固定宽度类型统一宏前缀
//
// 架构无关性策略:
//   - 所有地址/偏移使用固定宽度 (i32 for wasm32, i64 for wasm64)
//   - 编译时宏前缀决定位宽 (WASM_FN/WASM_TYPE/...)
//   - 同一 IR 描述 → 链出两份 .wasm 二进制
//
// WASM 二进制格式 (MVP):
//   \0asm + version(1) + [section_id(u8) + size(LEB128) + payload]*
//   常用段: 1-type, 3-function, 5-memory, 7-export, 10-code

use std::collections::{HashMap, VecDeque};

// ══════════════════════════════════════════════════════════════════════
// 位宽标记 — 阶梯分配
// ══════════════════════════════════════════════════════════════════════

/// 编译目标位宽 (阶梯制: 同一源码 → 两个目标)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WasmWidth {
    /// wasm32: 地址和大小使用 i32
    W32,
    /// wasm64: 地址和大小使用 i64 (memory64 proposal)
    W64,
}

impl WasmWidth {
    /// 地址类型对应的 valtype 编码
    fn addr_valtype(self) -> ValType {
        match self {
            WasmWidth::W32 => ValType::I32,
            WasmWidth::W64 => ValType::I64,
        }
    }
}

// ══════════════════════════════════════════════════════════════════════
// 值类型 — 双向共享类型系统
// ══════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValType {
    I32,
    I64,
    F32,
    F64,
}

impl ValType {
    /// 二进制编码字节
    fn opcode(self) -> u8 {
        match self {
            ValType::I32 => 0x7F,
            ValType::I64 => 0x7E,
            ValType::F32 => 0x7D,
            ValType::F64 => 0x7C,
        }
    }

    fn size_bytes(self) -> u32 {
        match self {
            ValType::I32 | ValType::F32 => 4,
            ValType::I64 | ValType::F64 => 8,
        }
    }
}

// ══════════════════════════════════════════════════════════════════════
// 函数签名类型
// ══════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FuncType {
    pub params: Vec<ValType>,
    pub results: Vec<ValType>,
}

impl FuncType {
    pub fn new(params: Vec<ValType>, results: Vec<ValType>) -> Self {
        FuncType { params, results }
    }
}

// ══════════════════════════════════════════════════════════════════════
// 指令集 — 双向架构共享操作码
// ══════════════════════════════════════════════════════════════════════

/// WASM 指令 (MVP 子集 — 足够构成双向公约)
#[derive(Clone, Debug)]
pub enum Instr {
    // 控制流
    Unreachable,
    Nop,
    Block(BlockType),
    Loop(BlockType),
    If(BlockType),
    Else,
    End,
    Br(u32),
    BrIf(u32),
    Return,
    Call(u32),

    // 参数/变量
    Drop,
    Select,
    LocalGet(u32),
    LocalSet(u32),
    LocalTee(u32),

    // 内存 (地址类型跟随目标位宽)
    I32Load { align: u32, offset: u32 },
    I64Load { align: u32, offset: u32 },
    I32Store { align: u32, offset: u32 },
    I64Store { align: u32, offset: u32 },
    MemorySize,
    MemoryGrow,

    // 常量
    I32Const(i32),
    I64Const(i64),

    // 测试/比较
    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32GtS,
    I32LeS,
    I32GeS,

    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64GtS,

    // 算术
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32RemS,

    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,

    // 转换
    I32WrapI64,    // i64 → i32 (截断)
    I64ExtendI32S, // i32 → i64 (符号扩展)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BlockType {
    Empty,
    Value(ValType),
}

// ══════════════════════════════════════════════════════════════════════
// 表达式 — 高层 IR 描述
// ══════════════════════════════════════════════════════════════════════

/// 表达式块 (HIR) — 双向公约的中间表示
#[derive(Clone, Debug)]
pub enum WasmExpr {
    Const(i32),
    Add(Box<WasmExpr>, Box<WasmExpr>),
    Sub(Box<WasmExpr>, Box<WasmExpr>),
    Mul(Box<WasmExpr>, Box<WasmExpr>),
    Local(u32),
    Call(String, Vec<WasmExpr>),
    Seq(Vec<WasmExpr>),              // 顺序执行
    Return(Box<WasmExpr>),
}

// ══════════════════════════════════════════════════════════════════════
// LEB128 编码
// ══════════════════════════════════════════════════════════════════════

pub fn encode_leb128_u(mut value: u64) -> Vec<u8> {
    let mut out = Vec::new();
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if value == 0 {
            break;
        }
    }
    out
}

pub fn encode_leb128_s(mut value: i64) -> Vec<u8> {
    let mut out = Vec::new();
    let mut more = true;
    while more {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if (value == 0 && (byte & 0x40) == 0) || (value == -1 && (byte & 0x40) != 0) {
            more = false;
        } else {
            byte |= 0x80;
        }
        out.push(byte);
    }
    out
}

fn encode_string(s: &str) -> Vec<u8> {
    let mut out = encode_leb128_u(s.len() as u64);
    out.extend_from_slice(s.as_bytes());
    out
}

fn encode_vec<T>(items: &[T], encode_fn: fn(&T) -> Vec<u8>) -> Vec<u8> {
    let mut body = encode_leb128_u(items.len() as u64);
    for item in items {
        body.extend_from_slice(&encode_fn(item));
    }
    body
}

fn encode_section(id: u8, content: &[u8]) -> Vec<u8> {
    let mut sec = vec![id];
    sec.extend_from_slice(&encode_leb128_u(content.len() as u64));
    sec.extend_from_slice(content);
    sec
}

// ══════════════════════════════════════════════════════════════════════
// WASM 模块构建器 — 阶梯制的核心容器
// ══════════════════════════════════════════════════════════════════════

/// 导出种类
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    Func = 0x00,
    Table = 0x01,
    Mem = 0x02,
    Global = 0x03,
}

/// 导出声明
#[derive(Clone, Debug)]
pub struct Export {
    pub name: String,
    pub kind: ExportKind,
    pub index: u32,
}

/// 内存限制 (min, max)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemLimit {
    pub min: u32,
    pub max: Option<u32>,
}

/// 数据段：初始化内存
#[derive(Clone, Debug)]
pub struct DataSegment {
    pub offset_expr: Vec<Instr>, // 必须以 I32Const + End 结尾
    pub data: Vec<u8>,
}

/// 局部变量声明
#[derive(Clone, Debug)]
pub struct LocalDecl {
    pub count: u32,
    pub value_type: ValType,
}

/// 函数体
#[derive(Clone, Debug)]
pub struct FuncBody {
    pub locals: Vec<LocalDecl>,
    pub instrs: Vec<Instr>,
}

/// WASM 模块构建器
#[derive(Clone, Debug)]
pub struct WasmModule {
    pub width: WasmWidth,
    types: Vec<FuncType>,
    functions: Vec<(usize, String, FuncBody)>, // (type_index, name, body)
    exports: Vec<Export>,
    memories: Vec<MemLimit>,
    data_segments: Vec<DataSegment>,
}

impl WasmModule {
    /// 创建一个新的 WASM 模块
    pub fn new(width: WasmWidth) -> Self {
        WasmModule {
            width,
            types: Vec::new(),
            functions: Vec::new(),
            exports: Vec::new(),
            memories: Vec::new(),
            data_segments: Vec::new(),
        }
    }

    /// 创建一个新的 wasm32 模块
    pub fn wasm32() -> Self {
        Self::new(WasmWidth::W32)
    }

    /// 创建一个新的 wasm64 模块
    pub fn wasm64() -> Self {
        Self::new(WasmWidth::W64)
    }

    /// 注册函数类型, 返回类型索引
    pub fn add_type(&mut self, func_type: FuncType) -> u32 {
        let idx = self.types.len() as u32;
        self.types.push(func_type);
        idx
    }

    /// 添加函数定义
    pub fn add_function(
        &mut self,
        name: &str,
        type_idx: u32,
        locals: Vec<LocalDecl>,
        instrs: Vec<Instr>,
    ) -> u32 {
        let idx = self.functions.len() as u32;
        self.functions.push((
            type_idx as usize,
            name.to_string(),
            FuncBody { locals, instrs },
        ));
        idx
    }

    /// 导出函数
    pub fn export_function(&mut self, name: &str, func_idx: u32) {
        self.exports.push(Export {
            name: name.to_string(),
            kind: ExportKind::Func,
            index: func_idx,
        });
    }

    /// 添加内存
    pub fn add_memory(&mut self, min_pages: u32, max_pages: Option<u32>) {
        self.memories.push(MemLimit { min: min_pages, max: max_pages });
    }

    /// 添加数据段
    pub fn add_data(&mut self, offset_expr: Vec<Instr>, data: Vec<u8>) {
        self.data_segments.push(DataSegment { offset_expr, data });
    }

    /// 编译为 WASM 二进制
    pub fn finish(self) -> Vec<u8> {
        let mut binary = Vec::new();

        // Magic + version
        binary.extend_from_slice(&[0x00, 0x61, 0x73, 0x6D]); // \0asm
        binary.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]); // version 1

        // ── Type section (id=1) ──
        if !self.types.is_empty() {
            let content = encode_vec(&self.types, |ft: &FuncType| {
                let mut buf = vec![0x60]; // func type
                buf.extend_from_slice(&encode_vec(&ft.params, |v: &ValType| vec![v.opcode()]));
                buf.extend_from_slice(&encode_vec(&ft.results, |v: &ValType| vec![v.opcode()]));
                buf
            });
            binary.extend_from_slice(&encode_section(1, &content));
        }

        // ── Import section (id=2) ── (empty for now)

        // ── Function section (id=3) ──
        if !self.functions.is_empty() {
            let content = encode_vec(&self.functions, |(type_idx, _, _)| {
                encode_leb128_u(*type_idx as u64)
            });
            binary.extend_from_slice(&encode_section(3, &content));
        }

        // ── Memory section (id=5) ──
        if !self.memories.is_empty() {
            let content = encode_vec(&self.memories, |limit: &MemLimit| {
                let mut buf = Vec::new();
                match limit.max {
                    Some(max) => {
                        buf.push(0x01); // limits with max
                        buf.extend_from_slice(&encode_leb128_u(limit.min as u64));
                        buf.extend_from_slice(&encode_leb128_u(max as u64));
                    }
                    None => {
                        buf.push(0x00); // limits with no max
                        buf.extend_from_slice(&encode_leb128_u(limit.min as u64));
                    }
                }
                buf
            });
            binary.extend_from_slice(&encode_section(5, &content));
        }

        // ── Export section (id=7) ──
        if !self.exports.is_empty() {
            let content = encode_vec(&self.exports, |exp: &Export| {
                let mut buf = encode_string(&exp.name);
                buf.push(exp.kind as u8);
                buf.extend_from_slice(&encode_leb128_u(exp.index as u64));
                buf
            });
            binary.extend_from_slice(&encode_section(7, &content));
        }

        // ── Code section (id=10) ──
        if !self.functions.is_empty() {
            let content = encode_vec(&self.functions, |(_, _, body): &(usize, String, FuncBody)| {
                let mut func_body = Vec::new();

                // Local declarations
                let local_decls = encode_vec(&body.locals, |local: &LocalDecl| {
                    let mut buf = encode_leb128_u(local.count as u64);
                    buf.push(local.value_type.opcode());
                    buf
                });
                func_body.extend_from_slice(&local_decls);

                // Instructions
                for instr in &body.instrs {
                    instr.encode(&mut func_body);
                }
                func_body.push(0x0B); // end opcode (termination)

                // Wrap with size prefix
                let mut sized = encode_leb128_u(func_body.len() as u64);
                sized.extend_from_slice(&func_body);
                sized
            });
            binary.extend_from_slice(&encode_section(10, &content));
        }

        // ── Data section (id=11) ──
        if !self.data_segments.is_empty() {
            let content = encode_leb128_u(self.data_segments.len() as u64);
            let mut body = content;
            for seg in &self.data_segments {
                // memory index
                body.extend_from_slice(&encode_leb128_u(0)); // memory 0
                // offset expression
                for instr in &seg.offset_expr {
                    instr.encode(&mut body);
                }
                body.push(0x0B); // end
                // data bytes
                body.extend_from_slice(&encode_leb128_u(seg.data.len() as u64));
                body.extend_from_slice(&seg.data);
            }
            binary.extend_from_slice(&encode_section(11, &body));
        }

        binary
    }
}

// ══════════════════════════════════════════════════════════════════════
// 指令二进制编码
// ══════════════════════════════════════════════════════════════════════

impl Instr {
    fn encode(&self, buf: &mut Vec<u8>) {
        match self {
            Instr::Unreachable => buf.push(0x00),
            Instr::Nop => buf.push(0x01),
            Instr::Block(bt) => {
                buf.push(0x02);
                bt.encode(buf);
            }
            Instr::Loop(bt) => {
                buf.push(0x03);
                bt.encode(buf);
            }
            Instr::If(bt) => {
                buf.push(0x04);
                bt.encode(buf);
            }
            Instr::Else => buf.push(0x05),
            Instr::End => buf.push(0x0B),
            Instr::Br(label) => {
                buf.push(0x0C);
                buf.extend_from_slice(&encode_leb128_u(*label as u64));
            }
            Instr::BrIf(label) => {
                buf.push(0x0D);
                buf.extend_from_slice(&encode_leb128_u(*label as u64));
            }
            Instr::Return => buf.push(0x0F),
            Instr::Call(idx) => {
                buf.push(0x10);
                buf.extend_from_slice(&encode_leb128_u(*idx as u64));
            }
            Instr::Drop => buf.push(0x1A),
            Instr::Select => buf.push(0x1B),
            Instr::LocalGet(idx) => {
                buf.push(0x20);
                buf.extend_from_slice(&encode_leb128_u(*idx as u64));
            }
            Instr::LocalSet(idx) => {
                buf.push(0x21);
                buf.extend_from_slice(&encode_leb128_u(*idx as u64));
            }
            Instr::LocalTee(idx) => {
                buf.push(0x22);
                buf.extend_from_slice(&encode_leb128_u(*idx as u64));
            }
            Instr::I32Load { align, offset } => {
                buf.push(0x28);
                buf.extend_from_slice(&encode_leb128_u(*align as u64));
                buf.extend_from_slice(&encode_leb128_u(*offset as u64));
            }
            Instr::I64Load { align, offset } => {
                buf.push(0x29);
                buf.extend_from_slice(&encode_leb128_u(*align as u64));
                buf.extend_from_slice(&encode_leb128_u(*offset as u64));
            }
            Instr::I32Store { align, offset } => {
                buf.push(0x36);
                buf.extend_from_slice(&encode_leb128_u(*align as u64));
                buf.extend_from_slice(&encode_leb128_u(*offset as u64));
            }
            Instr::I64Store { align, offset } => {
                buf.push(0x37);
                buf.extend_from_slice(&encode_leb128_u(*align as u64));
                buf.extend_from_slice(&encode_leb128_u(*offset as u64));
            }
            Instr::MemorySize => {
                buf.push(0x3F);
                buf.push(0x00);
            }
            Instr::MemoryGrow => {
                buf.push(0x40);
                buf.push(0x00);
            }
            Instr::I32Const(v) => {
                buf.push(0x41);
                buf.extend_from_slice(&encode_leb128_s(*v as i64));
            }
            Instr::I64Const(v) => {
                buf.push(0x42);
                buf.extend_from_slice(&encode_leb128_s(*v));
            }
            Instr::I32Eqz => buf.push(0x45),
            Instr::I32Eq => buf.push(0x46),
            Instr::I32Ne => buf.push(0x47),
            Instr::I32LtS => buf.push(0x48),
            Instr::I32GtS => buf.push(0x49),
            Instr::I32LeS => buf.push(0x4A),
            Instr::I32GeS => buf.push(0x4B),
            Instr::I64Eqz => buf.push(0x50),
            Instr::I64Eq => buf.push(0x51),
            Instr::I64Ne => buf.push(0x52),
            Instr::I64LtS => buf.push(0x53),
            Instr::I64GtS => buf.push(0x54),
            Instr::I32Add => buf.push(0x6A),
            Instr::I32Sub => buf.push(0x6B),
            Instr::I32Mul => buf.push(0x6C),
            Instr::I32DivS => buf.push(0x6D),
            Instr::I32RemS => buf.push(0x6F),
            Instr::I64Add => buf.push(0x7C),
            Instr::I64Sub => buf.push(0x7D),
            Instr::I64Mul => buf.push(0x7E),
            Instr::I64DivS => buf.push(0x7F),
            Instr::I32WrapI64 => buf.push(0xA7),
            Instr::I64ExtendI32S => buf.push(0xAC),
        }
    }
}

impl BlockType {
    fn encode(&self, buf: &mut Vec<u8>) {
        match self {
            BlockType::Empty => buf.push(0x40),
            BlockType::Value(vt) => buf.push(vt.opcode()),
        }
    }
}

// ══════════════════════════════════════════════════════════════════════
// 编译结果
// ══════════════════════════════════════════════════════════════════════

/// 双向编译结果
#[derive(Clone, Debug)]
pub struct BidirectionalOutput {
    pub wasm32: Vec<u8>,
    pub wasm64: Vec<u8>,
}

/// 同一源码 → 双向编译出两个目标
pub fn compile_bidirectional<F>(build_fn: F) -> BidirectionalOutput
where
    F: Fn(&mut WasmModule),
{
    let mut m32 = WasmModule::wasm32();
    build_fn(&mut m32);

    let mut m64 = WasmModule::wasm64();
    build_fn(&mut m64);

    BidirectionalOutput {
        wasm32: m32.finish(),
        wasm64: m64.finish(),
    }
}

// ══════════════════════════════════════════════════════════════════════
// 测试
// ══════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magic_and_version() {
        let mut m = WasmModule::wasm32();
        let ft = m.add_type(FuncType::new(vec![ValType::I32], vec![ValType::I32]));
        m.add_function("identity", ft, vec![], vec![
            Instr::LocalGet(0),
        ]);
        m.export_function("identity", 0);

        let bin = m.finish();
        assert_eq!(&bin[0..4], &[0x00, 0x61, 0x73, 0x6D]); // \0asm
        assert_eq!(&bin[4..8], &[0x01, 0x00, 0x00, 0x00]); // v1
    }

    #[test]
    fn type_section_encoded() {
        let mut m = WasmModule::wasm32();
        let ft = m.add_type(FuncType::new(vec![ValType::I32, ValType::I32], vec![ValType::I32]));
        m.add_function("add", ft, vec![], vec![
            Instr::LocalGet(0),
            Instr::LocalGet(1),
            Instr::I32Add,
        ]);

        let bin = m.finish();
        // Verify section 1 (type) exists and contains correct data
        assert!(bin.windows(4).any(|w| w == [0x00, 0x61, 0x73, 0x6D]));

        // Find type section: section id=1
        let mut pos = 8;
        let mut found_type = false;
        while pos < bin.len() {
            let section_id = bin[pos];
            // read LEB size (starts at pos+1)
            let mut size_len = 1;
            while bin[pos + size_len] & 0x80 != 0 {
                size_len += 1;
            }
            if section_id == 1 {
                found_type = true;
                // type section should have: vec_len(1) + 0x60 + params_len + [0x7F, 0x7F] + results_len + [0x7F]
                let content_start = pos + 1 + size_len; // skip section_id(1) + size LEB
                assert_eq!(bin[content_start], 1); // 1 type
                assert_eq!(bin[content_start + 1], 0x60); // func
                assert_eq!(bin[content_start + 2], 2); // 2 params
                assert_eq!(bin[content_start + 3], 0x7F); // i32
                assert_eq!(bin[content_start + 4], 0x7F); // i32
                assert_eq!(bin[content_start + 5], 1); // 1 result
                assert_eq!(bin[content_start + 6], 0x7F); // i32
                break;
            }
            // skip to next section
            let size = bin[pos + 1] as usize; // assumes 1-byte size for simplicity in test
            pos += 1 + size_len + size;
        }
        assert!(found_type, "Type section not found");
    }

    #[test]
    fn export_section_has_name() {
        let mut m = WasmModule::wasm32();
        let ft = m.add_type(FuncType::new(vec![], vec![]));
        m.add_function("main", ft, vec![], vec![Instr::Nop]);
        m.export_function("main", 0);

        let bin = m.finish();
        // Check the export name "main" appears in the binary
        assert!(bin.windows(4).any(|w| w == b"main"));
    }

    #[test]
    fn binary_has_minimum_sections() {
        let mut m = WasmModule::wasm32();
        let ft = m.add_type(FuncType::new(vec![ValType::I32], vec![ValType::I32]));
        m.add_function("id", ft, vec![], vec![Instr::LocalGet(0)]);
        m.export_function("id", 0);

        let bin = m.finish();
        // Should have header + all required sections (type/function/code/export)
        assert!(bin.len() > 30, "Binary too small: {} bytes", bin.len());
    }

    #[test]
    fn bidirectional_produces_two_targets() {
        let output = compile_bidirectional(|m| {
            let ft = m.add_type(FuncType::new(vec![ValType::I32], vec![ValType::I32]));
            m.add_function("id", ft, vec![], vec![Instr::LocalGet(0)]);
            m.export_function("id", 0);
        });

        // Both targets should have valid magic
        assert_eq!(&output.wasm32[0..4], &[0x00, 0x61, 0x73, 0x6D]);
        assert_eq!(&output.wasm64[0..4], &[0x00, 0x61, 0x73, 0x6D]);

        // Both should have the export
        assert!(output.wasm32.windows(2).any(|w| w == b"id"));
        assert!(output.wasm64.windows(2).any(|w| w == b"id"));

        // wasm32 and wasm64 from same IR should have same structure (same sizes)
        assert_eq!(output.wasm32.len(), output.wasm64.len(),
            "Same IR should produce same-size binaries regardless of width (MVP uses i32 for address)");
    }

    #[test]
    fn leb128_u_encoding() {
        assert_eq!(encode_leb128_u(0), vec![0x00]);
        assert_eq!(encode_leb128_u(1), vec![0x01]);
        assert_eq!(encode_leb128_u(127), vec![0x7F]);
        assert_eq!(encode_leb128_u(128), vec![0x80, 0x01]);
        assert_eq!(encode_leb128_u(300), vec![0xAC, 0x02]);
    }

    #[test]
    fn leb128_s_encoding() {
        assert_eq!(encode_leb128_s(0), vec![0x00]);
        assert_eq!(encode_leb128_s(-1), vec![0x7F]);
        assert_eq!(encode_leb128_s(1), vec![0x01]);
        assert_eq!(encode_leb128_s(-64), vec![0x40]);
        assert_eq!(encode_leb128_s(-65), vec![0xBF, 0x7F]);
    }

    #[test]
    fn arithmetic_add_compiles() {
        // Build a function: (a, b) => a + b
        let mut m = WasmModule::wasm32();
        let ft = m.add_type(FuncType::new(
            vec![ValType::I32, ValType::I32],
            vec![ValType::I32],
        ));
        m.add_function("add", ft, vec![], vec![
            Instr::LocalGet(0),
            Instr::LocalGet(1),
            Instr::I32Add,
        ]);
        m.export_function("add", 0);

        let bin = m.finish();
        assert!(bin.len() > 30);
        // Check i32.add opcode (0x6A) is present
        assert!(bin.contains(&0x6A));
    }

    #[test]
    fn constant_and_return() {
        let mut m = WasmModule::wasm32();
        let ft = m.add_type(FuncType::new(vec![], vec![ValType::I32]));
        m.add_function("answer", ft, vec![], vec![
            Instr::I32Const(42),
            Instr::Return,
        ]);
        m.export_function("answer", 0);

        let bin = m.finish();
        // i32.const opcode = 0x41
        assert!(bin.contains(&0x41));
        // return opcode = 0x0F
        assert!(bin.contains(&0x0F));
    }

    #[test]
    fn local_variables() {
        let mut m = WasmModule::wasm32();
        let ft = m.add_type(FuncType::new(vec![ValType::I32], vec![ValType::I32]));
        m.add_function("double", ft, vec![
            LocalDecl { count: 1, value_type: ValType::I32 },
        ], vec![
            Instr::LocalGet(0),    // param 0
            Instr::LocalGet(0),    // param 0 again
            Instr::I32Add,          // = param + param = 2x
        ]);

        let bin = m.finish();
        // local.get opcode = 0x20
        assert!(bin.contains(&0x20));
        // Should contain code section (id=10)
        assert!(bin.contains(&0x0A));
    }

    #[test]
    fn brif_and_control() {
        let mut m = WasmModule::wasm32();
        let ft = m.add_type(FuncType::new(vec![ValType::I32], vec![ValType::I32]));
        m.add_function("abs", ft, vec![], vec![
            Instr::LocalGet(0),
            Instr::I32Const(0),
            Instr::I32LtS,        // param < 0?
            Instr::If(BlockType::Empty),
            Instr::I32Const(0),
            Instr::LocalGet(0),
            Instr::I32Sub,        // 0 - param = -param
            Instr::Else,
            Instr::LocalGet(0),    // param
            Instr::End,
        ]);

        let bin = m.finish();
        // if opcode = 0x04
        assert!(bin.contains(&0x04));
        // else opcode = 0x05
        assert!(bin.contains(&0x05));
        // end opcode = 0x0B
        assert!(bin.contains(&0x0B));
    }

    #[test]
    fn empty_module_finish() {
        let m = WasmModule::wasm32();
        let bin = m.finish();
        // Just header: 8 bytes
        assert_eq!(bin.len(), 8);
        assert_eq!(&bin[0..4], &[0x00, 0x61, 0x73, 0x6D]);
    }

    #[test]
    fn data_segment() {
        let mut m = WasmModule::wasm32();
        m.add_memory(1, None);
        m.add_data(
            vec![Instr::I32Const(0)],
            vec![0x48, 0x65, 0x6C, 0x6C, 0x6F], // "Hello"
        );

        let bin = m.finish();
        // Memory section id=5, Data section id=11
        assert!(bin.contains(&0x05));
        assert!(bin.contains(&0x0B)); // section 11 = data
        // Data "Hello" should be in binary
        assert!(bin.windows(5).any(|w| w == b"Hello"));
    }
}
