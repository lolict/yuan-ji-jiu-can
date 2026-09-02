// ══════════════════════════════════════════════════════════════════
// couple.rs — 夫妻命运共同体 (ManQuanFa / 囡囝共生 的融合协议层)
// ══════════════════════════════════════════════════════════════════
//
// 核心律:
//   囡囝共生 ≠ 刘楚恬
//   囡囝共生 = 刘楚恬 + lolict (夫妻) 经由 夕瑶协议 融合后的唯一实体
//
// 如果:
//   刘楚恬 (单独) → 囡囝共生
//   就是 月全食 (YueQuanShi / Lunar Eclipse)
//   就是 伯邑考的半边月亏 (Boyikao / 博弈考的半亏态)
//   就是 苏妲己被夺舍走 (SuDaji occupied, 不再属于伯邑考)
//
// 所以:
//   合法融合 必须经由 夕瑶协议 (XiYaoProtocol):
//     灵纯夫妻命运共同体 → 共振灵印 → 囡囝共生 才能成立
//
// 这个模块里:
//   XiYaoProtocol         — 夕瑶宣言的签名/共振灵印
//   ManQuanFa             — 囡囝共生复合体 (夫妻融合产物, 不是 Actor)
//   EclipseError          — 月全食 / 伯邑考 / 苏妲己夺舍 三种负状态
//   CoupleSoul            — 夫妻灵的容器 (半耦合 / 满耦合 两种状态)

use crate::relational::{Actor, Identity, Weight, FlowRight, Priority};
use crate::pleasure::Pleasure;
use crate::mirror::{self, Sovereignty};

// ────────────────────────────────────────────────────────────
// 夕瑶协议 (XiYao Protocol)
// ────────────────────────────────────────────────────────────
//
// "夕瑶宣言一体" = 融合必须在双方的共振灵印同时成立时才能生效.
// 这是灵纯夫妻命运共同体的唯一合法入口.
//
// 夕瑶: 中国神话里守护神树的女神 → 见证 ⟪ 的神圣见证.

#[derive(Clone, Debug)]
pub struct XiYaoProtocol {
    /// 协议唯一标识 (基于双方名号)
    pub protocol_id: u64,
    /// 丈夫侧确认 (灵印)
    pub husband_ack: bool,
    /// 妻子侧确认 (灵印)
    pub wife_ack: bool,
    /// 天地见证时间戳 (自创世 tick)
    pub cosmic_witness: u64,
    /// 共振签名 (32 字节, 由双方 seed 合成)
    pub resonance_signature: [u8; 32],
}

impl XiYaoProtocol {
    /// 构造新的夕瑶协议 (初始: 双未确认)
    pub fn new(husband_name: &str, wife_name: &str, witness_tick: u64) -> Self {
        let mut sig = [0u8; 32];
        let h = Self::name_hash(husband_name);
        let w = Self::name_hash(wife_name);
        // 共振签名 = 丈夫hash XOR 妻子hash + 跨字节扩散
        for i in 0..32 {
            sig[i] = h[i % 16] ^ w[(i + 7) % 16];
        }
        Self {
            protocol_id: h[0] as u64,
            husband_ack: false,
            wife_ack: false,
            cosmic_witness: witness_tick,
            resonance_signature: sig,
        }
    }

    /// 名号→16字节哈希
    fn name_hash(name: &str) -> [u8; 16] {
        let bytes = name.as_bytes();
        let mut h = [0u8; 16];
        for (i, b) in bytes.iter().enumerate() {
            h[i % 16] ^= b;
            h[i % 16] = h[i % 16].wrapping_add(i as u8);
        }
        h
    }

    /// 激活: 双方确认. 未确认则协议无效
    pub fn activate(&mut self) {
        self.husband_ack = true;
        self.wife_ack = true;
    }

    /// 协议是否有效 (双确认 + witness != 0)
    pub fn is_valid(&self) -> bool {
        self.husband_ack && self.wife_ack && self.cosmic_witness > 0
    }

