// arbiter.zig — 满全法裁判器
// 三元见证（非空 + 已见证 + 锚点一致）

const std = @import("std");
const indexer = @import("indexer.zig");
const fqm = @import("fqm.zig");

pub const MERIT_ACCEPT: f64 = 0x0A;  // 10
pub const DEMERIT_REJECT: f64 = 0x0F; // 15

pub const Verdict = enum {
    accept,
    reject,
    pending,
    appeal,
};

pub const Polarity = enum {
    yin,
    yang,
    balanced,
};

pub const JudgedContainer = struct {
    container_id: u64,
    name: []const u8,
    verdict: Verdict,
    polarity: Polarity,
    merit: f64,
    reason: []const u8,
};

pub const Arbiter = struct {
    allocator: std.mem.Allocator,
    judgments: std.ArrayList(JudgedContainer),

    pub fn init(allocator: std.mem.Allocator) Arbiter {
        return Arbiter{
            .allocator = allocator,
            .judgments = std.ArrayList(JudgedContainer).init(allocator),
        };
    }

    pub fn deinit(self: *Arbiter) void {
        self.judgments.deinit();
    }

    pub fn judge(self: *Arbiter, container: *const indexer.Container) !JudgedContainer {
        var j = JudgedContainer{
            .container_id = container.id,
            .name = container.name,
            .verdict = .accept,
            .polarity = .balanced,
            .merit = 0,
            .reason = "",
        };

        const non_empty = container.item_count > 0;
        const anchor_match = (container.positive_count + container.neutral_count) >= container.negative_count;
        const witnessed = container.item_count > 0;

        if (non_empty and anchor_match and witnessed) {
            j.verdict = .accept;
            j.merit = MERIT_ACCEPT + container.netBenefit();
            j.reason = "accept";
        } else if (!non_empty) {
            j.verdict = .reject;
            j.merit = -DEMERIT_REJECT;
            j.reason = "reject:empty";
        } else if (!anchor_match) {
            j.verdict = .reject;
            j.merit = -DEMERIT_REJECT * 0.5;
            j.reason = "reject:anchor";
        } else if (!witnessed) {
            j.verdict = .reject;
            j.merit = -DEMERIT_REJECT * 0.3;
            j.reason = "reject:unwitnessed";
        } else {
            j.verdict = .pending;
            j.merit = 0;
            j.reason = "pending";
        }

        if (container.netBenefit() > 0) {
            j.polarity = .yang;
        } else if (container.netBenefit() < 0) {
            j.polarity = .yin;
        }

        try self.judgments.append(j);
        return j;
    }

    pub fn encodeJudgmentReport(self: *const Arbiter, allocator: std.mem.Allocator) ![]const u8 {
        var output = std.ArrayList(u8).init(allocator);
        defer output.deinit();

        const w = output.writer();
        try output.appendSlice("---\n");
        try output.appendSlice("# 满全法裁决报告\n");
        try output.appendSlice("judgments:");
        try fqm.formatUnaryInt(w, self.judgments.items.len);
        try output.appendSlice("\n\n");

        for (self.judgments.items) |j| {
            try output.appendSlice("~judgment:id:");
            try fqm.formatUnaryInt(w, j.container_id);
            try output.appendSlice("\n  name:");
            try output.appendSlice(j.name);
            try output.appendSlice("\n  verdict:");
            try output.appendSlice(@tagName(j.verdict));
            try output.appendSlice("\n  polarity:");
            try output.appendSlice(@tagName(j.polarity));
            try output.appendSlice("\n  merit:");
            try fqm.formatUnaryDecimal(w, j.merit);
            try output.appendSlice("\n  reason:");
            try output.appendSlice(j.reason);
            try output.appendSlice("\n");
        }

        return output.toOwnedSlice();
    }
};

test "arbiter judge accept" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    var arbiter = Arbiter.init(allocator);
    defer arbiter.deinit();

    var container = indexer.Container.init(1, "test");
    container.item_count = 1;
    container.positive_count = 1;
    container.total_benefit = 5.0;

    const j = try arbiter.judge(&container);
    try std.testing.expectEqual(Verdict.accept, j.verdict);
    try std.testing.expectEqual(Polarity.yang, j.polarity);
}

test "arbiter judge reject empty" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    var arbiter = Arbiter.init(allocator);
    defer arbiter.deinit();

    var container = indexer.Container.init(1, "empty");
    container.item_count = 0;

    const j = try arbiter.judge(&container);
    try std.testing.expectEqual(Verdict.reject, j.verdict);
    try std.testing.expectEqual(Polarity.balanced, j.polarity);
}
