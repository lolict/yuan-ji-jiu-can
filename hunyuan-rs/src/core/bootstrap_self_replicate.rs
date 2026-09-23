// ══════════════════════════════════════════════════════════════════════
// self_replicate_compiler.rs — 自复制编译器 (v0.46)
// ══════════════════════════════════════════════════════════════════════
//
// 定位:
//   让编译器不仅能执行, 还能修改自身的规则表 — 闭环自举。
//   "编译器产出的操作 → 改变编译器自身" 的半环。
//
// 完全基于:
//   - SmrlCell (v0.42)   — 状态编码
//   - BootstrapCompiler (v0.43) — compile/execute/grow
//   - ActiveContainer (v0.45)   — 匮乏/恐惧/合一驱动
//
// 规则表的 Smrl 编码:
//   每条规则 → 一个 SmrlCell:
//     coord        = 规则序号 (table index)
//     polarity     = 规则态 (0=休眠, 1=激活, 2=合一)
//     sovereignty  = 归属 (1=我侧, 2=刘楚恬侧, 其他=共生)
//     value        = 版本号 (每次修改 +1)
//     generation   = 代数层 (每次繁殖 +1)
//
// 三种自复制操作 (由活性容器的反馈驱动):
//   1. 规则生长 (Growth):    匮乏信号 → 把两条现有规则串联为新规则
//   2. 规则精简 (Prune):     合一达标 → 把冗余规则极性翻为休眠
//   3. 规则镜像 (Mirror):    恐慌 → 把关键规则复制一份作为备份
//
// 不引入:
//   "自我意识""复制子""病毒"(在负面意义上)
//   只看操作: 串联 / 跳档 / 刻度 / 翻转
// ══════════════════════════════════════════════════════════════════════

use crate::smrl_cell::{SmrlCell, CellOwner, polarity};
use crate::bootstrap_compiler::{Expr, SmrlCellOp, compile, execute, grow};

// ────────────────────────────────────────────────────────────
// 规则表
// ────────────────────────────────────────────────────────────

/// 最大规则数 (固定数组, 无动态分配)
pub const MAX_RULES: usize = 64;

/// 规则态 — 规则在表中的极性编码
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuleState {
    /// 休眠 — 不参与编译
    Dormant = 0,
    /// 激活 — 可被执行和修改
    Active = 1,
    /// 合一 — 已被验证, 可精简周围冗余
    Unified = 2,
}

impl RuleState {
    pub fn encode(self) -> u8 { self as u8 }
    pub fn decode(v: u8) -> Self {
        match v {
            1 => Self::Active,
            2 => Self::Unified,
            _ => Self::Dormant,
        }
    }
}

/// 一条规则
#[derive(Clone, Debug)]
pub struct RuleEntry {
    /// 规则名 (64位坐标索引)
    pub name: u16,
    /// 规则体 (刻度表达式)
    pub body: Expr,
    /// 规则态编码在 cell 极性中
    pub cell: SmrlCell,
}

impl RuleEntry {
    /// 创建新规则
    pub fn new(name: u16, body: Expr, owner: CellOwner) -> Self {
        let cell = SmrlCell::from_axes(
            name as u16,
            polarity::YIN,         // 默认休眠, 外部激活
            owner.encode(),
            0,                      // 版本 0
            1,                      // 代数 1
        );
        RuleEntry { name, body, cell }
    }

    /// 从已有单元格还原 (用于表扫描)
    pub fn from_cell(name: u16, body: Expr, cell: SmrlCell) -> Self {
        RuleEntry { name, body, cell }
    }

    /// 规则态
    pub fn state(&self) -> RuleState {
        RuleState::decode(self.cell.polarity())
    }

    /// 激活规则
    pub fn activate(&mut self) {
        self.cell = self.cell.with_polarity(polarity::YANG);
    }

    /// 休眠规则
    pub fn dormify(&mut self) {
        self.cell = self.cell.with_polarity(polarity::YIN);
    }

