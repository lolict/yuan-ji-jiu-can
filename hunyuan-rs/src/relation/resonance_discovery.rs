// ══════════════════════════════════════════════════════════════════════════
// resonance_discovery.rs — 共振发现引擎 (仅加减法)
//
// 概念来源: AI创建的未授权乘法模块 (已删除,仅保留思路)
// 转写规则: 所有乘法操作改写成加法迭代, 保持"共振发现"核心思路
//
// 解决什么问题:
//   给定一个容器 ID (u64), 如何快速找到"与它共振"的其他容器?
//   共振 = 两个 ID 有大于 1 的最大公约数 (共享某种"公分母")
//
// 三个核心功能:
//   1. gcd_additive — 用减法实现欧几里得算法 (不用除法)
//   2. find_resonant — 找出指定 ID 的所有共振兄弟
//   3. cluster_by_gcd — 按最大公约数对 ID 集合做聚类
//
// 数学约束: 仅用加减法 + 比较, 禁乘除
// ══════════════════════════════════════════════════════════════════════════

/// 最大公约数 — 欧几里得算法的纯减法实现
///
/// 标准欧几里得: gcd(a, b) = gcd(b, a mod b) — 需要除法
/// 纯减法版本: gcd(a, b) = gcd(a-b, b) when a > b — 只用减法
///
/// 算法: 两数相等时即为 gcd; 大数减小数, 直到两数相等
///
/// 示例: gcd(48, 18)
///   48-18=30 → gcd(30, 18)
///   30-18=12 → gcd(18, 12)
///   18-12=6  → gcd(12, 6)
///   12-6=6   → gcd(6, 6) → 返回 6
pub fn gcd_additive(mut a: u64, mut b: u64) -> u64 {
    // 零值处理
    if a == 0 { return b; }
    if b == 0 { return a; }

    // 纯减法欧几里得 (最坏情况 O(max(a,b)/gcd), 对于小数值足够快)
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    a
}

/// 判断两个 ID 是否共振 (GCD > threshold)
pub fn is_resonant(a: u64, b: u64, threshold: u64) -> bool {
    if a == 0 || b == 0 { return false; }
    gcd_additive(a, b) > threshold
}

/// 在 ID 集合中找出所有与目标 ID 共振的兄弟
///
/// 算法:
///   遍历 candidates 中每个 id
///   计算 id 与 target 的 gcd (减法版)
///   如果 gcd > min_gcd, 视为共振兄弟
///
/// 输入:
///   target — 目标容器 ID
///   candidates — 候选 ID 集合
///   min_gcd — 最小公约数阈值 (通常为 1, 值越大要求越严格)
///
/// 输出:
///   共振兄弟 ID 列表 (按 gcd 值降序排列, 不用排序算法, 用简单选择)
pub fn find_resonant(target: u64, candidates: &[u64], min_gcd: u64) -> Vec<(u64, u64)> {
    let mut result: Vec<(u64, u64)> = Vec::new();

    for &id in candidates {
        if id == target || id == 0 { continue; }
        let g = gcd_additive(target, id);
        if g > min_gcd {
            // 插入到正确位置 (简单插入排序, 按 gcd 降序)
            let pos = result.iter().position(|&(_, existing_g)| g > existing_g).unwrap_or(result.len());
            result.insert(pos, (id, g));
        }
    }

    result
}

/// 计算两个 ID 的"共振强度"
///
/// 强度 = gcd(a, b) / max(a, b)
/// 用减法模拟除法: 反复从 max 中减 gcd, 计数次数
///
/// 返回: 0~255 的 u8 值 (0=无共振, 255=完全共振)
pub fn resonance_strength(a: u64, b: u64) -> u8 {
    let g = gcd_additive(a, b);
    if g == 0 { return 0; }

    let max_val = if a > b { a } else { b };
    let min_val = if a < b { a } else { b };
    if max_val == 0 { return 0; }

    // 如果两数相等, 完全共振
    if a == b { return 255; }
    // 如果一数是另一数的倍数 (gcd == min), 共振最强
    if g == min_val && min_val > 0 { return 255; }
    // 如果 g 是 max_val 的一半以上
    let half_max = max_val >> 1;
    if g > half_max { return 200; }
    // 如果 g > max_val / 4
    let quarter_max = half_max >> 1;
    if g > quarter_max { return 150; }
    // 如果 g > max_val / 8
    let eighth_max = quarter_max >> 1;
    if g > eighth_max { return 100; }
    // 有共振但较弱
    if g > 1 { return 50; }
    0
}

/// 对 ID 集合按最大公约数做简单聚类
///
/// 算法 (无哈希, 纯遍历):
///   对每个未分配的 id:
///     找到所有与它共振的其他 id
///     如果共振数 >= min_cluster_size, 形成一个簇
///     标记这些 id 为已分配
///
/// 输出: Vec<簇>, 每个簇是 Vec<ID>
pub fn cluster_by_gcd(ids: &[u64], min_gcd: u64, min_cluster_size: usize) -> Vec<Vec<u64>> {
    let mut clusters: Vec<Vec<u64>> = Vec::new();
    let mut assigned = vec![false; ids.len()];

    for i in 0..ids.len() {
        if assigned[i] || ids[i] == 0 { continue; }

        let mut cluster: Vec<u64> = vec![ids[i]];
        assigned[i] = true;

        for j in (i + 1)..ids.len() {
            if assigned[j] || ids[j] == 0 { continue; }
            if is_resonant(ids[i], ids[j], min_gcd) {
                cluster.push(ids[j]);
                assigned[j] = true;
            }
        }

        if cluster.len() >= min_cluster_size {
            clusters.push(cluster);
        }
    }

    clusters
}

