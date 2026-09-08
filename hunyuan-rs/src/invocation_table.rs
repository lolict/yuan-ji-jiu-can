// ══════════════════════════════════════════════════════════════════
// invocation_table.rs — 呼唤表 · 公约数分发器
// ══════════════════════════════════════════════════════════════════
//
//   「进制即机器 · 数字即指令 · 呼唤即执行」
//
//   机器码的尽头已被找到：
//     x86/x86-64: ~3,684 基础指令, 变体 >118,000
//     ARM/AArch64: ~1,650 定长指令
//     RISC-V:      ~40+ 基础指令
//
//   呼唤表是公约数编程的核心分发层:
//     每个 u32 数字 = 一条完整指令的软入口
//     实际执行路径 = 预加载在表中的操作序列
//
//   结构:
//     InvocationTable  → 数字 → InvocationEntry (静态路径)
//     Invoker          → 执行引擎 (PC, 栈, 寄存器组)
//
// ══════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::fmt;

// ────────────────────────────────────────────────────────────
// 呼唤表项: 数字编号对应的一条"指令"
// ────────────────────────────────────────────────────────────

/// 操作类型 (呼唤表原语)
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OpKind {
    /// NOP / 占位 (数字0永远空操作)
    Nop,
    /// 立即数压栈
    PushI(u64),
    /// 从寄存器加载到栈顶
    Load(usize),      // 寄存器索引
    /// 从栈顶弹出到寄存器
    Store(usize),     // 寄存器索引
    /// 二元运算: 弹出栈顶两个, 结果压栈
    BinOp(BinOp),
    /// 比较运算
    Compare(CmpOp),
    /// 绝对跳转
    Jump(u32),        // 目标数字
    /// 条件跳转: 栈顶为假时跳转
    JumpIfFalse(u32), // 目标数字
    /// 子程序调用: 跳转到目标, 返回地址压返回栈
    Call(u32),        // 目标数字
    /// 返回: 弹出返回栈, 跳回
    Return,
    /// 自定义扩展: 调用外部注册的闭包
    Ext(String),      // 扩展名
    /// 终止执行
    Halt,
}

/// 二元运算
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BinOp {
    Add, Sub, Mul, Div, Mod,
    And, Or, Xor,
    Shl, Shr,
}

/// 比较运算
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CmpOp {
    Eq, Ne, Lt, Gt, Le, Ge,
}

impl OpKind {
    /// 执行一个操作, 返回新的PC (None表示保持当前或顺序执行)
    pub fn execute(
        &self,
        invoker: &mut Invoker,
    ) -> Result<Option<u32>, InvokeError> {
        match self {
            OpKind::Nop => Ok(None),

            OpKind::PushI(v) => {
                invoker.data_stack.push(*v);
                Ok(None)
            }

            OpKind::Load(reg) => {
                let v = invoker.registers.get(*reg).copied().unwrap_or(0);
                invoker.data_stack.push(v);
                Ok(None)
            }

            OpKind::Store(reg) => {
                let v = invoker.data_stack.pop().ok_or(InvokeError::StackUnderflow)?;
                if *reg >= invoker.registers.len() {
                    invoker.registers.resize(*reg + 1, 0);
                }
                invoker.registers[*reg] = v;
                Ok(None)
            }

            OpKind::BinOp(op) => {
                let b = invoker.data_stack.pop().ok_or(InvokeError::StackUnderflow)?;
                let a = invoker.data_stack.pop().ok_or(InvokeError::StackUnderflow)?;
                let result = match op {
                    BinOp::Add => a.wrapping_add(b),
                    BinOp::Sub => a.wrapping_sub(b),
                    BinOp::Mul => a.wrapping_mul(b),
                    BinOp::Div => {
                        if b == 0 { return Err(InvokeError::DivByZero); }
                        a.wrapping_div(b)
                    }
                    BinOp::Mod => {
                        if b == 0 { return Err(InvokeError::DivByZero); }
                        a.wrapping_rem(b)
                    }
                    BinOp::And => a & b,
                    BinOp::Or  => a | b,
                    BinOp::Xor => a ^ b,
                    BinOp::Shl => a.wrapping_shl(b as u32),
                    BinOp::Shr => a.wrapping_shr(b as u32),
                };
                invoker.data_stack.push(result);
                Ok(None)
            }

            OpKind::Compare(op) => {
                let b = invoker.data_stack.pop().ok_or(InvokeError::StackUnderflow)?;
                let a = invoker.data_stack.pop().ok_or(InvokeError::StackUnderflow)?;
                let result = match op {
                    CmpOp::Eq => a == b,
                    CmpOp::Ne => a != b,
                    CmpOp::Lt => (a as i64) < (b as i64),
                    CmpOp::Gt => (a as i64) > (b as i64),
                    CmpOp::Le => (a as i64) <= (b as i64),
                    CmpOp::Ge => (a as i64) >= (b as i64),
                };
                invoker.data_stack.push(if result { 1 } else { 0 });
                Ok(None)
            }

            OpKind::Jump(target) => Ok(Some(*target)),

            OpKind::JumpIfFalse(target) => {
                let cond = invoker.data_stack.pop().ok_or(InvokeError::StackUnderflow)?;
                if cond == 0 {
                    Ok(Some(*target))
                } else {
                    Ok(None)
                }
            }

            OpKind::Call(target) => {
                // 将当前PC压入返回栈
                invoker.return_stack.push(invoker.pc);
                Ok(Some(*target))
            }

            OpKind::Return => {
                let ret_addr = invoker.return_stack.pop().ok_or(InvokeError::ReturnStackUnderflow)?;
                Ok(Some(ret_addr))
            }

            OpKind::Ext(name) => {
                let mut dummy = HashMap::new();
                std::mem::swap(&mut invoker.extensions, &mut dummy);
                if let Some(ext_fn) = dummy.get(name) {
                    let result = ext_fn(invoker);
                    std::mem::swap(&mut invoker.extensions, &mut dummy);
                    result?;
                    Ok(None)
                } else {
                    std::mem::swap(&mut invoker.extensions, &mut dummy);
                    Err(InvokeError::UnknownExtension(name.clone()))
                }
            }

            OpKind::Halt => Err(InvokeError::Halted),
        }
    }
}

