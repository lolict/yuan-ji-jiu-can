# ============================================================
# MCOD 表规范 · 机器码容器编码表
#
# 定义 RISC-V RV32I 核心指令的语义 ID、机器码模板和容器格式
# 共 16 条核心指令，覆盖算术/逻辑/访存/分支
# ============================================================

from dataclasses import dataclass
from enum import IntEnum
from typing import Optional

# ── ISA 平台坐标 ──
class Platform(IntEnum):
    RISCV  = 0x0001
    ARM64  = 0x0002
    X86_64 = 0x0003
    MIPS   = 0x0004

# ── 极性 ──
class Polarity(IntEnum):
    OUTSIDER = 0x00  # 外人
    NEUTRAL  = 0xFF  # 中性
    OWNER    = 0x01  # 主人

# ── 主权 ──
class Ownership(IntEnum):
    PRIVATE = 0x00
    PUBLIC  = 0x01

# ── 指令格式 ──
class InsnFormat(IntEnum):
    R = 0
    I = 1
    S = 2
    B = 3
    U = 4
    J = 5

# ── 语义 ID（跨平台统一）──
class SemanticId(IntEnum):
    ADD    = 0x0001
    SUB    = 0x0002
    AND    = 0x0003
    OR     = 0x0004
    XOR    = 0x0005
    SLT    = 0x0006
    SLTU   = 0x0007
    SLL    = 0x0008
    SRL    = 0x0009
    SRA    = 0x000A
    ADDI   = 0x000B
    LW     = 0x000C
    SW     = 0x000D
    BEQ    = 0x000E
    JAL    = 0x000F
    LUI    = 0x0010
    AUIPC  = 0x0011

# ── 容器（64 位）──
@dataclass(frozen=True)
class Container:
    """64 位容器：坐标(16) | 极性(8) | 主权(8) | 价值(32)"""
    platform: int   # 16-bit ISA 坐标
    polarity: int   # 8-bit
    ownership: int  # 8-bit
    value: int      # 32-bit 机器码本体

    def encode(self) -> int:
        """打包成 64 位整数"""
        return (
            (self.platform << 48)
            | (self.polarity << 40)
            | (self.ownership << 32)
            | self.value
        )

    @staticmethod
    def decode(cell: int) -> 'Container':
        """从 64 位整数解包"""
        return Container(
            platform=(cell >> 48) & 0xFFFF,
            polarity=(cell >> 40) & 0xFF,
            ownership=(cell >> 32) & 0xFF,
            value=cell & 0xFFFFFFFF,
        )

# ── RISC-V RV32I 指令模板 ──
RV32I_TEMPLATES = {
    # 语义ID: (格式, opcode, funct3, funct7)
    SemanticId.ADD:   (InsnFormat.R, 0b0110011, 0b000, 0b0000000),
    SemanticId.SUB:   (InsnFormat.R, 0b0110011, 0b000, 0b0100000),
    SemanticId.AND:   (InsnFormat.R, 0b0110011, 0b111, 0b0000000),
    SemanticId.OR:    (InsnFormat.R, 0b0110011, 0b110, 0b0000000),
    SemanticId.XOR:   (InsnFormat.R, 0b0110011, 0b100, 0b0000000),
    SemanticId.SLT:   (InsnFormat.R, 0b0110011, 0b010, 0b0000000),
    SemanticId.SLTU:  (InsnFormat.R, 0b0110011, 0b011, 0b0000000),
    SemanticId.SLL:   (InsnFormat.R, 0b0110011, 0b001, 0b0000000),
    SemanticId.SRL:   (InsnFormat.R, 0b0110011, 0b101, 0b0000000),
    SemanticId.SRA:   (InsnFormat.R, 0b0110011, 0b101, 0b0100000),
    SemanticId.ADDI:  (InsnFormat.I, 0b0010011, 0b000, None),
    SemanticId.LW:    (InsnFormat.I, 0b0000011, 0b010, None),
    SemanticId.SW:    (InsnFormat.S, 0b0100011, 0b010, None),
    SemanticId.BEQ:   (InsnFormat.B, 0b1100011, 0b000, None),
    SemanticId.JAL:   (InsnFormat.J, 0b1101111, None, None),
    SemanticId.LUI:   (InsnFormat.U, 0b0110111, None, None),
    SemanticId.AUIPC: (InsnFormat.U, 0b0010111, None, None),
}


