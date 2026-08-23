//! 颜色进制解释器 · 交互式 REPL
//!
//! 支持命令:
//!   > canvas 16x16 "my_canvas"     — 创建画布
//!   > phase 128 weight 0.8         — 填充相位
//!   > pixel 3 4 phase 64 weight 0.5 — 设置单个像素
//!   > rect 0 0 8 8 family ipa      — 矩形区域
//!   > add "file.cb" at 4 4        — 加法（安装部件）
//!   > sub "file.cb" at 4 4        — 减法（剥离部件）
//!   > judge p1 p2                  — 审判配对
//!   > show                         — 显示当前画布
//!   > rgb                          — 翻译为 RGB
//!   > status                       — 系统状态
//!   > help                         — 帮助
//!   > quit                         — 退出

use std::collections::HashMap;
use std::io::{self, Write};
use crate::pixel::{Owner, PhaseCanvas, PhasePixel};
use crate::ops::{canvas_add, canvas_sub, BlendMode, PhaseOpsError};
use crate::balance::BalanceNetwork;
use crate::compiler::ColorBaseCompiler;

/// 交互式颜色进制解释器
pub struct ColorBaseInterpreter {
    pub canvas: Option<PhaseCanvas>,
    pub balance: BalanceNetwork,
    pub compiler: ColorBaseCompiler,
    pub history: Vec<String>,
    pub running: bool,
}

impl ColorBaseInterpreter {
    pub fn new() -> ColorBaseInterpreter {
        let mut balance = BalanceNetwork::new();
        balance.default_setup();
        ColorBaseInterpreter {
            canvas: None,
            balance,
            compiler: ColorBaseCompiler::new(),
            history: Vec::new(),
            running: false,
        }
    }

