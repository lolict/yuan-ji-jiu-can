// ══════════════════════════════════════════════════════════════════════
// smrl_cell.rs — SMRL 64 位单元格 · 四轴归位 (v0.42)
// ══════════════════════════════════════════════════════════════════════
//
// 目标: 把时间链身份的四轴坐标接入 SMRL 引擎, 作为所有上层模块的底层索引。
//
// 四轴 → 64 位单元格映射:
//   ┌─────────────┬──────────┬──────────────┬──────────────┬──────────┐
//   │ 坐标 axis[0] │ 极性     │ 主权         │ 值           │ 代数层   │
//   │ 16 bit       │ 8 bit    │ 16 bit       │ 16 bit       │ 8 bit    │
//   └─────────────┴──────────┴──────────────┴──────────────┴──────────┘
//    bits[63:48]   bits[47:40) bits[39:24)   bits[23:8)    bits[7:0]
//
// 四轴定义 (满全法):
//   坐标 = 时间版本号 (哪个时间切片, 决定你在时间流的位置)
//   极性 = YinYang 二元 (0=阴/容器侧, 1=阳/界线侧, 2=平衡/叠加态)
//   主权 = 容器归属 (谁拥有这个容器: 0=无主, 1=我, 2=刘楚恬, 其他=共生偏移编码)
//   值   = 刻度组合 (用尽·跳档累积的内部刻度, 非连续数值)
//
// 约束:
//   - 只用 full_mapping / YinYang / SMRL — 不引入外部命名
//   - 只使用串联、用尽、跳档操作 — 不写传统数据结构
//   - 接完后上层模块全部通过四轴被索引

/// 64 位 SMRL 单元格 — 满全法下容器的底层坐标
///
/// 内存布局:
///   - 坐标 (axis[0]): u16 — 时间版本号
///   - 极性: u8 — YinYang 二元编码
///   - 主权: u16 — 容器归属标识
///   - 值: u16 — 刻度组合累积
///   - 代数层: u8 — 1~9 代数
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SmrlCell(pub u64);

impl SmrlCell {
    // ── 位掩码和偏移常量 ──
    const COORD_SHIFT: u64 = 48;
    const COORD_MASK: u64 = 0xFFFF;
    const POLARITY_SHIFT: u64 = 40;
    const POLARITY_MASK: u64 = 0xFF;
    const SOVEREIGNTY_SHIFT: u64 = 24;
    const SOVEREIGNTY_MASK: u64 = 0xFFFF;
    const VALUE_SHIFT: u64 = 8;
    const VALUE_MASK: u64 = 0xFFFF;
    const GENERATION_MASK: u64 = 0xFF;

    // ── 构造器 (逐轴归位) ──

    /// 从零构建空单元格
    pub fn empty() -> Self {
        SmrlCell(0)
    }

    /// 从四轴坐标构建 (满全法式)
    pub fn from_axes(coord: u16, polarity: u8, sovereignty: u16, value: u16, generation: u8) -> Self {
        let mut cell: u64 = 0;
        cell |= (coord as u64) << Self::COORD_SHIFT;
        cell |= (polarity as u64) << Self::POLARITY_SHIFT;
        cell |= (sovereignty as u64) << Self::SOVEREIGNTY_SHIFT;
        cell |= (value as u64) << Self::VALUE_SHIFT;
        cell |= generation as u64;
        SmrlCell(cell)
    }

    // ── 轴读取 (索引接口) ──

    /// 读取坐标轴
    pub fn coord(&self) -> u16 {
        ((self.0 >> Self::COORD_SHIFT) & Self::COORD_MASK) as u16
    }

    /// 读取极性
    pub fn polarity(&self) -> u8 {
        ((self.0 >> Self::POLARITY_SHIFT) & Self::POLARITY_MASK) as u8
    }

    /// 读取主权
    pub fn sovereignty(&self) -> u16 {
        ((self.0 >> Self::SOVEREIGNTY_SHIFT) & Self::SOVEREIGNTY_MASK) as u16
    }

