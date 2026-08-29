// 相位器 (Phaser)
// 功能：调整相位偏差
// 核心：相位检测 + 相位调整 + 相位锁定 + 相位同步

const std = @import("std");
const mapper = @import("mapper.zig");
const aligner = @import("aligner.zig");
const indexer = @import("indexer.zig");
const stepper = @import("stepper.zig");
const locator = @import("locator.zig");

// ============================================================
// 1. 相位状态 (Phase State)
// ============================================================
pub const PhaseState = struct {
    current_phase: f64,         // 当前相位 (0-1)
    target_phase: f64,          // 目标相位
    phase_velocity: f64,        // 相位变化速度
    is_locked: bool,            // 是否锁定

    pub fn init(current_phase: f64, target_phase: f64) PhaseState {
        return PhaseState{
            .current_phase = @mod(current_phase, 1.0),
            .target_phase = @mod(target_phase, 1.0),
            .phase_velocity = 0,
            .is_locked = false,
        };
    }
};

// ============================================================
// 2. 相位调整策略 (Phase Adjustment Strategy)
// ============================================================
pub const AdjustmentStrategy = enum {
    gradual,        // 渐进调整（小步慢调）
    immediate,      // 立即调整（一步到位）
    predictive,     // 预测调整（根据趋势预调）
    causal,         // 因果调整（按正负因果调）
};

// ============================================================
// 3. 相位器核心 (Phaser Core)
// ============================================================
pub const Phaser = struct {
    allocator: std.mem.Allocator,
    phase_states: std.ArrayList(PhaseState),
    strategy: AdjustmentStrategy,
    adjustment_rate: f64,       // 调整速率（0-1）
    tolerance: f64,             // 锁定容差

    pub fn init(allocator: std.mem.Allocator, strategy: AdjustmentStrategy, adjustment_rate: f64, tolerance: f64) Phaser {
        return Phaser{
            .allocator = allocator,
            .phase_states = std.ArrayList(PhaseState).init(allocator),
            .strategy = strategy,
            .adjustment_rate = adjustment_rate,
            .tolerance = tolerance,
        };
    }

    pub fn deinit(self: *Phaser) void {
        self.phase_states.deinit();
    }

    // 添加相位状态
    pub fn addPhaseState(self: *Phaser, state: PhaseState) !void {
        try self.phase_states.append(state);
    }

    // 设置调整策略
    pub fn setStrategy(self: *Phaser, strategy: AdjustmentStrategy) void {
        self.strategy = strategy;
    }

    // 获取相位差（考虑循环）
    pub fn getPhaseDifference(self: *const Phaser, index: u32) f64 {
        if (index >= self.phase_states.items.len) return 0;
        const state = self.phase_states.items[index];
        const raw_diff = state.target_phase - state.current_phase;
        // 取最短路径
        if (raw_diff > 0.5) {
            return raw_diff - 1.0;
        } else if (raw_diff < -0.5) {
            return raw_diff + 1.0;
        }
        return raw_diff;
    }

    // 检查是否锁定
    pub fn isLocked(self: *const Phaser, index: u32) bool {
        if (index >= self.phase_states.items.len) return false;
        const diff = self.getPhaseDifference(index);
        return @abs(diff) <= self.tolerance;
    }

    // 调整单个相位
    pub fn adjustPhase(self: *Phaser, index: u32) bool {
        if (index >= self.phase_states.items.len) return false;
        var state = &self.phase_states.items[index];

        const diff = self.getPhaseDifference(index);
        if (@abs(diff) <= self.tolerance) {
            state.is_locked = true;
            return true;
        }

        state.is_locked = false;

        switch (self.strategy) {
            .gradual => {
                // 渐进调整：按调整速率逐步靠近
                const adjustment = diff * self.adjustment_rate;
                state.current_phase = @mod(state.current_phase + adjustment, 1.0);
                state.phase_velocity = adjustment;
            },
            .immediate => {
                // 立即调整：一步到位
                state.current_phase = state.target_phase;
                state.phase_velocity = diff;
                state.is_locked = true;
            },
            .predictive => {
                // 预测调整：根据当前速度预测
                const predicted = state.current_phase + state.phase_velocity;
                const pred_diff = state.target_phase - predicted;
                const adjustment = pred_diff * self.adjustment_rate;
                state.current_phase = @mod(state.current_phase + adjustment, 1.0);
                state.phase_velocity = adjustment;
            },
            .causal => {
                // 因果调整：正因果加速，负因果减速
                if (diff > 0) {
                    // 正因果：加速正向调整
                    const adjustment = diff * self.adjustment_rate * 1.5;
                    state.current_phase = @mod(state.current_phase + adjustment, 1.0);
                    state.phase_velocity = adjustment;
                } else {
                    // 负因果：减速反向调整
                    const adjustment = diff * self.adjustment_rate * 0.5;
                    state.current_phase = @mod(state.current_phase + adjustment, 1.0);
                    state.phase_velocity = adjustment;
                }
            },
        }

        return state.is_locked;
    }

    // 调整所有相位
    pub fn adjustAll(self: *Phaser) u32 {
        var locked_count: u32 = 0;
        for (0..self.phase_states.items.len) |i| {
            const locked = self.adjustPhase(@intCast(i));
            if (locked) {
                locked_count += 1;
            }
        }
        return locked_count;
    }

    // 检查是否全部锁定
    pub fn areAllLocked(self: *const Phaser) bool {
        for (0..self.phase_states.items.len) |i| {
            if (!self.isLocked(@intCast(i))) return false;
        }
        return true;
    }

    // 获取锁定比例
    pub fn getLockRatio(self: *const Phaser) f64 {
        if (self.phase_states.items.len == 0) return 0;
        var locked: f64 = 0;
        for (0..self.phase_states.items.len) |i| {
            if (self.isLocked(@intCast(i))) locked += 1;
        }
        return locked / @as(f64, @floatFromInt(self.phase_states.items.len));
    }

    // 获取指定索引的当前相位
    pub fn getCurrentPhase(self: *const Phaser, index: u32) f64 {
        if (index >= self.phase_states.items.len) return 0;
        return self.phase_states.items[index].current_phase;
    }

    // 获取指定索引的目标相位
    pub fn getTargetPhase(self: *const Phaser, index: u32) f64 {
        if (index >= self.phase_states.items.len) return 0;
        return self.phase_states.items[index].target_phase;
    }
};

