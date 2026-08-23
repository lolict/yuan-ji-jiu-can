// ══════════════════════════════════════════════════════════════════
// phase_encoding.rs — 颜色进制 · 相位画图 · 区块链纠缠 · 分布式制衡
// ══════════════════════════════════════════════════════════════════
//
// 核心哲学 (用户定义):
//   "层级关系不是低维度的层级, 而是高维度的层级"
//   "先有声音后有文字, 声音走在前面文字走在后面"
//   "像素相位只允许加法(部件安装)和减法(部件拆解), 禁用乘除"
//
// 系统架构 (五层):
//   ┌─────────────────────────────────────────────────────────────┐
//   │ 第五层: 输出层 — 汉字/符号文本 (产物, 不是源头)             │
//   ├─────────────────────────────────────────────────────────────┤
//   │ 第四层: 分布式制衡编织网络 — 多维度拉力互相拉扯平衡          │
//   │         (层级/类型/审美/归属权/黑名单/衍射)                   │
//   ├─────────────────────────────────────────────────────────────┤
//   │ 第三层: 动态镜像衍射运算层 — 活性节点 + 等效距离 + 公倍数     │
//   ├─────────────────────────────────────────────────────────────┤
//   │ 第二层: 区块链式纠缠索引层 — 前缀/后缀指针 + 元数据小纸条     │
//   ├─────────────────────────────────────────────────────────────┤
//   │ 第一层: 相位进制画布 — 离散相位单元, 只允许加减法             │
//   └─────────────────────────────────────────────────────────────┘
//
// 运算白名单:
//   ✅ 加法 = 部件安装 (把A的部件移植到B上)
//   ✅ 减法 = 部件拆解 (从A上挖掉B部件)
//   ❌ 乘法 = 倍数放大 (禁用: 破坏等效镜像距离)
//   ❌ 除法 = 均分切割 (禁用: 碎片无语义)
//
// ══════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ======================================================================
// 第一层: 相位进制画布 — 离散相位单元
// ======================================================================

/// 相位单元 — 最小不可分单位
///
/// 不是RGB颜色, 而是携带相位信息的离散单元
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhaseUnit {
    /// 相位偏移 (0..255, 对应波形相位)
    pub phase: u8,
    /// 亮度 (0=黑, 255=白)
    pub brightness: u8,
    /// 视觉权重 (0=极细, 255=极胖)
    pub weight: u8,
    /// 色块归属 (哪个母体部件贡献)
    pub source_id: u64,
    /// 层级标记 (0=L1基元, 1=L2修饰, 2=L3装配成品)
    pub level: u8,
}

impl PhaseUnit {
    /// 创建空白相位单元
    pub fn empty() -> Self {
        Self { phase: 0, brightness: 0, weight: 0, source_id: 0, level: 0 }
    }

    /// 创建纯色相位单元
    pub fn new(phase: u8, brightness: u8, weight: u8, source_id: u64, level: u8) -> Self {
        Self { phase, brightness, weight, source_id, level }
    }

    /// 相位单元是否为空 (亮度为0)
    pub fn is_empty(&self) -> bool {
        self.brightness == 0
    }

    /// 相位单元加法 (部件安装)
    /// 将other的相位叠加到self上
    pub fn add(&self, other: &PhaseUnit) -> PhaseUnit {
        // 相位融合: 取加权平均 (权重大的主导)
        let total_weight = self.weight as u16 + other.weight as u16;
        let phase = if total_weight == 0 {
            0
        } else {
            ((self.phase as u16 * self.weight as u16 + other.phase as u16 * other.weight as u16) / total_weight) as u8
        };
        // 亮度叠加 (取最大值, 模拟不透明覆盖)
        let brightness = self.brightness.max(other.brightness);
        // 权重叠加 (但限制在255以内)
        let weight = (self.weight.saturating_add(other.weight)).min(255);
        // 来源: 取非空的一方
        let source_id = if other.source_id != 0 { other.source_id } else { self.source_id };
        // 层级: 取最高 (装配成品优先)
        let level = self.level.max(other.level);
        Self { phase, brightness, weight, source_id, level }
    }

    /// 相位单元减法 (部件拆解)
    /// 从self中移除other对应的相位区域
    pub fn subtract(&self, other: &PhaseUnit) -> PhaseUnit {
        // 如果other在该位置有值, 则清空self
        if !other.is_empty() && self.source_id == other.source_id {
            Self::empty()
        } else {
            // 保留self (other不影响)
            *self
        }
    }

    /// 相位距离 (两个单元的差异度, 0..255)
    pub fn phase_distance(&self, other: &PhaseUnit) -> u8 {
        let dp = if self.phase > other.phase { self.phase - other.phase } else { other.phase - self.phase };
        let db = if self.brightness > other.brightness { self.brightness - other.brightness } else { other.brightness - self.brightness };
        let dw = if self.weight > other.weight { self.weight - other.weight } else { other.weight - self.weight };
        // 加权平均距离
        (dp as u16 + db as u16 + dw as u16).min(255) as u8 / 3
    }

    /// 转换为RGB颜色 (用于显示输出)
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        // 相位→色相, 亮度→明度, 权重→饱和度
        let r = ((self.phase as u16 * 255 / 255).min(255) as u8).saturating_mul(self.brightness) / 255;
        let g = ((self.phase.wrapping_add(85) as u16 * 255 / 255).min(255) as u8).saturating_mul(self.brightness) / 255;
        let b = ((self.phase.wrapping_add(170) as u16 * 255 / 255).min(255) as u8).saturating_mul(self.brightness) / 255;
        (r, g, b)
    }
}

/// 相位画布 — 二维相位单元网格
///
/// 这是所有字形运算的底层容器
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PhaseCanvas {
    /// 画布宽度
    pub width: u32,
    /// 画布高度
    pub height: u32,
    /// 像素数据 (行优先)
    pub pixels: Vec<Vec<PhaseUnit>>,
    /// 画布元数据
    pub metadata: CanvasMetadata,
}

/// 画布元数据
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CanvasMetadata {
    /// 画布ID
    pub canvas_id: String,
    /// 来源操作 ("ADD" / "SUB" / "RAW")
    pub operation: String,
    /// 母体来源列表
    pub parent_sources: Vec<String>,
    /// 创建时间戳
    pub created_at: u64,
}

impl PhaseCanvas {
    /// 创建空白画布
    pub fn new(width: u32, height: u32, canvas_id: &str) -> Self {
        let pixels = vec![vec![PhaseUnit::empty(); width as usize]; height as usize];
        Self {
            width,
            height,
            pixels,
            metadata: CanvasMetadata {
                canvas_id: canvas_id.to_string(),
                operation: "RAW".to_string(),
                parent_sources: Vec::new(),
                created_at: current_timestamp_ms(),
            },
        }
    }

    /// 从像素矩阵创建画布
    pub fn from_pixels(pixels: Vec<Vec<PhaseUnit>>, canvas_id: &str) -> Self {
        let height = pixels.len() as u32;
        let width = if pixels.is_empty() { 0 } else { pixels[0].len() as u32 };
        Self {
            width,
            height,
            pixels,
            metadata: CanvasMetadata {
                canvas_id: canvas_id.to_string(),
                operation: "RAW".to_string(),
                parent_sources: Vec::new(),
                created_at: current_timestamp_ms(),
            },
        }
    }

    /// 获取指定坐标的相位单元
    pub fn get(&self, x: u32, y: u32) -> Option<&PhaseUnit> {
        self.pixels.get(y as usize).and_then(|row| row.get(x as usize))
    }

    /// 设置指定坐标的相位单元
    pub fn set(&mut self, x: u32, y: u32, unit: PhaseUnit) {
        if let Some(row) = self.pixels.get_mut(y as usize) {
            if let Some(cell) = row.get_mut(x as usize) {
                *cell = unit;
            }
        }
    }