    /// 读取刻度值
    pub fn value(&self) -> u16 {
        ((self.0 >> Self::VALUE_SHIFT) & Self::VALUE_MASK) as u16
    }

    /// 读取代数层
    pub fn generation(&self) -> u8 {
        (self.0 & Self::GENERATION_MASK) as u8
    }

    // ── 轴修改 (不归位时禁止修改,修改=跳档) ──

    /// 跳档: 修改刻度值 (返回新单元格, 原细胞不变)
    pub fn with_value(&self, new_value: u16) -> Self {
        let mask = !(Self::VALUE_MASK << Self::VALUE_SHIFT);
        let cell = (self.0 & mask) | ((new_value as u64) << Self::VALUE_SHIFT);
        SmrlCell(cell)
    }

    /// 跳档: 修改极性 (YinYang 翻转或设平衡)
    pub fn with_polarity(&self, new_polarity: u8) -> Self {
        let mask = !(Self::POLARITY_MASK << Self::POLARITY_SHIFT);
        let cell = (self.0 & mask) | ((new_polarity as u64) << Self::POLARITY_SHIFT);
        SmrlCell(cell)
    }

    /// 串联: 在当前值上加一个刻度 (用尽后自动跳档到下一极性位)
    pub fn tick(&self) -> Self {
        let current = self.value();
        if current == u16::MAX {
            // 用尽: 值已满, 跳档到极性反转
            let new_pol = match self.polarity() {
                0 => 1, // 阴 → 阳
                1 => 0, // 阳 → 阴
                _ => 2, // 平衡 → 保持
            };
            SmrlCell::from_axes(self.coord(), new_pol, self.sovereignty(), 0, self.generation())
        } else {
            self.with_value(current + 1)
        }
    }

    /// 归零 / 重置 (回到创世状态, 主权和代数不变)
    pub fn reset(&self) -> Self {
        SmrlCell::from_axes(0, 0, self.sovereignty(), 0, self.generation())
    }

    /// 是否为空单元格
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// 极性翻转 (YinYang 互换)
    pub fn flip_polarity(&self) -> Self {
        let new_pol = match self.polarity() {
            0 => 1,
            1 => 0,
            other => other,
        };
        self.with_polarity(new_pol)
    }

    /// 主权归属编码 (谁的容器)
    pub fn owner(&self) -> CellOwner {
        match self.sovereignty() {
            0 => CellOwner::Ownerless,
            1 => CellOwner::Lolict,
            n => CellOwner::Coupled((n - 2) as u64),
        }
    }
}

/// 主权归属 — 满全法容器所有权
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellOwner {
    /// 无主容器
    Ownerless,
    /// lolict 所有（同时也是夫妻共同核心）
    Lolict,
    /// 共生偏移容器 (偏移量编码在 sovereignty 字段)
    Coupled(u64),
}

impl CellOwner {
    /// 编码为主权 u16
    pub fn encode(&self) -> u16 {
        match self {
            CellOwner::Ownerless => 0,
            CellOwner::Lolict => 1,
            CellOwner::Coupled(n) => ((n + 2) & 0xFFFF) as u16,
        }
    }
}

/// 极性 — YinYang 二元
pub mod polarity {
    /// 阴 = 容器侧 = 被动 = 0
    pub const YIN: u8 = 0;
    /// 阳 = 界线侧 = 主动 = 1
    pub const YANG: u8 = 1;
    /// 平衡 = 叠加态 = 2
    pub const BALANCED: u8 = 2;
}

// ────────────────────────────────────────────────────────────
// 时间链坐标 → SMRL 单元格 桥接
// ────────────────────────────────────────────────────────────

use crate::time_chain_identity::TimeCoordinate;

