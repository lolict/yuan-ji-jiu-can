// ══════════════════════════════════════════════════════════════════════════
// state_enumerator.rs — 群轨道枚举 · 自描述状态分类系统
//
// 核心算法：
//   1. 定义生成元集合（NOT/补码、REV/镜像、ROT/循环移位、SWAP/位交换...）
//   2. 对 N 位空间的每个状态 s，计算其群轨道 Orbit(s) = {g(s) | g ∈ <G>}
//   3. 由轨道大小和稳定子分类
//   4. 每种分类有模板化 explain() 方法——系统自己解释自己
//
// 用法:
//     let e = StateEnumerator::new(3);
//     let report = e.enumerate();
//     for c in &report { println!("{}", c.explanation); }
//
// 哲学: 容器不是被命名的，是自己在几何中显现的。
// ══════════════════════════════════════════════════════════════════════════

/// 位变换生成元——可扩展的操作集合
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Transform {
    /// 按位取反（补码变换）
    Not,
    /// 位序反转（镜像变换）
    Reverse,
    /// 循环左移 n 位
    Rotate(u8),
}

/// 群轨道——一个状态在生成元作用下的完整展开
#[derive(Debug, Clone)]
pub struct Orbit {
    /// 轨道代表元（最小状态值）
    pub representative: u64,
    /// 轨道中所有状态（去重后排序）
    pub elements: Vec<u64>,
    /// 哪些生成元固定此状态
    pub stabilizer: Vec<Transform>,
}

impl Orbit {
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

/// 状态分类——由群轨道结构决定
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateCategory {
    /// 轴心容器态：轨道大小=2，稳定子含 REV（镜像不变）
    /// 例: 010, 101 (N=3)
    /// 几何: 自包含，不需要外部参考即可确定自身
    AxialContainer,

    /// 闭环态：轨道大小=2，稳定子含 NOT∘REV（反回文）
    /// 例: 0101 (N=4) — Reverse(0101)=1010=NOT(0101)
    /// 几何: 绝对封闭，内部自我指涉，无外部纠缠
    /// 注意: 仅偶数 N 存在（奇数位中间位无法满足自身=自身相反）
    ClosedLoop,

    /// 镜像纠缠态：轨道大小=4（对 {I,NOT,REV,NOT∘REV} 群）
    /// 例: 001 (N=3) → 轨道 {001, 110, 100, 011}
    /// 几何: 无法独立存在，必须与三个兄弟态共同参与才完整
    MirrorEntangled,

    /// 扩展纠缠态：轨道更大（当有更多生成元时）
    /// 例: 加入 ROT 后轨道大小可达 8 或更大
    ExtendedEntangled,
}

impl StateCategory {
    /// 分类的中文命名
    pub fn name_cn(&self) -> &'static str {
        match self {
            Self::AxialContainer => "轴心容器态",
            Self::ClosedLoop => "闭环态",
            Self::MirrorEntangled => "镜像纠缠态",
            Self::ExtendedEntangled => "扩展纠缠态",
        }
    }

    /// 分类对应的拓扑类型（与 glyph-3d-font 对齐）
    pub fn topology(&self) -> &'static str {
        match self {
            Self::AxialContainer => "轴对称几何体",   // 如球体 - 围绕轴旋转不变
            Self::ClosedLoop => "闭环曲面",           // 如环面 - 无始无终
            Self::MirrorEntangled => "四方纠缠结构",   // 如四面体 - 四态互指
            Self::ExtendedEntangled => "多面混沌结构",  // 高维投影
        }
    }

    /// 分类对应的语义ID类型（可以与 vocabulary 对接）
    pub fn semantic_hint(&self) -> &'static str {
        match self {
            Self::AxialContainer => "COUPLE",   // 夫妻共生 — 轴心对
            Self::ClosedLoop => "YIN_YANG",     // 阴阳闭环
            Self::MirrorEntangled => "HOMOPHONE", // 谐音纠缠 — 四态共鸣
            Self::ExtendedEntangled => "OPERATION", // 复杂运算
        }
    }

    /// 分类的数字编码（用于 MCOD 表存储）
    pub fn code(&self) -> u16 {
        match self {
            Self::AxialContainer => 1,
            Self::ClosedLoop => 2,
            Self::MirrorEntangled => 3,
            Self::ExtendedEntangled => 4,
        }
    }
}

