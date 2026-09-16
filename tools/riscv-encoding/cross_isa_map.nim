## 跨 ISA 对齐映射 — 同一语义指令 → 统一字体符号

import std/[sequtils]

type
  SemKind* = enum
    semAdd, semSub, semAnd, semOr, semXor,
    semSll, semSrl, semSra, semSlt,
    semLoad, semStore, semBranchEq, semBranchNe,
    semJump, semJumpReg, semLui

  IsaKind* = enum
    isaRiscV, isaArm64, IsaX86_64, IsaMips

  MachineEncoding* = object
    isa*: IsaKind
    sem*: SemKind
    machine_hex*: string
    asm_text*: string

const SEM_SYM_BASE* = 0xF100'u32

const SEM_SYMBOLS: array[SemKind, uint32] = [
  SEM_SYM_BASE + 0,   SEM_SYM_BASE + 1,   SEM_SYM_BASE + 2,   SEM_SYM_BASE + 3,
  SEM_SYM_BASE + 4,   SEM_SYM_BASE + 5,   SEM_SYM_BASE + 6,   SEM_SYM_BASE + 7,
  SEM_SYM_BASE + 8,   SEM_SYM_BASE + 9,   SEM_SYM_BASE + 10,  SEM_SYM_BASE + 11,
  SEM_SYM_BASE + 12,  SEM_SYM_BASE + 13,  SEM_SYM_BASE + 14,  SEM_SYM_BASE + 15,
]

proc build_cross_isa_map*(): seq[MachineEncoding] =
  result = @[]
  # RISC-V
  result.add(MachineEncoding(isa: isaRiscV, sem: semAdd,       machine_hex: "00208033", asm_text: "add x1, x1, x2"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semSub,       machine_hex: "40208033", asm_text: "sub x1, x1, x2"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semAnd,       machine_hex: "00207033", asm_text: "and x1, x1, x2"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semOr,        machine_hex: "00206033", asm_text: "or x1, x1, x2"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semXor,       machine_hex: "00204033", asm_text: "xor x1, x1, x2"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semSll,       machine_hex: "00201033", asm_text: "sll x1, x1, x2"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semSrl,       machine_hex: "00205033", asm_text: "srl x1, x1, x2"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semSra,       machine_hex: "40205033", asm_text: "sra x1, x1, x2"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semLoad,      machine_hex: "0080A003", asm_text: "lw x1, 8(x2)"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semStore,     machine_hex: "0020A423", asm_text: "sw x1, 8(x2)"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semBranchEq,  machine_hex: "00200863", asm_text: "beq x1, x2, +16"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semBranchNe,  machine_hex: "00201863", asm_text: "bne x1, x2, +16"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semJump,      machine_hex: "0080006F", asm_text: "jal x0, +8"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semJumpReg,   machine_hex: "00008067", asm_text: "jalr x0, 0(x1)"))
  result.add(MachineEncoding(isa: isaRiscV, sem: semLui,       machine_hex: "12345037", asm_text: "lui x1, 0x12345"))

  # ARM64
  result.add(MachineEncoding(isa: isaArm64, sem: semAdd,       machine_hex: "8B020020", asm_text: "add x0, x1, x2"))
  result.add(MachineEncoding(isa: isaArm64, sem: semSub,       machine_hex: "CB020020", asm_text: "sub x0, x1, x2"))
  result.add(MachineEncoding(isa: isaArm64, sem: semAnd,       machine_hex: "8A020020", asm_text: "and x0, x1, x2"))
  result.add(MachineEncoding(isa: isaArm64, sem: semOr,        machine_hex: "AA020020", asm_text: "orr x0, x1, x2"))
  result.add(MachineEncoding(isa: isaArm64, sem: semXor,       machine_hex: "CA020020", asm_text: "eor x0, x1, x2"))
  result.add(MachineEncoding(isa: isaArm64, sem: semLoad,      machine_hex: "F9400420", asm_text: "ldr x0, [x1, #8]"))
  result.add(MachineEncoding(isa: isaArm64, sem: semStore,     machine_hex: "F9000420", asm_text: "str x0, [x1, #8]"))
  result.add(MachineEncoding(isa: isaArm64, sem: semBranchEq,  machine_hex: "04000054", asm_text: "b.eq +8"))

  # x86-64
  result.add(MachineEncoding(isa: IsaX86_64, sem: semAdd,      machine_hex: "01D8", asm_text: "add eax, ebx"))
  result.add(MachineEncoding(isa: IsaX86_64, sem: semSub,      machine_hex: "29D8", asm_text: "sub eax, ebx"))
  result.add(MachineEncoding(isa: IsaX86_64, sem: semAnd,      machine_hex: "21D8", asm_text: "and eax, ebx"))
  result.add(MachineEncoding(isa: IsaX86_64, sem: semOr,       machine_hex: "09D8", asm_text: "or eax, ebx"))
  result.add(MachineEncoding(isa: IsaX86_64, sem: semXor,      machine_hex: "31D8", asm_text: "xor eax, ebx"))

  # MIPS
  result.add(MachineEncoding(isa: IsaMips, sem: semAdd,        machine_hex: "00221020", asm_text: "add $2, $1, $1"))
  result.add(MachineEncoding(isa: IsaMips, sem: semSub,        machine_hex: "00221022", asm_text: "sub $2, $1, $1"))
  result.add(MachineEncoding(isa: IsaMips, sem: semLoad,       machine_hex: "8C220008", asm_text: "lw $2, 8($1)"))
  result.add(MachineEncoding(isa: IsaMips, sem: semStore,      machine_hex: "AC220008", asm_text: "sw $2, 8($1)"))

proc sem_to_sym*(s: SemKind): uint32 = SEM_SYMBOLS[s]

proc find_by_sem*(encs: seq[MachineEncoding], s: SemKind): seq[MachineEncoding] =
  for e in encs:
    if e.sem == s: result.add(e)

proc find_by_isa*(encs: seq[MachineEncoding], isa: IsaKind): seq[MachineEncoding] =
  for e in encs:
    if e.isa == isa: result.add(e)
