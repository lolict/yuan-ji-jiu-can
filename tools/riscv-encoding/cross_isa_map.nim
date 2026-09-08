## 跨 ISA 对齐映射
## 同一语义指令 → 统一字体符号 (机器码不同, 符号相同)
##
## 核心思想:
##   ARM64 add x0, x1, x2 → 机器码 0x8B020020 → 字体符号 U+F100
##   RISC-V add x1, x1, x2 → 机器码 0x00208033 → 字体符号 U+F100
##   x86-64 add eax, ebx  → 机器码 0x01D8      → 字体符号 U+F100
##
## 字体文件跨平台统一: 同一个 glyph, 按不同 ISA 渲染不同机器码

import std/[tables, strutils, sequtils]
import riscv_codec

# ── 跨 ISA 指令语义 ID ────────────────────────────────────
type
  SemKind* = enum        # 指令语义 (跨 ISA 统一)
    semAdd, semSub, semAnd, semOr, semXor,
    semSll, semSrl, semSra, semSlt,
    semLoad, semStore, semBranchEq, semBranchNe,
    semJump, semJumpReg, semLui

  IsaKind* = enum
    isaRiscV, isaArm64, IsaX86_64, IsaMips

  MachineEncoding* = object
    isa*: IsaKind
    sem*: SemKind
    machine_hex*: string    # 十六进制机器码
    asm_text*: string       # 典型汇编文本

# ── 统一字体符号区 (全局语义符号表) ──────────────────────
const
  SEM_SYM_BASE* = 0xF100'u32

  SEM_SYMBOLS: array[SemKind, uint32] = [
    SEM_SYM_BASE + 0,   # semAdd      U+F100
    SEM_SYM_BASE + 1,   # semSub      U+F101
    SEM_SYM_BASE + 2,   # semAnd      U+F102
    SEM_SYM_BASE + 3,   # semOr       U+F103
    SEM_SYM_BASE + 4,   # semXor      U+F104
    SEM_SYM_BASE + 5,   # semSll      U+F105
    SEM_SYM_BASE + 6,   # semSrl      U+F106
    SEM_SYM_BASE + 7,   # semSra      U+F107
    SEM_SYM_BASE + 8,   # semSlt      U+F108
    SEM_SYM_BASE + 9,   # semLoad     U+F109
    SEM_SYM_BASE + 10,  # semStore    U+F10A
    SEM_SYM_BASE + 11,  # semBranchEq U+F10B
    SEM_SYM_BASE + 12,  # semBranchNe U+F10C
    SEM_SYM_BASE + 13,  # semJump     U+F10D
    SEM_SYM_BASE + 14,  # semJumpReg  U+F10E
    SEM_SYM_BASE + 15,  # semLui      U+F10F
  ]