// ============================================================
// 4. 相位同步器 (Phase Synchronizer)
// ============================================================
pub const PhaseSynchronizer = struct {
    phaser: *Phaser,
    sync_target: f64,

    pub fn init(phaser: *Phaser, sync_target: f64) PhaseSynchronizer {
        return PhaseSynchronizer{
            .phaser = phaser,
            .sync_target = @mod(sync_target, 1.0),
        };
    }

    // 同步所有相位到目标
    pub fn synchronizeAll(self: *PhaseSynchronizer) u32 {
        for (0..self.phaser.phase_states.items.len) |i| {
            self.phaser.phase_states.items[i].target_phase = self.sync_target;
        }
        return self.phaser.adjustAll();
    }

    // 同步两个相位
    pub fn synchronizePair(self: *PhaseSynchronizer, index_a: u32, index_b: u32) bool {
        if (index_a >= self.phaser.phase_states.items.len or index_b >= self.phaser.phase_states.items.len) {
            return false;
        }

        // 将两个相位的平均值作为目标
        const avg = (self.phaser.phase_states.items[index_a].current_phase + self.phaser.phase_states.items[index_b].current_phase) / 2.0;
        self.phaser.phase_states.items[index_a].target_phase = avg;
        self.phaser.phase_states.items[index_b].target_phase = avg;

        _ = self.phaser.adjustPhase(index_a);
        _ = self.phaser.adjustPhase(index_b);

        return self.phaser.isLocked(index_a) and self.phaser.isLocked(index_b);
    }
};

