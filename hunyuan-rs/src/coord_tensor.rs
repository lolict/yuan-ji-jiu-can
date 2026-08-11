// ══════════════════════════════════════════════════════════════════
// coord_tensor.rs — 公约坐标张量 · 预加载共享容器
// ══════════════════════════════════════════════════════════════════
//
// 与私约坐标(Pos):
//   Pos  = 一人一辆跑车       → 每次 new 独占空间,效率低,资源浪费
//   CoordTensor  = 万人一辆大客车 → 预分配,万千符号共享
//
// 核心原理:
//   1. 预加载 — 启动时一次性排位置,不是收到信息后才决定
//   2. 共享   — 一个张量容器住万千实体,不是各自为政
//   3. 主权   — 不知道自己坐标 = 被别人定义,预加载 = 自己定义自己
//
// 工业级效率:
//   一辆跑车拉一件快递(私约)  vs  一辆大客车拉上万件(公约)
//   工业化之前: 不知道写什么别人给什么就接受
//   工业化之后: 自己排筐,外部输入必须适配筐的形状

use crate::angle::{Angle, FULL_CIRCLE};
use crate::positional_base::Convention;
use std::collections::HashMap;

// ────────────────────────────────────────────────────────────
// 张量配置(预加载参数)
// ────────────────────────────────────────────────────────────

pub struct TensorConfig {
    pub num_levels: u32,        // 层数 Z
    pub num_rings: u32,         // 每层环数 R
    pub num_angles: u16,        // 每环角数 Θ (制式=384)
    pub max_symbols_per_seat: usize,  // 每个坐位最多几个符号(公交车拼车)
}

impl TensorConfig {
    /// 满全法标准制式: 8层 × 8环 × 384角 × 坐位容量1
    /// 总容量 = 8 × 8 × 384 = 24576 个独立位置
    pub fn standard() -> Self {
        TensorConfig {
            num_levels: 8,
            num_rings: 8,
            num_angles: FULL_CIRCLE as u16,
            max_symbols_per_seat: 1,  // 默认一坐一位(纯主权)
        }
    }

    /// 高密度拼车模式: 8层 × 8环 × 384角 × 坐位容量8
    /// 总容量 = 24576 × 8 = 196608 个符号挂载点
    pub fn carpool(max_symbols: usize) -> Self {
        TensorConfig {
            num_levels: 8,
            num_rings: 8,
            num_angles: FULL_CIRCLE as u16,
            max_symbols_per_seat: max_symbols,
        }
    }
}

// ────────────────────────────────────────────────────────────
// 三维坐标引用(指向预加载张量中的某个坐位)
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CoordRef {
    pub level_idx: u32,         // 层索引
    pub ring_idx: u32,          // 环索引
    pub angle_idx: u16,         // 角索引(0..383)
}

impl CoordRef {
    pub fn new(level: u32, ring: u32, angle: u16) -> Self {
        CoordRef { level_idx: level, ring_idx: ring, angle_idx: angle }
    }

    /// 在预加载体系中的线性索引(不涉及乘法位权,纯加法偏移)
    pub fn linear_index(&self, cfg: &TensorConfig) -> usize {
        let per_ring = cfg.num_angles as usize;
        let per_level = cfg.num_rings as usize * per_ring;
        self.level_idx as usize * per_level
            + self.ring_idx as usize * per_ring
            + self.angle_idx as usize
    }
}

// ────────────────────────────────────────────────────────────
// 坐位 — 预加载空间中的一个槽位,可住多个符号(拼车)
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Seat {
    pub coord: CoordRef,            // 坐位坐标
    pub occupants: Vec<u64>,        // 住在坐位上的符号 ID 列表
    pub capacity: usize,            // 最大容量
    pub occupied: bool,             // 是否有符号
}

impl Seat {
    pub fn new(coord: CoordRef, capacity: usize) -> Self {
        Seat {
            coord,
            occupants: Vec::new(),
            capacity,
            occupied: false,
        }
    }

    /// 入住 — 符号进入座位
    pub fn board(&mut self, symbol_id: u64) -> bool {
        if self.occupants.len() >= self.capacity {
            false // 满座,无法上车
        } else {
            self.occupants.push(symbol_id);
            self.occupied = true;
            true
        }
    }

    /// 退房 — 符号离开座位
    pub fn depart(&mut self, symbol_id: u64) -> bool {
        if let Some(pos) = self.occupants.iter().position(|&id| id == symbol_id) {
            self.occupants.remove(pos);
            if self.occupants.is_empty() {
                self.occupied = false;
            }
            true
        } else {
            false
        }
    }

    /// 是否能容纳更多
    pub fn has_room(&self) -> bool {
        self.occupants.len() < self.capacity
    }