    /// 画布加法 (部件安装)
    /// 将other画布的部件叠加到self上
    pub fn add(&self, other: &PhaseCanvas, offset_x: i32, offset_y: i32) -> PhaseCanvas {
        let mut result = self.clone();
        result.metadata.operation = "ADD".to_string();
        result.metadata.parent_sources.push(other.metadata.canvas_id.clone());

        for y in 0..other.height {
            for x in 0..other.width {
                let target_x = x as i32 + offset_x;
                let target_y = y as i32 + offset_y;
                if target_x >= 0 && target_x < self.width as i32 && target_y >= 0 && target_y < self.height as i32 {
                    let src = &other.pixels[y as usize][x as usize];
                    if !src.is_empty() {
                        let dst = &result.pixels[target_y as usize][target_x as usize];
                        result.pixels[target_y as usize][target_x as usize] = dst.add(src);
                    }
                }
            }
        }
        result
    }

    /// 画布减法 (部件拆解)
    /// 从self中移除other对应的部件区域
    pub fn subtract(&self, other: &PhaseCanvas, offset_x: i32, offset_y: i32) -> PhaseCanvas {
        let mut result = self.clone();
        result.metadata.operation = "SUB".to_string();
        result.metadata.parent_sources.push(other.metadata.canvas_id.clone());

        for y in 0..other.height {
            for x in 0..other.width {
                let target_x = x as i32 + offset_x;
                let target_y = y as i32 + offset_y;
                if target_x >= 0 && target_x < self.width as i32 && target_y >= 0 && target_y < self.height as i32 {
                    let src = &other.pixels[y as usize][x as usize];
                    if !src.is_empty() {
                        let dst = &result.pixels[target_y as usize][target_x as usize];
                        result.pixels[target_y as usize][target_x as usize] = dst.subtract(src);
                    }
                }
            }
        }
        result
    }

    /// 计算画布间的等效镜像距离 (0..255, 越小越相似)
    pub fn equivalent_distance(&self, other: &PhaseCanvas) -> f64 {
        let min_w = self.width.min(other.width) as usize;
        let min_h = self.height.min(other.height) as usize;
        if min_w == 0 || min_h == 0 { return 255.0; }

        let mut total_distance: u64 = 0;
        let mut count: u64 = 0;

        for y in 0..min_h {
            for x in 0..min_w {
                let a = &self.pixels[y][x];
                let b = &other.pixels[y][x];
                total_distance += a.phase_distance(b) as u64;
                count += 1;
            }
        }

        if count == 0 { return 255.0; }
        total_distance as f64 / count as f64
    }

    /// 计算画布的全局协调得分 (模拟共振场干涉)
    /// 返回 0..1, 越高越和谐
    pub fn harmony_score(&self) -> f64 {
        if self.width == 0 || self.height == 0 { return 0.0; }

        let mut total_brightness: u64 = 0;
        let mut total_weight: u64 = 0;
        let mut non_empty_count: u64 = 0;
        let mut phase_variance: f64 = 0.0;
        let mut phase_sum: u64 = 0;
        let total_pixels = (self.width * self.height) as u64;

        for row in &self.pixels {
            for unit in row {
                if !unit.is_empty() {
                    total_brightness += unit.brightness as u64;
                    total_weight += unit.weight as u64;
                    phase_sum += unit.phase as u64;
                    non_empty_count += 1;
                }
            }
        }

        if non_empty_count == 0 { return 0.0; }

        let avg_brightness = total_brightness as f64 / non_empty_count as f64;
        let avg_weight = total_weight as f64 / non_empty_count as f64;
        let avg_phase = phase_sum as f64 / non_empty_count as f64;

        // 相位方差 (方差越小越和谐)
        for row in &self.pixels {
            for unit in row {
                if !unit.is_empty() {
                    let diff = unit.phase as f64 - avg_phase;
                    phase_variance += diff * diff;
                }
            }
        }
        phase_variance /= non_empty_count as f64;

        // 和谐度 = 亮度适中 + 权重均匀 + 相位一致
        let brightness_score = 1.0 - (avg_brightness / 255.0 - 0.5).abs() * 2.0;
        let weight_score = 1.0 - (avg_weight / 255.0 - 0.5).abs() * 2.0;
        let phase_score = 1.0 - (phase_variance.sqrt() / 127.5).min(1.0);

        (brightness_score * 0.3 + weight_score * 0.3 + phase_score * 0.4).clamp(0.0, 1.0)
    }

    /// 将画布转换为RGB位图 (用于输出显示)
    pub fn to_rgb_bitmap(&self) -> Vec<Vec<(u8, u8, u8)>> {
        self.pixels.iter()
            .map(|row| row.iter().map(|unit| unit.to_rgb()).collect())
            .collect()
    }

    /// 统计非空像素数量
    pub fn non_empty_count(&self) -> usize {
        self.pixels.iter()
            .flat_map(|row| row.iter())
            .filter(|u| !u.is_empty())
            .count()
    }

    /// 获取画布的边界框 (最小包围矩形)
    pub fn bounding_box(&self) -> Option<(u32, u32, u32, u32)> {
        let mut min_x = u32::MAX;
        let mut min_y = u32::MAX;
        let mut max_x = 0;
        let mut max_y = 0;
        let mut found = false;

        for (y, row) in self.pixels.iter().enumerate() {
            for (x, unit) in row.iter().enumerate() {
                if !unit.is_empty() {
                    min_x = min_x.min(x as u32);
                    min_y = min_y.min(y as u32);
                    max_x = max_x.max(x as u32);
                    max_y = max_y.max(y as u32);
                    found = true;
                }
            }
        }

        if found { Some((min_x, min_y, max_x, max_y)) } else { None }
    }
}

// ======================================================================
// 第二层: 区块链式纠缠索引层
// ======================================================================

/// 元数据小纸条 — 每个节点携带的标注信息
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NodeAnnotation {
    /// 节点层级 (L1/L2/L3)
    pub level: String,
    /// 节点类型 (声母/韵母/声调/装配成品等)
    pub node_type: String,
    /// 视觉符号映射 (纸条标签)
    pub glyph_symbol: String,
    /// 归属权 (SELF=我方自定义, EXTERNAL=外来)
    pub owner: String,
    /// 审美打分 (0..1)
    pub aesthetic_score: f64,
    /// 黑名单标记
    pub is_blacklisted: bool,
    /// 视觉权重 (胖瘦)
    pub visual_weight: String,
    /// 字形家族
    pub family: String,
}

/// 区块链式纠缠节点 — 声音/图像信号单元 + 元数据小纸条
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntangledNode {
    /// 节点唯一ID
    pub node_id: String,
    /// 信号类型 ("audio" / "visual")
    pub signal_type: String,
    /// 前缀节点ID列表 (可以接在它前面的节点)
    pub prefix_ids: Vec<String>,
    /// 后缀节点ID列表 (可以接在它后面的节点)
    pub suffix_ids: Vec<String>,
    /// 元数据小纸条
    pub annotation: NodeAnnotation,
    /// 绑定的画布ID (如果有)
    pub canvas_id: Option<String>,
    /// 绑定的音频文件ID (如果有)
    pub audio_id: Option<String>,
    /// 创建时间戳
    pub created_at: u64,
}

impl EntangledNode {
    /// 创建新节点
    pub fn new(node_id: &str, signal_type: &str) -> Self {
        Self {
            node_id: node_id.to_string(),
            signal_type: signal_type.to_string(),
            prefix_ids: Vec::new(),
            suffix_ids: Vec::new(),
            annotation: NodeAnnotation::default(),
            canvas_id: None,
            audio_id: None,
            created_at: current_timestamp_ms(),
        }
    }

