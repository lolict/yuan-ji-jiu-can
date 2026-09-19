// ────────────────────────────────────────────────────────────
// 集合
// ────────────────────────────────────────────────────────────

pub struct 集合;

impl 集合 {
    pub fn 交<'a>(a: &[&'a 身份条目], b: &[&'a 身份条目]) -> Vec<&'a 身份条目> {
        a.iter().filter(|x| b.iter().any(|y| y.名字 == x.名字)).copied().collect()
    }

    pub fn 并<'a>(a: &[&'a 身份条目], b: &[&'a 身份条目]) -> Vec<&'a 身份条目> {
        let mut result = a.to_vec();
        for x in b {
            if !result.iter().any(|r| r.名字 == x.名字) {
                result.push(x);
            }
        }
        result
    }
}