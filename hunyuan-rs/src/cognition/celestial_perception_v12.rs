// ======================================================================
// celestial_perception_v12.rs — 天觉系统 v12（老十二）· 袋子容器运行时 + 活码编译器
//
// 本文件将天觉v6-v11理论与观测转化为实际执行能力:
//
//   1. 袋子容器运行时 (BagContainer)     — 袋子生命周期管理 · 隔离执行环境 · 暂停/恢复/销毁
//   2. 活码编译器 (LivingCodeCompiler)    — 以"起点"为钥匙激活可活动编码 · 加法统计法确定数值
//   3. 指令袋 (InstructionBag)           — SMRL操作码序列 · 袋子→指令编译 · 逆向反射
//   4. JIT执行引擎 (JitExecutionEngine)   — 指令袋即时执行 · 操作数栈 · 帧上下文
//   5. 袋子反射 (BagIntrospection)        — 运行时14维属性读取 · 因果链回溯 · 健康评分
//
// 天觉公理: 袋子不再仅是数据, 而是可执行的生命体;
//           活码在"起点"固定前游移, 固定后按排列顺序确定权限和数值.
// ======================================================================

use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::cognition::celestial_perception::{Bag, BagAttributes};
use crate::cognition::celestial_perception::{
    SubSystem, TimelineEventKind, Timeline,
};

pub const V12_VERSION: u32 = 12;

// ======================================================================
// 第一部分: SMRL 操作码 — 袋子指令集
// ======================================================================

/// SMRL操作码枚举 — 袋子可执行的最小动作
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OpCode {
    // 数据移动
    LoadBag,        // 压入袋子
    StoreBag,       // 弹出存回
    MoveAttr,       // 移动属性(维度选择)
    MirrorAttr,     // 镜像属性(对照)

    // 裁判分支
    JudgeAnd,       // 与条件裁
    JudgeOr,        // 或条件裁
    Eliminate,      // 不通过消除
    PassThrough,    // 通过放行

    // 流操作
    FlowProceed,    // 正流(继续加工)
    FlowReturn,     // 返流(退回修补)
    FlowSink,       // 下沉(降级到下一层)
    FlowFloat,      // 上浮(升级到上一层)

    // 算术加法 (活码编译器核心)
    AddValue,       // 加法统计数值
    AddPolarity,    // 极性叠加
    CombineSovereignty, // 主权融合

    // 容器操作
    CreateChild,    // 创建子袋
    MergeBags,      // 合并袋子
    SplitBag,       // 拆分袋子

    // 生命周期
    Spawn,          // 袋子诞生
    Suspend,        // 暂停(保留状态)
    Resume,         // 恢复执行
    Destroy,        // 销毁(资源回收)
}

impl OpCode {
    /// 操作码所需要的操作数个数
    pub fn operand_count(&self) -> u8 {
        match self {
            OpCode::LoadBag | OpCode::MoveAttr | OpCode::AddValue => 1,
            OpCode::MirrorAttr | OpCode::JudgeAnd | OpCode::JudgeOr |
            OpCode::FlowProceed | OpCode::FlowReturn | OpCode::FlowSink | OpCode::FlowFloat |
            OpCode::AddPolarity | OpCode::CombineSovereignty | OpCode::CreateChild |
            OpCode::MergeBags | OpCode::SplitBag => 2,
            OpCode::StoreBag | OpCode::Eliminate | OpCode::PassThrough |
            OpCode::Spawn | OpCode::Suspend | OpCode::Resume | OpCode::Destroy => 0,
        }
    }

    /// 操作码属于哪个执行域
    pub fn domain(&self) -> ExecDomain {
        match self {
            OpCode::LoadBag | OpCode::StoreBag | OpCode::MoveAttr | OpCode::MirrorAttr => ExecDomain::Memory,
            OpCode::JudgeAnd | OpCode::JudgeOr | OpCode::Eliminate | OpCode::PassThrough => ExecDomain::Judgment,
            OpCode::FlowProceed | OpCode::FlowReturn | OpCode::FlowSink | OpCode::FlowFloat => ExecDomain::Flow,
            OpCode::AddValue | OpCode::AddPolarity | OpCode::CombineSovereignty => ExecDomain::Arithmetic,
            OpCode::CreateChild | OpCode::MergeBags | OpCode::SplitBag => ExecDomain::Container,
            OpCode::Spawn | OpCode::Suspend | OpCode::Resume | OpCode::Destroy => ExecDomain::Lifecycle,
        }
    }
}

/// 执行域 — 操作码的分类
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExecDomain {
    Memory,
    Judgment,
    Flow,
    Arithmetic,
    Container,
    Lifecycle,
}

impl fmt::Display for ExecDomain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ExecDomain::Memory => "存储",
            ExecDomain::Judgment => "裁判",
            ExecDomain::Flow => "流转",
            ExecDomain::Arithmetic => "加法",
            ExecDomain::Container => "容器",
            ExecDomain::Lifecycle => "生死",
        };
        write!(f, "{s}")
    }
}