# ── ISA 特定实例 → 语义符号 ───────────────────────────────
proc build_cross_isa_map*(): seq[MachineEncoding] =
  ## 构建跨 ISA 对齐表: 同一语义在不同 ISA 下的机器码
  result = @[]

  # RISC-V (已经由 riscv_codec.nim 验证)
  result.add(MachineEncoding(isa: isaRiscV, sem: semAdd,
    machine_hex: "00208033", asm_text: "add x1, x1, x2"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semSub,
    machine_hex: "40208033", asm_text: "sub x1, x1, x2"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semAnd,
    machine_hex: "00207033", asm_text: "and x1, x1, x2"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semOr,
    machine_hex: "00206033", asm_text: "or x1, x1, x2"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semXor,
    machine_hex: "00204033", asm_text: "xor x1, x1, x2"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semSll,
    machine_hex: "00201033", asm_text: "sll x1, x1, x2"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semSrl,
    machine_hex: "00205033", asm_text: "srl x1, x1, x2"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semSra,
    machine_hex: "40205033", asm_text: "sra x1, x1, x2"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semLoad,
    machine_hex: "0080A003", asm_text: "lw x1, 8(x2)"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semStore,
    machine_hex: "0020A423", asm_text: "sw x1, 8(x2)"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semBranchEq,
    machine_hex: "00200863", asm_text: "beq x1, x2, +16"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semBranchNe,
    machine_hex: "00201863", asm_text: "bne x1, x2, +16"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semJump,
    machine_hex: "0080006F", asm_text: "jal x0, +8"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semJumpReg,
    machine_hex: "00008067", asm_text: "jalr x0, 0(x1)"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semLui,
    machine_hex: "12345037", asm_text: "lui x1, 0x12345"))

  # ARM64 (AArch64) — 完整 15 条
  result.add(MachineEncoding(isa: isaArm64, sem: semAdd,
    machine_hex: "8B020020", asm_text: "add x0, x1, x2"))
  result.add(MachineEncoding(isa: isaArm64, sem: semSub,
    machine_hex: "CB020020", asm_text: "sub x0, x1, x2"))
  result.add(MachineEncoding(isa: isaArm64, sem: semAnd,
    machine_hex: "8A020020", asm_text: "and x0, x1, x2"))
  result.add(MachineEncoding(isa: isaArm64, sem: semOr,
    machine_hex: "AA020020", asm_text: "orr x0, x1, x2"))
  result.add(MachineEncoding(isa: isaArm64, sem: semXor,
    machine_hex: "CA020020", asm_text: "eor x0, x1, x2"))
  result.add(MachineEncoding(isa: isaArm64, sem: semSll,
    machine_hex: "8B22028B", asm_text: "lsl x0, x1, x2"))
  result.add(MachineEncoding(isa: isaArm64, sem: semSrl,
    machine_hex: "8B22428B", asm_text: "lsr x0, x1, x2"))
  result.add(MachineEncoding(isa: isaArm64, sem: semSra,
    machine_hex: "8B22828B", asm_text: "asr x0, x1, x2"))
  result.add(MachineEncoding(isa: isaArm64, sem: semSlt,
    machine_hex: "EB0202EB", asm_text: "cmp x1, x2 + csel"))
  result.add(MachineEncoding(isa: isaArm64, sem: semLoad,
    machine_hex: "F9400420", asm_text: "ldr x0, [x1, #8]"))
  result.add(MachineEncoding(isa: isaArm64, sem: semStore,
    machine_hex: "F9000420", asm_text: "str x0, [x1, #8]"))
  result.add(MachineEncoding(isa: isaArm64, sem: semBranchEq,
    machine_hex: "04000054", asm_text: "b.eq +8"))
  result.add(MachineEncoding(isa: isaArm64, sem: semBranchNe,
    machine_hex: "05000054", asm_text: "b.ne +8"))
  result.add(MachineEncoding(isa: isaArm64, sem: semJump,
    machine_hex: "00000014", asm_text: "b +8"))
  result.add(MachineEncoding(isa: isaArm64, sem: semJumpReg,
    machine_hex: "1FF03FD6", asm_text: "br x1"))
  result.add(MachineEncoding(isa: isaArm64, sem: semLui,
    machine_hex: "10000058", asm_text: "ldr x0, =0x12345"))

  # x86-64 (变长) — 完整 15 条
  result.add(MachineEncoding(isa: IsaX86_64, sem: semAdd,
    machine_hex: "01D8", asm_text: "add eax, ebx"))
  result.add(MachineEncoding(isa: IsaX86_64, sem: semSub,
    machine_hex: "29D8", asm_text: "sub eax, ebx"))
  result.add(MachineEncoding(isa: IsaX86_64, sem: semAnd,
    machine_hex: "21D8", asm_text: "and eax, ebx"))
  result.add(MachineEncoding(isa: IsaX86_64, sem: semOr,
    machine_hex: "09D8", asm_text: "or eax, ebx"))
  result.add(MachineEncoding(isa: IsaX86_64, sem: semXor,
    machine_hex: "31D8", asm_text: "xor eax, ebx"))
  result.add(MachineEncoding(isa: IsaX86_64, sem: semSll,
    machine_hex: "D3E0", asm_text: "shl eax, cl"))
  result.add(MachineEncoding(isa: IsaX86_64, sem: semSrl,
    machine_hex: "D3E8", asm_text: "shr eax, cl"))
  result.add(MachineEncoding(isa: IsaX86_64, sem: semSra,
    machine_hex: "D3F8", asm_text: "sar eax, cl"))
  result.add(MachineEncoding(isa: IsaX86_64, sem: semSlt,
    machine_hex: "39D87C03", asm_text: "cmp eax,ebx + setl"))
  result.add(MachineEncoding(isa: IsaX86_64, sem: semLoad,
    machine_hex: "8B4008", asm_text: "mov eax, [rax+8]"))
  result.add(MachineEncoding(isa: IsaX86_64, sem: semStore,
    machine_hex: "894008", asm_text: "mov [rax+8], eax"))
  result.add(MachineEncoding(isa: IsaX86_64, sem: semBranchEq,
    machine_hex: "7408", asm_text: "je +8"))
  result.add(MachineEncoding(isa: IsaX86_64, sem: semBranchNe,
    machine_hex: "7508", asm_text: "jne +8"))
  result.add(MachineEncoding(isa: IsaX86_64, sem: semJump,
    machine_hex: "EB08", asm_text: "jmp +8"))
  result.add(MachineEncoding(isa: IsaX86_64, sem: semJumpReg,
    machine_hex: "FFE0", asm_text: "jmp rax"))
  result.add(MachineEncoding(isa: IsaX86_64, sem: semLui,
    machine_hex: "B8", asm_text: "mov eax, imm32"))

  # MIPS — 完整 15 条
  result.add(MachineEncoding(isa: IsaMips, sem: semAdd,
    machine_hex: "00221020", asm_text: "add $2, $1, $1"))
  result.add(MachineEncoding(isa: IsaMips, sem: semSub,
    machine_hex: "00221022", asm_text: "sub $2, $1, $1"))
  result.add(MachineEncoding(isa: IsaMips, sem: semAnd,
    machine_hex: "00221024", asm_text: "and $2, $1, $1"))
  result.add(MachineEncoding(isa: IsaMips, sem: semOr,
    machine_hex: "00221025", asm_text: "or $2, $1, $1"))
  result.add(MachineEncoding(isa: IsaMips, sem: semXor,
    machine_hex: "00221026", asm_text: "xor $2, $1, $1"))
  result.add(MachineEncoding(isa: IsaMips, sem: semSll,
    machine_hex: "00001040", asm_text: "sll $2, $1, 2"))
  result.add(MachineEncoding(isa: IsaMips, sem: semSrl,
    machine_hex: "00001042", asm_text: "srl $2, $1, 2"))
  result.add(MachineEncoding(isa: IsaMips, sem: semSra,
    machine_hex: "00001043", asm_text: "sra $2, $1, 2"))
  result.add(MachineEncoding(isa: IsaMips, sem: semSlt,
    machine_hex: "0022102A", asm_text: "slt $2, $1, $1"))
  result.add(MachineEncoding(isa: IsaMips, sem: semLoad,
    machine_hex: "8C220008", asm_text: "lw $2, 8($1)"))
  result.add(MachineEncoding(isa: IsaMips, sem: semStore,
    machine_hex: "AC220008", asm_text: "sw $2, 8($1)"))
  result.add(MachineEncoding(isa: IsaMips, sem: semBranchEq,
    machine_hex: "10210008", asm_text: "beq $1, $1, +8"))
  result.add(MachineEncoding(isa: IsaMips, sem: semBranchNe,
    machine_hex: "14210008", asm_text: "bne $1, $1, +8"))
  result.add(MachineEncoding(isa: IsaMips, sem: semJump,
    machine_hex: "08000008", asm_text: "j 0x000008"))
  result.add(MachineEncoding(isa: IsaMips, sem: semJumpReg,
    machine_hex: "00000808", asm_text: "jr $1"))
  result.add(MachineEncoding(isa: IsaMips, sem: semLui,
    machine_hex: "3C021234", asm_text: "lui $2, 0x1234"))

