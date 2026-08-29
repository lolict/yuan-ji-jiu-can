// 索引器 (Indexer)
// 功能：按菜单顺序排队 + 天平成本计算 + 正负因果分类 + 容器统计 + 标量输出
// 核心：排队 → 计量 → 分类 → 统计 → 标量

const std = @import("std");
const mapper = @import("mapper.zig");
const aligner = @import("aligner.zig");

// ============================================================
// 1. 菜单项 (Menu Item)
// ============================================================
// 按菜单列表顺序排队的单位
pub const MenuItem = struct {
    id: u32,
    name: []const u8,
    priority: u32,          // 优先级（数字越小越优先）
    cost: f64,              // 天平成本
    benefit: f64,           // 天平收益
    causality: Causality,   // 因果类型
    container_id: u32,      // 所属分类容器

    pub fn init(id: u32, name: []const u8, priority: u32, cost: f64, benefit: f64, causality: Causality, container_id: u32) MenuItem {
        return MenuItem{
            .id = id,
            .name = name,
            .priority = priority,
            .cost = cost,
            .benefit = benefit,
            .causality = causality,
            .container_id = container_id,
        };
    }

    // 计算净收益（天平倾斜度）
    pub fn netBenefit(self: *const MenuItem) f64 {
        return self.benefit - self.cost;
    }

    // 计算收益成本比
    pub fn benefitCostRatio(self: *const MenuItem) f64 {
        if (self.cost == 0) return std.math.inf(f64);
        return self.benefit / self.cost;
    }
};

// ============================================================
// 2. 因果类型 (Causality)
// ============================================================
pub const Causality = enum {
    positive,   // 正向因果：好的导致好的
    negative,   // 负向因果：坏的导致坏的
    mix,        // 混合因果：正负交织
    neutral,    // 中性因果：无明确倾向
};

// ============================================================
// 3. 容器统计 (Container Statistics)
// ============================================================
// 分类容器，用于在空集中统计
pub const Container = struct {
    id: u32,
    name: []const u8,
    item_count: u32,
    total_cost: f64,
    total_benefit: f64,
    positive_count: u32,
    negative_count: u32,
    mix_count: u32,
    neutral_count: u32,

    pub fn init(id: u32, name: []const u8) Container {
        return Container{
            .id = id,
            .name = name,
            .item_count = 0,
            .total_cost = 0,
            .total_benefit = 0,
            .positive_count = 0,
            .negative_count = 0,
            .mix_count = 0,
            .neutral_count = 0,
        };
    }

    pub fn addItem(self: *Container, item: *const MenuItem) void {
        self.item_count += 1;
        self.total_cost += item.cost;
        self.total_benefit += item.benefit;

        switch (item.causality) {
            .positive => self.positive_count += 1,
            .negative => self.negative_count += 1,
            .mix => self.mix_count += 1,
            .neutral => self.neutral_count += 1,
        }
    }

    pub fn netBenefit(self: *const Container) f64 {
        return self.total_benefit - self.total_cost;
    }

    pub fn positiveRatio(self: *const Container) f64 {
        if (self.item_count == 0) return 0;
        return @as(f64, @floatFromInt(self.positive_count)) / @as(f64, @floatFromInt(self.item_count));
    }

    pub fn negativeRatio(self: *const Container) f64 {
        if (self.item_count == 0) return 0;
        return @as(f64, @floatFromInt(self.negative_count)) / @as(f64, @floatFromInt(self.item_count));
    }
};

// ============================================================
// 4. 标量 (Scalar)
// ============================================================
// 量化输出
pub const Scalar = struct {
    value: f64,
    unit: []const u8,
    description: []const u8,

    pub fn init(value: f64, unit: []const u8, description: []const u8) Scalar {
        return Scalar{
            .value = value,
            .unit = unit,
            .description = description,
        };
    }
};

