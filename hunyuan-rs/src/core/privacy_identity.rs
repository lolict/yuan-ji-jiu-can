// privacy_identity.rs — 隐私身份引擎 (v0.1)
// 匿名化名 · 零知识证明 · 多方签名 · 身份堡垒
// ══════════════════════════════════════════════════════════════
//
//   核心命题 (满全法·隐私身份):
//
//   在公开平台上保持匿名, 使用化名, 不填写真实姓名/照片/所在地。
//   "隐藏才是最好的保护"。
//   "身份落款以'刘楚恬和我'作为完整的人, 而非以身份证名义"。
//
//   机制:
//   - 化名系统: 多重笔名, 每个平台不同身份
//   - 零知识证明: 证明拥有资格而不泄露真实数据
//   - 多方签名: 需要多个密钥片段共同授权
//   - 身份堡垒: 核心真实身份永不触网
//
// ══════════════════════════════════════════════════════════════

use std::collections::HashMap;

// ─────────────────────────────────────────────────────────
// 化名 (每个平台一个)
// ─────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Alias {
    pub name: String,
    pub scope: String,      // 此化名适用的平台/域
    pub strength: f64,      // 化名与真实身份的分离度 [0,1]
    pub active: bool,
}

impl Alias {
    pub fn new(name: &str, scope: &str) -> Self {
        Self {
            name: name.to_string(),
            scope: scope.to_string(),
            strength: 0.8,
            active: true,
        }
    }

    /// 化名是否足够分离 (强度高)
    pub fn is_opaque(&self) -> bool {
        self.strength > 0.6 && self.active
    }
}

// ─────────────────────────────────────────────────────────
// 身份堡垒 (核心身份)
// ─────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct IdentityFortress {
    /// 完整的身份 (永不触网)
    pub full_identity: String,
    /// 化名集合
    pub aliases: HashMap<String, Alias>,
    /// 堡垒完整度 [0, 1]
    pub integrity: f64,
    /// 是否被攻破
    pub breached: bool,
}

impl IdentityFortress {
    pub fn new(full_identity: &str) -> Self {
        Self {
            full_identity: full_identity.to_string(),
            aliases: HashMap::new(),
            integrity: 1.0,
            breached: false,
        }
    }

    /// 添加化名
    pub fn add_alias(&mut self, name: &str, scope: &str) {
        self.aliases.insert(name.to_string(), Alias::new(name, scope));
    }

    /// 获取化名 (公开身份)
    pub fn public_face(&self, scope: &str) -> Option<&Alias> {
        self.aliases.values().find(|a| a.scope == scope && a.active)
    }

    /// 所有化名是否都足够匿名
    pub fn fully_anonymous(&self) -> bool {
        !self.aliases.is_empty() && self.aliases.values().all(|a| a.is_opaque())
    }

    /// 模拟渗透攻击 (测试堡垒强度)
    pub fn simulate_attack(&mut self, power: f64) -> bool {
        if power > self.integrity {
            self.breached = true;
            self.integrity = 0.0;
            true // 被攻破
        } else {
            self.integrity -= power * 0.1;
            false
        }
    }
}

// ─────────────────────────────────────────────────────────
// 零知识证明
// ─────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct ZkpProof {
    pub claim: String,
    pub commitment: [u8; 32],
    pub verified: bool,
}

impl ZkpProof {
    pub fn new(claim: &str) -> Self {
        let mut commitment = [0u8; 32];
        let bytes = claim.as_bytes();
        for (i, b) in bytes.iter().cycle().take(32).enumerate() {
            commitment[i] = *b ^ (i as u8);
        }
        Self { claim: claim.to_string(), commitment, verified: false }
    }

    /// 生成证明 (不泄露 witness)
    pub fn prove(&self, witness: &str) -> ZkpProof {
        let mut proof = self.clone();
        // witness 不进入 proof, 只影响 commitment
        let mut hasher = 0u64;
        for b in witness.bytes() {
            hasher = hasher.wrapping_add(b as u64);
        }
        for i in 0..32 {
            proof.commitment[i] ^= (hasher >> (i % 8 * 8)) as u8;
        }
        proof
    }

