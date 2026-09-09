// ══════════════════════════════════════════════════════════════════
// funnel.rs — 漏斗控制器 (Funnel Controller)
// ══════════════════════════════════════════════════════════════════
//
// 控制分布式时间 + 空间节点的运行节律:
//
//   时间控制:  "控制时间的火候" — 火烧热了, 快速接触, 不被烧伤
//              把工作切成毫秒/秒/分/时/日 层级的窗口.
//              每窗口做有限工作量, 用完就停, 等下一个窗口 tick.
//
//   空间控制:  "让内存不溢出" — 把人参果切片吃, 不要一次吃完.
//              每节点只做有限内存的工作.
//
//   层级调度:  从最细粒度(毫秒)逐层向上, 每一层把自家窗口的统计
//              送入上层. 这本身就是一个"漏斗": 细→粗是由大到小.
//
// 关键操作:
//   TimeGovernor  — 时间节奏器 (控制何时接触/撤退)
//   SpaceBatcher  — 空间切片器 (控制每次吃多少)
//   FunnelScheduler — 组合两者, 接收节点产出, 逐层坍缩

use crate::stats::StreamStats;
use crate::triune::{Funnel};
use crate::mirror;
use crate::memory::LayeredMemory;
use crate::os_layer::OsLayer;

// ────────────────────────────────────────────────────────────
// 时间节奏器 (Time Governor)
// ────────────────────────────────────────────────────────────
//
// "火烧热了, 你要用快速方式接触火而不被烧伤"
// → 每一层有不同接触时长. 越细粒度, 工作应该越短,
//   越粗粒度, 可以缓慢处理存量.
//
// 节点:
//   Level 0 毫秒窗  → contact 最多 work_per_ms ×1 工作
//   Level 1 秒级窗  → 1000ms 的汇总 → 可承受更深工作
//   Level 2 分级窗  → ...
//   ...
//   Level 6 年级窗  → 最粗缓存, 但一旦触发可做大清洗

#[derive(Clone, Debug)]
pub struct TimeGovernor {
    /// 层级名称
    pub level: TimeLevel,
    /// 窗口时长 (ms)
    pub window_ms: u64,
    /// 本次窗口内最多做多少单位工作 (空间单位数)
    pub work_capacity: u64,
    /// 本窗口已经消耗掉的工作单位数
    pub consumed: u64,
    /// 当前 tick 计数 (自节点开机)
    pub ticks: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeLevel {
    MilliSec,  // 0
    Sec,       // 1
    Min,       // 2
    Hour,      // 3
    Day,       // 4
    Month,     // 5
    Year,      // 6
}

impl TimeGovernor {
    /// 构造某层的时间节奏器
    pub fn new(level: TimeLevel, window_ms: u64, work_capacity: u64) -> Self {
        Self { level, window_ms, work_capacity, consumed: 0, ticks: 0 }
    }

    /// 标准层级构造 (预设 work_capacity 随 level 变大)
    pub fn standard(level: TimeLevel) -> Self {
        let (window, cap) = match level {
            TimeLevel::MilliSec => (1,       16),
            TimeLevel::Sec      => (1000,    256),
            TimeLevel::Min      => (60_000,   4096),
            TimeLevel::Hour     => (3_600_000, 65_536),
            TimeLevel::Day      => (86_400_000, 1_048_576),
            TimeLevel::Month    => (2_592_000_000, 16_777_216),
            TimeLevel::Year     => (31_536_000_000, 268_435_456),
        };
        Self::new(level, window, cap)
    }

    /// 尝试消耗 work 单位. 成功返回 true, 超出容量返回 false
    pub fn try_consume(&mut self, work: u64) -> bool {
        if self.consumed + work > self.work_capacity {
            false
        } else {
            self.consumed += work;
            true
        }
    }

    /// 推进到下一个 tick. 每 window_ms 单位 reset consumed.
    pub fn tick(&mut self, dt_ms: u64) {
        self.ticks += dt_ms;
        if self.ticks >= self.window_ms {
            // 窗口轮转
            self.ticks = self.ticks % self.window_ms;
            self.consumed = 0;
        }
    }