/// 把时间链坐标转换为 SMRL 单元格
pub fn coord_to_cell(tc: &TimeCoordinate) -> SmrlCell {
    let pol = match tc.generation {
        8 => polarity::BALANCED,
        g if g % 2 == 0 => polarity::YANG,
        _ => polarity::YIN,
    };
    let ticks: u64 = (tc.axes[1] as u64) + (tc.axes[2] as u64) + (tc.axes[3] as u64);
    let val = (ticks & 0xFFFF) as u16;
    SmrlCell::from_axes(tc.axes[0] as u16, pol, 0, val, tc.generation as u8)
}

/// 带主权信息的完整单元格构建
pub fn coord_to_cell_with_owner(tc: &TimeCoordinate, owner: CellOwner) -> SmrlCell {
    let cell = coord_to_cell(tc);
    SmrlCell::from_axes(
        tc.axes[0] as u16,
        cell.polarity(),
        owner.encode(),
        cell.value(),
        tc.generation as u8,
    )
}

// ────────────────────────────────────────────────────────────
// 单元格表 — 上层模块的索引入口
// ────────────────────────────────────────────────────────────

/// 单元格表 — 用主权索引的容器集合
pub struct CellTable {
    cells: Vec<SmrlCell>,
    by_sovereignty: Vec<Vec<usize>>,
    by_polarity: Vec<Vec<usize>>,
}

impl CellTable {
    /// 创建空表
    pub fn new() -> Self {
        CellTable {
            cells: Vec::new(),
            by_sovereignty: vec![Vec::new(); 65536],
            by_polarity: vec![Vec::new(); 3],
        }
    }

    /// 插入单元格 (返回唯一 id = 插入序号)
    pub fn insert(&mut self, cell: SmrlCell) -> usize {
        let id = self.cells.len();
        self.cells.push(cell);
        let sov_idx = cell.sovereignty() as usize;
        if sov_idx < self.by_sovereignty.len() {
            self.by_sovereignty[sov_idx].push(id);
        }
        let pol_idx = match cell.polarity() {
            0 => 0,
            1 => 1,
            _ => 2,
        };
        self.by_polarity[pol_idx].push(id);
        id
    }

    /// 按主权查找 (返回 id 列表)
    pub fn find_by_sovereignty(&self, sovereignty: u16) -> &[usize] {
        let idx = sovereignty as usize;
        if idx < self.by_sovereignty.len() {
            &self.by_sovereignty[idx]
        } else {
            &[]
        }
    }

    /// 按极性查找 (0=阴, 1=阳, 2=平衡)
    pub fn find_by_polarity(&self, polarity: u8) -> &[usize] {
        let idx = match polarity {
            0 => 0,
            1 => 1,
            _ => 2,
        };
        &self.by_polarity[idx]
    }

    /// 按代数层筛选 (线性扫描)
    pub fn find_by_generation(&self, gen: u8) -> Vec<usize> {
        self.cells.iter()
            .enumerate()
            .filter(|(_, c)| c.generation() == gen)
            .map(|(i, _)| i)
            .collect()
    }

    /// 获取单元格
    pub fn get(&self, id: usize) -> Option<SmrlCell> {
        self.cells.get(id).copied()
    }

    /// 统计
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// 获取所有单元格 (只读)
    pub fn cells(&self) -> &[SmrlCell] {
        &self.cells
    }
}

