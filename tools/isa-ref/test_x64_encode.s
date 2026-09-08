# x86-64 机器码实测 (GAS/AT&T 语法)
# 覆盖 full_alignment.md 中全部 x86-64 指令类别
# 编译: as --64 test_x64_encode.s -o test_x64_encode.o && objdump -d test_x64_encode.o
.intel_syntax noprefix

.text
.global _start

########## 1. 寄存器移动 ##########
# MOV reg→reg
mov rax, rbx           # 48 89 D8
mov r8, r9             # 4D 89 C8 (REX.RB)
# MOV imm→reg
mov eax, 1             # B8 01 00 00 00
mov rax, 0x12345678    # 48 B8 ...
mov r8, 0x7FFFFFFF     # 41 B8 ...
# XOR 清零
xor eax, eax           # 31 C0
xor r8d, r8d           # 45 31 C0
# PUSH/POP
push rax               # 50
pop rax                # 58

########## 2. 算术 R-R ##########
add rax, rbx           # 48 01 D8
sub rax, rbx           # 48 29 D8
and rax, rbx           # 48 21 D8
or  rax, rbx           # 48 09 D8
xor rax, rbx           # 48 31 D8
add r8, r9             # 4D 01 C8

########## 3. 算术立即数 ##########
add rax, 0xFF          # 48 83 C0 FF
add eax, 42            # 83 C0 2A
add rax, 0x1000        # 48 81 C0 00 10 00 00
sub rax, 0x10          # 48 83 E8 10
and rax, 0xFFFF        # 48 83 E0 FF (sign-extended) or 48 25 FF FF 00 00
or  rax, 0x1000        # 48 81 C8 00 10 00 00
xor rax, 0xFF          # 48 83 F0 FF
inc rax                # 48 FF C0
dec rax                # 48 FF C8
neg rax                # 48 F7 D8
not rax                # 48 F7 D0

########## 4. 移位 ##########
shl rax, 3             # 48 C1 E0 03
shr rax, 3             # 48 C1 E8 03
sar rax, 3             # 48 C1 F8 03
shl rax, 1             # 48 D1 E0
shr rax, cl            # 48 D3 E8
rol rax, 1             # 48 D1 C0
ror rax, 1             # 48 D1 C8
shld rax, rbx, 4       # 48 0F A4 D8 04

########## 5. 比较与测试 ##########
cmp rax, rbx           # 48 39 C3
cmp rax, 0x100         # 48 81 F8 00 01 00 00
cmp eax, 42            # 83 F8 2A
test rax, rax          # 48 85 C0
test eax, 0xFF         # A9 FF 00 00 00

########## 6. 条件跳转 ##########
1: jmp 1f              # EB FE (short) 或 E9 xx
   nop
1:
je  2f                 # 74 xx (short) 或 0F 84 xx
   nop
2:
jne 3f                 # 75 xx
   nop
3:
jl  4f                 # 7C xx 或 0F 8C
   nop
4:
jge 5f                 # 7D xx 或 0F 8D
   nop
5:
jb  6f                 # 72 xx 或 0F 82
   nop
6:
ja  7f                 # 77 xx 或 0F 87
   nop
7:

########## 7. CALL/RET ##########
call func              # E8 xx xx xx xx
ret                    # C3

########## 8. 内存访问 (load/store) ##########
# Load
mov rax, [rbx]         # 48 8B 03
mov rax, [rbx+8]       # 48 8B 43 08
mov rax, [rbx+0x100]   # 48 8B 83 00 01 00 00
mov rax, [rbx+rcx*8]   # 48 8B 04 CB
mov eax, [rbx]         # 8B 03
mov ax,  [rbx]         # 66 8B 03
mov al,  [rbx]         # 8A 03
movzx eax, byte ptr [rbx]  # 0F B6 03
movsx eax, word ptr [rbx]  # 0F BF 03
# Store
mov [rbx], rax         # 48 89 03
mov [rbx+8], rax       # 48 89 43 08
mov [rbx+0x80], eax    # 89 83 80 00 00 00
mov [rbx], ax          # 66 89 03
mov [rbx], al          # 88 03

########## 9. LEA ##########
lea rax, [rbx+8]       # 48 8D 43 08
lea rax, [rbx+rcx*4]   # 48 8D 04 8B
lea rsi, [rip+0]       # 48 8D 35 00 00 00 00 (RIP-relative)

########## 10. 原子操作 ##########
lock add [rbx], rax    # F0 48 01 03
lock xchg [rbx], rax   # F0 48 87 03
lock cmpxchg [rbx], rcx # F0 48 0F B1 0B
xchg rax, rbx          # 48 93
xchg eax, ebx          # 93 (implicit lock)
mfence                 # 0F AE F0
sfence                 # 0F AE F8
lfence                 # 0F AE E8
pause                  # F3 90

########## 11. 乘除法 ##########
imul rax, rbx          # 48 0F AF C3
imul rax, rbx, 0x100   # 48 69 C3 00 01 00 00
mul rbx                # 48 F7 E3
imul rbx               # 48 F7 EB
cqo                    # 48 98
idiv rbx               # 48 F7 FB
div rbx                # 48 F7 F3
imul eax, ebx          # 0F AF C3

########## 12. 位运算扩展 ##########
bsf rax, rbx           # 48 0F BC C3
bsr rax, rbx           # 48 0F BD C3
tzcnt rax, rbx         # F3 0F BC C3
lzcnt rax, rbx         # F3 0F BD C3
popcnt rax, rbx        # F3 0F B8 C3
bt  rax, rbx           # 48 0F A3 D8
btc rax, rbx           # 48 0F BB D8
bts rax, rbx           # 48 0F AB D8
btr rax, rbx           # 48 0F B3 D8

########## 13. 条件移动 ##########
cmove rax, rbx         # 48 0F 44 C3
cmovne rax, rbx        # 48 0F 45 C3
cmovl rax, rbx         # 48 0F 4C C3
cmovg rax, rbx         # 48 0F 4F C3

########## 14. 系统调用 ##########
syscall                # 0F 05
.byte 0x48, 0x0F, 0x07  # sysret (REX.W + 0F 07)
int3                   # CC
int 0x80               # CD 80

########## 15. NOP / 其他 ##########
nop                    # 90
nop dword ptr [rax]    # 0F 1F 00 (multi-byte nop)
hlt                    # F4
cli                    # FA
sti                    # FB
rdtsc                  # 0F 31
rdtscp                 # 0F 01 F9
iretq                  # 48 CF
cpuid                  # 0F A2
1: jmp 1b              # short self-jump for relative offset test

.type func, @function
func:
  mov rax, 0x1234
  ret
