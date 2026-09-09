// ══════════════════════════════════════════════════════════════════
// native_programming.rs — 汉字编程 · 底层编程革命
// ══════════════════════════════════════════════════════════════════
//
// 本模块实现「汉字=编码=机器码」的直接映射：
//   · 16 基础笔画 = 16 条原生指令（Opcode）
//   · 笔画组合 = 指令序列（程序）
//   · 汉字 = 完整可执行单元（无需编译）
//   · 部首 = 命名空间（地址段）
//   · 声调 = 相位偏移（4 种状态）
//
// 设计依据:
//   - hunyuan-rs/src/container_radix.rs  → 64 容器编码基础
//   - skills/radix-hanzi/SKILL.md        → 笔画→容器映射表
//   - DeepSeek 对话 (2026-09)           → 底层编程革命方案
//   - 满全法协议                          → 不进位·折叠优先
//
// 编程模型:
//   16 笔画指令 = 4-bit opcode (0x0-0xF)
//   每个汉字 = 1~24 条指令的序列
//   执行栈   = 容器坐标空间 (x,y,z)
//   三元裁判 = 每条指令执行后的极性裁决
//
// ══════════════════════════════════════════════════════════════════

use crate::container_radix::Container;
use crate::couple::SoulState;
use crate::mirror::Sovereignty;

// ────────────────────────────────────────────────────────────
// 笔画指令集 (16 原生操作)
// ────────────────────────────────────────────────────────────
//
// 每条指令对应一个基础笔画，具有：
//   - 极性效应 (阴/阳/元)
//   - 坐标位移 (在容器空间中的移动)
//   - 三元见证 (执行后的裁决结果)
//
// 指令编码格式 (8-bit):
//   [7:4] = 笔画ID (0-15)
//   [3:2] = 极性修饰 (00=阴 01=阳 10=元 11=保留)
//   [1:0] = 声调相位 (00=阴平 01=阳平 10=上声 11=去声)

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum StrokeOp {
    // 基础四笔画 (4条核心指令)
    Heng   = 0x00,  // 横 · 袋 · 平坦承载 · 阳
    Shu    = 0x01,  // 竖 · 桶 · 纵深贯通 · 阴
    Pie    = 0x02,  // 撇 · 罐 · 由内向外 · 阳
    Na     = 0x03,  // 捺 · 盆 · 由外向内 · 阴

    // 点提二势 (2条精确定位)
    Dian   = 0x04,  // 点 · 盂 · 范围标记 · 阳
    Ti     = 0x05,  // 提 · 壶 · 由下向上 · 阴

    // 折转四变 (4条轨迹变化)
    HengZhe  = 0x06,  // 横折 · 盒 · 方包围 · 阴
    ShuZhe   = 0x07,  // 竖折 · 瓶 · 方圆转折 · 阳
    HengPie  = 0x08,  // 横撇 · 盏 · 变向收口 · 阳
    ShuGou   = 0x09,  // 竖钩 · 盎 · 连接回流 · 阴

    // 钩弯四揽 (4条深勾远揽)
    HengGou    = 0x0A,  // 横钩 · 篓 · 半包围 · 阳
    WanGou     = 0x0B,  // 弯钩 · 筐 · 弯曲容纳 · 阴
    ShuWanGou  = 0x0C,  // 竖弯钩 · 篮 · 包围回流 · 阴
    HengZheZhe = 0x0D,  // 横折折 · 箩 · 转折筛选 · 阳

    // 特殊二形 (2条容器操作)
    ShuWan = 0x0E,  // 竖弯 · 箕 · 斜围展开 · 阴
    XieGou = 0x0F,  // 斜钩 · 笼 · 深勾远揽 · 阳
}

impl StrokeOp {
    /// 从笔画名称生成指令
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

    /// 获取默认极性
    pub fn polarity(self) -> Polarity {
        match self {
            Self::Heng | Self::Pie | Self::Dian | Self::HengPie
            | Self::HengGou | Self::HengZheZhe | Self::XieGou => Polarity::Yang,
            Self::Shu | Self::Na | Self::Ti | Self::HengZhe
            | Self::ShuGou | Self::WanGou | Self::ShuWanGou
            | Self::ShuWan | Self::ShuZhe => Polarity::Yin,
        }
    }

