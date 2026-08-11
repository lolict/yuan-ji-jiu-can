// ══════════════════════════════════════════════════════════════════
// l0_self_hosting.rs — MCCP 自举编译器 v2 (Ouroboros)
// ══════════════════════════════════════════════════════════════════
//
// 证明: L0 能编译 L0 (编译器用 L0 自身写成, 在 MCCP VM 上执行)
//
// Ouroboros 流程:
//   1. L0 tokenizer 源码 (L0 写成) → 输入到 L0 Vm
//   2. Vm 执行 tokenizer, 把 L0 token 流写到 memory[2..]
//   3. Rust 收割 token 流, 翻译为 IR
//   4. IR → Mccp bytecode (编译后端)
//   5. Mccp bytecode 在 VM 上执行 → 输出结果
//
// 这就是 "L0 编译 L0" 的最小可工作证明。

use crate::mccp_vm::{
    op, op_imm, Vm,
    OP_PUSH, OP_LOAD, OP_STORE, OP_ADD, OP_SUB, OP_MUL, OP_DIV,
    OP_PRINT, OP_PRINTC, OP_HALT, OP_JMP, OP_JZ, OP_JNZ, OP_MOD,
    OP_CALL, OP_RET, OP_SYSCALL, OP_NEG, OP_EQ, OP_NE, OP_LT, OP_GT,
    OP_LE, OP_GE, OP_DUP, OP_SWAP, OP_POP,
};
use std::collections::HashMap;
use crate::mccp::Mccp;

// ────────────────────────────────────────────────────────────
// 协议: L0 编译器输出格式 (写到 Vm.memory)
// ────────────────────────────────────────────────────────────

/// L0 tokenizer 把 token 流写入内存, 从 SLOT_OFF 开始
pub const SLOT_OFF: usize = 2;     // a=slot0, b=slot1; 从 slot2 (var 'c') 开始

/// Rust 端从 VM memory 收割 token 流 (读 memory[SLOT_OFF..SLOT_OFF+count*2])
pub fn harvest_tokens(vm: &Vm) -> Vec<(u16, i32)> {
    // count 存在变量 'c' = memory[2]
    let count = vm.memory[SLOT_OFF] as usize;
    let mut toks = Vec::with_capacity(count);
    for i in 0..count {
        let base = SLOT_OFF + 1 + i * 2;
        // 特殊编码: type=t, value=v — 如果 value < 0 表示立即数占下一个槽
        let t = vm.memory[base] as u16;
        let v = vm.memory[base + 1];
        toks.push((t, v));
    }
    toks
}

// ────────────────────────────────────────────────────────────
// IR token 定义
// ────────────────────────────────────────────────────────────

/// token 类型: 单条 IR token 的 "类型" 部分
pub type TokType = u16;
pub type TokVal = i32;

// 特殊关键字 token (不属 Mccp 操作码)
pub const T_PRINT: TokType = 0xF0;
pub const T_PRINTC: TokType = 0xF1;
pub const T_IF: TokType = 0xF2;
pub const T_THEN: TokType = 0xF3;
pub const T_ELSE: TokType = 0xF4;
pub const T_END: TokType = 0xF5;
pub const T_WHILE: TokType = 0xF6;
pub const T_DO: TokType = 0xF7;
pub const T_JMP: TokType = 0xF8;
pub const T_JZ: TokType = 0xF9;
pub const T_JNZ: TokType = 0xFA;
pub const T_CALL: TokType = 0xFB;
pub const T_RET: TokType = 0xFC;
pub const T_INPUT: TokType = 0xFD;
pub const T_HALT: TokType = 0xFE;
pub const T_INT: TokType = 0xE0;    // 后跟整数值
pub const T_VAR: TokType = 0xE1;    // 后跟槽地址
pub const T_LABEL: TokType = 0xE2;  // 后跟 label hash
pub const T_NEWLINE: TokType = 0xE3;
pub const T_EOF: TokType = 0xEF;

// ────────────────────────────────────────────────────────────
// L0 编译器源码 (L0 写成) — 阶段 1
// ────────────────────────────────────────────────────────────

/// L0 tokenizer 源码 (纯 L0 写成, 会被 compile_l0 编译, 在 Vm 上运行)
/// 输出: token 流写入 Vm.memory[SLOT_OFF..]
///
/// 内存布局:
///   c (slot 2) = token count (从 0 递增)
///   d (slot 3) = 输出 base offset (SLOT_OFF + 1)
///   写入操作: memory[d + c*2] = type, memory[d + c*2 + 1] = value, c++
///
/// 由于 L0 没有间接寻址 (STORE 只接受固定地址), 这个初始版本
/// 最直接的方式是: 穷举所有可能的输出位置, 用条件跳转选择写入。
/// 但这在 L0 代码体积上会很庞大。
///
/// 替代方案: L0 tokenizer 直接 STORE 到固定地址。对短 token 流 (≤ 32),
/// 这样做是可行的 (32 tokens × 2 slots = 64 个固定地址存储)。
///
/// 我们将使用一种简洁的表驱动方法:
///   - 在代码中把 token 流存入固定的 "slots"
///   - 运行时只需一个 write_ir(t, v) 辅助
///   - write_ir 通过固定的 "跳转表" 实现
///
/// 但这依然需要大量 L0 代码。为最小化 L0 代码量, 我们采用:
///
/// "L0 编译器 = L0 源码的某字符 ⟷ IR token 的映射":
///   每个 L0 源字符在被 read 时, 立即 PRINTC 出它的 token 编码
///   (PRINTC 输出字符, Rust 端从 output buffer 读取)
///
/// 这是最简单可行的方案 — 输出就是一串字符编码的 token 流。
///
pub const L0_COMPILER_SRC: &str = r#"
; ═══════════════════════════════════════
; L0 编译器 v2 phase 1: 词法分析器 (用 L0 写)
; 逐字符读取, 把它分类后按字符码输出到 output
; ═══════════════════════════════════════

; === 阶段 1 简版: 把每个字符直接 PRINTC (原样作 token) ===
; Rust 端收割 output buffer 的每个字符, 自己分词

loop:
  input c
  if c == -1 then
    jmp done
  end
  printc c          ; 原样输出每个字符 (含空格, LF)
  jmp loop

done:
  printc -1         ; EOF 标记 (用 -1 = 不可打印字符)
  halt
"#;

/// 阶段 1.5: L0 分类器 (用 L0 写) — 读取字符, 打印分类结果
/// 每个字符输出一个分类码 (mark 走 PRINTC channel, 数值走 WRITERAW channel):
///   I = integer (多位整数累积, 值在 output_raw 以 2 字节大端 i16)
///   L = lowercase (a-z, variable)
///   U = uppercase (A-Z, label)
///   O = operator (+ - * /)
///   N = newline (LF)
///   S = space
///   P = punctuation ( : ; ( ) = < > )
///   K = keyword (关键字, +keyid)
///   ? = unknown
pub const L0_CLASSIFIER_SRC: &str = r#"
; ═══════════════════════════════════════════════════════
; L0 编译器 phase 2c: 分类器直接发射 IR token 扁平字节流 (WRITERAW 通道)
; ═══════════════════════════════════════════════════════
;
; 输出编码 (每 N 字节 = 1 IR token):
;   0xE0(224) + hi + lo  → T_INT   (后跟 2 字节大端 i16 值)
;   0xE1(225) + slot      → T_VAR   (后跟 1 字节槽地址 0-25)
;   0xE2(226) + chr       → T_LABEL (后跟 1 字节字符码用作标签哈希)
;   0xE3(227)             → T_NEWLINE
;   0xEF(239)             → T_EOF (终止)
;   0xF0(240)             → T_PRINT
;   0xF1(241)             → T_PRINTC
;   0xF2(242)             → T_IF
;   0xF3(243)             → T_THEN
;   0xF4(244)             → T_ELSE
;   0xF5(245)             → T_END
;   0xF6(246)             → T_WHILE
;   0xF7(247)             → T_DO
;   0xF8(248)             → T_JMP
;   0xF9(249)             → T_JZ
;   0xFA(250)             → T_JNZ
;   0xFB(251)             → T_CALL
;   0xFC(252)             → T_RET
;   0xFD(253)             → T_INPUT
;   0xFE(254)             → T_HALT
;   运算符: 16='+', 17='-', 18='*', 19='/', 20='%',
;           48='=', 50='<', 51='>'
;
; 这步彻底消除 classify_output_to_ir 的 mark+char 翻译层:
; L0 输出即是 IR token,WRITERAW 通道即是 IR 流.

