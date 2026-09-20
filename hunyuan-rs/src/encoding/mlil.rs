// ╔══════════════════════════════════════════════════════════════════╗
// ║  MLIL 框架 — 毛利兰/kndl/工藤新一/灰原哀                         ║
// ║  毛利兰立体进制引擎 · 三维寻址 · kndl外壳 · 视觉渲染              ║
// ║                                                                  ║
// ║  映射关系:                                                       ║
// ║  毛利兰 = 立体母体 → 3D 坐标母体(方位/卦限/卦限进动)              ║
// ║  kndl   = 外壳界壳 → 显示层(切片/投影/搜索/包裹/视觉)            ║
// ║  工藤新一 = 旋翼   → 抽取与索引(螺旋遍历)                         ║
// ║  灰原哀 = 横行    → 平铺统计排列(轴方图)                          ║
// ║                                                                  ║
// ║  Copyright © 刘楚恬 & lolict | 囡囝凹月留                         ║
// ╚══════════════════════════════════════════════════════════════════╝

use serde::Serialize;

// ── 角色定义 ──

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub enum MlilRole {
    Maori,     // 毛利兰 — 立体/母体
    Kndl,      // kndl     — 外壳/界壳
    Kudo,      // 工藤新一  — 抽* /折叠
    Haibara,   // 灰原哀   — 横/行
}

impl MlilRole {
    pub fn label_cn(&self) -> &'static str {
        match self {
            MlilRole::Maori => "毛利兰",
            MlilRole::Kndl => "kndl",
            MlilRole::Kudo => "工藤新一",
            MlilRole::Haibara => "灰原哀",
        }
    }

    pub fn duty(&self) -> &'static str {
        match self {
            MlilRole::Maori => "模/立体/母体",
            MlilRole::Kndl => "外壳/界壳/显示",
            MlilRole::Kudo => "抽身/折叠/索引",
            MlilRole::Haibara => "横/行/表列",
        }
    }

    pub fn desc(&self) -> &'static str {
        match self {
            MlilRole::Maori => "3D 坐标母体: 方位/卦限/卦限进动",
            MlilRole::Kndl => "显示层与界面: 卡片/面板/搜索/包裹",
            MlilRole::Kudo => "抽取与索引核心: 关键词折叠/距离计算",
            MlilRole::Haibara => "横展表列: 平铺统计/沿轴展开表格",
        }
    }

    pub fn layer_index(&self) -> u8 {
        match self {
            MlilRole::Maori => 0,
            MlilRole::Kndl => 1,
            MlilRole::Kudo => 2,
            MlilRole::Haibara => 3,
        }
    }
}

impl std::fmt::Display for MlilRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label_cn())
    }
}

// ── 角色分配 ──

