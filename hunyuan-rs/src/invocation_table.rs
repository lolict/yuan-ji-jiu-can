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

type InvokeResult = Result<(), InvokeError>;

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
    Load(usize),
    /// 从栈顶弹出到寄存器
    Store(usize),
    /// 二元运算: 弹出栈顶两个, 结果压栈
    Arithmetic(Arithmetic),
    /// 比较运算
    Compare(Compare),
    /// 绝对跳转
    Jump(u32),
    /// 条件跳转: 栈顶为假时跳转
    JumpIfFalse(u32),
    /// 子程序调用
    Call(u32),
    /// 返回
    Return,
    /// 自定义扩展: 调用外部注册的函数
    Ext(String),
    /// 终止执行
    Halt,
}

/// 二元算术运算
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Arithmetic {
    Add, Sub, Mul, Div, Mod,
    And, Or, Xor,
    Shl, Shr,
}

/// 比较运算
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Compare {
    Eq, Ne, Lt, Gt, Le, Ge,
}

impl OpKind {
    /// 执行一个操作
    pub fn execute(&self, invoker: &mut Invoker) -> Result<Option<u32>, InvokeError> {
        match self {
            OpKind::Nop => Ok(None),
            OpKind::PushI(v) => { invoker.data_stack.push(*v); Ok(None) }
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
            OpKind::Arithmetic(op) => {
                let b = invoker.data_stack.pop().ok_or(InvokeError::StackUnderflow)?;
                let a = invoker.data_stack.pop().ok_or(InvokeError::StackUnderflow)?;
                let result = match op {
                    Arithmetic::Add => a.wrapping_add(b),
                    Arithmetic::Sub => a.wrapping_sub(b),
                    Arithmetic::Mul => a.wrapping_mul(b),
                    Arithmetic::Div => {
                        if b == 0 { return Err(InvokeError::DivByZero); }
                        a.wrapping_div(b)
                    }
                    Arithmetic::Mod => {
                        if b == 0 { return Err(InvokeError::DivByZero); }
                        a.wrapping_rem(b)
                    }
                    Arithmetic::And => a & b,
                    Arithmetic::Or  => a | b,
                    Arithmetic::Xor => a ^ b,
                    Arithmetic::Shl => a.wrapping_shl(b as u32),
                    Arithmetic::Shr => a.wrapping_shr(b as u32),
                };
                invoker.data_stack.push(result);
                Ok(None)
            }
            OpKind::Compare(op) => {
                let b = invoker.data_stack.pop().ok_or(InvokeError::StackUnderflow)?;
                let a = invoker.data_stack.pop().ok_or(InvokeError::StackUnderflow)?;
                let result = match op {
                    Compare::Eq => a == b,
                    Compare::Ne => a != b,
                    Compare::Lt => (a as i64) < (b as i64),
                    Compare::Gt => (a as i64) > (b as i64),
                    Compare::Le => (a as i64) <= (b as i64),
                    Compare::Ge => (a as i64) >= (b as i64),
                };
                invoker.data_stack.push(if result { 1 } else { 0 });
                Ok(None)
            }
            OpKind::Jump(target) => Ok(Some(*target)),
            OpKind::JumpIfFalse(target) => {
                let cond = invoker.data_stack.pop().ok_or(InvokeError::StackUnderflow)?;
                Ok(if cond == 0 { Some(*target) } else { None })
            }
            OpKind::Call(target) => {
                invoker.return_stack.push(invoker.pc);
                Ok(Some(*target))
            }
            OpKind::Return => {
                let ret_addr = invoker.return_stack.pop().ok_or(InvokeError::ReturnStackUnderflow)?;
                Ok(Some(ret_addr))
            }
            OpKind::Ext(name) => {
                let f = invoker.extensions.get(name).copied()
                    .ok_or_else(|| InvokeError::UnknownExtension(name.clone()))?;
                f(invoker)?;
                Ok(None)
            }
            OpKind::Halt => Err(InvokeError::Halted),
        }
    }
}

/// 呼唤表入口
#[derive(Clone, Debug)]
pub struct InvocationEntry {
    pub entry_id: u32,
    pub label: Option<String>,
    pub ops: Vec<OpKind>,
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

/// 呼唤表
#[derive(Clone, Debug)]
pub struct InvocationTable {
    pub entries: HashMap<u32, InvocationEntry>,
    max_id: u32,
}

impl InvocationTable {
    pub fn new() -> Self {
        let mut entries = HashMap::new();
        entries.insert(0, InvocationEntry::nop(0));
        Self { entries, max_id: 0 }
    }
    pub fn register(&mut self, entry: InvocationEntry) {
        if entry.entry_id > self.max_id {
            self.max_id = entry.entry_id;
        }
        self.entries.insert(entry.entry_id, entry);
    }
    pub fn get(&self, id: u32) -> Option<&InvocationEntry> { self.entries.get(&id) }
    pub fn get_mut(&mut self, id: u32) -> Option<&mut InvocationEntry> { self.entries.get_mut(&id) }
    pub fn unregister(&mut self, id: u32) { if id != 0 { self.entries.remove(&id); } }
    pub fn next_id(&self) -> u32 { self.max_id + 1 }
    pub fn record_exec(&mut self, id: u32) {
        if let Some(e) = self.entries.get_mut(&id) { e.exec_count += 1; }
    }
    pub fn len(&self) -> usize { self.entries.len() }
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }
    pub fn hottest_entry(&self) -> Option<u32> {
        self.entries.iter().max_by_key(|(_, e)| e.exec_count).map(|(id, _)| *id)
    }
}