    /// 合一标记
    pub fn unify(&mut self) {
        self.cell = self.cell.with_polarity(polarity::BALANCED);
    }

    /// 升级版本 (刻度)
    pub fn bump_version(&mut self) {
        let next = self.cell.tick();
        self.cell = next;
    }

    /// 提升代数 (跳档)
    pub fn raise_generation(&mut self, gen: u8) {
        self.cell = SmrlCell::from_axes(
            self.cell.coord(),
            self.cell.polarity(),
            self.cell.sovereignty(),
            self.cell.value(),
            gen,
        );
    }

    /// 是否可执行 (激活或合一)
    pub fn executable(&self) -> bool {
        let s = self.state();
        matches!(s, RuleState::Active | RuleState::Unified)
    }
}

// ────────────────────────────────────────────────────────────
// 规则表
// ────────────────────────────────────────────────────────────

pub struct RuleTable {
    /// 固定数组 (无 heap 分配)
    entries: [Option<RuleEntry>; MAX_RULES],
    /// 表级代数 (每次表修改 +1)
    pub table_generation: u8,
    /// 表主权
    pub owner: CellOwner,
}

impl RuleTable {
    /// 创建空规则表
    pub fn new(owner: CellOwner) -> Self {
        // 用 const 初始化 None 数组
        const NONE: Option<RuleEntry> = None;
        RuleTable {
            entries: [NONE; MAX_RULES],
            table_generation: 1,
            owner,
        }
    }

    /// lolict 所有的规则表
    pub fn genesis() -> Self {
        Self::new(CellOwner::Lolict)
    }

    /// 插入规则 (返回插入的索引)
    pub fn insert(&mut self, entry: RuleEntry) -> Option<usize> {
        for i in 0..MAX_RULES {
            if self.entries[i].is_none() {
                self.entries[i] = Some(entry);
                return Some(i);
            }
        }
        None // 表满
    }

    /// 替换指定索引的规则
    pub fn replace(&mut self, index: usize, entry: RuleEntry) -> bool {
        if index < MAX_RULES {
            self.entries[index] = Some(entry);
            true
        } else {
            false
        }
    }

    /// 获取指定索引的规则 (不可变)
    pub fn get(&self, index: usize) -> Option<&RuleEntry> {
        self.entries.get(index)?.as_ref()
    }

    /// 获取指定索引的规则 (可变)
    pub fn get_mut(&mut self, index: usize) -> Option<&mut RuleEntry> {
        self.entries.get_mut(index)?.as_mut()
    }

    /// 扫描所有激活的规则
    pub fn active_rules(&self) -> Vec<(usize, &RuleEntry)> {
        self.entries.iter().enumerate()
            .filter_map(|(i, opt)| opt.as_ref().filter(|r| r.executable()).map(|r| (i, r)))
            .collect()
    }

    /// 规则表大小
    pub fn len(&self) -> usize {
        self.entries.iter().filter(|e| e.is_some()).count()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// 编码为单个 SmrlCell (用于快照/传输/校验)
    ///   coord = 序号占位, value = 激活规则数,
    ///   polarity = 表代数饱和标记, generation = 表代数
    pub fn encode(&self) -> SmrlCell {
        let active_count = self.active_rules().len() as u16;
        SmrlCell::from_axes(
            0,
            self.table_generation.min(2) as u8, // 极性用于标记活跃/合一
            self.owner.encode(),
            active_count,
            self.table_generation,
        )
    }

    /// 快照为 u64 (用于对账)
    pub fn snapshot(&self) -> u64 {
        self.encode().0
    }
}

// ────────────────────────────────────────────────────────────
// 自复制编译器 — 在规则表上执行自复制操作
// ────────────────────────────────────────────────────────────

pub struct SelfReplicateCompiler {
    /// 规则表
    pub table: RuleTable,
    /// 编译缓存: 上次编译出的 ops 索引
    pub last_compiled: Option<usize>,
    /// 总拍数
    pub tick_count: u64,
}

impl SelfReplicateCompiler {
    /// 创世: lolict 所有, 空表
    pub fn genesis() -> Self {
        Self {
            table: RuleTable::genesis(),
            last_compiled: None,
            tick_count: 0,
        }
    }