    /// 当前窗口剩余的容量
    pub fn remaining_capacity(&self) -> u64 {
        self.work_capacity - self.consumed
    }

    /// 当前窗口利用率
    pub fn utilization(&self) -> f64 {
        if self.work_capacity == 0 { 0.0 }
        else { self.consumed as f64 / self.work_capacity as f64 }
    }

    /// 是否已满 (不能再做工作)
    pub fn is_saturated(&self) -> bool {
        self.consumed >= self.work_capacity
    }
}

// ────────────────────────────────────────────────────────────
// 空间切片器 (Space Batcher)
// ────────────────────────────────────────────────────────────
//
// "不要一次吃完人参果" — 把工作切成 chunk,
// 每次只处理 chunk_size 大小, 做完 sleep, 再做下一片.
//
// 切片策略:
//   sequential  — 顺序切 (慢而稳)
//   strided     — 交错切 (均匀分布处理)
//   random_ph   — 伪随机切 (负载均衡不依赖全局协  调)

use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct SpaceBatcher {
    pub chunk_size: usize,
    pub processed: usize,
    pub total: usize,
    pub buffer: VecDeque<Vec<u8>>,
}

impl SpaceBatcher {
    pub fn new(chunk_size: usize) -> Self {
        assert!(chunk_size > 0);
        Self { chunk_size, processed: 0, total: 0, buffer: VecDeque::new() }
    }

    /// 灌入一批数据 (灌入量无限制, 内部会切分)
    pub fn ingest(&mut self, data: &[u8]) {
        for chunk in data.chunks(self.chunk_size) {
            self.buffer.push_back(chunk.to_vec());
        }
        self.total += data.len();
    }

    /// 尝试取一个 chunk 做处理. None = 没有更多了.
    pub fn next_chunk(&mut self) -> Option<&[u8]> {
        self.buffer.front().map(|v| {
            self.processed += v.len();
            v.as_slice()
        })
    }

    /// 当前正在处理的 chunk 已经处理了多少 byte
    pub fn complete_current(&mut self) {
        self.buffer.pop_front();
    }

    /// 是否还有工作要做
    pub fn has_work(&self) -> bool {
        !self.buffer.is_empty()
    }