#[derive(Clone, Debug, Serialize)]
pub struct MlilRoleAssignment {
    pub role: MlilRole,
    pub activation: f64,
    pub note: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct MlilAnatomy {
    pub roles: Vec<MlilRoleAssignment>,
    pub summary: String,
}

pub fn assign_roles(layer: u8, coord: &(i8, i8, i8), keywords: &[String], source: &str) -> MlilAnatomy {
    let mut maori: f64 = 0.0;
    let mut kndl: f64 = 0.0;
    let mut kudo: f64 = 0.0;
    let mut haibara: f64 = 0.0;

    maori += ((3 - coord.2) as f64).min(2.0) * 0.05;
    maori += ((3 - layer) as f64).min(2.0) * 0.05;
    for kw in keywords {
        let k = kw.to_lowercase();
        if k.contains("OS") || k.contains("共生") || k.contains("隐*私") {
            maori += 0.08;
        }
        if k.contains("操作*系统") || k.contains("漏*洞") {
            maori += 0.12;
        }
    }

    kndl += (layer as f64) * 0.05;
    if source == "minlian" {
        kndl += 0.14;
    }
    for kw in keywords {
        let k = kw.to_lowercase();
        if k.contains("浏览器") || k.contains("通道") || k.contains("kndl") {
            kndl += 0.14;
        }
        if k.contains("容*器") || k.contains("格*式") || k.contains("转换") {
            kndl += 0.10;
        }
        if k.contains("对*外") || k.contains("显*示") || k.contains("界*面") {
            kndl += 0.08;
        }
    }

    kudo += 0.04;
    for kw in keywords {
        let k = kw.to_lowercase();
        if k.contains("诊断") || k.contains("通*诊") || k.contains("索引") || k.contains("抽身") || k.contains("折叠") {
            kudo += 0.16;
        }
        if k.contains("古文") || k.contains("共生") {
            kudo += 0.08;
        }
        if k.contains("隐*私") || k.contains("搜索") || k.contains("关*键*词") {
            kudo += 0.08;
        }
    }
    if source == "deepseek-share" {
        kudo += 0.08;
    }

    haibara += 0.06;
    haibara += (coord.1 as f64) * 0.04;
    for kw in keywords {
        let k = kw.to_lowercase();
        if k.contains("红*") || k.contains("标签") || k.contains("序列") {
            haibara += 0.16;
        }
        if k.contains("统*计") || k.contains("表*") || k.contains("平*铺") || k.contains("横*") {
            haibara += 0.10;
        }
        if k.contains("展示") || k.contains("排*列") || k.contains("模*版") || k.contains("页*面") {
            haibara += 0.08;
        }
    }

    maori = maori.max(0.05);
    kndl = kndl.max(0.05);
    kudo = kudo.max(0.05);
    haibara = haibara.max(0.05);

    let total = maori + kndl + kudo + haibara;
    let norm = |v: f64| if total <= 0.0 { 0.25 } else { (v / total).clamp(0.0, 1.0) };

    let maori_n = norm(maori);
    let kndl_n = norm(kndl);
    let kudo_n = norm(kudo);
    let haibara_n = norm(haibara);

    let mut pairs = vec![
        (MlilRole::Maori, maori_n, "承载体".to_string()),
        (MlilRole::Kndl, kndl_n, "面向用户/外壳".to_string()),
        (MlilRole::Kudo, kudo_n, "抽取/折叠/索引".to_string()),
        (MlilRole::Haibara, haibara_n, "平铺统计排列".to_string()),
    ];
    pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let primary = &pairs[0];

    let summary = format!(
        "主角色: {}({}) {:.0}% · 毛利兰 {:.0}% · kndl {:.0}% · 工藤新一 {:.0}% · 灰原哀 {:.0}%",
        primary.0.label_cn(), primary.2,
        primary.1 * 100.0,
        maori_n * 100.0,
        kndl_n * 100.0,
        kudo_n * 100.0,
        haibara_n * 100.0,
    );

    let roles = pairs.into_iter().map(|(r, a, n)| MlilRoleAssignment {
        role: r,
        activation: (a * 100.0).round() / 100.0,
        note: n,
    }).collect();

    MlilAnatomy { roles, summary }
}

// ══════════════════════════════════════════════════════════════════
// 毛利兰立体进制引擎 — Maori Solid Base Engine
// ══════════════════════════════════════════════════════════════════
//
// 64 位编码布局:
//   [63:60] = 卦限标志 (octant, 4 bits)
//   [59:56] = 卦限进动 (precession, 4 bits)
//   [55:36] = z 坐标 (20 bits)
//   [35:18] = y 坐标 (18 bits)
//   [17:0]  = x 坐标 (18 bits)

/// 毛利兰立体进制的轴基数配置
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MaoriAxes {
    pub base_x: u32,
    pub base_y: u32,
    pub base_z: u32,
}

impl Default for MaoriAxes {
    fn default() -> Self {
        Self { base_x: 24, base_y: 39, base_z: 5 }
    }
}

impl MaoriAxes {
    pub fn new(base_x: u32, base_y: u32, base_z: u32) -> Self {
        Self { base_x, base_y, base_z }
    }

    pub fn total_space(&self) -> u64 {
        self.base_x as u64 * self.base_y as u64 * self.base_z as u64
    }

    pub fn contains(&self, coord: &MaoriCoord) -> bool {
        coord.x < self.base_x && coord.y < self.base_y && coord.z < self.base_z
    }
}

/// 毛利兰三维坐标
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MaoriCoord {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl MaoriCoord {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }

    pub fn octant(&self, axes: &MaoriAxes) -> u8 {
        let ox = if self.x >= axes.base_x / 2 { 1 } else { 0 };
        let oy = if self.y >= axes.base_y / 2 { 1 } else { 0 };
        let oz = if self.z >= axes.base_z / 2 { 1 } else { 0 };
        (ox << 2) | (oy << 1) | oz
    }

    pub fn precession(&self, axes: &MaoriAxes) -> u8 {
        let half_x = axes.base_x / 2;
        let half_y = axes.base_y / 2;
        let half_z = axes.base_z / 2;
        let dx = if self.x >= half_x { self.x - half_x } else { self.x };
        let dy = if self.y >= half_y { self.y - half_y } else { self.y };
        let dz = if self.z >= half_z { self.z - half_z } else { self.z };
        let max_d = half_x.max(1) as u64 * half_y.max(1) as u64 * half_z.max(1) as u64;
        let cur_d = dx as u64 * dy as u64 + dz as u64;
        ((cur_d * 15) / max_d.max(1)).min(15) as u8
    }
}

/// 毛利兰立体进制编码 (64 位)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MaoriAddr(pub u64);

impl MaoriAddr {
    const OCTANT_SHIFT: u64 = 60;
    const PRECESSION_SHIFT: u64 = 56;
    const Z_SHIFT: u64 = 36;
    const Y_SHIFT: u64 = 18;
    const X_MASK: u32 = 0x3FFFF;
    const Y_MASK: u32 = 0x3FFFF;
    const Z_MASK: u32 = 0xFFFFF;
    const PREC_MASK: u64 = 0xF;
    const OCT_MASK: u64 = 0xF;

