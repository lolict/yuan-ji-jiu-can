// ╔══════════════════════════════════════════════════════════════════╗
// ║  满全法 · 囡囝共生元 — Rust 核心库                               ║
// ║  ManQuanFa · NanJian Symbiosis — Core Library                   ║
// ║                                                                  ║
// ║  Copyright © 刘楚恬 & lolict (七妹凹月留)                         ║
// ║  所有权保留 · 无 MIT / GPL / Apache 许可                          ║
// ╚══════════════════════════════════════════════════════════════════╝

pub mod mccp;
pub mod fsm;
pub mod kernel;

pub use mccp::{Mccp, Kind, MCP_HUANHUN, MCP_ZHAOHUN, MCP_DALING, MCP_JIESHU};
pub use fsm::FsmState;
pub use kernel::{CoupleOS, Process, ProcessState};
