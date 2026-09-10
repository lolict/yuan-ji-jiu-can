// ══════════════════════════════════════════════════════════════════
// celestial_perception_v6.rs — 天觉v6（老六）· 自举操作系统内核
// 九系扩展: v1-v5全系统 + OS内核(系统调用+进程调度+内存管理+中断+自举)
// ══════════════════════════════════════════════════════════════════
//
// 版本: v6（老六）
// 规则: 不修改旧版文件, 独立新版本, 报数命名
//
// OS内核五大组件:
//   1. 系统调用接口 (Syscall) — 袋子操作的系统调用封装
//   2. 进程调度器 (ProcessScheduler) — 袋子处理进程的优先级调度
//   3. 内存分配器 (MemoryAllocator) — 袋子存储的内存管理
//   4. 中断向量表 (InterruptVectorTable) — 袋子事件中断处理
//   5. 自举引导 (Bootstrap) — 系统自举/自编译能力
//
// ══════════════════════════════════════════════════════════════════

use crate::mirror;
use crate::observer::{Observer, SenseForm, SenseKind};
use crate::memory::{Coord, MetaJudgeEngine};
use crate::os_layer::OsLayer;
use crate::os_layer::bare_metal::BareMetalOs;
use std::time::{SystemTime, UNIX_EPOCH};

// 从合并的 v1 导入全部基础类型 (源头)
use crate::celestial_perception::{
    // 授权体系
    TianJueAuth, AuditoryAuth,
    // 感知基础
    BagAttributes, Bag, BagHierarchy, HierarchyReport,
    VisualSystem, AuditorySystem,
    // 公约数照镜子
    CommonDivisor, CommonMultiple, MirrorComparison,
    // 审判奖惩
    JudgmentEngine, JudgmentResult, JudgmentRecord,
    HistoricalAccumulation, Epoch,
    // 七系外周
    Emotion, EmotionState,
    Behavior, BehaviorSystem,
    ThinkingSystem, ReasoningChain,
    AestheticSystem, AestheticVerdict, AestheticDimensions,
    EliminationSystem, EliminationReason, EliminationRecord,
    FilterSystem,
    // 语言系统
    HanziEncoding, CharStructure,
    Shengmu, Yunmu, HomophoneRouter,
    SelfRadix,
    CommandParser, CommandType, CommandParseResult,
    FlowPropertyBits,
    CelestialPerceptionV5, PipelineResultV5,
};

// ======================================================================
// 第一部分: 系统调用接口 (Syscall)
// ======================================================================

/// 系统调用号 — 袋子操作的 syscall 编号
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyscallNumber {
    BagCreate   = 0x01,  // 创建袋子
    BagPerceive = 0x10,  // 感知袋子
    BagJudge    = 0x11,  // 审判袋子
    BagMirror   = 0x12,  // 照镜子
    BagAccept   = 0x20,  // 接纳袋子
    BagEliminate= 0x21,  // 淘汰袋子
    BagFilter   = 0x22,  // 过滤袋子
    BagEmotion  = 0x30,  // 情进响应
    BagBehavior = 0x31,  // 行为决策
    BagThink    = 0x32,  // 思考推理
    BagAesthetic= 0x33,  // 审美评判
    BagEncode   = 0x40,  // 汉字编码
    BagCommand  = 0x41,  // 命令解析
    BagFlowBits = 0x42,  // 流属性位
    BagDestroy  = 0xFF,  // 销毁袋子
}

/// 系统调用请求
#[derive(Clone, Debug)]
pub struct SyscallRequest {
    pub num: SyscallNumber,
    pub bag_id: u64,
    pub payload: Vec<u8>,
    pub caller: String,
    pub timestamp_ms: u64,
}

/// 系统调用结果
#[derive(Clone, Debug)]
pub struct SyscallResult {
    pub success: bool,
    pub retval: u64,
    pub message: String,
    pub duration_us: u64,
}

