// ═══════════════════════════════════════════════════════════════════════════════
// compiler.rs — 活码编译器 · 容器程序生成
// ═══════════════════════════════════════════════════════════════════════════════
//
// 核心原则:
//   活码 = 以"起点"为钥匙激活的编码
//   未固定时位置任意活动, 固定后按排列顺序确定权限和数值
//   加法统计法: 累积 = 基准值 + 排队者值
//   自举: 编译器自身输出能编码自身
//
// 产物:
//   容器程序 = 有序容器操作序列
//   可序列化为二进制 (2字节/操作: 1字节操作码 + 1字节容器号)
//   反序列化后可直接由 ContainerOS 执行
//
// ═══════════════════════════════════════════════════════════════════════════════

use crate::container_runtime::ContainerOS;

/// 容器操作码
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OpCode {
    /// 激活容器 (吸收笔画)
    Absorb    = 0xA0,
    /// 嵌套: 当前壳套入下一个容器
    Nest      = 0xA1,
    /// 汇合: 合并两个容器状态
    Merge     = 0xA2,
    /// 裁决: 三元仲裁请求
    Arbitrate = 0xA3,
    /// 产出: 输出当前状态
    Emit      = 0xA4,
    /// 等待: 等待时间触发
    Wait      = 0xA5,
    /// 跳转: 条件跳转到指定偏移
    Jump      = 0xA6,
    /// 结束
    Halt      = 0xFF,
}

/// 单条容器操作
#[derive(Clone, Debug)]
pub struct Instruction {
    pub op: OpCode,
    pub operand: u8,      // 容器索引 (0-15) 或参数
}

impl Instruction {
    pub fn serialize(&self) -> [u8; 2] {
        [self.op as u8, self.operand]
    }

    pub fn deserialize(b: [u8; 2]) -> Option<Self> {
        let op = match b[0] {
            0xA0 => OpCode::Absorb,
            0xA1 => OpCode::Nest,
            0xA2 => OpCode::Merge,
            0xA3 => OpCode::Arbitrate,
            0xA4 => OpCode::Emit,
            0xA5 => OpCode::Wait,
            0xA6 => OpCode::Jump,
            0xFF => OpCode::Halt,
            _ => return None,
        };
        Some(Self { op, operand: b[1] })
    }
}

/// 容器程序 = 操作码序列
#[derive(Clone)]
pub struct ContainerProgram {
    pub instructions: Vec<Instruction>,
    /// 起点偏移 (活码钥匙位置)
    pub entry_point: u32,
    /// 源字符 (用于自举反思)
    pub source_chars: String,
    /// 编译时 ContainerOS 终态
    pub final_state: Option<ContainerOS>,
}

impl ContainerProgram {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            entry_point: 0,
            source_chars: String::new(),
            final_state: None,
        }
    }

    pub fn emit(&mut self, op: OpCode, operand: u8) {
        self.instructions.push(Instruction { op, operand });
    }

    /// 将所有指令序列化为二进制
    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();
        // Header: magic(2) + entry(4) + instr_count(4) + source_len(2)
        out.extend_from_slice(b"CP");  // magic: Container Program
        out.extend_from_slice(&self.entry_point.to_le_bytes());
        out.extend_from_slice(&(self.instructions.len() as u32).to_le_bytes());
        let src_bytes = self.source_chars.as_bytes();
        out.extend_from_slice(&(src_bytes.len() as u16).to_le_bytes());
        out.extend_from_slice(src_bytes);
        // Body: 每条指令 2 字节
        for instr in &self.instructions {
            let [a, b] = instr.serialize();
            out.push(a);
            out.push(b);
        }
        out
    }

    /// 反序列化容器程序
    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 12 || bytes[0..2] != *b"CP" { return None; }
        let entry_point = u32::from_le_bytes(bytes[2..6].try_into().ok()?);
        let count = u32::from_le_bytes(bytes[6..10].try_into().ok()?);
        let src_len = u16::from_le_bytes(bytes[10..12].try_into().ok()?);

        let src_end = 12 + src_len as usize;
        if bytes.len() < src_end { return None; }
        let source_chars = String::from_utf8_lossy(&bytes[12..src_end]).to_string();

        let mut instructions = Vec::new();
        let mut pos = src_end;
        for _ in 0..count {
            if pos + 2 > bytes.len() { break; }
            if let Some(instr) = Instruction::deserialize([bytes[pos], bytes[pos + 1]]) {
                instructions.push(instr);
            }
            pos += 2;
        }

        Some(Self {
            instructions,
            entry_point,
            source_chars,
            final_state: None,
        })
    }

    /// 执行程序到 ContainerOS
    pub fn execute(&self, os: &mut ContainerOS) -> Option<String> {
        let mut output = String::new();
        let mut pc = self.entry_point as usize;

        while pc < self.instructions.len() {
            let instr = &self.instructions[pc];
            match instr.op {
                OpCode::Absorb => os.absorb(instr.operand),
                OpCode::Nest => os.absorb((instr.operand + 1) % 16),
                OpCode::Merge => { /* merge logic here */ }
                OpCode::Arbitrate => { /* 3-party logic */ }
                OpCode::Emit => {
                    output.push_str(&format!("{:04X} ", os.cell() & 0xFFFF));
                }
                OpCode::Wait => { /* time-trigger pause */ }
                OpCode::Jump => { pc = instr.operand as usize; continue; }
                OpCode::Halt => break,
            }
            pc += 1;
        }
        Some(output)
    }

    /// 导出为二进制容器程序 (cargo 模式下写入 .cp 文件)
    pub fn to_hex_dump(&self) -> String {
        let bytes = self.serialize();
        let mut buf = String::new();
        for (i, chunk) in bytes.chunks(8).enumerate() {
            buf.push_str(&format!("{:04X}: ", i * 8));
            for b in chunk {
                buf.push_str(&format!("{:02X} ", b));
            }
            buf.push('\n');
        }
        buf
    }

    pub fn len(&self) -> usize { self.instructions.len() }
    pub fn is_empty(&self) -> bool { self.instructions.is_empty() }
}

