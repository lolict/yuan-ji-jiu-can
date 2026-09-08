## 跨 ISA 全语义映射 v2 (62 语义 ID)
## 与 full_alignment.md / mc_align.nim ALIGN_TABLE 对齐

import std/[strutils, options]

type
  SemKind* = enum
    semMovReg, semMovZero, semMovImm32, semMovImm64,
    semPush, semPop,
    semAdd, semSub, semAnd, semOr, semXor,
    semAddI, semAndI, semOrI, semXorI, semSltI,
    semSll, semSrl, semSra,
    semSllI, semSrlI, semSraI,
    semSlt, semSltU,
    semBeq, semBne, semBlt, semBge,
    semJump, semCall, semRet, semJumpReg,
    semLb, semLh, semLw, semLbu, semLhu, semLd,
    semSb, semSh, semSw, semSd,
    semLui, semAuipc,
    semLr, semSc, semAmoswap, semAmoadd, semFence,
    semEcall, semEbreak, semSysret,
    semMul, semMulH, semDiv, semDivU, semRem, semRemU,
    semCmovEq, semCmovNe, semCmovLt, semCmovGt,
    semNop, semLea, semSyscall, semSysenter,
    semInt3, semInt80, semIret, semHalt,
    semAlign4, semAlign8,

  IsaKind* = enum
    isaRiscV, isaArm64, IsaX86_64, IsaMips, IsaMtk

  CrossEntry* = object
    sem*: SemKind
    rv*: string
    arm*: string
    x64*: string
    mips*: string

  MachineEncoding* = object
    isa*: IsaKind
    sem*: SemKind
    machine_hex*: string
    asm_text*: string
    bit_len*: int

const
  SEM_SYM_BASE* = 0xF100'u32
  SEM_SYMBOLS*: array[SemKind, uint32] = block:
    var arr: array[SemKind, uint32]
    var i = 0
    for e in SemKind:
      arr[e] = SEM_SYM_BASE + uint32(i)
      inc i
    arr

const SEM_NAMES*: array[SemKind, string] = [
    "MOV_REG", "MOV_ZERO", "MOV_IMM32", "MOV_IMM64",
    "PUSH", "POP",
    "ADD", "SUB", "AND", "OR", "XOR",
    "ADDI", "ANDI", "ORI", "XORI", "SLTI",
    "SLL", "SRL", "SRA",
    "SLLI", "SRLI", "SRAI",
    "SLT", "SLTU",
    "BEQ", "BNE", "BLT", "BGE",
    "JMP", "CALL", "RET", "JR",
    "LB", "LH", "LW", "LBU", "LHU", "LD",
    "SB", "SH", "SW", "SD",
    "LUI", "AUIPC",
    "LR", "SC", "AMOSWAP", "AMOADD", "FENCE",
    "ECALL", "EBREAK", "SYSRET",
    "MUL", "MULH", "DIV", "DIVU", "REM", "REMU",
    "CMOVE", "CMOVNE", "CMOVLT", "CMOVGT",
    "NOP", "LEA", "SYSCALL", "SYSENTER",
    "INT3", "INT80", "IRET", "HALT",
    "ALIGN4", "ALIGN8"
]

