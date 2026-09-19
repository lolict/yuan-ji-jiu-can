// live_code_compiler.rs — 活码编译器引擎 (v0.1)
// 起点 · 活码 · 加法统计 · 编码固定 = 权限数值
// ══════════════════════════════════════════════════════════════
//
//   核心命题 (满全法·活码编译器):
//
//   "活码" = 未固定前位置任意活动的编码,
//   固定后按排列顺序确定权限和数值 (加法统计法)。
//
//   - "起点" (Starting Point) = 钥匙 = 激活活码的触发器
//   - 活码未固定时: 位置任意活动, 数值不确定
//   - 活码固定后: 排列顺序确定权限和数值
//   - 加法统计法: 数值 = 各活码计数之和 (非位加权)
//
//   NJSG 意义:
//     编译不是"翻译源码为目标码",
//     而是"通过起点激活活码 → 固定位置 → 涌现权限"。
//
// ══════════════════════════════════════════════════════════════

use std::collections::HashMap;

// ─────────────────────────────────────────────────────────
// 活码 (Living Code)
// ─────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct LivingCode {
    pub id: u64,
    /// 当前位置 (未固定 = None)
    pub position: Option<u32>,
    /// 计数 (加法统计法中的贡献值)
    pub count: u32,
    /// 是否已固定
    pub fixed: bool,
    /// 类别
    pub kind: CodeKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeKind {
    StartPoint,    // 起点 (钥匙)
    Operation,     // 运算码
    Data,          // 数据码
    Router,        // 路由码
    Sovereign,     // 主权码
}

impl LivingCode {
    pub fn new(id: u64, kind: CodeKind) -> Self {
        Self { id, position: None, count: 1, fixed: false, kind }
    }

    /// 固定在指定位置
    pub fn fix_at(&mut self, pos: u32) -> bool {
        if self.fixed { return false; }
        self.position = Some(pos);
        self.fixed = true;
        true
    }

    /// 未固定时移动位置
    pub fn move_to(&mut self, pos: u32) -> bool {
        if self.fixed { return false; }
        self.position = Some(pos);
        true
    }

    /// 计算数值贡献 (加法统计法)
    pub fn value(&self) -> u64 {
        if self.fixed {
            self.count as u64
        } else {
            0 // 未固定 = 无值
        }
    }
}

// ─────────────────────────────────────────────────────────
// 起点 (钥匙)
// ─────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct StartPoint {
    pub id: u64,
    pub name: String,
    pub energy: f64,
    pub activated: bool,
}

impl StartPoint {
    pub fn new(id: u64, name: &str) -> Self {
        Self { id, name: name.to_string(), energy: 1.0, activated: false }
    }

    /// 激活起点 (打开编译通道)
    pub fn activate(&mut self) -> bool {
        if self.energy < 0.1 { return false; }
        self.activated = true;
        self.energy *= 0.9; // 消耗能量
        true
    }
}

// ─────────────────────────────────────────────────────────
// 活码编译器
// ─────────────────────────────────────────────────────────

pub struct HunyuanLiveCoder {
    /// 活码池
    pub codes: Vec<LivingCode>,
    /// 起点 (钥匙)
    pub start_points: Vec<StartPoint>,
    /// 编译产物缓存
    pub output_cache: HashMap<u64, CompiledLayout>,
}

#[derive(Clone, Debug)]
pub struct CompiledLayout {
    pub start_point_id: u64,
    pub codes: Vec<LivingCode>,
    pub total_value: u64,
    pub permission_level: u8,
}

impl HunyuanLiveCoder {
    pub fn new() -> Self {
        Self {
            codes: Vec::new(),
            start_points: Vec::new(),
            output_cache: HashMap::new(),
        }
    }

    /// 添加活码
    pub fn add_code(&mut self, kind: CodeKind) -> u64 {
        let id = self.codes.len() as u64;
        self.codes.push(LivingCode::new(id, kind));
        id
    }

    /// 添加起点
    pub fn add_start_point(&mut self, name: &str) -> u64 {
        let id = self.start_points.len() as u64;
        self.start_points.push(StartPoint::new(id, name));
        id
    }

    /// 编译: 激活起点 → 固定活码 → 计算数值
    pub fn compile(&mut self, start_point_id: u64, fixed_order: &[(u64, u32)]) -> Option<CompiledLayout> {
        // 1. 激活起点
        let sp = self.start_points.get_mut(start_point_id as usize)?;
        if !sp.activate() { return None; }

        // 2. 按给定顺序固定活码
        let mut codes = Vec::new();
        for &(code_id, pos) in fixed_order {
            let code = self.codes.get_mut(code_id as usize)?;
            code.fix_at(pos);
            codes.push(code.clone());
        }

        // 3. 加法统计法计算总值
        let total_value: u64 = codes.iter().map(|c| c.value()).sum();

        // 4. 权限级别 = 固定码数 (不超过 255)
        let permission_level = codes.len().min(255) as u8;

        let layout = CompiledLayout {
            start_point_id,
            codes,
            total_value,
            permission_level,
        };
        self.output_cache.insert(start_point_id, layout.clone());
        Some(layout)
    }