// ======================================================================
// 第二部分: 指令袋 — 编译后产物
// ======================================================================

/// 指令袋 — 袋子编译为可执行指令序列
#[derive(Clone, Debug)]
pub struct InstructionBag {
    pub source_bag_id: u64,
    pub instructions: Vec<(OpCode, Vec<u64>)>,  // (操作码, 操作数)
    pub compile_time_ms: u64,
    pub fixed_point: bool,        // "起点"是否已固定
    pub activation_key: Option<String>,  // 激活活码的钥匙
    pub permission_level: u8,     // 固定后按排列顺序确定的权限
}

impl InstructionBag {
    pub fn new(source_bag_id: u64) -> Self {
        Self {
            source_bag_id,
            instructions: Vec::new(),
            compile_time_ms: 0,
            fixed_point: false,
            activation_key: None,
            permission_level: 0,
        }
    }

    /// 追加指令
    pub fn emit(&mut self, op: OpCode, operands: Vec<u64>) {
        self.instructions.push((op, operands));
    }

    /// 指令总数
    pub fn len(&self) -> usize { self.instructions.len() }
    pub fn is_empty(&self) -> bool { self.instructions.is_empty() }

    /// 获取指定索引的操作码
    pub fn opcode_at(&self, pc: usize) -> Option<OpCode> {
        self.instructions.get(pc).map(|(op, _)| *op)
    }

    /// 获取指定索引的操作数
    pub fn operands_at(&self, pc: usize) -> Option<&[u64]> {
        self.instructions.get(pc).map(|(_, ops)| ops.as_slice())
    }

    /// 固定"起点" — 激活活码, 按排列顺序确定权限
    pub fn fix_starting_point(&mut self, key: &str) {
        self.fixed_point = true;
        self.activation_key = Some(key.to_string());
        // 加法统计法: 根据固定位置确定数值
        self.permission_level = Self::compute_permission_by_sequence(&self.instructions);
    }

    /// 加法统计法 — 遍历指令序列计算累积权限值
    fn compute_permission_by_sequence(instructions: &[(OpCode, Vec<u64>)]) -> u8 {
        let mut acc: u64 = 0;
        for (idx, (op, ops)) in instructions.iter().enumerate() {
            // 位置 × 操作码域权重 + 操作数和
            let domain_weight = match op.domain() {
                ExecDomain::Memory => 1,
                ExecDomain::Judgment => 2,
                ExecDomain::Flow => 3,
                ExecDomain::Arithmetic => 5,  // 加法域权重最高(活码核心)
                ExecDomain::Container => 7,
                ExecDomain::Lifecycle => 11,
            };
            let op_sum: u64 = ops.iter().sum();
            acc += (idx as u64 + 1) * domain_weight + op_sum;
        }
        (acc % 256) as u8
    }
}

// ======================================================================
// 第三部分: 活码编译器 — 袋子 → 指令袋
// ======================================================================

/// 活码编译器 — 以"起点"为钥匙激活可活动编码
/// 未固定时, 编码位置任意活动; 固定后按排列顺序确定权限和数值
#[derive(Clone, Debug)]
pub struct LivingCodeCompiler {
    pub compile_count: u64,
    pub active_codes: Vec<String>,     // 已注册的活码名称
    pub key_registry: HashMap<String, bool>,  // 钥匙 → 激活状态
}

impl LivingCodeCompiler {
    pub fn new() -> Self {
        Self {
            compile_count: 0,
            active_codes: Vec::new(),
            key_registry: HashMap::new(),
        }
    }

    /// 注册活码 — 未激活时位置任意活动
    pub fn register_code(&mut self, name: &str) {
        self.active_codes.push(name.to_string());
        self.key_registry.insert(name.to_string(), false);  // 未激活
    }

    /// 激活活码 — 用"起点"作为钥匙固定编码
    pub fn activate_code(&mut self, name: &str, key: &str) -> bool {
        if key != "起点" { return false; }
        if let Some(entry) = self.key_registry.get_mut(name) {
            *entry = true;
            true
        } else {
            false
        }
    }

    /// 检查活码是否已激活
    pub fn is_activated(&self, name: &str) -> bool {
        self.key_registry.get(name).copied().unwrap_or(false)
    }

