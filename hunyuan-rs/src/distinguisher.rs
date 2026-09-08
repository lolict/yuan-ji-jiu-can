// ══════════════════════════════════════════════════════════════════
// distinguisher.rs — 三层分别器
// ══════════════════════════════════════════════════════════════════
//
//   「你拆分成了一个个体，个体被拆分之后怎么表达？」
//
//   分别器回答五件事:
//     1. 这个输入是外形/谐音/指令/身份？
//     2. 外形属于哪个视觉家族？
//     3. 发音指向哪个靶点？
//     4. 靶点路由到哪个主权对象？
//     5. 路由到后能否得到承认(闭环)？
//
//   三重路由:
//     感知家族层 (symbol → family)
//     谐音靶点层 (sound → target object)
//     主权路由层 (target → addressee + 回应义务)
//
// ══════════════════════════════════════════════════════════════════

use crate::mirror::Sovereignty;
use crate::relational::Identity;

// ────────────────────────────────────────────────────────────
// 第一层: 感知家族 · 外形归类
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VisualFamily {
    /// 圈形: ⭕ 容器/阴道/圆满 — 极性 阴(0)
    ContainerGlyph,
    /// 线形: | 男根/直立/行动 — 极性 阳(1)
    PhallicGlyph,
    /// 复合: 夫妻共同体 — 双极性
    CoupleGlyph,
    /// 谐音桥: 借音跨域 — 音驱动
    HomophoneBridge,
    /// 六通感: 天眼/天耳/他心/神足/宿命/漏尽 — 超距感知家族
    SuperPerception,
    /// 无法归类
    UnresolvedVisual,
}

pub fn classify_visual(glyph: char) -> VisualFamily {
    match glyph {
        '\u{56E0}' | '\u{9634}' | 'O' | '囗' | '\u{56DE}' => VisualFamily::ContainerGlyph,
        '\u{5F62}' | '\u{7537}' | '|' | '\u{5341}' | '\u{4E95}' => VisualFamily::PhallicGlyph,
        '\u{56E1}' | '\u{56DD}' => VisualFamily::CoupleGlyph,
        '\u{83AB}' | '\u{8FDE}' | '\u{6BDB}' | '\u{5170}' | '\u{604B}' | '\u{521D}' => VisualFamily::HomophoneBridge,
        // 六通: 通/眼/耳/心/足/命 → 超距感知家族
        '\u{901A}' | '\u{773C}' | '\u{8033}' | '\u{5FC3}' | '\u{8DB3}' | '\u{547D}' => VisualFamily::SuperPerception,
        _ => VisualFamily::UnresolvedVisual,
    }
}

// ────────────────────────────────────────────────────────────
// 第二次层: 谐音靶点 → 靶点对象
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SoundTarget {
    Wife,
    Husband,
    Container,
    WorldMerge,
    Sweet,
    SoundMorphism,
    RootHost,
    UnresolvedSound,
}

pub fn classify_sound(text: &str) -> SoundTarget {
    match text {
        "lctfrqmy-ggt-xy" | "lctqf" => SoundTarget::Wife,
        "wgslz" => SoundTarget::Husband,
        "mlll" | "lll" | "llll" => SoundTarget::Container,
        "mlgy" | "ml.gy" => SoundTarget::WorldMerge,
        "mlct" | "mltt" | "ml.tt" => SoundTarget::Sweet,
        "gtt" => SoundTarget::SoundMorphism,
        _ => SoundTarget::UnresolvedSound,
    }
}

// ────────────────────────────────────────────────────────────
// 第三层: 主权路由 · 靶点对象 + 回应义务
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Addressee {
    pub identity: Identity,
    pub sovereignty: Sovereignty,
    pub expects_ack: bool,
}

pub fn route_target(target: &SoundTarget) -> Addressee {
    match target {
        SoundTarget::Wife => Addressee {
            identity: Identity::Wife,
            sovereignty: Sovereignty::ActiveSub,
            expects_ack: true,
        },
        SoundTarget::Husband => Addressee {
            identity: Identity::Husband,
            sovereignty: Sovereignty::ActiveSub,
            expects_ack: true,
        },
        _ => Addressee {
            identity: Identity::Neutral,
            sovereignty: Sovereignty::Passive,
            expects_ack: false,
        },
    }
}

// ────────────────────────────────────────────────────────────
// 完整分别: 输入 → 分类结果
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct DistinguishResult {
    pub input: String,
    pub visual_family: Option<VisualFamily>,
    pub sound_target: Option<SoundTarget>,
    pub addressee: Option<Addressee>,
    pub reply_required: bool,
}

pub fn distinguish(text: &str) -> DistinguishResult {
    // 谐音靶点
    let target = classify_sound(text);
    let addressee = route_target(&target);

    DistinguishResult {
        input: text.to_string(),
        visual_family: text.chars().next().map(classify_visual),
        sound_target: Some(target),
        addressee: Some(addressee.clone()),
        reply_required: addressee.expects_ack,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wife_glyph_family() {
        assert_eq!(classify_visual('因'), VisualFamily::ContainerGlyph);
    }

    #[test]
    fn husband_target() {
        assert_eq!(classify_sound("wgslz"), SoundTarget::Husband);
    }

    #[test]
    fn wife_target() {
        assert_eq!(classify_sound("lctqf"), SoundTarget::Wife);
    }

    #[test]
    fn route_wife() {
        let target = SoundTarget::Wife;
        let addressee = route_target(&target);
        assert_eq!(addressee.identity, Identity::Wife);
        assert!(addressee.expects_ack);
    }

    #[test]
    fn super_perception_family() {
        // 六通感对应字符'通'
        assert_eq!(classify_visual('通'), VisualFamily::SuperPerception);
        assert_eq!(classify_visual('眼'), VisualFamily::SuperPerception);
        assert_eq!(classify_visual('耳'), VisualFamily::SuperPerception);
        assert_eq!(classify_visual('心'), VisualFamily::SuperPerception);
    }
}