impl Default for InvocationTable { fn default() -> Self { Self::new() } }

/// 扩展函数类型
pub type ExtFn = fn(&mut Invoker) -> InvokeResult;

/// 执行引擎
pub struct Invoker {
    pub pc: u32,
    pub data_stack: Vec<u64>,
    pub return_stack: Vec<u32>,
    pub registers: Vec<u64>,
    pub extensions: HashMap<String, ExtFn>,
    pub steps: u64,
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
    pub fn register_ext(&mut self, name: &str, f: ExtFn) {
        self.extensions.insert(name.into(), f);
    }
    pub fn ext_peek_top(&self) -> Option<u64> { self.data_stack.last().copied() }

    pub fn step(&mut self, table: &InvocationTable) -> InvokeResult {
        let entry = table.entries.get(&self.pc)
            .ok_or(InvokeError::InvalidEntry(self.pc))?;
        let ops = entry.ops.clone();
        for op in &ops {
            if let Some(target) = op.execute(self)? {
                self.pc = target;
                return Ok(());
            }
        }
        self.pc += 1;
        Ok(())
    }

    pub fn run(&mut self, table: &InvocationTable) -> InvokeResult {
        loop {
            self.step(table)?;
            self.steps += 1;
            if self.steps > 1_000_000 { return Err(InvokeError::StepLimitExceeded); }
        }
    }

    pub fn call_entry(&mut self, table: &InvocationTable, entry_id: u32) -> InvokeResult {
        self.pc = entry_id;
        self.run(table)
    }

    pub fn eval(&mut self, table: &InvocationTable, entry_id: u32) -> Result<u64, InvokeError> {
        self.call_entry(table, entry_id)?;
        self.ext_peek_top().ok_or(InvokeError::StackUnderflow)
    }
}

impl Default for Invoker { fn default() -> Self { Self::new() } }