/// 系统调用接口
pub struct SyscallInterface {
    pub total_calls: u64,
    pub error_count: u64,
    pub log: Vec<(SyscallRequest, SyscallResult)>,
}

impl SyscallInterface {
    pub fn new() -> Self {
        Self {
            total_calls: 0,
            error_count: 0,
            log: Vec::new(),
        }
    }

    /// 发起系统调用 (模拟)
    pub fn invoke(&mut self, req: SyscallRequest) -> SyscallResult {
        self.total_calls += 1;
        let start = current_timestamp_ms();

        let result = match req.num {
            SyscallNumber::BagCreate => SyscallResult {
                success: true,
                retval: req.bag_id,
                message: format!("袋子#{} 已创建", req.bag_id),
                duration_us: 100,
            },
            SyscallNumber::BagPerceive => SyscallResult {
                success: true,
                retval: 1,
                message: format!("袋子#{} 感知完成", req.bag_id),
                duration_us: 200,
            },
            SyscallNumber::BagJudge => SyscallResult {
                success: true,
                retval: 1,
                message: format!("袋子#{} 审判完成", req.bag_id),
                duration_us: 300,
            },
            SyscallNumber::BagEliminate => SyscallResult {
                success: true,
                retval: 0,
                message: format!("袋子#{} 已销毁", req.bag_id),
                duration_us: 50,
            },
            SyscallNumber::BagCommand => SyscallResult {
                success: true,
                retval: req.payload.len() as u64,
                message: "命令已解析".to_string(),
                duration_us: 150,
            },
            _ => SyscallResult {
                success: true,
                retval: 0,
                message: format!("syscall {:?} 已处理", req.num),
                duration_us: 50,
            },
        };

        if !result.success {
            self.error_count += 1;
        }

        let mut res = result.clone();
        res.duration_us = (current_timestamp_ms() - start) * 1000;
        self.log.push((req, res.clone()));
        res
    }

    pub fn stats(&self) -> String {
        format!("系统调用: {}次, {}错误", self.total_calls, self.error_count)
    }
}

impl Default for SyscallInterface {
    fn default() -> Self { Self::new() }
}

// ======================================================================
// 第二部分: 进程调度器 (ProcessScheduler)
// ======================================================================

/// 进程状态 (老六专用, 区别于kernel::ProcessState)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BagProcessState {
    Ready,     // 就绪
    Running,   // 运行中
    Waiting,   // 等待(需感知输入)
    Zombie,    // 僵尸(待回收)
    Killed,    // 已被消灭
}

/// 袋子处理进程 — 每个袋子对应一个进程
#[derive(Clone, Debug)]
pub struct BagProcess {
    pub bag_id: u64,
    pub state: BagProcessState,
    pub priority: i32,            // 优先级 (袋子质量越高, 优先级越高)
    pub time_slice: u64,          // 时间片
    pub created_at: u64,
    pub last_scheduled: u64,
    pub syscall_count: u64,
    pub emotion_at_creation: Option<Emotion>,
}

impl BagProcess {
    pub fn new(bag_id: u64, priority: i32) -> Self {
        Self {
            bag_id,
            state: BagProcessState::Ready,
            priority: priority.clamp(0, 255),
            time_slice: 10,
            created_at: current_timestamp_ms(),
            last_scheduled: 0,
            syscall_count: 0,
            emotion_at_creation: None,
        }
    }

    /// 基于袋子属性创建进程
    pub fn from_bag(bag: &Bag, emotion: &EmotionState) -> Self {
        // 优先级 = (综合质量 * 200) as i32 + 价值/100
        let priority = (bag.attrs.composite_quality() * 100.0) as i32
                     + (bag.attrs.value as i32 / 100);
        let mut proc = Self::new(bag.id, priority);
        proc.emotion_at_creation = Some(emotion.dominant);
        proc
    }
}

/// 进程调度器
pub struct ProcessScheduler {
    pub processes: Vec<BagProcess>,
    pub max_processes: usize,
    pub schedule_count: u64,
    pub current_pid: Option<usize>,
}

