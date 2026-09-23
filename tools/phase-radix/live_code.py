#!/usr/bin/env python3
# ============================================================
# Live Code Compiler — 活码编译器
# 囡囝共生 OS 的即时写编执行核心:
#   REPL 表达式 → L0 字节码 → VM 执行 → 结果入细胞空间
#
# 哲学:
#   起点 = 钥匙 → 激活可活动的编码
#   未固定 = 任意位置可停留
#   固定后 = 按排列顺序定权限和数值
#   活码不是死的文本 — 写一条就编一条就跑一条
# ============================================================

import re
import struct

# ============================================================
# L0 操作码表 (从 C 头文件 /zig-stem/l0_compiler.h 同步)
# ============================================================
OPCODES = {
    'NOP':   0,     # 啊 = 空
    'PUSH':  2,     # 工 = 压栈
    'POP':   3,     # 空 = 出栈
    'ADD':   46,    # 合 = 加
    'SUB':   30,    # 七 = 减
    'MUL':   28,    # 五 = 乘
    'DIV':   29,    # 六 = 除
    'MOD':   31,    # 八 = 模
    'EQ':    27,    # 四 = 等于
    'LT':    24,    # 一 = 小于
    'GT':    25,    # 二 = 大于
    'JMP':   10,    # 机 = 跳转
    'JZ':    13,    # 序 = 零跳
    'JNZ':   12,    # 福 = 非零跳
    'CALL':  11,    # 挂 = 调用
    'RET':   20,    # 收 = 返回
    'LOAD':  18,    # 量 = 加载变量
    'STORE': 19,    # 端 = 存储变量
    'LOADI': 15,    # 土 = 加载立即数
    'PRINT': 21,    # 面 = 打印数值
    'PRINTC': 22,   # 令 = 打印字符
    'HALT':  50,    # 吉 = 停机
}

# 操作码 → 音质码 (反向映射)
OP_TO_VOICE = {v: k for k, v in OPCODES.items()}

# 变量槽位: a-z → 0-25, acc → 25(z) 累加器
VAR_SLOTS = {chr(ord('a') + i): i for i in range(26)}
VAR_SLOTS['acc'] = 25  # 累加器 = slot z

# L0 字节码格式: opcode(2B) operand1(2B) result(2B) = 6B/指令
INSTR_SIZE = 6
MAX_BYTECODE = 256
MAX_STACK = 64
MAX_VARS = 26