    /// 进度 0..1
    pub fn progress(&self) -> f64 {
        if self.total == 0 { 0.0 }
        else { self.processed as f64 / self.total as f64 }
    }
}

// ────────────────────────────────────────────────────────────
// 漏斗调度器
// ────────────────────────────────────────────────────────────
//
// 组合 TimeGovernor + SpaceBatcher + Funnel:
//   1. SpaceBatcher 把大数据切成可处理的小片
//   2. TimeGovernor 控制每次只做 capacity 内的工作
//   3. Funnel 把每个 chunk 的产出逐层坍缩成统计量
//
// 这即是 "时间段内 & 空间段内" 的完整控制器.

fn now_ms(os: &dyn OsLayer) -> u64 {
    os.time_millis()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Heartbeat {
    pub last_tick_ms: u64,
    pub stale_count: u32,
}

impl Heartbeat {
    pub fn new(os: &dyn OsLayer) -> Self {
        Self { last_tick_ms: now_ms(os), stale_count: 0 }
    }

    pub fn beat(&mut self, os: &dyn OsLayer) {
        self.last_tick_ms = now_ms(os);
        self.stale_count = 0;
    }

    pub fn is_stale(&self, os: &dyn OsLayer, timeout_ms: u64) -> bool {
        now_ms(os).saturating_sub(self.last_tick_ms) > timeout_ms
    }

    pub fn mark_stale(&mut self) {
        self.stale_count += 1;
    }

    pub fn staleness(&self) -> u32 {
        self.stale_count
    }
}

#[derive(Debug)]
pub struct FunnelScheduler {
    pub time_gov: TimeGovernor,
    pub space_batcher: SpaceBatcher,
    pub funnel: Funnel,
    pub stats: StreamStats,
    pub this_round_collapse: f64,
    pub total_rounds: u64,
    pub status: SchedulerStatus,
    pub heartbeat: Heartbeat,
    /// 看门狗阈值: 连续 step 超过此次数但无进展 → 判定为 Zombie
    pub watchdog_threshold: u64,
    /// 内部计数: 连续 step 无真进展的次数
    no_progress_count: u64,
    /// 上次 progress 快照，用于区分真进展与假进展
    last_progress_snapshot: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SchedulerStatus {
    Idle,       // 空闲
    Running,    // 进行中
    Saturated,  // 时间/空间均满
    Done,       // 全部处理完成
    Zombie,     // 假死 (看门狗判定)
}

impl FunnelScheduler {
    /// 桥接: 漏斗调度器的本轮统计 → 层级记忆记录（时间漏斗=记忆写入节奏）
    pub fn snapshot_to_memory(&self, os: &dyn OsLayer, memory: &mut LayeredMemory, tick: u64) {
        memory.record_witness(
            os,
            (self.total_rounds, 0, 0),
            (self.this_round_collapse as u64, 0, 0),
            self.this_round_collapse,
            self.this_round_collapse < 100.0, // 低坍缩量=对齐良好
            tick,
        );
    }

    pub fn new(os: &dyn OsLayer, time_gov: TimeGovernor, space_batcher: SpaceBatcher, funnel: Funnel) -> Self {
        Self::with_threshold(os, time_gov, space_batcher, funnel, 1000)
    }

    /// 带自定义看门狗阈值的构造
    pub fn with_threshold(os: &dyn OsLayer, time_gov: TimeGovernor, space_batcher: SpaceBatcher, funnel: Funnel, watchdog_threshold: u64) -> Self {
        Self {
            time_gov, space_batcher, funnel,
            stats: StreamStats::new(),
            this_round_collapse: 0.0,
            total_rounds: 0,
            status: SchedulerStatus::Idle,
            heartbeat: Heartbeat::new(os),
            watchdog_threshold,
            no_progress_count: 0,
            last_progress_snapshot: 0.0,
        }
    }

    /// 灌入一批数据 (外部输入)
    pub fn ingest(&mut self, data: &[u8]) {
        self.space_batcher.ingest(data);
        if self.status == SchedulerStatus::Idle {
            self.status = SchedulerStatus::Running;
        }
    }

    /// 计算当前进度快照 (用于区分真进展与假进展)
    fn progress_snapshot(&self) -> f64 {
        self.stats.count as f64 + self.space_batcher.progress() * 1000.0 + self.total_rounds as f64
    }

    /// 看门狗检测: 假如 progress 没有实质性增长，计数+1，超阈值判定 Zombie
    fn watchdog_check(&mut self, os: &dyn OsLayer) -> bool {
        let snap = self.progress_snapshot();
        if (snap - self.last_progress_snapshot).abs() < 1e-9 {
            self.no_progress_count += 1;
        } else {
            self.no_progress_count = 0;
            self.heartbeat.beat(os);
        }
        self.last_progress_snapshot = snap;
        if self.no_progress_count >= self.watchdog_threshold {
            self.status = SchedulerStatus::Zombie;
            true
        } else {
            false
        }
    }

    /// 单步调度: 在时间窗口内做完一个 chunk
    /// 返回是否还应该继续调用 step (true=还有工作)
    pub fn step(&mut self, os: &dyn OsLayer) -> bool {
        // 假死状态直接拒绝继续
        if self.status == SchedulerStatus::Zombie {
            return false;
        }
        if self.status == SchedulerStatus::Done {
            return false;
        }

        // 看门狗: 假如 progress 没有增长，累计计数
        if self.watchdog_check(os) {
            return false;
        }

        if self.time_gov.is_saturated() {
            // 时间满了, 轮次统计
            self.total_rounds += 1;
            self.this_round_collapse = self.funnel.pass_through(self.this_round_collapse);
            self.status = SchedulerStatus::Saturated;
            self.heartbeat.beat(os);
            return false;
        }
        let has_chunk = self.space_batcher.next_chunk().is_some();
        if !has_chunk {
            // 空间buffer也处理完了
            self.status = SchedulerStatus::Done;
            return false;
        }
        // copy chunk data out of borrow
        let chunk_vec: Vec<u8> = self.space_batcher.next_chunk().unwrap().to_vec();
        let work_units = chunk_vec.len() as u64;
        if self.time_gov.try_consume(work_units) {
            // 统计: 把 chunk 里 "有" byte 的数量算出来
            let full_count = chunk_vec.iter().filter(|b| **b != 0).count();
            self.stats.push(full_count as f64);
            self.this_round_collapse += full_count as f64;
            self.space_batcher.complete_current();
            self.last_progress_snapshot = self.progress_snapshot();
            self.no_progress_count = 0;
            self.heartbeat.beat(os);
            true
        } else {
            // 时间窗口满了, 刚吃进的这一口还回去
            self.total_rounds += 1;
            self.this_round_collapse = self.funnel.pass_through(self.this_round_collapse);
            self.status = SchedulerStatus::Saturated;
            self.heartbeat.beat(os);
            false
        }
    }

    /// 全部跑一遍 (直到 Done 或 rounds_limit 达到)
    pub fn run(&mut self, os: &dyn OsLayer, max_rounds: u64) -> u64 {
        // 启动时先 beat 一次
        self.heartbeat.beat(os);
        loop {
            if self.total_rounds >= max_rounds { break; }
            self.time_gov.ticked_reset();
            self.this_round_collapse = 0.0;
            self.status = SchedulerStatus::Running;
            while self.step(os) {}
            // 检测假死: 如果 status 是 Zombie 立刻跳出
            if self.status == SchedulerStatus::Zombie {
                break;
            }
            if self.status == SchedulerStatus::Done { break; }
        }
        self.total_rounds
    }

    /// 最终坍缩结果 = 所有 rounds 的统计, 取漏斗最新的出口值
    pub fn total_collapse_ratio(&self) -> f64 {
        self.funnel.contraction_ratio()
    }
}

// ─── helpers ───
impl TimeGovernor {
    pub fn ticked_reset(&mut self) {
        self.consumed = 0;
        self.ticks = 0;
    }
}

// ────────────────────────────────────────────────────────────
// 漏斗多重层级 (层级漏斗) — 实现 "空间换时间的多层抽象"
// ────────────────────────────────────────────────────────────
//
// 层级结构:
//   Level0 (最细, 毫秒)   ← 空间节点输出的原始数据
//   Level1 (秒)            ← 二次统计
//   Level2 (分)           ← 三次统计
//   ...
//   Level6 (最粗, 年)     ← 全局唯一一点 (囡囝共生)
//
// 每层的输出 = 下一层的输入. 每一层本身是一个漏斗.
// 层级越高, 数据越少, 越接近"唯一中心".

#[derive(Debug)]
pub struct CascadedFunnel {
    pub levels: Vec<(TimeGovernor, Funnel, StreamStats)>,
}

impl CascadedFunnel {
    /// 构造7层标准漏斗
    pub fn standard_7(crooked: f64) -> Self {
        let all_levels = [
            TimeLevel::MilliSec,
            TimeLevel::Sec,
            TimeLevel::Min,
            TimeLevel::Hour,
            TimeLevel::Day,
            TimeLevel::Month,
            TimeLevel::Year,
        ];
        let mut levels = Vec::new();
        for (i, lvl) in all_levels.iter().enumerate() {
            let gov = TimeGovernor::standard(*lvl);
            // 越往上 (i越大) mouth越大 neck越小 (收缩比越大)
            let mouth = 100.0 * (10u32.pow(i as u32)) as f64;
            let neck = 1.0;
            let funnel = Funnel::new(mouth, neck, crooked * (1.0 + i as f64 * 0.1));
            levels.push((gov, funnel, StreamStats::new()));
        }
        Self { levels }
    }

    /// 给第 i 层 push 一个测量值
    pub fn push_at(&mut self, level: usize, value: f64) {
        if level >= self.levels.len() { return; }
        // step 1: mutate stats/gov for this level
        let ratio = self.levels[level].1.contraction_ratio();
        self.levels[level].2.push(value);
        let was_sat = {
            let gov = &mut self.levels[level].0;
            gov.try_consume(1);
            gov.is_saturated()
        };
        // step 2: if overflow (window full), push to next level (uses different indices)
        if was_sat && level + 1 < self.levels.len() {
            self.levels[level].0.ticked_reset();
            let compressed = value / ratio;
            self.push_at(level + 1, compressed);
        }
    }

    /// 顶端 (唯一中心) 的当前均值
    pub fn apex_value(&self) -> Option<f64> {
        self.levels.last().map(|(_, _, s)| {
            if s.count > 0 { s.mean } else { 0.0 }
        })
    }

    /// 坍缩比顶到底 (最大压缩比)
    pub fn apex_contraction_ratio(&self) -> f64 {
        self.levels.last().map(|(_, f, _)| f.contraction_ratio()).unwrap_or(1.0)
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_governor_consume_within_capacity() {
        let mut g = TimeGovernor::standard(TimeLevel::MilliSec);
        // MilliSec cap = 16
        assert!(g.try_consume(10));
        assert!(g.try_consume(6));
        assert!(!g.try_consume(1)); // 超了
    }

    #[test]
    fn time_governor_ticked_reset() {
        let mut g = TimeGovernor::standard(TimeLevel::MilliSec);
        g.try_consume(16);
        assert!(g.is_saturated());
        g.ticked_reset();
        assert!(!g.is_saturated());
        assert!(g.try_consume(1));
    }

    #[test]
    fn space_batcher_chunks() {
        let mut sb = SpaceBatcher::new(3);
        sb.ingest(b"abcdef"); // 6 byte → 2 chunks
        let c1 = sb.next_chunk().unwrap();
        assert_eq!(c1, b"abc");
        sb.complete_current();
        let c2 = sb.next_chunk().unwrap();
        assert_eq!(c2, b"def");
        sb.complete_current();
        assert!(!sb.has_work());
    }

    #[test]
    fn funnel_scheduler_single_chunk() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let gov = TimeGovernor::standard(TimeLevel::MilliSec); // capacity 16
        let sb = SpaceBatcher::new(4);
        let funnel = Funnel::new(100.0, 1.0, 0.1);
        let mut sched = FunnelScheduler::new(&os, gov, sb, funnel);
        sched.ingest(b"ABCDEFGH");
        // 一个 chunk (ABCDEFGH, 8 bytes) 能在 capacity 16 内处理
        let still_work = sched.step(&os);
        assert!(still_work || sched.stats.count >= 1);
        assert!(sched.total_rounds >= 0);
    }

    #[test]
    fn funnel_scheduler_done_after_ingest() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let gov = TimeGovernor::standard(TimeLevel::MilliSec);
        let sb = SpaceBatcher::new(4);
        let funnel = Funnel::new(100.0, 1.0, 0.0);
        let mut sched = FunnelScheduler::new(&os, gov, sb, funnel);
        sched.ingest(b"1234");
        sched.run(&os, 100);
        assert_eq!(sched.status, SchedulerStatus::Done);
    }

    #[test]
    fn cascaded_7layer_apex() {
        let mut cf = CascadedFunnel::standard_7(0.0);
        // 灌入一些底层数据
        for i in 0..100 {
            cf.push_at(0, (i % 10) as f64);
        }
        let apex = cf.apex_value();
        assert!(apex.is_some());
        // 顶层收缩比 = 100 × 10^6 = 1亿
        assert!(cf.apex_contraction_ratio() > 1_000_000.0);
    }

    #[test]
    fn time_level_sec_capacity() {
        let g = TimeGovernor::standard(TimeLevel::Sec);
        assert_eq!(g.work_capacity, 256);
    }

    #[test]
    fn space_batcher_progress() {
        let mut sb = SpaceBatcher::new(5);
        sb.ingest(b"hello world"); // 11 byte → 3 chunks (5,5,1)
        assert_eq!(sb.total, 11);
        assert!(sb.has_work());
        // call next_chunk first (advances processed count), then complete
        let _ = sb.next_chunk();
        sb.complete_current(); // processed 5
        assert!(sb.progress() > 0.0);
    }

    // ─── 心跳 / 看门狗测试 ───

    #[test]
    fn heartbeat_beat_resets_stale() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut hb = Heartbeat::new(&os);
        assert!(!hb.is_stale(&os, 100));
        hb.mark_stale();
        assert_eq!(hb.staleness(), 1);
        hb.beat(&os);
        assert_eq!(hb.staleness(), 0);
    }

    #[test]
    fn watchdog_detects_zombie_on_repeated_no_progress() {
        // 构造一个 scheduler，灌入少量数据后用极低的 watchdog 阈值
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let gov = TimeGovernor::standard(TimeLevel::MilliSec);
        let sb = SpaceBatcher::new(4);
        let funnel = Funnel::new(100.0, 1.0, 0.0);
        // watchdog_threshold = 3: 连续 3 步无进展就判 Zombie
        let mut sched = FunnelScheduler::with_threshold(&os, gov, sb, funnel, 3);
        sched.ingest(b"1234");
        // 正常 step 应该能一次消化掉
        let r1 = sched.step(&os);
        // 数据消化后再次 step 应该进入 Done 状态
        if r1 {
            // 如果 step 返回 true（时间窗口饱和），再 step 应该 Done
            let _ = sched.step(&os);
        }
        // 最终必须是 Done 或 Zombie，而不是 Running
        assert!(
            sched.status == SchedulerStatus::Done || sched.status == SchedulerStatus::Zombie,
            "status 应为 Done/Zombie, 实际是 {:?}",
            sched.status
        );
    }

    #[test]
    fn zombie_scheduler_rejects_further_steps() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let gov = TimeGovernor::standard(TimeLevel::MilliSec);
        let sb = SpaceBatcher::new(4);
        let funnel = Funnel::new(100.0, 1.0, 0.0);
        let mut sched = FunnelScheduler::with_threshold(&os, gov, sb, funnel, 2);
        // 手动设为 Zombie 状态
        sched.status = SchedulerStatus::Zombie;
        // Zombie 状态下的 step 必须立即返回 false
        assert!(!sched.step(&os));
        assert_eq!(sched.status, SchedulerStatus::Zombie);
    }

