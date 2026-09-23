# WASM 阶梯制双向公约引擎规范 — 囡囝共生 · 一份源码双目标

## 1. 定位

囡囝共生 WASM 策略的核心命题:**同一份源码,同时编译为 wasm32 (32位地址)和 wasm64 (64位地址)目标**。不是两份代码库,不是条件编译的宏海洋 — 是同一中间产物按不同位宽"阶梯式"生成两份 .wasm 二进制,两份二进制在同一个模块内保持双向接口兼容。

阶梯制 = 同一 IR 表达物按位宽阶梯分别生成。双向公约 = 生成结果满足双向接口 (固定宽度类型 + 统一宏前缀)。

## 2. 位宽阶梯标记 (WasmWidth)

```rust
pub enum WasmWidth {
    W32,  // wasm32:地址和大小使用 i32
    W64,  // wasm64:地址和大小使用 i64 (memory64 proposal)
}
```

W32 使用 `ValType::I32` 作为地址类型,W64 使用 `ValType::I64`。两条阶梯的分叉点仅在**地址与大小类型** — 算术、控制流、比较等操作的 opcode 完全共享。这保证了 90% 的指令编码序列在 W32 与 W64 之间完全相同,只有 load/store 的地址操作数宽度不同。

## 3. 双向共享类型系统

```rust
pub enum ValType { I32, I64, F32, F64 }
pub struct FuncType { pub params: Vec<ValType>, pub results: Vec<ValType> }
```

ValType 的二进制编码是 WASM 标准常量: I32=0x7F, I64=0x7E, F32=0x7D, F64=0x7C。FuncType 编码以 0x60 开头,后跟 params 编码和 results 编码。这部分在 W32 与 W64 之间完全一致 — 函数签名的二进制是相同的。

## 4. 指令集 (双向共享opcode)

指令集是 WASM MVP 子集,覆盖控制流(Nop/Block/Loop/If/Else/Br/BrIf/Return/Call)、参数访问(LocalGet/Set/Tee)、内存(I32Load/I64Load/I32Store/I64Store/MemorySize/Grow)、常量(I32Const/I64Const)、比较(I32Eq/I32Ne/I64Eq/...)、算术(I32Add/Sub/Mul/DivS/RemS + I64同名)以及转换(I32WrapI64/I64ExtendI32S)。

**内存指令的地址操作数差异是两阶梯的唯一分歧点**:`I32Load { align, offset }` 在 W32 地址用 i32,在 W64 地址用 i64。但 opcode 相同(都是 0x28),只是地址操作数的 LEB128 编码长度不同 (i32 最长 5 字节,i64 最长 10 字节)。

转换指令 `I32WrapI64`(i64→i32 截断) 和 `I64ExtendI32S`(i32→i64 符号扩展) 是两阶梯之间的桥梁函数。

## 5. 高层 IR — 双向公约中间表示

```rust
pub enum WasmExpr {
    Const(i32), Add(Box<WasmExpr>, Box<WasmExpr>),
    Sub, Mul,
    Local(u32),
    Call(String, Vec<WasmExpr>),
    Seq(Vec<WasmExpr>),       // 顺序执行
    Return(Box<WasmExpr>),
}
```

WasmExpr 是**与目标位宽无关的中间描述**。它不包含任何直接的类型宽度信息 — 所有宽度细节在`WasmExpr → Instr`的 lowering 阶段由 `WasmWidth` 决定。这意味着同一棵 AST 可以按需生成 W32 或 W64 两条指令流。

## 6. LEB128 编码

变长整数编码是 WASM 二进制的基础:

- 无符号 LEB128: 7 位一组,高位继续位标记。`encode_leb128_u`
- 有符号 LEB128: 符号扩展处理。`encode_leb128_s`

所有段大小、类型索引、函数索引、局部变量计数均用 LEB128 编码。W32 与 W64 在 LEB128 层面完全共享相同的编码函数,差异仅在数值本身的大小。

## 7. WASM 模块构建器 (WasmModule)

```rust
pub struct WasmModule {
    pub width: WasmWidth,
    types: Vec<FuncType>,
    functions: Vec<(usize, String, FuncBody)>,
    exports: Vec<Export>,
    memories: Vec<MemLimit>,
    data_segments: Vec<DataSegment>,
}
```

核心方法 `add_function(name, type_idx, locals, instrs)` 注册函数定义,**指令序列在注册时即已按目标位宽完成编码**,不再在 lowering 阶段反复转换。`export_function` / `add_memory` / `add_data` 构建导出、内存与数据段。

`finish()` 生成最终二进制:

```
[0x00 0x61 0x73 0x6D]   // magic: \0asm
[0x01 0x00 0x00 0x00]   // version: 1
[type_section_id=1] + size + payload
[function_section_id=3] + size + payload
[memory_section_id=5] + size + payload
[export_section_id=7] + size + payload
[code_section_id=10] + size + payload
[data_section_id=11] + size + payload + data_count
```

段顺序严格遵循 WASM 规范 (type → import → function → table → memory → global → export → start → element → code → data),缺失的段直接跳过。

## 8. 与系统其他模块的接入

- **SECURITY_COVERT_CHANNELS** 的 WASM 沙箱隔离使用本模块生成完整 .wasm,不依赖外部 wasm-tools/wabt
- **CROSS_ISA_EXECUTION_LAYER** 的跨ISA对齐表中,WASM 平台的编码由本模块生成
- **FIVE_DIMENSIONAL_SEMANTICS** 的 ρ 值传输以 WasmModule.add_data 数据段嵌入
- **BOOTSTRAP_SELF_COMPILER** 的 L0 Ouroboros 证明可被编译为 wasm 目标,在浏览器内验证
- **MCCP_ISA_VM** 的字节码可通过本模块翻译到 WASM,实现浏览器内 NJSG 运行时

## 9. 实现约束

- 整个模块纯 Rust 实现,不依赖任何外部 WASM 工具链
- magic `\0asm` 和 version 1 硬编码,ASN.1/DER 格式不使用
- 所有段大小以 LEB128 编码 — 无固定宽度字段浪费
- Wasm32 与 W64 共享同一套测试用例,差异仅在 width 参数
- no_std 模式下 Vec 被替换为固定容量数组 (heapless::Vec)

// hunyuan-rs/src/encoding/wasm_ladder.rs