# ============================================================
# 编译器: AST/text → L0 字节码
# ============================================================
class LiveCodeCompiler:
    """活码编译器: 文本源码 → L0 字节码"""

    def __init__(self):
        self.ir = []          # 中间表示
        self.bytecode = bytearray()
        self.labels = {}      # 标签名 → 指令偏移
        self.fixups = []      # 待修复跳转
        self.source_lines = []

    def reset(self):
        self.ir.clear()
        self.bytecode = bytearray()
        self.labels.clear()
        self.fixups.clear()
        self.source_lines.clear()

    def current_offset(self):
        return len(self.bytecode) // INSTR_SIZE

    def emit(self, op, operand1=0, result=0):
        """发射一条指令到字节码流"""
        voice = OPCODES.get(op, 0)
        self.ir.append({
            'op_name': op,
            'voice': voice,
            'operand1': operand1,
            'result': result,
            'offset': self.current_offset()
        })
        self.bytecode.extend(struct.pack('<Hhh', voice, operand1, result))

    def parse_and_compile(self, source: str) -> bool:
        """解析并编译 L0 源码"""
        self.reset()
        lines = source.strip().split('\n')

        # 第一遍: 收集标签
        instr_idx = 0
        for line in lines:
            line = line.strip()
            if not line or line.startswith('#') or line.startswith('//'):
                continue
            if line.endswith(':'):
                self.labels[line[:-1]] = instr_idx
                continue
            instr_idx += 1

        # 第二遍: 生成代码
        for line in lines:
            line = line.strip()
            if not line or line.startswith('#') or line.startswith('//'):
                continue
            if line.endswith(':'):
                continue  # 标签在第一遍处理过

            self._compile_line(line)

        # 修复跳转
        for label, target_offset in self.fixups:
            if label in self.labels:
                patch_offset = target_offset
                self.bytecode[patch_offset * INSTR_SIZE + 2:patch_offset * INSTR_SIZE + 4] = \
                    struct.pack('<h', self.labels[label])
            else:
                return False  # 标签未定义

        return True

    def _compile_line(self, line: str):
        """编译单行"""
        parts = line.replace(',', ' ').split()
        if not parts:
            return

        op = parts[0].upper()

        if op not in OPCODES:
            return

        # 解析操作数
        o1 = 0
        r = 0

        if op == 'LOADI' and len(parts) >= 3:
            var = parts[1]
            val = int(parts[2], 0)
            o1 = VAR_SLOTS.get(var, 0)
            r = val
            self.emit('LOADI', o1, r)

        elif op == 'LOAD' and len(parts) >= 2:
            o1 = VAR_SLOTS.get(parts[1], 0)
            self.emit('LOAD', o1, 0)

        elif op == 'STORE' and len(parts) >= 2:
            o1 = VAR_SLOTS.get(parts[1], 0)
            self.emit('STORE', o1, 0)

        elif op == 'ADD' and len(parts) >= 4:
            # ADD a b result → load b, load a, add, store result
            dest = VAR_SLOTS.get(parts[3], 0)
            o1_val = VAR_SLOTS.get(parts[2], 0) if parts[2] in VAR_SLOTS else int(parts[2], 0)
            o2_val = VAR_SLOTS.get(parts[1], 0) if parts[1] in VAR_SLOTS else int(parts[1], 0)
            self.emit('LOADI', dest, 0)  # 目标初始化为0
            self.emit('PUSH', o1_val, 0)
            self.emit('PUSH', o2_val, 0)
            self.emit('ADD', 0, 0)
            self.emit('STORE', dest, 0)

        elif op in ('ADD', 'SUB', 'MUL', 'DIV', 'MOD') and len(parts) >= 3:
            # ADD a b → acc = a op b (push a, push b, op, store acc)
            slot_a = VAR_SLOTS.get(parts[1], 0)
            slot_b = VAR_SLOTS.get(parts[2], 0)
            acc = VAR_SLOTS.get('acc', 25)  # slot 25 = accumulator
            self.emit('LOAD', slot_a, 0)
            self.emit('LOAD', slot_b, 0)
            self.emit(op, 0, 0)
            self.emit('STORE', acc, 0)

        elif op in ('EQ', 'LT', 'GT') and len(parts) >= 3:
            # EQ a b → acc = (a == b) ? 1 : 0
            slot_a = VAR_SLOTS.get(parts[1], 0)
            slot_b = VAR_SLOTS.get(parts[2], 0)
            acc = VAR_SLOTS.get('acc', 25)
            self.emit('LOAD', slot_a, 0)
            self.emit('LOAD', slot_b, 0)
            self.emit(op, 0, 0)
            self.emit('STORE', acc, 0)


        elif op == 'JMP' and len(parts) >= 2:
            label = parts[1]
            offset = self.current_offset()
            self.emit('JMP', 0, 0)
            self.fixups.append((label, offset))

        elif op == 'JZ' and len(parts) >= 2:
            label = parts[1]
            offset = self.current_offset()
            self.emit('JZ', 0, 0)
            self.fixups.append((label, offset))

        elif op == 'JNZ' and len(parts) >= 2:
            label = parts[1]
            offset = self.current_offset()
            self.emit('JNZ', 0, 0)
            self.fixups.append((label, offset))

        elif op == 'PUSH' and len(parts) >= 2:
            val = int(parts[1], 0) if not parts[1] in VAR_SLOTS else VAR_SLOTS[parts[1]]
            self.emit('PUSH', val, 0)

        elif op == 'POP':
            self.emit('POP', 0, 0)

        elif op == 'PRINT':
            o1 = VAR_SLOTS.get(parts[1], 0) if len(parts) >= 2 else 0
            self.emit('PRINT', o1, 0)

        elif op == 'PRINTC' and len(parts) >= 2:
            ch_val = ord(parts[1]) if len(parts[1]) == 1 else int(parts[1], 0)
            self.emit('PRINTC', ch_val, 0)

        elif op == 'CALL' and len(parts) >= 2:
            label = parts[1]
            offset = self.current_offset()
            self.emit('CALL', 0, 0)
            self.fixups.append((label, offset))

        elif op == 'RET':
            self.emit('RET', 0, 0)

        elif op == 'HALT':
            self.emit('HALT', 0, 0)

        elif op == 'NOP':
            self.emit('NOP', 0, 0)

    def get_bytecode_hex(self) -> str:
        hex_str = ''
        for i in range(0, len(self.bytecode), INSTR_SIZE):
            chunk = self.bytecode[i:i+INSTR_SIZE]
            if len(chunk) >= 6:
                voice, op1, res = struct.unpack('<Hhh', chunk)
                hex_str += f'{voice:02x}{op1 & 0xFFFF:04x}{res & 0xFFFF:04x} '
        return hex_str.strip()

    def disassemble(self) -> list:
        lines = []
        for i, instr in enumerate(self.ir):
            voice = instr['voice']
            op_name = instr['op_name']
            o1 = instr['operand1']
            res = instr['result']
            lines.append(f"  [{i:3d}] v={voice:2d} {op_name:>6} {o1},{res}")
        return lines


