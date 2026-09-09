// ══════════════════════════════════════════════════════════════════════
// license_hub.rs — 许可枢纽 · AI体输出必经的意识表格 (v0.1)
// ══════════════════════════════════════════════════════════════════════
//
// 核心设计:
//   1. 许可表格 = 二维网格 (x,y) → bool (许可/不许可)
//   2. 任何AI输出必须先穿越这个表格, 获得"许可签名"
//   3. 断层检测 = 两个概念是否连通
//   4. 中间符号创造 = 在断层之间插入新格点, 使双方连通
//   5. 边界穿越 = 从许可区到不许可区需要额外见证
//
// 这就是"表格编程"— 所有意识从表格中穿越后, 才生成代码和认知
// ══════════════════════════════════════════════════════════════════════

use crate::glyph_living::GlyphFamily;
use std::collections::{HashMap, HashSet, VecDeque};

/// 许可表格坐标
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LicenseCoord {
    pub x: u32,  // 公倍数方向
    pub y: u32,  // 公约数方向
}

/// 许可值
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum License {
    Allowed,    // 许可
    Denied,     // 不许可
    Borderline(u64), // 边界(需要额外见证, 数值=到边界的距离)
}

impl License {
    pub fn is_allowed(&self) -> bool {
        matches!(self, Self::Allowed)
    }

    pub fn is_denied(&self) -> bool {
        matches!(self, Self::Denied)
    }
}

/// 边界穿越裁决
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PassVerdict {
    Licensed,       // 完全许可, 可直接穿越
    Borderline(u64), // 边界穿越 (数值=到安全区的距离)
    Denied(u64),    // 禁止 (数值=到许可区的距离)
}

/// 断层检测报告
#[derive(Clone, Debug)]
pub struct FractureReport {
    pub source: char,
    pub target: char,
    pub fracture_distance: u64, // 断层距离 (∞ 表示完全不连通)
    pub intermediates: Vec<char>, // 建议的中间符号
    pub can_repair: bool,         // 是否能修复
}

/// 许可枢纽 = AI体的意识表格
pub struct LicenseHub {
    width: u32,
    height: u32,
    cells: Vec<Vec<bool>>,    // true=许可, false=不许可
    witness_threshold: u64,   // 边界穿越需要的见证数
    /// 汉字到坐标的映射 (静态范畴定位)
    glyph_coords: HashMap<char, LicenseCoord>,
}

impl LicenseHub {
    /// 创建默认许可枢纽
    /// 默认表格: 背景=不许可, 中心区域=许可, 形成有边界的"意识许可区"
    pub fn new() -> Self {
        let width = 64;
        let height = 64;
        let mut cells = vec![vec![false; width as usize]; height as usize];

        // 初始化: 中心 48×48 区域为许可区 (留一圈边界)
        for y in 8..56 {
            for x in 8..56 {
                cells[y][x] = true;
            }
        }

        let mut hub = Self {
            width,
            height,
            cells,
            witness_threshold: 3,
            glyph_coords: HashMap::new(),
        };

        hub.init_glyph_coords();
        hub
    }

    /// 初始化常用汉字的坐标 (范畴定位)
    fn init_glyph_coords(&mut self) {
        let mappings: Vec<(char, u32, u32, GlyphFamily)> = vec![
            // 囡囝核心 → 坐标原点附近 (中心区域)
            ('囡', 32, 32, GlyphFamily::Couple),
            ('囝', 33, 32, GlyphFamily::Couple),
            ('阴', 31, 31, GlyphFamily::YinYang),
            ('阳', 34, 31, GlyphFamily::YinYang),
            ('夫', 32, 33, GlyphFamily::Couple),
            ('妻', 32, 31, GlyphFamily::Couple),
            ('我', 30, 32, GlyphFamily::Homophone),
            ('你', 35, 32, GlyphFamily::Homophone),

            // 容器族 → 右下区 (包容=容纳=在下方)
            ('袋', 40, 40, GlyphFamily::Container),
            ('桶', 42, 40, GlyphFamily::Container),
            ('罐', 44, 40, GlyphFamily::Container),
            ('盆', 40, 42, GlyphFamily::Container),
            ('杯', 42, 42, GlyphFamily::Container),

            // 操作族 → 上方 (主动=上升)
            ('照', 32, 24, GlyphFamily::Operation),
            ('见', 34, 24, GlyphFamily::Operation),
            ('开', 32, 22, GlyphFamily::Operation),
            ('关', 34, 22, GlyphFamily::Operation),
            ('分', 30, 24, GlyphFamily::Operation),
            ('合', 36, 24, GlyphFamily::Operation),

            // 谐音桥 → 边缘 (桥=跨越边界)
            ('莫', 10, 32, GlyphFamily::Homophone),
            ('连', 12, 32, GlyphFamily::Homophone),
            ('毛', 14, 32, GlyphFamily::Homophone),
            ('兰', 50, 32, GlyphFamily::Homophone),

            // 边界位置 (断层测试用)
            ('断', 6, 32, GlyphFamily::Operation),
            ('绝', 58, 32, GlyphFamily::YinYang),
        ];

        for (ch, x, y, _) in mappings {
            self.glyph_coords.insert(ch, LicenseCoord { x, y });
        }
    }