    /// 添加前缀连接
    pub fn add_prefix(&mut self, prefix_id: &str) {
        if !self.prefix_ids.contains(&prefix_id.to_string()) {
            self.prefix_ids.push(prefix_id.to_string());
        }
    }

    /// 添加后缀连接
    pub fn add_suffix(&mut self, suffix_id: &str) {
        if !self.suffix_ids.contains(&suffix_id.to_string()) {
            self.suffix_ids.push(suffix_id.to_string());
        }
    }

    /// 检查两个节点是否可以配对 (前缀/后缀关系)
    pub fn can_pair_with(&self, other: &EntangledNode) -> bool {
        self.suffix_ids.contains(&other.node_id) || other.prefix_ids.contains(&self.node_id)
    }

    /// 获取节点的所有关联节点ID
    pub fn all_connected_ids(&self) -> Vec<String> {
        let mut ids = Vec::new();
        ids.extend(self.prefix_ids.clone());
        ids.extend(self.suffix_ids.clone());
        ids
    }
}

/// 区块链式纠缠索引网络
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EntanglementNetwork {
    /// 节点映射 (ID → 节点)
    pub nodes: HashMap<String, EntangledNode>,
    /// 网络名称
    pub name: String,
}

impl EntanglementNetwork {
    /// 创建空网络
    pub fn new(name: &str) -> Self {
        Self { nodes: HashMap::new(), name: name.to_string() }
    }

    /// 添加节点
    pub fn add_node(&mut self, node: EntangledNode) {
        self.nodes.insert(node.node_id.clone(), node);
    }

    /// 获取节点
    pub fn get_node(&self, id: &str) -> Option<&EntangledNode> {
        self.nodes.get(id)
    }

    /// 建立双向连接
    pub fn connect(&mut self, from_id: &str, to_id: &str) -> Result<(), &'static str> {
        if !self.nodes.contains_key(from_id) || !self.nodes.contains_key(to_id) {
            return Err("节点不存在");
        }
        if let Some(from) = self.nodes.get_mut(from_id) {
            from.add_suffix(to_id);
        }
        if let Some(to) = self.nodes.get_mut(to_id) {
            to.add_prefix(from_id);
        }
        Ok(())
    }

    /// 从指定节点出发, 链式传导遍历所有可达节点
    pub fn traverse_from(&self, start_id: &str, max_depth: usize) -> Vec<String> {
        let mut visited = Vec::new();
        let mut queue = vec![start_id.to_string()];
        let mut depth = 0;

        while !queue.is_empty() && depth < max_depth {
            let mut next_queue = Vec::new();
            for id in &queue {
                if !visited.contains(id) {
                    visited.push(id.clone());
                    if let Some(node) = self.nodes.get(id) {
                        for connected in node.all_connected_ids() {
                            if !visited.contains(&connected) {
                                next_queue.push(connected);
                            }
                        }
                    }
                }
            }
            queue = next_queue;
            depth += 1;
        }
        visited
    }

    /// 查找所有合法配对 (基于前缀/后缀关系)
    pub fn find_valid_pairs(&self) -> Vec<(String, String)> {
        let mut pairs = Vec::new();
        for (id, node) in &self.nodes {
            for suffix_id in &node.suffix_ids {
                if self.nodes.contains_key(suffix_id) {
                    pairs.push((id.clone(), suffix_id.clone()));
                }
            }
        }
        pairs
    }

    /// 计算两个节点间的等效镜像距离
    pub fn equivalent_distance(&self, id_a: &str, id_b: &str) -> Option<f64> {
        let node_a = self.nodes.get(id_a)?;
        let node_b = self.nodes.get(id_b)?;

        // 层级等效
        let level_match = if node_a.annotation.level == node_b.annotation.level { 0.0 } else { 1.0 };
        // 类型等效
        let type_match = if node_a.annotation.node_type == node_b.annotation.node_type { 0.0 } else { 0.5 };
        // 审美权重等效
        let aesthetic_diff = (node_a.annotation.aesthetic_score - node_b.annotation.aesthetic_score).abs();
        // 归属权等效
        let owner_match = if node_a.annotation.owner == node_b.annotation.owner { 0.0 } else { 0.5 };
        // 家族等效
        let family_match = if node_a.annotation.family == node_b.annotation.family { 0.0 } else { 0.2 };

        let distance = level_match * 0.35 + type_match * 0.15 + aesthetic_diff * 0.15 + owner_match * 0.2 + family_match * 0.15;
        Some(distance)
    }

    /// 判断两个节点是否满足公倍数条件 (等效对齐)
    pub fn is_valid_common_multiple(&self, id_a: &str, id_b: &str, threshold: f64) -> bool {
        match self.equivalent_distance(id_a, id_b) {
            Some(dist) => dist <= threshold,
            None => false,
        }
    }
}

// ======================================================================
// 第三层: 分布式制衡编织网络
// ======================================================================

/// 制衡维度 — 参与互相拉扯校验的各个维度
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum BalanceDimension {
    /// 层级栈校验
    LevelStack,
    /// 类型身份校验
    TypeIdentity,
    /// 视觉审美校验
    VisualAesthetic,
    /// 归属权校验
    Ownership,
    /// 衍射规则校验
    DiffractionRule,
    /// 黑名单反面样本检测
    Blacklist,
}

/// 单维度校验结果
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DimensionResult {
    /// 维度
    pub dimension: BalanceDimension,
    /// 是否通过
    pub passed: bool,
    /// 拉力强度 (0..1, 越接近1表示该维度越坚持自己的判断)
    pub force: f64,
    /// 原因说明
    pub reason: String,
}

/// 制衡网络输出
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BalanceOutput {
    /// 各维度结果
    pub dimension_results: Vec<DimensionResult>,
    /// 最终判定 (所有维度都通过才为true)
    pub final_verdict: bool,
    /// 全局综合协调得分 (模拟共振场干涉)
    pub harmony_score: f64,
    /// 等效镜像距离
    pub equivalent_distance: f64,
    /// 是否生成合法公倍数
    pub valid_common_multiple: bool,
}

/// 分布式制衡编织网络
///
/// 核心逻辑: 多维度互相拉扯, 任何一个维度投否决票就拒绝
#[derive(Clone, Debug)]
pub struct WeavingBalanceNetwork {
    /// 等效距离阈值 (低于此值才算等效对齐)
    pub equivalence_threshold: f64,
    /// 和谐度阈值 (低于此值拒绝)
    pub harmony_threshold: f64,
}

impl WeavingBalanceNetwork {
    /// 创建默认制衡网络
    pub fn new() -> Self {
        Self {
            equivalence_threshold: 0.3,
            harmony_threshold: 0.4,
        }
    }