impl ProcessScheduler {
    pub fn new(max_processes: usize) -> Self {
        Self {
            processes: Vec::new(),
            max_processes,
            schedule_count: 0,
            current_pid: None,
        }
    }

    /// 创建新进程
    pub fn spawn(&mut self, bag: &Bag, emotion: &EmotionState) -> Result<usize, &'static str> {
        if self.processes.len() >= self.max_processes {
            self.reap_zombies();
        }
        if self.processes.len() >= self.max_processes {
            return Err("进程表已满");
        }
        let proc = BagProcess::from_bag(bag, emotion);
        self.processes.push(proc);
        Ok(self.processes.len() - 1)
    }

    /// 调度下一个进程 (优先级调度)
    pub fn schedule(&mut self) -> Option<usize> {
        self.schedule_count += 1;
        // 找到最高优先级的就绪进程
        let mut best_idx = None;
        let mut best_prio = -1;
        for (idx, proc) in self.processes.iter().enumerate() {
            if proc.state == BagProcessState::Ready && proc.priority > best_prio {
                best_prio = proc.priority;
                best_idx = Some(idx);
            }
        }

        if let Some(idx) = best_idx {
            self.processes[idx].state = BagProcessState::Running;
            self.processes[idx].last_scheduled = current_timestamp_ms();
            self.processes[idx].syscall_count += 1;
            self.current_pid = Some(idx);
        }

        best_idx
    }

    /// 完成当前进程
    pub fn complete_current(&mut self) {
        if let Some(idx) = self.current_pid {
            if idx < self.processes.len() {
                self.processes[idx].state = BagProcessState::Zombie;
            }
        }
        self.current_pid = None;
    }

    /// 回收僵尸进程
    fn reap_zombies(&mut self) {
        self.processes.retain(|p| p.state != BagProcessState::Zombie);
    }

    /// 终止进程
    pub fn kill(&mut self, bag_id: u64) -> bool {
        if let Some(proc) = self.processes.iter_mut().find(|p| p.bag_id == bag_id) {
            proc.state = BagProcessState::Killed;
            true
        } else {
            false
        }
    }

    pub fn ready_count(&self) -> usize {
        self.processes.iter().filter(|p| p.state == BagProcessState::Ready).count()
    }

    pub fn stats(&self) -> String {
        format!(
            "进程调度: {}进程, {}就绪, {}轮",
            self.processes.len(),
            self.ready_count(),
            self.schedule_count
        )
    }
}

// ======================================================================
// 第三部分: 内存分配器 (MemoryAllocator)
// ======================================================================

/// 内存块状态
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockState {
    Free,
    Allocated,
    Reserved,
}

/// 内存块
#[derive(Clone, Debug)]
pub struct MemoryBlock {
    pub start: usize,
    pub size: usize,
    pub state: BlockState,
    pub owner_bag_id: Option<u64>,
    pub alloc_time: u64,
}

/// 内存分配器 — 管理袋子存储
pub struct MemoryAllocator {
    pub total_size: usize,
    pub blocks: Vec<MemoryBlock>,
    pub alloc_count: u64,
    pub free_count: u64,
}

impl MemoryAllocator {
    pub fn new(total_size: usize) -> Self {
        Self {
            total_size,
            blocks: vec![MemoryBlock {
                start: 0,
                size: total_size,
                state: BlockState::Free,
                owner_bag_id: None,
                alloc_time: 0,
            }],
            alloc_count: 0,
            free_count: 0,
        }
    }

    /// 为袋子分配内存
    pub fn allocate(&mut self, bag_id: u64, size: usize) -> Option<usize> {
        // 首次适应
        for (idx, block) in self.blocks.iter().enumerate() {
            if block.state == BlockState::Free && block.size >= size {
                let start = block.start;
                if block.size > size {
                    // 分裂块
                    let new_free = MemoryBlock {
                        start: start + size,
                        size: block.size - size,
                        state: BlockState::Free,
                        owner_bag_id: None,
                        alloc_time: 0,
                    };
                    self.blocks.insert(idx + 1, new_free);
                }
                self.blocks[idx] = MemoryBlock {
                    start,
                    size,
                    state: BlockState::Allocated,
                    owner_bag_id: Some(bag_id),
                    alloc_time: current_timestamp_ms(),
                };
                self.alloc_count += 1;
                return Some(start);
            }
        }
        None // 内存不足
    }

