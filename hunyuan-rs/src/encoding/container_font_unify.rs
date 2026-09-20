// ══════════════════════════════════════════════════════════════════════
// container_font_unify.rs — 容器=字体=时间 三元统一引擎 (v0.48)
// ══════════════════════════════════════════════════════════════════════
//
//   「同一个编码有三种躯体 — 容器躯体 · 字体躯体 · 时间躯体」
//
//   三元统一引擎不转换编码 — 它让一个编码以三种形态同时存在:
//
//     容器编码 (u64 签名)
//       ↓ 直接解读
//     三副躯体:
//       ┌─────────────────────────────────────────────────────────┐
//       │ 容器躯体 (Container Body)                                │
//       │   64 进制坐标 · 极性 · 主权 · 价值                       │
//       ├─────────────────────────────────────────────────────────┤
//       │ 字体躯体 (Font Body)                                     │
//       │   笔画参数 · 几何位置 · 渲染色彩                          │
//       ├─────────────────────────────────────────────────────────┤
//       │ 时间躯体 (Time Body)                                     │
//       │   时相位 · 生命周期状态 · 事件序号                        │
//       └─────────────────────────────────────────────────────────┘
//
//   解读规则 (非转换):
//     - 容器躯体: 编码按 64 进制分层 → (x,y) 蜂窝坐标 + 群码
//     - 字体躯体: 编码的低位 → 笔画类型/数目; 高位 → 色彩相位
//     - 时间躯体: 编码对时间轴取模 → 相位序号 → 生命周期状态
//
//   设计约束:
//     - 不复制编码 — 是"直接读取不同维度"
//     - 不引入新类型 — 三个躯体只读, 不修改编码
//

// ────────────────────────────────────────────────────────────
// 容器躯体: 直接从编码解读
// ────────────────────────────────────────────────────────────

/// 容器坐标 (从编码解读)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContainerBody {
    pub x: u8,
    pub y: u8,
    pub group: u8,
    pub index: u8,
    pub polarity: u8,
    pub sovereignty: u8,
}

impl ContainerBody {
    pub fn decode(signature: u64) -> Self {
        let lo = signature as u8;
        let x = ((lo & 0b0101010) >> 1) | ((lo & 0b1000000) >> 4);
        let y = (lo & 0b0010101) | ((lo & 0b1000000) >> 5);

        Self {
            x: x & 0b111,
            y: y & 0b111,
            group: (lo >> 4) & 0b11,
            index: lo & 0b1111,
            polarity: (lo >> 6) & 1,
            sovereignty: (lo >> 7) & 1,
        }
    }

    pub fn container_code(&self) -> u8 {
        (self.group << 4) | self.index
    }

    pub fn is_wife_side(&self) -> bool { self.polarity == 0 }
    pub fn is_full_authority(&self) -> bool { self.sovereignty == 0 }
}

// ────────────────────────────────────────────────────────────
// 字体躯体: 编码 → 渲染参数
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StrokeForm {
    Horizontal, Vertical, LeftFalling, RightFalling,
    Rising, Hook, Turning, Dot,
}

impl StrokeForm {
    pub fn from_bits(bits: u8) -> Self {
        match bits & 0b111 {
            0 => Self::Horizontal,
            1 => Self::Vertical,
            2 => Self::LeftFalling,
            3 => Self::RightFalling,
            4 => Self::Rising,
            5 => Self::Hook,
            6 => Self::Turning,
            _ => Self::Dot,
        }
    }