    /// 启动 REPL
    pub fn run(&mut self) {
        self.running = true;
        println!("=== 颜色进制 · 相位编码解释器 v1.0 ===");
        println!("输入 'help' 查看命令, 'quit' 退出\n");

        while self.running {
            print!("φ> ");
            io::stdout().flush().unwrap();
            let mut line = String::new();
            match io::stdin().read_line(&mut line) {
                Ok(0) | Err(_) => {
                    println!();
                    break;
                }
                Ok(_) => {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }
                    self.history.push(line.to_string());
                    self._execute(line);
                }
            }
        }
    }

    fn _execute(&mut self, line: &str) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return;
        }
        let cmd = parts[0].to_lowercase();

        let result = match cmd.as_str() {
            "quit" | "exit" => {
                self.running = false;
                println!("再见");
                Ok(())
            }
            "help" => {
                ColorBaseInterpreter::_show_help();
                Ok::<(), ()>(())
            }
            "canvas" => {
                let w = if parts.len() > 1 {
                    parts[1].split('x').nth(0).and_then(|s| s.parse::<u16>().ok()).unwrap_or(16)
                } else {
                    16
                };
                let h = if parts.len() > 1 {
                    parts[1].split('x').nth(1).and_then(|s| s.parse::<u16>().ok()).unwrap_or(16)
                } else {
                    16
                };
                let name = if parts.len() > 2 {
                    parts[2].trim_matches('"')
                } else {
                    "canvas"
                };
                self.canvas = Some(PhaseCanvas::new(w, h, name));
                println!("  → 创建画布: {}x{} '{}'", w, h, name);
                Ok(())
            }
            "phase" => {
                if !self._check_canvas() {
                    return;
                }
                let kwargs = Self::parse_kwargs(&parts[1..]);
                let phase = kwargs.get("phase").and_then(|s| s.parse::<u16>().ok()).unwrap_or(0);
                let weight = kwargs.get("weight").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.5);
                let brightness = kwargs.get("brightness").and_then(|s| s.parse::<f32>().ok()).unwrap_or(1.0);
                let level = Self::parse_level(kwargs.get("level").map(|s| s.as_str()));
                let family = kwargs.get("family").cloned().unwrap_or_else(|| "default".to_string());
                if let Some(ref mut canvas) = self.canvas {
                    for y in 0..canvas.height as i32 {
                        for x in 0..canvas.width as i32 {
                            canvas.set(
                                x,
                                y,
                                PhasePixel {
                                    phase,
                                    weight,
                                    brightness,
                                    level,
                                    family: family.clone(),
                                    ..Default::default()
                                },
                            );
                        }
                    }
                }
                println!("  → 全局填充: φ={}", phase);
                Ok(())
            }
            "pixel" => {
                if !self._check_canvas() {
                    return;
                }
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
                    let p = canvas.get(x, y);
                    println!("  → 像素({},{}): φ={} w={:.2} L{}", x, y, p.phase, p.weight, p.level);
                }
                Ok(())
            }
            "rect" => {
                if !self._check_canvas() {
                    return;
                }
                let x = parts[1].parse::<i32>().unwrap_or(0);
                let y = parts[2].parse::<i32>().unwrap_or(0);
                let w = parts[3].parse::<u16>().unwrap_or(1);
                let h = parts[4].parse::<u16>().unwrap_or(1);
                let kwargs = Self::parse_kwargs(&parts[5..]);
                if let Some(ref mut canvas) = self.canvas {
                    let family = kwargs.get("family").cloned().unwrap_or_default();
                    let phase = kwargs.get("phase").and_then(|s| s.parse::<u16>().ok()).unwrap_or(0);
                    let weight = kwargs.get("weight").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.5);
                    let level = Self::parse_level(kwargs.get("level").map(|s| s.as_str()));
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
                println!("  → 矩形区域: ({},{})+{}x{}", x, y, w, h);
                Ok(())
            }
            "add" => {
                if !self._check_canvas() {
                    return;
                }
                let src = parts[1].trim_matches('"');
                let x = parts[3].parse::<i32>().unwrap_or(0);
                let y = parts[4].parse::<i32>().unwrap_or(0);
                let comp = PhaseCanvas::new(8, 8, src);
                if let Some(ref mut canvas) = self.canvas {
                    match canvas_add(canvas, &comp, x, y, BlendMode::Overlay) {
                        Ok(c) => {
                            self.canvas = Some(c);
                            println!("  → 添加组件 '{}' 在 ({},{})", src, x, y);
                        }
                        Err(e) => {
                            println!("  ✗ {}", e);
                        }
                    }
                }
                Ok(())
            }
            "sub" => {
                if !self._check_canvas() {
                    return;
                }
                let src = parts[1].trim_matches('"');
                let x = parts[3].parse::<i32>().unwrap_or(0);
                let y = parts[4].parse::<i32>().unwrap_or(0);
                let comp = PhaseCanvas::new(8, 8, src);
                if let Some(ref canvas) = self.canvas {
                    match canvas_sub(canvas, &comp, x, y) {
                        Ok(c) => {
                            self.canvas = Some(c);
                            println!("  → 剥离组件 '{}' 在 ({},{})", src, x, y);
                        }
                        Err(e) => {
                            println!("  ✗ {}", e);
                        }
                    }
                }
                Ok(())
            }
            "judge" => {
                if !self._check_canvas() {
                    return;
                }
                let x1 = parts[1].parse::<i32>().unwrap_or(0);
                let y1 = parts[2].parse::<i32>().unwrap_or(0);
                let x2 = parts[3].parse::<i32>().unwrap_or(0);
                let y2 = parts[4].parse::<i32>().unwrap_or(0);
                if let Some(ref canvas) = self.canvas {
                    let p1 = canvas.get(x1, y1);
                    let p2 = canvas.get(x2, y2);
                    let verdict = self.balance.judge(&p1, &p2);
                    println!("  → 审判结果: {}", verdict.summary);
                    println!("     全局分: {:.2}", verdict.global_score);
                    for (dim, (p, conf, reason)) in &verdict.dimension_votes {
                        let mark = if *p { "✓" } else { "✗" };
                        println!("     {} {}: {} ({:.1})", mark, dim, reason, conf);
                    }
                }
                Ok(())
            }
            "show" => {
                if !self._check_canvas() {
                    return;
                }
                if let Some(ref canvas) = self.canvas {
                    let max_h = canvas.height.min(16) as i32;
                    let max_w = canvas.width.min(32) as i32;
                    for y in 0..max_h {
                        let mut row = String::new();
                        for x in 0..max_w {
                            let p = canvas.get(x, y);
                            if p.phase == 0 {
                                row.push('·');
                            } else if p.phase < 50 {
                                row.push('░');
                            } else if p.phase < 150 {
                                row.push('▒');
                            } else {
                                row.push('█');
                            }
                        }
                        println!("  {}", row);
                    }
                }
                Ok(())
            }
            "rgb" => {
                if !self._check_canvas() {
                    return;
                }
                if let Some(ref canvas) = self.canvas {
                    let max_h = canvas.height.min(8) as i32;
                    let max_w = canvas.width.min(16) as i32;
                    for y in 0..max_h {
                        let mut row = String::new();
                        for x in 0..max_w {
                            let (r, g, b) = canvas.get(x, y).to_rgb();
                            row.push_str(&format!("\x1b[48;2;{};{};{}m  \x1b[0m", r, g, b));
                        }
                        println!("  {}", row);
                    }
                }
                Ok(())
            }
            "status" => {
                self._show_status();
                Ok(())
            }
            _ => {
                println!("  ? 未知命令 '{}', 输入 'help' 查看帮助", cmd);
                Ok(())
            }
        };

        if let Err(_e) = result {
            // silently handled by inner error printing
        }
    }

    fn _check_canvas(&self) -> bool {
        if self.canvas.is_none() {
            println!("  ✗ 未创建画布, 先执行: canvas 16x16 \"name\"");
            return false;
        }
        true
    }

    fn _show_help() {
        println!(
            "\n命令列表:\n\
             \x20  canvas WxH \"name\"          — 创建画布\n\
             \x20  phase V weight W [level L] — 全局填充相位\n\
             \x20  pixel X Y ...              — 设置单个像素\n\
             \x20  rect X Y W H ...           — 填充矩形区域\n\
             \x20  add \"file\" at X Y          — 加法: 安装部件\n\
             \x20  sub \"file\" at X Y          — 减法: 剥离部件\n\
             \x20  judge X1 Y1 X2 Y2          — 审判两个像素的配对\n\
             \x20  show                       — 字符方式显示画布\n\
             \x20  rgb                        — 真彩色显示（需要真彩色终端）\n\
             \x20  status                     — 系统状态\n\
             \x20  help                       — 此帮助\n\
             \x20  quit                       — 退出\n"
        );
    }

    fn _show_status(&self) {
        match &self.canvas {
            Some(c) => println!("  画布: {}", c),
            None => println!("  画布: 未创建"),
        }
        println!("  制衡维度: {}", self.balance.dimensions.len());
        println!("  历史命令: {}条", self.history.len());
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

    fn parse_level(s: Option<&str>) -> u8 {
        match s {
            Some(s) => {
                let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
                digits.parse::<u8>().unwrap_or(1)
            }
            None => 1,
        }
    }
}

impl Default for ColorBaseInterpreter {
    fn default() -> ColorBaseInterpreter {
        ColorBaseInterpreter::new()
    }
}