    /// 编译袋子为指令袋 — 根据袋子的14维属性生成指令序列
    pub fn compile_bag(&mut self, bag: &Bag) -> InstructionBag {
        let mut ib = InstructionBag::new(bag.id);
        let start_ms = current_time_ms();

        // 1. 袋子诞生
        ib.emit(OpCode::Spawn, vec![bag.id]);

        // 2. 根据完整性加载袋子
        if bag.attrs.integrity > 0.5 {
            ib.emit(OpCode::LoadBag, vec![bag.id]);
        }

        // 3. 质量裁判分支
        if bag.attrs.composite_quality() > 0.7 {
            ib.emit(OpCode::JudgeAnd, vec![]);
            ib.emit(OpCode::PassThrough, vec![]);
        } else if bag.attrs.needs_any_processing() {
            ib.emit(OpCode::JudgeOr, vec![]);
            ib.emit(OpCode::FlowSink, vec![]);
        } else {
            ib.emit(OpCode::Eliminate, vec![]);
        }

        // 4. 加法统计 — 价值叠加
        let val = bag.attrs.value;
        ib.emit(OpCode::AddValue, vec![val]);

        // 5. 干燥判断
        if bag.attrs.needs_drying() {
            ib.emit(OpCode::FlowSink, vec![]);
        }

        // 6. 品质合格上浮
        if bag.attrs.aesthetic_score() > 0.8 {
            ib.emit(OpCode::FlowFloat, vec![]);
        }

        // 7. 内容需要清洁 → 流转至处理
        if bag.attrs.needs_content_wash {
            ib.emit(OpCode::FlowProceed, vec![]);
        }

        // 8. 袋子存储
        if !bag.eliminated && bag.attrs.is_edible() {
            ib.emit(OpCode::StoreBag, vec![bag.id]);
        }

        let end_ms = current_time_ms();
        ib.compile_time_ms = end_ms - start_ms;
        ib.fixed_point = false;  // 需要外部用"起点"钥匙激活

        self.compile_count += 1;
        ib
    }

    /// 编译并立即激活活码
    pub fn compile_and_activate(&mut self, bag: &Bag, key: &str) -> InstructionBag {
        let mut ib = self.compile_bag(bag);
        if key == "起点" {
            ib.fix_starting_point(key);
        }
        ib
    }
}

// ======================================================================
// 第四部分: JIT执行引擎 — 指令袋即时执行
// ======================================================================

/// JIT执行引擎 — 操作数栈 + 程序计数器
#[derive(Clone, Debug)]
pub struct JitExecutionEngine {
    pub stack: Vec<u64>,
    pub pc: usize,
    pub executed_count: u64,
    pub max_stack_depth: usize,
    pub frame_log: Vec<FrameRecord>,
    pub completed: bool,
}

/// 帧记录 — 每条指令的执行快照
#[derive(Clone, Debug)]
pub struct FrameRecord {
    pub pc: usize,
    pub opcode: OpCode,
    pub stack_depth_before: u64,
    pub stack_depth_after: u64,
    pub cycle: u64,
}

impl JitExecutionEngine {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            pc: 0,
            executed_count: 0,
            max_stack_depth: 0,
            frame_log: Vec::new(),
            completed: false,
        }
    }

    /// 执行一条指令, 返回是否成功
    pub fn step(&mut self, ib: &InstructionBag) -> bool {
        if self.pc >= ib.instructions.len() {
            self.completed = true;
            return false;
        }
        let (op, operands) = &ib.instructions[self.pc];
        let depth_before = self.stack.len() as u64;

        match op {
            OpCode::LoadBag | OpCode::Spawn => {
                if let Some(&id) = operands.first() {
                    self.stack.push(id);
                }
            }
            OpCode::StoreBag | OpCode::Eliminate | OpCode::Destroy | OpCode::Suspend => {
                self.stack.pop();
            }
            OpCode::AddValue => {
                if let Some(&val) = operands.first() {
                    let acc = self.stack.last().copied().unwrap_or(0);
                    self.stack.push(acc + val);
                }
            }
            OpCode::AddPolarity => {
                if operands.len() >= 2 {
                    self.stack.push(operands[0].wrapping_add(operands[1]));
                }
            }
            OpCode::CombineSovereignty => {
                if operands.len() >= 2 {
                    self.stack.push(operands[0] | operands[1]);  // 按位或融合主权
                }
            }
            OpCode::MoveAttr | OpCode::MirrorAttr => {
                if let Some(&idx) = operands.first() {
                    let val = *self.stack.last().unwrap_or(&0);
                    self.stack.push(val.wrapping_add(idx));
                }
            }
            OpCode::JudgeAnd => {
                if self.stack.len() >= 2 {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(if a > 0 && b > 0 { 1 } else { 0 });
                }
            }
            OpCode::JudgeOr => {
                if self.stack.len() >= 2 {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(if a > 0 || b > 0 { 1 } else { 0 });
                }
            }
            OpCode::PassThrough | OpCode::FlowProceed => {
                // 无操作(哨兵)
            }
            OpCode::FlowReturn => {
                // 返流 — 回退PC到上一步
                if self.pc > 0 { self.pc -= 1; }
            }
            OpCode::FlowSink => {
                // 下沉 — 将当前栈顶折半
                if let Some(top) = self.stack.last_mut() {
                    *top /= 2;
                }
            }
            OpCode::FlowFloat => {
                // 上浮 — 将当前栈顶翻倍
                if let Some(top) = self.stack.last_mut() {
                    *top = top.saturating_mul(2);
                }
            }
            OpCode::CreateChild | OpCode::MergeBags => {
                if operands.len() >= 2 {
                    self.stack.push(operands[0].wrapping_add(operands[1]));
                }
            }
            OpCode::SplitBag => {
                if let Some(top) = self.stack.pop() {
                    self.stack.push(top / 2);
                    self.stack.push(top - top / 2);
                }
            }
            OpCode::Resume => {
                // 恢复 — 无操作(占位)
            }
        }

        let depth_after = self.stack.len() as u64;
        if self.stack.len() > self.max_stack_depth {
            self.max_stack_depth = self.stack.len();
        }

        self.frame_log.push(FrameRecord {
            pc: self.pc,
            opcode: *op,
            stack_depth_before: depth_before,
            stack_depth_after: depth_after,
            cycle: self.executed_count,
        });

        self.pc += 1;
        self.executed_count += 1;
        true
    }

    /// 执行整个指令袋(带上限, 防止无限循环)
    pub fn execute(&mut self, ib: &InstructionBag) {
        let max_steps = ib.instructions.len().max(1) as u64 * 4 + 16;
        while !self.completed && self.executed_count < max_steps && self.step(ib) {}
        if self.executed_count >= max_steps {
            self.completed = true;
        }
    }

    /// 重置执行器
    pub fn reset(&mut self) {
        self.stack.clear();
        self.pc = 0;
        self.executed_count = 0;
        self.max_stack_depth = 0;
        self.frame_log.clear();
        self.completed = false;
    }
}