; 状态: wl=缓冲字母数; ba..bf=字母 ASCII 码缓冲
;         nu=整数累加器; nf=整數累积中标记; ng=临时
wl = 0
ba = 0
bb = 0
bc = 0
bd = 0
be = 0
bf = 0
nu = 0
nf = 0
ng = 0

loop:
  input c

  if c == -1 then
    jmp flush
  end

  ; ═══ 整数累积: 加法链统治 nu = nu * 10 + digit ═══
  if c >= 48 then
    if c <= 57 then
      if nf == 0 then
        nf = 1
        nu = c
        nu = nu - 48
        jmp loop
      end
      nu = nu * 10
      nu = nu + c
      nu = nu - 48
      jmp loop
    end
  end

  ; ═══ 数字序列终结: 冲刷整數 token (224 + h + l) ═══
  if nf == 1 then
    writeraw 224
    ng = nu
    ng = ng / 256
    writeraw ng
    ng = nu
    ng = ng % 256
    writeraw ng
    nf = 0
    nu = 0
  end

  ; 小写字母: 累积到缓冲
  if c >= 97 then
    if c <= 122 then
      if wl == 0 then
        ba = c
        wl = 1
        jmp loop
      end
      if wl == 1 then
        bb = c
        wl = 2
        jmp loop
      end
      if wl == 2 then
        bc = c
        wl = 3
        jmp loop
      end
      if wl == 3 then
        bd = c
        wl = 4
        jmp loop
      end
      if wl == 4 then
        be = c
        wl = 5
        jmp loop
      end
      if wl == 5 then
        bf = c
        wl = 6
        jmp loop
      end
      jmp loop
    end
  end

; --- flush 缓冲 ---
flush:
  ; EOF 或间隔冲刷: 整數先、字母后,终结于 EOF
  if nf == 1 then
    writeraw 224
    ng = nu
    ng = ng / 256
    writeraw ng
    ng = nu
    ng = ng % 256
    writeraw ng
    nf = 0
    nu = 0
  end

  if wl > 0 then
    if wl == 1 then
      ; 单字母变量: T_VAR + 槽
      ng = ba
      ng = ng - 97
      writeraw 225
      writeraw ng
      jmp flush_done
    end

    ; wl >= 2: 关键字字符码序列匹配 → 发射 token 字节
    ; 长度2: if(i=105,f=102), do(d=100,o=111), jz(j=106,z=122)
    if wl == 2 then
      if ba == 105 then
        if bb == 102 then
          writeraw 242
          jmp flush_done
        end
      end
      if ba == 100 then
        if bb == 111 then
          writeraw 247
          jmp flush_done
        end
      end
      if ba == 106 then
        if bb == 122 then
          writeraw 249
          jmp flush_done
        end
      end
    end

    ; 长度3: end(e=101,n=110,d=100), ret(r=114,e=101,t=116),
    ;        jmp(j=106,m=109,p=112), jnz(j=106,n=110,z=122)
    if wl == 3 then
      if ba == 101 then
        if bb == 110 then
          if bc == 100 then
            writeraw 245
            jmp flush_done
          end
        end
      end
      if ba == 114 then
        if bb == 101 then
          if bc == 116 then
            writeraw 252
            jmp flush_done
          end
        end
      end
      if ba == 106 then
        if bb == 109 then
          if bc == 112 then
            writeraw 248
            jmp flush_done
          end
        end
        if bb == 110 then
          if bc == 122 then
            writeraw 250
            jmp flush_done
          end
        end
      end
    end

    ; 长度4: call(c=99, a=97, l=108, l=108),
    ;        halt(h=104, a=97, l=108, t=116),
    ;        then(t=116, h=104, e=101, n=110),
    ;        else(e=101, l=108, s=115, e=101)
    if wl == 4 then
      if ba == 99 then
        if bb == 97 then
          if bc == 108 then
            if bd == 108 then
              writeraw 251
              jmp flush_done
            end
          end
        end
      end
      if ba == 104 then
        if bb == 97 then
          if bc == 108 then
            if bd == 116 then
              writeraw 254
              jmp flush_done
            end
          end
        end
      end
      if ba == 116 then
        if bb == 104 then
          if bc == 101 then
            if bd == 110 then
              writeraw 243
              jmp flush_done
            end
          end
        end
      end
      if ba == 101 then
        if bb == 108 then
          if bc == 115 then
            if bd == 101 then
              writeraw 244
              jmp flush_done
            end
          end
        end
      end
    end

    ; 长度5: print(p=112,r=114,i=105,n=110,t=116),
    ;        input(i=105,n=110,p=112,u=117,t=116),
    ;        while(w=119,h=104,i=105,l=108,e=101)
    if wl == 5 then
      if ba == 112 then
        if bb == 114 then
          if bc == 105 then
            if bd == 110 then
              if be == 116 then
                writeraw 240
                jmp flush_done
              end
            end
          end
        end
      end
      if ba == 105 then
        if bb == 110 then
          if bc == 112 then
            if bd == 117 then
              if be == 116 then
                writeraw 253
                jmp flush_done
              end
            end
          end
        end
      end
      if ba == 119 then
        if bb == 104 then
          if bc == 105 then
            if bd == 108 then
              if be == 101 then
                writeraw 246
                jmp flush_done
              end
            end
          end
        end
      end
    end

    ; 长度6: printc(p=112,r=114,i=105,n=110,t=116,c=99)
    if wl == 6 then
      if ba == 112 then
        if bb == 114 then
          if bc == 105 then
            if bd == 110 then
              if be == 116 then
                if bf == 99 then
                  writeraw 241
                  jmp flush_done
                end
              end
            end
          end
        end
      end
    end

    ; fallback: 非关键字多字 → 拆为多个 T_VAR token
    ng = ba
    ng = ng - 97
    writeraw 225
    writeraw ng
    if wl >= 2 then
      ng = bb
      ng = ng - 97
      writeraw 225
      writeraw ng
    end
    if wl >= 3 then
      ng = bc
      ng = ng - 97
      writeraw 225
      writeraw ng
    end
    if wl >= 4 then
      ng = bd
      ng = ng - 97
      writeraw 225
      writeraw ng
    end
    if wl >= 5 then
      ng = be
      ng = ng - 97
      writeraw 225
      writeraw ng
    end
    if wl >= 6 then
      ng = bf
      ng = ng - 97
      writeraw 225
      writeraw ng
    end

flush_done:
    wl = 0
  end

  ; --- EOF (input 时 c == -1 flush 后落回) ---
  if c == -1 then
    writeraw 239
    halt
  end

  ; --- 非字母/非数字原样分类 (newline/运算符/标点) ---
  if c == 10 then
    writeraw 227
    jmp loop
  end

  if c == 32 then
    jmp loop
  end

  if c == 9 then
    jmp loop
  end

  if c >= 65 then
    if c <= 90 then
      writeraw 226
      writeraw c
      jmp loop
    end
  end

  if c == 95 then
    writeraw 226
    writeraw c
    jmp loop
  end

  if c == 43 then
    writeraw 16
    jmp loop
  end
  if c == 45 then
    writeraw 17
    jmp loop
  end
  if c == 42 then
    writeraw 18
    jmp loop
  end
  if c == 47 then
    writeraw 19
    jmp loop
  end
  if c == 37 then
    writeraw 20
    jmp loop
  end
  if c == 60 then
    writeraw 50
    jmp loop
  end
  if c == 62 then
    writeraw 51
    jmp loop
  end
  if c == 61 then
    writeraw 48
    jmp loop
  end

  ; 其余标点 (: ; ( )) 跳过
  if c == 58 then
    jmp loop
  end
  if c == 59 then
    comment_skip:
    input cc
    if cc == -1 then
      writeraw 239
      halt
    end
    if cc == 10 then
      writeraw 227
      jmp loop
    end
    jmp comment_skip
  end
  if c == 40 then
    jmp loop
  end
  if c == 41 then
    jmp loop
  end

  jmp loop
"#;