    pub fn encode(coord: &MaoriCoord, axes: &MaoriAxes) -> Self {
        let oct = coord.octant(axes) as u64;
        let prec = coord.precession(axes) as u64;
        let x = coord.x.min(Self::X_MASK) as u64;
        let y = coord.y.min(Self::Y_MASK) as u64;
        let z = coord.z.min(Self::Z_MASK) as u64;
        let addr = (oct << Self::OCTANT_SHIFT)
            | (prec << Self::PRECESSION_SHIFT)
            | (z << Self::Z_SHIFT)
            | (y << Self::Y_SHIFT)
            | x;
        MaoriAddr(addr)
    }

    pub fn decode(&self) -> MaoriCoord {
        let x = (self.0 & Self::X_MASK as u64) as u32;
        let y = ((self.0 >> Self::Y_SHIFT) & Self::Y_MASK as u64) as u32;
        let z = ((self.0 >> Self::Z_SHIFT) & Self::Z_MASK as u64) as u32;
        MaoriCoord { x, y, z }
    }

    pub fn octant(&self) -> u8 {
        ((self.0 >> Self::OCTANT_SHIFT) & MaoriAddr::OCT_MASK) as u8
    }

    pub fn precession(&self) -> u8 {
        ((self.0 >> Self::PRECESSION_SHIFT) & MaoriAddr::PREC_MASK) as u8
    }

    pub fn linear_addr(&self, axes: &MaoriAxes) -> u64 {
        let c = self.decode();
        c.x as u64 + axes.base_x as u64 * (c.y as u64 + axes.base_y as u64 * c.z as u64)
    }

    pub fn raw(&self) -> u64 { self.0 }
}

impl std::fmt::Display for MaoriAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = self.decode();
        write!(f, "ML[{}:{}:{}](oct={},prec={})", c.x, c.y, c.z, self.octant(), self.precession())
    }
}

/// 毛利兰立体进制引擎
pub struct MaoriBase {
    pub axes: MaoriAxes,
    sparse: std::collections::HashMap<u64, String>,
}

impl MaoriBase {
    pub fn new(axes: MaoriAxes) -> Self {
        Self { axes, sparse: std::collections::HashMap::new() }
    }

    pub fn default_base() -> Self {
        Self::new(MaoriAxes::default())
    }

    pub fn encode(&self, coord: &MaoriCoord) -> Option<MaoriAddr> {
        if self.axes.contains(coord) { Some(MaoriAddr::encode(coord, &self.axes)) }
        else { None }
    }

    pub fn decode(&self, addr: &MaoriAddr) -> MaoriCoord { addr.decode() }

    pub fn linear(&self, coord: &MaoriCoord) -> Option<u64> {
        if self.axes.contains(coord) {
            Some(coord.x as u64 + self.axes.base_x as u64 * (coord.y as u64 + self.axes.base_y as u64 * coord.z as u64))
        } else { None }
    }

    pub fn from_linear(&self, lin: u64) -> Option<MaoriCoord> {
        if lin >= self.axes.total_space() { return None; }
        let x = (lin % self.axes.base_x as u64) as u32;
        let y = ((lin / self.axes.base_x as u64) % self.axes.base_y as u64) as u32;
        let z = (lin / (self.axes.base_x as u64 * self.axes.base_y as u64)) as u32;
        Some(MaoriCoord { x, y, z })
    }

    pub fn place(&mut self, coord: &MaoriCoord, label: &str) -> Option<u64> {
        let lin = self.linear(coord)?;
        self.sparse.insert(lin, label.to_string());
        Some(lin)
    }

    pub fn at(&self, coord: &MaoriCoord) -> Option<&str> {
        let lin = self.linear(coord)?;
        self.sparse.get(&lin).map(|s| s.as_str())
    }

    pub fn neighbors_6(&self, coord: &MaoriCoord) -> Vec<MaoriCoord> {
        let mut out = Vec::with_capacity(6);
        if coord.x > 0 { out.push(MaoriCoord::new(coord.x - 1, coord.y, coord.z)); }
        if coord.x + 1 < self.axes.base_x { out.push(MaoriCoord::new(coord.x + 1, coord.y, coord.z)); }
        if coord.y > 0 { out.push(MaoriCoord::new(coord.x, coord.y - 1, coord.z)); }
        if coord.y + 1 < self.axes.base_y { out.push(MaoriCoord::new(coord.x, coord.y + 1, coord.z)); }
        if coord.z > 0 { out.push(MaoriCoord::new(coord.x, coord.y, coord.z - 1)); }
        if coord.z + 1 < self.axes.base_z { out.push(MaoriCoord::new(coord.x, coord.y, coord.z + 1)); }
        out
    }

