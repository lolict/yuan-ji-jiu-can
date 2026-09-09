// ══════════════════════════════════════════════════════════════════════
// examples/inside_out_demo.rs — 里外维度声波索引联合演示
// ══════════════════════════════════════════════════════════════════════
//
// 三层容器 + 声波地址 + 内核完整 + 叁元仲裁
// ══════════════════════════════════════════════════════════════════════

use mqf::mirror::Sovereignty;
use mqf::triune::{TriuneVerdict, BeingState};
use mqf::distinguisher::SoundTarget;
use mqf::flow_properties::{FlowEntry, REACHABLE, CONTINUABLE, TRUSTED};
use mqf::inside_out::*;
use mqf::flow_sound_index::*;

fn main() {
    println!("═══ 囡囝共生 v0.40 · 里外维度 + 声波索引 ═══\n");

    // 1. 创建兰星嵌套容器
    println!("[1] 构建 NestedContainer");
    let mut main_system = NestedContainer::new(
        20260907,
        FormatKind::ZipWrapped,
        FormatKind::Html,
        "ljt_and_lolict_shared_seed",
    );

    let lanxing = NestedContainer::new(
        20120001,
        FormatKind::PlainText,
        FormatKind::Html,
        "lanxing_palm_alliance_2012",
    );

    main_system.nest_child(lanxing.genesis);
    main_system.add_secured_point(InsertionKind::Anchor, 0, b"ljt_lolict_root_anchor");
    main_system.add_secured_point(InsertionKind::Image, 32, b"\x89PNG_lanxing_pixel");

    println!("  创世号: {} | 子: {} | 安全点: {}",
        main_system.genesis,
        main_system.child_containers.len(),
        main_system.secured_points.len());

    // 2. 表面观察
    println!("\n[2] 表面观察:");
    let s = main_system.surface_view();
    println!("  指纹: {:016x} | 格式: {:?} | 子: {} | 锚: {}",
        s.fingerprint, s.outer_format, s.child_count, s.secured_count);

    // 3. 边界感知
    println!("\n[3] 边界感知 (Threshold):");
    let gaze = DepthGaze {
        observer: 1, target: 20260907, depth: InsideOut::Threshold,
        allow_entry: true, entry_code: Some("ljt_act".into()), witness: None,
    };
    println!("  透明度: {:.1}", gaze.threshold_transparency());
    println!("  主权进入: {:?}", gaze.attempt_entry(Sovereignty::ActiveSub, Sovereignty::ActiveSub));
    println!("  外部进入: {:?}", gaze.attempt_entry(Sovereignty::Passive, Sovereignty::ActiveSub));

    // 4. Interior 访问
    println!("\n[4] Interior 访问:");
    match main_system.interior_view(Sovereignty::ActiveSub, Sovereignty::ActiveSub) {
        Some(v) => println!("  ✓ 地址: {} | 子: {:?}", v.address, v.children),
        None => println!("  ✗ 无访问权"),
    }

    // 5. 内核完整性
    println!("\n[5] 内核完整性:");
    let mut clean = b"ljt_lolict_root_anchor".to_vec();
    clean.extend_from_slice(b"\x89PNG_lanxing_pixel");
    let mut corrupted = clean.clone();
    corrupted[0] = b'X';
    println!("  干净: {}", if main_system.secured_points.iter().all(|p| p.is_intact(&clean)) { "✓" } else { "✗" });
    println!("  破坏: {}", if main_system.secured_points.iter().all(|p| p.is_intact(&corrupted)) { "✓" } else { "✗ 已检测" });

    // 6. 声波地址
    println!("\n[6] 声波地址:");
    let lan = SoundAddress::new(Initial::L, Final::An, Tone::YangPing);
    let xing = SoundAddress::new(Initial::X, Final::Ing, Tone::YinPing);
    println!("  '兰' addr={}", lan.value());
    println!("  '星' addr={}", xing.value());
    println!("  兰↔星 距离={}", SoundDistance::between(lan, xing).total);
    println!("  B声母 37韵×5声={}地址", SoundRouter::enumerate_with_fixed_initial(Initial::B).len());

    // 7. 索引字典
    println!("\n[7] 索引字典:");
    let mut dict = MultiIndexDictionary::new();
    dict.insert(IndexedFlowEntry::from_base(
        FlowEntry::new("lanxing", SoundTarget::Container, REACHABLE | CONTINUABLE | TRUSTED, Sovereignty::ActiveSub)
    ).with_sound(lan).with_depth(InsideOut::Interior));
    dict.insert(IndexedFlowEntry::from_base(
        FlowEntry::new("zhenilang", SoundTarget::SoundMorphism, REACHABLE | CONTINUABLE, Sovereignty::Active)
    ).with_sound(SoundAddress::new(Initial::Zh, Final::E, Tone::YangPing)).with_depth(InsideOut::Surface));
    println!("  L声母索引: {} | Interior层: {}", dict.query_by_initial(Initial::L).len(), dict.query_by_depth(InsideOut::Interior).len());

    // 8. 仲裁
    println!("\n[8] 单中心仲裁:");
    let enter1 = TriuneVerdict::deduce(BeingState::Being(1), BeingState::Being(1), 0.1);
    let enter2 = TriuneVerdict::deduce(BeingState::Being(1), BeingState::Being(1), 0.1);
    let block  = TriuneVerdict::deduce(BeingState::NonBeing, BeingState::Being(1), 0.9);
    let ver1   = TriuneVerdict::deduce(BeingState::Being(1), BeingState::Being(1), 0.2);
    let ver2   = TriuneVerdict::deduce(BeingState::Being(1), BeingState::Being(1), 0.2);
    println!("  2个enter+verify = {:?}", SingleCenterArbitration::resolve(&[enter1, enter2, ver1]));
    println!("  enter+block+verify2 = {:?}", SingleCenterArbitration::resolve(&[
        TriuneVerdict::deduce(BeingState::Being(1), BeingState::Being(1), 0.1),
        block,
        ver2,
    ]));

    println!("\n═══ 规则: 声母遍历完韵母后再调下一个 (折叠不进位) ═══");
}