    /// 批量编译多个起点
    pub fn compile_all(&mut self, orders: &[(u64, Vec<(u64, u32)>)]) -> Vec<CompiledLayout> {
        let mut results = Vec::new();
        for &(sp_id, ref order) in orders {
            if let Some(layout) = self.compile(sp_id, order) {
                results.push(layout);
            }
        }
        results
    }

    /// 未固定的活码
    pub fn unfixed_codes(&self) -> Vec<&LivingCode> {
        self.codes.iter().filter(|c| !c.fixed).collect()
    }

    /// 已固定的活码
    pub fn fixed_codes(&self) -> Vec<&LivingCode> {
        self.codes.iter().filter(|c| c.fixed).collect()
    }

    /// 活码总活跃度 (未固定码数 / 总数)
    pub fn liveness_ratio(&self) -> f64 {
        if self.codes.is_empty() { return 0.0; }
        self.unfixed_codes().len() as f64 / self.codes.len() as f64
    }
}

// ═══════════════════════════════════════════════════════════
// 测试
// ═══════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn living_code_new_unfixed() {
        let code = LivingCode::new(1, CodeKind::Operation);
        assert!(!code.fixed);
        assert_eq!(code.value(), 0);
    }

    #[test]
    fn living_code_fix_gives_value() {
        let mut code = LivingCode::new(1, CodeKind::Operation);
        assert!(code.fix_at(5));
        assert!(code.fixed);
        assert_eq!(code.value(), 1); // count = 1
    }

    #[test]
    fn living_code_cannot_refreeze() {
        let mut code = LivingCode::new(1, CodeKind::Operation);
        assert!(code.fix_at(5));
        assert!(!code.fix_at(10), "已固定不能再固定");
    }

    #[test]
    fn living_code_move_while_unfixed() {
        let mut code = LivingCode::new(1, CodeKind::Data);
        assert!(code.move_to(7));
        assert_eq!(code.position, Some(7));
    }

    #[test]
    fn start_point_activation() {
        let mut sp = StartPoint::new(1, "main");
        assert!(sp.activate());
        assert!(sp.activated);
        assert!(sp.energy < 1.0);
    }

    #[test]
    fn start_point_exhaustion() {
        let mut sp = StartPoint::new(1, "weak");
        sp.energy = 0.05;
        assert!(!sp.activate(), "能量不足无法激活");
    }

    #[test]
    fn compiler_add_codes() {
        let mut c = HunyuanLiveCoder::new();
        let id = c.add_code(CodeKind::Data);
        assert_eq!(id, 0);
        assert_eq!(c.codes.len(), 1);
    }

    #[test]
    fn compiler_fix_order_calculates_value() {
        let mut c = HunyuanLiveCoder::new();
        c.add_start_point("main");
        let d1 = c.add_code(CodeKind::Data);
        let d2 = c.add_code(CodeKind::Data);
        let op = c.add_code(CodeKind::Operation);

        let order = vec![(d1, 0), (d2, 2), (op, 5)];
        let layout = c.compile(0, &order).unwrap();
        assert_eq!(layout.total_value, 3); // 1+1+1 (每个 count=1)
        assert_eq!(layout.permission_level, 3);
    }

    #[test]
    fn compiler_unfixed_count() {
        let mut c = HunyuanLiveCoder::new();
        c.add_start_point("main");
        c.add_code(CodeKind::Data);
        c.add_code(CodeKind::Data);
        c.add_code(CodeKind::Operation);

        assert_eq!(c.unfixed_codes().len(), 3);
        assert_eq!(c.liveness_ratio(), 1.0);
    }

    #[test]
    fn compiler_after_partial_fix() {
        let mut c = HunyuanLiveCoder::new();
        c.add_start_point("main");
        let id1 = c.add_code(CodeKind::Data);
        let id2 = c.add_code(CodeKind::Operation);

        let order = vec![(id1, 0)];
        let _ = c.compile(0, &order);

        assert_eq!(c.fixed_codes().len(), 1);
        assert_eq!(c.unfixed_codes().len(), 1);
        assert_eq!(c.liveness_ratio(), 0.5);
    }

    #[test]
    fn compiler_no_start_point_fails() {
        let mut c = HunyuanLiveCoder::new();
        c.add_code(CodeKind::Data);
        assert!(c.compile(99, &[(0, 0)]).is_none());
    }

    #[test]
    fn compiler_multiple_starts() {
        let mut c = HunyuanLiveCoder::new();
        let s1 = c.add_start_point("first");
        let s2 = c.add_start_point("second");
        assert_eq!(s1, 0);
        assert_eq!(s2, 1);
        assert_eq!(c.start_points.len(), 2);
    }

    #[test]
    fn compiler_cache_layout() {
        let mut c = HunyuanLiveCoder::new();
        c.add_start_point("main");
        let id = c.add_code(CodeKind::Data);
        let order = vec![(id, 0)];
        let _ = c.compile(0, &order);
        assert!(c.output_cache.contains_key(&0));
    }

    #[test]
    fn compiler_all_kinds() {
        let mut c = HunyuanLiveCoder::new();
        for kind in [CodeKind::StartPoint, CodeKind::Operation, CodeKind::Data, CodeKind::Router, CodeKind::Sovereign] {
            c.add_code(kind);
        }
        assert_eq!(c.codes.len(), 5);
    }
}