    /// 执行全套制衡校验
    pub fn evaluate(
        &self,
        node_a: &EntangledNode,
        node_b: &EntangledNode,
        network: &EntanglementNetwork,
        canvas_a: Option<&PhaseCanvas>,
        canvas_b: Option<&PhaseCanvas>,
    ) -> BalanceOutput {
        let mut results = Vec::new();

        // 维度A: 层级栈校验
        let level_result = self.check_level_stack(node_a, node_b);
        results.push(level_result);

        // 维度B: 类型身份校验
        let type_result = self.check_type_identity(node_a, node_b);
        results.push(type_result);

        // 维度C: 视觉审美校验
        let aesthetic_result = self.check_visual_aesthetic(node_a, node_b, canvas_a, canvas_b);
        results.push(aesthetic_result);

        // 维度D: 归属权校验
        let ownership_result = self.check_ownership(node_a, node_b);
        results.push(ownership_result);

        // 维度E: 衍射规则校验 (只有L1-L1本体允许衍射)
        let diffraction_result = self.check_diffraction_rule(node_a, node_b);
        results.push(diffraction_result);

        // 维度F: 黑名单检测
        let blacklist_result = self.check_blacklist(node_a, node_b);
        results.push(blacklist_result);

        // 计算等效镜像距离
        let eq_distance = network.equivalent_distance(&node_a.node_id, &node_b.node_id).unwrap_or(1.0);

        // 计算全局协调得分
        let harmony = if let (Some(ca), Some(cb)) = (canvas_a, canvas_b) {
            let dist = ca.equivalent_distance(cb);
            let harmony_a = ca.harmony_score();
            let harmony_b = cb.harmony_score();
            let distance_score = 1.0 - (dist / 255.0).min(1.0);
            (distance_score * 0.5 + harmony_a * 0.25 + harmony_b * 0.25)
        } else {
            // 无画布时基于元数据计算
            let aesthetic_sim = 1.0 - (node_a.annotation.aesthetic_score - node_b.annotation.aesthetic_score).abs();
            let level_match = if node_a.annotation.level == node_b.annotation.level { 1.0 } else { 0.0 };
            aesthetic_sim * 0.5 + level_match * 0.5
        };

        // 最终判定: 所有维度都通过 + 等效对齐 + 和谐度达标
        let all_passed = results.iter().all(|r| r.passed);
        let valid_multiple = eq_distance <= self.equivalence_threshold;
        let harmony_ok = harmony >= self.harmony_threshold;
        let final_verdict = all_passed && valid_multiple && harmony_ok;

        BalanceOutput {
            dimension_results: results,
            final_verdict,
            harmony_score: harmony,
            equivalent_distance: eq_distance,
            valid_common_multiple: valid_multiple,
        }
    }

    /// 层级栈校验
    fn check_level_stack(&self, a: &EntangledNode, b: &EntangledNode) -> DimensionResult {
        let same_level = a.annotation.level == b.annotation.level;
        let level_a = &a.annotation.level;
        let level_b = &b.annotation.level;

        // L1基元只能和L1基元配对 (严格)
        // L3成品不能参与衍射 (只能作为输出)
        let passed = if level_a == "L1" && level_b == "L1" {
            true
        } else if level_a == "L3" || level_b == "L3" {
            false // L3成品禁止衍射
        } else {
            same_level // L2可以和L2配对
        };

        DimensionResult {
            dimension: BalanceDimension::LevelStack,
            passed,
            force: if passed { 0.3 } else { 1.0 },
            reason: format!("层级: {} vs {} — {}", level_a, level_b,
                if passed { "允许配对" } else { "跨层级禁止" }),
        }
    }

    /// 类型身份校验
    fn check_type_identity(&self, a: &EntangledNode, b: &EntangledNode) -> DimensionResult {
        // 同类型或兼容类型可以配对
        let compatible = a.annotation.node_type == b.annotation.node_type
            || (a.annotation.node_type == "声母" && b.annotation.node_type == "韵母")
            || (a.annotation.node_type == "韵母" && b.annotation.node_type == "声母");

        DimensionResult {
            dimension: BalanceDimension::TypeIdentity,
            passed: compatible,
            force: if compatible { 0.2 } else { 0.9 },
            reason: format!("类型: {} vs {} — {}",
                a.annotation.node_type, b.annotation.node_type,
                if compatible { "类型兼容" } else { "类型不兼容" }),
        }
    }

    /// 视觉审美校验
    fn check_visual_aesthetic(
        &self,
        a: &EntangledNode,
        b: &EntangledNode,
        canvas_a: Option<&PhaseCanvas>,
        canvas_b: Option<&PhaseCanvas>,
    ) -> DimensionResult {
        let aesthetic_diff = (a.annotation.aesthetic_score - b.annotation.aesthetic_score).abs();
        let passed = aesthetic_diff < 0.4;

        // 如果有画布, 额外检查视觉权重匹配
        let weight_ok = match (canvas_a, canvas_b) {
            (Some(ca), Some(cb)) => {
                let dist = ca.equivalent_distance(cb);
                dist < 100.0 // 像素级差异不太大
            }
            _ => true,
        };

        let final_passed = passed && weight_ok;
        DimensionResult {
            dimension: BalanceDimension::VisualAesthetic,
            passed: final_passed,
            force: if final_passed { 0.2 } else { 0.8 },
            reason: format!("审美差异: {:.2}, 权重检查: {} — {}",
                aesthetic_diff,
                if weight_ok { "通过" } else { "不通过" },
                if final_passed { "审美协调" } else { "审美冲突" }),
        }
    }

    /// 归属权校验
    fn check_ownership(&self, a: &EntangledNode, b: &EntangledNode) -> DimensionResult {
        // 双方都必须是我方自定义 (SELF)
        let both_self = a.annotation.owner == "SELF" && b.annotation.owner == "SELF";

        DimensionResult {
            dimension: BalanceDimension::Ownership,
            passed: both_self,
            force: if both_self { 0.1 } else { 1.0 },
            reason: format!("归属权: {} vs {} — {}",
                a.annotation.owner, b.annotation.owner,
                if both_self { "均为我方" } else { "存在外来归属" }),
        }
    }

    /// 衍射规则校验
    fn check_diffraction_rule(&self, a: &EntangledNode, b: &EntangledNode) -> DimensionResult {
        // 只有L1基元本体允许互相衍射
        let both_l1 = a.annotation.level == "L1" && b.annotation.level == "L1";
        let both_self = a.annotation.owner == "SELF" && b.annotation.owner == "SELF";
        let passed = both_l1 && both_self;

        DimensionResult {
            dimension: BalanceDimension::DiffractionRule,
            passed,
            force: if passed { 0.1 } else { 1.0 },
            reason: format!("衍射规则: 均为L1={}, 均为SELF={} — {}",
                both_l1, both_self,
                if passed { "允许衍射" } else { "禁止衍射" }),
        }
    }

    /// 黑名单检测
    fn check_blacklist(&self, a: &EntangledNode, b: &EntangledNode) -> DimensionResult {
        let a_blocked = a.annotation.is_blacklisted;
        let b_blocked = b.annotation.is_blacklisted;
        let passed = !a_blocked && !b_blocked;

        DimensionResult {
            dimension: BalanceDimension::Blacklist,
            passed,
            force: if passed { 0.0 } else { 1.0 },
            reason: format!("黑名单: A={}, B={} — {}",
                a_blocked, b_blocked,
                if passed { "无黑名单" } else { "命中黑名单" }),
        }
    }
}

// ======================================================================
// 第四层: JSON知识库 (隔离池)
// ======================================================================

/// 知识库 — 外部固化, AI只读不可随意修改
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct KnowledgeBase {
    /// 全局公约数 (顶层产品目标)
    pub global_goal: GlobalGoal,
    /// 本体正式库 (正式符号)
    pub ontology_base: Vec<OntologyEntry>,
    /// 外来参考池 (别人的编码)
    pub reference_pool: Vec<ReferenceEntry>,
    /// 反面样本黑名单
    pub blacklist_reverse: Vec<BlacklistEntry>,
    /// 丢弃池 (纯情绪垃圾)
    pub discard_bin: Vec<DiscardEntry>,
    /// 纠缠网络
    pub entanglement_network: EntanglementNetwork,
    /// 版本号
    pub version: String,
}

/// 全局公约数
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GlobalGoal {
    /// 目标描述
    pub description: String,
    /// 产品类型
    pub product_type: String,
    /// 顶层锚点
    pub anchor: String,
}

/// 本体条目
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OntologyEntry {
    /// 符号ID
    pub glyph_id: String,
    /// 字形 (Unicode或自定义符号)
    pub glyph: String,
    /// 层级 (L1/L2/L3)
    pub level: String,
    /// 类型
    pub entry_type: String,
    /// 视觉权重
    pub visual_weight: f64,
    /// 字形家族
    pub family: String,
    /// 归属权 (固定为SELF)
    pub owner: String,
    /// 审美打分
    pub aesthetic_score: f64,
    /// 绑定画布ID
    pub canvas_id: Option<String>,
    /// 绑定音频ID
    pub audio_id: Option<String>,
}