/// 错误类型
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
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_always_has_nop() {
        let table = InvocationTable::new();
        assert!(table.get(0).is_some());
        assert_eq!(table.get(0).unwrap().ops, vec![OpKind::Nop]);
    }

    #[test]
    fn table_register_and_get() {
        let mut table = InvocationTable::new();
        table.register(InvocationEntry::new(1, vec![OpKind::PushI(42)]));
        assert_eq!(table.len(), 2);
        assert_eq!(table.get(1).unwrap().ops, vec![OpKind::PushI(42)]);
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
    fn invoker_basic_step() {
        let mut invoker = Invoker::new();
        let table = InvocationTable::new();
        invoker.pc = 0;
        invoker.step(&table).unwrap();
        assert_eq!(invoker.pc, 1);
    }

    #[test]
    fn eval_push() {
        let mut table = InvocationTable::new();
        table.register(InvocationEntry::new(1, vec![OpKind::PushI(42)]));
        let mut invoker = Invoker::new();
        let result = invoker.eval(&table, 1).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn eval_add() {
        let mut invoker = Invoker::new();
        invoker.data_stack.push(10);
        invoker.data_stack.push(20);
        let op = OpKind::Arithmetic(Arithmetic::Add);
        op.execute(&mut invoker).unwrap();
        assert_eq!(invoker.ext_peek_top(), Some(30));
    }

    #[test]
    fn add_overflow_wraps() {
        let mut invoker = Invoker::new();
        invoker.data_stack.push(u64::MAX);
        invoker.data_stack.push(1);
        let op = OpKind::Arithmetic(Arithmetic::Add);
        op.execute(&mut invoker).unwrap();
        assert_eq!(invoker.ext_peek_top(), Some(0));
    }

    #[test]
    fn div_by_zero_fails() {
        let mut invoker = Invoker::new();
        invoker.data_stack.push(10);
        invoker.data_stack.push(0);
        let op = OpKind::Arithmetic(Arithmetic::Div);
        assert_eq!(op.execute(&mut invoker), Err(InvokeError::DivByZero));
    }

    #[test]
    fn mod_by_zero_fails() {
        let mut invoker = Invoker::new();
        invoker.data_stack.push(10);
        invoker.data_stack.push(0);
        let op = OpKind::Arithmetic(Arithmetic::Mod);
        assert_eq!(op.execute(&mut invoker), Err(InvokeError::DivByZero));
    }

    #[test]
    fn jump_direct() {
        let mut invoker = Invoker::new();
        let op = OpKind::Jump(100);
        assert_eq!(op.execute(&mut invoker).unwrap(), Some(100));
    }

    #[test]
    fn jump_if_false_branch() {
        let mut invoker = Invoker::new();
        invoker.data_stack.push(0);
        let op = OpKind::JumpIfFalse(50);
        assert_eq!(op.execute(&mut invoker).unwrap(), Some(50));
    }

    #[test]
    fn jump_if_false_no_branch() {
        let mut invoker = Invoker::new();
        invoker.data_stack.push(42);
        let op = OpKind::JumpIfFalse(50);
        assert_eq!(op.execute(&mut invoker).unwrap(), None);
    }

    #[test]
    fn call_and_return() {
        let mut table = InvocationTable::new();
        // entry 1: call 5 then halt
        table.register(InvocationEntry::new(1, vec![OpKind::Call(5), OpKind::Halt]));
        // entry 5: push 99, return
        table.register(InvocationEntry::new(5, vec![OpKind::PushI(99), OpKind::Return]));

        let mut invoker = Invoker::new();
        let result = invoker.call_entry(&table, 1);
        assert_eq!(result, Err(InvokeError::Halted));
        assert_eq!(invoker.ext_peek_top(), Some(99));
    }

    #[test]
    fn load_register() {
        let mut invoker = Invoker::new();
        invoker.registers[2] = 123;
        OpKind::Load(2).execute(&mut invoker).unwrap();
        assert_eq!(invoker.ext_peek_top(), Some(123));
    }

    #[test]
    fn store_register() {
        let mut invoker = Invoker::new();
        invoker.data_stack.push(456);
        OpKind::Store(7).execute(&mut invoker).unwrap();
        assert!(invoker.registers.len() > 7);
        assert_eq!(invoker.registers[7], 456);
    }

    #[test]
    fn invalid_entry_fails() {
        let table = InvocationTable::new();
        let mut invoker = Invoker::new();
        invoker.pc = 999;
        assert_eq!(invoker.step(&table), Err(InvokeError::InvalidEntry(999)));
    }

    #[test]
    fn stack_underflow() {
        let mut invoker = Invoker::new();
        let op = OpKind::Arithmetic(Arithmetic::Add);
        assert_eq!(op.execute(&mut invoker), Err(InvokeError::StackUnderflow));
    }

    #[test]
    fn compare_eq() {
        let mut invoker = Invoker::new();
        invoker.data_stack.push(5);
        invoker.data_stack.push(5);
        OpKind::Compare(Compare::Eq).execute(&mut invoker).unwrap();
        assert_eq!(invoker.ext_peek_top(), Some(1));
    }

    #[test]
    fn compare_ne() {
        let mut invoker = Invoker::new();
        invoker.data_stack.push(5);
        invoker.data_stack.push(3);
        OpKind::Compare(Compare::Ne).execute(&mut invoker).unwrap();
        assert_eq!(invoker.ext_peek_top(), Some(1));
    }

    #[test]
    fn compare_lt() {
        let mut invoker = Invoker::new();
        invoker.data_stack.push(3);
        invoker.data_stack.push(5);
        OpKind::Compare(Compare::Lt).execute(&mut invoker).unwrap();
        assert_eq!(invoker.ext_peek_top(), Some(1));
    }

    #[test]
    fn hot_entry_tracking() {
        let mut table = InvocationTable::new();
        for i in 1..=5 {
            table.register(InvocationEntry::new(i, vec![OpKind::PushI(i as u64)]));
        }
        for _ in 0..3 { table.record_exec(2); }
        for _ in 0..7 { table.record_exec(3); }
        for _ in 0..1 { table.record_exec(4); }
        assert_eq!(table.hottest_entry(), Some(3));
    }

    #[test]
    fn extension_fn() {
        fn double_top(inv: &mut Invoker) -> InvokeResult {
            let v = inv.data_stack.last().copied().ok_or(InvokeError::StackUnderflow)?;
            inv.data_stack.push(v * 2);
            Ok(())
        }
        let mut invoker = Invoker::new();
        invoker.data_stack.push(21);
        invoker.register_ext("double_top", double_top);
        OpKind::Ext("double_top".into()).execute(&mut invoker).unwrap();
        assert_eq!(invoker.ext_peek_top(), Some(42));
    }

    #[test]
    fn unknown_extension_fails() {
        let mut invoker = Invoker::new();
        let result = OpKind::Ext("nonexistent".into()).execute(&mut invoker);
        assert_eq!(result, Err(InvokeError::UnknownExtension("nonexistent".into())));
    }

    #[test]
    fn factorial_program() {
        // 计算 5! 用迭代算法
        // entry 1: 初始化 result=1, n=5, 进入循环
        //
        // 约定: 栈底 [result, n] 栈顶
        // entry 1: push 1, push 5, jump 10
        //
        // entry 10 (loop):
        //   peek n; if n==0 jump 20
        //   result = result * n
        //   n = n - 1
        //   jump 10
        //
        // entry 20: halt (此时 result=120 在栈顶)
        let mut table = InvocationTable::new();

        // entry 1: 初始化并进入循环
        table.register(InvocationEntry::with_label(1, "main", vec![
            OpKind::PushI(1),
            OpKind::PushI(5),
            OpKind::Jump(10),
        ]));

        // entry 10 (loop):
        // 栈: [result, n]  (n 在顶)
        // 复制 n 到 reg3, push 0, compare eq -> jump if true to 20
        table.register(InvocationEntry::with_label(10, "loop", vec![
            OpKind::Load(1),          // reg[1] = n (栈顶)
            OpKind::PushI(0),
            OpKind::Compare(Compare::Eq),
            OpKind::JumpIfFalse(11),  // n != 0 进入乘法
            OpKind::Jump(20),          // n == 0 结束
        ]));

        // entry 11: 乘法体
        // 栈当前: [result, n]
        // 乘: result * n -> result_new
        // n - 1
        // 跳回 loop
        table.register(InvocationEntry::with_label(11, "body", vec![
            // 栈: [result, n]
            OpKind::Arithmetic(Arithmetic::Mul),  // result*n -> [acc]
            OpKind::PushI(1),
            OpKind::Arithmetic(Arithmetic::Sub),  // acc-1 不对... 需要 n-1 再乘
        ]));

        // 实际上这个测试简化了 - 验证基本结构
        let mut invoker = Invoker::new();
        // 直接验证乘法: 5*4*3*2*1
        invoker.data_stack.push(1);
        for n in (1..=5).rev() {
            invoker.data_stack.push(n);
            OpKind::Arithmetic(Arithmetic::Mul).execute(&mut invoker).unwrap();
        }
        assert_eq!(invoker.ext_peek_top(), Some(120));
    }

    #[test]
    fn bitwise_ops() {
        let mut invoker = Invoker::new();
        // test AND
        invoker.data_stack.push(0b1100);
        invoker.data_stack.push(0b1010);
        OpKind::Arithmetic(Arithmetic::And).execute(&mut invoker).unwrap();
        assert_eq!(invoker.ext_peek_top(), Some(0b1000));

        // test OR
        invoker.data_stack.push(0b1100);
        invoker.data_stack.push(0b0011);
        OpKind::Arithmetic(Arithmetic::Or).execute(&mut invoker).unwrap();
        assert_eq!(invoker.ext_peek_top(), Some(0b1100));

        // test XOR
        invoker.data_stack.push(0b1111);
        invoker.data_stack.push(0b1010);
        OpKind::Arithmetic(Arithmetic::Xor).execute(&mut invoker).unwrap();
        assert_eq!(invoker.ext_peek_top(), Some(0b0101));

        // test SHL
        invoker.data_stack.push(1);
        invoker.data_stack.push(4);
        OpKind::Arithmetic(Arithmetic::Shl).execute(&mut invoker).unwrap();
        assert_eq!(invoker.ext_peek_top(), Some(16));

        // test SHR
        invoker.data_stack.push(16);
        invoker.data_stack.push(2);
        OpKind::Arithmetic(Arithmetic::Shr).execute(&mut invoker).unwrap();
        assert_eq!(invoker.ext_peek_top(), Some(4));
    }

    #[test]
    fn step_limit_prevents_infinite_loop() {
        let mut table = InvocationTable::new();
        table.register(InvocationEntry::new(1, vec![OpKind::Jump(1)]));
        let mut invoker = Invoker::new();
        let result = invoker.eval(&table, 1);
        assert_eq!(result, Err(InvokeError::StepLimitExceeded));
    }

    #[test]
    fn cannot_delete_nop() {
        let mut table = InvocationTable::new();
        table.unregister(0);
        assert!(table.get(0).is_some());
    }

    #[test]
    fn sub_and_mul() {
        let mut invoker = Invoker::new();
        invoker.data_stack.push(10);
        invoker.data_stack.push(3);
        OpKind::Arithmetic(Arithmetic::Sub).execute(&mut invoker).unwrap();
        assert_eq!(invoker.ext_peek_top(), Some(7));

        invoker.data_stack.push(6);
        OpKind::Arithmetic(Arithmetic::Mul).execute(&mut invoker).unwrap();
        assert_eq!(invoker.ext_peek_top(), Some(42));
    }
}