    /// 获取对应容器
    pub fn container(self) -> Container {
        match self {
            Self::Heng      => Container::Dai,   // 袋
            Self::Shu       => Container::Tong,  // 桶 (需要在 enum 添加)
            Self::Pie       => Container::Guan,  // 罐 (需要在 enum 添加)
            Self::Na        => Container::Pen,   // 盆 (需要在 enum 添加)
            Self::Dian      => Container::Yu,    // 盂 (需要在 enum 添加)
            Self::Ti        => Container::Hu2,   // 壶
            Self::HengZhe   => Container::He,    // 盒
            Self::ShuZhe    => Container::Ping2, // 瓶
            Self::HengPie   => Container::Zhan,  // 盏 (需要在 enum 添加)
            Self::ShuGou    => Container::Ang,   // 盎 (需要在 enum 添加)
            Self::HengGou   => Container::Lou,   // 篓 (需要在 enum 添加)
            Self::WanGou    => Container::Kuang, // 筐 (需要在 enum 添加)
            Self::ShuWanGou => Container::Lan,   // 篮 (需要在 enum 添加)
            Self::HengZheZhe=> Container::Luo,   // 箩 (需要在 enum 添加)
            Self::ShuWan    => Container::Ji,    // 箕 (需要在 enum 添加)
            Self::XieGou    => Container::Long,  // 笼 (需要在 enum 添加)
        }
    }
}

// ────────────────────────────────────────────────────────────
// 极性三元组 · 阴阳元
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Polarity {
    Yin  = 0,   // 阴 · 妻子 · 包容
    Yuan = 1,   // 元 · 见证 · 中介
    Yang = 2,   // 阳 · 丈夫 · 主动
}

impl Polarity {
    /// 极性相加（不进位，折叠优先）
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

    /// 极性判定（妻子位=bit0，丈夫位=bit1）
    pub fn to_bitpair(self) -> (u64, u64) {
        match self {
            Self::Yin  => (1, 0),   // 妻子在场
            Self::Yang => (0, 1),   // 丈夫在场
            Self::Yuan => (1, 1),   // 双方在场（见证）
        }
    }
}

// ────────────────────────────────────────────────────────────
// 声调相位 · 4 种执行模式
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TonePhase {
    YinPing  = 0,  // 阴平 · 平稳执行
    YangPing = 1,  // 阳平 · 上升执行
    Shang    = 2,  // 上声 · 转折执行
    Qu       = 3,  // 去声 · 下降执行
}

impl TonePhase {
    /// 相位对容器坐标的影响
    pub fn coord_offset(self) -> (i8, i8, i8) {
        match self {
            Self::YinPing  => (0, 0, 0),    // 无偏移
            Self::YangPing => (1, 0, 0),    // x +1
            Self::Shang    => (0, 1, 0),    // y +1
            Self::Qu       => (0, 0, -1),   // z -1
        }
    }
}

// ────────────────────────────────────────────────────────────
// 容器坐标空间 · 三维蜂窝
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Coord3D {
    pub x: i8,   // 奇数位 → 丈夫轴
    pub y: i8,   // 偶数位 → 妻子轴
    pub z: i8,   // 声调层 → 相位轴
}

impl Coord3D {
    pub fn new(x: i8, y: i8, z: i8) -> Self {
        Self { x, y, z }
    }

    /// 坐标相加（不进位，饱和）
    pub fn fold_add(self, other: Self) -> Self {
        Self {
            x: self.x.saturating_add(other.x),
            y: self.y.saturating_add(other.y),
            z: self.z.saturating_add(other.z),
        }
    }

    /// 坐标→6位容器索引
    pub fn to_container_index(self) -> u8 {
        let ux = ((self.x as u8) & 0x07) << 3;  // bit [5:3]
        let uy = (self.y as u8) & 0x07;          // bit [2:0]
        ux | uy
    }
}