// ============================================================
// 5. 相位锁定检测器 (Phase Lock Detector)
// ============================================================
pub const PhaseLockDetector = struct {
    phaser: *Phaser,
    lock_history: std.ArrayList(bool),
    history_size: u32,

    pub fn init(phaser: *Phaser, history_size: u32) PhaseLockDetector {
        return PhaseLockDetector{
            .phaser = phaser,
            .lock_history = std.ArrayList(bool).init(phaser.allocator),
            .history_size = history_size,
        };
    }

    pub fn deinit(self: *PhaseLockDetector) void {
        self.lock_history.deinit();
    }

    // 更新锁定历史
    pub fn update(self: *PhaseLockDetector) !void {
        const all_locked = self.phaser.areAllLocked();
        if (self.lock_history.items.len >= self.history_size) {
            // 移除最旧的数据
            var temp = std.ArrayList(bool).init(self.phaser.allocator);
            defer temp.deinit();
            _ = try temp.append(self.lock_history.items[1..]);
            self.lock_history = temp;
        }
        try self.lock_history.append(all_locked);
    }

    // 检查是否稳定锁定（历史记录中全部为true）
    pub fn isStableLocked(self: *const PhaseLockDetector) bool {
        if (self.lock_history.items.len == 0) return false;
        for (self.lock_history.items) |locked| {
            if (!locked) return false;
        }
        return true;
    }

    // 获取锁定稳定性（历史记录中true的比例）
    pub fn getStability(self: *const PhaseLockDetector) f64 {
        if (self.lock_history.items.len == 0) return 0;
        var true_count: f64 = 0;
        for (self.lock_history.items) |locked| {
            if (locked) true_count += 1;
        }
        return true_count / @as(f64, @floatFromInt(self.lock_history.items.len));
    }
};

// ============================================================
// 测试
// ============================================================
test "phaser gradual adjustment" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var phaser = Phaser.init(allocator, .gradual, 0.1, 0.05);
    defer phaser.deinit();

    // 当前相位0.0，目标相位0.5
    try phaser.addPhaseState(PhaseState.init(0.0, 0.5));

    // 渐进调整，需要多步才能接近目标
    var i: u32 = 0;
    while (i < 50 and !phaser.isLocked(0)) {
        _ = phaser.adjustPhase(0);
        i += 1;
    }

    // 应该在若干步后锁定
    try std.testing.expect(phaser.isLocked(0));
}

test "phaser immediate adjustment" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var phaser = Phaser.init(allocator, .immediate, 1.0, 0.05);
    defer phaser.deinit();

    try phaser.addPhaseState(PhaseState.init(0.0, 0.5));

    // 立即调整，一步到位
    const locked = phaser.adjustPhase(0);
    try std.testing.expect(locked);
    try std.testing.expectEqual(@as(f64, 0.5), phaser.getCurrentPhase(0));
}

test "phaser cyclic difference" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var phaser = Phaser.init(allocator, .gradual, 0.1, 0.05);
    defer phaser.deinit();

    // 相位0.95和目标相位0.05，循环差应该是0.1
    try phaser.addPhaseState(PhaseState.init(0.95, 0.05));

    const diff = phaser.getPhaseDifference(0);
    try std.testing.expectApproxEqAbs(@as(f64, 0.1), diff, 0.001);
}

test "phase synchronizer" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var phaser = Phaser.init(allocator, .immediate, 1.0, 0.05);
    defer phaser.deinit();

    try phaser.addPhaseState(PhaseState.init(0.2, 0.0));
    try phaser.addPhaseState(PhaseState.init(0.8, 0.0));

    var sync = PhaseSynchronizer.init(&phaser, 0.5);
    _ = sync.synchronizeAll();

    try std.testing.expect(phaser.isLocked(0));
    try std.testing.expect(phaser.isLocked(1));
}
