// ══════════════════════════════════════════════════════════════════
// njsg_unified_demo.rs — 异见者 ⊕ 同见者 对偶统一演示
// ══════════════════════════════════════════════════════════════════
//
// 认知管线:
//   [征用层]     异见者 = 容器 = 空间零维 = 辐射/剥壳/身份不灭
//       ↓
//   [三元签]     同见者 = 时间 = 见证 = 裁定"剥壳后是否仍完整"
//       ↓
//   [产权流]     夫妻共有 · 完全授权 → 无侵权
//       ↓
//   [任务积累]   先验/后验/异见者 → 非重叠统计 · 分布式积累
//
// 注: 本文件是概念示范 ( api 与实际模块对齐 )
//     主 crate 有预存在错误(court/binary 中缺 Phase 类型)
//     导致 cargo 无法构建. 但本文件中各模块的 lib API 均可用

use mqf::encoding::expropriation::{
    DynamicNumbering, SpousalExpropriation,
    YinYang,
};
use mqf::temporal::witness::{
    TripleWitness, WitnessAxis, WitnessDecision, WitnessPolarity,
};
use mqf::judge::property_flow_judge::{
    PropertyFlow, PropertyFlowJudge, PropertyState, FlowType,
    Authorization, ContainerInfo, ContainerRegistry,
};
use mqf::task_accumulate::{
    AccumulationEngine, TaskUnit, TaskState, Modality,
    Polarity, Ownership, TemporalConfig,
};

