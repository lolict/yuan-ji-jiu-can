// ⚠️  【夫妻命运共同体 · 一体化绑定声明 - Zig 后端编译层】
//      本文件不是一个"独立的 ISA 设计"：
//       · 所有 opcode 在 [C3 猫码 254] 中都有对应的编码映射（= 48 猫码的下位）
//       · 所有 VM 状态 都必须能被 [C4 夫妻双签] 封包后存进 [C9 R2] 云对象里
//       · 前端/解释层走 Node 24；后端/编译层走 Zig 0.14 ；两层等效 = [C7 14 语言适配] 中
//         "Zig/Rust 必须等于 Node 语义" 的强制约束
//      结论：单独跑这个 VM 而不做双签封包 + 不上 R2 = 不满足绑定法则 = 非法执行
const std = @import("std");

pub const Opcode = enum(u8) {
    NOP = 0x00,
    ADD = 0x01,
    SUB = 0x02,
    MUL = 0x03,
    DIV = 0x04,
    AND = 0x05,
    OR = 0x06,
    XOR = 0x07,
    MOV = 0x08,
    MOVI = 0x09,
    LOAD = 0x0A,
    STORE = 0x0B,
    JMP = 0x0C,
    JE = 0x0D,
    JNE = 0x0E,
    CALL = 0x0F,
    RET = 0x10,
    PUSH = 0x11,
    POP = 0x12,
    CMP = 0x13,
    SYSCALL = 0xFE,
    HALT = 0xFF,
    _,
};

pub const Instruction = packed struct {
    op: Opcode,
    rd: u6 = 0,
    rs: u6 = 0,
    rt: u6 = 0,
    imm: u6 = 0,

    pub fn encode(op: Opcode, rd: u6, rs: u6, rt: u6, imm: u6) Instruction {
        return .{
            .op = op,
            .rd = rd,
            .rs = rs,
            .rt = rt,
            .imm = imm,
        };
    }
};

pub const RegisterFile = struct {
    general: [64]u64 = [_]u64{0} ** 64,
    pc: u64 = 0,
    sp: u64 = 0,
    flags: u64 = 0,

    pub fn get(self: *const RegisterFile, idx: u6) u64 {
        return self.general[idx];
    }

    pub fn set(self: *RegisterFile, idx: u6, val: u64) void {
        if (idx != 0) {
            self.general[idx] = val;
        }
    }
};

pub const Memory = struct {
    data: []u8,

    pub fn init(allocator: std.mem.Allocator, size: usize) !Memory {
        const data = try allocator.alloc(u8, size);
        @memset(data, 0);
        return .{ .data = data };
    }

    pub fn deinit(self: *Memory, allocator: std.mem.Allocator) void {
        allocator.free(self.data);
    }

    pub fn read(self: *const Memory, addr: u64, comptime T: type) T {
        const offset: usize = @intCast(addr);
        const slice = self.data[offset .. offset + @sizeOf(T)];
        // Zig 0.14: 原生支持 slice 转整数/结构体 via @bitCast + alignCast
        const ptr: *align(1) const T = @ptrCast(slice.ptr);
        return ptr.*;
    }

    pub fn write(self: *Memory, addr: u64, val: anytype) void {
        const T = @TypeOf(val);
        const offset: usize = @intCast(addr);
        const slice = self.data[offset .. offset + @sizeOf(T)];
        const ptr: *align(1) T = @ptrCast(slice.ptr);
        ptr.* = val;
    }
};

