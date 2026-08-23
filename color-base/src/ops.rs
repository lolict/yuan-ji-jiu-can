//! 像素相位运算 · 仅允许加法(+)/减法(-)，禁止乘法(*)/除法(/)
//!
//! 加法 = 部件安装、拼装、移植
//! 减法 = 部件拆解、剥离、挖除
//! 乘除 = 对符号构造无意义，直接禁用

use crate::pixel::{PhaseCanvas, PhasePixel, Owner};

/// 相位运算错误
#[derive(Debug, Clone, PartialEq)]
pub struct PhaseOpsError {
    pub message: String,
}

impl std::fmt::Display for PhaseOpsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PhaseOpsError: {}", self.message)
    }
}

impl std::error::Error for PhaseOpsError {}

impl PhaseOpsError {
    pub fn new(msg: &str) -> PhaseOpsError {
        PhaseOpsError {
            message: msg.to_string(),
        }
    }
}

/// 混合模式
#[derive(Debug, Clone, PartialEq)]
pub enum BlendMode {
    Overlay,
    Merge,
    Stamp,
}

impl std::str::FromStr for BlendMode {
    type Err = PhaseOpsError;
    fn from_str(s: &str) -> Result<BlendMode, PhaseOpsError> {
        match s {
            "overlay" => Ok(BlendMode::Overlay),
            "merge" => Ok(BlendMode::Merge),
            "stamp" => Ok(BlendMode::Stamp),
            _ => Err(PhaseOpsError::new(&format!("未知混合模式: {}", s))),
        }
    }
}

/// 像素相位加法: base + component
/// 语义: 将组件部件安装/叠加到基础像素上
pub fn phase_add(
    base: &PhasePixel,
    component: &PhasePixel,
    blend_mode: BlendMode,
) -> Result<PhasePixel, PhaseOpsError> {
    if base.blacklisted || component.blacklisted {
        return Err(PhaseOpsError::new("无法对黑名单像素执行加法运算"));
    }

    let mut result = PhasePixel {
        phase: base.phase,
        weight: base.weight,
        brightness: base.brightness,
        owner: base.owner.clone(),
        level: base.level.max(component.level),
        family: base.family.clone(),
        parent_source: base.parent_source.clone(),
        blacklisted: false,
        prefix_ids: base.prefix_ids.clone(),
        suffix_ids: base.suffix_ids.clone(),
        r: base.r,
        g: base.g,
        b: base.b,
    };

    match blend_mode {
        BlendMode::Overlay => {
            if component.phase != 0 {
                result.phase = component.phase;
                result.weight = component.weight;
                result.brightness = component.brightness;
                result.family = component.family.clone();
                result.suffix_ids.extend(component.suffix_ids.clone());
            }
        }
        BlendMode::Merge => {
            let total_w = base.weight + component.weight;
            if total_w > 0.0 {
                result.phase = ((base.phase as f32 * base.weight
                    + component.phase as f32 * component.weight)
                    / total_w) as u16;
                result.weight = total_w.min(1.0);
                result.brightness = (base.brightness + component.brightness) / 2.0;
            }
            result.parent_source.push(format!(
                "merge(φ{}+φ{})",
                base.phase, component.phase
            ));
        }
        BlendMode::Stamp => {
            result.phase = component.phase;
            result.weight = component.weight;
            result.brightness = component.brightness;
            result.family = component.family.clone();
            result.parent_source
                .push(format!("stamp(φ{})", component.phase));
        }
    }

    Ok(result)
}

/// 像素相位减法: base - component
/// 语义: 从基础像素上剥离/挖除部件
pub fn phase_sub(base: &PhasePixel, component: &PhasePixel) -> Result<PhasePixel, PhaseOpsError> {
    if base.blacklisted {
        return Err(PhaseOpsError::new("无法对黑名单像素执行减法运算"));
    }

    let mut result = PhasePixel {
        phase: base.phase,
        weight: base.weight,
        brightness: base.brightness,
        owner: base.owner.clone(),
        level: base.level,
        family: base.family.clone(),
        parent_source: base.parent_source.clone(),
        blacklisted: base.blacklisted,
        prefix_ids: base.prefix_ids.clone(),
        suffix_ids: base.suffix_ids.clone(),
        r: base.r,
        g: base.g,
        b: base.b,
    };

    if base.phase == component.phase {
        result.phase = 0;
        result.weight = 0.0;
        result.brightness = 0.0;
        result.parent_source
            .push(format!("sub(φ{})", component.phase));
    } else if base.weight > component.weight {
        result.weight = base.weight - component.weight;
        result.parent_source
            .push(format!("sub_w({:.2})", component.weight));
    }

    Ok(result)
}

/// 画布级加法: 将 component 安装到 base 上
pub fn canvas_add(
    base: &PhaseCanvas,
    component: &PhaseCanvas,
    offset_x: i32,
    offset_y: i32,
    blend_mode: BlendMode,
) -> Result<PhaseCanvas, PhaseOpsError> {
    let mut result = PhaseCanvas::new(base.width, base.height, &format!("{}+{}", base.name, component.name));
    for y in 0..base.height as i32 {
        for x in 0..base.width as i32 {
            result.set(x, y, base.get(x, y));
        }
    }
    for y in 0..component.height as i32 {
        for x in 0..component.width as i32 {
            let tx = x + offset_x;
            let ty = y + offset_y;
            if tx >= 0 && tx < base.width as i32 && ty >= 0 && ty < base.height as i32 {
                let merged = phase_add(&result.get(tx, ty), &component.get(x, y), blend_mode.clone())?;
                result.set(tx, ty, merged);
            }
        }
    }
    Ok(result)
}

/// 画布级减法: 从 base 上剥离 component
pub fn canvas_sub(
    base: &PhaseCanvas,
    component: &PhaseCanvas,
    offset_x: i32,
    offset_y: i32,
) -> Result<PhaseCanvas, PhaseOpsError> {
    let mut result = PhaseCanvas::new(base.width, base.height, &format!("{}-{}", base.name, component.name));
    for y in 0..base.height as i32 {
        for x in 0..base.width as i32 {
            result.set(x, y, base.get(x, y));
        }
    }
    for y in 0..component.height as i32 {
        for x in 0..component.width as i32 {
            let tx = x + offset_x;
            let ty = y + offset_y;
            if tx >= 0 && tx < base.width as i32 && ty >= 0 && ty < base.height as i32 {
                let stripped = phase_sub(&result.get(tx, ty), &component.get(x, y))?;
                result.set(tx, ty, stripped);
            }
        }
    }
    Ok(result)
}

/// 乘法已禁用
pub fn phase_mul(_base: &PhasePixel, _component: &PhasePixel) -> Result<PhasePixel, PhaseOpsError> {
    Err(PhaseOpsError::new(
        "乘法运算已禁用: 对符号构造无意义（会改变视觉权重，破坏等效镜像距离）",
    ))
}

/// 除法已禁用
pub fn phase_div(_base: &PhasePixel, _component: &PhasePixel) -> Result<PhasePixel, PhaseOpsError> {
    Err(PhaseOpsError::new(
        "除法运算已禁用: 对符号构造无意义（不具有均分结构的合理业务）",
    ))
}