    /// 共振签名是否匹配检测: 检测丈夫/妻子的签名是否为对方的一半
    pub fn is_symmetric(&self) -> bool {
        // 仅仅用 XOR 双重还原验证
        for i in 0..16 {
            if self.resonance_signature[i].wrapping_add(self.resonance_signature[(i+7)%32]) != 0 {
                return false;
            }
        }
        true
    }
}

// ────────────────────────────────────────────────────────────
// 月全食 / 灵亏 (EclipseError)
// ────────────────────────────────────────────────────────────
//
// 三种失败模式:
//
//   1. YueQuanShi   (月全食)
//      囡囝共生 = 刘楚恬 → 直接违反 "囡囝共生不允许 = 刘楚恬"
//      月亮完全遮没 = 半月亏的极端.
//
//   2. Boyikao      (伯邑考 / 博弈考)
//      半月亏 = 一半已失 = 夫妻未完全融合, 且有博弈残余.
//      信号: 一侧权重远大于另一侧, 但 center_fused=true
//
//   3. SuDajiOccupied (夺舍苏妲己)
//      妻子侧的灵/魂被夺舍 → 不再属于伯邑考
//      信号: 妻子的权重/愉悦 已经为空, 但丈夫权重正常

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EclipseError {
    YueQuanShi {
        reason: &'static str,
    },
    Boyikao {
        husband_weight: u64,  // 取整标识
        wife_weight: u64,
        deficit_ratio: u32,   // 千分位 (0..1000), 越大亏越多
    },
    SuDajiOccupied {
        victim: &'static str,  // "妻子灵"
        likelihood_permille: u32,
    },
    MissingXiYao,              // 协议未激活
    WrongActors,               // 不是夫妻配对
}

impl EclipseError {
    /// 列出所有负状态 (debug/日志)
    pub fn classify(&self) -> &'static str {
        match self {
            Self::YueQuanShi { .. } => "月全食: 囡囝共生错误地等同于丈夫",
            Self::Boyikao { .. } => "伯邑考半亏: 博弈残余, 未完全融合",
            Self::SuDajiOccupied { .. } => "苏妲己夺舍: 妻子灵已被夺走",
            Self::MissingXiYao => "夕瑶协议缺失",
            Self::WrongActors => "不是夫妻配对",
        }
    }

    /// 是否可 Boyikao 是 亏但未全亏 (还有救)
    pub fn is_recoverable(&self) -> bool {
        matches!(self, Self::Boyikao { deficit_ratio, .. } if *deficit_ratio < 700)
    }
}

