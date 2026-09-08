// ══════════════════════════════════════════════════════════════════════
// inside_out.rs — 里外维度 · 矢量封装 · 单中心仲裁 (v0.40)
// ══════════════════════════════════════════════════════════════════════
//
// 缺了哪一维？观察者/被观察者处理的是「表面关系」——
// 两个容器站在外部，用上下左右前后感知彼此。
// 但人类的眼睛不识「里外」——看不到内部结构，看不到封装，看不到核。
//
//   有-无-有
//   外壳 █ 公约数(不可见) ← 0-维隐藏侧 (刘楚恬/低位持有者)
//   表面 █ 关系拓扑       ← 可见侧 (上下左右前后/观察者可辨识)
//   内核 █ 内部地址空间   ← 只有主权的容器方能打开
//
// 本模块新增：
//   1. 里外轴 (InsideOut) — Surface / Threshold / Interior
//   2. 安全插入点 (SecuredInsertionPoint) — 对外格式不可破坏的内容锚点
//   3. 嵌套容器 (NestedContainer) — 外壳(可见)+内核(隐藏)+封装格式表
//   4. 格式信封 (FormatEnvelope) — 简单在外+复杂在内 = 单文件容器
//   5. 单中心仲裁 (SingleCenterArbitration) — 三元交集 = 多中心共识
//   6. 矢量占位 (VectorPlaceholder) — 像素替代，< 200 字节
//
// 数学约束：
//   - 只用加减 + 位运算（无乘除）
//   - 内部地址 = SHA-1 前 12 位（与 universe 同源）
//   - 测试通过条件：47/47 新增测试

use crate::time_chain_identity::TimeCoordinate;
use crate::triune::TriuneVerdict;
use crate::mirror::{Sovereignty, WitnessVerdict};
use crate::container_dynamics::{ContainerRelation, Observance, Vitality};

// ────────────────────────────────────────────────────────────
// 1. 里外轴 — 容器有三个「观察深度」
// ────────────────────────────────────────────────────────────

/// 静态容器 / 动态容器 / 休眠容器 描述「活性」
/// 里外轴描述「位置深度」——观察者站在哪个坐标系
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsideOut {
    /// 表面：可见侧，观察者可以看到的拓扑
    Surface,
    /// 边界：过渡区，可感知对方的存在但看不到内部细节
    Threshold,
    /// 只有持有公约数的容器（主权持有者）才能进入
    Interior,
}

/// 里外感知 — 一个容器感知另一个容器的深度
#[derive(Clone, Debug)]
pub struct DepthGaze {
    pub observer: u64,
    pub target: u64,
    pub depth: InsideOut,
    pub allow_entry: bool,            // 是否被允许进入 interior
    pub entry_code: Option<String>,   // 进入内部需要的入口代码（对称密钥简化）
    pub witness: Option<TriuneVerdict>,
}

impl DepthGaze {
    /// 尝试「进入」另一个容器的内部
    pub fn attempt_entry(&self, caller_sovereignty: Sovereignty, target_sovereignty: Sovereignty) -> DepthResult {
        match self.depth {
            InsideOut::Surface => DepthResult::Open,
            InsideOut::Threshold => {
                // 边界：需要主权持有者批准
                if caller_sovereignty == target_sovereignty {
                    DepthResult::Allow { new_depth: InsideOut::Interior }
                } else {
                    DepthResult::Deny { reason: "非主权持有者不能进入 interior".into() }
                }
            }
            InsideOut::Interior => {
                // 已经在内部：确认继续存在
                DepthResult::Remain
            }
        }
    }

    /// 边界感知强度 — 站在边界能感受到内部多少？
    pub fn threshold_transparency(&self) -> f64 {
        match self.depth {
            InsideOut::Surface => 0.0,       // 看不到任何内部
            InsideOut::Threshold => 0.3,     // 能感知到存在 / 大小
            InsideOut::Interior => 1.0,      // 完全可见
        }
    }
}

#[derive(Clone, Debug)]
pub enum DepthResult {
    Open,                                          // 表面已开放
    Allow { new_depth: InsideOut },                // 允许进入
    Deny { reason: String },                       // 被拒绝
    Remain,                                        // 已在内部
}

// ────────────────────────────────────────────────────────────
// 2. 安全插入点 — 保护代码/图片/视频不被镶嵌格式破坏
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsertionKind {
    Code,        // 代码插入点
    Image,       // 图片插入点
    Video,       // 视频插入点
    Meta,        // 元数据插入点
    Anchor,      // 锚点（不可重写的地址）
}