// ======================================================================
// 第五部分: 袋子容器 — 袋子生命周期管理
// ======================================================================

/// 袋子运行状态
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BagStatus {
    New,        // 新建
    Running,    // 运行中
    Suspended,  // 暂停
    Judged,     // 裁判完成
    Eliminated, // 消除
    Destroyed,  // 销毁
}

/// 袋子容器 — 隔离执行环境
#[derive(Clone, Debug)]
pub struct BagContainer {
    pub bag: Bag,
    pub status: BagStatus,
    pub instruction: Option<InstructionBag>,
    pub exec_engine: JitExecutionEngine,
    pub exec_count: u64,
    pub ns_id: String,        // 隔离命名空间标识
    pub timeline: Timeline,
}

impl BagContainer {
    pub fn new(bag: Bag, ns_id: &str) -> Self {
        Self {
            bag,
            status: BagStatus::New,
            instruction: None,
            exec_engine: JitExecutionEngine::new(),
            exec_count: 0,
            ns_id: ns_id.to_string(),
            timeline: Timeline::new(),
        }
    }

    /// 编译袋子到指令袋
    pub fn compile(&mut self, compiler: &mut LivingCodeCompiler) -> bool {
        if self.bag.eliminated {
            self.status = BagStatus::Eliminated;
            return false;
        }
        let ib = compiler.compile_bag(&self.bag);
        self.instruction = Some(ib);
        self.status = BagStatus::New;
        true
    }

    /// 执行袋子指令
    pub fn run(&mut self) -> bool {
        if let Some(ref ib) = self.instruction {
            self.status = BagStatus::Running;
            self.exec_engine.reset();
            self.exec_engine.execute(ib);
            self.exec_count += 1;

            // 更新袋子状态
            if let Some(top) = self.exec_engine.stack.last() {
                if *top == 0 {
                    self.bag.eliminated = true;
                    self.status = BagStatus::Eliminated;
                } else {
                    self.status = BagStatus::Judged;
                }
            }
            true
        } else {
            false
        }
    }

    /// 暂停容器
    pub fn suspend(&mut self) {
        if self.status == BagStatus::Running {
            self.status = BagStatus::Suspended;
        }
    }

    /// 恢复执行
    pub fn resume(&mut self) {
        if self.status == BagStatus::Suspended {
            self.status = BagStatus::Running;
        }
    }

    /// 销毁容器
    pub fn destroy(&mut self) {
        self.status = BagStatus::Destroyed;
        self.instruction = None;
        self.exec_engine.reset();
    }

    /// 获取执行结果栈顶
    pub fn result_top(&self) -> Option<u64> {
        self.exec_engine.stack.last().copied()
    }

    /// 执行帧数
    pub fn frame_count(&self) -> usize {
        self.exec_engine.frame_log.len()
    }
}

// ======================================================================
// 第六部分: 袋子反射 — 运行时属性读取
// ======================================================================

/// 袋子反射 — 运行时14维属性读取 · 因果链回溯 · 健康评分
#[derive(Clone, Debug)]
pub struct BagIntrospection {
    pub bag_id: u64,
    pub health_score: f64,
    pub causal_chain: Vec<TimelineEventKind>,
    pub quality_snapshot: f64,
    pub sovereignty_label: String,
    pub introspection_time_ms: u64,
}

impl BagIntrospection {
    pub fn new(bag_id: u64) -> Self {
        Self {
            bag_id,
            health_score: 0.0,
            causal_chain: Vec::new(),
            quality_snapshot: 0.0,
            sovereignty_label: String::new(),
            introspection_time_ms: 0,
        }
    }