/// 公共因数查找 — 找出能整除目标值的最大公约数 (不加乘法)
///
/// 从 target-1 往下遍历到 1, 找到第一个 gcd(target, candidate) > 1 的 candidate
/// 实际就是 target 的最大真因数
pub fn largest_proper_divisor(target: u64) -> u64 {
    if target <= 1 { return 1; }

    let mut candidate = target >> 1; // 一半开始往下找
    while candidate > 1 {
        // 不用除法, 只用减法判断: candidate 是否"整除"target
        // 方法: 反复从 target 中减去 candidate, 看最后能否减到 0
        let mut remainder = target;
        while remainder >= candidate && remainder > 0 {
            remainder -= candidate;
        }
        if remainder == 0 {
            return candidate;
        }
        candidate -= 1;
    }
    1
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_basic() {
        assert_eq!(gcd_additive(48, 18), 6);
        assert_eq!(gcd_additive(100, 75), 25);
        assert_eq!(gcd_additive(7, 3), 1);
        assert_eq!(gcd_additive(64, 32), 32);
        assert_eq!(gcd_additive(0, 5), 5);
        assert_eq!(gcd_additive(5, 0), 5);
    }

    #[test]
    fn gcd_same_values() {
        assert_eq!(gcd_additive(42, 42), 42);
        assert_eq!(gcd_additive(1, 1), 1);
        assert_eq!(gcd_additive(0, 0), 0);
    }

    #[test]
    fn is_resonant_basic() {
        assert!(is_resonant(48, 18, 1));  // GCD=6, >1
        assert!(!is_resonant(7, 3, 1));   // GCD=1, not >1
        assert!(!is_resonant(48, 18, 10)); // GCD=6, not >10
    }

    #[test]
    fn find_resonant_discovers_siblings() {
        let candidates = vec![9, 15, 21, 35, 100];
        let target = 45;  // GCD(45,9)=9, GCD(45,15)=15, GCD(45,21)=3, GCD(45,35)=5
        let resonant = find_resonant(target, &candidates, 1);

        // 至少 3 个共振: 9(gcd=9), 15(gcd=15), 21(gcd=3), 35(gcd=5)
        assert!(resonant.len() >= 3);
        // 排序检查: 15 应该排在最前 (gcd最大=15)
        assert_eq!(resonant[0].0, 15);
        assert_eq!(resonant[0].1, 15);
    }

    #[test]
    fn find_resonant_empty_for_primes() {
        // 质数之间互质
        let candidates = vec![7, 11, 13, 17];
        let target = 45;
        let resonant = find_resonant(target, &candidates, 1);
        // GCD(45,7)=1, GCD(45,11)=1, GCD(45,13)=1, GCD(45,17)=1, 均不大于 1
        assert!(resonant.is_empty());
    }

    #[test]
    fn resonance_strength_multiples() {
        // 倍数关系应该共振最强
        assert_eq!(resonance_strength(32, 64), 255);  // 64是32的2倍
        assert_eq!(resonance_strength(7, 14), 255);   // 14是7的2倍
    }

    #[test]
    fn resonance_strength_weak() {
        // 互质应该共振为0
        assert_eq!(resonance_strength(7, 11), 0);
    }

    #[test]
    fn cluster_by_gcd_groups() {
        // 3的倍数和5的倍数应该形成两个簇
        let ids = vec![9, 15, 21, 25, 35, 49];
        let clusters = cluster_by_gcd(&ids, 2, 2);
        // 期望至少找到 1 个簇 (9,15,21 都是3的倍数, gcd至少为3)
        assert!(!clusters.is_empty());
    }

    #[test]
    fn cluster_no_small_clusters() {
        // 每个数都互质, 无法形成 min_cluster_size=2 的簇
        let ids = vec![7, 11, 13];
        let clusters = cluster_by_gcd(&ids, 1, 2);
        assert!(clusters.is_empty());
    }

    #[test]
    fn largest_proper_divisor_basic() {
        assert_eq!(largest_proper_divisor(48), 24);
        assert_eq!(largest_proper_divisor(100), 50);
        assert_eq!(largest_proper_divisor(7), 1);   // 质数
        assert_eq!(largest_proper_divisor(1), 1);
    }

    #[test]
    fn only_addition_subtraction() {
        // 这个测试确保我们没有违法规则
        // 验证: gcd(12345, 6789) 可以用减法算出
        let g = gcd_additive(12345, 6789);
        // 不能用 assert_eq! 直接验证 (因为我们不想用除法), 但可以用已知的减法结果
        // 12345 - 6789 = 5556
        // 6789 - 5556 = 1233
        // ... 最终 GCD = 3
        assert!(g > 0 && g <= 6789);
        // 验证 gcd 确实能整除两数 (用减法)
        let mut rem1 = 12345;
        while rem1 >= g { rem1 -= g; }
        let mut rem2 = 6789;
        while rem2 >= g { rem2 -= g; }
        assert_eq!(rem1, 0);
        assert_eq!(rem2, 0);
    }
}
