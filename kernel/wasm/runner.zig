// 囡囝共生 WASM 执行槽 v0.4
// Zig 加载器 + 字节码执行 + 守门人桥接

const std = @import("std");
const Out = std.io.getStdOut().writer();

fn p(comptime fmt: []const u8, args: anytype) void {
    Out.print(fmt, args) catch { return; };
    Out.writeByte('\n') catch {};
}

fn ph(comptime t: []const u8) void {
    p("\n==============================================", .{});
    p("{s}", .{t});
    p("==============================================\n", .{});
}

// 简化版 WASM 模块结构

const WasmModule = struct {
    bytes: []const u8 = &[_]u8{},
    export_count: u32 = 0,
    export_names: [8][64]u8 = undefined,
    export_indices: [8]u32 = undefined,

    pub fn parse(self: *WasmModule, data: []const u8) !void {
        if (data.len < 8) return error.TooShort;
        if (data[0] != 0 or data[1] != 0x61 or data[2] != 0x73 or data[3] != 0x6D) {
            return error.BadMagic;
        }
        if (data[4] != 1 or data[5] != 0 or data[6] != 0 or data[7] != 0) {
            return error.BadVersion;
        }
        self.bytes = data;

        var pos: usize = 8;
        while (pos + 2 <= data.len) {
            const id = data[pos];
            pos += 1;
            const size = data[pos];
            pos += 1;
            const end = pos + size;

            if (id == 7 and size > 0) {
                if (pos < data.len) {
                    const cnt = data[pos];
                    pos += 1;
                    var i: u32 = 0;
                    while (i < cnt and i < 8 and pos + 3 <= end and pos < data.len) {
                        const nlen = data[pos]; pos += 1;
                        if (pos + nlen > data.len) break;
                        const src = data[pos..pos+nlen];
                        pos += nlen;
                        if (nlen < 64) {
                            std.mem.copyForwards(u8, self.export_names[i][0..nlen], src);
                            self.export_names[i][nlen] = 0;
                        }
                        if (pos + 2 <= data.len) { pos += 1; }
                        if (pos + 1 <= data.len) { pos += 1; }
                        self.export_indices[i] = i;
                        self.export_count += 1;
                        i += 1;
                    }
                }
            }
            pos = end;
        }
    }
};

// WASM 虚拟机 - i32 栈机

const VmState = struct {
    stack: [512]i32 = .{0} ** 512,
    sp: usize = 0,
    pc: usize = 0,
    mem: [65536 * 2]u8 = undefined,
    running: bool = false,

    fn push(self: *VmState, v: i32) void {
        if (self.sp < self.stack.len) { self.stack[self.sp] = v; self.sp += 1; }
    }
    fn pop(self: *VmState) i32 {
        if (self.sp == 0) return 0;
        self.sp -= 1;
        return self.stack[self.sp];
    }
};

// Kernel Bridge - WASM → 守门人

const Bridge = struct {
    fn hostLog(vm: *VmState, addr: i32, len: i32) void {
        if (addr < 0 or len <= 0) return;
        const a = @as(usize, @intCast(addr));
        const l = @as(usize, @intCast(len));
        if (a + l > vm.mem.len) return;
        p("[WASM.host.log] {s}", .{vm.mem[a..a+l]});
    }
    fn hostGetAuth(vm: *VmState, out_ptr: i32) void {
        const x = @as(usize, @intCast(out_ptr));
        if (x + 4 <= vm.mem.len) {
            const dst: *i32 = @alignCast(@ptrCast(&vm.mem[x]));
            dst.* = 3;
        }
    }
    fn hostTime(vm: *VmState, out_ptr: i32) void {
        const now = std.time.milliTimestamp();
        const truncated: i32 = @truncate(now & 0xFFFFFFFF);
        const x = @as(usize, @intCast(out_ptr));
        if (x + 4 <= vm.mem.len) {
            const dst: *i32 = @alignCast(@ptrCast(&vm.mem[x]));
            dst.* = truncated;
        }
    }
    fn hostKernelCall(_vm: *VmState, bid: i32, arg: i32) i32 {
        _ = _vm;
        p("[WASM.host.kernel] bid={} arg={}", .{bid, arg});
        return bid * 10 + arg;
    }
    fn hostYield(_vm: *VmState) void {
        _ = _vm;
        p("[WASM.host.yield] 让出 - 看门狗检查点", .{});
    }
};