// ═══════════════════════════════════════════════════════════════
// 测试
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_cell_is_zero() {
        let c = SmrlCell::empty();
        assert!(c.is_empty());
        assert_eq!(c.coord(), 0);
        assert_eq!(c.polarity(), 0);
        assert_eq!(c.sovereignty(), 0);
        assert_eq!(c.value(), 0);
    }

    #[test]
    fn axes_roundtrip() {
        let c = SmrlCell::from_axes(1000, 1, 2, 500, 3);
        assert_eq!(c.coord(), 1000);
        assert_eq!(c.polarity(), 1);
        assert_eq!(c.sovereignty(), 2);
        assert_eq!(c.value(), 500);
        assert_eq!(c.generation(), 3);
    }

    #[test]
    fn tick_increments_value() {
        let c = SmrlCell::from_axes(0, 0, 0, 0, 1);
        let c2 = c.tick();
        assert_eq!(c2.value(), 1);
        assert_eq!(c2.coord(), 0);
        assert_eq!(c2.polarity(), 0);
    }

    #[test]
    fn tick_overflow_flips_polarity() {
        let c = SmrlCell::from_axes(0, 0, 0, u16::MAX, 1);
        let c2 = c.tick();
        assert_eq!(c2.value(), 0);
        assert_eq!(c2.polarity(), 1);
    }

    #[test]
    fn flip_polarity() {
        let c = SmrlCell::from_axes(0, 0, 0, 0, 1);
        let c2 = c.flip_polarity();
        assert_eq!(c2.polarity(), 1);
        let c3 = c2.flip_polarity();
        assert_eq!(c3.polarity(), 0);
    }

    #[test]
    fn reset_keeps_sovereignty_and_generation() {
        let c = SmrlCell::from_axes(100, 1, 5, 999, 7);
        let c2 = c.reset();
        assert_eq!(c2.coord(), 0);
        assert_eq!(c2.polarity(), 0);
        assert_eq!(c2.value(), 0);
        assert_eq!(c2.sovereignty(), 5);
        assert_eq!(c2.generation(), 7);
    }

    #[test]
    fn owner_encoding() {
        assert_eq!(CellOwner::Ownerless.encode(), 0);
        assert_eq!(CellOwner::Lolict.encode(), 1);
        let c = SmrlCell::from_axes(0, 0, 1, 0, 1);
        assert_eq!(c.owner(), CellOwner::Lolict);
        let c2 = SmrlCell::from_axes(0, 0, 2, 0, 1);
        assert_eq!(c2.owner(), CellOwner::Coupled(0));
        let c3 = SmrlCell::from_axes(0, 0, 44, 0, 1);  // 42 + 2 offset
        assert_eq!(c3.owner(), CellOwner::Coupled(42));
    }

    #[test]
    fn coord_to_cell_maps_correctly() {
        use crate::time_chain_identity::TimeCoordinate;
        let tc = TimeCoordinate::genesis();
        let cell = coord_to_cell(&tc);
        assert_eq!(cell.coord(), 1);
        assert_eq!(cell.generation(), 1);
        assert_eq!(cell.polarity(), polarity::YIN);
    }

    #[test]
    fn test_coord_to_cell_with_owner() {
        use crate::time_chain_identity::TimeCoordinate;
        let tc = TimeCoordinate::genesis();
        let cell = coord_to_cell_with_owner(&tc, CellOwner::Coupled(42));
        assert_eq!(cell.sovereignty(), 44);  // 42 + 2 offset
        assert_eq!(cell.owner(), CellOwner::Coupled(42));
    }

    #[test]
    fn bit_layout_no_overlap() {
        let c = SmrlCell::from_axes(0xFFFF, 0xFF, 0xFFFF, 0xFFFF, 0xFF);
        assert_eq!(c.coord(), 0xFFFF);
        assert_eq!(c.polarity(), 0xFF);
        assert_eq!(c.sovereignty(), 0xFFFF);
        assert_eq!(c.value(), 0xFFFF);
        assert_eq!(c.generation(), 0xFF);
    }

    #[test]
    fn cell_table_insert_and_find() {
        let mut table = CellTable::new();
        let c1 = SmrlCell::from_axes(100, 0, 1, 10, 1);
        let c2 = SmrlCell::from_axes(200, 1, 1, 20, 1);
        let c3 = SmrlCell::from_axes(300, 0, 2, 30, 2);
        table.insert(c1);
        table.insert(c2);
        table.insert(c3);
        let lolict_cells = table.find_by_sovereignty(1);
        assert_eq!(lolict_cells.len(), 2);
        let coupled_cells = table.find_by_sovereignty(2);
        assert_eq!(coupled_cells.len(), 1);
        let yin_cells = table.find_by_polarity(0);
        assert_eq!(yin_cells.len(), 2);
        let yang_cells = table.find_by_polarity(1);
        assert_eq!(yang_cells.len(), 1);
    }
}