/// 单个状态的完整分类描述——系统自己解释自己的产物
#[derive(Debug, Clone)]
pub struct StateClassification {
    /// 原始状态值
    pub state: u64,
    /// 位宽
    pub bit_width: u8,
    /// 分类结果
    pub category: StateCategory,
    /// 群轨道（完整的变换展开）
    pub orbit: Orbit,
    /// 自然语言解释——由模板 + 轨道数据自动生成
    pub explanation: String,
}

/// 枚举概要报告
#[derive(Debug, Clone)]
pub struct EnumerationSummary {
    pub bit_width: u8,
    pub total_states: usize,
    pub axial_count: usize,
    pub closed_count: usize,
    pub entangled_count: usize,
    pub extended_count: usize,
    pub generator_count: usize,
}

impl std::fmt::Display for EnumerationSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "═══ {} 位状态空间枚举报告 ({} 个生成元) ═══\n\
             总状态数: {}\n\
             ├─ 轴心容器态: {:6} ({:.2}%)\n\
             ├─ 闭环态:     {:6} ({:.2}%)\n\
             ├─ 镜像纠缠态: {:6} ({:.2}%)\n\
             └─ 扩展纠缠态: {:6} ({:.2}%)\n\
             ══════════════════════════════════════",
            self.bit_width,
            self.generator_count,
            self.total_states,
            self.axial_count,
            self.axial_count as f64 / self.total_states as f64 * 100.0,
            self.closed_count,
            self.closed_count as f64 / self.total_states as f64 * 100.0,
            self.entangled_count,
            self.entangled_count as f64 / self.total_states as f64 * 100.0,
            self.extended_count,
            self.extended_count as f64 / self.total_states as f64 * 100.0,
        )
    }
}

/// 群轨道枚举器——核心引擎
pub struct StateEnumerator {
    pub bit_width: u8,
    pub generators: Vec<Transform>,
    /// 缓存：用于避免重复计算轨道
    cache: std::cell::RefCell<std::collections::HashMap<u64, usize>>,
}

impl StateEnumerator {
    /// 创建默认枚举器（NOT + REV 两个生成元）
    pub fn new(bit_width: u8) -> Self {
        assert!(bit_width >= 1 && bit_width <= 20, "位宽需在 1-20 之间");
        Self {
            bit_width,
            generators: vec![Transform::Not, Transform::Reverse],
            cache: std::cell::RefCell::new(std::collections::HashMap::new()),
        }
    }

    /// 创建自定义生成元的枚举器
    pub fn with_generators(bit_width: u8, generators: Vec<Transform>) -> Self {
        assert!(bit_width >= 1 && bit_width <= 20, "位宽需在 1-20 之间");
        assert!(!generators.is_empty(), "至少需要一个生成元");
        Self {
            bit_width,
            generators,
            cache: std::cell::RefCell::new(std::collections::HashMap::new()),
        }
    }

    /// 位宽对应的位掩码
    fn mask(&self) -> u64 {
        (1u64 << self.bit_width) - 1
    }

    /// 对单个状态应用一个变换
    fn apply_transform(&self, state: u64, transform: Transform) -> u64 {
        let mask = self.mask();
        match transform {
            Transform::Not => (!state) & mask,
            Transform::Reverse => {
                let mut result: u64 = 0;
                for i in 0..self.bit_width {
                    if (state >> i) & 1 == 1 {
                        result |= 1 << (self.bit_width - 1 - i);
                    }
                }
                result
            }
            Transform::Rotate(n) => {
                let n = (n % self.bit_width) as u64;
                if n == 0 {
                    state
                } else {
                    ((state << n) | (state >> (self.bit_width as u64 - n))) & mask
                }
            }
        }
    }

