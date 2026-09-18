// ═══════════════════════════════════════════════════════════════════════════════
// multiplication_table.rs — 乘法表·双通道寻址·公约数聚类
// ═══════════════════════════════════════════════════════════════════════════════
//
//   在场本体论视角:寻韵母的在场指纹,非计算它们的数值。
//   加法寻址 = 寻线性序列上的在场位置
//   乘法寻址 = 寻几何级率上的在场共振
//
// ═══════════════════════════════════════════════════════════════════════════════

use crate::rhyme::RHYME_TABLE;

/// 乘法表:N×N 乘积矩阵 (编译期内嵌)
pub struct MulTable { size: u16 }

impl MulTable {
    pub fn new() -> Self { Self { size: 64 } }
    pub fn get(&self, i: u16, j: u16) -> u32 { (i as u32) * (j as u32) }
    pub fn row(&self, i: u16) -> Vec<u32> { (0..self.size).map(|j| self.get(i, j)).collect() }
    pub fn col(&self, j: u16) -> Vec<u32> { (0..self.size).map(|i| self.get(i, j)).collect() }
    pub fn find_factors(&self, product: u32) -> Vec<(u16, u16)> {
        let mut result = Vec::new();
        let limit = (product as f64).sqrt() as u16 + 1;
        for i in 1..=limit {
            if product % (i as u32) == 0 {
                let j = (product / i as u32) as u16;
                result.push((i, j));
                if i != j { result.push((j, i)); }
            }
        }
        result
    }
}

impl Default for MulTable { fn default() -> Self { Self::new() } }

/// 双通道寻址结果
#[derive(Clone, Debug)]
pub struct AddressResult {
    pub addition_addr: u32,
    pub multiplication_addr: u32,
}

/// 双通道寻址器
pub struct DualAddressor;

impl DualAddressor {
    pub fn address(rhyme_id: u16) -> AddressResult {
        let linear_val = (rhyme_id + 1) as u32;
        let resonance_val = rhyme_resonance_value(rhyme_id).max(1);
        AddressResult {
            addition_addr: rhyme_id as u32 + linear_val,
            multiplication_addr: rhyme_id as u32 * resonance_val,
        }
    }

    pub fn relate(id_a: u16, id_b: u16) -> (AddressResult, AddressResult, ChannelDiff) {
        let a = Self::address(id_a);
        let b = Self::address(id_b);
        (a.clone(), b.clone(), ChannelDiff::compute(&a, &b))
    }
}

impl Default for DualAddressor { fn default() -> Self { Self } }

/// 通道差分
#[derive(Clone, Debug)]
pub struct ChannelDiff { pub add_delta: u32, pub mul_delta: u32 }

impl ChannelDiff {
    pub fn compute(a: &AddressResult, b: &AddressResult) -> Self {
        Self {
            add_delta: if a.addition_addr > b.addition_addr { a.addition_addr - b.addition_addr } else { b.addition_addr - a.addition_addr },
            mul_delta: if a.multiplication_addr > b.multiplication_addr { a.multiplication_addr - b.multiplication_addr } else { b.multiplication_addr - a.multiplication_addr },
        }
    }
    pub fn divergence(&self) -> f64 {
        let max = self.add_delta.max(self.mul_delta).max(1) as f64;
        let min = self.add_delta.min(self.mul_delta) as f64;
        1.0 - min / max
    }
    pub fn is_consistent(&self) -> bool { self.divergence() < 0.5 }
}

/// 公约数聚类
pub struct MulCluster;

impl MulCluster {
    pub fn find_resonant(rhyme_id: u16) -> Vec<u16> {
        let target_val = rhyme_resonance_value(rhyme_id) as u32;
        if target_val <= 1 { return Vec::new(); }
        (0..RHYME_TABLE.len() as u16)
            .filter(|&id| id != rhyme_id)
            .filter(|&id| {
                let v = rhyme_resonance_value(id) as u32;
                v > 1 && gcd(target_val, v) > 1
            })
            .collect()
    }

    pub fn cluster_by_gcd(min_gcd: u32) -> Vec<Vec<u16>> {
        let mut clusters: Vec<Vec<u16>> = Vec::new();
        for id in 0..RHYME_TABLE.len() as u16 {
            let val = rhyme_resonance_value(id) as u32;
            if val < min_gcd { continue; }
            let mut found = false;
            for cluster in &mut clusters {
                if gcd(val, rhyme_resonance_value(cluster[0]) as u32) >= min_gcd {
                    cluster.push(id);
                    found = true;
                    break;
                }
            }
            if !found { clusters.push(vec![id]); }
        }
        clusters.sort_by_key(|c| std::cmp::Reverse(c.len()));
        clusters
    }
}

impl Default for MulCluster { fn default() -> Self { Self } }

pub fn gcd(mut a: u32, mut b: u32) -> u32 { while b != 0 { let t = b; b = a % b; a = t; } a }

fn rhyme_resonance_value(id: u16) -> u32 {
    match id {
        0 => 2, 1 => 3, 2 => 5, 3 => 4, 4 => 6, 5 => 8, 6 => 7, 7 => 9, 8 => 10, 9 => 12,
        10 => 4, 11 => 6, 12 => 8, 13 => 10, 14 => 14, 15 => 5, 16 => 7, 17 => 9,
        18 => 15, 19 => 6, 20 => 12, 21 => 16, 22 => 18, 23 => 20, 24 => 10,
        25 => 12, 26 => 14, 27 => 15, 28 => 16, 29 => 18, 30 => 20, 31 => 22,
        32 => 24, 33 => 3, 34 => 6, 35 => 9, 36 => 12, 37 => 15, 38 => 18,
        39 => 21, 40 => 24,
        _ => ((id + 1) % 12 + 1) as u32,
    }
}

pub fn mul_quick_lookup(a: &str, b: &str) -> u32 {
    let idx_a = rhyme_spelling_to_index(a);
    let idx_b = rhyme_spelling_to_index(b);
    (idx_a + 1) as u32 * (idx_b + 1) as u32
}

fn rhyme_spelling_to_index(spelling: &str) -> u16 {
    for (i, r) in RHYME_TABLE.iter().enumerate() {
        if r.spelling == spelling { return i as u16; }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_basic() {
        let t = MulTable::new();
        assert_eq!(t.get(3, 4), 12);
        assert_eq!(t.get(7, 8), 56);
    }

    #[test]
    fn test_gcd_basic() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(7, 13), 1);
    }

    #[test]
    fn test_find_resonant_has_values() {
        let resonant = MulCluster::find_resonant(10); // id=10=an, val=4
        assert!(!resonant.is_empty());
    }

    #[test]
    fn test_mul_lookup() {
        let r = mul_quick_lookup("a", "o");
        assert!(r > 0);
    }
}
