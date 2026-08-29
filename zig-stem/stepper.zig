// 步进器 (Stepper)
// 功能：控制节拍推进，让系统一步步前进
// 核心：节拍生成 + 步进控制 + 状态推进 + 同步等待

const std = @import("std");
const mapper = @import("mapper.zig");
const aligner = @import("aligner.zig");
const indexer = @import("indexer.zig");

// ============================================================
// 1. 节拍 (Beat)
// ============================================================
pub const Beat = struct {
    index: u64,             // 节拍序号
    timestamp: i128,        // 时间戳
    phase: f64,             // 相位位置 (0-1)
    is_active: bool,        // 是否激活

    pub fn init(index: u64, phase: f64) Beat {
        return Beat{
            .index = index,
            .timestamp = std.time.nanoTimestamp(),
            .phase = @mod(phase, 1.0),
            .is_active = true,
        };
    }
};

// ============================================================
// 2. 步进模式 (Step Mode)
// ============================================================
pub const StepMode = enum {
    single,         // 单步（每次一步）
    continuous,     // 连续（自动推进）
    sync,           // 同步（等待对齐后推进）
    causal,         // 因果（按正负因果推进）
};

// ============================================================
// 3. 步进器核心 (Stepper Core)
// ============================================================
pub const Stepper = struct {
    allocator: std.mem.Allocator,
    current_step: u64,
    total_steps: u64,
    mode: StepMode,
    beats: std.ArrayList(Beat),
    is_running: bool,
    mapper: *mapper.Mapper,
    aligner: *aligner.Aligner,
    indexer: *indexer.Indexer,

    pub fn init(allocator: std.mem.Allocator, total_steps: u64, mode: StepMode, mapper_ptr: *mapper.Mapper, aligner_ptr: *aligner.Aligner, indexer_ptr: *indexer.Indexer) Stepper {
        return Stepper{
            .allocator = allocator,
            .current_step = 0,
            .total_steps = total_steps,
            .mode = mode,
            .beats = std.ArrayList(Beat).init(allocator),
            .is_running = false,
            .mapper = mapper_ptr,
            .aligner = aligner_ptr,
            .indexer = indexer_ptr,
        };
    }

    pub fn deinit(self: *Stepper) void {
        self.beats.deinit();
    }

    // 启动步进器
    pub fn start(self: *Stepper) void {
        self.is_running = true;
        self.current_step = 0;
    }

    // 停止步进器
    pub fn stop(self: *Stepper) void {
        self.is_running = false;
    }

    // 单步推进
    pub fn step(self: *Stepper) bool {
        if (!self.is_running or self.current_step >= self.total_steps) {
            return false;
        }

        self.current_step += 1;

        // 记录节拍
        const beat = Beat.init(self.current_step, @as(f64, @floatFromInt(self.current_step)) / @as(f64, @floatFromInt(self.total_steps)));
        self.beats.append(beat) catch {};

        // 根据模式执行不同逻辑
        switch (self.mode) {
            .single => {
                // 单步模式：每步都暂停
                self.is_running = false;
            },
            .continuous => {
                // 连续模式：自动推进，不暂停
            },
            .sync => {
                // 同步模式：检查对齐状态
                const result = self.mapper.performAlignment();
                if (!result.is_aligned) {
                    // 未对齐，暂停等待
                    self.is_running = false;
                }
            },
            .causal => {
                // 因果模式：按正负因果推进
                self.indexer.classifyAndCount();
                // 如果有负因果，暂停
                for (self.indexer.containers.items) |container| {
                    if (container.negativeRatio() > 0.5) {
                        self.is_running = false;
                        break;
                    }
                }
            },
        }

        return true;
    }

    // 获取当前步进位置
    pub fn getCurrentStep(self: *const Stepper) u64 {
        return self.current_step;
    }

    // 获取总步数
    pub fn getTotalSteps(self: *const Stepper) u64 {
        return self.total_steps;
    }

    // 获取进度（0-1）
    pub fn getProgress(self: *const Stepper) f64 {
        if (self.total_steps == 0) return 0;
        return @as(f64, @floatFromInt(self.current_step)) / @as(f64, @floatFromInt(self.total_steps));
    }

    // 是否完成
    pub fn isComplete(self: *const Stepper) bool {
        return self.current_step >= self.total_steps;
    }

    // 是否运行中
    pub fn isRunning(self: *const Stepper) bool {
        return self.is_running;
    }

    // 获取当前节拍
    pub fn getCurrentBeat(self: *const Stepper) ?Beat {
        if (self.beats.items.len == 0) return null;
        return self.beats.items[self.beats.items.len - 1];
    }

    // 获取所有节拍
    pub fn getBeats(self: *const Stepper) []const Beat {
        return self.beats.items;
    }

    // 重置步进器
    pub fn reset(self: *Stepper) void {
        self.current_step = 0;
        self.is_running = false;
        self.beats.clearRetainingCapacity();
    }

    // 设置模式
    pub fn setMode(self: *Stepper, mode: StepMode) void {
        self.mode = mode;
    }

    // 获取模式
    pub fn getMode(self: *const Stepper) StepMode {
        return self.mode;
    }
};

