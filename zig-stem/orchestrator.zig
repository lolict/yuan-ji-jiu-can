// 协调器 (Orchestrator)
// 功能：统一指挥所有"器"的协同工作
// 核心：初始化所有模块 → 协调调度 → 统一执行

const std = @import("std");
const mapper = @import("mapper.zig");
const aligner = @import("aligner.zig");
const indexer = @import("indexer.zig");
const stepper = @import("stepper.zig");
const locator = @import("locator.zig");
pub const phaser = @import("phaser.zig");
const angler = @import("angler.zig");
const frequencer = @import("frequencer.zig");
const arbiter = @import("arbiter.zig");

// ============================================================
// 1. 协调器配置 (Orchestrator Config)
// ============================================================
pub const OrchestratorConfig = struct {
    total_steps: u64,
    step_mode: stepper.StepMode,
    mapper_time_window_ms: u64,
    aligner_tolerance: f64,
    phaser_tolerance: f64,
    phaser_adjustment_rate: f64,
    phaser_strategy: phaser.AdjustmentStrategy,
    angler_tolerance: f64,
    frequencer_tolerance_hz: f64,

    pub fn default() OrchestratorConfig {
        return OrchestratorConfig{
            .total_steps = 100,
            .step_mode = .continuous,
            .mapper_time_window_ms = 1000,
            .aligner_tolerance = 0.05,
            .phaser_tolerance = 0.05,
            .phaser_adjustment_rate = 0.1,
            .phaser_strategy = .gradual,
            .angler_tolerance = 5.0,
            .frequencer_tolerance_hz = 0.5,
        };
    }
};

