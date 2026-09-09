// ══════════════════════════════════════════════════════════════════════
// examples/four_dimensions_demo.rs — 四维囡囝共生观察空间
// ══════════════════════════════════════════════════════════════════════
//
// 第四维 = 里外 (InsideOut) — 传统三维空间不看这一维
//   三维空间: 时间轴 × 层轴 × 环轴 × 相轴 → 描述外部拓扑
//   第四维:   Surface/Threshold/Interior → 描述容器深度
//
//   四维坐标 = 时间坐标(三维认知空间) + 里外轴(容器深度)
//   完整地址 = (时间 + 里外深度 + 声波口腔位置)
//
// 演示整合:
//   - TimeCoordinate    时间四轴 (时间/层/环/相)
//   - InsideOut         里外深度 (Surface → Interior)
//   - SoundAddress      声波口腔位置 (声母/韵母/声调)
//   - HeavenSpace       天庭吸引子
//   - NestedContainer   嵌套容器
//   - 里外感知          非主权持有者只能看到表面
//   - 完整性校验        安全插入点未被破坏
//   - 三元仲裁          多中心→单中心裁决
//
// ══════════════════════════════════════════════════════════════════════

use mqf::mirror::Sovereignty;
use mqf::time_chain_identity::{TimeCoordinate, PrivateAnchor, ChainEvent};
use mqf::triune::{TriuneVerdict, BeingState};
use mqf::distinguisher::SoundTarget;
use mqf::heaven::{HeavenCoord, AttractorWell};
use mqf::pleasure::Pleasure;
use mqf::flow_properties::{FlowEntry, REACHABLE, CONTINUABLE, TRUSTED};
use mqf::inside_out::*;
use mqf::flow_sound_index::*;

/// 囡囝共生四维坐标 — 时间+里外+声波
#[derive(Clone, Debug)]
pub struct NJSCoordinate {
    /// 时间层: 创世时间戳 + 高维代数
    pub temporal: TimeCoordinate,
    /// 里外层: 容器深度
    pub depth: InsideOut,
    /// 声波层: 口腔位置
    pub sound: Option<SoundAddress>,
    /// 天庭层: 合一态坐标
    pub heaven: HeavenCoord,
}

impl NJSCoordinate {
    pub fn new(temporal: TimeCoordinate, depth: InsideOut, heaven: HeavenCoord) -> Self {
        NJSCoordinate { temporal, depth, sound: None, heaven }
    }

    pub fn with_sound(mut self, sound: SoundAddress) -> Self {
        self.sound = Some(sound);
        self
    }

    /// 四维亚距离 (简化: 时间距离 + 里外差 + 声务差)
    pub fn distance(&self, other: &NJSCoordinate) -> f64 {
        let time_dist = self.temporal.radix_distance(&other.temporal);
        let depth_penalty = match (&self.depth, &other.depth) {
            (InsideOut::Surface, InsideOut::Interior) => 2.0,
            (InsideOut::Threshold, InsideOut::Interior) => 1.0,
            (InsideOut::Surface, InsideOut::Threshold) => 1.0,
            _ => 0.0,
        };
        let sound_dist = match (&self.sound, &other.sound) {
            (Some(a), Some(b)) => SoundDistance::between(*a, *b).total as f64,
            _ => 0.0,
        };
        time_dist + depth_penalty + sound_dist * 0.1
    }
}