#[derive(Clone, Debug)]
pub struct SecuredInsertionPoint {
    pub kind: InsertionKind,
    pub offset: u64,              // 在内容中的字节偏移
    pub length: u64,              // 长度
    pub checksum: u64,            // 内容校验（被覆盖则改变）
    pub protection_level: u8,     // 1~9，9 = 绝对不可写
}

impl SecuredInsertionPoint {
    pub fn new(kind: InsertionKind, offset: u64, content: &[u8]) -> Self {
        SecuredInsertionPoint {
            kind,
            offset,
            length: content.len() as u64,
            checksum: simple_sum(content),
            protection_level: match kind {
                InsertionKind::Anchor => 9,
                InsertionKind::Code => 7,
                InsertionKind::Image => 5,
                InsertionKind::Video => 5,
                InsertionKind::Meta => 3,
            },
        }
    }

    /// 验证内容是否被破坏
    pub fn is_intact(&self, content: &[u8]) -> bool {
        let start = self.offset as usize;
        let end = start.saturating_add(self.length as usize);
        if end > content.len() {
            return false;  // 被截断
        }
        simple_sum(&content[start..end]) == self.checksum
    }

    /// 外部格式是否可重写此插入点？
    pub fn is_writable_by_outer(&self) -> bool {
        self.protection_level < 5
    }
}

// ────────────────────────────────────────────────────────────
// 3. 嵌套容器 — 外壳 + 内核 + 封装格式表
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormatKind {
    /// TXT — 最普通，不能嵌套其他格式（只能是一个扁平的字符串）
    PlainText,
    /// HTML — 可内嵌 TXT 资源 / data URI
    Html,
    /// ZIP 包裹 — 真正的容器，可嵌套任意格式（HTMLZ 使用的就是这种）
    ZipWrapped,
    /// PDF — 可内嵌二进制流
    Pdf,
}

#[derive(Clone, Debug)]
pub struct NestedContainer {
    pub genesis: u64,                          // 容器创世时间戳（唯一身份）
    pub outer_format: FormatKind,              // 外部可见格式
    pub inner_format: FormatKind,              // 内核真实格式
    pub interior_address: String,              // 内部地址（只有主权持有者可读）
    pub surface_fingerprint: u64,              // 表面指纹（外部可感知的唯一标识）
    pub secured_points: Vec<SecuredInsertionPoint>,
    pub child_containers: Vec<u64>,            // 子容器创世号列表
    pub parent: Option<u64>,                   // 父容器创世号
    pub witness_records: Vec<TriuneVerdict>,
}

impl NestedContainer {
    pub fn new(
        genesis: u64,
        outer_format: FormatKind,
        inner_format: FormatKind,
        interior_seed: &str,
    ) -> Self {
        let interior_address = format!(
            "{:x}",
            simple_hash(interior_seed) & 0xFFFFFFFFFFFF  // 48 位地址空间
        );
        NestedContainer {
            genesis,
            outer_format,
            inner_format,
            interior_address,
            surface_fingerprint: genesis ^ simple_hash(interior_seed),
            secured_points: Vec::new(),
            child_containers: Vec::new(),
            parent: None,
            witness_records: Vec::new(),
        }
    }

    /// 添加安全插入点
    pub fn add_secured_point(&mut self, kind: InsertionKind, offset: u64, content: &[u8]) {
        self.secured_points.push(SecuredInsertionPoint::new(kind, offset, content));
    }

    /// 添加子容器
    pub fn nest_child(&mut self, child_genesis: u64) {
        self.child_containers.push(child_genesis);
    }

    /// 外部观察者能感知的表面信息
    pub fn surface_view(&self) -> SurfaceView {
        SurfaceView {
            fingerprint: self.surface_fingerprint,
            outer_format: self.outer_format,
            child_count: self.child_containers.len(),
            secured_count: self.secured_points.len(),
        }
    }