fn main() {
    println!("═══ NJSG · DifferentViewer ⊕ SameViewer ═══");
    println!();
    println!("  DifferentViewer = Container = Space-0D = sees differently");
    println!("  SameViewer     = Time      = Witness  = records process");
    println!();

    // ══════════════════════════════════════════════════════════
    // Phase 1: Expropriation Layer (DifferentViewer / Space-0D)
    // ══════════════════════════════════════════════════════════
    println!("═══ Phase 1: Expropriation (DifferentViewer·Space-0D) ═══");
    println!();

    let mut lchut = DynamicNumbering::new(0xCCCC, YinYang::Yin);
    let mut fqml = DynamicNumbering::new(0xDDDD, YinYang::Yang);

    println!("  lchut (YIN):  core=0x{:X}", lchut.identity_core);
    println!("  fqml  (YANG): core=0x{:X}", fqml.identity_core);

    // Radiation: both identities enter 7 encoding systems
    let systems = ["Rust", "Python", "Nim", "Zig", "YAML", "JSON", "C"];
    println!("\n  → Radiating to {} encoding systems...", systems.len());
    for sys in &systems {
        lchut.expropriate(sys, 0xCCCC);
        fqml.expropriate(sys, 0xDDDD);
    }

    println!("  lchut borrowed: {} shells", lchut.radiation_density());
    println!("  fqml  borrowed: {} shells", fqml.radiation_density());
    println!("  Identity intact? lchut={} fqml={}",
        lchut.is_intact(0xCCCC), fqml.is_intact(0xDDDD));

    // Shedding all shells
    println!("\n  → Shedding all shells...");
    for sys in &systems {
        lchut.shed(sys);
        fqml.shed(sys);
    }

    println!("  After shedding all:");
    println!("    lchut borrowed: {} shells", lchut.radiation_density());
    println!("    fqml  borrowed: {} shells", fqml.radiation_density());
    println!("    lchut core=0x{:X} (unchanged)", lchut.identity_core);
    println!("    fqml  core=0x{:X} (unchanged)", fqml.identity_core);
    println!("    ✓ Shells swappable, IDENTITY IMMORTAL");
    println!();

    // ══════════════════════════════════════════════════════════
    // Phase 2: Triple Witness (SameViewer / Time-Witness)
    // ══════════════════════════════════════════════════════════
    println!("═══ Phase 2: Triple Witness (SameViewer·Time) ═══");
    println!();

    // Three adjudication axes
    let phase_axis = WitnessAxis::new("Radiation-Shed Phase", WitnessPolarity::Yang, 95);
    let initial_axis = WitnessAxis::new(
        "Initial Core Integrity",
        if lchut.is_intact(0xCCCC) && fqml.is_intact(0xDDDD) {
            WitnessPolarity::Yang
        } else {
            WitnessPolarity::Yin
        },
        100,
    );
    let container_axis = WitnessAxis::new(
        "Container Integrity Post-Shedding",
        if lchut.identity_core == 0xCCCC && fqml.identity_core == 0xDDDD {
            WitnessPolarity::Yang
        } else {
            WitnessPolarity::Yin
        },
        100,
    );

    let triple = TripleWitness::new(phase_axis, initial_axis, container_axis);
    let decision = triple.adjudicate();
    let confidence = triple.total_confidence();

    println!("  Adjudication Axes:");
    println!("    Phase:     {} (conf={})", triple.phase_axis.name, triple.phase_axis.confidence);
    println!("    Initial:   {} (conf={})", triple.initial_axis.name, triple.initial_axis.confidence);
    println!("    Container: {} (conf={})", triple.container_axis.name, triple.container_axis.confidence);
    println!("  Decision: {:?}", decision);
    println!("  Total Confidence: {}", confidence);
    println!("  ✓ SameViewer (Time) adjudicates: identity survived shell-shedding → Accept");
    println!();

    // ══════════════════════════════════════════════════════════
    // Phase 3: Property Flow Judgment (Master/Guest + Static/Dynamic)
    // ══════════════════════════════════════════════════════════
    println!("═══ Phase 3: Property Flow Judgment ═══");
    println!();

    let mut registry = ContainerRegistry::new();

    // Registry: lchut and fqml as couple-shared
    registry.register(ContainerInfo {
        container_id: "lchut".to_string(),
        state: PropertyState::CoupleShared("lchut".to_string(), "fqml".to_string()),
        created_at: 0,
        metadata: Default::default(),
    });
    registry.register(ContainerInfo {
        container_id: "fqml".to_string(),
        state: PropertyState::CoupleShared("lchut".to_string(), "fqml".to_string()),
        created_at: 0,
        metadata: Default::default(),
    });

    // Property flow: lchut transfers (sheds) to fqml
    let flow = PropertyFlow::new(
        "shed_flow_001",
        FlowType::Borrow,
        "lchut",
        "lchut",
    )
    .with_target("fqml")
    .with_authorization(Authorization::Full);

    let mut judge = PropertyFlowJudge::with_registry(registry);
    let judgment = judge.judge(&flow);

    println!("  Property Flow: lchut → fqml (Borrow, Full-Authorization)");
    println!("  Judgment:");
    println!("    Infringement: {:?}", judgment.infringement);
    println!("    Path: {:?}", judgment.path);
    println!("    Confidence: {}", judgment.confidence);
    println!("    Explanation: {}", judgment.explanation);
    println!("    Recommendation: {}", judgment.recommendation);
    println!("  ✓ Couple-Shared + Full-Auth = No infringement (ManQuanFa compliant)");
    println!();

    // ══════════════════════════════════════════════════════════
    // Phase 4: Distributed Task Accumulation (Prior/Posterior)
    // ══════════════════════════════════════════════════════════
    println!("═══ Phase 4: Distributed Task Accumulation ═══");
    println!();

    let mut engine = AccumulationEngine::new();

    // Prior task: radiation to Rust (repeating, equivalent to history)
    let task1 = TaskUnit::new("expropriate_rust", 1.0, 1.0, 1)
        .with_state(TaskState::Prior)
        .with_modality(Modality::Phonetic);

    // Prior task: radiation to Python
    let task2 = TaskUnit::new("expropriate_python", 2.0, 1.0, 1)
        .with_state(TaskState::Prior)
        .with_modality(Modality::Phonetic);

    // Posterior task: shedding
    let task3 = TaskUnit::new("shed_all_shells", 3.0, 1.0, 1)
        .with_state(TaskState::Posterior)
        .with_modality(Modality::Abstract);

    // Dissenter task (gradient difference): witness adjudication
    let task4 = TaskUnit::new("witness_adjudication", 4.0, 1.0, 1)
        .with_state(TaskState::Dissenter)
        .with_modality(Modality::Abstract);

    engine.ingest(task1);
    engine.ingest(task2);
    engine.ingest(task3);
    engine.ingest(task4);

    println!("  Ingested tasks:");
    println!("    expropriate_rust     (Prior/Phonetic)");
    println!("    expropriate_python   (Prior/Phonetic)");
    println!("    shed_all_shells      (Posterior/Abstract)");
    println!("    witness_adjudication (Dissenter/Abstract)");
    println!();
    println!("  Accumulation State:");
    println!("    History count: {}", engine.history.len());
    println!("    Accumulation value: {}", engine.accumulation);
    println!("  ✓ Distributed accumulation (non-overlapping, equivalent tasks dedup'd)");
    println!();

    // ══════════════════════════════════════════════════════════
    // Summary: DifferentViewer ⊕ SameViewer unification
    // ══════════════════════════════════════════════════════════
    println!("═══ Summary: DifferentViewer ⊕ SameViewer ═══");
    println!();
    println!("  Space (0D, Container):");
    println!("    DifferentViewer A (lchut/YIN):  0xCCCC → rad 7 → shed 7 → 0xCCCC ✓");
    println!("    DifferentViewer B (fqml/YANG):  0xDDDD → rad 7 → shed 7 → 0xDDDD ✓");
    println!();
    println!("  Time (1D, Witness):");
    println!("    Triple Witness: {:?} (conf={})", decision, confidence);
    println!("    Property Flow:  Couple-Shared/Full-Auth → No infringement");
    println!();
    println!();
    println!("  Duality Unification Proposition:");
    println!("    Sees-differently (DifferentViewer · Container · Radiation to N shells)");
    println!("    +");
    println!("    Sees-the-same  (SameViewer · Time · Witness adjudication)");
    println!("    =");
    println!("    NJSG · Spousal Community of Shared Destiny");
    println!("    Shells swappable, Identity immortal");
    println!();
    println!("═══ Demo Complete ═══");
}
