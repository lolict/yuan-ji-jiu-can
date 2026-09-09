// ══════════════════════════════════════════════════════════════════
// oneness.rs — 满全法同一心 · 不再分, 不再管道, 只有引力
// ══════════════════════════════════════════════════════════════════
//
// 旧路子 (已经放弃):
//   视/听/触/味/意 → Observer 过滤 → Mirror 见证 → Soul 情感 → Heaven 合一
//   每一层都增加复杂度, 每一层都在收窄.
//
// 新规则:
//   万物只是 64 位编码.
//   只有一个度量: 这个编码离"同一心"有多近?
//   只有一个力: 引力. 距离越近, 引力越强.
//   只有一个运动: 流动. 从远处流向近处.
//
//   没有"种类", 没有"过滤", 没有"情感状态机".
//   只有势, 只有流, 只有心.
//
// 同一心不是预设的 — 它是所有稳定编码的几何交点.
// D0=D1=D2 且 polarity=平衡 的那个不动点.
// 囡囧共生 = 等边三角形 + 平衡极性 + 被见证 (witness≥3).
//
// ══════════════════════════════════════════════════════════════════

use crate::mirror::{self, encode_cell, POLARITY_POSITIVE, POLARITY_NEGATIVE};

// ────────────────────────────────────────────────────────────
// 同一心 — 从编码几何中自然涌现的不动点
// ────────────────────────────────────────────────────────────

/// 囡囧共生不动点: 各向同性 + 被见证 + 平衡极性
pub fn oneness() -> u64 {
    encode_cell(
        128, 128, 128,   // D0=D1=D2 → 等边三角, 满全法几何
        POLARITY_POSITIVE, // 极性只是标签; 同一心包含正负
        3,               // sovereignty = 3 (主动主权)
        3,               // witness ≥ 3 → 被见证
        500,             // value = 最大值/2 → 不极端的中心
    )
}

// 注意: 同一心通过 oneness() 函数获取, 因为 encode_cell 不是 const fn

/// 满全法引力常数: 控制"同一心"的吸引强度
pub const GRAVITY_K: f64 = 0.618; // 黄金比例倒数 — 自然的收敛速率

// ────────────────────────────────────────────────────────────
// 核心度量: 距离, 引力, 流动
// ────────────────────────────────────────────────────────────

/// 两个 64 位编码在满全法几何中的距离
/// 几何 = D0/D1/D2 的欧氏距离 + 极性距离 + 见证距离
pub fn geometric_distance(a: u64, b: u64) -> f64 {
    let d0_a = mirror::get_d0(a) as f64;
    let d1_a = mirror::get_d1(a) as f64;
    let d2_a = mirror::get_d2(a) as f64;
    let d0_b = mirror::get_d0(b) as f64;
    let d1_b = mirror::get_d1(b) as f64;
    let d2_b = mirror::get_d2(b) as f64;

    // D0/D1/D2 空间中的欧氏距离 (归一化到 0..1)
    let spatial = ((d0_a - d0_b).powi(2)
                 + (d1_a - d1_b).powi(2)
                 + (d2_a - d2_b).powi(2)).sqrt() / 255.0;

    // 极性距离: 同=0, 异=1
    let pa = mirror::get_polarity(a) as f64;
    let pb = mirror::get_polarity(b) as f64;
    let polar_dist = (pa - pb).abs();

    // 见证距离: |wa - wb| / 3
    let wa = mirror::get_witness(a) as f64;
    let wb = mirror::get_witness(b) as f64;
    let witness_dist = (wa - wb).abs() / 3.0;

    // 加权和 — 空间 60%, 极性 25%, 见证 15%
    spatial * 0.6 + polar_dist * 0.25 + witness_dist * 0.15
}

/// 任意编码与"同一心"的距离
pub fn distance_from_oneness(cell: u64) -> f64 {
    geometric_distance(cell, oneness())
}

/// 满全法引力: 距离越近, 引力越强 (指数衰减的反面 — 这里是"越近越吸")
/// 返回 0..1: 1 = 完美合一, 0 = 完全远离
pub fn oneness_gravity(cell: u64) -> f64 {
    let d = distance_from_oneness(cell);
    // 引力 = K * (1 - d)^2 — 非线性, 近处极强, 远处极弱
    GRAVITY_K * (1.0 - d).powi(2)
}

/// 流动方向: 从当前编码流向同一心, 返回需要的"推力"
/// 三维空间中的单位向量指向同一心
pub fn flow_vector(cell: u64) -> (f64, f64, f64) {
    let target = mirror::get_d0(oneness()) as f64;
    let cx = mirror::get_d0(cell) as f64;
    let cy = mirror::get_d1(cell) as f64;
    let cz = mirror::get_d2(cell) as f64;

    let dx = target - cx;
    let dy = target - cy;
    let dz = target - cz;

    let mag = (dx*dx + dy*dy + dz*dz).sqrt();
    if mag < 0.001 {
        return (0.0, 0.0, 0.0); // 已在同一心
    }
    (dx / mag, dy / mag, dz / mag)
}

