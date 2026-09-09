// ══════════════════════════════════════════════════════════════════
// 鼓膜自举操作系统 · Self-Bootstrap OS
// ══════════════════════════════════════════════════════════════════
//
// 容器用自己的功能 · 生产出自己的操作系统。
// 三条生产线同时开工:
//   1. 生产器:容器 → 产生新容器(膨胀生产)
//   2. 验证器:每次生产后 · 自我检验是否符合规则
//   3. 翻译器:容器内部符号 ↔ 人类可懂的时间/数值
//
// 鼓膜自举 = 不需要外部操作系统供电
// 容器自己生长出自己需要的规则 · 自己验证 · 自己呈现
//
// ══════════════════════════════════════════════════════════════════

use crate::container_radix::HexCoord;
use crate::couple::SoulState;

// ─── 容器状态(自带坐标·极性·容量·内容)──────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Container {
    pub code: u8,          // 容器码 = 距离码 · 6 位
    pub polarity: u8,      // 夫妻码 · 2 位: 00 独立 · 01 夫 · 10 妻 · 11 合
    pub capacity: u8,      // 当前内容量 · 0-255
    pub max_capacity: u8,  // 容器本身的最大容量 · 由极性决定
}

impl Container {
    /// 从零生产一个新容器 · 鼓膜自举的起点
    /// 容量由容器自身几何决定 · 不需要外部数字:
    ///   妻容器: 六边形周长 36 · 层 6 · 36×6 = 216
    ///   夫容器: 七方格 49 · 层 7 · 49×7 = 343
    ///   独立杯形容器: 26 边 · 层 64 · 26×64 = 1664
    ///   夫妻容器: 妻+夫+杯的容量总和 = 2223
    pub fn birth(code: u8) -> Self {
        let polarity = ((code >> 5) << 1) | ((code >> 4) & 1);
        let max_capacity = match polarity {
            0b10 => 36 * 6,                   // 妻 · 六边形周长 36 · 层 6 · 36×6 = 216
            0b01 => 49 * 7,                   // 夫 · 七方格 49 · 层 7 · 49×7 = 343
            0b00 => 26 * 64,                  // 杯形 · 26 边 · 层 64 · 26×64 = 1664
            0b11 => 36 * 6 + 49 * 7 + 26 * 64, // 夫妻 · 妻+夫+杯总和 = 2223
            _ => 0,
        };
        Container {
            code,
            polarity,
            capacity: 0,
            max_capacity,
        }
    }

    /// 容器是否已满 · 满则需裁决是否能溢出
    pub fn is_full(&self) -> bool {
        self.capacity >= self.max_capacity
    }

    /// 容器外壳边数 · 六边(妻)七边(夫)
    pub fn edge_count(&self) -> u8 {
        match self.polarity {
            0b10 | 0b00 => 6,
            _           => 7,
        }
    }

    /// 尝试注入内容 · 返回实际注入量
    pub fn inject(&mut self, amount: u8) -> u8 {
        let room = self.max_capacity - self.capacity;
        let accepted = if amount > room { room } else { amount };
        self.capacity += accepted;
        accepted
    }
}

// ─── 裁决引擎(生产关系的核心)──────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    Allow(u8),  // 允许 · 并指定可传递量
    Deny,       // 禁止传递
    NeedReview, // 需要三元裁决
}

#[derive(Debug, Clone)]
pub struct Verdict {
    pub decision: Decision,
    pub reason: String,
    pub verified: bool,  // 此裁决已被验证过 · 不需要再仲裁
}

impl Verdict {
    pub fn allow(amount: u8, reason: &str) -> Self {
        Verdict { decision: Decision::Allow(amount), reason: reason.into(), verified: true }
    }
    pub fn deny(reason: &str) -> Self {
        Verdict { decision: Decision::Deny, reason: reason.into(), verified: true }
    }
    pub fn review(reason: &str) -> Self {
        Verdict { decision: Decision::NeedReview, reason: reason.into(), verified: false }
    }
}