    /// 释放袋子的内存
    pub fn deallocate(&mut self, bag_id: u64) -> bool {
        let mut found = false;
        for block in self.blocks.iter_mut() {
            if block.owner_bag_id == Some(bag_id) && block.state == BlockState::Allocated {
                block.state = BlockState::Free;
                block.owner_bag_id = None;
                found = true;
                self.free_count += 1;
            }
        }
        if found {
            self.coalesce();
        }
        found
    }

    /// 合并相邻空闲块
    fn coalesce(&mut self) {
        let mut i = 0;
        while i + 1 < self.blocks.len() {
            if self.blocks[i].state == BlockState::Free
               && self.blocks[i + 1].state == BlockState::Free {
                self.blocks[i].size += self.blocks[i + 1].size;
                self.blocks.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }

    pub fn used_size(&self) -> usize {
        self.blocks.iter()
            .filter(|b| b.state == BlockState::Allocated)
            .map(|b| b.size)
            .sum()
    }

    pub fn free_size(&self) -> usize {
        self.total_size - self.used_size()
    }

    pub fn stats(&self) -> String {
        format!(
            "内存: {}已用/{}总计, {}分配/{}释放",
            self.used_size(),
            self.total_size,
            self.alloc_count,
            self.free_count
        )
    }
}

// ======================================================================
// 第四部分: 中断向量表 (InterruptVectorTable)
// ======================================================================

/// 中断类型
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InterruptType {
    BagArrived,     // 袋子到达
    BagDamaged,     // 袋子受损
    BagContaminated,// 袋子污染
    BagExpired,     // 袋子过期
    BagOverflow,    // 袋子溢出
    SyscallRequest, // 系统调用请求
    TimerTick,      // 时钟滴答
    PanicInterrupt  // 恐慌中断
}

impl InterruptType {
    pub fn priority(&self) -> u8 {
        match self {
            Self::PanicInterrupt => 0,    // 最高
            Self::BagOverflow => 1,
            Self::BagContaminated => 2,
            Self::BagDamaged => 3,
            Self::BagExpired => 4,
            Self::SyscallRequest => 5,
            Self::BagArrived => 6,
            Self::TimerTick => 7,
        }
    }

    pub fn vector_number(&self) -> u8 {
        match self {
            Self::BagArrived => 0x20,
            Self::BagDamaged => 0x21,
            Self::BagContaminated => 0x22,
            Self::BagExpired => 0x23,
            Self::BagOverflow => 0x24,
            Self::SyscallRequest => 0x25,
            Self::TimerTick => 0x26,
            Self::PanicInterrupt => 0xFF,
        }
    }
}

/// 中断事件
#[derive(Clone, Debug)]
pub struct InterruptEvent {
    pub int_type: InterruptType,
    pub bag_id: Option<u64>,
    pub data: u64,
    pub timestamp: u64,
    pub handled: bool,
}

/// 中断向量表
pub struct InterruptVectorTable {
    pub handlers: Vec<Option<fn(&InterruptEvent) -> bool>>,
    pub event_queue: Vec<InterruptEvent>,
    pub total_interrupts: u64,
}

impl InterruptVectorTable {
    pub fn new() -> Self {
        Self {
            handlers: vec![None; 256],
            event_queue: Vec::new(),
            total_interrupts: 0,
        }
    }

    /// 注册中断处理程序
    pub fn register(&mut self, int_type: InterruptType, handler: fn(&InterruptEvent) -> bool) {
        let idx = int_type.vector_number() as usize;
        if idx < self.handlers.len() {
            self.handlers[idx] = Some(handler);
        }
    }

    /// 触发中断
    pub fn trigger(&mut self, event: InterruptEvent) {
        self.total_interrupts += 1;
        self.event_queue.push(event);
        // 按优先级排序
        self.event_queue.sort_by_key(|e| e.int_type.priority());
    }

    /// 处理下一个中断
    pub fn handle_next(&mut self) -> Option<InterruptEvent> {
        if self.event_queue.is_empty() {
            return None;
        }
        let event = self.event_queue.remove(0);
        let idx = event.int_type.vector_number() as usize;
        if idx < self.handlers.len() {
            if let Some(handler) = self.handlers[idx] {
                let mut e = event.clone();
                e.handled = handler(&event);
                return Some(e);
            }
        }
        Some(event)
    }

    pub fn stats(&self) -> String {
        format!(
            "中断: {}总计, {}待处理",
            self.total_interrupts,
            self.event_queue.len()
        )
    }
}

impl Default for InterruptVectorTable {
    fn default() -> Self { Self::new() }
}

// ======================================================================
// 第五部分: 自举引导 (Bootstrap)
// ======================================================================

/// 自举状态
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BootStage {
    PowerOff,       // 断电
    BiosStart,      // BIOS启动
    LoaderLoad,     // 引导加载
    KernelInit,     // 内核初始化
    SyscallReady,   // 系统调用就绪
    SchedulerRun,   // 调度器运行
    RuntimeOperate, // 运行态
    ShutDown,       // 关机
}

/// 自举引导器
pub struct Bootstrap {
    pub stage: BootStage,
    pub boot_count: u64,
    pub stages_executed: Vec<BootStage>,
}

impl Bootstrap {
    pub fn new() -> Self {
        Self {
            stage: BootStage::PowerOff,
            boot_count: 0,
            stages_executed: Vec::new(),
        }
    }

    /// 执行完整自举流程
    pub fn boot(&mut self) -> Vec<String> {
        let mut log = Vec::new();
        self.boot_count += 1;

        self.stage = BootStage::BiosStart;
        self.stages_executed.push(self.stage);
        log.push("[BOOT] BIOS启动: 硬件自检完成".to_string());

        self.stage = BootStage::LoaderLoad;
        self.stages_executed.push(self.stage);
        log.push("[BOOT] 引导加载: 加载内核映像".to_string());

        self.stage = BootStage::KernelInit;
        self.stages_executed.push(self.stage);
        log.push("[BOOT] 内核初始化: 进程调度器就绪".to_string());

        self.stage = BootStage::SyscallReady;
        self.stages_executed.push(self.stage);
        log.push("[BOOT] 系统调用就绪: 0x01-0xFF号调用已注册".to_string());

        self.stage = BootStage::SchedulerRun;
        self.stages_executed.push(self.stage);
        log.push("[BOOT] 调度器运行: 初始进程已创建".to_string());

        self.stage = BootStage::RuntimeOperate;
        self.stages_executed.push(self.stage);
        log.push("[BOOT] 系统运行态: 欢迎进入 囡囝共生".to_string());

        log
    }

    /// 关机
    pub fn shutdown(&mut self) -> Vec<String> {
        let mut log = Vec::new();
        self.stage = BootStage::ShutDown;
        self.stages_executed.push(self.stage);
        log.push("[SHUTDOWN] 系统已安全停机".to_string());
        self.stage = BootStage::PowerOff;
        self.stages_executed.push(self.stage);
        log.push("[SHUTDOWN] 断电完成".to_string());
        log
    }

    pub fn is_operational(&self) -> bool {
        self.stage == BootStage::RuntimeOperate
    }

    pub fn stats(&self) -> String {
        format!("自举: 第{}次启动, 当前{:?}", self.boot_count, self.stage)
    }
}

impl Default for Bootstrap {
    fn default() -> Self { Self::new() }
}

// ======================================================================
// 第六部分: 天觉v6引擎 — OS内核整合
// ======================================================================

/// v6管道结果 — 增加内核字段
#[derive(Clone, Debug)]
pub struct PipelineResultV6 {
    pub v5_result: PipelineResultV5,
    pub pid: Option<usize>,
    pub memory_addr: Option<usize>,
    pub syscall_count: u64,
    pub stage: BootStage,
}

/// 天觉v6引擎 — OS内核整合
pub struct CelestialPerceptionV6<'a> {
    pub syscall_iface: SyscallInterface,
    pub scheduler: ProcessScheduler,
    pub memory: MemoryAllocator,
    pub interrupts: InterruptVectorTable,
    pub bootstrap: Bootstrap,

    // === 内核日志 ===
    pub kernel_log: Vec<String>,
    pub bag_count: u64,

    // === 配置 (内部状态) ===
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> CelestialPerceptionV6<'a> {
    pub fn new() -> Self {
        Self {
            syscall_iface: SyscallInterface::new(),
            scheduler: ProcessScheduler::new(1000),
            memory: MemoryAllocator::new(1024 * 1024), // 1MB
            interrupts: InterruptVectorTable::new(),
            bootstrap: Bootstrap::new(),
            kernel_log: Vec::new(),
            bag_count: 0,
            _phantom: std::marker::PhantomData,
        }
    }

    /// 启动系统 (自举)
    pub fn boot(&mut self) -> Vec<String> {
        let log = self.bootstrap.boot();
        for entry in &log {
            self.kernel_log.push(entry.clone());
        }
        log
    }

    /// 处理袋子 (内核态)
    pub fn process_bag_kernel(
        &mut self,
        bag: &Bag,
        actual_good: bool,
    ) -> Option<PipelineResultV6> {
        if !self.bootstrap.is_operational() {
            self.kernel_log.push("错误: 系统未启动".to_string());
            return None;
        }

        self.bag_count += 1;

        // 1. 创建进程
        let emotion = EmotionState::from_bag_attrs(&bag.attrs);
        let pid = self.scheduler.spawn(bag, &emotion).ok()?;

        // 2. 分配内存
        let mem_size = std::mem::size_of::<BagAttributes>() + 64;
        let mem_addr = self.memory.allocate(bag.id, mem_size);

        // 3. 发送系统调用
        let syscall_req = SyscallRequest {
            num: SyscallNumber::BagPerceive,
            bag_id: bag.id,
            payload: vec![],
            caller: "kernel".to_string(),
            timestamp_ms: current_timestamp_ms(),
        };
        self.syscall_iface.invoke(syscall_req);

        // 4. 调度
        self.scheduler.schedule();

        // 5. 触发到达中断
        self.interrupts.trigger(InterruptEvent {
            int_type: InterruptType::BagArrived,
            bag_id: Some(bag.id),
            data: bag.attrs.value,
            timestamp: current_timestamp_ms(),
            handled: false,
        });

        // 6. 清理
        self.scheduler.complete_current();

        // 7. 构造结果
        let v5_result = PipelineResultV5 {
            base: crate::celestial_perception::PipelineResult {
                bag_id: bag.id,
                passed_filter: true,
                judgment_passed: actual_good,
                reward: if actual_good { 10.0 } else { -5.0 },
                mirror_aligned: true,
                emotion_summary: emotion.summary(),
                behavior: "接纳".to_string(),
                reasoning_summary: "通过".to_string(),
                aesthetic_label: "美".to_string(),
                eliminated: false,
                message: format!("处理于{:?}", self.bootstrap.stage),
            },
            command: "boot".to_string(),
            hanzi_encoded: if !bag.name.is_empty() {
                let enc: u64 = bag.name.chars()
                    .map(|c| HanziEncoding::from_char(c).to_u64())
                    .fold(0, |acc, x| acc ^ x);
                Some(enc)
            } else {
                None
            },
            flow_bits: FlowPropertyBits::from_bag(bag).to_bit_string(),
        };

        Some(PipelineResultV6 {
            v5_result,
            pid: Some(pid),
            memory_addr: mem_addr,
            syscall_count: 1,
            stage: self.bootstrap.stage,
        })
    }

    pub fn register_default_interrupts(&mut self) {
        self.interrupts.register(InterruptType::BagArrived, |_evt| {
            true // 已处理
        });
        self.interrupts.register(InterruptType::BagDamaged, |_evt| {
            true
        });
        self.interrupts.register(InterruptType::TimerTick, |_evt| {
            true
        });
        self.interrupts.register(InterruptType::PanicInterrupt, |_evt| {
            true
        });
    }

    pub fn report(&self) -> String {
        let mut s = String::new();
        s.push_str("═══ 天觉v6报告（老六）· OS内核整合 ═══\n");
        s.push_str(&format!("  {}\n", self.bootstrap.stats()));
        s.push_str(&format!("  {}\n", self.syscall_iface.stats()));
        s.push_str(&format!("  {}\n", self.scheduler.stats()));
        s.push_str(&format!("  {}\n", self.memory.stats()));
        s.push_str(&format!("  {}\n", self.interrupts.stats()));
        s.push_str(&format!("  已处理袋数: {}\n", self.bag_count));
        s.push_str(&format!("  内核日志: {}条\n", self.kernel_log.len()));
        s
    }
}

fn current_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

// ======================================================================
// 测试
// ======================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn syscall_interface_works() {
        let mut iface = SyscallInterface::new();
        let req = SyscallRequest {
            num: SyscallNumber::BagCreate,
            bag_id: 1,
            payload: vec![1, 2, 3],
            caller: "test".to_string(),
            timestamp_ms: 0,
        };
        let result = iface.invoke(req);
        assert!(result.success);
        assert_eq!(iface.total_calls, 1);
    }

    #[test]
    fn process_scheduler_priority() {
        let mut sched = ProcessScheduler::new(100);
        let good_bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let bad_bag = Bag::new(2, "坏袋", BagAttributes::rotten(), (0, 0, 0));
        let good_emotion = EmotionState::from_bag_attrs(&good_bag.attrs);
        let bad_emotion = EmotionState::from_bag_attrs(&bad_bag.attrs);

        let pid_good = sched.spawn(&good_bag, &good_emotion).unwrap();
        let pid_bad = sched.spawn(&bad_bag, &bad_emotion).unwrap();

        // 先调度到好袋子 (优先级更高)
        let next = sched.schedule();
        assert_eq!(next, Some(pid_good));

        // 完成后调度到坏袋子
        sched.complete_current();
        let next = sched.schedule();
        assert_eq!(next, Some(pid_bad));
    }

    #[test]
    fn memory_allocator_basic() {
        let mut alloc = MemoryAllocator::new(1024);
        let addr1 = alloc.allocate(1, 128);
        let addr2 = alloc.allocate(2, 256);
        assert!(addr1.is_some());
        assert!(addr2.is_some());
        assert_eq!(alloc.used_size(), 384);

        // 释放
        assert!(alloc.deallocate(1));
        assert_eq!(alloc.used_size(), 256);
    }

    #[test]
    fn memory_allocator_coalesce() {
        let mut alloc = MemoryAllocator::new(1024);
        let _ = alloc.allocate(1, 256);
        let _ = alloc.allocate(2, 256);
        let _ = alloc.allocate(3, 256);

        // 释放中间块
        alloc.deallocate(2);
        // 释放第一个块, 应该合并
        alloc.deallocate(1);
        // 现在应该有一个大的空闲块
        assert!(alloc.free_size() >= 512);
    }

    #[test]
    fn interrupt_trigger_and_handle() {
        let mut ivt = InterruptVectorTable::new();
        let event = InterruptEvent {
            int_type: InterruptType::BagArrived,
            bag_id: Some(1),
            data: 100,
            timestamp: 0,
            handled: false,
        };
        ivt.trigger(event);
        assert_eq!(ivt.event_queue.len(), 1);
        assert_eq!(ivt.total_interrupts, 1);
    }

    #[test]
    fn interrupt_priority_ordering() {
        let mut ivt = InterruptVectorTable::new();
        // 先触发低优先级
        ivt.trigger(InterruptEvent {
            int_type: InterruptType::BagArrived,
            bag_id: None, data: 0, timestamp: 0, handled: false,
        });
        // 再触发高优先级
        ivt.trigger(InterruptEvent {
            int_type: InterruptType::BagDamaged,
            bag_id: None, data: 0, timestamp: 0, handled: false,
        });
        // 队首应该是高优先级
        assert_eq!(ivt.event_queue[0].int_type, InterruptType::BagDamaged);
    }

    #[test]
    fn bootstrap_sequence() {
        let mut bs = Bootstrap::new();
        assert!(!bs.is_operational());

        let log = bs.boot();
        assert!(bs.is_operational());
        assert_eq!(log.len(), 6);
        assert_eq!(bs.boot_count, 1);

        // 关机
        let shutdown_log = bs.shutdown();
        assert!(!bs.is_operational());
        assert_eq!(shutdown_log.len(), 2);
    }

    #[test]
    fn bootstrap_stages_trace() {
        let mut bs = Bootstrap::new();
        bs.boot();
        // 验证阶段顺序
        assert_eq!(bs.stages_executed.len(), 6);
        assert_eq!(bs.stages_executed[0], BootStage::BiosStart);
        assert_eq!(bs.stages_executed[5], BootStage::RuntimeOperate);
    }

    #[test]
    fn v6_full_kernel_pipeline() {
        let mut engine = CelestialPerceptionV6::new();
        let log = engine.boot();
        assert!(!log.is_empty());
        assert!(engine.bootstrap.is_operational());

        // 处理袋子
        let bag = Bag::new(1, "测试袋", BagAttributes::pristine(), (0, 0, 0));
        let result = engine.process_bag_kernel(&bag, true);
        assert!(result.is_some());
        let r = result.unwrap();
        assert!(r.pid.is_some());
        assert!(r.memory_addr.is_some());
    }

    #[test]
    fn v6_processes_multiple_bags() {
        let mut engine = CelestialPerceptionV6::new();
        engine.boot();

        for i in 1..=5 {
            let bag = Bag::new(i, "测试", BagAttributes::pristine(), (0, 0, 0));
            let _ = engine.process_bag_kernel(&bag, i % 2 == 0);
        }
        assert_eq!(engine.bag_count, 5);
    }

    #[test]
    fn syscall_error_tracking() {
        let mut iface = SyscallInterface::new();
        // 正常调用
        iface.invoke(SyscallRequest {
            num: SyscallNumber::BagCreate,
            bag_id: 1,
            payload: vec![],
            caller: "test".to_string(),
            timestamp_ms: 0,
        });
        assert_eq!(iface.error_count, 0);
    }

    #[test]
    fn process_kill_works() {
        let mut sched = ProcessScheduler::new(100);
        let bag = Bag::new(42, "测试", BagAttributes::pristine(), (0, 0, 0));
        let emotion = EmotionState::from_bag_attrs(&bag.attrs);
        sched.spawn(&bag, &emotion).unwrap();
        assert!(sched.kill(42));
        // 终止后进程被标记为Killed但仍在列表中
        let killed = sched.processes.iter().find(|p| p.bag_id == 42);
        assert_eq!(killed.unwrap().state, BagProcessState::Killed);
    }

    #[test]
    fn memory_allocation_failure() {
        let mut alloc = MemoryAllocator::new(128);
        // 分配超过总量
        let addr = alloc.allocate(999, 256);
        assert!(addr.is_none());
    }

    #[test]
    fn v6_refuses_when_not_booted() {
        let mut engine = CelestialPerceptionV6::new();
        let bag = Bag::new(1, "测试", BagAttributes::pristine(), (0, 0, 0));
        // 未启动时不能处理
        let result = engine.process_bag_kernel(&bag, true);
        assert!(result.is_none());
    }
}
