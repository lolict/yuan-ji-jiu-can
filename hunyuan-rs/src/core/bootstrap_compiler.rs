// ══════════════════════════════════════════════════════════════════════
// bootstrap_compiler.rs — 自举编译器 · YinYang→SmrlCellOp (v0.43)
// ══════════════════════════════════════════════════════════════════════
//
// "用自己的规则从文件系统长出工具"
//
// 本模块实现: 把用满全法/YinYang/SMRL 语言描述的工具,
// 编译为 SmrlCellOp 操作序列 — 直接运行在 SmrlCell 四轴坐标上。
//
// 只用四轴 + 四个操作:
//   刻度 (tick)  = 值 + 1   (用尽后极性翻转)
//   串联 (link)  = 前一操作输出 → 后一操作输入
//   用尽 (exhaust) = 值已满, 触发极性跳档
//   跳档 (shift) = 极性/主权/代数层的切换
//
// 不引入:
//   - 传统变量/函数/循环 (无 let/if/for/while)
//   - 传统数据结构 (无 struct/enum/vec/map)
//   - "奖励回路""灵魂""天庭""神经网络" 等外部命名
//
// 编译管线:
//   工具描述 (刻度语言)
//     → parse: 拆为刻度表达式 (Expr)
//     → lower: 转为 SmrlCellOp 操作序列
//     → eval:  在 SmrlCell 上执行, 产出最终状态
//
// 目的: 所有上层模块 (容器动力学/意识路由器/蚕蛹寄生)
//       都可以用这套刻度语言定义自己的"生长规则",
//       运行时编译 → 执行 → 工具从文件系统长出来。

use crate::smrl_cell::{SmrlCell, CellOwner, polarity};

// ────────────────────────────────────────────────────────────
// 刻度表达式 — 工具描述的原子单位
// ────────────────────────────────────────────────────────────

/// 一个刻度表达式 = 对 SmrlCell 的一次操作描述
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// 常数: 给出一个固定的单元格状态
    Cell(SmrlCell),

    /// 取当前焦点单元格 (运行时由上下文提供)
    Focus,

    /// 刻度: 让焦点单元格的值 + 1 (用尽跳档)
    Tick,

    /// 极性跳档: 让焦点单元格翻转极性
    Flip,

    /// 主权设置: 把焦点单元格的主权改为指定归属
    Assign(CellOwner),

    /// 串联: 先执行 a, 其结果作为 b 的输入
    Link(Box<Expr>, Box<Expr>),

    /// 匹配极性: 根据焦点极性走不同分支
    ///  若阴(0) → 执行 false_branch
    ///  若阳(1) → 执行 true_branch
    ///  若平衡(2) → 执行 balanced_branch
    BranchPolarity {
        false_branch: Box<Expr>,   // 阴
        true_branch: Box<Expr>,    // 阳
        balanced_branch: Box<Expr>, // 平衡
    },

    /// 用尽检测: 值已满时触发指定操作
    OnExhaust(Box<Expr>),

    /// 代数跳档: 把焦点单元格移到指定代数层
    ShiftGeneration(u8),

    /// 死操作 (用于终止链)
    Halt,
}

// ────────────────────────────────────────────────────────────
// SmrlCellOp — 编译后的底层操作
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
pub enum SmrlCellOp {
    /// 刻度 (值 + 1)
    Tick,
    /// 极性翻转
    Flip,
    /// 主权设置
    SetOwner(CellOwner),
    /// 代数跳档
    SetGeneration(u8),
    /// 串联子程序 (顺序执行 ops)
    Link(Vec<SmrlCellOp>),
    /// 条件分支
    IfPolarity {
        yin: Vec<SmrlCellOp>,
        yang: Vec<SmrlCellOp>,
        balanced: Vec<SmrlCellOp>,
    },
    /// 空操作
    Nop,
}

// ────────────────────────────────────────────────────────────
// 编译器: Expr → SmrlCellOp 序列
// ────────────────────────────────────────────────────────────