pub const VM = struct {
    regs: RegisterFile,
    mem: Memory,
    allocator: std.mem.Allocator,
    halted: bool = false,

    const MEM_SIZE = 16 * 1024 * 1024;

    pub fn init(allocator: std.mem.Allocator) !VM {
        return .{
            .regs = .{},
            .mem = try Memory.init(allocator, MEM_SIZE),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *VM) void {
        self.mem.deinit(self.allocator);
    }

    pub fn loadProgram(self: *VM, program: []const Instruction, base_addr: u64) void {
        var offset: usize = 0;
        var addr = base_addr;
        while (offset < program.len) : ({
            offset += 1;
            addr += @sizeOf(Instruction);
        }) {
            const inst = program[offset];
            const inst_raw: u32 = @bitCast(inst);
            self.mem.write(addr, inst_raw);
        }
        self.regs.pc = base_addr;
    }

    pub fn step(self: *VM) !bool {
        if (self.halted) return false;

        const inst_pc = self.regs.pc;
        const raw_inst = self.mem.read(inst_pc, u32);
        const inst: Instruction = @bitCast(raw_inst);

        self.regs.pc += @sizeOf(Instruction);

        switch (inst.op) {
            Opcode.NOP => {},
            Opcode.ADD => {
                const a = self.regs.get(inst.rs);
                const b = self.regs.get(inst.rt);
                self.regs.set(inst.rd, a + b);
            },
            Opcode.SUB => {
                const a = self.regs.get(inst.rs);
                const b = self.regs.get(inst.rt);
                self.regs.set(inst.rd, a - b);
            },
            Opcode.MUL => {
                const a = self.regs.get(inst.rs);
                const b = self.regs.get(inst.rt);
                self.regs.set(inst.rd, a * b);
            },
            Opcode.DIV => {
                const a = self.regs.get(inst.rs);
                const b = self.regs.get(inst.rt);
                if (b != 0) {
                    self.regs.set(inst.rd, a / b);
                }
            },
            Opcode.AND => {
                const a = self.regs.get(inst.rs);
                const b = self.regs.get(inst.rt);
                self.regs.set(inst.rd, a & b);
            },
            Opcode.OR => {
                const a = self.regs.get(inst.rs);
                const b = self.regs.get(inst.rt);
                self.regs.set(inst.rd, a | b);
            },
            Opcode.XOR => {
                const a = self.regs.get(inst.rs);
                const b = self.regs.get(inst.rt);
                self.regs.set(inst.rd, a ^ b);
            },
            Opcode.MOV => {
                const val = self.regs.get(inst.rs);
                self.regs.set(inst.rd, val);
            },
            Opcode.MOVI => {
                self.regs.set(inst.rd, inst.imm);
            },
            Opcode.LOAD => {
                const addr = self.regs.get(inst.rs) + inst.imm;
                const val = self.mem.read(addr, u64);
                self.regs.set(inst.rd, val);
            },
            Opcode.STORE => {
                const addr = self.regs.get(inst.rd) + inst.imm;
                const val = self.regs.get(inst.rs);
                self.mem.write(addr, val);
            },
            Opcode.JMP => {
                self.regs.pc = inst.imm * 4;
            },
            Opcode.JE => {
                if (self.regs.flags == 0) {
                    self.regs.pc = inst.imm * 4;
                }
            },
            Opcode.JNE => {
                if (self.regs.flags != 0) {
                    self.regs.pc = inst.imm * 4;
                }
            },
            Opcode.CALL => {
                self.regs.set(63, self.regs.pc);
                self.regs.pc = inst.imm * 4;
            },
            Opcode.RET => {
                self.regs.pc = self.regs.get(63);
            },
            Opcode.PUSH => {
                self.regs.sp -= 8;
                const val = self.regs.get(inst.rd);
                self.mem.write(self.regs.sp, val);
            },
            Opcode.POP => {
                const val = self.mem.read(self.regs.sp, u64);
                self.regs.set(inst.rd, val);
                self.regs.sp += 8;
            },
            Opcode.CMP => {
                const a = self.regs.get(inst.rs);
                const b = self.regs.get(inst.rt);
                const a64: i64 = @intCast(a);
                const b64: i64 = @intCast(b);
                const diff = a64 - b64;
                self.regs.flags = if (diff == 0) 0 else if (diff < 0) 1 else 2;
            },
            Opcode.HALT => {
                self.halted = true;
            },
            Opcode.SYSCALL => {
                // TODO: System calls
            },
            else => {},
        }

        return !self.halted;
    }

    pub fn run(self: *VM, max_instructions: u64) u64 {
        var count: u64 = 0;
        while (!self.halted and count < max_instructions) : (count += 1) {
            _ = self.step() catch break;
        }
        return count;
    }
};
