// 映射器 (Mapper)
// 功能：跨主权度量系统的等效对齐
// 核心：报数人机制 + 四维度丈量 + 容差锁定
// 语义对齐层：留空接口，等人类定义

const std = @import("std");

// ============================================================
// 1. 报数人 (Fixed Reference)
// ============================================================
// 不能动的基准参考系，所有人排在后面报数
pub const FixedReference = struct {
    id: u32,
    base_value: u64,
    is_locked: bool = true,

    pub fn init(id: u32, base_value: u64) FixedReference {
        return FixedReference{
            .id = id,
            .base_value = base_value,
            .is_locked = true,
        };
    }

    pub fn getValue(self: *const FixedReference) u64 {
        return self.base_value;
    }
};

// ============================================================
// 2. 测量器 (Measurer)
// ============================================================
// 负责：计数 + 计时
pub const Measurer = struct {
    step_count: u64,
    time_window_ns: u64,
    start_time: i128,

    pub fn init(time_window_ms: u64) Measurer {
        return Measurer{
            .step_count = 0,
            .time_window_ns = time_window_ms * 1_000_000,
            .start_time = 0,
        };
    }

    pub fn start(self: *Measurer) void {
        self.start_time = std.time.nanoTimestamp();
        self.step_count = 0;
    }

    pub fn step(self: *Measurer) void {
        self.step_count += 1;
    }

    pub fn isWindowExpired(self: *const Measurer) bool {
        const now = std.time.nanoTimestamp();
        const elapsed = @abs(now - self.start_time);
        return @as(u64, @intCast(@as(i64, @truncate(elapsed)))) >= self.time_window_ns;
    }

    pub fn getStepCount(self: *const Measurer) u64 {
        return self.step_count;
    }
};

// ============================================================
// 3. 对齐维度 (Alignment Dimensions)
// ============================================================
// 四个维度：长度距离、速度、时间、幅度
pub const AlignmentDimensions = struct {
    length_distance: f64,
    speed: f64,
    time_phase: f64,
    amplitude: f64,

    pub fn init() AlignmentDimensions {
        return AlignmentDimensions{
            .length_distance = 0,
            .speed = 0,
            .time_phase = 0,
            .amplitude = 0,
        };
    }
};

// ============================================================
// 4. 容差窗口 (Tolerance Window)
// ============================================================
// "最小限定范畴"——不需要数学精确，只要在容差内就算对齐
pub const ToleranceWindow = struct {
    length_tolerance: f64,
    speed_tolerance: f64,
    phase_tolerance: f64,
    amplitude_tolerance: f64,

    pub fn init(length_tol: f64, speed_tol: f64, phase_tol: f64, amp_tol: f64) ToleranceWindow {
        return ToleranceWindow{
            .length_tolerance = length_tol,
            .speed_tolerance = speed_tol,
            .phase_tolerance = phase_tol,
            .amplitude_tolerance = amp_tol,
        };
    }

    pub fn default() ToleranceWindow {
        return ToleranceWindow.init(0.05, 0.05, 0.05, 0.05);
    }
};

// ============================================================
// 5. 维度对 (Dimension Pair)
// ============================================================
pub const DimensionPair = struct {
    a: AlignmentDimensions,
    b: AlignmentDimensions,
};

// ============================================================
// 6. 对齐结果 (Alignment Result)
// ============================================================
pub const AlignmentResult = struct {
    is_aligned: bool,
    length_match: bool,
    speed_match: bool,
    phase_match: bool,
    amplitude_match: bool,
    confidence: f64,

    pub fn init() AlignmentResult {
        return AlignmentResult{
            .is_aligned = false,
            .length_match = false,
            .speed_match = false,
            .phase_match = false,
            .amplitude_match = false,
            .confidence = 0,
        };
    }
};

// ============================================================
// 6. 语义对齐接口 (Semantic Alignment Interface)
// ============================================================
// 留空接口——只有人类有定义权
// AI没有感知、感受、品味、比喻、穿透现象看本质的能力
// 所以这里只定义接口，不实现
pub const SemanticAlignmentInterface = struct {
    // 判断两个符号是否代表同一个概念
    // 实现者：人类（第三方中介）
    pub fn isSameConcept(symbol_a: []const u8, symbol_b: []const u8) callconv(.C) bool {
        _ = symbol_a;
        _ = symbol_b;
        return false;
    }

    // 判断两个发音是否代表同一件事
    // 实现者：人类（语言翻译者）
    pub fn isSameMeaning(phonetic_a: []const u8, phonetic_b: []const u8) callconv(.C) bool {
        _ = phonetic_a;
        _ = phonetic_b;
        return false;
    }

    // 定义某个符号的含义
    // 实现者：人类（唯一有定义权的主体）
    pub fn defineSymbol(symbol: []const u8, meaning: []const u8) callconv(.C) bool {
        _ = symbol;
        _ = meaning;
        return false;
    }
};

