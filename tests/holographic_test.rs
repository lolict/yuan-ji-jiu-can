// ══════════════════════════════════════════════════════════════════════════
// tests/holographic_test.rs — 全息性验证（独立编译单元）
//
// 验证: 超出旧容器边界的内容已在相邻坐标内
// 容器不是隔离的盒子，而是三维空间中的一个切片
// ══════════════════════════════════════════════════════════════════════════

use mqf::cross_isa::coordinate::Coord3D;
use mqf::cross_isa::fridge::{ContainerPack, Fridge, TriuneJudge, Verdict};

/// 验证 ContainerPack 使用规范布局：func_code(2B)/param_slot(8B)/time_stamp(4B)
#[test]
fn container_pack_field_layout_matches_spec() {
    let coord = Coord3D::new(0x03, 0x0A10, 0x5F);
    let pack = ContainerPack::new(1, coord, 0, vec![0xB3, 0x00, 0x31, 0x00]);

    // 宽字段直接存储（规范布局）
    assert_eq!(pack.func_code, 0x0003);
    assert_eq!(pack.param_slot, 0x0A10);
    assert_eq!(pack.time_stamp, 0x0000_005F);
    assert_eq!(pack.align_flag, 0);

    // coord() 从宽字段恢复压缩坐标
    let recovered = pack.coord();
    assert_eq!(recovered.x, 0x03);
    assert_eq!(recovered.y, 0x0A10);
    assert_eq!(recovered.z, 0x5F);
    assert_eq!(recovered.to_address(), coord.to_address());
}

/// 全息性核心：Z 轴相邻容器地址差=1（真正连续）
/// 编码: [X:8/Y:16/Z:8]，Z+1 → 地址+1（最小步进）
#[test]
fn holographic_adjacent_containers_contiguous() {
    let c1 = Coord3D::new(0x03, 0x0001, 0x01);
    let c2 = Coord3D::new(0x03, 0x0001, 0x02);

    // Z 轴相邻 → 地址差 = 1（位级连续）
    assert_eq!(c2.to_address() - c1.to_address(), 1,
        "Z 轴 +1 应产生地址步进 1");

    let mut fridge = Fridge::new();
    fridge.store(ContainerPack::new(42, c1, 0, vec![0xDE, 0xAD, 0x00, 0x00]));
    fridge.store(ContainerPack::new(43, c2, 0, vec![0xBE, 0xEF, 0x00, 0x00]));

    // 同一张 HashMap 索引两个不同时刻的容器
    let p1 = fridge.fetch(c1.to_address()).unwrap();
    let p2 = fridge.fetch(c2.to_address()).unwrap();

    assert_eq!(p1.pack_id, 42);
    assert_eq!(p2.pack_id, 43);
    assert_ne!(p1.payload, p2.payload);

    // coord() 恢复坐标
    assert_eq!(p1.coord().x, 0x03);
    assert_eq!(p1.coord().z, 0x01);
    assert_eq!(p2.coord().z, 0x02);
}

/// 溢出到下一语义ID自然发生——坐标值+1就是"新容器"
#[test]
fn overflow_to_next_semantics_natural() {
    let mut fridge = Fridge::new();

    let edge: Vec<(u8, u16, u8)> = vec![
        (0x03, 0xFFFE, 0x01),  // Y 接近上限
        (0x03, 0xFFFF, 0x01),  // Y 轴极限
        (0x04, 0x0000, 0x01),  // "溢出"落到下一语义ID
        (0x04, 0x0001, 0x01),  // 继续
    ];

    for (i, (x, y, z)) in edge.iter().enumerate() {
        let coord = Coord3D::new(*x, *y, *z);
        fridge.store(ContainerPack::new(
            (i + 1) as u64,
            coord,
            0,
            vec![i as u8, 0xAA, 0x00, 0x00],
        ));
    }

    for (i, (x, y, z)) in edge.iter().enumerate() {
        let addr = Coord3D::new(*x, *y, *z).to_address();
        let pack = fridge.fetch(addr).unwrap();
        let recovered = pack.coord();
        assert_eq!(recovered.x, *x);
        assert_eq!(recovered.y, *y);
        assert_eq!(pack.pack_id, (i + 1) as u64);
    }
}

/// Z 轴 = 时间维度（执行时序），不是空间溢出
#[test]
fn z_axis_is_time_dimension() {
    let mut fridge = Fridge::new();
    let coord_t1 = Coord3D::new(0x03, 0x0001, 0x01);
    let coord_t2 = Coord3D::new(0x03, 0x0001, 0x02);

    fridge.store(ContainerPack::new(10, coord_t1, 0, vec![0xF1, 0x00, 0x00, 0x00]));
    fridge.store(ContainerPack::new(11, coord_t2, 0, vec![0xF2, 0x00, 0x00, 0x00]));

    let batch_t1 = fridge.fetch_time_slot(0x01);
    assert_eq!(batch_t1.len(), 1);
    assert_eq!(batch_t1[0].pack_id, 10);

    let batch_t2 = fridge.fetch_time_slot(0x02);
    assert_eq!(batch_t2.len(), 1);
    assert_eq!(batch_t2[0].pack_id, 11);

    // 空间重叠，时刻不同
    assert_eq!(coord_t1.x, coord_t2.x);
    assert_eq!(coord_t1.y, coord_t2.y);
}

/// 裁决机制：Z 轴对齐验证（Admit/Wait/Reject 三态）
#[test]
fn triune_judge_z_alignment() {
    let coord = Coord3D::new(0x03, 0x0001, 0x01);
    let pack = ContainerPack::new(100, coord, 0, vec![0xAA, 0xBB, 0x00, 0x00]);

    assert_eq!(TriuneJudge::judge(&pack, 0x01), Verdict::Admit);
    assert_eq!(TriuneJudge::judge(&pack, 0x99), Verdict::Wait);

    let empty = ContainerPack::new(0xFFFF, coord, 0, vec![0x01, 0x00, 0x00, 0x00]);
    assert_eq!(TriuneJudge::judge(&empty, 0x01), Verdict::Reject);
}