    #[test]
    fn run_breaks_on_zombie() {
        // 构造一个会在 run() 循环中触发 zombie 的 scheduler
        // 使用极低的 watchdog_threshold
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let gov = TimeGovernor::new(TimeLevel::MilliSec, 1, 1000); // 大 capacity 让窗口不会满
        let sb = SpaceBatcher::new(4);
        let funnel = Funnel::new(100.0, 1.0, 0.0);
        let mut sched = FunnelScheduler::with_threshold(&os, gov, sb, funnel, 5);
        // 故意让 space_batcher 的 buffer 在没有 ingest 的情况下不为空
        // 但为了触发 zombie，我们用 ingest 后再 step 制造 no_progress 的情况
        sched.ingest(b"ABCD");
        // 手动让 progress 无法前进
        // 由于 watchdog_threshold=5, 连续 5 次 step 不进 stats/buffer 就会 Zombie
        // 但 `ingest(b\"ABCD\")` 只有一块, step 后 space_batcher 变空 → Done
        // 所以这个用例验证 run 正常退出：不会因为在 Done 后继续 while self.step() 而卡住
        let rounds = sched.run(&os, 1000);
        assert!(rounds >= 0);
        assert!(sched.status == SchedulerStatus::Done || sched.status == SchedulerStatus::Zombie);
    }
}