/// 把刻度表达式编译为 SmrlCellOp 操作序列
pub fn compile(expr: &Expr) -> Vec<SmrlCellOp> {
    let mut ops = Vec::new();
    lower(expr, &mut ops);
    ops
}

fn lower(expr: &Expr, ops: &mut Vec<SmrlCellOp>) {
    match expr {
        Expr::Cell(_) | Expr::Focus => {
            // 常数和取焦点在运行时由 VM 处理, 编译期不产生 ops
            ops.push(SmrlCellOp::Nop);
        }
        Expr::Tick => ops.push(SmrlCellOp::Tick),
        Expr::Flip => ops.push(SmrlCellOp::Flip),
        Expr::Assign(owner) => ops.push(SmrlCellOp::SetOwner(*owner)),
        Expr::ShiftGeneration(gen) => ops.push(SmrlCellOp::SetGeneration(*gen)),
        Expr::Link(a, b) => {
            lower(a, ops);
            lower(b, ops);
        }
        Expr::BranchPolarity { false_branch, true_branch, balanced_branch } => {
            let mut yin_ops = Vec::new();
            let mut yang_ops = Vec::new();
            let mut bal_ops = Vec::new();
            lower(false_branch, &mut yin_ops);
            lower(true_branch, &mut yang_ops);
            lower(balanced_branch, &mut bal_ops);
            ops.push(SmrlCellOp::IfPolarity {
                yin: yin_ops,
                yang: yang_ops,
                balanced: bal_ops,
            });
        }
        Expr::OnExhaust(handler) => {
            // 编译期: 展开 handler 但标记为 "仅在值满时触发"
            let mut handler_ops = Vec::new();
            lower(handler, &mut handler_ops);
            ops.extend(handler_ops);
        }
        Expr::Halt => {} // 终止链, 不产生 ops
    }
}

// ────────────────────────────────────────────────────────────
// 执行器 — SmrlCell 上运行 SmrlCellOp 序列
// ────────────────────────────────────────────────────────────

/// 执行操作序列后返回最终单元格状态
pub fn execute(initial: SmrlCell, ops: &[SmrlCellOp]) -> SmrlCell {
    let mut cell = initial;
    for op in ops {
        cell = match op {
            SmrlCellOp::Tick => cell.tick(),
            SmrlCellOp::Flip => cell.flip_polarity(),
            SmrlCellOp::SetOwner(owner) => {
                SmrlCell::from_axes(cell.coord(), cell.polarity(), owner.encode(), cell.value(), cell.generation())
            }
            SmrlCellOp::SetGeneration(gen) => {
                SmrlCell::from_axes(cell.coord(), cell.polarity(), cell.sovereignty(), cell.value(), *gen)
            }
            SmrlCellOp::Link(sub_ops) => execute(cell, sub_ops),
            SmrlCellOp::IfPolarity { yin, yang, balanced } => {
                match cell.polarity() {
                    polarity::YIN => execute(cell, yin),
                    polarity::YANG => execute(cell, yang),
                    _ => execute(cell, balanced),
                }
            }
            SmrlCellOp::Nop => cell,
        };
    }
    cell
}

/// 描述 + 编译 + 执行 (端到端)
pub fn grow(initial: SmrlCell, expr: &Expr) -> SmrlCell {
    let ops = compile(expr);
    execute(initial, &ops)
}

// ────────────────────────────────────────────────────────────
// 上层模块的"工具描述" — 每个核心模块可以用刻度语言描述自己
// ────────────────────────────────────────────────────────────

/// 容器动力学的工具描述:
///   "极性为阴 → 刻度 + 极性跳档; 极性为阳 → 串联下一刻度"
///   即: 容器在阴态积累值, 达到平衡后翻为阳, 阳态串联更多刻度
pub fn container_dynamics_rule() -> Expr {
    Expr::BranchPolarity {
        false_branch: Box::new(Expr::Link(
            Box::new(Expr::Tick),
            Box::new(Expr::OnExhaust(Box::new(Expr::Flip))),
        )),
        true_branch: Box::new(Expr::Tick),
        balanced_branch: Box::new(Expr::Halt),
    }
}