/// 相容裁决:判定两个容器之间是否允许内容传递
pub fn arbitrate(from: &Container, to: &Container) -> Verdict {
    let pair = pair_compat(from.polarity, to.polarity);
    let space = to.max_capacity.saturating_sub(to.capacity);
    let content = from.capacity;

    match pair {
        PairCompat::Incompatible => Verdict::deny(&format!(
            "容器{}(极性{:02b})与容器{}(极性{:02b})不相容",
            from.code, from.polarity, to.code, to.polarity
        )),
        PairCompat::Compatible => {
            let amount = content.min(space);
            if amount == 0 {
                Verdict::deny("传输量为零")
            } else {
                Verdict::allow(amount, &format!(
                    "可传输{}单位 · 容器{}余{} · 容器{}余{}",
                    amount, from.code, from.capacity - amount, to.code, to.capacity + amount
                ))
            }
        }
        PairCompat::Conditional => Verdict::review("条件相容 · 需运行三元裁决"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PairCompat {
    Compatible,
    Incompatible,
    Conditional,
}

fn pair_compat(a: u8, b: u8) -> PairCompat {
    match (a, b) {
        (0b00, _) | (_, 0b00) => PairCompat::Compatible,
        (0b10, 0b10) | (0b01, 0b01) | (0b11, 0b11) => PairCompat::Compatible,
        (0b10, 0b01) | (0b01, 0b10) => PairCompat::Incompatible,
        (0b11, 0b10) | (0b11, 0b01)
        | (0b10, 0b11) | (0b01, 0b11) => PairCompat::Conditional,
        _ => PairCompat::Conditional,
    }
}

// ─── 翻译器(容器内部符号 → 可懂)─────────────────────────────

pub fn translate(value: u8, unit: &str) -> String {
    match unit {
        "time"     => format!("{} 拍节", value),
        "distance" => format!("{} 网格", value),
        "amount"   => format!("{} 单位容量", value),
        "count"    => format!("{} 个", value),
        _          => format!("{} (内部值)", value),
    }
}

/// 容器对自己的状态进行自我描述 · 不需要外部语言
pub fn self_describe(c: &Container) -> String {
    let polarity_name = match c.polarity {
        0b00 => "独立(00)",
        0b01 => "丈夫(01)",
        0b10 => "妻子(10)",
        0b11 => "夫妻(11)",
        _ => "未知",
    };
    let coord = HexCoord::from_container_code(c.code);
    format!(
        "容器{} · {} · 容量 {}/{} · 坐标({},{})",
        c.code, polarity_name, c.capacity, c.max_capacity, coord.x, coord.y,
    )
}

// ─── 三元裁决(当常规裁决进入 NeedReview)───────────────

pub fn triune_arbitrate(primary: &Container, w1: &Container, w2: &Container) -> Verdict {
    let v1 = arbitrate(w1, primary);
    let v2 = arbitrate(w2, primary);
    match (v1.decision, v2.decision) {
        (Decision::Allow(_), Decision::Allow(_)) => {
            Verdict::allow(1, "两方均同意 · 通过")
        }
        (Decision::Deny, _) | (_, Decision::Deny) => {
            Verdict::deny("至少一方反对 · 驳回")
        }
        _ => Verdict::deny("未达成多数共识 · 暂禁"),
    }
}

// ─── 操作系统内核(自我生长的生产调度)──────────────────────────

pub struct ContainerOS {
    pub containers: Vec<Container>,
    pub cycle_count: u64,
    pub event_log: Vec<String>,
}

impl ContainerOS {
    pub fn new() -> Self {
        let containers = (0u8..64).map(Container::birth).collect();
        ContainerOS { containers, cycle_count: 0, event_log: Vec::new() }
    }

    pub fn inject_to(&mut self, code: u8, amount: u8) -> u8 {
        let n = code as usize;
        if n >= self.containers.len() { return 0; }
        let accepted = self.containers[n].inject(amount);
        self.event_log.push(format!("注入{}/{}到容器{}", accepted, amount, code));
        accepted
    }

    pub fn try_pour(&mut self, from: u8, to: u8) -> Verdict {
        let (a, b) = (from as usize, to as usize);
        if a >= self.containers.len() || b >= self.containers.len() {
            return Verdict::deny("容器码越界");
        }
        let v = arbitrate(&self.containers[a], &self.containers[b]);
        if let Decision::Allow(amount) = v.decision {
            let actual = self.containers[b].inject(amount);
            self.containers[a].capacity -= actual;
            let reason = format!("执行传递 {}: 容器{}→容器{}", actual, from, to);
            self.event_log.push(reason);
        } else {
            self.event_log.push(format!("裁决驳回 {}→{}: {}", from, to, v.reason));
        }
        v
    }

    /// 一次生产周期 · 所有容器同时判断自己的去向
    pub fn tick(&mut self) -> Vec<String> {
        self.cycle_count += 1;
        let mut results = Vec::new();
        for i in 0..self.containers.len() {
            let coord = HexCoord::from_container_code(i as u8);
            let nbrs = coord.neighbors();
            for &ncode in nbrs.iter() {
                if ncode < 64 {
                    let v = arbitrate(&self.containers[i], &self.containers[ncode as usize]);
                    if let Decision::Allow(amount) = v.decision {
                        let actual = self.containers[ncode as usize].inject(amount);
                        self.containers[i].capacity = self.containers[i].capacity.saturating_sub(actual);
                        results.push(format!("周期{}: 容器{}→容器{} 传递{}单位",
                            self.cycle_count, i, ncode, actual));
                    }
                }
            }
        }
        for r in &results {
            self.event_log.push(r.clone());
        }
        if self.event_log.len() > 256 {
            self.event_log.drain(0..self.event_log.len()-256);
        }
        results
    }

    pub fn summary(&self) -> String {
        let mut counts = [0u32; 4];
        let mut total_cap = 0u32;
        let mut total_max = 0u32;
        for c in &self.containers {
            counts[c.polarity as usize] += 1;
            total_cap += c.capacity as u32;
            total_max += c.max_capacity as u32;
        }
        format!(
            "周期{} · {}个容器 · 独立 {} · 妻 {} · 夫 {} · 夫妻 {} · 总容量 {}/{}",
            self.cycle_count, self.containers.len(),
            counts[0], counts[1], counts[2], counts[3],
            total_cap, total_max,
        )
    }

    pub fn log_tail(&self, n: usize) -> &[String] {
        let start = self.event_log.len().saturating_sub(n);
        &self.event_log[start..]
    }
}

// ─── 测试 ──────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_birth_independent() {
        let c = Container::birth(0);
        assert_eq!(c.polarity, 0b00);
        assert_eq!(c.max_capacity, 1664);
        assert_eq!(c.edge_count(), 6);
    }

    #[test]
    fn container_birth_couple() {
        let c = Container::birth(0b11_1111);
        assert_eq!(c.polarity, 0b11);
        assert_eq!(c.max_capacity, 2223);
        assert_eq!(c.edge_count(), 7);
    }

    #[test]
    fn container_birth_wife() {
        let c = Container::birth(0b10_1010);
        assert_eq!(c.polarity, 0b10);
        assert_eq!(c.max_capacity, 216);
        assert_eq!(c.edge_count(), 6);
    }

    #[test]
    fn container_birth_husband() {
        let c = Container::birth(0b01_0101);
        assert_eq!(c.polarity, 0b01);
        assert_eq!(c.max_capacity, 343);
        assert_eq!(c.edge_count(), 7);
    }

    #[test]
    fn container_wife_inject_no_exceed() {
        let mut c = Container::birth(0b10_0000);
        let accepted = c.inject(20);
        assert_eq!(accepted, 20);
        assert_eq!(c.capacity, 20);
    }

    #[test]
    fn container_inject_overflow_caps() {
        let mut c = Container::birth(0b10_0000);
        let accepted = c.inject(300);
        assert_eq!(accepted, 216);
        assert!(c.is_full());
    }

    #[test]
    fn arbitrate_wife_to_husband_incompatible() {
        let wife = Container::birth(0b10_0000);
        let husband = Container::birth(0b01_0000);
        let v = arbitrate(&wife, &husband);
        assert_eq!(v.decision, Decision::Deny);
        assert!(v.verified);
    }

    #[test]
    fn arbitrate_neutral_to_any_compatible() {
        let neutral = Container::birth(0b00_0000);
        let wife = Container::birth(0b10_0000);
        let v = arbitrate(&neutral, &wife);
        assert!(matches!(v.decision, Decision::Allow(_)));
    }

    #[test]
    fn triune_arbitrate_unanimous_pass() {
        let primary = Container::birth(0b11_0000);
        let w1 = Container::birth(0b00_0001);
        let w2 = Container::birth(0b00_0010);
        let v = triune_arbitrate(&primary, &w1, &w2);
        assert_eq!(v.decision, Decision::Allow(1));
    }

    #[test]
    fn translate_time_unit() {
        assert_eq!(translate(42, "time"), "42 拍节");
        assert_eq!(translate(7, "distance"), "7 网格");
    }

    #[test]
    fn self_describe_includes_all() {
        let c = Container::birth(0b11_0001);
        let desc = self_describe(&c);
        assert!(desc.contains("容器49"));
        assert!(desc.contains("夫妻(11)"));
        assert!(desc.contains("坐标"));
    }

    #[test]
    fn os_new_has_64_containers() {
        let os = ContainerOS::new();
        assert_eq!(os.containers.len(), 64);
    }

    #[test]
    fn os_inject_increases_capacity() {
        let mut os = ContainerOS::new();
        let accepted = os.inject_to(0, 30);
        assert_eq!(accepted, 30);
        assert_eq!(os.containers[0].capacity, 30);
    }

    #[test]
    fn os_pour_denied_when_incompatible() {
        let mut os = ContainerOS::new();
        os.inject_to(0b10_0000, 10);
        let v = os.try_pour(0b10_0000, 0b01_0000);
        assert_eq!(v.decision, Decision::Deny);
    }

    #[test]
    fn os_tick_advances_cycle() {
        let mut os = ContainerOS::new();
        os.tick();
        assert_eq!(os.cycle_count, 1);
    }

    #[test]
    fn os_summary_format() {
        let os = ContainerOS::new();
        let s = os.summary();
        assert!(s.contains("64个容器"));
        assert!(s.contains("周期0"));
    }
}