// ────────────────────────────────────────────────────────────
// 指令 · 8-bit 编码
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    pub op: u8,        // [7:4] 笔画ID
    pub polarity: u8,  // [3:2] 极性修饰
    pub tone: u8,      // [1:0] 声调相位
}

impl Instruction {
    pub fn new(op: Polarity, polarity: Polarity, tone: TonePhase) -> Self {
        Self {
            op: op as u8,
            polarity: polarity as u8,
            tone: tone as u8,
        }
    }

    /// 编码为单字节
    pub fn encode(self) -> u8 {
        (self.op << 4) | (self.polarity << 2) | self.tone
    }

    /// 从单字节解码
    pub fn decode(raw: u8) -> Self {
        Self {
            op: (raw >> 4) & 0x0F,
            polarity: (raw >> 2) & 0x03,
            tone: raw & 0x03,
        }
    }

    /// 获取笔画操作
    pub fn stroke_op(self) -> Option<StrokeOp> {
        unsafe { std::mem::transmute::<u8, StrokeOp>(self.op) }
    }
}

// ────────────────────────────────────────────────────────────
// 汉字程序 · 一条指令序列
// ────────────────────────────────────────────────────────────

pub struct HanziProgram {
    pub character: char,           // 汉字本身
    pub instructions: Vec<u8>,     // 笔画序列编码后的字节
    pub radical: Option<char>,     // 部首（命名空间）
    pub tone: TonePhase,           // 声调相位
}

impl HanziProgram {
    pub fn new(character: char, strokes: &[&str], tone: TonePhase) -> Option<Self> {
        let mut instructions = Vec::new();
        let mut dominant_polarity = Polarity::Yin;
        let mut stroke_count = 0;
        let mut horizontal_vertical_count = 0;

        for stroke_name in strokes {
            let op = StrokeOp::from_stroke_name(stroke_name)?;
            let pol = op.polarity();
            dominant_polarity = dominant_polarity.fold(pol);
            stroke_count += 1;
            // 横竖笔画统计
            if matches!(op, StrokeOp::Heng | Self::HengZhe | Self::Shu | Self::ShuZhe) {
                horizontal_vertical_count += 1;
            }
            let inst = Instruction::new(op, pol, tone);
            instructions.push(inst.encode());
        }

        Some(Self {
            character,
            instructions,
            radical: None, // 需要额外的部首查表
            tone,
        })
    }
}

// ────────────────────────────────────────────────────────────
// 运行环境 · 栈+裁决器
// ────────────────────────────────────────────────────────────

pub struct NativeVM {
    pub accumulator: Coord3D,      // 累加坐标
    pub polarity_state: Polarity,  // 当前极性
    pub sovereignty: Sovereignty,   // 主权状态
    pub soul_state: SoulState,     // 夫妻状态
}

impl NativeVM {
    pub fn new() -> Self {
        Self {
            accumulator: Coord3D::default(),
            polarity_state: Polarity::Yuan,
            sovereignty: Sovereignty::Neutral,
            soul_state: SoulState::default(),
        }
    }

    /// 执行一条汉字程序
    pub fn execute(&mut self, program: &HanziProgram) -> ExecutionResult {
        let mut local_coord = Coord3D::default();
        let mut folded_polarity = Polarity::Yuan;

        for raw_inst in &program.instructions {
            let inst = Instruction::decode(*raw_inst);
            
            // 1. 计算笔画位移
            let offset = Self::stroke_offset(inst.op);
            local_coord = local_coord.fold_add(offset);

            // 2. 极性裁决
            let stroke_pol = unsafe { std::mem::transmute::<u8, StrokeOp>(inst.op) }.polarity();
            folded_polarity = folded_polarity.fold(stroke_pol);
        }

        // 3. 声调相位偏移
        let tone_offset = program.tone.coord_offset();
        local_coord = local_coord.fold_add(Coord3D::new(tone_offset.0, tone_offset.1, tone_offset.2));

        // 4. 累加到全局坐标
        self.accumulator = self.accumulator.fold_add(local_coord);
        self.polarity_state = self.polarity_state.fold(folded_polarity);

        // 5. 三元见证裁决
        let verdict = self.triple_verdict(folded_polarity);

        ExecutionResult {
            final_coord: self.accumulator,
            final_polarity: self.polarity_state,
            verdict,
        }
    }