const CROSS_ISA_MAP*: seq[CrossEntry] = @[
    CrossEntry(sem: semMovReg,   rv: "00008013",  arm: "AA0003E0", x64: "48 89 D8",       mips: "00 22 10 21"),
    CrossEntry(sem: semMovZero,  rv: "00000013",  arm: "E003804A", x64: "31 C0",         mips: "00 00 10 21"),
    CrossEntry(sem: semMovImm32, rv: "000x_xxxxx",arm: "D2800000", x64: "48 C7 C0 xx..", mips: "3C 02 xx xx"),
    CrossEntry(sem: semAdd,      rv: "00208033",  arm: "8B020020", x64: "48 01 D8",       mips: "00 22 10 21"),
    CrossEntry(sem: semSub,      rv: "40208033",  arm: "CB020020", x64: "48 29 D8",       mips: "00 22 10 23"),
    CrossEntry(sem: semAnd,      rv: "00207033",  arm: "8A020020", x64: "48 21 D8",       mips: "00 22 10 24"),
    CrossEntry(sem: semOr,       rv: "00206033",  arm: "AA020020", x64: "48 09 D8",       mips: "00 22 10 25"),
    CrossEntry(sem: semXor,      rv: "00204033",  arm: "CA020020", x64: "48 31 D8",       mips: "00 22 10 26"),
    CrossEntry(sem: semAddI,     rv: "0FF08013",  arm: "9103FFC0", x64: "05 FF 00 00 00", mips: "24 42 00 FF"),
    CrossEntry(sem: semAndI,     rv: "0xx7x_13",  arm: "8A0600+imm", x64: "25 imm32",     mips: "30 42 imm"),
    CrossEntry(sem: semOrI,      rv: "0xx6x_13",  arm: "AA0600+imm", x64: "0D imm32",     mips: "34 42 imm"),
    CrossEntry(sem: semXorI,     rv: "0xx4x_13",  arm: "CA0600+imm", x64: "35 imm32",     mips: "38 42 imm"),
    CrossEntry(sem: semSltI,     rv: "0xx2x_13",  arm: "cmp+cset",   x64: "cmp+setl",     mips: "28 42 imm"),
    CrossEntry(sem: semSll,      rv: "00209033",  arm: "9AC22020", x64: "48 D3 E0",        mips: "00 21 10 04"),
    CrossEntry(sem: semSrl,      rv: "0020D033",  arm: "9AC22420", x64: "48 D3 E8",        mips: "00 21 10 06"),
    CrossEntry(sem: semSra,      rv: "4020D033",  arm: "9AC22820", x64: "48 D3 F8",        mips: "00 21 10 07"),
    CrossEntry(sem: semSllI,     rv: "00309013",  arm: "D37FF820", x64: "48 C1 E0 03",     mips: "00 02 10 C0"),
    CrossEntry(sem: semSrlI,     rv: "0030D013",  arm: "D343FC20", x64: "48 C1 E8 03",     mips: "00 02 10 C2"),
    CrossEntry(sem: semSraI,     rv: "4030D013",  arm: "ASR_C3",   x64: "48 C1 F8 03",     mips: "00 02 10 C3"),
    CrossEntry(sem: semSlt,      rv: "0020A033",  arm: "cmp+cset",   x64: "cmp+setl",      mips: "00 44 08 2A"),
    CrossEntry(sem: semSltU,     rv: "0020B033",  arm: "cmp+csetlo",x64: "cmp+setb",      mips: "00 44 08 2B"),
    CrossEntry(sem: semBeq,      rv: "1100011_00",arm: "54000400", x64: "74 rel8/0F 84",  mips: "10 xx xx xx"),
    CrossEntry(sem: semBne,      rv: "1100011_01",arm: "54000401", x64: "75 rel8/0F 85",  mips: "14 xx xx xx"),
    CrossEntry(sem: semBlt,      rv: "1100011_04",arm: "5400040B", x64: "7C rel8/0F 8C",  mips: "slt+beq复合"),
    CrossEntry(sem: semBge,      rv: "1100011_05",arm: "5400040A", x64: "7D rel8/0F 8D",  mips: "04 xx xx xx"),
    CrossEntry(sem: semJump,     rv: "xxxx_1101111",arm: "1400xxxx",x64: "E9 xx xx xx xx",mips: "08 00 xx xx"),
    CrossEntry(sem: semCall,     rv: "xxxx_1101111",arm: "9400xxxx",x64: "E8 xx xx xx xx",mips: "0C 00 xx xx"),
    CrossEntry(sem: semRet,      rv: "00008067",  arm: "D65F03C0", x64: "C3",              mips: "03 E0 00 08"),
    CrossEntry(sem: semJumpReg,  rv: "000lx_1100011",arm: "D61F03C0",x64: "FF E0",         mips: "00 00 F8 09"),
    CrossEntry(sem: semLb,       rv: "0080C003",  arm: "39400820", x64: "0F B6 03",        mips: "80 42 00 08"),
    CrossEntry(sem: semLh,       rv: "00809003",  arm: "79400820", x64: "0F B7 03",        mips: "84 42 00 08"),
    CrossEntry(sem: semLw,       rv: "0080A003",  arm: "B9400820", x64: "48 8B 03",        mips: "8C 42 00 08"),
    CrossEntry(sem: semLbu,      rv: "0080C013",  arm: "39400820_ldrB",x64: "0F B6_03zx", mips: "90 42 00 08"),
    CrossEntry(sem: semLhu,      rv: "0080D013",  arm: "79400820_ldrh",x64:"0F BF 03",     mips: "94 42 00 08"),
    CrossEntry(sem: semSb,       rv: "00110423",  arm: "39000820", x64: "88 03",           mips: "A0 42 00 08"),
    CrossEntry(sem: semSh,       rv: "00111423",  arm: "79000820", x64: "66 89 03",        mips: "A4 42 00 08"),
    CrossEntry(sem: semSw,       rv: "00112423",  arm: "B9000820", x64: "48 89 03",        mips: "AC 42 00 08"),
    CrossEntry(sem: semLui,      rv: "xxxx_0110111",arm: "D2800000",x64: "48 C7 C0 xx..", mips: "3C 02 xx xx"),
    CrossEntry(sem: semAuipc,    rv: "xxxx_0010111",arm: "—",       x64: "—",              mips: "—"),
    CrossEntry(sem: semLr,       rv: "0000B02F",  arm: "F8400C20", x64: "48 8B 03+lock",   mips: "C0 42 00 00"),
    CrossEntry(sem: semSc,       rv: "0020B02F",  arm: "F8000C20", x64: "48 0F B1 0B",     mips: "E0 42 00 00"),
    CrossEntry(sem: semAmoswap,  rv: "0020902F",  arm: "F8220C20", x64: "48 87 03",        mips: "—"),
    CrossEntry(sem: semAmoadd,   rv: "0000A02F",  arm: "复合stadd",x64: "lock add",       mips: "—"),
    CrossEntry(sem: semFence,    rv: "0000000F",  arm: "D5033F9F", x64: "0F AE F0",        mips: "00 00 00 0F"),
    CrossEntry(sem: semEcall,    rv: "00000073",  arm: "D4000001", x64: "0F 05",           mips: "00 00 00 0C"),
    CrossEntry(sem: semEbreak,   rv: "00100073",  arm: "D4200000", x64: "CC",              mips: "00 00 00 0D"),
    CrossEntry(sem: semSysret,   rv: "10200073",  arm: "D69F03E0", x64: "48 0F 07",       mips: "42 00 00 18"),
    CrossEntry(sem: semMul,      rv: "02xxx1_0110011",arm: "9B029C",x64: "48 0F AF C3",  mips: "00 44 08 2A→"),
    CrossEntry(sem: semDiv,      rv: "02xxx1_0110011",arm: "9AC20C",x64: "48 F7 FB",      mips: "00 44 08 1A"),
    CrossEntry(sem: semRem,      rv: "02xxx1_0110011",arm: "sdiv+msub",x64:"idiv+rdx",    mips: "00 44 08 1A;mfhi"),
    CrossEntry(sem: semNop,      rv: "00000013",  arm: "D503201F", x64: "90",              mips: "00 00 00 00"),
    CrossEntry(sem: semCmovEq,   rv: "—",         arm: "CSEL_EQ",x64: "48 0F 44 C3",     mips: "MOVZ"),
    CrossEntry(sem: semCmovNe,   rv: "—",         arm: "CSEL_NE",x64: "48 0F 45 C3",     mips: "MOVN"),
    CrossEntry(sem: semLea,      rv: "—",         arm: "ADD+shift",x64:"48 8D xx",       mips: "add+addu"),
    CrossEntry(sem: semInt3,     rv: "00100073",  arm: "D4200000", x64: "CC",             mips: "00 00 00 0D"),
    CrossEntry(sem: semHalt,     rv: "—",         arm: "WFI",      x64: "F4",             mips: "—"),
    CrossEntry(sem: semSyscall,  rv: "00000073",  arm: "D4000001", x64: "0F 05",          mips: "00 00 00 0C"),
  ]