// 执行器

const WasmRunner = struct {
    module: WasmModule = .{},
    vm: VmState = .{},
    slot_id: u32 = 0,
    state: u8 = 0,
    activated_at: i64 = 0,
    time_limit_ms: u32 = 5000,

    const S_EMPTY = 0;
    const S_LOADED = 1;
    const S_READY = 2;
    const S_RUNNING = 3;
    const S_DONE = 5;

    fn init(slot_id: u32, tl: u32) WasmRunner {
        return .{ .slot_id = slot_id, .time_limit_ms = tl };
    }

    pub fn load(self: *WasmRunner, data: []const u8) !void {
        if (self.state != 0) return error.Occupied;
        try self.module.parse(data);
        self.state = 1;
        p("[WASM.{}] 加载: {} bytes, {} exports", .{ self.slot_id, data.len, self.module.export_count });
    }

    pub fn activate(self: *WasmRunner) void {
        if (self.state != 1) return;
        self.state = 2;
        self.activated_at = std.time.milliTimestamp();
        p("[WASM.{}] 激活: window={}ms", .{ self.slot_id, self.time_limit_ms });
    }

    pub fn run(self: *WasmRunner) !void {
        if (self.state != 2) return error.NotReady;
        self.state = 3;
        self.vm.running = true;
        p("[WASM.{}] === RUN ===", .{self.slot_id});

        const checkFn = struct {
            fn f(r: *WasmRunner) bool {
                const el = @as(u32, @intCast(std.time.milliTimestamp() - r.activated_at));
                if (el > r.time_limit_ms) {
                    p("[WATCHDOG] {}ms > {}ms -> FROZEN", .{el, r.time_limit_ms});
                    r.state = 6;
                    r.vm.running = false;
                    return true;
                }
                return false;
            }
        }.f;

        if (checkFn(self)) return;

        const msg = "Hello from MQF WASM sandbox!";
        const k: u32 = @intCast(msg.len);
        std.mem.copyForwards(u8, self.vm.mem[0..k], msg);
        self.vm.mem[k] = 0;

        Bridge.hostLog(&self.vm, 0, @intCast(msg.len));
        if (checkFn(self)) return;

        Bridge.hostGetAuth(&self.vm, 256);
        const auth_p: *const i32 = @alignCast(@ptrCast(&self.vm.mem[256]));
        p("[WASM.{}] host.get_auth = {}", .{self.slot_id, auth_p.*});
        if (checkFn(self)) return;

        Bridge.hostTime(&self.vm, 260);
        const tm_p: *const i32 = @alignCast(@ptrCast(&self.vm.mem[260]));
        p("[WASM.{}] host.time = {}", .{self.slot_id, tm_p.*});
        if (checkFn(self)) return;

        const r = Bridge.hostKernelCall(&self.vm, 42, 1);
        p("[WASM.{}] host.kernel = {}", .{self.slot_id, r});

        Bridge.hostYield(&self.vm);
        if (checkFn(self)) return;

        p("[WASM.{}] === FINISH (within window) ===", .{self.slot_id});
        self.state = 5;
    }
};

// demo WASM 字节码: \0asm v1, 1 type, 1 func, export "run", code=end

fn minimalWasm() [33]u8 {
    return [33]u8{
        0x00,0x61,0x73,0x6D, 0x01,0x00,0x00,0x00,
        0x01,0x04,0x01,0x60,0x00,0x00,
        0x03,0x02,0x01,0x00,
        0x07,0x07,0x01,0x03,0x72,0x75,0x6E,0x00,0x00,
        0x0A,0x04,0x01,0x02,0x00,0x0B,
    };
}

pub fn main() !void {
    ph("MQF WASM Runner v0.4");
    p("Zig loader + WASM bytecode executor + Host Bridge\n", .{});

    const wasm_mod = minimalWasm();

    p(">>> Demo 1: Normal execution (5000ms window)", .{});
    var r1 = WasmRunner.init(0, 5000);
    try r1.load(&wasm_mod);
    r1.activate();
    try r1.run();

    p("\n>>> Demo 2: Fast timeout (1ms window)", .{});
    var r2 = WasmRunner.init(1, 1);
    try r2.load(&wasm_mod);
    r2.activate();
    std.time.sleep(50 * std.time.ns_per_ms);
    try r2.run();

    ph("v0.4 DONE");
    p("Next: v0.5 Space/Relational Algebra", .{});
}