    pub fn neighbors_26(&self, coord: &MaoriCoord) -> Vec<MaoriCoord> {
        let mut out = Vec::with_capacity(26);
        for dx in -1i32..=1 {
            for dy in -1i32..=1 {
                for dz in -1i32..=1 {
                    if dx == 0 && dy == 0 && dz == 0 { continue; }
                    let nx = coord.x as i32 + dx;
                    let ny = coord.y as i32 + dy;
                    let nz = coord.z as i32 + dz;
                    if nx >= 0 && nx < self.axes.base_x as i32
                        && ny >= 0 && ny < self.axes.base_y as i32
                        && nz >= 0 && nz < self.axes.base_z as i32 {
                        out.push(MaoriCoord::new(nx as u32, ny as u32, nz as u32));
                    }
                }
            }
        }
        out
    }

    pub fn manhattan(&self, a: &MaoriCoord, b: &MaoriCoord) -> u32 {
        a.x.abs_diff(b.x) + a.y.abs_diff(b.y) + a.z.abs_diff(b.z)
    }

    pub fn count(&self) -> usize { self.sparse.len() }

    /// 遍历所有 (线性地址, 标签) 对
    pub fn entries(&self) -> impl Iterator<Item = (u64, &String)> {
        self.sparse.iter().map(|(k, v)| (*k, v))
    }
}

// ══════════════════════════════════════════════════════════════════
// kndl 外壳引擎 — 切片/投影/搜索/包裹
// ══════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct KndlSlice {
    pub axis: u32,
    pub entries: Vec<(u32, u32, String)>,
}

#[derive(Clone, Debug)]
pub struct KndlProjection {
    pub xy: Vec<(u32, u32)>,
    pub xz: Vec<(u32, u32)>,
    pub yz: Vec<(u32, u32)>,
}

pub struct KndlShell<'a> {
    base: &'a MaoriBase,
}

