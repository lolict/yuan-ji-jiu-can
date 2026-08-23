//! 分布式编织制衡网络 · 天平称量
//!
//! 维度之间互相拉扯、互相校验、互相压制
//!   - 任一方面发现问题就输出否决拉力
//!   - 不是单条规则独大
//!   - 最终是多方拉力平衡的结果

use std::collections::HashMap;
use std::fmt;
use crate::pixel::{Owner, PhasePixel};

/// 单个维度的投票结果
pub type DimensionVote = (bool, f32, String);

/// 审判结果
#[derive(Debug, Clone)]
pub struct BalanceVerdict {
    pub passed: bool,
    pub dimension_votes: HashMap<String, DimensionVote>,
    pub global_score: f64,
    pub rejecting_dimensions: Vec<String>,
    pub summary: String,
}

/// 单个制衡维度
pub struct BalanceDimension {
    pub name: String,
    pub weight: f64,
    pub veto_power: f64,
    check_fn: Box<dyn Fn(&PhasePixel, &PhasePixel) -> DimensionVote>,
}

impl BalanceDimension {
    pub fn new<F>(name: &str, weight: f64, veto_power: f64, check_fn: F) -> BalanceDimension
    where
        F: Fn(&PhasePixel, &PhasePixel) -> DimensionVote + 'static,
    {
        BalanceDimension {
            name: name.to_string(),
            weight,
            veto_power,
            check_fn: Box::new(check_fn),
        }
    }

    pub fn check(&self, base: &PhasePixel, target: &PhasePixel) -> DimensionVote {
        (self.check_fn)(base, target)
    }
}

/// 层级栈校验: 只能同级配对
pub fn level_dimension() -> BalanceDimension {
    BalanceDimension::new("层级栈", 1.0, 1.0, |base, target| {
        if base.level == target.level {
            (true, 1.0, format!("同级L{}", base.level))
        } else if base.level.abs_diff(target.level) == 1 {
            (false, 0.3, format!("跨一级(L{}↔L{})", base.level, target.level))
        } else {
            (
                false,
                0.0,
                format!("层级跳跃(L{}↔L{})", base.level, target.level),
            )
        }
    })
}

/// 类型/家族校验
pub fn type_dimension() -> BalanceDimension {
    BalanceDimension::new("类型身份", 1.0, 1.0, |base, target| {
        if base.family == target.family {
            (true, 1.0, format!("同族{}", base.family))
        } else {
            let compatible = matches!(
                (base.family.as_str(), target.family.as_str()),
                ("ipa_extend", "latin")
                    | ("ipa_extend", "cyrillic")
                    | ("latin", "ipa_extend")
            );
            if compatible {
                (true, 0.7, "跨族兼容".to_string())
            } else {
                (
                    false,
                    0.2,
                    format!("家族不同({}↔{})", base.family, target.family),
                )
            }
        }
    })
}

/// 视觉审美校验（胖瘦/权重）
pub fn aesthetic_dimension(threshold: f64) -> BalanceDimension {
    BalanceDimension::new("视觉审美", 1.0, 1.0, move |base, target| {
        let diff = (base.weight - target.weight).abs() as f64;
        if diff <= 0.15 {
            (true, 1.0, format!("视觉协调(diff={:.2})", diff))
        } else if diff <= threshold {
            (true, 0.6, format!("勉强协调(diff={:.2})", diff))
        } else {
            (
                false,
                0.0,
                format!("视觉差大(diff={:.2},瘦子配胖子)", diff),
            )
        }
    })
}

/// 归属权校验
pub fn ownership_dimension() -> BalanceDimension {
    BalanceDimension::new("归属权", 1.0, 0.9, |base, target| {
        if base.owner == Owner::SelfOwned && target.owner == Owner::SelfOwned {
            (true, 1.0, "双方均为己方".to_string())
        } else if base.owner == Owner::External || target.owner == Owner::External {
            (false, 0.2, "含外来素材(需先解构重归属)".to_string())
        } else {
            (true, 0.8, "归属正常".to_string())
        }
    })
}

/// 衍射规则校验: 同基础+变音不是独立配对
pub fn diffraction_dimension() -> BalanceDimension {
    BalanceDimension::new("衍射规则", 1.0, 1.0, |base, target| {
        if base.phase == target.phase && base.level != target.level {
            (false, 0.0, "同基础跨层级(应触发衍射)".to_string())
        } else {
            (true, 1.0, "独立基元配对".to_string())
        }
    })
}

/// 黑名单反面样本检测
pub fn blacklist_dimension() -> BalanceDimension {
    BalanceDimension::new("黑名单", 1.0, 1.0, |base, target| {
        if base.blacklisted || target.blacklisted {
            (false, 0.0, "黑名单标记".to_string())
        } else {
            (true, 1.0, "无黑名单".to_string())
        }
    })
}

/// 分布式编织制衡网络
pub struct BalanceNetwork {
    pub dimensions: Vec<BalanceDimension>,
}

impl BalanceNetwork {
    pub fn new() -> BalanceNetwork {
        BalanceNetwork {
            dimensions: Vec::new(),
        }
    }

    pub fn add_dimension(&mut self, dim: BalanceDimension) {
        self.dimensions.push(dim);
    }

    /// 默认配置: 六个维度
    pub fn default_setup(&mut self) {
        self.dimensions = vec![
            level_dimension(),
            type_dimension(),
            aesthetic_dimension(0.35),
            ownership_dimension(),
            diffraction_dimension(),
            blacklist_dimension(),
        ];
    }

    /// 运行全部维度的审判
    pub fn judge(&self, base: &PhasePixel, target: &PhasePixel) -> BalanceVerdict {
        let mut votes = HashMap::new();
        let mut rejecting = Vec::new();
        let mut all_passed = true;
        let mut weighted_sum = 0.0;
        let mut total_weight = 0.0;

        for dim in &self.dimensions {
            let (passed, confidence, reason) = dim.check(base, target);
            votes.insert(dim.name.clone(), (passed, confidence, reason));
            weighted_sum += confidence as f64 * dim.weight;
            total_weight += dim.weight;

            if !passed && dim.veto_power >= 1.0 {
                rejecting.push(dim.name.clone());
                all_passed = false;
            } else if !passed {
                all_passed = false;
            }
        }

        let global_score = if total_weight > 0.001 {
            weighted_sum / total_weight
        } else {
            0.0
        };

        let (passed, final_rejecting) = if !rejecting.is_empty() {
            (false, rejecting)
        } else if global_score < 0.5 {
            let mut r = rejecting;
            r.push("全局协调分不足".to_string());
            (false, r)
        } else {
            (all_passed, rejecting)
        };

        let summary = if passed {
            "通过".to_string()
        } else {
            format!("拒绝({})", final_rejecting.join(","))
        };

        BalanceVerdict {
            passed,
            dimension_votes: votes,
            global_score,
            rejecting_dimensions: final_rejecting,
            summary,
        }
    }
}

impl Default for BalanceNetwork {
    fn default() -> BalanceNetwork {
        BalanceNetwork::new()
    }
}

impl fmt::Display for BalanceNetwork {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BalanceNetwork({} dims)", self.dimensions.len())
    }
}
