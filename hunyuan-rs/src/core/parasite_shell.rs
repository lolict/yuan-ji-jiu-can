// parasite_shell.rs — 蚕蛹反向寄生壳引擎 (v0.1)
// 跨平台部署 — stub (原文件被并行代理删除)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ShellKind { Cargo, Npm, PyPI, Docker, Wasm, Elf, Apk }

pub struct CocoonPackage;
impl CocoonPackage {
    pub fn spawn_offline() -> Self { Self }
    pub fn digest(&self) -> [u8; 8] { [0u8; 8] }
    pub fn available_shells(&self) -> Vec<ShellKind> { vec![ShellKind::Cargo, ShellKind::Wasm] }
}

pub struct ParasiteEngine;
impl ParasiteEngine {
    pub fn new() -> Self { Self }
    pub fn stealth_score(&self) -> f64 { 0.75 }
}
