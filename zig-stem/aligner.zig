// 对齐器 (Aligner)
// 功能：相位确认——在映射器完成等效对齐后，进一步确认相位关系
// 核心：相位差计算 + 相位锁定 + 相位调整

const std = @import("std");
const mapper = @import("mapper.zig");

// ============================================================
// 相位类型 (Phase Types)
// ============================================================
pub const PhaseType = enum {
    time_phase,         // 时间相位（节拍起点、周期位置）
    space_phase,        // 空间相位（位置坐标、方向）
    semantic_phase,     // 语义相位（含义空间中的位置）
    relation_phase,     // 关系相位（关系网络中的相对位置）
};

// 单个相位值（0-1，表示在一个周期内的位置）
pub const PhaseValue = struct {
    phase_type: PhaseType,
    value: f64,         // 0-1，表示相位位置

    pub fn init(phase_type: PhaseType, value: f64) PhaseValue {
        return PhaseValue{
            .phase_type = phase_type,
            .value = @mod(value, 1.0),  // 归一化到0-1
        };
    }
};

// 相位集合
pub const PhaseSet = struct {
    phases: [4]PhaseValue,  // 四种相位

    pub fn init() PhaseSet {
        return PhaseSet{
            .phases = .{
                PhaseValue.init(.time_phase, 0),
                PhaseValue.init(.space_phase, 0),
                PhaseValue.init(.semantic_phase, 0),
                PhaseValue.init(.relation_phase, 0),
            },
        };
    }

    pub fn setPhase(self: *PhaseSet, phase_type: PhaseType, value: f64) void {
        self.phases[@intFromEnum(phase_type)] = PhaseValue.init(phase_type, value);
    }

    pub fn getPhase(self: *const PhaseSet, phase_type: PhaseType) PhaseValue {
        return self.phases[@intFromEnum(phase_type)];
    }
};

// ============================================================
// 相位差 (Phase Difference)
// ============================================================
pub const PhaseDifference = struct {
    phase_type: PhaseType,
    diff: f64,          // 相位差（0-1，0.5表示完全相反）
    is_locked: bool,    // 是否锁定（在容差范围内）

    pub fn init(phase_type: PhaseType, diff: f64) PhaseDifference {
        return PhaseDifference{
            .phase_type = phase_type,
            .diff = @mod(diff, 1.0),
            .is_locked = false,
        };
    }
};

// ============================================================
// 对齐器核心 (Aligner Core)
// ============================================================
pub const Aligner = struct {
    tolerance: f64,     // 相位容差（0-1，如0.05表示5%）

    pub fn init(tol: f64) Aligner {
        return Aligner{
            .tolerance = tol,
        };
    }

    pub fn default() Aligner {
        return Aligner.init(0.05);  // 默认5%容差
    }

    pub fn calculatePhaseDiff(self: *const Aligner, phase_a: PhaseValue, phase_b: PhaseValue) PhaseDifference {
        const raw_diff = @abs(phase_a.value - phase_b.value);
        const cyclic_diff = @min(raw_diff, 1.0 - raw_diff);
        var result = PhaseDifference.init(phase_a.phase_type, cyclic_diff);
        result.is_locked = cyclic_diff <= self.tolerance;
        return result;
    }

    pub fn alignPhases(self: *const Aligner, set_a: *const PhaseSet, set_b: *const PhaseSet) [4]PhaseDifference {
        var results: [4]PhaseDifference = undefined;
        results[0] = self.calculatePhaseDiff(set_a.getPhase(.time_phase), set_b.getPhase(.time_phase));
        results[1] = self.calculatePhaseDiff(set_a.getPhase(.space_phase), set_b.getPhase(.space_phase));
        results[2] = self.calculatePhaseDiff(set_a.getPhase(.semantic_phase), set_b.getPhase(.semantic_phase));
        results[3] = self.calculatePhaseDiff(set_a.getPhase(.relation_phase), set_b.getPhase(.relation_phase));
        return results;
    }

    pub fn areAllPhasesLocked(_: *const Aligner, diffs: [4]PhaseDifference) bool {
        for (diffs) |diff| {
            if (!diff.is_locked) return false;
        }
        return true;
    }

    pub fn calculateConfidence(_: *const Aligner, diffs: [4]PhaseDifference) f64 {
        var total_diff: f64 = 0;
        for (diffs) |diff| {
            total_diff += diff.diff;
        }
        const avg_diff = total_diff / 4.0;
        return 1.0 - avg_diff;
    }

    pub fn suggestAdjustment(_: *const Aligner, phase_a: PhaseValue, phase_b: PhaseValue) struct { direction: f64, magnitude: f64 } {
        const raw_diff = phase_b.value - phase_a.value;
        // 取最短路径调整
        var adj = raw_diff;
        if (adj > 0.5) {
            adj -= 1.0;  // 反向调整更短
        } else if (adj < -0.5) {
            adj += 1.0;
        }
        return .{
            .direction = if (adj > 0) 1.0 else if (adj < 0) -1.0 else 0.0,
            .magnitude = @abs(adj),
        };
    }
};