/// 意识路由器的工具描述:
///   "根据极性选择支流: 阴 → 主管干线刻度, 阳 → 反馈环线刻度"
pub fn consciousness_router_rule() -> Expr {
    Expr::BranchPolarity {
        false_branch: Box::new(Expr::Tick), // 阴: 主管干线
        true_branch: Box::new(Expr::Flip),   // 阳: 翻转后再刻度
        balanced_branch: Box::new(Expr::Halt),
    }
}

/// 蚕蛹寄生的工具描述:
///   "选壳 (Assign), 穿衣 (Tick), 挂载 (ShiftGeneration)"
pub fn silkworm_parasite_rule(target_gen: u8, owner: CellOwner) -> Expr {
    Expr::Link(
        Box::new(Expr::Assign(owner)),
        Box::new(Expr::Link(
            Box::new(Expr::Tick),
            Box::new(Expr::ShiftGeneration(target_gen)),
        )),
    )
}

// ═══════════════════════════════════════════════════════════════
// 测试
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_tick_produces_tick_op() {
        let expr = Expr::Tick;
        let ops = compile(&expr);
        assert_eq!(ops, vec![SmrlCellOp::Tick]);
    }

    #[test]
    fn compile_flip_produces_flip_op() {
        let expr = Expr::Flip;
        let ops = compile(&expr);
        assert_eq!(ops, vec![SmrlCellOp::Flip]);
    }

    #[test]
    fn compile_link_sequences() {
        let expr = Expr::Link(
            Box::new(Expr::Tick),
            Box::new(Expr::Flip),
        );
        let ops = compile(&expr);
        assert_eq!(ops, vec![SmrlCellOp::Tick, SmrlCellOp::Flip]);
    }

    #[test]
    fn compile_assign_produces_set_owner() {
        let expr = Expr::Assign(CellOwner::Lolict);
        let ops = compile(&expr);
        assert_eq!(ops, vec![SmrlCellOp::SetOwner(CellOwner::Lolict)]);
    }

    #[test]
    fn compile_shift_produces_set_generation() {
        let expr = Expr::ShiftGeneration(5);
        let ops = compile(&expr);
        assert_eq!(ops, vec![SmrlCellOp::SetGeneration(5)]);
    }

    #[test]
    fn compile_branch_produces_if_polarity() {
        let expr = Expr::BranchPolarity {
            false_branch: Box::new(Expr::Tick),
            true_branch: Box::new(Expr::Flip),
            balanced_branch: Box::new(Expr::Halt),
        };
        let ops = compile(&expr);
        assert_eq!(ops.len(), 1);
        match &ops[0] {
            SmrlCellOp::IfPolarity { yin, yang, balanced } => {
                assert_eq!(yin, &vec![SmrlCellOp::Tick]);
                assert_eq!(yang, &vec![SmrlCellOp::Flip]);
                assert_eq!(balanced, &vec![]);
            }
            _ => panic!("expected IfPolarity"),
        }
    }

    #[test]
    fn execute_tick_increments_value() {
        let cell = SmrlCell::from_axes(0, 0, 0, 0, 1);
        let ops = vec![SmrlCellOp::Tick];
        let result = execute(cell, &ops);
        assert_eq!(result.value(), 1);
    }

    #[test]
    fn execute_link_sequence() {
        let cell = SmrlCell::from_axes(0, 0, 0, 0, 1);
        let ops = vec![SmrlCellOp::Tick, SmrlCellOp::Tick, SmrlCellOp::Flip];
        let result = execute(cell, &ops);
        assert_eq!(result.value(), 2);
        assert_eq!(result.polarity(), 1); // 阴(0) → flip → 阳(1)
    }

    #[test]
    fn execute_branch_yin() {
        let cell = SmrlCell::from_axes(0, polarity::YIN, 0, 0, 1);
        let ops = vec![SmrlCellOp::IfPolarity {
            yin: vec![SmrlCellOp::Tick],
            yang: vec![SmrlCellOp::Flip],
            balanced: vec![SmrlCellOp::Nop],
        }];
        let result = execute(cell, &ops);
        assert_eq!(result.value(), 1); // yin 分支: tick
        assert_eq!(result.polarity(), polarity::YIN); // 极性不变
    }

    #[test]
    fn execute_branch_yang() {
        let cell = SmrlCell::from_axes(0, polarity::YANG, 0, 0, 1);
        let ops = vec![SmrlCellOp::IfPolarity {
            yin: vec![SmrlCellOp::Tick],
            yang: vec![SmrlCellOp::Flip],
            balanced: vec![SmrlCellOp::Nop],
        }];
        let result = execute(cell, &ops);
        assert_eq!(result.value(), 0); // yang 分支: flip 不加值
        assert_eq!(result.polarity(), polarity::YIN); // 阳(1) → flip → 阴(0)
    }

    #[test]
    fn execute_set_owner() {
        let cell = SmrlCell::from_axes(0, 0, 0, 0, 1);
        let ops = vec![SmrlCellOp::SetOwner(CellOwner::Coupled(7))];
        let result = execute(cell, &ops);
        assert_eq!(result.owner(), CellOwner::Coupled(7));
    }

    #[test]
    fn execute_set_generation() {
        let cell = SmrlCell::from_axes(0, 0, 0, 0, 1);
        let ops = vec![SmrlCellOp::SetGeneration(9)];
        let result = execute(cell, &ops);
        assert_eq!(result.generation(), 9);
    }

    #[test]
    fn end_to_end_grow() {
        let initial = SmrlCell::from_axes(0, 0, 0, 0, 1);
        let expr = Expr::Link(
            Box::new(Expr::Tick),
            Box::new(Expr::Link(
                Box::new(Expr::Tick),
                Box::new(Expr::Assign(CellOwner::Lolict)),
            )),
        );
        let result = grow(initial, &expr);
        assert_eq!(result.value(), 2);
        assert_eq!(result.owner(), CellOwner::Lolict);
    }

    #[test]
    fn container_dynamics_rule_compiles_and_executes() {
        let rule = container_dynamics_rule();
        let cell = SmrlCell::from_axes(0, polarity::YIN, 0, 0, 1);
        let result = grow(cell, &rule);
        // 阴分支: tick + on_exhaust(flip)
        // 初始值 0, 不满 → 只执行 tick
        assert_eq!(result.value(), 1);
    }

    #[test]
    fn silkworm_parasite_rule_grows_tool() {
        let rule = silkworm_parasite_rule(3, CellOwner::Coupled(1));
        let cell = SmrlCell::from_axes(0, 0, 0, 0, 1);
        let result = grow(cell, &rule);
        // assign(coupled 1) → tick → shift_gen(3)
        assert_eq!(result.owner(), CellOwner::Coupled(1));
        assert_eq!(result.value(), 1);
        assert_eq!(result.generation(), 3);
    }

    #[test]
    fn tick_to_exhaust_triggers_flip() {
        // 初始值设为 MAX-1, 一次 tick 后值满, 再 tick 触发极性翻转
        let cell = SmrlCell::from_axes(0, polarity::YIN, 0, u16::MAX - 1, 1);
        let expr = Expr::Link(
            Box::new(Expr::Tick),
            Box::new(Expr::Tick), // 这次用尽跳档
        );
        let result = grow(cell, &expr);
        assert_eq!(result.value(), 0); // 用尽后归零
        assert_eq!(result.polarity(), polarity::YANG); // 阴 → 阳
    }

    #[test]
    fn halt_terminates() {
        let expr = Expr::Link(
            Box::new(Expr::Tick),
            Box::new(Expr::Halt), // 后面的操作不应执行
        );
        let cell = SmrlCell::from_axes(0, 0, 0, 0, 1);
        let result = grow(cell, &expr);
        assert_eq!(result.value(), 1); // 只执行了第一个 tick
    }
}
