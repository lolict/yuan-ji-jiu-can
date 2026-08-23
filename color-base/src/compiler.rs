//! 颜色进制编译器
//!
//! 输入: .cb 文本格式（颜色进制脚本）
//! 输出: .cbf 二进制格式（可执行/可加载）
//!
//! CB 语法示例:
//!   canvas 16x16 "阿大"
//!   phase 128 weight 0.8 level L1
//!   rect 0 0 8 8 family ipa_extend
//!   add "声调部件.cb" at 4 4
//!   export "阿大.cbf"

use std::collections::HashMap;
use std::fmt;
use crate::pixel::{Owner, PhaseCanvas, PhasePixel};
use crate::ops::{canvas_add, canvas_sub, BlendMode, PhaseOpsError};
use crate::balance::BalanceNetwork;

/// CB → CBF 编译器
pub struct ColorBaseCompiler {
    pub canvas: Option<PhaseCanvas>,
    pub balance: BalanceNetwork,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ColorBaseCompiler {
    pub fn new() -> ColorBaseCompiler {
        let mut balance = BalanceNetwork::new();
        balance.default_setup();
        ColorBaseCompiler {
            canvas: None,
            balance,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// 编译 CB 源代码为 PhaseCanvas
    pub fn compile_source(&mut self, source: &str) -> Option<PhaseCanvas> {
        for (lineno, line) in source.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Err(e) = self._parse_line(line) {
                self.errors.push(format!("L{}: {}", lineno + 1, e));
            }
        }
        self.canvas.clone()
    }

    fn _parse_line(&mut self, line: &str) -> Result<(), PhaseOpsError> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }
        let cmd = parts[0].to_lowercase();

        match cmd.as_str() {
            "canvas" => {
                let dim: Vec<&str> = parts[1].split('x').collect();
                let w = dim[0].parse::<u16>().unwrap_or(16);
                let h = dim[1].parse::<u16>().unwrap_or(16);
                let name = parts.get(2).unwrap_or(&"canvas").trim_matches('"');
                self.canvas = Some(PhaseCanvas::new(w, h, name));
                Ok(())
            }
            "phase" => {
                let kwargs = Self::parse_kwargs(&parts[1..]);
                let pixel = Self::make_pixel(&kwargs);
                if let Some(ref mut canvas) = self.canvas {
                    for y in 0..canvas.height as i32 {
                        for x in 0..canvas.width as i32 {
                            canvas.set(x, y, pixel.clone());
                        }
                    }
                }
                Ok(())
            }
            "rect" => {
                let x = parts[1].parse::<i32>().unwrap_or(0);
                let y = parts[2].parse::<i32>().unwrap_or(0);
                let w = parts[3].parse::<u16>().unwrap_or(1);
                let h = parts[4].parse::<u16>().unwrap_or(1);
                let kwargs = Self::parse_kwargs(&parts[5..]);
                if let Some(ref mut canvas) = self.canvas {
                    let family = kwargs.get("family").cloned().unwrap_or_default();
                    let phase = kwargs.get("phase").and_then(|s| s.parse::<u16>().ok()).unwrap_or(0);
                    let weight = kwargs.get("weight").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.5);
                    let level = Self::parse_level(Some(kwargs.get("level").map(|s| s.as_str()).unwrap_or("L1")));
                    for dy in 0..h as i32 {
                        for dx in 0..w as i32 {
                            canvas.set(
                                x + dx,
                                y + dy,
                                PhasePixel {
                                    phase,
                                    weight,
                                    level,
                                    family: family.clone(),
                                    ..Default::default()
                                },
                            );
                        }
                    }
                }
                Ok(())
            }
            "pixel" => {
                let x = parts[1].parse::<i32>().unwrap_or(0);
                let y = parts[2].parse::<i32>().unwrap_or(0);
                let kwargs = Self::parse_kwargs(&parts[3..]);
                if let Some(ref mut canvas) = self.canvas {
                    let mut p = canvas.get(x, y);
                    if let Some(v) = kwargs.get("phase") {
                        p.phase = v.parse::<u16>().unwrap_or(p.phase);
                    }
                    if let Some(v) = kwargs.get("weight") {
                        p.weight = v.parse::<f32>().unwrap_or(p.weight);
                    }
                    if let Some(v) = kwargs.get("brightness") {
                        p.brightness = v.parse::<f32>().unwrap_or(p.brightness);
                    }
                    if let Some(v) = kwargs.get("level") {
                        p.level = Self::parse_level(Some(v));
                    }
                    if let Some(v) = kwargs.get("family") {
                        p.family = v.clone();
                    }
                    if let Some(v) = kwargs.get("owner") {
                        p.owner = if v == "SELF" {
                            Owner::SelfOwned
                        } else {
                            Owner::External
                        };
                    }
                    canvas.set(x, y, p);
                }
                Ok(())
            }
            "add" => {
                let src = parts[1].trim_matches('"');
                let x = parts[3].parse::<i32>().unwrap_or(0);
                let y = parts[4].parse::<i32>().unwrap_or(0);
                if let Some(ref mut canvas) = self.canvas {
                    let other = Self::load_placeholder(src);
                    match canvas_add(canvas, &other, x, y, BlendMode::Overlay) {
                        Ok(c) => {
                            self.canvas = Some(c);
                        }
                        Err(e) => {
                            self.errors.push(format!("add failed: {}", e));
                        }
                    }
                }
                Ok(())
            }
            "sub" => {
                let src = parts[1].trim_matches('"');
                let x = parts[3].parse::<i32>().unwrap_or(0);
                let y = parts[4].parse::<i32>().unwrap_or(0);
                if let Some(ref mut canvas) = self.canvas {
                    let other = Self::load_placeholder(src);
                    match canvas_sub(canvas, &other, x, y) {
                        Ok(c) => {
                            self.canvas = Some(c);
                        }
                        Err(e) => {
                            self.errors.push(format!("sub failed: {}", e));
                        }
                    }
                }
                Ok(())
            }
            _ => Err(PhaseOpsError::new(&format!("未知指令: {}", cmd))),
        }
    }

    fn parse_kwargs(tokens: &[&str]) -> HashMap<String, String> {
        let mut kwargs = HashMap::new();
        let mut i = 0;
        while i + 1 < tokens.len() {
            kwargs.insert(tokens[i].to_string(), tokens[i + 1].to_string());
            i += 2;
        }
        kwargs
    }

    fn make_pixel(kwargs: &HashMap<String, String>) -> PhasePixel {
        PhasePixel {
            phase: kwargs.get("phase").and_then(|s| s.parse::<u16>().ok()).unwrap_or(0),
            weight: kwargs.get("weight").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.5),
            brightness: kwargs.get("brightness").and_then(|s| s.parse::<f32>().ok()).unwrap_or(1.0),
            level: Self::parse_level(kwargs.get("level").map(|s| s.as_str())),
            family: kwargs.get("family").cloned().unwrap_or_else(|| "default".to_string()),
            owner: if kwargs.get("owner").map(|s| s.as_str()) == Some("SELF") {
                Owner::SelfOwned
            } else {
                Owner::External
            },
            ..Default::default()
        }
    }

    fn parse_level(s: Option<&str>) -> u8 {
        match s {
            Some(s) => {
                let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
                digits.parse::<u8>().unwrap_or(1)
            }
            None => 1,
        }
    }

    fn load_placeholder(name: &str) -> PhaseCanvas {
        let mut c = PhaseCanvas::new(8, 8, name);
        for y in 0..8 {
            for x in 0..8 {
                c.set(x, y, PhasePixel {
                    phase: 64,
                    weight: 0.7,
                    level: 2,
                    family: "modifier".to_string(),
                    ..Default::default()
                });
            }
        }
        c
    }

    /// 编译源代码为 CBF 二进制
    pub fn compile_to_bytes(&mut self, source: &str) -> Vec<u8> {
        match self.compile_source(source) {
            Some(canvas) => Self::canvas_to_cbf(&canvas),
            None => Vec::new(),
        }
    }

    fn canvas_to_cbf(canvas: &PhaseCanvas) -> Vec<u8> {
        let mut out = Vec::new();
        // magic: CBF\0
        out.extend_from_slice(b"CBF\x00");
        // width: 2 bytes
        out.extend_from_slice(&canvas.width.to_be_bytes());
        // height: 2 bytes
        out.extend_from_slice(&canvas.height.to_be_bytes());
        // name: 32 bytes (zero-padded)
        let name_bytes = canvas.name.as_bytes();
        let mut name_buf = vec![0u8; 32];
        let len = name_bytes.len().min(32);
        name_buf[..len].copy_from_slice(&name_bytes[..len]);
        out.extend_from_slice(&name_buf);
        // pixel data
        for y in 0..canvas.height as usize {
            for x in 0..canvas.width as usize {
                out.extend_from_slice(&canvas.grid[y][x].to_bytes());
            }
        }
        out
    }

    /// 获取编译报告
    pub fn get_report(&self) -> String {
        let mut lines = Vec::new();
        if !self.errors.is_empty() {
            lines.push(format!("Errors ({}):", self.errors.len()));
            for e in &self.errors {
                lines.push(format!("  {}", e));
            }
        }
        if !self.warnings.is_empty() {
            lines.push(format!("Warnings ({}):", self.warnings.len()));
            for w in &self.warnings {
                lines.push(format!("  {}", w));
            }
        }
        if self.errors.is_empty() && self.warnings.is_empty() {
            lines.push("编译成功, 无错误".to_string());
        }
        lines.join("\n")
    }
}

impl Default for ColorBaseCompiler {
    fn default() -> ColorBaseCompiler {
        ColorBaseCompiler::new()
    }
}

impl fmt::Display for ColorBaseCompiler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ColorBaseCompiler(canvas={}, errors={})",
            self.canvas.is_some(),
            self.errors.len()
        )
    }
}