// ============================================================
// 7. 映射器核心 (Mapper Core)
// ============================================================
pub const Mapper = struct {
    reference: *FixedReference,
    measurer_a: Measurer,
    measurer_b: Measurer,
    tolerance: ToleranceWindow,

    pub fn init(reference: *FixedReference, time_window_ms: u64) Mapper {
        return Mapper{
            .reference = reference,
            .measurer_a = Measurer.init(time_window_ms),
            .measurer_b = Measurer.init(time_window_ms),
            .tolerance = ToleranceWindow.default(),
        };
    }

    pub fn startMeasurement(self: *Mapper) void {
        self.measurer_a.start();
        self.measurer_b.start();
    }

    pub fn stepA(self: *Mapper) void {
        self.measurer_a.step();
    }

    pub fn stepB(self: *Mapper) void {
        self.measurer_b.step();
    }

    pub fn calculateDimensions(self: *const Mapper) DimensionPair {
        const steps_a = self.measurer_a.getStepCount();
        const steps_b = self.measurer_b.getStepCount();

        const base = self.reference.getValue();

        const length_a = @as(f64, @floatFromInt(steps_a)) * @as(f64, @floatFromInt(base));
        const length_b = @as(f64, @floatFromInt(steps_b)) * @as(f64, @floatFromInt(base));

        const speed_a = if (self.measurer_a.time_window_ns > 0)
            @as(f64, @floatFromInt(steps_a)) / @as(f64, @floatFromInt(self.measurer_a.time_window_ns)) * 1_000_000_000.0
        else 0;
        const speed_b = if (self.measurer_b.time_window_ns > 0)
            @as(f64, @floatFromInt(steps_b)) / @as(f64, @floatFromInt(self.measurer_b.time_window_ns)) * 1_000_000_000.0
        else 0;

        const phase_a = if (base > 0) @as(f64, @floatFromInt(steps_a % base)) / @as(f64, @floatFromInt(base)) else 0;
        const phase_b = if (base > 0) @as(f64, @floatFromInt(steps_b % base)) / @as(f64, @floatFromInt(base)) else 0;

        const amp_a = @as(f64, @floatFromInt(steps_a));
        const amp_b = @as(f64, @floatFromInt(steps_b));

        return DimensionPair{
            .a = AlignmentDimensions{
                .length_distance = length_a,
                .speed = speed_a,
                .time_phase = phase_a,
                .amplitude = amp_a,
            },
            .b = AlignmentDimensions{
                .length_distance = length_b,
                .speed = speed_b,
                .time_phase = phase_b,
                .amplitude = amp_b,
            },
        };
    }

    pub fn performAlignment(self: *const Mapper) AlignmentResult {
        const dims = self.calculateDimensions();
        var result = AlignmentResult.init();

        const length_ratio: f64 = if (dims.a.length_distance > 0 and dims.b.length_distance > 0)
            @min(dims.a.length_distance, dims.b.length_distance) / @max(dims.a.length_distance, dims.b.length_distance)
        else if (dims.a.length_distance == 0 and dims.b.length_distance == 0) 1.0 else 0.0;
        result.length_match = length_ratio >= (1.0 - self.tolerance.length_tolerance);

        const speed_ratio: f64 = if (dims.a.speed > 0 and dims.b.speed > 0)
            @min(dims.a.speed, dims.b.speed) / @max(dims.a.speed, dims.b.speed)
        else if (dims.a.speed == 0 and dims.b.speed == 0) 1.0 else 0.0;
        result.speed_match = speed_ratio >= (1.0 - self.tolerance.speed_tolerance);

        const phase_diff = @abs(dims.a.time_phase - dims.b.time_phase);
        result.phase_match = phase_diff <= self.tolerance.phase_tolerance;

        const amp_ratio: f64 = if (dims.a.amplitude > 0 and dims.b.amplitude > 0)
            @min(dims.a.amplitude, dims.b.amplitude) / @max(dims.a.amplitude, dims.b.amplitude)
        else if (dims.a.amplitude == 0 and dims.b.amplitude == 0) 1.0 else 0.0;
        result.amplitude_match = amp_ratio >= (1.0 - self.tolerance.amplitude_tolerance);

        result.is_aligned = result.length_match and result.speed_match and result.phase_match and result.amplitude_match;

        const length_val: f64 = if (result.length_match) 1.0 else 0.0;
        const speed_val: f64 = if (result.speed_match) 1.0 else 0.0;
        const phase_val: f64 = if (result.phase_match) 1.0 else 0.0;
        const amp_val: f64 = if (result.amplitude_match) 1.0 else 0.0;
        result.confidence = (length_val + speed_val + phase_val + amp_val) / 4.0;

        return result;
    }
};

// ============================================================
// 测试
// ============================================================
test "mapper basic alignment" {
    var ref = FixedReference.init(1, 10);
    var mapper = Mapper.init(&ref, 1000);

    mapper.startMeasurement();

    var i: u64 = 0;
    while (i < 23) : (i += 1) {
        mapper.stepA();
    }

    i = 0;
    while (i < 26) : (i += 1) {
        mapper.stepB();
    }

    const result = mapper.performAlignment();

    std.debug.print("\n=== Mapper Alignment Test ===\n", .{});
    std.debug.print("A steps: 23, B steps: 26\n", .{});
    std.debug.print("Base value: 10\n", .{});
    std.debug.print("Length match: {}\n", .{result.length_match});
    std.debug.print("Speed match: {}\n", .{result.speed_match});
    std.debug.print("Phase match: {}\n", .{result.phase_match});
    std.debug.print("Amplitude match: {}\n", .{result.amplitude_match});
    std.debug.print("Overall aligned: {}\n", .{result.is_aligned});
    std.debug.print("Confidence: {d:.2}\n", .{result.confidence});
}

test "mapper exact match" {
    var ref = FixedReference.init(1, 10);
    var mapper = Mapper.init(&ref, 1000);

    mapper.startMeasurement();

    var i: u64 = 0;
    while (i < 20) : (i += 1) {
        mapper.stepA();
        mapper.stepB();
    }

    const result = mapper.performAlignment();

    try std.testing.expect(result.is_aligned);
    try std.testing.expectEqual(@as(f64, 1.0), result.confidence);
}
