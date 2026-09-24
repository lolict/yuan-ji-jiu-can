# 呼唤表规范 — 囡囝共生 · 公约数分发器 · 机器码尽头

## 1. 定位

"机器码的尽头已被找到" — 呼唤表是把每条指令分配到一个 u32 编号的运行时分发层。不是编译器 — 是**指令序列的容器**: 每条"指令" = 一个 u32 入口编号 + 一个 OpKind 操作列表。

呼唤表是公约数编程的核心分发层: 每个 u32 数字 = 一条完整指令的软入口, 实际执行路径 = 预加载在表中的操作序列。它替代了传统操作系统的 syscall 表, 但使用**数字即指令**的更原始范式。

## 2. 操作类型 (OpKind)

```rust
pub enum OpKind {
    Nop,                    // #0 永远空操作
    PushI(u64),             // 立即数压栈
    Load(usize),            // 寄存器 → 栈
    Store(usize),           // 栈 → 寄存器
    Arithmetic(Arithmetic), // 二元运算
    Compare(Compare),       // 比较
    Jump(u32),              // 绝对跳转
    JumpIfFalse(u32),       // 条件跳转(栈顶为假)
    Call(u32),              // 子程序调用
    Return,                 // 返回
    Ext(String),            // 自定义扩展函数
    Halt,                   // 终止
}
```

12 种操作 = 完整的控制流 + 数据流 + 扩展点。`Ext(String)` 是扩展口 — 运行时注册外部函数, 字符串为键。

## 3. 呼唤表 (InvocationTable)

```rust
pub struct InvocationTable {
    entries: HashMap<u32, InvocationEntry>,
    max_id: u32,
}
```

`InvocationEntry` = `{ entry_id, label, ops, exec_count }`。入口 0 预留为 NOP。`hottest_entry()` 返回 exec_count 最高的入口编号 — 用于统计最常执行的"指令"。

## 4. 执行引擎 (Invoker)

```rust
pub struct Invoker {
    pub pc: u32,                        // 程序计数器 = 入口编号
    pub data_stack: Vec<u64>,           // 数据栈
    pub return_stack: Vec<u32>,         // 返回栈
    pub registers: Vec<u64>,            // 寄存器组(动态扩容)
    pub extensions: HashMap<String, ExtFn>, // 扩展函数表
    pub steps: u64,                     // 执行步数
}
```

- 4 个初始寄存器, 自动扩容
- step(): 取 entry[pc] → 顺序执行 ops → 遇到 Jump/Call/Return/Halt 跳转或终止
- run(): 循环 step 直到 Halted 或错误

## 5. 呼唤 = 执行的范式

在囡囝共生中, "呼唤编程" (Need/Offer/Ping/Pong/Arbitrate) 的底层实现就是呼唤表的 Call + Return。Need = Call(入口编号), 对应的 entry 执行后 Result = Return。Ping/Pong = Call + 参数传递(通过 data_stack)。

这意味着呼唤表的每个入口 = 一个 Need/Offer 的处理函数。整个分布式协商 = 跨节点的 Call + 返回值。

## 6. 与系统其他模块的接入

- **Flow 层的 CONSCIOUSNESS / PROPERTY 流** 在运行时通过 InvocationTable.Ext 注册函数
- **Swarm 多智能体** 的每个 CocoonAgent = 独立的 Invoker 实例 + 各自的 InvocationTable
- **CelestialSpace** 的天庭空间 Call = 进入更高层级的入口编号(500+)
- **SMRL 引擎** 的 Observation 记录 + 裁定 = 一条 OpKind 序列(Mark → Compare → Act)
- **Call Programming 原语** (Need/Offer/Ping/Pong/Arbitrate) 直接映射到 Call/Return/Compare
- **MCCP 字节码虚拟机** 的 Opcode = InvocationTable 的数字编码(u32 index)
- **LiveCode Compiler** 编译后的产物 = InvocationTable 注册表的快照

## 7. 实现约束

- entry_id 0 永远为 NOP, 不可覆盖, 不可删除
- Call 操作在 return_stack 压入 pc+1 (下一条入口编号), 而非当前 pc — 保证 Return 后继续执行下一条
- JumpIfFalse 弹出栈顶, 为零时跳转到 target, 非零时继续 — 这是唯一的条件分支
- Ext(String) 扩展不支持返回值 — 结果通过 data_stack 传递
- no_std 兼容: HashMap 替换为 heapless::HashMap, Vec 替换为 heapless::Vec

// hunyuan-rs/src/flow/invocation_table.rs
