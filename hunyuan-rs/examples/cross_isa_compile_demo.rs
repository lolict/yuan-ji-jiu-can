// ══════════════════════════════════════════════════════════════════════
// cross_isa_compile_demo — 积木编译端到端演示
//
// 完整流水线: 语义句子 → 词汇替换 → 平台编址 → 裁决 → 输出
//
// 运行: cargo run --example cross_isa_compile_demo
//
// 身份落款: 刘楚恬和我
// ══════════════════════════════════════════════════════════════════════

use mqf::cross_isa::*;

fn main() {
    println!("═══════════════════════════════════════════════");
    println!("  MCODE 积木编译 · 跨 ISA 端到端演示");
    println!("═══════════════════════════════════════════════\n");

    // ── 阶段 1: 构建语义句子 ──────────────────────
    let mut sentence = Sentence::new();
    sentence.append(VocabWord::new(
        SemanticId::Mov,
        vec![Operand::Reg(1), Operand::Imm(10)],
        Platform::RiscV,
    ));
    sentence.append(VocabWord::new(
        SemanticId::Mov,
        vec![Operand::Reg(2), Operand::Imm(20)],
        Platform::RiscV,
    ));
    sentence.append(VocabWord::new(
        SemanticId::Add,
        vec![Operand::Reg(3), Operand::Reg(1), Operand::Reg(2)],
        Platform::RiscV,
    ));

    println!("阶段 1: 句子构建");
    println!("  词汇数: {}", sentence.len());
    for (i, w) in sentence.words.iter().enumerate() {
        println!("    [{}] {:?} 操作数={:?} 编码={:02X?}",
            i, w.semantic_id, w.operands, w.native_encoding);
    }

    // ── 阶段 2: 坐标映射 ──────────────────────────
    println!("\n阶段 2: 三维坐标编址 [X语义:Y操作数:Z相位]");
    for (i, w) in sentence.words.iter().enumerate() {
        let coord = Coord3D::new(
            w.semantic_id as u8,
            w.operands.first().map(|o| o.imm().unwrap_or(0) as u16).unwrap_or(0),
            (i * 3) as u8,
        );
        let addr = coord.to_address();
        println!("  [{}] {:?} -> 0x{:08X} (X={} Y={} Z={})",
            i, w.semantic_id, addr, coord.x(), coord.y(), coord.z());
    }

    // ── 阶段 3: 裁决验证 ──────────────────────────
    println!("\n阶段 3: 冰箱裁决 (三元见证)");
    let judge = TriuneJudge;
    let mut packed = 0usize;
    let mut admitted = 0usize;
    for (i, w) in sentence.words.iter().enumerate() {
        let coord = Coord3D::new(
            w.semantic_id as u8,
            w.operands.first().map(|o| o.imm().unwrap_or(0) as u16).unwrap_or(0),
            (i * 3) as u8,
        );
        let pack = ContainerPack::new((i + 1) as u64, coord, 0x01, w.native_encoding.clone());
        let verdict = judge.judge(&pack, coord.z());
        let tag = match verdict {
            Verdict::Admit => { admitted += 1; "准入" }
            Verdict::Reject => "拒绝",
            Verdict::Wait => "等待",
        };
        let w_ok = pack.verify_witness();
        println!("  [{}] pack_id={} witness_valid={} verdict={}",
            i, pack.pack_id, w_ok, tag);
        packed += 1;
    }
    println!("  总计: {} packs, {} 准入", packed, admitted);

    // ── 阶段 4: 多平台原生码输出 ──────────────────
    println!("\n阶段 4: 多平台原生编码输出");

    let riscv_bytes = sentence.native_bytes(Platform::RiscV);
    println!("  RISC-V ({} bytes):", riscv_bytes.len());
    print_hex(&riscv_bytes, "    ");

    let mut arm_sentence = Sentence::new();
    arm_sentence.append(VocabWord::new(
        SemanticId::Mov,
        vec![Operand::Reg(1), Operand::Imm(10)],
        Platform::ARM64,
    ));
    arm_sentence.append(VocabWord::new(
        SemanticId::Mov,
        vec![Operand::Reg(2), Operand::Imm(20)],
        Platform::ARM64,
    ));
    arm_sentence.append(VocabWord::new(
        SemanticId::Add,
        vec![Operand::Reg(3), Operand::Reg(1), Operand::Reg(2)],
        Platform::ARM64,
    ));
    let arm_bytes = arm_sentence.native_bytes(Platform::ARM64);
    println!("  ARM64 ({} bytes):", arm_bytes.len());
    print_hex(&arm_bytes, "    ");

    let mut x64_sentence = Sentence::new();
    x64_sentence.append(VocabWord::new(
        SemanticId::Mov,
        vec![Operand::Reg(1), Operand::Imm(10)],
        Platform::X86_64,
    ));
    x64_sentence.append(VocabWord::new(
        SemanticId::Mov,
        vec![Operand::Reg(2), Operand::Imm(20)],
        Platform::X86_64,
    ));
    x64_sentence.append(VocabWord::new(
        SemanticId::Add,
        vec![Operand::Reg(0), Operand::Reg(0), Operand::Reg(3)],
        Platform::X86_64,
    ));
    let x64_bytes = x64_sentence.native_bytes(Platform::X86_64);
    println!("  x86-64 ({} bytes):", x64_bytes.len());
    print_hex(&x64_bytes, "    ");

    let mut mips_sentence = Sentence::new();
    mips_sentence.append(VocabWord::new(
        SemanticId::Mov,
        vec![Operand::Reg(1), Operand::Imm(10)],
        Platform::MIPS32,
    ));
    mips_sentence.append(VocabWord::new(
        SemanticId::Mov,
        vec![Operand::Reg(2), Operand::Imm(20)],
        Platform::MIPS32,
    ));
    mips_sentence.append(VocabWord::new(
        SemanticId::Add,
        vec![Operand::Reg(8), Operand::Reg(9), Operand::Reg(10)],
        Platform::MIPS32,
    ));
    let mips_bytes = mips_sentence.native_bytes(Platform::MIPS32);
    println!("  MIPS32 ({} bytes):", mips_bytes.len());
    print_hex(&mips_bytes, "    ");

    // ── 阶段 5: 统计总结 ──────────────────────────
    println!("\n阶段 5: 编译总结");
    println!("  语义句子: {} 条词汇", sentence.len());
    println!("  RISC-V : {:3} bytes", riscv_bytes.len());
    println!("  ARM64  : {:3} bytes", arm_bytes.len());
    println!("  x86-64 : {:3} bytes", x64_bytes.len());
    println!("  MIPS32 : {:3} bytes", mips_bytes.len());

    println!("\n═══════════════════════════════════════════════");
    println!("  积木编译完成");
    println!("═══════════════════════════════════════════════");
}

fn print_hex(bytes: &[u8], prefix: &str) {
    for (i, chunk) in bytes.chunks(4).enumerate() {
        let hex: String = chunk.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" ");
        println!("    {}[{:2}] {}", prefix, i * 4, hex);
    }
}