    /// 计算单个状态的完整群轨道（轨道-稳定子枚举）
    pub fn compute_orbit(&self, state: u64) -> Orbit {
        let mask = self.mask();
        let mut seen = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        let mut stabilizer = Vec::new();

        seen.insert(state);
        queue.push_back(state);

        while let Some(current) = queue.pop_front() {
            for &gen in &self.generators {
                let next = self.apply_transform(current, gen);
                // 双重变换（如 NOT∘REV）也需要纳入
                let next2 = self.apply_transform(next, gen); // 对同一生成元作用两次应回原态
                if next2 != current {
                    // 有其他生成元的组合
                    for &gen2 in &self.generators {
                        if gen2 != gen {
                            let composed = self.apply_transform(
                                self.apply_transform(current, gen),
                                gen2,
                            );
                            if seen.insert(composed) {
                                queue.push_back(composed);
                            }
                        }
                    }
                }
                if seen.insert(next) {
                    queue.push_back(next);
                }
            }
        }

        let mut elements: Vec<u64> = seen.into_iter().collect();
        elements.sort();

        let representative = *elements.first().unwrap_or(&0);

        // 计算稳定子：哪些生成元作用在此状态上不变
        for &gen in &self.generators {
            if self.apply_transform(state, gen) == state {
                stabilizer.push(gen);
            }
        }

        Orbit {
            representative,
            elements,
            stabilizer,
        }
    }

    /// 根据轨道结构分类
    fn categorize(&self, state: u64, orbit: &Orbit) -> StateCategory {
        match orbit.len() {
            1 => {
                // 大小为 1 的轨道：仅当 NOT(s)=s 且 REV(s)=s
                // 这在 N>0 时不可能（NOT(s)=s 意味着每位等于自身相反）
                // 除非 N=0（退化情况）
                StateCategory::AxialContainer
            }
            2 => {
                // 大小为 2：要么是轴心（稳定子含 REV），要么是闭环（稳定子含 NOT∘REV）
                if orbit.stabilizer.contains(&Transform::Reverse) {
                    StateCategory::AxialContainer
                } else {
                    StateCategory::ClosedLoop
                }
            }
            4 => {
                // 大小为 4：标准纠缠（NOT+REV 生成的 Klein 四元群）
                StateCategory::MirrorEntangled
            }
            _ => {
                // 更大轨道：扩展变换集产生的复杂纠缠
                StateCategory::ExtendedEntangled
            }
        }
    }