    pub fn path_hint(&self) -> &'static str {
        match self {
            Self::Horizontal => "M0,4 L8,4",
            Self::Vertical => "M4,0 L4,8",
            Self::LeftFalling => "M7,0 L1,8",
            Self::RightFalling => "M1,0 L7,8",
            Self::Rising => "M2,6 L6,2",
            Self::Hook => "M4,0 L4,4 L6,6",
            Self::Turning => "M2,0 L6,0 L6,4 L2,4",
            Self::Dot => "M4,3 L5,4 L4,5 L3,4 Z",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContainerShape {
    Square, Circle, Triangle, Hexagon,
}

impl ContainerShape {
    pub fn from_polarity_sovereignty(polarity: u8, sovereignty: u8) -> Self {
        match (polarity, sovereignty) {
            (0, 0) => Self::Square,
            (0, 1) => Self::Circle,
            (1, 0) => Self::Triangle,
            _       => Self::Hexagon,
        }
    }

    pub fn glyph_symbol(&self) -> &'static str {
        match self {
            Self::Square => "□",
            Self::Circle => "○",
            Self::Triangle => "△",
            Self::Hexagon => "⬡",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FontBody {
    pub primary_stroke: StrokeForm,
    pub stroke_count: u8,
    pub phase_color: u8,
    pub shape: ContainerShape,
}

impl FontBody {
    pub fn decode(signature: u64) -> Self {
        let lo = signature as u8;
        let hi = (signature >> 8) as u8;

        let primary_stroke = StrokeForm::from_bits(lo & 0b111);
        let stroke_count_raw = (lo >> 3) & 0b111;
        let stroke_count = if stroke_count_raw == 0 { 1 } else { stroke_count_raw + 1 };
        let phase_color = hi;
        let polarity = (lo >> 6) & 1;
        let sovereignty = (lo >> 7) & 1;
        let shape = ContainerShape::from_polarity_sovereignty(polarity, sovereignty);

        Self { primary_stroke, stroke_count, phase_color, shape }
    }

    pub fn color_hex(&self) -> String {
        let h = self.phase_color;
        let r = h;
        let g = h.wrapping_mul(3);
        let b = h.wrapping_mul(7);
        format!("#{:02x}{:02x}{:02x}", r & 0x3F << 2, g & 0x3F << 2, b & 0x3F << 2)
    }
}

// ────────────────────────────────────────────────────────────
// 时间躯体: 编码 → 时间相位
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LifePhase {
    Genesis, Growth, Flourish, Collapse, Dormant, Radiation,
}

impl LifePhase {
    pub fn from_bits(bits: u8) -> Self {
        match bits & 0b111 {
            0 => Self::Genesis,
            1 => Self::Growth,
            2 => Self::Flourish,
            3 => Self::Collapse,
            4 => Self::Dormant,
            _ => Self::Radiation,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Genesis => "创世",
            Self::Growth => "生长",
            Self::Flourish => "耀动",
            Self::Collapse => "坍缩",
            Self::Dormant => "休眠",
            Self::Radiation => "辐射",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TimeBody {
    pub phase: u8,
    pub lifecycle: LifePhase,
    pub temporal_layer: u8,
}

impl TimeBody {
    pub fn decode(signature: u64) -> Self {
        let phase = (signature & 0x3F) as u8;
        let lifecycle = LifePhase::from_bits((signature >> 3) as u8 & 0b111);
        let temporal_layer = ((signature >> 16) & 0xFF) as u8 % 8;
        let temporal_layer = if temporal_layer == 0 { 1 } else { temporal_layer };

        Self { phase, lifecycle, temporal_layer }
    }

    pub fn is_active(&self) -> bool { self.lifecycle != LifePhase::Dormant }
    pub fn is_critical(&self) -> bool {
        self.lifecycle == LifePhase::Genesis || self.lifecycle == LifePhase::Collapse
    }
}

// ────────────────────────────────────────────────────────────
// 三元统一解读
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Trinity {
    pub container: ContainerBody,
    pub font: FontBody,
    pub time: TimeBody,
}

impl Trinity {
    pub fn decode(signature: u64) -> Self {
        Self {
            container: ContainerBody::decode(signature),
            font: FontBody::decode(signature),
            time: TimeBody::decode(signature),
        }
    }

    pub fn is_coherent(&self) -> bool {
        match self.container.polarity {
            0 => self.time.phase < 32,
            _ => self.time.phase >= 32,
        }
    }

    pub fn is_present(&self) -> bool { self.time.is_active() }

    pub fn sovereignty_temporal_match(&self) -> bool {
        match self.container.sovereignty {
            0 => self.time.temporal_layer <= 4,
            _ => self.time.temporal_layer > 4,
        }
    }
}

// ────────────────────────────────────────────────────────────
// 三元统一引擎
// ────────────────────────────────────────────────────────────

pub struct UnifyEngine {
    pub trinities: Vec<(u64, Trinity)>,
}

impl UnifyEngine {
    pub fn new() -> Self { Self { trinities: Vec::new() } }

    pub fn decode(&mut self, signature: u64) -> &Trinity {
        let t = Trinity::decode(signature);
        self.trinities.push((signature, t));
        self.trinities.last().map(|(_, t)| t).unwrap()
    }

    pub fn decode_batch(&mut self, signatures: &[u64]) {
        for &sig in signatures {
            self.trinities.push((sig, Trinity::decode(sig)));
        }
    }

    pub fn coherent_set(&self) -> Vec<u64> {
        self.trinities.iter()
            .filter(|(_, t)| t.is_coherent())
            .map(|(sig, _)| *sig)
            .collect()
    }

    pub fn present_set(&self) -> Vec<u64> {
        self.trinities.iter()
            .filter(|(_, t)| t.is_present())
            .map(|(sig, _)| *sig)
            .collect()
    }

    pub fn sovereignty_matches(&self) -> Vec<u64> {
        self.trinities.iter()
            .filter(|(_, t)| t.sovereignty_temporal_match())
            .map(|(sig, _)| *sig)
            .collect()
    }

    pub fn stats(&self) -> FontUnifyStats {
        let total = self.trinities.len();
        FontUnifyStats {
            total,
            coherent: self.trinities.iter().filter(|(_, t)| t.is_coherent()).count(),
            present: self.trinities.iter().filter(|(_, t)| t.is_present()).count(),
            sovereignty_match: self.trinities.iter().filter(|(_, t)| t.sovereignty_temporal_match()).count(),
        }
    }

    pub fn get(&self, signature: u64) -> Option<&Trinity> {
        self.trinities.iter()
            .find(|(sig, _)| *sig == signature)
            .map(|(_, t)| t)
    }
}

impl Default for UnifyEngine {
    fn default() -> Self { Self::new() }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FontUnifyStats {
    pub total: usize,
    pub coherent: usize,
    pub present: usize,
    pub sovereignty_match: usize,
}

// ────────────────────────────────────────────────────────────
// 便捷函数
// ────────────────────────────────────────────────────────────

pub fn trinity_of(signature: u64) -> Trinity { Trinity::decode(signature) }
pub fn is_coherent(signature: u64) -> bool { Trinity::decode(signature).is_coherent() }

pub fn unify_batch(signatures: &[u64]) -> Vec<(u64, Trinity)> {
    signatures.iter().map(|&sig| (sig, Trinity::decode(sig))).collect()
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_body_decode() {
        let body = ContainerBody::decode(0b00010101);
        assert_eq!(body.index, 5);
        assert_eq!(body.group, 1);
        assert_eq!(body.polarity, 0);
        assert_eq!(body.sovereignty, 0);
        assert!(body.is_wife_side());
        assert!(body.is_full_authority());
    }

    #[test]
    fn test_stroke_form_from_bits() {
        assert_eq!(StrokeForm::from_bits(0), StrokeForm::Horizontal);
        assert_eq!(StrokeForm::from_bits(1), StrokeForm::Vertical);
        assert_eq!(StrokeForm::from_bits(7), StrokeForm::Dot);
    }

    #[test]
    fn test_container_shape_mapping() {
        assert_eq!(ContainerShape::from_polarity_sovereignty(0, 0), ContainerShape::Square);
        assert_eq!(ContainerShape::from_polarity_sovereignty(0, 1), ContainerShape::Circle);
        assert_eq!(ContainerShape::from_polarity_sovereignty(1, 0), ContainerShape::Triangle);
        assert_eq!(ContainerShape::from_polarity_sovereignty(1, 1), ContainerShape::Hexagon);
    }

    #[test]
    fn test_font_body_decode() {
        let font = FontBody::decode(0x1234);
        assert!(font.stroke_count >= 1 && font.stroke_count <= 8);
    }

    #[test]
    fn test_life_phase_from_bits() {
        assert_eq!(LifePhase::from_bits(0), LifePhase::Genesis);
        assert_eq!(LifePhase::from_bits(3), LifePhase::Collapse);
        assert_eq!(LifePhase::from_bits(7), LifePhase::Radiation);
    }

    #[test]
    fn test_time_body_active() {
        let time = TimeBody::decode(0b00001000);
        assert!(time.is_active());
        let time = TimeBody::decode(0b00100000);
        assert!(!time.is_active());
    }

    #[test]
    fn test_time_body_critical() {
        let time = TimeBody::decode(0);
        assert!(time.is_critical());
    }

    #[test]
    fn test_trinity_decode() {
        let t = Trinity::decode(0x00FF00AA);
        assert!(t.container.index < 16);
        assert!(t.time.phase < 64);
        assert!(t.font.stroke_count >= 1);
    }

    #[test]
    fn test_trinity_coherence_wife_side() {
        let t = Trinity::decode(0x00);
        assert!(t.is_coherent());
    }

    #[test]
    fn test_trinity_coherence_husband_side() {
        let t = Trinity::decode(0b01000000);
        assert_eq!(t.container.polarity, 1);
        assert!(!t.is_coherent());
    }

    #[test]
    fn test_trinity_present() {
        let t = Trinity::decode(0b00001001);
        assert!(t.is_present());
    }

    #[test]
    fn test_unify_engine_decode_batch() {
        let mut engine = UnifyEngine::new();
        // 0x20 → Dormant (inactive), others active → 4 present
        engine.decode_batch(&[0x10, 0x20, 0x30, 0x40, 0x50]);
        assert_eq!(engine.trinities.len(), 5);
        assert_eq!(engine.present_set().len(), 4);
    }

    #[test]
    fn test_unify_stats() {
        let mut engine = UnifyEngine::new();
        engine.decode_batch(&[0x00, 0x08, 0x10, 0x40]);
        let stats = engine.stats();
        assert_eq!(stats.total, 4);
        assert!(stats.present <= 4);
    }

    #[test]
    fn test_sovereignty_temporal_match() {
        let t = Trinity::decode(0x0200);
        assert!(t.sovereignty_temporal_match());
    }

    #[test]
    fn test_trinity_of() {
        let t = trinity_of(0xABCDEF);
        assert!(t.container.group < 4);
        assert!(!t.time.lifecycle.as_str().is_empty());
    }

    #[test]
    fn test_is_coherent_fn() {
        assert!(is_coherent(0x00));
        assert!(!is_coherent(0b01000000));
    }

    #[test]
    fn test_batch_unify() {
        let results = unify_batch(&[1, 2, 3]);
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_font_color_hex() {
        let font = FontBody::decode(0x00);
        let color = font.color_hex();
        assert!(color.starts_with('#'));
    }

    #[test]
    fn test_glyph_symbol() {
        assert_eq!(ContainerShape::Square.glyph_symbol(), "□");
        assert_eq!(ContainerShape::Circle.glyph_symbol(), "○");
        assert_eq!(ContainerShape::Triangle.glyph_symbol(), "△");
        assert_eq!(ContainerShape::Hexagon.glyph_symbol(), "⬡");
    }
}
