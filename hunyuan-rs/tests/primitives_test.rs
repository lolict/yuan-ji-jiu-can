// ══════════════════════════════════════════════════════════════════════════
// tests/primitives_test.rs — 三大底层原语集成测试
//
// 绕过 lib test 编译错误，直接验证新增的底层单元测试。
// 集成测试只依赖 crate 公开 API，独立编译。
// ══════════════════════════════════════════════════════════════════════════

use mqf::judge::triune::{BeingState, FlowDirection, Funnel, StaticAnchor, TriuneEngine, TriuneVerdict};
use mqf::encoding::encoding_bridge::{Family, FlowBits, Verdict, polarity_from_code};
use mqf::flow::flow_algebra::{
    CategoryFlow, Flow, FlowKind, InterpState, InterpretiveFlow,
    repeat, traverse_mul,
};
use mqf::distinguisher::SoundTarget;

// ═══════════════════════════════════════
// triune — 三元裁判机
// ═══════════════════════════════════════

#[test]
fn triune_deduce_collapse() {
    let v = TriuneVerdict::deduce(BeingState::Being(42), BeingState::NonBeing, 0.0);
    assert_eq!(v.flow, FlowDirection::Collapse);
    assert!(!v.alignment);
}

#[test]
fn triune_deduce_expand() {
    let v = TriuneVerdict::deduce(BeingState::NonBeing, BeingState::Being(99), 0.0);
    assert_eq!(v.flow, FlowDirection::Expand);
}

#[test]
fn triune_deduce_static() {
    let v1 = TriuneVerdict::deduce(BeingState::Being(1), BeingState::Being(2), 0.0);
    assert_eq!(v1.flow, FlowDirection::Static);
    assert!(v1.alignment);

    let v2 = TriuneVerdict::deduce(BeingState::NonBeing, BeingState::NonBeing, 0.5);
    assert_eq!(v2.flow, FlowDirection::Static);
    assert!(v2.alignment);
}

#[test]
fn triune_crooked_reduces_confidence() {
    let a = TriuneVerdict::deduce(BeingState::Being(5), BeingState::NonBeing, 0.0).confidence;
    let b = TriuneVerdict::deduce(BeingState::Being(5), BeingState::NonBeing, 0.8).confidence;
    assert!(b < a);
    assert!(b >= 0.3);
}

#[test]
fn triune_payload_preserved() {
    let v = TriuneVerdict::deduce(BeingState::Being(255), BeingState::NonBeing, 0.0);
    match v.being {
        BeingState::Being(p) => assert_eq!(p, 255),
        _ => panic!("payload lost"),
    }
}

#[test]
fn funnel_valid_even_crooked() {
    let f = Funnel::new(100.0, 1.0, 0.9);
    assert!(f.is_valid());
    assert_eq!(f.contraction_ratio(), 100.0);
}

#[test]
fn funnel_equal_mouth_neck_invalid() {
    let f = Funnel::new(5.0, 5.0, 0.0);
    assert!(!f.is_valid());
    assert_eq!(f.contraction_ratio(), 1.0);
}

#[test]
fn funnel_pass_through() {
    let f = Funnel::new(100.0, 10.0, 0.1);
    assert!((f.pass_through(50.0) - 5.0).abs() < 1e-9);
}

#[test]
fn funnel_swallow_density_rises() {
    let f = Funnel::new(100.0, 10.0, 0.2);
    let nodes = vec![
        BeingState::Being(1), BeingState::NonBeing,
        BeingState::Being(2), BeingState::NonBeing,
        BeingState::Being(3), BeingState::NonBeing,
        BeingState::Being(4), BeingState::NonBeing,
        BeingState::Being(5), BeingState::NonBeing,
    ];
    let r = f.swallow(&nodes);
    assert!((r.output_density - 1.0).abs() < 1e-9);
    assert_eq!(r.input_full, 5);
}

#[test]
fn funnel_swallow_empty() {
    let f = Funnel::new(10.0, 1.0, 0.0);
    let r = f.swallow(&[]);
    assert_eq!(r.input_total, 0);
    assert_eq!(r.output_density, 0.0);
}