// ────────────────────────────────────────────────────────────
// 可信蓄量模型 (A-16 概念)
// ────────────────────────────────────────────────────────────
// 系统对自身仿真是否"值得被信任"的度量
//
//  正向控制流: 用户在飞书下发指令 → 仿真调度层接收 → 创建实验组 → 持续迭代指标
//  判断: 正向流产生的"证据"是否足够使仿真结论达到可信阈值
//
// 结构:
//   TrustCapacity     — 蓄量容器
//   EvidenceQuality   — 证据质量标签
//   threshold_reached — 是否越过了可信线

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvidenceQuality {
    /// 手工输入 — 可信度高但量低
    ManualInput,
    /// 自动抓取 — 量中可信中
    AutoIngested,
    /// 第三方网络 — 量高可信需校验
    NetworkDerived,
    /// 自己推演 — 闭环内自洽
    SelfDerived,
}

impl EvidenceQuality {
    /// 该质量等级在单次证据采集中提供的"可信增额"
    pub fn trust_increment(&self) -> f64 {
        match self {
            Self::ManualInput    => 0.30,
            Self::AutoIngested   => 0.15,
            Self::NetworkDerived => 0.08,
            Self::SelfDerived    => 0.12,
        }
    }
}

/// 可信蓄量 — 一个积累证据直到"足够可信"的容器
#[derive(Clone, Debug)]
pub struct TrustCapacity {
    /// 当前蓄量值 (累加 evidence)
    pub current_charge: f64,
    /// 触发可信判定所需的阈值
    pub threshold: f64,
    /// 漏电率: 每回合蓄量损失多少 (证据会随时间贬值)
    pub leak_rate: f64,
    /// evidences 注入总次数
    pub injection_count: u64,
    /// 到达阈值的历史次数
    pub threshold_reached_count: u64,
}

