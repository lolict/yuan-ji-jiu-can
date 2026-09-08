// ══════════════════════════════════════════════════════════════════════
// cross_isa — 跨平台语义词汇统一架构
// 容器即字体，字体即时间
// 
// 语义ID层作为统一接口：
//   LOAD/STORE/ADD/SUB/AND/OR/XOR/JMP/BNE/CALL/RET/CMP/MOV
//   ↕ 各自映射到 x86-64/ARM64/RISC-V/MIPS32 原生机器码
//
// 三维坐标 [X:8][Y:16][Z:8] = 32位容器地址
//   X = 语义ID      Y = 操作数编码      Z = 时间窗口/相位对齐
//
// 消灭运行时：所有行为编译时预编排到时间表，执行时只取不解
// ══════════════════════════════════════════════════════════════════════

pub mod vocabulary;
pub mod coordinate;
pub mod fridge;
pub mod compose;

pub use vocabulary::*;
pub use coordinate::*;
pub use fridge::*;
pub use compose::*;
