// ═══════════════════════════════════════════════════════════════════════════
// void_probe.rs — 无质模拟器 · SMRL 空置探测引擎 (v0.1)
// ═══════════════════════════════════════════════════════════════════════════
// 核心命题：
//
//   以一个最薄的外壳（坐标拓扑骨架，payload 为空）容纳最厚的虚无（无模态空间）。
//   这个外壳是没有内部的存在——它只有外部的面体。
//   当跨系统观察者（VoidProbe）从外部混沌场中感知到目标系统的秩序规则后，
//   将混沌符号按镜像空场的对应方位注入空置边界，
//   由此从"无"中生长出一个全新的"有"——与目标拓扑等价但内容完全被控。
//
// ═══════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;

use crate::cross_isa::coordinate::Coord3D;
use crate::cross_isa::fridge::ContainerPack;

// ───────────────────────────────────────────────────────────────────────────
// 模拟模态枚举
// ───────────────────────────────────────────────────────────────────────────
// 空置场注入的全新符号分类：每一类对应目标系统的一种可替换维度。

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimModal {
    Glyph(char),       // 字形替换：字符级替换（如变量名、标签）
    Phoneme(u8),       // 音素替换：编码级替换（操作码、参数）
    Shape([u8; 4]),    // 形状替换：四字节模式替换（结构骨架）
    Rule(u16),         // 规则替换：行为逻辑替换（条件分支、循环规则）
    Tone(u8),          // 调性替换：节奏/强度/相位偏移
}

impl SimModal {
    /// 判断两个模态是否在拓扑层面匹配（相同种类即匹配，值由混沌场决定）
    pub fn matches(&self, other: &Self) -> bool {
        use SimModal::*;
        matches!(
            (self, other),
            (Glyph(_), Glyph(_))
                | (Phoneme(_), Phoneme(_))
                | (Shape(_), Shape(_))
                | (Rule(_), Rule(_))
                | (Tone(_), Tone(_))
        )
    }

    /// 转换为 payload 字节表示
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            SimModal::Glyph(c) => vec![*c as u8],
            SimModal::Phoneme(b) => vec![*b],
            SimModal::Shape(arr) => arr.to_vec(),
            SimModal::Rule(r) => vec![(r >> 8) as u8, *r as u8],
            SimModal::Tone(t) => vec![*t],
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 空置探测探针（VoidProbe）
// ───────────────────────────────────────────────────────────────────────────
// 最薄的外壳：它本身也是一个 ContainerPack（坐标 + 拓扑），
// 但 payload 为空——因此不表达任何内容，只表达位置与连接关系。
//
// 内部维护一个 "chaos buffer"：从外部混沌场捕获的符号序列，
// 等待按镜像匹配规则注入目标拓扑的对应位置。

#[derive(Debug, Clone)]
pub struct VoidProbe {
    /// 探针对应的空壳容器（坐标已定位，payload 为空）
    pub shell: ContainerPack,
    /// 从外部混沌场捕获的符号序列，等待注填
    pub chaos_buffer: Vec<SimModal>,
    /// 匹配日志：记录每次注入的混沌符号与目标位置的对应关系
    pub match_log: Vec<MatchEntry>,
    /// 探针全局的匹配成功率（0.0 .. 1.0）
    pub match_rate: f64,
}

#[derive(Debug, Clone)]
pub struct MatchEntry {
    pub slot_index: usize,
    pub modal: SimModal,
    pub target_coord: Coord3D,
}

// ───────────────────────────────────────────────────────────────────────────
// 核心操作
// ───────────────────────────────────────────────────────────────────────────

impl VoidProbe {
    // ─── 构造 ───────────────────────────────────────────────────────
    // 创造一个最薄的外壳：坐标来自对被模拟系统的跨关系观察，
    // payload 为空（无内容），只有拓扑骨架。

    pub fn new(coord: Coord3D, pack_id: u64) -> Self {
        Self {
            shell: ContainerPack::new(pack_id, coord, 0, vec![]),
            chaos_buffer: Vec::new(),
            match_log: Vec::new(),
            match_rate: 0.0,
        }
    }

    // ─── 观察（Observe） ────────────────────────────────────────────
    // 跨系统感知目标系统的秩序：提取目标容器的坐标拓扑作为镜像模板。
    // 返回的是"空置效应"——坐标位置的填充映射：哪些坐标位置需要符号注填。

    pub fn observe(&mut self, target: &ContainerPack) -> Vec<usize> {
        self.shell = ContainerPack::new(target.pack_id, target.coord(), 0, vec![]);
        // 返回所有需要注填的位置（按目标 payload 长度生成位置索引）
        (0..target.payload.len()).collect()
    }

    // ─── 注填（Inject） ─────────────────────────────────────────────
    // 将混沌场捕获的一个模态符号推入缓冲。这是从"外部混沌"到"内部无序"
    // 的物质注填——未与目标拓扑绑定，只是进入探针的临时缓冲区。