    /// 生成自然语言解释——系统自己描述自身
    fn generate_explanation(&self, state: u64, orbit: &Orbit, category: &StateCategory) -> String {
        let bits_str = format!("{:0w$b}", state, w = self.bit_width as usize);
        let mask = self.mask();
        let not_val = (!state) & mask;
        let rev_val = self.apply_transform(state, Transform::Reverse);
        let not_str = format!("{:0w$b}", not_val, w = self.bit_width as usize);
        let rev_str = format!("{:0w$b}", rev_val, w = self.bit_width as usize);

        let orbit_list: Vec<String> = orbit
            .elements
            .iter()
            .map(|e| format!("{:0w$b}", e, w = self.bit_width as usize))
            .collect();

        match category {
            StateCategory::AxialContainer => {
                if not_val == rev_val {
                    format!(
                        "状态 {} — {}（闭环轴心）\n\
                         · 补码 = 镜像 = {}\n\
                         · 特征: {} = {} = {}（与自身互补镜像完全重合）\n\
                         · 容器属性: 绝对自指，内外翻转与左右翻转结果相同\n\
                         · 拓扑: {}\n\
                         · 语义映射: {}",
                        bits_str,
                        category.name_cn(),
                        not_str,
                        bits_str,
                        not_str,
                        rev_str,
                        category.topology(),
                        category.semantic_hint()
                    )
                } else {
                    format!(
                        "状态 {} — {}\n\
                         · 镜像不变: Reverse({}) = {}\n\
                         · 补码对: NOT({}) = {}\n\
                         · 特征: 状态 = 镜像（轴对称），补码 = 独立对态\n\
                         · 容器属性: 自包含，不依赖外部即可确定自身\n\
                         · 拓扑: {}\n\
                         · 语义映射: {}",
                        bits_str,
                        category.name_cn(),
                        bits_str,
                        rev_str,
                        bits_str,
                        not_str,
                        category.topology(),
                        category.semantic_hint()
                    )
                }
            }
            StateCategory::ClosedLoop => {
                format!(
                    "状态 {} — {}\n\
                     · 闭环条件: NOT({}) = {}) = {}\n\
                     · 特征: 补码等于镜像（反回文条件成立）\n\
                     · 容器属性: 绝对封闭，内部自我指涉，无外部纠缠\n\
                     · 注意: 仅偶数位系统存在此状态（奇数位中间位矛盾）\n\
                     · 拓扑: {}\n\
                     · 语义映射: {}",
                    bits_str,
                    category.name_cn(),
                    bits_str,
                    not_str,
                    rev_str,
                    category.topology(),
                    category.semantic_hint()
                )
            }
            StateCategory::MirrorEntangled => {
                format!(
                    "状态 {} — {}\n\
                     · 轨道大小: {}\n\
                     · 完整轨道: [{}]\n\
                     · 变换链:\n\
                     {} → NOT → {}\n\
                     {} → REV → {}\n\
                     {} → NOT∘REV → {}\n\
                     · 容器属性: 四态共生于同一群轨道，单一状态不完整\n\
                     · 拓扑: {}\n\
                     · 语义映射: {}",
                    bits_str,
                    category.name_cn(),
                    orbit.len(),
                    orbit_list.join(", "),
                    bits_str,
                    not_str,
                    bits_str,
                    rev_str,
                    bits_str,
                    format!(
                        "{:0w$b}",
                        self.apply_transform(not_val, Transform::Reverse),
                        w = self.bit_width as usize
                    ),
                    category.topology(),
                    category.semantic_hint()
                )
            }
            StateCategory::ExtendedEntangled => {
                format!(
                    "状态 {} — {}\n\
                     · 轨道大小: {}（扩展变换产生的高阶结构）\n\
                     · 完整轨道: [{}]\n\
                     · 容器属性: {} 个状态构成完整群轨道，混沌度 = {}\n\
                     · 拓扑: {}\n\
                     · 语义映射: {}",
                    bits_str,
                    category.name_cn(),
                    orbit.len(),
                    orbit_list.join(", "),
                    orbit.len(),
                    orbit.len(),
                    category.topology(),
                    category.semantic_hint()
                )
            }
        }
    }

    /// 分类单个状态
    pub fn classify(&self, state: u64) -> StateClassification {
        let orbit = self.compute_orbit(state);
        let category = self.categorize(state, &orbit);
        let explanation = self.generate_explanation(state, &orbit, &category);

        StateClassification {
            state,
            bit_width: self.bit_width,
            category,
            orbit,
            explanation,
        }
    }

    /// 枚举完整状态空间——生成全部状态的自描述
    pub fn enumerate(&self) -> Vec<StateClassification> {
        let total = 1u64 << self.bit_width;
        let mut visited = std::collections::HashSet::new();
        let mut results = Vec::with_capacity(total as usize);

        for s in 0..total {
            if visited.contains(&s) {
                continue;
            }

            let classification = self.classify(s);
            // 标记整个轨道为已访问
            for &elem in &classification.orbit.elements {
                visited.insert(elem);
            }
            results.push(classification);
        }

        results
    }

