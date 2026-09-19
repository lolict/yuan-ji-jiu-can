// ────────────────────────────────────────────────────────────
// 范畴流 — 跨家族流转
// ────────────────────────────────────────────────────────────

pub struct 范畴流;

impl 范畴流 {
    pub fn 跨族<'a, const A: usize, const B: usize>(
        源身份: &str,
        源家族: &'a 身份家族<A>,
        目标家族: &'a 身份家族<B>,
    ) -> Option<&'a 身份条目> {
        let src = 源家族.取(源身份)?;
        let mut best: Option<&身份条目> = None;
        let mut i = 0;
        while i < B {
            let m = &目标家族.成员[i];
            if best.is_none() {
                best = Some(m);
            } else {
                let best_off = best.unwrap().偏移;
                let d_best = if best_off > src.偏移 { best_off - src.偏移 } else { src.偏移 - best_off };
                let d_m = if m.偏移 > src.偏移 { m.偏移 - src.偏移 } else { src.偏移 - m.偏移 };
                if d_m < d_best { best = Some(m); }
            }
            i += 1;
        }
        best
    }

    /// 对齐值
    pub fn 对齐<const A: usize, const B: usize>(
        源身份: &str,
        源家族: &身份家族<A>,
        目标身份: &str,
        目标家族: &身份家族<B>,
    ) -> Option<(u32, u32, u32)> {
        let src = 源家族.取(源身份)?;
        let tgt = 目标家族.取(目标身份)?;
        let diff = if src.偏移 > tgt.偏移 { src.偏移 - tgt.偏移 } else { tgt.偏移 - src.偏移 };
        Some((src.偏移, tgt.偏移, diff))
    }
}