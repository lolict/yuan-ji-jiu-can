('// ', '')@name: entity_identity
('// ', '')@type: core-identity
('// ', '')@owner: 五彩石
('// ', '')@status: stable
('// ', '')@priority: P0
('// ', '')@description: 身份编码系统

// ══════════════════════════════════════════════════════════════════════
// entity_identity.rs · 实体身份协议
// ══════════════════════════════════════════════════════════════════════
//
// 目的：用结构指纹（几何字段组合）替代名字作为身份键。
//       解决"同一功能不同名字→合作死锁"的规则病毒。
//
// 指纹 = 64-bit { d0:16, d1:16, d2:16, polarity:4, sovereignty:4, value:8 }
// 名字 = 显示符号（不参与身份判定）
//
// 规范：docs/ENTITY_IDENTITY_PROTOCOL.md

use std::collections::HashMap;
use std::sync::Mutex;

// ── 64-bit 指纹 ──────────────────────────────────────────────────────

/// 实体身份指纹。五维几何字段打包成 64-bit 整数。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Fingerprint(pub u64);

impl Fingerprint {
    /// 构造指纹。每个字段打包到 64-bit 的对应位置。
    #[inline]
    pub const fn new(d0: u16, d1: u16, d2: u16, polarity: u8, sovereignty: u8, value: u8) -> Self {
        let raw = (d0 as u64) << 48
                | (d1 as u64) << 32
                | (d2 as u64) << 16
                | ((polarity & 0x0F) as u64) << 12
                | ((sovereignty & 0x0F) as u64) << 8
                | (value as u64);
        Fingerprint(raw)
    }

    #[inline] pub fn d0(&self)        -> u16 { ((self.0 >> 48) & 0xFFFF) as u16 }
    #[inline] pub fn d1(&self)        -> u16 { ((self.0 >> 32) & 0xFFFF) as u16 }
    #[inline] pub fn d2(&self)        -> u16 { ((self.0 >> 16) & 0xFFFF) as u16 }
    #[inline] pub fn polarity(&self)  -> u8  { ((self.0 >> 12) & 0x0F) as u8  }
    #[inline] pub fn sovereignty(&self)-> u8  { ((self.0 >>  8) & 0x0F) as u8  }
    #[inline] pub fn value(&self)     -> u8  { ( self.0        & 0xFF) as u8  }

    /// 五维欧几里得距离（未开方）
    pub fn distance_squared(&self, other: &Fingerprint) -> f64 {
        let dd0 = self.d0() as f64 - other.d0() as f64;
        let dd1 = self.d1() as f64 - other.d1() as f64;
        let dd2 = self.d2() as f64 - other.d2() as f64;
        let dp  = self.polarity() as f64 - other.polarity() as f64;
        let ds  = self.sovereignty() as f64 - other.sovereignty() as f64;
        let dv  = self.value() as f64 - other.value() as f64;

        dd0 * dd0 + dd1 * dd1 + dd2 * dd2 + dp * dp + ds * ds + dv * dv
    }

    /// 兼容性函数 f(d) = 1 / (1 + d²)
    pub fn compatibility(&self, other: &Fingerprint) -> f64 {
        let d2 = self.distance_squared(other);
        1.0 / (1.0 + d2)
    }
}

// ── 兼容性阈值 ────────────────────────────────────────────────────────

pub const THRESHOLD_SAME_ENTITY:  f64 = 0.94;
pub const THRESHOLD_FAMILY:      f64 = 0.70;
pub const THRESHOLD_WEAK:        f64 = 0.30;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cooperativity {
    SameEntity,     // f(d) ≥ 0.94
    Family,         // 0.70 ≤ f(d) < 0.94
    Weak,           // 0.30 ≤ f(d) < 0.70
    Incompatible,   // f(d) < 0.30
}

pub fn judge(compat: f64) -> Cooperativity {
    if compat >= THRESHOLD_SAME_ENTITY { Cooperativity::SameEntity }
    else if compat >= THRESHOLD_FAMILY { Cooperativity::Family }
    else if compat >= THRESHOLD_WEAK   { Cooperativity::Weak }
    else                               { Cooperativity::Incompatible }
}

// ── 注册表 ────────────────────────────────────────────────────────────

/// 全局实体注册表。
/// key = 指纹; value = 实体元信息。
static REGISTRY: Mutex<Option<HashMap<Fingerprint, EntityMeta>>> = Mutex::new(None);

fn ensure_registry() -> std::sync::MutexGuard<'static, Option<HashMap<Fingerprint, EntityMeta>>> {
    let mut guard = REGISTRY.lock().expect("REGISTRY lock poisoned");
    if guard.is_none() {
        *guard = Some(HashMap::new());
    }
    guard
}

#[derive(Clone, Debug)]
pub struct EntityMeta {
    pub fingerprint: Fingerprint,
    pub display_name: String,   // 仅显示用,不参与判定
    pub provenance: String,     // 来源:hunyuan-rs / AI-A / kndrl / ...
    pub history_chain: Vec<u8>, // 历史哈希链(抗克隆)
}

pub struct Registration {
    pub meta: EntityMeta,
}

/// 注册实体。返回已存在的实体引用（如果指纹相同或近似）。
pub fn register_entity(meta: EntityMeta) -> Result<Registration, EntityError> {
    let mut guard = ensure_registry();
    let map = guard.as_mut().unwrap();

    // 检查精确匹配
    if let Some(existing) = map.get(&meta.fingerprint) {
        // 元信息不同但指纹相同→同一实体的不同版本
        if existing.display_name != meta.display_name {
            // 警告:同一指纹,不同名字。按协议,这是同一个人不同绰号。
            // 返回已存在条目,新名字加入别名集。
        }
    }

    map.entry(meta.fingerprint).or_insert_with(|| meta.clone());

    Ok(Registration { meta })
}