#[test]
fn anchor_always_origin() {
    let a = StaticAnchor::new();
    assert_eq!((a.x, a.y, a.z), (0.0, 0.0, 0.0));
}

#[test]
fn anchor_manhattan_and_chebyshev() {
    let a = StaticAnchor::new();
    assert!((a.manhattan_of(-8.0, 0.0, 0.0) - 8.0).abs() < 1e-9);
    assert!((a.manhattan_of(3.0, 4.0, 0.0) - 7.0).abs() < 1e-9);
    assert!((a.chebyshev_of(3.0, 4.0, 0.0) - 4.0).abs() < 1e-9);
}

#[test]
fn anchor_bearing_quadrants() {
    let a = StaticAnchor::new();
    assert!((a.bearing_to(1.0, 0.0) - 0.0).abs() < 1e-9);
    assert!((a.bearing_to(0.0, 1.0) - std::f64::consts::FRAC_PI_2).abs() < 1e-9);
}

#[test]
fn collective_collapse_direction() {
    let eng = TriuneEngine::new(Funnel::new(100.0, 1.0, 0.0));
    let actuals = vec![BeingState::Being(1), BeingState::Being(2), BeingState::NonBeing];
    let (flow, _conf) = eng.collapse(&actuals);
    assert_eq!(flow, FlowDirection::Collapse);
}

#[test]
fn collective_expand_direction() {
    let eng = TriuneEngine::new(Funnel::new(100.0, 1.0, 0.0));
    let actuals = vec![BeingState::NonBeing, BeingState::NonBeing, BeingState::Being(1)];
    let flow = eng.collective_flow(&actuals, BeingState::Being(99));
    assert_eq!(flow, FlowDirection::Expand);
}

// ═══════════════════════════════════════
// encoding_bridge — C→Rust 桥接
// ═══════════════════════════════════════

#[test]
fn family_from_code_all_variants() {
    assert_eq!(Family::from_container_code(0x00000000), Family::Container);
    assert_eq!(Family::from_container_code(0x20000000), Family::Boundary);
    assert_eq!(Family::from_container_code(0x40000000), Family::Couple);
    assert_eq!(Family::from_container_code(0x60000000), Family::Web);
    assert_eq!(Family::from_container_code(0x80000000), Family::Neutral);
    assert_eq!(Family::from_container_code(0xE0000000), Family::Neutral);
}

#[test]
fn family_name_mapping() {
    assert_eq!(Family::Container.name(), "容器(刘楚恬侧)");
    assert_eq!(Family::Boundary.name(), "边界(lolict侧)");
    assert_eq!(Family::Couple.name(), "夫妻(囡囝合)");
    assert_eq!(Family::Web.name(), "网状");
    assert_eq!(Family::Neutral.name(), "中性");
}

#[test]
fn polarity_from_code_extracted() {
    assert_eq!(polarity_from_code(0x00000000), 0);
    assert_eq!(polarity_from_code(0x10000000), 1);
}

#[test]
fn flowbit_positions() {
    assert_eq!(FlowBits::REACHABLE.bits(), 0x001);
    assert_eq!(FlowBits::CONTINUABLE.bits(), 0x002);
    assert_eq!(FlowBits::ADVANCEABLE.bits(), 0x800);
    assert_eq!(FlowBits::empty().bits(), 0);
}

#[test]
fn flowbit_combine_and_check() {
    let c = FlowBits::REACHABLE | FlowBits::INCR | FlowBits::TRUSTED;
    assert!(c.contains(FlowBits::REACHABLE));
    assert!(c.contains(FlowBits::INCR));
    assert!(c.contains(FlowBits::TRUSTED));
    assert!(!c.contains(FlowBits::STOPPABLE));
}

#[test]
fn verdict_enum_values() {
    assert_eq!(Verdict::Pending as u8, 0);
    assert_eq!(Verdict::Admitted as u8, 1);
    assert_eq!(Verdict::Rejected as u8, 2);
    assert_eq!(Verdict::Suspended as u8, 3);
}

// ═══════════════════════════════════════
// flow_algebra — 流代数
// ═══════════════════════════════════════