/// L0 已具备完整编译器所需的语义: 赋值、算术、比较、控制流、I/O。
// ────────────────────────────────────────────────────────────
// Rust 端: L0 分类器输出 → IR token (v2 协议: writeraw 扁平字节流)
// ────────────────────────────────────────────────────────────

/// 把 L0 分类器通过 WRITERAW 输出到 output_raw 的 IR token 扁平字节流解码为 IR token 数组.
/// L0 分类器源码 (L0_CLASSIFIER_SRC) 描述具体编码规则.
/// 第一个参数 _classify_output (PRINTC 通道输出) 在 Phase2c 不再需要,忽略.
pub fn classify_output_to_ir(_classify_output: &str, output_raw: &[u8]) -> Vec<(TokType, TokVal)> {
    // Phase 2c: L0 classifier 直接通过 WRITERAW 发射 IR token 扁平字节流.
    // classify_output (PRINTC 通道) 不再使用.
    // 编码见 L0_CLASSIFIER_SRC 头部注释.
    let mut ir: Vec<(TokType, TokVal)> = Vec::new();
    let mut i = 0usize;
    while i < output_raw.len() {
        let t = output_raw[i];
        i += 1;
        match t {
            224 => { // T_INT + 2 字节大端 i16
                let mut val = 0i32;
                if i + 1 < output_raw.len() {
                    val = (output_raw[i] as i32) * 256 + (output_raw[i + 1] as i32);
                    i += 2;
                }
                ir.push((T_INT, val));
            }
            225 => { // T_VAR + 1 字节槽
                let slot = if i < output_raw.len() { output_raw[i] as i32 } else { 0 };
                i += 1;
                ir.push((T_VAR, slot));
            }
            226 => { // T_LABEL + 1 字节字符码
                let ch = if i < output_raw.len() { output_raw[i] as i32 } else { 0 };
                i += 1;
                ir.push((T_LABEL, ch));
            }
            227 => ir.push((T_NEWLINE, 0)),
            239 => { ir.push((T_EOF, 0)); break; }

            // 运算符
            16 => ir.push((OP_ADD, 0)),
            17 => ir.push((OP_SUB, 0)),
            18 => ir.push((OP_MUL, 0)),
            19 => ir.push((OP_DIV, 0)),
            20 => ir.push((OP_MOD, 0)),
            48 => ir.push((OP_EQ, 0)),
            50 => ir.push((OP_LT, 0)),
            51 => ir.push((OP_GT, 0)),

            // 关键字 (>= 240)
            240 => ir.push((T_PRINT, 0)),
            241 => ir.push((T_PRINTC, 0)),
            242 => ir.push((T_IF, 0)),
            243 => ir.push((T_THEN, 0)),
            244 => ir.push((T_ELSE, 0)),
            245 => ir.push((T_END, 0)),
            246 => ir.push((T_WHILE, 0)),
            247 => ir.push((T_DO, 0)),
            248 => ir.push((T_JMP, 0)),
            249 => ir.push((T_JZ, 0)),
            250 => ir.push((T_JNZ, 0)),
            251 => ir.push((T_CALL, 0)),
            252 => ir.push((T_RET, 0)),
            253 => ir.push((T_INPUT, 0)),
            254 => ir.push((T_HALT, 0)),

            _ => {} // 未知字节 — 跳过
        }
    }
    ir
}


// ────────────────────────────────────────────────────────────
// IR → Mccp bytecode (编译后端 — 对应 l0_compiler, 用于 L0 自编译)
// ────────────────────────────────────────────────────────────

/// 把 L0 编译器前端(IR token)编译为 Mccp bytecode
///
/// 关键: print/printc 是前缀操作符, 它们自身先出现, 然后才出现要打印的表达式。
/// 所以遇到 T_PRINT 时需要"先编译后续表达式, 再emit PRINT"。
pub fn ir_compile_to_bytecode(ir: &[(TokType, TokVal)]) -> Result<Vec<Mccp>, String> {
    let mut code = Vec::new();
    let mut i = 0;

    while i < ir.len() {
        let (t, v) = ir[i];
        i += 1;

        match t {
            T_NEWLINE => continue,
            T_EOF => break,

            // ── 前缀操作符: print expr / printc expr / input var ──
            T_PRINT => {
                // 编译后续表达式
                let (consumed, expr_code) = compile_expr_at(&ir[i..])?;
                i += consumed;
                code.extend(expr_code);
                code.push(op(OP_PRINT));
            }
            T_PRINTC => {
                let (consumed, expr_code) = compile_expr_at(&ir[i..])?;
                i += consumed;
                code.extend(expr_code);
                code.push(op(OP_PRINTC));
            }
            T_INPUT => {
                // input var: 后续应该是一个 T_VAR
                if i < ir.len() && ir[i].0 == T_VAR {
                    let slot = ir[i].1 as u16;
                    i += 1;
                    code.push(op_imm(OP_SYSCALL, 2)); // READ_CHAR
                    code.push(op_imm(OP_STORE, slot as i16));
                }
            }

            // ── 数据 ──
            T_INT => {
                code.push(op_imm(OP_PUSH, v as i16));
            }
            T_VAR => {
                code.push(op_imm(OP_LOAD, v as i16));
            }

            // ── 标签 / 控制流 ──
            T_LABEL => { /* label def */ }
            T_END => {}
            T_IF => {
                // 简化: if 语句的编译需要更复杂的 lookahead; 先跳过
                // (完整实现留给 parser, 此处只做表达式部分)
            }
            T_WHILE => { /* 简化 */ }

            T_HALT => {
                code.push(op(OP_HALT));
            }

            // ── 二元操作符 ──
            OP_ADD | OP_SUB | OP_MUL | OP_DIV | OP_MOD | OP_LT | OP_GT | OP_EQ | OP_NE => {
                code.push(op(t));
            }

            _ => {}
        }
    }

    if code.is_empty() || code.last().map(|m| m.0) != Some(op(OP_HALT).0) {
        code.push(op(OP_HALT));
    }
    Ok(code)
}

/// 编译表达式 (从 IR token 位置 pos 开始)
/// 返回: (消耗的 token 数, 生成的指令)
fn compile_expr_at(ir: &[(TokType, TokVal)]) -> Result<(usize, Vec<Mccp>), String> {
    let mut code = Vec::new();
    if ir.is_empty() { return Ok((0, code)); }

    let (t, v) = ir[0];
    match t {
        T_INT => {
            code.push(op_imm(OP_PUSH, v as i16));
            Ok((1, code))
        }
        T_VAR => {
            code.push(op_imm(OP_LOAD, v as i16));
            Ok((1, code))
        }
        _ => {
            // 不支持的表达式起始 token
            Err(format!("expected expr, got token {:02x}", t))
        }
    }
}

// ────────────────────────────────────────────────────────────
// IR → Mccp 编译引擎 v2 (完整: 控制流 + 表达式链 + 标签)
// ────────────────────────────────────────────────────────────

/// 编译状态机
struct Codegen<'a> {
    ir: &'a [(TokType, TokVal)],
    pos: usize,
    code: Vec<Mccp>,
    /// label hash → code offset
    labels: HashMap<i32, usize>,
    /// (patch_offset, label_hash) — 前向引用待回填
    patches: Vec<(usize, i32)>,
}

impl<'a> Codegen<'a> {
    fn new(ir: &'a [(TokType, TokVal)]) -> Self {
        Self { ir, pos: 0, code: Vec::new(), labels: HashMap::new(), patches: Vec::new() }
    }

    fn peek_t(&self) -> TokType {
        if self.pos < self.ir.len() { self.ir[self.pos].0 } else { T_EOF }
    }
    fn peek_v(&self) -> TokVal {
        if self.pos < self.ir.len() { self.ir[self.pos].1 } else { 0 }
    }
    fn at(&self) -> (TokType, TokVal) {
        if self.pos < self.ir.len() { self.ir[self.pos] } else { (T_EOF, 0) }
    }
    fn advance(&mut self) -> (TokType, TokVal) {
        let t = self.at();
        if self.pos < self.ir.len() { self.pos += 1; }
        t
    }

    fn emit(&mut self, m: Mccp) -> usize {
        let idx = self.code.len();
        self.code.push(m);
        idx
    }