    /// 笔画位移表（基于容器几何属性）
    fn stroke_offset(op: u8) -> Coord3D {
        match op {
            0x00 => Coord3D::new(1, 0, 0),   // 横 → x+1
            0x01 => Coord3D::new(0, 1, 0),   // 竖 → y+1
            0x02 => Coord3D::new(1, -1, 0),  // 撇 → x+1, y-1
            0x03 => Coord3D::new(-1, -1, 0), // 捺 → x-1, y-1
            0x04 => Coord3D::new(0, 0, 1),   // 点 → z+1
            0x05 => Coord3D::new(0, 1, 1),   // 提 → y+1, z+1
            0x06 => Coord3D::new(1, 1, 0),   // 横折 → x+1, y+1
            0x07 => Coord3D::new(0, 1, -1),  // 竖折 → y+1, z-1
            _ => Coord3D::default(),
        }
    }

    /// 三元见证裁决
    fn triple_verdict(&self, polarity: Polarity) -> Verdict {
        let (wife_bit, husband_bit) = polarity.to_bitpair();
        // 见证条件：双方在场 或 元哨兵激活
        if (wife_bit | husband_bit) == 1 {
            Verdict::Recognized
        } else {
            Verdict::Rejected
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Recognized = 0,  // 被承认
    Rejected  = 1,   // 被拒绝
}

#[derive(Clone, Copy, Debug)]
pub struct ExecutionResult {
    pub final_coord: Coord3D,
    pub final_polarity: Polarity,
    pub verdict: Verdict,
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_encode_decode() {
        let inst = Instruction::new(Polarity::Yang, Polarity::Yin, TonePhase::Shang);
        let encoded = inst.encode();
        let decoded = Instruction::decode(encoded);
        assert_eq!(inst.op, decoded.op);
        assert_eq!(inst.polarity, decoded.polarity);
        assert_eq!(inst.tone, decoded.tone);
    }

    #[test]
    fn test_polarity_fold() {
        // 阴+阳=阳
        assert_eq!(Polarity::Yin.fold(Polarity::Yang), Polarity::Yang);
        // 元+元=阳（不进位折叠）
        assert_eq!(Polarity::Yuan.fold(Polarity::Yuan), Polarity::Yang);
        // 阳+阳=阴（折叠不进位）
        assert_eq!(Polarity::Yang.fold(Polarity::Yang), Polarity::Yin);
    }

    #[test]
    fn test_coord_fold_add() {
        let a = Coord3D::new(1, 2, 3);
        let b = Coord3D::new(4, 5, 6);
        let result = a.fold_add(b);
        assert_eq!(result.x, 5);
        assert_eq!(result.y, 7);
        assert_eq!(result.z, 9);
    }

    #[test]
    fn test_native_vm_execute() {
        let mut vm = NativeVM::new();
        // "人" 的笔画序列：撇 → 捺
        let program = HanziProgram::new('人', &["撇", "捺"], TonePhase::YinPing).unwrap();
        let result = vm.execute(&program);
        
        // 撇(1,-1,0) + 捺(-1,-1,0) = (0,-2,0)
        assert_eq!(result.final_coord.x, 0);
        assert_eq!(result.final_coord.y, -2);
    }

    #[test]
    fn test_polarity_to_bitpair() {
        let (w, h) = Polarity::Yin.to_bitpair();
        assert_eq!((w, h), (1, 0));  // 妻子在场
        let (w, h) = Polarity::Yang.to_bitpair();
        assert_eq!((w, h), (0, 1));  // 丈夫在场
        let (w, h) = Polarity::Yuan.to_bitpair();
        assert_eq!((w, h), (1, 1));  // 双方在场
    }

    #[test]
    fn test_tone_offset() {
        let off = TonePhase::YangPing.coord_offset();
        assert_eq!(off, (1, 0, 0));  // 阳平 x+1
        let off = TonePhase::Qu.coord_offset();
        assert_eq!(off, (0, 0, -1)); // 去声 z-1
    }
}