def encode_r_insn(fmt: InsnFormat, opcode: int, funct3: int, funct7: int,
                       rd: int, rs1: int, rs2: int) -> int:
    """R 格式编码"""
    return (
        (funct7 << 25)
        | (rs2 << 20)
        | (rs1 << 15)
        | (funct3 << 12)
        | (rd << 7)
        | opcode
    )


def encode_i_insn(opcode: int, funct3: int,
                  rd: int, rs1: int, imm: int) -> int:
    """I 格式编码"""
    imm12 = imm & 0xFFF
    return (
        (imm12 << 20)
        | (rs1 << 15)
        | (funct3 << 12)
        | (rd << 7)
        | opcode
    )


def encode_container(semantic_id: SemanticId, rd: int, rs1: int, rs2: int,
                     imm: int = 0) -> Optional[Container]:
    """
    将语义 ID + 操作数编码为 64 位容器
    极性=主人(0x01), 主权=公有(0x01), 坐标=RISC-V(0x0001)
    """
    if semantic_id not in RV32I_TEMPLATES:
        return None

    fmt, opcode, funct3, funct7 = RV32I_TEMPLATES[semantic_id]

    machine_code = None
    if fmt == InsnFormat.R:
        if funct7 is not None:
            machine_code = encode_r_insn(fmt, opcode, funct3, funct7, rd, rs1, rs2)
    elif fmt == InsnFormat.I:
        machine_code = encode_i_insn(opcode, funct3, rd, rs1, imm)
    # S/B/U/J 格式暂返回占位
    if machine_code is None:
        return None

    return Container(
        platform=Platform.RISCV,
        polarity=Polarity.OWNER,
        ownership=Ownership.PUBLIC,
        value=machine_code,
    )


def decode_container(container: Container) -> Optional[dict]:
    """从容器解出语义 ID 和操作数"""
    mc = container.value
    opcode = mc & 0x7F
    funct3 = (mc >> 12) & 0x7
    funct7 = (mc >> 25) & 0x7F
    rd     = (mc >> 7)  & 0x1F
    rs1    = (mc >> 15) & 0x1F
    rs2    = (mc >> 20) & 0x1F

    # 查表
    for sid, (fmt, op3, f3, f7) in RV32I_TEMPLATES.items():
        if op3 == opcode and f3 == funct3:
            if fmt == InsnFormat.R and f7 == funct7:
                return {
                    'semantic_id': sid,
                    'semantic_name': sid.name,
                    'format': fmt.name,
                    'rd': rd, 'rs1': rs1, 'rs2': rs2,
                    'machine_code': hex(mc),
                }
            elif fmt != InsnFormat.R:
                return {
                    'semantic_id': sid,
                    'semantic_name': sid.name,
                    'format': fmt.name,
                    'rd': rd, 'rs1': rs1,
                    'imm': mc & 0xFFF if fmt == InsnFormat.I else None,
                    'machine_code': hex(mc),
                }

    return None


if __name__ == '__main__':
    # 编码测试: ADD x1, x2, x3 = 0x003100B3
    print("=== 编码测试 ===")
    for sid_str, operands in [("ADD", (1, 2, 3)), ("ADDI", (5, 1, 100)), ("SUB", (10, 8, 9))]:
        sid = SemanticId[sid_str]
        if sid in RV32I_TEMPLATES and RV32I_TEMPLATES[sid][0] == InsnFormat.R:
            c = encode_container(sid, *operands, 0)
        elif sid == SemanticId.ADDI:
            c = encode_container(sid, operands[0], operands[1], 0, imm=operands[2])
        else:
            c = encode_container(sid, *operands, 0)

        if c:
            cell = c.encode()
            print(f"{sid_str} {operands} -> cell=0x{cell:016X}")
            decoded = decode_container(c)
            print(f"  解码: {decoded}")
    print("\n=== 解码测试 ===")
    test_codes = [0x003100B3, 0x00510093, 0x01258533]
    for mc in test_codes:
        c = Container.decode(mc + (Platform.RISCV << 48) + (Polarity.OWNER << 40) + (Ownership.PUBLIC << 32))
        decoded = decode_container(c)
        print(f"MC=0x{mc:08X} -> {decoded}")