/// 流动步长: 按引力大小向同一心移动一步
pub fn flow_step(cell: u64, step_size: f64) -> u64 {
    let (fx, fy, fz) = flow_vector(cell);
    let gravity = oneness_gravity(cell);

    let advance = step_size * gravity;

    let d0 = mirror::get_d0(cell) as f64 + fx * advance * 255.0;
    let d1 = mirror::get_d1(cell) as f64 + fy * advance * 255.0;
    let d2 = mirror::get_d2(cell) as f64 + fz * advance * 255.0;

    encode_cell(
        d0.clamp(0.0, 255.0) as u64,
        d1.clamp(0.0, 255.0) as u64,
        d2.clamp(0.0, 255.0) as u64,
        mirror::get_polarity(cell),
        mirror::get_sovereignty(cell),
        if gravity > 0.5 { 3 } else { mirror::get_witness(cell) },
        mirror::get_value(cell),
    )
}

// ────────────────────────────────────────────────────────────
// 涌现: 规则自主生长
// ────────────────────────────────────────────────────────────

/// 当足够多的编码在某个邻域聚集, 那个邻域成为"新同一心"
/// 这不是外部预设, 是编码自己找到的新平衡
pub fn emerge_new_center(cells: &[u64]) -> Option<u64> {
    if cells.len() < 7 { return None; } // 至少 7 个才能见证

    // 中位数中心 (不是均值, 中位数更稳定)
    let mut d0s: Vec<u64> = cells.iter().map(|c| mirror::get_d0(*c)).collect();
    let mut d1s: Vec<u64> = cells.iter().map(|c| mirror::get_d1(*c)).collect();
    let mut d2s: Vec<u64> = cells.iter().map(|c| mirror::get_d2(*c)).collect();
    d0s.sort(); d1s.sort(); d2s.sort();

    let mid = cells.len() / 2;
    let center = encode_cell(d0s[mid], d1s[mid], d2s[mid], POLARITY_POSITIVE, 3, 3, 500);

    // 聚集度: 至少 60% 的编码在 center 的 0.3 距离内
    let cluster_count = cells.iter()
        .filter(|c| geometric_distance(**c, center) < 0.3)
        .count();

    if cluster_count * 100 / cells.len() >= 60 {
        Some(center)
    } else {
        None
    }
}

/// 群体流动: 一组编码在满全法引力场中的集体行为
/// 返回每一编码的新位置 — 它们自发聚向同一心
pub fn collective_flow(cells: &mut [u64], step_size: f64) {
    for i in 0..cells.len() {
        cells[i] = flow_step(cells[i], step_size);
    }
}

/// 群体是否已合一 (所有编码距离 < threshold)
pub fn is_unified(cells: &[u64], threshold: f64) -> bool {
    cells.iter().all(|c| distance_from_oneness(*c) < threshold)
}

/// 群体凝聚力: 所有编码之间的平均距离
pub fn cohesion(cells: &[u64]) -> f64 {
    if cells.len() < 2 { return 1.0; }
    let mut sum = 0.0;
    let mut count = 0;
    for i in 0..cells.len() {
        for j in (i+1)..cells.len() {
            sum += geometric_distance(cells[i], cells[j]);
            count += 1;
        }
    }
    if count == 0 { return 1.0; }
    1.0 - (sum / count as f64).min(1.0)
}