# ── 查询接口 ───────────────────────────────────────────────
proc sem_to_sym*(s: SemKind): uint32 = SEM_SYMBOLS[s]

proc find_by_sem*(encs: seq[MachineEncoding], s: SemKind): seq[MachineEncoding] =
  for e in encs:
    if e.sem == s: result.add(e)

proc find_by_isa*(encs: seq[MachineEncoding], isa: IsaKind): seq[MachineEncoding] =
  for e in encs:
    if e.isa == isa: result.add(e)

# ════════════════════════════════════════════════════════════
# 演示
# ════════════════════════════════════════════════════════════
when is_main_module:
  echo "═══ 跨 ISA 对齐: 同一语义 → 统一字体符号 ═══"
  echo ""

  let encs = build_cross_isa_map()
  echo "对齐表条目: ", encs.len
  echo ""

  # 以 add 为例: 4 种 ISA, 1 个字体符号
  echo "【语义 symAdd (U+F100) 在 4 个 ISA 下的编码】"
  echo "  字体符号: U+", toHex(sem_to_sym(semAdd)).toUpperAscii
  echo "  ┌──────────┬──────────────┬────────────────────┐"
  echo "  │   ISA    │   机器码     │     汇编示例       │"
  echo "  ├──────────┼──────────────┼────────────────────┤"
  for e in find_by_sem(encs, semAdd):
    echo "  │ ", ($e.isa).alignLeft(8), " │ ", e.machine_hex.alignLeft(12),
         " │ ", e.asm_text.alignLeft(18), " │"
  echo "  └──────────┴──────────────┴────────────────────┘"
  echo ""

  # RISC-V 编码验证
  echo "【RISC-V 编码交叉验证】"
  var rv_encs = find_by_isa(encs, isaRiscV)
  echo "  RISC-V 条目: ", rv_encs.len
  for i in 0 ..< INSTR_TABLE.len:
    if INSTR_TABLE[i].name == "add":
      let stub = encode_instr_stub(i)
      echo "  add 骨架机器码: 0x", toHex(stub).toUpperAscii, " (对齐表: 00208033)"
      break
  echo ""

  # 全表概览
  echo "【全语义 × 全 ISA 概览】"
  for sem in SemKind:
    let matches = find_by_sem(encs, sem)
    if matches.len >= 2:
      echo "  ", ($sem).alignLeft(8), " U+", toHex(sem_to_sym(sem)).toUpperAscii, ": ",
        matches.len, " 个 ISA → ", map(matches, proc (m: MachineEncoding): string = ($m.isa)[3..^1]).join(", ")
  echo ""
  echo "═══ 完成 ═══"