# ============================================================
# L0 虚拟机: 执行字节码
# ============================================================
class LiveCodeVM:
    """L0 虚拟机 — 执行编译器产出的字节码"""

    def __init__(self):
        self.code = bytearray()
        self.stack = [0] * MAX_STACK
        self.sp = 0            # 栈指针 (指向下一个空闲位置)
        self.vars = [0] * MAX_VARS
        self.pc = 0            # 程序计数器
        self.running = False
        self.output = []       # 执行输出 (用于 PRINT 收集)
        self.output_fn = None  # 可选的外部输出回调
        self.cycles = 0        # 执行周期计数
        self.call_stack = []   # CALL/RET 栈

    def load(self, bytecode: bytearray):
        self.code = bytearray(bytecode)

    def reset(self):
        self.stack = [0] * MAX_STACK
        self.sp = 0
        self.vars = [0] * MAX_VARS
        self.pc = 0
        self.output = []
        self.cycles = 0
        self.call_stack.clear()

    def push(self, val: int):
        if self.sp < MAX_STACK:
            self.stack[self.sp] = val
            self.sp += 1

    def pop(self) -> int:
        if self.sp > 0:
            self.sp -= 1
            return self.stack[self.sp]
        return 0

    def emit_output(self, text: str):
        self.output.append(text)
        if self.output_fn:
            self.output_fn(text)

    def execute(self, max_cycles: int = 1000) -> dict:
        """执行已加载的字节码, 返回执行结果摘要"""
        self.running = True
        self.output = []
        self.cycles = 0

        while self.running and self.cycles < max_cycles:
            if self.pc * INSTR_SIZE >= len(self.code):
                break

            # 取指
            offset = self.pc * INSTR_SIZE
            voice, op1, res = struct.unpack('<Hhh', self.code[offset:offset + INSTR_SIZE])
            next_pc = self.pc + 1

            # 译码 + 执行
            if voice == OPCODES['NOP']:
                pass
            elif voice == OPCODES['HALT'] or voice == 50:
                self.running = False
                break
            elif voice == OPCODES['LOADI']:
                self.vars[op1] = res
            elif voice == OPCODES['LOAD']:
                self.push(self.vars[op1])
            elif voice == OPCODES['STORE']:
                self.vars[op1] = self.pop()
            elif voice == OPCODES['PUSH']:
                self.push(op1)
            elif voice == OPCODES['POP']:
                self.pop()
            elif voice == OPCODES['ADD']:
                if op1 > 0 and op1 < MAX_VARS:
                    self.vars[op1] += self.vars[res] if res < MAX_VARS else res
                else:
                    b = self.pop()
                    a = self.pop()
                    self.push(a + b)
            elif voice == OPCODES['SUB']:
                if op1 > 0 and op1 < MAX_VARS:
                    self.vars[op1] -= self.vars[res] if res < MAX_VARS else res
                else:
                    b = self.pop()
                    a = self.pop()
                    self.push(a - b)
            elif voice == OPCODES['MUL']:
                if op1 > 0 and op1 < MAX_VARS:
                    self.vars[op1] *= self.vars[res] if res < MAX_VARS else res
                else:
                    b = self.pop()
                    a = self.pop()
                    self.push(a * b)
            elif voice == OPCODES['DIV']:
                b = self.pop()
                a = self.pop()
                if b != 0:
                    self.push(a // b)
                else:
                    self.push(0)
            elif voice == OPCODES['MOD']:
                b = self.pop()
                a = self.pop()
                if b != 0:
                    self.push(a % b)
                else:
                    self.push(0)
            elif voice == OPCODES['EQ']:
                b = self.pop()
                a = self.pop()
                self.push(1 if a == b else 0)
            elif voice == OPCODES['LT']:
                b = self.pop()
                a = self.pop()
                self.push(1 if a < b else 0)
            elif voice == OPCODES['GT']:
                b = self.pop()
                a = self.pop()
                self.push(1 if a > b else 0)
            elif voice == OPCODES['JMP']:
                next_pc = op1
            elif voice == OPCODES['JZ']:
                val = self.pop()
                if val == 0:
                    next_pc = op1
            elif voice == OPCODES['JNZ']:
                val = self.pop()
                if val != 0:
                    next_pc = op1
            elif voice == OPCODES['CALL']:
                self.call_stack.append(next_pc)
                next_pc = op1
            elif voice == OPCODES['RET']:
                if self.call_stack:
                    next_pc = self.call_stack.pop()
                else:
                    self.running = False
            elif voice == OPCODES['PRINT']:
                if op1 < MAX_VARS:
                    val = self.vars[op1]
                    self.emit_output(str(val))
            elif voice == OPCODES['PRINTC']:
                self.emit_output(chr(op1) if 0 <= op1 < 128 else '?')

            self.pc = next_pc
            self.cycles += 1

        return {
            'cycles': self.cycles,
            'output': ''.join(self.output),
            'vars': {chr(ord('a')+i): self.vars[i] for i in range(26) if self.vars[i] != 0},
            'stack': self.stack[:self.sp]
        }


# ============================================================
# 高层封装: 从表达式到执行的完整管道
# ============================================================
class LiveCodePipeline:
    """活码管道: 表达式 → 编译 → 执行 → 结果"""

    def __init__(self):
        self.compiler = LiveCodeCompiler()
        self.vm = LiveCodeVM()

    def compile_expr(self, expr: str):
        """编译表达式"""
        ok = self.compiler.parse_and_compile(expr)
        return ok

    def execute(self):
        """执行当前编译好的字节码"""
        self.vm.load(self.compiler.bytecode)
        self.vm.reset()
        return self.vm.execute()

    def run(self, expr: str) -> dict:
        """完整管道: 编译 + 执行"""
        ok = self.compile_expr(expr)
        if not ok:
            return {'ok': False, 'error': 'compile failed'}
        result = self.execute()
        result['ok'] = True
        result['bytecode_hex'] = self.compiler.get_bytecode_hex()
        result['ir_count'] = len(self.compiler.ir)
        return result

    @property
    def bytecode(self):
        return self.compiler.bytecode

    @property
    def ir(self):
        return self.compiler.ir


# ============================================================
# CLI 自测
# ============================================================
if __name__ == '__main__':
    print("=== Live Code Compiler · 自测 ===\n")

    p = LiveCodePipeline()

    # 测试1: 计算 5 + 3
    prog1 = """LOADI a 5
LOADI b 3
ADD a b
PRINT a
HALT"""
    result = p.run(prog1)
    print(f"[Test 1] a=5 b=3 ADD a b → PRINT a")
    print(f"  bytecode: {result['bytecode_hex']}")
    print(f"  output:   {result['output']}")
    print(f"  vars:     {result['vars']}")
    print(f"  cycles:   {result['cycles']}")
    print()

    # 测试2: 乘法
    prog2 = """LOADI a 6
LOADI b 7
MUL a b
PRINT a
HALT"""
    result = p.run(prog2)
    print(f"[Test 2] a=6 b=7 MUL a b → PRINT a")
    print(f"  output:   {result['output']} (expected 42)")
    print(f"  vars:     {result['vars']}")
    print()

    # 测试3: 条件跳转
    prog3 = """LOADI a 1
LOADI b 0
GT a b
JNZ yes
LOADI c 999
JMP end
yes:
LOADI c 111
end:
PRINT c
HALT"""
    result = p.run(prog3)
    print(f"[Test 3] a=1 b=0 GT a b → if true c=111 else c=999")
    print(f"  output:   {result['output']} (expected 111)")
    print(f"  vars:     {result['vars']}")
    print()

    print("=== 自测完成 ===")