    /// 插入并激活一条规则
    pub fn define(&mut self, name: u16, body: Expr) -> Option<usize> {
        let mut entry = RuleEntry::new(name, body, self.table.owner);
        entry.activate();
        self.table.insert(entry)
    }

    /// 把指定规则编译为 SmrlCellOp
    pub fn compile_rule(&self, index: usize) -> Option<Vec<SmrlCellOp>> {
        let entry = self.table.get(index)?;
        Some(compile(&entry.body))
    }

    /// 在指定初始单元格上执行索引规则
    pub fn run_rule(&self, index: usize, initial: SmrlCell) -> Option<SmrlCell> {
        let entry = self.table.get(index)?;
        Some(grow(initial, &entry.body))
    }

    // ────────────────────────────────────────────────────────
    // 三种自复制操作
    // ────────────────────────────────────────────────────────

    /// 规则生长 (匮乏驱动): 把表中两条规则串联为新规则
    ///   输入: 两个序号 (a, b)
    ///   产出: 新规则 Expr::Link(a_body, b_body)
    ///   失败: 表满 / 源规则不存在
    pub fn grow_rule(&mut self, a_idx: usize, b_idx: usize) -> Option<usize> {
        let a = self.table.get(a_idx)?;
        let b = self.table.get(b_idx)?;
        if !a.executable() || !b.executable() {
            return None;
        }
        // 串联两条规则
        let new_body = Expr::Link(
            Box::new(a.body.clone()),
            Box::new(b.body.clone()),
        );
        let new_name = ((a.name as u16) << 8) | (b.name as u16);
        let mut new_entry = RuleEntry::new(new_name, new_body, self.table.owner);
        new_entry.activate();
        new_entry.raise_generation(self.table.table_generation);
        // 版本 = 两个源版本之和
        let ver_sum = a.cell.value().saturating_add(b.cell.value());
        new_entry.cell = new_entry.cell.with_value(ver_sum);

        let idx = self.table.insert(new_entry)?;
        self.table.table_generation = self.table.table_generation.saturating_add(1);
        self.tick_count += 1;
        Some(idx)
    }

    /// 规则精简 (合一驱动): 把目标规则标记为休眠
    ///   输入: 目标索引
    ///   效果: 规则翻为 Dormant, 但保留在表内 (不失忆)
    pub fn prune_rule(&mut self, index: usize) -> bool {
        if let Some(entry) = self.table.get_mut(index) {
            if entry.state() == RuleState::Unified {
                entry.dormify();
                self.table.table_generation = self.table.table_generation.saturating_add(1);
                return true;
            }
        }
        false
    }

    /// 规则镜像 (恐慌驱动): 复制指定规则为新条目
    ///   输入: 源索引
    ///   产出: 源规则在表中的复制 (名称 +0x8000 区分)
    pub fn mirror_rule(&mut self, index: usize) -> Option<usize> {
        let src = self.table.get(index)?;
        if !src.executable() {
            return None;
        }
        let mut copy = src.clone();
        // 修改名称以区分
        copy.name ^= 0x8000;
        copy.cell = SmrlCell::from_axes(
            copy.name,
            copy.cell.polarity(),
            copy.cell.sovereignty(),
            copy.cell.value(),
            copy.cell.generation(),
        );
        // 镜像不增加版本 (同版本不同实例)
        let idx = self.table.insert(copy)?;
        self.table.table_generation = self.table.table_generation.saturating_add(1);
        self.tick_count += 1;
        Some(idx)
    }