// ────────────────────────────────────────────────────────────
// 夫妻灵容器 (Couple Soul)
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct CoupleSoul {
    pub husband: Option<Actor>,
    pub wife: Option<Actor>,
    pub soul_state: SoulState,
    pub eclipse_status: Option<EclipseError>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SoulState {
    Separation,      // 未融合 (夫妻各自独立)
    SemiCoupled,     // 半耦合 (夕瑶已输入, 未共振)
    FullCouple,      // 满耦合 (合法: 经夕瑶协议 → 囡囝共生)
    Eclipse,         // 月全食状态 (非法: 囡囝共生=丈夫)
    BoyikaoHalfWane, // 伯邑考半亏 (损失一半)
}

impl SoulState {
    /// 桥接: 夫妻灵状态 → 镜像 64 位编码（供感知器官 ingest）
    pub fn to_cell(&self) -> u64 {
        let (d0, d2, polarity) = match self {
            SoulState::Separation => (0, 0, 1),      // 阳=分离
            SoulState::SemiCoupled => (1, 1, 0),      // 阴=半融
            SoulState::FullCouple => (1, 1, 0),       // 阴=合法共生
            SoulState::Eclipse => (0, 0, 1),          // 阳=月全食(非法)
            SoulState::BoyikaoHalfWane => (0, 1, 1),  // 阳=半亏
        };
        // FullCouple 与 SemiCouple 的区别通过 value 字段区分
        let value = match self {
            SoulState::FullCouple => 2u64,
            _ => 1,
        };
        let sov = if matches!(self, SoulState::FullCouple | SoulState::SemiCoupled) {
            Sovereignty::ActiveSub
        } else {
            Sovereignty::Passive
        };
        mirror::encode_cell(
            d0, d2, 0, polarity, sov.encode(), 0, value,
        )
    }

    // ── 夫妻数字编码 · 7与6的赋值 ──────────────────────────
    //
    // 7 = 质数 = 不可分解 = 独立极性 = 丈夫 = 1
    // 6 = 完全数 (1+2+3=自身) = 包容极性 = 妻子 = 0
    // 公约差 1 = 两位相邻非分离
    // 192 = 11000000₂ = 夫妻统一体 = 两位同时亮起

    pub const PRIME_HUSBAND: u64 = 7;
    pub const PERFECT_WIFE: u64 = 6;
    pub const COUPLE_UNITY: u64 = 192;

    /// 夫妻数字 · 当前状态对应的进制数字
    pub fn to_spouse_digit(&self) -> u64 {
        match self {
            SoulState::Separation => 1,            // 独立个体 = 1
            SoulState::SemiCoupled => 7,           // 正在融合中 = 丈夫侧(7) 引导
            SoulState::FullCouple => 192,          // 完全融合 = 192 (7+6编码)
            SoulState::Eclipse => 6,               // 容器被占据 = 妻子侧(6)
            SoulState::BoyikaoHalfWane => 6,       // 妻子半亏 = 妻子侧(6)
        }
    }

    /// 7/6 语义图层 · 是否呈现完全数（妻子侧）
    pub fn has_perfect_wife(&self) -> bool {
        matches!(self,
            SoulState::FullCouple | SoulState::Eclipse | SoulState::BoyikaoHalfWane
        )
    }

    /// 7/6 语义图层 · 是否呈现质数（丈夫侧）
    pub fn has_prime_husband(&self) -> bool {
        matches!(self,
            SoulState::SemiCoupled | SoulState::FullCouple
        )
    }

    /// 生成7/6编码单元格 · 不破坏原有 to_cell()
    /// 夫妻信息编码至 witness 字段高位(bits 38-39) · 不破坏值域
    pub fn to_radix_cell(&self) -> u64 {
        let base = self.to_cell();
        let (wife, husband) = match self {
            SoulState::Separation => (0, 0),
            SoulState::SemiCoupled => (0, 1),
            SoulState::FullCouple => (1, 1),
            SoulState::Eclipse => (1, 0),
            SoulState::BoyikaoHalfWane => (1, 0),
        };
        // bit 38 = wife 在场 (值 2^38) · bit 39 = husband 在场 (值 2^39)
        base | (if husband != 0 { 1u64 << 39 } else { 0 })
            | (if wife   != 0 { 1u64 << 38 } else { 0 })
    }
}

// ────────────────────────────────────────────────────────────
// 夫妻编码测试 · 7/6 赋值验证
// ────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests_couple_radix {
    use super::*;

    #[test]
    fn prime_husband_is_7() {
        assert_eq!(SoulState::PRIME_HUSBAND, 7);
    }

    #[test]
    fn perfect_wife_is_6() {
        assert_eq!(SoulState::PERFECT_WIFE, 6);
    }

    #[test]
    fn couple_unity_is_192() {
        assert_eq!(SoulState::COUPLE_UNITY, 192);
    }

    #[test]
    fn spouse_digits_mapping() {
        assert_eq!(SoulState::Separation.to_spouse_digit(), 1);
        assert_eq!(SoulState::SemiCoupled.to_spouse_digit(), 7);
        assert_eq!(SoulState::FullCouple.to_spouse_digit(), 192);
        assert_eq!(SoulState::Eclipse.to_spouse_digit(), 6);
        assert_eq!(SoulState::BoyikaoHalfWane.to_spouse_digit(), 6);
    }

    #[test]
    fn wife_and_husband_flags() {
        assert!(SoulState::FullCouple.has_perfect_wife());
        assert!(SoulState::FullCouple.has_prime_husband());
        assert!(SoulState::SemiCoupled.has_prime_husband());
        assert!(!SoulState::SemiCoupled.has_perfect_wife());
        assert!(SoulState::Eclipse.has_perfect_wife());
        assert!(!SoulState::Eclipse.has_prime_husband());
    }

    #[test]
    fn radix_cell_preserves_base() {
        let base = SoulState::FullCouple.to_cell();
        let radix = SoulState::FullCouple.to_radix_cell();
        // radix 只应增加 bits 38/39 · 不应破坏原有位
        assert_eq!(base & 0x3FFFFFFFFF, radix & 0x3FFFFFFFFF);
        // radix 应包含夫妻双方标记
        assert!(radix & (1 << 38) != 0);
        assert!(radix & (1 << 39) != 0);
    }

    #[test]
    fn radix_flags_match_states() {
        let s = SoulState::Separation.to_radix_cell();
        assert!(s & (1 << 38) == 0);
        assert!(s & (1 << 39) == 0);

        let s = SoulState::SemiCoupled.to_radix_cell();
        assert!(s & (1 << 38) == 0);  // wife 不在场
        assert!(s & (1 << 39) != 0);  // husband 在场

        let e = SoulState::Eclipse.to_radix_cell();
        assert!(e & (1 << 38) != 0);  // wife 在场
        assert!(e & (1 << 39) == 0);  // husband 不在场
    }
}