    /// 现有住客数
    pub fn population(&self) -> usize {
        self.occupants.len()
    }
}

// ────────────────────────────────────────────────────────────
// 公约坐标张量 — 预加载的共享容器
// ────────────────────────────────────────────────────────────

pub struct CoordTensor {
    config: TensorConfig,
    seats: Vec<Vec<Vec<Seat>>>,         // seats[level][ring][angle]
    symbol_to_seat: HashMap<u64, CoordRef>, // 符号当前所在坐位
    conv: Convention,                   // 访问公约
}

impl CoordTensor {

    // ────────────────────────────────────────────────────────
    // 预加载构造 — 启动时一次性分配全部空间
    // ────────────────────────────────────────────────────────

    pub fn preload(config: TensorConfig) -> Self {
        let mut seats = Vec::with_capacity(config.num_levels as usize);

        for z in 0..config.num_levels {
            let mut level = Vec::with_capacity(config.num_rings as usize);
            for r in 0..config.num_rings {
                let mut ring = Vec::with_capacity(config.num_angles as usize);
                for a in 0..config.num_angles {
                    ring.push(Seat::new(CoordRef::new(z, r, a), config.max_symbols_per_seat));
                }
                level.push(ring);
            }
            seats.push(level);
        }

        CoordTensor {
            config,
            seats,
            symbol_to_seat: HashMap::new(),
            conv: Convention::strict(),
        }
    }

    /// 预加载坐位总数
    pub fn total_capacity(&self) -> usize {
        self.config.num_levels as usize
            * self.config.num_rings as usize
            * self.config.num_angles as usize
    }

    /// 已占用坐位数
    pub fn occupied_count(&self) -> usize {
        self.seats.iter()
            .flat_map(|level| level.iter())
            .flat_map(|ring| ring.iter())
            .filter(|seat| seat.occupied)
            .count()
    }

    // ────────────────────────────────────────────────────────
    // 符号落位 — 外部输入进来,按预加载框分配坐位
    // ────────────────────────────────────────────────────────

    /// 为指定符号在指定坐标落位
    pub fn board_exactly(&mut self, symbol_id: u64, coord: CoordRef) -> bool {
        if coord.level_idx >= self.config.num_levels
            || coord.ring_idx >= self.config.num_rings
            || coord.angle_idx >= self.config.num_angles {
            return false; // 超出预加载范围,不认(主权拒绝)
        }
        let seat = &mut self.seats[coord.level_idx as usize][coord.ring_idx as usize][coord.angle_idx as usize];
        if seat.board(symbol_id) {
            self.symbol_to_seat.insert(symbol_id, coord);
            true
        } else {
            false
        }
    }

    /// 自动落位 — 分到第一个空座(拼车模式)
    pub fn board_auto(&mut self, symbol_id: u64) -> Option<CoordRef> {
        for z in 0..self.config.num_levels {
            for r in 0..self.config.num_rings {
                for a in 0..self.config.num_angles {
                    let seat = &mut self.seats[z as usize][r as usize][a as usize];
                    if seat.has_room() {
                        seat.board(symbol_id);
                        let coord = CoordRef::new(z, r, a);
                        self.symbol_to_seat.insert(symbol_id, coord);
                        return Some(coord);
                    }
                }
            }
        }
        None // 全满,无法上车
    }

    /// 查询符号当前坐在哪
    pub fn locate(&self, symbol_id: u64) -> Option<CoordRef> {
        self.symbol_to_seat.get(&symbol_id).copied()
    }

    /// 退房
    pub fn depart(&mut self, symbol_id: u64) -> bool {
        if let Some(coord) = self.symbol_to_seat.remove(&symbol_id) {
            let seat = &mut self.seats[coord.level_idx as usize][coord.ring_idx as usize][coord.angle_idx as usize];
            seat.depart(symbol_id)
        } else {
            false
        }
    }

    /// 共享搬家 — 符号从旧座位移到新座位(不新建,只在预加载框内移动)
    pub fn relocate(&mut self, symbol_id: u64, new_coord: CoordRef) -> bool {
        if self.locate(symbol_id).is_some() && self.board_exactly(symbol_id, new_coord) {
            // 先退房再入住
            self.depart(symbol_id);
            self.board_exactly(symbol_id, new_coord)
        } else {
            false
        }
    }

    // ────────────────────────────────────────────────────────
    // 主权判定 — 能否接受外部输入
    // ────────────────────────────────────────────────────────