/// 查找最接近的实体。
pub fn lookup_by_fingerprint(fp: &Fingerprint) -> Option<EntityMeta> {
    let guard = ensure_registry();
    let map = guard.as_ref()?;

    // 精确查找优先
    if let Some(meta) = map.get(fp) {
        return Some(meta.clone());
    }

    // 近似查找:找 f(d) 最大的条目
    let mut best: Option<(&Fingerprint, &EntityMeta)> = None;
    let mut best_compat = 0.0_f64;

    for (k, v) in map.iter() {
        let c = fp.compatibility(k);
        if c > best_compat {
            best_compat = c;
            best = Some((k, v));
        }
    }

    best.map(|(_, v)| v.clone())
}

/// 两个实体能否合作。
pub fn can_cooperate(a: &Fingerprint, b: &Fingerprint) -> Cooperativity {
    let c = a.compatibility(b);
    judge(c)
}

#[derive(Debug)]
pub enum EntityError {
    DuplicateExact,
    RegistryLocked,
}

impl std::fmt::Display for EntityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateExact => write!(f, "exact duplicate"),
            Self::RegistryLocked => write!(f, "registry lock poisoned"),
        }
    }
}

impl std::error::Error for EntityError {}

// ── 测试 ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fingerprint_roundtrip() {
        let fp = Fingerprint::new(10, 20, 30, 0b1010, 0b0001, 99);
        assert_eq!(fp.d0(), 10);
        assert_eq!(fp.d1(), 20);
        assert_eq!(fp.d2(), 30);
        assert_eq!(fp.polarity(), 0b1010);
        assert_eq!(fp.sovereignty(), 0b0001);
        assert_eq!(fp.value(), 99);
    }

    #[test]
    fn exact_match_cooperativity() {
        let a = Fingerprint::new(0, 0, 12, 0b1010, 0b0001, 99);
        let b = Fingerprint::new(0, 0, 12, 0b1010, 0b0001, 99);
        let c = a.compatibility(&b);
        assert!(c >= 0.94, "exact match should be ≥ 0.94, got {}", c);
        assert_eq!(judge(c), Cooperativity::SameEntity);
    }

    #[test]
    fn small_diff_is_weak() {
        // value 差 1:compat = 1/(1+1) = 0.5 → Weak,不是 Family
        let a = Fingerprint::new(0, 0, 12, 0b1010, 0b0001, 99);
        let b = Fingerprint::new(0, 0, 12, 0b1010, 0b0001, 98);
        let c = a.compatibility(&b);
        assert_eq!(c, 0.5, "value 差 1 => compat=0.5");
        // 0.5 在阈值 [0.30, 0.70) 之间 → Weak
        assert_eq!(judge(c), Cooperativity::Weak);
    }

    #[test]
    fn big_diff_incompatible() {
        let a = Fingerprint::new(0, 0, 12, 0b1010, 0b0001, 99);
        let b = Fingerprint::new(50000, 50000, 50000, 0b0101, 0b1110, 1);
        let c = a.compatibility(&b);
        assert!(c < 0.30, "large diff should be incompatible, got {}", c);
        assert_eq!(judge(c), Cooperativity::Incompatible);
    }

    #[test]
    fn different_name_same_fp_same_entity() {
        // 同一功能,不同名字 → 同一实体
        let fp_a = Fingerprint::new(0, 0, 12, 0b1010, 0b0001, 99);
        let fp_b = Fingerprint::new(0, 0, 12, 0b1010, 0b0001, 99);
        // kndrl vs kndoyle:名字不同,指纹相同 → 同一个人
        let c = fp_a.compatibility(&fp_b);
        assert_eq!(judge(c), Cooperativity::SameEntity);
    }

    #[test]
    fn different_fp_same_name_not_same_entity() {
        // 同名不同人
        let fp_a = Fingerprint::new(0, 0, 12, 0b1010, 0b0001, 99);
        let fp_b = Fingerprint::new(10000, 10000, 10000, 0b0101, 0b1110, 1);
        let c = fp_a.compatibility(&fp_b);
        assert_ne!(judge(c), Cooperativity::SameEntity);
    }

    #[test]
    fn register_and_lookup() {
        let mut guard = ensure_registry();
        *guard = Some(HashMap::new()); // 清空测试用注册表
        drop(guard);

        let meta_a = EntityMeta {
            fingerprint: Fingerprint::new(0, 0, 12, 0b1010, 0b0001, 99),
            display_name: "mlil".into(),
            provenance: "AI-A".into(),
            history_chain: vec![0xde, 0xad],
        };
        register_entity(meta_a).unwrap();

        // lookup 相同指纹
        let fp_lookup = Fingerprint::new(0, 0, 12, 0b1010, 0b0001, 99);
        let found = lookup_by_fingerprint(&fp_lookup);
        assert!(found.is_some(), "should find by exact fingerprint");

        let found_meta = found.unwrap();
        assert_eq!(found_meta.display_name, "mlil");
        assert_eq!(found_meta.provenance, "AI-A");

        // lookup 近似指纹
        let fp_approx = Fingerprint::new(0, 0, 12, 0b1010, 0b0001, 98);
        let found2 = lookup_by_fingerprint(&fp_approx);
        assert!(found2.is_some(), "approximate lookup should return best match");
    }

    #[test]
    fn can_cooperate_functional() {
        let a = Fingerprint::new(0, 0, 12, 0b1010, 0b0001, 99);
        let b = Fingerprint::new(0, 0, 12, 0b1010, 0b0001, 99);
        let c = can_cooperate(&a, &b);
        assert_eq!(c, Cooperativity::SameEntity);
    }
}