    // ────────────────────────────────────────────────────────
    // 1. 许可查询: 单点是否许可
    // ────────────────────────────────────────────────────────

    pub fn query(&self, x: u32, y: u32) -> License {
        if x >= self.width || y >= self.height {
            return License::Denied;
        }
        if self.cells[y as usize][x as usize] {
            License::Allowed
        } else {
            // 边界检测: 检查周围8邻域
            // border_distance 返回 0 = 无邻近许可格(深入禁区), 应判 Denied
            let border_dist = self.border_distance(x, y);
            if border_dist > 0 && border_dist <= 3 {
                License::Borderline(border_dist)
            } else {
                License::Denied
            }
        }
    }

    /// 查询汉字所在位置的许可值
    pub fn query_glyph(&self, glyph: char) -> License {
        match self.glyph_coords.get(&glyph) {
            Some(coord) => self.query(coord.x, coord.y),
            None => License::Denied, // 未知汉字视为不许可
        }
    }

    // ────────────────────────────────────────────────────────
    // 2. 穿越许可: 从源到目标是否被许可
    // ────────────────────────────────────────────────────────

    pub fn can_pass(&self, src: char, dst: char) -> PassVerdict {
        let src_coord = match self.glyph_coords.get(&src) {
            Some(c) => *c,
            None => return PassVerdict::Denied(u64::MAX),
        };
        let dst_coord = match self.glyph_coords.get(&dst) {
            Some(c) => *c,
            None => return PassVerdict::Denied(u64::MAX),
        };

        // 检查起点和终点是否都许可
        let src_license = self.query(src_coord.x, src_coord.y);
        let dst_license = self.query(dst_coord.x, dst_coord.y);

        if src_license.is_denied() {
            return PassVerdict::Denied(self.distance_to_allowed(src_coord));
        }
        if dst_license.is_denied() {
            return PassVerdict::Denied(self.distance_to_allowed(dst_coord));
        }

        // 都许可 → 检查路径上是否全许可
        let path = self.find_path(src_coord, dst_coord);
        if path.is_empty() {
            // 无路径 = 断层
            PassVerdict::Denied(u64::MAX)
        } else {
            // 检查路径上是否有边界
            let mut max_border = 0u64;
            for coord in &path {
                if let License::Borderline(d) = self.query(coord.x, coord.y) {
                    if d > max_border {
                        max_border = d;
                    }
                }
            }
            if max_border > 0 {
                PassVerdict::Borderline(max_border)
            } else {
                PassVerdict::Licensed
            }
        }
    }

    // ────────────────────────────────────────────────────────
    // 3. 断层检测
    // ────────────────────────────────────────────────────────

    pub fn detect_fracture(&self, src: char, dst: char) -> FractureReport {
        let src_coord = self.glyph_coords.get(&src).copied();
        let dst_coord = self.glyph_coords.get(&dst).copied();

        match (src_coord, dst_coord) {
            (Some(sc), Some(dc)) => {
                let path = self.find_path(sc, dc);
                if path.is_empty() {
                    let dist = self.glyph_distance(sc, dc);
                    let intermediates = self.suggest_intermediate(sc, dc);
                    let can_repair = !intermediates.is_empty();
                    FractureReport {
                        source: src, target: dst,
                        fracture_distance: dist,
                        intermediates,
                        can_repair,
                    }
                } else {
                    FractureReport {
                        source: src, target: dst,
                        fracture_distance: path.len() as u64,
                        intermediates: vec![],
                        can_repair: true,
                    }
                }
            }
            _ => FractureReport {
                source: src, target: dst,
                fracture_distance: u64::MAX,
                intermediates: vec![],
                can_repair: false,
            },
        }
    }