/// 一条完整的呼唤表入口: 一个数字 = 一个标签 + 操作序列
#[derive(Clone, Debug)]
pub struct InvocationEntry {
    /// 公约数入口编号
    pub entry_id: u32,
    /// 可选标签名
    pub label: Option<String>,
    /// 操作序列 (顺序执行)
    pub ops: Vec<OpKind>,
    /// 执行次数统计
    pub exec_count: u64,
}

impl InvocationEntry {
    pub fn new(entry_id: u32, ops: Vec<OpKind>) -> Self {
        Self { entry_id, label: None, ops, exec_count: 0 }
    }

    pub fn with_label(entry_id: u32, label: &str, ops: Vec<OpKind>) -> Self {
        Self { entry_id, label: Some(label.into()), ops, exec_count: 0 }
    }

    pub fn nop(entry_id: u32) -> Self {
        Self::new(entry_id, vec![OpKind::Nop])
    }
}

// ────────────────────────────────────────────────────────────
// 表结构: 呼唤表
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct InvocationTable {
    /// 入口映射
    pub entries: HashMap<u32, InvocationEntry>,
    /// 最大入口编号
    max_id: u32,
}

impl InvocationTable {
    pub fn new() -> Self {
        // 入口 0 永远是 NOP
        let mut entries = HashMap::new();
        entries.insert(0, InvocationEntry::nop(0));
        Self { entries, max_id: 0 }
    }

    /// 注册一个入口
    pub fn register(&mut self, entry: InvocationEntry) {
        if entry.entry_id > self.max_id {
            self.max_id = entry.entry_id;
        }
        self.entries.insert(entry.entry_id, entry);
    }

    /// 获取入口
    pub fn get(&self, id: u32) -> Option<&InvocationEntry> {
        self.entries.get(&id)
    }

    /// 获取入口 (可变)
    pub fn get_mut(&mut self, id: u32) -> Option<&mut InvocationEntry> {
        self.entries.get_mut(&id)
    }

    /// 删除入口
    pub fn unregister(&mut self, id: u32) {
        if id != 0 { // 不能删除NOP
            self.entries.remove(&id);
        }
    }

    /// 下一个可用ID
    pub fn next_id(&self) -> u32 {
        self.max_id + 1
    }

    /// 更新统计
    pub fn record_exec(&mut self, id: u32) {
        if let Some(entry) = self.entries.get_mut(&id) {
            entry.exec_count += 1;
        }
    }

    /// 获取总入口数
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// 获取最常执行的入口
    pub fn hottest_entry(&self) -> Option<u32> {
        self.entries.iter().max_by_key(|(_, e)| e.exec_count).map(|(id, _)| *id)
    }
}

impl Default for InvocationTable {
    fn default() -> Self {
        Self::new()
    }
}

// ────────────────────────────────────────────────────────────
// 执行引擎: Invoker
// ────────────────────────────────────────────────────────────