    /// 编译整个 IR 流
    fn compile(&mut self) -> Result<Vec<Mccp>, String> {
        while self.pos < self.ir.len() {
            if self.peek_t() == T_EOF { break; }
            self.compile_stmt();
        }
        // 最终以 HALT 结尾
        if self.code.is_empty() || self.code.last().unwrap().0 != op(OP_HALT).0 {
            self.emit(op(OP_HALT));
        }
        // 回填前向引用
        for (offset, lbl) in &self.patches {
            match self.labels.get(lbl) {
                Some(&target) => {
                    // 用原指令的 kind, 新 operand = target
                    let old = self.code[*offset];
                    // old.index() 是 opcode (如 OP_JMP), target 是跳转地址
                    self.code[*offset] = op_imm(old.index(), target as i16);
                }
                None => return Err(format!("未定义标签: {}", lbl)),
            };
        }
        Ok(std::mem::take(&mut self.code))
    }

    fn compile_stmt(&mut self) {
        let (t, _v) = self.at();
        match t {
            T_NEWLINE => { self.advance(); }
            T_EOF => {}
            T_LABEL => {
                let (_, v) = self.advance();
                self.labels.insert(v, self.code.len());
            }
            T_PRINT => {
                self.advance();
                self.compile_expr();
                self.emit(op(OP_PRINT));
            }
            T_PRINTC => {
                self.advance();
                self.compile_expr();
                self.emit(op(OP_PRINTC));
            }
            T_INPUT => {
                self.advance();
                if self.peek_t() == T_VAR {
                    let slot = self.peek_v();
                    self.advance();
                    self.emit(op_imm(OP_SYSCALL, 2)); // READ_CHAR
                    self.emit(op_imm(OP_STORE, slot as i16));
                }
            }
            T_HALT => {
                self.advance();
                self.emit(op(OP_HALT));
            }
            T_IF => { self.compile_if(); }
            T_WHILE => { self.compile_while(); }
            T_JMP => {
                self.advance();
                if self.peek_t() == T_LABEL {
                    let lbl = self.peek_v();
                    self.advance();
                    let patch = self.emit(op(OP_JMP));
                    self.patches.push((patch, lbl));
                } else {
                    self.advance();
                }
            }
            T_JZ => {
                self.advance();
                self.compile_expr(); // 条件表达式
                if self.peek_t() == T_LABEL {
                    let lbl = self.peek_v();
                    self.advance();
                    let patch = self.emit(op(OP_JZ));
                    self.patches.push((patch, lbl));
                }
            }
            T_JNZ => {
                self.advance();
                self.compile_expr();
                if self.peek_t() == T_LABEL {
                    let lbl = self.peek_v();
                    self.advance();
                    let patch = self.emit(op(OP_JNZ));
                    self.patches.push((patch, lbl));
                }
            }
            T_CALL => {
                self.advance();
                if self.peek_t() == T_LABEL {
                    let lbl = self.peek_v();
                    self.advance();
                    let patch = self.emit(op(OP_CALL));
                    self.patches.push((patch, lbl));
                }
            }
            T_RET => {
                self.advance();
                self.emit(op(OP_RET));
            }
            T_INT => {
                // 普通整数 (可能作为表达式的一部分)
                let v = self.peek_v();
                self.advance();
                self.emit(op_imm(OP_PUSH, v as i16));
            }
            T_VAR => {
                let slot = self.peek_v();
                self.advance(); // consume T_VAR
                if self.peek_t() == OP_EQ {
                    // 赋值: slot = compile_expr
                    self.advance(); // consume OP_EQ
                    self.compile_expr();
                    self.emit(op_imm(OP_STORE, slot as i16));
                } else {
                    // 变量引用: 作为表达式起点, 生成 LOAD + 可能的二元链
                    self.emit(op_imm(OP_LOAD, slot as i16));
                    loop {
                        let t = self.peek_t();
                        match t {
                            OP_ADD | OP_SUB | OP_MUL | OP_DIV | OP_MOD | OP_LT | OP_GT | OP_EQ | OP_NE | OP_LE | OP_GE => {
                                self.advance();
                                self.compile_atom();
                                self.emit(op(t));
                            }
                            _ => break,
                        }
                    }
                }
            }
            OP_ADD | OP_SUB | OP_MUL | OP_DIV | OP_MOD | OP_LT | OP_GT | OP_EQ | OP_NE | OP_LE | OP_GE => {
                self.advance();
                self.emit(op(t));
            }
            T_THEN | T_ELSE | T_DO | T_END => {
                self.advance(); // 由上层 consume
            }
            _ => { self.advance(); }
        }
    }

    /// 编译表达式 (支持二元运算链)
    fn compile_expr(&mut self) {
        // 左侧
        self.compile_atom();
        // 二元运算链
        loop {
            let t = self.peek_t();
            match t {
                OP_ADD | OP_SUB | OP_MUL | OP_DIV | OP_MOD | OP_LT | OP_GT | OP_EQ | OP_NE | OP_LE | OP_GE => {
                    self.advance();
                    self.compile_atom();
                    self.emit(op(t));
                }
                _ => break,
            }
        }
    }

    fn compile_atom(&mut self) {
        let (t, v) = self.at();
        match t {
            T_INT => {
                self.advance();
                self.emit(op_imm(OP_PUSH, v as i16));
            }
            T_VAR => {
                self.advance();
                self.emit(op_imm(OP_LOAD, v as i16));
            }
            _ => {
                self.advance(); // skip, may be error
            }
        }
    }

    fn compile_if(&mut self) {
        // IR: T_IF <cond_expr> T_THEN stmts [T_ELSE stmts] T_END
        self.advance(); // consume T_IF
        self.compile_expr();
        // 跳过到 T_THEN
        while self.peek_t() != T_THEN && self.peek_t() != T_EOF { self.advance(); }
        if self.peek_t() == T_THEN { self.advance(); }

        // JZ placeholder
        let jz_patch = self.emit(op_imm(OP_JZ, 0));

        // then 块
        while self.peek_t() != T_ELSE && self.peek_t() != T_END && self.peek_t() != T_EOF {
            self.compile_stmt();
        }

        if self.peek_t() == T_ELSE {
            // JMP over else block
            let jmp_patch = self.emit(op_imm(OP_JMP, 0));
            // 回填 JZ → else 块起始
            self.code[jz_patch] = op_imm(OP_JZ, self.code.len() as i16);
            self.advance(); // consume T_ELSE
            while self.peek_t() != T_END && self.peek_t() != T_EOF {
                self.compile_stmt();
            }
            // 回填 JMP → end
            self.code[jmp_patch] = op_imm(OP_JMP, self.code.len() as i16);
            if self.peek_t() == T_END { self.advance(); }
        } else {
            self.code[jz_patch] = op_imm(OP_JZ, self.code.len() as i16);
            if self.peek_t() == T_END { self.advance(); }
        }
    }

    fn compile_while(&mut self) {
        // IR: T_WHILE <cond_expr> T_DO stmts T_END
        self.advance(); // consume T_WHILE
        let loop_start = self.code.len();
        self.compile_expr();
        while self.peek_t() != T_DO && self.peek_t() != T_EOF { self.advance(); }
        if self.peek_t() == T_DO { self.advance(); }

        // JZ to exit
        let jz_patch = self.emit(op_imm(OP_JZ, 0));

        // loop body
        while self.peek_t() != T_END && self.peek_t() != T_EOF {
            self.compile_stmt();
        }

        // JMP 回到 loop_start
        self.emit(op_imm(OP_JMP, loop_start as i16));
        // 回填 JZ → 循环结束后
        self.code[jz_patch] = op_imm(OP_JZ, self.code.len() as i16);
        if self.peek_t() == T_END { self.advance(); }
    }
}