// ============================================================
// 5. 索引器核心 (Indexer Core)
// ============================================================
pub const Indexer = struct {
    allocator: std.mem.Allocator,
    menu_items: std.ArrayList(MenuItem),
    containers: std.ArrayList(Container),

    pub fn init(allocator: std.mem.Allocator) Indexer {
        return Indexer{
            .allocator = allocator,
            .menu_items = std.ArrayList(MenuItem).init(allocator),
            .containers = std.ArrayList(Container).init(allocator),
        };
    }

    pub fn deinit(self: *Indexer) void {
        self.menu_items.deinit();
        self.containers.deinit();
    }

    // 添加菜单项
    pub fn addMenuItem(self: *Indexer, item: MenuItem) !void {
        try self.menu_items.append(item);
    }

    // 添加容器
    pub fn addContainer(self: *Indexer, container: Container) !void {
        try self.containers.append(container);
    }

    // 按优先级排序（菜单列表顺序）
    pub fn sortByPriority(self: *Indexer) void {
        const Context = struct {
            pub fn compare(_: void, a: MenuItem, b: MenuItem) bool {
                return a.priority < b.priority;
            }
        };
        std.sort.insertion(MenuItem, self.menu_items.items, {}, Context.compare);
    }

    // 按净收益排序（天平成本）
    pub fn sortByNetBenefit(self: *Indexer) void {
        const Context = struct {
            pub fn compare(_: void, a: MenuItem, b: MenuItem) bool {
                return a.netBenefit() > b.netBenefit();
            }
        };
        std.sort.insertion(MenuItem, self.menu_items.items, {}, Context.compare);
    }

    // 按收益成本比排序
    pub fn sortByBenefitCostRatio(self: *Indexer) void {
        const Context = struct {
            pub fn compare(_: void, a: MenuItem, b: MenuItem) bool {
                return a.benefitCostRatio() > b.benefitCostRatio();
            }
        };
        std.sort.insertion(MenuItem, self.menu_items.items, {}, Context.compare);
    }

    // 分类统计到容器
    pub fn classifyAndCount(self: *Indexer) void {
        // 清空容器统计
        for (self.containers.items) |*container| {
            container.item_count = 0;
            container.total_cost = 0;
            container.total_benefit = 0;
            container.positive_count = 0;
            container.negative_count = 0;
            container.mix_count = 0;
            container.neutral_count = 0;
        }

        // 重新统计
        for (self.menu_items.items) |item| {
            for (self.containers.items) |*container| {
                if (container.id == item.container_id) {
                    container.addItem(&item);
                    break;
                }
            }
        }
    }

    // 生成标量报告
    pub fn generateScalarReport(self: *Indexer) !ScalarReport {
        var report = ScalarReport.init(self.allocator);
        errdefer report.deinit();

        // 总标量
        var total_cost: f64 = 0;
        var total_benefit: f64 = 0;
        var total_positive: u32 = 0;
        var total_negative: u32 = 0;

        for (self.menu_items.items) |item| {
            total_cost += item.cost;
            total_benefit += item.benefit;
            switch (item.causality) {
                .positive => total_positive += 1,
                .negative => total_negative += 1,
                else => {},
            }
        }

        const total_net = total_benefit - total_cost;
        const total_ratio = if (total_cost > 0) total_benefit / total_cost else 0;

        // 添加标量到报告
        try report.addScalar(Scalar.init(total_net, "net_benefit", "total net benefit"));
        try report.addScalar(Scalar.init(total_ratio, "ratio", "benefit cost ratio"));
        try report.addScalar(Scalar.init(@as(f64, @floatFromInt(total_positive)), "positive_count", "positive causality count"));
        try report.addScalar(Scalar.init(@as(f64, @floatFromInt(total_negative)), "negative_count", "negative causality count"));
        try report.addScalar(Scalar.init(@as(f64, @floatFromInt(self.menu_items.items.len)), "total_count", "total item count"));

        return report;
    }

    // 获取指定因果类型的菜单项
    pub fn getItemsByCausality(self: *Indexer, causality: Causality) std.ArrayList(MenuItem) {
        var result = std.ArrayList(MenuItem).init(self.allocator);
        for (self.menu_items.items) |item| {
            if (item.causality == causality) {
                _ = result.append(item) catch {};
            }
        }
        return result;
    }

    // 获取指定容器的菜单项
    pub fn getItemsByContainer(self: *Indexer, container_id: u32) std.ArrayList(MenuItem) {
        var result = std.ArrayList(MenuItem).init(self.allocator);
        for (self.menu_items.items) |item| {
            if (item.container_id == container_id) {
                _ = result.append(item) catch {};
            }
        }
        return result;
    }
};