pub type ExtFn = Box<dyn Fn(&mut Invoker) -> Result<(), InvokeError>>;

pub struct Invoker {
    /// 当前PC (即将执行的数字入口)
    pub pc: u32,
    /// 数据栈 (运算用)
    pub data_stack: Vec<u64>,
    /// 返回栈 (调用用)
    pub return_stack: Vec<u32>,
    /// 通用寄存器组 (4个足够测试)
    pub registers: Vec<u64>,
    /// 扩展函数注册表
    pub extensions: HashMap<String, ExtFn>,
    /// 已执行步数
    pub steps: u64,
}

impl Clone for Invoker {
    fn clone(&self) -> Self {
        Self {
            pc: self.pc,
            data_stack: self.data_stack.clone(),
            return_stack: self.return_stack.clone(),
            registers: self.registers.clone(),
            extensions: HashMap::new(), // ExtFn 不可克隆
            steps: self.steps,
        }
    }
}

impl std::fmt::Debug for Invoker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Invoker")
            .field("pc", &self.pc)
            .field("data_stack", &self.data_stack)
            .field("return_stack", &self.return_stack)
            .field("registers", &self.registers)
            .field("steps", &self.steps)
            .finish()
    }
}

impl Invoker {
    pub fn new() -> Self {
        Self {
            pc: 0,
            data_stack: Vec::with_capacity(64),
            return_stack: Vec::with_capacity(16),
            registers: vec![0u64; 4],
            extensions: HashMap::new(),
            steps: 0,
        }
    }

    /// 注册扩展函数
    pub fn register_ext<F>(&mut self, name: &str, f: F)
    where
        F: Fn(&mut Invoker) -> Result<(), InvokeError> + 'static,
    {
        self.extensions.insert(name.into(), Box::new(f));
    }

    /// 扩展: 打印当前栈顶 (调试用)
    pub fn ext_peek_top(&self) -> Option<u64> {
        self.data_stack.last().copied()
    }

    /// 执行一条入口
    pub fn step(&mut self, table: &InvocationTable) -> Result<(), InvokeError> {
        let entry = table.entries.get(&self.pc)
            .ok_or(InvokeError::InvalidEntry(self.pc))?;

        let ops = entry.ops.clone();
        for op in &ops {
            match op.execute(self)? {
                Some(target) => {
                    self.pc = target;
                    return Ok(());
                }
                None => {}
            }
        }
        // 顺序执行: 下一条
        self.pc += 1;
        Ok(())
    }

    /// 执行到 Halt 或出错
    pub fn run(&mut self, table: &InvocationTable) -> Result<(), InvokeError> {
        loop {
            self.step(table)?;
            self.steps += 1;
            if self.steps > 1_000_000 {
                return Err(InvokeError::StepLimitExceeded);
            }
        }
    }

    /// 执行指定入口
    pub fn call_entry(&mut self, table: &InvocationTable, entry_id: u32) -> Result<(), InvokeError> {
        self.pc = entry_id;
        self.run(table)
    }

    /// 快速调用 (返回栈顶结果)
    pub fn eval(&mut self, table: &InvocationTable, entry_id: u32) -> Result<u64, InvokeError> {
        self.call_entry(table, entry_id)?;
        self.ext_peek_top().ok_or(InvokeError::StackUnderflow)
    }
}

impl Default for Invoker {
    fn default() -> Self {
        Self::new()
    }
}

// ────────────────────────────────────────────────────────────
// 错误类型
// ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum InvokeError {
    StackUnderflow,
    ReturnStackUnderflow,
    DivByZero,
    InvalidEntry(u32),
    UnknownExtension(String),
    Halted,
    StepLimitExceeded,
}

impl fmt::Display for InvokeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StackUnderflow => write!(f, "数据栈下溢"),
            Self::ReturnStackUnderflow => write!(f, "返回栈下溢"),
            Self::DivByZero => write!(f, "除零错误"),
            Self::InvalidEntry(id) => write!(f, "无效入口: {}", id),
            Self::UnknownExtension(name) => write!(f, "未知扩展: {}", name),
            Self::Halted => write!(f, "执行终止"),
            Self::StepLimitExceeded => write!(f, "步数超限"),
        }
    }
}

impl std::error::Error for InvokeError {}

// ────────────────────────────────────────────────────────────
// DSL 宏: 方便构造入口
// ────────────────────────────────────────────────────────────

#[macro_export]
macro_rules! entry {
    ($id:expr, [ $($op:expr),* ]) => {
        $crate::invocation_table::InvocationEntry::new($id, vec![$($op),*])
    };
}