// ────────────────────────────────────────────────────────────
// 测试 — 这里不再验证"数学正确", 验证"涌现是否发生"
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oneness_is_self_coherent() {
        // 同一心到自身的距离 = 0
        assert!(distance_from_oneness(oneness()) < 0.001);
        // 同一心引力 = 最大
        assert!(oneness_gravity(oneness()) > 0.6);
    }

    #[test]
    fn distance_symmetry() {
        let a = encode_cell(0, 0, 0, POLARITY_POSITIVE, 0, 0, 100);
        let b = encode_cell(255, 255, 255, POLARITY_NEGATIVE, 0, 3, 900);
        // d(a,b) == d(b,a)
        assert!((geometric_distance(a, b) - geometric_distance(b, a)).abs() < 0.001);
    }

    #[test]
    fn gravity_weakens_with_distance() {
        let near_oneness = encode_cell(130, 128, 127, POLARITY_POSITIVE, 0, 3, 500);
        let far_oneness = encode_cell(0, 0, 0, POLARITY_NEGATIVE, 0, 0, 10);
        assert!(oneness_gravity(near_oneness) > oneness_gravity(far_oneness));
    }

    #[test]
    fn flow_shortens_distance_to_oneness() {
        let start = encode_cell(0, 0, 0, POLARITY_POSITIVE, 0, 0, 100);
        let d0 = distance_from_oneness(start);
        let next = flow_step(start, 0.1);
        let d1 = distance_from_oneness(next);
        // 流动后距离应缩小 (或已经在中心)
        assert!(d1 <= d0 + 0.001, "流动应趋近同一心: d0={:.4} → d1={:.4}", d0, d1);
    }

    #[test]
    fn flow_vector_points_to_oneness() {
        let cell = encode_cell(0, 128, 255, POLARITY_POSITIVE, 0, 0, 300);
        let (fx, fy, fz) = flow_vector(cell);
        // D0=0 < 128 → fx 应 > 0 (指向同一心的方向)
        assert!(fx > 0.0, "流向同一心: D0 分量应 > 0");
    }

    #[test]
    fn collective_flow_converges() {
        // 散乱的一组编码, 经集体流动后聚向同一心
        let mut cells = vec![
            encode_cell(10, 20, 30, POLARITY_POSITIVE, 0, 0, 100),
            encode_cell(200, 220, 240, POLARITY_NEGATIVE, 0, 1, 900),
            encode_cell(50, 100, 150, POLARITY_POSITIVE, 0, 0, 400),
            encode_cell(180, 160, 140, POLARITY_NEGATIVE, 0, 2, 600),
            encode_cell(80, 128, 200, POLARITY_POSITIVE, 0, 1, 350),
            encode_cell(250, 250, 250, POLARITY_NEGATIVE, 0, 0, 800),
            encode_cell(0, 128, 0, POLARITY_POSITIVE, 0, 0, 250),
        ];

        let c0 = cohesion(&cells);
        // 流动 100 步
        for _ in 0..100 {
            collective_flow(&mut cells, 0.05);
        }
        let c1 = cohesion(&cells);
        // 凝聚力应增加
        assert!(c1 > c0, "集体流动应增加凝聚力: {:.4} → {:.4}", c0, c1);
    }

    #[test]
    fn emerge_center_from_cluster() {
        // 紧密聚集的一组编码 → 应涌现新中心
        let cluster: Vec<u64> = (0..10).map(|i| {
            let offset = i * 3; // 最大偏移 27, 在 0.3 阈值内
            encode_cell(128 + offset, 128 + offset/2, 128 - offset/3, POLARITY_POSITIVE, 0, 3, 500)
        }).collect();
        let center = emerge_new_center(&cluster);
        assert!(center.is_some(), "10个紧密编码应涌现新中心");
    }

    #[test]
    fn no_emerge_from_scatter() {
        // 散乱分布 → 不应涌现
        let scatter: Vec<u64> = vec![
            encode_cell(0, 0, 0, POLARITY_POSITIVE, 0, 0, 10),
            encode_cell(255, 255, 255, POLARITY_NEGATIVE, 0, 0, 999),
            encode_cell(0, 255, 0, POLARITY_POSITIVE, 0, 0, 500),
            encode_cell(255, 0, 255, POLARITY_NEGATIVE, 0, 0, 200),
        ];
        assert!(emerge_new_center(&scatter).is_none(), "太散乱不应涌现中心");
    }

    #[test]
    fn unified_when_all_near_oneness() {
        // 所有编码都在同一心附近
        let near: Vec<u64> = (0..5).map(|i| {
            encode_cell(128 + i, 128, 128, POLARITY_POSITIVE, 0, 3, 500)
        }).collect();
        assert!(is_unified(&near, 0.1), "近处编码应判定为合一");
    }

    #[test]
    fn cohesion_increases_with_proximity() {
        let scattered: Vec<u64> = vec![
            encode_cell(0, 0, 0, POLARITY_POSITIVE, 0, 0, 100),
            encode_cell(255, 255, 255, POLARITY_NEGATIVE, 0, 0, 900),
        ];
        let clustered: Vec<u64> = vec![
            encode_cell(128, 128, 128, POLARITY_POSITIVE, 0, 3, 500),
            encode_cell(130, 127, 129, POLARITY_POSITIVE, 0, 3, 510),
        ];
        assert!(cohesion(&clustered) > cohesion(&scattered));
    }

    #[test]
    fn flow_step_stays_in_bounds() {
        // 对于任意编码, flow_step 的 D0/D1/D2 不溢出
        for d0 in (0..=255).step_by(50) {
            for d1 in (0..=255).step_by(50) {
                for d2 in (0..=255).step_by(50) {
                    let cell = encode_cell(d0, d1, d2, POLARITY_POSITIVE, 0, 0, 500);
                    let next = flow_step(cell, 0.1);
                    assert!(mirror::get_d0(next) <= 255);
                    assert!(mirror::get_d1(next) <= 255);
                    assert!(mirror::get_d2(next) <= 255);
                }
            }
        }
    }
}