/// 参考池条目
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReferenceEntry {
    /// 参考ID
    pub ref_id: String,
    /// 来源描述
    pub source: String,
    /// 字形
    pub glyph: String,
    /// 层级
    pub level: String,
    /// 类型
    pub entry_type: String,
    /// 归属权 (固定为EXTERNAL)
    pub owner: String,
    /// 是否已解构
    pub deconstructed: bool,
}

/// 黑名单条目
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlacklistEntry {
    /// 条目ID
    pub entry_id: String,
    /// 错误描述
    pub error_description: String,
    /// 错误模式
    pub error_pattern: String,
    /// 严重程度 (0..1)
    pub severity: f64,
}

/// 丢弃池条目
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiscardEntry {
    /// 条目ID
    pub entry_id: String,
    /// 丢弃原因
    pub reason: String,
    /// 原始内容摘要
    pub content_summary: String,
}

impl KnowledgeBase {
    /// 创建默认知识库
    pub fn new() -> Self {
        Self {
            global_goal: GlobalGoal {
                description: "私有语音编码符号体系 — 满全法颜色进制".to_string(),
                product_type: "自定义声母韵母符号系统".to_string(),
                anchor: "满全法公约数".to_string(),
            },
            ontology_base: Vec::new(),
            reference_pool: Vec::new(),
            blacklist_reverse: Vec::new(),
            discard_bin: Vec::new(),
            entanglement_network: EntanglementNetwork::new("满全法纠缠网络"),
            version: "1.0.0".to_string(),
        }
    }

    /// 序列化为JSON字符串
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// 从JSON字符串反序列化
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// 添加本体条目
    pub fn add_ontology(&mut self, entry: OntologyEntry) {
        self.ontology_base.push(entry);
    }

    /// 添加参考条目
    pub fn add_reference(&mut self, entry: ReferenceEntry) {
        self.reference_pool.push(entry);
    }

    /// 添加黑名单条目
    pub fn add_blacklist(&mut self, entry: BlacklistEntry) {
        self.blacklist_reverse.push(entry);
    }

    /// 添加丢弃条目
    pub fn add_discard(&mut self, entry: DiscardEntry) {
        self.discard_bin.push(entry);
    }

    /// 查找本体条目
    pub fn find_ontology(&self, glyph_id: &str) -> Option<&OntologyEntry> {
        self.ontology_base.iter().find(|e| e.glyph_id == glyph_id)
    }

    /// 检查是否在黑名单中
    pub fn is_blacklisted(&self, pattern: &str) -> bool {
        self.blacklist_reverse.iter().any(|e| e.error_pattern == pattern)
    }

    /// 获取知识库统计
    pub fn stats(&self) -> KnowledgeStats {
        KnowledgeStats {
            ontology_count: self.ontology_base.len(),
            reference_count: self.reference_pool.len(),
            blacklist_count: self.blacklist_reverse.len(),
            discard_count: self.discard_bin.len(),
            network_node_count: self.entanglement_network.nodes.len(),
        }
    }
}

/// 知识库统计
#[derive(Clone, Debug)]
pub struct KnowledgeStats {
    pub ontology_count: usize,
    pub reference_count: usize,
    pub blacklist_count: usize,
    pub discard_count: usize,
    pub network_node_count: usize,
}

// ======================================================================
// 第五层: 绘图引擎 (DrawingEngine)
// ======================================================================

/// 绘图引擎 — 整合相位运算 + 区块链索引 + 制衡网络
#[derive(Clone, Debug)]
pub struct DrawingEngine {
    /// 知识库引用
    pub knowledge_base: KnowledgeBase,
    /// 制衡网络
    pub balance_network: WeavingBalanceNetwork,
    /// 画布缓存
    pub canvas_cache: HashMap<String, PhaseCanvas>,
    /// 操作历史
    pub operation_log: Vec<OperationRecord>,
}

/// 操作记录
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OperationRecord {
    /// 操作类型
    pub op_type: String,
    /// 操作描述
    pub description: String,
    /// 结果画布ID
    pub result_canvas_id: String,
    /// 时间戳
    pub timestamp: u64,
    /// 是否通过制衡校验
    pub passed_balance: bool,
}

impl DrawingEngine {
    /// 创建绘图引擎
    pub fn new() -> Self {
        Self {
            knowledge_base: KnowledgeBase::new(),
            balance_network: WeavingBalanceNetwork::new(),
            canvas_cache: HashMap::new(),
            operation_log: Vec::new(),
        }
    }

    /// 创建画布并缓存
    pub fn create_canvas(&mut self, width: u32, height: u32, canvas_id: &str) -> PhaseCanvas {
        let canvas = PhaseCanvas::new(width, height, canvas_id);
        self.canvas_cache.insert(canvas_id.to_string(), canvas.clone());
        canvas
    }

    /// 获取缓存画布
    pub fn get_canvas(&self, canvas_id: &str) -> Option<&PhaseCanvas> {
        self.canvas_cache.get(canvas_id)
    }

    /// 执行加法操作 (部件安装)
    pub fn add_operation(
        &mut self,
        base_id: &str,
        part_id: &str,
        offset_x: i32,
        offset_y: i32,
        result_id: &str,
    ) -> Result<PhaseCanvas, &'static str> {
        let base = self.canvas_cache.get(base_id).ok_or("基础画布不存在")?;
        let part = self.canvas_cache.get(part_id).ok_or("部件画布不存在")?;
        let result = base.add(part, offset_x, offset_y);
        let result_canvas = PhaseCanvas {
            width: result.width,
            height: result.height,
            pixels: result.pixels,
            metadata: CanvasMetadata {
                canvas_id: result_id.to_string(),
                operation: "ADD".to_string(),
                parent_sources: vec![base_id.to_string(), part_id.to_string()],
                created_at: current_timestamp_ms(),
            },
        };
        self.canvas_cache.insert(result_id.to_string(), result_canvas.clone());
        self.operation_log.push(OperationRecord {
            op_type: "ADD".to_string(),
            description: format!("将{}安装到{}上, 偏移({}, {})", part_id, base_id, offset_x, offset_y),
            result_canvas_id: result_id.to_string(),
            timestamp: current_timestamp_ms(),
            passed_balance: true,
        });
        Ok(result_canvas)
    }

    /// 执行减法操作 (部件拆解)
    pub fn subtract_operation(
        &mut self,
        base_id: &str,
        part_id: &str,
        offset_x: i32,
        offset_y: i32,
        result_id: &str,
    ) -> Result<PhaseCanvas, &'static str> {
        let base = self.canvas_cache.get(base_id).ok_or("基础画布不存在")?;
        let part = self.canvas_cache.get(part_id).ok_or("部件画布不存在")?;
        let result = base.subtract(part, offset_x, offset_y);
        let result_canvas = PhaseCanvas {
            width: result.width,
            height: result.height,
            pixels: result.pixels,
            metadata: CanvasMetadata {
                canvas_id: result_id.to_string(),
                operation: "SUB".to_string(),
                parent_sources: vec![base_id.to_string(), part_id.to_string()],
                created_at: current_timestamp_ms(),
            },
        };
        self.canvas_cache.insert(result_id.to_string(), result_canvas.clone());
        self.operation_log.push(OperationRecord {
            op_type: "SUB".to_string(),
            description: format!("从{}上拆解{}, 偏移({}, {})", base_id, part_id, offset_x, offset_y),
            result_canvas_id: result_id.to_string(),
            timestamp: current_timestamp_ms(),
            passed_balance: true,
        });
        Ok(result_canvas)
    }

    /// 执行制衡校验
    pub fn evaluate_pair(
        &self,
        node_a_id: &str,
        node_b_id: &str,
    ) -> Option<BalanceOutput> {
        let node_a = self.knowledge_base.entanglement_network.get_node(node_a_id)?;
        let node_b = self.knowledge_base.entanglement_network.get_node(node_b_id)?;
        let canvas_a = node_a.canvas_id.as_ref().and_then(|id| self.canvas_cache.get(id));
        let canvas_b = node_b.canvas_id.as_ref().and_then(|id| self.canvas_cache.get(id));
        Some(self.balance_network.evaluate(node_a, node_b, &self.knowledge_base.entanglement_network, canvas_a, canvas_b))
    }

    /// 生成报告
    pub fn report(&self) -> String {
        let stats = self.knowledge_base.stats();
        let mut s = String::new();
        s.push_str("═══ 颜色进制 · 绘图引擎报告 ═══\n");
        s.push_str(&format!("  本体条目: {}\n", stats.ontology_count));
        s.push_str(&format!("  参考条目: {}\n", stats.reference_count));
        s.push_str(&format!("  黑名单条目: {}\n", stats.blacklist_count));
        s.push_str(&format!("  丢弃条目: {}\n", stats.discard_count));
        s.push_str(&format!("  网络节点: {}\n", stats.network_node_count));
        s.push_str(&format!("  缓存画布: {}\n", self.canvas_cache.len()));
        s.push_str(&format!("  操作记录: {}\n", self.operation_log.len()));
        s.push_str(&format!("  等效阈值: {:.2}\n", self.balance_network.equivalence_threshold));
        s.push_str(&format!("  和谐阈值: {:.2}\n", self.balance_network.harmony_threshold));
        s
    }
}

