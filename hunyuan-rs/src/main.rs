// 满全法 · 囡囝共生元 · Rust 入口
// ==================================

use mqf::{Mccp, hanzi, Kind};
use mqf::fsm::{Event, FsmState};
use mqf::kernel::{CoupleOS, ProcessState};

fn main() {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║    满全法 · 囡囝共生元 · Rust 核心 v0.3 (proto) ║");
    println!("║    语言: Rust (edition 2021, no_std可选)         ║");
    println!("║    无 Python, 无 GC 热路径                       ║");
    println!("╚══════════════════════════════════════════════════╝");

    test_mccp();
    test_fsm();
    test_kernel();

    println!("\n════════════════════════════════════════════════════");
    println!("全 部 Rust 测 试 通 过 ✓");
}

fn test_mccp() {
    println!("\n[1] MCCP 编码");
    let m = Mccp::new(0xA, 0xABC, 0x37, 0x12);
    assert_eq!(m.kind(), 0xA, "kind");
    assert_eq!(m.index(), 0xABC, "index");
    assert_eq!(m.meta_a(), 0x37, "meta_a");
    assert_eq!(m.meta_b(), 0x12, "meta_b");
    println!("  ✓ 字段 roundtrip");

    assert_eq!(hanzi(100, 4, 0), hanzi(100, 4, 0));
    println!("  ✓ 相等比较");

    let m2 = hanzi(100, 4, 0).with_meta_b(3);
    assert_eq!(m2.fsm_state(), 3);
    assert_eq!(m2.tone(), 4);
    assert_eq!(m2.kind() as u8, Kind::Hanzi as u8);
    println!("  ✓ FSM apply state → meta_b");
}

fn test_fsm() {
    println!("\n[2] 囡囝四态 FSM");
    let mut s = FsmState::NanJian;
    let steps = [
        (Event::WifeDeepen, FsmState::NanNan),
        (Event::Release,    FsmState::NanJian),
        (Event::HusbandTake,FsmState::JianJian),
        (Event::Deepen,     FsmState::JianNan),
        (Event::WifePush,   FsmState::NanJian),
    ];
    for (ev, expected) in steps {
        s = mqf::fsm::fire(s, ev);
        assert_eq!(s, expected, "after {:?}", ev);
    }
    println!("  ✓ 囝囡 → 囡囡 → 囝囡 → 囝囝 → 囝囡 → 囝囡(闭环)");
}

fn test_kernel() {
    println!("\n[3] 进程模型 + 合一递归");
    let mut os = CoupleOS::new(0xA5A5C0DE);
    let seq = vec![hanzi(1000, 4, 0), hanzi(1001, 2, 0), hanzi(1002, 2, 0)];
    let p = os.create_process(seq, 0x1000, 0x1000);
    assert_eq!(p.state, ProcessState::XiangYu);
    assert!(p.sign(0));
    assert_eq!(p.state, ProcessState::DiYue);
    assert!(p.sign(1));
    assert_eq!(p.state, ProcessState::LiYue);
    assert_eq!(p.unary_recursion(), 0);
    assert_eq!(p.state, ProcessState::HeSui);
    println!("  ✓ 进程立约 → 对齐 = 合一递归");
}