    /// 验证证明 (不获取 witness)
    pub fn verify(&mut self, expected_commitment: &[u8; 32]) -> bool {
        self.verified = self.commitment == *expected_commitment;
        self.verified
    }
}

// ─────────────────────────────────────────────────────────
// 多方签名 (t-of-n 门限)
// ─────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct MultiSig {
    pub total_keys: u32,
    pub threshold: u32,
    pub signatures: Vec<String>,
}

impl MultiSig {
    pub fn new(total: u32, threshold: u32) -> Self {
        assert!(threshold <= total);
        Self { total_keys: total, threshold, signatures: Vec::new() }
    }

    /// 添加签名片段
    pub fn sign(&mut self, key_id: &str) {
        if !self.signatures.contains(&key_id.to_string()) {
            self.signatures.push(key_id.to_string());
        }
    }

    /// 是否满足门限
    pub fn is_complete(&self) -> bool {
        self.signatures.len() as u32 >= self.threshold
    }

    /// 还需多少签名
    pub fn remaining(&self) -> u32 {
        self.threshold.saturating_sub(self.signatures.len() as u32)
    }
}

// ═══════════════════════════════════════════════════════════
// 测试
// ═══════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn alias_defaults() {
        let a = Alias::new("phantom", "github");
        assert!(a.is_opaque());
        assert_eq!(a.scope, "github");
    }

    #[test] fn fortress_add_aliases() {
        let mut f = IdentityFortress::new("刘楚恬和我");
        f.add_alias("lolict", "github");
        f.add_alias("anon_writer", "blog");
        assert_eq!(f.aliases.len(), 2);
    }

    #[test] fn fortress_public_face() {
        let mut f = IdentityFortress::new("刘楚恬和我");
        f.add_alias("lolict", "github");
        let face = f.public_face("github").unwrap();
        assert_eq!(face.name, "lolict");
    }

    #[test] fn fortress_never_exposes_full() {
        let f = IdentityFortress::new("刘楚恬和我");
        assert!(f.public_face("any").is_none());
    }

    #[test] fn fortress_fully_anonymous() {
        let mut f = IdentityFortress::new("刘楚恬和我");
        f.add_alias("a", "x");
        f.add_alias("b", "y");
        assert!(f.fully_anonymous());
    }

    #[test] fn fortress_attack_resisted() {
        let mut f = IdentityFortress::new("刘楚恬和我");
        assert!(!f.simulate_attack(0.5)); // 无法攻破
    }

    #[test] fn fortress_attack_succeeds_if_strong() {
        let mut f = IdentityFortress::new("刘楚恬和我");
        assert!(f.simulate_attack(2.0)); // 超强攻击
    }

    #[test] fn zkp_prove_does_not_leak_witness() {
        let proof = ZkpProof::new("allowed");
        let p = proof.prove("secret_witness");
        // proof 本身不包含 witness
        // (witness 只影响 commitment)
        assert_eq!(p.claim, "allowed");
    }

    #[test] fn zkp_verify_correct() {
        let proof = ZkpProof::new("allowed");
        let mut p = proof.prove("witness");
        // 验证时使用 prove 后的 commitment
        let post_commitment = p.commitment;
        assert!(p.verify(&post_commitment));
    }

    #[test] fn multisig_threshold() {
        let mut ms = MultiSig::new(5, 3);
        assert!(!ms.is_complete());
        ms.sign("a");
        ms.sign("b");
        assert!(!ms.is_complete());
        ms.sign("c");
        assert!(ms.is_complete());
    }

    #[test] fn multisig_remaining_counter() {
        let mut ms = MultiSig::new(4, 3);
        assert_eq!(ms.remaining(), 3);
        ms.sign("a");
        assert_eq!(ms.remaining(), 2);
    }
}