// ============================================================
// 4. 同步步进器 (Sync Stepper)
// ============================================================
// 等待对齐完成后才推进
pub const SyncStepper = struct {
    stepper: *Stepper,
    phase_set_a: *aligner.PhaseSet,
    phase_set_b: *aligner.PhaseSet,

    pub fn init(stepper: *Stepper, phase_set_a: *aligner.PhaseSet, phase_set_b: *aligner.PhaseSet) SyncStepper {
        return SyncStepper{
            .stepper = stepper,
            .phase_set_a = phase_set_a,
            .phase_set_b = phase_set_b,
        };
    }

    // 执行同步步进
    pub fn step(self: *SyncStepper) bool {
        // 先检查映射对齐
        const mapper_result = self.stepper.mapper.performAlignment();
        if (!mapper_result.is_aligned) {
            return false;
        }

        // 再检查相位对齐
        const phase_diffs = self.stepper.aligner.alignPhases(self.phase_set_a, self.phase_set_b);
        const all_locked = self.stepper.aligner.areAllPhasesLocked(phase_diffs);
        if (!all_locked) {
            return false;
        }

        // 对齐完成，推进
        return self.stepper.step();
    }
};

// ============================================================
// 5. 因果步进器 (Causal Stepper)
// ============================================================
// 按正负因果推进
pub const CausalStepper = struct {
    stepper: *Stepper,

    pub fn init(stepper: *Stepper) CausalStepper {
        return CausalStepper{
            .stepper = stepper,
        };
    }

    // 执行因果步进
    pub fn step(self: *CausalStepper) bool {
        // 先分类统计
        self.stepper.indexer.classifyAndCount();

        // 检查负因果比例
        for (self.stepper.indexer.containers.items) |container| {
            if (container.negativeRatio() > 0.5) {
                // 负因果过多，暂停
                return false;
            }
        }

        // 正因果占主导，推进
        return self.stepper.step();
    }
};

// ============================================================
// 测试
// ============================================================
test "stepper basic" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var ref = mapper.FixedReference.init(1, 10);
    var mapper_inst = mapper.Mapper.init(&ref, 1000);
    var aligner_inst = aligner.Aligner.default();
    var indexer_inst = indexer.Indexer.init(allocator);
    defer indexer_inst.deinit();

    var stepper = Stepper.init(allocator, 10, .single, &mapper_inst, &aligner_inst, &indexer_inst);
    defer stepper.deinit();

    stepper.start();

    // 单步模式：走一步就停
    const stepped = stepper.step();
    try std.testing.expect(stepped);
    try std.testing.expectEqual(@as(u64, 1), stepper.getCurrentStep());
    try std.testing.expect(!stepper.isRunning());
}

test "stepper continuous" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var ref = mapper.FixedReference.init(1, 10);
    var mapper_inst = mapper.Mapper.init(&ref, 1000);
    var aligner_inst = aligner.Aligner.default();
    var indexer_inst = indexer.Indexer.init(allocator);
    defer indexer_inst.deinit();

    var stepper = Stepper.init(allocator, 5, .continuous, &mapper_inst, &aligner_inst, &indexer_inst);
    defer stepper.deinit();

    stepper.start();

    // 连续模式：一直走到完成
    var count: u64 = 0;
    while (stepper.step()) {
        count += 1;
    }

    try std.testing.expectEqual(@as(u64, 5), count);
    try std.testing.expect(stepper.isComplete());
}

test "stepper progress" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var ref = mapper.FixedReference.init(1, 10);
    var mapper_inst = mapper.Mapper.init(&ref, 1000);
    var aligner_inst = aligner.Aligner.default();
    var indexer_inst = indexer.Indexer.init(allocator);
    defer indexer_inst.deinit();

    var stepper = Stepper.init(allocator, 10, .continuous, &mapper_inst, &aligner_inst, &indexer_inst);
    defer stepper.deinit();

    stepper.start();
    _ = stepper.step();
    _ = stepper.step();
    _ = stepper.step();

    try std.testing.expectEqual(@as(f64, 0.3), stepper.getProgress());
}