const SEM_COUNT* = ord(high(SemKind)) + 1  # 62

proc semToSym*(s: SemKind): uint32 = SEM_SYMBOLS[s]

proc symToSem*(code: uint32): Option[SemKind] =
  for e in SemKind:
    if SEM_SYMBOLS[e] == code: return some(e)
  return none(SemKind)

proc getEncoding*(sem: SemKind, isa: IsaKind): string =
  for e in CROSS_ISA_MAP:
    if e.sem == sem:
      case isa
      of isaRiscV: return e.rv
      of isaArm64: return e.arm
      of IsaX86_64: return e.x64
      of IsaMips:  return e.mips
      of IsaMtk:   return e.arm
  return "—"

when isMainModule:
  import std/strformat
  echo &"语义符号总数: {SEM_COUNT} (U+{SEM_SYM_BASE:04X} → U+{SEM_SYM_BASE + uint32(SEM_COUNT-1):04X})"
  echo "前 8 项映射 (RV → x64):"
  var i = 0
  for s in SemKind:
    if i >= 8: break
    let enc = getEncoding(s, IsaX86_64)
    echo &"  {SEM_NAMES[s]:15s} U+{SEM_SYMBOLS[s]:04X} x64={enc}"
    i += 1
  echo &"\n验证 sem↔sym 双向转换:"
  let testSem = semCmovGt
  let sym = semToSym(testSem)
  let back = symToSem(sym)
  echo &"  {SEM_NAMES[testSem]} → U+{sym:04X} → {back.isSome}"