// ============================================================
// 映射器-对齐器联动 (Mapper-Aligner Linkage)
// ============================================================
pub const MapperAligner = struct {
    mapper_ptr: *mapper.Mapper,
    aligner_ptr: *Aligner,

    pub fn init(mapper_ptr: *mapper.Mapper, aligner_ptr: *Aligner) MapperAligner {
        return MapperAligner{
            .mapper_ptr = mapper_ptr,
            .aligner_ptr = aligner_ptr,
        };
    }

    // 先映射对齐，再相位对齐
    pub fn fullAlign(self: *const MapperAligner, phase_a_set: *const PhaseSet, phase_b_set: *const PhaseSet) struct {
        mapper_result: mapper.AlignmentResult,
        aligner_results: [4]PhaseDifference,
        is_fully_aligned: bool,
        overall_confidence: f64,
    } {
        const mapper_result = self.mapper_ptr.performAlignment();
        const aligner_results = self.aligner_ptr.alignPhases(phase_a_set, phase_b_set);
        const all_phases_locked = self.aligner.areAllPhasesLocked(aligner_results);
        const phase_confidence = self.aligner.calculateConfidence(aligner_results);

        return .{
            .mapper_result = mapper_result,
            .aligner_results = aligner_results,
            .is_fully_aligned = mapper_result.is_aligned and all_phases_locked,
            .overall_confidence = (mapper_result.confidence + phase_confidence) / 2.0,
        };
    }
};

// ============================================================
// 测试
// ============================================================
test "aligner phase diff" {
    var aligner = Aligner.default();

    const phase_a = PhaseValue.init(.time_phase, 0.3);
    const phase_b = PhaseValue.init(.time_phase, 0.35);

    const diff = aligner.calculatePhaseDiff(phase_a, phase_b);

    try std.testing.expect(diff.is_locked);  // 5%差，在容差内
}

test "aligner phase diff cyclic" {
    var aligner = Aligner.default();

    // 0.95和0.05的差应该是0.1（循环最短路径）
    // 0.1 > 0.05容差，所以应该未锁定
    const phase_a = PhaseValue.init(.time_phase, 0.95);
    const phase_b = PhaseValue.init(.time_phase, 0.05);

    const diff = aligner.calculatePhaseDiff(phase_a, phase_b);

    // 0.1差 > 5%容差，应该未锁定
    try std.testing.expect(!diff.is_locked);
}

test "aligner phase diff cyclic locked" {
    // 使用0.9和0.1，差为0.2（循环最短路径也是0.2）
    // 0.2 > 0.05容差，应该未锁定
    var aligner = Aligner.default();

    const phase_a = PhaseValue.init(.time_phase, 0.9);
    const phase_b = PhaseValue.init(.time_phase, 0.1);

    const diff = aligner.calculatePhaseDiff(phase_a, phase_b);

    try std.testing.expect(!diff.is_locked);
}

test "aligner full alignment" {
    var ref = mapper.FixedReference.init(1, 10);
    var mapper_inst = mapper.Mapper.init(&ref, 1000);
    var aligner = Aligner.default();

    mapper_inst.startMeasurement();
    var i: u64 = 0;
    while (i < 20) : (i += 1) {
        mapper_inst.stepA();
        mapper_inst.stepB();
    }

    // 相同相位
    var set_a = PhaseSet.init();
    var set_b = PhaseSet.init();
    set_a.setPhase(.time_phase, 0.5);
    set_b.setPhase(.time_phase, 0.5);
    set_a.setPhase(.space_phase, 0.3);
    set_b.setPhase(.space_phase, 0.3);

    const phase_diffs = aligner.alignPhases(&set_a, &set_b);
    const all_locked = aligner.areAllPhasesLocked(phase_diffs);

    try std.testing.expect(all_locked);
}
