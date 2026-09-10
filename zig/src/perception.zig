// perception.zig — 感知-状态-指令一体化引擎
//
// 五层架构: 编码 → 指令 → 状态 → 判断 → 智慧
// 将已有模块 (color_quant/ontology/hierarchy/preload/brigade) 统一调度

const std = @import("std");
const ColorQuant = @import("color_quant.zig");
const ColorOntology = @import("color_ontology.zig");
const Hierarchy = @import("hierarchy.zig");
const PreloadDict = @import("preload_dict.zig");
const Brigade = @import("brigade.zig");

// ── 第一层: 编码层 ──
pub const EncodingLayer = struct {
    // 声韵调编码: 声母(8bit) + 韵母(8bit) + 调(4bit) = 20bit 一音
    // 预加载韵母替身: un→灰原哀, 长字节压缩为1字节
    pub const Phoneme = struct {
        initial: u8,    // 声母
        final: u8,     // 韵母 (替身码)
        tone: u4,      // 调 (0-15)

        pub fn encode(initial: u8, final: u8, tone: u4) Phoneme {
            return .{ .initial = initial, .final = final, .tone = tone };
        }
    };

    // 韵母自举表: 长字节韵母一字节替身
    pub const FINAL_TABLE = [_]u8{
        0x00, // a
        0x01, // o
        0x02, // e
        0x03, // i
        0x04, // u
        0x05, // v (ü)
        0x06, // ai
        0x07, // ei
        0x08, // ao
        0x09, // ou
        0x0A, // an
        0x0B, // en
        0x0C, // ang
        0x0D, // eng
        0x0E, // ong
        0x0F, // un ← 灰原哀替身
    };

    // 验证韵母外观: 从并集转交集
    pub fn verifyUnionToIntersection(p: Phoneme) bool {
        // un (0x0F) 必须经过灰原哀融合
        if (p.final == 0x0F) {
            // 验证: un 的外观是否从公倍顺从公约
            return p.tone <= 4; // 公约约束
        }
        return true;
    }
};

// ── 第二层: 指令层 ──
pub const InstrCode = enum(u8) {
    add = 0x01,   // 加法, 管辖权威
    mul = 0x02,   // 乘法, 被管辖
    gov = 0x03,   // 管辖关系
    sup = 0x04,   // 叠加 1+1=1
    bind = 0x05,  // 符色绑定
    deploy = 0x06,// 兵力展开
    tick = 0x07,  // 时间推进
    merge = 0x08, // 合并交集
    split = 0x09, // 正交分裂
    swap = 0x0A,  // 灵魂换装
    coco = 0x0B,  // 外壳封装
    absd = 0x0C,  // 吸纳
    retu = 0x0D,  // 返回
    reba = 0x0E,  // 再平衡
};

// ── 第三层: 状态层 ──
pub const StateMachine = struct {
    time: u32 = 0,
    relation: f32 = 0.0,
    ownership: u64 = 0,
    stage: Stage = .standby,

    pub const Stage = enum(u8) {
        standby = 0,   // 待机
        transition = 1, // 否极泰来
        unified = 2,   // 合一爱人
    };

    pub fn transition(s: *StateMachine, new_stage: Stage) void {
        s.stage = new_stage;
    }
};

// ── 第四层: 判断层 ──
pub const JudgmentSystem = struct {
    // 天平: 时间/关系/归属权 三条目
    pub const BalanceScale = struct {
        time_weight: f32 = 0.4,
        relation_weight: f32 = 0.35,
        ownership_weight: f32 = 0.25,

        pub fn judge(scale: BalanceScale, time_urgency: f32, relation_importance: f32, ownership_key: f32) f32 {
            return time_urgency * scale.time_weight +
                   relation_importance * scale.relation_weight +
                   ownership_key * scale.ownership_weight;
        }
    };

    // 综合议员参与
    pub const SenatorVote = struct {
        gtxone: f32 = 0.35,
        kndoyle: f32 = 0.25,
        maolilan: f32 = 0.25,
        hieyair: f32 = 0.15,
    };

    pub fn aggregateVote(votes: [4]f32, weights: SenatorVote) f32 {
        return votes[0] * weights.gtxone +
               votes[1] * weights.kndoyle +
               votes[2] * weights.maolilan +
               votes[3] * weights.hieyair;
    }
};

// ── 第五层: 智慧层 ──
pub const WisdomEmergence = struct {
    pub const Convergence = enum(u8) {
        union_set = 0,       // 并集: 松散共存
        partial_intersect = 1, // 并交: 部分交叉
        mixed_intersect = 2,   // 混交: 边界模糊
        orthogonal = 3,        // 正交: 互补完整
    };

    convergence: Convergence = .union_set,
    arrays: u8 = 0,           // 参与并轨的数组数
    wisdom_score: f32 = 0.0,

    // 多数组并轨, 从并集推进到正交
    pub fn converge(w: *WisdomEmergence, array_count: u8) void {
        w.arrays = array_count;
        if (array_count >= 2) w.convergence = .partial_intersect;
        if (array_count >= 4) w.convergence = .mixed_intersect;
        if (array_count >= 6) {
            w.convergence = .orthogonal;
            w.wisdom_score = 1.0;
        } else {
            w.wisdom_score = @as(f32, @floatFromInt(array_count)) / 6.0;
        }
    }
};