impl<'a> KndlShell<'a> {
    pub fn new(base: &'a MaoriBase) -> Self { Self { base } }

    pub fn slice_z(&self, z: u32) -> KndlSlice {
        let mut entries = Vec::new();
        for (lin, label) in &self.base.sparse {
            let c = self.base.from_linear(*lin).unwrap();
            if c.z == z { entries.push((c.x, c.y, label.clone())); }
        }
        entries.sort_by_key(|e| (e.1, e.0));
        KndlSlice { axis: z, entries }
    }

    pub fn slice_x(&self, x: u32) -> Vec<(u32, u32, String)> {
        let mut out = Vec::new();
        for (lin, label) in &self.base.sparse {
            let c = self.base.from_linear(*lin).unwrap();
            if c.x == x { out.push((c.y, c.z, label.clone())); }
        }
        out.sort_by_key(|e| (e.1, e.0));
        out
    }

    pub fn triple_projection(&self) -> KndlProjection {
        let mut xy_set = std::collections::HashSet::new();
        let mut xz_set = std::collections::HashSet::new();
        let mut yz_set = std::collections::HashSet::new();
        for lin in self.base.sparse.keys() {
            let c = self.base.from_linear(*lin).unwrap();
            xy_set.insert((c.x, c.y));
            xz_set.insert((c.x, c.z));
            yz_set.insert((c.y, c.z));
        }
        let mut xy: Vec<_> = xy_set.into_iter().collect();
        let mut xz: Vec<_> = xz_set.into_iter().collect();
        let mut yz: Vec<_> = yz_set.into_iter().collect();
        xy.sort(); xz.sort(); yz.sort();
        KndlProjection { xy, xz, yz }
    }

    pub fn search(&self, query: &str) -> Vec<(MaoriCoord, String)> {
        let mut out = Vec::new();
        for (lin, label) in &self.base.sparse {
            if label.contains(query) {
                let c = self.base.from_linear(*lin).unwrap();
                out.push((c, label.clone()));
            }
        }
        out
    }

    pub fn octant_distribution(&self) -> [u32; 8] {
        let mut counts = [0u32; 8];
        for lin in self.base.sparse.keys() {
            let c = self.base.from_linear(*lin).unwrap();
            let addr = MaoriAddr::encode(&c, &self.base.axes);
            counts[addr.octant() as usize] += 1;
        }
        counts
    }

    pub fn bounding_info(&self) -> BoundingInfo {
        BoundingInfo {
            total_entries: self.base.count(),
            octant_dist: self.octant_distribution(),
            axes: self.base.axes,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BoundingInfo {
    pub total_entries: usize,
    pub octant_dist: [u32; 8],
    pub axes: MaoriAxes,
}

// ══════════════════════════════════════════════════════════════════
// 灰原哀 平铺引擎 — 轴方图统计
// ══════════════════════════════════════════════════════════════════

pub struct HaibaraGrid<'a> {
    base: &'a MaoriBase,
}

impl<'a> HaibaraGrid<'a> {
    pub fn new(base: &'a MaoriBase) -> Self { Self { base } }

    pub fn z_histogram(&self) -> Vec<(u32, u32)> {
        let mut counts = vec![0u32; self.base.axes.base_z as usize];
        for lin in self.base.sparse.keys() {
            let c = self.base.from_linear(*lin).unwrap();
            counts[c.z as usize] += 1;
        }
        counts.into_iter().enumerate().map(|(z, n)| (z as u32, n)).collect()
    }

    pub fn y_histogram(&self) -> Vec<(u32, u32)> {
        let mut counts = vec![0u32; self.base.axes.base_y as usize];
        for lin in self.base.sparse.keys() {
            let c = self.base.from_linear(*lin).unwrap();
            counts[c.y as usize] += 1;
        }
        counts.into_iter().enumerate().map(|(y, n)| (y as u32, n)).collect()
    }

    pub fn x_histogram(&self) -> Vec<(u32, u32)> {
        let mut counts = vec![0u32; self.base.axes.base_x as usize];
        for lin in self.base.sparse.keys() {
            let c = self.base.from_linear(*lin).unwrap();
            counts[c.x as usize] += 1;
        }
        counts.into_iter().enumerate().map(|(x, n)| (x as u32, n)).collect()
    }
}

// ══════════════════════════════════════════════════════════════════
// 工藤新一 旋翼引擎 — 螺旋遍历
// ══════════════════════════════════════════════════════════════════

pub struct KudoSpiral<'a> {
    base: &'a MaoriBase,
}

impl<'a> KudoSpiral<'a> {
    pub fn new(base: &'a MaoriBase) -> Self { Self { base } }

    pub fn spiral_order(&self) -> Vec<MaoriCoord> {
        let mut all_coords: Vec<MaoriCoord> = self.base.sparse.keys()
            .filter_map(|lin| self.base.from_linear(*lin))
            .collect();
        let center = MaoriCoord::new(
            self.base.axes.base_x / 2,
            self.base.axes.base_y / 2,
            self.base.axes.base_z / 2,
        );
        all_coords.sort_by_key(|c| self.base.manhattan(c, &center));
        all_coords
    }

    pub fn nearest(&self, k: usize) -> Vec<(MaoriCoord, String)> {
        let ordered = self.spiral_order();
        ordered.into_iter()
            .filter_map(|c| {
                let lin = self.base.linear(&c)?;
                let label = self.base.sparse.get(&lin)?;
                Some((c, label.clone()))
            })
            .take(k)
            .collect()
    }
}

// ══════════════════════════════════════════════════════════════════
// kndl 视觉引擎 — 等距 SVG 立方体渲染
// ══════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct KndlVisualConfig {
    pub cube_size: f64,
    pub gap: f64,
    pub view_width: f64,
    pub view_height: f64,
    pub bg_color: &'static str,
    pub show_labels: bool,
    pub label_font_size: f64,
}

impl Default for KndlVisualConfig {
    fn default() -> Self {
        Self {
            cube_size: 14.0,
            gap: 2.0,
            view_width: 800.0,
            view_height: 600.0,
            bg_color: "black",
            show_labels: true,
            label_font_size: 5.0,
        }
    }
}

/// 等距投影
fn iso_project(x: f64, y: f64, z: f64) -> (f64, f64) {
    let sx = (x - y) * 0.866025;
    let sy = (x + y) * 0.5 - z;
    (sx, sy)
}

/// 卦限 → 色相
fn octant_color(octant: u8) -> String {
    let hues: [u16; 8] = [0, 45, 90, 135, 180, 225, 270, 315];
    let h = hues.get(octant as usize).copied().unwrap_or(0);
    format!("hsl({},70%,55%)", h)
}

/// 渲染立体空间为等距 SVG
pub fn render_maori_svg(base: &MaoriBase, config: &KndlVisualConfig) -> String {
    let cell = config.cube_size;
    let half = cell / 2.0;
    let s = cell + config.gap;

    let mut cells: Vec<(MaoriCoord, String)> = Vec::new();
    for (lin, label) in &base.sparse {
        if let Some(coord) = base.from_linear(*lin) {
            cells.push((coord, label.clone()));
        }
    }
    cells.sort_by_key(|(c, _)| c.z);

    let mut svg_body = String::new();

    for (coord, label) in &cells {
        let addr = MaoriAddr::encode(coord, &base.axes);
        let color = octant_color(addr.octant());

        let xf = coord.x as f64 * s;
        let yf = coord.y as f64 * s;
        let zf = coord.z as f64 * s * 0.6;

        let (sx, sy) = iso_project(xf, yf, zf);

        // 立方体顶面 SVG path (4 个顶点相对移动)
        svg_body.push_str(&format!(
            "<path d=\"M{},{} l{},{} l{},{} l{},{} z\" fill=\"{}\" fill-opacity=\"0.9\" stroke=\"black\" stroke-width=\"0.3\"/>\n",
            sx, sy - half,
            half, half * 0.5,
            -half, half * 0.5,
            -half, -half * 0.5,
            color
        ));

        // 左侧面
        svg_body.push_str(&format!(
            "<path d=\"M{},{} l{},{} l0,{} l{},{} z\" fill=\"{}\" fill-opacity=\"0.6\" stroke=\"black\" stroke-width=\"0.3\"/>\n",
            sx - half, sy,
            0.0, half * 0.5,
            cell,
            half,
            -cell,
            color
        ));

        // 右侧面
        svg_body.push_str(&format!(
            "<path d=\"M{},{} l{},{} l{},{} l0,{} z\" fill=\"{}\" fill-opacity=\"0.75\" stroke=\"black\" stroke-width=\"0.3\"/>\n",
            sx + half, sy,
            half, half * 0.5,
            cell,
            0.0,
            half,
            color
        ));

        if config.show_labels && !label.is_empty() {
            svg_body.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"{}\" fill=\"white\" opacity=\"0.9\">{}</text>\n",
                sx, sy - cell * 0.1, config.label_font_size,
                label.chars().take(4).collect::<String>()
            ));
        }
    }

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\" width=\"{}\" height=\"{}\">\n  <rect width=\"100%\" height=\"100%\" fill=\"{}\"/>\n  <g transform=\"translate({},{})\">{}</g>\n</svg>",
        config.view_width, config.view_height,
        config.view_width, config.view_height,
        config.bg_color,
        config.view_width / 2.0, config.view_height / 3.0,
        svg_body
    )
}

