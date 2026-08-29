// 频率器 (Frequencer)
// 功能：同步节律
// 核心：频率计算 + 节律同步 + 频率锁定 + 频率调整

const std = @import("std");
const mapper = @import("mapper.zig");
const aligner = @import("aligner.zig");
const indexer = @import("indexer.zig");
const stepper = @import("stepper.zig");
const locator = @import("locator.zig");
const phaser = @import("phaser.zig");
const angler = @import("angler.zig");

// ============================================================
// 1. 频率 (Frequency)
// ============================================================
pub const Frequency = struct {
    hz: f64,                // 频率（赫兹）

    pub fn init(hz: f64) Frequency {
        return Frequency{ .hz = hz };
    }

    pub fn fromPeriod(period_s: f64) Frequency {
        return Frequency.init(1.0 / period_s);
    }

    pub fn toPeriod(self: *const Frequency) f64 {
        return 1.0 / self.hz;
    }

    // 计算频率差
    pub fn difference(self: *const Frequency, other: *const Frequency) f64 {
        return @abs(self.hz - other.hz);
    }

    // 计算频率比
    pub fn ratio(self: *const Frequency, other: *const Frequency) f64 {
        if (other.hz == 0) return 0;
        return self.hz / other.hz;
    }
};

// ============================================================
// 2. 节律 (Rhythm)
// ============================================================
pub const Rhythm = struct {
    frequency: Frequency,
    amplitude: f64,         // 振幅
    phase: f64,             // 相位 (0-1)

    pub fn init(hz: f64, amplitude: f64, phase: f64) Rhythm {
        return Rhythm{
            .frequency = Frequency.init(hz),
            .amplitude = amplitude,
            .phase = @mod(phase, 1.0),
        };
    }
};

// ============================================================
// 3. 频率器核心 (Frequencer Core)
// ============================================================
pub const Frequencer = struct {
    allocator: std.mem.Allocator,
    rhythms: std.ArrayList(Rhythm),
    tolerance_hz: f64,

    pub fn init(allocator: std.mem.Allocator, tolerance_hz: f64) Frequencer {
        return Frequencer{
            .allocator = allocator,
            .rhythms = std.ArrayList(Rhythm).init(allocator),
            .tolerance_hz = tolerance_hz,
        };
    }

    pub fn deinit(self: *Frequencer) void {
        self.rhythms.deinit();
    }

    // 添加节律
    pub fn addRhythm(self: *Frequencer, rhythm: Rhythm) !void {
        try self.rhythms.append(rhythm);
    }

    // 计算频率差
    pub fn calculateFreqDiff(self: *const Frequencer, index_a: u32, index_b: u32) f64 {
        if (index_a >= self.rhythms.items.len or index_b >= self.rhythms.items.len) {
            return 0;
        }
        return self.rhythms.items[index_a].frequency.difference(&self.rhythms.items[index_b].frequency);
    }

    // 检查频率是否锁定
    pub fn isFreqLocked(self: *const Frequencer, index_a: u32, index_b: u32) bool {
        const diff = self.calculateFreqDiff(index_a, index_b);
        return diff <= self.tolerance_hz;
    }

    // 查找最接近的频率
    pub fn findClosestFrequency(self: *const Frequencer, target_hz: f64) ?u32 {
        if (self.rhythms.items.len == 0) return null;

        var min_diff = std.math.inf(f64);
        var closest_index: ?u32 = null;

        for (self.rhythms.items, 0..) |rhythm, i| {
            const diff = @abs(rhythm.frequency.hz - target_hz);
            if (diff < min_diff) {
                min_diff = diff;
                closest_index = @intCast(i);
            }
        }

        return closest_index;
    }

    // 计算平均频率
    pub fn calculateAverageFrequency(self: *const Frequencer) f64 {
        if (self.rhythms.items.len == 0) return 0;

        var sum: f64 = 0;
        for (self.rhythms.items) |rhythm| {
            sum += rhythm.frequency.hz;
        }

        return sum / @as(f64, @floatFromInt(self.rhythms.items.len));
    }

    // 计算频率离散度
    pub fn calculateFreqDispersion(self: *const Frequencer) f64 {
        if (self.rhythms.items.len == 0) return 0;

        const avg = self.calculateAverageFrequency();
        var total_diff: f64 = 0;

        for (self.rhythms.items) |rhythm| {
            total_diff += @abs(rhythm.frequency.hz - avg);
        }

        return total_diff / @as(f64, @floatFromInt(self.rhythms.items.len));
    }

    // 获取指定索引的频率
    pub fn getFrequency(self: *const Frequencer, index: u32) f64 {
        if (index >= self.rhythms.items.len) return 0;
        return self.rhythms.items[index].frequency.hz;
    }

    // 设置指定索引的频率
    pub fn setFrequency(self: *Frequencer, index: u32, hz: f64) void {
        if (index >= self.rhythms.items.len) return;
        self.rhythms.items[index].frequency.hz = hz;
    }
};