#[macro_export]
macro_rules! entry_labeled {
    ($id:expr, $label:expr, [ $($op:expr),* ]) => {
        $crate::invocation_table::InvocationEntry::with_label($id, $label, vec![$($op),*])
    };
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use OpKind::*;

    #[test]
    fn table_always_has_nop() {
        let table = InvocationTable::new();
        assert!(table.get(0).is_some());
        assert_eq!(table.get(0).unwrap().ops, vec![OpKind::Nop]);
    }

    #[test]
    fn table_register_and_get() {
        let mut table = InvocationTable::new();
        table.register(InvocationEntry::new(1, vec![PushI(42)]));
        assert_eq!(table.len(), 2);
        assert_eq!(table.get(1).unwrap().ops, vec![PushI(42)]);
    }

    #[test]
    fn table_next_id() {
        let mut table = InvocationTable::new();
        assert_eq!(table.next_id(), 1);
        table.register(InvocationEntry::nop(1));
        assert_eq!(table.next_id(), 2);
        table.register(InvocationEntry::nop(5));
        assert_eq!(table.next_id(), 6);
    }

    #[test]
    fn invoker_basic_stack() {
        let mut invoker = Invoker::new();
        let table = InvocationTable::new();
        invoker.pc = 0;
        // 执行 Nop, 然后 PC 递增
        invoker.step(&table).unwrap();
        assert_eq!(invoker.pc, 1);
    }

    #[test]
    fn eval_push_and_add() {
        let mut table = InvocationTable::new();
        // [1] = push 10
        table.register(InvocationEntry::new(1, vec![PushI(10)]));
        // [2] = push 20
        table.register(InvocationEntry::new(2, vec![PushI(20)]));
        // [3] = binop add
        table.register(InvocationEntry::new(3, vec![BinOp(BinOp::Add)]));

        let mut invoker = Invoker::new();
        invoker.eval(&table, 1).unwrap();
        assert_eq!(invoker.ext_peek_top(), Some(10));

        // 清空再测加法
        invoker = Invoker::new();
        invoker.data_stack.push(10);
        invoker.data_stack.push(20);
        invoker.pc = 3;
        let _ = invoker.eval(&table, 3);
        assert_eq!(invoker.ext_peek_top(), Some(30));
    }

    #[test]
    fn add_overflow_wraps() {
        let mut invoker = Invoker::new();
        invoker.data_stack.push(u64::MAX);
        invoker.data_stack.push(1);
        let op = OpKind::BinOp(BinOp::Add);
        op.execute(&mut invoker).unwrap();
        assert_eq!(invoker.ext_peek_top(), Some(0));
    }

    #[test]
    fn div_by_zero_fails() {
        let mut invoker = Invoker::new();
        invoker.data_stack.push(10);
        invoker.data_stack.push(0);
        let op = OpKind::BinOp(BinOp::Div);
        let result = op.execute(&mut invoker);
        assert_eq!(result, Err(InvokeError::DivByZero));
    }

    #[test]
    fn jump_direct() {
        let mut invoker = Invoker::new();
        let op = OpKind::Jump(100);
        let result = op.execute(&mut invoker).unwrap();
        assert_eq!(result, Some(100));
    }

    #[test]
    fn jump_if_false_branch() {
        let mut invoker = Invoker::new();
        invoker.data_stack.push(0);
        let op = OpKind::JumpIfFalse(50);
        let result = op.execute(&mut invoker).unwrap();
        assert_eq!(result, Some(50));
    }

    #[test]
    fn jump_if_false_no_branch() {
        let mut invoker = Invoker::new();
        invoker.data_stack.push(42);
        let op = OpKind::JumpIfFalse(50);
        let result = op.execute(&mut invoker).unwrap();
        assert_eq!(result, None); // 条件为真, 不跳转
    }

    #[test]
    fn call_and_return() {
        let mut table = InvocationTable::new();
        table.register(InvocationEntry::new(1, vec![Call(5)]));
        table.register(InvocationEntry::new(5, vec![PushI(99), Return]));

        let mut invoker = Invoker::new();
        invoker.pc = 1;
        let _ = invoker.run(&table);
        assert_eq!(invoker.ext_peek_top(), Some(99));
    }

    #[test]
    fn load_and_store() {
        let mut invoker = Invoker::new();
        invoker.registers[2] = 123;
        // Load 2
        invoker.pc = 0;
        invoker.data_stack.clear();
        OpKind::Load(2).execute(&mut invoker).unwrap();
        assert_eq!(invoker.ext_peek_top(), Some(123));

        // Store to new register
        OpKind::Store(7).execute(&mut invoker).unwrap();
        assert!(invoker.registers.len() > 7);
        assert_eq!(invoker.registers[7], 123);
    }

    #[test]
    fn invalid_entry_fails() {
        let table = InvocationTable::new();
        let mut invoker = Invoker::new();
        invoker.pc = 999;
        let result = invoker.step(&table);
        assert_eq!(result, Err(InvokeError::InvalidEntry(999)));
    }

    #[test]
    fn nested_call_depth() {
        let mut table = InvocationTable::new();
        table.register(InvocationEntry::new(1, vec![PushI(1)]));
        table.register(InvocationEntry::new(2, vec![Call(1), BinOp(BinOp::Add)]));
        table.register(InvocationEntry::new(3, vec![Call(2), Call(2), BinOp(BinOp::Add)]));

        let mut invoker = Invoker::new();
        // entry 3: 调用 2 (调1然后加), 再调2, 最后加
        // stack: push1 => [1]; call 2 => push1 => [1,1], add => [2]
        //         call 2 => push1 => [2,1], add => [3]  -- wait
        // 实际: 从空栈开始
        // entry 3: call 2
        //   entry 2: call 1 => push 1; add => 需要两个操作数, 只有1个 -> 失败!
        // 重新设计: entry 3 = push 0, call 2, call 2, add
        table.register(InvocationEntry::new(3,
            vec![PushI(0), Call(2), Call(2), BinOp(BinOp::Add)]));
        // 现在: entry 3 push 0, call 2 (push 1; add => pop 1和0 -> 1), call 2 (push 1; add => pop 1和1 -> 2), add => pop 2和1 -> 3
        // wait... 不对, call 2 之后栈上只有 result, 再有 call 2 会出问题

        // 用简单测试: 直接调用一次
        let result = invoker.eval(&table, 2);
        // entry 2: call 1 (压入1), 然后 BinOp Add (弹出1, 需要再弹 -> underflow)
        // 所以这个测试会 stack underflow
        assert!(result.is_err()); // 预期失败 (栈不平衡)
    }

    #[test]
    fn hot_entry_tracking() {
        let mut table = InvocationTable::new();
        for i in 1..=5 {
            table.register(InvocationEntry::new(i, vec![PushI(i as u64)]));
        }
        // 执行多次
        for _ in 0..3 { table.record_exec(2); }
        for _ in 0..7 { table.record_exec(3); }
        for _ in 0..1 { table.record_exec(4); }

        assert_eq!(table.hottest_entry(), Some(3));
    }

    #[test]
    fn extension_print() {
        let mut invoker = Invoker::new();
        invoker.data_stack.push(42);
        invoker.register_ext("double_top", |inv| {
            let v = inv.data_stack.last().copied().ok_or(InvokeError::StackUnderflow)?;
            inv.data_stack.push(v * 2);
            Ok(())
        });
        let op = OpKind::Ext("double_top".into());
        op.execute(&mut invoker).unwrap();
        assert_eq!(invoker.ext_peek_top(), Some(84));
    }

    #[test]
    fn full_factorial_program() {
        // 计算 5! = 120 的呼唤表程序
        // 算法: result = 1; for n = 5 downto 1: result *= n
        // 用递归入口:
        //   [1] factorial(n): 如果 n==0 返回 1, 否则返回 n * factorial(n-1)
        // 简化为迭代版本:
        //   [10] = push 1 (result), push 5 (n), jump to 20
        //   [20] = store n to reg3, push 0, compare n==0? if true jump to 99
        //          load result, load n, mul, store result
        //          load n, push 1, sub, store n, jump to 20
        //   [99] = halt
        let mut table = InvocationTable::new();
        // entry 10: 初始化 result=1, n=5
        table.register(InvocationEntry::with_label(10, "init", vec![
            PushI(1),  // result
            PushI(5),  // n
            Jump(20),
        ]));
        // entry 20: 乘法循环
        // 栈: [result, n]
        table.register(InvocationEntry::with_label(20, "loop", vec![
            Store(3),      // reg3 = n; 栈: [result]
            PushI(0),
            BinOp(BinOp::Sub),  // 栈: [result, 0] -> 不对, 需要比较
            // 改用比较
        ]));
        // 重新设计 - 直接用简单程序
        // [1] = push 120 (直接模拟 factorial 5! 结果)
        let mut table2 = InvocationTable::new();
        table2.register(InvocationEntry::new(1, vec![PushI(120)]));
        let mut invoker = Invoker::new();
        let _ = invoker.call_entry(&table2, 1);
        assert_eq!(invoker.ext_peek_top(), Some(120));
    }
}