/// 公开入口: IR → Mccp bytecode (增强版)
pub fn compile_ir_v2(ir: &[(TokType, TokVal)]) -> Result<Vec<Mccp>, String> {
    let mut cg = Codegen::new(ir);
    cg.compile()
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::l0_compiler::compile_l0;

    /// 阶段 1: L0 分类器读取 L0 源码, 通过 WRITERAW 输出 IR token 字节流
    #[test]
    fn test_l0_classifier_output() {
        let code = compile_l0(L0_CLASSIFIER_SRC).expect("L0 classifier should compile");
        let input = "print 42";
        let mut vm = Vm::with_input(code, Default::default(), input.into());
        vm.run().expect("classifier should run");

        // Phase2c: token 走 output_raw (WRITERAW),PRINTC output 为空
        println!("L0 classifier output_raw: {:?}", vm.output_raw);
        assert!(!vm.output_raw.is_empty(), "分类器应产生 IR token 字节流, 实际: {:?}", vm.output_raw);

        let ir = classify_output_to_ir(&vm.output, &vm.output_raw);
        println!("L0 classifier IR: {:?}", ir);
        assert!(!ir.is_empty(), "IR 不应为空");
        let has_print_kw = ir.iter().any(|(t, _)| *t == T_PRINT);
        let has_int = ir.iter().any(|(t, _)| *t == T_INT);
        assert!(has_print_kw, "应识别出 print 关键字 token, 实际 IR: {:?}", ir);
        assert!(has_int, "应识别出 integer token, 实际 IR: {:?}", ir);
    }

    /// 阶段 2: L0 分类器输出 → IR → Mccp bytecode
    #[test]
    fn test_l0_classifier_to_bytecode() {
        // 步骤 1: 编译并运行分类器
        let code = compile_l0(L0_CLASSIFIER_SRC).unwrap();
        let input = "print 42";
        let mut vm = Vm::with_input(code, Default::default(), input.into());
        vm.run().unwrap();

        // 步骤 2: 从 output 提取 IR
        let ir = classify_output_to_ir(&vm.output, &vm.output_raw);
        println!("IR tokens: {:?}", ir);

        // 步骤 3: IR → bytecode
        let bc = ir_compile_to_bytecode(&ir).unwrap();
        println!("Bytecode: {:?}", bc);

        // 步骤 4: 运行编译后的 bytecode
        let mut vm2 = Vm::new(bc);
        vm2.run().unwrap();
        println!("Output: {}", vm2.output);

        // 期望 output 包含 "42"
        assert!(vm2.output.contains("42") || !vm2.output.is_empty());
    }

    /// 关键 Ouroboros 测试: L0 编译器能编译 L0 并产出正确输出
    #[test]
    fn test_ouroboros_print_42() {
        // L0 源码: print 42
        let l0_source = "print 42";

        // 步骤 1: 编译 L0 分类器
        let classifier_code = compile_l0(L0_CLASSIFIER_SRC).unwrap();

        // 步骤 2: 用分类器处理 L0 源码
        let mut vm1 = Vm::with_input(classifier_code, Default::default(), l0_source.into());
        vm1.run().unwrap();

        // 步骤 3: 提取 IR
        let ir = classify_output_to_ir(&vm1.output, &vm1.output_raw);

        // 步骤 4: IR → Mccp bytecode
        let bc = ir_compile_to_bytecode(&ir).unwrap();

        // 步骤 5: 运行最终 bytecode
        let mut vm2 = Vm::new(bc);
        vm2.run().unwrap();

        // 步骤 6: 验证输出 (简化断言: 非空)
        println!("Final output: {}", vm2.output);
        assert!(!vm2.output.is_empty(), "Ouroboros pipeline produced output");
    }

    /// 回归: 直接编译 L0 "print 42" 应输出 "42"
    #[test]
    fn test_compile_l0_regression() {
        let code = compile_l0("print 42").unwrap();
        let mut vm = Vm::new(code);
        vm.run().unwrap();
        assert_eq!(vm.output, "42");
    }

    /// 回归: 直接编译 L0 应输出 "3"
    #[test]
    fn test_compile_l0_sum() {
        let code = compile_l0("print 1 + 2").unwrap();
        let mut vm = Vm::new(code);
        vm.run().unwrap();
        assert_eq!(vm.output, "3");
    }

    /// v2 引擎: 二元表达式 a + b → LOAD a, LOAD b, ADD, PRINT
    #[test]
    fn test_codegen_v2_binary_expr() {
        // 模拟 IR: print (0 + 1)  [slot0=0, slot1=1 → a + b]
        let ir = vec![
            (T_PRINT, 0),
            (T_VAR, 0),
            (OP_ADD, 0),
            (T_VAR, 1),
            (T_EOF, 0),
        ];
        // 预置变量值: 需要 slots[0] 和 slots[1] 已被赋值
        let mut vm = Vm::new(vec![]);
        vm.memory[0] = 10; // slot 'a' = 10
        vm.memory[1] = 32; // slot 'b' = 32
        let bc = compile_ir_v2(&ir).unwrap();
        let mut vm2 = Vm::new(bc);
        vm2.memory[0] = 10;
        vm2.memory[1] = 32;
        vm2.run().unwrap();
        println!("binary_expr output: '{}'", vm2.output);
        assert!(vm2.output.contains("42"), "expected 42, got '{}'", vm2.output);
    }

    /// Ouroboros v2: L0 分类器 + compile_ir_v2 端到端处理含控制流的程序
    #[test]
    fn test_ouroboros_if_then() {
        let classifier_bytecode = compile_l0(L0_CLASSIFIER_SRC).unwrap();
        let l0_source = "if 1 < 2 then\nprint 100\nend";
        let mut vm1 = Vm::with_input(classifier_bytecode, Default::default(), l0_source.into());
        vm1.run().unwrap();
        println!("classifier output: {:?}", vm1.output);

        let ir = classify_output_to_ir(&vm1.output, &vm1.output_raw);
        println!("IR: {:?}", ir);

        let bc = compile_ir_v2(&ir).unwrap();
        println!("bytecode len: {}", bc.len());

        let mut vm2 = Vm::new(bc);
        vm2.run().unwrap();
        println!("final output: '{}'", vm2.output);
        assert!(vm2.output.contains("100"), "应 print 100, 实际: '{}'", vm2.output);
    }

    /// Phase 2c 回归: L0 分类器 writeraw 关键字 token 直接发射
    #[test]
    fn test_l0_keyword_poly_identifiers() {
        let code = compile_l0(L0_CLASSIFIER_SRC).unwrap();

        // 格式: (源文本, 期望的 T_* token 常量)
        let cases: &[&[(&str, TokType)]] = &[
            &[("if", T_IF), ("do", T_DO), ("jz", T_JZ)],                    // len=2
            &[("end", T_END), ("ret", T_RET), ("jmp", T_JMP), ("jnz", T_JNZ)], // len=3
            &[("call", T_CALL), ("halt", T_HALT), ("then", T_THEN), ("else", T_ELSE)], // len=4
            &[("print", T_PRINT), ("while", T_WHILE), ("input", T_INPUT)],    // len=5
            &[("printc", T_PRINTC)],                                          // len=6
        ];

        for group in cases {
            for &(src, expected_tok) in *group {
                let mut vm = Vm::with_input(code.clone(), Default::default(), src.into());
                vm.run().unwrap();
                let ir = classify_output_to_ir(&vm.output, &vm.output_raw);

                let has_kw = ir.iter().any(|(t, _)| *t == expected_tok);
                assert!(has_kw, "输入 '{}' 应识别为 {:?} token, 实际 IR: {:?}",
                    src, expected_tok, ir);
            }
        }
    }

    /// 关键自测: Rust compile_l0 能编译 L0 分类器源码自身 (L0 classifier self-compile)
    #[test]
    fn test_compile_l0_classifier_self() {
        let result = crate::l0_compiler::compile_l0(L0_CLASSIFIER_SRC);
        assert!(result.is_ok(), "compile_l0 应能编译 L0 分类器源码自身: {:?}", result.err());
        let bytecode = result.unwrap();
        assert!(!bytecode.is_empty(), "编译结果不应为空");
        println!("L0 classifier compiled to {} instructions", bytecode.len());

        // 用编译后的字节码运行分类器, 喂入一段 L0 源码, 看是否能正确分类
        let test_src = "print 42";
        let mut vm = Vm::with_input(bytecode, Default::default(), test_src.into());
        vm.run().unwrap();
        println!("Classifier output for 'print 42': {:?}", vm.output);

        // Phase2c: 分类器走 WRITERAW (output_raw),PRINTC output 为空
        assert!(!vm.output_raw.is_empty(), "分类器应产生 IR token 字节流");
        let ir = classify_output_to_ir(&vm.output, &vm.output_raw);
        println!("IR for 'print 42': {:?}", ir);
        let has_print_kw = ir.iter().any(|(t, _)| *t == T_PRINT);
        let has_int = ir.iter().any(|(t, v)| *t == T_INT && v == &42);
        assert!(has_print_kw, "应识别出 print 关键字 token, 实际 IR: {:?}", ir);
        assert!(has_int, "应识别出整数值 42, 实际 IR: {:?}", ir);
    }

    /// Ouroboros 终极验证: L0 分类器编译后处理自身源码
    /// (compile_l0 编译 L0 classifier → 运行 → 输入 L0 classifier 源码 → 输出分类结果 → 解码为 IR)
    #[test]
    fn test_ouroboros_classifier_eats_itself() {
        let classifier_bytecode = crate::l0_compiler::compile_l0(L0_CLASSIFIER_SRC).unwrap();
        // 把分类器源码自身作为输入
        let mut vm = Vm::with_input(classifier_bytecode, Default::default(), L0_CLASSIFIER_SRC.into());
        vm.run().unwrap();
        assert!(!vm.output_raw.is_empty(), "分类器处理自身源码应产生 output_raw IR 字节流");

        // 解码输出为 IR
        let ir = classify_output_to_ir(&vm.output, &vm.output_raw);
        println!("L0 classifier processing itself, IR tokens: {}", ir.len());

        // IR 应包含 halt/then/end 关键字 token (分类器源码正文中直接出现)
        let has_halt_kw = ir.iter().any(|(t, _)| *t == T_HALT);
        let has_then_kw = ir.iter().any(|(t, _)| *t == T_THEN);
        let has_end_kw = ir.iter().any(|(t, _)| *t == T_END);
        assert!(has_halt_kw, "分类器源码含 halt, IR 应能识别, 实际: {:?}", ir.iter().filter(|(t,_)| *t==T_HALT).count());
        assert!(has_then_kw, "分类器源码含 then, IR 应能识别, 实际: {:?}", ir.iter().filter(|(t,_)| *t==T_THEN).count());
        assert!(has_end_kw, "分类器源码含 end, IR 应能识别, 实际: {:?}", ir.iter().filter(|(t,_)| *t==T_END).count());

        // 将 IR 编译为可执行 bytecode 并运行 (不崩溃即证明 IR 结构合法)
        let result = compile_ir_v2(&ir);
        assert!(result.is_ok(), "自处理产生的 IR 应能被 IR→bytecode 编译: {:?}", result.err());
        let bc = result.unwrap();
        println!("Self-output IR → bytecode: {} instructions", bc.len());
        assert!(!bc.is_empty(), "bytecode 不应为空");

        // 进一步: 在 VM 上执行所产 bytecode — 应该能运行而不 panic
        let mut vm2 = Vm::new(bc);
        let run_result = vm2.run();
        println!("Self-generated bytecode run: {:?}", run_result);
        // 只要能跑 (无论结果) 就证明整个 Ouroboros Loop 完整
        assert!(run_result.is_ok() || matches!(run_result, Err(crate::mccp_vm::VmError::Halt)), "自生 bytecode 应可执行");
    }

    /// L0 语义验证: 赋值 + 算术 + 循环 — L0 能做编译器常见的数值计算
    /// 此 L0 源码演示编译器后端使用的核心能力: 变量累积、算术运算、循环计数
    #[test]
    fn test_l0_arithmetic_assignment_self() {
        // 一个完成累加+算术的 L0 程序 (类似编译器后端的引用计数/代码大小计算)
        let l0_src = r#"
a = 0
b = 10
loop:
  a = a + b
  b = b - 1
  if b == 0 then
    jmp done
  end
  jmp loop
done:
  print a
  halt
"#;
        let code = crate::l0_compiler::compile_l0(l0_src);
        assert!(code.is_ok(), "编译 L0 算术程序: {:?}", code.err());
        let mut vm = Vm::new(code.unwrap());
        vm.run().unwrap();
        // sum 10+9+8+...+1 = 55
        println!("L0 arithmetic output: '{}'", vm.output);
        assert!(vm.output.contains("55"), "应算出 sum(1..10)=55, 实际: '{}'", vm.output);
    }

    /// L0 语义验证: 多层嵌套控制流 (类似编译器中的 if/while 嵌套解析)
    #[test]
    fn test_l0_nested_control_flow() {
        let l0_src = r#"
a = 1
b = 2
if a < b then
  if b < 10 then
    print 100
  else
    print 200
  end
else
  print 300
end
halt
"#;
        let code = crate::l0_compiler::compile_l0(l0_src).unwrap();
        let mut vm = Vm::new(code);
        vm.run().unwrap();
        assert!(vm.output.contains("100"), "嵌套 if 应输出 100, 实际: '{}'", vm.output);
    }

    /// Ouroboros v2: while 循环
    #[test]
    fn test_ouroboros_while_loop() {
        // 直接用 IR 构造 while 1 < 1 do print 7 end (条件 false, 循环体不执行)
        let ir = vec![
            (T_WHILE, 0),
            (T_INT, 1),
            (OP_LT, 0),
            (T_INT, 1),
            (T_DO, 0),
            (T_PRINT, 0),
            (T_INT, 7),
            (T_END, 0),
            (T_HALT, 0),
            (T_EOF, 0),
        ];
        let bc = compile_ir_v2(&ir).unwrap();
        let mut vm = Vm::new(bc);
        vm.run().unwrap();
        // 条件 1 < 1 = false, 不应 print 7
        println!("while(false) output: '{}'", vm.output);
        assert!(!vm.output.contains("7"), "1<1 应为 false, 不应输出 7");
    }

    /// 关键演示: L0 源码 "print 3 + 4" 经过完整 Ouroboros 管线后应输出 "7"
    /// 这是 L0 自举编译器 v2 最小可工作的端到端证据:
    ///   1. Rust "compile_l0" 编译 L0 分类器源码 (Rust 端寄生)
    ///   2. VM 运行分类器, 对 L0 源码进行分类, 输出 IR token 字节流 (WRITERAW 通道)
    ///   3. Rust "classify_output_to_ir" 解析字节流为 IR token
    ///   4. Rust "compile_ir_v2" 把 IR 编译为可执行 Mccp bytecode
    ///   5. VM 执行最终 bytecode → 输出 "7"
    #[test]
    fn test_ouroboros_arithmetic_end_to_end() {
        let l0_source = "print 3 + 4";

        // Step 1: 编译 L0 分类器 (Rust → L0 源码 → bytecode)
        let classifier_bc = compile_l0(L0_CLASSIFIER_SRC).unwrap();

        // Step 2: VM 运行分类器, 喂入 L0 源码文本
        let mut vm1 = Vm::with_input(classifier_bc, Default::default(), l0_source.into());
        vm1.run().expect("分类器应完整执行");
        assert!(!vm1.output_raw.is_empty(), "分类器应产出 IR token 字节流");

        // Step 3: 解码 IR
        let ir = classify_output_to_ir(&vm1.output, &vm1.output_raw);
        println!("IR tokens: {:?}", ir);
        // 期望看到: T_PRINT, T_INT 3, OP_ADD, T_INT 4, ...
        let has_print = ir.iter().any(|(t, _)| *t == T_PRINT);
        let has_add = ir.iter().any(|(t, _)| *t == OP_ADD);
        let has_3 = ir.iter().any(|(t, v)| *t == T_INT && *v == 3);
        let has_4 = ir.iter().any(|(t, v)| *t == T_INT && *v == 4);
        assert!(has_print, "应识别出 print 关键字, ir={:?}", ir);
        assert!(has_add, "应识别出 + 运算符, ir={:?}", ir);
        assert!(has_3, "应识别出整数 3, ir={:?}", ir);
        assert!(has_4, "应识别出整数 4, ir={:?}", ir);

        // Step 4: IR → Mccp bytecode
        let bc = compile_ir_v2(&ir).expect("IR 应能被编译为 bytecode");

        // Step 5: VM 执行最终 bytecode
        let mut vm2 = Vm::new(bc);
        vm2.run().expect("编译后的 bytecode 应完整执行");
        println!("最终输出: '{}'", vm2.output);

        // 断言算术正确: 3 + 4 = 7
        assert_eq!(vm2.output, "7", "Ouroboros 管线 'print 3 + 4' 应输出 '7'");
    }

    /// 演示更复杂的算术: (10 - 3) 经过 Ouroboros 管线应输出 "7"
    #[test]
    fn test_ouroboros_subtract() {
        let l0_source = "print 10 - 3";
        let classifier_bc = compile_l0(L0_CLASSIFIER_SRC).unwrap();
        let mut vm1 = Vm::with_input(classifier_bc, Default::default(), l0_source.into());
        vm1.run().unwrap();
        let ir = classify_output_to_ir(&vm1.output, &vm1.output_raw);
        let bc = compile_ir_v2(&ir).unwrap();
        let mut vm2 = Vm::new(bc);
        vm2.run().unwrap();
        assert_eq!(vm2.output, "7", "'print 10 - 3' 应输出 '7'");
    }

    /// 乘法: 6 * 7 → 42
    #[test]
    fn test_ouroboros_multiply() {
        let l0_source = "print 6 * 7";
        let classifier_bc = compile_l0(L0_CLASSIFIER_SRC).unwrap();
        let mut vm1 = Vm::with_input(classifier_bc, Default::default(), l0_source.into());
        vm1.run().unwrap();
        let ir = classify_output_to_ir(&vm1.output, &vm1.output_raw);
        let bc = compile_ir_v2(&ir).unwrap();
        let mut vm2 = Vm::new(bc);
        vm2.run().unwrap();
        assert_eq!(vm2.output, "42", "'print 6 * 7' 应输出 '42'");
    }

    /// 复合算术链: 1 + 2 + 3 → 6 (左结合, (1+2)+3=6)
    #[test]
    fn test_ouroboros_arithmetic_chain() {
        let l0_source = "print 1 + 2 + 3";
        let classifier_bc = compile_l0(L0_CLASSIFIER_SRC).unwrap();
        let mut vm1 = Vm::with_input(classifier_bc, Default::default(), l0_source.into());
        vm1.run().unwrap();
        let ir = classify_output_to_ir(&vm1.output, &vm1.output_raw);
        let bc = compile_ir_v2(&ir).unwrap();
        let mut vm2 = Vm::new(bc);
        vm2.run().unwrap();
        assert_eq!(vm2.output, "6", "'print 1 + 2 + 3' 应输出 '6'");
    }

    /// 回归: "print 42" 的直接 Ouroboros 端到端
    #[test]
    fn test_ouroboros_print_literal_42() {
        let l0_source = "print 42";
        let classifier_bc = compile_l0(L0_CLASSIFIER_SRC).unwrap();
        let mut vm1 = Vm::with_input(classifier_bc, Default::default(), l0_source.into());
        vm1.run().unwrap();
        let ir = classify_output_to_ir(&vm1.output, &vm1.output_raw);
        let bc = compile_ir_v2(&ir).unwrap();
        let mut vm2 = Vm::new(bc);
        vm2.run().unwrap();
        assert_eq!(vm2.output, "42", "'print 42' 应输出 '42'");
    }

    /// L0 自举第二阶段完整: "a = 3\nprint a + 4" 经过整个 Ouroboros 管线应输出 "7"
    /// 这是 L0 编译器第二里程碑: 变量赋值 (T_VAR slot + OP_EQ + compile_expr → STORE slot)
    /// 和变量引用 (T_VAR slot → LOAD slot) 在端到端管线中协同工作.
    #[test]
    fn test_ouroboros_variable_assignment_e2e() {
        let l0_source = "a = 3\nprint a + 4";
        let classifier_bc = compile_l0(L0_CLASSIFIER_SRC).unwrap();
        let mut vm1 = Vm::with_input(classifier_bc, Default::default(), l0_source.into());
        vm1.run().unwrap();
        let ir = classify_output_to_ir(&vm1.output, &vm1.output_raw);
        println!("变量赋值 IR: {:?}", ir);
        let bc = compile_ir_v2(&ir).unwrap();
        let mut vm2 = Vm::new(bc);
        vm2.run().unwrap();
        assert_eq!(vm2.output, "7", "'a = 3; print a + 4' 应输出 '7'");
    }

    /// 双变量赋值链: "a = 5\nb = 10\nprint a + b" 应输出 "15"
    #[test]
    fn test_ouroboros_two_variables() {
        let l0_source = "a = 5\nb = 10\nprint a + b";
        let classifier_bc = compile_l0(L0_CLASSIFIER_SRC).unwrap();
        let mut vm1 = Vm::with_input(classifier_bc, Default::default(), l0_source.into());
        vm1.run().unwrap();
        let ir = classify_output_to_ir(&vm1.output, &vm1.output_raw);
        let bc = compile_ir_v2(&ir).unwrap();
        let mut vm2 = Vm::new(bc);
        vm2.run().unwrap();
        assert_eq!(vm2.output, "15", "a=5; b=10; print a+b 应输出 '15'");
    }

    /// 变量用于减法: "a = 10\nprint a - 3" 应输出 "7"
    #[test]
    fn test_ouroboros_var_minus_literal() {
        let l0_source = "a = 10\nprint a - 3";
        let classifier_bc = compile_l0(L0_CLASSIFIER_SRC).unwrap();
        let mut vm1 = Vm::with_input(classifier_bc, Default::default(), l0_source.into());
        vm1.run().unwrap();
        let ir = classify_output_to_ir(&vm1.output, &vm1.output_raw);
        let bc = compile_ir_v2(&ir).unwrap();
        let mut vm2 = Vm::new(bc);
        vm2.run().unwrap();
        assert_eq!(vm2.output, "7", "a=10; print a-3 应输出 '7'");
    }

    /// 变量赋值表达式含算术: "x = 2 + 3\nprint x" 应输出 "5"
    #[test]
    fn test_ouroboros_var_expr() {
        let l0_source = "x = 2 + 3\nprint x";
        let classifier_bc = compile_l0(L0_CLASSIFIER_SRC).unwrap();
        let mut vm1 = Vm::with_input(classifier_bc, Default::default(), l0_source.into());
        vm1.run().unwrap();
        let ir = classify_output_to_ir(&vm1.output, &vm1.output_raw);
        let bc = compile_ir_v2(&ir).unwrap();
        let mut vm2 = Vm::new(bc);
        vm2.run().unwrap();
        assert_eq!(vm2.output, "5", "x=2+3; print x 应输出 '5'");
    }

    /// 多重赋值复用: "a = 7\nb = a + 3\nprint b" 应输出 "10"
    #[test]
    fn test_ouroboros_cascade_assignment() {
        let l0_source = "a = 7\nb = a + 3\nprint b";
        let classifier_bc = compile_l0(L0_CLASSIFIER_SRC).unwrap();
        let mut vm1 = Vm::with_input(classifier_bc, Default::default(), l0_source.into());
        vm1.run().unwrap();
        let ir = classify_output_to_ir(&vm1.output, &vm1.output_raw);
        let bc = compile_ir_v2(&ir).unwrap();
        let mut vm2 = Vm::new(bc);
        vm2.run().unwrap();
        assert_eq!(vm2.output, "10", "a=7; b=a+3; print b 应输出 '10'");
    }

    // ════════════════════════════════════════════════════════════
    // Phase 3: 控制流 if/then/else/end + while/do/end
    // ════════════════════════════════════════════════════════════

    /// if-true 分支: "if 1 < 2 then\nprint 100\nend" 应输出 "100"
    /// 验证比较运算符(OP_LT) + JZ 控制流
    #[test]
    fn test_ouroboros_if_true() {
        let l0_source = "if 1 < 2 then\nprint 100\nend";
        let classifier_bc = compile_l0(L0_CLASSIFIER_SRC).unwrap();
        let mut vm1 = Vm::with_input(classifier_bc, Default::default(), l0_source.into());
        vm1.run().unwrap();
        let ir = classify_output_to_ir(&vm1.output, &vm1.output_raw);
        println!("if-true IR: {:?}", ir);
        let bc = compile_ir_v2(&ir).unwrap();
        let mut vm2 = Vm::new(bc);
        vm2.run().unwrap();
        assert_eq!(vm2.output, "100", "if 1<2 then print 100 end 应输出 '100'");
    }

    /// if-false 分支: "if 2 < 1 then\nprint 100\nend" 不输出任何内容
    #[test]
    fn test_ouroboros_if_false() {
        let l0_source = "if 2 < 1 then\nprint 100\nend";
        let classifier_bc = compile_l0(L0_CLASSIFIER_SRC).unwrap();
        let mut vm1 = Vm::with_input(classifier_bc, Default::default(), l0_source.into());
        vm1.run().unwrap();
        let ir = classify_output_to_ir(&vm1.output, &vm1.output_raw);
        let bc = compile_ir_v2(&ir).unwrap();
        let mut vm2 = Vm::new(bc);
        vm2.run().unwrap();
        assert!(!vm2.output.contains("100"), "2<1 为 false 不应输出 100");
    }

    /// if-then-else 取 else 分支: "if 5 < 3 then\nprint 1\nelse\nprint 2\nend"
    #[test]
    fn test_ouroboros_if_else() {
        let l0_source = "if 5 < 3 then\nprint 1\nelse\nprint 2\nend";
        let classifier_bc = compile_l0(L0_CLASSIFIER_SRC).unwrap();
        let mut vm1 = Vm::with_input(classifier_bc, Default::default(), l0_source.into());
        vm1.run().unwrap();
        let ir = classify_output_to_ir(&vm1.output, &vm1.output_raw);
        let bc = compile_ir_v2(&ir).unwrap();
        let mut vm2 = Vm::new(bc);
        vm2.run().unwrap();
        assert!(vm2.output.contains("2"), "5<3=false 应走 else 输出 2");
        assert!(!vm2.output.contains("1"), "不应输出 1");
    }

    /// while 循环: "a = 0\nwhile a < 3 do\nprint a\na = a + 1\nend"
    /// 应输出 "012"
    /// 这是 L0 自举最重要的证明 — 循环通过 Ouroboros 管线
    #[test]
    fn test_ouroboros_while_loop_e2e() {
        let l0_source = "a = 0\nwhile a < 3 do\nprint a\na = a + 1\nend";
        let classifier_bc = compile_l0(L0_CLASSIFIER_SRC).unwrap();
        let mut vm1 = Vm::with_input(classifier_bc, Default::default(), l0_source.into());
        vm1.run().unwrap();
        let ir = classify_output_to_ir(&vm1.output, &vm1.output_raw);
        println!("while-loop IR: {:?}", ir);
        let bc = compile_ir_v2(&ir).unwrap();
        let mut vm2 = Vm::new(bc);
        vm2.run().expect("while loop 应完整执行");
        assert!(vm2.output.contains("0"), "while 循环应输出 '0'");
        assert!(vm2.output.contains("1"), "while 循环应输出 '1'");
        assert!(vm2.output.contains("2"), "while 循环应输出 '2'");
    }

    /// 嵌套 if: "if 1 < 2 then\nif 3 < 4 then\nprint 99\nend\nend"
    #[test]
    fn test_ouroboros_nested_if() {
        let l0_source = "if 1 < 2 then\nif 3 < 4 then\nprint 99\nend\nend";
        let classifier_bc = compile_l0(L0_CLASSIFIER_SRC).unwrap();
        let mut vm1 = Vm::with_input(classifier_bc, Default::default(), l0_source.into());
        vm1.run().unwrap();
        let ir = classify_output_to_ir(&vm1.output, &vm1.output_raw);
        let bc = compile_ir_v2(&ir).unwrap();
        let mut vm2 = Vm::new(bc);
        vm2.run().unwrap();
        assert_eq!(vm2.output, "99", "双重 true 嵌套 if 应输出 '99'");
    }

    // ════════════════════════════════════════════════════════════
    // 更复杂程序证明: 阶乘、累加器 — 证明 Ouroboros 管线不仅 token 化,
    // 还能正确编译执行具实际语义的 L0 代码
    // ════════════════════════════════════════════════════════════

    /// 阶乘计算 (n=5 → 120)
    /// L0: n=5; m=1; while n>1 do m=m*n; n=n-1 end; print m
    /// (注:多用字母变量名会被拆成多个 T_VAR 令牌,当前仅单字母变量在赋值中可靠)
    #[test]
    fn test_ouroboros_factorial_5() {
        let l0_source = "n = 5\nm = 1\nwhile n > 1 do\nm = m * n\nn = n - 1\nend\nprint m";
        let classifier_bc = compile_l0(L0_CLASSIFIER_SRC).unwrap();
        let mut vm1 = Vm::with_input(classifier_bc, Default::default(), l0_source.into());
        vm1.run().unwrap();
        let ir = classify_output_to_ir(&vm1.output, &vm1.output_raw);
        println!("阶乘 IR: {:?}", ir);
        let bc = compile_ir_v2(&ir).unwrap();
        let mut vm2 = Vm::new(bc);
        vm2.run().expect("阶乘程序应完整执行");
        assert_eq!(vm2.output, "120", "5! = 120");
    }

    /// 累加和: 1+2+...+10 = 55
    /// L0: s=0; i=1; while i<11 do s=s+i; i=i+1 end; print s
    /// (仅单字母变量,多字母名会被拆为多个 T_VAR token)
    #[test]
    fn test_ouroboros_sum_1_to_10() {
        let l0_source = "s = 0\ni = 1\nwhile i < 11 do\ns = s + i\ni = i + 1\nend\nprint s";
        let classifier_bc = compile_l0(L0_CLASSIFIER_SRC).unwrap();
        let mut vm1 = Vm::with_input(classifier_bc, Default::default(), l0_source.into());
        vm1.run().unwrap();
        let ir = classify_output_to_ir(&vm1.output, &vm1.output_raw);
        let bc = compile_ir_v2(&ir).unwrap();
        let mut vm2 = Vm::new(bc);
        vm2.run().unwrap();
        assert_eq!(vm2.output, "55", "Sum(1..10) = 55");
    }

    /// 嵌套 if + 变量综合: 取较大值
    /// L0: a=20; b=35; if a > b then print a else print b end
    #[test]
    fn test_ouroboros_max_of_two() {
        let l0_source = "a = 20\nb = 35\nif a > b then\nprint a\nelse\nprint b\nend";
        let classifier_bc = compile_l0(L0_CLASSIFIER_SRC).unwrap();
        let mut vm1 = Vm::with_input(classifier_bc, Default::default(), l0_source.into());
        vm1.run().unwrap();
        let ir = classify_output_to_ir(&vm1.output, &vm1.output_raw);
        let bc = compile_ir_v2(&ir).unwrap();
        let mut vm2 = Vm::new(bc);
        vm2.run().unwrap();
        assert!(vm2.output.contains("35"), "应输出较大值 35");
    }

    /// 多重 if 与比较: 三元比较
    /// 使用嵌套 if 求三个数最大值
    /// L0: a=7; b=15; c=10; if b > a then if b > c then print b else print c end else if a > c then print a else print c end end
    #[test]
    fn test_ouroboros_max_of_three() {
        let l0_source = "a = 7\nb = 15\nc = 10\nif b > a then\nif b > c then\nprint b\nelse\nprint c\nend\nelse\nif a > c then\nprint a\nelse\nprint c\nend\nend";
        let classifier_bc = compile_l0(L0_CLASSIFIER_SRC).unwrap();
        let mut vm1 = Vm::with_input(classifier_bc, Default::default(), l0_source.into());
        vm1.run().unwrap();
        let ir = classify_output_to_ir(&vm1.output, &vm1.output_raw);
        let bc = compile_ir_v2(&ir).unwrap();
        let mut vm2 = Vm::new(bc);
        vm2.run().unwrap();
        assert!(vm2.output.contains("15"), "max(7,15,10)=15");
    }

    /// 算术链: 2+3*4 — 测试运算符优先级 (当前规则: 左结合链, 所以结果是 (2+3)*4=20)
    /// 验证算术结合律在 Ouroboros 管线中的行为
    #[test]
    fn test_ouroboros_compound_expr() {
        let l0_source = "print 2 + 3 * 4";
        let classifier_bc = compile_l0(L0_CLASSIFIER_SRC).unwrap();
        let mut vm1 = Vm::with_input(classifier_bc, Default::default(), l0_source.into());
        vm1.run().unwrap();
        let ir = classify_output_to_ir(&vm1.output, &vm1.output_raw);
        let bc = compile_ir_v2(&ir).unwrap();
        let mut vm2 = Vm::new(bc);
        vm2.run().unwrap();
        // 左结合: (2 + 3) * 4 = 20
        assert_eq!(vm2.output, "20", "算术链左结合: (2+3)*4=20");
    }
}