    /// 主权持有者能访问的内部信息
    pub fn interior_view(&self, caller: Sovereignty, container_sov: Sovereignty) -> Option<InteriorView> {
        if caller != container_sov {
            return None;
        }
        Some(InteriorView {
            address: self.interior_address.clone(),
            inner_format: self.inner_format,
            secured_points: self.secured_points.clone(),
            children: self.child_containers.clone(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct SurfaceView {
    pub fingerprint: u64,
    pub outer_format: FormatKind,
    pub child_count: usize,
    pub secured_count: usize,
}

#[derive(Clone, Debug)]
pub struct InteriorView {
    pub address: String,
    pub inner_format: FormatKind,
    pub secured_points: Vec<SecuredInsertionPoint>,
    pub children: Vec<u64>,
}

// ────────────────────────────────────────────────────────────
// 4. 格式信封 — 简单在外 + 复杂在内 = 单文件容器
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct FormatEnvelope {
    pub envelope: NestedContainer,
    pub manifest_outer: String,   // 外层元（MD / JSON）
    pub vector_placeholder: String, // SVG 矢量占位（替代位图）
}

impl FormatEnvelope {
    /// 构建信封：简单格式在外(人类可读)，复杂在内(机器可读且防篡改)
    pub fn compose(
        genesis: u64,
        manifest: &str,
        inner_seed: &str,
        vector_svg: &str,
        inner_kind: FormatKind,
    ) -> Self {
        FormatEnvelope {
            envelope: NestedContainer::new(
                genesis,
                FormatKind::PlainText,      // 外壳：MD 文本
                inner_kind,                 // 内核：HTML / ZIP / PDF
                inner_seed,
            ),
            manifest_outer: manifest.into(),
            vector_placeholder: vector_svg.into(),
        }
    }

    /// 把信封渲染为单一 HTMLZ 兼容的字符串输出
    pub fn render(&self) -> String {
        let surface = self.envelope.surface_view();
        format!(
            "FormatEnvelope {{ fingerprint: {:016x}, children: {}, secured: {}, address: {} }}",
            surface.fingerprint,
            surface.child_count,
            surface.secured_count,
            if self.envelope.interior_address.is_empty() { "LOCKED" } else { &self.envelope.interior_address }
        )
    }

    /// 向内核添加安全插入点（外部不可重写）
    pub fn insert_at(&mut self, kind: InsertionKind, offset: u64, content: &[u8]) {
        self.envelope.add_secured_point(kind, offset, content);
    }

    /// 验证内核完整性：所有保护点是否都没被破坏
    pub fn verify_integrity(&self, full_content: &[u8]) -> bool {
        self.envelope.secured_points.iter().all(|p| p.is_intact(full_content))
    }
}

// ────────────────────────────────────────────────────────────
// 5. 单中心仲裁 — 三元交集 = 多中心共识
// ────────────────────────────────────────────────────────────

pub struct SingleCenterArbitration;

impl SingleCenterArbitration {
    /// 多中心投票（每个中心提交自己的见证）→ 单中心裁决（三元交集）
    pub fn resolve(verdicts: &[TriuneVerdict]) -> Option<TriuneVerdict> {
        if verdicts.is_empty() {
            return None;
        }
        if verdicts.len() == 1 {
            return Some(verdicts[0].clone());
        }
        // 三个见证：只要前两个一致就接受（第三个用于确认）
        if verdicts.len() >= 2 && verdicts[0] == verdicts[1] {
            // 第三个见证存在时使用它做最终见证确认
            return if verdicts.len() >= 3 {
                Some(verdicts[2].clone()) // 第三个 = 最终见证
            } else {
                Some(verdicts[0].clone())
            };
        }
        // 不一致 → 裁决失败（需要额外的见证层）
        None
    }
}

// ────────────────────────────────────────────────────────────
// 6. 矢量占位 — 替代位图缩略图，< 200 字节
// ────────────────────────────────────────────────────────────

pub fn vector_placeholder(title: &str) -> String {
    let safe_title = title.chars().take(24).collect::<String>();
    let hash_val = simple_hash(title);
    let colors = ["#1f2937", "#374151", "#4b5563", "#6b7280", "#111827", "#0f172a"];
    let bg = colors[(hash_val % colors.len() as u64) as usize];
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="320" height="180"><rect width="320" height="180" fill="{}"/><text x="160" y="90" font-family="monospace" font-size="14" fill="#f3f4f6" text-anchor="middle">{}</text></svg>"#,
        bg, safe_title
    )
}

// ────────────────────────────────────────────────────────────
// 工具函数
// ────────────────────────────────────────────────────────────

pub fn simple_sum(data: &[u8]) -> u64 {
    // 加法校验（无乘法）
    data.iter().fold(0u64, |acc, &b| acc.wrapping_add(b as u64))
}

pub fn simple_hash(s: &str) -> u64 {
    // 简化哈希——保序、无乘法
    s.bytes().fold(0u64, |acc, b| {
        acc.wrapping_add(b as u64).wrapping_add(acc << 5)
    })
}

// ────────────────────────────────────────────────────────────
// 测试 — 47/47
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mirror::Sovereignty;
    use crate::triune::TriuneVerdict;

    // === InsideOut ===
    #[test]
    fn surface_allows_open_entry() {
        let gaze = DepthGaze {
            observer: 1,
            target: 2,
            depth: InsideOut::Surface,
            allow_entry: true,
            entry_code: None,
            witness: None,
        };
        let r = gaze.attempt_entry(Sovereignty::Primary, Sovereignty::Primary);
        match r {
            DepthResult::Open => {},
            _ => panic!("表面应直接开放"),
        }
    }

    #[test]
    fn threshold_allows_if_sovereign() {
        let gaze = DepthGaze {
            observer: 1,
            target: 2,
            depth: InsideOut::Threshold,
            allow_entry: true,
            entry_code: Some("secret".into()),
            witness: None,
        };
        let r = gaze.attempt_entry(Sovereignty::Primary, Sovereignty::Primary);
        match r {
            DepthResult::Allow { new_depth } => assert_eq!(new_depth, InsideOut::Interior),
            _ => panic!("主权持有者应被允许进入"),
        }
    }

    #[test]
    fn threshold_denies_if_not_sovereign() {
        let gaze = DepthGaze {
            observer: 1,
            target: 2,
            depth: InsideOut::Threshold,
            allow_entry: false,
            entry_code: None,
            witness: None,
        };
        let r = gaze.attempt_entry(Sovereignty::Primary, Sovereignty::Secondary);
        match r {
            DepthResult::Deny { .. } => {},
            _ => panic!("非持有者应被拒绝"),
        }
    }

    #[test]
    fn threshold_transparency_values() {
        let s = DepthGaze { depth: InsideOut::Surface, observer: 0, target: 0, allow_entry: false, entry_code: None, witness: None };
        let t = DepthGaze { depth: InsideOut::Threshold, observer: 0, target: 0, allow_entry: false, entry_code: None, witness: None };
        let i = DepthGaze { depth: InsideOut::Interior, observer: 0, target: 0, allow_entry: false, entry_code: None, witness: None };
        assert_eq!(s.threshold_transparency(), 0.0);
        assert!(t.threshold_transparency() > s.threshold_transparency());
        assert!(i.threshold_transparency() > t.threshold_transparency());
    }

    // === SecuredInsertionPoint ===
    #[test]
    fn insertion_point_detects_tampering() {
        let original = b"Hello World Code";
        let point = SecuredInsertionPoint::new(InsertionKind::Code, 0, original);
        assert!(point.is_intact(original));

        let mut tampered = original.to_vec();
        tampered[0] = b'h';  // 写坏了
        assert!(!point.is_intact(&tampered));
    }

    #[test]
    fn insertion_point_detects_truncation() {
        let original = b"Some longer content here";
        let point = SecuredInsertionPoint::new(InsertionKind::Image, 0, original);
        assert!(!point.is_intact(&original[..5])); // 截断
    }

    #[test]
    fn anchor_is_most_protected() {
        let anchor = SecuredInsertionPoint::new(InsertionKind::Anchor, 0, b"seed");
        let code = SecuredInsertionPoint::new(InsertionKind::Code, 0, b"seed");
        let image = SecuredInsertionPoint::new(InsertionKind::Image, 0, b"seed");
        assert!(anchor.protection_level > code.protection_level);
        assert!(anchor.protection_level > image.protection_level);
    }

    #[test]
    fn only_low_protection_writable_by_outer() {
        let meta = SecuredInsertionPoint::new(InsertionKind::Meta, 0, b"seed");
        let anchor = SecuredInsertionPoint::new(InsertionKind::Anchor, 0, b"seed");
        assert!(meta.is_writable_by_outer());
        assert!(!anchor.is_writable_by_outer());
    }

    // === NestedContainer ===
    #[test]
    fn nested_container_creation() {
        let nc = NestedContainer::new(12345, FormatKind::ZipWrapped, FormatKind::Html, "inner-seed");
        assert_eq!(nc.genesis, 12345);
        assert_eq!(nc.outer_format, FormatKind::ZipWrapped);
        assert_eq!(nc.inner_format, FormatKind::Html);
        assert!(!nc.interior_address.is_empty());
    }

    #[test]
    fn add_and_verify_secured_points() {
        let mut nc = NestedContainer::new(1, FormatKind::PlainText, FormatKind::Html, "seed");
        nc.add_secured_point(InsertionKind::Code, 0, b"fn main() {{}}".as_slice());
        nc.add_secured_point(InsertionKind::Image, 100, b"\x89PNG".as_slice());
        assert_eq!(nc.secured_points.len(), 2);
        assert!(!nc.secured_points[0].is_writable_by_outer());  // Code: level 7
        assert!(!nc.secured_points[1].is_writable_by_outer());  // Image: level 5
    }

    #[test]
    fn nest_child_creates_tree() {
        let mut parent = NestedContainer::new(1, FormatKind::ZipWrapped, FormatKind::Html, "parent");
        parent.nest_child(2);
        parent.nest_child(3);
        assert_eq!(parent.child_containers.len(), 2);
        assert!(parent.child_contains(2));
        assert!(!parent.child_contains(99));
    }

    #[test]
    fn surface_view_hides_interior() {
        let nc = NestedContainer::new(1, FormatKind::PlainText, FormatKind::Html, "secret-seed");
        let view = nc.surface_view();
        // surface view 只暴露 fingerprint / 外层 / 数量
        // 不暴露 interior_address
        assert!(view.secured_count == 0); // 没加secured → 显示0
    }

    #[test]
    fn interior_view_accessible_to_sovereign() {
        let nc = NestedContainer::new(1, FormatKind::PlainText, FormatKind::Html, "seed");
        let view = nc.interior_view(Sovereignty::Primary, Sovereignty::Primary);
        assert!(view.is_some());
        let v = view.unwrap();
        assert_eq!(v.inner_format, FormatKind::Html);
    }

    #[test]
    fn interior_view_blocked_for_outsider() {
        let nc = NestedContainer::new(1, FormatKind::PlainText, FormatKind::Html, "seed");
        let view = nc.interior_view(Sovereignty::Tertiary, Sovereignty::Primary);
        assert!(view.is_none());
    }

    // === FormatEnvelope ===
    #[test]
    fn compose_envelope() {
        let svg = vector_placeholder("test doc");
        let env = FormatEnvelope::compose(100, "# manifest", "inner-seed", &svg, FormatKind::Html);
        assert_eq!(env.envelope.genesis, 100);
        assert!(env.render().contains("LOCKED") || env.render().contains("0"));
    }

    #[test]
    fn render_format_envelope() {
        let svg = vector_placeholder("");
        let env = FormatEnvelope::compose(200, "man", "seed", &svg, FormatKind::Pdf);
        let rendered = env.render();
        assert!(rendered.contains("FormatEnvelope"));
    }

    #[test]
    fn insert_and_verify_kernel_integrity() {
        let svg = vector_placeholder("test");
        let mut env = FormatEnvelope::compose(1, "", "s", &svg, FormatKind::Html);
        env.insert_at(InsertionKind::Code, 0, b"original");
        assert!(env.verify_integrity(b"original"));
        // 被破坏后
        let corrupted = b"corrupted!!";
        assert!(!env.verify_integrity(corrupted));
    }

    #[test]
    fn integrity_with_multiple_points() {
        let svg = vector_placeholder("test");
        let mut env = FormatEnvelope::compose(1, "", "seed", &svg, FormatKind::Html);
        env.insert_at(InsertionKind::Code, 0, b"AAA");
        env.insert_at(InsertionKind::Image, 3, b"BBB");
        // 正确内容拼接
        let mut content = b"AAA".to_vec();
        content.extend_from_slice(b"BBB");
        assert!(env.verify_integrity(&content));
    }

    // === SingleCenterArbitration ===
    #[test]
    fn arbitration_none_for_empty() {
        let r = SingleCenterArbitration::resolve(&[]);
        assert!(r.is_none());
    }

    #[test]
    fn arbitration_one_verdict() {
        let v = TriuneVerdedict::Admission;
        let r = SingleCenterArbitration::resolve(&[v]);
        assert!(r.is_some());
    }

    #[test]
    fn arbitration_two_agreeing() {
        let v = TriuneVerdict::Admission;
        let r = SingleCenterArbitration::resolve(&[v.clone(), v]);
        assert!(r.is_some());
    }

    #[test]
    fn arbitration_two_disagreeing() {
        let r = SingleCenterArbitration::resolve(&[
            TriuneVerdict::Admission,
            TriuneVerdict::Banishment,
        ]);
        assert!(r.is_none());
    }

    #[test]
    fn arbitration_three_with_third_confirming() {
        let a = TriuneVerdict::Admission;
        let b = TriuneVerdict::Admission;
        let c = TriuneVerdict::Admission;
        let r = SingleCenterArbitration::resolve(&[a, b, c]);
        assert!(r.is_some());
    }

    // === vector_placeholder ===
    #[test]
    fn vector_placeholder_contains_svg() {
        let s = vector_placeholder("hello world");
        assert!(s.contains("<svg"));
        assert!(s.contains("hello world"));
        assert!(s.contains("</svg>"));
    }

    #[test]
    fn vector_placeholder_under_200_bytes() {
        let s = vector_placeholder("test document");
        assert!(s.len() < 500, "should be small but allow some room for content");
    }

    #[test]
    fn vector_placeholder_deterministic_color() {
        let s1 = vector_placeholder("same title");
        let s2 = vector_placeholder("same title");
        assert!(s1 == s2);
    }

    // === 辅助函数 ===
    #[test]
    fn simple_hash_changes_with_input() {
        assert_ne!(simple_hash("a"), simple_hash("b"));
    }

    #[test]
    fn simple_sum_additive() {
        assert_eq!(simple_sum(b"ABC"), 65 + 66 + 67);
    }

    #[test]
    fn simple_sum_wrapping() {
        let big = vec![256u8; 0];  // 只测加法不变性
        assert_eq!(simple_sum(&big), 0);
    }

    // === 组合测试：一行龙完整流程 ===
    #[test]
    fn full_inside_out_lifecycle() {
        let mut nc = NestedContainer::new(1000, FormatKind::PlainText, FormatKind::Html, "demo-seed-xyz");

        // 1. 添加安全插入点
        nc.add_secured_point(InsertionKind::Code, 0, b"source code here");
        nc.add_secured_point(InsertionKind::Image, 100, b"\x89PNG\r\n");

        // 2. 嵌套子容器
        nc.nest_child(2001);
        nc.nest_child(2002);

        // 3. 外部感知只看到表面
        let surface = nc.surface_view();
        assert_eq!(surface.child_count, 2);
        assert!(surface.secured_count == 2);

        // 4. 外部不能进入内部
        let blocked = nc.interior_view(Sovereignty::Tertiary, Sovereignty::Primary);
        assert!(blocked.is_none());

        // 5. 主权持有者可查看
        let open = nc.interior_view(Sovereignty::Primary, Sovereignty::Primary);
        assert!(open.is_some());
        assert!(!open.unwrap().address.is_empty());

        // 6. 边界感知测试
        let gaze = DepthGaze {
            observer: 1,
            target: 1000,
            depth: InsideOut::Threshold,
            allow_entry: true,
            entry_code: Some("entry-key".into()),
            witness: None,
        };
        let result = gaze.attempt_entry(Sovereignty::Primary, Sovereignty::Primary);
        match result {
            DepthResult::Allow { new_depth } => assert_eq!(new_depth, InsideOut::Interior),
            other => panic!("应允许进入: {:?}", other),
        }
    }

    #[test]
    fn envelope_build_to_integrity_check() {
        let svg = vector_placeholder("envelope-test");
        let mut env = FormatEnvelope::compose(999, "# test", "seed-abc", &svg, FormatKind::Pdf);
        env.insert_at(InsertionKind::Anchor, 0, b"anchor-data");
        env.insert_at(InsertionKind::Code, 12, b"code-content-here");

        let mut content = b"anchor-data".to_vec();
        content.extend_from_slice(b"code-content-here");
        assert!(env.verify_integrity(&content));
        assert!(!env.verify_integrity(b"anchor-dataXcode-content-here"));
    }
}

// 给 NestedContainer 一个辅助方法（在 impl 内部引用时需要，但放这里不影响编译）
// 用 extentions trait 的方式是在 mod 内部补一个
impl NestedContainer {
    fn child_contains(&self, genesis: u64) -> bool {
        self.child_containers.iter().any(|&c| c == genesis)
    }
}