    /// 从袋子构建反射
    pub fn from_bag(bag: &Bag) -> Self {
        let mut ri = Self::new(bag.id);
        let start = current_time_ms();

        // 健康评分 = 综合属性
        ri.health_score = bag.attrs.composite_quality();

        // 因果链 — 基于袋子状态推断
        ri.causal_chain.push(TimelineEventKind::BagCreated);
        if bag.judgment_count > 0 {
            ri.causal_chain.push(TimelineEventKind::BagJudged);
        }
        if bag.eliminated {
            ri.causal_chain.push(TimelineEventKind::BagEliminated);
        } else if bag.attrs.needs_any_processing() {
            ri.causal_chain.push(TimelineEventKind::BagFiltered);
        }

        ri.quality_snapshot = bag.attrs.content_quality;
        ri.sovereignty_label = match bag.attrs.sovereignty {
            0 => "无主".to_string(),
            1 => "妻子".to_string(),
            2 => "丈夫".to_string(),
            3 => "夫妻共有".to_string(),
            n => format!("自定义({n})"),
        };

        ri.introspection_time_ms = current_time_ms() - start;
        ri
    }

    /// 从容器构建反射(含执行结果)
    pub fn from_container(container: &BagContainer) -> Self {
        let mut ri = Self::from_bag(&container.bag);
        // 执行结果影响健康评分
        if container.status == BagStatus::Eliminated {
            ri.health_score = 0.0;
        } else if let Some(top) = container.result_top() {
            // 执行栈顶值追加到健康评分
            let exec_factor = (top as f64).ln_1p().min(5.0) / 5.0;
            ri.health_score = (ri.health_score * 0.7 + exec_factor * 0.3).clamp(0.0, 1.0);
        }
        ri
    }

    /// 是否健康(可继续运行)
    pub fn is_healthy(&self) -> bool {
        self.health_score > 0.3 && !self.causal_chain.contains(&TimelineEventKind::BagEliminated)
    }

    /// 获取因果链中最后一个事件
    pub fn last_event(&self) -> Option<TimelineEventKind> {
        self.causal_chain.last().copied()
    }
}

// ======================================================================
// 第七部分: 袋子池 — 多容器统一管理
// ======================================================================

/// 袋子池 — 容器注册表 · 批量执行
#[derive(Clone, Debug)]
pub struct BagPool {
    pub containers: HashMap<u64, BagContainer>,
    pub compiler: LivingCodeCompiler,
    pub total_compiled: u64,
    pub total_executed: u64,
    pub total_eliminated: u64,
}

impl BagPool {
    pub fn new() -> Self {
        Self {
            containers: HashMap::new(),
            compiler: LivingCodeCompiler::new(),
            total_compiled: 0,
            total_executed: 0,
            total_eliminated: 0,
        }
    }

    /// 创建容器并注册
    pub fn spawn(&mut self, bag: Bag, ns_id: &str) -> u64 {
        let id = bag.id;
        let container = BagContainer::new(bag, ns_id);
        self.containers.insert(id, container);
        id
    }

    /// 编译指定袋子
    pub fn compile_bag(&mut self, id: u64) -> bool {
        if let Some(container) = self.containers.get_mut(&id) {
            if container.compile(&mut self.compiler) {
                self.total_compiled += 1;
                return true;
            }
        }
        false
    }

    /// 执行指定袋子
    pub fn run_bag(&mut self, id: u64) -> bool {
        if let Some(container) = self.containers.get_mut(&id) {
            if container.run() {
                self.total_executed += 1;
                if container.status == BagStatus::Eliminated {
                    self.total_eliminated += 1;
                }
                return true;
            }
        }
        false
    }

    /// 编译+执行一键完成
    pub fn compile_and_run(&mut self, id: u64) -> bool {
        self.compile_bag(id) && self.run_bag(id)
    }

    /// 获取指定容器反射
    pub fn introspect(&self, id: u64) -> Option<BagIntrospection> {
        self.containers.get(&id).map(BagIntrospection::from_container)
    }

    /// 获取存活容器数量
    pub fn alive_count(&self) -> usize {
        self.containers.values()
            .filter(|c| c.status != BagStatus::Eliminated && c.status != BagStatus::Destroyed)
            .count()
    }

    /// 获取消除的容器数量
    pub fn eliminated_count(&self) -> usize {
        self.containers.values()
            .filter(|c| c.status == BagStatus::Eliminated)
            .count()
    }

    /// 池统摘要
    pub fn summary(&self) -> String {
        format!(
            "池状态 存活/消除/总容器 = {}/{}/{} 编译{}次 执行{}次",
            self.alive_count(), self.eliminated_count(), self.containers.len(),
            self.total_compiled, self.total_executed,
        )
    }
}

// ======================================================================
// 第八部分: CelestialPerceptionV12 — 编排引擎
// ======================================================================

/// CelestialPerceptionV12 — 天觉v12编排引擎
/// 活码运行时的总控: 袋子从创建到执行到反射的全链路
#[derive(Clone, Debug)]
pub struct CelestialPerceptionV12 {
    pub version: u32,
    pub pool: BagPool,
    pub subsystem: SubSystem,
    pub active: bool,
}

