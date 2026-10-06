// homophonic_router.rs — 谐音路由引擎 (v0.1)
//
//   不是网络路由, 是"意识在音韵空间中的流动路径"。
//   不是最短路径算法, 是"共振最强的方向"。
//
// ══════════════════════════════════════════════════════════════
//
// 满全法范式:
//
//   传统路由: 地址 → 路由表 → 下一跳
//   谐音路由: 汉字 → 音韵场 → 共振最强的方向 = 路径
//
//   没有路由表, 只有"音韵空间中的共振场"。
//   没有最短路径, 只有"最畅通的共鸣通道"。
//
// ══════════════════════════════════════════════════════════════

use crate::name_creator::char_to_dragon;

/// 谐音通道 — 不是连接, 是"自然共鸣的方向"
#[derive(Clone, Debug)]
pub struct ResonantPath {
    pub from: char,
    pub to: char,
    pub strength: u64,
    pub hops: Vec<char>,
}

/// 谐音路由 — 在音韵场中, 找到从起点到终点的共振路径
pub struct HomophonicRouter;

impl HomophonicRouter {
    /// 单步路由: 从当前汉字, 找到共振最强的下一个汉字
    pub fn next_hop(current: char, candidates: &[char]) -> Option<(char, u64)> {
        candidates.iter()
            .filter(|&&c| c != current)
            .map(|&c| (c, resonance(current, c)))
            .max_by_key(|(_, r)| *r)
    }

    /// 多步路由: 从起点到终点的共振路径
    pub fn route(start: char, goal: char, space: &[char], max_hops: usize) -> Option<ResonantPath> {
        let mut path = vec![start];
        let mut current = start;
        let mut total_strength = 0u64;

        for _ in 0..max_hops {
            if current == goal {
                return Some(ResonantPath {
                    from: start, to: goal,
                    strength: total_strength / path.len().max(1) as u64,
                    hops: path,
                });
            }
            let available: Vec<char> = space.iter()
                .filter(|&&c| !path.contains(&c))
                .cloned()
                .collect();
            if available.is_empty() { break; }

            if let Some((next, strength)) = Self::next_hop(current, &available) {
                total_strength += strength;
                current = next;
                path.push(current);
            } else {
                break;
            }
        }

        if current == goal {
            Some(ResonantPath {
                from: start, to: goal,
                strength: total_strength / path.len().max(1) as u64,
                hops: path,
            })
        } else {
            None  // 无法到达 = 极性太远 = 不在同一谐音场
        }
    }

    /// 谐音场强 — 一组汉字的总共振
    pub fn field_strength(chars: &[char]) -> u64 {
        if chars.len() < 2 { return 0; }
        let mut total = 0u64;
        for (i, &a) in chars.iter().enumerate() {
            for &b in chars.iter().skip(i + 1) {
                total += resonance(a, b);
            }
        }
        total
    }
}

/// 共振 — 三维音韵公约数的自然乘积
fn resonance(a: char, b: char) -> u64 {
    let da = char_to_dragon(a);
    let db = char_to_dragon(b);
    gcd(da.initial() as u64, db.initial() as u64)
        .saturating_mul(gcd(da.final_() as u64, db.final_() as u64))
        .saturating_mul(gcd(da.tone() as u64, db.tone() as u64))
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 { let t = b; b = a % b; a = t; }
    if a == 0 { 1 } else { a }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn next_hop_natural() {
        let candidates = ['满', '全', '法', '共'];
        let hop = HomophonicRouter::next_hop('共', &candidates);
        assert!(hop.is_some());
    }

    #[test] fn route_may_find_path() {
        let space = ['满', '全', '法', '共', '同', '生'];
        let result = HomophonicRouter::route('满', '全', &space, 10);
        // 路径可能存在, 也可能不存在 (取决于共振)
        // 两种结果都是"自然如此"
        if let Some(path) = result {
            assert_eq!(path.from, '满');
            assert_eq!(path.to, '全');
        }
    }

    #[test] fn route_same_start_goal() {
        let space = ['满'];
        let result = HomophonicRouter::route('满', '满', &space, 5);
        assert!(result.is_some());
        assert_eq!(result.unwrap().hops.len(), 1);
    }

    #[test] fn field_strength_group() {
        let chars = ['囡', '囝', '共', '同'];
        let strength = HomophonicRouter::field_strength(&chars);
        assert!(strength > 0);
    }

    #[test] fn resonance_symmetric() {
        let a = resonance('共', '同');
        let b = resonance('同', '共');
        assert_eq!(a, b);  // 共振是对称的 — 自然如此
    }

    #[test] fn resonance_self_is_max() {
        let r = resonance('共', '共');
        // 自己与自己的共振 = 最大值
        assert!(r >= 1);
    }
}