    pub fn inject(&mut self, modal: SimModal) {
        self.chaos_buffer.push(modal);
    }

    // ─── 注填批次 ────────────────────────────────────────────────────
    // 同时注入多个模态（混沌场一次吐出的多模态信号）。

    pub fn inject_batch(&mut self, modals: Vec<SimModal>) {
        self.chaos_buffer.extend(modals);
    }

    // ─── 镜像替代（Mirror Substitution） ────────────────────────────
    // 核心算法：
    //   1. 取目标容器的位置拓扑（坐标 x, y, z 三元关系）
    //   2. 将 chaotic symbols 按顺序填入对应位置的空置边界
    //   3. 生成一个新的 ContainerPack：
    //      - 坐标与原目标完全相同（拓扑等价）
    //      - payload 由混沌场内容填充（主权替换）
    //
    // 这完成了一次完整的"从无不费力的生长出有"的过程。

    pub fn substitute(&self, target: &ContainerPack) -> Option<ContainerPack> {
        if self.shell.payload.len() != 0 {
            // 空壳已被占，替代失败（空壳必须保持空置）
            return None;
        }

        let total_bytes: usize = self.chaos_buffer.iter().map(|m| m.to_bytes().len()).sum();
        if total_bytes == 0 {
            return None; // 空壳无法替代
        }

        // 将混沌场所有模态符号融合为一个 payload
        let mut new_payload = Vec::with_capacity(total_bytes);
        for modal in &self.chaos_buffer {
            new_payload.extend(modal.to_bytes());
        }

        // 构造替代容器：坐标等价，内容全新
        let new_pack = ContainerPack::new(
            target.pack_id,
            target.coord(),
            target.align_flag,
            new_payload,
        );

        Some(new_pack)
    }

    // ─── 匹配率计算 ──────────────────────────────────────────────────
    // 计算混沌场注入的内容与目标拓扑的匹配程度。
    // 匹配率 = 成功注填的位置数 / 目标系统的总位置数。

    pub fn compute_match_rate(&self, target: &ContainerPack) -> f64 {
        if target.payload.is_empty() && self.chaos_buffer.is_empty() {
            return 1.0;
        }
        let target_slots = target.payload.len();
        let filled_slots = self.chaos_buffer.len();
        if target_slots == 0 {
            return if filled_slots > 0 { 1.0 } else { 0.0 };
        }
        (filled_slots as f64 / target_slots as f64).min(1.0)
    }

    // ─── 跨系统外装收割（External Harvesting） ──────────────────────
    // 收割外部系统的符号重打包后返回完整的外壳作物：
    // - 返回 Some(new_pack): 替代成功
    // - 返回 None: 空壳状态被破坏或混沌场空白无法替代