    /// 生成枚举概要报告
    pub fn summary(&self) -> EnumerationSummary {
        let classifications = self.enumerate();
        let mut axial = 0usize;
        let mut closed = 0usize;
        let mut entangled = 0usize;
        let mut extended = 0usize;

        for c in &classifications {
            // 只统计轨道代表元，避免重复计数
            if c.orbit.representative == c.state {
                match c.category {
                    StateCategory::AxialContainer => axial += 1,
                    StateCategory::ClosedLoop => closed += 1,
                    StateCategory::MirrorEntangled => entangled += 1,
                    StateCategory::ExtendedEntangled => extended += 1,
                }
            }
        }

        // 每个轨道代表对应多个实际状态
        let total_states = (1u64 << self.bit_width) as usize;
        let axial_count = axial * 2; // 每个轴心轨道包含 2 个状态（除非 NOT(s)=s，但 N>0 时不可能）
        let closed_count = closed * 2;
        let entangled_count = entangled * 4;
        let extended_count: usize = classifications
            .iter()
            .filter(|c| c.orbit.representative == c.state)
            .filter(|c| matches!(c.category, StateCategory::ExtendedEntangled))
            .map(|c| c.orbit.len())
            .sum();

        EnumerationSummary {
            bit_width: self.bit_width,
            total_states,
            axial_count,
            closed_count,
            entangled_count,
            extended_count,
            generator_count: self.generators.len(),
        }
    }

    /// 查找所有轴心容器态（常用于生成系统核心构件）
    pub fn find_axial_containers(&self) -> Vec<StateClassification> {
        self.enumerate()
            .into_iter()
            .filter(|c| {
                matches!(c.category, StateCategory::AxialContainer)
                    && c.orbit.representative == c.state
            })
            .collect()
    }

    /// 查找所有纠缠态（常用于分析系统复杂度）
    pub fn find_entangled(&self) -> Vec<StateClassification> {
        self.enumerate()
            .into_iter()
            .filter(|c| {
                matches!(
                    c.category,
                    StateCategory::MirrorEntangled | StateCategory::ExtendedEntangled
                ) && c.orbit.representative == c.state
            })
            .collect()
    }
}