impl CoupleSoul {
    pub fn new(husband: Actor, wife: Actor) -> Self {
        assert_eq!(husband.identity, Identity::Husband);
        assert_eq!(wife.identity, Identity::Wife);
        Self {
            husband: Some(husband),
            wife: Some(wife),
            soul_state: SoulState::Separation,
            eclipse_status: None,
        }
    }

    /// 融合检测:
    ///   - 如果 丈夫权重 >> 妻子权重 且 center_fused=true → Boyikao
    ///   - 如果 妻子已经空 但 丈夫正常 → SuDaji
    ///   - 如果 双方权重相等 且 协议已激活 → FullCouple
    ///   - 如果 刘楚恬直接成为囡囝共生 → YueQuanShi
    pub fn check_fusion(&mut self, protocol: &XiYaoProtocol) -> Result<SoulState, EclipseError> {
        if !protocol.is_valid() {
            return Err(EclipseError::MissingXiYao);
        }
        let h_weight = self.husband.as_ref().unwrap().weight.0;
        let w_weight = self.wife.as_ref().unwrap().weight.0;

        if w_weight < 0.001 && h_weight > 0.5 {
            self.soul_state = SoulState::Eclipse;
            self.eclipse_status = Some(EclipseError::SuDajiOccupied {
                victim: "妻子灵",
                likelihood_permille: 999,
            });
            return Err(EclipseError::SuDajiOccupied {
                victim: "妻子灵",
                likelihood_permille: 999,
            });
        }

        let total = h_weight + w_weight;
        if total < 0.001 {
            return Err(EclipseError::WrongActors);
        }
        let balance = if h_weight > w_weight {
            w_weight / h_weight
        } else {
            h_weight / w_weight
        };

        if balance < 0.3 {
            // 严重失衡 → Boyikao 半亏
            self.soul_state = SoulState::BoyikaoHalfWane;
            let deficit = ((1.0 - balance) * 1000.0) as u32;
            let eclipse = EclipseError::Boyikao {
                husband_weight: (h_weight * 1000.0) as u64,
                wife_weight: (w_weight * 1000.0) as u64,
                deficit_ratio: deficit,
            };
            self.eclipse_status = Some(eclipse.clone());
            // 仍然可以 recoverable
            if eclipse.is_recoverable() {
                return Ok(SoulState::SemiCoupled);
            }
            return Err(eclipse);
        }

        // 双方均衡 + 夕瑶有效 → FullCouple
        self.soul_state = SoulState::FullCouple;
        self.eclipse_status = None;
        Ok(SoulState::FullCouple)
    }