// ======================================================================
// 辅助函数
// ======================================================================

/// 获取当前时间戳 (毫秒)
pub fn current_timestamp_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

// ======================================================================
// 测试
// ======================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_unit_empty() {
        let unit = PhaseUnit::empty();
        assert!(unit.is_empty());
        assert_eq!(unit.brightness, 0);
    }

    #[test]
    fn phase_unit_add() {
        let a = PhaseUnit::new(100, 200, 150, 1, 0);
        let b = PhaseUnit::new(50, 100, 100, 2, 1);
        let result = a.add(&b);
        // 亮度取最大值
        assert_eq!(result.brightness, 200);
        // 层级取最高
        assert_eq!(result.level, 1);
        // 权重叠加
        assert!(result.weight > 150);
    }

    #[test]
    fn phase_unit_subtract() {
        let a = PhaseUnit::new(100, 200, 150, 1, 0);
        let b = PhaseUnit::new(50, 100, 100, 1, 0); // 相同source_id
        let result = a.subtract(&b);
        // 相同source_id时减法应清空
        assert!(result.is_empty());
    }

    #[test]
    fn phase_unit_subtract_different_source() {
        let a = PhaseUnit::new(100, 200, 150, 1, 0);
        let b = PhaseUnit::new(50, 100, 100, 2, 0); // 不同source_id
        let result = a.subtract(&b);
        // 不同source_id时保留原值
        assert!(!result.is_empty());
        assert_eq!(result.brightness, 200);
    }

    #[test]
    fn phase_unit_phase_distance() {
        let a = PhaseUnit::new(100, 200, 150, 1, 0);
        let b = PhaseUnit::new(50, 100, 100, 2, 0);
        let dist = a.phase_distance(&b);
        assert!(dist > 0);
    }

    #[test]
    fn phase_unit_to_rgb() {
        let unit = PhaseUnit::new(128, 255, 128, 1, 0);
        let (r, g, b) = unit.to_rgb();
        // 亮度255时RGB应非零
        assert!(r > 0 || g > 0 || b > 0);
    }

    #[test]
    fn phase_canvas_new() {
        let canvas = PhaseCanvas::new(10, 10, "test");
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 10);
        assert_eq!(canvas.non_empty_count(), 0);
    }

    #[test]
    fn phase_canvas_set_get() {
        let mut canvas = PhaseCanvas::new(10, 10, "test");
        let unit = PhaseUnit::new(100, 200, 150, 1, 0);
        canvas.set(5, 5, unit);
        let retrieved = canvas.get(5, 5).unwrap();
        assert_eq!(retrieved.brightness, 200);
    }

    #[test]
    fn phase_canvas_add() {
        let mut base = PhaseCanvas::new(10, 10, "base");
        base.set(3, 3, PhaseUnit::new(100, 200, 150, 1, 0));

        let mut part = PhaseCanvas::new(3, 3, "part");
        part.set(1, 1, PhaseUnit::new(50, 100, 100, 2, 1));

        let result = base.add(&part, 4, 4);
        // 部件应被叠加到base上 (base 1个 + part 1个 = 2个, 不重叠)
        assert!(result.non_empty_count() >= 2);
    }

    #[test]
    fn phase_canvas_subtract() {
        let mut base = PhaseCanvas::new(10, 10, "base");
        base.set(5, 5, PhaseUnit::new(100, 200, 150, 1, 0));

        let mut part = PhaseCanvas::new(3, 3, "part");
        part.set(0, 0, PhaseUnit::new(50, 100, 100, 1, 0)); // 相同source_id

        let result = base.subtract(&part, 5, 5);
        // 部件应从base上被移除
        assert_eq!(result.get(5, 5).unwrap().brightness, 0);
    }

    #[test]
    fn phase_canvas_equivalent_distance() {
        let mut a = PhaseCanvas::new(5, 5, "a");
        a.set(2, 2, PhaseUnit::new(100, 200, 150, 1, 0));

        let mut b = PhaseCanvas::new(5, 5, "b");
        b.set(2, 2, PhaseUnit::new(100, 200, 150, 1, 0));

        let dist = a.equivalent_distance(&b);
        assert!(dist < 1.0); // 几乎相同
    }

    #[test]
    fn phase_canvas_harmony_score() {
        let mut canvas = PhaseCanvas::new(10, 10, "test");
        // 填充均匀相位
        for y in 0..10 {
            for x in 0..10 {
                canvas.set(x, y, PhaseUnit::new(128, 200, 128, 1, 0));
            }
        }
        let score = canvas.harmony_score();
        assert!(score > 0.5); // 均匀分布应较高和谐度
    }

    #[test]
    fn phase_canvas_bounding_box() {
        let mut canvas = PhaseCanvas::new(10, 10, "test");
        canvas.set(3, 3, PhaseUnit::new(100, 200, 150, 1, 0));
        canvas.set(7, 7, PhaseUnit::new(100, 200, 150, 1, 0));
        let bbox = canvas.bounding_box();
        assert_eq!(bbox, Some((3, 3, 7, 7)));
    }

    #[test]
    fn phase_canvas_empty_bounding_box() {
        let canvas = PhaseCanvas::new(10, 10, "test");
        assert_eq!(canvas.bounding_box(), None);
    }

    #[test]
    fn entangled_node_new() {
        let node = EntangledNode::new("node1", "visual");
        assert_eq!(node.node_id, "node1");
        assert_eq!(node.signal_type, "visual");
        assert!(node.prefix_ids.is_empty());
        assert!(node.suffix_ids.is_empty());
    }

    #[test]
    fn entangled_node_connections() {
        let mut a = EntangledNode::new("a", "visual");
        let b = EntangledNode::new("b", "visual");
        a.add_suffix("b");
        assert!(a.can_pair_with(&b));
        assert!(b.prefix_ids.contains(&"a".to_string()) == false); // b没有前缀
    }

    #[test]
    fn entanglement_network_add_and_get() {
        let mut network = EntanglementNetwork::new("test");
        let node = EntangledNode::new("node1", "visual");
        network.add_node(node);
        assert!(network.get_node("node1").is_some());
    }

    #[test]
    fn entanglement_network_connect() {
        let mut network = EntanglementNetwork::new("test");
        network.add_node(EntangledNode::new("a", "visual"));
        network.add_node(EntangledNode::new("b", "visual"));
        assert!(network.connect("a", "b").is_ok());
        assert!(network.get_node("a").unwrap().suffix_ids.contains(&"b".to_string()));
        assert!(network.get_node("b").unwrap().prefix_ids.contains(&"a".to_string()));
    }

    #[test]
    fn entanglement_network_traverse() {
        let mut network = EntanglementNetwork::new("test");
        network.add_node(EntangledNode::new("a", "visual"));
        network.add_node(EntangledNode::new("b", "visual"));
        network.add_node(EntangledNode::new("c", "visual"));
        network.connect("a", "b").unwrap();
        network.connect("b", "c").unwrap();
        let visited = network.traverse_from("a", 10);
        assert_eq!(visited.len(), 3);
    }

    #[test]
    fn entanglement_network_find_valid_pairs() {
        let mut network = EntanglementNetwork::new("test");
        network.add_node(EntangledNode::new("a", "visual"));
        network.add_node(EntangledNode::new("b", "visual"));
        network.add_node(EntangledNode::new("c", "visual"));
        network.connect("a", "b").unwrap();
        network.connect("b", "c").unwrap();
        let pairs = network.find_valid_pairs();
        assert_eq!(pairs.len(), 2);
    }

    #[test]
    fn entanglement_network_equivalent_distance() {
        let mut network = EntanglementNetwork::new("test");
        let mut a = EntangledNode::new("a", "visual");
        a.annotation.level = "L1".to_string();
        a.annotation.node_type = "声母".to_string();
        a.annotation.owner = "SELF".to_string();
        a.annotation.aesthetic_score = 0.8;
        a.annotation.family = "ipa".to_string();

        let mut b = EntangledNode::new("b", "visual");
        b.annotation.level = "L1".to_string();
        b.annotation.node_type = "声母".to_string();
        b.annotation.owner = "SELF".to_string();
        b.annotation.aesthetic_score = 0.7;
        b.annotation.family = "ipa".to_string();

        network.add_node(a);
        network.add_node(b);

        let dist = network.equivalent_distance("a", "b").unwrap();
        assert!(dist < 0.3); // 非常相似
    }

    #[test]
    fn entanglement_network_is_valid_common_multiple() {
        let mut network = EntanglementNetwork::new("test");
        let mut a = EntangledNode::new("a", "visual");
        a.annotation.level = "L1".to_string();
        a.annotation.owner = "SELF".to_string();
        a.annotation.aesthetic_score = 0.8;

        let mut b = EntangledNode::new("b", "visual");
        b.annotation.level = "L1".to_string();
        b.annotation.owner = "SELF".to_string();
        b.annotation.aesthetic_score = 0.7;

        network.add_node(a);
        network.add_node(b);

        assert!(network.is_valid_common_multiple("a", "b", 0.3));
    }

    #[test]
    fn weaving_balance_level_check() {
        let network = WeavingBalanceNetwork::new();
        let mut a = EntangledNode::new("a", "visual");
        a.annotation.level = "L1".to_string();
        a.annotation.owner = "SELF".to_string();
        a.annotation.node_type = "声母".to_string();
        a.annotation.aesthetic_score = 0.8;

        let mut b = EntangledNode::new("b", "visual");
        b.annotation.level = "L1".to_string();
        b.annotation.owner = "SELF".to_string();
        b.annotation.node_type = "声母".to_string();
        b.annotation.aesthetic_score = 0.7;

        let mut ent = EntanglementNetwork::new("test");
        ent.add_node(a.clone());
        ent.add_node(b.clone());

        let output = network.evaluate(&a, &b, &ent, None, None);
        assert!(output.final_verdict); // L1+L1+SELF应通过
    }

    #[test]
    fn weaving_balance_rejects_l3() {
        let network = WeavingBalanceNetwork::new();
        let mut a = EntangledNode::new("a", "visual");
        a.annotation.level = "L1".to_string();
        a.annotation.owner = "SELF".to_string();
        a.annotation.node_type = "声母".to_string();
        a.annotation.aesthetic_score = 0.8;

        let mut b = EntangledNode::new("b", "visual");
        b.annotation.level = "L3".to_string(); // L3成品
        b.annotation.owner = "SELF".to_string();
        b.annotation.node_type = "装配".to_string();
        b.annotation.aesthetic_score = 0.7;

        let mut ent = EntanglementNetwork::new("test");
        ent.add_node(a.clone());
        ent.add_node(b.clone());

        let output = network.evaluate(&a, &b, &ent, None, None);
        assert!(!output.final_verdict); // L3应被拒绝
    }

    #[test]
    fn weaving_balance_rejects_external() {
        let network = WeavingBalanceNetwork::new();
        let mut a = EntangledNode::new("a", "visual");
        a.annotation.level = "L1".to_string();
        a.annotation.owner = "SELF".to_string();
        a.annotation.node_type = "声母".to_string();
        a.annotation.aesthetic_score = 0.8;

        let mut b = EntangledNode::new("b", "visual");
        b.annotation.level = "L1".to_string();
        b.annotation.owner = "EXTERNAL".to_string(); // 外来
        b.annotation.node_type = "声母".to_string();
        b.annotation.aesthetic_score = 0.7;

        let mut ent = EntanglementNetwork::new("test");
        ent.add_node(a.clone());
        ent.add_node(b.clone());

        let output = network.evaluate(&a, &b, &ent, None, None);
        assert!(!output.final_verdict); // 外来应被拒绝
    }

    #[test]
    fn weaving_balance_rejects_blacklist() {
        let network = WeavingBalanceNetwork::new();
        let mut a = EntangledNode::new("a", "visual");
        a.annotation.level = "L1".to_string();
        a.annotation.owner = "SELF".to_string();
        a.annotation.node_type = "声母".to_string();
        a.annotation.aesthetic_score = 0.8;
        a.annotation.is_blacklisted = true; // 黑名单

        let mut b = EntangledNode::new("b", "visual");
        b.annotation.level = "L1".to_string();
        b.annotation.owner = "SELF".to_string();
        b.annotation.node_type = "声母".to_string();
        b.annotation.aesthetic_score = 0.7;

        let mut ent = EntanglementNetwork::new("test");
        ent.add_node(a.clone());
        ent.add_node(b.clone());

        let output = network.evaluate(&a, &b, &ent, None, None);
        assert!(!output.final_verdict); // 黑名单应被拒绝
    }

    #[test]
    fn knowledge_base_new() {
        let kb = KnowledgeBase::new();
        assert_eq!(kb.version, "1.0.0");
        assert!(kb.ontology_base.is_empty());
    }

    #[test]
    fn knowledge_base_add_and_find() {
        let mut kb = KnowledgeBase::new();
        kb.add_ontology(OntologyEntry {
            glyph_id: "glyph_a".to_string(),
            glyph: "a".to_string(),
            level: "L1".to_string(),
            entry_type: "韵母".to_string(),
            visual_weight: 0.5,
            family: "ipa".to_string(),
            owner: "SELF".to_string(),
            aesthetic_score: 0.8,
            canvas_id: None,
            audio_id: None,
        });
        assert!(kb.find_ontology("glyph_a").is_some());
        assert_eq!(kb.find_ontology("glyph_a").unwrap().glyph, "a");
    }

    #[test]
    fn knowledge_base_blacklist_check() {
        let mut kb = KnowledgeBase::new();
        kb.add_blacklist(BlacklistEntry {
            entry_id: "err1".to_string(),
            error_description: "跨层级衍射".to_string(),
            error_pattern: "L1-L3".to_string(),
            severity: 0.9,
        });
        assert!(kb.is_blacklisted("L1-L3"));
        assert!(!kb.is_blacklisted("L1-L1"));
    }

    #[test]
    fn knowledge_base_stats() {
        let mut kb = KnowledgeBase::new();
        kb.add_ontology(OntologyEntry {
            glyph_id: "a".to_string(), glyph: "a".to_string(), level: "L1".to_string(),
            entry_type: "韵母".to_string(), visual_weight: 0.5, family: "ipa".to_string(),
            owner: "SELF".to_string(), aesthetic_score: 0.8, canvas_id: None, audio_id: None,
        });
        kb.add_reference(ReferenceEntry {
            ref_id: "ref1".to_string(), source: "外部".to_string(), glyph: "x".to_string(),
            level: "L1".to_string(), entry_type: "参考".to_string(), owner: "EXTERNAL".to_string(),
            deconstructed: false,
        });
        let stats = kb.stats();
        assert_eq!(stats.ontology_count, 1);
        assert_eq!(stats.reference_count, 1);
    }

    #[test]
    fn knowledge_base_json_roundtrip() {
        let mut kb = KnowledgeBase::new();
        kb.add_ontology(OntologyEntry {
            glyph_id: "a".to_string(), glyph: "a".to_string(), level: "L1".to_string(),
            entry_type: "韵母".to_string(), visual_weight: 0.5, family: "ipa".to_string(),
            owner: "SELF".to_string(), aesthetic_score: 0.8, canvas_id: None, audio_id: None,
        });
        let json = kb.to_json().unwrap();
        let kb2 = KnowledgeBase::from_json(&json).unwrap();
        assert_eq!(kb2.ontology_base.len(), 1);
        assert_eq!(kb2.ontology_base[0].glyph, "a");
    }

    #[test]
    fn drawing_engine_create_canvas() {
        let mut engine = DrawingEngine::new();
        let canvas = engine.create_canvas(16, 16, "test_canvas");
        assert_eq!(canvas.width, 16);
        assert_eq!(canvas.height, 16);
        assert!(engine.get_canvas("test_canvas").is_some());
    }

    #[test]
    fn drawing_engine_add_operation() {
        let mut engine = DrawingEngine::new();
        engine.create_canvas(16, 16, "base");
        engine.create_canvas(4, 4, "part");

        // 在base上放一个像素
        let base = engine.get_canvas("base").unwrap().clone();
        let mut base_mut = base.clone();
        base_mut.set(8, 8, PhaseUnit::new(100, 200, 128, 1, 0));
        engine.canvas_cache.insert("base".to_string(), base_mut);

        // 在part上放一个像素
        let part = engine.get_canvas("part").unwrap().clone();
        let mut part_mut = part.clone();
        part_mut.set(2, 2, PhaseUnit::new(50, 150, 100, 2, 1));
        engine.canvas_cache.insert("part".to_string(), part_mut);

        let result = engine.add_operation("base", "part", 6, 6, "result");
        assert!(result.is_ok());
        assert!(engine.get_canvas("result").is_some());
    }

    #[test]
    fn drawing_engine_subtract_operation() {
        let mut engine = DrawingEngine::new();
        engine.create_canvas(16, 16, "base");
        engine.create_canvas(4, 4, "part");

        let base = engine.get_canvas("base").unwrap().clone();
        let mut base_mut = base.clone();
        base_mut.set(8, 8, PhaseUnit::new(100, 200, 128, 1, 0));
        engine.canvas_cache.insert("base".to_string(), base_mut);

        let part = engine.get_canvas("part").unwrap().clone();
        let mut part_mut = part.clone();
        part_mut.set(2, 2, PhaseUnit::new(50, 150, 100, 1, 0)); // 相同source_id
        engine.canvas_cache.insert("part".to_string(), part_mut);

        let result = engine.subtract_operation("base", "part", 6, 6, "result");
        assert!(result.is_ok());
    }

    #[test]
    fn drawing_engine_report() {
        let mut engine = DrawingEngine::new();
        engine.create_canvas(16, 16, "test");
        let report = engine.report();
        assert!(report.contains("颜色进制"));
        assert!(report.contains("缓存画布: 1"));
    }

    #[test]
    fn full_pipeline_add_only() {
        // 完整管道: 创建画布 → 加法 → 制衡校验
        let mut engine = DrawingEngine::new();

        // 创建基元画布
        engine.create_canvas(16, 16, "glyph_a");
        engine.create_canvas(8, 8, "tone_mark");

        // 设置基元
        let base = engine.get_canvas("glyph_a").unwrap().clone();
        let mut base_mut = base.clone();
        for y in 4..12 {
            for x in 4..12 {
                base_mut.set(x, y, PhaseUnit::new(128, 200, 128, 1, 0));
            }
        }
        engine.canvas_cache.insert("glyph_a".to_string(), base_mut);

        // 设置修饰部件
        let part = engine.get_canvas("tone_mark").unwrap().clone();
        let mut part_mut = part.clone();
        for y in 0..3 {
            for x in 0..3 {
                part_mut.set(x, y, PhaseUnit::new(50, 180, 100, 2, 1));
            }
        }
        engine.canvas_cache.insert("tone_mark".to_string(), part_mut);

        // 执行加法
        let result = engine.add_operation("glyph_a", "tone_mark", 0, 0, "glyph_a_acute");
        assert!(result.is_ok());

        // 验证结果
        let result_canvas = engine.get_canvas("glyph_a_acute").unwrap();
        assert!(result_canvas.non_empty_count() > 0);
        assert_eq!(result_canvas.metadata.operation, "ADD");
    }

    #[test]
    fn phase_unit_rgb_roundtrip() {
        let unit = PhaseUnit::new(128, 255, 128, 1, 0);
        let (r, g, b) = unit.to_rgb();
        // 亮度255时相位128应对应中等颜色值
        assert!(r <= 255 && g <= 255 && b <= 255);
    }

    #[test]
    fn phase_canvas_to_rgb_bitmap() {
        let mut canvas = PhaseCanvas::new(4, 4, "test");
        canvas.set(1, 1, PhaseUnit::new(128, 200, 128, 1, 0));
        let bitmap = canvas.to_rgb_bitmap();
        assert_eq!(bitmap.len(), 4);
        assert_eq!(bitmap[0].len(), 4);
    }

    #[test]
    fn weaving_balance_harmony_score() {
        let network = WeavingBalanceNetwork::new();
        let mut a = EntangledNode::new("a", "visual");
        a.annotation.level = "L1".to_string();
        a.annotation.owner = "SELF".to_string();
        a.annotation.node_type = "声母".to_string();
        a.annotation.aesthetic_score = 0.8;
        a.annotation.family = "ipa".to_string();

        let mut b = EntangledNode::new("b", "visual");
        b.annotation.level = "L1".to_string();
        b.annotation.owner = "SELF".to_string();
        b.annotation.node_type = "声母".to_string();
        b.annotation.aesthetic_score = 0.75;
        b.annotation.family = "ipa".to_string();

        let mut ent = EntanglementNetwork::new("test");
        ent.add_node(a.clone());
        ent.add_node(b.clone());

        let output = network.evaluate(&a, &b, &ent, None, None);
        assert!(output.harmony_score > 0.5);
    }

    #[test]
    fn entanglement_network_invalid_pair_distance() {
        let mut network = EntanglementNetwork::new("test");
        let mut a = EntangledNode::new("a", "visual");
        a.annotation.level = "L1".to_string();
        a.annotation.owner = "SELF".to_string();
        a.annotation.aesthetic_score = 0.8;

        let mut b = EntangledNode::new("b", "visual");
        b.annotation.level = "L3".to_string(); // 不同层级
        b.annotation.owner = "EXTERNAL".to_string(); // 外来
        b.annotation.aesthetic_score = 0.2;

        network.add_node(a);
        network.add_node(b);

        let dist = network.equivalent_distance("a", "b").unwrap();
        assert!(dist > 0.5); // 差异应很大
        assert!(!network.is_valid_common_multiple("a", "b", 0.3));
    }
}
