// ══════════════════════════════════════════════════════════════════════
// isa-encode — RISC-V RV32I/RV64I/M/A 编码/解码 CLI 工具
// ══════════════════════════════════════════════════════════════════════

use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("用法: isa-encode <encode|decode|test|demo> [参数]");
        return ExitCode::from(1);
    }
    match args[1].as_str() {
        "encode" => {
            if args.len() < 3 { eprintln!("用法: isa-encode encode \"add x1, x2, x3\""); return ExitCode::from(1); }
            match mqf::isa::Insn::encode(&args[2]) {
                Some(insn) => { println!("{}", insn.hex()); }
                None => { eprintln!("编码失败: {}", args[2]); return ExitCode::from(1); }
            }
        }
        "decode" => {
            if args.len() < 3 { eprintln!("用法: isa-encode decode <hex32>"); return ExitCode::from(1); }
            let hex_str = args[2].trim_start_matches("0x");
            match u32::from_str_radix(hex_str, 16) {
                Ok(v) => println!("{}", mqf::isa::Insn(v).decode()),
                Err(e) => { eprintln!("无效 hex: {}", e); return ExitCode::from(1); }
            }
        }
        "demo" => {
            println!("═══ RISC-V RV32I/RV64I/M/A 核心指令演示 ═══");
            let demo_cases: &[(&str, &str)] = &[
                ("add x1, x2, x3",    "寄存器加法: x1 ← x2 + x3"),
                ("sub x1, x2, x3",    "寄存器减法: x1 ← x2 - x3"),
                ("and x1, x2, x3",    "按位与"),
                ("or x1, x2, x3",     "按位或"),
                ("xor x1, x2, x3",    "按位异或"),
                ("addi x1, x2, 100",  "立即数加法"),
                ("lw x5, 0(x6)",      "加载字: x5 ← Mem[x6+0]"),
                ("sw x5, 0(x6)",      "存储字: Mem[x6+0] ← x5"),
                ("beq x0, x1, 16",    "相等分支"),
                ("lui x1, 0x12345",   "加载高位立即数"),
                ("jal x1, 0",         "跳转并链接 (返回地址存 x1)"),
                ("jalr x0, 0(x1)",    "寄存器跳转 (返回)"),
                ("ecall",             "系统调用"),
                // RV64I 字宽
                ("addw x1, x2, x3",   "64位字加法: x1 ← sext(x2+x3)[31:0]"),
                ("addiw x1, x2, 42",  "立即数加法 64位字"),
                ("ld x1, 0(x2)",      "加载双字 (RV64)"),
                ("sd x1, 0(x2)",      "存储双字 (RV64)"),
                // M 扩展
                ("mul x1, x2, x3",    "乘法: x1 ← x2 * x3"),
                ("div x1, x2, x3",    "有符号除法"),
                ("remu x1, x2, x3",   "无符号取余"),
                // A 扩展
                ("lr.w x0, x1(x2)",   "原子加载预留 (32位)"),
                ("amoswap.w x0, x3, x2", "原子交换"),
            ];
            for (asm, desc) in demo_cases.iter() {
                if let Some(insn) = mqf::isa::Insn::encode(asm) {
                    println!("  {:24} → 0x{:08x}  // {}", asm, insn.0, desc);
                }
            }
        }
        "test" => {
            let mut ok = 0u32;
            let mut failed = 0u32;
            // 权威数据来自 RISC-V Unprivileged ISA 手册
            let manual_cases: &[(&str, u32)] = &[
                ("add x0, x1, x2", 0x00208033),
                ("sub x0, x1, x2", 0x40208033),
                ("and x0, x1, x2", 0x0020F033),
                ("or x0, x1, x2",  0x0020E033),
                ("xor x0, x1, x2", 0x0020C033),
                ("sll x0, x1, x2", 0x00209033),
                ("srl x0, x1, x2", 0x0020D033),
                ("sra x0, x1, x2", 0x4020D033),
                ("slt x0, x1, x2", 0x0020A033),
                ("addi x0, x1, 42", 0x02A08013),
                ("addi x0, x0, 0", 0x00000013),
                ("lw x0, 8(x1)",   0x0080A003),
                ("sw x0, 0(x1)",   0x0000A023),
                ("jal x0, 0",      0x0000006F),
                ("jalr x0, x1, 0", 0x00008067),
                ("ecall",          0x00000073),
                ("ld x1, 0(x2)",   0x00013083),
                ("sd x1, 0(x2)",   0x00113023),
                ("addiw x1, x2, 42", 0x02A1009B),
                ("addw x1, x2, x3",  0x003100BB),
            ];
            for (asm, expected) in manual_cases.iter() {
                if let Some(insn) = mqf::isa::Insn::encode(asm) {
                    if insn.0 == *expected { ok += 1; }
                    else { failed += 1; eprintln!("FAIL: {} 期望 {:08x} 得到 {:08x}", asm, expected, insn.0); }
                } else { failed += 1; eprintln!("FAIL: {} 编码失败", asm); }
            }
            // 双向 roundtrip
            for asm in &["add x1, x2, x3", "sub x4, x5, x6", "addi x5, x6, -100", "lw x3, 16(x4)", "sw x3, 32(x4)", "addiw x1, x2, 42"] {
                if let Some(insn1) = mqf::isa::Insn::encode(asm) {
                    let decoded = insn1.decode();
                    if let Some(insn2) = mqf::isa::Insn::encode(&decoded) {
                        if insn1.0 == insn2.0 { ok += 1; }
                        else { failed += 1; eprintln!("RT-MISMATCH: {} -> {} {:08x} != {:08x}", asm, decoded, insn1.0, insn2.0); }
                    } else { failed += 1; eprintln!("RT-DECODE-FAIL: {} -> {}", asm, decoded); }
                }
            }
            println!("[isa-encode test] 通过 {} 失败 {}", ok, failed);
            if failed > 0 { return ExitCode::from(1); }
        }
        _ => {
            eprintln!("未知子命令: {}", args[1]);
            return ExitCode::from(1);
        }
    }
    ExitCode::SUCCESS
}