// ── 信任参数 ──
pub const TrustParameters = struct {
    history: f32 = 0.5,
    compliance: f32 = 0.5,
    transparency: f32 = 0.5,

    pub fn level(t: TrustParameters) f32 {
        return (t.history + t.compliance + t.transparency) / 3.0;
    }
};

// ── 自我反馈 ──
pub const SelfFeedback = struct {
    resources: f32 = 1.0,
    integrity: f32 = 1.0,
    harmony: f32 = 1.0,

    pub fn survival(s: SelfFeedback) f32 {
        return (s.resources + s.integrity + s.harmony) / 3.0;
    }

    pub fn inDanger(s: SelfFeedback) bool {
        return s.survival() < 0.3;
    }
};

// ── 一体化引擎 ──
pub const PerceptionEngine = struct {
    encoding: EncodingLayer = .{},
    state: StateMachine = .{},
    scale: JudgmentSystem.BalanceScale = .{},
    weights: JudgmentSystem.SenatorVote = .{},
    wisdom: WisdomEmergence = .{},
    trust: TrustParameters = .{},
    self_feedback: SelfFeedback = .{},

    pub fn boot() PerceptionEngine {
        return .{};
    }

    // 运行一个周期
    pub fn cycle(e: *PerceptionEngine, time_urgency: f32, relation_importance: f32, ownership_key: f32) f32 {
        const score = e.scale.judge(time_urgency, relation_importance, ownership_key);

        if (e.self_feedback.inDanger()) {
            return 0.0;
        }

        e.state.time += 1;
        e.wisdom.converge(6);
        e.trust.history = @min(1.0, e.trust.history + score * 0.1);

        return score * e.wisdom.wisdom_score;
    }
};

// ── 测试 ──
test "encoding_phoneme" {
    const p = EncodingLayer.Phoneme.encode(0x01, 0x0F, 4);
    try std.testing.expectEqual(@as(u8, 0x01), p.initial);
    try std.testing.expectEqual(@as(u8, 0x0F), p.final); // un = 灰原哀替身
    try std.testing.expectEqual(@as(u4, 4), p.tone);
}

test "final_table_un" {
    // un 必须在第 15 个位置 (0x0F)
    try std.testing.expectEqual(@as(u8, 0x0F), EncodingLayer.FINAL_TABLE[15]);
}

test "verify_union_to_intersection" {
    const p = EncodingLayer.Phoneme.encode(0x01, 0x0F, 3);
    try std.testing.expect(EncodingLayer.verifyUnionToIntersection(p));
}

test "state_transition" {
    var s = StateMachine{};
    try std.testing.expectEqual(@as(u8, 0), @intFromEnum(s.stage));
    s.transition(.unified);
    try std.testing.expectEqual(@as(u8, 2), @intFromEnum(s.stage));
}

test "balance_scale_judge" {
    const scale = JudgmentSystem.BalanceScale{};
    const score = scale.judge(0.8, 0.6, 0.9);
    try std.testing.expect(score > 0.0 and score <= 1.0);
}

test "senator_vote" {
    const votes = [_]f32{ 0.9, 0.7, 0.8, 0.6 };
    const result = JudgmentSystem.aggregateVote(votes, .{});
    try std.testing.expect(result > 0.0 and result <= 1.0);
}

test "wisdom_converge" {
    var w = WisdomEmergence{};
    w.converge(6);
    try std.testing.expectEqual(@as(u8, 3), @intFromEnum(w.convergence));
    try std.testing.expectEqual(@as(f32, 1.0), w.wisdom_score);
}

test "trust_level" {
    const t = TrustParameters{ .history = 0.8, .compliance = 0.6, .transparency = 0.7 };
    const lvl = t.level();
    try std.testing.expect(lvl > 0.6 and lvl < 0.8);
}

test "self_feedback_survival" {
    const s = SelfFeedback{ .resources = 0.5, .integrity = 0.5, .harmony = 0.5 };
    const surv = s.survival();
    try std.testing.expectApproxEqAbs(0.5, surv, 0.01);
}

test "self_in_danger" {
    const s = SelfFeedback{ .resources = 0.1, .integrity = 0.2, .harmony = 0.1 };
    try std.testing.expect(s.inDanger());
}

test "perception_engine_boot" {
    const e = PerceptionEngine.boot();
    try std.testing.expectEqual(@as(u32, 0), e.state.time);
}

test "perception_cycle" {
    var e = PerceptionEngine.boot();
    const result = e.cycle(0.8, 0.6, 0.9);
    try std.testing.expect(result >= 0.0 and result <= 1.1);
    try std.testing.expectEqual(@as(u32, 1), e.state.time);
}

test "perception_defense_mode" {
    var e = PerceptionEngine.boot();
    e.self_feedback.resources = 0.1;
    e.self_feedback.integrity = 0.1;
    e.self_feedback.harmony = 0.1;
    const result = e.cycle(0.8, 0.6, 0.9);
    try std.testing.expectEqual(@as(f32, 0.0), result);
}