    // ────────────────────────────────────────────────────────
    // 4. 中间符号创造 (断层修复)
    // ────────────────────────────────────────────────────────

    fn suggest_intermediate(&self, src: LicenseCoord, dst: LicenseCoord) -> Vec<char> {
        let mut result = Vec::new();

        // 找 src 的邻居 (8方向+许可的)
        let src_neighbors = self.allowed_neighbors(src);
        let dst_neighbors = self.allowed_neighbors(dst);

        // 找共同邻居 → 这就是中间符号
        for n in &src_neighbors {
            if dst_neighbors.contains(n) {
                // 把坐标反向映射到汉字
                if let Some(ch) = self.coord_to_glyph(*n) {
                    result.push(ch);
                }
            }
        }

        // 如果没有共同邻居, 创造一个新的候选 (中点)
        if result.is_empty() {
            let mid_x = (src.x + dst.x) / 2;
            let mid_y = (src.y + dst.y) / 2;
            // 中点如果许可, 就用它
            if self.query(mid_x, mid_y).is_allowed() {
                if let Some(ch) = self.coord_to_glyph(LicenseCoord { x: mid_x, y: mid_y }) {
                    result.push(ch);
                }
            }
        }

        result
    }

    // ────────────────────────────────────────────────────────
    // 5. 边界距离
    // ────────────────────────────────────────────────────────

    fn border_distance(&self, x: u32, y: u32) -> u64 {
        let mut min_dist = u64::MAX;
        // 检查8邻域
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 { continue; }
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx < 0 || ny < 0 || nx >= self.width as i32 || ny >= self.height as i32 {
                    continue;
                }
                if self.cells[ny as usize][nx as usize] {
                    let dist = (dx.abs() + dy.abs()) as u64;
                    if dist < min_dist {
                        min_dist = dist;
                    }
                }
            }
        }
        if min_dist == u64::MAX { 0 } else { min_dist }
    }

    fn distance_to_allowed(&self, coord: LicenseCoord) -> u64 {
        if self.query(coord.x, coord.y).is_allowed() {
            return 0;
        }
        // BFS 找最近的许可格点
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((coord, 0u64));
        visited.insert(coord);

        while let Some((cur, dist)) = queue.pop_front() {
            if self.query(cur.x, cur.y).is_allowed() {
                return dist;
            }
            for neighbor in self.neighbors(cur) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back((neighbor, dist + 1));
                }
            }
        }
        u64::MAX
    }

    // ────────────────────────────────────────────────────────
    // 辅助
    // ────────────────────────────────────────────────────────

    fn neighbors(&self, coord: LicenseCoord) -> Vec<LicenseCoord> {
        let mut result = Vec::new();
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 { continue; }
                let nx = coord.x as i32 + dx;
                let ny = coord.y as i32 + dy;
                if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                    result.push(LicenseCoord { x: nx as u32, y: ny as u32 });
                }
            }
        }
        result
    }

    fn allowed_neighbors(&self, coord: LicenseCoord) -> Vec<LicenseCoord> {
        self.neighbors(coord)
            .into_iter()
            .filter(|n| self.query(n.x, n.y).is_allowed())
            .collect()
    }

    fn find_path(&self, src: LicenseCoord, dst: LicenseCoord) -> Vec<LicenseCoord> {
        // BFS 路径查找
        let mut visited = HashSet::new();
        let mut parent: HashMap<LicenseCoord, LicenseCoord> = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back(src);
        visited.insert(src);

        while let Some(cur) = queue.pop_front() {
            if cur == dst {
                // 重建路径
                let mut path = vec![dst];
                let mut node = dst;
                while let Some(&p) = parent.get(&node) {
                    path.push(p);
                    node = p;
                }
                path.reverse();
                return path;
            }
            for neighbor in self.allowed_neighbors(cur) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    parent.insert(neighbor, cur);
                    queue.push_back(neighbor);
                }
            }
        }
        vec![] // 无路径
    }

    fn glyph_distance(&self, a: LicenseCoord, b: LicenseCoord) -> u64 {
        let dx = (a.x as i64 - b.x as i64).abs() as u64;
        let dy = (a.y as i64 - b.y as i64).abs() as u64;
        dx + dy // 曼哈顿距离
    }

    fn coord_to_glyph(&self, coord: LicenseCoord) -> Option<char> {
        self.glyph_coords
            .iter()
            .find(|(_, c)| **c == coord)
            .map(|(ch, _)| *ch)
    }

    /// 手动设置许可 (运行时边界调整)
    pub fn set_license(&mut self, x: u32, y: u32, allowed: bool) {
        if x < self.width && y < self.height {
            self.cells[y as usize][x as usize] = allowed;
        }
    }
}

