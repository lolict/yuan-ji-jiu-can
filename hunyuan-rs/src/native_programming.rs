// ═══════════════════════════════════════════════════════════════════════════════
// native_programming.rs — 容器编程 · 三维立体进制数据库
// ═══════════════════════════════════════════════════════════════════════════════
//
// 核心哲学声明（用户定义，不可更改）：
//
//   容器 = 数据库
//   三维立体进制数据库
//   一个容器 = 一个几何图形
//   几何图形 = 索引地址
//   索引地址 = 可预加载的数据库
//   数值 = 字节码
//   字节码 = 容器名的符号
//   符号 ≠ 编码
//
// 与前版的关键区别：
//   前版：汉字 → 编码(u8) → 解释 → 操作
//   本版：容器就是数据库 → 直接操作（无编码层）
//
// ═══════════════════════════════════════════════════════════════════════════════

use crate::container_radix::Container;
use crate::couple::SoulState;
use crate::mirror::Sovereignty;

// ─────────────────────────────────────────────────────────────────────────────
// 容器几何体 · 物理属性
// ─────────────────────────────────────────────────────────────────────────────
//
// 容器本身就是几何体。它不是"对应"几何体，它 IS 几何体。

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContainerGeometry {
    pub vertices: u8,       // 顶点数 (决定几何形状)
    pub edges: u8,          // 边数
    pub faces: u8,          // 面数
    pub genus: u8,          // 亏格 (洞数，决定拓扑)
}

impl ContainerGeometry {
    /// 顶点数 + 边数 + 面数 → 欧拉示性数
    /// V - E + F = 2 - 2g
    pub fn euler_char(self) -> i8 {
        self.vertices as i8 - self.edges as i8 + self.faces as i8
    }

    /// 是否可定向 (单面容器 = 不可定向 = 莫比乌斯)
    pub fn is_orientable(self) -> bool {
        self.genus == 0 && self.faces > 0
    }