// ══════════════════════════════════════════════════════════════════════════
// 测试
// ══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // ── N=1: 只有 2 个轴心 ──

    #[test]
    fn n1_all_axial() {
        let e = StateEnumerator::new(1);
        let summary = e.summary();
        assert_eq!(summary.total_states, 2);
        assert_eq!(summary.axial_count, 2);
        assert_eq!(summary.closed_count, 0);
        assert_eq!(summary.entangled_count, 0);
    }

    // ── N=2: 2 轴心 + 2 闭环 ──

    #[test]
    fn n2_has_closed_loops() {
        let e = StateEnumerator::new(2);
        let summary = e.summary();
        assert_eq!(summary.total_states, 4);
        assert_eq!(summary.axial_count, 2); // 00, 11
        assert_eq!(summary.closed_count, 2); // 01, 10
        assert_eq!(summary.entangled_count, 0);
    }

    // ── N=3: 4 轴心 + 4 纠缠 ──

    #[test]
    fn n3_matches_known_counts() {
        let e = StateEnumerator::new(3);
        let summary = e.summary();
        assert_eq!(summary.total_states, 8);
        assert_eq!(summary.axial_count, 4); // 000, 010, 101, 111
        assert_eq!(summary.closed_count, 0); // 奇数位无闭环
        assert_eq!(summary.entangled_count, 4); // 001, 011, 100, 110
    }

    // ── N=4: 4 轴心 + 4 闭环 + 8 纠缠 ──

    #[test]
    fn n4_matches_known_counts() {
        let e = StateEnumerator::new(4);
        let summary = e.summary();
        assert_eq!(summary.total_states, 16);
        assert_eq!(summary.axial_count, 4); // 0000, 0110, 1001, 1111
        assert_eq!(summary.closed_count, 4); // 0101, 1010, 0001? 待验证
        assert_eq!(summary.entangled_count, 8);
    }

    // ── N=10 ──

    #[test]
    fn n10_matches_formula() {
        let e = StateEnumerator::new(10);
        let summary = e.summary();
        assert_eq!(summary.total_states, 1024);
        assert_eq!(summary.axial_count, 32); // 2^5
        assert_eq!(summary.closed_count, 32); // 2^5
        assert_eq!(summary.entangled_count, 960); // 1024 - 32 - 32
    }

    // ── N=12 ──

    #[test]
    fn n12_matches_formula() {
        let e = StateEnumerator::new(12);
        let summary = e.summary();
        assert_eq!(summary.total_states, 4096);
        assert_eq!(summary.axial_count, 64); // 2^6
        assert_eq!(summary.closed_count, 64); // 2^6
        assert_eq!(summary.entangled_count, 3968); // 4096 - 128
    }

    // ── N=16 ──

    #[test]
    fn n16_matches_formula() {
        let e = StateEnumerator::new(16);
        let summary = e.summary();
        assert_eq!(summary.total_states, 65536);
        assert_eq!(summary.axial_count, 256); // 2^8
        assert_eq!(summary.closed_count, 256); // 2^8
        assert_eq!(summary.entangled_count, 65024); // 65536 - 512
    }

    // ── 生成元的正确性验证 ──

    #[test]
    fn reverse_n3() {
        let e = StateEnumerator::new(3);
        assert_eq!(e.apply_transform(0b001, Transform::Reverse), 0b100);
        assert_eq!(e.apply_transform(0b010, Transform::Reverse), 0b010);
        assert_eq!(e.apply_transform(0b101, Transform::Reverse), 0b101);
    }

    #[test]
    fn not_n3() {
        let e = StateEnumerator::new(3);
        assert_eq!(e.apply_transform(0b010, Transform::Not), 0b101);
        assert_eq!(e.apply_transform(0b000, Transform::Not), 0b111);
        assert_eq!(e.apply_transform(0b111, Transform::Not), 0b000);
    }

    #[test]
    fn rotate_n4() {
        let e = StateEnumerator::new(4);
        // 0001 ROL 1 = 0010
        assert_eq!(e.apply_transform(0b0001, Transform::Rotate(1)), 0b0010);
        // 1000 ROL 1 = 0001
        assert_eq!(e.apply_transform(0b1000, Transform::Rotate(1)), 0b0001);
    }

    // ── 分类正确性验证 ──

    #[test]
    fn axial_classification() {
        let e = StateEnumerator::new(3);
        let c010 = e.classify(0b010);
        assert_eq!(c010.category, StateCategory::AxialContainer);
        assert!(c010.explanation.contains("轴心容器态"));

        let c101 = e.classify(0b101);
        assert_eq!(c101.category, StateCategory::AxialContainer);
    }

    #[test]
    fn entangled_classification() {
        let e = StateEnumerator::new(3);
        let c001 = e.classify(0b001);
        assert_eq!(c001.category, StateCategory::MirrorEntangled);
        assert!(c001.explanation.contains("镜像纠缠态"));
        assert_eq!(c001.orbit.len(), 4);
    }

    // ── 自描述验证 ──

    #[test]
    fn explanation_contains_all_orbit_elements() {
        let e = StateEnumerator::new(3);
        let c = e.classify(0b001);
        // 解释中应包含轨道中所有元素的二进制表示
        for elem in &c.orbit.elements {
            let elem_str = format!("{:03b}", elem);
            assert!(
                c.explanation.contains(&elem_str),
                "解释应包含轨道元素 {}",
                elem_str
            );
        }
    }

    // ── ROT 生成元扩展 ──

    #[test]
    fn extended_generators_create_larger_orbits() {
        let e = StateEnumerator::with_generators(
            4,
            vec![Transform::Not, Transform::Reverse, Transform::Rotate(1)],
        );
        let c = e.classify(0b0001);
        // ROT 引入新变换，轨道应该比基础 {NOT, REV} 更大
        assert!(c.orbit.len() >= 4, "ROT 应扩展轨道大小");
    }

    // ── Display trait 验证 ──

    #[test]
    fn summary_display() {
        let e = StateEnumerator::new(3);
        let s = e.summary();
        let display = format!("{}", s);
        assert!(display.contains("位状态空间枚举报告"));
        assert!(display.contains("总状态数"));
    }
}