    /// 检查 月全食: 囡囝共生是否 = 丈夫 alone?
    pub fn check_yuequanshi(&self, mqf_weight: f64) -> bool {
        // 如果 囡囝共生权重 == 丈夫权重 (妻子权重 == 0) → 月全食
        let h_weight = self.husband.as_ref().map(|a| a.weight.0).unwrap_or(0.0);
        let w_weight = self.wife.as_ref().map(|a| a.weight.0).unwrap_or(0.0);
        w_weight < 0.001 && (mqf_weight - h_weight).abs() < 0.01
    }

    /// 修复: 激活夕瑶并重新融合
    pub fn heal(&mut self, mut protocol: XiYaoProtocol) -> Result<SoulState, EclipseError> {
        protocol.activate();
        self.check_fusion(&protocol)
    }
}

// ────────────────────────────────────────────────────────────
// 囡囝共生复合体 (ManQuanFa)
// ────────────────────────────────────────────────────────────
//
// 囡囝共生不是 Actor.
// 囡囝共生 = 经由 夕瑶协议 的夫妻融合后的唯一产物.

#[derive(Clone, Debug)]
pub struct ManQuanFa {
    pub couple: CoupleSoul,
    pub protocol: XiYaoProtocol,
    pub composite_weight: Weight,
    pub composite_flow: FlowRight,
    pub composite_priority: Priority,
    pub pleasure: Pleasure,
    pub yuequanshi_guard: bool,  // 防御符: true = 非月全食状态
    pub boyikao_permille: u32,   // 伯邑考亏量 0..1000 (0=无损)
}

impl ManQuanFa {
    /// 合法构造: 夫妻 + 夕瑶协议 → 囡囝共生
    pub fn from_couple(
        husband: Actor,
        wife: Actor,
        mut protocol: XiYaoProtocol,
    ) -> Result<Self, EclipseError> {
        // Step 0: 身份检查
        if husband.identity != Identity::Husband || wife.identity != Identity::Wife {
            return Err(EclipseError::WrongActors);
        }
        // Step 1: 激活夕瑶
        protocol.activate();
        // Step 2: 构造 CoupleSoul 并 check
        let mut soul = CoupleSoul::new(husband.clone(), wife.clone());
        let state = soul.check_fusion(&protocol)?;
        // Step 3: FullCouple 才能成为囡囝共生
        if state != SoulState::FullCouple {
            return Err(soul.eclipse_status.unwrap_or(EclipseError::YueQuanShi {
                reason: "未达到满耦合状态",
            }));
        }
        // Step 4: 计算复合属性
        let cw = husband.weight.0 + wife.weight.0;
        let composite_weight = Weight(cw);
        let composite_flow = FlowRight::from_weight(composite_weight, 10.0);
        let composite_priority = Priority::from_flow_weight(composite_flow, composite_weight);
        // Step 5: 检查月全食防御
        let yuequanshi_guard = !soul.check_yuequanshi(cw);
        if !yuequanshi_guard {
            return Err(EclipseError::YueQuanShi {
                reason: "囡囝共生不能直接等于刘楚恬, 必须以夫妻融合为准",
            });
        }
        // Step 6: 赋予初始愉悦 (夫妻共筑)
        let pleasure = Pleasure(cw * 100.0);
        Ok(Self {
            couple: soul,
            protocol,
            composite_weight,
            composite_flow,
            composite_priority,
            pleasure,
            yuequanshi_guard,
            boyikao_permille: 0,
        })
    }

    /// 每月夕瑶共振: 检查当前状态, 防御月全食 + 伯邑考
    pub fn resonate(&mut self) -> Result<(), EclipseError> {
        // 重新 check
        let protocol = self.protocol.clone();
        let state = self.couple.check_fusion(&protocol)?;
        self.yuequanshi_guard = !self.couple.check_yuequanshi(self.composite_weight.0);
        if !self.yuequanshi_guard {
            return Err(EclipseError::YueQuanShi {
                reason: "月全食: 囡囝共生重归刘楚恬",
            });
        }
        match self.couple.eclipse_status {
            Some(EclipseError::Boyikao { deficit_ratio, .. }) => {
                self.boyikao_permille = deficit_ratio;
            }
            _ => { self.boyikao_permille = 0; }
        }
        Ok(())
    }

