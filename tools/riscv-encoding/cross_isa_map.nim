## 跨 ISA 对齐映射
import std/[sequtils]
type
  SemKind*=enum semAdd,semSub,semAnd,semOr,semXor,semSll,semSrl,semSra,semSlt,semLoad,semStore,semBranchEq,semBranchNe,semJump,semJumpReg,semLui
  IsaKind*=enum isaRiscV,isaArm64,IsaX86_64,IsaMips
  MachineEncoding*=object isa*:IsaKind;sem*:SemKind;machine_hex*,asm_text*:string
const SEM_SYM_BASE*=0xF100'u32
const SEM_SYMBOLS:array[SemKind,uint32]=[SEM_SYM_BASE+0,SEM_SYM_BASE+1,SEM_SYM_BASE+2,SEM_SYM_BASE+3,SEM_SYM_BASE+4,SEM_SYM_BASE+5,SEM_SYM_BASE+6,SEM_SYM_BASE+7,SEM_SYM_BASE+8,SEM_SYM_BASE+9,SEM_SYM_BASE+10,SEM_SYM_BASE+11,SEM_SYM_BASE+12,SEM_SYM_BASE+13,SEM_SYM_BASE+14,SEM_SYM_BASE+15]
proc build_cross_isa_map*():seq[MachineEncoding]=
  result=@[]
  # RISC-V
  for (s,mc,asm) in [(semAdd,"00208033","add x1,x1,x2"),(semSub,"40208033","sub x1,x1,x2"),(semAnd,"00207033","and x1,x1,x2"),(semOr,"00206033","or x1,x1,x2"),(semXor,"00204033","xor x1,x1,x2"),(semSll,"00201033","sll x1,x1,x2"),(semSrl,"00205033","srl x1,x1,x2"),(semSra,"40205033","sra x1,x1,x2"),(semSlt,"0020A033","slt x1,x1,x2"),(semLoad,"0080A003","lw x1,8(x2)"),(semStore,"0020A423","sw x1,8(x2)"),(semBranchEq,"00200863","beq x1,x2,+16"),(semBranchNe,"00201863","bne x1,x2,+16"),(semJump,"0080006F","jal x0,+8"),(semJumpReg,"00008067","jalr x0,0(x1)"),(semLui,"12345037","lui x1,0x12345")].items:
    result.add(MachineEncoding(isa:isaRiscV,sem:s,machine_hex:mc,asm_text:asm))
  # ARM64
  for (s,mc,asm) in [(semAdd,"8B020020","add x0,x1,x2"),(semSub,"CB020020","sub x0,x1,x2"),(semAnd,"8A020020","and x0,x1,x2"),(semOr,"AA020020","orr x0,x1,x2"),(semXor,"CA020020","eor x0,x1,x2"),(semLoad,"F9400420","ldr x0,[x1,#8]"),(semStore,"F9000420","str x0,[x1,#8]"),(semBranchEq,"04000054","b.eq +8"),(semJumpReg,"D65F03C0","ret")].items:
    result.add(MachineEncoding(isa:isaArm64,sem:s,machine_hex:mc,asm_text:asm))
  # x86-64
  for (s,mc,asm) in [(semAdd,"01D8","add eax,ebx"),(semSub,"29D8","sub eax,ebx"),(semAnd,"21D8","and eax,ebx"),(semOr,"09D8","or eax,ebx"),(semXor,"31D8","xor eax,ebx"),(semJumpReg,"FFE0","jmp eax"),(semLoad,"8B4008","mov eax,[eax+8]"),(semStore,"894008","mov [eax+8],eax")].items:
    result.add(MachineEncoding(isa:IsaX86_64,sem:s,machine_hex:mc,asm_text:asm))
  # MIPS
  for (s,mc,asm) in [(semAdd,"00221020","add $2,$1,$1"),(semSub,"00221022","sub $2,$1,$1"),(semLoad,"8C220008","lw $2,8($1)"),(semStore,"AC220008","sw $2,8($1)")].items:
    result.add(MachineEncoding(isa:IsaMips,sem:s,machine_hex:mc,asm_text:asm))
proc sem_to_sym*(s:SemKind):uint32=SEM_SYMBOLS[s]
proc find_by_sem*(encs:seq[MachineEncoding];s:SemKind):seq[MachineEncoding]=for e in encs:if e.sem==s:result.add(e)