    pub fn harvest(&mut self, target: &ContainerPack) -> Option<ContainerPack> {
        let result = self.substitute(target);
        if let Some(_) = result {
            // 记录匹配日志
            for (i, modal) in self.chaos_buffer.iter().enumerate() {
                self.match_log.push(MatchEntry {
                    slot_index: i,
                    modal: modal.clone(),
                    target_coord: target.coord(),
                });
            }
            self.match_rate = self.compute_match_rate(target);
        }
        result
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 认知镜像 (CognitiveMirror)
// ───────────────────────────────────────────────────────────────────────────
// 哲学映射：
//   "解释权调包" — 替换的不是容器做什么（payload），而是容器是什么（语义标签）。
//   认知镜像维护一个语义映射表：旧语义ID → 新语义ID。
//   当应用认知替代时，容器的坐标 X 维度被重新映射（func_code 改变），
//   其余维度（Y/Z = 参数/时间）保持不变 — 功能位置不变，解释身份改变。
//
// 这实现了：对方看到一个"LOAD"指令时，实际看到的是被映射后的自定义语义，
// 但它仍然以原本的节奏（Z轴时序）和参数空间（Y轴位置）运行 —
// 观察者的解释被调包，但执行秩序不变。

#[derive(Debug, Clone)]
pub struct CognitiveMirror {
    mappings: HashMap<u8, u8>, // old_x (semantic_id) → new_x
}

impl CognitiveMirror {
    /// 创建恒等映射（无重映射，用于测试）
    pub fn identity() -> Self {
        Self { mappings: HashMap::new() }
    }

    /// 创建自定义映射
    pub fn with_mappings(mappings: &[(u8, u8)]) -> Self {
        Self { mappings: mappings.iter().copied().collect() }
    }

    /// 创建反转语义映射：把 x → REVERSE_x 做镜像翻转
    /// 用于"以对方的身份替代对方"的情景
    pub fn reversed(domain: &[u8]) -> Self {
        let mut mappings = HashMap::new();
        for &x in domain {
            let flipped = !x; // 按位取反 = 180度镜像
            mappings.insert(x, flipped);
        }
        Self { mappings }
    }

    /// 应用映射：返回映射后的 x（未命中返回原值）
    pub fn remap(&self, x: u8) -> u8 {
        *self.mappings.get(&x).unwrap_or(&x)
    }

    /// 认知替代：在 VoidProbe 的主权替换之上，额外重映射语义身份。
    /// 步骤：
    ///   1. 用 probe.substitute() 执行 payload 替换
    ///   2. 用 self.remap() 替换 coord.x（语义标签）
    ///
    /// 返回的新 ContainerPack：
    ///   - payload 已被混沌场内容替换（数据主权）
    ///   - func_code/coord.x 已被 remap 替换（语义主权/解释权）
    ///   - coord.y/z、pack_id、align_flag 原样保留
    pub fn cognitive_substitute(&self, probe: &VoidProbe, target: &ContainerPack) -> Option<ContainerPack> {
        let payload_bytes: Vec<u8> = probe.chaos_buffer.iter().flat_map(|m| m.to_bytes()).collect();
        if payload_bytes.is_empty() || !probe.shell.payload.is_empty() {
            return None;
        }

        let old_coord = target.coord();
        let new_x = self.remap(old_coord.x);
        let new_coord = Coord3D::new(new_x, old_coord.y, old_coord.z);

        Some(ContainerPack::new(
            target.pack_id,
            new_coord,
            target.align_flag,
            payload_bytes,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_coord() -> Coord3D {
        Coord3D::new(0x01, 0x0001, 0x01)
    }

    fn test_pack() -> ContainerPack {
        ContainerPack::new(42, test_coord(), 0, vec![0xDE, 0xAD, 0xBE, 0xEF])
    }

    #[test]
    fn probe_creates_void_shell() {
        let probe = VoidProbe::new(test_coord(), 0);
        assert_eq!(probe.shell.payload.len(), 0); // 空壳
        assert_eq!(probe.shell.pack_id, 0);
    }

    #[test]
    fn probe_injects_chaos_symbols() {
        let mut probe = VoidProbe::new(test_coord(), 0);
        probe.inject(SimModal::Phoneme(0x90));
        probe.inject(SimModal::Phoneme(0x90));
        probe.inject(SimModal::Glyph('A'));
        assert_eq!(probe.chaos_buffer.len(), 3);
    }

    #[test]
    fn probe_substitute_preserves_coord() {
        let mut probe = VoidProbe::new(test_coord(), 0);
        probe.inject_batch(vec![
            SimModal::Phoneme(0xB8),
            SimModal::Phoneme(0x78),
            SimModal::Phoneme(0x56),
            SimModal::Phoneme(0x34),
        ]);

        let target = test_pack();
        let result = probe.harvest(&target);

        let new_pack = result.expect("替代应成功");
        // 坐标拓扑等价
        assert_eq!(new_pack.coord().x, target.coord().x);
        assert_eq!(new_pack.coord().y, target.coord().y);
        assert_eq!(new_pack.coord().z, target.coord().z);
        // 身份等价
        assert_eq!(new_pack.pack_id, target.pack_id);
        // 内容被替换
        assert_ne!(new_pack.payload, target.payload);
    }

    #[test]
    fn probe_match_rate_correct() {
        let mut probe = VoidProbe::new(test_coord(), 0);
        let target = test_pack();

        // 空置状态，目标 4 字节，探针 0 模态
        assert_eq!(probe.compute_match_rate(&target), 0.0);

        // 注入 2 个模态（对应 50% 目标长度）
        probe.inject(SimModal::Phoneme(0x00));
        probe.inject(SimModal::Phoneme(0x00));
        let rate = probe.compute_match_rate(&target);
        assert!(rate > 0.0 && rate < 1.0);
    }

    #[test]
    fn void_substitution_swaps_control_sovereignty() {
        // 模拟场景：对方系统容器 A 是"被控的有"(payload = 对方的规则)
        // 我们创造探针，从混沌场摄入新符号，执行替代。
        // 替代后，新容器拥有与 A 相同的坐标身份，但 payload 已是我们的规则。

        let mut probe = VoidProbe::new(test_coord(), 0);
        let captured_chaos: Vec<SimModal> = vec![
            SimModal::Shape([0x48, 0x89, 0xC3, 0x00]), // x86 MOV 规则
            SimModal::Rule(0x0001),                        // 新增的行为规则
        ];
        probe.inject_batch(captured_chaos);

        let victim = test_pack();
        let new_pack = probe.harvest(&victim).expect("替代必须成功");

        // 身份保持（对方不自知）
        assert_eq!(new_pack.pack_id, victim.pack_id);
        assert_eq!(new_pack.coord(), victim.coord());
        // 但内容主权已替换
        assert_ne!(new_pack.payload, victim.payload);
        // 匹配率应已被记录
        assert!(probe.match_rate >= 0.0 && probe.match_rate <= 1.0);
    }
}