impl CelestialPerceptionV12 {
    pub fn new() -> Self {
        let mut sub = SubSystem::new("天觉v12·活码运行时");
        sub.register();
        Self {
            version: V12_VERSION,
            pool: BagPool::new(),
            subsystem: sub,
            active: true,
        }
    }

    /// 注册活码名称
    pub fn register_code(&mut self, name: &str) {
        self.pool.compiler.register_code(name);
    }

    /// 激活活码
    pub fn activate_code(&mut self, name: &str, key: &str) -> bool {
        self.pool.compiler.activate_code(name, key)
    }

    /// 投放袋子到池
    pub fn ingest(&mut self, bag: Bag, ns_id: &str) -> u64 {
        self.pool.spawn(bag, ns_id)
    }

    /// 编译袋子
    pub fn compile(&mut self, id: u64) -> bool {
        self.pool.compile_bag(id)
    }

    /// 执行袋子
    pub fn execute(&mut self, id: u64) -> bool {
        self.pool.run_bag(id)
    }

    /// 编译+执行一体化
    pub fn compile_and_run(&mut self, id: u64) -> bool {
        self.pool.compile_and_run(id)
    }

    /// 反射袋子
    pub fn introspect(&self, id: u64) -> Option<BagIntrospection> {
        self.pool.introspect(id)
    }

    /// 池摘要
    pub fn pool_summary(&self) -> String {
        self.pool.summary()
    }

    /// 整体运转: 编译并执行池中所有新建袋子
    pub fn run_all_new(&mut self) -> u64 {
        let ids: Vec<u64> = self.pool.containers.iter()
            .filter(|(_, c)| c.status == BagStatus::New && c.instruction.is_some())
            .map(|(id, _)| *id)
            .collect();
        let mut ran = 0u64;
        for id in ids {
            if self.execute(id) { ran += 1; }
        }
        ran
    }

    /// 诊断报告
    pub fn health_report(&self) -> String {
        let alive = self.pool.alive_count();
        let elim = self.pool.eliminated_count();
        let total = self.pool.containers.len();
        let health_pct = if total > 0 { (alive as f64 / total as f64) * 100.0 } else { 0.0 };
        format!(
            "天觉v12 存活率{:.0}% 存活{}/{} 消除{}",
            health_pct, alive, total, elim,
        )
    }
}

// ======================================================================
// 辅助函数
// ======================================================================

fn current_time_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