// ============================================================
// 2. 协调器核心 (Orchestrator Core)
// ============================================================
pub const Orchestrator = struct {
    allocator: std.mem.Allocator,
    config: OrchestratorConfig,

    // 所有模块实例
    fixed_reference: mapper.FixedReference,
    mapper: mapper.Mapper,
    aligner: aligner.Aligner,
    indexer: indexer.Indexer,
    stepper: stepper.Stepper,
    locator: locator.Locator,
    phaser: phaser.Phaser,
    angler: angler.Angler,
    frequencer: frequencer.Frequencer,
    arbiter: arbiter.Arbiter,

    // 参考点（用于定位器）
    reference_point: locator.Coordinate,

    pub fn init(allocator: std.mem.Allocator, config: OrchestratorConfig) !Orchestrator {
        var orchestrator: Orchestrator = undefined;
        orchestrator.allocator = allocator;
        orchestrator.config = config;
        orchestrator.fixed_reference = mapper.FixedReference.init(1, 10);
        orchestrator.mapper = mapper.Mapper.init(&orchestrator.fixed_reference, config.mapper_time_window_ms);
        orchestrator.aligner = aligner.Aligner.init(config.aligner_tolerance);
        orchestrator.indexer = indexer.Indexer.init(allocator);
        orchestrator.stepper = stepper.Stepper.init(allocator, config.total_steps, config.step_mode, &orchestrator.mapper, &orchestrator.aligner, &orchestrator.indexer);
        orchestrator.reference_point = locator.Coordinate.init(0, 0, 0);
        orchestrator.locator = locator.Locator.init(allocator, &orchestrator.reference_point);
        orchestrator.phaser = phaser.Phaser.init(allocator, config.phaser_strategy, config.phaser_adjustment_rate, config.phaser_tolerance);
        orchestrator.angler = angler.Angler.init(allocator, config.angler_tolerance);
        orchestrator.frequencer = frequencer.Frequencer.init(allocator, config.frequencer_tolerance_hz);
        orchestrator.arbiter = arbiter.Arbiter.init(allocator);

        return orchestrator;
    }

    pub fn deinit(self: *Orchestrator) void {
        self.indexer.deinit();
        self.stepper.deinit();
        self.locator.deinit();
        self.phaser.deinit();
        self.angler.deinit();
        self.frequencer.deinit();
        self.arbiter.deinit();
    }

    // 启动协调器
    pub fn start(self: *Orchestrator) !void {
        // 初始化容器
        try self.indexer.addContainer(indexer.Container.init(1, "color_base"));
        try self.indexer.addContainer(indexer.Container.init(2, "sound_base"));
        try self.indexer.addContainer(indexer.Container.init(3, "relation_base"));

        // 初始化位置
        try self.locator.addPosition(locator.PositionEntry.init(1, "origin", locator.Coordinate.init(0, 0, 0), 1));
        try self.locator.addPosition(locator.PositionEntry.init(2, "partner", locator.Coordinate.init(1, 0, 0), 1));

        // 初始化关系
        try self.locator.addRelation(locator.RelationPosition.init(1, 2, .positive, 1.0, 1.0));

        // 启动步进器
        self.stepper.start();
    }

    // 执行一步协调
    pub fn step(self: *Orchestrator) !bool {
        if (!self.stepper.isRunning()) return false;

        // 1. 映射器：记录步进
        self.stepper.mapper.stepA();
        if (self.stepper.getCurrentStep() % 2 == 0) {
            self.stepper.mapper.stepB();
        }

        // 2. 检查对齐状态（结果可用于后续判断）
        _ = self.mapper.performAlignment();

        // 3. 索引器：分类统计
        self.indexer.classifyAndCount();

        // 4. 步进器：推进
        const stepped = self.stepper.step();
        if (!stepped) return false;

        return true;
    }

    // 获取整体进度
    pub fn getProgress(self: *const Orchestrator) f64 {
        return self.stepper.getProgress();
    }

    // 获取步进数
    pub fn getCurrentStep(self: *const Orchestrator) u64 {
        return self.stepper.getCurrentStep();
    }

    // 是否完成
    pub fn isComplete(self: *const Orchestrator) bool {
        return self.stepper.isComplete();
    }

    // 获取对齐状态
    pub fn getAlignmentStatus(self: *const Orchestrator) mapper.AlignmentResult {
        return self.mapper.performAlignment();
    }

    // 获取容器统计
    pub fn getContainerStats(self: *const Orchestrator) []const indexer.Container {
        return self.indexer.containers.items;
    }

    // 满全法裁决器报告
    pub fn judgeAllContainers(self: *Orchestrator) !void {
        const stats = self.indexer.containers.items;
        for (stats) |*container| {
            _ = try self.arbiter.judge(container);
        }
    }

    // 获取编码后的裁决报告
    pub fn getJudgmentReport(self: *const Orchestrator, allocator: std.mem.Allocator) ![]const u8 {
        return self.arbiter.encodeJudgmentReport(allocator);
    }

    // 运行完整流程
    pub fn run(self: *Orchestrator) !void {
        try self.start();
        while (!self.isComplete()) {
            _ = try self.step();
        }
    }
};

// ============================================================
// 测试
// ============================================================
test "orchestrator basic" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var config = OrchestratorConfig.default();
    config.total_steps = 10;

    var orchestrator = try Orchestrator.init(allocator, config);
    defer orchestrator.deinit();

    // 启动
    try orchestrator.start();

    // 运行完整流程
    try orchestrator.run();

    // 验证完成
    try std.testing.expect(orchestrator.isComplete());
    try std.testing.expectEqual(@as(u64, 10), orchestrator.getCurrentStep());
}

test "orchestrator step by step" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var config = OrchestratorConfig.default();
    config.total_steps = 5;

    var orchestrator = try Orchestrator.init(allocator, config);
    defer orchestrator.deinit();

    try orchestrator.start();

    var count: u64 = 0;
    while (!orchestrator.isComplete()) {
        _ = try orchestrator.step();
        count += 1;
    }

    try std.testing.expectEqual(@as(u64, 5), count);
}