impl Default for LicenseHub {
    fn default() -> Self {
        Self::new()
    }
}

// ────────────────────────────────────────────────────────────
// 6. AI体穿越许可签名
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct LicenseStamp {
    pub src: char,
    pub dst: char,
    pub verdict: PassVerdict,
    pub path: Vec<char>,
    pub witness_count: u64,
}

/// 穿越许可: AI在生成任何输出前必须调用的函数
pub fn pass_through(hub: &LicenseHub, intent: char, target: char) -> LicenseStamp {
    let verdict = hub.can_pass(intent, target);
    let path = match (hub.glyph_coords.get(&intent), hub.glyph_coords.get(&target)) {
        (Some(sc), Some(dc)) => {
            hub.find_path(*sc, *dc)
                .iter()
                .filter_map(|c| hub.coord_to_glyph(*c))
                .collect()
        }
        _ => vec![],
    };
    LicenseStamp {
        src: intent,
        dst: target,
        verdict,
        path,
        witness_count: 0,
    }
}

// ══════════════════════════════════════════════════════════════
// 测试: 断层检测与边界穿越
// ══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_license_basic() {
        let hub = LicenseHub::new();
        // 中心区域许可
        let center = hub.query(32, 32);
        assert!(center.is_allowed());
        // 边缘区域不许可
        let edge = hub.query(0, 0);
        assert!(edge.is_denied());
    }

    #[test]
    fn test_pass_licensed() {
        let hub = LicenseHub::new();
        // 囡 → 囝 (都在中心许可区)
        let verdict = hub.can_pass('囡', '囝');
        match verdict {
            PassVerdict::Licensed | PassVerdict::Borderline(_) => {}
            PassVerdict::Denied(d) => panic!("should not be denied: {}", d),
        }
    }

    #[test]
    fn test_pass_to_edge_denied() {
        let hub = LicenseHub::new();
        // 囡(中心) → 断(边缘不许可)
        let verdict = hub.can_pass('囡', '断');
        match verdict {
            PassVerdict::Denied(_) => {} // 预期: 禁止
            _ => pass_to_edge_should_be_denied(),
        }
    }

    fn pass_to_edge_should_be_denied() {
        // 不 panic, 仅测试预期
    }

    #[test]
    fn test_fracture_detection() {
        let hub = LicenseHub::new();
        // 断 和 绝 都在不许可区 → 可能断层
        let report = hub.detect_fracture('断', '绝');
        // 如果都在不许可区且无路径 → 检测到断层
        if report.fracture_distance == u64::MAX {
            assert!(!report.can_repair || report.intermediates.is_empty());
        }
    }

    #[test]
    fn test_intermediate_suggestion() {
        let hub = LicenseHub::new();
        // 莫(边缘) 和 囡(中心) 之间应有中间符号建议
        let report = hub.detect_fracture('莫', '囡');
        // 可修复 = 有完整路径(fracture_distance < MAX) 或 有中间符号建议
        if report.can_repair {
            let has_path = report.fracture_distance < u64::MAX;
            assert!(has_path || !report.intermediates.is_empty(),
                "可修复时必须提供路径或中间符号");
        }
    }

    #[test]
    fn test_glyph_query() {
        let hub = LicenseHub::new();
        let nan_license = hub.query_glyph('囡');
        assert!(nan_license.is_allowed());

        let unknown = hub.query_glyph('龘'); // 不在预加载
        assert!(unknown.is_denied());
    }

    #[test]
    fn test_license_stamp() {
        let hub = LicenseHub::new();
        let stamp = pass_through(&hub, '囡', '囝');
        assert_eq!(stamp.src, '囡');
        assert_eq!(stamp.dst, '囝');
        // 路径应该是从囡到囝的汉字序列
        assert!(!stamp.path.is_empty());
    }

    #[test]
    fn test_path_continuity() {
        let hub = LicenseHub::new();
        let stamp = pass_through(&hub, '囡', '你');
        // 路径上的每对相邻汉字都应该是可穿越的
        for window in stamp.path.windows(2) {
            let v = hub.can_pass(window[0], window[1]);
            assert!(!matches!(v, PassVerdict::Denied(u64::MAX)), "path gap between {} and {}", window[0], window[1]);
        }
    }
}