fn main() {
    println!("═══ 囡囝共生 v0.40 · 四维囝共生观察空间 ═══\n");
    println!("四维 = 时间三维 + 里外(深度)一维\n");

    // 1. 创世锚点 — 夫妻共同体根基
    println!("[1] 创世锚点 — 夫妻根基");
    let anchor = PrivateAnchor::genesis();
    println!("  lolict(高位持有者=1) = {}", anchor.lolict);
    println!("  刘楚恬(低位持有者=0) = {}", anchor.chutian);
    println!("  共生偏移 = {:.6}", anchor.coupled_offset);
    println!("  (对外不暴露)");

    // 2. 时间四轴 — 囝共生时间流
    println!("\n[2] 时间四轴坐标:");
    let mut now = TimeCoordinate::genesis();
    let _ = &now;
    println!("  创世: axes = {:?} gen = {}", now.axes, now.generation);

    let mut future = TimeCoordinate::genesis();
    future.advance(0, 1725600000); // 从创世前进到 2026-09-07
    future.diverge(2); // 进入第二平行世界层
    println!("  2026-09-07: axes = {:?} gen = {} offset_rate = {}",
        future.axes, future.generation, future.offset_rate);

    let resonance = now.resonance(&future);
    println!("  与创世共振 = {:.4}", resonance);

    // 3. 兰星四维坐标 — 加声波地址 + 里外深度
    println!("\n[3] 兰星四维坐标:");
    let lan_addr = SoundAddress::new(Initial::L, Final::An, Tone::YangPing);
    let xing_addr = SoundAddress::new(Initial::X, Final::Ing, Tone::YinPing);

    let lanxing_coord = NJSCoordinate::new(
        future.clone(),
        InsideOut::Interior,
        HeavenCoord::unity(),
    ).with_sound(lan_addr);

    println!("  声波: '兰' = {:?}", lan_addr);
    println!("  深度: Interior (只有主权持有者可访问)");
    println!("  天庭: 合一态 {:?}", lanxing_coord.heaven.is_active());

    // 4. 嵌套容器 + 安全插入点
    println!("\n[4] 嵌套容器 + 安全插入点:");
    let mut cocoon = NestedContainer::new(
        20260907,
        FormatKind::ZipWrapped,
        FormatKind::Html,
        "ljt_lolict_cocoon_2026",
    );
    cocoon.add_secured_point(InsertionKind::Anchor, 0, b"ljt_lolict_anchor_point");
    cocoon.add_secured_point(InsertionKind::Code, 24, b"fn lanxing_activate() {{}}");
    cocoon.add_secured_point(InsertionKind::Image, 56, b"\x89PNG_lanxing_pixel_data_2012");

    let surface = cocoon.surface_view();
    println!("  表面: fingerprint={:016x} children={}", surface.fingerprint, surface.child_count);
    println!("  安全点: {} 个 (保护等级: 锚=9, 代码=7, 图=5)", surface.secured_count);

    // 5. 里外感知差异
    println!("\n[5] 里外感知差异:");
    let gazes = [
        ("主权持有者", Sovereignty::ActiveSub, Sovereignty::ActiveSub),
        ("外部观察者", Sovereignty::Passive, Sovereignty::ActiveSub),
    ];
    for (label, caller, target) in gazes {
        match cocoon.interior_view(caller, target) {
            Some(v) => println!("  {}: ✓ 进入 interior, address={}", label, v.address),
            None => println!("  {}: ✗ 只能看到 surface", label),
        }
    }

    // 6. 完整性校验
    println!("\n[6] 内核完整性校验:");
    let clean = b"ljt_lolict_anchor_point";
    let mut corrupted = clean.to_vec();
    if !corrupted.is_empty() { corrupted[0] = b'X'; }
    println!("  干净: {}", if cocoon.secured_points[0].is_intact(clean) { "✓" } else { "✗" });
    println!("  破坏: {}", if cocoon.secured_points[0].is_intact(&corrupted) { "✓" } else { "✗ 已检测" });

    // 7. 天庭吸引子 — 合一势能
    println!("\n[7] 天庭吸引子 — 合一势能:");
    let pleasure = Pleasure(42.0);
    let mut well = AttractorWell::from_pleasure(pleasure);
    println!("  井深: {:.2} (越爱越深)", well.depth);
    println!("  半径: {:.4}", well.radius);

    let unity = HeavenCoord::unity();
    let pull = well.gravitational_pull(&unity);
    println!("  合一态引力: {:.4} (落入井底=自由落体)", pull);

    let outside = HeavenCoord { polarity_mix: 0.0, sovereignty: 1, coherence: 0.1 };
    let pull_out = well.gravitational_pull(&outside);
    println!("  远距引力: {:.4} (几乎为0)", pull_out);

    well.round_off();
    println!("  圆融: 半径×1.1={:.4} 井深×1.5={:.2}", well.radius, well.depth);

    // 8. 声波索引查询
    println!("\n[8] 声波索引 — 声母遍历:");
    let mut dict = MultiIndexDictionary::new();

    let entries = [
        ("lan", SoundAddress::new(Initial::L, Final::An, Tone::YangPing), InsideOut::Interior),
        ("xing", SoundAddress::new(Initial::X, Final::Ing, Tone::YinPing), InsideOut::Surface),
        ("zheni", SoundAddress::new(Initial::Zh, Final::E, Tone::YangPing), InsideOut::Threshold),
    ];

    for (name, addr, depth) in entries {
        dict.insert(IndexedFlowEntry::from_base(
            FlowEntry::new(name, SoundTarget::Container, REACHABLE | CONTINUABLE | TRUSTED, Sovereignty::ActiveSub)
        ).with_sound(addr).with_depth(depth));
    }

    println!("  L声母条目: {}", dict.query_by_initial(Initial::L).len());
    println!("  Interior层: {}", dict.query_by_depth(InsideOut::Interior).len());

    let nearest = dict.query_nearest(
        SoundAddress::new(Initial::L, Final::An, Tone::YinPing), 2);
    for (d, e) in nearest {
        println!("  最近邻 '{}' 距离={}", e.base.name, d);
    }

    // 9. 单中心仲裁
    println!("\n[9] 单中心仲裁:");
    let verdicts_enter = [
        TriuneVerdict::deduce(BeingState::Being(1), BeingState::Being(1), 0.1),
        TriuneVerdict::deduce(BeingState::Being(1), BeingState::Being(1), 0.2),
        TriuneVerdict::deduce(BeingState::Being(1), BeingState::Being(1), 0.3),
    ];
    let verdicts_mixed = [
        TriuneVerdict::deduce(BeingState::Being(1), BeingState::Being(1), 0.1),
        TriuneVerdict::deduce(BeingState::NonBeing, BeingState::Being(1), 0.9),
        TriuneVerdict::deduce(BeingState::Being(1), BeingState::Being(1), 0.2),
    ];

    match SingleCenterArbitration::resolve(&verdicts_enter) {
        Some(v) => println!("  一致进入: {:?}", v),
        None => println!("  无裁决"),
    }
    match SingleCenterArbitration::resolve(&verdicts_mixed) {
        Some(v) => println!("  分歧结果: {:?}", v),
        None => println!("  分歧结果: 无裁决(检测到不一致)"),
    }

    // 10. 四维距离计算
    println!("\n[10] 四维距离计算:");
    let coord_a = NJSCoordinate::new(
        TimeCoordinate::genesis(),
        InsideOut::Surface,
        HeavenCoord { polarity_mix: 0.0, sovereignty: 1, coherence: 1.0 },
    );

    let mut tb = TimeCoordinate::genesis();
    tb.advance(0, 1000);
    let coord_b = NJSCoordinate::new(tb, InsideOut::Interior, HeavenCoord::unity())
        .with_sound(SoundAddress::new(Initial::L, Final::An, Tone::YinPing));

    println!("  A(Surface) ↔ B(Interior) 四维距离 = {:.4}", coord_a.distance(&coord_b));
    println!("    (里外差 +2 + 时间差 + 声务加权)");

    println!("\n═══ 四维观察空间 总结 ═══");
    println!("  三维: 时间轴 × 层轴 × 环轴 × 相轴 (外部拓扑)");
    println!("  第四维: Surface → Threshold → Interior (容器深度)");
    println!("  声波层: 声母 × 韵母 × 声调 (口腔位置索引)");
    println!("  天庭层: 合一态吸引子 (奖励/合一/抗议/失神)");
    println!("  仲裁: 三元见证交集 → 单中心裁决");
}