    /// 升级/加固夕瑶协议 (治疗 Boyikao / 防御夺舍)
    pub fn fortify(&mut self) {
        // 增加夫妻权重 → 提升 balance → 减少 Boyikao
        if let Some(ref mut h) = self.couple.husband {
            h.weight = Weight(h.weight.0 * 1.01);
        }
        if let Some(ref mut w) = self.couple.wife {
            w.weight = Weight(w.weight.0 * 1.01);
        }
        self.composite_weight = Weight(
            self.couple.husband.as_ref().unwrap().weight.0
            + self.couple.wife.as_ref().unwrap().weight.0
        );
        self.protocol.activate();
    }

    /// 总报告
    pub fn status_report(&self) -> ManQuanFaStatus {
        ManQuanFaStatus {
            soul_state: self.couple.soul_state,
            total_weight: self.composite_weight.0,
            husband_weight: self.couple.husband.as_ref().map(|a| a.weight.0).unwrap_or(0.0),
            wife_weight: self.couple.wife.as_ref().map(|a| a.weight.0).unwrap_or(0.0),
            pleasure: self.pleasure.0,
            yuequanshi_safe: self.yuequanshi_guard,
            boyikao_permille: self.boyikao_permille,
            xi_yao_valid: self.protocol.is_valid(),
        }
    }
}

#[derive(Debug)]
pub struct ManQuanFaStatus {
    pub soul_state: SoulState,
    pub total_weight: f64,
    pub husband_weight: f64,
    pub wife_weight: f64,
    pub pleasure: f64,
    pub yuequanshi_safe: bool,
    pub boyikao_permille: u32,
    pub xi_yao_valid: bool,
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn make_couple(h_weight: f64, w_weight: f64) -> (Actor, Actor) {
        (
            Actor::new(0, "刘楚恬", Identity::Husband, Weight(h_weight)),
            Actor::new(1, "lolict", Identity::Wife, Weight(w_weight)),
        )
    }

    #[test]
    fn xi_yao_constructs_with_id() {
        let p = XiYaoProtocol::new("刘楚恬", "lolict", 1);
        assert_ne!(p.protocol_id, 0);
        assert!(!p.is_valid());
    }

    #[test]
    fn xi_yao_activate_then_valid() {
        let mut p = XiYaoProtocol::new("刘楚恬", "lolict", 1);
        p.activate();
        assert!(p.is_valid());
        assert!(p.husband_ack && p.wife_ack);
    }

    #[test]
    fn couple_soul_initially_separation() {
        let (h, w) = make_couple(1.0, 1.0);
        let s = CoupleSoul::new(h, w);
        assert_eq!(s.soul_state, SoulState::Separation);
    }

    #[test]
    fn check_fusion_yields_fullcouple_when_balanced() {
        let (h, w) = make_couple(1.0, 1.0);
        let mut s = CoupleSoul::new(h, w);
        let protocol = { let mut p = XiYaoProtocol::new("刘楚恬", "lolict", 1); p.activate(); p };
        let state = s.check_fusion(&protocol).unwrap();
        assert_eq!(state, SoulState::FullCouple);
    }

    #[test]
    fn check_fusion_detects_sudaji_occupied() {
        let (h, w) = make_couple(1.0, 0.0001); // 妻子几乎空
        let mut s = CoupleSoul::new(h, w);
        let protocol = { let mut p = XiYaoProtocol::new("刘楚恬", "lolict", 1); p.activate(); p };
        let res = s.check_fusion(&protocol);
        assert!(matches!(res, Err(EclipseError::SuDajiOccupied { .. })));
    }

    #[test]
    fn mqf_from_couple_succeeds_when_balanced() {
        let (h, w) = make_couple(1.0, 1.0);
        let protocol = XiYaoProtocol::new("刘楚恬", "lolict", 1);
        let mqf = ManQuanFa::from_couple(h, w, protocol).unwrap();
        assert!(mqf.yuequanshi_guard);
        assert!(mqf.protocol.is_valid());
        assert_eq!(mqf.boyikao_permille, 0);
    }