// ============================================================
// 4. 频率同步器 (Frequency Synchronizer)
// ============================================================
pub const FrequencySynchronizer = struct {
    frequencer: *Frequencer,
    target_hz: f64,

    pub fn init(frequencer: *Frequencer, target_hz: f64) FrequencySynchronizer {
        return FrequencySynchronizer{
            .frequencer = frequencer,
            .target_hz = target_hz,
        };
    }

    // 同步所有频率到目标
    pub fn synchronizeAll(self: *FrequencySynchronizer) void {
        for (0..self.frequencer.rhythms.items.len) |i| {
            self.frequencer.rhythms.items[i].frequency.hz = self.target_hz;
        }
    }

    // 渐进同步（按速率逐步靠近）
    pub fn gradualSynchronize(self: *FrequencySynchronizer, rate: f64) u32 {
        var synced_count: u32 = 0;

        for (0..self.frequencer.rhythms.items.len) |i| {
            const current_hz = self.frequencer.rhythms.items[i].frequency.hz;
            const diff = self.target_hz - current_hz;

            if (@abs(diff) <= self.frequencer.tolerance_hz) {
                synced_count += 1;
                continue;
            }

            // 按速率调整
            const adjustment = diff * rate;
            self.frequencer.rhythms.items[i].frequency.hz = current_hz + adjustment;

            if (@abs(self.target_hz - self.frequencer.rhythms.items[i].frequency.hz) <= self.frequencer.tolerance_hz) {
                synced_count += 1;
            }
        }

        return synced_count;
    }
};

// ============================================================
// 5. 频率锁定检测器 (Frequency Lock Detector)
// ============================================================
pub const FrequencyLockDetector = struct {
    frequencer: *Frequencer,
    lock_history: std.ArrayList(bool),
    history_size: u32,

    pub fn init(frequencer: *Frequencer, history_size: u32) FrequencyLockDetector {
        return FrequencyLockDetector{
            .frequencer = frequencer,
            .lock_history = std.ArrayList(bool).init(frequencer.allocator),
            .history_size = history_size,
        };
    }

    pub fn deinit(self: *FrequencyLockDetector) void {
        self.lock_history.deinit();
    }

    // 更新锁定历史
    pub fn update(self: *FrequencyLockDetector) !void {
        const all_locked = self.areAllLocked();
        if (self.lock_history.items.len >= self.history_size) {
            var temp = std.ArrayList(bool).init(self.frequencer.allocator);
            defer temp.deinit();
            for (self.lock_history.items[1..]) |item| {
                try temp.append(item);
            }
            self.lock_history = temp;
        }
        try self.lock_history.append(all_locked);
    }

    // 检查是否全部锁定
    fn areAllLocked(self: *const FrequencyLockDetector) bool {
        if (self.frequencer.rhythms.items.len < 2) return true;

        const first_hz = self.frequencer.rhythms.items[0].frequency.hz;
        for (self.frequencer.rhythms.items[1..]) |rhythm| {
            if (@abs(rhythm.frequency.hz - first_hz) > self.frequencer.tolerance_hz) {
                return false;
            }
        }
        return true;
    }

    // 检查是否稳定锁定
    pub fn isStableLocked(self: *const FrequencyLockDetector) bool {
        if (self.lock_history.items.len == 0) return false;
        for (self.lock_history.items) |locked| {
            if (!locked) return false;
        }
        return true;
    }

    // 获取锁定稳定性
    pub fn getStability(self: *const FrequencyLockDetector) f64 {
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
test "frequencer basic" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var frequencer = Frequencer.init(allocator, 0.5);
    defer frequencer.deinit();

    try frequencer.addRhythm(Rhythm.init(440.0, 1.0, 0.0));
    try frequencer.addRhythm(Rhythm.init(880.0, 0.8, 0.5));
    try frequencer.addRhythm(Rhythm.init(220.0, 0.6, 0.25));

    // 计算频率差
    const diff_01 = frequencer.calculateFreqDiff(0, 1);
    try std.testing.expectApproxEqAbs(@as(f64, 440.0), diff_01, 0.001);

    const diff_02 = frequencer.calculateFreqDiff(0, 2);
    try std.testing.expectApproxEqAbs(@as(f64, 220.0), diff_02, 0.001);
}

test "frequencer closest frequency" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var frequencer = Frequencer.init(allocator, 0.5);
    defer frequencer.deinit();

    try frequencer.addRhythm(Rhythm.init(100.0, 1.0, 0.0));
    try frequencer.addRhythm(Rhythm.init(200.0, 0.8, 0.5));
    try frequencer.addRhythm(Rhythm.init(300.0, 0.6, 0.25));

    // 查找最接近250Hz的频率（200或300都可以，取第一个找到的）
    const closest = frequencer.findClosestFrequency(250.0);
    // 250在200和300正中间，算法返回先遇到的（200，index 1）
    try std.testing.expectEqual(@as(u32, 1), closest.?);
}

test "frequencer average" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var frequencer = Frequencer.init(allocator, 0.5);
    defer frequencer.deinit();

    try frequencer.addRhythm(Rhythm.init(100.0, 1.0, 0.0));
    try frequencer.addRhythm(Rhythm.init(200.0, 0.8, 0.5));
    try frequencer.addRhythm(Rhythm.init(300.0, 0.6, 0.25));

    const avg = frequencer.calculateAverageFrequency();
    try std.testing.expectApproxEqAbs(@as(f64, 200.0), avg, 0.001);
}

test "frequency synchronizer" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var frequencer = Frequencer.init(allocator, 0.5);
    defer frequencer.deinit();

    try frequencer.addRhythm(Rhythm.init(100.0, 1.0, 0.0));
    try frequencer.addRhythm(Rhythm.init(200.0, 0.8, 0.5));
    try frequencer.addRhythm(Rhythm.init(300.0, 0.6, 0.25));

    var sync = FrequencySynchronizer.init(&frequencer, 440.0);
    sync.synchronizeAll();

    // 所有频率应该都同步到440Hz
    for (0..3) |i| {
        try std.testing.expectApproxEqAbs(@as(f64, 440.0), frequencer.getFrequency(@intCast(i)), 0.001);
    }
}