impl Default for ContainerProgram {
    fn default() -> Self { Self::new() }
}

/// 活码编译器: 将输入字符序列编译为容器程序
pub struct LiveCompiler {
    program: ContainerProgram,
}

impl LiveCompiler {
    pub fn new() -> Self {
        Self { program: ContainerProgram::new() }
    }

    /// 从一个字符的笔画生成容器操作序列
    pub fn compile_char(&mut self, ch: char, strokes: &[u8]) {
        self.program.source_chars.push(ch);

        // 每条笔画 → Absorb 操作
        for (i, &stroke) in strokes.iter().enumerate() {
            let container_idx = stroke % 16;
            self.program.emit(OpCode::Absorb, container_idx);
            if i > 0 && i % 2 == 0 {
                self.program.emit(OpCode::Nest, container_idx);
            }
        }

        // 字符结束 → Emit 产出当前 cell
        self.program.emit(OpCode::Emit, 0);
    }

    /// 编译一段文本 (多字累积)
    pub fn compile_text(&mut self, text: &str, stroke_lookup: impl Fn(char) -> Option<Vec<u8>>) {
        for ch in text.chars() {
            if let Some(strokes) = stroke_lookup(ch) {
                self.compile_char(ch, &strokes);
            }
        }
    }

    /// 结束编译 (添加 Halt)
    pub fn finalize(mut self, final_os: ContainerOS) -> ContainerProgram {
        self.program.emit(OpCode::Halt, 0);
        self.program.final_state = Some(final_os);
        self.program
    }

    /// 获取当前程序 (不消耗 self)
    pub fn program(&self) -> &ContainerProgram {
        &self.program
    }
}

impl Default for LiveCompiler {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod compiler_tests {
    use super::*;

    #[test]
    fn test_serialize_roundtrip() {
        let mut prog = ContainerProgram::new();
        prog.source_chars = "囝".to_string();
        prog.entry_point = 0;
        prog.emit(OpCode::Absorb, 0);
        prog.emit(OpCode::Absorb, 1);
        prog.emit(OpCode::Emit, 0);
        prog.emit(OpCode::Halt, 0);

        let bytes = prog.serialize();
        let prog2 = ContainerProgram::deserialize(&bytes).unwrap();
        assert_eq!(prog2.source_chars, "囝");
        assert_eq!(prog2.instructions.len(), 4);
    }

    #[test]
    fn test_live_compiler() {
        let mut comp = LiveCompiler::new();

        let strokes: std::collections::HashMap<char, Vec<u8>> = [
            ('囝', vec![1, 6, 0, 1, 0, 9]),
            ('共', vec![0, 1, 2, 3, 0, 4]),
        ].into_iter().collect();

        comp.compile_text("囝共", |ch| strokes.get(&ch).cloned());

        let prog = comp.finalize(ContainerOS::new());
        assert!(prog.len() > 0);
        assert_eq!(prog.source_chars, "囝共");
    }
}