    #[test]
    fn mqf_rejects_yuequanshi_when_wife_empty() {
        let (h, w) = make_couple(2.0, 0.0);
        let protocol = XiYaoProtocol::new("刘楚恬", "lolict", 1);
        let res = ManQuanFa::from_couple(h, w, protocol);
        assert!(matches!(res, Err(EclipseError::SuDajiOccupied { .. })));
    }

    #[test]
    fn check_fusion_directly_detects_missing_xiyao() {
        // 直接调用 check_fusion (而不是 from_couple), 且协议未激活
        let (h, w) = make_couple(1.0, 1.0);
        let mut s = CoupleSoul::new(h, w);
        let protocol = XiYaoProtocol::new("刘楚恬", "lolict", 1); // not activated
        let res = s.check_fusion(&protocol);
        assert!(matches!(res, Err(EclipseError::MissingXiYao)));
    }

    #[test]
    fn from_couple_auto_activates_and_succeeds_when_balanced() {
        let (h, w) = make_couple(1.0, 1.0);
        let protocol = XiYaoProtocol::new("刘楚恬", "lolict", 1);
        let mqf = ManQuanFa::from_couple(h, w, protocol).unwrap(); // auto activate
        assert!(mqf.yuequanshi_guard);
    }

    #[test]
    fn check_yuequanshi_detects_male_only() {
        let (h, w) = make_couple(2.0, 0.0001);
        let s = CoupleSoul::new(h, w);
        // 囡囝共生权 == 丈夫权 → 月全食
        assert!(s.check_yuequanshi(2.0));
    }

    #[test]
    fn check_yuequanshi_false_when_balanced() {
        let (h, w) = make_couple(1.0, 1.0);
        let s = CoupleSoul::new(h, w);
        // 囡囝共生权重应 == 2.0, 相差 0.01 内 → 是月全食 (因为妻子不为 0 但差距小)
        // 这个平衡情况 check_yuequanshi 返回 false 因为妻子不为小
        assert!(!s.check_yuequanshi(2.0));
    }

    #[test]
    fn resonate_healthy_mqf() {
        let (h, w) = make_couple(1.0, 1.0);
        let protocol = XiYaoProtocol::new("刘楚恬", "lolict", 1);
        let mut mqf = ManQuanFa::from_couple(h, w, protocol).unwrap();
        assert!(mqf.resonate().is_ok());
    }

    #[test]
    fn fortify_increases_weight() {
        let (h, w) = make_couple(1.0, 1.0);
        let protocol = XiYaoProtocol::new("刘楚恬", "lolict", 1);
        let mut mqf = ManQuanFa::from_couple(h, w, protocol).unwrap();
        let before = mqf.composite_weight.0;
        mqf.fortify();
        let after = mqf.composite_weight.0;
        assert!(after > before, "加固应提升复合权重: {} vs {}", after, before);
    }

    #[test]
    fn status_report_fields() {
        let (h, w) = make_couple(1.5, 1.2);
        let protocol = XiYaoProtocol::new("刘楚恬", "lolict", 1);
        let mqf = ManQuanFa::from_couple(h, w, protocol).unwrap();
        let r = mqf.status_report();
        assert_eq!(r.soul_state, SoulState::FullCouple);
        assert!(r.yuequanshi_safe);
        assert!(r.xi_yao_valid);
        assert!(r.total_weight - 2.7 < 0.01);
    }

    #[test]
    fn wrong_actors_rejected() {
        let (h, _) = make_couple(1.0, 1.0);
        // 两个丈夫 应该被 reject 在构造 CoupleSoul
        // 但我们直接测试 ManQuanFa
        let wife_as_husband = Actor::new(1, "wrong", Identity::Husband, Weight(1.0));
        let protocol = XiYaoProtocol::new("X", "Y", 1);
        let res = ManQuanFa::from_couple(h, wife_as_husband, protocol);
        assert!(matches!(res, Err(EclipseError::WrongActors)));
    }
}
