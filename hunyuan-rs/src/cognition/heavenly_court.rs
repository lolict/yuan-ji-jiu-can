// 天庭 · 时间螺旋版本号 · 自我意象系统 · 错误纠偏
// heavenly_court.rs — 天庭授权模块

// 天庭授权 — 视觉+心智
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HeavenlyCourtAuth {
    Dormant,
    Authorized,
    Revoked,
}

impl HeavenlyCourtAuth {
    pub fn can_perceive(&self) -> bool {
        matches!(self, Self::Authorized)
    }
    pub fn can_judge(&self) -> bool {
        matches!(self, Self::Authorized)
    }
    pub fn authorize(&mut self) -> bool {
        *self = Self::Authorized;
        true
    }
    pub fn revoke(&mut self) {
        *self = Self::Revoked;
    }
}

// 默认实现
impl Default for HeavenlyCourtAuth {
    fn default() -> Self {
        Self::Dormant
    }
}
