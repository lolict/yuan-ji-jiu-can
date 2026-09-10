// ═══════════════════════════════════════════════════════════════════════════════
// container_font_unified.rs — 容器字体统一架构
// ═══════════════════════════════════════════════════════════════════════════════
//
// 统一三个子系统：
//   · fridge (time-fridge-execution)     → 时间触发·冰箱坐标系·容器包
//   · machine-code (machine-code-container) → 语义ID层·多平台原生码
//   · glyph-font (glyph-3d-font)          → 三维几何投影·字体轮廓
//
// 统一接口：
//   · 32位地址 [X:8 语义类别][Y:16 操作数/参数][Z:8 时间窗口]
//   · 容器包 = pack_id(Container枚举) + func + param + time + payload + witness
//   · 拓扑映射 = X语义ID 直接对应 几何形状(不经过编码)
//   · 许可态射 = 范畴论许可表，决定两个容器之间是否存在合法转移
//
// ═══════════════════════════════════════════════════════════════════════════════

use crate::container_radix::Container;
use crate::couple::SoulState;
use crate::mirror::Sovereignty;

// ─────────────────────────────────────────────────────────────────────────────
// 32位统一地址 · [X:8][Y:16][Z:8]
// ─────────────────────────────────────────────────────────────────────────────
//
// 三个子系统共享同一个地址内核：
//   fridge:         功能ID(X) / 操作数(Y) / 时间窗口(Z)
//   machine-code:   语义ID(X) / 操作数编码(Y) / 相位对齐(Z)
//   glyph-font:     拓扑类型(X) / 变换参数(Y) / 视觉属性(Z)

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Address32(pub u32);

impl Address32 {
    /// 从三个分量构造地址
    pub fn new(x: u8, y: u16, z: u8) -> Self {
        let val = ((x as u32) << 24) | ((y as u32) << 8) | (z as u32);
        Self(val)
    }

    /// X分量：语义类别 / 功能ID / 拓扑类型
    pub fn x(self) -> u8 {
        ((self.0 >> 24) & 0xFF) as u8
    }

    /// Y分量：操作数 / 参数 / 变换
    pub fn y(self) -> u16 {
        ((self.0 >> 8) & 0xFFFF) as u16
    }

    /// Z分量：时间窗口 / 相位 / 视觉
    pub fn z(self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    /// 直接作为u32数值——不是编码，就是数值本身
    pub fn as_u32(self) -> u32 {
        self.0
    }

    /// 从u32直接构造——不经过解码，因为32位就是地址
    pub fn from_u32(v: u32) -> Self {
        Self(v)
    }

    /// 地址叠加（不进位折叠）
    pub fn fold(self, other: Self) -> Self {
        Self(self.0.saturating_add(other.0))
    }

    /// 莫顿码（Z阶曲线）编码——数值就是空间填充路径
    pub fn morton(self) -> u64 {
        morton_encode_3d(self.x() as u64, self.y() as u64, self.z() as u64)
    }
}

/// 莫顿码：三维坐标到一维数值的空间填充曲线
/// 不是编码，是几何投影——(X,Y,Z) → 数值轴上的行走路径
pub fn morton_encode_3d(x: u64, y: u64, z: u64) -> u64 {
    fn spread(mut v: u64) -> u64 {
        v &= 0x1FFFFF;
        v = (v | (v << 32)) & 0x1F00000000FFFF;
        v = (v | (v << 16)) & 0x1F0000FF0000FF;
        v = (v | (v <<  8)) & 0x100F00F00F00F00F;
        v = (v | (v <<  4)) & 0x10C30C30C30C30C3;
        v = (v | (v <<  2)) & 0x1249249249249249;
        v
    }
    spread(x) | (spread(y) << 1) | (spread(z) << 2)
}

// ─────────────────────────────────────────────────────────────────────────────
// 冰箱坐标系中的容器包 · Container Pack
// ─────────────────────────────────────────────────────────────────────────────
//
// 对齐 time-fridge-execution 技能，但消除 u8 编码层：
//   func_code 直接是 Container 枚举（不是 u8）
//   param_slot 是数值（不是编码）
//   payload 是预编码动作序列（不是源码）

#[derive(Clone)]
pub struct ContainerPack {
    pub pack_id: Container,           // 容器名 = 符号（不是编码）
    pub address: Address32,           // 32位统一地址
    pub payload: PackPayload,         // 预编码动作序列
    pub witness: WitnessSignature,    // 三元裁决签名
}

/// 动作 payload —— 直接是动作序列（不是源码，不是字节码）
#[derive(Clone, Debug)]
pub enum PackPayload {
    Empty,                                          // 空袋（许可：拒绝执行）
    Single(ContainerAddress),                        // 单步动作（一个容器地址）
    Chain(Vec<ContainerAddress>),                    // 链式动作（多个容器地址序列）
    Branch(Box<PackPayload>, Box<PackPayload>),      // 分支（由条件决定走哪条链）
    Parallel(Vec<PackPayload>),                      // 并行（相同时间窗的多个链）
}

/// 容器地址 —— 在64容器空间中的精确坐标
#[derive(Clone, Copy, Debug)]
pub struct ContainerAddress {
    pub x: i8,
    pub y: i8,
    pub z: i8,
    pub container: Container,
}

impl ContainerAddress {
    pub fn new(x: i8, y: i8, z: i8, container: Container) -> Self {
        Self { x, y, z, container }
    }

