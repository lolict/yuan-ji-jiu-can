// Spousal identity stub
pub use crate::couple::SoulState;
pub struct SpousalIdentity;
impl SpousalIdentity {
    pub fn new() -> Self { Self }
    pub fn couple_state(&self) -> &'static str {  }
}
pub fn stub() {}