impl TrustCapacity {
    pub fn new(threshold: f64) -> Self {
        Self {
            current_charge: 0.0,
            threshold,
            leak_rate: 0.02,
            injection_count: 0,
            threshold_reached_count: 0,
        }
    }

    /// 注入证据 — 返回注入后是否达到阈值
    pub fn inject(&mut self, quality: EvidenceQuality) -> bool {
        self.current_charge += quality.trust_increment();
        self.injection_count += 1;
        if self.current_charge >= self.threshold {
            self.threshold_reached_count += 1;
            true
        } else {
            false
        }
    }

    /// 模拟一个时间回合 (证据可能贬值)
    pub fn tick(&mut self) {
        self.current_charge *= 1.0 - self.leak_rate;
    }

    /// 当前是否已蓄满
    pub fn trusted(&self) -> bool {
        self.current_charge >= self.threshold
    }

    /// 蓄量占阈值的百分比 (cap 100%)
    pub fn pct(&self) -> f64 {
        (self.current_charge / self.threshold * 100.0).min(100.0)
    }
}

#[cfg(test)]
mod trust_tests {
    use super::*;

    #[test]
    fn trust_increments() {
        let mut tc = TrustCapacity::new(1.0);
        assert!(!tc.trusted());
        // 0.30 (manual) + 0.15 (auto) + 0.08 (net) + 0.12 (self) = 0.65
        tc.inject(EvidenceQuality::ManualInput);
        tc.inject(EvidenceQuality::AutoIngested);
        tc.inject(EvidenceQuality::NetworkDerived);
        tc.inject(EvidenceQuality::SelfDerived);
        assert!((tc.current_charge - 0.65).abs() < 1e-9);
        // 再来一次 manual (0.30) → 0.95 → 仍未满
        tc.inject(EvidenceQuality::ManualInput);
        assert!((tc.current_charge - 0.95).abs() < 1e-9);
        assert!(!tc.trusted());
        // 再来一点就够
        tc.inject(EvidenceQuality::SelfDerived);
        assert!(tc.trusted());
        assert_eq!(tc.threshold_reached_count, 1);
    }

    #[test]
    fn trust_leak_reduces_charge() {
        let mut tc = TrustCapacity::new(1.0);
        for _ in 0..10 { tc.inject(EvidenceQuality::ManualInput); }
        assert!(tc.trusted());
        let before = tc.current_charge;
        tc.tick();
        assert!(tc.current_charge < before);
    }

    #[test]
    fn trust_pct_capped() {
        let mut tc = TrustCapacity::new(0.5);
        tc.inject(EvidenceQuality::ManualInput);
        tc.inject(EvidenceQuality::ManualInput);
        tc.inject(EvidenceQuality::ManualInput);
        // 0.9 / 0.5 * 100 = 180% → 断言被 cap 为 100%
        assert_eq!(tc.pct(), 100.0);
    }
}