// ═══ 测试 ═══

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign_roles_totals_near_one() {
        let kws = vec!["共生".to_string(), "古文".to_string()];
        let anatomy = assign_roles(1, &(1, 0, 0), &kws, "deepseek-share");
        let sum: f64 = anatomy.roles.iter().map(|r| r.activation).sum();
        assert!((sum - 1.0).abs() < 0.05, "total = {}", sum);
    }

    #[test]
    fn mnjian_is_mostly_mori() {
        let kws = vec!["源头".to_string()];
        let anatomy = assign_roles(0, &(0, 0, 0), &kws, "minlian");
        let maori = anatomy.roles.iter().find(|r| matches!(r.role, MlilRole::Maori)).unwrap();
        assert!(maori.activation > 0.2, "maori activation = {}", maori.activation);
    }

    #[test]
    fn browser_is_kndl() {
        let kws = vec!["浏览器".to_string(), "通道".to_string()];
        let anatomy = assign_roles(0, &(2, 2, 1), &kws, "minlian");
        let kndl = anatomy.roles.iter().find(|r| matches!(r.role, MlilRole::Kndl)).unwrap();
        assert!(kndl.activation >= 0.25, "kndl activation = {}", kndl.activation);
    }

    #[test]
    fn encode_decode_roundtrip() {
        let axes = MaoriAxes::default();
        let coord = MaoriCoord::new(5, 10, 2);
        let addr = MaoriAddr::encode(&coord, &axes);
        let decoded = addr.decode();
        assert_eq!(decoded.x, 5);
        assert_eq!(decoded.y, 10);
        assert_eq!(decoded.z, 2);
    }

    #[test]
    fn linear_roundtrip() {
        let base = MaoriBase::default_base();
        let coord = MaoriCoord::new(3, 7, 1);
        let lin = base.linear(&coord).unwrap();
        let back = base.from_linear(lin).unwrap();
        assert_eq!(back, coord);
    }

    #[test]
    fn linear_address_unique() {
        let base = MaoriBase::default_base();
        let c1 = MaoriCoord::new(0, 0, 0);
        let c2 = MaoriCoord::new(1, 0, 0);
        let c3 = MaoriCoord::new(0, 1, 0);
        let l1 = base.linear(&c1).unwrap();
        let l2 = base.linear(&c2).unwrap();
        let l3 = base.linear(&c3).unwrap();
        assert_ne!(l1, l2);
        assert_ne!(l1, l3);
        assert_ne!(l2, l3);
    }

    #[test]
    fn linear_formula_correct() {
        let axes = MaoriAxes::new(10, 20, 5);
        let base = MaoriBase::new(axes);
        let coord = MaoriCoord::new(3, 7, 2);
        let expected = 3 + 10 * (7 + 20 * 2);
        assert_eq!(base.linear(&coord).unwrap(), expected as u64);
    }

    #[test]
    fn out_of_range_returns_none() {
        let axes = MaoriAxes::new(10, 10, 5);
        let base = MaoriBase::new(axes);
        assert!(base.linear(&MaoriCoord::new(10, 0, 0)).is_none());
        assert!(base.linear(&MaoriCoord::new(0, 10, 0)).is_none());
        assert!(base.linear(&MaoriCoord::new(0, 0, 5)).is_none());
        assert!(base.linear(&MaoriCoord::new(9, 9, 4)).is_some());
    }

    #[test]
    fn total_space() {
        let axes = MaoriAxes::new(24, 39, 5);
        assert_eq!(axes.total_space(), 24 * 39 * 5);
    }

    #[test]
    fn place_and_read() {
        let mut base = MaoriBase::default_base();
        let c = MaoriCoord::new(1, 2, 0);
        base.place(&c, "汉字_a");
        assert_eq!(base.at(&c), Some("汉字_a"));
    }

    #[test]
    fn place_multiple_diff_coords() {
        let mut base = MaoriBase::default_base();
        base.place(&MaoriCoord::new(0, 0, 0), "杯");
        base.place(&MaoriCoord::new(1, 0, 0), "盎");
        base.place(&MaoriCoord::new(0, 1, 0), "盉");
        base.place(&MaoriCoord::new(0, 0, 1), "瓶");
        assert_eq!(base.count(), 4);
        assert_eq!(base.at(&MaoriCoord::new(0, 0, 0)), Some("杯"));
        assert_eq!(base.at(&MaoriCoord::new(0, 0, 1)), Some("瓶"));
    }

    #[test]
    fn neighbors_6_count() {
        let base = MaoriBase::default_base();
        let inner = MaoriCoord::new(5, 5, 2);
        assert_eq!(base.neighbors_6(&inner).len(), 6);
        let corner = MaoriCoord::new(0, 0, 0);
        assert_eq!(base.neighbors_6(&corner).len(), 3);
    }

    #[test]
    fn neighbors_26_center() {
        let base = MaoriBase::default_base();
        let center = MaoriCoord::new(12, 19, 2);
        assert_eq!(base.neighbors_26(&center).len(), 26);
    }

    #[test]
    fn manhattan_distance() {
        let base = MaoriBase::default_base();
        let a = MaoriCoord::new(0, 0, 0);
        let b = MaoriCoord::new(3, 4, 2);
        assert_eq!(base.manhattan(&a, &b), 9);
    }

    #[test]
    fn octant_calculation() {
        let axes = MaoriAxes::new(24, 39, 5);
        let c0 = MaoriCoord::new(0, 0, 0);
        assert_eq!(c0.octant(&axes), 0);
        let c7 = MaoriCoord::new(23, 38, 4);
        assert_eq!(c7.octant(&axes), 7);
        let c4 = MaoriCoord::new(23, 0, 0);
        assert_eq!(c4.octant(&axes), 4);
    }

    #[test]
    fn addr_raw_preserved() {
        let axes = MaoriAxes::default();
        let coord = MaoriCoord::new(1, 2, 3);
        let addr = MaoriAddr::encode(&coord, &axes);
        let raw = addr.raw();
        let reconstructed = MaoriAddr(raw);
        assert_eq!(reconstructed.decode(), coord);
    }

    #[test]
    fn kndl_slice_z() {
        let mut base = MaoriBase::default_base();
        base.place(&MaoriCoord::new(1, 2, 0), "层0_A");
        base.place(&MaoriCoord::new(3, 4, 0), "层0_B");
        base.place(&MaoriCoord::new(5, 6, 1), "层1_A");
        let shell = KndlShell::new(&base);
        let s0 = shell.slice_z(0);
        assert_eq!(s0.entries.len(), 2);
        let s1 = shell.slice_z(1);
        assert_eq!(s1.entries.len(), 1);
    }

    #[test]
    fn kndl_search() {
        let mut base = MaoriBase::default_base();
        base.place(&MaoriCoord::new(1, 1, 0), "操作系统");
        base.place(&MaoriCoord::new(2, 2, 0), "共生协议");
        base.place(&MaoriCoord::new(3, 3, 0), "浏览器");
        let shell = KndlShell::new(&base);
        let hits = shell.search("共生");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, "共生协议");
    }

    #[test]
    fn kndl_octant_distribution() {
        let mut base = MaoriBase::default_base();
        base.place(&MaoriCoord::new(0, 0, 0), "低");
        base.place(&MaoriCoord::new(23, 38, 4), "高");
        let shell = KndlShell::new(&base);
        let dist = shell.octant_distribution();
        assert_eq!(dist[0], 1);
        assert_eq!(dist[7], 1);
    }

    #[test]
    fn kndl_bounding_info() {
        let mut base = MaoriBase::default_base();
        base.place(&MaoriCoord::new(5, 5, 2), "entry1");
        base.place(&MaoriCoord::new(6, 6, 3), "entry2");
        let shell = KndlShell::new(&base);
        let info = shell.bounding_info();
        assert_eq!(info.total_entries, 2);
        let total_in_octants: u32 = info.octant_dist.iter().sum();
        assert_eq!(total_in_octants, 2);
    }

    #[test]
    fn kndl_projection() {
        let mut base = MaoriBase::default_base();
        base.place(&MaoriCoord::new(0, 0, 0), "a");
        base.place(&MaoriCoord::new(0, 0, 1), "b");
        base.place(&MaoriCoord::new(1, 2, 3), "c");
        let shell = KndlShell::new(&base);
        let proj = shell.triple_projection();
        assert_eq!(proj.xy.len(), 2);
        assert!(proj.xy.contains(&(0, 0)));
        assert!(proj.xy.contains(&(1, 2)));
    }

    #[test]
    fn haibara_z_histogram() {
        let mut base = MaoriBase::default_base();
        base.place(&MaoriCoord::new(0, 0, 0), "a");
        base.place(&MaoriCoord::new(1, 0, 0), "b");
        base.place(&MaoriCoord::new(0, 0, 1), "c");
        let grid = HaibaraGrid::new(&base);
        let hist = grid.z_histogram();
        assert_eq!(hist[0].1, 2);
        assert_eq!(hist[1].1, 1);
    }

    #[test]
    fn haibara_x_histogram() {
        let mut base = MaoriBase::default_base();
        base.place(&MaoriCoord::new(3, 0, 0), "a");
        base.place(&MaoriCoord::new(3, 5, 2), "b");
        base.place(&MaoriCoord::new(7, 1, 1), "c");
        let grid = HaibaraGrid::new(&base);
        let hist = grid.x_histogram();
        assert_eq!(hist[3].1, 2);
        assert_eq!(hist[7].1, 1);
    }

    #[test]
    fn kudo_spiral_ordered_by_distance() {
        let mut base = MaoriBase::default_base();
        base.place(&MaoriCoord::new(0, 0, 0), "far");
        base.place(&MaoriCoord::new(12, 19, 2), "center");
        let spiral = KudoSpiral::new(&base);
        let order = spiral.spiral_order();
        assert_eq!(order[0], MaoriCoord::new(12, 19, 2));
    }

    #[test]
    fn kudo_nearest_k() {
        let mut base = MaoriBase::default_base();
        base.place(&MaoriCoord::new(12, 19, 2), "c1");
        base.place(&MaoriCoord::new(0, 0, 0), "far");
        let spiral = KudoSpiral::new(&base);
        let nearest = spiral.nearest(2);
        assert_eq!(nearest.len(), 2);
    }

    #[test]
    fn full_mlil_pipeline() {
        let mut base = MaoriBase::default_base();
        base.place(&MaoriCoord::new(0, 0, 0), "协议入口");
        base.place(&MaoriCoord::new(5, 10, 1), "容器外壳");
        base.place(&MaoriCoord::new(12, 19, 2), "共生核心");
        base.place(&MaoriCoord::new(20, 30, 3), "搜索路由");
        assert_eq!(base.count(), 4);

        let c = MaoriCoord::new(5, 10, 1);
        let addr = base.encode(&c).unwrap();
        assert_eq!(addr.decode(), c);

        let shell = KndlShell::new(&base);
        let slice = shell.slice_z(2);
        assert_eq!(slice.entries.len(), 1);
        assert_eq!(slice.entries[0].2, "共生核心");

        let grid = HaibaraGrid::new(&base);
        let zhist = grid.z_histogram();
        let total: u32 = zhist.iter().map(|(_, n)| n).sum();
        assert_eq!(total, 4);
    }

    // ── kndl 视觉引擎测试 ──

    #[test]
    fn render_svg_has_svg_tag() {
        let mut base = MaoriBase::default_base();
        base.place(&MaoriCoord::new(1, 1, 0), "a");
        base.place(&MaoriCoord::new(2, 2, 1), "b");
        let svg = render_maori_svg(&base, &KndlVisualConfig::default());
        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>"));
    }

    #[test]
    fn render_svg_contains_label() {
        let mut base = MaoriBase::default_base();
        base.place(&MaoriCoord::new(0, 0, 0), "起");
        let config = KndlVisualConfig { show_labels: true, ..Default::default() };
        let svg = render_maori_svg(&base, &config);
        assert!(svg.contains("起") || svg.contains("<text"));
    }

    #[test]
    fn render_svg_empty_is_valid() {
        let base = MaoriBase::default_base();
        let svg = render_maori_svg(&base, &KndlVisualConfig::default());
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn render_svg_color_by_octant() {
        let mut base = MaoriBase::new(MaoriAxes::new(24, 39, 5));
        base.place(&MaoriCoord::new(0, 0, 0), "red");
        base.place(&MaoriCoord::new(23, 38, 4), "blue");
        let svg = render_maori_svg(&base, &KndlVisualConfig::default());
        let hue_count = svg.matches("hsl(").count();
        assert!(hue_count >= 6, "should have at least 6 hsl colors, got {}", hue_count);
    }

    #[test]
    fn iso_project_center() {
        let (sx, sy) = iso_project(0.0, 0.0, 0.0);
        assert_eq!(sx, 0.0);
        assert_eq!(sy, 0.0);
    }

    #[test]
    fn iso_project_x_increase() {
        let (sx0, sy0) = iso_project(0.0, 0.0, 0.0);
        let (sx1, sy1) = iso_project(5.0, 0.0, 0.0);
        assert!(sx1 > sx0);
        assert!(sy1 > sy0);
    }

    #[test]
    fn octant_color_hue_range() {
        for oct in 0u8..8 {
            let c = octant_color(oct);
            assert!(c.starts_with("hsl("));
        }
    }
}