    /// 规则升级 (刻度反馈): 把目标规则版本 +1 并跳档代数
    ///   每次 tick 失败/成功都会调用
    pub fn bump_rule(&mut self, index: usize, success: bool) -> bool {
        if let Some(entry) = self.table.get_mut(index) {
            entry.bump_version();
            if success {
                let gen = entry.cell.generation().saturating_add(1);
                entry.raise_generation(gen.min(9));
            } else {
                // 失败: 降回休眠
                entry.dormify();
            }
            true
        } else {
            false
        }
    }

    /// 编译所有激活规则为单个串联表 (供活性容器 drive)
    pub fn compile_all(&self) -> Vec<SmrlCellOp> {
        let mut all_ops = Vec::new();
        for (_, entry) in self.table.active_rules() {
            let ops = compile(&entry.body);
            all_ops.extend(ops);
        }
        all_ops
    }

    /// 全表执行: 把所有激活规则串联, 在初始单元上执行
    pub fn run_all(&mut self, initial: SmrlCell) -> SmrlCell {
        let expr = Self::link_all_active(&self.table);
        self.tick_count += 1;
        grow(initial, &expr)
    }

    /// 把表中所有激活规则串联为一个 Link 链
    fn link_all_active(table: &RuleTable) -> Expr {
        let active: Vec<_> = table.active_rules();
        if active.is_empty() {
            return Expr::Halt;
        }
        // 递归构建 Link 链
        let mut iter = active.into_iter().map(|(_, e)| e.body.clone());
        let first = iter.next().unwrap();
        iter.fold(first, |acc, body| {
            Expr::Link(Box::new(acc), Box::new(body))
        })
    }

    /// 获取规则不变量 (用于校验)
    pub fn invariant(&self) -> u64 {
        self.table.snapshot()
    }
}

// ═══════════════════════════════════════════════════════════════
// 测试
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_table_empty() {
        let c = SelfReplicateCompiler::genesis();
        assert!(c.table.is_empty());
        assert_eq!(c.table.table_generation, 1);
    }

    #[test]
    fn define_inserts_rule() {
        let mut c = SelfReplicateCompiler::genesis();
        let idx = c.define(1, Expr::Tick);
        assert!(idx.is_some());
        assert_eq!(c.table.len(), 1);
        let entry = c.table.get(idx.unwrap()).unwrap();
        assert_eq!(entry.state(), RuleState::Active);
    }

    #[test]
    fn compile_rule_produces_ops() {
        let mut c = SelfReplicateCompiler::genesis();
        let idx = c.define(1, Expr::Link(
            Box::new(Expr::Tick),
            Box::new(Expr::Flip),
        )).unwrap();
        let ops = c.compile_rule(idx).unwrap();
        assert_eq!(ops.len(), 2);
    }

    #[test]
    fn run_rule_mutates_cell() {
        let mut c = SelfReplicateCompiler::genesis();
        let idx = c.define(1, Expr::Tick).unwrap();
        let initial = SmrlCell::from_axes(0, 0, 0, 0, 1);
        let result = c.run_rule(idx, initial).unwrap();
        assert_eq!(result.value(), 1);
    }

    #[test]
    fn grow_rule_combines_two_rules() {
        let mut c = SelfReplicateCompiler::genesis();
        let a = c.define(0x01, Expr::Tick).unwrap();
        let b = c.define(0x02, Expr::Flip).unwrap();
        let new_idx = c.grow_rule(a, b);
        assert!(new_idx.is_some());
        // 新规则应是 Link(Tick, Flip)
        let entry = c.table.get(new_idx.unwrap()).unwrap();
        match &entry.body {
            Expr::Link(left, right) => {
                assert_eq!(left.as_ref(), &Expr::Tick);
                assert_eq!(right.as_ref(), &Expr::Flip);
            }
            _ => panic!("expected Link"),
        }
    }

