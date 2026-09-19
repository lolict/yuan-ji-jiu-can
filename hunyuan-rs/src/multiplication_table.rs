// multiplication_table.rs — 乘法表 · 双通道寻址 · 公约数聚类

pub struct MulTable;

impl MulTable {
    pub fn multiply(rhyme_id: u8, stroke_value: u64) -> (u64, u64) {
        let factor = rhyme_resonance_value(rhyme_id);
        let primary = factor * stroke_value.max(1);
        let secondary = factor.wrapping_add(stroke_value);
        (primary, secondary)
    }
}

pub struct DualAddressor;

impl DualAddressor {
    pub fn locate(primary: u64, secondary: u64) -> u64 {
        primary ^ secondary
    }
}

pub fn rhyme_resonance_value(id: u8) -> u64 {
    let factors: [u64; 24] = [
        1, 2, 3, 5, 7, 11, 13, 17, 19, 23,
        29, 31, 37, 41, 43, 47, 53, 59, 61, 67,
        71, 73, 79, 83,
    ];
    factors.get(id as usize).copied().unwrap_or(1)
}

pub struct MulCluster;

impl MulCluster {
    pub fn find_resonant(id: u8) -> Vec<u8> {
        let base = rhyme_resonance_value(id);
        (0..24u8).filter(|&other| {
            if other == id { return true; }
            let other_val = rhyme_resonance_value(other);
            gcd(base, other_val) > 1
        }).collect()
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

#[derive(Clone, Debug)]
pub struct ChannelDiff {
    pub primary: u64,
    pub secondary: u64,
    pub diff: u64,
}

#[derive(Clone, Debug)]
pub struct AddressResult {
    pub addr: u64,
    pub rhyme_id: u8,
    pub stroke_value: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_basic() {
        let (p, s) = MulTable::multiply(0, 10);
        assert!(p > 0 && s > 0);
    }

    #[test]
    fn test_mul_lookup() {
        assert_eq!(rhyme_resonance_value(3), 5);
    }

    #[test]
    fn test_gcd_basic() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(7, 3), 1);
    }

    #[test]
    fn test_find_resonant_has_values() {
        // 韵母1(因子2)与所有偶数因子韵母有公约数>1
        let resonant = MulCluster::find_resonant(1);
        assert!(resonant.len() > 1);
        assert!(resonant.contains(&1));
    }
}