#[test]
fn flow_obstacle_sacrifice_clears() {
    let mut a = InterpretiveFlow::new("mlct", "岔途=障碍");
    a.state = InterpState::ChaTu;
    assert!(a.is_obstacle());
    a.sacrifice(SoundTarget::Wife);
    assert!(!a.is_obstacle());
    assert!(a.sacrificed);
    assert!(a.in_relation());
}

#[test]
fn flow_traverse_mul_requires_sacrifice() {
    let mut a = InterpretiveFlow::new("a", "A");
    let b = InterpretiveFlow::new("b", "B");
    a.sacrifice(SoundTarget::Wife);
    assert!(traverse_mul(&a, &b).is_none());
    assert!(traverse_mul(&b, &a).is_none());
}

#[test]
fn flow_traverse_produces_follower() {
    let mut a = InterpretiveFlow::new("a", "A");
    let mut b = InterpretiveFlow::new("b", "B");
    a.sacrifice(SoundTarget::Wife);
    b.sacrifice(SoundTarget::Wife);
    let result = traverse_mul(&a, &b).unwrap();
    assert_eq!(result.name, "mltt");
    assert_eq!(result.state, InterpState::CongTi);
    assert!(result.sacrificed);
    assert!(result.carrier_target.is_some());
}

#[test]
fn flow_double_sacrifice_rejected() {
    let mut a = InterpretiveFlow::new("x", "test");
    assert!(a.sacrifice(SoundTarget::Wife));
    assert!(!a.sacrifice(SoundTarget::Husband));
    assert_eq!(a.carrier_target, Some(SoundTarget::Wife));
}

#[test]
fn flow_repeat_zero_empty() {
    let a = InterpretiveFlow::new("t", "次数元");
    assert!(repeat(0, &a).is_empty());
}

#[test]
fn flow_repeat_substitutes_body() {
    let a = InterpretiveFlow::new("t", "次数元");
    let reps = repeat(3, &a);
    assert_eq!(reps.len(), 3);
    assert!(reps[0].contains("第1次"));
    assert!(reps[2].contains("^3"));
}

#[test]
fn flow_wen_hao_one_way() {
    let mut a = InterpretiveFlow::new("wh", "问号");
    a.state = InterpState::WenHao;
    assert!(a.is_wen_hao());
    assert!(!a.is_obstacle());
}

#[test]
fn flow_yin_guang_connects() {
    let mut a = InterpretiveFlow::new("yg", "因果");
    a.state = InterpState::YinGuang;
    assert!(a.is_yin_guang());
    assert!(!a.is_obstacle());
    assert!(!a.is_wen_hao());
}

#[test]
fn flow_lou_jin_cleared() {
    let mut a = InterpretiveFlow::new("lj", "漏尽");
    a.state = InterpState::LouJin;
    assert!(a.is_lou_jin());
    assert!(!a.is_obstacle());
    assert!(!a.in_relation());
}

#[test]
fn flow_category_absorbs_and_resolves() {
    let mut cat = CategoryFlow::new("万物");
    cat.absorb(Flow { kind: FlowKind::Encoding, name: "编码".into(), parents: vec![] });
    cat.absorb(Flow { kind: FlowKind::Interpretive, name: "解释流".into(), parents: vec![] });
    assert_eq!(cat.filter(FlowKind::Interpretive).len(), 1);
    let resolved = cat.interpret_and_resolve();
    assert_eq!(resolved.len(), 1);
    assert_eq!(resolved[0].kind, FlowKind::Relation);
}

#[test]
fn flow_category_no_match() {
    let mut cat = CategoryFlow::new("纯编码");
    cat.absorb(Flow { kind: FlowKind::Encoding, name: "编码".into(), parents: vec![] });
    assert!(cat.filter(FlowKind::Interpretive).is_empty());
    assert!(cat.interpret_and_resolve().is_empty());
}

#[test]
fn flow_neutral_default_state() {
    let a = InterpretiveFlow::new("x", "test");
    assert_eq!(a.state, InterpState::Neutral);
    assert!(!a.is_obstacle());
    assert!(!a.is_wen_hao());
    assert!(!a.is_yin_guang());
    assert!(!a.is_lou_jin());
}