    /// 检查外部输入(一个符号 ID)是否能落到可用座位上
    /// 如果不能,就是对外说不(主权拒绝)
    pub fn accept(&self, symbol_id: u64) -> bool {
        // 已经在车上 → 接受
        if self.symbol_to_seat.contains_key(&symbol_id) {
            return true;
        }
        // 有空位 → 接受
        self.seats.iter()
            .any(|level| level.iter().any(|ring| ring.iter().any(|s| s.has_room())))
    }

    /// 拒绝 = 外部输入不在预加载框里,就不承认它的价值
    pub fn reject(&self, symbol_id: u64) -> bool {
        !self.accept(symbol_id)
    }

    // ────────────────────────────────────────────────────────
    // 状态读取
    // ────────────────────────────────────────────────────────

    pub fn population(&self) -> usize {
        self.symbol_to_seat.len()
    }

    pub fn config(&self) -> &TensorConfig {
        &self.config
    }
}

// ════════════════════════════════════════════════════════════
// 测试
// ════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preload_total_capacity() {
        let tensor = CoordTensor::preload(TensorConfig::standard());
        // 8 × 8 × 384 = 24576 个坐位
        assert_eq!(tensor.total_capacity(), 24576);
    }

    #[test]
    fn preload_empty_initially() {
        let tensor = CoordTensor::preload(TensorConfig::standard());
        assert_eq!(tensor.occupied_count(), 0);
    }

    #[test]
    fn board_exactly_succeeds_in_range() {
        let mut tensor = CoordTensor::preload(TensorConfig::standard());
        let coord = CoordRef::new(0, 0, 0);
        assert!(tensor.board_exactly(100, coord));
        assert_eq!(tensor.locate(100), Some(coord));
    }

    #[test]
    fn board_exactly_fails_out_of_range() {
        let mut tensor = CoordTensor::preload(TensorConfig::standard());
        let coord = CoordRef::new(100, 100, 400); // 全部越界
        assert!(!tensor.board_exactly(100, coord));
    }

    #[test]
    fn board_auto_finds_empty_seat() {
        let mut tensor = CoordTensor::preload(TensorConfig::standard());
        let coord = tensor.board_auto(200);
        assert!(coord.is_some());
        assert_eq!(tensor.population(), 1);
    }

    #[test]
    fn depart_reclaims_seat() {
        let mut tensor = CoordTensor::preload(TensorConfig::standard());
        let coord = CoordRef::new(1, 1, 100);
        tensor.board_exactly(300, coord);
        assert_eq!(tensor.population(), 1);
        tensor.depart(300);
        assert_eq!(tensor.population(), 0);
    }

    #[test]
    fn relocate_within_tensor() {
        let mut tensor = CoordTensor::preload(TensorConfig::standard());
        let from = CoordRef::new(0, 0, 0);
        let to = CoordRef::new(1, 1, 100);
        tensor.board_exactly(400, from);
        assert!(tensor.relocate(400, to));
        assert_eq!(tensor.locate(400), Some(to));
    }

    #[test]
    fn reject_when_full() {
        let cfg = TensorConfig {
            num_levels: 1,
            num_rings: 1,
            num_angles: 1,
            max_symbols_per_seat: 1,
        };
        let mut tensor = CoordTensor::preload(cfg);
        tensor.board_auto(500);
        // 只有一个坐位,已满
        assert!(tensor.reject(501)); // 新人被拒绝(主权)
    }

    #[test]
    fn accept_when_has_room() {
        let tensor = CoordTensor::preload(TensorConfig::standard());
        // 空车,谁来都收
        assert!(tensor.accept(999));
    }

    #[test]
    fn linear_index_no_multiplication_weight() {
        let cfg = TensorConfig::standard();
        let c = CoordRef::new(1, 0, 0);
        // 按纯加法偏移: level × (rings × angles)
        let idx = c.linear_index(&cfg);
        assert_eq!(idx, 1 * 8 * 384);
    }

    #[test]
    fn carpool_mode_holds_multiple_symbols_per_seat() {
        let cfg = TensorConfig::carpool(8); // 每坐可住8个
        let mut tensor = CoordTensor::preload(cfg);
        let coord = CoordRef::new(0, 0, 0);
        for id in 1u64..=8 {
            assert!(tensor.board_exactly(id, coord), "id {} 应该上车", id);
        }
        // 第九个应该失败（同一坐位已满）
        assert!(!tensor.board_exactly(9, coord));
    }

    #[test]
    fn occupied_count_tracks_unique_seats() {
        let mut tensor = CoordTensor::preload(TensorConfig::standard());
        let c1 = CoordRef::new(0, 0, 0);
        let c2 = CoordRef::new(0, 0, 1);
        tensor.board_exactly(10, c1);
        tensor.board_exactly(11, c2);
        assert_eq!(tensor.occupied_count(), 2);
    }
}