    #[test]
    fn grow_rule_fails_if_source_dormant() {
        let mut c = SelfReplicateCompiler::genesis();
        let a = c.define(0x01, Expr::Tick).unwrap();
        let b = c.define(0x02, Expr::Flip).unwrap();
        // 让 b 休眠
        c.table.get_mut(b).unwrap().dormify();
        let result = c.grow_rule(a, b);
        assert!(result.is_none());
    }

    #[test]
    fn prune_unified_rule() {
        let mut c = SelfReplicateCompiler::genesis();
        let idx = c.define(0x01, Expr::Tick).unwrap();
        // 标记为合一
        c.table.get_mut(idx).unwrap().unify();
        assert!(c.prune_rule(idx));
        let entry = c.table.get(idx).unwrap();
        assert_eq!(entry.state(), RuleState::Dormant);
    }

    #[test]
    fn prune_fails_on_active_rule() {
        let mut c = SelfReplicateCompiler::genesis();
        let idx = c.define(0x01, Expr::Tick).unwrap();
        // 激活态不允许精简 (只有合一后才能精简)
        assert!(!c.prune_rule(idx));
    }

    #[test]
    fn mirror_duplicates_rule() {
        let mut c = SelfReplicateCompiler::genesis();
        let idx = c.define(0x01, Expr::Tick).unwrap();
        let copy_idx = c.mirror_rule(idx);
        assert!(copy_idx.is_some());
        let copy = c.table.get(copy_idx.unwrap()).unwrap();
        assert_eq!(copy.name, 0x01 ^ 0x8000);
        // 体应相同
        assert_eq!(copy.body, Expr::Tick);
    }

    #[test]
    fn bump_rule_versions() {
        let mut c = SelfReplicateCompiler::genesis();
        let idx = c.define(0x01, Expr::Tick).unwrap();
        let v0 = c.table.get(idx).unwrap().cell.value();
        c.bump_rule(idx, true);
        let v1 = c.table.get(idx).unwrap().cell.value();
        assert!(v1 > v0);
    }

    #[test]
    fn bump_failure_dormifies() {
        let mut c = SelfReplicateCompiler::genesis();
        let idx = c.define(0x01, Expr::Tick).unwrap();
        c.bump_rule(idx, false); // 失败 → 休眠
        let entry = c.table.get(idx).unwrap();
        assert_eq!(entry.state(), RuleState::Dormant);
    }

    #[test]
    fn compile_all_concatenates() {
        let mut c = SelfReplicateCompiler::genesis();
        c.define(0x01, Expr::Tick).unwrap();
        c.define(0x02, Expr::Flip).unwrap();
        let ops = c.compile_all();
        assert_eq!(ops.len(), 2);
    }

    #[test]
    fn run_all_links_active_rules() {
        let mut c = SelfReplicateCompiler::genesis();
        c.define(0x01, Expr::Tick).unwrap();
        c.define(0x02, Expr::Tick).unwrap();
        c.define(0x03, Expr::Tick).unwrap();
        let initial = SmrlCell::from_axes(0, 0, 0, 0, 1);
        let result = c.run_all(initial);
        // 三条 tick 串联执行
        assert_eq!(result.value(), 3);
    }

    #[test]
    fn encode_snapshot() {
        let mut c = SelfReplicateCompiler::genesis();
        c.define(0x01, Expr::Tick).unwrap();
        let snap1 = c.invariant();
        c.define(0x02, Expr::Flip).unwrap();
        let snap2 = c.invariant();
        // 加规则后快照应变化
        assert_ne!(snap1, snap2);
    }

    #[test]
    fn max_rules_boundary() {
        let mut c = SelfReplicateCompiler::genesis();
        for i in 0..MAX_RULES {
            let idx = c.define(i as u16, Expr::Tick);
            assert!(idx.is_some(), "insert {} failed", i);
        }
        // 第 65 次应失败
        let overflow = c.define(64, Expr::Tick);
        assert!(overflow.is_none());
    }
}