    /// 几何复杂度 = 顶点 × 边 × 面 (决定容器容量)
    pub fn capacity(self) -> u64 {
        self.vertices as u64 * self.edges as u64 * self.faces as u64
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 容器地址 · 三维坐标 = 索引地址 = 数据库入口
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ContainerAddress {
    pub x: i8,              // 丈夫轴 (奇数位)
    pub y: i8,              // 妻子轴 (偶数位)
    pub z: i8,              // 第三维 (时间/相位轴)
}

impl ContainerAddress {
    pub fn new(x: i8, y: i8, z: i8) -> Self {
        Self { x, y, z }
    }

    /// 坐标叠加（不进位折叠）
    pub fn fold(self, other: Self) -> Self {
        Self {
            x: self.x.saturating_add(other.x),
            y: self.y.saturating_add(other.y),
            z: self.z.saturating_add(other.z),
        }
    }

    /// 地址 → 6位容器索引 (用于 Container 枚举)
    pub fn as_index(self) -> u8 {
        let ux = ((self.x as u8) & 0x0F) << 2;
        let uy = (self.y as u8) & 0x0F;
        let uz = (self.z as u8) & 0x03;
        ux | (uy >> 2) | (uz << 6)
    }

    /// 地址 → 64位容器指纹 (用于数据库定位)
    pub fn as_fingerprint(self) -> u64 {
        let ux = (self.x as u64) & 0xFF;
        let uy = (self.y as u64) & 0xFF;
        let uz = (self.z as u64) & 0xFF;
        (ux << 56) | (uy << 48) | (uz << 40)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 容器数据库 · 容器 = 数据库 = 几何 = 地址 = 可预加载
// ─────────────────────────────────────────────────────────────────────────────
//
// 核心统一：一个结构体包含容器所有方面，不做任何编码转换。

#[derive(Clone, Debug)]
pub struct ContainerDatabase {
    pub name: Container,            // 容器名 = 符号（不是编码）
    pub geometry: ContainerGeometry, // 几何图形
    pub address: ContainerAddress,   // 索引地址
    pub data: Vec<u8>,              // 内容（可预加载的数据）
    pub preloaded: bool,            // 是否已预加载
}

impl ContainerDatabase {
    /// 从容器名创建数据库（自动推导几何和地址）
    pub fn new(name: Container) -> Self {
        let geo = Self::derive_geometry(name);
        let addr = Self::derive_address(name);
        Self {
            name,
            geometry: geo,
            address: addr,
            data: Vec::new(),
            preloaded: false,
        }
    }

    /// 推导容器几何属性
    ///
    /// 基于容器象征形状的拓扑结构：
    ///   · 袋 (Dai) — 开口扁平：V=4 E=4 F=1 genus=0 (开放平面)
    ///   · 桶 (Tong) — 圆柱体：V=0 E=2 F=3 genus=0 (闭合侧面)
    ///   · 罐 (Guan) — 球体：V=1 E=0 F=1 genus=0 (闭合实体)
    ///   · 盆 (Pen) — 半球：V=0 E=1 F=2 genus=0 (半开放)
    fn derive_geometry(name: Container) -> ContainerGeometry {
        match name {
            Container::Dai   => ContainerGeometry { vertices: 4, edges: 4, faces: 1, genus: 0 }, // 袋·方形·开放
            Container::Tong  => ContainerGeometry { vertices: 0, edges: 2, faces: 3, genus: 1 }, // 桶·圆柱·空心
            Container::Guan  => ContainerGeometry { vertices: 1, edges: 0, faces: 1, genus: 0 }, // 罐·球·闭合
            Container::Pen   => ContainerGeometry { vertices: 0, edges: 1, faces: 2, genus: 0 }, // 盆·半开
            Container::Yu    => ContainerGeometry { vertices: 4, edges: 6, faces: 4, genus: 0 }, // 盂·锥形·开放
            Container::Hu2   => ContainerGeometry { vertices: 1, edges: 2, faces: 2, genus: 0 }, // 壶·壶嘴+壶身
            Container::He    => ContainerGeometry { vertices: 8, edges: 12, faces: 6, genus: 0 }, // 盒·立方
            Container::Ping2 => ContainerGeometry { vertices: 4, edges: 6, faces: 4, genus: 0 }, // 瓶·柱形
            Container::Zhan  => ContainerGeometry { vertices: 4, edges: 4, faces: 2, genus: 0 }, // 盏·浅口
            Container::Ang   => ContainerGeometry { vertices: 6, edges: 9, faces: 5, genus: 0 }, // 盎·凹面
            Container::Lou   => ContainerGeometry { vertices: 4, edges: 6, faces: 3, genus: 0 }, // 篓·编织
            Container::Kuang => ContainerGeometry { vertices: 4, edges: 6, faces: 4, genus: 0 }, // 筐·方形编织
            Container::Lan   => ContainerGeometry { vertices: 3, edges: 3, faces: 2, genus: 0 }, // 篮·锥形编织
            Container::Luo   => ContainerGeometry { vertices: 5, edges: 8, faces: 4, genus: 0 }, // 箩·圆柱筛
            Container::Ji    => ContainerGeometry { vertices: 3, edges: 3, faces: 1, genus: 0 }, // 箕·三角
            Container::Long  => ContainerGeometry { vertices: 6, edges: 9, faces: 4, genus: 0 }, // 笼·柱形编织
            _ => ContainerGeometry { vertices: 4, edges: 4, faces: 1, genus: 0 },
        }
    }

    /// 推导容器地址
    ///
    /// 基于容器在64容器空间中的位置：
    ///   使用枚举判别值作为地址源，不经过编码转换。
    fn derive_address(name: Container) -> ContainerAddress {
        let idx = name as u8;
        ContainerAddress {
            x: ((idx / 8) % 8) as i8,
            y: (idx % 8) as i8,
            z: (idx / 64) as i8,
        }
    }

    /// 获取数据库内容（先预加载再返回）
    pub fn load(&mut self) -> &[u8] {
        if !self.preloaded {
            self.preload();
        }
        &self.data
    }

    /// 预加载数据库内容
    ///
    /// 内容 = 容器名的符号 + 几何指纹 + 地址串联
    fn preload(&mut self) {
        self.data.clear();
        // 第一字节: 容器名符号 (直接 u8，非"编码")
        self.data.push(self.name as u8);
        // 第2-5字节: 几何指纹
        self.data.push(self.geometry.vertices);
        self.data.push(self.geometry.edges);
        self.data.push(self.geometry.faces);
        self.data.push(self.geometry.genus);
        // 第6-8字节: 地址
        self.data.push(self.address.x as u8);
        self.data.push(self.address.y as u8);
        self.data.push(self.address.z as u8);
        self.preloaded = true;
    }

    /// 存储内容到数据库
    pub fn store(&mut self, data: &[u8]) {
        self.data.clear();
        self.data.extend_from_slice(data);
        self.preloaded = true;
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 数值 · 字节码 · 容器名符号
// ─────────────────────────────────────────────────────────────────────────────
//
// 数值不是被解释的——数值就是字节码，字节码就是容器名，容器名就是容器。
//
// u64 的位分布：
//   [63:56] = x 坐标值
//   [55:48] = y 坐标值
//   [47:40] = z 坐标值
//   [39:34] = 极性 (阴/阳/元)
//   [33:0]  = 数据/内容

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContainerValue(pub u64);

impl ContainerValue {
    /// 从坐标直接构造数值（不需要编码）
    pub fn from_address(addr: ContainerAddress, polarity: Polarity) -> Self {
        let mut val: u64 = 0;
        val |= ((addr.x as u64) & 0xFF) << 56;
        val |= ((addr.y as u64) & 0xFF) << 48;
        val |= ((addr.z as u64) & 0xFF) << 40;
        val |= (polarity as u64) << 34;
        Self(val)
    }

    /// 数值 → 地址 (直接提取，非解码)
    pub fn address(self) -> ContainerAddress {
        ContainerAddress {
            x: ((self.0 >> 56) & 0xFF) as i8,
            y: ((self.0 >> 48) & 0xFF) as i8,
            z: ((self.0 >> 40) & 0xFF) as i8,
        }
    }

    /// 数值 → 极性
    pub fn polarity(self) -> Polarity {
        match (self.0 >> 34) & 0x3F {
            0 => Polarity::Yin,
            1 => Polarity::Yuan,
            2 => Polarity::Yang,
            _ => Polarity::Yuan,
        }
    }

    /// 数值 → 内容 (低34位)
    pub fn content(self) -> u64 {
        self.0 & 0x3FFFFFFFF
    }

    /// 数值直接作为容器名符号
    pub fn as_container(self) -> Option<Container> {
        let name_byte = (self.0 & 0xFF) as u8;
        // 安全转换：检查是否是合法的容器枚举值
        if name_byte < 64 {
            Some(unsafe { std::mem::transmute::<u8, Container>(name_byte) })
        } else {
            None
        }
    }

    /// 两个数值的折叠操作（不进位）
    pub fn fold(self, other: Self) -> Self {
        let a = self.address();
        let b = other.address();
        let folded = a.fold(b);
        let pol = self.polarity().fold(other.polarity());
        Self::from_address(folded, pol)
    }

    /// 数值直接执行 = 操作对应的容器数据库
    pub fn execute(self) -> Option<ContainerDatabase> {
        let container = self.as_container()?;
        Some(ContainerDatabase::new(container))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 极性三元组 · 阴阳元
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity {
    Yin  = 0,   // 阴 · 妻子 · 包容
    Yuan = 1,   // 元 · 见证 · 中介
    Yang = 2,   // 阳 · 丈夫 · 主动
}

impl Polarity {
    pub fn fold(self, other: Self) -> Self {
        match (self as u8 + other as u8) {
            0 => Self::Yin,    // 阴+阴=阴
            1 => Self::Yuan,   // 阴+元=元 / 元+阴=元
            2 => Self::Yang,   // 阴+阳=阳 / 元+元=阳 / 阳+阴=阳
            3 => Self::Yuan,   // 元+阳=元（折叠不进位）
            4 => Self::Yin,    // 阳+阳=阴（折叠不进位）
            _ => Self::Yuan,   // 默认元哨兵
        }
    }

    pub fn to_bitpair(self) -> (u64, u64) {
        match self {
            Self::Yin  => (1, 0),
            Self::Yang => (0, 1),
            Self::Yuan => (1, 1),
        }
    }

    /// 极性对应的声调相位
    pub fn as_tone(self) -> TonePhase {
        match self {
            Self::Yin  => TonePhase::YinPing,
            Self::Yuan => TonePhase::YangPing,
            Self::Yang => TonePhase::Qu,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 声调相位 · 4 种执行模式
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TonePhase {
    YinPing  = 0,  // 阴平 · 平稳
    YangPing = 1,  // 阳平 · 上升
    Shang    = 2,  // 上声 · 转折
    Qu       = 3,  // 去声 · 下降
}

impl TonePhase {
    pub fn coord_offset(self) -> ContainerAddress {
        match self {
            Self::YinPing  => ContainerAddress::new(0, 0, 0),
            Self::YangPing => ContainerAddress::new(1, 0, 0),
            Self::Shang    => ContainerAddress::new(0, 1, 0),
            Self::Qu       => ContainerAddress::new(0, 0, -1),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 笔画操作 · 容器空间中的几何变换
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum StrokeOp {
    // 基础四笔画
    Heng   = 0x00,  // 横 · 袋
    Shu    = 0x01,  // 竖 · 桶
    Pie    = 0x02,  // 撇 · 罐
    Na     = 0x03,  // 捺 · 盆
    // 点提二势
    Dian   = 0x04,  // 点 · 盂
    Ti     = 0x05,  // 提 · 壶
    // 折转四变
    HengZhe  = 0x06,  // 横折 · 盒
    ShuZhe   = 0x07,  // 竖折 · 瓶
    HengPie  = 0x08,  // 横撇 · 盏
    ShuGou   = 0x09,  // 竖钩 · 盎
    // 钩弯四揽
    HengGou    = 0x0A,  // 横钩 · 篓
    WanGou     = 0x0B,  // 弯钩 · 筐
    ShuWanGou  = 0x0C,  // 竖弯钩 · 篮
    HengZheZhe = 0x0D,  // 横折折 · 箩
    // 特殊二形
    ShuWan = 0x0E,  // 竖弯 · 箕
    XieGou = 0x0F,  // 斜钩 · 笼
}

impl StrokeOp {
    pub fn from_stroke_name(name: &str) -> Option<Self> {
        match name {
            "横" | "一"   => Some(Self::Heng),
            "竖" | "丨"   => Some(Self::Shu),
            "撇" | "丿"   => Some(Self::Pie),
            "捺" | "㇏"   => Some(Self::Na),
            "点" | "丶"   => Some(Self::Dian),
            "提" | "㇀"   => Some(Self::Ti),
            "横折" | "㇕" => Some(Self::HengZhe),
            "竖折" | "㇗" => Some(Self::ShuZhe),
            "横撇" | "㇇" => Some(Self::HengPie),
            "竖钩" | "㇚" => Some(Self::ShuGou),
            "横钩" | "㇛" => Some(Self::HengGou),
            "弯钩" | "㇜" => Some(Self::WanGou),
            "竖弯钩" | "㇄" => Some(Self::ShuWanGou),
            "横折折" | "㇆" => Some(Self::HengZheZhe),
            "竖弯" | "㇁" => Some(Self::ShuWan),
            "斜钩" | "㇂" => Some(Self::XieGou),
            _ => None,
        }
    }

    /// 笔画的极性（由对应容器决定）
    pub fn polarity(self) -> Polarity {
        use StrokeOp::*;
        match self {
            Heng | Pie | Dian | HengPie | HengGou | HengZheZhe | XieGou => Polarity::Yang,
            _ => Polarity::Yin,
        }
    }

    /// 笔画对应的容器
    pub fn container(self) -> Container {
        use Container::*;
        match self {
            Self::Heng      => Dai,
            Self::Shu       => Tong,
            Self::Pie       => Guan,
            Self::Na        => Pen,
            Self::Dian      => Yu,
            Self::Ti        => Hu2,
            Self::HengZhe   => He,
            Self::ShuZhe    => Ping2,
            Self::HengPie   => Zhan,
            Self::ShuGou    => Ang,
            Self::HengGou   => Lou,
            Self::WanGou    => Kuang,
            Self::ShuWanGou => Lan,
            Self::HengZheZhe=> Luo,
            Self::ShuWan    => Ji,
            Self::XieGou    => Long,
        }
    }

    /// 笔画在容器空间中的位移
    pub fn displacement(self) -> ContainerAddress {
        use StrokeOp::*;
        match self {
            Heng      => ContainerAddress::new(1, 0, 0),
            Shu       => ContainerAddress::new(0, 1, 0),
            Pie       => ContainerAddress::new(1, -1, 0),
            Na        => ContainerAddress::new(-1, -1, 0),
            Dian      => ContainerAddress::new(0, 0, 1),
            Ti        => ContainerAddress::new(0, 1, 1),
            HengZhe   => ContainerAddress::new(1, 1, 0),
            ShuZhe    => ContainerAddress::new(0, 1, -1),
            HengPie   => ContainerAddress::new(1, 0, -1),
            ShuGou    => ContainerAddress::new(-1, 1, 0),
            HengGou   => ContainerAddress::new(1, 0, 1),
            WanGou    => ContainerAddress::new(0, -1, 1),
            ShuWanGou => ContainerAddress::new(-1, -1, 1),
            HengZheZhe=> ContainerAddress::new(1, 1, -1),
            ShuWan    => ContainerAddress::new(0, -1, -1),
            XieGou    => ContainerAddress::new(1, -1, 1),
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Heng      => "横",
            Self::Shu       => "竖",
            Self::Pie       => "撇",
            Self::Na        => "捺",
            Self::Dian      => "点",
            Self::Ti        => "提",
            Self::HengZhe   => "横折",
            Self::ShuZhe    => "竖折",
            Self::HengPie   => "横撇",
            Self::ShuGou    => "竖钩",
            Self::HengGou   => "横钩",
            Self::WanGou    => "弯钩",
            Self::ShuWanGou => "竖弯钩",
            Self::HengZheZhe=> "横折折",
            Self::ShuWan    => "竖弯",
            Self::XieGou    => "斜钩",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 汉字笔画数据库 · 内置笔顺表
// ─────────────────────────────────────────────────────────────────────────────

pub fn get_stroke_sequence(c: char) -> Option<Vec<StrokeOp>> {
    let strokes: &[(&str, &[&str])] = &[
        ("一", &["横"]),
        ("二", &["横", "横"]),
        ("三", &["横", "横", "横"]),
        ("十", &["横", "竖"]),
        ("人", &["撇", "捺"]),
        ("八", &["撇", "捺"]),
        ("入", &["撇", "捺"]),
        ("刀", &["横折钩", "撇"]),
        ("力", &["横折钩", "撇"]),
        ("又", &["横撇", "捺"]),
        ("上", &["竖", "横", "横"]),
        ("下", &["横", "竖", "点"]),
        ("土", &["横", "竖", "横"]),
        ("大", &["横", "撇", "捺"]),
        ("小", &["竖钩", "撇", "点"]),
        ("山", &["竖", "竖折", "竖"]),
        ("工", &["横", "竖", "横"]),
        ("己", &["横折", "横", "竖弯钩"]),
        ("已", &["横折", "横", "竖弯钩"]),
        ("女", &["撇点", "撇", "横"]),
        ("子", &["横撇", "竖钩", "横"]),
        ("口", &["竖", "横折", "横"]),
        ("日", &["竖", "横折", "横", "横"]),
        ("月", &["撇", "横折钩", "横", "横"]),
        ("水", &["竖钩", "横撇", "撇", "捺"]),
        ("火", &["点", "撇", "撇", "捺"]),
        ("木", &["横", "竖", "撇", "捺"]),
        ("本", &["横", "竖", "撇", "捺", "横"]),
        ("中", &["竖", "横折", "横", "竖"]),
        ("天", &["横", "横", "撇", "捺"]),
        ("夫", &["横", "横", "撇", "捺"]),
        ("王", &["横", "横", "竖", "横"]),
        ("元", &["横", "横", "撇", "竖弯钩"]),
        ("心", &["点", "弯钩", "点", "点"]),
        ("手", &["撇", "横", "横", "竖钩"]),
        ("文", &["点", "横", "撇", "捺"]),
        ("方", &["点", "横", "横折钩", "撇"]),
        ("合", &["撇", "捺", "横", "竖", "横折", "横"]),
        ("同", &["竖", "横折钩", "横", "竖", "横折", "横"]),
        ("国", &["竖", "横折", "横", "横", "竖", "横", "点"]),
        ("在", &["横", "撇", "竖", "横", "竖", "横"]),
        ("有", &["横", "撇", "竖", "横折钩", "横", "横"]),
        ("来", &["横", "点", "撇", "横", "竖", "撇", "捺"]),
        ("你", &["撇", "竖", "撇", "横撇", "竖钩", "撇", "点"]),
        ("我", &["撇", "横", "竖钩", "提", "斜钩", "撇", "点"]),
        ("他", &["撇", "竖", "横折钩", "竖", "竖弯钩"]),
        ("她", &["撇点", "撇", "横", "横折钩", "竖", "竖弯钩"]),
        ("好", &["撇点", "撇", "横", "横撇", "竖钩", "横"]),
        ("囡", &["竖", "横折", "横", "撇点", "撇", "横"]),
        ("囝", &["竖", "横折", "横", "竖钩", "横"]),
        ("命", &["撇", "捺", "横", "竖", "横折", "横", "横", "竖"]),
    ];
    for &(ch, names) in strokes {
        if ch.chars().next()? == c {
            let ops: Vec<StrokeOp> = names.iter()
                .filter_map(|n| StrokeOp::from_stroke_name(n))
                .collect();
            if !ops.is_empty() {
                return Some(ops);
            }
        }
    }
    None
}

// ─────────────────────────────────────────────────────────────────────────────
// 汉字程序 · 一序列笔画 = 一个程序 = 一个数据库事务
// ─────────────────────────────────────────────────────────────────────────────

pub struct HanziProgram {
    pub character: char,
    pub strokes: Vec<StrokeOp>,
    pub tone: TonePhase,
    pub entry: ContainerDatabase,    // 程序入口 = 容器数据库
}

impl HanziProgram {
    pub fn new(character: char, strokes: Vec<StrokeOp>, tone: TonePhase) -> Self {
        let first_container = strokes.first().map(|s| s.container()).unwrap_or(Container::Dai);
        Self {
            character,
            strokes,
            tone,
            entry: ContainerDatabase::new(first_container),
        }
    }

    pub fn from_char(c: char, tone: TonePhase) -> Option<Self> {
        let strokes = get_stroke_sequence(c)?;
        Some(Self::new(c, strokes, tone))
    }

    /// 获取程序的所有容器数据库（每个笔画对应一个容器数据库）
    pub fn databases(&self) -> Vec<ContainerDatabase> {
        self.strokes.iter().map(|s| ContainerDatabase::new(s.container())).collect()
    }

    /// 执行程序（直接操作容器数据库）
    pub fn execute(&self) -> ProgramResult {
        let mut accumulator = ContainerAddress::default();
        let mut polarity = Polarity::Yuan;
        let mut databases = Vec::new();

        for stroke in &self.strokes {
            // 笔画位移直接积累到地址
            accumulator = accumulator.fold(stroke.displacement());
            // 极性折叠
            polarity = polarity.fold(stroke.polarity());
            // 获取对应容器数据库
            databases.push(ContainerDatabase::new(stroke.container()));
        }

        // 声调相位偏移
        accumulator = accumulator.fold(self.tone.coord_offset());

        // 数值 = 地址 + 极性（直接就是字节码）
        let value = ContainerValue::from_address(accumulator, polarity);

        ProgramResult {
            final_address: accumulator,
            final_polarity: polarity,
            value,
            databases,
            verdict: if polarity == Polarity::Yuan || polarity == Polarity::Yang {
                Verdict::Recognized
            } else {
                Verdict::Recognized // 阴也是被承认的状态
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Recognized = 0,
    Rejected  = 1,
}

#[derive(Clone, Debug)]
pub struct ProgramResult {
    pub final_address: ContainerAddress,
    pub final_polarity: Polarity,
    pub value: ContainerValue,
    pub databases: Vec<ContainerDatabase>,
    pub verdict: Verdict,
}

// ─────────────────────────────────────────────────────────────────────────────
// 部首命名空间 · 214 部首 → 坐标段
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Radical {
    pub index: u8,
    pub coord_base: ContainerAddress,
}

impl Radical {
    pub fn from_index(index: u8) -> Option<Self> {
        if index == 0 || index > 214 {
            return None;
        }
        let i = index - 1;
        Some(Self {
            index,
            coord_base: ContainerAddress {
                x: ((i / 8) % 8) as i8,
                y: (i % 8) as i8,
                z: ((i / 64) % 8) as i8,
            },
        })
    }

    pub fn from_char(c: char) -> Option<Self> {
        let idx = match c {
            '一' => 1, '丨' => 2, '丶' => 3, '丿' => 4, '乙' => 5, '亅' => 6,
            '二' => 7, '亠' => 8, '人' => 9, '儿' => 10, '入' => 11, '八' => 12,
            '冂' => 13, '冖' => 14, '冫' => 15, '几' => 16, '凵' => 17, '刀' => 18,
            '力' => 19, '勹' => 20, '匕' => 21, '匚' => 22, '十' => 23, '卜' => 24,
            '厂' => 25, '口' => 30, '囗' => 31, '土' => 32, '夂' => 34, '夕' => 35,
            '大' => 37, '女' => 38, '子' => 39, '宀' => 40, '寸' => 41, '小' => 42,
            '山' => 46, '工' => 48, '巾' => 50, '广' => 53, '彳' => 60, '心' => 61,
            '戈' => 62, '日' => 72, '月' => 74, '木' => 75, '水' => 85, '火' => 86,
            '犬' => 94, '王' => 96, '目' => 109, '立' => 117, '竹' => 118, '糸' => 120,
            '耳' => 122, '言' => 149, '走' => 156, '足' => 157, '金' => 167, '門' => 168,
            '雨' => 173, '食' => 184, '馬' => 187,
            _ => return None,
        };
        Self::from_index(idx)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 虚拟机 · 容器数据库管理器
// ─────────────────────────────────────────────────────────────────────────────

pub struct ContainerVM {
    pub address: ContainerAddress,
    pub polarity: Polarity,
    pub databases: Vec<ContainerDatabase>,
    pub soul_state: SoulState,
    pub sovereignty: Sovereignty,
}

impl ContainerVM {
    pub fn new() -> Self {
        Self {
            address: ContainerAddress::default(),
            polarity: Polarity::Yuan,
            databases: Vec::new(),
            soul_state: SoulState::default(),
            sovereignty: Sovereignty::Neutral,
        }
    }

    /// 执行汉字程序
    pub fn execute(&mut self, program: &HanziProgram) -> ProgramResult {
        let mut result = program.execute();
        self.address = self.address.fold(result.final_address);
        self.polarity = self.polarity.fold(result.final_polarity);
        self.databases.append(&mut result.databases);
        result
    }

    /// 创建新的容器数据库
    pub fn create_container(&mut self, name: Container) -> &mut ContainerDatabase {
        let db = ContainerDatabase::new(name);
        self.databases.push(db);
        self.databases.last_mut().unwrap()
    }

    /// 加载所有预加载数据库
    pub fn preload_all(&mut self) {
        for db in &mut self.databases {
            if !db.preloaded {
                db.preload();
            }
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
    fn test_container_geometry_euler() {
        let geo = ContainerGeometry { vertices: 4, edges: 4, faces: 1, genus: 0 };
        // V - E + F = 4 - 4 + 1 = 1 (开放平面)
        assert_eq!(geo.euler_char(), 1);
        assert!(geo.is_orientable());
    }

    #[test]
    fn test_container_database_new() {
        let mut db = ContainerDatabase::new(Container::Dai);
        assert_eq!(db.name, Container::Dai);
        // 袋：V=4 E=4 F=1
        assert_eq!(db.geometry.vertices, 4);
        // 未预加载时 load() 会自动预加载
        let data = db.load();
        assert!(db.preloaded);
        assert_eq!(data[0], Container::Dai as u8);
    }

    #[test]
    fn test_container_value_roundtrip() {
        let addr = ContainerAddress::new(3, 5, 7);
        let val = ContainerValue::from_address(addr, Polarity::Yang);
        assert_eq!(val.address().x, 3);
        assert_eq!(val.address().y, 5);
        assert_eq!(val.address().z, 7);
        assert_eq!(val.polarity(), Polarity::Yang);
    }

    #[test]
    fn test_container_value_fold() {
        let a = ContainerValue::from_address(ContainerAddress::new(10, 20, 0), Polarity::Yin);
        let b = ContainerValue::from_address(ContainerAddress::new(5, 5, 0), Polarity::Yang);
        let folded = a.fold(b);
        assert_eq!(folded.address().x, 15);  // 10 + 5
        assert_eq!(folded.address().y, 25);  // 20 + 5
        assert_eq!(folded.polarity(), Polarity::Yang); // 阴 + 阳 = 阳
    }

    #[test]
    fn test_container_value_as_container() {
        // Container::Dai = 0x00
        let val = ContainerValue(Container::Dai as u64);
        assert_eq!(val.as_container(), Some(Container::Dai));
    }

    #[test]
    fn test_polarity_fold() {
        assert_eq!(Polarity::Yin.fold(Polarity::Yang), Polarity::Yang);
        assert_eq!(Polarity::Yuan.fold(Polarity::Yuan), Polarity::Yang);
        assert_eq!(Polarity::Yang.fold(Polarity::Yang), Polarity::Yin);
    }

    #[test]
    fn test_stroke_displacement() {
        let addr = StrokeOp::Heng.displacement();
        assert_eq!(addr.x, 1);
        assert_eq!(addr.y, 0);
    }

    #[test]
    fn test_hanzi_program_execute() {
        let mut vm = ContainerVM::new();
        // "人" = 撇 + 捺
        let program = HanziProgram::from_char('人', TonePhase::YinPing).unwrap();
        let result = vm.execute(&program);
        assert_eq!(result.verdict, Verdict::Recognized);
        // 撇(1,-1,0) + 捺(-1,-1,0) = (0,-2,0)
        assert_eq!(result.final_address.x, 0);
        assert_eq!(result.final_address.y, -2);
        // 每个笔画对应的容器数据库被创建
        assert_eq!(result.databases.len(), 2);
    }

    #[test]
    fn test_container_preloaded_database() {
        let mut vm = ContainerVM::new();
        let db = vm.create_container(Container::He); // 盒
        // 盒: V=8 E=12 F=6
        assert_eq!(db.geometry.vertices, 8);
        db.preload();
        let data = &db.data;
        assert_eq!(data[0], Container::He as u8); // 名称符号
        assert_eq!(data[1], 8);  // vertices
        assert_eq!(data[2], 12); // edges
        assert_eq!(data[3], 6);  // faces
    }

    #[test]
    fn test_radical_mapping() {
        let r = Radical::from_char('人').unwrap();
        assert_eq!(r.index, 9);
        assert_eq!(r.coord_base.x, 1);
        assert_eq!(r.coord_base.y, 0);
    }

    #[test]
    fn test_stroke_name_roundtrip() {
        for name in &["横", "竖", "撇", "捺", "点", "提", "横折"] {
            let op = StrokeOp::from_stroke_name(name).unwrap();
            assert_eq!(op.name(), *name);
        }
    }

    #[test]
    fn test_get_stroke_sequence() {
        let strokes = get_stroke_sequence('人').unwrap();
        assert_eq!(strokes.len(), 2);
        assert_eq!(strokes[0], StrokeOp::Pie);
        assert_eq!(strokes[1], StrokeOp::Na);
    }

    #[test]
    fn test_container_address_to_index() {
        let addr = ContainerAddress::new(3, 5, 0);
        let idx = addr.as_index();
        // x=3 → 0b11 << 2 = 0b1100
        // y=5 → 0b101 >> 2 = 0b01
        // z=0
        assert_eq!(idx & 0x0F, 0x0C | 0x01); // 0b1101 = 13
    }
}