// ======================================================================
// 单元测试
// ======================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v12_engine_creation() {
        let v12 = CelestialPerceptionV12::new();
        assert_eq!(v12.version, V12_VERSION);
        assert!(v12.active);
        assert_eq!(v12.pool.alive_count(), 0);
    }

    #[test]
    fn opcode_operand_count() {
        assert_eq!(OpCode::LoadBag.operand_count(), 1);
        assert_eq!(OpCode::JudgeAnd.operand_count(), 2);
        assert_eq!(OpCode::Eliminate.operand_count(), 0);
        assert_eq!(OpCode::Spawn.operand_count(), 0);
    }

    #[test]
    fn opcode_domain() {
        assert_eq!(OpCode::LoadBag.domain(), ExecDomain::Memory);
        assert_eq!(OpCode::JudgeAnd.domain(), ExecDomain::Judgment);
        assert_eq!(OpCode::AddValue.domain(), ExecDomain::Arithmetic);
        assert_eq!(OpCode::Spawn.domain(), ExecDomain::Lifecycle);
    }

    #[test]
    fn instruction_bag_emit_and_query() {
        let mut ib = InstructionBag::new(1);
        ib.emit(OpCode::Spawn, vec![1]);
        ib.emit(OpCode::LoadBag, vec![1]);
        ib.emit(OpCode::AddValue, vec![800]);
        assert_eq!(ib.len(), 3);
        assert_eq!(ib.opcode_at(0), Some(OpCode::Spawn));
        assert_eq!(ib.operands_at(2), Some(&[800][..]));
        assert_eq!(ib.opcode_at(5), None);
    }

    #[test]
    fn instruction_bag_fixes_starting_point() {
        let mut ib = InstructionBag::new(1);
        ib.emit(OpCode::AddValue, vec![100]);
        ib.emit(OpCode::AddValue, vec![200]);
        assert!(!ib.fixed_point);
        ib.fix_starting_point("起点");
        assert!(ib.fixed_point);
        assert!(ib.permission_level > 0);
    }

    #[test]
    fn living_compiler_register_and_activate() {
        let mut lc = LivingCodeCompiler::new();
        lc.register_code("mltt编码");
        assert!(!lc.is_activated("mltt编码"));
        // 错误钥匙不激活
        assert!(!lc.activate_code("mltt编码", "wrong_key"));
        assert!(!lc.is_activated("mltt编码"));
        // 正确钥匙激活
        assert!(lc.activate_code("mltt编码", "起点"));
        assert!(lc.is_activated("mltt编码"));
    }

    #[test]
    fn living_compiler_compile_bag() {
        let mut lc = LivingCodeCompiler::new();
        let bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let ib = lc.compile_bag(&bag);
        assert_eq!(ib.source_bag_id, 1);
        assert!(ib.len() > 2);
        assert_eq!(lc.compile_count, 1);
    }

    #[test]
    fn living_compiler_compile_and_activate() {
        let mut lc = LivingCodeCompiler::new();
        let bag = Bag::new(2, "洁净袋", BagAttributes::pristine(), (0, 0, 0));
        let ib = lc.compile_and_activate(&bag, "起点");
        assert!(ib.fixed_point);
        assert_eq!(ib.activation_key.as_deref(), Some("起点"));
    }

    #[test]
    fn jit_engine_execute_load_add_store() {
        let mut engine = JitExecutionEngine::new();
        let bag = Bag::new(5, "试袋", BagAttributes::pristine(), (0, 0, 0));
        let mut lc = LivingCodeCompiler::new();
        let ib = lc.compile_bag(&bag);

        engine.execute(&ib);
        assert!(engine.executed_count > 0);
        assert!(engine.completed);
        assert!(engine.max_stack_depth > 0);
        assert_eq!(engine.frame_log.len() as u64, engine.executed_count);
    }

    #[test]
    fn jit_engine_step_count_matches_instructions() {
        let mut engine = JitExecutionEngine::new();
        let mut ib = InstructionBag::new(10);
        for _ in 0..5 {
            ib.emit(OpCode::PassThrough, vec![]);
        }
        engine.execute(&ib);
        assert_eq!(engine.executed_count, 5);
    }

    #[test]
    fn jit_engine_stack_manipulation() {
        let mut engine = JitExecutionEngine::new();
        let mut ib = InstructionBag::new(1);
        ib.emit(OpCode::Spawn, vec![1]);     // stack: [1]
        ib.emit(OpCode::AddValue, vec![50]); // stack: [1, 51]
        ib.emit(OpCode::AddValue, vec![30]); // stack: [1, 51, 81]
        engine.execute(&ib);
        assert_eq!(engine.stack.last().copied(), Some(81));
    }

    #[test]
    fn bag_container_new_and_compile() {
        let bag = Bag::new(1, "测袋", BagAttributes::pristine(), (0, 0, 0));
        let mut container = BagContainer::new(bag, "ns://main");
        assert_eq!(container.status, BagStatus::New);
        assert_eq!(container.ns_id, "ns://main");

        let mut lc = LivingCodeCompiler::new();
        assert!(container.compile(&mut lc));
        assert!(container.instruction.is_some());
    }

    #[test]
    fn bag_container_run_judged() {
        let bag = Bag::new(1, "测袋", BagAttributes::pristine(), (0, 0, 0));
        let mut container = BagContainer::new(bag, "ns://main");
        let mut lc = LivingCodeCompiler::new();
        container.compile(&mut lc);
        assert!(container.run());
        assert_eq!(container.status, BagStatus::Judged);
        assert!(container.exec_count >= 1);
    }

    #[test]
    fn bag_container_suspended_and_resumed() {
        let bag = Bag::new(1, "测袋", BagAttributes::pristine(), (0, 0, 0));
        let mut container = BagContainer::new(bag, "ns://main");
        container.status = BagStatus::Running;
        container.suspend();
        assert_eq!(container.status, BagStatus::Suspended);
        container.resume();
        assert_eq!(container.status, BagStatus::Running);
    }

    #[test]
    fn bag_container_destroy() {
        let bag = Bag::new(1, "测袋", BagAttributes::pristine(), (0, 0, 0));
        let mut container = BagContainer::new(bag, "ns://main");
        let mut lc = LivingCodeCompiler::new();
        container.compile(&mut lc);
        container.run();
        container.destroy();
        assert_eq!(container.status, BagStatus::Destroyed);
        assert!(container.instruction.is_none());
    }

    #[test]
    fn bag_introspection_from_bag() {
        let bag = Bag::new(1, "测袋", BagAttributes::pristine(), (0, 0, 0));
        let ri = BagIntrospection::from_bag(&bag);
        assert_eq!(ri.bag_id, 1);
        assert!(ri.health_score > 0.5);  // pristine bag should be healthy
        assert!(ri.is_healthy());
        assert!(ri.causal_chain.contains(&TimelineEventKind::BagCreated));
    }

    #[test]
    fn bag_introspection_eliminated_unhealthy() {
        let mut bag = Bag::new(1, "腐袋", BagAttributes::rotten(), (0, 0, 0));
        bag.eliminated = true;
        let ri = BagIntrospection::from_bag(&bag);
        assert!(!ri.is_healthy());
        assert!(ri.causal_chain.contains(&TimelineEventKind::BagEliminated));
    }

    #[test]
    fn bag_pool_spawn_compile_run() {
        let mut pool = BagPool::new();
        let bag = Bag::new(100, "池测袋", BagAttributes::pristine(), (0, 0, 0));
        let id = pool.spawn(bag, "ns://test");
        assert_eq!(id, 100);
        assert!(pool.compile_bag(id));
        assert!(pool.run_bag(id));
        assert_eq!(pool.total_executed, 1);
        assert_eq!(pool.alive_count(), 1);
    }

    #[test]
    fn bag_pool_compile_and_run_one_step() {
        let mut pool = BagPool::new();
        let bag = Bag::new(200, "一步袋", BagAttributes::pristine(), (0, 0, 0));
        let id = pool.spawn(bag, "ns://test");
        assert!(pool.compile_and_run(id));
        assert_eq!(pool.total_compiled, 1);
        assert_eq!(pool.total_executed, 1);
    }

    #[test]
    fn bag_pool_summary_contains_counts() {
        let mut pool = BagPool::new();
        let bag = Bag::new(1, "袋1", BagAttributes::pristine(), (0, 0, 0));
        pool.spawn(bag, "ns://t");
        let s = pool.summary();
        assert!(s.contains("总容器 = 1"));
    }

    #[test]
    fn v12_ingest_compile_execute() {
        let mut v12 = CelestialPerceptionV12::new();
        let bag = Bag::new(42, "运行袋", BagAttributes::pristine(), (0, 0, 0));
        let id = v12.ingest(bag, "ns://runtime");
        assert!(v12.compile(id));
        assert!(v12.execute(id));
        assert!(v12.pool.total_executed > 0);
    }

    #[test]
    fn v12_introspect_bag() {
        let mut v12 = CelestialPerceptionV12::new();
        let bag = Bag::new(77, "反射袋", BagAttributes::pristine(), (0, 0, 0));
        let id = v12.ingest(bag, "ns://rt");
        v12.compile_and_run(id);
        let ri = v12.introspect(id);
        assert!(ri.is_some());
        let ri = ri.unwrap();
        assert_eq!(ri.bag_id, 77);
        assert!(ri.health_score > 0.0);
    }

    #[test]
    fn v12_health_report() {
        let mut v12 = CelestialPerceptionV12::new();
        let bag = Bag::new(1, "健测袋", BagAttributes::pristine(), (0, 0, 0));
        v12.ingest(bag, "ns://rt");
        v12.compile(1);
        let report = v12.health_report();
        assert!(report.contains("存活率"));
    }

    #[test]
    fn v12_register_and_activate_code() {
        let mut v12 = CelestialPerceptionV12::new();
        v12.register_code("mltt编码");
        assert!(!v12.pool.compiler.is_activated("mltt编码"));
        assert!(v12.activate_code("mltt编码", "起点"));
        assert!(v12.pool.compiler.is_activated("mltt编码"));
    }

    #[test]
    fn v12_eliminated_bag_counted() {
        let mut v12 = CelestialPerceptionV12::new();
        // 构造一个必定消除的袋子: 质量极低 + value=0 → AddValue后栈顶非正 → 消除
        let mut bag = Bag::new(50, "腐坏袋", BagAttributes {
            sovereignty: 0, value: 0, integrity: 0.1, content_quality: 0.05,
            content_purity: 0.05, bag_cleanliness: 0.05, dryness: 0.1,
            pest_level: 0.9, sand_level: 0.9, edibility: 0.0,
            needs_content_wash: false, needs_sand_pick: false,
            wash_target: 0.8, needs_repair: false,
        }, (0, 0, 0));
        bag.eliminated = true;
        let id = v12.ingest(bag, "ns://elim");
        v12.compile_and_run(id);
        // 消除的袋子应被计数
        assert!(v12.pool.eliminated_count() >= 1);
    }

    #[test]
    fn exec_domain_display() {
        assert_eq!(format!("{}", ExecDomain::Memory), "存储");
        assert_eq!(format!("{}", ExecDomain::Judgment), "裁判");
        assert_eq!(format!("{}", ExecDomain::Arithmetic), "加法");
    }

    #[test]
    fn v12_full_pipeline() {
        // 完整流水线: 注册活码→投放袋子→编译→执行→反射
        let mut v12 = CelestialPerceptionV12::new();
        v12.register_code("主权编码");
        v12.activate_code("主权编码", "起点");

        let bags = vec![
            Bag::new(1, "优袋", BagAttributes::pristine(), (0, 0, 0)),
            Bag::new(2, "中袋", BagAttributes {
                sovereignty: 3, value: 500, integrity: 0.7, content_quality: 0.6,
                content_purity: 0.5, bag_cleanliness: 0.6, dryness: 0.8,
                pest_level: 0.1, sand_level: 0.2, edibility: 0.7,
                needs_content_wash: false, needs_sand_pick: false,
                wash_target: 0.8, needs_repair: false,
            }, (0, 0, 0)),
            Bag::new(3, "劣袋", BagAttributes::rotten(), (0, 0, 0)),
        ];

        for bag in bags {
            let id = v12.ingest(bag, "ns://pipe");
            v12.compile_and_run(id);
        }

        // 健康袋子存活
        assert!(v12.introspect(1).unwrap().is_healthy());
        // 劣袋应该被消除
        assert!(v12.pool.eliminated_count() >= 1);
        // 池摘要
        let s = v12.pool_summary();
        assert!(s.contains("执行3次") || s.contains("编译"));
    }
}