    /// 坐标到容器空间的映射（直接数值运算，不经过编码）
    pub fn to_address32(&self) -> Address32 {
        Address32::new(
            self.x as u8,
            ((self.y as u16) << 8) | (self.z as u16),
            self.container as u8,
        )
    }

    fn fold(&self, other: &Self) -> Self {
        Self {
            x: self.x.saturating_add(other.x),
            y: self.y.saturating_add(other.y),
            z: self.z.saturating_add(other.z),
            container: self.container, // 主导容器不变
        }
    }
}

/// 三元裁决签名
#[derive(Clone, Copy, Debug)]
pub struct WitnessSignature {
    pub sovereignty: Sovereignty,    // 主权状态（主动/被动/中立）
    pub soul_state: SoulState,       // 夫妻状态
    pub polarity: Polarity,          // 极性（阴/阳/元）
}

impl WitnessSignature {
    pub fn verdict(&self) -> Verdict {
        // 数值准入
        if self.polarity == Polarity::Yuan || self.polarity == Polarity::Yang {
            Verdict::Recognized
        } else {
            Verdict::Rejected
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Recognized,
    Rejected,
    Waiting,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Polarity {
    Yin,
    Yuan,
    Yang,
}

// ─────────────────────────────────────────────────────────────────────────────
// 三维几何体 · 字体投影层
// ─────────────────────────────────────────────────────────────────────────────
//
// 对齐 glyph-3d-font 技能：
//   X语义ID直接对应几何形状，不经过任何中间编码

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Topology {
    Box,        // 盒型 (立方体) — LOAD/STORE — 容器
    Sphere,     // 球型 — 算逻运算 — 数据流
    Torus,      // 环面 — JMP/BNE — 循环/分支
    Tube,       // 管状 — CALL/RET — 调用栈
    Bicone,     // 双锥 — CMP — 条件判断
    Line,       // 线型 — MOV — 数据移动
    Point,      // 点型 — NOP — 空操作
    Grid,       // 网格型 — 表格编程单元
}

impl Topology {
    /// 从X语义ID直接映射到拓扑类型（不经过编码转换）
    pub fn from_semantic_x(x: u8) -> Self {
        match x {
            0x01..=0x02 => Topology::Box,       // LOAD/STORE
            0x03..=0x08 => Topology::Sphere,    // ADD/SUB/AND/OR/XOR/SHIFT
            0x09..=0x0A => Topology::Torus,     // JMP/BNE
            0x0B..=0x0C => Topology::Tube,      // CALL/RET
            0x0D       => Topology::Bicone,     // CMP
            0x0E       => Topology::Line,       // MOV
            0x00       => Topology::Point,      // NOP
            _          => Topology::Box,
        }
    }

    /// 几何体参数（顶点、边、面）— 直接数值，非编码
    pub fn geometry_params(self) -> (u8, u8, u8) {
        match self {
            Topology::Box     => (8, 12, 6),  // V=8 E=12 F=6
            Topology::Sphere  => (1, 0, 1),   // V=1 E=0 F=1
            Topology::Torus   => (0, 2, 1),   // V=0 E=2 F=1
            Topology::Tube    => (0, 2, 3),   // V=0 E=2 F=3
            Topology::Bicone  => (2, 1, 2),   // V=2 E=1 F=2
            Topology::Line    => (2, 1, 0),   // V=2 E=1 F=0
            Topology::Point   => (1, 0, 0),   // V=1 E=0 F=0
            Topology::Grid    => (4, 4, 1),   // V=4 E=4 F=1 (表格形)
        }
    }
}

/// 三维几何体 — 由地址直接投影而来
#[derive(Clone, Debug)]
pub struct Geometry3D {
    pub address: Address32,
    pub topology: Topology,
    pub rotation: u16,       // 旋转角 (0-359度映射)
    pub scale_x: u8,         // X缩放因子
    pub scale_y: u8,         // Y缩放因子
    pub color_phase: u8,     // 颜色相位
    pub pulse_freq: u8,      // 脉冲频率
}

impl Geometry3D {
    /// 从地址直接投影为三维几何体（不经过编码）
    pub fn project(addr: Address32) -> Self {
        let topology = Topology::from_semantic_x(addr.x());
        let (rot_hi, rot_lo) = (addr.y() >> 8, addr.y() & 0xFF);
        let rotation = ((rot_hi as u16) * 360 / 256) | (rot_lo as u16);
        let scale_x = ((addr.y() >> 4) & 0x0F) as u8;
        let scale_y = (addr.y() & 0x0F) as u8;
        let color_phase = (addr.z() >> 4) as u8;
        let pulse_freq = (addr.z() & 0x0F) as u8;

        Self {
            address: addr,
            topology,
            rotation,
            scale_x,
            scale_y,
            color_phase,
            pulse_freq,
        }
    }

    /// 几何体在空间中的位置（直接数值坐标）
    pub fn position(&self) -> (i8, i8, i8) {
        (self.address.x() as i8, self.address.y() as i8, self.address.z() as i8)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 许可态射表 · 范畴论许可证
// ─────────────────────────────────────────────────────────────────────────────
//
// 对齐用户的"表格编程"概念：
//   标题：两个容器之间的态射（转移）
//   内容：许可（存在）/ 许可（不存在）

#[derive(Clone, Debug)]
pub struct MorphismTable {
    morphisms: Vec<(Container, Container, MorphismKind)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MorphismKind {
    Allowed,    // 许可 — 态射存在，路径连通
    Denied,     // 不许 — 态射不存在，路径断裂
    Conditional(Polarity),  // 条件许可 — 需要特定极性才允许
}

impl MorphismTable {
    pub fn new() -> Self {
        let mut table = Self { morphisms: Vec::new() };
        table.init_default_morphisms();
        table
    }

    /// 查询两个容器之间的态射是否允许
    pub fn query(&self, from: Container, to: Container) -> MorphismKind {
        for &(src, dst, kind) in &self.morphisms {
            if src == from && dst == to {
                return kind;
            }
        }
        MorphismKind::Allowed // 默认许可
    }

    /// 添加一个态射规则
    pub fn add(&mut self, from: Container, to: Container, kind: MorphismKind) {
        self.morphisms.push((from, to, kind));
    }

    /// 初始化默认态射规则
    fn init_default_morphisms(&mut self) {
        use Container::*;
        // 夫妻容器之间的态射（核心锚点）
        self.add(Dai, Tong, MorphismKind::Allowed);      // 袋 → 桶 允许
        self.add(Tong, Guan, MorphismKind::Allowed);     // 桶 → 罐 允许
        self.add(Guan, Pen, MorphismKind::Allowed);      // 罐 → 盆 允许
        self.add(Pen, Yu, MorphismKind::Allowed);        // 盆 → 盂 允许
        self.add(Yu, Hu2, MorphismKind::Allowed);        // 盂 → 壶 允许
        self.add(Hu2, He, MorphismKind::Allowed);        // 壶 → 盒 允许
        self.add(He, Ping2, MorphismKind::Allowed);      // 盒 → 瓶 允许
        // 添加一些条件许可
        self.add(Dai, Long, MorphismKind::Conditional(Polarity::Yang)); // 袋→笼需阳性
        self.add(Tong, Ji, MorphismKind::Conditional(Polarity::Yin));   // 桶→箕需阴性
        // 添加一些不许可
        self.add(Long, Dai, MorphismKind::Denied);       // 笼 → 袋 禁止
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 统一容器世界 · 整合所有子系统
// ─────────────────────────────────────────────────────────────────────────────

pub struct UnifiedContainerWorld {
    /// 容器包存储：地址32位 → 容器包
    packs: Vec<(Address32, ContainerPack)>,
    /// 几何体缓存：地址32位 → 三维几何
    geometries: Vec<(Address32, Geometry3D)>,
    /// 态射表（范畴论许可）
    morphisms: MorphismTable,
    /// 当前时间光标（沿Z轴移动）
    time_cursor: u8,
}

impl UnifiedContainerWorld {
    pub fn new() -> Self {
        Self {
            packs: Vec::new(),
            geometries: Vec::new(),
            morphisms: MorphismTable::new(),
            time_cursor: 0,
        }
    }

    /// 注册一个容器包到世界
    pub fn register_pack(&mut self, addr: Address32, pack: ContainerPack) {
        self.packs.push((addr, pack));
    }

    /// 通过32位地址获取容器包（冰箱取袋）
    pub fn get_pack(&self, addr: Address32) -> Option<&ContainerPack> {
        self.packs.iter().find(|(a, _)| *a == addr).map(|(_, p)| p)
    }

    /// 通过32位地址获取三维几何体（字体投影）
    pub fn get_geometry(&self, addr: Address32) -> Option<&Geometry3D> {
        self.geometries.iter().find(|(a, _)| *a == addr).map(|(_, g)| g)
    }

    /// 查找或生成几何体
    pub fn geometry_or_project(&mut self, addr: Address32) -> &Geometry3D {
        if self.get_geometry(addr).is_none() {
            let geo = Geometry3D::project(addr);
            self.geometries.push((addr, geo));
        }
        self.geometries.iter().find(|(a, _)| *a == addr).map(|(_, g)| g).unwrap()
    }

    /// 执行容器包（时间触发：Z轴对齐当前光标才能取袋）
    pub fn execute_at(&mut self, addr: Address32) -> Option<Verdict> {
        let pack = self.get_pack(addr)?;
        // Z轴时间窗口对齐检查
        if pack.address.z() != self.time_cursor {
            return Some(Verdict::Waiting);
        }
        Some(pack.witness.verdict())
    }

    /// 推进时间光标
    pub fn advance_time(&mut self) {
        self.time_cursor = self.time_cursor.wrapping_add(1);
    }

    /// 查询两个容器是否允许态射（许可查询）
    pub fn permits(&self, from: Container, to: Container) -> bool {
        match self.morphisms.query(from, to) {
            MorphismKind::Allowed => true,
            MorphismKind::Denied => false,
            MorphismKind::Conditional(p) => p == Polarity::Yuan || p == Polarity::Yang,
        }
    }

    /// 沿Z轴扫描时间窗口，返回所有可执行的容器包
    pub fn scan_executable(&self) -> Vec<(Address32, &ContainerPack)> {
        self.packs
            .iter()
            .filter(|(addr, _)| addr.z() == self.time_cursor)
            .map(|(addr, pack)| (*addr, pack))
            .collect()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 容器包构建器 · 编译时编排
// ─────────────────────────────────────────────────────────────────────────────

pub struct PackBuilder {
    pack_id: Container,
    address: Address32,
    payload: PackPayload,
    witness: WitnessSignature,
}

impl PackBuilder {
    pub fn new(pack_id: Container, address: Address32) -> Self {
        Self {
            pack_id,
            address,
            payload: PackPayload::Empty,
            witness: WitnessSignature {
                sovereignty: Sovereignty::ActiveSub,
                soul_state: SoulState::FullCouple,
                polarity: Polarity::Yuan,
            },
        }
    }

    pub fn payload(mut self, payload: PackPayload) -> Self {
        self.payload = payload;
        self
    }

    pub fn witness(mut self, witness: WitnessSignature) -> Self {
        self.witness = witness;
        self
    }

    pub fn build(self) -> ContainerPack {
        ContainerPack {
            pack_id: self.pack_id,
            address: self.address,
            payload: self.payload,
            witness: self.witness,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 三维表格单元 · 表格编程范式
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct TableCell3D {
    pub address: Address32,        // 单元格位置
    pub permission: MorphismKind,   // 许可/不许
    pub content: Option<Container>, // 内容（容器）
    pub geometry_topology: Topology, // 该单元格的拓扑形状
}

impl TableCell3D {
    /// 查询该单元格是否允许与另一个单元格建立态射
    pub fn acts_on(&self, other: &Self, world: &UnifiedContainerWorld) -> bool {
        match (&self.content, &other.content) {
            (Some(my), Some(oth)) => world.permits(*my, *oth),
            _ => false, // 空单元格无态射
        }
    }

    /// 三维表格中串行的下一个单元格（沿X轴移动）
    pub fn next_on_x(&self) -> Self {
        let mut addr = self.address;
        let new_x = addr.x().wrapping_add(1);
        Address32::new(new_x, addr.y(), addr.z());
        Self {
            address: Address32::new(new_x, self.address.y(), self.address.z()),
            permission: MorphismKind::Allowed,
            content: None,
            geometry_topology: Topology::from_semantic_x(new_x),
        }
    }

    /// 三维表格中沿Y轴移动（表格的另一列）
    pub fn next_on_y(&self) -> Self {
        Self {
            address: Address32::new(self.address.x(), self.address.y().wrapping_add(1), self.address.z()),
            permission: MorphismKind::Allowed,
            content: None,
            geometry_topology: Topology::from_semantic_x(self.address.x()),
        }
    }

    /// 三维表格中沿Z轴推进（时间光标前进）
    pub fn advance_z(&self) -> Self {
        Self {
            address: Address32::new(self.address.x(), self.address.y(), self.address.z().wrapping_add(1)),
            permission: MorphismKind::Allowed,
            content: None,
            geometry_topology: Topology::from_semantic_x(self.address.x()),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 测试
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address32_construction() {
        let addr = Address32::new(0x03, 0xA1B2, 0x5F);
        assert_eq!(addr.x(), 0x03);
        assert_eq!(addr.y(), 0xA1B2);
        assert_eq!(addr.z(), 0x5F);
    }

    #[test]
    fn test_address32_roundtrip() {
        let original = Address32::new(0x01, 0x0000, 0x01);
        let as_u32 = original.as_u32();
        let recovered = Address32::from_u32(as_u32);
        assert_eq!(original, recovered);
    }

    #[test]
    fn test_topology_from_semantic() {
        assert_eq!(Topology::from_semantic_x(0x01), Topology::Box);    // LOAD
        assert_eq!(Topology::from_semantic_x(0x03), Topology::Sphere); // ADD
        assert_eq!(Topology::from_semantic_x(0x09), Topology::Torus);  // JMP
        assert_eq!(Topology::from_semantic_x(0x0D), Topology::Bicone); // CMP
        assert_eq!(Topology::from_semantic_x(0x0E), Topology::Line);   // MOV
    }

    #[test]
    fn test_geometry_projection() {
        let addr = Address32::new(0x03, 0x0A10, 0x5F);
        let geo = Geometry3D::project(addr);
        assert_eq!(geo.topology, Topology::Sphere);
        assert!(geo.rotation <= 360);
    }

    #[test]
    fn test_morphism_table_allows() {
        let table = MorphismTable::new();
        assert_eq!(table.query(Container::Dai, Container::Tong), MorphismKind::Allowed);
        assert_eq!(table.query(Container::Long, Container::Dai), MorphismKind::Denied);
    }

    #[test]
    fn test_morphism_table_conditional() {
        let table = MorphismTable::new();
        // 袋→笼是Conditional(Yang)
        let result = table.query(Container::Dai, Container::Long);
        assert_eq!(result, MorphismKind::Conditional(Polarity::Yang));
    }

    #[test]
    fn test_unified_world_get_pack() {
        let mut world = UnifiedContainerWorld::new();
        let addr = Address32::new(0x01, 0x0000, 0x01);
        let pack = PackBuilder::new(Container::Dai, addr)
            .payload(PackPayload::Single(ContainerAddress::new(0, 0, 0, Container::Dai)))
            .build();
        world.register_pack(addr, pack);

        let retrieved = world.get_pack(addr);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().pack_id, Container::Dai);
    }

    #[test]
    fn test_unified_world_time_trigger() {
        let mut world = UnifiedContainerWorld::new();
        let addr = Address32::new(0x01, 0x0000, 0x01);
        let pack = PackBuilder::new(Container::Dai, addr)
            .payload(PackPayload::Single(ContainerAddress::new(0, 0, 0, Container::Dai)))
            .build();
        world.register_pack(addr, pack);

        // 当前时间光标=0，Z窗口=1，不在同一时间，应该Waiting
        let verdict = world.execute_at(addr);
        assert_eq!(verdict, Some(Verdict::Waiting));

        // 推进时间光标到1，Z窗口对齐
        world.advance_time(); // time_cursor = 1
        let verdict = world.execute_at(addr);
        assert_eq!(verdict, Some(Verdict::Recognized));
    }

    #[test]
    fn test_pack_payload_chain() {
        let addr1 = ContainerAddress::new(0, 0, 0, Container::Dai);
        let addr2 = ContainerAddress::new(1, 0, 0, Container::Tong);
        let chain = PackPayload::Chain(vec![addr1, addr2]);
        match chain {
            PackPayload::Chain(ref v) => assert_eq!(v.len(), 2),
            _ => panic!("Expected Chain"),
        }
    }

    #[test]
    fn test_pack_payload_parallel() {
        let parallel = PackPayload::Parallel(vec![
            PackPayload::Single(ContainerAddress::new(0, 0, 0, Container::Dai)),
            PackPayload::Single(ContainerAddress::new(1, 0, 0, Container::Tong)),
        ]);
        match parallel {
            PackPayload::Parallel(ref v) => assert_eq!(v.len(), 2),
            _ => panic!("Expected Parallel"),
        }
    }

    #[test]
    fn test_geometry_params() {
        let (v, e, f) = Topology::Box.geometry_params();
        assert_eq!((v, e, f), (8, 12, 6));
        let (v, e, f) = Topology::Sphere.geometry_params();
        assert_eq!((v, e, f), (1, 0, 1));
        let (v, e, f) = Topology::Line.geometry_params();
        assert_eq!((v, e, f), (2, 1, 0));
    }

    #[test]
    fn test_table_cell_3d_movement() {
        let cell = TableCell3D {
            address: Address32::new(1, 2, 3),
            permission: MorphismKind::Allowed,
            content: Some(Container::Dai),
            geometry_topology: Topology::Box,
        };
        let next_x = cell.next_on_x();
        assert_eq!(next_x.address.x(), 2);
        assert_eq!(next_x.address.y(), 2);
        assert_eq!(next_x.address.z(), 3);
        let next_y = cell.next_on_y();
        assert_eq!(next_y.address.y(), 3);
        let next_z = cell.advance_z();
        assert_eq!(next_z.address.z(), 4);
    }
}