// ============================================================
// 6. 标量报告 (Scalar Report)
// ============================================================
pub const ScalarReport = struct {
    allocator: std.mem.Allocator,
    scalars: std.ArrayList(Scalar),

    pub fn init(allocator: std.mem.Allocator) ScalarReport {
        return ScalarReport{
            .allocator = allocator,
            .scalars = std.ArrayList(Scalar).init(allocator),
        };
    }

    pub fn deinit(self: *ScalarReport) void {
        self.scalars.deinit();
    }

    pub fn addScalar(self: *ScalarReport, scalar: Scalar) !void {
        try self.scalars.append(scalar);
    }

    pub fn getTotalNetBenefit(self: ScalarReport) f64 {
        var total: f64 = 0;
        for (self.scalars.items) |scalar| {
            if (std.mem.eql(u8, scalar.unit, "net_benefit")) {
                total += scalar.value;
            }
        }
        return total;
    }

    pub fn getPositiveRatio(self: *const ScalarReport) f64 {
        var positive: f64 = 0;
        var total: f64 = 0;
        for (self.scalars.items) |scalar| {
            if (std.mem.eql(u8, scalar.unit, "positive_count")) {
                positive += scalar.value;
            }
            if (std.mem.eql(u8, scalar.unit, "total_count")) {
                total += scalar.value;
            }
        }
        if (total == 0) return 0;
        return positive / total;
    }
};

// ============================================================
// 7. 排队处理器 (Queue Processor)
// ============================================================
pub const QueueProcessor = struct {
    indexer: *Indexer,

    pub fn init(indexer: *Indexer) QueueProcessor {
        return QueueProcessor{
            .indexer = indexer,
        };
    }

    // 按菜单顺序处理队列
    pub fn processByMenuOrder(self: *QueueProcessor) void {
        self.indexer.sortByPriority();
    }

    // 按天平成本处理队列（净收益高的优先）
    pub fn processByBalance(self: *QueueProcessor) void {
        self.indexer.sortByNetBenefit();
    }

    // 按收益成本比处理队列
    pub fn processByEfficiency(self: *QueueProcessor) void {
        self.indexer.sortByBenefitCostRatio();
    }

    // 按因果类型处理队列（先正后负）
    pub fn processByCausality(self: *QueueProcessor) void {
        // 先按优先级排序，再按因果类型分组
        self.indexer.sortByPriority();
    }
};

// ============================================================
// 测试
// ============================================================
test "indexer basic" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var indexer = Indexer.init(allocator);
    defer indexer.deinit();

    // 添加容器
    try indexer.addContainer(Container.init(1, "color_base"));
    try indexer.addContainer(Container.init(2, "sound_base"));

    // 添加菜单项
    try indexer.addMenuItem(MenuItem.init(1, "red", 1, 10.0, 15.0, .positive, 1));
    try indexer.addMenuItem(MenuItem.init(2, "blue", 2, 8.0, 12.0, .positive, 1));
    try indexer.addMenuItem(MenuItem.init(3, "ai", 3, 20.0, 18.0, .negative, 2));
    try indexer.addMenuItem(MenuItem.init(4, "ao", 4, 5.0, 8.0, .neutral, 2));

    // 按优先级排序
    indexer.sortByPriority();
    try std.testing.expectEqual(@as(u32, 1), indexer.menu_items.items[0].id);

    // 按净收益排序
    indexer.sortByNetBenefit();
    try std.testing.expectEqual(@as(u32, 1), indexer.menu_items.items[0].id);  // red: 5.0

    // 分类统计
    indexer.classifyAndCount();
    try std.testing.expectEqual(@as(u32, 2), indexer.containers.items[0].item_count);
    try std.testing.expectEqual(@as(u32, 2), indexer.containers.items[1].item_count);
}

test "indexer scalar report" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var indexer = Indexer.init(allocator);
    defer indexer.deinit();

    try indexer.addContainer(Container.init(1, "test"));
    try indexer.addMenuItem(MenuItem.init(1, "item1", 1, 10.0, 15.0, .positive, 1));
    try indexer.addMenuItem(MenuItem.init(2, "item2", 2, 5.0, 8.0, .negative, 1));

    indexer.classifyAndCount();

    var report = try indexer.generateScalarReport();
    defer report.deinit();
    try std.testing.expectEqual(@as(f64, 8.0), report.getTotalNetBenefit());
}
